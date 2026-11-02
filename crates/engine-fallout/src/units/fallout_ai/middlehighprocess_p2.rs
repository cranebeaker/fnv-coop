//! `fallout/ai/middlehighprocess.cpp` (Xbox PDB source unit), part 2: its functions from `00924b90` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::middlehighprocess`]; anything public there may be used here.

#[allow(unused_imports)]
use super::middlehighprocess::*;
#[allow(unused_imports)]
use crate::prelude::*;

// ---------------------------------------------------------------------
// Constants and helpers of this part.
// ---------------------------------------------------------------------

/// `TESSaveLoadGame::UseSaveGameBlocks` (Xbox PDB), `(save/load game)`.
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
/// Reads `size` bytes of the save into an address `(save/load game, data,
/// size)`.
const READ_BYTES: u32 = 0x0085_79e0;
/// `TESSaveLoadGame::LoadNumericID` (Xbox PDB) `(save/load game, out, 4)`.
const READ_NUMERIC_ID: u32 = 0x0085_7aa0;
/// Skips `count` bytes of the save `(save/load game, count)`.
const SKIP_BYTES: u32 = 0x0085_7bd0;
/// The save/load object's buffer position `(save/load game)`.
const BUFFER_POSITION: u32 = 0x0082_5c00;
/// The save format version byte `(save/load game)`.
const CURRENT_VERSION: u32 = 0x008d_f040;
/// `005b5e40`, cdecl with variable arguments: the SAVELOAD warnings.
const SAVELOAD_WARNING: u32 = 0x005b_5e40;
/// The block tag (`KOLB`) written before and checked after a block.
const BLOCK_TAG: u32 = 0x424c_4f4b;
/// `SAVELOAD: (LoadGame Buffer error) Block Header is incorrect ...` with
/// the record being loaded, and without one.
const BAD_HEADER_FORM_FORMAT: u32 = 0x0101_5718;
const BAD_HEADER_FORMAT: u32 = 0x0101_56a8;
/// `SAVELOAD: LoadGame Buffer overrun of %i bytes ...` and `underrun`, with
/// the record being loaded and without one.
const OVERRUN_FORM_FORMAT: u32 = 0x0101_5588;
const UNDERRUN_FORM_FORMAT: u32 = 0x0101_5500;
const OVERRUN_FORMAT: u32 = 0x0101_54a0;
const UNDERRUN_FORMAT: u32 = 0x0101_5440;
/// `MiddleLowProcess::LoadGame` (Xbox PDB), `(process, flags, flags, object)`.
const MIDDLE_LOW_LOAD_GAME: u32 = 0x0092_e730;
/// `LowProcess::InitLoadGame` (Xbox PDB), `(process, a, b, object)`.
const LOW_INIT_LOAD_GAME: u32 = 0x0091_0250;
/// `MagicItem::GetMagicItemByNumericID` (Xbox PDB), cdecl `(id)`.
const MAGIC_ITEM_BY_NUMERIC_ID: u32 = 0x0040_a250;
/// `MagicTarget::GetMagicTargetByNumericID` (Xbox PDB), cdecl `(id)`.
const MAGIC_TARGET_BY_NUMERIC_ID: u32 = 0x0082_5550;
/// activeeffect.cpp, cdecl `(effect list, actor)`: resolves the saved
/// effects' references after a load.
const ACTIVE_EFFECTS_INIT_LOAD: u32 = 0x0080_60e0;
/// activeeffect.cpp, cdecl `(effect list, actor)`: reads the saved
/// effects.
const ACTIVE_EFFECTS_LOAD: u32 = 0x0080_5d40;
/// `Actor::ReloadTargets` (Xbox PDB), `(actor, 0)`.
const ACTOR_RELOAD_TARGETS: u32 = 0x008b_0b00;
/// `(reference)`: the reference's loaded 3D (the engine has it as
/// `fn_0043fcd0`; no engine-map name), 0 without one.
const REFERENCE_3D: u32 = 0x0043_fcd0;
/// teswater.cpp, `(player)`: tested for the player before its lighting
/// property is replaced.
const PLAYER_LIGHTING_TEST: u32 = 0x004e_af60;
/// tes.cpp, `(this, index)`: the pointer at `index` (0 when out of range).
const INDEXED_POINTER: u32 = 0x0045_bc00;
/// `Actor::InitLightingPropertyPtr` (Xbox PDB), `(actor, property)`.
const ACTOR_INIT_LIGHTING_PROPERTY: u32 = 0x008b_0bd0;
/// `RagDollData::RagDollData` (Xbox PDB), `(block)`.
const RAG_DOLL_DATA_CONSTRUCT: u32 = 0x004d_9330;
/// ragdolldata.cpp, `(ragdoll)`: reads the record from the save.
const RAG_DOLL_DATA_LOAD: u32 = 0x004d_a3f0;
/// extradataobjects.cpp, `(object, 1)`: the delete a ragdoll goes
/// through.
const RAG_DOLL_DATA_DELETE: u32 = 0x0043_2da0;
/// animation.cpp, cdecl `(actor, word)`: reads the animation block.
const ANIMATION_LOAD_BLOCK: u32 = 0x0049_ab00;

fn save_load(e: &Engine) -> u32 {
    e.global(TES_SAVE_LOAD_GAME)
}

fn use_save_game_blocks(e: &mut Engine) -> bool {
    let save = save_load(e);
    e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool()
}

/// Reads `size` bytes of the save into `destination`.
fn read_bytes(e: &mut Engine, destination: u32, size: u32) {
    let save = save_load(e);
    e.call(READ_BYTES, &args![save, destination, size]);
}

/// Reads a saved numeric id (4 bytes) and answers it.
fn read_numeric_id(e: &mut Engine) -> u32 {
    e.with_stack(4, |e, cell| {
        let save = save_load(e);
        e.call(READ_NUMERIC_ID, &args![save, cell, 4u32]);
        e.mem.u32(cell.addr())
    })
}

/// Reads `size` (at most 4) bytes of the save into a zeroed word.
fn read_value(e: &mut Engine, size: u32) -> u32 {
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), 0);
        read_bytes(e, cell.addr(), size);
        e.mem.u32(cell.addr())
    })
}

fn current_version(e: &mut Engine) -> u32 {
    let save = save_load(e);
    u32::from(e.call(CURRENT_VERSION, &args![save]).u8())
}

fn buffer_position(e: &mut Engine) -> u32 {
    let save = save_load(e);
    e.call(BUFFER_POSITION, &args![save]).u32()
}

fn loading_record(e: &mut Engine) -> u32 {
    let save = save_load(e);
    e.call(PACKAGE_WORD_84, &args![save]).u32()
}

fn skip_bytes(e: &mut Engine, count: u32) {
    let save = save_load(e);
    e.call(SKIP_BYTES, &args![save, count]);
}

fn dynamic_cast(e: &mut Engine, object: u32, source: u32, target: u32) -> u32 {
    e.call(DYNAMIC_CAST, &args![object, 0u32, source, target, 0u32])
        .u32()
}

/// The form with `id` cast to `target` (a type descriptor):
/// `LOOKUP_FORM` then `__RTDynamicCast` from `TESForm`.
fn form_cast(e: &mut Engine, id: u32, target: u32) -> u32 {
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    dynamic_cast(e, form, TYPE_TES_FORM, target)
}

/// The start of a save block: with save game blocks, reads the `KOLB` tag
/// (warning at `line` when it is wrong, with the record being loaded or the
/// current version), notes the buffer position and reads the 16-bit block
/// length. Answers the position and the length (0, 0 without blocks).
fn load_block_header(e: &mut Engine, line: u32) -> (u32, u16) {
    if !use_save_game_blocks(e) {
        return (0, 0);
    }
    if read_value(e, 4) != BLOCK_TAG {
        let record = loading_record(e);
        if record != 0 {
            let id = e.mem.u32(record);
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            let flags = e.mem.u32(record + 5);
            let version = u32::from(e.mem.u8(record + 9));
            let name = e.vcall(form, 0x130, &[]).u32();
            let id = e.mem.u32(record);
            e.call(
                SAVELOAD_WARNING,
                &args![
                    BAD_HEADER_FORM_FORMAT,
                    SOURCE_FILE,
                    line,
                    id,
                    name,
                    version,
                    flags
                ],
            );
        } else {
            let version = current_version(e);
            e.call(
                SAVELOAD_WARNING,
                &args![BAD_HEADER_FORMAT, SOURCE_FILE, line, version],
            );
        }
    }
    let start = buffer_position(e);
    let length = read_value(e, 2) as u16;
    (start, length)
}

/// The end of a save block: with save game blocks, compares the position
/// with where the block should end and warns of an overrun or underrun at
/// `line`.
fn check_block_end(e: &mut Engine, line: u32, start: u32, length: u16) {
    if !use_save_game_blocks(e) {
        return;
    }
    let end = buffer_position(e);
    let record = loading_record(e);
    let expected = u32::from(length).wrapping_add(start);
    if record != 0 {
        let id = e.mem.u32(record);
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let (format, bytes) = if end > expected {
            (OVERRUN_FORM_FORMAT, end.wrapping_sub(expected))
        } else if end < expected {
            (UNDERRUN_FORM_FORMAT, expected.wrapping_sub(end))
        } else {
            return;
        };
        let flags = e.mem.u32(record + 5);
        let version = u32::from(e.mem.u8(record + 9));
        let name = e.vcall(form, 0x130, &[]).u32();
        let id = e.mem.u32(record);
        e.call(
            SAVELOAD_WARNING,
            &args![format, bytes, SOURCE_FILE, line, id, name, version, flags],
        );
    } else {
        let (format, bytes) = if end > expected {
            (OVERRUN_FORMAT, end.wrapping_sub(expected))
        } else if end < expected {
            (UNDERRUN_FORMAT, expected.wrapping_sub(end))
        } else {
            return;
        };
        let version = current_version(e);
        e.call(
            SAVELOAD_WARNING,
            &args![format, bytes, SOURCE_FILE, line, version],
        );
    }
}

// Translated from 00924b90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00924b90 - the C++ exception-unwinding frame (the unwind of the RagDollData allocation) is not translated
/// `MiddleHighProcess::LoadGame` (Xbox PDB): `(process, flags, flags,
/// object)`. Loads the low part (`0092e730`), then inside a save block
/// (header line 0x1c20, end check line 0x1ccd) the process fields in save
/// order: the bytes at +0x134 and +0x135; from version 0x34 the byte at
/// +0x168 and the word at +0x170; from 0x4D the word at +0x174. When bit
/// 0x20 of either flag word is set the process's slot +0x544 is called,
/// the actor's targets are reloaded (`008b0b00`, 0) and its lighting
/// property is initialised (`008b0bd0`) from `0043fcd0` (the player gets
/// `0045bc00` when `004eaf60` is false). Then the numeric ids for +0x158
/// and +0x140, the fields +0x108 (4 bytes), +0x1DA, +0xFC (12 bytes),
/// +0xDC, +0x13D, +0x144, and the furniture mark (+0x148: heading, byte,
/// point). A zero byte makes the process's slot +0x420 run with the actor;
/// otherwise a ragdoll record of a save before version 0x2F is read and
/// dropped. The byte at +0x13C, the animation block (flag 0x10000000,
/// `0049ab00` with the word at +0x1C0), the numeric ids for +0x160 and
/// +0x1BC, the effect list `00805d40`, then the version-gated fields
/// (+0xE0 from 0x45, +0x188 and +0x189 from 0x49, the word at +0xD8 and
/// the id list of +0xC8 from 0x65, +0x18B from 0x6D, the numeric id +0x164
/// from 0x71). The package loading guarded by `flags & 0` is compiled out
/// and left out here.
pub fn middle_high_process_load_game(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    flags: u32,
    flags_2: u32,
    object: u32,
) {
    let p = this.addr();
    let actor = dynamic_cast(e, object, TYPE_MOBILE_OBJECT, TYPE_ACTOR);
    e.call(MIDDLE_LOW_LOAD_GAME, &args![p, flags, flags_2, object]);
    let (start, length) = load_block_header(e, 0x1c20);
    read_bytes(e, p + 0x134, 1);
    read_bytes(e, p + 0x135, 1);
    if current_version(e) >= 0x34 {
        read_bytes(e, p + 0x168, 1);
        read_bytes(e, p + 0x170, 4);
    }
    if current_version(e) >= 0x4d {
        read_bytes(e, p + 0x174, 4);
    }
    if flags & 0x20 != 0 || flags_2 & 0x20 != 0 {
        e.vcall(p, 0x544, &args![object]);
        e.call(ACTOR_RELOAD_TARGETS, &args![actor, 0u32]);
        let lighting = e.call(REFERENCE_3D, &args![object]).u32();
        if lighting != 0 {
            let mut property = lighting;
            if object == e.global::<u32>(PLAYER)
                && !e.call(PLAYER_LIGHTING_TEST, &args![object]).bool()
            {
                property = e.call(INDEXED_POINTER, &args![property, 0u32]).u32();
            }
            e.call(ACTOR_INIT_LIGHTING_PROPERTY, &args![actor, property]);
        }
    }
    let id = read_numeric_id(e);
    e.mem.set_u32(p + 0x158, id);
    read_bytes(e, p + 0x108, 4);
    read_bytes(e, p + 0x1da, 1);
    if current_version(e) < 0x18 {
        read_value(e, 1);
    }
    read_bytes(e, p + 0xfc, 0xc);
    read_bytes(e, p + 0xdc, 4);
    read_bytes(e, p + 0x13d, 1);
    read_bytes(e, p + 0x144, 1);
    let id = read_numeric_id(e);
    e.mem.set_u32(p + 0x140, id);
    e.with_stack(12, |e, point| {
        e.call(IDENTITY, &args![point]);
        let heading = read_value(e, 4);
        let mark_byte = read_value(e, 1);
        read_bytes(e, point.addr(), 0xc);
        // The heading passes through the x87 stack (`FLD`/`FSTP`).
        e.call(
            FURNITURE_MARK_SET_HEADING,
            &args![p + 0x148, x87_quiet_bits(heading)],
        );
        e.call(SET_BYTE_E, &args![p + 0x148, mark_byte]);
        e.call(COPY_POINT, &args![p + 0x148, point]);
    });
    let present = read_value(e, 1) as u8;
    if present == 0 {
        let actor = dynamic_cast(e, object, TYPE_MOBILE_OBJECT, TYPE_ACTOR);
        e.vcall(p, 0x420, &args![actor]);
    } else if flags & 4 == 0 && current_version(e) < 0x2f && read_value(e, 1) as u8 != 0 {
        // A ragdoll record of an old save: made, read and deleted.
        let block = e.call(ALLOCATE, &args![0x14u32]).u32();
        let ragdoll = if block == 0 {
            0
        } else {
            e.call(RAG_DOLL_DATA_CONSTRUCT, &args![block]).u32()
        };
        e.call(RAG_DOLL_DATA_LOAD, &args![ragdoll]);
        if ragdoll != 0 {
            e.call(RAG_DOLL_DATA_DELETE, &args![ragdoll, 1u32]);
        }
    }
    e.mem.set_u8(p + 0x13c, present);
    if flags & 0x1000_0000 != 0 {
        let word = e.mem.u32(p + 0x1c0);
        e.call(ANIMATION_LOAD_BLOCK, &args![object, word]);
    }
    let id = read_numeric_id(e);
    e.mem.set_u32(p + 0x160, id);
    let id = read_numeric_id(e);
    e.mem.set_u32(p + 0x1bc, id);
    let effects = e.mem.u32(p + 0x1b8);
    e.call(ACTIVE_EFFECTS_LOAD, &args![effects, actor]);
    if current_version(e) >= 0x45 {
        read_bytes(e, p + 0xe0, 1);
    }
    if current_version(e) >= 0x49 {
        read_bytes(e, p + 0x188, 1);
        read_bytes(e, p + 0x189, 1);
    }
    if current_version(e) >= 0x65 {
        if current_version(e) < 0x68 {
            skip_bytes(e, 4);
        }
        read_bytes(e, p + 0xd8, 4);
        if current_version(e) < 0x68 {
            skip_bytes(e, 4);
        }
        let count = read_value(e, 2) as u16;
        for _ in 0..count {
            let id = read_numeric_id(e);
            e.with_stack(4, |e, cell| {
                e.mem.set_u32(cell.addr(), id);
                e.call(LIST_ADD, &args![p + 0xc8, cell]);
            });
        }
    }
    if current_version(e) >= 0x6d {
        read_bytes(e, p + 0x18b, 1);
    }
    if current_version(e) >= 0x71 {
        let id = read_numeric_id(e);
        e.mem.set_u32(p + 0x164, id);
    }
    check_block_end(e, 0x1ccd, start, length);
}

// Translated from 00925490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::InitLoadGame` (Xbox PDB): `(process, a, b, object)`.
/// Runs `LowProcess::InitLoadGame` (`00910250`) and the process's slot
/// +0x20C (its result only guarded a compiled-out package call), then
/// turns the saved form ids into pointers: +0x158 into an `Actor`, +0x140
/// into a `TESObjectREFR`, +0x160 and (from version 0x71) +0x164 into
/// magic items, +0x1BC into a magic target; resolves the effect list at
/// +0x1B8 (`008060e0`); and from version 0x65 walks the id list at +0xC8,
/// replacing each id by its `TESObjectREFR` and dropping the entries that
/// no longer resolve.
pub fn middle_high_process_init_load_game(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    flags: u32,
    flags_2: u32,
    object: u32,
) {
    let p = this.addr();
    let actor = dynamic_cast(e, object, TYPE_MOBILE_OBJECT, TYPE_ACTOR);
    e.call(LOW_INIT_LOAD_GAME, &args![p, flags, flags_2, object]);
    e.vcall(p, 0x20c, &[]);
    let id = e.mem.u32(p + 0x158);
    if id != 0 {
        let resolved = form_cast(e, id, TYPE_ACTOR);
        e.mem.set_u32(p + 0x158, resolved);
    }
    let id = e.mem.u32(p + 0x140);
    if id != 0 {
        let resolved = form_cast(e, id, TYPE_TES_OBJECT_REFR);
        e.mem.set_u32(p + 0x140, resolved);
    }
    let id = e.mem.u32(p + 0x160);
    if id != 0 {
        let resolved = e.call(MAGIC_ITEM_BY_NUMERIC_ID, &args![id]).u32();
        e.mem.set_u32(p + 0x160, resolved);
    }
    let id = e.mem.u32(p + 0x1bc);
    if id != 0 {
        let resolved = e.call(MAGIC_TARGET_BY_NUMERIC_ID, &args![id]).u32();
        e.mem.set_u32(p + 0x1bc, resolved);
    }
    let effects = e.mem.u32(p + 0x1b8);
    e.call(ACTIVE_EFFECTS_INIT_LOAD, &args![effects, actor]);
    if current_version(e) >= 0x65 {
        let mut node = p + 0xc8;
        let mut previous = 0;
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let slot = e.call(IDENTITY, &args![node]).u32();
            let id = e.mem.u32(slot);
            let mut resolved = 0;
            if id != 0 {
                resolved = form_cast(e, id, TYPE_TES_OBJECT_REFR);
                e.with_stack(4, |e, cell| {
                    e.mem.set_u32(cell.addr(), resolved);
                    e.call(NODE_SET_ITEM, &args![node, cell]);
                });
            }
            if resolved == 0 {
                if previous == 0 {
                    e.call(LIST_REMOVE_HEAD, &args![node]);
                } else {
                    e.with_stack(4, |e, cell| {
                        e.mem.set_u32(cell.addr(), id);
                        e.call(LIST_REMOVE, &args![previous, cell]);
                    });
                    node = e.call(WORD_4, &args![previous]).u32();
                }
            } else {
                previous = node;
                node = e.call(WORD_4, &args![node]).u32();
            }
        }
    }
    if current_version(e) >= 0x71 {
        let id = e.mem.u32(p + 0x164);
        if id != 0 {
            let resolved = e.call(MAGIC_ITEM_BY_NUMERIC_ID, &args![id]).u32();
            e.mem.set_u32(p + 0x164, resolved);
        }
    }
}

/// `BaseProcess::FinishInitLoadGame` (Xbox PDB), `(process, a, b, object)`.
const BASE_FINISH_INIT_LOAD_GAME: u32 = 0x008d_0e80;
/// `PlayerCharacter::GetAnimation` (Xbox PDB), `(player, first person)`.
const PLAYER_GET_ANIMATION: u32 = 0x0095_0a60;
/// `PlayerCharacter::GetBiped` (Xbox PDB), `(player, first person)`.
const PLAYER_GET_BIPED: u32 = 0x0095_0b00;
/// playercharacter.cpp, `(player, 0)`: the player's 3D (the reference's for 0,
/// the other for 1).
const PLAYER_GET_3D: u32 = 0x0095_0bb0;
/// `PlayerCharacter::SetFirstPerson` (Xbox PDB), `(player, flag)`.
const PLAYER_SET_FIRST_PERSON: u32 = 0x0095_0110;
/// tesfurniture.cpp, `(markers)`: the number of markers.
const MARKER_COUNT: u32 = 0x0050_9490;
/// tesfurniture.cpp, `(markers, index)`: the address of the marker's 16
/// byte entry.
const MARKER_ENTRY: u32 = 0x0050_94d0;
/// `BSFurnitureMarker::Find` (Xbox PDB), cdecl `(model)`.
const FURNITURE_MARKER_FIND: u32 = 0x00c5_4400;
/// `NiMatrix3::TransformVertices` (Xbox PDB), cdecl `(matrix, a, count, in,
/// out)`.
const TRANSFORM_VERTICES: u32 = 0x00a5_82f0;
/// `TESObjectREFR::GetOrientation` (Xbox PDB), `(reference, out)`.
const REFERENCE_GET_ORIENTATION: u32 = 0x0056_fa00;
/// `MobileObject::GetCharController` (Xbox PDB), `(actor)`.
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `bhkCharacterController::SetPosition` (Xbox PDB), `(controller, point)`.
const CONTROLLER_SET_POSITION: u32 = 0x0056_20e0;
/// tesfurniture.cpp, `(player)`: clears the player's furniture state.
const PLAYER_CLEAR_FURNITURE: u32 = 0x0050_98c0;
/// `bhkWorld::AdjustForceAddCount` (Xbox PDB), cdecl `(object_3d, 1, 1, 0)`.
const WORLD_ADJUST_FORCE_ADD_COUNT: u32 = 0x00c6_9fc0;
/// `bhkWorld::SetMotion` (Xbox PDB), cdecl `(object_3d, 1, 1, 0, 1)`.
const WORLD_SET_MOTION: u32 = 0x00c6_a350;
/// modelloader.cpp, `(out, 0.0, 0, 0)`: builds the 12 byte value the next
/// call takes.
const MOTION_VALUE_CONSTRUCT: u32 = 0x0043_d410;
/// `(3D object, value)` (`00a59c60`).
const WORLD_APPLY_MOTION_VALUE: u32 = 0x00a5_9c60;
/// activeeffect.cpp, cdecl `(effect list, actor)`.
const ACTIVE_EFFECTS_FINISH_LOAD: u32 = 0x0080_6130;
/// `TESSaveLoadGame::AddFormToDeferredDeletionsList` (Xbox PDB), `(save/load
/// game, form)`.
const ADD_FORM_TO_DEFERRED_DELETIONS: u32 = 0x0085_a3d0;

/// Gets the player into a furniture mark's pose at load: sets the
/// actor's marker state and sit/sleep state through the process's slot
/// +0x4D8, for the player also the first person flag.
fn finish_load_enter_furniture(e: &mut Engine, p: u32, actor: u32, reference: u32) {
    let player: u32 = e.global(PLAYER);
    let index = u32::from(e.mem.u8(p + 0x144));
    e.call(REFERENCE_SET_MARKER_USED, &args![reference, index, 1u32]);
    if actor == player {
        for _ in 0..2 {
            e.vcall(p, 0x4d8, &args![player]);
            let flag = e.call(PLAYER_LIGHTING_TEST, &args![player]).u8();
            e.call(PLAYER_SET_FIRST_PERSON, &args![player, u32::from(flag)]);
        }
    } else {
        e.vcall(p, 0x4d8, &args![actor]);
    }
}

// Translated from 00925700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::FinishInitLoadGame` (Xbox PDB): `(process, a, b,
/// object)`. After `BaseProcess::FinishInitLoadGame` (`008d0e80`): the
/// object's slot +0x1B8 answers an animation object whose slot +0xD8
/// gets (1, 0) when the signed byte at +0x13C is 1 to 4 and (0, 0) when it
/// is any other non-zero value; the process's slot +0x1CC is run for the
/// actor (twice for the player, with the first person and the third
/// person animation and biped) with the byte at +0x135. Then, when the
/// actor is in the same cell as the saved furniture reference (+0x140) and
/// is not running a package that stops it (slot +0x22C), the furniture
/// state (+0x13D: 1 to 4 become 1, 6 to 9 become 6) is applied through
/// slot +0x4D8, the marker is taken, and for the states 4 and 9 the actor
/// is moved to the marker (heading, rotated target offset, character
/// controller position); otherwise the furniture reference and state are
/// cleared (and the player's furniture state for the player). A state
/// without a reference is cleared. The 3D object (see `PLAYER_GET_3D`) gets its motion set
/// for the states 1 and 2, the active effects are finished
/// (`00806130`), `+0x180` is set, slot +0x580 is called with (1, 0, 0),
/// the process is queued for deferred deletion when +0x18B is set
/// without an actor (+0x158) and slot +0x7A8 is called with the actor.
pub fn middle_high_process_finish_init_load_game(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    flags: u32,
    flags_2: u32,
    object: u32,
) {
    let p = this.addr();
    let player: u32 = e.global(PLAYER);
    e.call(
        BASE_FINISH_INIT_LOAD_GAME,
        &args![p, flags, flags_2, object],
    );
    let actor = dynamic_cast(e, object, TYPE_MOBILE_OBJECT, TYPE_ACTOR);
    let animation = e.vcall(object, 0x1b8, &args![0u32]).u32();
    let state = e.mem.u8(p + 0x13c) as i8;
    if animation != 0 && state != 0 {
        if state > 0 && state <= 4 {
            e.vcall(animation, 0xd8, &args![1u32, 0u32]);
        } else {
            e.vcall(animation, 0xd8, &args![0u32, 0u32]);
        }
    }
    if actor != 0 {
        let first = u32::from(e.mem.u8(p + 0x135));
        if actor == player {
            for view in [1u32, 0] {
                let animation = e.call(PLAYER_GET_ANIMATION, &args![player, view]).u32();
                let biped = e.call(PLAYER_GET_BIPED, &args![player, view]).u32();
                e.vcall(p, 0x1cc, &args![first, biped, animation, player]);
            }
        } else {
            let value = e.vcall(actor, 0x1e8, &[]).u32();
            if e.vcall(p, 0x190, &args![value]).u32() != 0 {
                let second = e.vcall(actor, 0x1e4, &[]).u32();
                let third = e.vcall(actor, 0x1e8, &[]).u32();
                e.vcall(p, 0x1cc, &args![first, third, second, actor]);
            }
        }
    }
    let object_3d = if object == player {
        e.call(PLAYER_GET_3D, &args![player, 0u32]).u32()
    } else {
        e.call(REFERENCE_3D, &args![object]).u32()
    };
    if actor != 0
        && e.call(REFERENCE_3D, &args![actor]).u32() != 0
        && e.mem.u32(p + 0x140) != 0
        && !e.vcall(actor, 0x22c, &args![0u32]).bool()
    {
        let mut keep = true;
        let reference = e.mem.u32(p + 0x140);
        let actor_cell = e.call(REFERENCE_CELL, &args![actor]).u32();
        let reference_cell = e.call(REFERENCE_CELL, &args![reference]).u32();
        if actor_cell == reference_cell {
            let base = e.call(REFERENCE_BASE, &args![reference]).u32();
            let furniture = dynamic_cast(e, base, TYPE_TES_BOUND_OBJECT, TYPE_TES_FURNITURE);
            let mut markers = 0;
            if furniture != 0 {
                let model = e.vcall(reference, 0x1d0, &[]).u32();
                markers = e.call(FURNITURE_MARKER_FIND, &args![model]).u32();
                let index = u32::from(e.mem.u8(p + 0x144));
                if markers == 0 || index >= e.call(MARKER_COUNT, &args![markers]).u32() {
                    keep = false;
                }
            }
            if keep {
                let saved = e.mem.u8(p + 0x13d);
                match saved {
                    1..=4 => e.mem.set_u8(p + 0x13d, 1),
                    6..=9 => e.mem.set_u8(p + 0x13d, 6),
                    _ => {}
                }
                if e.mem.u8(p + 0x13d) != 0 {
                    finish_load_enter_furniture(e, p, actor, reference);
                }
                e.mem.set_u8(p + 0x13d, saved);
                let state = e.mem.u8(p + 0x13d);
                if (state == 4 || state == 9) && furniture != 0 {
                    finish_load_place_on_marker(e, p, actor, reference, furniture, markers, true);
                }
            }
        } else {
            keep = false;
        }
        if !keep {
            e.mem.set_u32(p + 0x140, 0);
            e.mem.set_u8(p + 0x13d, 0);
            if actor == player {
                e.call(PLAYER_CLEAR_FURNITURE, &args![player]);
            }
        } else {
            let state = e.mem.u8(p + 0x13d);
            if state == 9 || state == 4 {
                e.call(
                    WORLD_ADJUST_FORCE_ADD_COUNT,
                    &args![object_3d, 1u32, 1u32, 0u32],
                );
            }
        }
    }
    if actor != 0 && e.mem.u8(p + 0x13d) != 0 && e.mem.u32(p + 0x140) == 0 {
        e.mem.set_u8(p + 0x13d, 0);
        if actor == player {
            e.call(PLAYER_CLEAR_FURNITURE, &args![player]);
        }
    }
    let state = e.mem.u8(p + 0x13c) as i8;
    if (object_3d != 0 && state == 2) || state == 1 {
        e.call(WORLD_SET_MOTION, &args![object_3d, 1u32, 1u32, 0u32, 1u32]);
        e.with_stack(12, |e, value| {
            e.call(MOTION_VALUE_CONSTRUCT, &args![value, 0u32, 0u32, 0u32]);
            e.call(WORLD_APPLY_MOTION_VALUE, &args![object_3d, value]);
        });
    }
    let effects = e.mem.u32(p + 0x1b8);
    e.call(ACTIVE_EFFECTS_FINISH_LOAD, &args![effects, actor]);
    e.mem.set_u8(p + 0x180, 1);
    if actor != 0 {
        e.vcall(p, 0x580, &args![1u32, 0u32, 0u32]);
    }
    if e.mem.u8(p + 0x18b) != 0 && e.mem.u32(p + 0x158) == 0 {
        let save = save_load(e);
        e.call(ADD_FORM_TO_DEFERRED_DELETIONS, &args![save, object]);
    }
    if actor != 0 {
        e.vcall(p, 0x7a8, &args![actor]);
    }
}

/// The part of `FinishInitLoadGame` and `FinishLoadGame` for the furniture
/// states 4 and 9: the
/// marker's entry (when the saved index is inside the markers, which
/// `FinishLoadGame` does not check: `check_index`) gives the
/// mark its heading and offset, the mark's rotated target offset moves the
/// actor (slot +0x2A8), and the actor is turned and put in the character
/// controller.
fn finish_load_place_on_marker(
    e: &mut Engine,
    p: u32,
    actor: u32,
    reference: u32,
    furniture: u32,
    markers: u32,
    check_index: bool,
) {
    let player: u32 = e.global(PLAYER);
    let mark = p + 0x148;
    e.with_stack(0xc0, |e, locals| {
        let entry_copy = locals.addr();
        let base_point = entry_copy + 0x10;
        let out_point = entry_copy + 0x1c;
        let orientation = entry_copy + 0x28;
        let offset = entry_copy + 0x50;
        let matrix = entry_copy + 0x5c;
        let scratch_a = entry_copy + 0x80;
        let scratch_b = entry_copy + 0x8c;
        let index = u32::from(e.mem.u8(p + 0x144));
        if markers != 0 && (!check_index || index < e.call(MARKER_COUNT, &args![markers]).u32()) {
            let entry = e.call(MARKER_ENTRY, &args![markers, index]).u32();
            for word_index in 0..4u32 {
                let value = e.mem.u32(entry + word_index * 4);
                e.mem.set_u32(entry_copy + word_index * 4, value);
            }
            let source = e.call(IDENTITY, &args![entry_copy]).u32();
            for word_index in 0..3u32 {
                let value = e.mem.u32(source + word_index * 4);
                e.mem.set_u32(base_point + word_index * 4, value);
            }
            e.call(IDENTITY, &args![out_point]);
            let reference_data = e.vcall(reference, 0x1f4, &[]).u32();
            let reference_orientation = e
                .call(REFERENCE_GET_ORIENTATION, &args![reference, orientation])
                .u32();
            e.call(
                TRANSFORM_VERTICES,
                &args![
                    reference_orientation,
                    reference_data,
                    1u32,
                    base_point,
                    out_point
                ],
            );
            let marker_byte = e.call(MARK_BYTE_E, &args![entry_copy]).u32();
            e.call(SET_BYTE_E, &args![mark, marker_byte]);
            e.call(COPY_POINT, &args![mark, out_point]);
            let entry_heading = e.call(MARK_HEADING, &args![entry_copy]).f64();
            let extra = e.call(REFERENCE_PLUS_24, &args![reference]).u32();
            // `FLD` of the stored `float`, added to the heading in double
            // precision and stored back as a `float`.
            let loaded = x87_quiet(e.mem.f32(extra + 8));
            let heading = (f64::from(loaded) + entry_heading) as f32;
            e.call(
                FURNITURE_MARK_SET_HEADING,
                &args![mark, x87_quiet_bits(heading.to_bits())],
            );
        }
        let scale = e.call(REFERENCE_GET_SCALE, &args![actor]).f64() as f32;
        let marker_byte = e.call(MARK_BYTE_E, &args![mark]).u32();
        e.call(
            FURNITURE_GET_MARKER_TARGET_OFFSET,
            &args![furniture, offset, marker_byte, scale.to_bits()],
        );
        e.call(IDENTITY, &args![matrix]);
        let heading = e.call(MARK_HEADING, &args![mark]).f64() as f32;
        e.call(ROTATION_FROM_HEADING, &args![matrix, heading.to_bits()]);
        let rotated = e
            .call(ROTATE_POINT, &args![matrix, scratch_a, offset])
            .u32();
        for word_index in 0..3u32 {
            let value = e.mem.u32(rotated + word_index * 4);
            e.mem.set_u32(offset + word_index * 4, value);
        }
        let point = e.call(IDENTITY, &args![mark]).u32();
        let sum = e.call(POINT_ADD, &args![point, scratch_b, offset]).u32();
        e.vcall(actor, 0x2a8, &args![sum]);
        let state = e.mem.u8(p + 0x13d);
        e.mem.set_u8(p + 0x13d, 0);
        if actor != player {
            let heading = e.call(MARK_HEADING, &args![mark]).f64() as f32;
            e.vcall(actor, 0x2c4, &args![heading.to_bits()]);
            let marker_byte = e.call(MARK_BYTE_E, &args![mark]).u32();
            let angle = e
                .call(FURNITURE_MARKER_ANGLE, &args![furniture, marker_byte])
                .f64() as f32;
            e.call(MOBILE_TURN_BY, &args![actor, angle.to_bits()]);
        }
        e.mem.set_u8(p + 0x13d, state);
    });
    if e.call(GET_CHAR_CONTROLLER, &args![actor]).u32() != 0 {
        let point = e.call(IDENTITY, &args![mark]).u32();
        let controller = e.call(GET_CHAR_CONTROLLER, &args![actor]).u32();
        e.call(CONTROLLER_SET_POSITION, &args![controller, point]);
    }
}

/// `MiddleLowProcess::Revert` (Xbox PDB), `(process, flags, object)`.
const MIDDLE_LOW_REVERT: u32 = 0x0092_e9f0;
/// `(save/load game)`: 1 when the word at +0x48 of the save/load object is
/// 0 (the check that guards the field reset in `Revert`).
const SAVE_LOAD_GAME_FLAG: u32 = 0x0055_f5b0;
/// animation.cpp, `(animation, object)`: the animation's revert.
const ANIMATION_REVERT: u32 = 0x0049_a920;
/// activeeffect.cpp, cdecl `(effect list, actor)`.
const ACTIVE_EFFECTS_REVERT: u32 = 0x0080_61b0;

// Translated from 00926000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::Revert` (Xbox PDB): `(process, flags, object)`.
/// After `MiddleLowProcess::Revert` (`0092e9f0`): with flag 0x20, reloads
/// the actor's targets when +0x114 is set, calls the process's slots
/// +0x160 (0, 0, 0), +0x168, +0x16C and +0x164 with 0 and clears +0x16C.
/// When `0055f5b0` answers true on the save/load object, resets the
/// process: the animation (`0049a920`, for flag 0x10000000 with +0x1C0
/// set), the active effects (`008061b0`), slot +0x294 with the actor, and
/// every saved field to its initial value (the lists at +0xC8, +0xD0 and
/// +0x1AC emptied, the last one deleted). The package revert guarded by
/// `flags & 0` is compiled out and left out here. Always clears +0x248,
/// +0x24C and +0x250.
pub fn middle_high_process_revert(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    flags: u32,
    object: u32,
) {
    let p = this.addr();
    let actor = dynamic_cast(e, object, TYPE_MOBILE_OBJECT, TYPE_ACTOR);
    e.call(MIDDLE_LOW_REVERT, &args![p, flags, object]);
    if flags & 0x20 != 0 {
        if e.mem.u32(p + 0x114) != 0 {
            e.call(ACTOR_RELOAD_TARGETS, &args![actor, 0u32]);
        }
        e.vcall(p, 0x160, &args![0u32, 0u32, 0u32]);
        e.vcall(p, 0x168, &args![0u32]);
        e.vcall(p, 0x16c, &args![0u32]);
        e.vcall(p, 0x164, &args![0u32]);
        e.mem.set_u32(p + 0x16c, 0);
    }
    let save = save_load(e);
    if e.call(SAVE_LOAD_GAME_FLAG, &args![save]).bool() {
        if flags & 0x1000_0000 != 0 && e.mem.u32(p + 0x1c0) != 0 {
            let animation = e.mem.u32(p + 0x1c0);
            e.call(ANIMATION_REVERT, &args![animation, object]);
        }
        let effects = e.mem.u32(p + 0x1b8);
        e.call(ACTIVE_EFFECTS_REVERT, &args![effects, actor]);
        if actor != 0 {
            e.vcall(p, 0x294, &args![actor]);
        }
        e.mem.set_u8(p + 0x134, 0);
        e.vcall(p, 0x4c0, &args![actor, 0u32, 0u32, 0x7fu32]);
        e.mem.set_u32(p + 0x140, 0);
        e.mem.set_u8(p + 0x144, 0x7f);
        e.call(LIST_CLEAR, &args![p + 0xc8]);
        e.mem.set_u8(p + 0x124, 0);
        e.mem.set_u8(p + 0x125, 0);
        e.mem.set_u8(p + 0x126, 0);
        e.mem.set_u32(p + 0x158, 0);
        e.mem.set_u32(p + 0x15c, 0);
        e.vcall(p, 0x218, &args![0u32]);
        e.mem.set_u8(p + 0x135, 0);
        e.mem.set_u32(p + 0x58, 0);
        e.mem.set_f32(p + 0xd8, 0.0);
        e.mem.set_u8(p + 0x1da, 0);
        e.mem.set_u32(p + 0x108, 0);
        e.mem.set_f32(p + 0xdc, 0.0);
        e.mem.set_u8(p + 0x168, 0);
        e.mem.set_u32(p + 0x16c, 0);
        e.mem.set_f32(p + 0x170, 1.0);
        e.mem.set_f32(p + 0x174, 0.0);
        e.mem.set_u32(p + 0x184, 0);
        e.mem.set_u8(p + 0x180, 0);
        e.mem.set_u8(p + 0xe0, 0);
        e.mem.set_f32(p + 0xa4, 0.0);
        e.mem.set_u8(p + 0x188, 0);
        e.mem.set_u8(p + 0x189, 0);
        e.mem.set_u8(p + 0x17c, 0);
        e.mem.set_u8(p + 0x17d, 0);
        e.mem.set_u8(p + 0x17e, 0);
        e.mem.set_u8(p + 0x17f, 0);
        e.mem.set_u32(p + 0x68, 0);
        e.mem.set_u8(p + 0x18a, 0);
        e.call(LIST_CLEAR, &args![p + 0xd0]);
        let list = e.mem.u32(p + 0x1ac);
        if list != 0 {
            e.call(LIST_CLEAR, &args![list]);
            let list = e.mem.u32(p + 0x1ac);
            if list != 0 {
                e.call(LIST_DELETE, &args![list, 1u32]);
            }
            e.mem.set_u32(p + 0x1ac, 0);
        }
    }
    e.mem.set_u32(p + 0x248, 0);
    e.mem.set_u32(p + 0x24c, 0);
    e.mem.set_u32(p + 0x250, 0);
}

/// Writes `size` bytes of the save `(save/load game, data, size)` (the
/// save/load object's writer).
const WRITE_BYTES: u32 = 0x0085_79b0;
/// `(controller area, mask)`: the flag bits of the character controller's
/// state at +0x410 that are set in `mask` (all of them for -1).
const CONTROLLER_STATE_FLAGS: u32 = 0x0062_1270;
/// `(controller area)`: the word the character controller saves first,
/// answered by `00ca9600` (at +0x3E0 of the controller).
const CONTROLLER_SAVED_WORD: u32 = 0x00ca_9600;
/// `bhkCharacterController::GetPosition` (Xbox PDB), `(controller, out)`.
const CONTROLLER_GET_POSITION: u32 = 0x0081_2b00;
/// `bhkCharacterProxy::GetLinearVelocity` (Xbox PDB), `(controller, out)`.
const CONTROLLER_GET_LINEAR_VELOCITY: u32 = 0x0066_ca00;
/// `Actor::DoesFly` (Xbox PDB) as the exe has it on a controller:
/// `(controller, value)` stores the loaded movement mode.
const CONTROLLER_SET_MOVE_MODE: u32 = 0x008e_2680;
/// `(controller area, mask)`: sets the controller state flags in the
/// area at +0x410 to the value (`006296b0` clears with -1,
/// `00629670` sets `(value, 1)`).
const CONTROLLER_CLEAR_STATE_FLAGS: u32 = 0x0062_96b0;
const CONTROLLER_SET_STATE_FLAGS: u32 = 0x0062_9670;
/// `MobileObject::SetChaseBip` (Xbox PDB), `(actor, 0)`.
const SET_CHASE_BIP: u32 = 0x0093_1fb0;
/// `TES::GetWaterHeight` (Xbox PDB) as the exe has it on a controller:
/// `(controller)`.
const CONTROLLER_IS_IN_WATER: u32 = 0x0088_b0c0;
/// `bhkCharacterProxy::operatorP` (Xbox PDB), `(controller)`: the Havok
/// proxy (0 without one).
const CONTROLLER_GET_PROXY: u32 = 0x004a_e750;
/// `(out, vector)`, cdecl: builds the Havok vector the proxy takes.
const MAKE_HAVOK_VECTOR: u32 = 0x004a_3e00;
/// `(proxy, havok vector)`: sets the proxy's linear velocity.
const PROXY_SET_LINEAR_VELOCITY: u32 = 0x00ca_c960;

/// The state flag test of the controller state at `controller + 0x410`.
fn controller_state_flags(e: &mut Engine, controller: u32, mask: u32) -> u32 {
    e.call(CONTROLLER_STATE_FLAGS, &args![controller + 0x410, mask])
        .u32()
}

// Translated from 00926350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetCharControllerSaveSize` (Xbox PDB): `(process,
/// actor)`. 0 without a character controller (the process's slot +0x28C)
/// or for an actor whose slot +0x22C answers true; otherwise the bytes
/// `SaveCharController` writes: 12 (+12 when the controller's state flag
/// 0x800 is set) + 44, then +12 when the actor's slot +0x100 is true else
/// +8, and +8 from save version 0x77 (a 16-bit sum).
pub fn middle_high_process_get_char_controller_save_size(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) -> u16 {
    let mut size: u16 = 0;
    let controller = e.vcall(this.addr(), 0x28c, &[]).u32();
    if controller != 0 && !e.vcall(actor, 0x22c, &args![0u32]).bool() {
        size = size.wrapping_add(4).wrapping_add(4);
        let flags = controller_state_flags(e, controller, u32::MAX);
        size = size.wrapping_add(4);
        if flags & 0x800 != 0 {
            size = size.wrapping_add(0xc);
        }
        size = size.wrapping_add(0x10).wrapping_add(0x10).wrapping_add(0xc);
        if e.vcall(actor, 0x100, &[]).bool() {
            size = size.wrapping_add(4).wrapping_add(4).wrapping_add(4);
        } else {
            size = size.wrapping_add(4).wrapping_add(4);
        }
        if current_version(e) >= 0x77 {
            size = size.wrapping_add(4).wrapping_add(4);
        }
    }
    size
}

// Translated from 00926490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SaveCharController` (Xbox PDB): `(process, actor)`.
/// With a character controller (slot +0x28C) and an actor whose slot
/// +0x22C is false, writes the controller's saved word (`00ca9600`), its
/// word at +0x520, its state flags, the position (when flag 0x800 is set),
/// the 16 byte values at +0x4F0 and +0x500, the linear velocity (12
/// bytes), then +0x540/+0x544/+0x548 (actor slot +0x100 true) or
/// +0x54C/+0x548, and from save version 0x77 +0x550 and +0x558.
pub fn middle_high_process_save_char_controller(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    let controller = e.vcall(this.addr(), 0x28c, &[]).u32();
    if controller == 0 || e.vcall(actor, 0x22c, &args![0u32]).bool() {
        return;
    }
    let save = save_load(e);
    e.with_stack(0x24, |e, locals| {
        let word = locals.addr();
        let flags_cell = word + 4;
        let position = word + 8;
        let velocity = word + 0x14;
        let value = e
            .call(CONTROLLER_SAVED_WORD, &args![controller + 0x3e0])
            .u32();
        e.mem.set_u32(word, value);
        e.call(WRITE_BYTES, &args![save, word, 4u32]);
        e.call(WRITE_BYTES, &args![save, controller + 0x520, 4u32]);
        let flags = controller_state_flags(e, controller, u32::MAX);
        e.mem.set_u32(flags_cell, flags);
        e.call(WRITE_BYTES, &args![save, flags_cell, 4u32]);
        if flags & 0x800 != 0 {
            e.call(IDENTITY, &args![position]);
            e.call(CONTROLLER_GET_POSITION, &args![controller, position]);
            e.call(WRITE_BYTES, &args![save, position, 0xcu32]);
        }
        e.call(WRITE_BYTES, &args![save, controller + 0x4f0, 0x10u32]);
        e.call(WRITE_BYTES, &args![save, controller + 0x500, 0x10u32]);
        e.call(IDENTITY, &args![velocity]);
        e.call(CONTROLLER_GET_LINEAR_VELOCITY, &args![controller, velocity]);
        e.call(WRITE_BYTES, &args![save, velocity, 0xcu32]);
        if e.vcall(actor, 0x100, &[]).bool() {
            for offset in [0x540u32, 0x544, 0x548] {
                e.call(WRITE_BYTES, &args![save, controller + offset, 4u32]);
            }
        } else {
            for offset in [0x54cu32, 0x548] {
                e.call(WRITE_BYTES, &args![save, controller + offset, 4u32]);
            }
        }
        if current_version(e) >= 0x77 {
            e.call(WRITE_BYTES, &args![save, controller + 0x550, 4u32]);
            e.call(WRITE_BYTES, &args![save, controller + 0x558, 4u32]);
        }
    });
}

// Translated from 00926690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads what `SaveCharController` wrote back into the character
/// controller (the engine map has no name; the counterpart of
/// [`middle_high_process_save_char_controller`]): `(process, actor)`. With a
/// controller (slot +0x28C) and an actor whose slot +0x22C is false: reads
/// the movement mode (`008e2680`), +0x520, the state flags (cleared with
/// -1, set to the read value, `006296b0`/`00629670`), from version 0x23 the
/// position (kept as read, or replaced by the slot +0x4D4 point when slots
/// +0x4BC and +0x4D4 answer non-zero and the actor's slot +0x100 is false)
/// which is set on the controller, `SetChaseBip(0)` when slot +0x4BC
/// answers 0, the 16 byte values at +0x4F0 and +0x500, the linear velocity
/// (`00926980`), +0x540/+0x544/+0x548 (actor slot +0x100) or, from version
/// 0x20, +0x54C/+0x548; with movement mode 2 the float at +0x544 is cleared
/// when the controller is in water (`0088b0c0`) or has state flag 0x200
/// (`009269f0`); from version 0x77 +0x550 and +0x558.
pub fn fn_00926690(e: &mut Engine, this: Ptr<MiddleHighProcess>, actor: u32) {
    let p = this.addr();
    let controller = e.vcall(p, 0x28c, &[]).u32();
    if controller == 0 || e.vcall(actor, 0x22c, &args![0u32]).bool() {
        return;
    }
    let mode = read_value(e, 4);
    e.call(CONTROLLER_SET_MOVE_MODE, &args![controller, mode]);
    read_bytes(e, controller + 0x520, 4);
    let flags = read_value(e, 4);
    e.call(
        CONTROLLER_CLEAR_STATE_FLAGS,
        &args![controller + 0x410, u32::MAX],
    );
    e.call(
        CONTROLLER_SET_STATE_FLAGS,
        &args![controller + 0x410, flags, 1u32],
    );
    if flags & 0x800 != 0 && current_version(e) >= 0x23 {
        e.with_stack(12, |e, position| {
            e.call(IDENTITY, &args![position]);
            read_bytes(e, position.addr(), 0xc);
            if e.vcall(p, 0x4bc, &[]).u32() != 0 && e.vcall(p, 0x4d4, &[]).u32() != 0 {
                let mut chosen = 0;
                if e.vcall(actor, 0x100, &[]).bool() {
                    chosen = actor;
                }
                if chosen == 0 {
                    let point = e.vcall(p, 0x4d4, &[]).u32();
                    for offset in [0u32, 4, 8] {
                        let value = e.mem.u32(point + offset);
                        e.mem.set_u32(position.addr() + offset, value);
                    }
                }
            }
            e.call(CONTROLLER_SET_POSITION, &args![controller, position]);
        });
    }
    if e.vcall(p, 0x4bc, &[]).u32() == 0 {
        e.call(SET_CHASE_BIP, &args![actor, 0u32]);
    }
    read_bytes(e, controller + 0x4f0, 0x10);
    read_bytes(e, controller + 0x500, 0x10);
    e.with_stack(12, |e, velocity| {
        e.call(IDENTITY, &args![velocity]);
        read_bytes(e, velocity.addr(), 0xc);
        fn_00926980(e, controller, velocity.addr());
    });
    if e.vcall(actor, 0x100, &[]).bool() {
        read_bytes(e, controller + 0x540, 4);
        read_bytes(e, controller + 0x544, 4);
        read_bytes(e, controller + 0x548, 4);
    } else if current_version(e) >= 0x20 {
        read_bytes(e, controller + 0x54c, 4);
        read_bytes(e, controller + 0x548, 4);
    }
    if mode == 2
        && (e.call(CONTROLLER_IS_IN_WATER, &args![controller]).bool() || fn_009269f0(e, controller))
    {
        e.mem.set_f32(controller + 0x544, 0.0);
    }
    if current_version(e) >= 0x77 {
        read_bytes(e, controller + 0x550, 4);
        read_bytes(e, controller + 0x558, 4);
    }
}

// Translated from 00926980 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00926980 - the /GS stack cookie check (00ec408c) is not translated
/// `bhkCharacterProxy::SetLinearVelocity` (Xbox PDB): `(controller,
/// velocity)`. Builds a Havok vector from the 12 byte `velocity`
/// (`004a3e00`) and gives it to the controller's proxy (`004ae750`,
/// `00cac960`) when it has one.
pub fn fn_00926980(e: &mut Engine, this: u32, velocity: u32) {
    e.with_stack(0x1c, |e, vector| {
        e.call(IDENTITY, &args![vector]);
        let proxy = e.call(CONTROLLER_GET_PROXY, &args![this]).u32();
        if proxy != 0 {
            let made = e.call(MAKE_HAVOK_VECTOR, &args![vector, velocity]).u32();
            e.call(PROXY_SET_LINEAR_VELOCITY, &args![proxy, made]);
        }
    });
}

// Translated from 009269f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the character controller has any of the state flag bits 0x200
/// (`00621270` on the state at +0x410); the engine map has no name.
pub fn fn_009269f0(e: &mut Engine, this: u32) -> bool {
    controller_state_flags(e, this, 0x200) != 0
}

/// `MiddleLowProcess::SaveGame_ov2` (Xbox PDB), `(process, buffer)`.
const MIDDLE_LOW_SAVE_GAME_OV2: u32 = 0x0092_ea30;
/// `BGSSaveGameBuffer::SaveFormID` (Xbox PDB), `(buffer, form id, 0)`.
const SAVE_FORM_ID_VALUE: u32 = 0x0086_5db0;
/// `BGSSaveGameBuffer::StartVariableSizedValue` (Xbox PDB), `(buffer)`:
/// the position to patch the size at.
const START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
/// `BGSSaveGameBuffer::SaveVariableSizedValue_ov2` (Xbox PDB), `(buffer,
/// size, position)`.
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5ff0;
/// `ActorPackage::SaveGame` (Xbox PDB), `(package, buffer)`.
const ACTOR_PACKAGE_SAVE_GAME: u32 = 0x008c_65d0;
/// extradatalist.cpp, `(buffer, out)`: the save kind word at +0x17 of the
/// buffer, stored in `out`, which is answered.
const BUFFER_SAVE_KIND: u32 = 0x0042_8110;
/// extradatalist.cpp, `(kinds, mask)`: whether the word has a bit of
/// `mask`.
const KIND_HAS_BITS: u32 = 0x0042_80f0;
/// animation.cpp, `(animation, buffer)`: saves the animation block.
const ANIMATION_SAVE_GAME: u32 = 0x0049_ab40;
/// `ActiveEffect::SaveActiveEffectList` (Xbox PDB), cdecl `(buffer, list)`.
const ACTIVE_EFFECTS_SAVE: u32 = 0x0080_6a10;
/// `MagicItem::GetMagicItemFormID` (Xbox PDB), `(item)`.
const MAGIC_ITEM_FORM_ID: u32 = 0x0040_a1e0;
/// `MagicTarget::GetMagicTargetFormID` (Xbox PDB), `(target)`.
const MAGIC_TARGET_FORM_ID: u32 = 0x0082_54c0;
/// `QueuedItem::SaveGame` (Xbox PDB), `(item, buffer)`.
const QUEUED_ITEM_SAVE_GAME: u32 = 0x0091_3d60;

/// Saves `size` bytes of the process at `offset`.
fn save_field(e: &mut Engine, buffer: u32, address: u32, size: u32) {
    e.call(SAVE_DATA, &args![buffer, address, size, 0u32]);
}

// Translated from 00926a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SaveGame_ov2` (Xbox PDB): `(process, buffer)`.
/// After `MiddleLowProcess::SaveGame_ov2` (`0092ea30`) writes the
/// process's fields in save order (+0x134, +0x135, +0x168, +0x170,
/// +0x174, +0x108, +0x1DA, +0xFC (12 bytes), +0xDC, +0x13D, +0x144,
/// +0x156, +0x154 (2 bytes), +0x148 (12 bytes), +0x13C, +0xE0, +0x188,
/// +0x189, +0xD8, +0x18B, +0x1D0, +0x1D4, +0x1D8, +0x1D9, +0x228, +0x22A
/// (2 bytes), +0x1A8, +0xE1, +0x190, +0x198, +0x19C, +0x19D, +0x234,
/// +0x238, +0x23C, +0x244, +0x110), the form ids at +0x10C, +0x194,
/// +0x158 and +0x140, the word after the head node of +0x118 and the
/// counted id list at +0xC8 (the count patched in afterwards). Then the
/// package (`008c65d0`), the animation block as a counted value when the
/// buffer's reference (slot +0x08) exists and the buffer's save kind has
/// bit 0x10000000, the active effects (`00806a10` with +0x1B8), the form
/// ids of the magic items at +0x164 and +0x160 and the magic target at
/// +0x1BC (0 for none), and the queued items of the list at +0x230
/// (`QueuedItem::SaveGame`) as a counted value.
pub fn middle_high_process_save_game_ov2(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    buffer: u32,
) {
    let p = this.addr();
    let reference = e.vcall(buffer, 0x8, &[]).u32();
    e.call(MIDDLE_LOW_SAVE_GAME_OV2, &args![p, buffer]);
    for (offset, size) in [
        (0x134u32, 1u32),
        (0x135, 1),
        (0x168, 1),
        (0x170, 4),
        (0x174, 4),
        (0x108, 4),
        (0x1da, 1),
        (0xfc, 0xc),
        (0xdc, 4),
        (0x13d, 1),
        (0x144, 1),
        (0x156, 1),
        (0x154, 2),
        (0x148, 0xc),
        (0x13c, 1),
        (0xe0, 1),
        (0x188, 1),
        (0x189, 1),
        (0xd8, 4),
        (0x18b, 1),
        (0x1d0, 4),
        (0x1d4, 4),
        (0x1d8, 1),
        (0x1d9, 1),
        (0x228, 1),
        (0x22a, 2),
        (0x1a8, 4),
        (0xe1, 1),
        (0x190, 4),
        (0x198, 4),
        (0x19c, 1),
        (0x19d, 1),
        (0x234, 4),
        (0x238, 4),
        (0x23c, 4),
        (0x244, 4),
        (0x110, 1),
    ] {
        save_field(e, buffer, p + offset, size);
    }
    for offset in [0x10cu32, 0x194, 0x158, 0x140] {
        let form = e.mem.u32(p + offset);
        e.call(SAVE_FORM_ID, &args![buffer, form, 0u32]);
    }
    e.with_stack(4, |e, head_next| {
        e.mem.set_u32(head_next.addr(), 0);
        if e.mem.u32(p + 0x118) != 0 {
            let head = e.mem.u32(p + 0x118);
            let next = e.call(WORD_4, &args![head]).u32();
            e.mem.set_u32(head_next.addr(), next);
        }
        save_field(e, buffer, head_next.addr(), 4);
    });
    let mut count = 0u32;
    let position = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut node = p + 0xc8;
    while node != 0 {
        let slot = e.call(IDENTITY, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item != 0 {
            e.call(SAVE_FORM_ID, &args![buffer, item, 0u32]);
            count = count.wrapping_add(1);
        }
        node = e.call(WORD_4, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count, position]);
    e.call(ACTOR_PACKAGE_SAVE_GAME, &args![p + 0xe4, buffer]);
    if reference != 0 {
        let kind = e.with_stack(4, |e, out| {
            let out = e.call(BUFFER_SAVE_KIND, &args![buffer, out]).u32();
            e.call(KIND_HAS_BITS, &args![out, 0x1000_0000u32]).bool()
        });
        if kind {
            let position = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
            let before = e.call(WORD_C, &args![buffer]).u32();
            let animation = e.mem.u32(p + 0x1c0);
            e.call(ANIMATION_SAVE_GAME, &args![animation, buffer]);
            let after = e.call(WORD_C, &args![buffer]).u32();
            e.call(
                SAVE_VARIABLE_SIZED_VALUE,
                &args![buffer, after.wrapping_sub(before), position],
            );
        }
    }
    let effects = e.mem.u32(p + 0x1b8);
    e.call(ACTIVE_EFFECTS_SAVE, &args![buffer, effects]);
    for (offset, id_function) in [
        (0x164u32, MAGIC_ITEM_FORM_ID),
        (0x160, MAGIC_ITEM_FORM_ID),
        (0x1bc, MAGIC_TARGET_FORM_ID),
    ] {
        let object = e.mem.u32(p + offset);
        let id = if object != 0 {
            e.call(id_function, &args![object]).u32()
        } else {
            0
        };
        e.call(SAVE_FORM_ID_VALUE, &args![buffer, id, 0u32]);
    }
    let mut count = 0u32;
    let position = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut node = e.mem.u32(p + 0x230);
    while node != 0 {
        let slot = e.call(IDENTITY, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item != 0 {
            e.call(QUEUED_ITEM_SAVE_GAME, &args![item, buffer]);
            count = count.wrapping_add(1);
        }
        node = e.call(WORD_4, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count, position]);
}

/// `MiddleLowProcess::LoadGame_ov2` (Xbox PDB), `(process, buffer)`.
const MIDDLE_LOW_LOAD_GAME_OV2: u32 = 0x0092_eaa0;
/// `BGSLoadGameBuffer::LoadFormID` (Xbox PDB), `(buffer)`: the next form id.
const LOAD_FORM_ID_VALUE: u32 = LOAD_FORM_ID;
/// `BGSLoadGameBuffer::LoadFormID_ov2` (Xbox PDB), `(buffer, destination)`.
const LOAD_FORM_ID_INTO: u32 = 0x0086_48e0;
/// `BGSLoadGameBuffer::LoadVariableSizedValue` (Xbox PDB), `(buffer)`.
const LOAD_VARIABLE_SIZED_VALUE: u32 = 0x0086_4a60;
/// `ActorPackage::LoadGame_ov2` (Xbox PDB), `(package, buffer)`.
const ACTOR_PACKAGE_LOAD_GAME_OV2: u32 = 0x008c_6730;
/// middlehighprocess.cpp, `(actor, state)`: called with the saved state
/// byte +0x13D.
const ACTOR_APPLY_FURNITURE_STATE: u32 = 0x0092_0fb0;
/// `BGSSaveLoadGame::QueueSubBuffer` (Xbox PDB), `(save/load game, 0,
/// buffer, 0)`.
const QUEUE_SUB_BUFFER: u32 = 0x0084_a810;
/// `InventoryChanges::GetWornItem` (Xbox PDB), `(changes, 5, 0)`.
const GET_WORN_ITEM: u32 = 0x004c_8c10;
/// `TESObjectREFR` method (tesobjectrefr.cpp), `(actor, item form)`.
const ACTOR_EQUIP_WORN: u32 = 0x0057_1760;
/// `InventoryChanges::GetInventoryChanges` (Xbox PDB), cdecl `(reference)`.
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `InventoryChanges::GetObjectCount` (Xbox PDB), `(changes, form)`.
const INVENTORY_GET_OBJECT_COUNT: u32 = 0x004c_8f30;
/// `CombatProcedureAttackMelee::Initialize` (Xbox PDB) as the exe has it
/// on the item change: `(item change, count)`.
const ITEM_CHANGE_SET_COUNT: u32 = 0x006e_cd40;
/// The reference's name text `(reference)` (`0055d520`), and `strstr`
/// (CRT, cdecl `(text, pattern)`) with the text `"Lily"` (`0102fdc8`).
const REFERENCE_NAME: u32 = 0x0055_d520;
const STRSTR: u32 = 0x00ec_7750;
const LILY_TEXT: u32 = 0x0102_fdc8;
/// `Actor::GetAnimation` (Xbox PDB), `(actor)`.
const ACTOR_GET_ANIMATION: u32 = 0x008b_70d0;
/// `Actor::IsWeaponDrawn` (Xbox PDB), `(actor)`.
const ACTOR_IS_WEAPON_DRAWN: u32 = 0x008a_16d0;
/// The global object `0078d200` tests and `0040fba0` releases
/// (`011f11a0`, the lock `Actor::Resurrect` uses).
const ANIMATION_LOCK: u32 = 0x011f_11a0;
const ANIMATION_LOCK_TEST: u32 = 0x0078_d200;
const ANIMATION_LOCK_RELEASE: u32 = 0x0040_fba0;
/// `TESObjectREFR::InitAnimation` (Xbox PDB), `(reference)`.
const REFERENCE_INIT_ANIMATION: u32 = 0x0056_59f0;
/// `ActiveEffect::LoadActiveEffectList` (Xbox PDB), cdecl `(buffer, list)`.
const ACTIVE_EFFECTS_LOAD_OV2: u32 = 0x0080_6a90;
/// A player-character list header constructor `(block)` (`0096a2d0`).
const PLAYER_LIST_CONSTRUCT: u32 = LIST_CONSTRUCT;
/// `QueuedItem::QueuedItem` (this unit, `009139b0`), `(block)`.
const QUEUED_ITEM_CONSTRUCT: u32 = 0x0091_39b0;
/// `QueuedItem::LoadGame` (Xbox PDB), `(item, buffer)`.
const QUEUED_ITEM_LOAD_GAME: u32 = 0x0091_3e80;
/// The RTTI type descriptor of the class `LoadGame_ov2` casts the saved
/// form ids at +0x10C and +0x194 to (`01186a18`).
const TYPE_SAVED_FORM_CLASS: u32 = 0x0118_6a18;

/// Reads `size` bytes of the load buffer into `destination`.
fn load_field(e: &mut Engine, buffer: u32, destination: u32, size: u32) {
    e.call(LOAD_DATA, &args![buffer, destination, size]);
}

/// The next form id of the load buffer looked up and cast to the class at
/// `01186a18`.
fn load_cast_form(e: &mut Engine, buffer: u32) -> u32 {
    let id = e.call(LOAD_FORM_ID_VALUE, &args![buffer]).u32();
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    dynamic_cast(e, form, TYPE_TES_FORM, TYPE_SAVED_FORM_CLASS)
}

/// Whether the reference's name contains `"Lily"`.
fn name_has_lily(e: &mut Engine, reference: u32) -> bool {
    let name = e.call(REFERENCE_NAME, &args![reference]).u32();
    e.call(STRSTR, &args![name, LILY_TEXT]).u32() != 0
}

// Translated from 00927000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00927000 - the C++ exception-unwinding frame (the unwind of the list and QueuedItem allocations) is not translated
/// `MiddleHighProcess::LoadGame_ov2` (Xbox PDB): `(process, buffer)`.
/// The counterpart of [`middle_high_process_save_game_ov2`]: after
/// `MiddleLowProcess::LoadGame_ov2` (`0092eaa0`) reads the same fields in
/// the same order, with the loaded-version gates (buffer version byte:
/// +0x23C from 10, +0x244 from 3, +0x110 from 7), the form ids at +0x10C
/// and +0x194 (looked up and cast), +0x158 and +0x140, the word at the
/// local, the counted id list at +0xC8 and the package (`008c6730`). When
/// the buffer's reference (slot +0x0C) exists: a sub buffer is queued when
/// the save kind has bit 0x10000000, and when it has bit 0x8000020 and
/// the reference has container changes the worn item is restored through
/// slot +0x160, the item change at +0x114 is equipped (`00571760`,
/// slot +0x3EC), the count of +0x118 is limited to what the player has;
/// for a creature whose name contains `"Lily"` the default worn is
/// initialised instead. Then the active effects, the form ids of +0x164,
/// +0x160 and +0x1BC and, from buffer version 15, the counted queued
/// items of the list at +0x230.
pub fn middle_high_process_load_game_ov2(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    buffer: u32,
) {
    let p = this.addr();
    let actor = e.vcall(buffer, 0xc, &[]).u32();
    e.call(MIDDLE_LOW_LOAD_GAME_OV2, &args![p, buffer]);
    for (offset, size) in [
        (0x134u32, 1u32),
        (0x135, 1),
        (0x168, 1),
        (0x170, 4),
        (0x174, 4),
        (0x108, 4),
        (0x1da, 1),
        (0xfc, 0xc),
        (0xdc, 4),
        (0x13d, 1),
    ] {
        load_field(e, buffer, p + offset, size);
    }
    if actor != 0 {
        let state = u32::from(e.mem.u8(p + 0x13d));
        e.call(ACTOR_APPLY_FURNITURE_STATE, &args![actor, state]);
    }
    for (offset, size) in [(0x144u32, 1u32), (0x156, 1), (0x154, 2), (0x148, 0xc)] {
        load_field(e, buffer, p + offset, size);
    }
    let present = e.with_stack(4, |e, cell| {
        e.mem.set_u8(cell.addr(), 0);
        load_field(e, buffer, cell.addr(), 1);
        e.mem.u8(cell.addr())
    });
    if actor != 0 && present as i8 == 0 && e.mem.u8(p + 0x13c) as i8 != 0 {
        e.vcall(p, 0x420, &args![actor]);
    }
    e.mem.set_u8(p + 0x13c, present);
    for (offset, size) in [
        (0xe0u32, 1u32),
        (0x188, 1),
        (0x189, 1),
        (0xd8, 4),
        (0x18b, 1),
        (0x1d0, 4),
        (0x1d4, 4),
        (0x1d8, 1),
        (0x1d9, 1),
        (0x228, 1),
        (0x22a, 2),
        (0x1a8, 4),
        (0xe1, 1),
        (0x190, 4),
        (0x198, 4),
        (0x19c, 1),
        (0x19d, 1),
        (0x234, 4),
        (0x238, 4),
    ] {
        load_field(e, buffer, p + offset, size);
    }
    if e.vcall(buffer, 0x0, &[]).u8() >= 10 {
        load_field(e, buffer, p + 0x23c, 4);
    }
    if e.vcall(buffer, 0x0, &[]).u8() >= 3 {
        load_field(e, buffer, p + 0x244, 4);
    }
    if e.vcall(buffer, 0x0, &[]).u8() >= 7 {
        load_field(e, buffer, p + 0x110, 1);
    }
    let form = load_cast_form(e, buffer);
    e.mem.set_u32(p + 0x10c, form);
    let form = load_cast_form(e, buffer);
    e.mem.set_u32(p + 0x194, form);
    e.call(LOAD_FORM_ID_INTO, &args![buffer, p + 0x158]);
    e.call(LOAD_FORM_ID_INTO, &args![buffer, p + 0x140]);
    let remembered = e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), 0);
        load_field(e, buffer, cell.addr(), 4);
        e.mem.u32(cell.addr())
    });
    let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    for _ in 0..count {
        e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), 0);
            e.call(LOAD_FORM_ID_INTO, &args![buffer, cell]);
            e.call(LIST_ADD, &args![p + 0xc8, cell]);
        });
    }
    e.call(ACTOR_PACKAGE_LOAD_GAME_OV2, &args![p + 0xe4, buffer]);
    if actor != 0 {
        let has_sub_buffer = e.with_stack(4, |e, out| {
            let out = e.call(BUFFER_SAVE_KIND, &args![buffer, out]).u32();
            e.call(KIND_HAS_BITS, &args![out, 0x1000_0000u32]).bool()
        });
        if has_sub_buffer {
            let queue: u32 = e.global(SAVE_LOAD_GAME);
            e.call(QUEUE_SUB_BUFFER, &args![queue, 0u32, buffer, 0u32]);
        }
    }
    let wants_worn = e.with_stack(4, |e, out| {
        let out = e.call(BUFFER_SAVE_KIND, &args![buffer, out]).u32();
        e.call(KIND_HAS_BITS, &args![out, 0x0800_0020u32]).bool()
    });
    if wants_worn {
        let list = e.call(REFERENCE_EXTRA_LIST, &args![actor]).u32();
        let changes = e.call(GET_CONTAINER_CHANGES, &args![list]).u32();
        if changes != 0 {
            let worn = e.call(GET_WORN_ITEM, &args![changes, 5u32, 0u32]).u32();
            // 0092752b: slot 0x1d0 takes no stack word; the pushed 0 is the
            // third word of slot 0x160.
            let model = e.vcall(actor, 0x1d0, &args![]).u32();
            e.vcall(p, 0x160, &args![worn, model, 0u32]);
            let item = e.mem.u32(p + 0x114);
            let form = if item != 0 {
                e.call(ITEM_CHANGE_FORM, &args![item]).u32()
            } else {
                0
            };
            // Equip the item change when the actor has a model and is not
            // the "Lily" creature; otherwise (and for a missing item
            // change or model) look at the "Lily" case below.
            let mut equipped = false;
            if item != 0 && e.vcall(actor, 0x1d0, &[]).u32() != 0 {
                let skip = e.vcall(actor, 0x21c, &[]).bool() && name_has_lily(e, actor);
                if !skip {
                    equipped = true;
                    e.call(ACTOR_EQUIP_WORN, &args![actor, form]);
                    let process = e.call(WORD_68, &args![actor]).u32();
                    e.vcall(process, 0x168, &args![0u32]);
                    let item = e.mem.u32(p + 0x114);
                    let flag = e
                        .call(ITEM_CHANGE_MOD_EFFECT_ACTIVE, &args![item, 2u32])
                        .u8();
                    e.vcall(actor, 0x3ec, &args![form, 0u32, u32::from(flag), 1u32]);
                    if e.mem.u32(p + 0x118) != 0 {
                        let mut wanted = remembered;
                        let player: u32 = e.global(PLAYER);
                        let changes = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
                        if changes != 0 {
                            let held = e.mem.u32(p + 0x118);
                            let held_form = e.call(ITEM_CHANGE_FORM, &args![held]).u32();
                            wanted = e
                                .call(INVENTORY_GET_OBJECT_COUNT, &args![changes, held_form])
                                .u32();
                        }
                        let held = e.mem.u32(p + 0x118);
                        if wanted < remembered {
                            e.call(ITEM_CHANGE_SET_COUNT, &args![held, wanted]);
                            // The exe also stores `wanted` into the remembered count here; nothing reads it afterwards.
                        } else {
                            e.call(ITEM_CHANGE_SET_COUNT, &args![held, remembered]);
                        }
                    }
                }
            }
            if !equipped
                && e.mem.u32(p + 0x114) != 0
                && e.vcall(actor, 0x1d0, &[]).u32() != 0
                && e.vcall(actor, 0x21c, &[]).bool()
                && name_has_lily(e, actor)
            {
                let base = e.call(REFERENCE_BASE, &args![actor]).u32();
                e.call(
                    CREATURE_INIT_DEFAULT_WORN,
                    &args![base, actor, 1u32, 1u32, 0u32],
                );
                let process = e.call(WORD_68, &args![actor]).u32();
                let animation = e.call(ACTOR_GET_ANIMATION, &args![actor]).u32();
                let drawn = e.call(ACTOR_IS_WEAPON_DRAWN, &args![actor]).u8();
                e.vcall(
                    process,
                    0x1cc,
                    &args![u32::from(drawn), 0u32, animation, actor],
                );
                if e.call(ANIMATION_LOCK_TEST, &args![ANIMATION_LOCK]).bool() {
                    e.call(REFERENCE_INIT_ANIMATION, &args![actor]);
                    e.call(ANIMATION_LOCK_RELEASE, &args![ANIMATION_LOCK]);
                }
            }
        }
    }
    let effects = e.mem.u32(p + 0x1b8);
    e.call(ACTIVE_EFFECTS_LOAD_OV2, &args![buffer, effects]);
    e.call(LOAD_FORM_ID_INTO, &args![buffer, p + 0x164]);
    e.call(LOAD_FORM_ID_INTO, &args![buffer, p + 0x160]);
    e.call(LOAD_FORM_ID_INTO, &args![buffer, p + 0x1bc]);
    if e.vcall(buffer, 0x0, &[]).u8() >= 0xf {
        let queued = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
        if queued != 0 {
            let block = e.call(ALLOCATE, &args![8u32]).u32();
            let list = if block != 0 {
                e.call(PLAYER_LIST_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(p + 0x230, list);
            for _ in 0..queued {
                let block = e.call(ALLOCATE, &args![0x18u32]).u32();
                let item = if block != 0 {
                    e.call(QUEUED_ITEM_CONSTRUCT, &args![block]).u32()
                } else {
                    0
                };
                e.call(QUEUED_ITEM_LOAD_GAME, &args![item, buffer]);
                e.with_stack(4, |e, cell| {
                    e.mem.set_u32(cell.addr(), item);
                    let queue = e.mem.u32(p + 0x230);
                    e.call(LIST_ADD, &args![queue, cell]);
                });
            }
        }
    }
}

/// `LowProcess::InitLoadGame_ov2` (Xbox PDB), `(process, buffer)`.
const LOW_INIT_LOAD_GAME_OV2: u32 = 0x0091_0820;
/// `ActorPackage::InitLoadGame_ov2` (Xbox PDB), `(package, buffer)`.
const ACTOR_PACKAGE_INIT_LOAD_GAME_OV2: u32 = 0x008c_6970;
/// activeeffect.cpp, cdecl `(buffer, effect list)`.
const ACTIVE_EFFECTS_INIT_LOAD_OV2: u32 = 0x0080_6b00;
/// `MiddleHighProcess::SetWeaponFlags` (Xbox PDB, this unit), `(process)`.
const SET_WEAPON_FLAGS: u32 = 0x0091_e780;
/// `QueuedItem` scalar deleting destructor shape (this unit, `00915790`),
/// `(item, 1)`.
const QUEUED_ITEM_DELETE: u32 = 0x0091_5790;
/// `QueuedItem` preload for the buffer's reference (this unit,
/// `00913fb0`), `(item, buffer)`.
const QUEUED_ITEM_PRELOAD: u32 = 0x0091_3fb0;
/// `BaseProcess::FinishLoadGame` (Xbox PDB), `(process, buffer)`.
const BASE_FINISH_LOAD_GAME: u32 = 0x008d_1050;
/// `ActorPackage::FinishLoadGame` (Xbox PDB), `(package, buffer)`.
const ACTOR_PACKAGE_FINISH_LOAD_GAME: u32 = 0x008c_6b20;
/// activeeffect.cpp, cdecl `(buffer, effect list)`.
const ACTIVE_EFFECTS_FINISH_LOAD_OV2: u32 = 0x0080_6b50;
/// The form type byte of a furniture-like form.
const FORM_TYPE_27: u32 = 0x27;

/// Walks the list headed at `head`, keeping the entries `keep` accepts and
/// removing the others (the head with `LIST_REMOVE_HEAD`, the others with
/// `LIST_REMOVE` after the previous kept entry, whose next node is taken
/// afterwards). `keep` gets the node and the item stored in it.
fn filter_list(e: &mut Engine, head: u32, mut keep: impl FnMut(&mut Engine, u32, u32) -> bool) {
    let mut node = head;
    let mut previous = 0;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(IDENTITY, &args![node]).u32();
        let item = e.mem.u32(slot);
        if keep(e, node, item) {
            previous = node;
            node = e.call(WORD_4, &args![node]).u32();
        } else if previous == 0 {
            e.call(LIST_REMOVE_HEAD, &args![node]);
        } else {
            let slot = e.call(IDENTITY, &args![node]).u32();
            e.call(LIST_REMOVE, &args![previous, slot]);
            node = e.call(WORD_4, &args![previous]).u32();
        }
    }
}

// Translated from 009278d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::InitLoadGame_ov2` (Xbox PDB): `(process, buffer)`.
/// With the buffer's reference (slot +0x0C) as the actor: runs
/// `LowProcess::InitLoadGame_ov2` (`00910820`), turns the saved form ids
/// at +0x158 (an `Actor`) and +0x140 (a `TESObjectREFR`) into pointers,
/// resolves the id list at +0xC8 (dropping what does not resolve), runs
/// the package (`008c6970`) and the active effects (`00806b00`). A
/// furniture reference (+0x140) of form type 0x27 in the actor's cell
/// stays; any other, or one in another cell or for an actor that fails
/// slot +0x22C, is dropped through the process's slot +0x84; without a
/// reference the furniture state (+0x13D) is cleared (and the player's
/// furniture state). A reserved marker slot (+0x1A8) is reserved on the
/// target (+0x40). With an actor `SetWeaponFlags` (`0091e780`) runs, then
/// slots +0x6E4 and +0x7A8. The magic items at +0x164 and +0x160 and the
/// target at +0x1BC are resolved by id; from buffer version 15 the queued
/// items of +0x230 are kept when their object is set and preloaded, and
/// deleted otherwise.
pub fn middle_high_process_init_load_game_ov2(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    buffer: u32,
) {
    let p = this.addr();
    let actor = e.vcall(buffer, 0xc, &[]).u32();
    e.call(LOW_INIT_LOAD_GAME_OV2, &args![p, buffer]);
    let id = e.mem.u32(p + 0x158);
    let resolved = if id == 0 {
        0
    } else {
        form_cast(e, id, TYPE_ACTOR)
    };
    e.mem.set_u32(p + 0x158, resolved);
    let id = e.mem.u32(p + 0x140);
    let resolved = if id == 0 {
        0
    } else {
        form_cast(e, id, TYPE_TES_OBJECT_REFR)
    };
    e.mem.set_u32(p + 0x140, resolved);
    filter_list(e, p + 0xc8, |e, node, id| {
        if id == 0 {
            return false;
        }
        let resolved = form_cast(e, id, TYPE_TES_OBJECT_REFR);
        if resolved == 0 {
            return false;
        }
        e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), resolved);
            e.call(NODE_SET_ITEM, &args![node, cell]);
        });
        true
    });
    e.call(ACTOR_PACKAGE_INIT_LOAD_GAME_OV2, &args![p + 0xe4, buffer]);
    let effects = e.mem.u32(p + 0x1b8);
    e.call(ACTIVE_EFFECTS_INIT_LOAD_OV2, &args![buffer, effects]);
    let reference = e.mem.u32(p + 0x140);
    if reference != 0 {
        let base = e.call(REFERENCE_BASE, &args![reference]).u32();
        if e.call(FORM_TYPE, &args![base]).u32() == FORM_TYPE_27 {
            if actor != 0 {
                let mut drop_reference = e.vcall(actor, 0x22c, &args![0u32]).bool();
                if !drop_reference {
                    let actor_cell = e.call(REFERENCE_CELL, &args![actor]).u32();
                    let reference = e.mem.u32(p + 0x140);
                    let reference_cell = e.call(REFERENCE_CELL, &args![reference]).u32();
                    drop_reference = actor_cell != reference_cell;
                }
                if drop_reference {
                    e.vcall(p, 0x84, &args![actor]);
                }
            }
        } else {
            e.vcall(p, 0x84, &args![actor]);
        }
    } else if e.mem.u8(p + 0x13d) != 0 {
        e.mem.set_u8(p + 0x13d, 0);
        let player: u32 = e.global(PLAYER);
        if actor == player {
            e.call(PLAYER_CLEAR_FURNITURE, &args![player]);
        }
    }
    let slot = e.mem.u32(p + 0x1a8);
    if (slot as i32) >= 0 && e.mem.u32(p + 0x40) != 0 {
        let target = e.mem.u32(p + 0x40);
        e.call(SET_MARKER_RESERVED, &args![target, slot, 1u32]);
    }
    if actor != 0 {
        e.call(SET_WEAPON_FLAGS, &args![p]);
        e.vcall(p, 0x6e4, &args![actor]);
    }
    if actor != 0 {
        e.vcall(p, 0x7a8, &args![actor]);
    }
    for offset in [0x164u32, 0x160] {
        let id = e.mem.u32(p + offset);
        let resolved = if id == 0 {
            0
        } else {
            e.call(MAGIC_ITEM_BY_NUMERIC_ID, &args![id]).u32()
        };
        e.mem.set_u32(p + offset, resolved);
    }
    let id = e.mem.u32(p + 0x1bc);
    let resolved = if id == 0 {
        0
    } else {
        e.call(MAGIC_TARGET_BY_NUMERIC_ID, &args![id]).u32()
    };
    e.mem.set_u32(p + 0x1bc, resolved);
    if e.vcall(buffer, 0x0, &[]).u8() >= 0xf {
        let head = e.mem.u32(p + 0x230);
        filter_list(e, head, |e, _node, item| {
            if item == 0 {
                return false;
            }
            if e.mem.u32(item) == 0 {
                e.call(QUEUED_ITEM_DELETE, &args![item, 1u32]);
                return false;
            }
            e.call(QUEUED_ITEM_PRELOAD, &args![item, buffer]);
            true
        });
    }
}

// Translated from 00927d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::FinishLoadGame` (Xbox PDB): `(process, buffer)`.
/// With the buffer's reference (slot +0x0C) as the actor: runs
/// `BaseProcess::FinishLoadGame` (`008d1050`), the package
/// (`008c6b20`) and the active effects (`00806b50`). With a furniture
/// reference (+0x140) in state 4 or 9 the actor is placed on the
/// furniture's marker as `FinishInitLoadGame` does (no check of the
/// index against the marker count); the process's slot +0x84 is called
/// with the actor when the furniture has no markers.
pub fn middle_high_process_finish_load_game(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    buffer: u32,
) {
    let p = this.addr();
    let actor = e.vcall(buffer, 0xc, &[]).u32();
    e.call(BASE_FINISH_LOAD_GAME, &args![p, buffer]);
    e.call(ACTOR_PACKAGE_FINISH_LOAD_GAME, &args![p + 0xe4, buffer]);
    let effects = e.mem.u32(p + 0x1b8);
    e.call(ACTIVE_EFFECTS_FINISH_LOAD_OV2, &args![buffer, effects]);
    let reference = e.mem.u32(p + 0x140);
    if reference != 0 {
        let state = e.mem.u8(p + 0x13d);
        if state == 4 || state == 9 {
            let base = e.call(REFERENCE_BASE, &args![reference]).u32();
            let model = e.vcall(reference, 0x1d0, &[]).u32();
            let markers = e.call(FURNITURE_MARKER_FIND, &args![model]).u32();
            if markers == 0 {
                e.vcall(p, 0x84, &args![actor]);
            } else {
                finish_load_place_on_marker(e, p, actor, reference, base, markers, false);
            }
        }
    }
}

/// `MiddleLowProcess::Revert_ov2` (Xbox PDB), `(process, reverter)`.
const MIDDLE_LOW_REVERT_OV2: u32 = 0x0092_eb00;
/// `(buffer, out)`: stores the word at +0x2C of the object in `out` and
/// answers `out` (the engine has it as `fn_0042ce30`).
const BUFFER_FLAGS_B: u32 = 0x0042_ce30;
/// `ActorPackage::Revert_ov2` shape (actorpackage.cpp), `(package,
/// reverter)`.
const ACTOR_PACKAGE_REVERT_OV2: u32 = 0x008c_6c80;
/// animation.cpp, `(animation, reverter)`.
const ANIMATION_REVERT_OV2: u32 = 0x0049_bb70;
/// activeeffect.cpp, cdecl `(reverter, effect list)`.
const ACTIVE_EFFECTS_REVERT_OV2: u32 = 0x0080_6ba0;
/// `(object)`: whether bit 2 of the word at +0x28 is set (tesobjectrefr.cpp).
const OBJECT_FLAG_4: u32 = 0x0056_2d00;
/// The deferred-hide-limb entry destructor `(entry, 1)` (this unit,
/// [`fn_00914ab0`]).
const DEFERRED_HIDE_LIMB_DELETE: u32 = 0x0091_4ab0;

// Translated from 009280f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::Revert_ov2` (Xbox PDB): `(process, reverter)`.
/// After `MiddleLowProcess::Revert_ov2` (`0092eb00`): when the reverter's
/// word at +0x2C has a bit of 0x8000020 the process's slots +0x160 (0, 0,
/// 0), +0x168, +0x16C and +0x164 run with 0 and +0x16C is cleared. Reverts
/// the package (`008c6c80`), takes the reverter's slot +0x0C as the actor
/// (slot +0x4C0 with (actor, 0, 0, 0x7F) for one), resets every saved
/// field to its initial value (the lists at +0xC8 and +0xD0 emptied,
/// -1.0 for the float at +0x198, three words at +0x1C4 cleared), reverts
/// the animation (`0049bb70`, for bit 0x10000000 with +0x1C0 set) and the
/// active effects (`00806ba0`), deletes the entries of the deferred hide
/// limb list at +0x1B0, and runs slot +0x180; with an actor and the
/// reverter's flag `00562d00` slot +0x420 runs with the actor.
pub fn middle_high_process_revert_ov2(e: &mut Engine, this: Ptr<MiddleHighProcess>, reverter: u32) {
    let p = this.addr();
    e.call(MIDDLE_LOW_REVERT_OV2, &args![p, reverter]);
    let wants_reset = e.with_stack(4, |e, out| {
        let out = e.call(BUFFER_FLAGS_B, &args![reverter, out]).u32();
        e.call(KIND_HAS_BITS, &args![out, 0x0800_0020u32]).bool()
    });
    if wants_reset {
        e.vcall(p, 0x160, &args![0u32, 0u32, 0u32]);
        e.vcall(p, 0x168, &args![0u32]);
        e.vcall(p, 0x16c, &args![0u32]);
        e.vcall(p, 0x164, &args![0u32]);
        e.mem.set_u32(p + 0x16c, 0);
    }
    e.call(ACTOR_PACKAGE_REVERT_OV2, &args![p + 0xe4, reverter]);
    let actor = e.vcall(reverter, 0xc, &[]).u32();
    if actor != 0 {
        e.vcall(p, 0x4c0, &args![actor, 0u32, 0u32, 0x7fu32]);
    }
    e.mem.set_u32(p + 0x140, 0);
    e.mem.set_u8(p + 0x144, 0x7f);
    e.call(LIST_CLEAR, &args![p + 0xc8]);
    e.mem.set_u8(p + 0x124, 0);
    e.mem.set_u8(p + 0x125, 0);
    e.mem.set_u8(p + 0x126, 0);
    e.mem.set_u32(p + 0x158, 0);
    e.mem.set_u32(p + 0x15c, 0);
    e.mem.set_u8(p + 0x135, 0);
    e.mem.set_u8(p + 0x134, 0);
    e.mem.set_f32(p + 0xd8, 0.0);
    e.mem.set_u8(p + 0x1da, 0);
    e.mem.set_u32(p + 0x108, 0);
    e.mem.set_f32(p + 0xdc, 0.0);
    e.mem.set_u8(p + 0x168, 0);
    e.mem.set_u32(p + 0x16c, 0);
    e.mem.set_f32(p + 0x170, 1.0);
    e.mem.set_f32(p + 0x174, 0.0);
    e.mem.set_u32(p + 0x184, 0);
    e.mem.set_u8(p + 0x180, 0);
    e.mem.set_u8(p + 0xe0, 0);
    e.mem.set_u8(p + 0x188, 0);
    e.mem.set_u8(p + 0x189, 0);
    e.mem.set_u8(p + 0x17c, 0);
    e.mem.set_u8(p + 0x17d, 0);
    e.mem.set_u8(p + 0x17e, 0);
    e.mem.set_u8(p + 0x17f, 0);
    e.mem.set_u32(p + 0x68, 0);
    e.mem.set_u8(p + 0x18a, 0);
    e.call(LIST_CLEAR, &args![p + 0xd0]);
    e.mem.set_u16(p + 0x22a, 2);
    e.mem.set_u32(p + 0x1a8, u32::MAX);
    e.mem.set_u32(p + 0x22c, 0);
    e.mem.set_u8(p + 0xe1, 0);
    e.mem.set_u32(p + 0x190, u32::MAX);
    // The float is copied through the x87 stack (`FLD`/`FSTP`).
    let minus_one = x87_quiet(e.global::<f32>(MINUS_ONE));
    e.mem.set_f32(p + 0x198, minus_one);
    e.mem.set_u8(p + 0x19c, 0);
    e.mem.set_u8(p + 0x19d, 0);
    e.mem.set_u32(p + 0x1a4, 0);
    e.mem.set_f32(p + 0x234, 0.0);
    e.mem.set_f32(p + 0x238, 0.0);
    e.mem.set_f32(p + 0x23c, 0.0);
    e.mem.set_u32(p + 0x244, u32::MAX);
    e.mem.set_u8(p + 0x110, 0);
    e.mem.set_u32(p + 0x248, 0);
    e.mem.set_u32(p + 0x24c, 0);
    e.mem.set_u32(p + 0x250, 0);
    e.mem.set_u32(p + 0x10c, 0);
    e.mem.set_u32(p + 0x194, 0);
    for index in 0..3u32 {
        e.mem.set_u32(p + 0x1c4 + index * 4, 0);
    }
    let reverts_animation = e.with_stack(4, |e, out| {
        let out = e.call(BUFFER_FLAGS_B, &args![reverter, out]).u32();
        e.call(KIND_HAS_BITS, &args![out, 0x1000_0000u32]).bool()
    });
    if reverts_animation && e.mem.u32(p + 0x1c0) != 0 {
        let animation = e.mem.u32(p + 0x1c0);
        e.call(ANIMATION_REVERT_OV2, &args![animation, reverter]);
    }
    let effects = e.mem.u32(p + 0x1b8);
    e.call(ACTIVE_EFFECTS_REVERT_OV2, &args![reverter, effects]);
    while !e.call(LIST_IS_EMPTY, &args![p + 0x1b0]).bool() {
        let slot = e.call(IDENTITY, &args![p + 0x1b0]).u32();
        let entry = e.mem.u32(slot);
        if entry != 0 {
            e.call(DEFERRED_HIDE_LIMB_DELETE, &args![entry, 1u32]);
        }
        e.call(LIST_REMOVE_HEAD, &args![p + 0x1b0]);
    }
    e.vcall(p, 0x180, &[]);
    if actor != 0 && e.call(OBJECT_FLAG_4, &args![reverter]).bool() {
        e.vcall(p, 0x420, &args![actor]);
    }
}

/// `PlayerCharacter` state setter `(player, flag)` used around the furniture
/// idle (the engine map has no name).
const PLAYER_SET_ANIMATION_STATE: u32 = 0x005c_4210;
/// The idle manager object (word at this address).
const IDLE_MANAGER: u32 = 0x011c_b6a0;
/// `TESIdleManager::GetIdleToPlay` (Xbox PDB), `(manager, actor,
/// reference)`.
const GET_IDLE_TO_PLAY: u32 = 0x0060_0950;
/// `TESIdleForm::GetIdleAnimGroupSection` (Xbox PDB), `(idle)`.
const IDLE_ANIM_GROUP_SECTION: u32 = 0x005f_f160;
/// `Animation::SpecialIdleReplace` (Xbox PDB), `(animation, idle, actor,
/// section)`.
const SPECIAL_IDLE_REPLACE: u32 = 0x0049_8030;
/// `Animation::SpecialIdleFree` (Xbox PDB), `(animation, 1, 0)`.
const SPECIAL_IDLE_FREE: u32 = 0x0049_8910;
/// The float 0.1 (`FLD`/`FSTP` copied) the idle blend is set to.
const IDLE_BLEND_RATE: u32 = 0x0101_e2bc;

// Translated from 00928510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::LoadSitSleepIdle` (Xbox PDB): `(process, actor,
/// animation)`. With a furniture reference (+0x140): when its markers
/// (`BSFurnitureMarker::Find` on the reference's slot +0x1D0 value) are
/// missing or the marker index (+0x144) is out of range, the process's slot
/// +0x84 runs with the actor. Otherwise the furniture state (+0x13D) 1 to 4
/// is taken as 1 and 6 to 9 as 6 for `00920fb0`; with a state the marker is
/// marked used, the player's animation state is switched around the idle
/// (`004eaf60`, `PlayerCharacter::GetAnimation`, `005c4210`) and the idle to
/// play is applied to `animation`. The saved state is restored, the actor's
/// slot +0x2E0 is called with 0.1 (and `SetChaseBip` with 1, the controller
/// flag 0x8000000 set) when slot +0x2B4 answers true with a state, else
/// with 0 (chase 0, flag cleared), and `00920fb0` runs with the state.
pub fn middle_high_process_load_sit_sleep_idle(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    animation: u32,
) {
    let p = this.addr();
    let reference = e.mem.u32(p + 0x140);
    if reference == 0 {
        return;
    }
    e.call(REFERENCE_BASE, &args![reference]);
    let model = e.vcall(reference, 0x1d0, &[]).u32();
    let markers = e.call(FURNITURE_MARKER_FIND, &args![model]).u32();
    if markers == 0 || e.mem.u8(p + 0x144) as u32 >= e.call(MARKER_COUNT, &args![markers]).u32() {
        e.vcall(p, 0x84, &args![actor]);
        return;
    }
    let saved_state = e.mem.u8(p + 0x13d);
    match saved_state {
        1..=4 => e.mem.set_u8(p + 0x13d, 1),
        6..=9 => e.mem.set_u8(p + 0x13d, 6),
        _ => {}
    }
    let state = e.mem.u8(p + 0x13d);
    e.call(ACTOR_APPLY_FURNITURE_STATE, &args![actor, state as u32]);
    if e.mem.u8(p + 0x13d) != 0 {
        let index = e.mem.u8(p + 0x144) as u32;
        let reference = e.mem.u32(p + 0x140);
        e.call(REFERENCE_SET_MARKER_USED, &args![reference, index, 1u32]);
        let mut lighting = 0u8;
        let player: u32 = e.global(PLAYER);
        if actor == player {
            lighting = e.call(PLAYER_LIGHTING_TEST, &args![player]).u8();
            let flag = (lighting == 0) as u32;
            let current = e.call(PLAYER_GET_ANIMATION, &args![player, flag]).u32();
            if animation != current {
                e.call(PLAYER_SET_ANIMATION_STATE, &args![player, flag]);
            }
        }
        let manager: u32 = e.global(IDLE_MANAGER);
        let reference = e.mem.u32(p + 0x140);
        let idle = e
            .call(GET_IDLE_TO_PLAY, &args![manager, actor, reference])
            .u32();
        if idle != 0 {
            let section = e.call(IDLE_ANIM_GROUP_SECTION, &args![idle]).u32();
            e.call(
                SPECIAL_IDLE_REPLACE,
                &args![animation, idle, actor, section],
            );
        }
        if actor == player {
            e.call(PLAYER_SET_ANIMATION_STATE, &args![player, lighting as u32]);
        }
    }
    e.mem.set_u8(p + 0x13d, saved_state);
    let marker_byte = e.call(MARK_BYTE_E, &args![p + 0x148]).u32();
    let reference = e.mem.u32(p + 0x140);
    let reserved = e.vcall(p, 0x2b4, &args![reference, marker_byte]).u8();
    let controller = e.vcall(p, 0x28c, &[]).u32();
    if reserved != 0 && e.mem.u8(p + 0x13d) != 0 {
        let blend = x87_quiet_bits(e.global::<u32>(IDLE_BLEND_RATE));
        e.vcall(actor, 0x2e0, &args![blend]);
        e.call(SET_CHASE_BIP, &args![actor, 1u32]);
        if controller != 0 {
            e.call(
                CONTROLLER_SET_STATE_FLAGS,
                &args![controller + 0x410, 0x0800_0000u32, 1u32],
            );
        }
    } else {
        e.vcall(actor, 0x2e0, &args![0u32]);
        e.call(SET_CHASE_BIP, &args![actor, 0u32]);
        if controller != 0 {
            e.call(
                CONTROLLER_SET_STATE_FLAGS,
                &args![controller + 0x410, 0x0800_0000u32, 0u32],
            );
        }
    }
    let state = e.mem.u8(p + 0x13d);
    e.call(ACTOR_APPLY_FURNITURE_STATE, &args![actor, state as u32]);
}

// Translated from 009287e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ClearFurniture` (Xbox PDB): `(process, actor)`. With
/// a furniture reference (+0x140) its marker (+0x144) is marked unused and,
/// when the actor has an animation (slot +0x1E4), `SpecialIdleFree` runs on
/// it with (1, 0). The process's slot +0x4C0 then runs with (actor, 0, 0,
/// 0x7F), the player's furniture state is cleared when the actor is the
/// player (`005098c0`) and `0092c680` runs.
pub fn middle_high_process_clear_furniture(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    let p = this.addr();
    let reference = e.mem.u32(p + 0x140);
    if reference != 0 {
        let index = e.mem.u8(p + 0x144) as u32;
        e.call(REFERENCE_SET_MARKER_USED, &args![reference, index, 0u32]);
        if e.vcall(actor, 0x1e4, &[]).u32() != 0 {
            let animation = e.vcall(actor, 0x1e4, &[]).u32();
            e.call(SPECIAL_IDLE_FREE, &args![animation, 1u32, 0u32]);
        }
    }
    e.vcall(p, 0x4c0, &args![actor, 0u32, 0u32, 0x7fu32]);
    let player: u32 = e.global(PLAYER);
    if actor == player {
        e.call(PLAYER_CLEAR_FURNITURE, &args![player]);
    }
    e.call(CLEAR_FURNITURE_MARKER, &args![p]);
}

// Translated from 00928880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SaveCharController_ov2` (Xbox PDB): `(process,
/// buffer)`. The same record as [`middle_high_process_save_char_controller`]
/// written through the save buffer (`SAVE_DATA` with the buffer's reference,
/// slot +0x08, as the actor): the controller's saved word (1 byte), +0x520
/// (1 byte), the state flags (4), the position (12, when flag 0x800 is set),
/// the 16 byte values at +0x4F0 and +0x500, the linear velocity (12), then
/// +0x540/+0x544 (with an actor) or +0x54C, and +0x548, +0x550, +0x558.
/// Nothing is written without a controller (slot +0x28C) or for an actor
/// whose slot +0x22C is true.
pub fn middle_high_process_save_char_controller_ov2(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    buffer: u32,
) {
    let p = this.addr();
    let actor = e.vcall(buffer, 0x8, &[]).u32();
    let controller = e.vcall(p, 0x28c, &[]).u32();
    if controller == 0 {
        return;
    }
    if actor != 0 && e.vcall(actor, 0x22c, &args![0u32]).bool() {
        return;
    }
    e.with_stack(0x24, |e, locals| {
        let cell = locals.addr();
        let position = cell + 4;
        let velocity = cell + 0x10;
        let saved = e
            .call(CONTROLLER_SAVED_WORD, &args![controller + 0x3e0])
            .u8();
        e.mem.set_u8(cell, saved);
        save_field(e, buffer, cell, 1);
        let value = e.mem.u8(controller + 0x520);
        e.mem.set_u8(cell, value);
        save_field(e, buffer, cell, 1);
        let flags = controller_state_flags(e, controller, u32::MAX);
        e.mem.set_u32(cell, flags);
        save_field(e, buffer, cell, 4);
        if flags & 0x800 != 0 {
            e.call(IDENTITY, &args![position]);
            e.call(CONTROLLER_GET_POSITION, &args![controller, position]);
            save_field(e, buffer, position, 0xc);
        }
        save_field(e, buffer, controller + 0x4f0, 0x10);
        save_field(e, buffer, controller + 0x500, 0x10);
        e.call(IDENTITY, &args![velocity]);
        e.call(CONTROLLER_GET_LINEAR_VELOCITY, &args![controller, velocity]);
        save_field(e, buffer, velocity, 0xc);
        if actor != 0 {
            save_field(e, buffer, controller + 0x540, 4);
            save_field(e, buffer, controller + 0x544, 4);
        } else {
            save_field(e, buffer, controller + 0x54c, 4);
        }
        save_field(e, buffer, controller + 0x548, 4);
        save_field(e, buffer, controller + 0x550, 4);
        save_field(e, buffer, controller + 0x558, 4);
    });
}

/// `(character controller area at +0x450, 16 byte value)`: set through
/// `004a3f10` (the engine map has no name), see [`fn_00928d00`].
const CONTROLLER_SET_VALUE_450: u32 = 0x004a_3f10;
/// `MobileObject` reference position getter used on the controller: the
/// value `00458b20` answers (no arguments; the engine map has no name).
const CONTROLLER_NEXT_VALUE: u32 = 0x0045_8b20;
/// `(controller, value)` (`0080fb00`, the engine map has no name); the
/// answer is not used here.
const CONTROLLER_APPLY_VALUE: u32 = 0x0080_fb00;
/// `(out, x, y, z, w)` constructor of a 16 byte value (`005532a0`, the
/// engine map has no name).
const CONSTRUCT_QUATERNION: u32 = 0x0055_32a0;

// Translated from 00928a50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00928a50 - the /GS stack cookie check (00ec408c) is not translated
/// `MiddleHighProcess::LoadCharController` (Xbox PDB): `(process, buffer)`,
/// reading what [`middle_high_process_save_char_controller_ov2`] wrote
/// (`LOAD_DATA` on the buffer; the buffer's reference, slot +0x0C, is the
/// actor). With a controller (slot +0x28C): sets the movement mode
/// (`008e2680`) from a byte, reads +0x520 (a byte), the state flags
/// (cleared with -1, then set), the position (12, when flag 0x800 is set,
/// set with `005620e0`), `SetChaseBip(0)` for an actor whose slot +0x214
/// answers 0, the 16 byte values at +0x4F0 and +0x500, the linear velocity
/// (`00926980`), +0x540/+0x544 (with an actor) or +0x54C, then +0x548,
/// +0x550 and +0x558. For an actor in state 4 or 9 (slot +0x214) the flag
/// 0x8000000 is set when slot +0x2B4 answers true for the furniture marker,
/// the movement mode reset to 0, and `00928d00` receives the 16 byte value
/// built by `005532a0` with (0.0, 0.0, 1.0, 0.0) after `0080fb00` ran with
/// `00458b20`'s answer.
pub fn middle_high_process_load_char_controller(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    buffer: u32,
) {
    let p = this.addr();
    let actor = e.vcall(buffer, 0xc, &[]).u32();
    let controller = e.vcall(p, 0x28c, &[]).u32();
    if controller == 0 {
        return;
    }
    e.with_stack(0x40, |e, locals| {
        let cell = locals.addr();
        let position = cell + 8;
        let velocity = cell + 0x14;
        let value = cell + 0x20;
        e.mem.set_u8(cell, 0);
        e.call(LOAD_DATA, &args![buffer, cell, 1u32]);
        let mode = e.mem.u8(cell) as u32;
        e.call(CONTROLLER_SET_MOVE_MODE, &args![controller, mode]);
        e.mem.set_u8(cell, 0);
        e.call(LOAD_DATA, &args![buffer, cell, 1u32]);
        let word = e.mem.u8(cell) as u32;
        e.mem.set_u32(controller + 0x520, word);
        e.mem.set_u32(cell, 0);
        e.call(LOAD_DATA, &args![buffer, cell, 4u32]);
        let flags = e.mem.u32(cell);
        e.call(
            CONTROLLER_CLEAR_STATE_FLAGS,
            &args![controller + 0x410, u32::MAX],
        );
        e.call(
            CONTROLLER_SET_STATE_FLAGS,
            &args![controller + 0x410, flags, 1u32],
        );
        if flags & 0x800 != 0 {
            e.call(IDENTITY, &args![position]);
            e.call(LOAD_DATA, &args![buffer, position, 0xcu32]);
            e.call(CONTROLLER_SET_POSITION, &args![controller, position]);
        }
        if actor != 0 && e.vcall(actor, 0x214, &[]).u32() == 0 {
            e.call(SET_CHASE_BIP, &args![actor, 0u32]);
        }
        e.call(LOAD_DATA, &args![buffer, controller + 0x4f0, 0x10u32]);
        e.call(LOAD_DATA, &args![buffer, controller + 0x500, 0x10u32]);
        e.call(IDENTITY, &args![velocity]);
        e.call(LOAD_DATA, &args![buffer, velocity, 0xcu32]);
        fn_00926980(e, controller, velocity);
        if actor != 0 {
            e.call(LOAD_DATA, &args![buffer, controller + 0x540, 4u32]);
            e.call(LOAD_DATA, &args![buffer, controller + 0x544, 4u32]);
        } else {
            e.call(LOAD_DATA, &args![buffer, controller + 0x54c, 4u32]);
        }
        e.call(LOAD_DATA, &args![buffer, controller + 0x548, 4u32]);
        e.call(LOAD_DATA, &args![buffer, controller + 0x550, 4u32]);
        e.call(LOAD_DATA, &args![buffer, controller + 0x558, 4u32]);
        let state = if actor != 0 {
            e.vcall(actor, 0x214, &[]).u32()
        } else {
            0
        };
        if state == 4 || state == 9 {
            let marker_byte = e.call(MARK_BYTE_E, &args![p + 0x148]).u32();
            let reference = e.mem.u32(p + 0x140);
            if e.vcall(p, 0x2b4, &args![reference, marker_byte]).u8() != 0 {
                e.call(
                    CONTROLLER_SET_STATE_FLAGS,
                    &args![controller + 0x410, 0x0800_0000u32, 1u32],
                );
            }
            e.call(CONTROLLER_SET_MOVE_MODE, &args![controller, 0u32]);
            let next = e.call(CONTROLLER_NEXT_VALUE, &[]).u32();
            e.call(CONTROLLER_APPLY_VALUE, &args![controller, next]);
            e.call(
                CONSTRUCT_QUATERNION,
                &args![value, 0.0f32, 0.0f32, 1.0f32, 0.0f32],
            );
            fn_00928d00(e, controller, value);
        }
    });
}

// Translated from 00928d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the 16 byte value to the object at +0x450 of the character
/// controller (`004a3f10`); the engine map has no name. `(controller,
/// value)`.
pub fn fn_00928d00(e: &mut Engine, this: u32, value: u32) {
    e.call(CONTROLLER_SET_VALUE_450, &args![this + 0x450, value]);
}

// Translated from 00928d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetShouldRefreshMagicShadersEffects` (Xbox PDB):
/// `(process, a, b, c)`. Sets the refresh flag (+0x17C) and ors `b` into
/// +0x17D, `a` into +0x17E and `c` into +0x17F.
pub fn middle_high_process_set_should_refresh_magic_shaders_effects(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    a: u8,
    b: u8,
    c: u8,
) {
    let p = this.addr();
    e.mem.set_u8(p + 0x17c, 1);
    let value = e.mem.u8(p + 0x17d) | b;
    e.mem.set_u8(p + 0x17d, value);
    let value = e.mem.u8(p + 0x17e) | a;
    e.mem.set_u8(p + 0x17e, value);
    let value = e.mem.u8(p + 0x17f) | c;
    e.mem.set_u8(p + 0x17f, value);
}

/// `(process lists, actor)`: the magic shader effect found for the actor
/// (`00974cb0`, the engine map has no name); 0 for none.
const PROCESS_LISTS_FIND_EFFECT: u32 = 0x0097_4cb0;
/// `ProcessLists::FindAndCleanupWeaponShaderHitEffect` (Xbox PDB),
/// `(process lists, actor, effect form)`.
const PROCESS_LISTS_FIND_WEAPON_EFFECT: u32 = 0x0097_4db0;
/// `ProcessLists::AddTempEffect` (Xbox PDB), `(process lists, effect)`.
const PROCESS_LISTS_ADD_TEMP_EFFECT: u32 = 0x0097_3fd0;
/// `TESEnchantableForm::GetFormEnchanting` (Xbox PDB), cdecl `(form)`.
const FORM_ENCHANTING: u32 = 0x004b_e330;
/// `(item change)`: the float the enchantment is checked against (answers
/// in ST0; the engine map has no name).
const ITEM_CHARGE: u32 = 0x004b_d350;
/// `MagicItem::GetAVEffect` (Xbox PDB), `(enchantment + 0x18, 0)`.
const MAGIC_ITEM_GET_AV_EFFECT: u32 = 0x0040_a300;
/// `(effect setting)`: its form (`00407840`, the engine map has no name).
const EFFECT_SETTING_FORM: u32 = 0x0040_7840;
/// The allocator of the effect objects, `(size)`.
const EFFECT_ALLOCATE: u32 = 0x00aa_13e0;
/// Constructor of the 0x6C byte shader hit effect `(object, actor, effect
/// form, -1.0)` (`0081f580`, the engine map has no name).
const SHADER_EFFECT_CONSTRUCT: u32 = 0x0081_f580;
/// `(player)`: whether the player has a shader effect slot (`00490f80`, the
/// engine map has no name).
const PLAYER_HAS_SHADER_SLOT: u32 = 0x0049_0f80;
/// `(player)`: the player's current shader effect (`00820bf0`, the engine
/// map has no name).
const PLAYER_SHADER_EFFECT: u32 = 0x0082_0bf0;
/// The float 100.0 the effect's timer is set to.
const SHADER_EFFECT_TIMER: u32 = 0x0101_6410;

/// Calls the object's deleting destructor (slot 0 with 1).
fn delete_object(e: &mut Engine, object: u32) {
    if object != 0 {
        e.vcall(object, 0x0, &args![1u32]);
    }
}

// Translated from 00928d90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00928d90 - the exception frame (FS:[0]) is not translated
/// `MiddleHighProcess::RefreshMagicShaderEffects` (Xbox PDB): `(process,
/// actor)`. With +0x17D set, slot +0xC4 of the actor's magic shader effect
/// (`00974cb0`) runs. With +0x17E set: slot +0x578 runs with 0 and, when the
/// actor's saved acquire object (`008d8520`) has a slot +0x148 answer whose
/// change form has an enchantment with a charge check (slot +8 of the
/// enchantment's object at +0x24 with the actor, answering at most
/// `004bd350`) and an effect setting form, slot +0x578 runs with that form.
/// The weapon shader hit effect of slot +0x574 (`00974db0`) is refreshed, or
/// a new one (0x6C bytes, `0081f580`, flag set with `00929260`, timer 100.0
/// when +0x17F is set) is started and added (`00973fd0`). For the player
/// (when `00490f80` says so) the shader effect kept at the player
/// (`00820bf0`, `00929220`) is replaced, created or deleted to match slot
/// +0x574. All four flags +0x17C to +0x17F are cleared.
pub fn middle_high_process_refresh_magic_shader_effects(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    let p = this.addr();
    if e.mem.u8(p + 0x17d) != 0 {
        let effect = e
            .call(PROCESS_LISTS_FIND_EFFECT, &args![PROCESS_LISTS, actor])
            .u32();
        if effect != 0 {
            e.vcall(effect, 0xc4, &[]);
        }
    }
    if e.mem.u8(p + 0x17e) != 0 {
        e.vcall(p, 0x578, &args![0u32]);
        if e.call(WORD_68, &args![actor]).u32() != 0 {
            let acquired = e.call(WORD_68, &args![actor]).u32();
            if e.vcall(acquired, 0x148, &[]).u32() != 0 {
                let item = e.vcall(p, 0x148, &[]).u32();
                let change_form = e.call(ITEM_CHANGE_FORM, &args![item]).u32();
                let enchanting = e.call(FORM_ENCHANTING, &args![change_form]).u32();
                if enchanting != 0 {
                    let acquired = e.call(WORD_68, &args![actor]).u32();
                    let item = e.vcall(acquired, 0x148, &[]).u32();
                    let charge = e.call(ITEM_CHARGE, &args![item]).f64();
                    let value = e.vcall(enchanting + 0x24, 8, &args![actor]).f64();
                    if value <= charge {
                        let av_effect = e
                            .call(MAGIC_ITEM_GET_AV_EFFECT, &args![enchanting + 0x18, 0u32])
                            .u32();
                        if av_effect != 0
                            && e.call(EFFECT_SETTING_FORM, &args![av_effect]).u32() != 0
                        {
                            let form = e.call(EFFECT_SETTING_FORM, &args![av_effect]).u32();
                            e.vcall(p, 0x578, &args![form]);
                        }
                    }
                }
            }
        }
        let mut started_untimed = false;
        let form = e.vcall(p, 0x574, &[]).u32();
        let existing = e
            .call(
                PROCESS_LISTS_FIND_WEAPON_EFFECT,
                &args![PROCESS_LISTS, actor, form],
            )
            .u32();
        if existing != 0 {
            e.vcall(existing, 0xc4, &[]);
        } else if e.vcall(p, 0x574, &[]).u32() != 0 {
            let memory = e.call(EFFECT_ALLOCATE, &args![0x6cu32]).u32();
            let effect = if memory != 0 {
                let rate = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
                let form = e.vcall(p, 0x574, &[]).u32();
                e.call(SHADER_EFFECT_CONSTRUCT, &args![memory, actor, form, rate])
                    .u32()
            } else {
                0
            };
            fn_00929260(e, effect, 1);
            if e.mem.u8(p + 0x17f) != 0 {
                let timer = x87_quiet(e.global::<f32>(SHADER_EFFECT_TIMER));
                fn_00929240(e, effect, timer);
            }
            if e.vcall(effect, 0xc4, &[]).u8() != 0 {
                e.call(PROCESS_LISTS_ADD_TEMP_EFFECT, &args![PROCESS_LISTS, effect]);
                if e.mem.u8(p + 0x17f) == 0 {
                    started_untimed = true;
                }
            }
        }
        let player: u32 = e.global(PLAYER);
        if actor == player && e.call(PLAYER_HAS_SHADER_SLOT, &args![player]).u32() != 0 {
            let wanted = e.vcall(p, 0x574, &[]).u32();
            let mut current = e.call(PLAYER_SHADER_EFFECT, &args![player]).u32();
            if wanted == 0 {
                if current != 0 {
                    delete_object(e, current);
                    current = 0;
                    fn_00929220(e, player, 0);
                }
            } else {
                if current != 0 && e.call(PACKAGE_WORD_30, &args![current]).u32() != wanted {
                    delete_object(e, current);
                    current = 0;
                }
                if current == 0 {
                    let memory = e.call(EFFECT_ALLOCATE, &args![0x6cu32]).u32();
                    current = if memory != 0 {
                        let rate = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
                        e.call(
                            SHADER_EFFECT_CONSTRUCT,
                            &args![memory, player, wanted, rate],
                        )
                        .u32()
                    } else {
                        0
                    };
                    fn_00929260(e, current, 1);
                    fn_00929220(e, player, current);
                    if !started_untimed {
                        let timer = x87_quiet(e.global::<f32>(SHADER_EFFECT_TIMER));
                        fn_00929240(e, current, timer);
                    }
                }
            }
            if current != 0 && e.vcall(current, 0xc4, &[]).u8() == 0 {
                delete_object(e, current);
                fn_00929220(e, player, 0);
            }
        }
    }
    e.mem.set_u8(p + 0x17c, 0);
    e.mem.set_u8(p + 0x17d, 0);
    e.mem.set_u8(p + 0x17e, 0);
    e.mem.set_u8(p + 0x17f, 0);
}

// Translated from 00929220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the word `value` at +0x6A4 of the object (the player's shader
/// effect slot, see [`middle_high_process_refresh_magic_shader_effects`]);
/// the engine map has no name. `(object, value)`.
pub fn fn_00929220(e: &mut Engine, this: u32, value: u32) {
    e.mem.set_u32(this + 0x6a4, value);
}

// Translated from 00929240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the float `value` at +0x34 of the object (the shader effect's
/// timer); the engine map has no name. `(object, value)`.
pub fn fn_00929240(e: &mut Engine, this: u32, value: f32) {
    e.mem.set_f32(this + 0x34, x87_quiet(value));
}

// Translated from 00929260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` at +0x28 of the object (the shader effect's
/// flag); the engine map has no name. `(object, value)`.
pub fn fn_00929260(e: &mut Engine, this: u32, value: u8) {
    e.mem.set_u8(this + 0x28, value);
}

/// `TESObjectREFR::GetFaceAnimationData` (Xbox PDB), `(actor, 0)`.
const REFERENCE_FACE_ANIMATION_DATA: u32 = 0x0057_6420;

// Translated from 00929280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetFaceAnimationData` (Xbox PDB): `(process,
/// actor)`. Answers 0 and clears the cached data (+0x178) when the actor
/// has no 3D model (slot +0x1D0); otherwise the cache, filled from
/// `TESObjectREFR::GetFaceAnimationData` when empty.
pub fn middle_high_process_get_face_animation_data(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) -> u32 {
    let p = this.addr();
    if e.vcall(actor, 0x1d0, &[]).u32() == 0 {
        e.mem.set_u32(p + 0x178, 0);
        return 0;
    }
    if e.mem.u32(p + 0x178) == 0 {
        let data = e
            .call(REFERENCE_FACE_ANIMATION_DATA, &args![actor, 0u32])
            .u32();
        e.mem.set_u32(p + 0x178, data);
    }
    e.mem.u32(p + 0x178)
}

/// `(light)`: the light's flag bits answered by `0040df90` after
/// `007df1d0` (neither has an engine map name), cdecl `(value)`.
const LIGHT_FLARE_VALUE: u32 = 0x007d_f1d0;
const LIGHT_FLARE_BITS: u32 = 0x0040_df90;

// Translated from 009292e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ShouldCheckFlare` (Xbox PDB): `(process, list,
/// mask)`. While the check flag (+0x180) is set, rebuilds the combined bits
/// (+0x184) as the or of `0040df90(007df1d0(item))` over the items of
/// `list` and clears the flag. Answers whether `mask` has a bit of the
/// combined bits ([`fn_00929390`]).
pub fn middle_high_process_should_check_flare(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    list: u32,
    mask: u32,
) -> bool {
    let p = this.addr();
    if e.mem.u8(p + 0x180) != 0 {
        e.mem.set_u32(p + 0x184, 0);
        let mut node = list;
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let slot = e.call(IDENTITY, &args![node]).u32();
            let item = e.mem.u32(slot);
            let value = e.call(LIGHT_FLARE_VALUE, &args![item]).u32();
            let bits = e.call(LIGHT_FLARE_BITS, &args![value]).u32();
            let combined = e.mem.u32(p + 0x184) | bits;
            e.mem.set_u32(p + 0x184, combined);
            node = e.call(WORD_4, &args![node]).u32();
        }
        e.mem.set_u8(p + 0x180, 0);
    }
    let combined = e.mem.u32(p + 0x184);
    fn_00929390(e, mask, combined)
}

// Translated from 00929390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the two words have a bit in common (cdecl `(a, b)`); the
/// engine map has no name.
pub fn fn_00929390(_e: &mut Engine, a: u32, b: u32) -> bool {
    a & b != 0
}

// Translated from 009293b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009293b0 - the exception frame (FS:[0]) is not translated
/// `MiddleHighProcess::AddAttachedArrow` (Xbox PDB): `(process, arrow)`.
/// Without an arrow nothing happens. Creates the list of attached arrows
/// (+0x1AC, an 8 byte list) when missing and adds the arrow unless the list
/// already holds it (`005f65d0`).
pub fn middle_high_process_add_attached_arrow(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    arrow: u32,
) {
    let p = this.addr();
    if arrow == 0 {
        return;
    }
    if e.mem.u32(p + 0x1ac) == 0 {
        let memory = e.call(ALLOCATE, &args![8u32]).u32();
        let list = if memory != 0 {
            e.call(LIST_CONSTRUCT, &args![memory]).u32()
        } else {
            0
        };
        e.mem.set_u32(p + 0x1ac, list);
    }
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), arrow);
        let list = e.mem.u32(p + 0x1ac);
        if !e.call(LIST_CONTAINS, &args![list, cell]).bool() {
            e.call(LIST_ADD, &args![list, cell]);
        }
    });
}

// Translated from 00929470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::RemoveAttachedArrow` (Xbox PDB): `(process,
/// arrow)`. Removes the arrow from the attached arrow list (+0x1AC) when
/// both exist.
pub fn middle_high_process_remove_attached_arrow(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    arrow: u32,
) {
    let p = this.addr();
    if arrow == 0 || e.mem.u32(p + 0x1ac) == 0 {
        return;
    }
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), arrow);
        let list = e.mem.u32(p + 0x1ac);
        e.call(LIST_REMOVE, &args![list, cell]);
    });
}

/// `(cell, reference)`: moves the reference into the cell (`00548740`, the
/// engine map has no name).
const CELL_MOVE_REFERENCE: u32 = 0x0054_8740;
/// `(process lists, reference)`: updates the lists for a moved reference
/// (`0096e870`, the engine map has no name).
const PROCESS_LISTS_REFERENCE_MOVED: u32 = 0x0096_e870;

// Translated from 009294b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::MoveAttachedArrowsToParentCell` (Xbox PDB):
/// `(process, parent)`. For every attached arrow (+0x1AC) that is not in
/// the parent reference's cell, moves it into that cell (`00548740`) and
/// tells the process lists (`0096e870`). Nothing happens without a parent,
/// without the list or with an empty one.
pub fn middle_high_process_move_attached_arrows_to_parent_cell(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    parent: u32,
) {
    let p = this.addr();
    if parent == 0 || e.mem.u32(p + 0x1ac) == 0 {
        return;
    }
    let head = e.mem.u32(p + 0x1ac);
    if e.call(LIST_IS_EMPTY, &args![head]).bool() {
        return;
    }
    let cell = e.call(REFERENCE_CELL, &args![parent]).u32();
    let mut node = e.mem.u32(p + 0x1ac);
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(IDENTITY, &args![node]).u32();
        let arrow = e.mem.u32(slot);
        if arrow != 0 && e.call(REFERENCE_CELL, &args![arrow]).u32() != cell && cell != 0 {
            e.call(CELL_MOVE_REFERENCE, &args![cell, arrow]);
            e.call(PROCESS_LISTS_REFERENCE_MOVED, &args![PROCESS_LISTS, arrow]);
        }
        node = e.call(WORD_4, &args![node]).u32();
    }
}

/// `(animation, value)`: sets the blend value of an aim sequence
/// (`00495480`, the engine map has no name).
const SEQUENCE_SET_VALUE: u32 = 0x0049_5480;

// Translated from 00929570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetAimSequence` (Xbox PDB): `(process, index,
/// sequence)`. Stores the sequence in the aim sequence slot `index`
/// (+0x1C4 + 4 * index) and, when it exists, sets its value (`00495480`) to
/// 1.0 for index 0 and 0.0 for the others.
pub fn middle_high_process_set_aim_sequence(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    index: u32,
    sequence: u32,
) {
    let p = this.addr();
    e.mem.set_u32(
        p.wrapping_add(index.wrapping_mul(4)).wrapping_add(0x1c4),
        sequence,
    );
    if sequence != 0 {
        let value = if index == 0 { 1.0f32 } else { 0.0f32 };
        e.call(SEQUENCE_SET_VALUE, &args![sequence, value.to_bits()]);
    }
}

/// The aim sequence speed settings (objects at these addresses).
const AIM_SETTING_A: u32 = 0x011e_0474;
const AIM_SETTING_B: u32 = 0x011c_5724;
const AIM_SETTING_LEFT: u32 = 0x011e_048c;
const AIM_SETTING_RIGHT: u32 = 0x011e_0498;
/// `MobileObject::GetLooking` (Xbox PDB), `(actor)`, answered in ST0.
const MOBILE_GET_LOOKING: u32 = 0x0093_1d70;
/// cdecl `(a, b)` answering in ST0 (`0040ebd0`, the engine map has no name;
/// called with (1.0, value) here).
const MIN_OF_FLOATS: u32 = 0x0040_ebd0;
/// cdecl `(a, b)` answering in ST0 (`00404010`, the engine map has no name).
const MAX_OF_FLOATS: u32 = 0x0040_4010;
/// The float -1e-6 widened to double (exe double at `0x01089b18`, bytes
/// 0xbeb0c6f7a0000000, not the exact double -1e-6).
const MINUS_EPSILON: f64 = -1.0e-6_f32 as f64;
/// The float 1e-6 widened to double (exe double at `0x01017cf8`, bytes
/// 0x3eb0c6f7a0000000, not the exact double 1e-6).
const EPSILON: f64 = 1.0e-6_f32 as f64;
/// The double 0.017453292 (exe double at `0x01023128`), degrees to
/// radians.
const DEGREES_TO_RADIANS: f64 = 0.017453292_f32 as f64;

/// The float a setting object answers (`00403e20` then the load).
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let address = e.call(FLOAT_SETTING_VALUE, &args![setting]).u32();
    e.mem.f32(address)
}

// Translated from 009295c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::UpdateAimSequences` (Xbox PDB): `(process, actor)`.
/// Moves the aim angle (+0x1D0) towards zero by the frame time times the two
/// speed settings, snapping it to 0 when it crosses zero or ends strictly
/// within the float 1e-6 (widened to double) of it. The blend of the left
/// (+0x1C8) and right (+0x1CC) aim sequences is derived from the actor's
/// looking angle plus the aim angle
/// (divided by the side's setting times a degree in radians, limited with
/// `0040ebd0` to 1.0) and the base sequence (+0x1C4) gets `1 -` the
/// combined weight (`00404010`); each existing sequence's value is set with
/// `00495480`.
pub fn middle_high_process_update_aim_sequences(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    let p = this.addr();
    let mut left = 0.0f32;
    let mut right = 0.0f32;
    let frame_time = e.call(FLOAT_C, &args![GLOBAL_TIME_DATA]).f64();
    let angle = e.mem.f32(p + 0x1d0);
    let scaled = frame_time * f64::from(angle);
    let first = f64::from(setting_float(e, AIM_SETTING_A)) * scaled;
    let second = f64::from(setting_float(e, AIM_SETTING_B)) * first;
    let next = (f64::from(e.mem.f32(p + 0x1d0)) - second) as f32;
    let angle = e.mem.f32(p + 0x1d0);
    // The exe has three separate branches that each store 0.0 (00929650,
    // 00929683, 009296c0); kept as written.
    #[allow(clippy::if_same_then_else)]
    if angle > 0.0 && next < 0.0 {
        e.mem.set_f32(p + 0x1d0, 0.0);
    } else if angle < 0.0 && next > 0.0 {
        e.mem.set_f32(p + 0x1d0, 0.0);
    } else if next != 0.0 && f64::from(next) > MINUS_EPSILON && f64::from(next) < EPSILON {
        // 009296a3/009296b3: both compares are strict against the widened
        // float, so a `next` of exactly 0x358637bd (or 0xb58637bd) is kept.
        e.mem.set_f32(p + 0x1d0, 0.0);
    } else {
        e.mem.set_f32(p + 0x1d0, x87_quiet(next));
    }
    let looking = e.call(MOBILE_GET_LOOKING, &args![actor]).f64();
    let angle = e.mem.f32(p + 0x1d0);
    let total = (looking + f64::from(angle)) as f32;
    if total < 0.0 && e.mem.u32(p + 0x1c8) != 0 {
        let setting = setting_float(e, AIM_SETTING_LEFT);
        let divisor = f64::from(setting) * -DEGREES_TO_RADIANS;
        let ratio = (f64::from(total) / divisor) as f32;
        let ratio_bits = x87_quiet_bits(ratio.to_bits());
        left = e.call(MIN_OF_FLOATS, &args![1.0f32, ratio_bits]).f64() as f32;
    } else if total > 0.0 && e.mem.u32(p + 0x1cc) != 0 {
        let setting = setting_float(e, AIM_SETTING_RIGHT);
        let divisor = f64::from(setting) * DEGREES_TO_RADIANS;
        let ratio = (f64::from(total) / divisor) as f32;
        let ratio_bits = x87_quiet_bits(ratio.to_bits());
        right = e.call(MIN_OF_FLOATS, &args![1.0f32, ratio_bits]).f64() as f32;
    }
    let weight = (f64::from(left) + f64::from(right)) as f32;
    let combined = e
        .call(
            MAX_OF_FLOATS,
            &args![x87_quiet_bits(weight.to_bits()), 0.0f32],
        )
        .f64();
    let base = (1.0 - combined) as f32;
    let sequence = e.mem.u32(p + 0x1c4);
    if sequence != 0 {
        e.call(
            SEQUENCE_SET_VALUE,
            &args![sequence, x87_quiet_bits(base.to_bits())],
        );
    }
    let sequence = e.mem.u32(p + 0x1c8);
    if sequence != 0 {
        e.call(
            SEQUENCE_SET_VALUE,
            &args![sequence, x87_quiet_bits(left.to_bits())],
        );
    }
    let sequence = e.mem.u32(p + 0x1cc);
    if sequence != 0 {
        e.call(
            SEQUENCE_SET_VALUE,
            &args![sequence, x87_quiet_bits(right.to_bits())],
        );
    }
}

/// `(process)`: whether the process's package check says it is idle
/// (`008e7c00`, the engine map has no name).
const PROCESS_BLOCKS_IDLE: u32 = 0x008e_7c00;
/// `(extra)`: the package type an extra answers (`009f4640`).
const EXTRA_PACKAGE_TYPE: u32 = 0x009f_4640;

// Translated from 00929820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor is ready to play an idle for the process's package
/// (the engine map has no name): `(process, actor)`. False when
/// `008e7c00` says so; otherwise looks up the package's entry (slot +0x27C
/// package, slot +0x280 index in the table of its type, replaced for 0x2A
/// by `009f4640` of slot +0x274) and, for the 14 listed entries (0, 1, 4,
/// 5, 6, 7, 0x1B, 0x1C, 0x1E, 0x22, 0x2C, 0x2F, 0x31, 0x36), answers the
/// package's slot +0x13C with (actor, 0, -1.0, 0) when its type is 0 and
/// whether the actor's path is complete (`008b3bb0`) otherwise.
pub fn fn_00929820(e: &mut Engine, this: Ptr<MiddleHighProcess>, actor: u32) -> bool {
    let p = this.addr();
    if e.call(PROCESS_BLOCKS_IDLE, &args![p]).bool() {
        return false;
    }
    let package = e.vcall(p, 0x27c, &[]).u32();
    let mut ready = false;
    let package_type = e.call(PACKAGE_WORD_18, &args![package]).u32();
    let index = e.vcall(p, 0x280, &[]).u32();
    let table = e.mem.u32(PROCEDURE_TABLES + package_type.wrapping_mul(4));
    let mut entry = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    if entry == 0x2a {
        let extra = e.vcall(p, 0x274, &[]).u32();
        if extra != 0 {
            entry = e.call(EXTRA_PACKAGE_TYPE, &args![extra]).u32();
        }
    }
    if matches!(
        entry,
        0 | 1 | 4 | 5 | 6 | 7 | 0x1b | 0x1c | 0x1e | 0x22 | 0x2c | 0x2f | 0x31 | 0x36
    ) {
        if e.call(PACKAGE_WORD_18, &args![package]).u32() == 0 {
            let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
            ready = e
                .vcall(package, 0x13c, &args![actor, 0u32, minus_one, 0u32])
                .bool();
        } else if e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
            ready = true;
        }
    }
    ready
}

/// `(list, pointer to item)`: whether the list holds the item (`005f65d0`,
/// the engine map has no name).
const LIST_CONTAINS: u32 = 0x005f_65d0;

/// `(package)`: the idle collection of the package (`005f36f0`, the engine
/// map names it `ActorMover::GetPreferredMoveMode`, which does not fit
/// the use here).
const PACKAGE_IDLE_COLLECTION: u32 = 0x005f_36f0;
/// `Animation::SpecialIdleDonePlaying` (Xbox PDB), `(animation)`.
const SPECIAL_IDLE_DONE_PLAYING: u32 = 0x0049_85f0;
/// `(collection)`: the play mode of the idle collection (`00479e80`), 0 for
/// random and 1 for sequential in the use here.
const IDLE_COLLECTION_MODE: u32 = 0x0047_9e80;
/// `BGSIdleCollection::GetIdlesForActor` (Xbox PDB), `(collection, actor,
/// target, list)`: fills the list (a 10 entry array object) and answers
/// the idle count.
const GET_IDLES_FOR_ACTOR: u32 = 0x0047_a030;
/// Constructor `(array, capacity)` and destructor `(array)` of the idle
/// array object (16 bytes).
const IDLE_ARRAY_CONSTRUCT: u32 = 0x0090_58e0;
const IDLE_ARRAY_DESTRUCT: u32 = 0x0090_5910;
/// `(array, index)`: the address of the array's element.
const IDLE_ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// `(package)`: the idle the package last played (`00546930`, the engine
/// map has no name).
const PACKAGE_LAST_IDLE: u32 = 0x0054_6930;
/// The CRT `rand`.
const RAND: u32 = 0x00ec_adb8;
/// The float 0.8 (exe float at `0x01030ff0`): the idle timer for a random
/// idle.
const RANDOM_IDLE_TIMER: u32 = 0x0103_0ff0;

/// Picks the idle `index` of the array as the process's idle (+0x194) and
/// remembers the index (+0x190).
fn select_idle(e: &mut Engine, p: u32, array: u32, index: u32) {
    let address = e.call(IDLE_ARRAY_ELEMENT, &args![array, index]).u32();
    let idle = e.mem.u32(address);
    e.mem.set_u32(p + 0x194, idle);
    e.mem.set_u32(p + 0x190, index);
}

// Translated from 00929960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00929960 - the exception frame (FS:[0]) is not translated
/// Plays the next idle of the package's idle collection on the actor (the
/// engine map has no name): `(process, actor, force)`. The package is slot
/// +0x27C (a type 0x1C package comes from the actor's extra data
/// instead). Nothing happens without a package. With the idle flag (+0x110)
/// set `force` becomes true and the timer (+0x198) is cleared. The idle is
/// only changed when `fn_00929820` is true, the package has an idle
/// collection, the actor an animation (slot +0x1E4) whose special idle is
/// done (or `force`), and the actor's slot +0x234 is false. While the timer
/// is positive it is reduced by the frame time. Otherwise the idles of the
/// collection for the actor and the target (+0x40) are listed and, by the
/// collection's mode (`00479e80`), one is chosen at random (not the one
/// played before when there is a choice; timer 0.8) or the next in order
/// (timer -1.0), cleared when it is the package's last idle (`00546930`);
/// the idle is played through the process's slot +0x44 with (actor, idle,
/// 2, 1, 0, 1), and `0090ac20` runs when the collection (`fn_00929d20`) asks
/// for it at the end of the list.
pub fn fn_00929960(e: &mut Engine, this: Ptr<MiddleHighProcess>, actor: u32, force: u8) {
    let p = this.addr();
    let mut force = force != 0;
    let mut package = e.vcall(p, 0x27c, &[]).u32();
    if package != 0 && e.call(PACKAGE_TYPE, &args![package]).u32() == 0x1c {
        let extra = e.call(REFERENCE_EXTRA_LIST, &args![actor]).u32();
        package = e.call(GET_PACKAGE_EXTRA, &args![extra]).u32();
    }
    let animation = e.vcall(actor, 0x1e4, &[]).u32();
    if package == 0 {
        return;
    }
    if e.mem.u8(p + 0x110) != 0 {
        force = true;
        e.mem.set_f32(p + 0x198, 0.0);
    }
    if !fn_00929820(e, this, actor) {
        return;
    }
    let collection = e.call(PACKAGE_IDLE_COLLECTION, &args![package]).u32();
    if collection == 0 || animation == 0 {
        return;
    }
    if !e.call(SPECIAL_IDLE_DONE_PLAYING, &args![animation]).bool() && !force {
        return;
    }
    if e.vcall(actor, 0x234, &[]).bool() {
        return;
    }
    let timer = e.mem.f32(p + 0x198);
    if timer > 0.0 || timer.is_nan() {
        let frame_time = e.call(FLOAT_C, &args![GLOBAL_TIME_DATA]).f64();
        let timer = e.mem.f32(p + 0x198);
        e.mem
            .set_f32(p + 0x198, (f64::from(timer) - frame_time) as f32);
        return;
    }
    e.mem.set_u8(p + 0x110, 0);
    let previous = e.mem.u32(p + 0x194);
    e.mem.set_u32(p + 0x194, 0);
    let mode = e.call(IDLE_COLLECTION_MODE, &args![collection]).u32();
    e.with_stack(0x10, |e, array| {
        let array = array.addr();
        e.call(IDLE_ARRAY_CONSTRUCT, &args![array, 0xau32]);
        let target = e.mem.u32(p + 0x40);
        let count = e
            .call(
                GET_IDLES_FOR_ACTOR,
                &args![collection, actor, target, array],
            )
            .u32();
        if count == 0 {
            e.call(IDLE_ARRAY_DESTRUCT, &args![array]);
            return;
        }
        if mode == 1 {
            let index = e.mem.u32(p + 0x190);
            if index == u32::MAX || index.wrapping_add(1) >= count {
                e.mem.set_u32(p + 0x190, 0);
            } else {
                e.mem.set_u32(p + 0x190, index + 1);
            }
            let index = e.mem.u32(p + 0x190);
            select_idle(e, p, array, index);
            let last = e.call(PACKAGE_LAST_IDLE, &args![package]).u32();
            if last == e.mem.u32(p + 0x194) {
                e.mem.set_u32(p + 0x194, 0);
            }
            if e.mem.u32(p + 0x190).wrapping_add(1) >= count && fn_00929d20(e, collection) {
                e.call(SET_PROCESS_FLAG_40, &args![p, 1u32]);
            }
            let minus_one = x87_quiet(e.global::<f32>(MINUS_ONE));
            e.mem.set_f32(p + 0x198, minus_one);
        } else if mode == 0 {
            e.mem.set_u32(p + 0x194, previous);
            if previous == 0 {
                let index = e.call(RAND, &[]).u32() % count;
                select_idle(e, p, array, index);
            } else {
                while previous == e.mem.u32(p + 0x194) && count > 1 {
                    let index = e.call(RAND, &[]).u32() % count;
                    select_idle(e, p, array, index);
                }
            }
            let last = e.call(PACKAGE_LAST_IDLE, &args![package]).u32();
            if last == e.mem.u32(p + 0x194) {
                e.mem.set_u32(p + 0x194, 0);
            }
            if e.mem.u32(p + 0x194) != 0 {
                let timer = x87_quiet(e.global::<f32>(RANDOM_IDLE_TIMER));
                e.mem.set_f32(p + 0x198, timer);
                if fn_00929d20(e, collection) {
                    e.call(SET_PROCESS_FLAG_40, &args![p, 1u32]);
                }
            }
        }
        let idle = e.mem.u32(p + 0x194);
        if idle != 0 {
            e.vcall(p, 0x44, &args![actor, idle, 2u32, 1u32, 0u32, 1u32]);
        }
        e.call(IDLE_ARRAY_DESTRUCT, &args![array]);
    });
}

// Translated from 00929d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 2 of the byte at +4 of the idle collection is set (the
/// engine map has no name).
pub fn fn_00929d20(e: &mut Engine, this: u32) -> bool {
    e.mem.u8(this + 4) & 4 != 0
}

/// `(form)`: whether the weapon form fires automatically (`00524b40`, the
/// engine map has no name; the use here is the automatic shot count).
const FORM_IS_AUTOMATIC: u32 = 0x0052_4b40;
/// `(form)`: the form's base value answered in ST0 (`00821640`, the engine
/// map has no name).
const FORM_BASE_RATE: u32 = 0x0082_1640;
/// `TESObjectWEAP::GetAttackSpeed` (Xbox PDB), `(weapon, flag)`, answered in
/// ST0.
const WEAPON_ATTACK_SPEED: u32 = 0x0064_6020;
/// `(actor)`: a rate scale answered in ST0 (`004e3d00`, the engine map has
/// no name).
const ACTOR_RATE_SCALE: u32 = 0x004e_3d00;
/// `VATS::GetPlayerUpdateMult` (Xbox PDB), on the VATS object.
const VATS_PLAYER_UPDATE_MULT: u32 = 0x009c_8cc0;
/// `VATS::GetTargetUpdateMult` (Xbox PDB), on the VATS object.
const VATS_TARGET_UPDATE_MULT: u32 = 0x009c_8d60;
/// The VATS object.
const VATS: u32 = 0x011f_2250;
/// The VATS target reference (word at this address).
const VATS_TARGET: u32 = 0x011f_21cc;

/// The attack rate of the process's weapon for the shot counters:
/// `1 / (GetAttackSpeed(form, HasModEffectActive(item, 8)) * base rate)`.
fn automatic_shot_interval(e: &mut Engine, p: u32, form: u32) -> f32 {
    let base = e.call(FORM_BASE_RATE, &args![form]).f64();
    let item = e.mem.u32(p + 0x114);
    let mod_active = e
        .call(ITEM_CHANGE_MOD_EFFECT_ACTIVE, &args![item, 8u32])
        .u8();
    let speed = e
        .call(WEAPON_ATTACK_SPEED, &args![form, mod_active as u32])
        .f64();
    (speed * base) as f32
}

// Translated from 00929d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ClearAutomaticShotsToFire` (Xbox PDB): `(process,
/// clear)`. The shot counter (+0x1D4) is set to 0.0 without a weapon item
/// (+0x114) and list (+0x118), or when the item's form is not automatic or
/// `clear` is false; otherwise it is set to `1 / (attack speed times the
/// base rate)` (the time between automatic shots).
pub fn middle_high_process_clear_automatic_shots_to_fire(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    clear: u8,
) {
    let p = this.addr();
    if e.mem.u32(p + 0x114) == 0 || e.mem.u32(p + 0x118) == 0 {
        e.mem.set_f32(p + 0x1d4, 0.0);
        return;
    }
    let item = e.mem.u32(p + 0x114);
    let form = e.call(ITEM_CHANGE_FORM, &args![item]).u32();
    if e.call(FORM_IS_AUTOMATIC, &args![form]).bool() && clear != 0 {
        let interval = automatic_shot_interval(e, p, form);
        e.mem.set_f32(p + 0x1d4, (1.0 / f64::from(interval)) as f32);
    } else {
        e.mem.set_f32(p + 0x1d4, 0.0);
    }
}

// Translated from 00929df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetAutomaticShotsToFire` (Xbox PDB): `(process,
/// actor)`. 0 without a weapon item (+0x114) or when its form is not
/// automatic. Otherwise the shot counter (+0x1D4) is reduced by the frame
/// time times the setting and the process's rate scale (slot +0x1B8, 1.0
/// without one), times the VATS update multiplier when the actor is the
/// player or the VATS target (the player's `GetPlayerUpdateMult`, the
/// target's `GetTargetUpdateMult`), and the number of intervals
/// (`1 / (attack speed times base rate)`) needed to bring it to zero or
/// above is added back and answered.
pub fn middle_high_process_get_automatic_shots_to_fire(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) -> u8 {
    let p = this.addr();
    if e.mem.u32(p + 0x114) == 0 {
        return 0;
    }
    let item = e.mem.u32(p + 0x114);
    let form = e.call(ITEM_CHANGE_FORM, &args![item]).u32();
    if !e.call(FORM_IS_AUTOMATIC, &args![form]).bool() {
        return 0;
    }
    let frame_time = e.call(FLOAT_C, &args![GLOBAL_TIME_DATA]).f64();
    let setting = f64::from(setting_float(e, AIM_SETTING_B));
    let scaled = (setting * frame_time) as f32;
    let scale_source = e.vcall(p, 0x1b8, &[]).u32();
    let scale_value = if scale_source != 0 {
        let source = e.vcall(p, 0x1b8, &[]).u32();
        e.call(ACTOR_RATE_SCALE, &args![source]).f64() as f32
    } else {
        1.0
    };
    let scale = x87_quiet(scale_value);
    let counter = f64::from(e.mem.f32(p + 0x1d4));
    let player: u32 = e.global(PLAYER);
    let target: u32 = e.global(VATS_TARGET);
    let reduction = if actor == player {
        let mult = e.call(VATS_PLAYER_UPDATE_MULT, &args![VATS]).f64();
        mult * f64::from(scaled) * f64::from(scale)
    } else if actor == target {
        let frame_time = e.call(FLOAT_C, &args![GLOBAL_TIME_DATA]).f64();
        let setting = f64::from(setting_float(e, AIM_SETTING_B));
        let step = setting * frame_time * f64::from(scale);
        let mult = e.call(VATS_TARGET_UPDATE_MULT, &args![VATS]).f64();
        mult * step
    } else {
        let frame_time = e.call(FLOAT_C, &args![GLOBAL_TIME_DATA]).f64();
        let setting = f64::from(setting_float(e, AIM_SETTING_B));
        setting * frame_time * f64::from(scale)
    };
    e.mem.set_f32(p + 0x1d4, (counter - reduction) as f32);
    let interval = automatic_shot_interval(e, p, form);
    let mut count = 0u8;
    let step = (1.0 / f64::from(interval)) as f32;
    while e.mem.f32(p + 0x1d4) < 0.0 {
        let counter = f64::from(e.mem.f32(p + 0x1d4));
        e.mem.set_f32(p + 0x1d4, (counter + f64::from(step)) as f32);
        count = count.wrapping_add(1);
    }
    count
}

/// `(sandbox data, actor, radius)`: whether the actor is at its sandbox
/// target (`009f56b0`, the engine map has no name).
const SANDBOX_AT_TARGET: u32 = 0x009f_56b0;
/// Sandbox data accessors (`data + 0x10`, `data + 0x3C`, `data + 8`;
/// the engine map names `0044edb0` `BaseProcess::GetCurrentProcedureIndex`
/// which does not describe the use here).
const SANDBOX_WORD_10: u32 = 0x0044_edb0;
const SANDBOX_WORD_3C: u32 = 0x0063_9b40;
/// `Actor::IsPathing` (Xbox PDB), `(actor)`.
const ACTOR_IS_PATHING: u32 = 0x008b_3bd0;
/// 150.0 (exe float at `0x010231d0`): the sandbox distance slack.
const SANDBOX_SLACK: u32 = 0x0102_31d0;
/// `SandBoxActorPackageData::ForgetTargetAndPickProcedure` (Xbox PDB),
/// `(data, actor, package)`.
const SANDBOX_FORGET_TARGET: u32 = 0x009f_4680;
/// `(actor path handle word)`: the path handle's pointer (`006838b0`), and
/// the constructor `(handle, pointer)`, accessor `(handle)` and destructor
/// `(handle)` of the 8 byte handle object (`00633c90`, `00559450`,
/// `0045cec0`); none has an engine map name.
const PATH_HANDLE_POINTER: u32 = 0x0068_38b0;
/// `(path object)`: the path's location (`006a9540`), its worldspace
/// (`PathingLocation::GetWorldspace`, `00441110`) and cell
/// (`PathingLocation::GetCell`, `006dd4f0`).
const PATH_LOCATION: u32 = 0x006a_9540;
const PATH_LOCATION_WORLDSPACE: u32 = 0x0044_1110;
const PATH_LOCATION_CELL: u32 = 0x006d_d4f0;
/// `(location, out)`: the position of the path location (`005c3420`).
const PATH_LOCATION_POSITION: u32 = 0x005c_3420;
/// `(vector)`: the vector's length value (ST0; `004a7290`).
const VECTOR_LENGTH_VALUE: u32 = 0x004a_7290;
/// `()`: a float in ST0 (`006e25d0`, the engine map has no name).
const PATHING_SPEED_LIMIT: u32 = 0x006e_25d0;
/// `PathingRequest::PathingRequest` (Xbox PDB), `(request)`, and its
/// destructor (`006e2620`).
const PATHING_REQUEST_CONSTRUCT: u32 = 0x006e_2420;
const PATHING_REQUEST_RELEASE: u32 = 0x006e_2620;
/// `(request, 0)` (`008e7c50`, the engine map has no name).
const PATHING_REQUEST_SET_FLAG: u32 = 0x008e_7c50;
/// `Actor::BuildRequest` (Xbox PDB), `(actor, request, position, cell,
/// worldspace, speed, 0)`.
const ACTOR_BUILD_REQUEST: u32 = 0x008b_3880;
/// `Actor::SetPathfindingGoal` (Xbox PDB), `(actor, request)`.
const ACTOR_SET_GOAL_FROM_REQUEST: u32 = 0x008b_3630;
/// `(data, actor, package)` and `(data, actor, x)` sandbox state changes
/// (`009f4700`, `009f3af0`; the engine map names the first nothing).
const SANDBOX_PICK_PROCEDURE: u32 = 0x009f_4700;
const SANDBOX_START: u32 = 0x009f_3af0;
/// `(data, actor, 1)` (`009f4300`, the engine map has no name): whether the
/// actor reached the sandbox location.
const SANDBOX_REACHED: u32 = 0x009f_4300;
/// `SandBoxActorPackageData::ShouldPickNewProcedure` (Xbox PDB), `(data,
/// actor)`.
const SANDBOX_SHOULD_PICK: u32 = 0x009f_43c0;
/// `SandBoxActorPackageData::MoveToNextLocation` (Xbox PDB), `(data)`.
const SANDBOX_MOVE_TO_NEXT: u32 = 0x009f_3c10;
/// `Actor::ClearLookAtTarget` (Xbox PDB), `(actor)`.
const ACTOR_CLEAR_LOOK_AT_TARGET: u32 = 0x008b_3d30;
/// `(object)`: whether the sandbox target object is finished (`009de420`).
const SANDBOX_TARGET_DONE: u32 = 0x009d_e420;

/// Builds a pathing request for the actor towards the reference (or, for
/// none, the package's location) and makes it the actor's goal; the
/// sandbox state (+4) becomes 1. `package` is slot +0x22C's package, used
/// for the location when there is no reference.
fn sandbox_start_path(e: &mut Engine, p: u32, actor: u32, data: u32, reference: u32, package: u32) {
    let location_package = e.vcall(p, 0x27c, &[]).u32();
    let radius = e
        .call(PACKAGE_RADIUS, &args![location_package, actor])
        .u32() as f32;
    let limit = e.call(PATHING_SPEED_LIMIT, &[]).f64() as f32;
    let speed = e
        .call(MAX_OF_FLOATS, &args![radius.to_bits(), limit.to_bits()])
        .f64() as f32;
    e.with_stack(0xc0, |e, area| {
        let request = area.addr();
        let coordinate = request + 0xb0;
        e.call(PATHING_REQUEST_CONSTRUCT, &args![request]);
        if reference != 0 {
            let worldspace = e.call(REFERENCE_GET_WORLD_SPACE, &args![reference]).u32();
            let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
            let position = e.vcall(reference, 0x1f4, &[]).u32();
            e.call(
                ACTOR_BUILD_REQUEST,
                &args![
                    actor,
                    request,
                    position,
                    cell,
                    worldspace,
                    speed.to_bits(),
                    0u32
                ],
            );
        } else {
            let worldspace = e
                .call(PACKAGE_GET_LOCATION_WORLD, &args![package, actor])
                .u32();
            let cell = e
                .call(PACKAGE_GET_LOCATION_CELL, &args![package, actor])
                .u32();
            let position = e
                .call(
                    PACKAGE_GET_LOCATION_COORD,
                    &args![package, coordinate, actor],
                )
                .u32();
            e.call(
                ACTOR_BUILD_REQUEST,
                &args![
                    actor,
                    request,
                    position,
                    cell,
                    worldspace,
                    speed.to_bits(),
                    0u32
                ],
            );
        }
        e.call(PATHING_REQUEST_SET_FLAG, &args![request, 0u32]);
        e.call(ACTOR_SET_GOAL_FROM_REQUEST, &args![actor, request]);
        e.mem.set_u32(data + 4, 1);
        e.call(PATHING_REQUEST_RELEASE, &args![request]);
    });
}

// Translated from 00929fc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00929fc0 - the exception frame (FS:[0]) is not translated
/// `MiddleHighProcess::ProcessSandBox` (Xbox PDB): `(process, actor)`.
/// Without sandbox data (`008d0430`) slot +0x288 runs with (actor, 1).
/// Otherwise decides whether a new path is needed: with a move target
/// (`0044edb0`) and an anchor (`00639b40`) when the target is farther than
/// the package radius (`00676280`, as a float) plus 150.0 from the anchor
/// (the sandbox target is forgotten, `009f4680`); else, when the actor is
/// not pathing, when it is not at its target (`009f56b0`); else when it is
/// pathing to a target in the same place as its path's last point and the
/// target is farther than that radius plus 150.0 from the path location.
/// A new path (`PathingRequest` through `Actor::BuildRequest` to the target
/// or the package's location, `Actor::SetPathfindingGoal`) ends the call
/// with state 1. Otherwise the state machine on the sandbox data's state
/// (+4): 0 starts (`009f3af0`) and continues as 1; 1 moves the actor
/// (slot +0x418 in furniture states 4 and 9, `009f4300` to state 2); 2
/// moves to state 3 by the data's type (`0044ddc0`, the actor's slot +0x214
/// furniture state) and continues as 3; 3 picks a new procedure when
/// `ShouldPickNewProcedure` says so (a new path to the next location, the
/// look-at target cleared, state 1) and runs the type's action (slots
/// +0x844, +0x7BC, +0x84C, +0x868, +0x850) or forgets the target, then
/// slot +0x350 runs with (actor, 0x101, 1) while the actor is pathing.
pub fn middle_high_process_process_sand_box(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    let p = this.addr();
    let data = e.call(PROCESS_PACKAGE, &args![p]).u32();
    if data == 0 {
        e.vcall(p, 0x288, &args![actor, 1u32]);
        return;
    }
    let package = e.vcall(p, 0x22c, &[]).u32();
    let radius = e.call(PACKAGE_RADIUS, &args![package, actor]).u32() as f32;
    let at_target = e
        .call(SANDBOX_AT_TARGET, &args![data, actor, radius.to_bits()])
        .u8();
    let reference = e.call(SANDBOX_WORD_10, &args![data]).u32();
    let slack = x87_quiet(e.global::<f32>(SANDBOX_SLACK));
    let mut needs_path = false;
    if reference != 0 && e.call(SANDBOX_WORD_3C, &args![data]).u32() != 0 {
        let anchor = e.call(SANDBOX_WORD_3C, &args![data]).u32();
        let distance = e
            .call(
                REFERENCE_GET_DISTANCE_FROM_REFERENCE,
                &args![reference, anchor, 0u32, 0u32],
            )
            .f64();
        needs_path = f64::from(radius) + f64::from(slack) < distance;
        if needs_path {
            e.call(SANDBOX_FORGET_TARGET, &args![data, actor, package]);
        }
    } else if !e.call(ACTOR_IS_PATHING, &args![actor]).bool() {
        needs_path = at_target == 0;
    } else if reference != 0 {
        let path_word = e.mem.u32(actor + 0x190);
        let handle_pointer = e.call(PATH_HANDLE_POINTER, &args![path_word]).u32();
        e.with_stack(0x40, |e, area| {
            let handle = area.addr();
            let vector = handle + 0x10;
            let scratch = handle + 0x20;
            e.call(POINTER_INIT, &args![handle, handle_pointer]);
            if e.call(FIRST_WORD, &args![handle]).u32() != 0 {
                let path = e.call(FIRST_WORD, &args![handle]).u32();
                let location = e.call(PATH_LOCATION, &args![path]).u32();
                let path_worldspace = e.call(PATH_LOCATION_WORLDSPACE, &args![location]).u32();
                let mut path_cell = e.call(PATH_LOCATION_CELL, &args![location]).u32();
                if path_cell != 0 && !e.call(CELL_FLAG, &args![path_cell]).bool() {
                    path_cell = 0;
                }
                let worldspace = e.call(REFERENCE_GET_WORLD_SPACE, &args![reference]).u32();
                let mut cell = e.call(REFERENCE_CELL, &args![reference]).u32();
                if cell != 0 && !e.call(CELL_FLAG, &args![cell]).bool() {
                    cell = 0;
                }
                let same_place = if cell != 0 || path_cell != 0 {
                    cell == path_cell
                } else {
                    worldspace == path_worldspace
                };
                let limit = (f64::from(radius) + f64::from(slack)) as f32;
                if same_place && radius >= 0.0 {
                    let position = e
                        .call(PATH_LOCATION_POSITION, &args![location, scratch])
                        .u32();
                    let from = e.vcall(reference, 0x1f4, &[]).u32();
                    e.call(POINT_SUBTRACT, &args![from, vector, position]);
                    let length = e.call(VECTOR_LENGTH_VALUE, &args![vector]).f64();
                    needs_path = f64::from(limit) * f64::from(limit) < length;
                }
            }
            e.call(POINTER_RELEASE, &args![handle]);
        });
    }
    if needs_path {
        sandbox_start_path(e, p, actor, data, reference, package);
        return;
    }
    let state = e.mem.u32(data + 4);
    if state > 3 {
        return;
    }
    if state <= 1 {
        if state == 0 {
            e.mem.set_u32(data + 4, 1);
            let word = e.call(PACKAGE_LOCATION, &args![package]).u32();
            e.call(SANDBOX_START, &args![data, actor, word]);
        }
        let furniture_state = e.vcall(actor, 0x214, &[]).u32();
        if furniture_state == 9 || e.vcall(actor, 0x214, &[]).u32() == 4 {
            e.vcall(actor, 0x418, &[]);
        } else if e.call(SANDBOX_REACHED, &args![data, actor, 1u32]).bool() {
            e.mem.set_u32(data + 4, 2);
        }
        return;
    }
    if state == 2 {
        match e.call(ITEM_CHANGE_FORM, &args![data]).u32() {
            0 | 2 => {
                if (e.vcall(actor, 0x214, &[]).u32() as i32) >= 1
                    && (e.vcall(actor, 0x214, &[]).u32() as i32) <= 4
                {
                    e.mem.set_u32(data + 4, 3);
                }
            }
            1 => {
                if (e.vcall(actor, 0x214, &[]).u32() as i32) >= 6
                    && (e.vcall(actor, 0x214, &[]).u32() as i32) <= 9
                {
                    e.mem.set_u32(data + 4, 3);
                }
            }
            3 => e.mem.set_u32(data + 4, 3),
            4 if !e.call(ACTOR_IS_PATHING, &args![actor]).bool() && e.mem.u32(p + 0x194) != 0 => {
                e.mem.set_u32(data + 4, 3);
            }
            _ => {}
        }
    }
    if e.call(SANDBOX_SHOULD_PICK, &args![data, actor]).bool() {
        if e.call(SANDBOX_MOVE_TO_NEXT, &args![data]).bool() {
            let next = e.call(SANDBOX_WORD_10, &args![data]).u32();
            if next != 0 {
                sandbox_start_path(e, p, actor, data, next, package);
            }
        }
        e.vcall(p, 0x244, &[]);
        e.call(ACTOR_CLEAR_LOOK_AT_TARGET, &args![actor]);
        let package = e.vcall(p, 0x22c, &[]).u32();
        e.call(SANDBOX_PICK_PROCEDURE, &args![data, actor, package]);
        e.mem.set_u32(data + 4, 1);
    }
    let anchor = e.call(SANDBOX_WORD_3C, &args![data]).u32();
    let mut acts = true;
    if anchor == 0 {
        let kind = e.call(ITEM_CHANGE_FORM, &args![data]).u32();
        if fn_0092a980(e, kind) {
            acts = false;
        }
    }
    let mut result = acts;
    if acts {
        match e.call(ITEM_CHANGE_FORM, &args![data]).u32() {
            0 | 1 => result = e.vcall(p, 0x844, &args![actor, anchor]).u8() != 0,
            2 => {
                let index = e.call(SANDBOX_WORD_10, &args![data]).u32();
                e.vcall(p, 0x7bc, &args![actor, index]);
            }
            3 => {
                let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
                e.vcall(p, 0x84c, &args![actor, 0u32, minus_one]);
            }
            4 => {
                e.vcall(p, 0x868, &args![actor, anchor]);
            }
            5 => {
                if e.vcall(p, 0x850, &args![actor, anchor]).u8() != 0 {
                    let package = e.vcall(p, 0x22c, &[]).u32();
                    e.call(SANDBOX_PICK_PROCEDURE, &args![data, actor, package]);
                    e.mem.set_u32(data + 4, 2);
                }
            }
            _ => result = false,
        }
    }
    let target = e.mem.u32(actor + 0x190);
    if !result || (target != 0 && e.call(SANDBOX_TARGET_DONE, &args![target]).bool()) {
        let package = e.vcall(p, 0x22c, &[]).u32();
        e.call(SANDBOX_FORGET_TARGET, &args![data, actor, package]);
        e.mem.set_u32(data + 4, 1);
    }
    if e.call(ACTOR_IS_PATHING, &args![actor]).bool() {
        e.vcall(p, 0x350, &args![actor, 0x101u32, 1u32]);
    }
}

// Translated from 0092a980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the sandbox data type is outside 2 to 3 (cdecl `(type)`, a signed
/// compare); the engine map has no name.
pub fn fn_0092a980(_e: &mut Engine, kind: u32) -> bool {
    !(2..=3).contains(&(kind as i32))
}

/// `TESFurniture::CanSitOn` (Xbox PDB), `(furniture)`.
const FURNITURE_CAN_SIT_ON: u32 = 0x0050_93f0;
/// `LowProcess::ProcessPatrolStartAtRefBehavior` (Xbox PDB), `(process,
/// actor, data)`: see [`middle_high_process_process_patrol_start_at_ref_behavior`].
const LOW_PROCESS_PATROL_START_AT_REF: u32 = 0x0091_2960;

// Translated from 0092a9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessUseFurniture` (Xbox PDB): `(process, actor,
/// furniture)`. False when the furniture is not a furniture reference.
/// When it is not the process's furniture (+0x140) and the actor already is
/// in a furniture state, that state's slot +0x418 runs (states 4 and 9) and
/// true is answered; otherwise it becomes the process's furniture
/// (`0092c680`). The wanted furniture state is 4 when `CanSitOn` says so,
/// else 9; an actor in another state whose slot +0x218 is true enters it
/// (slot +0x12C with the furniture when the target (+0x40) is missing, slot
/// +0x7C8 with (actor, 0)); in the wanted state slot +0x118 runs with 1.
/// When `fn_00915ef0` accepts the actor the furniture is dropped (heading
/// cleared, marker cleared, slots +0x294 and +0x288) and false is
/// answered. In state 9 the actor's face animation data's slot +0xD4
/// decides whether slot +0xD8 runs with (1, 0).
pub fn middle_high_process_process_use_furniture(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    furniture: u32,
) -> bool {
    let p = this.addr();
    e.vcall(p, 0x27c, &[]);
    if furniture == 0 || !e.call(REFERENCE_IS_FURNITURE, &args![furniture]).bool() {
        return false;
    }
    if e.mem.u32(p + 0x140) != furniture {
        if e.vcall(actor, 0x214, &[]).u32() != 0 {
            if e.vcall(actor, 0x214, &[]).u32() == 4 || e.vcall(actor, 0x214, &[]).u32() == 9 {
                e.vcall(actor, 0x418, &[]);
            }
            return true;
        }
        e.mem.set_u32(p + 0x140, furniture);
        e.call(CLEAR_FURNITURE_MARKER, &args![p]);
    }
    if e.mem.u32(p + 0x140) == 0 {
        e.call(CLEAR_FURNITURE_MARKER, &args![p]);
    }
    let current = e.mem.u32(p + 0x140);
    let base = e.call(REFERENCE_BASE, &args![current]).u32();
    let can_sit = e.call(FURNITURE_CAN_SIT_ON, &args![base]).u8();
    let wanted = if can_sit != 0 { 4 } else { 9 };
    if e.vcall(actor, 0x214, &[]).u32() != wanted && e.vcall(actor, 0x218, &[]).bool() {
        if e.mem.u32(p + 0x40) == 0 {
            let current = e.mem.u32(p + 0x140);
            e.vcall(p, 0x12c, &args![current]);
        }
        e.vcall(p, 0x7c8, &args![actor, 0u32]);
    }
    if e.vcall(actor, 0x214, &[]).u32() == wanted {
        e.vcall(p, 0x118, &args![1u32]);
    }
    if e.call(PROCESS_FURNITURE_DROP, &args![p, actor]).bool() {
        e.mem.set_u32(p + 0x140, 0);
        e.call(FURNITURE_MARK_SET_HEADING, &args![p + 0x148, 0.0f32]);
        e.call(SET_BYTE_E, &args![p + 0x148, 0u32]);
        e.call(COPY_POINT, &args![p + 0x148, FURNITURE_MARK_ORIGIN]);
        e.vcall(p, 0x294, &args![actor]);
        e.vcall(p, 0x288, &args![actor, 1u32]);
        return false;
    }
    if e.vcall(actor, 0x214, &[]).u32() == 9 {
        let face = e.call(ACTOR_GET_FACE_ANIMATION_DATA, &args![actor]).u32();
        if face != 0 && !e.vcall(face, 0xd4, &[]).bool() {
            e.vcall(face, 0xd8, &args![1u32, 0u32]);
        }
    }
    true
}

/// `(process, actor)`: whether the actor should leave the furniture
/// (`00915ef0`, this unit; the engine map has no name).
const PROCESS_FURNITURE_DROP: u32 = 0x0091_5ef0;
/// The Calendar's hour-of-day accessor reading a process time value:
/// `(process area)` answered in ST0 (`00621b00`, the engine map has no name).
const PROCESS_TIME_VALUE: u32 = 0x0062_1b00;
/// `(float)`: `fabs` on a float (`00408860`; `00408840` wraps it).
const FLOAT_ABS_WRAPPER: u32 = 0x0040_8840;
const FLOAT_ABS: u32 = 0x0040_8860;
/// The double 0.05 (exe double at `0x0103c7c8`, the float 0.05 widened).
const MOVEMENT_BLOCKED_TIME: f64 = 0.05_f32 as f64;
/// `TESPackage::SetIsCreated` (Xbox PDB), `(package, 1)`.
const PACKAGE_SET_IS_CREATED: u32 = 0x0067_4d70;
/// `Actor::ClearLookAtTarget` is the same getter as [`ACTOR_CLEAR_LOOK_AT_TARGET`].
/// `(actor)`: sets the actor's special-idle stopped state (`008b3ab0`, the
/// engine map has no name).
const ACTOR_STOP_SPECIAL_IDLE: u32 = 0x008b_3ab0;

// Translated from 0092ac10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessMovementBlocked` (Xbox PDB): `(process,
/// actor)`. Takes the time value of the process area (the one at +0xE4 when
/// slot +0x20C answers non-zero, else the one at +4) and compares the
/// absolute difference between the calendar's hour and that value with
/// 0.05: when at least that, slot +0x288 runs with (actor, 1) and the
/// package behind the area (+4) is marked created (`TESPackage::SetIsCreated`
/// with 1); otherwise `008b3ab0` runs on the actor.
pub fn middle_high_process_process_movement_blocked(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    let p = this.addr();
    let area = if e.vcall(p, 0x20c, &[]).u32() != 0 {
        p + 0xe4
    } else {
        p + 4
    };
    let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64();
    let value = e.call(PROCESS_TIME_VALUE, &args![area]).f64();
    let difference = (hour - value) as f32;
    let difference_bits = x87_quiet_bits(difference.to_bits());
    let distance = e.call(FLOAT_ABS_WRAPPER, &args![difference_bits]).f64() as f32;
    if f64::from(distance) >= MOVEMENT_BLOCKED_TIME {
        e.vcall(p, 0x288, &args![actor, 1u32]);
        let package = e.call(FIRST_WORD, &args![p + 4]).u32();
        e.call(PACKAGE_SET_IS_CREATED, &args![package, 1u32]);
    } else {
        e.call(ACTOR_STOP_SPECIAL_IDLE, &args![actor]);
    }
}

// Translated from 0092acd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessPatrol` (Xbox PDB): `(process, actor)`.
/// Without patrol data (`008d0430`) slot +0x284 runs with -1. An actor in
/// a furniture state (slot +0x214) whose patrol reference
/// (`PatrolActorPackageData::GetCurrentReference`) is not the process's
/// furniture (+0x140) gets slot +0x418 on the actor. Otherwise the patrol
/// state (+4) 0, 1 or 2 runs slot +0x854, +0x85C or +0x864 with (actor,
/// data).
pub fn middle_high_process_process_patrol(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    let p = this.addr();
    let data = e.call(PROCESS_PACKAGE, &args![p]).u32();
    if data == 0 {
        e.vcall(p, 0x284, &args![u32::MAX]);
        return;
    }
    if e.vcall(actor, 0x214, &[]).u32() != 0
        && e.call(PATROL_GET_CURRENT_REFERENCE, &args![data]).u32() != e.mem.u32(p + 0x140)
    {
        e.vcall(actor, 0x418, &[]);
        return;
    }
    match e.mem.u32(data + 4) {
        0 => {
            e.vcall(p, 0x854, &args![actor, data]);
        }
        1 => {
            e.vcall(p, 0x85c, &args![actor, data]);
        }
        2 => {
            e.vcall(p, 0x864, &args![actor, data]);
        }
        _ => {}
    }
}

/// `PatrolActorPackageData::Init` (Xbox PDB), `(data, actor, package)`.
const PATROL_INIT: u32 = 0x009f_2ba0;

// Translated from 0092adb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessPatrolInit` (Xbox PDB): `(process, actor,
/// data)`. Initialises the patrol data for the actor and the process's
/// package (slot +0x22C); on failure slot +0x284 runs with -1 and false is
/// answered, otherwise slot +0x858 runs with (actor, data) and true is
/// answered.
pub fn middle_high_process_process_patrol_init(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    data: u32,
) -> bool {
    let p = this.addr();
    let package = e.vcall(p, 0x22c, &[]).u32();
    if !e.call(PATROL_INIT, &args![data, actor, package]).bool() {
        e.vcall(p, 0x284, &args![u32::MAX]);
        return false;
    }
    e.vcall(p, 0x858, &args![actor, data]);
    true
}

/// 32.0 (exe float at `0x0101e340`): the patrol arrival radius used when
/// the package gives none.
const PATROL_DEFAULT_RADIUS: u32 = 0x0101_e340;
/// 10.0 (exe float at `0x01017b78`): the speed used for a type 0x30 start.
const PATROL_START_SPEED: u32 = 0x0101_7b78;
/// `(request, float)` (`008e7c20`, the engine map has no name).
const PATHING_REQUEST_SET_FLOAT: u32 = 0x008e_7c20;
/// `(package)`: whether the package allows travelling at speed
/// (`0067a480`, the engine map has no name).
const PACKAGE_ALLOWS_SPEED: u32 = 0x0067_a480;
/// 64.0 (exe float at `0x0101e6ec`): the distance above which a patrol
/// target is chased faster.
const PATROL_RUN_DISTANCE: u32 = 0x0101_e6ec;
/// `ActorMover::GetCurrentPercent` (Xbox PDB), `(actor path handle word)`,
/// answered in ST0.
const PATH_PROGRESS: u32 = 0x009d_cbf0;
/// 0.925 (exe float at `0x01089b28`).
const PATH_PROGRESS_LIMIT: u32 = 0x0108_9b28;
/// The double 50.0 (exe double at `0x0101e2c0`).
const PATROL_RADIUS_MARGIN: f64 = 50.0;
/// The double 32400.0 (exe double at `0x01089b20`), 180 squared.
const PATROL_HEIGHT_LIMIT_SQUARED: f64 = 32400.0;
/// `(extra data list)`: the word at +0xC of the list's extra of type 0x6F
/// (`0041fe90`, 0 for none), and `(object)`: the float at its start
/// (`006a7f50`); the engine map has no names.
const EXTRA_LIST_TYPE_6F_WORD: u32 = 0x0041_fe90;
const FLOAT_AT_START: u32 = 0x006a_7f50;
/// `(vector)`: the squared length of the first two components (`00595c80`,
/// answered in ST0; the engine map has no name).
const VECTOR_LENGTH_2D_SQUARED: u32 = 0x0059_5c80;

/// The distance (at `reference`) the patrol travel and arrival checks use:
/// 32.0 by default, the speed limit (`006e25d0`) for type 0x30 bases or the
/// patrol base form, combined with the package radius by max (`00404010`).
fn patrol_radius(e: &mut Engine, p: u32, actor: u32, reference: u32) -> f32 {
    let mut radius = x87_quiet(e.global::<f32>(PATROL_DEFAULT_RADIUS));
    let base = e.call(REFERENCE_BASE, &args![reference]).u32();
    if e.call(FORM_TYPE, &args![base]).u32() == 0x30 {
        radius = e.call(PATHING_SPEED_LIMIT, &[]).f64() as f32;
        return radius;
    }
    let base = e.call(REFERENCE_BASE, &args![reference]).u32();
    if base == e.global::<u32>(ACTIVATE_FORM_A) {
        radius = e.call(PATHING_SPEED_LIMIT, &[]).f64() as f32;
    }
    let package = e.vcall(p, 0x22c, &[]).u32();
    if package != 0 {
        let integer = e.call(PACKAGE_RADIUS, &args![package, actor]).u32() as f32;
        radius = e
            .call(
                MAX_OF_FLOATS,
                &args![x87_quiet_bits(radius.to_bits()), integer.to_bits()],
            )
            .f64() as f32;
    }
    radius
}

// Translated from 0092ae20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0092ae20 - the exception frame (FS:[0]) is not translated
/// `MiddleHighProcess::ProcessPatrolStartTravelToNextRef` (Xbox PDB):
/// `(process, actor, data)`. Without a patrol reference (`GetCurrentReference`)
/// slot +0x284 runs with -1. Otherwise slot +0x12C runs with the reference
/// and a pathing request to it is built (`Actor::BuildRequest`, speed from
/// `006e25d0` combined with the package radius by max, 10.0 for a type 0x30
/// base), given a float (`008e7c20`, the reference's +24 point's +8) for
/// the type 0x30 base or the patrol base form, set as the actor's goal, the
/// patrol state (+4) becomes 1, and slot +0x860 runs with (actor, data) when
/// the actor's path is complete.
pub fn middle_high_process_process_patrol_start_travel_to_next_ref(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    data: u32,
) {
    let p = this.addr();
    let reference = e.call(PATROL_GET_CURRENT_REFERENCE, &args![data]).u32();
    if reference == 0 {
        e.vcall(p, 0x284, &args![u32::MAX]);
        return;
    }
    e.vcall(p, 0x12c, &args![reference]);
    let mut speed = e.call(PATHING_SPEED_LIMIT, &[]).f64() as f32;
    let package = e.vcall(p, 0x27c, &[]).u32();
    if package != 0 {
        let integer = e.call(PACKAGE_RADIUS, &args![package, actor]).u32() as f32;
        speed = e
            .call(MAX_OF_FLOATS, &args![speed.to_bits(), integer.to_bits()])
            .f64() as f32;
    }
    let base = e.call(REFERENCE_BASE, &args![reference]).u32();
    let is_type_30 = e.call(FORM_TYPE, &args![base]).u32() == 0x30;
    if is_type_30 {
        speed = x87_quiet(e.global::<f32>(PATROL_START_SPEED));
    }
    e.with_stack(0xc0, |e, request| {
        let request = request.addr();
        e.call(PATHING_REQUEST_CONSTRUCT, &args![request]);
        let worldspace = e.call(REFERENCE_GET_WORLD_SPACE, &args![reference]).u32();
        let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
        let position = e.vcall(reference, 0x1f4, &[]).u32();
        e.call(
            ACTOR_BUILD_REQUEST,
            &args![
                actor,
                request,
                position,
                cell,
                worldspace,
                x87_quiet_bits(speed.to_bits()),
                0u32
            ],
        );
        e.call(PATHING_REQUEST_SET_FLAG, &args![request, 0u32]);
        if is_type_30 || base == e.global::<u32>(ACTIVATE_FORM_A) {
            let plus_24 = e.call(REFERENCE_PLUS_24, &args![reference]).u32();
            let value = x87_quiet_bits(e.mem.u32(plus_24 + 8));
            e.call(PATHING_REQUEST_SET_FLOAT, &args![request, value]);
        }
        e.call(ACTOR_SET_GOAL_FROM_REQUEST, &args![actor, request]);
        e.mem.set_u32(data + 4, 1);
        if e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
            e.vcall(p, 0x860, &args![actor, data]);
        }
        e.call(PATHING_REQUEST_RELEASE, &args![request]);
    });
}

/// `PatrolActorPackageData::SetNextReferenceForPatrol` (Xbox PDB), `(data,
/// 1)`.
const PATROL_SET_NEXT_REFERENCE: u32 = 0x009f_2ea0;

/// The squared horizontal distance between the actor and the patrol
/// reference and the vertical offset: the actor's point (slot +0x1F4) minus
/// the reference's, written to `vector`.
fn patrol_offset(e: &mut Engine, actor: u32, reference: u32, vector: u32) -> (f32, f32) {
    let reference_point = e.vcall(reference, 0x1f4, &[]).u32();
    let actor_point = e.vcall(actor, 0x1f4, &[]).u32();
    e.call(POINT_SUBTRACT, &args![actor_point, vector, reference_point]);
    let length = e.call(VECTOR_LENGTH_2D_SQUARED, &args![vector]).f64() as f32;
    let height = e.mem.f32(vector + 8);
    (length, height)
}

/// The wait value of the reference's type 0x6F extra data (0.0 for none).
fn patrol_wait(e: &mut Engine, reference: u32) -> f32 {
    let list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
    let mut wait = 0.0f32;
    if e.call(EXTRA_LIST_TYPE_6F_WORD, &args![list]).u32() != 0 {
        let list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
        let extra = e.call(EXTRA_LIST_TYPE_6F_WORD, &args![list]).u32();
        wait = e.call(FLOAT_AT_START, &args![extra]).f64() as f32;
    }
    wait
}

// Translated from 0092b030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessPatrolTravelingToNextRef` (Xbox PDB):
/// `(process, actor, data)`. When the actor waits on a path (`008b3b90`)
/// slot +0x858 runs with (actor, data). With a patrol reference the
/// movement is set: mode 0x101, or, when `0067a480` allows it for the
/// package and the reference is farther than 64.0, the mode of slot
/// +0x34C with (actor, distance, the package radius from the actor to its
/// location (`TESPackage::GetPackageRadiusActorToLocation`, Xbox PDB), twice
/// that, 0, 0);
/// then slot +0x350 runs with (actor, mode, running). A furniture
/// reference (form type 0x27) is used through slot +0x844 and, when it is
/// the process's furniture in furniture state 3 or 8, slot +0x860 runs; the
/// call ends. Otherwise the arrival is decided from the horizontal offset
/// to the reference (against the squared patrol radius, plus 50.0 and a
/// wait of 0 while the actor still travels) and the height offset (against
/// 180 squared): slot +0x860 runs on arrival and slot +0x858 while still
/// travelling.
pub fn middle_high_process_process_patrol_traveling_to_next_ref(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    data: u32,
) {
    let p = this.addr();
    if e.call(ACTOR_BYTE_70, &args![actor]).bool() {
        e.vcall(p, 0x858, &args![actor, data]);
        return;
    }
    let reference = e.call(PATROL_GET_CURRENT_REFERENCE, &args![data]).u32();
    if reference != 0 {
        let mut mode = 0x101u32;
        let package = e.vcall(p, 0x27c, &[]).u32();
        let allows_speed = e.call(PACKAGE_ALLOWS_SPEED, &args![package]).u8();
        let mut running = false;
        if allows_speed != 0 {
            let distance = e
                .call(
                    REFERENCE_GET_DISTANCE_FROM_REFERENCE,
                    &args![actor, reference, 0u32, 0u32],
                )
                .f64() as f32;
            let run_distance = x87_quiet(e.global::<f32>(PATROL_RUN_DISTANCE));
            running = f64::from(run_distance) < f64::from(distance);
            if running {
                let speed = e
                    .call(PACKAGE_GET_RADIUS_TO_LOCATION, &args![package, actor, 0u32])
                    .f64() as f32;
                let doubled = (f64::from(speed) + f64::from(speed)) as f32;
                mode = e
                    .vcall(
                        p,
                        0x34c,
                        &args![
                            actor,
                            x87_quiet_bits(distance.to_bits()),
                            x87_quiet_bits(speed.to_bits()),
                            x87_quiet_bits(doubled.to_bits()),
                            0u32,
                            0u32
                        ],
                    )
                    .u32();
            }
        }
        e.vcall(p, 0x350, &args![actor, mode, running as u32]);
    }
    if reference != 0 {
        let base = e.call(REFERENCE_BASE, &args![reference]).u32();
        if e.call(FORM_TYPE, &args![base]).u32() == 0x27 {
            e.vcall(p, 0x844, &args![actor, reference]);
            if reference == e.mem.u32(p + 0x140) {
                let state = e.vcall(p, 0x4bc, &[]).u32();
                if state == 3 || e.vcall(p, 0x4bc, &[]).u32() == 8 {
                    e.vcall(p, 0x860, &args![actor, data]);
                }
            }
            return;
        }
    }
    let mut has_idles = false;
    let package = e.vcall(p, 0x27c, &[]).u32();
    if package != 0 {
        let collection = e.call(PACKAGE_IDLE_COLLECTION, &args![package]).u32();
        if collection != 0 && (fn_0092b440(e, collection) as i32) > 0 {
            has_idles = true;
        }
    }
    let path_complete = e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).u8();
    let path_word = e.mem.u32(actor + 0x190);
    let progress = e.call(PATH_PROGRESS, &args![path_word]).f64() as f32;
    let progress_limit = x87_quiet(e.global::<f32>(PATH_PROGRESS_LIMIT));
    if path_complete == 0 {
        if has_idles {
            return;
        }
        if f64::from(progress_limit) >= f64::from(progress) || progress.is_nan() {
            return;
        }
    }
    e.with_stack(0x10, |e, vector| {
        let vector = vector.addr();
        let (length, height) = patrol_offset(e, actor, reference, vector);
        let radius = patrol_radius(e, p, actor, reference);
        let radius_squared = (f64::from(radius) * f64::from(radius)) as f32;
        let height_squared = f64::from(height) * f64::from(height);
        let mut waiting_done = false;
        if path_complete == 0 {
            let margin = (f64::from(radius) + PATROL_RADIUS_MARGIN) as f32;
            let margin_squared = (f64::from(margin) * f64::from(margin)) as f32;
            if f64::from(margin_squared) < f64::from(length) {
                return;
            }
            if height_squared > PATROL_HEIGHT_LIMIT_SQUARED {
                return;
            }
            if patrol_wait(e, reference) != 0.0 {
                return;
            }
            waiting_done = true;
        }
        let travelling = f64::from(radius_squared) < f64::from(length)
            && height_squared < PATROL_HEIGHT_LIMIT_SQUARED;
        if waiting_done || !travelling {
            e.vcall(p, 0x860, &args![actor, data]);
        } else {
            e.vcall(p, 0x858, &args![actor, data]);
        }
    });
}

// Translated from 0092b440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +5 of the object zero-extended (the idle collection's play
/// count); the engine map has no name.
pub fn fn_0092b440(e: &mut Engine, this: u32) -> u32 {
    e.mem.u8(this + 5) as u32
}

// Translated from 0092b550 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0xC of the object (the word-at-+8 getter run on the area at
/// +4; the engine map has no name).
pub fn fn_0092b550(e: &mut Engine, this: u32) -> u32 {
    e.call(ITEM_CHANGE_FORM, &args![this + 4]).u32()
}

/// `Script::Run` (Xbox PDB), `(script, actor, 0, 0, 0)`.
const SCRIPT_RUN: u32 = 0x005a_c1e0;

// Translated from 0092b460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessPatrolStartAtRefBehavior` (Xbox PDB):
/// `(process, actor, data)`. Runs `LowProcess::ProcessPatrolStartAtRefBehavior`
/// (`00912960`) and clears the actor's look-at target. Unless the patrol
/// reference is a type 0x30 form the idle timer (+0x198) is reset to 0.0.
/// With the reference's type 0x6F extra (`0041fe90`) the extra's point
/// (`fn_0092b550`) is used through slot +0x2A4 with (actor, point, 0, 0,
/// 1, 0) when the actor's acquire object's slot +0x30C answers false, and
/// the extra's patrol data (`008d0430`) is cleared with (actor, 0, 0, 0)
/// (`005ac1e0`).
pub fn middle_high_process_process_patrol_start_at_ref_behavior(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    data: u32,
) {
    let p = this.addr();
    e.call(LOW_PROCESS_PATROL_START_AT_REF, &args![p, actor, data]);
    e.call(ACTOR_CLEAR_LOOK_AT_TARGET, &args![actor]);
    let reference = e.call(PATROL_GET_CURRENT_REFERENCE, &args![data]).u32();
    let mut type_30 = false;
    if reference != 0 {
        let base = e.call(REFERENCE_BASE, &args![reference]).u32();
        type_30 = e.call(FORM_TYPE, &args![base]).u32() == 0x30;
    }
    if !type_30 {
        e.mem.set_f32(p + 0x198, 0.0);
    }
    let list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
    let extra = e.call(EXTRA_LIST_TYPE_6F_WORD, &args![list]).u32();
    if extra != 0 {
        let point = fn_0092b550(e, extra);
        if point != 0 {
            let acquired = e.call(WORD_68, &args![actor]).u32();
            if !e.vcall(acquired, 0x30c, &[]).bool() {
                e.vcall(p, 0x2a4, &args![actor, point, 0u32, 0u32, 1u32, 0u32]);
            }
        }
        let patrol_data = e.call(PROCESS_PACKAGE, &args![extra]).u32();
        if patrol_data != 0 {
            e.call(SCRIPT_RUN, &args![patrol_data, actor, 0u32, 0u32, 0u32]);
        }
    }
}

/// The double 180.0 (exe double at `0x0104ed58`): the height limit of
/// `ProcessPatrolAtRef`.
const AT_REF_HEIGHT_LIMIT: f64 = 180.0;
/// `Actor::IsPlayingLowerBodySpecialIdle` (Xbox PDB), `(actor)`.
const ACTOR_PLAYING_LOWER_BODY_IDLE: u32 = 0x008b_a530;

// Translated from 0092b570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessPatrolAtRef` (Xbox PDB): `(process, actor,
/// data)`. Without a patrol reference: slot +0x858 runs with (actor, data)
/// when `SetNextReferenceForPatrol(data, 1)` finds another, else slot
/// +0x284 with -1. With one, when the actor's path is complete and the
/// actor is farther from the reference than the patrol radius (squared) or
/// above the height limit (180), slot +0x858 runs and the call ends.
/// Otherwise the wait value of the reference's extra data decides: when it
/// is positive for a type 0x30 reference slot +0x868 runs with (actor,
/// reference); the elapsed time (data +8) grows by the frame time and,
/// once at least the wait and unless the actor is playing a lower body
/// special idle (`008ba530`), the next reference is taken (slot +0x858) or
/// slot +0x288 runs with (actor, 1).
pub fn middle_high_process_process_patrol_at_ref(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    data: u32,
) {
    let p = this.addr();
    let reference = e.call(PATROL_GET_CURRENT_REFERENCE, &args![data]).u32();
    if reference == 0 {
        if e.call(PATROL_SET_NEXT_REFERENCE, &args![data, 1u32]).bool() {
            e.vcall(p, 0x858, &args![actor, data]);
        } else {
            e.vcall(p, 0x284, &args![u32::MAX]);
        }
        return;
    }
    if e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
        let leaves = e.with_stack(0x10, |e, vector| {
            let vector = vector.addr();
            let (length, height) = patrol_offset(e, actor, reference, vector);
            let radius = patrol_radius(e, p, actor, reference);
            let radius_squared = (f64::from(radius) * f64::from(radius)) as f32;
            if f64::from(radius_squared) < f64::from(length) {
                return true;
            }
            let absolute = e
                .call(FLOAT_ABS, &args![x87_quiet_bits(height.to_bits())])
                .f64();
            absolute > AT_REF_HEIGHT_LIMIT
        });
        if leaves {
            e.vcall(p, 0x858, &args![actor, data]);
            return;
        }
    }
    let wait = patrol_wait(e, reference);
    if wait > 0.0 {
        let base = e.call(REFERENCE_BASE, &args![reference]).u32();
        if e.call(FORM_TYPE, &args![base]).u32() == 0x30 {
            e.vcall(p, 0x868, &args![actor, reference]);
        }
    }
    let frame_time = e.call(FLOAT_C, &args![GLOBAL_TIME_DATA]).f64();
    let elapsed = (frame_time + f64::from(e.mem.f32(data + 8))) as f32;
    e.mem.set_f32(data + 8, elapsed);
    let elapsed = e.mem.f32(data + 8);
    if f64::from(wait) <= f64::from(elapsed) {
        if e.call(ACTOR_PLAYING_LOWER_BODY_IDLE, &args![actor]).bool() {
            return;
        }
        if e.call(PATROL_SET_NEXT_REFERENCE, &args![data, 1u32]).bool() {
            e.vcall(p, 0x858, &args![actor, data]);
        } else {
            e.vcall(p, 0x288, &args![actor, 1u32]);
        }
    }
}

// Translated from 0092b7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the 15 damage node slots (0x3C bytes at +0x1DC) and the two
/// dismembered limb slots (+0x218, +0x21C); the engine map has no name.
pub fn fn_0092b7f0(e: &mut Engine, this: Ptr<MiddleHighProcess>) {
    let p = this.addr();
    e.call(MEMSET, &args![p + 0x1dc, 0u32, 0x3cu32]);
    e.mem.set_u32(p + 0x218, 0);
    e.mem.set_u32(p + 0x21c, 0);
}

/// `(object)`: the object's damage node array at +0x34 (`006a9540`, the
/// same getter as the path location).
/// `(node)`: the node's name data (`007fa950`) and `(root, name)` the node
/// found under the root (`004aae30`, cdecl); `(node)` the node's slot index
/// (`005e5300`); `BGSBodyPartData::GetBodyPart` (Xbox PDB) `(data, flag)`;
/// `(part)` its node name (`0043b230`). None has an engine map name except
/// `GetBodyPart`.
const NODE_NAME: u32 = 0x007f_a950;
const FIND_NODE: u32 = 0x004a_ae30;
const NODE_SLOT: u32 = 0x005e_5300;
const BODY_PART_DATA_GET_BODY_PART: u32 = 0x005e_50f0;
const BODY_PART_NODE_NAME: u32 = 0x0043_b230;

// Translated from 0092b830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::StoreDamageNodes` (Xbox PDB): `(process, root,
/// data)`. For each of the 15 body part entries of `data` (`006a9540`, the
/// array at +0x34) that exists, finds its node under `root` (`004aae30`
/// with the entry's name, `007fa950`) and stores it at the entry's slot
/// (`005e5300`, +0x1DC + 4 * slot). The data's body parts 1 and 0
/// (`BGSBodyPartData::GetBodyPart`) with a node name (`0043b230`) have
/// their nodes found under `root` and stored at +0x218 and +0x21C.
pub fn middle_high_process_store_damage_nodes(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    root: u32,
    data: u32,
) {
    let p = this.addr();
    let entries = e.call(PATH_LOCATION, &args![data]).u32();
    for index in 0..15u32 {
        let entry = e.mem.u32(entries + index * 4);
        if entry == 0 {
            continue;
        }
        let name = e.call(NODE_NAME, &args![entry]).u32();
        let node = if name != 0 {
            e.call(FIND_NODE, &args![root, name]).u32()
        } else {
            0
        };
        let slot = e.call(NODE_SLOT, &args![entry]).u32();
        e.mem.set_u32(
            p.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x1dc),
            node,
        );
    }
    let first = if data != 0 {
        e.call(BODY_PART_DATA_GET_BODY_PART, &args![data, 1u32])
            .u32()
    } else {
        0
    };
    if first != 0 && e.call(BODY_PART_NODE_NAME, &args![first]).u32() != 0 {
        let name = e.call(BODY_PART_NODE_NAME, &args![first]).u32();
        let node = e.call(FIND_NODE, &args![root, name]).u32();
        e.mem.set_u32(p + 0x218, node);
    }
    let second = if data != 0 {
        e.call(BODY_PART_DATA_GET_BODY_PART, &args![data, 0u32])
            .u32()
    } else {
        0
    };
    if second != 0 && e.call(BODY_PART_NODE_NAME, &args![second]).u32() != 0 {
        let name = e.call(BODY_PART_NODE_NAME, &args![second]).u32();
        let node = e.call(FIND_NODE, &args![root, name]).u32();
        e.mem.set_u32(p + 0x21c, node);
    }
}

// Translated from 0092b970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::StoreDismemberedLimbNode` (Xbox PDB): `(process,
/// index, node)`. Stores the node in the damage node slot `index` (+0x1DC +
/// 4 * index); index 1 clears the dismembered node at +0x218 and index 0
/// the one at +0x21C.
pub fn middle_high_process_store_dismembered_limb_node(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    index: u32,
    node: u32,
) {
    let p = this.addr();
    e.mem.set_u32(
        p.wrapping_add(index.wrapping_mul(4)).wrapping_add(0x1dc),
        node,
    );
    if index == 1 {
        e.mem.set_u32(p + 0x218, 0);
    }
    if index == 0 {
        e.mem.set_u32(p + 0x21c, 0);
    }
}

// Translated from 0092b9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetDamageNode` (Xbox PDB): `(process, index)`. The
/// node stored in the damage node slot `index`.
pub fn middle_high_process_get_damage_node(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    index: u32,
) -> u32 {
    let p = this.addr();
    e.mem
        .u32(p.wrapping_add(index.wrapping_mul(4)).wrapping_add(0x1dc))
}

/// `ItemChange::GetItemHealth` (Xbox PDB), `(item change, 1)` answered in
/// ST0.
const ITEM_CHANGE_GET_ITEM_HEALTH: u32 = 0x004b_cdb0;
/// `(form)`: whether bit 0x20 of the form's byte at +0x100 is set
/// (`0046e8c0`, the engine map has no name).
const WEAPON_FORM_FLAG: u32 = 0x0046_e8c0;
/// `(weapon)`: a weapon attribute for the repair (`00522c30`, no stack word,
/// the engine map has no name).
const WEAPON_ATTRIBUTE: u32 = 0x0052_2c30;
/// The double 0.1 (exe double at `0x0101ffa0`, the float 0.1 widened) the health is scaled by.
const CONDITION_SCALE: u32 = 0x0101_ffa0;
/// `(value)`: a float rounding helper answering in ST0 (`004e4430`, the
/// engine map has no name; it calls the CRT through `004e4450`).
const FLOAT_ROUND: u32 = 0x004e_4430;

// Translated from 0092b9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetWeaponCondition` (Xbox PDB): `(process, object)`.
/// 0 without a weapon item (+0x114). The item's health
/// (`ItemChange::GetItemHealth(item, 1)`) is replaced by slot +0x39C with
/// the form's attribute and the object's base form when the item's form has
/// flag 0x20 in its byte at +0x100. The health scaled by 0.1 is rounded
/// (`004e4430`), truncated and reduced by one when above 1.
pub fn middle_high_process_get_weapon_condition(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    object: u32,
) -> u32 {
    let p = this.addr();
    if e.mem.u32(p + 0x114) == 0 {
        return 0;
    }
    let item = e.mem.u32(p + 0x114);
    let form = e.call(ITEM_CHANGE_FORM, &args![item]).u32();
    let item = e.mem.u32(p + 0x114);
    let mut health = e
        .call(ITEM_CHANGE_GET_ITEM_HEALTH, &args![item, 1u32])
        .f64() as f32;
    if form != 0 && e.call(WEAPON_FORM_FLAG, &args![form]).bool() {
        // 0092ba3c: `00522c30` takes no stack word (plain RET); the pushed object
        // is the last word of the process' +0x39c (RET 0xc).
        let attribute = e.call(WEAPON_ATTRIBUTE, &args![form]).u32();
        let base = e.call(ACTOR_EXTRA_OBJECT, &args![object]).u32();
        health = e.vcall(p, 0x39c, &args![base, attribute, object]).f64() as f32;
    }
    let scale = e.global::<f64>(CONDITION_SCALE);
    let scaled = (f64::from(health) * scale) as f32;
    let rounded = e
        .call(FLOAT_ROUND, &args![x87_quiet_bits(scaled.to_bits())])
        .f64();
    let mut condition = e.call(FTOL, &args![rounded]).u32() as i32;
    if condition > 1 {
        condition -= 1;
    }
    condition as u32
}

/// `TESBipedModelForm` accessors on the armor's biped data at +0x70:
/// `(biped)` bit 4 of its byte at +8 (`00480d30`), `(biped)`
/// `IsPowerArmor` (`00480d10`, Xbox PDB), `(biped, slot, a, b)`
/// `FillsBipedSlot` (`00480af0`, Xbox PDB) and `(biped)` bit 0x80 of the
/// byte at +8 (`004c0bd0`).
const BIPED_FLAG_4: u32 = 0x0048_0d30;
const BIPED_IS_POWER_ARMOR: u32 = 0x0048_0d10;
const BIPED_FILLS_SLOT: u32 = 0x0048_0af0;
const BIPED_FLAG_80: u32 = 0x004c_0bd0;
/// `ItemChange::GetWorn` (Xbox PDB), `(item change, 0)`.
const ITEM_CHANGE_GET_WORN: u32 = 0x004b_ddd0;
/// `(actor)`: the actor's word at +0x1B4 (`005d9f90`, the engine map has no
/// name).
const ACTOR_WORD_1B4: u32 = 0x005d_9f90;

// `Actor::GetInventoryChanges` (`004bf220`, `(actor)`): the actor's
// inventory change list holder; the engine map has no name.
// `(actor)`: a word of the actor (`008b70d0`, the engine map has no name).

// Translated from 0092baa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::UpdateArmorWorn` (Xbox PDB): `(process, actor)`.
/// Recomputes the worn armor flags +0x127..+0x12A from the actor's worn
/// armor forms (type 0x18): +0x12A when the biped data has flag 4, +0x129
/// or +0x128 for power armor depending on whether it fills slot 1 or 0,
/// +0x127 for biped data with flag 0x80 filling neither. The walk stops when
/// +0x128, +0x129 and +0x12A are all set. When the gate object
/// (`0042ce10` on the word at `0x011ddf38`) is false, +0x12A or +0x128
/// changed, +0x135 is clear, an item is held (+0x114) and the actor's slot
/// +0x218 is true, slot +0x1CC runs with (0, actor word at +0x1B4,
/// actor word `008b70d0`, actor).
pub fn middle_high_process_update_armor_worn(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    let p = this.addr();
    let old_power = e.mem.u8(p + 0x128);
    let old_flag = e.mem.u8(p + 0x12a);
    e.mem.set_u8(p + 0x127, 0);
    e.mem.set_u8(p + 0x128, 0);
    e.mem.set_u8(p + 0x129, 0);
    e.mem.set_u8(p + 0x12a, 0);
    let inventory = e.call(GET_INVENTORY_CHANGES, &args![actor]).u32();
    let mut node = e.mem.u32(inventory);
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        if e.mem.u8(p + 0x128) != 0 && e.mem.u8(p + 0x129) != 0 && e.mem.u8(p + 0x12a) != 0 {
            break;
        }
        let slot = e.call(IDENTITY, &args![node]).u32();
        let entry = e.mem.u32(slot);
        let form = e.call(ITEM_CHANGE_FORM, &args![entry]).u32();
        if e.call(ITEM_CHANGE_GET_WORN, &args![entry, 0u32]).u8() != 0
            && e.call(FORM_TYPE, &args![form]).u32() == 0x18
        {
            let biped = form + 0x70;
            if e.call(BIPED_FLAG_4, &args![biped]).u8() != 0 {
                e.mem.set_u8(p + 0x12a, 1);
            }
            if e.call(BIPED_IS_POWER_ARMOR, &args![biped]).u8() != 0 {
                let fills_first = e
                    .call(BIPED_FILLS_SLOT, &args![biped, 1u32, 0u32, 0u32])
                    .u8();
                if fills_first != 0
                    || e.call(BIPED_FILLS_SLOT, &args![biped, 0u32, 0u32, 0u32])
                        .u8()
                        != 0
                {
                    e.mem.set_u8(p + 0x129, 1);
                } else {
                    e.mem.set_u8(p + 0x128, 1);
                }
            }
            if e.call(BIPED_FLAG_80, &args![biped]).u8() != 0
                && e.call(BIPED_FILLS_SLOT, &args![biped, 1u32, 0u32, 0u32])
                    .u8()
                    == 0
                && e.call(BIPED_FILLS_SLOT, &args![biped, 0u32, 0u32, 0u32])
                    .u8()
                    == 0
            {
                e.mem.set_u8(p + 0x127, 1);
            }
        }
        node = e.call(WORD_4, &args![node]).u32();
    }
    let gate = e.global::<u32>(SAVE_LOAD_GAME);
    if e.call(SAVE_LOAD_FLAG_2, &args![gate]).u8() != 0 {
        return;
    }
    let changed = old_flag != e.mem.u8(p + 0x12a) || old_power != e.mem.u8(p + 0x128);
    if changed
        && e.mem.u8(p + 0x135) == 0
        && e.mem.u32(p + 0x114) != 0
        && e.vcall(actor, 0x218, &[]).u8() != 0
    {
        let word = e.call(ACTOR_GET_ANIMATION, &args![actor]).u32();
        let base = e.call(ACTOR_WORD_1B4, &args![actor]).u32();
        e.vcall(p, 0x1cc, &args![0u32, base, word, actor]);
    }
}

/// `(weapon form, actor)`: the projectile entry of the form (`00525a90`, the
/// engine map has no name) and `BGSProjectile::GetPreCloneCount` (Xbox
/// PDB), `(entry, weapon form)` (RET 4).
const FORM_PROJECTILE: u32 = 0x0052_5a90;
const PROJECTILE_GET_PRE_CLONE_COUNT: u32 = 0x004f_e080;
/// `ModelLoader::FindModel` (Xbox PDB), `(model loader, the model's +0x14 answer,
/// handle address)` (RET 8); the model loader is the word at `0x011c3b3c`.
const MODEL_LOADER_FIND_MODEL: u32 = 0x0044_72a0;
/// `(node name holder, count)`: constructs a 12 byte object (`004b0da0`) and
/// `(list, object)` takes it (`0092c8b0`); the engine map has no names.
const PROJECTILE_CLONE_CONSTRUCT: u32 = 0x004b_0da0;
/// The destructor of the handle `NiPointer` (`0040c110`).
const HANDLE_DESTRUCT: u32 = 0x0040_c110;

// Translated from 0092bd00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0092bd00 - the exception frame (FS:[0]) is not translated
/// `MiddleHighProcess::TryReserveProjectileClones` (Xbox PDB): `(process,
/// actor)`. Does nothing when the word list at +0x120 already has a first
/// word, or the one at +0x11C has none or fails `0043caa0`. The items
/// behind slots +0x148 and +0x14C must both exist; the inventory entry of
/// the first item's form for the actor (`00525a90`) gives a count
/// (`004fe080`); when positive the entry's object at +0x3C answers (slot
/// +0x14 with a handle) and `004472a0` runs with the word at `0x011c3b3c`;
/// with the handle's object having a node name (`0043b230`) a 12 byte
/// clone object is built (`004b0da0` with the name and the count) and taken
/// by the list at +0x120 (`0092c8b0`). The handle is destructed.
pub fn middle_high_process_try_reserve_projectile_clones(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    let p = this.addr();
    if e.call(FIRST_WORD, &args![p + 0x120]).u32() != 0 {
        return;
    }
    let first = e.call(FIRST_WORD, &args![p + 0x11c]).u32();
    if first == 0 || e.call(QUEUED_FILE_DONE, &args![first]).u8() == 0 {
        return;
    }
    let slot_first = e.vcall(p, 0x148, &[]).u32();
    let slot_second = e.vcall(p, 0x14c, &[]).u32();
    let item_form = if slot_first != 0 {
        let item = e.vcall(p, 0x148, &[]).u32();
        e.call(ITEM_CHANGE_FORM, &args![item]).u32()
    } else {
        0
    };
    let second_form = if slot_second != 0 {
        let item = e.vcall(p, 0x14c, &[]).u32();
        e.call(ITEM_CHANGE_FORM, &args![item]).u32()
    } else {
        0
    };
    if item_form == 0 || second_form == 0 {
        return;
    }
    if e.call(FORM_PROJECTILE, &args![item_form, actor]).u32() == 0 {
        return;
    }
    let entry = e.call(FORM_PROJECTILE, &args![item_form, actor]).u32();
    // 0092be32: `00525a90` takes one word (RET 4, the actor); the item form pushed
    // before it is the word of `004fe080` (RET 4).
    let count = e
        .call(PROJECTILE_GET_PRE_CLONE_COUNT, &args![entry, item_form])
        .u32() as i32;
    if count <= 0 {
        return;
    }
    e.with_stack(4, |e, handle| {
        let handle = handle.addr();
        e.call(POINTER_INIT, &args![handle, 0u32]);
        let entry = e.call(FORM_PROJECTILE, &args![item_form, actor]).u32();
        let object = entry + 0x3c;
        // 0092be76: the model's +0x14 (`00408da0`, plain RET) takes no stack word; the
        // pushed handle address is the last word of `004472a0` (RET 8).
        let answer = e.vcall(object, 0x14, &args![]).u32();
        let target = e.global::<u32>(MODEL_LOADER);
        e.call(MODEL_LOADER_FIND_MODEL, &args![target, answer, handle]);
        let handle_object = e.call(FIRST_WORD, &args![handle]).u32();
        if handle_object != 0 && e.call(BODY_PART_NODE_NAME, &args![handle_object]).u32() != 0 {
            let memory = e.call(ALLOCATE, &args![0xcu32]).u32();
            let clone = if memory != 0 {
                let handle_object = e.call(FIRST_WORD, &args![handle]).u32();
                let name = e.call(BODY_PART_NODE_NAME, &args![handle_object]).u32();
                e.call(
                    PROJECTILE_CLONE_CONSTRUCT,
                    &args![memory, name, count as u32],
                )
                .u32()
            } else {
                0
            };
            e.call(CLONE_RESERVER_ASSIGN, &args![p + 0x120, clone]);
        }
        e.call(HANDLE_DESTRUCT, &args![handle]);
    });
}

// Translated from 0092bf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ClearShootingAction` (Xbox PDB): `(process, actor)`.
/// Clears the actor's look-at target (`008b3d30`), runs slot +0x434 with 0
/// and clears the byte at +0xE1.
pub fn middle_high_process_clear_shooting_action(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    let p = this.addr();
    e.call(ACTOR_CLEAR_LOOK_AT_TARGET, &args![actor]);
    e.vcall(p, 0x434, &args![0u32]);
    e.mem.set_u8(p + 0xe1, 0);
}

// Translated from 0092bf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetLightingShaderProperty` (Xbox PDB): `(process,
/// property)`. Assigns the lighting shader property pointer at +0x220.
pub fn middle_high_process_set_lighting_shader_property(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    property: u32,
) {
    let p = this.addr();
    e.call(POINTER_ASSIGN, &args![p + 0x220, property]);
}

// Translated from 0092bf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::IsPackageDone` (Xbox PDB): `(process)`. True without
/// a current package (slot +0x27C); otherwise true when the package has a
/// procedure type (`009611e0`, not -1) whose procedure table entry for the
/// current procedure index (slot +0x280) is 0x36.
/// The stack word (`RET 4`; the callers push the actor) is not read.
pub fn middle_high_process_is_package_done(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    _unused_1: u32,
) -> bool {
    let p = this.addr();
    let package = e.vcall(p, 0x27c, &[]).u32();
    if package == 0 {
        return true;
    }
    let kind = e.call(PACKAGE_WORD_18, &args![package]).u32();
    if kind != u32::MAX {
        let kind = e.call(PACKAGE_WORD_18, &args![package]).u32();
        let index = e.vcall(p, 0x280, &[]).u32();
        let table = e
            .mem
            .u32(PROCEDURE_TABLES.wrapping_add(kind.wrapping_mul(4)));
        if e.mem.u32(table.wrapping_add(index.wrapping_mul(4))) == 0x36 {
            return true;
        }
    }
    false
}

/// `(this)`: the constructor of a hide-dismembered-limb entry
/// (`0092c0c0`, 0x14 bytes, in the next part of this unit).
const DEFERRED_HIDE_CONSTRUCT: u32 = 0x0092_c0c0;

// Translated from 0092bff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0092bff0 - the exception frame (FS:[0]) is not translated
/// `MiddleHighProcess::DeferHideDismemberedLimb` (Xbox PDB): `(process, a,
/// b, c, d)`. Builds a 0x14 byte entry (`0092c0c0`): float at +0 set to 0.0,
/// `a` at +4, `b` and `c` assigned to the pointers at +8 and +0xC, byte `d`
/// at +0x10; the entry is appended to the list at +0x1B0.
pub fn middle_high_process_defer_hide_dismembered_limb(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    a: u32,
    b: u32,
    c: u32,
    d: u8,
) {
    let p = this.addr();
    let memory = e.call(ALLOCATE, &args![0x14u32]).u32();
    let entry = if memory != 0 {
        e.call(DEFERRED_HIDE_CONSTRUCT, &args![memory]).u32()
    } else {
        0
    };
    e.mem.set_f32(entry, 0.0);
    e.mem.set_u8(entry + 0x10, d);
    e.mem.set_u32(entry + 4, a);
    e.call(POINTER_ASSIGN, &args![entry + 8, b]);
    e.call(POINTER_ASSIGN, &args![entry + 0xc, c]);
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), entry);
        e.call(LIST_ADD, &args![p + 0x1b0, cell.addr()]);
    });
}

// Translated from 0092c0c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0092c0c0 - the exception frame (FS:[0]) is not translated
/// The constructor of a hide-dismembered-limb entry (no Xbox PDB name):
/// initialises the pointers at +8 and +0xC to null through the pointer
/// constructor (`00633c90`) and returns the entry. The float at +0, the
/// word at +4 and the byte at +0x10 are not touched.
pub fn fn_0092c0c0(e: &mut Engine, this: u32) -> u32 {
    e.call(POINTER_INIT, &args![this + 8, 0u32]);
    e.call(POINTER_INIT, &args![this + 0xc, 0u32]);
    this
}

/// `(vats)`: the VATS object's word at +8, its mode (`0044ddc0`).
const VATS_MODE: u32 = 0x0044_ddc0;
/// The float game setting object `UpdateDeferredHideDismemberedLimbs`
/// reads while the VATS mode is 4.
const HIDE_DELAY_SETTING_VATS: u32 = 0x011c_f9e0;
/// The float game setting object it reads otherwise.
const HIDE_DELAY_SETTING: u32 = 0x011c_f338;
/// `Actor::FillBoneArray` (Xbox PDB), `(actor, first word of the entry's
/// +8 pointer, 0)`.
const ACTOR_FILL_BONE_ARRAY: u32 = 0x008b_6ff0;
/// `Actor::HideDismemberedLimb` (Xbox PDB), `(actor, entry word +4, the
/// +8 pointer's first word, the +0xC pointer's first word, entry byte
/// +0x10)`.
const ACTOR_HIDE_DISMEMBERED_LIMB: u32 = 0x008b_6290;

// Translated from 0092c120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::UpdateDeferredHideDismemberedLimbs` (Xbox PDB):
/// `(process, actor, delta)`. Walks the list at +0x1B0 of entries made by
/// `DeferHideDismemberedLimb`. An entry whose +8 pointer is null or points
/// at something whose second word is at most 1 (or likewise for its +0xC
/// pointer) is finished; otherwise, once its float at +0 (grown by `delta`)
/// exceeds the game setting (the VATS-mode setting while the VATS object's
/// mode is 4), the actor's bone array is filled and the limb hidden, and
/// the entry is finished. A finished entry is deleted and removed from the
/// list.
pub fn middle_high_process_update_deferred_hide_dismembered_limbs(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    delta: f32,
) {
    let mode = e.call(VATS_MODE, &args![VATS]).u32();
    let setting = if mode == 4 {
        HIDE_DELAY_SETTING_VATS
    } else {
        HIDE_DELAY_SETTING
    };
    let slot = e.call(FLOAT_SETTING_VALUE, &args![setting]).u32();
    let threshold = x87_quiet(e.mem.f32(slot));
    let mut list = this.addr() + 0x1b0;
    let mut previous = 0u32;
    loop {
        if list == 0 || e.call(LIST_IS_EMPTY, &args![list]).bool() {
            return;
        }
        let item = e.call(IDENTITY, &args![list]).u32();
        let entry = e.mem.u32(item);
        let mut finished = false;
        if e.call(FIRST_WORD, &args![entry + 8]).u32() == 0 {
            finished = true;
        } else {
            let first = e.call(FIRST_WORD, &args![entry + 8]).u32();
            if e.call(WORD_4, &args![first]).u32() <= 1 {
                finished = true;
            } else if e.call(FIRST_WORD, &args![entry + 0xc]).u32() != 0 {
                let second = e.call(FIRST_WORD, &args![entry + 0xc]).u32();
                if e.call(WORD_4, &args![second]).u32() <= 1 {
                    finished = true;
                }
            }
        }
        let grown = (f64::from(x87_quiet(e.mem.f32(entry))) + f64::from(x87_quiet(delta))) as f32;
        e.mem.set_f32(entry, grown);
        // `FCOMPP` of the setting against the entry's float, then the
        // parity test: the body runs only when the setting is below it.
        if !finished && f64::from(threshold) < f64::from(e.mem.f32(entry)) {
            let bones = e.call(FIRST_WORD, &args![entry + 8]).u32();
            e.call(ACTOR_FILL_BONE_ARRAY, &args![actor, bones, 0u32]);
            let flag = u32::from(e.mem.u8(entry + 0x10));
            let second = e.call(FIRST_WORD, &args![entry + 0xc]).u32();
            let first = e.call(FIRST_WORD, &args![entry + 8]).u32();
            let word = e.mem.u32(entry + 4);
            e.call(
                ACTOR_HIDE_DISMEMBERED_LIMB,
                &args![actor, word, first, second, flag],
            );
            finished = true;
        }
        if !finished {
            previous = list;
            list = e.call(WORD_4, &args![list]).u32();
        } else {
            if entry != 0 {
                e.call(DEFERRED_HIDE_LIMB_DELETE, &args![entry, 1u32]);
            }
            if previous == 0 {
                e.call(LIST_REMOVE_HEAD, &args![list]);
            } else {
                e.with_stack(4, |e, cell| {
                    e.mem.set_u32(cell.addr(), entry);
                    e.call(LIST_REMOVE, &args![previous, cell.addr()]);
                });
                list = e.call(WORD_4, &args![previous]).u32();
            }
        }
    }
}

// Translated from 0092c2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetCreatureLipSynchAnim` (Xbox PDB): deletes the
/// current lip-synch animation at +0x1A0 (its scalar deleting destructor)
/// and stores the new one.
pub fn middle_high_process_set_creature_lip_synch_anim(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    anim: u32,
) {
    let p = this.addr();
    let current = e.mem.u32(p + 0x1a0);
    if current != 0 {
        e.call(LIP_SYNCH_ANIM_DELETE, &args![current, 1u32]);
    }
    e.mem.set_u32(p + 0x1a0, anim);
}

/// `(object)`: true when `0082_56d0` (`LIST_IS_EMPTY`) of the object is
/// false (`00493830`).
const NOT_EMPTY: u32 = 0x0049_3830;

// Translated from 0092c340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::HasMovementAnim` (Xbox PDB): `(process, group)`.
/// Without the object at +0x1C0, or when `00493830` of it is true, or for
/// a group outside the cached range, the answer is yes. Otherwise a
/// two-bit cache at +0x22C per group (1 loaded, 2 not loaded) answers,
/// and a miss asks `Animation::GroupLoaded` of that object and records it.
pub fn middle_high_process_has_movement_anim(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    group: u32,
) -> bool {
    let p = this.addr();
    let animation = e.mem.u32(p + 0x1c0);
    if animation == 0 || e.call(NOT_EMPTY, &args![animation]).eax as u8 != 0 {
        return true;
    }
    let shift = group.wrapping_mul(2).wrapping_sub(6);
    if shift > 0x20 {
        return true;
    }
    let cached = e.mem.u32(p + 0x22c) >> (shift & 31);
    if cached & 1 != 0 {
        return true;
    }
    if cached & 2 != 0 {
        return false;
    }
    let loaded = e
        .call(ANIMATION_GROUP_LOADED, &args![animation, group & 0xffff])
        .eax as u8;
    let state: u32 = if loaded != 0 { 1 } else { 2 };
    let flags = (state << (shift & 31)) | e.mem.u32(p + 0x22c);
    e.mem.set_u32(p + 0x22c, flags);
    loaded != 0
}

/// `HitData::HitData` (Xbox PDB), `(hit data)`.
const HIT_DATA_CONSTRUCT: u32 = 0x009b_4d90;
/// `HitData::Copy` (Xbox PDB), `(hit data, source)`.
const HIT_DATA_COPY: u32 = 0x009b_4ec0;
/// `HitData::ClearData` (Xbox PDB), `(hit data)`.
const HIT_DATA_CLEAR: u32 = 0x009b_4dc0;

/// Makes the `HitData` (0x64 bytes) at `process + offset` when it does not
/// exist yet; returns whether it was made.
fn make_hit_data(e: &mut Engine, process: u32, offset: u32) -> bool {
    if e.mem.u32(process + offset) != 0 {
        return false;
    }
    let block = e.call(ALLOCATE, &args![0x64u32]).u32();
    let hit = if block != 0 {
        e.call(HIT_DATA_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.mem.set_u32(process + offset, hit);
    true
}

// Translated from 0092c400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0092c400 - the exception frame (FS:[0]) is not translated
/// `MiddleHighProcess::SetLastHitData` (Xbox PDB): `(process, hit)`. Makes
/// the `HitData` at +0x240 when missing, then copies `hit` into it.
pub fn middle_high_process_set_last_hit_data(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    hit: u32,
) {
    let p = this.addr();
    make_hit_data(e, p, 0x240);
    let data = e.mem.u32(p + 0x240);
    e.call(HIT_DATA_COPY, &args![data, hit]);
}

// Translated from 0092c4a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0092c4a0 - the exception frame (FS:[0]) is not translated
/// `MiddleHighProcess::ClearLastHitData` (Xbox PDB): makes the `HitData` at
/// +0x240 when missing (a fresh one is left as constructed), otherwise
/// clears it.
pub fn middle_high_process_clear_last_hit_data(e: &mut Engine, this: Ptr<MiddleHighProcess>) {
    let p = this.addr();
    if !make_hit_data(e, p, 0x240) {
        let data = e.mem.u32(p + 0x240);
        e.call(HIT_DATA_CLEAR, &args![data]);
    }
}

// Translated from 0092c540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0092c540 - the exception frame (FS:[0]) is not translated
/// `MiddleHighProcess::SetLastAttackHitData` (Xbox PDB): as
/// `SetLastHitData` for the `HitData` at +0x254.
pub fn middle_high_process_set_last_attack_hit_data(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    hit: u32,
) {
    let p = this.addr();
    make_hit_data(e, p, 0x254);
    let data = e.mem.u32(p + 0x254);
    e.call(HIT_DATA_COPY, &args![data, hit]);
}

// Translated from 0092c5e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0092c5e0 - the exception frame (FS:[0]) is not translated
/// `MiddleHighProcess::ClearLastAttackHitData` (Xbox PDB): as
/// `ClearLastHitData` for the `HitData` at +0x254.
pub fn middle_high_process_clear_last_attack_hit_data(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
) {
    let p = this.addr();
    if !make_hit_data(e, p, 0x254) {
        let data = e.mem.u32(p + 0x254);
        e.call(HIT_DATA_CLEAR, &args![data]);
    }
}

// Translated from 0092c680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ClearFurnitureMarker` (Xbox PDB): unless the
/// save/load object's flag (`0042ce10`) or the data handler's byte
/// (`004226e0`) is set, un-reserves the marker slot (byte +0x144) of the
/// furniture reference at +0x140; then resets the slot to 0x7F, the
/// furniture mark at +0x148 to heading 0.0, its byte +0xE to 0 and its
/// origin point.
pub fn middle_high_process_clear_furniture_marker(e: &mut Engine, this: Ptr<MiddleHighProcess>) {
    let p = this.addr();
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    if e.call(SAVE_LOAD_FLAG_2, &args![save_load]).eax as u8 == 0 && e.mem.u32(p + 0x140) != 0 {
        let handler = e.global::<u32>(DATA_HANDLER);
        if e.call(DATA_HANDLER_BYTE_61D, &args![handler]).eax as u8 == 0 {
            let reference = e.mem.u32(p + 0x140);
            let slot = u32::from(e.mem.u8(p + 0x144));
            e.call(SET_MARKER_RESERVED, &args![reference, slot, 0u32]);
        }
    }
    e.mem.set_u8(p + 0x144, 0x7f);
    e.call(FURNITURE_MARK_SET_HEADING, &args![p + 0x148, 0u32]);
    e.call(SET_BYTE_E, &args![p + 0x148, 0u32]);
    e.call(COPY_POINT, &args![p + 0x148, FURNITURE_MARK_ORIGIN]);
}

/// `(this, mask, value)` (actor.cpp): sets or clears the flag `mask`
/// according to `value` when the word at +0x2C of `this` is non-null.
const CACHED_FLAG_SET: u32 = 0x008a_cf70;
/// `(extra data list)`: a byte (extradatalist.cpp).
const EXTRA_LIST_BYTE: u32 = 0x0041_8a80;

// Translated from 0092c720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::CheckCachedValueFlags` (Xbox PDB): `(this, actor)`.
/// Sets flag 0x20000000 of `this` to whether the value 0x10 read through
/// slot +0x20 of the object at the actor's +0xA4 exceeds the one read
/// through slot +0xC (each a float in `ST0`), flag 0x40000000 the same for
/// the value 0x16, and flag 0x10000000 from `00418a80` of the actor's
/// extra data list (`005d43c0`), all through `008acf70`.
pub fn middle_high_process_check_cached_value_flags(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    let p = this.addr();
    let part = actor + 0xa4;
    let base_10 = e.vcall(part, 0xc, &args![0x10u32]).f64();
    let current_10 = e.vcall(part, 0x20, &args![0x10u32]).f64();
    let low = current_10 > base_10;
    let base_16 = e.vcall(part, 0xc, &args![0x16u32]).f64();
    let current_16 = e.vcall(part, 0x20, &args![0x16u32]).f64();
    let high = current_16 > base_16;
    e.call(CACHED_FLAG_SET, &args![p, 0x2000_0000u32, u32::from(low)]);
    e.call(CACHED_FLAG_SET, &args![p, 0x4000_0000u32, u32::from(high)]);
    let list = e.call(REFERENCE_EXTRA_LIST, &args![actor]).u32();
    let flag = e.call(EXTRA_LIST_BYTE, &args![list]).eax as u8;
    e.call(CACHED_FLAG_SET, &args![p, 0x1000_0000u32, u32::from(flag)]);
}

/// `(referent)`: releases the object a reference cell pointed at
/// (`004b0ef0`).
const CELL_RELEASE: u32 = 0x004b_0ef0;
/// `(referent)`: takes a reference to the object (`0092c870`: passes
/// `referent + 8` to `0040b460`).
const CELL_ADD_REFERENCE: u32 = 0x0092_c870;

// Translated from 0092c890 (decompiled, FalloutNV.exe 1.4.0.525)
/// A reference cell's release (no Xbox PDB name): `(cell)`; releases what
/// the cell holds, if anything.
pub fn fn_0092c890(e: &mut Engine, this: u32) {
    let held = e.mem.u32(this);
    if held != 0 {
        e.call(CELL_RELEASE, &args![held]);
    }
}

// Translated from 0092c8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A reference cell's assignment (no Xbox PDB name): `(cell, value)`.
/// Releases the old referent, stores `value` and takes a reference to
/// it; returns the cell.
pub fn fn_0092c8b0(e: &mut Engine, this: u32, value: u32) -> u32 {
    if e.mem.u32(this) != value {
        let held = e.mem.u32(this);
        if held != 0 {
            e.call(CELL_RELEASE, &args![held]);
        }
        e.mem.set_u32(this, value);
        let held = e.mem.u32(this);
        if held != 0 {
            e.call(CELL_ADD_REFERENCE, &args![held]);
        }
    }
    this
}

// Translated from 0092c900 (decompiled, FalloutNV.exe 1.4.0.525)
/// A reference cell's copy assignment (no Xbox PDB name): `(cell, source
/// cell)`. As `fn_0092c8b0` with the source's referent, read again after
/// the release as the exe does; returns the cell.
pub fn fn_0092c900(e: &mut Engine, this: u32, source: u32) -> u32 {
    if e.mem.u32(this) != e.mem.u32(source) {
        let held = e.mem.u32(this);
        if held != 0 {
            e.call(CELL_RELEASE, &args![held]);
        }
        let value = e.mem.u32(source);
        e.mem.set_u32(this, value);
        let held = e.mem.u32(this);
        if held != 0 {
            e.call(CELL_ADD_REFERENCE, &args![held]);
        }
    }
    this
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    type Process = Ptr<MiddleHighProcess>;
    vec![
        entry!(
            0x00924b90,
            middle_high_process_load_game(Process, u32, u32, u32)
        ),
        entry!(
            0x00925490,
            middle_high_process_init_load_game(Process, u32, u32, u32)
        ),
        entry!(
            0x00925700,
            middle_high_process_finish_init_load_game(Process, u32, u32, u32)
        ),
        entry!(0x00926000, middle_high_process_revert(Process, u32, u32)),
        entry!(0x00926350, middle_high_process_get_char_controller_save_size(Process, u32) -> u16),
        entry!(
            0x00926490,
            middle_high_process_save_char_controller(Process, u32)
        ),
        entry!(0x00926690, fn_00926690(Process, u32)),
        entry!(0x00926980, fn_00926980(u32, u32)),
        entry!(0x009269f0, fn_009269f0(u32) -> bool),
        entry!(0x00926a20, middle_high_process_save_game_ov2(Process, u32)),
        entry!(0x00927000, middle_high_process_load_game_ov2(Process, u32)),
        entry!(
            0x009278d0,
            middle_high_process_init_load_game_ov2(Process, u32)
        ),
        entry!(
            0x00927d70,
            middle_high_process_finish_load_game(Process, u32)
        ),
        entry!(0x009280f0, middle_high_process_revert_ov2(Process, u32)),
        entry!(
            0x00928510,
            middle_high_process_load_sit_sleep_idle(Process, u32, u32)
        ),
        entry!(
            0x009287e0,
            middle_high_process_clear_furniture(Process, u32)
        ),
        entry!(
            0x00928880,
            middle_high_process_save_char_controller_ov2(Process, u32)
        ),
        entry!(
            0x00928a50,
            middle_high_process_load_char_controller(Process, u32)
        ),
        entry!(0x00928d00, fn_00928d00(u32, u32)),
        entry!(
            0x00928d20,
            middle_high_process_set_should_refresh_magic_shaders_effects(Process, u8, u8, u8)
        ),
        entry!(
            0x00928d90,
            middle_high_process_refresh_magic_shader_effects(Process, u32)
        ),
        entry!(0x00929220, fn_00929220(u32, u32)),
        entry!(0x00929240, fn_00929240(u32, f32)),
        entry!(0x00929260, fn_00929260(u32, u8)),
        entry!(0x00929280, middle_high_process_get_face_animation_data(Process, u32) -> u32),
        entry!(0x009292e0, middle_high_process_should_check_flare(Process, u32, u32) -> bool),
        entry!(0x00929390, fn_00929390(u32, u32) -> bool),
        entry!(
            0x009293b0,
            middle_high_process_add_attached_arrow(Process, u32)
        ),
        entry!(
            0x00929470,
            middle_high_process_remove_attached_arrow(Process, u32)
        ),
        entry!(
            0x009294b0,
            middle_high_process_move_attached_arrows_to_parent_cell(Process, u32)
        ),
        entry!(
            0x00929570,
            middle_high_process_set_aim_sequence(Process, u32, u32)
        ),
        entry!(
            0x009295c0,
            middle_high_process_update_aim_sequences(Process, u32)
        ),
        entry!(0x00929820, fn_00929820(Process, u32) -> bool),
        entry!(0x00929960, fn_00929960(Process, u32, u8)),
        entry!(0x00929d20, fn_00929d20(u32) -> bool),
        entry!(
            0x00929d40,
            middle_high_process_clear_automatic_shots_to_fire(Process, u8)
        ),
        entry!(0x00929df0, middle_high_process_get_automatic_shots_to_fire(Process, u32) -> u8),
        entry!(
            0x00929fc0,
            middle_high_process_process_sand_box(Process, u32)
        ),
        entry!(0x0092a980, fn_0092a980(u32) -> bool),
        entry!(0x0092a9b0, middle_high_process_process_use_furniture(Process, u32, u32) -> bool),
        entry!(
            0x0092ac10,
            middle_high_process_process_movement_blocked(Process, u32)
        ),
        entry!(0x0092acd0, middle_high_process_process_patrol(Process, u32)),
        entry!(0x0092adb0, middle_high_process_process_patrol_init(Process, u32, u32) -> bool),
        entry!(
            0x0092ae20,
            middle_high_process_process_patrol_start_travel_to_next_ref(Process, u32, u32)
        ),
        entry!(
            0x0092b030,
            middle_high_process_process_patrol_traveling_to_next_ref(Process, u32, u32)
        ),
        entry!(0x0092b440, fn_0092b440(u32) -> u32),
        entry!(
            0x0092b460,
            middle_high_process_process_patrol_start_at_ref_behavior(Process, u32, u32)
        ),
        entry!(0x0092b550, fn_0092b550(u32) -> u32),
        entry!(
            0x0092b570,
            middle_high_process_process_patrol_at_ref(Process, u32, u32)
        ),
        entry!(0x0092b7f0, fn_0092b7f0(Process)),
        entry!(
            0x0092b830,
            middle_high_process_store_damage_nodes(Process, u32, u32)
        ),
        entry!(
            0x0092b970,
            middle_high_process_store_dismembered_limb_node(Process, u32, u32)
        ),
        entry!(0x0092b9c0, middle_high_process_get_damage_node(Process, u32) -> u32),
        entry!(0x0092b9e0, middle_high_process_get_weapon_condition(Process, u32) -> u32),
        entry!(
            0x0092baa0,
            middle_high_process_update_armor_worn(Process, u32)
        ),
        entry!(
            0x0092bd00,
            middle_high_process_try_reserve_projectile_clones(Process, u32)
        ),
        entry!(
            0x0092bf20,
            middle_high_process_clear_shooting_action(Process, u32)
        ),
        entry!(
            0x0092bf60,
            middle_high_process_set_lighting_shader_property(Process, u32)
        ),
        entry!(0x0092bf80, middle_high_process_is_package_done(Process, u32) -> bool),
        entry!(
            0x0092bff0,
            middle_high_process_defer_hide_dismembered_limb(Process, u32, u32, u32, u8)
        ),
        entry!(0x0092c0c0, fn_0092c0c0(u32) -> u32),
        entry!(
            0x0092c120,
            middle_high_process_update_deferred_hide_dismembered_limbs(Process, u32, f32)
        ),
        entry!(
            0x0092c2e0,
            middle_high_process_set_creature_lip_synch_anim(Process, u32)
        ),
        entry!(0x0092c340, middle_high_process_has_movement_anim(Process, u32) -> bool),
        entry!(
            0x0092c400,
            middle_high_process_set_last_hit_data(Process, u32)
        ),
        entry!(0x0092c4a0, middle_high_process_clear_last_hit_data(Process)),
        entry!(
            0x0092c540,
            middle_high_process_set_last_attack_hit_data(Process, u32)
        ),
        entry!(
            0x0092c5e0,
            middle_high_process_clear_last_attack_hit_data(Process)
        ),
        entry!(
            0x0092c680,
            middle_high_process_clear_furniture_marker(Process)
        ),
        entry!(
            0x0092c720,
            middle_high_process_check_cached_value_flags(Process, u32)
        ),
        entry!(0x0092c890, fn_0092c890(u32)),
        entry!(0x0092c8b0, fn_0092c8b0(u32, u32) -> u32),
        entry!(0x0092c900, fn_0092c900(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fake vtable targets: slot `n` of a test process, actor or other
    /// object calls `<base> + n`.
    const PROCESS_SLOTS: u32 = 0x0300_0000;
    const ACTOR_SLOTS: u32 = 0x0310_0000;
    const OBJECT_SLOTS: u32 = 0x0320_0000;
    const VTABLE_SIZE: u32 = 0x900;

    /// Every exe function this file calls by address.
    const CALLEES: &[u32] = &[
        ACTOR_FILL_BONE_ARRAY,
        ACTOR_HIDE_DISMEMBERED_LIMB,
        ANIMATION_GROUP_LOADED,
        CACHED_FLAG_SET,
        CELL_ADD_REFERENCE,
        CELL_RELEASE,
        COPY_POINT,
        DATA_HANDLER_BYTE_61D,
        DEFERRED_HIDE_LIMB_DELETE,
        EXTRA_LIST_BYTE,
        FLOAT_SETTING_VALUE,
        FURNITURE_MARK_SET_HEADING,
        HIT_DATA_CLEAR,
        HIT_DATA_CONSTRUCT,
        HIT_DATA_COPY,
        LIP_SYNCH_ANIM_DELETE,
        LIST_REMOVE,
        LIST_REMOVE_HEAD,
        NOT_EMPTY,
        POINTER_INIT,
        REFERENCE_EXTRA_LIST,
        SAVE_LOAD_FLAG_2,
        SET_BYTE_E,
        SET_MARKER_RESERVED,
        VATS_MODE,
        ACTIVE_EFFECTS_FINISH_LOAD,
        ACTIVE_EFFECTS_FINISH_LOAD_OV2,
        ACTIVE_EFFECTS_INIT_LOAD,
        ACTIVE_EFFECTS_INIT_LOAD_OV2,
        ACTIVE_EFFECTS_LOAD,
        ACTIVE_EFFECTS_LOAD_OV2,
        ACTIVE_EFFECTS_REVERT,
        ACTIVE_EFFECTS_REVERT_OV2,
        ACTIVE_EFFECTS_SAVE,
        ACTOR_APPLY_FURNITURE_STATE,
        ACTOR_BUILD_REQUEST,
        ACTOR_BYTE_70,
        ACTOR_CLEAR_LOOK_AT_TARGET,
        ACTOR_EQUIP_WORN,
        ACTOR_EXTRA_OBJECT,
        ACTOR_GET_ANIMATION,
        ACTOR_GET_FACE_ANIMATION_DATA,
        ACTOR_INIT_LIGHTING_PROPERTY,
        ACTOR_IS_PATHING,
        ACTOR_IS_PATHING_COMPLETE,
        ACTOR_IS_WEAPON_DRAWN,
        ACTOR_PACKAGE_FINISH_LOAD_GAME,
        ACTOR_PACKAGE_INIT_LOAD_GAME_OV2,
        ACTOR_PACKAGE_LOAD_GAME_OV2,
        ACTOR_PACKAGE_REVERT_OV2,
        ACTOR_PACKAGE_SAVE_GAME,
        ACTOR_PLAYING_LOWER_BODY_IDLE,
        ACTOR_RATE_SCALE,
        ACTOR_RELOAD_TARGETS,
        ACTOR_SET_GOAL_FROM_REQUEST,
        ACTOR_STOP_SPECIAL_IDLE,
        ACTOR_WORD_1B4,
        ADD_FORM_TO_DEFERRED_DELETIONS,
        ALLOCATE,
        ANIMATION_LOAD_BLOCK,
        ANIMATION_LOCK_RELEASE,
        ANIMATION_LOCK_TEST,
        ANIMATION_REVERT,
        ANIMATION_REVERT_OV2,
        ANIMATION_SAVE_GAME,
        BASE_FINISH_INIT_LOAD_GAME,
        BASE_FINISH_LOAD_GAME,
        BIPED_FILLS_SLOT,
        BIPED_FLAG_4,
        BIPED_FLAG_80,
        BIPED_IS_POWER_ARMOR,
        BODY_PART_DATA_GET_BODY_PART,
        BODY_PART_NODE_NAME,
        BUFFER_FLAGS_B,
        BUFFER_POSITION,
        BUFFER_SAVE_KIND,
        CALENDAR_GET_HOUR,
        CELL_FLAG,
        CELL_MOVE_REFERENCE,
        CLEAR_FURNITURE_MARKER,
        CLONE_RESERVER_ASSIGN,
        CONSTRUCT_QUATERNION,
        CONTROLLER_APPLY_VALUE,
        CONTROLLER_CLEAR_STATE_FLAGS,
        CONTROLLER_GET_LINEAR_VELOCITY,
        CONTROLLER_GET_POSITION,
        CONTROLLER_GET_PROXY,
        CONTROLLER_IS_IN_WATER,
        CONTROLLER_NEXT_VALUE,
        CONTROLLER_SAVED_WORD,
        CONTROLLER_SET_MOVE_MODE,
        CONTROLLER_SET_POSITION,
        CONTROLLER_SET_STATE_FLAGS,
        CONTROLLER_SET_VALUE_450,
        CONTROLLER_STATE_FLAGS,
        COPY_POINT,
        CREATURE_INIT_DEFAULT_WORN,
        CURRENT_VERSION,
        DEFERRED_HIDE_CONSTRUCT,
        DEFERRED_HIDE_LIMB_DELETE,
        DYNAMIC_CAST,
        EFFECT_ALLOCATE,
        EFFECT_SETTING_FORM,
        PROJECTILE_GET_PRE_CLONE_COUNT,
        FLOAT_AT_START,
        EXTRA_PACKAGE_TYPE,
        EXTRA_LIST_TYPE_6F_WORD,
        FIND_NODE,
        FIRST_WORD,
        FLOAT_ABS,
        FLOAT_ABS_WRAPPER,
        FLOAT_C,
        FLOAT_ROUND,
        FLOAT_SETTING_VALUE,
        FORM_BASE_RATE,
        FORM_ENCHANTING,
        FORM_PROJECTILE,
        FORM_IS_AUTOMATIC,
        FORM_TYPE,
        FORM_TYPE_27,
        FTOL,
        FURNITURE_CAN_SIT_ON,
        FURNITURE_GET_MARKER_TARGET_OFFSET,
        FURNITURE_MARKER_ANGLE,
        FURNITURE_MARKER_FIND,
        FURNITURE_MARK_SET_HEADING,
        GET_CHAR_CONTROLLER,
        GET_CONTAINER_CHANGES,
        GET_IDLES_FOR_ACTOR,
        GET_IDLE_TO_PLAY,
        GET_INVENTORY_CHANGES,
        GET_PACKAGE_EXTRA,
        GET_WORN_ITEM,
        HANDLE_DESTRUCT,
        IDENTITY,
        IDLE_ANIM_GROUP_SECTION,
        IDLE_ARRAY_CONSTRUCT,
        IDLE_ARRAY_DESTRUCT,
        IDLE_ARRAY_ELEMENT,
        IDLE_COLLECTION_MODE,
        INDEXED_POINTER,
        INVENTORY_GET_OBJECT_COUNT,
        ITEM_CHANGE_FORM,
        ITEM_CHANGE_GET_ITEM_HEALTH,
        ITEM_CHANGE_GET_WORN,
        ITEM_CHANGE_MOD_EFFECT_ACTIVE,
        ITEM_CHANGE_SET_COUNT,
        ITEM_CHARGE,
        KIND_HAS_BITS,
        LIGHT_FLARE_BITS,
        LIGHT_FLARE_VALUE,
        LIST_ADD,
        LIST_CLEAR,
        LIST_CONSTRUCT,
        LIST_CONTAINS,
        LIST_DELETE,
        LIST_IS_EMPTY,
        LIST_REMOVE,
        LIST_REMOVE_HEAD,
        LOAD_DATA,
        LOAD_FORM_ID,
        LOAD_FORM_ID_INTO,
        LOAD_VARIABLE_SIZED_VALUE,
        LOOKUP_FORM,
        LOW_INIT_LOAD_GAME,
        LOW_INIT_LOAD_GAME_OV2,
        LOW_PROCESS_PATROL_START_AT_REF,
        MAGIC_ITEM_BY_NUMERIC_ID,
        MAGIC_ITEM_FORM_ID,
        MAGIC_ITEM_GET_AV_EFFECT,
        MAGIC_TARGET_BY_NUMERIC_ID,
        MAGIC_TARGET_FORM_ID,
        MAKE_HAVOK_VECTOR,
        MARKER_COUNT,
        MARKER_ENTRY,
        MARK_BYTE_E,
        MARK_HEADING,
        MAX_OF_FLOATS,
        MEMSET,
        MIDDLE_LOW_LOAD_GAME,
        MIDDLE_LOW_LOAD_GAME_OV2,
        MIDDLE_LOW_REVERT,
        MIDDLE_LOW_REVERT_OV2,
        MIDDLE_LOW_SAVE_GAME_OV2,
        MIN_OF_FLOATS,
        MOBILE_GET_LOOKING,
        MOBILE_TURN_BY,
        MOTION_VALUE_CONSTRUCT,
        NODE_NAME,
        NODE_SET_ITEM,
        NODE_SLOT,
        OBJECT_FLAG_4,
        PACKAGE_ALLOWS_SPEED,
        PACKAGE_GET_LOCATION_CELL,
        PACKAGE_GET_LOCATION_COORD,
        PACKAGE_GET_LOCATION_WORLD,
        PACKAGE_GET_RADIUS_TO_LOCATION,
        PACKAGE_IDLE_COLLECTION,
        PACKAGE_LAST_IDLE,
        PACKAGE_LOCATION,
        PACKAGE_RADIUS,
        PACKAGE_SET_IS_CREATED,
        PACKAGE_TYPE,
        PACKAGE_WORD_18,
        PACKAGE_WORD_30,
        PACKAGE_WORD_84,
        PATHING_REQUEST_CONSTRUCT,
        PATHING_REQUEST_RELEASE,
        PATHING_REQUEST_SET_FLAG,
        PATHING_REQUEST_SET_FLOAT,
        PATHING_SPEED_LIMIT,
        PATH_HANDLE_POINTER,
        PATH_LOCATION,
        PATH_LOCATION_CELL,
        PATH_LOCATION_POSITION,
        PATH_LOCATION_WORLDSPACE,
        PATH_PROGRESS,
        SCRIPT_RUN,
        PATROL_GET_CURRENT_REFERENCE,
        PATROL_INIT,
        PATROL_SET_NEXT_REFERENCE,
        PLAYER_CLEAR_FURNITURE,
        PLAYER_GET_3D,
        PLAYER_GET_ANIMATION,
        PLAYER_GET_BIPED,
        PLAYER_HAS_SHADER_SLOT,
        PLAYER_LIGHTING_TEST,
        PLAYER_SET_ANIMATION_STATE,
        PLAYER_SET_FIRST_PERSON,
        PLAYER_SHADER_EFFECT,
        POINTER_ASSIGN,
        POINTER_INIT,
        POINTER_RELEASE,
        POINT_ADD,
        POINT_SUBTRACT,
        PROCESS_BLOCKS_IDLE,
        PROCESS_FURNITURE_DROP,
        PROCESS_LISTS_ADD_TEMP_EFFECT,
        PROCESS_LISTS_FIND_EFFECT,
        PROCESS_LISTS_FIND_WEAPON_EFFECT,
        PROCESS_LISTS_REFERENCE_MOVED,
        PROCESS_PACKAGE,
        PROCESS_TIME_VALUE,
        PROJECTILE_CLONE_CONSTRUCT,
        PROXY_SET_LINEAR_VELOCITY,
        QUEUED_FILE_DONE,
        QUEUED_ITEM_CONSTRUCT,
        QUEUED_ITEM_DELETE,
        QUEUED_ITEM_LOAD_GAME,
        QUEUED_ITEM_PRELOAD,
        QUEUED_ITEM_SAVE_GAME,
        QUEUE_SUB_BUFFER,
        RAG_DOLL_DATA_CONSTRUCT,
        RAG_DOLL_DATA_DELETE,
        RAG_DOLL_DATA_LOAD,
        RAND,
        READ_BYTES,
        READ_NUMERIC_ID,
        REFERENCE_3D,
        REFERENCE_BASE,
        REFERENCE_CELL,
        REFERENCE_EXTRA_LIST,
        REFERENCE_FACE_ANIMATION_DATA,
        REFERENCE_GET_DISTANCE_FROM_REFERENCE,
        REFERENCE_GET_ORIENTATION,
        REFERENCE_GET_SCALE,
        REFERENCE_GET_WORLD_SPACE,
        REFERENCE_INIT_ANIMATION,
        REFERENCE_IS_FURNITURE,
        REFERENCE_NAME,
        REFERENCE_PLUS_24,
        REFERENCE_SET_MARKER_USED,
        ROTATE_POINT,
        ROTATION_FROM_HEADING,
        SANDBOX_AT_TARGET,
        SANDBOX_FORGET_TARGET,
        SANDBOX_MOVE_TO_NEXT,
        SANDBOX_PICK_PROCEDURE,
        SANDBOX_REACHED,
        SANDBOX_SHOULD_PICK,
        SANDBOX_START,
        SANDBOX_TARGET_DONE,
        SANDBOX_WORD_10,
        SANDBOX_WORD_3C,
        SAVELOAD_WARNING,
        SAVE_DATA,
        SAVE_FORM_ID,
        SAVE_FORM_ID_VALUE,
        SAVE_LOAD_FLAG_2,
        SAVE_LOAD_GAME_FLAG,
        SAVE_VARIABLE_SIZED_VALUE,
        SEQUENCE_SET_VALUE,
        SET_BYTE_E,
        SET_CHASE_BIP,
        SET_MARKER_RESERVED,
        SET_PROCESS_FLAG_40,
        SET_WEAPON_FLAGS,
        SHADER_EFFECT_CONSTRUCT,
        SKIP_BYTES,
        SPECIAL_IDLE_DONE_PLAYING,
        SPECIAL_IDLE_FREE,
        SPECIAL_IDLE_REPLACE,
        START_VARIABLE_SIZED_VALUE,
        MODEL_LOADER_FIND_MODEL,
        STRSTR,
        TRANSFORM_VERTICES,
        USE_SAVE_GAME_BLOCKS,
        VATS_PLAYER_UPDATE_MULT,
        VATS_TARGET_UPDATE_MULT,
        VECTOR_LENGTH_2D_SQUARED,
        VECTOR_LENGTH_VALUE,
        WEAPON_ATTACK_SPEED,
        WEAPON_ATTRIBUTE,
        WEAPON_FORM_FLAG,
        WORD_4,
        WORD_68,
        WORD_C,
        WORLD_ADJUST_FORCE_ADD_COUNT,
        WORLD_APPLY_MOTION_VALUE,
        WORLD_SET_MOTION,
        WRITE_BYTES,
    ];

    /// Every exe global this file reads or writes.
    const GLOBALS: &[u32] = &[
        DATA_HANDLER,
        ACTIVATE_FORM_A,
        AIM_SETTING_A,
        AIM_SETTING_B,
        AIM_SETTING_LEFT,
        AIM_SETTING_RIGHT,
        ANIMATION_LOCK,
        BAD_HEADER_FORMAT,
        BAD_HEADER_FORM_FORMAT,
        CALENDAR,
        CONDITION_SCALE,
        FURNITURE_MARK_ORIGIN,
        GLOBAL_TIME_DATA,
        IDLE_BLEND_RATE,
        IDLE_MANAGER,
        LILY_TEXT,
        MINUS_ONE,
        MODEL_LOADER,
        OVERRUN_FORMAT,
        OVERRUN_FORM_FORMAT,
        PATH_PROGRESS_LIMIT,
        PATROL_DEFAULT_RADIUS,
        PATROL_RUN_DISTANCE,
        PATROL_START_SPEED,
        PLAYER,
        PROCEDURE_TABLES,
        PROCESS_LISTS,
        RANDOM_IDLE_TIMER,
        SANDBOX_SLACK,
        SAVE_LOAD_GAME,
        SHADER_EFFECT_TIMER,
        SOURCE_FILE,
        TES_SAVE_LOAD_GAME,
        TYPE_ACTOR,
        TYPE_MOBILE_OBJECT,
        TYPE_SAVED_FORM_CLASS,
        TYPE_TES_BOUND_OBJECT,
        TYPE_TES_FORM,
        TYPE_TES_FURNITURE,
        TYPE_TES_OBJECT_REFR,
        UNDERRUN_FORMAT,
        UNDERRUN_FORM_FORMAT,
        VATS,
        VATS_TARGET,
    ];

    /// Where the tests put the player character.
    const TEST_PLAYER: u32 = 0x0600_0000;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ret_f(st0: f64) -> Ret {
        Ret {
            st0,
            ..Ret::default()
        }
    }

    /// Every callee of this file a no-op returning 0 (the trivial list and
    /// word getters behave as in the game), the fake vtable slots likewise,
    /// the globals mapped, calls logged.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for &addr in CALLEES {
            e.register(addr, |_, _| Ret::default());
        }
        for base in [PROCESS_SLOTS, ACTOR_SLOTS, OBJECT_SLOTS] {
            for slot in (0..VTABLE_SIZE).step_by(4) {
                e.register(base + slot, |_, _| Ret::default());
            }
        }
        e.register(IDENTITY, |_, a| ret(a[0]));
        e.register(WORD_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(ITEM_CHANGE_FORM, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(LIST_IS_EMPTY, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        for &addr in GLOBALS {
            e.map(addr, 8);
        }
        // Null pointers read zeros here (the game would fault); the tests
        // check calls, not those reads.
        e.map(0, 0x1000);
        e.map(TEST_PLAYER, 0x800);
        e.set_global(PLAYER, TEST_PLAYER);
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, size: u32, vtable: u32, base: u32) -> u32 {
        let slots: Vec<u32> = (0..VTABLE_SIZE).step_by(4).map(|s| base + s).collect();
        e.put_vtable(vtable, &slots);
        let p = e.mem.alloc(size);
        e.mem.set_u32(p, vtable);
        p
    }

    fn process(e: &mut Engine) -> Ptr<MiddleHighProcess> {
        Ptr::new(object(e, 0x260, 0x0500_0000, PROCESS_SLOTS))
    }

    fn actor(e: &mut Engine) -> u32 {
        object(e, 0x700, 0x0501_0000, ACTOR_SLOTS)
    }

    fn thing(e: &mut Engine) -> u32 {
        object(e, 0x100, 0x0502_0000, OBJECT_SLOTS)
    }

    /// The arguments of every logged call of `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Makes slot `offset` of the fake vtable at `base` answer `value`.
    fn slot_returns(e: &mut Engine, base: u32, offset: u32, value: u32) {
        e.register_double(base + offset, move |_, _| ret(value));
    }

    /// The save reader: `READ_BYTES` writes the next of `values` (then
    /// `default`) as little endian bytes, the numeric id reader writes
    /// 0x77, the save format version is `version`.
    fn save_reader(e: &mut Engine, values: &[u32], default: u32, version: u32) {
        let values = values.to_vec();
        let mut next = 0;
        e.register_double(READ_BYTES, move |e, a| {
            let value = values.get(next).copied().unwrap_or(default);
            next += 1;
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, (value >> (8 * (i % 4))) as u8);
            }
            Ret::default()
        });
        e.register(READ_NUMERIC_ID, |e, a| {
            e.mem.set_u32(a[1], 0x77);
            Ret::default()
        });
        e.register_double(CURRENT_VERSION, move |_, _| ret(version));
    }

    #[test]
    fn load_game_reads_the_fields_in_save_order() {
        let mut e = engine();
        save_reader(&mut e, &[], 2, 0x71);
        let p = process(&mut e);
        let object = actor(&mut e);
        e.register_double(REFERENCE_3D, |_, _| ret(0x5555));
        middle_high_process_load_game(&mut e, p, 0x20, 0, object);
        assert_eq!(
            calls(&e, MIDDLE_LOW_LOAD_GAME),
            vec![vec![p.addr(), 0x20, 0, object]]
        );
        let a = p.addr();
        assert_eq!(e.mem.u8(a + 0x134), 2);
        assert_eq!(e.mem.u8(a + 0x135), 2);
        assert_eq!(e.mem.u32(a + 0x174), 2);
        assert_eq!(e.mem.u32(a + 0x158), 0x77);
        assert_eq!(e.mem.u32(a + 0x140), 0x77);
        assert_eq!(e.mem.u8(a + 0x13c), 2);
        // Flag 0x20: slot +0x544, targets, lighting property.
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x544), vec![vec![a, object]]);
        assert_eq!(calls(&e, ACTOR_RELOAD_TARGETS), vec![vec![object, 0]]);
        assert_eq!(
            calls(&e, ACTOR_INIT_LIGHTING_PROPERTY),
            vec![vec![object, 0x5555]]
        );
        // The count of 2 ids for the list at +0xC8.
        let adds = calls(&e, LIST_ADD);
        assert_eq!(adds.len(), 2);
        assert!(adds.iter().all(|c| c[0] == a + 0xc8));
        assert!(calls(&e, PROCESS_SLOTS + 0x420).is_empty());
        assert_eq!(calls(&e, ACTIVE_EFFECTS_LOAD).len(), 1);

        // No furniture-mark byte: slot +0x420 runs, the player's lighting
        // property goes through the indexed pointer.
        let mut e = engine();
        let mut values = vec![2u32; 14];
        values.push(0);
        save_reader(&mut e, &values, 2, 0x71);
        let p = process(&mut e);
        e.register_double(REFERENCE_3D, |_, _| ret(0x5555));
        e.register_double(PLAYER_LIGHTING_TEST, |_, _| ret(0));
        e.register_double(INDEXED_POINTER, |_, _| ret(0x6666));
        middle_high_process_load_game(&mut e, p, 0x20, 0, TEST_PLAYER);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x420),
            vec![vec![p.addr(), TEST_PLAYER]]
        );
        assert_eq!(e.mem.u8(p.addr() + 0x13c), 0);
        assert_eq!(
            calls(&e, ACTOR_INIT_LIGHTING_PROPERTY),
            vec![vec![TEST_PLAYER, 0x6666]]
        );
    }

    #[test]
    fn init_load_game_turns_ids_into_pointers() {
        let mut e = engine();
        save_reader(&mut e, &[], 2, 0x71);
        e.register(LOOKUP_FORM, |_, a| ret(a[0] + 0x1000));
        e.register(MAGIC_ITEM_BY_NUMERIC_ID, |_, a| ret(a[0] + 0x2000));
        e.register(MAGIC_TARGET_BY_NUMERIC_ID, |_, a| ret(a[0] + 0x3000));
        let p = process(&mut e);
        let a = p.addr();
        let object = actor(&mut e);
        e.mem.set_u32(a + 0x158, 1);
        e.mem.set_u32(a + 0x140, 2);
        e.mem.set_u32(a + 0x160, 3);
        e.mem.set_u32(a + 0x164, 4);
        e.mem.set_u32(a + 0x1bc, 5);
        middle_high_process_init_load_game(&mut e, p, 1, 2, object);
        assert_eq!(calls(&e, LOW_INIT_LOAD_GAME), vec![vec![a, 1, 2, object]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x20c), vec![vec![a]]);
        assert_eq!(e.mem.u32(a + 0x158), 0x1001);
        assert_eq!(e.mem.u32(a + 0x140), 0x1002);
        assert_eq!(e.mem.u32(a + 0x160), 0x2003);
        assert_eq!(e.mem.u32(a + 0x164), 0x2004);
        assert_eq!(e.mem.u32(a + 0x1bc), 0x3005);
        assert_eq!(calls(&e, ACTIVE_EFFECTS_INIT_LOAD).len(), 1);

        // Before version 0x65 and 0x71 the id list and +0x164 stay as saved.
        let mut e = engine();
        save_reader(&mut e, &[], 2, 0x60);
        let p = process(&mut e);
        e.mem.set_u32(p.addr() + 0x164, 4);
        middle_high_process_init_load_game(&mut e, p, 0, 0, 0);
        assert_eq!(e.mem.u32(p.addr() + 0x164), 4);
        assert!(calls(&e, LIST_IS_EMPTY).is_empty());
    }

    #[test]
    fn finish_init_load_game_applies_the_saved_state() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let object = actor(&mut e);
        let animation = thing(&mut e);
        e.register_double(ACTOR_SLOTS + 0x1b8, move |_, _| ret(animation));
        e.register_double(ACTOR_SLOTS + 0x1e8, |_, _| ret(0x31));
        e.register_double(ACTOR_SLOTS + 0x1e4, |_, _| ret(0x32));
        e.register_double(PROCESS_SLOTS + 0x190, |_, _| ret(1));
        e.mem.set_u8(a + 0x13c, 3);
        e.mem.set_u8(a + 0x135, 1);
        e.mem.set_u8(a + 0x18b, 1);
        middle_high_process_finish_init_load_game(&mut e, p, 5, 6, object);
        assert_eq!(
            calls(&e, BASE_FINISH_INIT_LOAD_GAME),
            vec![vec![a, 5, 6, object]]
        );
        // State 3 is in 1..=4: slot +0xD8 gets (1, 0).
        assert_eq!(calls(&e, OBJECT_SLOTS + 0xd8), vec![vec![animation, 1, 0]]);
        // The process's slot +0x1CC for the actor: first, third, second, actor.
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x1cc),
            vec![vec![a, 1, 0x31, 0x32, object]]
        );
        assert_eq!(e.mem.u8(a + 0x180), 1);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x580), vec![vec![a, 1, 0, 0]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x7a8), vec![vec![a, object]]);
        assert_eq!(calls(&e, ACTIVE_EFFECTS_FINISH_LOAD).len(), 1);
        // +0x18B without an actor id (+0x158) defers the deletion.
        assert_eq!(calls(&e, ADD_FORM_TO_DEFERRED_DELETIONS).len(), 1);

        // Another state value (5) gives (0, 0); a furniture state without a
        // reference is cleared (and the player's furniture state).
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let animation = thing(&mut e);
        e.register_double(PLAYER_GET_ANIMATION, |_, _| ret(0x41));
        e.register_double(PLAYER_GET_BIPED, |_, _| ret(0x42));
        let player = actor(&mut e);
        e.set_global(PLAYER, player);
        slot_returns(&mut e, ACTOR_SLOTS, 0x1b8, animation);
        e.mem.set_u8(a + 0x13c, 5);
        e.mem.set_u8(a + 0x13d, 2);
        middle_high_process_finish_init_load_game(&mut e, p, 0, 0, player);
        assert_eq!(calls(&e, OBJECT_SLOTS + 0xd8), vec![vec![animation, 0, 0]]);
        assert_eq!(e.mem.u8(a + 0x13d), 0);
        assert_eq!(calls(&e, PLAYER_CLEAR_FURNITURE), vec![vec![player]]);
        // The player gets both views through slot +0x1CC.
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x1cc).len(), 2);
    }

    #[test]
    fn revert_resets_the_saved_fields() {
        let mut e = engine();
        e.register(SAVE_LOAD_GAME_FLAG, |_, _| ret(1));
        let p = process(&mut e);
        let a = p.addr();
        let object = actor(&mut e);
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(a + 0x114, 0x99);
        e.mem.set_u32(a + 0x1c0, 0x98);
        e.mem.set_u32(a + 0x1ac, list);
        e.mem.set_u32(a + 0x140, 0x55);
        e.mem.set_u8(a + 0x135, 1);
        e.mem.set_u32(a + 0x248, 9);
        middle_high_process_revert(&mut e, p, 0x1000_0020, object);
        assert_eq!(
            calls(&e, MIDDLE_LOW_REVERT),
            vec![vec![a, 0x1000_0020, object]]
        );
        assert_eq!(calls(&e, ACTOR_RELOAD_TARGETS), vec![vec![object, 0]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x160), vec![vec![a, 0, 0, 0]]);
        assert_eq!(calls(&e, ANIMATION_REVERT), vec![vec![0x98, object]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x294), vec![vec![a, object]]);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x4c0),
            vec![vec![a, object, 0, 0, 0x7f]]
        );
        assert_eq!(e.mem.u32(a + 0x140), 0);
        assert_eq!(e.mem.u8(a + 0x144), 0x7f);
        assert_eq!(e.mem.u8(a + 0x135), 0);
        assert_eq!(e.mem.f32(a + 0x170), 1.0);
        assert_eq!(e.mem.u32(a + 0x1ac), 0);
        assert_eq!(calls(&e, LIST_DELETE), vec![vec![list, 1]]);
        let clears: Vec<u32> = calls(&e, LIST_CLEAR).iter().map(|c| c[0]).collect();
        assert_eq!(clears, vec![a + 0xc8, a + 0xd0, list]);
        assert_eq!(e.mem.u32(a + 0x248), 0);

        // The save/load object's check false: only the trailing clears run.
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        e.mem.set_u32(a + 0x140, 0x55);
        e.mem.set_u32(a + 0x24c, 9);
        middle_high_process_revert(&mut e, p, 0, 0);
        assert_eq!(e.mem.u32(a + 0x140), 0x55);
        assert_eq!(e.mem.u32(a + 0x24c), 0);
        assert!(calls(&e, ACTIVE_EFFECTS_REVERT).is_empty());
    }

    #[test]
    fn char_controller_save_size_counts_the_saved_bytes() {
        let mut e = engine();
        let p = process(&mut e);
        let object = actor(&mut e);
        // No controller: nothing.
        assert_eq!(
            middle_high_process_get_char_controller_save_size(&mut e, p, object),
            0
        );
        slot_returns(&mut e, PROCESS_SLOTS, 0x28c, 0x4000);
        e.register(CONTROLLER_STATE_FLAGS, |_, _| ret(0x800));
        let mut e2 = e;
        save_reader(&mut e2, &[], 0, 0x70);
        // Position saved, no flying: 4+4+4+12+16+16+12+4+4.
        assert_eq!(
            middle_high_process_get_char_controller_save_size(&mut e2, p, object),
            76
        );
        // The actor's slot +0x22C true: nothing.
        slot_returns(&mut e2, ACTOR_SLOTS, 0x22c, 1);
        assert_eq!(
            middle_high_process_get_char_controller_save_size(&mut e2, p, object),
            0
        );
        slot_returns(&mut e2, ACTOR_SLOTS, 0x22c, 0);
        // Slot +0x100 true (+4) and version 0x77 (+8).
        slot_returns(&mut e2, ACTOR_SLOTS, 0x100, 1);
        save_reader(&mut e2, &[], 0, 0x77);
        assert_eq!(
            middle_high_process_get_char_controller_save_size(&mut e2, p, object),
            88
        );
        // Without the 0x800 flag the position (12 bytes) is not counted.
        e2.register(CONTROLLER_STATE_FLAGS, |_, _| ret(0));
        assert_eq!(
            middle_high_process_get_char_controller_save_size(&mut e2, p, object),
            76
        );
    }

    #[test]
    fn save_char_controller_writes_what_the_size_counts() {
        let mut e = engine();
        e.register(CONTROLLER_STATE_FLAGS, |_, _| ret(0x800));
        e.register(CONTROLLER_SAVED_WORD, |_, _| ret(0xabcd));
        save_reader(&mut e, &[], 0, 0x77);
        let p = process(&mut e);
        let object = actor(&mut e);
        // No controller: no writes.
        middle_high_process_save_char_controller(&mut e, p, object);
        assert!(calls(&e, WRITE_BYTES).is_empty());
        let controller = 0x4000u32;
        slot_returns(&mut e, PROCESS_SLOTS, 0x28c, controller);
        slot_returns(&mut e, ACTOR_SLOTS, 0x100, 1);
        let expected = middle_high_process_get_char_controller_save_size(&mut e, p, object);
        e.call_log = Some(vec![]);
        middle_high_process_save_char_controller(&mut e, p, object);
        let writes = calls(&e, WRITE_BYTES);
        let total: u32 = writes.iter().map(|w| w[2]).sum();
        assert_eq!(total, u32::from(expected));
        // First the saved word, then +0x520, the flags (0x800), the position.
        assert_eq!(writes[1][1], controller + 0x520);
        assert_eq!(e.mem.u32(writes[2][1]), 0x800);
        assert_eq!(writes[3][2], 0xc);
        assert_eq!(writes[4][1], controller + 0x4f0);
        assert_eq!(writes[5][1], controller + 0x500);
        let tail: Vec<u32> = writes[7..].iter().map(|w| w[1]).collect();
        assert_eq!(
            tail,
            vec![
                controller + 0x540,
                controller + 0x544,
                controller + 0x548,
                controller + 0x550,
                controller + 0x558
            ]
        );
        assert_eq!(calls(&e, CONTROLLER_GET_POSITION).len(), 1);
        assert_eq!(calls(&e, CONTROLLER_GET_LINEAR_VELOCITY).len(), 1);
    }

    #[test]
    fn fn_00926690_reads_the_character_controller() {
        let mut e = engine();
        let p = process(&mut e);
        let object = actor(&mut e);
        // No controller: nothing is read.
        save_reader(&mut e, &[], 0xff, 0x77);
        fn_00926690(&mut e, p, object);
        assert!(calls(&e, READ_BYTES).is_empty());

        let controller = e.mem.alloc(0x600);
        slot_returns(&mut e, PROCESS_SLOTS, 0x28c, controller);
        e.register(CONTROLLER_IS_IN_WATER, |_, _| ret(0));
        // Movement mode 2, flags 0x800 (the position is set), version 0x77.
        save_reader(&mut e, &[2, 0, 0x800], 0x55, 0x77);
        e.call_log = Some(vec![]);
        fn_00926690(&mut e, p, object);
        assert_eq!(
            calls(&e, CONTROLLER_SET_MOVE_MODE),
            vec![vec![controller, 2]]
        );
        assert_eq!(
            calls(&e, CONTROLLER_CLEAR_STATE_FLAGS),
            vec![vec![controller + 0x410, u32::MAX]]
        );
        assert_eq!(
            calls(&e, CONTROLLER_SET_STATE_FLAGS),
            vec![vec![controller + 0x410, 0x800, 1]]
        );
        assert_eq!(calls(&e, CONTROLLER_SET_POSITION).len(), 1);
        // Slot +0x4BC answers 0: the chase bip is cleared.
        assert_eq!(calls(&e, SET_CHASE_BIP), vec![vec![object, 0]]);
        // The velocity goes through 00926980 into the proxy builder only
        // with a proxy.
        assert!(calls(&e, PROXY_SET_LINEAR_VELOCITY).is_empty());
        // Version 0x77 reads +0x550 and +0x558 last.
        let reads = calls(&e, READ_BYTES);
        assert_eq!(reads[reads.len() - 2][1], controller + 0x550);
        assert_eq!(reads[reads.len() - 1][1], controller + 0x558);
        // Mode 2 with state flag 0x200 clears the float at +0x544; the
        // flags double answers 0x200 here.
        e.register(CONTROLLER_STATE_FLAGS, |_, _| ret(0x200));
        e.mem.set_f32(controller + 0x544, 5.0);
        save_reader(&mut e, &[2], 0x55, 0x77);
        slot_returns(&mut e, ACTOR_SLOTS, 0x100, 1);
        fn_00926690(&mut e, p, object);
        assert_eq!(e.mem.f32(controller + 0x544), 0.0);
    }

    #[test]
    fn fn_00926980_sets_the_proxy_velocity() {
        let mut e = engine();
        e.register(MAKE_HAVOK_VECTOR, |_, a| ret(a[0] + 0x100));
        // No proxy: the vector is not built.
        fn_00926980(&mut e, 0x4000, 0x5000);
        assert!(calls(&e, MAKE_HAVOK_VECTOR).is_empty());
        e.register(CONTROLLER_GET_PROXY, |_, _| ret(0x7000));
        fn_00926980(&mut e, 0x4000, 0x5000);
        let made = calls(&e, MAKE_HAVOK_VECTOR);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0][1], 0x5000);
        assert_eq!(
            calls(&e, PROXY_SET_LINEAR_VELOCITY),
            vec![vec![0x7000, made[0][0] + 0x100]]
        );
    }

    #[test]
    fn fn_009269f0_tests_the_flag_bit_0x200() {
        let mut e = engine();
        e.register(CONTROLLER_STATE_FLAGS, |_, a| ret(a[1] & 0x300));
        assert!(fn_009269f0(&mut e, 0x4000));
        assert_eq!(
            calls(&e, CONTROLLER_STATE_FLAGS),
            vec![vec![0x4000 + 0x410, 0x200]]
        );
        e.register(CONTROLLER_STATE_FLAGS, |_, _| ret(0));
        assert!(!fn_009269f0(&mut e, 0x4000));
    }

    #[test]
    fn save_game_ov2_writes_the_fields_in_order() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let buffer = thing(&mut e);
        let reference = actor(&mut e);
        slot_returns(&mut e, OBJECT_SLOTS, 0x8, reference);
        e.register(START_VARIABLE_SIZED_VALUE, |_, _| ret(0x1234));
        e.register(BUFFER_SAVE_KIND, |_, a| ret(a[1]));
        e.register(KIND_HAS_BITS, |_, _| ret(1));
        e.register(MAGIC_ITEM_FORM_ID, |_, a| ret(a[0] + 0x100));
        let mut sizes = 0u32;
        e.register_double(WORD_C, move |_, _| {
            sizes += 100;
            ret(sizes)
        });
        // The id list at +0xC8: two ids, then the queued items at +0x230.
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x22);
        e.mem.set_u32(a + 0xc8, 0x21);
        e.mem.set_u32(a + 0xcc, second);
        let queued = e.mem.alloc(8);
        e.mem.set_u32(queued, 0x41);
        e.mem.set_u32(a + 0x230, queued);
        e.mem.set_u32(a + 0x10c, 0x11);
        e.mem.set_u32(a + 0x164, 0x31);
        e.mem.set_u32(a + 0x1c0, 0x61);
        middle_high_process_save_game_ov2(&mut e, p, buffer);
        assert_eq!(calls(&e, MIDDLE_LOW_SAVE_GAME_OV2), vec![vec![a, buffer]]);
        let saved = calls(&e, SAVE_DATA);
        assert_eq!(saved[0], vec![buffer, a + 0x134, 1, 0]);
        assert_eq!(saved[7], vec![buffer, a + 0xfc, 0xc, 0]);
        assert_eq!(saved[36], vec![buffer, a + 0x110, 1, 0]);
        assert_eq!(saved[37][2], 4);
        let ids: Vec<Vec<u32>> = calls(&e, SAVE_FORM_ID);
        assert_eq!(ids[0], vec![buffer, 0x11, 0]);
        assert_eq!(ids[1], vec![buffer, 0, 0]);
        // The counted id list: both ids, count patched in.
        assert_eq!(ids[4], vec![buffer, 0x21, 0]);
        assert_eq!(ids[5], vec![buffer, 0x22, 0]);
        let patched = calls(&e, SAVE_VARIABLE_SIZED_VALUE);
        assert_eq!(patched[0], vec![buffer, 2, 0x1234]);
        assert_eq!(
            calls(&e, ACTOR_PACKAGE_SAVE_GAME),
            vec![vec![a + 0xe4, buffer]]
        );
        // Animation block: size from the two WORD_C readings.
        assert_eq!(calls(&e, ANIMATION_SAVE_GAME), vec![vec![0x61, buffer]]);
        assert_eq!(patched[1], vec![buffer, 100, 0x1234]);
        // Magic ids: +0x164 maps through the item id, the others are 0.
        let magic = calls(&e, SAVE_FORM_ID_VALUE);
        assert_eq!(
            magic,
            vec![
                vec![buffer, 0x131, 0],
                vec![buffer, 0, 0],
                vec![buffer, 0, 0]
            ]
        );
        assert_eq!(calls(&e, QUEUED_ITEM_SAVE_GAME), vec![vec![0x41, buffer]]);
        assert_eq!(patched[2], vec![buffer, 1, 0x1234]);

        // Without the buffer's reference (slot +0x08) there is no animation.
        let mut e = engine();
        let p = process(&mut e);
        let buffer = thing(&mut e);
        middle_high_process_save_game_ov2(&mut e, p, buffer);
        assert!(calls(&e, ANIMATION_SAVE_GAME).is_empty());
    }

    #[test]
    fn load_game_ov2_reads_the_fields_in_order() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let buffer = thing(&mut e);
        let reference = actor(&mut e);
        let model = thing(&mut e);
        let process_object = thing(&mut e);
        slot_returns(&mut e, OBJECT_SLOTS, 0x0, 0x10);
        slot_returns(&mut e, OBJECT_SLOTS, 0xc, reference);
        slot_returns(&mut e, ACTOR_SLOTS, 0x1d0, model);
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(LIST_CONSTRUCT, |_, a| ret(a[0]));
        e.register(LOAD_VARIABLE_SIZED_VALUE, |_, _| ret(2));
        e.register(BUFFER_SAVE_KIND, |_, a| ret(a[1]));
        e.register(KIND_HAS_BITS, |_, _| ret(1));
        e.register(LOAD_DATA, |e, a| {
            if a[2] == 4 {
                e.mem.set_u32(a[1], 5);
            }
            Ret::default()
        });
        e.register(GET_CONTAINER_CHANGES, |_, _| ret(0x500));
        e.register(GET_WORN_ITEM, |_, _| ret(0x600));
        e.register_double(WORD_68, move |_, _| ret(process_object));
        e.register(ITEM_CHANGE_MOD_EFFECT_ACTIVE, |_, _| ret(1));
        e.register(GET_INVENTORY_CHANGES, |_, _| ret(0x900));
        e.register(INVENTORY_GET_OBJECT_COUNT, |_, _| ret(3));
        let form = 0x4444;
        let item = e.mem.alloc(0x20);
        e.mem.set_u32(item + 8, form);
        let held = e.mem.alloc(0x20);
        e.mem.set_u32(a + 0x114, item);
        e.mem.set_u32(a + 0x118, held);
        e.mem.set_u8(a + 0x13c, 1);
        middle_high_process_load_game_ov2(&mut e, p, buffer);
        assert_eq!(calls(&e, MIDDLE_LOW_LOAD_GAME_OV2), vec![vec![a, buffer]]);
        let loaded = calls(&e, LOAD_DATA);
        assert_eq!(loaded[0], vec![buffer, a + 0x134, 1]);
        // Version 16: +0x23C, +0x244 and +0x110 are read too.
        assert!(loaded.iter().any(|c| c[1] == a + 0x23c));
        assert!(loaded.iter().any(|c| c[1] == a + 0x244));
        assert!(loaded.iter().any(|c| c[1] == a + 0x110));
        assert_eq!(
            calls(&e, ACTOR_APPLY_FURNITURE_STATE),
            vec![vec![reference, 0]]
        );
        // The mark byte read is 0 and the old one was set: slot +0x420 runs.
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x420), vec![vec![a, reference]]);
        let into: Vec<u32> = calls(&e, LOAD_FORM_ID_INTO).iter().map(|c| c[1]).collect();
        assert_eq!(&into[..2], &[a + 0x158, a + 0x140]);
        assert_eq!(&into[into.len() - 3..], &[a + 0x164, a + 0x160, a + 0x1bc]);
        // Two ids were added to the list at +0xC8.
        let adds = calls(&e, LIST_ADD);
        assert_eq!(adds.iter().filter(|c| c[0] == a + 0xc8).count(), 2);
        assert_eq!(
            calls(&e, ACTOR_PACKAGE_LOAD_GAME_OV2),
            vec![vec![a + 0xe4, buffer]]
        );
        assert_eq!(calls(&e, QUEUE_SUB_BUFFER), vec![vec![0, 0, buffer, 0]]);
        // The worn item is restored and equipped, the count limited to 3.
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x160),
            vec![vec![a, 0x600, model, 0]]
        );
        assert_eq!(calls(&e, ACTOR_EQUIP_WORN), vec![vec![reference, form]]);
        assert_eq!(
            calls(&e, ACTOR_SLOTS + 0x3ec),
            vec![vec![reference, form, 0, 1, 1]]
        );
        assert_eq!(calls(&e, ITEM_CHANGE_SET_COUNT), vec![vec![held, 3]]);
        // The two queued items were made and added to the list at +0x230.
        assert_eq!(calls(&e, QUEUED_ITEM_LOAD_GAME).len(), 2);
        let queue = e.mem.u32(a + 0x230);
        assert_ne!(queue, 0);
        assert_eq!(
            calls(&e, LIST_ADD).iter().filter(|c| c[0] == queue).count(),
            2
        );

        // A "Lily" creature gets its default worn instead.
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let buffer = thing(&mut e);
        let reference = actor(&mut e);
        let model = thing(&mut e);
        slot_returns(&mut e, OBJECT_SLOTS, 0xc, reference);
        slot_returns(&mut e, ACTOR_SLOTS, 0x1d0, model);
        slot_returns(&mut e, ACTOR_SLOTS, 0x21c, 1);
        e.register(BUFFER_SAVE_KIND, |_, a| ret(a[1]));
        e.register(KIND_HAS_BITS, |_, _| ret(1));
        e.register(GET_CONTAINER_CHANGES, |_, _| ret(0x500));
        e.register(STRSTR, |_, _| ret(1));
        e.register(REFERENCE_BASE, |_, _| ret(0x77));
        e.register(ANIMATION_LOCK_TEST, |_, _| ret(1));
        let lily_process = thing(&mut e);
        e.register_double(WORD_68, move |_, _| ret(lily_process));
        let item = e.mem.alloc(0x20);
        e.mem.set_u32(a + 0x114, item);
        middle_high_process_load_game_ov2(&mut e, p, buffer);
        assert!(calls(&e, ACTOR_EQUIP_WORN).is_empty());
        assert_eq!(
            calls(&e, CREATURE_INIT_DEFAULT_WORN),
            vec![vec![0x77, reference, 1, 1, 0]]
        );
        assert_eq!(calls(&e, REFERENCE_INIT_ANIMATION), vec![vec![reference]]);
    }

    #[test]
    fn init_load_game_ov2_resolves_ids_and_the_furniture() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let buffer = thing(&mut e);
        let reference = actor(&mut e);
        slot_returns(&mut e, OBJECT_SLOTS, 0x0, 0x10);
        slot_returns(&mut e, OBJECT_SLOTS, 0xc, reference);
        e.register(LOOKUP_FORM, |_, a| ret(a[0] + 0x1000));
        e.register(FORM_TYPE, |_, _| ret(0x27));
        e.register(REFERENCE_CELL, |_, a| {
            ret(if a[0] == 0x1002 { 0x20 } else { 0x10 })
        });
        e.register(MAGIC_ITEM_BY_NUMERIC_ID, |_, a| ret(a[0] + 0x2000));
        e.register(MAGIC_TARGET_BY_NUMERIC_ID, |_, a| ret(a[0] + 0x3000));
        e.mem.set_u32(a + 0x158, 1);
        e.mem.set_u32(a + 0x140, 2);
        e.mem.set_u32(a + 0x164, 4);
        e.mem.set_u32(a + 0x160, 5);
        e.mem.set_u32(a + 0x1bc, 6);
        e.mem.set_u32(a + 0x1a8, 5);
        e.mem.set_u32(a + 0x40, 0x9000);
        // Two queued items: the first has its object set and is preloaded,
        // the second is deleted.
        let (first, second) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(first, 0x1111);
        let (node_1, node_2) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(node_1, first);
        e.mem.set_u32(node_1 + 4, node_2);
        e.mem.set_u32(node_2, second);
        e.mem.set_u32(a + 0x230, node_1);
        e.register(LIST_REMOVE, |e, a| {
            let next = e.mem.u32(a[1] + 4);
            e.mem.set_u32(a[0] + 4, next);
            Ret::default()
        });
        middle_high_process_init_load_game_ov2(&mut e, p, buffer);
        assert_eq!(calls(&e, LOW_INIT_LOAD_GAME_OV2), vec![vec![a, buffer]]);
        assert_eq!(e.mem.u32(a + 0x158), 0x1001);
        assert_eq!(e.mem.u32(a + 0x140), 0x1002);
        assert_eq!(e.mem.u32(a + 0x164), 0x2004);
        assert_eq!(e.mem.u32(a + 0x160), 0x2005);
        assert_eq!(e.mem.u32(a + 0x1bc), 0x3006);
        assert_eq!(
            calls(&e, ACTOR_PACKAGE_INIT_LOAD_GAME_OV2),
            vec![vec![a + 0xe4, buffer]]
        );
        // A furniture reference in another cell is dropped by slot +0x84.
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x84), vec![vec![a, reference]]);
        assert_eq!(calls(&e, SET_MARKER_RESERVED), vec![vec![0x9000, 5, 1]]);
        assert_eq!(calls(&e, SET_WEAPON_FLAGS), vec![vec![a]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x6e4), vec![vec![a, reference]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x7a8), vec![vec![a, reference]]);
        assert_eq!(calls(&e, QUEUED_ITEM_PRELOAD), vec![vec![first, buffer]]);
        assert_eq!(calls(&e, QUEUED_ITEM_DELETE), vec![vec![second, 1]]);
        assert_eq!(calls(&e, LIST_REMOVE), vec![vec![node_1, node_2]]);

        // Without a furniture reference the furniture state is cleared.
        let mut e = engine();
        let p = process(&mut e);
        let buffer = thing(&mut e);
        let player = actor(&mut e);
        e.set_global(PLAYER, player);
        slot_returns(&mut e, OBJECT_SLOTS, 0xc, player);
        e.mem.set_u8(p.addr() + 0x13d, 3);
        middle_high_process_init_load_game_ov2(&mut e, p, buffer);
        assert_eq!(e.mem.u8(p.addr() + 0x13d), 0);
        assert_eq!(calls(&e, PLAYER_CLEAR_FURNITURE), vec![vec![player]]);
    }

    #[test]
    fn finish_load_game_places_the_actor_on_the_marker() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let buffer = thing(&mut e);
        let reference = thing(&mut e);
        let target = actor(&mut e);
        slot_returns(&mut e, OBJECT_SLOTS, 0xc, target);
        e.mem.set_u32(a + 0x140, reference);
        e.mem.set_u8(a + 0x13d, 4);
        // No markers: slot +0x84 runs with the actor.
        middle_high_process_finish_load_game(&mut e, p, buffer);
        assert_eq!(calls(&e, BASE_FINISH_LOAD_GAME), vec![vec![a, buffer]]);
        assert_eq!(
            calls(&e, ACTOR_PACKAGE_FINISH_LOAD_GAME),
            vec![vec![a + 0xe4, buffer]]
        );
        assert_eq!(calls(&e, ACTIVE_EFFECTS_FINISH_LOAD_OV2).len(), 1);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x84), vec![vec![a, target]]);
        assert!(calls(&e, MARKER_ENTRY).is_empty());

        // With markers the entry is used (no index check) and the actor
        // is moved and put in the character controller.
        let entry = e.mem.alloc(0x10);
        e.register(FURNITURE_MARKER_FIND, |_, _| ret(0x500));
        e.register_double(MARKER_ENTRY, move |_, _| ret(entry));
        e.register(GET_CHAR_CONTROLLER, |_, _| ret(0x700));
        e.register(REFERENCE_PLUS_24, |e, _| ret(e.mem.alloc(0x10)));
        e.call_log = Some(vec![]);
        middle_high_process_finish_load_game(&mut e, p, buffer);
        assert_eq!(calls(&e, MARKER_ENTRY), vec![vec![0x500, 0]]);
        assert_eq!(calls(&e, ACTOR_SLOTS + 0x2a8).len(), 1);
        assert_eq!(
            calls(&e, CONTROLLER_SET_POSITION),
            vec![vec![0x700, a + 0x148]]
        );
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x84).len(), 0);

        // Another furniture state does nothing more.
        e.mem.set_u8(a + 0x13d, 2);
        e.call_log = Some(vec![]);
        middle_high_process_finish_load_game(&mut e, p, buffer);
        assert!(calls(&e, MARKER_ENTRY).is_empty());
    }

    #[test]
    fn revert_ov2_resets_the_saved_fields() {
        let mut e = engine();
        e.set_global(MINUS_ONE, -1.0f32);
        let p = process(&mut e);
        let a = p.addr();
        let reverter = thing(&mut e);
        let target = actor(&mut e);
        slot_returns(&mut e, OBJECT_SLOTS, 0xc, target);
        e.register(BUFFER_FLAGS_B, |_, a| ret(a[1]));
        e.register(KIND_HAS_BITS, |_, _| ret(1));
        e.register(OBJECT_FLAG_4, |_, _| ret(1));
        e.register(LIST_REMOVE_HEAD, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        let entry = e.mem.alloc(8);
        e.mem.set_u32(a + 0x1b0, entry);
        e.mem.set_u32(a + 0x1c0, 0x66);
        e.mem.set_u32(a + 0x140, 0x55);
        e.mem.set_u32(a + 0x250, 9);
        middle_high_process_revert_ov2(&mut e, p, reverter);
        assert_eq!(calls(&e, MIDDLE_LOW_REVERT_OV2), vec![vec![a, reverter]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x160), vec![vec![a, 0, 0, 0]]);
        assert_eq!(
            calls(&e, ACTOR_PACKAGE_REVERT_OV2),
            vec![vec![a + 0xe4, reverter]]
        );
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x4c0),
            vec![vec![a, target, 0, 0, 0x7f]]
        );
        assert_eq!(e.mem.u32(a + 0x140), 0);
        assert_eq!(e.mem.u8(a + 0x144), 0x7f);
        assert_eq!(e.mem.f32(a + 0x170), 1.0);
        assert_eq!(e.mem.f32(a + 0x198), -1.0);
        assert_eq!(e.mem.u32(a + 0x1a8), u32::MAX);
        assert_eq!(e.mem.u16(a + 0x22a), 2);
        assert_eq!(e.mem.u32(a + 0x250), 0);
        assert_eq!(calls(&e, ANIMATION_REVERT_OV2), vec![vec![0x66, reverter]]);
        assert_eq!(calls(&e, ACTIVE_EFFECTS_REVERT_OV2).len(), 1);
        assert_eq!(calls(&e, DEFERRED_HIDE_LIMB_DELETE), vec![vec![entry, 1]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x180), vec![vec![a]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x420), vec![vec![a, target]]);

        // Without an actor and without the flag bits nothing optional runs.
        let mut e = engine();
        e.set_global(MINUS_ONE, -1.0f32);
        let p = process(&mut e);
        let reverter = thing(&mut e);
        e.register(BUFFER_FLAGS_B, |_, a| ret(a[1]));
        middle_high_process_revert_ov2(&mut e, p, reverter);
        assert!(calls(&e, PROCESS_SLOTS + 0x160).is_empty());
        assert!(calls(&e, PROCESS_SLOTS + 0x4c0).is_empty());
        assert!(calls(&e, PROCESS_SLOTS + 0x420).is_empty());
    }

    #[test]
    fn load_sit_sleep_idle_applies_the_idle() {
        let mut e = engine();
        let player = actor(&mut e);
        e.set_global(PLAYER, player);
        e.set_global(IDLE_BLEND_RATE, 0.1f32);
        let p = process(&mut e);
        let a = p.addr();
        let reference = thing(&mut e);
        e.mem.set_u32(a + 0x140, reference);
        e.mem.set_u8(a + 0x144, 1);
        e.mem.set_u8(a + 0x13d, 3);
        // No markers: slot +0x84 runs and nothing else.
        middle_high_process_load_sit_sleep_idle(&mut e, p, player, 0x4000);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x84), vec![vec![a, player]]);
        assert!(calls(&e, ACTOR_APPLY_FURNITURE_STATE).is_empty());

        e.register(FURNITURE_MARKER_FIND, |_, _| ret(0x500));
        e.register(MARKER_COUNT, |_, _| ret(3));
        e.register(PLAYER_GET_ANIMATION, |_, _| ret(0x77));
        e.register(GET_IDLE_TO_PLAY, |_, _| ret(0x88));
        e.register(IDLE_ANIM_GROUP_SECTION, |_, _| ret(5));
        slot_returns(&mut e, PROCESS_SLOTS, 0x2b4, 1);
        slot_returns(&mut e, PROCESS_SLOTS, 0x28c, 0x700);
        e.call_log = Some(vec![]);
        middle_high_process_load_sit_sleep_idle(&mut e, p, player, 0x4000);
        // State 3 is applied as 1, then the saved 3 is restored.
        assert_eq!(
            calls(&e, ACTOR_APPLY_FURNITURE_STATE),
            vec![vec![player, 1], vec![player, 3]]
        );
        assert_eq!(e.mem.u8(a + 0x13d), 3);
        assert_eq!(
            calls(&e, REFERENCE_SET_MARKER_USED),
            vec![vec![reference, 1, 1]]
        );
        assert_eq!(
            calls(&e, SPECIAL_IDLE_REPLACE),
            vec![vec![0x4000, 0x88, player, 5]]
        );
        assert_eq!(calls(&e, PLAYER_SET_ANIMATION_STATE).len(), 2);
        assert_eq!(
            calls(&e, ACTOR_SLOTS + 0x2e0),
            vec![vec![player, 0.1f32.to_bits()]]
        );
        assert_eq!(calls(&e, SET_CHASE_BIP), vec![vec![player, 1]]);
        assert_eq!(
            calls(&e, CONTROLLER_SET_STATE_FLAGS),
            vec![vec![0x700 + 0x410, 0x0800_0000, 1]]
        );

        // Slot +0x2B4 false: blend 0, chase bip and flag cleared.
        slot_returns(&mut e, PROCESS_SLOTS, 0x2b4, 0);
        e.call_log = Some(vec![]);
        middle_high_process_load_sit_sleep_idle(&mut e, p, player, 0x4000);
        assert_eq!(calls(&e, ACTOR_SLOTS + 0x2e0), vec![vec![player, 0]]);
        assert_eq!(calls(&e, SET_CHASE_BIP), vec![vec![player, 0]]);
        assert_eq!(
            calls(&e, CONTROLLER_SET_STATE_FLAGS),
            vec![vec![0x700 + 0x410, 0x0800_0000, 0]]
        );
    }

    #[test]
    fn clear_furniture_frees_the_marker_and_the_idle() {
        let mut e = engine();
        let player = actor(&mut e);
        e.set_global(PLAYER, player);
        let p = process(&mut e);
        let a = p.addr();
        let reference = thing(&mut e);
        e.mem.set_u32(a + 0x140, reference);
        e.mem.set_u8(a + 0x144, 2);
        slot_returns(&mut e, ACTOR_SLOTS, 0x1e4, 0x70);
        middle_high_process_clear_furniture(&mut e, p, player);
        assert_eq!(
            calls(&e, REFERENCE_SET_MARKER_USED),
            vec![vec![reference, 2, 0]]
        );
        assert_eq!(calls(&e, SPECIAL_IDLE_FREE), vec![vec![0x70, 1, 0]]);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x4c0),
            vec![vec![a, player, 0, 0, 0x7f]]
        );
        assert_eq!(calls(&e, PLAYER_CLEAR_FURNITURE), vec![vec![player]]);
        assert_eq!(calls(&e, CLEAR_FURNITURE_MARKER), vec![vec![a]]);

        // Without a reference only the slot and the marker clear run.
        let mut e = engine();
        let p = process(&mut e);
        let other = actor(&mut e);
        middle_high_process_clear_furniture(&mut e, p, other);
        assert!(calls(&e, REFERENCE_SET_MARKER_USED).is_empty());
        assert!(calls(&e, PLAYER_CLEAR_FURNITURE).is_empty());
        assert_eq!(calls(&e, CLEAR_FURNITURE_MARKER).len(), 1);
    }

    #[test]
    fn save_char_controller_ov2_writes_through_the_buffer() {
        let mut e = engine();
        e.register(CONTROLLER_STATE_FLAGS, |_, _| ret(0x800));
        let p = process(&mut e);
        let buffer = thing(&mut e);
        let target = actor(&mut e);
        let controller = e.mem.alloc(0x600);
        // No controller: nothing.
        middle_high_process_save_char_controller_ov2(&mut e, p, buffer);
        assert!(calls(&e, SAVE_DATA).is_empty());
        slot_returns(&mut e, PROCESS_SLOTS, 0x28c, controller);
        slot_returns(&mut e, OBJECT_SLOTS, 0x8, target);
        middle_high_process_save_char_controller_ov2(&mut e, p, buffer);
        let writes = calls(&e, SAVE_DATA);
        let sizes: Vec<u32> = writes.iter().map(|w| w[2]).collect();
        assert_eq!(sizes, vec![1, 1, 4, 12, 16, 16, 12, 4, 4, 4, 4, 4]);
        let tail: Vec<u32> = writes[7..].iter().map(|w| w[1]).collect();
        assert_eq!(
            tail,
            vec![
                controller + 0x540,
                controller + 0x544,
                controller + 0x548,
                controller + 0x550,
                controller + 0x558
            ]
        );
        // An actor whose slot +0x22C is true: nothing more is written.
        slot_returns(&mut e, ACTOR_SLOTS, 0x22c, 1);
        e.call_log = Some(vec![]);
        middle_high_process_save_char_controller_ov2(&mut e, p, buffer);
        assert!(calls(&e, SAVE_DATA).is_empty());
    }

    /// The load buffer reader: `LOAD_DATA` writes the next of `values` (then
    /// 0) as little endian bytes.
    fn load_reader(e: &mut Engine, values: &[u32]) {
        let values = values.to_vec();
        let mut next = 0;
        e.register_double(LOAD_DATA, move |e, a| {
            let value = values.get(next).copied().unwrap_or(0);
            next += 1;
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, (value >> (8 * (i % 4))) as u8);
            }
            Ret::default()
        });
    }

    /// A float cell holding `value`, answered as the address of a setting.
    fn float_cell(e: &mut Engine, value: f32) -> u32 {
        let cell = e.mem.alloc(4);
        e.mem.set_f32(cell, value);
        cell
    }

    #[test]
    fn load_char_controller_reads_the_record() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let buffer = thing(&mut e);
        let target = actor(&mut e);
        let controller = e.mem.alloc(0x600);
        // No controller: nothing is read.
        middle_high_process_load_char_controller(&mut e, p, buffer);
        assert!(calls(&e, LOAD_DATA).is_empty());
        slot_returns(&mut e, PROCESS_SLOTS, 0x28c, controller);
        slot_returns(&mut e, OBJECT_SLOTS, 0xc, target);
        slot_returns(&mut e, ACTOR_SLOTS, 0x214, 4);
        slot_returns(&mut e, PROCESS_SLOTS, 0x2b4, 1);
        e.register(CONTROLLER_NEXT_VALUE, |_, _| ret(0x4321));
        load_reader(&mut e, &[3, 7, 0x800]);
        middle_high_process_load_char_controller(&mut e, p, buffer);
        let sizes: Vec<u32> = calls(&e, LOAD_DATA).iter().map(|c| c[2]).collect();
        assert_eq!(sizes, vec![1, 1, 4, 12, 16, 16, 12, 4, 4, 4, 4, 4]);
        assert_eq!(
            calls(&e, CONTROLLER_SET_MOVE_MODE),
            vec![vec![controller, 3], vec![controller, 0]]
        );
        assert_eq!(e.mem.u32(controller + 0x520), 7);
        assert_eq!(
            calls(&e, CONTROLLER_CLEAR_STATE_FLAGS),
            vec![vec![controller + 0x410, u32::MAX]]
        );
        assert_eq!(
            calls(&e, CONTROLLER_SET_STATE_FLAGS),
            vec![
                vec![controller + 0x410, 0x800, 1],
                vec![controller + 0x410, 0x0800_0000, 1]
            ]
        );
        assert_eq!(calls(&e, CONTROLLER_SET_POSITION).len(), 1);
        // State 4: the actor is not given a chase bip reset, the value is
        // built and handed to 00928d00.
        assert!(calls(&e, SET_CHASE_BIP).is_empty());
        assert_eq!(
            calls(&e, CONTROLLER_APPLY_VALUE),
            vec![vec![controller, 0x4321]]
        );
        let built = calls(&e, CONSTRUCT_QUATERNION);
        assert_eq!(built.len(), 1);
        assert_eq!(&built[0][1..], &[0, 0, 1.0f32.to_bits(), 0]);
        assert_eq!(
            calls(&e, CONTROLLER_SET_VALUE_450),
            vec![vec![controller + 0x450, built[0][0]]]
        );
        let _ = a;

        // State 0: the chase bip is reset and no value is built.
        slot_returns(&mut e, ACTOR_SLOTS, 0x214, 0);
        e.call_log = Some(vec![]);
        load_reader(&mut e, &[3, 7, 0]);
        middle_high_process_load_char_controller(&mut e, p, buffer);
        assert_eq!(calls(&e, SET_CHASE_BIP), vec![vec![target, 0]]);
        assert!(calls(&e, CONSTRUCT_QUATERNION).is_empty());
        assert!(calls(&e, CONTROLLER_SET_POSITION).is_empty());
    }

    #[test]
    fn fn_00928d00_gives_the_value_to_the_controller() {
        let mut e = engine();
        fn_00928d00(&mut e, 0x4000, 0x5000);
        assert_eq!(
            calls(&e, CONTROLLER_SET_VALUE_450),
            vec![vec![0x4000 + 0x450, 0x5000]]
        );
    }

    #[test]
    fn set_should_refresh_magic_shaders_effects_ors_the_flags() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        middle_high_process_set_should_refresh_magic_shaders_effects(&mut e, p, 1, 2, 4);
        assert_eq!(
            [
                e.mem.u8(a + 0x17c),
                e.mem.u8(a + 0x17d),
                e.mem.u8(a + 0x17e),
                e.mem.u8(a + 0x17f)
            ],
            [1, 2, 1, 4]
        );
        middle_high_process_set_should_refresh_magic_shaders_effects(&mut e, p, 0, 8, 0);
        assert_eq!(e.mem.u8(a + 0x17d), 10);
        assert_eq!(e.mem.u8(a + 0x17e), 1);
        assert_eq!(e.mem.u8(a + 0x17f), 4);
    }

    #[test]
    fn refresh_magic_shader_effects_starts_and_updates_effects() {
        let mut e = engine();
        e.set_global(SHADER_EFFECT_TIMER, 100.0f32);
        e.set_global(MINUS_ONE, -1.0f32);
        let p = process(&mut e);
        let a = p.addr();
        let target = actor(&mut e);
        let effect = thing(&mut e);
        // Flag +0x17D: the actor's existing magic shader effect refreshes.
        e.mem.set_u8(a + 0x17c, 1);
        e.mem.set_u8(a + 0x17d, 1);
        e.register_double(PROCESS_LISTS_FIND_EFFECT, move |_, _| ret(effect));
        middle_high_process_refresh_magic_shader_effects(&mut e, p, target);
        assert_eq!(calls(&e, OBJECT_SLOTS + 0xc4), vec![vec![effect]]);
        assert_eq!(
            [
                e.mem.u8(a + 0x17c),
                e.mem.u8(a + 0x17d),
                e.mem.u8(a + 0x17e),
                e.mem.u8(a + 0x17f)
            ],
            [0, 0, 0, 0]
        );

        // Flag +0x17E with a shader form and no existing weapon effect:
        // a new effect is built, flagged, timed (+0x17F) and added.
        let mut e = engine();
        e.set_global(SHADER_EFFECT_TIMER, 100.0f32);
        e.set_global(MINUS_ONE, -1.0f32);
        let p = process(&mut e);
        let a = p.addr();
        let target = actor(&mut e);
        let created = thing(&mut e);
        e.register(EFFECT_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register_double(SHADER_EFFECT_CONSTRUCT, move |_, _| ret(created));
        e.register(PROCESS_LISTS_ADD_TEMP_EFFECT, |_, _| Ret::default());
        slot_returns(&mut e, PROCESS_SLOTS, 0x574, 0x99);
        slot_returns(&mut e, OBJECT_SLOTS, 0xc4, 1);
        e.mem.set_u8(a + 0x17e, 1);
        e.mem.set_u8(a + 0x17f, 1);
        middle_high_process_refresh_magic_shader_effects(&mut e, p, target);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x578), vec![vec![a, 0]]);
        assert_eq!(calls(&e, SHADER_EFFECT_CONSTRUCT).len(), 1);
        assert_eq!(
            calls(&e, SHADER_EFFECT_CONSTRUCT)[0][1..],
            [target, 0x99, (-1.0f32).to_bits()]
        );
        assert_eq!(e.mem.u8(created + 0x28), 1);
        assert_eq!(e.mem.f32(created + 0x34), 100.0);
        assert_eq!(
            calls(&e, PROCESS_LISTS_ADD_TEMP_EFFECT),
            vec![vec![PROCESS_LISTS, created]]
        );

        // An existing weapon effect is refreshed instead.
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let target = actor(&mut e);
        let existing = thing(&mut e);
        e.register_double(PROCESS_LISTS_FIND_WEAPON_EFFECT, move |_, _| ret(existing));
        e.mem.set_u8(a + 0x17e, 1);
        middle_high_process_refresh_magic_shader_effects(&mut e, p, target);
        assert_eq!(calls(&e, OBJECT_SLOTS + 0xc4), vec![vec![existing]]);
        assert!(calls(&e, SHADER_EFFECT_CONSTRUCT).is_empty());

        // The player's own shader effect is deleted when the process has
        // none.
        let mut e = engine();
        let player = actor(&mut e);
        e.set_global(PLAYER, player);
        let p = process(&mut e);
        let a = p.addr();
        let held = thing(&mut e);
        e.register(PLAYER_HAS_SHADER_SLOT, |_, _| ret(1));
        e.register_double(PLAYER_SHADER_EFFECT, move |_, _| ret(held));
        e.mem.set_u32(player + 0x6a4, held);
        e.mem.set_u8(a + 0x17e, 1);
        middle_high_process_refresh_magic_shader_effects(&mut e, p, player);
        assert_eq!(calls(&e, OBJECT_SLOTS), vec![vec![held, 1]]);
        assert_eq!(e.mem.u32(player + 0x6a4), 0);
    }

    #[test]
    fn fn_00929220_stores_the_word() {
        let mut e = engine();
        let object = e.mem.alloc(0x700);
        fn_00929220(&mut e, object, 0x1234);
        assert_eq!(e.mem.u32(object + 0x6a4), 0x1234);
    }

    #[test]
    fn fn_00929240_stores_the_quieted_float() {
        let mut e = engine();
        let object = e.mem.alloc(0x40);
        fn_00929240(&mut e, object, 2.5);
        assert_eq!(e.mem.f32(object + 0x34), 2.5);
        // A signalling NaN is quieted by the x87 load and store.
        fn_00929240(&mut e, object, f32::from_bits(0x7f80_0001));
        assert_eq!(e.mem.u32(object + 0x34), 0x7fc0_0001);
    }

    #[test]
    fn fn_00929260_stores_the_byte() {
        let mut e = engine();
        let object = e.mem.alloc(0x40);
        fn_00929260(&mut e, object, 7);
        assert_eq!(e.mem.u8(object + 0x28), 7);
    }

    #[test]
    fn get_face_animation_data_caches_the_data() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let target = actor(&mut e);
        e.register(REFERENCE_FACE_ANIMATION_DATA, |_, _| ret(0x55));
        e.mem.set_u32(a + 0x178, 0x44);
        // No model: the cache is cleared and 0 answered.
        assert_eq!(
            middle_high_process_get_face_animation_data(&mut e, p, target),
            0
        );
        assert_eq!(e.mem.u32(a + 0x178), 0);
        // With a model the data is fetched once.
        slot_returns(&mut e, ACTOR_SLOTS, 0x1d0, 0x66);
        assert_eq!(
            middle_high_process_get_face_animation_data(&mut e, p, target),
            0x55
        );
        assert_eq!(
            middle_high_process_get_face_animation_data(&mut e, p, target),
            0x55
        );
        assert_eq!(
            calls(&e, REFERENCE_FACE_ANIMATION_DATA),
            vec![vec![target, 0]]
        );
    }

    #[test]
    fn should_check_flare_combines_the_light_bits() {
        let mut e = engine();
        e.register(LIGHT_FLARE_VALUE, |_, a| ret(a[0]));
        e.register(LIGHT_FLARE_BITS, |_, a| ret(a[0]));
        let p = process(&mut e);
        let a = p.addr();
        let (first, second) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(first, 1);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, 4);
        e.mem.set_u8(a + 0x180, 1);
        assert!(middle_high_process_should_check_flare(&mut e, p, first, 4));
        assert_eq!(e.mem.u32(a + 0x184), 5);
        assert_eq!(e.mem.u8(a + 0x180), 0);
        // The bits are cached until the flag is set again.
        assert!(!middle_high_process_should_check_flare(&mut e, p, first, 2));
        assert_eq!(calls(&e, LIGHT_FLARE_VALUE).len(), 2);
    }

    #[test]
    fn fn_00929390_tests_common_bits() {
        let mut e = engine();
        assert!(fn_00929390(&mut e, 6, 3));
        assert!(!fn_00929390(&mut e, 4, 3));
        assert!(!fn_00929390(&mut e, 0, u32::MAX));
    }

    #[test]
    fn add_attached_arrow_creates_the_list_and_adds_once() {
        let mut e = engine();
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(LIST_CONSTRUCT, |_, a| ret(a[0]));
        let p = process(&mut e);
        let a = p.addr();
        middle_high_process_add_attached_arrow(&mut e, p, 0);
        assert!(calls(&e, ALLOCATE).is_empty());
        middle_high_process_add_attached_arrow(&mut e, p, 0x70);
        let list = e.mem.u32(a + 0x1ac);
        assert_ne!(list, 0);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![8]]);
        let adds = calls(&e, LIST_ADD);
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][0], list);
        // An arrow the list holds is not added again.
        e.register(LIST_CONTAINS, |_, _| ret(1));
        middle_high_process_add_attached_arrow(&mut e, p, 0x70);
        assert_eq!(calls(&e, LIST_ADD).len(), 1);
        assert_eq!(calls(&e, ALLOCATE).len(), 1);
    }

    #[test]
    fn remove_attached_arrow_removes_from_the_list() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        middle_high_process_remove_attached_arrow(&mut e, p, 0x70);
        assert!(calls(&e, LIST_REMOVE).is_empty());
        e.mem.set_u32(a + 0x1ac, 0x5000);
        middle_high_process_remove_attached_arrow(&mut e, p, 0);
        assert!(calls(&e, LIST_REMOVE).is_empty());
        middle_high_process_remove_attached_arrow(&mut e, p, 0x70);
        let removed = calls(&e, LIST_REMOVE);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], 0x5000);
    }

    #[test]
    fn move_attached_arrows_moves_the_ones_in_other_cells() {
        let mut e = engine();
        e.register(REFERENCE_CELL, |_, a| {
            ret(match a[0] {
                0x1000 => 0x10,
                0x2000 => 0x20,
                0x3000 => 0x10,
                _ => 0,
            })
        });
        let p = process(&mut e);
        let a = p.addr();
        let (first, second) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(first, 0x2000);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, 0x3000);
        // No list or no parent: nothing happens.
        middle_high_process_move_attached_arrows_to_parent_cell(&mut e, p, 0x1000);
        e.mem.set_u32(a + 0x1ac, first);
        middle_high_process_move_attached_arrows_to_parent_cell(&mut e, p, 0);
        assert!(calls(&e, CELL_MOVE_REFERENCE).is_empty());
        middle_high_process_move_attached_arrows_to_parent_cell(&mut e, p, 0x1000);
        assert_eq!(calls(&e, CELL_MOVE_REFERENCE), vec![vec![0x10, 0x2000]]);
        assert_eq!(
            calls(&e, PROCESS_LISTS_REFERENCE_MOVED),
            vec![vec![PROCESS_LISTS, 0x2000]]
        );
    }

    #[test]
    fn set_aim_sequence_stores_and_sets_the_value() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        middle_high_process_set_aim_sequence(&mut e, p, 0, 0x70);
        middle_high_process_set_aim_sequence(&mut e, p, 2, 0x72);
        middle_high_process_set_aim_sequence(&mut e, p, 1, 0);
        assert_eq!(e.mem.u32(a + 0x1c4), 0x70);
        assert_eq!(e.mem.u32(a + 0x1cc), 0x72);
        assert_eq!(e.mem.u32(a + 0x1c8), 0);
        assert_eq!(
            calls(&e, SEQUENCE_SET_VALUE),
            vec![vec![0x70, 1.0f32.to_bits()], vec![0x72, 0]]
        );
    }

    #[test]
    fn update_aim_sequences_blends_the_sequences() {
        let mut e = engine();
        e.register(FLOAT_C, |_, _| ret_f(0.5));
        let cell = float_cell(&mut e, 2.0);
        e.register_double(FLOAT_SETTING_VALUE, move |_, _| ret(cell));
        e.register(MOBILE_GET_LOOKING, |_, _| ret_f(-0.5));
        e.register(MIN_OF_FLOATS, |_, a| {
            ret_f(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        e.register(MAX_OF_FLOATS, |_, a| {
            ret_f(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        let p = process(&mut e);
        let a = p.addr();
        let target = actor(&mut e);
        e.mem.set_f32(a + 0x1d0, 1.0);
        e.mem.set_u32(a + 0x1c4, 0x70);
        e.mem.set_u32(a + 0x1c8, 0x71);
        e.mem.set_u32(a + 0x1cc, 0x72);
        middle_high_process_update_aim_sequences(&mut e, p, target);
        // The aim angle crossed zero and snaps to it.
        assert_eq!(e.mem.f32(a + 0x1d0), 0.0);
        // Looking left of centre by more than the setting: the left
        // sequence gets the full weight, the base none.
        assert_eq!(
            calls(&e, SEQUENCE_SET_VALUE),
            vec![vec![0x70, 0], vec![0x71, 1.0f32.to_bits()], vec![0x72, 0]]
        );

        // Looking right: the right sequence is blended, an eighth of the weight
        // (looking 0.00436 radians against 2 * 0.017453292 gives
        // 0.125).
        let mut e2 = e;
        e2.register(MOBILE_GET_LOOKING, |_, _| ret_f(0.00436332325));
        e2.mem.set_f32(a + 0x1d0, 0.0);
        e2.call_log = Some(vec![]);
        middle_high_process_update_aim_sequences(&mut e2, p, target);
        let sets = calls(&e2, SEQUENCE_SET_VALUE);
        assert_eq!(sets[1], vec![0x71, 0]);
        let right = f32::from_bits(sets[2][1]);
        assert!((right - 0.125).abs() < 1e-4, "right weight {right}");
        let base = f32::from_bits(sets[0][1]);
        assert!((base - (1.0 - right)).abs() < 1e-6);
    }

    #[test]
    fn update_aim_sequences_snaps_strictly_inside_the_widened_float_epsilon() {
        // 009296a3/009296b3 compare against the float 1e-6 widened to
        // double (0x3eb0c6f7a0000000), strictly: the float 1e-6 itself
        // (0x358637bd) and its negative are kept, the next float towards
        // zero snaps to 0. No frame time, so `next` is the angle itself.
        for (bits, expected) in [
            (0x3586_37bd_u32, 0x3586_37bd_u32),
            (0xb586_37bd, 0xb586_37bd),
            (0x3586_37bc, 0),
            (0xb586_37bc, 0),
        ] {
            let mut e = engine();
            e.register(FLOAT_C, |_, _| ret_f(0.0));
            let cell = float_cell(&mut e, 2.0);
            e.register_double(FLOAT_SETTING_VALUE, move |_, _| ret(cell));
            e.register(MOBILE_GET_LOOKING, |_, _| ret_f(0.0));
            e.register(MIN_OF_FLOATS, |_, a| {
                ret_f(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
            });
            e.register(MAX_OF_FLOATS, |_, a| {
                ret_f(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
            });
            let p = process(&mut e);
            let a = p.addr();
            let target = actor(&mut e);
            e.mem.set_f32(a + 0x1d0, f32::from_bits(bits));
            middle_high_process_update_aim_sequences(&mut e, p, target);
            assert_eq!(e.mem.u32(a + 0x1d0), expected, "angle {bits:#010x}");
        }
    }

    /// A procedure table for package type `kind` whose entry `index` is
    /// `entry`.
    fn procedure_table(e: &mut Engine, kind: u32, index: u32, entry: u32) {
        let table = e.mem.alloc(0x100);
        e.mem.set_u32(table + 4 * index, entry);
        e.mem.set_u32(PROCEDURE_TABLES + 4 * kind, table);
    }

    #[test]
    fn fn_00929820_decides_from_the_procedure_entry() {
        let mut e = engine();
        e.set_global(MINUS_ONE, -1.0f32);
        let p = process(&mut e);
        let a = p.addr();
        let target = actor(&mut e);
        let package = thing(&mut e);
        slot_returns(&mut e, PROCESS_SLOTS, 0x27c, package);
        slot_returns(&mut e, PROCESS_SLOTS, 0x280, 1);
        slot_returns(&mut e, OBJECT_SLOTS, 0x13c, 1);
        procedure_table(&mut e, 0, 1, 0x2a);
        // The process blocks idles: false.
        e.register(PROCESS_BLOCKS_IDLE, |_, _| ret(1));
        assert!(!fn_00929820(&mut e, p, target));
        e.register(PROCESS_BLOCKS_IDLE, |_, _| ret(0));
        // Entry 0x2A is replaced by the extra's package type (here 4, in
        // the list); package type 0 asks the package's slot +0x13C.
        slot_returns(&mut e, PROCESS_SLOTS, 0x274, 0x80);
        e.register(EXTRA_PACKAGE_TYPE, |_, _| ret(4));
        assert!(fn_00929820(&mut e, p, target));
        assert_eq!(
            calls(&e, OBJECT_SLOTS + 0x13c),
            vec![vec![package, target, 0, (-1.0f32).to_bits(), 0]]
        );
        // An entry outside the list: false.
        e.register(EXTRA_PACKAGE_TYPE, |_, _| ret(2));
        assert!(!fn_00929820(&mut e, p, target));
        // Another package type uses the actor's path state.
        e.register(PACKAGE_WORD_18, |_, _| ret(1));
        procedure_table(&mut e, 1, 1, 0x1b);
        assert!(!fn_00929820(&mut e, p, target));
        e.register(ACTOR_IS_PATHING_COMPLETE, |_, _| ret(1));
        assert!(fn_00929820(&mut e, p, target));
        let _ = a;
    }

    /// The pieces `fn_00929960` needs: a package with an idle collection
    /// (play mode `mode`, `flags` byte at +4) whose three idles are
    /// 0x101..0x103. Answers the process, the actor and the collection.
    fn idle_scene(e: &mut Engine, mode: u32, flags: u8) -> (Ptr<MiddleHighProcess>, u32, u32) {
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(RANDOM_IDLE_TIMER, 0.8f32);
        let p = process(e);
        let target = actor(e);
        let package = thing(e);
        let collection = e.mem.alloc(0x10);
        e.mem.set_u8(collection + 4, flags);
        let idles = e.mem.alloc(0x10);
        for (i, id) in [0x101u32, 0x102, 0x103].into_iter().enumerate() {
            e.mem.set_u32(idles + 4 * i as u32, id);
        }
        slot_returns(e, PROCESS_SLOTS, 0x27c, package);
        slot_returns(e, PROCESS_SLOTS, 0x280, 1);
        slot_returns(e, OBJECT_SLOTS, 0x13c, 1);
        slot_returns(e, ACTOR_SLOTS, 0x1e4, 0x70);
        procedure_table(e, 0, 1, 0);
        e.register_double(PACKAGE_IDLE_COLLECTION, move |_, _| ret(collection));
        e.register(SPECIAL_IDLE_DONE_PLAYING, |_, _| ret(1));
        e.register_double(IDLE_COLLECTION_MODE, move |_, _| ret(mode));
        e.register(GET_IDLES_FOR_ACTOR, |_, _| ret(3));
        e.register_double(IDLE_ARRAY_ELEMENT, move |_, a| ret(idles + 4 * a[1]));
        e.register(RAND, |_, _| ret(7));
        (p, target, collection)
    }

    #[test]
    fn fn_00929960_plays_the_next_idle() {
        // Sequential: the next idle after the one before.
        let mut e = engine();
        let (p, target, _) = idle_scene(&mut e, 1, 4);
        let a = p.addr();
        e.mem.set_u32(a + 0x190, u32::MAX);
        fn_00929960(&mut e, p, target, 0);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x44),
            vec![vec![a, target, 0x101, 2, 1, 0, 1]]
        );
        assert_eq!(e.mem.f32(a + 0x198), -1.0);
        assert_eq!(calls(&e, IDLE_ARRAY_DESTRUCT).len(), 1);
        fn_00929960(&mut e, p, target, 0);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x44)[1][2], 0x102);
        // The last idle with the collection's flag asks to stop the idle.
        e.mem.set_u32(a + 0x190, 1);
        fn_00929960(&mut e, p, target, 0);
        assert_eq!(calls(&e, SET_PROCESS_FLAG_40), vec![vec![a, 1]]);

        // Random: rand 7 of 3 idles picks the second; the timer is 0.8.
        let mut e = engine();
        let (p, target, _) = idle_scene(&mut e, 0, 4);
        let a = p.addr();
        fn_00929960(&mut e, p, target, 0);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x44)[0][2], 0x102);
        assert_eq!(e.mem.f32(a + 0x198), 0.8);
        assert_eq!(calls(&e, SET_PROCESS_FLAG_40), vec![vec![a, 1]]);

        // While the timer is positive it only counts down.
        let mut e = engine();
        let (p, target, _) = idle_scene(&mut e, 0, 0);
        let a = p.addr();
        e.register(FLOAT_C, |_, _| ret_f(0.25));
        e.mem.set_f32(a + 0x198, 0.5);
        fn_00929960(&mut e, p, target, 0);
        assert_eq!(e.mem.f32(a + 0x198), 0.25);
        assert!(calls(&e, PROCESS_SLOTS + 0x44).is_empty());

        // An idle still playing blocks unless forced; the idle flag
        // (+0x110) forces and clears the timer.
        let mut e = engine();
        let (p, target, _) = idle_scene(&mut e, 1, 0);
        let a = p.addr();
        e.register(SPECIAL_IDLE_DONE_PLAYING, |_, _| ret(0));
        fn_00929960(&mut e, p, target, 0);
        assert!(calls(&e, PROCESS_SLOTS + 0x44).is_empty());
        fn_00929960(&mut e, p, target, 1);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x44).len(), 1);
        e.mem.set_u8(a + 0x110, 1);
        e.mem.set_f32(a + 0x198, 5.0);
        fn_00929960(&mut e, p, target, 0);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x44).len(), 2);
        assert_eq!(e.mem.u8(a + 0x110), 0);

        // No package: nothing.
        let mut e = engine();
        let p = process(&mut e);
        let target = actor(&mut e);
        fn_00929960(&mut e, p, target, 1);
        assert!(calls(&e, PROCESS_BLOCKS_IDLE).is_empty());
    }

    #[test]
    fn fn_00929d20_tests_bit_4_of_the_flag_byte() {
        let mut e = engine();
        let collection = e.mem.alloc(0x10);
        assert!(!fn_00929d20(&mut e, collection));
        e.mem.set_u8(collection + 4, 4);
        assert!(fn_00929d20(&mut e, collection));
        e.mem.set_u8(collection + 4, 0xfb);
        assert!(!fn_00929d20(&mut e, collection));
    }

    /// A process holding an automatic weapon: base rate 2.0, attack speed
    /// 0.25, so a shot interval of 0.5.
    fn automatic_weapon(e: &mut Engine) -> Ptr<MiddleHighProcess> {
        let p = process(e);
        let a = p.addr();
        let item = e.mem.alloc(0x20);
        e.mem.set_u32(item + 8, 0x99);
        e.mem.set_u32(a + 0x114, item);
        e.mem.set_u32(a + 0x118, 0x55);
        e.register(FORM_IS_AUTOMATIC, |_, _| ret(1));
        e.register(FORM_BASE_RATE, |_, _| ret_f(2.0));
        e.register(WEAPON_ATTACK_SPEED, |_, _| ret_f(0.25));
        p
    }

    #[test]
    fn clear_automatic_shots_to_fire_sets_the_shot_interval() {
        let mut e = engine();
        let p = automatic_weapon(&mut e);
        let a = p.addr();
        middle_high_process_clear_automatic_shots_to_fire(&mut e, p, 1);
        // 1 / (0.25 * 2.0).
        assert_eq!(e.mem.f32(a + 0x1d4), 2.0);
        assert_eq!(calls(&e, ITEM_CHANGE_MOD_EFFECT_ACTIVE).len(), 1);
        assert_eq!(calls(&e, WEAPON_ATTACK_SPEED), vec![vec![0x99, 0]]);
        middle_high_process_clear_automatic_shots_to_fire(&mut e, p, 0);
        assert_eq!(e.mem.f32(a + 0x1d4), 0.0);
        // Not automatic, or no weapon list: zero.
        e.mem.set_f32(a + 0x1d4, 5.0);
        e.register(FORM_IS_AUTOMATIC, |_, _| ret(0));
        middle_high_process_clear_automatic_shots_to_fire(&mut e, p, 1);
        assert_eq!(e.mem.f32(a + 0x1d4), 0.0);
        e.mem.set_f32(a + 0x1d4, 5.0);
        e.mem.set_u32(a + 0x118, 0);
        middle_high_process_clear_automatic_shots_to_fire(&mut e, p, 1);
        assert_eq!(e.mem.f32(a + 0x1d4), 0.0);
    }

    #[test]
    fn get_automatic_shots_to_fire_counts_the_intervals() {
        let mut e = engine();
        let p = automatic_weapon(&mut e);
        let a = p.addr();
        let cell = float_cell(&mut e, 1.0);
        e.register_double(FLOAT_SETTING_VALUE, move |_, _| ret(cell));
        e.register(FLOAT_C, |_, _| ret_f(0.1));
        e.register(VATS_PLAYER_UPDATE_MULT, |_, _| ret_f(30.0));
        e.register(VATS_TARGET_UPDATE_MULT, |_, _| ret_f(30.0));
        let player = actor(&mut e);
        e.set_global(PLAYER, player);
        let target = actor(&mut e);
        e.set_global(VATS_TARGET, target);
        let other = actor(&mut e);
        // Another actor: 0.1 off a zero counter, one interval of 2.0 back.
        assert_eq!(
            middle_high_process_get_automatic_shots_to_fire(&mut e, p, other),
            1
        );
        assert!((e.mem.f32(a + 0x1d4) - 1.9).abs() < 1e-6);
        // The player and the VATS target are scaled by 30: 3.0 off.
        e.mem.set_f32(a + 0x1d4, 0.0);
        assert_eq!(
            middle_high_process_get_automatic_shots_to_fire(&mut e, p, player),
            2
        );
        e.mem.set_f32(a + 0x1d4, 0.0);
        assert_eq!(
            middle_high_process_get_automatic_shots_to_fire(&mut e, p, target),
            2
        );
        // Not automatic: 0.
        e.register(FORM_IS_AUTOMATIC, |_, _| ret(0));
        assert_eq!(
            middle_high_process_get_automatic_shots_to_fire(&mut e, p, other),
            0
        );
    }

    /// A process with sandbox data (a 0x80 byte block, type `kind` at +8,
    /// state `state` at +4) and a package behind slot +0x22C.
    fn sandbox_scene(
        e: &mut Engine,
        kind: u32,
        state: u32,
    ) -> (Ptr<MiddleHighProcess>, u32, u32, u32) {
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(SANDBOX_SLACK, 150.0f32);
        let p = process(e);
        let target = actor(e);
        let data = e.mem.alloc(0x80);
        e.mem.set_u32(data + 4, state);
        e.mem.set_u32(data + 8, kind);
        let package = thing(e);
        e.register_double(PROCESS_PACKAGE, move |_, _| ret(data));
        slot_returns(e, PROCESS_SLOTS, 0x22c, package);
        slot_returns(e, PROCESS_SLOTS, 0x27c, package);
        e.register(PACKAGE_RADIUS, |_, _| ret(100));
        e.register(SANDBOX_AT_TARGET, |_, _| ret(1));
        e.register(PATHING_SPEED_LIMIT, |_, _| ret_f(5.0));
        e.register(MAX_OF_FLOATS, |_, a| {
            ret_f(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        (p, target, data, package)
    }

    #[test]
    fn process_sand_box_without_data_ends_the_package() {
        let mut e = engine();
        let p = process(&mut e);
        let target = actor(&mut e);
        middle_high_process_process_sand_box(&mut e, p, target);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x288),
            vec![vec![p.addr(), target, 1]]
        );
    }

    #[test]
    fn process_sand_box_starts_and_reaches_the_location() {
        let mut e = engine();
        let (p, target, data, package) = sandbox_scene(&mut e, 0, 0);
        e.register(PACKAGE_LOCATION, |_, _| ret(0x44));
        e.register(SANDBOX_REACHED, |_, _| ret(1));
        middle_high_process_process_sand_box(&mut e, p, target);
        assert_eq!(
            calls(&e, SANDBOX_AT_TARGET),
            vec![vec![data, target, 100.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, PACKAGE_LOCATION), vec![vec![package]]);
        assert_eq!(calls(&e, SANDBOX_START), vec![vec![data, target, 0x44]]);
        assert_eq!(calls(&e, SANDBOX_REACHED), vec![vec![data, target, 1]]);
        assert_eq!(e.mem.u32(data + 4), 2);

        // In furniture state 4 or 9 the actor's slot +0x418 runs instead.
        let mut e = engine();
        let (p, target, data, _) = sandbox_scene(&mut e, 0, 1);
        slot_returns(&mut e, ACTOR_SLOTS, 0x214, 9);
        middle_high_process_process_sand_box(&mut e, p, target);
        assert_eq!(calls(&e, ACTOR_SLOTS + 0x418), vec![vec![target]]);
        assert!(calls(&e, SANDBOX_REACHED).is_empty());
        assert_eq!(e.mem.u32(data + 4), 1);
    }

    #[test]
    fn process_sand_box_builds_a_path_when_not_at_the_target() {
        let mut e = engine();
        let (p, target, data, package) = sandbox_scene(&mut e, 0, 1);
        e.register(SANDBOX_AT_TARGET, |_, _| ret(0));
        e.register(PACKAGE_GET_LOCATION_WORLD, |_, _| ret(0x11));
        e.register(PACKAGE_GET_LOCATION_CELL, |_, _| ret(0x22));
        e.register(PACKAGE_GET_LOCATION_COORD, |_, a| ret(a[1]));
        middle_high_process_process_sand_box(&mut e, p, target);
        let built = calls(&e, ACTOR_BUILD_REQUEST);
        assert_eq!(built.len(), 1);
        let coordinate = calls(&e, PACKAGE_GET_LOCATION_COORD)[0][1];
        // (actor, request, position, cell, worldspace, speed, 0): the speed
        // is the larger of the package radius and the speed limit.
        assert_eq!(built[0][0], target);
        assert_eq!(
            built[0][2..],
            [coordinate, 0x22, 0x11, 100.0f32.to_bits(), 0]
        );
        assert_eq!(
            calls(&e, ACTOR_SET_GOAL_FROM_REQUEST),
            vec![vec![target, built[0][1]]]
        );
        assert_eq!(calls(&e, PATHING_REQUEST_RELEASE), vec![vec![built[0][1]]]);
        assert_eq!(e.mem.u32(data + 4), 1);
        let _ = package;

        // A target farther than the radius plus 150.0 from the anchor is
        // forgotten and a path to the target built.
        let mut e = engine();
        let (p, target, data, package) = sandbox_scene(&mut e, 0, 1);
        let reference = thing(&mut e);
        e.register_double(SANDBOX_WORD_10, move |_, _| ret(reference));
        e.register(SANDBOX_WORD_3C, |_, _| ret(0x9000));
        e.register(REFERENCE_GET_DISTANCE_FROM_REFERENCE, |_, _| ret_f(500.0));
        middle_high_process_process_sand_box(&mut e, p, target);
        assert_eq!(
            calls(&e, SANDBOX_FORGET_TARGET),
            vec![vec![data, target, package]]
        );
        assert_eq!(calls(&e, ACTOR_BUILD_REQUEST).len(), 1);
        assert_eq!(calls(&e, ACTOR_BUILD_REQUEST)[0][2], 0);
        // Within the radius plus 150.0 nothing is rebuilt.
        e.register(REFERENCE_GET_DISTANCE_FROM_REFERENCE, |_, _| ret_f(200.0));
        e.call_log = Some(vec![]);
        middle_high_process_process_sand_box(&mut e, p, target);
        assert!(calls(&e, ACTOR_BUILD_REQUEST).is_empty());
    }

    #[test]
    fn process_sand_box_runs_the_action_of_its_type() {
        // (type, slot, arguments after the process and actor)
        let cases: [(u32, u32, &[u32]); 5] = [
            (0, 0x844, &[0x9000]),
            (2, 0x7bc, &[0x55]),
            (3, 0x84c, &[0, (-1.0f32).to_bits()]),
            (4, 0x868, &[0x9000]),
            (5, 0x850, &[0x9000]),
        ];
        for (kind, slot, extra) in cases {
            let mut e = engine();
            let (p, target, data, package) = sandbox_scene(&mut e, kind, 3);
            e.register(SANDBOX_WORD_3C, |_, _| ret(0x9000));
            e.register(SANDBOX_WORD_10, |_, _| ret(0x55));
            slot_returns(&mut e, PROCESS_SLOTS, slot, 1);
            middle_high_process_process_sand_box(&mut e, p, target);
            let mut expected = vec![p.addr(), target];
            expected.extend_from_slice(extra);
            assert_eq!(
                calls(&e, PROCESS_SLOTS + slot),
                vec![expected],
                "type {kind}"
            );
            if kind == 5 {
                // The action's success picks a new procedure.
                assert_eq!(
                    calls(&e, SANDBOX_PICK_PROCEDURE),
                    vec![vec![data, target, package]]
                );
                assert_eq!(e.mem.u32(data + 4), 2);
            } else {
                assert_eq!(e.mem.u32(data + 4), 3, "type {kind}");
            }
        }

        // A failing action forgets the target; a pathing actor then gets
        // slot +0x350 with the walk mode.
        let mut e = engine();
        let (p, target, data, package) = sandbox_scene(&mut e, 0, 3);
        e.register(SANDBOX_WORD_3C, |_, _| ret(0x9000));
        e.register(ACTOR_IS_PATHING, |_, _| ret(1));
        middle_high_process_process_sand_box(&mut e, p, target);
        assert_eq!(
            calls(&e, SANDBOX_FORGET_TARGET),
            vec![vec![data, target, package]]
        );
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x350),
            vec![vec![p.addr(), target, 0x101, 1]]
        );

        // State 2 with type 0 moves on to state 3 once the actor is in a
        // sitting state; a new procedure is picked when asked for.
        let mut e = engine();
        let (p, target, data, package) = sandbox_scene(&mut e, 0, 2);
        slot_returns(&mut e, ACTOR_SLOTS, 0x214, 3);
        e.register(SANDBOX_SHOULD_PICK, |_, _| ret(1));
        e.register(SANDBOX_MOVE_TO_NEXT, |_, _| ret(0));
        middle_high_process_process_sand_box(&mut e, p, target);
        assert_eq!(calls(&e, ACTOR_CLEAR_LOOK_AT_TARGET), vec![vec![target]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x244), vec![vec![p.addr()]]);
        assert_eq!(
            calls(&e, SANDBOX_PICK_PROCEDURE),
            vec![vec![data, target, package]]
        );
        assert_eq!(e.mem.u32(data + 4), 1);
    }

    #[test]
    fn fn_0092a980_is_false_for_types_2_and_3() {
        let mut e = engine();
        assert!(fn_0092a980(&mut e, 0));
        assert!(fn_0092a980(&mut e, 1));
        assert!(!fn_0092a980(&mut e, 2));
        assert!(!fn_0092a980(&mut e, 3));
        assert!(fn_0092a980(&mut e, 4));
        assert!(fn_0092a980(&mut e, u32::MAX));
    }

    #[test]
    fn process_use_furniture_takes_the_furniture() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let target = actor(&mut e);
        let furniture = thing(&mut e);
        // Not a furniture reference.
        assert!(!middle_high_process_process_use_furniture(
            &mut e, p, target, 0
        ));
        assert!(!middle_high_process_process_use_furniture(
            &mut e, p, target, furniture
        ));
        e.register(REFERENCE_IS_FURNITURE, |_, _| ret(1));
        e.register(FURNITURE_CAN_SIT_ON, |_, _| ret(1));
        slot_returns(&mut e, ACTOR_SLOTS, 0x218, 1);
        // A fresh actor takes it and enters the sitting state.
        assert!(middle_high_process_process_use_furniture(
            &mut e, p, target, furniture
        ));
        assert_eq!(e.mem.u32(a + 0x140), furniture);
        assert_eq!(calls(&e, CLEAR_FURNITURE_MARKER), vec![vec![a]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x12c), vec![vec![a, furniture]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x7c8), vec![vec![a, target, 0]]);

        // The furniture of another reference while sitting: slot +0x418.
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let target = actor(&mut e);
        let furniture = thing(&mut e);
        e.register(REFERENCE_IS_FURNITURE, |_, _| ret(1));
        e.mem.set_u32(a + 0x140, 0x1234);
        slot_returns(&mut e, ACTOR_SLOTS, 0x214, 4);
        assert!(middle_high_process_process_use_furniture(
            &mut e, p, target, furniture
        ));
        assert_eq!(calls(&e, ACTOR_SLOTS + 0x418), vec![vec![target]]);
        assert_eq!(e.mem.u32(a + 0x140), 0x1234);

        // In the sleeping state (no CanSitOn) the face animation data's
        // slot +0xD8 runs with (1, 0).
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let target = actor(&mut e);
        let furniture = thing(&mut e);
        let face = thing(&mut e);
        e.register(REFERENCE_IS_FURNITURE, |_, _| ret(1));
        e.register_double(ACTOR_GET_FACE_ANIMATION_DATA, move |_, _| ret(face));
        e.mem.set_u32(a + 0x140, furniture);
        slot_returns(&mut e, ACTOR_SLOTS, 0x214, 9);
        assert!(middle_high_process_process_use_furniture(
            &mut e, p, target, furniture
        ));
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x118), vec![vec![a, 1]]);
        assert_eq!(calls(&e, OBJECT_SLOTS + 0xd8), vec![vec![face, 1, 0]]);

        // A dropped furniture clears the marker and ends the package.
        e.register(PROCESS_FURNITURE_DROP, |_, _| ret(1));
        assert!(!middle_high_process_process_use_furniture(
            &mut e, p, target, furniture
        ));
        assert_eq!(e.mem.u32(a + 0x140), 0);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x294), vec![vec![a, target]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x288), vec![vec![a, target, 1]]);
        assert_eq!(
            calls(&e, FURNITURE_MARK_SET_HEADING),
            vec![vec![a + 0x148, 0]]
        );
    }

    #[test]
    fn process_movement_blocked_compares_the_hour() {
        let mut e = engine();
        e.register(FLOAT_ABS_WRAPPER, |_, a| {
            ret_f(f64::from(f32::from_bits(a[0]).abs()))
        });
        e.register(CALENDAR_GET_HOUR, |_, _| ret_f(12.0));
        e.register(PROCESS_TIME_VALUE, |_, _| ret_f(11.0));
        let p = process(&mut e);
        let a = p.addr();
        let target = actor(&mut e);
        e.mem.set_u32(a + 4, 0x77);
        // An hour apart: the package ends and is marked created.
        middle_high_process_process_movement_blocked(&mut e, p, target);
        assert_eq!(calls(&e, PROCESS_TIME_VALUE), vec![vec![a + 4]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x288), vec![vec![a, target, 1]]);
        assert_eq!(calls(&e, PACKAGE_SET_IS_CREATED), vec![vec![0x77, 1]]);
        assert!(calls(&e, ACTOR_STOP_SPECIAL_IDLE).is_empty());

        // Slot +0x20C true reads the other area; exactly 0.05 still counts.
        slot_returns(&mut e, PROCESS_SLOTS, 0x20c, 1);
        e.register(CALENDAR_GET_HOUR, |_, _| ret_f(f64::from(0.05f32)));
        e.register(PROCESS_TIME_VALUE, |_, _| ret_f(0.0));
        e.call_log = Some(vec![]);
        middle_high_process_process_movement_blocked(&mut e, p, target);
        assert_eq!(calls(&e, PROCESS_TIME_VALUE), vec![vec![a + 0xe4]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x288).len(), 1);

        // Close together: only the special idle stop.
        e.register(CALENDAR_GET_HOUR, |_, _| ret_f(1.0));
        e.register(PROCESS_TIME_VALUE, |_, _| ret_f(1.01));
        e.call_log = Some(vec![]);
        middle_high_process_process_movement_blocked(&mut e, p, target);
        assert!(calls(&e, PROCESS_SLOTS + 0x288).is_empty());
        assert_eq!(calls(&e, ACTOR_STOP_SPECIAL_IDLE), vec![vec![target]]);
    }

    #[test]
    fn process_patrol_runs_the_state_slot() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let target = actor(&mut e);
        // Without patrol data: slot +0x284 with -1.
        middle_high_process_process_patrol(&mut e, p, target);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x284), vec![vec![a, u32::MAX]]);
        let data = e.mem.alloc(0x20);
        e.register_double(PROCESS_PACKAGE, move |_, _| ret(data));
        for (state, slot) in [(0u32, 0x854u32), (1, 0x85c), (2, 0x864)] {
            e.mem.set_u32(data + 4, state);
            middle_high_process_process_patrol(&mut e, p, target);
            let ran = calls(&e, PROCESS_SLOTS + slot);
            assert_eq!(ran, vec![vec![a, target, data]], "state {state}");
        }
        e.mem.set_u32(data + 4, 3);
        e.call_log = Some(vec![]);
        middle_high_process_process_patrol(&mut e, p, target);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .all(|(addr, _)| *addr != PROCESS_SLOTS + 0x854));
        // An actor in a furniture state at another patrol reference.
        slot_returns(&mut e, ACTOR_SLOTS, 0x214, 4);
        e.register(PATROL_GET_CURRENT_REFERENCE, |_, _| ret(0x4321));
        e.call_log = Some(vec![]);
        middle_high_process_process_patrol(&mut e, p, target);
        assert_eq!(calls(&e, ACTOR_SLOTS + 0x418), vec![vec![target]]);
        assert!(calls(&e, PROCESS_SLOTS + 0x864).is_empty());
    }

    #[test]
    fn process_patrol_init_starts_or_ends() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let target = actor(&mut e);
        let package = thing(&mut e);
        slot_returns(&mut e, PROCESS_SLOTS, 0x22c, package);
        assert!(!middle_high_process_process_patrol_init(
            &mut e, p, target, 0x5000
        ));
        assert_eq!(calls(&e, PATROL_INIT), vec![vec![0x5000, target, package]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x284), vec![vec![a, u32::MAX]]);
        e.register(PATROL_INIT, |_, _| ret(1));
        assert!(middle_high_process_process_patrol_init(
            &mut e, p, target, 0x5000
        ));
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x858),
            vec![vec![a, target, 0x5000]]
        );
    }

    #[test]
    fn process_patrol_start_travel_builds_the_path() {
        let mut e = engine();
        e.set_global(PATROL_START_SPEED, 10.0f32);
        e.set_global(ACTIVATE_FORM_A, 0x1234u32);
        e.register(PATHING_SPEED_LIMIT, |_, _| ret_f(3.0));
        e.register(MAX_OF_FLOATS, |_, a| {
            ret_f(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        e.register(PACKAGE_RADIUS, |_, _| ret(100));
        e.register(ACTOR_IS_PATHING_COMPLETE, |_, _| ret(1));
        let p = process(&mut e);
        let a = p.addr();
        let target = actor(&mut e);
        let package = thing(&mut e);
        let reference = thing(&mut e);
        slot_returns(&mut e, PROCESS_SLOTS, 0x27c, package);
        let data = e.mem.alloc(0x20);
        // No reference: slot +0x284 with -1.
        middle_high_process_process_patrol_start_travel_to_next_ref(&mut e, p, target, data);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x284), vec![vec![a, u32::MAX]]);
        e.register_double(PATROL_GET_CURRENT_REFERENCE, move |_, _| ret(reference));
        e.register(REFERENCE_GET_WORLD_SPACE, |_, _| ret(0x11));
        e.register(REFERENCE_CELL, |_, _| ret(0x22));
        middle_high_process_process_patrol_start_travel_to_next_ref(&mut e, p, target, data);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x12c), vec![vec![a, reference]]);
        let built = calls(&e, ACTOR_BUILD_REQUEST);
        // The package radius (100) beats the speed limit (3.0).
        assert_eq!(built[0][3..], [0x22, 0x11, 100.0f32.to_bits(), 0]);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(
            calls(&e, ACTOR_SET_GOAL_FROM_REQUEST),
            vec![vec![target, built[0][1]]]
        );
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x860),
            vec![vec![a, target, data]]
        );
        assert!(calls(&e, PATHING_REQUEST_SET_FLOAT).is_empty());

        // A type 0x30 base form travels at 10.0 and gets the float.
        let plus_24 = e.mem.alloc(0x10);
        e.mem.set_f32(plus_24 + 8, 2.5);
        e.register_double(REFERENCE_PLUS_24, move |_, _| ret(plus_24));
        e.register(FORM_TYPE, |_, _| ret(0x30));
        e.call_log = Some(vec![]);
        middle_high_process_process_patrol_start_travel_to_next_ref(&mut e, p, target, data);
        let built = calls(&e, ACTOR_BUILD_REQUEST);
        assert_eq!(built[0][5], 10.0f32.to_bits());
        assert_eq!(
            calls(&e, PATHING_REQUEST_SET_FLOAT),
            vec![vec![built[0][1], 2.5f32.to_bits()]]
        );
    }

    /// A patrol scene: the current reference, a package and patrol data.
    fn patrol_scene(e: &mut Engine) -> (Ptr<MiddleHighProcess>, u32, u32, u32, u32) {
        e.set_global(PATROL_RUN_DISTANCE, 64.0f32);
        e.set_global(PATROL_DEFAULT_RADIUS, 32.0f32);
        e.set_global(PATH_PROGRESS_LIMIT, 0.925f32);
        let p = process(e);
        let target = actor(e);
        let reference = thing(e);
        let package = thing(e);
        let data = e.mem.alloc(0x20);
        e.register_double(PATROL_GET_CURRENT_REFERENCE, move |_, _| ret(reference));
        slot_returns(e, PROCESS_SLOTS, 0x27c, package);
        slot_returns(e, PROCESS_SLOTS, 0x22c, package);
        e.register(PACKAGE_RADIUS, |_, _| ret(10));
        e.register(MAX_OF_FLOATS, |_, a| {
            ret_f(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        (p, target, reference, package, data)
    }

    /// Makes the patrol offset to the reference `length` (squared, 2D) long
    /// and `height` high.
    fn patrol_offset_is(e: &mut Engine, length: f32, height: f32) {
        e.register_double(POINT_SUBTRACT, move |e, a| {
            e.mem.set_f32(a[1] + 8, height);
            ret(a[1])
        });
        e.register_double(VECTOR_LENGTH_2D_SQUARED, move |_, _| {
            ret_f(f64::from(length))
        });
    }

    #[test]
    fn patrol_traveling_waiting_on_the_path_keeps_going() {
        let mut e = engine();
        let (p, target, _, _, data) = patrol_scene(&mut e);
        e.register(ACTOR_BYTE_70, |_, _| ret(1));
        middle_high_process_process_patrol_traveling_to_next_ref(&mut e, p, target, data);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x858),
            vec![vec![p.addr(), target, data]]
        );
        assert!(calls(&e, PROCESS_SLOTS + 0x350).is_empty());
    }

    #[test]
    fn patrol_traveling_sets_the_move_mode() {
        let mut e = engine();
        let (p, target, reference, package, data) = patrol_scene(&mut e);
        let a = p.addr();
        e.register(PACKAGE_ALLOWS_SPEED, |_, _| ret(1));
        e.register(REFERENCE_GET_DISTANCE_FROM_REFERENCE, |_, _| ret_f(100.0));
        e.register(PACKAGE_GET_RADIUS_TO_LOCATION, |_, _| ret_f(3.0));
        slot_returns(&mut e, PROCESS_SLOTS, 0x34c, 0x205);
        // Closer than 64.0 would not run: here 100.0 does, at speed 3.0.
        e.register(ACTOR_IS_PATHING_COMPLETE, |_, _| ret(1));
        patrol_offset_is(&mut e, 0.0, 0.0);
        middle_high_process_process_patrol_traveling_to_next_ref(&mut e, p, target, data);
        assert_eq!(calls(&e, PACKAGE_ALLOWS_SPEED), vec![vec![package]]);
        assert_eq!(
            calls(&e, PACKAGE_GET_RADIUS_TO_LOCATION),
            vec![vec![package, target, 0]]
        );
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x34c),
            vec![vec![
                a,
                target,
                100.0f32.to_bits(),
                3.0f32.to_bits(),
                6.0f32.to_bits(),
                0,
                0
            ]]
        );
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x350),
            vec![vec![a, target, 0x205, 1]]
        );

        // Nearer than 64.0: the plain mode 0x101 without running.
        e.register(REFERENCE_GET_DISTANCE_FROM_REFERENCE, |_, _| ret_f(10.0));
        e.call_log = Some(vec![]);
        middle_high_process_process_patrol_traveling_to_next_ref(&mut e, p, target, data);
        assert!(calls(&e, PROCESS_SLOTS + 0x34c).is_empty());
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x350),
            vec![vec![a, target, 0x101, 0]]
        );
        let _ = reference;
    }

    #[test]
    fn patrol_traveling_uses_a_furniture_reference() {
        let mut e = engine();
        let (p, target, reference, _, data) = patrol_scene(&mut e);
        let a = p.addr();
        e.register(FORM_TYPE, |_, _| ret(0x27));
        e.mem.set_u32(a + 0x140, reference);
        slot_returns(&mut e, PROCESS_SLOTS, 0x4bc, 3);
        middle_high_process_process_patrol_traveling_to_next_ref(&mut e, p, target, data);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x844),
            vec![vec![a, target, reference]]
        );
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x860),
            vec![vec![a, target, data]]
        );
        // A furniture state other than 3 or 8 does not leave.
        slot_returns(&mut e, PROCESS_SLOTS, 0x4bc, 5);
        e.call_log = Some(vec![]);
        middle_high_process_process_patrol_traveling_to_next_ref(&mut e, p, target, data);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x844).len(), 1);
        assert!(calls(&e, PROCESS_SLOTS + 0x860).is_empty());
        assert!(calls(&e, PROCESS_SLOTS + 0x858).is_empty());
    }

    #[test]
    fn patrol_traveling_decides_arrival_from_the_offset() {
        // Path complete: arrived inside the radius (32.0, squared 1024),
        // still travelling outside it.
        let mut e = engine();
        let (p, target, _, _, data) = patrol_scene(&mut e);
        let a = p.addr();
        e.register(ACTOR_IS_PATHING_COMPLETE, |_, _| ret(1));
        patrol_offset_is(&mut e, 100.0, 0.0);
        middle_high_process_process_patrol_traveling_to_next_ref(&mut e, p, target, data);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x860),
            vec![vec![a, target, data]]
        );
        patrol_offset_is(&mut e, 2000.0, 0.0);
        e.call_log = Some(vec![]);
        middle_high_process_process_patrol_traveling_to_next_ref(&mut e, p, target, data);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x858),
            vec![vec![a, target, data]]
        );
        // More than 180 high is not "travelling": the leg counts as done.
        patrol_offset_is(&mut e, 2000.0, 200.0);
        e.call_log = Some(vec![]);
        middle_high_process_process_patrol_traveling_to_next_ref(&mut e, p, target, data);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x860).len(), 1);

        // Path not complete: less than 0.925 done returns at once.
        let mut e = engine();
        let (p, target, _, _, data) = patrol_scene(&mut e);
        let a = p.addr();
        e.register(PATH_PROGRESS, |_, _| ret_f(0.5));
        patrol_offset_is(&mut e, 100.0, 0.0);
        middle_high_process_process_patrol_traveling_to_next_ref(&mut e, p, target, data);
        assert!(calls(&e, PROCESS_SLOTS + 0x860).is_empty());
        assert!(calls(&e, PROCESS_SLOTS + 0x858).is_empty());
        // Nearly there, close to the reference and without a wait: arrived.
        e.register(PATH_PROGRESS, |_, _| ret_f(0.99));
        middle_high_process_process_patrol_traveling_to_next_ref(&mut e, p, target, data);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x860),
            vec![vec![a, target, data]]
        );
        // A pending wait keeps the actor where it is.
        let wait = e.mem.alloc(8);
        e.register_double(EXTRA_LIST_TYPE_6F_WORD, move |_, _| ret(wait));
        e.register(FLOAT_AT_START, |_, _| ret_f(2.0));
        e.call_log = Some(vec![]);
        middle_high_process_process_patrol_traveling_to_next_ref(&mut e, p, target, data);
        assert!(calls(&e, PROCESS_SLOTS + 0x860).is_empty());
        // Too far from the reference (margin 82): also keeps going.
        e.register(EXTRA_LIST_TYPE_6F_WORD, |_, _| ret(0));
        patrol_offset_is(&mut e, 10000.0, 0.0);
        e.call_log = Some(vec![]);
        middle_high_process_process_patrol_traveling_to_next_ref(&mut e, p, target, data);
        assert!(calls(&e, PROCESS_SLOTS + 0x860).is_empty());
        assert!(calls(&e, PROCESS_SLOTS + 0x858).is_empty());
    }

    #[test]
    fn fn_0092b440_reads_the_byte_at_5() {
        let mut e = engine();
        let object = e.mem.alloc(0x10);
        e.mem.set_u8(object + 5, 0xfe);
        assert_eq!(fn_0092b440(&mut e, object), 0xfe);
    }

    #[test]
    fn fn_0092b550_asks_the_area_at_4() {
        let mut e = engine();
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + 4 + 8, 0x4444);
        assert_eq!(fn_0092b550(&mut e, object), 0x4444);
        assert_eq!(calls(&e, ITEM_CHANGE_FORM), vec![vec![object + 4]]);
    }

    #[test]
    fn patrol_start_at_ref_behavior_prepares_the_wait() {
        let mut e = engine();
        let (p, target, reference, _, data) = patrol_scene(&mut e);
        let a = p.addr();
        let extra = e.mem.alloc(0x20);
        e.mem.set_u32(extra + 12, 0x4444);
        let acquired = thing(&mut e);
        let patrol_data = e.mem.alloc(0x10);
        e.register_double(EXTRA_LIST_TYPE_6F_WORD, move |_, _| ret(extra));
        e.register_double(WORD_68, move |_, _| ret(acquired));
        e.register_double(PROCESS_PACKAGE, move |_, _| ret(patrol_data));
        e.register(REFERENCE_EXTRA_LIST, |_, _| ret(0x600));
        e.mem.set_f32(a + 0x198, 5.0);
        middle_high_process_process_patrol_start_at_ref_behavior(&mut e, p, target, data);
        assert_eq!(
            calls(&e, LOW_PROCESS_PATROL_START_AT_REF),
            vec![vec![a, target, data]]
        );
        assert_eq!(calls(&e, ACTOR_CLEAR_LOOK_AT_TARGET), vec![vec![target]]);
        // Not a type 0x30 reference: the idle timer is reset.
        assert_eq!(e.mem.f32(a + 0x198), 0.0);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x2a4),
            vec![vec![a, target, 0x4444, 0, 0, 1, 0]]
        );
        assert_eq!(
            calls(&e, SCRIPT_RUN),
            vec![vec![patrol_data, target, 0, 0, 0]]
        );
        let _ = reference;

        // A type 0x30 reference keeps the timer; the acquire object's
        // slot +0x30C answering true skips slot +0x2A4; no extra, no more.
        let mut e = engine();
        let (p, target, _, _, data) = patrol_scene(&mut e);
        let a = p.addr();
        e.register(FORM_TYPE, |_, _| ret(0x30));
        e.mem.set_f32(a + 0x198, 5.0);
        middle_high_process_process_patrol_start_at_ref_behavior(&mut e, p, target, data);
        assert_eq!(e.mem.f32(a + 0x198), 5.0);
        assert!(calls(&e, PROCESS_SLOTS + 0x2a4).is_empty());
        assert!(calls(&e, SCRIPT_RUN).is_empty());
    }

    #[test]
    fn patrol_at_ref_waits_and_moves_on() {
        // No reference: the next one, or the end of the package.
        let mut e = engine();
        let (p, target, _, _, data) = patrol_scene(&mut e);
        let a = p.addr();
        e.register(PATROL_GET_CURRENT_REFERENCE, |_, _| ret(0));
        e.register(PATROL_SET_NEXT_REFERENCE, |_, _| ret(1));
        middle_high_process_process_patrol_at_ref(&mut e, p, target, data);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x858),
            vec![vec![a, target, data]]
        );
        e.register(PATROL_SET_NEXT_REFERENCE, |_, _| ret(0));
        middle_high_process_process_patrol_at_ref(&mut e, p, target, data);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x284), vec![vec![a, u32::MAX]]);

        // At the reference: too far away (inside 32.0 squared otherwise)
        // starts the next patrol leg.
        let mut e = engine();
        let (p, target, _, _, data) = patrol_scene(&mut e);
        let a = p.addr();
        e.register(ACTOR_IS_PATHING_COMPLETE, |_, _| ret(1));
        e.register(FLOAT_ABS, |_, a| {
            ret_f(f64::from(f32::from_bits(a[0]).abs()))
        });
        patrol_offset_is(&mut e, 5000.0, 0.0);
        middle_high_process_process_patrol_at_ref(&mut e, p, target, data);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x858),
            vec![vec![a, target, data]]
        );
        // Or more than 180 above.
        patrol_offset_is(&mut e, 0.0, -200.0);
        e.call_log = Some(vec![]);
        middle_high_process_process_patrol_at_ref(&mut e, p, target, data);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x858).len(), 1);

        // Waiting there: type 0x30 gets slot +0x868 and the elapsed time
        // grows by the frame time until it reaches the wait.
        let mut e = engine();
        let (p, target, reference, _, data) = patrol_scene(&mut e);
        let a = p.addr();
        let wait = e.mem.alloc(8);
        e.register(FORM_TYPE, |_, _| ret(0x30));
        e.register(REFERENCE_EXTRA_LIST, |_, _| ret(0x600));
        e.register_double(EXTRA_LIST_TYPE_6F_WORD, move |_, _| ret(wait));
        e.register(FLOAT_AT_START, |_, _| ret_f(2.0));
        e.register(FLOAT_C, |_, _| ret_f(0.75));
        e.register(PATROL_SET_NEXT_REFERENCE, |_, _| ret(0));
        middle_high_process_process_patrol_at_ref(&mut e, p, target, data);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x868),
            vec![vec![a, target, reference]]
        );
        assert_eq!(e.mem.f32(data + 8), 0.75);
        assert!(calls(&e, PROCESS_SLOTS + 0x288).is_empty());
        middle_high_process_process_patrol_at_ref(&mut e, p, target, data);
        assert_eq!(e.mem.f32(data + 8), 1.5);
        middle_high_process_process_patrol_at_ref(&mut e, p, target, data);
        assert_eq!(e.mem.f32(data + 8), 2.25);
        // Waited long enough: no next reference ends it with slot +0x288.
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x288), vec![vec![a, target, 1]]);
        // A special idle still playing holds it.
        e.register(ACTOR_PLAYING_LOWER_BODY_IDLE, |_, _| ret(1));
        e.call_log = Some(vec![]);
        middle_high_process_process_patrol_at_ref(&mut e, p, target, data);
        assert!(calls(&e, PROCESS_SLOTS + 0x288).is_empty());
        assert!(calls(&e, PATROL_SET_NEXT_REFERENCE).is_empty());
    }

    #[test]
    fn fn_0092b7f0_clears_the_damage_nodes() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        for offset in [0x1dcu32, 0x1e0, 0x214, 0x218, 0x21c] {
            e.mem.set_u32(a + offset, 0xffff);
        }
        fn_0092b7f0(&mut e, p);
        assert_eq!(calls(&e, MEMSET), vec![vec![a + 0x1dc, 0, 0x3c]]);
        assert_eq!(e.mem.u32(a + 0x218), 0);
        assert_eq!(e.mem.u32(a + 0x21c), 0);
    }

    #[test]
    fn store_damage_nodes_finds_the_nodes_under_the_root() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let entries = e.mem.alloc(0x40);
        e.mem.set_u32(entries + 4, 0x701);
        e.mem.set_u32(entries + 12, 0x704);
        e.register_double(PATH_LOCATION, move |_, _| ret(entries));
        // The node name of entry 0x704 is missing: its node is 0.
        e.register(NODE_NAME, |_, a| {
            ret(if a[0] == 0x704 { 0 } else { a[0] + 0x10 })
        });
        e.register(FIND_NODE, |_, a| ret(a[0] + a[1]));
        e.register(NODE_SLOT, |_, a| ret(a[0] - 0x700));
        e.register(BODY_PART_DATA_GET_BODY_PART, |_, a| ret(0x900 + a[1]));
        e.register(BODY_PART_NODE_NAME, |_, a| ret(a[0] + 1));
        e.mem.set_u32(a + 0x1dc + 16, 0xdead);
        middle_high_process_store_damage_nodes(&mut e, p, 0x8000, 0x5000);
        assert_eq!(e.mem.u32(a + 0x1dc + 4), 0x8000 + 0x711);
        assert_eq!(e.mem.u32(a + 0x1dc + 16), 0);
        // Body parts 1 and 0 give the two dismember nodes.
        assert_eq!(e.mem.u32(a + 0x218), 0x8000 + 0x901 + 1);
        assert_eq!(e.mem.u32(a + 0x21c), 0x8000 + 0x900 + 1);
        assert_eq!(
            calls(&e, BODY_PART_DATA_GET_BODY_PART),
            vec![vec![0x5000, 1], vec![0x5000, 0]]
        );

        // Without data no body part is asked for.
        let mut e = engine();
        let p = process(&mut e);
        let entries = e.mem.alloc(0x40);
        e.register_double(PATH_LOCATION, move |_, _| ret(entries));
        middle_high_process_store_damage_nodes(&mut e, p, 0x8000, 0);
        assert!(calls(&e, BODY_PART_DATA_GET_BODY_PART).is_empty());
    }

    #[test]
    fn store_dismembered_limb_node_clears_the_limb_slots() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        e.mem.set_u32(a + 0x218, 5);
        e.mem.set_u32(a + 0x21c, 6);
        middle_high_process_store_dismembered_limb_node(&mut e, p, 2, 0x77);
        assert_eq!(e.mem.u32(a + 0x1dc + 8), 0x77);
        assert_eq!((e.mem.u32(a + 0x218), e.mem.u32(a + 0x21c)), (5, 6));
        middle_high_process_store_dismembered_limb_node(&mut e, p, 1, 0x78);
        assert_eq!(e.mem.u32(a + 0x1dc + 4), 0x78);
        assert_eq!((e.mem.u32(a + 0x218), e.mem.u32(a + 0x21c)), (0, 6));
        middle_high_process_store_dismembered_limb_node(&mut e, p, 0, 0x79);
        assert_eq!(e.mem.u32(a + 0x1dc), 0x79);
        assert_eq!((e.mem.u32(a + 0x218), e.mem.u32(a + 0x21c)), (0, 0));
    }

    #[test]
    fn get_damage_node_reads_the_slot() {
        let mut e = engine();
        let p = process(&mut e);
        e.mem.set_u32(p.addr() + 0x1dc + 12, 0x99);
        assert_eq!(middle_high_process_get_damage_node(&mut e, p, 3), 0x99);
        assert_eq!(middle_high_process_get_damage_node(&mut e, p, 0), 0);
    }

    #[test]
    fn get_weapon_condition_scales_and_rounds_the_health() {
        let mut e = engine();
        e.set_global(CONDITION_SCALE, 0.1f64);
        e.register(FLOAT_ROUND, |_, a| ret_f(f64::from(f32::from_bits(a[0]))));
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            ret(value as i32 as u32)
        });
        e.register(ITEM_CHANGE_GET_ITEM_HEALTH, |_, _| ret_f(50.0));
        let p = process(&mut e);
        let a = p.addr();
        // No weapon item: 0.
        assert_eq!(middle_high_process_get_weapon_condition(&mut e, p, 0x70), 0);
        let item = e.mem.alloc(0x20);
        e.mem.set_u32(item + 8, 0x99);
        e.mem.set_u32(a + 0x114, item);
        // 50.0 * 0.1 = 5, minus one.
        assert_eq!(middle_high_process_get_weapon_condition(&mut e, p, 0x70), 4);
        assert_eq!(calls(&e, ITEM_CHANGE_GET_ITEM_HEALTH), vec![vec![item, 1]]);
        // 10.0 * 0.1 = 1 stays 1.
        e.register(ITEM_CHANGE_GET_ITEM_HEALTH, |_, _| ret_f(10.0));
        assert_eq!(middle_high_process_get_weapon_condition(&mut e, p, 0x70), 1);
        // A form with flag 0x20: slot +0x39C replaces the health.
        e.register(WEAPON_FORM_FLAG, |_, _| ret(1));
        e.register(WEAPON_ATTRIBUTE, |_, _| ret(0x33));
        e.register(ACTOR_EXTRA_OBJECT, |_, _| ret(0x44));
        e.register_double(PROCESS_SLOTS + 0x39c, |_, _| ret_f(80.0));
        assert_eq!(middle_high_process_get_weapon_condition(&mut e, p, 0x70), 7);
        assert_eq!(calls(&e, WEAPON_ATTRIBUTE), vec![vec![0x99]]);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x39c),
            vec![vec![a, 0x44, 0x33, 0x70]]
        );
    }

    #[test]
    fn update_armor_worn_sets_the_worn_flags() {
        let mut e = engine();
        e.register(ACTOR_GET_ANIMATION, |_, _| ret(0x61));
        e.register(ACTOR_WORD_1B4, |_, _| ret(0x62));
        let p = process(&mut e);
        let a = p.addr();
        let target = actor(&mut e);
        // One worn power armor (type 0x18 form) with biped flag 4.
        let entry = e.mem.alloc(0x20);
        let form = e.mem.alloc(0x100);
        e.mem.set_u32(entry + 8, form);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, entry);
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder, node);
        e.register_double(GET_INVENTORY_CHANGES, move |_, _| ret(holder));
        e.register(ITEM_CHANGE_GET_WORN, |_, _| ret(1));
        e.register(FORM_TYPE, |_, _| ret(0x18));
        e.register(BIPED_FLAG_4, |_, _| ret(1));
        e.register(BIPED_IS_POWER_ARMOR, |_, _| ret(1));
        e.register(BIPED_FILLS_SLOT, |_, a| ret(u32::from(a[1] == 0)));
        slot_returns(&mut e, ACTOR_SLOTS, 0x218, 1);
        e.mem.set_u32(a + 0x114, 0x55);
        e.mem.set_u8(a + 0x127, 1);
        e.mem.set_u8(a + 0x12a, 0);
        middle_high_process_update_armor_worn(&mut e, p, target);
        // Flag 4 -> +0x12A; power armor filling slot 0 -> +0x129.
        assert_eq!(e.mem.u8(a + 0x12a), 1);
        assert_eq!(e.mem.u8(a + 0x129), 1);
        assert_eq!(e.mem.u8(a + 0x128), 0);
        assert_eq!(e.mem.u8(a + 0x127), 0);
        // +0x12A changed and an item is held: slot +0x1CC refreshes.
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x1cc),
            vec![vec![a, 0, 0x62, 0x61, target]]
        );

        // Nothing worn and nothing changed: no refresh; the gate object
        // (bit 2) true also blocks it.
        let mut e = engine();
        let p = process(&mut e);
        let target = actor(&mut e);
        let a = p.addr();
        let holder = e.mem.alloc(8);
        e.register_double(GET_INVENTORY_CHANGES, move |_, _| ret(holder));
        slot_returns(&mut e, ACTOR_SLOTS, 0x218, 1);
        e.mem.set_u32(a + 0x114, 0x55);
        e.mem.set_u8(a + 0x12a, 1);
        middle_high_process_update_armor_worn(&mut e, p, target);
        assert_eq!(e.mem.u8(a + 0x12a), 0);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x1cc).len(), 1);
        e.register(SAVE_LOAD_FLAG_2, |_, _| ret(1));
        e.mem.set_u8(a + 0x12a, 1);
        e.call_log = Some(vec![]);
        middle_high_process_update_armor_worn(&mut e, p, target);
        assert!(calls(&e, PROCESS_SLOTS + 0x1cc).is_empty());
    }

    #[test]
    fn try_reserve_projectile_clones_builds_the_clone() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        let target = actor(&mut e);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(first + 8, 0x4000);
        e.mem.set_u32(second + 8, 0x5000);
        let entry = e.mem.alloc(0x80);
        let object = thing(&mut e);
        e.mem.set_u32(a + 0x11c, 0x77);
        slot_returns(&mut e, PROCESS_SLOTS, 0x148, first);
        slot_returns(&mut e, PROCESS_SLOTS, 0x14c, second);
        e.register(QUEUED_FILE_DONE, |_, _| ret(1));
        e.register_double(FORM_PROJECTILE, move |_, _| ret(entry));
        e.register(PROJECTILE_GET_PRE_CLONE_COUNT, |_, _| ret(3));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(BODY_PART_NODE_NAME, |_, a| ret(a[0] + 1));
        e.register(PROJECTILE_CLONE_CONSTRUCT, |_, a| ret(a[0]));
        // The entry's object (at +0x3C) answers a handle through its slot
        // +0x14: here the double stores 0x6000 into the handle.
        e.put_vtable(
            0x0503_0000,
            &(0..VTABLE_SIZE)
                .step_by(4)
                .map(|s| OBJECT_SLOTS + s)
                .collect::<Vec<u32>>(),
        );
        e.mem.set_u32(entry + 0x3c, 0x0503_0000);
        e.register(OBJECT_SLOTS + 0x14, |_, a| {
            assert_eq!(a.len(), 1);
            ret(0x7777)
        });
        // The model loader stores 0x6000 into the handle (its last word).
        e.register(MODEL_LOADER_FIND_MODEL, |e, a| {
            e.mem.set_u32(a[2], 0x6000);
            Ret::default()
        });
        e.register(POINTER_INIT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        middle_high_process_try_reserve_projectile_clones(&mut e, p, target);
        assert_eq!(
            calls(&e, PROJECTILE_GET_PRE_CLONE_COUNT),
            vec![vec![entry, 0x4000]]
        );
        let found = calls(&e, MODEL_LOADER_FIND_MODEL);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0][..2], [0, 0x7777]);
        let made = calls(&e, PROJECTILE_CLONE_CONSTRUCT);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0][1..], [0x6001, 3]);
        assert_eq!(
            calls(&e, CLONE_RESERVER_ASSIGN),
            vec![vec![a + 0x120, made[0][0]]]
        );
        assert_eq!(calls(&e, HANDLE_DESTRUCT).len(), 1);
        let _ = object;

        // A list that already has a first word is left alone.
        e.mem.set_u32(a + 0x120, 0x1234);
        e.call_log = Some(vec![]);
        middle_high_process_try_reserve_projectile_clones(&mut e, p, target);
        assert!(calls(&e, PROJECTILE_GET_PRE_CLONE_COUNT).is_empty());
    }

    #[test]
    fn clear_shooting_action_clears_the_look_at_and_the_byte() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        e.mem.set_u8(a + 0xe1, 1);
        middle_high_process_clear_shooting_action(&mut e, p, 0x5000);
        assert_eq!(calls(&e, ACTOR_CLEAR_LOOK_AT_TARGET), vec![vec![0x5000]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x434), vec![vec![a, 0]]);
        assert_eq!(e.mem.u8(a + 0xe1), 0);
    }

    #[test]
    fn set_lighting_shader_property_assigns_the_pointer() {
        let mut e = engine();
        let p = process(&mut e);
        middle_high_process_set_lighting_shader_property(&mut e, p, 0x7000);
        assert_eq!(
            calls(&e, POINTER_ASSIGN),
            vec![vec![p.addr() + 0x220, 0x7000]]
        );
    }

    #[test]
    fn is_package_done_checks_the_procedure_entry() {
        let mut e = engine();
        let p = process(&mut e);
        // No package: done.
        assert!(middle_high_process_is_package_done(&mut e, p, 0));
        let package = thing(&mut e);
        slot_returns(&mut e, PROCESS_SLOTS, 0x27c, package);
        slot_returns(&mut e, PROCESS_SLOTS, 0x280, 2);
        // Type -1: not done.
        e.register(PACKAGE_WORD_18, |_, _| ret(u32::MAX));
        assert!(!middle_high_process_is_package_done(&mut e, p, 0));
        // The table entry 0x36 means done, another one not.
        e.register(PACKAGE_WORD_18, |_, _| ret(0));
        procedure_table(&mut e, 0, 2, 0x36);
        assert!(middle_high_process_is_package_done(&mut e, p, 0));
        procedure_table(&mut e, 0, 2, 0x35);
        assert!(!middle_high_process_is_package_done(&mut e, p, 0));
    }

    #[test]
    fn defer_hide_dismembered_limb_queues_an_entry() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(DEFERRED_HIDE_CONSTRUCT, |_, a| ret(a[0]));
        middle_high_process_defer_hide_dismembered_limb(&mut e, p, 0x11, 0x22, 0x33, 1);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x14]]);
        let entry = calls(&e, DEFERRED_HIDE_CONSTRUCT)[0][0];
        assert_eq!(e.mem.f32(entry), 0.0);
        assert_eq!(e.mem.u32(entry + 4), 0x11);
        assert_eq!(e.mem.u8(entry + 0x10), 1);
        assert_eq!(
            calls(&e, POINTER_ASSIGN),
            vec![vec![entry + 8, 0x22], vec![entry + 0xc, 0x33]]
        );
        let added = calls(&e, LIST_ADD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], a + 0x1b0);
    }

    #[test]
    fn fn_0092c0c0_nulls_the_two_pointers() {
        let mut e = engine();
        let entry = e.mem.alloc(0x14);
        assert_eq!(fn_0092c0c0(&mut e, entry), entry);
        assert_eq!(
            calls(&e, POINTER_INIT),
            vec![vec![entry + 8, 0], vec![entry + 0xc, 0]]
        );
    }

    /// A list node at `at` holding `entry` and followed by `next`.
    fn list_node(e: &mut Engine, at: u32, entry: u32, next: u32) {
        e.mem.set_u32(at, entry);
        e.mem.set_u32(at + 4, next);
    }

    /// A deferred entry: float `time`, the +8 pointer holding something
    /// whose second word is `first_count`, the +0xC pointer likewise
    /// (`None` leaves it null).
    fn deferred_entry(e: &mut Engine, time: f32, first: Option<u32>, second: Option<u32>) -> u32 {
        let entry = e.mem.alloc(0x14);
        e.mem.set_f32(entry, time);
        e.mem.set_u32(entry + 4, 0x44);
        e.mem.set_u8(entry + 0x10, 7);
        for (offset, count) in [(8, first), (0xc, second)] {
            if let Some(count) = count {
                let target = e.mem.alloc(8);
                e.mem.set_u32(target + 4, count);
                e.mem.set_u32(entry + offset, target);
            }
        }
        entry
    }

    /// The setting is `1.0`; removing the head of a list empties it.
    fn deferred_scene(e: &mut Engine, vats_mode: u32) {
        let cell = e.mem.alloc(8);
        e.mem.set_f32(cell, 1.0);
        e.register_double(FLOAT_SETTING_VALUE, move |_, _| ret(cell));
        e.register_double(VATS_MODE, move |_, _| ret(vats_mode));
        e.register(LIST_REMOVE_HEAD, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
    }

    #[test]
    fn update_deferred_hide_keeps_a_young_entry() {
        let mut e = engine();
        let p = process(&mut e);
        deferred_scene(&mut e, 0);
        let entry = deferred_entry(&mut e, 0.5, Some(5), None);
        list_node(&mut e, p.addr() + 0x1b0, entry, 0);
        middle_high_process_update_deferred_hide_dismembered_limbs(&mut e, p, 0x900, 0.25);
        assert_eq!(e.mem.f32(entry), 0.75);
        assert!(calls(&e, ACTOR_HIDE_DISMEMBERED_LIMB).is_empty());
        assert!(calls(&e, DEFERRED_HIDE_LIMB_DELETE).is_empty());
        assert_eq!(
            calls(&e, FLOAT_SETTING_VALUE),
            vec![vec![HIDE_DELAY_SETTING]]
        );
    }

    #[test]
    fn update_deferred_hide_hides_the_limb_when_the_time_passes() {
        let mut e = engine();
        let p = process(&mut e);
        deferred_scene(&mut e, 4);
        let entry = deferred_entry(&mut e, 0.5, Some(5), Some(6));
        let first = e.mem.u32(entry + 8);
        let second = e.mem.u32(entry + 0xc);
        list_node(&mut e, p.addr() + 0x1b0, entry, 0);
        middle_high_process_update_deferred_hide_dismembered_limbs(&mut e, p, 0x900, 0.75);
        assert_eq!(e.mem.f32(entry), 1.25);
        assert_eq!(
            calls(&e, FLOAT_SETTING_VALUE),
            vec![vec![HIDE_DELAY_SETTING_VATS]]
        );
        assert_eq!(
            calls(&e, ACTOR_FILL_BONE_ARRAY),
            vec![vec![0x900, first, 0]]
        );
        assert_eq!(
            calls(&e, ACTOR_HIDE_DISMEMBERED_LIMB),
            vec![vec![0x900, 0x44, first, second, 7]]
        );
        assert_eq!(calls(&e, DEFERRED_HIDE_LIMB_DELETE), vec![vec![entry, 1]]);
        assert_eq!(calls(&e, LIST_REMOVE_HEAD), vec![vec![p.addr() + 0x1b0]]);
    }

    #[test]
    fn update_deferred_hide_finishes_entries_with_dead_targets() {
        let mut e = engine();
        let p = process(&mut e);
        deferred_scene(&mut e, 0);
        // The +8 pointer is null: finished without hiding.
        let entry = deferred_entry(&mut e, 0.0, None, Some(9));
        list_node(&mut e, p.addr() + 0x1b0, entry, 0);
        middle_high_process_update_deferred_hide_dismembered_limbs(&mut e, p, 0x900, 0.1);
        assert!(calls(&e, ACTOR_HIDE_DISMEMBERED_LIMB).is_empty());
        assert_eq!(calls(&e, DEFERRED_HIDE_LIMB_DELETE), vec![vec![entry, 1]]);
        // The +8 pointer's target has a count of 1: finished.
        let mut e = engine();
        let p = process(&mut e);
        deferred_scene(&mut e, 0);
        let entry = deferred_entry(&mut e, 0.0, Some(1), Some(9));
        list_node(&mut e, p.addr() + 0x1b0, entry, 0);
        middle_high_process_update_deferred_hide_dismembered_limbs(&mut e, p, 0x900, 0.1);
        assert_eq!(calls(&e, DEFERRED_HIDE_LIMB_DELETE).len(), 1);
        // The +0xC pointer's target has a count of 1: finished.
        let mut e = engine();
        let p = process(&mut e);
        deferred_scene(&mut e, 0);
        let entry = deferred_entry(&mut e, 0.0, Some(2), Some(1));
        list_node(&mut e, p.addr() + 0x1b0, entry, 0);
        middle_high_process_update_deferred_hide_dismembered_limbs(&mut e, p, 0x900, 0.1);
        assert_eq!(calls(&e, DEFERRED_HIDE_LIMB_DELETE).len(), 1);
    }

    #[test]
    fn update_deferred_hide_removes_a_later_entry_after_its_predecessor() {
        let mut e = engine();
        let p = process(&mut e);
        deferred_scene(&mut e, 0);
        let young = deferred_entry(&mut e, 0.0, Some(5), None);
        let dead = deferred_entry(&mut e, 0.0, None, None);
        let head = p.addr() + 0x1b0;
        let node = e.mem.alloc(8);
        list_node(&mut e, head, young, node);
        list_node(&mut e, node, dead, 0);
        e.register(LIST_REMOVE, |e, a| {
            // Unlink the node after `a[0]`.
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        middle_high_process_update_deferred_hide_dismembered_limbs(&mut e, p, 0x900, 0.1);
        assert_eq!(calls(&e, DEFERRED_HIDE_LIMB_DELETE), vec![vec![dead, 1]]);
        let removed = calls(&e, LIST_REMOVE);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], head);
        assert!(calls(&e, LIST_REMOVE_HEAD).is_empty());
    }

    #[test]
    fn set_creature_lip_synch_anim_deletes_the_old_one() {
        let mut e = engine();
        let p = process(&mut e);
        middle_high_process_set_creature_lip_synch_anim(&mut e, p, 0x1111);
        assert!(calls(&e, LIP_SYNCH_ANIM_DELETE).is_empty());
        assert_eq!(e.mem.u32(p.addr() + 0x1a0), 0x1111);
        middle_high_process_set_creature_lip_synch_anim(&mut e, p, 0x2222);
        assert_eq!(calls(&e, LIP_SYNCH_ANIM_DELETE), vec![vec![0x1111, 1]]);
        assert_eq!(e.mem.u32(p.addr() + 0x1a0), 0x2222);
    }

    #[test]
    fn has_movement_anim_caches_the_answer_per_group() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        // No animation object: yes.
        assert!(middle_high_process_has_movement_anim(&mut e, p, 5));
        e.mem.set_u32(a + 0x1c0, 0x7000);
        // `00493830` true: yes.
        e.register(NOT_EMPTY, |_, _| ret(1));
        assert!(middle_high_process_has_movement_anim(&mut e, p, 5));
        e.register(NOT_EMPTY, |_, _| ret(0));
        // Outside the cached range (group 2 gives -2, group 0x20 gives 0x3A).
        assert!(middle_high_process_has_movement_anim(&mut e, p, 2));
        assert!(middle_high_process_has_movement_anim(&mut e, p, 0x20));
        assert!(calls(&e, ANIMATION_GROUP_LOADED).is_empty());
        // A miss asks the animation and records the answer (shift 4).
        e.register(ANIMATION_GROUP_LOADED, |_, _| ret(0));
        assert!(!middle_high_process_has_movement_anim(&mut e, p, 5));
        assert_eq!(calls(&e, ANIMATION_GROUP_LOADED), vec![vec![0x7000, 5]]);
        assert_eq!(e.mem.u32(a + 0x22c), 2 << 4);
        // The cached "not loaded" answers without asking.
        assert!(!middle_high_process_has_movement_anim(&mut e, p, 5));
        assert_eq!(calls(&e, ANIMATION_GROUP_LOADED).len(), 1);
        // Another group answers loaded and sets bit 0 of its pair (shift 6).
        e.register(ANIMATION_GROUP_LOADED, |_, _| ret(1));
        assert!(middle_high_process_has_movement_anim(&mut e, p, 6));
        assert_eq!(e.mem.u32(a + 0x22c), (2 << 4) | (1 << 6));
        assert!(middle_high_process_has_movement_anim(&mut e, p, 6));
        assert_eq!(calls(&e, ANIMATION_GROUP_LOADED).len(), 2);
    }

    /// Allocation and the hit data constructor hand out a block.
    fn hit_data_doubles(e: &mut Engine) {
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(HIT_DATA_CONSTRUCT, |_, a| ret(a[0]));
    }

    #[test]
    fn set_last_hit_data_makes_the_data_once_and_copies() {
        let mut e = engine();
        let p = process(&mut e);
        hit_data_doubles(&mut e);
        middle_high_process_set_last_hit_data(&mut e, p, 0x5000);
        let data = e.mem.u32(p.addr() + 0x240);
        assert_ne!(data, 0);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x64]]);
        assert_eq!(calls(&e, HIT_DATA_CONSTRUCT), vec![vec![data]]);
        assert_eq!(calls(&e, HIT_DATA_COPY), vec![vec![data, 0x5000]]);
        middle_high_process_set_last_hit_data(&mut e, p, 0x6000);
        assert_eq!(calls(&e, ALLOCATE).len(), 1);
        assert_eq!(calls(&e, HIT_DATA_COPY)[1], vec![data, 0x6000]);
        assert_eq!(e.mem.u32(p.addr() + 0x254), 0);
    }

    #[test]
    fn set_last_hit_data_stores_null_when_the_allocation_fails() {
        let mut e = engine();
        let p = process(&mut e);
        e.register(ALLOCATE, |_, _| ret(0));
        middle_high_process_set_last_hit_data(&mut e, p, 0x5000);
        assert!(calls(&e, HIT_DATA_CONSTRUCT).is_empty());
        assert_eq!(e.mem.u32(p.addr() + 0x240), 0);
        assert_eq!(calls(&e, HIT_DATA_COPY), vec![vec![0, 0x5000]]);
    }

    #[test]
    fn clear_last_hit_data_makes_or_clears() {
        let mut e = engine();
        let p = process(&mut e);
        hit_data_doubles(&mut e);
        middle_high_process_clear_last_hit_data(&mut e, p);
        let data = e.mem.u32(p.addr() + 0x240);
        assert_ne!(data, 0);
        assert!(calls(&e, HIT_DATA_CLEAR).is_empty());
        middle_high_process_clear_last_hit_data(&mut e, p);
        assert_eq!(calls(&e, HIT_DATA_CLEAR), vec![vec![data]]);
        assert_eq!(calls(&e, ALLOCATE).len(), 1);
    }

    #[test]
    fn set_last_attack_hit_data_uses_the_second_slot() {
        let mut e = engine();
        let p = process(&mut e);
        hit_data_doubles(&mut e);
        middle_high_process_set_last_attack_hit_data(&mut e, p, 0x5000);
        let data = e.mem.u32(p.addr() + 0x254);
        assert_ne!(data, 0);
        assert_eq!(calls(&e, HIT_DATA_COPY), vec![vec![data, 0x5000]]);
        middle_high_process_set_last_attack_hit_data(&mut e, p, 0x6000);
        assert_eq!(calls(&e, ALLOCATE).len(), 1);
        assert_eq!(e.mem.u32(p.addr() + 0x240), 0);
    }

    #[test]
    fn clear_last_attack_hit_data_makes_or_clears() {
        let mut e = engine();
        let p = process(&mut e);
        hit_data_doubles(&mut e);
        middle_high_process_clear_last_attack_hit_data(&mut e, p);
        let data = e.mem.u32(p.addr() + 0x254);
        assert_ne!(data, 0);
        assert!(calls(&e, HIT_DATA_CLEAR).is_empty());
        middle_high_process_clear_last_attack_hit_data(&mut e, p);
        assert_eq!(calls(&e, HIT_DATA_CLEAR), vec![vec![data]]);
    }

    #[test]
    fn clear_furniture_marker_unreserves_the_slot_unless_loading() {
        let mut e = engine();
        let p = process(&mut e);
        let a = p.addr();
        e.set_global(SAVE_LOAD_GAME, 0x8100);
        e.set_global(DATA_HANDLER, 0x8200);
        e.mem.set_u32(a + 0x140, 0x7700);
        e.mem.set_u8(a + 0x144, 3);
        middle_high_process_clear_furniture_marker(&mut e, p);
        assert_eq!(calls(&e, SET_MARKER_RESERVED), vec![vec![0x7700, 3, 0]]);
        assert_eq!(calls(&e, SAVE_LOAD_FLAG_2), vec![vec![0x8100]]);
        assert_eq!(calls(&e, DATA_HANDLER_BYTE_61D), vec![vec![0x8200]]);
        assert_eq!(e.mem.u8(a + 0x144), 0x7f);
        assert_eq!(
            calls(&e, FURNITURE_MARK_SET_HEADING),
            vec![vec![a + 0x148, 0]]
        );
        assert_eq!(calls(&e, SET_BYTE_E), vec![vec![a + 0x148, 0]]);
        assert_eq!(
            calls(&e, COPY_POINT),
            vec![vec![a + 0x148, FURNITURE_MARK_ORIGIN]]
        );
        // The save/load flag set: no un-reserving.
        e.call_log = Some(vec![]);
        e.register(SAVE_LOAD_FLAG_2, |_, _| ret(1));
        middle_high_process_clear_furniture_marker(&mut e, p);
        assert!(calls(&e, SET_MARKER_RESERVED).is_empty());
        assert!(calls(&e, DATA_HANDLER_BYTE_61D).is_empty());
        // The data handler's byte set: likewise.
        e.call_log = Some(vec![]);
        e.register(SAVE_LOAD_FLAG_2, |_, _| ret(0));
        e.register(DATA_HANDLER_BYTE_61D, |_, _| ret(1));
        middle_high_process_clear_furniture_marker(&mut e, p);
        assert!(calls(&e, SET_MARKER_RESERVED).is_empty());
        // No furniture: nothing is asked.
        e.call_log = Some(vec![]);
        e.mem.set_u32(a + 0x140, 0);
        middle_high_process_clear_furniture_marker(&mut e, p);
        assert!(calls(&e, DATA_HANDLER_BYTE_61D).is_empty());
        assert_eq!(calls(&e, FURNITURE_MARK_SET_HEADING).len(), 1);
    }

    #[test]
    fn check_cached_value_flags_compares_the_two_slots() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        let part_vtable = 0x0503_0000;
        let slots: Vec<u32> = (0..VTABLE_SIZE)
            .step_by(4)
            .map(|s| OBJECT_SLOTS + s)
            .collect();
        e.put_vtable(part_vtable, &slots);
        e.mem.set_u32(a + 0xa4, part_vtable);
        // Value 0x10: slot 0xC says 5, slot 0x20 says 6 (higher). Value
        // 0x16: 7 and 7 (not higher).
        e.register(OBJECT_SLOTS + 0xc, |_, args| {
            ret_f(if args[1] == 0x10 { 5.0 } else { 7.0 })
        });
        e.register(OBJECT_SLOTS + 0x20, |_, args| {
            ret_f(if args[1] == 0x10 { 6.0 } else { 7.0 })
        });
        let list = 0x7100;
        e.register_double(REFERENCE_EXTRA_LIST, move |_, _| ret(list));
        e.register(EXTRA_LIST_BYTE, |_, _| ret(0x101));
        middle_high_process_check_cached_value_flags(&mut e, p, a);
        assert_eq!(
            calls(&e, CACHED_FLAG_SET),
            vec![
                vec![p.addr(), 0x2000_0000, 1],
                vec![p.addr(), 0x4000_0000, 0],
                vec![p.addr(), 0x1000_0000, 1],
            ]
        );
        assert_eq!(calls(&e, REFERENCE_EXTRA_LIST), vec![vec![a]]);
        assert_eq!(calls(&e, EXTRA_LIST_BYTE), vec![vec![list]]);
        // A NaN compares as not higher.
        e.call_log = Some(vec![]);
        e.register(OBJECT_SLOTS + 0x20, |_, args| {
            ret_f(if args[1] == 0x10 { f64::NAN } else { 8.0 })
        });
        middle_high_process_check_cached_value_flags(&mut e, p, a);
        let set = calls(&e, CACHED_FLAG_SET);
        assert_eq!(set[0], vec![p.addr(), 0x2000_0000, 0]);
        assert_eq!(set[1], vec![p.addr(), 0x4000_0000, 1]);
    }

    #[test]
    fn fn_0092c890_releases_what_the_cell_holds() {
        let mut e = engine();
        let cell = e.mem.alloc(4);
        fn_0092c890(&mut e, cell);
        assert!(calls(&e, CELL_RELEASE).is_empty());
        e.mem.set_u32(cell, 0x4400);
        fn_0092c890(&mut e, cell);
        assert_eq!(calls(&e, CELL_RELEASE), vec![vec![0x4400]]);
    }

    #[test]
    fn fn_0092c8b0_replaces_the_referent() {
        let mut e = engine();
        let cell = e.mem.alloc(4);
        // Empty cell: no release, the new referent is referenced.
        assert_eq!(fn_0092c8b0(&mut e, cell, 0x4400), cell);
        assert!(calls(&e, CELL_RELEASE).is_empty());
        assert_eq!(calls(&e, CELL_ADD_REFERENCE), vec![vec![0x4400]]);
        // The same value: nothing happens.
        fn_0092c8b0(&mut e, cell, 0x4400);
        assert_eq!(calls(&e, CELL_ADD_REFERENCE).len(), 1);
        // Another value: the old one is released.
        fn_0092c8b0(&mut e, cell, 0x5500);
        assert_eq!(calls(&e, CELL_RELEASE), vec![vec![0x4400]]);
        assert_eq!(calls(&e, CELL_ADD_REFERENCE)[1], vec![0x5500]);
        assert_eq!(e.mem.u32(cell), 0x5500);
        // Null: released, stored, not referenced.
        fn_0092c8b0(&mut e, cell, 0);
        assert_eq!(calls(&e, CELL_RELEASE)[1], vec![0x5500]);
        assert_eq!(calls(&e, CELL_ADD_REFERENCE).len(), 2);
        assert_eq!(e.mem.u32(cell), 0);
    }

    #[test]
    fn fn_0092c900_copies_the_referent_of_another_cell() {
        let mut e = engine();
        let cell = e.mem.alloc(4);
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 0x4400);
        assert_eq!(fn_0092c900(&mut e, cell, source), cell);
        assert_eq!(e.mem.u32(cell), 0x4400);
        assert!(calls(&e, CELL_RELEASE).is_empty());
        assert_eq!(calls(&e, CELL_ADD_REFERENCE), vec![vec![0x4400]]);
        // Equal: nothing happens.
        fn_0092c900(&mut e, cell, source);
        assert_eq!(calls(&e, CELL_ADD_REFERENCE).len(), 1);
        // A null source releases and stores null.
        e.mem.set_u32(source, 0);
        fn_0092c900(&mut e, cell, source);
        assert_eq!(calls(&e, CELL_RELEASE), vec![vec![0x4400]]);
        assert_eq!(calls(&e, CELL_ADD_REFERENCE).len(), 1);
        assert_eq!(e.mem.u32(cell), 0);
    }
}
