//! `(unplaced)/run_00574900` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESObjectREFR::HasContainer` (Xbox PDB, `0055d310`): `thiscall`; the reference's container
/// data, or null.
const HAS_CONTAINER: u32 = 0x0055_d310;
/// `InventoryChanges::GetInventoryChanges` (Xbox PDB, `004bf220`): `cdecl`, one stack word (the
/// reference).
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `InventoryChanges::WearingObject` (Xbox PDB, `004bfda0`): `thiscall`, `RET 8` (an item, a
/// second word).
const WEARING_OBJECT: u32 = 0x004b_fda0;

// Translated from 00575400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4` (an item): false when the reference has no
/// container data (`HasContainer`); otherwise whether its inventory changes
/// (`GetInventoryChanges(this)`) report `WearingObject(item, 0)` non-zero. The decompiler shows
/// the inventory changes call with the wrong `this`; the disassembly passes the reference to
/// the `cdecl` getter and its result as `this` of the second call.
pub fn fn_00575400(e: &mut Engine, this: Ptr, item: Ptr) -> bool {
    let mut result = false;
    if e.call(HAS_CONTAINER, &args![this]).u32() != 0 {
        let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
        if e.call(WEARING_OBJECT, &args![changes, item, 0u32]).u32() != 0 {
            result = true;
        }
    }
    result
}

/// The player character (`011dea3c`, a pointer global).
const PLAYER: u32 = 0x011d_ea3c;
/// The pointer global `011ddf38` that `00574920` asks `0042ce10` of.
const HOLDER_011DDF38: u32 = 0x011d_df38;
/// The pointer global `011c3f2c` that `00740960` is asked of.
const HOLDER_011C3F2C: u32 = 0x011c_3f2c;
/// `005d43c0` (`ECX` = reference): the reference's extra data list.
const EXTRA_LIST: u32 = 0x005d_43c0;
/// `Script::SetActionFlag` (Xbox PDB name of `005ac750`; cdecl, three words).
const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `008d6f30` (`ECX` = reference).
const REFERENCE_8D6F30: u32 = 0x008d_6f30;
/// `005656d0` (`ECX` = reference): a byte.
const REFERENCE_5656D0: u32 = 0x0056_56d0;
/// `00440da0` (`ECX` = reference): a byte.
const REFERENCE_440DA0: u32 = 0x0044_0da0;
/// `005f5950` (cdecl, seven words).
const NOTIFY_5F5950: u32 = 0x005f_5950;
/// `00401000` (cdecl: size): allocates.
const ALLOCATE: u32 = 0x0040_1000;
/// `00401170` (`ECX` = form): its form type.
const FORM_TYPE: u32 = 0x0040_1170;
/// `006815c0` (`ECX` = list node or object).
const NODE_ITEM: u32 = 0x0068_15c0;
/// `TESScriptableForm::GetFormScript` (Xbox PDB name of `004826d0`; cdecl,
/// the form).
const GET_FORM_SCRIPT: u32 = 0x0048_26d0;
/// `004c29a0` (`ECX` = inventory changes; three words; `RET 0xc`).
const CHANGES_4C29A0: u32 = 0x004c_29a0;
/// `00545560` (`ECX` = cell, one word; `RET 4`).
const CELL_545560: u32 = 0x0054_5560;
/// `00545590` (`ECX` = cell, one word; `RET 4`).
const CELL_545590: u32 = 0x0054_5590;
/// Virtual slot `0x100` of a reference: a byte (`MOVZX`/`TEST` on `AL`).
const SLOT_100: u32 = 0x100;

// Translated from 00574900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 0x81`. The engine map gives it no name.
pub fn fn_00574900(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x81)
}

// Translated from 00574920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall`, one byte word (`RET 4`). The flag is forced to 1 when
/// `0042ce10` on the global `011ddf38` reports false and `00577de0` on the
/// reference true. Nothing more without container data
/// (`TESObjectREFR::HasContainer`). Then, with the reference's inventory
/// changes (`GetInventoryChanges`): with the flag, `004d17a0` on them; without
/// it, `0041aeb0` on the extra data list (`005d43c0`) first. When virtual
/// slot `0x100` reports true, `008adc50` on the reference. Then `004d1440`,
/// `004d1610` (without the flag only) and `004d1960` on the changes, and
/// `0041aeb0` on the extra data list again when `0042cde0` on the changes
/// reports true. Last, the dropped-item list (`0041df90` on the extra data
/// list): while it is not null and `008256d0` on it reports false, its first
/// item (the word `006815c0` points at) is taken off (`0041de40` with 0 on
/// the item's extra data list), marked deleted (`00572270`) unless its
/// virtual slot `0x160` reports true, and `0063f7b0` runs on the list. The
/// list word itself is not re-read. The engine map gives it no name.
pub fn fn_00574920(e: &mut Engine, this: Ptr, flag: u8) {
    let reference = this.addr();
    let mut flag = flag;
    let holder = e.mem.u32(HOLDER_011DDF38);
    if !e.call(0x0042_ce10, &args![holder]).bool() && e.call(0x0057_7de0, &args![this]).bool() {
        flag = 1;
    }
    if e.call(HAS_CONTAINER, &args![this]).u32() == 0 {
        return;
    }
    let changes = if flag != 0 {
        let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
        e.call(0x004d_17a0, &args![changes]);
        changes
    } else {
        let list = e.call(EXTRA_LIST, &args![this]).u32();
        e.call(0x0041_aeb0, &args![list]);
        e.call(GET_INVENTORY_CHANGES, &args![this]).u32()
    };
    if e.vcall(reference, SLOT_100, &[]).bool() {
        e.call(0x008a_dc50, &args![this]);
    }
    e.call(0x004d_1440, &args![changes]);
    if flag == 0 {
        e.call(0x004d_1610, &args![changes]);
    }
    e.call(0x004d_1960, &args![changes]);
    if e.call(0x0042_cde0, &args![changes]).bool() {
        let list = e.call(EXTRA_LIST, &args![this]).u32();
        e.call(0x0041_aeb0, &args![list]);
    }
    let list = e.call(EXTRA_LIST, &args![this]).u32();
    let dropped = e.call(0x0041_df90, &args![list]).u32();
    if dropped == 0 {
        return;
    }
    loop {
        if e.call(0x0082_56d0, &args![dropped]).bool() {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![dropped]).u32();
        let item = e.mem.u32(slot);
        let item_list = e.call(EXTRA_LIST, &args![item]).u32();
        e.call(0x0041_de40, &args![item_list, 0u32]);
        if !e.vcall(item, 0x160, &[]).bool() {
            e.call(0x0057_2270, &args![item]);
        }
        e.call(0x0063_f7b0, &args![dropped]);
    }
}

/// The opening both `00574b30` and `00574fa0` share after their container
/// check: when virtual slot `0x100` reports false or `00440da0` true, and
/// `008d6f30` gives something, and `extra_check` passes, and `005656d0` on
/// the reference reports false, `00545560` on `008d6f30`'s result with the
/// reference.
fn cell_notice(e: &mut Engine, this: Ptr, extra_check: impl FnOnce(&mut Engine) -> bool) {
    let reference = this.addr();
    if e.vcall(reference, SLOT_100, &[]).bool() && !e.call(REFERENCE_440DA0, &args![this]).bool() {
        return;
    }
    if e.call(REFERENCE_8D6F30, &args![this]).u32() == 0 {
        return;
    }
    if !extra_check(e) {
        return;
    }
    if e.call(REFERENCE_5656D0, &args![this]).bool() {
        return;
    }
    let cell = e.call(REFERENCE_8D6F30, &args![this]).u32();
    e.call(CELL_545560, &args![cell, this]);
}

