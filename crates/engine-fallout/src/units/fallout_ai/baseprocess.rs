//! `fallout/ai/baseprocess.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BaseProcess` is declared in `actor_p2.rs` (only `pCachedValues`); the
//! other fields this unit reads are offsets below, Xbox PDB names, checked
//! against the PC code. The PC vtable is at `01085e2c` (its RTTI locator
//! `0111e298` names `.?AVBaseProcess@@`). Slot names quoted from the Xbox
//! PDB's `BaseProcess` vtable are not otherwise confirmed on PC.
//!
//! Several bodies here are folded stubs the PC vtable uses in many slots;
//! they are named `fn_<addr>` and described by what they do.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_ai::actor_p2::{BaseProcess, CachedValues};

/// `CurrentPackage` (Xbox PDB): the embedded `ActorPackage` at +0x04.
const CURRENT_PACKAGE: u32 = 0x04;
/// `fSystemTimeClock` (Xbox PDB), +0x1C, `float`.
const SYSTEM_TIME_CLOCK: u32 = 0x1c;
/// `fHourLastProcessed` (Xbox PDB), +0x20, `float`.
const HOUR_LAST_PROCESSED: u32 = 0x20;
/// `iDateLastProcessed` (Xbox PDB), +0x24.
const DATE_LAST_PROCESSED: u32 = 0x24;
/// `pCachedValues` (Xbox PDB), +0x2C.
const CACHED_VALUES: u32 = 0x2c;

/// `BaseProcess`'s PC vtable, stored by the destructor body.
const BASE_PROCESS_VTABLE: u32 = 0x0108_5e2c;
/// A global three-word point (zero in the exe image) that two stubs copy
/// out or answer by address.
const SHARED_POINT: u32 = 0x011f_426c;
/// `float` `FLT_MAX` (`7f7fffff` in the image); the last-time-detected stub
/// answers its negation.
const LARGEST_FLOAT: u32 = 0x0108_5df0;
/// `double` 0.0, the threshold `Copy` compares a slot's result against.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `float` -1.0 (`bf800000` in the image), stored by `Revert_ov2`.
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// `float` 20.0 (`41a00000` in the image), answered by `GetBreathTimer`.
const BREATH_TIMER_FLOAT: u32 = 0x0101_7868;
/// The player character pointer (`011dea3c`, as the other AI units name it).
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;

/// The save/load game object every save function works through.
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// The setting object whose value byte switches the save size messages on
/// (`00408d60` answers the address of its value).
const SAVE_LOG_SETTING: u32 = 0x011d_e4e8;
/// `00408d60`: the setting's value address.
const SETTING_VALUE: u32 = 0x0040_8d60;
/// `00862110`, `TESSaveLoadGame::UseSaveGameBlocks` (Xbox PDB).
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
/// `00825c00`: the save/load object's word at +0x14 (the buffer position
/// the size checks subtract).
const BUFFER_POSITION: u32 = 0x0082_5c00;
/// `008579b0`: writes `size` bytes from an address (`RET 8`).
const WRITE_BYTES: u32 = 0x0085_79b0;
/// `008579e0`: reads `size` bytes into an address (`RET 8`).
const READ_BYTES: u32 = 0x0085_79e0;
/// `004fd3e0`: the save/load object's word at +0x88 (the record being
/// saved: id at +0, flags at +5, version byte at +9); the engine map's
/// name `TES::GetWorldSpace` is a folded body.
const SAVING_RECORD: u32 = 0x004f_d3e0;
/// `004fd3c0`: the save/load object's word at +0x84 (the record being
/// loaded, same shape).
const LOADING_RECORD: u32 = 0x004f_d3c0;
/// `008df040`: the save/load object's byte at +0x80 (the message's
/// "current version").
const CURRENT_VERSION: u32 = 0x008d_f040;
/// `004839c0` (cdecl, one word): the form for a record's id word.
const FORM_FOR_ID: u32 = 0x0048_39c0;
/// `0040fbe0`, `Error` (Xbox PDB), cdecl with variable arguments.
const ERROR_MESSAGE: u32 = 0x0040_fbe0;
/// `005b5e40`, cdecl with variable arguments: the SAVELOAD warnings.
const SAVELOAD_WARNING: u32 = 0x005b_5e40;
/// The block tag written before and checked after a process block.
const BLOCK_TAG: u32 = 0x424c_4f4b;

/// `D:\_Fallout3\Platforms\Common\Code\Fallout\AI\BaseProcess.cpp`.
const SOURCE_FILE: u32 = 0x0108_65e0;
/// `GetSaveSize(): %-5i for form %08X %s with flags %08X ending at line %i in file %s`.
const SAVE_SIZE_FORM_FORMAT: u32 = 0x0101_2cb0;
/// `GetSaveSize(): %-5i ending at line %i in file %s`.
const SAVE_SIZE_FORMAT: u32 = 0x0101_2c78;
/// `SaveGame(): %-5i for form %08X %s with flags %08X ending at line %i in file %s`.
const SAVE_GAME_FORM_FORMAT: u32 = 0x0101_53a0;
/// `SaveGame(): %-5i ending at line %i in file %s`.
const SAVE_GAME_FORMAT: u32 = 0x0101_536c;
/// `SAVELOAD: Save Game Block in file %s on line %i is greater than maximum short size`.
const BLOCK_TOO_LARGE_FORMAT: u32 = 0x0101_5318;
/// `SAVELOAD: (LoadGame Buffer error) Block Header is incorrect ... Currently loading form is %08X %s with version %i and flags %08X`.
const BAD_HEADER_FORM_FORMAT: u32 = 0x0101_5718;
/// `SAVELOAD: (LoadGame Buffer error) Block Header is incorrect ... Current version is %i`.
const BAD_HEADER_FORMAT: u32 = 0x0101_56a8;
/// `SAVELOAD: LoadGame Buffer overrun of %i bytes ... Currently loading form is ...`.
const OVERRUN_FORM_FORMAT: u32 = 0x0101_5588;
/// `SAVELOAD: LoadGame Buffer underrun of %i bytes ... Currently loading form is ...`.
const UNDERRUN_FORM_FORMAT: u32 = 0x0101_5500;
/// `SAVELOAD: LoadGame Buffer overrun of %i bytes ... Current version is %i`.
const OVERRUN_FORMAT: u32 = 0x0101_54a0;
/// `SAVELOAD: LoadGame Buffer underrun of %i bytes ... Current version is %i`.
const UNDERRUN_FORMAT: u32 = 0x0101_5440;

/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors (`.?AV<name>@@` at +8).
const TYPE_BASE_PROCESS: u32 = 0x0118_a6b8;
const TYPE_HIGH_PROCESS: u32 = 0x0118_a6d4;
const TYPE_MIDDLE_HIGH_PROCESS: u32 = 0x0118_c64c;
const TYPE_MIDDLE_LOW_PROCESS: u32 = 0x011a_3668;
const TYPE_LOW_PROCESS: u32 = 0x011a_364c;
const TYPE_MOBILE_OBJECT: u32 = 0x0118_4920;
const TYPE_ACTOR: u32 = 0x0118_46d4;

/// `__RTDynamicCast(object, 0, from, to, 0)`.
fn dynamic_cast(e: &mut Engine, object: u32, from: u32, to: u32) -> u32 {
    e.call(RT_DYNAMIC_CAST, &args![object, 0u32, from, to, 0u32])
        .u32()
}

fn save_load_game(e: &Engine) -> u32 {
    e.global(SAVE_LOAD_GAME)
}

fn use_save_game_blocks(e: &mut Engine) -> bool {
    let save_load = save_load_game(e);
    e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool()
}

fn buffer_position(e: &mut Engine) -> u32 {
    let save_load = save_load_game(e);
    e.call(BUFFER_POSITION, &args![save_load]).u32()
}

fn save_log_enabled(e: &mut Engine) -> bool {
    let value = e.call(SETTING_VALUE, &args![SAVE_LOG_SETTING]).u32();
    e.mem.u8(value) != 0
}

/// The words a save message prints for a record, in the order the code
/// reads them: flags (+5) and version (+9) before the name slot, the id
/// after it.
struct RecordWords {
    form_id: u32,
    name: u32,
    version: u32,
    flags: u32,
}

/// Reads a record's message words: the form for its id, then the record's
/// flags (and version when `with_version`), then the form's name (vtable
/// +0x130), then its id.
fn record_words(e: &mut Engine, record: u32, with_version: bool) -> RecordWords {
    let id = e.mem.u32(record);
    let form = e.call(FORM_FOR_ID, &args![id]).u32();
    let flags = e.mem.u32(record.wrapping_add(5));
    let version = if with_version {
        e.mem.u8(record.wrapping_add(9)) as u32
    } else {
        0
    };
    let name = e.vcall(form, 0x130, &[]).u32();
    let form_id = e.mem.u32(record);
    RecordWords {
        form_id,
        name,
        version,
        flags,
    }
}

// Translated from 0044edb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the word at `this + 0x10`. The engine map names this folded body
/// `BaseProcess::GetCurrentProcedureIndex`; other units call it on other
/// objects.
pub fn fn_0044edb0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x10))
}

// Translated from 00717e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers `this + 4`. The engine map names this folded body
/// `BaseProcess::GetActorPackageThatIsRunning` (the PC `BaseProcess`
/// vtable has it in slot +0x278, where it answers the embedded
/// `CurrentPackage`); other units call it on other objects.
pub fn fn_00717e50(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(4)
}

// Translated from 008d02d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::GetLastTimeDetected` (Xbox PDB): answers the negated
/// `float` at `01085df0` (`-FLT_MAX` in the image); both words unread.
pub fn base_process_get_last_time_detected(
    e: &mut Engine,
    _this: Ptr<BaseProcess>,
    _unused_0: u32,
    _unused_1: u32,
) -> f32 {
    -x87_quiet(e.global(LARGEST_FLOAT))
}

