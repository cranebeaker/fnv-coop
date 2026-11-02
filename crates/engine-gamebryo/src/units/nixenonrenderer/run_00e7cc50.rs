//! `NiXenonRenderer/run_00e7cc50` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// A table of words by render-target index (`0126f580`; entry 0 is never touched here) and
/// the word `0126f590` that `00e7cca0` clears.
const TARGETS_0126F580: u32 = 0x0126_f580;
const DEPTH_0126F590: u32 = 0x0126_f590;

// Translated from 00e7cc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cc50`), `cdecl`, two words (the Direct3D device, an
/// index): for a non-zero index whose entry of the table `0126f580` is set, calls the device's
/// slot +0x94 with the index and 0 (in the `IDirect3DDevice9` order that slot is
/// `SetRenderTarget`; not confirmed in the exe) and clears the entry. Index 0 does nothing.
pub fn fn_00e7cc50(e: &mut Engine, device: u32, index: u32) {
    if index == 0 {
        return;
    }
    let entry = TARGETS_0126F580.wrapping_add(index.wrapping_mul(4));
    if e.mem.u32(entry) == 0 {
        return;
    }
    e.vcall(device, 0x94, &args![index, 0u32]);
    e.mem.set_u32(entry, 0);
}

// Translated from 00e7cc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cc50`), `thiscall`, no stack words (a vtable slot):
/// calls the object's own slot +0x68 and answers true.
pub fn fn_00e7cc90(e: &mut Engine, this: Ptr) -> bool {
    e.vcall(this.addr(), 0x68, &args![]);
    true
}

// Translated from 00e7cca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cc50`), `cdecl`, one word (the Direct3D device): when
/// the word `0126f590` is set, calls the device's slot +0x9c with 0 (in the
/// `IDirect3DDevice9` order `SetDepthStencilSurface`; not confirmed in the exe); a negative
/// result answers false and keeps the word, else the word is cleared. Answers true otherwise
/// (only `AL` is set; the caller tests `AL`).
pub fn fn_00e7cca0(e: &mut Engine, device: u32) -> bool {
    if e.global::<u32>(DEPTH_0126F590) != 0 {
        let result = e.vcall(device, 0x9c, &args![0u32]).u32();
        if (result as i32) < 0 {
            return false;
        }
        e.set_global(DEPTH_0126F590, 0u32);
    }
    true
}

// Translated from 00e7ccd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cc50`), `thiscall`, no stack words (a vtable slot):
/// calls slot +0x44 of the object at +0x58 with four zero words and answers whether the
/// result is not negative (`SETGE`; only `AL` is set).
pub fn fn_00e7ccd0(e: &mut Engine, this: Ptr) -> bool {
    let object = e.mem.u32(this.addr() + 0x58);
    let result = e.vcall(object, 0x44, &args![0u32, 0u32, 0u32, 0u32]).u32();
    (result as i32) >= 0
}

// Translated from 00e7ccf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cc50`), `cdecl`, one word: 2, 4 and 8 for 1, 2 and 3
/// (the compiler's jump table at `00e7cd18`), 0 for every other value.
pub fn fn_00e7ccf0(_e: &mut Engine, value: u32) -> u32 {
    match value {
        1 => 2,
        2 => 4,
        3 => 8,
        _ => 0,
    }
}

/// The address `00e7cd30` answers (`0126f59c`).
const RECORD_0126F59C: u32 = 0x0126_f59c;

