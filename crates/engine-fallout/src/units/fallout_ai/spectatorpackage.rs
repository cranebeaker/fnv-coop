//! `fallout/ai/spectatorpackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// `0072ba80(this, address, count)`: the per-element destroy step
/// (translated in `fallout/interface/menus/recipemenu.cpp`).
const DESTROY_ELEMENTS: u32 = 0x0072_ba80;
/// `0042fc00(this, new_reserved, size)` (`extradatalist.cpp`): resizes the
/// buffer for the new reserved size.
const RESIZE_BUFFER: u32 = 0x0042_fc00;

// Translated from 0084b5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<Actor *, 1024>::SetReservedSize` (engine map name; the
/// body sets the reserved size field, +0xc). Nothing happens when the
/// reserved size is unchanged. When the new size is below `iSize`, the
/// elements from the new size on are destroyed (`0072ba80`) and `iSize` is
/// cut to the new size. Then the buffer is resized (`0042fc00`, new size and
/// current `iSize`) and the reserved size is stored.
pub fn bs_simple_array_set_reserved_size(e: &mut Engine, this: Ptr<BSSimpleArray>, new_size: u32) {
    if new_size == e.get(this, BSSimpleArray::iReservedSize) {
        return;
    }
    let size = e.get(this, BSSimpleArray::iSize);
    if new_size < size {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let first = buffer.wrapping_add(new_size.wrapping_mul(4));
        e.call(DESTROY_ELEMENTS, &args![this, first, size - new_size]);
        e.set(this, BSSimpleArray::iSize, new_size);
    }
    let size = e.get(this, BSSimpleArray::iSize);
    e.call(RESIZE_BUFFER, &args![this, new_size, size]);
    e.set(this, BSSimpleArray::iReservedSize, new_size);
}

/// `009f8ac0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const ELEMENT_TARGET_009F8AC0: u32 = 0x009f_8ac0;

// Translated from 009f8a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (spectator package; callers `009f8420`, `009f8020`, `009f76e0`):
/// forwards `this` and an index to `009f8ac0`, which returns the word at `this+4` plus `index`
/// times 0x34: the address of a 0x34-byte element. The wrapper returns that address.
pub fn fn_009f8a80(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    e.call(ELEMENT_TARGET_009F8AC0, &args![this, index]).ptr()
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VSpectatorThreatInfo@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_01093980: u32 = 0x0109_3980;
/// `009f8d00` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_009F8D00: u32 = 0x009f_8d00;

// Translated from 009f8b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VSpectatorThreatInfo@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `009f8d00(this, 1)`.
pub fn fn_009f8b10(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01093980);
    e.call(ARRAY_RELEASE_009F8D00, &args![this, 1u32]);
}

/// `SpectatorPackage` (Xbox PDB names; the PC offsets are 0x10 below the
/// PDB's, `TESForm` being 0x10 bytes smaller): `iLastPathRequestTime`
/// +0x80, `fTimer` +0x84, `fConversationTimer` +0x88, `fEventRadius` +0x8C,
/// `bFlagCompleted` +0x90, `EventCenter` +0x94 (three `float`s),
/// `ThreatInfoList` +0xA0 (`BSSimpleArray<SpectatorThreatInfo,1024>`) and
/// `spDebugGeometry` +0xB0.
const LAST_PATH_REQUEST_TIME: u32 = 0x80;
const TIMER: u32 = 0x84;
const CONVERSATION_TIMER: u32 = 0x88;
const EVENT_RADIUS: u32 = 0x8c;
const FLAG_COMPLETED: u32 = 0x90;
const EVENT_CENTER: u32 = 0x94;
const THREAT_INFO_LIST: u32 = 0xa0;
const DEBUG_GEOMETRY: u32 = 0xb0;

/// `SpectatorThreatInfo` (Xbox PDB, 0x34 bytes as on the PC): `pSource` +0,
/// `pTarget` +4, `eThreatLevel` +8, `iLastPerceivedTime` +0xC, `fRadius`
/// +0x10, `LastKnownPosSource` +0x14, `LastKnownPosTarget` +0x20,
/// `bHasSource` +0x2C, `bHasTarget` +0x2D, `spDebugGeometry` +0x30.
const THREAT_SIZE: u32 = 0x34;
const THREAT_SOURCE: u32 = 0x00;
const THREAT_TARGET: u32 = 0x04;
const THREAT_LEVEL: u32 = 0x08;
const THREAT_LAST_PERCEIVED_TIME: u32 = 0x0c;
const THREAT_RADIUS: u32 = 0x10;
const THREAT_SOURCE_POSITION: u32 = 0x14;
const THREAT_TARGET_POSITION: u32 = 0x20;
const THREAT_HAS_SOURCE: u32 = 0x2c;
const THREAT_HAS_TARGET: u32 = 0x2d;
const THREAT_DEBUG_GEOMETRY: u32 = 0x30;

/// `BSSimpleArray` (Xbox PDB): `pBuffer` +4, `uiSize` +8, `uiAllocSize` +0xC.
const ARRAY_BUFFER: u32 = 0x04;
const ARRAY_SIZE: u32 = 0x08;
const ARRAY_ALLOC_SIZE: u32 = 0x0c;

/// `0044ddc0` (`ECX` only): the word at +8 (an array's size).
const WORD_AT_8: u32 = 0x0044_ddc0;
/// `00559450` (`ECX` only): the word at +0 (a threat's `pSource`).
const WORD_AT_0: u32 = 0x0055_9450;
/// `00726070` (`ECX` only): the word at +4 (a threat's `pTarget`).
const WORD_AT_4: u32 = 0x0072_6070;
/// `0084e3a0` (`ECX` only): the word at +0xC (a threat's `iLastPerceivedTime`).
const WORD_AT_C: u32 = 0x0084_e3a0;
/// The actor's virtual slot (byte offset) whose result the threat keeps as
/// its target.
const SLOT_ACTOR_TARGET: u32 = 0x42c;
/// `TESObjectREFR::SetTargeted` (`00564db0`, `thiscall`, `RET 4`: a flag).
const SET_TARGETED: u32 = 0x0056_4db0;
/// `009f6d50` (`run_009f6c90.rs`, `ECX` only): refreshes a threat.
const THREAT_REFRESH_009F6D50: u32 = 0x009f_6d50;
/// `009a4e30` (`thiscall`, `RET 0x10`: two point addresses and two
/// `float`s) and `006915d0` (`thiscall`, `RET 0xC`: a point address and two
/// `float`s): build the 0x24-byte shape `009f6f30` copies out.
const SHAPE_OF_TWO_009A4E30: u32 = 0x009a_4e30;
const SHAPE_OF_ONE_006915D0: u32 = 0x0069_15d0;
/// `100.0` (`float`).
const HUNDRED: u32 = 0x0101_6410;
/// `009f7050` / `009f7080` (`run_009f7050.rs`, `RET 4`): copy a threat's
/// source / target position into the given point and return it.
const THREAT_SOURCE_POSITION_009F7050: u32 = 0x009f_7050;
const THREAT_TARGET_POSITION_009F7080: u32 = 0x009f_7080;
/// `009f8de0` (`run_009f8de0.rs`; `thiscall`, `RET 8`: the first item and
/// the count): constructs threats in place.
const THREATS_CONSTRUCT_009F8DE0: u32 = 0x009f_8de0;
/// `009f8fa0` (`run_009f8fa0.rs`; `thiscall`, `RET 0xC`: destination,
/// source, count): moves threats.
const THREATS_MOVE_009F8FA0: u32 = 0x009f_8fa0;
/// `009f9180` (`run_009f9180.rs`; `thiscall`, `RET 4`): a threat's scalar
/// deleting destructor (with 0: destroys only).
const THREAT_DELETE_009F9180: u32 = 0x009f_9180;
/// `BGSSaveGameBuffer` writes (`00865e50`: the address, the size and a flag
/// word; `00865df0`: a form and a flag word; `00865f60`
/// `SaveVariableSizedValue`), and the `BGSLoadGameBuffer` reads
/// (`00864980`: the address and the size; `008648e0` `LoadFormID_ov2`: the
/// address of the id; `00864a60` `LoadVariableSizedValue`).
const SAVE_DATA: u32 = 0x0086_5e50;
const SAVE_FORM_ID: u32 = 0x0086_5df0;
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f60;
const LOAD_DATA: u32 = 0x0086_4980;
const LOAD_FORM_ID: u32 = 0x0086_48e0;
const LOAD_VARIABLE_SIZED_VALUE: u32 = 0x0086_4a60;
/// `00457fe0` (no arguments, `tes.cpp`): the time the threats are stamped with.
const CURRENT_TIME_00457FE0: u32 = 0x0045_7fe0;
/// `004839c0` (cdecl, one word): the form of an id.
const LOOKUP_FORM_004839C0: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and its type
/// descriptors: `.?AVTESForm@@`, `.?AVActor@@`, `.?AVSpectatorPackage@@`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_ACTOR: u32 = 0x0118_46d4;
const TYPE_SPECTATOR_PACKAGE: u32 = 0x011a_5fd8;
/// `TESPackage::TESPackage` (`006707c0`, engine map) and its destructor
/// body (`006708b0`).
const PACKAGE_CONSTRUCT: u32 = 0x0067_07c0;
const PACKAGE_DESTRUCT_006708B0: u32 = 0x0067_08b0;
/// The `SpectatorPackage` vtable (RTTI `.?AVSpectatorPackage@@`).
const SPECTATOR_PACKAGE_VTABLE: u32 = 0x0109_381c;
/// The point at `011f426c` (three `float`s) the event center starts as.
const POINT_011F426C: u32 = 0x011f_426c;
/// `00633c90` (`thiscall`, `RET 4`): constructs an `NiPointer` with a value.
const NI_POINTER_CONSTRUCT_00633C90: u32 = 0x0063_3c90;
/// `0066b0d0` (`thiscall`, `RET 4`): assigns an `NiPointer`.
const NI_POINTER_ASSIGN_0066B0D0: u32 = 0x0066_b0d0;
/// `0045cec0` (`ECX` only): destroys an `NiPointer`.
const NI_POINTER_DESTRUCT_0045CEC0: u32 = 0x0045_cec0;
/// `NiPointer<..>::operator=` (`006e5cc0`, engine map; `RET 4`).
const NI_POINTER_COPY: u32 = 0x006e_5cc0;
/// `TESPackage::SetPackType` (`00670fc0`, engine map; `RET 4`).
const PACKAGE_SET_PACK_TYPE: u32 = 0x0067_0fc0;
/// `00984f60` (`thiscall`, `RET 4`): the package step the constructor runs with 0xF.
const PACKAGE_STEP_00984F60: u32 = 0x0098_4f60;
/// `009611e0` (`ECX` only): the word at +0x18 (the geometry's parent).
const WORD_AT_18: u32 = 0x0096_11e0;
/// The parent's virtual slot (byte offset) the destructor detaches the
/// debug geometry with.
const SLOT_DETACH_CHILD: u32 = 0xe8;
/// `00673290` (`thiscall`, `RET 4`): copies the `TESPackage` part of a form.
const PACKAGE_COPY_00673290: u32 = 0x0067_3290;
/// `00404010` (cdecl, two `float`s): the larger (the second when not
/// below it), in ST0; `0040ebd0`: the smaller (the second when not above it).
const MAX_00404010: u32 = 0x0040_4010;
const MIN_0040EBD0: u32 = 0x0040_ebd0;
/// `0063c8a0` (`thiscall`, `RET 4`): adds a point to `this`; `00439180`
/// (`thiscall`, `RET 4`: a `float`): scales `this`.
const POINT_ADD_IN_PLACE_0063C8A0: u32 = 0x0063_c8a0;
const POINT_SCALE_IN_PLACE_00439180: u32 = 0x0043_9180;
/// `0.5` (`float`).
const HALF: u32 = 0x0101_6248;
/// `00439ef0` (`thiscall`, `RET 8`: the result, the other point): `this - other`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `004a7290` (`ECX` only): a point's squared length in ST0.
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
/// `004019d0` (cdecl, a `float`): its result in ST0 (the radius from the
/// largest squared distance).
const RADIUS_OF_004019D0: u32 = 0x0040_19d0;
/// `008d8520` (`ECX` only): the word at +0x68 of an actor (its process).
const ACTOR_PROCESS: u32 = 0x008d_8520;
/// `Actor::StopMoving` (`008b3ab0`, engine map; `ECX` only).
const ACTOR_STOP_MOVING: u32 = 0x008b_3ab0;
/// `00487f50` (no arguments): a random number.
const RANDOM_00487F50: u32 = 0x0048_7f50;
/// The target's virtual slot (byte offset) whose byte says it may be watched.
const SLOT_TARGET_WATCHABLE: u32 = 0x100;
/// `00403e20` (`thiscall` on a setting): the address of its value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The settings `009f7d70` and `009f8020` read (`float`s, by address).
const SETTING_011CDC04: u32 = 0x011c_dc04;
const SETTING_011CCFF0: u32 = 0x011c_cff0;
const SETTING_011CD264: u32 = 0x011c_d264;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `1000.0` (`double`).
const THOUSAND: u32 = 0x0101_7b70;
/// `TESTopic::GetTopic` (`0061a2d0`, engine map; cdecl, two words).
const TOPIC_GET_TOPIC: u32 = 0x0061_a2d0;
/// `0057bd60` (`thiscall`, `RET 4`): the reference step `009f7d70` runs
/// with the target before the conversation.
const REFERENCE_STEP_0057BD60: u32 = 0x0057_bd60;
/// The process's virtual slots (byte offsets) the spectator uses.
const SLOT_PROCESS_0X310: u32 = 0x310;
const SLOT_PROCESS_0X30C: u32 = 0x30c;
const SLOT_PROCESS_0X2A4: u32 = 0x2a4;
const SLOT_PROCESS_0X628: u32 = 0x628;
const SLOT_PROCESS_0X644: u32 = 0x644;
const SLOT_PROCESS_0X504: u32 = 0x504;
/// The game timer object and its `float` at +0xC (`0084d030`).
const GAME_TIMER: u32 = 0x011f_6394;
const TIMER_VALUE_0084D030: u32 = 0x0084_d030;
/// `00825c00` (`thiscall` on the game timer): the time in milliseconds.
const TIMER_NOW: u32 = 0x0082_5c00;
/// The actor's virtual slots (byte offsets) `009f8020` asks about a target
/// (0x22C with 0, 0x234, 0x230) and a source (0x218).
const SLOT_ACTOR_0X22C: u32 = 0x22c;
const SLOT_ACTOR_0X234: u32 = 0x234;
const SLOT_ACTOR_0X230: u32 = 0x230;
const SLOT_ACTOR_0X218: u32 = 0x218;
/// `00440d80` (`ECX` only): bit 0x20 of the word at +8 of a form.
const FORM_FLAG_20_00440D80: u32 = 0x0044_0d80;
/// `004534f0` (`thiscall`, `RET 4`, `tes.cpp`): the step `009f8020` ends with.
const PACKAGE_UPDATE_004534F0: u32 = 0x0045_34f0;
/// `00aa13e0` (cdecl, the size): allocates a block.
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
/// `006e3850` (`ECX` only): constructs the 0x18-byte array `009f8570` returns.
const SHAPE_ARRAY_CONSTRUCT_006E3850: u32 = 0x006e_3850;
/// `006e3ad0` (`thiscall`, `RET 8`: the size and a flag): its `SetSize`.
const SHAPE_ARRAY_SET_SIZE_006E3AD0: u32 = 0x006e_3ad0;
/// `006a1400` (`thiscall`, `RET 4`): the address of its element.
const SHAPE_ARRAY_AT_006A1400: u32 = 0x006a_1400;
/// `008454f0` (`thiscall`, `RET 4`): empties an array (with 1, also frees).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// `BSSimpleArray<..>::SetReservedSize` (`0084b5c0`, engine map; `RET 4`).
const ARRAY_SET_RESERVED_SIZE: u32 = 0x0084_b5c0;
/// `007cb2e0` (`thiscall`, `RET 4`): appends the item whose address it gets.
const ARRAY_ADD: u32 = 0x007c_b2e0;
/// The `TESPackage` parts of the save functions: `00678fd0` (the save
/// size, a halfword), `00679120` and `TESPackage::LoadGame` (`00679340`),
/// `006796c0`, `TESPackage::SaveGame` (`006797c0`), `LoadGame_ov2`
/// (`006798c0`) and `InitLoadGame` (`00679a90`).
const PACKAGE_SAVE_SIZE_00678FD0: u32 = 0x0067_8fd0;
const PACKAGE_SAVE_00679120: u32 = 0x0067_9120;
const PACKAGE_LOAD_GAME: u32 = 0x0067_9340;
const PACKAGE_STEP_006796C0: u32 = 0x0067_96c0;
const PACKAGE_SAVE_GAME: u32 = 0x0067_97c0;
const PACKAGE_LOAD_GAME_OV2: u32 = 0x0067_98c0;
const PACKAGE_INIT_LOAD_GAME: u32 = 0x0067_9a90;
/// `TESForm::SaveGameDataOLD` / `LoadGameDataOLD` (`00484ce0` / `00484d00`,
/// engine map; `thiscall`, `RET 8`: the address and the size).
const FORM_SAVE_GAME_DATA_OLD: u32 = 0x0048_4ce0;
const FORM_LOAD_GAME_DATA_OLD: u32 = 0x0048_4d00;
/// `0042f5a0` (cdecl, two words): the count `009f8c70` moves.
const MOVE_COUNT_0042F5A0: u32 = 0x0042_f5a0;
/// `0042f5d0` (cdecl: the block and the size): reallocates.
const REALLOCATE_0042F5D0: u32 = 0x0042_f5d0;
/// `MemoryManager` allocation (`00401000`, cdecl, the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;
/// `006a8500` (`ECX` only): frees an array's buffer through the array's
/// virtual slot +8 and clears the buffer pointer.
const ARRAY_FREE_BUFFER_006A8500: u32 = 0x006a_8500;
/// The array's virtual slots (byte offsets): +4 allocates `n` items, +0xC
/// reallocates a buffer to `n` items.
const SLOT_ARRAY_ALLOCATE: u32 = 0x4;
const SLOT_ARRAY_REALLOCATE: u32 = 0xc;

fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// `FLD dword [address]; FSTP dword`: the word as the x87 copy leaves it.
fn float_copy(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(address))
}

/// The address of the threat `index` of the array at `this` + 0xA0
/// (`009f8a80`).
fn threat_at(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    fn_009f8a80(e, Ptr::new(this.addr() + THREAT_INFO_LIST), index).addr()
}

// Translated from 009f6c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`SpectatorThreatInfo`; `RET 4`): makes the
/// threat about `source`: `pSource`, `pTarget` from the source's virtual
/// slot +0x42C, `bHasSource` 1, `bHasTarget` whether there is a target;
/// both are marked targeted (`SetTargeted(1)`, the target only when set),
/// then `009f6d50` refreshes the threat.
pub fn fn_009f6c20(e: &mut Engine, this: Ptr, source: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + THREAT_SOURCE, source.addr());
    let target = e.vcall(source.addr(), SLOT_ACTOR_TARGET, &args![]).u32();
    e.mem.set_u32(base + THREAT_TARGET, target);
    e.mem.set_u8(base + THREAT_HAS_SOURCE, 1);
    let has_target = u8::from(e.mem.u32(base + THREAT_TARGET) != 0);
    e.mem.set_u8(base + THREAT_HAS_TARGET, has_target);
    let source = e.mem.u32(base + THREAT_SOURCE);
    e.call(SET_TARGETED, &args![source, 1u32]);
    if e.mem.u32(base + THREAT_TARGET) != 0 {
        let target = e.mem.u32(base + THREAT_TARGET);
        e.call(SET_TARGETED, &args![target, 1u32]);
    }
    e.call(THREAT_REFRESH_009F6D50, &args![this]);
}

// Translated from 009f6f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`SpectatorThreatInfo`; `RET 4`): writes the
/// threat's 0x24-byte shape into `out` (nine words): with a source and a
/// target, `009a4e30` over both positions, the radius and 100.0; with one
/// of them, `006915d0` over its position, the radius and 100.0; with
/// neither, `out` is left as it is.
pub fn fn_009f6f30(e: &mut Engine, this: Ptr, out: Ptr) {
    let base = this.addr();
    let has_source = e.mem.u8(base + THREAT_HAS_SOURCE) != 0;
    let has_target = e.mem.u8(base + THREAT_HAS_TARGET) != 0;
    if !has_source && !has_target {
        return;
    }
    // The frame from EBP-0xA0: the points (EBP-0x30, -0x3C, -0x6C, -0x9C)
    // and the shapes (EBP-0x24, -0x60, -0x90).
    e.with_stack(0xa0, |e, frame| {
        let at = |offset: u32| frame.addr() + 0xa0 - offset;
        let hundred = x87_quiet_bits(e.global::<u32>(HUNDRED));
        let radius = float_copy(e, base + THREAT_RADIUS);
        let shape = if has_source && has_target {
            let target = e
                .call(THREAT_TARGET_POSITION_009F7080, &args![this, at(0x30)])
                .u32();
            let source = e
                .call(THREAT_SOURCE_POSITION_009F7050, &args![this, at(0x3c)])
                .u32();
            e.call(
                SHAPE_OF_TWO_009A4E30,
                &args![at(0x24), source, target, radius, hundred],
            )
            .u32()
        } else if has_source {
            let source = e
                .call(THREAT_SOURCE_POSITION_009F7050, &args![this, at(0x6c)])
                .u32();
            e.call(
                SHAPE_OF_ONE_006915D0,
                &args![at(0x60), source, radius, hundred],
            )
            .u32()
        } else {
            let target = e
                .call(THREAT_TARGET_POSITION_009F7080, &args![this, at(0x9c)])
                .u32();
            e.call(
                SHAPE_OF_ONE_006915D0,
                &args![at(0x90), target, radius, hundred],
            )
            .u32()
        };
        copy_words(e, out.addr(), shape, 9);
    });
}

// Translated from 009f70b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SpectatorThreatInfo::SaveGame` (engine map; `RET 4`): `eThreatLevel`,
/// the time since `iLastPerceivedTime` (`00457fe0` minus it, from a local),
/// `fRadius`, both last known positions (0xC bytes each), both flags, then
/// the form ids of the source and the target.
pub fn spectator_threat_info_save_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    e.call(SAVE_DATA, &args![buffer, base + THREAT_LEVEL, 4u32, 0u32]);
    let now = e.call(CURRENT_TIME_00457FE0, &args![]).u32();
    let since = now.wrapping_sub(e.mem.u32(base + THREAT_LAST_PERCEIVED_TIME));
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), since);
        e.call(SAVE_DATA, &args![buffer, slot, 4u32, 0u32]);
    });
    let fields: [(u32, u32); 5] = [
        (THREAT_RADIUS, 4),
        (THREAT_SOURCE_POSITION, 0xc),
        (THREAT_TARGET_POSITION, 0xc),
        (THREAT_HAS_SOURCE, 1),
        (THREAT_HAS_TARGET, 1),
    ];
    for (offset, size) in fields {
        e.call(SAVE_DATA, &args![buffer, base + offset, size, 0u32]);
    }
    for offset in [THREAT_SOURCE, THREAT_TARGET] {
        let form = e.mem.u32(base + offset);
        e.call(SAVE_FORM_ID, &args![buffer, form, 0u32]);
    }
}

// Translated from 009f7170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SpectatorThreatInfo::LoadGame` (engine map; `RET 4`), the mirror of
/// `SaveGame`: `eThreatLevel`; the time since perceived (into a local)
/// gives `iLastPerceivedTime` as `00457fe0` minus it; `fRadius`, the two
/// positions, the two flags, then the form ids of the source and the target.
pub fn spectator_threat_info_load_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    e.call(LOAD_DATA, &args![buffer, base + THREAT_LEVEL, 4u32]);
    let since = e.with_stack(4, |e, slot| {
        e.call(LOAD_DATA, &args![buffer, slot, 4u32]);
        e.mem.u32(slot.addr())
    });
    let now = e.call(CURRENT_TIME_00457FE0, &args![]).u32();
    e.mem
        .set_u32(base + THREAT_LAST_PERCEIVED_TIME, now.wrapping_sub(since));
    let fields: [(u32, u32); 5] = [
        (THREAT_RADIUS, 4),
        (THREAT_SOURCE_POSITION, 0xc),
        (THREAT_TARGET_POSITION, 0xc),
        (THREAT_HAS_SOURCE, 1),
        (THREAT_HAS_TARGET, 1),
    ];
    for (offset, size) in fields {
        e.call(LOAD_DATA, &args![buffer, base + offset, size]);
    }
    e.call(LOAD_FORM_ID, &args![buffer, base + THREAT_SOURCE]);
    e.call(LOAD_FORM_ID, &args![buffer, base + THREAT_TARGET]);
}

/// A loaded id resolved to an `Actor`: null stays null, else
/// `__RTDynamicCast(004839c0(id), 0, TESForm, Actor, 0)`.
fn resolve_actor(e: &mut Engine, id: u32) -> u32 {
    if id == 0 {
        return 0;
    }
    let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, TYPE_ACTOR, 0u32],
    )
    .u32()
}

// Translated from 009f7230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SpectatorThreatInfo` `InitLoadGame` step (unnamed in the engine
/// map; `RET 4`, the buffer word unread): the loaded source and target ids
/// become `Actor`s (null when absent or of another type), and each one set
/// is marked targeted.
pub fn fn_009f7230(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let base = this.addr();
    let id = e.mem.u32(base + THREAT_SOURCE);
    let source = resolve_actor(e, id);
    e.mem.set_u32(base + THREAT_SOURCE, source);
    let id = e.mem.u32(base + THREAT_TARGET);
    let target = resolve_actor(e, id);
    e.mem.set_u32(base + THREAT_TARGET, target);
    if e.mem.u32(base + THREAT_SOURCE) != 0 {
        let source = e.mem.u32(base + THREAT_SOURCE);
        e.call(SET_TARGETED, &args![source, 1u32]);
    }
    if e.mem.u32(base + THREAT_TARGET) != 0 {
        let target = e.mem.u32(base + THREAT_TARGET);
        e.call(SET_TARGETED, &args![target, 1u32]);
    }
}

// Translated from 009f72f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f72f0 - its FS:[0] exception frame (the unwind states that destroy the package and the members when a later constructor throws) is not translated
/// `SpectatorPackage::SpectatorPackage` (engine map; `ECX` only): the
/// `TESPackage` constructor, the class's vtable, `EventCenter` from the
/// point at `011f426c`, the threat list (`009f8ae0(0x20, 0)`), the debug
/// geometry pointer (`00633c90(0)`), `SetPackType(0x18)`, then
/// `iLastPathRequestTime` 0, `fConversationTimer` and `fEventRadius` 0.0,
/// `bFlagCompleted` 0 and `00984f60(0xF)`. Returns `this`.
pub fn spectator_package_spectator_package(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(PACKAGE_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, SPECTATOR_PACKAGE_VTABLE);
    copy_words(e, base + EVENT_CENTER, POINT_011F426C, 3);
    fn_009f8ae0(e, Ptr::new(base + THREAT_INFO_LIST), 0x20, 0);
    e.call(
        NI_POINTER_CONSTRUCT_00633C90,
        &args![base + DEBUG_GEOMETRY, 0u32],
    );
    e.call(PACKAGE_SET_PACK_TYPE, &args![this, 0x18u32]);
    e.mem.set_u32(base + LAST_PATH_REQUEST_TIME, 0);
    e.mem.set_u32(base + CONVERSATION_TIMER, x87_store32(0.0));
    e.mem.set_u32(base + EVENT_RADIUS, x87_store32(0.0));
    e.mem.set_u8(base + FLAG_COMPLETED, 0);
    e.call(PACKAGE_STEP_00984F60, &args![this, 0xfu32]);
    this
}

