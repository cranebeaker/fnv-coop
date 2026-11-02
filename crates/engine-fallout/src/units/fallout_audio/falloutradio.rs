//! `fallout/audio/falloutradio.cpp` (Xbox PDB source unit), subsystem `fallout/audio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_008321c0` calls on the member (`00836030`).
const CALLEE_00836030: u32 = 0x0083_6030;

// Translated from 008321c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00836030` with `ECX` = `this + 0x4` and passes on nothing (the callee returns no value).
pub fn fn_008321c0(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_00836030, &args![this.byte_add(0x4)]);
}

// Translated from 008372c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0107cf18` (RTTI `.?AVRadioConvTask@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00c3cea0`. One word, plain `RET`, returns nothing.
pub fn fn_008372c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0107_cf18);
    e.call(0x00c3_cea0, &args![this]);
}

// ---------------------------------------------------------------------------
// The radio's statics (`FalloutRadio`, Xbox PDB, has only statics). Which PDB
// static each address is is not confirmed; they are named by their use.

/// Set by `Shutdown`; every update stops while it is set.
const SHUTTING_DOWN: u32 = 0x011d_d436;
/// The station list (`BSSimpleList` of `FORadioStation *`, first node
/// embedded): its entries are deleted with the station's deleting destructor.
const STATION_LIST: u32 = 0x011d_d554;
/// The receiver list (`BSSimpleList` of `FORadioReceiver *`).
const RECEIVER_LIST: u32 = 0x011d_d58c;
/// The list of stations already reported (references).
const DISCOVERED_LIST: u32 = 0x011d_d59c;
/// The time of the last periodic update.
const LAST_UPDATE: u32 = 0x011d_d424;
/// The setting with the periodic update interval.
const UPDATE_INTERVAL_SETTING: u32 = 0x011d_d5cc;
/// The timer object `00825c00` reads the time from.
const TIMER: u32 = 0x011f_6394;
/// A byte the update sets once the player's check (`00425fd0`) has passed.
const PLAYER_CHECK_SEEN: u32 = 0x011d_d435;
/// The queued and the current radio state flags.
const QUEUED_STATE: u32 = 0x011d_d438;
const RADIO_STATE: u32 = 0x011d_d43c;
/// The sound form the update looks up once (data handler `011c3f2c`).
const ATTENUATION_SOUND: u32 = 0x011d_d440;
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The Pip-Boy's current and lost stations.
const CURRENT_PIPBOY_STATION: u32 = 0x011d_d42c;
const LOST_PIPBOY_STATION: u32 = 0x011d_d430;
/// The other saved statics: a word, a byte and a string.
const SAVED_WORD: u32 = 0x011d_d428;
const SAVED_BYTE: u32 = 0x011d_d434;
const SAVED_STRING: u32 = 0x011d_d448;
/// A byte that suppresses the new-station message once.
const SKIP_DISCOVERY_MESSAGE: u32 = 0x011a_179c;
/// The setting `UpdateStations` scales signal strength with.
const SIGNAL_SETTING: u32 = 0x011d_d580;
/// The player (`PlayerCharacter *` global) and the task queue.
const PLAYER: u32 = 0x011d_ea3c;
const TASK_QUEUE: u32 = 0x0120_2d98;

/// List helpers: the address of a node's item, the next node, the
/// emptiness test, the add-at-head (address of the item), the removal (by
/// the item's address), the membership test, the clear and the destructor
/// bodies of a list, and the constructor of one.
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const NODE_EMPTY: u32 = 0x0082_56d0;
const LIST_ADD_HEAD: u32 = 0x005a_e3d0;
const LIST_REMOVE: u32 = 0x0090_5330;
const LIST_CONTAINS: u32 = 0x005f_65d0;
const LIST_CLEAR: u32 = 0x0047_0470;
const LIST_DESTRUCTOR: u32 = 0x0046_ffb0;
const LIST_CONSTRUCTOR: u32 = 0x0096_a2d0;
/// The float list's clear and destructor bodies (`008373d0` builds one).
const FLOAT_LIST_CLEAR: u32 = 0x0083_7410;
const FLOAT_LIST_DESTRUCTOR: u32 = 0x0083_73f0;
/// `operator new` and `operator delete` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The deleting destructor of a `Conversation` (`this`, flags).
const CONVERSATION_DELETE: u32 = 0x0079_8450;
/// `Conversation::Conversation_ov2` (`this`, two references, topic).
const CONVERSATION_CONSTRUCT: u32 = 0x0083_b850;
/// `RadioConvTask`'s constructor (`this`, station).
const RADIO_TASK_CONSTRUCT: u32 = 0x0083_7250;
/// `TESTopic::GetTopic` (cdecl: kind, word).
const GET_TOPIC: u32 = 0x0061_a2d0;
/// `BSSoundHandle` constructor, `Stop` and `Release`.
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
const SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
const SOUND_HANDLE_DESTRUCTOR: u32 = 0x0048_3710;
/// The form of a reference (`this` only) and a form's type byte.
const FORM_OF: u32 = 0x007a_f430;
const FORM_TYPE: u32 = 0x0040_1170;
/// The talking activator's radio station form (`this` = base form).
const RADIO_STATION_FORM: u32 = 0x004f_d3c0;
/// `BGSTalkingActivator::GetRadioStationsInRangeOfRef` (cdecl: reference,
/// reference list, float list).
const STATIONS_IN_RANGE: u32 = 0x004f_f1a0;
/// A test on a reference (`this` only).
const REFERENCE_TEST_00440DA0: u32 = 0x0044_0da0;
/// A word of a station reference (`this` only).
const REFERENCE_VALUE_0056B210: u32 = 0x0056_b210;
/// `Script::IsPendingEnabledReference` (cdecl, one word).
const PENDING_ENABLED: u32 = 0x005a_a680;
/// `BSAudio::QInstance` and the audio time (`this` = the audio).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_TIME: u32 = 0x0063_d040;
/// A random number in a range (cdecl: low, high).
const RANDOM_RANGE: u32 = 0x0094_4460;
/// The time of the timer object (`this` = timer).
const TIMER_TIME: u32 = 0x0082_5c00;
/// A setting's value address (`ECX` = the setting).
const SETTING_VALUE_ADDRESS: u32 = 0x0043_d4d0;
const SETTING_FLOAT_ADDRESS: u32 = 0x0040_3e20;
/// `Interface::GetLoadingMenuVisible` (no arguments).
const LOADING_MENU_VISIBLE: u32 = 0x0070_5e80;
/// The player's object and its test of the update (`008d6f30`, `00425fd0`).
const PLAYER_OBJECT: u32 = 0x008d_6f30;
const PLAYER_OBJECT_TEST: u32 = 0x0042_5fd0;
/// `TESDataHandler::GetSound_ov2` (`this`, editor id).
const GET_SOUND: u32 = 0x0046_16c0;
/// The save buffer's `SaveFormID_ov2`, plain data, `SaveString`,
/// `StartVariableSizedValue` and `SaveVariableSizedValue_ov2`.
const SAVE_FORM_ID: u32 = 0x0086_5df0;
const SAVE_DATA: u32 = 0x0086_5e50;
const SAVE_STRING: u32 = 0x0086_5e70;
const SAVE_START_SIZED: u32 = 0x0086_5f20;
const SAVE_SIZED: u32 = 0x0086_5ff0;
/// `Conversation::SaveGame` (`this`, buffer).
const CONVERSATION_SAVE: u32 = 0x0083_c270;
/// A station state call `00832490` (`this` = state, flag).
const STATE_CALL_00832490: u32 = 0x0083_2490;
/// The station state's power setter (`this` = state, power byte).
const STATE_SET_POWER: u32 = 0x0045_8b30;
/// `BSStringT` constructor, destructor, `Format` and C string.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
const STRING_FORMAT: u32 = 0x0040_6f60;
const STRING_C_STR: u32 = 0x0055_9450;
/// A setting's string (`ECX` = the setting).
const SETTING_STRING: u32 = 0x004c_69f0;
/// The display name of a reference (`this` only).
const REFERENCE_NAME: u32 = 0x0055_d520;
/// `BSAudio::GetSoundHandleByName` and `BSSoundHandle::Play`.
const AUDIO_SOUND_BY_NAME: u32 = 0x00ad_7550;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// The message box shower (cdecl, six words).
const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// `FalloutRadio::PipboyRadioEnable` (cdecl, one word) and the Pip-Boy
/// station setter `00832240` (cdecl: reference, word).
const PIPBOY_RADIO_ENABLE: u32 = 0x0083_24e0;
const SET_PIPBOY_STATION: u32 = 0x0083_2240;
/// Three float values of a station reference (`this` only; ST0).
const STATION_VALUE_0056B190: u32 = 0x0056_b190;
const STATION_VALUE_0056B1D0: u32 = 0x0056_b1d0;

/// The sound name and flags of the new-station message, its texts and
/// the message time.
const DISCOVERY_SOUND: u32 = 0x0107_ce94;
const DISCOVERY_SETTING: u32 = 0x011d_4348;
const DISCOVERY_TEXT: u32 = 0x0107_ce44;
const MESSAGE_TIME: u32 = 0x0101_62c0;
/// Constants of `UpdateStations`.
const LARGEST_FLOAT: u32 = 0x0101_6970;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
/// The editor id of the sound `Update` looks up.
const ATTENUATION_SOUND_ID: u32 = 0x0107_ce14;

layout! {
    /// `FalloutRadio::FORadioReceiver` (Xbox PDB), 0x20 bytes.
    pub struct FORadioReceiver: 0x20 {
        /// `pRadio` (Xbox PDB): `TESObjectREFR *`.
        0x00 pRadio: u32,
        /// `bShouldBePlaying` (Xbox PDB).
        0x1C bShouldBePlaying: u8,
        /// `bJustStarted` (Xbox PDB).
        0x1D bJustStarted: u8,
        /// `bInRange` (Xbox PDB).
        0x1E bInRange: u8,
        /// `bIsActor` (Xbox PDB).
        0x1F bIsActor: u8,
    }
}

layout! {
    /// `FalloutRadio::StationState` (Xbox PDB), 0x20 bytes.
    pub struct StationState: 0x20 {
        /// `pConvo` (Xbox PDB): `Conversation *`.
        0x00 pConvo: u32,
        /// `pNextConvo` (Xbox PDB).
        0x04 pNextConvo: u32,
        /// `iCurrentItemStartTime` (Xbox PDB).
        0x08 iCurrentItemStartTime: u32,
        /// `iCurrentItemDuration` (Xbox PDB).
        0x0C iCurrentItemDuration: i32,
        /// `cStationPower` (Xbox PDB).
        0x10 cStationPower: u8,
        /// `cTargetPower` (Xbox PDB).
        0x11 cTargetPower: u8,
        /// `iStationFlags` (Xbox PDB).
        0x14 iStationFlags: u32,
    }
}

/// `FalloutRadio::FORadioStation` (Xbox PDB), 0x24 bytes: the station
/// reference, then its `StationState` at +4 (whose `StationUsers` list is
/// at +0x1c of the station).
const STATION_STATE: u32 = 4;
const STATION_USERS: u32 = 0x1c;

fn item_of(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(slot)
}

/// The radio reference of the receiver at a node.
fn radio_of(e: &mut Engine, node: u32) -> u32 {
    let receiver = item_of(e, node);
    e.mem.u32(receiver)
}

/// A flag byte of the receiver at a node.
fn receiver_byte(e: &mut Engine, node: u32, offset: u32) -> u8 {
    let receiver = item_of(e, node);
    e.mem.u8(receiver + offset)
}

/// Whether the reference is an actor (its slot `+0x100`).
fn is_actor(e: &mut Engine, reference: u32) -> bool {
    e.vcall(reference, 0x100, &args![]).bool()
}

fn next_of(e: &mut Engine, node: u32) -> u32 {
    e.call(NODE_NEXT, &args![node]).u32()
}

fn is_empty(e: &mut Engine, node: u32) -> bool {
    e.call(NODE_EMPTY, &args![node]).bool()
}

fn shutting_down(e: &Engine) -> bool {
    e.global::<u8>(SHUTTING_DOWN) != 0
}

/// Stops and releases a sound handle.
fn stop_and_release(e: &mut Engine, handle: u32) {
    e.call(SOUND_HANDLE_STOP, &args![handle]);
    e.call(SOUND_HANDLE_RELEASE, &args![handle]);
}

/// Deletes a conversation through its deleting destructor (when there is
/// one).
fn delete_conversation(e: &mut Engine, conversation: u32) {
    if conversation != 0 {
        e.call(CONVERSATION_DELETE, &args![conversation, 1u32]);
    }
}

/// The audio time (`BSAudio::QInstance()` then `0063d040`).
fn audio_time(e: &mut Engine) -> u32 {
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(AUDIO_TIME, &args![audio]).u32()
}

/// Sets or clears `mask` in `iStationFlags`.
fn set_station_flag(e: &mut Engine, this: Ptr<StationState>, mask: u32, on: u8) {
    let flags = e.get(this, StationState::iStationFlags);
    let flags = if on != 0 { flags | mask } else { flags & !mask };
    e.set(this, StationState::iStationFlags, flags);
}

fn station_flag(e: &Engine, this: Ptr<StationState>, mask: u32) -> bool {
    e.get(this, StationState::iStationFlags) & mask != 0
}

// Translated from 00832010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::Shutdown` (Xbox PDB name from the call graph, cdecl):
/// sets the shutting-down flag, clears every receiver
/// (`ClearInactiveReceivers(1)`), deletes each station's next conversation
/// and the station, clears the station list, deletes the receivers and
/// clears the receiver and discovered lists.
pub fn fallout_radio_shutdown(e: &mut Engine) {
    e.set_global(SHUTTING_DOWN, 1u8);
    fallout_radio_clear_inactive_receivers(e, 1);
    let mut node = STATION_LIST;
    while node != 0 && !is_empty(e, node) {
        let station = item_of(e, node);
        if e.mem.u32(station.wrapping_add(4)) != 0 {
            let next_conversation = item_of(e, node);
            let next_conversation = e.mem.u32(next_conversation.wrapping_add(4));
            delete_conversation(e, next_conversation);
            let station = item_of(e, node);
            e.mem.set_u32(station.wrapping_add(4), 0);
        }
        let station = item_of(e, node);
        if station != 0 {
            fn_00832160(e, Ptr::new(station), 1);
        }
        node = next_of(e, node);
    }
    e.call(LIST_CLEAR, &args![STATION_LIST]);
    let mut node = RECEIVER_LIST;
    while node != 0 && !is_empty(e, node) {
        let receiver = item_of(e, node);
        if receiver != 0 {
            fn_00832190(e, Ptr::new(receiver), 1);
        }
        node = next_of(e, node);
    }
    e.call(LIST_CLEAR, &args![RECEIVER_LIST]);
    e.call(LIST_CLEAR, &args![DISCOVERED_LIST]);
}

// Translated from 00832160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The station's deleting destructor: `008321c0` (the station state's
/// destructor at +4) and, when bit 0 of `flags` is set, `operator delete`.
/// Returns `this`.
pub fn fn_00832160(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_008321c0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00832190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The receiver's deleting destructor: `008321e0` and, when bit 0 of
/// `flags` is set, `operator delete`. Returns `this`.
pub fn fn_00832190(e: &mut Engine, this: Ptr<FORadioReceiver>, flags: u32) -> Ptr<FORadioReceiver> {
    fn_008321e0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008321e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `FORadioReceiver` destructor: destroys the static handle (+0x10)
/// and the sound handle (+4).
// Translation incomplete: 008321e0 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_008321e0(e: &mut Engine, this: Ptr<FORadioReceiver>) {
    e.call(SOUND_HANDLE_DESTRUCTOR, &args![this.byte_add(0x10)]);
    e.call(SOUND_HANDLE_DESTRUCTOR, &args![this.byte_add(4)]);
}

// Translated from 00832420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag `0x100` of `iStationFlags` is set.
pub fn fn_00832420(e: &mut Engine, this: Ptr<StationState>) -> bool {
    station_flag(e, this, 0x100)
}

// Translated from 00832440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag 1 of `iStationFlags`.
pub fn fn_00832440(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 1, on);
}

// Translated from 008325b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Turns a radio reference on or off (cdecl). Nothing while shutting down,
/// when its receiver already is in the wanted state, or when the reference
/// is not a talking activator (`0x15`) with a radio station. On: finds or
/// creates the station (`00832cb0`), creates the receiver when there is
/// none (added to the receiver list), marks it playing and just started,
/// adds it to the station's users, sets the station's flag 1 and, with
/// flag `0x100`, restarts its item in 50 ms (`00832490(0)`). Off: stops and
/// releases both handles, marks it not playing and clears the inactive
/// receivers.
// Translation incomplete: 008325b0 - the C++ exception-unwinding frame (FS:[0]) that frees the new receiver is not translated
pub fn fn_008325b0(e: &mut Engine, radio: u32, on: u8) {
    if shutting_down(e) {
        return;
    }
    let receiver = fallout_radio_get_active_receiver(e, radio);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), receiver);
        let mut added = false;
        if receiver != 0 {
            let playing = e.get(
                Ptr::<FORadioReceiver>::new(receiver),
                FORadioReceiver::bShouldBePlaying,
            );
            if (on != 0 && playing != 0) || (on == 0 && playing == 0) {
                return;
            }
        }
        let mut station = 0u32;
        if radio != 0 {
            let base = e.call(FORM_OF, &args![radio]).u32();
            if e.call(FORM_TYPE, &args![base]).u32() == 0x15 {
                let base = e.call(FORM_OF, &args![radio]).u32();
                station = e.call(RADIO_STATION_FORM, &args![base]).u32();
            }
        }
        if station == 0 {
            return;
        }
        let mut entry = fn_00832830(e, station);
        if on != 0 {
            if entry == 0 {
                entry = fn_00832cb0(e, radio, 0);
                if entry == 0 {
                    return;
                }
            }
            if e.mem.u32(slot.addr()) == 0 {
                let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
                let created = if block != 0 {
                    fn_008327d0(e, Ptr::new(block)).addr()
                } else {
                    0
                };
                e.mem.set_u32(slot.addr(), created);
                added = true;
            }
            let receiver = Ptr::<FORadioReceiver>::new(e.mem.u32(slot.addr()));
            e.set(receiver, FORadioReceiver::pRadio, radio);
            e.set(receiver, FORadioReceiver::bShouldBePlaying, 1);
            e.set(receiver, FORadioReceiver::bJustStarted, 1);
            e.set(receiver, FORadioReceiver::bIsActor, 0);
            if added {
                e.call(LIST_ADD_HEAD, &args![RECEIVER_LIST, slot]);
            }
            e.call(LIST_ADD_HEAD, &args![entry + STATION_USERS, slot]);
            let state = Ptr::<StationState>::new(entry + STATION_STATE);
            fn_00832440(e, state, 1);
            if fn_00832420(e, state) {
                let now = audio_time(e);
                e.set(
                    state,
                    StationState::iCurrentItemStartTime,
                    now.wrapping_add(0x32),
                );
                e.call(STATE_CALL_00832490, &args![state, 0u32]);
            }
        } else if receiver != 0 {
            stop_and_release(e, receiver + 4);
            stop_and_release(e, receiver + 0x10);
            e.set(
                Ptr::<FORadioReceiver>::new(receiver),
                FORadioReceiver::bShouldBePlaying,
                0,
            );
            fallout_radio_clear_inactive_receivers(e, 0);
        }
    });
}

// Translated from 008327d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `FORadioReceiver` constructor: constructs the sound handle (+4) and
/// the static handle (+0x10). Returns `this`. (The engine map's name,
/// `BSSimpleList<FalloutRadio::FORadioReceiver_P>::AddHead`, does not fit
/// the body.)
// Translation incomplete: 008327d0 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_008327d0(e: &mut Engine, this: Ptr<FORadioReceiver>) -> Ptr<FORadioReceiver> {
    e.call(SOUND_HANDLE_CONSTRUCT, &args![this.byte_add(4)]);
    e.call(SOUND_HANDLE_CONSTRUCT, &args![this.byte_add(0x10)]);
    this
}

// Translated from 00832830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the station entry of a station form (cdecl): among the stations
/// whose reference's form is `form`, the first; a later one replaces it
/// when the first's reference passes `00440da0` and the later's does not,
/// when the best's `0056b210` word is 0 and the later's is not, or when the
/// best's word is not 1 and the later's is 1. 0 while shutting down.
pub fn fn_00832830(e: &mut Engine, form: u32) -> u32 {
    let mut best = 0u32;
    let mut best_word = 0i32;
    if shutting_down(e) {
        return 0;
    }
    let mut node = STATION_LIST;
    while node != 0 && !is_empty(e, node) {
        let station = item_of(e, node);
        let reference = e.mem.u32(station);
        if e.call(FORM_OF, &args![reference]).u32() == form {
            let take = best == 0 || {
                let best_reference = e.mem.u32(best);
                let replaces = e
                    .call(REFERENCE_TEST_00440DA0, &args![best_reference])
                    .bool()
                    && {
                        let station = item_of(e, node);
                        let reference = e.mem.u32(station);
                        !e.call(REFERENCE_TEST_00440DA0, &args![reference]).bool()
                    };
                replaces
                    || (best_word == 0 && {
                        let station = item_of(e, node);
                        let reference = e.mem.u32(station);
                        e.call(REFERENCE_VALUE_0056B210, &args![reference]).u32() != 0
                    })
                    || (best_word != 1 && {
                        let station = item_of(e, node);
                        let reference = e.mem.u32(station);
                        e.call(REFERENCE_VALUE_0056B210, &args![reference]).u32() == 1
                    })
            };
            if take {
                best = item_of(e, node);
                let reference = e.mem.u32(best);
                best_word = e.call(REFERENCE_VALUE_0056B210, &args![reference]).i32();
            }
        }
        node = next_of(e, node);
    }
    best
}

// Translated from 00832930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::GetActiveReceiver` (Xbox PDB name from the call graph,
/// cdecl): the first receiver whose radio is `radio`, or 0.
pub fn fallout_radio_get_active_receiver(e: &mut Engine, radio: u32) -> u32 {
    let mut found = 0u32;
    let mut done = false;
    let mut node = RECEIVER_LIST;
    while node != 0 && !is_empty(e, node) && !done {
        let receiver = item_of(e, node);
        if radio == e.mem.u32(receiver) {
            found = item_of(e, node);
            done = true;
        }
        node = next_of(e, node);
    }
    found
}

// Translated from 008329a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::ClearInactiveReceivers` (Xbox PDB name from the call
/// graph, cdecl): removes every receiver that should not be playing (all
/// of them with `force`) from the receiver list and from every station's
/// users (stopping and releasing its sound handle each time), then deletes
/// it; the walk restarts after each removal.
pub fn fallout_radio_clear_inactive_receivers(e: &mut Engine, force: u8) {
    e.with_stack(4, |e, slot| {
        let mut node = RECEIVER_LIST;
        while node != 0 && !is_empty(e, node) {
            let receiver = item_of(e, node);
            let playing = e.mem.u8(receiver.wrapping_add(0x1c));
            if playing != 0 && force == 0 {
                node = next_of(e, node);
                continue;
            }
            let receiver = item_of(e, node);
            e.mem.set_u32(slot.addr(), receiver);
            let item = e.call(NODE_ITEM, &args![node]).u32();
            e.call(LIST_REMOVE, &args![RECEIVER_LIST, item]);
            node = RECEIVER_LIST;
            let mut station_node = STATION_LIST;
            while station_node != 0 && !is_empty(e, station_node) {
                let station = item_of(e, station_node);
                if station != 0 {
                    while e
                        .call(LIST_CONTAINS, &args![station + STATION_USERS, slot])
                        .bool()
                    {
                        let receiver = e.mem.u32(slot.addr());
                        stop_and_release(e, receiver.wrapping_add(4));
                        e.call(LIST_REMOVE, &args![station + STATION_USERS, slot]);
                    }
                }
                station_node = next_of(e, station_node);
            }
            let receiver = e.mem.u32(slot.addr());
            if receiver != 0 {
                fn_00832190(e, Ptr::new(receiver), 1);
            }
        }
    });
}

// Translated from 00832ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::Update` (Xbox PDB, cdecl): decides whether the stations
/// get a full update (the update interval setting `011dd5cc` passed and no
/// loading menu; the player's check `00425fd0` once, the first time it
/// passes; or `force`), applies the queued state flags (1 or 2) to the
/// radio state, clears the inactive receivers, looks up the sound
/// `0107ce14` once, updates the stations, clears the inactive receivers
/// again and clears the radio state. Answers true.
pub fn fallout_radio_update(e: &mut Engine, force: u8) -> bool {
    let mut full = false;
    if e.global::<u32>(LAST_UPDATE) == 0 {
        let now = e.call(TIMER_TIME, &args![TIMER]).u32();
        e.set_global(LAST_UPDATE, now);
    }
    let elapsed = e
        .call(TIMER_TIME, &args![TIMER])
        .u32()
        .wrapping_sub(e.global::<u32>(LAST_UPDATE));
    let interval = e
        .call(SETTING_VALUE_ADDRESS, &args![UPDATE_INTERVAL_SETTING])
        .u32();
    if elapsed > e.mem.u32(interval) && !e.call(LOADING_MENU_VISIBLE, &args![]).bool() {
        full = true;
        let now = e.call(TIMER_TIME, &args![TIMER]).u32();
        e.set_global(LAST_UPDATE, now);
    }
    let player = e.global::<u32>(PLAYER);
    if player != 0 && e.call(PLAYER_OBJECT, &args![player]).u32() != 0 {
        let player = e.global::<u32>(PLAYER);
        let object = e.call(PLAYER_OBJECT, &args![player]).u32();
        if e.call(PLAYER_OBJECT_TEST, &args![object]).bool() {
            full = e.global::<u8>(PLAYER_CHECK_SEEN) == 0;
            e.set_global(PLAYER_CHECK_SEEN, 1u8);
        }
    }
    if force != 0 {
        full = true;
    }
    let queued = e.global::<u32>(QUEUED_STATE);
    if queued & 1 != 0 {
        let state = e.global::<u32>(RADIO_STATE) & !2;
        e.set_global(RADIO_STATE, state | 1);
    } else if queued & 2 != 0 {
        let state = e.global::<u32>(RADIO_STATE) & !1;
        e.set_global(RADIO_STATE, state | 2);
    }
    e.set_global(QUEUED_STATE, 0u32);
    fallout_radio_clear_inactive_receivers(e, 0);
    if e.global::<u32>(ATTENUATION_SOUND) == 0 {
        let handler = e.global::<u32>(DATA_HANDLER);
        let sound = e
            .call(GET_SOUND, &args![handler, ATTENUATION_SOUND_ID])
            .u32();
        e.set_global(ATTENUATION_SOUND, sound);
    }
    fallout_radio_update_stations(e, u8::from(full));
    fallout_radio_clear_inactive_receivers(e, 0);
    e.set_global(RADIO_STATE, 0u32);
    true
}

