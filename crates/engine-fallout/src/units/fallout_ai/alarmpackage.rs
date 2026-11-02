//! `fallout/ai/alarmpackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds `Crime` (a reference-counted record of one crime: who did
//! it, to what, who knows of it) and `AlarmPackage` (a `TESPackage` with a
//! list of crimes at +0x80), with their save-game code.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `Crime` (Xbox PDB), 0x3C bytes (the PC offsets are the Xbox ones).
    pub struct Crime: 0x3c {
        /// `iRefCount` (Xbox PDB).
        0x00 iRefCount: u32,
        /// `ecrime` (Xbox PDB): `PackageNS::CRIME_TYPE`.
        0x04 ecrime: u32,
        /// `pcrimetarget` (Xbox PDB): `TESObjectREFR*`.
        0x08 pcrimetarget: u32,
        /// `pcriminal` (Xbox PDB): `Actor*`.
        0x0C pcriminal: u32,
        /// `breported` (Xbox PDB).
        0x10 breported: u8,
        /// `pObjectStolen` (Xbox PDB): `TESBoundObject*`.
        0x14 pObjectStolen: u32,
        /// `inumberstolen` (Xbox PDB).
        0x18 inumberstolen: u32,
        /// `pOwnership` (Xbox PDB): `TESForm*`.
        0x24 pOwnership: u32,
        /// `iCrimeNumber` (Xbox PDB).
        0x28 iCrimeNumber: u32,
        /// `iCrimeStamp` (Xbox PDB).
        0x30 iCrimeStamp: u32,
        /// `fCrimeTimeStamp` (Xbox PDB): a `CombatTimeStamp` (one `float`).
        0x34 fCrimeTimeStamp: u32,
        /// `pListofEnemyFactions` (Xbox PDB): `BSSimpleList<TESFaction*>*`.
        0x38 pListofEnemyFactions: u32,
    }
}

/// `pActorsknowofCrime` (Xbox PDB): the `BSSimpleList<Actor*>` at +0x1C.
const KNOWN_ACTORS: u32 = 0x1c;
/// `bguardrespond` (Xbox PDB): the byte at +0x2C.
const GUARD_RESPOND: u32 = 0x2c;
/// `AlarmPackage::Crimes` (Xbox PDB +0x90; +0x80 on PC, where `TESForm` is
/// 0x10 smaller): a `BSSimpleList<Crime*>`.
const CRIMES: u32 = 0x80;
/// `AlarmPackage`'s vtable (`01092bdc`, by its RTTI locator).
const ALARM_PACKAGE_VTABLE: u32 = 0x0109_2bdc;

/// `operator new` (`00401000`) and `operator delete` (`00401030`), `cdecl`.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The list code (`BSSimpleList`, all `thiscall`): constructor
/// (`0096a2d0`), node item address (`006815c0`), next node (`00726070`; the
/// same body reads +4 of a `Crime`, its type), empty test (`008256d0`),
/// count (`005ae380`), contains (`005f65d0(item pointer)`), add
/// (`005ae3d0(item pointer)`), remove (`00905330(item pointer)`), remove
/// all (`00470470`), destructor (`0046ffb0`), scalar deleting destructor
/// (`004702f0(flags)`), set a node's item (`00726c60(item pointer)`) and
/// remove the head (`0063f7b0`).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const LIST_EMPTY: u32 = 0x0082_56d0;
const LIST_COUNT: u32 = 0x005a_e380;
const LIST_CONTAINS: u32 = 0x005f_65d0;
const LIST_ADD: u32 = 0x005a_e3d0;
const LIST_REMOVE: u32 = 0x0090_5330;
const LIST_REMOVE_ALL: u32 = 0x0047_0470;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
const LIST_DELETE: u32 = 0x0047_02f0;
const NODE_SET_ITEM: u32 = 0x0072_6c60;
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// The process lists (`011e0e80`): the crime counter word at +0x103B8
/// ([`fn_009eb620`]), `009726e0(crime)`, `009722e0(faction, crime)`,
/// `ProcessLists::GetActorsProcessCrimeAlarm` (engine map; `009719e0`),
/// `ProcessLists::GetCrimeIndex` (`00972740(type, crime)`) and
/// `ProcessLists::GetCrimeByIndex` (`009727c0(type, index)`).
const PROCESS_LISTS: u32 = 0x011e_0e80;
const LISTS_FORGET_CRIME: u32 = 0x0097_26e0;
const LISTS_FACTION_CHECK: u32 = 0x0097_22e0;
const LISTS_CRIME_ALARM_ACTORS: u32 = 0x0097_19e0;
const LISTS_CRIME_INDEX: u32 = 0x0097_2740;
const LISTS_CRIME_BY_INDEX: u32 = 0x0097_27c0;
/// The calendar (`011de7b8`): `Calendar::GetMonth`, `GetDay` (engine map)
/// and the year (`00867c60`).
const CALENDAR: u32 = 0x011d_e7b8;
const CALENDAR_MONTH: u32 = 0x0086_7d20;
const CALENDAR_DAY: u32 = 0x0086_7d60;
const CALENDAR_YEAR: u32 = 0x0086_7c60;
/// The current combat time (`00435dd0`, `cdecl`, ST0: the `float` at
/// `011f1bf0`), the time stamp setter (`00435de0`, `thiscall(float)`) and
/// constructor (`004300a0`), and `CombatTimeStamp::SaveGame`/`LoadGame`
/// (engine map; `009a5f90`/`009a5ff0`, `thiscall(buffer)`).
const COMBAT_TIME: u32 = 0x0043_5dd0;
const TIME_STAMP_SET: u32 = 0x0043_5de0;
const TIME_STAMP_CONSTRUCT: u32 = 0x0043_00a0;
const TIME_STAMP_SAVE: u32 = 0x009a_5f90;
const TIME_STAMP_LOAD: u32 = 0x009a_5ff0;
/// `TESObjectREFR::SetTargeted` (engine map; `00564db0`, `thiscall(bool)`).
const SET_TARGETED: u32 = 0x0056_4db0;
/// The test `004226e0` on the object at `011c3f2c`: the destructor's
/// clean-up runs only when it is false.
const CLEANUP_OWNER: u32 = 0x011c_3f2c;
const CLEANUP_TEST: u32 = 0x0042_26e0;
/// An actor's extra data (`005d43c0`) and its
/// `ExtraDataList::GetPlayerCrimeList` (engine map; `0041cf00`).
const EXTRA_DATA: u32 = 0x005d_43c0;
const CRIME_LIST: u32 = 0x0041_cf00;
/// `0047eb90(0)` on a faction the process lists let go.
const FACTION_RELEASE: u32 = 0x0047_eb90;
/// `0044ddc0` (`thiscall`: the word at +8, the crime's target).
const CRIME_TARGET: u32 = 0x0044_ddc0;
/// `Actor::IsAlarmed` and `Actor::EndInterruptPackage` (engine map;
/// `008a61b0`, `00881680(word)`).
const IS_ALARMED: u32 = 0x008a_61b0;
const END_INTERRUPT_PACKAGE: u32 = 0x0088_1680;
/// The setting at `011cd39c` and its value address (`0043d4d0`): the stamp
/// age after which a crime goes.
const CRIME_AGE_SETTING: u32 = 0x011c_d39c;
const SETTING_INT: u32 = 0x0043_d4d0;
/// The save-load game (`011de45c`) and its calls: blocks in use
/// (`TESSaveLoadGame::UseSaveGameBlocks`, engine map; `00862110`), the
/// write position (`00825c00`), write and read (`008579b0` and `008579e0`,
/// `thiscall(pointer, size)`), `TESSaveLoadGame::SaveNumericID` (engine map;
/// `00857a10(pointer, size)`), the record being written (`004fd3e0`) or read
/// (`004fd3c0`), and `008df040` (a byte the load messages print).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const USE_BLOCKS: u32 = 0x0086_2110;
const WRITE_POSITION: u32 = 0x0082_5c00;
const WRITE: u32 = 0x0085_79b0;
const READ: u32 = 0x0085_79e0;
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
const SAVE_RECORD: u32 = 0x004f_d3e0;
const LOAD_RECORD: u32 = 0x004f_d3c0;
const LOAD_BYTE: u32 = 0x008d_f040;
/// The save debug setting (`011de4e8`, its byte through `00408d60`).
const SAVE_DEBUG_SETTING: u32 = 0x011d_e4e8;
const SETTING_VALUE: u32 = 0x0040_8d60;
/// A form's id (`0084e3a0`, `thiscall`) and the form of an id
/// (`004839c0`, `cdecl(id)`).
const FORM_ID: u32 = 0x0084_e3a0;
const FORM_OF_ID: u32 = 0x0048_39c0;
/// `"KOLB"` read as a word: the block tag.
const BLOCK_TAG: u32 = 0x424c_4f4b;
/// The debug prints: `Error` (engine map; `0040fbe0`) and `005b5e40`, both
/// `cdecl(format, ...)`; the source file name (`01092b94`) and the formats
/// `"GetSaveSize(): %-5i for form %08X %s with flags %08X..."` (`01012cb0`),
/// `"GetSaveSize(): %-5i ending at line %i in file %s"` (`01012c78`),
/// `"SaveGame(): %-5i for form %08X ..."` (`010153a0`), `"SaveGame(): %-5i
/// ending at line %i in file %s"` (`0101536c`), `"SAVELOAD: Save Game Block
/// in file %s on line %i ..."` (`01015318`), the two `"(LoadGame Buffer
/// error) Block Header"` messages (`01015718` with a record, `010156a8`
/// without) and the over- and underrun messages (`01015588`, `01015500`
/// with a record; `010154a0`, `01015440` without).
const ERROR_PRINT: u32 = 0x0040_fbe0;
const DEBUG_PRINT: u32 = 0x005b_5e40;
const SOURCE_FILE: u32 = 0x0109_2b94;
const SIZE_FORMAT: u32 = 0x0101_2cb0;
const SIZE_END_FORMAT: u32 = 0x0101_2c78;
const SAVE_FORMAT: u32 = 0x0101_53a0;
const SAVE_END_FORMAT: u32 = 0x0101_536c;
const BLOCK_TOO_BIG: u32 = 0x0101_5318;
const BAD_HEADER_FORMAT: u32 = 0x0101_5718;
const BAD_HEADER_END_FORMAT: u32 = 0x0101_56a8;
const OVERRUN_FORMAT: u32 = 0x0101_5588;
const UNDERRUN_FORMAT: u32 = 0x0101_5500;
const OVERRUN_END_FORMAT: u32 = 0x0101_54a0;
const UNDERRUN_END_FORMAT: u32 = 0x0101_5440;
/// The save buffer (`BGSSaveGameBuffer`, engine map): write (`00865e50`,
/// `thiscall(pointer, size, word)`), `SaveFormID_ov2` (`00865df0(form,
/// word)`), `StartVariableSizedValue` (`00865f20`) and
/// `SaveVariableSizedValue_ov2` (`00865ff0(value, mark)`).
const BUFFER_WRITE: u32 = 0x0086_5e50;
const BUFFER_SAVE_FORM: u32 = 0x0086_5df0;
const BUFFER_START_SIZE: u32 = 0x0086_5f20;
const BUFFER_SAVE_SIZE: u32 = 0x0086_5ff0;
/// The load buffer (`BGSLoadGameBuffer`, engine map): read (`00864980`,
/// `thiscall(pointer, size)`), `LoadFormID` (`008648a0`), `LoadFormID_ov2`
/// (`008648e0(pointer)`) and `LoadVariableSizedValue` (`00864a60`).
const BUFFER_READ: u32 = 0x0086_4980;
const BUFFER_LOAD_FORM_ID: u32 = 0x0086_48a0;
const BUFFER_LOAD_FORM: u32 = 0x0086_48e0;
const BUFFER_LOAD_SIZE: u32 = 0x0086_4a60;
/// `__RTDynamicCast` (`00ec43fb`) and the type descriptors of `TESForm`
/// (`01183028`), `TESBoundObject` (`01183108`), `TESFaction` (`01184704`),
/// `TESObjectREFR` (`011841cc`), `Actor` (`011846d4`) and `AlarmPackage`
/// (`011a39c8`), named by their decorated names.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TES_FORM_TYPE: u32 = 0x0118_3028;
const TES_BOUND_OBJECT_TYPE: u32 = 0x0118_3108;
const TES_FACTION_TYPE: u32 = 0x0118_4704;
const TES_OBJECT_REFR_TYPE: u32 = 0x0118_41cc;
const ACTOR_TYPE: u32 = 0x0118_46d4;
const ALARM_PACKAGE_TYPE: u32 = 0x011a_39c8;
/// `TESPackage`'s constructor (`TESPackage::TESPackage`, engine map;
/// `006707c0`), destructor (`006708b0`), copy (`00673290(form)`), save size
/// (`00678fd0`), save (`00679120`), `TESPackage::LoadGame` (`00679340`),
/// `TESPackage::LoadGame_ov2` (`006798c0(buffer)`) and
/// `TESForm::LoadGameDataOLD` (`00484d00(pointer, size)`).
const PACKAGE_CONSTRUCT: u32 = 0x0067_07c0;
const PACKAGE_DESTRUCT: u32 = 0x0067_08b0;
const PACKAGE_COPY: u32 = 0x0067_3290;
const PACKAGE_SAVE_SIZE: u32 = 0x0067_8fd0;
const PACKAGE_SAVE: u32 = 0x0067_9120;
const PACKAGE_LOAD: u32 = 0x0067_9340;
const PACKAGE_LOAD_BUFFER: u32 = 0x0067_98c0;
const LOAD_GAME_DATA_OLD: u32 = 0x0048_4d00;
/// The crime list of another package (`006130e0`, `thiscall`).
const PACKAGE_CRIMES: u32 = 0x0061_30e0;