// Translated from 009f73e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SpectatorPackage`'s scalar deleting destructor (unnamed in the engine
/// map; `RET 4`): the destructor body `009f7410`, then `00401030(this)`
/// when bit 0 of `flags` is set. Returns `this`.
pub fn fn_009f73e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009f7410(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 009f7410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f7410 - its FS:[0] exception frame (the unwind states that finish destroying the members when one destructor throws) is not translated
/// The `SpectatorPackage` destructor body (unnamed in the engine map;
/// `ECX` only): the class's vtable; a debug geometry (`00559450` of the
/// pointer at +0xB0) with a parent (`009611e0`) is detached from it (the
/// parent's virtual slot +0xE8) and the pointer cleared (`0066b0d0(0)`);
/// then the pointer (`0045cec0`), the threat list (`009f8b10`) and the
/// `TESPackage` part (`006708b0`) are destroyed.
pub fn fn_009f7410(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let pointer = base + DEBUG_GEOMETRY;
    e.mem.set_u32(base, SPECTATOR_PACKAGE_VTABLE);
    if e.call(WORD_AT_0, &args![pointer]).u32() != 0 {
        let geometry = e.call(WORD_AT_0, &args![pointer]).u32();
        if e.call(WORD_AT_18, &args![geometry]).u32() != 0 {
            let geometry = e.call(WORD_AT_0, &args![pointer]).u32();
            let parent = e.call(WORD_AT_18, &args![geometry]).u32();
            let geometry = e.call(WORD_AT_0, &args![pointer]).u32();
            e.vcall(parent, SLOT_DETACH_CHILD, &args![geometry]);
        }
        e.call(NI_POINTER_ASSIGN_0066B0D0, &args![pointer, 0u32]);
    }
    e.call(NI_POINTER_DESTRUCT_0045CEC0, &args![pointer]);
    fn_009f8b10(e, Ptr::new(base + THREAT_INFO_LIST));
    e.call(PACKAGE_DESTRUCT_006708B0, &args![this]);
}

// Translated from 009f7500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): copies `other` into this package:
/// the `TESPackage` part (`00673290`); when `other` is a `SpectatorPackage`
/// (`__RTDynamicCast`), the path request time, the three timers (copied
/// through the x87 stack), `EventCenter`, `bFlagCompleted`, and the threat
/// list sized to `other`'s (`009f8b30(count, 1)`) with each threat copied
/// (`009f7630`; the index compared signed).
pub fn fn_009f7500(e: &mut Engine, this: Ptr, other: Ptr) {
    let base = this.addr();
    e.call(PACKAGE_COPY_00673290, &args![this, other]);
    let source = e
        .call(
            RT_DYNAMIC_CAST,
            &args![other, 0u32, TYPE_TES_FORM, TYPE_SPECTATOR_PACKAGE, 0u32],
        )
        .u32();
    if source == 0 {
        return;
    }
    let time = e.mem.u32(source + LAST_PATH_REQUEST_TIME);
    e.mem.set_u32(base + LAST_PATH_REQUEST_TIME, time);
    for offset in [TIMER, CONVERSATION_TIMER, EVENT_RADIUS] {
        let value = float_copy(e, source + offset);
        e.mem.set_u32(base + offset, value);
    }
    copy_words(e, base + EVENT_CENTER, source + EVENT_CENTER, 3);
    let completed = e.mem.u8(source + FLAG_COMPLETED);
    e.mem.set_u8(base + FLAG_COMPLETED, completed);
    let count = e.call(WORD_AT_8, &args![source + THREAT_INFO_LIST]).u32();
    fn_009f8b30(e, Ptr::new(base + THREAT_INFO_LIST), count, 1);
    let mut index: i32 = 0;
    while index < count as i32 {
        let from = threat_at(e, Ptr::new(source), index as u32);
        let to = threat_at(e, this, index as u32);
        fn_009f7630(e, Ptr::new(to), Ptr::new(from));
        index = index.wrapping_add(1);
    }
}

// Translated from 009f7630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SpectatorThreatInfo`'s assignment (unnamed in the engine map; `RET 4`):
/// the source, target, level and time, `fRadius` (through the x87 stack),
/// both positions, both flags, and the debug geometry pointer
/// (`NiPointer::operator=`). Returns `this`.
pub fn fn_009f7630(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let (to, from) = (this.addr(), other.addr());
    copy_words(e, to, from, 4);
    let radius = float_copy(e, from + THREAT_RADIUS);
    e.mem.set_u32(to + THREAT_RADIUS, radius);
    copy_words(
        e,
        to + THREAT_SOURCE_POSITION,
        from + THREAT_SOURCE_POSITION,
        3,
    );
    copy_words(
        e,
        to + THREAT_TARGET_POSITION,
        from + THREAT_TARGET_POSITION,
        3,
    );
    let has_source = e.mem.u8(from + THREAT_HAS_SOURCE);
    e.mem.set_u8(to + THREAT_HAS_SOURCE, has_source);
    let has_target = e.mem.u8(from + THREAT_HAS_TARGET);
    e.mem.set_u8(to + THREAT_HAS_TARGET, has_target);
    e.call(
        NI_POINTER_COPY,
        &args![to + THREAT_DEBUG_GEOMETRY, from + THREAT_DEBUG_GEOMETRY],
    );
    this
}

/// `FLD dword [p]; FSTP dword [ESP]` twice and a cdecl call: `function(a, b)`
/// over the words at `a` and `b` (pushed `b` first), stored as a `float`.
fn float_pair(e: &mut Engine, function: u32, a: u32, b: u32) -> u32 {
    let first = float_copy(e, a);
    let second = float_copy(e, b);
    let value = e.call(function, &args![first, second]).f64();
    x87_store32(value)
}

/// A threat's source (`009f7050`) or target (`009f7080`) position copied
/// into `scratch`; returns its address.
fn threat_position(e: &mut Engine, threat: u32, source: bool, scratch: u32) -> u32 {
    let function = if source {
        THREAT_SOURCE_POSITION_009F7050
    } else {
        THREAT_TARGET_POSITION_009F7080
    };
    e.call(function, &args![threat, scratch]).u32()
}

/// `009f76e0`'s widening of the bounds by one position: for x, y, z the
/// high bound becomes `00404010(high, position)`, then for x, y, z the low
/// bound `0040ebd0(low, position)`, the position copied anew into each of
/// the six `scratch` points.
fn widen_bounds(e: &mut Engine, threat: u32, source: bool, high: u32, low: u32, scratch: [u32; 6]) {
    for (i, at) in scratch.iter().enumerate() {
        let axis = 4 * (i as u32 % 3);
        let (function, bound) = if i < 3 {
            (MAX_00404010, high)
        } else {
            (MIN_0040EBD0, low)
        };
        let point = threat_position(e, threat, source, *at);
        let value = float_pair(e, function, bound + axis, point + axis);
        e.mem.set_u32(bound + axis, value);
    }
}

// Translated from 009f76e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the event center and radius
/// from the threats. Nothing without threats. The bounds start as the
/// point at `011f426c`; the first threat with a known position sets them
/// (both positions: the larger and the smaller of each coordinate through
/// `00404010` and `0040ebd0`, counting 2; one position: both bounds that
/// position, counting 1); every later threat widens them by each position
/// it has (counting each). With no position at all `bFlagCompleted` is
/// set; otherwise the center is the bounds' midpoint (the high bound,
/// plus the low bound with `0063c8a0`, times 0.5 with `00439180`), the
/// largest squared distance from it to a position (`00439ef0`,
/// `004a7290`; kept when not below) gives `fEventRadius` through
/// `004019d0`, and the center is `EventCenter`. The threat indices are
/// compared signed.
pub fn fn_009f76e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let list = base + THREAT_INFO_LIST;
    if e.call(WORD_AT_8, &args![list]).u32() == 0 {
        return;
    }
    let count = e.call(WORD_AT_8, &args![list]).u32() as i32;
    // The frame from EBP-0x1CC: `at(n)` is EBP-n. The high bound (EBP-0x38),
    // the low bound (EBP-0x2C), the center (EBP-0x14) and the 12-byte
    // temporaries each call uses.
    e.with_stack(0x1cc, |e, frame| {
        let origin = frame.addr();
        let at = move |offset: u32| origin + 0x1cc - offset;
        let (high, low, center) = (at(0x38), at(0x2c), at(0x14));
        let mut positions: u32 = 0;
        copy_words(e, high, POINT_011F426C, 3);
        copy_words(e, low, POINT_011F426C, 3);
        let mut index: i32 = 0;
        while index < count {
            let threat = threat_at(e, this, index as u32);
            let has_source = e.mem.u8(threat + THREAT_HAS_SOURCE) != 0;
            let has_target = e.mem.u8(threat + THREAT_HAS_TARGET) != 0;
            if has_source && has_target {
                // Each coordinate: the target's copy, then the source's;
                // `function(source, target)`.
                let scratch = [
                    (0x54, 0x60),
                    (0x6c, 0x78),
                    (0x84, 0x90),
                    (0x9c, 0xa8),
                    (0xb4, 0xc0),
                    (0xcc, 0xd8),
                ];
                for (i, (target_scratch, source_scratch)) in scratch.iter().enumerate() {
                    let axis = 4 * (i as u32 % 3);
                    let (function, bound) = if i < 3 {
                        (MAX_00404010, high)
                    } else {
                        (MIN_0040EBD0, low)
                    };
                    let target = threat_position(e, threat, false, at(*target_scratch));
                    let source = threat_position(e, threat, true, at(*source_scratch));
                    let value = float_pair(e, function, source + axis, target + axis);
                    e.mem.set_u32(bound + axis, value);
                }
                positions = positions.wrapping_add(2);
                break;
            }
            if has_source {
                let point = threat_position(e, threat, true, at(0xe4));
                copy_words(e, high, point, 3);
                let point = threat_position(e, threat, true, at(0xf0));
                copy_words(e, low, point, 3);
                positions = positions.wrapping_add(1);
                break;
            }
            if has_target {
                let point = threat_position(e, threat, false, at(0xfc));
                copy_words(e, high, point, 3);
                let point = threat_position(e, threat, false, at(0x108));
                copy_words(e, low, point, 3);
                positions = positions.wrapping_add(1);
                break;
            }
            index = index.wrapping_add(1);
        }
        index = index.wrapping_add(1);
        while index < count {
            let threat = threat_at(e, this, index as u32);
            if e.mem.u8(threat + THREAT_HAS_SOURCE) != 0 {
                let scratch = [0x114, 0x120, 0x12c, 0x138, 0x144, 0x150].map(at);
                widen_bounds(e, threat, true, high, low, scratch);
                positions = positions.wrapping_add(1);
            }
            if e.mem.u8(threat + THREAT_HAS_TARGET) != 0 {
                let scratch = [0x15c, 0x168, 0x174, 0x180, 0x18c, 0x198].map(at);
                widen_bounds(e, threat, false, high, low, scratch);
                positions = positions.wrapping_add(1);
            }
            index = index.wrapping_add(1);
        }
        if positions == 0 {
            e.mem.set_u8(base + FLAG_COMPLETED, 1);
            return;
        }
        copy_words(e, center, high, 3);
        e.call(POINT_ADD_IN_PLACE_0063C8A0, &args![center, low]);
        let half = x87_quiet_bits(e.global::<u32>(HALF));
        e.call(POINT_SCALE_IN_PLACE_00439180, &args![center, half]);
        let mut largest = x87_store32(0.0);
        let mut index: i32 = 0;
        while index < count {
            let threat = threat_at(e, this, index as u32);
            let has_source = e.mem.u8(threat + THREAT_HAS_SOURCE) != 0;
            let has_target = e.mem.u8(threat + THREAT_HAS_TARGET) != 0;
            if has_source || has_target {
                if e.mem.u8(threat + THREAT_HAS_SOURCE) != 0 {
                    let point = threat_position(e, threat, true, at(0x1a4));
                    let offset = e
                        .call(POINT_SUBTRACT, &args![center, at(0x1b0), point])
                        .u32();
                    let squared = e.call(POINT_LENGTH_SQUARED, &args![offset]).f64();
                    let squared = x87_store32(squared);
                    // `FLD d; FLD best; FCOMPP; TEST AH,0x41; JP`: kept when
                    // the best is not above it (ordered).
                    if x87_load32(largest) <= x87_load32(squared) {
                        largest = x87_quiet_bits(squared);
                    }
                }
                if e.mem.u8(threat + THREAT_HAS_TARGET) != 0 {
                    let point = threat_position(e, threat, false, at(0x1bc));
                    let offset = e
                        .call(POINT_SUBTRACT, &args![center, at(0x1c8), point])
                        .u32();
                    let squared = e.call(POINT_LENGTH_SQUARED, &args![offset]).f64();
                    let squared = x87_store32(squared);
                    if x87_load32(largest) <= x87_load32(squared) {
                        largest = x87_quiet_bits(squared);
                    }
                }
            }
            index = index.wrapping_add(1);
        }
        copy_words(e, base + EVENT_CENTER, center, 3);
        let radius = e
            .call(RADIUS_OF_004019D0, &args![x87_quiet_bits(largest)])
            .f64();
        e.mem.set_u32(base + EVENT_RADIUS, x87_store32(radius));
    });
}

/// The first threat's target (`009f8aa0`, then `00726070`) when it answers
/// its virtual slot +0x100, else 0.
fn watchable_target(e: &mut Engine, this: Ptr) -> u32 {
    let first = fn_009f8aa0(e, Ptr::new(this.addr() + THREAT_INFO_LIST));
    let target = e.call(WORD_AT_4, &args![first]).u32();
    if target != 0 && e.vcall(target, SLOT_TARGET_WATCHABLE, &args![]).u8() != 0 {
        target
    } else {
        0
    }
}

// Translated from 009f7d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): without `watching`, the actor's
/// process gets its virtual slot +0x310 with 0 and the actor stops
/// (`Actor::StopMoving`). With threats, a roll `00487f50() % 100` and the
/// first threat's watchable target (none: nothing more): when the actor has
/// a process that does not answer its slot +0x30C, the setting at
/// `011cdc04` is above the roll, `fConversationTimer` is at most 0.0 and
/// `TESTopic::GetTopic(6, 0)` gives a topic, the actor runs `0057bd60` with
/// the target, its process's slot +0x2A4 gets (actor, topic, 0, 0, 1, 0)
/// and the timer becomes the setting at `011ccff0`. Then the timer goes
/// down by the game timer's `float` (`0084d030`).
pub fn fn_009f7d70(e: &mut Engine, this: Ptr, actor: Ptr, watching: u8) {
    let base = this.addr();
    let actor = actor.addr();
    if watching == 0 {
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        e.vcall(process, SLOT_PROCESS_0X310, &args![0u32]);
        e.call(ACTOR_STOP_MOVING, &args![actor]);
        return;
    }
    if e.call(WORD_AT_8, &args![base + THREAT_INFO_LIST]).u32() == 0 {
        return;
    }
    let roll = e.call(RANDOM_00487F50, &args![]).u32() % 100;
    let target = watchable_target(e, this);
    if target == 0 {
        return;
    }
    let talks = e.call(ACTOR_PROCESS, &args![actor]).u32() != 0
        && {
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            e.vcall(process, SLOT_PROCESS_0X30C, &args![]).u8() == 0
        }
        && {
            // `FILD roll; FSTP double`, then `FLD setting; FCOMP double;
            // TEST AH,0x41; JNZ`: on when the setting is above the roll.
            let value = e.call(SETTING_VALUE, &args![SETTING_011CDC04]).u32();
            x87_load32(e.mem.u32(value)) > f64::from(roll)
        }
        && {
            // `FCOMP 0.0; TEST AH,0x41; JP`: on when the timer is at most 0.
            x87_load32(e.mem.u32(base + CONVERSATION_TIMER)) <= e.global::<f64>(ZERO)
        };
    if talks {
        let topic = e.call(TOPIC_GET_TOPIC, &args![6u32, 0u32]).u32();
        if topic != 0 {
            e.call(REFERENCE_STEP_0057BD60, &args![actor, target]);
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            e.vcall(
                process,
                SLOT_PROCESS_0X2A4,
                &args![actor, topic, 0u32, 0u32, 1u32, 0u32],
            );
            let value = e.call(SETTING_VALUE, &args![SETTING_011CCFF0]).u32();
            let timer = float_copy(e, value);
            e.mem.set_u32(base + CONVERSATION_TIMER, timer);
        }
    }
    let elapsed = e.call(TIMER_VALUE_0084D030, &args![GAME_TIMER]).f64();
    // `FSUBR [timer]`: the timer minus the elapsed time.
    let timer = x87_sub(x87_m32(e.mem.u32(base + CONVERSATION_TIMER)), elapsed);
    e.mem.set_u32(base + CONVERSATION_TIMER, x87_store32(timer));
}

// Translated from 009f7f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): with `watching`, threats and a
/// watchable first target, the actor's process gets its virtual slot
/// +0x628 with the target; otherwise its slot +0x644 with 1.
pub fn fn_009f7f20(e: &mut Engine, this: Ptr, actor: Ptr, watching: u8) {
    let actor = actor.addr();
    if watching != 0
        && e.call(WORD_AT_8, &args![this.addr() + THREAT_INFO_LIST])
            .u32()
            != 0
    {
        let target = watchable_target(e, this);
        if target != 0 {
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            e.vcall(process, SLOT_PROCESS_0X628, &args![target]);
            return;
        }
    }
    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
    e.vcall(process, SLOT_PROCESS_0X644, &args![1u32]);
}

// Translated from 009f7fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): whether the package is done:
/// no threats, or `bFlagCompleted` set.
pub fn fn_009f7fe0(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.call(WORD_AT_8, &args![base + THREAT_INFO_LIST]).u32() == 0
        || e.mem.u8(base + FLAG_COMPLETED) != 0
}

/// `009f8020`'s test of a threat's source: whether the actor's process
/// slot +0x504 (with the source and 0) gives an object whose word at +8 is
/// above 0 (signed); with no source, a source that does not answer its slot
/// +0x218, or no process, it passes.
fn source_still_fighting(e: &mut Engine, actor: u32, source: u32) -> bool {
    if source == 0 || e.vcall(source, SLOT_ACTOR_0X218, &args![]).u8() == 0 {
        return true;
    }
    if e.call(ACTOR_PROCESS, &args![actor]).u32() == 0 {
        return true;
    }
    let found = if e.call(ACTOR_PROCESS, &args![actor]).u32() != 0 {
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        e.vcall(process, SLOT_PROCESS_0X504, &args![source, 0u32])
            .u32()
    } else {
        0
    };
    found != 0 && (e.mem.u32(found + 8) as i32) > 0
}

