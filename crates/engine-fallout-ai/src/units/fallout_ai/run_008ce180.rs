//! `fallout/ai/run_008ce180` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008ce390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x8C (`thiscall`; no Xbox PDB name).
pub fn fn_008ce390(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x8c)
}

// ---- n028: more `ArrowProjectile` methods (the vtables and fields as in
// `run_008caa70.rs`) ----

/// `ArrowProjectile` fields (Xbox PDB names, PC offsets 0x10 below the
/// PDB's): `pCurrentProcess`, `psCollisionData`, `eState`, `pShooter`,
/// `pEnchantment`, `pPoison`, `bNeedsFinishInitLoadGame`,
/// `bAddedToInventory`, `bBowIgnoresWeaponsResist`.
const PROCESS: u32 = 0x68;
const COLLISION: u32 = 0x88;
const STATE: u32 = 0x8c;
const SHOOTER: u32 = 0xa4;
const ENCHANTMENT: u32 = 0xa8;
const POISON: u32 = 0xb0;
const DELTA: u32 = 0xb4;
const NEEDS_FINISH_INIT_LOAD_GAME: u32 = 0xc0;
const ADDED_TO_INVENTORY: u32 = 0xc1;
const BOW_IGNORES_WEAPONS_RESIST: u32 = 0xc2;
/// `ArrowProjectile::CollisionInfo`: `eCollisionType`, `Point`, `Normal`,
/// `Velocity`, `pCollidee`, `pClosestObject`, `Rotation`.
const COLLISION_TYPE: u32 = 0x00;
const COLLISION_POINT: u32 = 0x04;
const COLLISION_NORMAL: u32 = 0x10;
const COLLISION_VELOCITY: u32 = 0x1c;
const COLLISION_COLLIDEE: u32 = 0x28;
const COLLISION_CLOSEST_OBJECT: u32 = 0x2c;
const COLLISION_ROTATION: u32 = 0x30;
/// The record's constructor (`008cc6f0`, `run_008caa70.rs`) and its size.
const COLLISION_CONSTRUCT: u32 = 0x008c_c6f0;
const COLLISION_SIZE: u32 = 0x54;
/// The nine floats at `011a9448` (an identity matrix in the file's data).
const IDENTITY_ROTATION: u32 = 0x011a_9448;

/// The word holding the save/load game object (`TESSaveLoadGame`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `ProcessLists` (`011e0e80`).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The `Setting`-like object at `011de4e8` whose value `00408d60` gives the
/// address of; the save code prints its sizes when that byte is set.
const SAVE_DEBUG_SETTING: u32 = 0x011d_e4e8;
const SETTING_ADDRESS: u32 = 0x0040_8d60;
/// `TESSaveLoadGame::UseSaveGameBlocks` (Xbox PDB name of `00862110`).
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
/// `008df040` (`thiscall` on the save/load game): the save's version byte.
const SAVE_VERSION: u32 = 0x008d_f040;
/// `00825c00` (`thiscall`): the save buffer's current position.
const SAVE_POSITION: u32 = 0x0082_5c00;
/// `008579b0` / `008579e0` (`thiscall(address, size)` on the save/load game):
/// write / read raw bytes.
const SAVE_WRITE: u32 = 0x0085_79b0;
const SAVE_READ: u32 = 0x0085_79e0;
/// `TESForm::SaveGameDataOLD` (`00484ce0`), `TESForm::SaveNumericID`
/// (`00484d20`), `TESForm::LoadGameDataOLD` (`00484d00`) and
/// `TESForm::LoadNumericID` (`00484d40`): `thiscall(address, size)` on the
/// form (Xbox PDB names).
const SAVE_DATA: u32 = 0x0048_4ce0;
const SAVE_NUMERIC_ID: u32 = 0x0048_4d20;
const LOAD_DATA: u32 = 0x0048_4d00;
const LOAD_NUMERIC_ID: u32 = 0x0048_4d40;
/// `0084e3a0` (`thiscall` on a form): the id it is saved under.
const FORM_SAVE_ID: u32 = 0x0084_e3a0;
/// `004839c0` (cdecl `(id)`): the form with that id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `004fd3e0` / `004fd3c0` (`thiscall` on the save/load game): the record of
/// the form being saved / loaded (form id at +0, flags at +5, version at +9),
/// or null.
const SAVING_FORM: u32 = 0x004f_d3e0;
const LOADING_FORM: u32 = 0x004f_d3c0;
/// `Error` (Xbox PDB name of `0040fbe0`) and `005b5e40`, both cdecl with a
/// format and its arguments.
const ERROR: u32 = 0x0040_fbe0;
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout\Ai\ArrowProjectile.cpp"`.
const SOURCE_FILE: u32 = 0x0108_5d58;
/// The block tag `'BLOK'` the save writes before an arrow's data.
const BLOCK_TAG: u32 = 0x424c_4f4b;
/// `__RTDynamicCast` and the type descriptors of `TESForm` (`01183028`),
/// `Actor` (`011846d4`), `EnchantmentItem` (`01183594`), `AlchemyItem`
/// (`011830cc`), `TESObjectREFR` (`011841cc`), `MobileObject` (`01184920`)
/// and `ArrowProjectile` (`011a28e0`; names read at the descriptors).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const FORM_TYPE: u32 = 0x0118_3028;
const ACTOR_TYPE: u32 = 0x0118_46d4;
const ENCHANTMENT_TYPE: u32 = 0x0118_3594;
const ALCHEMY_TYPE: u32 = 0x0118_30cc;
const REFERENCE_TYPE: u32 = 0x0118_41cc;
const MOBILE_OBJECT_TYPE: u32 = 0x0118_4920;
const ARROW_PROJECTILE_TYPE: u32 = 0x011a_28e0;
/// `0045cd60` (`thiscall` on a process): its level (word at +0x28).
const PROCESS_LEVEL: u32 = 0x0045_cd60;
/// `ProcessLists::RemoveReference` (`0096d470`, `thiscall(reference, level)`)
/// and `ProcessLists::AddReference` (`0096d450`, `thiscall`, five words),
/// Xbox PDB names.
const REMOVE_REFERENCE: u32 = 0x0096_d470;
const ADD_REFERENCE: u32 = 0x0096_d450;
/// `MobileObject::GetCurrentProcessType` (Xbox PDB name of `00931850`).
const CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
/// `operator new` / `operator delete` (cdecl, one word).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The no-op constructor of an `NiPoint3`/`NiQuaternion` (`006815c0`).
const CONSTRUCT_IN_PLACE: u32 = 0x0068_15c0;
/// `0043fcd0` (`thiscall` on a reference): its loaded 3D.
const REFERENCE_3D: u32 = 0x0043_fcd0;

/// The words of a cdecl call: the callee's arguments in order.
fn print(e: &mut Engine, callee: u32, words: &[u32]) {
    e.call(callee, words);
}

// Translated from 008ce180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes up to `count` arrows of the item `item` attached to `reference`
/// (`0085f810`) from the process lists (`011e0e80`'s list object,
/// `00717e50`): those with `bAddedToInventory` set (`fn_008ce370`) only when
/// `added_only`. In list 0 (from index 0) each is deleted (slot 0x10 with
/// 1) when `delete` is set, else made to drop (`008cc6d0(3)`); in list 1 the
/// loop starts at the list's end index (`005be5c0(1)`) and runs while below
/// it, so it does nothing unless that index changes between the calls.
pub fn fn_008ce180(
    e: &mut Engine,
    item: u32,
    count: i32,
    reference: u32,
    delete: bool,
    added_only: bool,
) {
    let mut removed = 0i32;
    let list = e.call(0x0071_7e50, &args![PROCESS_LISTS]).u32();
    let mut index = 0u32;
    while removed < count && index < e.call(0x005b_e5c0, &args![list, 0u32]).u32() {
        if let Some(arrow) = matching_arrow(e, list, index, item, reference, added_only) {
            if delete {
                // The cast result is never null here; the exe still tests it.
                if arrow != 0 {
                    e.vcall(arrow, 0x10, &args![1u32]);
                }
            } else {
                e.call(0x008c_c6d0, &args![arrow, 3u32]);
            }
            removed += 1;
        }
        index += 1;
    }
    let mut index = e.call(0x005b_e5c0, &args![list, 1u32]).u32();
    while removed < count && index < e.call(0x005b_e5c0, &args![list, 1u32]).u32() {
        if let Some(arrow) = matching_arrow(e, list, index, item, reference, added_only) {
            if arrow != 0 {
                e.vcall(arrow, 0x10, &args![1u32]);
            }
            removed += 1;
        }
        index += 1;
    }
}

