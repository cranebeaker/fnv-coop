//! `fallout shared/tesobjectcont.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005160f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005160f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESObjectCONT::TESObjectCONT` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0x1b (0x9c bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn tesobjectcont_tesobjectcont(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0048_1610, &args![base + 0x30]);
    e.call(0x0040_2d00, &args![base + 0x3c]);
    e.call(0x0048_a3d0, &args![base + 0x48]);
    e.call(0x0048_cc50, &args![base + 0x68]);
    e.call(0x0048_eac0, &args![base + 0x74]);
    e.call(0x0047_5350, &args![base + 0x7c]);
    e.call(0x0051_0620, &args![base + 0x84]);
    e.mem.set_u32(base, 0x0102_aeb4);
    e.mem.set_u32(base + 0x30, 0x0102_ae9c);
    e.mem.set_u32(base + 0x3c, 0x0102_ae88);
    e.mem.set_u32(base + 0x48, 0x0102_ae64);
    e.mem.set_u32(base + 0x68, 0x0102_ae50);
    e.mem.set_u32(base + 0x74, 0x0102_ae3c);
    e.mem.set_u32(base + 0x7c, 0x0102_ae28);
    e.mem.set_u32(base + 0x84, 0x0102_ae18);
    e.call(0x004f_15a0, &args![this, 0x1b]);
    e.call(0x0051_6330, &args![this]);
    this
}

/// Record tags (`u32::from_le_bytes` of the four characters) the loader `00516580` tells apart.
const MODL: u32 = u32::from_le_bytes(*b"MODL");
const MODD: u32 = u32::from_le_bytes(*b"MODD");
const MOSD: u32 = u32::from_le_bytes(*b"MOSD");
const MODT: u32 = u32::from_le_bytes(*b"MODT");
const MODS: u32 = u32::from_le_bytes(*b"MODS");
const OBND: u32 = u32::from_le_bytes(*b"OBND");
const EDID: u32 = u32::from_le_bytes(*b"EDID");
const FULL: u32 = u32::from_le_bytes(*b"FULL");
const SCRI: u32 = u32::from_le_bytes(*b"SCRI");
const CNTO: u32 = u32::from_le_bytes(*b"CNTO");
const COED: u32 = u32::from_le_bytes(*b"COED");
const DATA: u32 = u32::from_le_bytes(*b"DATA");
const DSTD: u32 = u32::from_le_bytes(*b"DSTD");
const DEST: u32 = u32::from_le_bytes(*b"DEST");
const SNAM: u32 = u32::from_le_bytes(*b"SNAM");
const QNAM: u32 = u32::from_le_bytes(*b"QNAM");
const RNAM: u32 = u32::from_le_bytes(*b"RNAM");
/// The container item entry (`004817f0`'s answer) whose extra data a following `COED` chunk fills.
const PENDING_CONTAINER_ITEM: u32 = 0x011c_9348;

// Translated from 00516330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Start values of `TESObjectCONT` (form type 0x1b, constructed by `005160f0`): clears the byte
/// at `+0x98` and the words at `+0x8c`, `+0x90` and `+0x94`.
pub fn fn_00516330(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u8(base + 0x98, 0);
    e.mem.set_u32(base + 0x8c, 0);
    e.mem.set_u32(base + 0x90, 0);
    e.mem.set_u32(base + 0x94, 0);
}

// Translated from 00516580 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00516580 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Record loader of `TESObjectCONT` (form type 0x1b; the engine map gives the function no name).
/// `this` is the form and `file` the `TESFile` positioned on its record. Answers false without
/// touching the form when the record is not of type 0x1b, otherwise true.
///
/// It runs `TESForm::LoadForm` (`00485110`), `00484ab0` with 0, and walks the chunks:
///
/// - `EDID`: reads the chunk (limit 0x200) and runs virtual slot `+0x134` on it; `OBND`: slot
///   `+0xe0` with the file;
/// - `FULL`: `00487050` on `+0x3c`; `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk`
///   (`004892d0`) on `+0x48`; `MODS`: the texture swaps into `+0x48` (`0048a7b0`);
/// - `SCRI`: reads a word, `006ecd40` on `+0x68` with it and `0048cd90` on `+0x68` with the form;
/// - `CNTO` and `COED` (the file's chunk is asked again): for `CNTO` it reads 8 bytes into a
///   local (zeroed with `memset`), runs `00462230` on it when `00401680` answers true and keeps
///   `004817f0(+0x30, local)` in the global at `0x011c9348`; for `COED` with that global set it
///   makes a 0xc byte object (`00401000`, `0040e690`), stores it at `+8` of the entry, loads it
///   (`0040e780` with the file) and clears the global;
/// - `DATA`: `TESForm::LoadData` (`004861f0`) of 1 byte into `+0x98`; `DSTD`, `DEST`:
///   `004781e0` on `+0x7c`;
/// - `SNAM`, `QNAM`, `RNAM`: reads a word and gives it to `008cc6d0`, `004fb0b0` and
///   `004febf0`.
///
/// The buffers the game takes from the stack (`__alloca_probe_16`) are heap blocks here. The
/// exception frame of the allocation of the 0xc byte object is not translated.
pub fn fn_00516580(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if e.call(0x0047_2660, &args![file]).u8() != 0x1b {
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
                let target = if base != 0 { base + 0x3c } else { 0 };
                e.call(0x0048_7050, &args![target, file]);
            }
            MODL | MODD | MOSD | MODT => {
                let model = if base != 0 { base + 0x48 } else { 0 };
                e.call(0x0048_92d0, &args![model, file]);
            }
            MODS => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, size]);
                let swap_flags = e.call(0x0040_3570, &args![file]).u32() & 0xffff;
                e.call(0x0048_a7b0, &args![base + 0x48, buffer, size, swap_flags]);
                e.mem.free(buffer);
            }
            SCRI => {
                let value = read_word(e, file);
                e.call(0x006e_cd40, &args![base + 0x68, value]);
                e.call(0x0048_cd90, &args![base + 0x68, this]);
            }
            CNTO | COED => {
                let current = e.call(0x0047_26b0, &args![file]).u32();
                if current == COED {
                    let entry = e.global::<u32>(PENDING_CONTAINER_ITEM);
                    if entry != 0 {
                        let block = e.call(0x0040_1000, &args![0xcu32]).u32();
                        let extra = if block != 0 {
                            e.call(0x0040_e690, &args![block]).u32()
                        } else {
                            0
                        };
                        let entry = e.global::<u32>(PENDING_CONTAINER_ITEM);
                        e.mem.set_u32(entry + 8, extra);
                        let entry = e.global::<u32>(PENDING_CONTAINER_ITEM);
                        let extra = e.mem.u32(entry + 8);
                        e.call(0x0040_e780, &args![extra, file]);
                        e.set_global(PENDING_CONTAINER_ITEM, 0u32);
                    }
                } else if current == CNTO {
                    let local = e.mem.alloc(8);
                    e.call(0x0040_3d30, &args![local, 0u32, 8u32]);
                    e.call(0x0047_2890, &args![file, local, 8u32]);
                    if e.call(0x0040_1680, &args![file]).bool() {
                        e.call(0x0046_2230, &args![local]);
                    }
                    let item = e.call(0x0048_17f0, &args![base + 0x30, local]).u32();
                    e.set_global(PENDING_CONTAINER_ITEM, item);
                    e.mem.free(local);
                }
            }
            DATA => {
                e.call(0x0048_61f0, &args![this, file, base + 0x98, 1u32]);
            }
            DSTD | DEST => {
                let target = if base != 0 { base + 0x7c } else { 0 };
                e.call(0x0047_81e0, &args![target, file]);
            }
            SNAM => {
                let value = read_word(e, file);
                e.call(0x008c_c6d0, &args![this, value]);
            }
            QNAM => {
                let value = read_word(e, file);
                e.call(0x004f_b0b0, &args![this, value]);
            }
            RNAM => {
                let value = read_word(e, file);
                e.call(0x004f_ebf0, &args![this, value]);
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

// Translated from 00516210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00516240` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00516210(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_6240, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The player reference (`011dea3c`), compared with the activating actor.
const PLAYER: u32 = 0x011d_ea3c;
/// The form `00516dc0` asks the actor for first with `Actor::HasObjects` (`011ca268`; the
/// NPC path also hands it to `TESIdleManager::SetUsedItem`).
const USED_ITEM_FORM: u32 = 0x011c_a268;
/// The container being unlocked with a key while the player activates it (`011c92d0`): set
/// for the duration of that activation, otherwise 0.
const UNLOCKING_CONTAINER: u32 = 0x011c_92d0;
/// The `float` 2.0 the code pushes (`FLD`/`FSTP`) as the message display time.
const MESSAGE_TIME: u32 = 0x0101_62c0;
/// `.?AVTESObjectREFR@@` and `.?AVActor@@`: the cast of the activating reference.
const OBJECT_REFR_TYPE: u32 = 0x0118_41cc;
const ACTOR_TYPE: u32 = 0x0118_46d4;
/// "UILockpickingUnlock", the sound played when a key opens the lock.
const UNLOCK_SOUND: u32 = 0x0102_b13c;
/// "Interface\Icons\Message Icons\glow_message_key.dds".
const KEY_ICON: u32 = 0x0102_b108;
/// "Interface\Icons\Message Icons\glow_message_padlock.dds".
const PADLOCK_ICON: u32 = 0x0102_b0d0;
/// The setting whose string (`00403df0` on it) is the "unlocked with key" format.
const UNLOCKED_WITH_KEY_SETTING: u32 = 0x011d_5110;
/// The setting whose string is the message for a lock that cannot be picked.
const CANNOT_PICK_SETTING: u32 = 0x011d_276c;
/// "MASTERFILE: Could not find open sound (%08X) for container '%s' (%08X)." (the exe uses
/// it for both sounds at +0x8c and +0x90).
const MISSING_OPEN_SOUND: u32 = 0x0102_b088;
/// "MASTERFILE: Could not find pLoopingSound (%08X) for container '%s' (%08X)."
const MISSING_LOOPING_SOUND: u32 = 0x0102_b038;
/// `.?AVTESForm@@` and `.?AVTESSound@@`, the sound casts of `InitItem`.
const TES_FORM_TYPE: u32 = 0x0118_3028;
const SOUND_TYPE: u32 = 0x0118_32fc;

// Translated from 00516bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): the word at +0x90 (the container's
/// second sound, resolved by `InitItem`).
pub fn fn_00516bf0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x90)
}