// Translated from 009f8020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): updates the threats for `actor`.
/// A threat whose source now targets someone else (slot +0x42C, non-null
/// and not the threat's target) gets a new threat for the actor
/// (`009f8330`). Then, for every threat last perceived more than 2000 ms
/// ago (`00457fe0` against `iLastPerceivedTime` + 0x7D0): one older than
/// the setting at `011cd264` times 1000.0 is removed (`009f8c70(index,
/// 1)`); a target that answers its slot +0x22C (with 0), has form flag
/// 0x20 or answers +0x234 or +0x230 is dropped from it; a source still
/// fighting (see the helper) and still aiming at the target (slot +0x42C)
/// gets the threat refreshed (`009f6d50`). Any change recomputes the event
/// (`009f76e0`); last `004534f0(actor)`. The indices are compared signed.
pub fn fn_009f8020(e: &mut Engine, this: Ptr, actor: Ptr) {
    let base = this.addr();
    let list = base + THREAT_INFO_LIST;
    let actor_word = actor.addr();
    let mut changed = false;
    let count = e.call(WORD_AT_8, &args![list]).u32() as i32;
    let mut index: i32 = 0;
    while index < count {
        let threat = threat_at(e, this, index as u32);
        if e.call(WORD_AT_0, &args![threat]).u32() != 0
            && e.call(WORD_AT_4, &args![threat]).u32() != 0
        {
            let source = e.call(WORD_AT_0, &args![threat]).u32();
            let aimed = e.vcall(source, SLOT_ACTOR_TARGET, &args![]).u32();
            if aimed != 0 && aimed != e.call(WORD_AT_4, &args![threat]).u32() {
                fn_009f8330(e, this, actor);
                changed = true;
            }
        }
        index = index.wrapping_add(1);
    }
    let mut count = e.call(WORD_AT_8, &args![list]).u32() as i32;
    let mut index: i32 = 0;
    while index < count {
        let threat = threat_at(e, this, index as u32);
        let source = e.call(WORD_AT_0, &args![threat]).u32();
        let target = e.call(WORD_AT_4, &args![threat]).u32();
        let now = e.call(CURRENT_TIME_00457FE0, &args![]).u32();
        let last = e.call(WORD_AT_C, &args![threat]).u32();
        if now <= last.wrapping_add(0x7d0) {
            index = index.wrapping_add(1);
            continue;
        }
        // `FILD qword` of the zero-extended words, then
        // `FLD setting; FMUL 1000.0; FADD last; FCOMP now; TEST AH,5; JP`:
        // removed when the deadline is below now (ordered).
        let now = f64::from(e.call(CURRENT_TIME_00457FE0, &args![]).u32());
        let last = f64::from(e.call(WORD_AT_C, &args![threat]).u32());
        let value = e.call(SETTING_VALUE, &args![SETTING_011CD264]).u32();
        let thousand: f64 = e.global(THOUSAND);
        let deadline = x87_add(x87_mul(x87_load32(e.mem.u32(value)), thousand), last);
        if deadline < now {
            fn_009f8c70(e, Ptr::new(list), index as u32, 1);
            count = count.wrapping_sub(1);
            changed = true;
            // The index steps back one and the loop steps it on again.
            continue;
        }
        if target != 0 {
            let gone = e.vcall(target, SLOT_ACTOR_0X22C, &args![0u32]).u8() != 0
                || e.call(FORM_FLAG_20_00440D80, &args![target]).u8() != 0
                || e.vcall(target, SLOT_ACTOR_0X234, &args![]).u8() != 0
                || e.vcall(target, SLOT_ACTOR_0X230, &args![]).u8() != 0;
            if gone {
                e.mem.set_u32(threat + THREAT_TARGET, 0);
                changed = true;
                index = index.wrapping_add(1);
                continue;
            }
        }
        if !source_still_fighting(e, actor_word, source) {
            index = index.wrapping_add(1);
            continue;
        }
        if source != 0
            && target != 0
            && e.vcall(source, SLOT_ACTOR_TARGET, &args![]).u32() != target
        {
            index = index.wrapping_add(1);
            continue;
        }
        changed = true;
        e.call(THREAT_REFRESH_009F6D50, &args![threat]);
        index = index.wrapping_add(1);
    }
    if changed {
        fn_009f76e0(e, this);
    }
    e.call(PACKAGE_UPDATE_004534F0, &args![this, actor]);
}

// Translated from 009f8300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): copies `EventCenter` into `out`
/// and returns `out`.
pub fn fn_009f8300(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    copy_words(e, out.addr(), this.addr() + EVENT_CENTER, 3);
    out
}

// Translated from 009f8330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): records `source` as a threat. The
/// threat with this source and its current target (slot +0x42C) is reused;
/// otherwise the list grows by one (`009f8b30(count + 1, 1)`) and the new
/// last threat is used. It is made about the source (`009f6c20`) and the
/// event recomputed (`009f76e0`). The indices are compared signed.
pub fn fn_009f8330(e: &mut Engine, this: Ptr, source: Ptr) {
    let base = this.addr();
    let list = base + THREAT_INFO_LIST;
    let target = e.vcall(source.addr(), SLOT_ACTOR_TARGET, &args![]).u32();
    let count = e.call(WORD_AT_8, &args![list]).u32() as i32;
    let mut found = 0u32;
    let mut index: i32 = 0;
    while index < count {
        let threat = threat_at(e, this, index as u32);
        if e.call(WORD_AT_0, &args![threat]).u32() == source.addr()
            && e.call(WORD_AT_4, &args![threat]).u32() == target
        {
            found = threat;
            break;
        }
        index = index.wrapping_add(1);
    }
    if found == 0 {
        let size = e.call(WORD_AT_8, &args![list]).u32();
        fn_009f8b30(e, Ptr::new(list), size.wrapping_add(1), 1);
        found = threat_at(e, this, size);
    }
    fn_009f6c20(e, Ptr::new(found), source);
    fn_009f76e0(e, this);
}

// Translated from 009f8420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): forgets `actor`: threats from it
/// are removed (`009f8c70(index, 1)`), threats at it lose their target
/// (`pTarget` 0, `bHasTarget` 0). The indices are compared signed.
pub fn fn_009f8420(e: &mut Engine, this: Ptr, actor: Ptr) {
    let list = this.addr() + THREAT_INFO_LIST;
    let mut count = e.call(WORD_AT_8, &args![list]).u32() as i32;
    let mut index: i32 = 0;
    while index < count {
        let threat = threat_at(e, this, index as u32);
        if e.call(WORD_AT_0, &args![threat]).u32() == actor.addr() {
            fn_009f8c70(e, Ptr::new(list), index as u32, 1);
            count = count.wrapping_sub(1);
            index = index.wrapping_sub(1);
        } else if e.call(WORD_AT_4, &args![threat]).u32() == actor.addr() {
            e.mem.set_u32(threat + THREAT_TARGET, 0);
            e.mem.set_u8(threat + THREAT_HAS_TARGET, 0);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 009f84d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): removes every threat from or at
/// `actor` (`009f8c70(index, 1)`; the indices compared signed).
pub fn fn_009f84d0(e: &mut Engine, this: Ptr, actor: Ptr) {
    let list = this.addr() + THREAT_INFO_LIST;
    let mut count = e.call(WORD_AT_8, &args![list]).u32() as i32;
    let mut index: i32 = 0;
    while index < count {
        let threat = threat_at(e, this, index as u32);
        if e.call(WORD_AT_0, &args![threat]).u32() == actor.addr()
            || e.call(WORD_AT_4, &args![threat]).u32() == actor.addr()
        {
            fn_009f8c70(e, Ptr::new(list), index as u32, 1);
            count = count.wrapping_sub(1);
            index = index.wrapping_sub(1);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 009f8570 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f8570 - its FS:[0] exception frame (the unwind state that frees the new array when its constructor throws) is not translated
/// Unnamed in the engine map (`ECX` only): a new 0x18-byte array
/// (`00aa13e0`, constructed by `006e3850`; null when the allocation fails)
/// holding the shape of every threat: for each, the array grows by one
/// (`006e3ad0(size + 1, 1)`) and `009f6f30` fills its last element
/// (`006a1400`). Returns the array.
pub fn fn_009f8570(e: &mut Engine, this: Ptr) -> Ptr {
    let block = e.call(ALLOCATE_00AA13E0, &args![0x18u32]).u32();
    let array = if block != 0 {
        e.call(SHAPE_ARRAY_CONSTRUCT_006E3850, &args![block]).u32()
    } else {
        0
    };
    let count = e
        .call(WORD_AT_8, &args![this.addr() + THREAT_INFO_LIST])
        .u32() as i32;
    let mut index: i32 = 0;
    while index < count {
        let threat = threat_at(e, this, index as u32);
        let size = e.call(WORD_AT_8, &args![array]).u32();
        e.call(
            SHAPE_ARRAY_SET_SIZE_006E3AD0,
            &args![array, size.wrapping_add(1), 1u32],
        );
        let element = e.call(SHAPE_ARRAY_AT_006A1400, &args![array, size]).u32();
        fn_009f6f30(e, Ptr::new(threat), Ptr::new(element));
        index = index.wrapping_add(1);
    }
    Ptr::new(array)
}

// Translated from 009f8660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SpectatorPackage::CreateThreatArray` (engine map; `RET 4`): empties
/// `out` (`008454f0(1)`), reserves twice the threat count, and appends
/// every threat's non-null source (`007cb2e0` with the address of a local
/// holding it; the indices compared signed).
pub fn spectator_package_create_threat_array(e: &mut Engine, this: Ptr, out: Ptr) {
    e.call(ARRAY_RELEASE_008454F0, &args![out, 1u32]);
    let count = e
        .call(WORD_AT_8, &args![this.addr() + THREAT_INFO_LIST])
        .u32();
    e.call(ARRAY_SET_RESERVED_SIZE, &args![out, count << 1]);
    e.with_stack(4, |e, slot| {
        let mut index: i32 = 0;
        while index < count as i32 {
            let threat = threat_at(e, this, index as u32);
            let source = e.call(WORD_AT_0, &args![threat]).u32();
            e.mem.set_u32(slot.addr(), source);
            if source != 0 {
                e.call(ARRAY_ADD, &args![out, slot]);
            }
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 009f86f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the save size, the `TESPackage`
/// part's (`00678fd0`, a halfword) plus 8, as a halfword.
pub fn fn_009f86f0(e: &mut Engine, this: Ptr) -> u16 {
    let size = e.call(PACKAGE_SAVE_SIZE_00678FD0, &args![this]).u32() as u16;
    size.wrapping_add(8)
}

// Translated from 009f8730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the `TESPackage` part
/// (`00679120`), then `fTimer` and `fConversationTimer` (four bytes each,
/// `TESForm::SaveGameDataOLD`).
pub fn fn_009f8730(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(PACKAGE_SAVE_00679120, &args![this]);
    e.call(FORM_SAVE_GAME_DATA_OLD, &args![this, base + TIMER, 4u32]);
    e.call(
        FORM_SAVE_GAME_DATA_OLD,
        &args![this, base + CONVERSATION_TIMER, 4u32],
    );
}

// Translated from 009f8770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): `TESPackage::LoadGame`, then
/// `fTimer` and `fConversationTimer` (`TESForm::LoadGameDataOLD`).
pub fn fn_009f8770(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(PACKAGE_LOAD_GAME, &args![this]);
    e.call(FORM_LOAD_GAME_DATA_OLD, &args![this, base + TIMER, 4u32]);
    e.call(
        FORM_LOAD_GAME_DATA_OLD,
        &args![this, base + CONVERSATION_TIMER, 4u32],
    );
}

// Translated from 009f87b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): forwards to the `TESPackage`
/// step `006796c0`.
pub fn fn_009f87b0(e: &mut Engine, this: Ptr) {
    e.call(PACKAGE_STEP_006796C0, &args![this]);
}

// Translated from 009f87d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SpectatorPackage::SaveGame` (engine map; `RET 4`):
/// `TESPackage::SaveGame`, the time since the last path request (now minus
/// `iLastPathRequestTime`, from a local), the three timers, the completed
/// flag and `EventCenter`; then the threat count (0 when the list's address
/// is null) and each threat (`SpectatorThreatInfo::SaveGame`).
pub fn spectator_package_save_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    e.call(PACKAGE_SAVE_GAME, &args![this, buffer]);
    let now = e.call(TIMER_NOW, &args![GAME_TIMER]).u32();
    let since = now.wrapping_sub(e.mem.u32(base + LAST_PATH_REQUEST_TIME));
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), since);
        e.call(SAVE_DATA, &args![buffer, slot, 4u32, 0u32]);
    });
    let fields: [(u32, u32); 5] = [
        (TIMER, 4),
        (CONVERSATION_TIMER, 4),
        (EVENT_RADIUS, 4),
        (FLAG_COMPLETED, 1),
        (EVENT_CENTER, 0xc),
    ];
    for (offset, size) in fields {
        e.call(SAVE_DATA, &args![buffer, base + offset, size, 0u32]);
    }
    let list = base.wrapping_add(THREAT_INFO_LIST);
    let count = if list != 0 {
        e.call(WORD_AT_8, &args![list]).u32()
    } else {
        0
    };
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count]);
    let mut index = 0u32;
    while index < count {
        let threat = threat_at(e, this, index);
        spectator_threat_info_save_game(e, Ptr::new(threat), buffer);
        index = index.wrapping_add(1);
    }
}

// Translated from 009f8900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SpectatorPackage::LoadGame` (engine map; `RET 4`), the mirror of
/// `SaveGame`: `TESPackage::LoadGame_ov2`, the time since the last path
/// request (into a local) gives `iLastPathRequestTime` as now minus it, the
/// three timers, the flag and `EventCenter`; the threat count, the list
/// sized to it (`009f8b30(count, 1)`) and each threat loaded.
pub fn spectator_package_load_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    e.call(PACKAGE_LOAD_GAME_OV2, &args![this, buffer]);
    let since = e.with_stack(4, |e, slot| {
        e.call(LOAD_DATA, &args![buffer, slot, 4u32]);
        e.mem.u32(slot.addr())
    });
    let now = e.call(TIMER_NOW, &args![GAME_TIMER]).u32();
    e.mem
        .set_u32(base + LAST_PATH_REQUEST_TIME, now.wrapping_sub(since));
    let fields: [(u32, u32); 5] = [
        (TIMER, 4),
        (CONVERSATION_TIMER, 4),
        (EVENT_RADIUS, 4),
        (FLAG_COMPLETED, 1),
        (EVENT_CENTER, 0xc),
    ];
    for (offset, size) in fields {
        e.call(LOAD_DATA, &args![buffer, base + offset, size]);
    }
    let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    fn_009f8b30(e, Ptr::new(base + THREAT_INFO_LIST), count, 1);
    let mut index = 0u32;
    while index < count {
        let threat = threat_at(e, this, index);
        spectator_threat_info_load_game(e, Ptr::new(threat), buffer);
        index = index.wrapping_add(1);
    }
}

// Translated from 009f8a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): `TESPackage::InitLoadGame`, then
/// each threat's `InitLoadGame` step (`009f7230`).
pub fn fn_009f8a10(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(PACKAGE_INIT_LOAD_GAME, &args![this, buffer]);
    let count = e
        .call(WORD_AT_8, &args![this.addr() + THREAT_INFO_LIST])
        .u32();
    let mut index = 0u32;
    while index < count {
        let threat = threat_at(e, this, index);
        fn_009f7230(e, Ptr::new(threat), buffer.addr());
        index = index.wrapping_add(1);
    }
}

// Translated from 009f8aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the address of the first
/// threat (`009f8ac0(0)`).
pub fn fn_009f8aa0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_009f8ac0(e, this, 0)
}

// Translated from 009f8ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): the address of threat `index`:
/// the buffer (+4) plus `index` times 0x34.
pub fn fn_009f8ac0(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    let buffer = e.mem.u32(this.addr() + ARRAY_BUFFER);
    Ptr::new(index.wrapping_mul(THREAT_SIZE).wrapping_add(buffer))
}

// Translated from 009f8ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<SpectatorThreatInfo,1024>` constructor (unnamed in
/// the engine map; `RET 8`): the vtable, then `009f90f0(alloc_size,
/// size)`. Returns `this`.
pub fn fn_009f8ae0(e: &mut Engine, this: Ptr, alloc_size: u32, size: u32) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01093980);
    fn_009f90f0(e, this, alloc_size, size);
    this
}