/// The reference at `index` of the list object (`00968670`) when it is an
/// `ArrowProjectile` (`__RTDynamicCast` from `MobileObject`) attached to
/// `reference` (`0085f810`) whose item (`007af430`) is `item` and, with
/// `added_only`, whose `bAddedToInventory` is set.
fn matching_arrow(
    e: &mut Engine,
    list: u32,
    index: u32,
    item: u32,
    reference: u32,
    added_only: bool,
) -> Option<u32> {
    let object = e.call(0x0096_8670, &args![list, index]).u32();
    let arrow = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                object,
                0u32,
                MOBILE_OBJECT_TYPE,
                ARROW_PROJECTILE_TYPE,
                0u32
            ],
        )
        .u32();
    if arrow == 0
        || e.call(0x0085_f810, &args![arrow]).u32() != reference
        || e.call(0x007a_f430, &args![arrow]).u32() != item
    {
        return None;
    }
    if added_only && fn_008ce370(e, Ptr::new(arrow)) == 0 {
        return None;
    }
    Some(arrow)
}

// Translated from 008ce370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bAddedToInventory` (the byte at +0xC1).
pub fn fn_008ce370(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + ADDED_TO_INVENTORY)
}

/// The record of the form being saved and the size the debug messages of
/// the save functions print: `Error(long, size, form id, name, flags, line,
/// file)` with the form (`004839c0`, its name from slot 0x130), else
/// `Error(short, size, line, file)`.
fn report_save_size(e: &mut Engine, size: u32, line: u32, long: u32, short: u32) {
    let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
    let record = e.call(SAVING_FORM, &args![save_load_game]).u32();
    if record == 0 {
        print(e, ERROR, &[short, size, line, SOURCE_FILE]);
        return;
    }
    let id = e.mem.u32(record);
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    let flags = e.mem.u32(record + 5);
    let name = e.vcall(form, 0x130, &[]).u32();
    let id = e.mem.u32(record);
    print(e, ERROR, &[long, size, id, name, flags, line, SOURCE_FILE]);
}

/// Whether the save code's debug byte (`00408d60` on `011de4e8`) is set.
fn save_debug(e: &mut Engine) -> bool {
    let setting = e.call(SETTING_ADDRESS, &args![SAVE_DEBUG_SETTING]).u32();
    e.mem.u8(setting) != 0
}

/// The save's version byte (`008df040`).
fn save_version(e: &mut Engine) -> u32 {
    let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
    u32::from(e.call(SAVE_VERSION, &args![save_load_game]).u8())
}

/// Whether the save uses blocks (`TESSaveLoadGame::UseSaveGameBlocks`).
fn uses_blocks(e: &mut Engine) -> bool {
    let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
    e.call(USE_SAVE_GAME_BLOCKS, &args![save_load_game]).bool()
}

// Translated from 008ce3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetSaveSize` of an arrow (the exe's messages name it): the
/// `MobileObject` size (`00931ff0` with `flags`), 6 more with save blocks,
/// 0x31 for the fields, and with a collision record 0x28 more, 0x10 for the
/// rotation from version 0x50 and 8 for a record of type 0 or 1; one byte
/// each from versions 0x54 and 0x55. With the debug byte set the size is
/// printed (`Error`, line 0x87E).
pub fn fn_008ce3b0(e: &mut Engine, this: Ptr, flags: u32) -> u16 {
    let at = this.addr();
    let base = e.call(0x0093_1ff0, &args![this, flags]).u16();
    let mut size = base;
    if uses_blocks(e) {
        size = size.wrapping_add(4).wrapping_add(2);
    }
    // `eState`, five floats, `DeltaS`, three form ids, the record flag.
    size = size.wrapping_add(4 + 4 * 5 + 0xc + 4 * 3 + 1);
    if e.mem.u32(at + COLLISION) != 0 {
        size = size.wrapping_add(4 + 0xc * 3);
        if save_version(e) >= 0x50 {
            size = size.wrapping_add(0x10);
        }
        let kind = e.mem.u32(e.mem.u32(at + COLLISION) + COLLISION_TYPE);
        if kind == 1 || kind == 0 {
            size = size.wrapping_add(4 + 2 + 2);
        }
    }
    if save_version(e) >= 0x54 {
        size = size.wrapping_add(1);
    }
    if save_version(e) >= 0x55 {
        size = size.wrapping_add(1);
    }
    if save_debug(e) {
        let printed = u32::from(size).wrapping_sub(u32::from(base));
        report_save_size(e, printed, 0x87e, 0x0101_2cb0, 0x0101_2c78);
    }
    size
}

// Translated from 008ce610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveGame` of an arrow (the exe's messages name it): the `MobileObject`
/// part (`00932070` with `flags`), then (in a `'BLOK'` block with a 16-bit
/// size when the save uses blocks) `eState`, the five floats from +0x90,
/// `DeltaS`, the ids of shooter, enchantment and poison, whether there is a
/// collision record and, with one, its type, point, normal and velocity,
/// its rotation as a quaternion (`00a6df40`) from version 0x50, and for
/// types 0 and 1 the collidee's id and the closest object as two 16-bit
/// halves (the collision object counts `004b66d0`/`004b6810` of the
/// collidee's 3D, or the stored halves while a load is pending);
/// `bAddedToInventory` from version 0x54 and `bBowIgnoresWeaponsResist`
/// from 0x55. The debug byte prints the size (`Error`, line 0x8EA); a block
/// over 0xFFFF bytes is reported (`005b5e40`).
pub fn fn_008ce610(e: &mut Engine, this: Ptr, flags: u32) {
    let at = this.addr();
    e.call(0x0093_2070, &args![this, flags]);
    e.with_stack(0x28, |e, frame| {
        // Locals: the block size (EBP-0x10), the tag (-0x20), the three ids
        // (-0xC, -0x1C, -0x18), the record flag (-0x1), the quaternion
        // (-0x30), the collidee id (-0x38) and the two halves (-0x3C, -0x34).
        let block_size = frame.addr();
        let tag = block_size + 4;
        let id = block_size + 8;
        let flag = block_size + 0xc;
        let quaternion = block_size + 0x10;
        let halves = block_size + 0x20;
        e.mem.set_u16(block_size, 0);
        let mut block = 0u32;
        let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
        let mut start = e.call(SAVE_POSITION, &args![save_load_game]).u32();
        if save_debug(e) {
            let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
            start = e.call(SAVE_POSITION, &args![save_load_game]).u32();
        }
        if uses_blocks(e) {
            e.mem.set_u32(tag, BLOCK_TAG);
            let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
            e.call(SAVE_WRITE, &args![save_load_game, tag, 4u32]);
            let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
            block = e.call(SAVE_POSITION, &args![save_load_game]).u32();
            let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
            e.call(SAVE_WRITE, &args![save_load_game, block_size, 2u32]);
        }
        let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
        e.call(SAVE_WRITE, &args![save_load_game, at + STATE, 4u32]);
        for offset in [0x90, 0x94, 0x98, 0x9c, 0xa0] {
            e.call(SAVE_DATA, &args![this, at + offset, 4u32]);
        }
        e.call(SAVE_DATA, &args![this, at + DELTA, 0xcu32]);
        for field in [SHOOTER, ENCHANTMENT, POISON] {
            e.mem.set_u32(id, 0);
            let form = e.mem.u32(at + field);
            if form != 0 {
                let saved = e.call(FORM_SAVE_ID, &args![form]).u32();
                e.mem.set_u32(id, saved);
            }
            e.call(SAVE_NUMERIC_ID, &args![this, id, 4u32]);
        }
        let has_record = e.mem.u32(at + COLLISION) != 0;
        e.mem.set_u8(flag, u8::from(has_record));
        e.call(SAVE_DATA, &args![this, flag, 1u32]);
        if e.mem.u8(flag) != 0 {
            let record = e.mem.u32(at + COLLISION);
            let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
            e.call(SAVE_WRITE, &args![save_load_game, record, 4u32]);
            for offset in [COLLISION_POINT, COLLISION_NORMAL, COLLISION_VELOCITY] {
                let record = e.mem.u32(at + COLLISION);
                e.call(SAVE_DATA, &args![this, record + offset, 0xcu32]);
            }
            if save_version(e) >= 0x50 {
                e.call(CONSTRUCT_IN_PLACE, &args![quaternion]);
                let record = e.mem.u32(at + COLLISION);
                e.call(0x00a6_df40, &args![quaternion, record + COLLISION_ROTATION]);
                let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
                e.call(SAVE_WRITE, &args![save_load_game, quaternion, 0x10u32]);
            }
            let kind = e.mem.u32(e.mem.u32(at + COLLISION) + COLLISION_TYPE);
            if kind == 1 || kind == 0 {
                save_collidee(e, this, id, halves);
            }
        }
        if save_version(e) >= 0x54 {
            e.call(SAVE_DATA, &args![this, at + ADDED_TO_INVENTORY, 1u32]);
        }
        if save_version(e) >= 0x55 {
            e.call(
                SAVE_DATA,
                &args![this, at + BOW_IGNORES_WEAPONS_RESIST, 1u32],
            );
        }
        if save_debug(e) {
            let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
            let now = e.call(SAVE_POSITION, &args![save_load_game]).u32();
            report_save_size(e, now.wrapping_sub(start), 0x8ea, 0x0101_53a0, 0x0101_536c);
        }
        if uses_blocks(e) {
            let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
            let now = e.call(SAVE_POSITION, &args![save_load_game]).u32();
            if now > block.wrapping_add(0xffff) {
                print(e, PRINT_MESSAGE, &[0x0101_5318, SOURCE_FILE, 0x8ea]);
            }
            e.mem.set_u16(block, now.wrapping_sub(block) as u16);
        }
    });
}