// Translated from 00e7cd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no argument read; the shape of a `GetRTTI`):
/// answers the address `0126f59c`.
pub fn fn_00e7cd30(_e: &mut Engine) -> u32 {
    RECORD_0126F59C
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_cd30, fn_00e7cd30() -> u32),
        entry!(0x00e7_cc50, fn_00e7cc50(u32, u32)),
        entry!(0x00e7_cc90, fn_00e7cc90(Ptr) -> bool),
        entry!(0x00e7_cca0, fn_00e7cca0(u32) -> bool),
        entry!(0x00e7_ccd0, fn_00e7ccd0(Ptr) -> bool),
        entry!(0x00e7_ccf0, fn_00e7ccf0(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0e50_0000;
    const SLOT_44: u32 = 0x0e51_0044;
    const SLOT_68: u32 = 0x0e51_0068;
    const SLOT_94: u32 = 0x0e51_0094;
    const SLOT_9C: u32 = 0x0e51_009c;

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x28];
        slots[0x44 / 4] = SLOT_44;
        slots[0x68 / 4] = SLOT_68;
        slots[0x94 / 4] = SLOT_94;
        slots[0x9c / 4] = SLOT_9C;
        e.put_vtable(VTABLE, &slots);
        for slot in [SLOT_44, SLOT_68, SLOT_94] {
            e.register(slot, |_, _| Ret::default());
        }
        e.map(0x0126_f000, 0x1000);
        let device = e.mem.alloc(0x60);
        e.mem.set_u32(device, VTABLE);
        e.call_log = Some(vec![]);
        (e, device)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00e7cc50_clears_a_set_target() {
        let (mut e, device) = engine();
        e.mem.set_u32(TARGETS_0126F580 + 8, 0x77);
        e.call(0x00e7_cc50, &args![device, 2u32]);
        assert_eq!(calls(&e, SLOT_94), vec![vec![device, 2, 0]]);
        assert_eq!(e.mem.u32(TARGETS_0126F580 + 8), 0);
        // An entry already clear, and index 0 (its entry set), do nothing.
        e.mem.set_u32(TARGETS_0126F580, 0x55);
        e.call(0x00e7_cc50, &args![device, 2u32]);
        e.call(0x00e7_cc50, &args![device, 0u32]);
        assert_eq!(calls(&e, SLOT_94).len(), 1);
        assert_eq!(e.mem.u32(TARGETS_0126F580), 0x55);
    }

    #[test]
    fn fn_00e7cc90_calls_its_slot_and_answers_true() {
        let (mut e, object) = engine();
        assert!(e.call(0x00e7_cc90, &args![object]).bool());
        assert_eq!(calls(&e, SLOT_68), vec![vec![object]]);
    }

    #[test]
    fn fn_00e7cca0_clears_the_depth_word_unless_the_call_fails() {
        let (mut e, device) = engine();
        e.register(SLOT_9C, |_, _| 0x8000_1234u32.into_ret());
        e.set_global(DEPTH_0126F590, 1u32);
        assert!(!e.call(0x00e7_cca0, &args![device]).bool());
        assert_eq!(e.global::<u32>(DEPTH_0126F590), 1);
        e.register(SLOT_9C, |_, _| Ret::default());
        assert!(e.call(0x00e7_cca0, &args![device]).bool());
        assert_eq!(e.global::<u32>(DEPTH_0126F590), 0);
        assert!(e.call(0x00e7_cca0, &args![device]).bool());
        assert_eq!(calls(&e, SLOT_9C), vec![vec![device, 0], vec![device, 0]]);
    }

    #[test]
    fn fn_00e7ccd0_answers_whether_the_call_succeeded() {
        let (mut e, inner) = engine();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x58, inner);
        assert!(e.call(0x00e7_ccd0, &args![this]).bool());
        assert_eq!(calls(&e, SLOT_44), vec![vec![inner, 0, 0, 0, 0]]);
        e.register(SLOT_44, |_, _| 0x8876_0868u32.into_ret());
        assert!(!e.call(0x00e7_ccd0, &args![this]).bool());
    }

    #[test]
    fn fn_00e7ccf0_maps_the_cases() {
        let mut e = Engine::new();
        let answers: Vec<u32> = [0u32, 1, 2, 3, 4, 0xffff_ffff]
            .iter()
            .map(|v| e.call(0x00e7_ccf0, &args![*v]).u32())
            .collect();
        assert_eq!(answers, vec![0, 2, 4, 8, 0, 0]);
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    #[test]
    fn fn_00e7cd30_answers_the_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e7_cd30, &args![]).u32(), 0x0126_f59c);
    }
}