// Translated from 00832cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the station entry of a reference (cdecl): for a radio-kind
/// reference (form types `0x15`, `0x2a`..`0x2d`) that is not an actor, the
/// radio station of its talking activator found in range of itself
/// (`008356e0`); for an actor of those kinds with `other`, the station of
/// `other`'s form; for form type `0x16`, the reference itself. Without one
/// the new entry is deleted and the answer is 0. The entry starts with no
/// conversation, an item start a random 0..30 s in the past, duration 1,
/// flags `0x20`/`0x10` from the form's slot `+0xb4`, flag 2, both powers
/// 100, and is added to the station list.
// Translation incomplete: 00832cb0 - the C++ exception-unwinding frame (FS:[0]) that frees the new entry is not translated
pub fn fn_00832cb0(e: &mut Engine, reference: u32, other: u32) -> u32 {
    if reference == 0 {
        return 0;
    }
    let block = e.call(OPERATOR_NEW, &args![0x24u32]).u32();
    let entry = if block != 0 {
        fn_00833150(e, Ptr::new(block)).addr()
    } else {
        0
    };
    let mut radio_kind = false;
    for kind in [0x15u32, 0x2a, 0x2b, 0x2c, 0x2d] {
        let base = e.call(FORM_OF, &args![reference]).u32();
        if e.call(FORM_TYPE, &args![base]).u32() == kind {
            radio_kind = true;
            break;
        }
    }
    let base = e.call(FORM_OF, &args![reference]).u32();
    let talker_kind = e.call(FORM_TYPE, &args![base]).u32() == 0x16;
    let delete_entry = |e: &mut Engine| {
        if entry != 0 {
            fn_00832160(e, Ptr::new(entry), 1);
        }
    };
    let mut station = 0u32;
    if radio_kind && !e.vcall(reference, 0x100, &args![]).bool() {
        let base = e.call(FORM_OF, &args![reference]).u32();
        let form = e.call(RADIO_STATION_FORM, &args![base]).u32();
        if form == 0 {
            delete_entry(e);
            return 0;
        }
        station = fn_008356e0(e, form, reference);
        if station == 0 {
            delete_entry(e);
            return 0;
        }
    } else if radio_kind && e.vcall(reference, 0x100, &args![]).bool() && other != 0 {
        let form = e.call(FORM_OF, &args![other]).u32();
        if form == 0 {
            delete_entry(e);
            return 0;
        }
        station = fn_008356e0(e, form, reference);
        if station == 0 {
            delete_entry(e);
            return 0;
        }
    } else if talker_kind {
        station = reference;
    }
    if station == 0 {
        delete_entry(e);
        return 0;
    }
    e.mem.set_u32(entry, station);
    let state = Ptr::<StationState>::new(entry + STATION_STATE);
    e.set(state, StationState::pConvo, 0);
    e.set(state, StationState::pNextConvo, 0);
    let now = audio_time(e);
    let back = e.call(RANDOM_RANGE, &args![0u32, 0x7530u32]).u32();
    e.set(
        state,
        StationState::iCurrentItemStartTime,
        now.wrapping_sub(back),
    );
    e.set(state, StationState::iCurrentItemDuration, 1);
    let form = e.call(FORM_OF, &args![e.mem.u32(entry)]).u32();
    let flag = e.vcall(form, 0xb4, &args![]).u8();
    fn_00833100(e, state, flag);
    let same = u8::from(fn_00833040(e, state));
    fn_008330b0(e, state, same);
    fn_00833060(e, state, 1);
    e.set(state, StationState::cStationPower, 100);
    e.set(state, StationState::cTargetPower, 100);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), entry);
        e.call(LIST_ADD_HEAD, &args![STATION_LIST, slot]);
    });
    entry
}

// Translated from 00833040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag `0x20` of `iStationFlags` is set.
pub fn fn_00833040(e: &mut Engine, this: Ptr<StationState>) -> bool {
    station_flag(e, this, 0x20)
}

// Translated from 00833060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag 2 of `iStationFlags`.
pub fn fn_00833060(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 2, on);
}

// Translated from 008330b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag `0x10` of `iStationFlags`.
pub fn fn_008330b0(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 0x10, on);
}

// Translated from 00833100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag `0x20` of `iStationFlags`.
pub fn fn_00833100(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 0x20, on);
}

// Translated from 00833150 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `FORadioStation` constructor: constructs the station state at +4.
/// Returns `this`.
pub fn fn_00833150(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00833170(e, Ptr::new(this.addr() + STATION_STATE));
    this
}

// Translated from 00833170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StationState` constructor: constructs the users list (+0x18) and
/// clears both conversations, the start time, the duration and the flags
/// (the two power bytes are left as they are). Returns `this`.
pub fn fn_00833170(e: &mut Engine, this: Ptr<StationState>) -> Ptr<StationState> {
    e.call(LIST_CONSTRUCTOR, &args![this.byte_add(0x18)]);
    e.set(this, StationState::pConvo, 0);
    e.set(this, StationState::pNextConvo, 0);
    e.set(this, StationState::iCurrentItemStartTime, 0);
    e.set(this, StationState::iCurrentItemDuration, 0);
    e.set(this, StationState::iStationFlags, 0);
    this
}

// Translated from 00833c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cStationPower`, in `AL`; the code loads `this` into `EAX` first, so the
/// upper bytes of the result are `this`'s.
pub fn fn_00833c20(e: &mut Engine, this: Ptr<StationState>) -> u32 {
    let power = e.get(this, StationState::cStationPower);
    (this.addr() & !0xff) | u32::from(power)
}

// Translated from 00833c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag 8 of `iStationFlags`.
pub fn fn_00833c40(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 8, on);
}

// Translated from 00833d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::UpdateStations` (Xbox PDB, cdecl). With `full`: the
/// stations in range of the player (a reference list and a float list)
/// are walked and each new one (not pending-disabled, not yet reported) is
/// reported with a message (unless the skip byte is set) and remembered;
/// the skip byte is cleared. Then for each station: with `full` and the
/// station in range, its power is computed from the range values
/// (`0056b190`, `0056b1d0`, the signal setting `011dd580`, x87) and stored
/// (`00458b30`) and flag 4 cleared; out of range, flag 4 is set and the
/// power 0. A station in range that is the lost Pip-Boy station is
/// reported and handed back to the Pip-Boy. Each station is then updated
/// (`00834260`). Nothing while shutting down.
// Translation incomplete: 00833d00 - the C++ exception-unwinding frame (FS:[0]) that destroys the lists, strings and sound handles is not translated
pub fn fallout_radio_update_stations(e: &mut Engine, full: u8) {
    e.with_stack(0x10, |e, lists| {
        let references = lists.addr();
        let ranges = lists.addr() + 8;
        e.call(LIST_CONSTRUCTOR, &args![references]);
        fn_008373d0(e, Ptr::new(ranges));
        if shutting_down(e) {
            e.call(FLOAT_LIST_DESTRUCTOR, &args![ranges]);
            e.call(LIST_DESTRUCTOR, &args![references]);
            return;
        }
        if full != 0 {
            report_new_stations(e, references, ranges);
        }
        let signal = e.call(SETTING_FLOAT_ADDRESS, &args![SIGNAL_SETTING]).u32();
        let mut falloff = x87_store32(x87_sub(1.0, x87_m32(e.mem.u32(signal))));
        let mut node = STATION_LIST;
        while node != 0 && !is_empty(e, node) {
            let mut reference_node = references;
            let mut range_node = ranges;
            let mut found = false;
            // The code resets the range to the largest float here; the value
            // is overwritten before any read (only a station in range reads it).
            let station = item_of(e, node);
            if station != 0 {
                while full != 0 && reference_node != 0 && range_node != 0 {
                    if item_of(e, reference_node) != e.mem.u32(station) {
                        reference_node = next_of(e, reference_node);
                        range_node = next_of(e, range_node);
                        continue;
                    }
                    found = true;
                    let mut power = 100u8;
                    fn_00834210(e, Ptr::new(station + STATION_STATE), 0);
                    let slot = e.call(NODE_ITEM, &args![range_node]).u32();
                    let range = x87_quiet_bits(e.mem.u32(slot));
                    let value = e
                        .call(STATION_VALUE_0056B190, &args![e.mem.u32(station)])
                        .f64();
                    let station_range = x87_store32(value);
                    let strength = e
                        .call(STATION_VALUE_0056B1D0, &args![e.mem.u32(station)])
                        .f64();
                    if strength != e.global::<f64>(ZERO_DOUBLE) {
                        let strength = e
                            .call(STATION_VALUE_0056B1D0, &args![e.mem.u32(station)])
                            .f64();
                        let scaled = x87_div(strength, e.global::<f64>(HUNDRED_DOUBLE));
                        falloff = x87_store32(x87_sub(1.0, scaled));
                    }
                    let zero = e.global::<f64>(ZERO_DOUBLE);
                    if e.call(REFERENCE_VALUE_0056B210, &args![e.mem.u32(station)])
                        .u32()
                        == 0
                        && x87_m32(station_range) > zero
                        && x87_m32(range) > zero
                    {
                        let ratio = x87_store32(x87_div(x87_m32(range), x87_m32(station_range)));
                        // FCOMPP; TEST AH,5; JP: only when the falloff is
                        // below the ratio.
                        if x87_m32(falloff) < x87_m32(ratio) {
                            let over = x87_sub(x87_m32(ratio), x87_m32(falloff));
                            let span = x87_sub(1.0, x87_m32(falloff));
                            let lost =
                                x87_mul(x87_div(over, span), e.global::<f64>(HUNDRED_DOUBLE));
                            let lost = crate::x87::fistp_chop(lost);
                            power = 100u8.wrapping_sub(lost as u8);
                        }
                    }
                    e.call(
                        STATE_SET_POWER,
                        &args![station + STATION_STATE, u32::from(power)],
                    );
                    reference_node = 0;
                    range_node = 0;
                }
                if full != 0 && !found {
                    fn_00834210(e, Ptr::new(station + STATION_STATE), 1);
                    e.call(STATE_SET_POWER, &args![station + STATION_STATE, 0u32]);
                }
                if found && e.global::<u32>(LOST_PIPBOY_STATION) == station {
                    e.with_stack(0x14, |e, locals| {
                        let text = locals.addr();
                        let handle = locals.addr() + 8;
                        e.call(STRING_CONSTRUCT, &args![text]);
                        play_discovery_sound(e, handle);
                        show_discovery(e, text, e.mem.u32(station));
                        e.call(PIPBOY_RADIO_ENABLE, &args![1u32]);
                        e.call(SET_PIPBOY_STATION, &args![e.mem.u32(station), 0u32]);
                        e.set_global(LOST_PIPBOY_STATION, 0u32);
                        e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
                        e.call(STRING_DESTROY, &args![text]);
                    });
                }
                fn_00834260(e, station);
            }
            node = next_of(e, node);
        }
        e.call(FLOAT_LIST_CLEAR, &args![ranges]);
        e.call(LIST_CLEAR, &args![references]);
        e.call(FLOAT_LIST_DESTRUCTOR, &args![ranges]);
        e.call(LIST_DESTRUCTOR, &args![references]);
    });
}

/// Plays the new-station sound into the handle on the stack (flags
/// `0x100121`, `Play(0)` on the handle).
fn play_discovery_sound(e: &mut Engine, handle: u32) {
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(
        AUDIO_SOUND_BY_NAME,
        &args![audio, handle, DISCOVERY_SOUND, 0x0010_0121u32],
    );
    e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
}

/// Formats the station's name with the setting `011d4348` into the string
/// and shows it as a message.
fn show_discovery(e: &mut Engine, text: u32, reference: u32) {
    let name = e.call(REFERENCE_NAME, &args![reference]).u32();
    let format = e.call(SETTING_STRING, &args![DISCOVERY_SETTING]).u32();
    e.call(STRING_FORMAT, &args![text, format, name]);
    let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME));
    let c_str = e.call(STRING_C_STR, &args![text]).u32();
    e.call(
        SHOW_MESSAGE,
        &args![c_str, 0u32, DISCOVERY_TEXT, 0u32, time, 0u32],
    );
}

/// The `full` part of [`fallout_radio_update_stations`] that reports the
/// stations newly in range.
fn report_new_stations(e: &mut Engine, references: u32, ranges: u32) {
    let player = e.global::<u32>(PLAYER);
    e.call(STATIONS_IN_RANGE, &args![player, references, ranges]);
    let mut node = references;
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let reference = item_of(e, node);
        let skip = e.call(REFERENCE_TEST_00440DA0, &args![reference]).bool() && {
            let reference = item_of(e, node);
            !e.call(PENDING_ENABLED, &args![reference]).bool()
        };
        if !skip {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            if !e.call(LIST_CONTAINS, &args![DISCOVERED_LIST, slot]).bool() {
                if e.global::<u8>(SKIP_DISCOVERY_MESSAGE) == 0 {
                    e.with_stack(0x14, |e, locals| {
                        let text = locals.addr();
                        let handle = locals.addr() + 8;
                        e.call(STRING_CONSTRUCT, &args![text]);
                        play_discovery_sound(e, handle);
                        let reference = item_of(e, node);
                        show_discovery(e, text, reference);
                        e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
                        e.call(STRING_DESTROY, &args![text]);
                    });
                }
                let slot = e.call(NODE_ITEM, &args![node]).u32();
                e.call(LIST_ADD_HEAD, &args![DISCOVERED_LIST, slot]);
            }
        }
        node = next_of(e, node);
    }
    e.set_global(SKIP_DISCOVERY_MESSAGE, 0u8);
}

// Translated from 00834210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag 4 of `iStationFlags`.
pub fn fn_00834210(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 4, on);
}

/// Callees of `00834260` (`FalloutRadio::UpdateStation` by the name it
/// enters the radio's critical section with).
const RADIO_CRITICAL_SECTION: u32 = 0x011d_d5e8;
const UPDATE_STATION_NAME: u32 = 0x0107_ceb0;
const ENTER_CRITICAL_SECTION: u32 = 0x0045_38a0;
const LEAVE_CRITICAL_SECTION: u32 = 0x0045_38c0;
const AUDIO_LISTENER_POSITION: u32 = 0x00ad_78e0;
const CURRENT_RESPONSE: u32 = 0x0083_c820;
const NEXT_ITEM: u32 = 0x0083_b9f0;
const FIRST_ITEM: u32 = 0x0083_b9a0;
const PRECACHE_SOUNDS: u32 = 0x0083_c470;
const FIRST_RESPONSE: u32 = 0x0083_c7b0;
const NEXT_RESPONSE: u32 = 0x0083_c7e0;
const RESPONSE_TOPIC_INFO: u32 = 0x0084_e3a0;
const TOPIC_INFO_TEST: u32 = 0x0057_9200;
const RUN_RESULT: u32 = 0x0061_f170;
const STATE_TEST_00833C00: u32 = 0x0083_3c00;
const STATE_CALL_00835690: u32 = 0x0083_5690;
const STATE_TEST_00835670: u32 = 0x0083_5670;
const RADIO_TEST_00833CE0: u32 = 0x0083_3ce0;
const RADIO_CALL_008325A0: u32 = 0x0083_25a0;
const MEDIA_CLOSE: u32 = 0x0083_04a0;
const MEDIA_PAUSE: u32 = 0x0083_05e0;
const MEDIA_UNPAUSE: u32 = 0x0083_0610;
const MEDIA_PLAY_008300C0: u32 = 0x0083_00c0;
const MEDIA_CALL_008304C0: u32 = 0x0083_04c0;
const MEDIA_DURATION: u32 = 0x0083_08c0;
const PIPBOY_UPDATE: u32 = 0x0083_31c0;
const STATE_VALUE_00824060: u32 = 0x0082_4060;
const STATE_CALL_00632D20: u32 = 0x0063_2d20;
const SOUND_IS_VALID: u32 = 0x00ad_8ce0;
const SOUND_PAUSE: u32 = 0x00ad_88b0;
const SOUND_DURATION: u32 = 0x00ad_8b30;
const SOUND_SET_VOLUME: u32 = 0x00ad_89e0;
const SOUND_SET_POSITION: u32 = 0x00ad_8b60;
const SOUND_SET_MIN_MAX: u32 = 0x00ad_8be0;
const SOUND_SET_COMPLETION: u32 = 0x00ad_8e60;
const SOUND_SET_SYNCH_START: u32 = 0x00ad_8fd0;
const SOUND_SET_FLAG_004F15A0: u32 = 0x004f_15a0;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const AUDIO_SOUND_BY_FILE: u32 = 0x00ad_7480;
const REFERENCE_WORLD_SPACE: u32 = 0x0057_5d70;
const VECTOR_SUBTRACT: u32 = 0x0043_9ef0;
const VECTOR_LENGTH: u32 = 0x0045_7990;
const SOUND_FORM_BYTE: u32 = 0x0055_3b90;
const ACTOR_VOICE_TYPE: u32 = 0x0041_81e0;
const TOPIC_INFO_SPEAKER: u32 = 0x0063_9b40;
const RESPONSE_OBJECT: u32 = 0x0044_ddc0;
const RESPONSE_PARENT: u32 = 0x0096_11e0;
const RESPONSE_LOCATION: u32 = 0x0044_1110;
const LIST_COUNT_0044A670: u32 = 0x0044_a670;
const RESPONSE_LIST_00460140: u32 = 0x0046_0140;
const SOUND_FILE_NAME: u32 = 0x0051_1840;
const STRING_FIND: u32 = 0x0048_12f0;
const STRING_FIND_LAST: u32 = 0x0043_b9d0;
const COPY_BOUNDED: u32 = 0x0040_6d30;
const APPEND_BOUNDED: u32 = 0x0040_6d50;
const VOLUME_CURVE: u32 = 0x004d_d150;
const SOUND_SOURCE_CAST: u32 = 0x0040_7840;
const RTTI_CAST: u32 = 0x00ec_43fb;
/// The Pip-Boy's voice and static handles (`PipboyVoiceHndl`,
/// `PipboyStaticHndl` in the Xbox PDB; which is which is not confirmed).
const PIPBOY_HANDLE_5BC: u32 = 0x011d_d5bc;
const PIPBOY_HANDLE_638: u32 = 0x011d_d638;
/// Constants and settings of `00834260`.
const DEFAULT_RANGE: u32 = 0x0104_fca0;
const NO_POSITION: u32 = 0x011f_426c;
const STATIC_SOUND: u32 = 0x0107_ce34;
const VOLUME_EXPONENT: u32 = 0x0101_6264;
const RANGE_SCALE: u32 = 0x0106_ebc8;
const ACTOR_RANGE_SETTING: u32 = 0x011d_d62c;
const ACTOR_RANGE_SCALE: u32 = 0x0102_90f8;
const MIN_RANGE_SETTING: u32 = 0x011d_d5b0;
const VOICE_SUFFIX: u32 = 0x0107_cea8;
const TYPE_FORM_BASE: u32 = 0x0118_3108;
const TYPE_SOUND_SOURCE: u32 = 0x0118_6584;
const COMPLETION_CALLBACK: u32 = 0x0083_57e0;

/// The volume of a power byte, `power / 100.0` as `float`.
fn power_volume(e: &Engine, power: u8) -> u32 {
    x87_store32(x87_div(f64::from(power), e.global::<f64>(HUNDRED_DOUBLE)))
}

/// The static's volume: `004dd150((100 - power) / 100, exponent)`.
fn static_volume(e: &mut Engine, power: u8) -> u32 {
    let exponent = x87_quiet_bits(e.global::<u32>(VOLUME_EXPONENT));
    let lost = 100i32 - i32::from(power);
    let base = x87_store32(x87_div(f64::from(lost), e.global::<f64>(HUNDRED_DOUBLE)));
    let value = e
        .call(VOLUME_CURVE, &args![x87_quiet_bits(base), exponent])
        .f64();
    x87_store32(value)
}

/// Runs the result script of the response's topic info unless its test
/// (`00579200`) passes, then starts its first response and restarts the
/// item in 50 ms.
fn start_response(e: &mut Engine, station: u32) {
    let conversation = e.mem.u32(station + 4);
    let response = e.call(CURRENT_RESPONSE, &args![conversation]).u32();
    if e.call(RESPONSE_TOPIC_INFO, &args![response]).u32() != 0 {
        let conversation = e.mem.u32(station + 4);
        let response = e.call(CURRENT_RESPONSE, &args![conversation]).u32();
        let info = e.call(RESPONSE_TOPIC_INFO, &args![response]).u32();
        if !e.call(TOPIC_INFO_TEST, &args![info]).bool() {
            let reference = e.mem.u32(station);
            let conversation = e.mem.u32(station + 4);
            let response = e.call(CURRENT_RESPONSE, &args![conversation]).u32();
            let info = e.call(RESPONSE_TOPIC_INFO, &args![response]).u32();
            e.call(RUN_RESULT, &args![info, 0u32, reference]);
        }
    }
    let conversation = e.mem.u32(station + 4);
    let response = e.call(CURRENT_RESPONSE, &args![conversation]).u32();
    e.call(FIRST_RESPONSE, &args![response]);
    let now = audio_time(e);
    e.mem.set_u32(station + 0xc, now.wrapping_add(0x32));
    e.mem.set_u32(station + 0x10, 0);
}

/// Stops and releases every user's sound handle.
fn stop_users(e: &mut Engine, station: u32) {
    let mut node = station + STATION_USERS;
    while node != 0 && !is_empty(e, node) {
        let receiver = item_of(e, node);
        e.call(SOUND_HANDLE_STOP, &args![receiver + 4]);
        let receiver = item_of(e, node);
        e.call(SOUND_HANDLE_RELEASE, &args![receiver + 4]);
        node = next_of(e, node);
    }
}

// Translated from 00834260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates one station (cdecl; `FalloutRadio::UpdateStation` by the name
/// it enters the radio's critical section `011dd5e8` with). Nothing for a
/// null station or reference.
///
/// - While shutting down it returns (the critical section is left
///   entered, as the code does); a station that is not the Pip-Boy's and
///   has no users leaves it and returns.
/// - With the station disabled (`00440da0`) or the radio state paused (1)
///   or resumed (2), every user's handles are stopped and released,
///   paused or replayed (playing users), and the Pip-Boy's handles and
///   media follow; then it returns.
/// - Otherwise the item's duration is taken from the Pip-Boy handle or the
///   media (`008308c0(7)`), a start time in the future is pulled back, and
///   when the item is over the conversation moves to its next item
///   (running its result script) or is deleted, the users' handles stop
///   and the Pip-Boy station's media restarts; with no response a new
///   conversation starts when flag `0x20` is set (`GetNextConversation`).
///   A response without a sound restarts in 50 ms.
/// - Each playing user then gets its position (the radio's slot `+0x1f4`),
///   is muted out of the player's worldspace or cell, gets the static
///   (`0107ce34`, volume from the power, `004dd150`) while the power is
///   below 100, and, when it has no sound playing and the station has
///   flag 1, starts the response's voice file (the actor's slot `+0x284`
///   for an NPC whose voice type matches, else by file name with a
///   completion callback `008357e0`) when the listener is within range.
/// - The Pip-Boy update (`008331c0`) runs once, with 1 for a user that
///   did not just start, else with 0 at the end; the critical section is
///   left.
// Translation incomplete: 00834260 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary sound handles and the /GS stack cookie check (00ec408c) are not translated
pub fn fn_00834260(e: &mut Engine, station: u32) {
    if station == 0 || e.mem.u32(station) == 0 {
        return;
    }
    e.with_stack(0x210, |e, frame| {
        update_station(e, station, frame.addr() + 0x210)
    });
}

