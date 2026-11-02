//! `BSAudio/bsxenonaudio.cpp` (Xbox PDB source unit), subsystem `BSAudio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! On PC the audio object of this unit is `BSWin32Audio` (RTTI `.?AVBSWin32Audio@@`, vtable
//! `010a39d8`, the global at `011f7080`); its listener is `BSWin32AudioListener` (RTTI
//! `.?AVBSWin32AudioListener@@`, vtable `010a3b7c`). Their DirectSound fields are PC-only and
//! are read at their offsets below, named by what the code does with them.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsaudio::bsaudio::BSAudio;

/// `BSWin32Audio` +0x38: the `IDirectSound8*` that `DirectSoundCreate8` writes.
const DIRECT_SOUND: u32 = 0x38;
/// `BSWin32Audio` +0x3c: the primary `IDirectSoundBuffer*` that `CreateSoundBuffer` writes.
const PRIMARY_BUFFER: u32 = 0x3c;
/// `BSWin32Audio` +0x40: the 0x60-byte capabilities block `GetCaps` fills (its first word is
/// the block size, 0x60).
const CAPS: u32 = 0x40;
/// `BSWin32Audio` +0xa0: the window handle `Init` receives.
const WINDOW: u32 = 0xa0;

/// `CoInitialize` (import slot).
const CO_INITIALIZE: u32 = 0x00fd_f454;
/// `DirectSoundCreate8` (import thunk `00ee81e0`, `JMP [00fdf034]`).
const DIRECT_SOUND_CREATE8: u32 = 0x00ee_81e0;
/// `memset` (CRT, cdecl).
const MEMSET: u32 = 0x00ec_61c0;
/// The memory manager's allocation `00aa3e40` (`thiscall` on the manager, `RET 4`: size).
const ALLOCATE: u32 = 0x00aa_3e40;
/// The memory manager the code allocates through (`ECX` = `011f6238`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `BSAudioManager::QInstance` (engine map; no stack words).
const MANAGER_INSTANCE: u32 = 0x00ad_9060;
/// `BSAudioManager::Initialize` (engine map; `thiscall`, no stack words).
const MANAGER_INITIALIZE: u32 = 0x00ad_9640;
/// `BSAudio::Init` (engine map; `thiscall`, `RET 4`).
const AUDIO_INIT: u32 = 0x00ad_7020;
/// The error report `0040fbe0` (cdecl, variable arguments).
const ERROR: u32 = 0x0040_fbe0;

/// The setting object whose value byte (+4, `011f6da0`) enables audio. The code tests the
/// object's constant address for null before taking the value's address.
const AUDIO_SETTING: u32 = 0x011f_6d9c;
/// The value byte of [`AUDIO_SETTING`].
const AUDIO_SETTING_VALUE: u32 = 0x011f_6da0;
/// The fallback byte the code clears and reads when [`AUDIO_SETTING`] is null.
const AUDIO_SETTING_FALLBACK: u32 = 0x0120_2800;

/// The listener interface's IID, read from the exe at `01100fa4`; its bytes are
/// `279AFA84-4981-11CE-A521-0020AF0BE560` (the SDK's `IID_IDirectSound3DListener`).
const LISTENER_IID: u32 = 0x0110_0fa4;
/// The float the listener's position is filled with (`01016970`, `0x7f7fffff` in the exe).
const FAR_POSITION: u32 = 0x0101_6970;

/// "Unable to create DirectSound Interface. Audio will be disabled."
const MSG_CREATE: u32 = 0x010a_3b10;
/// "Unable to set DirectSound Cooperative Level."
const MSG_COOPERATIVE_LEVEL: u32 = 0x010a_3ae0;
/// "Unable to determine DirectSound interface capabilities. Some audio functionality may not
/// be available."
const MSG_CAPS: u32 = 0x010a_3a78;
/// "Unable to create Primary Sound Buffer. Audio will be disabled."
const MSG_PRIMARY_BUFFER: u32 = 0x010a_3a38;
/// "Unable to obtain Listener Interface. Audio will be disabled."
const MSG_LISTENER: u32 = 0x010a_39f8;
/// "Audio system could not be initialized."
const MSG_NOT_INITIALIZED: u32 = 0x010a_3b50;

