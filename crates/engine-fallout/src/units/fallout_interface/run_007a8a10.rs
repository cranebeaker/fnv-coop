//! `fallout/interface/run_007a8a10` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00a1c4a0` (`this` only).
const BASE_CONSTRUCT: u32 = 0x00a1_c4a0;
/// `MessageMenu`'s vtable (RTTI `.?AVMessageMenu@@`).
const MESSAGE_MENU_VTABLE: u32 = 0x0107_566c;
/// The member constructors run at +0x40 (`007642f0`) and +0x70
/// (`0076bda0`), `this` only.
const MEMBER_AT_40_CONSTRUCT: u32 = 0x0076_42f0;
const MEMBER_AT_70_CONSTRUCT: u32 = 0x0076_bda0;
/// The function `004b7210` (no arguments) whose result's byte at +0xE4
/// the constructor sets to 0xFF.
const GLOBAL_OBJECT: u32 = 0x004b_7210;

// Translated from 007a8a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageMenu`'s constructor: the base constructor, the vtable
/// (`0107566c`), the members at +0x40 and +0x70; then the byte at +0xE4 of
/// the object `004b7210` returns becomes 0xFF and the word at +0x90 of the
/// menu 0. Returns `this`.
// Translation incomplete: 007a8a10 - SEH exception frame (FS:[0]) that destroys the constructed base and members on unwinding is not modelled.
pub fn fn_007a8a10(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), MESSAGE_MENU_VTABLE);
    e.call(
        MEMBER_AT_40_CONSTRUCT,
        &args![this.addr().wrapping_add(0x40)],
    );
    e.call(
        MEMBER_AT_70_CONSTRUCT,
        &args![this.addr().wrapping_add(0x70)],
    );
    let object = e.call(GLOBAL_OBJECT, &[]).u32();
    e.mem.set_u8(object.wrapping_add(0xe4), 0xff);
    e.mem.set_u32(this.addr().wrapping_add(0x90), 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007a_8a10, fn_007a8a10(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_in_order() {
        let mut e = Engine::new();
        let global = e.mem.alloc(0x100);
        e.register(BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        e.register(MEMBER_AT_40_CONSTRUCT, |_, _| Ret::default());
        e.register(MEMBER_AT_70_CONSTRUCT, |_, _| Ret::default());
        e.register_double(GLOBAL_OBJECT, move |_, _| Ret {
            eax: global,
            ..Ret::default()
        });
        let menu = e.mem.alloc(0x94);
        e.mem.set_u32(menu + 0x90, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007a_8a10, &args![menu]).u32(), menu);
        assert_eq!(e.mem.u32(menu), MESSAGE_MENU_VTABLE);
        assert_eq!(e.mem.u32(menu + 0x90), 0);
        assert_eq!(e.mem.u8(global + 0xe4), 0xff);
        let order: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap();
        assert_eq!(
            order,
            vec![
                (0x007a_8a10, vec![menu]),
                (BASE_CONSTRUCT, vec![menu]),
                (MEMBER_AT_40_CONSTRUCT, vec![menu + 0x40]),
                (MEMBER_AT_70_CONSTRUCT, vec![menu + 0x70]),
                (GLOBAL_OBJECT, vec![]),
            ]
        );
    }
}
