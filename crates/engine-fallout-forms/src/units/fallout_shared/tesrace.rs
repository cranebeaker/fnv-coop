//! `fallout shared/tesrace.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESRace` has no layout here yet: its fields are read at their PC offsets
//! (the code's own), named in comments where the Xbox PDB confirms them.

#[allow(unused_imports)]
use crate::prelude::*;

/// The form id (the word at `form + 0x0c`), `__thiscall`.
const FORM_ID: u32 = 0x0084_e3a0;
/// A list node's data address, and the next node.
const NODE_DATA: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// Whether a list node ends the walk (an empty node).
const NODE_IS_END: u32 = 0x0082_56d0;
/// Adds the word at the pointer argument to a list (`__thiscall`, `RET 4`).
const LIST_ADD_005AE3D0: u32 = 0x005a_e3d0;
const LIST_ADD_00905820: u32 = 0x0090_5820;
/// Empties a list (`__thiscall`).
const LIST_CLEAR: u32 = 0x0047_0470;
/// The race's hair list and eye list (`__thiscall`, no stack words).
const HAIR_LIST: u32 = 0x0045_bb80;
const EYE_LIST: u32 = 0x0050_3650;
/// `TESActorBase::GetSex` (engine map).
const GET_SEX: u32 = 0x005f_0cc0;
/// `BGSSaveFormBuffer::GetForm` (engine map) on the global at `011dea3c`:
/// compared with the actor base, so it gives the player's base form.
const GET_BASE_FORM: u32 = 0x007a_f430;
const PLAYER: u32 = 0x011d_ea3c;
/// The face-generation switch string (`00408d60` gives its text).
const FACEGEN_SWITCH: u32 = 0x011c_b99c;
const STRING_TEXT: u32 = 0x0040_8d60;
/// `MapMarkerData::GetLocationName` (engine map; the body returns a
/// string's text, here of a texture or model path component).
const PATH_TEXT: u32 = 0x0040_8da0;
/// `strcpy_s`/`strcat_s`-style helpers (cdecl: buffer, size, text).
const STRING_COPY: u32 = 0x0040_6d30;
const STRING_APPEND: u32 = 0x0040_6d50;
/// `sprintf_s`-style formatter (cdecl: buffer, size, format, ...).
const FORMAT: u32 = 0x0040_6d00;
/// The error printer (`005b5e40`, cdecl, format and arguments).
const PRINT_ERROR: u32 = 0x005b_5e40;
/// The pointer held by a reference-counted holder (`00559450`).
const HELD: u32 = 0x0055_9450;
/// A reference-counted holder: constructed empty (`00633c90` with 0),
/// assigned (`0066b0d0`) and released (`0045cec0`).
const HOLDER_INIT: u32 = 0x0063_3c90;
const HOLDER_SET: u32 = 0x0066_b0d0;
const HOLDER_RELEASE: u32 = 0x0045_cec0;
/// `BSStringT` construction and destruction (8 bytes).
const STRING_INIT: u32 = 0x0040_37b0;
const STRING_FREE: u32 = 0x0040_37d0;
/// A face coordinate set for the race and sex (`005d9fb0`, `__thiscall`),
/// the default set (`BSFaceGenManager::GetFaceGenCoord`) and the copy
/// (`BSFaceGenManager::CopyFaceGenCoord`, cdecl: from, to, 0, 0).
const RACE_FACE_COORD: u32 = 0x005d_9fb0;
const DEFAULT_FACE_COORD: u32 = 0x0065_21e0;
const COPY_FACE_COORD: u32 = 0x0065_29a0;
/// The EGT path built from a model path (`006537d0`, cdecl: string, path).
const EGT_PATH: u32 = 0x0065_37d0;
const MESHES_PREFIX: u32 = 0x0101_6fac;
const TEXTURES_PREFIX: u32 = 0x0101_d7fc;

// Translated from 006130e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this + 0x80` (`ECX`, no stack words). The engine map gives it no name.
pub fn fn_006130e0(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x80)
}

// Translated from 00613790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this + 0x50` (`ECX`, no stack words). The engine map gives it no name.
pub fn fn_00613790(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x50)
}

// Translated from 00610c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `voice` as voice type `index` (`this + 0x4fc + 4 * index`); a
/// null `voice` stores the default for index 0 (`[011cb298]`) or index 1
/// (`[011cb29c]`) and nothing for other indices. `__thiscall`, `RET 8`.
/// The engine map gives it no name.
pub fn fn_00610c70(e: &mut Engine, this: Ptr, index: i32, voice: u32) {
    let at = |index: i32| {
        this.addr()
            .wrapping_add((index as u32).wrapping_mul(4))
            .wrapping_add(0x4fc)
    };
    if voice == 0 {
        if index == 0 {
            let default = e.global::<u32>(0x011c_b298);
            e.mem.set_u32(at(index), default);
        }
        if index == 1 {
            let default = e.global::<u32>(0x011c_b29c);
            e.mem.set_u32(at(index), default);
        }
    } else {
        e.mem.set_u32(at(index), voice);
    }
}

// Translated from 00613890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `hair` as default hair `index` (`this + 0x94 + 4 * index`).
/// `__thiscall`, `RET 8`. The engine map gives it no name.
pub fn fn_00613890(e: &mut Engine, this: Ptr, index: i32, hair: u32) {
    let at = this
        .addr()
        .wrapping_add((index as u32).wrapping_mul(4))
        .wrapping_add(0x94);
    e.mem.set_u32(at, hair);
}

// Translated from 00611e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` at `this + 0x9c + index`. `__thiscall`, `RET 8`.
/// The engine map gives it no name.
pub fn fn_00611e30(e: &mut Engine, this: Ptr, index: i32, value: u8) {
    let at = this.addr().wrapping_add(index as u32).wrapping_add(0x9c);
    e.mem.set_u8(at, value);
}

// Translated from 00613fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` at `this + 0xe1` (`__thiscall`, `RET 4`; the
/// callers pass a face node). The engine map gives it no name.
pub fn fn_00613fb0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0xe1, value);
}

// Translated from 00614630 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `0119b4e4`. No arguments. The engine map gives it no name.
pub fn fn_00614630(e: &mut Engine) -> u32 {
    e.global::<u32>(0x0119_b4e4)
}

// Translated from 006131a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this + 0x3c8 + 0x18 * sex` (`__thiscall`, `RET 4`): the race's body
/// model for the sex. The engine map gives it no name.
pub fn fn_006131a0(_e: &mut Engine, this: Ptr, sex: i32) -> Ptr {
    Ptr::new(
        this.addr()
            .wrapping_add((sex as u32).wrapping_mul(0x18))
            .wrapping_add(0x3c8),
    )
}

/// The element address the three part getters below compute: `this + base
/// + stride * sex + size * part`, or null when `part` is outside
/// `0..count` (signed).
fn part_address(
    this: Ptr,
    base: u32,
    stride: u32,
    size: u32,
    count: i32,
    sex: i32,
    part: i32,
) -> Ptr {
    if part < 0 || part >= count {
        return Ptr::NULL;
    }
    Ptr::new(
        this.addr()
            .wrapping_add((sex as u32).wrapping_mul(stride))
            .wrapping_add(base)
            .wrapping_add((part as u32).wrapping_mul(size)),
    )
}

// Translated from 00613a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this + 0x338 + 0x48 * sex + 0x18 * part`, or null when `part` is not
/// in 0..3. `__thiscall`, `RET 8`. The engine map gives it no name.
pub fn fn_00613a20(_e: &mut Engine, this: Ptr, sex: i32, part: i32) -> Ptr {
    part_address(this, 0x338, 0x48, 0x18, 3, sex, part)
}

// Translated from 00613970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The head part model `part` (0..8) for `sex`: `this + 0xb0 + 0xc0 * sex
/// + 0x18 * part`, or null outside 0..8. `__thiscall`, `RET 8`. The engine
/// map gives it no name.
pub fn fn_00613970(_e: &mut Engine, this: Ptr, sex: i32, part: i32) -> Ptr {
    part_address(this, 0xb0, 0xc0, 0x18, 8, sex, part)
}

// Translated from 00613b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The head part texture `part` (0..8) for `sex`: `this + 0x230 + 0x60 *
/// sex + 0xc * part`, or null outside 0..8. `__thiscall`, `RET 8`. The
/// engine map gives it no name.
pub fn fn_00613b20(_e: &mut Engine, this: Ptr, sex: i32, part: i32) -> Ptr {
    part_address(this, 0x230, 0x60, 0xc, 8, sex, part)
}

/// The first element of `list` whose form id is `id`, or 0 (the hair and
/// eye searches): walks while the node is non-null and holds a non-null
/// element.
fn find_by_form_id(e: &mut Engine, list: u32, id: u32) -> u32 {
    let mut node = list;
    while node != 0 {
        let data = e.call(NODE_DATA, &args![node]).u32();
        if e.mem.u32(data) == 0 {
            break;
        }
        let data = e.call(NODE_DATA, &args![node]).u32();
        let element = e.mem.u32(data);
        if e.call(FORM_ID, &args![element]).u32() == id {
            let data = e.call(NODE_DATA, &args![node]).u32();
            return e.mem.u32(data);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    0
}

// Translated from 006137b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The hair in the race's hair list (`0045bb80`) whose form id is `id`, or
/// 0. `__thiscall`, `RET 4`. The engine map gives it no name.
pub fn fn_006137b0(e: &mut Engine, this: Ptr, id: u32) -> u32 {
    let list = e.call(HAIR_LIST, &args![this]).u32();
    find_by_form_id(e, list, id)
}

// Translated from 006138b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The eyes in the race's eye list (`00503650`) whose form id is `id`, or
/// 0. `__thiscall`, `RET 4`. The engine map gives it no name.
pub fn fn_006138b0(e: &mut Engine, this: Ptr, id: u32) -> u32 {
    let list = e.call(EYE_LIST, &args![this]).u32();
    find_by_form_id(e, list, id)
}

/// Adds `form` to `list` (`00905820` with the address of the word).
fn add_form(e: &mut Engine, list: u32, form: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), form);
        e.call(LIST_ADD_00905820, &args![list, slot]);
    });
}

// Translated from 00613810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `hair` to the race's hair list (`0045bb80`, `00905820`) unless it
/// is null or a hair with its form id is already there (`fn_006137b0`).
/// `__thiscall`, `RET 4`. The engine map gives it no name.
pub fn fn_00613810(e: &mut Engine, this: Ptr, hair: u32) {
    if hair == 0 {
        return;
    }
    let id = e.call(FORM_ID, &args![hair]).u32();
    if fn_006137b0(e, this, id) != 0 {
        return;
    }
    let list = e.call(HAIR_LIST, &args![this]).u32();
    add_form(e, list, hair);
}

// Translated from 00613910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `eyes` to the race's eye list (`00503650`, `00905820`) unless it
/// is null or eyes with its form id are already there (`fn_006138b0`).
/// `__thiscall`, `RET 4`. The engine map gives it no name.
pub fn fn_00613910(e: &mut Engine, this: Ptr, eyes: u32) {
    if eyes == 0 {
        return;
    }
    let id = e.call(FORM_ID, &args![eyes]).u32();
    if fn_006138b0(e, this, id) != 0 {
        return;
    }
    let list = e.call(EYE_LIST, &args![this]).u32();
    add_form(e, list, eyes);
}

// Translated from 00613850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the race's hair list (`0045bb80`, `00470470`). `ECX`, no stack
/// words. The engine map gives it no name.
pub fn fn_00613850(e: &mut Engine, this: Ptr) {
    let list = e.call(HAIR_LIST, &args![this]).u32();
    e.call(LIST_CLEAR, &args![list]);
}

// Translated from 00613950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the race's eye list (`00503650`, `00470470`). `ECX`, no stack
/// words. The engine map gives it no name.
pub fn fn_00613950(e: &mut Engine, this: Ptr) {
    let list = e.call(EYE_LIST, &args![this]).u32();
    e.call(LIST_CLEAR, &args![list]);
}

/// The InitItem references: the field, the target `TypeDescriptor` and the
/// message printed when the form is missing.
const INIT_ITEM_REFERENCES: [(u32, u32, u32); 6] = [
    // Male and female voice types (`.?AVBGSVoiceType@@`).
    (0x4fc, 0x0118_6318, 0x0104_b818),
    (0x500, 0x0118_6318, 0x0104_b7c8),
    // Male and female default hair (`.?AVTESHair@@`).
    (0x94, 0x0118_63a0, 0x0104_b778),
    (0x98, 0x0118_63a0, 0x0104_b728),
    // Older and younger race (`.?AVTESRace@@`).
    (0x504, 0x0118_6370, 0x0104_b6e0),
    (0x508, 0x0118_6370, 0x0104_b698),
];

// Translated from 006128c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRace::InitItem` (engine map): unless `004013e0` answers true, turns
/// each form id the record left in the voice-type, default-hair and
/// older/younger-race fields into its form: the id gets the race file's
/// compile index (`TESForm::GetFile(-1)`, `TESForm::AddCompileIndex`), is
/// looked up (`004839c0`) and cast (`__RTDynamicCast` from `TESForm`);
/// a missing form prints its `MASTERFILE: Could not find ...` message with
/// the id, the race's name (virtual slot 0x130) and form id. Then the spell
/// list (`TESSpellList::InitItem` on +0x2c), the reaction form
/// (`TESReactionForm::InitItem` on +0x40) and `00484ab0(1)`. `ECX`, no
/// stack words.
pub fn tes_race_init_item(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    for (offset, target, message) in INIT_ITEM_REFERENCES {
        if e.mem.u32(base + offset) == 0 {
            continue;
        }
        let id = e.mem.u32(base + offset);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), id);
            let file = e.call(0x0048_4e60, &args![this, 0xffff_ffffu32]).u32();
            e.call(0x0048_5d50, &args![slot, file]);
            let id = e.mem.u32(slot.addr());
            let form = e.call(0x0048_39c0, &args![id]).u32();
            let cast = e
                .call(
                    0x00ec_43fb,
                    &args![form, 0u32, 0x0118_3028u32, target, 0u32],
                )
                .u32();
            e.mem.set_u32(base + offset, cast);
            if e.mem.u32(base + offset) == 0 {
                let form_id = e.call(FORM_ID, &args![this]).u32();
                let name = e.vcall(base, 0x130, &args![]).u32();
                e.call(PRINT_ERROR, &args![message, id, name, form_id]);
            }
        });
    }
    e.call(0x0048_d400, &args![base + 0x2c, this]);
    e.call(0x0048_bc30, &args![base + 0x40, this]);
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

// Translated from 006141f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006141f0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `TESRace::GetFaceGenData` (engine map): fills the face data at `data`
/// for `npc` (or, with no NPC, for the race and `sex`). `__thiscall`,
/// `RET 0x10`.
///
/// With an NPC: its face coordinates (`TESNPC::GetFaceCoord`) and sex; for
/// the player's base form the byte at `data + 0xd5` is set, and `006047a0`
/// on the player can mark the data as the player's own. Without one: the
/// race's coordinates for the sex (`005d9fb0`, else the default set) are
/// copied. Then +0x80 (the NPC's hair `00603b50` when `005fdfa0` accepts it
/// and the data is not the player's, else the race default `00613870`),
/// +0x88 (`00602130`, a float), +0x84, +0x8c, +0x90 for an NPC, or +0x80
/// and +0x84 (`00613100`, sign-extended) for the race. For each of the 8
/// head parts the model (`fn_00613970`), the texture (`fn_00613b20`) and
/// `00605d40(part)` are added to the lists at +0x94, +0xa4, +0xb4
/// (`0096a610`); with an NPC, the switch string set and `skip` clear, the
/// NPC's part holder (`006044e0`) is added to +0xc4. Then +0xd4 (the
/// switch string's first byte), +0xe0 (`fn_00614630`), the NPC's head
/// parts and their extra parts into +0xe4 (not for the player's own data),
/// +0xd8 and +0xdc (models 6 and 7), and with no eyes at +0x8c the race's
/// first eyes or the first loaded eyes whose texture is
/// `Characters\Eyes\EyeDefault.dds`. The exception frame is not
/// translated.
pub fn tes_race_get_face_gen_data(
    e: &mut Engine,
    this: Ptr,
    npc: u32,
    data: u32,
    sex: u32,
    skip: u8,
) {
    let race = this.addr();
    let mut players_own = false;
    let mut sex = sex;
    if npc != 0 {
        e.call(0x0060_3ad0, &args![npc, data]);
        sex = e.call(GET_SEX, &args![npc]).u32();
        let player = e.global::<u32>(PLAYER);
        if npc == e.call(GET_BASE_FORM, &args![player]).u32() {
            e.mem.set_u8(data + 0xd5, 1);
            let player = e.global::<u32>(PLAYER);
            if e.call(0x0060_47a0, &args![player]).bool() {
                players_own = true;
            }
        }
    } else {
        let mut coord = e.call(RACE_FACE_COORD, &args![this, sex]).u32();
        if coord == 0 {
            coord = e.call(DEFAULT_FACE_COORD, &args![]).u32();
        }
        e.call(COPY_FACE_COORD, &args![coord, data, 0u32, 0u32]);
    }
    if npc != 0 {
        let own_hair = !players_own && e.call(0x0060_3b50, &args![npc]).u32() != 0 && {
            let hair = e.call(0x0060_3b50, &args![npc]).u32();
            e.call(0x005f_dfa0, &args![hair, npc]).bool()
        };
        let hair = if own_hair {
            e.call(0x0060_3b50, &args![npc]).u32()
        } else {
            e.call(0x0061_3870, &args![this, sex]).u32()
        };
        e.mem.set_u32(data + 0x80, hair);
        let length = e.call(0x0060_2130, &args![npc]).f64();
        e.mem.set_u32(data + 0x88, x87_store32(length));
        let value = e.call(0x0041_69d0, &args![npc]).u32();
        e.mem.set_u32(data + 0x84, value);
        let value = e.call(0x0060_2150, &args![npc]).u32();
        e.mem.set_u32(data + 0x8c, value);
        let value = e.call(GET_SEX, &args![npc]).u32();
        e.mem.set_u32(data + 0x90, value);
    } else {
        let hair = e.call(0x0061_3870, &args![this, sex]).u32();
        e.mem.set_u32(data + 0x80, hair);
        let value = e.call(0x0061_3100, &args![this, sex]).u8() as i8;
        e.mem.set_i32(data + 0x84, i32::from(value));
    }
    for part in 0..8u32 {
        e.with_stack(4, |e, slot| {
            let model = fn_00613970(e, this, sex as i32, part as i32).addr();
            e.mem.set_u32(slot.addr(), model);
            e.call(0x0096_a610, &args![data + 0x94, slot]);
        });
        e.with_stack(4, |e, slot| {
            let texture = fn_00613b20(e, this, sex as i32, part as i32).addr();
            e.mem.set_u32(slot.addr(), texture);
            e.call(0x0096_a610, &args![data + 0xa4, slot]);
        });
        e.with_stack(4, |e, slot| {
            let value = e.call(0x0060_5d40, &args![part]).u32();
            e.mem.set_u32(slot.addr(), value);
            e.call(0x0096_a610, &args![data + 0xb4, slot]);
        });
        if npc != 0 {
            let text = e.call(STRING_TEXT, &args![FACEGEN_SWITCH]).u32();
            if e.mem.u8(text) != 0 && skip == 0 {
                e.with_stack(4, |e, holder| {
                    e.call(HOLDER_INIT, &args![holder, 0u32]);
                    e.call(0x0060_44e0, &args![npc, part, holder]);
                    e.call(0x0060_ba10, &args![data + 0xc4, holder]);
                    e.call(HOLDER_RELEASE, &args![holder]);
                });
            }
        }
    }
    let text = e.call(STRING_TEXT, &args![FACEGEN_SWITCH]).u32();
    let first = e.mem.u8(text);
    e.mem.set_u8(data + 0xd4, first);
    let value = fn_00614630(e);
    e.mem.set_u32(data + 0xe0, value);
    if npc != 0 && !players_own {
        let mut node = e.call(0x0060_2170, &args![npc]).u32();
        while node != 0 && !e.call(NODE_IS_END, &args![node]).bool() {
            let item_data = e.call(NODE_DATA, &args![node]).u32();
            let item = e.mem.u32(item_data);
            node = e.call(NODE_NEXT, &args![node]).u32();
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), item);
                e.call(LIST_ADD_005AE3D0, &args![data + 0xe4, slot]);
            });
            let mut extra = e.call(0x0062_4700, &args![item]).u32();
            while extra != 0 && !e.call(NODE_IS_END, &args![extra]).bool() {
                let extra_data = e.call(NODE_DATA, &args![extra]).u32();
                let part = e.mem.u32(extra_data);
                extra = e.call(NODE_NEXT, &args![extra]).u32();
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), part);
                    e.call(LIST_ADD_005AE3D0, &args![data + 0xe4, slot]);
                });
            }
        }
    }
    let model = fn_00613970(e, this, sex as i32, 6).addr();
    e.mem.set_u32(data + 0xd8, model);
    let model = fn_00613970(e, this, sex as i32, 7).addr();
    e.mem.set_u32(data + 0xdc, model);
    if e.mem.u32(data + 0x8c) == 0 {
        let list = e.call(EYE_LIST, &args![race]).u32();
        if list != 0 && !e.call(NODE_IS_END, &args![list]).bool() {
            let first = e.call(NODE_DATA, &args![list]).u32();
            let eyes = e.mem.u32(first);
            e.mem.set_u32(data + 0x8c, eyes);
        } else {
            let loaded = e.global::<u32>(0x011c_3f2c);
            let mut node = e.call(0x0043_c490, &args![loaded]).u32();
            while node != 0 {
                let node_data = e.call(NODE_DATA, &args![node]).u32();
                let eyes = e.mem.u32(node_data);
                if eyes != 0 {
                    let texture = e.call(PATH_TEXT, &args![eyes + 0x24]).u32();
                    if e.call(0x0040_4dc0, &args![texture, 0x0104_b8a0u32]).u32() == 0 {
                        e.mem.set_u32(data + 0x8c, eyes);
                        break;
                    }
                }
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
        }
    }
}

