//! `fallout shared/run_004ee920` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The node's scalar deleting destructor (`004ee360`; `thiscall`: flags).
const NODE_DELETING_DESTRUCTOR: u32 = 0x004e_e360;
/// The `NiPointer` temporary's constructor (`00633c90`; `thiscall`: object) and destructor
/// (`0045cec0`; `thiscall`).
const POINTER_CONSTRUCTOR: u32 = 0x0063_3c90;
const POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
/// The assignment `004ee920` ends with (`006e5cc0`; `thiscall` on `this`: the temporary's
/// address as the constructor answers it).
const POINTER_STORE: u32 = 0x006e_5cc0;
/// Callees of the constructor `004ee9f0`, by address.
const BASE_PREPARE: u32 = 0x0041_3f40;
const BASE_CONSTRUCTOR: u32 = 0x00a3_2a10;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
const SOURCE_FIRST: u32 = 0x0070_0300;
const SOURCE_SECOND: u32 = 0x005f_5f80;
const SOURCE_THIRD: u32 = 0x006c_9900;
const SCOPE_CONSTRUCTOR: u32 = 0x004a_d0c0;
const SCOPE_DESTRUCTOR: u32 = 0x004a_d1d0;
const SOURCE_REGISTER: u32 = 0x00a3_43c0;
/// The member assignment `004eeae0` forwards to (`0043ba10`; `thiscall` on `this + 0x5C`).
const MEMBER_SET: u32 = 0x0043_ba10;
/// The virtual table the constructor stores (`0101de3c`).
const CONSTRUCTED_VTABLE: u32 = 0x0101_de3c;

// Translated from 004ee920 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ee920 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// Unnamed in the engine map (`thiscall`, plain `RET`): deletes the singly linked chain
/// that starts at `this + 4` (each node unlinked by clearing its next word at `+4`, then
/// its scalar deleting destructor `004ee360` with flags 1), then constructs a null
/// `NiPointer` temporary (`00633c90`), passes it to `006e5cc0` on `this` and destroys it
/// (`0045cec0`).
pub fn fn_004ee920(e: &mut Engine, this: Ptr) {
    let head = this.addr() + 4;
    while e.mem.u32(head) != 0 {
        let node = e.mem.u32(head);
        let next = e.mem.u32(node + 4);
        e.mem.set_u32(node + 4, 0);
        let node = e.mem.u32(head);
        if node != 0 {
            e.call(NODE_DELETING_DESTRUCTOR, &args![node, 1u32]);
        }
        e.mem.set_u32(head, next);
    }
    e.with_stack(4, |e, temporary| {
        let built = e.call(POINTER_CONSTRUCTOR, &args![temporary, 0u32]).u32();
        e.call(POINTER_STORE, &args![this, built]);
        e.call(POINTER_DESTRUCTOR, &args![temporary]);
    });
}

// Translated from 004eeae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): passes `value` to `0043ba10` on the
/// member at `this + 0x5C`.
pub fn fn_004eeae0(e: &mut Engine, this: Ptr, value: u32) {
    e.call(MEMBER_SET, &args![this.addr() + 0x5c, value]);
}

