//! `fallout shared/tesobjectdoor.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Record tags (`u32::from_le_bytes` of the four characters) `TESObjectDOOR::Load` tells apart.
const MODL: u32 = u32::from_le_bytes(*b"MODL");
const MODD: u32 = u32::from_le_bytes(*b"MODD");
const MOSD: u32 = u32::from_le_bytes(*b"MOSD");
const MODT: u32 = u32::from_le_bytes(*b"MODT");
const MODS: u32 = u32::from_le_bytes(*b"MODS");
const OBND: u32 = u32::from_le_bytes(*b"OBND");
const EDID: u32 = u32::from_le_bytes(*b"EDID");
const FULL: u32 = u32::from_le_bytes(*b"FULL");
const SCRI: u32 = u32::from_le_bytes(*b"SCRI");
const DSTD: u32 = u32::from_le_bytes(*b"DSTD");
const DEST: u32 = u32::from_le_bytes(*b"DEST");
const ANAM: u32 = u32::from_le_bytes(*b"ANAM");
const BNAM: u32 = u32::from_le_bytes(*b"BNAM");
const SNAM: u32 = u32::from_le_bytes(*b"SNAM");
const FNAM: u32 = u32::from_le_bytes(*b"FNAM");
const TNAM: u32 = u32::from_le_bytes(*b"TNAM");

// Translated from 005177f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005177f0 - the security-cookie check (00ec408c) at the end is not translated.
/// `TESObjectDOOR::Load` (Xbox PDB). `this` is the form and `file` the `TESFile` positioned on
/// its record. Answers false without touching the form when the record is not of type 0x1c,
/// otherwise true.
///
/// It runs `TESForm::LoadForm` (`00485110`), `00484ab0` with 0, and walks the chunks:
///
/// - `EDID`: reads the chunk (limit 0x200) and runs virtual slot `+0x134` on it; `OBND`: slot
///   `+0xe0` with the file;
/// - `FULL`: `00487050` on `+0x30`; `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk`
///   (`004892d0`) on `+0x3c`; `MODS`: the texture swaps into `+0x3c` (`0048a7b0`);
/// - `SCRI`: reads a word, `006ecd40` on `+0x5c` with it and `0048cd90` on `+0x5c` with the form;
/// - `DSTD`, `DEST`: `004781e0` on `+0x68`;
/// - `SNAM`, `ANAM`, `BNAM`: read a form id, give it the file's compile index
///   (`TESForm::AddCompileIndex` `00485d50`), look the form up (`004839c0`), cast it with
///   `__RTDynamicCast` from the type at `0x01183028` to the type at `0x011832fc` and store the
///   result at `+0x78`, `+0x7c` and `+0x80`;
/// - `FNAM`: reads one byte into `+0x84`;
/// - `TNAM`: reads a word and runs `005ae3d0` on `+0x88` with the address of a copy of it.
///
/// The buffers the game takes from the stack (`__alloca_probe_16`) are heap blocks here.
pub fn tesobjectdoor_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if e.call(0x0047_2660, &args![file]).u8() != 0x1c {
        return false;
    }
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            EDID => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, 0x200u32]);
                e.vcall(base, 0x134, &args![buffer]);
                e.mem.free(buffer);
            }
            OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            FULL => {
                let target = if base != 0 { base + 0x30 } else { 0 };
                e.call(0x0048_7050, &args![target, file]);
            }
            MODL | MODD | MOSD | MODT => {
                let model = if base != 0 { base + 0x3c } else { 0 };
                e.call(0x0048_92d0, &args![model, file]);
            }
            MODS => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, size]);
                let swap_flags = e.call(0x0040_3570, &args![file]).u32() & 0xffff;
                e.call(0x0048_a7b0, &args![base + 0x3c, buffer, size, swap_flags]);
                e.mem.free(buffer);
            }
            SCRI => {
                let value = read_word(e, file);
                e.call(0x006e_cd40, &args![base + 0x5c, value]);
                e.call(0x0048_cd90, &args![base + 0x5c, this]);
            }
            DSTD | DEST => {
                let target = if base != 0 { base + 0x68 } else { 0 };
                e.call(0x0047_81e0, &args![target, file]);
            }
            SNAM => read_form_reference(e, file, base + 0x78),
            ANAM => read_form_reference(e, file, base + 0x7c),
            BNAM => read_form_reference(e, file, base + 0x80),
            FNAM => {
                e.call(0x0047_2890, &args![file, base + 0x84, 1u32]);
            }
            TNAM => {
                let value = read_word(e, file);
                let copy = e.mem.alloc(4);
                e.mem.set_u32(copy, value);
                e.call(0x005a_e3d0, &args![base + 0x88, copy]);
                e.mem.free(copy);
            }
            _ => {}
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    true
}

/// A zeroed local read with `TESFile::GetChunkData` (`004727f0`).
fn read_word(e: &mut Engine, file: Ptr) -> u32 {
    let local = e.mem.alloc(4);
    e.call(0x0047_27f0, &args![file, local]);
    let value = e.mem.u32(local);
    e.mem.free(local);
    value
}

/// Reads a form id, gives it the file's compile index, looks it up (`004839c0`), casts the form
/// (`__RTDynamicCast`, type at `0x01183028` to the type at `0x011832fc`) and stores the result
/// in the pointer at `field`.
fn read_form_reference(e: &mut Engine, file: Ptr, field: u32) {
    let local = e.mem.alloc(4);
    e.call(0x0047_27f0, &args![file, local]);
    e.call(0x0048_5d50, &args![local, file]);
    let id = e.mem.u32(local);
    e.mem.free(local);
    let form = e.call(0x0048_39c0, &args![id]).u32();
    let cast = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, 0x0118_3028u32, 0x0118_32fcu32, 0u32],
        )
        .u32();
    e.mem.set_u32(field, cast);
}

// Translated from 00519b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<int_bool>` scalar deleting destructor (engine map name): runs the destructor body `00519bf0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_pointer_map_int_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0051_9bf0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00519c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned_int>_int_bool>` scalar deleting destructor (engine map name): runs the destructor body `00519c50` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_base_ni_t_pointer_allocator_unsigned_int_int_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0051_9c50, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The player reference (`011dea3c`).
const PLAYER_REFERENCE: u32 = 0x011d_ea3c;
/// The form `005180b0` asks the actor for first with `Actor::HasObjects` (`011ca268`; the
/// NPC path also hands it to `TESIdleManager::SetUsedItem`).
const USED_ITEM_FORM: u32 = 0x011c_a268;
/// The door being unlocked with a key while the player activates it (`011c9350`): set for
/// the duration of that activation, otherwise 0.
const UNLOCKING_DOOR: u32 = 0x011c_9350;
/// A byte `00518b70` sets from the player's `00425fd0` answer.
const PLAYER_FLAG_011C9354: u32 = 0x011c_9354;
/// The `float` 2.0 the code pushes (`FLD`/`FSTP`) as the message display time.
const MESSAGE_TIME: u32 = 0x0101_62c0;
/// `FLT_MAX` (`0101_6970`), pushed when the teleport cell is not loaded.
const FLOAT_MAX: u32 = 0x0101_6970;
/// `1.0` (`double`, `0101_2070`).
const ONE: u32 = 0x0101_2070;
/// `.?AVTESObjectREFR@@` and `.?AVActor@@`: the cast of the activating reference.
const OBJECT_REFR_TYPE: u32 = 0x0118_41cc;
const ACTOR_TYPE: u32 = 0x0118_46d4;
/// "UILockpickingUnlock".
const UNLOCK_SOUND: u32 = 0x0102_b13c;
/// "Interface\Icons\Message Icons\glow_message_key.dds".
const KEY_ICON: u32 = 0x0102_b108;
/// "Interface\Icons\Message Icons\glow_message_padlock.dds".
const PADLOCK_ICON: u32 = 0x0102_b0d0;
/// "Interface\Icons\Message Icons\glow_message_vaultboy_sad.dds".
const SAD_ICON: u32 = 0x0102_08a0;
/// The settings whose strings (`00403df0` on them) are the messages: "unlocked with key"
/// (a format), "cannot be picked", and the one shown when a door has no destination.
const UNLOCKED_WITH_KEY_SETTING: u32 = 0x011d_5110;
const CANNOT_PICK_SETTING: u32 = 0x011d_276c;
const NO_DESTINATION_SETTING: u32 = 0x011c_9384;
/// "MASTERFILE: Could not find RandomTeleport (%08X) for Door '%s' (%08X)."
const MISSING_TELEPORT_FORM: u32 = 0x0102_b380;
/// `BSAudio::QInstance` (`00453a70`, no arguments).
const AUDIO: u32 = 0x0045_3a70;
/// `BSAudio::GetSoundHandleByNumericID` (`00ad73b0`, `thiscall`: handle, ID, flags).
const SOUND_BY_ID: u32 = 0x00ad_73b0;
/// `BSSoundHandle::Play` (`00ad8830`, `thiscall`, one word) and `SetPosition` (`00ad8b60`,
/// three `float`s).
const SOUND_PLAY: u32 = 0x00ad_8830;
const SOUND_SET_POSITION: u32 = 0x00ad_8b60;
/// The sound handle's destructor (`00483710`).
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;

/// Whether bit `mask` of the flags byte at +0x84 is set.
fn door_flag(e: &mut Engine, this: Ptr, mask: u8) -> bool {
    e.mem.u8(this.addr() + 0x84) & mask != 0
}

// Translated from 00517fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): whether bit 1 (0x02) of the door's
/// flags byte at +0x84 is set.
pub fn fn_00517fa0(e: &mut Engine, this: Ptr) -> bool {
    door_flag(e, this, 0x02)
}

// Translated from 00518000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectDOOR::IsMinimalUse` (Xbox PDB; `ECX` only, plain `RET`): whether bit 3 (0x08)
/// of the flags byte at +0x84 is set.
pub fn tes_object_door_is_minimal_use(e: &mut Engine, this: Ptr) -> bool {
    door_flag(e, this, 0x08)
}

// Translated from 00518080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectDOOR::IsSlidingDoor` (Xbox PDB; `ECX` only, plain `RET`): whether bit 4 (0x10)
/// of the flags byte at +0x84 is set.
pub fn tes_object_door_is_sliding_door(e: &mut Engine, this: Ptr) -> bool {
    door_flag(e, this, 0x10)
}

// Translated from 005176b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectDOOR::InitItem` (Xbox PDB; `ECX` only, plain `RET`): unless `004013e0` says
/// otherwise, runs the components' `InitItem`s with the form (`0048cd90` on the scriptable
/// form at +0x5c, `00478be0` on the destructible object at +0x68, `0048aa80` on the model
/// texture swap at +0x3c), then resolves the form IDs in the list `00871a30` gives. Walking
/// from its first node while the node is not null and `008256d0` on it is false: the node's
/// ID (the word at `006815c0`'s address) is fixed up in a stack slot through `00485d50`
/// (cdecl, the slot and the door's file from `00484e60` with -1) and looked up with
/// `004839c0`. A found form is stored in the node (`00726c60` with the form's stack slot)
/// and the walk moves on (`00726070`), the node becoming the previous one. A missing one
/// prints the warning at `0x0102_b380` (the ID, the door's name from vtable slot +0x130 and
/// the word `0084e3a0` gives) and is removed: after a previous node through `00905330` on
/// that node (with a stack copy of the ID), the walk continuing at the previous node's next;
/// at the head through `0063f7b0` on the node, which is then checked again. Then
/// `00484ab0` with 1.
pub fn tes_object_door_init_item(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    e.call(0x0048_cd90, &args![base + 0x5c, this]);
    e.call(0x0047_8be0, &args![base + 0x68, this]);
    e.call(0x0048_aa80, &args![base + 0x3c, this]);
    let mut node = e.call(0x0087_1a30, &args![this]).u32();
    let mut previous = 0u32;
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        e.with_stack(0xc, |e, slots| {
            let id_slot = slots.addr();
            let form_slot = id_slot + 4;
            let copy_slot = id_slot + 8;
            let item = e.call(0x0068_15c0, &args![node]).u32();
            let loaded = e.mem.u32(item);
            e.mem.set_u32(id_slot, loaded);
            let file = e.call(0x0048_4e60, &args![this, 0xffff_ffffu32]).u32();
            e.call(0x0048_5d50, &args![id_slot, file]);
            let id = e.mem.u32(id_slot);
            let form = e.call(0x0048_39c0, &args![id]).u32();
            e.mem.set_u32(form_slot, form);
            if form != 0 {
                e.call(0x0072_6c60, &args![node, form_slot]);
                previous = node;
                node = e.call(0x0072_6070, &args![node]).u32();
                return;
            }
            let form_id = e.call(0x0084_e3a0, &args![this]).u32();
            let name = e.vcall(base, 0x130, &args![]).u32();
            e.call(
                0x005b_5e40,
                &args![MISSING_TELEPORT_FORM, id, name, form_id],
            );
            if previous != 0 {
                e.mem.set_u32(copy_slot, id);
                e.call(0x0090_5330, &args![previous, copy_slot]);
                node = e.call(0x0072_6070, &args![previous]).u32();
            } else {
                e.call(0x0063_f7b0, &args![node]);
            }
        });
    }
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

// Translated from 00518e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectDOOR::CanActorIgnoreLock` (Xbox PDB; cdecl, five words: the door lock, the
/// actor, the reference, two flag bytes): false without an actor or a lock. True when the
/// actor follows someone (`Actor::IsFollowing`) whose package target (vtable slot +0x128 of
/// `MiddleHighProcess::GetSavedAcquireObject`'s object) is the player and `00568e50` on the
/// reference is nonzero, or when the actor's vtable slot +0x448 answers true; otherwise
/// `DoorLock::CanActorIgnoreDoorLock` (`00501fd0` on the lock, with the actor and the two
/// flags) decides.
pub fn tes_object_door_can_actor_ignore_lock(
    e: &mut Engine,
    lock: Ptr,
    actor: Ptr,
    reference: Ptr,
    first_flag: u8,
    second_flag: u8,
) -> bool {
    if actor.is_null() || lock.is_null() {
        return false;
    }
    if e.call(0x0088_42c0, &args![actor]).bool() {
        let acquire = e.call(0x008d_8520, &args![actor]).u32();
        let target = e.vcall(acquire, 0x128, &args![]).u32();
        if target == e.global::<u32>(PLAYER_REFERENCE)
            && e.call(0x0056_8e50, &args![reference]).u32() != 0
        {
            return true;
        }
    }
    if e.vcall(actor.addr(), 0x448, &args![]).bool() {
        return true;
    }
    e.call(
        0x0050_1fd0,
        &args![lock, actor, u32::from(first_flag), u32::from(second_flag)],
    )
    .bool()
}

// Translated from 00518f00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00518f00 - the exception-unwinding frame (FS:[0] chain, handler 00ef4848, state variable) is not translated.
/// `TESObjectDOOR::CanActorIgnoreLock` (overload 2, Xbox PDB; cdecl, four words: the
/// reference, the actor, two flag bytes): builds a `DoorLock` for the reference on the stack
/// (`00501d20`, 0x18 bytes: the constructor writes +0x14; the exe's local is EBP-0x24..-0xc), asks [`tes_object_door_can_actor_ignore_lock`] with it, the
/// actor, the reference and the flags, destroys the lock (`00501e50`) and returns the
/// answer.
pub fn tes_object_door_can_actor_ignore_lock_ov2(
    e: &mut Engine,
    reference: Ptr,
    actor: Ptr,
    first_flag: u8,
    second_flag: u8,
) -> bool {
    e.with_stack(0x18, |e, lock| {
        e.call(0x0050_1d20, &args![lock, reference]);
        let answer = tes_object_door_can_actor_ignore_lock(
            e,
            lock,
            actor,
            reference,
            first_flag,
            second_flag,
        );
        e.call(0x0050_1e50, &args![lock]);
        answer
    })
}