/// The body of [`fn_00834260`]; `ebp` is the end of the modelled frame
/// (the address-taken locals sit below it at the exe's offsets).
fn update_station(e: &mut Engine, station: u32, ebp: u32) {
    let listener = ebp - 0x24;
    let position = ebp - 0x5c;
    let path = ebp - 0x17c;
    let extension = ebp - 0x184;
    let difference = ebp - 0x1a8;
    let state = Ptr::<StationState>::new(station + STATION_STATE);
    let mut has_voice_file = false;
    let mut has_alternative = false;
    let mut pipboy_updated = false;
    let disabled = e
        .call(REFERENCE_TEST_00440DA0, &args![e.mem.u32(station)])
        .bool();
    let paused = e.global::<u32>(RADIO_STATE) & 1 != 0;
    let resumed = e.global::<u32>(RADIO_STATE) & 2 != 0;
    let is_pipboy = station == e.global::<u32>(CURRENT_PIPBOY_STATION);
    let mut range = x87_quiet_bits(e.global::<u32>(DEFAULT_RANGE));
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(AUDIO_LISTENER_POSITION, &args![audio, listener]);
    e.call(
        ENTER_CRITICAL_SECTION,
        &args![RADIO_CRITICAL_SECTION, UPDATE_STATION_NAME],
    );
    let mut response = if e.mem.u32(station + 4) != 0 {
        e.call(CURRENT_RESPONSE, &args![e.mem.u32(station + 4)])
            .u32()
    } else {
        0
    };
    if shutting_down(e) {
        return;
    }
    if !is_pipboy && is_empty(e, station + STATION_USERS) {
        e.call(LEAVE_CRITICAL_SECTION, &args![RADIO_CRITICAL_SECTION]);
        return;
    }
    let mut sound = e.global::<u32>(ATTENUATION_SOUND);
    if e.mem.u32(station) != 0 {
        let form = e.call(FORM_OF, &args![e.mem.u32(station)]).u32();
        let source = e
            .call(
                RTTI_CAST,
                &args![form, 0i32, TYPE_FORM_BASE, TYPE_SOUND_SOURCE, 0i32],
            )
            .u32();
        if source != 0 && e.call(SOUND_SOURCE_CAST, &args![source]).u32() != 0 {
            sound = e.call(SOUND_SOURCE_CAST, &args![source]).u32();
        }
    }
    if disabled || paused || resumed {
        pause_or_stop(e, station, disabled, paused, resumed, is_pipboy);
        e.call(LEAVE_CRITICAL_SECTION, &args![RADIO_CRITICAL_SECTION]);
        return;
    }
    if e.get(state, StationState::iCurrentItemDuration) <= 0 && is_pipboy {
        if e.call(SOUND_IS_VALID, &args![PIPBOY_HANDLE_5BC]).bool() {
            let duration = e.call(SOUND_DURATION, &args![PIPBOY_HANDLE_5BC]).u32();
            e.mem.set_u32(station + 0x10, duration);
        } else if response != 0 {
            let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
            if e.call(FORM_OF, &args![current]).u32() != 0
                && e.call(MEDIA_DURATION, &args![7i32]).u32() != 0
            {
                let duration = e.call(MEDIA_DURATION, &args![7i32]).u32();
                e.mem.set_u32(station + 0x10, duration);
            }
        }
    }
    let now = audio_time(e);
    if e.mem.u32(station + 0xc) > now.wrapping_add(100) {
        e.mem.set_u32(station + 0xc, now);
    }
    if e.mem.i32(station + 0x10) > 0
        && e.mem
            .u32(station + 0xc)
            .wrapping_add(e.mem.u32(station + 0x10))
            < now
    {
        if e.mem.u32(station + 4) != 0
            && e.call(CURRENT_RESPONSE, &args![e.mem.u32(station + 4)])
                .u32()
                != 0
        {
            if e.call(NEXT_ITEM, &args![e.mem.u32(station + 4)]).bool() {
                start_response(e, station);
                if is_empty(e, station + STATION_USERS)
                    && e.global::<u32>(CURRENT_PIPBOY_STATION) == 0
                {
                    e.call(STATE_CALL_00832490, &args![state, 1u32]);
                }
            } else {
                let conversation = e.mem.u32(station + 4);
                delete_conversation(e, conversation);
                e.mem.set_u32(station + 4, 0);
                response = 0;
                e.mem.set_u32(station + 0xc, 0);
                e.mem.set_u32(station + 0x10, 0);
                if e.call(STATE_TEST_00833C00, &args![state]).bool() {
                    e.call(STATE_CALL_00835690, &args![state, 0u32]);
                }
            }
            stop_users(e, station);
            if station == e.global::<u32>(CURRENT_PIPBOY_STATION) {
                stop_and_release(e, PIPBOY_HANDLE_5BC);
                if !e.call(RADIO_TEST_00833CE0, &args![]).bool() {
                    e.call(MEDIA_CLOSE, &args![]);
                    e.call(RADIO_CALL_008325A0, &args![1u32]);
                    e.call(
                        MEDIA_PLAY_008300C0,
                        &args![7i32, 0u32, 0x3e8u32, 0u32, 0u32, 0.0f32, 0u32],
                    );
                    e.call(MEDIA_CALL_008304C0, &args![]);
                }
            }
        }
        if e.mem.u32(station + 4) == 0
            || e.call(CURRENT_RESPONSE, &args![e.mem.u32(station + 4)])
                .u32()
                == 0
        {
            if fn_00833040(e, state) {
                fn_008330b0(e, state, 1);
                fn_00837060(e, station, 0);
                e.call(PRECACHE_SOUNDS, &args![e.mem.u32(station + 4)]);
                e.call(FIRST_ITEM, &args![e.mem.u32(station + 4)]);
                if e.call(CURRENT_RESPONSE, &args![e.mem.u32(station + 4)])
                    .u32()
                    != 0
                {
                    response = e
                        .call(CURRENT_RESPONSE, &args![e.mem.u32(station + 4)])
                        .u32();
                    start_response(e, station);
                    stop_users(e, station);
                    if station == e.global::<u32>(CURRENT_PIPBOY_STATION) {
                        stop_and_release(e, PIPBOY_HANDLE_5BC);
                    }
                }
            } else {
                fn_008330b0(e, state, 0);
            }
        }
    }
    if e.mem.u32(station + 4) != 0
        && e.call(CURRENT_RESPONSE, &args![e.mem.u32(station + 4)])
            .u32()
            != 0
    {
        response = e
            .call(CURRENT_RESPONSE, &args![e.mem.u32(station + 4)])
            .u32();
        if e.call(CURRENT_RESPONSE, &args![response]).u32() == 0
            && !e.call(NEXT_RESPONSE, &args![response]).bool()
        {
            let now = audio_time(e);
            e.mem.set_u32(station + 0xc, now.wrapping_add(0x32));
            e.mem.set_u32(station + 0x10, 1);
        }
    }
    let byte = e.call(STATE_VALUE_00824060, &args![state]).u32() & 0xff;
    e.call(STATE_CALL_00632D20, &args![state, byte]);
    let power = fn_00833c20(e, state) as u8;
    let static_on = power < 100;
    let mut node = station + STATION_USERS;
    while node != 0 && !is_empty(e, node) {
        let receiver = item_of(e, node);
        if e.mem.u8(receiver + 0x1c) == 0 {
            node = next_of(e, node);
            continue;
        }
        if e.mem.i32(station + 0x10) <= 0 {
            let receiver = item_of(e, node);
            if e.call(SOUND_IS_VALID, &args![receiver + 4]).bool() {
                let receiver = item_of(e, node);
                if e.call(SOUND_DURATION, &args![receiver + 4]).u32() != 0 {
                    let receiver = item_of(e, node);
                    let duration = e.call(SOUND_DURATION, &args![receiver + 4]).u32();
                    e.mem.set_u32(station + 0x10, duration);
                }
            }
        }
        for i in 0..3 {
            let word = e.mem.u32(NO_POSITION + 4 * i);
            e.mem.set_u32(position + 4 * i, word);
        }
        for i in 0..3 {
            let largest = x87_quiet_bits(e.global::<u32>(LARGEST_FLOAT));
            e.mem.set_u32(position + 4 * i, largest);
        }
        let receiver = item_of(e, node);
        if receiver != 0 && e.mem.u32(receiver) != 0 {
            let radio = radio_of(e, node);
            let at = e.vcall(radio, 0x1f4, &args![]).u32();
            for i in 0..3 {
                let word = e.mem.u32(at + 4 * i);
                e.mem.set_u32(position + 4 * i, word);
            }
        } else if receiver != 0 {
            let receiver = item_of(e, node);
            e.mem.set_u8(receiver + 0x1c, 0);
            node = next_of(e, node);
            continue;
        }
        if out_of_reach(e, node) {
            let receiver = item_of(e, node);
            e.call(SOUND_SET_VOLUME, &args![receiver + 4, 0.0f32]);
            let receiver = item_of(e, node);
            e.mem.set_u8(receiver + 0x1c, 0);
        }
        if item_of(e, node) != 0 {
            let receiver = item_of(e, node);
            if e.call(SOUND_IS_VALID, &args![receiver + 4]).bool() {
                let handle = item_of(e, node) + 4;
                set_position(e, handle, position);
            }
        }
        update_static(e, node, power, static_on, position);
        let receiver = item_of(e, node);
        let can_start = response != 0
            && !e.call(SOUND_IS_VALID, &args![receiver + 4]).bool()
            && fn_00835650(e, state)
            && receiver_byte(e, node, 0x1c) != 0
            && radio_of(e, node) != 0;
        if !can_start {
            let receiver = item_of(e, node);
            if e.mem.u32(receiver) == 0 {
                e.mem.set_u8(receiver + 0x1c, 0);
            }
            node = next_of(e, node);
            continue;
        }
        if e.call(CURRENT_RESPONSE, &args![response]).u32() == 0 {
            if !e.call(NEXT_RESPONSE, &args![response]).bool()
                && !e.call(FIRST_RESPONSE, &args![response]).bool()
            {
                let now = audio_time(e);
                e.mem.set_u32(station + 0xc, now.wrapping_add(0x32));
                e.mem.set_u32(station + 0x10, 1);
            }
            node = next_of(e, node);
            continue;
        }
        let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
        if e.call(FORM_OF, &args![current]).u32() != 0 {
            has_voice_file = true;
        }
        let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
        if current != 0 {
            let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
            let slot = e.call(NODE_ITEM, &args![current]).u32();
            if e.call(STRING_C_STR, &args![slot]).u32() != 0 {
                let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
                let slot = e.call(NODE_ITEM, &args![current]).u32();
                let list = e.call(STRING_C_STR, &args![slot]).u32();
                let count = e.call(LIST_COUNT_0044A670, &args![list]).u32();
                has_alternative = count > 1 && {
                    let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
                    let alternatives = e.call(RESPONSE_LIST_00460140, &args![current]).u32();
                    e.call(STRING_C_STR, &args![alternatives]).u32() != 0
                };
            }
        }
        if has_voice_file {
            let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
            let form = e.call(FORM_OF, &args![current]).u32();
            let name = e.call(SOUND_FILE_NAME, &args![form]).u32();
            e.call(COPY_BOUNDED, &args![path, 0x104u32, name]);
            if e.call(STRING_FIND, &args![path, VOICE_SUFFIX]).u32() == 0 {
                let dot = e.call(STRING_FIND_LAST, &args![path, 0x2ei32]).u32();
                if dot != 0 {
                    e.call(COPY_BOUNDED, &args![extension, 4u32, dot + 1]);
                    e.mem.set_u8(dot, 0);
                    e.call(APPEND_BOUNDED, &args![path, 0x104u32, VOICE_SUFFIX]);
                    e.call(APPEND_BOUNDED, &args![path, 0x104u32, extension]);
                }
            }
            fn_00833c40(e, state, 1);
        } else if has_alternative {
            let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
            let alternatives = e.call(RESPONSE_LIST_00460140, &args![current]).u32();
            let text = e.call(STRING_C_STR, &args![alternatives]).u32();
            e.call(COPY_BOUNDED, &args![path, 0x104u32, text]);
            fn_00833c40(e, state, 0);
        } else {
            e.mem.set_u32(station + 0x10, 1);
            node = next_of(e, node);
            fn_00833c40(e, state, 0);
            continue;
        }
        let mut voice_match = false;
        let mut is_actor_speaker = false;
        let mut actor = 0u32;
        let radio = radio_of(e, node);
        if radio != 0 && is_actor(e, radio) {
            actor = radio_of(e, node);
            if e.call(RESPONSE_TOPIC_INFO, &args![response]).u32() != 0 {
                let voice = e.call(ACTOR_VOICE_TYPE, &args![actor]).u32();
                let info = e.call(RESPONSE_TOPIC_INFO, &args![response]).u32();
                if voice == e.call(TOPIC_INFO_SPEAKER, &args![info]).u32() {
                    voice_match = true;
                }
            }
            if e.vcall(actor, 0x21c, &args![]).bool() {
                is_actor_speaker = true;
            }
        }
        if sound != 0 && !is_actor_speaker {
            let byte = e.call(SOUND_FORM_BYTE, &args![sound]).u8();
            range = x87_store32(x87_mul(f64::from(byte), e.global::<f64>(RANGE_SCALE)));
        } else if is_actor_speaker {
            let setting = e
                .call(SETTING_FLOAT_ADDRESS, &args![ACTOR_RANGE_SETTING])
                .u32();
            range = x87_store32(x87_mul(
                x87_m32(e.mem.u32(setting)),
                e.global::<f64>(ACTOR_RANGE_SCALE),
            ));
        }
        let away = e
            .call(VECTOR_SUBTRACT, &args![listener, difference, position])
            .u32();
        let distance = e.call(VECTOR_LENGTH, &args![away]).f64();
        // FCOMPP; TEST AH,5; JP: a user beyond the range (the range below
        // the distance) is skipped; NaN is not.
        if x87_m32(range) < distance {
            node = next_of(e, node);
            continue;
        }
        audio_time(e);
        if actor != 0 && voice_match {
            speak_with_actor(e, actor, node, response, path);
        } else {
            let voice = Voice {
                station,
                sound,
                is_actor_speaker,
                power,
                position,
                path,
                ebp,
            };
            play_voice_file(e, node, &voice);
        }
        if is_pipboy && receiver_byte(e, node, 0x1d) == 0 && !pipboy_updated {
            pipboy_updated = true;
            e.call(PIPBOY_UPDATE, &args![1u32]);
        }
        if receiver_byte(e, node, 0x1d) != 0 {
            let receiver = item_of(e, node);
            e.mem.set_u8(receiver + 0x1d, 0);
        }
        node = next_of(e, node);
    }
    if station == e.global::<u32>(CURRENT_PIPBOY_STATION) && !pipboy_updated {
        e.call(PIPBOY_UPDATE, &args![0u32]);
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![RADIO_CRITICAL_SECTION]);
}

/// Whether a user's radio is out of the player's reach: another
/// worldspace (when the player has one), or, when the player's object
/// passes `00425fd0`, another object (`008d6f30`).
fn out_of_reach(e: &mut Engine, node: u32) -> bool {
    let player = e.global::<u32>(PLAYER);
    if e.call(REFERENCE_WORLD_SPACE, &args![player]).u32() != 0 {
        let player = e.global::<u32>(PLAYER);
        let mine = e.call(REFERENCE_WORLD_SPACE, &args![player]).u32();
        let radio = radio_of(e, node);
        if mine != e.call(REFERENCE_WORLD_SPACE, &args![radio]).u32() {
            return true;
        }
    }
    let player = e.global::<u32>(PLAYER);
    let object = e.call(PLAYER_OBJECT, &args![player]).u32();
    if !e.call(PLAYER_OBJECT_TEST, &args![object]).bool() {
        return false;
    }
    let radio = radio_of(e, node);
    let theirs = e.call(PLAYER_OBJECT, &args![radio]).u32();
    let player = e.global::<u32>(PLAYER);
    theirs != e.call(PLAYER_OBJECT, &args![player]).u32()
}

/// Sets a handle's position from the three floats at `position` (each
/// copied as an `FLD`/`FSTP` copy).
fn set_position(e: &mut Engine, handle: u32, position: u32) {
    let x = x87_quiet_bits(e.mem.u32(position));
    let y = x87_quiet_bits(e.mem.u32(position + 4));
    let z = x87_quiet_bits(e.mem.u32(position + 8));
    e.call(SOUND_SET_POSITION, &args![handle, x, y, z]);
}

/// The disabled / paused / resumed part of [`fn_00834260`].
fn pause_or_stop(
    e: &mut Engine,
    station: u32,
    disabled: bool,
    paused: bool,
    resumed: bool,
    is_pipboy: bool,
) {
    let mut node = station + STATION_USERS;
    while node != 0 && !is_empty(e, node) {
        let receiver = item_of(e, node);
        if e.call(SOUND_IS_VALID, &args![receiver + 4]).bool() {
            if disabled {
                for offset in [4u32, 0x10] {
                    let receiver = item_of(e, node);
                    e.call(SOUND_HANDLE_STOP, &args![receiver + offset]);
                    let receiver = item_of(e, node);
                    e.call(SOUND_HANDLE_RELEASE, &args![receiver + offset]);
                }
            } else if paused {
                for offset in [4u32, 0x10] {
                    let receiver = item_of(e, node);
                    e.call(SOUND_PAUSE, &args![receiver + offset]);
                }
            } else if resumed && receiver_byte(e, node, 0x1c) != 0 {
                for offset in [4u32, 0x10] {
                    let receiver = item_of(e, node);
                    e.call(SOUND_HANDLE_PLAY, &args![receiver + offset, 0u32]);
                }
            }
        }
        node = next_of(e, node);
    }
    if !is_pipboy {
        return;
    }
    if disabled {
        e.call(PIPBOY_RADIO_ENABLE, &args![0u32]);
    }
    let current = e.global::<u32>(CURRENT_PIPBOY_STATION);
    if current != 0
        && e.call(STATE_TEST_00835670, &args![current + STATION_STATE])
            .bool()
    {
        if paused {
            e.call(SOUND_PAUSE, &args![PIPBOY_HANDLE_638]);
            e.call(MEDIA_PAUSE, &args![]);
        } else if resumed {
            e.call(SOUND_HANDLE_PLAY, &args![PIPBOY_HANDLE_638, 0u32]);
            e.call(MEDIA_UNPAUSE, &args![]);
        }
    } else if paused {
        e.call(SOUND_PAUSE, &args![PIPBOY_HANDLE_638]);
        e.call(SOUND_PAUSE, &args![PIPBOY_HANDLE_5BC]);
    } else if resumed {
        e.call(SOUND_HANDLE_PLAY, &args![PIPBOY_HANDLE_638, 0u32]);
        e.call(SOUND_HANDLE_PLAY, &args![PIPBOY_HANDLE_5BC, 0u32]);
    }
}

/// The static of a user in [`fn_00834260`]: started (`0107ce34`) when the
/// power is below 100 and none plays, kept at the power's volume and the
/// position while it plays, stopped when the power is back to 100.
fn update_static(e: &mut Engine, node: u32, power: u8, static_on: bool, position: u32) {
    let handle = item_of(e, node) + 0x10;
    if static_on && !e.call(SOUND_IS_VALID, &args![handle]).bool() {
        let handle = item_of(e, node) + 0x10;
        stop_and_release(e, handle);
        e.with_stack(0xc, |e, temporary| {
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            let created = e
                .call(
                    AUDIO_SOUND_BY_NAME,
                    &args![audio, temporary, STATIC_SOUND, 0x0010_0012u32],
                )
                .u32();
            let handle = item_of(e, node) + 0x10;
            e.call(SOUND_HANDLE_ASSIGN, &args![handle, created]);
            e.call(SOUND_HANDLE_DESTRUCTOR, &args![temporary]);
        });
        let handle = item_of(e, node) + 0x10;
        e.call(SOUND_SET_FLAG_004F15A0, &args![handle, 1u32]);
        let handle = item_of(e, node) + 0x10;
        set_position(e, handle, position);
        let volume = static_volume(e, power);
        let handle = item_of(e, node) + 0x10;
        e.call(SOUND_SET_VOLUME, &args![handle, volume]);
        let handle = item_of(e, node) + 0x10;
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
    } else if static_on {
        let volume = static_volume(e, power);
        let handle = item_of(e, node) + 0x10;
        e.call(SOUND_SET_VOLUME, &args![handle, volume]);
        let handle = item_of(e, node) + 0x10;
        set_position(e, handle, position);
    } else {
        let handle = item_of(e, node) + 0x10;
        if e.call(SOUND_IS_VALID, &args![handle]).bool() {
            let handle = item_of(e, node) + 0x10;
            stop_and_release(e, handle);
        }
    }
}

/// An NPC speaking the response itself (its slot `+0x284`, the float
/// result dropped).
fn speak_with_actor(e: &mut Engine, actor: u32, node: u32, response: u32, path: u32) {
    let radio = radio_of(e, node);
    let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
    let location = e.call(RESPONSE_LOCATION, &args![current]).u32();
    let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
    let parent = e.call(RESPONSE_PARENT, &args![current]).u32();
    let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
    let info = e.call(RESPONSE_TOPIC_INFO, &args![current]).u32();
    let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
    let object = e.call(RESPONSE_OBJECT, &args![current]).u32();
    let handle = item_of(e, node) + 4;
    e.vcall(
        actor,
        0x284,
        &args![
            path, handle, object, info, 0x96u32, parent, location, radio, 0u32, 0u32, 0u32, 1u32,
            1u32
        ],
    );
}

/// What [`play_voice_file`] plays and where.
struct Voice {
    station: u32,
    sound: u32,
    is_actor_speaker: bool,
    power: u8,
    position: u32,
    path: u32,
    ebp: u32,
}

/// Starts the voice file at `path` on a user's handle with the completion
/// callback, start time, position and the power's volume (an actor
/// speaker's handle also gets the two range settings).
fn play_voice_file(e: &mut Engine, node: u32, voice: &Voice) {
    let flags = 0x0050_0102u32;
    let temporary = if voice.is_actor_speaker {
        voice.ebp - 0x1b4
    } else {
        voice.ebp - 0x1c0
    };
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    let form = if voice.is_actor_speaker {
        0
    } else {
        voice.sound
    };
    let created = e
        .call(
            AUDIO_SOUND_BY_FILE,
            &args![audio, temporary, voice.path, flags, form],
        )
        .u32();
    let handle = item_of(e, node) + 4;
    e.call(SOUND_HANDLE_ASSIGN, &args![handle, created]);
    e.call(SOUND_HANDLE_DESTRUCTOR, &args![temporary]);
    if voice.is_actor_speaker {
        let far = e
            .call(SETTING_FLOAT_ADDRESS, &args![ACTOR_RANGE_SETTING])
            .u32();
        let far = x87_quiet_bits(e.mem.u32(far));
        let near = e
            .call(SETTING_FLOAT_ADDRESS, &args![MIN_RANGE_SETTING])
            .u32();
        let near = x87_quiet_bits(e.mem.u32(near));
        let handle = item_of(e, node) + 4;
        e.call(SOUND_SET_MIN_MAX, &args![handle, near, far]);
    }
    let handle = item_of(e, node) + 4;
    e.call(
        SOUND_SET_COMPLETION,
        &args![handle, COMPLETION_CALLBACK, voice.station],
    );
    let handle = item_of(e, node) + 4;
    let start = e.mem.u32(voice.station + 0xc);
    e.call(SOUND_SET_SYNCH_START, &args![handle, start]);
    let handle = item_of(e, node) + 4;
    set_position(e, handle, voice.position);
    let volume = power_volume(e, voice.power);
    let handle = item_of(e, node) + 4;
    e.call(SOUND_SET_VOLUME, &args![handle, volume]);
    let handle = item_of(e, node) + 4;
    e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
}

// Translated from 00835650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag 1 of `iStationFlags` is set.
pub fn fn_00835650(e: &mut Engine, this: Ptr<StationState>) -> bool {
    station_flag(e, this, 1)
}

// Translated from 008356e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first station reference in range of `reference` (the player when
/// it is 0) whose form is `form` (cdecl), from a temporary reference list
/// (`BGSTalkingActivator::GetRadioStationsInRangeOfRef`); the walk stops
/// at an empty item or one without a form. 0 when none.
// Translation incomplete: 008356e0 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary list is not translated
pub fn fn_008356e0(e: &mut Engine, form: u32, reference: u32) -> u32 {
    e.with_stack(8, |e, list| {
        e.call(LIST_CONSTRUCTOR, &args![list]);
        let center = if reference == 0 {
            e.global::<u32>(PLAYER)
        } else {
            reference
        };
        e.call(STATIONS_IN_RANGE, &args![center, list, 0u32]);
        let mut found = 0u32;
        let mut done = false;
        let mut node = list.addr();
        while !done && node != 0 {
            let candidate = item_of(e, node);
            if candidate == 0 {
                break;
            }
            let candidate = item_of(e, node);
            if e.call(FORM_OF, &args![candidate]).u32() == 0 {
                break;
            }
            let candidate = item_of(e, node);
            if form == e.call(FORM_OF, &args![candidate]).u32() {
                found = item_of(e, node);
                done = true;
            }
            node = next_of(e, node);
        }
        e.call(LIST_DESTRUCTOR, &args![list]);
        found
    })
}

// Translated from 008357e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the word at +0x10 of `object` to 1 unless `skip` is set or the
/// object is null (cdecl).
pub fn fn_008357e0(e: &mut Engine, object: u32, skip: u8) {
    if skip == 0 && object != 0 {
        e.mem.set_u32(object + 0x10, 1);
    }
}

// Translated from 00835980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::DisableNPCRadio` (Xbox PDB name from the call graph,
/// cdecl): for an actor with a receiver, stops and releases its sound
/// handle and clears its playing flag and radio.
pub fn fallout_radio_disable_npc_radio(e: &mut Engine, actor: u32) {
    if !e.vcall(actor, 0x100, &args![]).bool() {
        return;
    }
    let receiver = fallout_radio_get_active_receiver(e, actor);
    if receiver == 0 {
        return;
    }
    stop_and_release(e, receiver + 4);
    let receiver = Ptr::<FORadioReceiver>::new(receiver);
    e.set(receiver, FORadioReceiver::bShouldBePlaying, 0);
    e.set(receiver, FORadioReceiver::pRadio, 0);
}

// Translated from 00835f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves a receiver: its radio's form id and its four flag bytes.
pub fn fn_00835f10(e: &mut Engine, this: Ptr<FORadioReceiver>, buffer: u32) {
    let radio = e.get(this, FORadioReceiver::pRadio);
    e.call(SAVE_FORM_ID, &args![buffer, radio, 0u32]);
    for offset in [0x1cu32, 0x1d, 0x1e, 0x1f] {
        e.call(SAVE_DATA, &args![buffer, this.addr() + offset, 1u32, 0u32]);
    }
}

// Translated from 00836030 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StationState` destructor: deletes the next conversation (when it
/// is not the current one) and the current one, clears both and destroys
/// the users list.
// Translation incomplete: 00836030 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_00836030(e: &mut Engine, this: Ptr<StationState>) {
    if e.get(this, StationState::pNextConvo) != e.get(this, StationState::pConvo) {
        let next = e.get(this, StationState::pNextConvo);
        delete_conversation(e, next);
        e.set(this, StationState::pNextConvo, 0);
    }
    e.set(this, StationState::pNextConvo, 0);
    if e.get(this, StationState::pConvo) != 0 {
        let current = e.get(this, StationState::pConvo);
        delete_conversation(e, current);
        e.set(this, StationState::pConvo, 0);
    }
    e.set(this, StationState::pConvo, 0);
    e.call(LIST_DESTRUCTOR, &args![this.byte_add(0x18)]);
}

// Translated from 00836110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves a station state: the time into the current item (0 without a
/// duration), the duration, both powers and the flags, the users' radio
/// form ids (a sized value with their count), whether there is a
/// conversation and then the conversation.
pub fn fn_00836110(e: &mut Engine, this: Ptr<StationState>, buffer: u32) {
    e.with_stack(8, |e, locals| {
        let elapsed = locals.addr();
        let has_conversation = locals.addr() + 4;
        let now = audio_time(e);
        let start = e.get(this, StationState::iCurrentItemStartTime);
        e.mem.set_u32(elapsed, now.wrapping_sub(start));
        if e.get(this, StationState::iCurrentItemDuration) <= 0 {
            e.mem.set_u32(elapsed, 0);
        }
        e.call(SAVE_DATA, &args![buffer, elapsed, 4u32, 0u32]);
        for (offset, size) in [(0xcu32, 4u32), (0x10, 1), (0x11, 1), (0x14, 4)] {
            e.call(SAVE_DATA, &args![buffer, this.addr() + offset, size, 0u32]);
        }
        let mut count = 0u32;
        let mark = e.call(SAVE_START_SIZED, &args![buffer]).u32();
        let mut node = this.addr() + 0x18;
        while node != 0 {
            let receiver = item_of(e, node);
            if receiver != 0 {
                e.call(SAVE_FORM_ID, &args![buffer, e.mem.u32(receiver), 0u32]);
                count = count.wrapping_add(1);
            }
            node = next_of(e, node);
        }
        e.call(SAVE_SIZED, &args![buffer, count, mark]);
        let has = u8::from(e.get(this, StationState::pConvo) != 0);
        e.mem.set_u8(has_conversation, has);
        e.call(SAVE_DATA, &args![buffer, has_conversation, 1u32, 0u32]);
        if has != 0 {
            let conversation = e.get(this, StationState::pConvo);
            e.call(CONVERSATION_SAVE, &args![conversation, buffer]);
        }
    });
}

// Translated from 00836600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves a station: its reference's form id, then its state.
pub fn fn_00836600(e: &mut Engine, this: Ptr, buffer: u32) {
    let reference = e.mem.u32(this.addr());
    e.call(SAVE_FORM_ID, &args![buffer, reference, 0u32]);
    fn_00836110(e, Ptr::new(this.addr() + STATION_STATE), buffer);
}

// Translated from 008366e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio`'s save (cdecl): the saved word, byte and string, the
/// receivers and the stations (each a sized value with its count), the
/// forms of the current and the lost Pip-Boy stations, and the reported
/// stations.
pub fn fn_008366e0(e: &mut Engine, buffer: u32) {
    e.call(SAVE_DATA, &args![buffer, SAVED_WORD, 4u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, SAVED_BYTE, 1u32, 0u32]);
    e.call(SAVE_STRING, &args![buffer, SAVED_STRING, 0u32]);
    let mut count = 0u32;
    let mark = e.call(SAVE_START_SIZED, &args![buffer]).u32();
    let mut node = RECEIVER_LIST;
    while node != 0 {
        let receiver = item_of(e, node);
        if receiver != 0 {
            fn_00835f10(e, Ptr::new(receiver), buffer);
            count = count.wrapping_add(1);
        }
        node = next_of(e, node);
    }
    e.call(SAVE_SIZED, &args![buffer, count, mark]);
    let mut count = 0u32;
    let mark = e.call(SAVE_START_SIZED, &args![buffer]).u32();
    let mut node = STATION_LIST;
    while node != 0 {
        let station = item_of(e, node);
        if station != 0 {
            fn_00836600(e, Ptr::new(station), buffer);
            count = count.wrapping_add(1);
        }
        node = next_of(e, node);
    }
    e.call(SAVE_SIZED, &args![buffer, count, mark]);
    for global in [CURRENT_PIPBOY_STATION, LOST_PIPBOY_STATION] {
        let station = e.global::<u32>(global);
        let form = if station != 0 {
            e.call(FORM_OF, &args![e.mem.u32(station)]).u32()
        } else {
            0
        };
        e.call(SAVE_FORM_ID, &args![buffer, form, 0u32]);
    }
    let mut count = 0u32;
    let mark = e.call(SAVE_START_SIZED, &args![buffer]).u32();
    let mut node = DISCOVERED_LIST;
    while node != 0 {
        let reference = item_of(e, node);
        if reference != 0 {
            e.call(SAVE_FORM_ID, &args![buffer, reference, 0u32]);
            count = count.wrapping_add(1);
        }
        node = next_of(e, node);
    }
    e.call(SAVE_SIZED, &args![buffer, count, mark]);
}