/// The save-load game.
fn save_load_game(e: &mut Engine) -> u32 {
    e.global(SAVE_LOAD_GAME)
}

/// Whether the save game uses blocks.
fn uses_blocks(e: &mut Engine) -> bool {
    let game = save_load_game(e);
    e.call(USE_BLOCKS, &args![game]).bool()
}

/// The save game's write position.
fn write_position(e: &mut Engine) -> u32 {
    let game = save_load_game(e);
    e.call(WRITE_POSITION, &args![game]).u32()
}

/// Writes `size` bytes at `pointer` into the save game.
fn write(e: &mut Engine, pointer: u32, size: u32) {
    let game = save_load_game(e);
    e.call(WRITE, &args![game, pointer, size]);
}

/// Reads `size` bytes into `pointer` from the save game.
fn read(e: &mut Engine, pointer: u32, size: u32) {
    let game = save_load_game(e);
    e.call(READ, &args![game, pointer, size]);
}

/// The save debug setting's byte.
fn save_debug(e: &mut Engine) -> bool {
    let value = e.call(SETTING_VALUE, &args![SAVE_DEBUG_SETTING]).u32();
    e.mem.u8(value) != 0
}

/// The size or save report: `Error(with_record, amount, record id, name,
/// record +5, line, file)` when the save game has a record, else
/// `Error(plain, amount, line, file)`.
fn report_written(e: &mut Engine, amount: u32, line: u32, with_record: u32, plain: u32) {
    let game = save_load_game(e);
    let record = e.call(SAVE_RECORD, &args![game]).u32();
    if record != 0 {
        let id = e.mem.u32(record);
        let form = e.call(FORM_OF_ID, &args![id]).u32();
        let flags = e.mem.u32(record + 5);
        let name = e.vcall(form, 0x130, &[]).u32();
        let id = e.mem.u32(record);
        e.call(
            ERROR_PRINT,
            &args![with_record, amount, id, name, flags, line, SOURCE_FILE],
        );
    } else {
        e.call(ERROR_PRINT, &args![plain, amount, line, SOURCE_FILE]);
    }
}

/// The end of a written block: with blocks, its length (complaining
/// through `005b5e40` when it exceeds 0xFFFF) as a 16-bit word at
/// `length_at`.
fn finish_block(e: &mut Engine, length_at: u32, line: u32) {
    if !uses_blocks(e) {
        return;
    }
    let now = write_position(e);
    if now > length_at.wrapping_add(0xffff) {
        e.call(DEBUG_PRINT, &args![BLOCK_TOO_BIG, SOURCE_FILE, line]);
    }
    e.mem.set_u16(length_at, now.wrapping_sub(length_at) as u16);
}

/// The start of a written block: with blocks, the tag and a 16-bit length
/// placeholder (zero), whose position is returned (0 without blocks).
fn start_block(e: &mut Engine, length: u32) -> u32 {
    if !uses_blocks(e) {
        return 0;
    }
    e.with_stack(4, |e, tag| {
        e.mem.set_u32(tag.addr(), BLOCK_TAG);
        write(e, tag.addr(), 4);
    });
    let position = write_position(e);
    write(e, length, 2);
    position
}

/// The calendar stamp of now: the month shifted by 9, the day (a signed
/// byte), the year shifted by 13, ORed.
fn calendar_stamp(e: &mut Engine) -> u32 {
    let month = e.call(CALENDAR_MONTH, &args![CALENDAR]).u32() << 9;
    let day = e.call(CALENDAR_DAY, &args![CALENDAR]).u8() as i8 as i32 as u32;
    let year = e.call(CALENDAR_YEAR, &args![CALENDAR]).u32() << 13;
    month | day | year
}

/// `__RTDynamicCast(form, 0, TESForm, target, 0)`.
fn cast(e: &mut Engine, form: u32, target: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TES_FORM_TYPE, target, 0u32],
    )
    .u32()
}

/// Removes the word at `item` from the actor's player crime list, when it
/// has one.
fn leave_crime_list(e: &mut Engine, actor: u32, crime: u32) {
    let extra = e.call(EXTRA_DATA, &args![actor]).u32();
    let list = e.call(CRIME_LIST, &args![extra]).u32();
    if list != 0 {
        e.with_stack(4, |e, item| {
            e.mem.set_u32(item.addr(), crime);
            e.call(LIST_REMOVE, &args![list, item]);
        });
    }
}

/// Calls `function(list, &word)` with the word in the game's stack slot.
fn with_word(e: &mut Engine, function: u32, list: u32, word: u32) -> Ret {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), word);
        e.call(function, &args![list, slot])
    })
}

// Translated from 009eb500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009eb500 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `Crime::Crime` (Xbox PDB; `thiscall(type, target, criminal, stolen,
/// count, ownership)`): constructs the known-actors list and the time
/// stamp, a zero reference count, marks `target` targeted
/// (`SetTargeted(1)`, when set), stores the words, clears `breported`,
/// takes the next crime number ([`fn_009eb620`] on `011e0e80`), no enemy
/// factions, the calendar stamp, and the current combat time. Returns
/// `this`.
pub fn crime_crime(
    e: &mut Engine,
    this: Ptr<Crime>,
    crime_type: u32,
    target: u32,
    criminal: u32,
    stolen: u32,
    count: u32,
    ownership: u32,
) -> Ptr<Crime> {
    let base = this.addr();
    e.call(LIST_CONSTRUCT, &args![base + KNOWN_ACTORS]);
    e.call(TIME_STAMP_CONSTRUCT, &args![base + 0x34]);
    e.set(this, Crime::ecrime, crime_type);
    e.set(this, Crime::iRefCount, 0);
    if target != 0 {
        e.call(SET_TARGETED, &args![target, 1u32]);
    }
    e.set(this, Crime::pcrimetarget, target);
    e.set(this, Crime::pcriminal, criminal);
    e.set(this, Crime::breported, 0);
    e.set(this, Crime::pObjectStolen, stolen);
    e.set(this, Crime::inumberstolen, count);
    e.set(this, Crime::pOwnership, ownership);
    let number = fn_009eb620(e, Ptr::new(PROCESS_LISTS));
    e.set(this, Crime::iCrimeNumber, number);
    e.set(this, Crime::pListofEnemyFactions, 0);
    let stamp = calendar_stamp(e);
    e.set(this, Crime::iCrimeStamp, stamp);
    let now = x87_store32(e.call(COMBAT_TIME, &[]).f64());
    let stamp = e.with_stack(4, |e, local| {
        e.call(TIME_STAMP_SET, &args![local, x87_quiet_bits(now)]);
        e.mem.u32(local.addr())
    });
    e.set(this, Crime::fCrimeTimeStamp, stamp);
    this
}

// Translated from 009eb620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, on the process lists): returns the
/// word at +0x103B8 and increments it.
pub fn fn_009eb620(e: &mut Engine, this: Ptr) -> u32 {
    let counter = this.addr() + 0x103b8;
    let value = e.mem.u32(counter);
    e.mem.set_u32(counter, value.wrapping_add(1));
    value
}

