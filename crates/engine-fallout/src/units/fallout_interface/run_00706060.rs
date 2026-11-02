//! `fallout/interface/run_00706060` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The terminal geometry holder (`011db63c`, an `NiPointer`, read through `00559450`)
/// and its fade (`006d2c40`, `thiscall`, `ST0`).
const TERMINAL_GEOMETRY_011DB63C: u32 = 0x011d_b63c;
const FIRST_WORD: u32 = 0x0055_9450;
const GEOMETRY_FADE_006D2C40: u32 = 0x006d_2c40;
/// `MemoryManager` free (`00401030`, cdecl), the `NiPointer` destructor (`0045cec0`) and
/// the `FORenderedMenu` destructor body (`007fad10`).
const FREE: u32 = 0x0040_1030;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
const RENDERED_MENU_DESTRUCT: u32 = 0x007f_ad10;

// Translated from 00706060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `this` not read; `ST0`): the terminal geometry's fade
/// (`006d2c40`) when there is terminal geometry, else 1.0; through a `float` local.
pub fn fn_00706060(e: &mut Engine, _this: Ptr) -> f32 {
    let fade = if e.call(FIRST_WORD, &args![TERMINAL_GEOMETRY_011DB63C]).u32() != 0 {
        let geometry = e.call(FIRST_WORD, &args![TERMINAL_GEOMETRY_011DB63C]).u32();
        let fade = e.call(GEOMETRY_FADE_006D2C40, &args![geometry]).f64();
        x87_store32(fade)
    } else {
        x87_store32(1.0)
    };
    f32::from_bits(x87_quiet_bits(fade))
}

// Translated from 007060a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedTerminal`'s scalar deleting destructor (its vtable `0106ebe4`, slot 0;
/// `thiscall`, `RET 4`): the destructor (`007060d0`), then the block freed when bit 0 of
/// `flags` is set. Answers `this`.
pub fn fn_007060a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_007060d0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 007060d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007060d0 - the C++ exception-unwinding frame (FS:[0] around the exit button pointer) is not translated.
/// The `FORenderedTerminal` destructor body (`thiscall`): destroys the exit button's
/// `NiPointer` (`spExitButton`, +0xe4), then the `FORenderedMenu` destructor (`007fad10`).
pub fn fn_007060d0(e: &mut Engine, this: Ptr) {
    e.call(NI_POINTER_DESTRUCT, &args![this.addr() + 0xe4]);
    e.call(RENDERED_MENU_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0070_6060, fn_00706060(Ptr) -> f32),
        entry!(0x0070_60a0, fn_007060a0(Ptr, u32) -> Ptr),
        entry!(0x0070_60d0, fn_007060d0(Ptr)),
    ]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn fade_is_the_geometry_s_or_one() {
        let mut e = Engine::new();
        e.map(0x011d_b000, 0x1000);
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(GEOMETRY_FADE_006D2C40, |_, a| Ret {
            st0: f64::from(a[0]) / 0x10000 as f64,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0070_6060, &args![0u32]).f32(), 1.0);
        e.set_global(TERMINAL_GEOMETRY_011DB63C, 0x8000u32);
        assert_eq!(e.call(0x0070_6060, &args![0u32]).f32(), 0.5);
    }

    #[test]
    fn destructors_release_the_button_and_the_menu() {
        let mut e = Engine::new();
        for address in [FREE, NI_POINTER_DESTRUCT, RENDERED_MENU_DESTRUCT] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0070_60a0, &args![0x5000u32, 1u32]).u32(), 0x5000);
        let log: Vec<_> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (NI_POINTER_DESTRUCT, vec![0x50e4]),
                (RENDERED_MENU_DESTRUCT, vec![0x5000]),
                (FREE, vec![0x5000]),
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x0070_60a0, &args![0x5000u32, 0u32]);
        assert!(!e.call_log.unwrap().iter().any(|(a, _)| *a == FREE));
    }
}