// Translated from 009f8b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<SpectatorThreatInfo,1024>`'s `SetSize` (unnamed in the
/// engine map; `RET 8`; items of 0x34 bytes). Size 0 releases the array
/// (`009f8d00(shrink)`). Above the reserved size the buffer is allocated
/// (virtual slot +4) when there is none, else reallocated (`009f9040(size,
/// old size)`), the new threats constructed (`009f8de0`) and the size set.
/// Below the current size the extra threats are destroyed (`009f8f60`), the
/// size set, and with `shrink` and a size at most a quarter of the reserved
/// one the buffer is reallocated to the size. Otherwise the new threats are
/// constructed and the size set.
pub fn fn_009f8b30(e: &mut Engine, this: Ptr, size: u32, shrink: u8) {
    let base = this.addr();
    if size == 0 {
        fn_009f8d00(e, this, shrink);
        return;
    }
    let item = |e: &Engine, index: u32| {
        index
            .wrapping_mul(THREAT_SIZE)
            .wrapping_add(e.mem.u32(base + ARRAY_BUFFER))
    };
    if size > e.mem.u32(base + ARRAY_ALLOC_SIZE) {
        if e.mem.u32(base + ARRAY_ALLOC_SIZE) == 0 {
            let buffer = e.vcall(base, SLOT_ARRAY_ALLOCATE, &args![size]).u32();
            e.mem.set_u32(base + ARRAY_BUFFER, buffer);
            e.mem.set_u32(base + ARRAY_ALLOC_SIZE, size);
        } else {
            let old = e.mem.u32(base + ARRAY_SIZE);
            fn_009f9040(e, this, size, old);
            e.mem.set_u32(base + ARRAY_ALLOC_SIZE, size);
        }
        let old = e.mem.u32(base + ARRAY_SIZE);
        let first = item(e, old);
        e.call(
            THREATS_CONSTRUCT_009F8DE0,
            &args![this, first, size.wrapping_sub(old)],
        );
        e.mem.set_u32(base + ARRAY_SIZE, size);
    } else if size < e.mem.u32(base + ARRAY_SIZE) {
        let old = e.mem.u32(base + ARRAY_SIZE);
        let first = item(e, size);
        fn_009f8f60(e, this, Ptr::new(first), old.wrapping_sub(size));
        e.mem.set_u32(base + ARRAY_SIZE, size);
        if shrink != 0 && size <= e.mem.u32(base + ARRAY_ALLOC_SIZE) >> 2 {
            fn_009f9040(e, this, size, size);
            e.mem.set_u32(base + ARRAY_ALLOC_SIZE, size);
        }
    } else {
        let old = e.mem.u32(base + ARRAY_SIZE);
        let first = item(e, old);
        e.call(
            THREATS_CONSTRUCT_009F8DE0,
            &args![this, first, size.wrapping_sub(old)],
        );
        e.mem.set_u32(base + ARRAY_SIZE, size);
    }
}

// Translated from 009f8c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<SpectatorThreatInfo,1024>`'s `RemoveAt` (unnamed in the
/// engine map; `RET 8`): destroys `count` threats from `start`
/// (`009f8f60`), moves the `0042f5a0(size - start - count, count)` threats
/// from the end into the gap (`009f8fa0`) and lowers the size by `count`.
pub fn fn_009f8c70(e: &mut Engine, this: Ptr, start: u32, count: u32) {
    let base = this.addr();
    let size = e.mem.u32(base + ARRAY_SIZE);
    let moved = e
        .call(
            MOVE_COUNT_0042F5A0,
            &args![size.wrapping_sub(start).wrapping_sub(count), count],
        )
        .u32();
    let gap = start
        .wrapping_mul(THREAT_SIZE)
        .wrapping_add(e.mem.u32(base + ARRAY_BUFFER));
    fn_009f8f60(e, this, Ptr::new(gap), count);
    let size = e.mem.u32(base + ARRAY_SIZE);
    let source = e
        .mem
        .u32(base + ARRAY_BUFFER)
        .wrapping_add(size.wrapping_mul(THREAT_SIZE))
        .wrapping_sub(moved.wrapping_mul(THREAT_SIZE));
    let gap = start
        .wrapping_mul(THREAT_SIZE)
        .wrapping_add(e.mem.u32(base + ARRAY_BUFFER));
    e.call(THREATS_MOVE_009F8FA0, &args![this, gap, source, moved]);
    let size = e.mem.u32(base + ARRAY_SIZE);
    e.mem.set_u32(base + ARRAY_SIZE, size.wrapping_sub(count));
}

// Translated from 009f8d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<SpectatorThreatInfo,1024>` release (unnamed in the
/// engine map; `RET 4`): with a buffer, its threats are destroyed
/// (`009f8f60`), with `free` the buffer is freed (`006a8500`) and the
/// reserved size cleared, and the size becomes 0; without one nothing
/// happens.
pub fn fn_009f8d00(e: &mut Engine, this: Ptr, free: u8) {
    let base = this.addr();
    if e.mem.u32(base + ARRAY_BUFFER) == 0 {
        return;
    }
    let buffer = e.mem.u32(base + ARRAY_BUFFER);
    let size = e.mem.u32(base + ARRAY_SIZE);
    fn_009f8f60(e, this, Ptr::new(buffer), size);
    if free != 0 {
        e.call(ARRAY_FREE_BUFFER_006A8500, &args![this]);
        e.mem.set_u32(base + ARRAY_BUFFER, 0);
        e.mem.set_u32(base + ARRAY_ALLOC_SIZE, 0);
    }
    e.mem.set_u32(base + ARRAY_SIZE, 0);
}

// Translated from 009f8d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<SpectatorThreatInfo,1024>::_Allocate` (engine map;
/// `RET 4`): `00401000(count * 0x34)`.
pub fn bs_simple_array_spectator_threat_info_allocate(
    e: &mut Engine,
    _this: Ptr,
    count: u32,
) -> u32 {
    e.call(ALLOCATE, &args![count.wrapping_mul(THREAT_SIZE)])
        .u32()
}

// Translated from 009f8d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<SpectatorThreatInfo,1024>::_Reallocate` (engine map;
/// `RET 8`): `0042f5d0(buffer, count * 0x34)`.
pub fn bs_simple_array_spectator_threat_info_reallocate(
    e: &mut Engine,
    _this: Ptr,
    buffer: u32,
    count: u32,
) -> u32 {
    e.call(
        REALLOCATE_0042F5D0,
        &args![buffer, count.wrapping_mul(THREAT_SIZE)],
    )
    .u32()
}

// Translated from 009f8db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<SpectatorThreatInfo,1024>` scalar deleting destructor
/// (engine map, from the vtable; `RET 4`): the destructor body `009f8b10`,
/// then `00401030(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_spectator_threat_info_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_009f8b10(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 009f8f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The threat destroy step (unnamed in the engine map; `RET 8`): each of
/// the `count` threats from `first` gets `009f9180(threat, 0)`.
pub fn fn_009f8f60(e: &mut Engine, _this: Ptr, first: Ptr, count: u32) {
    let mut index = 0u32;
    while index < count {
        let threat = index.wrapping_mul(THREAT_SIZE).wrapping_add(first.addr());
        e.call(THREAT_DELETE_009F9180, &args![threat, 0u32]);
        index = index.wrapping_add(1);
    }
}

// Translated from 009f9040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The threat buffer reallocation (unnamed in the engine map; `RET 8`):
/// with a buffer, a `count` equal to the reserved size reallocates it in
/// place (virtual slot +0xC with the buffer and `alloc_size`); otherwise a
/// new buffer (slot +4) receives the `count` threats (`009f8fa0`), the old
/// one is freed (`006a8500`) and the new one kept. Without a buffer one is
/// allocated (slot +4) and the reserved size becomes `alloc_size`.
pub fn fn_009f9040(e: &mut Engine, this: Ptr, alloc_size: u32, count: u32) {
    let base = this.addr();
    if e.mem.u32(base + ARRAY_BUFFER) == 0 {
        let buffer = e.vcall(base, SLOT_ARRAY_ALLOCATE, &args![alloc_size]).u32();
        e.mem.set_u32(base + ARRAY_BUFFER, buffer);
        e.mem.set_u32(base + ARRAY_ALLOC_SIZE, alloc_size);
        return;
    }
    if count == e.mem.u32(base + ARRAY_ALLOC_SIZE) {
        let buffer = e.mem.u32(base + ARRAY_BUFFER);
        let buffer = e
            .vcall(base, SLOT_ARRAY_REALLOCATE, &args![buffer, alloc_size])
            .u32();
        e.mem.set_u32(base + ARRAY_BUFFER, buffer);
    } else {
        let fresh = e.vcall(base, SLOT_ARRAY_ALLOCATE, &args![alloc_size]).u32();
        let buffer = e.mem.u32(base + ARRAY_BUFFER);
        e.call(THREATS_MOVE_009F8FA0, &args![this, fresh, buffer, count]);
        e.call(ARRAY_FREE_BUFFER_006A8500, &args![this]);
        e.mem.set_u32(base + ARRAY_BUFFER, fresh);
    }
}

