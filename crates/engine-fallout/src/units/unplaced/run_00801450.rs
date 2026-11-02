//! `(unplaced)/run_00801450` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two groups of functions. The first works on the VATS effect manager of
//! `run_008001b0` (the same fields: the target actor at +4, its model root held at +8,
//! the body-part index at +0x23C, the extra part list at +0x240/+0x244, the slot list
//! at +0xC/+0x14, the flags at +0x129, +0x284, +0x298 and +0x2B4): drawing its targets
//! and switching the target's body parts on and off. The second is a table of 0x44-byte
//! text entries (+0x14 count, entries from +0x20) built by `008027d0` and kept in the
//! global `011db800`; its fields are read at their offsets.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00559450` (`thiscall`, no stack word): the word at `this`.
const FIRST_WORD: u32 = 0x0055_9450;
/// `0043c4b0` (no arguments): an object the draw functions ask for first.
const OBJECT_0043C4B0: u32 = 0x0043_c4b0;
/// `0045c670` (no arguments; `fallout shared/tes.cpp`): the object the draw functions get
/// their accumulator (`008d80e0`) and list (`006629f0`) from.
const OBJECT_0045C670: u32 = 0x0045_c670;
/// `008d80e0` (`thiscall`, no stack word).
const ACCUMULATOR_008D80E0: u32 = 0x008d_80e0;
/// `006629f0` (`thiscall`, no stack word).
const LIST_006629F0: u32 = 0x0066_29f0;
/// `004a0fd0` and `004fb0b0` (`thiscall`, one word each) on the accumulator.
const SET_004A0FD0: u32 = 0x004a_0fd0;
const SET_004FB0B0: u32 = 0x004f_b0b0;
/// `BSShaderUtil::AccumulateScene` (`00b6bee0`, engine map; `cdecl`: list, object,
/// accumulator).
const ACCUMULATE_SCENE: u32 = 0x00b6_bee0;
/// `00b6c0d0` (`cdecl`, three words: list, object, 0).
const FINISH_00B6C0D0: u32 = 0x00b6_c0d0;
/// `008d8520` (`thiscall`, no stack word; the engine map's name is
/// `MiddleHighProcess::GetSavedAcquireObject`).
const PROCESS_OBJECT_008D8520: u32 = 0x008d_8520;
/// `00456610` (`thiscall`, `bool`) and `00450f90` (`thiscall`, one word: hidden or not) on
/// a scene object.
const IS_HIDDEN_00456610: u32 = 0x0045_6610;
const SET_HIDDEN_00450F90: u32 = 0x0045_0f90;
/// `004181e0` (`thiscall`, no stack word; `fallout shared/extradatalist.cpp`).
const BASE_OBJECT_004181E0: u32 = 0x0041_81e0;
/// `BGSBodyPartData::GetBodyPart` (`005e50f0`, engine map; `thiscall`, one word).
const GET_BODY_PART: u32 = 0x005e_50f0;
/// `007fa950` (`thiscall`, no stack word): a body part's node name.
const BODY_PART_NAME: u32 = 0x007f_a950;
/// `00438170` (`thiscall`, one word) and `004381b0` (`thiscall`): the constructor and
/// destructor of the 4-byte name local.
const NAME_CONSTRUCT: u32 = 0x0043_8170;
const NAME_DESTRUCT: u32 = 0x0043_81b0;
/// `004e3270` (no arguments; `fallout shared/teswater.cpp`), `004ebbc0` (`thiscall`, one
/// word) and `004f15a0` (`thiscall`, one word): the object, an element of it by number and
/// that element's switch.
const OBJECT_004E3270: u32 = 0x004e_3270;
const ELEMENT_004EBBC0: u32 = 0x004e_bbc0;
const SWITCH_004F15A0: u32 = 0x004f_15a0;
/// `004e9bb0` (`thiscall`, one word), `0041fd00` (`thiscall`, one word), `0045bbe0`
/// (`thiscall`) and `00a694a0` (`thiscall`, one word): the other calls of `00801880`.
const CALL_004E9BB0: u32 = 0x004e_9bb0;
const CALL_0041FD00: u32 = 0x0041_fd00;
const CALL_0045BBE0: u32 = 0x0045_bbe0;
const CALL_00A694A0: u32 = 0x00a6_94a0;
/// `BSShaderProperty::SetFlagRecurse` (`00ba91a0`, engine map; `cdecl`: object, flag,
/// value).
const SET_FLAG_RECURSE: u32 = 0x00ba_91a0;
/// `00709d00` (no arguments; `fallout/interface/interface.cpp`, `bool`).
const CHECK_00709D00: u32 = 0x0070_9d00;
/// `00414430` (`thiscall`, four floats) and the colour calls
/// `SystemColorManager::GetInstance` (`00718b60`) and `SystemColorManager::GetColor`
/// (`00719060`, `thiscall`: a number, the colour out) (engine map).
const COLOR_CONSTRUCT: u32 = 0x0041_4430;
const SYSTEM_COLOR_MANAGER_GET_INSTANCE: u32 = 0x0071_8b60;
const SYSTEM_COLOR_MANAGER_GET_COLOR: u32 = 0x0071_9060;
/// `00403e20` (`thiscall` on a float setting): the address of its value.
const FLOAT_SETTING_VALUE: u32 = 0x0040_3e20;
/// The settings read here (objects in `.data`; names not confirmed).
const SETTING_011DB694: u32 = 0x011d_b694;
const SETTING_011DB6E4: u32 = 0x011d_b6e4;
const SETTING_011DB714: u32 = 0x011d_b714;
const SETTING_011DB720: u32 = 0x011d_b720;
const SETTING_011DB778: u32 = 0x011d_b778;
const SETTING_011DB79C: u32 = 0x011d_b79c;
const SETTING_011DB7B4: u32 = 0x011d_b7b4;
/// `NiPointer<..>::operator=` (`006e5cc0`, engine map; `thiscall`, one word) and
/// `0066b0d0` (`thiscall`, one word): set a held pointer.
const POINTER_ASSIGN: u32 = 0x006e_5cc0;
const SET_POINTER_0066B0D0: u32 = 0x0066_b0d0;
/// `BSRenderedTexture::GetTexture` (`004bc320`, engine map; `thiscall`, one word).
const GET_TEXTURE: u32 = 0x004b_c320;
/// `float -1.0` (`01012054`).
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// `double 0.5` (`01011588`).
const HALF_DOUBLE: u32 = 0x0101_1588;
/// The globals `00801ad0` and `00801bb0` fill (a shader's parameters; names not
/// confirmed).
const SHADER_ON: u32 = 0x0120_07fc;
const FADE_FLOAT_01201F2C: u32 = 0x0120_1f2c;
const FADE_COLOR_01201F30: u32 = 0x0120_1f30;
const FADE_HELD_01201F40: u32 = 0x0120_1f40;
const FADE_TEXTURE_01201F44: u32 = 0x0120_1f44;
const FADE_FLOAT_011AE614: u32 = 0x011a_e614;
const FADE_FLOAT_011AE618: u32 = 0x011a_e618;
const GLOW_TEXTURE_01200818: u32 = 0x0120_0818;
const GLOW_FLOAT_012007F0: u32 = 0x0120_07f0;
const GLOW_FLOAT_011AE5F8: u32 = 0x011a_e5f8;
const GLOW_FLOAT_012007F4: u32 = 0x0120_07f4;
const GLOW_BYTE_012007F8: u32 = 0x0120_07f8;
const GLOW_FLOAT_012007E8: u32 = 0x0120_07e8;
const GLOW_HELD_01200810: u32 = 0x0120_0810;
const GLOW_COLOR_01200800: u32 = 0x0120_0800;
const GLOW_FLOAT_011AE5EC: u32 = 0x011a_e5ec;
const GLOW_FLOAT_011AE5E8: u32 = 0x011a_e5e8;
const GLOW_FLOAT_011AE5FC: u32 = 0x011a_e5fc;
const GLOW_FLOAT_011AE5F0: u32 = 0x011a_e5f0;
const GLOW_FLOAT_011AE5F4: u32 = 0x011a_e5f4;
const GLOW_FLAG_012007EC: u32 = 0x0120_07ec;
/// `0043b480` (`thiscall`: a node's child count) and `0043b4a0` (`thiscall`, one word: a
/// child).
const CHILD_COUNT: u32 = 0x0043_b480;
const CHILD_AT: u32 = 0x0043_b4a0;
/// `Animation::FindSkinnedNode` (`004910d0`, engine map; `cdecl`, one word, `bool`).
const FIND_SKINNED_NODE: u32 = 0x0049_10d0;
/// `0043fad0` (`thiscall`) and `00653270` (`cdecl`: `011f49d8`, the result): the skin
/// instance of a geometry.
const SKIN_0043FAD0: u32 = 0x0043_fad0;
const SKIN_LOOKUP_00653270: u32 = 0x0065_3270;
const SKIN_TABLE_011F49D8: u32 = 0x011f_49d8;
/// `BSDismemberSkinInstance::SetBodyPartEnabled` (`005e4810`, engine map; `thiscall`: the
/// part, enabled).
const SET_BODY_PART_ENABLED: u32 = 0x005e_4810;
/// `0043f220` (`thiscall`, one word): an entry of a biped's table; its object at +8.
const BIPED_ENTRY_0043F220: u32 = 0x0043_f220;
/// `memset` (`00ec61c0`), `strcat_s`-like `00406d50` (`cdecl`: buffer, size, text) and
/// `", "`-like text at `01012630` (the separator; not confirmed).
const MEMSET: u32 = 0x00ec_61c0;
const APPEND_00406D50: u32 = 0x0040_6d50;
const SEPARATOR_01012630: u32 = 0x0101_2630;
/// `strncmp`-like `004564f0` (`cdecl`: text, entry, 0x40).
const COMPARE_004564F0: u32 = 0x0045_64f0;
/// `00802b80` (`thiscall`, one word): the base constructor `00802b20` runs.
const BASE_CONSTRUCT_00802B80: u32 = 0x0080_2b80;
/// The vtable `00802b20` stores.
const VTABLE_01079170: u32 = 0x0107_9170;
/// The compiler's `eh vector constructor iterator` (`00401050`, `stdcall`: array, element
/// size, count, constructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// The game's `operator new` (`00401000`, `cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// The table's global (`011db800`).
const TABLE_GLOBAL: u32 = 0x011d_b800;