/// The type 0/1 part of `fn_008ce610`: the collidee's id and the closest
/// object's two halves, in the locals at `id` and `halves` (two words).
fn save_collidee(e: &mut Engine, this: Ptr, id: u32, halves: u32) {
    let at = this.addr();
    let record = |e: &Engine| e.mem.u32(at + COLLISION);
    e.mem.set_u32(id, 0);
    let collidee = e.mem.u32(record(e) + COLLISION_COLLIDEE);
    if collidee != 0 {
        let saved = e.call(FORM_SAVE_ID, &args![collidee]).u32();
        e.mem.set_u32(id, saved);
    }
    e.call(SAVE_NUMERIC_ID, &args![this, id, 4u32]);
    let high = halves;
    let low = halves + 4;
    e.mem.set_u16(high, 0);
    e.mem.set_u16(low, 0xffff);
    if e.mem.u32(record(e) + COLLISION_COLLIDEE) != 0
        && e.mem.u32(record(e) + COLLISION_CLOSEST_OBJECT) != 0
    {
        if e.mem.u8(at + NEEDS_FINISH_INIT_LOAD_GAME) == 0 {
            let actor = u32::from(e.mem.u32(record(e) + COLLISION_TYPE) == 0);
            let collidee = e.mem.u32(record(e) + COLLISION_COLLIDEE);
            let node = e.call(REFERENCE_3D, &args![collidee]).u32();
            let count = e.call(0x004b_66d0, &args![node, actor, actor, 1u32]).u16();
            e.mem.set_u16(high, count);
            let closest = e.mem.u32(record(e) + COLLISION_CLOSEST_OBJECT);
            let object = e.call(0x004a_8b00, &args![closest]).u32();
            let collidee = e.mem.u32(record(e) + COLLISION_COLLIDEE);
            let node = e.call(REFERENCE_3D, &args![collidee]).u32();
            let index = e
                .call(0x004b_6810, &args![node, object, actor, actor, 1u32])
                .u16();
            e.mem.set_u16(low, index);
        } else {
            let word = e.mem.u32(record(e) + COLLISION_CLOSEST_OBJECT);
            e.mem.set_u16(high, (word >> 16) as u16);
            e.mem.set_u16(low, word as u16);
        }
    }
    e.call(SAVE_DATA, &args![this, high, 2u32]);
    e.call(SAVE_DATA, &args![this, low, 2u32]);
}

/// A load message with the form being loaded: `005b5e40(long, file, line,
/// form id, name, version, flags)` with the form, else `005b5e40(short, file,
/// line, version)`; `value` (when given) goes after the format.
fn report_load(e: &mut Engine, value: Option<u32>, line: u32, long: u32, short: u32) {
    let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
    let record = e.call(LOADING_FORM, &args![save_load_game]).u32();
    let mut words = vec![];
    if record == 0 {
        let version = save_version(e);
        words.push(short);
        words.extend(value);
        words.extend([SOURCE_FILE, line, version]);
    } else {
        let id = e.mem.u32(record);
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let flags = e.mem.u32(record + 5);
        let version = u32::from(e.mem.u8(record + 9));
        let name = e.vcall(form, 0x130, &[]).u32();
        let id = e.mem.u32(record);
        words.push(long);
        words.extend(value);
        words.extend([SOURCE_FILE, line, id, name, version, flags]);
    }
    print(e, PRINT_MESSAGE, &words);
}

// Translated from 008ceb50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008ceb50 - the compiler's exception frame (FS:[0]) is not translated
/// `LoadGame` of an arrow, the reverse of `fn_008ce610`: the
/// `MobileObject` part (`009320e0`), the block header (a wrong tag is
/// reported, line 0x8F5), the fields and ids (stored unresolved), 8 bytes
/// skipped (`00857bd0`) before version 0x20, the record flag (from version
/// 0x20; before it, states 1 and 2 have a record), and with a record a new
/// one (identity rotation) with type, point, normal, velocity, the
/// rotation from its quaternion (`004f0180`) from version 0x50, and for
/// types 0 and 1 the collidee id and the closest object (two 16-bit halves
/// from version 0x20, the low one sign-extended; before it a length byte and
/// that many bytes of a zeroed `operator new` block, kept as the pointer).
/// Versions 0x54/0x55 add the two flags. A block whose end does not match
/// the position is reported (line 0x965).
pub fn fn_008ceb50(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    let at = this.addr();
    e.call(0x0093_20e0, &args![this, first, second]);
    e.with_stack(0x60, |e, frame| {
        // Locals: the block size (EBP-0x18), the tag (-0x28), the ids
        // (-0x14, -0x24, -0x20), the record flag (-0xD), the quaternion
        // (-0x64), the matrix (-0x54), the collidee (-0x68), the halves
        // (-0x70, -0x6C) and the old name length (-0x75).
        let block_size = frame.addr();
        let tag = block_size + 4;
        let id = block_size + 8;
        let flag = block_size + 0xc;
        let quaternion = block_size + 0x10;
        let matrix = block_size + 0x20;
        let halves = block_size + 0x44;
        let length = block_size + 0x58;
        e.mem.set_u16(block_size, 0);
        let mut block = 0u32;
        if uses_blocks(e) {
            let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
            e.call(SAVE_READ, &args![save_load_game, tag, 4u32]);
            if e.mem.u32(tag) != BLOCK_TAG {
                report_load(e, None, 0x8f5, 0x0101_5718, 0x0101_56a8);
            }
            let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
            block = e.call(SAVE_POSITION, &args![save_load_game]).u32();
            let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
            e.call(SAVE_READ, &args![save_load_game, block_size, 2u32]);
        }
        let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
        e.call(SAVE_READ, &args![save_load_game, at + STATE, 4u32]);
        for offset in [0x90, 0x94, 0x98, 0x9c, 0xa0] {
            e.call(LOAD_DATA, &args![this, at + offset, 4u32]);
        }
        e.call(LOAD_DATA, &args![this, at + DELTA, 0xcu32]);
        for field in [SHOOTER, ENCHANTMENT, POISON] {
            e.call(LOAD_NUMERIC_ID, &args![this, id, 4u32]);
            let loaded = e.mem.u32(id);
            e.mem.set_u32(at + field, loaded);
        }
        if save_version(e) < 0x20 {
            let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
            e.call(0x0085_7bd0, &args![save_load_game, 8u32]);
        }
        e.mem.set_u8(flag, 0);
        if save_version(e) >= 0x20 {
            e.call(LOAD_DATA, &args![this, flag, 1u32]);
        }
        if save_version(e) < 0x20 {
            let state = e.mem.u32(at + STATE);
            e.mem.set_u8(flag, u8::from(state == 1 || state == 2));
        }
        if e.mem.u8(flag) != 0 {
            load_record(e, this, quaternion, matrix, halves, length);
        }
        if save_version(e) >= 0x54 {
            e.call(LOAD_DATA, &args![this, at + ADDED_TO_INVENTORY, 1u32]);
        }
        if save_version(e) >= 0x55 {
            e.call(
                LOAD_DATA,
                &args![this, at + BOW_IGNORES_WEAPONS_RESIST, 1u32],
            );
        }
        if uses_blocks(e) {
            let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
            let now = e.call(SAVE_POSITION, &args![save_load_game]).u32();
            let end = u32::from(e.mem.u16(block_size)).wrapping_add(block);
            let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
            let record = e.call(LOADING_FORM, &args![save_load_game]).u32();
            if record != 0 {
                let form_id = e.mem.u32(record);
                let form = e.call(LOOKUP_FORM, &args![form_id]).u32();
                let message = if now > end {
                    Some((0x0101_5588, now.wrapping_sub(end)))
                } else if now < end {
                    Some((0x0101_5500, end.wrapping_sub(now)))
                } else {
                    None
                };
                if let Some((format, difference)) = message {
                    let flags = e.mem.u32(record + 5);
                    let version = u32::from(e.mem.u8(record + 9));
                    let name = e.vcall(form, 0x130, &[]).u32();
                    let form_id = e.mem.u32(record);
                    print(
                        e,
                        PRINT_MESSAGE,
                        &[
                            format,
                            difference,
                            SOURCE_FILE,
                            0x965,
                            form_id,
                            name,
                            version,
                            flags,
                        ],
                    );
                }
            } else if now > end {
                let version = save_version(e);
                let difference = now.wrapping_sub(end);
                print(
                    e,
                    PRINT_MESSAGE,
                    &[0x0101_54a0, difference, SOURCE_FILE, 0x965, version],
                );
            } else if now < end {
                let version = save_version(e);
                let difference = end.wrapping_sub(now);
                print(
                    e,
                    PRINT_MESSAGE,
                    &[0x0101_5440, difference, SOURCE_FILE, 0x965, version],
                );
            }
        }
    });
}

