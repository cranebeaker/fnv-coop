//! `BSSystemUtilities/run_00af37d0` (the engine map's run of unnamed functions from
//! `00af37d0`), subsystem `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525,
//! translated (docs/ENGINE_CRATE.md). The disc boot and system info utilities' singleton
//! getters (the `_Immortalize` shape of `run_00af3470`) and the cache drive utility's
//! constructor and destructor body.

#[allow(unused_imports)]
use crate::prelude::*;

/// The C runtime's `_atexit` (`00ec658f`, cdecl, one word).
const ATEXIT: u32 = 0x00ec_658f;
/// The disc boot utility: guard (`011f7ff8`), object (`011f7fa8`), constructor (`00af3a40`,
/// `thiscall`) and `atexit` stub (`00fdc350`).
const DISC_BOOT_GUARD: u32 = 0x011f_7ff8;
const DISC_BOOT: u32 = 0x011f_7fa8;
const DISC_BOOT_CONSTRUCT: u32 = 0x00af_3a40;
const DISC_BOOT_ATEXIT: u32 = 0x00fd_c350;
/// The system info utility: guard (`011f8048`), object (`011f8000`), constructor
/// (`00af3b20`) and `atexit` stub (`00fdc360`).
const SYS_INFO_GUARD: u32 = 0x011f_8048;
const SYS_INFO: u32 = 0x011f_8000;
const SYS_INFO_CONSTRUCT: u32 = 0x00af_3b20;
const SYS_INFO_ATEXIT: u32 = 0x00fd_c360;
/// The cache drive utility's vtable (`010a42ec`).
const CACHE_DRIVE_VTABLE: u32 = 0x010a_42ec;

// Translated from 00af37d0 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af37d0 - the SEH frame (FS:[0] handler 00f2312e, state 0 around the constructor call) is not translated.
/// The disc boot utility's singleton getter (no label; Ghidra folds it into
/// `_Immortalize<std::_Generic_error_category>`; no arguments): the first call sets bit 0 of
/// the guard word `011f7ff8`, constructs the object at `011f7fa8` (`00af3a40`) and registers
/// its destructor stub `00fdc350` with `_atexit`. Answers `011f7fa8`.
pub fn fn_00af37d0(e: &mut Engine) -> u32 {
    let guard = e.global::<u32>(DISC_BOOT_GUARD);
    if guard & 1 == 0 {
        e.set_global::<u32>(DISC_BOOT_GUARD, guard | 1);
        e.call(DISC_BOOT_CONSTRUCT, &args![DISC_BOOT]);
        e.call(ATEXIT, &args![DISC_BOOT_ATEXIT]);
    }
    DISC_BOOT
}

// Translated from 00af3850 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af3850 - the SEH frame (FS:[0] handler 00f2315e, state 0 around the constructor call) is not translated.
/// The system info utility's singleton getter (no label; the same shape as
/// [`fn_00af37d0`]): guard `011f8048`, object `011f8000` constructed by `00af3b20`, stub
/// `00fdc360`. Answers `011f8000`.
pub fn fn_00af3850(e: &mut Engine) -> u32 {
    let guard = e.global::<u32>(SYS_INFO_GUARD);
    if guard & 1 == 0 {
        e.set_global::<u32>(SYS_INFO_GUARD, guard | 1);
        e.call(SYS_INFO_CONSTRUCT, &args![SYS_INFO]);
        e.call(ATEXIT, &args![SYS_INFO_ATEXIT]);
    }
    SYS_INFO
}

// Translated from 00af38d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The cache drive utility's constructor (no label; `thiscall`, plain `RET`): the vtable
/// `010a42ec`, the byte at +4 and the word at +8 cleared. Answers `this`.
pub fn fn_00af38d0(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, CACHE_DRIVE_VTABLE);
    e.mem.set_u8(this.wrapping_add(4), 0);
    e.mem.set_u32(this.wrapping_add(8), 0);
    this
}

// Translated from 00af3900 (disassembled, FalloutNV.exe 1.4.0.525)
/// The cache drive utility's destructor body (no label; `thiscall`, plain `RET`; the scalar
/// deleting destructor `00af3950` calls it): stores the vtable `010a42ec` again and does
/// nothing else.
pub fn fn_00af3900(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, CACHE_DRIVE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_37d0, fn_00af37d0() -> u32),
        entry!(0x00af_3850, fn_00af3850() -> u32),
        entry!(0x00af_38d0, fn_00af38d0(u32) -> u32),
        entry!(0x00af_3900, fn_00af3900(u32)),
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

    #[test]
    fn each_getter_constructs_once() {
        let mut e = Engine::new();
        for (addr, len) in [
            (DISC_BOOT_GUARD, 4),
            (DISC_BOOT, 0x50),
            (SYS_INFO_GUARD, 4),
            (SYS_INFO, 8),
        ] {
            e.map(addr, len);
        }
        e.register(ATEXIT, |_, _| 0u32.into_ret());
        e.register(DISC_BOOT_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SYS_INFO_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        for _ in 0..2 {
            assert_eq!(e.call(0x00af_37d0, &args![]).u32(), DISC_BOOT);
            assert_eq!(e.call(0x00af_3850, &args![]).u32(), SYS_INFO);
        }
        assert_eq!(calls(&e, DISC_BOOT_CONSTRUCT), vec![vec![DISC_BOOT]]);
        assert_eq!(calls(&e, SYS_INFO_CONSTRUCT), vec![vec![SYS_INFO]]);
        assert_eq!(
            calls(&e, ATEXIT),
            vec![vec![DISC_BOOT_ATEXIT], vec![SYS_INFO_ATEXIT]]
        );
        assert_eq!(e.global::<u32>(DISC_BOOT_GUARD), 1);
        assert_eq!(e.global::<u32>(SYS_INFO_GUARD), 1);
    }

    #[test]
    fn the_cache_drive_constructor_and_destructor() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for offset in (0..0x10).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00af_38d0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), CACHE_DRIVE_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0xffff_ff00);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0xffff_ffff);
        e.mem.set_u32(this, 0);
        e.call(0x00af_3900, &args![this]);
        assert_eq!(e.mem.u32(this), CACHE_DRIVE_VTABLE);
    }
}
