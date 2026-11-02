//! `fallout/interface/run_007043c0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007043c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no words): answers false (`AL` = 0;
/// the rest of `EAX` is not set). Its callers pass words it does not read.
pub fn fn_007043c0(_e: &mut Engine) -> bool {
    false
}

/// The interface manager (`004b7210`) and its `009373f0` (`thiscall`, a `bool`), the
/// map menu singleton (`011da368`, by the map menu's own use) and `0079b430` (cdecl, one
/// word; `mapmenu.cpp`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const MANAGER_009373F0: u32 = 0x0093_73f0;
const MAP_MENU: u32 = 0x011d_a368;
const MAP_MENU_0079B430: u32 = 0x0079_b430;

/// Whether there is an interface manager and its `009373f0` answers true.
fn manager_ready(e: &mut Engine) -> bool {
    if e.call(INTERFACE_MANAGER, &args![]).u32() == 0 {
        return false;
    }
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    e.call(MANAGER_009373F0, &args![manager]).bool()
}

// Translated from 007043d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no words): with the interface manager ready (`009373f0`), `00704400`;
/// otherwise false.
pub fn fn_007043d0(e: &mut Engine) -> bool {
    if manager_ready(e) {
        fn_00704400(e)
    } else {
        false
    }
}

// Translated from 00704400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no words): the map menu's byte at +0x12c, or true without a map menu.
pub fn fn_00704400(e: &mut Engine) -> bool {
    let menu = e.global::<u32>(MAP_MENU);
    if menu != 0 {
        e.mem.u8(menu + 0x12c) != 0
    } else {
        true
    }
}

// Translated from 00704420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, a `bool`): with the interface manager ready, `00704450(value)`.
pub fn fn_00704420(e: &mut Engine, value: u8) {
    if manager_ready(e) {
        fn_00704450(e, value);
    }
}

// Translated from 00704450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, a `bool`): stores it in the map menu's byte at +0x12c, when there is a
/// map menu.
pub fn fn_00704450(e: &mut Engine, value: u8) {
    let menu = e.global::<u32>(MAP_MENU);
    if menu != 0 {
        e.mem.set_u8(menu + 0x12c, value);
    }
}

// Translated from 00704470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word): `0079b430(object)` for a non-zero object.
pub fn fn_00704470(e: &mut Engine, object: u32) {
    if object != 0 {
        e.call(MAP_MENU_0079B430, &args![object]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0070_43c0, fn_007043c0() -> bool),
        entry!(0x0070_43d0, fn_007043d0() -> bool),
        entry!(0x0070_4400, fn_00704400() -> bool),
        entry!(0x0070_4420, fn_00704420(u8)),
        entry!(0x0070_4450, fn_00704450(u8)),
        entry!(0x0070_4470, fn_00704470(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_007043c0_answers_false() {
        let mut e = Engine::new();
        assert!(!e.call(0x0070_43c0, &args![]).bool());
        assert!(!e.call(0x0070_43c0, &args![0x1234u32]).bool());
    }
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    fn engine(ready: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011d_a000, 0x1000);
        e.register(INTERFACE_MANAGER, |_, _| 0x5000u32.into_ret());
        if ready {
            e.register(MANAGER_009373F0, |_, a| {
                u32::from(a[0] == 0x5000).into_ret()
            });
        } else {
            e.register(MANAGER_009373F0, |_, _| Ret::default());
        }
        let menu = e.mem.alloc(0x130);
        (e, menu)
    }

    #[test]
    fn map_menu_flag_reads_and_writes() {
        let (mut e, menu) = engine(true);
        assert!(e.call(0x0070_4400, &args![]).bool());
        assert!(e.call(0x0070_43d0, &args![]).bool());
        e.set_global(MAP_MENU, menu);
        assert!(!e.call(0x0070_43d0, &args![]).bool());
        e.call(0x0070_4420, &args![1u32]);
        assert_eq!(e.mem.u8(menu + 0x12c), 1);
        assert!(e.call(0x0070_4400, &args![]).bool());
        e.call(0x0070_4450, &args![0u32]);
        assert_eq!(e.mem.u8(menu + 0x12c), 0);
        let (mut e, menu) = engine(false);
        e.set_global(MAP_MENU, menu);
        e.mem.set_u8(menu + 0x12c, 1);
        assert!(!e.call(0x0070_43d0, &args![]).bool());
        e.call(0x0070_4420, &args![0u32]);
        assert_eq!(e.mem.u8(menu + 0x12c), 1);
    }

    #[test]
    fn fn_00704470_forwards_non_zero_objects() {
        let mut e = Engine::new();
        e.register(MAP_MENU_0079B430, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0070_4470, &args![0u32]);
        e.call(0x0070_4470, &args![0x7000u32]);
        let calls: Vec<_> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == MAP_MENU_0079B430)
            .collect();
        assert_eq!(calls, vec![(MAP_MENU_0079B430, vec![0x7000])]);
    }
}