// Translated from 00837060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::GetNextConversation` (name used by the decompiler at its
/// call site, cdecl): with a next conversation queued, deletes the current
/// one, moves the next one in, deletes it through its deleting destructor
/// as the code does, clears the next slot and queues a `RadioConvTask`
/// (0x38 bytes, `00837250`) on the task queue (`01202d98`, slot `+0x48`),
/// answering true; otherwise deletes the current conversation and creates
/// a new one (0xc bytes, `0083b850`) on the station's reference with
/// `topic` (topic `7` from `TESTopic::GetTopic` when it is 0), answering
/// false.
// Translation incomplete: 00837060 - the C++ exception-unwinding frame (FS:[0]) that frees the new objects is not translated
pub fn fn_00837060(e: &mut Engine, station: u32, topic: u32) -> bool {
    let topic = if topic == 0 {
        e.call(GET_TOPIC, &args![7u32, 0u32]).u32()
    } else {
        topic
    };
    // The station's words: +4 the current conversation (`pConvo` of its
    // state), +8 the next one.
    if e.mem.u32(station + 8) != 0 {
        if e.mem.u32(station + 4) != 0 {
            let old = e.mem.u32(station + 4);
            delete_conversation(e, old);
            e.mem.set_u32(station + 4, 0);
        }
        let next = e.mem.u32(station + 8);
        e.mem.set_u32(station + 4, next);
        let next = e.mem.u32(station + 8);
        delete_conversation(e, next);
        e.mem.set_u32(station + 8, 0);
        let block = e.call(OPERATOR_NEW, &args![0x38u32]).u32();
        let task = if block != 0 {
            e.call(RADIO_TASK_CONSTRUCT, &args![block, station]).u32()
        } else {
            0
        };
        let queue = e.global::<u32>(TASK_QUEUE);
        e.vcall(queue, 0x48, &args![task]);
        return true;
    }
    if e.mem.u32(station + 4) != 0 {
        let old = e.mem.u32(station + 4);
        delete_conversation(e, old);
        e.mem.set_u32(station + 4, 0);
    }
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let conversation = if block != 0 {
        let reference = e.mem.u32(station);
        e.call(
            CONVERSATION_CONSTRUCT,
            &args![block, reference, reference, topic],
        )
        .u32()
    } else {
        0
    };
    e.mem.set_u32(station + 4, conversation);
    false
}

// Translated from 008373d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of a list of floats (first node embedded): item 0.0,
/// no next node. Returns `this`.
pub fn fn_008373d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

/// The byte `008325a0` stores (`011dd313`; `falloutaudiomedia.rs` reads it as the media
/// hold byte) and the byte `00833ce0` answers (`011dd437`).
const HOLD_FLAG_011DD313: u32 = 0x011d_d313;
const STORED_FLAG_011DD437: u32 = 0x011d_d437;

// Translated from 008325a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): stores its low byte at `011dd313`.
pub fn fn_008325a0(e: &mut Engine, value: u8) {
    e.set_global(HOLD_FLAG_011DD313, value);
}

// Translated from 00832490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag `0x100` of `iStationFlags` (`thiscall`, `RET 4`).
pub fn fn_00832490(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 0x100, on);
}

// Translated from 00833c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag `0x80` of `iStationFlags` is set (`EAX` exactly 0 or 1).
pub fn fn_00833c00(e: &mut Engine, this: Ptr<StationState>) -> bool {
    station_flag(e, this, 0x80)
}

// Translated from 00833ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the byte at `011dd437`, in `AL`.
pub fn fn_00833ce0(e: &mut Engine) -> u8 {
    e.global(STORED_FLAG_011DD437)
}

// Translated from 00835670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag `8` of `iStationFlags` is set (`EAX` exactly 0 or 1).
pub fn fn_00835670(e: &mut Engine, this: Ptr<StationState>) -> bool {
    station_flag(e, this, 8)
}

// Translated from 00835690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag `0x80` of `iStationFlags` (`thiscall`, `RET 4`).
pub fn fn_00835690(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 0x80, on);
}

// Translated from 00833bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag `4` of `iStationFlags` is set (`EAX` exactly 0 or 1).
pub fn fn_00833bc0(e: &mut Engine, this: Ptr<StationState>) -> bool {
    station_flag(e, this, 4)
}

// Translated from 00833be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag `0x10` of `iStationFlags` is set (`EAX` exactly 0 or 1).
pub fn fn_00833be0(e: &mut Engine, this: Ptr<StationState>) -> bool {
    station_flag(e, this, 0x10)
}

// Translated from 00833c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag `0x40` of `iStationFlags` (`thiscall`, `RET 4`).
pub fn fn_00833c90(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 0x40, on);
}

/// The byte [`fn_00833cf0`] answers (`011dcfa4`; the dialogue menu sets and clears it).
const DIALOGUE_FLAG_011DCFA4: u32 = 0x011d_cfa4;

// Translated from 00833cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the byte at `011dcfa4`, in `AL`.
pub fn fn_00833cf0(e: &mut Engine) -> u8 {
    e.global(DIALOGUE_FLAG_011DCFA4)
}

/// The task base constructor `RadioConvTask`'s constructor runs (`00c3ce60`, unit
/// `queuedfiles.obj`, unnamed; `thiscall`, one word) and `RadioConvTask`'s vtable
/// (`0107cf18`, RTTI `.?AVRadioConvTask@@`).
const TASK_BASE_CONSTRUCT: u32 = 0x00c3_ce60;
const RADIO_CONV_TASK_VTABLE: u32 = 0x0107_cf18;

// Translated from 00837250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RadioConvTask`'s constructor (the engine map gives no name; the vtable's RTTI names the
/// class; `thiscall`, `RET 4`): the base (`00c3ce60` with 0), the vtable, the station at
/// +0x30. Returns `this`.
pub fn fn_00837250(e: &mut Engine, this: Ptr, station: u32) -> Ptr {
    e.call(TASK_BASE_CONSTRUCT, &args![this, 0u32]);
    e.mem.set_u32(this.addr(), RADIO_CONV_TASK_VTABLE);
    e.mem.set_u32(this.addr() + 0x30, station);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0083_25a0, fn_008325a0(u8)),
        entry!(0x0083_2490, fn_00832490(Ptr<StationState>, u8)),
        entry!(0x0083_3c00, fn_00833c00(Ptr<StationState>) -> bool),
        entry!(0x0083_3ce0, fn_00833ce0() -> u8),
        entry!(0x0083_3bc0, fn_00833bc0(Ptr<StationState>) -> bool),
        entry!(0x0083_3be0, fn_00833be0(Ptr<StationState>) -> bool),
        entry!(0x0083_3c90, fn_00833c90(Ptr<StationState>, u8)),
        entry!(0x0083_3cf0, fn_00833cf0() -> u8),
        entry!(0x0083_7250, fn_00837250(Ptr, u32) -> Ptr),
        entry!(0x0083_5670, fn_00835670(Ptr<StationState>) -> bool),
        entry!(0x0083_5690, fn_00835690(Ptr<StationState>, u8)),
        entry!(0x0083_72c0, fn_008372c0(Ptr)),
        entry!(0x0083_21c0, fn_008321c0(Ptr)),
        entry!(0x0083_2010, fallout_radio_shutdown()),
        entry!(0x0083_2160, fn_00832160(Ptr, u32) -> Ptr),
        entry!(
            0x0083_2190,
            fn_00832190(Ptr<FORadioReceiver>, u32) -> Ptr<FORadioReceiver>
        ),
        entry!(0x0083_21e0, fn_008321e0(Ptr<FORadioReceiver>)),
        entry!(0x0083_2420, fn_00832420(Ptr<StationState>) -> bool),
        entry!(0x0083_2440, fn_00832440(Ptr<StationState>, u8)),
        entry!(0x0083_25b0, fn_008325b0(u32, u8)),
        entry!(
            0x0083_27d0,
            fn_008327d0(Ptr<FORadioReceiver>) -> Ptr<FORadioReceiver>
        ),
        entry!(0x0083_2830, fn_00832830(u32) -> u32),
        entry!(0x0083_2930, fallout_radio_get_active_receiver(u32) -> u32),
        entry!(0x0083_29a0, fallout_radio_clear_inactive_receivers(u8)),
        entry!(0x0083_2ad0, fallout_radio_update(u8) -> bool),
        entry!(0x0083_2cb0, fn_00832cb0(u32, u32) -> u32),
        entry!(0x0083_3040, fn_00833040(Ptr<StationState>) -> bool),
        entry!(0x0083_3060, fn_00833060(Ptr<StationState>, u8)),
        entry!(0x0083_30b0, fn_008330b0(Ptr<StationState>, u8)),
        entry!(0x0083_3100, fn_00833100(Ptr<StationState>, u8)),
        entry!(0x0083_3150, fn_00833150(Ptr) -> Ptr),
        entry!(
            0x0083_3170,
            fn_00833170(Ptr<StationState>) -> Ptr<StationState>
        ),
        entry!(0x0083_3c20, fn_00833c20(Ptr<StationState>) -> u32),
        entry!(0x0083_3c40, fn_00833c40(Ptr<StationState>, u8)),
        entry!(0x0083_3d00, fallout_radio_update_stations(u8)),
        entry!(0x0083_4210, fn_00834210(Ptr<StationState>, u8)),
        entry!(0x0083_4260, fn_00834260(u32)),
        entry!(0x0083_5650, fn_00835650(Ptr<StationState>) -> bool),
        entry!(0x0083_56e0, fn_008356e0(u32, u32) -> u32),
        entry!(0x0083_57e0, fn_008357e0(u32, u8)),
        entry!(0x0083_5980, fallout_radio_disable_npc_radio(u32)),
        entry!(0x0083_5f10, fn_00835f10(Ptr<FORadioReceiver>, u32)),
        entry!(0x0083_6030, fn_00836030(Ptr<StationState>)),
        entry!(0x0083_6110, fn_00836110(Ptr<StationState>, u32)),
        entry!(0x0083_6600, fn_00836600(Ptr, u32)),
        entry!(0x0083_66e0, fn_008366e0(u32)),
        entry!(0x0083_7060, fn_00837060(u32, u32) -> bool),
        entry!(0x0083_73d0, fn_008373d0(Ptr) -> Ptr),
    ]
    .into_iter()
    .chain(funcs_n086())
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_008321c0_passes_this_plus_4_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00836030, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0083_21c0, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00836030).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x4]);
    }

    #[test]
    fn fn_008372c0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x00c3_cea0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0083_72c0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00c3_cea0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0107_cf18);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0107_cf18,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod b070_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn answer(e: &mut Engine, address: u32, eax: u32) {
        e.register_double(address, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    /// The list node helpers as the exe has them: the item's address is
    /// the node, the next node is at +4, a node is empty when both words
    /// are 0.
    fn lists(e: &mut Engine) {
        e.register(NODE_ITEM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(NODE_NEXT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(NODE_EMPTY, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0),
            ..Ret::default()
        });
    }

    /// Maps the radio's statics.
    fn globals(e: &mut Engine) {
        for page in [
            0x011d_d000u32,
            0x011a_1000,
            0x011d_e000,
            0x0101_6000,
            0x011c_3000,
            0x0104_f000,
        ] {
            e.map(page, 0x1000);
        }
    }

    /// Writes a list into the embedded first node at `head` (more nodes
    /// allocated).
    fn fill(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, &item) in items.iter().enumerate() {
            e.mem.set_u32(node, item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            } else {
                e.mem.set_u32(node + 4, 0);
            }
        }
    }

    /// An object whose vtable slots are `base + slot`.
    fn object_with_vtable(e: &mut Engine, size: u32, base: u32) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = e.mem.alloc(0x400);
        e.mem.set_u32(object, vtable);
        for slot in (0..0x400).step_by(4) {
            e.mem.set_u32(vtable + slot, base + slot);
        }
        object
    }

    #[test]
    fn station_flags() {
        let mut e = Engine::new();
        let state = e.new_object::<StationState>();
        for (setter, mask) in [
            (0x0083_2440u32, 1u32),
            (0x0083_3060, 2),
            (0x0083_4210, 4),
            (0x0083_3c40, 8),
            (0x0083_30b0, 0x10),
            (0x0083_3100, 0x20),
        ] {
            e.call(setter, &args![state, 1u32]);
            assert_eq!(e.get(state, StationState::iStationFlags) & mask, mask);
        }
        assert!(e.call(0x0083_5650, &args![state]).bool());
        assert!(e.call(0x0083_3040, &args![state]).bool());
        assert!(!e.call(0x0083_2420, &args![state]).bool());
        e.set(state, StationState::iStationFlags, 0x13f);
        assert!(e.call(0x0083_2420, &args![state]).bool());
        e.call(0x0083_3100, &args![state, 0u32]);
        e.call(0x0083_2440, &args![state, 0u32]);
        assert_eq!(e.get(state, StationState::iStationFlags), 0x11e);
        assert!(!e.call(0x0083_5650, &args![state]).bool());
    }

    #[test]
    fn station_power_byte_keeps_the_upper_bytes_of_this() {
        let mut e = Engine::new();
        let state = e.new_object::<StationState>();
        e.set(state, StationState::cStationPower, 0x42);
        let result = e.call(0x0083_3c20, &args![state]).u32();
        assert_eq!(result, (state.addr() & !0xff) | 0x42);
    }

    #[test]
    fn station_and_state_constructors() {
        let mut e = Engine::new();
        stub(&mut e, &[LIST_CONSTRUCTOR]);
        let station = e.mem.alloc(0x24);
        e.mem.write(station, &[0xcd; 0x24]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_3150, &args![station]).u32(), station);
        let state = Ptr::<StationState>::new(station + 4);
        assert_eq!(e.get(state, StationState::pConvo), 0);
        assert_eq!(e.get(state, StationState::iStationFlags), 0);
        assert_eq!(
            e.get(state, StationState::cStationPower),
            0xcd,
            "left as is"
        );
        assert_eq!(calls_to(&e, LIST_CONSTRUCTOR), vec![vec![station + 0x1c]]);
        let list = e.mem.alloc(8);
        e.mem.write(list, &[0xcd; 8]);
        assert_eq!(e.call(0x0083_73d0, &args![list]).u32(), list);
        assert_eq!(e.mem.u64(list), 0);
    }

    #[test]
    fn receiver_construction_and_deletion() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                SOUND_HANDLE_CONSTRUCT,
                SOUND_HANDLE_DESTRUCTOR,
                OPERATOR_DELETE,
                0x0083_6030,
            ],
        );
        let receiver = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_27d0, &args![receiver]).u32(), receiver);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_CONSTRUCT),
            vec![vec![receiver + 4], vec![receiver + 0x10]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0083_2190, &args![receiver, 1u32]);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_DESTRUCTOR),
            vec![vec![receiver + 0x10], vec![receiver + 4]]
        );
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![receiver]]);
        e.call_log = Some(vec![]);
        e.call(0x0083_2160, &args![receiver, 0u32]);
        assert_eq!(calls_to(&e, 0x0083_6030), vec![vec![receiver + 4]]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn active_receiver_lookup() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(first, 0x5001);
        e.mem.set_u32(second, 0x5002);
        fill(&mut e, RECEIVER_LIST, &[first, second]);
        assert_eq!(e.call(0x0083_2930, &args![0x5002u32]).u32(), second);
        assert_eq!(e.call(0x0083_2930, &args![0x5003u32]).u32(), 0);
    }

    #[test]
    fn station_lookup_prefers_the_enabled_one() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        let first = e.mem.alloc(0x24);
        let second = e.mem.alloc(0x24);
        e.mem.set_u32(first, 0x5001);
        e.mem.set_u32(second, 0x5002);
        fill(&mut e, STATION_LIST, &[first, second]);
        answer(&mut e, FORM_OF, 0x6000);
        e.register(REFERENCE_TEST_00440DA0, |_, a| Ret {
            eax: u32::from(a[0] == 0x5001),
            ..Ret::default()
        });
        answer(&mut e, REFERENCE_VALUE_0056B210, 0);
        assert_eq!(e.call(0x0083_2830, &args![0x6000u32]).u32(), second);
        assert_eq!(e.call(0x0083_2830, &args![0x6001u32]).u32(), 0);
        e.set_global(SHUTTING_DOWN, 1u8);
        assert_eq!(e.call(0x0083_2830, &args![0x6000u32]).u32(), 0);
    }

    #[test]
    fn clearing_removes_silent_receivers_from_the_stations() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        let silent = e.mem.alloc(0x20);
        let playing = e.mem.alloc(0x20);
        e.mem.set_u8(playing + 0x1c, 1);
        fill(&mut e, RECEIVER_LIST, &[playing, silent]);
        let station = e.mem.alloc(0x24);
        fill(&mut e, STATION_LIST, &[station]);
        // The receiver list's removal drops the item it is given.
        e.register(LIST_REMOVE, |e, a| {
            if a[0] == RECEIVER_LIST {
                let node = e.mem.u32(a[0] + 4);
                e.mem.set_u32(node, 0);
                e.mem.set_u32(a[0] + 4, 0);
            }
            Ret::default()
        });
        let mut contained = 1;
        e.register_double(LIST_CONTAINS, move |_, _| {
            let answer = contained;
            contained = 0;
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        stub(
            &mut e,
            &[
                SOUND_HANDLE_STOP,
                SOUND_HANDLE_RELEASE,
                SOUND_HANDLE_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0083_29a0, &args![0u32]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_STOP), vec![vec![silent + 4]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![silent]]);
        assert_eq!(calls_to(&e, LIST_REMOVE).len(), 2);
    }

    #[test]
    fn shutdown_deletes_stations_and_receivers() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station + 4, 0x7000);
        fill(&mut e, STATION_LIST, &[station]);
        stub(
            &mut e,
            &[
                CONVERSATION_DELETE,
                0x0083_6030,
                OPERATOR_DELETE,
                LIST_CLEAR,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0083_2010, &args![]);
        assert_eq!(e.global::<u8>(SHUTTING_DOWN), 1);
        assert_eq!(calls_to(&e, CONVERSATION_DELETE), vec![vec![0x7000, 1]]);
        assert_eq!(e.mem.u32(station + 4), 0);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![station]]);
        assert_eq!(
            calls_to(&e, LIST_CLEAR),
            vec![
                vec![STATION_LIST],
                vec![RECEIVER_LIST],
                vec![DISCOVERED_LIST]
            ]
        );
    }

    #[test]
    fn turning_a_radio_on_creates_its_receiver() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x5000);
        fill(&mut e, STATION_LIST, &[station]);
        answer(&mut e, FORM_OF, 0x6000);
        answer(&mut e, FORM_TYPE, 0x15);
        answer(&mut e, RADIO_STATION_FORM, 0x6000);
        answer(&mut e, REFERENCE_VALUE_0056B210, 0);
        let block = e.mem.alloc(0x20);
        answer(&mut e, OPERATOR_NEW, block);
        stub(&mut e, &[SOUND_HANDLE_CONSTRUCT, LIST_ADD_HEAD]);
        e.call_log = Some(vec![]);
        e.call(0x0083_25b0, &args![0x5100u32, 1u32]);
        let receiver = Ptr::<FORadioReceiver>::new(block);
        assert_eq!(e.get(receiver, FORadioReceiver::pRadio), 0x5100);
        assert_eq!(e.get(receiver, FORadioReceiver::bShouldBePlaying), 1);
        assert_eq!(e.get(receiver, FORadioReceiver::bJustStarted), 1);
        let adds = calls_to(&e, LIST_ADD_HEAD);
        assert_eq!(adds[0][0], RECEIVER_LIST);
        assert_eq!(adds[1][0], station + 0x1c);
        let state = Ptr::<StationState>::new(station + 4);
        assert_eq!(e.get(state, StationState::iStationFlags), 1);
    }

    #[test]
    fn turning_a_radio_off_stops_it() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        // Removing from the receiver list empties it.
        e.register(LIST_REMOVE, |e, a| {
            if a[0] == RECEIVER_LIST {
                e.mem.set_u32(RECEIVER_LIST, 0);
                e.mem.set_u32(RECEIVER_LIST + 4, 0);
            }
            Ret::default()
        });
        let receiver = e.mem.alloc(0x20);
        e.mem.set_u32(receiver, 0x5100);
        e.mem.set_u8(receiver + 0x1c, 1);
        fill(&mut e, RECEIVER_LIST, &[receiver]);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x5000);
        fill(&mut e, STATION_LIST, &[station]);
        answer(&mut e, FORM_OF, 0x6000);
        answer(&mut e, FORM_TYPE, 0x15);
        answer(&mut e, RADIO_STATION_FORM, 0x6000);
        answer(&mut e, REFERENCE_VALUE_0056B210, 0);
        stub(
            &mut e,
            &[
                SOUND_HANDLE_STOP,
                SOUND_HANDLE_RELEASE,
                LIST_CONTAINS,
                SOUND_HANDLE_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0083_25b0, &args![0x5100u32, 0u32]);
        assert_eq!(e.mem.u8(receiver + 0x1c), 0);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_STOP)[..2],
            [vec![receiver + 4], vec![receiver + 0x10]]
        );
        e.set_global(SHUTTING_DOWN, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x0083_25b0, &args![0x5100u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn creating_a_station_for_a_talker() {
        let mut e = Engine::new();
        globals(&mut e);
        let block = e.mem.alloc(0x24);
        answer(&mut e, OPERATOR_NEW, block);
        answer(&mut e, FORM_TYPE, 0x16);
        let form = object_with_vtable(&mut e, 0x10, 0x00c1_0000);
        answer(&mut e, FORM_OF, form);
        answer(&mut e, 0x00c1_00b4, 1);
        answer(&mut e, AUDIO_INSTANCE, 0x900);
        answer(&mut e, AUDIO_TIME, 50_000);
        answer(&mut e, RANDOM_RANGE, 1000);
        stub(&mut e, &[LIST_CONSTRUCTOR, LIST_ADD_HEAD]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_2cb0, &args![0x5000u32, 0u32]).u32(), block);
        let state = Ptr::<StationState>::new(block + 4);
        assert_eq!(e.mem.u32(block), 0x5000);
        assert_eq!(e.get(state, StationState::iCurrentItemStartTime), 49_000);
        assert_eq!(e.get(state, StationState::iCurrentItemDuration), 1);
        assert_eq!(e.get(state, StationState::iStationFlags), 0x32);
        assert_eq!(e.get(state, StationState::cStationPower), 100);
        assert_eq!(calls_to(&e, RANDOM_RANGE), vec![vec![0, 0x7530]]);
        assert_eq!(calls_to(&e, LIST_ADD_HEAD)[0][0], STATION_LIST);
        assert_eq!(e.call(0x0083_2cb0, &args![0u32, 0u32]).u32(), 0);
    }

    #[test]
    fn creating_a_station_without_a_kind_deletes_the_entry() {
        let mut e = Engine::new();
        globals(&mut e);
        let block = e.mem.alloc(0x24);
        answer(&mut e, OPERATOR_NEW, block);
        answer(&mut e, FORM_OF, 0x6000);
        answer(&mut e, FORM_TYPE, 0x10);
        stub(&mut e, &[LIST_CONSTRUCTOR, 0x0083_6030, OPERATOR_DELETE]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_2cb0, &args![0x5000u32, 0u32]).u32(), 0);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![block]]);
    }

    #[test]
    fn update_applies_the_queued_state_and_updates_the_stations() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        e.set_global(QUEUED_STATE, 2u32);
        e.set_global(RADIO_STATE, 1u32);
        e.set_global(LAST_UPDATE, 100u32);
        answer(&mut e, TIMER_TIME, 400);
        let interval = e.mem.alloc(4);
        e.mem.set_u32(interval, 200);
        answer(&mut e, SETTING_VALUE_ADDRESS, interval);
        answer(&mut e, LOADING_MENU_VISIBLE, 0);
        answer(&mut e, GET_SOUND, 0x7700);
        stub(
            &mut e,
            &[LIST_CONSTRUCTOR, FLOAT_LIST_DESTRUCTOR, LIST_DESTRUCTOR],
        );
        e.set_global(SHUTTING_DOWN, 1u8);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0083_2ad0, &args![0u32]).bool());
        assert_eq!(e.global::<u32>(LAST_UPDATE), 400);
        assert_eq!(e.global::<u32>(QUEUED_STATE), 0);
        assert_eq!(e.global::<u32>(RADIO_STATE), 0, "cleared at the end");
        assert_eq!(e.global::<u32>(ATTENUATION_SOUND), 0x7700);
        assert_eq!(calls_to(&e, LIST_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn update_stations_computes_the_power_of_a_station_in_range() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_7000, 0x1000);
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x5000);
        // Not a station with users: 00834260 returns at once.
        fill(&mut e, STATION_LIST, &[station]);
        let signal = e.mem.alloc(4);
        e.mem.set_f32(signal, 0.5);
        answer(&mut e, SETTING_FLOAT_ADDRESS, signal);
        e.register(STATIONS_IN_RANGE, |e, a| {
            e.mem.set_u32(a[1], 0x5000);
            e.mem.set_f32(a[2], 75.0);
            Ret::default()
        });
        answer(&mut e, REFERENCE_TEST_00440DA0, 0);
        answer(&mut e, LIST_CONTAINS, 1);
        e.register(STATION_VALUE_0056B190, |_, _| Ret {
            st0: 100.0,
            ..Ret::default()
        });
        e.register(STATION_VALUE_0056B1D0, |_, _| Ret::default());
        answer(&mut e, REFERENCE_VALUE_0056B210, 0);
        stub(
            &mut e,
            &[
                LIST_CONSTRUCTOR,
                STATE_SET_POWER,
                FLOAT_LIST_CLEAR,
                LIST_CLEAR,
                FLOAT_LIST_DESTRUCTOR,
                LIST_DESTRUCTOR,
                AUDIO_INSTANCE,
                AUDIO_LISTENER_POSITION,
                ENTER_CRITICAL_SECTION,
                LEAVE_CRITICAL_SECTION,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0083_3d00, &args![1u32]);
        // Ratio 0.75 over the falloff 0.5: (0.25 / 0.5) * 100 = 50 lost.
        assert_eq!(calls_to(&e, STATE_SET_POWER), vec![vec![station + 4, 50]]);
        assert_eq!(e.global::<u8>(SKIP_DISCOVERY_MESSAGE), 0);
        assert_eq!(calls_to(&e, LEAVE_CRITICAL_SECTION).len(), 1);
    }

    #[test]
    fn update_stations_cuts_a_station_out_of_range() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x5000);
        fill(&mut e, STATION_LIST, &[station]);
        let signal = e.mem.alloc(4);
        answer(&mut e, SETTING_FLOAT_ADDRESS, signal);
        stub(
            &mut e,
            &[
                LIST_CONSTRUCTOR,
                STATIONS_IN_RANGE,
                STATE_SET_POWER,
                FLOAT_LIST_CLEAR,
                LIST_CLEAR,
                FLOAT_LIST_DESTRUCTOR,
                LIST_DESTRUCTOR,
                AUDIO_INSTANCE,
                AUDIO_LISTENER_POSITION,
                ENTER_CRITICAL_SECTION,
                LEAVE_CRITICAL_SECTION,
                REFERENCE_TEST_00440DA0,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0083_3d00, &args![1u32]);
        assert_eq!(calls_to(&e, STATE_SET_POWER), vec![vec![station + 4, 0]]);
        let state = Ptr::<StationState>::new(station + 4);
        assert_eq!(e.get(state, StationState::iStationFlags), 4);
    }

    #[test]
    fn update_station_pauses_its_users() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        e.map(0x0104_f000, 0x1000);
        e.set_global(RADIO_STATE, 1u32);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x5000);
        let receiver = e.mem.alloc(0x20);
        fill(&mut e, station + 0x1c, &[receiver]);
        answer(&mut e, SOUND_IS_VALID, 1);
        stub(
            &mut e,
            &[
                REFERENCE_TEST_00440DA0,
                AUDIO_INSTANCE,
                AUDIO_LISTENER_POSITION,
                ENTER_CRITICAL_SECTION,
                LEAVE_CRITICAL_SECTION,
                FORM_OF,
                RTTI_CAST,
                SOUND_PAUSE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0083_4260, &args![station]);
        assert_eq!(
            calls_to(&e, SOUND_PAUSE),
            vec![vec![receiver + 4], vec![receiver + 0x10]]
        );
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION),
            vec![vec![RADIO_CRITICAL_SECTION, UPDATE_STATION_NAME]]
        );
        assert_eq!(calls_to(&e, LEAVE_CRITICAL_SECTION).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x0083_4260, &args![0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn update_station_starts_the_voice_file_of_a_user_in_range() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        for page in [0x0104_f000u32, 0x011f_4000, 0x0101_7000, 0x0106_e000] {
            e.map(page, 0x1000);
        }
        e.set_global(DEFAULT_RANGE, 1000.0f32.to_bits());
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x5000);
        e.mem.set_u32(station + 4, 0x7000);
        let state = Ptr::<StationState>::new(station + 4);
        e.set(state, StationState::iCurrentItemStartTime, 1000);
        e.set(state, StationState::iCurrentItemDuration, 10);
        e.set(state, StationState::cStationPower, 100);
        e.set(state, StationState::iStationFlags, 1);
        let radio = object_with_vtable(&mut e, 0x10, 0x00c1_0000);
        let at = e.mem.alloc(12);
        e.mem.set_f32(at, 3.0);
        answer(&mut e, 0x00c1_01f4, at);
        answer(&mut e, 0x00c1_0100, 0);
        let receiver = e.mem.alloc(0x20);
        e.mem.set_u32(receiver, radio);
        e.mem.set_u8(receiver + 0x1c, 1);
        e.mem.set_u8(receiver + 0x1d, 1);
        fill(&mut e, station + 0x1c, &[receiver]);
        answer(&mut e, AUDIO_TIME, 1005);
        // The conversation's current response is 0x7100, whose own current
        // item 0x7200 has a voice form.
        e.register(CURRENT_RESPONSE, |_, a| Ret {
            eax: if a[0] == 0x7000 { 0x7100 } else { 0x7200 },
            ..Ret::default()
        });
        answer(&mut e, FORM_OF, 0x7300);
        answer(&mut e, SOUND_FILE_NAME, 0x7400);
        answer(&mut e, STRING_FIND, 1);
        answer(&mut e, STRING_C_STR, 0);
        answer(&mut e, VECTOR_SUBTRACT, 0x7500);
        e.register(VECTOR_LENGTH, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        answer(&mut e, AUDIO_SOUND_BY_FILE, 0x7600);
        answer(&mut e, STATE_VALUE_00824060, 0);
        stub(
            &mut e,
            &[
                REFERENCE_TEST_00440DA0,
                AUDIO_INSTANCE,
                AUDIO_LISTENER_POSITION,
                ENTER_CRITICAL_SECTION,
                LEAVE_CRITICAL_SECTION,
                RTTI_CAST,
                SOUND_IS_VALID,
                STATE_CALL_00632D20,
                REFERENCE_WORLD_SPACE,
                PLAYER_OBJECT,
                PLAYER_OBJECT_TEST,
                COPY_BOUNDED,
                SOUND_HANDLE_ASSIGN,
                SOUND_HANDLE_DESTRUCTOR,
                SOUND_SET_COMPLETION,
                SOUND_SET_SYNCH_START,
                SOUND_SET_POSITION,
                SOUND_SET_VOLUME,
                SOUND_HANDLE_PLAY,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0083_4260, &args![station]);
        let by_file = calls_to(&e, AUDIO_SOUND_BY_FILE);
        assert_eq!(by_file.len(), 1);
        assert_eq!(by_file[0][3], 0x0050_0102);
        assert_eq!(by_file[0][4], 0, "no attenuation sound");
        assert_eq!(
            calls_to(&e, SOUND_SET_COMPLETION),
            vec![vec![receiver + 4, COMPLETION_CALLBACK, station]]
        );
        assert_eq!(
            calls_to(&e, SOUND_SET_SYNCH_START),
            vec![vec![receiver + 4, 1000]]
        );
        assert_eq!(
            calls_to(&e, SOUND_SET_VOLUME),
            vec![vec![receiver + 4, 1.0f32.to_bits()]]
        );
        let positions = calls_to(&e, SOUND_SET_POSITION);
        assert_eq!(positions[0][1], 3.0f32.to_bits());
        assert_eq!(calls_to(&e, SOUND_HANDLE_PLAY), vec![vec![receiver + 4, 0]]);
        assert_eq!(e.mem.u8(receiver + 0x1d), 0, "no longer just started");
        assert_eq!(e.get(state, StationState::iStationFlags), 9);
    }

    #[test]
    fn stations_in_range_lookup() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        e.set_global(PLAYER, 0x3000u32);
        e.register(STATIONS_IN_RANGE, |e, a| {
            e.mem.set_u32(a[1], 0x5001);
            let next = e.mem.alloc(8);
            e.mem.set_u32(a[1] + 4, next);
            e.mem.set_u32(next, 0x5002);
            Ret::default()
        });
        e.register(FORM_OF, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        stub(&mut e, &[LIST_CONSTRUCTOR, LIST_DESTRUCTOR]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_56e0, &args![0x6002u32, 0u32]).u32(), 0x5002);
        assert_eq!(calls_to(&e, STATIONS_IN_RANGE)[0][0], 0x3000);
        assert_eq!(e.call(0x0083_56e0, &args![0x6009u32, 0x4000u32]).u32(), 0);
    }

    #[test]
    fn completion_callback_marks_the_item_over() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x14);
        e.call(0x0083_57e0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object + 0x10), 0);
        e.call(0x0083_57e0, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object + 0x10), 1);
        e.call(0x0083_57e0, &args![0u32, 0u32]);
    }

    #[test]
    fn disabling_an_npc_radio() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        let actor = object_with_vtable(&mut e, 0x10, 0x00c1_0000);
        answer(&mut e, 0x00c1_0100, 1);
        let receiver = e.mem.alloc(0x20);
        e.mem.set_u32(receiver, actor);
        e.mem.set_u8(receiver + 0x1c, 1);
        fill(&mut e, RECEIVER_LIST, &[receiver]);
        stub(&mut e, &[SOUND_HANDLE_STOP, SOUND_HANDLE_RELEASE]);
        e.call(0x0083_5980, &args![actor]);
        assert_eq!(e.mem.u32(receiver), 0);
        assert_eq!(e.mem.u8(receiver + 0x1c), 0);
    }

    #[test]
    fn saving_receivers_states_and_the_radio() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        stub(
            &mut e,
            &[
                SAVE_FORM_ID,
                SAVE_DATA,
                SAVE_STRING,
                SAVE_SIZED,
                CONVERSATION_SAVE,
            ],
        );
        answer(&mut e, SAVE_START_SIZED, 0x99);
        answer(&mut e, AUDIO_INSTANCE, 0x900);
        answer(&mut e, AUDIO_TIME, 1500);
        let receiver = e.mem.alloc(0x20);
        e.mem.set_u32(receiver, 0x5100);
        fill(&mut e, RECEIVER_LIST, &[receiver]);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x5000);
        e.mem.set_u32(station + 4, 0x7000);
        e.mem.set_u32(station + 0xc, 1000);
        e.mem.set_i32(station + 0x10, 5);
        fill(&mut e, station + 0x1c, &[receiver]);
        fill(&mut e, STATION_LIST, &[station]);
        fill(&mut e, DISCOVERED_LIST, &[0x5200]);
        e.set_global(CURRENT_PIPBOY_STATION, station);
        answer(&mut e, FORM_OF, 0x6000);
        e.register(SAVE_DATA, |e, a| {
            if a[2] == 4 && a[1] != SAVED_WORD && e.mem.u32(a[1]) == 500 {
                e.mem.set_u32(a[1], 0xe1a5);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0083_66e0, &args![0x8000u32]);
        let forms = calls_to(&e, SAVE_FORM_ID);
        assert_eq!(
            forms,
            vec![
                vec![0x8000, 0x5100, 0],
                vec![0x8000, 0x5000, 0],
                vec![0x8000, 0x5100, 0],
                vec![0x8000, 0x6000, 0],
                vec![0x8000, 0, 0],
                vec![0x8000, 0x5200, 0]
            ]
        );
        assert_eq!(
            calls_to(&e, SAVE_SIZED),
            vec![
                vec![0x8000, 1, 0x99],
                vec![0x8000, 1, 0x99],
                vec![0x8000, 1, 0x99],
                vec![0x8000, 1, 0x99]
            ]
        );
        assert_eq!(calls_to(&e, CONVERSATION_SAVE), vec![vec![0x7000, 0x8000]]);
        let data = calls_to(&e, SAVE_DATA);
        assert_eq!(data[0], vec![0x8000, SAVED_WORD, 4, 0]);
        // The receiver's four flag bytes, then the elapsed time 500.
        assert_eq!(data[2][1], receiver + 0x1c);
        assert_eq!(data[6][2], 4, "the elapsed time");
    }

    #[test]
    fn state_destructor_deletes_both_conversations() {
        let mut e = Engine::new();
        stub(&mut e, &[CONVERSATION_DELETE, LIST_DESTRUCTOR]);
        let state = e.new_object::<StationState>();
        e.set(state, StationState::pConvo, 0x7000);
        e.set(state, StationState::pNextConvo, 0x7100);
        e.call_log = Some(vec![]);
        e.call(0x0083_6030, &args![state]);
        assert_eq!(
            calls_to(&e, CONVERSATION_DELETE),
            vec![vec![0x7100, 1], vec![0x7000, 1]]
        );
        assert_eq!(e.get(state, StationState::pConvo), 0);
        assert_eq!(
            calls_to(&e, LIST_DESTRUCTOR),
            vec![vec![state.addr() + 0x18]]
        );
        e.set(state, StationState::pConvo, 0x7000);
        e.set(state, StationState::pNextConvo, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x0083_6030, &args![state]);
        assert_eq!(calls_to(&e, CONVERSATION_DELETE), vec![vec![0x7000, 1]]);
    }

    #[test]
    fn next_conversation_queues_a_task_or_creates_one() {
        let mut e = Engine::new();
        globals(&mut e);
        e.map(TASK_QUEUE & !0xfff, 0x1000);
        let queue = object_with_vtable(&mut e, 0x10, 0x00c1_0000);
        e.set_global(TASK_QUEUE, queue);
        stub(&mut e, &[CONVERSATION_DELETE, 0x00c1_0048]);
        answer(&mut e, GET_TOPIC, 0x6100);
        let block = e.mem.alloc(0x38);
        answer(&mut e, OPERATOR_NEW, block);
        answer(&mut e, RADIO_TASK_CONSTRUCT, block);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x5000);
        e.mem.set_u32(station + 4, 0x7000);
        e.mem.set_u32(station + 8, 0x7100);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0083_7060, &args![station, 0u32]).bool());
        assert_eq!(calls_to(&e, GET_TOPIC), vec![vec![7, 0]]);
        assert_eq!(
            calls_to(&e, CONVERSATION_DELETE),
            vec![vec![0x7000, 1], vec![0x7100, 1]]
        );
        assert_eq!(e.mem.u32(station + 4), 0x7100);
        assert_eq!(e.mem.u32(station + 8), 0);
        assert_eq!(calls_to(&e, 0x00c1_0048), vec![vec![queue, block]]);
        answer(&mut e, CONVERSATION_CONSTRUCT, 0x7200);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0083_7060, &args![station, 0x6200u32]).bool());
        assert_eq!(
            calls_to(&e, CONVERSATION_CONSTRUCT),
            vec![vec![block, 0x5000, 0x5000, 0x6200]]
        );
        assert_eq!(e.mem.u32(station + 4), 0x7200);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn the_two_radio_bytes() {
        let mut e = Engine::new();
        e.map(0x011d_d000, 0x1000);
        e.call(0x0083_25a0, &args![0x1234_5607u32]);
        assert_eq!(e.global::<u8>(HOLD_FLAG_011DD313), 7);
        assert_eq!(e.global::<u8>(HOLD_FLAG_011DD313 + 1), 0);
        e.set_global(STORED_FLAG_011DD437, 0x5au8);
        assert_eq!(e.call(0x0083_3ce0, &args![]).u8(), 0x5a);
    }

    #[test]
    fn station_flags_0x100_0x80_and_8() {
        let mut e = Engine::new();
        let state = e.new_object::<StationState>();
        e.set(state, StationState::iStationFlags, 0x0001_0008);
        // Only the low byte of the flag word counts.
        e.call(0x0083_2490, &args![state, 0x100u32]);
        assert_eq!(e.get(state, StationState::iStationFlags), 0x0001_0008);
        e.call(0x0083_2490, &args![state, 1u32]);
        assert_eq!(e.get(state, StationState::iStationFlags), 0x0001_0108);
        assert!(!e.call(0x0083_3c00, &args![state]).bool());
        e.call(0x0083_5690, &args![state, 1u32]);
        assert_eq!(e.call(0x0083_3c00, &args![state]).u32(), 1);
        assert_eq!(e.call(0x0083_5670, &args![state]).u32(), 1);
        e.call(0x0083_5690, &args![state, 0u32]);
        e.call(0x0083_2490, &args![state, 0u32]);
        assert_eq!(e.get(state, StationState::iStationFlags), 0x0001_0008);
        e.set(state, StationState::iStationFlags, !8u32);
        assert_eq!(e.call(0x0083_5670, &args![state]).u32(), 0);
    }

    #[test]
    fn station_flags_4_0x10_and_0x40() {
        let mut e = Engine::new();
        let state = e.new_object::<StationState>();
        e.set(state, StationState::iStationFlags, 0x0001_0004);
        assert_eq!(e.call(0x0083_3bc0, &args![state]).u32(), 1);
        assert_eq!(e.call(0x0083_3be0, &args![state]).u32(), 0);
        e.set(state, StationState::iStationFlags, 0x10);
        assert_eq!(e.call(0x0083_3bc0, &args![state]).u32(), 0);
        assert_eq!(e.call(0x0083_3be0, &args![state]).u32(), 1);
        // Only the low byte of the flag word counts.
        e.call(0x0083_3c90, &args![state, 0x100u32]);
        assert_eq!(e.get(state, StationState::iStationFlags), 0x10);
        e.call(0x0083_3c90, &args![state, 1u32]);
        assert_eq!(e.get(state, StationState::iStationFlags), 0x50);
        e.call(0x0083_3c90, &args![state, 0u32]);
        assert_eq!(e.get(state, StationState::iStationFlags), 0x10);
    }

    #[test]
    fn dialogue_flag_byte() {
        let mut e = Engine::new();
        e.map(DIALOGUE_FLAG_011DCFA4, 4);
        e.set_global(DIALOGUE_FLAG_011DCFA4, 0x0000_0207u32);
        assert_eq!(e.call(0x0083_3cf0, &args![]).u8(), 7);
    }

    #[test]
    fn radio_conv_task_constructor() {
        let mut e = Engine::new();
        e.register(TASK_BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_7250, &args![this, 0x5150u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(TASK_BASE_CONSTRUCT, vec![this, 0])]);
        assert_eq!(e.mem.u32(this), RADIO_CONV_TASK_VTABLE);
        assert_eq!(e.mem.u32(this + 0x30), 0x5150);
    }
}