// Translated from 00574b30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00574b30 - the exception-unwinding frame (FS:[0]) around the three allocations is not translated.
/// `thiscall`; an item, a word, two byte words (`RET 0x10`). Nothing
/// without container data. Then the shared opening (`cell_notice`) with the
/// item's `005656d0` as the extra check; for the player (`011dea3c`),
/// `005f5950(10, word, form, 0, 0, 0, 0)` and `005f5950(6, ...)` with the
/// item's `007af430`; `Script::SetActionFlag(this, item's 005d43c0, 1)`;
/// the inventory changes. When the item's virtual slot `0x224` reports
/// true the plain add (`004c1c90`) is skipped and instead:
/// - when `009d9f00` gives something: a 0xc-byte block (`00401000`)
///   becomes an `ItemChange` (`004bc650` with `009d9f00` again; 0 without a
///   block); when it and its `00559450` are not null and the word
///   `006815c0` of that points at is not 0, `SetActionFlag(this, that word,
///   1)`; then `004c3380(changes, item change, 1)`;
/// - otherwise, when `00476c90` gives a weapon: its ammo is the weapon's
///   `00525a90(0)`'s `004fd400` when both exist, else the weapon; that
///   ammo when `004c0bf0` reports true, else `TESObjectWEAP::GetCurrentAmmo`
///   (`00525980`) with the reference (or 0 when its slot `0x100` reports
///   false). With an ammo: when it has a script (`004826d0`), a 0x20-byte
///   block becomes an `ExtraDataList` (`00410360`), gets count 1
///   (`00419ad0`, called even with no list), and when the list exists and
///   has no script (`00418800`) it gets the script (`00419ed0`) and its
///   `005abf60` (`00419f80`), a 0x68-byte block becomes a reference
///   (`0055a2f0`) that is made temporary (`00484490`) and the script runs
///   on it (`Script::Run` with the list's script locals `00418830`, 0, 0)
///   before it is deleted (virtual slot `0x10` with 1);
///   `SetActionFlag(this, list, 1)` and `004c29a0(changes, ammo, list, 1)`.
///   Without a script, `004c29a0(changes, ammo, 0, 1)`.
///
/// Last, when the item persists (`005653d0`), `0041c7f0` on the item's
/// extra data list with the reference and the item's virtual slot `0x48`
/// with 0x400. The engine map gives it no name.
pub fn fn_00574b30(
    e: &mut Engine,
    this: Ptr,
    item: Ptr,
    word: u32,
    first_flag: u8,
    second_flag: u8,
) {
    let reference = this.addr();
    if e.call(HAS_CONTAINER, &args![this]).u32() == 0 {
        return;
    }
    cell_notice(e, this, |e| e.call(REFERENCE_5656D0, &args![item]).bool());
    if reference == e.mem.u32(PLAYER) {
        let form = e.call(0x007a_f430, &args![item]).u32();
        e.call(
            NOTIFY_5F5950,
            &args![10u32, word, form, 0u32, 0u32, 0u32, 0u32],
        );
        let form = e.call(0x007a_f430, &args![item]).u32();
        e.call(
            NOTIFY_5F5950,
            &args![6u32, word, form, 0u32, 0u32, 0u32, 0u32],
        );
    }
    let item_list = e.call(EXTRA_LIST, &args![item]).u32();
    e.call(SET_ACTION_FLAG, &args![this, item_list, 1u32]);
    let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
    let mut plain = true;
    if e.vcall(item.addr(), 0x224, &[]).bool() {
        plain = false;
        if e.call(0x009d_9f00, &args![item]).u32() != 0 {
            let block = e.call(ALLOCATE, &args![0xcu32]).u32();
            let change = if block != 0 {
                let source = e.call(0x009d_9f00, &args![item]).u32();
                e.call(0x004b_c650, &args![block, source]).u32()
            } else {
                0
            };
            if change != 0 && e.call(0x0055_9450, &args![change]).u32() != 0 {
                let node = e.call(0x0055_9450, &args![change]).u32();
                let slot = e.call(NODE_ITEM, &args![node]).u32();
                if e.mem.u32(slot) != 0 {
                    let node = e.call(0x0055_9450, &args![change]).u32();
                    let slot = e.call(NODE_ITEM, &args![node]).u32();
                    let first = e.mem.u32(slot);
                    e.call(SET_ACTION_FLAG, &args![this, first, 1u32]);
                }
            }
            e.call(0x004c_3380, &args![changes, change, 1u32]);
        } else if e.call(0x0047_6c90, &args![item]).u32() != 0 {
            let mut weapon = e.call(0x0047_6c90, &args![item]).u32();
            if e.call(0x0052_5a90, &args![weapon, 0u32]).u32() != 0 {
                let part = e.call(0x0052_5a90, &args![weapon, 0u32]).u32();
                if e.call(0x004f_d400, &args![part]).u32() != 0 {
                    let part = e.call(0x0052_5a90, &args![weapon, 0u32]).u32();
                    weapon = e.call(0x004f_d400, &args![part]).u32();
                }
            }
            let ammo = if e.call(0x004c_0bf0, &args![weapon]).bool() {
                weapon
            } else {
                let owner = if e.vcall(reference, SLOT_100, &[]).bool() {
                    reference
                } else {
                    0
                };
                e.call(0x0052_5980, &args![weapon, owner]).u32()
            };
            if ammo != 0 {
                let script = e.call(GET_FORM_SCRIPT, &args![ammo]).u32();
                if script != 0 {
                    let block = e.call(ALLOCATE, &args![0x20u32]).u32();
                    let list = if block != 0 {
                        e.call(0x0041_0360, &args![block]).u32()
                    } else {
                        0
                    };
                    e.call(0x0041_9ad0, &args![list, 1u32]);
                    if list != 0 && e.call(0x0041_8800, &args![list]).u32() == 0 {
                        e.call(0x0041_9ed0, &args![list, script]);
                        let value = e.call(0x005a_bf60, &args![script]).u32();
                        e.call(0x0041_9f80, &args![list, value]);
                        let block = e.call(ALLOCATE, &args![0x68u32]).u32();
                        let temporary = if block != 0 {
                            e.call(0x0055_a2f0, &args![block]).u32()
                        } else {
                            0
                        };
                        e.call(0x0048_4490, &args![temporary]);
                        let locals = e.call(0x0041_8830, &args![list]).u32();
                        e.call(0x005a_c1e0, &args![script, temporary, locals, 0u32, 0u32]);
                        if temporary != 0 {
                            e.vcall(temporary, 0x10, &args![1u32]);
                        }
                    }
                    e.call(SET_ACTION_FLAG, &args![this, list, 1u32]);
                    e.call(CHANGES_4C29A0, &args![changes, ammo, list, 1u32]);
                } else {
                    e.call(CHANGES_4C29A0, &args![changes, ammo, 0u32, 1u32]);
                }
            }
        }
    }
    if plain {
        e.call(
            0x004c_1c90,
            &args![
                changes,
                item,
                word,
                u32::from(first_flag),
                u32::from(second_flag)
            ],
        );
    }
    if e.call(0x0056_53d0, &args![item]).bool() {
        let list = e.call(EXTRA_LIST, &args![item]).u32();
        e.call(0x0041_c7f0, &args![list, this]);
        e.vcall(item.addr(), 0x48, &args![0x400u32]);
    }
}

