//! `BSSystemUtilities/run_00af3470` (the engine map's run of unnamed functions from
//! `00af3470`), subsystem `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525,
//! translated (docs/ENGINE_CRATE.md). Three function-local static singletons (the
//! `_Immortalize` shape: a guard word whose bit 0 says constructed, the object in `.data`, its
//! destructor stub registered with `_atexit`) and the strings object's constructor and
//! destructor body.

#[allow(unused_imports)]
use crate::prelude::*;

/// The C runtime's `_atexit` (`00ec658f`, cdecl, one word).
const ATEXIT: u32 = 0x00ec_658f;
/// The cache drive utility: guard (`011f7724`), object (`011f72f8`), constructor
/// (`00af38d0`, `thiscall`) and `atexit` stub (`00fdc320`).
const CACHE_DRIVE_GUARD: u32 = 0x011f_7724;
const CACHE_DRIVE: u32 = 0x011f_72f8;
const CACHE_DRIVE_CONSTRUCT: u32 = 0x00af_38d0;
const CACHE_DRIVE_ATEXIT: u32 = 0x00fd_c320;
/// The game data utility: guard (`011f7f78`), object (`011f7728`), constructor (`00af3980`)
/// and `atexit` stub (`00fdc330`).
const GAME_DATA_GUARD: u32 = 0x011f_7f78;
const GAME_DATA: u32 = 0x011f_7728;
const GAME_DATA_CONSTRUCT: u32 = 0x00af_3980;
const GAME_DATA_ATEXIT: u32 = 0x00fd_c330;
/// The strings object: guard (`011f7fa0`), object (`011f7f7c`), its vtable (`010a42dc`) and
/// `atexit` stub (`00fdc340`).
const STRINGS_GUARD: u32 = 0x011f_7fa0;
const STRINGS: u32 = 0x011f_7f7c;
const STRINGS_VTABLE: u32 = 0x010a_42dc;
const STRINGS_ATEXIT: u32 = 0x00fd_c340;

// Translated from 00af3470 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af3470 - the SEH frame (FS:[0] handler 00f2309e, state 0 around the constructor call) is not translated.
/// The cache drive utility's singleton getter (no label; Ghidra folds it into
/// `_Immortalize<std::_Generic_error_category>`; no arguments): the first call sets bit 0 of
/// the guard word `011f7724`, constructs the object at `011f72f8` (`00af38d0`) and registers
/// its destructor stub `00fdc320` with `_atexit`. Answers `011f72f8`.
pub fn fn_00af3470(e: &mut Engine) -> u32 {
    let guard = e.global::<u32>(CACHE_DRIVE_GUARD);
    if guard & 1 == 0 {
        e.set_global::<u32>(CACHE_DRIVE_GUARD, guard | 1);
        e.call(CACHE_DRIVE_CONSTRUCT, &args![CACHE_DRIVE]);
        e.call(ATEXIT, &args![CACHE_DRIVE_ATEXIT]);
    }
    CACHE_DRIVE
}

// Translated from 00af34f0 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af34f0 - the SEH frame (FS:[0] handler 00f230ce, state 0 around the constructor call) is not translated.
/// The game data utility's singleton getter (no label; the same shape as [`fn_00af3470`]):
/// guard `011f7f78`, object `011f7728` constructed by `00af3980`, stub `00fdc330`. Answers
/// `011f7728`.
pub fn fn_00af34f0(e: &mut Engine) -> u32 {
    let guard = e.global::<u32>(GAME_DATA_GUARD);
    if guard & 1 == 0 {
        e.set_global::<u32>(GAME_DATA_GUARD, guard | 1);
        e.call(GAME_DATA_CONSTRUCT, &args![GAME_DATA]);
        e.call(ATEXIT, &args![GAME_DATA_ATEXIT]);
    }
    GAME_DATA
}