// Translated from 00519290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00519290 - the exception-unwinding frame (FS:[0] chain, handler 00ef48c6, state variable) is not translated.
/// Unnamed in the engine map (no arguments, plain `RET`): sorts the player's followers
/// after a door teleport. With the follower data of the player's extra data list
/// (`00422700` on `005d43c0`'s list; nothing without it), two lists are allocated (8 bytes
/// each through `00401000`, constructed by `0096a2d0`, 0 when the allocation fails); every
/// follower node of the data (+0xc, walked until a node with no item) is appended to the
/// second (`00905820` with the node's item address). For each follower in the second list
/// that is not null: one that `Actor::CouldBeFollowing` the player stops moving
/// (`Actor::ForceStopMoving`) and its process gets `00693d50` with the calendar hour
/// (`Calendar::GetHour` on `011de7b8`) minus 1.0 as a `float`; any other goes into the first
/// list (`005ae3d0` with its stack slot). Each follower in the first list is then removed
/// from the player (`ExtraDataList::RemoveFollower`), the first list is emptied
/// (`00470470`) and deleted (`004702f0` with 1); the second is not freed. When the
/// data's follower list is now empty (`008256d0`), `ExtraDataList::RemoveFollowerExtra`
/// runs on the player's list.
pub fn fn_00519290(e: &mut Engine) {
    let player: u32 = e.global(PLAYER_REFERENCE);
    let extra = e.call(0x005d_43c0, &args![player]).u32();
    let followers = e.call(0x0042_2700, &args![extra]).u32();
    if followers == 0 {
        return;
    }
    let make_list = |e: &mut Engine| {
        let block = e.call(0x0040_1000, &args![8u32]).u32();
        if block == 0 {
            0
        } else {
            e.call(0x0096_a2d0, &args![block]).u32()
        }
    };
    let leaving = make_list(e);
    let staying = make_list(e);
    let mut node = e.mem.u32(followers + 0xc);
    while node != 0 {
        let item = e.call(0x0068_15c0, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(0x0068_15c0, &args![node]).u32();
        e.call(0x0090_5820, &args![staying, item]);
        node = e.call(0x0072_6070, &args![node]).u32();
    }
    let mut node = staying;
    while node != 0 {
        let item = e.call(0x0068_15c0, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(0x0068_15c0, &args![node]).u32();
        let actor = e.mem.u32(item);
        if actor != 0 {
            let player: u32 = e.global(PLAYER_REFERENCE);
            if e.call(0x008b_c860, &args![actor, player]).bool() {
                e.call(0x008b_3ad0, &args![actor]);
                let hour = e.call(0x0086_7da0, &args![0x011d_e7b8u32]).f64();
                let one: f64 = e.global(ONE);
                let delay = x87_store32(x87_sub(hour, one));
                let process = e.call(0x008d_8520, &args![actor]).u32();
                e.call(0x0069_3d50, &args![process, x87_quiet_bits(delay)]);
            } else {
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), actor);
                    e.call(0x005a_e3d0, &args![leaving, slot]);
                });
            }
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }
    let mut node = leaving;
    while node != 0 {
        let item = e.call(0x0068_15c0, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(0x0068_15c0, &args![node]).u32();
        let actor = e.mem.u32(item);
        let player: u32 = e.global(PLAYER_REFERENCE);
        let extra = e.call(0x005d_43c0, &args![player]).u32();
        e.call(0x0042_2690, &args![extra, actor]);
        node = e.call(0x0072_6070, &args![node]).u32();
    }
    e.call(0x0047_0470, &args![leaving]);
    if leaving != 0 {
        e.call(0x0047_02f0, &args![leaving, 1u32]);
    }
    let list = e.mem.u32(followers + 0xc);
    if e.call(0x0082_56d0, &args![list]).bool() {
        let player: u32 = e.global(PLAYER_REFERENCE);
        let extra = e.call(0x005d_43c0, &args![player]).u32();
        e.call(0x0042_2720, &args![extra]);
    }
}

// Translated from 00518cf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00518cf0 - the exception-unwinding frame (FS:[0] chain, handler 00ef4818, state variable) is not translated.
/// Unnamed in the engine map (cdecl, one word: the door reference; `00518b70` stores it as
/// the callback of the player's position request): with a reference, the followers are
/// sorted ([`fn_00519290`]); when the player's object from `008d6f30` exists and `00425fd0`
/// on it is true, `0054af80` runs on it; when the door's base form (`007af430` on the
/// reference) has a sound (`00407840`) and the linked reference (`00559450` on `00568e50`'s
/// data) answers nonzero through vtable slot +0x1d0, that sound plays (numeric ID through
/// `0084e3a0`, flags 0x121). Then, with or without a reference: when the player's object
/// exists and `00425fd0` on it is false, `00557be0` runs on it; when the byte at
/// `00408d60`'s address for the setting at `011d8980` is set and the object exists,
/// `00851d30` runs on the object at `011de134`.
pub fn fn_00518cf0(e: &mut Engine, reference: Ptr) {
    if !reference.is_null() {
        fn_00519290(e);
        let player: u32 = e.global(PLAYER_REFERENCE);
        if e.call(0x008d_6f30, &args![player]).u32() != 0 {
            let player: u32 = e.global(PLAYER_REFERENCE);
            let object = e.call(0x008d_6f30, &args![player]).u32();
            if e.call(0x0042_5fd0, &args![object]).bool() {
                let player: u32 = e.global(PLAYER_REFERENCE);
                let object = e.call(0x008d_6f30, &args![player]).u32();
                e.call(0x0054_af80, &args![object]);
            }
        }
        let door = e.call(0x007a_f430, &args![reference]).u32();
        if e.call(0x0040_7840, &args![door]).u32() != 0 {
            let data = e.call(0x0056_8e50, &args![reference]).u32();
            if e.call(0x0055_9450, &args![data]).u32() != 0 {
                let data = e.call(0x0056_8e50, &args![reference]).u32();
                let linked = e.call(0x0055_9450, &args![data]).u32();
                if e.vcall(linked, 0x1d0, &args![]).u32() != 0 {
                    let sound = e.call(0x0040_7840, &args![door]).u32();
                    let sound_id = e.call(0x0084_e3a0, &args![sound]).u32();
                    e.with_stack(0xc, |e, handle| {
                        let audio = e.call(AUDIO, &args![]).u32();
                        e.call(SOUND_BY_ID, &args![audio, handle, sound_id, 0x121u32]);
                        e.call(SOUND_PLAY, &args![handle, 0u32]);
                        e.call(SOUND_HANDLE_DESTROY, &args![handle]);
                    });
                }
            }
        }
    }
    let player: u32 = e.global(PLAYER_REFERENCE);
    let object = e.call(0x008d_6f30, &args![player]).u32();
    if object != 0 && !e.call(0x0042_5fd0, &args![object]).bool() {
        e.call(0x0055_7be0, &args![object]);
    }
    let flag = e.call(0x0040_8d60, &args![0x011d_8980u32]).u32();
    if e.mem.u8(flag) != 0 && object != 0 {
        let target: u32 = e.global(0x011d_e134);
        e.call(0x0085_1d30, &args![target]);
    }
}

// Translated from 00518b70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00518b70 - the exception-unwinding frame (FS:[0] chain, handler 00eecb2b, state variable) is not translated.
/// Unnamed in the engine map (`ECX` the door, one stack word: the door reference,
/// `RET 4`): when the reference is not null, its base form (`007af430`) is this door and it
/// has teleport data (`00568e50`): a 0x34-byte position request is allocated (`00401000`,
/// constructed by `00456540`, 0 when the allocation fails) and filled: the 12 bytes at the
/// teleport data's `00717e50` address at +8, those at its `00460140` address at +0x14, 0 at
/// byte +0x20, the callback [`fn_00518cf0`] (`0x0051_8cf0`) at +0x24 and the reference at
/// +0x28; the destination is the teleport cell (`DoorTeleportData::GetTeleportCell`) at
/// +4 when it is set and `00425fd0` on it is true, else the worldspace
/// (`GetTeleportWorldSpace`) at +0. `PlayerCharacter::RequestPositionPlayer` gets it; then
/// the byte at `011c9354` is set to `00425fd0`'s answer for the player's object from
/// `008d6f30` (0 without one).
pub fn fn_00518b70(e: &mut Engine, this: Ptr, reference: Ptr) {
    if reference.is_null() || e.call(0x007a_f430, &args![reference]).u32() != this.addr() {
        return;
    }
    let teleport = e.call(0x0056_8e50, &args![reference]).u32();
    if teleport == 0 {
        return;
    }
    let cell = e.call(0x0043_a2b0, &args![teleport]).u32();
    let worldspace = e.call(0x0043_a320, &args![teleport]).u32();
    let block = e.call(0x0040_1000, &args![0x34u32]).u32();
    let request = if block == 0 {
        0
    } else {
        e.call(0x0045_6540, &args![block]).u32()
    };
    let position = e.call(0x0071_7e50, &args![teleport]).u32();
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(position + offset);
        e.mem.set_u32(request.wrapping_add(8 + offset), word);
    }
    let rotation = e.call(0x0046_0140, &args![teleport]).u32();
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(rotation + offset);
        e.mem.set_u32(request.wrapping_add(0x14 + offset), word);
    }
    e.mem.set_u8(request.wrapping_add(0x20), 0);
    e.mem.set_u32(request.wrapping_add(0x24), 0x0051_8cf0);
    e.mem.set_u32(request.wrapping_add(0x28), reference.addr());
    if cell != 0 && e.call(0x0042_5fd0, &args![cell]).bool() {
        e.mem.set_u32(request.wrapping_add(4), cell);
    } else {
        e.mem.set_u32(request, worldspace);
    }
    let player: u32 = e.global(PLAYER_REFERENCE);
    e.call(0x0093_be30, &args![player, request]);
    let player: u32 = e.global(PLAYER_REFERENCE);
    let object = e.call(0x008d_6f30, &args![player]).u32();
    let flag = object != 0 && e.call(0x0042_5fd0, &args![object]).bool();
    e.set_global(PLAYER_FLAG_011C9354, u8::from(flag));
}

/// The message display time pushed as a `float` (`FLD`/`FSTP` of `0101_62c0`).
fn message_time(e: &mut Engine) -> u32 {
    x87_quiet_bits(e.global::<u32>(MESSAGE_TIME))
}

/// Unlocks the reference's lock: with the extra data `TESObjectREFR::GetExtraLock`
/// (`005691f0`) gives, `00517360` (in `tesobjectcont.cpp`) unlocks it and
/// `TESObjectREFR::AddLockChange` (`005789b0`) records the change.
fn unlock(e: &mut Engine, reference: Ptr) {
    let extra = e.call(0x0056_91f0, &args![reference]).u32();
    if extra != 0 {
        e.call(0x0051_7360, &args![extra]);
        e.call(0x0057_89b0, &args![reference]);
    }
}

/// The player picking a lock without the key: the entry point 0x20 runs
/// (`BGSEntryPoint::HandleEntryPoint` with the player and a zeroed `float`); when the lock
/// is level 5 or broken the "cannot pick" message shows with the padlock icon and menu sound
/// 0x12 (`Interface::PlayMenuSound`), otherwise `00705830` starts lockpicking.
fn player_picks_lock(e: &mut Engine, reference: Ptr) {
    let extra = e.call(0x0056_91f0, &args![reference]).u32();
    e.with_stack(4, |e, bonus| {
        e.mem.set_f32(bonus.addr(), 0.0);
        let player: u32 = e.global(PLAYER_REFERENCE);
        e.call(0x005e_58f0, &args![0x20u32, player, bonus]);
    });
    let unpickable = extra != 0 && {
        let lock_data = e.mem.u32(extra + 0xc);
        e.call(0x0043_09e0, &args![lock_data, reference]).u32() == 5 || {
            let lock_data = e.mem.u32(extra + 0xc);
            e.call(0x0043_0ae0, &args![lock_data]).bool()
        }
    };
    if unpickable {
        let time = message_time(e);
        let text = e.call(0x0040_3df0, &args![CANNOT_PICK_SETTING]).u32();
        e.call(
            0x0070_52f0,
            &args![text, 0u32, PADLOCK_ICON, 0u32, time, 0u32],
        );
        e.call(0x0070_6f30, &args![0x12u32]);
    } else {
        e.call(0x0070_5830, &args![reference, 0u32, 0u32]);
    }
}

/// An NPC using the lock: a special idle its process (vtable slot +0x1e4) has is freed
/// (`Animation::SpecialIdleFree` with 1, 0), the used item is set (`TESIdleManager`: the
/// form at `011ca268`, activate 1, level 1) around vtable slot +0x44 of `acquire` (the
/// actor, 0, 2, 1, 0, 1), reset (0, 0, -1), and the lock is unlocked.
fn npc_uses_lock(e: &mut Engine, actor: u32, acquire: u32, reference: Ptr) {
    let process = e.vcall(actor, 0x1e4, &args![]).u32();
    if process != 0 {
        e.call(0x0049_8910, &args![process, 1u32, 0u32]);
    }
    let used_item: u32 = e.global(USED_ITEM_FORM);
    e.call(0x0060_0900, &args![used_item]);
    e.call(0x0060_0940, &args![1u32]);
    e.call(0x0060_0920, &args![1u32]);
    e.vcall(acquire, 0x44, &args![actor, 0u32, 2u32, 1u32, 0u32, 1u32]);
    e.call(0x0060_0900, &args![0u32]);
    e.call(0x0060_0940, &args![0u32]);
    e.call(0x0060_0920, &args![0xffff_ffffu32]);
    unlock(e, reference);
}

/// What the locked part of the activation decided.
enum LockOutcome {
    /// Return this answer now.
    Answer(bool),
    /// Go on to the door's activation.
    Continue,
}

/// The locked part of [`fn_005180b0`] (the lock is active and the actor cannot ignore it).
fn activate_locked(
    e: &mut Engine,
    reference: Ptr,
    actor: u32,
    acquire: u32,
    lock: u32,
) -> LockOutcome {
    e.with_stack(4, |e, found| {
        let mut has_used_item = false;
        if e.vcall(actor, 0x218, &args![]).bool() {
            let used_item: u32 = e.global(USED_ITEM_FORM);
            has_used_item = e
                .call(
                    0x0089_1db0,
                    &args![actor, used_item, 0u32, 1u32, 0u32, found],
                )
                .bool();
        }
        let key = e.mem.u32(lock + 4);
        let without_key = key == 0
            || (e.global::<u32>(UNLOCKING_DOOR) == 0
                && !e
                    .call(0x0089_1db0, &args![actor, key, 0u32, 1u32, 0u32, found])
                    .bool());
        if without_key {
            if actor == e.global::<u32>(PLAYER_REFERENCE) {
                player_picks_lock(e, reference);
                return LockOutcome::Answer(false);
            }
            if !has_used_item {
                return LockOutcome::Answer(false);
            }
            npc_uses_lock(e, actor, acquire, reference);
            return LockOutcome::Continue;
        }
        if actor != e.global::<u32>(PLAYER_REFERENCE) || e.global::<u32>(UNLOCKING_DOOR) != 0 {
            return LockOutcome::Continue;
        }
        e.set_global(UNLOCKING_DOOR, reference.addr());
        unlock(e, reference);
        e.with_stack(0xc, |e, handle| {
            let audio = e.call(AUDIO, &args![]).u32();
            let sound = e
                .call(0x00ad_7550, &args![audio, handle, UNLOCK_SOUND, 0x121u32])
                .u32();
            e.call(SOUND_PLAY, &args![sound, 0u32]);
            e.call(SOUND_HANDLE_DESTROY, &args![handle]);
        });
        let key = e.mem.u32(lock + 4);
        let name = e.call(0x0040_8da0, &args![key + 0x30]).u32();
        let format = e.call(0x0040_3df0, &args![UNLOCKED_WITH_KEY_SETTING]).u32();
        e.with_stack(0x100, |e, text| {
            e.call(0x0040_6d00, &args![text, 0xffu32, format, name]);
            let time = message_time(e);
            e.call(0x0070_52f0, &args![text, 0u32, KEY_ICON, 0u32, time, 0u32]);
        });
        let unlocking: u32 = e.global(UNLOCKING_DOOR);
        let player: u32 = e.global(PLAYER_REFERENCE);
        e.call(0x0057_3170, &args![unlocking, player, 0u32, 0u32, 1u32]);
        e.set_global(UNLOCKING_DOOR, 0u32);
        LockOutcome::Answer(true)
    })
}