// Translated from 009f90f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<SpectatorThreatInfo,1024>` initialization (unnamed in
/// the engine map; `RET 8`): no buffer, size or reserved size; the reserved
/// size is at least `size`; a positive one allocates the buffer (virtual
/// slot +4); a positive `size` constructs that many threats (`009f8de0`)
/// and becomes the size.
pub fn fn_009f90f0(e: &mut Engine, this: Ptr, alloc_size: u32, size: u32) {
    let base = this.addr();
    e.mem.set_u32(base + ARRAY_BUFFER, 0);
    e.mem.set_u32(base + ARRAY_SIZE, 0);
    e.mem.set_u32(base + ARRAY_ALLOC_SIZE, 0);
    let mut alloc_size = alloc_size;
    if size > alloc_size {
        alloc_size = size;
    }
    if alloc_size > 0 {
        let buffer = e.vcall(base, SLOT_ARRAY_ALLOCATE, &args![alloc_size]).u32();
        e.mem.set_u32(base + ARRAY_BUFFER, buffer);
        e.mem.set_u32(base + ARRAY_ALLOC_SIZE, alloc_size);
    }
    if size > 0 {
        let buffer = e.mem.u32(base + ARRAY_BUFFER);
        e.call(THREATS_CONSTRUCT_009F8DE0, &args![this, buffer, size]);
        e.mem.set_u32(base + ARRAY_SIZE, size);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009f_6c20, fn_009f6c20(Ptr, Ptr)),
        entry!(0x009f_6f30, fn_009f6f30(Ptr, Ptr)),
        entry!(0x009f_70b0, spectator_threat_info_save_game(Ptr, Ptr)),
        entry!(0x009f_7170, spectator_threat_info_load_game(Ptr, Ptr)),
        entry!(0x009f_7230, fn_009f7230(Ptr, u32)),
        entry!(0x009f_72f0, spectator_package_spectator_package(Ptr) -> Ptr),
        entry!(0x009f_73e0, fn_009f73e0(Ptr, u32) -> Ptr),
        entry!(0x009f_7410, fn_009f7410(Ptr)),
        entry!(0x009f_7500, fn_009f7500(Ptr, Ptr)),
        entry!(0x009f_7630, fn_009f7630(Ptr, Ptr) -> Ptr),
        entry!(0x009f_76e0, fn_009f76e0(Ptr)),
        entry!(0x009f_7d70, fn_009f7d70(Ptr, Ptr, u8)),
        entry!(0x009f_7f20, fn_009f7f20(Ptr, Ptr, u8)),
        entry!(0x009f_7fe0, fn_009f7fe0(Ptr) -> bool),
        entry!(0x009f_8020, fn_009f8020(Ptr, Ptr)),
        entry!(0x009f_8300, fn_009f8300(Ptr, Ptr) -> Ptr),
        entry!(0x009f_8330, fn_009f8330(Ptr, Ptr)),
        entry!(0x009f_8420, fn_009f8420(Ptr, Ptr)),
        entry!(0x009f_84d0, fn_009f84d0(Ptr, Ptr)),
        entry!(0x009f_8570, fn_009f8570(Ptr) -> Ptr),
        entry!(0x009f_8660, spectator_package_create_threat_array(Ptr, Ptr)),
        entry!(0x009f_86f0, fn_009f86f0(Ptr) -> u16),
        entry!(0x009f_8730, fn_009f8730(Ptr)),
        entry!(0x009f_8770, fn_009f8770(Ptr)),
        entry!(0x009f_87b0, fn_009f87b0(Ptr)),
        entry!(0x009f_87d0, spectator_package_save_game(Ptr, Ptr)),
        entry!(0x009f_8900, spectator_package_load_game(Ptr, Ptr)),
        entry!(0x009f_8a10, fn_009f8a10(Ptr, Ptr)),
        entry!(0x009f_8aa0, fn_009f8aa0(Ptr) -> Ptr),
        entry!(0x009f_8ac0, fn_009f8ac0(Ptr, u32) -> Ptr),
        entry!(0x009f_8ae0, fn_009f8ae0(Ptr, u32, u32) -> Ptr),
        entry!(0x009f_8b30, fn_009f8b30(Ptr, u32, u8)),
        entry!(0x009f_8c70, fn_009f8c70(Ptr, u32, u32)),
        entry!(0x009f_8d00, fn_009f8d00(Ptr, u8)),
        entry!(
            0x009f_8d60,
            bs_simple_array_spectator_threat_info_allocate(Ptr, u32) -> u32
        ),
        entry!(
            0x009f_8d90,
            bs_simple_array_spectator_threat_info_reallocate(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x009f_8db0,
            bs_simple_array_spectator_threat_info_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x009f_8f60, fn_009f8f60(Ptr, Ptr, u32)),
        entry!(0x009f_9040, fn_009f9040(Ptr, u32, u32)),
        entry!(0x009f_90f0, fn_009f90f0(Ptr, u32, u32)),
        entry!(0x009f_8a80, fn_009f8a80(Ptr, u32) -> Ptr),
        entry!(
            0x0084b5c0,
            bs_simple_array_set_reserved_size(Ptr<BSSimpleArray>, u32)
        ),
        entry!(0x009f_8b10, fn_009f8b10(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn world() -> (Engine, Ptr<BSSimpleArray>) {
        let mut e = Engine::new();
        e.register(DESTROY_ELEMENTS, |_, _| Ret::default());
        e.register(RESIZE_BUFFER, |_, _| Ret::default());
        let this = e.new_object::<BSSimpleArray>();
        e.set(this, BSSimpleArray::pBuffer, 0x5000);
        e.set(this, BSSimpleArray::iSize, 6);
        e.set(this, BSSimpleArray::iReservedSize, 8);
        (e, this)
    }

    #[test]
    fn an_unchanged_reserved_size_does_nothing() {
        let (mut e, this) = world();
        e.call_log = Some(vec![]);
        e.call(0x0084b5c0, &args![this, 8u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1, "only the call itself");
        assert_eq!(e.get(this, BSSimpleArray::iSize), 6);
    }

    #[test]
    fn shrinking_below_the_size_destroys_the_tail() {
        let (mut e, this) = world();
        e.call_log = Some(vec![]);
        e.call(0x0084b5c0, &args![this, 4u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (DESTROY_ELEMENTS, vec![this.addr(), 0x5010, 2]));
        assert_eq!(log[2], (RESIZE_BUFFER, vec![this.addr(), 4, 4]));
        assert_eq!(e.get(this, BSSimpleArray::iSize), 4);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 4);
    }

    #[test]
    fn growing_keeps_the_elements() {
        let (mut e, this) = world();
        e.call_log = Some(vec![]);
        e.call(0x0084b5c0, &args![this, 20u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[1], (RESIZE_BUFFER, vec![this.addr(), 20, 6]));
        assert_eq!(e.get(this, BSSimpleArray::iSize), 6);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 20);
    }

    #[test]
    fn a_new_size_equal_to_the_size_destroys_nothing() {
        let (mut e, this) = world();
        e.call_log = Some(vec![]);
        e.call(0x0084b5c0, &args![this, 6u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2, "no destroy step when new size == iSize");
        assert_eq!(log[1], (RESIZE_BUFFER, vec![this.addr(), 6, 6]));
    }

    #[test]
    fn forwarder_fn_009f8a80_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(ELEMENT_TARGET_009F8AC0, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x009f_8a80, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == ELEMENT_TARGET_009F8AC0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_009f8b10_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x009f_8b10, ARRAY_VTABLE_01093980, ARRAY_RELEASE_009F8D00);
    }
}

#[cfg(test)]
mod spectator_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
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

    /// An object whose vtable has `slots` (byte offset, target); `size`
    /// bytes, the vtable at +0.
    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let length = slots.iter().map(|(s, _)| s + 4).max().unwrap_or(4);
        let vtable = e.mem.alloc(length);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    const ALLOCATE_SLOT: u32 = 0x0670_0004;
    const FREE_SLOT: u32 = 0x0670_0008;
    const REALLOCATE_SLOT: u32 = 0x0670_000c;

    /// The threat list's own doubles: the array vtable at `01093980` (slots
    /// +4 allocate, +8 free, +0xC reallocate), the word getters, and the
    /// outside threat steps (construct, delete, move) over plain memory.
    fn threat_world(e: &mut Engine) {
        e.put_vtable(
            ARRAY_VTABLE_01093980,
            &[0, ALLOCATE_SLOT, FREE_SLOT, REALLOCATE_SLOT],
        );
        e.register(ALLOCATE_SLOT, |e, a| {
            e.mem.alloc(a[1] * THREAT_SIZE + 4).into_ret()
        });
        e.register(FREE_SLOT, nothing);
        // Reallocation keeps the first four threats (all a test holds then;
        // 0x34 words).
        e.register(REALLOCATE_SLOT, |e, a| {
            let fresh = e.mem.alloc(a[2] * THREAT_SIZE + 4);
            for i in 0..THREAT_SIZE {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(fresh + 4 * i, word);
            }
            fresh.into_ret()
        });
        e.register(WORD_AT_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(WORD_AT_0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(WORD_AT_4, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(WORD_AT_C, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(THREATS_CONSTRUCT_009F8DE0, |e, a| {
            for i in 0..(a[2] * THREAT_SIZE / 4) {
                e.mem.set_u32(a[1] + 4 * i, 0);
            }
            Ret::default()
        });
        e.register(THREAT_DELETE_009F9180, |_, a| a[0].into_ret());
        e.register(THREATS_MOVE_009F8FA0, |e, a| {
            let words: Vec<u32> = (0..(a[3] * THREAT_SIZE / 4))
                .map(|i| e.mem.u32(a[2] + 4 * i))
                .collect();
            for (i, w) in words.iter().enumerate() {
                e.mem.set_u32(a[1] + 4 * i as u32, *w);
            }
            Ret::default()
        });
        e.register(MOVE_COUNT_0042F5A0, |_, a| a[0].min(a[1]).into_ret());
        e.register(ARRAY_FREE_BUFFER_006A8500, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
    }

    /// A `SpectatorPackage` (0xB4 bytes) whose threat list holds `threats`
    /// (source, target, has source, has target); returns it.
    fn package(e: &mut Engine, threats: &[(u32, u32, u8, u8)]) -> u32 {
        let this = e.mem.alloc(0xb4);
        let list = this + THREAT_INFO_LIST;
        e.mem.set_u32(list, ARRAY_VTABLE_01093980);
        let buffer = e.mem.alloc(THREAT_SIZE * 8);
        e.mem.set_u32(list + 4, buffer);
        e.mem.set_u32(list + 8, threats.len() as u32);
        e.mem.set_u32(list + 0xc, 8);
        for (i, (source, target, has_source, has_target)) in threats.iter().enumerate() {
            let threat = buffer + THREAT_SIZE * i as u32;
            e.mem.set_u32(threat + THREAT_SOURCE, *source);
            e.mem.set_u32(threat + THREAT_TARGET, *target);
            e.mem.set_u8(threat + THREAT_HAS_SOURCE, *has_source);
            e.mem.set_u8(threat + THREAT_HAS_TARGET, *has_target);
        }
        this
    }

    fn threat(e: &Engine, this: u32, index: u32) -> u32 {
        e.mem.u32(this + THREAT_INFO_LIST + 4) + THREAT_SIZE * index
    }

    fn set_point(e: &mut Engine, at: u32, point: [f32; 3]) {
        for (i, v) in point.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    fn point(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    #[test]
    fn at_009f6c20_makes_the_threat_about_the_source() {
        for target in [0x7777u32, 0] {
            let mut e = Engine::new();
            e.register(SET_TARGETED, nothing);
            e.register(THREAT_REFRESH_009F6D50, nothing);
            e.register_double(0x0671_0000, move |_, _| target.into_ret());
            let source = object(&mut e, 0x10, &[(SLOT_ACTOR_TARGET, 0x0671_0000)]);
            let threat = e.mem.alloc(THREAT_SIZE);
            e.call_log = Some(vec![]);
            e.call(0x009f_6c20, &args![threat, source]);
            assert_eq!(e.mem.u32(threat + THREAT_SOURCE), source);
            assert_eq!(e.mem.u32(threat + THREAT_TARGET), target);
            assert_eq!(e.mem.u8(threat + THREAT_HAS_SOURCE), 1);
            assert_eq!(e.mem.u8(threat + THREAT_HAS_TARGET), u8::from(target != 0));
            let mut marked = vec![vec![source, 1]];
            if target != 0 {
                marked.push(vec![target, 1]);
            }
            assert_eq!(calls_to(&e, SET_TARGETED), marked);
            assert_eq!(calls_to(&e, THREAT_REFRESH_009F6D50), vec![vec![threat]]);
        }
    }

    #[test]
    fn at_009f6f30_shapes_by_the_known_positions() {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.set_global(HUNDRED, 100.0f32);
        e.register(THREAT_SOURCE_POSITION_009F7050, |_, a| a[1].into_ret());
        e.register(THREAT_TARGET_POSITION_009F7080, |_, a| a[1].into_ret());
        e.register(SHAPE_OF_TWO_009A4E30, |e, a| {
            for i in 0..9 {
                e.mem.set_u32(a[0] + 4 * i, 0x200 + i);
            }
            a[0].into_ret()
        });
        e.register(SHAPE_OF_ONE_006915D0, |e, a| {
            for i in 0..9 {
                e.mem.set_u32(a[0] + 4 * i, 0x100 + i);
            }
            a[0].into_ret()
        });
        let threat = e.mem.alloc(THREAT_SIZE);
        e.mem.set_f32(threat + THREAT_RADIUS, 5.0);
        let out = e.mem.alloc(0x24);
        // Neither: untouched.
        e.call(0x009f_6f30, &args![threat, out]);
        assert_eq!(e.mem.u32(out), 0);
        // Both: the two-position shape with the radius and 100.0.
        e.mem.set_u8(threat + THREAT_HAS_SOURCE, 1);
        e.mem.set_u8(threat + THREAT_HAS_TARGET, 1);
        e.call_log = Some(vec![]);
        e.call(0x009f_6f30, &args![threat, out]);
        assert_eq!(e.mem.u32(out + 0x20), 0x208);
        let two = calls_to(&e, SHAPE_OF_TWO_009A4E30);
        assert_eq!(two[0][3..], [5.0f32.to_bits(), 100.0f32.to_bits()]);
        // Source only, then target only: the one-position shape.
        for (has_source, has_target) in [(1u8, 0u8), (0, 1)] {
            e.mem.set_u8(threat + THREAT_HAS_SOURCE, has_source);
            e.mem.set_u8(threat + THREAT_HAS_TARGET, has_target);
            e.call_log = Some(vec![]);
            e.call(0x009f_6f30, &args![threat, out]);
            assert_eq!(e.mem.u32(out), 0x100);
            let one = calls_to(&e, SHAPE_OF_ONE_006915D0);
            assert_eq!(one[0][2..], [5.0f32.to_bits(), 100.0f32.to_bits()]);
            let used = if has_source != 0 {
                THREAT_SOURCE_POSITION_009F7050
            } else {
                THREAT_TARGET_POSITION_009F7080
            };
            assert_eq!(calls_to(&e, used).len(), 1);
        }
    }

    #[test]
    fn at_009f70b0_and_009f7170_save_and_load_the_threat() {
        let mut e = Engine::new();
        e.register(CURRENT_TIME_00457FE0, |_, _| 1000u32.into_ret());
        e.register(SAVE_FORM_ID, nothing);
        e.register(LOAD_FORM_ID, nothing);
        let threat = e.mem.alloc(THREAT_SIZE);
        e.mem.set_u32(threat + THREAT_LAST_PERCEIVED_TIME, 900);
        e.mem.set_u32(threat + THREAT_SOURCE, 0x11);
        e.mem.set_u32(threat + THREAT_TARGET, 0x22);
        let saved = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let sink = saved.clone();
        e.register_double(SAVE_DATA, move |e, a| {
            sink.borrow_mut().push((a[1], a[2], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009f_70b0, &args![threat, 0x55u32]);
        let saved = saved.borrow();
        let fields: Vec<(u32, u32)> = saved
            .iter()
            .map(|s| (s.0.wrapping_sub(threat), s.1))
            .collect();
        assert_eq!(fields[0], (8, 4));
        assert_eq!(saved[1].2, 100, "the time since perceived");
        assert_eq!(
            &fields[2..],
            &[(0x10, 4), (0x14, 0xc), (0x20, 0xc), (0x2c, 1), (0x2d, 1)]
        );
        assert_eq!(
            calls_to(&e, SAVE_FORM_ID),
            vec![vec![0x55, 0x11, 0], vec![0x55, 0x22, 0]]
        );
        // Loading: the local holds 250, so the time becomes 1000 - 250.
        e.register(LOAD_DATA, |e, a| {
            if a[2] == 4 && e.mem.u32(a[1]) == 0 {
                e.mem.set_u32(a[1], 250);
            }
            Ret::default()
        });
        let fresh = e.mem.alloc(THREAT_SIZE);
        e.mem.set_u32(fresh + THREAT_LEVEL, 7);
        e.call_log = Some(vec![]);
        e.call(0x009f_7170, &args![fresh, 0x55u32]);
        assert_eq!(e.mem.u32(fresh + THREAT_LAST_PERCEIVED_TIME), 750);
        let loads: Vec<(u32, u32)> = calls_to(&e, LOAD_DATA)
            .iter()
            .filter(|a| (fresh..fresh + THREAT_SIZE).contains(&a[1]))
            .map(|a| (a[1] - fresh, a[2]))
            .collect();
        assert_eq!(
            loads,
            vec![
                (8, 4),
                (0x10, 4),
                (0x14, 0xc),
                (0x20, 0xc),
                (0x2c, 1),
                (0x2d, 1)
            ]
        );
        assert_eq!(
            calls_to(&e, LOAD_FORM_ID),
            vec![vec![0x55, fresh], vec![0x55, fresh + 4]]
        );
    }

    #[test]
    fn at_009f7230_resolves_the_actors() {
        let mut e = Engine::new();
        e.register(SET_TARGETED, nothing);
        e.register(LOOKUP_FORM_004839C0, |_, a| (a[0] + 0x1000).into_ret());
        // Only the source id turns out to be an actor.
        e.register(RT_DYNAMIC_CAST, |_, a| {
            if a[0] == 0x1011 {
                a[0].into_ret()
            } else {
                Ret::default()
            }
        });
        let threat = e.mem.alloc(THREAT_SIZE);
        e.mem.set_u32(threat + THREAT_SOURCE, 0x11);
        e.mem.set_u32(threat + THREAT_TARGET, 0x22);
        e.call_log = Some(vec![]);
        e.call(0x009f_7230, &args![threat, 0u32]);
        assert_eq!(e.mem.u32(threat + THREAT_SOURCE), 0x1011);
        assert_eq!(e.mem.u32(threat + THREAT_TARGET), 0);
        assert_eq!(calls_to(&e, SET_TARGETED), vec![vec![0x1011, 1]]);
        let casts = calls_to(&e, RT_DYNAMIC_CAST);
        assert_eq!(casts[0], vec![0x1011, 0, TYPE_TES_FORM, TYPE_ACTOR, 0]);
    }

    #[test]
    fn at_009f72f0_constructs_and_009f73e0_destroys_the_package() {
        let mut e = Engine::new();
        threat_world(&mut e);
        e.map(0x011f_4000, 0x1000);
        set_point(&mut e, POINT_011F426C, [1.0, 2.0, 3.0]);
        for callee in [
            PACKAGE_CONSTRUCT,
            PACKAGE_SET_PACK_TYPE,
            PACKAGE_STEP_00984F60,
            PACKAGE_DESTRUCT_006708B0,
            NI_POINTER_DESTRUCT_0045CEC0,
            DEALLOCATE,
        ] {
            e.register(callee, nothing);
        }
        e.register(NI_POINTER_CONSTRUCT_00633C90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(NI_POINTER_ASSIGN_0066B0D0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0xb4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f_72f0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SPECTATOR_PACKAGE_VTABLE);
        assert_eq!(point(&e, this + EVENT_CENTER), [1.0, 2.0, 3.0]);
        assert_eq!(
            calls_to(&e, ALLOCATE_SLOT),
            vec![vec![this + THREAT_INFO_LIST, 0x20]]
        );
        assert_eq!(e.mem.u32(this + THREAT_INFO_LIST + 0xc), 0x20);
        assert_eq!(e.mem.u32(this + THREAT_INFO_LIST + 8), 0);
        assert_eq!(calls_to(&e, PACKAGE_SET_PACK_TYPE), vec![vec![this, 0x18]]);
        assert_eq!(calls_to(&e, PACKAGE_STEP_00984F60), vec![vec![this, 0xf]]);
        assert_eq!(e.mem.u8(this + FLAG_COMPLETED), 0);
        // A debug geometry with a parent is detached on destruction.
        let parent = object(&mut e, 0x10, &[(SLOT_DETACH_CHILD, 0x0672_0000)]);
        e.register(0x0672_0000, nothing);
        let geometry = e.mem.alloc(0x20);
        e.mem.set_u32(geometry + 0x18, parent);
        e.mem.set_u32(this + DEBUG_GEOMETRY, geometry);
        e.register(WORD_AT_18, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f_73e0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x0672_0000), vec![vec![parent, geometry]]);
        assert_eq!(
            calls_to(&e, NI_POINTER_ASSIGN_0066B0D0),
            vec![vec![this + DEBUG_GEOMETRY, 0]]
        );
        assert_eq!(e.mem.u32(this + THREAT_INFO_LIST), ARRAY_VTABLE_01093980);
        assert_eq!(calls_to(&e, PACKAGE_DESTRUCT_006708B0), vec![vec![this]]);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![this]]);
        // Without the flag nothing is freed.
        e.call_log = Some(vec![]);
        e.call(0x009f_73e0, &args![this, 0u32]);
        assert!(calls_to(&e, DEALLOCATE).is_empty());
    }

    #[test]
    fn at_009f7500_copies_a_spectator_package() {
        let mut e = Engine::new();
        threat_world(&mut e);
        e.register(PACKAGE_COPY_00673290, nothing);
        e.register(NI_POINTER_COPY, nothing);
        let other = package(&mut e, &[(0x11, 0x22, 1, 1), (0x33, 0, 1, 0)]);
        e.mem.set_u32(other + LAST_PATH_REQUEST_TIME, 77);
        e.mem.set_f32(other + EVENT_RADIUS, 9.0);
        set_point(&mut e, other + EVENT_CENTER, [4.0, 5.0, 6.0]);
        e.mem.set_u8(other + FLAG_COMPLETED, 1);
        let this = package(&mut e, &[]);
        e.mem.set_u32(this + THREAT_INFO_LIST + 4, 0);
        e.mem.set_u32(this + THREAT_INFO_LIST + 0xc, 0);
        // Not a spectator package: only the package part.
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        e.call(0x009f_7500, &args![this, other]);
        assert_eq!(e.mem.u32(this + LAST_PATH_REQUEST_TIME), 0);
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009f_7500, &args![this, other]);
        assert_eq!(calls_to(&e, PACKAGE_COPY_00673290), vec![vec![this, other]]);
        assert_eq!(e.mem.u32(this + LAST_PATH_REQUEST_TIME), 77);
        assert_eq!(e.mem.f32(this + EVENT_RADIUS), 9.0);
        assert_eq!(point(&e, this + EVENT_CENTER), [4.0, 5.0, 6.0]);
        assert_eq!(e.mem.u8(this + FLAG_COMPLETED), 1);
        assert_eq!(e.mem.u32(this + THREAT_INFO_LIST + 8), 2);
        assert_eq!(e.mem.u32(threat(&e, this, 1) + THREAT_SOURCE), 0x33);
        assert_eq!(e.mem.u8(threat(&e, this, 0) + THREAT_HAS_TARGET), 1);
    }

    #[test]
    fn at_009f7630_assigns_a_threat() {
        let mut e = Engine::new();
        e.register(NI_POINTER_COPY, nothing);
        let from = e.mem.alloc(THREAT_SIZE);
        for i in 0..0xc {
            e.mem.set_u32(from + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(from + THREAT_RADIUS, 0x7f80_0001);
        e.mem.set_u8(from + THREAT_HAS_SOURCE, 1);
        e.mem.set_u8(from + THREAT_HAS_TARGET, 2);
        let to = e.mem.alloc(THREAT_SIZE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f_7630, &args![to, from]).u32(), to);
        for i in [0u32, 1, 2, 3, 5, 6, 7, 8, 9, 10] {
            assert_eq!(e.mem.u32(to + 4 * i), 0x100 + i);
        }
        // The radius goes through the x87 stack: a signalling NaN is quieted.
        assert_eq!(e.mem.u32(to + THREAT_RADIUS), 0x7fc0_0001);
        assert_eq!(e.mem.u8(to + THREAT_HAS_TARGET), 2);
        assert_eq!(
            calls_to(&e, NI_POINTER_COPY),
            vec![vec![to + 0x30, from + 0x30]]
        );
    }

    /// The doubles of `009f76e0`: positions copied, the larger and smaller of
    /// two floats, the in-place point add and scale, the difference, its
    /// squared length and the square root.
    fn event_world(e: &mut Engine) {
        threat_world(e);
        e.map(0x011f_4000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.set_global(HALF, 0.5f32);
        e.register(THREAT_SOURCE_POSITION_009F7050, |e, a| {
            copy_words(e, a[1], a[0] + THREAT_SOURCE_POSITION, 3);
            a[1].into_ret()
        });
        e.register(THREAT_TARGET_POSITION_009F7080, |e, a| {
            copy_words(e, a[1], a[0] + THREAT_TARGET_POSITION, 3);
            a[1].into_ret()
        });
        e.register(MAX_00404010, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            Ret {
                st0: f64::from(if x <= y { y } else { x }),
                ..Ret::default()
            }
        });
        e.register(MIN_0040EBD0, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            Ret {
                st0: f64::from(if y <= x { y } else { x }),
                ..Ret::default()
            }
        });
        e.register(POINT_ADD_IN_PLACE_0063C8A0, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            a[0].into_ret()
        });
        e.register(POINT_SCALE_IN_PLACE_00439180, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * f32::from_bits(a[1]);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            a[0].into_ret()
        });
        e.register(POINT_SUBTRACT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_LENGTH_SQUARED, |e, a| Ret {
            st0: (0..3)
                .map(|i| f64::from(e.mem.f32(a[0] + 4 * i)).powi(2))
                .sum(),
            ..Ret::default()
        });
        e.register(RADIUS_OF_004019D0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).sqrt(),
            ..Ret::default()
        });
    }

    #[test]
    fn at_009f76e0_centers_the_event_on_the_positions() {
        let mut e = Engine::new();
        event_world(&mut e);
        let this = package(&mut e, &[(0, 0, 0, 0), (1, 2, 1, 1), (3, 0, 0, 1)]);
        let first = threat(&e, this, 1);
        set_point(&mut e, first + THREAT_SOURCE_POSITION, [0.0, 0.0, 0.0]);
        set_point(&mut e, first + THREAT_TARGET_POSITION, [4.0, 2.0, 0.0]);
        let second = threat(&e, this, 2);
        set_point(&mut e, second + THREAT_TARGET_POSITION, [-4.0, 6.0, 0.0]);
        e.call(0x009f_76e0, &args![this]);
        // Bounds x [-4, 4], y [0, 6]: center (0, 3, 0); the farthest
        // position is 5 away.
        assert_eq!(point(&e, this + EVENT_CENTER), [0.0, 3.0, 0.0]);
        assert_eq!(e.mem.f32(this + EVENT_RADIUS), 5.0);
        assert_eq!(e.mem.u8(this + FLAG_COMPLETED), 0);
        // No known position: completed.
        let empty = package(&mut e, &[(1, 0, 0, 0)]);
        e.call(0x009f_76e0, &args![empty]);
        assert_eq!(e.mem.u8(empty + FLAG_COMPLETED), 1);
        // No threats: nothing.
        let none = package(&mut e, &[]);
        e.call(0x009f_76e0, &args![none]);
        assert_eq!(e.mem.u8(none + FLAG_COMPLETED), 0);
    }

    #[test]
    fn at_009f76e0_a_single_source_sets_both_bounds() {
        let mut e = Engine::new();
        event_world(&mut e);
        let this = package(&mut e, &[(1, 0, 1, 0)]);
        let only = threat(&e, this, 0);
        set_point(&mut e, only + THREAT_SOURCE_POSITION, [7.0, 8.0, 9.0]);
        e.call(0x009f_76e0, &args![this]);
        assert_eq!(point(&e, this + EVENT_CENTER), [7.0, 8.0, 9.0]);
        assert_eq!(e.mem.f32(this + EVENT_RADIUS), 0.0);
    }

    /// The process doubles of the conversation tests: the process at +0x68
    /// of the actor with slots +0x310, +0x30C (answering `busy`), +0x2A4,
    /// +0x628 and +0x644.
    fn conversation_world(e: &mut Engine, busy: u32) -> (u32, u32) {
        threat_world(e);
        let process = object(
            e,
            0x10,
            &[
                (SLOT_PROCESS_0X310, 0x0673_0310),
                (SLOT_PROCESS_0X30C, 0x0673_030c),
                (SLOT_PROCESS_0X2A4, 0x0673_02a4),
                (SLOT_PROCESS_0X628, 0x0673_0628),
                (SLOT_PROCESS_0X644, 0x0673_0644),
            ],
        );
        for slot in [0x0673_0310, 0x0673_02a4, 0x0673_0628, 0x0673_0644] {
            e.register(slot, nothing);
        }
        e.register_double(0x0673_030c, move |_, _| busy.into_ret());
        let actor = e.mem.alloc(0x80);
        e.mem.set_u32(actor + 0x68, process);
        e.register(ACTOR_PROCESS, |e, a| e.mem.u32(a[0] + 0x68).into_ret());
        (actor, process)
    }

    /// A target that answers its slot +0x100 with `watchable`.
    fn watched_target(e: &mut Engine, watchable: u32) -> u32 {
        e.register_double(0x0674_0100, move |_, _| watchable.into_ret());
        object(e, 0x10, &[(SLOT_TARGET_WATCHABLE, 0x0674_0100)])
    }

    #[test]
    fn at_009f7d70_stops_or_starts_a_conversation() {
        let mut e = Engine::new();
        let (actor, process) = conversation_world(&mut e, 0);
        e.register(ACTOR_STOP_MOVING, nothing);
        e.register(RANDOM_00487F50, |_, _| 142u32.into_ret());
        e.map(0x011c_c000, 0x2000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.set_global(SETTING_011CDC04 + 4, 50.0f32);
        e.set_global(SETTING_011CCFF0 + 4, 12.0f32);
        e.register(TOPIC_GET_TOPIC, |_, _| 0x7070u32.into_ret());
        e.register(REFERENCE_STEP_0057BD60, nothing);
        e.register(TIMER_VALUE_0084D030, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        let target = watched_target(&mut e, 1);
        let this = package(&mut e, &[(0x11, target, 1, 1)]);
        // Not watching: stop.
        e.call_log = Some(vec![]);
        e.call(0x009f_7d70, &args![this, actor, 0u32]);
        assert_eq!(calls_to(&e, 0x0673_0310), vec![vec![process, 0]]);
        assert_eq!(calls_to(&e, ACTOR_STOP_MOVING), vec![vec![actor]]);
        // Watching, roll 42 below 50, timer 0: the conversation, then the
        // timer goes down by the frame time.
        e.call_log = Some(vec![]);
        e.call(0x009f_7d70, &args![this, actor, 1u32]);
        assert_eq!(
            calls_to(&e, REFERENCE_STEP_0057BD60),
            vec![vec![actor, target]]
        );
        assert_eq!(
            calls_to(&e, 0x0673_02a4),
            vec![vec![process, actor, 0x7070, 0, 0, 1, 0]]
        );
        assert_eq!(e.mem.f32(this + CONVERSATION_TIMER), 11.5);
        // The timer still running: no conversation, the timer goes on.
        e.call_log = Some(vec![]);
        e.call(0x009f_7d70, &args![this, actor, 1u32]);
        assert!(calls_to(&e, TOPIC_GET_TOPIC).is_empty());
        assert_eq!(e.mem.f32(this + CONVERSATION_TIMER), 11.0);
        // A roll at the setting: none either.
        e.mem.set_f32(this + CONVERSATION_TIMER, 0.0);
        e.register(RANDOM_00487F50, |_, _| 150u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009f_7d70, &args![this, actor, 1u32]);
        assert!(calls_to(&e, TOPIC_GET_TOPIC).is_empty());
        assert_eq!(e.mem.f32(this + CONVERSATION_TIMER), -0.5);
        // An unwatchable target: nothing at all, not even the timer.
        let hidden = watched_target(&mut e, 0);
        let other = package(&mut e, &[(0x11, hidden, 1, 1)]);
        e.call_log = Some(vec![]);
        e.call(0x009f_7d70, &args![other, actor, 1u32]);
        assert!(calls_to(&e, TIMER_VALUE_0084D030).is_empty());
    }

    #[test]
    fn at_009f7f20_faces_the_target_or_gives_up() {
        let mut e = Engine::new();
        let (actor, process) = conversation_world(&mut e, 0);
        let target = watched_target(&mut e, 1);
        let this = package(&mut e, &[(0x11, target, 1, 1)]);
        e.call_log = Some(vec![]);
        e.call(0x009f_7f20, &args![this, actor, 1u32]);
        assert_eq!(calls_to(&e, 0x0673_0628), vec![vec![process, target]]);
        assert!(calls_to(&e, 0x0673_0644).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x009f_7f20, &args![this, actor, 0u32]);
        assert_eq!(calls_to(&e, 0x0673_0644), vec![vec![process, 1]]);
        let empty = package(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x009f_7f20, &args![empty, actor, 1u32]);
        assert_eq!(calls_to(&e, 0x0673_0644), vec![vec![process, 1]]);
    }

    #[test]
    fn at_009f7fe0_done_without_threats_or_when_completed() {
        let mut e = Engine::new();
        threat_world(&mut e);
        let empty = package(&mut e, &[]);
        assert!(e.call(0x009f_7fe0, &args![empty]).bool());
        let busy = package(&mut e, &[(1, 2, 1, 1)]);
        assert!(!e.call(0x009f_7fe0, &args![busy]).bool());
        e.mem.set_u8(busy + FLAG_COMPLETED, 1);
        assert!(e.call(0x009f_7fe0, &args![busy]).bool());
    }

    #[test]
    fn at_009f8020_expires_drops_and_refreshes_threats() {
        let mut e = Engine::new();
        event_world(&mut e);
        e.register(CURRENT_TIME_00457FE0, |_, _| 10_000u32.into_ret());
        e.map(0x011c_d000, 0x1000);
        e.map(0x0101_7000, 0x1000);
        e.set_global(THOUSAND, 1000.0f64);
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.set_global(SETTING_011CD264 + 4, 5.0f32);
        e.register(FORM_FLAG_20_00440D80, |_, _| Ret::default());
        e.register(THREAT_REFRESH_009F6D50, nothing);
        e.register(PACKAGE_UPDATE_004534F0, nothing);
        e.register(ACTOR_PROCESS, |_, _| Ret::default());
        // A target that answers +0x22C (gone), one that does not.
        let gone = object(&mut e, 0x10, &[(SLOT_ACTOR_0X22C, 0x0675_0000)]);
        e.register(0x0675_0000, |_, _| 1u32.into_ret());
        let here = object(
            &mut e,
            0x10,
            &[
                (SLOT_ACTOR_0X22C, 0x0675_0004),
                (SLOT_ACTOR_0X230, 0x0675_0004),
                (SLOT_ACTOR_0X234, 0x0675_0004),
            ],
        );
        e.register(0x0675_0004, nothing);
        // A source aiming at `here`.
        let source = object(
            &mut e,
            0x10,
            &[
                (SLOT_ACTOR_0X218, 0x0675_0008),
                (SLOT_ACTOR_TARGET, 0x0675_000c),
            ],
        );
        e.register(0x0675_0008, nothing);
        e.register_double(0x0675_000c, move |_, _| here.into_ret());
        let this = package(
            &mut e,
            &[
                (0, 0, 0, 0),
                (0, gone, 0, 1),
                (source, here, 1, 1),
                (0, 0, 0, 0),
            ],
        );
        // Threat 0: perceived 6 s ago, past the 5 s limit: removed.
        e.mem
            .set_u32(threat(&e, this, 0) + THREAT_LAST_PERCEIVED_TIME, 4000);
        // Threat 1 (3 s ago): its target is gone. Threat 2 (3 s ago):
        // refreshed. Threat 3 (1 s ago): left alone.
        e.mem
            .set_u32(threat(&e, this, 1) + THREAT_LAST_PERCEIVED_TIME, 7000);
        e.mem
            .set_u32(threat(&e, this, 2) + THREAT_LAST_PERCEIVED_TIME, 7000);
        e.mem
            .set_u32(threat(&e, this, 3) + THREAT_LAST_PERCEIVED_TIME, 9000);
        let actor = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        e.call(0x009f_8020, &args![this, actor]);
        // The removal moved threat 3 into the gap: 3, 1, 2.
        assert_eq!(e.mem.u32(this + THREAT_INFO_LIST + 8), 3);
        assert_eq!(
            e.mem.u32(threat(&e, this, 0) + THREAT_LAST_PERCEIVED_TIME),
            9000
        );
        assert_eq!(e.mem.u32(threat(&e, this, 1) + THREAT_TARGET), 0);
        assert_eq!(e.mem.u32(threat(&e, this, 2) + THREAT_SOURCE), source);
        assert_eq!(
            calls_to(&e, THREAT_REFRESH_009F6D50),
            vec![vec![threat(&e, this, 2)]]
        );
        // The change recomputed the event (009f76e0 asked for positions).
        assert!(!calls_to(&e, THREAT_SOURCE_POSITION_009F7050).is_empty());
        assert_eq!(
            calls_to(&e, PACKAGE_UPDATE_004534F0),
            vec![vec![this, actor]]
        );
    }

    #[test]
    fn at_009f8300_copies_the_event_center() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb4);
        set_point(&mut e, this + EVENT_CENTER, [1.0, 2.0, 3.0]);
        let out = e.mem.alloc(12);
        assert_eq!(e.call(0x009f_8300, &args![this, out]).u32(), out);
        assert_eq!(point(&e, out), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn at_009f8330_reuses_or_appends_the_threat() {
        let mut e = Engine::new();
        event_world(&mut e);
        e.register(SET_TARGETED, nothing);
        e.register(THREAT_REFRESH_009F6D50, nothing);
        e.register_double(0x0676_0000, |_, _| 0x2222u32.into_ret());
        let source = object(&mut e, 0x10, &[(SLOT_ACTOR_TARGET, 0x0676_0000)]);
        let this = package(&mut e, &[(0x9, 0, 1, 0), (source, 0x2222, 1, 1)]);
        e.call(0x009f_8330, &args![this, source]);
        assert_eq!(e.mem.u32(this + THREAT_INFO_LIST + 8), 2);
        // Another source: a new threat at the end.
        let other = object(&mut e, 0x10, &[(SLOT_ACTOR_TARGET, 0x0676_0000)]);
        e.call(0x009f_8330, &args![this, other]);
        assert_eq!(e.mem.u32(this + THREAT_INFO_LIST + 8), 3);
        assert_eq!(e.mem.u32(threat(&e, this, 2) + THREAT_SOURCE), other);
        assert_eq!(e.mem.u32(threat(&e, this, 2) + THREAT_TARGET), 0x2222);
    }

    #[test]
    fn at_009f8420_and_009f84d0_forget_an_actor() {
        let mut e = Engine::new();
        threat_world(&mut e);
        let this = package(&mut e, &[(5, 6, 1, 1), (7, 5, 1, 1), (8, 9, 1, 1)]);
        e.call(0x009f_8420, &args![this, 5u32]);
        // The removal moves the last threat into the gap: (8, 9), (7, 5).
        assert_eq!(e.mem.u32(this + THREAT_INFO_LIST + 8), 2);
        assert_eq!(e.mem.u32(threat(&e, this, 0) + THREAT_SOURCE), 8);
        let second = threat(&e, this, 1);
        assert_eq!(e.mem.u32(second + THREAT_SOURCE), 7);
        assert_eq!(e.mem.u32(second + THREAT_TARGET), 0);
        assert_eq!(e.mem.u8(second + THREAT_HAS_TARGET), 0);
        let other = package(&mut e, &[(5, 6, 1, 1), (7, 5, 1, 1), (8, 9, 1, 1)]);
        e.call(0x009f_84d0, &args![other, 5u32]);
        assert_eq!(e.mem.u32(other + THREAT_INFO_LIST + 8), 1);
        assert_eq!(e.mem.u32(threat(&e, other, 0) + THREAT_SOURCE), 8);
    }

    #[test]
    fn at_009f8570_collects_the_shapes() {
        let mut e = Engine::new();
        threat_world(&mut e);
        e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SHAPE_ARRAY_CONSTRUCT_006E3850, |_, a| a[0].into_ret());
        e.register(SHAPE_ARRAY_SET_SIZE_006E3AD0, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(SHAPE_ARRAY_AT_006A1400, |_, a| (0x9000 + a[1]).into_ret());
        e.register(THREAT_SOURCE_POSITION_009F7050, |_, a| a[1].into_ret());
        e.register(SHAPE_OF_ONE_006915D0, |_, a| a[0].into_ret());
        let this = package(&mut e, &[(0, 0, 0, 0), (0, 0, 0, 0)]);
        e.call_log = Some(vec![]);
        let array = e.call(0x009f_8570, &args![this]).u32();
        assert_eq!(e.mem.u32(array + 8), 2);
        let sizes: Vec<u32> = calls_to(&e, SHAPE_ARRAY_SET_SIZE_006E3AD0)
            .iter()
            .map(|a| a[1])
            .collect();
        assert_eq!(sizes, vec![1, 2]);
        assert_eq!(
            calls_to(&e, SHAPE_ARRAY_AT_006A1400),
            vec![vec![array, 0], vec![array, 1]]
        );
    }

    #[test]
    fn at_009f8660_lists_the_sources() {
        let mut e = Engine::new();
        threat_world(&mut e);
        e.register(ARRAY_RELEASE_008454F0, nothing);
        e.register(ARRAY_SET_RESERVED_SIZE, nothing);
        let added = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let sink = added.clone();
        e.register_double(ARRAY_ADD, move |e, a| {
            sink.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        let this = package(&mut e, &[(5, 6, 1, 1), (0, 5, 0, 1), (8, 9, 1, 1)]);
        e.call_log = Some(vec![]);
        e.call(0x009f_8660, &args![this, 0x4000u32]);
        assert_eq!(*added.borrow(), vec![5, 8]);
        assert_eq!(calls_to(&e, ARRAY_RELEASE_008454F0), vec![vec![0x4000, 1]]);
        assert_eq!(calls_to(&e, ARRAY_SET_RESERVED_SIZE), vec![vec![0x4000, 6]]);
    }

    #[test]
    fn save_size_and_old_save_functions() {
        let mut e = Engine::new();
        e.register(PACKAGE_SAVE_SIZE_00678FD0, |_, _| 0xfffcu32.into_ret());
        for callee in [
            PACKAGE_SAVE_00679120,
            PACKAGE_LOAD_GAME,
            PACKAGE_STEP_006796C0,
            FORM_SAVE_GAME_DATA_OLD,
            FORM_LOAD_GAME_DATA_OLD,
        ] {
            e.register(callee, nothing);
        }
        // 0xFFFC + 8 wraps as a halfword.
        assert_eq!(e.call(0x009f_86f0, &args![0x5000u32]).u32() & 0xffff, 4);
        e.call_log = Some(vec![]);
        e.call(0x009f_8730, &args![0x5000u32]);
        assert_eq!(calls_to(&e, PACKAGE_SAVE_00679120), vec![vec![0x5000]]);
        assert_eq!(
            calls_to(&e, FORM_SAVE_GAME_DATA_OLD),
            vec![vec![0x5000, 0x5084, 4], vec![0x5000, 0x5088, 4]]
        );
        e.call_log = Some(vec![]);
        e.call(0x009f_8770, &args![0x5000u32]);
        assert_eq!(calls_to(&e, PACKAGE_LOAD_GAME), vec![vec![0x5000]]);
        assert_eq!(
            calls_to(&e, FORM_LOAD_GAME_DATA_OLD),
            vec![vec![0x5000, 0x5084, 4], vec![0x5000, 0x5088, 4]]
        );
        e.call_log = Some(vec![]);
        e.call(0x009f_87b0, &args![0x5000u32]);
        assert_eq!(calls_to(&e, PACKAGE_STEP_006796C0), vec![vec![0x5000]]);
    }

    #[test]
    fn at_009f87d0_009f8900_009f8a10_save_load_and_init() {
        let mut e = Engine::new();
        threat_world(&mut e);
        e.register(TIMER_NOW, |_, _| 5000u32.into_ret());
        e.register(CURRENT_TIME_00457FE0, |_, _| 5000u32.into_ret());
        for callee in [
            PACKAGE_SAVE_GAME,
            PACKAGE_LOAD_GAME_OV2,
            PACKAGE_INIT_LOAD_GAME,
            SAVE_DATA,
            SAVE_FORM_ID,
            SAVE_VARIABLE_SIZED_VALUE,
            LOAD_FORM_ID,
            SET_TARGETED,
        ] {
            e.register(callee, nothing);
        }
        let this = package(&mut e, &[(0, 0, 0, 0), (0, 0, 0, 0)]);
        e.mem.set_u32(this + LAST_PATH_REQUEST_TIME, 4000);
        e.call_log = Some(vec![]);
        e.call(0x009f_87d0, &args![this, 0x55u32]);
        assert_eq!(calls_to(&e, PACKAGE_SAVE_GAME), vec![vec![this, 0x55]]);
        assert_eq!(calls_to(&e, SAVE_VARIABLE_SIZED_VALUE), vec![vec![0x55, 2]]);
        let saves = calls_to(&e, SAVE_DATA);
        let own: Vec<(u32, u32)> = saves
            .iter()
            .filter(|a| (this..this + 0xb4).contains(&a[1]))
            .map(|a| (a[1] - this, a[2]))
            .collect();
        assert_eq!(
            &own[..5],
            &[(0x84, 4), (0x88, 4), (0x8c, 4), (0x90, 1), (0x94, 0xc)]
        );
        // Two threats saved: eight writes each.
        assert_eq!(saves.len(), 1 + 5 + 2 * 7);
        // Loading three threats: the list grows to three, each loaded.
        e.register(LOAD_VARIABLE_SIZED_VALUE, |_, _| 3u32.into_ret());
        e.register(LOAD_DATA, |e, a| {
            if a[2] == 4 && e.mem.u32(a[1]) == 0 {
                e.mem.set_u32(a[1], 300);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009f_8900, &args![this, 0x55u32]);
        assert_eq!(e.mem.u32(this + LAST_PATH_REQUEST_TIME), 4700);
        assert_eq!(e.mem.u32(this + THREAT_INFO_LIST + 8), 3);
        assert_eq!(calls_to(&e, LOAD_FORM_ID).len(), 6);
        // Init: each threat resolves its (null) actors.
        e.call_log = Some(vec![]);
        e.call(0x009f_8a10, &args![this, 0x55u32]);
        assert_eq!(calls_to(&e, PACKAGE_INIT_LOAD_GAME), vec![vec![this, 0x55]]);
    }

    #[test]
    fn threat_array_element_construct_and_init() {
        let mut e = Engine::new();
        threat_world(&mut e);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x8000);
        assert_eq!(
            e.call(0x009f_8ac0, &args![array, 3u32]).u32(),
            0x8000 + 3 * 0x34
        );
        assert_eq!(e.call(0x009f_8aa0, &args![array]).u32(), 0x8000);
        // 009f8ae0 / 009f90f0: the reserved size at least the size.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f_8ae0, &args![array, 2u32, 5u32]).u32(), array);
        assert_eq!(e.mem.u32(array), ARRAY_VTABLE_01093980);
        assert_eq!(calls_to(&e, ALLOCATE_SLOT), vec![vec![array, 5]]);
        let buffer = e.mem.u32(array + 4);
        assert_eq!(
            calls_to(&e, THREATS_CONSTRUCT_009F8DE0),
            vec![vec![array, buffer, 5]]
        );
        assert_eq!((e.mem.u32(array + 8), e.mem.u32(array + 0xc)), (5, 5));
        // Nothing at all: no buffer.
        e.call_log = Some(vec![]);
        e.call(0x009f_90f0, &args![array, 0u32, 0u32]);
        assert!(calls_to(&e, ALLOCATE_SLOT).is_empty());
        assert_eq!(
            (
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ),
            (0, 0, 0)
        );
    }

    #[test]
    fn threat_array_set_size_remove_and_release() {
        let mut e = Engine::new();
        threat_world(&mut e);
        let this = package(&mut e, &[]);
        let list = this + THREAT_INFO_LIST;
        e.mem.set_u32(list + 4, 0);
        e.mem.set_u32(list + 0xc, 0);
        // Growing from nothing: allocated and constructed.
        e.call_log = Some(vec![]);
        e.call(0x009f_8b30, &args![list, 4u32, 1u32]);
        assert_eq!(calls_to(&e, ALLOCATE_SLOT), vec![vec![list, 4]]);
        assert_eq!((e.mem.u32(list + 8), e.mem.u32(list + 0xc)), (4, 4));
        for i in 0..4 {
            e.mem.set_u32(threat(&e, this, i), 0x10 + i);
        }
        // Past the reserved size with the buffer: the count equals the
        // reserved size, so slot +0xC reallocates in place.
        e.call_log = Some(vec![]);
        e.call(0x009f_8b30, &args![list, 0x10u32, 0u32]);
        assert_eq!(calls_to(&e, REALLOCATE_SLOT).len(), 1);
        assert_eq!(e.mem.u32(threat(&e, this, 3)), 0x13);
        assert_eq!((e.mem.u32(list + 8), e.mem.u32(list + 0xc)), (0x10, 0x10));
        // Remove threat 1: destroyed, the end moves down.
        e.mem.set_u32(list + 8, 4);
        e.call_log = Some(vec![]);
        e.call(0x009f_8c70, &args![list, 1u32, 1u32]);
        assert_eq!(
            calls_to(&e, THREAT_DELETE_009F9180),
            vec![vec![threat(&e, this, 1), 0]]
        );
        assert_eq!(e.mem.u32(list + 8), 3);
        assert_eq!(e.mem.u32(threat(&e, this, 1)), 0x13);
        // Shrinking to a quarter with the flag: the buffer is reallocated
        // through a new one (the count differs from the reserved size).
        e.call_log = Some(vec![]);
        e.call(0x009f_8b30, &args![list, 2u32, 1u32]);
        assert_eq!(calls_to(&e, THREAT_DELETE_009F9180).len(), 1);
        assert_eq!(calls_to(&e, ALLOCATE_SLOT), vec![vec![list, 2]]);
        assert_eq!(calls_to(&e, ARRAY_FREE_BUFFER_006A8500), vec![vec![list]]);
        assert_eq!((e.mem.u32(list + 8), e.mem.u32(list + 0xc)), (2, 2));
        assert_eq!(e.mem.u32(threat(&e, this, 0)), 0x10);
        // Size 0: released; with the flag the buffer goes.
        e.call_log = Some(vec![]);
        e.call(0x009f_8b30, &args![list, 0u32, 1u32]);
        assert_eq!(calls_to(&e, THREAT_DELETE_009F9180).len(), 2);
        assert_eq!(
            (
                e.mem.u32(list + 4),
                e.mem.u32(list + 8),
                e.mem.u32(list + 0xc)
            ),
            (0, 0, 0)
        );
        // Released again without a buffer: nothing.
        e.mem.set_u32(list + 8, 7);
        e.call(0x009f_8d00, &args![list, 1u32]);
        assert_eq!(e.mem.u32(list + 8), 7);
    }

    #[test]
    fn threat_array_allocation_entries_and_destructor() {
        let mut e = Engine::new();
        threat_world(&mut e);
        e.register(ALLOCATE, |_, a| (0x100_0000 + a[0]).into_ret());
        e.register(REALLOCATE_0042F5D0, |_, a| (a[0] + a[1]).into_ret());
        e.register(DEALLOCATE, nothing);
        assert_eq!(
            e.call(0x009f_8d60, &args![0u32, 3u32]).u32(),
            0x100_0000 + 3 * 0x34
        );
        assert_eq!(
            e.call(0x009f_8d90, &args![0u32, 0x1000u32, 2u32]).u32(),
            0x1068
        );
        // The destroy step deletes each threat with 0.
        e.call_log = Some(vec![]);
        e.call(0x009f_8f60, &args![0u32, 0x2000u32, 2u32]);
        assert_eq!(
            calls_to(&e, THREAT_DELETE_009F9180),
            vec![vec![0x2000, 0], vec![0x2034, 0]]
        );
        // The deleting destructor: the body (vtable, release), then the free.
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f_8db0, &args![array, 1u32]).u32(), array);
        assert_eq!(e.mem.u32(array), ARRAY_VTABLE_01093980);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![array]]);
        e.call_log = Some(vec![]);
        e.call(0x009f_8db0, &args![array, 0u32]);
        assert!(calls_to(&e, DEALLOCATE).is_empty());
    }

    #[test]
    fn at_009f9040_reallocates_or_allocates() {
        let mut e = Engine::new();
        threat_world(&mut e);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, ARRAY_VTABLE_01093980);
        // No buffer: allocated, the reserved size set.
        e.call_log = Some(vec![]);
        e.call(0x009f_9040, &args![array, 3u32, 0u32]);
        assert_eq!(calls_to(&e, ALLOCATE_SLOT), vec![vec![array, 3]]);
        assert_eq!(e.mem.u32(array + 0xc), 3);
        // A count other than the reserved size: a new buffer, the threats moved.
        let old = e.mem.u32(array + 4);
        e.call_log = Some(vec![]);
        e.call(0x009f_9040, &args![array, 6u32, 2u32]);
        let fresh = e.mem.u32(array + 4);
        assert_eq!(
            calls_to(&e, THREATS_MOVE_009F8FA0),
            vec![vec![array, fresh, old, 2]]
        );
        assert_eq!(calls_to(&e, ARRAY_FREE_BUFFER_006A8500), vec![vec![array]]);
        assert_eq!(
            e.mem.u32(array + 0xc),
            3,
            "the caller sets the reserved size"
        );
    }
}
