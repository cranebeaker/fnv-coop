//! `fallout shared/tesflora.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004013e0` (`thiscall`, answers a byte): true makes `InitItem` do
/// nothing; `00484ab0`: the form flag setter (one word).
const FORM_INIT_SKIP: u32 = 0x0040_13e0;
const FORM_SET_FLAG: u32 = 0x0048_4ab0;
/// `00510770` (`thiscall` on the form) and `0048b720` (`thiscall` on the
/// produce part at `this - 0xC`, one word: the form or null), the two calls
/// `InitItem` makes.
const FORM_00510770: u32 = 0x0051_0770;
const PRODUCE_INIT_ITEM: u32 = 0x0048_b720;
/// `TESForm::StartForm` (`004855a0`), `TESForm::CloseForm` (`00485680`),
/// `TESFullName::Save` (`00487010`), `TESModelTextureSwap::Save`
/// (`0048a520`, three chunk words), `TESScriptableForm::Save` (`0048cca0`),
/// `BGSDestructibleObjectForm::Save` (`00477f20`) and the produce part's
/// save (`0048b5c0`).
const FORM_START: u32 = 0x0048_55a0;
const FORM_CLOSE: u32 = 0x0048_5680;
const SAVE_FULL_NAME: u32 = 0x0048_7010;
const SAVE_MODEL_TEXTURE_SWAP: u32 = 0x0048_a520;
const SAVE_SCRIPTABLE: u32 = 0x0048_cca0;
const SAVE_DESTRUCTIBLE: u32 = 0x0047_7f20;
const SAVE_PRODUCE: u32 = 0x0048_b5c0;
/// `__RTDynamicCast` (`00ec43fb`) from `TESForm` (`01183028`) to `TESFlora`
/// (`01189f7c`), and `TESForm::CopyAllComponents` (`004851b0`) /
/// `TESForm::CompareAllComponents` (`00485270`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_FORM: u32 = 0x0118_3028;
const TYPE_FLORA: u32 = 0x0118_9f7c;
const FORM_COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const FORM_COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// The reading calls of the load: `TESForm::LoadForm` (`00485110`, one
/// word), `TESFile::GetTESChunk` (`004726b0`), the chunk's size
/// (`00401660`), `TESFile::GetChunkData` (`004727f0`, one word),
/// `TESFile::GetChunkData_ov3` (`00472890`, two words), the file's version
/// (`00403570`, answers `AX`) and the next chunk (`004726f0`, answers a
/// byte).
const FORM_LOAD_FORM: u32 = 0x0048_5110;
const FILE_GET_CHUNK: u32 = 0x0047_26b0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_GET_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const FILE_VERSION: u32 = 0x0040_3570;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
/// The component readers the load hands chunks to: the full name
/// (`00487050`, `cdecl (component, file)`), `TESModel::LoadModelChunk`
/// (`004892d0`, `cdecl`), `BGSDestructibleObjectForm::LoadChunk`
/// (`004781e0`, `cdecl`), `TESModelTextureSwap::UnpackTextureSwapChunkData`
/// (`0048a7b0`, `thiscall`, three words), the word setter `006ecd40`
/// (`thiscall`, one word), `TESScriptableForm::InitItem` (`0048cd90`,
/// `thiscall`, one word) and the produce part's `PFPC` reader (`0048b670`,
/// `thiscall`, one word: the 4 bytes read).
const LOAD_FULL_NAME: u32 = 0x0048_7050;
const LOAD_MODEL: u32 = 0x0048_92d0;
const LOAD_DESTRUCTIBLE: u32 = 0x0047_81e0;
const UNPACK_TEXTURE_SWAP: u32 = 0x0048_a7b0;
const SET_WORD: u32 = 0x006e_cd40;
const SCRIPTABLE_INIT_ITEM: u32 = 0x0048_cd90;
const PRODUCE_READ_CHANCES: u32 = 0x0048_b670;
/// The chunk codes the load compares (signed).
const CHUNK_PFIG: i32 = 0x4749_4650;
const CHUNK_OBND: i32 = 0x444e_424f;
const CHUNK_PFPC: i32 = 0x4350_4650;
const CHUNK_MODD: i32 = 0x4444_4f4d;
const CHUNK_EDID: i32 = 0x4449_4445;
const CHUNK_MOSD: i32 = 0x4453_4f4d;
const CHUNK_DSTD: i32 = 0x4454_5344;
const CHUNK_MODS: i32 = 0x5344_4f4d;
const CHUNK_SCRI: i32 = 0x4952_4353;
const CHUNK_MODL: i32 = 0x4c44_4f4d;
const CHUNK_FULL: i32 = 0x4c4c_5546;
const CHUNK_MODT: i32 = 0x5444_4f4d;
const CHUNK_DEST: i32 = 0x5453_4544;

/// `this + offset`, or null when `this` is 0xC (the exe's cast of the
/// produce part pointer `this - 0xC` back to a component of the flora).
fn component(this: u32, offset: u32) -> u32 {
    if this.wrapping_sub(0xc) == 0 {
        0
    } else {
        this.wrapping_add(offset)
    }
}