/// The record part of `fn_008ceb50`.
fn load_record(e: &mut Engine, this: Ptr, quaternion: u32, matrix: u32, halves: u32, length: u32) {
    let at = this.addr();
    let record = |e: &Engine| e.mem.u32(at + COLLISION);
    let block = e.call(OPERATOR_NEW, &args![COLLISION_SIZE]).u32();
    let created = if block == 0 {
        0
    } else {
        e.call(COLLISION_CONSTRUCT, &args![block]).u32()
    };
    e.mem.set_u32(at + COLLISION, created);
    e.mem.set_u32(record(e) + COLLISION_CLOSEST_OBJECT, 0);
    e.mem.set_u32(record(e) + COLLISION_COLLIDEE, 0);
    for i in 0..9 {
        let word = e.mem.u32(IDENTITY_ROTATION + 4 * i);
        e.mem.set_u32(record(e) + COLLISION_ROTATION + 4 * i, word);
    }
    let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
    let destination = record(e);
    e.call(SAVE_READ, &args![save_load_game, destination, 4u32]);
    for offset in [COLLISION_POINT, COLLISION_NORMAL, COLLISION_VELOCITY] {
        let destination = record(e) + offset;
        e.call(LOAD_DATA, &args![this, destination, 0xcu32]);
    }
    if save_version(e) >= 0x50 {
        e.call(CONSTRUCT_IN_PLACE, &args![quaternion]);
        let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
        e.call(SAVE_READ, &args![save_load_game, quaternion, 0x10u32]);
        e.call(CONSTRUCT_IN_PLACE, &args![matrix]);
        let destination = record(e) + COLLISION_ROTATION;
        e.call(0x004f_0180, &args![quaternion, destination]);
    }
    let kind = e.mem.u32(record(e) + COLLISION_TYPE);
    if kind != 1 && kind != 0 {
        return;
    }
    let collidee = halves + 0x10;
    e.call(LOAD_NUMERIC_ID, &args![this, collidee, 4u32]);
    let loaded = e.mem.u32(collidee);
    e.mem.set_u32(record(e) + COLLISION_COLLIDEE, loaded);
    if save_version(e) >= 0x20 {
        let high = halves;
        let low = halves + 4;
        e.call(LOAD_DATA, &args![this, high, 2u32]);
        e.call(LOAD_DATA, &args![this, low, 2u32]);
        let value =
            (u32::from(e.mem.u16(high)) << 16).wrapping_add(e.mem.u16(low) as i16 as i32 as u32);
        e.mem.set_u32(record(e) + COLLISION_CLOSEST_OBJECT, value);
    }
    if save_version(e) < 0x20 {
        e.call(LOAD_DATA, &args![this, length, 1u32]);
        let size = u32::from(e.mem.u8(length)) + 1;
        let name = e.call(OPERATOR_NEW, &args![size]).u32();
        e.call(0x0040_3d30, &args![name, 0u32, size]);
        let count = u32::from(e.mem.u8(length));
        e.call(LOAD_DATA, &args![this, name, count]);
        e.mem.set_u32(record(e) + COLLISION_CLOSEST_OBJECT, name);
    }
}

// Translated from 008cf2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InitLoadGame` of an arrow: the `MobileObject` part (`00932490`), then the
/// loaded ids resolved (`004839c0`, `__RTDynamicCast` from `TESForm`): the
/// shooter to an `Actor`, the enchantment to an `EnchantmentItem`, the
/// poison to an `AlchemyItem` and, for a record of type 0 or 1, the
/// collidee to a `TESObjectREFR`. The arrow is moved out of and back into
/// the process lists at its process type (`RemoveReference`,
/// `AddReference`).
pub fn fn_008cf2c0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    let at = this.addr();
    e.call(0x0093_2490, &args![this, first, second]);
    for (field, target) in [
        (SHOOTER, ACTOR_TYPE),
        (ENCHANTMENT, ENCHANTMENT_TYPE),
        (POISON, ALCHEMY_TYPE),
    ] {
        let id = e.mem.u32(at + field);
        if id != 0 {
            let resolved = resolve(e, id, target);
            e.mem.set_u32(at + field, resolved);
        }
    }
    let record = e.mem.u32(at + COLLISION);
    if record != 0 {
        let kind = e.mem.u32(record + COLLISION_TYPE);
        if kind == 1 || kind == 0 {
            let id = e.mem.u32(e.mem.u32(at + COLLISION) + COLLISION_COLLIDEE);
            if id != 0 {
                let resolved = resolve(e, id, REFERENCE_TYPE);
                let record = e.mem.u32(at + COLLISION);
                e.mem.set_u32(record + COLLISION_COLLIDEE, resolved);
            }
        }
    }
    let kind = e.call(CURRENT_PROCESS_TYPE, &args![this]).u32();
    e.call(REMOVE_REFERENCE, &args![PROCESS_LISTS, this, kind]);
    let kind = e.call(CURRENT_PROCESS_TYPE, &args![this]).u32();
    e.call(
        ADD_REFERENCE,
        &args![PROCESS_LISTS, this, kind, 0u32, 0u32, 0u32],
    );
}

/// The form of `id` (`004839c0`) cast from `TESForm` to `target`.
fn resolve(e: &mut Engine, id: u32, target: u32) -> u32 {
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    e.call(RT_DYNAMIC_CAST, &args![form, 0u32, FORM_TYPE, target, 0u32])
        .u32()
}

