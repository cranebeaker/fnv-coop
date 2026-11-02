//! `BSAudio/bsaudiomanager.cpp` (Xbox PDB source unit), subsystem `BSAudio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! # Where the unit stands
//!
//! The unit has 147 functions. Session 1 translated the first 60 open
//! functions in address order, `00ad9060` to `00adcd00`: the singleton
//! getter, constructor, destructor, initialization and shutdown, the audio
//! thread control, `Update`, `SynchVoiceStates`, `SetVoiceState`, the message
//! pool (`FreeMessage`, `FreeQueueMessage`), the message queuing function
//! `00adb1a0` and the thin wrappers around it (`Play`, `Stop`, `SetVolume`,
//! ...), and the sound-state queries (`IsValid`, `IsPlaying`, ...). The next
//! session continues at `00adcd80`.
//!
//! Session 2 translated the next 60 not-done functions in address order,
//! `00adcd80` to `00ae8550`: the cache upkeep and lookup, the precache
//! functions, `Process` and `ProcessMessages`, the sound update (`00adddd0`,
//! `00adea90`), the whole of `ProcessMessage` (`00adf080`, one `message_*`
//! helper per case; kinds are the Xbox PDB `SOUND_MSG` values), the sound-handle
//! getters, `GetDebugString`, the cache and pause-table settings, `AudioLoadTask`,
//! the audio thread object, the message stack, the pointer-map constructors,
//! the moving-reference map (`SetAt`, `GetAt`, `GetNext`) and the list helpers.
//! The next session continues at `00ae8650`. The constants of this session are
//! in one block before the first `layout!`; the bits of a sound's state and type
//! flags are tested by value because their meanings are not confirmed.
//!
//! # Layout of the manager on PC
//!
//! `BSAudioManager` is a 0x188-byte singleton (the static object at
//! `011f6ef0`, built by `00ad9060`). Up to `CachedSoundList` (+0x94) its
//! offsets are the Xbox PDB's. The five critical sections are 0x18 bytes
//! here (0x28 in the PDB), so they sit at +0xac, +0xc4, +0xdc, +0xf4 and
//! +0x10c, `CacheLoadTaskList` is at +0x124 (PDB +0x174) and every later
//! field is 0x50 lower than in the PDB: `iDelayTimerDelta` at +0x130,
//! the volumes from +0x140, `iNextID` at +0x180.
//!
//! Message queuing: most public methods build a [`BSSoundMessage`] with the
//! type number, three words and (for a few) a smart pointer and three
//! floats, and hand it to `00adb1a0`, which takes a message from the
//! free-message stack, fills it and appends it to the current message list.
//! The methods that carry no extra data pass the three floats at
//! `011f426c` (a constant vector the audio code uses as its default
//! position) and a null smart pointer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleList, NiTPointerList, NiTPointerMap};
use crate::units::bsaudio::bssoundhandle::BSSoundHandle;

/// The static `BSAudioManager` object `00ad9060` hands out.
const MANAGER_INSTANCE: u32 = 0x011f_6ef0;
/// Bit 0 of this word records that [`MANAGER_INSTANCE`] has been built (the
/// compiler's guard for the function-local static).
const MANAGER_BUILT_GUARD: u32 = 0x011f_7078;
/// The function `00ad9060` registers with `atexit` (destroys the manager).
const MANAGER_EXIT_THUNK: u32 = 0x00fd_bfd0;
/// `_atexit` (CRT, cdecl).
const ATEXIT: u32 = 0x00ec_658f;
/// The `BSAudio` object the audio code reads its flags from (see
/// `units/bsaudio/bsaudio.rs`): the byte at +5 says audio is initialized,
/// the byte at +6 says the audio runs on its own thread.
const AUDIO_GLOBAL: u32 = 0x011f_6d98;
/// The game's memory manager object: `ECX` of `Allocate` / `Deallocate`.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Allocate` (platform): `(manager, size)`.
const ALLOCATE: u32 = 0x00aa_3e40;
/// `MemoryManager::Deallocate` (platform): `(manager, block)`.
const DEALLOCATE: u32 = 0x00aa_4060;
/// A free of a table (`00aa10f0`, cdecl, one argument), as the map
/// destructors do it.
const FREE_TABLE: u32 = 0x00aa_10f0;
/// The setting byte `Initialize` tests (audio enabled), `011f6da0`.
const AUDIO_ENABLED_SETTING: u32 = 0x011f_6da0;
/// The debug-information setting byte `SetVoiceState` tests
/// (`bUseAudioDebugInformation`, 1 byte).
const AUDIO_DEBUG_INFORMATION_SETTING: u32 = 0x011f_6e60;
/// The byte whose value `Shutdown` clears around its last cache flush.
const CACHE_FLUSH_FLAG: u32 = 0x011f_6ec0;
/// The three words of the default message vector.
const DEFAULT_MESSAGE_VECTOR: u32 = 0x011f_426c;
/// A double, 1000.0, by which seconds are scaled to milliseconds.
const MILLISECOND_SCALE: u32 = 0x0101_7b70;
/// The `fMasterVolumeMult` setting value (a float) that scales every volume.
const MASTER_VOLUME_MULT: u32 = 0x011f_6e78;
/// The TLS word at +0x2b4 of the thread block that the audio methods set to
/// 0xb while they run (see `units/bsaudio/bsaudio.rs`).
const AUDIO_TLS_CONTEXT: u32 = 0x2b4;
/// The value stored in [`AUDIO_TLS_CONTEXT`] while an audio method runs.
const AUDIO_CONTEXT_VALUE: u32 = 0xb;
/// The name `StartAudioThread` gives the audio thread (`"LibAudioUpdate"`).
const AUDIO_THREAD_NAME: u32 = 0x010a_33d8;
/// The `BSThread` processor setting word `StartAudioThread` hands to
/// `SetThreadProcessor` (`011f6ee4`).
const AUDIO_THREAD_PROCESSOR: u32 = 0x011f_6ee4;
/// The text `SetVoiceState` copies as the debug name when a sound has no
/// name (`"NotTESSound"`).
const NOT_TES_SOUND_TEXT: u32 = 0x010a_3478;
/// The double 3.4028235e+37 `SetVoiceState` compares distances against.
const HUGE_DISTANCE_LIMIT: u32 = 0x010a_3470;
/// The float written instead of a distance that exceeds
/// [`HUGE_DISTANCE_LIMIT`] (-1.0).
const HUGE_DISTANCE_REPLACEMENT: u32 = 0x0101_2054;

/// `EnterCriticalSection` import slot.
const IMPORT_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
/// `LeaveCriticalSection` import slot.
const IMPORT_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `InitializeCriticalSection` import slot.
const IMPORT_INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
/// `DeleteCriticalSection` import slot.
const IMPORT_DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// `GetTickCount` import slot.
const IMPORT_GET_TICK_COUNT: u32 = 0x00fd_f060;
/// `InterlockedIncrement` import slot.
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `InterlockedDecrement` import slot.
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `WaitForSingleObject` import slot.
const IMPORT_WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;

/// `_eh_vector_constructor_iterator_` (CRT, cdecl: block, element size,
/// count, constructor, destructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// `_eh_vector_destructor_iterator_` (CRT, cdecl: block, element size,
/// count, destructor).
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// The constructor of the `NiTPointerList<BSSoundMessage *>` elements.
const MESSAGE_LIST_CONSTRUCTOR: u32 = 0x00a1_b940;
/// The destructor of those list elements.
const MESSAGE_LIST_DESTRUCTOR: u32 = 0x00a1_bbd0;
/// `memset` (CRT, cdecl: block, byte, count).
const MEMSET: u32 = 0x00ec_61c0;
/// `strcpy_s` (CRT, cdecl: destination, size, source).
const STRCPY_S: u32 = 0x00ec_65a6;
/// `_ftol2`: truncates the float in `ST0` (the leading `f64` argument).
const FTOL2: u32 = 0x00ec_62c0;

/// Constructor of an `AudioMessageStack` (this at the stack).
const MESSAGE_STACK_CONSTRUCTOR: u32 = 0x00ae_7510;
/// Destructor of an `AudioMessageStack`: frees every message in it.
const MESSAGE_STACK_DESTRUCTOR: u32 = 0x00ae_7550;
/// `AudioMessageStack::Push` (Xbox PDB): `(stack, message)`.
const MESSAGE_STACK_PUSH: u32 = 0x00ae_75f0;
/// The pop of an `AudioMessageStack` (`00ae76b0`): the message on top, or a
/// new one when the stack is empty and bottomless.
const MESSAGE_STACK_POP: u32 = 0x00ae_76b0;
/// Constructor of the 0x4c-byte audio thread object
/// (`BSAudioManagerThread::BSAudioManagerThread`, Xbox PDB).
const THREAD_CONSTRUCTOR: u32 = 0x00ae_77d0;
/// `BSSemaphore::Signal` (Xbox PDB) on the audio thread's semaphore.
const THREAD_SEMAPHORE_SIGNAL: u32 = 0x00ae_78e0;
/// Constructor of a `BSSoundMessage` (`00ae8650`).
const MESSAGE_CONSTRUCTOR: u32 = 0x00ae_8650;
/// `BSSoundMessage::~BSSoundMessage` (Xbox PDB).
const MESSAGE_DESTRUCTOR: u32 = 0x00ae_86d0;
/// The copy-assignment of a `BSSoundMessage` (`00ae7990`).
const MESSAGE_ASSIGN: u32 = 0x00ae_7990;
/// `BSSoundInfo::BSSoundInfo` (Xbox PDB).
const SOUND_INFO_CONSTRUCTOR: u32 = 0x00ae_89f0;
/// `BSSoundInfo::BSSoundInfo` overload 2 (Xbox PDB): copy-constructs from a
/// `BSSoundInfo`.
const SOUND_INFO_COPY_CONSTRUCTOR: u32 = 0x00ae_8a40;
/// `BSSoundInfo::~BSSoundInfo` (Xbox PDB).
const SOUND_INFO_DESTRUCTOR: u32 = 0x00ae_8af0;
/// `BSSoundInfo::operator=` (Xbox PDB).
const SOUND_INFO_ASSIGN: u32 = 0x00ae_7920;
/// `BSDebugSoundInfo::BSDebugSoundInfo` (Xbox PDB).
const DEBUG_SOUND_INFO_CONSTRUCTOR: u32 = 0x00ae_8770;
/// `BSDebugSoundInfo::~BSDebugSoundInfo` (Xbox PDB).
const DEBUG_SOUND_INFO_DESTRUCTOR: u32 = 0x00ae_8830;
/// `BSDebugSoundInfo::operator=` (Xbox PDB).
const DEBUG_SOUND_INFO_ASSIGN: u32 = 0x00ae_88c0;
/// The append of a pointer, passed by address, to a message list: the
/// engine map names this `BSAudioManager::GetFromCache`, but the body
/// adds a node to the list and stores the pointer in it.
const LIST_APPEND_POINTER: u32 = 0x00ae_8bf0;
/// `NiTPointerListBase<..>::RemoveHead` (Xbox PDB name, for the message
/// lists): removes the first node and returns its pointer.
const LIST_REMOVE_HEAD: u32 = 0x00ae_7f90;
/// Removes every node of a pointer-map (`00ae91f0`).
const MAP_CLEAR: u32 = 0x00ae_91f0;
/// `NiTPointerList` destructor-like clear of a list (`00ae8370`).
const LIST_CLEAR: u32 = 0x00ae_8370;
/// Constructors of the `NiTPointerMap` instances, `(map, bucket count)`.
const MAP_CONSTRUCTOR_SOUND_LIST: u32 = 0x00ae_7b40;
const MAP_CONSTRUCTOR_SOUND_STATES: u32 = 0x00ae_7bc0;
const MAP_CONSTRUCTOR_MOVING_REFERENCES: u32 = 0x00ae_7c40;
/// The pointer-map operations the manager's four maps use; all take the map
/// as `this`.
/// `GetNext`: `(map, position slot, key out, value out)`.
const MAP_GET_NEXT: u32 = 0x00a0_c850;
/// `GetAt`: `(map, key, value out)`, true when the key is present.
const MAP_GET_VALUE: u32 = 0x00a1_e380;
/// `RemoveKey`: `(map, key)`, true when something was removed.
const MAP_REMOVE: u32 = 0x00a1_e240;
/// `SetAt`: `(map, key, value)`.
const MAP_SET_VALUE: u32 = 0x00a0_c6c0;

/// `NiPointer<..>::operator=` (`006e5cc0`): `(destination, source slot)`.
const SMART_POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// Destructor of a smart-pointer temporary (`0045cec0`).
const SMART_POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
/// `(0066b0d0)`: sets a smart pointer slot to a pointer, `(slot, pointer)`.
const SMART_POINTER_SET: u32 = 0x0066_b0d0;
/// The function (`00c45830`, cdecl, one argument) `FreeMessage` calls with
/// the pointer a message was holding.
const RELEASE_MESSAGE_POINTER: u32 = 0x00c4_5830;
/// `BSThread::Initialize` (Xbox PDB): `(thread, stack size, name, 0)`.
const THREAD_INITIALIZE: u32 = 0x00aa_6430;
/// `BSThread::SetName` (Xbox PDB).
const THREAD_SET_NAME: u32 = 0x00aa_6540;
/// `BSThread::SetThreadProcessor` (Xbox PDB).
const THREAD_SET_PROCESSOR: u32 = 0x00aa_6550;
/// `SetThreadName` (Xbox PDB, cdecl): `(thread id, name)`.
const SET_THREAD_NAME: u32 = 0x00aa_2740;
/// The call `StopAudioThread` makes on the audio thread once it has stopped
/// (`00aa64f0`, unnamed in the map).
const THREAD_FINISH: u32 = 0x00aa_64f0;
/// `BSAudio::Precache` (Xbox PDB): `(BSAudio object, sound name, flags)`.
const AUDIO_PRECACHE: u32 = 0x00ad_7fa0;
/// The manager's `Process` (`00addc00`, Xbox PDB `BSAudioManager::Process`).
const MANAGER_PROCESS: u32 = 0x00ad_dc00;
/// The manager function `Shutdown` calls with 0 while the cache-flush flag
/// is cleared (`00adcd80`, still to be translated).
const MANAGER_CACHE_FLUSH: u32 = 0x00ad_cd80;
/// Tests the sound's flag the debug fill needs (`00ae9cf0`, bool).
const SOUND_IS_EMITTER_SOUND: u32 = 0x00ae_9cf0;
/// Second test of the same condition (`00ae9d10`, bool).
const SOUND_HAS_EMITTER: u32 = 0x00ae_9d10;

// Virtual slots of a sound (`BSGameSound`, Xbox PDB vtable offsets).
/// `BSGameSound::IsPlaying`.
const SOUND_SLOT_IS_PLAYING: u32 = 0x04;
/// `BSGameSound::GetVolume`.
const SOUND_SLOT_GET_VOLUME: u32 = 0x10;
/// `BSGameSound::GetEmitterPosition`.
const SOUND_SLOT_GET_EMITTER_POSITION: u32 = 0x54;
// Fields of a sound (`BSGameSound`, Xbox PDB names, offsets match on PC).
const SOUND_ID: u32 = 0x04;
const SOUND_TYPE_FLAGS: u32 = 0x08;
const SOUND_DURATION: u32 = 0x14;
const SOUND_STATIC_ATTENUATION: u32 = 0x18;
const SOUND_SYSTEM_ATTENUATION: u32 = 0x1c;
const SOUND_DISTANCE_ATTENUATION: u32 = 0x1e;
const SOUND_FILENAME: u32 = 0x36;
const SOUND_MAX_DISTANCE: u32 = 0x13c;
const SOUND_MIN_DISTANCE: u32 = 0x140;
const SOUND_DISTANCE_TO_LISTENER: u32 = 0x150;

// ---- constants of the second session (cache, sound updates, messages) ----
// The bits of a sound's `iStateFlags` (+0x10) and `iSoundTypeFlags` (+0x08)
// the code below tests are written as their values; what each one means is
// not confirmed here.

/// `bEnableAudioCache:Audio` (setting, byte): the same byte as
/// [`CACHE_FLUSH_FLAG`].
const CACHE_ENABLED_SETTING: u32 = CACHE_FLUSH_FLAG;
/// `iAudioCacheSize:Audio` (setting, KB).
const AUDIO_CACHE_SIZE_SETTING: u32 = 0x011f_6ecc;
/// `iMaxSizeForCachedSound:Audio` (setting, KB).
const MAX_CACHED_SOUND_SIZE_SETTING: u32 = 0x011f_6ed8;
/// `fAudioDebugDelay:Audio` (setting, seconds).
const AUDIO_DEBUG_DELAY_SETTING: u32 = 0x011f_6e6c;
/// `fDialogueFadeDecibels:Audio` (setting).
const DIALOGUE_FADE_DECIBELS: u32 = 0x011f_6e84;
/// `fDialogueFadeSecondsIn:Audio` (setting).
const DIALOGUE_FADE_SECONDS_IN: u32 = 0x011f_6e90;
/// `fDialogueFadeSecondsOut:Audio` (setting).
const DIALOGUE_FADE_SECONDS_OUT: u32 = 0x011f_6e9c;
/// The global the audio update scales its playback speed by (a float, 1.0
/// at startup).
const TIME_MULTIPLIER: u32 = 0x011a_c3a0;
/// A double, 0.25: the playback speed is raised to [`MINIMUM_SPEED`] below
/// it.
const MINIMUM_SPEED_DOUBLE: u32 = 0x0102_90b0;
/// A float, 0.25.
const MINIMUM_SPEED: u32 = 0x0101_622c;
/// A double, 1.05: how far past its maximum distance a sound is stopped.
const RANGE_MARGIN: u32 = 0x0108_2d20;
/// A double, 0.0.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// A double, 0.01: the playback-speed difference that triggers a change.
const FREQUENCY_TOLERANCE: u32 = 0x0101_6408;
/// A double, 200.0: added to a followed object's height for sounds with type
/// bit 4.
const EMITTER_HEIGHT_OFFSET: u32 = 0x0101_79e0;
/// A float, 38.0: the first float of the fade message a lost followed object
/// gets.
const FADE_MESSAGE_X: u32 = 0x010a_3484;
/// A double, 100.0.
const PERCENT_SCALE: u32 = 0x0101_7a40;
/// A double, 2.0.
const TWO_DOUBLE: u32 = 0x0101_1590;
/// A double, 4294967296.0.
const TWO_POWER_32_DOUBLE: u32 = 0x0101_a6b8;
/// A double, 3500.0: the scale of a fade's system attenuation.
const FADE_ATTENUATION_SCALE: u32 = 0x0108_8f48;
/// A double, 0.09375: the scale of the time constraints of a sound.
const TIME_CONSTRAINT_SCALE: u32 = 0x0103_4200;
/// The global hash and tick of the sound the load task cached last.
const LAST_CACHED_HASH: u32 = 0x011f_6e0c;
const LAST_CACHED_TICK: u32 = 0x011f_6e10;
/// The global load queue object (`01202d98`), whose slot 0x48 takes a load
/// task.
const LOAD_QUEUE_GLOBAL: u32 = 0x0120_2d98;
const LOAD_QUEUE_SLOT_ADD: u32 = 0x48;
/// The global random generator (`BSRandom`) and the guard word whose bit 0
/// says it has been constructed.
const BSRANDOM_OBJECT: u32 = 0x011c_4180;
const BSRANDOM_GUARD: u32 = 0x011c_4b4c;
const BSRANDOM_CONSTRUCTOR: u32 = 0x00aa_5140;
const BSRANDOM_UNSIGNED_INT: u32 = 0x00aa_5230;

/// `_rand` (CRT, cdecl).
const RAND: u32 = 0x00ec_adb8;
/// `fabs` (CRT, cdecl, double in and out).
const FABS: u32 = 0x00ec_6cde;
/// `_sprintf` (CRT, cdecl, variable arguments).
const SPRINTF: u32 = 0x00ec_623a;
/// `strcat` (CRT, cdecl).
const STRCAT: u32 = 0x00ec_6380;
/// `strstr` (CRT, cdecl).
const STRSTR: u32 = 0x00ec_7750;
/// The debug log call (`005b5e40`, cdecl, a format and its arguments).
const DEBUG_PRINT: u32 = 0x005b_5e40;
/// A table allocation (`00aa1070`, cdecl, one argument), the counterpart of
/// [`FREE_TABLE`].
const TABLE_ALLOCATE: u32 = 0x00aa_1070;

/// The `BSAudio` object's fields (Xbox PDB names, PC offsets).
const AUDIO_ENABLED: u32 = 0x04;
const AUDIO_LISTENER: u32 = 0x10;
const AUDIO_ID_CALLBACK: u32 = 0x20;
const AUDIO_NAME_CALLBACK: u32 = 0x24;
const AUDIO_SETTINGS_CALLBACK: u32 = 0x2c;
/// The `BSAudio` virtual slots the code calls. The Xbox PDB has
/// `GetLoopPointCallback` at +0x14, `CreateNewGameSound` at +0x18 and
/// `FixSoundPath` at +0x1c; the PC calls with the same meaning use the slots
/// 4 lower (matched by what they are passed and return).
const AUDIO_SLOT_GET_LOOP_POINT_CALLBACK: u32 = 0x10;
const AUDIO_SLOT_CREATE_NEW_GAME_SOUND: u32 = 0x14;
const AUDIO_SLOT_FIX_SOUND_PATH: u32 = 0x18;
/// The listener object's slots called through `BSAudio` +0x10.
const LISTENER_SLOT_SET_POSITION: u32 = 0x08;
const LISTENER_SLOT_GET_POSITION: u32 = 0x0c;
const LISTENER_SLOT_AFTER_UPDATE: u32 = 0x10;
const LISTENER_SLOT_GET_ENVIRONMENT: u32 = 0x2c;

/// The pointer-list steps (thiscall on the list; `00a0c9f0` and `00ae8010`
/// have no name in the engine map).
/// Adds `*item` at the head `(list, &item)`.
const LIST_ADD_HEAD: u32 = 0x00a0_c9f0;
/// Removes the tail and answers its element `(list)`.
const LIST_POP_TAIL: u32 = 0x00ae_8010;
/// Removes the node `*slot` and moves `*slot` to the next node, answering
/// the element `(list, &slot)`.
const LIST_REMOVE_AT: u32 = 0x00ae_8090;
/// Allocates a node from the list's allocator `(list + 8)`.
const LIST_NEW_NODE: u32 = 0x0043_a010;
/// Links a node at the tail `(list, node)`.
const LIST_ADD_NODE_TAIL: u32 = 0x00a1_1db0;
/// Frees a node `(list + 8, node)`.
const LIST_NODE_FREE: u32 = 0x0045_cee0;
/// `BSSimpleList<AudioDebugString *>`'s append `(list, &element)`.
const SIMPLE_LIST_ADD: u32 = 0x00af_25b0;

/// The functions of other units the sound code calls.
/// `BSAudioSynchTimer` update, run on the `BSAudio` object +0x14.
const SYNCH_TIMER_UPDATE: u32 = 0x00ae_a100;
/// `BSGameSound::GetNameHash`-like: the hash of a file name (`00ae9d80`,
/// cdecl, one argument).
const SOUND_NAME_HASH: u32 = 0x00ae_9d80;
/// `BSGameSound::CalculateHashes` (`00ae9d50`, thiscall, one byte).
const SOUND_CALCULATE_HASHES: u32 = 0x00ae_9d50;
/// The range test of a sound (`00ae9b50`, thiscall, two words: 0 and the
/// game hour as a float).
const SOUND_RANGE_TEST: u32 = 0x00ae_9b50;
/// A step run on a sound with a flag byte (`00ae8b90`).
const SOUND_ENVIRONMENT_STEP: u32 = 0x00ae_8b90;
/// `BSSoundInfo`'s debug record getter (`00ae8b40`, `(record, out)`).
const DEBUG_INFO_GETTER: u32 = 0x00ae_8b40;
/// The `AudioLoadTask` constructor [`fn_00ae6f50`] (thiscall, the sound).
const LOAD_TASK_CONSTRUCTOR: u32 = 0x00ae_6f50;
/// The base constructor of the task (`00c3ce60`, one word: 0).
const LOAD_TASK_BASE_CONSTRUCTOR: u32 = 0x00c3_ce60;
/// The base destructor of the task.
const LOAD_TASK_BASE_DESTRUCTOR: u32 = 0x00c3_cea0;
/// Sets the task's file name `(task, name)`.
const LOAD_TASK_SET_FILE_NAME: u32 = 0x00c3_cee0;
/// Sets the task's context `(task, 3)`.
const LOAD_TASK_SET_CONTEXT: u32 = 0x00c3_cf60;
/// The base description formatter (`00c3d0e0`, `(task, a, b, text)`).
const LOAD_TASK_DESCRIBE: u32 = 0x00c3_d0e0;
/// Fixes a file name into a buffer (`00af4310`, cdecl: source, destination,
/// size).
const FILE_NAME_FIX: u32 = 0x00af_4310;
/// `BSThread::BSThread` and `BSThread::~BSThread`-like base calls of the
/// audio thread (`00aa6360`, `00aa63c0`), and its stop step (`00aa6590`).
const THREAD_BASE_CONSTRUCTOR: u32 = 0x00aa_6360;
const THREAD_BASE_DESTRUCTOR: u32 = 0x00aa_63c0;
const THREAD_STOP_STEP: u32 = 0x00aa_6590;
/// The vtables: `AudioLoadTask`, `BSAudioManagerThread`.
const AUDIO_LOAD_TASK_VTABLE: u32 = 0x010a_3718;
const BSAUDIO_MANAGER_THREAD_VTABLE: u32 = 0x010a_33d0;
/// Strings the code logs or formats.
const NO_FILENAME_MESSAGE: u32 = 0x010a_34f8;
const NO_DIMENSION_MESSAGE: u32 = 0x010a_34b0;
const ADD_SOUND_MESSAGE: u32 = 0x010a_3488;
const CACHE_SIZE_MESSAGE: u32 = 0x010a_36c8;
const LOAD_TASK_DESCRIPTION: u32 = 0x010a_3748;
const DEBUG_PAGE_FORMAT: u32 = 0x010a_36c0;
const DEBUG_HEADER_FORMAT: u32 = 0x010a_35f8;
const DEBUG_3D_FORMAT: u32 = 0x010a_35c0;
const DEBUG_2D_FORMAT: u32 = 0x010a_35a0;
const DEBUG_2D_RADIUS_FORMAT: u32 = 0x010a_3560;
const DEBUG_ERROR_FORMAT: u32 = 0x010a_353c;
const DEBUG_PLAIN_FORMAT: u32 = 0x010a_3524;
const STOPPED_TEXT: u32 = 0x010a_352c;
const PLAYING_TEXT: u32 = 0x010a_3534;
const STOP_TEXT: u32 = 0x0106_39ec;
const PLAY_TEXT: u32 = 0x0106_3958;
const BACKSLASH_TEXT: u32 = 0x0101_3444;
/// `strcpy` (CRT, cdecl: destination, source).
const STRCPY: u32 = 0x00ec_6370;
/// The vtables of the three pointer-map pairs the manager owns. Each map
/// class has a `NiTMapBase<NiTPointerAllocator<unsigned int>, int, T>` base
/// vtable and an `NiTPointerMap<int, T>` vtable over it (the names are the
/// RTTI type descriptors of these vtables): `T` is `BSGameSound *`
/// (`010a3370` over `010a3348`), `BSSoundInfo *` (`010a3390` over
/// `010a3328`) and `NiPointer<NiAVObject>` (`010a33b0` over `010a3308`).
const SOUND_MAP_BASE_VTABLE: u32 = 0x010a_3370;
const SOUND_MAP_VTABLE: u32 = 0x010a_3348;
const STATE_MAP_BASE_VTABLE: u32 = 0x010a_3390;
const STATE_MAP_VTABLE: u32 = 0x010a_3328;
const REFERENCE_MAP_BASE_VTABLE: u32 = 0x010a_33b0;
const REFERENCE_MAP_VTABLE: u32 = 0x010a_3308;
/// The comparison `GetDebugString` sorts with (`00ae5fb0`).
const DEBUG_LINE_COMPARE: u32 = 0x00ae_5fb0;

/// The `NiTPointerMap` virtual slots the moving-reference map's functions use:
/// hash of a key, key comparison `(key, item key)`, node initialisation
/// `(node, key, value)` and node allocation.
const MAP_SLOT_HASH: u32 = 0x04;
const MAP_SLOT_KEY_EQUAL: u32 = 0x08;
const MAP_SLOT_INIT_NODE: u32 = 0x0c;
const MAP_SLOT_NEW_NODE: u32 = 0x14;

/// `GetCurrentThreadId` import slot.
const IMPORT_GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
/// `TryEnterCriticalSection` import slot.
const IMPORT_TRY_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f1b0;
/// `Sleep` import slot.
const IMPORT_SLEEP: u32 = 0x00fd_f104;
/// `CreateSemaphoreA` import slot.
const IMPORT_CREATE_SEMAPHORE: u32 = 0x00fd_f0b4;
/// `ReleaseSemaphore` import slot.
const IMPORT_RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;
/// `CloseHandle` import slot.
const IMPORT_CLOSE_HANDLE: u32 = 0x00fd_f07c;
/// `DebugBreak` import slot.
const IMPORT_DEBUG_BREAK: u32 = 0x00fd_f0c8;

// More virtual slots of a sound (`BSGameSound`, Xbox PDB vtable offsets,
// which match the PC).
const SOUND_SLOT_DESTRUCTOR: u32 = 0x00;
const SOUND_SLOT_IS_IN_DECAY: u32 = 0x08;
const SOUND_SLOT_IS_PAUSED: u32 = 0x0c;
const SOUND_SLOT_SET_INAUDIBLE: u32 = 0x14;
const SOUND_SLOT_SET_ENVIRONMENT_TYPE: u32 = 0x18;
const SOUND_SLOT_OPEN: u32 = 0x24;
const SOUND_SLOT_PREPARE: u32 = 0x28;
const SOUND_SLOT_COPY: u32 = 0x2c;
const SOUND_SLOT_PLAY: u32 = 0x30;
const SOUND_SLOT_PAUSE: u32 = 0x34;
const SOUND_SLOT_STOP: u32 = 0x38;
const SOUND_SLOT_SET_VOLUME: u32 = 0x3c;
const SOUND_SLOT_UPDATE: u32 = 0x44;
const SOUND_SLOT_PROCESS_DATA_REQUEST: u32 = 0x48;
const SOUND_SLOT_SET_EMITTER_POSITION: u32 = 0x4c;
const SOUND_SLOT_SET_MIN_MAX: u32 = 0x5c;
const SOUND_SLOT_SET_ATTENUATION_CURVE: u32 = 0x60;
const SOUND_SLOT_SET_FREQUENCY: u32 = 0x64;
const SOUND_SLOT_GET_FREQUENCY: u32 = 0x68;
const SOUND_SLOT_SEEK: u32 = 0x6c;
// More fields of a sound (`BSGameSound`, Xbox PDB names, PC offsets).
const SOUND_STATE_FLAGS: u32 = 0x10;
const SOUND_MODIFIER_FLAGS: u32 = 0x0c;
const SOUND_REVERB_ATTENUATION: u32 = 0x1a;
const SOUND_FADER_ATTENUATION: u32 = 0x20;
const SOUND_CURRENT_VOLUME: u32 = 0x24;
const SOUND_BEGIN_TIME: u32 = 0x28;
const SOUND_END_TIME: u32 = 0x2c;
const SOUND_SAMPLES_PER_SECOND: u32 = 0x34;
const SOUND_HASH: u32 = 0x144;
const SOUND_DIRECTORY_HASH: u32 = 0x148;
const SOUND_SIZE: u32 = 0x14c;
const SOUND_FREQUENCY_VARIANCE: u32 = 0x158;
const SOUND_MOD_SAMPLES_PER_SECOND: u32 = 0x15a;
const SOUND_SYNCH_START_TIME: u32 = 0x15c;
const SOUND_CLEANUP_DELAY: u32 = 0x160;
const SOUND_COMPLETION_CALLBACK: u32 = 0x164;
const SOUND_PLAYED_CALLBACK: u32 = 0x168;
const SOUND_COMPLETION_CONTEXT: u32 = 0x16c;
const SOUND_PLAY_CONTEXT: u32 = 0x170;
const SOUND_LOOP_START: u32 = 0x174;
const SOUND_LOOP_END: u32 = 0x178;
const SOUND_BEAM_END: u32 = 0x17c;
const SOUND_PRIORITY: u32 = 0x188;
const SOUND_QUEUED_MESSAGES: u32 = 0x18c;

layout! {
    /// A Win32 `CRITICAL_SECTION` (24 bytes), as the manager embeds it.
    pub struct BSCriticalSection: 0x18 {}

    /// `AudioMessageStack` (Xbox PDB), 0x10 bytes: a stack of free
    /// messages linked through their `pData` word.
    pub struct AudioMessageStack: 0x10 {
        /// `iCount` (Xbox PDB).
        0x00 iCount: u32,
        /// `bBottomless` (Xbox PDB): a pop on an empty stack makes a new
        /// message.
        0x04 bBottomless: bool,
        /// `pTop` (Xbox PDB).
        0x08 pTop: u32,
        /// `iTimeLastAllocated` (Xbox PDB), a `GetTickCount` value.
        0x0C iTimeLastAllocated: u32,
    }

    /// `BSSoundMessage` (Xbox PDB), 0x20 bytes.
    pub struct BSSoundMessage: 0x20 {
        /// `msg` (Xbox PDB): the message type.
        0x00 msg: u32,
        /// `iID` (Xbox PDB): the sound id.
        0x04 iID: u32,
        /// `iData` (Xbox PDB).
        0x08 iData: u32,
        /// `pData` (Xbox PDB), also the link of a free message.
        0x0C pData: u32,
        /// `spData` (Xbox PDB): a smart pointer to a reference-counted
        /// object (an `NiAVObject` for the moving-sound messages).
        0x10 spData: u32,
        /// `vector3[0]` (Xbox PDB).
        0x14 vector3_x: f32,
        /// `vector3[1]` (Xbox PDB).
        0x18 vector3_y: f32,
        /// `vector3[2]` (Xbox PDB).
        0x1C vector3_z: f32,
        /// `curve[0]` (Xbox PDB `u16[10]` overlaying the vector).
        0x14 curve_0: u16,
        /// `curve[1]`.
        0x16 curve_1: u16,
        /// `curve[2]`.
        0x18 curve_2: u16,
        /// `curve[3]`.
        0x1A curve_3: u16,
        /// `curve[4]`.
        0x1C curve_4: u16,
    }

    /// `BSDebugSoundInfo` (Xbox PDB), 0x23c bytes. The PC code fills only
    /// the fields below.
    pub struct BSDebugSoundInfo: 0x23C {
        /// `pName` (Xbox PDB): points at the name buffer at +4.
        0x00 pName: u32,
        /// `Name` (Xbox PDB), a 260-byte buffer.
        0x04 Name: u8,
        /// `pFileName` (Xbox PDB): points at the name buffer that follows.
        0x108 pFileName: u32,
        /// `FileName` (Xbox PDB), a 260-byte buffer.
        0x10C FileName: u8,
        /// `fRadiusMin` (Xbox PDB), the sound's minimum distance.
        0x210 fRadiusMin: f32,
        /// `fRadiusMax` (Xbox PDB), the sound's maximum distance.
        0x214 fRadiusMax: f32,
        /// First coordinate of `Location` (Xbox PDB), the emitter x.
        0x218 locationX: f32,
        /// Second coordinate of `Location`, the emitter y.
        0x21C locationY: f32,
        /// The word the PDB calls the third coordinate of `Location`; the
        /// PC code stores the sound's distance to the listener here.
        0x220 distanceToListener: f32,
        /// `iStateFlags` (Xbox PDB).
        0x224 iStateFlags: u32,
        /// `fBeginTime` (Xbox PDB).
        0x228 fBeginTime: f32,
        /// `fEndTime` (Xbox PDB).
        0x22C fEndTime: f32,
        /// `iStaticAttenuation` (Xbox PDB).
        0x230 iStaticAttenuation: u32,
        /// `iSystemAttenuation` (Xbox PDB).
        0x234 iSystemAttenuation: u32,
        /// `iDistanceAttenuation` (Xbox PDB).
        0x238 iDistanceAttenuation: u32,
    }

    /// `BSSoundInfo` (Xbox PDB), 0x254 bytes: the manager's record of a
    /// sound for the voice-state sync, the debug record at +0x18.
    pub struct BSSoundInfo: 0x254 {
        /// `iID` (Xbox PDB).
        0x00 iID: u32,
        /// `fFrequency` (Xbox PDB).
        0x04 fFrequency: f32,
        /// `fVolume` (Xbox PDB).
        0x08 fVolume: f32,
        /// `uiSoundType` (Xbox PDB).
        0x0C uiSoundType: u32,
        /// `uiDuration` (Xbox PDB).
        0x10 uiDuration: u32,
        /// `bIsPlaying` (Xbox PDB).
        0x14 bIsPlaying: u8,
        /// `pDebugInfo` (Xbox PDB).
        0x18 pDebugInfo: Inline<BSDebugSoundInfo>,
    }

    /// `BSAudioManager` (Xbox PDB), 0x188 bytes on PC.
    pub struct BSAudioManager: 0x188 {
        /// The vtable (`010a3368`, the scalar deleting destructor).
        0x00 vtable: u32,
        /// `iMessageListIndex` (Xbox PDB): the list new messages join.
        0x04 iMessageListIndex: u32,
        /// `iMessageProcessingListIndex` (Xbox PDB).
        0x08 iMessageProcessingListIndex: u32,
        /// `Messages` (Xbox PDB): two lists of 0xc bytes.
        0x0C Messages: Inline<NiTPointerList>,
        /// `FreeMessages` (Xbox PDB).
        0x24 FreeMessages: Inline<AudioMessageStack>,
        /// `FreeQueueMessages` (Xbox PDB).
        0x34 FreeQueueMessages: Inline<AudioMessageStack>,
        /// `GarbageMessages` (Xbox PDB).
        0x44 GarbageMessages: Inline<AudioMessageStack>,
        /// `SoundList` (Xbox PDB): sound id to sound.
        0x54 SoundList: Inline<NiTPointerMap>,
        /// `SoundStateMap` (Xbox PDB): sound id to `BSSoundInfo`.
        0x64 SoundStateMap: Inline<NiTPointerMap>,
        /// `StateUpdateMap` (Xbox PDB): pending `BSSoundInfo` updates.
        0x74 StateUpdateMap: Inline<NiTPointerMap>,
        /// `MovingRefrList` (Xbox PDB).
        0x84 MovingRefrList: Inline<NiTPointerMap>,
        /// `CachedSoundList` (Xbox PDB).
        0x94 CachedSoundList: Inline<NiTPointerList>,
        /// `iCurrentCachSize` (Xbox PDB).
        0xA0 iCurrentCachSize: u32,
        /// `iWeatherFlags` (Xbox PDB).
        0xA4 iWeatherFlags: u32,
        /// `fCurrentGamehour` (Xbox PDB).
        0xA8 fCurrentGamehour: f32,
        /// `MessageCritSection` (Xbox PDB).
        0xAC MessageCritSection: Inline<BSCriticalSection>,
        /// `StateCritSection` (Xbox PDB).
        0xC4 StateCritSection: Inline<BSCriticalSection>,
        /// `CacheCritSection` (Xbox PDB).
        0xDC CacheCritSection: Inline<BSCriticalSection>,
        /// `ProcessingCritSection` (Xbox PDB).
        0xF4 ProcessingCritSection: Inline<BSCriticalSection>,
        /// `CacheLoadTaskCritSection` (Xbox PDB).
        0x10C CacheLoadTaskCritSection: Inline<BSCriticalSection>,
        /// `CacheLoadTaskList` (Xbox PDB).
        0x124 CacheLoadTaskList: Inline<NiTPointerList>,
        /// `iDelayTimerDelta` (Xbox PDB, 0x180 there): the constructor
        /// stores `GetTickCount` here.
        0x130 iDelayTimerDelta: u32,
        /// `bPrecacheCompleted` (Xbox PDB): set once `Update` has
        /// precached the interface sounds.
        0x134 bPrecacheCompleted: bool,
        /// `bListenerMoved` (Xbox PDB).
        0x135 bListenerMoved: bool,
        /// `dwAudioThreadID` (Xbox PDB).
        0x138 dwAudioThreadID: u32,
        /// `pUpdateThread` (Xbox PDB): the `BSAudioManagerThread`.
        0x13C pUpdateThread: u32,
        /// `fMasterVolume` (Xbox PDB).
        0x140 fMasterVolume: f32,
        /// `fFootVolume` (Xbox PDB).
        0x144 fFootVolume: f32,
        /// `fVoiceVolume` (Xbox PDB).
        0x148 fVoiceVolume: f32,
        /// `fEffectsVolume` (Xbox PDB).
        0x14C fEffectsVolume: f32,
        /// `fMusicVolume` (Xbox PDB).
        0x150 fMusicVolume: f32,
        /// `fRadioVolume` (Xbox PDB).
        0x154 fRadioVolume: f32,
        /// `fMasterSecondaryVolume` (Xbox PDB).
        0x158 fMasterSecondaryVolume: f32,
        /// `fFootSecondaryVolume` (Xbox PDB).
        0x15C fFootSecondaryVolume: f32,
        /// `fVoiceSecondaryVolume` (Xbox PDB).
        0x160 fVoiceSecondaryVolume: f32,
        /// `fEffectsSecondaryVolume` (Xbox PDB).
        0x164 fEffectsSecondaryVolume: f32,
        /// `fMusicSecondaryVolume` (Xbox PDB).
        0x168 fMusicSecondaryVolume: f32,
        /// `fRadioSecondaryVolume` (Xbox PDB).
        0x16C fRadioSecondaryVolume: f32,
        /// `iMuteMask` (Xbox PDB).
        0x170 iMuteMask: u32,
        /// `bDialogueFade` (Xbox PDB).
        0x174 bDialogueFade: bool,
        /// `bListenerUnderwater` (Xbox PDB).
        0x175 bListenerUnderwater: bool,
        /// `pPauseEntries` (Xbox PDB).
        0x178 pPauseEntries: u32,
        /// `iPauseListSize` (Xbox PDB).
        0x17C iPauseListSize: u32,
        /// `iNextID` (Xbox PDB).
        0x180 iNextID: u32,
        /// `bIgnoreGlobalTimeMultiplier` (Xbox PDB).
        0x184 bIgnoreGlobalTimeMultiplier: bool,
    }
}

layout! {
    /// `AudioLoadTask` (Xbox PDB), 0x38 bytes, derived from `BSTask`: the
    /// fields before `pSound` are those of the task bases (not declared).
    pub struct AudioLoadTask: 0x38 {
        /// `pSound` (Xbox PDB): the sound the task loads.
        0x30 pSound: u32,
    }

    /// `BSAudioManagerThread` (Xbox PDB), 0x4c bytes on PC: a `BSThread`
    /// (0x30 bytes, its critical section 0x18 here) followed by the two
    /// semaphores (`m_SuspendSema` at +0x30 and `m_SuspendedSema` at +0x3c,
    /// each count, handle, maximum count).
    pub struct BSAudioManagerThread: 0x4C {
        /// `m_ThreadID` (Xbox PDB).
        0x24 m_ThreadID: u32,
        /// `m_bSuspended` (Xbox PDB).
        0x48 m_bSuspended: bool,
    }
}

/// `AddHead` (`00a0c9f0`) of the pointer `element` on the pointer list at
/// `list`.
fn list_add_head(e: &mut Engine, list: u32, element: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), element);
        e.call(LIST_ADD_HEAD, &[list, slot.addr()]);
    });
}

/// Removes the node `node` from the pointer list at `list` (`00ae8090`).
fn list_remove_node(e: &mut Engine, list: u32, node: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), node);
        e.call(LIST_REMOVE_AT, &[list, slot.addr()]);
    });
}

/// Appends the pointer `element` to the pointer list at `list` (`00ae8bf0`,
/// which the engine map names `GetFromCache`; its body adds a node and
/// stores the pointer in it).
fn list_append_pointer(e: &mut Engine, list: u32, element: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), element);
        e.call(LIST_APPEND_POINTER, &[list, slot.addr()]);
    });
}

/// Enters the critical section at `section`.
fn enter_section(e: &mut Engine, section: u32) {
    e.call(IMPORT_ENTER_CRITICAL_SECTION, &[section]);
}

/// Leaves the critical section at `section`.
fn leave_section(e: &mut Engine, section: u32) {
    e.call(IMPORT_LEAVE_CRITICAL_SECTION, &[section]);
}

/// Adds a reference to the reference-counted object at `object` (the count
/// is at +4), as the inlined smart-pointer constructor does for a non-null
/// pointer.
fn add_reference(e: &mut Engine, object: u32) {
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &[object + 4]);
    }
}

/// Drops a reference of the object at `object`; when the count reaches
/// zero calls its virtual method at +4 (the inlined smart-pointer
/// destructor).
fn drop_reference(e: &mut Engine, object: u32) {
    if object != 0 && e.call(IMPORT_INTERLOCKED_DECREMENT, &[object + 4]).u32() == 0 {
        e.vcall(object, 4, &[]);
    }
}

/// Destroys a manager-owned object: when `object` is not null runs its
/// destructor at `destructor` and then frees it (the scalar deleting
/// destructor the compiler inlined).
fn destroy_object(e: &mut Engine, destructor: u32, object: u32) {
    if object != 0 {
        e.call(destructor, &[object]);
        e.call(DEALLOCATE, &[MEMORY_MANAGER, object]);
    }
}

/// The inlined start of an `NiTPointerMap` walk: the first item of the
/// first non-empty bucket, or 0 for an empty map.
fn map_first_item(e: &Engine, map: Ptr<NiTPointerMap>) -> u32 {
    let buckets = e.get(map, NiTPointerMap::m_uiHashSize);
    let table = e.get(map, NiTPointerMap::m_ppkHashTable);
    for index in 0..buckets {
        let item = e.mem.u32(table.wrapping_add(index * 4));
        if item != 0 {
            return item;
        }
    }
    0
}

/// `GetNext` (`00a0c850`) on a map: returns the item's key and value and
/// moves `position` to the next item (0 at the end).
fn map_next(e: &mut Engine, map: Ptr<NiTPointerMap>, position: &mut u32) -> (u32, u32) {
    let current = *position;
    e.with_stack(12, |e, slots| {
        e.mem.set_u32(slots.addr(), current);
        e.call(
            MAP_GET_NEXT,
            &[map.addr(), slots.addr(), slots.addr() + 4, slots.addr() + 8],
        );
        *position = e.mem.u32(slots.addr());
        (e.mem.u32(slots.addr() + 4), e.mem.u32(slots.addr() + 8))
    })
}

/// `GetAt` (`00a1e380`) on a map: whether the key is present, and the value
/// slot afterwards (it keeps `initial` when the key is absent).
fn map_get(e: &mut Engine, map: Ptr<NiTPointerMap>, key: u32, initial: u32) -> (bool, u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), initial);
        let found = e
            .call(MAP_GET_VALUE, &[map.addr(), key, slot.addr()])
            .bool();
        (found, e.mem.u32(slot.addr()))
    })
}

/// Reads the three words at the default message vector.
fn default_message_vector(e: &Engine) -> (f32, f32, f32) {
    (
        e.global::<f32>(DEFAULT_MESSAGE_VECTOR),
        e.global::<f32>(DEFAULT_MESSAGE_VECTOR + 4),
        e.global::<f32>(DEFAULT_MESSAGE_VECTOR + 8),
    )
}

/// What most public methods do: queue a message of type `kind` with the
/// three words, a null smart pointer and the default vector.
fn queue_plain(e: &mut Engine, this: Ptr<BSAudioManager>, kind: u32, a: u32, b: u32, c: u32) {
    let (x, y, z) = default_message_vector(e);
    fn_00adb1a0(e, this, kind, a, b, c, 0, x, y, z);
}

/// The `float * 1000` then truncate that several wrappers do inline
/// (`FISTP` with truncation, the low word of the 64-bit result).
fn scaled_milliseconds_inline(e: &Engine, seconds: f32) -> u32 {
    let scale = e.mem.f64(MILLISECOND_SCALE);
    let scaled = seconds as f64 * scale;
    if scaled.is_nan() || scaled >= i64::MAX as f64 || scaled < i64::MIN as f64 {
        // FISTP stores the "integer indefinite" value 0x8000000000000000.
        return 0;
    }
    (scaled.trunc() as i64) as u32
}

/// The `float * 1000` converted through `_ftol2` (`00ec62c0`).
fn scaled_milliseconds_ftol(e: &mut Engine, seconds: f32) -> u32 {
    let scale = e.mem.f64(MILLISECOND_SCALE);
    let scaled = seconds as f64 * scale;
    e.call(FTOL2, &args![scaled]).u32()
}

/// Runs `body` with the audio TLS context word set to 0xb, restoring the
/// previous value afterwards (the prologue/epilogue of the audio methods
/// that save and set `+0x2b4` of the thread block).
fn in_audio_context<R>(e: &mut Engine, body: impl FnOnce(&mut Engine) -> R) -> R {
    let at = e.tls() + AUDIO_TLS_CONTEXT;
    let previous = e.mem.u32(at);
    e.mem.set_u32(at, AUDIO_CONTEXT_VALUE);
    let result = body(e);
    e.mem.set_u32(at, previous);
    result
}

// Translated from 00ad9060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::QInstance` (Xbox PDB): the manager singleton. The first
/// call runs the constructor on the static object and registers its
/// destruction with `atexit` (the compiler's function-local static guard in
/// the word at `011f7078`).
// Translation incomplete: 00ad9060 - C++ exception unwinding (SEH frame state) of the static initialization is not translated.
pub fn bs_audio_manager_q_instance(e: &mut Engine) -> Ptr<BSAudioManager> {
    let guard = e.mem.u32(MANAGER_BUILT_GUARD);
    if guard & 1 == 0 {
        e.mem.set_u32(MANAGER_BUILT_GUARD, guard | 1);
        fn_00ad90e0(e, Ptr::new(MANAGER_INSTANCE));
        e.call(ATEXIT, &[MANAGER_EXIT_THUNK]);
    }
    Ptr::new(MANAGER_INSTANCE)
}

// Translated from 00ad90e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSAudioManager` constructor (unnamed in the engine map): sets the
/// vtable, constructs the two message lists, the three message stacks, the
/// four maps (37 buckets each) and the lists, initializes the critical
/// sections and zeroes the state; `iDelayTimerDelta` takes `GetTickCount`.
/// Returns `this`.
// Translation incomplete: 00ad90e0 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_00ad90e0(e: &mut Engine, this: Ptr<BSAudioManager>) -> Ptr<BSAudioManager> {
    let base = this.addr();
    e.mem.set_u32(base, 0x010a_3368);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &[
            base + 0x0c,
            0x0c,
            2,
            MESSAGE_LIST_CONSTRUCTOR,
            MESSAGE_LIST_DESTRUCTOR,
        ],
    );
    e.call(MESSAGE_STACK_CONSTRUCTOR, &[base + 0x24]);
    e.call(MESSAGE_STACK_CONSTRUCTOR, &[base + 0x34]);
    e.call(MESSAGE_STACK_CONSTRUCTOR, &[base + 0x44]);
    e.call(MAP_CONSTRUCTOR_SOUND_LIST, &[base + 0x54, 0x25]);
    e.mem.set_u32(base + 0x54, 0x010a_3348);
    e.call(MAP_CONSTRUCTOR_SOUND_STATES, &[base + 0x64, 0x25]);
    e.mem.set_u32(base + 0x64, 0x010a_3328);
    e.call(MAP_CONSTRUCTOR_SOUND_STATES, &[base + 0x74, 0x25]);
    e.mem.set_u32(base + 0x74, 0x010a_3328);
    e.call(MAP_CONSTRUCTOR_MOVING_REFERENCES, &[base + 0x84, 0x25]);
    e.mem.set_u32(base + 0x84, 0x010a_3308);
    // CachedSoundList: an empty NiTPointerList.
    e.set(
        this.at(BSAudioManager::CachedSoundList),
        NiTPointerList::m_uiCount,
        0,
    );
    e.set(
        this.at(BSAudioManager::CachedSoundList),
        NiTPointerList::m_pkHead,
        0,
    );
    e.set(
        this.at(BSAudioManager::CachedSoundList),
        NiTPointerList::m_pkTail,
        0,
    );
    for section in [0xac, 0xc4, 0xdc, 0xf4, 0x10c] {
        e.call(IMPORT_INITIALIZE_CRITICAL_SECTION, &[base + section]);
    }
    // CacheLoadTaskList: an empty NiTPointerList.
    e.set(
        this.at(BSAudioManager::CacheLoadTaskList),
        NiTPointerList::m_uiCount,
        0,
    );
    e.set(
        this.at(BSAudioManager::CacheLoadTaskList),
        NiTPointerList::m_pkHead,
        0,
    );
    e.set(
        this.at(BSAudioManager::CacheLoadTaskList),
        NiTPointerList::m_pkTail,
        0,
    );
    e.set(this, BSAudioManager::dwAudioThreadID, 0);
    for volume in [
        BSAudioManager::fMasterVolume,
        BSAudioManager::fFootVolume,
        BSAudioManager::fVoiceVolume,
        BSAudioManager::fEffectsVolume,
        BSAudioManager::fMusicVolume,
        BSAudioManager::fRadioVolume,
        BSAudioManager::fMasterSecondaryVolume,
        BSAudioManager::fFootSecondaryVolume,
        BSAudioManager::fVoiceSecondaryVolume,
        BSAudioManager::fEffectsSecondaryVolume,
        BSAudioManager::fMusicSecondaryVolume,
        BSAudioManager::fRadioSecondaryVolume,
    ] {
        e.set(this, volume, 0.0);
    }
    e.set(this, BSAudioManager::bPrecacheCompleted, false);
    e.set(this, BSAudioManager::iWeatherFlags, 0);
    e.set(this, BSAudioManager::iNextID, 1);
    e.set(this, BSAudioManager::iMessageListIndex, 0);
    e.set(this, BSAudioManager::iMessageProcessingListIndex, 1);
    e.set(this, BSAudioManager::iCurrentCachSize, 0);
    e.set(this, BSAudioManager::pUpdateThread, 0);
    e.set(this, BSAudioManager::iMuteMask, 0);
    e.set(this, BSAudioManager::bListenerMoved, false);
    e.set(this, BSAudioManager::iWeatherFlags, 0);
    e.set(this, BSAudioManager::fCurrentGamehour, 0.0);
    let ticks = e.call(IMPORT_GET_TICK_COUNT, &[]).u32();
    e.set(this, BSAudioManager::iDelayTimerDelta, ticks);
    e.set(this, BSAudioManager::bDialogueFade, false);
    e.set(this, BSAudioManager::bListenerUnderwater, false);
    this
}

// Translated from 00ad93f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSAudioManager` destructor body (unnamed in the engine map): resets
/// the vtable, empties the cache-load list and the cached-sound list,
/// deletes the critical sections, clears and frees the four maps (each map
/// sets its two base vtables on the way), empties the three message stacks
/// and destroys the two message lists.
// Translation incomplete: 00ad93f0 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_00ad93f0(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010a_3368);
    e.call(LIST_CLEAR, &[base + 0x124]);
    for section in [0x10c, 0xf4, 0xdc, 0xc4, 0xac] {
        e.call(IMPORT_DELETE_CRITICAL_SECTION, &[base + section]);
    }
    e.call(LIST_CLEAR, &[base + 0x94]);
    // Each map: the derived vtable, a clear, the base vtable, a clear and
    // the table freed.
    for (map, derived, base_vtable) in [
        (base + 0x84, 0x010a_3308, 0x010a_33b0),
        (base + 0x74, 0x010a_3328, 0x010a_3390),
        (base + 0x64, 0x010a_3328, 0x010a_3390),
        (base + 0x54, 0x010a_3348, 0x010a_3370),
    ] {
        e.mem.set_u32(map, derived);
        e.call(MAP_CLEAR, &[map]);
        e.mem.set_u32(map, base_vtable);
        e.call(MAP_CLEAR, &[map]);
        let table = e.mem.u32(map + 8);
        e.call(FREE_TABLE, &[table]);
    }
    e.call(MESSAGE_STACK_DESTRUCTOR, &[base + 0x44]);
    e.call(MESSAGE_STACK_DESTRUCTOR, &[base + 0x34]);
    e.call(MESSAGE_STACK_DESTRUCTOR, &[base + 0x24]);
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &[base + 0x0c, 0x0c, 2, MESSAGE_LIST_DESTRUCTOR],
    );
}

// Translated from 00ad9640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Initialize` (Xbox PDB): when the audio setting at
/// `011f6da0` is on, allocates and clears the 8-entry pause list, sets the
/// volumes (`00ad9da0`), fills the free-message stack and the free-queue
/// stack with 0x100 constructed messages each (the stacks then become
/// bottomless, the second one's third byte stays 0) and constructs the
/// audio thread object.
// Translation incomplete: 00ad9640 - C++ exception unwinding (SEH frame state) around the allocations is not translated.
pub fn bs_audio_manager_initialize(e: &mut Engine, this: Ptr<BSAudioManager>) {
    if e.mem.u8(AUDIO_ENABLED_SETTING) == 0 {
        return;
    }
    e.set(this, BSAudioManager::iWeatherFlags, 0);
    e.set(this, BSAudioManager::iPauseListSize, 8);
    // Size of the pause list: 8 entries of 8 bytes, saturating at 32 bits.
    let entries = e.get(this, BSAudioManager::iPauseListSize);
    let bytes = entries as u64 * 8;
    let size = if bytes >> 32 != 0 {
        u32::MAX
    } else {
        bytes as u32
    };
    let block = e.call(ALLOCATE, &[MEMORY_MANAGER, size]).u32();
    e.set(this, BSAudioManager::pPauseEntries, block);
    let entries = e.get(this, BSAudioManager::iPauseListSize);
    e.call(MEMSET, &[block, 0, entries << 3]);
    fn_00ad9da0(e, this);
    for _ in 0..0x100 {
        let raw = e.call(ALLOCATE, &[MEMORY_MANAGER, 0x20]).u32();
        let message = if raw != 0 {
            e.call(MESSAGE_CONSTRUCTOR, &[raw]).u32()
        } else {
            0
        };
        e.call(MESSAGE_STACK_PUSH, &[this.addr() + 0x24, message]);
    }
    e.set(
        this.at(BSAudioManager::FreeMessages),
        AudioMessageStack::bBottomless,
        true,
    );
    for _ in 0..0x100 {
        let raw = e.call(ALLOCATE, &[MEMORY_MANAGER, 0x20]).u32();
        let message = if raw != 0 {
            e.call(MESSAGE_CONSTRUCTOR, &[raw]).u32()
        } else {
            0
        };
        e.call(MESSAGE_STACK_PUSH, &[this.addr() + 0x34, message]);
    }
    e.set(
        this.at(BSAudioManager::FreeQueueMessages),
        AudioMessageStack::bBottomless,
        true,
    );
    e.set(
        this.at(BSAudioManager::GarbageMessages),
        AudioMessageStack::bBottomless,
        false,
    );
    let raw = e.call(ALLOCATE, &[MEMORY_MANAGER, 0x4c]).u32();
    let thread = if raw != 0 {
        e.call(THREAD_CONSTRUCTOR, &[raw]).u32()
    } else {
        0
    };
    e.set(this, BSAudioManager::pUpdateThread, thread);
}

// Translated from 00ad9870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Shutdown` (Xbox PDB): runs `Update` three times when the
/// audio does not run on its own thread, empties both message lists
/// (destroying every message), deletes the audio thread object, removes
/// every sound from the sound list (setting its voice state to none and
/// deleting it), clears the moving-reference list, deletes every record in
/// the two state maps, flushes the cache (`00adcd80`, with its flag byte
/// cleared meanwhile) and frees the pause list.
pub fn bs_audio_manager_shutdown(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    if e.mem.u8(audio + 6) == 0 {
        for _ in 0..3 {
            bs_audio_manager_update(e, this, 0);
        }
    }
    for list in [this.addr() + 0x0c, this.addr() + 0x18] {
        while e.mem.u32(list + 8) != 0 {
            let message = e.call(LIST_REMOVE_HEAD, &[list]).u32();
            destroy_object(e, MESSAGE_DESTRUCTOR, message);
        }
    }
    let thread = e.get(this, BSAudioManager::pUpdateThread);
    if thread != 0 {
        // The scalar deleting destructor, first slot, with the delete flag.
        e.vcall(thread, 0, &[1]);
        e.set(this, BSAudioManager::pUpdateThread, 0);
    }
    // Every sound: remove it from the list, clear its voice state, delete it.
    let sounds = this.at(BSAudioManager::SoundList);
    let mut position = map_first_item(e, sounds);
    while position != 0 {
        let (_key, sound) = map_next(e, sounds, &mut position);
        let id = e.mem.u32(sound + SOUND_ID);
        e.call(MAP_REMOVE, &[sounds.addr(), id]);
        let id = e.mem.u32(sound + SOUND_ID);
        bs_audio_manager_set_voice_state(e, this, id, Ptr::NULL);
        if sound != 0 {
            e.vcall(sound, 0, &[1]);
        }
    }
    e.call(MAP_CLEAR, &[this.addr() + 0x84]);
    // Every pending state update, with the record it replaces.
    let updates = this.at(BSAudioManager::StateUpdateMap);
    let states = this.at(BSAudioManager::SoundStateMap);
    let mut position = map_first_item(e, updates);
    let mut old = 0;
    while position != 0 {
        let (key, update) = map_next(e, updates, &mut position);
        loop {
            let (found, record) = map_get(e, states, key, old);
            old = record;
            if !found {
                break;
            }
            while e.call(MAP_REMOVE, &[states.addr(), key]).bool() {}
            destroy_object(e, SOUND_INFO_DESTRUCTOR, old);
        }
        destroy_object(e, SOUND_INFO_DESTRUCTOR, update);
    }
    e.call(MAP_CLEAR, &[this.addr() + 0x74]);
    let saved = e.mem.u8(CACHE_FLUSH_FLAG);
    e.mem.set_u8(CACHE_FLUSH_FLAG, 0);
    e.call(MANAGER_CACHE_FLUSH, &[this.addr(), 0]);
    e.mem.set_u8(CACHE_FLUSH_FLAG, saved);
    let pause_entries = e.get(this, BSAudioManager::pPauseEntries);
    e.call(DEALLOCATE, &[MEMORY_MANAGER, pause_entries]);
}

// Translated from 00ad9da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the six volumes from the `f...VolumeDefault` settings (the float
/// values at `011f6e18`, `e24`, `e30`, `e3c`, `e48`, `e54`: master, foot,
/// voice, effects, music, radio) and the six secondary volumes to 1.0.
/// (The compiler's null-setting fallbacks are folded away.)
pub fn fn_00ad9da0(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let defaults = [
        (0x011f_6e18, BSAudioManager::fMasterVolume),
        (0x011f_6e24, BSAudioManager::fFootVolume),
        (0x011f_6e30, BSAudioManager::fVoiceVolume),
        (0x011f_6e3c, BSAudioManager::fEffectsVolume),
        (0x011f_6e48, BSAudioManager::fMusicVolume),
        (0x011f_6e54, BSAudioManager::fRadioVolume),
    ];
    for (setting, field) in defaults {
        let value = x87_quiet(e.global::<f32>(setting));
        e.set(this, field, value);
    }
    for field in [
        BSAudioManager::fMasterSecondaryVolume,
        BSAudioManager::fFootSecondaryVolume,
        BSAudioManager::fVoiceSecondaryVolume,
        BSAudioManager::fEffectsSecondaryVolume,
        BSAudioManager::fMusicSecondaryVolume,
        BSAudioManager::fRadioSecondaryVolume,
    ] {
        e.set(this, field, 1.0);
    }
}

// Translated from 00ad9f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::StartAudioThread` (Xbox PDB): when the audio thread
/// object exists and has no thread id yet, initializes the thread (0x8000
/// bytes of stack, named `LibAudioUpdate`), remembers its id, sets the
/// processor and the name and names the thread for the debugger; then
/// signals the thread's semaphore.
pub fn bs_audio_manager_start_audio_thread(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let thread = e.get(this, BSAudioManager::pUpdateThread);
    if thread == 0 {
        return;
    }
    // BSAudioManagerThread: the thread id is at +0x24 on PC.
    if e.mem.u32(thread + 0x24) == 0 {
        e.call(THREAD_INITIALIZE, &[thread, 0x8000, AUDIO_THREAD_NAME, 0]);
        let id = e.mem.u32(thread + 0x24);
        e.set(this, BSAudioManager::dwAudioThreadID, id);
        let processor = e.global::<u32>(AUDIO_THREAD_PROCESSOR);
        e.call(THREAD_SET_PROCESSOR, &[thread, processor]);
        e.call(THREAD_SET_NAME, &[thread, AUDIO_THREAD_NAME]);
        let id = e.mem.u32(thread + 0x24);
        e.call(SET_THREAD_NAME, &[id, AUDIO_THREAD_NAME]);
    }
    e.call(THREAD_SEMAPHORE_SIGNAL, &[thread]);
}

/// The shared tail of `SuspendAudioThread` and `StopAudioThread`: after the
/// message is queued, waits on the thread's semaphore handle (+0x40,
/// infinite) and decrements the counter at +0x3c.
fn wait_for_thread_semaphore(e: &mut Engine, thread: u32) {
    let handle = e.mem.u32(thread + 0x40);
    e.call(IMPORT_WAIT_FOR_SINGLE_OBJECT, &[handle, 0xffff_ffff]);
    e.call(IMPORT_INTERLOCKED_DECREMENT, &[thread + 0x3c]);
}

// Translated from 00ada020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SuspendAudioThread` (Xbox PDB): signals the thread,
/// queues the suspend message (0x35) and waits for the thread to
/// acknowledge on its semaphore.
pub fn bs_audio_manager_suspend_audio_thread(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let thread = e.get(this, BSAudioManager::pUpdateThread);
    if thread != 0 {
        e.call(THREAD_SEMAPHORE_SIGNAL, &[thread]);
    }
    queue_plain(e, this, 0x35, 0, 0, 0);
    let thread = e.get(this, BSAudioManager::pUpdateThread);
    if thread != 0 {
        wait_for_thread_semaphore(e, thread);
    }
}

// Translated from 00ada0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::StopAudioThread` (Xbox PDB): like `SuspendAudioThread`
/// with the stop message (0x34), and afterwards finishes the thread
/// (`00aa64f0`).
pub fn bs_audio_manager_stop_audio_thread(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let thread = e.get(this, BSAudioManager::pUpdateThread);
    if thread != 0 {
        e.call(THREAD_SEMAPHORE_SIGNAL, &[thread]);
    }
    queue_plain(e, this, 0x34, 0, 0, 0);
    let thread = e.get(this, BSAudioManager::pUpdateThread);
    if thread != 0 {
        wait_for_thread_semaphore(e, thread);
        let thread = e.get(this, BSAudioManager::pUpdateThread);
        e.call(THREAD_FINISH, &[thread]);
    }
}

/// The interface sounds `Update` precaches the first time it runs: name
/// text address and the flags passed with it.
const PRECACHED_INTERFACE_SOUNDS: [(u32, u32); 10] = [
    (0x0106_f060, 0),
    (0x0106_f39c, 0x20000),
    (0x0106_f360, 0x20001),
    (0x010a_3444, 0x20000),
    (0x0106_f380, 0x20001),
    (0x0106_f38c, 1),
    (0x010a_342c, 0x102),
    (0x010a_3418, 0x102),
    (0x010a_33fc, 0x101),
    (0x010a_33e8, 0x101),
];

// Translated from 00ada1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Update` (Xbox PDB): the first time, precaches ten
/// interface and weapon sounds through `BSAudio::Precache`; then syncs the
/// voice states and, when the audio does not run on its own thread, runs
/// `Process` on the singleton. The one stack word is not read.
pub fn bs_audio_manager_update(e: &mut Engine, this: Ptr<BSAudioManager>, _unused_1: u32) {
    if !e.get(this, BSAudioManager::bPrecacheCompleted) {
        let audio = e.global::<u32>(AUDIO_GLOBAL);
        for (name, flags) in PRECACHED_INTERFACE_SOUNDS {
            e.call(AUDIO_PRECACHE, &[audio, name, flags]);
        }
        e.set(this, BSAudioManager::bPrecacheCompleted, true);
    }
    bs_audio_manager_synch_voice_states(e, this);
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    if e.mem.u8(audio + 6) == 0 {
        let manager = bs_audio_manager_q_instance(e);
        e.call(MANAGER_PROCESS, &[manager.addr()]);
    }
}

// Translated from 00ada300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the weather flags and the game hour (unnamed in the engine map).
pub fn fn_00ada300(e: &mut Engine, this: Ptr<BSAudioManager>, game_hour: f32, weather_flags: u32) {
    e.set(this, BSAudioManager::iWeatherFlags, weather_flags);
    e.set(this, BSAudioManager::fCurrentGamehour, x87_quiet(game_hour));
}

// Translated from 00ada330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SynchVoiceStates` (Xbox PDB): under the state critical
/// section, walks the pending-update map: each entry replaces the record in
/// the state map (the old record is deleted); an entry whose id is 0 is
/// deleted instead. The pending-update map is then cleared. (A debug
/// listing of the state map, guarded by a compile-time zero, is folded
/// away.)
pub fn bs_audio_manager_synch_voice_states(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let section = this.addr() + 0xc4;
    enter_section(e, section);
    if e.get(
        this.at(BSAudioManager::StateUpdateMap),
        NiTPointerMap::m_uiCount,
    ) != 0
    {
        let updates = this.at(BSAudioManager::StateUpdateMap);
        let states = this.at(BSAudioManager::SoundStateMap);
        let mut position = map_first_item(e, updates);
        let mut old = 0;
        while position != 0 {
            let (key, update) = map_next(e, updates, &mut position);
            let (found, record) = map_get(e, states, key, old);
            old = record;
            if found {
                while e.call(MAP_REMOVE, &[states.addr(), key]).bool() {}
                destroy_object(e, SOUND_INFO_DESTRUCTOR, old);
            }
            if e.mem.u32(update) != 0 {
                e.call(MAP_SET_VALUE, &[states.addr(), key, update]);
            } else {
                destroy_object(e, SOUND_INFO_DESTRUCTOR, update);
            }
        }
        e.call(MAP_CLEAR, &[updates.addr()]);
    }
    leave_section(e, section);
}

/// Clamps a distance the way `SetVoiceState` does for the debug record: a
/// value greater than 3.4028235e+37 (a "no limit" marker) becomes the
/// float at `01012054` (-1.0); anything else, NaN included, is kept.
fn clamp_distance(e: &Engine, distance: f32) -> f32 {
    let limit = e.mem.f64(HUGE_DISTANCE_LIMIT);
    if distance as f64 > limit {
        x87_quiet(e.global::<f32>(HUGE_DISTANCE_REPLACEMENT))
    } else {
        x87_quiet(distance)
    }
}

/// The debug-record fill of `SetVoiceState`: the file name, the distances
/// and the attenuations of `sound` into the debug record at `record`
/// (cleared first).
fn fill_debug_record(e: &mut Engine, record: Ptr<BSDebugSoundInfo>, sound: u32) {
    e.call(MEMSET, &[record.addr(), 0, 0x23c]);
    let buffer = record.addr() + 0x10c;
    if e.get(record, BSDebugSoundInfo::pFileName) != buffer {
        e.set(record, BSDebugSoundInfo::pFileName, buffer);
    }
    let name_pointer = e.get(record, BSDebugSoundInfo::pFileName);
    // The PC code tests `sound + 0x36 != 0`, false only for the address
    // 0xffffffca.
    if sound.wrapping_add(SOUND_FILENAME) != 0 {
        e.call(
            STRCPY_S,
            &[name_pointer, 0x104, sound.wrapping_add(SOUND_FILENAME)],
        );
    } else {
        e.call(STRCPY_S, &[name_pointer, 0x104, NOT_TES_SOUND_TEXT]);
    }
    let positioned = e.call(SOUND_IS_EMITTER_SOUND, &[sound]).bool()
        || e.call(SOUND_HAS_EMITTER, &[sound]).bool();
    if positioned {
        let distance = clamp_distance(e, e.mem.f32(sound + SOUND_DISTANCE_TO_LISTENER));
        e.set(record, BSDebugSoundInfo::distanceToListener, distance);
        let x = e.with_stack(12, |e, out| {
            let position = e
                .vcall(sound, SOUND_SLOT_GET_EMITTER_POSITION, &[out.addr()])
                .u32();
            x87_quiet(e.mem.f32(position))
        });
        e.set(record, BSDebugSoundInfo::locationX, x);
        let y = e.with_stack(12, |e, out| {
            let position = e
                .vcall(sound, SOUND_SLOT_GET_EMITTER_POSITION, &[out.addr()])
                .u32();
            x87_quiet(e.mem.f32(position + 4))
        });
        e.set(record, BSDebugSoundInfo::locationY, y);
        let minimum = clamp_distance(e, e.mem.f32(sound + SOUND_MIN_DISTANCE));
        e.set(record, BSDebugSoundInfo::fRadiusMin, minimum);
        let maximum = clamp_distance(e, e.mem.f32(sound + SOUND_MAX_DISTANCE));
        e.set(record, BSDebugSoundInfo::fRadiusMax, maximum);
    } else {
        e.set(record, BSDebugSoundInfo::distanceToListener, 0.0);
        e.set(record, BSDebugSoundInfo::locationX, 0.0);
        e.set(record, BSDebugSoundInfo::locationY, 0.0);
        e.set(record, BSDebugSoundInfo::fRadiusMin, 0.0);
        e.set(record, BSDebugSoundInfo::fRadiusMax, 0.0);
    }
    let static_attenuation = e.mem.u16(sound + SOUND_STATIC_ATTENUATION) as u32;
    e.set(
        record,
        BSDebugSoundInfo::iStaticAttenuation,
        static_attenuation,
    );
    let system_attenuation = e.mem.u16(sound + SOUND_SYSTEM_ATTENUATION) as u32;
    e.set(
        record,
        BSDebugSoundInfo::iSystemAttenuation,
        system_attenuation,
    );
    let distance_attenuation = e.mem.u16(sound + SOUND_DISTANCE_ATTENUATION) as u32;
    e.set(
        record,
        BSDebugSoundInfo::iDistanceAttenuation,
        distance_attenuation,
    );
}

/// Copies the debug record at `record` into a temporary (the by-value
/// argument copy the compiler makes), assigns it to the debug part of
/// `target` (`BSDebugSoundInfo::operator=` on `target + 0x18`) and destroys
/// the temporary.
fn assign_debug_record(e: &mut Engine, target: u32, record: Ptr<BSDebugSoundInfo>) {
    let bytes = e.mem.bytes(record.addr(), 0x23c);
    e.with_stack(0x23c, |e, copy| {
        e.mem.write(copy.addr(), &bytes);
        e.call(DEBUG_SOUND_INFO_ASSIGN, &[target + 0x18, copy.addr()]);
        e.call(DEBUG_SOUND_INFO_DESTRUCTOR, &[copy.addr()]);
    });
}

// Translated from 00ada5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetVoiceState` (Xbox PDB): records the state of the
/// sound `sound` (null after `Shutdown`) under `id` in the pending-update
/// map. A local `BSSoundInfo` is built from the sound (its id, a frequency
/// of 1.0, its volume, type flags, duration and whether it is playing) and,
/// when the debug-information setting is on, a debug record (file name,
/// distances, position, attenuations). Under the state critical section the
/// record either overwrites the entry already in the map or becomes a new
/// 0x254-byte entry.
// Translation incomplete: 00ada5d0 - C++ exception unwinding (SEH frame state) and the stack cookie check are not translated.
pub fn bs_audio_manager_set_voice_state(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    sound: Ptr,
) {
    let sound = sound.addr();
    in_audio_context(e, |e| {
        // Locals the game keeps on its stack: the working record, two debug
        // records, the temporary built from the sound and a copy slot.
        e.with_stack(0xb5c, |e, frame| {
            let info = frame.addr();
            let debug_a = Ptr::<BSDebugSoundInfo>::new(info + 0x254);
            let debug_b = info + 0x490;
            let temporary = info + 0x6cc;
            e.call(SOUND_INFO_CONSTRUCTOR, &[info]);
            e.call(DEBUG_SOUND_INFO_CONSTRUCTOR, &[debug_a.addr()]);
            e.call(DEBUG_SOUND_INFO_CONSTRUCTOR, &[debug_b]);
            let debug = e.mem.u8(AUDIO_DEBUG_INFORMATION_SETTING) != 0;
            if sound != 0 {
                let duration = e.mem.u32(sound + SOUND_DURATION);
                let sound_type = e.mem.u32(sound + SOUND_TYPE_FLAGS);
                let sound_id = e.mem.u32(sound + SOUND_ID);
                let volume = e.vcall(sound, SOUND_SLOT_GET_VOLUME, &[]).f32();
                let playing = e.vcall(sound, SOUND_SLOT_IS_PLAYING, &[]).u8();
                e.call(DEBUG_SOUND_INFO_CONSTRUCTOR, &[temporary + 0x18]);
                e.mem.set_u32(temporary, sound_id);
                e.mem.set_f32(temporary + 4, 1.0);
                e.mem.set_f32(temporary + 8, volume);
                e.mem.set_u32(temporary + 0xc, sound_type);
                e.mem.set_u32(temporary + 0x10, duration);
                e.mem.set_u8(temporary + 0x14, playing);
                e.call(SOUND_INFO_ASSIGN, &[info, temporary]);
                e.call(SOUND_INFO_DESTRUCTOR, &[temporary]);
                if debug {
                    fill_debug_record(e, debug_a, sound);
                }
            }
            let section = this.addr() + 0xc4;
            enter_section(e, section);
            let updates = this.at(BSAudioManager::StateUpdateMap);
            let (found, existing) = map_get(e, updates, id, 0);
            if found {
                if debug {
                    assign_debug_record(e, existing, debug_a);
                }
                let record = Ptr::<BSSoundInfo>::new(existing);
                let source = Ptr::<BSSoundInfo>::new(info);
                let value = e.get(source, BSSoundInfo::iID);
                e.set(record, BSSoundInfo::iID, value);
                let value = x87_quiet(e.get(source, BSSoundInfo::fFrequency));
                e.set(record, BSSoundInfo::fFrequency, value);
                let value = x87_quiet(e.get(source, BSSoundInfo::fVolume));
                e.set(record, BSSoundInfo::fVolume, value);
                let value = e.get(source, BSSoundInfo::uiSoundType);
                e.set(record, BSSoundInfo::uiSoundType, value);
                let value = e.get(source, BSSoundInfo::uiDuration);
                e.set(record, BSSoundInfo::uiDuration, value);
                let value = e.get(source, BSSoundInfo::bIsPlaying);
                e.set(record, BSSoundInfo::bIsPlaying, value);
            } else {
                let raw = e.call(ALLOCATE, &[MEMORY_MANAGER, 0x254]).u32();
                let created = if raw != 0 {
                    e.call(SOUND_INFO_COPY_CONSTRUCTOR, &[raw, info]).u32()
                } else {
                    0
                };
                if debug {
                    assign_debug_record(e, created, debug_a);
                }
                e.call(MAP_SET_VALUE, &[updates.addr(), id, created]);
            }
            leave_section(e, section);
            e.call(DEBUG_SOUND_INFO_DESTRUCTOR, &[debug_b]);
            e.call(DEBUG_SOUND_INFO_DESTRUCTOR, &[debug_a.addr()]);
            e.call(SOUND_INFO_DESTRUCTOR, &[info]);
        });
    });
}

// Translated from 00adac30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The volume a sound of type mask `flags` plays at (unnamed in the engine
/// map): `volume` times the master volume, its secondary volume and the
/// `fMasterVolumeMult` setting, then times the volume and secondary volume
/// of the category the mask selects: music for 0x800000 (volume then
/// secondary), foot (8), voice (4), music (0x800), radio (0x100000), else
/// effects.
pub fn fn_00adac30(e: &mut Engine, this: Ptr<BSAudioManager>, volume: f32, flags: u32) -> f32 {
    let master = e.get(this, BSAudioManager::fMasterVolume) as f64;
    let master_secondary = e.get(this, BSAudioManager::fMasterSecondaryVolume) as f64;
    let multiplier = e.global::<f32>(MASTER_VOLUME_MULT) as f64;
    let scaled = (((master * master_secondary) * multiplier) * volume as f64) as f32;
    let category = |e: &Engine,
                    volume_field: Field<BSAudioManager, f32>,
                    secondary: Field<BSAudioManager, f32>| {
        (
            e.get(this, volume_field) as f64,
            e.get(this, secondary) as f64,
        )
    };
    if flags & 0x80_0000 != 0 {
        let (music, music_secondary) = category(
            e,
            BSAudioManager::fMusicVolume,
            BSAudioManager::fMusicSecondaryVolume,
        );
        return ((scaled as f64 * music) * music_secondary) as f32;
    }
    let (category_volume, category_secondary) = if flags & 8 != 0 {
        category(
            e,
            BSAudioManager::fFootVolume,
            BSAudioManager::fFootSecondaryVolume,
        )
    } else if flags & 4 != 0 {
        category(
            e,
            BSAudioManager::fVoiceVolume,
            BSAudioManager::fVoiceSecondaryVolume,
        )
    } else if flags & 0x800 != 0 {
        category(
            e,
            BSAudioManager::fMusicVolume,
            BSAudioManager::fMusicSecondaryVolume,
        )
    } else if flags & 0x10_0000 != 0 {
        category(
            e,
            BSAudioManager::fRadioVolume,
            BSAudioManager::fRadioSecondaryVolume,
        )
    } else {
        category(
            e,
            BSAudioManager::fEffectsVolume,
            BSAudioManager::fEffectsSecondaryVolume,
        )
    };
    ((category_volume * category_secondary) * scaled as f64) as f32
}

// Translated from 00adad60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes a message from the free-message stack (unnamed in the engine map):
/// pops `FreeMessages` (+0x24) and returns the message, or a new one when
/// the stack is empty and bottomless.
pub fn fn_00adad60(e: &mut Engine, this: Ptr<BSAudioManager>) -> Ptr<BSSoundMessage> {
    e.call(MESSAGE_STACK_POP, &[this.addr() + 0x24]).ptr()
}

/// The common part of `FreeMessage` and `FreeQueueMessage`: empties the
/// message and pushes it on the stack at `stack`.
fn free_message_to(e: &mut Engine, stack: u32, message: Ptr<BSSoundMessage>) {
    let message_address = message.addr();
    let (x, y, z) = default_message_vector(e);
    // A smart-pointer temporary (a local the game keeps on its stack) takes
    // over the message's pointer; it lives to the end of the function.
    e.with_stack(4, |e, held| {
        e.call(SMART_POINTER_ASSIGN, &[held.addr(), message_address + 0x10]);
        let object = e.mem.u32(held.addr());
        if object != 0 {
            // The message lets go of the object, which is then handed on.
            e.with_stack(4, |e, empty| {
                e.call(
                    SMART_POINTER_ASSIGN,
                    &[message_address + 0x10, empty.addr()],
                );
                e.call(SMART_POINTER_DESTRUCTOR, &[empty.addr()]);
            });
            e.call(RELEASE_MESSAGE_POINTER, &[object]);
            e.call(SMART_POINTER_SET, &[held.addr(), 0]);
        }
        e.set(message, BSSoundMessage::msg, 0);
        e.set(message, BSSoundMessage::iID, 0);
        e.set(message, BSSoundMessage::iData, 0);
        e.set(message, BSSoundMessage::pData, 0);
        // The cleared message takes a null smart pointer (its previous
        // pointer is released by the assignment).
        e.with_stack(4, |e, empty| {
            e.call(
                SMART_POINTER_ASSIGN,
                &[message_address + 0x10, empty.addr()],
            );
        });
        e.set(message, BSSoundMessage::vector3_x, x87_quiet(x));
        e.set(message, BSSoundMessage::vector3_y, x87_quiet(y));
        e.set(message, BSSoundMessage::vector3_z, x87_quiet(z));
        e.call(MESSAGE_STACK_PUSH, &[stack, message_address]);
        // End of the temporary's scope.
        let remaining = e.mem.u32(held.addr());
        drop_reference(e, remaining);
    });
}

// Translated from 00adad80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::FreeMessage` (Xbox PDB): clears a message (releasing the
/// object its smart pointer held) and pushes it on `FreeMessages`.
// Translation incomplete: 00adad80 - C++ exception unwinding (SEH frame state) is not translated.
pub fn bs_audio_manager_free_message(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    free_message_to(e, this.addr() + 0x24, message);
}

// Translated from 00adaf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes a message from the free-queue-message stack (unnamed in the engine
/// map): pops `FreeQueueMessages` (+0x34).
pub fn fn_00adaf80(e: &mut Engine, this: Ptr<BSAudioManager>) -> Ptr<BSSoundMessage> {
    e.call(MESSAGE_STACK_POP, &[this.addr() + 0x34]).ptr()
}

// Translated from 00adafa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::FreeQueueMessage` (Xbox PDB): the same as `FreeMessage`
/// for the `FreeQueueMessages` stack (+0x34).
// Translation incomplete: 00adafa0 - C++ exception unwinding (SEH frame state) is not translated.
pub fn bs_audio_manager_free_queue_message(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    free_message_to(e, this.addr() + 0x34, message);
}

// Translated from 00adb1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues a message (unnamed in the engine map; the callers' common step):
/// when the audio is initialized (byte +5 of the `BSAudio` object), takes a
/// message from `FreeMessages` under the message critical section, fills it
/// with the type, the three words, the smart pointer and the three floats,
/// and appends it to the message list `iMessageListIndex` selects.
/// `pointer` is passed by value as a smart pointer: the message takes its
/// own reference and the argument's reference is dropped on return.
// Translation incomplete: 00adb1a0 - C++ exception unwinding (SEH frame state) is not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_00adb1a0(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    kind: u32,
    a: u32,
    b: u32,
    c: u32,
    pointer: u32,
    x: f32,
    y: f32,
    z: f32,
) {
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    if e.mem.u8(audio + 5) != 0 {
        in_audio_context(e, |e| {
            let section = this.addr() + 0xac;
            enter_section(e, section);
            let message = fn_00adad60(e, this);
            if !message.is_null() {
                // The smart-pointer temporary built from the argument.
                e.with_stack(4, |e, temporary| {
                    e.mem.set_u32(temporary.addr(), pointer);
                    add_reference(e, pointer);
                    e.set(message, BSSoundMessage::msg, kind);
                    e.set(message, BSSoundMessage::iID, a);
                    e.set(message, BSSoundMessage::iData, b);
                    e.set(message, BSSoundMessage::pData, c);
                    e.call(
                        SMART_POINTER_ASSIGN,
                        &[message.addr() + 0x10, temporary.addr()],
                    );
                    e.set(message, BSSoundMessage::vector3_x, x87_quiet(x));
                    e.set(message, BSSoundMessage::vector3_y, x87_quiet(y));
                    e.set(message, BSSoundMessage::vector3_z, x87_quiet(z));
                    let held = e.mem.u32(temporary.addr());
                    drop_reference(e, held);
                });
                append_message(e, this, message);
            }
            leave_section(e, section);
        });
    }
    drop_reference(e, pointer);
}

/// Appends `message` to the message list `iMessageListIndex` selects.
fn append_message(e: &mut Engine, this: Ptr<BSAudioManager>, message: Ptr<BSSoundMessage>) {
    let index = e.get(this, BSAudioManager::iMessageListIndex);
    let list = this.addr() + 0x0c + index.wrapping_mul(0x0c);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), message.addr());
        e.call(LIST_APPEND_POINTER, &[list, slot.addr()]);
    });
}

// Translated from 00adb3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Play` (Xbox PDB): unless the sound is already playing,
/// queues message 0x2d (when `flag` is set) and message 1 for it. Always
/// true.
pub fn bs_audio_manager_play(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, flag: u8) -> bool {
    if bs_audio_manager_is_playing(e, this, id) {
        return true;
    }
    if flag != 0 {
        queue_plain(e, this, 0x2d, id, 0, 0);
    }
    queue_plain(e, this, 0x01, id, 0, 0);
    true
}

// Translated from 00adb4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::PlayAfter` (Xbox PDB): unless the sound is already
/// playing, queues message 5 with the time `delay` milliseconds (at most
/// 10000) after `GetTickCount`. Always true. The third stack word is not
/// read.
pub fn bs_audio_manager_play_after(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    delay: u32,
    _unused_3: u32,
) -> bool {
    if bs_audio_manager_is_playing(e, this, id) {
        return true;
    }
    let delay = delay.min(10000);
    let now = e.call(IMPORT_GET_TICK_COUNT, &[]).u32();
    queue_plain(e, this, 0x05, id, now.wrapping_add(delay), 0);
    true
}

// Translated from 00adb590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Pause` (Xbox PDB): queues message 3 for the sound.
/// Returns false.
pub fn bs_audio_manager_pause(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) -> bool {
    queue_plain(e, this, 0x03, id, 0, 0);
    false
}

// Translated from 00adb610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Stop` (Xbox PDB): queues message 4 for the sound.
/// Returns false.
pub fn bs_audio_manager_stop(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) -> bool {
    queue_plain(e, this, 0x04, id, 0, 0);
    false
}

// Translated from 00adb690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::StopMovingSounds` (Xbox PDB): queues message 0x29 with
/// the object (a reference-counted object; the call takes a reference for
/// the smart-pointer argument), `seconds` converted to milliseconds
/// (`_ftol2` of seconds times the double at `01017b70`) and the last word.
pub fn bs_audio_manager_stop_moving_sounds(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    object: u32,
    seconds: f32,
    value: u32,
) {
    let (x, y, z) = default_message_vector(e);
    add_reference(e, object);
    let milliseconds = scaled_milliseconds_ftol(e, seconds);
    fn_00adb1a0(e, this, 0x29, 0, milliseconds, value, object, x, y, z);
}

// Translated from 00adb720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetStaticAttenuation` (Xbox PDB): queues message 0x21
/// with the attenuation limited to 10000. Returns false.
pub fn bs_audio_manager_set_static_attenuation(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    attenuation: u16,
) -> bool {
    let attenuation = attenuation.min(10000) as u32;
    queue_plain(e, this, 0x21, id, attenuation, 0);
    false
}

// Translated from 00adb7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x13 with `seconds` converted to milliseconds through
/// `_ftol2` (unnamed in the engine map). Returns false.
pub fn fn_00adb7c0(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, seconds: f32) -> bool {
    let milliseconds = scaled_milliseconds_ftol(e, seconds);
    queue_plain(e, this, 0x13, id, milliseconds, 0);
    false
}

// Translated from 00adb850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetModFrequency` (Xbox PDB): queues message 0x42 with
/// the value times 1000 through `_ftol2`. Returns false.
pub fn bs_audio_manager_set_mod_frequency(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    value: f32,
) -> bool {
    let scaled = scaled_milliseconds_ftol(e, value);
    queue_plain(e, this, 0x42, id, scaled, 0);
    false
}

// Translated from 00adb8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x14 with a byte (unnamed in the engine map). Returns
/// false.
pub fn fn_00adb8e0(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, flag: u8) -> bool {
    queue_plain(e, this, 0x14, id, flag as u32, 0);
    false
}

// Translated from 00adb960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Does nothing and returns false (unnamed in the engine map); both stack
/// words are ignored.
pub fn fn_00adb960(
    _e: &mut Engine,
    _this: Ptr<BSAudioManager>,
    _unused_1: u32,
    _unused_2: u32,
) -> bool {
    false
}

/// Queues a message whose three floats are `x`, `y`, `z` instead of the
/// default vector (`SetPosition` and its sibling).
fn queue_with_vector(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    kind: u32,
    id: u32,
    (x, y, z): (f32, f32, f32),
) {
    fn_00adb1a0(e, this, kind, id, 0, 0, 0, x, y, z);
}

// Translated from 00adb970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetPosition` (Xbox PDB): queues message 0x1e with the
/// three coordinates as the message vector. Returns false.
pub fn bs_audio_manager_set_position(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    x: f32,
    y: f32,
    z: f32,
) -> bool {
    queue_with_vector(e, this, 0x1e, id, (x, y, z));
    false
}

// Translated from 00adb9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x1f with three floats as the message vector (unnamed in
/// the engine map). Returns false.
pub fn fn_00adb9f0(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    x: f32,
    y: f32,
    z: f32,
) -> bool {
    queue_with_vector(e, this, 0x1f, id, (x, y, z));
    false
}

// Translated from 00adba70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetMinMax` (Xbox PDB): queues message 0x23 with the
/// minimum and maximum distances times 1000, each truncated by an inline
/// `FISTP` (low 32 bits of the 64-bit result).
pub fn bs_audio_manager_set_min_max(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    minimum: f32,
    maximum: f32,
) {
    let maximum = scaled_milliseconds_inline(e, maximum);
    let minimum = scaled_milliseconds_inline(e, minimum);
    queue_plain(e, this, 0x23, id, minimum, maximum);
}

// Translated from 00adbb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x24 with five 16-bit values, each limited to 100
/// (unnamed in the engine map). Takes a message from `FreeMessages`,
/// builds a local message with the type and the sound id, assigns it to the
/// pooled message (`00ae7990`), destroys the local one, stores the five
/// limited values over the message's vector words (`curve[0..5]`) and
/// appends the message to the current list.
// Translation incomplete: 00adbb30 - C++ exception unwinding (SEH frame state) and the stack cookie check are not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_00adbb30(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    value_0: u16,
    value_1: u16,
    value_2: u16,
    value_3: u16,
    value_4: u16,
) {
    let values = [value_0, value_1, value_2, value_3, value_4].map(|value| value.min(100));
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    if e.mem.u8(audio + 5) == 0 {
        return;
    }
    in_audio_context(e, |e| {
        let section = this.addr() + 0xac;
        enter_section(e, section);
        let message = fn_00adad60(e, this);
        let (x, y, z) = default_message_vector(e);
        // Locals: the null smart pointer that is assigned into the local
        // message, and the local message itself.
        e.with_stack(4 + 0x20, |e, frame| {
            let pointer_slot = frame.addr();
            let local = frame.addr() + 4;
            e.mem.set_u32(local, 0x24);
            e.mem.set_u32(local + 4, id);
            e.mem.set_u32(local + 8, 0);
            e.mem.set_u32(local + 0xc, 0);
            e.call(SMART_POINTER_ASSIGN, &[local + 0x10, pointer_slot]);
            e.mem.set_f32(local + 0x14, x87_quiet(x));
            e.mem.set_f32(local + 0x18, x87_quiet(y));
            e.mem.set_f32(local + 0x1c, x87_quiet(z));
            e.call(SMART_POINTER_DESTRUCTOR, &[pointer_slot]);
            e.call(MESSAGE_ASSIGN, &[message.addr(), local]);
            e.call(MESSAGE_DESTRUCTOR, &[local]);
        });
        e.set(message, BSSoundMessage::curve_0, values[0]);
        e.set(message, BSSoundMessage::curve_1, values[1]);
        e.set(message, BSSoundMessage::curve_2, values[2]);
        e.set(message, BSSoundMessage::curve_3, values[3]);
        e.set(message, BSSoundMessage::curve_4, values[4]);
        append_message(e, this, message);
        leave_section(e, section);
    });
}

// Translated from 00adbec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x22 with a 16-bit value (unnamed in the engine map).
pub fn fn_00adbec0(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, value: u16) {
    queue_plain(e, this, 0x22, id, value as u32, 0);
}

// Translated from 00adbf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Does nothing and returns false (unnamed in the engine map); all four
/// stack words are ignored.
pub fn fn_00adbf40(
    _e: &mut Engine,
    _this: Ptr<BSAudioManager>,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> bool {
    false
}

// Translated from 00adbf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Release` (Xbox PDB): queues message 0x26 for the sound.
/// Returns true.
pub fn bs_audio_manager_release(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) -> bool {
    queue_plain(e, this, 0x26, id, 0, 0);
    true
}

// Translated from 00adbfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetCompletionCallback` (Xbox PDB): queues message 0x3a
/// with the first stack word after the sound id as the message's third
/// word and the second as its second. Returns true.
pub fn bs_audio_manager_set_completion_callback(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    first: u32,
    second: u32,
) -> bool {
    queue_plain(e, this, 0x3a, id, second, first);
    true
}

// Translated from 00adc050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x3b with the two words laid out like
/// `SetCompletionCallback` (unnamed in the engine map). Returns true.
pub fn fn_00adc050(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    first: u32,
    second: u32,
) -> bool {
    queue_plain(e, this, 0x3b, id, second, first);
    true
}

// Translated from 00adc0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetVolume` (Xbox PDB): queues message 0x10 with the
/// volume times 1000 through `_ftol2`. Returns true.
pub fn bs_audio_manager_set_volume(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    volume: f32,
) -> bool {
    let scaled = scaled_milliseconds_ftol(e, volume);
    queue_plain(e, this, 0x10, id, scaled, 0);
    true
}

// Translated from 00adc160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::VolumeChanged` (Xbox PDB): queues message 0x0e with no
/// data.
pub fn bs_audio_manager_volume_changed(e: &mut Engine, this: Ptr<BSAudioManager>) {
    queue_plain(e, this, 0x0e, 0, 0, 0);
}

// Translated from 00adc1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetObjectToFollow` (Xbox PDB): queues message 0x27 with
/// the object as the message's smart pointer (a reference is taken for the
/// argument).
pub fn bs_audio_manager_set_object_to_follow(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    object: u32,
) {
    let (x, y, z) = default_message_vector(e);
    add_reference(e, object);
    fn_00adb1a0(e, this, 0x27, id, 0, 0, object, x, y, z);
}

// Translated from 00adc260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x3f with a word (unnamed in the engine map).
pub fn fn_00adc260(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, value: u32) {
    queue_plain(e, this, 0x3f, id, value, 0);
}

// Translated from 00adc2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x40 with a byte (unnamed in the engine map).
pub fn fn_00adc2e0(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, flag: u8) {
    queue_plain(e, this, 0x40, id, flag as u32, 0);
}

// Translated from 00adc360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetSynchStartTime` (Xbox PDB): queues message 0x3c with
/// the time in both the second and third word.
pub fn bs_audio_manager_set_synch_start_time(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    time: u32,
) {
    queue_plain(e, this, 0x3c, id, time, time);
}

// Translated from 00adc3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x3d for the sound (unnamed in the engine map).
pub fn fn_00adc3e0(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) {
    queue_plain(e, this, 0x3d, id, 0, 0);
}

// Translated from 00adc460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetPriority` (Xbox PDB): queues message 0x41 with the
/// priority.
pub fn bs_audio_manager_set_priority(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    priority: u32,
) {
    queue_plain(e, this, 0x41, id, priority, 0);
}

// Translated from 00adc4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x3e with two bytes (unnamed in the engine map): the
/// first as the second word, the second as the third.
pub fn fn_00adc4e0(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, first: u8, second: u8) {
    queue_plain(e, this, 0x3e, id, first as u32, second as u32);
}

// Translated from 00adc560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::FadeOut` (Xbox PDB): queues message 9 with `value` as
/// the third word and `time`, converted to a float, as the first of the
/// message's three floats (the other two are 0.0).
pub fn bs_audio_manager_fade_out(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    value: u32,
    time: u32,
) {
    fn_00adb1a0(e, this, 0x09, id, 0, value, 0, time as f32, 0.0, 0.0);
}

// Translated from 00adc5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x0b with a word as the third word (unnamed in the engine
/// map). The third stack word is not read.
pub fn fn_00adc5f0(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, value: u32, _unused_3: u32) {
    queue_plain(e, this, 0x0b, id, 0, value);
}

// Translated from 00adc670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unless the sound is already playing, queues messages 7 and 1 for it and
/// then message 0x0b with `value` as the third word (unnamed in the engine
/// map).
pub fn fn_00adc670(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, value: u32) {
    if bs_audio_manager_is_playing(e, this, id) {
        return;
    }
    queue_plain(e, this, 0x07, id, 0, 0);
    queue_plain(e, this, 0x01, id, 0, 0);
    queue_plain(e, this, 0x0b, id, 0, value);
}

// Translated from 00adc7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::IsValid` (Xbox PDB): whether the sound state map has a
/// record for `id` whose own id is `id`.
pub fn bs_audio_manager_is_valid(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) -> bool {
    let (_found, record) = map_get(e, this.at(BSAudioManager::SoundStateMap), id, 0);
    record != 0 && e.mem.u32(record) == id
}

// Translated from 00adc830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::GetDuration` (Xbox PDB): the duration in the sound's
/// state record (+0x10), 0 when there is none. Reads the map under the
/// state critical section.
pub fn bs_audio_manager_get_duration(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) -> u32 {
    let section = this.addr() + 0xc4;
    enter_section(e, section);
    let (_found, record) = map_get(e, this.at(BSAudioManager::SoundStateMap), id, 0);
    leave_section(e, section);
    if record != 0 {
        e.get(Ptr::<BSSoundInfo>::new(record), BSSoundInfo::uiDuration)
    } else {
        0
    }
}

// Translated from 00adc8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::GetVolume` (Xbox PDB): the volume in the sound's state
/// record (+8), 0.0 when there is none. Reads the map under the state
/// critical section.
pub fn bs_audio_manager_get_volume(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) -> f32 {
    let section = this.addr() + 0xc4;
    enter_section(e, section);
    let (_found, record) = map_get(e, this.at(BSAudioManager::SoundStateMap), id, 0);
    leave_section(e, section);
    if record != 0 {
        x87_quiet(e.get(Ptr::<BSSoundInfo>::new(record), BSSoundInfo::fVolume))
    } else {
        0.0
    }
}

// Translated from 00adc910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::IsPlaying` (Xbox PDB): whether the sound's state
/// record says it is playing. Reads the map under the state critical
/// section.
pub fn bs_audio_manager_is_playing(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) -> bool {
    let section = this.addr() + 0xc4;
    enter_section(e, section);
    let (found, record) = map_get(e, this.at(BSAudioManager::SoundStateMap), id, 0);
    let playing = found
        && record != 0
        && e.get(Ptr::<BSSoundInfo>::new(record), BSSoundInfo::bIsPlaying) != 0;
    leave_section(e, section);
    playing
}

// Translated from 00adc9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sound type word (+0xc) of the sound's state record, 0 when there is
/// none (unnamed in the engine map). Reads the map under the state critical
/// section.
pub fn fn_00adc9a0(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) -> u32 {
    let section = this.addr() + 0xc4;
    enter_section(e, section);
    let (found, record) = map_get(e, this.at(BSAudioManager::SoundStateMap), id, 0);
    let value = if found && record != 0 {
        e.get(Ptr::<BSSoundInfo>::new(record), BSSoundInfo::uiSoundType)
    } else {
        0
    };
    leave_section(e, section);
    value
}

// Translated from 00adca20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::MuteAllOfType` (Xbox PDB): queues message 0x18 with the
/// third stack word as the first message word and the first two as the
/// second and third.
pub fn bs_audio_manager_mute_all_of_type(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    first: u32,
    second: u32,
    third: u32,
) {
    queue_plain(e, this, 0x18, third, first, second);
}

// Translated from 00adcaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x19 with the two words as the second and third message
/// word (unnamed in the engine map).
pub fn fn_00adcaa0(e: &mut Engine, this: Ptr<BSAudioManager>, first: u32, second: u32) {
    queue_plain(e, this, 0x19, 0, first, second);
}

// Translated from 00adcb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x1c when `flag` is set and 0x1a otherwise, with `value`
/// as the second message word (unnamed in the engine map).
pub fn fn_00adcb20(e: &mut Engine, this: Ptr<BSAudioManager>, value: u32, flag: u8) {
    let kind = if flag != 0 { 0x1c } else { 0x1a };
    queue_plain(e, this, kind, 0, value, 0);
}

// Translated from 00adcc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x1d when `flag` is set and 0x1b otherwise, with `value`
/// as the second message word (unnamed in the engine map).
pub fn fn_00adcc10(e: &mut Engine, this: Ptr<BSAudioManager>, value: u32, flag: u8) {
    let kind = if flag != 0 { 0x1d } else { 0x1b };
    queue_plain(e, this, kind, 0, value, 0);
}

// Translated from 00adcd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::KillAllOfType` (Xbox PDB): queues message 0x2a with the
/// two words as the second and third message word.
pub fn bs_audio_manager_kill_all_of_type(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    first: u32,
    second: u32,
) {
    queue_plain(e, this, 0x2a, 0, first, second);
}

// Translated from 00adcd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cache upkeep after a sound has been prepared (unnamed in the engine
/// map). Under the cache critical section (+0xdc): with the cache setting
/// (`bEnableAudioCache`) off, deletes every cached sound; with it on, keeps
/// `sound` only when it is not larger than `iMaxSizeForCachedSound` (KB),
/// does not have type bit 0x10000 and no cached sound has its hash. A
/// copy of it (made through the `BSAudio` slot 0x14 and the sound's `Copy`,
/// slot 0x2c, with state bit 0x8000 added) is put at the head of the cached
/// list and counted into `iCurrentCachSize`; then sounds are removed from the
/// tail until the size fits `iAudioCacheSize` (KB) or the list is empty,
/// putting back at the head (at most 31 times) those whose type bits
/// 0x20000 or 0x20000000 are set.
pub fn fn_00adcd80(e: &mut Engine, this: Ptr<BSAudioManager>, sound: u32) {
    let section = this.addr() + 0xdc;
    let list = this.addr() + 0x94;
    enter_section(e, section);
    let cache_enabled = e.mem.u8(CACHE_ENABLED_SETTING) != 0;
    if !cache_enabled && (e.mem.u32(list + 8) as i32) > 0 {
        while e.mem.u32(list + 8) != 0 {
            let removed = e.call(LIST_REMOVE_HEAD, &[list]).u32();
            if removed != 0 {
                e.vcall(removed, SOUND_SLOT_DESTRUCTOR, &[1]);
            }
        }
        leave_section(e, section);
        return;
    }
    let size_limit = e.mem.u32(MAX_CACHED_SOUND_SIZE_SETTING) << 10;
    if !cache_enabled
        || sound == 0
        || e.mem.u32(sound + SOUND_SIZE) > size_limit
        || e.mem.u32(sound + SOUND_TYPE_FLAGS) & 0x10000 != 0
    {
        leave_section(e, section);
        return;
    }
    let mut node = e.mem.u32(list);
    while node != 0 {
        let cached = e.mem.u32(node + 8);
        if e.mem.u32(sound + SOUND_HASH) == e.mem.u32(cached + SOUND_HASH) {
            leave_section(e, section);
            return;
        }
        node = e.mem.u32(node);
    }
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    let copy = e.vcall(audio, AUDIO_SLOT_CREATE_NEW_GAME_SOUND, &[0]).u32();
    let type_flags = e.mem.u32(sound + SOUND_TYPE_FLAGS);
    e.mem.set_u32(copy + SOUND_TYPE_FLAGS, type_flags);
    e.vcall(copy, SOUND_SLOT_COPY, &[sound, 0]);
    let type_flags = e.mem.u32(sound + SOUND_TYPE_FLAGS);
    e.mem.set_u32(copy + SOUND_TYPE_FLAGS, type_flags | 0x8000);
    list_add_head(e, list, copy);
    if e.mem.u32(sound + SOUND_TYPE_FLAGS) & 0x2000_0000 == 0 {
        let size = e.mem.u32(copy + SOUND_SIZE);
        let total = e.get(this, BSAudioManager::iCurrentCachSize);
        e.set(
            this,
            BSAudioManager::iCurrentCachSize,
            total.wrapping_add(size),
        );
    }
    let mut round = 0i32;
    loop {
        let limit = e.mem.u32(AUDIO_CACHE_SIZE_SETTING) << 10;
        if e.get(this, BSAudioManager::iCurrentCachSize) <= limit || e.mem.u32(list + 8) == 0 {
            break;
        }
        let item = e.call(LIST_POP_TAIL, &[list]).u32();
        let protected = item != 0 && {
            let flags = e.mem.u32(item + SOUND_TYPE_FLAGS);
            (flags & 0x2_0000 != 0 || flags & 0x2000_0000 != 0) && round <= 0x1e
        };
        if item != 0 && !protected {
            let size = e.mem.u32(item + SOUND_SIZE);
            let total = e.get(this, BSAudioManager::iCurrentCachSize);
            e.set(
                this,
                BSAudioManager::iCurrentCachSize,
                total.wrapping_sub(size),
            );
            e.vcall(item, SOUND_SLOT_DESTRUCTOR, &[1]);
        } else if item != 0 {
            list_add_head(e, list, item);
        }
        round += 1;
    }
    leave_section(e, section);
}

// Translated from 00add1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes a sound out of the cache and hands back a fresh copy of it
/// (unnamed in the engine map; probably the cache lookup the PDB calls
/// `GetFromCache`, unconfirmed). Under the cache critical section: a cached
/// sound with `sound_hash` wins at once; otherwise, when `sound_hash` is 0
/// and `directory_hash` is not, one of the sounds with that directory hash
/// is picked (the last one in the list, or, by a `rand` draw, the one before
/// it). The chosen sound is removed from the list; a new sound is created
/// through `BSAudio` slot 0x14, given `type_flags`, and made a copy of the
/// chosen one (`Copy`, slot 0x2c). An exact hit goes back to the head of the
/// list unless its type bit 0x20000000 is set (then it is deleted). Returns
/// the new sound, or 0.
pub fn fn_00add1e0(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    sound_hash: u32,
    directory_hash: u32,
    type_flags: u32,
) -> u32 {
    let section = this.addr() + 0xdc;
    let list = this.addr() + 0x94;
    let pick_by_directory = directory_hash != 0 && sound_hash == 0;
    enter_section(e, section);
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    let mut node = e.mem.u32(list);
    // The last directory match and the one before it, with their nodes.
    let (mut best_sound, mut best_node) = (0u32, 0u32);
    let (mut other_sound, mut other_node) = (0u32, 0u32);
    while node != 0 {
        let sound = e.mem.u32(node + 8);
        if sound_hash == e.mem.u32(sound + SOUND_HASH) {
            let copy = e.vcall(audio, AUDIO_SLOT_CREATE_NEW_GAME_SOUND, &[0]).u32();
            e.mem.set_u32(copy + SOUND_TYPE_FLAGS, type_flags);
            e.vcall(copy, SOUND_SLOT_COPY, &[sound, 1]);
            e.mem.set_u32(copy + SOUND_TYPE_FLAGS, type_flags);
            list_remove_node(e, list, node);
            if e.mem.u32(sound + SOUND_TYPE_FLAGS) & 0x2000_0000 == 0 {
                list_add_head(e, list, sound);
            } else {
                e.vcall(sound, SOUND_SLOT_DESTRUCTOR, &[1]);
            }
            leave_section(e, section);
            return copy;
        }
        let directory = e.mem.u32(sound + SOUND_DIRECTORY_HASH);
        if directory_hash == directory && directory != 0 {
            if best_sound != 0 && pick_by_directory && e.call(RAND, &[]).i32() % 3 == 0 {
                other_sound = best_sound;
                other_node = best_node;
            }
            best_sound = sound;
            best_node = node;
        }
        node = e.mem.u32(node);
    }
    if pick_by_directory && other_sound != 0 && best_sound != 0 && e.call(RAND, &[]).i32() % 2 == 0
    {
        best_sound = other_sound;
        best_node = other_node;
    }
    let mut copy = 0;
    if best_sound != 0 {
        list_remove_node(e, list, best_node);
        list_add_head(e, list, best_sound);
        copy = e.vcall(audio, AUDIO_SLOT_CREATE_NEW_GAME_SOUND, &[0]).u32();
        e.mem.set_u32(copy + SOUND_TYPE_FLAGS, type_flags);
        e.vcall(copy, SOUND_SLOT_COPY, &[best_sound, 1]);
        e.mem.set_u32(copy + SOUND_TYPE_FLAGS, type_flags);
    }
    leave_section(e, section);
    copy
}

// Translated from 00add480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::IsInCache` (Xbox PDB): whether a cached sound has the
/// hash `hash`; with `match_directory` also whether one has it as its
/// (non-zero) directory hash. A zero hash is never in the cache. Under the
/// cache critical section.
pub fn bs_audio_manager_is_in_cache(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    hash: u32,
    match_directory: u8,
) -> bool {
    if hash == 0 {
        return false;
    }
    let section = this.addr() + 0xdc;
    enter_section(e, section);
    let mut found = false;
    let mut node = e.mem.u32(this.addr() + 0x94);
    while node != 0 {
        let sound = e.mem.u32(node + 8);
        if sound != 0 {
            if hash == e.mem.u32(sound + SOUND_HASH) {
                found = true;
                break;
            }
            let directory = e.mem.u32(sound + SOUND_DIRECTORY_HASH);
            if match_directory != 0 && hash == directory && directory != 0 {
                found = true;
                break;
            }
        }
        node = e.mem.u32(node);
    }
    leave_section(e, section);
    found
}

/// The shared start of `Precache` and `Precache_ov2`: when audio is
/// enabled and `BSAudio` has the callback at `callback_offset`, calls it
/// with the name, a 264-byte buffer, the address of `flags` and the address
/// of an extra word (initially 0). Returns `(flags, extra)` when the
/// callback answered true; the buffer's address is the first element.
fn precache_callback(
    e: &mut Engine,
    callback_offset: u32,
    name: u32,
    flags: u32,
    run: impl FnOnce(&mut Engine, u32, u32, u32),
) {
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    let callback = e.mem.u32(audio + callback_offset);
    if e.mem.u8(audio + AUDIO_ENABLED) == 0 || callback == 0 {
        return;
    }
    e.with_stack(0x108 + 8, |e, block| {
        let buffer = block.addr();
        let flags_slot = buffer + 0x108;
        let extra_slot = buffer + 0x10c;
        e.mem.set_u32(flags_slot, flags);
        e.mem.set_u32(extra_slot, 0);
        if e.call(callback, &[name, buffer, flags_slot, extra_slot])
            .bool()
        {
            let flags = e.mem.u32(flags_slot);
            let extra = e.mem.u32(extra_slot);
            run(e, buffer, flags, extra);
        }
    });
}

// Translated from 00add570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Precache` (Xbox PDB): when audio is enabled and
/// `BSAudio` has a name callback (`pfnNameCallback`, +0x24), asks it about
/// `name` (it fills a 264-byte buffer and a word); when it answers true,
/// precaches the buffer with the caller's `flags` (`Precache_ov3`).
// Translation incomplete: 00add570 - the /GS stack cookie check (00ec408c) is not translated.
pub fn bs_audio_manager_precache(e: &mut Engine, this: Ptr<BSAudioManager>, name: u32, flags: u32) {
    precache_callback(
        e,
        AUDIO_NAME_CALLBACK,
        name,
        flags,
        |e, buffer, _, extra| {
            bs_audio_manager_precache_ov3(e, this, buffer, flags, extra);
        },
    );
}

// Translated from 00add630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Precache_ov2` (Xbox PDB): like `Precache` through the
/// ID callback (`pfnIDCallback`, +0x20); the flags precached are the
/// caller's `flags` or-ed with the ones the callback wrote back.
// Translation incomplete: 00add630 - the /GS stack cookie check (00ec408c) is not translated.
pub fn bs_audio_manager_precache_ov2(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    name: u32,
    flags: u32,
) {
    precache_callback(
        e,
        AUDIO_ID_CALLBACK,
        name,
        flags,
        |e, buffer, written, extra| {
            bs_audio_manager_precache_ov3(e, this, buffer, flags | written, extra);
        },
    );
}

// Translated from 00add6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Precache_ov3` (Xbox PDB): when audio is enabled and
/// `name` is not null, takes the next sound id (`iNextID`), copies the name
/// to a 260-byte buffer and lets `BSAudio` fix the path (slot 0x18). Unless a
/// sound with that name's hash is cached already (`IsInCache`, directory
/// hashes included), creates a sound for `name` (slot 0x14), sets its id and
/// type flags, calculates its hashes, lets the settings callback
/// (`BSAudio` +0x2c) fill a word from the id record and `extra`, copies the
/// loop points the loop-point callback (slot 0x10) reports for `extra` into
/// the sound (+0x174, +0x178), stores the settings word at +0xc and queues
/// the sound for loading (`00add960`).
// Translation incomplete: 00add6f0 - C++ exception unwinding (SEH frame state) and the /GS stack cookie check (00ec408c) are not translated.
pub fn bs_audio_manager_precache_ov3(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    name: u32,
    flags: u32,
    extra: u32,
) {
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    if name == 0 || e.mem.u8(audio + AUDIO_ENABLED) == 0 {
        return;
    }
    let id = e.get(this, BSAudioManager::iNextID);
    e.set(this, BSAudioManager::iNextID, id.wrapping_add(1));
    // The id record handed to the settings callback: the id, a byte and a word.
    e.with_stack(12 + 0x104 + 8, |e, block| {
        let record = block.addr();
        let buffer = record + 12;
        let out = buffer + 0x104;
        e.mem.set_u32(record, id);
        e.call(STRCPY_S, &[buffer, 0x104, name]);
        e.vcall(audio, AUDIO_SLOT_FIX_SOUND_PATH, &[buffer]);
        let hash = e.call(SOUND_NAME_HASH, &[buffer]).u32();
        if bs_audio_manager_is_in_cache(e, this, hash, 1) {
            return;
        }
        let sound = e
            .vcall(audio, AUDIO_SLOT_CREATE_NEW_GAME_SOUND, &[name])
            .u32();
        e.mem.set_u32(sound + SOUND_ID, id);
        e.mem.set_u32(sound + SOUND_TYPE_FLAGS, flags);
        e.call(SOUND_CALCULATE_HASHES, &[sound, 0]);
        let settings_callback = e.mem.u32(audio + AUDIO_SETTINGS_CALLBACK);
        e.mem.set_u32(out, 0);
        if settings_callback != 0 {
            e.call(settings_callback, &[record, extra, out]);
        }
        let loop_callback = e
            .vcall(audio, AUDIO_SLOT_GET_LOOP_POINT_CALLBACK, &[])
            .u32();
        if loop_callback != 0 {
            e.with_stack(8, |e, points| {
                if e.call(loop_callback, &[extra, points.addr(), points.addr() + 4])
                    .bool()
                {
                    let start = e.mem.u32(points.addr());
                    let end = e.mem.u32(points.addr() + 4);
                    e.mem.set_u32(sound + SOUND_LOOP_START, start);
                    e.mem.set_u32(sound + SOUND_LOOP_END, end);
                }
            });
        }
        let settings = e.mem.u32(out);
        e.mem.set_u32(sound + SOUND_MODIFIER_FLAGS, settings);
        fn_00add960(e, this, sound);
    });
}

// Translated from 00add960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues a precached sound for loading (unnamed in the engine map). A sound
/// is deleted instead when audio is disabled, it has no file name, its type
/// bit 0x10000 is set, or it is cached already (`IsInCache`, hash only).
/// Otherwise, under the load-task critical section (+0x10c): when a task for
/// a sound with the same hash and directory hash is queued, the sound is
/// deleted; else its type bit 0x8000 is set, an `AudioLoadTask` (0x38 bytes,
/// built by `00ae6f50`) is made for it, the sound gets state bit 0x1000, the
/// task goes to the load queue object `01202d98` (slot 0x48) and to the end
/// of `CacheLoadTaskList` (+0x124).
// Translation incomplete: 00add960 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_00add960(e: &mut Engine, this: Ptr<BSAudioManager>, sound: u32) {
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    let discard = e.mem.u8(audio + AUDIO_ENABLED) == 0
        || e.mem.u8(sound + SOUND_FILENAME) == 0
        || e.mem.u32(sound + SOUND_TYPE_FLAGS) & 0x10000 != 0
        || {
            let hash = e.mem.u32(sound + SOUND_HASH);
            bs_audio_manager_is_in_cache(e, this, hash, 0)
        };
    if discard {
        if sound != 0 {
            e.vcall(sound, SOUND_SLOT_DESTRUCTOR, &[1]);
        }
        return;
    }
    let section = this.addr() + 0x10c;
    enter_section(e, section);
    let mut queued = false;
    let mut node = e.mem.u32(this.addr() + 0x124);
    while node != 0 {
        // AudioLoadTask +0x30: the sound the task loads.
        let task = e.mem.u32(node + 8);
        let loading = e.mem.u32(task + 0x30);
        if e.mem.u32(loading + SOUND_HASH) == e.mem.u32(sound + SOUND_HASH)
            && e.mem.u32(loading + SOUND_DIRECTORY_HASH) == e.mem.u32(sound + SOUND_DIRECTORY_HASH)
        {
            queued = true;
            break;
        }
        node = e.mem.u32(node);
    }
    if queued {
        leave_section(e, section);
        e.vcall(sound, SOUND_SLOT_DESTRUCTOR, &[1]);
        return;
    }
    let type_flags = e.mem.u32(sound + SOUND_TYPE_FLAGS);
    e.mem.set_u32(sound + SOUND_TYPE_FLAGS, type_flags | 0x8000);
    let raw = e.call(ALLOCATE, &[MEMORY_MANAGER, 0x38]).u32();
    let task = if raw != 0 {
        e.call(LOAD_TASK_CONSTRUCTOR, &[raw, sound]).u32()
    } else {
        0
    };
    let state = e.mem.u32(sound + SOUND_STATE_FLAGS);
    e.mem.set_u32(sound + SOUND_STATE_FLAGS, state | 0x1000);
    let queue = e.global::<u32>(LOAD_QUEUE_GLOBAL);
    e.vcall(queue, LOAD_QUEUE_SLOT_ADD, &[task]);
    list_append_pointer(e, this.addr() + 0x124, task);
    leave_section(e, section);
}

// Translated from 00addc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Process` (Xbox PDB): runs the `BSAudio` synch timer's
/// update (`00aea100` on the object at +0x14) and, when the processing
/// critical section (+0xf4 of `this`) can be entered without waiting,
/// processes the messages and then the sounds of the singleton.
pub fn bs_audio_manager_process(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    e.call(SYNCH_TIMER_UPDATE, &[audio + 0x14]);
    let section = this.addr() + 0xf4;
    if e.call(IMPORT_TRY_ENTER_CRITICAL_SECTION, &[section]).u32() != 0 {
        let manager = bs_audio_manager_q_instance(e);
        bs_audio_manager_process_messages(e, manager);
        let manager = bs_audio_manager_q_instance(e);
        fn_00adddd0(e, manager);
        leave_section(e, section);
    }
}

// Translated from 00addc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::ProcessMessages` (Xbox PDB): takes the messages of the
/// list `iMessageProcessingListIndex` selects from the head and processes
/// each (`ProcessMessage`), pushing it on `GarbageMessages` afterwards; when
/// `ProcessMessage` answers true it returns at once without the rest.
/// Afterwards a list that has a count but lost its head or tail is cleared;
/// under the message critical section (+0xac) the two list indexes swap and
/// every garbage message is freed (`FreeMessage`).
pub fn bs_audio_manager_process_messages(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let index = e.get(this, BSAudioManager::iMessageProcessingListIndex);
    let list = this.addr() + 0x0c + index.wrapping_mul(0x0c);
    while e.mem.u32(list + 8) != 0 && e.mem.u32(list) != 0 && e.mem.u32(list + 4) != 0 {
        let message = e.call(LIST_REMOVE_HEAD, &[list]).u32();
        if fn_00adf080(e, this, Ptr::new(message)) {
            return;
        }
        e.call(MESSAGE_STACK_PUSH, &[this.addr() + 0x44, message]);
    }
    if e.mem.u32(list + 8) != 0 && (e.mem.u32(list) == 0 || e.mem.u32(list + 4) == 0) {
        e.call(LIST_CLEAR, &[list]);
    }
    let section = this.addr() + 0xac;
    enter_section(e, section);
    // `(x + 1) % 2` with the compiler's signed remainder.
    let next = (e
        .get(this, BSAudioManager::iMessageListIndex)
        .wrapping_add(1) as i32
        % 2) as u32;
    e.set(this, BSAudioManager::iMessageListIndex, next);
    let next = (next.wrapping_add(1) as i32 % 2) as u32;
    e.set(this, BSAudioManager::iMessageProcessingListIndex, next);
    loop {
        let message = e.call(MESSAGE_STACK_POP, &[this.addr() + 0x44]).u32();
        if message == 0 {
            break;
        }
        bs_audio_manager_free_message(e, this, Ptr::new(message));
    }
    leave_section(e, section);
}

// Translated from 00adddd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates every sound of the manager (unnamed in the engine map; runs after
/// the messages each `Process`). Without a listener (`BSAudio` +0x10) it does
/// nothing. Otherwise: `iDelayTimerDelta` becomes the time since the last
/// run; the playback speed is the global time multiplier (`011ac3a0`, at
/// least `0.25`) or 1.0 when `bIgnoreGlobalTimeMultiplier` is set. Each
/// sound in the sound list is then handled by its state bits: prepared
/// sounds are prepared and cached (`00adcd80`) and their queued messages
/// replayed; the range test (`00ae9b50`) toggles state bit 0x10000 and the
/// sound is updated; a stopped sound waiting to start gets state bit 0x10;
/// the cleanup delay counts down by the elapsed time; a finished sound runs
/// its completion callback, leaves the sound list and the moving-reference
/// list, drops its queued messages, loses its voice state and is deleted;
/// others get their voice state set, are stopped and made inaudible beyond
/// 1.05 times their maximum distance (and played again within it), and their
/// frequency is set to the playback speed. Afterwards `bListenerMoved` is
/// cleared, the moving sounds follow their objects (`00adea90`) and the
/// listener's slot 0x10 is called.
// Translation incomplete: 00adddd0 - C++ exception unwinding (SEH frame state) around the local message list is not translated.
pub fn fn_00adddd0(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let previous_delta = e.get(this, BSAudioManager::iDelayTimerDelta);
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    if e.mem.u32(audio + AUDIO_LISTENER) == 0 {
        return;
    }
    let ticks = e.call(IMPORT_GET_TICK_COUNT, &[]).u32();
    e.set(
        this,
        BSAudioManager::iDelayTimerDelta,
        ticks.wrapping_sub(previous_delta),
    );
    let mut speed = x87_quiet(e.global::<f32>(TIME_MULTIPLIER));
    if (speed as f64) < e.mem.f64(MINIMUM_SPEED_DOUBLE) {
        speed = x87_quiet(e.global::<f32>(MINIMUM_SPEED));
    }
    if e.get(this, BSAudioManager::bIgnoreGlobalTimeMultiplier) {
        speed = 1.0;
    }
    // The moving-reference object slot `00ae7e60` fills (a smart pointer), in the exe's frame
    // (`EBP` frame, exception record, `SUB ESP,0x174`, the cookie: 0x184 bytes) at +0x160
    // (`EBP-0x24`, 00ade56b).
    e.with_stack(0x184, |e, frame| {
        let object_slot = frame.addr() + 0x160;
        let sounds = this.at(BSAudioManager::SoundList);
        let mut position = map_first_item(e, sounds);
        while position != 0 {
            let (_key, sound) = map_next(e, sounds, &mut position);
            update_sound(e, this, sound, speed, object_slot);
        }
        e.set(this, BSAudioManager::bListenerMoved, false);
        fn_00adea90(e, this);
        let listener = e.mem.u32(audio + AUDIO_LISTENER);
        e.vcall(listener, LISTENER_SLOT_AFTER_UPDATE, &[]);
        let ticks = e.call(IMPORT_GET_TICK_COUNT, &[]).u32();
        e.set(this, BSAudioManager::iDelayTimerDelta, ticks);
        let held = e.mem.u32(object_slot);
        drop_reference(e, held);
    });
}

/// One sound of `00adddd0`.
fn update_sound(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    sound: u32,
    speed: f32,
    object_slot: u32,
) {
    let state = |e: &Engine| e.mem.u32(sound + SOUND_STATE_FLAGS);
    let kind = |e: &Engine| e.mem.u32(sound + SOUND_TYPE_FLAGS);
    let id = e.mem.u32(sound + SOUND_ID);
    if state(e) & 1 != 0 && state(e) & 0x800 == 0 && state(e) & 0x100 != 0 {
        e.vcall(sound, SOUND_SLOT_PREPARE, &[]);
        fn_00adcd80(e, this, sound);
        if state(e) & 0x800 != 0
            && state(e) & 0x100 != 0
            && e.mem.u32(sound + SOUND_QUEUED_MESSAGES + 8) != 0
        {
            replay_queued_messages(e, this, sound);
        }
    }
    if state(e) & 1 != 0 && state(e) & 0x800 != 0 && state(e) & 0x100 != 0 {
        let hour = x87_quiet(e.get(this, BSAudioManager::fCurrentGamehour));
        let in_range = |e: &mut Engine| e.call(SOUND_RANGE_TEST, &args![sound, 0u32, hour]).bool();
        if state(e) & 0x10000 == 0 {
            if !in_range(e) {
                let flags = state(e);
                e.mem.set_u32(sound + SOUND_STATE_FLAGS, flags | 0x10000);
            }
        } else if in_range(e) {
            let flags = state(e);
            e.mem.set_u32(sound + SOUND_STATE_FLAGS, flags & !0x10000);
        }
        e.vcall(sound, SOUND_SLOT_UPDATE, &[0]);
    }
    if state(e) & 0x2000 != 0
        && !e.vcall(sound, SOUND_SLOT_IS_PLAYING, &[]).bool()
        && kind(e) & 0x100 != 0
        && !e.vcall(sound, SOUND_SLOT_IS_PAUSED, &[]).bool()
        && state(e) & 0x800 != 0
    {
        let flags = state(e);
        e.mem.set_u32(sound + SOUND_STATE_FLAGS, flags | 0x10);
    }
    if state(e) & 0x10 != 0
        && state(e) & 0x200 == 0
        && state(e) & 0x1000 == 0
        && e.mem.u32(sound + SOUND_CLEANUP_DELAY) != 0
    {
        let delay = e.mem.u32(sound + SOUND_CLEANUP_DELAY);
        let elapsed = e.get(this, BSAudioManager::iDelayTimerDelta);
        e.mem
            .set_u32(sound + SOUND_CLEANUP_DELAY, delay.saturating_sub(elapsed));
    }
    if state(e) & 0x10 != 0
        && state(e) & 0x200 == 0
        && state(e) & 0x1000 == 0
        && e.mem.u32(sound + SOUND_CLEANUP_DELAY) == 0
    {
        finish_sound(e, this, sound, id, object_slot);
        return;
    }
    if state(e) & 0x800 != 0 {
        bs_audio_manager_set_voice_state(e, this, id, Ptr::new(sound));
    }
    let distance = e.mem.f32(sound + SOUND_DISTANCE_TO_LISTENER);
    let maximum = e.mem.f32(sound + SOUND_MAX_DISTANCE);
    let mut handled = false;
    if (e.call(SOUND_IS_EMITTER_SOUND, &[sound]).bool()
        || e.call(SOUND_HAS_EMITTER, &[sound]).bool())
        && kind(e) & 0x100 == 0
        && state(e) & 0x2_0000 == 0
        && e.vcall(sound, SOUND_SLOT_IS_PLAYING, &[]).bool()
    {
        let attenuated =
            kind(e) & 0x10 != 0 || kind(e) & 0x400_0000 != 0 || kind(e) & 0x200_0000 != 0;
        if attenuated
            && kind(e) & 0x200_0000 == 0
            && distance as f64 > maximum as f64 * e.mem.f64(RANGE_MARGIN)
        {
            e.vcall(sound, SOUND_SLOT_STOP, &[]);
            e.vcall(sound, SOUND_SLOT_SET_INAUDIBLE, &[1]);
            handled = true;
        }
    }
    if !handled
        && state(e) & 0x2_0000 != 0
        && !e.vcall(sound, SOUND_SLOT_IS_PAUSED, &[]).bool()
        && distance < maximum
    {
        e.vcall(sound, SOUND_SLOT_SET_INAUDIBLE, &[0]);
        e.vcall(sound, SOUND_SLOT_PLAY, &[1]);
    }
    let frequency = e.vcall(sound, SOUND_SLOT_GET_FREQUENCY, &[]).f32();
    let difference = (speed as f64 - frequency as f64) as f32;
    let flag_0080 = kind(e) & 0x80 != 0;
    let below = (difference as f64) < e.mem.f64(ZERO_DOUBLE);
    let magnitude = e.call(FABS, &args![difference as f64]).f64() as f32;
    if magnitude as f64 > e.mem.f64(FREQUENCY_TOLERANCE)
        && (!flag_0080 || below)
        && state(e) & 0x800 != 0
        && kind(e) & 0x40_0000 == 0
        && kind(e) & 0x10_0000 == 0
        && kind(e) & 0x800 == 0
        && kind(e) & 0x20 == 0
        && kind(e) & 0x20_0000 == 0
    {
        e.vcall(sound, SOUND_SLOT_SET_FREQUENCY, &args![speed]);
    }
}

/// The part of `00adddd0` that replays a sound's queued messages: they move
/// to a local list; the ones of type 0x1e (set position) are processed
/// first, then every message is taken from the head, processed and freed
/// (`FreeQueueMessage`). The local list is cleared at the end.
fn replay_queued_messages(e: &mut Engine, this: Ptr<BSAudioManager>, sound: u32) {
    let queue = sound + SOUND_QUEUED_MESSAGES;
    e.with_stack(12, |e, local| {
        let list = local.addr();
        e.call(MESSAGE_LIST_CONSTRUCTOR, &[list]);
        while e.mem.u32(queue + 8) != 0 {
            let message = e.call(LIST_REMOVE_HEAD, &[queue]).u32();
            let node = e.call(LIST_NEW_NODE, &[list + 8]).u32();
            e.mem.set_u32(node + 8, message);
            e.call(LIST_ADD_NODE_TAIL, &[list, node]);
        }
        let mut node = e.mem.u32(list);
        while node != 0 {
            let message = e.mem.u32(node + 8);
            node = e.mem.u32(node);
            if message != 0 && e.mem.u32(message) == 0x1e {
                fn_00adf080(e, this, Ptr::new(message));
            }
        }
        while e.mem.u32(list + 8) != 0 {
            let message = e.call(LIST_REMOVE_HEAD, &[list]).u32();
            fn_00adf080(e, this, Ptr::new(message));
            bs_audio_manager_free_queue_message(e, this, Ptr::new(message));
        }
        e.call(LIST_CLEAR, &[list]);
    });
}

/// The end of a finished sound in `00adddd0`: runs the completion callback
/// (`context`, whether state bit 0x100 is set), removes the sound from the
/// sound list and the moving-reference list (releasing the object it
/// followed), frees its queued messages, clears its voice state and
/// deletes it.
fn finish_sound(e: &mut Engine, this: Ptr<BSAudioManager>, sound: u32, id: u32, object_slot: u32) {
    e.vcall(sound, SOUND_SLOT_UPDATE, &[0]);
    let callback = e.mem.u32(sound + SOUND_COMPLETION_CALLBACK);
    if callback != 0 {
        let prepared = (e.mem.u32(sound + SOUND_STATE_FLAGS) & 0x100 != 0) as u32;
        let context = e.mem.u32(sound + SOUND_COMPLETION_CONTEXT);
        e.call(callback, &[context, prepared]);
    }
    e.call(MAP_REMOVE, &[this.addr() + 0x54, id]);
    fn_00ae7e60(e, this.at(BSAudioManager::MovingRefrList), id, object_slot);
    e.call(MAP_REMOVE, &[this.addr() + 0x84, id]);
    let object = e.mem.u32(object_slot);
    if object != 0 {
        e.call(RELEASE_MESSAGE_POINTER, &[object]);
        e.call(SMART_POINTER_SET, &[object_slot, 0]);
    }
    if e.mem.u32(sound + SOUND_QUEUED_MESSAGES + 8) != 0 {
        let queue = sound + SOUND_QUEUED_MESSAGES;
        while e.mem.u32(queue + 8) != 0 {
            let message = e.call(LIST_REMOVE_HEAD, &[queue]).u32();
            bs_audio_manager_free_queue_message(e, this, Ptr::new(message));
        }
    }
    bs_audio_manager_set_voice_state(e, this, id, Ptr::NULL);
    e.vcall(sound, SOUND_SLOT_DESTRUCTOR, &[1]);
}

// Translated from 00adea90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the sounds that follow an object (unnamed in the engine map; run by
/// `00adddd0`). Without a listener it does nothing. Otherwise it calls the
/// listener's slot 0xc (the position it returns is copied to locals the code
/// never reads) and walks the moving-reference map (+0x84): for each entry
/// it looks the sound up in the sound list and takes the followed object
/// (`00ae7ee0`). A sound that is playing (or has state bit 0x20000) whose
/// object has more than two references is moved to the object's position
/// (+0x8c; 200.0 higher when the sound's type bit 4 is set,
/// `SetEmitterPosition`, slot 0x4c). An entry without a sound is removed and
/// the object released. An entry whose object is gone while the sound still
/// exists queues message 9 (fade out) of 1000 for that sound and is removed.
// Translation incomplete: 00adea90 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_00adea90(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    let listener = e.mem.u32(audio + AUDIO_LISTENER);
    if listener == 0 {
        return;
    }
    e.vcall(listener, LISTENER_SLOT_GET_POSITION, &[]);
    let moving = this.at(BSAudioManager::MovingRefrList);
    let sounds = this.at(BSAudioManager::SoundList);
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0xf4`, the cookie): 0x104 bytes.
    // The followed-object smart pointer the loop keeps in one slot at +0xc0 (`EBP-0x44`), the
    // map position at +0xf0 (`EBP-0x14`) and the key at +0xdc (`EBP-0x28`) (00adebf6).
    e.with_stack(0x104, |e, frame| {
        let frame = frame.addr();
        let object_slot = frame + 0xc0;
        let (position_slot, key_slot) = (frame + 0xf0, frame + 0xdc);
        let mut position = map_first_item(e, moving);
        while position != 0 {
            e.mem.set_u32(position_slot, position);
            fn_00ae7ee0(e, moving, position_slot, key_slot, object_slot);
            position = e.mem.u32(position_slot);
            let key = e.mem.u32(key_slot);
            let (_, sound) = map_get(e, sounds, key, 0);
            let object = e.mem.u32(object_slot);
            let references = if object != 0 {
                e.mem.u32(object + 4)
            } else {
                0
            };
            if sound != 0
                && (e.vcall(sound, SOUND_SLOT_IS_PLAYING, &[]).bool()
                    || e.mem.u32(sound + SOUND_STATE_FLAGS) & 0x2_0000 != 0)
                && object != 0
                && references > 2
            {
                // NiAVObject world translation (+0x8c).
                let x = e.mem.f32(object + 0x8c);
                let y = e.mem.f32(object + 0x90);
                let mut z = e.mem.f32(object + 0x94);
                if e.mem.u32(sound + SOUND_TYPE_FLAGS) & 4 != 0 {
                    z = (z as f64 + e.mem.f64(EMITTER_HEIGHT_OFFSET)) as f32;
                }
                e.vcall(sound, SOUND_SLOT_SET_EMITTER_POSITION, &args![x, y, z]);
            } else if sound == 0 {
                e.call(MAP_REMOVE, &[moving.addr(), key]);
                release_followed_object(e, object_slot, object);
            } else if object == 0 {
                let id = e.mem.u32(sound + SOUND_ID);
                let fade_x = x87_quiet(e.global::<f32>(FADE_MESSAGE_X));
                fn_00adb1a0(e, this, 9, id, 0, 0x3e8, 0, fade_x, 0.0, 0.0);
                e.call(MAP_REMOVE, &[moving.addr(), key]);
                release_followed_object(e, object_slot, object);
            }
        }
        let held = e.mem.u32(object_slot);
        drop_reference(e, held);
    });
}

/// Releases a followed object the way `00adea90` does: with a temporary
/// reference held, the object is handed to `00c45830`, the slot is set to
/// null and the temporary is dropped. Nothing happens for a null object.
fn release_followed_object(e: &mut Engine, slot: u32, object: u32) {
    if object != 0 {
        add_reference(e, object);
        e.call(RELEASE_MESSAGE_POINTER, &[object]);
        e.call(SMART_POINTER_SET, &[slot, 0]);
        drop_reference(e, object);
    }
}

// Translated from 00adef80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::PositionListener` (Xbox PDB): queues message 0x38
/// (set listener position) with the three coordinates as the message
/// vector.
pub fn bs_audio_manager_position_listener(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    x: f32,
    y: f32,
    z: f32,
) {
    fn_00adb1a0(e, this, 0x38, 0, 0, 0, 0, x, y, z);
}

// Translated from 00adeff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetListenerUnderwater` (Xbox PDB): when `underwater`
/// differs from `bListenerUnderwater`, stores it and queues message 0x37
/// with the default vector.
pub fn bs_audio_manager_set_listener_underwater(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    underwater: u8,
) {
    if underwater != e.mem.u8(this.addr() + 0x175) {
        e.mem.set_u8(this.addr() + 0x175, underwater);
        queue_plain(e, this, 0x37, 0, 0, 0);
    }
}

/// Stores the handle of no sound in `result`.
fn set_invalid_handle(e: &mut Engine, result: Ptr<BSSoundHandle>) {
    e.set(result, BSSoundHandle::iSoundID, 0xffff_ffff);
    e.set(result, BSSoundHandle::bAssumeSuccess, false);
    e.set(result, BSSoundHandle::eState, 0);
}

/// The flag changes `GetSoundHandleByName` and `GetSoundHandleByNumericID`
/// make to the caller's flags from the bits the `BSAudio` callback wrote:
/// bit 0x2000000 or bit 0x4000000 of `written` selects that bit in the
/// flags and clears bit 0x10.
fn apply_written_sound_flags(written: u32, mut flags: u32) -> u32 {
    if written & 0x200_0000 != 0 {
        flags |= 0x200_0000;
        flags &= !0x10;
    } else if written & 0x400_0000 != 0 {
        flags |= 0x400_0000;
        flags &= !0x10;
    }
    flags
}

/// The shared opening of `GetSoundHandleByName` and
/// `GetSoundHandleByNumericID`: asks the `BSAudio` callback at
/// `callback_offset` about `argument` (it fills a 264-byte buffer, a word of
/// flags and a word of extra data). Returns the buffer's address, the
/// flags it wrote and the extra word when the audio is enabled and the
/// callback exists and answered true; `None` otherwise.
fn sound_handle_callback(
    e: &mut Engine,
    callback_offset: u32,
    argument: u32,
    body: impl FnOnce(&mut Engine, u32, u32, u32),
) -> bool {
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    let callback = e.mem.u32(audio + callback_offset);
    if callback == 0 {
        return false;
    }
    e.with_stack(0x108 + 8, |e, block| {
        let buffer = block.addr();
        let written_slot = buffer + 0x108;
        let extra_slot = buffer + 0x10c;
        if !e
            .call(callback, &[argument, buffer, written_slot, extra_slot])
            .bool()
        {
            return false;
        }
        let written = e.mem.u32(written_slot);
        let extra = e.mem.u32(extra_slot);
        body(e, buffer, written, extra);
        true
    })
}

// Translated from 00ae5680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::GetSoundHandleByName` (Xbox PDB): fills `result` with a
/// handle for the sound `name` stands for. The handle is empty (id -1) when
/// the audio is disabled, `BSAudio` has no name callback (`pfnNameCallback`,
/// +0x24), the callback answers false or leaves the file name empty.
/// Otherwise `flags` gets bits 0x1 and 0x8000000 when the callback set
/// 0x8000000 and bit 0x2000000 or 0x4000000 (instead of 0x10) as the
/// callback set them, and the handle comes from `GetSoundHandleByFilename`
/// with the file name and the callback's extra word. Returns `result`.
// Translation incomplete: 00ae5680 - the /GS stack cookie check (00ec408c) is not translated.
pub fn bs_audio_manager_get_sound_handle_by_name(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    result: Ptr<BSSoundHandle>,
    name: u32,
    mut flags: u32,
) -> Ptr<BSSoundHandle> {
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    if e.mem.u8(audio + AUDIO_ENABLED) == 0 {
        set_invalid_handle(e, result);
        return result;
    }
    let found = sound_handle_callback(e, AUDIO_NAME_CALLBACK, name, |e, buffer, written, extra| {
        if written & 0x800_0000 != 0 {
            flags |= 1;
            flags |= 0x800_0000;
        }
        flags = apply_written_sound_flags(written, flags);
        if e.mem.u8(buffer) == 0 {
            set_invalid_handle(e, result);
        } else {
            bs_audio_manager_get_sound_handle_by_filename(e, this, result, buffer, flags, extra);
        }
    });
    if !found {
        set_invalid_handle(e, result);
    }
    result
}

// Translated from 00ae5870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::GetSoundHandleByNumericID` (Xbox PDB): like
/// `GetSoundHandleByName` through the ID callback (`pfnIDCallback`, +0x20)
/// with `id`, without the empty-name test. The flags change a little
/// differently: callback bit 0x1 with caller bit 0x2 turns 0x2 into 0x1, and
/// callback bit 0x8000000 sets 0x1 and 0x8000000 and clears 0x2.
// Translation incomplete: 00ae5870 - the /GS stack cookie check (00ec408c) is not translated.
pub fn bs_audio_manager_get_sound_handle_by_numeric_id(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    result: Ptr<BSSoundHandle>,
    id: u32,
    mut flags: u32,
) -> Ptr<BSSoundHandle> {
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    if e.mem.u8(audio + AUDIO_ENABLED) == 0 {
        set_invalid_handle(e, result);
        return result;
    }
    let found = sound_handle_callback(e, AUDIO_ID_CALLBACK, id, |e, buffer, written, extra| {
        if written & 1 != 0 && flags & 2 != 0 {
            flags = (flags & !2) | 1;
        }
        if written & 0x800_0000 != 0 {
            flags |= 0x800_0000;
            flags |= 1;
            flags &= !2;
        }
        flags = apply_written_sound_flags(written, flags);
        bs_audio_manager_get_sound_handle_by_filename(e, this, result, buffer, flags, extra);
    });
    if !found {
        set_invalid_handle(e, result);
    }
    result
}

// Translated from 00ae5a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::GetSoundHandleByFilename` (Xbox PDB): creates a sound
/// for `filename` and fills `result` with its handle (id = the next
/// `iNextID`). With the audio disabled or an empty file name (it logs
/// `00ae5b02`'s message, `0x10a34f8`) the handle is empty. A name with
/// neither bit 0x1 nor 0x2 in `flags` gets bit 0x1 (and is logged). The sound
/// (slot 0x14 of `BSAudio`) gets the flags, the id, the cleanup delay of
/// `fAudioDebugDelay` seconds in milliseconds, and the listener's
/// environment type (listener slot 0x2c, sound slot 0x18). A `BSSoundInfo`
/// record (0x254 bytes) for the id goes into the state map (+0x64) under the
/// state critical section. When `extra` is not 0 the settings callback
/// (`BSAudio` +0x2c) is asked for flags (bit 0x8000000 sets 0x1 and
/// 0x8000000 and clears 0x2; bits 0x10, 0x2000000 and 0x4000000 are taken
/// over from it) and a word that becomes the sound's modifier flags (+0xc);
/// then message 0x25 (add sound to list) is queued with the sound, the
/// callback is run again with no output, and the loop-point callback
/// (slot 0x10) fills the sound's loop points (+0x174, +0x178).
// Translation incomplete: 00ae5a50 - C++ exception unwinding (SEH frame state) and the sound-handle temporaries' cleanup are not translated.
pub fn bs_audio_manager_get_sound_handle_by_filename(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    result: Ptr<BSSoundHandle>,
    filename: u32,
    mut flags: u32,
    extra: u32,
) -> Ptr<BSSoundHandle> {
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    if e.mem.u8(audio + AUDIO_ENABLED) == 0 {
        set_invalid_handle(e, result);
        return result;
    }
    let id = e.get(this, BSAudioManager::iNextID);
    e.set(this, BSAudioManager::iNextID, id.wrapping_add(1));
    if e.mem.u8(filename) == 0 {
        e.call(DEBUG_PRINT, &[NO_FILENAME_MESSAGE]);
        set_invalid_handle(e, result);
        return result;
    }
    if flags & 1 == 0 && flags & 2 == 0 {
        e.call(DEBUG_PRINT, &[NO_DIMENSION_MESSAGE, filename]);
        flags |= 1;
    }
    let sound = e
        .vcall(audio, AUDIO_SLOT_CREATE_NEW_GAME_SOUND, &[filename])
        .u32();
    e.mem.set_u32(sound + SOUND_TYPE_FLAGS, flags);
    e.mem.set_u32(sound + SOUND_ID, id);
    let delay = e.global::<f32>(AUDIO_DEBUG_DELAY_SETTING);
    let delay_ms = scaled_milliseconds_inline(e, delay);
    e.mem.set_u32(sound + SOUND_CLEANUP_DELAY, delay_ms);
    let listener = e.mem.u32(audio + AUDIO_LISTENER);
    if listener != 0 {
        let environment = e.vcall(listener, LISTENER_SLOT_GET_ENVIRONMENT, &[]).u32();
        e.vcall(sound, SOUND_SLOT_SET_ENVIRONMENT_TYPE, &[environment]);
    }
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0xb4`, the cookie): 0xc4 bytes.
    // The handle the caller gets (the id, no assumed success, state 0) at +0x94 (`EBP-0x30`),
    // the settings word at +0xac (`EBP-0x18`), the loop points at +0xa0 and +0xb0
    // (`EBP-0x24`, `EBP-0x14`).
    e.with_stack(0xc4, |e, frame| {
        let frame = frame.addr();
        let handle = frame + 0x94;
        e.mem.set_u32(handle, id);
        e.mem.set_u8(handle + 4, 0);
        e.mem.set_u32(handle + 8, 0);
        let section = this.addr() + 0xc4;
        enter_section(e, section);
        let raw = e.call(ALLOCATE, &[MEMORY_MANAGER, 0x254]).u32();
        let record = if raw != 0 {
            e.call(DEBUG_SOUND_INFO_CONSTRUCTOR, &[raw + 0x18]);
            e.mem.set_u32(raw, id);
            e.mem.set_u32(raw + 0xc, 0);
            e.mem.set_u32(raw + 0x10, 0);
            e.mem.set_u8(raw + 0x14, 0);
            raw
        } else {
            0
        };
        e.call(MAP_SET_VALUE, &[this.addr() + 0x64, id, record]);
        leave_section(e, section);
        let settings_callback = e.mem.u32(audio + AUDIO_SETTINGS_CALLBACK);
        let mut settings_flags = 0;
        let word = frame + 0xac;
        e.mem.set_u32(word, 0);
        if extra != 0 && settings_callback != 0 {
            settings_flags = e.call(settings_callback, &[handle, extra, word]).u32();
        }
        let settings_word = e.mem.u32(word);
        e.mem.set_u32(sound + SOUND_MODIFIER_FLAGS, settings_word);
        if settings_flags != 0 {
            if settings_flags & 0x800_0000 != 0 {
                flags = (flags & !2) | 1 | 0x800_0000;
            }
            if settings_flags & 0x10 != 0
                || settings_flags & 0x200_0000 != 0
                || settings_flags & 0x400_0000 != 0
            {
                flags = (settings_flags & 0x600_0010) | (flags & 0xf9ff_ffef);
            }
            e.mem.set_u32(sound + SOUND_TYPE_FLAGS, flags);
        }
        queue_plain(e, this, 0x25, id, 0, sound);
        if extra != 0 && settings_callback != 0 {
            e.call(settings_callback, &[handle, extra, 0]);
        }
        let loop_callback = e
            .vcall(audio, AUDIO_SLOT_GET_LOOP_POINT_CALLBACK, &[])
            .u32();
        let (start_slot, end_slot) = (frame + 0xa0, frame + 0xb0);
        e.mem.set_u32(start_slot, 0);
        e.mem.set_u32(end_slot, 0);
        if loop_callback != 0 && e.call(loop_callback, &[extra, start_slot, end_slot]).bool() {
            let start = e.mem.u32(start_slot);
            let end = e.mem.u32(end_slot);
            e.mem.set_u32(sound + SOUND_LOOP_START, start);
            e.mem.set_u32(sound + SOUND_LOOP_END, end);
        }
        // The caller gets the handle record as the callbacks left it.
        let handle_id = e.mem.u32(handle);
        let assumed = e.mem.u8(handle + 4) != 0;
        let state = e.mem.u32(handle + 8);
        e.set(result, BSSoundHandle::iSoundID, handle_id);
        e.set(result, BSSoundHandle::bAssumeSuccess, assumed);
        e.set(result, BSSoundHandle::eState, state);
    });
    result
}

// Translated from 00ae5f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills `result` with the handle of no sound and returns it (unnamed in
/// the engine map). It takes six more stack words that it never reads.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ae5f70(
    e: &mut Engine,
    _this: Ptr<BSAudioManager>,
    result: Ptr<BSSoundHandle>,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> Ptr<BSSoundHandle> {
    set_invalid_handle(e, result);
    result
}

// Translated from 00ae5fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares the unsigned words `a` and `b` point to (unnamed in the engine
/// map; the comparison `GetDebugString` sorts its lines with): -1 when the
/// first is smaller, 1 when the second is, else 0.
pub fn fn_00ae5fb0(e: &mut Engine, a: u32, b: u32) -> i32 {
    let first = e.mem.u32(a);
    let second = e.mem.u32(b);
    if first < second {
        -1
    } else {
        (second < first) as i32
    }
}

/// Appends a line to the debug list `list` (a head node of
/// `BSSimpleList<AudioDebugString *>`): an 8-byte record of the sort index
/// and the text, added through `00af25b0`.
fn add_debug_line(e: &mut Engine, list: u32, index: u32, text: u32) {
    let raw = e.call(ALLOCATE, &[MEMORY_MANAGER, 8]).u32();
    if raw != 0 {
        e.mem.set_u32(raw, index);
        e.mem.set_u32(raw + 4, text);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), raw);
        e.call(SIMPLE_LIST_ADD, &[list, slot.addr()]);
    });
}

/// The lines `GetDebugString` adds for the pages 1 to 10: the text appended
/// to `Page %i`, and the filter bits (or, for page 10, the stopped-sounds
/// mode).
const DEBUG_PAGES: [(u32, u32); 10] = [
    (0x010a_36b4, 2),
    (0x010a_36a8, 1),
    (0x010a_3694, 0x800_0000),
    (0x010a_3684, 4),
    (0x010a_3670, 8),
    (0x010a_3660, 0x20),
    (0x010a_3650, 0x1000),
    (0x010a_3638, 0x4000_0000),
    (0x010a_3624, 0x1000_0000),
    (0x010a_3608, 0),
];

/// Copies the debug record of the sound state `record` into `target` (a
/// `BSDebugSoundInfo`), through a temporary built by `00ae8b40`.
fn copy_debug_record(e: &mut Engine, target: u32, record: u32) {
    e.with_stack(0x23c, |e, temporary| {
        let source = e.call(DEBUG_INFO_GETTER, &[record, temporary.addr()]).u32();
        e.call(DEBUG_SOUND_INFO_ASSIGN, &[target, source]);
        e.call(DEBUG_SOUND_INFO_DESTRUCTOR, &[temporary.addr()]);
    });
}

// Translated from 00ae5ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::GetDebugString` (Xbox PDB): builds the on-screen sound
/// list for `page` and returns it (the head node of a
/// `BSSimpleList<AudioDebugString *>`, each record a sort index and a
/// text). The voice states are synchronized first. Pages 0 to 98 start with
/// the lines `Page %i` plus the page's filter text (pages 1 to 10 select a
/// sound-type bit, page 10 the stopped or silent sounds) and `ID\tIsPlaying?`;
/// page 99 filters on type bit 0x800. Then, under the state critical
/// section, at most 19 entries of the state map are listed: those that are
/// playing and audible (largest attenuation at most 3000), or, on page 10,
/// the others, that pass the type filter. A sound with a file name gets one
/// line with its name (from the last backslash), playing state, distances
/// and attenuations (the format depends on its type bits 0x1, 0x2 and
/// 0x8000000); one without gets `id\tstate`. The list is sorted by index
/// (`00ae81e0` with `00ae5fb0`).
// Translation incomplete: 00ae5ff0 - C++ exception unwinding (SEH frame state) and the /GS stack cookie check (00ec408c) are not translated.
pub fn bs_audio_manager_get_debug_string(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    page: u32,
) -> u32 {
    let raw = e.call(ALLOCATE, &[MEMORY_MANAGER, 8]).u32();
    let list = if raw != 0 {
        e.mem.set_u32(raw, 0);
        e.mem.set_u32(raw + 4, 0);
        raw
    } else {
        0
    };
    let mut filter = 0xffff_ffffu32;
    bs_audio_manager_synch_voice_states(e, this);
    let mut only_stopped = false;
    if page < 99 {
        let text = e.call(ALLOCATE, &[MEMORY_MANAGER, 0x104]).u32();
        e.call(SPRINTF, &[text, DEBUG_PAGE_FORMAT, page.wrapping_add(1)]);
        if (1..=10).contains(&page) {
            let (suffix, bits) = DEBUG_PAGES[page as usize - 1];
            e.call(STRCAT, &[text, suffix]);
            if page == 10 {
                only_stopped = true;
            } else {
                filter = bits;
            }
        }
        add_debug_line(e, list, 0, text);
        let text = e.call(ALLOCATE, &[MEMORY_MANAGER, 0x104]).u32();
        e.call(SPRINTF, &[text, DEBUG_HEADER_FORMAT]);
        add_debug_line(e, list, 1, text);
    } else if page == 99 {
        filter = 0x800;
    }
    let section = this.addr() + 0xc4;
    enter_section(e, section);
    let states = this.at(BSAudioManager::SoundStateMap);
    let mut position = map_first_item(e, states);
    // The exe's frame: the exception record (three pushes) and `SUB ESP,0xa84` under `EBP`, 0xa90
    // bytes; the sound info record at `EBP - 0x494` (00ae6443).
    e.with_stack(0xa90, |e, frame| {
        let info = frame.addr() + 0xa90 - 0x494;
        e.call(DEBUG_SOUND_INFO_CONSTRUCTOR, &[info]);
        let mut counter = 1i32;
        while position != 0 && counter < 0x14 {
            let (_key, record) = map_next(e, states, &mut position);
            let text = e.call(ALLOCATE, &[MEMORY_MANAGER, 0x104]).u32();
            let mut loudest = 0u32;
            if record != 0 {
                copy_debug_record(e, info, record);
                let static_attenuation = e.mem.u32(info + 0x230);
                let system_attenuation = e.mem.u32(info + 0x234);
                let distance_attenuation = e.mem.u32(info + 0x238);
                loudest = loudest.max(static_attenuation);
                loudest = loudest.max(system_attenuation);
                loudest = loudest.max(distance_attenuation);
            }
            let playing = record != 0 && e.mem.u8(record + 0x14) != 0;
            let audible = (loudest as i32) <= 0xbb8;
            let listed = if only_stopped && record != 0 {
                !playing || !audible
            } else {
                !only_stopped && record != 0 && playing && audible
            };
            if !listed || e.mem.u32(record + 0xc) & filter == 0 {
                continue;
            }
            copy_debug_record(e, info, record);
            let id = e.mem.u32(record);
            let state_text = if playing { PLAY_TEXT } else { STOP_TEXT };
            let name = e.mem.u32(info + 0x108);
            if name != 0 {
                // The name pointer is made to point at the record's own buffer.
                let name = info + 0x10c;
                e.mem.set_u32(info + 0x108, name);
                let mut last = 0;
                e.with_stack(0x104 + 0x104, |e, buffers| {
                    let path = buffers.addr();
                    let formatted = path + 0x104;
                    e.call(STRCPY_S, &[path, 0x104, name]);
                    let mut slash = e.call(STRSTR, &[path, BACKSLASH_TEXT]).u32();
                    while slash != 0 && e.call(STRSTR, &[slash + 1, BACKSLASH_TEXT]).u32() != 0 {
                        slash = e.call(STRSTR, &[slash + 1, BACKSLASH_TEXT]).u32();
                    }
                    if slash != 0 && e.mem.u8(slash + 1) != 0 {
                        e.call(STRCPY_S, &[name, 0x104, slash]);
                        let kind = e.mem.u32(record + 0xc);
                        let distance = e.mem.f32(info + 0x220) as f64;
                        let minimum = e.mem.f32(info + 0x210) as f64;
                        let maximum = e.mem.f32(info + 0x214) as f64;
                        let static_attenuation = e.mem.u32(info + 0x230);
                        let system_attenuation = e.mem.u32(info + 0x234);
                        let distance_attenuation = e.mem.u32(info + 0x238);
                        let words = if kind & 2 != 0 {
                            args![
                                formatted,
                                DEBUG_3D_FORMAT,
                                id,
                                name,
                                state_text,
                                distance,
                                minimum,
                                maximum,
                                static_attenuation,
                                system_attenuation,
                                distance_attenuation
                            ]
                        } else if kind & 1 != 0 && kind & 0x800_0000 == 0 {
                            args![
                                formatted,
                                DEBUG_2D_FORMAT,
                                id,
                                name,
                                state_text,
                                static_attenuation,
                                system_attenuation
                            ]
                        } else if kind & 1 != 0 {
                            args![
                                formatted,
                                DEBUG_2D_RADIUS_FORMAT,
                                id,
                                name,
                                state_text,
                                distance,
                                minimum,
                                maximum,
                                static_attenuation,
                                system_attenuation,
                                distance_attenuation
                            ]
                        } else {
                            args![
                                formatted,
                                DEBUG_ERROR_FORMAT,
                                id,
                                name,
                                state_text,
                                static_attenuation,
                                system_attenuation
                            ]
                        };
                        e.call(SPRINTF, &words);
                        e.call(STRCPY_S, &[text, 0x104, formatted]);
                        last = 1;
                    }
                });
                if last == 0 {
                    e.call(DEALLOCATE, &[MEMORY_MANAGER, text]);
                    continue;
                }
            } else {
                e.with_stack(0x104, |e, formatted| {
                    let shown = if playing { PLAYING_TEXT } else { STOPPED_TEXT };
                    e.call(SPRINTF, &[formatted.addr(), DEBUG_PLAIN_FORMAT, id, shown]);
                    e.call(STRCAT, &[text, formatted.addr()]);
                });
            }
            add_debug_line(e, list, id.wrapping_add(2), text);
            counter += 1;
        }
        leave_section(e, section);
        bs_simple_list_audio_debug_string_sort(e, Ptr::new(list), DEBUG_LINE_COMPARE);
        e.call(DEBUG_SOUND_INFO_DESTRUCTOR, &[info]);
    });
    list
}

// Translated from 00ae6bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetAudioCacheSizeInK` (Xbox PDB): a size of 0 first
/// switches the cache off (`00ae6ca0`); a size under 4096 becomes the
/// `iAudioCacheSize` setting, a larger one is logged
/// (`AUDIO: Attempted to set audio cache size to %i...`) and ignored.
pub fn bs_audio_manager_set_audio_cache_size_in_k(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    size_in_k: u32,
) {
    if size_in_k == 0 {
        fn_00ae6ca0(e, this, 0);
    }
    if size_in_k < 0x1000 {
        e.mem.set_u32(AUDIO_CACHE_SIZE_SETTING, size_in_k);
    } else {
        e.call(DEBUG_PRINT, &[CACHE_SIZE_MESSAGE, size_in_k]);
    }
}

// Translated from 00ae6c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `iAudioCacheSize` setting (unnamed in the engine map).
pub fn fn_00ae6c60(e: &mut Engine, _this: Ptr<BSAudioManager>) -> u32 {
    e.mem.u32(AUDIO_CACHE_SIZE_SETTING)
}

// Translated from 00ae6ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `enabled` as the `bEnableAudioCache` setting (unnamed in the
/// engine map); switching it off flushes the cache (`00adcd80` with no
/// sound).
pub fn fn_00ae6ca0(e: &mut Engine, this: Ptr<BSAudioManager>, enabled: u8) {
    e.mem.set_u8(CACHE_ENABLED_SETTING, enabled);
    if enabled == 0 {
        fn_00adcd80(e, this, 0);
    }
}

// Translated from 00ae6cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `bEnableAudioCache` setting (unnamed in the engine map).
pub fn fn_00ae6cf0(e: &mut Engine, _this: Ptr<BSAudioManager>) -> u8 {
    e.mem.u8(CACHE_ENABLED_SETTING)
}

/// The address of entry `index` of the pause table (`pPauseEntries`, 8
/// bytes each: a type mask and a mode word; `iPauseListSize` entries).
fn pause_entry(e: &Engine, this: Ptr<BSAudioManager>, index: u32) -> u32 {
    e.get(this, BSAudioManager::pPauseEntries)
        .wrapping_add(index.wrapping_mul(8))
}

// Translated from 00ae6d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the sound type mask `mask` is held by the pause table (unnamed in
/// the engine map): an entry of mode 1 matches when all its bits are in
/// `mask`, one of mode 2 when any of them is.
pub fn fn_00ae6d30(e: &mut Engine, this: Ptr<BSAudioManager>, mask: u32) -> bool {
    let size = e.get(this, BSAudioManager::iPauseListSize);
    for index in 0..size {
        let entry = pause_entry(e, this, index);
        let held = e.mem.u32(entry);
        let mode = e.mem.u16(entry + 4);
        if mode == 1 && mask & held == held {
            return true;
        }
        if mode == 2 && mask & held != 0 {
            return true;
        }
    }
    false
}

// Translated from 00ae6de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts `mask` into the first free entry of the pause table (unnamed in the
/// engine map): mode 1 when `exact` is set, else 2. False when the table is
/// full.
pub fn fn_00ae6de0(e: &mut Engine, this: Ptr<BSAudioManager>, mask: u32, exact: u8) -> bool {
    let size = e.get(this, BSAudioManager::iPauseListSize);
    for index in 0..size {
        let entry = pause_entry(e, this, index);
        if e.mem.u16(entry + 4) == 0 {
            e.mem.set_u16(entry + 4, if exact != 0 { 1 } else { 2 });
            e.mem.set_u32(entry, mask);
            return true;
        }
    }
    false
}

// Translated from 00ae6e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the pause-table entry that holds `mask` in mode 1 (`exact` set) or
/// mode 2 (unnamed in the engine map). False when there is none.
pub fn fn_00ae6e60(e: &mut Engine, this: Ptr<BSAudioManager>, mask: u32, exact: u8) -> bool {
    let size = e.get(this, BSAudioManager::iPauseListSize);
    for index in 0..size {
        let entry = pause_entry(e, this, index);
        let mode = e.mem.u16(entry + 4);
        if e.mem.u32(entry) == mask && ((exact == 0 && mode == 2) || (exact != 0 && mode == 1)) {
            e.mem.set_u16(entry + 4, 0);
            e.mem.set_u32(entry, 0);
            return true;
        }
    }
    false
}

// Translated from 00ae6f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::RemoveLoadTask` (Xbox PDB): under the load-task
/// critical section (+0x10c), removes `task` from `CacheLoadTaskList`
/// (+0x124).
pub fn bs_audio_manager_remove_load_task(e: &mut Engine, this: Ptr<BSAudioManager>, task: u32) {
    let section = this.addr() + 0x10c;
    enter_section(e, section);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), task);
        fn_00ae8150(e, this.addr() + 0x124, slot.addr());
    });
    leave_section(e, section);
}

// Translated from 00ae6f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AudioLoadTask` constructor (unnamed in the engine map): the base
/// constructor (`00c3ce60`, argument 0), the vtable `010a3718`, the sound
/// (+0x30), the sound's file name (+0x36) fixed into a 260-byte buffer
/// (`00af4310`) and set as the task's file name (`00c3cee0`), and context 3
/// (`00c3cf60`). Returns `this`.
// Translation incomplete: 00ae6f50 - C++ exception unwinding (SEH frame state) and the /GS stack cookie check (00ec408c) are not translated.
pub fn fn_00ae6f50(e: &mut Engine, this: Ptr<AudioLoadTask>, sound: u32) -> Ptr<AudioLoadTask> {
    e.call(LOAD_TASK_BASE_CONSTRUCTOR, &[this.addr(), 0]);
    e.mem.set_u32(this.addr(), AUDIO_LOAD_TASK_VTABLE);
    e.set(this, AudioLoadTask::pSound, sound);
    e.with_stack(0x104, |e, buffer| {
        let sound = e.get(this, AudioLoadTask::pSound);
        e.call(
            FILE_NAME_FIX,
            &[sound + SOUND_FILENAME, buffer.addr(), 0x104],
        );
        e.call(LOAD_TASK_SET_FILE_NAME, &[this.addr(), buffer.addr()]);
    });
    e.call(LOAD_TASK_SET_CONTEXT, &[this.addr(), 3]);
    this
}

// Translated from 00ae7020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AudioLoadTask` destructor body (unnamed in the engine map): resets
/// the vtable to `010a3718` and runs the base destructor (`00c3cea0`). The
/// sound pointer is copied onto itself and stays.
pub fn fn_00ae7020(e: &mut Engine, this: Ptr<AudioLoadTask>) {
    e.mem.set_u32(this.addr(), AUDIO_LOAD_TASK_VTABLE);
    e.call(LOAD_TASK_BASE_DESTRUCTOR, &[this.addr()]);
}

// Translated from 00ae7050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AudioLoadTask::Run` (Xbox PDB): loads the task's sound. When the sound
/// has no hash but a directory hash, its hashes are calculated (`00ae9d50`);
/// if it is in the cache then (`IsInCache`, the manager singleton) a sound
/// without type bit 0x8000 loses state bits 0x200 and 0x1000 and gets 0x10,
/// and one with it is deleted. A sound with type bit 0x8000 whose hash is the
/// one `011f6e0c` remembers, less than a second after the tick
/// `011f6e10`, is deleted. Otherwise the sound is opened (slot 0x24); with
/// type bit 0x8000 a successful open remembers the hash and the tick and
/// queues message 0x33 (add sound to cache) with the sound, a failed one
/// deletes it.
// Translation incomplete: 00ae7050 - C++ exception unwinding (SEH frame state) around the message's temporary is not translated.
pub fn audio_load_task_run(e: &mut Engine, this: Ptr<AudioLoadTask>) {
    let sound = e.get(this, AudioLoadTask::pSound);
    let cache_flag = e.mem.u32(sound + SOUND_TYPE_FLAGS) & 0x8000 != 0;
    if e.mem.u32(sound + SOUND_HASH) == 0 && e.mem.u32(sound + SOUND_DIRECTORY_HASH) != 0 {
        e.call(SOUND_CALCULATE_HASHES, &[sound, 1]);
        let hash = e.mem.u32(sound + SOUND_HASH);
        let manager = bs_audio_manager_q_instance(e);
        if bs_audio_manager_is_in_cache(e, manager, hash, 0) {
            if !cache_flag {
                let mut state = e.mem.u32(sound + SOUND_STATE_FLAGS);
                state &= !0x200;
                state &= !0x1000;
                state |= 0x10;
                e.mem.set_u32(sound + SOUND_STATE_FLAGS, state);
            } else {
                e.vcall(sound, SOUND_SLOT_DESTRUCTOR, &[1]);
            }
            return;
        }
    }
    if cache_flag && e.mem.u32(LAST_CACHED_HASH) == e.mem.u32(sound + SOUND_HASH) {
        let now = e.call(IMPORT_GET_TICK_COUNT, &[]).u32();
        if now.wrapping_sub(e.mem.u32(LAST_CACHED_TICK)) < 1000 {
            e.vcall(sound, SOUND_SLOT_DESTRUCTOR, &[1]);
            return;
        }
    }
    let opened = e.vcall(sound, SOUND_SLOT_OPEN, &[]).bool();
    if !cache_flag {
        return;
    }
    if opened {
        let hash = e.mem.u32(sound + SOUND_HASH);
        e.mem.set_u32(LAST_CACHED_HASH, hash);
        let now = e.call(IMPORT_GET_TICK_COUNT, &[]).u32();
        e.mem.set_u32(LAST_CACHED_TICK, now);
        let manager = bs_audio_manager_q_instance(e);
        queue_plain(e, manager, 0x33, 0, 0, sound);
    } else {
        e.vcall(sound, SOUND_SLOT_DESTRUCTOR, &[1]);
    }
}

// Translated from 00ae7360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AudioLoadTask::Finish` (Xbox PDB): removes the task from the manager's
/// load list (`RemoveLoadTask` on the singleton). The sound pointer is
/// copied onto itself.
pub fn audio_load_task_finish(e: &mut Engine, this: Ptr<AudioLoadTask>) {
    let manager = bs_audio_manager_q_instance(e);
    bs_audio_manager_remove_load_task(e, manager, this.addr());
}

// Translated from 00ae7390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AudioLoadTask::Cancel` (Xbox PDB): removes the task from the manager's
/// load list; a sound with type bit 0x8000 is deleted, any other loses state
/// bit 0x200 and gets 0x10. Both stack words are ignored.
pub fn audio_load_task_cancel(
    e: &mut Engine,
    this: Ptr<AudioLoadTask>,
    _unused_1: u32,
    _unused_2: u32,
) {
    let manager = bs_audio_manager_q_instance(e);
    bs_audio_manager_remove_load_task(e, manager, this.addr());
    let sound = e.get(this, AudioLoadTask::pSound);
    if sound == 0 {
        return;
    }
    if e.mem.u32(sound + SOUND_TYPE_FLAGS) & 0x8000 != 0 {
        e.vcall(sound, SOUND_SLOT_DESTRUCTOR, &[1]);
    } else {
        let mut state = e.mem.u32(sound + SOUND_STATE_FLAGS);
        state &= !0x200;
        state |= 0x10;
        e.mem.set_u32(sound + SOUND_STATE_FLAGS, state);
    }
}

// Translated from 00ae7480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AudioLoadTask::GetDescription` (Xbox PDB): hands the two caller words
/// and the text `audio load task` to `00c3d0e0` (the base description
/// formatter) and returns its result.
pub fn audio_load_task_get_description(
    e: &mut Engine,
    this: Ptr<AudioLoadTask>,
    buffer: u32,
    size: u32,
) -> u32 {
    e.call(
        LOAD_TASK_DESCRIBE,
        &[this.addr(), buffer, size, LOAD_TASK_DESCRIPTION],
    )
    .u32()
}

// Translated from 00ae74b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManagerThread::ThreadProc` (Xbox PDB): waits for the thread's
/// first semaphore (`m_SuspendSema`), counts it down, clears `m_bSuspended`
/// and then never returns: while the load queue object `01202d98` exists it
/// runs `Process` on the manager singleton, and sleeps a millisecond. (The
/// engine runs one thread, so nothing calls this.)
pub fn bs_audio_manager_thread_thread_proc(e: &mut Engine, this: Ptr<BSAudioManagerThread>) -> u32 {
    let semaphore = this.addr() + 0x30;
    let handle = e.mem.u32(semaphore + 4);
    e.call(IMPORT_WAIT_FOR_SINGLE_OBJECT, &[handle, 0xffff_ffff]);
    e.call(IMPORT_INTERLOCKED_DECREMENT, &[semaphore]);
    e.set(this, BSAudioManagerThread::m_bSuspended, false);
    loop {
        if e.mem.u32(LOAD_QUEUE_GLOBAL) != 0 {
            let manager = bs_audio_manager_q_instance(e);
            bs_audio_manager_process(e, manager);
        }
        e.call(IMPORT_SLEEP, &[1]);
    }
}

// Translated from 00ae7510 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AudioMessageStack` constructor (unnamed in the engine map): an empty
/// stack that is not bottomless. Returns `this`.
pub fn fn_00ae7510(e: &mut Engine, this: Ptr<AudioMessageStack>) -> Ptr<AudioMessageStack> {
    e.set(this, AudioMessageStack::iCount, 0);
    e.set(this, AudioMessageStack::pTop, 0);
    e.set(this, AudioMessageStack::bBottomless, false);
    e.set(this, AudioMessageStack::iTimeLastAllocated, 0);
    this
}

// Translated from 00ae7550 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AudioMessageStack` destructor (unnamed in the engine map): pops
/// every message and destroys and frees it.
pub fn fn_00ae7550(e: &mut Engine, this: Ptr<AudioMessageStack>) {
    loop {
        let top = e.get(this, AudioMessageStack::pTop);
        if top == 0 {
            break;
        }
        let next = e.mem.u32(top + 0xc);
        e.set(this, AudioMessageStack::pTop, next);
        let count = e.get(this, AudioMessageStack::iCount);
        e.set(this, AudioMessageStack::iCount, count.wrapping_sub(1));
        e.mem.set_u32(top + 0xc, 0);
        e.call(MESSAGE_DESTRUCTOR, &[top]);
        e.call(DEALLOCATE, &[MEMORY_MANAGER, top]);
    }
}

// Translated from 00ae75f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AudioMessageStack::Push` (Xbox PDB): puts `message` on the stack. A
/// bottomless stack of more than 0x200 messages whose last allocation was
/// over a second ago deletes the message instead. A null message does
/// nothing.
pub fn audio_message_stack_push(e: &mut Engine, this: Ptr<AudioMessageStack>, message: u32) {
    if message == 0 {
        return;
    }
    if e.get(this, AudioMessageStack::bBottomless) && e.get(this, AudioMessageStack::iCount) > 0x200
    {
        let now = e.call(IMPORT_GET_TICK_COUNT, &[]).u32();
        if now.wrapping_sub(e.get(this, AudioMessageStack::iTimeLastAllocated)) > 1000 {
            e.call(MESSAGE_DESTRUCTOR, &[message]);
            e.call(DEALLOCATE, &[MEMORY_MANAGER, message]);
            return;
        }
    }
    let top = e.get(this, AudioMessageStack::pTop);
    e.mem.set_u32(message + 0xc, top);
    e.set(this, AudioMessageStack::pTop, message);
    let count = e.get(this, AudioMessageStack::iCount);
    e.set(this, AudioMessageStack::iCount, count.wrapping_add(1));
}

// Translated from 00ae76b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pop of an `AudioMessageStack` (unnamed in the engine map): the top
/// message with its link cleared; on an empty bottomless stack a new
/// message (0x20 bytes, built by `00ae8650`) with `iTimeLastAllocated` set
/// to now; else 0.
// Translation incomplete: 00ae76b0 - C++ exception unwinding (SEH frame state) around the allocation is not translated.
pub fn fn_00ae76b0(e: &mut Engine, this: Ptr<AudioMessageStack>) -> u32 {
    let top = e.get(this, AudioMessageStack::pTop);
    if top != 0 {
        let next = e.mem.u32(top + 0xc);
        e.set(this, AudioMessageStack::pTop, next);
        e.mem.set_u32(top + 0xc, 0);
        let count = e.get(this, AudioMessageStack::iCount);
        e.set(this, AudioMessageStack::iCount, count.wrapping_sub(1));
        return top;
    }
    if e.get(this, AudioMessageStack::bBottomless) {
        let raw = e.call(ALLOCATE, &[MEMORY_MANAGER, 0x20]).u32();
        let message = if raw != 0 {
            e.call(MESSAGE_CONSTRUCTOR, &[raw]).u32()
        } else {
            0
        };
        e.mem.set_u32(message + 0xc, 0);
        let now = e.call(IMPORT_GET_TICK_COUNT, &[]).u32();
        e.set(this, AudioMessageStack::iTimeLastAllocated, now);
        return message;
    }
    0
}

// Translated from 00ae77a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::~BSAudioManager`'s scalar deleting destructor (Xbox
/// PDB): the destructor body (`00ad93f0`), then, when bit 0 of `flags` is
/// set, the object is freed. Returns `this`.
pub fn bs_audio_manager_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    flags: u32,
) -> Ptr<BSAudioManager> {
    fn_00ad93f0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &[MEMORY_MANAGER, this.addr()]);
    }
    this
}

// Translated from 00ae77d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManagerThread::BSAudioManagerThread` (Xbox PDB): the `BSThread`
/// constructor (`00aa6360`), the vtable `010a33d0`, and the two semaphores
/// (+0x30 and +0x3c: count 0, maximum 1, each made with `CreateSemaphoreA`);
/// `m_bSuspended` starts set. Returns `this`.
pub fn bs_audio_manager_thread_constructor(
    e: &mut Engine,
    this: Ptr<BSAudioManagerThread>,
) -> Ptr<BSAudioManagerThread> {
    e.call(THREAD_BASE_CONSTRUCTOR, &[this.addr()]);
    e.mem.set_u32(this.addr(), BSAUDIO_MANAGER_THREAD_VTABLE);
    for semaphore in [this.addr() + 0x30, this.addr() + 0x3c] {
        e.mem.set_u32(semaphore, 0);
        e.mem.set_u32(semaphore + 8, 1);
        let handle = e.call(IMPORT_CREATE_SEMAPHORE, &[0, 0, 1, 0]).u32();
        e.mem.set_u32(semaphore + 4, handle);
    }
    e.set(this, BSAudioManagerThread::m_bSuspended, true);
    this
}

// Translated from 00ae7870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManagerThread`'s scalar deleting destructor (Xbox PDB): the
/// destructor body (`00ae78a0`), then, when bit 0 of `flags` is set, the
/// object is freed. Returns `this`.
pub fn bs_audio_manager_thread_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSAudioManagerThread>,
    flags: u32,
) -> Ptr<BSAudioManagerThread> {
    fn_00ae78a0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &[MEMORY_MANAGER, this.addr()]);
    }
    this
}

// Translated from 00ae78a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSAudioManagerThread` destructor body (unnamed in the engine map):
/// resets the vtable to `010a33d0`, closes the two semaphore handles (+0x40,
/// +0x34) and runs the `BSThread` destructor (`00aa63c0`).
pub fn fn_00ae78a0(e: &mut Engine, this: Ptr<BSAudioManagerThread>) {
    e.mem.set_u32(this.addr(), BSAUDIO_MANAGER_THREAD_VTABLE);
    let handle = e.mem.u32(this.addr() + 0x40);
    e.call(IMPORT_CLOSE_HANDLE, &[handle]);
    let handle = e.mem.u32(this.addr() + 0x34);
    e.call(IMPORT_CLOSE_HANDLE, &[handle]);
    e.call(THREAD_BASE_DESTRUCTOR, &[this.addr()]);
}

// Translated from 00ae7920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundInfo::operator=` (Xbox PDB): member-wise copy of `other` (the id,
/// the two floats, the type, the duration, the playing byte) and
/// `BSDebugSoundInfo::operator=` for the debug record at +0x18. Returns
/// `this`.
pub fn bs_sound_info_operator_assign(
    e: &mut Engine,
    this: Ptr<BSSoundInfo>,
    other: Ptr<BSSoundInfo>,
) -> Ptr<BSSoundInfo> {
    let id = e.get(other, BSSoundInfo::iID);
    e.set(this, BSSoundInfo::iID, id);
    let frequency = x87_quiet(e.get(other, BSSoundInfo::fFrequency));
    e.set(this, BSSoundInfo::fFrequency, frequency);
    let volume = x87_quiet(e.get(other, BSSoundInfo::fVolume));
    e.set(this, BSSoundInfo::fVolume, volume);
    let sound_type = e.get(other, BSSoundInfo::uiSoundType);
    e.set(this, BSSoundInfo::uiSoundType, sound_type);
    let duration = e.get(other, BSSoundInfo::uiDuration);
    e.set(this, BSSoundInfo::uiDuration, duration);
    let playing = e.get(other, BSSoundInfo::bIsPlaying);
    e.set(this, BSSoundInfo::bIsPlaying, playing);
    e.call(
        DEBUG_SOUND_INFO_ASSIGN,
        &[this.addr() + 0x18, other.addr() + 0x18],
    );
    this
}

// Translated from 00ae7990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The member-wise assignment of a `BSSoundMessage` (unnamed in the engine
/// map): the four words, the smart pointer (`NiPointer::operator=`), the
/// three vector floats and then the five 16-bit curve values, which lie over
/// the first 10 bytes of the vector. The floats go through the x87 stack
/// (a signalling NaN comes back quiet), but the 16-bit copies then overwrite
/// the first ten bytes with the source's own, so only the last two bytes of
/// the third float keep the quieted value. Returns `this`.
pub fn fn_00ae7990(
    e: &mut Engine,
    this: Ptr<BSSoundMessage>,
    other: Ptr<BSSoundMessage>,
) -> Ptr<BSSoundMessage> {
    for offset in [0, 4, 8, 0xc] {
        let word = e.mem.u32(other.addr() + offset);
        e.mem.set_u32(this.addr() + offset, word);
    }
    e.call(
        SMART_POINTER_ASSIGN,
        &[this.addr() + 0x10, other.addr() + 0x10],
    );
    for index in 0..3 {
        let value = x87_quiet(e.mem.f32(other.addr() + 0x14 + index * 4));
        e.mem.set_f32(this.addr() + 0x14 + index * 4, value);
    }
    for index in 0..5 {
        let value = e.mem.u16(other.addr() + 0x14 + index * 2);
        e.mem.set_u16(this.addr() + 0x14 + index * 2, value);
    }
    this
}

/// The suspend step of the audio thread: sets `m_bSuspended`, releases the
/// second semaphore (`ReleaseSemaphore` by one and its count up).
fn thread_signal_suspended(e: &mut Engine, this: Ptr<BSAudioManagerThread>) {
    e.set(this, BSAudioManagerThread::m_bSuspended, true);
    let semaphore = this.addr() + 0x3c;
    let handle = e.mem.u32(semaphore + 4);
    e.call(IMPORT_RELEASE_SEMAPHORE, &[handle, 1, 0]);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &[semaphore]);
}

// Translated from 00ae7a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// A step of the audio thread (unnamed in the engine map): marks it
/// suspended and releases the second semaphore (+0x3c), then waits for the
/// first (+0x30, infinite), counts it down and clears the suspended flag.
pub fn fn_00ae7a50(e: &mut Engine, this: Ptr<BSAudioManagerThread>) {
    thread_signal_suspended(e, this);
    let semaphore = this.addr() + 0x30;
    let handle = e.mem.u32(semaphore + 4);
    e.call(IMPORT_WAIT_FOR_SINGLE_OBJECT, &[handle, 0xffff_ffff]);
    e.call(IMPORT_INTERLOCKED_DECREMENT, &[semaphore]);
    e.set(this, BSAudioManagerThread::m_bSuspended, false);
}

// Translated from 00ae7ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A step of the audio thread (unnamed in the engine map): marks it
/// suspended, releases the second semaphore (+0x3c) and calls `00aa6590`
/// with two zeros on the thread.
pub fn fn_00ae7ac0(e: &mut Engine, this: Ptr<BSAudioManagerThread>) {
    thread_signal_suspended(e, this);
    e.call(THREAD_STOP_STEP, &[this.addr(), 0, 0]);
}

// Translated from 00ae7b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AudioLoadTask`'s scalar deleting destructor (Xbox PDB): the destructor
/// body (`00ae7020`), then, when bit 0 of `flags` is set, the task is freed.
/// Returns `this`.
pub fn audio_load_task_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<AudioLoadTask>,
    flags: u32,
) -> Ptr<AudioLoadTask> {
    fn_00ae7020(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &[MEMORY_MANAGER, this.addr()]);
    }
    this
}

/// The constructor of the sound-id and state maps: `vtable` (the map's
/// base vtable), `buckets` buckets in a table of `buckets * 4` bytes
/// (`00aa1070`) cleared with `memset`, no items. Returns `this`.
fn map_constructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    vtable: u32,
    buckets: u32,
) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), vtable);
    e.set(this, NiTPointerMap::m_uiHashSize, buckets);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    let size = buckets << 2;
    let table = e.call(TABLE_ALLOCATE, &[size]).u32();
    e.set(this, NiTPointerMap::m_ppkHashTable, table);
    e.call(MEMSET, &[table, 0, size]);
    this
}

// Translated from 00ae7b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `SoundList`'s map base (unnamed in the engine map):
/// vtable `010a3370`, `buckets` empty buckets. Returns `this`.
pub fn fn_00ae7b40(e: &mut Engine, this: Ptr<NiTPointerMap>, buckets: u32) -> Ptr<NiTPointerMap> {
    map_constructor(e, this, 0x010a_3370, buckets)
}

// Translated from 00ae7bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the state maps' base (unnamed in the engine map):
/// vtable `010a3390`, `buckets` empty buckets. Returns `this`.
pub fn fn_00ae7bc0(e: &mut Engine, this: Ptr<NiTPointerMap>, buckets: u32) -> Ptr<NiTPointerMap> {
    map_constructor(e, this, 0x010a_3390, buckets)
}

// Translated from 00ae7c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the moving-reference map's base (unnamed in the
/// engine map): vtable `010a33b0`, `buckets` empty buckets. Returns `this`.
pub fn fn_00ae7c40(e: &mut Engine, this: Ptr<NiTPointerMap>, buckets: u32) -> Ptr<NiTPointerMap> {
    map_constructor(e, this, 0x010a_33b0, buckets)
}

// Translated from 00ae7cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the value for `key` in the moving-reference map (the
/// `NiTPointerMap<int, NiPointer<NiAVObject>>`, unnamed in the engine map).
/// `value` is a smart pointer passed by value: the function drops that
/// reference when it is done. The bucket of the key (map slot 4) is searched
/// with the key comparison (slot 8); a match takes the value
/// (`NiPointer::operator=`). Without a match a node is made (slot 0x14),
/// filled with the key and the value (slot 0xc, which takes a reference to
/// the value) and put at the head of the bucket, and the item count grows.
// Translation incomplete: 00ae7cc0 - C++ exception unwinding (SEH frame state) around the node is not translated.
pub fn fn_00ae7cc0(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, value: u32) {
    let index = e.vcall(this.addr(), MAP_SLOT_HASH, &[key]).u32();
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut node = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    while node != 0 {
        let node_key = e.mem.u32(node + 4);
        if e.vcall(this.addr(), MAP_SLOT_KEY_EQUAL, &[key, node_key])
            .bool()
        {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), value);
                e.call(SMART_POINTER_ASSIGN, &[node + 8, slot.addr()]);
            });
            drop_reference(e, value);
            return;
        }
        node = e.mem.u32(node);
    }
    let node = e.vcall(this.addr(), MAP_SLOT_NEW_NODE, &[]).u32();
    add_reference(e, value);
    e.vcall(this.addr(), MAP_SLOT_INIT_NODE, &[node, key, value]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let head = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    e.mem.set_u32(node, head);
    e.mem
        .set_u32(table.wrapping_add(index.wrapping_mul(4)), node);
    let count = e.get(this, NiTPointerMap::m_uiCount);
    e.set(this, NiTPointerMap::m_uiCount, count.wrapping_add(1));
    drop_reference(e, value);
}

// Translated from 00ae7e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks `key` up in the moving-reference map (unnamed in the engine map):
/// the bucket of the key (map slot 4) is searched with the key comparison
/// (slot 8); on a match the smart pointer at `out` takes the item's value
/// (`NiPointer::operator=`). Returns whether the key was found.
pub fn fn_00ae7e60(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, out: u32) -> bool {
    let index = e.vcall(this.addr(), MAP_SLOT_HASH, &[key]).u32();
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut node = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    while node != 0 {
        let node_key = e.mem.u32(node + 4);
        if e.vcall(this.addr(), MAP_SLOT_KEY_EQUAL, &[key, node_key])
            .bool()
        {
            e.call(SMART_POINTER_ASSIGN, &[out, node + 8]);
            return true;
        }
        node = e.mem.u32(node);
    }
    false
}

// Translated from 00ae7ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetNext` of the moving-reference map (unnamed in the engine map):
/// `iterator` holds the current item; its key goes to `key_out`, its value
/// into the smart pointer at `value_out`, and `*iterator` moves to the next
/// item of the bucket, else to the first item of the next non-empty bucket,
/// else 0.
pub fn fn_00ae7ee0(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    iterator: u32,
    key_out: u32,
    value_out: u32,
) {
    let item = e.mem.u32(iterator);
    let key = e.mem.u32(item + 4);
    e.mem.set_u32(key_out, key);
    e.call(SMART_POINTER_ASSIGN, &[value_out, item + 8]);
    let next = e.mem.u32(item);
    if next != 0 {
        e.mem.set_u32(iterator, next);
        return;
    }
    let mut bucket = e.vcall(this.addr(), MAP_SLOT_HASH, &[key]).u32();
    let size = e.get(this, NiTPointerMap::m_uiHashSize);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    loop {
        bucket = bucket.wrapping_add(1);
        if bucket >= size {
            e.mem.set_u32(iterator, 0);
            return;
        }
        let found = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
        if found != 0 {
            e.mem.set_u32(iterator, found);
            return;
        }
    }
}

// Translated from 00ae8150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the first node of the pointer list `list` whose element equals
/// the word `value` points at (unnamed in the engine map): the node is
/// removed through `00ae8090` and its element returned; without a match the
/// word itself is returned.
pub fn fn_00ae8150(e: &mut Engine, list: u32, value: u32) -> u32 {
    let wanted = e.mem.u32(value);
    let mut node = e.mem.u32(list);
    let mut found = 0;
    while node != 0 {
        let current = node;
        node = e.mem.u32(node);
        if wanted == e.mem.u32(current + 8) {
            found = current;
            break;
        }
    }
    if found != 0 {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), found);
            e.call(LIST_REMOVE_AT, &[list, slot.addr()]).u32()
        })
    } else {
        wanted
    }
}

// Translated from 00ae81e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleList<AudioDebugString *>::Sort` (Xbox PDB): sorts the list
/// starting at the node `this` with `compare` (a cdecl comparison of two
/// elements). The elements of the first nodes (as many as there are nodes
/// with a non-null element) are copied into an array, shell-sorted
/// (`00ae8550`); the list after the head is freed (`00ae83e0`) and rebuilt:
/// the head keeps the first element, a new 8-byte node (`Allocate`) holds
/// each other one. (The compiler's fallback for a failed stack allocation,
/// a bubble sort through `00ae8470`, cannot happen here.)
// Translation incomplete: 00ae81e0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn bs_simple_list_audio_debug_string_sort(
    e: &mut Engine,
    this: Ptr<BSSimpleList>,
    compare: u32,
) {
    let head = this.addr();
    let mut count = 0u32;
    let mut node = head;
    while node != 0 {
        if e.mem.u32(node) != 0 {
            count += 1;
        }
        node = e.mem.u32(node + 4);
    }
    e.with_stack(count * 4 + 4, |e, array| {
        let mut node = head;
        for index in 0..count {
            let element = e.mem.u32(node);
            e.mem.set_u32(array.addr() + index * 4, element);
            node = e.mem.u32(node + 4);
        }
        fn_00ae8550(e, Ptr::new(head), array.addr(), compare, count);
        fn_00ae83e0(e, Ptr::new(head));
        let mut current = head;
        let mut previous = 0;
        for index in 0..count {
            let element = e.mem.u32(array.addr() + index * 4);
            if index == 0 {
                e.mem.set_u32(current, element);
                e.mem.set_u32(current + 4, 0);
            } else {
                let raw = e.call(ALLOCATE, &[MEMORY_MANAGER, 8]).u32();
                if raw != 0 {
                    e.mem.set_u32(raw, element);
                    e.mem.set_u32(raw + 4, 0);
                }
                current = raw;
            }
            if previous != 0 {
                e.mem.set_u32(previous + 4, current);
            }
            previous = current;
        }
    });
}

// Translated from 00ae8370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties a pointer list (unnamed in the engine map): every node's element
/// is cleared and the node freed (`0045cee0` on the list's allocator at
/// +8); then count, head and tail are 0.
pub fn fn_00ae8370(e: &mut Engine, list: Ptr<NiTPointerList>) {
    let mut node = e.get(list, NiTPointerList::m_pkHead);
    while node != 0 {
        let current = node;
        node = e.mem.u32(node);
        e.mem.set_u32(current + 8, 0);
        e.call(LIST_NODE_FREE, &[list.addr() + 8, current]);
    }
    e.set(list, NiTPointerList::m_uiCount, 0);
    e.set(list, NiTPointerList::m_pkHead, 0);
    e.set(list, NiTPointerList::m_pkTail, 0);
}

// Translated from 00ae83e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the nodes that follow `this` in a singly linked chain (next
/// pointer at +4; unnamed in the engine map): each follower is detached and
/// deleted (its own followers first, then it is freed), and the first word
/// of `this` is cleared.
pub fn fn_00ae83e0(e: &mut Engine, this: Ptr<BSSimpleList>) {
    loop {
        let follower = e.get(this, BSSimpleList::m_pkNext);
        if follower == 0 {
            break;
        }
        let after = e.mem.u32(follower + 4);
        e.mem.set_u32(follower + 4, 0);
        fn_00ae83e0(e, Ptr::new(follower));
        e.call(DEALLOCATE, &[MEMORY_MANAGER, follower]);
        e.set(this, BSSimpleList::m_pkNext, after);
    }
    e.set(this, BSSimpleList::m_item, 0);
}

// Translated from 00ae8470 (decompiled, FalloutNV.exe 1.4.0.525)
/// A bubble sort of the chain of nodes starting at `this` (item at +0, next
/// at +4; unnamed in the engine map) by `compare` (cdecl, previous element
/// then current): as many passes as there are nodes with a non-null item,
/// swapping the (non-null) items of neighbours that compare greater, and
/// stopping at a pass without a swap.
pub fn fn_00ae8470(e: &mut Engine, this: Ptr<BSSimpleList>, compare: u32) {
    let head = this.addr();
    let mut passes = 0i32;
    let mut node = head;
    while node != 0 {
        if e.mem.u32(node) != 0 {
            passes += 1;
        }
        node = e.mem.u32(node + 4);
    }
    let mut swapped = true;
    while passes != 0 && swapped {
        swapped = false;
        let mut current = head;
        let mut previous = 0;
        while current != 0 {
            if previous != 0 {
                let current_item = e.mem.u32(current);
                let previous_item = e.mem.u32(previous);
                if e.call(compare, &[previous_item, current_item]).i32() > 0 {
                    if previous_item != 0 {
                        e.mem.set_u32(current, previous_item);
                    }
                    if current_item != 0 {
                        e.mem.set_u32(previous, current_item);
                    }
                    swapped = true;
                }
            }
            previous = current;
            current = e.mem.u32(current + 4);
        }
        passes -= 1;
    }
}

// Translated from 00ae8550 (decompiled, FalloutNV.exe 1.4.0.525)
/// A shell sort of the `count` words at `array` by `compare` (a cdecl
/// comparison; the word being inserted first, a negative answer moves it
/// down), with the gaps 1, 4, 13, ... below a ninth of the range (unnamed
/// in the engine map). `this` is not used.
pub fn fn_00ae8550(e: &mut Engine, _this: Ptr<BSSimpleList>, array: u32, compare: u32, count: u32) {
    let high = (count as i32).wrapping_sub(1);
    let mut gap = 1i32;
    while gap <= high / 9 {
        gap = gap * 3 + 1;
    }
    while gap > 0 {
        let mut index = gap;
        while index <= high {
            let mut position = index;
            let inserted = e.mem.u32(array.wrapping_add(index as u32 * 4));
            while position >= gap {
                let other = e.mem.u32(array.wrapping_add((position - gap) as u32 * 4));
                if e.call(compare, &[inserted, other]).i32() >= 0 {
                    break;
                }
                e.mem
                    .set_u32(array.wrapping_add(position as u32 * 4), other);
                position -= gap;
            }
            e.mem
                .set_u32(array.wrapping_add(position as u32 * 4), inserted);
            index += 1;
        }
        gap /= 3;
    }
}

/// Copies the float at `from` to `to` the way `FLD` / `FSTP` do.
fn copy_float_x87(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

// Translated from 00ae8650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of a `BSSoundMessage` (unnamed in the engine map): the
/// smart pointer at +0x10 is set to null (the inlined smart-pointer
/// constructor's reference increment can never run, the pointer having just
/// been cleared), the four words at +0 to +0xc are cleared and the three
/// vector floats at +0x14 are set to 0.0. Returns `this`.
pub fn fn_00ae8650(e: &mut Engine, this: Ptr<BSSoundMessage>) -> Ptr<BSSoundMessage> {
    e.set(this, BSSoundMessage::spData, 0);
    e.set(this, BSSoundMessage::msg, 0);
    e.set(this, BSSoundMessage::iID, 0);
    e.set(this, BSSoundMessage::iData, 0);
    e.set(this, BSSoundMessage::pData, 0);
    e.set(this, BSSoundMessage::vector3_x, 0.0);
    e.set(this, BSSoundMessage::vector3_y, 0.0);
    e.set(this, BSSoundMessage::vector3_z, 0.0);
    this
}

// Translated from 00ae86d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ae86d0 - the exception-unwinding frame (FS:[0] handler, state 0 then -1) is not modelled; the ordinary behaviour is complete
/// `BSSoundMessage::~BSSoundMessage` (Xbox PDB): when the smart pointer at
/// +0x10 is not null its object is handed to `00c45830`; then the smart
/// pointer's destructor drops the reference and, at zero, calls the
/// object's virtual method at +4.
pub fn bs_sound_message_destructor(e: &mut Engine, this: Ptr<BSSoundMessage>) {
    let object = e.get(this, BSSoundMessage::spData);
    if object != 0 {
        e.call(RELEASE_MESSAGE_POINTER, &[object]);
    }
    let object = e.get(this, BSSoundMessage::spData);
    drop_reference(e, object);
}

// Translated from 00ae8770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDebugSoundInfo::BSDebugSoundInfo` (Xbox PDB): both name buffers start
/// empty and their pointers (+0 and +0x108) point at them (+4 and +0x10c),
/// the radii, times and attenuations are 0, the location is the default
/// vector at `011f426c` (copied as words) and the state flags are 0.
/// Returns `this`.
pub fn bs_debug_sound_info_constructor(
    e: &mut Engine,
    this: Ptr<BSDebugSoundInfo>,
) -> Ptr<BSDebugSoundInfo> {
    let base = this.addr();
    e.mem.set_u8(base + 0x04, 0);
    e.mem.set_u8(base + 0x10c, 0);
    e.set(this, BSDebugSoundInfo::pName, base + 0x04);
    e.set(this, BSDebugSoundInfo::pFileName, base + 0x10c);
    e.set(this, BSDebugSoundInfo::fRadiusMin, 0.0);
    e.set(this, BSDebugSoundInfo::fRadiusMax, 0.0);
    for index in 0..3 {
        let word = e.mem.u32(DEFAULT_MESSAGE_VECTOR + index * 4);
        e.mem.set_u32(base + 0x218 + index * 4, word);
    }
    e.set(this, BSDebugSoundInfo::iStateFlags, 0);
    e.set(this, BSDebugSoundInfo::fBeginTime, 0.0);
    e.set(this, BSDebugSoundInfo::fEndTime, 0.0);
    e.set(this, BSDebugSoundInfo::iStaticAttenuation, 0);
    e.set(this, BSDebugSoundInfo::iSystemAttenuation, 0);
    e.set(this, BSDebugSoundInfo::iDistanceAttenuation, 0);
    this
}

// Translated from 00ae8830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDebugSoundInfo::~BSDebugSoundInfo` (Xbox PDB): clears the two name
/// pointers, the radii, the state flags, the times and the three
/// attenuations (the location and the name buffers stay as they are).
pub fn bs_debug_sound_info_destructor(e: &mut Engine, this: Ptr<BSDebugSoundInfo>) {
    e.set(this, BSDebugSoundInfo::pName, 0);
    e.set(this, BSDebugSoundInfo::pFileName, 0);
    e.set(this, BSDebugSoundInfo::fRadiusMin, 0.0);
    e.set(this, BSDebugSoundInfo::fRadiusMax, 0.0);
    e.set(this, BSDebugSoundInfo::iStateFlags, 0);
    e.set(this, BSDebugSoundInfo::fBeginTime, 0.0);
    e.set(this, BSDebugSoundInfo::fEndTime, 0.0);
    e.set(this, BSDebugSoundInfo::iStaticAttenuation, 0);
    e.set(this, BSDebugSoundInfo::iSystemAttenuation, 0);
    e.set(this, BSDebugSoundInfo::iDistanceAttenuation, 0);
}

// Translated from 00ae88c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDebugSoundInfo::operator=` (Xbox PDB): the first byte of both of this
/// record's name buffers is cleared, then each of the two names is copied
/// with `strcpy` (to this record's own name pointers) when the other
/// record's pointer is not null. The radii and times are copied as floats
/// through the x87 stack, the location, state flags and attenuations as
/// words. Returns `this`.
pub fn bs_debug_sound_info_operator_assign(
    e: &mut Engine,
    this: Ptr<BSDebugSoundInfo>,
    other: Ptr<BSDebugSoundInfo>,
) -> Ptr<BSDebugSoundInfo> {
    let (to, from) = (this.addr(), other.addr());
    e.mem.set_u8(to + 0x04, 0);
    e.mem.set_u8(to + 0x10c, 0);
    let source = e.mem.u32(from);
    if source != 0 {
        let destination = e.mem.u32(to);
        e.call(STRCPY, &[destination, source]);
    }
    let source = e.mem.u32(from + 0x108);
    if source != 0 {
        let destination = e.mem.u32(to + 0x108);
        e.call(STRCPY, &[destination, source]);
    }
    copy_float_x87(e, from + 0x210, to + 0x210);
    copy_float_x87(e, from + 0x214, to + 0x214);
    for offset in [0x218, 0x21c, 0x220, 0x224] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    copy_float_x87(e, from + 0x228, to + 0x228);
    copy_float_x87(e, from + 0x22c, to + 0x22c);
    for offset in [0x230, 0x234, 0x238] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    this
}

// Translated from 00ae89f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundInfo::BSSoundInfo` (Xbox PDB): constructs the debug record at
/// +0x18, then clears the id, the frequency (0.0), the type, the duration
/// and the playing byte (the volume at +8 is not set). Returns `this`.
pub fn bs_sound_info_constructor(e: &mut Engine, this: Ptr<BSSoundInfo>) -> Ptr<BSSoundInfo> {
    e.call(DEBUG_SOUND_INFO_CONSTRUCTOR, &[this.addr() + 0x18]);
    e.set(this, BSSoundInfo::iID, 0);
    e.set(this, BSSoundInfo::fFrequency, 0.0);
    e.set(this, BSSoundInfo::uiSoundType, 0);
    e.set(this, BSSoundInfo::uiDuration, 0);
    e.set(this, BSSoundInfo::bIsPlaying, 0);
    this
}

// Translated from 00ae8a40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ae8a40 - the exception-unwinding frame (FS:[0] handler, state -1 then 0 then -1) is not modelled; the ordinary behaviour is complete
/// `BSSoundInfo::BSSoundInfo` overload 2 (Xbox PDB), the copy constructor:
/// constructs the debug record at +0x18, copies the id, the two floats (x87
/// load and store), the type, the duration and the playing byte of `other`
/// and assigns its debug record. Returns `this`.
pub fn bs_sound_info_copy_constructor(
    e: &mut Engine,
    this: Ptr<BSSoundInfo>,
    other: Ptr<BSSoundInfo>,
) -> Ptr<BSSoundInfo> {
    e.call(DEBUG_SOUND_INFO_CONSTRUCTOR, &[this.addr() + 0x18]);
    let (to, from) = (this.addr(), other.addr());
    let id = e.mem.u32(from);
    e.mem.set_u32(to, id);
    copy_float_x87(e, from + 0x04, to + 0x04);
    copy_float_x87(e, from + 0x08, to + 0x08);
    for offset in [0x0c, 0x10] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    let playing = e.mem.u8(from + 0x14);
    e.mem.set_u8(to + 0x14, playing);
    e.call(DEBUG_SOUND_INFO_ASSIGN, &[to + 0x18, from + 0x18]);
    this
}

// Translated from 00ae8af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundInfo::~BSSoundInfo` (Xbox PDB): clears the id, the two floats
/// (0.0), the type, the duration and the playing byte, then runs the debug
/// record's destructor (+0x18).
pub fn bs_sound_info_destructor(e: &mut Engine, this: Ptr<BSSoundInfo>) {
    e.set(this, BSSoundInfo::iID, 0);
    e.set(this, BSSoundInfo::fFrequency, 0.0);
    e.set(this, BSSoundInfo::fVolume, 0.0);
    e.set(this, BSSoundInfo::uiSoundType, 0);
    e.set(this, BSSoundInfo::uiDuration, 0);
    e.set(this, BSSoundInfo::bIsPlaying, 0);
    e.call(DEBUG_SOUND_INFO_DESTRUCTOR, &[this.addr() + 0x18]);
}

// Translated from 00ae8b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// A getter of a `BSSoundInfo`'s debug record (unnamed in the engine map):
/// copies the 0x23c bytes at +0x18 to `out` (`REP MOVSD`, 0x8f words) and
/// returns `out`.
pub fn fn_00ae8b40(e: &mut Engine, this: Ptr<BSSoundInfo>, out: u32) -> u32 {
    for index in 0..0x8f {
        let word = e.mem.u32(this.addr() + 0x18 + index * 4);
        e.mem.set_u32(out + index * 4, word);
    }
    out
}

// Translated from 00ae8b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method that returns the constant 2 in `AL` (unnamed in the engine map;
/// its only reference is the data word at `010a3744`, which lies in the
/// vtable of `AudioLoadTask`, `010a3718`, as slot 11; which method that slot
/// is, is not confirmed). `this` is not used.
pub fn fn_00ae8b80(_e: &mut Engine, _this: u32) -> u8 {
    2
}

// Translated from 00ae8b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A step on a sound (unnamed in the engine map; `ProcessMessage` calls it
/// to apply a flag): sets bit 0x400 of the sound's state flags (+0x10) when
/// the byte `flag` is not zero, clears it otherwise, then calls the sound's
/// virtual method at +0x3c (`SetVolume`) with the current volume (+0x24,
/// passed through the x87 stack).
pub fn fn_00ae8b90(e: &mut Engine, sound: u32, flag: u8) {
    let state = e.mem.u32(sound + SOUND_STATE_FLAGS);
    let state = if flag != 0 {
        state | 0x400
    } else {
        state & 0xffff_fbff
    };
    e.mem.set_u32(sound + SOUND_STATE_FLAGS, state);
    let volume = x87_quiet_bits(e.mem.u32(sound + SOUND_CURRENT_VOLUME));
    e.vcall(sound, SOUND_SLOT_SET_VOLUME, &args![volume]);
}

// Translated from 00ae8bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a pointer to a pointer list (the engine map names this
/// `BSAudioManager::GetFromCache`, but the body adds a node and stores the
/// pointer in it): a node from the list's allocator (`0043a010` on
/// `list + 8`), the word `element_slot` points at stored at node +8, the node
/// linked at the tail (`00a11db0`).
pub fn fn_00ae8bf0(e: &mut Engine, list: u32, element_slot: u32) {
    let node = e.call(LIST_NEW_NODE, &[list + 8]).u32();
    let element = e.mem.u32(element_slot);
    e.mem.set_u32(node + 8, element);
    e.call(LIST_ADD_NODE_TAIL, &[list, node]);
}

/// The destructor body shared by the pointer-map classes of the manager:
/// the vtable is set to `vtable`, the items are removed (`00ae91f0`) and the
/// bucket table (`this + 8`) freed (`00aa10f0`).
fn map_destroy(e: &mut Engine, this: u32, vtable: u32) {
    e.mem.set_u32(this, vtable);
    e.call(MAP_CLEAR, &[this]);
    let table = e.mem.u32(this + 8);
    e.call(FREE_TABLE, &[table]);
}

/// The destructor of a derived map class: the derived part (vtable
/// `vtable`, items removed) first, then the base's [`map_destroy`].
fn map_destroy_derived(e: &mut Engine, this: u32, vtable: u32, base_vtable: u32) {
    e.mem.set_u32(this, vtable);
    e.call(MAP_CLEAR, &[this]);
    map_destroy(e, this, base_vtable);
}

/// The deleting step of the scalar deleting destructors: the map is freed
/// when bit 0 of `flags` is set.
fn map_delete(e: &mut Engine, this: u32, flags: u32) {
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &[MEMORY_MANAGER, this]);
    }
}

// Translated from 00ae8c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `BSGameSound *` map's base class (vtable
/// `010a3370`, `NiTMapBase<NiTPointerAllocator<unsigned int>, int,
/// BSGameSound *>` by RTTI; unnamed in the engine map).
pub fn fn_00ae8c30(e: &mut Engine, this: u32) {
    map_destroy(e, this, SOUND_MAP_BASE_VTABLE);
}

// Translated from 00ae8c60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ae8c60 - the exception-unwinding frame (FS:[0] handler) is not modelled; the ordinary behaviour is complete
/// The destructor of the `BSGameSound *` map (`NiTPointerMap<int,
/// BSGameSound *>` by RTTI, vtable `010a3348`; unnamed in the engine map):
/// the derived part, then the base's.
pub fn fn_00ae8c60(e: &mut Engine, this: u32) {
    map_destroy_derived(e, this, SOUND_MAP_VTABLE, SOUND_MAP_BASE_VTABLE);
}

// Translated from 00ae8ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `BSSoundInfo *` map's base class (vtable
/// `010a3390`, `NiTMapBase<NiTPointerAllocator<unsigned int>, int,
/// BSSoundInfo *>` by RTTI; unnamed in the engine map).
pub fn fn_00ae8ce0(e: &mut Engine, this: u32) {
    map_destroy(e, this, STATE_MAP_BASE_VTABLE);
}

// Translated from 00ae8d10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ae8d10 - the exception-unwinding frame (FS:[0] handler) is not modelled; the ordinary behaviour is complete
/// The destructor of the `BSSoundInfo *` map (`NiTPointerMap<int,
/// BSSoundInfo *>` by RTTI, vtable `010a3328`; unnamed in the engine map):
/// the derived part, then the base's.
pub fn fn_00ae8d10(e: &mut Engine, this: u32) {
    map_destroy_derived(e, this, STATE_MAP_VTABLE, STATE_MAP_BASE_VTABLE);
}

// Translated from 00ae8d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the moving-reference map's base class (vtable
/// `010a33b0`, `NiTMapBase<NiTPointerAllocator<unsigned int>, int,
/// NiPointer<NiAVObject>>` by RTTI; unnamed in the engine map).
pub fn fn_00ae8d90(e: &mut Engine, this: u32) {
    map_destroy(e, this, REFERENCE_MAP_BASE_VTABLE);
}

// Translated from 00ae8dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The key comparison of the integer-keyed maps (unnamed in the engine map;
/// referenced from their vtables): whether the two keys are equal. `this` is
/// not used.
pub fn fn_00ae8dc0(_e: &mut Engine, _this: u32, key: u32, other_key: u32) -> bool {
    key == other_key
}

// Translated from 00ae8de0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ae8de0 - the exception-unwinding frame (FS:[0] handler) is not modelled; the ordinary behaviour is complete
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, int,
/// NiPointer<NiAVObject>>::SetValue` (Xbox PDB name): stores `key` at
/// `node` +4 and assigns the smart pointer `value` (passed by value) to the
/// one at `node` +8 (`NiPointer::operator=`); the by-value argument is then
/// destroyed, dropping its reference. The code works on `node` and never
/// reads `this`.
pub fn ni_t_map_base_reference_set_value(
    e: &mut Engine,
    _this: u32,
    node: u32,
    key: u32,
    value: u32,
) {
    e.mem.set_u32(node + 4, key);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(SMART_POINTER_ASSIGN, &[node + 8, slot.addr()]);
        let remaining = e.mem.u32(slot.addr());
        drop_reference(e, remaining);
    });
}

// Translated from 00ae8e70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ae8e70 - the exception-unwinding frame (FS:[0] handler) is not modelled; the ordinary behaviour is complete
/// The destructor of the moving-reference map (`NiTPointerMap<int,
/// NiPointer<NiAVObject>>` by RTTI, vtable `010a3308`; unnamed in the engine
/// map): the derived part, then the base's.
pub fn fn_00ae8e70(e: &mut Engine, this: u32) {
    map_destroy_derived(e, this, REFERENCE_MAP_VTABLE, REFERENCE_MAP_BASE_VTABLE);
}

// Translated from 00ae8ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<int, NiPointer<NiAVObject>>::DeleteItem` (Xbox PDB name):
/// sets the smart pointer in `node` +8 to null (`0066b0d0`), then frees the
/// node on the allocator at `this + 0xc` (`0045cee0`).
pub fn ni_t_pointer_map_reference_delete_item(e: &mut Engine, this: u32, node: u32) {
    e.call(SMART_POINTER_SET, &[node + 8, 0]);
    e.call(LIST_NODE_FREE, &[this + 0xc, node]);
}

// Translated from 00ae8f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of the `BSGameSound *` map's base class
/// (vtable `010a3370`): the destructor body, then the object is freed when
/// bit 0 of `flags` is set. Returns `this`.
pub fn fn_00ae8f20(e: &mut Engine, this: u32, flags: u32) -> u32 {
    map_destroy(e, this, SOUND_MAP_BASE_VTABLE);
    map_delete(e, this, flags);
    this
}

// Translated from 00ae8f70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ae8f70 - the exception-unwinding frame (FS:[0] handler) is not modelled; the ordinary behaviour is complete
/// The scalar deleting destructor of the `BSGameSound *` map (vtable
/// `010a3348`): the derived and base destruction, then the free. Returns
/// `this`.
pub fn fn_00ae8f70(e: &mut Engine, this: u32, flags: u32) -> u32 {
    map_destroy_derived(e, this, SOUND_MAP_VTABLE, SOUND_MAP_BASE_VTABLE);
    map_delete(e, this, flags);
    this
}

// Translated from 00ae9010 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of the `BSSoundInfo *` map's base class
/// (vtable `010a3390`). Returns `this`.
pub fn fn_00ae9010(e: &mut Engine, this: u32, flags: u32) -> u32 {
    map_destroy(e, this, STATE_MAP_BASE_VTABLE);
    map_delete(e, this, flags);
    this
}

// Translated from 00ae9060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ae9060 - the exception-unwinding frame (FS:[0] handler) is not modelled; the ordinary behaviour is complete
/// The scalar deleting destructor of the `BSSoundInfo *` map (vtable
/// `010a3328`). Returns `this`.
pub fn fn_00ae9060(e: &mut Engine, this: u32, flags: u32) -> u32 {
    map_destroy_derived(e, this, STATE_MAP_VTABLE, STATE_MAP_BASE_VTABLE);
    map_delete(e, this, flags);
    this
}

// Translated from 00ae9100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, int,
/// NiPointer<NiAVObject>>::_scalar_deleting_destructor_` (Xbox PDB name):
/// the destructor body (vtable `010a33b0`), then the free when bit 0 of
/// `flags` is set. Returns `this`.
pub fn ni_t_map_base_reference_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    map_destroy(e, this, REFERENCE_MAP_BASE_VTABLE);
    map_delete(e, this, flags);
    this
}

// Translated from 00ae9150 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ae9150 - the exception-unwinding frame (FS:[0] handler) is not modelled; the ordinary behaviour is complete
/// `NiTPointerMap<int, NiPointer<NiAVObject>>::_scalar_deleting_destructor_`
/// (Xbox PDB name): the derived (vtable `010a3308`) and base destruction,
/// then the free when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_pointer_map_reference_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    map_destroy_derived(e, this, REFERENCE_MAP_VTABLE, REFERENCE_MAP_BASE_VTABLE);
    map_delete(e, this, flags);
    this
}
/// The sound `message` is about: the sound list entry (+0x54) for the
/// message's id, when there is one that is not null.
fn message_sound(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) -> Option<u32> {
    let id = e.get(message, BSSoundMessage::iID);
    let (found, sound) = map_get(e, this.at(BSAudioManager::SoundList), id, 0);
    if found && sound != 0 {
        Some(sound)
    } else {
        None
    }
}

/// Whether the sound is ready for messages to act on it: state bits 0x800
/// and 0x100 are both set. Otherwise the messages wait in its own queue.
fn sound_is_ready(e: &Engine, sound: u32) -> bool {
    let state = e.mem.u32(sound + SOUND_STATE_FLAGS);
    state & 0x800 != 0 && state & 0x100 != 0
}

/// Copies `message` (a `FreeQueueMessages` message takes its place) to the
/// end of the sound's own queue (+0x18c), where it waits until the sound is
/// ready (`00adcd80`'s caller replays it).
fn queue_on_sound(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    sound: u32,
    message: Ptr<BSSoundMessage>,
) {
    let copy = fn_00adaf80(e, this);
    fn_00ae7990(e, copy, message);
    list_append_pointer(e, sound + SOUND_QUEUED_MESSAGES, copy.addr());
}

/// Queues the message again for later (the manager's list, kind and the
/// three words as they came; no smart pointer, default vector).
fn requeue_message(e: &mut Engine, this: Ptr<BSAudioManager>, message: Ptr<BSSoundMessage>) {
    let kind = e.get(message, BSSoundMessage::msg);
    let id = e.get(message, BSSoundMessage::iID);
    let data = e.get(message, BSSoundMessage::iData);
    let pointer_data = e.get(message, BSSoundMessage::pData);
    queue_plain(e, this, kind, id, data, pointer_data);
}

/// Sets bits of the sound's state flags.
fn set_state_bits(e: &mut Engine, sound: u32, bits: u32) {
    let state = e.mem.u32(sound + SOUND_STATE_FLAGS);
    e.mem.set_u32(sound + SOUND_STATE_FLAGS, state | bits);
}

/// Clears bits of the sound's state flags.
fn clear_state_bits(e: &mut Engine, sound: u32, bits: u32) {
    let state = e.mem.u32(sound + SOUND_STATE_FLAGS);
    e.mem.set_u32(sound + SOUND_STATE_FLAGS, state & !bits);
}

/// `SetVoiceState(id, sound)` with the sound's own id.
fn set_voice_state_of(e: &mut Engine, this: Ptr<BSAudioManager>, sound: u32) {
    let id = e.mem.u32(sound + SOUND_ID);
    bs_audio_manager_set_voice_state(e, this, id, Ptr::new(sound));
}

/// `SetVolume` (sound slot 0x3c) with the sound's current volume
/// (`fCurrentVolume`, +0x24).
fn apply_current_volume(e: &mut Engine, sound: u32) {
    let volume = x87_quiet(e.mem.f32(sound + SOUND_CURRENT_VOLUME));
    e.vcall(sound, SOUND_SLOT_SET_VOLUME, &args![volume]);
}

/// Message 0x25 (`MADD_SOUND_TO_LIST`): `message.pData` is a new sound. With
/// no load queue (`01202d98`) the message is queued again. Otherwise the
/// sound's hashes are calculated; unless its type has bit 0x10000, a
/// cached sound with its hashes is taken (`00add1e0`). Without one, the sound
/// gets an `AudioLoadTask` (state bit 0x1000, the load queue's slot 0x48)
/// and goes into the sound list. With one, the cached sound takes over: it
/// gets the new sound's type flags, the new sound is deleted when the hashes
/// are the same (or queued for precaching, `00add960`, when only the
/// directory hash is), and the cached sound, with the message's id, goes into
/// the sound list and gets its voice state.
fn message_add_sound_to_list(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    let id = e.get(message, BSSoundMessage::iID);
    let sound = e.get(message, BSSoundMessage::pData);
    if e.mem.u32(LOAD_QUEUE_GLOBAL) == 0 {
        requeue_message(e, this, message);
        return;
    }
    if sound == 0 {
        return;
    }
    let no_hash = !e.call(SOUND_CALCULATE_HASHES, &[sound, 0]).bool();
    let mut cached = 0;
    let type_flags = e.mem.u32(sound + SOUND_TYPE_FLAGS);
    if type_flags & 0x10000 == 0 {
        let directory_hash = e.mem.u32(sound + SOUND_DIRECTORY_HASH);
        let hash = e.mem.u32(sound + SOUND_HASH);
        cached = fn_00add1e0(e, this, hash, directory_hash, type_flags);
    }
    if cached == 0 {
        if e.mem.u8(sound + SOUND_FILENAME) == 0 {
            e.call(DEBUG_PRINT, &[ADD_SOUND_MESSAGE, sound + SOUND_FILENAME]);
        }
        let raw = e.call(ALLOCATE, &[MEMORY_MANAGER, 0x38]).u32();
        let task = if raw != 0 {
            e.call(LOAD_TASK_CONSTRUCTOR, &[raw, sound]).u32()
        } else {
            0
        };
        set_state_bits(e, sound, 0x1000);
        let queue = e.global::<u32>(LOAD_QUEUE_GLOBAL);
        e.vcall(queue, LOAD_QUEUE_SLOT_ADD, &[task]);
        e.call(MAP_SET_VALUE, &[this.addr() + 0x54, id, sound]);
        return;
    }
    let type_flags = e.mem.u32(sound + SOUND_TYPE_FLAGS);
    e.mem.set_u32(cached + SOUND_TYPE_FLAGS, type_flags);
    if !no_hash && e.mem.u32(sound + SOUND_HASH) == e.mem.u32(cached + SOUND_HASH) {
        e.vcall(sound, SOUND_SLOT_DESTRUCTOR, &[1]);
    } else {
        let directory = e.mem.u32(sound + SOUND_DIRECTORY_HASH);
        if directory == e.mem.u32(cached + SOUND_DIRECTORY_HASH)
            && e.mem.u32(cached + SOUND_DIRECTORY_HASH) != 0
        {
            fn_00add960(e, this, sound);
        }
    }
    e.mem.set_u32(cached + SOUND_ID, id);
    e.call(MAP_SET_VALUE, &[this.addr() + 0x54, id, cached]);
    bs_audio_manager_set_voice_state(e, this, id, Ptr::new(cached));
}

/// Message 0x26 (`MDELETE_SOUND`): the sound gets state bit 0x10 and loses
/// 0x200.
fn message_delete_sound(e: &mut Engine, this: Ptr<BSAudioManager>, message: Ptr<BSSoundMessage>) {
    if let Some(sound) = message_sound(e, this, message) {
        set_state_bits(e, sound, 0x10);
        clear_state_bits(e, sound, 0x200);
    }
}

/// Message 0x3a (`MSET_COMPLETION_CALLBACK`): the sound's completion
/// callback is `pData` and its context `iData`.
fn message_set_completion_callback(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    if let Some(sound) = message_sound(e, this, message) {
        let context = e.get(message, BSSoundMessage::iData);
        let callback = e.get(message, BSSoundMessage::pData);
        e.mem.set_u32(sound + SOUND_COMPLETION_CALLBACK, callback);
        e.mem.set_u32(sound + SOUND_COMPLETION_CONTEXT, context);
    }
}

/// Message 0x3b (`MSET_PLAY_CALLBACK`): the sound's played callback
/// (+0x168) is `pData` and its context (+0x170) `iData`.
fn message_set_play_callback(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    if let Some(sound) = message_sound(e, this, message) {
        let context = e.get(message, BSSoundMessage::iData);
        let callback = e.get(message, BSSoundMessage::pData);
        e.mem.set_u32(sound + SOUND_PLAYED_CALLBACK, callback);
        e.mem.set_u32(sound + SOUND_PLAY_CONTEXT, context);
    }
}

/// Message 0x27 (`MREGISTER_MOVING_SOUND`): the object in the message's
/// smart pointer becomes the one the sound follows. An entry already in the
/// moving-reference map for the id is removed (its object released when the
/// map held the only reference); the id is set to the object (`00ae7cc0`),
/// and the message lets go of the object.
fn message_register_moving_sound(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    let id = e.get(message, BSSoundMessage::iID);
    let moving = this.at(BSAudioManager::MovingRefrList);
    let slots = e.mem.alloc(8);
    let object_slot = slots;
    let old_slot = slots + 4;
    e.call(SMART_POINTER_ASSIGN, &[object_slot, message.addr() + 0x10]);
    if fn_00ae7e60(e, moving, id, old_slot) {
        e.call(MAP_REMOVE, &[moving.addr(), id]);
        let old = e.mem.u32(old_slot);
        if old != 0 && e.mem.u32(old + 4) == 1 {
            e.call(RELEASE_MESSAGE_POINTER, &[old]);
        }
        e.call(SMART_POINTER_SET, &[old_slot, 0]);
    }
    let object = e.mem.u32(object_slot);
    add_reference(e, object);
    fn_00ae7cc0(e, moving, id, object);
    e.with_stack(4, |e, empty| {
        e.call(SMART_POINTER_ASSIGN, &[message.addr() + 0x10, empty.addr()]);
        e.call(SMART_POINTER_DESTRUCTOR, &[empty.addr()]);
    });
    let old = e.mem.u32(old_slot);
    drop_reference(e, old);
    let object = e.mem.u32(object_slot);
    drop_reference(e, object);
    e.mem.free(slots);
}

/// Message 0x40 (`MSET_AT_LISTENER`): the sound's type bit 0x40000 is set
/// when `iData` is not 0, else cleared.
fn message_set_at_listener(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    if let Some(sound) = message_sound(e, this, message) {
        let flags = e.mem.u32(sound + SOUND_TYPE_FLAGS);
        if e.get(message, BSSoundMessage::iData) != 0 {
            e.mem.set_u32(sound + SOUND_TYPE_FLAGS, flags | 0x4_0000);
        } else {
            e.mem.set_u32(sound + SOUND_TYPE_FLAGS, flags & !0x4_0000);
        }
    }
}

/// Message 0x3f (`MSEEK`): a ready sound seeks to `iData` (slot 0x6c); any
/// other keeps the message in its own queue.
fn message_seek(e: &mut Engine, this: Ptr<BSAudioManager>, message: Ptr<BSSoundMessage>) {
    if let Some(sound) = message_sound(e, this, message) {
        if sound_is_ready(e, sound) {
            let position = e.get(message, BSSoundMessage::iData);
            e.vcall(sound, SOUND_SLOT_SEEK, &[position]);
        } else {
            queue_on_sound(e, this, sound, message);
        }
    }
}

/// x87 `FISTP` to 64 bits with truncation (the control word rounds toward
/// zero): the integer, or the "integer indefinite" value for NaN and for
/// values that do not fit.
fn fistp_truncate_i64(value: f64) -> i64 {
    if value.is_nan() || value >= i64::MAX as f64 || value < i64::MIN as f64 {
        i64::MIN
    } else {
        value.trunc() as i64
    }
}

/// x87 `FISTP` to 32 bits with truncation: the integer, or the "integer
/// indefinite" value 0x80000000 for NaN and for values that do not fit.
fn fistp_truncate_i32(value: f64) -> i32 {
    if value.is_nan() || value >= 2_147_483_648.0 || value <= -2_147_483_649.0 {
        i32::MIN
    } else {
        value.trunc() as i32
    }
}

/// `BSRandom::UnsignedInt(0xffffffff)` on the global random generator
/// (`011c4180`), which is constructed the first time (the guard bit in
/// `011c4b4c`).
fn bs_random_unsigned_int(e: &mut Engine) -> u32 {
    let guard = e.mem.u32(BSRANDOM_GUARD);
    if guard & 1 == 0 {
        e.mem.set_u32(BSRANDOM_GUARD, guard | 1);
        e.call(BSRANDOM_CONSTRUCTOR, &[BSRANDOM_OBJECT]);
    }
    e.call(BSRANDOM_UNSIGNED_INT, &[BSRANDOM_OBJECT, 0xffff_ffff])
        .u32()
}

/// The frequency variance step of a starting sound: a sound of type bit
/// 0x40 gets its frequency (slot 0x68) moved by a random amount of up to
/// its `iFreqVariance` percent either way (`BSRandom`), through
/// `SetFrequency` (slot 0x64).
fn vary_frequency(e: &mut Engine, sound: u32) {
    let frequency = e.vcall(sound, SOUND_SLOT_GET_FREQUENCY, &[]).f32();
    let variance =
        (e.mem.u8(sound + SOUND_FREQUENCY_VARIANCE) as f64 / e.mem.f64(PERCENT_SCALE)) as f32;
    let random = bs_random_unsigned_int(e);
    let range = (variance as f64 * e.mem.f64(TWO_DOUBLE)) as f32;
    let offset = (range as f64 * random as f64 / e.mem.f64(TWO_POWER_32_DOUBLE)) as f32;
    let varied = ((offset as f64 - variance as f64) + frequency as f64) as f32;
    e.vcall(sound, SOUND_SLOT_SET_FREQUENCY, &args![varied]);
}

/// The part of starting a sound shared by `MPLAY` and `MPLAY_AFTER` once
/// the sound is ready: marks it out of range (state bit 0x10000) when the
/// range test (`00ae9b50`) fails; sets full volume (system attenuation
/// 10000) for a muted type (not type bit 0x800); sets the fader
/// attenuation from `fDialogueFadeDecibels` while the dialogue fade is on
/// and the type is none of bits 0x20, 0x4, 0x800 or 0x100000; varies the
/// frequency (type bit 0x40); runs `00ae8b90` with the listener-underwater
/// flag (modifier bit 1 or type bit 0x1000); sets the volume; plays
/// (`MPLAY_AFTER` only when the sound is not paused); then pauses it, stops
/// it (and gives it state bit 0x10) or runs `00ae8b90(1)` as the pause table
/// (`00ae6d30`) and the type bits 0x400000, 0x2000, 0x1000, 0x10, 0x4000000
/// and 0x2000000 say. Finally state bit 0x2000 is set and the voice state
/// updated.
fn start_ready_sound(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    sound: u32,
    only_if_not_paused: bool,
) {
    let hour = x87_quiet(e.get(this, BSAudioManager::fCurrentGamehour));
    if !e.call(SOUND_RANGE_TEST, &args![sound, 0u32, hour]).bool() {
        set_state_bits(e, sound, 0x10000);
    }
    let kind = |e: &Engine| e.mem.u32(sound + SOUND_TYPE_FLAGS);
    if kind(e) & e.get(this, BSAudioManager::iMuteMask) != 0 && kind(e) & 0x800 == 0 {
        e.mem.set_u16(sound + SOUND_SYSTEM_ATTENUATION, 10000);
        apply_current_volume(e, sound);
    }
    if e.get(this, BSAudioManager::bDialogueFade)
        && kind(e) & 0x20 == 0
        && kind(e) & 4 == 0
        && kind(e) & 0x800 == 0
        && kind(e) & 0x10_0000 == 0
    {
        let decibels = e.global::<f32>(DIALOGUE_FADE_DECIBELS);
        let scaled = fistp_truncate_i32(decibels as f64 * e.mem.f64(PERCENT_SCALE)) as u16;
        e.mem.set_u16(sound + SOUND_FADER_ATTENUATION, scaled);
        apply_current_volume(e, sound);
    }
    if kind(e) & 0x40 != 0 {
        vary_frequency(e, sound);
    }
    if e.mem.u32(sound + SOUND_MODIFIER_FLAGS) & 1 != 0 || kind(e) & 0x1000 != 0 {
        let underwater = e.get(this, BSAudioManager::bListenerUnderwater) as u32;
        e.call(SOUND_ENVIRONMENT_STEP, &[sound, underwater]);
    }
    apply_current_volume(e, sound);
    if only_if_not_paused {
        if !e.vcall(sound, SOUND_SLOT_IS_PAUSED, &[]).bool() {
            e.vcall(sound, SOUND_SLOT_PLAY, &[0]);
        }
    } else {
        e.vcall(sound, SOUND_SLOT_PLAY, &[0]);
    }
    let paused_by_table = fn_00ae6d30(e, this, kind(e));
    if paused_by_table && kind(e) & 0x40_0000 == 0 && kind(e) & 0x2000 == 0 && kind(e) & 0x1000 == 0
    {
        e.vcall(sound, SOUND_SLOT_PAUSE, &[]);
    } else if paused_by_table && kind(e) & 0x40_0000 != 0 {
        e.call(SOUND_ENVIRONMENT_STEP, &[sound, 1]);
    } else if paused_by_table {
        let attenuated =
            kind(e) & 0x10 != 0 || kind(e) & 0x400_0000 != 0 || kind(e) & 0x200_0000 != 0;
        if !attenuated && (kind(e) & 0x2000 != 0 || kind(e) & 0x1000 != 0) {
            e.vcall(sound, SOUND_SLOT_STOP, &[]);
            set_state_bits(e, sound, 0x10);
        }
    }
    set_state_bits(e, sound, 0x2000);
    set_voice_state_of(e, this, sound);
}

/// Message 1 (`MPLAY`): a sound that is not ready is queued again through
/// the manager when it does not have state bit 1, else kept in its own
/// queue; a ready one starts (`start_ready_sound`).
fn message_play(e: &mut Engine, this: Ptr<BSAudioManager>, message: Ptr<BSSoundMessage>) {
    if let Some(sound) = message_sound(e, this, message) {
        if !sound_is_ready(e, sound) {
            if e.mem.u32(sound + SOUND_STATE_FLAGS) & 1 == 0 {
                requeue_message(e, this, message);
            } else {
                queue_on_sound(e, this, sound, message);
            }
            return;
        }
        start_ready_sound(e, this, sound, false);
    }
}

/// Message 2 (`MPLAYLOOPING`): a ready sound plays with its argument 1 and
/// gets its voice state; any other keeps the message in its own queue.
fn message_play_looping(e: &mut Engine, this: Ptr<BSAudioManager>, message: Ptr<BSSoundMessage>) {
    if let Some(sound) = message_sound(e, this, message) {
        if sound_is_ready(e, sound) {
            e.vcall(sound, SOUND_SLOT_PLAY, &[1]);
            set_voice_state_of(e, this, sound);
        } else {
            queue_on_sound(e, this, sound, message);
        }
    }
}

/// Message 3 (`MPAUSE`): a ready sound is paused (slot 0x34) and gets its
/// voice state; any other keeps the message in its own queue.
fn message_pause(e: &mut Engine, this: Ptr<BSAudioManager>, message: Ptr<BSSoundMessage>) {
    if let Some(sound) = message_sound(e, this, message) {
        if sound_is_ready(e, sound) {
            e.vcall(sound, SOUND_SLOT_PAUSE, &[]);
            set_voice_state_of(e, this, sound);
        } else {
            queue_on_sound(e, this, sound, message);
        }
    }
}

/// Message 4 (`MSTOP`): a ready sound is stopped (slot 0x38) and gets its
/// voice state; any other keeps the message in its own queue.
fn message_stop(e: &mut Engine, this: Ptr<BSAudioManager>, message: Ptr<BSSoundMessage>) {
    if let Some(sound) = message_sound(e, this, message) {
        if sound_is_ready(e, sound) {
            e.vcall(sound, SOUND_SLOT_STOP, &[]);
            set_voice_state_of(e, this, sound);
        } else {
            queue_on_sound(e, this, sound, message);
        }
    }
}

/// Message 5 (`MPLAY_AFTER`): before the time in `iData` the message is
/// queued again as it is. Then a sound that is not ready is queued as an
/// `MPLAY` message (kind 1, same words); a ready one starts like `MPLAY`,
/// but plays only if it is not paused.
fn message_play_after(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
    now: u32,
) {
    if e.get(message, BSSoundMessage::iData) > now {
        requeue_message(e, this, message);
        return;
    }
    if let Some(sound) = message_sound(e, this, message) {
        if !sound_is_ready(e, sound) {
            let id = e.get(message, BSSoundMessage::iID);
            let data = e.get(message, BSSoundMessage::iData);
            let pointer_data = e.get(message, BSSoundMessage::pData);
            queue_plain(e, this, 1, id, data, pointer_data);
            return;
        }
        start_ready_sound(e, this, sound, true);
    }
}

/// The message to send when a fade is over: `message.vector_x`, read as an
/// integer (x87 truncation, low 32 bits), is the kind; a positive one is
/// queued for the sound with no data and the default vector.
fn send_fade_followup(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
    sound: u32,
) {
    let value = e.get(message, BSSoundMessage::vector3_x);
    let kind = fistp_truncate_i64(value as f64) as u32 as i32;
    if kind > 0 {
        let id = e.mem.u32(sound + SOUND_ID);
        queue_plain(e, this, kind as u32, id, 0, 0);
    }
}

/// Queues the fade message `kind` again for the sound with the message's
/// times (`iData`, `pData`) and its vector.
fn requeue_fade(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
    kind: u32,
    sound: u32,
) {
    let id = e.mem.u32(sound + SOUND_ID);
    let start = e.get(message, BSSoundMessage::iData);
    let end = e.get(message, BSSoundMessage::pData);
    let x = e.get(message, BSSoundMessage::vector3_x);
    let y = e.get(message, BSSoundMessage::vector3_y);
    let z = e.get(message, BSSoundMessage::vector3_z);
    fn_00adb1a0(e, this, kind, id, start, end, 0, x, y, z);
}

/// Message 9 (`MFADE_OUT`): `iData` is the start time (0 means now, and
/// `pData`, a duration then, becomes the end time) and `pData` the end time.
/// A ready sound at or past the end gets full system attenuation (10000)
/// and loses state bit 0x200, and the message in `vector_x` is sent
/// (`send_fade_followup`); one before it gets state bit 0x200 and a system
/// attenuation of `(1 - remaining / duration) * 3500`, and the message is
/// queued again. The volume is applied and the voice state set. A sound that
/// is not ready only takes the end-of-fade steps (full attenuation, no
/// 0x200, follow-up message) at the end time, else the message is queued
/// again.
fn message_fade_out(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
    now: u32,
) {
    let Some(sound) = message_sound(e, this, message) else {
        return;
    };
    if sound_is_ready(e, sound) {
        if e.get(message, BSSoundMessage::iData) == 0 {
            e.set(message, BSSoundMessage::iData, now);
            let duration = e.get(message, BSSoundMessage::pData);
            e.set(message, BSSoundMessage::pData, duration.wrapping_add(now));
        }
        let start = e.get(message, BSSoundMessage::iData);
        let end = e.get(message, BSSoundMessage::pData);
        if now >= end {
            e.mem.set_u16(sound + SOUND_SYSTEM_ATTENUATION, 10000);
            apply_current_volume(e, sound);
            clear_state_bits(e, sound, 0x200);
            send_fade_followup(e, this, message, sound);
        } else {
            set_state_bits(e, sound, 0x200);
            let elapsed = end.wrapping_sub(start) as f64;
            let remaining = end.wrapping_sub(now) as f64;
            let scaled = (1.0 - remaining / elapsed) * e.mem.f64(FADE_ATTENUATION_SCALE);
            e.mem.set_u16(
                sound + SOUND_SYSTEM_ATTENUATION,
                fistp_truncate_i32(scaled) as u16,
            );
            apply_current_volume(e, sound);
            requeue_fade(e, this, message, 9, sound);
        }
        apply_current_volume(e, sound);
        set_voice_state_of(e, this, sound);
    } else if now >= e.get(message, BSSoundMessage::pData) {
        e.mem.set_u16(sound + SOUND_SYSTEM_ATTENUATION, 10000);
        apply_current_volume(e, sound);
        clear_state_bits(e, sound, 0x200);
        send_fade_followup(e, this, message, sound);
    } else {
        requeue_fade(e, this, message, 9, sound);
    }
}

/// Calls `body(key, sound)` for every entry of the sound list (+0x54), the
/// way the game walks the map: the first item of the first non-empty bucket,
/// then `GetNext` moving on before the body runs.
fn for_each_sound(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    mut body: impl FnMut(&mut Engine, u32, u32),
) {
    let sounds = this.at(BSAudioManager::SoundList);
    let mut position = map_first_item(e, sounds);
    while position != 0 {
        let (key, sound) = map_next(e, sounds, &mut position);
        body(e, key, sound);
    }
}

/// Queues the fade message `kind` for the sound with the message's own id
/// (`iID`), its times and its vector (the "not ready" and "keep fading"
/// cases of `MFADE_FROM` and `MFADE_TO`).
fn requeue_fade_by_message_id(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
    kind: u32,
    id: u32,
) {
    let start = e.get(message, BSSoundMessage::iData);
    let end = e.get(message, BSSoundMessage::pData);
    let x = e.get(message, BSSoundMessage::vector3_x);
    let y = e.get(message, BSSoundMessage::vector3_y);
    let z = e.get(message, BSSoundMessage::vector3_z);
    fn_00adb1a0(e, this, kind, id, start, end, 0, x, y, z);
}

/// The low 32 bits of `message.vector_x` times 100, truncated (the target
/// level `MFADE_TO` and `MFADE_FROM` carry in the vector's first float).
fn fade_target_level(e: &Engine, message: Ptr<BSSoundMessage>) -> u32 {
    let value = e.get(message, BSSoundMessage::vector3_x);
    fistp_truncate_i64(value as f64 * e.mem.f64(PERCENT_SCALE)) as u32
}

/// Message 10 (`MFADE_TO`): the sound's fader attenuation (+0x20) moves
/// from 0 to the target level (`vector_x * 100`) between `iData` and `pData`.
/// A ready sound at or past the end gets the target, one before it
/// `target * (1 - remaining / duration)` and the message queued again; the
/// volume is applied and the voice state set. A sound that is not ready
/// (and not marked 0x10) has the message queued again.
fn message_fade_to(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
    now: u32,
) {
    let Some(sound) = message_sound(e, this, message) else {
        return;
    };
    let id = e.mem.u32(sound + SOUND_ID);
    if sound_is_ready(e, sound) {
        let target = fade_target_level(e, message);
        let start = e.get(message, BSSoundMessage::iData);
        let end = e.get(message, BSSoundMessage::pData);
        if now >= end {
            e.mem
                .set_u16(sound + SOUND_FADER_ATTENUATION, target as u16);
            apply_current_volume(e, sound);
        } else {
            let elapsed = end.wrapping_sub(start) as f64;
            let remaining = end.wrapping_sub(now) as f64;
            let level = target as f64 * (1.0 - remaining / elapsed);
            e.mem.set_u16(
                sound + SOUND_FADER_ATTENUATION,
                fistp_truncate_i32(level) as u16,
            );
            apply_current_volume(e, sound);
            requeue_fade_by_message_id(e, this, message, 10, id);
        }
        apply_current_volume(e, sound);
        set_voice_state_of(e, this, sound);
    } else if e.mem.u32(sound + SOUND_STATE_FLAGS) & 0x10 == 0 {
        requeue_fade_by_message_id(e, this, message, 10, id);
    }
}

/// Message 12 (`MFADE_FROM`): the fader attenuation moves from the target
/// level (`vector_x * 100`) to 0: a ready sound at or past the end gets 0,
/// one before it `target * (remaining / duration)` and the message queued
/// again; then the voice state is set. A sound that is not ready (and not
/// marked 0x10) has the message queued again.
fn message_fade_from(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
    now: u32,
) {
    let Some(sound) = message_sound(e, this, message) else {
        return;
    };
    let id = e.get(message, BSSoundMessage::iID);
    if sound_is_ready(e, sound) {
        let target = fade_target_level(e, message);
        let start = e.get(message, BSSoundMessage::iData);
        let end = e.get(message, BSSoundMessage::pData);
        if now >= end {
            e.mem.set_u16(sound + SOUND_FADER_ATTENUATION, 0);
            apply_current_volume(e, sound);
        } else {
            let elapsed = end.wrapping_sub(start) as f64;
            let remaining = end.wrapping_sub(now) as f64;
            let level = target as f64 * (remaining / elapsed);
            e.mem.set_u16(
                sound + SOUND_FADER_ATTENUATION,
                fistp_truncate_i32(level) as u16,
            );
            apply_current_volume(e, sound);
            requeue_fade_by_message_id(e, this, message, 12, id);
        }
        set_voice_state_of(e, this, sound);
    } else if e.mem.u32(sound + SOUND_STATE_FLAGS) & 0x10 == 0 {
        requeue_fade_by_message_id(e, this, message, 12, id);
    }
}

/// Message 11 (`MFADE_IN`): the system attenuation falls from full to 0.
/// `iData` is the start time (0 means now, and `pData`, a duration then,
/// becomes the end time). A ready sound at or past the end gets 0 and loses
/// state bit 0x200; one before it gets bit 0x200 and
/// `remaining / duration * 3500` and the message queued again (default
/// vector); then the volume is applied and the voice state set. A sound that
/// is not ready: with state bit 0x800 and at or past the end, system
/// attenuation 0, no 0x200 and message 8 (`MUNMUTE`) queued; else the
/// message is queued again.
fn message_fade_in(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
    now: u32,
) {
    let Some(sound) = message_sound(e, this, message) else {
        return;
    };
    let id = e.get(message, BSSoundMessage::iID);
    if sound_is_ready(e, sound) {
        if e.get(message, BSSoundMessage::iData) == 0 {
            e.set(message, BSSoundMessage::iData, now);
            let duration = e.get(message, BSSoundMessage::pData);
            e.set(message, BSSoundMessage::pData, duration.wrapping_add(now));
        }
        let start = e.get(message, BSSoundMessage::iData);
        let end = e.get(message, BSSoundMessage::pData);
        if now >= end {
            e.mem.set_u16(sound + SOUND_SYSTEM_ATTENUATION, 0);
            apply_current_volume(e, sound);
            clear_state_bits(e, sound, 0x200);
        } else {
            set_state_bits(e, sound, 0x200);
            let elapsed = end.wrapping_sub(start) as f64;
            let remaining = end.wrapping_sub(now) as f64;
            let level = remaining / elapsed * e.mem.f64(FADE_ATTENUATION_SCALE);
            e.mem.set_u16(
                sound + SOUND_SYSTEM_ATTENUATION,
                fistp_truncate_i32(level) as u16,
            );
            apply_current_volume(e, sound);
            let start = e.get(message, BSSoundMessage::iData);
            let end = e.get(message, BSSoundMessage::pData);
            queue_plain(e, this, 11, id, start, end);
        }
        apply_current_volume(e, sound);
        set_voice_state_of(e, this, sound);
    } else {
        let start = e.get(message, BSSoundMessage::iData);
        let end = e.get(message, BSSoundMessage::pData);
        let state = e.mem.u32(sound + SOUND_STATE_FLAGS);
        if state & 0x800 != 0 && now >= end {
            e.mem.set_u16(sound + SOUND_SYSTEM_ATTENUATION, 0);
            apply_current_volume(e, sound);
            clear_state_bits(e, sound, 0x200);
            queue_plain(e, this, 8, id, start, end);
        } else {
            queue_plain(e, this, 11, id, start, end);
        }
    }
}

/// Message 13 (`MDIALOGUE_FADE`): `iID` switches the dialogue fade
/// (`bDialogueFade`). The fade time is `fDialogueFadeSecondsOut` (switching
/// on) or `...SecondsIn` (off) in milliseconds. Every sound whose type has
/// none of the bits 0x20, 0x4, 0x800 and 0x100000 gets a fade message of
/// that length: `MFADE_TO` (switching on) or `MFADE_FROM`, whose vector's
/// first float is `fDialogueFadeDecibels`.
fn message_dialogue_fade(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
    now: u32,
) {
    let on = e.get(message, BSSoundMessage::iID) != 0;
    e.set(this, BSAudioManager::bDialogueFade, on);
    let seconds = if on {
        e.global::<f32>(DIALOGUE_FADE_SECONDS_OUT)
    } else {
        e.global::<f32>(DIALOGUE_FADE_SECONDS_IN)
    };
    let milliseconds = fistp_truncate_i64(seconds as f64 * e.mem.f64(MILLISECOND_SCALE)) as u32;
    for_each_sound(e, this, |e, _key, sound| {
        let kind = e.mem.u32(sound + SOUND_TYPE_FLAGS);
        if kind & 0x20 == 0 && kind & 4 == 0 && kind & 0x800 == 0 && kind & 0x10_0000 == 0 {
            let decibels = e.global::<f32>(DIALOGUE_FADE_DECIBELS);
            let id = e.mem.u32(sound + SOUND_ID);
            let message_kind = if e.get(this, BSAudioManager::bDialogueFade) {
                10
            } else {
                12
            };
            fn_00adb1a0(
                e,
                this,
                message_kind,
                id,
                now,
                now.wrapping_add(milliseconds),
                0,
                decibels,
                0.0,
                0.0,
            );
        }
    });
}

/// Message 14 (`MVOLUME_LEVELS_CHANGED`): every sound applies its current
/// volume again.
fn message_volume_levels_changed(e: &mut Engine, this: Ptr<BSAudioManager>) {
    for_each_sound(e, this, |e, _key, sound| {
        if sound != 0 {
            apply_current_volume(e, sound);
        }
    });
}

/// Message 16 (`MSET_VOLUME`): a ready sound takes `iData / 1000` as its
/// volume (slot 0x3c) and gets its voice state; a sound that is not ready
/// (and not marked 0x10) has the message queued again.
fn message_set_volume(e: &mut Engine, this: Ptr<BSAudioManager>, message: Ptr<BSSoundMessage>) {
    if let Some(sound) = message_sound(e, this, message) {
        if sound_is_ready(e, sound) {
            let milli = e.get(message, BSSoundMessage::iData) as i32;
            let volume = (milli as f64 / e.mem.f64(MILLISECOND_SCALE)) as f32;
            e.vcall(sound, SOUND_SLOT_SET_VOLUME, &args![volume]);
            set_voice_state_of(e, this, sound);
        } else if e.mem.u32(sound + SOUND_STATE_FLAGS) & 0x10 == 0 {
            let id = e.get(message, BSSoundMessage::iID);
            let data = e.get(message, BSSoundMessage::iData);
            queue_plain(e, this, 0x10, id, data, 0);
        }
    }
}

/// Message 19 (`MSET_FREQUENCY`): the sound's frequency (slot 0x64) becomes
/// `iData / 1000`.
fn message_set_frequency(e: &mut Engine, this: Ptr<BSAudioManager>, message: Ptr<BSSoundMessage>) {
    if let Some(sound) = message_sound(e, this, message) {
        let milli = e.get(message, BSSoundMessage::iData) as i32;
        let frequency = (milli as f64 / e.mem.f64(MILLISECOND_SCALE)) as f32;
        e.vcall(sound, SOUND_SLOT_SET_FREQUENCY, &args![frequency]);
    }
}

/// Message 66 (`MSET_MOD_FREQUENCY`): the sound's modified sample rate
/// (+0x15a) becomes its sample rate (+0x34) times `iData / 1000`.
fn message_set_mod_frequency(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    if let Some(sound) = message_sound(e, this, message) {
        let samples_per_second = e.mem.u16(sound + SOUND_SAMPLES_PER_SECOND) as f64;
        let milli = e.get(message, BSSoundMessage::iData) as i32;
        let ratio = (milli as f64 / e.mem.f64(MILLISECOND_SCALE)) as f32;
        let modified = fistp_truncate_i32(samples_per_second * ratio as f64) as u16;
        e.mem
            .set_u16(sound + SOUND_MOD_SAMPLES_PER_SECOND, modified);
    }
}

/// Message 20 (`MSET_FREQUENCY_VARIANCE`): the sound gets type bit 0x40 and
/// the variance byte (+0x158) from `iData`.
fn message_set_frequency_variance(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    if let Some(sound) = message_sound(e, this, message) {
        let kind = e.mem.u32(sound + SOUND_TYPE_FLAGS);
        e.mem.set_u32(sound + SOUND_TYPE_FLAGS, kind | 0x40);
        let variance = e.get(message, BSSoundMessage::iData) as u8;
        e.mem.set_u8(sound + SOUND_FREQUENCY_VARIANCE, variance);
    }
}

/// Appends `message` to the sound's queue the way the inlined list code of
/// `MMUTE` and `MUNMUTE` does (a node from the list's allocator, the
/// element, `AddNodeTail`) instead of through `00ae8bf0`.
fn queue_on_sound_inline(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    sound: u32,
    message: Ptr<BSSoundMessage>,
) {
    let copy = fn_00adaf80(e, this);
    fn_00ae7990(e, copy, message);
    let list = sound + SOUND_QUEUED_MESSAGES;
    let node = e.call(LIST_NEW_NODE, &[list + 8]).u32();
    e.mem.set_u32(node + 8, copy.addr());
    e.call(LIST_ADD_NODE_TAIL, &[list, node]);
}

/// Message 7 (`MMUTE`, `iID` is the sound) and message 8 (`MUNMUTE`): a
/// ready sound gets full (10000) or no (0) system attenuation and applies its
/// volume; any other keeps the message in its own queue.
fn message_mute(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
    attenuation: u16,
) {
    if let Some(sound) = message_sound(e, this, message) {
        if sound_is_ready(e, sound) {
            e.mem.set_u16(sound + SOUND_SYSTEM_ATTENUATION, attenuation);
            apply_current_volume(e, sound);
        } else {
            queue_on_sound_inline(e, this, sound, message);
        }
    }
}

/// The step several type-wide messages run for a sound: `00ae8b90` with a
/// flag byte.
fn sound_environment_step(e: &mut Engine, sound: u32, flag: u32) {
    e.call(SOUND_ENVIRONMENT_STEP, &[sound, flag]);
}

/// Message 24 (`MMUTE_ALL_OFTYPE`): `iData` (a type mask) joins the mute mask
/// (+0x170). Each sound whose type has a bit of the mask but not 0x800 or
/// 0x100000 is faded out over `pData` milliseconds (message 9 from now) when
/// `pData` is not 0 and `iID` is over 5000; with `pData` 0 it is muted at
/// once (`00ae8b90(1)`, volume applied).
fn message_mute_all_of_type(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
    now: u32,
) {
    let gate = e.get(message, BSSoundMessage::iID);
    let mask = e.get(message, BSSoundMessage::iData);
    let duration = e.get(message, BSSoundMessage::pData);
    let muted = e.get(this, BSAudioManager::iMuteMask);
    e.set(this, BSAudioManager::iMuteMask, muted | mask);
    for_each_sound(e, this, |e, _key, sound| {
        let kind = e.mem.u32(sound + SOUND_TYPE_FLAGS);
        if kind & mask != 0 && kind & 0x800 == 0 && kind & 0x10_0000 == 0 {
            if duration != 0 && gate > 0x1388 {
                let id = e.mem.u32(sound + SOUND_ID);
                queue_plain(e, this, 9, id, now, now.wrapping_add(duration));
            } else if duration == 0 {
                sound_environment_step(e, sound, 1);
                apply_current_volume(e, sound);
            }
        }
    });
}

/// Message 25 (`MUNMUTE_ALL_OFTYPE`): `iData` (a type mask) leaves the mute
/// mask. Each sound whose type has a bit of the mask but not 0x800 or
/// 0x100000 is faded in (message 11 from now, `pData` milliseconds) when
/// `pData` is not 0, else unmuted at once (`00ae8b90(0)`, volume applied).
fn message_unmute_all_of_type(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
    now: u32,
) {
    let mask = e.get(message, BSSoundMessage::iData);
    let duration = e.get(message, BSSoundMessage::pData);
    let muted = e.get(this, BSAudioManager::iMuteMask);
    e.set(this, BSAudioManager::iMuteMask, !mask & muted);
    for_each_sound(e, this, |e, _key, sound| {
        let kind = e.mem.u32(sound + SOUND_TYPE_FLAGS);
        if kind & mask != 0 && kind & 0x800 == 0 && kind & 0x10_0000 == 0 {
            if duration != 0 {
                let id = e.mem.u32(sound + SOUND_ID);
                queue_plain(e, this, 11, id, now, now.wrapping_add(duration));
            } else {
                sound_environment_step(e, sound, 0);
                apply_current_volume(e, sound);
            }
        }
    });
}

/// Message 26 (`MPAUSE_ALL_OFTYPE`): the type mask in `iData` goes into the
/// pause table (`00ae6de0`, not exact). Each sound that is playing, has a
/// bit of the mask and none of 0x800, 0x100000 or 0x400000 is paused; a
/// sound with a bit of the mask and type bit 0x400000 runs `00ae8b90(1)`.
fn message_pause_all_of_type(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    let mask = e.get(message, BSSoundMessage::iData);
    fn_00ae6de0(e, this, mask, 0);
    for_each_sound(e, this, |e, _key, sound| {
        let kind = e.mem.u32(sound + SOUND_TYPE_FLAGS);
        if !e.vcall(sound, SOUND_SLOT_IS_PLAYING, &[]).bool()
            || kind & mask == 0
            || kind & 0x800 != 0
            || kind & 0x10_0000 != 0
            || kind & 0x40_0000 != 0
        {
            if kind & mask != 0 && kind & 0x40_0000 != 0 {
                sound_environment_step(e, sound, 1);
            }
        } else {
            e.vcall(sound, SOUND_SLOT_PAUSE, &[]);
        }
    });
}

/// Message 27 (`MUNPAUSE_ALL_OFTYPE`): each sound that is not marked 0x10
/// and is paused, has a bit of the mask in `iData` and none of 0x800,
/// 0x100000 or 0x400000 plays again (slot 0x30, argument 0); a sound with a
/// bit of the mask and type bit 0x400000 runs `00ae8b90(0)`. Afterwards the
/// mask leaves the pause table (`00ae6e60`, not exact).
fn message_unpause_all_of_type(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    let mask = e.get(message, BSSoundMessage::iData);
    for_each_sound(e, this, |e, _key, sound| {
        let kind = e.mem.u32(sound + SOUND_TYPE_FLAGS);
        let state = e.mem.u32(sound + SOUND_STATE_FLAGS);
        if state & 0x10 == 0
            && e.vcall(sound, SOUND_SLOT_IS_PAUSED, &[]).bool()
            && kind & mask != 0
            && kind & 0x800 == 0
            && kind & 0x10_0000 == 0
            && kind & 0x40_0000 == 0
        {
            e.vcall(sound, SOUND_SLOT_PLAY, &[0]);
        } else if kind & mask != 0 && kind & 0x40_0000 != 0 {
            sound_environment_step(e, sound, 0);
        }
    });
    fn_00ae6e60(e, this, mask, 0);
}

/// Message 28 (`MPAUSE_ALL_OFTYPE_EXACT`): the mask in `iData` goes into the
/// pause table as exact (`00ae6de0`). A sound that is not in decay, is
/// playing, has all the bits of the mask and none of 0x800, 0x100000,
/// 0x400000, 0x2000 or 0x1000 and is not marked 0x10 is paused; one with all
/// the bits and type bit 0x400000, 0x1000 or 0x2000 runs `00ae8b90(1)`.
fn message_pause_all_of_type_exact(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    let mask = e.get(message, BSSoundMessage::iData);
    fn_00ae6de0(e, this, mask, 1);
    for_each_sound(e, this, |e, _key, sound| {
        let kind = e.mem.u32(sound + SOUND_TYPE_FLAGS);
        let state = e.mem.u32(sound + SOUND_STATE_FLAGS);
        if !e.vcall(sound, SOUND_SLOT_IS_IN_DECAY, &[]).bool()
            && e.vcall(sound, SOUND_SLOT_IS_PLAYING, &[]).bool()
            && kind & mask == mask
            && kind & 0x800 == 0
            && kind & 0x10_0000 == 0
            && kind & 0x40_0000 == 0
            && kind & 0x2000 == 0
            && kind & 0x1000 == 0
            && state & 0x10 == 0
        {
            e.vcall(sound, SOUND_SLOT_PAUSE, &[]);
        } else if kind & mask == mask
            && (kind & 0x40_0000 != 0 || kind & 0x1000 != 0 || kind & 0x2000 != 0)
        {
            sound_environment_step(e, sound, 1);
        }
    });
}

/// Message 29 (`MUNPAUSE_ALL_OFTYPE_EXACT`): each sound not marked 0x10 that
/// is paused, has all the bits of the mask in `iData` and none of 0x800,
/// 0x100000, 0x400000, 0x2000 or 0x1000 plays again; one with all the bits
/// and type bit 0x400000, 0x1000 or 0x2000 runs `00ae8b90(0)`. Afterwards the
/// mask leaves the pause table as exact (`00ae6e60`).
fn message_unpause_all_of_type_exact(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    let mask = e.get(message, BSSoundMessage::iData);
    for_each_sound(e, this, |e, _key, sound| {
        let kind = e.mem.u32(sound + SOUND_TYPE_FLAGS);
        let state = e.mem.u32(sound + SOUND_STATE_FLAGS);
        if state & 0x10 == 0
            && e.vcall(sound, SOUND_SLOT_IS_PAUSED, &[]).bool()
            && kind & mask == mask
            && kind & 0x800 == 0
            && kind & 0x10_0000 == 0
            && kind & 0x40_0000 == 0
            && kind & 0x2000 == 0
            && kind & 0x1000 == 0
        {
            e.vcall(sound, SOUND_SLOT_PLAY, &[0]);
        } else if kind & mask == mask
            && (kind & 0x40_0000 != 0 || kind & 0x1000 != 0 || kind & 0x2000 != 0)
        {
            sound_environment_step(e, sound, 0);
        }
    });
    fn_00ae6e60(e, this, mask, 1);
}

/// Message 30 (`MSET_POS`): a ready sound moves its emitter to the message's
/// vector (slot 0x4c). Any other keeps only the latest position: a position
/// message already waiting in its queue (kind 0x1e) is overwritten with this
/// one, else this one is appended (the inlined list append).
fn message_set_position(e: &mut Engine, this: Ptr<BSAudioManager>, message: Ptr<BSSoundMessage>) {
    let Some(sound) = message_sound(e, this, message) else {
        return;
    };
    let x = e.get(message, BSSoundMessage::vector3_x);
    let y = e.get(message, BSSoundMessage::vector3_y);
    let z = e.get(message, BSSoundMessage::vector3_z);
    if sound_is_ready(e, sound) {
        e.vcall(sound, SOUND_SLOT_SET_EMITTER_POSITION, &args![x, y, z]);
        return;
    }
    let mut replaced = false;
    let mut node = e.mem.u32(sound + SOUND_QUEUED_MESSAGES);
    while node != 0 && !replaced {
        let waiting = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if waiting != 0 && e.mem.u32(waiting) == 0x1e {
            fn_00ae7990(e, Ptr::new(waiting), message);
            replaced = true;
        }
    }
    if !replaced {
        queue_on_sound_inline(e, this, sound, message);
    }
}

/// Message 31 (`MSET_BEAM_END`): a ready sound stores the vector as its beam
/// end (+0x17c) and gets state bit 0x8000; a sound that is not ready (and
/// not marked 0x10) has the message, with the object in its smart pointer,
/// queued again, and the original message lets go of the object.
fn message_set_beam_end(e: &mut Engine, this: Ptr<BSAudioManager>, message: Ptr<BSSoundMessage>) {
    let Some(sound) = message_sound(e, this, message) else {
        return;
    };
    let x = e.get(message, BSSoundMessage::vector3_x);
    let y = e.get(message, BSSoundMessage::vector3_y);
    let z = e.get(message, BSSoundMessage::vector3_z);
    if sound_is_ready(e, sound) {
        e.mem.set_f32(sound + SOUND_BEAM_END, x);
        e.mem.set_f32(sound + SOUND_BEAM_END + 4, y);
        e.mem.set_f32(sound + SOUND_BEAM_END + 8, z);
        set_state_bits(e, sound, 0x8000);
    } else if e.mem.u32(sound + SOUND_STATE_FLAGS) & 0x10 == 0 {
        let slot = e.mem.alloc(4);
        e.call(SMART_POINTER_ASSIGN, &[slot, message.addr() + 0x10]);
        let object = e.mem.u32(slot);
        add_reference(e, object);
        let id = e.get(message, BSSoundMessage::iID);
        let data = e.get(message, BSSoundMessage::iData);
        let pointer_data = e.get(message, BSSoundMessage::pData);
        fn_00adb1a0(e, this, 0x1f, id, data, pointer_data, object, x, y, z);
        e.with_stack(4, |e, empty| {
            e.call(SMART_POINTER_ASSIGN, &[message.addr() + 0x10, empty.addr()]);
        });
        let held = e.mem.u32(slot);
        drop_reference(e, held);
        e.mem.free(slot);
    }
}

/// Message 33 (`MSET_STATIC_ATTENUATION`): the sound's static attenuation
/// (+0x18) takes the low 16 bits of `iData` and the volume is applied.
fn message_set_static_attenuation(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    if let Some(sound) = message_sound(e, this, message) {
        let value = e.get(message, BSSoundMessage::iData) as u16;
        e.mem.set_u16(sound + SOUND_STATIC_ATTENUATION, value);
        apply_current_volume(e, sound);
    }
}

/// Message 34 (`MSET_REVERB_ATTENUATION`): the sound's reverb attenuation
/// (+0x1a) takes the low 16 bits of `iData` and the volume is applied.
fn message_set_reverb_attenuation(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    if let Some(sound) = message_sound(e, this, message) {
        let value = e.get(message, BSSoundMessage::iData) as u16;
        e.mem.set_u16(sound + SOUND_REVERB_ATTENUATION, value);
        apply_current_volume(e, sound);
    }
}

/// Message 35 (`MSET_MINMAX_DISTANCE`): a ready sound gets the minimum
/// (`iData` / 1000, signed) and maximum (`pData` / 1000, unsigned) distances
/// (slot 0x5c); a sound that is not ready (and not marked 0x10) has the
/// message queued again.
fn message_set_min_max_distance(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    let Some(sound) = message_sound(e, this, message) else {
        return;
    };
    let scale = e.mem.f64(MILLISECOND_SCALE);
    let maximum = (e.get(message, BSSoundMessage::pData) as f64 / scale) as f32;
    if sound_is_ready(e, sound) {
        let minimum = (e.get(message, BSSoundMessage::iData) as i32 as f64 / scale) as f32;
        e.vcall(sound, SOUND_SLOT_SET_MIN_MAX, &args![minimum, maximum]);
    } else if e.mem.u32(sound + SOUND_STATE_FLAGS) & 0x10 == 0 {
        requeue_message(e, this, message);
    }
}

/// Message 36 (`MSET_ATTENUATION_CURVE`): the five 16-bit curve values
/// (the message's first ten bytes of vector): a ready sound gets them
/// (slot 0x60), any other has them queued (`00adbb30`, which limits each to
/// 100).
fn message_set_attenuation_curve(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    let Some(sound) = message_sound(e, this, message) else {
        return;
    };
    let curve = [
        e.get(message, BSSoundMessage::curve_0),
        e.get(message, BSSoundMessage::curve_1),
        e.get(message, BSSoundMessage::curve_2),
        e.get(message, BSSoundMessage::curve_3),
        e.get(message, BSSoundMessage::curve_4),
    ];
    if sound_is_ready(e, sound) {
        let words = curve.map(|value| value as u32);
        e.vcall(sound, SOUND_SLOT_SET_ATTENUATION_CURVE, &words);
    } else {
        let id = e.get(message, BSSoundMessage::iID);
        fn_00adbb30(
            e, this, id, curve[0], curve[1], curve[2], curve[3], curve[4],
        );
    }
}

/// Message 40 (`MPURGE_MOVING_SOUNDS`): every entry of the moving-reference
/// map is removed (its object released when the map held the only
/// reference) and its sound, if the id still has one, is stopped, marked
/// 0x10 and loses 0x200. (A sound slot is kept between entries: an id with
/// no sound reuses the previous entry's sound.)
fn message_purge_moving_sounds(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let moving = this.at(BSAudioManager::MovingRefrList);
    let sounds = this.at(BSAudioManager::SoundList);
    let object_slot = e.mem.alloc(4);
    let mut position = map_first_item(e, moving);
    let mut sound = 0;
    while position != 0 {
        let key = e.with_stack(8, |e, words| {
            e.mem.set_u32(words.addr(), position);
            fn_00ae7ee0(e, moving, words.addr(), words.addr() + 4, object_slot);
            position = e.mem.u32(words.addr());
            e.mem.u32(words.addr() + 4)
        });
        e.call(MAP_REMOVE, &[moving.addr(), key]);
        let object = e.mem.u32(object_slot);
        if e.mem.u32(object + 4) == 1 {
            e.call(RELEASE_MESSAGE_POINTER, &[object]);
        }
        e.call(SMART_POINTER_SET, &[object_slot, 0]);
        let (_, found) = map_get(e, sounds, key, sound);
        sound = found;
        if sound != 0 {
            e.vcall(sound, SOUND_SLOT_STOP, &[]);
            set_state_bits(e, sound, 0x10);
            clear_state_bits(e, sound, 0x200);
        }
    }
    let held = e.mem.u32(object_slot);
    drop_reference(e, held);
    e.mem.free(object_slot);
}

/// Message 41 (`MSTOP_MOVING_SOUNDS`): the message's smart pointer names an
/// object; every sound of the moving-reference map that follows that object
/// is stopped when the mask in `pData` is 0 or shares a bit with the sound's
/// type, and any sound with a non-zero type gets type bit 0x100. (A sound
/// slot is kept between entries.)
fn message_stop_moving_sounds(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    let moving = this.at(BSAudioManager::MovingRefrList);
    let sounds = this.at(BSAudioManager::SoundList);
    let slots = e.mem.alloc(8);
    let wanted_slot = slots;
    let object_slot = slots + 4;
    e.call(SMART_POINTER_ASSIGN, &[wanted_slot, message.addr() + 0x10]);
    let mask = e.get(message, BSSoundMessage::pData);
    let mut position = map_first_item(e, moving);
    let mut sound = 0;
    while position != 0 {
        let key = e.with_stack(8, |e, words| {
            e.mem.set_u32(words.addr(), position);
            fn_00ae7ee0(e, moving, words.addr(), words.addr() + 4, object_slot);
            position = e.mem.u32(words.addr());
            e.mem.u32(words.addr() + 4)
        });
        if e.mem.u32(object_slot) == e.mem.u32(wanted_slot) {
            let (_, found) = map_get(e, sounds, key, sound);
            sound = found;
            if sound != 0 {
                let kind = e.mem.u32(sound + SOUND_TYPE_FLAGS);
                if mask == 0 || kind & mask != 0 {
                    e.vcall(sound, SOUND_SLOT_STOP, &[]);
                }
                let kind = e.mem.u32(sound + SOUND_TYPE_FLAGS);
                if kind != 0 {
                    e.mem.set_u32(sound + SOUND_TYPE_FLAGS, kind | 0x100);
                }
            }
        }
    }
    let held = e.mem.u32(object_slot);
    drop_reference(e, held);
    let held = e.mem.u32(wanted_slot);
    drop_reference(e, held);
    e.mem.free(slots);
}

/// Message 42 (`MKILL_ALL_OFTYPE`): every sound whose type shares a bit with
/// `iData` and has neither 0x800 nor 0x100000 is stopped, loses state bit
/// 0x200 and gets 0x10.
fn message_kill_all_of_type(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    let mask = e.get(message, BSSoundMessage::iData);
    for_each_sound(e, this, |e, _key, sound| {
        let kind = e.mem.u32(sound + SOUND_TYPE_FLAGS);
        if kind & mask != 0 && kind & 0x800 == 0 && kind & 0x10_0000 == 0 {
            e.vcall(sound, SOUND_SLOT_STOP, &[]);
            clear_state_bits(e, sound, 0x200);
            set_state_bits(e, sound, 0x10);
        }
    });
}

/// Message 51 (`MADD_SOUND_TO_CACHE`): `pData` is a loaded sound; it goes
/// through the cache upkeep (`00adcd80`) and is then deleted.
fn message_add_sound_to_cache(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    let sound = e.get(message, BSSoundMessage::pData);
    if sound != 0 {
        fn_00adcd80(e, this, sound);
        e.vcall(sound, SOUND_SLOT_DESTRUCTOR, &[1]);
    }
}

/// The step `MTERMINATE_THREAD` and `MSUSPEND_THREAD` begin with: the message
/// goes on the garbage stack (+0x44).
fn put_message_in_garbage(e: &mut Engine, this: Ptr<BSAudioManager>, message: Ptr<BSSoundMessage>) {
    e.call(MESSAGE_STACK_PUSH, &[this.addr() + 0x44, message.addr()]);
}

/// Whether the audio runs on its own thread, `ProcessMessage` runs on it and
/// the manager has a thread object: the condition of the two thread
/// messages.
fn on_audio_thread(e: &mut Engine, this: Ptr<BSAudioManager>) -> bool {
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    e.mem.u8(audio + 6) != 0
        && e.call(IMPORT_GET_CURRENT_THREAD_ID, &[]).u32()
            == e.get(this, BSAudioManager::dwAudioThreadID)
        && e.get(this, BSAudioManager::pUpdateThread) != 0
}

/// Message 52 (`MTERMINATE_THREAD`): the message goes on the garbage stack;
/// when it is running on the audio thread, the multi-thread flag
/// (`BSAudio` +6) is cleared and the thread object's stop step (`00ae7ac0`)
/// runs.
fn message_terminate_thread(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    put_message_in_garbage(e, this, message);
    if on_audio_thread(e, this) {
        let audio = e.global::<u32>(AUDIO_GLOBAL);
        e.mem.set_u8(audio + 6, 0);
        let thread = e.get(this, BSAudioManager::pUpdateThread);
        fn_00ae7ac0(e, Ptr::new(thread));
    }
}

/// Message 53 (`MSUSPEND_THREAD`): the message goes on the garbage stack;
/// on the audio thread the multi-thread flag is cleared and, with the
/// processing critical section (+0xf4) left, the thread object's suspend
/// step (`00ae7a50`) runs before the section is entered again; `ProcessMessage`
/// then answers true. Off the audio thread, a thread id that is not the
/// manager's one raises a debugger break.
fn message_suspend_thread(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) -> bool {
    put_message_in_garbage(e, this, message);
    if on_audio_thread(e, this) {
        let audio = e.global::<u32>(AUDIO_GLOBAL);
        e.mem.set_u8(audio + 6, 0);
        let section = this.addr() + 0xf4;
        leave_section(e, section);
        let thread = e.get(this, BSAudioManager::pUpdateThread);
        fn_00ae7a50(e, Ptr::new(thread));
        enter_section(e, section);
        true
    } else {
        if e.call(IMPORT_GET_CURRENT_THREAD_ID, &[]).u32()
            != e.get(this, BSAudioManager::dwAudioThreadID)
        {
            e.call(IMPORT_DEBUG_BREAK, &[]);
        }
        false
    }
}

/// Message 55 (`MSET_UNDERWATER`): every sound that is not marked 0x10 and
/// has modifier bit 1 or type bit 0x1000 runs `00ae8b90` with the
/// listener-underwater flag.
fn message_set_underwater(e: &mut Engine, this: Ptr<BSAudioManager>) {
    for_each_sound(e, this, |e, _key, sound| {
        let state = e.mem.u32(sound + SOUND_STATE_FLAGS);
        if state & 0x10 == 0
            && (e.mem.u32(sound + SOUND_MODIFIER_FLAGS) & 1 != 0
                || e.mem.u32(sound + SOUND_TYPE_FLAGS) & 0x1000 != 0)
        {
            let underwater = e.get(this, BSAudioManager::bListenerUnderwater) as u32;
            sound_environment_step(e, sound, underwater);
        }
    });
}

/// Message 56 (`MSET_LISTENER_POSITION`): with a listener (`BSAudio` +0x10)
/// its slot 8 gets a pointer to a stack copy of the message's vector
/// (`00ae4f7c`..`00ae5009`: copied to EBP-0x198, then `LEA`/`PUSH` of it),
/// and `bListenerMoved` is set.
fn message_set_listener_position(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    let x = e.get(message, BSSoundMessage::vector3_x);
    let y = e.get(message, BSSoundMessage::vector3_y);
    let z = e.get(message, BSSoundMessage::vector3_z);
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    let listener = e.mem.u32(audio + AUDIO_LISTENER);
    if listener != 0 {
        e.with_stack(12, |e, position| {
            e.mem.set_f32(position.addr(), x);
            e.mem.set_f32(position.addr() + 4, y);
            e.mem.set_f32(position.addr() + 8, z);
            e.vcall(listener, LISTENER_SLOT_SET_POSITION, &[position.addr()]);
        });
        e.set(this, BSAudioManager::bListenerMoved, true);
    }
}

/// Message 57 (`MDATA_REQUEST`): a ready sound processes the data request
/// `iData` (slot 0x48); a sound that is not ready (and not marked 0x10) has
/// the message queued again.
fn message_data_request(e: &mut Engine, this: Ptr<BSAudioManager>, message: Ptr<BSSoundMessage>) {
    if let Some(sound) = message_sound(e, this, message) {
        if sound_is_ready(e, sound) {
            let request = e.get(message, BSSoundMessage::iData);
            e.vcall(sound, SOUND_SLOT_PROCESS_DATA_REQUEST, &[request]);
        } else if e.mem.u32(sound + SOUND_STATE_FLAGS) & 0x10 == 0 {
            let id = e.get(message, BSSoundMessage::iID);
            let data = e.get(message, BSSoundMessage::iData);
            queue_plain(e, this, 0x39, id, data, 0);
        }
    }
}

/// Message 60 (`MSET_SYNCH_START`) and message 61 (`MCLEAR_SYNCH_START`):
/// a sound with state bit 0x100 gets the start time `iData` (message 60, and
/// type bit 0x1000000) or none (message 61, clearing the bit); a sound
/// without it (and not marked 0x10) has the message queued again.
fn message_synch_start(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
    set: bool,
) {
    let Some(sound) = message_sound(e, this, message) else {
        return;
    };
    let state = e.mem.u32(sound + SOUND_STATE_FLAGS);
    if state & 0x100 != 0 {
        let kind = e.mem.u32(sound + SOUND_TYPE_FLAGS);
        if set {
            let start = e.get(message, BSSoundMessage::iData);
            e.mem.set_u32(sound + SOUND_SYNCH_START_TIME, start);
            e.mem.set_u32(sound + SOUND_TYPE_FLAGS, kind | 0x100_0000);
        } else {
            e.mem.set_u32(sound + SOUND_SYNCH_START_TIME, 0);
            e.mem.set_u32(sound + SOUND_TYPE_FLAGS, kind & !0x100_0000);
        }
    } else if state & 0x10 == 0 {
        requeue_message(e, this, message);
    }
}

/// Message 62 (`MSET_TIME_CONSTRAINTS`): the sound's begin and end times
/// (+0x28, +0x2c) become `iData` and the low 16 bits of `pData`, times
/// 0.09375.
fn message_set_time_constraints(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    if let Some(sound) = message_sound(e, this, message) {
        let scale = e.mem.f64(TIME_CONSTRAINT_SCALE);
        let begin = e.get(message, BSSoundMessage::iData) as i32;
        e.mem
            .set_f32(sound + SOUND_BEGIN_TIME, (begin as f64 * scale) as f32);
        let end = (e.get(message, BSSoundMessage::pData) & 0xffff) as i32;
        e.mem
            .set_f32(sound + SOUND_END_TIME, (end as f64 * scale) as f32);
    }
}

/// Message 65 (`MSET_PRIORITY`): the sound's priority (+0x188) becomes
/// `iData`.
fn message_set_priority(e: &mut Engine, this: Ptr<BSAudioManager>, message: Ptr<BSSoundMessage>) {
    if let Some(sound) = message_sound(e, this, message) {
        let priority = e.get(message, BSSoundMessage::iData);
        e.mem.set_u32(sound + SOUND_PRIORITY, priority);
    }
}

// Translated from 00adf080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::ProcessMessage` (Xbox PDB): carries out one queued
/// message and answers whether the caller must stop processing (only
/// `MSUSPEND_THREAD` on the audio thread). The audio TLS context word is 0xb
/// while it runs. The message kinds are the Xbox PDB's `SOUND_MSG`
/// (1 `MPLAY` to 66 `MSET_MOD_FREQUENCY`); a kind without a case does
/// nothing. Most cases look the sound up in the sound list (+0x54) by the
/// message's id and act on it only when it is ready (state bits 0x800 and
/// 0x100); others keep the message in the sound's own queue or queue it
/// again until it is.
// Translation incomplete: 00adf080 - C++ exception unwinding (SEH frame state) around the smart-pointer temporaries and the local message list is not translated.
pub fn fn_00adf080(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) -> bool {
    in_audio_context(e, |e| {
        let now = e.call(IMPORT_GET_TICK_COUNT, &[]).u32();
        let mut stop = false;
        match e.get(message, BSSoundMessage::msg) {
            1 => message_play(e, this, message),
            2 => message_play_looping(e, this, message),
            3 => message_pause(e, this, message),
            4 => message_stop(e, this, message),
            5 => message_play_after(e, this, message, now),
            7 => message_mute(e, this, message, 10000),
            8 => message_mute(e, this, message, 0),
            9 => message_fade_out(e, this, message, now),
            10 => message_fade_to(e, this, message, now),
            11 => message_fade_in(e, this, message, now),
            12 => message_fade_from(e, this, message, now),
            13 => message_dialogue_fade(e, this, message, now),
            14 => message_volume_levels_changed(e, this),
            16 => message_set_volume(e, this, message),
            19 => message_set_frequency(e, this, message),
            20 => message_set_frequency_variance(e, this, message),
            24 => message_mute_all_of_type(e, this, message, now),
            25 => message_unmute_all_of_type(e, this, message, now),
            26 => message_pause_all_of_type(e, this, message),
            27 => message_unpause_all_of_type(e, this, message),
            28 => message_pause_all_of_type_exact(e, this, message),
            29 => message_unpause_all_of_type_exact(e, this, message),
            30 => message_set_position(e, this, message),
            31 => message_set_beam_end(e, this, message),
            33 => message_set_static_attenuation(e, this, message),
            34 => message_set_reverb_attenuation(e, this, message),
            35 => message_set_min_max_distance(e, this, message),
            36 => message_set_attenuation_curve(e, this, message),
            37 => message_add_sound_to_list(e, this, message),
            38 => message_delete_sound(e, this, message),
            39 => message_register_moving_sound(e, this, message),
            40 => message_purge_moving_sounds(e, this),
            41 => message_stop_moving_sounds(e, this, message),
            42 => message_kill_all_of_type(e, this, message),
            51 => message_add_sound_to_cache(e, this, message),
            52 => message_terminate_thread(e, this, message),
            53 => stop = message_suspend_thread(e, this, message),
            55 => message_set_underwater(e, this),
            56 => message_set_listener_position(e, this, message),
            57 => message_data_request(e, this, message),
            58 => message_set_completion_callback(e, this, message),
            59 => message_set_play_callback(e, this, message),
            60 => message_synch_start(e, this, message, true),
            61 => message_synch_start(e, this, message, false),
            62 => message_set_time_constraints(e, this, message),
            63 => message_seek(e, this, message),
            64 => message_set_at_listener(e, this, message),
            65 => message_set_priority(e, this, message),
            66 => message_set_mod_frequency(e, this, message),
            _ => {}
        }
        stop
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ad9060, bs_audio_manager_q_instance() -> Ptr<BSAudioManager>),
        entry!(
            0x00ad90e0,
            fn_00ad90e0(Ptr<BSAudioManager>) -> Ptr<BSAudioManager>
        ),
        entry!(0x00ad93f0, fn_00ad93f0(Ptr<BSAudioManager>)),
        entry!(0x00ad9640, bs_audio_manager_initialize(Ptr<BSAudioManager>)),
        entry!(0x00ad9870, bs_audio_manager_shutdown(Ptr<BSAudioManager>)),
        entry!(0x00ad9da0, fn_00ad9da0(Ptr<BSAudioManager>)),
        entry!(
            0x00ad9f30,
            bs_audio_manager_start_audio_thread(Ptr<BSAudioManager>)
        ),
        entry!(
            0x00ada020,
            bs_audio_manager_suspend_audio_thread(Ptr<BSAudioManager>)
        ),
        entry!(
            0x00ada0f0,
            bs_audio_manager_stop_audio_thread(Ptr<BSAudioManager>)
        ),
        entry!(
            0x00ada1d0,
            bs_audio_manager_update(Ptr<BSAudioManager>, u32)
        ),
        entry!(0x00ada300, fn_00ada300(Ptr<BSAudioManager>, f32, u32)),
        entry!(
            0x00ada330,
            bs_audio_manager_synch_voice_states(Ptr<BSAudioManager>)
        ),
        entry!(
            0x00ada5d0,
            bs_audio_manager_set_voice_state(Ptr<BSAudioManager>, u32, Ptr)
        ),
        entry!(
            0x00adac30,
            fn_00adac30(Ptr<BSAudioManager>, f32, u32) -> f32
        ),
        entry!(
            0x00adad60,
            fn_00adad60(Ptr<BSAudioManager>) -> Ptr<BSSoundMessage>
        ),
        entry!(
            0x00adad80,
            bs_audio_manager_free_message(Ptr<BSAudioManager>, Ptr<BSSoundMessage>)
        ),
        entry!(
            0x00adaf80,
            fn_00adaf80(Ptr<BSAudioManager>) -> Ptr<BSSoundMessage>
        ),
        entry!(
            0x00adafa0,
            bs_audio_manager_free_queue_message(Ptr<BSAudioManager>, Ptr<BSSoundMessage>)
        ),
        entry!(
            0x00adb1a0,
            fn_00adb1a0(Ptr<BSAudioManager>, u32, u32, u32, u32, u32, f32, f32, f32)
        ),
        entry!(
            0x00adb3e0,
            bs_audio_manager_play(Ptr<BSAudioManager>, u32, u8) -> bool
        ),
        entry!(
            0x00adb4e0,
            bs_audio_manager_play_after(Ptr<BSAudioManager>, u32, u32, u32) -> bool
        ),
        entry!(
            0x00adb590,
            bs_audio_manager_pause(Ptr<BSAudioManager>, u32) -> bool
        ),
        entry!(
            0x00adb610,
            bs_audio_manager_stop(Ptr<BSAudioManager>, u32) -> bool
        ),
        entry!(
            0x00adb690,
            bs_audio_manager_stop_moving_sounds(Ptr<BSAudioManager>, u32, f32, u32)
        ),
        entry!(
            0x00adb720,
            bs_audio_manager_set_static_attenuation(Ptr<BSAudioManager>, u32, u16) -> bool
        ),
        entry!(
            0x00adb7c0,
            fn_00adb7c0(Ptr<BSAudioManager>, u32, f32) -> bool
        ),
        entry!(
            0x00adb850,
            bs_audio_manager_set_mod_frequency(Ptr<BSAudioManager>, u32, f32) -> bool
        ),
        entry!(
            0x00adb8e0,
            fn_00adb8e0(Ptr<BSAudioManager>, u32, u8) -> bool
        ),
        entry!(
            0x00adb960,
            fn_00adb960(Ptr<BSAudioManager>, u32, u32) -> bool
        ),
        entry!(
            0x00adb970,
            bs_audio_manager_set_position(Ptr<BSAudioManager>, u32, f32, f32, f32) -> bool
        ),
        entry!(
            0x00adb9f0,
            fn_00adb9f0(Ptr<BSAudioManager>, u32, f32, f32, f32) -> bool
        ),
        entry!(
            0x00adba70,
            bs_audio_manager_set_min_max(Ptr<BSAudioManager>, u32, f32, f32)
        ),
        entry!(
            0x00adbb30,
            fn_00adbb30(Ptr<BSAudioManager>, u32, u16, u16, u16, u16, u16)
        ),
        entry!(0x00adbec0, fn_00adbec0(Ptr<BSAudioManager>, u32, u16)),
        entry!(
            0x00adbf40,
            fn_00adbf40(Ptr<BSAudioManager>, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00adbf50,
            bs_audio_manager_release(Ptr<BSAudioManager>, u32) -> bool
        ),
        entry!(
            0x00adbfd0,
            bs_audio_manager_set_completion_callback(Ptr<BSAudioManager>, u32, u32, u32) -> bool
        ),
        entry!(
            0x00adc050,
            fn_00adc050(Ptr<BSAudioManager>, u32, u32, u32) -> bool
        ),
        entry!(
            0x00adc0d0,
            bs_audio_manager_set_volume(Ptr<BSAudioManager>, u32, f32) -> bool
        ),
        entry!(
            0x00adc160,
            bs_audio_manager_volume_changed(Ptr<BSAudioManager>)
        ),
        entry!(
            0x00adc1e0,
            bs_audio_manager_set_object_to_follow(Ptr<BSAudioManager>, u32, u32)
        ),
        entry!(0x00adc260, fn_00adc260(Ptr<BSAudioManager>, u32, u32)),
        entry!(0x00adc2e0, fn_00adc2e0(Ptr<BSAudioManager>, u32, u8)),
        entry!(
            0x00adc360,
            bs_audio_manager_set_synch_start_time(Ptr<BSAudioManager>, u32, u32)
        ),
        entry!(0x00adc3e0, fn_00adc3e0(Ptr<BSAudioManager>, u32)),
        entry!(
            0x00adc460,
            bs_audio_manager_set_priority(Ptr<BSAudioManager>, u32, u32)
        ),
        entry!(0x00adc4e0, fn_00adc4e0(Ptr<BSAudioManager>, u32, u8, u8)),
        entry!(
            0x00adc560,
            bs_audio_manager_fade_out(Ptr<BSAudioManager>, u32, u32, u32)
        ),
        entry!(0x00adc5f0, fn_00adc5f0(Ptr<BSAudioManager>, u32, u32, u32)),
        entry!(0x00adc670, fn_00adc670(Ptr<BSAudioManager>, u32, u32)),
        entry!(
            0x00adc7d0,
            bs_audio_manager_is_valid(Ptr<BSAudioManager>, u32) -> bool
        ),
        entry!(
            0x00adc830,
            bs_audio_manager_get_duration(Ptr<BSAudioManager>, u32) -> u32
        ),
        entry!(
            0x00adc8b0,
            bs_audio_manager_get_volume(Ptr<BSAudioManager>, u32) -> f32
        ),
        entry!(
            0x00adc910,
            bs_audio_manager_is_playing(Ptr<BSAudioManager>, u32) -> bool
        ),
        entry!(0x00adc9a0, fn_00adc9a0(Ptr<BSAudioManager>, u32) -> u32),
        entry!(
            0x00adca20,
            bs_audio_manager_mute_all_of_type(Ptr<BSAudioManager>, u32, u32, u32)
        ),
        entry!(0x00adcaa0, fn_00adcaa0(Ptr<BSAudioManager>, u32, u32)),
        entry!(0x00adcb20, fn_00adcb20(Ptr<BSAudioManager>, u32, u8)),
        entry!(0x00adcc10, fn_00adcc10(Ptr<BSAudioManager>, u32, u8)),
        entry!(
            0x00adcd00,
            bs_audio_manager_kill_all_of_type(Ptr<BSAudioManager>, u32, u32)
        ),
        entry!(0x00adcd80, fn_00adcd80(Ptr<BSAudioManager>, u32)),
        entry!(
            0x00add1e0,
            fn_00add1e0(Ptr<BSAudioManager>, u32, u32, u32) -> u32
        ),
        entry!(
            0x00add480,
            bs_audio_manager_is_in_cache(Ptr<BSAudioManager>, u32, u8) -> bool
        ),
        entry!(
            0x00add570,
            bs_audio_manager_precache(Ptr<BSAudioManager>, u32, u32)
        ),
        entry!(
            0x00add630,
            bs_audio_manager_precache_ov2(Ptr<BSAudioManager>, u32, u32)
        ),
        entry!(
            0x00add6f0,
            bs_audio_manager_precache_ov3(Ptr<BSAudioManager>, u32, u32, u32)
        ),
        entry!(0x00add960, fn_00add960(Ptr<BSAudioManager>, u32)),
        entry!(0x00addc00, bs_audio_manager_process(Ptr<BSAudioManager>)),
        entry!(
            0x00addc70,
            bs_audio_manager_process_messages(Ptr<BSAudioManager>)
        ),
        entry!(0x00adddd0, fn_00adddd0(Ptr<BSAudioManager>)),
        entry!(0x00adea90, fn_00adea90(Ptr<BSAudioManager>)),
        entry!(
            0x00adef80,
            bs_audio_manager_position_listener(Ptr<BSAudioManager>, f32, f32, f32)
        ),
        entry!(
            0x00adeff0,
            bs_audio_manager_set_listener_underwater(Ptr<BSAudioManager>, u8)
        ),
        entry!(
            0x00adf080,
            fn_00adf080(Ptr<BSAudioManager>, Ptr<BSSoundMessage>) -> bool
        ),
        entry!(
            0x00ae5680,
            bs_audio_manager_get_sound_handle_by_name(
                Ptr<BSAudioManager>,
                Ptr<BSSoundHandle>,
                u32,
                u32,
            ) -> Ptr<BSSoundHandle>
        ),
        entry!(
            0x00ae5870,
            bs_audio_manager_get_sound_handle_by_numeric_id(
                Ptr<BSAudioManager>,
                Ptr<BSSoundHandle>,
                u32,
                u32,
            ) -> Ptr<BSSoundHandle>
        ),
        entry!(
            0x00ae5a50,
            bs_audio_manager_get_sound_handle_by_filename(
                Ptr<BSAudioManager>,
                Ptr<BSSoundHandle>,
                u32,
                u32,
                u32,
            ) -> Ptr<BSSoundHandle>
        ),
        entry!(
            0x00ae5f70,
            fn_00ae5f70(
                Ptr<BSAudioManager>,
                Ptr<BSSoundHandle>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> Ptr<BSSoundHandle>
        ),
        entry!(0x00ae5fb0, fn_00ae5fb0(u32, u32) -> i32),
        entry!(
            0x00ae5ff0,
            bs_audio_manager_get_debug_string(Ptr<BSAudioManager>, u32) -> u32
        ),
        entry!(
            0x00ae6bf0,
            bs_audio_manager_set_audio_cache_size_in_k(Ptr<BSAudioManager>, u32)
        ),
        entry!(0x00ae6c60, fn_00ae6c60(Ptr<BSAudioManager>) -> u32),
        entry!(0x00ae6ca0, fn_00ae6ca0(Ptr<BSAudioManager>, u8)),
        entry!(0x00ae6cf0, fn_00ae6cf0(Ptr<BSAudioManager>) -> u8),
        entry!(0x00ae6d30, fn_00ae6d30(Ptr<BSAudioManager>, u32) -> bool),
        entry!(
            0x00ae6de0,
            fn_00ae6de0(Ptr<BSAudioManager>, u32, u8) -> bool
        ),
        entry!(
            0x00ae6e60,
            fn_00ae6e60(Ptr<BSAudioManager>, u32, u8) -> bool
        ),
        entry!(
            0x00ae6f10,
            bs_audio_manager_remove_load_task(Ptr<BSAudioManager>, u32)
        ),
        entry!(
            0x00ae6f50,
            fn_00ae6f50(Ptr<AudioLoadTask>, u32) -> Ptr<AudioLoadTask>
        ),
        entry!(0x00ae7020, fn_00ae7020(Ptr<AudioLoadTask>)),
        entry!(0x00ae7050, audio_load_task_run(Ptr<AudioLoadTask>)),
        entry!(0x00ae7360, audio_load_task_finish(Ptr<AudioLoadTask>)),
        entry!(
            0x00ae7390,
            audio_load_task_cancel(Ptr<AudioLoadTask>, u32, u32)
        ),
        entry!(
            0x00ae7480,
            audio_load_task_get_description(Ptr<AudioLoadTask>, u32, u32) -> u32
        ),
        entry!(
            0x00ae74b0,
            bs_audio_manager_thread_thread_proc(Ptr<BSAudioManagerThread>) -> u32
        ),
        entry!(
            0x00ae7510,
            fn_00ae7510(Ptr<AudioMessageStack>) -> Ptr<AudioMessageStack>
        ),
        entry!(0x00ae7550, fn_00ae7550(Ptr<AudioMessageStack>)),
        entry!(
            0x00ae75f0,
            audio_message_stack_push(Ptr<AudioMessageStack>, u32)
        ),
        entry!(0x00ae76b0, fn_00ae76b0(Ptr<AudioMessageStack>) -> u32),
        entry!(
            0x00ae77a0,
            bs_audio_manager_scalar_deleting_destructor(
                Ptr<BSAudioManager>,
                u32,
            ) -> Ptr<BSAudioManager>
        ),
        entry!(
            0x00ae77d0,
            bs_audio_manager_thread_constructor(
                Ptr<BSAudioManagerThread>,
            ) -> Ptr<BSAudioManagerThread>
        ),
        entry!(
            0x00ae7870,
            bs_audio_manager_thread_scalar_deleting_destructor(
                Ptr<BSAudioManagerThread>,
                u32,
            )
                -> Ptr<BSAudioManagerThread>
        ),
        entry!(0x00ae78a0, fn_00ae78a0(Ptr<BSAudioManagerThread>)),
        entry!(
            0x00ae7920,
            bs_sound_info_operator_assign(Ptr<BSSoundInfo>, Ptr<BSSoundInfo>) -> Ptr<BSSoundInfo>
        ),
        entry!(
            0x00ae7990,
            fn_00ae7990(Ptr<BSSoundMessage>, Ptr<BSSoundMessage>) -> Ptr<BSSoundMessage>
        ),
        entry!(0x00ae7a50, fn_00ae7a50(Ptr<BSAudioManagerThread>)),
        entry!(0x00ae7ac0, fn_00ae7ac0(Ptr<BSAudioManagerThread>)),
        entry!(
            0x00ae7b10,
            audio_load_task_scalar_deleting_destructor(
                Ptr<AudioLoadTask>,
                u32,
            ) -> Ptr<AudioLoadTask>
        ),
        entry!(
            0x00ae7b40,
            fn_00ae7b40(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00ae7bc0,
            fn_00ae7bc0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00ae7c40,
            fn_00ae7c40(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00ae7cc0, fn_00ae7cc0(Ptr<NiTPointerMap>, u32, u32)),
        entry!(
            0x00ae7e60,
            fn_00ae7e60(Ptr<NiTPointerMap>, u32, u32) -> bool
        ),
        entry!(0x00ae7ee0, fn_00ae7ee0(Ptr<NiTPointerMap>, u32, u32, u32)),
        entry!(0x00ae8150, fn_00ae8150(u32, u32) -> u32),
        entry!(
            0x00ae81e0,
            bs_simple_list_audio_debug_string_sort(Ptr<BSSimpleList>, u32)
        ),
        entry!(0x00ae8370, fn_00ae8370(Ptr<NiTPointerList>)),
        entry!(0x00ae83e0, fn_00ae83e0(Ptr<BSSimpleList>)),
        entry!(0x00ae8470, fn_00ae8470(Ptr<BSSimpleList>, u32)),
        entry!(0x00ae8550, fn_00ae8550(Ptr<BSSimpleList>, u32, u32, u32)),
        entry!(
            0x00ae8650,
            fn_00ae8650(Ptr<BSSoundMessage>) -> Ptr<BSSoundMessage>
        ),
        entry!(0x00ae86d0, bs_sound_message_destructor(Ptr<BSSoundMessage>)),
        entry!(
            0x00ae8770,
            bs_debug_sound_info_constructor(Ptr<BSDebugSoundInfo>) -> Ptr<BSDebugSoundInfo>
        ),
        entry!(
            0x00ae8830,
            bs_debug_sound_info_destructor(Ptr<BSDebugSoundInfo>)
        ),
        entry!(
            0x00ae88c0,
            bs_debug_sound_info_operator_assign(
                Ptr<BSDebugSoundInfo>,
                Ptr<BSDebugSoundInfo>,
            ) -> Ptr<BSDebugSoundInfo>
        ),
        entry!(
            0x00ae89f0,
            bs_sound_info_constructor(Ptr<BSSoundInfo>) -> Ptr<BSSoundInfo>
        ),
        entry!(
            0x00ae8a40,
            bs_sound_info_copy_constructor(Ptr<BSSoundInfo>, Ptr<BSSoundInfo>) -> Ptr<BSSoundInfo>
        ),
        entry!(0x00ae8af0, bs_sound_info_destructor(Ptr<BSSoundInfo>)),
        entry!(0x00ae8b40, fn_00ae8b40(Ptr<BSSoundInfo>, u32) -> u32),
        entry!(0x00ae8b80, fn_00ae8b80(u32) -> u8),
        entry!(0x00ae8b90, fn_00ae8b90(u32, u8)),
        entry!(0x00ae8bf0, fn_00ae8bf0(u32, u32)),
        entry!(0x00ae8c30, fn_00ae8c30(u32)),
        entry!(0x00ae8c60, fn_00ae8c60(u32)),
        entry!(0x00ae8ce0, fn_00ae8ce0(u32)),
        entry!(0x00ae8d10, fn_00ae8d10(u32)),
        entry!(0x00ae8d90, fn_00ae8d90(u32)),
        entry!(0x00ae8dc0, fn_00ae8dc0(u32, u32, u32) -> bool),
        entry!(
            0x00ae8de0,
            ni_t_map_base_reference_set_value(u32, u32, u32, u32)
        ),
        entry!(0x00ae8e70, fn_00ae8e70(u32)),
        entry!(0x00ae8ef0, ni_t_pointer_map_reference_delete_item(u32, u32)),
        entry!(0x00ae8f20, fn_00ae8f20(u32, u32) -> u32),
        entry!(0x00ae8f70, fn_00ae8f70(u32, u32) -> u32),
        entry!(0x00ae9010, fn_00ae9010(u32, u32) -> u32),
        entry!(0x00ae9060, fn_00ae9060(u32, u32) -> u32),
        entry!(
            0x00ae9100,
            ni_t_map_base_reference_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(
            0x00ae9150,
            ni_t_pointer_map_reference_scalar_deleting_destructor(u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine with the KERNEL32 imports the manager uses, the pages of
    /// exe data it reads mapped, and the constants set.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let imports = [
            (IMPORT_ENTER_CRITICAL_SECTION, "EnterCriticalSection"),
            (IMPORT_LEAVE_CRITICAL_SECTION, "LeaveCriticalSection"),
            (
                IMPORT_INITIALIZE_CRITICAL_SECTION,
                "InitializeCriticalSection",
            ),
            (IMPORT_DELETE_CRITICAL_SECTION, "DeleteCriticalSection"),
            (IMPORT_GET_TICK_COUNT, "GetTickCount"),
            (IMPORT_INTERLOCKED_INCREMENT, "InterlockedIncrement"),
            (IMPORT_INTERLOCKED_DECREMENT, "InterlockedDecrement"),
        ]
        .map(|(slot, name)| crate::exe::Import {
            slot,
            dll: "KERNEL32.dll".into(),
            name: name.into(),
        });
        assert_eq!(e.register_imports(&imports), 7);
        for page in [
            0x0101_2000,
            0x0101_7000,
            0x010a_3000,
            0x011f_4000,
            0x011f_5000,
            0x011f_6000,
            0x011f_7000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f64(MILLISECOND_SCALE, 1000.0);
        e.mem
            .set_f64(HUGE_DISTANCE_LIMIT, 3.402_823_466_385_288_6e37);
        e.mem.set_f32(HUGE_DISTANCE_REPLACEMENT, -1.0);
        e.mem.set_cstr(NOT_TES_SOUND_TEXT, b"NotTESSound");
        for (i, v) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(DEFAULT_MESSAGE_VECTOR + 4 * i as u32, v);
        }
        e
    }

    // ---- a small model of the pointer maps and lists the manager uses ----

    fn map_node(e: &Engine, map: u32, key: u32) -> Option<u32> {
        let size = e.mem.u32(map + 4);
        let table = e.mem.u32(map + 8);
        let mut node = e.mem.u32(table + (key % size) * 4);
        while node != 0 {
            if e.mem.u32(node + 4) == key {
                return Some(node);
            }
            node = e.mem.u32(node);
        }
        None
    }

    fn fake_get_value(e: &mut Engine, a: &[u32]) -> Ret {
        match map_node(e, a[0], a[1]) {
            Some(node) => {
                let value = e.mem.u32(node + 8);
                e.mem.set_u32(a[2], value);
                ret(1)
            }
            None => ret(0),
        }
    }

    fn fake_set_value(e: &mut Engine, a: &[u32]) -> Ret {
        if let Some(node) = map_node(e, a[0], a[1]) {
            e.mem.set_u32(node + 8, a[2]);
            return ret(0);
        }
        let size = e.mem.u32(a[0] + 4);
        let table = e.mem.u32(a[0] + 8);
        let slot = table + (a[1] % size) * 4;
        let node = e.mem.alloc(12);
        e.mem.set_u32(node, e.mem.u32(slot));
        e.mem.set_u32(node + 4, a[1]);
        e.mem.set_u32(node + 8, a[2]);
        e.mem.set_u32(slot, node);
        let count = e.mem.u32(a[0] + 0xc);
        e.mem.set_u32(a[0] + 0xc, count + 1);
        ret(0)
    }

    fn fake_remove(e: &mut Engine, a: &[u32]) -> Ret {
        let size = e.mem.u32(a[0] + 4);
        let table = e.mem.u32(a[0] + 8);
        let mut link = table + (a[1] % size) * 4;
        loop {
            let node = e.mem.u32(link);
            if node == 0 {
                return ret(0);
            }
            if e.mem.u32(node + 4) == a[1] {
                let next = e.mem.u32(node);
                e.mem.set_u32(link, next);
                let count = e.mem.u32(a[0] + 0xc);
                e.mem.set_u32(a[0] + 0xc, count - 1);
                return ret(1);
            }
            link = node;
        }
    }

    fn fake_get_next(e: &mut Engine, a: &[u32]) -> Ret {
        let current = e.mem.u32(a[1]);
        let key = e.mem.u32(current + 4);
        let value = e.mem.u32(current + 8);
        e.mem.set_u32(a[2], key);
        e.mem.set_u32(a[3], value);
        let mut next = e.mem.u32(current);
        if next == 0 {
            let size = e.mem.u32(a[0] + 4);
            let table = e.mem.u32(a[0] + 8);
            let mut index = key % size + 1;
            while index < size {
                let head = e.mem.u32(table + index * 4);
                if head != 0 {
                    next = head;
                    break;
                }
                index += 1;
            }
        }
        e.mem.set_u32(a[1], next);
        ret(0)
    }

    fn fake_map_clear(e: &mut Engine, a: &[u32]) -> Ret {
        let size = e.mem.u32(a[0] + 4);
        let table = e.mem.u32(a[0] + 8);
        for index in 0..size {
            e.mem.set_u32(table + index * 4, 0);
        }
        e.mem.set_u32(a[0] + 0xc, 0);
        ret(0)
    }

    fn install_map_model(e: &mut Engine) {
        e.register(MAP_GET_VALUE, fake_get_value);
        e.register(MAP_SET_VALUE, fake_set_value);
        e.register(MAP_REMOVE, fake_remove);
        e.register(MAP_GET_NEXT, fake_get_next);
        e.register(MAP_CLEAR, fake_map_clear);
    }

    /// Appends `element` to a pointer list (head, tail, count; nodes are
    /// `[next, previous, element]`).
    fn list_push(e: &mut Engine, list: u32, element: u32) {
        let node = e.mem.alloc(12);
        e.mem.set_u32(node + 8, element);
        let tail = e.mem.u32(list + 4);
        e.mem.set_u32(node + 4, tail);
        if tail == 0 {
            e.mem.set_u32(list, node);
        } else {
            e.mem.set_u32(tail, node);
        }
        e.mem.set_u32(list + 4, node);
        let count = e.mem.u32(list + 8);
        e.mem.set_u32(list + 8, count + 1);
    }

    fn fake_list_remove_head(e: &mut Engine, a: &[u32]) -> Ret {
        let node = e.mem.u32(a[0]);
        let next = e.mem.u32(node);
        e.mem.set_u32(a[0], next);
        if next == 0 {
            e.mem.set_u32(a[0] + 4, 0);
        } else {
            e.mem.set_u32(next + 4, 0);
        }
        let count = e.mem.u32(a[0] + 8);
        e.mem.set_u32(a[0] + 8, count - 1);
        ret(e.mem.u32(node + 8))
    }

    /// `NiPointer::operator=` as a test double: the destination takes the
    /// source's pointer, adding a reference to it and dropping one from the
    /// old one (the real function does this through its callees).
    fn fake_pointer_assign(e: &mut Engine, a: &[u32]) -> Ret {
        let new = e.mem.u32(a[1]);
        let old = e.mem.u32(a[0]);
        if new != old {
            if old != 0 {
                let count = e.mem.u32(old + 4);
                e.mem.set_u32(old + 4, count - 1);
            }
            e.mem.set_u32(a[0], new);
            if new != 0 {
                let count = e.mem.u32(new + 4);
                e.mem.set_u32(new + 4, count + 1);
            }
        }
        ret(a[0])
    }

    // ---- the message queue ----

    /// What a queued message holds.
    #[derive(Debug, PartialEq)]
    struct Message {
        kind: u32,
        a: u32,
        b: u32,
        c: u32,
        pointer: u32,
        vector: [f32; 3],
    }

    /// The message with the default vector and no pointer.
    fn plain(kind: u32, a: u32, b: u32, c: u32) -> Message {
        Message {
            kind,
            a,
            b,
            c,
            pointer: 0,
            vector: [1.0, 2.0, 3.0],
        }
    }

    /// A manager whose message pool hands out fresh messages and whose
    /// message list records what is appended, with the audio initialized.
    struct Fixture {
        e: Engine,
        this: Ptr<BSAudioManager>,
        queued: Rc<RefCell<Vec<u32>>>,
    }

    impl Fixture {
        fn new() -> Self {
            let mut e = engine();
            install_map_model(&mut e);
            let audio = e.mem.alloc(0x10);
            e.mem.set_u8(audio + 5, 1);
            e.set_global(AUDIO_GLOBAL, audio);
            e.register(MESSAGE_STACK_POP, |e, _| {
                let message = e.mem.alloc(0x20);
                ret(message)
            });
            e.register(SMART_POINTER_ASSIGN, fake_pointer_assign);
            let queued = Rc::new(RefCell::new(Vec::new()));
            let log = queued.clone();
            e.register_double(LIST_APPEND_POINTER, move |e, a| {
                log.borrow_mut().push(e.mem.u32(a[1]));
                Ret::default()
            });
            let this: Ptr<BSAudioManager> = e.new_object();
            for offset in [0x54, 0x64, 0x74, 0x84] {
                let table = e.mem.alloc(7 * 4);
                e.mem.set_u32(this.addr() + offset + 4, 7);
                e.mem.set_u32(this.addr() + offset + 8, table);
            }
            Fixture { e, this, queued }
        }

        fn messages(&self) -> Vec<Message> {
            self.queued
                .borrow()
                .iter()
                .map(|&m| Message {
                    kind: self.e.mem.u32(m),
                    a: self.e.mem.u32(m + 4),
                    b: self.e.mem.u32(m + 8),
                    c: self.e.mem.u32(m + 0xc),
                    pointer: self.e.mem.u32(m + 0x10),
                    vector: [
                        self.e.mem.f32(m + 0x14),
                        self.e.mem.f32(m + 0x18),
                        self.e.mem.f32(m + 0x1c),
                    ],
                })
                .collect()
        }

        /// Puts a record for sound `id` into the state map (+0x64): its id,
        /// volume, type word, duration and playing byte.
        fn add_state(&mut self, id: u32, volume: f32, sound_type: u32, duration: u32, playing: u8) {
            let record = self.e.mem.alloc(0x20);
            self.e.mem.set_u32(record, id);
            self.e.mem.set_f32(record + 8, volume);
            self.e.mem.set_u32(record + 0xc, sound_type);
            self.e.mem.set_u32(record + 0x10, duration);
            self.e.mem.set_u8(record + 0x14, playing);
            let map = self.this.addr() + 0x64;
            fake_set_value(&mut self.e, &[map, id, record]);
        }

        fn call(&mut self, address: u32, rest: &[u32]) -> Ret {
            let mut words = vec![self.this.addr()];
            words.extend_from_slice(rest);
            self.e.call(address, &words)
        }
    }

    #[test]
    fn q_instance_builds_the_manager_once() {
        let mut e = engine();
        let exits = Rc::new(RefCell::new(Vec::new()));
        let log = exits.clone();
        e.register_double(ATEXIT, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(0)
        });
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| ret(0));
        e.register(MESSAGE_STACK_CONSTRUCTOR, |_, _| ret(0));
        e.register(MAP_CONSTRUCTOR_SOUND_LIST, |_, _| ret(0));
        e.register(MAP_CONSTRUCTOR_SOUND_STATES, |_, _| ret(0));
        e.register(MAP_CONSTRUCTOR_MOVING_REFERENCES, |_, _| ret(0));
        assert_eq!(e.call(0x00ad9060, &[]).u32(), MANAGER_INSTANCE);
        assert_eq!(e.mem.u32(MANAGER_BUILT_GUARD) & 1, 1);
        assert_eq!(e.mem.u32(MANAGER_INSTANCE), 0x010a_3368);
        assert_eq!(*exits.borrow(), vec![MANAGER_EXIT_THUNK]);
        // The second call returns the same object without building again.
        e.mem.set_u32(MANAGER_INSTANCE, 0);
        assert_eq!(e.call(0x00ad9060, &[]).u32(), MANAGER_INSTANCE);
        assert_eq!(e.mem.u32(MANAGER_INSTANCE), 0);
        assert_eq!(exits.borrow().len(), 1);
    }

    #[test]
    fn constructor_sets_the_vtables_and_initial_state() {
        let mut e = engine();
        e.clock_ms = 4250;
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| ret(0));
        e.register(MESSAGE_STACK_CONSTRUCTOR, |_, _| ret(0));
        e.register(MAP_CONSTRUCTOR_SOUND_LIST, |_, _| ret(0));
        e.register(MAP_CONSTRUCTOR_SOUND_STATES, |_, _| ret(0));
        e.register(MAP_CONSTRUCTOR_MOVING_REFERENCES, |_, _| ret(0));
        let this: Ptr<BSAudioManager> = e.new_object();
        e.mem.write(this.addr(), &[0xaa; 0x188]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad90e0, &args![this]).u32(), this.addr());
        let log: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            log,
            vec![
                0x00ad90e0,
                VECTOR_CONSTRUCTOR_ITERATOR,
                MESSAGE_STACK_CONSTRUCTOR,
                MESSAGE_STACK_CONSTRUCTOR,
                MESSAGE_STACK_CONSTRUCTOR,
                MAP_CONSTRUCTOR_SOUND_LIST,
                MAP_CONSTRUCTOR_SOUND_STATES,
                MAP_CONSTRUCTOR_SOUND_STATES,
                MAP_CONSTRUCTOR_MOVING_REFERENCES,
                IMPORT_INITIALIZE_CRITICAL_SECTION,
                IMPORT_INITIALIZE_CRITICAL_SECTION,
                IMPORT_INITIALIZE_CRITICAL_SECTION,
                IMPORT_INITIALIZE_CRITICAL_SECTION,
                IMPORT_INITIALIZE_CRITICAL_SECTION,
                IMPORT_GET_TICK_COUNT,
            ]
        );
        assert_eq!(e.get(this, BSAudioManager::vtable), 0x010a_3368);
        assert_eq!(e.mem.u32(this.addr() + 0x54), 0x010a_3348);
        assert_eq!(e.mem.u32(this.addr() + 0x64), 0x010a_3328);
        assert_eq!(e.mem.u32(this.addr() + 0x74), 0x010a_3328);
        assert_eq!(e.mem.u32(this.addr() + 0x84), 0x010a_3308);
        assert_eq!(e.get(this, BSAudioManager::iMessageListIndex), 0);
        assert_eq!(e.get(this, BSAudioManager::iMessageProcessingListIndex), 1);
        assert_eq!(e.get(this, BSAudioManager::iNextID), 1);
        assert_eq!(e.get(this, BSAudioManager::iDelayTimerDelta), 4250);
        assert_eq!(e.get(this, BSAudioManager::fMasterVolume), 0.0);
        assert_eq!(e.get(this, BSAudioManager::fRadioSecondaryVolume), 0.0);
        assert_eq!(e.get(this, BSAudioManager::pUpdateThread), 0);
        assert_eq!(e.get(this, BSAudioManager::iMuteMask), 0);
        assert!(!e.get(this, BSAudioManager::bPrecacheCompleted));
        assert!(!e.get(this, BSAudioManager::bListenerUnderwater));
        assert_eq!(e.mem.u32(this.addr() + 0x94), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x98), 0);
        assert_eq!(e.mem.u32(this.addr() + 0xa0), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x124 + 8), 0);
    }

    #[test]
    fn destructor_tears_the_members_down_in_order() {
        let mut e = engine();
        e.register(LIST_CLEAR, |_, _| ret(0));
        e.register(MAP_CLEAR, |_, _| ret(0));
        e.register(FREE_TABLE, |_, _| ret(0));
        e.register(MESSAGE_STACK_DESTRUCTOR, |_, _| ret(0));
        e.register(VECTOR_DESTRUCTOR_ITERATOR, |_, _| ret(0));
        let this: Ptr<BSAudioManager> = e.new_object();
        let base = this.addr();
        e.mem.set_u32(base + 0x54 + 8, 0x1111);
        e.mem.set_u32(base + 0x64 + 8, 0x2222);
        e.mem.set_u32(base + 0x74 + 8, 0x3333);
        e.mem.set_u32(base + 0x84 + 8, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x00ad93f0, &args![this]);
        let log = e.call_log.take().unwrap();
        let expected: Vec<(u32, Vec<u32>)> = vec![
            (0x00ad93f0, vec![base]),
            (LIST_CLEAR, vec![base + 0x124]),
            (IMPORT_DELETE_CRITICAL_SECTION, vec![base + 0x10c]),
            (IMPORT_DELETE_CRITICAL_SECTION, vec![base + 0xf4]),
            (IMPORT_DELETE_CRITICAL_SECTION, vec![base + 0xdc]),
            (IMPORT_DELETE_CRITICAL_SECTION, vec![base + 0xc4]),
            (IMPORT_DELETE_CRITICAL_SECTION, vec![base + 0xac]),
            (LIST_CLEAR, vec![base + 0x94]),
            (MAP_CLEAR, vec![base + 0x84]),
            (MAP_CLEAR, vec![base + 0x84]),
            (FREE_TABLE, vec![0x4444]),
            (MAP_CLEAR, vec![base + 0x74]),
            (MAP_CLEAR, vec![base + 0x74]),
            (FREE_TABLE, vec![0x3333]),
            (MAP_CLEAR, vec![base + 0x64]),
            (MAP_CLEAR, vec![base + 0x64]),
            (FREE_TABLE, vec![0x2222]),
            (MAP_CLEAR, vec![base + 0x54]),
            (MAP_CLEAR, vec![base + 0x54]),
            (FREE_TABLE, vec![0x1111]),
            (MESSAGE_STACK_DESTRUCTOR, vec![base + 0x44]),
            (MESSAGE_STACK_DESTRUCTOR, vec![base + 0x34]),
            (MESSAGE_STACK_DESTRUCTOR, vec![base + 0x24]),
            (
                VECTOR_DESTRUCTOR_ITERATOR,
                vec![base + 0x0c, 0x0c, 2, MESSAGE_LIST_DESTRUCTOR],
            ),
        ];
        assert_eq!(log, expected);
        assert_eq!(e.get(this, BSAudioManager::vtable), 0x010a_3368);
        // The last map to be torn down ends with its base vtable.
        assert_eq!(e.mem.u32(base + 0x54), 0x010a_3370);
        assert_eq!(e.mem.u32(base + 0x84), 0x010a_33b0);
    }

    #[test]
    fn initialize_fills_the_message_stacks_and_makes_the_thread() {
        let mut e = engine();
        e.mem.set_f32(0x011f_6e18, 0.5);
        e.register(MEMSET, |_, _| ret(0));
        e.register(MESSAGE_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register(THREAD_CONSTRUCTOR, |_, a| ret(a[0]));
        let pushes = Rc::new(RefCell::new(Vec::new()));
        let log = pushes.clone();
        e.register_double(MESSAGE_STACK_PUSH, move |_, a| {
            log.borrow_mut().push((a[0], a[1]));
            ret(0)
        });
        let this: Ptr<BSAudioManager> = e.new_object();
        // Audio disabled: nothing happens.
        e.call(0x00ad9640, &args![this]);
        assert_eq!(e.get(this, BSAudioManager::iPauseListSize), 0);
        assert!(pushes.borrow().is_empty());
        e.mem.set_u8(AUDIO_ENABLED_SETTING, 1);
        e.mem.set_u32(this.addr() + 0xa4, 77);
        e.call_log = Some(vec![]);
        e.call(0x00ad9640, &args![this]);
        assert_eq!(e.get(this, BSAudioManager::iWeatherFlags), 0);
        assert_eq!(e.get(this, BSAudioManager::iPauseListSize), 8);
        let pause_entries = e.get(this, BSAudioManager::pPauseEntries);
        assert_ne!(pause_entries, 0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(MEMSET, vec![pause_entries, 0, 64])));
        assert_eq!(e.get(this, BSAudioManager::fMasterVolume), 0.5);
        let pushes = pushes.borrow();
        assert_eq!(pushes.len(), 0x200);
        assert!(pushes[..0x100].iter().all(|p| p.0 == this.addr() + 0x24));
        assert!(pushes[0x100..].iter().all(|p| p.0 == this.addr() + 0x34));
        assert!(pushes.iter().all(|p| p.1 != 0));
        assert!(e.get(
            this.at(BSAudioManager::FreeMessages),
            AudioMessageStack::bBottomless
        ));
        assert!(e.get(
            this.at(BSAudioManager::FreeQueueMessages),
            AudioMessageStack::bBottomless
        ));
        assert!(!e.get(
            this.at(BSAudioManager::GarbageMessages),
            AudioMessageStack::bBottomless
        ));
        let thread = e.get(this, BSAudioManager::pUpdateThread);
        assert_ne!(thread, 0);
    }

    #[test]
    fn volumes_come_from_the_default_settings() {
        let mut e = engine();
        let this: Ptr<BSAudioManager> = e.new_object();
        for (i, setting) in [
            0x011f_6e18,
            0x011f_6e24,
            0x011f_6e30,
            0x011f_6e3c,
            0x011f_6e48,
            0x011f_6e54,
        ]
        .into_iter()
        .enumerate()
        {
            e.mem.set_f32(setting, 0.1 * (i + 1) as f32);
        }
        e.call(0x00ad9da0, &args![this]);
        assert_eq!(e.get(this, BSAudioManager::fMasterVolume), 0.1f32);
        assert_eq!(e.get(this, BSAudioManager::fFootVolume), 0.2f32);
        assert_eq!(e.get(this, BSAudioManager::fVoiceVolume), 0.3f32);
        assert_eq!(e.get(this, BSAudioManager::fEffectsVolume), 0.4f32);
        assert_eq!(e.get(this, BSAudioManager::fMusicVolume), 0.5f32);
        assert_eq!(e.get(this, BSAudioManager::fRadioVolume), 0.6f32);
        for field in [
            BSAudioManager::fMasterSecondaryVolume,
            BSAudioManager::fFootSecondaryVolume,
            BSAudioManager::fVoiceSecondaryVolume,
            BSAudioManager::fEffectsSecondaryVolume,
            BSAudioManager::fMusicSecondaryVolume,
            BSAudioManager::fRadioSecondaryVolume,
        ] {
            assert_eq!(e.get(this, field), 1.0);
        }
    }

    #[test]
    fn start_audio_thread_initializes_a_thread_without_an_id() {
        let mut e = engine();
        e.register(THREAD_INITIALIZE, |e, a| {
            // The initialization gives the thread its id.
            e.mem.set_u32(a[0] + 0x24, 0x77);
            ret(0)
        });
        e.register(THREAD_SET_PROCESSOR, |_, _| ret(0));
        e.register(THREAD_SET_NAME, |_, _| ret(0));
        e.register(SET_THREAD_NAME, |_, _| ret(0));
        e.register(THREAD_SEMAPHORE_SIGNAL, |_, _| ret(0));
        e.set_global(AUDIO_THREAD_PROCESSOR, 3u32);
        let this: Ptr<BSAudioManager> = e.new_object();
        // No thread object: nothing is called.
        e.call_log = Some(vec![]);
        e.call(0x00ad9f30, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        let thread = e.mem.alloc(0x4c);
        e.set(this, BSAudioManager::pUpdateThread, thread);
        e.call_log = Some(vec![]);
        e.call(0x00ad9f30, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log,
            vec![
                (0x00ad9f30, vec![this.addr()]),
                (
                    THREAD_INITIALIZE,
                    vec![thread, 0x8000, AUDIO_THREAD_NAME, 0]
                ),
                (THREAD_SET_PROCESSOR, vec![thread, 3]),
                (THREAD_SET_NAME, vec![thread, AUDIO_THREAD_NAME]),
                (SET_THREAD_NAME, vec![0x77, AUDIO_THREAD_NAME]),
                (THREAD_SEMAPHORE_SIGNAL, vec![thread]),
            ]
        );
        assert_eq!(e.get(this, BSAudioManager::dwAudioThreadID), 0x77);
        // A thread that already has an id is only signalled.
        e.call_log = Some(vec![]);
        e.call(0x00ad9f30, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[1], (THREAD_SEMAPHORE_SIGNAL, vec![thread]));
    }

    /// The manager of a fixture with an audio thread object whose
    /// semaphore handle is 0x55 and whose counter is 3.
    fn thread_fixture() -> (Fixture, u32) {
        let mut f = Fixture::new();
        f.e.register(THREAD_SEMAPHORE_SIGNAL, |_, _| ret(0));
        f.e.register(IMPORT_WAIT_FOR_SINGLE_OBJECT, |_, _| ret(0));
        f.e.register(THREAD_FINISH, |_, _| ret(0));
        let thread = f.e.mem.alloc(0x4c);
        f.e.mem.set_u32(thread + 0x3c, 3);
        f.e.mem.set_u32(thread + 0x40, 0x55);
        f.e.set(f.this, BSAudioManager::pUpdateThread, thread);
        (f, thread)
    }

    #[test]
    fn suspend_audio_thread_queues_the_message_and_waits() {
        let (mut f, thread) = thread_fixture();
        f.e.call_log = Some(vec![]);
        f.e.call(0x00ada020, &args![f.this]);
        let log = f.e.call_log.take().unwrap();
        let addresses: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert!(addresses.starts_with(&[0x00ada020, THREAD_SEMAPHORE_SIGNAL]));
        assert!(addresses.ends_with(&[IMPORT_WAIT_FOR_SINGLE_OBJECT, IMPORT_INTERLOCKED_DECREMENT]));
        assert!(log.contains(&(IMPORT_WAIT_FOR_SINGLE_OBJECT, vec![0x55, 0xffff_ffff])));
        assert_eq!(f.messages(), vec![plain(0x35, 0, 0, 0)]);
        assert_eq!(f.e.mem.u32(thread + 0x3c), 2);
        assert!(!addresses.contains(&THREAD_FINISH));
    }

    #[test]
    fn stop_audio_thread_also_finishes_the_thread() {
        let (mut f, thread) = thread_fixture();
        f.e.call_log = Some(vec![]);
        f.e.call(0x00ada0f0, &args![f.this]);
        let log = f.e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap(), &(THREAD_FINISH, vec![thread]));
        assert_eq!(f.messages(), vec![plain(0x34, 0, 0, 0)]);
        assert_eq!(f.e.mem.u32(thread + 0x3c), 2);
        // Without a thread object only the message is queued.
        f.e.set(f.this, BSAudioManager::pUpdateThread, 0);
        f.e.call_log = Some(vec![]);
        f.e.call(0x00ada0f0, &args![f.this]);
        let log = f.e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == THREAD_FINISH));
        assert_eq!(f.messages().len(), 2);
    }

    #[test]
    fn update_precaches_once_then_syncs_and_processes() {
        let mut f = Fixture::new();
        let precached = Rc::new(RefCell::new(Vec::new()));
        let log = precached.clone();
        f.e.register_double(AUDIO_PRECACHE, move |_, a| {
            log.borrow_mut().push((a[1], a[2]));
            ret(0)
        });
        let processed = Rc::new(RefCell::new(0));
        let count = processed.clone();
        f.e.register_double(MANAGER_PROCESS, move |_, a| {
            assert_eq!(a[0], MANAGER_INSTANCE);
            *count.borrow_mut() += 1;
            ret(0)
        });
        f.e.mem.set_u32(MANAGER_BUILT_GUARD, 1);
        let audio = f.e.global::<u32>(AUDIO_GLOBAL);
        f.call(0x00ada1d0, &[0]);
        assert_eq!(precached.borrow().len(), 10);
        assert_eq!(precached.borrow()[0], (0x0106_f060, 0));
        assert_eq!(precached.borrow()[9], (0x010a_33e8, 0x101));
        assert!(f.e.get(f.this, BSAudioManager::bPrecacheCompleted));
        assert_eq!(*processed.borrow(), 1);
        // Second run: already precached; the audio thread runs: no Process.
        f.e.mem.set_u8(audio + 6, 1);
        f.call(0x00ada1d0, &[0]);
        assert_eq!(precached.borrow().len(), 10);
        assert_eq!(*processed.borrow(), 1);
    }

    #[test]
    fn weather_and_hour_are_stored() {
        let mut e = engine();
        let this: Ptr<BSAudioManager> = e.new_object();
        e.call(0x00ada300, &args![this, 13.5f32, 0x25u32]);
        assert_eq!(e.get(this, BSAudioManager::fCurrentGamehour), 13.5);
        assert_eq!(e.get(this, BSAudioManager::iWeatherFlags), 0x25);
    }

    #[test]
    fn synch_voice_states_moves_updates_into_the_state_map() {
        let mut f = Fixture::new();
        let destroyed = Rc::new(RefCell::new(Vec::new()));
        let log = destroyed.clone();
        f.e.register_double(SOUND_INFO_DESTRUCTOR, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(0)
        });
        let updates = f.this.addr() + 0x74;
        let states = f.this.addr() + 0x64;
        // The old record for sound 5, the new one for 5 and an update with
        // id 0 for sound 6.
        let old = f.e.mem.alloc(0x20);
        f.e.mem.set_u32(old, 5);
        fake_set_value(&mut f.e, &[states, 5, old]);
        let new = f.e.mem.alloc(0x20);
        f.e.mem.set_u32(new, 5);
        fake_set_value(&mut f.e, &[updates, 5, new]);
        let empty = f.e.mem.alloc(0x20);
        fake_set_value(&mut f.e, &[updates, 6, empty]);
        f.e.call_log = Some(vec![]);
        f.call(0x00ada330, &[]);
        let log = f.e.call_log.take().unwrap();
        assert_eq!(log.first().unwrap().0, 0x00ada330);
        assert_eq!(
            log[1],
            (IMPORT_ENTER_CRITICAL_SECTION, vec![f.this.addr() + 0xc4])
        );
        assert_eq!(
            log.last().unwrap(),
            &(IMPORT_LEAVE_CRITICAL_SECTION, vec![f.this.addr() + 0xc4])
        );
        assert_eq!(
            map_node(&f.e, states, 5).map(|n| f.e.mem.u32(n + 8)),
            Some(new)
        );
        assert_eq!(map_node(&f.e, states, 6), None);
        assert_eq!(f.e.mem.u32(updates + 0xc), 0);
        let mut gone = destroyed.borrow().clone();
        gone.sort_unstable();
        let mut expected = vec![old, empty];
        expected.sort_unstable();
        assert_eq!(gone, expected);
        // Nothing pending: only the critical section is entered and left.
        f.e.call_log = Some(vec![]);
        f.call(0x00ada330, &[]);
        assert_eq!(f.e.call_log.take().unwrap().len(), 3);
    }

    /// Doubles for the callees of `SetVoiceState` that build and copy
    /// records.
    fn install_record_doubles(e: &mut Engine) {
        e.register(SOUND_INFO_CONSTRUCTOR, |e, a| {
            e.mem.write(a[0], &[0; 0x18]);
            ret(a[0])
        });
        e.register(DEBUG_SOUND_INFO_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register(DEBUG_SOUND_INFO_DESTRUCTOR, |_, _| ret(0));
        e.register(SOUND_INFO_DESTRUCTOR, |_, _| ret(0));
        e.register(SOUND_INFO_ASSIGN, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x18);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        e.register(SOUND_INFO_COPY_CONSTRUCTOR, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x18);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        e.register(DEBUG_SOUND_INFO_ASSIGN, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x23c);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            ret(a[0])
        });
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            ret(0)
        });
    }

    /// A sound object with a vtable whose `GetVolume` answers `volume`,
    /// `IsPlaying` answers `playing` and `GetEmitterPosition` points at
    /// the position `(7, 8, 9)`.
    fn make_sound(e: &mut Engine, id: u32, volume: f32, playing: u8) -> u32 {
        let sound = e.mem.alloc(0x198);
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 7.0);
        e.mem.set_f32(position + 4, 8.0);
        e.mem.set_f32(position + 8, 9.0);
        let vtable = 0x0300_0100;
        if !e.mem.is_mapped(vtable) {
            e.mem.map(vtable, 0x100);
        }
        e.mem.set_u32(vtable + SOUND_SLOT_IS_PLAYING, 0x0300_0200);
        e.mem.set_u32(vtable + SOUND_SLOT_GET_VOLUME, 0x0300_0204);
        e.mem
            .set_u32(vtable + SOUND_SLOT_GET_EMITTER_POSITION, 0x0300_0208);
        e.mem.set_u32(sound, vtable);
        e.mem.set_u32(sound + SOUND_ID, id);
        e.mem.set_u32(sound + SOUND_TYPE_FLAGS, 0x44);
        e.mem.set_u32(sound + SOUND_DURATION, 1500);
        e.register_double(0x0300_0200, move |_, _| ret(playing as u32));
        e.register_double(0x0300_0204, move |_, _| Ret {
            st0: volume as f64,
            ..Ret::default()
        });
        e.register_double(0x0300_0208, move |_, _| ret(position));
        sound
    }

    #[test]
    fn set_voice_state_creates_the_record_for_a_new_sound() {
        let mut f = Fixture::new();
        install_record_doubles(&mut f.e);
        let sound = make_sound(&mut f.e, 9, 0.75, 1);
        let context = f.e.tls() + AUDIO_TLS_CONTEXT;
        f.e.mem.set_u32(context, 0x1234);
        f.e.call_log = Some(vec![]);
        f.call(0x00ada5d0, &[9, sound]);
        let log = f.e.call_log.take().unwrap();
        assert!(log.iter().any(|c| c.0 == SOUND_INFO_COPY_CONSTRUCTOR));
        let updates = f.this.addr() + 0x74;
        let record = map_node(&f.e, updates, 9)
            .map(|n| f.e.mem.u32(n + 8))
            .unwrap();
        assert_eq!(f.e.mem.u32(record), 9);
        assert_eq!(f.e.mem.f32(record + 4), 1.0);
        assert_eq!(f.e.mem.f32(record + 8), 0.75);
        assert_eq!(f.e.mem.u32(record + 0xc), 0x44);
        assert_eq!(f.e.mem.u32(record + 0x10), 1500);
        assert_eq!(f.e.mem.u8(record + 0x14), 1);
        // The audio context word is restored.
        let context = f.e.tls() + AUDIO_TLS_CONTEXT;
        assert_eq!(f.e.mem.u32(context), 0x1234);
        // The critical section brackets the map work.
        let positions: Vec<u32> = log.iter().map(|c| c.0).collect();
        let enter = positions
            .iter()
            .position(|a| *a == IMPORT_ENTER_CRITICAL_SECTION)
            .unwrap();
        let leave = positions
            .iter()
            .position(|a| *a == IMPORT_LEAVE_CRITICAL_SECTION)
            .unwrap();
        let set = positions.iter().position(|a| *a == MAP_SET_VALUE).unwrap();
        assert!(enter < set && set < leave);
    }

    #[test]
    fn set_voice_state_overwrites_an_existing_record() {
        let mut f = Fixture::new();
        install_record_doubles(&mut f.e);
        let sound = make_sound(&mut f.e, 9, 0.25, 0);
        let updates = f.this.addr() + 0x74;
        let existing = f.e.mem.alloc(0x254);
        f.e.mem.set_u32(existing + 0xc, 1);
        fake_set_value(&mut f.e, &[updates, 9, existing]);
        f.call(0x00ada5d0, &[9, sound]);
        assert_eq!(f.e.mem.u32(existing), 9);
        assert_eq!(f.e.mem.f32(existing + 8), 0.25);
        assert_eq!(f.e.mem.u32(existing + 0xc), 0x44);
        assert_eq!(f.e.mem.u8(existing + 0x14), 0);
        assert_eq!(f.e.mem.u32(updates + 0xc), 1);
    }

    #[test]
    fn set_voice_state_without_a_sound_stores_an_empty_record() {
        let mut f = Fixture::new();
        install_record_doubles(&mut f.e);
        f.call(0x00ada5d0, &[4, 0]);
        let updates = f.this.addr() + 0x74;
        let record = map_node(&f.e, updates, 4)
            .map(|n| f.e.mem.u32(n + 8))
            .unwrap();
        assert_eq!(f.e.mem.u32(record), 0);
        assert_eq!(f.e.mem.u32(record + 0xc), 0);
    }

    #[test]
    fn set_voice_state_fills_the_debug_record_when_enabled() {
        let mut f = Fixture::new();
        install_record_doubles(&mut f.e);
        f.e.mem.set_u8(AUDIO_DEBUG_INFORMATION_SETTING, 1);
        f.e.register(SOUND_IS_EMITTER_SOUND, |_, _| ret(0));
        f.e.register(SOUND_HAS_EMITTER, |_, _| ret(1));
        let copies = Rc::new(RefCell::new(Vec::new()));
        let log = copies.clone();
        f.e.register_double(DEBUG_SOUND_INFO_ASSIGN, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.bytes(a[1], 0x23c)));
            ret(a[0])
        });
        let sound = make_sound(&mut f.e, 9, 0.5, 1);
        f.e.mem
            .set_cstr(sound + SOUND_FILENAME, b"Sound\\fx\\boom.wav");
        f.e.mem.set_f32(sound + SOUND_DISTANCE_TO_LISTENER, 50.0);
        f.e.mem.set_f32(sound + SOUND_MIN_DISTANCE, 3.0e38);
        f.e.mem.set_f32(sound + SOUND_MAX_DISTANCE, 400.0);
        f.e.mem.set_u16(sound + SOUND_STATIC_ATTENUATION, 11);
        f.e.mem.set_u16(sound + SOUND_SYSTEM_ATTENUATION, 22);
        f.e.mem.set_u16(sound + SOUND_DISTANCE_ATTENUATION, 33);
        f.call(0x00ada5d0, &[9, sound]);
        let copies = copies.borrow();
        assert_eq!(copies.len(), 1);
        let record = &copies[0].1;
        let word =
            |offset: usize| u32::from_le_bytes(record[offset..offset + 4].try_into().unwrap());
        let float = |offset: usize| f32::from_bits(word(offset));
        assert_eq!(&record[0x10c..0x10c + 18], b"Sound\\fx\\boom.wav\0");
        assert_ne!(word(0x108), 0);
        assert_eq!(float(0x210), -1.0);
        assert_eq!(float(0x214), 400.0);
        assert_eq!(float(0x218), 7.0);
        assert_eq!(float(0x21c), 8.0);
        assert_eq!(float(0x220), 50.0);
        assert_eq!(word(0x230), 11);
        assert_eq!(word(0x234), 22);
        assert_eq!(word(0x238), 33);
    }

    #[test]
    fn set_voice_state_zeroes_the_distances_of_an_unpositioned_sound() {
        let mut f = Fixture::new();
        install_record_doubles(&mut f.e);
        f.e.mem.set_u8(AUDIO_DEBUG_INFORMATION_SETTING, 1);
        f.e.register(SOUND_IS_EMITTER_SOUND, |_, _| ret(0));
        f.e.register(SOUND_HAS_EMITTER, |_, _| ret(0));
        let copies = Rc::new(RefCell::new(Vec::new()));
        let log = copies.clone();
        f.e.register_double(DEBUG_SOUND_INFO_ASSIGN, move |e, a| {
            log.borrow_mut().push(e.mem.bytes(a[1], 0x23c));
            ret(a[0])
        });
        let sound = make_sound(&mut f.e, 9, 0.5, 1);
        f.e.mem.set_f32(sound + SOUND_DISTANCE_TO_LISTENER, 50.0);
        f.call(0x00ada5d0, &[9, sound]);
        let copies = copies.borrow();
        assert!(copies[0][0x210..0x224].iter().all(|b| *b == 0));
        // No file name: the text for a sound without one is not used while
        // the name exists; an empty name stays empty.
        assert_eq!(copies[0][0x10c], 0);
    }

    #[test]
    fn volume_scales_by_the_category_of_the_sound_type() {
        let mut e = engine();
        e.mem.set_f32(MASTER_VOLUME_MULT, 2.0);
        let this: Ptr<BSAudioManager> = e.new_object();
        let set = |e: &mut Engine, field: Field<BSAudioManager, f32>, value: f32| {
            e.set(this, field, value)
        };
        set(&mut e, BSAudioManager::fMasterVolume, 0.5);
        set(&mut e, BSAudioManager::fMasterSecondaryVolume, 0.5);
        set(&mut e, BSAudioManager::fFootVolume, 2.0);
        set(&mut e, BSAudioManager::fFootSecondaryVolume, 3.0);
        set(&mut e, BSAudioManager::fVoiceVolume, 4.0);
        set(&mut e, BSAudioManager::fVoiceSecondaryVolume, 0.5);
        set(&mut e, BSAudioManager::fEffectsVolume, 0.25);
        set(&mut e, BSAudioManager::fEffectsSecondaryVolume, 8.0);
        set(&mut e, BSAudioManager::fMusicVolume, 10.0);
        set(&mut e, BSAudioManager::fMusicSecondaryVolume, 0.5);
        set(&mut e, BSAudioManager::fRadioVolume, 6.0);
        set(&mut e, BSAudioManager::fRadioSecondaryVolume, 0.5);
        // Master part: 0.5 * 0.5 * 2.0 * 4.0 = 2.0.
        let volume =
            |e: &mut Engine, flags: u32| e.call(0x00adac30, &args![this, 4.0f32, flags]).f32();
        assert_eq!(volume(&mut e, 0), 2.0 * 2.0); // effects: 0.25 * 8
        assert_eq!(volume(&mut e, 8), 2.0 * 6.0); // foot: 2 * 3
        assert_eq!(volume(&mut e, 4), 2.0 * 2.0); // voice: 4 * 0.5
        assert_eq!(volume(&mut e, 0x800), 2.0 * 5.0); // music: 10 * 0.5
        assert_eq!(volume(&mut e, 0x10_0000), 2.0 * 3.0); // radio: 6 * 0.5
        assert_eq!(volume(&mut e, 0x80_0000), 2.0 * 10.0 * 0.5); // music, other order
                                                                 // The flag 0x800000 wins over the others, 8 over 4.
        assert_eq!(volume(&mut e, 0x80_0008), 10.0);
        assert_eq!(volume(&mut e, 0xc), 2.0 * 6.0);
    }

    #[test]
    fn the_message_pools_pop_their_own_stacks() {
        let mut e = engine();
        let this: Ptr<BSAudioManager> = e.new_object();
        e.register(MESSAGE_STACK_POP, |_, a| ret(a[0] + 1));
        assert_eq!(e.call(0x00adad60, &args![this]).u32(), this.addr() + 0x25);
        assert_eq!(e.call(0x00adaf80, &args![this]).u32(), this.addr() + 0x35);
    }

    /// A message holding a reference-counted object whose count is 2.
    fn message_with_object(e: &mut Engine) -> (u32, u32) {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object + 4, 2);
        let message = e.mem.alloc(0x20);
        e.mem.set_u32(message, 0x13);
        e.mem.set_u32(message + 4, 5);
        e.mem.set_u32(message + 8, 6);
        e.mem.set_u32(message + 0xc, 7);
        e.mem.set_u32(message + 0x10, object);
        (message, object)
    }

    /// The `(stack, message)` pairs pushed so far.
    type PushLog = Rc<RefCell<Vec<(u32, u32)>>>;

    fn free_message_fixture() -> (Engine, Ptr<BSAudioManager>, PushLog) {
        let mut e = engine();
        e.register(SMART_POINTER_ASSIGN, fake_pointer_assign);
        e.register(SMART_POINTER_DESTRUCTOR, |_, _| ret(0));
        e.register(RELEASE_MESSAGE_POINTER, |_, _| ret(0));
        e.register(SMART_POINTER_SET, |e, a| {
            // Sets the slot to the pointer, dropping the old reference.
            let old = e.mem.u32(a[0]);
            if old != 0 {
                let count = e.mem.u32(old + 4);
                e.mem.set_u32(old + 4, count - 1);
            }
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        let pushes = Rc::new(RefCell::new(Vec::new()));
        let log = pushes.clone();
        e.register_double(MESSAGE_STACK_PUSH, move |_, a| {
            log.borrow_mut().push((a[0], a[1]));
            ret(0)
        });
        let this: Ptr<BSAudioManager> = e.new_object();
        (e, this, pushes)
    }

    #[test]
    fn free_message_clears_the_message_and_pushes_it() {
        let (mut e, this, pushes) = free_message_fixture();
        let (message, object) = message_with_object(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00adad80, &args![this, message]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(RELEASE_MESSAGE_POINTER, vec![object])));
        assert_eq!(*pushes.borrow(), vec![(this.addr() + 0x24, message)]);
        assert_eq!(e.mem.u32(message), 0);
        assert_eq!(e.mem.u32(message + 4), 0);
        assert_eq!(e.mem.u32(message + 8), 0);
        assert_eq!(e.mem.u32(message + 0xc), 0);
        assert_eq!(e.mem.u32(message + 0x10), 0);
        assert_eq!(e.mem.f32(message + 0x14), 1.0);
        assert_eq!(e.mem.f32(message + 0x18), 2.0);
        assert_eq!(e.mem.f32(message + 0x1c), 3.0);
        // The reference count: 2 + 1 (the temporary) - 1 (the message let
        // go) - 1 (the temporary released) = 1.
        assert_eq!(e.mem.u32(object + 4), 1);
    }

    #[test]
    fn free_message_without_an_object_skips_the_release() {
        let (mut e, this, pushes) = free_message_fixture();
        let message = e.mem.alloc(0x20);
        e.mem.set_u32(message, 0x13);
        e.call_log = Some(vec![]);
        e.call(0x00adad80, &args![this, message]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == RELEASE_MESSAGE_POINTER));
        assert_eq!(*pushes.borrow(), vec![(this.addr() + 0x24, message)]);
        assert_eq!(e.mem.u32(message), 0);
    }

    #[test]
    fn free_queue_message_pushes_on_the_queue_stack() {
        let (mut e, this, pushes) = free_message_fixture();
        let (message, object) = message_with_object(&mut e);
        e.call(0x00adafa0, &args![this, message]);
        assert_eq!(*pushes.borrow(), vec![(this.addr() + 0x34, message)]);
        assert_eq!(e.mem.u32(message + 0x10), 0);
        assert_eq!(e.mem.u32(object + 4), 1);
    }

    #[test]
    fn queue_message_fills_a_message_and_appends_it() {
        let mut f = Fixture::new();
        f.e.set(f.this, BSAudioManager::iMessageListIndex, 1);
        let context = f.e.tls() + AUDIO_TLS_CONTEXT;
        f.e.mem.set_u32(context, 9);
        f.e.call_log = Some(vec![]);
        f.call(
            0x00adb1a0,
            &[
                0x31,
                1,
                2,
                3,
                0,
                4.5f32.to_bits(),
                5.5f32.to_bits(),
                6.5f32.to_bits(),
            ],
        );
        let log = f.e.call_log.take().unwrap();
        let append = log.iter().find(|c| c.0 == LIST_APPEND_POINTER).unwrap();
        assert_eq!(append.1[0], f.this.addr() + 0x0c + 0x0c);
        assert_eq!(
            f.messages(),
            vec![Message {
                kind: 0x31,
                a: 1,
                b: 2,
                c: 3,
                pointer: 0,
                vector: [4.5, 5.5, 6.5],
            }]
        );
        let context = f.e.tls() + AUDIO_TLS_CONTEXT;
        assert_eq!(f.e.mem.u32(context), 9);
        let enters = log
            .iter()
            .filter(|c| **c == (IMPORT_ENTER_CRITICAL_SECTION, vec![f.this.addr() + 0xac]))
            .count();
        assert_eq!(enters, 1);
    }

    #[test]
    fn queue_message_does_nothing_while_the_audio_is_not_initialized() {
        let mut f = Fixture::new();
        let audio = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem.set_u8(audio + 5, 0);
        let object = f.e.mem.alloc(8);
        f.e.mem.set_u32(object + 4, 2);
        f.e.mem.set_u32(object, 0x0300_0300);
        f.call(0x00adb1a0, &[0x31, 1, 2, 3, object, 0, 0, 0]);
        assert!(f.messages().is_empty());
        // The argument's reference is still dropped.
        assert_eq!(f.e.mem.u32(object + 4), 1);
    }

    #[test]
    fn queue_message_keeps_the_reference_the_message_takes() {
        let mut f = Fixture::new();
        let object = f.e.mem.alloc(8);
        f.e.mem.set_u32(object + 4, 2);
        f.call(0x00adb1a0, &[0x31, 1, 2, 3, object, 0, 0, 0]);
        assert_eq!(f.messages()[0].pointer, object);
        // 2 (caller and argument) + 1 (temporary) + 1 (message) - 1 - 1 = 2.
        assert_eq!(f.e.mem.u32(object + 4), 2);
    }

    #[test]
    fn queue_message_with_an_empty_pool_appends_nothing() {
        let mut f = Fixture::new();
        f.e.register(MESSAGE_STACK_POP, |_, _| ret(0));
        f.e.call_log = Some(vec![]);
        f.call(0x00adb1a0, &[0x31, 1, 2, 3, 0, 0, 0, 0]);
        let log = f.e.call_log.take().unwrap();
        assert!(f.messages().is_empty());
        assert_eq!(
            log.last().unwrap(),
            &(IMPORT_LEAVE_CRITICAL_SECTION, vec![f.this.addr() + 0xac])
        );
    }

    #[test]
    fn play_queues_the_play_messages_unless_playing() {
        let mut f = Fixture::new();
        assert!(f.call(0x00adb3e0, &[7, 1]).bool());
        assert_eq!(f.messages(), vec![plain(0x2d, 7, 0, 0), plain(1, 7, 0, 0)]);
        f.queued.borrow_mut().clear();
        assert!(f.call(0x00adb3e0, &[7, 0]).bool());
        assert_eq!(f.messages(), vec![plain(1, 7, 0, 0)]);
        f.queued.borrow_mut().clear();
        f.add_state(7, 0.0, 0, 0, 1);
        assert!(f.call(0x00adb3e0, &[7, 1]).bool());
        assert!(f.messages().is_empty());
    }

    #[test]
    fn play_after_adds_the_delay_to_the_tick_count() {
        let mut f = Fixture::new();
        f.e.clock_ms = 100_000;
        assert!(f.call(0x00adb4e0, &[7, 250, 0]).bool());
        assert!(f.call(0x00adb4e0, &[8, 99_999, 0]).bool());
        assert_eq!(
            f.messages(),
            vec![plain(5, 7, 100_250, 0), plain(5, 8, 110_000, 0)]
        );
        f.queued.borrow_mut().clear();
        f.add_state(7, 0.0, 0, 0, 1);
        assert!(f.call(0x00adb4e0, &[7, 250, 0]).bool());
        assert!(f.messages().is_empty());
    }

    #[test]
    fn pause_and_stop_queue_their_messages() {
        let mut f = Fixture::new();
        assert!(!f.call(0x00adb590, &[3]).bool());
        assert!(!f.call(0x00adb610, &[3]).bool());
        assert_eq!(f.messages(), vec![plain(3, 3, 0, 0), plain(4, 3, 0, 0)]);
    }

    #[test]
    fn stop_moving_sounds_passes_the_object_and_the_milliseconds() {
        let mut f = Fixture::new();
        install_ftol(&mut f.e);
        let object = f.e.mem.alloc(8);
        f.e.mem.set_u32(object + 4, 1);
        f.call(0x00adb690, &[object, 2.5f32.to_bits(), 77]);
        assert_eq!(
            f.messages(),
            vec![Message {
                kind: 0x29,
                a: 0,
                b: 2500,
                c: 77,
                pointer: object,
                vector: [1.0, 2.0, 3.0],
            }]
        );
        // 1 (caller) + 1 (argument) + 1 (temporary) + 1 (message) - 1 - 1.
        assert_eq!(f.e.mem.u32(object + 4), 2);
    }

    #[test]
    fn static_attenuation_is_limited_to_10000() {
        let mut f = Fixture::new();
        assert!(!f.call(0x00adb720, &[7, 500]).bool());
        assert!(!f.call(0x00adb720, &[7, 20000]).bool());
        assert_eq!(
            f.messages(),
            vec![plain(0x21, 7, 500, 0), plain(0x21, 7, 10000, 0)]
        );
    }

    /// `_ftol2` as a double: truncates the leading `f64`.
    fn install_ftol(e: &mut Engine) {
        e.register(FTOL2, |_, a| {
            let value = f64::from_bits((a[0] as u64) | ((a[1] as u64) << 32));
            ret(value as i32 as u32)
        });
    }

    #[test]
    fn the_millisecond_wrappers_scale_through_ftol() {
        let mut f = Fixture::new();
        install_ftol(&mut f.e);
        assert!(!f.call(0x00adb7c0, &[7, 0.5f32.to_bits()]).bool());
        assert!(!f.call(0x00adb850, &[7, 1.25f32.to_bits()]).bool());
        assert!(f.call(0x00adc0d0, &[7, 0.75f32.to_bits()]).bool());
        assert_eq!(
            f.messages(),
            vec![
                plain(0x13, 7, 500, 0),
                plain(0x42, 7, 1250, 0),
                plain(0x10, 7, 750, 0)
            ]
        );
    }

    #[test]
    fn byte_and_word_wrappers_queue_their_values() {
        let mut f = Fixture::new();
        assert!(!f.call(0x00adb8e0, &[7, 0x1ff]).bool());
        f.call(0x00adbec0, &[7, 0xffff_1234]);
        f.call(0x00adc260, &[7, 0xdead]);
        f.call(0x00adc2e0, &[7, 0x1ab]);
        f.call(0x00adc4e0, &[7, 0x1cd, 0x2ef]);
        assert_eq!(
            f.messages(),
            vec![
                plain(0x14, 7, 0xff, 0),
                plain(0x22, 7, 0x1234, 0),
                plain(0x3f, 7, 0xdead, 0),
                plain(0x40, 7, 0xab, 0),
                plain(0x3e, 7, 0xcd, 0xef),
            ]
        );
    }

    #[test]
    fn the_empty_wrappers_return_false() {
        let mut f = Fixture::new();
        assert!(!f.call(0x00adb960, &[1, 2]).bool());
        assert!(!f.call(0x00adbf40, &[1, 2, 3, 4]).bool());
        assert!(f.messages().is_empty());
    }

    #[test]
    fn position_wrappers_put_the_coordinates_in_the_vector() {
        let mut f = Fixture::new();
        assert!(!f
            .call(
                0x00adb970,
                &[7, 10.0f32.to_bits(), 20.0f32.to_bits(), 30.0f32.to_bits()]
            )
            .bool());
        assert!(!f
            .call(
                0x00adb9f0,
                &[7, 1.5f32.to_bits(), 2.5f32.to_bits(), 3.5f32.to_bits()]
            )
            .bool());
        let message = |kind, vector| Message {
            kind,
            a: 7,
            b: 0,
            c: 0,
            pointer: 0,
            vector,
        };
        assert_eq!(
            f.messages(),
            vec![
                message(0x1e, [10.0, 20.0, 30.0]),
                message(0x1f, [1.5, 2.5, 3.5])
            ]
        );
    }

    #[test]
    fn set_min_max_truncates_the_scaled_distances() {
        let mut f = Fixture::new();
        f.call(0x00adba70, &[7, 1.5f32.to_bits(), 2.0004f32.to_bits()]);
        f.call(0x00adba70, &[7, (-1.5f32).to_bits(), f32::NAN.to_bits()]);
        assert_eq!(
            f.messages(),
            vec![
                plain(0x23, 7, 1500, 2000),
                plain(0x23, 7, (-1500i32) as u32, 0),
            ]
        );
    }

    #[test]
    fn five_values_are_limited_to_100_and_overwrite_the_vector() {
        let mut f = Fixture::new();
        f.e.register(MESSAGE_ASSIGN, |e, a| {
            // Copies the first 0x14 bytes (the type, id and words).
            let bytes = e.mem.bytes(a[1], 0x14);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        f.e.register(MESSAGE_DESTRUCTOR, |_, _| ret(0));
        f.e.register(SMART_POINTER_DESTRUCTOR, |_, _| ret(0));
        let context = f.e.tls() + AUDIO_TLS_CONTEXT;
        f.e.mem.set_u32(context, 5);
        f.call(0x00adbb30, &[7, 1, 100, 101, 65535, 99]);
        let message = f.queued.borrow()[0];
        assert_eq!(f.e.mem.u32(message), 0x24);
        assert_eq!(f.e.mem.u32(message + 4), 7);
        let curve: Vec<u16> = (0..5)
            .map(|i| f.e.mem.u16(message + 0x14 + 2 * i))
            .collect();
        assert_eq!(curve, vec![1, 100, 100, 100, 99]);
        let context = f.e.tls() + AUDIO_TLS_CONTEXT;
        assert_eq!(f.e.mem.u32(context), 5);
        // Audio not initialized: nothing happens.
        let audio = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem.set_u8(audio + 5, 0);
        f.call(0x00adbb30, &[7, 1, 2, 3, 4, 5]);
        assert_eq!(f.queued.borrow().len(), 1);
    }

    #[test]
    fn release_and_the_completion_callbacks() {
        let mut f = Fixture::new();
        assert!(f.call(0x00adbf50, &[7]).bool());
        assert!(f.call(0x00adbfd0, &[7, 0x111, 0x222]).bool());
        assert!(f.call(0x00adc050, &[7, 0x333, 0x444]).bool());
        assert_eq!(
            f.messages(),
            vec![
                plain(0x26, 7, 0, 0),
                plain(0x3a, 7, 0x222, 0x111),
                plain(0x3b, 7, 0x444, 0x333),
            ]
        );
    }

    #[test]
    fn volume_changed_and_priority_and_synch_time() {
        let mut f = Fixture::new();
        f.call(0x00adc160, &[]);
        f.call(0x00adc360, &[7, 4000]);
        f.call(0x00adc3e0, &[7]);
        f.call(0x00adc460, &[7, 3]);
        assert_eq!(
            f.messages(),
            vec![
                plain(0x0e, 0, 0, 0),
                plain(0x3c, 7, 4000, 4000),
                plain(0x3d, 7, 0, 0),
                plain(0x41, 7, 3, 0),
            ]
        );
    }

    #[test]
    fn set_object_to_follow_passes_the_object() {
        let mut f = Fixture::new();
        let object = f.e.mem.alloc(8);
        f.e.mem.set_u32(object + 4, 1);
        f.call(0x00adc1e0, &[7, object]);
        assert_eq!(
            f.messages(),
            vec![Message {
                kind: 0x27,
                a: 7,
                b: 0,
                c: 0,
                pointer: object,
                vector: [1.0, 2.0, 3.0],
            }]
        );
        assert_eq!(f.e.mem.u32(object + 4), 2);
        // A null object is queued without any reference counting.
        f.call(0x00adc1e0, &[8, 0]);
        assert_eq!(f.messages()[1].pointer, 0);
    }

    #[test]
    fn fade_out_puts_the_time_in_the_vector() {
        let mut f = Fixture::new();
        f.call(0x00adc560, &[7, 0x55, 3000]);
        assert_eq!(
            f.messages(),
            vec![Message {
                kind: 9,
                a: 7,
                b: 0,
                c: 0x55,
                pointer: 0,
                vector: [3000.0, 0.0, 0.0],
            }]
        );
    }

    #[test]
    fn word_message_and_the_conditional_play_sequence() {
        let mut f = Fixture::new();
        f.call(0x00adc5f0, &[7, 0x99, 0x1234]);
        assert_eq!(f.messages(), vec![plain(0x0b, 7, 0, 0x99)]);
        f.queued.borrow_mut().clear();
        f.call(0x00adc670, &[7, 0x66]);
        assert_eq!(
            f.messages(),
            vec![
                plain(7, 7, 0, 0),
                plain(1, 7, 0, 0),
                plain(0x0b, 7, 0, 0x66)
            ]
        );
        f.queued.borrow_mut().clear();
        f.add_state(7, 0.0, 0, 0, 1);
        f.call(0x00adc670, &[7, 0x66]);
        assert!(f.messages().is_empty());
    }

    #[test]
    fn state_queries_read_the_record() {
        let mut f = Fixture::new();
        f.add_state(7, 0.75, 0x1234, 4500, 1);
        f.add_state(8, 0.5, 0, 0, 0);
        // An entry whose record id differs from its key is not valid.
        let stray = f.e.mem.alloc(0x20);
        f.e.mem.set_u32(stray, 3);
        let states = f.this.addr() + 0x64;
        fake_set_value(&mut f.e, &[states, 9, stray]);
        assert!(f.call(0x00adc7d0, &[7]).bool());
        assert!(f.call(0x00adc7d0, &[8]).bool());
        assert!(!f.call(0x00adc7d0, &[9]).bool());
        assert!(!f.call(0x00adc7d0, &[10]).bool());
        assert_eq!(f.call(0x00adc830, &[7]).u32(), 4500);
        assert_eq!(f.call(0x00adc830, &[10]).u32(), 0);
        assert_eq!(f.call(0x00adc8b0, &[7]).f32(), 0.75);
        assert_eq!(f.call(0x00adc8b0, &[10]).f32(), 0.0);
        assert!(f.call(0x00adc910, &[7]).bool());
        assert!(!f.call(0x00adc910, &[8]).bool());
        assert!(!f.call(0x00adc910, &[10]).bool());
        assert_eq!(f.call(0x00adc9a0, &[7]).u32(), 0x1234);
        assert_eq!(f.call(0x00adc9a0, &[10]).u32(), 0);
    }

    #[test]
    fn the_queries_hold_the_state_critical_section() {
        let mut f = Fixture::new();
        f.add_state(7, 0.75, 0x1234, 4500, 1);
        f.e.call_log = Some(vec![]);
        f.call(0x00adc830, &[7]);
        let log = f.e.call_log.take().unwrap();
        let section = f.this.addr() + 0xc4;
        assert_eq!(log[1], (IMPORT_ENTER_CRITICAL_SECTION, vec![section]));
        assert_eq!(log[3], (IMPORT_LEAVE_CRITICAL_SECTION, vec![section]));
        // IsValid reads the map without the critical section.
        f.e.call_log = Some(vec![]);
        f.call(0x00adc7d0, &[7]);
        let log = f.e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == IMPORT_ENTER_CRITICAL_SECTION));
    }

    #[test]
    fn type_mask_wrappers_queue_their_messages() {
        let mut f = Fixture::new();
        f.call(0x00adca20, &[0x11, 0x22, 0x33]);
        f.call(0x00adcaa0, &[0x44, 0x55]);
        f.call(0x00adcb20, &[0x66, 1]);
        f.call(0x00adcb20, &[0x67, 0]);
        f.call(0x00adcc10, &[0x77, 1]);
        f.call(0x00adcc10, &[0x78, 0]);
        f.call(0x00adcd00, &[0x88, 0x99]);
        assert_eq!(
            f.messages(),
            vec![
                plain(0x18, 0x33, 0x11, 0x22),
                plain(0x19, 0, 0x44, 0x55),
                plain(0x1c, 0, 0x66, 0),
                plain(0x1a, 0, 0x67, 0),
                plain(0x1d, 0, 0x77, 0),
                plain(0x1b, 0, 0x78, 0),
                plain(0x2a, 0, 0x88, 0x99),
            ]
        );
    }

    #[test]
    fn shutdown_empties_everything() {
        let mut f = Fixture::new();
        install_record_doubles(&mut f.e);
        f.e.register(LIST_REMOVE_HEAD, fake_list_remove_head);
        f.e.register(MESSAGE_DESTRUCTOR, |_, _| ret(0));
        f.e.register(MANAGER_CACHE_FLUSH, |e, a| {
            // The flag byte is clear while the cache is flushed.
            assert_eq!(a[1], 0);
            assert_eq!(e.mem.u8(CACHE_FLUSH_FLAG), 0);
            ret(0)
        });
        f.e.mem.set_u8(CACHE_FLUSH_FLAG, 1);
        let audio = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem.set_u8(audio + 6, 1);
        // Two messages in the first list, one in the second.
        let messages: Vec<u32> = (0..3).map(|_| f.e.mem.alloc(0x20)).collect();
        list_push(&mut f.e, f.this.addr() + 0x0c, messages[0]);
        list_push(&mut f.e, f.this.addr() + 0x0c, messages[1]);
        list_push(&mut f.e, f.this.addr() + 0x18, messages[2]);
        // The audio thread object: a vtable whose first slot records the
        // deleting-destructor call.
        let deletions = Rc::new(RefCell::new(Vec::new()));
        let log = deletions.clone();
        let vtable = 0x0300_0400;
        f.e.mem.map(vtable, 0x10);
        f.e.mem.set_u32(vtable, 0x0300_0410);
        f.e.register_double(0x0300_0410, move |_, a| {
            log.borrow_mut().push((a[0], a[1]));
            ret(0)
        });
        let thread = f.e.mem.alloc(0x4c);
        f.e.mem.set_u32(thread, vtable);
        f.e.set(f.this, BSAudioManager::pUpdateThread, thread);
        // Two sounds (their deleting destructor is the same slot).
        let sounds = f.this.addr() + 0x54;
        let first = f.e.mem.alloc(0x198);
        f.e.mem.set_u32(first, vtable);
        f.e.mem.set_u32(first + SOUND_ID, 3);
        let second = f.e.mem.alloc(0x198);
        f.e.mem.set_u32(second, vtable);
        f.e.mem.set_u32(second + SOUND_ID, 4);
        fake_set_value(&mut f.e, &[sounds, 3, first]);
        fake_set_value(&mut f.e, &[sounds, 4, second]);
        // A pending update and the state it replaces.
        let updates = f.this.addr() + 0x74;
        let states = f.this.addr() + 0x64;
        let pending = f.e.mem.alloc(0x20);
        fake_set_value(&mut f.e, &[updates, 20, pending]);
        let stale = f.e.mem.alloc(0x20);
        fake_set_value(&mut f.e, &[states, 20, stale]);
        let pause_entries = f.e.mem.alloc(64);
        f.e.set(f.this, BSAudioManager::pPauseEntries, pause_entries);
        let destroyed = Rc::new(RefCell::new(Vec::new()));
        let log = destroyed.clone();
        f.e.register_double(SOUND_INFO_DESTRUCTOR, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(0)
        });
        f.e.call_log = Some(vec![]);
        f.call(0x00ad9870, &[]);
        let log = f.e.call_log.take().unwrap();
        let removed: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == LIST_REMOVE_HEAD)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(
            removed,
            vec![
                f.this.addr() + 0x0c,
                f.this.addr() + 0x0c,
                f.this.addr() + 0x18
            ]
        );
        assert_eq!(f.e.mem.u32(f.this.addr() + 0x14), 0);
        assert_eq!(f.e.mem.u32(f.this.addr() + 0x20), 0);
        // The audio thread was deleted and forgotten.
        assert_eq!(f.e.get(f.this, BSAudioManager::pUpdateThread), 0);
        let deletions = deletions.borrow();
        assert!(deletions.contains(&(thread, 1)));
        assert!(deletions.contains(&(first, 1)));
        assert!(deletions.contains(&(second, 1)));
        // Sound removed from the list; the state update replaced the stale
        // record, which was deleted along with the pending one; both maps
        // end up empty of pending work.
        assert_eq!(f.e.mem.u32(sounds + 0xc), 0);
        assert_eq!(f.e.mem.u32(updates + 0xc), 0);
        let mut gone = destroyed.borrow().clone();
        gone.sort_unstable();
        assert!(gone.contains(&stale));
        assert!(gone.contains(&pending));
        // The moving-sound map was cleared and the pause list freed.
        assert!(log.contains(&(MAP_CLEAR, vec![f.this.addr() + 0x84])));
        assert!(log.contains(&(DEALLOCATE, vec![MEMORY_MANAGER, pause_entries])));
        assert!(log.contains(&(MANAGER_CACHE_FLUSH, vec![f.this.addr(), 0])));
        assert_eq!(f.e.mem.u8(CACHE_FLUSH_FLAG), 1);
    }

    #[test]
    fn shutdown_updates_the_audio_first_when_not_multithreaded() {
        let mut f = Fixture::new();
        install_record_doubles(&mut f.e);
        f.e.register(MANAGER_CACHE_FLUSH, |_, _| ret(0));
        f.e.register(LIST_REMOVE_HEAD, fake_list_remove_head);
        f.e.mem.set_u32(MANAGER_BUILT_GUARD, 1);
        f.e.register(MANAGER_PROCESS, |_, _| ret(0));
        f.e.register(AUDIO_PRECACHE, |_, _| ret(0));
        f.e.call_log = Some(vec![]);
        f.call(0x00ad9870, &[]);
        let log = f.e.call_log.take().unwrap();
        // Three `Update` runs, each ending with `Process`.
        assert_eq!(log.iter().filter(|c| c.0 == MANAGER_PROCESS).count(), 3);
    }

    // ---- the second session's tests: cache, sound updates, messages ----

    /// Calls made through a fake vtable: (slot offset, this, arguments).
    type VtableLog = Rc<RefCell<Vec<(u32, u32, Vec<u32>)>>>;
    /// What the fake vtable slots answer, by slot offset.
    type VtableReturns = Rc<RefCell<std::collections::HashMap<u32, Ret>>>;

    /// A fake vtable of 0x80 bytes whose slots log their calls and answer
    /// from `returns` (default: 0).
    struct FakeVtable {
        address: u32,
        log: VtableLog,
        returns: VtableReturns,
    }

    impl FakeVtable {
        /// The slots called with `slot` (`this` first, then the arguments).
        fn calls(&self, slot: u32) -> Vec<Vec<u32>> {
            self.log
                .borrow()
                .iter()
                .filter(|c| c.0 == slot)
                .map(|c| {
                    let mut words = vec![c.1];
                    words.extend_from_slice(&c.2);
                    words
                })
                .collect()
        }

        fn answer(&self, slot: u32, ret: Ret) {
            self.returns.borrow_mut().insert(slot, ret);
        }
    }

    /// Builds a [`FakeVtable`] at `address` (code addresses `0x7300_0000 +
    /// address low bits + slot` hold the slot doubles).
    fn fake_vtable(e: &mut Engine, address: u32) -> FakeVtable {
        let log: VtableLog = Rc::new(RefCell::new(Vec::new()));
        let returns: VtableReturns = Rc::new(RefCell::new(Default::default()));
        let mut slots = Vec::new();
        let code = 0x7300_0000 + (address & 0xfff_ffff);
        for slot in (0..0x80).step_by(4) {
            let target = code + slot;
            slots.push(target);
            let log = log.clone();
            let returns = returns.clone();
            e.register_double(target, move |_, a| {
                log.borrow_mut().push((slot, a[0], a[1..].to_vec()));
                returns.borrow().get(&slot).copied().unwrap_or_default()
            });
        }
        e.put_vtable(address, &slots);
        FakeVtable {
            address,
            log,
            returns,
        }
    }

    /// Adds the imports the second half of the unit uses and the extra pages
    /// of exe data to `e`.
    fn extend_engine(e: &mut Engine) {
        let imports = [
            (IMPORT_TRY_ENTER_CRITICAL_SECTION, "TryEnterCriticalSection"),
            (IMPORT_GET_CURRENT_THREAD_ID, "GetCurrentThreadId"),
            (IMPORT_SLEEP, "Sleep"),
        ]
        .map(|(slot, name)| crate::exe::Import {
            slot,
            dll: "KERNEL32.dll".into(),
            name: name.into(),
        });
        assert_eq!(e.register_imports(&imports), 3);
        for page in [
            0x0101_1000,
            0x0101_3000,
            0x0101_6000,
            0x0101_a000,
            0x0102_9000,
            0x0103_4000,
            0x0106_3000,
            0x0108_2000,
            0x0108_8000,
            0x011a_c000,
            0x011c_4000,
            0x0120_2000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f64(0x0102_90b0, 0.25);
        e.mem.set_f32(0x0101_622c, 0.25);
        e.mem.set_f32(TIME_MULTIPLIER, 1.0);
        e.mem.set_f64(RANGE_MARGIN, 1.05);
        e.mem.set_f64(FREQUENCY_TOLERANCE, 0.01);
        e.mem.set_f64(EMITTER_HEIGHT_OFFSET, 200.0);
        e.mem.set_f64(PERCENT_SCALE, 100.0);
        e.mem.set_f64(TWO_DOUBLE, 2.0);
        e.mem.set_f64(TWO_POWER_32_DOUBLE, 4_294_967_296.0);
        e.mem.set_f64(FADE_ATTENUATION_SCALE, 3500.0);
        e.mem.set_f64(TIME_CONSTRAINT_SCALE, 0.09375);
        e.mem.set_f32(FADE_MESSAGE_X, 38.0);
        e.mem.set_f32(DIALOGUE_FADE_DECIBELS, 6.0);
        e.mem.set_f32(DIALOGUE_FADE_SECONDS_IN, 2.0);
        e.mem.set_f32(DIALOGUE_FADE_SECONDS_OUT, 1.0);
        e.mem.set_u32(AUDIO_CACHE_SIZE_SETTING, 0x800);
        e.mem.set_u32(MAX_CACHED_SOUND_SIZE_SETTING, 0x100);
        e.mem.set_u8(CACHE_ENABLED_SETTING, 1);
    }

    fn engine2() -> Engine {
        let mut e = engine();
        extend_engine(&mut e);
        e
    }

    /// The fixture of the message cases: a manager with the extended engine,
    /// the record doubles and a `BSAudio` object with a fake vtable.
    fn audio_fixture() -> (Fixture, FakeVtable) {
        let mut f = Fixture::new();
        extend_engine(&mut f.e);
        install_record_doubles(&mut f.e);
        let audio = f.e.global::<u32>(AUDIO_GLOBAL);
        let vtable = fake_vtable(&mut f.e, 0x7200_0000);
        f.e.mem.set_u32(audio, vtable.address);
        (f, vtable)
    }

    /// A zeroed 0x198-byte sound with the fake vtable.
    fn fake_sound(e: &mut Engine, vtable: &FakeVtable, id: u32) -> u32 {
        let sound = e.mem.alloc(0x198);
        e.mem.set_u32(sound, vtable.address);
        e.mem.set_u32(sound + SOUND_ID, id);
        sound
    }

    /// Puts `sound` into the manager's sound list (+0x54) under `id`.
    fn list_sound(f: &mut Fixture, id: u32, sound: u32) {
        let map = f.this.addr() + 0x54;
        fake_set_value(&mut f.e, &[map, id, sound]);
    }

    /// A message with the given words and vector.
    fn fake_message(e: &mut Engine, words: [u32; 4], vector: [f32; 3]) -> Ptr<BSSoundMessage> {
        let message: Ptr<BSSoundMessage> = e.new_object();
        for (i, w) in words.into_iter().enumerate() {
            e.mem.set_u32(message.addr() + 4 * i as u32, w);
        }
        for (i, v) in vector.into_iter().enumerate() {
            e.mem.set_f32(message.addr() + 0x14 + 4 * i as u32, v);
        }
        message
    }

    // ---- pointer-list models ----

    /// The elements of a pointer list, head first.
    fn list_elements(e: &Engine, list: u32) -> Vec<u32> {
        let mut result = Vec::new();
        let mut node = e.mem.u32(list);
        while node != 0 {
            result.push(e.mem.u32(node + 8));
            node = e.mem.u32(node);
        }
        result
    }

    /// `AddHead` (`00a0c9f0`): `(list, &element)`.
    fn fake_add_head(e: &mut Engine, a: &[u32]) -> Ret {
        let element = e.mem.u32(a[1]);
        let node = e.mem.alloc(12);
        let head = e.mem.u32(a[0]);
        e.mem.set_u32(node, head);
        e.mem.set_u32(node + 8, element);
        if head == 0 {
            e.mem.set_u32(a[0] + 4, node);
        } else {
            e.mem.set_u32(head + 4, node);
        }
        e.mem.set_u32(a[0], node);
        let count = e.mem.u32(a[0] + 8);
        e.mem.set_u32(a[0] + 8, count + 1);
        ret(0)
    }

    /// `RemoveTail` (`00ae8010`): `(list)`.
    fn fake_pop_tail(e: &mut Engine, a: &[u32]) -> Ret {
        let node = e.mem.u32(a[0] + 4);
        let previous = e.mem.u32(node + 4);
        e.mem.set_u32(a[0] + 4, previous);
        if previous == 0 {
            e.mem.set_u32(a[0], 0);
        } else {
            e.mem.set_u32(previous, 0);
        }
        let count = e.mem.u32(a[0] + 8);
        e.mem.set_u32(a[0] + 8, count - 1);
        ret(e.mem.u32(node + 8))
    }

    /// `00ae8090`: `(list, &node)`; removes the node and moves the slot to
    /// the next one.
    fn fake_remove_at(e: &mut Engine, a: &[u32]) -> Ret {
        let node = e.mem.u32(a[1]);
        let next = e.mem.u32(node);
        let previous = e.mem.u32(node + 4);
        if previous == 0 {
            e.mem.set_u32(a[0], next);
        } else {
            e.mem.set_u32(previous, next);
        }
        if next == 0 {
            e.mem.set_u32(a[0] + 4, previous);
        } else {
            e.mem.set_u32(next + 4, previous);
        }
        e.mem.set_u32(a[1], next);
        let count = e.mem.u32(a[0] + 8);
        e.mem.set_u32(a[0] + 8, count - 1);
        ret(e.mem.u32(node + 8))
    }

    /// `AddTail` of a pointer by address (`00ae8bf0`).
    fn fake_append_pointer(e: &mut Engine, a: &[u32]) -> Ret {
        let element = e.mem.u32(a[1]);
        list_push(e, a[0], element);
        ret(0)
    }

    // ---- the cache ----

    #[test]
    fn cache_flush_with_the_cache_off_deletes_every_cached_sound() {
        let (mut f, _audio) = audio_fixture();
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        f.e.register(LIST_REMOVE_HEAD, fake_list_remove_head);
        f.e.mem.set_u8(CACHE_ENABLED_SETTING, 0);
        let list = f.this.addr() + 0x94;
        let first = fake_sound(&mut f.e, &sounds, 1);
        let second = fake_sound(&mut f.e, &sounds, 2);
        list_push(&mut f.e, list, first);
        list_push(&mut f.e, list, second);
        f.call(0x00adcd80, &[0]);
        assert_eq!(
            sounds.calls(SOUND_SLOT_DESTRUCTOR),
            vec![vec![first, 1], vec![second, 1]]
        );
        assert_eq!(f.e.mem.u32(list + 8), 0);
    }

    #[test]
    fn cache_flush_refuses_oversized_unneeded_and_duplicate_sounds() {
        let (mut f, audio) = audio_fixture();
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        f.e.register(LIST_ADD_HEAD, fake_add_head);
        let list = f.this.addr() + 0x94;
        // Larger than iMaxSizeForCachedSound (256 KB).
        let big = fake_sound(&mut f.e, &sounds, 1);
        f.e.mem.set_u32(big + SOUND_SIZE, 0x100 * 1024 + 1);
        f.call(0x00adcd80, &[big]);
        // Type bit 0x10000.
        let flagged = fake_sound(&mut f.e, &sounds, 2);
        f.e.mem.set_u32(flagged + SOUND_TYPE_FLAGS, 0x10000);
        f.call(0x00adcd80, &[flagged]);
        // A null sound.
        f.call(0x00adcd80, &[0]);
        // A sound whose hash is cached already.
        let cached = fake_sound(&mut f.e, &sounds, 3);
        f.e.mem.set_u32(cached + SOUND_HASH, 77);
        list_push(&mut f.e, list, cached);
        let duplicate = fake_sound(&mut f.e, &sounds, 4);
        f.e.mem.set_u32(duplicate + SOUND_HASH, 77);
        f.call(0x00adcd80, &[duplicate]);
        assert!(audio.calls(AUDIO_SLOT_CREATE_NEW_GAME_SOUND).is_empty());
        assert_eq!(list_elements(&f.e, list), vec![cached]);
    }

    #[test]
    fn cache_flush_adds_a_copy_and_evicts_from_the_tail() {
        let (mut f, audio) = audio_fixture();
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        f.e.register(LIST_ADD_HEAD, fake_add_head);
        f.e.register(LIST_POP_TAIL, fake_pop_tail);
        f.e.mem.set_u32(AUDIO_CACHE_SIZE_SETTING, 2);
        let list = f.this.addr() + 0x94;
        let old = fake_sound(&mut f.e, &sounds, 1);
        f.e.mem.set_u32(old + SOUND_SIZE, 500);
        let protected = fake_sound(&mut f.e, &sounds, 2);
        f.e.mem.set_u32(protected + SOUND_TYPE_FLAGS, 0x20000);
        list_push(&mut f.e, list, old);
        list_push(&mut f.e, list, protected);
        f.e.set(f.this, BSAudioManager::iCurrentCachSize, 500);
        let sound = fake_sound(&mut f.e, &sounds, 9);
        f.e.mem.set_u32(sound + SOUND_TYPE_FLAGS, 0x44);
        f.e.mem.set_u32(sound + SOUND_HASH, 1234);
        let copy = fake_sound(&mut f.e, &sounds, 0);
        f.e.mem.set_u32(copy + SOUND_SIZE, 2000);
        audio.answer(AUDIO_SLOT_CREATE_NEW_GAME_SOUND, ret(copy));
        f.call(0x00adcd80, &[sound]);
        // The copy was made from the sound and carries state bit 0x8000.
        assert_eq!(sounds.calls(SOUND_SLOT_COPY), vec![vec![copy, sound, 0]]);
        assert_eq!(f.e.mem.u32(copy + SOUND_TYPE_FLAGS), 0x44 | 0x8000);
        // The protected sound at the tail went back to the head, `old` was
        // deleted and its size taken off the total.
        assert_eq!(sounds.calls(SOUND_SLOT_DESTRUCTOR), vec![vec![old, 1]]);
        assert_eq!(list_elements(&f.e, list), vec![protected, copy]);
        assert_eq!(f.e.get(f.this, BSAudioManager::iCurrentCachSize), 2000);
    }

    #[test]
    fn get_from_cache_hands_back_a_copy_of_an_exact_hit() {
        let (mut f, audio) = audio_fixture();
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        f.e.register(LIST_ADD_HEAD, fake_add_head);
        f.e.register(LIST_REMOVE_AT, fake_remove_at);
        let list = f.this.addr() + 0x94;
        let hit = fake_sound(&mut f.e, &sounds, 1);
        f.e.mem.set_u32(hit + SOUND_HASH, 5);
        let other = fake_sound(&mut f.e, &sounds, 2);
        f.e.mem.set_u32(other + SOUND_HASH, 6);
        list_push(&mut f.e, list, other);
        list_push(&mut f.e, list, hit);
        let copy = fake_sound(&mut f.e, &sounds, 0);
        audio.answer(AUDIO_SLOT_CREATE_NEW_GAME_SOUND, ret(copy));
        assert_eq!(f.call(0x00add1e0, &[5, 0, 0x30]).u32(), copy);
        assert_eq!(sounds.calls(SOUND_SLOT_COPY), vec![vec![copy, hit, 1]]);
        assert_eq!(f.e.mem.u32(copy + SOUND_TYPE_FLAGS), 0x30);
        // The hit moved to the head.
        assert_eq!(list_elements(&f.e, list), vec![hit, other]);
        // With type bit 0x20000000 the hit is deleted instead.
        f.e.mem.set_u32(hit + SOUND_TYPE_FLAGS, 0x2000_0000);
        assert_eq!(f.call(0x00add1e0, &[5, 0, 0x30]).u32(), copy);
        assert_eq!(sounds.calls(SOUND_SLOT_DESTRUCTOR), vec![vec![hit, 1]]);
        assert_eq!(list_elements(&f.e, list), vec![other]);
        // No hit: nothing.
        assert_eq!(f.call(0x00add1e0, &[99, 0, 0x30]).u32(), 0);
    }

    #[test]
    fn get_from_cache_picks_by_directory_with_random_draws() {
        let (mut f, audio) = audio_fixture();
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        f.e.register(LIST_ADD_HEAD, fake_add_head);
        f.e.register(LIST_REMOVE_AT, fake_remove_at);
        let draws = Rc::new(RefCell::new(vec![0u32, 0u32]));
        let source = draws.clone();
        f.e.register_double(RAND, move |_, _| ret(source.borrow_mut().remove(0)));
        let list = f.this.addr() + 0x94;
        let first = fake_sound(&mut f.e, &sounds, 1);
        f.e.mem.set_u32(first + SOUND_DIRECTORY_HASH, 7);
        let second = fake_sound(&mut f.e, &sounds, 2);
        f.e.mem.set_u32(second + SOUND_DIRECTORY_HASH, 7);
        f.e.mem.set_u32(first + SOUND_HASH, 11);
        f.e.mem.set_u32(second + SOUND_HASH, 12);
        list_push(&mut f.e, list, first);
        list_push(&mut f.e, list, second);
        let copy = fake_sound(&mut f.e, &sounds, 0);
        audio.answer(AUDIO_SLOT_CREATE_NEW_GAME_SOUND, ret(copy));
        // Hash 0 and a directory hash: both candidates, the draws swap the
        // choice back to the first.
        assert_eq!(f.call(0x00add1e0, &[0, 7, 0x10]).u32(), copy);
        assert!(draws.borrow().is_empty());
        assert_eq!(sounds.calls(SOUND_SLOT_COPY), vec![vec![copy, first, 1]]);
        assert_eq!(list_elements(&f.e, list), vec![first, second]);
    }

    #[test]
    fn is_in_cache_matches_the_hash_and_optionally_the_directory() {
        let (mut f, _audio) = audio_fixture();
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        let list = f.this.addr() + 0x94;
        let cached = fake_sound(&mut f.e, &sounds, 1);
        f.e.mem.set_u32(cached + SOUND_HASH, 5);
        f.e.mem.set_u32(cached + SOUND_DIRECTORY_HASH, 6);
        list_push(&mut f.e, list, cached);
        f.e.call_log = Some(vec![]);
        assert!(!f.call(0x00add480, &[0, 1]).bool());
        // A zero hash does not take the critical section.
        assert_eq!(f.e.call_log.take().unwrap().len(), 1);
        assert!(f.call(0x00add480, &[5, 0]).bool());
        assert!(!f.call(0x00add480, &[6, 0]).bool());
        assert!(f.call(0x00add480, &[6, 1]).bool());
        assert!(!f.call(0x00add480, &[7, 1]).bool());
    }

    /// The code address of slot `slot` of a fake vtable (to replace its
    /// double with one that has an effect).
    fn vtable_code(vtable: &FakeVtable, slot: u32) -> u32 {
        0x7300_0000 + (vtable.address & 0xfff_ffff) + slot
    }

    /// Doubles every audio message case needs that are not under test.
    fn common_message_doubles(f: &mut Fixture) {
        f.e.register(SMART_POINTER_DESTRUCTOR, |_, _| ret(0));
        f.e.register(RELEASE_MESSAGE_POINTER, |_, _| ret(0));
        f.e.register(SMART_POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(0)
        });
        f.e.register(MESSAGE_STACK_PUSH, |_, _| ret(0));
        f.e.register(LIST_ADD_HEAD, fake_add_head);
        f.e.register(LIST_REMOVE_AT, fake_remove_at);
        f.e.register(LIST_POP_TAIL, fake_pop_tail);
        f.e.register(LIST_REMOVE_HEAD, fake_list_remove_head);
    }

    /// A moving-reference map (+0x84) model: a vtable whose hash slot is
    /// `key % buckets`, whose comparison is equality, whose node allocation
    /// makes 12 bytes and whose node initialisation stores key and value.
    fn moving_map_model(f: &mut Fixture) -> Ptr<NiTPointerMap> {
        let e = &mut f.e;
        e.put_vtable(
            0x7230_0000,
            &[0, 0x7230_1004, 0x7230_1008, 0x7230_100c, 0, 0x7230_1014],
        );
        e.register(0x7230_1004, |e, a| ret(a[1] % e.mem.u32(a[0] + 4)));
        e.register(0x7230_1008, |_, a| ret((a[1] == a[2]) as u32));
        e.register(0x7230_100c, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            ret(0)
        });
        e.register(0x7230_1014, |e, _| ret(e.mem.alloc(12)));
        let map = f.this.addr() + 0x84;
        f.e.mem.set_u32(map, 0x7230_0000);
        Ptr::new(map)
    }

    /// A followed object: reference count at +4, position at +0x8c.
    fn fake_object(e: &mut Engine, references: u32, position: [f32; 3]) -> u32 {
        let object = e.mem.alloc(0xa0);
        e.mem.set_u32(object + 4, references);
        for (i, v) in position.into_iter().enumerate() {
            e.mem.set_f32(object + 0x8c + 4 * i as u32, v);
        }
        object
    }

    /// The BSAudio object, its fake vtable and the load queue object that
    /// the precache cases use.
    fn precache_world(
        f: &mut Fixture,
        audio: &FakeVtable,
        sounds: &FakeVtable,
    ) -> (u32, FakeVtable, u32) {
        let object = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem.set_u8(object + AUDIO_ENABLED, 1);
        f.e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            ret(0)
        });
        f.e.register(SOUND_NAME_HASH, |_, _| ret(0x1111));
        f.e.register(SOUND_CALCULATE_HASHES, |_, _| ret(1));
        f.e.register(LOAD_TASK_CONSTRUCTOR, |_, a| ret(a[0]));
        f.e.register(LIST_APPEND_POINTER, fake_append_pointer);
        let sound = fake_sound(&mut f.e, sounds, 0);
        f.e.mem.set_cstr(sound + SOUND_FILENAME, b"sound.wav");
        audio.answer(AUDIO_SLOT_CREATE_NEW_GAME_SOUND, ret(sound));
        let queue = fake_vtable(&mut f.e, 0x7220_0000);
        let queue_object = f.e.mem.alloc(8);
        f.e.mem.set_u32(queue_object, queue.address);
        f.e.mem.set_u32(LOAD_QUEUE_GLOBAL, queue_object);
        (sound, queue, queue_object)
    }

    /// A C string in game memory.
    fn text(e: &mut Engine, value: &[u8]) -> u32 {
        let address = e.mem.alloc(value.len() as u32 + 1);
        e.mem.set_cstr(address, value);
        address
    }

    #[test]
    fn precache_ov3_creates_the_sound_and_queues_its_load() {
        let (mut f, audio) = audio_fixture();
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        let (sound, queue, queue_object) = precache_world(&mut f, &audio, &sounds);
        f.e.set(f.this, BSAudioManager::iNextID, 5);
        let name = text(&mut f.e, b"sound.wav");
        f.call(0x00add6f0, &[name, 0x123, 7]);
        assert_eq!(f.e.mem.u32(sound + SOUND_ID), 5);
        assert_eq!(f.e.mem.u32(sound + SOUND_TYPE_FLAGS), 0x123 | 0x8000);
        assert_eq!(f.e.get(f.this, BSAudioManager::iNextID), 6);
        assert_eq!(f.e.mem.u32(sound + SOUND_STATE_FLAGS) & 0x1000, 0x1000);
        assert_eq!(audio.calls(AUDIO_SLOT_CREATE_NEW_GAME_SOUND).len(), 1);
        assert_eq!(audio.calls(AUDIO_SLOT_FIX_SOUND_PATH).len(), 1);
        // The load task went to the queue and to the cache-load list.
        let task = queue.calls(LOAD_QUEUE_SLOT_ADD);
        assert_eq!(task.len(), 1);
        assert_eq!(task[0][0], queue_object);
        assert_eq!(list_elements(&f.e, f.this.addr() + 0x124), vec![task[0][1]]);
    }

    #[test]
    fn precache_ov3_skips_a_cached_sound_and_a_disabled_audio() {
        let (mut f, audio) = audio_fixture();
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        let (_sound, _queue, _object) = precache_world(&mut f, &audio, &sounds);
        let name = text(&mut f.e, b"sound.wav");
        // Cached with the hash `SOUND_NAME_HASH` answers.
        let cached = fake_sound(&mut f.e, &sounds, 1);
        f.e.mem.set_u32(cached + SOUND_HASH, 0x1111);
        list_push(&mut f.e, f.this.addr() + 0x94, cached);
        f.call(0x00add6f0, &[name, 0, 0]);
        assert!(audio.calls(AUDIO_SLOT_CREATE_NEW_GAME_SOUND).is_empty());
        // A null name and a disabled audio do nothing, not even take an id.
        f.call(0x00add6f0, &[0, 0, 0]);
        let object = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem.set_u8(object + AUDIO_ENABLED, 0);
        f.call(0x00add6f0, &[name, 0, 0]);
        assert_eq!(f.e.get(f.this, BSAudioManager::iNextID), 1);
    }

    #[test]
    fn precache_asks_the_name_callback_and_precaches_the_result() {
        let (mut f, audio) = audio_fixture();
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        let (sound, _queue, _object) = precache_world(&mut f, &audio, &sounds);
        let object = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem.set_u32(object + AUDIO_NAME_CALLBACK, 0x7400_0000);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        f.e.register_double(0x7400_0000, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            e.mem.set_cstr(a[1], b"found.wav");
            e.mem.set_u32(a[2], e.mem.u32(a[2]) | 4);
            e.mem.set_u32(a[3], 9);
            ret(1)
        });
        let name = text(&mut f.e, b"asked");
        f.call(0x00add570, &[name, 0x10]);
        assert_eq!(seen.borrow().len(), 1);
        assert_eq!(seen.borrow()[0][0], name);
        // The flags passed on are the caller's own; the callback's extra
        // word is not used by the sound creation here.
        assert_eq!(f.e.mem.u32(sound + SOUND_TYPE_FLAGS), 0x10 | 0x8000);
        assert_eq!(audio.calls(AUDIO_SLOT_CREATE_NEW_GAME_SOUND).len(), 1);
        // A callback that answers false stops it; a missing one too.
        f.e.register(0x7400_0000, |_, _| ret(0));
        f.call(0x00add570, &[name, 0x10]);
        assert_eq!(audio.calls(AUDIO_SLOT_CREATE_NEW_GAME_SOUND).len(), 1);
        f.e.mem.set_u32(object + AUDIO_NAME_CALLBACK, 0);
        f.call(0x00add570, &[name, 0x10]);
        assert_eq!(audio.calls(AUDIO_SLOT_CREATE_NEW_GAME_SOUND).len(), 1);
    }

    #[test]
    fn precache_ov2_or_s_the_flags_the_id_callback_wrote() {
        let (mut f, audio) = audio_fixture();
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        let (sound, _queue, _object) = precache_world(&mut f, &audio, &sounds);
        let object = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem.set_u32(object + AUDIO_ID_CALLBACK, 0x7400_0010);
        f.e.register(0x7400_0010, |e, a| {
            e.mem.set_cstr(a[1], b"id.wav");
            e.mem.set_u32(a[2], 0x2);
            ret(1)
        });
        f.call(0x00add630, &[77, 0x10]);
        assert_eq!(f.e.mem.u32(sound + SOUND_TYPE_FLAGS), 0x12 | 0x8000);
    }

    #[test]
    fn queueing_a_precached_sound_discards_the_unwanted_ones() {
        let (mut f, audio) = audio_fixture();
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        let (_sound, queue, _object) = precache_world(&mut f, &audio, &sounds);
        let object = f.e.global::<u32>(AUDIO_GLOBAL);
        // No file name.
        let nameless = fake_sound(&mut f.e, &sounds, 1);
        f.call(0x00add960, &[nameless]);
        // Type bit 0x10000.
        let flagged = fake_sound(&mut f.e, &sounds, 2);
        f.e.mem.set_cstr(flagged + SOUND_FILENAME, b"a");
        f.e.mem.set_u32(flagged + SOUND_TYPE_FLAGS, 0x10000);
        f.call(0x00add960, &[flagged]);
        // Cached already.
        let cached = fake_sound(&mut f.e, &sounds, 3);
        f.e.mem.set_u32(cached + SOUND_HASH, 40);
        list_push(&mut f.e, f.this.addr() + 0x94, cached);
        let duplicate = fake_sound(&mut f.e, &sounds, 4);
        f.e.mem.set_cstr(duplicate + SOUND_FILENAME, b"a");
        f.e.mem.set_u32(duplicate + SOUND_HASH, 40);
        f.call(0x00add960, &[duplicate]);
        // A task for the same hashes is queued already.
        let loading = fake_sound(&mut f.e, &sounds, 5);
        f.e.mem.set_u32(loading + SOUND_HASH, 50);
        f.e.mem.set_u32(loading + SOUND_DIRECTORY_HASH, 51);
        let task = f.e.mem.alloc(0x38);
        f.e.mem.set_u32(task + 0x30, loading);
        list_push(&mut f.e, f.this.addr() + 0x124, task);
        let queued = fake_sound(&mut f.e, &sounds, 6);
        f.e.mem.set_cstr(queued + SOUND_FILENAME, b"a");
        f.e.mem.set_u32(queued + SOUND_HASH, 50);
        f.e.mem.set_u32(queued + SOUND_DIRECTORY_HASH, 51);
        f.call(0x00add960, &[queued]);
        // Audio off.
        let late = fake_sound(&mut f.e, &sounds, 7);
        f.e.mem.set_cstr(late + SOUND_FILENAME, b"a");
        f.e.mem.set_u8(object + AUDIO_ENABLED, 0);
        f.call(0x00add960, &[late]);
        let deleted: Vec<u32> = sounds
            .calls(SOUND_SLOT_DESTRUCTOR)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(deleted, vec![nameless, flagged, duplicate, queued, late]);
        assert!(queue.calls(LOAD_QUEUE_SLOT_ADD).is_empty());
    }

    #[test]
    fn queueing_a_new_sound_marks_it_and_hands_its_task_to_the_queue() {
        let (mut f, audio) = audio_fixture();
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        let (_sound, queue, queue_object) = precache_world(&mut f, &audio, &sounds);
        let fresh = fake_sound(&mut f.e, &sounds, 8);
        f.e.mem.set_cstr(fresh + SOUND_FILENAME, b"fresh.wav");
        f.call(0x00add960, &[fresh]);
        assert_eq!(f.e.mem.u32(fresh + SOUND_TYPE_FLAGS), 0x8000);
        assert_eq!(f.e.mem.u32(fresh + SOUND_STATE_FLAGS), 0x1000);
        let calls = queue.calls(LOAD_QUEUE_SLOT_ADD);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], queue_object);
        assert_eq!(
            list_elements(&f.e, f.this.addr() + 0x124),
            vec![calls[0][1]]
        );
    }

    #[test]
    fn process_takes_the_section_and_runs_messages_and_sounds_on_the_singleton() {
        let (mut f, _audio) = audio_fixture();
        common_message_doubles(&mut f);
        f.e.register(MESSAGE_STACK_POP, |_, _| ret(0));
        f.e.mem.set_u32(MANAGER_BUILT_GUARD, 1);
        let updates = Rc::new(RefCell::new(Vec::new()));
        let log = updates.clone();
        f.e.register_double(SYNCH_TIMER_UPDATE, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(0)
        });
        let audio = f.e.global::<u32>(AUDIO_GLOBAL);
        f.call(0x00addc00, &[]);
        assert_eq!(*updates.borrow(), vec![audio + 0x14]);
        // `ProcessMessages` ran on the singleton: the indexes swapped.
        assert_eq!(f.e.mem.u32(MANAGER_INSTANCE + 4), 1);
        assert_eq!(f.e.mem.u32(MANAGER_INSTANCE + 8), 0);
        // When the section is busy nothing is processed.
        f.e.mem.set_u32(MANAGER_INSTANCE + 4, 0);
        f.e.register(IMPORT_TRY_ENTER_CRITICAL_SECTION, |_, _| ret(0));
        f.call(0x00addc00, &[]);
        assert_eq!(f.e.mem.u32(MANAGER_INSTANCE + 4), 0);
    }

    #[test]
    fn process_messages_runs_each_message_and_swaps_the_lists() {
        let (mut f, _audio) = audio_fixture();
        common_message_doubles(&mut f);
        f.e.register(MESSAGE_STACK_POP, |_, _| ret(0));
        let pushed = Rc::new(RefCell::new(Vec::new()));
        let log = pushed.clone();
        f.e.register_double(MESSAGE_STACK_PUSH, move |_, a| {
            log.borrow_mut().push((a[0], a[1]));
            ret(0)
        });
        f.e.set(f.this, BSAudioManager::iMessageProcessingListIndex, 0);
        f.e.set(f.this, BSAudioManager::iMessageListIndex, 1);
        // Messages of kind 6 have no case, so processing them does nothing.
        let first = fake_message(&mut f.e, [6, 0, 0, 0], [0.0; 3]);
        let second = fake_message(&mut f.e, [6, 0, 0, 0], [0.0; 3]);
        let list = f.this.addr() + 0x0c;
        list_push(&mut f.e, list, first.addr());
        list_push(&mut f.e, list, second.addr());
        f.call(0x00addc70, &[]);
        let garbage = f.this.addr() + 0x44;
        assert_eq!(
            *pushed.borrow(),
            vec![(garbage, first.addr()), (garbage, second.addr())]
        );
        // The list that was being filled is processed next time.
        assert_eq!(f.e.get(f.this, BSAudioManager::iMessageListIndex), 0);
        assert_eq!(
            f.e.get(f.this, BSAudioManager::iMessageProcessingListIndex),
            1
        );
    }

    #[test]
    fn process_messages_clears_a_list_that_lost_its_ends() {
        let (mut f, _audio) = audio_fixture();
        common_message_doubles(&mut f);
        f.e.register(MESSAGE_STACK_POP, |_, _| ret(0));
        let cleared = Rc::new(RefCell::new(Vec::new()));
        let log = cleared.clone();
        f.e.register_double(LIST_CLEAR, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(0)
        });
        f.e.set(f.this, BSAudioManager::iMessageProcessingListIndex, 1);
        let list = f.this.addr() + 0x18;
        // A count, but no head or tail.
        f.e.mem.set_u32(list + 8, 2);
        f.call(0x00addc70, &[]);
        assert_eq!(*cleared.borrow(), vec![list]);
    }

    /// Logs (list, element) for every pointer appended to a list and also
    /// puts it into the list model.
    fn track_appends(f: &mut Fixture) -> Rc<RefCell<Vec<(u32, u32)>>> {
        let appended = Rc::new(RefCell::new(Vec::new()));
        let log = appended.clone();
        f.e.register_double(LIST_APPEND_POINTER, move |e, a| {
            let element = e.mem.u32(a[1]);
            log.borrow_mut().push((a[0], element));
            list_push(e, a[0], element);
            ret(0)
        });
        appended
    }

    /// `AddNodeTail` (`00a11db0`): `(list, node)`.
    fn fake_add_node_tail(e: &mut Engine, a: &[u32]) -> Ret {
        let tail = e.mem.u32(a[0] + 4);
        e.mem.set_u32(a[1], 0);
        e.mem.set_u32(a[1] + 4, tail);
        if tail == 0 {
            e.mem.set_u32(a[0], a[1]);
        } else {
            e.mem.set_u32(tail, a[1]);
        }
        e.mem.set_u32(a[0] + 4, a[1]);
        let count = e.mem.u32(a[0] + 8);
        e.mem.set_u32(a[0] + 8, count + 1);
        ret(0)
    }

    /// The listener object with a fake vtable, installed in `BSAudio`.
    fn install_listener(f: &mut Fixture) -> (u32, FakeVtable) {
        let vtable = fake_vtable(&mut f.e, 0x7240_0000);
        let listener = f.e.mem.alloc(0x20);
        f.e.mem.set_u32(listener, vtable.address);
        let audio = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem.set_u32(audio + AUDIO_LISTENER, listener);
        (listener, vtable)
    }

    /// `fabs` as a double for the update cases.
    fn install_fabs(e: &mut Engine) {
        e.register(FABS, |_, a| {
            let value = f64::from_bits((a[1] as u64) << 32 | a[0] as u64);
            Ret {
                st0: value.abs(),
                ..Ret::default()
            }
        });
    }

    /// The doubles the sound update cases share.
    fn update_doubles(f: &mut Fixture) {
        common_message_doubles(f);
        install_fabs(&mut f.e);
        f.e.register(SOUND_IS_EMITTER_SOUND, |_, _| ret(0));
        f.e.register(SOUND_HAS_EMITTER, |_, _| ret(0));
        f.e.register(SOUND_RANGE_TEST, |_, _| ret(1));
        f.e.register(MESSAGE_LIST_CONSTRUCTOR, |e, a| {
            e.mem.write(a[0], &[0; 12]);
            ret(a[0])
        });
        f.e.register(LIST_NEW_NODE, |e, _| ret(e.mem.alloc(12)));
        f.e.register(LIST_ADD_NODE_TAIL, fake_add_node_tail);
        f.e.register(LIST_CLEAR, |_, _| ret(0));
    }

    #[test]
    fn sound_update_does_nothing_without_a_listener() {
        let (mut f, _audio) = audio_fixture();
        update_doubles(&mut f);
        f.e.set(f.this, BSAudioManager::iDelayTimerDelta, 77);
        f.call(0x00adddd0, &[]);
        assert_eq!(f.e.get(f.this, BSAudioManager::iDelayTimerDelta), 77);
    }

    #[test]
    fn sound_update_finishes_a_sound_that_has_run_out() {
        let (mut f, _audio) = audio_fixture();
        update_doubles(&mut f);
        let (_listener, listener_vtable) = install_listener(&mut f);
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        moving_map_model(&mut f);
        f.e.clock_ms = 1000;
        f.e.set(f.this, BSAudioManager::iDelayTimerDelta, 400);
        let sound = fake_sound(&mut f.e, &sounds, 9);
        f.e.mem.set_u32(sound + SOUND_STATE_FLAGS, 0x10);
        f.e.mem
            .set_u32(sound + SOUND_COMPLETION_CALLBACK, 0x7600_0000);
        f.e.mem.set_u32(sound + SOUND_COMPLETION_CONTEXT, 0xc0de);
        let callbacks = Rc::new(RefCell::new(Vec::new()));
        let log = callbacks.clone();
        f.e.register_double(0x7600_0000, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(0)
        });
        list_sound(&mut f, 9, sound);
        f.call(0x00adddd0, &[]);
        // The completion callback ran with its context and the "prepared"
        // flag (state bit 0x100 is not set).
        assert_eq!(*callbacks.borrow(), vec![vec![0xc0de, 0]]);
        // The sound left the list, lost its voice state and was deleted.
        assert!(map_node(&f.e, f.this.addr() + 0x54, 9).is_none());
        assert!(map_node(&f.e, f.this.addr() + 0x74, 9).is_some());
        assert_eq!(sounds.calls(SOUND_SLOT_DESTRUCTOR), vec![vec![sound, 1]]);
        assert_eq!(sounds.calls(SOUND_SLOT_UPDATE), vec![vec![sound, 0]]);
        // The elapsed time and the end-of-update steps.
        assert_eq!(f.e.get(f.this, BSAudioManager::iDelayTimerDelta), 1000);
        assert_eq!(listener_vtable.calls(LISTENER_SLOT_GET_POSITION).len(), 1);
        assert_eq!(listener_vtable.calls(LISTENER_SLOT_AFTER_UPDATE).len(), 1);
    }

    #[test]
    fn sound_update_counts_the_cleanup_delay_down() {
        let (mut f, _audio) = audio_fixture();
        update_doubles(&mut f);
        install_listener(&mut f);
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        moving_map_model(&mut f);
        f.e.clock_ms = 1000;
        f.e.set(f.this, BSAudioManager::iDelayTimerDelta, 400);
        let sound = fake_sound(&mut f.e, &sounds, 9);
        f.e.mem.set_u32(sound + SOUND_STATE_FLAGS, 0x10);
        f.e.mem.set_u32(sound + SOUND_CLEANUP_DELAY, 800);
        list_sound(&mut f, 9, sound);
        f.call(0x00adddd0, &[]);
        // 1000 - 400 = 600 ms elapsed.
        assert_eq!(f.e.mem.u32(sound + SOUND_CLEANUP_DELAY), 200);
        assert!(sounds.calls(SOUND_SLOT_DESTRUCTOR).is_empty());
    }

    #[test]
    fn sound_update_stops_a_playing_sound_far_beyond_its_maximum() {
        let (mut f, _audio) = audio_fixture();
        update_doubles(&mut f);
        install_listener(&mut f);
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        moving_map_model(&mut f);
        f.e.register(SOUND_IS_EMITTER_SOUND, |_, _| ret(1));
        sounds.answer(SOUND_SLOT_IS_PLAYING, ret(1));
        sounds.answer(
            SOUND_SLOT_GET_FREQUENCY,
            Ret {
                st0: 1.0,
                ..Ret::default()
            },
        );
        let sound = fake_sound(&mut f.e, &sounds, 9);
        f.e.mem
            .set_u32(sound + SOUND_STATE_FLAGS, 0x1 | 0x100 | 0x800);
        f.e.mem.set_u32(sound + SOUND_TYPE_FLAGS, 0x10);
        f.e.mem.set_f32(sound + SOUND_MAX_DISTANCE, 50.0);
        f.e.mem.set_f32(sound + SOUND_DISTANCE_TO_LISTENER, 100.0);
        list_sound(&mut f, 9, sound);
        f.call(0x00adddd0, &[]);
        assert_eq!(sounds.calls(SOUND_SLOT_STOP), vec![vec![sound]]);
        assert_eq!(sounds.calls(SOUND_SLOT_SET_INAUDIBLE), vec![vec![sound, 1]]);
        // The voice state was recorded and the speed already matches.
        assert!(map_node(&f.e, f.this.addr() + 0x74, 9).is_some());
        assert!(sounds.calls(SOUND_SLOT_SET_FREQUENCY).is_empty());
    }

    #[test]
    fn sound_update_plays_an_inaudible_sound_that_came_back_in_range() {
        let (mut f, _audio) = audio_fixture();
        update_doubles(&mut f);
        install_listener(&mut f);
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        moving_map_model(&mut f);
        sounds.answer(
            SOUND_SLOT_GET_FREQUENCY,
            Ret {
                st0: 1.0,
                ..Ret::default()
            },
        );
        let sound = fake_sound(&mut f.e, &sounds, 9);
        f.e.mem.set_u32(sound + SOUND_STATE_FLAGS, 0x20000);
        f.e.mem.set_f32(sound + SOUND_MAX_DISTANCE, 50.0);
        f.e.mem.set_f32(sound + SOUND_DISTANCE_TO_LISTENER, 10.0);
        list_sound(&mut f, 9, sound);
        f.call(0x00adddd0, &[]);
        assert_eq!(sounds.calls(SOUND_SLOT_SET_INAUDIBLE), vec![vec![sound, 0]]);
        assert_eq!(sounds.calls(SOUND_SLOT_PLAY), vec![vec![sound, 1]]);
    }

    #[test]
    fn sound_update_sets_the_frequency_to_the_playback_speed() {
        let (mut f, _audio) = audio_fixture();
        update_doubles(&mut f);
        install_listener(&mut f);
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        moving_map_model(&mut f);
        f.e.mem.set_f32(TIME_MULTIPLIER, 0.1);
        sounds.answer(
            SOUND_SLOT_GET_FREQUENCY,
            Ret {
                st0: 0.5,
                ..Ret::default()
            },
        );
        let sound = fake_sound(&mut f.e, &sounds, 9);
        f.e.mem.set_u32(sound + SOUND_STATE_FLAGS, 0x800);
        list_sound(&mut f, 9, sound);
        f.call(0x00adddd0, &[]);
        // The speed is raised to the 0.25 minimum.
        assert_eq!(
            sounds.calls(SOUND_SLOT_SET_FREQUENCY),
            vec![vec![sound, 0.25f32.to_bits()]]
        );
        // With bIgnoreGlobalTimeMultiplier the speed is 1.0.
        f.e.set(f.this, BSAudioManager::bIgnoreGlobalTimeMultiplier, true);
        f.call(0x00adddd0, &[]);
        assert_eq!(
            sounds.calls(SOUND_SLOT_SET_FREQUENCY)[1],
            vec![sound, 1.0f32.to_bits()]
        );
    }

    #[test]
    fn sound_update_prepares_a_sound_and_replays_its_queued_messages() {
        let (mut f, _audio) = audio_fixture();
        update_doubles(&mut f);
        install_listener(&mut f);
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        moving_map_model(&mut f);
        f.e.mem.set_u8(CACHE_ENABLED_SETTING, 0);
        let pushed = Rc::new(RefCell::new(Vec::new()));
        let log = pushed.clone();
        f.e.register_double(MESSAGE_STACK_PUSH, move |_, a| {
            log.borrow_mut().push((a[0], a[1]));
            ret(0)
        });
        let sound = fake_sound(&mut f.e, &sounds, 9);
        f.e.mem.set_u32(sound + SOUND_STATE_FLAGS, 0x1 | 0x100);
        // Preparing the sound makes it ready (state bit 0x800).
        f.e.register(vtable_code(&sounds, SOUND_SLOT_PREPARE), move |e, a| {
            let state = e.mem.u32(a[0] + SOUND_STATE_FLAGS);
            e.mem.set_u32(a[0] + SOUND_STATE_FLAGS, state | 0x800);
            ret(0)
        });
        let queued = fake_message(&mut f.e, [6, 0, 0, 0], [0.0; 3]);
        list_push(&mut f.e, sound + SOUND_QUEUED_MESSAGES, queued.addr());
        list_sound(&mut f, 9, sound);
        f.call(0x00adddd0, &[]);
        assert_eq!(f.e.mem.u32(sound + SOUND_STATE_FLAGS) & 0x800, 0x800);
        // The queued message was processed and handed back to the pool of
        // queue messages (+0x34).
        assert_eq!(
            *pushed.borrow(),
            vec![(f.this.addr() + 0x34, queued.addr())]
        );
        assert_eq!(f.e.mem.u32(sound + SOUND_QUEUED_MESSAGES + 8), 0);
    }

    #[test]
    fn followed_objects_move_their_sounds_and_lost_ones_are_dropped() {
        let (mut f, _audio) = audio_fixture();
        common_message_doubles(&mut f);
        let (_listener, listener_vtable) = install_listener(&mut f);
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        let map = moving_map_model(&mut f);
        // Entry 3: a playing sound of type bit 4 follows an object.
        sounds.answer(SOUND_SLOT_IS_PLAYING, ret(1));
        let moving = fake_sound(&mut f.e, &sounds, 3);
        f.e.mem.set_u32(moving + SOUND_TYPE_FLAGS, 4);
        list_sound(&mut f, 3, moving);
        let object = fake_object(&mut f.e, 5, [1.0, 2.0, 3.0]);
        f.e.call(0x00ae7cc0, &args![map, 3u32, object]);
        // Entry 4: no sound any more.
        let lost_object = fake_object(&mut f.e, 5, [0.0; 3]);
        f.e.call(0x00ae7cc0, &args![map, 4u32, lost_object]);
        let released = Rc::new(RefCell::new(Vec::new()));
        let log = released.clone();
        f.e.register_double(RELEASE_MESSAGE_POINTER, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(0)
        });
        f.call(0x00adea90, &[]);
        assert_eq!(listener_vtable.calls(LISTENER_SLOT_GET_POSITION).len(), 1);
        // The emitter follows the object, 200 higher for type bit 4.
        assert_eq!(
            sounds.calls(SOUND_SLOT_SET_EMITTER_POSITION),
            vec![vec![
                moving,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                203.0f32.to_bits()
            ]]
        );
        // The entry without a sound is removed and its object released.
        assert!(map_node(&f.e, map.addr(), 4).is_none());
        assert_eq!(*released.borrow(), vec![lost_object]);
        assert!(map_node(&f.e, map.addr(), 3).is_some());
    }

    #[test]
    fn a_followed_object_that_vanished_fades_its_sound_out() {
        let (mut f, _audio) = audio_fixture();
        common_message_doubles(&mut f);
        install_listener(&mut f);
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        let map = moving_map_model(&mut f);
        let sound = fake_sound(&mut f.e, &sounds, 6);
        list_sound(&mut f, 6, sound);
        f.e.call(0x00ae7cc0, &args![map, 6u32, 0u32]);
        f.call(0x00adea90, &[]);
        assert!(map_node(&f.e, map.addr(), 6).is_none());
        assert_eq!(
            f.messages(),
            vec![Message {
                kind: 9,
                a: 6,
                b: 0,
                c: 0x3e8,
                pointer: 0,
                vector: [38.0, 0.0, 0.0],
            }]
        );
        // Without a listener nothing happens at all.
        let audio = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem.set_u32(audio + AUDIO_LISTENER, 0);
        f.e.call(0x00ae7cc0, &args![map, 7u32, 0u32]);
        f.call(0x00adea90, &[]);
        assert!(map_node(&f.e, map.addr(), 7).is_some());
    }

    #[test]
    fn the_followed_object_loop_keeps_its_slots_at_the_exe_frame_offsets() {
        let (mut f, _audio) = audio_fixture();
        common_message_doubles(&mut f);
        install_listener(&mut f);
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        let map = moving_map_model(&mut f);
        let sound = fake_sound(&mut f.e, &sounds, 6);
        list_sound(&mut f, 6, sound);
        let object = fake_object(&mut f.e, 5, [0.0; 3]);
        f.e.call(0x00ae7cc0, &args![map, 6u32, object]);
        // The frame's place on the stack (its words stay there after the call: nothing takes
        // that place again before they are read).
        let frame = f.e.next_stack_local(0x104);
        f.call(0x00adea90, &[]);
        // The position (+0xf0, ended at 0), the key (+0xdc) and the object slot (+0xc0)
        // the walk filled.
        assert_eq!(f.e.mem.u32(frame + 0xf0), 0);
        assert_eq!(f.e.mem.u32(frame + 0xdc), 6);
        assert_eq!(f.e.mem.u32(frame + 0xc0), object);
    }

    #[test]
    fn the_sound_update_keeps_the_object_slot_at_frame_offset_0x160() {
        let (mut f, _audio) = audio_fixture();
        update_doubles(&mut f);
        install_listener(&mut f);
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        let map = moving_map_model(&mut f);
        let sound = fake_sound(&mut f.e, &sounds, 9);
        f.e.mem.set_u32(sound + SOUND_STATE_FLAGS, 0x10);
        list_sound(&mut f, 9, sound);
        let object = fake_object(&mut f.e, 5, [0.0; 3]);
        f.e.call(0x00ae7cc0, &args![map, 9u32, object]);
        let cleared = Rc::new(RefCell::new(Vec::new()));
        let log = cleared.clone();
        f.e.register_double(SMART_POINTER_SET, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        let frame = f.e.next_stack_local(0x184);
        f.call(0x00adddd0, &[]);
        // The finished sound's followed object is fetched into the slot and cleared there.
        assert_eq!(cleared.borrow()[0], vec![frame + 0x160, 0]);
    }

    #[test]
    fn the_listener_position_and_underwater_flag_are_queued() {
        let (mut f, _audio) = audio_fixture();
        f.call(
            0x00adef80,
            &[1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()],
        );
        assert_eq!(
            f.messages(),
            vec![Message {
                kind: 0x38,
                a: 0,
                b: 0,
                c: 0,
                pointer: 0,
                vector: [1.0, 2.0, 3.0],
            }]
        );
        // The flag only queues a message when it changes.
        f.call(0x00adeff0, &[0]);
        assert_eq!(f.messages().len(), 1);
        f.call(0x00adeff0, &[1]);
        assert_eq!(f.e.mem.u8(f.this.addr() + 0x175), 1);
        assert_eq!(f.messages()[1], plain(0x37, 0, 0, 0));
    }

    // ---- ProcessMessage ----

    /// A manager with the message doubles, the `BSAudio` fake and a sound
    /// vtable.
    struct World {
        f: Fixture,
        audio: FakeVtable,
        sounds: FakeVtable,
    }

    fn world() -> World {
        let (mut f, audio) = audio_fixture();
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        common_message_doubles(&mut f);
        f.e.register(SOUND_RANGE_TEST, |_, _| ret(1));
        f.e.register(SOUND_ENVIRONMENT_STEP, |_, _| ret(0));
        install_fabs(&mut f.e);
        f.e.register(LIST_NEW_NODE, |e, _| ret(e.mem.alloc(12)));
        f.e.register(LIST_ADD_NODE_TAIL, fake_add_node_tail);
        World { f, audio, sounds }
    }

    /// A sound in the list with the given state and type flags.
    fn add_sound(w: &mut World, id: u32, state: u32, kind: u32) -> u32 {
        let sound = fake_sound(&mut w.f.e, &w.sounds, id);
        w.f.e.mem.set_u32(sound + SOUND_STATE_FLAGS, state);
        w.f.e.mem.set_u32(sound + SOUND_TYPE_FLAGS, kind);
        list_sound(&mut w.f, id, sound);
        sound
    }

    /// A sound that is ready for messages (state bits 0x800 and 0x100).
    fn ready_sound(w: &mut World, id: u32, kind: u32) -> u32 {
        add_sound(w, id, 0x900, kind)
    }

    /// Processes a message and answers what `ProcessMessage` answered.
    fn send(w: &mut World, words: [u32; 4], vector: [f32; 3]) -> bool {
        let message = fake_message(&mut w.f.e, words, vector);
        w.f.call(0x00adf080, &[message.addr()]).bool()
    }

    /// Whether the voice state of sound `id` has been recorded.
    fn has_voice_state(w: &World, id: u32) -> bool {
        map_node(&w.f.e, w.f.this.addr() + 0x74, id).is_some()
    }

    /// A pause table of four entries.
    fn install_pause_table(w: &mut World) -> u32 {
        let table = w.f.e.mem.alloc(32);
        w.f.e.set(w.f.this, BSAudioManager::pPauseEntries, table);
        w.f.e.set(w.f.this, BSAudioManager::iPauseListSize, 4);
        table
    }

    #[test]
    fn unknown_message_kinds_do_nothing_and_the_context_is_restored() {
        let mut w = world();
        let context = w.f.e.tls() + AUDIO_TLS_CONTEXT;
        w.f.e.mem.set_u32(context, 0x1234);
        for kind in [0, 6, 15, 17, 43, 54, 67] {
            assert!(!send(&mut w, [kind, 1, 2, 3], [0.0; 3]));
        }
        assert_eq!(w.f.e.mem.u32(context), 0x1234);
        assert!(w.f.messages().is_empty());
    }

    #[test]
    fn play_starts_a_ready_sound() {
        let mut w = world();
        w.f.e.register(SOUND_RANGE_TEST, |_, _| ret(0));
        let sound = ready_sound(&mut w, 4, 0);
        w.f.e.mem.set_f32(sound + SOUND_CURRENT_VOLUME, 0.5);
        send(&mut w, [1, 4, 0, 0], [0.0; 3]);
        // Out of range: state bit 0x10000; then the volume and the play.
        let state = w.f.e.mem.u32(sound + SOUND_STATE_FLAGS);
        assert_eq!(state, 0x900 | 0x10000 | 0x2000);
        assert_eq!(
            w.sounds.calls(SOUND_SLOT_SET_VOLUME)[0],
            vec![sound, 0.5f32.to_bits()]
        );
        assert_eq!(w.sounds.calls(SOUND_SLOT_PLAY), vec![vec![sound, 0]]);
        assert!(has_voice_state(&w, 4));
    }

    #[test]
    fn play_varies_the_frequency_and_applies_the_mute_and_dialogue_fade() {
        let mut w = world();
        w.f.e.register(BSRANDOM_CONSTRUCTOR, |_, _| ret(0));
        w.f.e.register(BSRANDOM_UNSIGNED_INT, |_, a| {
            assert_eq!(a, [BSRANDOM_OBJECT, 0xffff_ffff]);
            ret(0x4000_0000)
        });
        w.f.e.set(w.f.this, BSAudioManager::iMuteMask, 2);
        w.f.e.set(w.f.this, BSAudioManager::bDialogueFade, true);
        w.f.e
            .set(w.f.this, BSAudioManager::bListenerUnderwater, true);
        let sound = ready_sound(&mut w, 4, 0x40 | 2);
        w.f.e.mem.set_u8(sound + SOUND_FREQUENCY_VARIANCE, 50);
        w.f.e.mem.set_u32(sound + SOUND_MODIFIER_FLAGS, 1);
        w.sounds.answer(
            SOUND_SLOT_GET_FREQUENCY,
            Ret {
                st0: 1.0,
                ..Ret::default()
            },
        );
        let steps = Rc::new(RefCell::new(Vec::new()));
        let log = steps.clone();
        w.f.e.register_double(SOUND_ENVIRONMENT_STEP, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(0)
        });
        send(&mut w, [1, 4, 0, 0], [0.0; 3]);
        assert_eq!(w.f.e.mem.u16(sound + SOUND_SYSTEM_ATTENUATION), 10000);
        // 6.0 decibels times 100.
        assert_eq!(w.f.e.mem.u16(sound + SOUND_FADER_ATTENUATION), 600);
        // (1 * 2 * 0.25) - 0.5 + 1.0 = 1.0 - 0.0: a quarter of the range.
        assert_eq!(
            w.sounds.calls(SOUND_SLOT_SET_FREQUENCY),
            vec![vec![sound, 0.75f32.to_bits()]]
        );
        assert_eq!(*steps.borrow(), vec![vec![sound, 1]]);
    }

    #[test]
    fn play_waits_for_a_sound_that_is_not_ready() {
        let mut w = world();
        // Without state bit 1 the message is queued again.
        add_sound(&mut w, 4, 0, 0);
        send(&mut w, [1, 4, 8, 9], [5.0, 6.0, 7.0]);
        assert_eq!(w.f.messages(), vec![plain(1, 4, 8, 9)]);
        let appended = track_appends(&mut w.f);
        // With it the message goes to the sound's own queue.
        let sound = add_sound(&mut w, 5, 1, 0);
        send(&mut w, [1, 5, 8, 9], [5.0, 6.0, 7.0]);
        let queued = appended.borrow().last().copied().unwrap();
        assert_eq!(queued.0, sound + SOUND_QUEUED_MESSAGES);
        let copy = queued.1;
        assert_eq!(w.f.e.mem.u32(copy), 1);
        assert_eq!(w.f.e.mem.u32(copy + 4), 5);
        assert_eq!(w.f.e.mem.f32(copy + 0x14), 5.0);
    }

    #[test]
    fn looping_pause_and_stop_act_on_a_ready_sound_and_queue_otherwise() {
        let mut w = world();
        let appended = track_appends(&mut w.f);
        let sound = ready_sound(&mut w, 4, 0);
        send(&mut w, [2, 4, 0, 0], [0.0; 3]);
        send(&mut w, [3, 4, 0, 0], [0.0; 3]);
        send(&mut w, [4, 4, 0, 0], [0.0; 3]);
        assert_eq!(w.sounds.calls(SOUND_SLOT_PLAY), vec![vec![sound, 1]]);
        assert_eq!(w.sounds.calls(SOUND_SLOT_PAUSE), vec![vec![sound]]);
        assert_eq!(w.sounds.calls(SOUND_SLOT_STOP), vec![vec![sound]]);
        assert!(has_voice_state(&w, 4));
        let waiting = add_sound(&mut w, 5, 0, 0);
        for kind in [2, 3, 4] {
            send(&mut w, [kind, 5, 0, 0], [0.0; 3]);
        }
        assert_eq!(appended.borrow().len(), 3);
        assert!(appended
            .borrow()
            .iter()
            .all(|a| a.0 == waiting + SOUND_QUEUED_MESSAGES));
        assert_eq!(w.sounds.calls(SOUND_SLOT_PLAY).len(), 1);
    }

    #[test]
    fn play_after_waits_for_its_time_and_plays_unless_paused() {
        let mut w = world();
        w.f.e.clock_ms = 1000;
        add_sound(&mut w, 3, 0, 0);
        let sound = ready_sound(&mut w, 4, 0);
        // Too early: queued again as it is.
        send(&mut w, [5, 4, 2000, 7], [0.0; 3]);
        assert_eq!(w.f.messages(), vec![plain(5, 4, 2000, 7)]);
        // On time but paused: no play call.
        w.sounds.answer(SOUND_SLOT_IS_PAUSED, ret(1));
        send(&mut w, [5, 4, 1000, 7], [0.0; 3]);
        assert!(w.sounds.calls(SOUND_SLOT_PLAY).is_empty());
        // Not paused: it plays.
        w.sounds.answer(SOUND_SLOT_IS_PAUSED, ret(0));
        send(&mut w, [5, 4, 1000, 7], [0.0; 3]);
        assert_eq!(w.sounds.calls(SOUND_SLOT_PLAY), vec![vec![sound, 0]]);
        // A sound that is not ready gets an `MPLAY` message.
        send(&mut w, [5, 3, 500, 7], [0.0; 3]);
        assert_eq!(w.f.messages()[1], plain(1, 3, 500, 7));
    }

    #[test]
    fn a_paused_by_table_sound_is_paused_or_stopped_after_starting() {
        let mut w = world();
        let table = install_pause_table(&mut w);
        // Entry 0: type mask 0x4 (any bit), mode 2.
        w.f.e.mem.set_u32(table, 4);
        w.f.e.mem.set_u16(table + 4, 2);
        let sound = ready_sound(&mut w, 4, 4);
        send(&mut w, [1, 4, 0, 0], [0.0; 3]);
        assert_eq!(w.sounds.calls(SOUND_SLOT_PAUSE), vec![vec![sound]]);
        // With type bit 0x1000 it is stopped instead (not attenuated).
        let other = ready_sound(&mut w, 5, 4 | 0x1000);
        send(&mut w, [1, 5, 0, 0], [0.0; 3]);
        assert_eq!(w.sounds.calls(SOUND_SLOT_STOP), vec![vec![other]]);
        assert_eq!(w.f.e.mem.u32(other + SOUND_STATE_FLAGS) & 0x10, 0x10);
        // With 0x400000 the environment step runs with 1.
        let steps = Rc::new(RefCell::new(Vec::new()));
        let log = steps.clone();
        w.f.e.register_double(SOUND_ENVIRONMENT_STEP, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(0)
        });
        let third = ready_sound(&mut w, 6, 4 | 0x40_0000);
        send(&mut w, [1, 6, 0, 0], [0.0; 3]);
        assert_eq!(*steps.borrow(), vec![vec![third, 1]]);
    }

    #[test]
    fn mute_and_unmute_set_the_system_attenuation_or_wait() {
        let mut w = world();
        let sound = ready_sound(&mut w, 4, 0);
        send(&mut w, [7, 4, 0, 0], [0.0; 3]);
        assert_eq!(w.f.e.mem.u16(sound + SOUND_SYSTEM_ATTENUATION), 10000);
        send(&mut w, [8, 4, 0, 0], [0.0; 3]);
        assert_eq!(w.f.e.mem.u16(sound + SOUND_SYSTEM_ATTENUATION), 0);
        assert_eq!(w.sounds.calls(SOUND_SLOT_SET_VOLUME).len(), 2);
        // A sound that is not ready keeps a copy of the message.
        let waiting = add_sound(&mut w, 5, 0, 0);
        send(&mut w, [7, 5, 0, 0], [0.0; 3]);
        let queued = list_elements(&w.f.e, waiting + SOUND_QUEUED_MESSAGES);
        assert_eq!(queued.len(), 1);
        assert_eq!(w.f.e.mem.u32(queued[0]), 7);
    }

    #[test]
    fn fade_out_runs_over_its_time_and_sends_its_follow_up() {
        let mut w = world();
        w.f.e.clock_ms = 1500;
        let sound = ready_sound(&mut w, 4, 0);
        // Halfway: attenuation (1 - 500/1000) * 3500 and the message again.
        send(&mut w, [9, 4, 1000, 2000], [15.0, 1.0, 2.0]);
        assert_eq!(w.f.e.mem.u16(sound + SOUND_SYSTEM_ATTENUATION), 1750);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_STATE_FLAGS) & 0x200, 0x200);
        assert_eq!(
            w.f.messages(),
            vec![Message {
                kind: 9,
                a: 4,
                b: 1000,
                c: 2000,
                pointer: 0,
                vector: [15.0, 1.0, 2.0]
            }]
        );
        assert!(has_voice_state(&w, 4));
        // Past the end: full attenuation, no 0x200, and message 15 follows.
        w.f.e.clock_ms = 2500;
        send(&mut w, [9, 4, 1000, 2000], [15.0, 1.0, 2.0]);
        assert_eq!(w.f.e.mem.u16(sound + SOUND_SYSTEM_ATTENUATION), 10000);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_STATE_FLAGS) & 0x200, 0);
        assert_eq!(w.f.messages()[1], plain(15, 4, 0, 0));
        // A start time of 0 means "now" and the end is a duration.
        w.f.e.clock_ms = 125;
        let message = fake_message(&mut w.f.e, [9, 4, 0, 300], [0.0; 3]);
        w.f.call(0x00adf080, &[message.addr()]);
        assert_eq!(w.f.e.mem.u32(message.addr() + 8), 125);
        assert_eq!(w.f.e.mem.u32(message.addr() + 0xc), 425);
    }

    #[test]
    fn fade_out_of_a_sound_that_is_not_ready_waits_for_the_end() {
        let mut w = world();
        add_sound(&mut w, 4, 0, 0);
        w.f.e.clock_ms = 1500;
        send(&mut w, [9, 4, 1000, 2000], [0.0, 1.0, 2.0]);
        assert_eq!(w.f.messages().len(), 1);
        assert_eq!(w.f.messages()[0].kind, 9);
        w.f.e.clock_ms = 2500;
        send(&mut w, [9, 4, 1000, 2000], [7.0, 0.0, 0.0]);
        assert_eq!(w.f.messages().len(), 2);
        assert_eq!(w.f.messages()[1], plain(7, 4, 0, 0));
    }

    #[test]
    fn fade_in_raises_the_attenuation_back_to_zero() {
        let mut w = world();
        w.f.e.clock_ms = 1500;
        let sound = ready_sound(&mut w, 4, 0);
        send(&mut w, [11, 4, 1000, 2000], [0.0; 3]);
        assert_eq!(w.f.e.mem.u16(sound + SOUND_SYSTEM_ATTENUATION), 1750);
        assert_eq!(w.f.messages(), vec![plain(11, 4, 1000, 2000)]);
        w.f.e.clock_ms = 2000;
        send(&mut w, [11, 4, 1000, 2000], [0.0; 3]);
        assert_eq!(w.f.e.mem.u16(sound + SOUND_SYSTEM_ATTENUATION), 0);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_STATE_FLAGS) & 0x200, 0);
        // A sound that is not ready: with state bit 0x800 and at the end
        // time it is unmuted, before it the message waits.
        add_sound(&mut w, 5, 0x800, 0);
        send(&mut w, [11, 5, 1000, 2000], [0.0; 3]);
        send(&mut w, [11, 5, 1000, 3000], [0.0; 3]);
        let messages = w.f.messages();
        assert_eq!(messages[1], plain(8, 5, 1000, 2000));
        assert_eq!(messages[2], plain(11, 5, 1000, 3000));
    }

    #[test]
    fn fade_to_and_fade_from_move_the_fader_attenuation() {
        let mut w = world();
        w.f.e.clock_ms = 1500;
        let sound = ready_sound(&mut w, 4, 0);
        // The target level is 2.0 * 100 = 200.
        send(&mut w, [10, 4, 1000, 2000], [2.0, 0.0, 0.0]);
        assert_eq!(w.f.e.mem.u16(sound + SOUND_FADER_ATTENUATION), 100);
        send(&mut w, [12, 4, 1000, 2000], [2.0, 0.0, 0.0]);
        assert_eq!(w.f.e.mem.u16(sound + SOUND_FADER_ATTENUATION), 100);
        assert_eq!(w.f.messages().len(), 2);
        assert_eq!(w.f.messages()[0].kind, 10);
        assert_eq!(w.f.messages()[1].kind, 12);
        w.f.e.clock_ms = 2000;
        send(&mut w, [10, 4, 1000, 2000], [2.0, 0.0, 0.0]);
        assert_eq!(w.f.e.mem.u16(sound + SOUND_FADER_ATTENUATION), 200);
        send(&mut w, [12, 4, 1000, 2000], [2.0, 0.0, 0.0]);
        assert_eq!(w.f.e.mem.u16(sound + SOUND_FADER_ATTENUATION), 0);
        // A sound that is not ready keeps the fade queued, unless marked 0x10.
        add_sound(&mut w, 5, 0, 0);
        add_sound(&mut w, 6, 0x10, 0);
        send(&mut w, [10, 5, 1000, 2000], [2.0, 0.0, 0.0]);
        send(&mut w, [12, 5, 1000, 2000], [2.0, 0.0, 0.0]);
        send(&mut w, [10, 6, 1000, 2000], [2.0, 0.0, 0.0]);
        send(&mut w, [12, 6, 1000, 2000], [2.0, 0.0, 0.0]);
        assert_eq!(w.f.messages().len(), 4);
    }

    #[test]
    fn dialogue_fade_fades_every_eligible_sound() {
        let mut w = world();
        w.f.e.clock_ms = 5000;
        ready_sound(&mut w, 4, 0);
        ready_sound(&mut w, 5, 0x20);
        send(&mut w, [13, 1, 0, 0], [0.0; 3]);
        assert!(w.f.e.get(w.f.this, BSAudioManager::bDialogueFade));
        // Switching on fades to the dialogue level over the "out" seconds.
        assert_eq!(
            w.f.messages(),
            vec![Message {
                kind: 10,
                a: 4,
                b: 5000,
                c: 6000,
                pointer: 0,
                vector: [6.0, 0.0, 0.0]
            }]
        );
        send(&mut w, [13, 0, 0, 0], [0.0; 3]);
        assert!(!w.f.e.get(w.f.this, BSAudioManager::bDialogueFade));
        assert_eq!(w.f.messages()[1].kind, 12);
        assert_eq!(w.f.messages()[1].c, 7000);
    }

    #[test]
    fn volume_changes_reach_every_sound() {
        let mut w = world();
        let first = ready_sound(&mut w, 4, 0);
        let second = ready_sound(&mut w, 5, 0);
        w.f.e.mem.set_f32(first + SOUND_CURRENT_VOLUME, 0.25);
        w.f.e.mem.set_f32(second + SOUND_CURRENT_VOLUME, 0.75);
        send(&mut w, [14, 0, 0, 0], [0.0; 3]);
        assert_eq!(
            w.sounds.calls(SOUND_SLOT_SET_VOLUME),
            vec![
                vec![first, 0.25f32.to_bits()],
                vec![second, 0.75f32.to_bits()]
            ]
        );
    }

    #[test]
    fn set_volume_scales_the_value_by_a_thousand() {
        let mut w = world();
        let sound = ready_sound(&mut w, 4, 0);
        send(&mut w, [16, 4, 500, 0], [0.0; 3]);
        assert_eq!(
            w.sounds.calls(SOUND_SLOT_SET_VOLUME),
            vec![vec![sound, 0.5f32.to_bits()]]
        );
        assert!(has_voice_state(&w, 4));
        add_sound(&mut w, 5, 0, 0);
        add_sound(&mut w, 6, 0x10, 0);
        send(&mut w, [16, 5, 250, 0], [0.0; 3]);
        send(&mut w, [16, 6, 250, 0], [0.0; 3]);
        assert_eq!(w.f.messages(), vec![plain(16, 5, 250, 0)]);
    }

    #[test]
    fn frequency_messages_set_the_sound_frequency_fields() {
        let mut w = world();
        let sound = ready_sound(&mut w, 4, 0);
        send(&mut w, [19, 4, 1500, 0], [0.0; 3]);
        assert_eq!(
            w.sounds.calls(SOUND_SLOT_SET_FREQUENCY),
            vec![vec![sound, 1.5f32.to_bits()]]
        );
        send(&mut w, [20, 4, 0x1_23, 0], [0.0; 3]);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_TYPE_FLAGS) & 0x40, 0x40);
        assert_eq!(w.f.e.mem.u8(sound + SOUND_FREQUENCY_VARIANCE), 0x23);
        w.f.e.mem.set_u16(sound + SOUND_SAMPLES_PER_SECOND, 44100);
        send(&mut w, [66, 4, 500, 0], [0.0; 3]);
        assert_eq!(w.f.e.mem.u16(sound + SOUND_MOD_SAMPLES_PER_SECOND), 22050);
    }

    #[test]
    fn mute_all_of_type_fades_or_mutes_matching_sounds() {
        let mut w = world();
        w.f.e.clock_ms = 125;
        let steps = Rc::new(RefCell::new(Vec::new()));
        let log = steps.clone();
        w.f.e.register_double(SOUND_ENVIRONMENT_STEP, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(0)
        });
        let matching = ready_sound(&mut w, 4, 0x4);
        ready_sound(&mut w, 5, 0x2);
        ready_sound(&mut w, 6, 0x4 | 0x800);
        // Immediate: the id word is the fade gate, `pData` the duration (0).
        send(&mut w, [24, 6000, 0x4, 0], [0.0; 3]);
        assert_eq!(w.f.e.get(w.f.this, BSAudioManager::iMuteMask), 4);
        assert_eq!(*steps.borrow(), vec![vec![matching, 1]]);
        assert_eq!(w.sounds.calls(SOUND_SLOT_SET_VOLUME).len(), 1);
        // With a duration and a gate over 5000 the sound is faded out.
        send(&mut w, [24, 6000, 0x4, 200], [0.0; 3]);
        assert_eq!(w.f.messages(), vec![plain(9, 4, 125, 325)]);
        // With a duration and a gate of 5000 or less nothing happens.
        send(&mut w, [24, 5000, 0x4, 200], [0.0; 3]);
        assert_eq!(w.f.messages().len(), 1);
    }

    #[test]
    fn unmute_all_of_type_fades_in_or_unmutes() {
        let mut w = world();
        w.f.e.clock_ms = 125;
        let steps = Rc::new(RefCell::new(Vec::new()));
        let log = steps.clone();
        w.f.e.register_double(SOUND_ENVIRONMENT_STEP, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(0)
        });
        w.f.e.set(w.f.this, BSAudioManager::iMuteMask, 0x6);
        let matching = ready_sound(&mut w, 4, 0x4);
        send(&mut w, [25, 0, 0x4, 0], [0.0; 3]);
        assert_eq!(w.f.e.get(w.f.this, BSAudioManager::iMuteMask), 0x2);
        assert_eq!(*steps.borrow(), vec![vec![matching, 0]]);
        send(&mut w, [25, 0, 0x4, 200], [0.0; 3]);
        assert_eq!(w.f.messages(), vec![plain(11, 4, 125, 325)]);
    }

    #[test]
    fn pause_and_unpause_all_of_type_use_the_pause_table() {
        let mut w = world();
        let table = install_pause_table(&mut w);
        let playing = ready_sound(&mut w, 4, 0x4);
        let other = ready_sound(&mut w, 5, 0x2);
        w.sounds.answer(SOUND_SLOT_IS_PLAYING, ret(1));
        send(&mut w, [26, 0, 0x4, 0], [0.0; 3]);
        // The mask is in the table (any-bit mode) and the matching playing
        // sound is paused.
        assert_eq!(w.f.e.mem.u32(table), 4);
        assert_eq!(w.f.e.mem.u16(table + 4), 2);
        assert_eq!(w.sounds.calls(SOUND_SLOT_PAUSE), vec![vec![playing]]);
        w.sounds.answer(SOUND_SLOT_IS_PAUSED, ret(1));
        send(&mut w, [27, 0, 0x4, 0], [0.0; 3]);
        assert_eq!(w.sounds.calls(SOUND_SLOT_PLAY), vec![vec![playing, 0]]);
        // The mask left the table.
        assert_eq!(w.f.e.mem.u16(table + 4), 0);
        let _ = other;
    }

    #[test]
    fn exact_pause_and_unpause_need_all_the_bits() {
        let mut w = world();
        let table = install_pause_table(&mut w);
        let both = ready_sound(&mut w, 4, 0x6);
        ready_sound(&mut w, 5, 0x2);
        w.sounds.answer(SOUND_SLOT_IS_PLAYING, ret(1));
        send(&mut w, [28, 0, 0x6, 0], [0.0; 3]);
        assert_eq!(w.f.e.mem.u16(table + 4), 1);
        assert_eq!(w.sounds.calls(SOUND_SLOT_PAUSE), vec![vec![both]]);
        w.sounds.answer(SOUND_SLOT_IS_PAUSED, ret(1));
        send(&mut w, [29, 0, 0x6, 0], [0.0; 3]);
        assert_eq!(w.sounds.calls(SOUND_SLOT_PLAY), vec![vec![both, 0]]);
        assert_eq!(w.f.e.mem.u16(table + 4), 0);
    }

    #[test]
    fn kill_all_of_type_stops_the_matching_sounds() {
        let mut w = world();
        let victim = ready_sound(&mut w, 4, 0x4);
        let spared = ready_sound(&mut w, 5, 0x4 | 0x100000);
        send(&mut w, [42, 0, 0x4, 0], [0.0; 3]);
        assert_eq!(w.sounds.calls(SOUND_SLOT_STOP), vec![vec![victim]]);
        assert_eq!(w.f.e.mem.u32(victim + SOUND_STATE_FLAGS), 0x910);
        assert_eq!(w.f.e.mem.u32(spared + SOUND_STATE_FLAGS), 0x900);
    }

    #[test]
    fn positions_replace_the_waiting_position_message() {
        let mut w = world();
        let ready = ready_sound(&mut w, 4, 0);
        send(&mut w, [30, 4, 0, 0], [1.0, 2.0, 3.0]);
        assert_eq!(
            w.sounds.calls(SOUND_SLOT_SET_EMITTER_POSITION),
            vec![vec![
                ready,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits()
            ]]
        );
        let waiting = add_sound(&mut w, 5, 0, 0);
        send(&mut w, [30, 5, 0, 0], [1.0, 2.0, 3.0]);
        let queued = list_elements(&w.f.e, waiting + SOUND_QUEUED_MESSAGES);
        assert_eq!(queued.len(), 1);
        send(&mut w, [30, 5, 0, 0], [4.0, 5.0, 6.0]);
        // The same queued message now holds the newer position.
        let queued_again = list_elements(&w.f.e, waiting + SOUND_QUEUED_MESSAGES);
        assert_eq!(queued_again, queued);
        assert_eq!(w.f.e.mem.f32(queued[0] + 0x14), 4.0);
        assert_eq!(w.f.e.mem.f32(queued[0] + 0x1c), 6.0);
    }

    #[test]
    fn beam_end_is_stored_or_queued_with_its_object() {
        let mut w = world();
        let ready = ready_sound(&mut w, 4, 0);
        send(&mut w, [31, 4, 0, 0], [1.0, 2.0, 3.0]);
        assert_eq!(w.f.e.mem.f32(ready + SOUND_BEAM_END + 4), 2.0);
        assert_eq!(w.f.e.mem.u32(ready + SOUND_STATE_FLAGS) & 0x8000, 0x8000);
        add_sound(&mut w, 5, 0, 0);
        let object = fake_object(&mut w.f.e, 3, [0.0; 3]);
        let message = fake_message(&mut w.f.e, [31, 5, 8, 9], [1.0, 2.0, 3.0]);
        w.f.e.mem.set_u32(message.addr() + 0x10, object);
        w.f.call(0x00adf080, &[message.addr()]);
        let queued = w.f.messages();
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0].kind, 0x1f);
        assert_eq!(queued[0].pointer, object);
        assert_eq!(queued[0].vector, [1.0, 2.0, 3.0]);
        // The original message let go of the object.
        assert_eq!(w.f.e.mem.u32(message.addr() + 0x10), 0);
    }

    #[test]
    fn attenuation_messages_set_the_sound_fields() {
        let mut w = world();
        let sound = ready_sound(&mut w, 4, 0);
        send(&mut w, [33, 4, 0x1_0007, 0], [0.0; 3]);
        send(&mut w, [34, 4, 0x2_0009, 0], [0.0; 3]);
        assert_eq!(w.f.e.mem.u16(sound + SOUND_STATIC_ATTENUATION), 7);
        assert_eq!(w.f.e.mem.u16(sound + SOUND_REVERB_ATTENUATION), 9);
        assert_eq!(w.sounds.calls(SOUND_SLOT_SET_VOLUME).len(), 2);
        send(&mut w, [35, 4, 2500, 7500], [0.0; 3]);
        assert_eq!(
            w.sounds.calls(SOUND_SLOT_SET_MIN_MAX),
            vec![vec![sound, 2.5f32.to_bits(), 7.5f32.to_bits()]]
        );
        let message = fake_message(&mut w.f.e, [36, 4, 0, 0], [0.0; 3]);
        for (i, v) in [10u16, 20, 30, 40, 50].into_iter().enumerate() {
            w.f.e.mem.set_u16(message.addr() + 0x14 + 2 * i as u32, v);
        }
        w.f.call(0x00adf080, &[message.addr()]);
        assert_eq!(
            w.sounds.calls(SOUND_SLOT_SET_ATTENUATION_CURVE),
            vec![vec![sound, 10, 20, 30, 40, 50]]
        );
    }

    #[test]
    fn min_max_and_curve_wait_for_a_sound_that_is_not_ready() {
        let mut w = world();
        w.f.e.register(MESSAGE_DESTRUCTOR, |_, _| ret(0));
        add_sound(&mut w, 5, 0, 0);
        add_sound(&mut w, 6, 0x10, 0);
        send(&mut w, [35, 5, 1, 2], [0.0; 3]);
        send(&mut w, [35, 6, 1, 2], [0.0; 3]);
        assert_eq!(w.f.messages(), vec![plain(0x23, 5, 1, 2)]);
        // The curve goes through `00adbb30`, which limits each value to 100.
        let message = fake_message(&mut w.f.e, [36, 5, 0, 0], [0.0; 3]);
        for (i, v) in [10u16, 200, 30, 40, 50].into_iter().enumerate() {
            w.f.e.mem.set_u16(message.addr() + 0x14 + 2 * i as u32, v);
        }
        w.f.call(0x00adf080, &[message.addr()]);
        let queued = w.f.messages();
        assert_eq!(queued.len(), 2);
        assert_eq!(queued[1].kind, 0x24);
        assert_eq!(queued[1].a, 5);
    }

    #[test]
    fn a_new_sound_is_loaded_through_a_task_or_swapped_for_a_cached_one() {
        let mut w = world();
        // No load queue yet: the message waits.
        send(&mut w, [37, 4, 5, 0x1234], [0.0; 3]);
        assert_eq!(w.f.messages(), vec![plain(37, 4, 5, 0x1234)]);
        let queue = fake_vtable(&mut w.f.e, 0x7220_0000);
        let queue_object = w.f.e.mem.alloc(8);
        w.f.e.mem.set_u32(queue_object, queue.address);
        w.f.e.mem.set_u32(LOAD_QUEUE_GLOBAL, queue_object);
        w.f.e.register(SOUND_CALCULATE_HASHES, |_, _| ret(1));
        w.f.e.register(LOAD_TASK_CONSTRUCTOR, |_, a| ret(a[0]));
        let fresh = fake_sound(&mut w.f.e, &w.sounds, 0);
        w.f.e.mem.set_cstr(fresh + SOUND_FILENAME, b"fresh.wav");
        send(&mut w, [37, 4, 0, fresh], [0.0; 3]);
        // A task was queued and the sound is in the list under the id.
        assert_eq!(queue.calls(LOAD_QUEUE_SLOT_ADD).len(), 1);
        assert_eq!(queue.calls(LOAD_QUEUE_SLOT_ADD)[0][0], queue_object);
        assert_eq!(w.f.e.mem.u32(fresh + SOUND_STATE_FLAGS) & 0x1000, 0x1000);
        let (found, listed) = map_get(&mut w.f.e, w.f.this.at(BSAudioManager::SoundList), 4, 0);
        assert!(found);
        assert_eq!(listed, fresh);
    }

    #[test]
    fn a_new_sound_that_is_cached_is_replaced_by_the_cached_copy() {
        let mut w = world();
        let queue = fake_vtable(&mut w.f.e, 0x7220_0000);
        let object = w.f.e.mem.alloc(8);
        w.f.e.mem.set_u32(object, queue.address);
        w.f.e.mem.set_u32(LOAD_QUEUE_GLOBAL, object);
        w.f.e.register(SOUND_CALCULATE_HASHES, |_, _| ret(1));
        let hash = 0x77;
        w.f.e.register(LIST_ADD_HEAD, fake_add_head);
        w.f.e.register(LIST_REMOVE_AT, fake_remove_at);
        // The cache holds a sound with the hash; the lookup makes a copy.
        let cached = fake_sound(&mut w.f.e, &w.sounds, 1);
        w.f.e.mem.set_u32(cached + SOUND_HASH, hash);
        list_push(&mut w.f.e, w.f.this.addr() + 0x94, cached);
        let copy = fake_sound(&mut w.f.e, &w.sounds, 0);
        w.f.e.mem.set_u32(copy + SOUND_HASH, hash);
        w.audio.answer(AUDIO_SLOT_CREATE_NEW_GAME_SOUND, ret(copy));
        let incoming = fake_sound(&mut w.f.e, &w.sounds, 0);
        w.f.e.mem.set_u32(incoming + SOUND_HASH, hash);
        w.f.e.mem.set_u32(incoming + SOUND_TYPE_FLAGS, 0x44);
        send(&mut w, [37, 4, 0, incoming], [0.0; 3]);
        assert_eq!(w.sounds.calls(SOUND_SLOT_COPY), vec![vec![copy, cached, 1]]);
        // Same hash: the incoming sound is deleted; the copy takes the type
        // flags and the id, goes into the list and gets its voice state.
        assert_eq!(
            w.sounds.calls(SOUND_SLOT_DESTRUCTOR),
            vec![vec![incoming, 1]]
        );
        assert_eq!(w.f.e.mem.u32(copy + SOUND_TYPE_FLAGS), 0x44);
        assert_eq!(w.f.e.mem.u32(copy + SOUND_ID), 4);
        assert!(has_voice_state(&w, 4));
        let (_, listed) = map_get(&mut w.f.e, w.f.this.at(BSAudioManager::SoundList), 4, 0);
        assert_eq!(listed, copy);
    }

    #[test]
    fn delete_sound_and_the_callbacks_and_small_setters() {
        let mut w = world();
        track_appends(&mut w.f);
        let sound = ready_sound(&mut w, 4, 0);
        w.f.e.mem.set_u32(sound + SOUND_STATE_FLAGS, 0x900 | 0x200);
        send(&mut w, [38, 4, 0, 0], [0.0; 3]);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_STATE_FLAGS), 0x910);
        send(&mut w, [58, 4, 0xaa, 0xbb], [0.0; 3]);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_COMPLETION_CALLBACK), 0xbb);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_COMPLETION_CONTEXT), 0xaa);
        send(&mut w, [59, 4, 0xcc, 0xdd], [0.0; 3]);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_PLAYED_CALLBACK), 0xdd);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_PLAY_CONTEXT), 0xcc);
        send(&mut w, [64, 4, 1, 0], [0.0; 3]);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_TYPE_FLAGS), 0x4_0000);
        send(&mut w, [64, 4, 0, 0], [0.0; 3]);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_TYPE_FLAGS), 0);
        send(&mut w, [65, 4, 9, 0], [0.0; 3]);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_PRIORITY), 9);
        send(&mut w, [63, 4, 777, 0], [0.0; 3]);
        assert_eq!(w.sounds.calls(SOUND_SLOT_SEEK), vec![vec![sound, 777]]);
        // A seek for a sound that is not ready waits in its queue.
        let waiting = add_sound(&mut w, 5, 0, 0);
        send(&mut w, [63, 5, 1, 0], [0.0; 3]);
        assert_eq!(
            list_elements(&w.f.e, waiting + SOUND_QUEUED_MESSAGES).len(),
            1
        );
    }

    #[test]
    fn synch_start_time_constraints_and_data_requests() {
        let mut w = world();
        let sound = ready_sound(&mut w, 4, 0);
        send(&mut w, [60, 4, 55, 0], [0.0; 3]);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_SYNCH_START_TIME), 55);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_TYPE_FLAGS), 0x100_0000);
        send(&mut w, [61, 4, 0, 0], [0.0; 3]);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_SYNCH_START_TIME), 0);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_TYPE_FLAGS), 0);
        // Without state bit 0x100 the messages wait, unless marked 0x10.
        add_sound(&mut w, 5, 0, 0);
        add_sound(&mut w, 6, 0x10, 0);
        send(&mut w, [60, 5, 1, 2], [0.0; 3]);
        send(&mut w, [61, 5, 1, 2], [0.0; 3]);
        send(&mut w, [60, 6, 1, 2], [0.0; 3]);
        assert_eq!(w.f.messages(), vec![plain(60, 5, 1, 2), plain(61, 5, 1, 2)]);
        send(&mut w, [62, 4, 16, 0x1_0020], [0.0; 3]);
        assert_eq!(w.f.e.mem.f32(sound + SOUND_BEGIN_TIME), 1.5);
        assert_eq!(w.f.e.mem.f32(sound + SOUND_END_TIME), 3.0);
        send(&mut w, [57, 4, 12, 0], [0.0; 3]);
        assert_eq!(
            w.sounds.calls(SOUND_SLOT_PROCESS_DATA_REQUEST),
            vec![vec![sound, 12]]
        );
        send(&mut w, [57, 5, 13, 0], [0.0; 3]);
        send(&mut w, [57, 6, 14, 0], [0.0; 3]);
        assert_eq!(w.f.messages()[2], plain(0x39, 5, 13, 0));
        assert_eq!(w.f.messages().len(), 3);
    }

    #[test]
    fn moving_sounds_are_registered_stopped_and_purged() {
        let mut w = world();
        let map = moving_map_model(&mut w.f);
        let released = Rc::new(RefCell::new(Vec::new()));
        let log = released.clone();
        w.f.e.register_double(RELEASE_MESSAGE_POINTER, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(0)
        });
        let sound = ready_sound(&mut w, 4, 0x4);
        let object = fake_object(&mut w.f.e, 1, [0.0; 3]);
        let message = fake_message(&mut w.f.e, [39, 4, 0, 0], [0.0; 3]);
        w.f.e.mem.set_u32(message.addr() + 0x10, object);
        w.f.call(0x00adf080, &[message.addr()]);
        // The id follows the object now; the message let go of it.
        let node = map_node(&w.f.e, map.addr(), 4).unwrap();
        assert_eq!(w.f.e.mem.u32(node + 8), object);
        assert_eq!(w.f.e.mem.u32(message.addr() + 0x10), 0);
        // Stopping the sounds that follow that object: a mask of 0x4.
        let stop = fake_message(&mut w.f.e, [41, 0, 0, 0x4], [0.0; 3]);
        w.f.e.mem.set_u32(stop.addr() + 0x10, object);
        w.f.call(0x00adf080, &[stop.addr()]);
        assert_eq!(w.sounds.calls(SOUND_SLOT_STOP), vec![vec![sound]]);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_TYPE_FLAGS) & 0x100, 0x100);
        // Purging removes the entry and stops the sound again.
        send(&mut w, [40, 0, 0, 0], [0.0; 3]);
        assert!(map_node(&w.f.e, map.addr(), 4).is_none());
        assert_eq!(w.sounds.calls(SOUND_SLOT_STOP).len(), 2);
        assert_eq!(w.f.e.mem.u32(sound + SOUND_STATE_FLAGS) & 0x210, 0x10);
    }

    #[test]
    fn the_cache_message_caches_and_deletes_the_sound() {
        let mut w = world();
        w.f.e.mem.set_u8(CACHE_ENABLED_SETTING, 0);
        let sound = fake_sound(&mut w.f.e, &w.sounds, 4);
        send(&mut w, [51, 0, 0, sound], [0.0; 3]);
        assert_eq!(w.sounds.calls(SOUND_SLOT_DESTRUCTOR), vec![vec![sound, 1]]);
        // A null sound does nothing.
        send(&mut w, [51, 0, 0, 0], [0.0; 3]);
        assert_eq!(w.sounds.calls(SOUND_SLOT_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn underwater_and_listener_position_messages() {
        let mut w = world();
        let steps = Rc::new(RefCell::new(Vec::new()));
        let log = steps.clone();
        w.f.e.register_double(SOUND_ENVIRONMENT_STEP, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(0)
        });
        w.f.e
            .set(w.f.this, BSAudioManager::bListenerUnderwater, true);
        let marked = ready_sound(&mut w, 4, 0x1000);
        ready_sound(&mut w, 5, 0);
        add_sound(&mut w, 6, 0x10, 0x1000);
        send(&mut w, [55, 0, 0, 0], [0.0; 3]);
        assert_eq!(*steps.borrow(), vec![vec![marked, 1]]);
        // No listener: nothing; with one the position is passed on.
        send(&mut w, [56, 0, 0, 0], [1.0, 2.0, 3.0]);
        assert!(!w.f.e.get(w.f.this, BSAudioManager::bListenerMoved));
        let (_listener, listener_vtable) = install_listener(&mut w.f);
        send(&mut w, [56, 0, 0, 0], [1.0, 2.0, 3.0]);
        let call = listener_vtable.calls(LISTENER_SLOT_SET_POSITION)[0].clone();
        assert_eq!(call.len(), 2, "one word: the vector's address");
        assert_eq!(
            [0, 4, 8].map(|k| w.f.e.mem.f32(call[1] + k)),
            [1.0, 2.0, 3.0]
        );
        assert!(w.f.e.get(w.f.this, BSAudioManager::bListenerMoved));
    }

    #[test]
    fn thread_messages_go_to_the_garbage_stack_and_stop_or_suspend_the_thread() {
        let mut w = world();
        let pushed = Rc::new(RefCell::new(Vec::new()));
        let log = pushed.clone();
        w.f.e.register_double(MESSAGE_STACK_PUSH, move |_, a| {
            log.borrow_mut().push((a[0], a[1]));
            ret(0)
        });
        let breaks = Rc::new(RefCell::new(0));
        let count = breaks.clone();
        w.f.e.register_double(IMPORT_DEBUG_BREAK, move |_, _| {
            *count.borrow_mut() += 1;
            ret(0)
        });
        // Not multi-threaded and on the "wrong" thread: only a break.
        let first = fake_message(&mut w.f.e, [52, 0, 0, 0], [0.0; 3]);
        assert!(!w.f.call(0x00adf080, &[first.addr()]).bool());
        let second = fake_message(&mut w.f.e, [53, 0, 0, 0], [0.0; 3]);
        assert!(!w.f.call(0x00adf080, &[second.addr()]).bool());
        let garbage = w.f.this.addr() + 0x44;
        assert_eq!(
            *pushed.borrow(),
            vec![(garbage, first.addr()), (garbage, second.addr())]
        );
        assert_eq!(*breaks.borrow(), 1);
        // On the audio thread: the flag is cleared and the steps run.
        let audio = w.f.e.global::<u32>(AUDIO_GLOBAL);
        let thread = w.f.e.mem.alloc(0x4c);
        w.f.e.mem.set_u32(w.f.this.addr() + 0x13c, thread);
        w.f.e.mem.set_u32(w.f.this.addr() + 0x138, 1);
        w.f.e.mem.set_u8(audio + 6, 1);
        w.f.e.register(IMPORT_RELEASE_SEMAPHORE, |_, _| ret(1));
        w.f.e.register(THREAD_STOP_STEP, |_, _| ret(0));
        w.f.e.register(IMPORT_WAIT_FOR_SINGLE_OBJECT, |_, _| ret(0));
        let third = fake_message(&mut w.f.e, [52, 0, 0, 0], [0.0; 3]);
        assert!(!w.f.call(0x00adf080, &[third.addr()]).bool());
        assert_eq!(w.f.e.mem.u8(audio + 6), 0);
        assert_eq!(w.f.e.mem.u8(thread + 0x48), 1);
        w.f.e.mem.set_u8(audio + 6, 1);
        let fourth = fake_message(&mut w.f.e, [53, 0, 0, 0], [0.0; 3]);
        assert!(w.f.call(0x00adf080, &[fourth.addr()]).bool());
        assert_eq!(w.f.e.mem.u8(audio + 6), 0);
        // The suspended flag was set, then cleared when the thread resumed.
        assert_eq!(w.f.e.mem.u8(thread + 0x48), 0);
    }

    // ---- sound handles, debug strings, settings, tasks, threads, stacks, maps, lists ----

    /// The handle at `result` as (id, assume-success byte, state).
    fn handle(e: &Engine, result: u32) -> (u32, u8, u32) {
        (
            e.mem.u32(result),
            e.mem.u8(result + 4),
            e.mem.u32(result + 8),
        )
    }

    /// A world for the sound-handle cases: audio enabled, the sound
    /// creation answering `sound`, a listener-less `BSAudio`.
    fn handle_world() -> (Fixture, FakeVtable, u32, u32) {
        let (mut f, audio) = audio_fixture();
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        let object = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem.set_u8(object + AUDIO_ENABLED, 1);
        f.e.register(DEBUG_PRINT, |_, _| ret(0));
        let sound = fake_sound(&mut f.e, &sounds, 0);
        audio.answer(AUDIO_SLOT_CREATE_NEW_GAME_SOUND, ret(sound));
        let result = f.e.mem.alloc(12);
        (f, audio, sound, result)
    }

    #[test]
    fn get_sound_handle_by_filename_creates_the_sound_and_its_state() {
        let (mut f, audio, sound, result) = handle_world();
        f.e.mem.set_f32(AUDIO_DEBUG_DELAY_SETTING, 0.5);
        f.e.set(f.this, BSAudioManager::iNextID, 9);
        let name = text(&mut f.e, b"x.wav");
        let returned = f.call(0x00ae5a50, &[result, name, 2, 0]).u32();
        assert_eq!(returned, result);
        assert_eq!(handle(&f.e, result), (9, 0, 0));
        assert_eq!(f.e.mem.u32(sound + SOUND_TYPE_FLAGS), 2);
        assert_eq!(f.e.mem.u32(sound + SOUND_ID), 9);
        assert_eq!(f.e.mem.u32(sound + SOUND_CLEANUP_DELAY), 500);
        assert_eq!(f.e.get(f.this, BSAudioManager::iNextID), 10);
        assert_eq!(
            audio.calls(AUDIO_SLOT_CREATE_NEW_GAME_SOUND),
            vec![vec![f.e.global::<u32>(AUDIO_GLOBAL), name]]
        );
        // A `BSSoundInfo` record for the id is in the state map; message
        // 0x25 hands the sound over.
        assert!(map_node(&f.e, f.this.addr() + 0x64, 9).is_some());
        assert_eq!(f.messages(), vec![plain(0x25, 9, 0, sound)]);
    }

    #[test]
    fn get_sound_handle_by_filename_refuses_empty_names_and_disabled_audio() {
        let (mut f, audio, _sound, result) = handle_world();
        let logged = Rc::new(RefCell::new(Vec::new()));
        let log = logged.clone();
        f.e.register_double(DEBUG_PRINT, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(0)
        });
        let empty = text(&mut f.e, b"");
        f.call(0x00ae5a50, &[result, empty, 1, 0]);
        assert_eq!(handle(&f.e, result), (0xffff_ffff, 0, 0));
        assert_eq!(*logged.borrow(), vec![vec![NO_FILENAME_MESSAGE]]);
        // The id was taken anyway.
        assert_eq!(f.e.get(f.this, BSAudioManager::iNextID), 1);
        // Neither the 2D nor the 3D flag: logged, and the 2D flag is added.
        let name = text(&mut f.e, b"y.wav");
        f.call(0x00ae5a50, &[result, name, 0, 0]);
        assert_eq!(logged.borrow()[1], vec![NO_DIMENSION_MESSAGE, name]);
        assert_eq!(audio.calls(AUDIO_SLOT_CREATE_NEW_GAME_SOUND).len(), 1);
        let object = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem.set_u8(object + AUDIO_ENABLED, 0);
        f.call(0x00ae5a50, &[result, name, 1, 0]);
        assert_eq!(handle(&f.e, result), (0xffff_ffff, 0, 0));
        assert_eq!(f.e.get(f.this, BSAudioManager::iNextID), 2);
    }

    #[test]
    fn get_sound_handle_by_filename_uses_the_settings_and_loop_callbacks() {
        let (mut f, audio, sound, result) = handle_world();
        let object = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem
            .set_u32(object + AUDIO_SETTINGS_CALLBACK, 0x7400_0020);
        let calls = Rc::new(RefCell::new(Vec::new()));
        let log = calls.clone();
        f.e.register_double(0x7400_0020, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            if a[2] != 0 {
                e.mem.set_u32(a[2], 7);
            }
            ret(0x800_0000 | 0x10)
        });
        audio.answer(AUDIO_SLOT_GET_LOOP_POINT_CALLBACK, ret(0x7400_0030));
        f.e.register(0x7400_0030, |e, a| {
            e.mem.set_u32(a[1], 100);
            e.mem.set_u32(a[2], 200);
            ret(1)
        });
        let name = text(&mut f.e, b"z.wav");
        f.call(0x00ae5a50, &[result, name, 2, 5]);
        // Flag changes from the callback: bit 0x8000000 sets 0x1 and drops
        // 0x2, bit 0x10 is taken over.
        assert_eq!(f.e.mem.u32(sound + SOUND_TYPE_FLAGS), 0x800_0000 | 0x10 | 1);
        assert_eq!(f.e.mem.u32(sound + SOUND_MODIFIER_FLAGS), 7);
        assert_eq!(f.e.mem.u32(sound + SOUND_LOOP_START), 100);
        assert_eq!(f.e.mem.u32(sound + SOUND_LOOP_END), 200);
        // The callback ran twice: with an output word and then without.
        assert_eq!(calls.borrow().len(), 2);
        assert_eq!(calls.borrow()[1][1..], [5, 0]);
    }

    #[test]
    fn get_sound_handle_by_filename_keeps_its_locals_at_the_exe_frame_offsets() {
        let (mut f, audio, _sound, result) = handle_world();
        let object = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem
            .set_u32(object + AUDIO_SETTINGS_CALLBACK, 0x7400_0020);
        let calls = Rc::new(RefCell::new(Vec::new()));
        let log = calls.clone();
        f.e.register_double(0x7400_0020, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(0)
        });
        audio.answer(AUDIO_SLOT_GET_LOOP_POINT_CALLBACK, ret(0x7400_0030));
        let points = Rc::new(RefCell::new(Vec::new()));
        let log = points.clone();
        f.e.register_double(0x7400_0030, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(0)
        });
        let name = text(&mut f.e, b"z.wav");
        let frame = f.e.next_stack_local(0xc4);
        f.call(0x00ae5a50, &[result, name, 2, 5]);
        // The handle at +0x94, the settings word at +0xac, the loop points at +0xa0 / +0xb0.
        assert_eq!(calls.borrow()[0], vec![frame + 0x94, 5, frame + 0xac]);
        assert_eq!(calls.borrow()[1], vec![frame + 0x94, 5, 0]);
        assert_eq!(*points.borrow(), vec![vec![5, frame + 0xa0, frame + 0xb0]]);
    }

    #[test]
    fn get_sound_handle_by_name_asks_the_name_callback() {
        let (mut f, _audio, sound, result) = handle_world();
        let object = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem.set_u32(object + AUDIO_NAME_CALLBACK, 0x7400_0040);
        f.e.register(0x7400_0040, |e, a| {
            e.mem.set_cstr(a[1], b"named.wav");
            e.mem.set_u32(a[2], 0x200_0000);
            e.mem.set_u32(a[3], 0);
            ret(1)
        });
        let name = text(&mut f.e, b"anything");
        assert_eq!(f.call(0x00ae5680, &[result, name, 2]).u32(), result);
        // Callback bit 0x2000000 sets it in the flags and clears 0x10.
        assert_eq!(f.e.mem.u32(sound + SOUND_TYPE_FLAGS), 0x200_0002);
        assert_eq!(handle(&f.e, result).0, 0);
        // An empty file name from the callback gives the empty handle.
        f.e.register(0x7400_0040, |e, a| {
            e.mem.set_cstr(a[1], b"");
            ret(1)
        });
        f.call(0x00ae5680, &[result, name, 2]);
        assert_eq!(handle(&f.e, result), (0xffff_ffff, 0, 0));
        // So does a callback that says no, a missing one or disabled audio.
        f.e.register(0x7400_0040, |_, _| ret(0));
        f.call(0x00ae5680, &[result, name, 2]);
        assert_eq!(handle(&f.e, result).0, 0xffff_ffff);
        f.e.mem.set_u32(object + AUDIO_NAME_CALLBACK, 0);
        f.e.mem.set_u32(result, 5);
        f.call(0x00ae5680, &[result, name, 2]);
        assert_eq!(handle(&f.e, result).0, 0xffff_ffff);
        f.e.mem.set_u8(object + AUDIO_ENABLED, 0);
        f.e.mem.set_u32(result, 5);
        f.call(0x00ae5680, &[result, name, 2]);
        assert_eq!(handle(&f.e, result).0, 0xffff_ffff);
    }

    #[test]
    fn get_sound_handle_by_numeric_id_adjusts_the_flags_its_own_way() {
        let (mut f, _audio, sound, result) = handle_world();
        let object = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem.set_u32(object + AUDIO_ID_CALLBACK, 0x7400_0050);
        f.e.register(0x7400_0050, |e, a| {
            e.mem.set_cstr(a[1], b"id.wav");
            e.mem.set_u32(a[2], 1);
            e.mem.set_u32(a[3], 0);
            ret(1)
        });
        f.call(0x00ae5870, &[result, 4321, 2]);
        // Callback bit 1 with caller bit 2: 2 becomes 1.
        assert_eq!(f.e.mem.u32(sound + SOUND_TYPE_FLAGS), 1);
        f.e.register(0x7400_0050, |e, a| {
            e.mem.set_cstr(a[1], b"id.wav");
            e.mem.set_u32(a[2], 0x800_0000 | 0x400_0000);
            ret(1)
        });
        f.call(0x00ae5870, &[result, 4321, 2 | 0x10]);
        assert_eq!(
            f.e.mem.u32(sound + SOUND_TYPE_FLAGS),
            0x800_0000 | 0x400_0000 | 1
        );
        // A callback that says no, or a missing one, gives the empty handle.
        f.e.register(0x7400_0050, |_, _| ret(0));
        f.call(0x00ae5870, &[result, 4321, 2]);
        assert_eq!(handle(&f.e, result), (0xffff_ffff, 0, 0));
        f.e.mem.set_u32(object + AUDIO_ID_CALLBACK, 0);
        f.e.mem.set_u32(result, 0);
        f.call(0x00ae5870, &[result, 4321, 2]);
        assert_eq!(handle(&f.e, result).0, 0xffff_ffff);
    }

    #[test]
    fn the_empty_handle_and_the_line_comparison() {
        let mut e = engine();
        let this: Ptr<BSAudioManager> = e.new_object();
        let result = e.mem.alloc(12);
        e.mem.write(result, &[7; 12]);
        let returned = e
            .call(0x00ae5f70, &[this.addr(), result, 1, 2, 3, 4, 5, 6])
            .u32();
        assert_eq!(returned, result);
        assert_eq!(handle(&e, result), (0xffff_ffff, 0, 0));
        let first = e.mem.alloc(4);
        let second = e.mem.alloc(4);
        e.mem.set_u32(first, 3);
        e.mem.set_u32(second, 5);
        assert_eq!(e.call(0x00ae5fb0, &[first, second]).i32(), -1);
        assert_eq!(e.call(0x00ae5fb0, &[second, first]).i32(), 1);
        assert_eq!(e.call(0x00ae5fb0, &[first, first]).i32(), 0);
        // Unsigned: 0xffffffff is the larger.
        e.mem.set_u32(second, 0xffff_ffff);
        assert_eq!(e.call(0x00ae5fb0, &[first, second]).i32(), -1);
    }

    // ---- GetDebugString ----

    /// A small `sprintf` for the formats of the debug lines: `%i`, `%s` and
    /// `%[.N]f` (a double, two words).
    fn fake_sprintf(e: &mut Engine, a: &[u32]) -> Ret {
        let format = e.mem.cstr(a[1]);
        let mut out = Vec::new();
        let mut arg = 2;
        let mut i = 0;
        while i < format.len() {
            if format[i] != b'%' {
                out.push(format[i]);
                i += 1;
                continue;
            }
            i += 1;
            let mut precision = 6;
            if format[i] == b'.' {
                i += 1;
                precision = 0;
                while format[i].is_ascii_digit() {
                    precision = precision * 10 + (format[i] - b'0') as usize;
                    i += 1;
                }
            }
            match format[i] {
                b'i' => out.extend(format!("{}", a[arg] as i32).bytes()),
                b's' => out.extend(e.mem.cstr(a[arg])),
                b'f' => {
                    let value = f64::from_bits((a[arg + 1] as u64) << 32 | a[arg] as u64);
                    out.extend(format!("{:.*}", precision, value).bytes());
                    arg += 1;
                }
                other => panic!("format %{}", other as char),
            }
            arg += 1;
            i += 1;
        }
        e.mem.set_cstr(a[0], &out);
        ret(out.len() as u32)
    }

    /// `BSSimpleList::AddTail` (`00af25b0`): `(head node, &element)`.
    fn fake_simple_list_add(e: &mut Engine, a: &[u32]) -> Ret {
        let element = e.mem.u32(a[1]);
        if e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0 {
            e.mem.set_u32(a[0], element);
            return ret(0);
        }
        let mut last = a[0];
        while e.mem.u32(last + 4) != 0 {
            last = e.mem.u32(last + 4);
        }
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, element);
        e.mem.set_u32(last + 4, node);
        ret(0)
    }

    /// The lines of a debug list as (index, text), in list order.
    fn debug_lines(e: &Engine, list: u32) -> Vec<(u32, String)> {
        let mut lines = Vec::new();
        let mut node = list;
        while node != 0 {
            let item = e.mem.u32(node);
            if item != 0 {
                let text = String::from_utf8(e.mem.cstr(e.mem.u32(item + 4))).unwrap();
                lines.push((e.mem.u32(item), text));
            }
            node = e.mem.u32(node + 4);
        }
        lines
    }

    /// A sound state record in the state map (+0x64) with a debug block
    /// the getter hands out.
    fn debug_record(
        f: &mut Fixture,
        blocks: &Rc<RefCell<std::collections::HashMap<u32, u32>>>,
        id: u32,
        kind: u32,
        playing: u8,
        name: &[u8],
        distance: f32,
    ) {
        let record = f.e.mem.alloc(0x254);
        f.e.mem.set_u32(record, id);
        f.e.mem.set_u32(record + 0xc, kind);
        f.e.mem.set_u8(record + 0x14, playing);
        let block = f.e.mem.alloc(0x23c);
        f.e.mem.set_u32(block + 0x108, block + 0x10c);
        f.e.mem.set_cstr(block + 0x10c, name);
        f.e.mem.set_f32(block + 0x210, 1.0);
        f.e.mem.set_f32(block + 0x214, 99.0);
        f.e.mem.set_f32(block + 0x220, distance);
        f.e.mem.set_u32(block + 0x230, 3);
        f.e.mem.set_u32(block + 0x234, 4);
        f.e.mem.set_u32(block + 0x238, 5);
        blocks.borrow_mut().insert(record, block);
        let map = f.this.addr() + 0x64;
        fake_set_value(&mut f.e, &[map, id, record]);
    }

    /// The doubles of the debug-string cases.
    fn debug_doubles(f: &mut Fixture) -> Rc<RefCell<std::collections::HashMap<u32, u32>>> {
        let blocks = Rc::new(RefCell::new(std::collections::HashMap::new()));
        let source = blocks.clone();
        f.e.register_double(DEBUG_INFO_GETTER, move |_, a| ret(source.borrow()[&a[0]]));
        f.e.register(SPRINTF, fake_sprintf);
        f.e.register(STRCAT, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[1]));
            e.mem.set_cstr(a[0], &text);
            ret(a[0])
        });
        f.e.register(STRSTR, |e, a| {
            let hay = e.mem.cstr(a[0]);
            let needle = e.mem.cstr(a[1]);
            match hay
                .windows(needle.len())
                .position(|w| w == needle.as_slice())
            {
                Some(at) => ret(a[0] + at as u32),
                None => ret(0),
            }
        });
        f.e.register(SIMPLE_LIST_ADD, fake_simple_list_add);
        f.e.mem.set_cstr(BACKSLASH_TEXT, b"\\");
        for (address, value) in [
            (DEBUG_PAGE_FORMAT, &b"Page %i"[..]),
            (DEBUG_HEADER_FORMAT, b"ID\tIsPlaying?"),
            (
                DEBUG_3D_FORMAT,
                b"%i:%s\t [%s][Dist:%.2f] [%.0f/%.0f] [ST:%i][SY:%i][D:%i]",
            ),
            (DEBUG_2D_FORMAT, b"%i:%s\t [%s][2D] [ST:%i][SY:%i]"),
            (
                DEBUG_2D_RADIUS_FORMAT,
                b"%i:%s\t [%s][2DRADIUS][Dist:%.2f][%.0f/%.0f][ST:%i][SY:%i][D:%i]",
            ),
            (DEBUG_ERROR_FORMAT, b"%i:%s\t [%s][ERROR] [ST:%i][SY:%i]"),
            (DEBUG_PLAIN_FORMAT, b"%i\t%s"),
            (STOPPED_TEXT, b"Stopped"),
            (PLAYING_TEXT, b"Playing"),
            (STOP_TEXT, b"Stop"),
            (PLAY_TEXT, b"Play"),
            (0x010a_36b4, b"(3D only)"),
            (0x010a_3608, b"(Stopped or Silent only)"),
        ] {
            f.e.mem.set_cstr(address, value);
        }
        f.e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            ret(0)
        });
        blocks
    }

    #[test]
    fn the_debug_string_lists_the_playing_sounds_sorted_by_index() {
        let (mut f, _audio) = audio_fixture();
        let blocks = debug_doubles(&mut f);
        debug_record(&mut f, &blocks, 3, 2, 1, b"data\\sound\\b.wav", 12.5);
        debug_record(&mut f, &blocks, 4, 2, 0, b"data\\sound\\c.wav", 1.0);
        debug_record(&mut f, &blocks, 6, 1, 1, b"data\\sound\\d.wav", 1.0);
        debug_record(&mut f, &blocks, 7, 0x8000001, 1, b"data\\sound\\e.wav", 2.0);
        debug_record(&mut f, &blocks, 8, 0x10, 1, b"data\\sound\\f.wav", 2.0);
        let list = f.call(0x00ae5ff0, &[0]).u32();
        assert_eq!(
            debug_lines(&f.e, list),
            vec![
                (0, "Page 1".to_string()),
                (1, "ID\tIsPlaying?".to_string()),
                (
                    5,
                    "3:\\b.wav\t [Play][Dist:12.50] [1/99] [ST:3][SY:4][D:5]".to_string()
                ),
                (8, "6:\\d.wav\t [Play][2D] [ST:3][SY:4]".to_string()),
                (
                    9,
                    "7:\\e.wav\t [Play][2DRADIUS][Dist:2.00][1/99][ST:3][SY:4][D:5]".to_string()
                ),
                (10, "8:\\f.wav\t [Play][ERROR] [ST:3][SY:4]".to_string()),
            ]
        );
    }

    #[test]
    fn the_debug_sound_info_sits_0x494_under_the_frames_ebp() {
        let (mut f, _audio) = audio_fixture();
        let blocks = debug_doubles(&mut f);
        debug_record(&mut f, &blocks, 3, 2, 1, b"p\\a.wav", 1.0);
        let frame = f.e.next_stack_local(0xa90);
        f.e.call_log = Some(vec![]);
        f.call(0x00ae5ff0, &[0]);
        let log = f.e.call_log.take().unwrap();
        let built: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == DEBUG_SOUND_INFO_CONSTRUCTOR)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(built, vec![frame + 0xa90 - 0x494]);
    }

    #[test]
    fn debug_pages_filter_by_type_and_the_stopped_page_lists_the_others() {
        let (mut f, _audio) = audio_fixture();
        let blocks = debug_doubles(&mut f);
        debug_record(&mut f, &blocks, 3, 2, 1, b"p\\a.wav", 1.0);
        debug_record(&mut f, &blocks, 4, 4, 1, b"p\\b.wav", 1.0);
        debug_record(&mut f, &blocks, 5, 2, 0, b"p\\c.wav", 1.0);
        let list = f.call(0x00ae5ff0, &[1]).u32();
        let lines = debug_lines(&f.e, list);
        assert_eq!(lines[0], (0, "Page 2(3D only)".to_string()));
        // Only the playing sound with type bit 2.
        assert_eq!(lines.len(), 3);
        assert!(lines[2].1.starts_with("3:\\a.wav"));
        // The stopped page: the sound that is not playing, whatever its type.
        let list = f.call(0x00ae5ff0, &[10]).u32();
        let lines = debug_lines(&f.e, list);
        assert_eq!(lines[0].1, "Page 11(Stopped or Silent only)");
        assert_eq!(lines.len(), 3);
        assert!(lines[2].1.starts_with("5:\\c.wav"));
        assert!(lines[2].1.contains("[Stop]"));
        // Page 99 has no header lines and filters on type bit 0x800.
        let list = f.call(0x00ae5ff0, &[99]).u32();
        assert!(debug_lines(&f.e, list).is_empty());
    }

    #[test]
    fn a_sound_without_a_name_gets_a_short_line() {
        let (mut f, _audio) = audio_fixture();
        let blocks = debug_doubles(&mut f);
        debug_record(&mut f, &blocks, 3, 2, 1, b"x", 1.0);
        // No backslash in the name: the line is skipped.
        let list = f.call(0x00ae5ff0, &[0]).u32();
        assert_eq!(debug_lines(&f.e, list).len(), 2);
        // A debug record with no name pointer gets `id\tstate`.
        let (record, block) = {
            let map = blocks.borrow();
            let (r, b) = map.iter().next().unwrap();
            (*r, *b)
        };
        let _ = record;
        f.e.mem.set_u32(block + 0x108, 0);
        let list = f.call(0x00ae5ff0, &[0]).u32();
        assert_eq!(debug_lines(&f.e, list)[2], (5, "3\tPlaying".to_string()));
    }

    // ---- settings and the pause table ----

    #[test]
    fn the_cache_size_setting_is_limited_and_zero_turns_the_cache_off() {
        let mut e = engine2();
        let this: Ptr<BSAudioManager> = e.new_object();
        let logged = Rc::new(RefCell::new(Vec::new()));
        let log = logged.clone();
        e.register_double(DEBUG_PRINT, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(0)
        });
        e.call(0x00ae6bf0, &args![this, 1024u32]);
        assert_eq!(e.call(0x00ae6c60, &args![this]).u32(), 1024);
        // Too large: logged and ignored.
        e.call(0x00ae6bf0, &args![this, 5000u32]);
        assert_eq!(*logged.borrow(), vec![vec![CACHE_SIZE_MESSAGE, 5000]]);
        assert_eq!(e.call(0x00ae6c60, &args![this]).u32(), 1024);
        // Zero switches the cache off (and flushes it).
        assert_eq!(e.call(0x00ae6cf0, &args![this]).u8(), 1);
        e.call(0x00ae6bf0, &args![this, 0u32]);
        assert_eq!(e.call(0x00ae6cf0, &args![this]).u8(), 0);
        assert_eq!(e.call(0x00ae6c60, &args![this]).u32(), 0);
        // Setting it again does not flush.
        e.call(0x00ae6ca0, &args![this, 1u8]);
        assert_eq!(e.call(0x00ae6cf0, &args![this]).u8(), 1);
    }

    #[test]
    fn the_pause_table_holds_masks_in_two_modes() {
        let mut e = engine2();
        let this: Ptr<BSAudioManager> = e.new_object();
        let table = e.mem.alloc(16);
        e.set(this, BSAudioManager::pPauseEntries, table);
        e.set(this, BSAudioManager::iPauseListSize, 2);
        assert!(!e.call(0x00ae6d30, &args![this, 0x6u32]).bool());
        // Any-bit mode and exact mode.
        assert!(e.call(0x00ae6de0, &args![this, 0x6u32, 0u8]).bool());
        assert!(e.call(0x00ae6de0, &args![this, 0x18u32, 1u8]).bool());
        // The table is full now.
        assert!(!e.call(0x00ae6de0, &args![this, 0x1u32, 0u8]).bool());
        assert!(e.call(0x00ae6d30, &args![this, 0x2u32]).bool());
        assert!(!e.call(0x00ae6d30, &args![this, 0x8u32]).bool());
        assert!(e.call(0x00ae6d30, &args![this, 0x1cu32]).bool());
        // Removal needs the mask and the mode.
        assert!(!e.call(0x00ae6e60, &args![this, 0x6u32, 1u8]).bool());
        assert!(e.call(0x00ae6e60, &args![this, 0x6u32, 0u8]).bool());
        assert!(!e.call(0x00ae6d30, &args![this, 0x2u32]).bool());
        assert!(e.call(0x00ae6e60, &args![this, 0x18u32, 1u8]).bool());
        assert_eq!(e.mem.bytes(table, 16), vec![0; 16]);
    }

    // ---- load tasks ----

    #[test]
    fn the_load_task_is_built_from_its_sound() {
        let mut e = engine2();
        let task: Ptr<AudioLoadTask> = e.new_object();
        let sound = e.mem.alloc(0x198);
        e.mem.set_cstr(sound + SOUND_FILENAME, b"data\\x.wav");
        let calls = Rc::new(RefCell::new(Vec::new()));
        let log = calls.clone();
        e.register_double(LOAD_TASK_BASE_CONSTRUCTOR, move |_, a| {
            log.borrow_mut().push(("base", a.to_vec()));
            ret(a[0])
        });
        e.register(FILE_NAME_FIX, |e, a| {
            let name = e.mem.cstr(a[0]);
            assert_eq!(a[2], 0x104);
            e.mem.set_cstr(a[1], &name);
            ret(0)
        });
        let log = calls.clone();
        e.register_double(LOAD_TASK_SET_FILE_NAME, move |e, a| {
            let name = e.mem.cstr(a[1]);
            log.borrow_mut()
                .push(("name", vec![a[0], name.len() as u32]));
            ret(0)
        });
        let log = calls.clone();
        e.register_double(LOAD_TASK_SET_CONTEXT, move |_, a| {
            log.borrow_mut().push(("context", a.to_vec()));
            ret(0)
        });
        assert_eq!(e.call(0x00ae6f50, &args![task, sound]).u32(), task.addr());
        assert_eq!(e.mem.u32(task.addr()), AUDIO_LOAD_TASK_VTABLE);
        assert_eq!(e.get(task, AudioLoadTask::pSound), sound);
        assert_eq!(
            *calls.borrow(),
            vec![
                ("base", vec![task.addr(), 0]),
                ("name", vec![task.addr(), 10]),
                ("context", vec![task.addr(), 3]),
            ]
        );
    }

    #[test]
    fn the_load_task_destructor_resets_the_vtable_and_runs_the_base() {
        let mut e = engine2();
        let task: Ptr<AudioLoadTask> = e.new_object();
        let ran = Rc::new(RefCell::new(Vec::new()));
        let log = ran.clone();
        e.register_double(LOAD_TASK_BASE_DESTRUCTOR, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(0)
        });
        e.call(0x00ae7020, &args![task]);
        assert_eq!(e.mem.u32(task.addr()), AUDIO_LOAD_TASK_VTABLE);
        assert_eq!(*ran.borrow(), vec![task.addr()]);
        // The scalar deleting destructor frees the task when asked to.
        e.register(LOAD_TASK_BASE_DESTRUCTOR, |_, _| ret(0));
        let block = e.mem.alloc(0x38);
        e.call(0x00ae7b10, &[block, 1]);
        assert_eq!(e.mem.block_size(block), None);
        let kept = e.mem.alloc(0x38);
        assert_eq!(e.call(0x00ae7b10, &[kept, 0]).u32(), kept);
        assert_eq!(e.mem.block_size(kept), Some(0x38));
    }

    /// The fixture of the load-task cases: the singleton is built, the
    /// sound's vtable is faked.
    fn task_world() -> (Fixture, FakeVtable, u32, Ptr<AudioLoadTask>) {
        let (mut f, _audio) = audio_fixture();
        f.e.mem.set_u32(MANAGER_BUILT_GUARD, 1);
        f.e.register(LIST_REMOVE_AT, fake_remove_at);
        f.e.register(SOUND_CALCULATE_HASHES, |e, a| {
            e.mem.set_u32(a[0] + SOUND_HASH, 0x55);
            ret(1)
        });
        let sounds = fake_vtable(&mut f.e, 0x7210_0000);
        let sound = fake_sound(&mut f.e, &sounds, 1);
        let task: Ptr<AudioLoadTask> = f.e.new_object();
        f.e.set(task, AudioLoadTask::pSound, sound);
        (f, sounds, sound, task)
    }

    #[test]
    fn running_a_task_for_a_sound_that_is_cached_by_directory() {
        let (mut f, sounds, sound, task) = task_world();
        f.e.mem.set_u32(sound + SOUND_DIRECTORY_HASH, 9);
        f.e.mem.set_u32(sound + SOUND_STATE_FLAGS, 0x1200);
        let cached = fake_sound(&mut f.e, &sounds, 2);
        f.e.mem.set_u32(cached + SOUND_HASH, 0x55);
        list_push(&mut f.e, MANAGER_INSTANCE + 0x94, cached);
        f.e.call(0x00ae7050, &args![task]);
        // The hashes were calculated, the sound is "ready" (0x10), not opened.
        assert_eq!(f.e.mem.u32(sound + SOUND_HASH), 0x55);
        assert_eq!(f.e.mem.u32(sound + SOUND_STATE_FLAGS), 0x10);
        assert!(sounds.calls(SOUND_SLOT_OPEN).is_empty());
        // A sound with type bit 0x8000 is deleted instead.
        f.e.mem.set_u32(sound + SOUND_HASH, 0);
        f.e.mem.set_u32(sound + SOUND_TYPE_FLAGS, 0x8000);
        f.e.call(0x00ae7050, &args![task]);
        assert_eq!(sounds.calls(SOUND_SLOT_DESTRUCTOR), vec![vec![sound, 1]]);
    }

    #[test]
    fn running_a_task_opens_the_sound_and_announces_a_cached_one() {
        let (mut f, sounds, sound, task) = task_world();
        f.e.clock_ms = 5000;
        f.e.mem.set_u32(sound + SOUND_HASH, 0x77);
        f.e.mem.set_u32(sound + SOUND_TYPE_FLAGS, 0x8000);
        sounds.answer(SOUND_SLOT_OPEN, ret(1));
        f.e.call(0x00ae7050, &args![task]);
        assert_eq!(sounds.calls(SOUND_SLOT_OPEN), vec![vec![sound]]);
        assert_eq!(f.e.mem.u32(LAST_CACHED_HASH), 0x77);
        assert_eq!(f.e.mem.u32(LAST_CACHED_TICK), 5000);
        assert_eq!(f.messages(), vec![plain(0x33, 0, 0, sound)]);
        // The same hash again within a second: deleted without opening.
        f.e.clock_ms = 5900;
        f.e.call(0x00ae7050, &args![task]);
        assert_eq!(sounds.calls(SOUND_SLOT_OPEN).len(), 1);
        assert_eq!(sounds.calls(SOUND_SLOT_DESTRUCTOR), vec![vec![sound, 1]]);
        // A failed open of a cached sound deletes it too.
        f.e.clock_ms = 9000;
        sounds.answer(SOUND_SLOT_OPEN, ret(0));
        f.e.call(0x00ae7050, &args![task]);
        assert_eq!(sounds.calls(SOUND_SLOT_DESTRUCTOR).len(), 2);
        // Without type bit 0x8000 the sound is only opened.
        f.e.mem.set_u32(sound + SOUND_TYPE_FLAGS, 0);
        sounds.answer(SOUND_SLOT_OPEN, ret(1));
        f.e.call(0x00ae7050, &args![task]);
        assert_eq!(sounds.calls(SOUND_SLOT_OPEN).len(), 3);
        assert_eq!(f.messages().len(), 1);
    }

    #[test]
    fn finishing_and_cancelling_a_task_remove_it_from_the_load_list() {
        let (mut f, sounds, sound, task) = task_world();
        let list = MANAGER_INSTANCE + 0x124;
        let other: Ptr<AudioLoadTask> = f.e.new_object();
        list_push(&mut f.e, list, other.addr());
        list_push(&mut f.e, list, task.addr());
        f.e.call(0x00ae7360, &args![task]);
        assert_eq!(list_elements(&f.e, list), vec![other.addr()]);
        // Cancel: a sound with type bit 0x8000 is deleted ...
        list_push(&mut f.e, list, task.addr());
        f.e.mem.set_u32(sound + SOUND_TYPE_FLAGS, 0x8000);
        f.e.call(0x00ae7390, &args![task, 0u32, 0u32]);
        assert_eq!(list_elements(&f.e, list), vec![other.addr()]);
        assert_eq!(sounds.calls(SOUND_SLOT_DESTRUCTOR), vec![vec![sound, 1]]);
        // ... any other becomes "ready" (0x10) and loses 0x200.
        f.e.mem.set_u32(sound + SOUND_TYPE_FLAGS, 0);
        f.e.mem.set_u32(sound + SOUND_STATE_FLAGS, 0x200);
        f.e.call(0x00ae7390, &args![task, 0u32, 0u32]);
        assert_eq!(f.e.mem.u32(sound + SOUND_STATE_FLAGS), 0x10);
        // A task without a sound only leaves the list.
        f.e.set(task, AudioLoadTask::pSound, 0);
        f.e.call(0x00ae7390, &args![task, 0u32, 0u32]);
        assert_eq!(sounds.calls(SOUND_SLOT_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn remove_load_task_takes_the_section_and_removes_one_entry() {
        let (mut f, _sounds, _sound, _task) = task_world();
        let list = f.this.addr() + 0x124;
        for task in [1u32, 2, 3] {
            list_push(&mut f.e, list, task);
        }
        f.e.call_log = Some(vec![]);
        f.call(0x00ae6f10, &[2]);
        let log = f.e.call_log.take().unwrap();
        assert_eq!(list_elements(&f.e, list), vec![1, 3]);
        assert_eq!(log[1].0, IMPORT_ENTER_CRITICAL_SECTION);
        assert_eq!(log[1].1, vec![f.this.addr() + 0x10c]);
        assert_eq!(log.last().unwrap().0, IMPORT_LEAVE_CRITICAL_SECTION);
        // An entry that is not there leaves the list alone.
        f.call(0x00ae6f10, &[9]);
        assert_eq!(list_elements(&f.e, list), vec![1, 3]);
    }

    #[test]
    fn a_task_describes_itself_through_the_base_formatter() {
        let mut e = engine2();
        let task: Ptr<AudioLoadTask> = e.new_object();
        e.register(LOAD_TASK_DESCRIBE, |_, a| {
            assert_eq!(a[1..], [0x1000, 64, LOAD_TASK_DESCRIPTION]);
            ret(7)
        });
        assert_eq!(e.call(0x00ae7480, &args![task, 0x1000u32, 64u32]).u32(), 7);
    }

    // ---- threads ----

    #[test]
    fn the_thread_procedure_waits_once_and_then_works_forever() {
        let mut e = engine2();
        let thread: Ptr<BSAudioManagerThread> = e.new_object();
        e.mem.set_u32(thread.addr() + 0x34, 0xbeef);
        e.mem.set_u32(thread.addr() + 0x30, 1);
        e.mem.set_u8(thread.addr() + 0x48, 1);
        let waited = Rc::new(RefCell::new(Vec::new()));
        let log = waited.clone();
        e.register_double(IMPORT_WAIT_FOR_SINGLE_OBJECT, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(0)
        });
        let sleeps = Rc::new(RefCell::new(0));
        let count = sleeps.clone();
        e.register_double(IMPORT_SLEEP, move |_, a| {
            assert_eq!(a, [1]);
            *count.borrow_mut() += 1;
            if *count.borrow() == 3 {
                panic!("stop the thread");
            }
            ret(0)
        });
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x00ae74b0, &args![thread]);
        }));
        assert!(result.is_err());
        assert_eq!(*waited.borrow(), vec![vec![0xbeef, 0xffff_ffff]]);
        // The first semaphore was counted down and the thread runs.
        assert_eq!(e.mem.u32(thread.addr() + 0x30), 0);
        assert_eq!(e.mem.u8(thread.addr() + 0x48), 0);
        assert_eq!(*sleeps.borrow(), 3);
    }

    #[test]
    fn the_thread_constructor_makes_two_semaphores() {
        let mut e = engine2();
        let thread: Ptr<BSAudioManagerThread> = e.new_object();
        let made = Rc::new(RefCell::new(Vec::new()));
        let log = made.clone();
        e.register_double(IMPORT_CREATE_SEMAPHORE, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(0x1000 + log.borrow().len() as u32)
        });
        e.register(THREAD_BASE_CONSTRUCTOR, |_, a| ret(a[0]));
        assert_eq!(e.call(0x00ae77d0, &args![thread]).u32(), thread.addr());
        assert_eq!(e.mem.u32(thread.addr()), BSAUDIO_MANAGER_THREAD_VTABLE);
        assert_eq!(*made.borrow(), vec![vec![0, 0, 1, 0], vec![0, 0, 1, 0]]);
        assert_eq!(e.mem.u32(thread.addr() + 0x34), 0x1001);
        assert_eq!(e.mem.u32(thread.addr() + 0x40), 0x1002);
        assert_eq!(e.mem.u32(thread.addr() + 0x38), 1);
        assert_eq!(e.mem.u32(thread.addr() + 0x44), 1);
        assert_eq!(e.mem.u8(thread.addr() + 0x48), 1);
    }

    #[test]
    fn the_thread_destructor_closes_both_handles_and_the_deleting_one_frees_it() {
        let mut e = engine2();
        let thread: Ptr<BSAudioManagerThread> = e.new_object();
        e.mem.set_u32(thread.addr() + 0x34, 0xa1);
        e.mem.set_u32(thread.addr() + 0x40, 0xa2);
        let closed = Rc::new(RefCell::new(Vec::new()));
        let log = closed.clone();
        e.register_double(IMPORT_CLOSE_HANDLE, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(1)
        });
        let base = Rc::new(RefCell::new(Vec::new()));
        let log = base.clone();
        e.register_double(THREAD_BASE_DESTRUCTOR, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(0)
        });
        e.call(0x00ae78a0, &args![thread]);
        assert_eq!(*closed.borrow(), vec![0xa2, 0xa1]);
        assert_eq!(*base.borrow(), vec![thread.addr()]);
        assert_eq!(e.mem.u32(thread.addr()), BSAUDIO_MANAGER_THREAD_VTABLE);
        let block = e.mem.alloc(0x4c);
        assert_eq!(e.call(0x00ae7870, &[block, 1]).u32(), block);
        assert_eq!(e.mem.block_size(block), None);
    }

    #[test]
    fn the_thread_steps_signal_the_suspended_semaphore() {
        let mut e = engine2();
        let thread: Ptr<BSAudioManagerThread> = e.new_object();
        e.mem.set_u32(thread.addr() + 0x40, 0xb2);
        e.mem.set_u32(thread.addr() + 0x34, 0xb1);
        e.mem.set_u32(thread.addr() + 0x30, 1);
        let released = Rc::new(RefCell::new(Vec::new()));
        let log = released.clone();
        e.register_double(IMPORT_RELEASE_SEMAPHORE, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(1)
        });
        e.register(IMPORT_WAIT_FOR_SINGLE_OBJECT, |_, a| {
            assert_eq!(a, [0xb1, 0xffff_ffff]);
            ret(0)
        });
        e.call(0x00ae7a50, &args![thread]);
        assert_eq!(*released.borrow(), vec![vec![0xb2, 1, 0]]);
        // The second semaphore counted up, the first down, flag cleared.
        assert_eq!(e.mem.u32(thread.addr() + 0x3c), 1);
        assert_eq!(e.mem.u32(thread.addr() + 0x30), 0);
        assert_eq!(e.mem.u8(thread.addr() + 0x48), 0);
        let stops = Rc::new(RefCell::new(Vec::new()));
        let log = stops.clone();
        e.register_double(THREAD_STOP_STEP, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(0)
        });
        e.call(0x00ae7ac0, &args![thread]);
        assert_eq!(*stops.borrow(), vec![vec![thread.addr(), 0, 0]]);
        assert_eq!(e.mem.u8(thread.addr() + 0x48), 1);
        assert_eq!(e.mem.u32(thread.addr() + 0x3c), 2);
    }

    // ---- message stacks, manager and record copies ----

    #[test]
    fn the_message_stack_pushes_pops_and_deletes_old_messages() {
        let mut e = engine2();
        let stack: Ptr<AudioMessageStack> = e.new_object();
        e.mem.write(stack.addr(), &[0xaa; 16]);
        assert_eq!(e.call(0x00ae7510, &args![stack]).u32(), stack.addr());
        assert_eq!(e.get(stack, AudioMessageStack::iCount), 0);
        assert_eq!(e.get(stack, AudioMessageStack::pTop), 0);
        assert!(!e.get(stack, AudioMessageStack::bBottomless));
        assert_eq!(e.get(stack, AudioMessageStack::iTimeLastAllocated), 0);
        let deleted = Rc::new(RefCell::new(Vec::new()));
        let log = deleted.clone();
        e.register_double(MESSAGE_DESTRUCTOR, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(0)
        });
        e.register(MESSAGE_CONSTRUCTOR, |_, a| ret(a[0]));
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.call(0x00ae75f0, &args![stack, first]);
        e.call(0x00ae75f0, &args![stack, second]);
        e.call(0x00ae75f0, &args![stack, 0u32]);
        assert_eq!(e.get(stack, AudioMessageStack::iCount), 2);
        assert_eq!(e.mem.u32(second + 0xc), first);
        assert_eq!(e.call(0x00ae76b0, &args![stack]).u32(), second);
        assert_eq!(e.mem.u32(second + 0xc), 0);
        assert_eq!(e.get(stack, AudioMessageStack::iCount), 1);
        // A bottomless stack over 0x200 messages deletes pushes a second
        // after its last allocation.
        e.set(stack, AudioMessageStack::bBottomless, true);
        e.set(stack, AudioMessageStack::iCount, 0x201);
        e.set(stack, AudioMessageStack::iTimeLastAllocated, 100);
        e.clock_ms = 1000;
        let spare = e.mem.alloc(0x20);
        e.call(0x00ae75f0, &args![stack, spare]);
        assert!(deleted.borrow().is_empty());
        assert_eq!(e.get(stack, AudioMessageStack::iCount), 0x202);
        e.clock_ms = 1125;
        let doomed = e.mem.alloc(0x20);
        e.call(0x00ae75f0, &args![stack, doomed]);
        assert_eq!(*deleted.borrow(), vec![doomed]);
        assert_eq!(e.mem.block_size(doomed), None);
        assert_eq!(e.get(stack, AudioMessageStack::iCount), 0x202);
    }

    #[test]
    fn an_empty_bottomless_stack_makes_a_message_and_a_stack_destructor_empties_it() {
        let mut e = engine2();
        let stack: Ptr<AudioMessageStack> = e.new_object();
        e.set(stack, AudioMessageStack::bBottomless, true);
        e.register(MESSAGE_CONSTRUCTOR, |_, a| ret(a[0]));
        e.clock_ms = 4250;
        let made = e.call(0x00ae76b0, &args![stack]).u32();
        assert_ne!(made, 0);
        assert_eq!(e.get(stack, AudioMessageStack::iTimeLastAllocated), 4250);
        // Not bottomless and empty: nothing.
        e.set(stack, AudioMessageStack::bBottomless, false);
        assert_eq!(e.call(0x00ae76b0, &args![stack]).u32(), 0);
        // The destructor pops every message, destroying and freeing it.
        let deleted = Rc::new(RefCell::new(Vec::new()));
        let log = deleted.clone();
        e.register_double(MESSAGE_DESTRUCTOR, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(0)
        });
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.call(0x00ae75f0, &args![stack, first]);
        e.call(0x00ae75f0, &args![stack, second]);
        e.call(0x00ae7550, &args![stack]);
        assert_eq!(*deleted.borrow(), vec![second, first]);
        assert_eq!(e.get(stack, AudioMessageStack::iCount), 0);
        assert_eq!(e.mem.block_size(first), None);
    }

    #[test]
    fn the_manager_deleting_destructor_runs_the_body_and_frees_on_request() {
        let mut e = engine2();
        e.register(LIST_CLEAR, |_, _| ret(0));
        e.register(MAP_CLEAR, |_, _| ret(0));
        e.register(FREE_TABLE, |_, _| ret(0));
        e.register(MESSAGE_STACK_DESTRUCTOR, |_, _| ret(0));
        e.register(VECTOR_DESTRUCTOR_ITERATOR, |_, _| ret(0));
        let block = e.mem.alloc(0x188);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae77a0, &[block, 1]).u32(), block);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(block), 0x010a_3368);
        assert!(log.iter().any(|c| c.0 == IMPORT_DELETE_CRITICAL_SECTION));
        assert_eq!(e.mem.block_size(block), None);
        let kept = e.mem.alloc(0x188);
        e.call(0x00ae77a0, &[kept, 0]);
        assert_eq!(e.mem.block_size(kept), Some(0x188));
    }

    #[test]
    fn sound_info_and_message_assignments_copy_member_by_member() {
        let mut e = engine2();
        let debug_copies = Rc::new(RefCell::new(Vec::new()));
        let log = debug_copies.clone();
        e.register_double(DEBUG_SOUND_INFO_ASSIGN, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(a[0])
        });
        let source: Ptr<BSSoundInfo> = e.new_object();
        let target: Ptr<BSSoundInfo> = e.new_object();
        e.set(source, BSSoundInfo::iID, 5);
        e.set(source, BSSoundInfo::fFrequency, 1.5);
        e.set(source, BSSoundInfo::fVolume, 0.25);
        e.set(source, BSSoundInfo::uiSoundType, 0x44);
        e.set(source, BSSoundInfo::uiDuration, 900);
        e.set(source, BSSoundInfo::bIsPlaying, 1);
        assert_eq!(
            e.call(0x00ae7920, &args![target, source]).u32(),
            target.addr()
        );
        assert_eq!(e.get(target, BSSoundInfo::iID), 5);
        assert_eq!(e.get(target, BSSoundInfo::fFrequency), 1.5);
        assert_eq!(e.get(target, BSSoundInfo::fVolume), 0.25);
        assert_eq!(e.get(target, BSSoundInfo::uiSoundType), 0x44);
        assert_eq!(e.get(target, BSSoundInfo::uiDuration), 900);
        assert_eq!(e.get(target, BSSoundInfo::bIsPlaying), 1);
        assert_eq!(
            *debug_copies.borrow(),
            vec![vec![target.addr() + 0x18, source.addr() + 0x18]]
        );
        // A message: the words, the smart pointer, the vector.
        e.register(SMART_POINTER_ASSIGN, fake_pointer_assign);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object + 4, 1);
        let from = fake_message(&mut e, [7, 8, 9, 10], [1.5, 2.5, 3.5]);
        e.mem.set_u32(from.addr() + 0x10, object);
        let to = fake_message(&mut e, [0, 0, 0, 0], [0.0; 3]);
        assert_eq!(e.call(0x00ae7990, &args![to, from]).u32(), to.addr());
        assert_eq!(
            e.mem.bytes(to.addr(), 4 * 4),
            e.mem.bytes(from.addr(), 4 * 4)
        );
        assert_eq!(e.mem.u32(to.addr() + 0x10), object);
        assert_eq!(e.mem.u32(object + 4), 2);
        assert_eq!(e.mem.f32(to.addr() + 0x14), 1.5);
        assert_eq!(e.mem.f32(to.addr() + 0x1c), 3.5);
    }

    // ---- map constructors and the moving-reference map ----

    #[test]
    fn the_map_constructors_set_their_vtables_and_clear_a_table() {
        let mut e = engine2();
        e.register(TABLE_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MEMSET, |e, a| {
            assert_eq!(a[1], 0);
            e.mem.write(a[0], &vec![0; a[2] as usize]);
            ret(a[0])
        });
        for (address, vtable) in [
            (0x00ae_7b40, 0x010a_3370),
            (0x00ae_7bc0, 0x010a_3390),
            (0x00ae_7c40, 0x010a_33b0),
        ] {
            let map: Ptr<NiTPointerMap> = e.new_object();
            e.mem.write(map.addr(), &[0xcc; 16]);
            assert_eq!(e.call(address, &args![map, 37u32]).u32(), map.addr());
            assert_eq!(e.mem.u32(map.addr()), vtable);
            assert_eq!(e.get(map, NiTPointerMap::m_uiHashSize), 37);
            assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 0);
            let table = e.get(map, NiTPointerMap::m_ppkHashTable);
            assert!(e.mem.block_size(table).unwrap() >= 37 * 4);
        }
    }

    #[test]
    fn the_moving_reference_map_sets_gets_and_walks() {
        let (mut f, _audio) = audio_fixture();
        let map = moving_map_model(&mut f);
        let e = &mut f.e;
        let first = fake_object(e, 5, [0.0; 3]);
        let second = fake_object(e, 5, [0.0; 3]);
        e.call(0x00ae7cc0, &args![map, 3u32, first]);
        e.call(0x00ae7cc0, &args![map, 10u32, second]);
        assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 2);
        // Same bucket (3 and 10 with 7 buckets): the newer entry is first.
        let slot = e.mem.alloc(4);
        assert!(e.call(0x00ae7e60, &args![map, 3u32, slot]).bool());
        assert_eq!(e.mem.u32(slot), first);
        assert!(!e.call(0x00ae7e60, &args![map, 4u32, slot]).bool());
        // Setting an existing key replaces the value and drops the argument's
        // reference.
        let third = fake_object(e, 5, [0.0; 3]);
        e.call(0x00ae7cc0, &args![map, 3u32, third]);
        assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 2);
        assert!(e.call(0x00ae7e60, &args![map, 3u32, slot]).bool());
        assert_eq!(e.mem.u32(slot), third);
        // Walking: the first item of bucket 3, then the next one in it, then
        // the end of the map.
        let words = e.mem.alloc(12);
        e.mem.set_u32(words, map_first_item(e, map));
        let mut keys = Vec::new();
        while e.mem.u32(words) != 0 {
            e.call(0x00ae7ee0, &args![map, words, words + 4, words + 8]);
            keys.push(e.mem.u32(words + 4));
        }
        assert_eq!(keys, vec![10, 3]);
    }

    // ---- pointer lists ----

    #[test]
    fn removing_an_entry_from_a_pointer_list() {
        let mut e = engine2();
        e.register(LIST_REMOVE_AT, fake_remove_at);
        let list = e.mem.alloc(12);
        for element in [10u32, 20, 30] {
            list_push(&mut e, list, element);
        }
        let wanted = e.mem.alloc(4);
        e.mem.set_u32(wanted, 20);
        assert_eq!(e.call(0x00ae8150, &[list, wanted]).u32(), 20);
        assert_eq!(list_elements(&e, list), vec![10, 30]);
        e.mem.set_u32(wanted, 99);
        assert_eq!(e.call(0x00ae8150, &[list, wanted]).u32(), 99);
        assert_eq!(list_elements(&e, list), vec![10, 30]);
    }

    #[test]
    fn a_pointer_list_is_emptied_node_by_node() {
        let mut e = engine2();
        let freed = Rc::new(RefCell::new(Vec::new()));
        let log = freed.clone();
        e.register_double(LIST_NODE_FREE, move |e, a| {
            assert_eq!(e.mem.u32(a[1] + 8), 0);
            log.borrow_mut().push(a.to_vec());
            ret(0)
        });
        let list: Ptr<NiTPointerList> = e.new_object();
        list_push(&mut e, list.addr(), 1);
        list_push(&mut e, list.addr(), 2);
        let nodes = {
            let first = e.mem.u32(list.addr());
            vec![first, e.mem.u32(first)]
        };
        e.call(0x00ae8370, &args![list]);
        assert_eq!(
            *freed.borrow(),
            vec![
                vec![list.addr() + 8, nodes[0]],
                vec![list.addr() + 8, nodes[1]]
            ]
        );
        assert_eq!(e.mem.bytes(list.addr(), 12), vec![0; 12]);
    }

    /// A chain node: item then next.
    fn chain_node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.mem.set_u32(node + 4, next);
        node
    }

    /// The items of a chain.
    fn chain_items(e: &Engine, head: u32) -> Vec<u32> {
        let mut items = Vec::new();
        let mut node = head;
        while node != 0 {
            items.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        items
    }

    /// A signed word comparison for the sort cases.
    fn install_word_compare(e: &mut Engine) -> u32 {
        e.register(0x7700_0000, |_, a| {
            ret((a[0] as i32).cmp(&(a[1] as i32)) as i32 as u32)
        });
        0x7700_0000
    }

    #[test]
    fn a_chain_is_freed_behind_its_head() {
        let mut e = engine2();
        let third = chain_node(&mut e, 3, 0);
        let second = chain_node(&mut e, 2, third);
        let head = chain_node(&mut e, 1, second);
        e.call(0x00ae83e0, &[head]);
        assert_eq!(chain_items(&e, head), vec![0]);
        assert_eq!(e.mem.block_size(second), None);
        assert_eq!(e.mem.block_size(third), None);
        assert_eq!(e.mem.block_size(head), Some(8));
    }

    #[test]
    fn the_bubble_and_shell_sorts_order_with_the_comparison() {
        let mut e = engine2();
        let compare = install_word_compare(&mut e);
        let mut next = 0;
        for item in [4u32, 1, 3, 2, 5].into_iter().rev() {
            next = chain_node(&mut e, item, next);
        }
        e.call(0x00ae8470, &[next, compare]);
        assert_eq!(chain_items(&e, next), vec![1, 2, 3, 4, 5]);
        // The shell sort on an array of words (13 of them: two gaps).
        let words = [9u32, 3, 7, 1, 8, 2, 6, 0, 5, 4, 12, 11, 10];
        let array = e.mem.alloc(4 * words.len() as u32);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *w);
        }
        e.call(0x00ae8550, &[0, array, compare, words.len() as u32]);
        let sorted: Vec<u32> = (0..words.len() as u32)
            .map(|i| e.mem.u32(array + 4 * i))
            .collect();
        assert_eq!(sorted, (0..13).collect::<Vec<u32>>());
    }

    #[test]
    fn the_list_sort_rebuilds_the_chain_in_order() {
        let mut e = engine2();
        let compare = install_word_compare(&mut e);
        let mut next = 0;
        for item in [30u32, 10, 20].into_iter().rev() {
            next = chain_node(&mut e, item, next);
        }
        let old_second = e.mem.u32(next + 4);
        e.call(0x00ae81e0, &[next, compare]);
        assert_eq!(chain_items(&e, next), vec![10, 20, 30]);
        let _ = old_second;
        // An empty list (no non-null items) stays a lone head.
        let empty = chain_node(&mut e, 0, 0);
        e.call(0x00ae81e0, &[empty, compare]);
        assert_eq!(chain_items(&e, empty), vec![0]);
    }

    // ---- the functions of 00ae8650 to 00ae9150 ----

    /// A zeroed block with the object layout the smart pointers expect: the
    /// vtable at +0, the reference count at +4.
    fn counted_object(e: &mut Engine, vtable: &FakeVtable, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable.address);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// `strcpy` that copies the bytes up to and including the terminator.
    fn install_strcpy(e: &mut Engine) {
        e.register(STRCPY, |e, a| {
            let mut index = 0;
            loop {
                let byte = e.mem.u8(a[1] + index);
                e.mem.set_u8(a[0] + index, byte);
                if byte == 0 {
                    break;
                }
                index += 1;
            }
            ret(a[0])
        });
    }

    #[test]
    fn a_sound_message_starts_empty() {
        let mut e = engine();
        let message: Ptr<BSSoundMessage> = e.new_object();
        for offset in (0..0x20).step_by(4) {
            e.mem.set_u32(message.addr() + offset, 0xdead_beef);
        }
        let result = e.call(0x00ae8650, &args![message]).u32();
        assert_eq!(result, message.addr());
        for offset in (0..0x20).step_by(4) {
            assert_eq!(e.mem.u32(message.addr() + offset), 0, "+{offset:#x}");
        }
        // Every float of the vector is +0.0 (all bits clear).
        assert_eq!(e.get(message, BSSoundMessage::vector3_z).to_bits(), 0);
    }

    #[test]
    fn a_sound_message_releases_its_pointer_and_drops_the_reference() {
        let mut e = engine();
        let vtable = fake_vtable(&mut e, 0x7200_0000);
        let message: Ptr<BSSoundMessage> = e.new_object();
        // No pointer: nothing is called.
        e.call_log = Some(vec![]);
        e.call(0x00ae86d0, &args![message]);
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![(0x00ae86d0, vec![message.addr()])]
        );
        // A pointer with two references: released through 00c45830, one
        // reference left, no destruction.
        e.register(RELEASE_MESSAGE_POINTER, |_, _| ret(0));
        let object = counted_object(&mut e, &vtable, 2);
        e.set(message, BSSoundMessage::spData, object);
        e.call_log = Some(vec![]);
        e.call(0x00ae86d0, &args![message]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(RELEASE_MESSAGE_POINTER, vec![object])));
        assert_eq!(e.mem.u32(object + 4), 1);
        assert!(vtable.calls(4).is_empty());
        // The last reference: the object's virtual method at +4 runs.
        e.call(0x00ae86d0, &args![message]);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(vtable.calls(4), vec![vec![object]]);
    }

    #[test]
    fn a_debug_record_starts_with_empty_names_and_the_default_location() {
        let mut e = engine();
        let record: Ptr<BSDebugSoundInfo> = e.new_object();
        let base = record.addr();
        for offset in (0..0x23c).step_by(4) {
            e.mem.set_u32(base + offset, 0xdead_beef);
        }
        let result = e.call(0x00ae8770, &args![record]).u32();
        assert_eq!(result, base);
        assert_eq!(e.mem.u32(base), base + 4);
        assert_eq!(e.mem.u32(base + 0x108), base + 0x10c);
        assert_eq!(e.mem.u8(base + 4), 0);
        assert_eq!(e.mem.u8(base + 0x10c), 0);
        assert_eq!(e.get(record, BSDebugSoundInfo::fRadiusMin), 0.0);
        assert_eq!(e.get(record, BSDebugSoundInfo::fRadiusMax), 0.0);
        assert_eq!(e.get(record, BSDebugSoundInfo::locationX), 1.0);
        assert_eq!(e.get(record, BSDebugSoundInfo::locationY), 2.0);
        assert_eq!(e.get(record, BSDebugSoundInfo::distanceToListener), 3.0);
        for offset in [0x224, 0x228, 0x22c, 0x230, 0x234, 0x238] {
            assert_eq!(e.mem.u32(base + offset), 0, "+{offset:#x}");
        }
        // The bytes between the fields stay as they were.
        assert_eq!(e.mem.u32(base + 0x8), 0xdead_beef);
    }

    #[test]
    fn a_debug_record_clears_its_pointers_and_counters_but_not_the_location() {
        let mut e = engine();
        let record: Ptr<BSDebugSoundInfo> = e.new_object();
        let base = record.addr();
        for offset in (0..0x23c).step_by(4) {
            e.mem.set_u32(base + offset, 0xdead_beef);
        }
        e.call(0x00ae8830, &args![record]);
        for offset in [
            0x0, 0x108, 0x210, 0x214, 0x224, 0x228, 0x22c, 0x230, 0x234, 0x238,
        ] {
            assert_eq!(e.mem.u32(base + offset), 0, "+{offset:#x}");
        }
        for offset in [0x218, 0x21c, 0x220, 0x4, 0x10c] {
            assert_eq!(e.mem.u32(base + offset), 0xdead_beef, "+{offset:#x}");
        }
    }

    #[test]
    fn a_debug_record_assignment_copies_names_and_values() {
        let mut e = engine();
        install_strcpy(&mut e);
        let to: Ptr<BSDebugSoundInfo> = e.new_object();
        let from: Ptr<BSDebugSoundInfo> = e.new_object();
        e.call(0x00ae8770, &args![to]);
        e.call(0x00ae8770, &args![from]);
        e.mem.set_cstr(to.addr() + 4, b"stale");
        e.mem.set_cstr(to.addr() + 0x10c, b"stale");
        e.mem.set_cstr(from.addr() + 4, b"sound");
        e.mem.set_cstr(from.addr() + 0x10c, b"data\\file.wav");
        e.set(from, BSDebugSoundInfo::fRadiusMin, 5.0);
        e.set(from, BSDebugSoundInfo::fRadiusMax, 50.0);
        e.set(from, BSDebugSoundInfo::iStateFlags, 0x1234);
        e.set(from, BSDebugSoundInfo::iStaticAttenuation, 7);
        e.set(from, BSDebugSoundInfo::iSystemAttenuation, 8);
        e.set(from, BSDebugSoundInfo::iDistanceAttenuation, 9);
        e.set(from, BSDebugSoundInfo::locationX, 10.0);
        // A signalling NaN loaded through the x87 stack comes back quiet.
        e.mem.set_u32(from.addr() + 0x228, 0x7f80_0001);
        e.mem.set_u32(from.addr() + 0x22c, 0x7fc0_0123);
        let result = e.call(0x00ae88c0, &args![to, from]).u32();
        assert_eq!(result, to.addr());
        assert_eq!(e.mem.cstr(to.addr() + 4), b"sound");
        assert_eq!(e.mem.cstr(to.addr() + 0x10c), b"data\\file.wav");
        assert_eq!(e.get(to, BSDebugSoundInfo::fRadiusMin), 5.0);
        assert_eq!(e.get(to, BSDebugSoundInfo::fRadiusMax), 50.0);
        assert_eq!(e.get(to, BSDebugSoundInfo::iStateFlags), 0x1234);
        assert_eq!(e.get(to, BSDebugSoundInfo::locationX), 10.0);
        assert_eq!(e.get(to, BSDebugSoundInfo::locationY), 2.0);
        assert_eq!(e.get(to, BSDebugSoundInfo::iStaticAttenuation), 7);
        assert_eq!(e.get(to, BSDebugSoundInfo::iSystemAttenuation), 8);
        assert_eq!(e.get(to, BSDebugSoundInfo::iDistanceAttenuation), 9);
        assert_eq!(e.mem.u32(to.addr() + 0x228), 0x7fc0_0001);
        assert_eq!(e.mem.u32(to.addr() + 0x22c), 0x7fc0_0123);
    }

    #[test]
    fn a_debug_record_assignment_skips_the_names_of_a_record_without_pointers() {
        let mut e = engine();
        let copies = Rc::new(RefCell::new(0));
        let counter = copies.clone();
        e.register_double(STRCPY, move |_, _| {
            *counter.borrow_mut() += 1;
            ret(0)
        });
        let to: Ptr<BSDebugSoundInfo> = e.new_object();
        let from: Ptr<BSDebugSoundInfo> = e.new_object();
        e.call(0x00ae8770, &args![to]);
        e.mem.set_u8(to.addr() + 4, b'x');
        e.call(0x00ae88c0, &args![to, from]);
        // Both name pointers of `from` are null: no copy, but this record's
        // buffers are cleared.
        assert_eq!(*copies.borrow(), 0);
        assert_eq!(e.mem.u8(to.addr() + 4), 0);
        // Only the file name pointer set: one copy.
        e.mem.set_u32(from.addr() + 0x108, from.addr() + 0x10c);
        e.call(0x00ae88c0, &args![to, from]);
        assert_eq!(*copies.borrow(), 1);
    }

    #[test]
    fn a_sound_info_starts_cleared_around_its_debug_record() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(DEBUG_SOUND_INFO_CONSTRUCTOR, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(a[0])
        });
        let info: Ptr<BSSoundInfo> = e.new_object();
        let base = info.addr();
        for offset in (0..0x18).step_by(4) {
            e.mem.set_u32(base + offset, 0xdead_beef);
        }
        let result = e.call(0x00ae89f0, &args![info]).u32();
        assert_eq!(result, base);
        assert_eq!(*seen.borrow(), vec![vec![base + 0x18]]);
        for offset in [0x0, 0x4, 0xc, 0x10] {
            assert_eq!(e.mem.u32(base + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.u8(base + 0x14), 0);
        // The volume is not touched by the constructor.
        assert_eq!(e.mem.u32(base + 0x8), 0xdead_beef);
    }

    #[test]
    fn a_sound_info_copy_constructs_its_debug_record_and_copies_the_values() {
        let mut e = engine();
        e.register(DEBUG_SOUND_INFO_CONSTRUCTOR, |_, a| ret(a[0]));
        let assigned = Rc::new(RefCell::new(Vec::new()));
        let log = assigned.clone();
        e.register_double(DEBUG_SOUND_INFO_ASSIGN, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(a[0])
        });
        let to: Ptr<BSSoundInfo> = e.new_object();
        let from: Ptr<BSSoundInfo> = e.new_object();
        e.set(from, BSSoundInfo::iID, 12);
        e.set(from, BSSoundInfo::fFrequency, 1.5);
        e.mem.set_u32(from.addr() + 8, 0x7f80_0042);
        e.set(from, BSSoundInfo::uiSoundType, 3);
        e.set(from, BSSoundInfo::uiDuration, 4000);
        e.set(from, BSSoundInfo::bIsPlaying, 1);
        let result = e.call(0x00ae8a40, &args![to, from]).u32();
        assert_eq!(result, to.addr());
        assert_eq!(e.get(to, BSSoundInfo::iID), 12);
        assert_eq!(e.get(to, BSSoundInfo::fFrequency), 1.5);
        assert_eq!(e.mem.u32(to.addr() + 8), 0x7fc0_0042);
        assert_eq!(e.get(to, BSSoundInfo::uiSoundType), 3);
        assert_eq!(e.get(to, BSSoundInfo::uiDuration), 4000);
        assert_eq!(e.get(to, BSSoundInfo::bIsPlaying), 1);
        assert_eq!(
            *assigned.borrow(),
            vec![vec![to.addr() + 0x18, from.addr() + 0x18]]
        );
    }

    #[test]
    fn a_sound_info_clears_itself_and_runs_the_debug_destructor_last() {
        let mut e = engine();
        e.register(DEBUG_SOUND_INFO_DESTRUCTOR, |_, _| ret(0));
        let info: Ptr<BSSoundInfo> = e.new_object();
        let base = info.addr();
        for offset in (0..0x18).step_by(4) {
            e.mem.set_u32(base + offset, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        e.call(0x00ae8af0, &args![info]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log,
            vec![
                (0x00ae8af0, vec![base]),
                (DEBUG_SOUND_INFO_DESTRUCTOR, vec![base + 0x18])
            ]
        );
        for offset in (0..0x14).step_by(4) {
            assert_eq!(e.mem.u32(base + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.u8(base + 0x14), 0);
    }

    #[test]
    fn the_debug_record_getter_copies_the_whole_record() {
        let mut e = engine();
        let info: Ptr<BSSoundInfo> = e.new_object();
        let out = e.mem.alloc(0x23c + 8);
        for index in 0..0x8f {
            e.mem.set_u32(info.addr() + 0x18 + index * 4, 0x100 + index);
        }
        e.mem.set_u32(out + 0x23c, 0x5555);
        let result = e.call(0x00ae8b40, &args![info, out]).u32();
        assert_eq!(result, out);
        for index in 0..0x8f {
            assert_eq!(e.mem.u32(out + index * 4), 0x100 + index);
        }
        assert_eq!(e.mem.u32(out + 0x23c), 0x5555);
    }

    #[test]
    fn the_constant_method_answers_two() {
        let mut e = engine();
        assert_eq!(e.call(0x00ae8b80, &[0]).u8(), 2);
    }

    #[test]
    fn the_sound_flag_step_sets_or_clears_bit_0x400_and_resends_the_volume() {
        let mut e = engine();
        let vtable = fake_vtable(&mut e, 0x7200_0000);
        let sound = fake_sound(&mut e, &vtable, 1);
        e.mem.set_u32(sound + SOUND_STATE_FLAGS, 0x0000_0101);
        e.mem.set_f32(sound + SOUND_CURRENT_VOLUME, 0.75);
        e.call(0x00ae8b90, &[sound, 1]);
        assert_eq!(e.mem.u32(sound + SOUND_STATE_FLAGS), 0x0000_0501);
        assert_eq!(
            vtable.calls(SOUND_SLOT_SET_VOLUME),
            vec![vec![sound, 0.75f32.to_bits()]]
        );
        // Only the low byte of the argument counts.
        e.call(0x00ae8b90, &[sound, 0x100]);
        assert_eq!(e.mem.u32(sound + SOUND_STATE_FLAGS), 0x0000_0101);
        // A signalling NaN volume is passed on quiet.
        e.mem.set_u32(sound + SOUND_CURRENT_VOLUME, 0x7f80_0001);
        e.call(0x00ae8b90, &[sound, 0xff]);
        assert_eq!(e.mem.u32(sound + SOUND_STATE_FLAGS), 0x0000_0501);
        assert_eq!(
            vtable.calls(SOUND_SLOT_SET_VOLUME).last().unwrap(),
            &vec![sound, 0x7fc0_0001]
        );
    }

    #[test]
    fn appending_a_pointer_takes_a_node_stores_the_pointer_and_links_it() {
        let mut e = engine();
        let list = e.mem.alloc(0x14);
        let node = e.mem.alloc(12);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(LIST_NEW_NODE, move |_, a| {
            log.borrow_mut().push((LIST_NEW_NODE, a.to_vec()));
            ret(node)
        });
        let log = seen.clone();
        e.register_double(LIST_ADD_NODE_TAIL, move |_, a| {
            log.borrow_mut().push((LIST_ADD_NODE_TAIL, a.to_vec()));
            ret(0)
        });
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0xabcd);
        e.call(0x00ae8bf0, &[list, slot]);
        assert_eq!(e.mem.u32(node + 8), 0xabcd);
        assert_eq!(
            *seen.borrow(),
            vec![
                (LIST_NEW_NODE, vec![list + 8]),
                (LIST_ADD_NODE_TAIL, vec![list, node])
            ]
        );
    }

    /// Doubles for `MAP_CLEAR` and `FREE_TABLE` that log the vtable the map
    /// has when it is cleared and the table freed.
    fn install_map_doubles(e: &mut Engine) -> Rc<RefCell<Vec<(&'static str, u32)>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(MAP_CLEAR, move |e, a| {
            log.borrow_mut().push(("clear", e.mem.u32(a[0])));
            ret(0)
        });
        let log = seen.clone();
        e.register_double(FREE_TABLE, move |_, a| {
            log.borrow_mut().push(("free", a[0]));
            ret(0)
        });
        seen
    }

    #[test]
    fn the_map_destructors_clear_under_each_vtable_in_turn() {
        // (address, vtables the map is cleared under, in order)
        let cases: [(u32, &[u32]); 6] = [
            (0x00ae8c30, &[0x010a_3370]),
            (0x00ae8c60, &[0x010a_3348, 0x010a_3370]),
            (0x00ae8ce0, &[0x010a_3390]),
            (0x00ae8d10, &[0x010a_3328, 0x010a_3390]),
            (0x00ae8d90, &[0x010a_33b0]),
            (0x00ae8e70, &[0x010a_3308, 0x010a_33b0]),
        ];
        for (address, vtables) in cases {
            let mut e = engine();
            let seen = install_map_doubles(&mut e);
            let map = e.mem.alloc(0x10);
            e.mem.set_u32(map + 8, 0x4242);
            e.call(address, &[map]);
            let mut expected: Vec<(&str, u32)> =
                vtables.iter().map(|vtable| ("clear", *vtable)).collect();
            expected.push(("free", 0x4242));
            assert_eq!(*seen.borrow(), expected, "{address:08x}");
            assert_eq!(e.mem.u32(map), *vtables.last().unwrap());
        }
    }

    #[test]
    fn the_scalar_deleting_map_destructors_free_the_map_only_when_asked() {
        let cases: [(u32, &[u32]); 5] = [
            (0x00ae8f20, &[0x010a_3370]),
            (0x00ae8f70, &[0x010a_3348, 0x010a_3370]),
            (0x00ae9010, &[0x010a_3390]),
            (0x00ae9060, &[0x010a_3328, 0x010a_3390]),
            (0x00ae9100, &[0x010a_33b0]),
        ];
        let mut all = cases.to_vec();
        all.push((0x00ae9150, &[0x010a_3308, 0x010a_33b0]));
        for (address, vtables) in all {
            let mut e = engine();
            let seen = install_map_doubles(&mut e);
            let map = e.mem.alloc(0x10);
            e.mem.set_u32(map + 8, 0x4242);
            // Without the delete flag the map stays allocated.
            let result = e.call(address, &[map, 0]).u32();
            assert_eq!(result, map);
            assert_eq!(e.mem.block_size(map), Some(0x10), "{address:08x}");
            assert_eq!(seen.borrow().len(), vtables.len() + 1);
            assert_eq!(e.mem.u32(map), *vtables.last().unwrap());
            // With it (any word with bit 0 set) the map is deallocated.
            let result = e.call(address, &[map, 3]).u32();
            assert_eq!(result, map);
            assert_eq!(e.mem.block_size(map), None, "{address:08x}");
        }
    }

    #[test]
    fn the_integer_key_comparison_tests_equality() {
        let mut e = engine();
        assert!(e.call(0x00ae8dc0, &[0, 5, 5]).bool());
        assert!(!e.call(0x00ae8dc0, &[0, 5, 6]).bool());
    }

    #[test]
    fn set_value_stores_the_key_assigns_the_pointer_and_drops_the_argument() {
        let mut e = engine();
        let vtable = fake_vtable(&mut e, 0x7200_0000);
        let assigned = Rc::new(RefCell::new(Vec::new()));
        let log = assigned.clone();
        e.register_double(SMART_POINTER_ASSIGN, move |e, a| {
            let value = e.mem.u32(a[1]);
            log.borrow_mut().push((a[0], value));
            e.mem.set_u32(a[0], value);
            ret(a[0])
        });
        let node = e.mem.alloc(12);
        // Two references: the assignment keeps one, the argument's goes.
        let object = counted_object(&mut e, &vtable, 2);
        e.call(0x00ae8de0, &[0, node, 77, object]);
        assert_eq!(e.mem.u32(node + 4), 77);
        assert_eq!(e.mem.u32(node + 8), object);
        assert_eq!(*assigned.borrow(), vec![(node + 8, object)]);
        assert_eq!(e.mem.u32(object + 4), 1);
        assert!(vtable.calls(4).is_empty());
        // The argument held the last reference: destroyed.
        let lone = counted_object(&mut e, &vtable, 1);
        e.call(0x00ae8de0, &[0, node, 78, lone]);
        assert_eq!(e.mem.u32(lone + 4), 0);
        assert_eq!(vtable.calls(4), vec![vec![lone]]);
        // A null pointer is assigned and nothing is dropped.
        e.call(0x00ae8de0, &[0, node, 79, 0]);
        assert_eq!(e.mem.u32(node + 4), 79);
        assert_eq!(e.mem.u32(node + 8), 0);
        assert_eq!(vtable.calls(4).len(), 1);
    }

    #[test]
    fn delete_item_nulls_the_pointer_and_frees_the_node_on_the_allocator() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(SMART_POINTER_SET, move |_, a| {
            log.borrow_mut().push((SMART_POINTER_SET, a.to_vec()));
            ret(0)
        });
        let log = seen.clone();
        e.register_double(LIST_NODE_FREE, move |_, a| {
            log.borrow_mut().push((LIST_NODE_FREE, a.to_vec()));
            ret(0)
        });
        e.call(0x00ae8ef0, &[0x1000, 0x2000]);
        assert_eq!(
            *seen.borrow(),
            vec![
                (SMART_POINTER_SET, vec![0x2008, 0]),
                (LIST_NODE_FREE, vec![0x100c, 0x2000])
            ]
        );
    }

    #[test]
    fn floats_that_pass_through_the_x87_stack_come_back_quiet() {
        const SIGNALLING: u32 = 0x7f80_0001;
        const QUIET: u32 = 0x7fc0_0001;
        const ONE: u32 = 0x3f80_0000;
        // The helper: only a signalling NaN changes.
        assert_eq!(x87_quiet_bits(SIGNALLING), QUIET);
        assert_eq!(x87_quiet_bits(0xffa0_0000), 0xffe0_0000);
        assert_eq!(x87_quiet_bits(QUIET), QUIET);
        assert_eq!(x87_quiet_bits(0x7f80_0000), 0x7f80_0000);
        assert_eq!(x87_quiet_bits(ONE), ONE);
        assert_eq!(x87_quiet_bits(0x0000_0001), 0x0000_0001);
        // SetPosition: the queued message's vector (FLD / FSTP in the exe).
        let mut f = Fixture::new();
        f.call(0x00adb970, &[3, SIGNALLING, ONE, SIGNALLING]);
        let message = f.queued.borrow()[0];
        assert_eq!(f.e.mem.u32(message + 0x14), QUIET);
        assert_eq!(f.e.mem.u32(message + 0x18), ONE);
        assert_eq!(f.e.mem.u32(message + 0x1c), QUIET);
        // The game hour.
        let this: Ptr<BSAudioManager> = f.e.new_object();
        f.e.call(0x00ada300, &[this.addr(), SIGNALLING, 5]);
        assert_eq!(f.e.mem.u32(this.addr() + 0xa8), QUIET);
        // The volume resent to a sound.
        let vtable = fake_vtable(&mut f.e, 0x7200_0000);
        let sound = fake_sound(&mut f.e, &vtable, 1);
        f.e.mem.set_u32(sound + SOUND_CURRENT_VOLUME, SIGNALLING);
        apply_current_volume(&mut f.e, sound);
        assert_eq!(
            vtable.calls(SOUND_SLOT_SET_VOLUME),
            vec![vec![sound, QUIET]]
        );
        // BSSoundInfo::operator= copies the frequency and volume the same way.
        f.e.register(DEBUG_SOUND_INFO_ASSIGN, |_, a| ret(a[0]));
        let to: Ptr<BSSoundInfo> = f.e.new_object();
        let from: Ptr<BSSoundInfo> = f.e.new_object();
        f.e.mem.set_u32(from.addr() + 4, SIGNALLING);
        f.e.mem.set_u32(from.addr() + 8, ONE);
        f.e.call(0x00ae7920, &args![to, from]);
        assert_eq!(f.e.mem.u32(to.addr() + 4), QUIET);
        assert_eq!(f.e.mem.u32(to.addr() + 8), ONE);
        // BSSoundMessage assignment copies the vector the same way.
        f.e.register(SMART_POINTER_ASSIGN, |_, a| ret(a[0]));
        let target: Ptr<BSSoundMessage> = f.e.new_object();
        let source: Ptr<BSSoundMessage> = f.e.new_object();
        f.e.mem.set_u32(source.addr() + 0x14, ONE);
        f.e.mem.set_u32(source.addr() + 0x18, SIGNALLING);
        f.e.mem.set_u32(source.addr() + 0x1c, SIGNALLING);
        f.e.call(0x00ae7990, &args![target, source]);
        assert_eq!(f.e.mem.u32(target.addr() + 0x14), ONE);
        // The 16-bit copies that follow overwrite the first ten bytes with the
        // source's own bytes, so only the last two bytes of the third float
        // keep what the x87 store made of it.
        assert_eq!(f.e.mem.u32(target.addr() + 0x18), SIGNALLING);
        assert_eq!(f.e.mem.u32(target.addr() + 0x1c), QUIET);
    }
}