/// Plays the door's open sound for the activation (`00407820` on the door; numeric ID
/// through `0084e3a0`): flags 0x80000121 for the player, 0x102 (at the reference's
/// position, vtable slot +0x1f4) for an NPC. The handle (`0041a250`) is assigned through
/// `00418900` from a temporary that `GetSoundHandleByNumericID` fills. For the player, when
/// `005bb4d0` on the audio is false, `00ad7740` runs four times with 0.
fn play_open_sound(e: &mut Engine, this: Ptr, reference: Ptr, actor: u32) {
    e.with_stack(0x18, |e, handles| {
        let handle = handles.addr();
        let temporary = handle + 0xc;
        e.call(0x0041_a250, &args![handle]);
        let is_player = actor == e.global::<u32>(PLAYER_REFERENCE);
        let flags = if is_player { 0x8000_0121u32 } else { 0x102 };
        let sound = e.call(0x0040_7820, &args![this]).u32();
        let sound_id = e.call(0x0084_e3a0, &args![sound]).u32();
        let audio = e.call(AUDIO, &args![]).u32();
        let filled = e
            .call(SOUND_BY_ID, &args![audio, temporary, sound_id, flags])
            .u32();
        e.call(0x0041_8900, &args![handle, filled]);
        e.call(SOUND_HANDLE_DESTROY, &args![temporary]);
        if actor != e.global::<u32>(PLAYER_REFERENCE) {
            let position = e.vcall(reference.addr(), 0x1f4, &args![]).u32();
            let x = e.mem.u32(position);
            let y = e.mem.u32(position + 4);
            let z = e.mem.u32(position + 8);
            e.call(
                SOUND_SET_POSITION,
                &args![
                    handle,
                    x87_quiet_bits(x),
                    x87_quiet_bits(y),
                    x87_quiet_bits(z)
                ],
            );
        }
        e.call(SOUND_PLAY, &args![handle, 0u32]);
        if actor == e.global::<u32>(PLAYER_REFERENCE) {
            let audio = e.call(AUDIO, &args![]).u32();
            if !e.call(0x005b_b4d0, &args![audio]).bool() {
                for _ in 0..4 {
                    let audio = e.call(AUDIO, &args![]).u32();
                    e.call(0x00ad_7740, &args![audio, 0u32]);
                }
            }
        }
        e.call(SOUND_HANDLE_DESTROY, &args![handle]);
    });
}

// Translated from 005180b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005180b0 - the exception-unwinding frame (FS:[0] chain, handler 00ef47dc, state variable) and the /GS cookie check (00ec408c) are not translated.
/// The `TESObjectDOOR` activation (unnamed in the engine map; `ECX` the door and five stack
/// words, `RET 0x14`: the door reference, the activating reference and three words never
/// read). Returns `AL`.
///
/// False without a reference or when `00477ba0` on it is true. A reference without teleport
/// data (`00568e50`) on a door with random destinations (`005194d0`) gets one through
/// `005195e0` (the reference, the activator) and `00519810`; without one the "no
/// destination" message (setting `011c9384`, the sad Vault Boy icon) shows and the answer
/// is false, and so it is when the data is still missing afterwards.
///
/// For an activating actor (`__RTDynamicCast` to `Actor`): false when its acquire object's
/// vtable slot +0x4c8 and its own slot +0x214 both answer nonzero; the player's word at
/// +0x200 is cleared; when the reference's lock (`TESObjectREFR::GetLock`) is active
/// (`005021a0`) and the actor cannot ignore it
/// ([`tes_object_door_can_actor_ignore_lock_ov2`] with 1, 1), the locked part runs: the
/// player picks a lock without the key, an NPC that has the used item uses it and goes on,
/// the player with the key unlocks the door and activates it (true), see
/// `activate_locked`.
///
/// With teleport data and an actor: unless the player is sleeping or resting
/// (`PlayerCharacter::IsSleepingorResting`), `0045cd60` on the acquire object is nonzero or
/// its vtable slot +0x610 answers 4, the door's open sound plays (with a sound, a reference
/// and the reference's slot +0x1d0 nonzero; see `play_open_sound`) and
/// `HighProcess::FadeOut` (`008fe960` with the actor, the reference, 1) starts; the answer
/// is whether the actor is the player. Otherwise the acquire object's slot +0x134 gets the
/// reference and the actor moves: the player through `00962880` and [`fn_00518b70`] (with
/// the reference recorded at player +0x604 when not in an interior, and byte +0x20c set
/// around it); an NPC through `SetLocationOnReference` (the teleport data's `00717e50`
/// position), `005757d0` with the destination's angle (`00460140` +8) and `00575770` with
/// 0.0 when the cell is loaded (`TES::IsCellLoaded` on `011dea10`), else `00575770` with
/// `FLT_MAX`, `Actor::SetForceUpdateQuest(0)` for an actor and
/// `TESObjectREFR::MoveRefToNewSpace` (cdecl: the activator, cell, worldspace). True.
///
/// Without teleport data or an actor: the lock is unlocked and
/// `BGSOpenCloseForm::HandleActivate` runs (the reference, the activator, the
/// open-close component at +0x70 or 0); true.
pub fn fn_005180b0(
    e: &mut Engine,
    this: Ptr,
    reference: Ptr,
    activator: Ptr,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> bool {
    let base = this.addr();
    if reference.is_null() || e.call(0x0047_7ba0, &args![reference]).bool() {
        return false;
    }
    let mut teleport = e.call(0x0056_8e50, &args![reference]).u32();
    let lock = e.call(0x0056_9160, &args![reference]).u32();
    if teleport == 0 && e.call(0x0051_94d0, &args![this]).bool() {
        let destination = e
            .call(0x0051_95e0, &args![this, reference, activator])
            .u32();
        if destination == 0 {
            let time = message_time(e);
            let text = e.call(0x0040_3df0, &args![NO_DESTINATION_SETTING]).u32();
            e.call(0x0070_52f0, &args![text, 0u32, SAD_ICON, 0u32, time, 0u32]);
            return false;
        }
        e.call(0x0051_9810, &args![reference, destination]);
        teleport = e.call(0x0056_8e50, &args![reference]).u32();
        if teleport == 0 {
            return false;
        }
    }
    let actor = e
        .call(
            0x00ec_43fb,
            &args![activator, 0u32, OBJECT_REFR_TYPE, ACTOR_TYPE, 0u32],
        )
        .u32();
    let acquire = if actor == 0 {
        0
    } else {
        e.call(0x008d_8520, &args![actor]).u32()
    };
    if actor != 0 {
        let process = e.call(0x008d_8520, &args![actor]).u32();
        if e.vcall(process, 0x4c8, &args![]).u32() != 0
            && e.vcall(actor, 0x214, &args![]).u32() != 0
        {
            return false;
        }
        let player: u32 = e.global(PLAYER_REFERENCE);
        if actor == player {
            e.mem.set_u32(player + 0x200, 0);
        }
        let ignores =
            tes_object_door_can_actor_ignore_lock_ov2(e, reference, Ptr::new(actor), 1, 1);
        if lock != 0 && e.call(0x0050_21a0, &args![lock]).bool() && !ignores {
            if let LockOutcome::Answer(answer) = activate_locked(e, reference, actor, acquire, lock)
            {
                return answer;
            }
        }
    }
    if teleport == 0 || actor == 0 {
        unlock(e, reference);
        let open_close = if base == 0 { 0 } else { base + 0x70 };
        e.call(0x0047_a560, &args![reference, activator, open_close]);
        return true;
    }
    let player: u32 = e.global(PLAYER_REFERENCE);
    if !e.call(0x0094_df60, &args![player]).bool()
        && e.call(0x0045_cd60, &args![acquire]).u32() == 0
        && e.vcall(acquire, 0x610, &args![]).u32() != 4
    {
        if e.call(0x0040_7820, &args![this]).u32() != 0
            && !reference.is_null()
            && e.vcall(reference.addr(), 0x1d0, &args![]).u32() != 0
        {
            play_open_sound(e, this, reference, actor);
        }
        e.call(0x008f_e960, &args![acquire, actor, reference, 1u32]);
        return actor == e.global::<u32>(PLAYER_REFERENCE);
    }
    let cell = e.call(0x0043_a2b0, &args![teleport]).u32();
    let worldspace = e.call(0x0043_a320, &args![teleport]).u32();
    let process = e.call(0x008d_8520, &args![actor]).u32();
    e.vcall(process, 0x134, &args![reference]);
    let player: u32 = e.global(PLAYER_REFERENCE);
    if actor == player {
        if !e.call(0x0057_5d10, &args![player]).bool() {
            let player: u32 = e.global(PLAYER_REFERENCE);
            e.mem.set_u32(player + 0x604, reference.addr());
        }
        let player: u32 = e.global(PLAYER_REFERENCE);
        e.mem.set_u8(player + 0x20c, 1);
        e.call(0x0096_2880, &args![player]);
        fn_00518b70(e, this, reference);
        let player: u32 = e.global(PLAYER_REFERENCE);
        e.mem.set_u8(player + 0x20c, 0);
        return true;
    }
    let position = e.call(0x0071_7e50, &args![teleport]).u32();
    e.call(0x0057_5830, &args![activator, position]);
    let tes: u32 = e.global(0x011d_ea10);
    if cell != 0 && e.call(0x0045_11e0, &args![tes, cell, 0u32]).bool() {
        let rotation = e.call(0x0046_0140, &args![teleport]).u32();
        let angle = x87_quiet_bits(e.mem.u32(rotation + 8));
        e.call(0x0057_57d0, &args![activator, angle]);
        e.call(0x0057_5770, &args![activator, 0.0f32.to_bits()]);
    } else {
        let far = x87_quiet_bits(e.global::<u32>(FLOAT_MAX));
        e.call(0x0057_5770, &args![activator, far]);
    }
    let moved_actor = e
        .call(
            0x00ec_43fb,
            &args![activator, 0u32, OBJECT_REFR_TYPE, ACTOR_TYPE, 0u32],
        )
        .u32();
    if moved_actor != 0 {
        e.call(0x008a_e640, &args![moved_actor, 0u32]);
    }
    e.call(0x0057_3800, &args![activator, cell, worldspace]);
    true
}

/// `TESForm::StartForm` (`004855a0`) and `TESForm::CloseForm` (`00485680`),
/// `thiscall` on the form, no stack words.
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
/// The component saves (engine map; `thiscall`): `TESFullName::Save`
/// (`00487010`), `TESModelTextureSwap::Save` (`0048a520`, three chunk
/// tags), `TESScriptableForm::Save` (`0048cca0`) and
/// `BGSDestructibleObjectForm::Save` (`00477f20`).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const MODEL_TEXTURE_SWAP_SAVE: u32 = 0x0048_a520;
const SCRIPTABLE_FORM_SAVE: u32 = 0x0048_cca0;
const DESTRUCTIBLE_SAVE: u32 = 0x0047_7f20;
/// One-word (`00485910`) and one-byte (`004858f0`) chunks, cdecl(chunk,
/// value).
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
const ADD_CHUNK_BYTE: u32 = 0x0048_58f0;
/// The form-id getter (`0084e3a0`, `thiscall`: the word at +0xc).
const FORM_ID: u32 = 0x0084_e3a0;
/// `BSSimpleList` node helpers (`thiscall` on a node): whether it holds no
/// item (`008256d0`), its item's address (`006815c0`), the next node
/// (`00726070`); `00905820` (`RET 4`: the address of an item) appends to a
/// list; `005ae380` counts the nodes with an item; `005f65d0` (`RET 4`:
/// the address of an item) whether a list holds that item.
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const LIST_APPEND: u32 = 0x0090_5820;
const LIST_COUNT: u32 = 0x005a_e380;
const LIST_HOLDS: u32 = 0x005f_65d0;
/// The door's random-teleport list (`00871a30`: `this + 0x88`) and its
/// clear (`00519550`).
const TELEPORT_LIST: u32 = 0x0087_1a30;
const TELEPORT_LIST_CLEAR: u32 = 0x0051_9550;
/// The door's sound getters (`thiscall`): +0x78 (`00407820`), +0x7c
/// (`00407840`), +0x80 (`004fb070`).
const SOUND_78: u32 = 0x0040_7820;
const SOUND_7C: u32 = 0x0040_7840;
const SOUND_80: u32 = 0x004f_b070;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and the type
/// descriptors `TESForm` (`01183028`), `TESBoundObject` (`01183108`) and
/// `TESObjectDOOR` (`01186530`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_BOUND_OBJECT: u32 = 0x0118_3108;
const TYPE_TES_OBJECT_DOOR: u32 = 0x0118_6530;
/// `TESForm::CopyAllComponents` (`004851b0`, `RET 4`) and
/// `TESForm::CompareAllComponents` (`00485270`, `RET 4`).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `NiTPointerMap<int,bool>`'s and `NiTMapBase<..,int,bool>`'s vtables
/// (their slot 0 is `00519b50` and `00519c80`), the map's clear
/// (`00438af0`), `NiAlloc` (`00aa1070`, cdecl), `NiFree` (`00aa10f0`, cdecl)
/// and the `memset` wrapper (`00403d30`, cdecl).
const POINTER_MAP_VTABLE: u32 = 0x0102_b3d0;
const MAP_BASE_VTABLE: u32 = 0x0102_b3f0;
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
const NI_ALLOC: u32 = 0x00aa_1070;
const NI_FREE: u32 = 0x00aa_10f0;
const MEMSET: u32 = 0x0040_3d30;
/// `005195e0`'s callees: `TESObjectREFR::GetWorldSpace` (`00575d70`), the
/// reference's `008d6f30` and `00425fd0` on its answer, the visited map's
/// constructor (`00519b20`, `RET 4`: the bucket count), lookup (`0057c850`,
/// `RET 8`: key, out byte) and insert (`0084d310`, `RET 8`: key, value), the
/// random number (`00487f50`), the player's `00962810` (`RET 4`) and
/// `00962850` (`RET 8`), and the cell search `0054afb0` (cdecl, three
/// words).
const GET_WORLD_SPACE: u32 = 0x0057_5d70;
const REFERENCE_PARENT_008D6F30: u32 = 0x008d_6f30;
const PARENT_CHECK_00425FD0: u32 = 0x0042_5fd0;
const VISITED_MAP_CONSTRUCT: u32 = 0x0051_9b20;
const VISITED_MAP_LOOKUP: u32 = 0x0057_c850;
const VISITED_MAP_INSERT: u32 = 0x0084_d310;
const RANDOM: u32 = 0x0048_7f50;
const PLAYER_PREFERRED_DOOR: u32 = 0x0096_2810;
const PLAYER_SET_DOOR_CHOICE: u32 = 0x0096_2850;
const FIND_DESTINATION: u32 = 0x0054_afb0;
/// `00519810`'s callees: the reference's base object
/// (`BGSSaveFormBuffer::GetForm` in the engine map, `007af430`), its
/// teleport data (`00568e50`) and `TESObjectREFR::AddTeleport` (`00568e70`),
/// the linked door setter on the teleport data (`00537e90`, `RET 4`), the
/// reference's marker (`00569b20`), position (vtable slot `+0x1f4`) and
/// rotation (`00430830`), `TESObjectREFR::GetDeltaForAngle` (`00572500`,
/// `RET 8`: out vector, angle), the vector's scale (`00439180`, `RET 4`),
/// sum (`00439e90`, `RET 8`: out, other) and set (`00416870`, `RET 0xc`),
/// the teleport data's position (`006f4ed0`, `RET 4`) and rotation
/// (`0043a280`, `RET 4`) setters, a float setting's address (`00403e20`),
/// and `00404010` (cdecl, two floats, the answer in `ST0`).
const BASE_OBJECT: u32 = 0x007a_f430;
const TELEPORT_DATA: u32 = 0x0056_8e50;
const ADD_TELEPORT: u32 = 0x0056_8e70;
const SET_LINKED_DOOR: u32 = 0x0053_7e90;
const TELEPORT_MARKER: u32 = 0x0056_9b20;
const REFERENCE_ROTATION: u32 = 0x0043_0830;
const DELTA_FOR_ANGLE: u32 = 0x0057_2500;
const VECTOR_SCALE: u32 = 0x0043_9180;
const VECTOR_ADD: u32 = 0x0043_9e90;
const VECTOR_SET: u32 = 0x0041_6870;
const SET_TELEPORT_POSITION: u32 = 0x006f_4ed0;
const SET_TELEPORT_ROTATION: u32 = 0x0043_a280;
const SETTING_FLOAT_ADDRESS: u32 = 0x0040_3e20;
const FLOAT_PAIR_00404010: u32 = 0x0040_4010;
/// The two float settings `00519810` reads (`011c9398`, `011c93c4`), the
/// double added to the second (100.0 at `01017a40`) and the angle at
/// `0102b3c8`.
const DOOR_DISTANCE_SETTING: u32 = 0x011c_9398;
const LOAD_DOOR_DISTANCE_SETTING: u32 = 0x011c_93c4;
const DISTANCE_EXTRA: u32 = 0x0101_7a40;
const DOOR_ANGLE: u32 = 0x0102_b3c8;
/// `00759030`'s: the sound form of its argument (`005e9100`).
const ARRIVAL_SOUND: u32 = 0x005e_9100;

// Translated from 00517c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The door's save (engine map: no name; no stack words): `StartForm`, the
/// full name (+0x30), the model with texture swaps (+0x3c; `MODL`, `MODT`,
/// `MODS`), script (+0x5c), destructible data (+0x68), the sounds at +0x78,
/// +0x7c, +0x80 by id when set (`SNAM`, `ANAM`, `BNAM`), the flags byte
/// (+0x84) as `FNAM`, the id of each item of the teleport list (+0x88) as
/// `TNAM` until a node holds none, `CloseForm`.
pub fn fn_00517c30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![base + 0x30]);
    e.call(
        MODEL_TEXTURE_SWAP_SAVE,
        &args![base + 0x3c, MODL, MODT, MODS],
    );
    e.call(SCRIPTABLE_FORM_SAVE, &args![base + 0x5c]);
    e.call(DESTRUCTIBLE_SAVE, &args![base + 0x68]);
    for (offset, tag) in [(0x78u32, SNAM), (0x7c, ANAM), (0x80, BNAM)] {
        let form = e.mem.u32(base + offset);
        if form != 0 {
            let id = e.call(FORM_ID, &args![form]).u32();
            e.call(ADD_CHUNK_WORD, &args![tag, id]);
        }
    }
    let flags = u32::from(e.mem.u8(base + 0x84));
    e.call(ADD_CHUNK_BYTE, &args![FNAM, flags]);
    let mut node = e.call(TELEPORT_LIST, &args![this]).u32();
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let form = e.mem.u32(item);
        let id = e.call(FORM_ID, &args![form]).u32();
        e.call(ADD_CHUNK_WORD, &args![TNAM, id]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(CLOSE_FORM, &args![this]);
}