// Translated from 00613c50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00613c50 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// Builds the two face nodes for `npc` (or the race with `sex`) and answers
/// true. `__thiscall`, `RET 0x18`. The engine map gives it no name.
///
/// Under a memory scope (`00404eb0(0x37, 1, "...TESRace.cpp", 0x623)`,
/// ended by `00404ee0`) it clears both out pointers, makes a 0x1c0-byte
/// object (`00aa13e0`, `00649680`) and two 0xf0-byte face nodes
/// (`006628c0`) named `BSFaceGenNiNodeBiped` and `BSFaceGenNiNodeSkinned`
/// (`00a5b950` with a temporary fixed string), giving each the object
/// (virtual slot 0x104) and 1 or 0 to slots 0x114 and 0x11c. It then fills
/// a face data block (`005dd590`, `TESRace::GetFaceGenData`, `005dd6f0`),
/// builds the heads (`00654720` with `byte_arg`), applies the data to both
/// nodes when `apply` is set (`00655fa0`), and marks both nodes
/// (`fn_00613fb0(1)`) for the form with id 7. The exception frame is not
/// translated.
pub fn fn_00613c50(
    e: &mut Engine,
    this: Ptr,
    biped_out: u32,
    skinned_out: u32,
    npc: u32,
    apply: u8,
    byte_arg: u8,
    sex: u32,
) -> bool {
    e.with_stack(4, |e, scope| {
        e.call(
            0x0040_4eb0,
            &args![scope, 0x37u32, 1u32, 0x0104_b860u32, 0x623u32],
        );
        e.mem.set_u32(biped_out, 0);
        e.mem.set_u32(skinned_out, 0);
        let block = e.call(0x00aa_13e0, &args![0x1c0u32]).u32();
        let object = if block != 0 {
            e.call(0x0064_9680, &args![block]).u32()
        } else {
            0
        };
        for (out, name, flag) in [
            (biped_out, 0x0102_0408u32, 1u32),
            (skinned_out, 0x0102_03f0u32, 0u32),
        ] {
            let block = e.call(0x00aa_13e0, &args![0xf0u32]).u32();
            let node = if block != 0 {
                e.call(0x0066_28c0, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(out, node);
            e.with_stack(4, |e, fixed| {
                let text = e.call(0x0043_8170, &args![fixed, name]).u32();
                let node = e.mem.u32(out);
                e.call(0x00a5_b950, &args![node, text]);
                e.call(0x0043_81b0, &args![fixed]);
            });
            let node = e.mem.u32(out);
            e.vcall(node, 0x104, &args![object]);
            let node = e.mem.u32(out);
            e.vcall(node, 0x114, &args![flag]);
            let node = e.mem.u32(out);
            e.vcall(node, 0x11c, &args![flag]);
        }
        e.with_stack(0xf0, |e, face| {
            e.call(0x005d_d590, &args![face]);
            tes_race_get_face_gen_data(e, this, npc, face.addr(), sex, 0);
            e.call(
                0x0065_4720,
                &args![biped_out, skinned_out, face, u32::from(byte_arg)],
            );
            if apply != 0 {
                let node = e.mem.u32(biped_out);
                e.call(0x0065_5fa0, &args![node, face]);
                let node = e.mem.u32(skinned_out);
                e.call(0x0065_5fa0, &args![node, face]);
            }
            if npc != 0 && e.call(FORM_ID, &args![npc]).u32() == 7 {
                let node = e.mem.u32(biped_out);
                fn_00613fb0(e, Ptr::new(node), 1);
                let node = e.mem.u32(skinned_out);
                fn_00613fb0(e, Ptr::new(node), 1);
            }
            e.call(0x005d_d6f0, &args![face]);
        });
        e.call(0x0040_4ee0, &args![scope]);
    });
    true
}

/// Appends the race's head part model path to `path`: `Meshes\` and the
/// model's own path (virtual slot 0x14).
fn meshes_path(e: &mut Engine, path: Ptr, model: u32) {
    e.call(STRING_COPY, &args![path, 0x104u32, MESHES_PREFIX]);
    let model_path = e.vcall(model, 0x14, &args![]).u32();
    e.call(STRING_APPEND, &args![path, 0x104u32, model_path]);
}

/// Drops the EGT data of `model` from the face model cache: the EGT path
/// from the mesh path (`006537d0`) is removed through
/// `BSFaceGenManager::GetModelCache` and `00650780`.
fn kill_model_egt(e: &mut Engine, egt: Ptr, path: Ptr, model: u32) {
    meshes_path(e, path, model);
    e.call(EGT_PATH, &args![egt, path]);
    let text = e.call(HELD, &args![egt]).u32();
    let cache = e.call(0x0065_2110, &args![]).u32();
    e.call(0x0065_0780, &args![cache, text]);
}

// Translated from 00613fd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00613fd0 - the exception-unwinding frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated.
/// `TESRace::KillEGTData` (engine map): for each of the 8 head parts,
/// drops the EGT data of the male model (`fn_00613970(0, part)`) when it
/// has a path, and of the female model (`fn_00613970(1, part)`) when it
/// has a path and its virtual slot 0xc answers true with the male model.
/// `ECX`, no stack words. The exception frame and the stack cookie check
/// are not translated.
pub fn tes_race_kill_egt_data(e: &mut Engine, this: Ptr) {
    e.with_stack(8, |e, egt| {
        e.call(STRING_INIT, &args![egt]);
        e.with_stack(0x108, |e, path| {
            for part in 0..8 {
                let male = fn_00613970(e, this, 0, part).addr();
                if male != 0 {
                    let model_path = e.vcall(male, 0x14, &args![]).u32();
                    if e.mem.i8(model_path) != 0 {
                        kill_model_egt(e, egt, path, male);
                    }
                }
                let female = fn_00613970(e, this, 1, part).addr();
                if female != 0 {
                    let model_path = e.vcall(female, 0x14, &args![]).u32();
                    if e.mem.i8(model_path) != 0 && e.vcall(female, 0xc, &args![male]).bool() {
                        kill_model_egt(e, egt, path, female);
                    }
                }
            }
        });
        e.call(STRING_FREE, &args![egt]);
    });
}

// Translated from 00614640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the texture path of body part `part` for `npc` into `path`
/// (`Textures\` and the texture's path) and answers true; false when the
/// part index (`00615300`) is not 0..3 or the race
/// has no texture for it. The texture is `this + 0x2f0 + 0x24 * sex + 0xc
/// * index` (sex 0 with no NPC); an empty index 2 falls back to 1. For the
/// player's base form with index 1 or 2 the extension is replaced by
/// `_1st` and the texture's own extension. `__thiscall`, `RET 0xc`. The
/// engine map gives it no name.
pub fn fn_00614640(e: &mut Engine, this: Ptr, npc: u32, part: u32, path: Ptr) -> bool {
    let mut index = e.call(0x0061_5300, &args![part]).i32();
    if !(0..3).contains(&index) {
        return false;
    }
    let sex = if npc != 0 {
        e.call(GET_SEX, &args![npc]).u32()
    } else {
        0
    };
    let texture = |index: i32| {
        this.addr()
            .wrapping_add(sex.wrapping_mul(0x24))
            .wrapping_add(0x2f0)
            .wrapping_add((index as u32).wrapping_mul(0xc))
    };
    if e.call(0x0048_cee0, &args![texture(index)]).u32() == 0 && index == 2 {
        index = 1;
    }
    if e.call(0x0048_cee0, &args![texture(index)]).u32() == 0 {
        return false;
    }
    e.call(STRING_COPY, &args![path, 0x104u32, TEXTURES_PREFIX]);
    let text = e.call(PATH_TEXT, &args![texture(index)]).u32();
    e.call(STRING_APPEND, &args![path, 0x104u32, text]);
    if npc != 0 {
        let player = e.global::<u32>(PLAYER);
        if npc == e.call(GET_BASE_FORM, &args![player]).u32() && (index == 1 || index == 2) {
            let text = e.call(PATH_TEXT, &args![texture(index)]).u32();
            let extension = e.call(0x00ec_6e30, &args![text, 0x2eu32]).u32();
            let dot = e.call(0x0040_ab30, &args![path, 0x2eu32]).u32();
            if dot != 0 && extension != 0 {
                e.mem.set_u8(dot, 0);
                e.call(STRING_APPEND, &args![path, 0x104u32, 0x0104_b8c0u32]);
                e.call(STRING_APPEND, &args![path, 0x104u32, extension]);
            }
        }
    }
    true
}

/// The last template of `npc` (`006044c0` followed while it answers a
/// form), or `npc` itself.
fn last_template(e: &mut Engine, npc: u32) -> u32 {
    let mut form = npc;
    if e.call(0x0060_44c0, &args![npc]).u32() != 0 {
        form = e.call(0x0060_44c0, &args![npc]).u32();
        while form != 0 && e.call(0x0060_44c0, &args![form]).u32() != 0 {
            form = e.call(0x0060_44c0, &args![form]).u32();
        }
    }
    form
}

// Translated from 006147e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006147e0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESRace::GetBodyModTextureName` (engine map): formats the body-mod
/// texture name of `npc` into `name` (0x104 bytes): `"%08XModBody%s"` with
/// the low 24 bits of the form id of the NPC's last template and `Male` or
/// `Female` (`TESActorBase::GetSex` 1), or `"%08x_%08XModBody%s"` with the
/// race's id first when virtual slot 0x2c of `npc + 0x30` answers true.
/// `__thiscall`, `RET 8`. The stack cookie check is not translated.
pub fn tes_race_get_body_mod_texture_name(e: &mut Engine, this: Ptr, npc: u32, name: Ptr) {
    let template = last_template(e, npc);
    e.with_stack(0x10, |e, sex_name| {
        let text = if e.call(GET_SEX, &args![npc]).u32() == 1 {
            0x0104_a0ecu32
        } else {
            0x0104_a0f4
        };
        e.call(0x00ec_623a, &args![sex_name, text]);
        let race_id = e.call(FORM_ID, &args![this]).u32() & 0x00ff_ffff;
        let template_id = e.call(FORM_ID, &args![template]).u32() & 0x00ff_ffff;
        if e.vcall(npc + 0x30, 0x2c, &args![]).bool() {
            e.call(
                FORMAT,
                &args![
                    name,
                    0x104u32,
                    0x0104_b8d8u32,
                    race_id,
                    template_id,
                    sex_name
                ],
            );
        } else {
            e.call(
                FORMAT,
                &args![name, 0x104u32, 0x0104_b8c8u32, template_id, sex_name],
            );
        }
    });
}

// Translated from 006148f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRace::GetBodyModTextureFileName` (engine map): formats
/// `data\Textures\Characters\BodyMods\<file>\<name>.dds` into `path` and
/// answers true, where `<file>` is the name of the file that owns the
/// NPC's last template (`TESForm::GetOwnerMaster`, `00891170`), or
/// `FalloutNV.ESM` when that name starts with `update` (`004564f0`, 6
/// characters). False when the template has no owner file. `__thiscall`,
/// `RET 0xc` (`this` is not read).
pub fn tes_race_get_body_mod_texture_file_name(
    e: &mut Engine,
    _this: Ptr,
    npc: u32,
    name: Ptr,
    path: Ptr,
) -> bool {
    let template = last_template(e, npc);
    let master = e.call(0x0048_4ee0, &args![template]).u32();
    if master == 0 {
        return false;
    }
    let file_name = e.call(0x0089_1170, &args![master]).u32();
    let is_update = e
        .call(0x0045_64f0, &args![file_name, 0x0104_a714u32, 6u32])
        .u32()
        == 0;
    let directory = if is_update {
        0x0104_a704
    } else {
        e.call(0x0089_1170, &args![master]).u32()
    };
    e.call(
        FORMAT,
        &args![path, 0x104u32, 0x0104_b8ecu32, directory, name],
    );
    true
}

/// The cleanup every exit of `TESRace::GetBodyTexture` runs, in the exe's
/// order: the string, the coordinate array (`eh vector destructor
/// iterator` with `00449680`), then the four holders last first.
fn body_texture_cleanup(e: &mut Engine, string: Ptr, coords: Ptr, holders: [Ptr; 4]) {
    e.call(STRING_FREE, &args![string]);
    e.call(0x00ec_5fce, &args![coords, 0x20u32, 4u32, 0x0044_9680u32]);
    for holder in holders.into_iter().rev() {
        e.call(HOLDER_RELEASE, &args![holder]);
    }
}

// Translated from 006149b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006149b0 - the exception-unwinding frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated.
/// `TESRace::GetBodyTexture` (engine map): loads the body texture of body
/// part `part` for `npc` into the holder at `texture_out` and its body-mod
/// texture into `mod_out`. `__thiscall`, `RET 0x14`; `from_scratch` is the
/// last word (a byte).
///
/// It clears `texture_out`, sets `mod_out` to the default modulation
/// texture (`006045f0`) unless `from_scratch` is set or `npc` is the
/// player's base form, and answers false when there is no texture path
/// (`fn_00614640`) or the texture does not load (`TES::CreateTextureImage`
/// on `[011dea10]`). With the face-generation switch string empty it
/// answers true. Otherwise it loads the body-mod texture file
/// (`TESRace::GetBodyModTextureName`, `...FileName`) unless `from_scratch`;
/// when that failed (`00822510`) or `from_scratch` is set, it builds the
/// texture: prints the "Failed to find body mod texture" message (not for
/// the player or `from_scratch`), takes the body model (`fn_006131a0`; none:
/// false), answers true when the texture palette already has the texture
/// (`BSTexturePalette::GetTexture_ov2`), loads the model's FaceGen mesh
/// (`006537d0`, `00651b50`; none: false), gets the face coordinates (the
/// NPC's `00601800` blended by `00652af0`, or the race's copied), removes a
/// stale palette entry when `from_scratch`, applies the coordinates
/// (`BSFaceGenModel::ApplyCoordinateTexturingToMesh` with `006151d0`;
/// nothing made: false), names the texture after the file
/// (`NiSourceTexture::SetFilename`) and puts it in the palette
/// (`BSTexturePalette::SetTexture`). The exception frame and the stack
/// cookie check are not translated.
pub fn tes_race_get_body_texture(
    e: &mut Engine,
    this: Ptr,
    texture_out: u32,
    mod_out: u32,
    npc: u32,
    part: u32,
    from_scratch: u8,
) -> bool {
    let race = this.addr();
    // Locals, as the exe lays them out: four holders, the four face
    // coordinate sets (0x20 bytes each), a string, three paths and a name.
    let frame = e.mem.alloc(0x4f0);
    let holder_a = Ptr::new(frame + 0x130);
    let holder_b = Ptr::new(frame + 0x128);
    let holder_c = Ptr::new(frame + 0x12c);
    let holder_d = Ptr::new(frame + 0x348);
    let coords = Ptr::new(frame + 0x358);
    let string = Ptr::new(frame + 0x350);
    let texture_path: Ptr = Ptr::new(frame + 0x138);
    let mod_path: Ptr = Ptr::new(frame + 0x240);
    let mod_name: Ptr = Ptr::new(frame + 0x3d8);
    let mesh_path: Ptr = Ptr::new(frame + 0x20);
    let fixed: Ptr = Ptr::new(frame + 0x10);
    let holders = [holder_a, holder_b, holder_c, holder_d];
    for holder in holders {
        e.call(HOLDER_INIT, &args![holder, 0u32]);
    }
    e.call(
        0x00ec_782f,
        &args![coords, 0x20u32, 4u32, 0x0044_9610u32, 0x0044_9680u32],
    );
    let sex = if npc != 0 {
        e.call(GET_SEX, &args![npc]).u32()
    } else {
        0
    };
    e.call(STRING_INIT, &args![string]);
    e.call(HOLDER_SET, &args![texture_out, 0u32]);
    if from_scratch == 0 {
        let player = e.global::<u32>(PLAYER);
        if npc != e.call(GET_BASE_FORM, &args![player]).u32() {
            let default = e.call(0x0060_45f0, &args![]).u32();
            e.call(HOLDER_SET, &args![mod_out, default]);
        }
    }
    let result = 'body: {
        if !fn_00614640(e, this, npc, part, texture_path) {
            break 'body false;
        }
        let tes = e.global::<u32>(0x011d_ea10);
        e.call(
            0x0045_68c0,
            &args![tes, texture_path, texture_out, 0u32, 0u32],
        );
        if e.call(HELD, &args![texture_out]).u32() == 0 {
            break 'body false;
        }
        let text = e.call(STRING_TEXT, &args![FACEGEN_SWITCH]).u32();
        if e.mem.u8(text) == 0 {
            break 'body true;
        }
        tes_race_get_body_mod_texture_name(e, this, npc, mod_name);
        if tes_race_get_body_mod_texture_file_name(e, this, npc, mod_name, mod_path)
            && from_scratch == 0
        {
            let tes = e.global::<u32>(0x011d_ea10);
            e.call(0x0045_68c0, &args![tes, mod_path, mod_out, 1u32, 0u32]);
        }
        if !e.call(0x0082_2510, &args![mod_out, 0u32]).bool() && from_scratch == 0 {
            break 'body true;
        }
        if from_scratch == 0 {
            let player = e.global::<u32>(PLAYER);
            if npc != e.call(GET_BASE_FORM, &args![player]).u32() {
                let form_id = e.call(FORM_ID, &args![this]).u32();
                let race_name = e.vcall(race, 0x130, &args![]).u32();
                e.call(
                    PRINT_ERROR,
                    &args![0x0104_b918u32, mod_name, race_name, form_id],
                );
            }
        }
        let npc_sex = e.call(GET_SEX, &args![npc]).i32();
        let model = fn_006131a0(e, this, npc_sex).addr();
        if model == 0 {
            break 'body false;
        }
        if from_scratch == 0 {
            let player = e.global::<u32>(PLAYER);
            if npc != e.call(GET_BASE_FORM, &args![player]).u32() {
                e.call(0x00a6_1b90, &args![mod_name, mod_out]);
                if e.call(HELD, &args![mod_out]).u32() != 0 {
                    break 'body true;
                }
            }
        }
        meshes_path(e, mesh_path, model);
        let egt = e.call(EGT_PATH, &args![string, mesh_path]).u32();
        let mesh = e
            .call(
                0x0065_1b50,
                &args![0u32, 0u32, 0u32, egt, 1u32, 0xffff_ffffu32, 0u32],
            )
            .u32();
        e.call(HOLDER_SET, &args![holder_a, mesh]);
        if e.call(HELD, &args![holder_a]).u32() == 0 {
            break 'body false;
        }
        if npc != 0 {
            let blended = e.call(0x0060_1800, &args![npc]).u32();
            let npc_sex = e.call(GET_SEX, &args![npc]).u32();
            let coord = e.call(RACE_FACE_COORD, &args![this, npc_sex]).u32();
            e.call(
                0x0065_2af0,
                &args![coord, blended, coords, 0u32, 0.0f32.to_bits()],
            );
        } else {
            let mut coord = e.call(RACE_FACE_COORD, &args![this, sex]).u32();
            if coord == 0 {
                coord = e.call(DEFAULT_FACE_COORD, &args![]).u32();
            }
            e.call(COPY_FACE_COORD, &args![coord, coords, 0u32, 0u32]);
        }
        if from_scratch != 0 {
            e.call(0x00a6_1fb0, &args![mod_path]);
        }
        let palette_flag = e.call(0x0061_51d0, &args![]).u32();
        let mesh = e.call(HELD, &args![holder_a]).u32();
        e.call(0x0065_b410, &args![mesh, coords, mod_out, palette_flag]);
        if e.call(HELD, &args![mod_out]).u32() == 0 {
            break 'body false;
        }
        let file_name = e.call(0x0043_8170, &args![fixed, mod_path]).u32();
        let texture = e.call(HELD, &args![mod_out]).u32();
        e.call(0x00a5_fba0, &args![texture, file_name]);
        e.call(0x0043_81b0, &args![fixed]);
        let texture = e.call(HELD, &args![mod_out]).u32();
        e.call(0x00a6_1c50, &args![texture, 0u32]);
        true
    };
    body_texture_cleanup(e, string, coords, holders);
    e.mem.free(frame);
    result
}

const GET_TES_FORM: u32 = 0x0047_2660;
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The current chunk's size (the word at `file + 0x25c`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// The byte at `file + 0x299` (non-zero: the data needs byte swapping).
const FILE_SWAPS: u32 = 0x0040_1680;
/// `TESFile::GetChunkData_ov3` (destination, size), `TESFile::GetChunkData`
/// (a word) and `TESFile::GetChunkData_ov2` (engine map), `__thiscall`.
const GET_CHUNK_DATA: u32 = 0x0047_2890;
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
const GET_CHUNK_DATA_OV2: u32 = 0x0047_2840;
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// Swaps the bytes of the word at the pointer (cdecl: pointer, 0).
const SWAP_WORD: u32 = 0x0040_1080;
/// `TESForm::LoadData` (engine map): `__thiscall`, file, destination, size.
const TES_FORM_LOAD_DATA: u32 = 0x0048_61f0;
/// `TESForm::AddCompileIndex` (engine map), cdecl: id pointer, file.
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// The form with an id (`004839c0`, cdecl) and `__RTDynamicCast`.
const LOOKUP_FORM: u32 = 0x0048_39c0;
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TES_FORM_TYPE: u32 = 0x0118_3028;
/// The game's allocator (`00401000`, cdecl size) and its free (`00401030`).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
const MEMSET: u32 = 0x0040_3d30;
/// `TESTexture::IsTextureChunk`, `TESModel::IsModelChunk`,
/// `TESTexture::LoadTextureChunk`, `TESModel::LoadModelChunk` (engine map),
/// all cdecl.
const IS_TEXTURE_CHUNK: u32 = 0x0048_e3b0;
const IS_MODEL_CHUNK: u32 = 0x0048_9280;
const LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// The face-geometry float vectors: `0060b340` resizes one (count, 1),
/// `0060b370` gives an iterator at an index into a temporary and
/// `00629ab0` the element it points at.
const VECTOR_RESIZE: u32 = 0x0060_b340;
const VECTOR_AT: u32 = 0x0060_b370;
const ITERATOR_ELEMENT: u32 = 0x0062_9ab0;
/// Default model paths (pointer tables in `.data`): head parts by sex
/// (8 each), head part textures (8 each), body parts by sex (3 each) and
/// the body models by sex.
const HEAD_MODEL_PATHS: u32 = 0x0119_9fe8;
const HEAD_TEXTURE_PATHS: u32 = 0x0119_a028;
const BODY_PART_PATHS: u32 = 0x0119_a080;
const BODY_MODEL_PATHS: u32 = 0x0119_9fbc;
/// The form type of a race record.
const FORM_TYPE_RACE: u32 = 0xc;

const CHUNK_CNAM: u32 = 0x4d41_4e43;
const CHUNK_DESC: u32 = 0x4353_4544;
const CHUNK_NAM2: u32 = 0x324d_414e;
const CHUNK_NAM0: u32 = 0x304d_414e;
const CHUNK_NAM1: u32 = 0x314d_414e;
const CHUNK_FGGA: u32 = 0x4147_4746;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_VTCK: u32 = 0x4b43_5456;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_SPLO: u32 = 0x4f4c_5053;
const CHUNK_DNAM: u32 = 0x4d41_4e44;
const CHUNK_ENAM: u32 = 0x4d41_4e45;
const CHUNK_FNAM: u32 = 0x4d41_4e46;
const CHUNK_HNAM: u32 = 0x4d41_4e48;
const CHUNK_MNAM: u32 = 0x4d41_4e4d;
const CHUNK_ONAM: u32 = 0x4d41_4e4f;
const CHUNK_PNAM: u32 = 0x4d41_4e50;
const CHUNK_SNAM: u32 = 0x4d41_4e53;
const CHUNK_UNAM: u32 = 0x4d41_4e55;
const CHUNK_XNAM: u32 = 0x4d41_4e58;
const CHUNK_YNAM: u32 = 0x4d41_4e59;
const CHUNK_FGTS: u32 = 0x5354_4746;
const CHUNK_ATTR: u32 = 0x5254_5441;
const CHUNK_FGGS: u32 = 0x5347_4746;
const CHUNK_INDX: u32 = 0x5844_4e49;

/// `base + stride * a + size * b`, as the exe's `IMUL`/`LEA` arithmetic.
fn element(base: u32, stride: u32, a: u32, size: u32, b: i32) -> u32 {
    base.wrapping_add(stride.wrapping_mul(a))
        .wrapping_add(size.wrapping_mul(b as u32))
}

/// Reads two words (8 bytes) of the current chunk, byte-swapping them when
/// the file asks for it.
fn read_two_words(e: &mut Engine, file: Ptr, pair: Ptr) -> (u32, u32) {
    e.call(GET_CHUNK_DATA, &args![file, pair, 8u32]);
    if e.call(FILE_SWAPS, &args![file]).bool() {
        for i in 0..2 {
            e.call(SWAP_WORD, &args![pair.addr() + 4 * i, 0u32]);
        }
    }
    (e.mem.u32(pair.addr()), e.mem.u32(pair.addr() + 4))
}

/// The `HNAM` and `ENAM` lists: an array of form ids (the chunk size must
/// be a non-zero multiple of 4), each given the file's compile index,
/// looked up and cast to `target`; found forms are added (`add`), missing
/// ones print `message`.
fn load_form_list(
    e: &mut Engine,
    this: Ptr,
    file: Ptr,
    target: u32,
    message: u32,
    add: fn(&mut Engine, Ptr, u32),
) {
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    if size % 4 != 0 {
        return;
    }
    let count = size >> 2;
    if count == 0 {
        return;
    }
    // `MUL`/`SETO`/`NEG`/`OR`: all ones when the byte count overflows.
    let bytes = count.checked_mul(4).unwrap_or(u32::MAX);
    let array = e.call(ALLOCATE, &args![bytes]).u32();
    e.call(GET_CHUNK_DATA, &args![file, array, size]);
    for i in 0..count {
        let slot = array + 4 * i;
        if e.call(FILE_SWAPS, &args![file]).bool() {
            e.call(SWAP_WORD, &args![slot, 0u32]);
        }
        e.call(ADD_COMPILE_INDEX, &args![slot, file]);
        let id = e.mem.u32(slot);
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let cast = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TES_FORM_TYPE, target, 0u32],
            )
            .u32();
        if cast != 0 {
            add(e, this, cast);
        } else {
            let form_id = e.call(FORM_ID, &args![this]).u32();
            let name = e.vcall(this.addr(), 0x130, &args![]).u32();
            let id = e.mem.u32(slot);
            e.call(PRINT_ERROR, &args![message, id, name, form_id]);
        }
    }
    e.call(FREE, &args![array]);
}