// Translated from 00574fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall`; an item, an extra data list, a count (`RET 0xc`). Nothing
/// without container data. Then the shared opening (`cell_notice`) with
/// "the item has a script" (`004826d0`) as the extra check; for the player
/// (`011dea3c`), `005f5950(10, count, item, 0, 0, 0, 0)` and
/// `005f5950(6, ...)`; `Script::SetActionFlag(this, list, 1)`; then
/// `004c29a0(item, list, count)` on the inventory changes. The engine map
/// gives it no name.
pub fn fn_00574fa0(e: &mut Engine, this: Ptr, item: Ptr, list: u32, count: u32) {
    let reference = this.addr();
    if e.call(HAS_CONTAINER, &args![this]).u32() == 0 {
        return;
    }
    cell_notice(e, this, |e| {
        e.call(GET_FORM_SCRIPT, &args![item]).u32() != 0
    });
    if reference == e.mem.u32(PLAYER) {
        e.call(
            NOTIFY_5F5950,
            &args![10u32, count, item, 0u32, 0u32, 0u32, 0u32],
        );
        e.call(
            NOTIFY_5F5950,
            &args![6u32, count, item, 0u32, 0u32, 0u32, 0u32],
        );
    }
    e.call(SET_ACTION_FLAG, &args![this, list, 1u32]);
    let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
    e.call(CHANGES_4C29A0, &args![changes, item, list, count]);
}

// Translated from 005750a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall`, ten stack words (`RET 0x28`; the 4th, 5th, 9th and 10th are
/// bytes): 0 for a null item or without container data. Otherwise: a byte
/// is noted (`005656d0` on the reference, only when its virtual slot `0x100`
/// reports false and `008d6f30` gives something), `Script::SetActionFlag(
/// this, list, 4)`, and the result is `004c37d0` on the inventory changes
/// with `(this, item, byte_14, word_10, list, byte_18, word_1c, word_20,
/// word_24, byte_28, byte_2c, 0)`. When the reference's slot `0x100`
/// reports true (an actor):
/// - when the reference's slot `0x360` reports true, the item's form type
///   (`00401170`) is 0x28, its `BGSAmmoForm::IsRockItLauncher` (`00474a80`
///   on `item + 0xa4`) reports true and the actor holds none of it
///   (`00575610`), `PlayerCharacter::ReturnRockItAmmo` (`009691d0`) on the
///   player;
/// - when its process (`008d8520`) exists, its slot `0x22c` with 0 reports
///   false, it is not the player and `00705020` and `00705000` report
///   false: the process's slot `0x17c` with the item and the list; then,
///   for an item of form type 0x28, when `00493bb0` reports false and the
///   actor's slot `0x3bc` with 6 gives something, `0044ddc0` on that gives the
///   object for `0088c830` when `Interface::IsInMenuMode` (`00702360`) and
///   `00566950` report true, else for `0088c650`, both with `(1, 0, 1, 0, 1)`
///   after it (the five pushes before `0044ddc0`, which takes none, belong to
///   them).
///
/// Last, when the noted byte is nonzero and `005656d0` now reports false,
/// `00545590` on `008d6f30`'s result with the reference. The engine map
/// gives it no name.
pub fn fn_005750a0(
    e: &mut Engine,
    this: Ptr,
    item: Ptr,
    list: u32,
    word_10: u32,
    byte_14: u8,
    byte_18: u8,
    word_1c: u32,
    word_20: u32,
    word_24: u32,
    byte_28: u8,
    byte_2c: u8,
) -> u32 {
    let reference = this.addr();
    if item.is_null() {
        return 0;
    }
    if e.call(HAS_CONTAINER, &args![this]).u32() == 0 {
        return 0;
    }
    let mut noted = 0u8;
    if !e.vcall(reference, SLOT_100, &[]).bool()
        && e.call(REFERENCE_8D6F30, &args![this]).u32() != 0
    {
        noted = e.call(REFERENCE_5656D0, &args![this]).u8();
    }
    e.call(SET_ACTION_FLAG, &args![this, list, 4u32]);
    let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
    let result = e
        .call(
            0x004c_37d0,
            &args![
                changes,
                this,
                item,
                u32::from(byte_14),
                word_10,
                list,
                u32::from(byte_18),
                word_1c,
                word_20,
                word_24,
                u32::from(byte_28),
                u32::from(byte_2c),
                0u32
            ],
        )
        .u32();
    if e.vcall(reference, SLOT_100, &[]).bool() {
        let actor = reference;
        if actor != 0
            && e.vcall(actor, 0x360, &[]).bool()
            && e.call(FORM_TYPE, &args![item]).u32() == 0x28
            && e.call(0x0047_4a80, &args![item.addr() + 0xa4]).bool()
            && e.call(0x0057_5610, &args![actor, item]).u32() == 0
        {
            let player = e.mem.u32(PLAYER);
            e.call(0x0096_91d0, &args![player]);
        }
        if actor != 0
            && e.call(0x008d_8520, &args![actor]).u32() != 0
            && !e.vcall(actor, 0x22c, &args![0u32]).bool()
            && reference != e.mem.u32(PLAYER)
            && !e.call(0x0070_5020, &[]).bool()
            && !e.call(0x0070_5000, &[]).bool()
        {
            let process = e.call(0x008d_8520, &args![actor]).u32();
            e.vcall(process, 0x17c, &args![item, list]);
            if e.call(FORM_TYPE, &args![item]).u32() == 0x28
                && !e.call(0x0049_3bb0, &args![actor]).bool()
            {
                let entry = e.vcall(actor, 0x3bc, &args![6u32]).u32();
                if entry != 0 {
                    let immediate = e.call(0x0070_2360, &[]).bool()
                        && e.call(0x0056_6950, &args![actor]).bool();
                    let object = e.call(0x0044_ddc0, &args![entry]).u32();
                    let target = if immediate { 0x0088_c830 } else { 0x0088_c650 };
                    e.call(target, &args![actor, object, 1u32, 0u32, 1u32, 0u32, 1u32]);
                }
            }
        }
    }
    if noted != 0 && !e.call(REFERENCE_5656D0, &args![this]).bool() {
        let cell = e.call(REFERENCE_8D6F30, &args![this]).u32();
        e.call(CELL_545590, &args![cell, this]);
    }
    result
}

// Translated from 00575510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall`; a byte flag and two out words (`RET 0xc`): both outs are
/// zeroed; with container data, the second becomes the inventory changes
/// (`00740960` on the global `011c3f2c` with the flag, else
/// `GetInventoryChanges`) and the first
/// `InventoryChanges::StartOEIFastInventoryIteration` (`004ca200`) on them.
/// The engine map gives it no name.
pub fn fn_00575510(e: &mut Engine, this: Ptr, flag: u8, iteration: Ptr, changes: Ptr) {
    let container = e.call(HAS_CONTAINER, &args![this]).u32();
    e.mem.set_u32(changes.addr(), 0);
    e.mem.set_u32(iteration.addr(), 0);
    if container == 0 {
        return;
    }
    let value = if flag != 0 {
        let holder = e.mem.u32(HOLDER_011C3F2C);
        e.call(0x0074_0960, &args![holder]).u32()
    } else {
        e.call(GET_INVENTORY_CHANGES, &args![this]).u32()
    };
    e.mem.set_u32(changes.addr(), value);
    let current = e.mem.u32(changes.addr());
    let first = e.call(0x004c_a200, &args![current]).u32();
    e.mem.set_u32(iteration.addr(), first);
}