// Translated from 008d02f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers false in AL; six argument words unread (PC vtable slot +0x44).
#[allow(clippy::too_many_arguments)] // The exe ABI's argument words.
pub fn fn_008d02f0(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> bool {
    false
}

// Translated from 008d0300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers false in AL; two argument words unread. Shared by many slots of
/// the PC `BaseProcess` vtable and called from `actor_p3.rs`.
pub fn fn_008d0300(_e: &mut Engine, _this: Ptr, _unused_0: u32, _unused_1: u32) -> bool {
    false
}

// Translated from 008d0310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three words of the global point at `011f426c` into `result`
/// and answers `result`; two more words unread (PC vtable slots +0x1C and
/// +0x98).
pub fn fn_008d0310(e: &mut Engine, _this: Ptr, result: Ptr, _unused_0: u32, _unused_1: u32) -> Ptr {
    for offset in [0, 4, 8] {
        let word = e.mem.u32(SHARED_POINT + offset);
        e.mem.set_u32(result.addr().wrapping_add(offset), word);
    }
    result
}

// Translated from 008d0340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers 0 in EAX; seven argument words unread (PC vtable slot +0xF0).
#[allow(clippy::too_many_arguments)] // The exe ABI's argument words.
pub fn fn_008d0340(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    0
}

// Translated from 008d0350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Does nothing; seven argument words unread (PC vtable slot +0xFC).
#[allow(clippy::too_many_arguments)] // The exe ABI's argument words.
pub fn fn_008d0350(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) {
}

// Translated from 008d0360 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Returns true in AL without reading the receiver. This folded body is
/// also used by a reference vtable; its mapped unit does not identify all
/// callers, and no Xbox PDB name is asserted.
pub fn fn_008d0360(_e: &mut Engine, _this: Ptr) -> bool {
    true
}

// Translated from 008d0370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers false in AL; one argument word unread. Shared by many slots of
/// the PC `BaseProcess` vtable and called from `actor_p2.rs`/`actor_p3.rs`.
pub fn fn_008d0370(_e: &mut Engine, _this: Ptr, _unused_0: u32) -> bool {
    false
}

// Translated from 008d0380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Does nothing; eleven argument words unread (PC vtable slot +0x178).
#[allow(clippy::too_many_arguments)] // The exe ABI's argument words.
pub fn fn_008d0380(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
    _unused_8: u32,
    _unused_9: u32,
    _unused_10: u32,
) {
}

// Translated from 008d0390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers 0.0 in ST0 (`FLDZ`); two argument words unread (PC vtable slot
/// +0x208).
pub fn fn_008d0390(_e: &mut Engine, _this: Ptr, _unused_0: u32, _unused_1: u32) -> f32 {
    0.0
}

// Translated from 008d03a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::ClearCurrentPackage` (Xbox PDB): `ActorPackage::Clear`
/// (`008c5ef0`) on the embedded current package.
pub fn base_process_clear_current_package(e: &mut Engine, this: Ptr<BaseProcess>) {
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x008c_5ef0, &args![package]);
}

// Translated from 008d03c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::SetCurrentProcedureIndex` (Xbox PDB): `0041fd00` (a
/// folded one-word store at +0x0C) on the embedded current package.
pub fn base_process_set_current_procedure_index(
    e: &mut Engine,
    this: Ptr<BaseProcess>,
    index: u32,
) {
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x0041_fd00, &args![package, index]);
}

// Translated from 008d03e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers `0084e3a0` (a getter of the word at +0x0C) on the embedded
/// current package; PC vtable slot +0x23C, the Xbox PDB's
/// `GetCurrentProcedureIndex` slot.
pub fn fn_008d03e0(e: &mut Engine, this: Ptr<BaseProcess>) -> u32 {
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x0084_e3a0, &args![package]).u32()
}

// Translated from 008d0400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::AddToCurrentProcedureIndex` (Xbox PDB): `0049bb50` (adds
/// its argument to the word at +0x0C) on the embedded current package.
pub fn base_process_add_to_current_procedure_index(
    e: &mut Engine,
    this: Ptr<BaseProcess>,
    amount: u32,
) {
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x0049_bb50, &args![package, amount]);
}

// Translated from 008d0420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Does nothing; eight argument words unread (PC vtable slot +0x250).
#[allow(clippy::too_many_arguments)] // The exe ABI's argument words.
pub fn fn_008d0420(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
) {
}

// Translated from 008d0430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers `00726070` (a getter of the word at +0x04) on the embedded
/// current package; PC vtable slot +0x274.
pub fn fn_008d0430(e: &mut Engine, this: Ptr<BaseProcess>) -> u32 {
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x0072_6070, &args![package]).u32()
}

// Translated from 008d0450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::GetProcedureIndexRunning` (Xbox PDB): answers the
/// receiver's own vtable slot +0x23C.
pub fn base_process_get_procedure_index_running(e: &mut Engine, this: Ptr<BaseProcess>) -> u32 {
    e.vcall(this.addr(), 0x23c, &[]).u32()
}

// Translated from 008d0470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::SetProcedureIndexRunning` (Xbox PDB): the receiver's own
/// vtable slot +0x238 with `index`.
pub fn base_process_set_procedure_index_running(
    e: &mut Engine,
    this: Ptr<BaseProcess>,
    index: u32,
) {
    e.vcall(this.addr(), 0x238, &args![index]);
}

// Translated from 008d04a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::AddToProcedureIndexRunning` (Xbox PDB): the receiver's own
/// vtable slot +0x240 with the second word; the first is unread.
pub fn base_process_add_to_procedure_index_running(
    e: &mut Engine,
    this: Ptr<BaseProcess>,
    _unused_0: u32,
    amount: u32,
) {
    e.vcall(this.addr(), 0x240, &args![amount]);
}

// Translated from 008d04d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::SetCharController` (Xbox PDB): only releases the argument
/// it was passed by value: `0045cec0` on the address of the argument word
/// (it acts on the pointer there when it is not null).
pub fn base_process_set_char_controller(e: &mut Engine, _this: Ptr<BaseProcess>, controller: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), controller);
        e.call(0x0045_cec0, &args![slot]);
    });
}

// Translated from 008d04f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Does nothing; six argument words unread (PC vtable slot +0x2A4).
#[allow(clippy::too_many_arguments)] // The exe ABI's argument words.
pub fn fn_008d04f0(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
}

// Translated from 008d0500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Does nothing; five argument words unread (PC vtable slots +0x2BC and
/// +0x418).
pub fn fn_008d0500(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) {
}

// Translated from 008d0510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers false in AL; four argument words unread (PC vtable slot +0x2CC).
pub fn fn_008d0510(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) -> bool {
    false
}

// Translated from 008d0520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers false in AL; three argument words unread (several PC vtable
/// slots).
pub fn fn_008d0520(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
) -> bool {
    false
}

// Translated from 008d0530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers false in AL; thirteen argument words unread (PC vtable slot
/// +0x33C).
#[allow(clippy::too_many_arguments)] // The exe ABI's argument words.
pub fn fn_008d0530(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
    _unused_8: u32,
    _unused_9: u32,
    _unused_10: u32,
    _unused_11: u32,
    _unused_12: u32,
) -> bool {
    false
}

// Translated from 008d0540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::CalculateMoveMode` (Xbox PDB): answers 0x101 in EAX; six
/// argument words unread.
#[allow(clippy::too_many_arguments)] // The exe ABI's argument words.
pub fn base_process_calculate_move_mode(
    _e: &mut Engine,
    _this: Ptr<BaseProcess>,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    0x101
}

// Translated from 008d0560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers 0 in EAX; one argument word unread. Shared by many slots of the
/// PC `BaseProcess` vtable.
pub fn fn_008d0560(_e: &mut Engine, _this: Ptr, _unused_0: u32) -> u32 {
    0
}

// Translated from 008d0570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers -1 in EAX; three argument words unread (PC vtable slot +0x398).
pub fn fn_008d0570(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
) -> i32 {
    -1
}

// Translated from 008d0580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers 0.0 in ST0 (`FLDZ`); three argument words unread (PC vtable slot
/// +0x39C).
pub fn fn_008d0580(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
) -> f32 {
    0.0
}

// Translated from 008d0590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::GetSoundHandle` (Xbox PDB): constructs an empty sound
/// handle in `result` (`0041a250`) and answers `result`; the second word is
/// unread.
pub fn base_process_get_sound_handle(
    e: &mut Engine,
    _this: Ptr<BaseProcess>,
    result: Ptr,
    _unused_0: u32,
) -> Ptr {
    e.call(0x0041_a250, &args![result]);
    result
}

// Translated from 008d0610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body, frees the object through `00401030` when bit 0 of
/// `flags` is set, and answers `this`.
pub fn base_process_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BaseProcess>,
    flags: u32,
) -> Ptr<BaseProcess> {
    fn_008d0640(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 008d0640 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body: stores `BaseProcess`'s vtable and destroys the
/// embedded current package (`008c5e70`).
pub fn fn_008d0640(e: &mut Engine, this: Ptr<BaseProcess>) {
    e.mem.set_u32(this.addr(), BASE_PROCESS_VTABLE);
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x008c_5e70, &args![package]);
}

/// `Copy`'s step for one high process: when its vtable slot +0x1E0 answers
/// more than the `double` at `01012060` (not NaN), slot +0x214, then slot
/// +0x12C with 0.
fn copy_reset_high_process(e: &mut Engine, process: u32) {
    if process == 0 {
        return;
    }
    let value = e.vcall(process, 0x1e0, &[]).f64();
    let threshold: f64 = e.global(ZERO_DOUBLE);
    // `FCOMP; TEST AH,0x41; JNZ`: only an ordered "greater" carries on.
    if value > threshold {
        e.vcall(process, 0x214, &[]);
        e.vcall(process, 0x12c, &args![0u32]);
    }
}

// Translated from 008d0660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::Copy` (Xbox PDB): does nothing for a null `other`.
/// Otherwise casts `other` and then `this` to `HighProcess`,
/// `MiddleHighProcess`, `MiddleLowProcess` and `LowProcess`; resets each
/// high process (see `copy_reset_high_process`, `other` first); then for
/// each level both are, calls that level's copy on `this` with `other`
/// (`008d9fa0` `HighProcess::HighCopy`, `00914c60`
/// `MiddleHighProcess::MiddleHighCopy`, `0092ca70`, `00907510`); last,
/// when both have cached values, `008675e0` on this's with other's.
pub fn base_process_copy(e: &mut Engine, this: Ptr<BaseProcess>, other: Ptr<BaseProcess>) {
    if other.is_null() {
        return;
    }
    let levels = [
        TYPE_HIGH_PROCESS,
        TYPE_MIDDLE_HIGH_PROCESS,
        TYPE_MIDDLE_LOW_PROCESS,
        TYPE_LOW_PROCESS,
    ];
    let mut other_as = [0u32; 4];
    for (slot, level) in other_as.iter_mut().zip(levels) {
        *slot = dynamic_cast(e, other.addr(), TYPE_BASE_PROCESS, level);
    }
    let mut this_as = [0u32; 4];
    for (slot, level) in this_as.iter_mut().zip(levels) {
        *slot = dynamic_cast(e, this.addr(), TYPE_BASE_PROCESS, level);
    }
    copy_reset_high_process(e, other_as[0]);
    copy_reset_high_process(e, this_as[0]);
    let level_copies = [0x008d_9fa0u32, 0x0091_4c60, 0x0092_ca70, 0x0090_7510];
    for ((target, source), copy) in this_as.into_iter().zip(other_as).zip(level_copies) {
        if target != 0 && source != 0 {
            e.call(copy, &args![target, source]);
        }
    }
    let this_cached = e.mem.u32(this.addr().wrapping_add(CACHED_VALUES));
    if this_cached != 0 {
        let other_cached = e.mem.u32(other.addr().wrapping_add(CACHED_VALUES));
        if other_cached != 0 {
            e.call(0x0086_75e0, &args![this_cached, other_cached]);
        }
    }
}

// Translated from 008d0870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::HandleDeletedReference` (Xbox PDB): passes `reference` to
/// the embedded package's handler (`008c5ec0`), then to vtable slot +0x04
/// of the object `fn_008d0430` answers, when there is one.
pub fn base_process_handle_deleted_reference(
    e: &mut Engine,
    this: Ptr<BaseProcess>,
    reference: u32,
) {
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x008c_5ec0, &args![package, reference]);
    let running = fn_008d0430(e, this);
    if running != 0 {
        e.vcall(running, 0x4, &args![reference]);
    }
}