/// Whether the target-only state lets the draw functions switch the body parts: (byte
/// +0x129 set and +0x2B4 clear, or `flag`) and `mode` not 1.
fn switches_parts(e: &Engine, at: u32, flag: u8, mode: u32) -> bool {
    ((e.mem.u8(at + 0x129) != 0 && e.mem.u8(at + 0x2b4) == 0) || flag != 0) && mode != 1
}

/// The body part's node `key` names on `object` (`004181e0` → vtable +0x180 →
/// `GetBodyPart`, then the model root's (vtable +0x1D0) slot +0x9C with the part's name
/// through a name local), or 0 when the object has no such part.
fn body_part_node(e: &mut Engine, object: u32, key: u32) -> u32 {
    let base = e.call(BASE_OBJECT_004181E0, &args![object]).u32();
    let data = e.vcall(base, 0x180, &[]).u32();
    let part = e.call(GET_BODY_PART, &args![data, key]).u32();
    if part == 0 {
        return 0;
    }
    let root = e.vcall(object, 0x1d0, &[]).u32();
    let name = e.call(BODY_PART_NAME, &args![part]).u32();
    e.with_stack(4, |e, local| {
        let made = e.call(NAME_CONSTRUCT, &args![local, name]).u32();
        let node = e.vcall(root, 0x9c, &args![made]).u32();
        e.call(NAME_DESTRUCT, &args![local]);
        node
    })
}

// Translated from 00801450 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00801450 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f10440, state variable around the two name locals) is not translated
/// Unnamed in the engine map (`thiscall`, two words): draws the VATS target. Nothing without
/// the actor at +4. With [`switches_parts`] the body parts are switched off first
/// ([`fn_00801da0`]) and on again at the end ([`fn_008022c0`]). Between: the accumulator of
/// `0045c670`'s object gets the object held at +0x2CC and 1 (`004a0fd0`, `004fb0b0`); for a
/// 3D-loaded actor (vtable +0x100) the process object (`008d8520`) gives a node (its slot
/// +0x190 with the actor's slot +0x1E8), accumulated alone when the part index is 0xE or
/// `mode` is 1 (and +0x2B4 clear or `flag`); otherwise (index not 0xE, or `mode` 1) the
/// node is shown while the root held at +8 is accumulated, and for a 3D-loaded actor so
/// are the nodes of the part at +0x23C and of each part in the list at +0x244. Then
/// `00b6c0d0` with the object held at +0x2CC, and the accumulator reset (0, 0).
pub fn fn_00801450(e: &mut Engine, this: Ptr, flag: u8, mode: u32) {
    let at = this.addr();
    if e.mem.u32(at + 4) == 0 {
        return;
    }
    if switches_parts(e, at, flag, mode) {
        fn_00801da0(e, this);
    }
    e.call(OBJECT_0043C4B0, &[]);
    let source = e.call(OBJECT_0045C670, &[]).u32();
    let actor = e.mem.u32(at + 4);
    let accumulator = e.call(ACCUMULATOR_008D80E0, &args![source]).u32();
    let held = e.call(FIRST_WORD, &args![at + 0x2cc]).u32();
    e.call(SET_004A0FD0, &args![accumulator, held]);
    e.call(SET_004FB0B0, &args![accumulator, 1u32]);
    let list = e.call(LIST_006629F0, &args![source]).u32();
    let mut process = 0;
    let mut node = 0;
    if e.vcall(e.mem.u32(at + 4), 0x100, &[]).bool() {
        process = e.call(PROCESS_OBJECT_008D8520, &args![actor]).u32();
    }
    if process != 0 {
        let key = e.vcall(actor, 0x1e8, &[]).u32();
        node = e.vcall(process, 0x190, &args![key]).u32();
    }
    if (e.mem.u32(at + 0x23c) == 0xe || mode == 1)
        && e.vcall(e.mem.u32(at + 4), 0x100, &[]).bool()
        && (e.mem.u8(at + 0x2b4) == 0 || flag != 0)
        && node != 0
        && process != 0
    {
        e.call(ACCUMULATE_SCENE, &args![list, node, accumulator]);
    }
    if e.mem.u32(at + 0x23c) != 0xe || mode == 1 {
        let mut restore = false;
        if node != 0 {
            restore = !e.call(IS_HIDDEN_00456610, &args![node]).bool();
            e.call(SET_HIDDEN_00450F90, &args![node, 1u32]);
        }
        let root = e.call(FIRST_WORD, &args![at + 8]).u32();
        e.call(ACCUMULATE_SCENE, &args![list, root, accumulator]);
        if e.vcall(e.mem.u32(at + 4), 0x100, &[]).bool() {
            let object = e.mem.u32(at + 4);
            let key = e.mem.u32(at + 0x23c);
            let part = body_part_node(e, object, key);
            if part != 0 {
                e.call(ACCUMULATE_SCENE, &args![list, part, accumulator]);
            }
            if e.mem.i32(at + 0x240) > 0 {
                let mut i = 0i32;
                while i < e.mem.i32(at + 0x240) {
                    let key = e
                        .mem
                        .u32(at.wrapping_add((i as u32).wrapping_mul(4)) + 0x244);
                    let part = body_part_node(e, object, key);
                    if part != 0 {
                        e.call(ACCUMULATE_SCENE, &args![list, part, accumulator]);
                    }
                    i += 1;
                }
            }
        }
        if node != 0 && restore {
            e.call(SET_HIDDEN_00450F90, &args![node, 0u32]);
        }
    }
    let held = e.call(FIRST_WORD, &args![at + 0x2cc]).u32();
    e.call(FINISH_00B6C0D0, &args![list, held, 0u32]);
    e.call(SET_004A0FD0, &args![accumulator, 0u32]);
    e.call(SET_004FB0B0, &args![accumulator, 0u32]);
    if switches_parts(e, at, flag, mode) {
        fn_008022c0(e, this);
    }
}