// Translated from 00575590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall`; a byte flag and a byte (`RET 8`): -2 without container data,
/// otherwise `004c8fd0` (with the byte) on the inventory changes
/// (`00740960` on the global `011c3f2c` with the flag, else
/// `GetInventoryChanges`) minus one. The engine map gives it no name.
pub fn fn_00575590(e: &mut Engine, this: Ptr, flag: u8, value: u8) -> i32 {
    let mut result = 0xffff_ffffu32;
    if e.call(HAS_CONTAINER, &args![this]).u32() != 0 {
        let changes = if flag != 0 {
            let holder = e.mem.u32(HOLDER_011C3F2C);
            e.call(0x0074_0960, &args![holder]).u32()
        } else {
            e.call(GET_INVENTORY_CHANGES, &args![this]).u32()
        };
        result = e.call(0x004c_8fd0, &args![changes, u32::from(value)]).u32();
    }
    result.wrapping_sub(1) as i32
}

/// The save-load game pointer `0047c850` is asked of, and `0047c850` itself (`AL`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const SAVE_LOAD_TEST_0047C850: u32 = 0x0047_c850;
/// The inventory-changes calls the reference slots below forward to (`ECX` = the
/// changes): `004bffe0` (six stack words), `004c6d40` (three), `004c0cf0` (nine; the
/// first is the address of the result byte), `004cfe20` (one, `AL`), `004c94f0` (one).
const CHANGES_4BFFE0: u32 = 0x004b_ffe0;
const CHANGES_4C6D40: u32 = 0x004c_6d40;
const CHANGES_4C0CF0: u32 = 0x004c_0cf0;
const CHANGES_4CFE20: u32 = 0x004c_fe20;
const CHANGES_4C94F0: u32 = 0x004c_94f0;
/// `00740960` (`ECX` = the data handler): the inventory changes used instead of the
/// reference's own.
const HANDLER_CHANGES_00740960: u32 = 0x0074_0960;

// Translated from 00574a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// A reference vtable slot shared by many reference vtables (`0102f6e0`, `0107a6d8`, ...;
/// unnamed in the engine map; `ECX` and four stack words, `RET 0x10`, the fourth not read):
/// false without container data. Otherwise, for the player (`011dea3c`) two notices
/// (`005f5950` with 10 and then 6, `second`, `first` and four zeros), the action flag 1 for
/// `third` (`Script::SetActionFlag`), then `004bffe0(first, second, this, third, 0, 0)` on
/// the reference's inventory changes; true.
pub fn fn_00574a80(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    _unused_4: u32,
) -> bool {
    if e.call(HAS_CONTAINER, &args![this]).u32() == 0 {
        return false;
    }
    let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
    if this.addr() == e.global::<u32>(PLAYER) {
        for kind in [10u32, 6] {
            e.call(
                NOTIFY_5F5950,
                &args![kind, second, first, 0u32, 0u32, 0u32, 0u32],
            );
        }
    }
    e.call(SET_ACTION_FLAG, &args![this, third, 1u32]);
    e.call(
        CHANGES_4BFFE0,
        &args![changes, first, second, this, third, 0u32, 0u32],
    );
    true
}

// Translated from 00575330 (decompiled, FalloutNV.exe 1.4.0.525)
/// A reference vtable slot shared by many reference vtables (`0102f6dc`, `0107a6d4`, ...;
/// unnamed in the engine map; `ECX` and three stack words, `RET 0xc`, the second a byte):
/// with container data, `004c6d40(first, second, third)` on the inventory changes.
pub fn fn_00575330(e: &mut Engine, this: Ptr, first: u32, second: u8, third: u32) {
    if e.call(HAS_CONTAINER, &args![this]).u32() == 0 {
        return;
    }
    let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
    e.call(
        CHANGES_4C6D40,
        &args![changes, first, u32::from(second), third],
    );
}

// Translated from 00575380 (decompiled, FalloutNV.exe 1.4.0.525)
/// A reference vtable slot shared by many reference vtables (`0102f6e4`, `0107a6dc`, ...;
/// unnamed in the engine map; `ECX` and three stack words, `RET 0xc`): true without
/// container data. Otherwise sets the action flag 8 for `third` unless `0047c850` answers
/// true for the save-load game, then answers the byte `004c0cf0(&result, first, second,
/// this, third, 0, 0, 0, 0)` leaves in its local (1 before the call).
pub fn fn_00575380(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) -> u8 {
    let container = e.call(HAS_CONTAINER, &args![this]).u32();
    e.with_stack(4, |e, result| {
        e.mem.set_u8(result.addr(), 1);
        if container != 0 {
            let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
            let game: u32 = e.global(SAVE_LOAD_GAME);
            if e.call(SAVE_LOAD_TEST_0047C850, &args![game]).u8() == 0 {
                e.call(SET_ACTION_FLAG, &args![this, third, 8u32]);
            }
            e.call(
                CHANGES_4C0CF0,
                &args![changes, result, first, second, this, third, 0u32, 0u32, 0u32, 0u32],
            );
        }
        e.mem.u8(result.addr())
    })
}

// Translated from 00575450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` and one stack word, `RET 4`; called by `ProcessEscort`,
/// `ProcessAccompany`, `008f2610`): false without container data, else the byte
/// `004cfe20(item)` answers on the inventory changes.
pub fn fn_00575450(e: &mut Engine, this: Ptr, item: u32) -> u8 {
    if e.call(HAS_CONTAINER, &args![this]).u32() == 0 {
        return 0;
    }
    let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
    e.call(CHANGES_4CFE20, &args![changes, item]).u8()
}