// Translated from 008d08b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::SetCurrentPackage` (Xbox PDB): computes the package's
/// initial target for `actor` (`TESPackage::ComputeInitialTarget`,
/// `006759e0`) and starts the embedded package from both
/// (`ActorPackage::InitFromPackage`, `008c6080`).
pub fn base_process_set_current_package(
    e: &mut Engine,
    this: Ptr<BaseProcess>,
    package: u32,
    actor: u32,
) {
    let target = e.call(0x0067_59e0, &args![package, actor]).u32();
    let current = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x008c_6080, &args![current, package, target]);
}

// Translated from 008d08f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::GetWeaponLastPos` (Xbox PDB): answers the address of the
/// global point at `011f426c`.
pub fn base_process_get_weapon_last_pos(_e: &mut Engine, _this: Ptr<BaseProcess>) -> u32 {
    SHARED_POINT
}

// Translated from 008d0900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::GetSaveSize` (Xbox PDB): the 16-bit size of the process
/// block: 6 for the block header when save game blocks are used, plus the
/// embedded package's size (`008c61a0` with `version`), plus 4 for
/// `fSystemTimeClock`. With the save log setting on, prints it through
/// `Error` (line 0xE8), with the record being saved when there is one. The
/// second word is unread.
pub fn base_process_get_save_size(
    e: &mut Engine,
    this: Ptr<BaseProcess>,
    argument: u32,
    _unused_0: u32,
) -> u16 {
    let mut size: u16 = 0;
    if use_save_game_blocks(e) {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    let package_size = e.call(0x008c_61a0, &args![package, argument]).u16();
    size = size.wrapping_add(package_size);
    size = size.wrapping_add(4);
    if save_log_enabled(e) {
        let save_load = save_load_game(e);
        let record = e.call(SAVING_RECORD, &args![save_load]).u32();
        if record != 0 {
            let words = record_words(e, record, false);
            e.call(
                ERROR_MESSAGE,
                &args![
                    SAVE_SIZE_FORM_FORMAT,
                    size as u32,
                    words.form_id,
                    words.name,
                    words.flags,
                    0xe8u32,
                    SOURCE_FILE
                ],
            );
        } else {
            e.call(
                ERROR_MESSAGE,
                &args![SAVE_SIZE_FORMAT, size as u32, 0xe8u32, SOURCE_FILE],
            );
        }
    }
    size
}

// Translated from 008d0a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::SaveGame` (Xbox PDB): with save game blocks, writes the
/// block tag `KOLB` and a 16-bit zero length placeholder; then the embedded
/// package (`008c6250`) and `fSystemTimeClock`. With the save log setting
/// on, prints the bytes written since the start (`Error`, line 0xF9). With
/// blocks, stores the block's length (position now minus the placeholder's
/// position) as 16 bits at the placeholder, warning first when it exceeds
/// 0xFFFF. The second word is unread.
pub fn base_process_save_game(
    e: &mut Engine,
    this: Ptr<BaseProcess>,
    argument: u32,
    _unused_0: u32,
) {
    let mut length_at: u32 = 0;
    let mut start = buffer_position(e);
    if save_log_enabled(e) {
        start = buffer_position(e);
    }
    if use_save_game_blocks(e) {
        e.with_stack(4, |e, tag| {
            e.mem.set_u32(tag.addr(), BLOCK_TAG);
            let save_load = save_load_game(e);
            e.call(WRITE_BYTES, &args![save_load, tag, 4u32]);
        });
        length_at = buffer_position(e);
        e.with_stack(4, |e, length| {
            let save_load = save_load_game(e);
            e.call(WRITE_BYTES, &args![save_load, length, 2u32]);
        });
    }
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x008c_6250, &args![package, argument]);
    let save_load = save_load_game(e);
    let clock = this.addr().wrapping_add(SYSTEM_TIME_CLOCK);
    e.call(WRITE_BYTES, &args![save_load, clock, 4u32]);
    if save_log_enabled(e) {
        let end = buffer_position(e);
        let save_load = save_load_game(e);
        let record = e.call(SAVING_RECORD, &args![save_load]).u32();
        let written = end.wrapping_sub(start);
        if record != 0 {
            let words = record_words(e, record, false);
            e.call(
                ERROR_MESSAGE,
                &args![
                    SAVE_GAME_FORM_FORMAT,
                    written,
                    words.form_id,
                    words.name,
                    words.flags,
                    0xf9u32,
                    SOURCE_FILE
                ],
            );
        } else {
            e.call(
                ERROR_MESSAGE,
                &args![SAVE_GAME_FORMAT, written, 0xf9u32, SOURCE_FILE],
            );
        }
    }
    if use_save_game_blocks(e) {
        let end = buffer_position(e);
        if end > length_at.wrapping_add(0xffff) {
            e.call(
                SAVELOAD_WARNING,
                &args![BLOCK_TOO_LARGE_FORMAT, SOURCE_FILE, 0xf9u32],
            );
        }
        e.mem.set_u16(length_at, end.wrapping_sub(length_at) as u16);
    }
}

// Translated from 008d0bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::LoadGame` (Xbox PDB): with save game blocks, reads the
/// block tag (warning at line 0x104 when it is not `KOLB`, with the record
/// being loaded or the current version) and the 16-bit block length after
/// noting the position. Then loads the embedded package
/// (`ActorPackage::LoadGame`, `008c6320`, with the first two words) and
/// `fSystemTimeClock`. With blocks, compares the position with where the
/// block should end and warns of an overrun or underrun (line 0x109). The
/// third word is unread.
pub fn base_process_load_game(
    e: &mut Engine,
    this: Ptr<BaseProcess>,
    argument_0: u32,
    argument_1: u32,
    _unused_0: u32,
) {
    let mut length: u16 = 0;
    let mut block_start: u32 = 0;
    if use_save_game_blocks(e) {
        let tag = e.with_stack(4, |e, tag| {
            let save_load = save_load_game(e);
            e.call(READ_BYTES, &args![save_load, tag, 4u32]);
            e.mem.u32(tag.addr())
        });
        if tag != BLOCK_TAG {
            let save_load = save_load_game(e);
            let record = e.call(LOADING_RECORD, &args![save_load]).u32();
            if record != 0 {
                let words = record_words(e, record, true);
                e.call(
                    SAVELOAD_WARNING,
                    &args![
                        BAD_HEADER_FORM_FORMAT,
                        SOURCE_FILE,
                        0x104u32,
                        words.form_id,
                        words.name,
                        words.version,
                        words.flags
                    ],
                );
            } else {
                let save_load = save_load_game(e);
                let current = e.call(CURRENT_VERSION, &args![save_load]).u8() as u32;
                e.call(
                    SAVELOAD_WARNING,
                    &args![BAD_HEADER_FORMAT, SOURCE_FILE, 0x104u32, current],
                );
            }
        }
        block_start = buffer_position(e);
        length = e.with_stack(4, |e, slot| {
            let save_load = save_load_game(e);
            e.call(READ_BYTES, &args![save_load, slot, 2u32]);
            e.mem.u16(slot.addr())
        });
    }
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x008c_6320, &args![package, argument_0, argument_1]);
    let save_load = save_load_game(e);
    let clock = this.addr().wrapping_add(SYSTEM_TIME_CLOCK);
    e.call(READ_BYTES, &args![save_load, clock, 4u32]);
    if !use_save_game_blocks(e) {
        return;
    }
    let end = buffer_position(e);
    let save_load = save_load_game(e);
    let record = e.call(LOADING_RECORD, &args![save_load]).u32();
    let expected = (length as u32).wrapping_add(block_start);
    if record != 0 {
        let id = e.mem.u32(record);
        let form = e.call(FORM_FOR_ID, &args![id]).u32();
        let format = if end > expected {
            OVERRUN_FORM_FORMAT
        } else if end < expected {
            UNDERRUN_FORM_FORMAT
        } else {
            return;
        };
        let bytes = if end > expected {
            end.wrapping_sub(expected)
        } else {
            expected.wrapping_sub(end)
        };
        let flags = e.mem.u32(record.wrapping_add(5));
        let record_version = e.mem.u8(record.wrapping_add(9)) as u32;
        let name = e.vcall(form, 0x130, &[]).u32();
        let form_id = e.mem.u32(record);
        e.call(
            SAVELOAD_WARNING,
            &args![
                format,
                bytes,
                SOURCE_FILE,
                0x109u32,
                form_id,
                name,
                record_version,
                flags
            ],
        );
    } else {
        let (format, bytes) = if end > expected {
            (OVERRUN_FORMAT, end.wrapping_sub(expected))
        } else if end < expected {
            (UNDERRUN_FORMAT, expected.wrapping_sub(end))
        } else {
            return;
        };
        let save_load = save_load_game(e);
        let current = e.call(CURRENT_VERSION, &args![save_load]).u8() as u32;
        e.call(
            SAVELOAD_WARNING,
            &args![format, bytes, SOURCE_FILE, 0x109u32, current],
        );
    }
}