/// `BSAudioListener`'s vtable (`010a3bb4`, RTTI `.?AVBSAudioListener@@`).
const LISTENER_BASE_VTABLE: u32 = 0x010a_3bb4;
/// `BSWin32AudioListener`'s vtable (`010a3b7c`, RTTI `.?AVBSWin32AudioListener@@`).
const WIN32_LISTENER_VTABLE: u32 = 0x010a_3b7c;

// Translated from 00aea590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the result check `BSWin32Audio`'s `Init` runs after each
/// DirectSound call. `thiscall`, two words (`RET 8`): an `HRESULT` and a message. A negative
/// result reports "Audio system could not be initialized." (`0040fbe0`, that string alone: the
/// message word is never read) and answers false; otherwise answers true.
pub fn fn_00aea590(e: &mut Engine, _this: Ptr, result: i32, _unused_message: u32) -> bool {
    if result >= 0 {
        return true;
    }
    e.call(ERROR, &args![MSG_NOT_INITIALIZED]);
    false
}

// Translated from 00aea730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSWin32AudioListener` constructor (by the vtables it
/// stores). `thiscall`, one word (`RET 4`): the DirectSound listener interface. Stores
/// `BSAudioListener`'s vtable, then `BSWin32AudioListener`'s, clears +0x48, stores 0.0 at +0x3c
/// (the Xbox PDB's `BSAudioListener::fRolloffFactor` offset), then stores the interface at
/// +0x48. Answers `this`.
pub fn fn_00aea730(e: &mut Engine, this: Ptr, listener: u32) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, LISTENER_BASE_VTABLE);
    e.mem.set_u32(at, WIN32_LISTENER_VTABLE);
    e.mem.set_u32(at + 0x48, 0);
    // FLDZ; FSTP float: +0.0.
    e.mem.set_u32(at + 0x3c, 0);
    e.mem.set_u32(at + 0x48, listener);
    this
}