// Translated from 008cf440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FinishInitLoadGame` of an arrow (`a`, `b`: the two words the base
/// `MobileObject::FinishInitLoadGame` takes). After `0046a010(0)`: without
/// 3D, an arrow whose collidee passes `00440da0` and `00440d80` is dropped
/// (slot 0xC4 with 1, the closest object cleared); otherwise the finish is
/// left for later (`bNeedsFinishInitLoadGame`, `iLoadGameFlags` = `a`).
/// With 3D: the controller (`fn_008caea0` with the shooter and 1.0), the
/// base part (`00932520`), and for a record of type 0 or 1 the closest
/// object resolved: from version 0x20 the saved halves, when the
/// collidee's collision object count (`GetCollisionObjectCountInSceneGraph`)
/// still equals the high half, pick the object (`004b6980`, its
/// `0044ddc0`) unless the word was -1, else the message `"AI: Collision
/// object count has changed on object %s %08"`; before 0x20 the saved name
/// is looked up (`004aae30`) and freed. A record left without one becomes
/// type 3. Last `0085f710` on the save/load game.
pub fn fn_008cf440(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    let at = this.addr();
    let record = |e: &Engine| e.mem.u32(at + COLLISION);
    e.call(0x0046_a010, &args![this, 0u32]);
    e.call(REFERENCE_3D, &args![this]);
    if e.call(REFERENCE_3D, &args![this]).u32() == 0 {
        if record(e) != 0 && e.mem.u32(record(e) + COLLISION_COLLIDEE) != 0 {
            let collidee = e.mem.u32(record(e) + COLLISION_COLLIDEE);
            if e.call(0x0044_0da0, &args![collidee]).bool() {
                let collidee = e.mem.u32(record(e) + COLLISION_COLLIDEE);
                if e.call(0x0044_0d80, &args![collidee]).bool() {
                    e.vcall(at, 0xc4, &args![1u32]);
                    let record = record(e);
                    e.mem.set_u32(record + COLLISION_CLOSEST_OBJECT, 0);
                    return;
                }
            }
        }
        e.mem.set_u8(at + NEEDS_FINISH_INIT_LOAD_GAME, 1);
        e.mem.set_u32(at + 0xc4, a);
        return;
    }
    let shooter = e.mem.u32(at + SHOOTER);
    e.call(0x008c_aea0, &args![this, shooter, 1.0f32]);
    e.call(0x0093_2520, &args![this, a, b]);
    if record(e) != 0 {
        let kind = e.mem.u32(record(e) + COLLISION_TYPE);
        if kind == 1 || kind == 0 {
            if save_version(e) >= 0x20 {
                let actor = u32::from(e.mem.u32(record(e) + COLLISION_TYPE) == 0);
                let word = e.mem.u32(record(e) + COLLISION_CLOSEST_OBJECT);
                let high = word >> 16;
                let target = record(e);
                e.mem.set_u32(target + COLLISION_CLOSEST_OBJECT, 0);
                if e.mem.u32(record(e) + COLLISION_COLLIDEE) != 0 {
                    let collidee = e.mem.u32(record(e) + COLLISION_COLLIDEE);
                    let node = e.call(REFERENCE_3D, &args![collidee]).u32();
                    let count = e.call(0x004b_66d0, &args![node, actor, actor, 1u32]).u16();
                    if high != u32::from(count) {
                        let collidee = e.mem.u32(record(e) + COLLISION_COLLIDEE);
                        let id = e.call(FORM_SAVE_ID, &args![collidee]).u32();
                        let collidee = e.mem.u32(record(e) + COLLISION_COLLIDEE);
                        let name = e.vcall(collidee, 0x130, &[]).u32();
                        print(e, PRINT_MESSAGE, &[0x0108_5d9c, name, id]);
                    } else if word != 0xffff_ffff {
                        let collidee = e.mem.u32(record(e) + COLLISION_COLLIDEE);
                        let node = e.call(REFERENCE_3D, &args![collidee]).u32();
                        let object = e
                            .call(0x004b_6980, &args![node, word, actor, actor, 1u32])
                            .u32();
                        if object != 0 {
                            let closest = e.call(0x0044_ddc0, &args![object]).u32();
                            let target = record(e);
                            e.mem.set_u32(target + COLLISION_CLOSEST_OBJECT, closest);
                        }
                    }
                }
            }
            if save_version(e) < 0x20 {
                let name = e.mem.u32(record(e) + COLLISION_CLOSEST_OBJECT);
                if e.mem.u32(record(e) + COLLISION_COLLIDEE) != 0 {
                    let collidee = e.mem.u32(record(e) + COLLISION_COLLIDEE);
                    let node = e.call(REFERENCE_3D, &args![collidee]).u32();
                    let closest = e.call(0x004a_ae30, &args![node, name]).u32();
                    let target = record(e);
                    e.mem.set_u32(target + COLLISION_CLOSEST_OBJECT, closest);
                } else {
                    let target = record(e);
                    e.mem.set_u32(target + COLLISION_CLOSEST_OBJECT, 0);
                }
                e.call(OPERATOR_DELETE, &args![name]);
            }
            if e.mem.u32(record(e) + COLLISION_CLOSEST_OBJECT) == 0 {
                let target = record(e);
                e.mem.set_u32(target + COLLISION_TYPE, 3);
            }
        }
    }
    let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
    e.call(0x0085_f710, &args![save_load_game, this]);
}

// Translated from 008cf740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MobileObject` part of a revert (`00932750` with `flags`); when the
/// save/load game's test (`0055f5b0`) holds, the collision record is freed
/// and cleared.
pub fn fn_008cf740(e: &mut Engine, this: Ptr, flags: u32) {
    let at = this.addr();
    e.call(0x0093_2750, &args![this, flags]);
    let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
    if e.call(0x0055_f5b0, &args![save_load_game]).bool() {
        let record = e.mem.u32(at + COLLISION);
        if record != 0 {
            e.call(OPERATOR_DELETE, &args![record]);
        }
        e.mem.set_u32(at + COLLISION, 0);
    }
}

/// The level moves of `fn_008cf7a0` and `fn_008cf8f0`: unless the arrow
/// already has a process at `level`, it leaves the process lists at its
/// current level, gets a new process of `size` bytes (`constructor`) that
/// copies the old one (slot 0x4), the old one is deleted (slot 0 with 1),
/// the arrow joins the lists at `level` (`AddReference`), slot 0x1FC with 0
/// runs and the new process's slot 0x48. Returns true.
fn move_to_process(e: &mut Engine, this: Ptr, level: u32, size: u32, constructor: u32) -> bool {
    let at = this.addr();
    let process = e.mem.u32(at + PROCESS);
    if process != 0 && e.call(PROCESS_LEVEL, &args![process]).u32() == level {
        return true;
    }
    let process = e.mem.u32(at + PROCESS);
    let current = e.call(PROCESS_LEVEL, &args![process]).u32();
    e.call(REMOVE_REFERENCE, &args![PROCESS_LISTS, this, current]);
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    let created = if block == 0 {
        0
    } else {
        e.call(constructor, &args![block]).u32()
    };
    let old = e.mem.u32(at + PROCESS);
    e.vcall(created, 0x4, &args![old]);
    let old = e.mem.u32(at + PROCESS);
    if old != 0 {
        e.vcall(old, 0x0, &args![1u32]);
    }
    e.mem.set_u32(at + PROCESS, created);
    e.call(
        ADD_REFERENCE,
        &args![PROCESS_LISTS, this, level, 0u32, 0u32, 0u32],
    );
    e.vcall(at, 0x1fc, &args![0u32]);
    let process = e.mem.u32(at + PROCESS);
    e.vcall(process, 0x48, &[]);
    true
}

// Translated from 008cf7a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008cf7a0 - the compiler's exception frame (FS:[0]) is not translated
/// Moves the arrow to high processing: a new `HighProcess` (0x46C bytes,
/// `HighProcess::HighProcess` `008d7510`) at level 0 (see
/// `move_to_process`). Returns true.
pub fn fn_008cf7a0(e: &mut Engine, this: Ptr) -> bool {
    move_to_process(e, this, 0, 0x46c, 0x008d_7510)
}

// Translated from 008cf8f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008cf8f0 - the compiler's exception frame (FS:[0]) is not translated
/// Moves the arrow to middle-high processing: a new process of 0x25C bytes
/// (`00913fe0`) at level 1 (see `move_to_process`). Returns true.
pub fn fn_008cf8f0(e: &mut Engine, this: Ptr) -> bool {
    move_to_process(e, this, 1, 0x25c, 0x0091_3fe0)
}