/// The media calls `PipboyRadioEnable` makes (`cdecl`): whether media is
/// playing (`00830ad0`, a byte), close (`008304a0`), play (`008300c0`, seven
/// words) and `008304c0`.
const MEDIA_IS_PLAYING_00830AD0: u32 = 0x0083_0ad0;
/// The byte `PipboyRadioEnable` stores and answers (`011dd434`, the
/// Pip-Boy radio's on flag).
const PIPBOY_RADIO_ON: u32 = SAVED_BYTE;
/// The save buffer's form id read (`008648e0`, `thiscall`, one word: the
/// address it stores the id at), byte read (`00864980`, `thiscall`:
/// address, size) and count (`00864a60`, `thiscall`).
const BUFFER_READ_FORM_ID: u32 = 0x0086_48e0;
const BUFFER_READ: u32 = 0x0086_4980;
const BUFFER_COUNT: u32 = 0x0086_4a60;
/// The RTTI type descriptors of `TESForm` (`01183028`) and
/// `TESObjectREFR` (`011841cc`) for `__RTDynamicCast` (`00ec43fb`).
const RTTI_TES_FORM: u32 = 0x0118_3028;
const RTTI_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// The form lookup by id (`004839c0`, `cdecl`).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `Conversation::Conversation` (`0083b830`, 0xc bytes) and
/// `Conversation::LoadGame_ov2` (`0083c330`, one word: the buffer), and the
/// call `008363a0` makes on its conversation (`0083c420`, one word).
const CONVERSATION_CONSTRUCTOR: u32 = 0x0083_b830;
const CONVERSATION_LOAD_GAME_OV2: u32 = 0x0083_c330;
const CONVERSATION_0083C420: u32 = 0x0083_c420;
/// `0063f7b0` (`thiscall` on a list node): removes the head node.
const LIST_REMOVE_HEAD_0063F7B0: u32 = 0x0063_f7b0;
/// The text `"Radio conversation task"` (`0107cf4c`) and the bounded
/// format (`00406d00`, `cdecl`: buffer, size, format, ...).
const TEXT_RADIO_CONVERSATION_TASK: u32 = 0x0107_cf4c;
const FORMAT_00406D00: u32 = 0x0040_6d00;
/// The fill (`00403d30`, `cdecl`: block, byte, size).
const FILL_00403D30: u32 = 0x0040_3d30;

/// A form id at `slot` resolved to a reference (`004839c0`, then
/// `__RTDynamicCast` from `TESForm` to `TESObjectREFR`), or null for 0.
fn resolve_reference(e: &mut Engine, slot: u32) {
    let reference = if e.mem.u32(slot) != 0 {
        let id = e.mem.u32(slot);
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        e.call(
            RTTI_CAST,
            &args![form, 0u32, RTTI_TES_FORM, RTTI_TES_OBJECT_REFR, 0u32],
        )
        .u32()
    } else {
        0
    };
    e.mem.set_u32(slot, reference);
}

// Translated from 00832240 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00832240 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The Pip-Boy station setter (`cdecl`, two words: a reference and a
/// byte; unnamed in the engine map): false unless the Pip-Boy radio is on
/// (`011dd434`) and not shutting down. Clears the saved string
/// (`011dd448`, 0x104 bytes). Without a reference, the first station in
/// range of the player (`004ff1a0` into a stack list) whose reference has
/// a form is taken; with one and `keep`, the lost station is cleared and
/// the stations updated (`00833d00` with 1). The current station
/// (`011dd42c`) becomes the reference's form's station (`00832830`), else a
/// new one (`00832cb0`; none turns the radio off, `008324e0` with 0), the
/// stations updated when not `keep`; a found station of the same form takes
/// the reference. When its flag `0x100` is set, its item restarts 50 after
/// the audio time (`00832490` with 0); then its flag 1 is set and the lost
/// station cleared. True.
pub fn fn_00832240(e: &mut Engine, reference: u32, keep: u32) -> bool {
    if e.global::<u8>(PIPBOY_RADIO_ON) == 0 || shutting_down(e) {
        return false;
    }
    let keep = keep as u8;
    let mut reference = reference;
    e.call(FILL_00403D30, &args![SAVED_STRING, 0u32, 0x104u32]);
    if reference == 0 {
        e.with_stack(8, |e, list| {
            e.call(LIST_CONSTRUCTOR, &args![list]);
            let player = e.global::<u32>(PLAYER);
            e.call(STATIONS_IN_RANGE, &args![player, list, 0u32]);
            let node = list.addr();
            if node != 0 && item_of(e, node) != 0 {
                let first = item_of(e, node);
                if e.call(FORM_OF, &args![first]).u32() != 0 {
                    reference = item_of(e, node);
                }
            }
            e.call(LIST_DESTRUCTOR, &args![list]);
        });
    } else if keep != 0 {
        e.set_global(LOST_PIPBOY_STATION, 0u32);
        fallout_radio_update_stations(e, 1);
    }
    let form = e.call(FORM_OF, &args![reference]).u32();
    let station = fn_00832830(e, form);
    e.set_global(CURRENT_PIPBOY_STATION, station);
    if e.global::<u32>(CURRENT_PIPBOY_STATION) == 0 {
        let station = fn_00832cb0(e, reference, 0);
        e.set_global(CURRENT_PIPBOY_STATION, station);
        if e.global::<u32>(CURRENT_PIPBOY_STATION) == 0 {
            fallout_radio_pipboy_radio_enable(e, 0);
        }
        if keep == 0 {
            fallout_radio_update_stations(e, 1);
        }
    } else {
        let station = e.global::<u32>(CURRENT_PIPBOY_STATION);
        let station_reference = e.mem.u32(station);
        let station_form = e.call(FORM_OF, &args![station_reference]).u32();
        if station_form == e.call(FORM_OF, &args![reference]).u32() {
            let station = e.global::<u32>(CURRENT_PIPBOY_STATION);
            e.mem.set_u32(station, reference);
        }
    }
    let state = |e: &Engine| {
        Ptr::new(
            e.global::<u32>(CURRENT_PIPBOY_STATION)
                .wrapping_add(STATION_STATE),
        )
    };
    if fn_00832420(e, state(e)) {
        let start = audio_time(e).wrapping_add(0x32);
        let station = e.global::<u32>(CURRENT_PIPBOY_STATION);
        e.mem.set_u32(station.wrapping_add(0xc), start);
        fn_00832490(e, state(e), 0);
    }
    fn_00832440(e, state(e), 1);
    e.set_global(LOST_PIPBOY_STATION, 0u32);
    true
}

// Translated from 008324e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::PipboyRadioEnable` (engine map; `cdecl`, one byte): false
/// while shutting down. On: unless the byte at `011dd437` is set, the media
/// is closed (`008304a0`), `008325a0` gets 1, media 7 plays (`008300c0`
/// with 0, 1000, 0, 0, 0.0, 0) and `008304c0` runs. Off: playing media is
/// closed (`008325a0` with 0), both Pip-Boy sound handles (`011dd5bc`,
/// `011dd638`) stopped and released and the current station cleared. The
/// byte is stored at `011dd434` and answered.
pub fn fallout_radio_pipboy_radio_enable(e: &mut Engine, on: u32) -> u8 {
    if shutting_down(e) {
        return 0;
    }
    let on = on as u8;
    if on != 0 {
        if e.global::<u8>(STORED_FLAG_011DD437) == 0 {
            e.call(MEDIA_CLOSE, &[]);
            fn_008325a0(e, 1);
            e.call(
                MEDIA_PLAY_008300C0,
                &args![7u32, 0u32, 0x3e8u32, 0u32, 0u32, 0.0f32.to_bits(), 0u32],
            );
            e.call(MEDIA_CALL_008304C0, &[]);
        }
    } else {
        if e.call(MEDIA_IS_PLAYING_00830AD0, &[]).u32() & 0xff != 0 {
            e.call(MEDIA_CLOSE, &[]);
            fn_008325a0(e, 0);
        }
        stop_and_release(e, PIPBOY_HANDLE_5BC);
        e.set_global(CURRENT_PIPBOY_STATION, 0u32);
        stop_and_release(e, PIPBOY_HANDLE_638);
    }
    e.set_global(PIPBOY_RADIO_ON, on);
    e.global::<u8>(PIPBOY_RADIO_ON)
}

// Translated from 00832c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no words): queued radio state
/// (`011dd438`) bit 1 cleared and bit 0 set (two writes), then the current
/// Pip-Boy station, when there is one, updated (`00834260`).
pub fn fn_00832c40(e: &mut Engine) {
    let state = e.global::<u32>(QUEUED_STATE) & 0xffff_fffd;
    e.set_global(QUEUED_STATE, state);
    let state = e.global::<u32>(QUEUED_STATE) | 1;
    e.set_global(QUEUED_STATE, state);
    if e.global::<u32>(CURRENT_PIPBOY_STATION) != 0 {
        let station = e.global::<u32>(CURRENT_PIPBOY_STATION);
        fn_00834260(e, station);
    }
}

// Translated from 00832c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no words): queued radio state
/// (`011dd438`) bit 0 cleared and bit 1 set (two writes).
pub fn fn_00832c80(e: &mut Engine) {
    let state = e.global::<u32>(QUEUED_STATE) & 0xffff_fffe;
    e.set_global(QUEUED_STATE, state);
    let state = e.global::<u32>(QUEUED_STATE) | 2;
    e.set_global(QUEUED_STATE, state);
}

// Translated from 00835b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word: a reference): false while
/// shutting down; otherwise whether the station of the reference's form
/// (`00832830`) has its `00833040` flag, false without a station.
pub fn fn_00835b90(e: &mut Engine, reference: u32) -> u8 {
    if shutting_down(e) {
        return 0;
    }
    let form = e.call(FORM_OF, &args![reference]).u32();
    let station = fn_00832830(e, form);
    if station == 0 {
        return 0;
    }
    u8::from(fn_00833040(
        e,
        Ptr::new(station.wrapping_add(STATION_STATE)),
    ))
}

// Translated from 00835f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The receiver's load (unnamed in the engine map; `thiscall`, `RET 4`: the
/// load buffer): the radio's form id into `+0` (`008648e0`), then the four
/// flag bytes `+0x1c` to `+0x1f` (`00864980`, one byte each).
pub fn fn_00835f80(e: &mut Engine, this: Ptr<FORadioReceiver>, buffer: u32) {
    e.call(BUFFER_READ_FORM_ID, &args![buffer, this]);
    for offset in 0x1c..0x20u32 {
        e.call(BUFFER_READ, &args![buffer, this.addr() + offset, 1u32]);
    }
}

// Translated from 00835fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The receiver's load fix-up (unnamed in the engine map; `thiscall`,
/// `RET 4`, the stack word is not read): the form id at `+0` becomes its
/// reference (`004839c0`, cast to `TESObjectREFR`), or null for 0.
pub fn fn_00835fe0(e: &mut Engine, this: Ptr<FORadioReceiver>, _unused_1: u32) {
    resolve_reference(e, this.addr());
}