// Translated from 00517360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` the lock's extra data, plain `RET`): runs
/// `REFR_LOCK::SetLocked` (`00430a90`) on the lock at +0xc with 0 (unlocks it); its `EAX`
/// is left as the result.
pub fn fn_00517360(e: &mut Engine, this: Ptr) -> u32 {
    let lock = e.mem.u32(this.addr() + 0xc);
    e.call(0x0043_0a90, &args![lock, 0u32]).u32()
}

// Translated from 00517380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCONT::HandleOpenFinish` (Xbox PDB; `ECX` and two stack words, `RET 8`): when
/// `reference` is not null and `actor` is the player (`011dea3c`), queues a menu through
/// `Interface::QueueMenuCreate` (`00709470`, cdecl: 1, the reference, 0, 0, 1, 0), then
/// runs `BGSOpenCloseForm::HandleActivate` (`0047a560`, cdecl) with the reference, the actor
/// and 0.
pub fn tes_object_cont_handle_open_finish(e: &mut Engine, _this: Ptr, reference: Ptr, actor: Ptr) {
    if reference.is_null() || actor.addr() != e.global::<u32>(PLAYER) {
        return;
    }
    e.call(0x0070_9470, &args![1u32, reference, 0u32, 0u32, 1u32, 0u32]);
    e.call(0x0047_a560, &args![reference, actor, 0u32]);
}

// Translated from 00516240 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00516240 - the exception-unwinding frame (FS:[0] chain, handler 00ef46c4, state variable) is not translated.
/// Destructor of `TESObjectCONT` (unnamed in the engine map; the scalar deleting destructor
/// `00516210` runs it; `ECX` only, plain `RET`): stores the vtables of the class and its
/// components (+0 `0x0102_aeb4`, +0x30 `0x0102_ae9c`, +0x3c `0x0102_ae88`, +0x48
/// `0x0102_ae64`, +0x68 `0x0102_ae50`, +0x74 `0x0102_ae3c`, +0x7c `0x0102_ae28`, +0x84
/// `0x0102_ae18`), then the component destructors in reverse order: `00475380` on +0x7c,
/// `0048eaf0` on +0x74, `0048a490` on +0x48, `00591300` on +0x3c, `00481680` on +0x30 and
/// `0050f740` on the object.
pub fn fn_00516240(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, vtable) in [
        (0x00u32, 0x0102_aeb4u32),
        (0x30, 0x0102_ae9c),
        (0x3c, 0x0102_ae88),
        (0x48, 0x0102_ae64),
        (0x68, 0x0102_ae50),
        (0x74, 0x0102_ae3c),
        (0x7c, 0x0102_ae28),
        (0x84, 0x0102_ae18),
    ] {
        e.mem.set_u32(base + offset, vtable);
    }
    e.call(0x0047_5380, &args![base + 0x7c]);
    e.call(0x0048_eaf0, &args![base + 0x74]);
    e.call(0x0048_a490, &args![base + 0x48]);
    e.call(0x0059_1300, &args![base + 0x3c]);
    e.call(0x0048_1680, &args![base + 0x30]);
    e.call(0x0050_f740, &args![this]);
}

/// One sound of `InitItem`: the form ID at `offset`, when not 0, is fixed up in a stack slot
/// through `00485d50` (cdecl, the slot and the container's file from `00484e60` with -1),
/// looked up with `004839c0`, cast to `TESSound` and stored back; a null result prints
/// `warning` with the ID, the container's name (vtable slot +0x130) and the word
/// `0084e3a0` gives.
fn resolve_sound(e: &mut Engine, this: Ptr, offset: u32, warning: u32) {
    let base = this.addr();
    let loaded = e.mem.u32(base + offset);
    if loaded == 0 {
        return;
    }
    let id = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), loaded);
        let file = e.call(0x0048_4e60, &args![this, 0xffff_ffffu32]).u32();
        e.call(0x0048_5d50, &args![slot, file]);
        e.mem.u32(slot.addr())
    });
    let form = e.call(0x0048_39c0, &args![id]).u32();
    let sound = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, TES_FORM_TYPE, SOUND_TYPE, 0u32],
        )
        .u32();
    e.mem.set_u32(base + offset, sound);
    if e.mem.u32(base + offset) != 0 {
        return;
    }
    let form_id = e.call(0x0084_e3a0, &args![this]).u32();
    let name = e.vcall(base, 0x130, &args![]).u32();
    e.call(0x005b_5e40, &args![warning, id, name, form_id]);
}