/// Stores one float of a face-geometry chunk into element `index` of
/// `vector` (FLD/FSTP: a signalling NaN is quieted).
fn store_face_float(e: &mut Engine, vector: u32, index: u32, value: u32) {
    e.with_stack(8, |e, iterator| {
        let at = e.call(VECTOR_AT, &args![vector, iterator, index]).u32();
        let element = e.call(ITERATOR_ELEMENT, &args![at, 0u32]).u32();
        e.mem.set_u32(element, x87_quiet_bits(value));
    });
}

/// The `FGGS`, `FGGA` and `FGTS` chunks: the floats go into the vector of
/// the chunk at `+0x3f8`, and also into the one at `+0x478` while the sex
/// is male or no `NAM2` has been read.
fn load_face_geometry(e: &mut Engine, race: u32, file: Ptr, chunk: u32, sex: u32, separate: bool) {
    let (group, kind): (u32, u32) = match chunk {
        CHUNK_FGGS => (0, 0),
        CHUNK_FGGA => (0, 1),
        _ => (1, 0),
    };
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    let count = size >> 2;
    let offset = (group << 6).wrapping_add(kind << 5);
    let first = race.wrapping_add(0x478).wrapping_add(offset);
    let second = race.wrapping_add(0x3f8).wrapping_add(offset);
    let both = sex == 0 || separate;
    if both {
        e.call(VECTOR_RESIZE, &args![first, count, 1u32]);
    }
    e.call(VECTOR_RESIZE, &args![second, count, 1u32]);
    // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
    let reserved = size.wrapping_add(15) & !15;
    e.with_stack(reserved, |e, buffer| {
        e.call(MEMSET, &args![buffer, 0u32, size]);
        e.call(GET_CHUNK_DATA, &args![file, buffer, size]);
        for i in 0..count {
            let slot = buffer.addr() + 4 * i;
            if e.call(FILE_SWAPS, &args![file]).bool() {
                e.call(SWAP_WORD, &args![slot, 0u32]);
            }
            if both {
                let value = e.mem.u32(slot);
                store_face_float(e, first, i, value);
            }
            let value = e.mem.u32(slot);
            store_face_float(e, second, i, value);
        }
    });
}

/// The `INDX` chunk's index shift for head parts: 1 for indices above 1
/// once the parts are numbered without the second slot (no `NAM2` read).
fn head_part_shift(separate: bool, index: i32) -> i32 {
    if separate && index > 1 {
        1
    } else {
        0
    }
}

/// `INDX`: resets the model and texture of the part the index names to
/// their defaults (the path tables).
fn reset_indexed_part(
    e: &mut Engine,
    race: u32,
    sex: u32,
    index: i32,
    head_parts: bool,
    separate: bool,
) {
    if head_parts {
        let shift = head_part_shift(separate, index);
        if index < 8 && index - shift != 1 {
            let part = index - shift;
            let model = element(race + 0xb0, 0xc0, sex, 0x18, part);
            e.vcall(model, 0, &args![]);
            let path = e.mem.u32(element(HEAD_MODEL_PATHS, 0x20, sex, 4, part));
            e.vcall(model, 0x18, &args![path]);
            let texture = element(race + 0x230, 0x60, sex, 0xc, part);
            e.vcall(texture, 0, &args![]);
            let path = e.mem.u32(element(HEAD_TEXTURE_PATHS, 0x20, sex, 4, part));
            e.call(0x0048_9100, &args![texture, path]);
        }
        return;
    }
    if index < 3 {
        let model = element(race + 0x338, 0x48, sex, 0x18, index);
        e.vcall(model, 0, &args![]);
        let path = e.mem.u32(element(BODY_PART_PATHS, 0xc, sex, 4, index));
        e.vcall(model, 0x18, &args![path]);
        let texture = element(race + 0x2f0, 0x24, sex, 0xc, index);
        e.vcall(texture, 0, &args![]);
    }
    if index == 3 {
        let model = element(race + 0x3c8, 0x18, sex, 0, 0);
        e.vcall(model, 0, &args![]);
        let path = e.mem.u32(element(BODY_MODEL_PATHS, 4, sex, 0, 0));
        e.vcall(model, 0x18, &args![path]);
    }
}

/// Any other chunk: a model or texture chunk of the part the last `INDX`
/// named.
fn load_indexed_part(
    e: &mut Engine,
    race: u32,
    file: Ptr,
    chunk: u32,
    sex: u32,
    index: i32,
    head_parts: bool,
    separate: bool,
) {
    if head_parts {
        let shift = head_part_shift(separate, index);
        let part = index - shift;
        if part < 8 && part != 1 {
            if e.call(IS_TEXTURE_CHUNK, &args![chunk]).bool() {
                let texture = element(race + 0x230, 0x60, sex, 0xc, part);
                e.call(LOAD_TEXTURE_CHUNK, &args![texture, file]);
            } else if e.call(IS_MODEL_CHUNK, &args![chunk]).bool() {
                let model = element(race + 0xb0, 0xc0, sex, 0x18, part);
                e.call(LOAD_MODEL_CHUNK, &args![model, file]);
            }
        }
        return;
    }
    if index < 3 {
        if e.call(IS_TEXTURE_CHUNK, &args![chunk]).bool() {
            let texture = element(race + 0x2f0, 0x24, sex, 0xc, index);
            e.call(LOAD_TEXTURE_CHUNK, &args![texture, file]);
        } else if e.call(IS_MODEL_CHUNK, &args![chunk]).bool() {
            let model = element(race + 0x338, 0x48, sex, 0x18, index);
            e.call(LOAD_MODEL_CHUNK, &args![model, file]);
        }
    }
    if index == 3 {
        let model = element(race + 0x3c8, 0x18, sex, 0, 0);
        e.call(LOAD_MODEL_CHUNK, &args![model, file]);
    }
}

// Translated from 00610cd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00610cd0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESRace::Load` (engine map): `__thiscall`, `RET 4`, `file`. Answers
/// false when the record's form type is not 0xc; otherwise reads the form
/// header and every chunk and answers true. The chunks:
/// - `EDID` (virtual slot 0x134 with a stack copy), `OBND` (slot 0xe0),
///   `FULL` (`00487050` on +0x18), `DESC` (`LoadDescriptionData` on +0x24),
///   `SPLO` (a spell id for `TESSpellList::AddSpell_ov3` on +0x2c);
/// - `DATA`: 0x24 bytes into +0x50 (`TESForm::LoadData`), byte-swapped by
///   `00611db0` after the chunk when the file asks for it;
/// - `XNAM`: a 0xc-byte allocation read (and swapped by `005a6b10`), added
///   to the list `00717e50` gives for +0x40;
/// - `VTCK` (voices, `fn_00610c70`; 4 bytes: the defaults), `DNAM`
///   (default hair, `fn_00613890`), `CNAM` (one or two bytes at +0x9c),
///   `ONAM`/`YNAM` (older and younger race ids at +0x504/+0x508), `PNAM`,
///   `UNAM` (words at +0xa0, +0xa4), `SNAM` (`GetChunkData_ov2` on +0x4f8
///   when at least 2 bytes), `ATTR` (`00480130` on +0x74 and +0x80);
/// - `HNAM`/`ENAM`: hair and eye lists (`fn_00613810`, `fn_00613910`);
/// - `MNAM`/`FNAM` select the sex, `NAM0`/`NAM1` head or body parts,
///   `NAM2` the head part numbering, `INDX` a part (resetting its model and
///   texture), and other chunks are that part's model or texture chunks;
/// - `FGGS`, `FGGA`, `FGTS`: the face geometry floats.
///
/// The stack cookie check is not translated.
pub fn tes_race_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let race = this.addr();
    let mut sex = 0u32;
    let mut index = 0i32;
    let mut head_parts = false;
    let mut separate = true;
    if e.call(GET_TES_FORM, &args![file]).u32() != FORM_TYPE_RACE {
        return false;
    }
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                    e.vcall(race, 0x134, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(race, 0xe0, &args![file]);
            }
            CHUNK_FULL => {
                let name = if race == 0 { 0 } else { race + 0x18 };
                e.call(0x0048_7050, &args![name, file]);
            }
            CHUNK_DESC => {
                let description = if race == 0 { 0 } else { race + 0x24 };
                e.call(0x0048_2d50, &args![description, file]);
            }
            CHUNK_SPLO => {
                e.with_stack(4, |e, spell| {
                    e.call(GET_CHUNK_WORD, &args![file, spell]);
                    let spell = e.mem.u32(spell.addr());
                    e.call(0x0048_d290, &args![race + 0x2c, spell]);
                });
            }
            CHUNK_XNAM => {
                let block = e.call(ALLOCATE, &args![0xcu32]).u32();
                e.call(GET_CHUNK_DATA, &args![file, block, 0xcu32]);
                if e.call(FILE_SWAPS, &args![file]).bool() {
                    e.call(0x005a_6b10, &args![block]);
                }
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), block);
                    let list = e.call(0x0071_7e50, &args![race + 0x40]).u32();
                    e.call(LIST_ADD_00905820, &args![list, slot]);
                });
            }
            CHUNK_DATA => {
                e.call(TES_FORM_LOAD_DATA, &args![this, file, race + 0x50, 0x24u32]);
            }
            CHUNK_ONAM | CHUNK_YNAM => {
                e.with_stack(4, |e, id| {
                    e.call(GET_CHUNK_WORD, &args![file, id]);
                    let offset = if chunk == CHUNK_ONAM { 0x504 } else { 0x508 };
                    let id = e.mem.u32(id.addr());
                    e.mem.set_u32(race + offset, id);
                });
            }
            CHUNK_VTCK => {
                if e.call(CHUNK_SIZE, &args![file]).u32() == 4 {
                    let male = e.global::<u32>(0x011c_b298);
                    fn_00610c70(e, this, 0, male);
                    let female = e.global::<u32>(0x011c_b29c);
                    fn_00610c70(e, this, 1, female);
                } else if e.call(CHUNK_SIZE, &args![file]).u32() == 8 {
                    let (male, female) = e.with_stack(8, |e, pair| read_two_words(e, file, pair));
                    fn_00610c70(e, this, 0, male);
                    fn_00610c70(e, this, 1, female);
                }
            }
            CHUNK_CNAM => {
                if e.call(CHUNK_SIZE, &args![file]).u32() == 1 {
                    e.call(GET_CHUNK_DATA, &args![file, race + 0x9c, 2u32]);
                    e.call(GET_CHUNK_DATA, &args![file, race + 0x9d, 2u32]);
                } else if e.call(CHUNK_SIZE, &args![file]).u32() == 2 {
                    e.with_stack(4, |e, bytes| {
                        e.call(GET_CHUNK_DATA, &args![file, bytes, 2u32]);
                        for i in 0..2 {
                            let value = e.mem.u8(bytes.addr() + i as u32);
                            fn_00611e30(e, this, i, value);
                        }
                    });
                }
            }
            CHUNK_PNAM => {
                e.call(GET_CHUNK_WORD, &args![file, race + 0xa0]);
            }
            CHUNK_UNAM => {
                e.call(GET_CHUNK_WORD, &args![file, race + 0xa4]);
            }
            CHUNK_DNAM => {
                let (male, female) = e.with_stack(8, |e, pair| read_two_words(e, file, pair));
                fn_00613890(e, this, 0, male);
                fn_00613890(e, this, 1, female);
            }
            CHUNK_ATTR => {
                let count = e.call(0x009d_2510, &args![race + 0x74]).u32();
                let size = count << 1;
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, size]);
                    e.call(0x0048_0130, &args![race + 0x74, buffer]);
                    e.call(
                        0x0048_0130,
                        &args![race + 0x80, buffer.addr().wrapping_add(count)],
                    );
                });
            }
            CHUNK_FGGS | CHUNK_FGGA | CHUNK_FGTS => {
                load_face_geometry(e, race, file, chunk, sex, separate);
            }
            CHUNK_FNAM => sex = 1,
            CHUNK_MNAM => sex = 0,
            CHUNK_INDX => {
                e.with_stack(4, |e, slot| {
                    e.mem.set_i32(slot.addr(), index);
                    e.call(GET_CHUNK_WORD, &args![file, slot]);
                    index = e.mem.i32(slot.addr());
                });
                reset_indexed_part(e, race, sex, index, head_parts, separate);
            }
            CHUNK_NAM0 => head_parts = true,
            CHUNK_NAM1 => head_parts = false,
            CHUNK_NAM2 => separate = false,
            CHUNK_SNAM => {
                if e.call(CHUNK_SIZE, &args![file]).u32() >= 2 {
                    e.call(GET_CHUNK_DATA_OV2, &args![file, race + 0x4f8]);
                }
            }
            CHUNK_HNAM => {
                load_form_list(e, this, file, 0x0118_63a0, 0x0104_b658, fn_00613810);
            }
            CHUNK_ENAM => {
                load_form_list(e, this, file, 0x0118_6388, 0x0104_b618, fn_00613910);
            }
            _ => load_indexed_part(e, race, file, chunk, sex, index, head_parts, separate),
        }
        if chunk == CHUNK_DATA && e.call(FILE_SWAPS, &args![file]).bool() {
            e.call(0x0061_1db0, &args![race + 0x50]);
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// `TESForm::StartForm` (`004855a0`) and `TESForm::CloseForm` (`00485680`),
/// `thiscall` on the form, no stack words.
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
/// The component saves (engine map; `thiscall`): `TESFullName::Save`
/// (`00487010`), `TESDescription::Save` (`00482cb0`), `TESSpellList::Save`
/// (`0048d330`), the reaction form's save (`0048bb70`), `TESModel::Save`
/// (`004891d0`, two tags), `TESModel::SaveFlags` (`00489240`, a tag) and
/// `TESTexture::Save` (`0048e370`, a tag).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const DESCRIPTION_SAVE: u32 = 0x0048_2cb0;
const SPELL_LIST_SAVE: u32 = 0x0048_d330;
const REACTION_SAVE: u32 = 0x0048_bb70;
const MODEL_SAVE: u32 = 0x0048_91d0;
const MODEL_SAVE_FLAGS: u32 = 0x0048_9240;
const TEXTURE_SAVE: u32 = 0x0048_e370;
/// `00401500`: the byte at `011c54ba` (whether saved words are byte-swapped).
const SAVE_SWAPS_BYTES: u32 = 0x0040_1500;
/// `TESForm` chunk writers (cdecl): `__SaveData` (`00485e20`, `thiscall`,
/// `RET 8`: data, size), a word (`00485910`: chunk, word), an empty chunk
/// (`TESForm::AddChunk`, `004856d0`: chunk), `AddChunkArray` (`004856f0`:
/// chunk, data, size), `AddChunkArray_ov2` (`00485710`: chunk, data, count)
/// and `AddChunk_ov2` (`00485950`: chunk, 16-bit word).
const SAVE_DATA: u32 = 0x0048_5e20;
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
const ADD_CHUNK_EMPTY: u32 = 0x0048_56d0;
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
const ADD_CHUNK_ARRAY_OV2: u32 = 0x0048_5710;
const ADD_CHUNK_SHORT: u32 = 0x0048_5950;
/// The attributes components (`thiscall`): their count (`009d2510`, always
/// 7) and their copy into a buffer (`00480100`, `RET 4`).
const ATTRIBUTE_COUNT: u32 = 0x009d_2510;
const ATTRIBUTE_COPY: u32 = 0x0048_0100;
/// `005ae380` (`thiscall` on a list): the number of nodes with an item;
/// `009611e0` (`thiscall` on a face-geometry vector): its size (+0x18).
const LIST_COUNT: u32 = 0x005a_e380;
const VECTOR_SIZE: u32 = 0x0096_11e0;
/// `__RTDynamicCast`'s type descriptor of `TESRace` (`01186370`),
/// `TESForm::CopyAllComponents` (`004851b0`, `RET 4`),
/// `TESForm::CompareAllComponents` (`00485270`, `RET 4`) and `_memcmp`
/// (`00ec4835`, cdecl).
const TYPE_TES_RACE: u32 = 0x0118_6370;
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
const MEMCMP: u32 = 0x00ec_4835;
/// The race accessors outside this unit (`thiscall`): the voice type of a
/// sex (`00604780`, `RET 4`: the word at `+0x4fc + 4 * sex`), the floats
/// at `+0x60 + 4 * sex` (`00604210`) and `+0x68 + 4 * sex` (`006042d0`)
/// in `ST0` (`RET 4`), the attributes' address (`004fadc0`: `+0x74`), the
/// word at +0x70 (`004fd380`) and the 16-bit word at +0x4f8 (`00607350`).
const VOICE_TYPE: u32 = 0x0060_4780;
const FLOAT_60: u32 = 0x0060_4210;
const FLOAT_68: u32 = 0x0060_42d0;
const ATTRIBUTES: u32 = 0x004f_adc0;
const WORD_70: u32 = 0x004f_d380;
const WORD_4F8: u32 = 0x0060_7350;
/// `00652900` (cdecl, two face coordinate sets): whether they differ; the
/// random number (`00487f50`).
const FACE_COORDS_DIFFER: u32 = 0x0065_2900;
const RANDOM: u32 = 0x0048_7f50;
/// The defaults of the two floats at +0xa0 and +0xa4 (5.0 at `0101712c`,
/// 3.0 at `01017718`) and the 0.0 they are compared with (`01012060`).
const DEFAULT_A0: u32 = 0x0101_712c;
const DEFAULT_A4: u32 = 0x0101_7718;
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The model and texture chunk tags.
const CHUNK_MODL: u32 = u32::from_le_bytes(*b"MODL");
const CHUNK_MODT: u32 = u32::from_le_bytes(*b"MODT");
const CHUNK_MODD: u32 = u32::from_le_bytes(*b"MODD");
const CHUNK_ICON: u32 = u32::from_le_bytes(*b"ICON");
/// `TESSoundFile::SetSoundFile` (engine map, `00489100`, `thiscall`,
/// `RET 4`): the race's texture name setter.
const SET_TEXTURE_NAME: u32 = 0x0048_9100;

// Translated from 00611db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the race data's words (engine map: no name; no stack words):
/// `+0x10 + 4 * i` and `+0x18 + 4 * i` for `i` in 0..2, then `+0x20`
/// (`00401080` with a zero byte).
pub fn fn_00611db0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for i in 0..2u32 {
        e.call(SWAP_WORD, &args![base + 0x10 + 4 * i, 0u32]);
        e.call(SWAP_WORD, &args![base + 0x18 + 4 * i, 0u32]);
    }
    e.call(SWAP_WORD, &args![base + 0x20, 0u32]);
}