// Translated from 005754a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` and two stack words, `RET 8`; called by
/// `ShowInventoryFunction`, `0090dd80`, `0090e660`): 0 without container data, else
/// `004c94f0(item)` on the inventory changes: the data handler's (`00740960`) when
/// `handler` is nonzero, else the reference's own.
pub fn fn_005754a0(e: &mut Engine, this: Ptr, item: u32, handler: u8) -> u32 {
    if e.call(HAS_CONTAINER, &args![this]).u32() == 0 {
        return 0;
    }
    let changes = if handler != 0 {
        let data_handler: u32 = e.global(HOLDER_011C3F2C);
        e.call(HANDLER_CHANGES_00740960, &args![data_handler]).u32()
    } else {
        e.call(GET_INVENTORY_CHANGES, &args![this]).u32()
    };
    e.call(CHANGES_4C94F0, &args![changes, item]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0057_4a80, fn_00574a80(Ptr, u32, u32, u32, u32) -> bool),
        entry!(0x0057_5330, fn_00575330(Ptr, u32, u8, u32)),
        entry!(0x0057_5380, fn_00575380(Ptr, u32, u32, u32) -> u8),
        entry!(0x0057_5450, fn_00575450(Ptr, u32) -> u8),
        entry!(0x0057_54a0, fn_005754a0(Ptr, u32, u8) -> u32),
        entry!(0x0057_5400, fn_00575400(Ptr, Ptr) -> bool),
        entry!(0x0057_4900, fn_00574900(Ptr) -> u8),
        entry!(0x0057_4920, fn_00574920(Ptr, u8)),
        entry!(0x0057_4b30, fn_00574b30(Ptr, Ptr, u32, u8, u8)),
        entry!(0x0057_4fa0, fn_00574fa0(Ptr, Ptr, u32, u32)),
        entry!(
            0x0057_50a0,
            fn_005750a0(Ptr, Ptr, u32, u32, u8, u8, u32, u32, u32, u8, u8) -> u32
        ),
        entry!(0x0057_5510, fn_00575510(Ptr, u8, Ptr, Ptr)),
        entry!(0x0057_5590, fn_00575590(Ptr, u8, u8) -> i32),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn at_00575400_asks_the_inventory_changes_only_for_a_reference_with_a_container() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(HAS_CONTAINER, |_, _| Ret::default());
        e.register(GET_INVENTORY_CHANGES, |_, _| 0x9000u32.into_ret());
        e.register(WEARING_OBJECT, |_, _| 1u32.into_ret());
        assert!(!e.call(0x0057_5400, &args![0x4000u32, 0x5000u32]).bool());
        assert!(calls(&e, GET_INVENTORY_CHANGES).is_empty());
        e.register(HAS_CONTAINER, |_, _| 0x7000u32.into_ret());
        assert!(e.call(0x0057_5400, &args![0x4000u32, 0x5000u32]).bool());
        assert_eq!(calls(&e, GET_INVENTORY_CHANGES), vec![vec![0x4000]]);
        assert_eq!(calls(&e, WEARING_OBJECT), vec![vec![0x9000, 0x5000, 0]]);
    }

    #[test]
    fn at_00575400_is_false_when_the_item_is_not_worn() {
        let mut e = Engine::new();
        e.register(HAS_CONTAINER, |_, _| 0x7000u32.into_ret());
        e.register(GET_INVENTORY_CHANGES, |_, _| 0x9000u32.into_ret());
        e.register(WEARING_OBJECT, |_, _| Ret::default());
        assert!(!e.call(0x0057_5400, &args![0x4000u32, 0x5000u32]).bool());
        // Only the full register counts, not just its low byte.
        e.register(WEARING_OBJECT, |_, _| 0x100u32.into_ret());
        assert!(e.call(0x0057_5400, &args![0x4000u32, 0x5000u32]).bool());
    }
}