// Translated from 00516370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCONT::InitItem` (Xbox PDB; `ECX` only, plain `RET`): unless `004013e0` says
/// otherwise, runs the components' `InitItem`s with the form (`0048cd90` on the scriptable
/// form at +0x68, `00481bb0` on the container at +0x30, `00478be0` on the destructible
/// object at +0x7c, `0048aa80` on the model texture swap at +0x48), resolves the sounds at
/// +0x8c and +0x90 (the "open sound" warning) and +0x94 (the "pLoopingSound" warning; see
/// `resolve_sound`), then `00484ab0` with 1.
pub fn tes_object_cont_init_item(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    e.call(0x0048_cd90, &args![base + 0x68, this]);
    e.call(0x0048_1bb0, &args![base + 0x30, this]);
    e.call(0x0047_8be0, &args![base + 0x7c, this]);
    e.call(0x0048_aa80, &args![base + 0x48, this]);
    resolve_sound(e, this, 0x8c, MISSING_OPEN_SOUND);
    resolve_sound(e, this, 0x90, MISSING_OPEN_SOUND);
    resolve_sound(e, this, 0x94, MISSING_LOOPING_SOUND);
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

/// The message display time pushed as a `float` (`FLD`/`FSTP` of `0101_62c0`).
fn message_time(e: &mut Engine) -> u32 {
    x87_quiet_bits(e.global::<u32>(MESSAGE_TIME))
}

/// Unlocks the reference's lock: when `TESObjectREFR::GetExtraLock` (`005691f0`) gives
/// extra data, `00517360` unlocks it and `TESObjectREFR::AddLockChange` (`005789b0`)
/// records the change.
fn unlock(e: &mut Engine, reference: Ptr) {
    let extra = e.call(0x0056_91f0, &args![reference]).u32();
    if extra != 0 {
        fn_00517360(e, Ptr::new(extra));
        e.call(0x0057_89b0, &args![reference]);
    }
}

// Translated from 00516dc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00516dc0 - the exception-unwinding frame (FS:[0] chain, handler 00ef4726, state variable) and the /GS cookie check (00ec408c) are not translated.
/// The `TESObjectCONT` activation (unnamed in the engine map; `ECX` the container and five
/// stack words, `RET 0x14`: the reference, the activating reference, one word never read,
/// and two words handed on to the reference's vtable slot +0x17c and `Actor::StealAlarm`).
/// Returns `AL`.
///
/// The activating reference is cast to `Actor` (`__RTDynamicCast` from `TESObjectREFR`);
/// without an actor the answer is false. The actor owns the container when it is not the
/// player and `TESObjectREFR::IsAnOwner` (`005785e0`, with 1) says so.
///
/// A locked container (`TESObjectREFR::GetLock` gives a lock and `005021a0` on it is true)
/// the actor does not own:
/// - `Actor::HasObjects` (`00891db0`) is asked for the form at `011ca268` (its answer is
///   kept and not used); when the lock has a key (+4) and the actor has it too: with no
///   container being unlocked and the actor the player, the container is recorded at
///   `011c92d0`, unlocked (see `unlock`), "UILockpickingUnlock" plays (through
///   `BSAudio::QInstance`, `GetSoundHandleByName` with 0x121, `BSSoundHandle::Play` with 0,
///   and the handle's destructor `00483710`), the "unlocked with key" message (the setting
///   at `011d5110` formatted with the key's name, `MapMarkerData::GetLocationName` on key
///   +0x30, into a 0x100-byte buffer through `00406d00` with 0xff) is shown through
///   `007052f0` with the key icon and 2.0, the container is activated by the player
///   (`TESObjectREFR::Activate`, with the player, 0, 0, 1), `011c92d0` is cleared and the
///   answer is true; otherwise false;
/// - without the key, and no container being unlocked: for the player, the entry point
///   0x20 runs (`BGSEntryPoint::HandleEntryPoint` with the player and a zeroed `float`);
///   when the lock is level 5 (`REFR_LOCK::GetLockLevel`) or broken (`REFR_LOCK::IsBroken`)
///   the "cannot pick" message (the setting at `011d276c`) shows with the padlock icon,
///   otherwise `00705830` starts lockpicking (the reference, 0, 0). For an NPC: a special
///   idle the actor's process (vtable slot +0x1e4) has is freed (`Animation::
///   SpecialIdleFree`, 1, 0); the used item is set (`TESIdleManager`: the form at
///   `011ca268`, activate 1, level 1) around vtable slot +0x44 of
///   `MiddleHighProcess::GetSavedAcquireObject`'s object (the actor, 0, 2, 1, 0, 1), then
///   reset (0, 0, -1), and the container is unlocked. The answer is false.
///
/// Otherwise (unlocked, or owned): for an NPC with an open sound (`008ce390` on the
/// container) and a reference whose vtable slot +0x1d0 answers nonzero, that sound plays at
/// the reference's position (vtable slot +0x1f4: three floats) through
/// `GetSoundHandleByNumericID` (the sound's `0084e3a0` word, 0x102), `SetPosition` and
/// `Play` with 0. Then for the player: an open container (`BGSOpenCloseForm::GetOpenState`
/// 1) goes to vtable slot +0 of the component at +0x84 (the reference and the actor), any
/// other to `BGSOpenCloseForm::HandleActivate` (the reference, the actor, 0); for an NPC:
/// with a nonzero first handed-on word, the reference's vtable slot +0x17c (that word, 0, the second, 0, 0, the
/// actor, 0, 0, 1, 0); then, when the actor's vtable slot +0x218 answers true, the actor
/// does not own the reference and `TESObjectREFR::GetOwner` gives an owner,
/// `Actor::StealAlarm` (the reference, the two handed-on words, 1, 0). The answer is true.
pub fn fn_00516dc0(
    e: &mut Engine,
    this: Ptr,
    reference: Ptr,
    activator: Ptr,
    _unused_3: u32,
    first_word: u32,
    second_word: u32,
) -> bool {
    let base = this.addr();
    let actor = e
        .call(
            0x00ec_43fb,
            &args![activator, 0u32, OBJECT_REFR_TYPE, ACTOR_TYPE, 0u32],
        )
        .u32();
    // `Actor::StealAlarm`'s last word: a local the code clears and never changes.
    let steal_value = 0u32;
    if actor == 0 {
        return false;
    }
    let player: u32 = e.global(PLAYER);
    let owns =
        activator.addr() != player && e.call(0x0057_85e0, &args![reference, actor, 1u32]).bool();
    let lock = e.call(0x0056_9160, &args![reference]).u32();
    let locked = lock != 0 && e.call(0x0050_21a0, &args![lock]).bool() && !owns;
    if locked {
        return e.with_stack(4, |e, found| {
            let used_item: u32 = e.global(USED_ITEM_FORM);
            let _has_used_item = e
                .call(
                    0x0089_1db0,
                    &args![actor, used_item, 0u32, 1u32, 0u32, found],
                )
                .bool();
            let key = e.mem.u32(lock + 4);
            let has_key = key != 0
                && e.call(0x0089_1db0, &args![actor, key, 0u32, 1u32, 0u32, found])
                    .bool();
            if has_key {
                if e.global::<u32>(UNLOCKING_CONTAINER) != 0 || actor != e.global::<u32>(PLAYER) {
                    return false;
                }
                e.set_global(UNLOCKING_CONTAINER, reference.addr());
                unlock(e, reference);
                e.with_stack(0xc, |e, handle| {
                    let audio = e.call(0x0045_3a70, &args![]).u32();
                    let sound = e
                        .call(0x00ad_7550, &args![audio, handle, UNLOCK_SOUND, 0x121u32])
                        .u32();
                    e.call(0x00ad_8830, &args![sound, 0u32]);
                    e.call(0x0048_3710, &args![handle]);
                });
                let key = e.mem.u32(lock + 4);
                let name = e.call(0x0040_8da0, &args![key + 0x30]).u32();
                let format = e.call(0x0040_3df0, &args![UNLOCKED_WITH_KEY_SETTING]).u32();
                e.with_stack(0x100, |e, text| {
                    e.call(0x0040_6d00, &args![text, 0xffu32, format, name]);
                    let time = message_time(e);
                    e.call(0x0070_52f0, &args![text, 0u32, KEY_ICON, 0u32, time, 0u32]);
                });
                let unlocking: u32 = e.global(UNLOCKING_CONTAINER);
                let player: u32 = e.global(PLAYER);
                e.call(0x0057_3170, &args![unlocking, player, 0u32, 0u32, 1u32]);
                e.set_global(UNLOCKING_CONTAINER, 0u32);
                return true;
            }
            if e.global::<u32>(UNLOCKING_CONTAINER) != 0 {
                return false;
            }
            if actor == e.global::<u32>(PLAYER) {
                let extra = e.call(0x0056_91f0, &args![reference]).u32();
                e.with_stack(4, |e, bonus| {
                    e.mem.set_f32(bonus.addr(), 0.0);
                    let player: u32 = e.global(PLAYER);
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
                } else {
                    e.call(0x0070_5830, &args![reference, 0u32, 0u32]);
                }
            } else {
                let process = e.vcall(actor, 0x1e4, &args![]).u32();
                if process != 0 {
                    e.call(0x0049_8910, &args![process, 1u32, 0u32]);
                }
                let used_item: u32 = e.global(USED_ITEM_FORM);
                e.call(0x0060_0900, &args![used_item]);
                e.call(0x0060_0940, &args![1u32]);
                e.call(0x0060_0920, &args![1u32]);
                let acquire = e.call(0x008d_8520, &args![actor]).u32();
                e.vcall(acquire, 0x44, &args![actor, 0u32, 2u32, 1u32, 0u32, 1u32]);
                e.call(0x0060_0900, &args![0u32]);
                e.call(0x0060_0940, &args![0u32]);
                e.call(0x0060_0920, &args![0xffff_ffffu32]);
                unlock(e, reference);
            }
            false
        });
    }
    if actor != e.global::<u32>(PLAYER)
        && e.call(0x008c_e390, &args![this]).u32() != 0
        && !reference.is_null()
        && e.vcall(reference.addr(), 0x1d0, &args![]).u32() != 0
    {
        let sound = e.call(0x008c_e390, &args![this]).u32();
        let sound_id = e.call(0x0084_e3a0, &args![sound]).u32();
        e.with_stack(0xc, |e, handle| {
            let audio = e.call(0x0045_3a70, &args![]).u32();
            e.call(0x00ad_73b0, &args![audio, handle, sound_id, 0x102u32]);
            let position = e.vcall(reference.addr(), 0x1f4, &args![]).u32();
            let x = e.mem.u32(position);
            let y = e.mem.u32(position + 4);
            let z = e.mem.u32(position + 8);
            e.call(
                0x00ad_8b60,
                &args![
                    handle,
                    x87_quiet_bits(x),
                    x87_quiet_bits(y),
                    x87_quiet_bits(z)
                ],
            );
            e.call(0x00ad_8830, &args![handle, 0u32]);
            e.call(0x0048_3710, &args![handle]);
        });
    }
    if actor == e.global::<u32>(PLAYER) {
        if e.call(0x0047_b250, &args![reference]).u32() == 1 {
            e.vcall(base + 0x84, 0, &args![reference, actor]);
        } else {
            e.call(0x0047_a560, &args![reference, actor, 0u32]);
        }
        return true;
    }
    if first_word != 0 {
        e.vcall(
            reference.addr(),
            0x17c,
            &args![
                first_word,
                0u32,
                second_word,
                0u32,
                0u32,
                actor,
                0u32,
                0u32,
                1u32,
                0u32
            ],
        );
    }
    if e.vcall(actor, 0x218, &args![]).bool()
        && !e.call(0x0057_85e0, &args![reference, actor, 1u32]).bool()
        && e.call(0x0056_7790, &args![reference]).u32() != 0
    {
        e.call(
            0x008b_fa40,
            &args![actor, reference, first_word, second_word, 1u32, steal_value],
        );
    }
    true
}

/// `TESForm::StartForm` (`004855a0`) and `TESForm::CloseForm` (`00485680`),
/// `thiscall` on the form, no stack words.
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
/// The component saves (engine map; `thiscall`): `TESFullName::Save`
/// (`00487010`), `TESModelTextureSwap::Save` (`0048a520`, three chunk
/// tags), `TESScriptableForm::Save` (`0048cca0`), `TESContainer::Save`
/// (`00481b40`) and `BGSDestructibleObjectForm::Save` (`00477f20`).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const MODEL_TEXTURE_SWAP_SAVE: u32 = 0x0048_a520;
const SCRIPTABLE_FORM_SAVE: u32 = 0x0048_cca0;
const CONTAINER_SAVE: u32 = 0x0048_1b40;
const DESTRUCTIBLE_SAVE: u32 = 0x0047_7f20;
/// `00401500`: the byte at `011c54ba` (whether saved words are
/// byte-swapped); `00483710` (`thiscall` on the data): its byte swap, whose
/// body does nothing.
const SAVE_SWAPS_BYTES: u32 = 0x0040_1500;
const SWAP_CONTAINER_DATA: u32 = 0x0048_3710;
/// `TESForm::__SaveData` (`00485e20`, `thiscall`, `RET 8`: data, size).
const SAVE_DATA: u32 = 0x0048_5e20;
/// `00485910` (cdecl(chunk, word)): adds a one-word chunk.
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// The form-id getter (`0084e3a0`, `thiscall`: the word at +0xc).
const FORM_ID: u32 = 0x0084_e3a0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and the type
/// descriptor `TESObjectCONT` (`0118a674`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_OBJECT_CONT: u32 = 0x0118_a674;
/// `TESForm::CopyAllComponents` (`004851b0`, `RET 4`) and
/// `TESForm::CompareAllComponents` (`00485270`, `RET 4`).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// The container's accessors (`thiscall`): the data byte's address
/// (`00461150`: `this + 0x98`), the word at +0x8c (`008ce390`) and its
/// setter (`008cc6d0`, `RET 4`), the setter of the word at +0x90
/// (`004fb0b0`, `RET 4`; `fn_00516bf0` reads it), the word at +0x94
/// (`009185e0`) and its setter (`004febf0`, `RET 4`).
const DATA_ADDRESS: u32 = 0x0046_1150;
const OPEN_SOUND: u32 = 0x008c_e390;
const SET_OPEN_SOUND: u32 = 0x008c_c6d0;
const SET_CLOSE_SOUND: u32 = 0x004f_b0b0;
const LOOPING_SOUND: u32 = 0x0091_85e0;
const SET_LOOPING_SOUND: u32 = 0x004f_ebf0;
/// `CanActivate`'s callees: `TESObjectREFR::GetLock` (`00569160`, no stack
/// words), `005021a0` (`thiscall` on the lock), `TESObjectREFR::IsAnOwner`
/// (`005785e0`, `RET 8`) and `Actor::HasObjects` (`00891db0`, `RET 0x14`).
const GET_LOCK: u32 = 0x0056_9160;
const LOCK_CHECK_005021A0: u32 = 0x0050_21a0;
const IS_AN_OWNER: u32 = 0x0057_85e0;
const HAS_OBJECTS: u32 = 0x0089_1db0;

// Translated from 00516a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCONT::Save` (engine map; no stack words): `StartForm`, the
/// full name (+0x3c), the model with texture swaps (+0x48; `MODL`, `MODT`,
/// `MODS`), script (+0x68), contents (`TESContainer::Save` on +0x30),
/// destructible data (+0x7c), the data byte at +0x98 (`__SaveData`, 1 byte;
/// `00483710` before and after when `00401500` says so), the forms at
/// +0x8c, +0x90, +0x94 by id when set (`SNAM`, `QNAM`, `RNAM`), `CloseForm`.
pub fn tes_object_cont_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![base + 0x3c]);
    e.call(
        MODEL_TEXTURE_SWAP_SAVE,
        &args![base + 0x48, MODL, MODT, MODS],
    );
    e.call(SCRIPTABLE_FORM_SAVE, &args![base + 0x68]);
    e.call(CONTAINER_SAVE, &args![base + 0x30]);
    e.call(DESTRUCTIBLE_SAVE, &args![base + 0x7c]);
    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
        e.call(SWAP_CONTAINER_DATA, &args![base + 0x98]);
    }
    e.call(SAVE_DATA, &args![this, base + 0x98, 1u32]);
    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
        e.call(SWAP_CONTAINER_DATA, &args![base + 0x98]);
    }
    for (offset, tag) in [(0x8cu32, SNAM), (0x90, QNAM), (0x94, RNAM)] {
        let form = e.mem.u32(base + offset);
        if form != 0 {
            let id = e.call(FORM_ID, &args![form]).u32();
            e.call(ADD_CHUNK_WORD, &args![tag, id]);
        }
    }
    e.call(CLOSE_FORM, &args![this]);
}