// Translated from 00801880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): draws the slot list. With slots
/// (count +0xC > 0): the accumulator of `0045c670`'s object gets the object held at +0x2CC;
/// the list goes through `004e9bb0` (on `0043c4b0`'s object), `0041fd00` and `00a694a0`
/// (with `0045bbe0`'s answer); each slot's held root is accumulated, the selected one
/// (+0x10) first flagged (`SetFlagRecurse(0x31, 1)`); then `00b6c0d0`, the accumulator
/// reset and the selected root's flag cleared again.
pub fn fn_00801880(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    if e.mem.i32(at + 0xc) <= 0 {
        return;
    }
    e.call(OBJECT_0043C4B0, &[]);
    let source = e.call(OBJECT_0045C670, &[]).u32();
    let accumulator = e.call(ACCUMULATOR_008D80E0, &args![source]).u32();
    let held = e.call(FIRST_WORD, &args![at + 0x2cc]).u32();
    e.call(SET_004A0FD0, &args![accumulator, held]);
    let list = e.call(LIST_006629F0, &args![source]).u32();
    let object = e.call(OBJECT_0043C4B0, &[]).u32();
    e.call(CALL_004E9BB0, &args![object, list]);
    e.call(CALL_0041FD00, &args![accumulator, list]);
    let value = e.call(CALL_0045BBE0, &args![list]).u32();
    e.call(CALL_00A694A0, &args![accumulator, value]);
    let mut selected = None;
    let mut i = 0i32;
    while i < e.mem.i32(at + 0xc) {
        let root = e
            .call(
                FIRST_WORD,
                &args![at.wrapping_add((i as u32).wrapping_mul(8)) + 0x18],
            )
            .u32();
        if i == e.mem.i32(at + 0x10) {
            e.call(SET_FLAG_RECURSE, &args![root, 0x31u32, 1u32]);
            selected = Some(root);
        }
        e.call(ACCUMULATE_SCENE, &args![list, root, accumulator]);
        i += 1;
    }
    let held = e.call(FIRST_WORD, &args![at + 0x2cc]).u32();
    e.call(FINISH_00B6C0D0, &args![list, held, 0u32]);
    e.call(SET_004A0FD0, &args![accumulator, 0u32]);
    if let Some(root) = selected {
        e.call(SET_FLAG_RECURSE, &args![root, 0x31u32, 0u32]);
    }
}

/// `004ebbc0(004e3270(), number)`: element `number` of the object.
fn element(e: &mut Engine, number: u32) -> u32 {
    let object = e.call(OBJECT_004E3270, &[]).u32();
    e.call(ELEMENT_004EBBC0, &args![object, number]).u32()
}

// Translated from 008019d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): switches elements 0xA and 0xB of
/// `004e3270`'s object off (`004f15a0` with 0), calls their vtable slot +0x14, and clears
/// `012007fc`.
pub fn fn_008019d0(e: &mut Engine, _this: Ptr) {
    let first = element(e, 0xa);
    e.call(SWITCH_004F15A0, &args![first, 0u32]);
    let second = element(e, 0xb);
    e.call(SWITCH_004F15A0, &args![second, 0u32]);
    let first = element(e, 0xa);
    e.vcall(first, 0x14, &[]);
    let second = element(e, 0xb);
    e.vcall(second, 0x14, &[]);
    e.mem.set_u32(SHADER_ON, 0);
}

// Translated from 00801a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, four words): unless `00709d00` answers true, and
/// with byte +0x129 set: [`fn_00801ad0`] when +0x2B4 is set, [`fn_00801bb0`] otherwise (the
/// words passed on, `flag` as its byte).
pub fn fn_00801a60(e: &mut Engine, this: Ptr, first: u32, texture: u32, third: u32, flag: u8) {
    if e.call(CHECK_00709D00, &[]).bool() {
        return;
    }
    let at = this.addr();
    if e.mem.u8(at + 0x129) == 0 {
        return;
    }
    if e.mem.u8(at + 0x2b4) != 0 {
        fn_00801ad0(e, this, first, texture, third, flag);
    } else {
        fn_00801bb0(e, this, first, texture, third, flag);
    }
}

/// The colour `00801ad0` and `00801bb0` read: a zeroed 16-byte colour (`00414430` with four
/// 0.0) filled by `SystemColorManager::GetColor(1, ..)`; its four words.
fn system_color_1(e: &mut Engine) -> [u32; 4] {
    e.with_stack(0x10, |e, color| {
        let zero = 0.0f32.to_bits();
        e.call(COLOR_CONSTRUCT, &args![color, zero, zero, zero, zero]);
        let manager = e.call(SYSTEM_COLOR_MANAGER_GET_INSTANCE, &[]).u32();
        e.call(SYSTEM_COLOR_MANAGER_GET_COLOR, &args![manager, 1u32, color]);
        let at = color.addr();
        [
            e.mem.u32(at),
            e.mem.u32(at + 4),
            e.mem.u32(at + 8),
            e.mem.u32(at + 12),
        ]
    })
}

/// A float setting's value, as `FLD`/`FSTP` copy it.
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(FLOAT_SETTING_VALUE, &args![setting]).u32();
    x87_quiet_bits(e.mem.u32(value))
}

// Translated from 00801ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, four words; the first, third and fourth unread):
/// sets the fade parameters: `01201f2c` = +0x2BC, `011ae614` and `011ae618` = settings
/// `011db7b4` and `011db694`, the colour (system colour 1) at `01201f30`, the object held at
/// +0x2C4 into `01201f40`, `texture`'s texture (`004bc320` with 0) into `01201f44`, and
/// element 0xB switched on.
pub fn fn_00801ad0(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    texture: u32,
    _unused_3: u32,
    _unused_4: u8,
) {
    let at = this.addr();
    let color = system_color_1(e);
    let value = e.mem.u32(at + 0x2bc);
    e.mem.set_u32(FADE_FLOAT_01201F2C, x87_quiet_bits(value));
    let value = setting_bits(e, SETTING_011DB7B4);
    e.mem.set_u32(FADE_FLOAT_011AE614, value);
    let value = setting_bits(e, SETTING_011DB694);
    e.mem.set_u32(FADE_FLOAT_011AE618, value);
    for (i, word) in color.iter().enumerate() {
        e.mem.set_u32(FADE_COLOR_01201F30 + 4 * i as u32, *word);
    }
    e.call(POINTER_ASSIGN, &args![FADE_HELD_01201F40, at + 0x2c4]);
    let image = e.call(GET_TEXTURE, &args![texture, 0u32]).u32();
    e.call(SET_POINTER_0066B0D0, &args![FADE_TEXTURE_01201F44, image]);
    let second = element(e, 0xb);
    e.call(SWITCH_004F15A0, &args![second, 1u32]);
}

// Translated from 00801bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, four words; the first and third unread): sets the
/// glow parameters: `012007fc` = 1, element 0xA switched on, `texture`'s texture into
/// `01200818`; with byte +0x284 set `012007f0`, `011ae5f8` = settings `011db6e4`,
/// `011db79c`, `012007f4` = +0x28C and byte `012007f8` = 0, otherwise 0.0, 0.0 and -1.0;
/// `012007e8` = 0.0; the object held at +0x2C4 into `01200810`, the colour (system colour 1)
/// at `01200800`; `011ae5ec` and `011ae5e8` = settings `011db714` + +0x2AC and `011db720` +
/// +0x2B0, each scaled (with byte +0x298 set) by 1 + +0x2A8 * 0.5 and 1 + +0x2A8;
/// `011ae5fc` = +0x2C0, `011ae5f0` = setting `011db778`, `011ae5f4` = 1.0 and `012007ec` =
/// whether `flag` is set.
pub fn fn_00801bb0(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    texture: u32,
    _unused_3: u32,
    flag: u8,
) {
    let at = this.addr();
    let color = system_color_1(e);
    e.mem.set_u32(SHADER_ON, 1);
    let first = element(e, 0xa);
    e.call(SWITCH_004F15A0, &args![first, 1u32]);
    let image = e.call(GET_TEXTURE, &args![texture, 0u32]).u32();
    e.call(SET_POINTER_0066B0D0, &args![GLOW_TEXTURE_01200818, image]);
    if e.mem.u8(at + 0x284) != 0 {
        let value = setting_bits(e, SETTING_011DB6E4);
        e.mem.set_u32(GLOW_FLOAT_012007F0, value);
        let value = setting_bits(e, SETTING_011DB79C);
        e.mem.set_u32(GLOW_FLOAT_011AE5F8, value);
        let value = e.mem.u32(at + 0x28c);
        e.mem.set_u32(GLOW_FLOAT_012007F4, x87_quiet_bits(value));
        e.mem.set_u8(GLOW_BYTE_012007F8, 0);
    } else {
        e.mem.set_u32(GLOW_FLOAT_012007F0, 0.0f32.to_bits());
        e.mem.set_u32(GLOW_FLOAT_011AE5F8, 0.0f32.to_bits());
        let value = e.mem.u32(MINUS_ONE_FLOAT);
        e.mem.set_u32(GLOW_FLOAT_012007F4, x87_quiet_bits(value));
    }
    e.mem.set_u32(GLOW_FLOAT_012007E8, 0.0f32.to_bits());
    e.call(POINTER_ASSIGN, &args![GLOW_HELD_01200810, at + 0x2c4]);
    for (i, word) in color.iter().enumerate() {
        e.mem.set_u32(GLOW_COLOR_01200800 + 4 * i as u32, *word);
    }
    let value = setting_bits(e, SETTING_011DB714);
    e.mem.set_u32(GLOW_FLOAT_011AE5EC, value);
    let value = setting_bits(e, SETTING_011DB720);
    e.mem.set_u32(GLOW_FLOAT_011AE5E8, value);
    let sum = x87_add(
        x87_load32(e.mem.u32(GLOW_FLOAT_011AE5EC)),
        x87_m32(e.mem.u32(at + 0x2ac)),
    );
    e.mem.set_u32(GLOW_FLOAT_011AE5EC, x87_store32(sum));
    let sum = x87_add(
        x87_load32(e.mem.u32(GLOW_FLOAT_011AE5E8)),
        x87_m32(e.mem.u32(at + 0x2b0)),
    );
    e.mem.set_u32(GLOW_FLOAT_011AE5E8, x87_store32(sum));
    if e.mem.u8(at + 0x298) != 0 {
        let half = x87_mul(x87_load32(e.mem.u32(at + 0x2a8)), e.mem.f64(HALF_DOUBLE));
        // FLD1; FADDP ST(1),ST(0) (bytes DE C1).
        let scale = x87_add(half, 1.0);
        let value = x87_mul(scale, x87_m32(e.mem.u32(GLOW_FLOAT_011AE5EC)));
        e.mem.set_u32(GLOW_FLOAT_011AE5EC, x87_store32(value));
        let scale = x87_add(x87_load32(e.mem.u32(at + 0x2a8)), 1.0);
        let value = x87_mul(scale, x87_m32(e.mem.u32(GLOW_FLOAT_011AE5E8)));
        e.mem.set_u32(GLOW_FLOAT_011AE5E8, x87_store32(value));
    }
    let value = e.mem.u32(at + 0x2c0);
    e.mem.set_u32(GLOW_FLOAT_011AE5FC, x87_quiet_bits(value));
    let value = setting_bits(e, SETTING_011DB778);
    e.mem.set_u32(GLOW_FLOAT_011AE5F0, value);
    e.mem.set_u32(GLOW_FLOAT_011AE5F4, 1.0f32.to_bits());
    e.mem.set_u32(GLOW_FLAG_012007EC, u32::from(flag != 0));
}