#[cfg(test)]
mod b027_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// The function a virtual slot of the object made by `object` calls.
    fn slot(table: u32, offset: u32) -> u32 {
        table + 0x10_0000 + offset
    }

    /// An object whose vtable at `table` has 0x100 slots, each a double
    /// returning 0 at `slot(table, offset)`.
    fn object(e: &mut Engine, table: u32) -> u32 {
        let slots: Vec<u32> = (0..0x100).map(|i| slot(table, 4 * i)).collect();
        for s in &slots {
            e.register(*s, |_, _| ret(0));
        }
        e.put_vtable(table, &slots);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this, table);
        this
    }

    fn engine(zero: &[u32]) -> Engine {
        let mut e = Engine::new();
        for addr in zero {
            e.register(*addr, |_, _| ret(0));
        }
        e.map(PLAYER, 4);
        e.map(HOLDER_011DDF38, 4);
        e.map(HOLDER_011C3F2C, 4);
        e
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn at_00574900_reads_the_byte_at_0x81() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x84);
        e.mem.set_u8(this + 0x81, 0x9c);
        assert_eq!(e.call(0x0057_4900, &args![this]).u32(), 0x9c);
    }

    const DROP_CALLEES: [u32; 17] = [
        0x0042_ce10,
        0x0057_7de0,
        HAS_CONTAINER,
        GET_INVENTORY_CHANGES,
        0x004d_17a0,
        EXTRA_LIST,
        0x0041_aeb0,
        0x008a_dc50,
        0x004d_1440,
        0x004d_1610,
        0x004d_1960,
        0x0042_cde0,
        0x0041_df90,
        0x0082_56d0,
        0x0041_de40,
        0x0057_2270,
        0x0063_f7b0,
    ];

    #[test]
    fn at_00574920_does_nothing_more_without_a_container() {
        let mut e = engine(&DROP_CALLEES);
        let this = object(&mut e, 0x0f00_0000);
        e.mem.set_u32(HOLDER_011DDF38, 0x6100);
        let (_, log) = run(&mut e, 0x0057_4920, &[this, 0]);
        assert_eq!(
            log,
            vec![
                (0x0042_ce10, vec![0x6100]),
                (0x0057_7de0, vec![this]),
                (HAS_CONTAINER, vec![this]),
            ]
        );
    }

    #[test]
    fn at_00574920_without_the_flag_clears_and_drops_the_items() {
        let mut e = engine(&DROP_CALLEES);
        let this = object(&mut e, 0x0f00_0000);
        let item = object(&mut e, 0x0f20_0000);
        let item_slot = e.mem.alloc(4);
        e.mem.set_u32(item_slot, item);
        e.register(0x0042_ce10, |_, _| ret(1));
        e.register(HAS_CONTAINER, |_, _| ret(0x7000));
        e.register(GET_INVENTORY_CHANGES, |_, _| ret(0x9000));
        e.register(EXTRA_LIST, |_, a| ret(a[0] + 0x1000));
        e.register(0x0042_cde0, |_, _| ret(1));
        e.register(0x0041_df90, |_, _| ret(0x8800));
        e.register_double(NODE_ITEM, move |_, _| ret(item_slot));
        let empty = Rc::new(Cell::new(false));
        let seen = empty.clone();
        e.register_double(0x0082_56d0, move |_, _| ret(u32::from(seen.get())));
        e.register_double(0x0063_f7b0, move |_, _| {
            empty.set(true);
            ret(0)
        });
        e.register(slot(0x0f00_0000, SLOT_100), |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0057_4920, &[this, 0]);
        assert!(calls(&log, 0x004d_17a0).is_empty());
        assert_eq!(calls(&log, 0x0041_aeb0), vec![vec![this + 0x1000]; 2]);
        assert_eq!(calls(&log, 0x008a_dc50), vec![vec![this]]);
        assert_eq!(calls(&log, 0x004d_1610), vec![vec![0x9000]]);
        assert_eq!(calls(&log, 0x0041_df90), vec![vec![this + 0x1000]]);
        assert_eq!(calls(&log, 0x0041_de40), vec![vec![item + 0x1000, 0]]);
        assert_eq!(calls(&log, 0x0057_2270), vec![vec![item]]);
        assert_eq!(calls(&log, 0x0063_f7b0), vec![vec![0x8800]]);
        assert_eq!(calls(&log, 0x0082_56d0).len(), 2);
    }

    #[test]
    fn at_00574920_forces_the_flag() {
        let mut e = engine(&DROP_CALLEES);
        let this = object(&mut e, 0x0f00_0000);
        e.register(0x0057_7de0, |_, _| ret(1));
        e.register(HAS_CONTAINER, |_, _| ret(0x7000));
        e.register(GET_INVENTORY_CHANGES, |_, _| ret(0x9000));
        let (_, log) = run(&mut e, 0x0057_4920, &[this, 0]);
        assert_eq!(calls(&log, 0x004d_17a0), vec![vec![0x9000]]);
        assert!(calls(&log, 0x0041_aeb0).is_empty());
        assert!(calls(&log, 0x004d_1610).is_empty());
        assert!(calls(&log, 0x008a_dc50).is_empty());
        assert!(calls(&log, 0x0082_56d0).is_empty());
    }

    const ADD_CALLEES: [u32; 36] = [
        HAS_CONTAINER,
        REFERENCE_440DA0,
        REFERENCE_8D6F30,
        REFERENCE_5656D0,
        CELL_545560,
        0x007a_f430,
        NOTIFY_5F5950,
        EXTRA_LIST,
        SET_ACTION_FLAG,
        GET_INVENTORY_CHANGES,
        0x009d_9f00,
        ALLOCATE,
        0x004b_c650,
        0x0055_9450,
        NODE_ITEM,
        0x004c_3380,
        0x0047_6c90,
        0x0052_5a90,
        0x004f_d400,
        0x004c_0bf0,
        0x0052_5980,
        GET_FORM_SCRIPT,
        0x0041_0360,
        0x0041_9ad0,
        0x0041_8800,
        0x0041_9ed0,
        0x005a_bf60,
        0x0041_9f80,
        0x0055_a2f0,
        0x0048_4490,
        0x0041_8830,
        0x005a_c1e0,
        CHANGES_4C29A0,
        0x004c_1c90,
        0x0056_53d0,
        0x0041_c7f0,
    ];

    /// A reference and an item for `00574b30`; the item's vtable is at
    /// `0f20_0000`.
    fn add_setup() -> (Engine, u32, u32) {
        let mut e = engine(&ADD_CALLEES);
        let this = object(&mut e, 0x0f00_0000);
        let item = object(&mut e, 0x0f20_0000);
        e.register(HAS_CONTAINER, |_, _| ret(0x7000));
        e.register(GET_INVENTORY_CHANGES, |_, _| ret(0x9000));
        e.register(EXTRA_LIST, |_, a| ret(a[0] + 0x1000));
        (e, this, item)
    }

    #[test]
    fn at_00574b30_stops_without_a_container() {
        let (mut e, this, item) = add_setup();
        e.register(HAS_CONTAINER, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0057_4b30, &[this, item, 3, 1, 2]);
        assert_eq!(log, vec![(HAS_CONTAINER, vec![this])]);
    }

    #[test]
    fn at_00574b30_adds_plainly_for_the_player() {
        let (mut e, this, item) = add_setup();
        e.mem.set_u32(PLAYER, this);
        e.register(0x007a_f430, |_, _| ret(0x5500));
        e.register(REFERENCE_8D6F30, |_, _| ret(0x6600));
        e.register(0x0056_53d0, |_, _| ret(1));
        e.register_double(REFERENCE_5656D0, move |_, a| ret(u32::from(a[0] == item)));
        let (_, log) = run(&mut e, 0x0057_4b30, &[this, item, 3, 0x101, 2]);
        assert_eq!(calls(&log, CELL_545560), vec![vec![0x6600, this]]);
        assert_eq!(
            calls(&log, NOTIFY_5F5950),
            vec![
                vec![10, 3, 0x5500, 0, 0, 0, 0],
                vec![6, 3, 0x5500, 0, 0, 0, 0]
            ]
        );
        assert_eq!(
            calls(&log, SET_ACTION_FLAG),
            vec![vec![this, item + 0x1000, 1]]
        );
        // The byte words go on zero-extended (`MOVZX`), so 0x101 passes as 1.
        assert_eq!(calls(&log, 0x004c_1c90), vec![vec![0x9000, item, 3, 1, 2]]);
        assert_eq!(calls(&log, 0x0041_c7f0), vec![vec![item + 0x1000, this]]);
        assert_eq!(
            calls(&log, slot(0x0f20_0000, 0x48)),
            vec![vec![item, 0x400]]
        );
    }

    #[test]
    fn at_00574b30_takes_an_item_change() {
        let (mut e, this, item) = add_setup();
        e.register(slot(0x0f20_0000, 0x224), |_, _| ret(1));
        e.register(0x009d_9f00, |_, _| ret(0x5100));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x004b_c650, |_, a| ret(a[0]));
        e.register(0x0055_9450, |_, _| ret(0x5200));
        let first = e.mem.alloc(4);
        e.mem.set_u32(first, 0x5300);
        e.register_double(NODE_ITEM, move |_, _| ret(first));
        let (_, log) = run(&mut e, 0x0057_4b30, &[this, item, 3, 1, 2]);
        let block = calls(&log, 0x004b_c650)[0][0];
        assert_eq!(calls(&log, 0x004b_c650), vec![vec![block, 0x5100]]);
        assert_eq!(
            calls(&log, SET_ACTION_FLAG),
            vec![vec![this, item + 0x1000, 1], vec![this, 0x5300, 1]]
        );
        assert_eq!(calls(&log, 0x004c_3380), vec![vec![0x9000, block, 1]]);
        assert!(calls(&log, 0x004c_1c90).is_empty());
    }

    #[test]
    fn at_00574b30_runs_the_ammo_script_on_a_temporary_reference() {
        let (mut e, this, item) = add_setup();
        e.register(slot(0x0f20_0000, 0x224), |_, _| ret(1));
        e.register(0x0047_6c90, |_, _| ret(0x5400));
        e.register(0x0052_5a90, |_, _| ret(0x5500));
        e.register(0x004f_d400, |_, _| ret(0x5600));
        e.register(0x0052_5980, |_, _| ret(0x5700));
        e.register(GET_FORM_SCRIPT, |_, _| ret(0x5800));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x0041_0360, |_, a| ret(a[0]));
        e.register(0x005a_bf60, |_, _| ret(0x5900));
        e.register(0x0041_8830, |_, _| ret(0x5a00));
        e.register(0x0055_a2f0, |e, a| {
            e.mem.set_u32(a[0], 0x0f40_0000);
            ret(a[0])
        });
        e.register(slot(0x0f00_0000, SLOT_100), |_, _| ret(1));
        object(&mut e, 0x0f40_0000);
        let (_, log) = run(&mut e, 0x0057_4b30, &[this, item, 3, 1, 2]);
        assert_eq!(calls(&log, 0x004f_d400), vec![vec![0x5500]; 2]);
        // The ammo comes from the weapon's part, asked with the actor.
        assert_eq!(calls(&log, 0x0052_5980), vec![vec![0x5600, this]]);
        let list = calls(&log, 0x0041_0360)[0][0];
        assert_eq!(calls(&log, 0x0041_9ad0), vec![vec![list, 1]]);
        assert_eq!(calls(&log, 0x0041_9ed0), vec![vec![list, 0x5800]]);
        assert_eq!(calls(&log, 0x0041_9f80), vec![vec![list, 0x5900]]);
        let temporary = calls(&log, 0x0055_a2f0)[0][0];
        assert_eq!(calls(&log, 0x0048_4490), vec![vec![temporary]]);
        assert_eq!(
            calls(&log, 0x005a_c1e0),
            vec![vec![0x5800, temporary, 0x5a00, 0, 0]]
        );
        assert_eq!(
            calls(&log, slot(0x0f40_0000, 0x10)),
            vec![vec![temporary, 1]]
        );
        assert_eq!(
            calls(&log, SET_ACTION_FLAG),
            vec![vec![this, item + 0x1000, 1], vec![this, list, 1]]
        );
        assert_eq!(
            calls(&log, CHANGES_4C29A0),
            vec![vec![0x9000, 0x5700, list, 1]]
        );
        assert!(calls(&log, 0x004c_1c90).is_empty());
    }

    #[test]
    fn at_00574b30_adds_ammo_without_a_script() {
        let (mut e, this, item) = add_setup();
        e.register(slot(0x0f20_0000, 0x224), |_, _| ret(1));
        e.register(0x0047_6c90, |_, _| ret(0x5400));
        e.register(0x004c_0bf0, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0057_4b30, &[this, item, 3, 1, 2]);
        assert!(calls(&log, 0x0052_5980).is_empty());
        assert_eq!(
            calls(&log, CHANGES_4C29A0),
            vec![vec![0x9000, 0x5400, 0, 1]]
        );
    }

    #[test]
    fn at_00574fa0_notices_the_cell_and_adds() {
        let mut e = engine(&ADD_CALLEES);
        let this = object(&mut e, 0x0f00_0000);
        e.mem.set_u32(PLAYER, this);
        e.register(HAS_CONTAINER, |_, _| ret(0x7000));
        e.register(GET_INVENTORY_CHANGES, |_, _| ret(0x9000));
        e.register(REFERENCE_8D6F30, |_, _| ret(0x6600));
        e.register(GET_FORM_SCRIPT, |_, _| ret(0x5800));
        let (_, log) = run(&mut e, 0x0057_4fa0, &[this, 0x5000, 0x5100, 4]);
        assert_eq!(calls(&log, GET_FORM_SCRIPT), vec![vec![0x5000]]);
        assert_eq!(calls(&log, CELL_545560), vec![vec![0x6600, this]]);
        assert_eq!(
            calls(&log, NOTIFY_5F5950),
            vec![
                vec![10, 4, 0x5000, 0, 0, 0, 0],
                vec![6, 4, 0x5000, 0, 0, 0, 0]
            ]
        );
        assert_eq!(calls(&log, SET_ACTION_FLAG), vec![vec![this, 0x5100, 1]]);
        assert_eq!(
            calls(&log, CHANGES_4C29A0),
            vec![vec![0x9000, 0x5000, 0x5100, 4]]
        );
        // An actor whose `00440da0` reports false skips the cell notice.
        e.mem.set_u32(PLAYER, 0);
        e.register(slot(0x0f00_0000, SLOT_100), |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0057_4fa0, &[this, 0x5000, 0x5100, 4]);
        assert!(calls(&log, CELL_545560).is_empty());
        assert!(calls(&log, NOTIFY_5F5950).is_empty());
    }

    const REMOVE_CALLEES: [u32; 20] = [
        HAS_CONTAINER,
        REFERENCE_8D6F30,
        REFERENCE_5656D0,
        SET_ACTION_FLAG,
        GET_INVENTORY_CHANGES,
        0x004c_37d0,
        FORM_TYPE,
        0x0047_4a80,
        0x0057_5610,
        0x0096_91d0,
        0x008d_8520,
        0x0070_5020,
        0x0070_5000,
        0x0049_3bb0,
        0x0070_2360,
        0x0056_6950,
        0x0044_ddc0,
        0x0088_c830,
        0x0088_c650,
        CELL_545590,
    ];

    #[test]
    fn at_005750a0_returns_zero_without_an_item_or_container() {
        let mut e = engine(&REMOVE_CALLEES);
        let this = object(&mut e, 0x0f00_0000);
        let words = [this, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let (result, log) = run(&mut e, 0x0057_50a0, &words);
        assert_eq!(result.u32(), 0);
        assert!(log.is_empty());
        let words = [this, 0x5000, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let (result, log) = run(&mut e, 0x0057_50a0, &words);
        assert_eq!(result.u32(), 0);
        assert_eq!(log, vec![(HAS_CONTAINER, vec![this])]);
    }

    #[test]
    fn at_005750a0_removes_from_a_container_and_tells_the_cell() {
        let mut e = engine(&REMOVE_CALLEES);
        let this = object(&mut e, 0x0f00_0000);
        e.register(HAS_CONTAINER, |_, _| ret(0x7000));
        e.register(GET_INVENTORY_CHANGES, |_, _| ret(0x9000));
        e.register(0x004c_37d0, |_, _| ret(0x77));
        e.register(REFERENCE_8D6F30, |_, _| ret(0x6600));
        let noted = Rc::new(Cell::new(0u32));
        let count = noted.clone();
        e.register_double(REFERENCE_5656D0, move |_, _| {
            count.set(count.get() + 1);
            ret(u32::from(count.get() == 1))
        });
        let words = [
            this, 0x5000, 0x5100, 0x12, 0x1ff, 0x4, 0x1c, 0x20, 0x24, 0x28, 0x2c,
        ];
        let (result, log) = run(&mut e, 0x0057_50a0, &words);
        assert_eq!(result.u32(), 0x77);
        assert_eq!(calls(&log, SET_ACTION_FLAG), vec![vec![this, 0x5100, 4]]);
        assert_eq!(
            calls(&log, 0x004c_37d0),
            vec![vec![
                0x9000, this, 0x5000, 0xff, 0x12, 0x5100, 0x4, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0
            ]]
        );
        assert_eq!(calls(&log, CELL_545590), vec![vec![0x6600, this]]);
        assert_eq!(noted.get(), 2);
    }

    #[test]
    fn at_005750a0_lets_an_actor_return_ammo_and_equip() {
        let mut e = engine(&REMOVE_CALLEES);
        let this = object(&mut e, 0x0f00_0000);
        let process = object(&mut e, 0x0f20_0000);
        let item = e.mem.alloc(0x100);
        e.register(HAS_CONTAINER, |_, _| ret(0x7000));
        e.register(GET_INVENTORY_CHANGES, |_, _| ret(0x9000));
        e.register(slot(0x0f00_0000, SLOT_100), |_, _| ret(1));
        e.register(slot(0x0f00_0000, 0x360), |_, _| ret(1));
        e.register(slot(0x0f00_0000, 0x3bc), |_, _| ret(0x5300));
        e.register(FORM_TYPE, |_, _| ret(0x28));
        e.register(0x0047_4a80, |_, _| ret(1));
        e.register_double(0x008d_8520, move |_, _| ret(process));
        e.register(0x0044_ddc0, |_, _| ret(0x5400));
        e.mem.set_u32(PLAYER, 0x0123_0000);
        let words = [this, item, 0x5100, 0, 0, 0, 0, 0, 0, 0, 0];
        let (_, log) = run(&mut e, 0x0057_50a0, &words);
        assert_eq!(calls(&log, 0x0047_4a80), vec![vec![item + 0xa4]]);
        assert_eq!(calls(&log, 0x0057_5610), vec![vec![this, item]]);
        assert_eq!(calls(&log, 0x0096_91d0), vec![vec![0x0123_0000]]);
        assert_eq!(
            calls(&log, slot(0x0f20_0000, 0x17c)),
            vec![vec![process, item, 0x5100]]
        );
        assert_eq!(calls(&log, slot(0x0f00_0000, 0x3bc)), vec![vec![this, 6]]);
        assert_eq!(calls(&log, 0x0044_ddc0), vec![vec![0x5300]]);
        assert_eq!(
            calls(&log, 0x0088_c650),
            vec![vec![this, 0x5400, 1, 0, 1, 0, 1]]
        );
        assert!(calls(&log, 0x0088_c830).is_empty());
        // In menu mode with a forced update the other equip runs.
        e.register(0x0070_2360, |_, _| ret(1));
        e.register(0x0056_6950, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0057_50a0, &words);
        assert_eq!(
            calls(&log, 0x0088_c830),
            vec![vec![this, 0x5400, 1, 0, 1, 0, 1]]
        );
        // The player itself takes neither path of the process.
        e.mem.set_u32(PLAYER, this);
        let (_, log) = run(&mut e, 0x0057_50a0, &words);
        assert!(calls(&log, slot(0x0f20_0000, 0x17c)).is_empty());
    }

    #[test]
    fn at_00575510_starts_an_iteration() {
        let mut e = engine(&[
            HAS_CONTAINER,
            GET_INVENTORY_CHANGES,
            0x0074_0960,
            0x004c_a200,
        ]);
        e.mem.set_u32(HOLDER_011C3F2C, 0x6200);
        let outs = e.mem.alloc(8);
        e.mem.set_u32(outs, 0x11);
        e.mem.set_u32(outs + 4, 0x22);
        let (_, log) = run(&mut e, 0x0057_5510, &[0x4000, 1, outs, outs + 4]);
        assert_eq!(log, vec![(HAS_CONTAINER, vec![0x4000])]);
        assert_eq!((e.mem.u32(outs), e.mem.u32(outs + 4)), (0, 0));
        e.register(HAS_CONTAINER, |_, _| ret(0x7000));
        e.register(0x0074_0960, |_, _| ret(0x9100));
        e.register(GET_INVENTORY_CHANGES, |_, _| ret(0x9000));
        e.register(0x004c_a200, |_, a| ret(a[0] + 1));
        let (_, log) = run(&mut e, 0x0057_5510, &[0x4000, 1, outs, outs + 4]);
        assert_eq!(calls(&log, 0x0074_0960), vec![vec![0x6200]]);
        assert_eq!((e.mem.u32(outs), e.mem.u32(outs + 4)), (0x9101, 0x9100));
        let (_, log) = run(&mut e, 0x0057_5510, &[0x4000, 0, outs, outs + 4]);
        assert_eq!(calls(&log, GET_INVENTORY_CHANGES), vec![vec![0x4000]]);
        assert_eq!((e.mem.u32(outs), e.mem.u32(outs + 4)), (0x9001, 0x9000));
    }

    #[test]
    fn at_00575590_counts_one_less() {
        let mut e = engine(&[
            HAS_CONTAINER,
            GET_INVENTORY_CHANGES,
            0x0074_0960,
            0x004c_8fd0,
        ]);
        assert_eq!(e.call(0x0057_5590, &args![0x4000u32, 0u32, 1u32]).i32(), -2);
        e.register(HAS_CONTAINER, |_, _| ret(0x7000));
        e.register(GET_INVENTORY_CHANGES, |_, _| ret(0x9000));
        e.register(0x0074_0960, |_, _| ret(0x9100));
        e.register(0x004c_8fd0, |_, a| ret(a[0] - 0x9000 + 5 + a[1]));
        assert_eq!(e.call(0x0057_5590, &args![0x4000u32, 0u32, 1u32]).i32(), 5);
        // The byte goes on zero-extended: 0x301 passes as 1.
        assert_eq!(
            e.call(0x0057_5590, &args![0x4000u32, 2u32, 0x301u32]).i32(),
            0x105
        );
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine(container: bool) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.register(
            HAS_CONTAINER,
            if container {
                |_, _| 0x40u32.into_ret()
            } else {
                |_, _| 0u32.into_ret()
            },
        );
        e.register(GET_INVENTORY_CHANGES, |_, a| (a[0] + 0x100).into_ret());
        for callee in [
            NOTIFY_5F5950,
            SET_ACTION_FLAG,
            CHANGES_4BFFE0,
            CHANGES_4C6D40,
            CHANGES_4C0CF0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00574a80_hands_the_item_on_and_notifies_for_the_player() {
        let mut e = engine(true);
        e.set_global(PLAYER, 0x1000u32);
        assert!(e
            .call(0x0057_4a80, &args![0x1000u32, 1u32, 2u32, 3u32, 4u32])
            .bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, NOTIFY_5F5950),
            vec![vec![10, 2, 1, 0, 0, 0, 0], vec![6, 2, 1, 0, 0, 0, 0]]
        );
        assert_eq!(calls(&log, SET_ACTION_FLAG), vec![vec![0x1000, 3, 1]]);
        assert_eq!(
            calls(&log, CHANGES_4BFFE0),
            vec![vec![0x1100, 1, 2, 0x1000, 3, 0, 0]]
        );
        let mut e = engine(true);
        assert!(e
            .call(0x0057_4a80, &args![0x2000u32, 1u32, 2u32, 3u32, 4u32])
            .bool());
        assert!(calls(&e.call_log.take().unwrap(), NOTIFY_5F5950).is_empty());
        let mut e = engine(false);
        assert!(!e
            .call(0x0057_4a80, &args![0x2000u32, 1u32, 2u32, 3u32, 4u32])
            .bool());
    }

    #[test]
    fn fn_00575330_forwards_with_a_container() {
        let mut e = engine(true);
        e.call(0x0057_5330, &args![0x1000u32, 1u32, 0x302u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, CHANGES_4C6D40), vec![vec![0x1100, 1, 2, 3]]);
        let mut e = engine(false);
        e.call(0x0057_5330, &args![0x1000u32, 1u32, 2u32, 3u32]);
        assert!(calls(&e.call_log.take().unwrap(), CHANGES_4C6D40).is_empty());
    }

    #[test]
    fn fn_00575380_answers_the_byte_the_changes_leave() {
        let mut e = engine(true);
        e.register(SAVE_LOAD_TEST_0047C850, |_, _| 0u32.into_ret());
        e.register(CHANGES_4C0CF0, |e, a| {
            assert_eq!(e.mem.u8(a[1]), 1);
            e.mem.set_u8(a[1], 0);
            Ret::default()
        });
        assert_eq!(
            e.call(0x0057_5380, &args![0x1000u32, 1u32, 2u32, 3u32])
                .u32()
                & 0xff,
            0
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SET_ACTION_FLAG), vec![vec![0x1000, 3, 8]]);
        let call = &calls(&log, CHANGES_4C0CF0)[0];
        assert_eq!(call[0], 0x1100);
        assert_eq!(call[2..], [1, 2, 0x1000, 3, 0, 0, 0, 0]);
        let mut e = engine(true);
        e.register(SAVE_LOAD_TEST_0047C850, |_, _| 1u32.into_ret());
        assert_eq!(
            e.call(0x0057_5380, &args![0x1000u32, 1u32, 2u32, 3u32])
                .u32()
                & 0xff,
            1
        );
        assert!(calls(&e.call_log.take().unwrap(), SET_ACTION_FLAG).is_empty());
        let mut e = engine(false);
        assert_eq!(
            e.call(0x0057_5380, &args![0x1000u32, 1u32, 2u32, 3u32])
                .u32()
                & 0xff,
            1
        );
    }

    #[test]
    fn fn_00575450_and_005754a0_ask_the_changes() {
        let mut e = engine(true);
        e.register(CHANGES_4CFE20, |_, a| (0x1200 | a[1]).into_ret());
        assert_eq!(e.call(0x0057_5450, &args![0x1000u32, 7u32]).u32() & 0xff, 7);
        e.register(CHANGES_4C94F0, |_, a| (a[0] + a[1]).into_ret());
        e.set_global(HOLDER_011C3F2C, 0x5000u32);
        e.register(HANDLER_CHANGES_00740960, |_, a| (a[0] + 0x10).into_ret());
        assert_eq!(
            e.call(0x0057_54a0, &args![0x1000u32, 7u32, 0u32]).u32(),
            0x1107
        );
        assert_eq!(
            e.call(0x0057_54a0, &args![0x1000u32, 7u32, 1u32]).u32(),
            0x5017
        );
        let mut e = engine(false);
        assert_eq!(e.call(0x0057_5450, &args![0x1000u32, 7u32]).u32() & 0xff, 0);
        assert_eq!(e.call(0x0057_54a0, &args![0x1000u32, 7u32, 1u32]).u32(), 0);
    }
}