// Translated from 004ee9f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ee9f0 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// A constructor (unnamed in the engine map; `thiscall`, `RET 8`: `object`, `source`). The
/// base constructor (`00a32a10`) with what `00413f40` answers for `source` (`source + 8`)
/// and the words 0 and 1, the virtual table `0101de3c`, the pointer at `+0x74` constructed null
/// (`00633c90`) and assigned `object` (`0066b0d0`), the pointer at `+0x20` assigned what
/// `00700300` answers for `source`, the word at `+0x60` set to `005f5f80`'s answer,
/// `006c9900`'s answer passed to [`fn_004eeae0`]; then a 0x10-byte scope (`004ad0c0` with
/// 0x101 .. `004ad1d0`) around `00a343c0` on `source` (with `this` and the scope). Answers
/// `this`.
pub fn fn_004ee9f0(e: &mut Engine, this: Ptr, object: u32, source: u32) -> Ptr {
    let prepared = e.call(BASE_PREPARE, &args![source]).u32();
    e.call(BASE_CONSTRUCTOR, &args![this, prepared, 0u32, 1u32]);
    e.mem.set_u32(this.addr(), CONSTRUCTED_VTABLE);
    e.call(POINTER_CONSTRUCTOR, &args![this.addr() + 0x74, 0u32]);
    e.call(POINTER_ASSIGN, &args![this.addr() + 0x74, object]);
    let first = e.call(SOURCE_FIRST, &args![source]).u32();
    e.call(POINTER_ASSIGN, &args![this.addr() + 0x20, first]);
    let second = e.call(SOURCE_SECOND, &args![source]).u32();
    e.mem.set_u32(this.addr() + 0x60, second);
    let third = e.call(SOURCE_THIRD, &args![source]).u32();
    fn_004eeae0(e, this, third);
    e.with_stack(0x10, |e, scope| {
        e.call(SCOPE_CONSTRUCTOR, &args![scope, 0x101u32]);
        e.call(SOURCE_REGISTER, &args![source, this, scope]);
        e.call(SCOPE_DESTRUCTOR, &args![scope]);
    });
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004e_e920, fn_004ee920(Ptr)),
        entry!(0x004e_eae0, fn_004eeae0(Ptr, u32)),
        entry!(0x004e_e9f0, fn_004ee9f0(Ptr, u32, u32) -> Ptr),
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
    fn deletes_the_chain_then_stores_a_null_pointer() {
        let mut e = Engine::new();
        e.register(NODE_DELETING_DESTRUCTOR, |_, _| Ret::default());
        e.register(POINTER_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(POINTER_STORE, |_, _| Ret::default());
        e.register(POINTER_DESTRUCTOR, |_, _| Ret::default());
        let second = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first + 4, second);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, first);
        e.call_log = Some(vec![]);
        e.call(0x004e_e920, &args![this]);
        assert_eq!(
            calls(&e, NODE_DELETING_DESTRUCTOR),
            vec![vec![first, 1], vec![second, 1]]
        );
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(first + 4), 0);
        let temporary = calls(&e, POINTER_CONSTRUCTOR)[0][0];
        assert_eq!(calls(&e, POINTER_STORE), vec![vec![this, temporary]]);
        assert_eq!(calls(&e, POINTER_DESTRUCTOR), vec![vec![temporary]]);
    }

    #[test]
    fn member_set_targets_the_member() {
        let mut e = Engine::new();
        e.register(MEMBER_SET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004e_eae0, &args![0x1000u32, 7u32]);
        assert_eq!(calls(&e, MEMBER_SET), vec![vec![0x105c, 7]]);
    }

    #[test]
    fn constructor_wires_the_members_in_order() {
        let mut e = Engine::new();
        e.register(BASE_PREPARE, |_, _| 0x51u32.into_ret());
        e.register(SOURCE_FIRST, |_, _| 0x52u32.into_ret());
        e.register(SOURCE_SECOND, |_, _| 0x53u32.into_ret());
        e.register(SOURCE_THIRD, |_, _| 0x54u32.into_ret());
        for address in [
            BASE_CONSTRUCTOR,
            POINTER_CONSTRUCTOR,
            POINTER_ASSIGN,
            SCOPE_CONSTRUCTOR,
            SCOPE_DESTRUCTOR,
            SOURCE_REGISTER,
            MEMBER_SET,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x004e_e9f0, &args![this, 0x77u32, 0x88u32]).u32(),
            this
        );
        assert_eq!(calls(&e, BASE_PREPARE), vec![vec![0x88]]);
        assert_eq!(calls(&e, BASE_CONSTRUCTOR), vec![vec![this, 0x51, 0, 1]]);
        assert_eq!(e.mem.u32(this), CONSTRUCTED_VTABLE);
        assert_eq!(
            calls(&e, POINTER_ASSIGN),
            vec![vec![this + 0x74, 0x77], vec![this + 0x20, 0x52]]
        );
        assert_eq!(e.mem.u32(this + 0x60), 0x53);
        assert_eq!(calls(&e, MEMBER_SET), vec![vec![this + 0x5c, 0x54]]);
        let scope = calls(&e, SCOPE_CONSTRUCTOR)[0].clone();
        assert_eq!(scope[1], 0x101);
        assert_eq!(calls(&e, SOURCE_REGISTER), vec![vec![0x88, this, scope[0]]]);
        assert_eq!(calls(&e, SCOPE_DESTRUCTOR), vec![vec![scope[0]]]);
    }
}