/// The skinned geometry walk `00801da0` and `008022c0` share: for each child of `root`
/// (`0043b480`/`0043b4a0`) that `FindSkinnedNode` accepts, the geometry from its slot +0x18
/// and then each of its slot +0xC list's children's (slot +0x18) is looked up as a skin
/// (`0043fad0`, `00653270` on `011f49d8`), and `found` runs on every skin found; a child it
/// does not accept gets `00450f90(other)`.
fn walk_skins(e: &mut Engine, root: u32, other: u32, mut found: impl FnMut(&mut Engine, u32)) {
    let mut i = 0u32;
    while i < e.call(CHILD_COUNT, &args![root]).u32() {
        let child = e.call(CHILD_AT, &args![root, i]).u32();
        if child != 0 {
            if e.call(FIND_SKINNED_NODE, &args![child]).bool() {
                let list = e.vcall(child, 0xc, &[]).u32();
                let mut geometry = e.vcall(child, 0x18, &[]).u32();
                let mut j = 0u32;
                loop {
                    if geometry == 0 && (list == 0 || j >= e.call(CHILD_COUNT, &args![list]).u32())
                    {
                        break;
                    }
                    if list != 0 {
                        geometry = if e.call(CHILD_AT, &args![list, j]).u32() != 0 {
                            let next = e.call(CHILD_AT, &args![list, j]).u32();
                            e.vcall(next, 0x18, &[]).u32()
                        } else {
                            0
                        };
                        j = j.wrapping_add(1);
                    }
                    let skin = if geometry != 0 {
                        e.call(SKIN_0043FAD0, &args![geometry]).u32()
                    } else {
                        0
                    };
                    let instance = e
                        .call(SKIN_LOOKUP_00653270, &args![SKIN_TABLE_011F49D8, skin])
                        .u32();
                    if instance != 0 {
                        found(e, instance);
                    }
                    geometry = 0;
                }
            } else {
                e.call(SET_HIDDEN_00450F90, &args![child, other]);
            }
        }
        i = i.wrapping_add(1);
    }
}

// Translated from 00801da0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00801da0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef0598, state variable around the name local) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): switches the VATS target's body
/// parts for drawing. Nothing unless the actor at +4 is 3D-loaded (vtable +0x100) and the
/// part index +0x23C is neither 0xE nor 0xF. Every skin of the model root (vtable +0x1D0)
/// gets each part 0..0xE enabled only when it is the index ([`fn_00802220`]), and enabled
/// again for each part in the list at +0x244; other children are hidden. With index 0 the
/// part nodes 0..0xE of the actor are shown only for the index (`00450f90` with 0, others
/// with 1). Then [`fn_00802260`] hides the biped entries 3, 4 (unless the index is 3 or
/// 4), 4 (unless 5 or 6) and, unless the index is 1 or 2, 0xA..0xD, 9, 0xE..0x10, 1 and 0.
pub fn fn_00801da0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    if !e.vcall(e.mem.u32(at + 4), 0x100, &[]).bool() {
        return;
    }
    if e.mem.u32(at + 0x23c) == 0xe || e.mem.u32(at + 0x23c) == 0xf {
        return;
    }
    let actor = e.mem.u32(at + 4);
    let root = e.vcall(e.mem.u32(at + 4), 0x1d0, &[]).u32();
    walk_skins(e, root, 1, |e, instance| {
        let mut part = 0i32;
        while part < 0xf {
            let selected = part as u32 == e.mem.u32(at + 0x23c);
            fn_00802220(e, this, instance, part as u32, u8::from(selected));
            let mut k = 0i32;
            while k < e.mem.i32(at + 0x240) {
                if part as u32
                    == e.mem
                        .u32(at.wrapping_add((k as u32).wrapping_mul(4)) + 0x244)
                {
                    fn_00802220(e, this, instance, part as u32, 1);
                }
                k += 1;
            }
            part += 1;
        }
    });
    if actor != 0 && e.mem.u32(at + 0x23c) == 0 {
        let mut part = 0i32;
        while part < 0xf {
            let key = e.mem.u32(at + 0x23c);
            let node = body_part_node(e, actor, key);
            if node != 0 && part as u32 == e.mem.u32(at + 0x23c) {
                e.call(SET_HIDDEN_00450F90, &args![node, 0u32]);
            } else if node != 0 {
                e.call(SET_HIDDEN_00450F90, &args![node, 1u32]);
            }
            part += 1;
        }
    }
    let index = e.mem.u32(at + 0x23c);
    if index != 3 && e.mem.u32(at + 0x23c) != 4 {
        fn_00802260(e, this, actor, 3, 0);
    }
    if e.mem.u32(at + 0x23c) != 5 && e.mem.u32(at + 0x23c) != 6 {
        fn_00802260(e, this, actor, 4, 0);
    }
    if e.mem.u32(at + 0x23c) == 1 || e.mem.u32(at + 0x23c) == 2 {
        return;
    }
    for slot in [0xau32, 0xb, 0xc, 0xd, 9, 0xe, 0xf, 0x10, 1, 0] {
        fn_00802260(e, this, actor, slot, 0);
    }
}

// Translated from 00802220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three words): `SetBodyPartEnabled` on `instance`
/// for `part` (as its low 16 bits) and for `part * 1000`, with `enabled` (zero-extended).
pub fn fn_00802220(e: &mut Engine, _this: Ptr, instance: u32, part: u32, enabled: u8) {
    let enabled = u32::from(enabled);
    e.call(
        SET_BODY_PART_ENABLED,
        &args![instance, part & 0xffff, enabled],
    );
    e.call(
        SET_BODY_PART_ENABLED,
        &args![instance, part.wrapping_mul(1000), enabled],
    );
}

// Translated from 00802260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three words): the object of `actor`'s biped
/// (vtable +0x1E8) entry `slot` (`0043f220`, its +8) is hidden when `visible` is clear and
/// shown when set (`00450f90` with `visible == 0`).
pub fn fn_00802260(e: &mut Engine, _this: Ptr, actor: u32, slot: u32, visible: u8) {
    let biped = e.vcall(actor, 0x1e8, &[]).u32();
    if biped == 0 {
        return;
    }
    let entry = e.call(BIPED_ENTRY_0043F220, &args![biped, slot]).u32();
    let object = e.mem.u32(entry + 8);
    if object != 0 {
        e.call(SET_HIDDEN_00450F90, &args![object, u32::from(visible == 0)]);
    }
}

