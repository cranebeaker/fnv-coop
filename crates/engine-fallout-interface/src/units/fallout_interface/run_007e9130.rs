//! `fallout/interface/run_007e9130` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `VATSMenu`'s vtable (`010700d4`, RTTI `.?AVVATSMenu@@`), the fader manager
/// (`011d8804`) and `FaderManager::RemoveFader` (`007010e0`, `RET 8`), `007f6920`
/// (`vatsmenu.cpp`, no words), the static `011db0d4` whose +4 (`00726070`) holds an
/// object deleted through its slot 0, the members' destructors at +0x11c (`009ca4c0`)
/// and +0xb0 (`00723820`), and the `Menu` destructor body (`00a1c520`).
const VATS_MENU_VTABLE: u32 = 0x0107_00d4;
const FADER_MANAGER: u32 = 0x011d_8804;
const REMOVE_FADER: u32 = 0x0070_10e0;
const VATS_007F6920: u32 = 0x007f_6920;
const STATIC_011DB0D4: u32 = 0x011d_b0d4;
const SECOND_WORD_00726070: u32 = 0x0072_6070;
const MEMBER_DESTRUCT_009CA4C0: u32 = 0x009c_a4c0;
const MEMBER_DESTRUCT_00723820: u32 = 0x0072_3820;
const MENU_DESTRUCT: u32 = 0x00a1_c520;

// Translated from 007e9130 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e9130 - the C++ exception-unwinding frame (FS:[0] around the members) is not translated.
/// The `VATSMenu` destructor body (unnamed in the engine map; `thiscall`): the vtable;
/// removes the fader (`RemoveFader(0, 0)`), `007f6920`; deletes the object at +4 of the
/// static `011db0d4` (slot 0 with 1) when there is one and clears the static; destroys
/// the members at +0x11c and +0xb0 and runs the `Menu` destructor.
pub fn fn_007e9130(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VATS_MENU_VTABLE);
    let fader = e.global::<u32>(FADER_MANAGER);
    e.call(REMOVE_FADER, &args![fader, 0u32, 0u32]);
    e.call(VATS_007F6920, &args![]);
    let holder = e.global::<u32>(STATIC_011DB0D4);
    let object = e.call(SECOND_WORD_00726070, &args![holder]).u32();
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
    e.set_global(STATIC_011DB0D4, 0u32);
    e.call(MEMBER_DESTRUCT_009CA4C0, &args![base + 0x11c]);
    e.call(MEMBER_DESTRUCT_00723820, &args![base + 0xb0]);
    e.call(MENU_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007e_9130, fn_007e9130(Ptr))]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn vats_menu_destructor() {
        let mut e = Engine::new();
        e.map(0x011d_8000, 0x1000);
        e.map(0x011d_b000, 0x1000);
        for address in [
            REMOVE_FADER,
            VATS_007F6920,
            MEMBER_DESTRUCT_009CA4C0,
            MEMBER_DESTRUCT_00723820,
            MENU_DESTRUCT,
            0x0e00_0000,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(SECOND_WORD_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.put_vtable(0x0f00_0000, &[0x0e00_0000]);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0f00_0000);
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder + 4, object);
        e.set_global(STATIC_011DB0D4, holder);
        e.set_global(FADER_MANAGER, 0x8800u32);
        let this = e.mem.alloc(0x160);
        e.call_log = Some(vec![]);
        e.call(0x007e_9130, &args![this]);
        assert_eq!(e.mem.u32(this), VATS_MENU_VTABLE);
        assert_eq!(e.global::<u32>(STATIC_011DB0D4), 0);
        let log: Vec<_> = e.call_log.unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (REMOVE_FADER, vec![0x8800, 0, 0]),
                (VATS_007F6920, vec![]),
                (SECOND_WORD_00726070, vec![holder]),
                (0x0e00_0000, vec![object, 1]),
                (MEMBER_DESTRUCT_009CA4C0, vec![this + 0x11c]),
                (MEMBER_DESTRUCT_00723820, vec![this + 0xb0]),
                (MENU_DESTRUCT, vec![this]),
            ]
        );
    }
}