/// Each sex's chunk marker: `MNAM` for 0, `FNAM` for 1, nothing else.
fn add_sex_marker(e: &mut Engine, sex: u32) {
    if sex == 0 {
        e.call(ADD_CHUNK_EMPTY, &args![CHUNK_MNAM]);
    } else if sex == 1 {
        e.call(ADD_CHUNK_EMPTY, &args![CHUNK_FNAM]);
    }
}

/// The form ids of a list's items until the first null item, into a block
/// of `4 * count` bytes (all ones on overflow) added as `tag`
/// (`AddChunkArray_ov2` with the count) and freed.
fn save_form_list(e: &mut Engine, list: u32, tag: u32) {
    let mut node = list;
    let mut index = 0u32;
    let count = e.call(LIST_COUNT, &args![node]).u32();
    let (size, overflow) = count.overflowing_mul(4);
    let block = e
        .call(ALLOCATE, &args![if overflow { 0xffff_ffff } else { size }])
        .u32();
    while node != 0 {
        let at = e.call(NODE_DATA, &args![node]).u32();
        if e.mem.u32(at) == 0 {
            break;
        }
        let at = e.call(NODE_DATA, &args![node]).u32();
        let form = e.mem.u32(at);
        let id = e.call(FORM_ID, &args![form]).u32();
        e.mem.set_u32(block.wrapping_add(index.wrapping_mul(4)), id);
        index = index.wrapping_add(1);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(ADD_CHUNK_ARRAY_OV2, &args![tag, block, count]);
    e.call(FREE, &args![block]);
}

/// One face-geometry vector of a sex (`male` for sex 0, `female` for sex 1,
/// none for others): when it holds values, they are copied (`FLD`/`FSTP`,
/// element by element through `0060b370` and `00629ab0` on an iterator the
/// game keeps on its stack) into a zeroed block of `4 * count` bytes added
/// as `tag` (`AddChunkArray_ov2` with the count) and freed.
fn save_face_vector(e: &mut Engine, this: u32, sex: u32, (male, female): (u32, u32), tag: u32) {
    let vector = match sex {
        0 => this + male,
        1 => this + female,
        _ => 0,
    };
    let count = if vector != 0 {
        e.call(VECTOR_SIZE, &args![vector]).u32()
    } else {
        0
    };
    if count == 0 {
        return;
    }
    let bytes = count << 2;
    let block = e.call(ALLOCATE, &args![bytes]).u32();
    e.call(MEMSET, &args![block, 0u32, bytes]);
    e.with_stack(8, |e, iterator| {
        let mut index = 0u32;
        while index < count {
            if sex == 0 || sex == 1 {
                let at = e.call(VECTOR_AT, &args![vector, iterator, index]).u32();
                let element = e.call(ITERATOR_ELEMENT, &args![at, 0u32]).u32();
                let value = x87_quiet_bits(e.mem.u32(element));
                e.mem
                    .set_u32(block.wrapping_add(index.wrapping_mul(4)), value);
            }
            index += 1;
        }
    });
    e.call(ADD_CHUNK_ARRAY_OV2, &args![tag, block, count]);
    e.call(FREE, &args![block]);
}

/// The form ids of the two words at `this + offset` (0 for a null word)
/// into a stack pair added as `tag` (`AddChunkArray_ov2` with 2), when
/// either word is set.
fn save_form_pair(e: &mut Engine, this: u32, offset: u32, tag: u32) {
    if e.mem.u32(this + offset) == 0 && e.mem.u32(this + offset + 4) == 0 {
        return;
    }
    e.with_stack(8, |e, pair| {
        for i in 0..2u32 {
            let form = e.mem.u32(this + offset + 4 * i);
            let id = if form != 0 {
                e.call(FORM_ID, &args![form]).u32()
            } else {
                0
            };
            e.mem.set_u32(pair.addr() + 4 * i, id);
        }
        e.call(ADD_CHUNK_ARRAY_OV2, &args![tag, pair, 2u32]);
    });
}

// Translated from 00611e50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00611e50 - the /GS cookie check (00ec408c) is not translated; the __alloca_probe_16 (00ec5ec0) attribute buffer is a with_stack block (its 16-byte alignment padding is not modelled).
/// `TESRace`'s save (engine map: no name; no stack words): `StartForm`, the
/// full name (+0x18), description (+0x24), spells (+0x2c), reactions
/// (+0x40, `0048bb70`); the 0x24 bytes of data at +0x50 (`__SaveData`;
/// `fn_00611db0` before and after when `00401500` says so); the forms at
/// +0x504 and +0x508 by id (`ONAM`, `YNAM`); `NAM2`; the voice types
/// (+0x4fc) as `VTCK` and the default hairs (+0x94) as `DNAM` when either
/// of each pair is set; the two bytes at +0x9c (`CNAM`); the floats at
/// +0xa0 (`PNAM`) and +0xa4 (`UNAM`); the attributes of both sexes (+0x74,
/// +0x80, 7 bytes each) as `ATTR` (with size 2, as the exe passes); `NAM0`
/// and per sex (`MNAM`, `FNAM`) the eight head parts (`INDX`, the model
/// with `MODL`/`MODT`, its flags as `MODD`, the texture as `ICON`); `NAM1`
/// and per sex the three body parts (`INDX`, texture, model, flags) and
/// the body model (`INDX` 3); the hairs (`HNAM`) and eyes (`ENAM`) by id;
/// per sex the face geometry vectors (`FGGS`, `FGGA`, `FGTS`) and the
/// 16-bit word at +0x4f8 (`SNAM`); `CloseForm`.
pub fn fn_00611e50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![base + 0x18]);
    e.call(DESCRIPTION_SAVE, &args![base + 0x24]);
    e.call(SPELL_LIST_SAVE, &args![base + 0x2c]);
    e.call(REACTION_SAVE, &args![base + 0x40]);
    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
        fn_00611db0(e, Ptr::new(base + 0x50));
    }
    e.call(SAVE_DATA, &args![this, base + 0x50, 0x24u32]);
    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
        fn_00611db0(e, Ptr::new(base + 0x50));
    }
    for (offset, tag) in [(0x504u32, CHUNK_ONAM), (0x508, CHUNK_YNAM)] {
        let form = e.mem.u32(base + offset);
        if form != 0 {
            let id = e.call(FORM_ID, &args![form]).u32();
            e.call(ADD_CHUNK_WORD, &args![tag, id]);
        }
    }
    e.call(ADD_CHUNK_EMPTY, &args![CHUNK_NAM2]);
    save_form_pair(e, base, 0x4fc, CHUNK_VTCK);
    save_form_pair(e, base, 0x94, CHUNK_DNAM);
    e.call(ADD_CHUNK_ARRAY, &args![CHUNK_CNAM, base + 0x9c, 2u32]);
    for (offset, tag) in [(0xa0u32, CHUNK_PNAM), (0xa4, CHUNK_UNAM)] {
        let value = x87_quiet_bits(e.mem.u32(base + offset));
        e.call(ADD_CHUNK_WORD, &args![tag, value]);
    }
    let count = e.call(ATTRIBUTE_COUNT, &args![base + 0x74]).u32();
    e.with_stack(count << 1, |e, buffer| {
        e.call(ATTRIBUTE_COPY, &args![base + 0x74, buffer]);
        e.call(
            ATTRIBUTE_COPY,
            &args![base + 0x80, buffer.addr().wrapping_add(count)],
        );
        e.call(ADD_CHUNK_ARRAY, &args![CHUNK_ATTR, buffer, 2u32]);
    });
    e.call(ADD_CHUNK_EMPTY, &args![CHUNK_NAM0]);
    for sex in 0..2u32 {
        add_sex_marker(e, sex);
        for part in 0..8u32 {
            e.call(ADD_CHUNK_WORD, &args![CHUNK_INDX, part]);
            let model = base + 0xb0 + sex * 0xc0 + part * 0x18;
            e.call(MODEL_SAVE, &args![model, CHUNK_MODL, CHUNK_MODT]);
            e.call(MODEL_SAVE_FLAGS, &args![model, CHUNK_MODD]);
            let texture = base + 0x230 + sex * 0x60 + part * 0xc;
            e.call(TEXTURE_SAVE, &args![texture, CHUNK_ICON]);
        }
    }
    e.call(ADD_CHUNK_EMPTY, &args![CHUNK_NAM1]);
    for sex in 0..2u32 {
        add_sex_marker(e, sex);
        let mut part = 0u32;
        while part < 3 {
            e.call(ADD_CHUNK_WORD, &args![CHUNK_INDX, part]);
            let texture = base + 0x2f0 + sex * 0x24 + part * 0xc;
            e.call(TEXTURE_SAVE, &args![texture, CHUNK_ICON]);
            let model = base + 0x338 + sex * 0x48 + part * 0x18;
            e.call(MODEL_SAVE, &args![model, CHUNK_MODL, CHUNK_MODT]);
            e.call(MODEL_SAVE_FLAGS, &args![model, CHUNK_MODD]);
            part += 1;
        }
        e.call(ADD_CHUNK_WORD, &args![CHUNK_INDX, part]);
        let body = base + 0x3c8 + sex * 0x18;
        e.call(MODEL_SAVE, &args![body, CHUNK_MODL, CHUNK_MODT]);
    }
    let hairs = e.call(HAIR_LIST, &args![this]).u32();
    save_form_list(e, hairs, CHUNK_HNAM);
    let eyes = e.call(EYE_LIST, &args![this]).u32();
    save_form_list(e, eyes, CHUNK_ENAM);
    for sex in 0..2u32 {
        add_sex_marker(e, sex);
        save_face_vector(e, base, sex, (0x478, 0x3f8), CHUNK_FGGS);
        save_face_vector(e, base, sex, (0x498, 0x418), CHUNK_FGGA);
        save_face_vector(e, base, sex, (0x4b8, 0x438), CHUNK_FGTS);
        let word = u32::from(e.mem.u16(base + 0x4f8));
        e.call(ADD_CHUNK_SHORT, &args![CHUNK_SNAM, word]);
    }
    e.call(CLOSE_FORM, &args![this]);
}

/// `other` as a `TESRace` (`__RTDynamicCast` from `TESForm`), or 0.
fn as_race(e: &mut Engine, other: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![other, 0u32, TES_FORM_TYPE, TYPE_TES_RACE, 0u32],
    )
    .u32()
}