// Translated from 008022c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008022c0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f10468, state variable around the name local) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): undoes [`fn_00801da0`]. Nothing
/// unless the actor at +4 is 3D-loaded (vtable +0x100) and the part index +0x23C is neither
/// 0xE nor 0xF. Shows the biped entries 3, 4, 0xA..0xD, 9, 0xE..0x10, 1 and 0
/// ([`fn_00802260`] with 1); every skin of the model root gets parts `p` and `p * 1000`
/// enabled and `p + 100` and `p + 200` disabled for `p` in 0..0xE, other children shown;
/// with index 0 the actor's part nodes 0..0xE are shown.
pub fn fn_008022c0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    if !e.vcall(e.mem.u32(at + 4), 0x100, &[]).bool() {
        return;
    }
    if e.mem.u32(at + 0x23c) == 0xe || e.mem.u32(at + 0x23c) == 0xf {
        return;
    }
    let actor = e.mem.u32(at + 4);
    for slot in [3u32, 4, 0xa, 0xb, 0xc, 0xd, 9, 0xe, 0xf, 0x10, 1, 0] {
        fn_00802260(e, this, actor, slot, 1);
    }
    let root = e.vcall(e.mem.u32(at + 4), 0x1d0, &[]).u32();
    walk_skins(e, root, 0, |e, instance| {
        let mut part = 0i32;
        while part < 0xf {
            let p = part as u32;
            e.call(SET_BODY_PART_ENABLED, &args![instance, p & 0xffff, 1u32]);
            e.call(
                SET_BODY_PART_ENABLED,
                &args![instance, p.wrapping_mul(1000), 1u32],
            );
            e.call(SET_BODY_PART_ENABLED, &args![instance, p + 100, 0u32]);
            e.call(SET_BODY_PART_ENABLED, &args![instance, p + 200, 0u32]);
            part += 1;
        }
    });
    if actor != 0 && e.mem.u32(at + 0x23c) == 0 {
        let mut part = 0i32;
        while part < 0xf {
            let key = e.mem.u32(at + 0x23c);
            let node = body_part_node(e, actor, key);
            if node != 0 {
                e.call(SET_HIDDEN_00450F90, &args![node, 0u32]);
            }
            part += 1;
        }
    }
}

// Translated from 008026b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): for each of the 15 slots, clears
/// byte +0x119 and, for the effects at +0x12C and then +0x16C, lets each finish
/// (`00c4edf0` with an out word and 1), deletes it (`004a4410` with 1) and clears it.
pub fn fn_008026b0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.with_stack(4, |e, out| {
        for i in 0..0xfu32 {
            e.mem.set_u8(at + 0x119 + i, 0);
            e.mem.set_u32(out.addr(), 0);
            for array in [0x12cu32, 0x16c] {
                let field = at + array + i * 4;
                let effect = e.mem.u32(field);
                if effect != 0 {
                    e.call(EFFECT_QUERY_00C4EDF0, &args![effect, out, 1u32]);
                    let effect = e.mem.u32(field);
                    if effect != 0 {
                        e.call(EFFECT_DELETE, &args![effect, 1u32]);
                    }
                    e.mem.set_u32(field, 0);
                }
            }
        }
    });
}

/// `00c4edf0` (`thiscall`: an out word, 1) and `004a4410` (`thiscall`, 1): an effect's
/// finish and its scalar deleting destructor.
const EFFECT_QUERY_00C4EDF0: u32 = 0x00c4_edf0;
const EFFECT_DELETE: u32 = 0x004a_4410;

// Translated from 008027d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008027d0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f1049b, state variable around the member at +4) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): the text table's constructor: +0 =
/// 1, the member at +4 ([`fn_00802b20`] with 0x25), +0x14, +0x18 and +0x1C = 0, and the
/// 0x40 entries of 0x44 bytes at +0x20 constructed with [`fn_00802860`] (through the vector
/// constructor iterator). Returns `this`.
pub fn fn_008027d0(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, 1);
    fn_00802b20(e, Ptr::new(at + 4), 0x25);
    e.mem.set_u32(at + 0x14, 0);
    e.mem.set_u32(at + 0x18, 0);
    e.mem.set_u32(at + 0x1c, 0);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![at + 0x20, 0x44u32, 0x40u32, 0x0080_2860u32],
    );
    this
}

// Translated from 00802860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): a table entry's constructor:
/// clears the word at +0x40. Returns `this`.
pub fn fn_00802860(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr() + 0x40, 0);
    this
}

// Translated from 00802880 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00802880 - the /GS security cookie check (00ec408c) at the end is not translated
/// Unnamed in the engine map (`thiscall`, one word): hands the table's entries to `callback`
/// (`cdecl`, the text) five at a time: a zeroed 0x200-byte buffer gets up to five entries
/// (+0x20 + 0x44 * n) appended (`00406d50`), separated by the text at `01012630`, and is
/// passed to `callback` for each group.
pub fn fn_00802880(e: &mut Engine, this: Ptr, callback: u32) {
    let at = this.addr();
    e.with_stack(0x200, |e, buffer| {
        let text = buffer.addr();
        e.mem.set_u8(text, 0);
        e.call(MEMSET, &args![text + 1, 0u32, 0x1ffu32]);
        let mut start = 0i32;
        while start < e.mem.i32(at + 0x14) {
            e.mem.set_u8(text, 0);
            let mut n = 0i32;
            while n < 5 && n.wrapping_add(start) < e.mem.i32(at + 0x14) {
                let entry = at
                    .wrapping_add((start.wrapping_add(n) as u32).wrapping_mul(0x44))
                    .wrapping_add(0x20);
                e.call(APPEND_00406D50, &args![text, 0x200u32, entry]);
                if start.wrapping_add(n).wrapping_add(1) < e.mem.i32(at + 0x14) {
                    e.call(APPEND_00406D50, &args![text, 0x200u32, SEPARATOR_01012630]);
                }
                n += 1;
            }
            e.call(callback, &args![text]);
            start = start.wrapping_add(5);
        }
    });
}

// Translated from 008029c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words): selects the entry named `name`
/// ([`fn_00802a20`]): +0x18 = its index and +0x1C = `value`. Returns whether it was found.
pub fn fn_008029c0(e: &mut Engine, this: Ptr, name: u32, value: u32) -> bool {
    let index = fn_00802a20(e, this, name);
    if index != -1 {
        e.mem.set_u32(this.addr() + 0x18, index as u32);
        e.mem.set_u32(this.addr() + 0x1c, value);
    }
    index != -1
}

// Translated from 00802a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words, neither read): returns 0.
pub fn fn_00802a00(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    0
}

// Translated from 00802a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): the index of the first entry whose
/// text matches `name` (`004564f0` with 0x40 answering 0), or -1.
pub fn fn_00802a20(e: &mut Engine, this: Ptr, name: u32) -> i32 {
    let at = this.addr();
    let mut found = -1i32;
    let mut i = 0i32;
    while i < e.mem.i32(at + 0x14) && found == -1 {
        let entry = at
            .wrapping_add((i as u32).wrapping_mul(0x44))
            .wrapping_add(0x20);
        if e.call(COMPARE_004564F0, &args![name, entry, 0x40u32]).u32() == 0 {
            found = i;
        }
        i += 1;
    }
    found
}