// Translated from 008d0e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::InitLoadGame` (Xbox PDB): `ActorPackage::InitLoadGame`
/// (`008c64d0`) on the embedded package with the first two words; the
/// third is unread.
pub fn base_process_init_load_game(
    e: &mut Engine,
    this: Ptr<BaseProcess>,
    argument_0: u32,
    argument_1: u32,
    _unused_0: u32,
) {
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x008c_64d0, &args![package, argument_0, argument_1]);
}

// Translated from 008d0e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::FinishInitLoadGame` (Xbox PDB): when the third word casts
/// from `MobileObject` to an `Actor` that is not the player, the process's
/// slot +0x22C answers 0 and slot +0x280 answers -1, calls slot +0x284 with
/// 0 and `Actor::EvaluatePackage` (`008a6ce0`) on the actor with (1, 1).
/// The first two words are unread.
pub fn base_process_finish_init_load_game(
    e: &mut Engine,
    this: Ptr<BaseProcess>,
    _unused_0: u32,
    _unused_1: u32,
    object: u32,
) {
    let actor = dynamic_cast(e, object, TYPE_MOBILE_OBJECT, TYPE_ACTOR);
    if actor == 0 || actor == e.global::<u32>(PLAYER_CHARACTER) {
        return;
    }
    if e.vcall(this.addr(), 0x22c, &[]).u32() != 0 {
        return;
    }
    if e.vcall(this.addr(), 0x280, &[]).u32() != u32::MAX {
        return;
    }
    e.vcall(this.addr(), 0x284, &args![0u32]);
    e.call(0x008a_6ce0, &args![actor, 1u32, 1u32]);
}

// Translated from 008d0f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::Revert` (Xbox PDB): `008c65a0` on the embedded package
/// with the first word; the second is unread.
pub fn base_process_revert(e: &mut Engine, this: Ptr<BaseProcess>, argument: u32, _unused_0: u32) {
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x008c_65a0, &args![package, argument]);
}

// Translated from 008d0f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::SaveGame_ov2` (Xbox PDB): writes `fSystemTimeClock`,
/// `fHourLastProcessed` and `iDateLastProcessed` (4 bytes each, through
/// `00865e50` with a final 0), then `ActorPackage::SaveGame` (`008c65d0`)
/// on the embedded package.
pub fn base_process_save_game_ov2(e: &mut Engine, this: Ptr<BaseProcess>, buffer: u32) {
    for field in [SYSTEM_TIME_CLOCK, HOUR_LAST_PROCESSED, DATE_LAST_PROCESSED] {
        let at = this.addr().wrapping_add(field);
        e.call(0x0086_5e50, &args![buffer, at, 4u32, 0u32]);
    }
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x008c_65d0, &args![package, buffer]);
}

// Translated from 008d0f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::LoadGame_ov2` (Xbox PDB): reads `fSystemTimeClock`
/// (`00864980`, 4 bytes); when the buffer's vtable slot +0x00 answers a
/// byte of at least 6, also `fHourLastProcessed` and `iDateLastProcessed`;
/// then `ActorPackage::LoadGame_ov2` (`008c6730`) on the embedded package.
pub fn base_process_load_game_ov2(e: &mut Engine, this: Ptr<BaseProcess>, buffer: u32) {
    let clock = this.addr().wrapping_add(SYSTEM_TIME_CLOCK);
    e.call(0x0086_4980, &args![buffer, clock, 4u32]);
    let buffer_version = e.vcall(buffer, 0x0, &[]).u8() as i32;
    if buffer_version >= 6 {
        for field in [HOUR_LAST_PROCESSED, DATE_LAST_PROCESSED] {
            let at = this.addr().wrapping_add(field);
            e.call(0x0086_4980, &args![buffer, at, 4u32]);
        }
    }
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x008c_6730, &args![package, buffer]);
}

// Translated from 008d1000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::InitLoadGame_ov2` (Xbox PDB): `ActorPackage::InitLoadGame_ov2`
/// (`008c6970`) on the embedded package, then clears the cached values'
/// flags when there are cached values.
pub fn base_process_init_load_game_ov2(e: &mut Engine, this: Ptr<BaseProcess>, buffer: u32) {
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x008c_6970, &args![package, buffer]);
    let cached = e.get(this, BaseProcess::pCachedValues);
    if !cached.is_null() {
        fn_008d1030(e, cached);
    }
}

// Translated from 008d1030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears `CachedValues::iFlags` (+0x44), marking every cached value out of
/// date.
pub fn fn_008d1030(e: &mut Engine, this: Ptr<CachedValues>) {
    e.set(this, CachedValues::iFlags, 0);
}

// Translated from 008d1050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::FinishLoadGame` (Xbox PDB): `ActorPackage::FinishLoadGame`
/// (`008c6b20`) on the embedded package.
pub fn base_process_finish_load_game(e: &mut Engine, this: Ptr<BaseProcess>, buffer: u32) {
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x008c_6b20, &args![package, buffer]);
}

// Translated from 008d1070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::Revert_ov2` (Xbox PDB): sets `fHourLastProcessed` to the
/// `float` at `01012054` (-1.0; copied through the x87 stack) and
/// `iDateLastProcessed` to 0, reverts the embedded package
/// (`ActorPackage::Revert`, `008c6c80`) and clears the cached values'
/// flags when there are cached values.
pub fn base_process_revert_ov2(e: &mut Engine, this: Ptr<BaseProcess>, buffer: u32) {
    let hour = x87_quiet(e.global(MINUS_ONE_FLOAT));
    e.mem
        .set_f32(this.addr().wrapping_add(HOUR_LAST_PROCESSED), hour);
    e.mem
        .set_u32(this.addr().wrapping_add(DATE_LAST_PROCESSED), 0);
    let package = this.addr().wrapping_add(CURRENT_PACKAGE);
    e.call(0x008c_6c80, &args![package, buffer]);
    let cached = e.get(this, BaseProcess::pCachedValues);
    if !cached.is_null() {
        fn_008d1030(e, cached);
    }
}

// Translated from 008d10c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::GetBreathTimer` (Xbox PDB): answers the `float` at
/// `01017868` (20.0 in the image).
pub fn base_process_get_breath_timer(e: &mut Engine, _this: Ptr<BaseProcess>) -> f32 {
    x87_quiet(e.global(BREATH_TIMER_FLOAT))
}

