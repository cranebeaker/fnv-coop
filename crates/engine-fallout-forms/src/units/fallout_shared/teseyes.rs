//! `fallout shared/teseyes.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005fc050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fc050 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESEyes::TESEyes` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0xb (0x34 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn teseyes_teseyes(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_e270, &args![base + 0x24]);
    e.mem.set_u32(base, 0x0104_973c);
    e.mem.set_u32(base + 0x18, 0x0104_9724);
    e.mem.set_u32(base + 0x24, 0x0104_9704);
    e.call(0x004f_15a0, &args![this, 0xb]);
    e.call(0x005f_c1b0, &args![this]);
    this
}

// Translated from 005fc1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears `TESEyes`' flags byte (`cFlags`, Xbox PDB; PC +0x30).
pub fn fn_005fc1b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x30, 0);
}

// Translated from 005fc120 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fc120 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `TESEyes`' destructor body: stores the three vtables (`TESEyes`, its
/// `TESFullName` at +0x18 and `TESTexture` at +0x24), then destroys the
/// texture (`0048e2e0`), the full name (`00591300`) and the `TESForm` base
/// (`TESForm::~TESForm`, `00483630`).
pub fn fn_005fc120(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0104_973c);
    e.mem.set_u32(base + 0x18, 0x0104_9724);
    e.mem.set_u32(base + 0x24, 0x0104_9704);
    e.call(0x0048_e2e0, &args![base + 0x24]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 005fc0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEyes`' scalar deleting destructor: the destructor body
/// (`005fc120`), then frees `this` (`00401030`) when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_005fc0f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_005fc120(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `TESForm::StartForm` (`004855a0`), `TESFullName::Save` (`00487010`),
/// `TESTexture::Save` (`0048e370`, one chunk word), `TESForm::AddChunk`
/// with a byte (`004858f0`, `cdecl`) and `TESForm::CloseForm` (`00485680`).
const SAVE_FORM_START: u32 = 0x0048_55a0;
const SAVE_FULL_NAME: u32 = 0x0048_7010;
const SAVE_TEXTURE: u32 = 0x0048_e370;
const SAVE_CHUNK_BYTE: u32 = 0x0048_58f0;
const SAVE_FORM_CLOSE: u32 = 0x0048_5680;

// Translated from 005fc1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEyes::Save` (Xbox PDB): starts the form, saves the full name
/// (+0x18) and the texture (+0x24, `ICON`), adds the flags byte at +0x30
/// as `DATA` and closes the form.
pub fn tes_eyes_save(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.call(SAVE_FORM_START, &args![this]);
    e.call(SAVE_FULL_NAME, &args![at + 0x18]);
    e.call(SAVE_TEXTURE, &args![at + 0x24, 0x4e4f_4349u32]);
    let flags = u32::from(e.mem.u8(at + 0x30));
    e.call(SAVE_CHUNK_BYTE, &args![0x4154_4144u32, flags]);
    e.call(SAVE_FORM_CLOSE, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_c050, teseyes_teseyes(Ptr) -> Ptr),
        entry!(0x005f_c1b0, fn_005fc1b0(Ptr)),
        entry!(0x005f_c120, fn_005fc120(Ptr)),
        entry!(0x005f_c0f0, fn_005fc0f0(Ptr, u32) -> Ptr),
        entry!(0x005f_c1d0, tes_eyes_save(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn teseyes_teseyes_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_c050,
            size: 0x34,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_e270, &[A::T(0x24)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0xb)]),
                (0x005f_c1b0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_973c),
                (0x18, 4, 0x0104_9724),
                (0x24, 4, 0x0104_9704),
            ],
        });
    }
}

#[cfg(test)]
mod b020_tests {
    use crate::prelude::*;

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != skip)
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for a in [0x0048_e2e0, 0x0059_1300, 0x0048_3630, 0x0040_1030] {
            e.register(a, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn b020_005fc1b0_clears_the_flags() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x34);
        e.mem.set_u32(p + 0x30, 0xffff_ffff);
        e.call(0x005f_c1b0, &args![p]);
        assert_eq!(e.mem.u32(p + 0x30), 0xffff_ff00);
    }

    #[test]
    fn b020_005fc120_destroys_the_eyes() {
        let mut e = engine();
        let p = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        e.call(0x005f_c120, &args![p]);
        assert_eq!(e.mem.u32(p), 0x0104_973c);
        assert_eq!(e.mem.u32(p + 0x18), 0x0104_9724);
        assert_eq!(e.mem.u32(p + 0x24), 0x0104_9704);
        assert_eq!(
            calls(&mut e, 0x005f_c120),
            vec![
                (0x0048_e2e0, vec![p + 0x24]),
                (0x0059_1300, vec![p + 0x18]),
                (0x0048_3630, vec![p]),
            ]
        );
    }

    #[test]
    fn b020_005fc0f0_frees_on_bit_0() {
        let mut e = engine();
        let p = e.mem.alloc(0x34);
        for (flags, frees) in [(0u32, false), (2, false), (3, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x005f_c0f0, &args![p, flags]).u32(), p);
            let log = calls(&mut e, 0x005f_c0f0);
            assert_eq!(log.len(), if frees { 4 } else { 3 });
            assert_eq!(log[0], (0x0048_e2e0, vec![p + 0x24]));
            if frees {
                assert_eq!(log[3], (0x0040_1030, vec![p]));
            }
        }
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    #[test]
    fn save_writes_name_texture_and_flags() {
        let mut e = Engine::new();
        for addr in [
            SAVE_FORM_START,
            SAVE_FULL_NAME,
            SAVE_TEXTURE,
            SAVE_CHUNK_BYTE,
            SAVE_FORM_CLOSE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x34);
        e.mem.set_u8(this + 0x30, 3);
        e.call_log = Some(vec![]);
        e.call(0x005f_c1d0, &args![this]);
        let log: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (SAVE_FORM_START, vec![this]),
                (SAVE_FULL_NAME, vec![this + 0x18]),
                (SAVE_TEXTURE, vec![this + 0x24, 0x4e4f_4349]),
                (SAVE_CHUNK_BYTE, vec![0x4154_4144, 3]),
                (SAVE_FORM_CLOSE, vec![this]),
            ]
        );
    }
}