// Translated from 00802a90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00802a90 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f109cb, state variable around the construction) is not translated
/// Unnamed in the engine map (no arguments): creates the text table once: when `011db800`
/// is null, a 0x1120-byte block (`operator new`) constructed with [`fn_008027d0`] (0 when
/// the allocation fails) goes there.
pub fn fn_00802a90(e: &mut Engine) {
    if e.mem.u32(TABLE_GLOBAL) != 0 {
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![0x1120u32]).u32();
    let table = if block != 0 {
        fn_008027d0(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.mem.set_u32(TABLE_GLOBAL, table);
}

// Translated from 00802b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): `00802b80(this, size)`, then the
/// vtable `01079170`. Returns `this`.
pub fn fn_00802b20(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    e.call(BASE_CONSTRUCT_00802B80, &args![this, size]);
    e.mem.set_u32(this.addr(), VTABLE_01079170);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_1450, fn_00801450(Ptr, u8, u32)),
        entry!(0x0080_1880, fn_00801880(Ptr)),
        entry!(0x0080_19d0, fn_008019d0(Ptr)),
        entry!(0x0080_1a60, fn_00801a60(Ptr, u32, u32, u32, u8)),
        entry!(0x0080_1ad0, fn_00801ad0(Ptr, u32, u32, u32, u8)),
        entry!(0x0080_1bb0, fn_00801bb0(Ptr, u32, u32, u32, u8)),
        entry!(0x0080_1da0, fn_00801da0(Ptr)),
        entry!(0x0080_2220, fn_00802220(Ptr, u32, u32, u8)),
        entry!(0x0080_2260, fn_00802260(Ptr, u32, u32, u8)),
        entry!(0x0080_22c0, fn_008022c0(Ptr)),
        entry!(0x0080_26b0, fn_008026b0(Ptr)),
        entry!(0x0080_27d0, fn_008027d0(Ptr) -> Ptr),
        entry!(0x0080_2860, fn_00802860(Ptr) -> Ptr),
        entry!(0x0080_2880, fn_00802880(Ptr, u32)),
        entry!(0x0080_29c0, fn_008029c0(Ptr, u32, u32) -> bool),
        entry!(0x0080_2a00, fn_00802a00(Ptr, u32, u32) -> u32),
        entry!(0x0080_2a20, fn_00802a20(Ptr, u32) -> i32),
        entry!(0x0080_2a90, fn_00802a90()),
        entry!(0x0080_2b20, fn_00802b20(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = [(u32, Vec<u32>)];

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object whose vtable slots answer the given words.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x400);
        for &(slot, value) in slots {
            let target = vtable + slot;
            e.mem.set_u32(vtable + slot, target);
            e.register_double(target, move |_, _| value.into_ret());
        }
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        object
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [
            OBJECT_0043C4B0,
            SET_004A0FD0,
            SET_004FB0B0,
            ACCUMULATE_SCENE,
            FINISH_00B6C0D0,
            IS_HIDDEN_00456610,
            SET_HIDDEN_00450F90,
            NAME_DESTRUCT,
            SWITCH_004F15A0,
            CALL_004E9BB0,
            CALL_0041FD00,
            CALL_0045BBE0,
            CALL_00A694A0,
            SET_FLAG_RECURSE,
            COLOR_CONSTRUCT,
            SYSTEM_COLOR_MANAGER_GET_INSTANCE,
            POINTER_ASSIGN,
            SET_POINTER_0066B0D0,
            SET_BODY_PART_ENABLED,
            EFFECT_QUERY_00C4EDF0,
            EFFECT_DELETE,
            MEMSET,
            APPEND_00406D50,
            BASE_CONSTRUCT_00802B80,
            VECTOR_CONSTRUCTOR_ITERATOR,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(OBJECT_0045C670, |_, _| 0x4500u32.into_ret());
        e.register(ACCUMULATOR_008D80E0, |_, _| 0x4600u32.into_ret());
        e.register(LIST_006629F0, |_, _| 0x4700u32.into_ret());
        e.register(GET_BODY_PART, |_, a| (a[1] + 0x3000).into_ret());
        e.register(BODY_PART_NAME, |_, _| 0x4e00u32.into_ret());
        e.register(NAME_CONSTRUCT, |_, a| a[0].into_ret());
        e
    }

    /// The scene of a VATS target: returns (this, actor). The actor is 3D-loaded, its model
    /// root's slot +0x9C gives the part node 0x9999, its slot +0x1E8 the biped (whose
    /// entries hold 0x5500 at +8), and its base object's slot +0x180 the body part data.
    fn target(e: &mut Engine) -> (u32, u32) {
        let root = object(e, &[(0x9c, 0x9999)]);
        let biped = object(e, &[]);
        let actor = object(e, &[(0x100, 1), (0x1e8, biped), (0x1d0, root)]);
        let base = object(e, &[(0x180, 0xda7a)]);
        e.register_double(BASE_OBJECT_004181E0, move |_, _| base.into_ret());
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry + 8, 0x5500);
        e.register_double(BIPED_ENTRY_0043F220, move |_, _| entry.into_ret());
        let this = e.mem.alloc(0x2d0);
        e.mem.set_u32(this + 4, actor);
        e.mem.set_u32(this + 8, 0x8800);
        e.mem.set_u32(this + 0x2cc, 0x2cc0);
        (this, actor)
    }

    /// Gives the actor's model root one skinned child whose list holds one geometry with
    /// the skin instance 0x6800.
    fn skinned(e: &mut Engine, actor: u32) {
        let root = e.vcall(actor, 0x1d0, &[]).u32();
        let geometry = object(e, &[(0x18, 0x6600)]);
        let list = e.mem.alloc(4);
        let child = object(e, &[(0xc, list), (0x18, 0)]);
        e.register_double(CHILD_COUNT, move |_, a| {
            u32::from(a[0] == root || a[0] == list).into_ret()
        });
        e.register_double(CHILD_AT, move |_, a| {
            if a[0] == root {
                child.into_ret()
            } else {
                geometry.into_ret()
            }
        });
        e.register(FIND_SKINNED_NODE, |_, _| 1u32.into_ret());
        e.register(SKIN_0043FAD0, |_, a| (a[0] + 0x100).into_ret());
        e.register(SKIN_LOOKUP_00653270, |_, a| {
            assert_eq!(a[0], SKIN_TABLE_011F49D8);
            (a[1] + 0x100).into_ret()
        });
    }

    #[test]
    fn fn_00801450_draws_the_target_and_its_parts() {
        let mut e = engine();
        let (this, _) = target(&mut e);
        let process = object(&mut e, &[(0x190, 0x7000)]);
        e.register_double(PROCESS_OBJECT_008D8520, move |_, _| process.into_ret());
        e.mem.set_u32(this + 0x23c, 2);
        e.mem.set_u32(this + 0x240, 1);
        e.mem.set_u32(this + 0x244, 7);
        e.call_log = Some(vec![]);
        e.call(0x0080_1450, &args![this, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ACCUMULATE_SCENE),
            vec![
                vec![0x4700, 0x8800, 0x4600],
                vec![0x4700, 0x9999, 0x4600],
                vec![0x4700, 0x9999, 0x4600],
            ]
        );
        assert_eq!(
            calls_to(&log, GET_BODY_PART),
            vec![vec![0xda7a, 2], vec![0xda7a, 7]]
        );
        assert_eq!(
            calls_to(&log, SET_HIDDEN_00450F90),
            vec![vec![0x7000, 1], vec![0x7000, 0]]
        );
        assert_eq!(
            calls_to(&log, FINISH_00B6C0D0),
            vec![vec![0x4700, 0x2cc0, 0]]
        );
        assert_eq!(
            calls_to(&log, SET_004A0FD0),
            vec![vec![0x4600, 0x2cc0], vec![0x4600, 0]]
        );
        assert_eq!(
            calls_to(&log, SET_004FB0B0),
            vec![vec![0x4600, 1], vec![0x4600, 0]]
        );
        // The part index 0xE without mode 1: only the process node.
        e.mem.set_u32(this + 0x23c, 0xe);
        e.call_log = Some(vec![]);
        e.call(0x0080_1450, &args![this, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ACCUMULATE_SCENE),
            vec![vec![0x4700, 0x7000, 0x4600]]
        );
        // No actor: nothing.
        e.mem.set_u32(this + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0080_1450, &args![this, 0u32, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00801880_draws_the_slots_flagging_the_selected_one() {
        let mut e = engine();
        let this = e.mem.alloc(0x2d0);
        e.call_log = Some(vec![]);
        e.call(0x0080_1880, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.register(OBJECT_0043C4B0, |_, _| 0x43c4u32.into_ret());
        e.register(CALL_0045BBE0, |_, _| 0x45bbu32.into_ret());
        e.mem.set_u32(this + 0xc, 2);
        e.mem.set_u32(this + 0x10, 1);
        e.mem.set_u32(this + 0x18, 0xa1);
        e.mem.set_u32(this + 0x20, 0xa2);
        e.mem.set_u32(this + 0x2cc, 0x2cc0);
        e.call_log = Some(vec![]);
        e.call(0x0080_1880, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SET_FLAG_RECURSE),
            vec![vec![0xa2, 0x31, 1], vec![0xa2, 0x31, 0]]
        );
        assert_eq!(
            calls_to(&log, ACCUMULATE_SCENE),
            vec![vec![0x4700, 0xa1, 0x4600], vec![0x4700, 0xa2, 0x4600]]
        );
        assert_eq!(calls_to(&log, CALL_004E9BB0), vec![vec![0x43c4, 0x4700]]);
        assert_eq!(calls_to(&log, CALL_0041FD00), vec![vec![0x4600, 0x4700]]);
        assert_eq!(calls_to(&log, CALL_00A694A0), vec![vec![0x4600, 0x45bb]]);
        assert_eq!(
            calls_to(&log, FINISH_00B6C0D0),
            vec![vec![0x4700, 0x2cc0, 0]]
        );
    }

    /// Elements 0xA and 0xB of `004e3270`'s object (each with a slot +0x14).
    fn elements(e: &mut Engine) -> (u32, u32) {
        let first = object(e, &[(0x14, 0)]);
        let second = object(e, &[(0x14, 0)]);
        e.register(OBJECT_004E3270, |_, _| 0x4e30u32.into_ret());
        e.register_double(ELEMENT_004EBBC0, move |_, a| {
            assert_eq!(a[0], 0x4e30);
            if a[1] == 0xa {
                first.into_ret()
            } else {
                second.into_ret()
            }
        });
        (first, second)
    }

    fn map_globals(e: &mut Engine) {
        e.map(0x0101_1000, 0x2000);
        e.mem.set_f32(MINUS_ONE_FLOAT, -1.0);
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        e.map(0x011a_e000, 0x1000);
        e.map(0x011d_b000, 0x1000);
        e.map(0x0120_0000, 0x2000);
        e.register(FLOAT_SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(SYSTEM_COLOR_MANAGER_GET_COLOR, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[2] + 4 * i, 0xc0 + i);
            }
            Ret::default()
        });
        e.register(GET_TEXTURE, |_, a| (a[0] + 1).into_ret());
    }

    #[test]
    fn fn_008019d0_switches_both_elements_off() {
        let mut e = engine();
        map_globals(&mut e);
        let (first, second) = elements(&mut e);
        e.mem.set_u32(SHADER_ON, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_19d0, &args![0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SWITCH_004F15A0),
            vec![vec![first, 0], vec![second, 0]]
        );
        let first_slot = e.mem.u32(e.mem.u32(first) + 0x14);
        let second_slot = e.mem.u32(e.mem.u32(second) + 0x14);
        assert_eq!(calls_to(&log, first_slot), vec![vec![first]]);
        assert_eq!(calls_to(&log, second_slot), vec![vec![second]]);
        assert_eq!(e.mem.u32(SHADER_ON), 0);
    }

    #[test]
    fn fn_00801a60_picks_the_fade_or_the_glow() {
        let mut e = engine();
        map_globals(&mut e);
        let (first, second) = elements(&mut e);
        e.register(CHECK_00709D00, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x2d0);
        e.mem.set_u8(this + 0x129, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_1a60, &args![this, 0u32, 0x700u32, 0u32, 1u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        e.register(CHECK_00709D00, |_, _| Ret::default());
        e.mem.set_u8(this + 0x2b4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_1a60, &args![this, 0u32, 0x700u32, 0u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SWITCH_004F15A0), vec![vec![second, 1]]);
        e.mem.set_u8(this + 0x2b4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0080_1a60, &args![this, 0u32, 0x700u32, 0u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SWITCH_004F15A0), vec![vec![first, 1]]);
        assert_eq!(e.mem.u32(GLOW_FLAG_012007EC), 1);
        // Without +0x129 nothing runs.
        e.mem.set_u8(this + 0x129, 0);
        e.call_log = Some(vec![]);
        e.call(0x0080_1a60, &args![this, 0u32, 0x700u32, 0u32, 1u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn fn_00801ad0_sets_the_fade_parameters() {
        let mut e = engine();
        map_globals(&mut e);
        let (_, second) = elements(&mut e);
        e.mem.set_f32(SETTING_011DB7B4 + 4, 2.0);
        e.mem.set_f32(SETTING_011DB694 + 4, 3.0);
        let this = e.mem.alloc(0x2d0);
        e.mem.set_f32(this + 0x2bc, 0.5);
        e.call_log = Some(vec![]);
        e.call(0x0080_1ad0, &args![this, 0u32, 0x700u32, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.f32(FADE_FLOAT_01201F2C), 0.5);
        assert_eq!(e.mem.f32(FADE_FLOAT_011AE614), 2.0);
        assert_eq!(e.mem.f32(FADE_FLOAT_011AE618), 3.0);
        assert_eq!(e.mem.u32(FADE_COLOR_01201F30 + 12), 0xc3);
        assert_eq!(
            calls_to(&log, POINTER_ASSIGN),
            vec![vec![FADE_HELD_01201F40, this + 0x2c4]]
        );
        assert_eq!(calls_to(&log, GET_TEXTURE), vec![vec![0x700, 0]]);
        assert_eq!(
            calls_to(&log, SET_POINTER_0066B0D0),
            vec![vec![FADE_TEXTURE_01201F44, 0x701]]
        );
        assert_eq!(calls_to(&log, SWITCH_004F15A0), vec![vec![second, 1]]);
        let color = &calls_to(&log, COLOR_CONSTRUCT)[0];
        assert_eq!(color[1..], [0, 0, 0, 0]);
    }

    #[test]
    fn fn_00801bb0_sets_the_glow_parameters() {
        let mut e = engine();
        map_globals(&mut e);
        let (first, _) = elements(&mut e);
        e.mem.set_f32(SETTING_011DB6E4 + 4, 5.0);
        e.mem.set_f32(SETTING_011DB79C + 4, 6.0);
        e.mem.set_f32(SETTING_011DB714 + 4, 2.0);
        e.mem.set_f32(SETTING_011DB720 + 4, 3.0);
        e.mem.set_f32(SETTING_011DB778 + 4, 9.0);
        let this = e.mem.alloc(0x2d0);
        e.mem.set_f32(this + 0x2ac, 1.0);
        e.mem.set_f32(this + 0x2b0, 1.0);
        e.mem.set_f32(this + 0x28c, 0.25);
        e.mem.set_f32(this + 0x2c0, 0.75);
        e.mem.set_u8(GLOW_BYTE_012007F8, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_1bb0, &args![this, 0u32, 0x700u32, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(SHADER_ON), 1);
        assert_eq!(calls_to(&log, SWITCH_004F15A0), vec![vec![first, 1]]);
        assert_eq!(
            calls_to(&log, SET_POINTER_0066B0D0),
            vec![vec![GLOW_TEXTURE_01200818, 0x701]]
        );
        assert_eq!(e.mem.f32(GLOW_FLOAT_012007F0), 0.0);
        assert_eq!(e.mem.f32(GLOW_FLOAT_012007F4), -1.0);
        assert_eq!(e.mem.u8(GLOW_BYTE_012007F8), 1);
        assert_eq!(e.mem.f32(GLOW_FLOAT_011AE5EC), 3.0);
        assert_eq!(e.mem.f32(GLOW_FLOAT_011AE5E8), 4.0);
        assert_eq!(e.mem.f32(GLOW_FLOAT_011AE5FC), 0.75);
        assert_eq!(e.mem.f32(GLOW_FLOAT_011AE5F0), 9.0);
        assert_eq!(e.mem.f32(GLOW_FLOAT_011AE5F4), 1.0);
        assert_eq!(e.mem.u32(GLOW_COLOR_01200800), 0xc0);
        assert_eq!(e.mem.u32(GLOW_FLAG_012007EC), 0);
        assert_eq!(
            calls_to(&log, POINTER_ASSIGN),
            vec![vec![GLOW_HELD_01200810, this + 0x2c4]]
        );
        // The progress and the timed effect.
        e.mem.set_u8(this + 0x284, 1);
        e.mem.set_u8(this + 0x298, 1);
        e.mem.set_f32(this + 0x2a8, 2.0);
        e.call(0x0080_1bb0, &args![this, 0u32, 0x700u32, 0u32, 4u32]);
        assert_eq!(e.mem.f32(GLOW_FLOAT_012007F0), 5.0);
        assert_eq!(e.mem.f32(GLOW_FLOAT_011AE5F8), 6.0);
        assert_eq!(e.mem.f32(GLOW_FLOAT_012007F4), 0.25);
        assert_eq!(e.mem.u8(GLOW_BYTE_012007F8), 0);
        assert_eq!(e.mem.f32(GLOW_FLOAT_011AE5EC), 6.0);
        assert_eq!(e.mem.f32(GLOW_FLOAT_011AE5E8), 12.0);
        assert_eq!(e.mem.u32(GLOW_FLAG_012007EC), 1);
    }

    #[test]
    fn fn_00801da0_switches_the_parts_to_the_target_one() {
        let mut e = engine();
        let (this, actor) = target(&mut e);
        skinned(&mut e, actor);
        e.mem.set_u32(this + 0x240, 1);
        e.mem.set_u32(this + 0x244, 3);
        e.call_log = Some(vec![]);
        e.call(0x0080_1da0, &args![this]);
        let log = e.call_log.take().unwrap();
        let parts = calls_to(&log, SET_BODY_PART_ENABLED);
        assert_eq!(parts.len(), 32);
        assert_eq!(parts[0], vec![0x6800, 0, 1]);
        assert_eq!(parts[1], vec![0x6800, 0, 1]);
        assert_eq!(parts[2], vec![0x6800, 1, 0]);
        assert_eq!(parts[3], vec![0x6800, 1000, 0]);
        assert_eq!(parts[8..10], [vec![0x6800, 3, 1], vec![0x6800, 3000, 1]]);
        let hidden = calls_to(&log, SET_HIDDEN_00450F90);
        assert_eq!(hidden.len(), 15 + 12);
        assert_eq!(hidden[0], vec![0x9999, 0]);
        assert_eq!(hidden[1], vec![0x9999, 1]);
        assert_eq!(hidden[15..], vec![vec![0x5500, 1]; 12]);
        let slots: Vec<u32> = calls_to(&log, BIPED_ENTRY_0043F220)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(slots, [3, 4, 0xa, 0xb, 0xc, 0xd, 9, 0xe, 0xf, 0x10, 1, 0]);
        // The index 1 keeps the head entries; 0xE stops at once.
        e.mem.set_u32(this + 0x23c, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_1da0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, BIPED_ENTRY_0043F220).len(), 2);
        e.mem.set_u32(this + 0x23c, 0xe);
        e.call_log = Some(vec![]);
        e.call(0x0080_1da0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, CHILD_COUNT).is_empty());
    }

    #[test]
    fn fn_00802220_enables_the_part_and_its_thousand() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0080_2220, &args![0u32, 0x6800u32, 0x1_0003u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SET_BODY_PART_ENABLED),
            vec![vec![0x6800, 3, 1], vec![0x6800, 0x1_0003 * 1000, 1]]
        );
    }

    #[test]
    fn fn_00802260_shows_or_hides_the_biped_entry() {
        let mut e = engine();
        let (_, actor) = target(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0080_2260, &args![0u32, actor, 5u32, 1u32]);
        e.call(0x0080_2260, &args![0u32, actor, 6u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SET_HIDDEN_00450F90),
            vec![vec![0x5500, 0], vec![0x5500, 1]]
        );
        let empty = object(&mut e, &[(0x1e8, 0)]);
        e.call_log = Some(vec![]);
        e.call(0x0080_2260, &args![0u32, empty, 5u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, BIPED_ENTRY_0043F220).is_empty());
    }

    #[test]
    fn fn_008022c0_restores_every_part() {
        let mut e = engine();
        let (this, actor) = target(&mut e);
        skinned(&mut e, actor);
        e.call_log = Some(vec![]);
        e.call(0x0080_22c0, &args![this]);
        let log = e.call_log.take().unwrap();
        let parts = calls_to(&log, SET_BODY_PART_ENABLED);
        assert_eq!(parts.len(), 60);
        assert_eq!(
            parts[4..8],
            [
                vec![0x6800, 1, 1],
                vec![0x6800, 1000, 1],
                vec![0x6800, 101, 0],
                vec![0x6800, 201, 0],
            ]
        );
        let hidden = calls_to(&log, SET_HIDDEN_00450F90);
        assert_eq!(hidden[..12], vec![vec![0x5500, 0]; 12]);
        assert_eq!(hidden[12..], vec![vec![0x9999, 0]; 15]);
    }

    #[test]
    fn fn_008026b0_finishes_and_deletes_every_effect() {
        let mut e = engine();
        let this = e.mem.alloc(0x2d0);
        e.mem.set_u32(this + 0x12c, 0x5000);
        e.mem.set_u32(this + 0x16c + 14 * 4, 0x6000);
        e.mem.set_u8(this + 0x119 + 3, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_26b0, &args![this]);
        let log = e.call_log.take().unwrap();
        let queries = calls_to(&log, EFFECT_QUERY_00C4EDF0);
        assert_eq!(queries.len(), 2);
        assert_eq!((queries[0][0], queries[0][2]), (0x5000, 1));
        assert_eq!(queries[1][0], 0x6000);
        assert_eq!(
            calls_to(&log, EFFECT_DELETE),
            vec![vec![0x5000, 1], vec![0x6000, 1]]
        );
        assert_eq!(e.mem.u32(this + 0x12c), 0);
        assert_eq!(e.mem.u32(this + 0x16c + 14 * 4), 0);
        assert_eq!(e.mem.u8(this + 0x119 + 3), 0);
    }

    #[test]
    fn fn_008027d0_constructs_the_table() {
        let mut e = engine();
        let this = e.mem.alloc(0x1120);
        e.mem.set_u32(this + 0x18, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_27d0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), 1);
        assert_eq!(e.mem.u32(this + 4), VTABLE_01079170);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(
            calls_to(&log, BASE_CONSTRUCT_00802B80),
            vec![vec![this + 4, 0x25]]
        );
        assert_eq!(
            calls_to(&log, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![this + 0x20, 0x44, 0x40, 0x0080_2860]]
        );
    }

    #[test]
    fn fn_00802860_clears_the_entry_word() {
        let mut e = engine();
        let entry = e.mem.alloc(0x44);
        e.mem.set_u32(entry + 0x40, 3);
        assert_eq!(e.call(0x0080_2860, &args![entry]).u32(), entry);
        assert_eq!(e.mem.u32(entry + 0x40), 0);
    }

    #[test]
    fn fn_00802880_hands_the_entries_out_in_fives() {
        let mut e = engine();
        e.register(0x2fff_0000, |_, _| Ret::default());
        let this = e.mem.alloc(0x1120);
        e.mem.set_u32(this + 0x14, 7);
        e.call_log = Some(vec![]);
        e.call(0x0080_2880, &args![this, 0x2fff_0000u32]);
        let log = e.call_log.take().unwrap();
        let appended: Vec<u32> = calls_to(&log, APPEND_00406D50)
            .iter()
            .map(|w| w[2])
            .collect();
        let entry = |n: u32| this + 0x20 + 0x44 * n;
        let sep = SEPARATOR_01012630;
        assert_eq!(
            appended,
            [
                entry(0),
                sep,
                entry(1),
                sep,
                entry(2),
                sep,
                entry(3),
                sep,
                entry(4),
                sep,
                entry(5),
                sep,
                entry(6),
            ]
        );
        let callbacks = calls_to(&log, 0x2fff_0000);
        assert_eq!(callbacks.len(), 2);
        let buffer = calls_to(&log, APPEND_00406D50)[0][0];
        assert_eq!(callbacks[0], vec![buffer]);
        assert_eq!(calls_to(&log, MEMSET), vec![vec![buffer + 1, 0, 0x1ff]]);
    }

    #[test]
    fn fn_00802a20_and_fn_008029c0_find_an_entry_by_name() {
        let mut e = engine();
        let this = e.mem.alloc(0x1120);
        e.mem.set_u32(this + 0x14, 4);
        let wanted = this + 0x20 + 0x44 * 2;
        e.register_double(COMPARE_004564F0, move |_, a| {
            assert_eq!((a[0], a[2]), (0x1234, 0x40));
            u32::from(a[1] != wanted).into_ret()
        });
        assert_eq!(e.call(0x0080_2a20, &args![this, 0x1234u32]).i32(), 2);
        assert!(e.call(0x0080_29c0, &args![this, 0x1234u32, 9u32]).bool());
        assert_eq!((e.mem.u32(this + 0x18), e.mem.u32(this + 0x1c)), (2, 9));
        e.mem.set_u32(this + 0x14, 2);
        assert_eq!(e.call(0x0080_2a20, &args![this, 0x1234u32]).i32(), -1);
        assert!(!e.call(0x0080_29c0, &args![this, 0x1234u32, 5u32]).bool());
        assert_eq!(e.mem.u32(this + 0x1c), 9);
    }

    #[test]
    fn fn_00802a00_answers_zero() {
        let mut e = engine();
        assert_eq!(e.call(0x0080_2a00, &args![0u32, 1u32, 2u32]).u32(), 0);
    }

    #[test]
    fn fn_00802a90_creates_the_table_once() {
        let mut e = engine();
        e.map(TABLE_GLOBAL & !0xfff, 0x1000);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0080_2a90, &[]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x1120]]);
        let table = e.mem.u32(TABLE_GLOBAL);
        assert_eq!(e.mem.u32(table), 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_2a90, &[]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(TABLE_GLOBAL), table);
    }

    #[test]
    fn fn_00802b20_constructs_then_sets_the_vtable() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_2b20, &args![this, 7u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, BASE_CONSTRUCT_00802B80), vec![vec![this, 7]]);
        assert_eq!(e.mem.u32(this), VTABLE_01079170);
    }
}