// Translated from 00aea230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSXenonAudio::Init` (Xbox PDB; on PC `BSWin32Audio`'s vtable slot +0x8). `thiscall`, one
/// word (`RET 4`): the window handle, stored at +0xa0. Does nothing more once `bInitialized`
/// (+5) is set. When the audio setting's value byte is set: `CoInitialize(NULL)`,
/// `DirectSoundCreate8(NULL, this + 0x38, NULL)` (a failure stops here), then on the
/// `IDirectSound8`: `SetCooperativeLevel` (slot +0x18) with the window and 2, `GetCaps` (slot
/// +0x10) into +0x40 with its size word 0x60 (both results only reported), `CreateSoundBuffer`
/// (slot +0xc) of a 0x24-byte description with flags 0x91 into +0x3c (a failure stops here),
/// then `QueryInterface` (slot +0) on the primary buffer for the listener IID (a failure stops
/// here); a 100-byte `BSWin32AudioListener` is allocated and constructed around that
/// interface and becomes `pListener` (+0x10; null when the allocation fails). With a listener,
/// its position (slot +0x8) is set to three copies of the float at `01016970` and its slot
/// +0x30 (`SetRolloffFactor` in the Xbox PDB) gets 1.0. Then `bInitialized` and
/// `bAudioEnabled` are set, `BSAudioManager::Initialize` runs on the manager and
/// `BSAudio::Init(this, 0)`.
// Translation incomplete: 00aea230 - C++ exception unwinding (SEH frame state around the listener's allocation) and the /GS stack cookie check (00ec408c) are not translated.
pub fn bs_xenon_audio_init(e: &mut Engine, this: Ptr<BSAudio>, window: u32) {
    let at = this.addr();
    e.mem.set_u32(at + WINDOW, window);
    if e.get(this, BSAudio::bInitialized) != 0 {
        return;
    }
    // The compiler tests the setting object's constant address: never null, so the fallback
    // branch cannot run; translated as the exe has it.
    let setting = AUDIO_SETTING;
    let enabled_at = if setting != 0 {
        AUDIO_SETTING_VALUE
    } else {
        e.mem.set_u8(AUDIO_SETTING_FALLBACK, 0);
        AUDIO_SETTING_FALLBACK
    };
    if e.mem.u8(enabled_at) != 0 {
        let created = e.with_stack(0x40, |e, frame| {
            // The game's frame from EBP-0x50: the position (-0x50, used below), the listener
            // interface (-0x3c) and the buffer description (-0x34).
            let listener_out = frame.addr() + 0x14;
            let description = frame.addr() + 0x1c;
            e.call(CO_INITIALIZE, &args![0u32]);
            let result = e
                .call(DIRECT_SOUND_CREATE8, &args![0u32, at + DIRECT_SOUND, 0u32])
                .u32();
            if !fn_00aea590(e, this.cast(), result as i32, MSG_CREATE) {
                return false;
            }
            let window = e.mem.u32(at + WINDOW);
            let sound = e.mem.u32(at + DIRECT_SOUND);
            let result = e.vcall(sound, 0x18, &args![window, 2u32]).u32();
            fn_00aea590(e, this.cast(), result as i32, MSG_COOPERATIVE_LEVEL);
            e.mem.set_u32(at + CAPS, 0x60);
            let sound = e.mem.u32(at + DIRECT_SOUND);
            let result = e.vcall(sound, 0x10, &args![at + CAPS]).u32();
            fn_00aea590(e, this.cast(), result as i32, MSG_CAPS);
            e.call(MEMSET, &args![description, 0u32, 0x24u32]);
            e.mem.set_u32(description, 0x24);
            e.mem.set_u32(description + 4, 0x91);
            let sound = e.mem.u32(at + DIRECT_SOUND);
            let result = e
                .vcall(sound, 0xc, &args![description, at + PRIMARY_BUFFER, 0u32])
                .u32();
            if !fn_00aea590(e, this.cast(), result as i32, MSG_PRIMARY_BUFFER) {
                return false;
            }
            let buffer = e.mem.u32(at + PRIMARY_BUFFER);
            let result = e.vcall(buffer, 0, &args![LISTENER_IID, listener_out]).u32();
            if !fn_00aea590(e, this.cast(), result as i32, MSG_LISTENER) {
                return false;
            }
            let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 100u32]).u32();
            let listener = if block != 0 {
                let interface = e.mem.u32(listener_out);
                fn_00aea730(e, Ptr::new(block), interface).addr()
            } else {
                0
            };
            e.set(this, BSAudio::pListener, Ptr::new(listener));
            true
        });
        if !created {
            return;
        }
    }
    if !e.get(this, BSAudio::pListener).is_null() {
        e.with_stack(12, |e, position| {
            let p = position.addr();
            // FLD float; FSTP float, three times.
            let far = x87_quiet_bits(e.global::<u32>(FAR_POSITION));
            e.mem.set_u32(p, far);
            let far = x87_quiet_bits(e.global::<u32>(FAR_POSITION));
            e.mem.set_u32(p + 4, far);
            let far = x87_quiet_bits(e.global::<u32>(FAR_POSITION));
            e.mem.set_u32(p + 8, far);
            let listener = e.get(this, BSAudio::pListener).addr();
            e.vcall(listener, 0x8, &args![p]);
        });
        let listener = e.get(this, BSAudio::pListener).addr();
        // FLD1; FSTP float [ESP]: 1.0 pushed as its bit pattern.
        e.vcall(listener, 0x30, &args![1.0f32.to_bits()]);
    }
    e.set(this, BSAudio::bInitialized, 1);
    e.set(this, BSAudio::bAudioEnabled, 1);
    let manager = e.call(MANAGER_INSTANCE, &[]).u32();
    e.call(MANAGER_INITIALIZE, &args![manager]);
    e.call(AUDIO_INIT, &args![this, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ae_a590, fn_00aea590(Ptr, i32, u32) -> bool),
        entry!(0x00ae_a730, fn_00aea730(Ptr, u32) -> Ptr),
        entry!(0x00ae_a230, bs_xenon_audio_init(Ptr<BSAudio>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00aea590_reports_only_negative_results() {
        let mut e = Engine::new();
        e.register(ERROR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00ae_a590, &args![0x100u32, 0u32, MSG_CREATE])
            .bool());
        assert!(e
            .call(0x00ae_a590, &args![0x100u32, 1u32, MSG_CREATE])
            .bool());
        assert!(calls_to(e.call_log.as_ref().unwrap(), ERROR).is_empty());
        assert!(!e
            .call(0x00ae_a590, &args![0x100u32, 0x8007_000eu32, MSG_CREATE])
            .bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ERROR), vec![vec![MSG_NOT_INITIALIZED]]);
    }

    #[test]
    fn fn_00aea730_builds_the_win32_listener() {
        let mut e = Engine::new();
        let this = e.mem.alloc(100);
        e.mem.write(this, &[0xcd; 100]);
        assert_eq!(e.call(0x00ae_a730, &args![this, 0x4444u32]).u32(), this);
        assert_eq!(e.mem.u32(this), WIN32_LISTENER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0x4444);
        assert_eq!(e.mem.u32(this + 4), 0xcdcd_cdcd);
    }

    const DS_VTABLE: u32 = 0x0300_0000;
    const BUFFER_VTABLE: u32 = 0x0300_1000;
    const DS_CREATE_BUFFER: u32 = 0x0310_000c;
    const DS_GET_CAPS: u32 = 0x0310_0010;
    const DS_COOPERATIVE: u32 = 0x0310_0018;
    const BUFFER_QUERY: u32 = 0x0320_0000;
    const LISTENER_POSITION: u32 = 0x0330_0008;
    const LISTENER_ROLLOFF: u32 = 0x0330_0030;

    /// Engine with the audio object, the DirectSound doubles and the given setting byte.
    /// `fail_at` names the double that answers a failure (`E_FAIL`).
    fn audio_engine(enabled: u8, fail_at: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(AUDIO_SETTING, 8);
        e.mem.set_u8(AUDIO_SETTING_VALUE, enabled);
        e.map(FAR_POSITION, 4);
        e.mem.set_u32(FAR_POSITION, 0x7f7f_ffff);
        let this = e.mem.alloc(0xa4);
        let mut ds_slots = vec![0u32; 7];
        ds_slots[3] = DS_CREATE_BUFFER;
        ds_slots[4] = DS_GET_CAPS;
        ds_slots[6] = DS_COOPERATIVE;
        e.put_vtable(DS_VTABLE, &ds_slots);
        e.put_vtable(BUFFER_VTABLE, &[BUFFER_QUERY]);
        let mut listener_slots = vec![0u32; 13];
        listener_slots[2] = LISTENER_POSITION;
        listener_slots[12] = LISTENER_ROLLOFF;
        e.put_vtable(WIN32_LISTENER_VTABLE, &listener_slots);
        let sound = e.mem.alloc(4);
        e.mem.set_u32(sound, DS_VTABLE);
        let buffer = e.mem.alloc(4);
        e.mem.set_u32(buffer, BUFFER_VTABLE);
        let interface = e.mem.alloc(4);
        let fail = |addr: u32| if addr == fail_at { 0x8000_4005u32 } else { 0 };
        let r_create = fail(DIRECT_SOUND_CREATE8);
        let r_coop = fail(DS_COOPERATIVE);
        let r_buffer = fail(DS_CREATE_BUFFER);
        let r_query = fail(BUFFER_QUERY);
        e.register(CO_INITIALIZE, |_, _| Ret::default());
        e.register_double(DIRECT_SOUND_CREATE8, move |e, a| {
            e.mem.set_u32(a[1], sound);
            r_create.into_ret()
        });
        e.register_double(DS_COOPERATIVE, move |_, _| r_coop.into_ret());
        e.register(DS_GET_CAPS, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x60);
            0u32.into_ret()
        });
        e.register(MEMSET, |e, a| {
            let bytes = vec![a[1] as u8; a[2] as usize];
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register_double(DS_CREATE_BUFFER, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x24);
            assert_eq!(e.mem.u32(a[1] + 4), 0x91);
            assert_eq!(e.mem.bytes(a[1] + 8, 0x1c), vec![0; 0x1c]);
            e.mem.set_u32(a[2], buffer);
            r_buffer.into_ret()
        });
        e.register_double(BUFFER_QUERY, move |e, a| {
            e.mem.set_u32(a[2], interface);
            r_query.into_ret()
        });
        e.register(ALLOCATE, |e, _| e.mem.alloc(100).into_ret());
        e.register(ERROR, |_, _| Ret::default());
        e.register(LISTENER_POSITION, |e, a| {
            assert_eq!(e.mem.bytes(a[1], 12), [0xff, 0xff, 0x7f, 0x7f].repeat(3));
            Ret::default()
        });
        e.register(LISTENER_ROLLOFF, |_, _| Ret::default());
        e.register(MANAGER_INSTANCE, |_, _| 0x7777u32.into_ret());
        e.register(MANAGER_INITIALIZE, |_, _| Ret::default());
        e.register(AUDIO_INIT, |_, _| Ret::default());
        (e, this)
    }

    #[test]
    fn bs_xenon_audio_init_creates_direct_sound_and_the_listener() {
        let (mut e, this) = audio_engine(1, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ae_a230, &args![this, 0xabcu32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + WINDOW), 0xabc);
        assert_eq!(calls_to(&log, CO_INITIALIZE), vec![vec![0]]);
        assert_eq!(
            calls_to(&log, DIRECT_SOUND_CREATE8),
            vec![vec![0, this + 0x38, 0]]
        );
        let sound = e.mem.u32(this + 0x38);
        assert_eq!(calls_to(&log, DS_COOPERATIVE), vec![vec![sound, 0xabc, 2]]);
        assert_eq!(calls_to(&log, DS_GET_CAPS), vec![vec![sound, this + 0x40]]);
        let create = calls_to(&log, DS_CREATE_BUFFER);
        assert_eq!(create.len(), 1);
        assert_eq!(create[0][0], sound);
        assert_eq!(create[0][2..], [this + 0x3c, 0]);
        let buffer = e.mem.u32(this + 0x3c);
        let query = calls_to(&log, BUFFER_QUERY);
        assert_eq!(query.len(), 1);
        assert_eq!(query[0][..2], [buffer, LISTENER_IID]);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![MEMORY_MANAGER, 100]]);
        let listener = e.mem.u32(this + 0x10);
        assert_ne!(listener, 0);
        assert_eq!(e.mem.u32(listener), WIN32_LISTENER_VTABLE);
        assert_ne!(e.mem.u32(listener + 0x48), 0);
        assert_eq!(calls_to(&log, LISTENER_POSITION).len(), 1);
        assert_eq!(
            calls_to(&log, LISTENER_ROLLOFF),
            vec![vec![listener, 0x3f80_0000]]
        );
        assert_eq!(e.mem.u8(this + 4), 1);
        assert_eq!(e.mem.u8(this + 5), 1);
        assert_eq!(calls_to(&log, MANAGER_INITIALIZE), vec![vec![0x7777]]);
        assert_eq!(calls_to(&log, AUDIO_INIT), vec![vec![this, 0]]);
        assert!(calls_to(&log, ERROR).is_empty());
    }

    #[test]
    fn bs_xenon_audio_init_stops_when_a_required_step_fails() {
        for fail_at in [DIRECT_SOUND_CREATE8, DS_CREATE_BUFFER, BUFFER_QUERY] {
            let (mut e, this) = audio_engine(1, fail_at);
            e.call_log = Some(vec![]);
            e.call(0x00ae_a230, &args![this, 0xabcu32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, ERROR), vec![vec![MSG_NOT_INITIALIZED]]);
            assert_eq!(e.mem.u8(this + 5), 0);
            assert_eq!(e.mem.u32(this + 0x10), 0);
            assert!(calls_to(&log, AUDIO_INIT).is_empty());
            assert!(calls_to(&log, ALLOCATE).is_empty());
        }
    }

    #[test]
    fn bs_xenon_audio_init_goes_on_after_a_cooperative_level_failure() {
        let (mut e, this) = audio_engine(1, DS_COOPERATIVE);
        e.call_log = Some(vec![]);
        e.call(0x00ae_a230, &args![this, 0xabcu32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ERROR).len(), 1);
        assert_eq!(calls_to(&log, AUDIO_INIT), vec![vec![this, 0]]);
    }

    #[test]
    fn bs_xenon_audio_init_without_audio_or_when_initialized() {
        // Audio disabled: no DirectSound, no listener, but the audio manager starts.
        let (mut e, this) = audio_engine(0, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ae_a230, &args![this, 0x55u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, CO_INITIALIZE).is_empty());
        assert!(calls_to(&log, LISTENER_POSITION).is_empty());
        assert_eq!(e.mem.u8(this + 5), 1);
        assert_eq!(calls_to(&log, AUDIO_INIT), vec![vec![this, 0]]);
        // Already initialized: only the window is stored.
        e.call_log = Some(vec![]);
        e.call(0x00ae_a230, &args![this, 0x66u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + WINDOW), 0x66);
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn bs_xenon_audio_init_keeps_a_null_listener_when_allocation_fails() {
        let (mut e, this) = audio_engine(1, 0);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00ae_a230, &args![this, 0xabcu32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert!(calls_to(&log, LISTENER_POSITION).is_empty());
        assert_eq!(calls_to(&log, AUDIO_INIT), vec![vec![this, 0]]);
    }
}