/// `other` as a `TESObjectDOOR` (`__RTDynamicCast` from `TESForm`), or 0.
fn as_door(e: &mut Engine, other: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![other, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_DOOR, 0u32],
    )
    .u32()
}

// Translated from 00517d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The door's copy (engine map: no name; `RET 4`): when `other` is a
/// `TESObjectDOOR`, empties this teleport list (`00519550`) and appends each
/// of the other's items until a node holds none (`00905820` on the current
/// destination node, which then moves on to its next node when there is
/// one), copies the flags byte (+0x84) and the three sounds (+0x78, +0x7c,
/// +0x80), then all components (`TESForm::CopyAllComponents` with
/// `other`).
pub fn fn_00517d70(e: &mut Engine, this: Ptr, other: u32) {
    let source = as_door(e, other);
    if source == 0 {
        return;
    }
    e.call(TELEPORT_LIST_CLEAR, &args![this]);
    let mut node = e.call(TELEPORT_LIST, &args![source]).u32();
    let mut target = e.call(TELEPORT_LIST, &args![this]).u32();
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        e.call(LIST_APPEND, &args![target, item]);
        if e.call(NODE_NEXT, &args![target]).u32() != 0 {
            target = e.call(NODE_NEXT, &args![target]).u32();
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let flags = e.mem.u8(source + 0x84);
    e.mem.set_u8(this.addr() + 0x84, flags);
    for (getter, offset) in [(SOUND_78, 0x78u32), (SOUND_7C, 0x7c), (SOUND_80, 0x80)] {
        let sound = e.call(getter, &args![source]).u32();
        e.mem.set_u32(this.addr() + offset, sound);
    }
    e.call(COPY_ALL_COMPONENTS, &args![this, other]);
}

// Translated from 00517e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The door's compare (engine map: no name; `RET 4`): true when `other` is
/// no `TESObjectDOOR`, when `TESForm::CompareAllComponents` (with the cast
/// form) says they differ, when a sound (+0x78, +0x7c, +0x80) or the flags
/// byte differs, when the teleport lists hold different numbers of items
/// (`005ae380`, this one's first), or when an item of the other's list is
/// not in this one's (`005f65d0`).
pub fn fn_00517e70(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let source = as_door(e, other);
    if source == 0 {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, source]).u32() as u8 != 0 {
        return true;
    }
    for (getter, offset) in [(SOUND_78, 0x78u32), (SOUND_7C, 0x7c), (SOUND_80, 0x80)] {
        let theirs = e.call(getter, &args![source]).u32();
        if e.mem.u32(this.addr() + offset) != theirs {
            return true;
        }
    }
    if e.mem.u8(this.addr() + 0x84) != e.mem.u8(source + 0x84) {
        return true;
    }
    let mut node = e.call(TELEPORT_LIST, &args![source]).u32();
    let list = e.call(TELEPORT_LIST, &args![this]).u32();
    let mine = e.call(LIST_COUNT, &args![list]).u32();
    if mine != e.call(LIST_COUNT, &args![node]).u32() {
        return true;
    }
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let list = e.call(TELEPORT_LIST, &args![this]).u32();
        if e.call(LIST_HOLDS, &args![list, item]).u32() as u8 == 0 {
            return true;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 00517fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): whether bit 2 (0x04)
/// of the door's flags byte at +0x84 is set.
pub fn fn_00517fd0(e: &mut Engine, this: Ptr) -> bool {
    door_flag(e, this, 0x04)
}

// Translated from 00518030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on` non-zero) or clears bit 3 (0x08) of the door's flags byte at
/// +0x84 (engine map: no name; `RET 4`, a byte).
pub fn fn_00518030(e: &mut Engine, this: Ptr, on: u8) {
    let flags = e.mem.u8(this.addr() + 0x84);
    let flags = if on != 0 { flags | 0x08 } else { flags & 0xf7 };
    e.mem.set_u8(this.addr() + 0x84, flags);
}

// Translated from 005195e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005195e0 - the FS:[0] exception frame (it destroys the visited map if a step throws) is not translated.
/// Picks a random teleport destination for the door (engine map: no name;
/// `RET 8`, the reference and a word it does not read): 0 without a
/// reference or without a world space (`TESObjectREFR::GetWorldSpace`, else
/// the reference's `008d6f30` when `00425fd0` passes it). Otherwise, with a
/// visited map (`00519b20` with 0x25 buckets) and the player's preferred
/// index (`00962810` with the door), up to as many tries as the teleport
/// list holds items, until a destination is found: a random index
/// (`00487f50` modulo the count) not marked in the map (redrawn while the
/// map holds it with a set value), the list's item at that index (0 past
/// the end), `0054afb0(item, world space, &alternative)` as the
/// destination; the first index giving an alternative is kept as the
/// choice; a destination at the preferred index becomes the alternative
/// and the search goes on; the index the walk stopped at is marked
/// (`0084d310` with 1). With an index drawn, the player gets its low byte
/// (`00962850` with the door). The map is destroyed (`00519bf0`). Answers
/// the destination.
pub fn fn_005195e0(e: &mut Engine, this: Ptr, reference: u32, _unused_2: u32) -> u32 {
    let mut found = 0u32;
    if reference == 0 {
        return found;
    }
    let mut world_space = e.call(GET_WORLD_SPACE, &args![reference]).u32();
    if world_space == 0 {
        let parent = e.call(REFERENCE_PARENT_008D6F30, &args![reference]).u32();
        if parent != 0 {
            let parent = e.call(REFERENCE_PARENT_008D6F30, &args![reference]).u32();
            if e.call(PARENT_CHECK_00425FD0, &args![parent]).u32() as u8 != 0 {
                world_space = e.call(REFERENCE_PARENT_008D6F30, &args![reference]).u32();
            }
        }
    }
    if world_space == 0 {
        return found;
    }
    e.with_stack(0x18, |e, locals| {
        let map = locals.addr();
        let alternative_at = locals.addr() + 0x10;
        let marked_at = locals.addr() + 0x14;
        e.call(VISITED_MAP_CONSTRUCT, &args![map, 0x25u32]);
        let mut tries = 0u32;
        let list = e.call(TELEPORT_LIST, &args![this]).u32();
        let count = e.call(LIST_COUNT, &args![list]).u32();
        let mut index = 0xffff_ffffu32;
        let mut _choice = 0xffff_ffffu32;
        let player = e.global::<u32>(PLAYER_REFERENCE);
        let preferred = u32::from(e.call(PLAYER_PREFERRED_DOOR, &args![player, this]).u32() as u8);
        while tries < count && found == 0 {
            tries += 1;
            index = e.call(RANDOM, &[]).u32() % count;
            e.mem.set_u8(marked_at, 0);
            while e
                .call(VISITED_MAP_LOOKUP, &args![map, index, marked_at])
                .u32() as u8
                != 0
                && e.mem.u8(marked_at) != 0
            {
                index = e.call(RANDOM, &[]).u32() % count;
                e.mem.set_u8(marked_at, 0);
            }
            let mut item = 0u32;
            let mut position = 0u32;
            let mut node = e.call(TELEPORT_LIST, &args![this]).u32();
            while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
                if position == index {
                    let at = e.call(NODE_ITEM, &args![node]).u32();
                    item = e.mem.u32(at);
                    break;
                }
                position += 1;
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
            found = e
                .call(FIND_DESTINATION, &args![item, world_space, alternative_at])
                .u32();
            if e.mem.u32(alternative_at) != 0 && _choice == 0xffff_ffff {
                _choice = index;
            }
            if found != 0 && index == preferred {
                e.mem.set_u32(alternative_at, found);
                _choice = index;
                found = 0;
            }
            e.call(VISITED_MAP_INSERT, &args![map, position, 1u32]);
        }
        if index != 0xffff_ffff {
            let player = e.global::<u32>(PLAYER_REFERENCE);
            e.call(PLAYER_SET_DOOR_CHOICE, &args![player, this, index & 0xff]);
        }
        fn_00519bf0(e, Ptr::new(map));
    });
    found
}

/// One side of `00519810`: the teleport data `data` leads to `target`
/// (`00537e90`); its position and rotation are the target's marker's when
/// it has one, else the target's position moved along its facing
/// (`GetDeltaForAngle` with the angle at `0102b3c8`, scaled by
/// `load_distance` when the target's door `target_door` has flag 0x02,
/// else by `distance`) and its rotation with the angle added to its third
/// component.
fn place_teleport(
    e: &mut Engine,
    data: u32,
    target: u32,
    target_door: u32,
    (distance, load_distance, angle): (u32, u32, u32),
) {
    e.call(SET_LINKED_DOOR, &args![data, target]);
    let marker = e.call(TELEPORT_MARKER, &args![target]).u32();
    if marker != 0 {
        let position = e.vcall(marker, 0x1f4, &[]).u32();
        e.call(SET_TELEPORT_POSITION, &args![data, position]);
        let rotation = e.call(REFERENCE_ROTATION, &args![marker]).u32();
        e.call(SET_TELEPORT_ROTATION, &args![data, rotation]);
        return;
    }
    e.with_stack(0x24, |e, vectors| {
        let delta = vectors.addr();
        let sum = vectors.addr() + 0xc;
        let rotation = vectors.addr() + 0x18;
        e.call(DELTA_FOR_ANGLE, &args![target, delta, angle]);
        let scale = if fn_00517fa0(e, Ptr::new(target_door)) {
            load_distance
        } else {
            distance
        };
        e.call(VECTOR_SCALE, &args![delta, scale]);
        let position = e.vcall(target, 0x1f4, &[]).u32();
        let moved = e.call(VECTOR_ADD, &args![position, sum, delta]).u32();
        e.call(SET_TELEPORT_POSITION, &args![data, moved]);
        let facing = e.call(REFERENCE_ROTATION, &args![target]).u32();
        let z = x87_store32(x87_add(x87_load32(e.mem.u32(facing + 8)), x87_m32(angle)));
        let set = e.call(VECTOR_SET, &args![rotation, 0u32, 0u32, z]).u32();
        e.call(SET_TELEPORT_ROTATION, &args![data, set]);
    });
}

// Translated from 00519810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Links two door references (engine map: no name; cdecl, two words):
/// nothing when either is null, they are the same, either's base object is
/// no `TESObjectDOOR`, either already has teleport data (`00568e50`), or
/// either's new teleport data (`TESObjectREFR::AddTeleport`) is null.
/// Otherwise reads the distance setting (`011c9398`) and the load distance
/// (`00404010` of it and the setting at `011c93c4` plus 100.0), and makes
/// each side lead to the other (`place_teleport`; the first's data to the
/// second door first).
pub fn fn_00519810(e: &mut Engine, first: u32, second: u32) {
    if first == 0 || second == 0 || first == second {
        return;
    }
    let base = e.call(BASE_OBJECT, &args![first]).u32();
    let first_door = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                base,
                0u32,
                TYPE_TES_BOUND_OBJECT,
                TYPE_TES_OBJECT_DOOR,
                0u32
            ],
        )
        .u32();
    let base = e.call(BASE_OBJECT, &args![second]).u32();
    let second_door = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                base,
                0u32,
                TYPE_TES_BOUND_OBJECT,
                TYPE_TES_OBJECT_DOOR,
                0u32
            ],
        )
        .u32();
    if first_door == 0 || second_door == 0 {
        return;
    }
    if e.call(TELEPORT_DATA, &args![first]).u32() != 0
        || e.call(TELEPORT_DATA, &args![second]).u32() != 0
    {
        return;
    }
    let first_data = e.call(ADD_TELEPORT, &args![first]).u32();
    let second_data = e.call(ADD_TELEPORT, &args![second]).u32();
    if first_data == 0 || second_data == 0 {
        return;
    }
    let setting = e
        .call(SETTING_FLOAT_ADDRESS, &args![DOOR_DISTANCE_SETTING])
        .u32();
    let distance = x87_quiet_bits(e.mem.u32(setting));
    let setting = e
        .call(SETTING_FLOAT_ADDRESS, &args![LOAD_DOOR_DISTANCE_SETTING])
        .u32();
    let widened = x87_store32(x87_add(
        x87_load32(e.mem.u32(setting)),
        e.global::<f64>(DISTANCE_EXTRA),
    ));
    let pair = e
        .call(
            FLOAT_PAIR_00404010,
            &args![distance, x87_quiet_bits(widened)],
        )
        .f64();
    let load_distance = x87_store32(pair);
    let angle = x87_quiet_bits(e.global::<u32>(DOOR_ANGLE));
    let floats = (distance, load_distance, angle);
    place_teleport(e, first_data, second, second_door, floats);
    place_teleport(e, second_data, first, first_door, floats);
}

// Translated from 00519b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<NiTPointerAllocator<unsigned int>,int,bool>` constructor
/// (engine map: no name; `RET 4`): its vtable (`0102b3f0`), `size` buckets
/// at +4, 0 items at +0xc, and `4 * size` bytes of buckets (`NiAlloc`, not
/// checked) at +8, cleared. Returns `this`.
pub fn fn_00519b80(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    let map = this.addr();
    e.mem.set_u32(map, MAP_BASE_VTABLE);
    e.mem.set_u32(map + 4, size);
    e.mem.set_u32(map + 0xc, 0);
    let bytes = e.mem.u32(map + 4) << 2;
    let buckets = e.call(NI_ALLOC, &args![bytes]).u32();
    e.mem.set_u32(map + 8, buckets);
    let bytes = e.mem.u32(map + 4) << 2;
    let buckets = e.mem.u32(map + 8);
    e.call(MEMSET, &args![buckets, 0u32, bytes]);
    this
}

// Translated from 00519bf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00519bf0 - the FS:[0] exception frame (it runs the base destructor if the clear throws) is not translated.
/// The `NiTPointerMap<int,bool>` destructor body (the scalar deleting
/// destructor `00519b50` runs it; no stack words): its vtable (`0102b3d0`),
/// the map emptied (`00438af0`), then the base destructor (`fn_00519c50`).
pub fn fn_00519bf0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), POINTER_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00519c50(e, this);
}