// Translated from 00836240 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00836240 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The station state's load (unnamed in the engine map; `thiscall`,
/// `RET 4`: the load buffer): `+8` and `+0xc` (4 bytes), `+0x10` and
/// `+0x11` (1 byte), `+0x14` (4 bytes); the buffer's count of user form ids,
/// each user that is an active receiver (`GetActiveReceiver`) added to the
/// users at `+0x18` (`005ae3d0`); then a byte, and when it is set a new
/// conversation (`0083b830`, 0xc bytes; null when the allocation fails) at
/// `+0` loaded from the buffer (`0083c330`).
pub fn fn_00836240(e: &mut Engine, this: Ptr<StationState>, buffer: u32) {
    let base = this.addr();
    for (offset, size) in [(8u32, 4u32), (0xc, 4), (0x10, 1), (0x11, 1), (0x14, 4)] {
        e.call(BUFFER_READ, &args![buffer, base + offset, size]);
    }
    e.with_stack(0xc, |e, locals| {
        let id = locals.addr();
        let receiver = locals.addr() + 4;
        let flag = locals.addr() + 8;
        let users = e.call(BUFFER_COUNT, &args![buffer]).u32();
        for _ in 0..users {
            e.mem.set_u32(id, 0);
            e.call(BUFFER_READ_FORM_ID, &args![buffer, id]);
            let radio = e.mem.u32(id);
            let found = fallout_radio_get_active_receiver(e, radio);
            e.mem.set_u32(receiver, found);
            if found != 0 {
                e.call(LIST_ADD_HEAD, &args![base + 0x18, receiver]);
            }
        }
        e.mem.set_u8(flag, 0);
        e.call(BUFFER_READ, &args![buffer, flag, 1u32]);
        if e.mem.u8(flag) != 0 {
            let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
            let conversation = if block != 0 {
                e.call(CONVERSATION_CONSTRUCTOR, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(base, conversation);
            let conversation = e.mem.u32(base);
            e.call(CONVERSATION_LOAD_GAME_OV2, &args![conversation, buffer]);
        }
    });
}

// Translated from 008363a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The station state's load fix-up (unnamed in the engine map;
/// `thiscall`, `RET 4`): the conversation (when set) gets `0083c420` with
/// the word; then the users at `+0x18` lose every entry without a receiver
/// or whose receiver has no radio (removed after the previous node,
/// `00905330`, or as the head, `0063f7b0`), until an empty node.
pub fn fn_008363a0(e: &mut Engine, this: Ptr<StationState>, value: u32) {
    let base = this.addr();
    if e.mem.u32(base) != 0 {
        let conversation = e.mem.u32(base);
        e.call(CONVERSATION_0083C420, &args![conversation, value]);
    }
    let mut node = base + 0x18;
    let mut previous = 0u32;
    while node != 0 && !is_empty(e, node) {
        let receiver = item_of(e, node);
        let remove = receiver == 0 || e.mem.u32(receiver) == 0;
        if remove {
            if previous != 0 {
                let slot = e.call(NODE_ITEM, &args![node]).u32();
                e.call(LIST_REMOVE, &args![previous, slot]);
                node = next_of(e, previous);
            } else {
                e.call(LIST_REMOVE_HEAD_0063F7B0, &args![node]);
            }
        } else {
            previous = node;
            node = next_of(e, node);
        }
    }
}

// Translated from 00836470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The station state's load finish (unnamed in the engine map;
/// `thiscall`, `RET 4`, the stack word is not read): when the audio time is
/// before the item's start (`+8`) or the duration (`+0xc`) is not positive,
/// the item restarts (start 0, duration 1, the conversation deleted,
/// `00798450` with 1); otherwise the start becomes the audio time less it.
/// Then each user (until an empty node or a null entry) whose radio is
/// missing, has no form, or is a talking activator (`0x15`) without a
/// radio station (`004fd3c0`) has both sound handles stopped and released
/// and stops playing.
pub fn fn_00836470(e: &mut Engine, this: Ptr<StationState>, _unused_1: u32) {
    let base = this.addr();
    let now = audio_time(e);
    if now < e.mem.u32(base + 8) || e.mem.u32(base + 0xc) as i32 <= 0 {
        e.mem.set_u32(base + 8, 0);
        e.mem.set_u32(base + 0xc, 1);
        let conversation = e.mem.u32(base);
        delete_conversation(e, conversation);
        e.mem.set_u32(base, 0);
    } else {
        let start = audio_time(e).wrapping_sub(e.mem.u32(base + 8));
        e.mem.set_u32(base + 8, start);
    }
    let mut node = base + 0x18;
    while node != 0 && !is_empty(e, node) && item_of(e, node) != 0 {
        let stop = if radio_of(e, node) == 0 {
            true
        } else {
            let radio = radio_of(e, node);
            if e.call(FORM_OF, &args![radio]).u32() == 0 {
                true
            } else {
                let radio = radio_of(e, node);
                let form = e.call(FORM_OF, &args![radio]).u32();
                if e.call(FORM_TYPE, &args![form]).u32() == 0x15 {
                    let radio = radio_of(e, node);
                    let form = e.call(FORM_OF, &args![radio]).u32();
                    e.call(RADIO_STATION_FORM, &args![form]).u32() == 0
                } else {
                    false
                }
            }
        };
        if stop {
            for offset in [4u32, 0x10] {
                let receiver = item_of(e, node);
                e.call(SOUND_HANDLE_STOP, &args![receiver + offset]);
                let receiver = item_of(e, node);
                e.call(SOUND_HANDLE_RELEASE, &args![receiver + offset]);
            }
            let receiver = item_of(e, node);
            e.mem.set_u8(receiver + 0x1c, 0);
        }
        node = next_of(e, node);
    }
}

// Translated from 00836630 (decompiled, FalloutNV.exe 1.4.0.525)
/// The station's load (unnamed in the engine map; `thiscall`, `RET 4`: the
/// load buffer): the reference's form id into `+0` (`008648e0`), then the
/// state at `+4` (`00836240`).
pub fn fn_00836630(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(BUFFER_READ_FORM_ID, &args![buffer, this]);
    fn_00836240(e, Ptr::new(this.addr() + STATION_STATE), buffer);
}

// Translated from 00836660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The station's load fix-up (unnamed in the engine map; `thiscall`,
/// `RET 4`): the form id at `+0` resolved to its reference (null for 0),
/// then the state's fix-up (`008363a0`) with the word.
pub fn fn_00836660(e: &mut Engine, this: Ptr, value: u32) {
    resolve_reference(e, this.addr());
    fn_008363a0(e, Ptr::new(this.addr() + STATION_STATE), value);
}

// Translated from 008366c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The station's load finish (unnamed in the engine map; `thiscall`,
/// `RET 4`): the state's (`00836470`) with the word.
pub fn fn_008366c0(e: &mut Engine, this: Ptr, value: u32) {
    fn_00836470(e, Ptr::new(this.addr() + STATION_STATE), value);
}

// Translated from 00837280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`; a `RadioConvTask`
/// slot): 3 in `AL`.
pub fn fn_00837280(_e: &mut Engine, _this: Ptr) -> u8 {
    3
}

// Translated from 00837290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RadioConvTask`'s scalar deleting destructor (engine map; `thiscall`,
/// `RET 4`): the destructor (`008372c0`), then `00401030` frees `this` when
/// bit 0 of `flags` is set. Returns `this`.
pub fn radio_conv_task_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_008372c0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008373a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RadioConvTask::GetDescription` (engine map; `thiscall`, `RET 8`):
/// `"Radio conversation task"` formatted into the buffer (`00406d00`), and
/// true.
pub fn radio_conv_task_get_description(e: &mut Engine, _this: Ptr, buffer: u32, size: u32) -> bool {
    e.call(
        FORMAT_00406D00,
        &args![buffer, size, TEXT_RADIO_CONVERSATION_TASK],
    );
    true
}

/// This unit's n086 translations, by exe address.
fn funcs_n086() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0083_2240, fn_00832240(u32, u32) -> bool),
        entry!(0x0083_24e0, fallout_radio_pipboy_radio_enable(u32) -> u8),
        entry!(0x0083_2c40, fn_00832c40()),
        entry!(0x0083_2c80, fn_00832c80()),
        entry!(0x0083_5b90, fn_00835b90(u32) -> u8),
        entry!(0x0083_5f80, fn_00835f80(Ptr<FORadioReceiver>, u32)),
        entry!(0x0083_5fe0, fn_00835fe0(Ptr<FORadioReceiver>, u32)),
        entry!(0x0083_6240, fn_00836240(Ptr<StationState>, u32)),
        entry!(0x0083_63a0, fn_008363a0(Ptr<StationState>, u32)),
        entry!(0x0083_6470, fn_00836470(Ptr<StationState>, u32)),
        entry!(0x0083_6630, fn_00836630(Ptr, u32)),
        entry!(0x0083_6660, fn_00836660(Ptr, u32)),
        entry!(0x0083_66c0, fn_008366c0(Ptr, u32)),
        entry!(0x0083_7280, fn_00837280(Ptr) -> u8),
        entry!(
            0x0083_7290,
            radio_conv_task_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0083_73a0,
            radio_conv_task_get_description(Ptr, u32, u32) -> bool
        ),
        entry!(0x0083_31c0, fallout_radio_pipboy_update(u32)),
        entry!(0x0083_5810, fallout_radio_enable_npc_radio(u32, u32)),
        entry!(0x0083_59e0, fn_008359e0(u32, u32)),
        entry!(
            0x0083_5be0,
            fallout_radio_start_radio_conversation(u32, u32)
        ),
        entry!(0x0083_68b0, fallout_radio_load_game(u32)),
        entry!(0x0083_6af0, fallout_radio_init_load_game(u32)),
        entry!(0x0083_6e20, fallout_radio_finish_load_game(u32)),
        entry!(0x0083_6ef0, fallout_radio_revert()),
        entry!(0x0083_72e0, radio_conv_task_run(Ptr)),
    ]
}

#[cfg(test)]
mod n086_tests {
    use super::*;

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
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    fn lists(e: &mut Engine) {
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
    }

    fn globals(e: &mut Engine) {
        for page in [0x011d_d000u32, 0x011d_e000] {
            e.map(page, 0x1000);
        }
    }

    /// Nodes (item +0, next +4) for `items` from `head` on.
    fn fill(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, &item) in items.iter().enumerate() {
            e.mem.set_u32(node, item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            } else {
                e.mem.set_u32(node + 4, 0);
            }
        }
    }

    #[test]
    fn queued_state_bits_and_the_pipboy_update() {
        let mut e = Engine::new();
        globals(&mut e);
        e.set_global(QUEUED_STATE, 0x12u32);
        // A station without a reference: 00834260 returns at once.
        let station = e.mem.alloc(0x24);
        e.set_global(CURRENT_PIPBOY_STATION, station);
        e.call(0x0083_2c40, &[]);
        assert_eq!(e.global::<u32>(QUEUED_STATE), 0x11);
        e.call(0x0083_2c80, &[]);
        assert_eq!(e.global::<u32>(QUEUED_STATE), 0x12);
    }

    #[test]
    fn pipboy_radio_enable_on_and_off() {
        let mut e = Engine::new();
        globals(&mut e);
        quiet(
            &mut e,
            &[
                MEDIA_CLOSE,
                MEDIA_PLAY_008300C0,
                MEDIA_CALL_008304C0,
                SOUND_HANDLE_STOP,
                SOUND_HANDLE_RELEASE,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_24e0, &args![0x101u32]).u32() & 0xff, 1);
        assert_eq!(e.global::<u8>(PIPBOY_RADIO_ON), 1);
        assert_eq!(e.global::<u8>(HOLD_FLAG_011DD313), 1);
        assert_eq!(
            calls(&e, MEDIA_PLAY_008300C0),
            vec![vec![7, 0, 0x3e8, 0, 0, 0, 0]]
        );
        e.register(MEDIA_IS_PLAYING_00830AD0, |_, _| 0x101u32.into_ret());
        e.set_global(CURRENT_PIPBOY_STATION, 0x77u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_24e0, &args![0u32]).u32() & 0xff, 0);
        assert_eq!(e.global::<u8>(HOLD_FLAG_011DD313), 0);
        assert_eq!(e.global::<u32>(CURRENT_PIPBOY_STATION), 0);
        assert_eq!(
            calls(&e, SOUND_HANDLE_STOP),
            vec![vec![PIPBOY_HANDLE_5BC], vec![PIPBOY_HANDLE_638]]
        );
        // While shutting down: nothing.
        e.set_global(SHUTTING_DOWN, 1u8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_24e0, &args![1u32]).u32() & 0xff, 0);
        assert!(calls(&e, MEDIA_CLOSE).is_empty());
    }

    #[test]
    fn set_pipboy_station_takes_the_found_station() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        quiet(&mut e, &[FILL_00403D30]);
        e.register(FORM_OF, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(REFERENCE_TEST_00440DA0, |_, _| 0u32.into_ret());
        e.register(REFERENCE_VALUE_0056B210, |_, _| 0u32.into_ret());
        e.register(AUDIO_INSTANCE, |_, _| 0u32.into_ret());
        e.register(AUDIO_TIME, |_, _| 1000u32.into_ret());
        // A station of form 0x55 in the station list.
        let old = e.mem.alloc(0x30);
        e.mem.set_u32(old + 0x20, 0x55);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, old);
        e.mem.set_u32(station + 4 + 0x14, 0x100);
        fill(&mut e, STATION_LIST, &[station]);
        let reference = e.mem.alloc(0x30);
        e.mem.set_u32(reference + 0x20, 0x55);
        e.set_global(PIPBOY_RADIO_ON, 1u8);
        e.set_global(LOST_PIPBOY_STATION, 9u32);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0083_2240, &args![reference, 0u32]).bool());
        assert_eq!(e.global::<u32>(CURRENT_PIPBOY_STATION), station);
        assert_eq!(e.mem.u32(station), reference);
        // Flag 0x100 restarted the item 50 after the audio time, then
        // cleared; flag 1 set.
        assert_eq!(e.mem.u32(station + 0xc), 1050);
        assert_eq!(e.mem.u32(station + 4 + 0x14), 1);
        assert_eq!(e.global::<u32>(LOST_PIPBOY_STATION), 0);
        assert_eq!(calls(&e, FILL_00403D30), vec![vec![SAVED_STRING, 0, 0x104]]);
        // Radio off: false at once.
        e.set_global(PIPBOY_RADIO_ON, 0u8);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0083_2240, &args![reference, 0u32]).bool());
        assert!(calls(&e, FILL_00403D30).is_empty());
    }

    #[test]
    fn station_flag_of_a_reference() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        e.register(FORM_OF, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(REFERENCE_TEST_00440DA0, |_, _| 0u32.into_ret());
        e.register(REFERENCE_VALUE_0056B210, |_, _| 0u32.into_ret());
        let reference = e.mem.alloc(0x30);
        e.mem.set_u32(reference + 0x20, 0x66);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, reference);
        fill(&mut e, STATION_LIST, &[station]);
        let flag = fn_00833040(&mut e, Ptr::new(station + 4));
        assert_eq!(
            e.call(0x0083_5b90, &args![reference]).u32() & 0xff,
            u32::from(flag)
        );
        e.mem.set_u32(station + 4 + 0x14, 0xffff_ffff);
        let flag = fn_00833040(&mut e, Ptr::new(station + 4));
        assert!(flag);
        assert_eq!(e.call(0x0083_5b90, &args![reference]).u32() & 0xff, 1);
        let other = e.mem.alloc(0x30);
        assert_eq!(e.call(0x0083_5b90, &args![other]).u32() & 0xff, 0);
        e.set_global(SHUTTING_DOWN, 1u8);
        assert_eq!(e.call(0x0083_5b90, &args![reference]).u32() & 0xff, 0);
    }

    #[test]
    fn receiver_and_station_loads_read_their_fields() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        quiet(
            &mut e,
            &[
                BUFFER_READ_FORM_ID,
                BUFFER_READ,
                LIST_ADD_HEAD,
                CONVERSATION_LOAD_GAME_OV2,
            ],
        );
        let receiver = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0083_5f80, &args![receiver, 0x4000u32]);
        assert_eq!(calls(&e, BUFFER_READ_FORM_ID), vec![vec![0x4000, receiver]]);
        assert_eq!(
            calls(&e, BUFFER_READ),
            (0x1c..0x20)
                .map(|o| vec![0x4000, receiver + o, 1])
                .collect::<Vec<_>>()
        );

        // The station: its state's fields, two users (one active), and a
        // conversation.
        e.register(BUFFER_COUNT, |_, _| 2u32.into_ret());
        e.register(BUFFER_READ_FORM_ID, |e, a| {
            e.mem.set_u32(a[1], 0x1234);
            Ret::default()
        });
        e.register(BUFFER_READ, |e, a| {
            if a[2] == 1 {
                e.mem.set_u8(a[1], 1);
            }
            Ret::default()
        });
        let active = e.mem.alloc(0x20);
        e.mem.set_u32(active, 0x1234);
        fill(&mut e, RECEIVER_LIST, &[active]);
        e.register(OPERATOR_NEW, |_, _| 0x9000u32.into_ret());
        e.register(CONVERSATION_CONSTRUCTOR, |_, a| (a[0] + 4).into_ret());
        let station = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        e.call(0x0083_6630, &args![station, 0x4000u32]);
        let reads = calls(&e, BUFFER_READ);
        assert_eq!(reads[0], vec![0x4000, station + 4 + 8, 4]);
        assert_eq!(reads[4], vec![0x4000, station + 4 + 0x14, 4]);
        assert_eq!(calls(&e, LIST_ADD_HEAD).len(), 2);
        assert_eq!(calls(&e, LIST_ADD_HEAD)[0][0], station + 4 + 0x18);
        assert_eq!(e.mem.u32(station + 4), 0x9004);
        assert_eq!(
            calls(&e, CONVERSATION_LOAD_GAME_OV2),
            vec![vec![0x9004, 0x4000]]
        );
    }

    #[test]
    fn load_fix_ups_resolve_and_prune() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        quiet(&mut e, &[CONVERSATION_0083C420, LIST_REMOVE_HEAD_0063F7B0]);
        e.register(LIST_REMOVE, |e, a| {
            let after = e.mem.u32(a[1] + 4);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| (a[0] + 1).into_ret());
        e.register(RTTI_CAST, |_, a| (a[0] + 2).into_ret());
        let receiver = e.mem.alloc(0x20);
        e.mem.set_u32(receiver, 0x500);
        e.call_log = Some(vec![]);
        e.call(0x0083_5fe0, &args![receiver, 0u32]);
        assert_eq!(e.mem.u32(receiver), 0x503);
        assert_eq!(
            calls(&e, RTTI_CAST),
            vec![vec![0x501, 0, RTTI_TES_FORM, RTTI_TES_OBJECT_REFR, 0]]
        );
        e.mem.set_u32(receiver, 0);
        e.call(0x0083_5fe0, &args![receiver, 0u32]);
        assert_eq!(e.mem.u32(receiver), 0);

        // A station whose second user lost its radio.
        let kept = e.mem.alloc(0x20);
        e.mem.set_u32(kept, 0x77);
        let lost = e.mem.alloc(0x20);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x600);
        e.mem.set_u32(station + 4, 0x0c0c);
        fill(&mut e, station + 4 + 0x18, &[kept, lost]);
        let second = e.mem.u32(station + 4 + 0x18 + 4);
        e.call_log = Some(vec![]);
        e.call(0x0083_6660, &args![station, 9u32]);
        assert_eq!(e.mem.u32(station), 0x603);
        assert_eq!(calls(&e, CONVERSATION_0083C420), vec![vec![0x0c0c, 9]]);
        assert_eq!(
            calls(&e, LIST_REMOVE),
            vec![vec![station + 4 + 0x18, second]]
        );
        // A head entry without a receiver goes as the head.
        let head = station + 4 + 0x18;
        e.mem.set_u32(head, 0);
        e.mem.set_u32(head + 4, 0);
        e.register(LIST_REMOVE_HEAD_0063F7B0, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        let next = e.mem.alloc(8);
        e.mem.set_u32(head + 4, next);
        e.call_log = Some(vec![]);
        e.call(0x0083_63a0, &args![station + 4, 9u32]);
        assert_eq!(calls(&e, LIST_REMOVE_HEAD_0063F7B0), vec![vec![head]]);
    }

    #[test]
    fn load_finish_restarts_or_rebases_the_item() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        quiet(
            &mut e,
            &[CONVERSATION_DELETE, SOUND_HANDLE_STOP, SOUND_HANDLE_RELEASE],
        );
        e.register(AUDIO_INSTANCE, |_, _| 0u32.into_ret());
        e.register(AUDIO_TIME, |_, _| 500u32.into_ret());
        e.register(FORM_OF, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(FORM_TYPE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(RADIO_STATION_FORM, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let station = e.mem.alloc(0x24);
        let state = station + 4;
        e.mem.set_u32(state + 8, 200);
        e.mem.set_u32(state + 0xc, 10);
        // Users: a plain radio (kept), a talking activator without a
        // station (stopped).
        let form = e.mem.alloc(8);
        e.mem.set_u32(form, 0x15);
        let radio = e.mem.alloc(0x24);
        e.mem.set_u32(radio + 0x20, form);
        let plain_form = e.mem.alloc(8);
        let plain = e.mem.alloc(0x24);
        e.mem.set_u32(plain + 0x20, plain_form);
        let kept = e.mem.alloc(0x20);
        e.mem.set_u32(kept, plain);
        e.mem.set_u8(kept + 0x1c, 1);
        let stopped = e.mem.alloc(0x20);
        e.mem.set_u32(stopped, radio);
        e.mem.set_u8(stopped + 0x1c, 1);
        fill(&mut e, state + 0x18, &[kept, stopped]);
        e.call_log = Some(vec![]);
        e.call(0x0083_66c0, &args![station, 0u32]);
        assert_eq!(e.mem.u32(state + 8), 300);
        assert_eq!(e.mem.u8(kept + 0x1c), 1);
        assert_eq!(e.mem.u8(stopped + 0x1c), 0);
        assert_eq!(
            calls(&e, SOUND_HANDLE_STOP),
            vec![vec![stopped + 4], vec![stopped + 0x10]]
        );
        // A non-positive duration restarts the item.
        e.mem.set_u32(state, 0x0d0d);
        e.mem.set_u32(state + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0083_6470, &args![state, 0u32]);
        assert_eq!(e.mem.u32(state + 8), 0);
        assert_eq!(e.mem.u32(state + 0xc), 1);
        assert_eq!(e.mem.u32(state), 0);
        assert_eq!(calls(&e, CONVERSATION_DELETE), vec![vec![0x0d0d, 1]]);
    }

    #[test]
    fn radio_conv_task_slots() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x00c3_cea0, OPERATOR_DELETE, FORMAT_00406D00]);
        assert_eq!(e.call(0x0083_7280, &args![0u32]).u32() & 0xff, 3);
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_7290, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0107_cf18);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0083_7290, &args![this, 1u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0083_73a0, &args![this, 0x500u32, 0x40u32]).bool());
        assert_eq!(
            calls(&e, FORMAT_00406D00),
            vec![vec![0x500, 0x40, TEXT_RADIO_CONVERSATION_TASK]]
        );
    }
}

/// The texts and doubles of `PipboyUpdate`: the lost-signal sound
/// `"UIRadioSignalLost"` (`0107ce80`), the music file extension `".wav"`
/// (`0107ce2c`), the news path `"data\sound\"` (`0107ce20`) with the format
/// `"%s%s"` (`0101996c`), and the volume factor (`0101ffa0`, a double).
const SOUND_SIGNAL_LOST: u32 = 0x0107_ce80;
const TEXT_WAV: u32 = 0x0107_ce2c;
const TEXT_SOUND_PATH: u32 = 0x0107_ce20;
const TEXT_TWO_STRINGS: u32 = 0x0101_996c;
const VOLUME_FACTOR_0101FFA0: u32 = 0x0101_ffa0;
/// The setting (`011d25f8`) whose text (`00403df0`) is the lost-signal
/// message format, and the HUD object (`011d96c0`) whose `00775380`
/// (`thiscall`, six words) shows it.
const SETTING_SIGNAL_LOST: u32 = 0x011d_25f8;
const SETTING_TEXT_00403DF0: u32 = 0x0040_3df0;
const HUD_011D96C0: u32 = 0x011d_96c0;
const HUD_SHOW_00775380: u32 = 0x0077_5380;
/// `00830750` (`cdecl`: media, 1; a byte): whether media 7 is playing.
const MEDIA_PLAYING_00830750: u32 = 0x0083_0750;
/// `00700270` (`thiscall`, one word; a byte): the response list's test.
const RESPONSE_LIST_TEST_00700270: u32 = 0x0070_0270;
/// `strstr` (`00ec7750`, `cdecl`).
const STRSTR: u32 = 0x00ec_7750;
/// The string constructor from a text (`0040c0e0`, `thiscall`, one word).
const STRING_FROM_TEXT: u32 = 0x0040_c0e0;
/// `00ad8930` (`thiscall` on a sound handle; a byte).
const SOUND_TEST_00AD8930: u32 = 0x00ad_8930;
/// `00ad7ae0` (`thiscall` on the audio; a float in `ST0`): the volume the
/// radio's sounds are scaled by outside dialogue.
const AUDIO_VOLUME_00AD7AE0: u32 = 0x00ad_7ae0;
/// `00705210` (`cdecl`, nine words): the subtitle for a played response
/// (its text, the sound handle and the player's position by value, the
/// player, 0).
const SHOW_SUBTITLE_00705210: u32 = 0x0070_5210;
/// The save buffer's string read (`008649a0`, one word: the address) and
/// form read (`008648a0`, no words: the form id).
const BUFFER_READ_STRING: u32 = 0x0086_49a0;
const BUFFER_READ_FORM: u32 = 0x0086_48a0;
/// The RTTI type descriptor of `TESBoundObject` (`01183108`).
const RTTI_TES_BOUND_OBJECT: u32 = 0x0118_3108;
/// `00726c60` (`thiscall` on a list node, one word: the address of the
/// item): sets the node's item.
const NODE_SET_ITEM_00726C60: u32 = 0x0072_6c60;
/// `00511370` (`thiscall` on the station's form, one word): the broadcast
/// state given to the form.
const FORM_SET_BROADCAST_00511370: u32 = 0x0051_1370;
/// The critical section names (`0107cecc`
/// `"FalloutRadio::SetBroadcastState"`, `0107ceec`
/// `"FalloutRadio::StartRadioConversation"`).
const NAME_SET_BROADCAST_STATE: u32 = 0x0107_cecc;
const NAME_START_RADIO_CONVERSATION: u32 = 0x0107_ceec;
/// The import `EnableNPCRadio` calls for a station without a reference
/// (`00fdf0c8`, no words).
const IMPORT_00FDF0C8: u32 = 0x00fd_f0c8;

/// Media 7 restarted for the Pip-Boy (`008304a0`, `008325a0` with 1,
/// `008300c0` with 0, 1000, 0, 0, 0.0, 0, `008304c0`).
fn restart_media(e: &mut Engine) {
    e.call(MEDIA_CLOSE, &[]);
    fn_008325a0(e, 1);
    e.call(
        MEDIA_PLAY_008300C0,
        &args![7u32, 0u32, 0x3e8u32, 0u32, 0u32, 0.0f32.to_bits(), 0u32],
    );
    e.call(MEDIA_CALL_008304C0, &[]);
}

/// The Pip-Boy's voice handle (`011dd5bc`) given the volume: the strength
/// times the double `0101ffa0` in dialogue (`00833cf0`), else the audio's
/// volume (`00ad7ae0`) times the strength.
fn set_voice_volume(e: &mut Engine, strength: u32) {
    let volume = if fn_00833cf0(e) != 0 {
        let factor = f64::from_bits(e.mem.u64(VOLUME_FACTOR_0101FFA0));
        x87_store32(x87_mul(x87_load32(strength), factor))
    } else {
        let audio = e.call(AUDIO_INSTANCE, &[]).u32();
        let volume = e.call(AUDIO_VOLUME_00AD7AE0, &args![audio]).f64();
        x87_store32(x87_mul(volume, x87_m32(strength)))
    };
    e.call(
        SOUND_SET_VOLUME,
        &args![PIPBOY_HANDLE_5BC, x87_quiet_bits(volume)],
    );
}

/// A sound by name into the handle at `handle` (`GetSoundHandleByName`
/// into a stack temporary, assigned, the temporary destroyed).
fn load_named_sound(e: &mut Engine, handle: u32, name: u32, flags: u32) {
    e.with_stack(0xc, |e, temporary| {
        let audio = e.call(AUDIO_INSTANCE, &[]).u32();
        let found = e
            .call(AUDIO_SOUND_BY_NAME, &args![audio, temporary, name, flags])
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![handle, found]);
        e.call(SOUND_HANDLE_DESTRUCTOR, &args![temporary]);
    });
}

/// A sound by file into the voice handle (`00ad7480` into a stack
/// temporary with the flags `0x100101` and the form, assigned, the
/// temporary destroyed).
fn load_voice_file(e: &mut Engine, file: u32, form: u32) {
    e.with_stack(0xc, |e, temporary| {
        let audio = e.call(AUDIO_INSTANCE, &[]).u32();
        let found = e
            .call(
                AUDIO_SOUND_BY_FILE,
                &args![audio, temporary, file, 0x0010_0101u32, form],
            )
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![PIPBOY_HANDLE_5BC, found]);
        e.call(SOUND_HANDLE_DESTRUCTOR, &args![temporary]);
    });
}