// Translated from 008cfa40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008cfa40 - the compiler's exception frame (FS:[0]) is not translated
/// `Load3D` of an arrow: `MobileObject::Load3D` (`009335b0` with the low byte
/// of `flag`); a loaded node is named `"Arrow"` (`0102031c`, through the
/// fixed string `00438170` ... `004381b0` and `00a5b950`). Returns the node.
pub fn fn_008cfa40(e: &mut Engine, this: Ptr, flag: u32) -> u32 {
    let node = e.call(0x0093_35b0, &args![this, flag & 0xff]).u32();
    if node != 0 {
        e.with_stack(4, |e, name| {
            let string = e.call(0x0043_8170, &args![name, 0x0102_031cu32]).u32();
            e.call(0x00a5_b950, &args![node, string]);
            e.call(0x0043_81b0, &args![name]);
        });
    }
    node
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008c_e390, fn_008ce390(Ptr) -> u32),
        entry!(0x008c_e180, fn_008ce180(u32, i32, u32, bool, bool)),
        entry!(0x008c_e370, fn_008ce370(Ptr) -> u8),
        entry!(0x008c_e3b0, fn_008ce3b0(Ptr, u32) -> u16),
        entry!(0x008c_e610, fn_008ce610(Ptr, u32)),
        entry!(0x008c_eb50, fn_008ceb50(Ptr, u32, u32)),
        entry!(0x008c_f2c0, fn_008cf2c0(Ptr, u32, u32)),
        entry!(0x008c_f440, fn_008cf440(Ptr, u32, u32)),
        entry!(0x008c_f740, fn_008cf740(Ptr, u32)),
        entry!(0x008c_f7a0, fn_008cf7a0(Ptr) -> bool),
        entry!(0x008c_f8f0, fn_008cf8f0(Ptr) -> bool),
        entry!(0x008c_fa40, fn_008cfa40(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_word_at_0x8c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x8c, 0xdead_beef);
        assert_eq!(e.call(0x008c_e390, &args![this]).u32(), 0xdead_beef);
    }
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn stub_all(e: &mut Engine, callees: &[u32]) {
        for &callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
    }

    fn stub(e: &mut Engine, callee: u32, eax: u32) {
        e.register_double(callee, move |_, _| ret(eax));
    }

    /// An object of `size` bytes whose vtable sends slot `n` to `base + n`.
    fn object(e: &mut Engine, size: u32, base: u32) -> u32 {
        let this = e.mem.alloc(size);
        let table = e.mem.alloc(0x800);
        for slot in (0..0x800).step_by(4) {
            e.mem.set_u32(table + slot, base + slot);
        }
        e.mem.set_u32(this, table);
        this
    }

    const ARROW_SLOTS: u32 = 0x7100_0000;
    const OTHER_SLOTS: u32 = 0x7300_0000;

    /// The save/load game word, the debug setting and the identity matrix.
    fn map_data(e: &mut Engine, version: u8, blocks: bool) {
        e.map(0x011a_0000, 0x0005_0000);
        e.mem.set_u32(SAVE_LOAD_GAME, 0x5a1e);
        for i in 0..9 {
            let value = if i % 4 == 0 { 1.0f32 } else { 0.0 };
            e.mem.set_f32(IDENTITY_ROTATION + 4 * i, value);
        }
        stub(e, SAVE_VERSION, u32::from(version));
        stub(e, USE_SAVE_GAME_BLOCKS, u32::from(blocks));
        let debug = e.mem.alloc(4);
        stub(e, SETTING_ADDRESS, debug);
    }

    #[test]
    fn fn_008ce180_drops_or_deletes_matching_arrows() {
        let mut e = Engine::new();
        stub(&mut e, 0x0071_7e50, 0x1150);
        e.register(0x005b_e5c0, |_, a| ret(if a[1] == 0 { 4 } else { 2 }));
        let arrows: Vec<u32> = (0..4).map(|_| object(&mut e, 0xc8, ARROW_SLOTS)).collect();
        let list = arrows.clone();
        e.register_double(0x0096_8670, move |_, a| ret(list[a[1] as usize]));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        let first = arrows[0];
        e.register_double(0x0085_f810, move |_, a| {
            ret(if a[0] == first { 0x9999 } else { 0x7a00 })
        });
        stub(&mut e, 0x007a_f430, 0x1a00);
        e.mem.set_u8(arrows[2] + ADDED_TO_INVENTORY, 1);
        e.mem.set_u8(arrows[3] + ADDED_TO_INVENTORY, 1);
        stub_all(&mut e, &[0x008c_c6d0, ARROW_SLOTS + 0x10]);
        // Up to two arrows of item 0x1a00 on 0x7a00, added ones only, dropped.
        e.call_log = Some(vec![]);
        e.call(0x008c_e180, &[0x1a00, 2, 0x7a00, 0, 1]);
        assert_eq!(
            calls_to(&e, 0x008c_c6d0),
            vec![vec![arrows[2], 3], vec![arrows[3], 3]]
        );
        // The second list starts at its end index: nothing more is read.
        assert_eq!(calls_to(&e, 0x0096_8670).len(), 4);
        // One arrow, any, deleted.
        e.call_log = Some(vec![]);
        e.call(0x008c_e180, &[0x1a00, 1, 0x7a00, 1, 0]);
        assert_eq!(calls_to(&e, ARROW_SLOTS + 0x10), vec![vec![arrows[1], 1]]);
        assert!(calls_to(&e, 0x008c_c6d0).is_empty());
    }

    #[test]
    fn fn_008ce370_reads_the_added_flag() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc8);
        assert_eq!(e.call(0x008c_e370, &[this]).u8(), 0);
        e.mem.set_u8(this + ADDED_TO_INVENTORY, 2);
        assert_eq!(e.call(0x008c_e370, &[this]).u8(), 2);
    }

    #[test]
    fn fn_008ce3b0_counts_the_saved_bytes() {
        let mut e = Engine::new();
        map_data(&mut e, 0x55, true);
        stub(&mut e, 0x0093_1ff0, 0x100);
        let this = e.mem.alloc(0xc8);
        // No record: 0x100 + 6 + 0x31 + 1 + 1.
        assert_eq!(e.call(0x008c_e3b0, &[this, 0]).u16(), 0x139);
        let record = e.mem.alloc(COLLISION_SIZE);
        e.mem.set_u32(this + COLLISION, record);
        assert_eq!(
            e.call(0x008c_e3b0, &[this, 0]).u16(),
            0x139 + 0x28 + 0x10 + 8
        );
        e.mem.set_u32(record + COLLISION_TYPE, 2);
        map_data(&mut e, 0x4f, false);
        assert_eq!(e.call(0x008c_e3b0, &[this, 0]).u16(), 0x100 + 0x31 + 0x28);
    }

    #[test]
    fn fn_008ce3b0_prints_the_size_when_asked() {
        let mut e = Engine::new();
        map_data(&mut e, 0x50, false);
        stub(&mut e, 0x0093_1ff0, 0x10);
        let debug = e.mem.alloc(4);
        e.mem.set_u8(debug, 1);
        stub(&mut e, SETTING_ADDRESS, debug);
        stub(&mut e, SAVING_FORM, 0);
        stub_all(&mut e, &[ERROR]);
        let this = e.mem.alloc(0xc8);
        e.call_log = Some(vec![]);
        e.call(0x008c_e3b0, &[this, 0]);
        assert_eq!(
            calls_to(&e, ERROR),
            vec![vec![0x0101_2c78, 0x31, 0x87e, SOURCE_FILE]]
        );
        let saving = e.mem.alloc(12);
        e.mem.set_u32(saving, 0x0001_2345);
        e.mem.write(saving + 5, &0x0000_0040u32.to_le_bytes());
        stub(&mut e, SAVING_FORM, saving);
        let form = object(&mut e, 0x20, OTHER_SLOTS);
        stub(&mut e, LOOKUP_FORM, form);
        stub(&mut e, OTHER_SLOTS + 0x130, 0x0abc);
        e.call_log = Some(vec![]);
        e.call(0x008c_e3b0, &[this, 0]);
        assert_eq!(
            calls_to(&e, ERROR),
            vec![vec![
                0x0101_2cb0,
                0x31,
                0x0001_2345,
                0x0abc,
                0x40,
                0x87e,
                SOURCE_FILE
            ]]
        );
    }

    /// The save/read doubles that keep what was written, in order.
    fn save_setup(e: &mut Engine, version: u8, blocks: bool) -> u32 {
        map_data(e, version, blocks);
        stub_all(
            e,
            &[
                0x0093_2070,
                SAVE_WRITE,
                SAVE_DATA,
                SAVE_NUMERIC_ID,
                CONSTRUCT_IN_PLACE,
                0x00a6_df40,
                PRINT_MESSAGE,
            ],
        );
        e.register(FORM_SAVE_ID, |_, a| ret(a[0] + 1));
        object(e, 0xc8, ARROW_SLOTS)
    }

    #[test]
    fn fn_008ce610_writes_the_arrow_in_a_block() {
        let mut e = Engine::new();
        let this = save_setup(&mut e, 0x55, true);
        let buffer = e.mem.alloc(0x10);
        let positions = [buffer, buffer + 4, buffer + 0x40];
        let mut next = 0usize;
        e.register_double(SAVE_POSITION, move |_, _| {
            let position = positions[next.min(2)];
            next += 1;
            ret(position)
        });
        e.mem.set_u32(this + SHOOTER, 0x0a00);
        let record = e.mem.alloc(COLLISION_SIZE);
        e.mem.set_u32(this + COLLISION, record);
        e.mem.set_u32(record + COLLISION_TYPE, 1);
        e.mem.set_u32(record + COLLISION_COLLIDEE, 0x0b00);
        e.mem
            .set_u32(record + COLLISION_CLOSEST_OBJECT, 0x0003_0004);
        e.mem.set_u8(this + NEEDS_FINISH_INIT_LOAD_GAME, 1);
        e.call_log = Some(vec![]);
        e.call(0x008c_e610, &[this, 7]);
        assert_eq!(calls_to(&e, 0x0093_2070), vec![vec![this, 7]]);
        let writes = calls_to(&e, SAVE_WRITE);
        assert_eq!(writes[0][2], 4);
        assert_eq!(writes[1][2], 2);
        assert_eq!(writes[2][1..], [this + STATE, 4]);
        assert_eq!(writes[3][1..], [record, 4]);
        assert_eq!(writes[4][2], 0x10);
        let data = calls_to(&e, SAVE_DATA);
        assert_eq!(data[0][1..], [this + 0x90, 4]);
        assert_eq!(data[5][1..], [this + DELTA, 0xc]);
        assert_eq!(data[6][2], 1);
        // The ids: shooter, enchantment, poison and collidee.
        assert_eq!(calls_to(&e, SAVE_NUMERIC_ID).len(), 4);
        // The two halves of the pending closest object.
        let halves = &data[10..12];
        assert_eq!(halves[0][2], 2);
        assert_eq!(e.mem.u16(halves[0][1]), 3);
        assert_eq!(e.mem.u16(halves[1][1]), 4);
        assert_eq!(data[12][1..], [this + ADDED_TO_INVENTORY, 1]);
        assert_eq!(data[13][1..], [this + BOW_IGNORES_WEAPONS_RESIST, 1]);
        // The block size: from the size word's position to the end.
        assert_eq!(e.mem.u16(buffer + 4), 0x3c);
        assert!(calls_to(&e, PRINT_MESSAGE).is_empty());
    }

    #[test]
    fn fn_008ce610_counts_the_collision_objects_of_a_loaded_actor() {
        let mut e = Engine::new();
        let this = save_setup(&mut e, 0x40, false);
        stub(&mut e, SAVE_POSITION, 0);
        let record = e.mem.alloc(COLLISION_SIZE);
        e.mem.set_u32(this + COLLISION, record);
        e.mem.set_u32(record + COLLISION_COLLIDEE, 0x0b00);
        e.mem.set_u32(record + COLLISION_CLOSEST_OBJECT, 0x0c00);
        stub(&mut e, REFERENCE_3D, 0x3d00);
        stub(&mut e, 0x004b_66d0, 9);
        stub(&mut e, 0x004a_8b00, 0x0c01);
        stub(&mut e, 0x004b_6810, 5);
        e.call_log = Some(vec![]);
        e.call(0x008c_e610, &[this, 0]);
        assert_eq!(calls_to(&e, 0x004b_66d0), vec![vec![0x3d00, 1, 1, 1]]);
        assert_eq!(calls_to(&e, 0x004a_8b00), vec![vec![0x0c00]]);
        assert_eq!(
            calls_to(&e, 0x004b_6810),
            vec![vec![0x3d00, 0x0c01, 1, 1, 1]]
        );
        let data = calls_to(&e, SAVE_DATA);
        let count = data.len();
        assert_eq!(e.mem.u16(data[count - 2][1]), 9);
        assert_eq!(e.mem.u16(data[count - 1][1]), 5);
        // Version 0x40: no quaternion, no flags.
        assert!(calls_to(&e, 0x00a6_df40).is_empty());
    }

    /// The load doubles: every read fills its bytes from `values` in order.
    fn load_setup(e: &mut Engine, version: u8, blocks: bool, values: Vec<u32>) -> u32 {
        map_data(e, version, blocks);
        stub_all(
            e,
            &[
                0x0093_20e0,
                CONSTRUCT_IN_PLACE,
                0x004f_0180,
                PRINT_MESSAGE,
                0x0085_7bd0,
            ],
        );
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(COLLISION_CONSTRUCT, |_, a| ret(a[0]));
        e.register(0x0040_3d30, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            Ret::default()
        });
        let mut next = 0usize;
        let reader = move |e: &mut Engine, a: &[u32]| {
            let value = values.get(next).copied().unwrap_or(0);
            next += 1;
            let bytes = value.to_le_bytes();
            let size = a[2] as usize;
            let data: Vec<u8> = (0..size)
                .map(|i| if i < 4 { bytes[i] } else { 0 })
                .collect();
            e.mem.write(a[1], &data);
            Ret::default()
        };
        let shared = std::rc::Rc::new(std::cell::RefCell::new(reader));
        for callee in [SAVE_READ, LOAD_DATA, LOAD_NUMERIC_ID] {
            let shared = shared.clone();
            e.register_double(callee, move |e, a| (shared.borrow_mut())(e, a));
        }
        object(e, 0xc8, ARROW_SLOTS)
    }

    #[test]
    fn fn_008ceb50_reads_the_arrow_back() {
        let mut e = Engine::new();
        let values = vec![
            BLOCK_TAG, 0x30, // block tag, size
            2,    // eState
            1, 2, 3, 4, 5, 6, // the five floats and DeltaS
            0x0a01, 0, 0x0b01, // shooter, enchantment, poison ids
            1,      // record flag
            0,      // record type
            7, 8, 9,      // point, normal, velocity
            0x11,   // quaternion
            0x0c01, // collidee id
            0x0003, 0xfffe, // halves (the low one sign-extended)
            1, 1, // the two flags
        ];
        let this = load_setup(&mut e, 0x55, true, values);
        stub(&mut e, SAVE_POSITION, 0x100);
        stub(&mut e, LOADING_FORM, 0);
        e.call_log = Some(vec![]);
        e.call(0x008c_eb50, &[this, 5, 6]);
        assert_eq!(calls_to(&e, 0x0093_20e0), vec![vec![this, 5, 6]]);
        assert_eq!(e.mem.u32(this + STATE), 2);
        assert_eq!(e.mem.u32(this + SHOOTER), 0x0a01);
        assert_eq!(e.mem.u32(this + POISON), 0x0b01);
        let record = e.mem.u32(this + COLLISION);
        assert_eq!(e.mem.u32(record + COLLISION_COLLIDEE), 0x0c01);
        assert_eq!(e.mem.u32(record + COLLISION_CLOSEST_OBJECT), 0x0002_fffe);
        assert_eq!(e.mem.f32(record + COLLISION_ROTATION), 1.0);
        assert_eq!(calls_to(&e, 0x004f_0180)[0][1], record + COLLISION_ROTATION);
        assert_eq!(e.mem.u8(this + ADDED_TO_INVENTORY), 1);
        // The block ends at 0x100 + 0x30, the position is 0x100: short read.
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![0x0101_5440, 0x30, SOURCE_FILE, 0x965, 0x55]]
        );
    }

    #[test]
    fn fn_008ceb50_reads_an_old_record_name() {
        let mut e = Engine::new();
        let values = vec![
            1, // eState 1: a record before version 0x20
            1,
            2,
            3,
            4,
            5,
            6,
            0,
            0,
            0, // floats, DeltaS, ids
            1, // record type
            7,
            8,
            9,           // point, normal, velocity
            0x0c01,      // collidee id
            3,           // name length
            0x0063_6261, // "abc"
        ];
        let this = load_setup(&mut e, 0x1f, false, values);
        e.call_log = Some(vec![]);
        e.call(0x008c_eb50, &[this, 0, 0]);
        assert_eq!(calls_to(&e, 0x0085_7bd0), vec![vec![0x5a1e, 8]]);
        let record = e.mem.u32(this + COLLISION);
        assert_eq!(e.mem.u32(record + COLLISION_TYPE), 1);
        let name = e.mem.u32(record + COLLISION_CLOSEST_OBJECT);
        assert_eq!(e.mem.cstr(name), b"abc".to_vec());
        assert!(calls_to(&e, PRINT_MESSAGE).is_empty());
    }

    #[test]
    fn fn_008cf2c0_resolves_the_loaded_ids() {
        let mut e = Engine::new();
        let this = object(&mut e, 0xc8, ARROW_SLOTS);
        e.mem.set_u32(this + SHOOTER, 0x0a01);
        e.mem.set_u32(this + POISON, 0x0b01);
        let record = e.mem.alloc(COLLISION_SIZE);
        e.mem.set_u32(this + COLLISION, record);
        e.mem.set_u32(record + COLLISION_COLLIDEE, 0x0c01);
        e.register(LOOKUP_FORM, |_, a| ret(a[0] + 0x1000));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0] + 0x10));
        stub(&mut e, CURRENT_PROCESS_TYPE, 2);
        stub_all(&mut e, &[0x0093_2490, REMOVE_REFERENCE, ADD_REFERENCE]);
        e.call_log = Some(vec![]);
        e.call(0x008c_f2c0, &[this, 3, 4]);
        assert_eq!(calls_to(&e, 0x0093_2490), vec![vec![this, 3, 4]]);
        assert_eq!(e.mem.u32(this + SHOOTER), 0x1a11);
        assert_eq!(e.mem.u32(this + ENCHANTMENT), 0);
        assert_eq!(e.mem.u32(this + POISON), 0x1b11);
        assert_eq!(e.mem.u32(record + COLLISION_COLLIDEE), 0x1c11);
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST),
            vec![
                vec![0x1a01, 0, FORM_TYPE, ACTOR_TYPE, 0],
                vec![0x1b01, 0, FORM_TYPE, ALCHEMY_TYPE, 0],
                vec![0x1c01, 0, FORM_TYPE, REFERENCE_TYPE, 0]
            ]
        );
        assert_eq!(
            calls_to(&e, REMOVE_REFERENCE),
            vec![vec![PROCESS_LISTS, this, 2]]
        );
        assert_eq!(
            calls_to(&e, ADD_REFERENCE),
            vec![vec![PROCESS_LISTS, this, 2, 0, 0, 0]]
        );
    }

    #[test]
    fn fn_008cf440_defers_or_finishes_the_load() {
        let mut e = Engine::new();
        map_data(&mut e, 0x55, false);
        let this = object(&mut e, 0xc8, ARROW_SLOTS);
        stub_all(
            &mut e,
            &[
                0x0046_a010,
                0x008c_aea0,
                0x0093_2520,
                0x0085_f710,
                PRINT_MESSAGE,
                ARROW_SLOTS + 0xc4,
            ],
        );
        stub(&mut e, REFERENCE_3D, 0);
        e.call(0x008c_f440, &[this, 9, 0]);
        assert_eq!(e.mem.u8(this + NEEDS_FINISH_INIT_LOAD_GAME), 1);
        assert_eq!(e.mem.u32(this + 0xc4), 9);
        // With 3D: the halves still match the collidee's count.
        let record = e.mem.alloc(COLLISION_SIZE);
        e.mem.set_u32(this + COLLISION, record);
        e.mem.set_u32(record + COLLISION_COLLIDEE, 0x0b00);
        e.mem
            .set_u32(record + COLLISION_CLOSEST_OBJECT, 0x0004_0002);
        e.mem.set_u32(this + SHOOTER, 0x0a00);
        e.register(REFERENCE_3D, |_, a| ret(a[0] + 0x10));
        stub(&mut e, 0x004b_66d0, 4);
        stub(&mut e, 0x004b_6980, 0x0e00);
        stub(&mut e, 0x0044_ddc0, 0x0e01);
        e.call_log = Some(vec![]);
        e.call(0x008c_f440, &[this, 1, 2]);
        assert_eq!(
            calls_to(&e, 0x008c_aea0),
            vec![vec![this, 0x0a00, 1.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x0093_2520), vec![vec![this, 1, 2]]);
        assert_eq!(calls_to(&e, 0x004b_66d0), vec![vec![0x0b10, 1, 1, 1]]);
        assert_eq!(
            calls_to(&e, 0x004b_6980),
            vec![vec![0x0b10, 0x0004_0002, 1, 1, 1]]
        );
        assert_eq!(e.mem.u32(record + COLLISION_CLOSEST_OBJECT), 0x0e01);
        assert_eq!(e.mem.u32(record + COLLISION_TYPE), 0);
        assert_eq!(calls_to(&e, 0x0085_f710), vec![vec![0x5a1e, this]]);
        // A changed count: the message, and the record becomes type 3.
        e.mem
            .set_u32(record + COLLISION_CLOSEST_OBJECT, 0x0005_0002);
        let collidee = object(&mut e, 0x20, OTHER_SLOTS);
        e.mem.set_u32(record + COLLISION_COLLIDEE, collidee);
        e.register(FORM_SAVE_ID, |_, _| ret(0x0001_2345));
        stub(&mut e, OTHER_SLOTS + 0x130, 0x0abc);
        e.call_log = Some(vec![]);
        e.call(0x008c_f440, &[this, 1, 2]);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![0x0108_5d9c, 0x0abc, 0x0001_2345]]
        );
        assert_eq!(e.mem.u32(record + COLLISION_TYPE), 3);
    }

    #[test]
    fn fn_008cf440_drops_an_arrow_whose_collidee_is_gone() {
        let mut e = Engine::new();
        map_data(&mut e, 0x55, false);
        let this = object(&mut e, 0xc8, ARROW_SLOTS);
        let record = e.mem.alloc(COLLISION_SIZE);
        e.mem.set_u32(this + COLLISION, record);
        e.mem.set_u32(record + COLLISION_COLLIDEE, 0x0b00);
        e.mem.set_u32(record + COLLISION_CLOSEST_OBJECT, 0x0c00);
        stub(&mut e, REFERENCE_3D, 0);
        stub(&mut e, 0x0044_0da0, 1);
        stub(&mut e, 0x0044_0d80, 1);
        stub_all(&mut e, &[0x0046_a010, ARROW_SLOTS + 0xc4]);
        e.call_log = Some(vec![]);
        e.call(0x008c_f440, &[this, 9, 0]);
        assert_eq!(calls_to(&e, ARROW_SLOTS + 0xc4), vec![vec![this, 1]]);
        assert_eq!(e.mem.u32(record + COLLISION_CLOSEST_OBJECT), 0);
        assert_eq!(e.mem.u8(this + NEEDS_FINISH_INIT_LOAD_GAME), 0);
    }

    #[test]
    fn fn_008cf740_frees_the_record_on_the_test() {
        let mut e = Engine::new();
        map_data(&mut e, 0x55, false);
        let this = e.mem.alloc(0xc8);
        e.mem.set_u32(this + COLLISION, 0x0c0c);
        stub_all(&mut e, &[0x0093_2750, OPERATOR_DELETE]);
        stub(&mut e, 0x0055_f5b0, 0);
        e.call_log = Some(vec![]);
        e.call(0x008c_f740, &[this, 3]);
        assert_eq!(calls_to(&e, 0x0093_2750), vec![vec![this, 3]]);
        assert_eq!(e.mem.u32(this + COLLISION), 0x0c0c);
        stub(&mut e, 0x0055_f5b0, 1);
        e.call(0x008c_f740, &[this, 3]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x0c0c]]);
        assert_eq!(e.mem.u32(this + COLLISION), 0);
    }

    fn process_setup(e: &mut Engine, level: u32) -> (u32, u32) {
        let this = object(e, 0xc8, ARROW_SLOTS);
        let old = object(e, 0x30, OTHER_SLOTS);
        e.mem.set_u32(old + 0x28, level);
        e.mem.set_u32(this + PROCESS, old);
        e.register(PROCESS_LEVEL, |e, a| ret(e.mem.u32(a[0] + 0x28)));
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        stub_all(
            e,
            &[
                REMOVE_REFERENCE,
                ADD_REFERENCE,
                0x7400_0004,
                0x7400_0048,
                OTHER_SLOTS,
                ARROW_SLOTS + 0x1fc,
            ],
        );
        (this, old)
    }

    #[test]
    fn fn_008cf7a0_moves_to_a_new_high_process() {
        let mut e = Engine::new();
        let (this, old) = process_setup(&mut e, 1);
        e.register(0x008d_7510, |e, a| {
            let table = e.mem.alloc(0x100);
            for slot in (0..0x100).step_by(4) {
                e.mem.set_u32(table + slot, 0x7400_0000 + slot);
            }
            e.mem.set_u32(a[0], table);
            ret(a[0])
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x008c_f7a0, &[this]).bool());
        let created = e.mem.u32(this + PROCESS);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x46c]]);
        assert_eq!(
            calls_to(&e, REMOVE_REFERENCE),
            vec![vec![PROCESS_LISTS, this, 1]]
        );
        assert_eq!(calls_to(&e, 0x7400_0004), vec![vec![created, old]]);
        assert_eq!(calls_to(&e, OTHER_SLOTS), vec![vec![old, 1]]);
        assert_eq!(
            calls_to(&e, ADD_REFERENCE),
            vec![vec![PROCESS_LISTS, this, 0, 0, 0, 0]]
        );
        assert_eq!(calls_to(&e, ARROW_SLOTS + 0x1fc), vec![vec![this, 0]]);
        assert_eq!(calls_to(&e, 0x7400_0048), vec![vec![created]]);
        // Already high: nothing.
        e.call_log = Some(vec![]);
        assert!(e.call(0x008c_f7a0, &[this]).bool());
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn fn_008cf8f0_moves_to_middle_high() {
        let mut e = Engine::new();
        let (this, _) = process_setup(&mut e, 0);
        e.register(0x0091_3fe0, |e, a| {
            let table = e.mem.alloc(0x100);
            for slot in (0..0x100).step_by(4) {
                e.mem.set_u32(table + slot, 0x7400_0000 + slot);
            }
            e.mem.set_u32(a[0], table);
            e.mem.set_u32(a[0] + 0x28, 1);
            ret(a[0])
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x008c_f8f0, &[this]).bool());
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x25c]]);
        assert_eq!(
            calls_to(&e, ADD_REFERENCE),
            vec![vec![PROCESS_LISTS, this, 1, 0, 0, 0]]
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x008c_f8f0, &[this]).bool());
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn fn_008cfa40_names_the_loaded_node() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc8);
        stub(&mut e, 0x0093_35b0, 0x3d00);
        e.register(0x0043_8170, |_, a| ret(a[0]));
        stub_all(&mut e, &[0x00a5_b950, 0x0043_81b0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008c_fa40, &[this, 0x1ff]).u32(), 0x3d00);
        assert_eq!(calls_to(&e, 0x0093_35b0), vec![vec![this, 0xff]]);
        let name = calls_to(&e, 0x0043_8170)[0][0];
        assert_eq!(calls_to(&e, 0x0043_8170), vec![vec![name, 0x0102_031c]]);
        assert_eq!(calls_to(&e, 0x00a5_b950), vec![vec![0x3d00, name]]);
        assert_eq!(calls_to(&e, 0x0043_81b0), vec![vec![name]]);
        stub(&mut e, 0x0093_35b0, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008c_fa40, &[this, 0]).u32(), 0);
        assert!(calls_to(&e, 0x0043_8170).is_empty());
    }
}