// Translated from 009eb660 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009eb660 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `Crime::~Crime` (Xbox PDB): unless the object at `011c3f2c` is null or
/// its `004226e0` holds: clears the known actors
/// ([`crime_clear_actor_know_list`]), has the process lists forget the crime
/// (`009726e0`), takes it off the target's player crime list, releases
/// (`0047eb90(0)`) each enemy faction the process lists' `009722e0` refuses,
/// takes it off the list of the target (`0044ddc0`) when its slot 0x100
/// holds, and walks the actors of `GetActorsProcessCrimeAlarm(crime, 1)`
/// (ending an alarmed actor's interrupt package and taking the crime off its
/// list). That walk never moves to the next node: the exe loops on the first
/// node for as long as it is not null. Then deletes the enemy faction list
/// and empties and destroys the known-actor list.
pub fn crime_destructor(e: &mut Engine, this: Ptr<Crime>) {
    let base = this.addr();
    let owner: u32 = e.global(CLEANUP_OWNER);
    if owner != 0 && !e.call(CLEANUP_TEST, &args![owner]).bool() {
        crime_clear_actor_know_list(e, this);
        e.call(LISTS_FORGET_CRIME, &args![PROCESS_LISTS, base]);
        let target = e.get(this, Crime::pcrimetarget);
        if target != 0 {
            let extra = e.call(EXTRA_DATA, &args![target]).u32();
            if e.call(CRIME_LIST, &args![extra]).u32() != 0 {
                let target = e.get(this, Crime::pcrimetarget);
                let extra = e.call(EXTRA_DATA, &args![target]).u32();
                let list = e.call(CRIME_LIST, &args![extra]).u32();
                with_word(e, LIST_REMOVE, list, base);
            }
        }
        let mut node = e.get(this, Crime::pListofEnemyFactions);
        while node != 0 && !e.call(LIST_EMPTY, &args![node]).bool() {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            let faction = e.mem.u32(slot);
            if !e
                .call(LISTS_FACTION_CHECK, &args![PROCESS_LISTS, faction, base])
                .bool()
            {
                e.call(FACTION_RELEASE, &args![faction, 0u32]);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        let holder = e.call(CRIME_TARGET, &args![base]).u32();
        if holder != 0 && e.vcall(holder, 0x100, &[]).bool() {
            leave_crime_list(e, holder, base);
        }
        let node = e
            .call(LISTS_CRIME_ALARM_ACTORS, &args![PROCESS_LISTS, base, 1u32])
            .u32();
        if node != 0 {
            // The exe's loop tests the same, unchanged node each time round.
            loop {
                let slot = e.call(NODE_ITEM, &args![node]).u32();
                let actor = e.mem.u32(slot);
                if e.call(IS_ALARMED, &args![actor]).bool() {
                    e.call(END_INTERRUPT_PACKAGE, &args![actor, 0u32]);
                }
                leave_crime_list(e, actor, base);
            }
        }
    }
    let factions = e.get(this, Crime::pListofEnemyFactions);
    if factions != 0 {
        e.call(LIST_REMOVE_ALL, &args![factions]);
        let factions = e.get(this, Crime::pListofEnemyFactions);
        if factions != 0 {
            e.call(LIST_DELETE, &args![factions, 1u32]);
        }
    }
    e.call(LIST_REMOVE_ALL, &args![base + KNOWN_ACTORS]);
    e.call(LIST_DESTRUCT, &args![base + KNOWN_ACTORS]);
}

// Translated from 009eb8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Crime::ShouldCrimeBeRemoved` (Xbox PDB): true when the calendar stamp
/// is at least the `011cd39c` setting past the crime's stamp (signed), when
/// the reference count is below 1, when the known-actor list is empty, or
/// when every known actor's slot 0x22C (with 0) holds.
pub fn crime_should_crime_be_removed(e: &mut Engine, this: Ptr<Crime>) -> bool {
    let now = calendar_stamp(e);
    let age = now.wrapping_sub(e.get(this, Crime::iCrimeStamp)) as i32;
    let limit = e.call(SETTING_INT, &args![CRIME_AGE_SETTING]).u32();
    if age >= e.mem.i32(limit) {
        return true;
    }
    if (e.get(this, Crime::iRefCount) as i32) < 1 {
        return true;
    }
    let mut node = this.addr().wrapping_add(KNOWN_ACTORS);
    if node == 0 || e.call(LIST_EMPTY, &args![node]).bool() {
        return true;
    }
    let mut all = true;
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 || !all {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let actor = e.mem.u32(slot);
        if !e.vcall(actor, 0x22c, &args![0u32]).bool() {
            all = false;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    all
}

// Translated from 009eb9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(actor)`): whether `actor` is in the
/// known-actor list (`005f65d0` with the address of the argument).
pub fn fn_009eb9a0(e: &mut Engine, this: Ptr<Crime>, actor: u32) -> bool {
    with_word(e, LIST_CONTAINS, this.addr() + KNOWN_ACTORS, actor).bool()
}

// Translated from 009eb9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(actor)`): adds `actor` to the
/// known-actor list when missing, counting a reference.
pub fn fn_009eb9c0(e: &mut Engine, this: Ptr<Crime>, actor: u32) {
    let list = this.addr() + KNOWN_ACTORS;
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), actor);
        if !e.call(LIST_CONTAINS, &args![list, slot]).bool() {
            e.call(LIST_ADD, &args![list, slot]);
            let count = e.get(this, Crime::iRefCount).wrapping_add(1);
            e.set(this, Crime::iRefCount, count);
        }
    });
}

// Translated from 009eba00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(actor)`): removes `actor` from the
/// known-actor list and drops a reference.
pub fn fn_009eba00(e: &mut Engine, this: Ptr<Crime>, actor: u32) {
    with_word(e, LIST_REMOVE, this.addr() + KNOWN_ACTORS, actor);
    let count = e.get(this, Crime::iRefCount).wrapping_sub(1);
    e.set(this, Crime::iRefCount, count);
}

// Translated from 009eba30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Crime::ClearActorKnowList` (Xbox PDB): takes the crime off the player
/// crime list of every known actor (up to the first empty node) and clears
/// the reference count. The list itself is left as it is.
pub fn crime_clear_actor_know_list(e: &mut Engine, this: Ptr<Crime>) {
    let base = this.addr();
    let mut node = base.wrapping_add(KNOWN_ACTORS);
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let actor = e.mem.u32(slot);
        leave_crime_list(e, actor, base);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.set(this, Crime::iRefCount, 0);
}

// Translated from 009ebab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the number of known actors (0
/// for an empty list).
pub fn fn_009ebab0(e: &mut Engine, this: Ptr<Crime>) -> u32 {
    let list = this.addr() + KNOWN_ACTORS;
    if e.call(LIST_EMPTY, &args![list]).bool() {
        return 0;
    }
    e.call(LIST_COUNT, &args![list]).u32()
}

// Translated from 009ebae0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ebae0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Unnamed in the engine map (`thiscall(faction)`): adds `faction` to the
/// enemy faction list, making the list (8 bytes) first when there is none
/// (null when the allocation fails).
pub fn fn_009ebae0(e: &mut Engine, this: Ptr<Crime>, faction: u32) {
    if e.get(this, Crime::pListofEnemyFactions) == 0 {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let list = if block != 0 {
            e.call(LIST_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.set(this, Crime::pListofEnemyFactions, list);
    }
    let list = e.get(this, Crime::pListofEnemyFactions);
    with_word(e, LIST_ADD, list, faction);
}

// Translated from 009ebb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`; the crime's save size, by its
/// body): 6 for the block header when blocks are used, 0x20 for the fixed
/// words, 4 per known actor and 2 for their count, as a 16-bit size; with
/// the save debug setting it reports the size (`Error`, line 0xF5).
pub fn fn_009ebb80(e: &mut Engine, this: Ptr<Crime>) -> u16 {
    let mut size = 0u16;
    let start = size;
    if uses_blocks(e) {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    size = size.wrapping_add(0x10);
    size = size.wrapping_add(0x10);
    let count = e.call(LIST_COUNT, &args![this.addr() + KNOWN_ACTORS]).u32();
    size = u32::from(size)
        .wrapping_add(count.wrapping_mul(4))
        .wrapping_add(2) as u16;
    if save_debug(e) {
        let amount = i32::from(size).wrapping_sub(i32::from(start)) as u32;
        report_written(e, amount, 0xf5, SIZE_FORMAT, SIZE_END_FORMAT);
    }
    size
}

// Translated from 009ebca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`; the crime's save, by its body):
/// in a block when blocks are used, writes `breported`, `bguardrespond`,
/// the type, the count stolen and the crime number, the ids of the target,
/// criminal, stolen object and ownership (0 for none,
/// `TESSaveLoadGame::SaveNumericID`), then the known actors' ids after a
/// 16-bit count patched in afterwards. With the save debug setting it
/// reports the bytes written (`Error`, line 0x12D); the block's length is
/// patched at its end.
pub fn fn_009ebca0(e: &mut Engine, this: Ptr<Crime>) {
    let base = this.addr();
    e.with_stack(8, |e, words| {
        let length = words.addr();
        let count = length + 4;
        let mut start = write_position(e);
        if save_debug(e) {
            start = write_position(e);
        }
        let length_at = start_block(e, length);
        write(e, base + 0x10, 1);
        write(e, base + GUARD_RESPOND, 1);
        write(e, base + 4, 4);
        write(e, base + 0x18, 4);
        write(e, base + 0x28, 4);
        for field in [0x08, 0x0c, 0x14, 0x24] {
            let form = e.mem.u32(base + field);
            let id = if form != 0 {
                e.call(FORM_ID, &args![form]).u32()
            } else {
                0
            };
            save_numeric_id(e, id);
        }
        e.mem.set_u16(count, 0);
        let count_at = write_position(e);
        write(e, count, 2);
        let mut node = base.wrapping_add(KNOWN_ACTORS);
        while node != 0 && !e.call(LIST_EMPTY, &args![node]).bool() {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            let actor = e.mem.u32(slot);
            let id = e.call(FORM_ID, &args![actor]).u32();
            save_numeric_id(e, id);
            let counted = e.mem.u16(count).wrapping_add(1);
            e.mem.set_u16(count, counted);
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        let counted = e.mem.u16(count);
        e.mem.set_u16(count_at, counted);
        if save_debug(e) {
            let end = write_position(e);
            report_written(
                e,
                end.wrapping_sub(start),
                0x12d,
                SAVE_FORMAT,
                SAVE_END_FORMAT,
            );
        }
        finish_block(e, length_at, 0x12d);
    });
}

/// `TESSaveLoadGame::SaveNumericID(&id, 4)`.
fn save_numeric_id(e: &mut Engine, id: u32) {
    let game = save_load_game(e);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), id);
        e.call(SAVE_NUMERIC_ID, &args![game, slot, 4u32]);
    });
}