/// The current Pip-Boy station's state (`011dd42c` read again, plus 4).
fn pipboy_state(e: &Engine) -> Ptr<StationState> {
    Ptr::new(
        e.global::<u32>(CURRENT_PIPBOY_STATION)
            .wrapping_add(STATION_STATE),
    )
}

/// The current Pip-Boy station (`011dd42c`).
fn pipboy_station(e: &Engine) -> u32 {
    e.global::<u32>(CURRENT_PIPBOY_STATION)
}

/// The current response of `item` (`0083c820`).
fn current_response(e: &mut Engine, item: u32) -> u32 {
    e.call(CURRENT_RESPONSE, &args![item]).u32()
}

// Translated from 008331c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008331c0 - the C++ exception-unwinding frame (FS:[0]) and the /GS stack-cookie check (00ec408c) are not translated.
/// `FalloutRadio::PipboyUpdate` (engine map; `cdecl`, one byte: force).
/// The strength is the current station's power over 100 (0 without a
/// station). A station that lost its signal (flag 4) plays
/// `"UIRadioSignalLost"`, becomes the lost station, turns the Pip-Boy radio
/// off, stops the static and shows the setting `011d25f8`'s message with
/// the station's name (`00775380`), and nothing else happens. Below full
/// power the static (`"UIRadioStaticLP"`, started when not valid) gets the
/// curve volume of the missing power; at full power a valid static stops.
/// A station with no duration takes the voice's (`00ad8b30`). With a
/// conversation, unless media 7 or the voice is playing (and not `force`):
/// the current item's response (first one made current when there is
/// none): a response with a form plays as news (flag 8, unless
/// `00833ce0`), a response with a long enough text whose list passes
/// `00700270`, or whose sound file has `".wav"`, plays as music through the
/// voice handle (by file from the form, else from the response list; the
/// completion callback `008357e0` with the station, the synch start, the
/// volume, a subtitle when the voice is not valid), and news plays as media
/// 7 from `"data\sound\"` and the file (kept at `011dd448`, the duration
/// taken from the media when the station has none); neither sets the
/// duration to 1. Without one, a station without flags `0x20`, `0x80` and
/// `0x10` and with the voice not `00ad8930` plays the static through the
/// voice handle (flag `0x40` set, flag 8 cleared). The voice gets the
/// volume unless music started it.
pub fn fallout_radio_pipboy_update(e: &mut Engine, force: u32) {
    let force = force as u8;
    e.with_stack(0x1e0, |e, frame| {
        // The frame is EBP - 0x1e0.
        let f = frame.addr();
        let at = |offset: u32| f + 0x1e0 - offset;
        let strength = if pipboy_station(e) != 0 {
            let power = fn_00833c20(e, pipboy_state(e)) as u8;
            power_volume(e, power)
        } else {
            0.0f32.to_bits()
        };
        let mut volume_set = false;
        if pipboy_station(e) != 0 && fn_00833bc0(e, pipboy_state(e)) {
            let message = at(0x1c);
            let temporary = at(0x28);
            e.call(STRING_CONSTRUCT, &args![message]);
            let audio = e.call(AUDIO_INSTANCE, &[]).u32();
            e.call(
                AUDIO_SOUND_BY_NAME,
                &args![audio, temporary, SOUND_SIGNAL_LOST, 0x0010_0121u32],
            );
            e.call(SOUND_HANDLE_PLAY, &args![temporary, 0u32]);
            let station = pipboy_station(e);
            let reference = e.mem.u32(station);
            let name = e.call(REFERENCE_NAME, &args![reference]).u32();
            let format = e
                .call(SETTING_TEXT_00403DF0, &args![SETTING_SIGNAL_LOST])
                .u32();
            e.call(STRING_FORMAT, &args![message, format, name]);
            let station = pipboy_station(e);
            e.set_global(LOST_PIPBOY_STATION, station);
            fallout_radio_pipboy_radio_enable(e, 0);
            stop_and_release(e, PIPBOY_HANDLE_638);
            let text = e.call(STRING_C_STR, &args![message]).u32();
            let hud = e.global::<u32>(HUD_011D96C0);
            let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME));
            e.call(
                HUD_SHOW_00775380,
                &args![hud, text, 0u32, DISCOVERY_TEXT, 0u32, time, 0u32],
            );
            e.call(SOUND_HANDLE_DESTRUCTOR, &args![temporary]);
            e.call(STRING_DESTROY, &args![message]);
            return;
        }
        if pipboy_station(e) != 0 && (fn_00833c20(e, pipboy_state(e)) & 0xff) < 100 {
            if e.call(SOUND_IS_VALID, &args![PIPBOY_HANDLE_638]).u32() & 0xff == 0 {
                stop_and_release(e, PIPBOY_HANDLE_638);
                load_named_sound(e, PIPBOY_HANDLE_638, STATIC_SOUND, 0x0010_0031);
                e.call(SOUND_HANDLE_PLAY, &args![PIPBOY_HANDLE_638, 0u32]);
            }
            let power = fn_00833c20(e, pipboy_state(e)) as u8;
            let volume = static_volume(e, power);
            e.call(
                SOUND_SET_VOLUME,
                &args![PIPBOY_HANDLE_638, x87_quiet_bits(volume)],
            );
        } else if pipboy_station(e) != 0
            && fn_00833c20(e, pipboy_state(e)) & 0xff == 100
            && e.call(SOUND_IS_VALID, &args![PIPBOY_HANDLE_638]).u32() & 0xff != 0
        {
            stop_and_release(e, PIPBOY_HANDLE_638);
        }
        if pipboy_station(e) != 0
            && e.mem.u32(pipboy_station(e) + 0x10) as i32 <= 0
            && e.call(SOUND_IS_VALID, &args![PIPBOY_HANDLE_5BC]).u32() & 0xff != 0
            && e.call(SOUND_DURATION, &args![PIPBOY_HANDLE_5BC]).u32() != 0
        {
            let duration = e.call(SOUND_DURATION, &args![PIPBOY_HANDLE_5BC]).u32();
            let station = pipboy_station(e);
            e.mem.set_u32(station + 0x10, duration);
        }
        let talking = pipboy_station(e) != 0
            && e.mem.u32(pipboy_station(e) + 4) != 0
            && ((e.call(MEDIA_PLAYING_00830750, &args![7u32, 1u32]).u32() & 0xff == 0
                && e.call(SOUND_IS_VALID, &args![PIPBOY_HANDLE_5BC]).u32() & 0xff == 0)
                || force != 0);
        if talking {
            let conversation = e.mem.u32(pipboy_station(e) + 4);
            let item = current_response(e, conversation);
            if item == 0 {
                return;
            }
            let mut news = false;
            let mut music = false;
            if current_response(e, item) == 0 {
                e.call(FIRST_RESPONSE, &args![item]);
            }
            let response = current_response(e, item);
            let with_form = response != 0 && {
                let response = current_response(e, item);
                e.call(FORM_OF, &args![response]).u32() != 0
            };
            if with_form && fn_00833ce0(e) == 0 {
                news = true;
                if pipboy_station(e) != 0 {
                    fn_00833c40(e, pipboy_state(e), 1);
                }
            } else if pipboy_station(e) != 0 {
                fn_00833c40(e, pipboy_state(e), 0);
            }
            let has_text = current_response(e, item) != 0 && {
                let response = current_response(e, item);
                let slot = e.call(NODE_ITEM, &args![response]).u32();
                e.call(STRING_C_STR, &args![slot]).u32() != 0
            };
            if has_text {
                let response = current_response(e, item);
                let slot = e.call(NODE_ITEM, &args![response]).u32();
                let text = e.call(STRING_C_STR, &args![slot]).u32();
                let length = e.call(LIST_COUNT_0044A670, &args![text]).u32();
                music = length > 2 && {
                    let response = current_response(e, item);
                    let list = e.call(RESPONSE_LIST_00460140, &args![response]).u32();
                    e.call(RESPONSE_LIST_TEST_00700270, &args![list, 0u32])
                        .u32()
                        & 0xff
                        != 0
                };
                if !music {
                    let response = current_response(e, item);
                    if e.call(FORM_OF, &args![response]).u32() != 0 {
                        let response = current_response(e, item);
                        let form = e.call(FORM_OF, &args![response]).u32();
                        let file = e.call(SOUND_FILE_NAME, &args![form]).u32();
                        if e.call(STRSTR, &args![file, TEXT_WAV]).u32() != 0 {
                            music = true;
                        }
                    }
                }
            }
            let _elapsed = audio_time(e).wrapping_sub(e.mem.u32(pipboy_station(e) + 0xc));
            if music {
                let response = current_response(e, item);
                let has_form = e.call(FORM_OF, &args![response]).u32() != 0;
                stop_and_release(e, PIPBOY_HANDLE_5BC);
                if has_form {
                    let response = current_response(e, item);
                    let form = e.call(FORM_OF, &args![response]).u32();
                    let response = current_response(e, item);
                    let form_again = e.call(FORM_OF, &args![response]).u32();
                    let file = e.call(SOUND_FILE_NAME, &args![form_again]).u32();
                    load_voice_file(e, file, form);
                } else {
                    let response = current_response(e, item);
                    let list = e.call(RESPONSE_LIST_00460140, &args![response]).u32();
                    let file = e.call(STRING_C_STR, &args![list]).u32();
                    load_voice_file(e, file, 0);
                }
                let station = pipboy_station(e);
                e.call(
                    SOUND_SET_COMPLETION,
                    &args![PIPBOY_HANDLE_5BC, COMPLETION_CALLBACK, station],
                );
                let start = e.mem.u32(pipboy_station(e) + 0xc);
                e.call(SOUND_SET_SYNCH_START, &args![PIPBOY_HANDLE_5BC, start]);
                set_voice_volume(e, strength);
                volume_set = true;
                e.call(SOUND_HANDLE_PLAY, &args![PIPBOY_HANDLE_5BC, 0u32]);
                if e.call(SOUND_IS_VALID, &args![PIPBOY_HANDLE_5BC]).u32() & 0xff == 0 {
                    show_subtitle(e, item, at(0x184));
                }
            }
            if news && fn_00833ce0(e) == 0 {
                let buffer = at(0x154);
                let path = at(0x15c);
                let response = current_response(e, item);
                let form = e.call(FORM_OF, &args![response]).u32();
                let file = e.call(SOUND_FILE_NAME, &args![form]).u32();
                e.call(
                    FORMAT_00406D00,
                    &args![buffer, 0x104u32, TEXT_TWO_STRINGS, TEXT_SOUND_PATH, file],
                );
                e.call(STRING_FROM_TEXT, &args![path, buffer]);
                let start = e.mem.u32(pipboy_station(e) + 0xc);
                e.call(
                    MEDIA_PLAY_008300C0,
                    &args![7u32, buffer, 0x1f4u32, 0u32, 1u32, 0.0f32.to_bits(), start],
                );
                e.call(COPY_BOUNDED, &args![SAVED_STRING, 0x104u32, buffer]);
                fn_00833c40(e, pipboy_state(e), 1);
                if e.mem.u32(pipboy_station(e) + 0x10) as i32 <= 0
                    && e.call(MEDIA_DURATION, &args![7u32]).u32() != 0
                {
                    let duration = e.call(MEDIA_DURATION, &args![7u32]).u32();
                    let station = pipboy_station(e);
                    e.mem.set_u32(station + 0x10, duration);
                }
                e.call(STRING_DESTROY, &args![path]);
            } else {
                fn_00833c40(e, pipboy_state(e), 0);
            }
            if !news && !music {
                let station = pipboy_station(e);
                e.mem.set_u32(station + 0x10, 1);
            }
        } else if !fn_00833040(e, pipboy_state(e))
            && !fn_00833c00(e, pipboy_state(e))
            && !fn_00833be0(e, pipboy_state(e))
            && e.call(SOUND_TEST_00AD8930, &args![PIPBOY_HANDLE_5BC]).u32() & 0xff == 0
        {
            stop_and_release(e, PIPBOY_HANDLE_5BC);
            load_named_sound(e, PIPBOY_HANDLE_5BC, STATIC_SOUND, 0x0010_0031);
            e.call(SOUND_SET_FLAG_004F15A0, &args![PIPBOY_HANDLE_5BC, 1u32]);
            e.call(SOUND_HANDLE_PLAY, &args![PIPBOY_HANDLE_5BC, 0u32]);
            fn_00833c90(e, pipboy_state(e), 1);
            fn_00833c40(e, pipboy_state(e), 0);
        }
        if !volume_set {
            set_voice_volume(e, strength);
        }
    });
}

/// The subtitle of `PipboyUpdate` (`00705210`): the voice handle copied
/// into a stack temporary (`00418900`), the player's position (its slot
/// `+0x1f4`) and both passed by value with the response's text, the player
/// and 0.
fn show_subtitle(e: &mut Engine, item: u32, handle_copy: u32) {
    let player = e.global::<u32>(PLAYER);
    let position = e.vcall(player, 0x1f4, &[]).u32();
    let point: Vec<u32> = (0..3).map(|i| e.mem.u32(position + i * 4)).collect();
    e.call(SOUND_HANDLE_ASSIGN, &args![handle_copy, PIPBOY_HANDLE_5BC]);
    let handle: Vec<u32> = (0..3).map(|i| e.mem.u32(handle_copy + i * 4)).collect();
    let response = current_response(e, item);
    let slot = e.call(NODE_ITEM, &args![response]).u32();
    let text = e.call(STRING_C_STR, &args![slot]).u32();
    e.call(
        SHOW_SUBTITLE_00705210,
        &args![text, handle[0], handle[1], handle[2], point[0], point[1], point[2], player, 0u32],
    );
}

// Translated from 00835810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00835810 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `FalloutRadio::EnableNPCRadio` (engine map; `cdecl`: the radio and the
/// station references): nothing while shutting down. The station of the
/// station's form (the age of its item computed from the audio time), else
/// a new one (`00832cb0`; one without a reference calls the import at
/// `00fdf0c8`); none stops here. The radio's receiver (a new one, 0x20
/// bytes, added to the receiver list, when it has none) gets the radio,
/// should-be-playing, just-started and is-actor set, joins the station's
/// users and the station's flag 1 is set.
pub fn fallout_radio_enable_npc_radio(e: &mut Engine, radio: u32, station_reference: u32) {
    e.with_stack(4, |e, slot| {
        let receiver = fallout_radio_get_active_receiver(e, radio);
        e.mem.set_u32(slot.addr(), receiver);
        if shutting_down(e) {
            return;
        }
        let form = e.call(FORM_OF, &args![station_reference]).u32();
        let mut station = fn_00832830(e, form);
        if station != 0 {
            let _age = audio_time(e).wrapping_sub(e.mem.u32(station + 0xc));
        } else {
            station = fn_00832cb0(e, radio, station_reference);
            if station != 0 && e.mem.u32(station) == 0 {
                e.call(IMPORT_00FDF0C8, &[]);
            }
            if station == 0 {
                return;
            }
        }
        let mut created = false;
        if e.mem.u32(slot.addr()) == 0 {
            let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
            let receiver = if block != 0 {
                fn_008327d0(e, Ptr::new(block)).addr()
            } else {
                0
            };
            e.mem.set_u32(slot.addr(), receiver);
            created = true;
        }
        let receiver = e.mem.u32(slot.addr());
        e.mem.set_u32(receiver, radio);
        e.mem.set_u8(receiver + 0x1c, 1);
        e.mem.set_u8(receiver + 0x1d, 1);
        e.mem.set_u8(receiver + 0x1f, 1);
        if created {
            e.call(LIST_ADD_HEAD, &args![RECEIVER_LIST, slot]);
        }
        e.call(LIST_ADD_HEAD, &args![station + STATION_USERS, slot]);
        fn_00832440(e, Ptr::new(station + STATION_STATE), 1);
    });
}

// Translated from 008359e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The broadcast state setter (unnamed in the engine map; its lock is
/// named `"FalloutRadio::SetBroadcastState"`; `cdecl`: the station
/// reference and a byte): nothing while shutting down. Inside the radio's
/// critical section, the station's form gets the byte (`00511370`); a
/// station with flag `0x20` turned off has its users' voices stopped, the
/// Pip-Boy voice stopped and media 7 restarted when it is the Pip-Boy's
/// station (unless `00833ce0`), flag `0x10` cleared and its conversation
/// deleted; then the station's flag `0x20` takes the byte.
pub fn fn_008359e0(e: &mut Engine, station_reference: u32, on: u32) {
    if shutting_down(e) {
        return;
    }
    let on = on as u8;
    let form = e.call(FORM_OF, &args![station_reference]).u32();
    let station = fn_00832830(e, form);
    e.call(
        ENTER_CRITICAL_SECTION,
        &args![RADIO_CRITICAL_SECTION, NAME_SET_BROADCAST_STATE],
    );
    let form = e.call(FORM_OF, &args![station_reference]).u32();
    e.call(FORM_SET_BROADCAST_00511370, &args![form, u32::from(on)]);
    if station != 0 {
        let state = Ptr::new(station + STATION_STATE);
        if fn_00833040(e, state) && on == 0 {
            let mut node = station + STATION_USERS;
            while node != 0 && !is_empty(e, node) {
                let receiver = item_of(e, node);
                if e.call(SOUND_IS_VALID, &args![receiver + 4]).u32() & 0xff != 0 {
                    let receiver = item_of(e, node);
                    e.call(SOUND_HANDLE_STOP, &args![receiver + 4]);
                    let receiver = item_of(e, node);
                    e.call(SOUND_HANDLE_RELEASE, &args![receiver + 4]);
                }
                node = next_of(e, node);
            }
            if pipboy_station(e) == station && e.global::<u8>(PIPBOY_RADIO_ON) != 0 {
                stop_and_release(e, PIPBOY_HANDLE_5BC);
                if fn_00833ce0(e) == 0 {
                    restart_media(e);
                }
            }
            fn_008330b0(e, state, 0);
            let next = e.mem.u32(station + 4);
            delete_conversation(e, next);
            e.mem.set_u32(station + 4, 0);
        }
        fn_00833100(e, state, on);
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![RADIO_CRITICAL_SECTION]);
}

// Translated from 00835be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::StartRadioConversation` (engine map; `cdecl`: the station
/// reference and a topic, 0 for topic 7 of `0061a2d0`): nothing while
/// shutting down, or when the station (found in the station list until a
/// null entry, else made by `00832cb0`) is missing or its reference passes
/// `00440da0`. Inside the radio's critical section: the conversation is run
/// to its end (`NextItem`); a station with flag `0x10` is updated
/// (`00834260`) with flag `0x20` cleared and duration 1; the Pip-Boy voice
/// stopped (media 7 restarted when `00833ce0` is set) for the Pip-Boy's
/// station; the users' voices stopped; the conversation started on the
/// topic (`00837060`) with duration 0, its sounds precached and its first
/// item made current; without a current response it is deleted (duration
/// 1), otherwise the Pip-Boy voice is handled again and the first response
/// started; a duration over 1 becomes 0; the item restarts in 50
/// (`00835690` with 1) and flag `0x20` gets its old value back.
pub fn fallout_radio_start_radio_conversation(e: &mut Engine, station_reference: u32, topic: u32) {
    if shutting_down(e) {
        return;
    }
    let mut station = 0u32;
    let mut node = STATION_LIST;
    while node != 0 && item_of(e, node) != 0 {
        let entry = item_of(e, node);
        if e.mem.u32(entry) == station_reference {
            station = item_of(e, node);
            break;
        }
        node = next_of(e, node);
    }
    if station == 0 {
        station = fn_00832cb0(e, station_reference, 0);
    }
    let topic = if topic != 0 {
        topic
    } else {
        e.call(GET_TOPIC, &args![7u32, 0u32]).u32()
    };
    if station == 0 {
        return;
    }
    let reference = e.mem.u32(station);
    if e.call(REFERENCE_TEST_00440DA0, &args![reference]).u32() & 0xff != 0 {
        return;
    }
    let state = Ptr::new(station + STATION_STATE);
    let old = u8::from(fn_00833040(e, state));
    e.call(
        ENTER_CRITICAL_SECTION,
        &args![RADIO_CRITICAL_SECTION, NAME_START_RADIO_CONVERSATION],
    );
    if e.mem.u32(station + 4) != 0 {
        loop {
            let conversation = e.mem.u32(station + 4);
            if e.call(NEXT_ITEM, &args![conversation]).u32() & 0xff == 0 {
                break;
            }
        }
    }
    if fn_00833be0(e, state) {
        fn_00833100(e, state, 0);
        e.mem.set_u32(station + 0x10, 1);
        fn_00834260(e, station);
    }
    let pipboy_voice = |e: &mut Engine| {
        if pipboy_station(e) == station && e.global::<u8>(PIPBOY_RADIO_ON) != 0 {
            stop_and_release(e, PIPBOY_HANDLE_5BC);
            if fn_00833ce0(e) != 0 {
                restart_media(e);
            }
        }
    };
    pipboy_voice(e);
    let mut node = station + STATION_USERS;
    while node != 0 && item_of(e, node) != 0 {
        let receiver = item_of(e, node);
        e.call(SOUND_HANDLE_STOP, &args![receiver + 4]);
        e.call(SOUND_HANDLE_RELEASE, &args![receiver + 4]);
        node = next_of(e, node);
    }
    fn_00837060(e, station, topic);
    e.mem.set_u32(station + 0x10, 0);
    let conversation = e.mem.u32(station + 4);
    e.call(PRECACHE_SOUNDS, &args![conversation]);
    let conversation = e.mem.u32(station + 4);
    e.call(FIRST_ITEM, &args![conversation]);
    let conversation = e.mem.u32(station + 4);
    if current_response(e, conversation) == 0 {
        let conversation = e.mem.u32(station + 4);
        delete_conversation(e, conversation);
        e.mem.set_u32(station + 4, 0);
        e.mem.set_u32(station + 0x10, 1);
    } else {
        pipboy_voice(e);
        let conversation = e.mem.u32(station + 4);
        let response = current_response(e, conversation);
        e.call(FIRST_RESPONSE, &args![response]);
    }
    if e.mem.u32(station + 0x10) as i32 > 1 {
        e.mem.set_u32(station + 0x10, 0);
    }
    let start = audio_time(e).wrapping_add(0x32);
    e.mem.set_u32(station + 0xc, start);
    fn_00835690(e, state, 1);
    fn_00833100(e, state, old);
    e.call(LEAVE_CRITICAL_SECTION, &args![RADIO_CRITICAL_SECTION]);
}

/// A form id read from the buffer (`008648a0`) resolved to a
/// `TESBoundObject` (`004839c0`, `__RTDynamicCast` from `TESForm`).
fn read_bound_object(e: &mut Engine, buffer: u32) -> u32 {
    let id = e.call(BUFFER_READ_FORM, &args![buffer]).u32();
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    e.call(
        RTTI_CAST,
        &args![form, 0u32, RTTI_TES_FORM, RTTI_TES_BOUND_OBJECT, 0u32],
    )
    .u32()
}

// Translated from 008368b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008368b0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `FalloutRadio::LoadGame` (engine map; `cdecl`, one word: the load
/// buffer): the saved word (4 bytes), the Pip-Boy radio byte and the saved
/// string; the buffer's count of receivers (0x20 bytes each, `008327d0`,
/// loaded by `00835f80`) and of stations (0x24 bytes, `00833150`, loaded by
/// `00836630`), each added at the head of its list; the current and lost
/// Pip-Boy stations as `TESBoundObject`s (fixed up later); and from version
/// 10 on (the buffer's slot `+0`), the discovered stations' form ids.
pub fn fallout_radio_load_game(e: &mut Engine, buffer: u32) {
    e.call(BUFFER_READ, &args![buffer, SAVED_WORD, 4u32]);
    e.call(BUFFER_READ, &args![buffer, SAVED_BYTE, 1u32]);
    e.call(BUFFER_READ_STRING, &args![buffer, SAVED_STRING]);
    e.with_stack(4, |e, slot| {
        let receivers = e.call(BUFFER_COUNT, &args![buffer]).u32();
        for _ in 0..receivers {
            let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
            let receiver = if block != 0 {
                fn_008327d0(e, Ptr::new(block)).addr()
            } else {
                0
            };
            e.mem.set_u32(slot.addr(), receiver);
            fn_00835f80(e, Ptr::new(receiver), buffer);
            e.call(LIST_ADD_HEAD, &args![RECEIVER_LIST, slot]);
        }
        let stations = e.call(BUFFER_COUNT, &args![buffer]).u32();
        for _ in 0..stations {
            let block = e.call(OPERATOR_NEW, &args![0x24u32]).u32();
            let station = if block != 0 {
                fn_00833150(e, Ptr::new(block)).addr()
            } else {
                0
            };
            e.mem.set_u32(slot.addr(), station);
            fn_00836630(e, Ptr::new(station), buffer);
            e.call(LIST_ADD_HEAD, &args![STATION_LIST, slot]);
        }
    });
    let current = read_bound_object(e, buffer);
    e.set_global(CURRENT_PIPBOY_STATION, current);
    let lost = read_bound_object(e, buffer);
    e.set_global(LOST_PIPBOY_STATION, lost);
    if e.vcall(buffer, 0, &[]).u32() & 0xff >= 10 {
        let count = e.call(BUFFER_COUNT, &args![buffer]).u32();
        e.with_stack(4, |e, slot| {
            for _ in 0..count {
                e.mem.set_u32(slot.addr(), 0);
                e.call(BUFFER_READ_FORM_ID, &args![buffer, slot]);
                e.call(LIST_ADD_HEAD, &args![DISCOVERED_LIST, slot]);
            }
        });
    }
}

/// The pruning walk of `InitLoadGame` and `FinishLoadGame`: `keep` decides
/// for each entry (until an empty node) whether it stays; a dropped one is
/// removed after the previous node (`00905330`) or as the head
/// (`0063f7b0`).
fn prune(e: &mut Engine, list: u32, keep: &mut dyn FnMut(&mut Engine, u32) -> bool) {
    let mut node = list;
    let mut previous = 0u32;
    while node != 0 && !is_empty(e, node) {
        if keep(e, node) {
            previous = node;
            node = next_of(e, node);
        } else if previous != 0 {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            e.call(LIST_REMOVE, &args![previous, slot]);
            node = next_of(e, previous);
        } else {
            e.call(LIST_REMOVE_HEAD_0063F7B0, &args![node]);
        }
    }
}

