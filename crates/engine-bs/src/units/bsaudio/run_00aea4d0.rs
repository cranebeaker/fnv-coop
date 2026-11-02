//! `BSAudio/run_00aea4d0` (Xbox PDB source unit), subsystem `BSAudio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bsaudio::bsaudio::BSAudio;

/// `BSAudio::Shutdown` (engine map, `00ad7150`, `thiscall`, no stack words).
const AUDIO_SHUTDOWN: u32 = 0x00ad_7150;
/// Imported `CoUninitialize` (import slot `00fdf464`, no arguments).
const IMPORT_CO_UNINITIALIZE: u32 = 0x00fd_f464;

// Translated from 00aea4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the PC audio object's shutdown.
/// Clears `bInitialized` (+5), runs `BSAudio::Shutdown`, then `CoUninitialize`. Deletes the
/// listener (`pListener`, +0x10) through its vtable slot 0 with 1 when there is one and clears
/// the word; then releases the COM objects at +0x3c and at +0x38 (PC words past the Xbox
/// PDB's 0x38-byte `BSAudio`) through their vtable slot +8 (`Release`, the object pushed),
/// each when not null, and clears them.
pub fn fn_00aea4d0(e: &mut Engine, this: Ptr<BSAudio>) {
    e.set(this, BSAudio::bInitialized, 0);
    e.call(AUDIO_SHUTDOWN, &args![this]);
    e.call(IMPORT_CO_UNINITIALIZE, &args![]);
    let listener = e.get(this, BSAudio::pListener);
    if !listener.is_null() {
        e.vcall(listener.addr(), 0, &args![1u32]);
        e.set(this, BSAudio::pListener, Ptr::new(0));
    }
    for offset in [0x3cu32, 0x38] {
        let object = e.mem.u32(this.addr() + offset);
        if object != 0 {
            e.vcall(object, 8, &args![]);
            e.mem.set_u32(this.addr() + offset, 0);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ae_a4d0, fn_00aea4d0(Ptr<BSAudio>))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const LISTENER_DELETE: u32 = 0x0e10_0000;
    const RELEASE_3C: u32 = 0x0e10_0010;
    const RELEASE_38: u32 = 0x0e10_0020;

    #[test]
    fn fn_00aea4d0_shuts_down_and_releases_what_it_holds() {
        let mut e = Engine::new();
        for callee in [
            AUDIO_SHUTDOWN,
            IMPORT_CO_UNINITIALIZE,
            LISTENER_DELETE,
            RELEASE_3C,
            RELEASE_38,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = Ptr::<BSAudio>::new(e.mem.alloc(0x40));
        e.set(this, BSAudio::bInitialized, 1);
        let mut objects = vec![];
        for (offset, slot, target) in [
            (0x10u32, 0u32, LISTENER_DELETE),
            (0x3c, 8, RELEASE_3C),
            (0x38, 8, RELEASE_38),
        ] {
            let vtable = e.mem.alloc(0x10);
            e.mem.set_u32(vtable + slot, target);
            let object = e.mem.alloc(8);
            e.mem.set_u32(object, vtable);
            e.mem.set_u32(this.addr() + offset, object);
            objects.push(object);
        }
        e.call_log = Some(vec![]);
        e.call(0x00ae_a4d0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (AUDIO_SHUTDOWN, vec![this.addr()]),
                (IMPORT_CO_UNINITIALIZE, vec![]),
                (LISTENER_DELETE, vec![objects[0], 1]),
                (RELEASE_3C, vec![objects[1]]),
                (RELEASE_38, vec![objects[2]]),
            ]
        );
        assert_eq!(e.get(this, BSAudio::bInitialized), 0);
        for offset in [0x10u32, 0x38, 0x3c] {
            assert_eq!(e.mem.u32(this.addr() + offset), 0, "{offset:#x}");
        }

        // Nothing held: only the shutdown and CoUninitialize.
        e.call_log = Some(vec![]);
        e.call(0x00ae_a4d0, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
    }
}