// Translated from 00612c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRace`'s copy (engine map: no name; `RET 4`): when `other` is a
/// `TESRace`: all components (`TESForm::CopyAllComponents` with `other`),
/// the words at +0x504 and +0x508, the voice types and default hairs of
/// both sexes, the two bytes at +0x9c, the floats at +0xa0 and +0xa4 (their
/// getters, `FSTP`), the attributes (+0x74, +0x80, vtable slot `+8` with the
/// other's), the 7 pairs of data bytes at +0x50, the floats at +0x60 and
/// +0x68 of both sexes, the word at +0x70; per sex the eight head part
/// models (by name, `fn_006139b0`) and textures (`fn_00613b60`), the three
/// body textures (`fn_00613c00`) and models (`fn_00613a60`) and the body
/// model (`fn_00613ad0`); the hair and eye lists (emptied, then each item
/// of the other's until a null one appended); both sexes' face coordinates
/// (`BSFaceGenManager::CopyFaceGenCoord`) and the word at +0x4f8.
pub fn fn_00612c90(e: &mut Engine, this: Ptr, other: u32) {
    let source = as_race(e, other);
    if source == 0 {
        return;
    }
    let base = this.addr();
    e.call(COPY_ALL_COMPONENTS, &args![this, other]);
    for offset in [0x504u32, 0x508] {
        let word = e.mem.u32(source + offset);
        e.mem.set_u32(base + offset, word);
    }
    for sex in 0..2u32 {
        let voice = e.call(VOICE_TYPE, &args![source, sex]).u32();
        e.mem.set_u32(base + 0x4fc + 4 * sex, voice);
    }
    for sex in 0..2i32 {
        let hair = fn_00613870(e, Ptr::new(source), sex);
        e.mem.set_u32(base + 0x94 + 4 * sex as u32, hair);
    }
    for sex in 0..2i32 {
        let byte = fn_00613100(e, Ptr::new(source), sex);
        e.mem.set_u8(base + 0x9c + sex as u32, byte);
    }
    let value = fn_00613120(e, Ptr::new(source));
    e.mem.set_u32(base + 0xa0, x87_store32(f64::from(value)));
    let value = fn_00613160(e, Ptr::new(source));
    e.mem.set_u32(base + 0xa4, x87_store32(f64::from(value)));
    let attributes = e.call(ATTRIBUTES, &args![source]).u32();
    e.vcall(base + 0x74, 8, &args![attributes]);
    let attributes = fn_006130e0(e, Ptr::new(source)).addr();
    e.vcall(base + 0x80, 8, &args![attributes]);
    for index in 0..7i32 {
        let at = fn_006130c0(e, Ptr::new(source), index);
        let byte = e.mem.u8(at.addr());
        e.mem.set_u8(base + 0x50 + 2 * index as u32, byte);
        let at = fn_006130c0(e, Ptr::new(source), index);
        let byte = e.mem.u8(at.addr() + 1);
        e.mem.set_u8(base + 0x51 + 2 * index as u32, byte);
    }
    for sex in 0..2u32 {
        let value = e.call(FLOAT_60, &args![source, sex]).f64();
        e.mem.set_u32(base + 0x60 + 4 * sex, x87_store32(value));
        let value = e.call(FLOAT_68, &args![source, sex]).f64();
        e.mem.set_u32(base + 0x68 + 4 * sex, x87_store32(value));
    }
    let word = e.call(WORD_70, &args![source]).u32();
    e.mem.set_u32(base + 0x70, word);
    for sex in 0..2i32 {
        for part in 0..8i32 {
            let model = fn_00613970(e, Ptr::new(source), sex, part);
            let name = e.vcall(model.addr(), 0x14, &[]).u32();
            fn_006139b0(e, this, sex, part, name);
            let texture = fn_00613b20(e, Ptr::new(source), sex, part);
            let text = e.call(PATH_TEXT, &args![texture]).u32();
            fn_00613b60(e, this, sex, part, text);
        }
    }
    for sex in 0..2i32 {
        for part in 0..3i32 {
            let texture = fn_00613bb0(e, Ptr::new(source), sex, part);
            let text = e.call(PATH_TEXT, &args![texture]).u32();
            fn_00613c00(e, this, sex, part, text);
        }
        for part in 0..3i32 {
            let model = fn_00613a20(e, Ptr::new(source), sex, part);
            let name = e.vcall(model.addr(), 0x14, &[]).u32();
            fn_00613a60(e, this, sex, part, name);
        }
        let body = fn_006131a0(e, Ptr::new(source), sex);
        let name = e.vcall(body.addr(), 0x14, &[]).u32();
        fn_00613ad0(e, this, sex, name);
    }
    for (list_of, clear) in [
        (HAIR_LIST, fn_00613850 as fn(&mut Engine, Ptr)),
        (EYE_LIST, fn_00613950),
    ] {
        let mut node = e.call(list_of, &args![source]).u32();
        clear(e, this);
        while node != 0 {
            let at = e.call(NODE_DATA, &args![node]).u32();
            if e.mem.u32(at) == 0 {
                break;
            }
            let at = e.call(NODE_DATA, &args![node]).u32();
            let list = e.call(list_of, &args![this]).u32();
            e.call(LIST_ADD_00905820, &args![list, at]);
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
    for sex in 0..2u32 {
        let to = e.call(RACE_FACE_COORD, &args![this, sex]).u32();
        let from = e.call(RACE_FACE_COORD, &args![source, sex]).u32();
        e.call(COPY_FACE_COORD, &args![from, to, 0u32, 0u32]);
    }
    let word = e.call(WORD_4F8, &args![source]).u32() as u16;
    e.mem.set_u16(base + 0x4f8, word);
}

// Translated from 006130c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the data byte pair `index` (`this + 0x50 + 2 * index`;
/// engine map: no name; `RET 4`).
pub fn fn_006130c0(_e: &mut Engine, this: Ptr, index: i32) -> Ptr {
    Ptr::new(
        this.addr()
            .wrapping_add((index as u32).wrapping_mul(2))
            .wrapping_add(0x50),
    )
}

// Translated from 00613100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 0x9c + index` (engine map: no name; `RET 4`).
pub fn fn_00613100(e: &mut Engine, this: Ptr, index: i32) -> u8 {
    e.mem
        .u8(this.addr().wrapping_add(index as u32).wrapping_add(0x9c))
}

/// The float at `this + offset` when it is above 0.0 (`FCOMP` with the
/// double at `01012060`), else the float at `default`; loaded and returned
/// in `ST0`.
fn positive_or_default(e: &mut Engine, this: Ptr, offset: u32, default: u32) -> f32 {
    let bits = e.mem.u32(this.addr() + offset);
    let chosen = if x87_load32(bits) > e.global::<f64>(ZERO_DOUBLE) {
        bits
    } else {
        e.global::<u32>(default)
    };
    f32::from_bits(x87_quiet_bits(chosen))
}

// Translated from 00613120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at +0xa0 when above 0.0, else 5.0 (`0101712c`), in `ST0`
/// (engine map: no name; no stack words).
pub fn fn_00613120(e: &mut Engine, this: Ptr) -> f32 {
    positive_or_default(e, this, 0xa0, DEFAULT_A0)
}

// Translated from 00613160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at +0xa4 when above 0.0, else 3.0 (`01017718`), in `ST0`
/// (engine map: no name; no stack words).
pub fn fn_00613160(e: &mut Engine, this: Ptr) -> f32 {
    positive_or_default(e, this, 0xa4, DEFAULT_A4)
}

/// A component's compare (its vtable slot `+0xc` with the other's
/// component), tested on `AL` as the exe does.
fn part_differs(e: &mut Engine, part: u32, other: u32) -> bool {
    e.vcall(part, 0xc, &args![other]).u32() as u8 != 0
}

// Translated from 006131c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRace`'s compare (engine map: no name; `RET 4`): true when `other` is
/// no `TESRace` or when anything differs, in the exe's order:
/// `TESForm::CompareAllComponents` (with the cast form), the 0x24 bytes of
/// data at +0x50 (`_memcmp`), the words at +0x504 and +0x508, the voice
/// types, default hairs and the bytes at +0x9c of both sexes, the floats of
/// `fn_00613120` and `fn_00613160` (`FUCOMPP`: a NaN differs), the
/// attributes (+0x74, +0x80, vtable slot `+0xc` with the other's), per sex
/// the eight head part models and textures and the three body textures,
/// three body models and the body model (each component's slot `+0xc`),
/// the hair and eye counts and each of the other's hairs and eyes by form
/// id (`fn_006137b0`, `fn_006138b0`); then for each sex whose face
/// coordinates differ (`00652900`) the word at +0x4f8 becomes a random
/// number (`00487f50`) and the answer is true; finally the words at +0x4f8.
pub fn fn_006131c0(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let source = as_race(e, other);
    if source == 0 {
        return true;
    }
    let base = this.addr();
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, source]).u32() as u8 != 0 {
        return true;
    }
    let data = fn_00613790(e, Ptr::new(source)).addr();
    if e.call(MEMCMP, &args![base + 0x50, data, 0x24u32]).u32() != 0 {
        return true;
    }
    for offset in [0x504u32, 0x508] {
        if e.mem.u32(base + offset) != e.mem.u32(source + offset) {
            return true;
        }
    }
    for sex in 0..2u32 {
        let mine = e.call(VOICE_TYPE, &args![this, sex]).u32();
        if mine != e.call(VOICE_TYPE, &args![source, sex]).u32() {
            return true;
        }
    }
    for sex in 0..2i32 {
        let mine = fn_00613870(e, this, sex);
        if mine != fn_00613870(e, Ptr::new(source), sex) {
            return true;
        }
    }
    for sex in 0..2i32 {
        let mine = fn_00613100(e, this, sex) as i8;
        if mine != fn_00613100(e, Ptr::new(source), sex) as i8 {
            return true;
        }
    }
    let mine = f64::from(fn_00613120(e, this));
    if mine != f64::from(fn_00613120(e, Ptr::new(source))) {
        return true;
    }
    let mine = f64::from(fn_00613160(e, this));
    if mine != f64::from(fn_00613160(e, Ptr::new(source))) {
        return true;
    }
    let attributes = e.call(ATTRIBUTES, &args![source]).u32();
    if part_differs(e, base + 0x74, attributes) {
        return true;
    }
    let attributes = fn_006130e0(e, Ptr::new(source)).addr();
    if part_differs(e, base + 0x80, attributes) {
        return true;
    }
    for sex in 0..2u32 {
        for part in 0..8u32 {
            let theirs = fn_00613970(e, Ptr::new(source), sex as i32, part as i32).addr();
            if part_differs(e, base + 0xb0 + sex * 0xc0 + part * 0x18, theirs) {
                return true;
            }
            let theirs = fn_00613b20(e, Ptr::new(source), sex as i32, part as i32).addr();
            if part_differs(e, base + 0x230 + sex * 0x60 + part * 0xc, theirs) {
                return true;
            }
        }
    }
    for sex in 0..2u32 {
        for part in 0..3u32 {
            let offset = 0x2f0 + sex * 0x24 + part * 0xc;
            if part_differs(e, base + offset, source + offset) {
                return true;
            }
            let offset = 0x338 + sex * 0x48 + part * 0x18;
            if part_differs(e, base + offset, source + offset) {
                return true;
            }
        }
        let offset = 0x3c8 + sex * 0x18;
        if part_differs(e, base + offset, source + offset) {
            return true;
        }
    }
    for (list_of, holds) in [
        (HAIR_LIST, fn_006137b0 as fn(&mut Engine, Ptr, u32) -> u32),
        (EYE_LIST, fn_006138b0),
    ] {
        let mut node = e.call(list_of, &args![source]).u32();
        let list = e.call(list_of, &args![this]).u32();
        let mine = e.call(LIST_COUNT, &args![list]).u32();
        if mine != e.call(LIST_COUNT, &args![node]).u32() {
            return true;
        }
        while node != 0 {
            let at = e.call(NODE_DATA, &args![node]).u32();
            if e.mem.u32(at) == 0 {
                break;
            }
            let at = e.call(NODE_DATA, &args![node]).u32();
            let form = e.mem.u32(at);
            let id = e.call(FORM_ID, &args![form]).u32();
            if holds(e, this, id) == 0 {
                return true;
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
    for sex in 0..2u32 {
        let mine = e.call(RACE_FACE_COORD, &args![this, sex]).u32();
        let theirs = e.call(RACE_FACE_COORD, &args![source, sex]).u32();
        if e.call(FACE_COORDS_DIFFER, &args![theirs, mine]).u32() as u8 != 0 {
            let random = e.call(RANDOM, &[]).u32() as u16;
            e.mem.set_u16(base + 0x4f8, random);
            return true;
        }
    }
    let mine = e.call(WORD_4F8, &args![this]).u32() as u16;
    mine != e.call(WORD_4F8, &args![source]).u32() as u16
}

// Translated from 00613870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Default hair `index` (`this + 0x94 + 4 * index`; engine map: no name;
/// `RET 4`).
pub fn fn_00613870(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    e.mem.u32(
        this.addr()
            .wrapping_add((index as u32).wrapping_mul(4))
            .wrapping_add(0x94),
    )
}

/// `vcall(component, 0x18, [value])` on a model or texture component (its
/// setter, slot `+0x18`).
fn set_component(e: &mut Engine, component: u32, value: u32) {
    e.vcall(component, 0x18, &args![value]);
}

// Translated from 006139b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets head part model `part` (0..8) of `sex` (0..2, signed checks) to
/// `name` (the model's vtable slot `+0x18`; engine map: no name; `RET 0xc`);
/// nothing outside the ranges.
pub fn fn_006139b0(e: &mut Engine, this: Ptr, sex: i32, part: i32, name: u32) {
    if !(0..8).contains(&part) || !(0..2).contains(&sex) {
        return;
    }
    let model = this
        .addr()
        .wrapping_add((sex as u32).wrapping_mul(0xc0))
        .wrapping_add(0xb0)
        .wrapping_add((part as u32).wrapping_mul(0x18));
    set_component(e, model, name);
}

// Translated from 00613a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets body part model `part` (0..3) of `sex` (0..2, signed checks) to
/// `name` (the model's vtable slot `+0x18`; engine map: no name; `RET 0xc`).
pub fn fn_00613a60(e: &mut Engine, this: Ptr, sex: i32, part: i32, name: u32) {
    if !(0..3).contains(&part) || !(0..2).contains(&sex) {
        return;
    }
    let model = this
        .addr()
        .wrapping_add((sex as u32).wrapping_mul(0x48))
        .wrapping_add(0x338)
        .wrapping_add((part as u32).wrapping_mul(0x18));
    set_component(e, model, name);
}

// Translated from 00613ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the body model of `sex` (0..2, signed check) to `name` (vtable slot
/// `+0x18` of `this + 0x3c8 + 0x18 * sex`; engine map: no name; `RET 8`).
pub fn fn_00613ad0(e: &mut Engine, this: Ptr, sex: i32, name: u32) {
    if !(0..2).contains(&sex) {
        return;
    }
    let model = this
        .addr()
        .wrapping_add((sex as u32).wrapping_mul(0x18))
        .wrapping_add(0x3c8);
    set_component(e, model, name);
}

// Translated from 00613b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets head part texture `part` (0..8) of `sex` to `name`
/// (`TESSoundFile::SetSoundFile`, `00489100`, on `this + 0x230 + 0x60 *
/// sex + 0xc * part`; engine map: no name; `RET 0xc`). The exe accepts sex
/// 0..=2 here (`JLE`), one more than the other setters.
pub fn fn_00613b60(e: &mut Engine, this: Ptr, sex: i32, part: i32, name: u32) {
    if !(0..8).contains(&part) || !(0..=2).contains(&sex) {
        return;
    }
    let texture = this
        .addr()
        .wrapping_add((sex as u32).wrapping_mul(0x60))
        .wrapping_add(0x230)
        .wrapping_add((part as u32).wrapping_mul(0xc));
    e.call(SET_TEXTURE_NAME, &args![texture, name]);
}

// Translated from 00613bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Body part texture `part` (0..3) of `sex` (0..2): `this + 0x2f0 + 0x24 *
/// sex + 0xc * part`, or 0 outside the ranges (signed checks; engine map:
/// no name; `RET 8`).
pub fn fn_00613bb0(_e: &mut Engine, this: Ptr, sex: i32, part: i32) -> u32 {
    if !(0..2).contains(&sex) || !(0..3).contains(&part) {
        return 0;
    }
    this.addr()
        .wrapping_add((sex as u32).wrapping_mul(0x24))
        .wrapping_add(0x2f0)
        .wrapping_add((part as u32).wrapping_mul(0xc))
}

// Translated from 00613c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets body part texture `part` (0..3) of `sex` (0..2) to `name`
/// (`00489100` on its address; engine map: no name; `RET 0xc`); nothing
/// outside the ranges.
pub fn fn_00613c00(e: &mut Engine, this: Ptr, sex: i32, part: i32, name: u32) {
    if !(0..2).contains(&sex) || !(0..3).contains(&part) {
        return;
    }
    let texture = this
        .addr()
        .wrapping_add((sex as u32).wrapping_mul(0x24))
        .wrapping_add(0x2f0)
        .wrapping_add((part as u32).wrapping_mul(0xc));
    e.call(SET_TEXTURE_NAME, &args![texture, name]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_30e0, fn_006130e0(Ptr) -> Ptr),
        entry!(0x0061_3790, fn_00613790(Ptr) -> Ptr),
        entry!(0x0061_0c70, fn_00610c70(Ptr, i32, u32)),
        entry!(0x0061_3890, fn_00613890(Ptr, i32, u32)),
        entry!(0x0061_1e30, fn_00611e30(Ptr, i32, u8)),
        entry!(0x0061_3fb0, fn_00613fb0(Ptr, u8)),
        entry!(0x0061_4630, fn_00614630() -> u32),
        entry!(0x0061_31a0, fn_006131a0(Ptr, i32) -> Ptr),
        entry!(0x0061_3a20, fn_00613a20(Ptr, i32, i32) -> Ptr),
        entry!(0x0061_3970, fn_00613970(Ptr, i32, i32) -> Ptr),
        entry!(0x0061_3b20, fn_00613b20(Ptr, i32, i32) -> Ptr),
        entry!(0x0061_37b0, fn_006137b0(Ptr, u32) -> u32),
        entry!(0x0061_38b0, fn_006138b0(Ptr, u32) -> u32),
        entry!(0x0061_3810, fn_00613810(Ptr, u32)),
        entry!(0x0061_3910, fn_00613910(Ptr, u32)),
        entry!(0x0061_3850, fn_00613850(Ptr)),
        entry!(0x0061_3950, fn_00613950(Ptr)),
        entry!(0x0061_28c0, tes_race_init_item(Ptr)),
        entry!(
            0x0061_41f0,
            tes_race_get_face_gen_data(Ptr, u32, u32, u32, u8)
        ),
        entry!(0x0061_3c50, fn_00613c50(Ptr, u32, u32, u32, u8, u8, u32) -> bool),
        entry!(0x0061_3fd0, tes_race_kill_egt_data(Ptr)),
        entry!(0x0061_4640, fn_00614640(Ptr, u32, u32, Ptr) -> bool),
        entry!(
            0x0061_47e0,
            tes_race_get_body_mod_texture_name(Ptr, u32, Ptr)
        ),
        entry!(0x0061_48f0, tes_race_get_body_mod_texture_file_name(Ptr, u32, Ptr, Ptr) -> bool),
        entry!(0x0061_49b0, tes_race_get_body_texture(Ptr, u32, u32, u32, u32, u8) -> bool),
        entry!(0x0061_0cd0, tes_race_load(Ptr, Ptr) -> bool),
        entry!(0x0061_1db0, fn_00611db0(Ptr)),
        entry!(0x0061_1e50, fn_00611e50(Ptr)),
        entry!(0x0061_2c90, fn_00612c90(Ptr, u32)),
        entry!(0x0061_30c0, fn_006130c0(Ptr, i32) -> Ptr),
        entry!(0x0061_3100, fn_00613100(Ptr, i32) -> u8),
        entry!(0x0061_3120, fn_00613120(Ptr) -> f32),
        entry!(0x0061_3160, fn_00613160(Ptr) -> f32),
        entry!(0x0061_31c0, fn_006131c0(Ptr, u32) -> bool),
        entry!(0x0061_3870, fn_00613870(Ptr, i32) -> u32),
        entry!(0x0061_39b0, fn_006139b0(Ptr, i32, i32, u32)),
        entry!(0x0061_3a60, fn_00613a60(Ptr, i32, i32, u32)),
        entry!(0x0061_3ad0, fn_00613ad0(Ptr, i32, u32)),
        entry!(0x0061_3b60, fn_00613b60(Ptr, i32, i32, u32)),
        entry!(0x0061_3bb0, fn_00613bb0(Ptr, i32, i32) -> u32),
        entry!(0x0061_3c00, fn_00613c00(Ptr, i32, i32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// One vtable for every fake object: slot `n` calls `SLOTS + n`.
    const VTABLE: u32 = 0x0200_0000;
    const SLOTS: u32 = 0x0300_0000;

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
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn slot(offset: u32) -> u32 {
        SLOTS + offset
    }

    /// An engine with the shared vtable (all slots no-ops) and no-op
    /// doubles for `callees`.
    fn engine(callees: &[u32]) -> Engine {
        let mut e = Engine::new();
        e.map(VTABLE, 0x200);
        for i in 0..0x80 {
            e.mem.set_u32(VTABLE + 4 * i, slot(4 * i));
            e.register(slot(4 * i), |_, _| Ret::default());
        }
        for &callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    /// A zeroed object of `size` bytes with the shared vtable.
    fn object(e: &mut Engine, size: u32) -> u32 {
        let p = e.mem.alloc(size);
        e.mem.set_u32(p, VTABLE);
        p
    }

    #[test]
    fn field_addresses() {
        let mut e = engine(&[]);
        let r: Ptr = Ptr::new(0x0100_0000);
        assert_eq!(e.call(0x0061_30e0, &args![r]).u32(), 0x0100_0080);
        assert_eq!(e.call(0x0061_3790, &args![r]).u32(), 0x0100_0050);
        assert_eq!(e.call(0x0061_31a0, &args![r, 1i32]).u32(), 0x0100_03e0);
        assert_eq!(
            e.call(0x0061_3a20, &args![r, 1i32, 2i32]).u32(),
            0x0100_0338 + 0x48 + 0x30
        );
        assert_eq!(e.call(0x0061_3a20, &args![r, 1i32, 3i32]).u32(), 0);
        assert_eq!(e.call(0x0061_3a20, &args![r, 0i32, -1i32]).u32(), 0);
        assert_eq!(
            e.call(0x0061_3970, &args![r, 1i32, 7i32]).u32(),
            0x0100_00b0 + 0xc0 + 7 * 0x18
        );
        assert_eq!(e.call(0x0061_3970, &args![r, 0i32, 8i32]).u32(), 0);
        assert_eq!(
            e.call(0x0061_3b20, &args![r, 1i32, 7i32]).u32(),
            0x0100_0230 + 0x60 + 7 * 0xc
        );
        assert_eq!(e.call(0x0061_3b20, &args![r, 0i32, -1i32]).u32(), 0);
        e.map(0x0119_b4e4, 4);
        e.mem.set_u32(0x0119_b4e4, 0x1234);
        assert_eq!(e.call(0x0061_4630, &args![]).u32(), 0x1234);
    }

    #[test]
    fn small_setters() {
        let mut e = engine(&[]);
        let r = e.mem.alloc(0x510);
        e.call(0x0061_3890, &args![r, 1i32, 0x77u32]);
        assert_eq!(e.mem.u32(r + 0x98), 0x77);
        e.call(0x0061_1e30, &args![r, 1i32, 0x1ffu32]);
        assert_eq!(e.mem.u8(r + 0x9d), 0xff);
        e.call(0x0061_3fb0, &args![r, 1u32]);
        assert_eq!(e.mem.u8(r + 0xe1), 1);
    }

    #[test]
    fn voices_take_the_value_or_the_default() {
        let mut e = engine(&[]);
        e.map(0x011c_b298, 8);
        e.mem.set_u32(0x011c_b298, 0xaaa);
        e.mem.set_u32(0x011c_b29c, 0xbbb);
        let r = e.mem.alloc(0x510);
        e.call(0x0061_0c70, &args![r, 1i32, 0x55u32]);
        assert_eq!(e.mem.u32(r + 0x500), 0x55);
        e.call(0x0061_0c70, &args![r, 0i32, 0u32]);
        assert_eq!(e.mem.u32(r + 0x4fc), 0xaaa);
        e.call(0x0061_0c70, &args![r, 1i32, 0u32]);
        assert_eq!(e.mem.u32(r + 0x500), 0xbbb);
        e.call(0x0061_0c70, &args![r, 2i32, 0u32]);
        assert_eq!(e.mem.u32(r + 0x504), 0);
    }

    /// A list of `{data, next}` nodes holding `forms` (form ids at +0xc).
    fn list(e: &mut Engine, forms: &[u32]) -> u32 {
        let mut next = 0;
        for &form in forms.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, form);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    fn form_with_id(e: &mut Engine, id: u32) -> u32 {
        let form = object(e, 0x40);
        e.mem.set_u32(form + 0xc, id);
        form
    }

    fn list_engine(hair: u32, eyes: u32) -> Engine {
        let mut e = engine(&[LIST_ADD_00905820, LIST_CLEAR]);
        e.register(NODE_DATA, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register_double(HAIR_LIST, move |_, _| ret(hair));
        e.register_double(EYE_LIST, move |_, _| ret(eyes));
        e
    }

    #[test]
    fn hair_and_eyes_are_found_by_form_id() {
        let mut probe = Engine::new();
        let a = form_with_id(&mut probe, 0x10);
        let b = form_with_id(&mut probe, 0x20);
        let hair = list(&mut probe, &[a, b]);
        let eyes = list(&mut probe, &[b, 0, a]);
        let mut e = list_engine(hair, eyes);
        e.mem = probe.mem;
        let r = Ptr::<()>::new(0x0100_0000);
        assert_eq!(e.call(0x0061_37b0, &args![r, 0x20u32]).u32(), b);
        assert_eq!(e.call(0x0061_37b0, &args![r, 0x30u32]).u32(), 0);
        assert_eq!(e.call(0x0061_38b0, &args![r, 0x20u32]).u32(), b);
        // The walk stops at the null element before `a`.
        assert_eq!(e.call(0x0061_38b0, &args![r, 0x10u32]).u32(), 0);
    }

    #[test]
    fn adding_skips_null_and_known_forms() {
        let mut probe = Engine::new();
        let a = form_with_id(&mut probe, 0x10);
        let b = form_with_id(&mut probe, 0x20);
        let hair = list(&mut probe, &[a]);
        let mut e = list_engine(hair, hair);
        e.mem = probe.mem;
        let added = Rc::new(RefCell::new(vec![]));
        let seen = added.clone();
        e.register_double(LIST_ADD_00905820, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let r = Ptr::<()>::new(0x0100_0000);
        for addr in [0x0061_3810, 0x0061_3910] {
            e.call(addr, &args![r, 0u32]);
            e.call(addr, &args![r, a]);
            e.call(addr, &args![r, b]);
        }
        assert_eq!(*added.borrow(), vec![(hair, b), (hair, b)]);
        e.call(0x0061_3850, &args![r]);
        e.call(0x0061_3950, &args![r]);
        assert_eq!(calls_to(&e, LIST_CLEAR), vec![vec![hair], vec![hair]]);
    }

    #[test]
    fn init_item_resolves_the_references() {
        let callees = [
            0x0048_4e60,
            0x0048_5d50,
            PRINT_ERROR,
            0x0048_d400,
            0x0048_bc30,
            0x0048_4ab0,
        ];
        let mut e = engine(&callees);
        e.register(0x0040_13e0, |_, _| ret(0));
        e.register(FORM_ID, |_, _| ret(0x0abc));
        e.register(slot(0x130), |_, _| ret(0x0def));
        e.register(0x0048_4e60, |_, _| ret(0x0f11e));
        // The lookup answers the id + 1; the cast answers it unless the id
        // was 0x200 (missing).
        e.register(LOOKUP_FORM, |_, a| ret(a[0] + 1));
        e.register(RT_DYNAMIC_CAST, |_, a| {
            ret(if a[0] == 0x201 { 0 } else { a[0] })
        });
        let r = object(&mut e, 0x510);
        e.mem.set_u32(r + 0x4fc, 0x100);
        e.mem.set_u32(r + 0x98, 0x200);
        e.mem.set_u32(r + 0x508, 0x300);
        e.call(0x0061_28c0, &args![r]);
        assert_eq!(e.mem.u32(r + 0x4fc), 0x101);
        assert_eq!(e.mem.u32(r + 0x500), 0);
        assert_eq!(e.mem.u32(r + 0x98), 0);
        assert_eq!(e.mem.u32(r + 0x508), 0x301);
        let casts = calls_to(&e, RT_DYNAMIC_CAST);
        assert_eq!(casts.len(), 3);
        assert_eq!(casts[0], vec![0x101, 0, TES_FORM_TYPE, 0x0118_6318, 0]);
        assert_eq!(casts[1][3], 0x0118_63a0);
        assert_eq!(casts[2][3], 0x0118_6370);
        assert_eq!(calls_to(&e, 0x0048_4e60)[0], vec![r, 0xffff_ffff]);
        assert_eq!(
            calls_to(&e, PRINT_ERROR),
            vec![vec![0x0104_b728, 0x200, 0x0def, 0x0abc]]
        );
        assert_eq!(calls_to(&e, 0x0048_d400), vec![vec![r + 0x2c, r]]);
        assert_eq!(calls_to(&e, 0x0048_bc30), vec![vec![r + 0x40, r]]);
        assert_eq!(calls_to(&e, 0x0048_4ab0), vec![vec![r, 1]]);
    }

    #[test]
    fn init_item_does_nothing_when_told_so() {
        let mut e = engine(&[]);
        e.register(0x0040_13e0, |_, _| ret(1));
        let r = object(&mut e, 0x510);
        e.mem.set_u32(r + 0x4fc, 0x100);
        e.call(0x0061_28c0, &args![r]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        assert_eq!(e.mem.u32(r + 0x4fc), 0x100);
    }

    const FACE_CALLEES: [u32; 22] = [
        0x0060_3ad0,
        COPY_FACE_COORD,
        0x0096_a610,
        HOLDER_INIT,
        0x0060_44e0,
        0x0060_ba10,
        HOLDER_RELEASE,
        LIST_ADD_005AE3D0,
        0x0060_2170,
        0x0062_4700,
        0x0060_5d40,
        0x0061_3870,
        0x0061_3100,
        0x0060_3b50,
        0x0060_47a0,
        0x0041_69d0,
        0x0060_2150,
        0x0060_2130,
        0x0043_c490,
        0x0040_4dc0,
        PATH_TEXT,
        0x005f_dfa0,
    ];

    fn face_engine() -> Engine {
        let mut e = engine(&FACE_CALLEES);
        e.map(PLAYER, 4);
        e.mem.set_u32(PLAYER, 0x0900);
        e.map(0x011c_3f2c, 4);
        e.map(0x0119_b4e4, 4);
        e.mem.set_u32(0x0119_b4e4, 0x4e4);
        e.register(STRING_TEXT, |_, _| ret(0x0500_0000));
        e.map(0x0500_0000, 4);
        e.register(NODE_DATA, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(NODE_IS_END, |_, _| ret(0));
        e.register(GET_SEX, |_, _| ret(1));
        e.register(GET_BASE_FORM, |_, _| ret(0x0777));
        e.register(0x0061_3870, |_, a| ret(0x3870 + a[1]));
        e.register(0x0061_3100, |_, _| ret(0xff));
        e.register(0x0060_2130, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        e.register(0x0041_69d0, |_, _| ret(0x69d0));
        e.register(0x0060_2150, |_, _| ret(0x2150));
        e.register(0x0060_5d40, |_, a| ret(0x5d40 + a[0]));
        e.register(0x0061_3870, |_, a| ret(0x3870 + a[1]));
        e
    }

    #[test]
    fn face_data_for_the_race_alone() {
        let mut e = face_engine();
        e.register(RACE_FACE_COORD, |_, _| ret(0));
        e.register(DEFAULT_FACE_COORD, |_, _| ret(0x0def));
        let r = e.mem.alloc(0x510);
        let data = e.mem.alloc(0x100);
        // The race has eyes.
        let eyes = e.mem.alloc(8);
        e.mem.set_u32(eyes, 0x0e1e);
        e.register_double(EYE_LIST, move |_, _| ret(eyes));
        e.call(0x0061_41f0, &args![r, 0u32, data, 0u32, 0u32]);
        assert_eq!(
            calls_to(&e, COPY_FACE_COORD),
            vec![vec![0x0def, data, 0, 0]]
        );
        assert_eq!(e.mem.u32(data + 0x80), 0x3870);
        assert_eq!(e.mem.i32(data + 0x84), -1);
        let adds = calls_to(&e, 0x0096_a610);
        assert_eq!(adds.len(), 24);
        assert_eq!(adds[0][0], data + 0x94);
        assert_eq!(adds[1][0], data + 0xa4);
        assert_eq!(adds[2][0], data + 0xb4);
        assert!(calls_to(&e, 0x0060_44e0).is_empty());
        assert_eq!(e.mem.u32(data + 0xe0), 0x4e4);
        assert_eq!(e.mem.u32(data + 0xd8), r + 0xb0 + 6 * 0x18);
        assert_eq!(e.mem.u32(data + 0xdc), r + 0xb0 + 7 * 0x18);
        assert_eq!(e.mem.u32(data + 0x8c), 0x0e1e);
    }

    #[test]
    fn face_data_for_an_npc() {
        let mut e = face_engine();
        let r = e.mem.alloc(0x510);
        let data = e.mem.alloc(0x100);
        let npc = 0x0777u32;
        e.mem.set_u8(0x0500_0000, 1);
        e.register(0x0060_47a0, |_, _| ret(0));
        e.register(0x0060_3b50, |_, _| ret(0x3b50));
        e.register(0x005f_dfa0, |_, _| ret(1));
        // No eyes from the NPC.
        e.register(0x0060_2150, |_, _| ret(0));
        // Head parts: one node holding a part with one extra part.
        let extra = e.mem.alloc(8);
        e.mem.set_u32(extra, 0x0ee);
        let parts = e.mem.alloc(8);
        e.mem.set_u32(parts, 0x0aa);
        e.register_double(0x0060_2170, move |_, _| ret(parts));
        e.register_double(0x0062_4700, move |_, a| {
            ret(if a[0] == 0x0aa { extra } else { 0 })
        });
        let added = Rc::new(RefCell::new(vec![]));
        let seen = added.clone();
        e.register_double(LIST_ADD_005AE3D0, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        // No race eyes: the loaded eyes are searched by texture.
        e.register(EYE_LIST, |_, _| ret(0));
        let first = e.mem.alloc(0x40);
        let second = e.mem.alloc(0x40);
        let loaded = list(&mut e, &[0, first, second]);
        e.register_double(0x0043_c490, move |_, _| ret(loaded));
        e.register(PATH_TEXT, |_, a| ret(a[0]));
        e.register_double(0x0040_4dc0, move |_, a| {
            ret(u32::from(a[0] != second + 0x24))
        });
        e.call(0x0061_41f0, &args![r, npc, data, 0u32, 0u32]);
        assert_eq!(calls_to(&e, 0x0060_3ad0), vec![vec![npc, data]]);
        assert_eq!(e.mem.u8(data + 0xd5), 1);
        assert_eq!(calls_to(&e, 0x005f_dfa0), vec![vec![0x3b50, npc]]);
        assert_eq!(e.mem.u32(data + 0x80), 0x3b50);
        assert_eq!(e.mem.f32(data + 0x88), 1.5);
        assert_eq!(e.mem.u32(data + 0x84), 0x69d0);
        assert_eq!(e.mem.u32(data + 0x8c), second);
        assert_eq!(e.mem.u32(data + 0x90), 1);
        assert_eq!(calls_to(&e, 0x0060_44e0).len(), 8);
        assert_eq!(calls_to(&e, 0x0060_44e0)[3][1], 3);
        assert_eq!(e.mem.u8(data + 0xd4), 1);
        assert_eq!(
            *added.borrow(),
            vec![(data + 0xe4, 0x0aa), (data + 0xe4, 0x0ee)]
        );
        // The sex the NPC gives (1) picks the models.
        assert_eq!(e.mem.u32(data + 0xd8), r + 0xb0 + 0xc0 + 6 * 0x18);
    }

    #[test]
    fn the_players_own_face_keeps_the_race_hair_and_no_parts() {
        let mut e = face_engine();
        let r = e.mem.alloc(0x510);
        let data = e.mem.alloc(0x100);
        e.mem.set_u32(data + 0x8c, 1);
        e.register(0x0060_47a0, |_, _| ret(1));
        e.register(0x0060_3b50, |_, _| ret(0x3b50));
        e.call(0x0061_41f0, &args![r, 0x0777u32, data, 0u32, 1u32]);
        assert!(calls_to(&e, 0x005f_dfa0).is_empty());
        assert_eq!(e.mem.u32(data + 0x80), 0x3870 + 1);
        assert!(calls_to(&e, 0x0060_2170).is_empty());
        // `skip` set: no part holders.
        assert!(calls_to(&e, 0x0060_44e0).is_empty());
    }

    #[test]
    fn the_face_nodes_are_built_and_marked() {
        let callees = [
            0x0040_4eb0,
            0x0040_4ee0,
            0x00a5_b950,
            0x0043_81b0,
            0x005d_d590,
            0x005d_d6f0,
            0x0065_4720,
            0x0065_5fa0,
        ];
        let mut e = face_engine();
        for c in callees {
            e.register(c, |_, _| Ret::default());
        }
        e.register(RACE_FACE_COORD, |_, _| ret(0x0c00));
        e.register(EYE_LIST, |_, _| ret(0));
        e.register(0x0043_8170, |_, a| ret(a[1]));
        // The allocator hands out blocks; the constructors answer them.
        e.register(0x00aa_13e0, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x0064_9680, |_, a| ret(a[0]));
        e.register(0x0066_28c0, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            ret(a[0])
        });
        e.register(FORM_ID, |_, _| ret(7));
        let r = e.mem.alloc(0x510);
        let outs = e.mem.alloc(8);
        let result = e.call(
            0x0061_3c50,
            &args![r, outs, outs + 4, 0x0777u32, 1u32, 0x1ffu32, 0u32],
        );
        assert!(result.bool());
        let biped = e.mem.u32(outs);
        let skinned = e.mem.u32(outs + 4);
        assert_ne!(biped, 0);
        assert_ne!(skinned, 0);
        assert_eq!(
            calls_to(&e, 0x00a5_b950),
            vec![vec![biped, 0x0102_0408], vec![skinned, 0x0102_03f0]]
        );
        let object = calls_to(&e, 0x0064_9680)[0][0];
        assert_eq!(
            calls_to(&e, slot(0x104)),
            vec![vec![biped, object], vec![skinned, object]]
        );
        assert_eq!(
            calls_to(&e, slot(0x114)),
            vec![vec![biped, 1], vec![skinned, 0]]
        );
        assert_eq!(
            calls_to(&e, slot(0x11c)),
            vec![vec![biped, 1], vec![skinned, 0]]
        );
        let built = calls_to(&e, 0x0065_4720);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][0], outs);
        assert_eq!(built[0][1], outs + 4);
        assert_eq!(built[0][3], 0xff);
        assert_eq!(calls_to(&e, 0x0065_5fa0).len(), 2);
        assert_eq!(e.mem.u8(biped + 0xe1), 1);
        assert_eq!(e.mem.u8(skinned + 0xe1), 1);
        assert_eq!(
            calls_to(&e, 0x0040_4eb0)[0][1..],
            [0x37, 1, 0x0104_b860, 0x623]
        );
        assert_eq!(calls_to(&e, 0x0040_4ee0).len(), 1);
    }

    #[test]
    fn egt_data_is_dropped_for_models_with_paths() {
        let callees = [
            STRING_INIT,
            STRING_FREE,
            STRING_COPY,
            STRING_APPEND,
            EGT_PATH,
            0x0065_0780,
        ];
        let mut e = engine(&callees);
        e.register(HELD, |_, _| ret(0x0e67));
        e.register(0x0065_2110, |_, _| ret(0xcac4e));
        let r = e.mem.alloc(0x510);
        let empty = e.mem.alloc(4);
        let named = e.mem.alloc(4);
        e.mem.set_u8(named, b'm');
        // Male part 0 and female part 0 have paths; the female check
        // answers true for part 0 only.
        for part in 0..8 {
            for sex in 0..2 {
                let model = r + 0xb0 + sex * 0xc0 + part * 0x18;
                e.mem.set_u32(model, VTABLE);
            }
        }
        e.register_double(slot(0x14), move |_, a| {
            ret(if (a[0] - r - 0xb0) % 0xc0 == 0 {
                named
            } else {
                empty
            })
        });
        let male0 = r + 0xb0;
        e.register_double(slot(0xc), move |_, a| ret(u32::from(a[1] == male0)));
        e.call(0x0061_3fd0, &args![r]);
        let drops = calls_to(&e, 0x0065_0780);
        assert_eq!(drops, vec![vec![0xcac4e, 0x0e67], vec![0xcac4e, 0x0e67]]);
        assert_eq!(calls_to(&e, slot(0xc)), vec![vec![r + 0xb0 + 0xc0, male0]]);
        assert_eq!(calls_to(&e, STRING_INIT).len(), 1);
        assert_eq!(calls_to(&e, STRING_FREE).len(), 1);
        assert_eq!(calls_to(&e, STRING_COPY)[0][2], MESHES_PREFIX);
    }

    fn texture_engine(index: u32, present: &'static [u32]) -> Engine {
        let mut e = engine(&[STRING_COPY, STRING_APPEND]);
        e.register_double(0x0061_5300, move |_, _| ret(index));
        e.register(GET_SEX, |_, _| ret(1));
        e.register_double(0x0048_cee0, move |_, a| {
            ret(u32::from(
                present.contains(&((a[0] - 0x2f0 - 0x24) % 0x100 / 0xc)),
            ))
        });
        e.register(PATH_TEXT, |_, a| ret(a[0]));
        e.map(PLAYER, 4);
        e.register(GET_BASE_FORM, |_, _| ret(0x0777));
        e.register(0x00ec_6e30, |_, _| ret(0x0e47));
        e.register(0x0040_ab30, |_, _| ret(0x0500_0000));
        e.map(0x0500_0000, 4);
        e.mem.set_u8(0x0500_0000, b'.');
        e
    }

    #[test]
    fn body_texture_paths() {
        // Index 3 is out of range.
        let mut e = texture_engine(3, &[0, 1, 2]);
        let path: Ptr = Ptr::new(0x0600_0000);
        assert!(!e
            .call(0x0061_4640, &args![0u32, 0x0555u32, 0u32, path])
            .bool());
        // Index 2 without a texture falls back to 1; the player's base form
        // gets the `_1st` name.
        let mut e = texture_engine(2, &[1]);
        assert!(e
            .call(0x0061_4640, &args![0u32, 0x0777u32, 0u32, path])
            .bool());
        let texture = 0x2f0 + 0x24 + 0xc;
        assert_eq!(
            calls_to(&e, STRING_APPEND),
            vec![
                vec![path.addr(), 0x104, texture],
                vec![path.addr(), 0x104, 0x0104_b8c0],
                vec![path.addr(), 0x104, 0x0e47],
            ]
        );
        assert_eq!(e.mem.u8(0x0500_0000), 0);
        assert_eq!(
            calls_to(&e, STRING_COPY),
            vec![vec![path.addr(), 0x104, TEXTURES_PREFIX]]
        );
        // Another NPC: just the texture path.
        let mut e = texture_engine(0, &[0]);
        assert!(e
            .call(0x0061_4640, &args![0u32, 0x0888u32, 0u32, path])
            .bool());
        assert_eq!(calls_to(&e, STRING_APPEND).len(), 1);
        // No texture: false.
        let mut e = texture_engine(0, &[1]);
        assert!(!e
            .call(0x0061_4640, &args![0u32, 0x0888u32, 0u32, path])
            .bool());
    }

    #[test]
    fn body_mod_texture_names() {
        let mut e = engine(&[FORMAT, 0x00ec_623a]);
        // The NPC 0x10 has template 0x20, which has template 0x30.
        e.register(0x0060_44c0, |_, a| {
            ret(match a[0] {
                0x10 => 0x20,
                0x20 => 0x30,
                _ => 0,
            })
        });
        e.register(FORM_ID, |_, a| ret(0xff00_0000 | a[0]));
        e.register(GET_SEX, |_, _| ret(1));
        e.map(0x40, 0x40);
        e.mem.set_u32(0x40, VTABLE);
        e.register(slot(0x2c), |_, _| ret(0));
        let name: Ptr = Ptr::new(0x0600_0000);
        e.call(0x0061_47e0, &args![0x0abcu32, 0x10u32, name]);
        assert_eq!(calls_to(&e, 0x00ec_623a)[0][1], 0x0104_a0ec);
        let formats = calls_to(&e, FORMAT);
        assert_eq!(&formats[0][..4], &[name.addr(), 0x104, 0x0104_b8c8, 0x30]);
        e.register(GET_SEX, |_, _| ret(0));
        e.register(slot(0x2c), |_, _| ret(1));
        e.call(0x0061_47e0, &args![0x0abcu32, 0x10u32, name]);
        assert_eq!(calls_to(&e, 0x00ec_623a)[1][1], 0x0104_a0f4);
        let formats = calls_to(&e, FORMAT);
        assert_eq!(
            &formats[1][..5],
            &[name.addr(), 0x104, 0x0104_b8d8, 0x0abc, 0x30]
        );
    }

    #[test]
    fn body_mod_texture_file_names() {
        let mut e = engine(&[FORMAT]);
        e.register(0x0060_44c0, |_, _| ret(0));
        e.register(0x0048_4ee0, |_, a| {
            ret(if a[0] == 0x10 { 0 } else { 0x0f11e })
        });
        e.register(0x0089_1170, |_, _| ret(0x0a4e));
        let name: Ptr = Ptr::new(0x0600_0000);
        let path: Ptr = Ptr::new(0x0600_1000);
        assert!(!e
            .call(0x0061_48f0, &args![0u32, 0x10u32, name, path])
            .bool());
        e.register(0x0045_64f0, |_, _| ret(0));
        assert!(e
            .call(0x0061_48f0, &args![0u32, 0x11u32, name, path])
            .bool());
        assert_eq!(
            calls_to(&e, 0x0045_64f0),
            vec![vec![0x0a4e, 0x0104_a714, 6]]
        );
        e.register(0x0045_64f0, |_, _| ret(1));
        assert!(e
            .call(0x0061_48f0, &args![0u32, 0x11u32, name, path])
            .bool());
        let formats = calls_to(&e, FORMAT);
        assert_eq!(
            formats[0],
            vec![path.addr(), 0x104, 0x0104_b8ec, 0x0104_a704, name.addr()]
        );
        assert_eq!(
            formats[1],
            vec![path.addr(), 0x104, 0x0104_b8ec, 0x0a4e, name.addr()]
        );
    }

    const BODY_CALLEES: [u32; 26] = [
        HOLDER_INIT,
        HOLDER_SET,
        HOLDER_RELEASE,
        0x00ec_782f,
        0x00ec_5fce,
        STRING_INIT,
        STRING_FREE,
        0x0045_68c0,
        STRING_COPY,
        STRING_APPEND,
        FORMAT,
        0x00ec_623a,
        PRINT_ERROR,
        0x00a6_1b90,
        EGT_PATH,
        0x0065_1b50,
        0x0060_1800,
        0x0065_2af0,
        COPY_FACE_COORD,
        0x00a6_1fb0,
        0x0061_51d0,
        0x0065_b410,
        0x0043_8170,
        0x00a5_fba0,
        0x0043_81b0,
        0x00a6_1c50,
    ];

    /// A body-texture setup: holders hold what was last set into them
    /// (`HOLDER_SET` stores the word, `HELD` reads it).
    fn body_engine(path_found: bool, switch: u8, mod_failed: bool) -> (Engine, u32, u32) {
        let mut e = engine(&BODY_CALLEES);
        e.register(HOLDER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(HELD, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0060_45f0, |_, _| ret(0x0de7));
        e.map(PLAYER, 4);
        e.register(GET_BASE_FORM, |_, _| ret(0x0777));
        e.register(GET_SEX, |_, _| ret(0));
        e.register_double(0x0061_5300, move |_, _| ret(if path_found { 0 } else { 5 }));
        e.register(0x0048_cee0, |_, _| ret(1));
        e.register(PATH_TEXT, |_, _| ret(0x0aaa));
        // Loading the body texture (flag 0) stores 0x1111 into its holder;
        // the body-mod load (flag 1) leaves its holder as it was.
        e.register(0x0045_68c0, |e, a| {
            if a[3] == 0 {
                e.mem.set_u32(a[2], 0x1111);
            }
            Ret::default()
        });
        e.map(0x011d_ea10, 4);
        e.register(STRING_TEXT, |_, _| ret(0x0500_0000));
        e.map(0x0500_0000, 4);
        e.mem.set_u8(0x0500_0000, switch);
        e.register(0x0060_44c0, |_, _| ret(0));
        e.register(0x0048_4ee0, |_, _| ret(0x0f11e));
        e.register(0x0089_1170, |_, _| ret(0x0a4e));
        e.register(0x0045_64f0, |_, _| ret(1));
        e.register(FORM_ID, |_, _| ret(0x0abc));
        e.register_double(0x0082_2510, move |_, _| ret(u32::from(mod_failed)));
        e.register(EGT_PATH, |_, _| ret(0x0e67));
        e.register(0x0065_1b50, |_, _| ret(0x0e5e));
        e.register(RACE_FACE_COORD, |_, _| ret(0x0c00));
        e.register(0x0060_1800, |_, _| ret(0x0b1e));
        // Applying the coordinates makes the texture.
        e.register(0x0065_b410, |e, a| {
            e.mem.set_u32(a[2], 0x2222);
            Ret::default()
        });
        e.register(0x0043_8170, |_, _| ret(0x0f17));
        // The palette has no such texture.
        e.register(0x00a6_1b90, |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        let race = object(&mut e, 0x510);
        e.mem.set_u32(race + 0x3c8, VTABLE);
        let npc = e.mem.alloc(0x40);
        e.mem.set_u32(npc + 0x30, VTABLE);
        (e, race, npc)
    }

    fn body(e: &mut Engine, race: u32, npc: u32, from_scratch: u32) -> (bool, u32, u32) {
        let holders = e.mem.alloc(8);
        let result = e
            .call(
                0x0061_49b0,
                &args![race, holders, holders + 4, npc, 0x4u32, from_scratch],
            )
            .bool();
        (result, e.mem.u32(holders), e.mem.u32(holders + 4))
    }

    fn cleaned_up(e: &Engine) -> bool {
        calls_to(e, STRING_FREE).len() == 1
            && calls_to(e, 0x00ec_5fce).len() == 1
            && calls_to(e, HOLDER_RELEASE).len() == 4
    }

    #[test]
    fn a_body_texture_needs_a_path() {
        let (mut e, race, npc) = body_engine(false, 1, false);
        let (result, texture, modulation) = body(&mut e, race, npc, 0);
        assert!(!result);
        assert_eq!(texture, 0);
        assert_eq!(modulation, 0x0de7);
        assert!(cleaned_up(&e));
        assert_eq!(calls_to(&e, HOLDER_INIT).len(), 4);
    }

    #[test]
    fn without_facegen_the_plain_texture_is_enough() {
        let (mut e, race, npc) = body_engine(true, 0, false);
        let (result, texture, _) = body(&mut e, race, npc, 0);
        assert!(result);
        assert_eq!(texture, 0x1111);
        assert!(calls_to(&e, FORMAT).is_empty());
        assert!(cleaned_up(&e));
    }

    #[test]
    fn a_loaded_body_mod_texture_is_used() {
        let (mut e, race, npc) = body_engine(true, 1, false);
        let (result, _, modulation) = body(&mut e, race, npc, 0);
        assert!(result);
        // The default modulation texture stays (the double loads nothing).
        assert_eq!(modulation, 0x0de7);
        assert_eq!(calls_to(&e, 0x0045_68c0).len(), 2);
        assert!(calls_to(&e, PRINT_ERROR).is_empty());
        assert!(cleaned_up(&e));
    }

    #[test]
    fn a_missing_body_mod_texture_is_built() {
        let (mut e, race, npc) = body_engine(true, 1, true);
        let (result, _, modulation) = body(&mut e, race, npc, 0);
        assert!(result);
        assert_eq!(modulation, 0x2222);
        assert_eq!(calls_to(&e, PRINT_ERROR)[0][0], 0x0104_b918);
        assert_eq!(calls_to(&e, 0x00a6_1b90).len(), 1);
        assert_eq!(
            calls_to(&e, 0x0065_1b50),
            vec![vec![0, 0, 0, 0x0e67, 1, 0xffff_ffff, 0]]
        );
        let blend = calls_to(&e, 0x0065_2af0);
        assert_eq!(blend[0][0], 0x0c00);
        assert_eq!(blend[0][1], 0x0b1e);
        assert_eq!(&blend[0][3..], &[0, 0]);
        assert_eq!(calls_to(&e, 0x00a5_fba0), vec![vec![0x2222, 0x0f17]]);
        assert_eq!(calls_to(&e, 0x00a6_1c50), vec![vec![0x2222, 0]]);
        assert!(calls_to(&e, 0x00a6_1fb0).is_empty());
        assert!(cleaned_up(&e));
    }

    #[test]
    fn from_scratch_for_the_race_alone() {
        let (mut e, race, _) = body_engine(true, 1, false);
        // The body-mod name reads the (null) NPC's +0x30 vtable.
        e.map(0, 0x40);
        e.mem.set_u32(0x30, VTABLE);
        let (result, _, modulation) = body(&mut e, race, 0, 1);
        assert!(result);
        assert_eq!(modulation, 0x2222);
        // No message, no palette lookup, the race's coordinates copied and
        // the stale palette entry removed.
        assert!(calls_to(&e, PRINT_ERROR).is_empty());
        assert!(calls_to(&e, 0x00a6_1b90).is_empty());
        assert_eq!(calls_to(&e, COPY_FACE_COORD)[0][0], 0x0c00);
        assert_eq!(calls_to(&e, 0x00a6_1fb0).len(), 1);
        assert!(cleaned_up(&e));
    }

    #[test]
    fn a_palette_texture_ends_the_build() {
        let (mut e, race, npc) = body_engine(true, 1, true);
        e.register(0x00a6_1b90, |e, a| {
            e.mem.set_u32(a[1], 0x3333);
            Ret::default()
        });
        let (result, _, modulation) = body(&mut e, race, npc, 0);
        assert!(result);
        assert_eq!(modulation, 0x3333);
        assert!(calls_to(&e, 0x0065_1b50).is_empty());
        assert!(cleaned_up(&e));
    }

    // TESRace::Load.

    const ELEMENTS: u32 = 0x0700_0000;

    type Chunks = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;

    const LOAD_CALLEES: [u32; 20] = [
        TES_FORM_LOAD_FORM,
        SWAP_WORD,
        TES_FORM_LOAD_DATA,
        ADD_COMPILE_INDEX,
        FREE,
        MEMSET,
        LOAD_TEXTURE_CHUNK,
        LOAD_MODEL_CHUNK,
        VECTOR_RESIZE,
        PRINT_ERROR,
        LIST_ADD_00905820,
        0x0048_7050,
        0x0048_2d50,
        0x0048_d290,
        0x005a_6b10,
        0x0048_0130,
        GET_CHUNK_DATA_OV2,
        0x0061_1db0,
        0x0048_9100,
        0x0071_7e50,
    ];

    /// A record double: each chunk has a signature and its bytes; the
    /// reads copy the bytes (at most the size asked for).
    fn load_engine(form_type: u32, chunks: &[(u32, &[u8])], swaps: bool) -> (Engine, u32, Ptr) {
        let mut e = engine(&LOAD_CALLEES);
        let queue: Chunks = Rc::new(RefCell::new(
            chunks.iter().map(|(s, d)| (*s, d.to_vec())).collect(),
        ));
        e.register_double(GET_TES_FORM, move |_, _| ret(form_type));
        let q = queue.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(q.borrow().first().map_or(0, |c| c.0))
        });
        let q = queue.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            ret(q.borrow().first().map_or(0, |c| c.1.len() as u32))
        });
        let q = queue.clone();
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let data = q.borrow().first().map(|c| c.1.clone()).unwrap_or_default();
            let n = (a[2] as usize).min(data.len());
            e.mem.write(a[1], &data[..n]);
            Ret::default()
        });
        let q = queue.clone();
        e.register_double(GET_CHUNK_WORD, move |e, a| {
            let data = q.borrow().first().map(|c| c.1.clone()).unwrap_or_default();
            e.mem.write(a[1], &data[..4.min(data.len())]);
            Ret::default()
        });
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut q = queue.borrow_mut();
            q.remove(0);
            ret(u32::from(!q.is_empty()))
        });
        e.register_double(FILE_SWAPS, move |_, _| ret(u32::from(swaps)));
        let race = object(&mut e, 0x510);
        let file: Ptr = Ptr::new(e.mem.alloc(0x300));
        (e, race, file)
    }

    fn load(e: &mut Engine, race: u32, file: Ptr) -> bool {
        e.call(0x0061_0cd0, &args![race, file]).bool()
    }

    #[test]
    fn load_needs_a_race_record() {
        let (mut e, race, file) = load_engine(0xb, &[(CHUNK_DATA, &[0; 0x24])], false);
        assert!(!load(&mut e, race, file));
        assert!(calls_to(&e, TES_FORM_LOAD_FORM).is_empty());
    }

    #[test]
    fn load_reads_the_simple_chunks() {
        let chunks: [(u32, &[u8]); 13] = [
            (CHUNK_EDID, b"Human\0"),
            (CHUNK_OBND, &[0; 12]),
            (CHUNK_FULL, b"x\0"),
            (CHUNK_DESC, &[0; 4]),
            (CHUNK_SPLO, &[0x34, 0x12, 0, 0]),
            (CHUNK_DATA, &[0; 0x24]),
            (CHUNK_ONAM, &[1, 0, 0, 0]),
            (CHUNK_YNAM, &[2, 0, 0, 0]),
            (CHUNK_PNAM, &[3, 0, 0, 0]),
            (CHUNK_UNAM, &[4, 0, 0, 0]),
            (CHUNK_SNAM, &[5, 0]),
            (CHUNK_CNAM, &[7, 8]),
            (CHUNK_DNAM, &[9, 0, 0, 0, 10, 0, 0, 0]),
        ];
        let (mut e, race, file) = load_engine(0xc, &chunks, false);
        let text = Rc::new(RefCell::new(vec![]));
        let seen = text.clone();
        e.register_double(slot(0x134), move |e, a| {
            *seen.borrow_mut() = e.mem.cstr(a[1]);
            Ret::default()
        });
        assert!(load(&mut e, race, file));
        let f = file.addr();
        assert_eq!(calls_to(&e, TES_FORM_LOAD_FORM), vec![vec![race, f]]);
        assert_eq!(*text.borrow(), b"Human");
        assert_eq!(calls_to(&e, slot(0xe0)), vec![vec![race, f]]);
        assert_eq!(calls_to(&e, 0x0048_7050), vec![vec![race + 0x18, f]]);
        assert_eq!(calls_to(&e, 0x0048_2d50), vec![vec![race + 0x24, f]]);
        assert_eq!(calls_to(&e, 0x0048_d290), vec![vec![race + 0x2c, 0x1234]]);
        assert_eq!(
            calls_to(&e, TES_FORM_LOAD_DATA),
            vec![vec![race, f, race + 0x50, 0x24]]
        );
        assert!(calls_to(&e, 0x0061_1db0).is_empty());
        assert_eq!(e.mem.u32(race + 0x504), 1);
        assert_eq!(e.mem.u32(race + 0x508), 2);
        assert_eq!(e.mem.u32(race + 0xa0), 3);
        assert_eq!(e.mem.u32(race + 0xa4), 4);
        assert_eq!(
            calls_to(&e, GET_CHUNK_DATA_OV2),
            vec![vec![f, race + 0x4f8]]
        );
        assert_eq!(e.mem.u8(race + 0x9c), 7);
        assert_eq!(e.mem.u8(race + 0x9d), 8);
        assert_eq!(e.mem.u32(race + 0x94), 9);
        assert_eq!(e.mem.u32(race + 0x98), 10);
    }

    #[test]
    fn load_swaps_and_reads_the_other_layouts() {
        let chunks: [(u32, &[u8]); 6] = [
            (CHUNK_DATA, &[0; 0x24]),
            (CHUNK_VTCK, &[1, 0, 0, 0, 2, 0, 0, 0]),
            (CHUNK_VTCK, &[0; 4]),
            (CHUNK_CNAM, &[5]),
            (CHUNK_SNAM, &[5]),
            (CHUNK_XNAM, &[0; 12]),
        ];
        let (mut e, race, file) = load_engine(0xc, &chunks, true);
        e.map(0x011c_b298, 8);
        e.mem.set_u32(0x011c_b298, 0xaaa);
        e.mem.set_u32(0x011c_b29c, 0xbbb);
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x0071_7e50, |_, a| ret(a[0] + 1));
        assert!(load(&mut e, race, file));
        assert_eq!(calls_to(&e, 0x0061_1db0), vec![vec![race + 0x50]]);
        // The second VTCK (4 bytes) stored the defaults over the first.
        assert_eq!(e.mem.u32(race + 0x4fc), 0xaaa);
        assert_eq!(e.mem.u32(race + 0x500), 0xbbb);
        assert_eq!(calls_to(&e, SWAP_WORD).len(), 2);
        // CNAM of one byte: two 2-byte reads at +0x9c and +0x9d.
        let reads = calls_to(&e, GET_CHUNK_DATA);
        assert!(reads.contains(&vec![file.addr(), race + 0x9c, 2]));
        assert!(reads.contains(&vec![file.addr(), race + 0x9d, 2]));
        // SNAM of one byte: nothing.
        assert!(calls_to(&e, GET_CHUNK_DATA_OV2).is_empty());
        let block = calls_to(&e, 0x005a_6b10)[0][0];
        let adds = calls_to(&e, LIST_ADD_00905820);
        assert_eq!(adds[0][0], race + 0x41);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0xc]]);
        assert_ne!(block, 0);
    }

    #[test]
    fn load_reads_hair_and_eye_lists() {
        let chunks: [(u32, &[u8]); 3] = [
            (CHUNK_HNAM, &[1, 0, 0, 0, 2, 0, 0, 0]),
            (CHUNK_ENAM, &[3, 0, 0, 0]),
            (CHUNK_ENAM, &[3, 0, 0]),
        ];
        let (mut e, race, file) = load_engine(0xc, &chunks, false);
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(LOOKUP_FORM, |_, a| ret(a[0] << 8));
        e.register(RT_DYNAMIC_CAST, |_, a| {
            ret(if a[0] == 0x200 { 0 } else { a[0] })
        });
        e.register(FORM_ID, |e, a| {
            ret(if a[0] == 0x0300 || a[0] == 0x0100 {
                a[0]
            } else {
                e.mem.u32(a[0] + 0xc)
            })
        });
        e.register(slot(0x130), |_, _| ret(0x0aae));
        e.register(HAIR_LIST, |_, _| ret(0));
        e.register(EYE_LIST, |_, _| ret(0));
        assert!(load(&mut e, race, file));
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![8], vec![4]]);
        let adds = calls_to(&e, LIST_ADD_00905820);
        assert_eq!(adds.len(), 2);
        let casts = calls_to(&e, RT_DYNAMIC_CAST);
        assert_eq!(casts[0][3], 0x0118_63a0);
        assert_eq!(casts[2][3], 0x0118_6388);
        let errors = calls_to(&e, PRINT_ERROR);
        assert_eq!(errors.len(), 1);
        assert_eq!(&errors[0][..3], &[0x0104_b658, 2, 0x0aae]);
        assert_eq!(calls_to(&e, FREE).len(), 2);
        assert_eq!(calls_to(&e, ADD_COMPILE_INDEX).len(), 3);
    }

    #[test]
    fn load_reads_attributes_and_face_geometry() {
        let chunks: [(u32, &[u8]); 5] = [
            (CHUNK_ATTR, &[1, 2, 3, 4, 5, 6]),
            (CHUNK_FNAM, &[]),
            (CHUNK_FGGS, &[0, 0, 0x80, 0x3f, 0, 0, 0, 0x40]),
            (CHUNK_NAM2, &[]),
            (CHUNK_FGTS, &[1, 0, 0x80, 0x7f]),
        ];
        let (mut e, race, file) = load_engine(0xc, &chunks, false);
        e.register(0x009d_2510, |_, _| ret(3));
        // Element `i` of a vector lives at `ELEMENTS + 0x10 * (vector % 0x1000) + 4 * i`.
        e.map(ELEMENTS, 0x10000);
        e.register(VECTOR_AT, |e, a| {
            e.mem
                .set_u32(a[1], ELEMENTS + 0x10 * (a[0] % 0x1000) + 4 * a[2]);
            ret(a[1])
        });
        e.register(ITERATOR_ELEMENT, |e, a| ret(e.mem.u32(a[0])));
        let at = |vector: u32, i: u32| ELEMENTS + 0x10 * (vector % 0x1000) + 4 * i;
        assert!(load(&mut e, race, file));
        let attributes = calls_to(&e, 0x0048_0130);
        assert_eq!(attributes[0][0], race + 0x74);
        assert_eq!(attributes[1], vec![race + 0x80, attributes[0][1] + 3]);
        // FGGS for a female with separate numbering: both vectors.
        let resizes = calls_to(&e, VECTOR_RESIZE);
        assert_eq!(
            resizes,
            vec![
                vec![race + 0x478, 2, 1],
                vec![race + 0x3f8, 2, 1],
                vec![race + 0x3f8 + 0x40, 1, 1]
            ]
        );
        assert_eq!(e.mem.f32(at(race + 0x478, 0)), 1.0);
        assert_eq!(e.mem.f32(at(race + 0x478, 1)), 2.0);
        assert_eq!(e.mem.f32(at(race + 0x3f8, 1)), 2.0);
        // FGTS after NAM2 for a female: only the second vector, the
        // signalling NaN quieted.
        assert_eq!(e.mem.u32(at(race + 0x3f8 + 0x40, 0)), 0x7fc0_0001);
        assert_eq!(e.mem.u32(at(race + 0x478 + 0x40, 0)), 0);
    }

    #[test]
    fn load_indexes_parts() {
        let chunks: [(u32, &[u8]); 10] = [
            (CHUNK_NAM1, &[]),
            (CHUNK_INDX, &[1, 0, 0, 0]),
            (0x4c444f4d, &[]),
            (CHUNK_INDX, &[3, 0, 0, 0]),
            (0x4e4f4349, &[]),
            (CHUNK_NAM0, &[]),
            (CHUNK_MNAM, &[]),
            (CHUNK_INDX, &[4, 0, 0, 0]),
            (0x4e4f4349, &[]),
            (0x4c444f4d, &[]),
        ];
        let (mut e, race, file) = load_engine(0xc, &chunks, false);
        e.register(IS_TEXTURE_CHUNK, |_, a| ret(u32::from(a[0] == 0x4e4f4349)));
        e.register(IS_MODEL_CHUNK, |_, a| ret(u32::from(a[0] == 0x4c444f4d)));
        e.map(0x0119_9fbc, 0x100);
        e.mem.set_u32(0x0119_a080 + 4, 0x0b0d);
        e.mem.set_u32(0x0119_9fbc, 0x0b0e);
        e.mem.set_u32(0x0119_9fe8 + 4 * 3, 0x4ead);
        e.mem.set_u32(0x0119_a028 + 4 * 3, 0x7e47);
        for model in [
            race + 0x338 + 0x18,
            race + 0x2f0 + 0xc,
            race + 0x3c8,
            race + 0xb0 + 3 * 0x18,
            race + 0x230 + 3 * 0xc,
        ] {
            e.mem.set_u32(model, VTABLE);
        }
        assert!(load(&mut e, race, file));
        let f = file.addr();
        // Body part 1: model and texture reset, then its model chunk.
        let defaults = calls_to(&e, slot(0x18));
        assert_eq!(
            defaults,
            vec![
                vec![race + 0x338 + 0x18, 0x0b0d],
                vec![race + 0x3c8, 0x0b0e],
                vec![race + 0xb0 + 3 * 0x18, 0x4ead],
            ]
        );
        let models = calls_to(&e, LOAD_MODEL_CHUNK);
        assert_eq!(
            models,
            vec![
                vec![race + 0x338 + 0x18, f],
                // Body index 3 always loads the body model.
                vec![race + 0x3c8, f],
                // Head part 4, shifted to 3.
                vec![race + 0xb0 + 3 * 0x18, f],
            ]
        );
        assert_eq!(
            calls_to(&e, LOAD_TEXTURE_CHUNK),
            vec![vec![race + 0x230 + 3 * 0xc, f]]
        );
        assert_eq!(
            calls_to(&e, 0x0048_9100),
            vec![vec![race + 0x230 + 3 * 0xc, 0x7e47]]
        );
    }
}