// Translated from 00836af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::InitLoadGame` (engine map; `cdecl`, one word: the load
/// buffer): every receiver's radio resolved (`00835fe0`); stations fixed up
/// (`00836660`) and those without a reference deleted (`00832160` with 1)
/// and dropped, null ones dropped; receivers without a radio deleted
/// (`00832190` with 1) and dropped; the current and lost Pip-Boy stations
/// looked up from their objects (`00832830`); and from version 10 on the
/// discovered stations resolved to references (null ones dropped, the rest
/// stored back with `00726c60`).
pub fn fallout_radio_init_load_game(e: &mut Engine, buffer: u32) {
    let mut node = RECEIVER_LIST;
    while node != 0 {
        let receiver = item_of(e, node);
        if receiver != 0 {
            fn_00835fe0(e, Ptr::new(receiver), buffer);
        }
        node = next_of(e, node);
    }
    prune(e, STATION_LIST, &mut |e, node| {
        let station = item_of(e, node);
        if station == 0 {
            return false;
        }
        fn_00836660(e, Ptr::new(station), buffer);
        if e.mem.u32(station) == 0 {
            fn_00832160(e, Ptr::new(station), 1);
            return false;
        }
        true
    });
    prune(e, RECEIVER_LIST, &mut |e, node| {
        let receiver = item_of(e, node);
        if receiver == 0 {
            return false;
        }
        if e.mem.u32(receiver) == 0 {
            fn_00832190(e, Ptr::new(receiver), 1);
            return false;
        }
        true
    });
    if e.global::<u32>(CURRENT_PIPBOY_STATION) != 0 {
        let object = e.global::<u32>(CURRENT_PIPBOY_STATION);
        let station = fn_00832830(e, object);
        e.set_global(CURRENT_PIPBOY_STATION, station);
    }
    if e.global::<u32>(LOST_PIPBOY_STATION) != 0 {
        let object = e.global::<u32>(LOST_PIPBOY_STATION);
        let station = fn_00832830(e, object);
        e.set_global(LOST_PIPBOY_STATION, station);
    }
    if e.vcall(buffer, 0, &[]).u32() & 0xff >= 10 {
        e.with_stack(4, |e, slot| {
            prune(e, DISCOVERED_LIST, &mut |e, node| {
                let id = item_of(e, node);
                e.mem.set_u32(slot.addr(), id);
                if id == 0 {
                    return false;
                }
                resolve_reference(e, slot.addr());
                if e.mem.u32(slot.addr()) == 0 {
                    return false;
                }
                e.call(NODE_SET_ITEM_00726C60, &args![node, slot]);
                true
            });
        });
    }
}

// Translated from 00836e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::FinishLoadGame` (engine map; `cdecl`, one word): each
/// station's load finish (`008366c0`), null entries dropped; the Pip-Boy
/// radio set to its loaded byte (`PipboyRadioEnable`), and with a Pip-Boy
/// station the Pip-Boy updated (`PipboyUpdate` with 1).
pub fn fallout_radio_finish_load_game(e: &mut Engine, value: u32) {
    prune(e, STATION_LIST, &mut |e, node| {
        let station = item_of(e, node);
        if station == 0 {
            return false;
        }
        fn_008366c0(e, Ptr::new(station), value);
        true
    });
    let on = e.global::<u8>(PIPBOY_RADIO_ON);
    fallout_radio_pipboy_radio_enable(e, u32::from(on));
    if e.global::<u32>(CURRENT_PIPBOY_STATION) != 0 {
        fallout_radio_pipboy_update(e, 1);
    }
}

// Translated from 00836ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::Revert` (engine map; `cdecl`): no current or lost
/// station, last update 1, saved word 0, radio off, the discovery message
/// shown again, empty saved string; every station deleted (`00832160` with
/// 1) and every receiver (its valid sound stopped and released,
/// `00832190` with 1) with their list heads removed; the discovered list
/// cleared; both Pip-Boy handles stopped and released when valid.
pub fn fallout_radio_revert(e: &mut Engine) {
    e.set_global(CURRENT_PIPBOY_STATION, 0u32);
    e.set_global(LOST_PIPBOY_STATION, 0u32);
    e.set_global(LAST_UPDATE, 1u32);
    e.set_global(SAVED_WORD, 0u32);
    e.set_global(PIPBOY_RADIO_ON, 0u8);
    e.set_global(SKIP_DISCOVERY_MESSAGE, 1u8);
    e.set_global(SAVED_STRING, 0u8);
    while !is_empty(e, STATION_LIST) {
        let station = item_of(e, STATION_LIST);
        if station != 0 {
            fn_00832160(e, Ptr::new(station), 1);
        }
        e.call(LIST_REMOVE_HEAD_0063F7B0, &args![STATION_LIST]);
    }
    while !is_empty(e, RECEIVER_LIST) {
        let receiver = item_of(e, RECEIVER_LIST);
        if e.call(SOUND_IS_VALID, &args![receiver + 4]).u32() & 0xff != 0 {
            e.call(SOUND_HANDLE_STOP, &args![receiver + 4]);
            e.call(SOUND_HANDLE_RELEASE, &args![receiver + 4]);
        }
        if receiver != 0 {
            fn_00832190(e, Ptr::new(receiver), 1);
        }
        e.call(LIST_REMOVE_HEAD_0063F7B0, &args![RECEIVER_LIST]);
    }
    e.call(LIST_CLEAR, &args![DISCOVERED_LIST]);
    for handle in [PIPBOY_HANDLE_5BC, PIPBOY_HANDLE_638] {
        if e.call(SOUND_IS_VALID, &args![handle]).u32() & 0xff != 0 {
            stop_and_release(e, handle);
        }
    }
}

// Translated from 008372e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008372e0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `RadioConvTask::Run` (engine map; `thiscall`, plain `RET`): a station
/// (`+0x30`) without a next conversation (`+8`) gets a new one (0xc bytes;
/// `Conversation::Conversation_ov2` with the station's reference twice and
/// topic 7 of `0061a2d0`; null when the allocation fails).
pub fn radio_conv_task_run(e: &mut Engine, this: Ptr) {
    let station = e.mem.u32(this.addr() + 0x30);
    if station == 0 || e.mem.u32(station + 8) != 0 {
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let conversation = if block != 0 {
        let topic = e.call(GET_TOPIC, &args![7u32, 0u32]).u32();
        let station = e.mem.u32(this.addr() + 0x30);
        let reference = e.mem.u32(station);
        let station = e.mem.u32(this.addr() + 0x30);
        let speaker = e.mem.u32(station);
        e.call(
            CONVERSATION_CONSTRUCT,
            &args![block, speaker, reference, topic],
        )
        .u32()
    } else {
        0
    };
    let station = e.mem.u32(this.addr() + 0x30);
    e.mem.set_u32(station + 8, conversation);
}

#[cfg(test)]
mod n086b_tests {
    use super::*;

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
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    fn lists(e: &mut Engine) {
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_REMOVE_HEAD_0063F7B0, |e, a| {
            // Take the next node's words into the head.
            let next = e.mem.u32(a[0] + 4);
            let (item, after) = if next != 0 {
                (e.mem.u32(next), e.mem.u32(next + 4))
            } else {
                (0, 0)
            };
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.register(LIST_REMOVE, |e, a| {
            let after = e.mem.u32(a[1] + 4);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
    }

    fn globals(e: &mut Engine) {
        for page in [
            0x011d_d000u32,
            0x011d_c000,
            0x011d_e000,
            0x011a_1000,
            0x011d_9000,
            0x011d_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_f000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_u64(HUNDRED_DOUBLE, 100.0f64.to_bits());
    }

    fn fill(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, &item) in items.iter().enumerate() {
            e.mem.set_u32(node, item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            } else {
                e.mem.set_u32(node + 4, 0);
            }
        }
    }

    fn sounds(e: &mut Engine) {
        quiet(
            e,
            &[
                SOUND_HANDLE_STOP,
                SOUND_HANDLE_RELEASE,
                SOUND_HANDLE_PLAY,
                SOUND_HANDLE_ASSIGN,
                SOUND_HANDLE_DESTRUCTOR,
                SOUND_SET_VOLUME,
                SOUND_SET_COMPLETION,
                SOUND_SET_SYNCH_START,
                SOUND_SET_FLAG_004F15A0,
            ],
        );
        e.register(AUDIO_INSTANCE, |_, _| 0xa0d10u32.into_ret());
        e.register(AUDIO_SOUND_BY_NAME, |_, a| a[1].into_ret());
        e.register(AUDIO_SOUND_BY_FILE, |_, a| a[1].into_ret());
        e.register(SOUND_IS_VALID, |_, _| 0u32.into_ret());
    }

    /// A station (0x24 bytes) with `reference` and the state flags.
    fn station(e: &mut Engine, reference: u32, flags: u32) -> u32 {
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, reference);
        e.mem.set_u32(station + 4 + 0x14, flags);
        station
    }

    #[test]
    fn pipboy_update_signal_lost_shows_the_message() {
        let mut e = Engine::new();
        globals(&mut e);
        sounds(&mut e);
        quiet(
            &mut e,
            &[
                STRING_CONSTRUCT,
                STRING_DESTROY,
                STRING_FORMAT,
                HUD_SHOW_00775380,
                MEDIA_IS_PLAYING_00830AD0,
            ],
        );
        e.register(REFERENCE_NAME, |_, _| 0x4e4eu32.into_ret());
        e.register(SETTING_TEXT_00403DF0, |_, _| 0xf0f0u32.into_ret());
        e.register(STRING_C_STR, |_, _| 0x7e7eu32.into_ret());
        let st = station(&mut e, 0x3030, 4);
        e.set_global(CURRENT_PIPBOY_STATION, st);
        e.set_global(PIPBOY_RADIO_ON, 1u8);
        e.mem.set_f32(MESSAGE_TIME, 2.0);
        e.call_log = Some(vec![]);
        e.call(0x0083_31c0, &args![0u32]);
        assert_eq!(
            calls(&e, AUDIO_SOUND_BY_NAME)[0][2..],
            [SOUND_SIGNAL_LOST, 0x0010_0121]
        );
        assert_eq!(calls(&e, STRING_FORMAT)[0][1..], [0xf0f0, 0x4e4e]);
        assert_eq!(e.global::<u32>(LOST_PIPBOY_STATION), st);
        assert_eq!(e.global::<u8>(PIPBOY_RADIO_ON), 0);
        assert_eq!(e.global::<u32>(CURRENT_PIPBOY_STATION), 0);
        let hud = calls(&e, HUD_SHOW_00775380);
        assert_eq!(
            hud[0][1..],
            [0x7e7e, 0, DISCOVERY_TEXT, 0, 2.0f32.to_bits(), 0]
        );
        assert!(calls(&e, SOUND_SET_VOLUME).is_empty());
    }

    #[test]
    fn pipboy_update_plays_static_and_news() {
        let mut e = Engine::new();
        globals(&mut e);
        sounds(&mut e);
        lists(&mut e);
        quiet(
            &mut e,
            &[
                FIRST_RESPONSE,
                MEDIA_PLAY_008300C0,
                COPY_BOUNDED,
                FORMAT_00406D00,
                STRING_FROM_TEXT,
                STRING_DESTROY,
            ],
        );
        e.register(VOLUME_CURVE, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.register(MEDIA_PLAYING_00830750, |_, _| 0u32.into_ret());
        e.register(MEDIA_DURATION, |_, _| 900u32.into_ret());
        e.register(AUDIO_TIME, |_, _| 10u32.into_ret());
        e.register(AUDIO_VOLUME_00AD7AE0, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        // The conversation's item has a response with a form (news) and
        // no text.
        e.register(CURRENT_RESPONSE, |_, a| (a[0] + 1).into_ret());
        e.register(FORM_OF, |_, a| (a[0] + 0x100).into_ret());
        e.register(STRING_C_STR, |_, _| 0u32.into_ret());
        e.register(SOUND_FILE_NAME, |_, _| 0x5f5fu32.into_ret());
        let st = station(&mut e, 0x3030, 0);
        e.mem.set_u8(st + 4 + 0x10, 50);
        e.mem.set_u32(st + 4, 0x4000);
        e.set_global(CURRENT_PIPBOY_STATION, st);
        e.call_log = Some(vec![]);
        e.call(0x0083_31c0, &args![0u32]);
        // The static: started (not valid) at the curve volume.
        assert_eq!(
            calls(&e, AUDIO_SOUND_BY_NAME)[0][2..],
            [STATIC_SOUND, 0x0010_0031]
        );
        assert_eq!(
            calls(&e, SOUND_SET_VOLUME)[0],
            vec![PIPBOY_HANDLE_638, 0.5f32.to_bits()]
        );
        // News: media 7 from the sound path, its duration taken; flag 8 set.
        let media = calls(&e, MEDIA_PLAY_008300C0);
        assert_eq!(media[0][0], 7);
        assert_eq!(media[0][2..], [0x1f4, 0, 1, 0, 0]);
        let format = calls(&e, FORMAT_00406D00);
        assert_eq!(
            format[0][1..],
            [0x104, TEXT_TWO_STRINGS, TEXT_SOUND_PATH, 0x5f5f]
        );
        assert_eq!(e.mem.u32(st + 0x10), 900);
        assert_eq!(e.mem.u32(st + 4 + 0x14) & 8, 8);
        // The voice's volume: the audio's volume times the strength 0.5.
        assert_eq!(
            calls(&e, SOUND_SET_VOLUME).last().unwrap(),
            &vec![PIPBOY_HANDLE_5BC, 0.25f32.to_bits()]
        );
    }

    #[test]
    fn pipboy_update_plays_the_voice_static_without_a_conversation() {
        let mut e = Engine::new();
        globals(&mut e);
        sounds(&mut e);
        e.register(SOUND_TEST_00AD8930, |_, _| 0u32.into_ret());
        e.register(AUDIO_VOLUME_00AD7AE0, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        let st = station(&mut e, 0x3030, 0);
        e.mem.set_u8(st + 4 + 0x10, 100);
        e.mem.set_u32(st + 0x10, 5);
        e.set_global(CURRENT_PIPBOY_STATION, st);
        e.call_log = Some(vec![]);
        e.call(0x0083_31c0, &args![0u32]);
        assert_eq!(
            calls(&e, SOUND_SET_FLAG_004F15A0),
            vec![vec![PIPBOY_HANDLE_5BC, 1]]
        );
        assert_eq!(
            calls(&e, SOUND_HANDLE_PLAY),
            vec![vec![PIPBOY_HANDLE_5BC, 0]]
        );
        assert_eq!(e.mem.u32(st + 4 + 0x14), 0x40);
        assert_eq!(
            calls(&e, SOUND_SET_VOLUME),
            vec![vec![PIPBOY_HANDLE_5BC, 1.0f32.to_bits()]]
        );
    }

    #[test]
    fn enable_npc_radio_adds_a_receiver() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        quiet(&mut e, &[LIST_ADD_HEAD, SOUND_HANDLE_CONSTRUCT]);
        e.register(FORM_OF, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(REFERENCE_TEST_00440DA0, |_, _| 0u32.into_ret());
        e.register(REFERENCE_VALUE_0056B210, |_, _| 0u32.into_ret());
        e.register(AUDIO_INSTANCE, |_, _| 0u32.into_ret());
        e.register(AUDIO_TIME, |_, _| 10u32.into_ret());
        let reference = e.mem.alloc(0x30);
        e.mem.set_u32(reference + 0x20, 0x55);
        let st = station(&mut e, reference, 0);
        fill(&mut e, STATION_LIST, &[st]);
        let block = e.mem.alloc(0x20);
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0083_5810, &args![0x7777u32, reference]);
        assert_eq!(e.mem.u32(block), 0x7777);
        assert_eq!(e.mem.u8(block + 0x1c), 1);
        assert_eq!(e.mem.u8(block + 0x1f), 1);
        let adds = calls(&e, LIST_ADD_HEAD);
        assert_eq!(adds.len(), 2);
        assert_eq!(adds[0][0], RECEIVER_LIST);
        assert_eq!(adds[1][0], st + STATION_USERS);
        assert_eq!(e.mem.u32(st + 4 + 0x14) & 1, 1);
        // Shutting down: nothing.
        e.set_global(SHUTTING_DOWN, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x0083_5810, &args![0x7777u32, reference]);
        assert!(calls(&e, LIST_ADD_HEAD).is_empty());
    }

    #[test]
    fn broadcast_state_off_stops_the_users() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        sounds(&mut e);
        quiet(
            &mut e,
            &[
                ENTER_CRITICAL_SECTION,
                LEAVE_CRITICAL_SECTION,
                FORM_SET_BROADCAST_00511370,
                CONVERSATION_DELETE,
                MEDIA_CLOSE,
                MEDIA_PLAY_008300C0,
                MEDIA_CALL_008304C0,
            ],
        );
        e.register(SOUND_IS_VALID, |_, _| 1u32.into_ret());
        e.register(FORM_OF, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(REFERENCE_TEST_00440DA0, |_, _| 0u32.into_ret());
        e.register(REFERENCE_VALUE_0056B210, |_, _| 0u32.into_ret());
        let reference = e.mem.alloc(0x30);
        e.mem.set_u32(reference + 0x20, 0x55);
        let st = station(&mut e, reference, 0x32);
        e.mem.set_u32(st + 4, 0x0c0c);
        let receiver = e.mem.alloc(0x20);
        fill(&mut e, st + STATION_USERS, &[receiver]);
        fill(&mut e, STATION_LIST, &[st]);
        e.set_global(CURRENT_PIPBOY_STATION, st);
        e.set_global(PIPBOY_RADIO_ON, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x0083_59e0, &args![reference, 0u32]);
        assert_eq!(calls(&e, FORM_SET_BROADCAST_00511370), vec![vec![0x55, 0]]);
        assert_eq!(
            calls(&e, SOUND_HANDLE_STOP),
            vec![vec![receiver + 4], vec![PIPBOY_HANDLE_5BC]]
        );
        assert_eq!(calls(&e, MEDIA_PLAY_008300C0).len(), 1);
        assert_eq!(calls(&e, CONVERSATION_DELETE), vec![vec![0x0c0c, 1]]);
        assert_eq!(e.mem.u32(st + 4), 0);
        // Flags 0x20 and 0x10 cleared.
        assert_eq!(e.mem.u32(st + 4 + 0x14), 2);
        assert_eq!(
            calls(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![RADIO_CRITICAL_SECTION]]
        );
        // Turning it on sets flag 0x20 again.
        e.call_log = Some(vec![]);
        e.call(0x0083_59e0, &args![reference, 1u32]);
        assert_eq!(e.mem.u32(st + 4 + 0x14), 0x22);
        assert!(calls(&e, SOUND_HANDLE_STOP).is_empty());
    }

    #[test]
    fn start_radio_conversation_starts_the_first_response() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        sounds(&mut e);
        quiet(
            &mut e,
            &[
                ENTER_CRITICAL_SECTION,
                LEAVE_CRITICAL_SECTION,
                PRECACHE_SOUNDS,
                FIRST_ITEM,
                FIRST_RESPONSE,
                CONVERSATION_DELETE,
            ],
        );
        e.register(REFERENCE_TEST_00440DA0, |_, _| 0u32.into_ret());
        e.register(NEXT_ITEM, |_, _| 0u32.into_ret());
        e.register(AUDIO_TIME, |_, _| 1000u32.into_ret());
        e.register(CURRENT_RESPONSE, |_, a| (a[0] + 1).into_ret());
        let reference = e.mem.alloc(0x30);
        let st = station(&mut e, reference, 0x20);
        // 00837060 (this file) replaces the conversation: 0x6000 again.
        e.mem.set_u32(st + 4, 0x6000);
        fill(&mut e, STATION_LIST, &[st]);
        e.register(OPERATOR_NEW, |_, _| 0x5000u32.into_ret());
        e.register(CONVERSATION_CONSTRUCT, |_, _| 0x6000u32.into_ret());
        e.call_log = Some(vec![]);
        fallout_radio_start_radio_conversation(&mut e, reference, 0x8080);
        assert_eq!(calls(&e, NEXT_ITEM), vec![vec![0x6000]]);
        assert_eq!(calls(&e, FIRST_RESPONSE), vec![vec![0x6001]]);
        assert_eq!(e.mem.u32(st + 0xc), 1050);
        // Flag 0x80 set, flag 0x20 kept.
        assert_eq!(e.mem.u32(st + 4 + 0x14) & 0xa0, 0xa0);
        assert_eq!(e.mem.u32(st + 0x10), 0);
        // Its reference passing 00440da0 stops it before the lock.
        e.register(REFERENCE_TEST_00440DA0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0083_5be0, &args![reference, 0x8080u32]);
        assert!(calls(&e, ENTER_CRITICAL_SECTION).is_empty());
    }

    #[test]
    fn load_game_reads_receivers_stations_and_pipboy_stations() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        quiet(
            &mut e,
            &[
                BUFFER_READ,
                BUFFER_READ_STRING,
                BUFFER_READ_FORM_ID,
                LIST_ADD_HEAD,
                SOUND_HANDLE_CONSTRUCT,
                LIST_CONSTRUCTOR,
                CONVERSATION_LOAD_GAME_OV2,
            ],
        );
        e.register(BUFFER_COUNT, |_, _| 1u32.into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BUFFER_READ_FORM, |_, _| 0x99u32.into_ret());
        e.register(LOOKUP_FORM, |_, a| (a[0] + 1).into_ret());
        e.register(RTTI_CAST, |_, a| (a[0] + 2).into_ret());
        let buffer = e.mem.alloc(4);
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(buffer, vtable);
        e.mem.set_u32(vtable, 0x7700_0000);
        e.register(0x7700_0000, |_, _| 10u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0083_68b0, &args![buffer]);
        let reads = calls(&e, BUFFER_READ);
        assert_eq!(reads[0], vec![buffer, SAVED_WORD, 4]);
        assert_eq!(reads[1], vec![buffer, SAVED_BYTE, 1]);
        let adds = calls(&e, LIST_ADD_HEAD);
        assert_eq!(
            adds.iter().map(|a| a[0]).collect::<Vec<_>>(),
            vec![RECEIVER_LIST, STATION_LIST, DISCOVERED_LIST]
        );
        assert_eq!(e.global::<u32>(CURRENT_PIPBOY_STATION), 0x9c);
        assert_eq!(e.global::<u32>(LOST_PIPBOY_STATION), 0x9c);
        assert_eq!(
            calls(&e, RTTI_CAST)[0],
            vec![0x9a, 0, RTTI_TES_FORM, RTTI_TES_BOUND_OBJECT, 0]
        );
        // An older save has no discovered list.
        e.register(0x7700_0000, |_, _| 9u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0083_68b0, &args![buffer]);
        assert_eq!(calls(&e, LIST_ADD_HEAD).len(), 2);
    }

    #[test]
    fn init_load_game_resolves_and_prunes() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        quiet(
            &mut e,
            &[
                CONVERSATION_0083C420,
                NODE_SET_ITEM_00726C60,
                OPERATOR_DELETE,
                SOUND_HANDLE_DESTRUCTOR,
                LIST_DESTRUCTOR,
            ],
        );
        e.register(LOOKUP_FORM, |_, a| a[0].into_ret());
        e.register(RTTI_CAST, |_, a| a[0].into_ret());
        e.register(FORM_OF, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(REFERENCE_TEST_00440DA0, |_, _| 0u32.into_ret());
        e.register(REFERENCE_VALUE_0056B210, |_, _| 0u32.into_ret());
        // A receiver with a radio id, and one without.
        let kept = e.mem.alloc(0x20);
        e.mem.set_u32(kept, 0x0123);
        let lost = e.mem.alloc(0x20);
        fill(&mut e, RECEIVER_LIST, &[kept, lost]);
        // A station without a reference id: deleted.
        let empty_station = e.mem.alloc(0x24);
        fill(&mut e, STATION_LIST, &[empty_station]);
        let buffer = e.mem.alloc(4);
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(buffer, vtable);
        e.mem.set_u32(vtable, 0x7700_0000);
        e.register(0x7700_0000, |_, _| 9u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0083_6af0, &args![buffer]);
        assert_eq!(e.mem.u32(kept), 0x0123);
        // The deleting destructors are this file's: they ran for real on
        // the removed entries, which leaves the lists with the kept one.
        assert_eq!(e.mem.u32(RECEIVER_LIST), kept);
        assert_eq!(e.mem.u32(STATION_LIST), 0);
    }

    #[test]
    fn finish_load_game_and_revert() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        sounds(&mut e);
        quiet(
            &mut e,
            &[
                CONVERSATION_DELETE,
                MEDIA_CLOSE,
                MEDIA_IS_PLAYING_00830AD0,
                LIST_CLEAR,
                OPERATOR_DELETE,
                SOUND_HANDLE_DESTRUCTOR,
                LIST_DESTRUCTOR,
            ],
        );
        e.register(AUDIO_TIME, |_, _| 10u32.into_ret());
        let st = station(&mut e, 0x3030, 0);
        e.mem.set_u32(st + 0xc, 4);
        e.mem.set_u32(st + 0x10, 1);
        fill(&mut e, STATION_LIST, &[st, 0]);
        e.call_log = Some(vec![]);
        e.call(0x0083_6e20, &args![0u32]);
        // The station's item rebased to 10 - 4; the radio stays off.
        assert_eq!(e.mem.u32(st + 0xc), 6);
        assert_eq!(e.global::<u8>(PIPBOY_RADIO_ON), 0);

        // Revert: statics reset, every list emptied.
        e.set_global(CURRENT_PIPBOY_STATION, st);
        e.set_global(SAVED_WORD, 5u32);
        e.register(SOUND_IS_VALID, |_, _| 1u32.into_ret());
        let receiver = e.mem.alloc(0x20);
        fill(&mut e, RECEIVER_LIST, &[receiver]);
        e.call_log = Some(vec![]);
        e.call(0x0083_6ef0, &[]);
        assert_eq!(e.global::<u32>(CURRENT_PIPBOY_STATION), 0);
        assert_eq!(e.global::<u32>(LAST_UPDATE), 1);
        assert_eq!(e.global::<u32>(SAVED_WORD), 0);
        assert_eq!(e.global::<u8>(SKIP_DISCOVERY_MESSAGE), 1);
        assert_eq!(e.mem.u32(STATION_LIST), 0);
        assert_eq!(e.mem.u32(RECEIVER_LIST), 0);
        assert_eq!(calls(&e, LIST_CLEAR), vec![vec![DISCOVERED_LIST]]);
        assert_eq!(
            calls(&e, SOUND_HANDLE_STOP),
            vec![
                vec![receiver + 4],
                vec![PIPBOY_HANDLE_5BC],
                vec![PIPBOY_HANDLE_638]
            ]
        );
    }

    #[test]
    fn radio_conv_task_run_makes_the_next_conversation() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0xc);
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        e.register(GET_TOPIC, |_, _| 0x7070u32.into_ret());
        e.register(CONVERSATION_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x34);
        let st = station(&mut e, 0x3030, 0);
        e.mem.set_u32(this + 0x30, st);
        e.call_log = Some(vec![]);
        e.call(0x0083_72e0, &args![this]);
        assert_eq!(e.mem.u32(st + 8), block);
        assert_eq!(
            calls(&e, CONVERSATION_CONSTRUCT),
            vec![vec![block, 0x3030, 0x3030, 0x7070]]
        );
        assert_eq!(calls(&e, GET_TOPIC), vec![vec![7, 0]]);
        // A next conversation already there: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0083_72e0, &args![this]);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
    }
}