// Translated from 00af3570 (disassembled, FalloutNV.exe 1.4.0.525)
/// The strings object's constructor (no label; `thiscall`, plain `RET`): the vtable
/// `010a42dc`, then the eight string pointers (+4..+0x24) cleared in order. Answers `this`.
pub fn fn_00af3570(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, STRINGS_VTABLE);
    for i in 0..8u32 {
        e.mem.set_u32(this.wrapping_add(i * 4 + 4), 0);
    }
    this
}

// Translated from 00af35c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The strings object's destructor body (no label; `thiscall`, plain `RET`; the scalar
/// deleting destructor `00af37a0` calls it): stores the vtable `010a42dc` again and does
/// nothing else.
pub fn fn_00af35c0(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, STRINGS_VTABLE);
}

// Translated from 00af35e0 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af35e0 - the SEH frame (FS:[0] handler 00f230fe, state 0 around the constructor call) is not translated.
/// The strings object's singleton getter (no label; the same shape as [`fn_00af3470`]):
/// guard `011f7fa0`, object `011f7f7c` constructed by [`fn_00af3570`], stub `00fdc340`.
/// Answers `011f7f7c`.
pub fn fn_00af35e0(e: &mut Engine) -> u32 {
    let guard = e.global::<u32>(STRINGS_GUARD);
    if guard & 1 == 0 {
        e.set_global::<u32>(STRINGS_GUARD, guard | 1);
        fn_00af3570(e, STRINGS);
        e.call(ATEXIT, &args![STRINGS_ATEXIT]);
    }
    STRINGS
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_3470, fn_00af3470() -> u32),
        entry!(0x00af_34f0, fn_00af34f0() -> u32),
        entry!(0x00af_3570, fn_00af3570(u32) -> u32),
        entry!(0x00af_35c0, fn_00af35c0(u32)),
        entry!(0x00af_35e0, fn_00af35e0() -> u32),
    ]
}

#[cfg(test)]
mod tests {
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

    fn engine() -> Engine {
        let mut e = Engine::new();
        for (addr, len) in [
            (CACHE_DRIVE_GUARD, 4),
            (CACHE_DRIVE, 0x10),
            (GAME_DATA_GUARD, 4),
            (GAME_DATA, 0x850),
            (STRINGS_GUARD, 4),
            (STRINGS, 0x24),
        ] {
            e.map(addr, len);
        }
        e.register(ATEXIT, |_, _| 0u32.into_ret());
        e.register(CACHE_DRIVE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(GAME_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn each_getter_constructs_once() {
        let mut e = engine();
        for _ in 0..2 {
            assert_eq!(e.call(0x00af_3470, &args![]).u32(), CACHE_DRIVE);
            assert_eq!(e.call(0x00af_34f0, &args![]).u32(), GAME_DATA);
            assert_eq!(e.call(0x00af_35e0, &args![]).u32(), STRINGS);
        }
        assert_eq!(calls(&e, CACHE_DRIVE_CONSTRUCT), vec![vec![CACHE_DRIVE]]);
        assert_eq!(calls(&e, GAME_DATA_CONSTRUCT), vec![vec![GAME_DATA]]);
        assert_eq!(e.mem.u32(STRINGS), STRINGS_VTABLE);
        assert_eq!(
            calls(&e, ATEXIT),
            vec![
                vec![CACHE_DRIVE_ATEXIT],
                vec![GAME_DATA_ATEXIT],
                vec![STRINGS_ATEXIT]
            ]
        );
        // The other guard bits are kept.
        assert_eq!(e.global::<u32>(STRINGS_GUARD), 1);
        e.set_global::<u32>(CACHE_DRIVE_GUARD, 0x10);
        fn_00af3470(&mut e);
        assert_eq!(e.global::<u32>(CACHE_DRIVE_GUARD), 0x11);
    }

    #[test]
    fn the_strings_constructor_and_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x28);
        for offset in (0..0x28).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00af_3570, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), STRINGS_VTABLE);
        for offset in (4..0x24).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x24), 0xffff_ffff);
        e.mem.set_u32(this, 0);
        e.call(0x00af_35c0, &args![this]);
        assert_eq!(e.mem.u32(this), STRINGS_VTABLE);
    }
}