#[cfg(test)]
mod save_copy_compare_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    /// A race of 0x510 bytes; its hair list is at +0x8 and its eye list at
    /// +0x10 (two-word nodes: item, next) for the list doubles.
    fn race(e: &mut Engine) -> u32 {
        e.mem.alloc(0x510)
    }

    fn form(e: &mut Engine, id: u32) -> u32 {
        let form = e.mem.alloc(0x18);
        e.mem.set_u32(form + 0xc, id);
        form
    }

    fn list_doubles(e: &mut Engine) {
        e.register(HAIR_LIST, |_, a| ret(a[0] + 8));
        e.register(EYE_LIST, |_, a| ret(a[0] + 0x10));
        e.register(NODE_DATA, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_COUNT, |e, a| {
            let (mut node, mut n) = (a[0], 0);
            while node != 0 {
                n += u32::from(e.mem.u32(node) != 0);
                node = e.mem.u32(node + 4);
            }
            ret(n)
        });
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
    }

    fn set_list(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
    }

    #[test]
    fn data_swap_covers_five_words() {
        let mut e = Engine::new();
        quiet(&mut e, &[SWAP_WORD]);
        let (_, log) = run(&mut e, 0x0061_1db0, &args![0x4000u32]);
        let swapped: Vec<u32> = only(&log, SWAP_WORD).iter().map(|c| c[0]).collect();
        assert_eq!(swapped, vec![0x4010, 0x4018, 0x4014, 0x401c, 0x4020]);
        assert!(only(&log, SWAP_WORD).iter().all(|c| c[1] == 0));
    }

    fn save_engine() -> Engine {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                START_FORM,
                CLOSE_FORM,
                FULL_NAME_SAVE,
                DESCRIPTION_SAVE,
                SPELL_LIST_SAVE,
                REACTION_SAVE,
                SAVE_DATA,
                ADD_CHUNK_WORD,
                ADD_CHUNK_EMPTY,
                ADD_CHUNK_ARRAY,
                ADD_CHUNK_ARRAY_OV2,
                ADD_CHUNK_SHORT,
                MODEL_SAVE,
                MODEL_SAVE_FLAGS,
                TEXTURE_SAVE,
                SWAP_WORD,
                FREE,
            ],
        );
        list_doubles(&mut e);
        e.register(SAVE_SWAPS_BYTES, |_, _| ret(0));
        e.register(ATTRIBUTE_COUNT, |_, _| ret(7));
        e.register(ATTRIBUTE_COPY, |e, a| {
            for i in 0..7 {
                e.mem.set_u8(a[1] + i, (a[0] & 0xff) as u8);
            }
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0].clamp(4, 0x100))));
        e.register(MEMSET, |_, _| Ret::default());
        e.register(VECTOR_SIZE, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        e.register(VECTOR_AT, |e, a| {
            let data = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], data + 4 * a[2]);
            ret(a[1])
        });
        e.register(ITERATOR_ELEMENT, |e, a| ret(e.mem.u32(a[0]) + 4 * a[1]));
        e
    }

    #[test]
    fn save_writes_the_chunks_in_order() {
        let mut e = save_engine();
        let mut words = vec![];
        let this = race(&mut e);
        let older = form(&mut e, 0x111);
        e.mem.set_u32(this + 0x504, older);
        let voice = form(&mut e, 0x222);
        e.mem.set_u32(this + 0x500, voice);
        e.mem.set_u16(this + 0x9c, 0x0102);
        e.mem.set_u32(this + 0xa0, 0x7f80_0001);
        e.mem.set_u32(this + 0xa4, 2.0f32.to_bits());
        e.mem.set_u16(this + 0x4f8, 0x55);
        let hair = form(&mut e, 0x333);
        set_list(&mut e, this + 8, &[hair]);
        let values = e.mem.alloc(8);
        e.mem.set_u32(values, 1.0f32.to_bits());
        e.mem.set_u32(values + 4, 2.0f32.to_bits());
        let vector = this + 0x478;
        e.mem.set_u32(vector, values);
        e.mem.set_u32(vector + 0x18, 2);
        let seen = e.mem.alloc(0x40);
        e.register_double(ADD_CHUNK_ARRAY_OV2, move |e, a| {
            if a[0] == CHUNK_VTCK || a[0] == CHUNK_FGGS || a[0] == CHUNK_HNAM {
                let n = e.mem.u32(seen + 0x3c);
                for i in 0..a[2] {
                    let word = e.mem.u32(a[1] + 4 * i);
                    e.mem.set_u32(seen + 4 * (n + i), word);
                }
                e.mem.set_u32(seen + 0x3c, n + a[2]);
            }
            Ret::default()
        });
        let (_, log) = run(&mut e, 0x0061_1e50, &args![this]);
        for c in only(&log, ADD_CHUNK_WORD) {
            words.push((c[0], c[1]));
        }
        assert_eq!(words[0], (CHUNK_ONAM, 0x111));
        assert_eq!(words[1], (CHUNK_PNAM, 0x7fc0_0001));
        assert_eq!(words[2], (CHUNK_UNAM, 2.0f32.to_bits()));
        assert_eq!(words.iter().filter(|w| w.0 == CHUNK_INDX).count(), 16 + 8);
        let arrays = only(&log, ADD_CHUNK_ARRAY_OV2);
        let tags: Vec<u32> = arrays.iter().map(|c| c[0]).collect();
        assert_eq!(tags, vec![CHUNK_VTCK, CHUNK_HNAM, CHUNK_ENAM, CHUNK_FGGS]);
        assert_eq!(
            (0..5).map(|i| e.mem.u32(seen + 4 * i)).collect::<Vec<_>>(),
            vec![0, 0x222, 0x333, 1.0f32.to_bits(), 2.0f32.to_bits()]
        );
        let plain = only(&log, ADD_CHUNK_ARRAY);
        assert_eq!(plain[0], vec![CHUNK_CNAM, this + 0x9c, 2]);
        assert_eq!((plain[1][0], plain[1][2]), (CHUNK_ATTR, 2));
        assert_eq!(e.mem.u8(plain[1][1] + 7), ((this + 0x80) & 0xff) as u8);
        assert_eq!(
            only(&log, ADD_CHUNK_SHORT),
            vec![vec![CHUNK_SNAM, 0x55], vec![CHUNK_SNAM, 0x55]]
        );
        let empty: Vec<u32> = only(&log, ADD_CHUNK_EMPTY).iter().map(|c| c[0]).collect();
        assert_eq!(
            empty,
            vec![
                CHUNK_NAM2, CHUNK_NAM0, CHUNK_MNAM, CHUNK_FNAM, CHUNK_NAM1, CHUNK_MNAM, CHUNK_FNAM,
                CHUNK_MNAM, CHUNK_FNAM
            ]
        );
        assert_eq!(only(&log, MODEL_SAVE).len(), 16 + 6 + 2);
        assert_eq!(
            only(&log, MODEL_SAVE)[16],
            vec![this + 0x338, CHUNK_MODL, CHUNK_MODT]
        );
        assert_eq!(only(&log, TEXTURE_SAVE).len(), 16 + 6);
        assert_eq!(only(&log, FREE).len(), 3);
    }

    fn copy_engine() -> Engine {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        quiet(
            &mut e,
            &[COPY_ALL_COMPONENTS, SET_TEXTURE_NAME, COPY_FACE_COORD],
        );
        e.register(VOICE_TYPE, |e, a| ret(e.mem.u32(a[0] + 0x4fc + 4 * a[1])));
        e.register(FLOAT_60, |e, a| Ret {
            st0: x87_load32(e.mem.u32(a[0] + 0x60 + 4 * a[1])),
            ..Ret::default()
        });
        e.register(FLOAT_68, |e, a| Ret {
            st0: x87_load32(e.mem.u32(a[0] + 0x68 + 4 * a[1])),
            ..Ret::default()
        });
        e.register(ATTRIBUTES, |_, a| ret(a[0] + 0x74));
        e.register(WORD_70, |e, a| ret(e.mem.u32(a[0] + 0x70)));
        e.register(WORD_4F8, |e, a| ret(u32::from(e.mem.u16(a[0] + 0x4f8))));
        e.register(PATH_TEXT, |_, a| ret(a[0] + 1));
        e.register(LIST_CLEAR, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(LIST_ADD_00905820, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut node = a[0];
            if e.mem.u32(node) == 0 {
                e.mem.set_u32(node, item);
                return Ret::default();
            }
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let new = e.mem.alloc(8);
            e.mem.set_u32(new, item);
            e.mem.set_u32(node + 4, new);
            Ret::default()
        });
        e.register(RACE_FACE_COORD, |_, a| ret(a[0] + 0x100 + a[1]));
        e.map(DEFAULT_A0 & !0xfff, 0x1000);
        e.map(DEFAULT_A4 & !0xfff, 0x1000);
        e.map(ZERO_DOUBLE & !0xfff, 0x1000);
        e.set_global::<u32>(DEFAULT_A0, 5.0f32.to_bits());
        e.set_global::<u32>(DEFAULT_A4, 3.0f32.to_bits());
        e
    }

    /// Gives every model and texture component of `race` a vtable whose
    /// slots `+0xc` (compare: the two words differ) and `+0x14` (name: the
    /// component's address plus 1) and `+0x18` (set) are doubles.
    fn components(e: &mut Engine, race: u32) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, 0x0f4c_0008);
        e.mem.set_u32(vtable + 0xc, 0x0f4c_000c);
        e.mem.set_u32(vtable + 0x14, 0x0f4c_0014);
        e.mem.set_u32(vtable + 0x18, 0x0f4c_0018);
        e.register(0x0f4c_000c, |e, a| {
            ret(u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)))
        });
        e.register(0x0f4c_0014, |_, a| ret(a[0] + 1));
        e.register(0x0f4c_0008, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(0x0f4c_0018, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        let mut at = vec![0x74u32, 0x80, 0x3c8, 0x3e0];
        for sex in 0..2u32 {
            for part in 0..8u32 {
                at.push(0xb0 + sex * 0xc0 + part * 0x18);
                at.push(0x230 + sex * 0x60 + part * 0xc);
            }
            for part in 0..3u32 {
                at.push(0x2f0 + sex * 0x24 + part * 0xc);
                at.push(0x338 + sex * 0x48 + part * 0x18);
            }
        }
        for offset in at {
            e.mem.set_u32(race + offset, vtable);
        }
    }

    #[test]
    fn copy_takes_every_field_of_another_race() {
        let mut e = copy_engine();
        let this = race(&mut e);
        let other = race(&mut e);
        components(&mut e, this);
        components(&mut e, other);
        e.mem.set_u32(other + 0x504, 0x504);
        e.mem.set_u32(other + 0x500, 0x500);
        e.mem.set_u32(other + 0x98, 0x98);
        e.mem.set_u16(other + 0x9c, 0x0201);
        e.mem.set_u32(other + 0xa0, 0.0f32.to_bits());
        e.mem.set_u32(other + 0xa4, 4.0f32.to_bits());
        e.mem.set_u8(other + 0x5d, 0x77);
        e.mem.set_u32(other + 0x6c, 6.5f32.to_bits());
        e.mem.set_u32(other + 0x70, 0x70);
        e.mem.set_u16(other + 0x4f8, 0x4f8);
        let hair = form(&mut e, 0x333);
        set_list(&mut e, other + 8, &[hair]);
        set_list(&mut e, this + 8, &[0x999]);
        let (_, log) = run(&mut e, 0x0061_2c90, &args![this, other]);
        assert_eq!(e.mem.u32(this + 0x504), 0x504);
        assert_eq!(e.mem.u32(this + 0x500), 0x500);
        assert_eq!(e.mem.u32(this + 0x98), 0x98);
        assert_eq!(e.mem.u16(this + 0x9c), 0x0201);
        assert_eq!(e.mem.u32(this + 0xa0), 5.0f32.to_bits());
        assert_eq!(e.mem.u32(this + 0xa4), 4.0f32.to_bits());
        assert_eq!(e.mem.u8(this + 0x5d), 0x77);
        assert_eq!(e.mem.u32(this + 0x6c), 6.5f32.to_bits());
        assert_eq!(e.mem.u32(this + 0x70), 0x70);
        assert_eq!(e.mem.u16(this + 0x4f8), 0x4f8);
        assert_eq!(e.mem.u32(this + 8), hair);
        assert_eq!(e.mem.u32(this + 0x74 + 4), other + 0x74);
        assert_eq!(e.mem.u32(this + 0xb0 + 4), other + 0xb0 + 1);
        assert_eq!(e.mem.u32(this + 0x3e0 + 4), other + 0x3e0 + 1);
        let textures = only(&log, SET_TEXTURE_NAME);
        assert_eq!(textures.len(), 16 + 6);
        assert_eq!(textures[0], vec![this + 0x230, other + 0x230 + 1]);
        assert_eq!(
            only(&log, COPY_FACE_COORD),
            vec![
                vec![other + 0x100, this + 0x100, 0, 0],
                vec![other + 0x101, this + 0x101, 0, 0]
            ]
        );
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        let fresh = race(&mut e);
        e.call(0x0061_2c90, &args![fresh, other]);
        assert_eq!(e.mem.u32(fresh + 0x504), 0);
    }

    #[test]
    fn compare_finds_each_kind_of_difference() {
        let mut e = copy_engine();
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(0));
        e.register(MEMCMP, |e, a| {
            let differ = (0..a[2]).any(|i| e.mem.u8(a[0] + i) != e.mem.u8(a[1] + i));
            ret(u32::from(differ))
        });
        e.register(FACE_COORDS_DIFFER, |e, a| {
            ret(e.mem.u32(a[0]) ^ e.mem.u32(a[1]))
        });
        e.register(RANDOM, |_, _| ret(0x1_2345));
        let pair = |e: &mut Engine| {
            let this = race(e);
            let other = race(e);
            components(e, this);
            components(e, other);
            (this, other)
        };
        let differs =
            |e: &mut Engine, this: u32, other: u32| e.call(0x0061_31c0, &args![this, other]).bool();
        let (this, other) = pair(&mut e);
        assert!(!differs(&mut e, this, other));
        for (offset, value) in [
            (0x60u32, 1u32),
            (0x508, 1),
            (0x4fc, 1),
            (0x98, 1),
            (0x9d, 1),
            (0xa4, 9.0f32.to_bits()),
            (0x80 + 4, 1),
            (0xb0 + 0xc0 + 4, 1),
            (0x230 + 4, 1),
            (0x2f0 + 0x24 + 4, 1),
            (0x338 + 4, 1),
            (0x3e0 + 4, 1),
            (0x4f8, 1),
        ] {
            let (this, other) = pair(&mut e);
            e.mem.set_u32(other + offset, value);
            assert!(differs(&mut e, this, other), "offset {offset:#x}");
        }
        let (this, other) = pair(&mut e);
        let a = form(&mut e, 0x41);
        let b = form(&mut e, 0x42);
        set_list(&mut e, this + 0x10, &[a]);
        set_list(&mut e, other + 0x10, &[b]);
        assert!(differs(&mut e, this, other));
        set_list(&mut e, other + 0x10, &[a]);
        assert!(!differs(&mut e, this, other));
        set_list(&mut e, other + 0x10, &[a, b]);
        assert!(differs(&mut e, this, other));
        let (this, other) = pair(&mut e);
        e.mem.set_u32(other + 0x101, 1);
        assert!(differs(&mut e, this, other));
        assert_eq!(e.mem.u16(this + 0x4f8), 0x2345);
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(differs(&mut e, this, this));
    }

    #[test]
    fn getters_and_setters_check_their_ranges() {
        let mut e = copy_engine();
        let this = race(&mut e);
        components(&mut e, this);
        e.mem.set_u32(this + 0x98, 0x98);
        e.mem.set_u8(this + 0x9d, 0x9d);
        assert_eq!(e.call(0x0061_30c0, &args![this, 3u32]).u32(), this + 0x56);
        assert_eq!(e.call(0x0061_3100, &args![this, 1u32]).u32() & 0xff, 0x9d);
        assert_eq!(e.call(0x0061_3870, &args![this, 1u32]).u32(), 0x98);
        e.mem.set_u32(this + 0xa0, f32::NAN.to_bits());
        assert_eq!(e.call(0x0061_3120, &args![this]).f64(), 5.0);
        e.mem.set_u32(this + 0xa0, 0.5f32.to_bits());
        assert_eq!(e.call(0x0061_3120, &args![this]).f64(), 0.5);
        e.mem.set_u32(this + 0xa4, (-1.0f32).to_bits());
        assert_eq!(e.call(0x0061_3160, &args![this]).f64(), 3.0);
        e.call(0x0061_39b0, &args![this, 1u32, 7u32, 0x51u32]);
        assert_eq!(e.mem.u32(this + 0xb0 + 0xc0 + 7 * 0x18 + 4), 0x51);
        e.call(0x0061_39b0, &args![this, 2u32, 0u32, 0x52u32]);
        e.call(0x0061_39b0, &args![this, 0u32, 8u32, 0x52u32]);
        e.call(0x0061_3a60, &args![this, 0u32, 2u32, 0x53u32]);
        assert_eq!(e.mem.u32(this + 0x338 + 2 * 0x18 + 4), 0x53);
        e.call(0x0061_3a60, &args![this, 0u32, 3u32, 0x54u32]);
        e.call(0x0061_3ad0, &args![this, 1u32, 0x55u32]);
        assert_eq!(e.mem.u32(this + 0x3e0 + 4), 0x55);
        e.call(0x0061_3ad0, &args![this, (-1i32) as u32, 0x56u32]);
        e.call_log = Some(vec![]);
        e.call(0x0061_3b60, &args![this, 2u32, 1u32, 0x57u32]);
        e.call(0x0061_3b60, &args![this, 3u32, 1u32, 0x57u32]);
        e.call(0x0061_3c00, &args![this, 1u32, 2u32, 0x58u32]);
        e.call(0x0061_3c00, &args![this, 2u32, 2u32, 0x58u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, SET_TEXTURE_NAME),
            vec![
                vec![this + 0x230 + 2 * 0x60 + 0xc, 0x57],
                vec![this + 0x2f0 + 0x24 + 0x18, 0x58]
            ]
        );
        assert_eq!(
            e.call(0x0061_3bb0, &args![this, 1u32, 2u32]).u32(),
            this + 0x2f0 + 0x24 + 0x18
        );
        assert_eq!(e.call(0x0061_3bb0, &args![this, 1u32, 3u32]).u32(), 0);
        assert_eq!(e.call(0x0061_3bb0, &args![this, 2u32, 0u32]).u32(), 0);
        let untouched: Vec<u32> = [0xb0u32 + 2 * 0xc0, 0xb0 + 8 * 0x18, 0x338 + 3 * 0x18]
            .iter()
            .map(|o| e.mem.u32(this + o + 4))
            .collect();
        assert!(untouched.iter().all(|w| *w != 0x52 && *w != 0x54));
    }
}