// Translated from 009ebfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Crime::SaveGame` (Xbox PDB; `thiscall(buffer)`): writes the reference
/// count, type, `breported`, the count stolen, crime number,
/// `bguardrespond` and stamp (`00865e50`), the time stamp, the target,
/// criminal, stolen object and ownership (`SaveFormID_ov2`), then the known
/// actors and the enemy factions, each as a variable-sized count after its
/// non-null forms.
pub fn crime_save_game(e: &mut Engine, this: Ptr<Crime>, buffer: u32) {
    let base = this.addr();
    for (offset, size) in [
        (0x00, 4u32),
        (0x04, 4),
        (0x10, 1),
        (0x18, 4),
        (0x28, 4),
        (GUARD_RESPOND, 1),
        (0x30, 4),
    ] {
        e.call(BUFFER_WRITE, &args![buffer, base + offset, size, 0u32]);
    }
    e.call(TIME_STAMP_SAVE, &args![base + 0x34, buffer]);
    for field in [0x08, 0x0c, 0x14, 0x24] {
        let form = e.mem.u32(base + field);
        e.call(BUFFER_SAVE_FORM, &args![buffer, form, 0u32]);
    }
    let mut count = 0u32;
    let mark = e.call(BUFFER_START_SIZE, &args![buffer]).u32();
    let mut node = base.wrapping_add(KNOWN_ACTORS);
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let actor = e.mem.u32(slot);
        if actor != 0 {
            e.call(BUFFER_SAVE_FORM, &args![buffer, actor, 0u32]);
            count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(BUFFER_SAVE_SIZE, &args![buffer, count, mark]);
    let mut count = 0u32;
    let mark = e.call(BUFFER_START_SIZE, &args![buffer]).u32();
    let mut node = e.get(this, Crime::pListofEnemyFactions);
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let faction = e.mem.u32(slot);
        if faction != 0 {
            e.call(BUFFER_SAVE_FORM, &args![buffer, faction, 0u32]);
            count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(BUFFER_SAVE_SIZE, &args![buffer, count, mark]);
}

/// A form read by id (`LoadFormID`) and cast to `target`.
fn load_cast(e: &mut Engine, buffer: u32, target: u32) -> u32 {
    let id = e.call(BUFFER_LOAD_FORM_ID, &args![buffer]).u32();
    let form = e.call(FORM_OF_ID, &args![id]).u32();
    cast(e, form, target)
}

// Translated from 009ec1a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ec1a0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `Crime::LoadGame` (Xbox PDB; `thiscall(buffer)`): reads the fields
/// [`crime_save_game`] writes (the target and criminal as form ids, the
/// stolen object cast to `TESBoundObject`, the ownership as an id), adds
/// each known actor id to the list, makes a new enemy faction list (8 bytes)
/// with each form that casts to `TESFaction`, and deletes that list again
/// when it holds none.
pub fn crime_load_game(e: &mut Engine, this: Ptr<Crime>, buffer: u32) {
    let base = this.addr();
    for (offset, size) in [
        (0x00, 4u32),
        (0x04, 4),
        (0x10, 1),
        (0x18, 4),
        (0x28, 4),
        (GUARD_RESPOND, 1),
        (0x30, 4),
    ] {
        e.call(BUFFER_READ, &args![buffer, base + offset, size]);
    }
    e.call(TIME_STAMP_LOAD, &args![base + 0x34, buffer]);
    e.call(BUFFER_LOAD_FORM, &args![buffer, base + 8]);
    e.call(BUFFER_LOAD_FORM, &args![buffer, base + 0xc]);
    let stolen = load_cast(e, buffer, TES_BOUND_OBJECT_TYPE);
    e.set(this, Crime::pObjectStolen, stolen);
    e.call(BUFFER_LOAD_FORM, &args![buffer, base + 0x24]);
    let count = e.call(BUFFER_LOAD_SIZE, &args![buffer]).u32();
    for _ in 0..count {
        e.with_stack(4, |e, slot| {
            e.call(BUFFER_LOAD_FORM, &args![buffer, slot]);
            e.call(LIST_ADD, &args![base + KNOWN_ACTORS, slot]);
        });
    }
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let list = if block != 0 {
        e.call(LIST_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.set(this, Crime::pListofEnemyFactions, list);
    let count = e.call(BUFFER_LOAD_SIZE, &args![buffer]).u32();
    for _ in 0..count {
        let faction = load_cast(e, buffer, TES_FACTION_TYPE);
        if faction != 0 {
            let list = e.get(this, Crime::pListofEnemyFactions);
            with_word(e, LIST_ADD, list, faction);
        }
    }
    let list = e.get(this, Crime::pListofEnemyFactions);
    let slot = e.call(NODE_ITEM, &args![list]).u32();
    if e.mem.u32(slot) == 0 {
        let list = e.get(this, Crime::pListofEnemyFactions);
        if list != 0 {
            e.call(LIST_DELETE, &args![list, 1u32]);
        }
        e.set(this, Crime::pListofEnemyFactions, 0);
    }
}

/// The form of a loaded id, cast to `target` (null for a zero id).
fn resolve(e: &mut Engine, id: u32, target: u32) -> u32 {
    if id == 0 {
        return 0;
    }
    let form = e.call(FORM_OF_ID, &args![id]).u32();
    cast(e, form, target)
}

// Translated from 009ec400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(unused)`; the crime's load fix-up,
/// by its body): turns the loaded ids into forms (the target as a
/// `TESObjectREFR`, the criminal as an `Actor`, the ownership as any
/// form), replaces each known actor id by its actor and removes the ids
/// that are zero or no actor (`00905330` through the previous node, or
/// `0063f7b0` at the head), then marks the target targeted.
pub fn fn_009ec400(e: &mut Engine, this: Ptr<Crime>, _unused_1: u32) {
    let target = e.get(this, Crime::pcrimetarget);
    let target = resolve(e, target, TES_OBJECT_REFR_TYPE);
    e.set(this, Crime::pcrimetarget, target);
    let criminal = e.get(this, Crime::pcriminal);
    let criminal = resolve(e, criminal, ACTOR_TYPE);
    e.set(this, Crime::pcriminal, criminal);
    let ownership = e.get(this, Crime::pOwnership);
    let ownership = if ownership != 0 {
        e.call(FORM_OF_ID, &args![ownership]).u32()
    } else {
        0
    };
    e.set(this, Crime::pOwnership, ownership);
    let mut node = this.addr().wrapping_add(KNOWN_ACTORS);
    let mut previous = 0u32;
    while node != 0 && !e.call(LIST_EMPTY, &args![node]).bool() {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let id = e.mem.u32(slot);
        let mut remove = false;
        if id != 0 {
            let actor = resolve(e, id, ACTOR_TYPE);
            if actor != 0 {
                with_word(e, NODE_SET_ITEM, node, actor);
            } else {
                remove = true;
            }
        } else {
            remove = true;
        }
        if remove {
            if previous != 0 {
                let item = e.call(NODE_ITEM, &args![node]).u32();
                e.call(LIST_REMOVE, &args![previous, item]);
                node = e.call(NODE_NEXT, &args![previous]).u32();
            } else {
                e.call(LIST_REMOVE_HEAD, &args![node]);
            }
        } else {
            previous = node;
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
    let target = e.get(this, Crime::pcrimetarget);
    if target != 0 {
        e.call(SET_TARGETED, &args![target, 1u32]);
    }
}

// Translated from 009ec5d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ec5d0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `AlarmPackage` constructor (unnamed in the engine map; by the vtable it
/// sets, `01092bdc`): the `TESPackage` part, the vtable and the crime list.
/// Returns `this`.
pub fn fn_009ec5d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(PACKAGE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), ALARM_PACKAGE_VTABLE);
    e.call(LIST_CONSTRUCT, &args![this.addr() + CRIMES]);
    this
}

// Translated from 009ec640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AlarmPackage`'s scalar deleting destructor (unnamed in the engine map):
/// [`fn_009ec700`], then `operator delete` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_009ec640(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009ec700(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009ec670 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ec670 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `AlarmPackage` constructor with a crime (unnamed in the engine map;
/// `thiscall(crime)`): as [`fn_009ec5d0`], then adds `crime` when set.
/// Returns `this`.
pub fn fn_009ec670(e: &mut Engine, this: Ptr, crime: u32) -> Ptr {
    e.call(PACKAGE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), ALARM_PACKAGE_VTABLE);
    e.call(LIST_CONSTRUCT, &args![this.addr() + CRIMES]);
    if crime != 0 {
        with_word(e, LIST_ADD, this.addr() + CRIMES, crime);
    }
    this
}

// Translated from 009ec700 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ec700 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `AlarmPackage` destructor body (unnamed in the engine map): the vtable,
/// the crime list emptied and destroyed, then the `TESPackage` part
/// (`006708b0`).
pub fn fn_009ec700(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ALARM_PACKAGE_VTABLE);
    e.call(LIST_REMOVE_ALL, &args![this.addr() + CRIMES]);
    e.call(LIST_DESTRUCT, &args![this.addr() + CRIMES]);
    e.call(PACKAGE_DESTRUCT, &args![this]);
}

// Translated from 009ec780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(form)`; a copy, by its body): the
/// `TESPackage` copy (`00673290`), then the crime list replaced by the
/// crimes of `form` when it casts to `AlarmPackage` (up to its first empty
/// node).
pub fn fn_009ec780(e: &mut Engine, this: Ptr, form: u32) {
    e.call(PACKAGE_COPY, &args![this, form]);
    e.call(LIST_REMOVE_ALL, &args![this.addr() + CRIMES]);
    let source = cast(e, form, ALARM_PACKAGE_TYPE);
    if source == 0 {
        return;
    }
    let mut node = e.call(PACKAGE_CRIMES, &args![source]).u32();
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        e.call(LIST_ADD, &args![this.addr() + CRIMES, item]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 009ec810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AlarmPackage::IsCrimeInList` (Xbox PDB; `thiscall(crime)`).
pub fn alarm_package_is_crime_in_list(e: &mut Engine, this: Ptr, crime: u32) -> bool {
    with_word(e, LIST_CONTAINS, this.addr() + CRIMES, crime).bool()
}

// Translated from 009ec830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(crime)`): adds `crime` to the list.
pub fn fn_009ec830(e: &mut Engine, this: Ptr, crime: u32) {
    with_word(e, LIST_ADD, this.addr() + CRIMES, crime);
}

// Translated from 009ec850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AlarmPackage::RemoveCrime` (Xbox PDB; `thiscall(crime, actor)`): removes
/// `crime`; when the list is then empty, `Actor::EndInterruptPackage(0)` on
/// `actor`.
pub fn alarm_package_remove_crime(e: &mut Engine, this: Ptr, crime: u32, actor: u32) {
    with_word(e, LIST_REMOVE, this.addr() + CRIMES, crime);
    if e.call(LIST_EMPTY, &args![this.addr() + CRIMES]).bool() {
        e.call(END_INTERRUPT_PACKAGE, &args![actor, 0u32]);
    }
}

// Translated from 009ec890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`; the package's save size, by its
/// body): the `TESPackage` size plus 6 for the block header when blocks are
/// used, as a 16-bit size; with the save debug setting it reports the
/// header bytes (`Error`, line 0x1EB).
pub fn fn_009ec890(e: &mut Engine, this: Ptr) -> u16 {
    let package = e.call(PACKAGE_SAVE_SIZE, &args![this]).u16();
    let mut size = package;
    let start = size;
    if uses_blocks(e) {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    if save_debug(e) {
        let amount = i32::from(size).wrapping_sub(i32::from(start)) as u32;
        report_written(e, amount, 0x1eb, SIZE_FORMAT, SIZE_END_FORMAT);
    }
    size
}

// Translated from 009ec990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`; the package's save, by its body):
/// the `TESPackage` save (`00679120`), then in a block when blocks are used
/// a 16-bit crime count. The exe's crime loop starts from a null node, so
/// it never runs and the count stays 0; it is translated as the exe has it
/// (each crime's type byte and its `GetCrimeIndex` word). With the save
/// debug setting it reports the bytes written (`Error`, line 0x20E); the
/// block's length is patched at its end.
pub fn fn_009ec990(e: &mut Engine, this: Ptr) {
    e.call(PACKAGE_SAVE, &args![this]);
    e.with_stack(0xc, |e, words| {
        let length = words.addr();
        let count = length + 4;
        let entry = length + 8;
        let mut start = write_position(e);
        if save_debug(e) {
            start = write_position(e);
        }
        let length_at = start_block(e, length);
        e.mem.set_u16(count, 0);
        let count_at = write_position(e);
        write(e, count, 2);
        // The exe sets the node to 0 before testing it.
        let mut node = 0u32;
        while node != 0 && !e.call(LIST_EMPTY, &args![node]).bool() {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            let crime = e.mem.u32(slot);
            let crime_type = e.call(NODE_NEXT, &args![crime]).u8();
            e.mem.set_u8(entry, crime_type);
            write(e, entry, 1);
            let index = e
                .call(
                    LISTS_CRIME_INDEX,
                    &args![PROCESS_LISTS, crime_type as i8 as i32, crime],
                )
                .u16();
            e.mem.set_u16(entry + 2, index);
            write(e, entry + 2, 2);
            let counted = e.mem.u16(count).wrapping_add(1);
            e.mem.set_u16(count, counted);
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        let counted = e.mem.u16(count);
        e.mem.set_u16(count_at, counted);
        if save_debug(e) {
            let end = write_position(e);
            report_written(
                e,
                end.wrapping_sub(start),
                0x20e,
                SAVE_FORMAT,
                SAVE_END_FORMAT,
            );
        }
        finish_block(e, length_at, 0x20e);
    });
}

// Translated from 009ecbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AlarmPackage::LoadGame` (Xbox PDB): the `TESPackage` load
/// (`00679340`); with blocks, reads the tag (complaining through
/// `005b5e40` when it is not `"KOLB"`, with the record being read or
/// `008df040`), then the block's 16-bit length. Reads a 16-bit crime count
/// through `TESForm::LoadGameDataOLD` and, for each crime, its type byte and
/// index word (`GetCrimeByIndex`, whose result is not kept). With blocks it
/// then complains of an over- or underrun of the block (line 0x22A).
pub fn alarm_package_load_game(e: &mut Engine, this: Ptr) {
    e.call(PACKAGE_LOAD, &args![this]);
    e.with_stack(0x10, |e, words| {
        let length = words.addr();
        let tag = length + 4;
        let count = length + 8;
        let entry = length + 0xc;
        e.mem.set_u16(length, 0);
        let mut start = 0u32;
        if uses_blocks(e) {
            read(e, tag, 4);
            if e.mem.u32(tag) != BLOCK_TAG {
                let game = save_load_game(e);
                let record = e.call(LOAD_RECORD, &args![game]).u32();
                if record != 0 {
                    let id = e.mem.u32(record);
                    let form = e.call(FORM_OF_ID, &args![id]).u32();
                    let flags = e.mem.u32(record + 5);
                    let kind = u32::from(e.mem.u8(record + 9));
                    let name = e.vcall(form, 0x130, &[]).u32();
                    let id = e.mem.u32(record);
                    e.call(
                        DEBUG_PRINT,
                        &args![
                            BAD_HEADER_FORMAT,
                            SOURCE_FILE,
                            0x216u32,
                            id,
                            name,
                            kind,
                            flags
                        ],
                    );
                } else {
                    let game = save_load_game(e);
                    let byte = u32::from(e.call(LOAD_BYTE, &args![game]).u8());
                    e.call(
                        DEBUG_PRINT,
                        &args![BAD_HEADER_END_FORMAT, SOURCE_FILE, 0x216u32, byte],
                    );
                }
            }
            start = write_position(e);
            read(e, length, 2);
        }
        e.call(LOAD_GAME_DATA_OLD, &args![this, count, 2u32]);
        let mut index = 0i32;
        while index < i32::from(e.mem.u16(count)) {
            read(e, entry, 1);
            read(e, entry + 2, 2);
            let crime_type = e.mem.u8(entry) as i8 as i32;
            let crime_index = u32::from(e.mem.u16(entry + 2));
            e.call(
                LISTS_CRIME_BY_INDEX,
                &args![PROCESS_LISTS, crime_type, crime_index],
            );
            index += 1;
        }
        if !uses_blocks(e) {
            return;
        }
        let now = write_position(e);
        let game = save_load_game(e);
        let record = e.call(LOAD_RECORD, &args![game]).u32();
        let end = u32::from(e.mem.u16(length)).wrapping_add(start);
        if record != 0 {
            let id = e.mem.u32(record);
            let form = e.call(FORM_OF_ID, &args![id]).u32();
            let (format, amount) = if now > end {
                (OVERRUN_FORMAT, now.wrapping_sub(end))
            } else if now < end {
                (UNDERRUN_FORMAT, end.wrapping_sub(now))
            } else {
                return;
            };
            let flags = e.mem.u32(record + 5);
            let kind = u32::from(e.mem.u8(record + 9));
            let name = e.vcall(form, 0x130, &[]).u32();
            let id = e.mem.u32(record);
            e.call(
                DEBUG_PRINT,
                &args![format, amount, SOURCE_FILE, 0x22au32, id, name, kind, flags],
            );
        } else if now > end {
            let byte = u32::from(e.call(LOAD_BYTE, &args![game]).u8());
            let amount = now.wrapping_sub(end);
            e.call(
                DEBUG_PRINT,
                &args![OVERRUN_END_FORMAT, amount, SOURCE_FILE, 0x22au32, byte],
            );
        } else if now < end {
            let byte = u32::from(e.call(LOAD_BYTE, &args![game]).u8());
            let amount = end.wrapping_sub(now);
            e.call(
                DEBUG_PRINT,
                &args![UNDERRUN_END_FORMAT, amount, SOURCE_FILE, 0x22au32, byte],
            );
        }
    });
}

// Translated from 009ecec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AlarmPackage::SaveGame` (Xbox PDB; `thiscall(buffer)`): each non-null
/// crime as its type byte and `GetCrimeIndex` word (`00865e50`), then their
/// number as a variable-sized value.
pub fn alarm_package_save_game(e: &mut Engine, this: Ptr, buffer: u32) {
    let mut count = 0u32;
    let mark = e.call(BUFFER_START_SIZE, &args![buffer]).u32();
    let mut node = this.addr().wrapping_add(CRIMES);
    e.with_stack(4, |e, entry| {
        let entry = entry.addr();
        while node != 0 {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            let crime = e.mem.u32(slot);
            if crime != 0 {
                let crime_type = e.call(NODE_NEXT, &args![crime]).u8();
                e.mem.set_u8(entry + 1, crime_type);
                let index = e
                    .call(
                        LISTS_CRIME_INDEX,
                        &args![PROCESS_LISTS, crime_type as i8 as i32, crime],
                    )
                    .u16();
                e.mem.set_u16(entry + 2, index);
                e.call(BUFFER_WRITE, &args![buffer, entry + 1, 1u32, 0u32]);
                e.call(BUFFER_WRITE, &args![buffer, entry + 2, 2u32, 0u32]);
                count = count.wrapping_add(1);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
    e.call(BUFFER_SAVE_SIZE, &args![buffer, count, mark]);
}

// Translated from 009ecf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AlarmPackage::LoadGame` (Xbox PDB, the second overload;
/// `thiscall(buffer)`): `TESPackage::LoadGame_ov2`, then for each saved
/// crime its type byte and index word, adding the crime `GetCrimeByIndex`
/// finds (when there is one).
pub fn alarm_package_load_game_ov2(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(PACKAGE_LOAD_BUFFER, &args![this, buffer]);
    let count = e.call(BUFFER_LOAD_SIZE, &args![buffer]).u32();
    e.with_stack(8, |e, words| {
        let entry = words.addr();
        let crime_slot = entry + 4;
        for _ in 0..count {
            e.mem.set_u8(entry, 0);
            e.mem.set_u16(entry + 2, 0);
            e.call(BUFFER_READ, &args![buffer, entry, 1u32]);
            e.call(BUFFER_READ, &args![buffer, entry + 2, 2u32]);
            let crime_type = e.mem.u8(entry) as i8 as i32;
            let index = u32::from(e.mem.u16(entry + 2));
            let crime = e
                .call(
                    LISTS_CRIME_BY_INDEX,
                    &args![PROCESS_LISTS, crime_type, index],
                )
                .u32();
            e.mem.set_u32(crime_slot, crime);
            if crime != 0 {
                e.call(LIST_ADD, &args![this.addr() + CRIMES, crime_slot]);
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x009e_b500,
            crime_crime(Ptr<Crime>, u32, u32, u32, u32, u32, u32) -> Ptr<Crime>
        ),
        entry!(0x009e_b620, fn_009eb620(Ptr) -> u32),
        entry!(0x009e_b660, crime_destructor(Ptr<Crime>)),
        entry!(
            0x009e_b8a0,
            crime_should_crime_be_removed(Ptr<Crime>) -> bool
        ),
        entry!(0x009e_b9a0, fn_009eb9a0(Ptr<Crime>, u32) -> bool),
        entry!(0x009e_b9c0, fn_009eb9c0(Ptr<Crime>, u32)),
        entry!(0x009e_ba00, fn_009eba00(Ptr<Crime>, u32)),
        entry!(0x009e_ba30, crime_clear_actor_know_list(Ptr<Crime>)),
        entry!(0x009e_bab0, fn_009ebab0(Ptr<Crime>) -> u32),
        entry!(0x009e_bae0, fn_009ebae0(Ptr<Crime>, u32)),
        entry!(0x009e_bb80, fn_009ebb80(Ptr<Crime>) -> u16),
        entry!(0x009e_bca0, fn_009ebca0(Ptr<Crime>)),
        entry!(0x009e_bfe0, crime_save_game(Ptr<Crime>, u32)),
        entry!(0x009e_c1a0, crime_load_game(Ptr<Crime>, u32)),
        entry!(0x009e_c400, fn_009ec400(Ptr<Crime>, u32)),
        entry!(0x009e_c5d0, fn_009ec5d0(Ptr) -> Ptr),
        entry!(0x009e_c640, fn_009ec640(Ptr, u32) -> Ptr),
        entry!(0x009e_c670, fn_009ec670(Ptr, u32) -> Ptr),
        entry!(0x009e_c700, fn_009ec700(Ptr)),
        entry!(0x009e_c780, fn_009ec780(Ptr, u32)),
        entry!(0x009e_c810, alarm_package_is_crime_in_list(Ptr, u32) -> bool),
        entry!(0x009e_c830, fn_009ec830(Ptr, u32)),
        entry!(0x009e_c850, alarm_package_remove_crime(Ptr, u32, u32)),
        entry!(0x009e_c890, fn_009ec890(Ptr) -> u16),
        entry!(0x009e_c990, fn_009ec990(Ptr)),
        entry!(0x009e_cbe0, alarm_package_load_game(Ptr)),
        entry!(0x009e_cec0, alarm_package_save_game(Ptr, u32)),
        entry!(0x009e_cf70, alarm_package_load_game_ov2(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0200_0000;
    const SLOT_100: u32 = 0x0300_0100;
    const SLOT_22C: u32 = 0x0300_022c;
    const SLOT_130: u32 = 0x0300_0130;
    const GAME: u32 = 0x0500_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The calls to any of `addrs`, in order.
    fn sequence(e: &Engine, addrs: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| addrs.contains(a))
            .cloned()
            .collect()
    }

    fn ret_f64(v: f64) -> Ret {
        Ret {
            st0: v,
            ..Ret::default()
        }
    }

    /// Lists as chains of 8-byte nodes (item, next); the head node is the
    /// list. A `Crime`'s type at +4 is what `00726070` reads, as in the exe.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x230 / 4];
        slots[0x100 / 4] = SLOT_100;
        slots[0x22c / 4] = SLOT_22C;
        slots[0x130 / 4] = SLOT_130;
        e.put_vtable(VTABLE, &slots);
        for page in [
            0x011c_3000,
            0x011c_d000,
            0x011d_e000,
            0x011e_0000,
            0x011f_1000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.map(GAME, 0x30000);
        e.set_global(SAVE_LOAD_GAME, GAME);
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_CONTAINS, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) == item {
                    return true.into_ret();
                }
                node = e.mem.u32(node + 4);
            }
            false.into_ret()
        });
        e.register(LIST_ADD, |e, a| {
            let item = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) == 0 {
                e.mem.set_u32(a[0], item);
                return Ret::default();
            }
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let fresh = e.mem.alloc(8);
            e.mem.set_u32(fresh, item);
            e.mem.set_u32(node + 4, fresh);
            Ret::default()
        });
        e.register(LIST_COUNT, |e, a| {
            let mut count = 0u32;
            let mut node = a[0];
            while node != 0 {
                count += u32::from(e.mem.u32(node) != 0);
                node = e.mem.u32(node + 4);
            }
            count.into_ret()
        });
        for addr in [
            LIST_REMOVE,
            LIST_REMOVE_ALL,
            LIST_DESTRUCT,
            LIST_DELETE,
            LIST_CONSTRUCT,
            NODE_SET_ITEM,
            LIST_REMOVE_HEAD,
        ] {
            e.register(addr, |_, a| a[0].into_ret());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn list_of(e: &mut Engine, items: &[u32]) -> u32 {
        let head = e.mem.alloc(8);
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        head
    }

    fn crime(e: &mut Engine) -> Ptr<Crime> {
        e.new_object::<Crime>()
    }

    /// Puts the list nodes of `items` at the crime's +0x1C.
    fn know(e: &mut Engine, crime: Ptr<Crime>, items: &[u32]) {
        let list = list_of(e, items);
        let first = e.mem.u64(list);
        e.mem.set_u64(crime.addr() + KNOWN_ACTORS, first);
    }

    fn object(e: &mut Engine) -> u32 {
        let p = e.mem.alloc(0x20);
        e.mem.set_u32(p, VTABLE);
        p
    }

    fn calendar(e: &mut Engine, month: u32, day: u8, year: u32) {
        e.register_double(CALENDAR_MONTH, move |_, _| month.into_ret());
        e.register_double(CALENDAR_DAY, move |_, _| {
            (0x1200 | u32::from(day)).into_ret()
        });
        e.register_double(CALENDAR_YEAR, move |_, _| year.into_ret());
    }

    #[test]
    fn a_new_crime_takes_its_number_and_stamps() {
        let mut e = engine();
        calendar(&mut e, 3, 5, 2);
        e.register(SET_TARGETED, |_, _| Ret::default());
        e.register(TIME_STAMP_CONSTRUCT, |_, _| Ret::default());
        e.register(COMBAT_TIME, |_, _| ret_f64(12.5));
        e.register(TIME_STAMP_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.set_global(PROCESS_LISTS + 0x103b8, 41u32);
        let this = crime(&mut e);
        e.mem.set_u32(this.addr(), 9);
        e.mem.set_u8(this.addr() + 0x10, 1);
        let result = e
            .call(
                0x009e_b500,
                &args![this, 4u32, 0x7000u32, 0x7100u32, 0x7200u32, 3u32, 0x7300u32],
            )
            .u32();
        assert_eq!(result, this.addr());
        assert_eq!(e.get(this, Crime::iRefCount), 0);
        assert_eq!(e.get(this, Crime::ecrime), 4);
        assert_eq!(e.get(this, Crime::pcrimetarget), 0x7000);
        assert_eq!(e.get(this, Crime::pcriminal), 0x7100);
        assert_eq!(e.get(this, Crime::breported), 0);
        assert_eq!(e.get(this, Crime::pObjectStolen), 0x7200);
        assert_eq!(e.get(this, Crime::inumberstolen), 3);
        assert_eq!(e.get(this, Crime::pOwnership), 0x7300);
        assert_eq!(e.get(this, Crime::iCrimeNumber), 41);
        assert_eq!(e.global::<u32>(PROCESS_LISTS + 0x103b8), 42);
        assert_eq!(e.get(this, Crime::iCrimeStamp), (3 << 9) | 5 | (2 << 13));
        assert_eq!(e.get(this, Crime::fCrimeTimeStamp), 12.5f32.to_bits());
        assert_eq!(calls(&e, SET_TARGETED), vec![vec![0x7000, 1]]);
        assert_eq!(calls(&e, LIST_CONSTRUCT), vec![vec![this.addr() + 0x1c]]);
        // A negative day (a signed byte) sets every high bit; no target, no
        // SetTargeted.
        calendar(&mut e, 3, 0xfe, 2);
        e.call_log = Some(vec![]);
        e.call(
            0x009e_b500,
            &args![this, 4u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(e.get(this, Crime::iCrimeStamp), 0xffff_fffe);
        assert!(calls(&e, SET_TARGETED).is_empty());
        assert_eq!(e.call(0x009e_b620, &args![PROCESS_LISTS]).u32(), 43);
    }

    #[test]
    fn the_destructor_cleans_up_only_with_its_owner() {
        let mut e = engine();
        let this = crime(&mut e);
        let factions = list_of(&mut e, &[0x8800, 0x8900]);
        e.set(this, Crime::pListofEnemyFactions, factions);
        e.call(0x009e_b660, &args![this]);
        assert_eq!(
            sequence(&e, &[LIST_REMOVE_ALL, LIST_DELETE, LIST_DESTRUCT]),
            vec![
                (LIST_REMOVE_ALL, vec![factions]),
                (LIST_DELETE, vec![factions, 1]),
                (LIST_REMOVE_ALL, vec![this.addr() + 0x1c]),
                (LIST_DESTRUCT, vec![this.addr() + 0x1c]),
            ]
        );
        // With the owner: the clean-up runs first.
        e.set_global(CLEANUP_OWNER, 0x4400u32);
        e.register(CLEANUP_TEST, |_, _| false.into_ret());
        e.register(LISTS_FORGET_CRIME, |_, _| Ret::default());
        e.register(LISTS_FACTION_CHECK, |_, a| (a[1] == 0x8800).into_ret());
        e.register(FACTION_RELEASE, |_, _| Ret::default());
        e.register(LISTS_CRIME_ALARM_ACTORS, |_, _| 0u32.into_ret());
        e.register(CRIME_TARGET, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(EXTRA_DATA, |_, a| a[0].into_ret());
        e.register(CRIME_LIST, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        let target = object(&mut e);
        let target_list = e.mem.alloc(8);
        e.mem.set_u32(target + 0x10, target_list);
        e.set(this, Crime::pcrimetarget, target);
        e.mem.set_u32(target + 4, 1);
        e.register(SLOT_100, |e, a| (e.mem.u32(a[0] + 4) as u8).into_ret());
        let known = object(&mut e);
        know(&mut e, this, &[known]);
        e.call_log = Some(vec![]);
        e.call(0x009e_b660, &args![this]);
        assert_eq!(
            calls(&e, LISTS_FORGET_CRIME),
            vec![vec![PROCESS_LISTS, this.addr()]]
        );
        assert_eq!(calls(&e, FACTION_RELEASE), vec![vec![0x8900, 0]]);
        // Taken off the target's list twice: as the target, then as the
        // +8 holder whose slot 0x100 holds.
        let removed: Vec<u32> = calls(&e, LIST_REMOVE).iter().map(|w| w[0]).collect();
        assert_eq!(removed, vec![target_list, target_list]);
        assert_eq!(e.get(this, Crime::iRefCount), 0);
        assert_eq!(
            calls(&e, LISTS_CRIME_ALARM_ACTORS),
            vec![vec![PROCESS_LISTS, this.addr(), 1]]
        );
        // The owner's test holding skips the clean-up.
        e.register(CLEANUP_TEST, |_, _| true.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009e_b660, &args![this]);
        assert!(calls(&e, LISTS_FORGET_CRIME).is_empty());
    }

    #[test]
    fn old_unreferenced_or_ignored_crimes_go() {
        let mut e = engine();
        calendar(&mut e, 0, 10, 0);
        e.register(SETTING_INT, |_, a| a[0].into_ret());
        e.set_global(CRIME_AGE_SETTING, 5u32);
        e.register(SLOT_22C, |e, a| (e.mem.u32(a[0] + 4) as u8).into_ret());
        let this = crime(&mut e);
        e.set(this, Crime::iCrimeStamp, 4);
        let remove = |e: &mut Engine| e.call(0x009e_b8a0, &args![this]).bool();
        assert!(remove(&mut e));
        e.set(this, Crime::iCrimeStamp, 6);
        assert!(remove(&mut e));
        e.set(this, Crime::iRefCount, 1);
        assert!(remove(&mut e));
        let ignoring = object(&mut e);
        e.mem.set_u32(ignoring + 4, 1);
        let watching = object(&mut e);
        know(&mut e, this, &[ignoring, watching]);
        assert!(!remove(&mut e));
        assert_eq!(calls(&e, SLOT_22C).last().unwrap(), &vec![watching, 0]);
        e.mem.set_u32(watching + 4, 1);
        assert!(remove(&mut e));
    }

    #[test]
    fn the_known_actor_list() {
        let mut e = engine();
        let this = crime(&mut e);
        e.call(0x009e_b9c0, &args![this, 0x6600u32]);
        e.call(0x009e_b9c0, &args![this, 0x6600u32]);
        e.call(0x009e_b9c0, &args![this, 0x6700u32]);
        assert_eq!(e.get(this, Crime::iRefCount), 2);
        assert!(e.call(0x009e_b9a0, &args![this, 0x6700u32]).bool());
        assert!(!e.call(0x009e_b9a0, &args![this, 0x6800u32]).bool());
        assert_eq!(e.call(0x009e_bab0, &args![this]).u32(), 2);
        e.call(0x009e_ba00, &args![this, 0x6600u32]);
        assert_eq!(e.get(this, Crime::iRefCount), 1);
        let removed = calls(&e, LIST_REMOVE);
        assert_eq!(removed[0][0], this.addr() + 0x1c);
        let other = crime(&mut e);
        assert_eq!(e.call(0x009e_bab0, &args![other]).u32(), 0);
        // Clearing takes the crime off each actor's list.
        e.register(EXTRA_DATA, |_, a| a[0].into_ret());
        e.register(CRIME_LIST, |_, a| {
            if a[0] == 0x6600 { 0x6610u32 } else { 0 }.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x009e_ba30, &args![this]);
        assert_eq!(calls(&e, LIST_REMOVE).len(), 1);
        assert_eq!(calls(&e, LIST_REMOVE)[0][0], 0x6610);
        assert_eq!(e.get(this, Crime::iRefCount), 0);
    }

    #[test]
    fn enemy_factions_get_a_list_once() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = crime(&mut e);
        e.call(0x009e_bae0, &args![this, 0x8800u32]);
        e.call(0x009e_bae0, &args![this, 0x8900u32]);
        let list = e.get(this, Crime::pListofEnemyFactions);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![8]]);
        assert_eq!(calls(&e, LIST_CONSTRUCT), vec![vec![list]]);
        assert_eq!(calls(&e, LIST_ADD).len(), 2);
        assert_eq!(e.mem.u32(list), 0x8800);
    }

    /// The save game: blocks per `GAME + 0x10`, debug per the setting's
    /// byte at `011de4e8`, the write position counting up from 0x1000 by
    /// the bytes written; the writes are recorded with their bytes.
    fn saving(e: &mut Engine, blocks: bool, debug: bool) {
        e.mem.set_u8(GAME + 0x10, blocks as u8);
        e.mem.set_u8(SAVE_DEBUG_SETTING, debug as u8);
        e.mem.set_u32(GAME + 0x14, GAME + 0x100);
        e.register(USE_BLOCKS, |e, a| e.mem.u8(a[0] + 0x10).into_ret());
        e.register(SETTING_VALUE, |_, a| a[0].into_ret());
        e.register(WRITE_POSITION, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(WRITE, |e, a| {
            let position = e.mem.u32(a[0] + 0x14);
            e.mem.set_u32(a[0] + 0x14, position + a[2]);
            Ret::default()
        });
        e.register(SAVE_NUMERIC_ID, |e, a| {
            let position = e.mem.u32(a[0] + 0x14);
            e.mem.set_u32(a[0] + 0x14, position + a[2]);
            Ret::default()
        });
        e.register(FORM_ID, |_, a| (a[0] + 1).into_ret());
        e.register(SAVE_RECORD, |_, _| 0u32.into_ret());
        e.register(ERROR_PRINT, |_, _| Ret::default());
        e.register(DEBUG_PRINT, |_, _| Ret::default());
    }

    #[test]
    fn the_crime_save_size_and_block() {
        let mut e = engine();
        saving(&mut e, true, true);
        e.register(SLOT_130, |_, _| 0x5150u32.into_ret());
        let this = crime(&mut e);
        know(&mut e, this, &[0x6600, 0x6700]);
        assert_eq!(e.call(0x009e_bb80, &args![this]).u32(), 6 + 0x20 + 8 + 2);
        assert_eq!(
            calls(&e, ERROR_PRINT),
            vec![vec![SIZE_END_FORMAT, 48, 0xf5, SOURCE_FILE]]
        );
        // With a record, the long form.
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record, 0x77);
        e.mem.set_u32(record + 5, 0x1234_5678);
        let form = object(&mut e);
        e.register_double(SAVE_RECORD, move |_, _| record.into_ret());
        e.register_double(FORM_OF_ID, move |_, _| form.into_ret());
        saving(&mut e, false, true);
        e.register_double(SAVE_RECORD, move |_, _| record.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009e_bb80, &args![this]).u32(), 0x20 + 8 + 2);
        assert_eq!(
            calls(&e, ERROR_PRINT),
            vec![vec![
                SIZE_FORMAT,
                42,
                0x77,
                0x5150,
                0x1234_5678,
                0xf5,
                SOURCE_FILE
            ]]
        );
        // The save writes in a block and patches the lengths.
        saving(&mut e, true, false);
        e.set(this, Crime::pcrimetarget, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x009e_bca0, &args![this]);
        let sizes: Vec<u32> = sequence(&e, &[WRITE, SAVE_NUMERIC_ID])
            .iter()
            .map(|(_, w)| w[2])
            .collect();
        assert_eq!(sizes, vec![4, 2, 1, 1, 4, 4, 4, 4, 4, 4, 4, 2, 4, 4]);
        let writes = calls(&e, WRITE);
        assert_eq!(writes[2][1], this.addr() + 0x10);
        assert_eq!(writes[3][1], this.addr() + 0x2c);
        // Block: tag at 0x100, length at 0x104, count at 0x124, end 0x12e.
        assert_eq!(e.mem.u16(GAME + 0x104), 0x2a);
        assert_eq!(e.mem.u16(GAME + 0x124), 2);
        // Ids: 0 for no form, the form id otherwise.
        let ids = calls(&e, SAVE_NUMERIC_ID);
        assert_eq!(ids.len(), 6);
        assert!(calls(&e, ERROR_PRINT).is_empty());
    }

    #[test]
    fn crime_save_and_load_game() {
        let mut e = engine();
        for addr in [
            BUFFER_WRITE,
            BUFFER_SAVE_FORM,
            BUFFER_SAVE_SIZE,
            BUFFER_READ,
            BUFFER_LOAD_FORM,
            TIME_STAMP_SAVE,
            TIME_STAMP_LOAD,
            LIST_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(BUFFER_START_SIZE, |_, _| 0x55u32.into_ret());
        let this = crime(&mut e);
        e.set(this, Crime::pcrimetarget, 0x7000);
        know(&mut e, this, &[0x6600, 0, 0x6700]);
        let factions = list_of(&mut e, &[0x8800]);
        e.set(this, Crime::pListofEnemyFactions, factions);
        e.call(0x009e_bfe0, &args![this, 0x9000u32]);
        let fields: Vec<(u32, u32)> = calls(&e, BUFFER_WRITE)
            .iter()
            .map(|w| (w[1] - this.addr(), w[2]))
            .collect();
        assert_eq!(
            fields,
            vec![
                (0, 4),
                (4, 4),
                (0x10, 1),
                (0x18, 4),
                (0x28, 4),
                (0x2c, 1),
                (0x30, 4)
            ]
        );
        let forms: Vec<u32> = calls(&e, BUFFER_SAVE_FORM).iter().map(|w| w[1]).collect();
        assert_eq!(forms, vec![0x7000, 0, 0, 0, 0x6600, 0x6700, 0x8800]);
        assert_eq!(
            calls(&e, BUFFER_SAVE_SIZE),
            vec![vec![0x9000, 2, 0x55], vec![0x9000, 1, 0x55]]
        );
        // Load: two actor ids, a stolen object cast, two factions of which
        // one casts.
        e.register(BUFFER_LOAD_SIZE, |_, _| 2u32.into_ret());
        e.register(BUFFER_LOAD_FORM_ID, |_, _| 0x31u32.into_ret());
        e.register(FORM_OF_ID, |_, a| (a[0] + 0x100).into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| {
            if a[3] == TES_FACTION_TYPE { 0u32 } else { a[0] }.into_ret()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let fresh = crime(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x009e_c1a0, &args![fresh, 0x9100u32]);
        assert_eq!(calls(&e, BUFFER_READ).len(), 7);
        assert_eq!(e.get(fresh, Crime::pObjectStolen), 0x131);
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST)[0],
            vec![0x131, 0, TES_FORM_TYPE, TES_BOUND_OBJECT_TYPE, 0]
        );
        assert_eq!(calls(&e, LIST_ADD).len(), 2);
        // No faction cast: the new list is deleted again.
        let list = calls(&e, LIST_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, LIST_DELETE), vec![vec![list, 1]]);
        assert_eq!(e.get(fresh, Crime::pListofEnemyFactions), 0);
    }

    #[test]
    fn the_load_fix_up_resolves_and_drops_ids() {
        let mut e = engine();
        e.register(FORM_OF_ID, |_, a| (a[0] + 0x100).into_ret());
        // Ids below 0x50 are actors.
        e.register(RT_DYNAMIC_CAST, |_, a| {
            if a[3] == ACTOR_TYPE && a[0] >= 0x150 {
                0u32
            } else {
                a[0]
            }
            .into_ret()
        });
        e.register(LIST_REMOVE, |e, a| {
            let next = e.mem.u32(a[1] + 4);
            e.mem.set_u32(a[0] + 4, next);
            Ret::default()
        });
        e.register(SET_TARGETED, |_, _| Ret::default());
        let this = crime(&mut e);
        e.set(this, Crime::pcrimetarget, 0x10);
        e.set(this, Crime::pcriminal, 0x60);
        e.set(this, Crime::pOwnership, 0x20);
        know(&mut e, this, &[0x11, 0x70, 0x12]);
        let head = this.addr() + 0x1c;
        let second = e.mem.u32(head + 4);
        e.call(0x009e_c400, &args![this, 0u32]);
        assert_eq!(e.get(this, Crime::pcrimetarget), 0x110);
        assert_eq!(e.get(this, Crime::pcriminal), 0);
        assert_eq!(e.get(this, Crime::pOwnership), 0x120);
        assert_eq!(calls(&e, SET_TARGETED), vec![vec![0x110, 1]]);
        assert_eq!(calls(&e, NODE_SET_ITEM).len(), 2);
        assert_eq!(calls(&e, LIST_REMOVE), vec![vec![head, second]]);
        // A zero id at the head goes through 0063f7b0 (the double moves the
        // next node into the head).
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let moved = e.mem.u64(next);
            e.mem.set_u64(a[0], moved);
            Ret::default()
        });
        know(&mut e, this, &[0, 0x11]);
        e.call_log = Some(vec![]);
        e.call(0x009e_c400, &args![this, 0u32]);
        assert_eq!(calls(&e, LIST_REMOVE_HEAD), vec![vec![head]]);
    }

    #[test]
    fn the_alarm_package_keeps_crimes() {
        let mut e = engine();
        e.register(PACKAGE_CONSTRUCT, |_, _| Ret::default());
        e.register(PACKAGE_DESTRUCT, |_, _| Ret::default());
        e.register(PACKAGE_COPY, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(END_INTERRUPT_PACKAGE, |_, _| Ret::default());
        let this = e.mem.alloc(0x90);
        assert_eq!(e.call(0x009e_c5d0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), ALARM_PACKAGE_VTABLE);
        assert_eq!(calls(&e, LIST_CONSTRUCT), vec![vec![this + 0x80]]);
        assert_eq!(e.call(0x009e_c670, &args![this, 0x4400u32]).u32(), this);
        assert!(e.call(0x009e_c810, &args![this, 0x4400u32]).bool());
        e.call(0x009e_c830, &args![this, 0x4500u32]);
        assert!(e.call(0x009e_c810, &args![this, 0x4500u32]).bool());
        assert_eq!(calls(&e, LIST_ADD).len(), 2);
        // Removing: the list is not empty, so no EndInterruptPackage.
        e.call(0x009e_c850, &args![this, 0x4400u32, 0x6600u32]);
        assert!(calls(&e, END_INTERRUPT_PACKAGE).is_empty());
        e.mem.set_u64(this + 0x80, 0);
        e.call(0x009e_c850, &args![this, 0x4500u32, 0x6600u32]);
        assert_eq!(calls(&e, END_INTERRUPT_PACKAGE), vec![vec![0x6600, 0]]);
        // A copy takes the crimes of an alarm package.
        let source = e.mem.alloc(0x90);
        let crimes = list_of(&mut e, &[0x4600, 0x4700]);
        e.register(RT_DYNAMIC_CAST, |e, a| {
            if e.mem.u32(a[0]) == 1 { a[0] } else { 0 }.into_ret()
        });
        e.register_double(PACKAGE_CRIMES, move |_, _| crimes.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009e_c780, &args![this, source]);
        assert!(calls(&e, LIST_ADD).is_empty());
        e.mem.set_u32(source, 1);
        e.call(0x009e_c780, &args![this, source]);
        assert_eq!(calls(&e, LIST_ADD).len(), 2);
        assert_eq!(calls(&e, PACKAGE_COPY).len(), 2);
        assert_eq!(calls(&e, LIST_REMOVE_ALL), vec![vec![this + 0x80]; 2]);
        // The deleting destructor.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009e_c640, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, LIST_DESTRUCT), vec![vec![this + 0x80]]);
        assert_eq!(calls(&e, PACKAGE_DESTRUCT), vec![vec![this]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
        e.call(0x009e_c640, &args![this, 0u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
    }

    #[test]
    fn the_alarm_package_save_size_and_save() {
        let mut e = engine();
        saving(&mut e, true, true);
        e.register(PACKAGE_SAVE_SIZE, |_, _| 0x30u16.into_ret());
        e.register(PACKAGE_SAVE, |_, _| Ret::default());
        let this = e.mem.alloc(0x90);
        assert_eq!(e.call(0x009e_c890, &args![this]).u32(), 0x36);
        assert_eq!(
            calls(&e, ERROR_PRINT),
            vec![vec![SIZE_END_FORMAT, 6, 0x1eb, SOURCE_FILE]]
        );
        e.call_log = Some(vec![]);
        e.call(0x009e_c990, &args![this]);
        let sizes: Vec<u32> = calls(&e, WRITE).iter().map(|w| w[2]).collect();
        assert_eq!(sizes, vec![4, 2, 2]);
        assert_eq!(e.mem.u16(GAME + 0x104), 4);
        assert_eq!(e.mem.u16(GAME + 0x106), 0);
        assert_eq!(
            calls(&e, ERROR_PRINT),
            vec![vec![SAVE_END_FORMAT, 8, 0x20e, SOURCE_FILE]]
        );
        // A block over 0xFFFF bytes complains.
        e.register(WRITE_POSITION, |e, a| {
            let position = e.mem.u32(a[0] + 0x14);
            e.mem.set_u32(a[0] + 0x14, position + 0x8000);
            position.into_ret()
        });
        e.mem.set_u8(SAVE_DEBUG_SETTING, 0);
        e.call_log = Some(vec![]);
        e.call(0x009e_c990, &args![this]);
        assert_eq!(
            calls(&e, DEBUG_PRINT),
            vec![vec![BLOCK_TOO_BIG, SOURCE_FILE, 0x20e]]
        );
    }

    #[test]
    fn the_alarm_package_load_game_checks_its_block() {
        let mut e = engine();
        saving(&mut e, true, false);
        e.register(PACKAGE_LOAD, |_, _| Ret::default());
        e.register(LOAD_RECORD, |_, _| 0u32.into_ret());
        e.register(LOAD_BYTE, |_, _| 0x1207u32.into_ret());
        // Reads: the tag, then the length 3, then (type, index) pairs; the
        // position moves by what is read.
        e.register(READ, |e, a| {
            let position = e.mem.u32(a[0] + 0x14);
            e.mem.set_u32(a[0] + 0x14, position + a[2]);
            let value = match a[2] {
                4 => e.mem.u32(GAME + 0x20),
                2 => 5,
                _ => 0xff,
            };
            match a[2] {
                4 => e.mem.set_u32(a[1], value),
                2 => e.mem.set_u16(a[1], value as u16),
                _ => e.mem.set_u8(a[1], value as u8),
            }
            Ret::default()
        });
        e.register(LOAD_GAME_DATA_OLD, |e, a| {
            e.mem.set_u16(a[1], 1);
            Ret::default()
        });
        e.register(LISTS_CRIME_BY_INDEX, |_, _| 0u32.into_ret());
        e.mem.set_u32(GAME + 0x20, BLOCK_TAG);
        let this = e.mem.alloc(0x90);
        e.call(0x009e_cbe0, &args![this]);
        assert_eq!(
            calls(&e, LISTS_CRIME_BY_INDEX),
            vec![vec![PROCESS_LISTS, 0xffff_ffff, 5]]
        );
        // Start at 0x104 after the tag; length 5; the reads end at 0x109:
        // exactly the block, no complaint.
        assert!(calls(&e, DEBUG_PRINT).is_empty());
        // A bad tag complains (no record: with 008df040's byte); a longer
        // read overruns.
        e.mem.set_u32(GAME + 0x20, 0x1234);
        e.mem.set_u32(GAME + 0x14, GAME + 0x100);
        e.register(LOAD_GAME_DATA_OLD, |e, a| {
            e.mem.set_u16(a[1], 2);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009e_cbe0, &args![this]);
        assert_eq!(
            calls(&e, DEBUG_PRINT),
            vec![
                vec![BAD_HEADER_END_FORMAT, SOURCE_FILE, 0x216, 7],
                vec![OVERRUN_END_FORMAT, 3, SOURCE_FILE, 0x22a, 7],
            ]
        );
    }

    #[test]
    fn the_alarm_package_buffer_save_and_load() {
        let mut e = engine();
        e.register(BUFFER_START_SIZE, |_, _| 0x55u32.into_ret());
        e.register(BUFFER_SAVE_SIZE, |_, _| Ret::default());
        e.register(BUFFER_WRITE, |_, _| Ret::default());
        e.register(LISTS_CRIME_INDEX, |_, a| a[1].wrapping_add(a[2]).into_ret());
        let this = e.mem.alloc(0x90);
        let first = crime(&mut e);
        e.set(first, Crime::ecrime, 0x80);
        let list = list_of(&mut e, &[first.addr(), 0]);
        let head = e.mem.u64(list);
        e.mem.set_u64(this + 0x80, head);
        e.call(0x009e_cec0, &args![this, 0x9000u32]);
        assert_eq!(
            calls(&e, LISTS_CRIME_INDEX),
            vec![vec![PROCESS_LISTS, 0xffff_ff80, first.addr()]]
        );
        assert_eq!(calls(&e, BUFFER_SAVE_SIZE), vec![vec![0x9000, 1, 0x55]]);
        let writes = calls(&e, BUFFER_WRITE);
        assert_eq!((writes[0][2], writes[1][2]), (1, 2));
        // Load: two entries, one found.
        e.register(PACKAGE_LOAD_BUFFER, |_, _| Ret::default());
        e.register(BUFFER_LOAD_SIZE, |_, _| 2u32.into_ret());
        e.register(BUFFER_READ, |e, a| {
            if a[2] == 1 {
                e.mem.set_u8(a[1], 0xfe);
            } else {
                e.mem.set_u16(a[1], 9);
            }
            Ret::default()
        });
        let found = std::cell::Cell::new(0u32);
        e.register_double(LISTS_CRIME_BY_INDEX, move |_, _| {
            found.set(found.get() + 1);
            (if found.get() == 1 { 0x4400u32 } else { 0 }).into_ret()
        });
        let target = e.mem.alloc(0x90);
        e.call_log = Some(vec![]);
        e.call(0x009e_cf70, &args![target, 0x9100u32]);
        assert_eq!(
            calls(&e, LISTS_CRIME_BY_INDEX),
            vec![vec![PROCESS_LISTS, 0xffff_fffe, 9]; 2]
        );
        assert_eq!(e.mem.u32(target + 0x80), 0x4400);
        assert_eq!(calls(&e, LIST_ADD).len(), 1);
    }
}