// Translated from 00519c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<NiTPointerAllocator<unsigned int>,int,bool>` destructor
/// body (the scalar deleting destructor `00519c80` runs it; no stack
/// words): its vtable (`0102b3f0`), the map emptied (`00438af0`), the
/// buckets freed (`NiFree`).
pub fn fn_00519c50(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(NI_FREE, &args![buckets]);
}

// Translated from 00759030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00759030 - the FS:[0] exception frame (it destroys the sound handle if Play throws) is not translated.
/// `TESObjectDOOR::DoorTeleportPlayerArrivalCallback` (engine map; `RET 4`,
/// `this` not read): plays the sound of `source` (`005e9100`) by its form id
/// (`BSAudio::GetSoundHandleByNumericID` with flags 0x101 into a stack
/// handle, `BSSoundHandle::Play` with 0) and destroys the handle
/// (`00483710`). The handle the game keeps on its stack is a block here.
pub fn tes_object_door_door_teleport_player_arrival_callback(
    e: &mut Engine,
    _this: Ptr,
    source: u32,
) {
    let sound = e.call(ARRIVAL_SOUND, &args![source]).u32();
    let id = e.call(FORM_ID, &args![sound]).u32();
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(AUDIO, &[]).u32();
        let filled = e
            .call(SOUND_BY_ID, &args![audio, handle, id, 0x101u32])
            .u32();
        e.call(SOUND_PLAY, &args![filled, 0u32]);
        e.call(SOUND_HANDLE_DESTROY, &args![handle]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_77f0, tesobjectdoor_load(Ptr, Ptr) -> bool),
        entry!(0x0051_9b50, ni_t_pointer_map_int_bool_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0051_9c80, ni_t_map_base_ni_t_pointer_allocator_unsigned_int_int_bool_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0051_7fa0, fn_00517fa0(Ptr) -> bool),
        entry!(0x0051_8000, tes_object_door_is_minimal_use(Ptr) -> bool),
        entry!(0x0051_8080, tes_object_door_is_sliding_door(Ptr) -> bool),
        entry!(0x0051_76b0, tes_object_door_init_item(Ptr)),
        entry!(0x0051_8e60, tes_object_door_can_actor_ignore_lock(Ptr, Ptr, Ptr, u8, u8) -> bool),
        entry!(0x0051_8f00, tes_object_door_can_actor_ignore_lock_ov2(Ptr, Ptr, u8, u8) -> bool),
        entry!(0x0051_9290, fn_00519290()),
        entry!(0x0051_8cf0, fn_00518cf0(Ptr)),
        entry!(0x0051_8b70, fn_00518b70(Ptr, Ptr)),
        entry!(0x0051_80b0, fn_005180b0(Ptr, Ptr, Ptr, u32, u32, u32) -> bool),
        entry!(0x0051_7c30, fn_00517c30(Ptr)),
        entry!(0x0051_7d70, fn_00517d70(Ptr, u32)),
        entry!(0x0051_7e70, fn_00517e70(Ptr, u32) -> bool),
        entry!(0x0051_7fd0, fn_00517fd0(Ptr) -> bool),
        entry!(0x0051_8030, fn_00518030(Ptr, u8)),
        entry!(0x0051_95e0, fn_005195e0(Ptr, u32, u32) -> u32),
        entry!(0x0051_9810, fn_00519810(u32, u32)),
        entry!(0x0051_9b80, fn_00519b80(Ptr, u32) -> Ptr),
        entry!(0x0051_9bf0, fn_00519bf0(Ptr)),
        entry!(0x0051_9c50, fn_00519c50(Ptr)),
        entry!(
            0x0075_9030,
            tes_object_door_door_teleport_player_arrival_callback(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::fallout_shared::tesmodel::load_test::{
        self, calls_to, run, stub, tag, CHUNK_SIZE, CHUNK_VALUE, FILE,
    };

    type Loaded = (Engine, u32, bool, load_test::Calls);

    /// Runs the loader on a door form and a file holding `chunks` of a record of `form_type`.
    /// The form's virtual slots `+0xe0` and `+0x134` are the doubles `0x0090_0500` and
    /// `0x0090_0600`. The lookup `004839c0` answers the id xor `0x1000_0000`, the compile
    /// index goes into the id's top byte and the cast keeps the lookup's answer.
    fn load(form_type: u32, chunks: &[u32]) -> Loaded {
        let mut e = load_test::engine(form_type, chunks, false);
        stub(
            &mut e,
            &[
                0x0048_5110,
                0x0048_4ab0,
                0x0048_7050,
                0x0048_92d0,
                0x0048_a7b0,
                0x006e_cd40,
                0x0048_cd90,
                0x0047_81e0,
            ],
        );
        // `005ae3d0` is given the address of a copy of the word: it keeps the word it finds there.
        e.register(0x005a_e3d0, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| Ret {
            eax: a[0] ^ 0x1000_0000,
            ..Ret::default()
        });
        e.register(0x00ec_43fb, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this = load_test::object(
            &mut e,
            0x90,
            0x0900_0000,
            &[(0xe0, 0x0090_0500), (0x134, 0x0090_0600)],
        );
        let (ret, calls) = run(&mut e, 0x0051_77f0, &[this, FILE]);
        (e, this, ret.bool(), calls)
    }

    fn door(chunks: &[u32]) -> Loaded {
        load(0x1c, chunks)
    }

    #[test]
    fn another_form_type_is_refused_before_anything_is_loaded() {
        let (_, _, answer, calls) = load(0x1b, &[tag(b"EDID")]);
        assert!(!answer);
        assert!(calls.is_empty());
    }

    #[test]
    fn the_form_is_loaded_first_and_the_flag_bit_cleared() {
        let (_, this, answer, calls) = door(&[]);
        assert!(answer);
        assert_eq!(
            calls,
            [
                (0x0048_5110, vec![this, FILE]),
                (0x0048_4ab0, vec![this, 0])
            ]
        );
    }

    #[test]
    fn the_buffer_and_virtual_chunks() {
        let (_, this, _, calls) = door(&[tag(b"EDID")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, 0x200));
        assert_eq!(calls_to(&calls, 0x0090_0600), [vec![this, read[0][1]]]);

        let (_, this, _, calls) = door(&[tag(b"OBND")]);
        assert_eq!(calls_to(&calls, 0x0090_0500), [vec![this, FILE]]);

        let (_, this, _, calls) = door(&[tag(b"MODS")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, CHUNK_SIZE));
        assert_eq!(
            calls_to(&calls, 0x0048_a7b0),
            [vec![this + 0x3c, read[0][1], CHUNK_SIZE, 0x1234]]
        );
    }

    #[test]
    fn the_components_get_their_chunks() {
        for (chunk, callee, offset) in [
            (tag(b"FULL"), 0x0048_7050, 0x30),
            (tag(b"MODL"), 0x0048_92d0, 0x3c),
            (tag(b"MODD"), 0x0048_92d0, 0x3c),
            (tag(b"MOSD"), 0x0048_92d0, 0x3c),
            (tag(b"MODT"), 0x0048_92d0, 0x3c),
            (tag(b"DSTD"), 0x0047_81e0, 0x68),
            (tag(b"DEST"), 0x0047_81e0, 0x68),
        ] {
            let (_, this, _, calls) = door(&[chunk]);
            assert_eq!(
                calls_to(&calls, callee),
                [vec![this + offset, FILE]],
                "{chunk:08x}"
            );
        }
    }

    #[test]
    fn the_script_chunk_sets_the_script_and_initialises_it() {
        let (_, this, _, calls) = door(&[tag(b"SCRI")]);
        assert_eq!(
            calls_to(&calls, 0x006e_cd40),
            [vec![this + 0x5c, CHUNK_VALUE]]
        );
        assert_eq!(calls_to(&calls, 0x0048_cd90), [vec![this + 0x5c, this]]);
    }

    #[test]
    fn the_sound_chunks_store_the_cast_form() {
        for (chunk, offset) in [
            (tag(b"SNAM"), 0x78),
            (tag(b"ANAM"), 0x7c),
            (tag(b"BNAM"), 0x80),
        ] {
            let (e, this, _, calls) = door(&[chunk]);
            // 0x12345678 with the compile index, then the lookup's xor.
            let id = CHUNK_VALUE | 0x0100_0000;
            assert_eq!(e.mem.u32(this + offset), id ^ 0x1000_0000);
            assert_eq!(calls_to(&calls, 0x0048_39c0), [vec![id]]);
            assert_eq!(
                calls_to(&calls, 0x00ec_43fb),
                [vec![id ^ 0x1000_0000, 0, 0x0118_3028, 0x0118_32fc, 0]]
            );
            let index = calls_to(&calls, 0x0048_5d50);
            assert_eq!((index.len(), index[0][1]), (1, FILE));
        }
    }

    #[test]
    fn the_flag_and_trigger_chunks() {
        let (_, this, _, calls) = door(&[tag(b"FNAM")]);
        assert_eq!(calls_to(&calls, 0x0047_2890), [vec![FILE, this + 0x84, 1]]);

        let (e, this, _, calls) = door(&[tag(b"TNAM")]);
        let call = calls_to(&calls, 0x005a_e3d0);
        assert_eq!(call.len(), 1);
        assert_eq!(call[0][0], this + 0x88);
        assert_eq!(e.mem.u32(this + 0x88), CHUNK_VALUE);
    }

    #[test]
    fn unknown_chunks_are_skipped() {
        let (_, _, answer, calls) = door(&[tag(b"XXXX"), tag(b"YYYY")]);
        assert!(answer);
        assert_eq!(calls.len(), 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_00519b50_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_9bf0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_9b50, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_9b50)
                .collect();
            let mut expected = vec![(0x0051_9bf0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00519c80_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_9c50, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_9c80, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_9c80)
                .collect();
            let mut expected = vec![(0x0051_9c50, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod door_tests {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn door_flag_getters_test_their_bits() {
        let mut e = Engine::new();
        let door = e.mem.alloc(0x88);
        for (flags, two, eight, sixteen) in [
            (0u8, false, false, false),
            (0x02, true, false, false),
            (0x08, false, true, false),
            (0x10, false, false, true),
            (0xe5, false, false, false),
            (0xff, true, true, true),
        ] {
            e.mem.set_u8(door + 0x84, flags);
            assert_eq!(e.call(0x0051_7fa0, &args![door]).bool(), two);
            assert_eq!(e.call(0x0051_8000, &args![door]).bool(), eight);
            assert_eq!(e.call(0x0051_8080, &args![door]).bool(), sixteen);
        }
    }

    /// A door for `InitItem` whose list nodes hold the given IDs: node `i` is an 8-byte block
    /// (item word, next node); `008256d0` says a node is empty when its item is 0. IDs below
    /// 0x2000 are found.
    fn door_with_list(e: &mut Engine, ids: &[u32]) -> (u32, Vec<u32>) {
        let door = e.mem.alloc(0x90);
        e.put_vtable(0x0300_0000, &[0; 0x50]);
        e.mem.set_u32(0x0300_0000 + 0x130, 0x0300_1130);
        e.mem.set_u32(door, 0x0300_0000);
        let mut nodes = vec![];
        for _ in ids {
            nodes.push(e.mem.alloc(8));
        }
        for (index, id) in ids.iter().enumerate() {
            e.mem.set_u32(nodes[index], *id);
            let next = nodes.get(index + 1).copied().unwrap_or(0);
            e.mem.set_u32(nodes[index] + 4, next);
        }
        let first = nodes.first().copied().unwrap_or(0);
        e.register_double(0x0087_1a30, move |_, _| ret(first));
        e.register(0x0300_1130, |_, _| ret(0x7777));
        e.register(0x0040_13e0, |_, _| ret(0));
        e.register(0x0082_56d0, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x0072_6070, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0048_4e60, |_, _| ret(0x9000));
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 1);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| {
            ret(if a[0] < 0x2000 { a[0] + 0x100 } else { 0 })
        });
        e.register(0x0072_6c60, |e, a| {
            let form = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], form);
            Ret::default()
        });
        // Removing after a previous node unlinks the next one.
        e.register(0x0090_5330, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        // Removing the head moves the next node's contents into it.
        e.register(0x0063_f7b0, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        e.register(0x0084_e3a0, |_, _| ret(0xbeef));
        for address in [
            0x0048_cd90u32,
            0x0047_8be0,
            0x0048_aa80,
            0x005b_5e40,
            0x0048_4ab0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        (door, nodes)
    }

    #[test]
    fn door_init_item_resolves_and_drops_list_entries() {
        let mut e = Engine::new();
        let (door, nodes) = door_with_list(&mut e, &[0x3000, 0x10, 0x3100, 0x20]);
        e.call_log = Some(vec![]);
        e.call(0x0051_76b0, &args![door]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[2..5],
            [
                (0x0048_cd90, vec![door + 0x5c, door]),
                (0x0047_8be0, vec![door + 0x68, door]),
                (0x0048_aa80, vec![door + 0x3c, door]),
            ]
        );
        // The head (missing) was replaced by the second entry, then the third (missing)
        // was unlinked after it.
        assert_eq!(e.mem.u32(nodes[0]), 0x111);
        assert_eq!(e.mem.u32(nodes[0] + 4), nodes[3]);
        assert_eq!(e.mem.u32(nodes[3]), 0x121);
        assert_eq!(calls_to(&log, 0x0063_f7b0), vec![vec![nodes[0]]]);
        let removed = calls_to(&log, 0x0090_5330);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], nodes[0]);
        assert_eq!(
            calls_to(&log, 0x005b_5e40),
            vec![
                vec![MISSING_TELEPORT_FORM, 0x3001, 0x7777, 0xbeef],
                vec![MISSING_TELEPORT_FORM, 0x3101, 0x7777, 0xbeef],
            ]
        );
        assert_eq!(log.last().unwrap(), &(0x0048_4ab0, vec![door, 1]));
    }

    #[test]
    fn door_init_item_stops_when_checked() {
        let mut e = Engine::new();
        let (door, _) = door_with_list(&mut e, &[]);
        e.register(0x0040_13e0, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0051_76b0, &args![door]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    /// Doubles for `CanActorIgnoreLock`: the actor follows (`following`), its package
    /// target is `target`, the reference's word is `linked`, its slot +0x448 answers
    /// `ignores`, and the door lock check answers `lock_answer`.
    fn ignore_case(
        e: &mut Engine,
        following: bool,
        target: u32,
        linked: u32,
        ignores: bool,
        lock_answer: bool,
    ) -> u32 {
        e.map(PLAYER_REFERENCE, 4);
        e.set_global(PLAYER_REFERENCE, 0x0007_0000u32);
        let actor = e.mem.alloc(8);
        let table = e.mem.alloc(0x450);
        e.mem.set_u32(table + 0x448, 0x0300_1448);
        e.mem.set_u32(actor, table);
        let acquire = e.mem.alloc(8);
        let acquire_table = e.mem.alloc(0x130);
        e.mem.set_u32(acquire_table + 0x128, 0x0300_1128);
        e.mem.set_u32(acquire, acquire_table);
        e.register_double(0x0088_42c0, move |_, _| ret(u32::from(following)));
        e.register_double(0x008d_8520, move |_, _| ret(acquire));
        e.register_double(0x0300_1128, move |_, _| ret(target));
        e.register_double(0x0056_8e50, move |_, _| ret(linked));
        e.register_double(0x0300_1448, move |_, _| ret(u32::from(ignores)));
        e.register_double(0x0050_1fd0, move |_, _| ret(u32::from(lock_answer)));
        actor
    }

    #[test]
    fn can_actor_ignore_lock_follows_its_branches() {
        let player = 0x0007_0000u32;
        // (following, target, linked, ignores, lock check, answer, lock check asked)
        for (following, target, linked, ignores, lock_answer, answer, asked) in [
            (true, player, 1u32, false, false, true, false),
            (true, player, 0, false, false, false, true),
            (true, 0x1234, 1, false, true, true, true),
            (false, player, 1, true, false, true, false),
            (false, player, 1, false, true, true, true),
            (false, player, 1, false, false, false, true),
        ] {
            let mut e = Engine::new();
            let actor = ignore_case(&mut e, following, target, linked, ignores, lock_answer);
            e.call_log = Some(vec![]);
            let result = e
                .call(
                    0x0051_8e60,
                    &args![0x6000u32, actor, 0x5000u32, 0x0101u32, 0xff00u32],
                )
                .bool();
            assert_eq!(result, answer);
            let log = e.call_log.take().unwrap();
            let checks = calls_to(&log, 0x0050_1fd0);
            if asked {
                // The flag bytes are the low bytes of their words.
                assert_eq!(checks, vec![vec![0x6000, actor, 1, 0]]);
            } else {
                assert!(checks.is_empty());
            }
        }
        let mut e = Engine::new();
        let actor = ignore_case(&mut e, true, player, 1, true, true);
        assert!(!e
            .call(0x0051_8e60, &args![0u32, actor, 0x5000u32, 1u32, 1u32])
            .bool());
        assert!(!e
            .call(0x0051_8e60, &args![0x6000u32, 0u32, 0x5000u32, 1u32, 1u32])
            .bool());
    }

    #[test]
    fn can_actor_ignore_lock_ov2_builds_a_door_lock_for_the_reference() {
        let mut e = Engine::new();
        let actor = ignore_case(&mut e, false, 0, 0, false, true);
        e.register(0x0050_1d20, |_, _| Ret::default());
        e.register(0x0050_1e50, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0051_8f00, &args![0x5000u32, actor, 1u32, 0u32])
            .bool());
        let log = e.call_log.take().unwrap();
        let built = calls_to(&log, 0x0050_1d20);
        let lock = built[0][0];
        assert_eq!(built[0][1], 0x5000);
        assert_eq!(calls_to(&log, 0x0050_1fd0), vec![vec![lock, actor, 1, 0]]);
        assert_eq!(calls_to(&log, 0x0050_1e50), vec![vec![lock]]);
    }

    /// A singly linked list of 8-byte nodes (item word, next) holding `items`.
    fn node_list(e: &mut Engine, items: &[u32]) -> u32 {
        let first = e.mem.alloc(8);
        let mut node = first;
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            let next = if index + 1 < items.len() {
                e.mem.alloc(8)
            } else {
                0
            };
            e.mem.set_u32(node + 4, next);
            node = next;
        }
        first
    }

    #[test]
    fn fn_00519290_sorts_the_followers() {
        let mut e = Engine::new();
        e.map(PLAYER_REFERENCE, 4);
        e.set_global(PLAYER_REFERENCE, 0x0007_0000u32);
        e.map(ONE, 8);
        e.set_global(ONE, 1.0f64);
        // Followers: two actors; the first could be following the player.
        let follower_list = node_list(&mut e, &[0x0007_1000, 0x0007_2000]);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data + 0xc, follower_list);
        let staying = node_list(&mut e, &[]);
        let leaving = node_list(&mut e, &[]);
        let lists = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let made = lists.clone();
        e.register(0x005d_43c0, |_, _| ret(0x0008_0000));
        e.register_double(0x0042_2700, move |_, _| ret(data));
        e.register(0x0040_1000, |_, _| ret(0x0009_0000));
        e.register_double(0x0096_a2d0, move |_, _| {
            made.set(made.get() + 1);
            ret(if made.get() == 1 { leaving } else { staying })
        });
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x0072_6070, |e, a| ret(e.mem.u32(a[0] + 4)));
        // Appending puts the item's actor into the staying list (one slot per call).
        let appended = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let count = appended.clone();
        e.register_double(0x0090_5820, move |e, a| {
            let actor = e.mem.u32(a[1]);
            let mut node = a[0];
            for _ in 0..count.get() {
                let next = e.mem.u32(node + 4);
                let next = if next == 0 {
                    let fresh = e.mem.alloc(8);
                    e.mem.set_u32(node + 4, fresh);
                    fresh
                } else {
                    next
                };
                node = next;
            }
            e.mem.set_u32(node, actor);
            count.set(count.get() + 1);
            Ret::default()
        });
        e.register(0x008b_c860, |_, a| ret(u32::from(a[0] == 0x0007_1000)));
        e.register(0x005a_e3d0, |e, a| {
            let actor = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], actor);
            Ret::default()
        });
        e.register(0x0086_7da0, |_, _| Ret {
            st0: 9.5,
            ..Ret::default()
        });
        e.register(0x008d_8520, |_, a| ret(a[0] + 0x10));
        e.register(0x0082_56d0, |_, _| ret(1));
        for address in [
            0x008b_3ad0u32,
            0x0069_3d50,
            0x0042_2690,
            0x0047_0470,
            0x0047_02f0,
            0x0042_2720,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0051_9290, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(lists.get(), 2);
        assert_eq!(calls_to(&log, 0x0090_5820).len(), 2);
        assert_eq!(calls_to(&log, 0x008b_3ad0), vec![vec![0x0007_1000]]);
        assert_eq!(calls_to(&log, 0x0086_7da0), vec![vec![0x011d_e7b8]]);
        assert_eq!(
            calls_to(&log, 0x0069_3d50),
            vec![vec![0x0007_1010, 8.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, 0x0042_2690),
            vec![vec![0x0008_0000, 0x0007_2000]]
        );
        assert_eq!(calls_to(&log, 0x0047_0470), vec![vec![leaving]]);
        assert_eq!(calls_to(&log, 0x0047_02f0), vec![vec![leaving, 1]]);
        assert_eq!(calls_to(&log, 0x0042_2720), vec![vec![0x0008_0000]]);
    }

    #[test]
    fn fn_00519290_needs_follower_data() {
        let mut e = Engine::new();
        e.map(PLAYER_REFERENCE, 4);
        e.register(0x005d_43c0, |_, _| ret(0x0008_0000));
        e.register(0x0042_2700, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0051_9290, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
    }

    /// Doubles for `00518cf0`: the player's object is `object` (`00425fd0` answers `ready`),
    /// the door has `sound`, the linked reference answers `linked` through slot +0x1d0, the
    /// setting byte is `flag`.
    fn callback_case(e: &mut Engine, object: u32, ready: bool, sound: u32, linked: u32, flag: u8) {
        e.map(PLAYER_REFERENCE, 4);
        e.set_global(PLAYER_REFERENCE, 0x0007_0000u32);
        e.map(0x011d_e134, 4);
        e.set_global(0x011d_e134, 0x0009_9000u32);
        let flag_byte = e.mem.alloc(4);
        e.mem.set_u8(flag_byte, flag);
        let linked_ref = e.mem.alloc(8);
        let table = e.mem.alloc(0x1d4);
        e.mem.set_u32(table + 0x1d0, 0x0300_11d0);
        e.mem.set_u32(linked_ref, table);
        // Followers: no data, so 00519290 stops at once.
        e.register(0x005d_43c0, |_, _| ret(0x0008_0000));
        e.register(0x0042_2700, |_, _| ret(0));
        e.register_double(0x008d_6f30, move |_, _| ret(object));
        e.register_double(0x0042_5fd0, move |_, _| ret(u32::from(ready)));
        e.register(0x007a_f430, |_, _| ret(0x0006_0000));
        e.register_double(0x0040_7840, move |_, _| ret(sound));
        e.register(0x0056_8e50, |_, _| ret(0x0006_1000));
        e.register_double(0x0055_9450, move |_, _| ret(linked_ref));
        e.register_double(0x0300_11d0, move |_, _| ret(linked));
        e.register(0x0084_e3a0, |_, a| ret(a[0] + 2));
        e.register(AUDIO, |_, _| ret(0x0009_0000));
        e.register_double(0x0040_8d60, move |_, _| ret(flag_byte));
        for address in [
            0x0054_af80u32,
            SOUND_BY_ID,
            SOUND_PLAY,
            SOUND_HANDLE_DESTROY,
            0x0055_7be0,
            0x0085_1d30,
        ] {
            e.register(address, |_, _| Ret::default());
        }
    }

    #[test]
    fn fn_00518cf0_plays_the_door_sound_and_updates_the_player() {
        let mut e = Engine::new();
        callback_case(&mut e, 0x0009_8000, true, 0x0006_2000, 1, 1);
        e.call_log = Some(vec![]);
        e.call(0x0051_8cf0, &args![0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0054_af80), vec![vec![0x0009_8000]]);
        let sound = calls_to(&log, SOUND_BY_ID);
        assert_eq!(sound[0][0], 0x0009_0000);
        assert_eq!(sound[0][2..], [0x0006_2002, 0x121]);
        assert_eq!(calls_to(&log, SOUND_PLAY), vec![vec![sound[0][1], 0]]);
        // The object is ready, so 00557be0 does not run; the flag byte is set.
        assert!(calls_to(&log, 0x0055_7be0).is_empty());
        assert_eq!(calls_to(&log, 0x0040_8d60), vec![vec![0x011d_8980]]);
        assert_eq!(calls_to(&log, 0x0085_1d30), vec![vec![0x0009_9000]]);
    }

    #[test]
    fn fn_00518cf0_without_a_reference_only_updates_the_player() {
        for (object, ready, flag, updated, notified) in [
            (0x0009_8000u32, false, 1u8, true, true),
            (0, false, 1, false, false),
            (0x0009_8000, false, 0, true, false),
        ] {
            let mut e = Engine::new();
            callback_case(&mut e, object, ready, 0x0006_2000, 1, flag);
            e.call_log = Some(vec![]);
            e.call(0x0051_8cf0, &args![0u32]);
            let log = e.call_log.take().unwrap();
            assert!(calls_to(&log, 0x0042_2700).is_empty());
            assert!(calls_to(&log, SOUND_BY_ID).is_empty());
            assert_eq!(calls_to(&log, 0x0055_7be0).len(), usize::from(updated));
            assert_eq!(calls_to(&log, 0x0085_1d30).len(), usize::from(notified));
        }
    }

    #[test]
    fn fn_00518cf0_skips_the_sound_without_one_or_a_listening_reference() {
        for (sound, linked) in [(0u32, 1u32), (0x0006_2000, 0)] {
            let mut e = Engine::new();
            callback_case(&mut e, 0x0009_8000, true, sound, linked, 0);
            e.call_log = Some(vec![]);
            e.call(0x0051_8cf0, &args![0x5000u32]);
            let log = e.call_log.take().unwrap();
            assert!(calls_to(&log, SOUND_BY_ID).is_empty());
        }
    }

    /// Doubles for `00518b70`: the reference's base form is `base`, its teleport data is
    /// `teleport`, the cell `cell` (`00425fd0` answers `cell_ready` for it and
    /// `player_ready` for the player's object).
    fn request_case(
        e: &mut Engine,
        base: u32,
        teleport: u32,
        cell: u32,
        cell_ready: bool,
        player_ready: bool,
    ) -> u32 {
        e.map(PLAYER_REFERENCE, 4);
        e.set_global(PLAYER_REFERENCE, 0x0007_0000u32);
        e.map(PLAYER_FLAG_011C9354, 4);
        e.set_global(PLAYER_FLAG_011C9354, 0xffu8);
        let request = e.mem.alloc(0x34);
        let position = e.mem.alloc(12);
        let rotation = e.mem.alloc(12);
        for index in 0..3u32 {
            e.mem.set_u32(position + 4 * index, 0x10 + index);
            e.mem.set_u32(rotation + 4 * index, 0x20 + index);
        }
        e.register_double(0x007a_f430, move |_, _| ret(base));
        e.register_double(0x0056_8e50, move |_, _| ret(teleport));
        e.register_double(0x0043_a2b0, move |_, _| ret(cell));
        e.register(0x0043_a320, |_, _| ret(0x0006_5000));
        e.register(0x0040_1000, |_, _| ret(0x0009_0000));
        e.register_double(0x0045_6540, move |_, _| ret(request));
        e.register_double(0x0071_7e50, move |_, _| ret(position));
        e.register_double(0x0046_0140, move |_, _| ret(rotation));
        e.register_double(0x0042_5fd0, move |_, a| {
            ret(u32::from(if a[0] == cell {
                cell_ready
            } else {
                player_ready
            }))
        });
        e.register(0x0093_be30, |_, _| Ret::default());
        e.register(0x008d_6f30, |_, _| ret(0x0009_8000));
        request
    }

    #[test]
    fn fn_00518b70_requests_the_player_position() {
        for (cell, cell_ready, destination_offset) in [
            (0x0006_4000u32, true, 4u32),
            (0x0006_4000, false, 0),
            (0, true, 0),
        ] {
            let mut e = Engine::new();
            let request = request_case(&mut e, 0x4000, 0x0006_1000, cell, cell_ready, true);
            e.call_log = Some(vec![]);
            e.call(0x0051_8b70, &args![0x4000u32, 0x5000u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls_to(&log, 0x0093_be30),
                vec![vec![0x0007_0000, request]]
            );
            assert_eq!(e.mem.u32(request + 8), 0x10);
            assert_eq!(e.mem.u32(request + 0x10), 0x12);
            assert_eq!(e.mem.u32(request + 0x14), 0x20);
            assert_eq!(e.mem.u32(request + 0x1c), 0x22);
            assert_eq!(e.mem.u32(request + 0x24), 0x0051_8cf0);
            assert_eq!(e.mem.u32(request + 0x28), 0x5000);
            if destination_offset == 4 {
                assert_eq!(e.mem.u32(request + 4), cell);
                assert_eq!(e.mem.u32(request), 0);
            } else {
                assert_eq!(e.mem.u32(request), 0x0006_5000);
                assert_eq!(e.mem.u32(request + 4), 0);
            }
            assert_eq!(e.global::<u8>(PLAYER_FLAG_011C9354), 1);
        }
    }

    #[test]
    fn fn_00518b70_needs_this_door_and_teleport_data() {
        for (reference, base, teleport) in [
            (0u32, 0x4000u32, 0x0006_1000u32),
            (0x5000, 0x4100, 0x0006_1000),
            (0x5000, 0x4000, 0),
        ] {
            let mut e = Engine::new();
            request_case(&mut e, base, teleport, 0, false, false);
            e.call_log = Some(vec![]);
            e.call(0x0051_8b70, &args![0x4000u32, reference]);
            let log = e.call_log.take().unwrap();
            assert!(calls_to(&log, 0x0093_be30).is_empty());
            assert_eq!(e.global::<u8>(PLAYER_FLAG_011C9354), 0xff);
        }
    }

    /// The answers the door activation's doubles give.
    #[derive(Clone, Copy)]
    struct Activation {
        actor: u32,
        busy: bool,
        teleport: u32,
        random: bool,
        destination: u32,
        teleport_after: u32,
        lock: u32,
        lock_active: bool,
        ignores: bool,
        key: u32,
        has_key: bool,
        has_used_item: bool,
        unlocking: u32,
        sleeping: bool,
        sound: u32,
        interior: bool,
        cell_loaded: bool,
    }

    const PLAYER_REF: u32 = 0x0007_0000;
    const NPC_REF: u32 = 0x0007_1000;
    const REFERENCE: u32 = 0x0005_0000;
    const DOOR: u32 = 0x0004_0000;

    fn activation() -> Activation {
        Activation {
            actor: PLAYER_REF,
            busy: false,
            teleport: 0,
            random: false,
            destination: 0,
            teleport_after: 0,
            lock: 0,
            lock_active: false,
            ignores: false,
            key: 0,
            has_key: false,
            has_used_item: false,
            unlocking: 0,
            sleeping: false,
            sound: 0,
            interior: false,
            cell_loaded: false,
        }
    }

    fn activate_door(case: Activation) -> (bool, Vec<(u32, Vec<u32>)>, Engine) {
        let mut e = Engine::new();
        e.map(PLAYER_REFERENCE, 4);
        e.set_global(PLAYER_REFERENCE, PLAYER_REF);
        e.map(USED_ITEM_FORM, 4);
        e.set_global(USED_ITEM_FORM, 0x0008_0000u32);
        e.map(UNLOCKING_DOOR, 8);
        e.set_global(UNLOCKING_DOOR, case.unlocking);
        e.map(MESSAGE_TIME, 4);
        e.set_global(MESSAGE_TIME, 2.0f32);
        e.map(FLOAT_MAX, 4);
        e.set_global(FLOAT_MAX, f32::MAX);
        e.map(0x011d_ea10, 4);
        e.set_global(0x011d_ea10, 0x0009_a000u32);
        e.map(DOOR, 0x100);
        // The actor (player or NPC) and the reference with their vtables.
        e.map(PLAYER_REF, 0x1000);
        e.map(NPC_REF, 0x1000);
        e.put_vtable(0x0300_0000, &[0; 0x90]);
        for (slot, target) in [
            (0x1e4u32, 0x0300_11e4u32),
            (0x214, 0x0300_1214),
            (0x218, 0x0300_1218),
            (0x448, 0x0300_1448),
        ] {
            e.mem.set_u32(0x0300_0000 + slot, target);
        }
        e.mem.set_u32(case.actor, 0x0300_0000);
        e.map(REFERENCE, 0x100);
        e.put_vtable(0x0300_0600, &[0; 0x80]);
        for (slot, target) in [(0x1d0u32, 0x0300_11d0u32), (0x1f4, 0x0300_11f4)] {
            e.mem.set_u32(0x0300_0600 + slot, target);
        }
        e.mem.set_u32(REFERENCE, 0x0300_0600);
        e.mem.set_u32(REFERENCE + 0x40, 1.0f32.to_bits());
        e.mem.set_u32(REFERENCE + 0x44, 2.0f32.to_bits());
        e.mem.set_u32(REFERENCE + 0x48, 3.0f32.to_bits());
        // The acquire object.
        e.map(0x0009_3000, 4);
        e.put_vtable(0x0300_0c00, &[0; 0x190]);
        for (slot, target) in [
            (0x44u32, 0x0300_1c44u32),
            (0x128, 0x0300_1d28),
            (0x134, 0x0300_1d34),
            (0x4c8, 0x0300_20c8),
            (0x610, 0x0300_2210),
        ] {
            e.mem.set_u32(0x0300_0c00 + slot, target);
        }
        e.mem.set_u32(0x0009_3000, 0x0300_0c00);
        if case.lock != 0 {
            e.map(case.lock, 0x10);
            e.mem.set_u32(case.lock + 4, case.key);
        }
        e.map(0x0006_2000, 0x10);
        e.mem.set_u32(0x0006_2000 + 0xc, 0x0006_8000);
        e.map(0x0006_3000, 0x10);
        let teleport_calls = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let seen = teleport_calls.clone();
        e.register_double(0x0056_8e50, move |_, _| {
            seen.set(seen.get() + 1);
            ret(if seen.get() == 1 {
                case.teleport
            } else {
                case.teleport_after
            })
        });
        e.register(0x00ec_43fb, |_, a| ret(a[0]));
        e.register_double(0x0047_7ba0, move |_, _| ret(u32::from(case.busy)));
        e.register_double(0x0056_9160, move |_, _| ret(case.lock));
        e.register_double(0x0051_94d0, move |_, _| ret(u32::from(case.random)));
        e.register_double(0x0051_95e0, move |_, _| ret(case.destination));
        e.register(0x008d_8520, |_, _| ret(0x0009_3000));
        e.register(0x0300_20c8, |_, _| ret(0));
        e.register(0x0300_1214, |_, _| ret(0));
        // CanActorIgnoreLock: only slot +0x448 decides here.
        e.register(0x0050_1d20, |_, _| Ret::default());
        e.register(0x0050_1e50, |_, _| Ret::default());
        e.register(0x0088_42c0, |_, _| ret(0));
        e.register_double(0x0300_1448, move |_, _| ret(u32::from(case.ignores)));
        e.register(0x0050_1fd0, |_, _| ret(0));
        e.register_double(0x0050_21a0, move |_, _| ret(u32::from(case.lock_active)));
        e.register(0x0300_1218, |_, _| ret(1));
        e.register_double(0x0089_1db0, move |_, a| {
            ret(u32::from(if a[1] == 0x0008_0000 {
                case.has_used_item
            } else {
                a[1] == case.key && case.has_key
            }))
        });
        e.register(0x0056_91f0, |_, _| ret(0x0006_2000));
        e.register(0x0043_09e0, |_, _| ret(2));
        e.register(0x0043_0ae0, |_, _| ret(0));
        e.register(0x0300_11e4, |_, _| ret(0));
        e.register(AUDIO, |_, _| ret(0x0009_0000));
        e.register(0x00ad_7550, |_, _| ret(0x0009_1000));
        e.register(0x0040_8da0, |_, _| ret(0x0009_2000));
        e.register(0x0040_3df0, |_, a| ret(a[0] + 1));
        e.register_double(0x0094_df60, move |_, _| ret(u32::from(case.sleeping)));
        e.register(0x0045_cd60, |_, _| ret(0));
        e.register(0x0300_2210, |_, _| ret(0));
        e.register_double(0x0040_7820, move |_, _| ret(case.sound));
        e.register(0x0300_11d0, |_, _| ret(1));
        e.register(0x0300_11f4, |_, a| ret(a[0] + 0x40));
        e.register(0x0084_e3a0, |_, a| ret(a[0] + 2));
        e.register(SOUND_BY_ID, |_, a| ret(a[1]));
        e.register(0x005b_b4d0, |_, _| ret(0));
        e.register(0x0043_a2b0, |_, _| ret(0x0006_4000));
        e.register(0x0043_a320, |_, _| ret(0x0006_5000));
        e.register_double(0x0057_5d10, move |_, _| ret(u32::from(case.interior)));
        e.register(0x0071_7e50, |_, _| ret(0x0006_3000));
        e.register(0x0046_0140, |_, _| ret(0x0006_3000));
        e.register_double(0x0045_11e0, move |_, _| ret(u32::from(case.cell_loaded)));
        // 00518b70: the reference's base form is another door, so it stops at once.
        e.register(0x007a_f430, |_, _| ret(0x0004_1000));
        for address in [
            0x0070_52f0u32,
            0x0051_9810,
            0x0051_7360,
            0x0057_89b0,
            0x005e_58f0,
            0x0070_5830,
            0x0070_6f30,
            0x0049_8910,
            0x0060_0900,
            0x0060_0940,
            0x0060_0920,
            0x0300_1c44,
            SOUND_PLAY,
            SOUND_HANDLE_DESTROY,
            0x0040_6d00,
            0x0057_3170,
            0x0047_a560,
            0x0041_a250,
            0x0041_8900,
            SOUND_SET_POSITION,
            0x00ad_7740,
            0x008f_e960,
            0x0300_1d34,
            0x0096_2880,
            0x0057_5830,
            0x0057_57d0,
            0x0057_5770,
            0x008a_e640,
            0x0057_3800,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let answer = e
            .call(
                0x0051_80b0,
                &args![DOOR, REFERENCE, case.actor, 0u32, 0u32, 0u32],
            )
            .bool();
        let log = e.call_log.take().unwrap();
        (answer, log, e)
    }

    #[test]
    fn door_activation_refuses_without_a_reference_or_when_busy() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert!(!e
            .call(
                0x0051_80b0,
                &args![DOOR, 0u32, PLAYER_REF, 0u32, 0u32, 0u32]
            )
            .bool());
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        let (answer, log, _) = activate_door(Activation {
            busy: true,
            ..activation()
        });
        assert!(!answer);
        assert!(calls_to(&log, 0x0056_8e50).is_empty());
    }

    #[test]
    fn door_activation_finds_a_random_destination_or_gives_up() {
        let (answer, log, _) = activate_door(Activation {
            random: true,
            ..activation()
        });
        assert!(!answer);
        assert_eq!(
            calls_to(&log, 0x0070_52f0),
            vec![vec![
                NO_DESTINATION_SETTING + 1,
                0,
                SAD_ICON,
                0,
                2.0f32.to_bits(),
                0
            ]]
        );
        // A destination that still leaves no teleport data.
        let (answer, log, _) = activate_door(Activation {
            random: true,
            destination: 0x0006_6000,
            ..activation()
        });
        assert!(!answer);
        assert_eq!(
            calls_to(&log, 0x0051_9810),
            vec![vec![REFERENCE, 0x0006_6000]]
        );
        assert!(calls_to(&log, 0x00ec_43fb).is_empty());
    }

    #[test]
    fn door_activation_without_teleport_opens_the_door() {
        let (answer, log, e) = activate_door(activation());
        assert!(answer);
        assert_eq!(e.mem.u32(PLAYER_REF + 0x200), 0);
        assert_eq!(calls_to(&log, 0x0051_7360), vec![vec![0x0006_2000]]);
        assert_eq!(
            calls_to(&log, 0x0047_a560),
            vec![vec![REFERENCE, PLAYER_REF, DOOR + 0x70]]
        );
        // The ignore check is asked with both flags set.
        assert_eq!(calls_to(&log, 0x0050_1d20)[0][1], REFERENCE);
    }

    #[test]
    fn door_activation_player_picks_a_locked_door() {
        for key in [0u32, 0x0006_7000] {
            let (answer, log, _) = activate_door(Activation {
                lock: 0x0006_0000,
                lock_active: true,
                key,
                ..activation()
            });
            assert!(!answer);
            assert_eq!(calls_to(&log, 0x0070_5830), vec![vec![REFERENCE, 0, 0]]);
            assert!(calls_to(&log, 0x0047_a560).is_empty());
        }
    }

    #[test]
    fn door_activation_player_with_the_key_unlocks_and_activates() {
        let (answer, log, e) = activate_door(Activation {
            lock: 0x0006_0000,
            lock_active: true,
            key: 0x0006_7000,
            has_key: true,
            ..activation()
        });
        assert!(answer);
        assert_eq!(calls_to(&log, 0x0040_8da0), vec![vec![0x0006_7030]]);
        assert_eq!(
            calls_to(&log, 0x0057_3170),
            vec![vec![REFERENCE, PLAYER_REF, 0, 0, 1]]
        );
        assert_eq!(e.global::<u32>(UNLOCKING_DOOR), 0);
        assert_eq!(calls_to(&log, SOUND_PLAY), vec![vec![0x0009_1000, 0]]);
    }

    #[test]
    fn door_activation_npc_uses_the_lock_then_opens_the_door() {
        // Without the used item the NPC gives up; with it, it unlocks and goes on.
        let (answer, log, _) = activate_door(Activation {
            actor: NPC_REF,
            lock: 0x0006_0000,
            lock_active: true,
            ..activation()
        });
        assert!(!answer);
        assert!(calls_to(&log, 0x0060_0900).is_empty());
        let (answer, log, _) = activate_door(Activation {
            actor: NPC_REF,
            lock: 0x0006_0000,
            lock_active: true,
            has_used_item: true,
            ..activation()
        });
        assert!(answer);
        assert_eq!(
            calls_to(&log, 0x0300_1c44),
            vec![vec![0x0009_3000, NPC_REF, 0, 2, 1, 0, 1]]
        );
        assert_eq!(
            calls_to(&log, 0x0047_a560),
            vec![vec![REFERENCE, NPC_REF, DOOR + 0x70]]
        );
        // An actor that can ignore the lock goes straight on.
        let (answer, log, _) = activate_door(Activation {
            actor: NPC_REF,
            lock: 0x0006_0000,
            lock_active: true,
            ignores: true,
            ..activation()
        });
        assert!(answer);
        assert!(calls_to(&log, 0x0089_1db0).is_empty());
    }

    #[test]
    fn door_activation_fades_out_through_a_load_door() {
        for (actor, sound) in [
            (PLAYER_REF, 0x0006_9000u32),
            (NPC_REF, 0x0006_9000),
            (NPC_REF, 0),
        ] {
            let (answer, log, _) = activate_door(Activation {
                actor,
                teleport: 0x0006_1000,
                sound,
                ..activation()
            });
            assert_eq!(answer, actor == PLAYER_REF);
            assert_eq!(
                calls_to(&log, 0x008f_e960),
                vec![vec![0x0009_3000, actor, REFERENCE, 1]]
            );
            let handles = calls_to(&log, SOUND_BY_ID);
            if sound == 0 {
                assert!(handles.is_empty());
                continue;
            }
            let flags = if actor == PLAYER_REF {
                0x8000_0121
            } else {
                0x102
            };
            assert_eq!(handles[0][2..], [sound + 2, flags]);
            let handle = calls_to(&log, 0x0041_a250)[0][0];
            assert_eq!(
                calls_to(&log, 0x0041_8900),
                vec![vec![handle, handles[0][1]]]
            );
            assert_eq!(calls_to(&log, SOUND_PLAY), vec![vec![handle, 0]]);
            if actor == PLAYER_REF {
                assert!(calls_to(&log, SOUND_SET_POSITION).is_empty());
                assert_eq!(calls_to(&log, 0x00ad_7740).len(), 4);
            } else {
                assert_eq!(
                    calls_to(&log, SOUND_SET_POSITION),
                    vec![vec![
                        handle,
                        1.0f32.to_bits(),
                        2.0f32.to_bits(),
                        3.0f32.to_bits()
                    ]]
                );
                assert!(calls_to(&log, 0x00ad_7740).is_empty());
            }
        }
    }

    #[test]
    fn door_activation_moves_the_player_while_sleeping() {
        for interior in [false, true] {
            let (answer, log, e) = activate_door(Activation {
                teleport: 0x0006_1000,
                sleeping: true,
                interior,
                ..activation()
            });
            assert!(answer);
            assert_eq!(
                calls_to(&log, 0x0300_1d34),
                vec![vec![0x0009_3000, REFERENCE]]
            );
            assert_eq!(calls_to(&log, 0x0096_2880), vec![vec![PLAYER_REF]]);
            assert_eq!(
                e.mem.u32(PLAYER_REF + 0x604),
                if interior { 0 } else { REFERENCE }
            );
            assert_eq!(e.mem.u8(PLAYER_REF + 0x20c), 0);
            assert!(calls_to(&log, 0x008f_e960).is_empty());
        }
    }

    #[test]
    fn door_activation_moves_an_npc_to_the_teleport_destination() {
        for loaded in [false, true] {
            // The player sleeping or resting sends an NPC through the move path too.
            let case = Activation {
                actor: NPC_REF,
                teleport: 0x0006_1000,
                cell_loaded: loaded,
                sleeping: true,
                ..activation()
            };
            let (answer, log, _) = activate_door(case);
            assert!(answer);
            assert_eq!(
                calls_to(&log, 0x0057_5830),
                vec![vec![NPC_REF, 0x0006_3000]]
            );
            assert_eq!(
                calls_to(&log, 0x0045_11e0),
                vec![vec![0x0009_a000, 0x0006_4000, 0]]
            );
            if loaded {
                assert_eq!(calls_to(&log, 0x0057_57d0), vec![vec![NPC_REF, 0]]);
                assert_eq!(calls_to(&log, 0x0057_5770), vec![vec![NPC_REF, 0]]);
            } else {
                assert!(calls_to(&log, 0x0057_57d0).is_empty());
                assert_eq!(
                    calls_to(&log, 0x0057_5770),
                    vec![vec![NPC_REF, f32::MAX.to_bits()]]
                );
            }
            assert_eq!(calls_to(&log, 0x008a_e640), vec![vec![NPC_REF, 0]]);
            assert_eq!(
                calls_to(&log, 0x0057_3800),
                vec![vec![NPC_REF, 0x0006_4000, 0x0006_5000]]
            );
        }
    }
}

#[cfg(test)]
mod save_copy_teleport_tests {
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

    /// List doubles: a node is two words (item, next); a door's teleport
    /// list is its `+0x88` node.
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(TELEPORT_LIST, |_, a| ret(a[0] + 0x88));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(NODE_IS_EMPTY, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(LIST_COUNT, |e, a| {
            let (mut node, mut n) = (a[0], 0);
            while node != 0 {
                n += u32::from(e.mem.u32(node) != 0);
                node = e.mem.u32(node + 4);
            }
            ret(n)
        });
        e.register(LIST_HOLDS, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) == item {
                    return ret(1);
                }
                node = e.mem.u32(node + 4);
            }
            ret(0)
        });
        e.register(LIST_APPEND, |e, a| {
            let item = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) == 0 {
                e.mem.set_u32(a[0], item);
                return Ret::default();
            }
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let new = e.mem.alloc(8);
            e.mem.set_u32(new, item);
            e.mem.set_u32(node + 4, new);
            Ret::default()
        });
        e.register(TELEPORT_LIST_CLEAR, |e, a| {
            e.mem.set_u32(a[0] + 0x88, 0);
            e.mem.set_u32(a[0] + 0x8c, 0);
            Ret::default()
        });
        e.register(SOUND_78, |e, a| ret(e.mem.u32(a[0] + 0x78)));
        e.register(SOUND_7C, |e, a| ret(e.mem.u32(a[0] + 0x7c)));
        e.register(SOUND_80, |e, a| ret(e.mem.u32(a[0] + 0x80)));
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e
    }

    /// A door (0x90 bytes) whose teleport list holds `items`.
    fn door(e: &mut Engine, items: &[u32]) -> u32 {
        let this = e.mem.alloc(0x90);
        let mut node = this + 0x88;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        this
    }

    fn items(e: &Engine, this: u32) -> Vec<u32> {
        let mut out = vec![];
        let mut node = this + 0x88;
        while node != 0 && e.mem.u32(node) != 0 {
            out.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        out
    }

    fn form(e: &mut Engine, id: u32) -> u32 {
        let form = e.mem.alloc(0x18);
        e.mem.set_u32(form + 0xc, id);
        form
    }

    #[test]
    fn save_writes_components_sounds_flags_and_teleports() {
        let mut e = list_engine();
        quiet(
            &mut e,
            &[
                START_FORM,
                CLOSE_FORM,
                FULL_NAME_SAVE,
                MODEL_TEXTURE_SWAP_SAVE,
                SCRIPTABLE_FORM_SAVE,
                DESTRUCTIBLE_SAVE,
                ADD_CHUNK_WORD,
                ADD_CHUNK_BYTE,
            ],
        );
        let a = form(&mut e, 0x31);
        let b = form(&mut e, 0x32);
        let this = door(&mut e, &[a, b]);
        e.mem.set_u32(this + 0x7c, a);
        e.mem.set_u8(this + 0x84, 0x15);
        let (_, log) = run(&mut e, 0x0051_7c30, &args![this]);
        assert_eq!(
            only(&log, MODEL_TEXTURE_SWAP_SAVE),
            vec![vec![this + 0x3c, MODL, MODT, MODS]]
        );
        assert_eq!(only(&log, SCRIPTABLE_FORM_SAVE), vec![vec![this + 0x5c]]);
        assert_eq!(only(&log, DESTRUCTIBLE_SAVE), vec![vec![this + 0x68]]);
        assert_eq!(only(&log, ADD_CHUNK_BYTE), vec![vec![FNAM, 0x15]]);
        assert_eq!(
            only(&log, ADD_CHUNK_WORD),
            vec![vec![ANAM, 0x31], vec![TNAM, 0x31], vec![TNAM, 0x32]]
        );
        assert_eq!(only(&log, CLOSE_FORM), vec![vec![this]]);
    }

    #[test]
    fn copy_and_compare_follow_lists_flags_and_sounds() {
        let mut e = list_engine();
        quiet(&mut e, &[COPY_ALL_COMPONENTS]);
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(0x100));
        let other = door(&mut e, &[0x501, 0x502]);
        e.mem.set_u8(other + 0x84, 6);
        e.mem.set_u32(other + 0x78, 0x78);
        e.mem.set_u32(other + 0x7c, 0x7c);
        e.mem.set_u32(other + 0x80, 0x80);
        let this = door(&mut e, &[0x999]);
        assert!(e.call(0x0051_7e70, &args![this, other]).bool());
        let (_, log) = run(&mut e, 0x0051_7d70, &args![this, other]);
        assert_eq!(items(&e, this), vec![0x501, 0x502]);
        assert_eq!(e.mem.u8(this + 0x84), 6);
        assert_eq!(
            [0x78, 0x7c, 0x80].map(|o| e.mem.u32(this + o)),
            [0x78, 0x7c, 0x80]
        );
        assert_eq!(only(&log, COPY_ALL_COMPONENTS), vec![vec![this, other]]);
        assert!(!e.call(0x0051_7e70, &args![this, other]).bool());
        e.mem.set_u8(this + 0x84, 7);
        assert!(e.call(0x0051_7e70, &args![this, other]).bool());
        e.mem.set_u8(this + 0x84, 6);
        e.mem.set_u32(this + 0x80, 1);
        assert!(e.call(0x0051_7e70, &args![this, other]).bool());
        e.mem.set_u32(this + 0x80, 0x80);
        let swapped = door(&mut e, &[0x502, 0x501]);
        for o in [0x78u32, 0x7c, 0x80] {
            e.mem.set_u32(swapped + o, o);
        }
        e.mem.set_u8(swapped + 0x84, 6);
        assert!(!e.call(0x0051_7e70, &args![swapped, other]).bool());
        let different = door(&mut e, &[0x502, 0x503]);
        for o in [0x78u32, 0x7c, 0x80] {
            e.mem.set_u32(different + o, o);
        }
        e.mem.set_u8(different + 0x84, 6);
        assert!(e.call(0x0051_7e70, &args![different, other]).bool());
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(1));
        assert!(e.call(0x0051_7e70, &args![this, other]).bool());
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(e.call(0x0051_7e70, &args![this, other]).bool());
        let fresh = door(&mut e, &[0x777]);
        e.call(0x0051_7d70, &args![fresh, other]);
        assert_eq!(items(&e, fresh), vec![0x777]);
    }

    #[test]
    fn flag_bits_two_and_three() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x90);
        assert!(!e.call(0x0051_7fd0, &args![this]).bool());
        e.mem.set_u8(this + 0x84, 0x04);
        assert!(e.call(0x0051_7fd0, &args![this]).bool());
        e.call(0x0051_8030, &args![this, 1u32]);
        assert_eq!(e.mem.u8(this + 0x84), 0x0c);
        e.call(0x0051_8030, &args![this, 0x100u32]);
        assert_eq!(e.mem.u8(this + 0x84), 0x04);
    }

    #[test]
    fn map_construction_and_destruction() {
        let mut e = Engine::new();
        e.register(NI_ALLOC, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MEMSET, |_, _| Ret::default());
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(NI_FREE, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        let (r, log) = run(&mut e, 0x0051_9b80, &args![map, 0x25u32]);
        assert_eq!(r.eax, map);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
        assert_eq!((e.mem.u32(map + 4), e.mem.u32(map + 0xc)), (0x25, 0));
        let buckets = e.mem.u32(map + 8);
        assert_eq!(only(&log, NI_ALLOC), vec![vec![0x94]]);
        assert_eq!(only(&log, MEMSET), vec![vec![buckets, 0, 0x94]]);
        let (_, log) = run(&mut e, 0x0051_9bf0, &args![map]);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
        assert_eq!(only(&log, MAP_REMOVE_ALL), vec![vec![map], vec![map]]);
        assert_eq!(only(&log, NI_FREE), vec![vec![buckets]]);
        e.mem.set_u32(map, 0);
        let (_, log) = run(&mut e, 0x0051_9c50, &args![map]);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
        assert_eq!(only(&log, NI_FREE).len(), 1);
    }

    #[test]
    fn arrival_callback_plays_the_sound() {
        let mut e = Engine::new();
        e.register(ARRIVAL_SOUND, |_, a| ret(a[0] + 0x100));
        e.register(FORM_ID, |_, a| ret(a[0] + 1));
        e.register(AUDIO, |_, _| ret(0xa000));
        e.register(SOUND_BY_ID, |_, a| ret(a[1]));
        quiet(&mut e, &[SOUND_PLAY, SOUND_HANDLE_DESTROY]);
        let (_, log) = run(&mut e, 0x0075_9030, &args![0u32, 0x5000u32]);
        let by_id = only(&log, SOUND_BY_ID);
        assert_eq!(
            (by_id[0][0], by_id[0][2], by_id[0][3]),
            (0xa000, 0x5101, 0x101)
        );
        let handle = by_id[0][1];
        assert_eq!(only(&log, SOUND_PLAY), vec![vec![handle, 0]]);
        assert_eq!(only(&log, SOUND_HANDLE_DESTROY), vec![vec![handle]]);
    }

    /// The random door search with `items` in the list and `random` the
    /// sequence `00487f50` answers; `0054afb0` finds item `0x700 + n` as
    /// `0x7700 + n` when `n` is in `hits`.
    fn search(
        items: &[u32],
        random: Vec<u32>,
        hits: Vec<u32>,
        preferred: u32,
    ) -> (Engine, u32, Log) {
        let mut e = list_engine();
        e.map(PLAYER_REFERENCE & !0xfff, 0x1000);
        e.set_global::<u32>(PLAYER_REFERENCE, 0x0bad_0000);
        e.register(GET_WORLD_SPACE, |_, _| ret(0x4400));
        let mut draws = random.into_iter();
        e.register_double(RANDOM, move |_, _| ret(draws.next().unwrap()));
        e.register(VISITED_MAP_CONSTRUCT, |e, a| {
            let set = e.mem.alloc(0x40);
            e.mem.set_u32(a[0], set);
            Ret::default()
        });
        e.register(VISITED_MAP_LOOKUP, |e, a| {
            let set = e.mem.u32(a[0]);
            let marked = e.mem.u8(set + a[1]);
            e.mem.set_u8(a[2], marked);
            ret(u32::from(marked != 0))
        });
        e.register(VISITED_MAP_INSERT, |e, a| {
            let set = e.mem.u32(a[0]);
            e.mem.set_u8(set + a[1], a[2] as u8);
            Ret::default()
        });
        e.register_double(PLAYER_PREFERRED_DOOR, move |_, _| ret(preferred));
        e.register_double(FIND_DESTINATION, move |e, a| {
            let n = a[0].wrapping_sub(0x700);
            if hits.contains(&n) {
                e.mem.set_u32(a[2], 1);
                ret(0x7700 + n)
            } else {
                ret(0)
            }
        });
        quiet(&mut e, &[PLAYER_SET_DOOR_CHOICE, MAP_REMOVE_ALL, NI_FREE]);
        let this = door(&mut e, items);
        let (r, log) = run(&mut e, 0x0051_95e0, &args![this, 0x9000u32, 0u32]);
        (e, r.eax, log)
    }

    #[test]
    fn random_destination_skips_visited_and_preferred() {
        let items = [0x700, 0x701, 0x702];
        let (_, found, log) = search(&items, vec![4, 2], vec![1], 0xff);
        assert_eq!(found, 0x7701);
        assert_eq!(only(&log, FIND_DESTINATION).len(), 1);
        assert_eq!(only(&log, PLAYER_SET_DOOR_CHOICE)[0][2], 1);
        let (_, found, log) = search(&items, vec![1, 1, 0, 2], vec![1], 1);
        assert_eq!(found, 0);
        assert_eq!(only(&log, RANDOM).len(), 4);
        assert_eq!(only(&log, FIND_DESTINATION).len(), 3);
        assert_eq!(only(&log, VISITED_MAP_INSERT).len(), 3);
        assert_eq!(only(&log, MAP_REMOVE_ALL).len(), 2);
    }

    #[test]
    fn random_destination_needs_a_reference_and_a_world_space() {
        let mut e = list_engine();
        let this = door(&mut e, &[0x700]);
        let (r, log) = run(&mut e, 0x0051_95e0, &args![this, 0u32, 0u32]);
        assert_eq!((r.eax, log.len()), (0, 1));
        e.register(GET_WORLD_SPACE, |_, _| ret(0));
        e.register(REFERENCE_PARENT_008D6F30, |_, _| ret(0x5500));
        e.register(PARENT_CHECK_00425FD0, |_, _| ret(0x100));
        let (r, log) = run(&mut e, 0x0051_95e0, &args![this, 0x9000u32, 0u32]);
        assert_eq!(r.eax, 0);
        assert_eq!(only(&log, REFERENCE_PARENT_008D6F30).len(), 2);
        assert!(only(&log, VISITED_MAP_CONSTRUCT).is_empty());
    }

    /// Two door references (vtable slot `+0x1f4` answers their position
    /// block), their bases doors, and the teleport doubles.
    fn link_engine() -> (Engine, u32, u32) {
        let mut e = list_engine();
        e.register(BASE_OBJECT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(TELEPORT_DATA, |_, _| ret(0));
        e.register(ADD_TELEPORT, |_, a| ret(a[0] + 0x40));
        let setting = e.mem.alloc(8);
        e.mem.set_u32(setting, 64.0f32.to_bits());
        e.mem.set_u32(setting + 4, 32.0f32.to_bits());
        e.register_double(SETTING_FLOAT_ADDRESS, move |_, a| {
            ret(if a[0] == DOOR_DISTANCE_SETTING {
                setting
            } else {
                setting + 4
            })
        });
        e.map(DISTANCE_EXTRA & !0xfff, 0x1000);
        e.set_global::<f64>(DISTANCE_EXTRA, 100.0);
        e.map(DOOR_ANGLE & !0xfff, 0x1000);
        e.set_global::<u32>(DOOR_ANGLE, 0.5f32.to_bits());
        e.register(FLOAT_PAIR_00404010, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        e.register(TELEPORT_MARKER, |_, _| ret(0));
        e.register(REFERENCE_ROTATION, |_, a| ret(a[0] + 0x30));
        e.register(VECTOR_ADD, |_, a| ret(a[1]));
        e.register(VECTOR_SET, |_, a| ret(a[0]));
        quiet(
            &mut e,
            &[
                SET_LINKED_DOOR,
                SET_TELEPORT_POSITION,
                SET_TELEPORT_ROTATION,
                DELTA_FOR_ANGLE,
                VECTOR_SCALE,
            ],
        );
        e.register(0x0f48_01f4, |_, a| ret(a[0] + 0x20));
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + 0x1f4, 0x0f48_01f4);
        let reference = |e: &mut Engine, flags: u8| {
            let r = e.mem.alloc(0x80);
            e.mem.set_u32(r, vtable);
            let base = e.mem.alloc(0x90);
            e.mem.set_u8(base + 0x84, flags);
            e.mem.set_u32(r + 4, base);
            e.mem.set_u32(r + 0x38, 1.0f32.to_bits());
            r
        };
        let first = reference(&mut e, 0);
        let second = reference(&mut e, 2);
        (e, first, second)
    }

    #[test]
    fn linking_places_each_side_at_the_other() {
        let (mut e, first, second) = link_engine();
        let (_, log) = run(&mut e, 0x0051_9810, &args![first, second]);
        assert_eq!(
            only(&log, SET_LINKED_DOOR),
            vec![vec![first + 0x40, second], vec![second + 0x40, first]]
        );
        let scales = only(&log, VECTOR_SCALE);
        // The second door has flag 0x02: the larger of 64 and 32 + 100.
        assert_eq!(scales[0][1], 132.0f32.to_bits());
        assert_eq!(scales[1][1], 64.0f32.to_bits());
        let deltas = only(&log, DELTA_FOR_ANGLE);
        assert_eq!((deltas[0][0], deltas[0][2]), (second, 0.5f32.to_bits()));
        let sets = only(&log, VECTOR_SET);
        assert_eq!(sets[0][1..], [0, 0, 1.5f32.to_bits()]);
        let positions = only(&log, SET_TELEPORT_POSITION);
        assert_eq!(positions[0][0], first + 0x40);
        assert_eq!(only(&log, VECTOR_ADD)[0][0], second + 0x20);
    }

    #[test]
    fn linking_uses_markers_and_refuses_bad_pairs() {
        let (mut e, first, second) = link_engine();
        let marker_ref = e.mem.alloc(0x40);
        let vtable = e.mem.u32(first);
        e.mem.set_u32(marker_ref, vtable);
        e.register_double(TELEPORT_MARKER, move |_, _| ret(marker_ref));
        let (_, log) = run(&mut e, 0x0051_9810, &args![first, second]);
        assert_eq!(
            only(&log, SET_TELEPORT_POSITION),
            vec![
                vec![first + 0x40, marker_ref + 0x20],
                vec![second + 0x40, marker_ref + 0x20]
            ]
        );
        assert_eq!(
            only(&log, SET_TELEPORT_ROTATION),
            vec![
                vec![first + 0x40, marker_ref + 0x30],
                vec![second + 0x40, marker_ref + 0x30]
            ]
        );
        assert!(only(&log, DELTA_FOR_ANGLE).is_empty());
        for (a, b) in [(0, second), (first, 0), (first, first)] {
            let (_, log) = run(&mut e, 0x0051_9810, &args![a, b]);
            assert_eq!(log.len(), 1);
        }
        e.register(TELEPORT_DATA, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0051_9810, &args![first, second]);
        assert!(only(&log, ADD_TELEPORT).is_empty());
        e.register(TELEPORT_DATA, |_, _| ret(0));
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0051_9810, &args![first, second]);
        assert!(only(&log, TELEPORT_DATA).is_empty());
    }
}