/// `other` as a `TESObjectCONT` (`__RTDynamicCast` from `TESForm`), or 0.
fn as_container(e: &mut Engine, other: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![other, 0u32, TES_FORM_TYPE, TYPE_TES_OBJECT_CONT, 0u32],
    )
    .u32()
}

// Translated from 00516b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The container's copy (engine map: no name; `RET 4`): when `other` is a
/// `TESObjectCONT`, copies all components (`TESForm::CopyAllComponents`
/// with `other`), the data byte (+0x98) and the three sounds (+0x8c, +0x90,
/// +0x94) through their getters and setters.
pub fn fn_00516b60(e: &mut Engine, this: Ptr, other: u32) {
    let source = as_container(e, other);
    if source == 0 {
        return;
    }
    e.call(COPY_ALL_COMPONENTS, &args![this, other]);
    let data = e.call(DATA_ADDRESS, &args![source]).u32();
    let byte = e.mem.u8(data);
    e.mem.set_u8(this.addr() + 0x98, byte);
    let open = e.call(OPEN_SOUND, &args![source]).u32();
    e.call(SET_OPEN_SOUND, &args![this, open]);
    let close = fn_00516bf0(e, Ptr::new(source));
    e.call(SET_CLOSE_SOUND, &args![this, close]);
    let looping = e.call(LOOPING_SOUND, &args![source]).u32();
    e.call(SET_LOOPING_SOUND, &args![this, looping]);
}

// Translated from 00516c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The container's compare (engine map: no name; `RET 4`): true when
/// `other` is no `TESObjectCONT`, when `TESForm::CompareAllComponents`
/// (with the cast form) says they differ, or when the data bytes (signed)
/// or any of the three sounds differ (this one's asked first).
pub fn fn_00516c10(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let source = as_container(e, other);
    if source == 0 {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, source]).u32() as u8 != 0 {
        return true;
    }
    let mine = e.mem.i8(this.addr() + 0x98);
    let data = e.call(DATA_ADDRESS, &args![source]).u32();
    if mine != e.mem.i8(data) {
        return true;
    }
    let mine = e.call(OPEN_SOUND, &args![this]).u32();
    if mine != e.call(OPEN_SOUND, &args![source]).u32() {
        return true;
    }
    let mine = fn_00516bf0(e, this);
    if mine != fn_00516bf0(e, Ptr::new(source)) {
        return true;
    }
    let mine = e.call(LOOPING_SOUND, &args![this]).u32();
    mine != e.call(LOOPING_SOUND, &args![source]).u32()
}