// Translated from 005085b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFlora::InitItem` (Xbox PDB): unless `004013e0` answers non-zero,
/// runs `00510770`, then the produce part's `0048b720` (on `this - 0xC`,
/// given `this`, or null when that part is null), and sets form flag 1
/// (`00484ab0`).
pub fn tes_flora_init_item(e: &mut Engine, this: Ptr) {
    if e.call(FORM_INIT_SKIP, &args![this]).u8() != 0 {
        return;
    }
    e.call(FORM_00510770, &args![this]);
    let form = component(this.addr(), 0);
    e.call(
        PRODUCE_INIT_ITEM,
        &args![this.addr().wrapping_sub(0xc), form],
    );
    e.call(FORM_SET_FLAG, &args![this, 1u32]);
}

// Translated from 00508610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00508610 - the /GS cookie check (00ec408c) is not translated; the EDID and MODS buffers from _RtlCheckStack12 (00ec5ec0, the _alloca) are with_stack blocks of the chunk size (their alignment, and the stack growing with each chunk, are not modelled)
/// Unnamed in the engine map (`thiscall`, one word: the file; the load of
/// a `TESFlora`): `TESForm::LoadForm`, form flag 0 cleared, then each chunk
/// (codes compared signed) until `GetTESChunk` answers 0 or the next chunk
/// fails: `EDID` (a stack buffer of the chunk's size, read with at most
/// 0x200 bytes, to virtual slot `+0x134`), `OBND` (virtual slot `+0xE0`
/// with the file), `FULL` (+0x30), `MODL`/`MODT`/`MODD`/`MOSD` (the model
/// at +0x3C), `MODS` (the chunk unpacked into the model's texture swaps
/// with the file's version), `SCRI` (the script id into +0x5C, then
/// `TESScriptableForm::InitItem`), `DEST`/`DSTD` (+0x68), `PFIG` (a word
/// into the produce part at `this - 0xC`) and `PFPC` (4 bytes to the
/// produce part's `0048b670`). Answers true.
pub fn fn_00508610(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.call(FORM_LOAD_FORM, &args![this, file]);
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    loop {
        let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        let code = chunk as i32;
        if code == CHUNK_EDID {
            let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
            e.with_stack(size, |e, name| {
                e.call(FILE_GET_CHUNK_DATA_SIZED, &args![file, name, 0x200u32]);
                e.vcall(base, 0x134, &args![name]);
            });
        } else if code == CHUNK_OBND {
            e.vcall(base, 0xe0, &args![file]);
        } else if code == CHUNK_FULL {
            let name = component(base, 0x30);
            e.call(LOAD_FULL_NAME, &args![name, file]);
        } else if code == CHUNK_MODL
            || code == CHUNK_MODT
            || code == CHUNK_MODD
            || code == CHUNK_MOSD
        {
            let model = component(base, 0x3c);
            e.call(LOAD_MODEL, &args![model, file]);
        } else if code == CHUNK_MODS {
            let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
            e.with_stack(size, |e, data| {
                e.call(FILE_GET_CHUNK_DATA_SIZED, &args![file, data, size]);
                let version = u32::from(e.call(FILE_VERSION, &args![file]).u16());
                e.call(
                    UNPACK_TEXTURE_SWAP,
                    &args![base.wrapping_add(0x3c), data, size, version],
                );
            });
        } else if code == CHUNK_SCRI {
            let id = e.with_stack(4, |e, word| {
                e.mem.set_u32(word.addr(), 0);
                e.call(FILE_GET_CHUNK_DATA, &args![file, word]);
                e.mem.u32(word.addr())
            });
            e.call(SET_WORD, &args![base.wrapping_add(0x5c), id]);
            let form = component(base, 0);
            e.call(SCRIPTABLE_INIT_ITEM, &args![base.wrapping_add(0x5c), form]);
        } else if code == CHUNK_DEST || code == CHUNK_DSTD {
            let destructible = component(base, 0x68);
            e.call(LOAD_DESTRUCTIBLE, &args![destructible, file]);
        } else if code == CHUNK_PFIG {
            let id = e.with_stack(4, |e, word| {
                e.mem.set_u32(word.addr(), 0);
                e.call(FILE_GET_CHUNK_DATA, &args![file, word]);
                e.mem.u32(word.addr())
            });
            e.call(SET_WORD, &args![base.wrapping_sub(0xc), id]);
        } else if code == CHUNK_PFPC {
            e.with_stack(4, |e, chances| {
                e.call(FILE_GET_CHUNK_DATA_SIZED, &args![file, chances, 4u32]);
                e.call(
                    PRODUCE_READ_CHANCES,
                    &args![base.wrapping_sub(0xc), chances],
                );
            });
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 00508920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`; the save of a `TESFlora`):
/// starts the form, saves the full name (+0x30), the model (+0x3C: `MODL`,
/// `MODT`, `MODS`), the script (+0x5C), the destructible data (+0x68) and
/// the produce part (`this - 0xC`), and closes the form.
pub fn fn_00508920(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(FORM_START, &args![this]);
    e.call(SAVE_FULL_NAME, &args![base + 0x30]);
    e.call(
        SAVE_MODEL_TEXTURE_SWAP,
        &args![base + 0x3c, 0x4c44_4f4du32, 0x5444_4f4du32, 0x5344_4f4du32],
    );
    e.call(SAVE_SCRIPTABLE, &args![base + 0x5c]);
    e.call(SAVE_DESTRUCTIBLE, &args![base + 0x68]);
    e.call(SAVE_PRODUCE, &args![base.wrapping_sub(0xc)]);
    e.call(FORM_CLOSE, &args![this]);
}

/// `other` cast from `TESForm` to `TESFlora`.
fn cast_flora(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![other, 0u32, TYPE_FORM, TYPE_FLORA, 0u32],
    )
    .u32()
}

// Translated from 00508990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; the copy of a
/// `TESFlora`): when `other` is one, `CopyAllComponents` from `other` (the
/// word as given, not the cast).
pub fn fn_00508990(e: &mut Engine, this: Ptr, other: Ptr) {
    if cast_flora(e, other) == 0 {
        return;
    }
    e.call(FORM_COPY_ALL_COMPONENTS, &args![this, other]);
}

// Translated from 005089d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; the comparison of a
/// `TESFlora`): true (different) when `other` is none, else
/// `CompareAllComponents` with the cast's form part (`cast + 0xC`).
pub fn fn_005089d0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let flora = cast_flora(e, other);
    if flora == 0 {
        return true;
    }
    let form = if flora != 0 { flora + 0xc } else { 0 };
    e.call(FORM_COMPARE_ALL_COMPONENTS, &args![this, form]).u8() != 0
}

/// `__RTDynamicCast` from `TESObjectREFR` (`011841cc`) to `Actor`
/// (`011846d4`), the type descriptors' names read at +8.
const TYPE_OBJECT_REFR: u32 = 0x0118_41cc;
const TYPE_ACTOR: u32 = 0x0118_46d4;
/// The player (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `00508d70` (`thiscall` on the reference, answers a byte: when set the
/// flora gives nothing), the ingredient getter (`00726070`, `thiscall` on
/// the produce part: the word at +4), `00867ca0` (`thiscall` on
/// `011de7b8`), the produce part's chance for that answer (`00508d90`,
/// `thiscall`, one word, answers a byte) and the random word `00487f50`.
const REFERENCE_00508D70: u32 = 0x0050_8d70;
const PRODUCE_INGREDIENT: u32 = 0x0072_6070;
const SEASON_OWNER: u32 = 0x011d_e7b8;
const SEASON_00867CA0: u32 = 0x0086_7ca0;
const PRODUCE_CHANCE: u32 = 0x0050_8d90;
const RANDOM: u32 = 0x0048_7f50;
/// `TESScriptableForm::GetFormScript` (`004826d0`, `cdecl (form)`), the
/// actor's `00574fa0` (`thiscall`, three words) and virtual slot `+0x190`
/// (three words), the allocator (`00401000`, `cdecl (size)`), the
/// `ExtraDataList` constructor (`00410360`), `ExtraDataList::SetCount`
/// (`00419ad0`, one word), `ExtraDataList::GetScript` (`00418800`),
/// `ExtraDataList::SetScript` (`00419ed0`, one word), `005abf60` (on the
/// script), `00419f80` (`thiscall` on the list, one word),
/// `ExtraDataList::GetScriptLocals` (`00418830`), the `TESObjectREFR`
/// constructor (`0055a2f0`) and `Script::Run` (`005ac1e0`, `thiscall`, four
/// words).
const GET_FORM_SCRIPT: u32 = 0x0048_26d0;
const ACTOR_00574FA0: u32 = 0x0057_4fa0;
const ALLOCATE: u32 = 0x0040_1000;
const EXTRA_DATA_LIST_CTOR: u32 = 0x0041_0360;
const LIST_SET_COUNT: u32 = 0x0041_9ad0;
const LIST_GET_SCRIPT: u32 = 0x0041_8800;
const LIST_SET_SCRIPT: u32 = 0x0041_9ed0;
const SCRIPT_005ABF60: u32 = 0x005a_bf60;
const LIST_00419F80: u32 = 0x0041_9f80;
const LIST_GET_SCRIPT_LOCALS: u32 = 0x0041_8830;
const OBJECT_REFR_CTOR: u32 = 0x0055_a2f0;
const SCRIPT_RUN: u32 = 0x005a_c1e0;
/// The messages: the `BSStringT` constructor (`004037b0`) and destructor
/// (`004037d0`), the format (`00403df0` on the setting at `011c856c`; on
/// `011c8590` the text shown when nothing is picked), the printer
/// `00406f60` (`cdecl (string, format, name)`), the string's text
/// (`00559450`), the message box `007052f0` (`cdecl`, six words, the fifth
/// the float at `010162c0`) and `Interface::PlayMenuSound` (`00706f30`,
/// `cdecl`, one word).
const STRING_CTOR: u32 = 0x0040_37b0;
const STRING_DTOR: u32 = 0x0040_37d0;
const SETTING_TEXT: u32 = 0x0040_3df0;
const SETTING_PICKED: u32 = 0x011c_856c;
const SETTING_NOTHING: u32 = 0x011c_8590;
const STRING_PRINT: u32 = 0x0040_6f60;
const STRING_TEXT: u32 = 0x0055_9450;
const SHOW_MESSAGE: u32 = 0x0070_52f0;
const MESSAGE_TIME: u32 = 0x0101_62c0;
const PLAY_MENU_SOUND: u32 = 0x0070_6f30;
/// The sound: the `BSSoundHandle` constructor (`0041a250`), the audio
/// object (`00453a70`), `BSAudio::GetSoundHandleByName` (`00ad7550`,
/// `thiscall (audio, out, name, flags)`), the handle's assignment
/// (`00418900`, one word), its destructor (`00483710`) and
/// `BSSoundHandle::Play` (`00ad8830`, one word); the name
/// `"ITMPickupOrganic"` (`01026be0`).
const SOUND_HANDLE_CTOR: u32 = 0x0041_a250;
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_GET_HANDLE: u32 = 0x00ad_7550;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const SOUND_HANDLE_DTOR: u32 = 0x0048_3710;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_PICKUP_ORGANIC: u32 = 0x0102_6be0;
/// `00484580` (`thiscall` on the reference, one word).
const REFERENCE_00484580: u32 = 0x0048_4580;

/// The message box call `Activate` makes twice: `text`, three zeros, the
/// float at `010162c0` and a zero.
fn show_message(e: &mut Engine, text: u32) {
    let time = x87_quiet_bits(e.mem.u32(MESSAGE_TIME));
    e.call(SHOW_MESSAGE, &args![text, 0u32, 0u32, 0u32, time, 0u32]);
}

// Translated from 00508a40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00508a40 - the C++ exception frame (FS:[0], states for the two allocations, the message string and the sound handles) is not translated
/// `TESFlora::Activate` (Xbox PDB, `thiscall`, five words; only the first
/// two are read): with a reference and an actor (`activator` cast to
/// `Actor`), picks when the reference allows it (`00508d70` 0), the flora
/// has an ingredient and the random word (`00487f50`) modulo 100 is below
/// the produce chance (`00508d90` with `00867ca0`'s answer). Picked: an
/// ingredient without a script goes to the actor's `00574fa0`; one with a
/// script gets a new `ExtraDataList` (count 1; with no script yet the
/// script, its `005abf60` answer, and a run of the script on a new
/// `TESObjectREFR` with the list's locals) and goes to the actor's slot
/// `+0x190`; for the player a message (the setting at `011c856c` printed
/// with the ingredient's name) and the `ITMPickupOrganic` sound. Not
/// picked, for the player: the message of `011c8590` and menu sound 0x1D.
/// Then `00484580(1)` on the reference; answers true.
pub fn tes_flora_activate(
    e: &mut Engine,
    this: Ptr,
    reference: u32,
    activator: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> bool {
    let actor = e
        .call(
            DYNAMIC_CAST,
            &args![activator, 0u32, TYPE_OBJECT_REFR, TYPE_ACTOR, 0u32],
        )
        .u32();
    if reference == 0 || actor == 0 {
        return true;
    }
    let produce = this.addr().wrapping_sub(0xc);
    let mut picked = false;
    if e.call(REFERENCE_00508D70, &args![reference]).u8() == 0
        && e.call(PRODUCE_INGREDIENT, &args![produce]).u32() != 0
    {
        let season = e.call(SEASON_00867CA0, &args![SEASON_OWNER]).u32();
        let chance = e.call(PRODUCE_CHANCE, &args![produce, season]).u8();
        if chance != 0 && e.call(RANDOM, &args![]).u32() % 100 < u32::from(chance) {
            picked = true;
        }
    }
    if picked {
        let ingredient = e.call(PRODUCE_INGREDIENT, &args![produce]).u32();
        let script = e.call(GET_FORM_SCRIPT, &args![ingredient]).u32();
        if script != 0 {
            let block = e.call(ALLOCATE, &args![0x20u32]).u32();
            let list = if block != 0 {
                e.call(EXTRA_DATA_LIST_CTOR, &args![block]).u32()
            } else {
                0
            };
            e.call(LIST_SET_COUNT, &args![list, 1u32]);
            if list != 0 && e.call(LIST_GET_SCRIPT, &args![list]).u32() == 0 {
                e.call(LIST_SET_SCRIPT, &args![list, script]);
                let attached = e.call(LIST_GET_SCRIPT, &args![list]).u32();
                let value = e.call(SCRIPT_005ABF60, &args![attached]).u32();
                e.call(LIST_00419F80, &args![list, value]);
                let block = e.call(ALLOCATE, &args![0x68u32]).u32();
                let runner = if block != 0 {
                    e.call(OBJECT_REFR_CTOR, &args![block]).u32()
                } else {
                    0
                };
                let locals = e.call(LIST_GET_SCRIPT_LOCALS, &args![list]).u32();
                e.call(SCRIPT_RUN, &args![script, runner, locals, 0u32, 0u32]);
            }
            e.vcall(actor, 0x190, &args![ingredient, list, 1u32]);
        } else {
            e.call(ACTOR_00574FA0, &args![actor, ingredient, 0u32, 1u32]);
        }
        if actor == e.mem.u32(PLAYER) {
            e.with_stack(0x30, |e, locals| {
                // +0: the message string; +0x10: the sound handle; +0x20:
                // the handle the audio answers into.
                let message = locals.addr();
                let handle = locals.addr() + 0x10;
                let found = locals.addr() + 0x20;
                e.call(STRING_CTOR, &args![message]);
                let name = e
                    .call(0x0040_8da0, &args![ingredient.wrapping_add(0x30)])
                    .u32();
                let format = e.call(SETTING_TEXT, &args![SETTING_PICKED]).u32();
                e.call(STRING_PRINT, &args![message, format, name]);
                let text = e.call(STRING_TEXT, &args![message]).u32();
                show_message(e, text);
                e.call(SOUND_HANDLE_CTOR, &args![handle]);
                let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                let answer = e
                    .call(
                        AUDIO_GET_HANDLE,
                        &args![audio, found, SOUND_PICKUP_ORGANIC, 0x120u32],
                    )
                    .u32();
                e.call(SOUND_HANDLE_ASSIGN, &args![handle, answer]);
                e.call(SOUND_HANDLE_DTOR, &args![found]);
                e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
                e.call(SOUND_HANDLE_DTOR, &args![handle]);
                e.call(STRING_DTOR, &args![message]);
            });
        }
    } else if actor == e.mem.u32(PLAYER) {
        let text = e.call(SETTING_TEXT, &args![SETTING_NOTHING]).u32();
        show_message(e, text);
        e.call(PLAY_MENU_SOUND, &args![0x1du32]);
    }
    e.call(REFERENCE_00484580, &args![reference, 1u32]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_85b0, tes_flora_init_item(Ptr)),
        entry!(0x0050_8610, fn_00508610(Ptr, Ptr) -> bool),
        entry!(0x0050_8920, fn_00508920(Ptr)),
        entry!(0x0050_8990, fn_00508990(Ptr, Ptr)),
        entry!(0x0050_89d0, fn_005089d0(Ptr, Ptr) -> bool),
        entry!(
            0x0050_8a40,
            tes_flora_activate(Ptr, u32, u32, u32, u32, u32) -> bool
        ),
    ]
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    const STATE: u32 = 0x7300_0000;
    const FLORA: u32 = 0x7600_0000;
    const FILE: u32 = 0x7700_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(STATE, 0x100);
        e.map(FLORA, 0x100);
        e
    }

    #[test]
    fn init_item_unless_skipped() {
        let mut e = engine();
        quiet(&mut e, &[FORM_00510770, PRODUCE_INIT_ITEM, FORM_SET_FLAG]);
        e.register(FORM_INIT_SKIP, |e, _| e.mem.u32(STATE).into_ret());
        let this = FLORA + 0xc;
        e.call_log = Some(vec![]);
        e.call(0x0050_85b0, &args![this]);
        assert_eq!(calls(&e, FORM_00510770), vec![vec![this]]);
        assert_eq!(calls(&e, PRODUCE_INIT_ITEM), vec![vec![FLORA, this]]);
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this, 1]]);
        // The form at 0xC: the produce part is null, so is the form given.
        e.call_log = Some(vec![]);
        e.call(0x0050_85b0, &args![0xcu32]);
        assert_eq!(calls(&e, PRODUCE_INIT_ITEM), vec![vec![0, 0]]);
        e.mem.set_u32(STATE, 1);
        e.call_log = Some(vec![]);
        e.call(0x0050_85b0, &args![this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    /// The chunks the file double hands out, one per `GetTESChunk`, then 0.
    const CHUNKS: [i32; 13] = [
        CHUNK_EDID, CHUNK_OBND, CHUNK_FULL, CHUNK_MODL, CHUNK_MODT, CHUNK_MODD, CHUNK_MOSD,
        CHUNK_MODS, CHUNK_SCRI, CHUNK_DEST, CHUNK_DSTD, CHUNK_PFIG, CHUNK_PFPC,
    ];
    const SLOT_EDID: u32 = 0x7100_0134;
    const SLOT_OBND: u32 = 0x7100_00e0;

    fn load_engine() -> Engine {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                FORM_LOAD_FORM,
                FORM_SET_FLAG,
                LOAD_FULL_NAME,
                LOAD_MODEL,
                LOAD_DESTRUCTIBLE,
                UNPACK_TEXTURE_SWAP,
                SET_WORD,
                SCRIPTABLE_INIT_ITEM,
                PRODUCE_READ_CHANCES,
                SLOT_EDID,
                SLOT_OBND,
                FILE_GET_CHUNK_DATA_SIZED,
            ],
        );
        e.register(FILE_GET_CHUNK, |e, _| {
            let n = e.mem.u32(STATE) as usize;
            let chunk = CHUNKS.get(n).map_or(0, |c| *c as u32);
            chunk.into_ret()
        });
        e.register(FILE_NEXT_CHUNK, |e, _| {
            let n = e.mem.u32(STATE);
            e.mem.set_u32(STATE, n + 1);
            true.into_ret()
        });
        e.register(FILE_CHUNK_SIZE, |_, _| 0x10u32.into_ret());
        e.register(FILE_VERSION, |_, _| 0xabcd_000fu32.into_ret());
        e.register(FILE_GET_CHUNK_DATA, |e, a| {
            e.mem.set_u32(a[1], 0x0100_0000 + e.mem.u32(STATE));
            Ret::default()
        });
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0x134, SLOT_EDID);
        e.mem.set_u32(vtable + 0xe0, SLOT_OBND);
        e.mem.set_u32(FLORA + 0xc, vtable);
        e
    }

    #[test]
    fn load_reads_every_chunk() {
        let mut e = load_engine();
        let this = FLORA + 0xc;
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_8610, &args![this, FILE]).bool());
        assert_eq!(calls(&e, FORM_LOAD_FORM), vec![vec![this, FILE]]);
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this, 0]]);
        assert_eq!(calls(&e, SLOT_EDID).len(), 1);
        assert_eq!(calls(&e, SLOT_OBND), vec![vec![this, FILE]]);
        assert_eq!(calls(&e, LOAD_FULL_NAME), vec![vec![this + 0x30, FILE]]);
        assert_eq!(calls(&e, LOAD_MODEL), vec![vec![this + 0x3c, FILE]; 4]);
        let unpack = calls(&e, UNPACK_TEXTURE_SWAP);
        assert_eq!(
            (unpack[0][0], unpack[0][2], unpack[0][3]),
            (this + 0x3c, 0x10, 0xf)
        );
        assert_eq!(
            calls(&e, SET_WORD),
            vec![vec![this + 0x5c, 0x0100_0008], vec![FLORA, 0x0100_000b]]
        );
        assert_eq!(
            calls(&e, SCRIPTABLE_INIT_ITEM),
            vec![vec![this + 0x5c, this]]
        );
        assert_eq!(
            calls(&e, LOAD_DESTRUCTIBLE),
            vec![vec![this + 0x68, FILE]; 2]
        );
        assert_eq!(calls(&e, PRODUCE_READ_CHANCES).len(), 1);
        assert_eq!(calls(&e, PRODUCE_READ_CHANCES)[0][0], FLORA);
        let sized = calls(&e, FILE_GET_CHUNK_DATA_SIZED);
        assert_eq!((sized[0][2], sized[1][2], sized[2][2]), (0x200, 0x10, 4));
        assert_eq!(calls(&e, FILE_GET_CHUNK).len(), 14);
    }

    #[test]
    fn load_with_the_form_at_0xc_gives_null_components() {
        let mut e = load_engine();
        e.mem.set_u32(STATE, 2);
        e.register(FILE_NEXT_CHUNK, |e, _| {
            let n = e.mem.u32(STATE);
            e.mem.set_u32(STATE, n + 1);
            (n < 3).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0050_8610, &args![0xcu32, FILE]);
        assert_eq!(calls(&e, LOAD_FULL_NAME), vec![vec![0, FILE]]);
        assert_eq!(calls(&e, LOAD_MODEL), vec![vec![0, FILE]]);
        // The next chunk failing ends the walk.
        assert_eq!(calls(&e, FILE_GET_CHUNK).len(), 2);
    }

    #[test]
    fn save_writes_the_components() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                FORM_START,
                FORM_CLOSE,
                SAVE_FULL_NAME,
                SAVE_MODEL_TEXTURE_SWAP,
                SAVE_SCRIPTABLE,
                SAVE_DESTRUCTIBLE,
                SAVE_PRODUCE,
            ],
        );
        let this = FLORA + 0xc;
        e.call_log = Some(vec![]);
        e.call(0x0050_8920, &args![this]);
        let order: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (FORM_START, vec![this]),
                (SAVE_FULL_NAME, vec![this + 0x30]),
                (
                    SAVE_MODEL_TEXTURE_SWAP,
                    vec![this + 0x3c, 0x4c44_4f4d, 0x5444_4f4d, 0x5344_4f4d]
                ),
                (SAVE_SCRIPTABLE, vec![this + 0x5c]),
                (SAVE_DESTRUCTIBLE, vec![this + 0x68]),
                (SAVE_PRODUCE, vec![FLORA]),
                (FORM_CLOSE, vec![this]),
            ]
        );
    }

    #[test]
    fn copy_and_compare() {
        let mut e = engine();
        quiet(&mut e, &[FORM_COPY_ALL_COMPONENTS]);
        e.register(DYNAMIC_CAST, |e, a| (a[0] & e.mem.u32(STATE)).into_ret());
        e.register(FORM_COMPARE_ALL_COMPONENTS, |e, _| {
            e.mem.u32(STATE + 4).into_ret()
        });
        e.mem.set_u32(STATE, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x0050_8990, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![0x2000, 0, TYPE_FORM, TYPE_FLORA, 0]]
        );
        assert_eq!(
            calls(&e, FORM_COPY_ALL_COMPONENTS),
            vec![vec![0x1000, 0x2000]]
        );
        assert!(!e.call(0x0050_89d0, &args![0x1000u32, 0x2000u32]).bool());
        assert_eq!(
            calls(&e, FORM_COMPARE_ALL_COMPONENTS),
            vec![vec![0x1000, 0x200c]]
        );
        e.mem.set_u32(STATE + 4, 1);
        assert!(e.call(0x0050_89d0, &args![0x1000u32, 0x2000u32]).bool());
        e.mem.set_u32(STATE, 0);
        e.mem.set_u32(STATE + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0050_8990, &args![0x1000u32, 0x2000u32]);
        assert!(e.call(0x0050_89d0, &args![0x1000u32, 0x2000u32]).bool());
        assert!(calls(&e, FORM_COPY_ALL_COMPONENTS).is_empty());
        assert!(calls(&e, FORM_COMPARE_ALL_COMPONENTS).is_empty());
    }
}

#[cfg(test)]
mod n043_activate_tests {
    use super::*;

    const STATE: u32 = 0x7300_0000;
    const FLORA: u32 = 0x7600_0000;
    const ACTOR: u32 = 0x7800_0000;
    const SLOT_190: u32 = 0x7100_0190;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// State words: +0 the reference's `00508d70`, +4 the ingredient, +8
    /// the chance, +0xC the random word, +0x10 the script, +0x14 the list's
    /// script.
    fn engine(chance: u32, random: u32, script: u32) -> Engine {
        let mut e = Engine::new();
        e.map(STATE, 0x40);
        e.map(FLORA, 0x100);
        e.map(0x011d_e000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.mem.set_u32(MESSAGE_TIME, 2.0f32.to_bits());
        e.mem.set_u32(STATE + 4, 0x5000);
        e.mem.set_u32(STATE + 8, chance);
        e.mem.set_u32(STATE + 0xc, random);
        e.mem.set_u32(STATE + 0x10, script);
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(REFERENCE_00508D70, |e, _| e.mem.u32(STATE).into_ret());
        e.register(PRODUCE_INGREDIENT, |e, _| e.mem.u32(STATE + 4).into_ret());
        e.register(SEASON_00867CA0, |_, _| 3u32.into_ret());
        e.register(PRODUCE_CHANCE, |e, _| e.mem.u32(STATE + 8).into_ret());
        e.register(RANDOM, |e, _| e.mem.u32(STATE + 0xc).into_ret());
        e.register(GET_FORM_SCRIPT, |e, _| e.mem.u32(STATE + 0x10).into_ret());
        e.register(ALLOCATE, |_, a| (0x6000_0000 + a[0]).into_ret());
        e.register(EXTRA_DATA_LIST_CTOR, |_, a| a[0].into_ret());
        e.register(OBJECT_REFR_CTOR, |_, a| a[0].into_ret());
        e.register(LIST_GET_SCRIPT, |e, _| e.mem.u32(STATE + 0x14).into_ret());
        e.register(LIST_SET_SCRIPT, |e, a| {
            e.mem.set_u32(STATE + 0x14, a[1]);
            Ret::default()
        });
        e.register(SCRIPT_005ABF60, |_, a| (a[0] + 1).into_ret());
        e.register(LIST_GET_SCRIPT_LOCALS, |_, _| 0x4444u32.into_ret());
        e.register(SETTING_TEXT, |_, a| (a[0] + 1).into_ret());
        e.register(0x0040_8da0, |_, a| (a[0] + 2).into_ret());
        e.register(STRING_TEXT, |_, _| 0x5555u32.into_ret());
        e.register(AUDIO_INSTANCE, |_, _| 0x6666u32.into_ret());
        e.register(AUDIO_GET_HANDLE, |_, a| a[1].into_ret());
        for addr in [
            ACTOR_00574FA0,
            LIST_SET_COUNT,
            LIST_00419F80,
            SCRIPT_RUN,
            SLOT_190,
            STRING_CTOR,
            STRING_DTOR,
            STRING_PRINT,
            SHOW_MESSAGE,
            PLAY_MENU_SOUND,
            SOUND_HANDLE_CTOR,
            SOUND_HANDLE_ASSIGN,
            SOUND_HANDLE_DTOR,
            SOUND_HANDLE_PLAY,
            REFERENCE_00484580,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.map(ACTOR, 0x10);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + 0x190, SLOT_190);
        e.mem.set_u32(ACTOR, vtable);
        e.mem.set_u32(PLAYER, ACTOR);
        e
    }

    fn activate(e: &mut Engine) -> bool {
        e.call_log = Some(vec![]);
        e.call(
            0x0050_8a40,
            &args![FLORA + 0xc, 0x9000u32, ACTOR, 0u32, 0u32, 0u32],
        )
        .bool()
    }

    #[test]
    fn picked_with_a_script_runs_it_and_tells_the_player() {
        let mut e = engine(50, 149, 0x7777);
        assert!(activate(&mut e));
        assert_eq!(calls(&e, PRODUCE_CHANCE), vec![vec![FLORA, 3]]);
        assert_eq!(calls(&e, SEASON_00867CA0), vec![vec![SEASON_OWNER]]);
        let list = 0x6000_0020;
        assert_eq!(calls(&e, LIST_SET_COUNT), vec![vec![list, 1]]);
        assert_eq!(calls(&e, LIST_SET_SCRIPT), vec![vec![list, 0x7777]]);
        assert_eq!(calls(&e, LIST_00419F80), vec![vec![list, 0x7778]]);
        assert_eq!(
            calls(&e, SCRIPT_RUN),
            vec![vec![0x7777, 0x6000_0068, 0x4444, 0, 0]]
        );
        assert_eq!(calls(&e, SLOT_190), vec![vec![ACTOR, 0x5000, list, 1]]);
        let printed = calls(&e, STRING_PRINT);
        assert_eq!((printed[0][1], printed[0][2]), (SETTING_PICKED + 1, 0x5032));
        assert_eq!(
            calls(&e, SHOW_MESSAGE),
            vec![vec![0x5555, 0, 0, 0, 2.0f32.to_bits(), 0]]
        );
        let handle = calls(&e, AUDIO_GET_HANDLE);
        assert_eq!(
            (handle[0][0], handle[0][2], handle[0][3]),
            (0x6666, SOUND_PICKUP_ORGANIC, 0x120)
        );
        assert_eq!(calls(&e, SOUND_HANDLE_PLAY)[0][1], 0);
        assert_eq!(calls(&e, SOUND_HANDLE_DTOR).len(), 2);
        assert_eq!(calls(&e, REFERENCE_00484580), vec![vec![0x9000, 1]]);
        assert!(calls(&e, PLAY_MENU_SOUND).is_empty());
    }

    #[test]
    fn picked_without_a_script_and_not_the_player() {
        let mut e = engine(50, 49, 0);
        e.mem.set_u32(PLAYER, 0);
        assert!(activate(&mut e));
        assert_eq!(calls(&e, ACTOR_00574FA0), vec![vec![ACTOR, 0x5000, 0, 1]]);
        assert!(calls(&e, ALLOCATE).is_empty());
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
        assert_eq!(calls(&e, REFERENCE_00484580).len(), 1);
    }

    #[test]
    fn a_list_that_has_a_script_is_not_run() {
        let mut e = engine(50, 0, 0x7777);
        e.mem.set_u32(STATE + 0x14, 0x1234);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert!(activate(&mut e));
        // The allocation failed: the count still goes to the null list.
        assert_eq!(calls(&e, LIST_SET_COUNT), vec![vec![0, 1]]);
        assert!(calls(&e, SCRIPT_RUN).is_empty());
        assert_eq!(calls(&e, SLOT_190), vec![vec![ACTOR, 0x5000, 0, 1]]);
    }

    #[test]
    fn not_picked_tells_the_player() {
        // The roll 50 is not below the chance 50.
        let mut e = engine(50, 150, 0);
        assert!(activate(&mut e));
        assert!(calls(&e, GET_FORM_SCRIPT).is_empty());
        assert_eq!(
            calls(&e, SHOW_MESSAGE),
            vec![vec![SETTING_NOTHING + 1, 0, 0, 0, 2.0f32.to_bits(), 0]]
        );
        assert_eq!(calls(&e, PLAY_MENU_SOUND), vec![vec![0x1d]]);
        // A zero chance rolls nothing.
        let mut e = engine(0, 0, 0);
        assert!(activate(&mut e));
        assert!(calls(&e, RANDOM).is_empty());
        assert_eq!(calls(&e, PLAY_MENU_SOUND).len(), 1);
        // The reference forbids it; no ingredient.
        let mut e = engine(50, 0, 0);
        e.mem.set_u32(STATE, 1);
        assert!(activate(&mut e));
        assert!(calls(&e, PRODUCE_INGREDIENT).is_empty());
        let mut e = engine(50, 0, 0);
        e.mem.set_u32(STATE + 4, 0);
        assert!(activate(&mut e));
        assert!(calls(&e, PRODUCE_CHANCE).is_empty());
        assert_eq!(calls(&e, PLAY_MENU_SOUND).len(), 1);
    }

    #[test]
    fn no_reference_or_actor_does_nothing() {
        let mut e = engine(50, 0, 0);
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(activate(&mut e));
        assert!(calls(&e, REFERENCE_00484580).is_empty());
        let mut e = engine(50, 0, 0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(
                0x0050_8a40,
                &args![FLORA + 0xc, 0u32, ACTOR, 0u32, 0u32, 0u32]
            )
            .bool());
        assert!(calls(&e, REFERENCE_00484580).is_empty());
    }
}
