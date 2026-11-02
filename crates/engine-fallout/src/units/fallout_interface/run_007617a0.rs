//! `fallout/interface/run_007617a0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Menu` constructor `00a1c4a0` (`thiscall`).
const MENU_CONSTRUCTOR: u32 = 0x00a1_c4a0;
/// `DialogMenu`'s vtable (`0107257c`).
const DIALOG_MENU_VTABLE: u32 = 0x0107_257c;
/// Member constructors (`thiscall`): `007642f0` (`dialogmenu.cpp`) at +0x40, `0041a250`
/// at +0x74, and `006815c0` (which only answers its `this`) on +0x110, +0x118, +0x120.
const MEMBER_CONSTRUCTOR_007642F0: u32 = 0x0076_42f0;
const MEMBER_CONSTRUCTOR_0041A250: u32 = 0x0041_a250;
const MEMBER_006815C0: u32 = 0x0068_15c0;
/// A global byte the constructor sets (`011d9514`).
const FLAG_011D9514: u32 = 0x011d_9514;
/// The `MenuTopicManager` singleton getter (`0083e590`).
const MENU_TOPIC_MANAGER: u32 = 0x0083_e590;
/// `100.0` (`float`, `01016410`) and the global float it is copied to (`011a008c`).
const HUNDRED: u32 = 0x0101_6410;
const HUNDRED_COPY_011A008C: u32 = 0x011a_008c;

// Translated from 007617a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `DialogMenu` constructor (unnamed in the engine map; `thiscall`): constructs the
/// `Menu` base, stores the vtable, constructs the members at +0x40 and +0x74 (and runs
/// `006815c0` on +0x110, +0x118, +0x120), clears its words +0x28, +0x2c, +0x80, +0x10c,
/// +0x12c, its bytes +0x84, +0x104, +0x130, +0x131, +0x138, +0x139, +0x13a and the float
/// +0x108, keeps the `MenuTopicManager` singleton at +0x70, sets the byte at `011d9514`
/// and copies 100.0 (`01016410`) to the float at `011a008c`. Returns `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 007617a0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_007617a0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MENU_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, DIALOG_MENU_VTABLE);
    e.call(MEMBER_CONSTRUCTOR_007642F0, &args![base + 0x40]);
    e.call(MEMBER_CONSTRUCTOR_0041A250, &args![base + 0x74]);
    for offset in [0x110u32, 0x118, 0x120] {
        e.call(MEMBER_006815C0, &args![base + offset]);
    }
    e.mem.set_u32(base + 0x80, 0);
    e.mem.set_u8(FLAG_011D9514, 1);
    e.mem.set_u8(base + 0x130, 0);
    e.mem.set_u8(base + 0x131, 0);
    e.mem.set_u8(base + 0x138, 0);
    e.mem.set_u32(base + 0x108, 0);
    e.mem.set_u32(base + 0x28, 0);
    e.mem.set_u32(base + 0x2c, 0);
    let manager = e.call(MENU_TOPIC_MANAGER, &args![]).u32();
    e.mem.set_u32(base + 0x70, manager);
    e.mem.set_u8(base + 0x84, 0);
    e.mem.set_u8(base + 0x104, 0);
    e.mem.set_u32(base + 0x12c, 0);
    let hundred = x87_quiet_bits(e.mem.u32(HUNDRED));
    e.mem.set_u32(HUNDRED_COPY_011A008C, hundred);
    e.mem.set_u8(base + 0x139, 0);
    e.mem.set_u8(base + 0x13a, 0);
    e.mem.set_u32(base + 0x10c, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0076_17a0, fn_007617a0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_the_dialog_menu() {
        let mut e = Engine::new();
        for page in [0x0101_6000, 0x011a_0000, 0x011d_9000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(HUNDRED, 100.0);
        for address in [
            MENU_CONSTRUCTOR,
            MEMBER_CONSTRUCTOR_007642F0,
            MEMBER_CONSTRUCTOR_0041A250,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(MEMBER_006815C0, |_, a| a[0].into_ret());
        e.register(MENU_TOPIC_MANAGER, |_, _| 0x7070u32.into_ret());
        let this = e.mem.alloc(0x140);
        e.mem.write(this, &[0xee; 0x140]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_17a0, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), DIALOG_MENU_VTABLE);
        assert_eq!(e.mem.u32(this + 0x70), 0x7070);
        for offset in [0x28u32, 0x2c, 0x80, 0x108, 0x10c, 0x12c] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:x}");
        }
        for offset in [0x84u32, 0x104, 0x130, 0x131, 0x138, 0x139, 0x13a] {
            assert_eq!(e.mem.u8(this + offset), 0, "{offset:x}");
        }
        assert_eq!(e.mem.u8(this + 0x132), 0xee);
        assert_eq!(e.mem.u8(FLAG_011D9514), 1);
        assert_eq!(e.mem.f32(HUNDRED_COPY_011A008C), 100.0);
        let log = e.call_log.take().unwrap();
        let members: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == MEMBER_006815C0)
            .map(|(_, w)| w[0] - this)
            .collect();
        assert_eq!(members, vec![0x110, 0x118, 0x120]);
    }
}