// Translated from 00516cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCONT::CanActivate` (engine map; `RET 0xc`: the reference, the
/// actor and a byte; `this` is not read): false without an actor or when
/// its vtable slot `+0x100` answers false. True when the reference has no
/// lock (`TESObjectREFR::GetLock`) or `005021a0` says the lock is open;
/// when the actor is not the player and owns the reference
/// (`TESObjectREFR::IsAnOwner(actor, 1)`); when the actor holds the lock's
/// key (the lock's word +4, when set; `Actor::HasObjects(key, 0, 1, 0,
/// &count)`); or, with `use_item` set, when it holds the form at `011ca268`.
/// Else false. The count the game keeps on its stack is a block here.
pub fn tes_object_cont_can_activate(
    e: &mut Engine,
    _this: Ptr,
    reference: u32,
    actor: u32,
    use_item: u8,
) -> bool {
    if actor == 0 || e.vcall(actor, 0x100, &[]).u32() as u8 == 0 {
        return false;
    }
    let lock = e.call(GET_LOCK, &args![reference]).u32();
    if lock == 0 || e.call(LOCK_CHECK_005021A0, &args![lock]).u32() as u8 == 0 {
        return true;
    }
    if actor != e.global::<u32>(PLAYER)
        && e.call(IS_AN_OWNER, &args![reference, actor, 1u32]).u32() as u8 != 0
    {
        return true;
    }
    e.with_stack(4, |e, count| {
        let key = e.mem.u32(lock + 4);
        if key != 0
            && e.call(HAS_OBJECTS, &args![actor, key, 0u32, 1u32, 0u32, count])
                .u32() as u8
                != 0
        {
            return true;
        }
        if use_item != 0 {
            let item = e.global::<u32>(USED_ITEM_FORM);
            if e.call(HAS_OBJECTS, &args![actor, item, 0u32, 1u32, 0u32, count])
                .u32() as u8
                != 0
            {
                return true;
            }
        }
        false
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_60f0, tesobjectcont_tesobjectcont(Ptr) -> Ptr),
        entry!(0x0051_6330, fn_00516330(Ptr)),
        entry!(0x0051_6580, fn_00516580(Ptr, Ptr) -> bool),
        entry!(0x0051_6210, fn_00516210(Ptr, u32) -> Ptr),
        entry!(0x0051_6bf0, fn_00516bf0(Ptr) -> u32),
        entry!(0x0051_7360, fn_00517360(Ptr) -> u32),
        entry!(
            0x0051_7380,
            tes_object_cont_handle_open_finish(Ptr, Ptr, Ptr)
        ),
        entry!(0x0051_6240, fn_00516240(Ptr)),
        entry!(0x0051_6370, tes_object_cont_init_item(Ptr)),
        entry!(0x0051_6dc0, fn_00516dc0(Ptr, Ptr, Ptr, u32, u32, u32) -> bool),
        entry!(0x0051_6a20, tes_object_cont_save(Ptr)),
        entry!(0x0051_6b60, fn_00516b60(Ptr, u32)),
        entry!(0x0051_6c10, fn_00516c10(Ptr, u32) -> bool),
        entry!(
            0x0051_6cd0,
            tes_object_cont_can_activate(Ptr, u32, u32, u8) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn tesobjectcont_tesobjectcont_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_60f0,
            size: 0x9c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_f6f0, &[A::T(0x0)]),
                (0x0048_1610, &[A::T(0x30)]),
                (0x0040_2d00, &[A::T(0x3c)]),
                (0x0048_a3d0, &[A::T(0x48)]),
                (0x0048_cc50, &[A::T(0x68)]),
                (0x0048_eac0, &[A::T(0x74)]),
                (0x0047_5350, &[A::T(0x7c)]),
                (0x0051_0620, &[A::T(0x84)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x1b)]),
                (0x0051_6330, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_aeb4),
                (0x30, 4, 0x0102_ae9c),
                (0x3c, 4, 0x0102_ae88),
                (0x48, 4, 0x0102_ae64),
                (0x68, 4, 0x0102_ae50),
                (0x74, 4, 0x0102_ae3c),
                (0x7c, 4, 0x0102_ae28),
                (0x84, 4, 0x0102_ae18),
            ],
        });
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::fallout_shared::tesmodel::load_test::{
        self, calls_to, run, stub, tag, CHUNK_SIZE, CHUNK_VALUE, FILE,
    };

    const PENDING: u32 = 0x011c_9348;

    type Loaded = (Engine, u32, bool, load_test::Calls);

    /// Runs the loader on a container form and a file holding `chunks`. The form's virtual
    /// slots `+0xe0` and `+0x134` are the doubles `0x0090_0500` and `0x0090_0600`.
    fn load(form_type: u32, chunks: &[u32], version_flag: bool, pending: u32) -> Loaded {
        let mut e = load_test::engine(form_type, chunks, version_flag);
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
                0x008c_c6d0,
                0x004f_b0b0,
                0x004f_ebf0,
                0x0048_61f0,
                0x0040_3d30,
                0x0046_2230,
                0x0040_e780,
            ],
        );
        e.register(0x0048_17f0, |_, _| Ret {
            eax: 0x0777_0001,
            ..Ret::default()
        });
        e.register(0x0040_1000, |_, _| Ret {
            eax: 0x0b10_c000,
            ..Ret::default()
        });
        e.register(0x0040_e690, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.map(PENDING, 4);
        e.set_global(PENDING, pending);
        let this = load_test::object(
            &mut e,
            0xa0,
            0x0900_0000,
            &[(0xe0, 0x0090_0500), (0x134, 0x0090_0600)],
        );
        let (ret, calls) = run(&mut e, 0x0051_6580, &[this, FILE]);
        (e, this, ret.bool(), calls)
    }

    fn container(chunks: &[u32]) -> Loaded {
        load(0x1b, chunks, false, 0)
    }

    #[test]
    fn fn_00516330_clears_the_start_values() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa0);
        e.mem.write(this, &[0xcd; 0xa0]);
        e.call(0x0051_6330, &args![Ptr::<()>::new(this)]);
        let mut want = vec![0xcd_u8; 0xa0];
        want[0x8c..0x98].fill(0);
        want[0x98] = 0;
        assert_eq!(e.mem.bytes(this, 0xa0), want);
    }

    #[test]
    fn another_form_type_is_refused_before_anything_is_loaded() {
        let (_, _, answer, calls) = load(0x1c, &[tag(b"EDID")], false, 0);
        assert!(!answer);
        assert!(calls.is_empty());
    }

    #[test]
    fn the_form_is_loaded_first_and_the_flag_bit_cleared() {
        let (_, this, answer, calls) = container(&[]);
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
        let (_, this, _, calls) = container(&[tag(b"EDID")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, 0x200));
        assert_eq!(calls_to(&calls, 0x0090_0600), [vec![this, read[0][1]]]);

        let (_, this, _, calls) = container(&[tag(b"OBND")]);
        assert_eq!(calls_to(&calls, 0x0090_0500), [vec![this, FILE]]);

        let (_, this, _, calls) = container(&[tag(b"MODS")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, CHUNK_SIZE));
        assert_eq!(
            calls_to(&calls, 0x0048_a7b0),
            [vec![this + 0x48, read[0][1], CHUNK_SIZE, 0x1234]]
        );
    }

    #[test]
    fn the_components_get_their_chunks() {
        for (chunk, callee, offset) in [
            (tag(b"FULL"), 0x0048_7050, 0x3c),
            (tag(b"MODL"), 0x0048_92d0, 0x48),
            (tag(b"MODD"), 0x0048_92d0, 0x48),
            (tag(b"MOSD"), 0x0048_92d0, 0x48),
            (tag(b"MODT"), 0x0048_92d0, 0x48),
            (tag(b"DSTD"), 0x0047_81e0, 0x7c),
            (tag(b"DEST"), 0x0047_81e0, 0x7c),
        ] {
            let (_, this, _, calls) = container(&[chunk]);
            assert_eq!(
                calls_to(&calls, callee),
                [vec![this + offset, FILE]],
                "{chunk:08x}"
            );
        }
    }

    #[test]
    fn the_word_chunks_reach_their_setters() {
        let (_, this, _, calls) = container(&[tag(b"SCRI")]);
        assert_eq!(
            calls_to(&calls, 0x006e_cd40),
            [vec![this + 0x68, CHUNK_VALUE]]
        );
        assert_eq!(calls_to(&calls, 0x0048_cd90), [vec![this + 0x68, this]]);
        for (chunk, callee) in [
            (tag(b"SNAM"), 0x008c_c6d0),
            (tag(b"QNAM"), 0x004f_b0b0),
            (tag(b"RNAM"), 0x004f_ebf0),
        ] {
            let (_, this, _, calls) = container(&[chunk]);
            assert_eq!(calls_to(&calls, callee), [vec![this, CHUNK_VALUE]]);
        }
        let (_, this, _, calls) = container(&[tag(b"DATA")]);
        assert_eq!(
            calls_to(&calls, 0x0048_61f0),
            [vec![this, FILE, this + 0x98, 1]]
        );
    }

    #[test]
    fn a_container_item_chunk_remembers_the_entry_and_its_extra_data_chunk_loads_it() {
        // CNTO: the 8 byte local is cleared, read, fixed up and handed to 004817f0.
        let (e, this, _, calls) = load(0x1b, &[tag(b"CNTO")], true, 0);
        let read = calls_to(&calls, 0x0047_2890);
        let local = read[0][1];
        assert_eq!((read[0][0], read[0][2]), (FILE, 8));
        assert_eq!(calls_to(&calls, 0x0040_3d30), [vec![local, 0, 8]]);
        assert_eq!(calls_to(&calls, 0x0046_2230), [vec![local]]);
        assert_eq!(calls_to(&calls, 0x0048_17f0), [vec![this + 0x30, local]]);
        assert_eq!(e.global::<u32>(PENDING), 0x0777_0001);

        // Without the flag the fix-up is skipped.
        let (_, _, _, calls) = load(0x1b, &[tag(b"CNTO")], false, 0);
        assert!(calls_to(&calls, 0x0046_2230).is_empty());

        // COED with an entry waiting: a new extra-data object is stored in it and loaded.
        let mut e = load_test::engine(0x1b, &[tag(b"COED")], false);
        stub(&mut e, &[0x0048_5110, 0x0048_4ab0, 0x0040_e780]);
        e.register(0x0040_1000, |_, _| Ret {
            eax: 0x0b10_c000,
            ..Ret::default()
        });
        e.register(0x0040_e690, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        let entry = e.mem.alloc(0x10);
        e.map(PENDING, 4);
        e.set_global(PENDING, entry);
        let form = e.mem.alloc(0xa0);
        let (_, calls) = run(&mut e, 0x0051_6580, &[form, FILE]);
        assert_eq!(calls_to(&calls, 0x0040_1000), [vec![0xc]]);
        assert_eq!(calls_to(&calls, 0x0040_e690), [vec![0x0b10_c000]]);
        assert_eq!(e.mem.u32(entry + 8), 0x0b10_c001);
        assert_eq!(calls_to(&calls, 0x0040_e780), [vec![0x0b10_c001, FILE]]);
        assert_eq!(e.global::<u32>(PENDING), 0);

        // COED with nothing waiting does nothing.
        let (_, _, _, calls) = load(0x1b, &[tag(b"COED")], false, 0);
        assert!(calls_to(&calls, 0x0040_1000).is_empty());
        assert!(calls_to(&calls, 0x0040_e780).is_empty());
    }
    #[test]
    fn at_00516210_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_6240, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_6210, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_6210)
                .collect();
            let mut expected = vec![(0x0051_6240, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod container_tests {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn fn_00516bf0_reads_the_second_sound() {
        let mut e = Engine::new();
        let container = e.mem.alloc(0x98);
        e.mem.set_u32(container + 0x90, 0x4321);
        assert_eq!(e.call(0x0051_6bf0, &args![container]).u32(), 0x4321);
    }

    #[test]
    fn fn_00517360_unlocks_the_lock() {
        let mut e = Engine::new();
        let extra = e.mem.alloc(0x10);
        e.mem.set_u32(extra + 0xc, 0x6000);
        e.register(0x0043_0a90, |_, _| Ret {
            eax: 0x55,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0051_7360, &args![extra]).u32(), 0x55);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0043_0a90), vec![vec![0x6000, 0]]);
    }

    #[test]
    fn handle_open_finish_queues_the_menu_for_the_player_only() {
        for (reference, actor, runs) in [
            (0u32, 0x7000u32, false),
            (0x5000, 0x7100, false),
            (0x5000, 0x7000, true),
        ] {
            let mut e = Engine::new();
            e.map(PLAYER, 4);
            e.set_global(PLAYER, 0x7000u32);
            e.register(0x0070_9470, |_, _| Ret::default());
            e.register(0x0047_a560, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x0051_7380, &args![0x4000u32, reference, actor]);
            let log = e.call_log.take().unwrap();
            if runs {
                assert_eq!(
                    log[1..],
                    [
                        (0x0070_9470, vec![1, reference, 0, 0, 1, 0]),
                        (0x0047_a560, vec![reference, actor, 0]),
                    ]
                );
            } else {
                assert_eq!(log.len(), 1);
            }
        }
    }

    #[test]
    fn fn_00516240_stores_the_vtables_and_runs_the_component_destructors() {
        let mut e = Engine::new();
        let container = e.mem.alloc(0x98);
        for address in [
            0x0047_5380u32,
            0x0048_eaf0,
            0x0048_a490,
            0x0059_1300,
            0x0048_1680,
            0x0050_f740,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0051_6240, &args![container]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0047_5380, vec![container + 0x7c]),
                (0x0048_eaf0, vec![container + 0x74]),
                (0x0048_a490, vec![container + 0x48]),
                (0x0059_1300, vec![container + 0x3c]),
                (0x0048_1680, vec![container + 0x30]),
                (0x0050_f740, vec![container]),
            ]
        );
        assert_eq!(e.mem.u32(container), 0x0102_aeb4);
        assert_eq!(e.mem.u32(container + 0x84), 0x0102_ae18);
        assert_eq!(e.mem.u32(container + 0x48), 0x0102_ae64);
    }

    #[test]
    fn cont_init_item_runs_the_components_and_resolves_the_sounds() {
        let mut e = Engine::new();
        let container = e.mem.alloc(0x98);
        e.put_vtable(0x0300_0000, &[0; 0x50]);
        e.mem.set_u32(0x0300_0000 + 0x130, 0x0300_1130);
        e.mem.set_u32(container, 0x0300_0000);
        e.mem.set_u32(container + 0x8c, 0x10);
        e.mem.set_u32(container + 0x90, 0x3000);
        e.mem.set_u32(container + 0x94, 0x3100);
        e.register(0x0300_1130, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        e.register(0x0040_13e0, |_, _| Ret::default());
        e.register(0x0048_4e60, |_, _| Ret {
            eax: 0x9000,
            ..Ret::default()
        });
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 1);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x00ec_43fb, |_, a| Ret {
            eax: if a[0] < 0x2000 { a[0] + 0x100 } else { 0 },
            ..Ret::default()
        });
        e.register(0x0084_e3a0, |_, _| Ret {
            eax: 0xbeef,
            ..Ret::default()
        });
        for address in [
            0x0048_cd90u32,
            0x0048_1bb0,
            0x0047_8be0,
            0x0048_aa80,
            0x005b_5e40,
            0x0048_4ab0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0051_6370, &args![container]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[2..6],
            [
                (0x0048_cd90, vec![container + 0x68, container]),
                (0x0048_1bb0, vec![container + 0x30, container]),
                (0x0047_8be0, vec![container + 0x7c, container]),
                (0x0048_aa80, vec![container + 0x48, container]),
            ]
        );
        assert_eq!(e.mem.u32(container + 0x8c), 0x111);
        assert_eq!(e.mem.u32(container + 0x90), 0);
        assert_eq!(e.mem.u32(container + 0x94), 0);
        assert_eq!(
            calls_to(&log, 0x005b_5e40),
            vec![
                vec![MISSING_OPEN_SOUND, 0x3001, 0x7777, 0xbeef],
                vec![MISSING_LOOPING_SOUND, 0x3101, 0x7777, 0xbeef],
            ]
        );
        assert_eq!(log.last().unwrap(), &(0x0048_4ab0, vec![container, 1]));
    }

    /// The answers the activation's doubles give.
    #[derive(Clone, Copy)]
    struct Activation {
        actor: u32,
        owns: bool,
        lock: u32,
        lock_active: bool,
        key: u32,
        has_key: bool,
        unlocking: u32,
        extra: u32,
        lock_level: u32,
        broken: bool,
        open_sound: u32,
        position_check: u32,
        open_state: u32,
        process: u32,
        stealing: bool,
        owner: u32,
    }

    const PLAYER_REF: u32 = 0x0007_0000;
    const NPC_REF: u32 = 0x0007_1000;
    const REFERENCE: u32 = 0x0005_0000;

    fn activation() -> Activation {
        Activation {
            actor: PLAYER_REF,
            owns: false,
            lock: 0,
            lock_active: false,
            key: 0,
            has_key: false,
            unlocking: 0,
            extra: 0,
            lock_level: 0,
            broken: false,
            open_sound: 0,
            position_check: 0,
            open_state: 0,
            process: 0,
            stealing: false,
            owner: 0,
        }
    }

    /// Runs the activation with `case`'s doubles; the activating reference is `case.actor`
    /// (the cast passes it through). Returns the answer and the calls.
    fn activate(
        case: Activation,
        first_word: u32,
        second_word: u32,
    ) -> (bool, Vec<(u32, Vec<u32>)>, Engine) {
        let mut e = Engine::new();
        e.map(PLAYER, 4);
        e.set_global(PLAYER, PLAYER_REF);
        e.map(USED_ITEM_FORM, 4);
        e.set_global(USED_ITEM_FORM, 0x0008_0000u32);
        e.map(UNLOCKING_CONTAINER, 4);
        e.set_global(UNLOCKING_CONTAINER, case.unlocking);
        e.map(MESSAGE_TIME, 4);
        e.set_global(MESSAGE_TIME, 2.0f32);
        let container = e.mem.alloc(0x98);
        // The actor's and reference's vtables.
        e.map(case.actor & !0xfff, 0x1000);
        e.put_vtable(0x0300_0000, &[0; 0x90]);
        for (slot, target) in [(0x1e4u32, 0x0300_11e4u32), (0x218, 0x0300_1218)] {
            e.mem.set_u32(0x0300_0000 + slot, target);
        }
        e.mem.set_u32(case.actor, 0x0300_0000);
        e.map(REFERENCE, 0x100);
        e.put_vtable(0x0300_0400, &[0; 0x80]);
        for (slot, target) in [
            (0x17cu32, 0x0300_117c),
            (0x1d0, 0x0300_11d0),
            (0x1f4, 0x0300_11f4),
        ] {
            e.mem.set_u32(0x0300_0400 + slot, target);
        }
        e.mem.set_u32(REFERENCE, 0x0300_0400);
        e.mem.set_u32(REFERENCE + 0x40, 1.0f32.to_bits());
        e.mem.set_u32(REFERENCE + 0x44, 2.0f32.to_bits());
        e.mem.set_u32(REFERENCE + 0x48, 3.0f32.to_bits());
        // The open-close component at +0x84.
        e.put_vtable(0x0300_0800, &[0x0300_1084]);
        e.mem.set_u32(container + 0x84, 0x0300_0800);
        if case.lock != 0 {
            e.map(case.lock, 0x10);
            e.mem.set_u32(case.lock + 4, case.key);
        }
        if case.extra != 0 {
            e.map(case.extra, 0x10);
            e.mem.set_u32(case.extra + 0xc, 0x0006_8000);
        }
        let ret = |eax: u32| Ret {
            eax,
            ..Ret::default()
        };
        e.register(0x00ec_43fb, move |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register_double(0x0057_85e0, move |_, _| ret(u32::from(case.owns)));
        e.register_double(0x0056_9160, move |_, _| ret(case.lock));
        e.register_double(0x0050_21a0, move |_, _| ret(u32::from(case.lock_active)));
        e.register_double(0x0089_1db0, move |_, a| {
            ret(u32::from(a[1] == case.key && case.has_key))
        });
        e.register_double(0x0056_91f0, move |_, _| ret(case.extra));
        e.register_double(0x0043_09e0, move |_, _| ret(case.lock_level));
        e.register_double(0x0043_0ae0, move |_, _| ret(u32::from(case.broken)));
        e.register_double(0x008c_e390, move |_, _| ret(case.open_sound));
        e.register_double(0x0300_11d0, move |_, _| ret(case.position_check));
        e.register(0x0300_11f4, |_, a| Ret {
            eax: a[0] + 0x40,
            ..Ret::default()
        });
        e.register_double(0x0047_b250, move |_, _| ret(case.open_state));
        e.register_double(0x0300_11e4, move |_, _| ret(case.process));
        e.register_double(0x0300_1218, move |_, _| ret(u32::from(case.stealing)));
        e.register_double(0x0056_7790, move |_, _| ret(case.owner));
        e.register(0x0045_3a70, |_, _| Ret {
            eax: 0x0009_0000,
            ..Ret::default()
        });
        e.register(0x00ad_7550, |_, _| Ret {
            eax: 0x0009_1000,
            ..Ret::default()
        });
        e.register(0x0040_8da0, |_, _| Ret {
            eax: 0x0009_2000,
            ..Ret::default()
        });
        e.register(0x0040_3df0, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(0x008d_8520, |_, _| Ret {
            eax: 0x0009_3000,
            ..Ret::default()
        });
        e.map(0x0009_3000, 4);
        e.put_vtable(0x0300_0c00, &[0; 0x12]);
        e.mem.set_u32(0x0300_0c00 + 0x44, 0x0300_1c44);
        e.mem.set_u32(0x0009_3000, 0x0300_0c00);
        e.register(0x0084_e3a0, |_, a| Ret {
            eax: a[0] + 2,
            ..Ret::default()
        });
        for address in [
            0x0043_0a90u32,
            0x0057_89b0,
            0x00ad_8830,
            0x0048_3710,
            0x0040_6d00,
            0x0070_52f0,
            0x0057_3170,
            0x005e_58f0,
            0x0070_5830,
            0x0049_8910,
            0x0060_0900,
            0x0060_0940,
            0x0060_0920,
            0x0300_1c44,
            0x00ad_73b0,
            0x00ad_8b60,
            0x0300_1084,
            0x0047_a560,
            0x0300_117c,
            0x008b_fa40,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let answer = e
            .call(
                0x0051_6dc0,
                &args![
                    container,
                    REFERENCE,
                    case.actor,
                    0xdeadu32,
                    first_word,
                    second_word
                ],
            )
            .bool();
        let log = e.call_log.take().unwrap();
        (answer, log, e)
    }

    #[test]
    fn activation_needs_an_actor() {
        let mut e = Engine::new();
        e.register(0x00ec_43fb, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let answer = e
            .call(
                0x0051_6dc0,
                &args![0x4000u32, REFERENCE, 0x6000u32, 0u32, 0u32, 0u32],
            )
            .bool();
        assert!(!answer);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x00ec_43fb),
            vec![vec![0x6000, 0, OBJECT_REFR_TYPE, ACTOR_TYPE, 0]]
        );
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn activation_with_the_key_unlocks_and_activates_for_the_player() {
        let case = Activation {
            lock: 0x0006_0000,
            lock_active: true,
            key: 0x0006_1000,
            has_key: true,
            extra: 0x0006_2000,
            ..activation()
        };
        let (answer, log, e) = activate(case, 0, 0);
        assert!(answer);
        // The player is not asked about ownership.
        assert!(calls_to(&log, 0x0057_85e0).is_empty());
        assert_eq!(calls_to(&log, 0x0089_1db0).len(), 2);
        assert_eq!(calls_to(&log, 0x0089_1db0)[0][1], 0x0008_0000);
        assert_eq!(calls_to(&log, 0x0043_0a90), vec![vec![0x0006_8000, 0]]);
        assert_eq!(calls_to(&log, 0x0057_89b0), vec![vec![REFERENCE]]);
        let sound = calls_to(&log, 0x00ad_7550);
        assert_eq!(sound[0][0], 0x0009_0000);
        assert_eq!(sound[0][2..], [UNLOCK_SOUND, 0x121]);
        assert_eq!(calls_to(&log, 0x00ad_8830), vec![vec![0x0009_1000, 0]]);
        assert_eq!(calls_to(&log, 0x0040_8da0), vec![vec![0x0006_1030]]);
        let text = calls_to(&log, 0x0040_6d00);
        assert_eq!(
            text[0][1..],
            [0xff, UNLOCKED_WITH_KEY_SETTING + 1, 0x0009_2000]
        );
        let message = calls_to(&log, 0x0070_52f0);
        assert_eq!(message[0][0], text[0][0]);
        assert_eq!(message[0][1..], [0, KEY_ICON, 0, 2.0f32.to_bits(), 0]);
        assert_eq!(
            calls_to(&log, 0x0057_3170),
            vec![vec![REFERENCE, PLAYER_REF, 0, 0, 1]]
        );
        assert_eq!(e.global::<u32>(UNLOCKING_CONTAINER), 0);
    }

    #[test]
    fn activation_with_the_key_does_nothing_while_unlocking_or_for_an_npc() {
        for (actor, unlocking) in [(PLAYER_REF, 0x0005_5000u32), (NPC_REF, 0)] {
            let case = Activation {
                actor,
                lock: 0x0006_0000,
                lock_active: true,
                key: 0x0006_1000,
                has_key: true,
                unlocking,
                ..activation()
            };
            let (answer, log, _) = activate(case, 0, 0);
            assert!(!answer);
            assert!(calls_to(&log, 0x0057_3170).is_empty());
            assert!(calls_to(&log, 0x0070_52f0).is_empty());
        }
    }

    #[test]
    fn activation_of_an_owned_locked_container_goes_the_unlocked_way() {
        let case = Activation {
            actor: NPC_REF,
            owns: true,
            lock: 0x0006_0000,
            lock_active: true,
            ..activation()
        };
        let (answer, log, _) = activate(case, 0, 0);
        assert!(answer);
        assert!(calls_to(&log, 0x0089_1db0).is_empty());
        assert_eq!(calls_to(&log, 0x0057_85e0)[0], vec![REFERENCE, NPC_REF, 1]);
    }

    #[test]
    fn locked_without_a_key_the_player_picks_or_is_refused() {
        // (lock level, broken, extra data, refused)
        for (level, broken, extra, refused) in [
            (2u32, false, 0x0006_2000u32, false),
            (5, false, 0x0006_2000, true),
            (2, true, 0x0006_2000, true),
            (5, true, 0, false),
        ] {
            let case = Activation {
                lock: 0x0006_0000,
                lock_active: true,
                key: 0x0006_1000,
                has_key: false,
                extra,
                lock_level: level,
                broken,
                ..activation()
            };
            let (answer, log, _) = activate(case, 0, 0);
            assert!(!answer);
            let entry = calls_to(&log, 0x005e_58f0);
            assert_eq!(entry[0][..2], [0x20, PLAYER_REF]);
            if refused {
                assert_eq!(
                    calls_to(&log, 0x0070_52f0),
                    vec![vec![
                        CANNOT_PICK_SETTING + 1,
                        0,
                        PADLOCK_ICON,
                        0,
                        2.0f32.to_bits(),
                        0
                    ]]
                );
                assert!(calls_to(&log, 0x0070_5830).is_empty());
            } else {
                assert_eq!(calls_to(&log, 0x0070_5830), vec![vec![REFERENCE, 0, 0]]);
            }
            if extra != 0 {
                assert_eq!(
                    calls_to(&log, 0x0043_09e0),
                    vec![vec![0x0006_8000, REFERENCE]]
                );
            }
        }
    }

    #[test]
    fn locked_without_a_key_an_npc_uses_the_lock_and_unlocks_it() {
        for process in [0u32, 0x0009_4000] {
            let case = Activation {
                actor: NPC_REF,
                lock: 0x0006_0000,
                lock_active: true,
                extra: 0x0006_2000,
                process,
                ..activation()
            };
            let (answer, log, _) = activate(case, 0, 0);
            assert!(!answer);
            assert_eq!(
                calls_to(&log, 0x0049_8910),
                if process == 0 {
                    vec![]
                } else {
                    vec![vec![process, 1, 0]]
                }
            );
            assert_eq!(
                calls_to(&log, 0x0060_0900),
                vec![vec![0x0008_0000], vec![0]]
            );
            assert_eq!(calls_to(&log, 0x0060_0940), vec![vec![1], vec![0]]);
            assert_eq!(
                calls_to(&log, 0x0060_0920),
                vec![vec![1], vec![0xffff_ffff]]
            );
            assert_eq!(
                calls_to(&log, 0x0300_1c44),
                vec![vec![0x0009_3000, NPC_REF, 0, 2, 1, 0, 1]]
            );
            assert_eq!(calls_to(&log, 0x0043_0a90), vec![vec![0x0006_8000, 0]]);
        }
    }

    #[test]
    fn locked_without_a_key_while_unlocking_does_nothing_more() {
        let case = Activation {
            lock: 0x0006_0000,
            lock_active: true,
            unlocking: 0x0005_5000,
            ..activation()
        };
        let (answer, log, _) = activate(case, 0, 0);
        assert!(!answer);
        assert!(calls_to(&log, 0x005e_58f0).is_empty());
        assert!(calls_to(&log, 0x0060_0900).is_empty());
    }

    #[test]
    fn unlocked_the_player_opens_or_closes_the_container() {
        for (state, through_component) in [(1u32, true), (0, false), (2, false)] {
            let case = Activation {
                open_state: state,
                open_sound: 0x0009_5000,
                position_check: 1,
                ..activation()
            };
            let (answer, log, _) = activate(case, 0, 0);
            assert!(answer);
            // The player hears no open sound here.
            assert!(calls_to(&log, 0x00ad_73b0).is_empty());
            if through_component {
                assert!(!calls_to(&log, 0x0300_1084).is_empty());
                assert!(calls_to(&log, 0x0047_a560).is_empty());
            } else {
                assert_eq!(
                    calls_to(&log, 0x0047_a560),
                    vec![vec![REFERENCE, PLAYER_REF, 0]]
                );
            }
        }
    }

    #[test]
    fn unlocked_an_npc_plays_the_open_sound_and_may_raise_the_alarm() {
        let case = Activation {
            actor: NPC_REF,
            open_sound: 0x0009_5000,
            position_check: 1,
            stealing: true,
            owner: 0x0009_6000,
            ..activation()
        };
        let (answer, log, _) = activate(case, 0x0abc, 3);
        assert!(answer);
        let handle = calls_to(&log, 0x00ad_73b0);
        assert_eq!(handle[0][0], 0x0009_0000);
        assert_eq!(handle[0][2..], [0x0009_5002, 0x102]);
        assert_eq!(
            calls_to(&log, 0x00ad_8b60),
            vec![vec![
                handle[0][1],
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits()
            ]]
        );
        assert_eq!(calls_to(&log, 0x00ad_8830), vec![vec![handle[0][1], 0]]);
        assert_eq!(
            calls_to(&log, 0x0300_117c),
            vec![vec![REFERENCE, 0x0abc, 0, 3, 0, 0, NPC_REF, 0, 0, 1, 0]]
        );
        assert_eq!(
            calls_to(&log, 0x008b_fa40),
            vec![vec![NPC_REF, REFERENCE, 0x0abc, 3, 1, 0]]
        );
    }

    #[test]
    fn unlocked_an_npc_without_a_sound_word_or_owner_raises_nothing() {
        for (stealing, owns, owner) in [
            (false, false, 0x0009_6000u32),
            (true, true, 0x0009_6000),
            (true, false, 0),
        ] {
            let case = Activation {
                actor: NPC_REF,
                stealing,
                owns,
                owner,
                ..activation()
            };
            let (answer, log, _) = activate(case, 0, 0);
            assert!(answer);
            assert!(calls_to(&log, 0x0300_117c).is_empty());
            assert!(calls_to(&log, 0x008b_fa40).is_empty());
            assert!(calls_to(&log, 0x00ad_73b0).is_empty());
        }
    }
}

#[cfg(test)]
mod save_copy_activate_tests {
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

    #[test]
    fn save_writes_components_data_and_sounds() {
        let mut e = Engine::new();
        for a in [
            START_FORM,
            CLOSE_FORM,
            FULL_NAME_SAVE,
            MODEL_TEXTURE_SWAP_SAVE,
            SCRIPTABLE_FORM_SAVE,
            CONTAINER_SAVE,
            DESTRUCTIBLE_SAVE,
            SWAP_CONTAINER_DATA,
            SAVE_DATA,
            ADD_CHUNK_WORD,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.register(SAVE_SWAPS_BYTES, |_, _| ret(0));
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        let this = e.mem.alloc(0xa0);
        let sound = e.mem.alloc(0x18);
        e.mem.set_u32(sound + 0xc, 0x0000_0456);
        e.mem.set_u32(this + 0x90, sound);
        let (_, log) = run(&mut e, 0x0051_6a20, &args![this]);
        assert_eq!(only(&log, FULL_NAME_SAVE), vec![vec![this + 0x3c]]);
        assert_eq!(
            only(&log, MODEL_TEXTURE_SWAP_SAVE),
            vec![vec![this + 0x48, MODL, MODT, MODS]]
        );
        assert_eq!(only(&log, SCRIPTABLE_FORM_SAVE), vec![vec![this + 0x68]]);
        assert_eq!(only(&log, CONTAINER_SAVE), vec![vec![this + 0x30]]);
        assert_eq!(only(&log, DESTRUCTIBLE_SAVE), vec![vec![this + 0x7c]]);
        assert_eq!(only(&log, SAVE_DATA), vec![vec![this, this + 0x98, 1]]);
        assert!(only(&log, SWAP_CONTAINER_DATA).is_empty());
        assert_eq!(only(&log, ADD_CHUNK_WORD), vec![vec![QNAM, 0x456]]);
        e.register(SAVE_SWAPS_BYTES, |_, _| ret(1));
        e.mem.set_u32(this + 0x8c, sound);
        e.mem.set_u32(this + 0x94, sound);
        let (_, log) = run(&mut e, 0x0051_6a20, &args![this]);
        assert_eq!(only(&log, SWAP_CONTAINER_DATA).len(), 2);
        assert_eq!(
            only(&log, ADD_CHUNK_WORD),
            vec![vec![SNAM, 0x456], vec![QNAM, 0x456], vec![RNAM, 0x456]]
        );
    }

    fn accessor_engine() -> Engine {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(0));
        e.register(DATA_ADDRESS, |_, a| ret(a[0] + 0x98));
        for (get, set, offset) in [
            (OPEN_SOUND, SET_OPEN_SOUND, 0x8c),
            (0, SET_CLOSE_SOUND, 0x90),
            (LOOPING_SOUND, SET_LOOPING_SOUND, 0x94),
        ] {
            if get != 0 {
                e.register_double(get, move |e, a| ret(e.mem.u32(a[0] + offset)));
            }
            e.register_double(set, move |e, a| {
                e.mem.set_u32(a[0] + offset, a[1]);
                Ret::default()
            });
        }
        e
    }

    #[test]
    fn copy_takes_the_data_byte_and_sounds() {
        let mut e = accessor_engine();
        let this = e.mem.alloc(0xa0);
        let other = e.mem.alloc(0xa0);
        e.mem.set_u8(other + 0x98, 0x81);
        for (i, offset) in [0x8cu32, 0x90, 0x94].iter().enumerate() {
            e.mem.set_u32(other + offset, 0x700 + i as u32);
        }
        let (_, log) = run(&mut e, 0x0051_6b60, &args![this, other]);
        assert_eq!(only(&log, COPY_ALL_COMPONENTS), vec![vec![this, other]]);
        assert_eq!(e.mem.u8(this + 0x98), 0x81);
        assert_eq!(
            [0x8c, 0x90, 0x94].map(|o| e.mem.u32(this + o)),
            [0x700, 0x701, 0x702]
        );
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        let fresh = e.mem.alloc(0xa0);
        e.call(0x0051_6b60, &args![fresh, other]);
        assert_eq!(e.mem.u8(fresh + 0x98), 0);
    }

    #[test]
    fn compare_reports_each_difference() {
        let mut e = accessor_engine();
        let pair = |e: &mut Engine, offset: u32| {
            let this = e.mem.alloc(0xa0);
            let other = e.mem.alloc(0xa0);
            for o in [0x8cu32, 0x90, 0x94] {
                e.mem.set_u32(this + o, 0x55);
                e.mem.set_u32(other + o, 0x55);
            }
            if offset != 0 {
                let v = e.mem.u32(other + offset) + 1;
                e.mem.set_u32(other + offset, v);
            }
            e.call(0x0051_6c10, &args![this, other]).bool()
        };
        assert!(!pair(&mut e, 0));
        for offset in [0x8c, 0x90, 0x94, 0x98] {
            assert!(pair(&mut e, offset));
        }
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(1));
        assert!(pair(&mut e, 0));
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(0));
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(pair(&mut e, 0));
    }

    /// An actor whose vtable slot `+0x100` answers `active`.
    fn activate_engine(
        active: u32,
        lock_open: u32,
        owner: u32,
        has: [u32; 2],
    ) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let actor = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x110);
        e.mem.set_u32(actor, vtable);
        e.mem.set_u32(vtable + 0x100, 0x0f47_0100);
        e.register_double(0x0f47_0100, move |_, _| ret(active));
        let lock = e.mem.alloc(8);
        e.mem.set_u32(lock + 4, 0x0abc);
        e.register_double(GET_LOCK, move |_, _| ret(lock));
        e.register_double(LOCK_CHECK_005021A0, move |_, _| ret(lock_open));
        e.register_double(IS_AN_OWNER, move |_, _| ret(owner));
        e.register_double(HAS_OBJECTS, move |_, a| {
            ret(if a[1] == 0x0abc { has[0] } else { has[1] })
        });
        e.map(PLAYER & !0xfff, 0x1000);
        e.map(USED_ITEM_FORM & !0xfff, 0x1000);
        e.set_global::<u32>(USED_ITEM_FORM, 0x0def);
        (e, actor, lock)
    }

    fn can(e: &mut Engine, actor: u32, use_item: u32) -> bool {
        e.call(0x0051_6cd0, &args![0u32, 0x9000u32, actor, use_item])
            .bool()
    }

    #[test]
    fn can_activate_checks_actor_lock_owner_and_keys() {
        let (mut e, actor, _) = activate_engine(1, 1, 0, [0, 0]);
        assert!(!can(&mut e, 0, 1));
        assert!(!can(&mut e, actor, 1));
        let (mut e, actor, _) = activate_engine(0x100, 0, 0, [0, 0]);
        assert!(!can(&mut e, actor, 1));
        let (mut e, actor, _) = activate_engine(1, 0x100, 0, [0, 0]);
        assert!(can(&mut e, actor, 0));
        let (mut e, actor, _) = activate_engine(1, 1, 1, [0, 0]);
        assert!(can(&mut e, actor, 0));
        e.set_global::<u32>(PLAYER, actor);
        assert!(!can(&mut e, actor, 0));
        let (mut e, actor, lock) = activate_engine(1, 1, 0, [1, 0]);
        let (_, log) = run(&mut e, 0x0051_6cd0, &args![0u32, 0x9000u32, actor, 0u32]);
        assert!(log.len() > 1);
        let has = only(&log, HAS_OBJECTS);
        assert_eq!(has[0][..5], [actor, 0x0abc, 0, 1, 0]);
        assert!(can(&mut e, actor, 0));
        e.mem.set_u32(lock + 4, 0);
        assert!(!can(&mut e, actor, 0));
        let (mut e, actor, _) = activate_engine(1, 1, 0, [0, 1]);
        assert!(!can(&mut e, actor, 0x100));
        assert!(can(&mut e, actor, 1));
    }
}