// Translated from 00401280 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the address of the empty string at 01011584 without reading the
/// receiver. The map names this folded body
/// `BaseProcess::GetCurrentHeadTrackTypeString`; forms use it as their
/// name getter (vtable +0x130), which script error reports call.
pub fn fn_00401280(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0101_1584
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d0360, fn_008d0360(Ptr) -> bool),
        entry!(0x00401280, fn_00401280(Ptr) -> u32),
        entry!(0x0044edb0, fn_0044edb0(Ptr) -> u32),
        entry!(0x00717e50, fn_00717e50(Ptr) -> u32),
        entry!(
            0x008d02d0,
            base_process_get_last_time_detected(Ptr<BaseProcess>, u32, u32) -> f32
        ),
        entry!(
            0x008d02f0,
            fn_008d02f0(Ptr, u32, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(0x008d0300, fn_008d0300(Ptr, u32, u32) -> bool),
        entry!(0x008d0310, fn_008d0310(Ptr, Ptr, u32, u32) -> Ptr),
        entry!(
            0x008d0340,
            fn_008d0340(Ptr, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x008d0350,
            fn_008d0350(Ptr, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x008d0370, fn_008d0370(Ptr, u32) -> bool),
        entry!(
            0x008d0380,
            fn_008d0380(Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x008d0390, fn_008d0390(Ptr, u32, u32) -> f32),
        entry!(
            0x008d03a0,
            base_process_clear_current_package(Ptr<BaseProcess>)
        ),
        entry!(
            0x008d03c0,
            base_process_set_current_procedure_index(Ptr<BaseProcess>, u32)
        ),
        entry!(0x008d03e0, fn_008d03e0(Ptr<BaseProcess>) -> u32),
        entry!(
            0x008d0400,
            base_process_add_to_current_procedure_index(Ptr<BaseProcess>, u32)
        ),
        entry!(
            0x008d0420,
            fn_008d0420(Ptr, u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x008d0430, fn_008d0430(Ptr<BaseProcess>) -> u32),
        entry!(
            0x008d0450,
            base_process_get_procedure_index_running(Ptr<BaseProcess>) -> u32
        ),
        entry!(
            0x008d0470,
            base_process_set_procedure_index_running(Ptr<BaseProcess>, u32)
        ),
        entry!(
            0x008d04a0,
            base_process_add_to_procedure_index_running(Ptr<BaseProcess>, u32, u32)
        ),
        entry!(
            0x008d04d0,
            base_process_set_char_controller(Ptr<BaseProcess>, u32)
        ),
        entry!(0x008d04f0, fn_008d04f0(Ptr, u32, u32, u32, u32, u32, u32)),
        entry!(0x008d0500, fn_008d0500(Ptr, u32, u32, u32, u32, u32)),
        entry!(0x008d0510, fn_008d0510(Ptr, u32, u32, u32, u32) -> bool),
        entry!(0x008d0520, fn_008d0520(Ptr, u32, u32, u32) -> bool),
        entry!(
            0x008d0530,
            fn_008d0530(Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32)
                -> bool
        ),
        entry!(
            0x008d0540,
            base_process_calculate_move_mode(Ptr<BaseProcess>, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x008d0560, fn_008d0560(Ptr, u32) -> u32),
        entry!(0x008d0570, fn_008d0570(Ptr, u32, u32, u32) -> i32),
        entry!(0x008d0580, fn_008d0580(Ptr, u32, u32, u32) -> f32),
        entry!(
            0x008d0590,
            base_process_get_sound_handle(Ptr<BaseProcess>, Ptr, u32) -> Ptr
        ),
        entry!(
            0x008d0610,
            base_process_scalar_deleting_destructor(Ptr<BaseProcess>, u32) -> Ptr<BaseProcess>
        ),
        entry!(0x008d0640, fn_008d0640(Ptr<BaseProcess>)),
        entry!(
            0x008d0660,
            base_process_copy(Ptr<BaseProcess>, Ptr<BaseProcess>)
        ),
        entry!(
            0x008d0870,
            base_process_handle_deleted_reference(Ptr<BaseProcess>, u32)
        ),
        entry!(
            0x008d08b0,
            base_process_set_current_package(Ptr<BaseProcess>, u32, u32)
        ),
        entry!(
            0x008d08f0,
            base_process_get_weapon_last_pos(Ptr<BaseProcess>) -> u32
        ),
        entry!(
            0x008d0900,
            base_process_get_save_size(Ptr<BaseProcess>, u32, u32) -> u16
        ),
        entry!(
            0x008d0a10,
            base_process_save_game(Ptr<BaseProcess>, u32, u32)
        ),
        entry!(
            0x008d0bc0,
            base_process_load_game(Ptr<BaseProcess>, u32, u32, u32)
        ),
        entry!(
            0x008d0e60,
            base_process_init_load_game(Ptr<BaseProcess>, u32, u32, u32)
        ),
        entry!(
            0x008d0e80,
            base_process_finish_init_load_game(Ptr<BaseProcess>, u32, u32, u32)
        ),
        entry!(0x008d0f10, base_process_revert(Ptr<BaseProcess>, u32, u32)),
        entry!(
            0x008d0f30,
            base_process_save_game_ov2(Ptr<BaseProcess>, u32)
        ),
        entry!(
            0x008d0f90,
            base_process_load_game_ov2(Ptr<BaseProcess>, u32)
        ),
        entry!(
            0x008d1000,
            base_process_init_load_game_ov2(Ptr<BaseProcess>, u32)
        ),
        entry!(0x008d1030, fn_008d1030(Ptr<CachedValues>)),
        entry!(
            0x008d1050,
            base_process_finish_load_game(Ptr<BaseProcess>, u32)
        ),
        entry!(0x008d1070, base_process_revert_ov2(Ptr<BaseProcess>, u32)),
        entry!(
            0x008d10c0,
            base_process_get_breath_timer(Ptr<BaseProcess>) -> f32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predicate_returns_true_without_reading_receiver_or_writing_memory() {
        // Six native cases on unpacked 1.4.0.525, SHA-256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f,
        // FCW 027f / MXCSR 1f80: AL=1, RET 0, no faults or guard writes.
        let mut e = Engine::new();
        let guard = e.mem.alloc(16);
        e.mem.write(guard, &[0xa5; 16]);
        for receiver in [0, 1, 0x1000_0000, 0xdead_beef, u32::MAX, guard] {
            assert!(e.call(0x008d0360, &[receiver]).bool());
            assert_eq!(e.mem.bytes(guard, 16), [0xa5; 16]);
        }
    }

    #[test]
    fn name_getter_answers_the_shared_empty_string() {
        let mut e = Engine::new();
        for receiver in [0, 0x1234, u32::MAX] {
            assert_eq!(e.call(0x00401280, &[receiver]).u32(), 0x0101_1584);
        }
    }

    #[test]
    fn load_buffer_type_queries_reach_both_shared_predicates() {
        // BGSLoadFormBuffer::GetReference / GetActor (Xbox PDB) use
        // reference vtable slots +0xf0 / +0x100 on PC. Exercise the real
        // registered translations, with generated receivers and vtables.
        let mut e = Engine::new();
        let buffer = e.mem.alloc(0x30);
        let buffer_vtable = e.mem.alloc(16);
        let reference = e.mem.alloc(4);
        let reference_vtable = e.mem.alloc(0x104);
        e.mem.set_u32(buffer, buffer_vtable);
        e.mem.set_u32(buffer + 0x24, reference);
        e.mem.set_u32(buffer_vtable + 4, 0x008645b0);
        e.mem.set_u32(buffer_vtable + 8, 0x00864610);
        e.mem.set_u32(buffer_vtable + 12, 0x00864660);
        e.mem.set_u32(reference, reference_vtable);
        e.mem.set_u32(reference_vtable + 0xf0, 0x008d0360);
        e.mem.set_u32(reference_vtable + 0x100, 0x0047c850);
        assert_eq!(e.call(0x00864610, &[buffer]).u32(), reference);
        assert_eq!(e.call(0x00864660, &[buffer]).u32(), 0);
        e.mem.set_u32(buffer + 0x24, 0);
        assert_eq!(e.call(0x00864610, &[buffer]).u32(), 0);
    }

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// The argument words of every logged call to `addr`.
    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log: &CallLog = e.call_log.as_ref().expect("call log on");
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The logged call targets, in order, without the test's own call.
    fn targets(e: &Engine) -> Vec<u32> {
        let log: &CallLog = e.call_log.as_ref().expect("call log on");
        log.iter().skip(1).map(|(a, _)| *a).collect()
    }

    fn stub(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| ret(eax));
    }

    /// A 0x30-byte process with guard bytes, the receiver of the stubs.
    fn guarded_process(e: &mut Engine) -> u32 {
        let process = e.mem.alloc(0x30);
        e.mem.write(process, &[0x5a; 0x30]);
        process
    }

    /// Calls a stub with `words` argument words after a guarded receiver and
    /// checks it wrote nothing; answers the result.
    fn call_stub(addr: u32, words: usize) -> Ret {
        let mut e = Engine::new();
        let process = guarded_process(&mut e);
        let mut args = vec![process];
        args.extend((0..words as u32).map(|i| 0x1000 + i));
        logged(&mut e);
        let result = e.call(addr, &args);
        assert_eq!(e.mem.bytes(process, 0x30), [0x5a; 0x30]);
        assert!(targets(&e).is_empty());
        result
    }

    #[test]
    fn folded_getter_answers_the_word_at_0x10() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x14);
        e.mem.set_u32(object + 0x10, 0x1234_5678);
        assert_eq!(e.call(0x0044edb0, &[object]).u32(), 0x1234_5678);
    }

    #[test]
    fn folded_getter_answers_the_address_four_bytes_in() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00717e50, &[0x1000]).u32(), 0x1004);
        assert_eq!(e.call(0x00717e50, &[u32::MAX]).u32(), 3);
    }

    #[test]
    fn last_time_detected_negates_the_largest_float() {
        let mut e = Engine::new();
        e.map(LARGEST_FLOAT, 4);
        e.set_global(LARGEST_FLOAT, f32::MAX);
        let r = e.call(0x008d02d0, &[0, 1, 2]);
        assert_eq!(r.f32(), -f32::MAX);
        // A signalling NaN is quieted by the load, then its sign flipped.
        e.set_global(LARGEST_FLOAT, f32::from_bits(0x7f80_0001));
        let value = base_process_get_last_time_detected(&mut e, Ptr::new(0), 1, 2);
        assert_eq!(value.to_bits(), 0xffc0_0001);
    }

    #[test]
    fn stub_008d02f0_answers_false() {
        assert!(!call_stub(0x008d02f0, 6).bool());
    }

    #[test]
    fn stub_008d0300_answers_false() {
        assert!(!call_stub(0x008d0300, 2).bool());
    }

    #[test]
    fn shared_point_is_copied_into_the_result() {
        let mut e = Engine::new();
        e.map(SHARED_POINT, 12);
        for (i, word) in [0x3f80_0000u32, 0xffc0_0001, 0x7f80_0001]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(SHARED_POINT + 4 * i as u32, *word);
        }
        let result = e.mem.alloc(12);
        assert_eq!(e.call(0x008d0310, &[0, result, 7, 8]).u32(), result);
        assert_eq!(e.mem.u32(result), 0x3f80_0000);
        assert_eq!(e.mem.u32(result + 4), 0xffc0_0001);
        assert_eq!(e.mem.u32(result + 8), 0x7f80_0001);
    }

    #[test]
    fn stub_008d0340_answers_zero() {
        assert_eq!(call_stub(0x008d0340, 7).u32(), 0);
    }

    #[test]
    fn stub_008d0350_does_nothing() {
        call_stub(0x008d0350, 7);
    }

    #[test]
    fn stub_008d0370_answers_false() {
        assert!(!call_stub(0x008d0370, 1).bool());
    }

    #[test]
    fn stub_008d0380_does_nothing() {
        call_stub(0x008d0380, 11);
    }

    #[test]
    fn stub_008d0390_answers_zero_float() {
        let value = call_stub(0x008d0390, 2).f64();
        assert_eq!(value.to_bits(), 0);
    }

    #[test]
    fn clear_current_package_clears_the_embedded_package() {
        let mut e = Engine::new();
        e.register(0x008c_5ef0, |_, _| Ret::default());
        logged(&mut e);
        e.call(0x008d03a0, &[0x2000]);
        assert_eq!(calls_to(&e, 0x008c_5ef0), [vec![0x2004]]);
    }

    #[test]
    fn set_current_procedure_index_stores_through_the_package() {
        let mut e = Engine::new();
        e.register(0x0041_fd00, |_, _| Ret::default());
        logged(&mut e);
        e.call(0x008d03c0, &[0x2000, 5]);
        assert_eq!(calls_to(&e, 0x0041_fd00), [vec![0x2004, 5]]);
    }

    #[test]
    fn procedure_index_getter_asks_the_package() {
        let mut e = Engine::new();
        stub(&mut e, 0x0084_e3a0, 9);
        logged(&mut e);
        assert_eq!(e.call(0x008d03e0, &[0x2000]).u32(), 9);
        assert_eq!(calls_to(&e, 0x0084_e3a0), [vec![0x2004]]);
    }

    #[test]
    fn add_to_current_procedure_index_adds_through_the_package() {
        let mut e = Engine::new();
        e.register(0x0049_bb50, |_, _| Ret::default());
        logged(&mut e);
        e.call(0x008d0400, &[0x2000, u32::MAX]);
        assert_eq!(calls_to(&e, 0x0049_bb50), [vec![0x2004, u32::MAX]]);
    }

    #[test]
    fn stub_008d0420_does_nothing() {
        call_stub(0x008d0420, 8);
    }

    #[test]
    fn running_instance_getter_asks_the_package() {
        let mut e = Engine::new();
        stub(&mut e, 0x0072_6070, 0x3000);
        logged(&mut e);
        assert_eq!(e.call(0x008d0430, &[0x2000]).u32(), 0x3000);
        assert_eq!(calls_to(&e, 0x0072_6070), [vec![0x2004]]);
    }

    /// A process whose vtable slots point at doubles that log their
    /// arguments and answer `answer`.
    fn process_with_slots(e: &mut Engine, slots: &[(u32, u32)], answer: u32) -> u32 {
        let process = e.mem.alloc(0x30);
        let vtable = e.mem.alloc(0x300);
        e.mem.set_u32(process, vtable);
        for &(slot, target) in slots {
            e.mem.set_u32(vtable + slot, target);
            stub(e, target, answer);
        }
        process
    }

    #[test]
    fn procedure_index_running_uses_the_current_index_slot() {
        let mut e = Engine::new();
        let process = process_with_slots(&mut e, &[(0x23c, 0x0070_0001)], 4);
        logged(&mut e);
        assert_eq!(e.call(0x008d0450, &[process]).u32(), 4);
        assert_eq!(calls_to(&e, 0x0070_0001), [vec![process]]);
    }

    #[test]
    fn set_procedure_index_running_uses_the_set_slot() {
        let mut e = Engine::new();
        let process = process_with_slots(&mut e, &[(0x238, 0x0070_0002)], 0);
        logged(&mut e);
        e.call(0x008d0470, &[process, 6]);
        assert_eq!(calls_to(&e, 0x0070_0002), [vec![process, 6]]);
    }

    #[test]
    fn add_to_procedure_index_running_passes_the_second_word() {
        let mut e = Engine::new();
        let process = process_with_slots(&mut e, &[(0x240, 0x0070_0003)], 0);
        logged(&mut e);
        e.call(0x008d04a0, &[process, 11, 22]);
        assert_eq!(calls_to(&e, 0x0070_0003), [vec![process, 22]]);
    }

    #[test]
    fn set_char_controller_releases_its_argument_slot() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let seen_in = seen.clone();
        e.register_double(0x0045_cec0, move |e, a| {
            seen_in.set(e.mem.u32(a[0]));
            Ret::default()
        });
        e.call(0x008d04d0, &[0x2000, 0x4444]);
        assert_eq!(seen.get(), 0x4444);
    }

    #[test]
    fn stub_008d04f0_does_nothing() {
        call_stub(0x008d04f0, 6);
    }

    #[test]
    fn stub_008d0500_does_nothing() {
        call_stub(0x008d0500, 5);
    }

    #[test]
    fn stub_008d0510_answers_false() {
        assert!(!call_stub(0x008d0510, 4).bool());
    }

    #[test]
    fn stub_008d0520_answers_false() {
        assert!(!call_stub(0x008d0520, 3).bool());
    }

    #[test]
    fn stub_008d0530_answers_false() {
        assert!(!call_stub(0x008d0530, 13).bool());
    }

    #[test]
    fn calculate_move_mode_answers_0x101() {
        assert_eq!(call_stub(0x008d0540, 6).u32(), 0x101);
    }

    #[test]
    fn stub_008d0560_answers_zero() {
        assert_eq!(call_stub(0x008d0560, 1).u32(), 0);
    }

    #[test]
    fn stub_008d0570_answers_minus_one() {
        assert_eq!(call_stub(0x008d0570, 3).i32(), -1);
    }

    #[test]
    fn stub_008d0580_answers_zero_float() {
        let value = call_stub(0x008d0580, 3).f64();
        assert_eq!(value.to_bits(), 0);
    }

    #[test]
    fn get_sound_handle_constructs_the_result() {
        let mut e = Engine::new();
        e.register(0x0041_a250, |_, a| ret(a[0]));
        logged(&mut e);
        assert_eq!(e.call(0x008d0590, &[0x2000, 0x5000, 9]).u32(), 0x5000);
        assert_eq!(calls_to(&e, 0x0041_a250), [vec![0x5000]]);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_0() {
        for (flags, frees) in [(0u32, false), (1, true), (2, false), (3, true)] {
            let mut e = Engine::new();
            e.register(0x008c_5e70, |_, _| Ret::default());
            e.register(0x0040_1030, |_, _| Ret::default());
            let process = e.mem.alloc(0x30);
            logged(&mut e);
            assert_eq!(e.call(0x008d0610, &[process, flags]).u32(), process);
            assert_eq!(e.mem.u32(process), BASE_PROCESS_VTABLE);
            assert_eq!(calls_to(&e, 0x008c_5e70), [vec![process + 4]]);
            let freed = calls_to(&e, 0x0040_1030);
            assert_eq!(freed.len(), usize::from(frees));
            if frees {
                assert_eq!(freed[0], [process]);
            }
        }
    }

    #[test]
    fn destructor_body_stores_the_vtable_and_destroys_the_package() {
        let mut e = Engine::new();
        e.register(0x008c_5e70, |_, _| Ret::default());
        let process = e.mem.alloc(0x30);
        e.mem.set_u32(process, 0x1234);
        logged(&mut e);
        e.call(0x008d0640, &[process]);
        assert_eq!(e.mem.u32(process), BASE_PROCESS_VTABLE);
        assert_eq!(targets(&e), [0x008c_5e70]);
    }

    /// `Copy`'s doubles: casts answer from a table of (object, type,
    /// result) rows; the level copies and the cached-values copy only log.
    fn copy_doubles(e: &mut Engine, casts: Vec<(u32, u32, u32)>) {
        e.map(ZERO_DOUBLE, 8);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            assert_eq!((a[1], a[2], a[4]), (0, TYPE_BASE_PROCESS, 0));
            let hit = casts
                .iter()
                .find(|(object, to, _)| *object == a[0] && *to == a[3]);
            ret(hit.map_or(0, |c| c.2))
        });
        for copy in [
            0x008d_9fa0u32,
            0x0091_4c60,
            0x0092_ca70,
            0x0090_7510,
            0x0086_75e0,
        ] {
            e.register(copy, |_, _| Ret::default());
        }
        e.register(0x0070_2000, |_, _| Ret::default());
        e.register(0x0070_3000, |_, _| Ret::default());
    }

    /// A high process whose slot +0x1E0 answers `timer` in ST0; its slots
    /// +0x214 and +0x12C are the logging doubles at 00702000 / 00703000.
    fn high_process_with_timer(e: &mut Engine, timer: f64, timer_slot: u32) -> u32 {
        let process = e.mem.alloc(0x30);
        let vtable = e.mem.alloc(0x300);
        e.mem.set_u32(process, vtable);
        e.mem.set_u32(vtable + 0x1e0, timer_slot);
        e.mem.set_u32(vtable + 0x214, 0x0070_2000);
        e.mem.set_u32(vtable + 0x12c, 0x0070_3000);
        e.register_double(timer_slot, move |_, _| Ret {
            st0: timer,
            ..Ret::default()
        });
        process
    }

    #[test]
    fn copy_skips_a_null_source() {
        let mut e = Engine::new();
        copy_doubles(&mut e, vec![]);
        logged(&mut e);
        e.call(0x008d0660, &[0x2000, 0]);
        assert!(targets(&e).is_empty());
    }

    #[test]
    fn copy_resets_high_processes_and_copies_shared_levels() {
        let mut e = Engine::new();
        let other = high_process_with_timer(&mut e, 1.5, 0x0070_1000);
        let this = high_process_with_timer(&mut e, f64::NAN, 0x0070_1004);
        let other_cached = e.mem.alloc(0x48);
        let this_cached = e.mem.alloc(0x48);
        e.mem.set_u32(other + 0x2c, other_cached);
        e.mem.set_u32(this + 0x2c, this_cached);
        let casts = vec![
            (other, TYPE_HIGH_PROCESS, other),
            (other, TYPE_MIDDLE_HIGH_PROCESS, other + 1),
            (other, TYPE_LOW_PROCESS, other + 3),
            (this, TYPE_HIGH_PROCESS, this),
            (this, TYPE_MIDDLE_HIGH_PROCESS, this + 1),
            (this, TYPE_MIDDLE_LOW_PROCESS, this + 2),
        ];
        copy_doubles(&mut e, casts);
        logged(&mut e);
        e.call(0x008d0660, &[this, other]);
        let casts: Vec<(u32, u32)> = calls_to(&e, RT_DYNAMIC_CAST)
            .iter()
            .map(|a| (a[0], a[3]))
            .collect();
        let levels = [
            TYPE_HIGH_PROCESS,
            TYPE_MIDDLE_HIGH_PROCESS,
            TYPE_MIDDLE_LOW_PROCESS,
            TYPE_LOW_PROCESS,
        ];
        let expected: Vec<(u32, u32)> = [other, this]
            .iter()
            .flat_map(|&p| levels.iter().map(move |&t| (p, t)))
            .collect();
        assert_eq!(casts, expected);
        // Only the source's timer (1.5 > 0.0) resets it; NaN does not.
        assert_eq!(calls_to(&e, 0x0070_2000), [vec![other]]);
        assert_eq!(calls_to(&e, 0x0070_3000), [vec![other, 0]]);
        assert_eq!(calls_to(&e, 0x008d_9fa0), [vec![this, other]]);
        assert_eq!(calls_to(&e, 0x0091_4c60), [vec![this + 1, other + 1]]);
        assert!(calls_to(&e, 0x0092_ca70).is_empty());
        assert!(calls_to(&e, 0x0090_7510).is_empty());
        assert_eq!(calls_to(&e, 0x0086_75e0), [vec![this_cached, other_cached]]);
    }

    #[test]
    fn copy_threshold_is_strictly_greater_and_cached_values_need_both() {
        let mut e = Engine::new();
        let other = high_process_with_timer(&mut e, 0.0, 0x0070_1000);
        let this = e.mem.alloc(0x30);
        let this_cached = e.mem.alloc(0x48);
        e.mem.set_u32(this + 0x2c, this_cached);
        copy_doubles(&mut e, vec![(other, TYPE_HIGH_PROCESS, other)]);
        logged(&mut e);
        e.call(0x008d0660, &[this, other]);
        assert!(calls_to(&e, 0x0070_2000).is_empty());
        assert!(calls_to(&e, 0x008d_9fa0).is_empty());
        assert!(calls_to(&e, 0x0086_75e0).is_empty());
    }

    #[test]
    fn handle_deleted_reference_reaches_the_running_instance() {
        for running in [0u32, 1] {
            let mut e = Engine::new();
            e.register(0x008c_5ec0, |_, _| Ret::default());
            let instance = if running == 0 {
                0
            } else {
                process_with_slots(&mut e, &[(0x4, 0x0070_0004)], 0)
            };
            stub(&mut e, 0x0072_6070, instance);
            logged(&mut e);
            e.call(0x008d0870, &[0x2000, 0x7777]);
            assert_eq!(calls_to(&e, 0x008c_5ec0), [vec![0x2004, 0x7777]]);
            assert_eq!(calls_to(&e, 0x0072_6070), [vec![0x2004]]);
            if running == 0 {
                assert_eq!(targets(&e), [0x008c_5ec0, 0x0072_6070]);
            } else {
                assert_eq!(calls_to(&e, 0x0070_0004), [vec![instance, 0x7777]]);
            }
        }
    }

    #[test]
    fn set_current_package_starts_from_the_initial_target() {
        let mut e = Engine::new();
        stub(&mut e, 0x0067_59e0, 0x9999);
        e.register(0x008c_6080, |_, _| Ret::default());
        logged(&mut e);
        e.call(0x008d08b0, &[0x2000, 0x3000, 0x4000]);
        assert_eq!(calls_to(&e, 0x0067_59e0), [vec![0x3000, 0x4000]]);
        assert_eq!(calls_to(&e, 0x008c_6080), [vec![0x2004, 0x3000, 0x9999]]);
    }

    #[test]
    fn weapon_last_pos_is_the_shared_point() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x008d08f0, &[0x2000]).u32(), SHARED_POINT);
    }

    /// Save/load doubles: blocks and logging switches, buffer positions in
    /// call order, a record (or none) and bytes the reads deliver.
    struct SaveSetup {
        blocks: bool,
        log: bool,
        record: bool,
        positions: Vec<u32>,
        reads: Vec<u32>,
    }

    const FORM_ID: u32 = 0x0001_2345;
    const FORM_FLAGS: u32 = 0x8000_0020;
    const FORM_VERSION: u8 = 0x0f;
    const FORM_NAME: u32 = 0x0055_0000;
    const BUFFER_VERSION: u8 = 0x0e;

    fn save_world(setup: SaveSetup) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(SAVE_LOAD_GAME, 4);
        let save_load = e.mem.alloc(0x100);
        e.set_global(SAVE_LOAD_GAME, save_load);
        let setting = e.mem.alloc(4);
        e.mem.set_u8(setting, u8::from(setup.log));
        let blocks = setup.blocks;
        e.register_double(USE_SAVE_GAME_BLOCKS, move |_, _| ret(u32::from(blocks)));
        e.register_double(SETTING_VALUE, move |_, _| ret(setting));
        let mut positions = setup.positions.into_iter();
        e.register_double(BUFFER_POSITION, move |_, _| {
            ret(positions.next().expect("a buffer position"))
        });
        let record = if setup.record {
            let record = e.mem.alloc(12);
            e.mem.set_u32(record, FORM_ID);
            e.mem.set_u32(record + 5, FORM_FLAGS);
            e.mem.set_u8(record + 9, FORM_VERSION);
            record
        } else {
            0
        };
        stub(&mut e, SAVING_RECORD, record);
        stub(&mut e, LOADING_RECORD, record);
        let form = e.mem.alloc(4);
        let form_vtable = e.mem.alloc(0x134);
        e.mem.set_u32(form, form_vtable);
        e.mem.set_u32(form_vtable + 0x130, 0x0070_0130);
        stub(&mut e, 0x0070_0130, FORM_NAME);
        stub(&mut e, FORM_FOR_ID, form);
        stub(&mut e, CURRENT_VERSION, 0xffff_ff00 | BUFFER_VERSION as u32);
        e.register(WRITE_BYTES, |_, _| Ret::default());
        let mut reads = setup.reads.into_iter();
        e.register_double(READ_BYTES, move |e, a| {
            if let Some(value) = reads.next() {
                let bytes = value.to_le_bytes();
                e.mem.write(a[1], &bytes[..a[2] as usize]);
            }
            Ret::default()
        });
        e.register(ERROR_MESSAGE, |_, _| Ret::default());
        e.register(SAVELOAD_WARNING, |_, _| Ret::default());
        (e, save_load)
    }

    #[test]
    fn save_size_counts_header_package_and_clock() {
        for (blocks, package, size) in [
            (true, 0x20u32, 0x2au16),
            (false, 0x20, 0x24),
            (true, 0xfffe, 8),
        ] {
            let (mut e, _) = save_world(SaveSetup {
                blocks,
                log: false,
                record: false,
                positions: vec![],
                reads: vec![],
            });
            stub(&mut e, 0x008c_61a0, 0xabcd_0000 | package);
            logged(&mut e);
            assert_eq!(e.call(0x008d0900, &[0x2000, 7, 8]).u16(), size);
            assert_eq!(calls_to(&e, 0x008c_61a0), [vec![0x2004, 7]]);
            assert!(calls_to(&e, ERROR_MESSAGE).is_empty());
        }
    }

    #[test]
    fn save_size_message_names_the_record_when_there_is_one() {
        for record in [true, false] {
            let (mut e, _) = save_world(SaveSetup {
                blocks: true,
                log: true,
                record,
                positions: vec![],
                reads: vec![],
            });
            stub(&mut e, 0x008c_61a0, 0x10);
            logged(&mut e);
            assert_eq!(e.call(0x008d0900, &[0x2000, 7, 8]).u16(), 0x1a);
            let expected = if record {
                vec![
                    SAVE_SIZE_FORM_FORMAT,
                    0x1a,
                    FORM_ID,
                    FORM_NAME,
                    FORM_FLAGS,
                    0xe8,
                    SOURCE_FILE,
                ]
            } else {
                vec![SAVE_SIZE_FORMAT, 0x1a, 0xe8, SOURCE_FILE]
            };
            assert_eq!(calls_to(&e, ERROR_MESSAGE), [expected]);
        }
    }

    #[test]
    fn save_game_writes_the_block_and_patches_its_length() {
        let (mut e, save_load) = save_world(SaveSetup {
            blocks: true,
            log: false,
            record: false,
            positions: vec![],
            reads: vec![],
        });
        let buffer = e.mem.alloc(0x40);
        // The placeholder position is where the length is patched.
        let mut positions = vec![0x100, buffer + 4, buffer + 0x30].into_iter();
        e.register_double(BUFFER_POSITION, move |_, _| ret(positions.next().unwrap()));
        e.register(0x008c_6250, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        logged(&mut e);
        e.call(0x008d0a10, &[this, 0x6000, 9]);
        let writes = calls_to(&e, WRITE_BYTES);
        assert_eq!(writes.len(), 3);
        assert_eq!((writes[0][0], writes[0][2]), (save_load, 4));
        assert_eq!((writes[1][0], writes[1][2]), (save_load, 2));
        assert_eq!(writes[2], [save_load, this + 0x1c, 4]);
        assert_eq!(calls_to(&e, 0x008c_6250), [vec![this + 4, 0x6000]]);
        assert_eq!(e.mem.u16(buffer + 4), 0x2c);
        assert!(calls_to(&e, SAVELOAD_WARNING).is_empty());
        assert!(calls_to(&e, ERROR_MESSAGE).is_empty());
    }

    #[test]
    fn save_game_tag_is_kolb() {
        let (mut e, _) = save_world(SaveSetup {
            blocks: true,
            log: false,
            record: false,
            positions: vec![],
            reads: vec![],
        });
        let buffer = e.mem.alloc(0x10);
        let mut positions = vec![0, buffer, buffer + 2].into_iter();
        e.register_double(BUFFER_POSITION, move |_, _| ret(positions.next().unwrap()));
        let tags = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let tags_in = tags.clone();
        e.register_double(WRITE_BYTES, move |e, a| {
            tags_in.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        e.register(0x008c_6250, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x1c, 0x4120_0000);
        e.call(0x008d0a10, &[this, 0, 0]);
        assert_eq!(tags.borrow()[0], BLOCK_TAG);
        assert_eq!(tags.borrow()[1] & 0xffff, 0);
        assert_eq!(tags.borrow()[2], 0x4120_0000);
    }

    #[test]
    fn save_game_logs_and_warns_about_oversized_blocks() {
        for record in [true, false] {
            let (mut e, _) = save_world(SaveSetup {
                blocks: true,
                log: true,
                record,
                positions: vec![],
                reads: vec![],
            });
            let buffer = e.mem.alloc(0x10);
            // start, start again (logging), placeholder, end (log), end.
            let end = buffer + 0x1_0000;
            let mut positions = vec![1, 0x10, buffer, end, end].into_iter();
            e.register_double(BUFFER_POSITION, move |_, _| ret(positions.next().unwrap()));
            e.register(0x008c_6250, |_, _| Ret::default());
            let this = e.mem.alloc(0x30);
            logged(&mut e);
            e.call(0x008d0a10, &[this, 0, 0]);
            let written = end - 0x10;
            let expected = if record {
                vec![
                    SAVE_GAME_FORM_FORMAT,
                    written,
                    FORM_ID,
                    FORM_NAME,
                    FORM_FLAGS,
                    0xf9,
                    SOURCE_FILE,
                ]
            } else {
                vec![SAVE_GAME_FORMAT, written, 0xf9, SOURCE_FILE]
            };
            assert_eq!(calls_to(&e, ERROR_MESSAGE), [expected]);
            assert_eq!(
                calls_to(&e, SAVELOAD_WARNING),
                [vec![BLOCK_TOO_LARGE_FORMAT, SOURCE_FILE, 0xf9]]
            );
            assert_eq!(e.mem.u16(buffer), 0);
        }
    }

    #[test]
    fn save_game_without_blocks_writes_package_and_clock_only() {
        let (mut e, save_load) = save_world(SaveSetup {
            blocks: false,
            log: false,
            record: false,
            positions: vec![0],
            reads: vec![],
        });
        e.register(0x008c_6250, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        logged(&mut e);
        e.call(0x008d0a10, &[this, 0x6000, 0]);
        assert_eq!(calls_to(&e, WRITE_BYTES), [vec![save_load, this + 0x1c, 4]]);
        assert_eq!(calls_to(&e, BUFFER_POSITION).len(), 1);
    }

    fn load_world(record: bool, tag: u32, length: u16, start: u32, end: u32) -> (Engine, u32) {
        let (mut e, _) = save_world(SaveSetup {
            blocks: true,
            log: false,
            record,
            positions: vec![start, end],
            reads: vec![tag, length as u32],
        });
        e.register(0x008c_6320, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        logged(&mut e);
        e.call(0x008d0bc0, &[this, 0x6000, 0x7000, 9]);
        (e, this)
    }

    #[test]
    fn load_game_reads_the_block_and_accepts_an_exact_length() {
        let (e, this) = load_world(true, BLOCK_TAG, 0x20, 0x100, 0x120);
        let save_load: u32 = e.global(SAVE_LOAD_GAME);
        let reads = calls_to(&e, READ_BYTES);
        assert_eq!(reads.len(), 3);
        assert_eq!(reads[0][2], 4);
        assert_eq!(reads[1][2], 2);
        assert_eq!(reads[2], [save_load, this + 0x1c, 4]);
        assert_eq!(calls_to(&e, 0x008c_6320), [vec![this + 4, 0x6000, 0x7000]]);
        assert!(calls_to(&e, SAVELOAD_WARNING).is_empty());
        // The record's form is still looked up before the comparison.
        assert_eq!(calls_to(&e, FORM_FOR_ID), [vec![FORM_ID]]);
    }

    #[test]
    fn load_game_warns_about_a_bad_header() {
        for record in [true, false] {
            let (e, _) = load_world(record, 0x1234_5678, 0x20, 0x100, 0x120);
            let warnings = calls_to(&e, SAVELOAD_WARNING);
            let expected = if record {
                vec![
                    BAD_HEADER_FORM_FORMAT,
                    SOURCE_FILE,
                    0x104,
                    FORM_ID,
                    FORM_NAME,
                    FORM_VERSION as u32,
                    FORM_FLAGS,
                ]
            } else {
                vec![BAD_HEADER_FORMAT, SOURCE_FILE, 0x104, BUFFER_VERSION as u32]
            };
            assert_eq!(warnings, [expected]);
        }
    }

    #[test]
    fn load_game_warns_about_overruns_and_underruns() {
        let cases = [
            (true, 0x130u32, OVERRUN_FORM_FORMAT, 0x10u32),
            (true, 0x118, UNDERRUN_FORM_FORMAT, 0x8),
            (false, 0x130, OVERRUN_FORMAT, 0x10),
            (false, 0x118, UNDERRUN_FORMAT, 0x8),
        ];
        for (record, end, format, bytes) in cases {
            let (e, _) = load_world(record, BLOCK_TAG, 0x20, 0x100, end);
            let expected = if record {
                vec![
                    format,
                    bytes,
                    SOURCE_FILE,
                    0x109,
                    FORM_ID,
                    FORM_NAME,
                    FORM_VERSION as u32,
                    FORM_FLAGS,
                ]
            } else {
                vec![format, bytes, SOURCE_FILE, 0x109, BUFFER_VERSION as u32]
            };
            assert_eq!(calls_to(&e, SAVELOAD_WARNING), [expected]);
        }
    }

    #[test]
    fn load_game_without_blocks_reads_package_and_clock_only() {
        let (mut e, save_load) = save_world(SaveSetup {
            blocks: false,
            log: false,
            record: true,
            positions: vec![],
            reads: vec![],
        });
        e.register(0x008c_6320, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        logged(&mut e);
        e.call(0x008d0bc0, &[this, 1, 2, 3]);
        assert_eq!(calls_to(&e, READ_BYTES), [vec![save_load, this + 0x1c, 4]]);
        assert_eq!(
            targets(&e),
            [
                USE_SAVE_GAME_BLOCKS,
                0x008c_6320,
                READ_BYTES,
                USE_SAVE_GAME_BLOCKS
            ]
        );
    }

    #[test]
    fn init_load_game_passes_the_first_two_words() {
        let mut e = Engine::new();
        e.register(0x008c_64d0, |_, _| Ret::default());
        logged(&mut e);
        e.call(0x008d0e60, &[0x2000, 1, 2, 3]);
        assert_eq!(calls_to(&e, 0x008c_64d0), [vec![0x2004, 1, 2]]);
    }

    fn finish_init_world(idle: u32, running: u32, actor: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(PLAYER_CHARACTER, 4);
        e.set_global(PLAYER_CHARACTER, 0x0090_0000u32);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            assert_eq!(&a[1..], [0, TYPE_MOBILE_OBJECT, TYPE_ACTOR, 0]);
            ret(actor)
        });
        let process = e.mem.alloc(0x30);
        let vtable = e.mem.alloc(0x300);
        e.mem.set_u32(process, vtable);
        e.mem.set_u32(vtable + 0x22c, 0x0070_022c);
        e.mem.set_u32(vtable + 0x280, 0x0070_0280);
        e.mem.set_u32(vtable + 0x284, 0x0070_0284);
        stub(&mut e, 0x0070_022c, idle);
        stub(&mut e, 0x0070_0280, running);
        e.register(0x0070_0284, |_, _| Ret::default());
        e.register(0x008a_6ce0, |_, _| Ret::default());
        logged(&mut e);
        e.call(0x008d0e80, &[process, 1, 2, 0x0080_0000]);
        (e, process)
    }

    #[test]
    fn finish_init_load_game_reevaluates_an_idle_actor() {
        let (e, process) = finish_init_world(0, u32::MAX, 0x0091_0000);
        assert_eq!(calls_to(&e, RT_DYNAMIC_CAST)[0][0], 0x0080_0000);
        assert_eq!(calls_to(&e, 0x0070_0284), [vec![process, 0]]);
        assert_eq!(calls_to(&e, 0x008a_6ce0), [vec![0x0091_0000, 1, 1]]);
        for (idle, running, actor) in [
            (0, u32::MAX, 0),
            (0, u32::MAX, 0x0090_0000),
            (1, u32::MAX, 0x0091_0000),
            (0, 0, 0x0091_0000),
        ] {
            let (e, _) = finish_init_world(idle, running, actor);
            assert!(calls_to(&e, 0x0070_0284).is_empty());
            assert!(calls_to(&e, 0x008a_6ce0).is_empty());
        }
    }

    #[test]
    fn revert_passes_the_first_word() {
        let mut e = Engine::new();
        e.register(0x008c_65a0, |_, _| Ret::default());
        logged(&mut e);
        e.call(0x008d0f10, &[0x2000, 1, 2]);
        assert_eq!(calls_to(&e, 0x008c_65a0), [vec![0x2004, 1]]);
    }

    #[test]
    fn save_game_ov2_writes_three_fields_then_the_package() {
        let mut e = Engine::new();
        e.register(0x0086_5e50, |_, _| Ret::default());
        e.register(0x008c_65d0, |_, _| Ret::default());
        logged(&mut e);
        e.call(0x008d0f30, &[0x2000, 0x6000]);
        assert_eq!(
            calls_to(&e, 0x0086_5e50),
            [
                vec![0x6000, 0x201c, 4, 0],
                vec![0x6000, 0x2020, 4, 0],
                vec![0x6000, 0x2024, 4, 0]
            ]
        );
        assert_eq!(calls_to(&e, 0x008c_65d0), [vec![0x2004, 0x6000]]);
        assert_eq!(targets(&e).last(), Some(&0x008c_65d0));
    }

    #[test]
    fn load_game_ov2_reads_the_hour_and_date_from_version_6() {
        for (version, extra) in [
            (5u32, false),
            (6, true),
            (0xffff_ff06, true),
            (0x105, false),
        ] {
            let mut e = Engine::new();
            e.register(0x0086_4980, |_, _| Ret::default());
            e.register(0x008c_6730, |_, _| Ret::default());
            let buffer = e.mem.alloc(4);
            let vtable = e.mem.alloc(4);
            e.mem.set_u32(buffer, vtable);
            e.mem.set_u32(vtable, 0x0070_0000);
            stub(&mut e, 0x0070_0000, version);
            logged(&mut e);
            e.call(0x008d0f90, &[0x2000, buffer]);
            let mut expected = vec![vec![buffer, 0x201c, 4]];
            if extra {
                expected.push(vec![buffer, 0x2020, 4]);
                expected.push(vec![buffer, 0x2024, 4]);
            }
            assert_eq!(calls_to(&e, 0x0086_4980), expected);
            assert_eq!(calls_to(&e, 0x008c_6730), [vec![0x2004, buffer]]);
        }
    }

    #[test]
    fn init_load_game_ov2_clears_cached_flags_when_cached() {
        for cached in [false, true] {
            let mut e = Engine::new();
            e.register(0x008c_6970, |_, _| Ret::default());
            let this = e.mem.alloc(0x30);
            let values = e.mem.alloc(0x48);
            e.mem.set_u32(values + 0x44, 0xffff_ffff);
            if cached {
                e.mem.set_u32(this + 0x2c, values);
            }
            logged(&mut e);
            e.call(0x008d1000, &[this, 0x6000]);
            assert_eq!(calls_to(&e, 0x008c_6970), [vec![this + 4, 0x6000]]);
            let flags = if cached { 0 } else { 0xffff_ffff };
            assert_eq!(e.mem.u32(values + 0x44), flags);
        }
    }

    #[test]
    fn cached_values_flags_are_cleared() {
        let mut e = Engine::new();
        let values = e.mem.alloc(0x48);
        e.mem.write(values, &[0xff; 0x48]);
        e.call(0x008d1030, &[values]);
        assert_eq!(e.mem.u32(values + 0x44), 0);
        assert_eq!(e.mem.u32(values + 0x40), 0xffff_ffff);
    }

    #[test]
    fn finish_load_game_finishes_the_package() {
        let mut e = Engine::new();
        e.register(0x008c_6b20, |_, _| Ret::default());
        logged(&mut e);
        e.call(0x008d1050, &[0x2000, 0x6000]);
        assert_eq!(calls_to(&e, 0x008c_6b20), [vec![0x2004, 0x6000]]);
    }

    #[test]
    fn revert_ov2_resets_hour_and_date() {
        for (cached, hour_bits, stored_bits) in [
            (true, 0xbf80_0000u32, 0xbf80_0000u32),
            (false, 0x7f80_0001, 0x7fc0_0001),
        ] {
            let mut e = Engine::new();
            e.map(MINUS_ONE_FLOAT, 4);
            e.mem.set_u32(MINUS_ONE_FLOAT, hour_bits);
            e.register(0x008c_6c80, |_, _| Ret::default());
            let this = e.mem.alloc(0x30);
            let values = e.mem.alloc(0x48);
            e.mem.set_u32(values + 0x44, 7);
            e.mem.set_u32(this + 0x24, 0x1234);
            if cached {
                e.mem.set_u32(this + 0x2c, values);
            }
            logged(&mut e);
            e.call(0x008d1070, &[this, 0x6000]);
            assert_eq!(e.mem.u32(this + 0x20), stored_bits);
            assert_eq!(e.mem.u32(this + 0x24), 0);
            assert_eq!(calls_to(&e, 0x008c_6c80), [vec![this + 4, 0x6000]]);
            assert_eq!(e.mem.u32(values + 0x44), if cached { 0 } else { 7 });
        }
    }

    #[test]
    fn breath_timer_is_the_global_float() {
        let mut e = Engine::new();
        e.map(BREATH_TIMER_FLOAT, 4);
        e.set_global(BREATH_TIMER_FLOAT, 20.0f32);
        assert_eq!(e.call(0x008d10c0, &[0x2000]).f32(), 20.0);
        e.mem.set_u32(BREATH_TIMER_FLOAT, 0xff80_0001);
        let value = base_process_get_breath_timer(&mut e, Ptr::new(0));
        assert_eq!(value.to_bits(), 0xffc0_0001);
    }
}
