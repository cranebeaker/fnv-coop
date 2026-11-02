//! `fallout shared/run_0048e270` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// The `BSStringT<char>` constructor (`004037b0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// A `BSStringT`'s text setter (`004037f0`, `RET 8`): (text, length).
const SET_TEXT: u32 = 0x0040_37f0;
/// The string's text pointer (`00559450`: the word at `this`).
const TEXT_POINTER: u32 = 0x0055_9450;
/// This class's vtable.
const VTABLE: u32 = 0x0101_d7c4;

// Translated from 0048e270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `TESTexture` (the engine map gives it no name; in the
/// Xbox PDB `TESTexture` is a `BaseFormComponent` followed by the
/// `TextureName` string at +4, and `TESIcon` derives from it): runs the base
/// constructor, sets the vtable, constructs the string at `+4` and clears it
/// with `0048e340`. Returns `this`.
// Translation incomplete: 0048e270 - SEH exception unwinding and the stack security cookie are unsupported.
pub fn fn_0048e270(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(STRING_CONSTRUCT, &args![this.byte_add(4)]);
    fn_0048e340(e, this);
    this
}

// Translated from 0048e340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the texture name: when the string at `+4` has text, sets its text
/// to (0, 0) with `004037f0`. (Many vtables list this address as well as the
/// constructor calling it; which slot is not checked here.)
pub fn fn_0048e340(e: &mut Engine, this: Ptr) {
    let name = this.byte_add(4);
    if e.call(TEXT_POINTER, &args![name]).u32() != 0 {
        e.call(SET_TEXT, &args![name, 0u32, 0u32]);
    }
}

// Translated from 0048e2e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048e2e0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// The destructor of the class `fn_0048e270` constructs (`TESTexture` by
/// its layout; the engine map gives it no name): stores the vtable
/// (`0101d7c4`) back, runs `00483710` on `this` (an empty function in the
/// exe) and destroys the string at +4 (`004037d0`). `ECX`, no stack words.
/// The exception-unwinding frame is not translated.
pub fn fn_0048e2e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0040_37d0, &args![this.byte_add(4)]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048e270, fn_0048e270(Ptr) -> Ptr),
        entry!(0x0048e340, fn_0048e340(Ptr)),
        entry!(0x0048_e2e0, fn_0048e2e0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(BASE_COMPONENT_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(SET_TEXT, |_, _| Ret::default());
        e.register(TEXT_POINTER, |e, a| e.mem.u32(a[0]).into_ret());
        e
    }

    #[test]
    fn clearing_sets_the_text_only_when_there_is_one() {
        let mut e = engine();
        let object = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0048_e340, &args![object]);
        let log = e.call_log.take().unwrap();
        assert!(
            log.iter().all(|c| c.0 != SET_TEXT),
            "no text: nothing to clear"
        );

        e.mem.set_u32(object + 4, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0048_e340, &args![object]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (TEXT_POINTER, vec![object + 4]));
        assert_eq!(log[2], (SET_TEXT, vec![object + 4, 0, 0]));
    }

    #[test]
    fn the_constructor_runs_base_vtable_string_and_clear_in_order() {
        let mut e = engine();
        let object = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let back = e.call(0x0048_e270, &args![object]).u32();
        assert_eq!(back, object);
        assert_eq!(e.mem.u32(object), VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (BASE_COMPONENT_CONSTRUCT, vec![object]),
                (STRING_CONSTRUCT, vec![object + 4]),
                (TEXT_POINTER, vec![object + 4]),
            ]
        );
    }
}

#[cfg(test)]
mod b022_tests {
    use super::*;

    #[test]
    fn the_destructor_restores_the_vtable_and_destroys_the_name() {
        let mut e = Engine::new();
        e.register(0x0048_3710, |_, _| Ret::default());
        e.register(0x0040_37d0, |_, _| Ret::default());
        let object = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        e.call(0x0048_e2e0, &args![object]);
        assert_eq!(e.mem.u32(object), VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(0x0048_3710, vec![object]), (0x0040_37d0, vec![object + 4])]
        );
    }
}
