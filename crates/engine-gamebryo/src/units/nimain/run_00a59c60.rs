//! `NiMain/run_00a59c60` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Virtual slot (byte offset) called on `this` with the argument and 0.
const FIRST_SLOT: u32 = 0xa4;
/// Virtual slot (byte offset) called with no argument on the object held at
/// `+0x18`.
const SECOND_SLOT: u32 = 0xfc;

// Translated from 00a59c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__thiscall`, one argument (`RET 4`): calls the virtual method at vtable
/// `+0xa4` of `this` with (`argument`, 0), then, when the pointer at `+0x18`
/// of `this` is not null, the virtual method at vtable `+0xfc` of that
/// object. The slot names are not known.
pub fn fn_00a59c60(e: &mut Engine, this: Ptr, argument: u32) {
    e.vcall(this.addr(), FIRST_SLOT, &args![argument, 0u32]);
    let child = e.mem.u32(this.addr() + 0x18);
    if child != 0 {
        e.vcall(child, SECOND_SLOT, &args![]);
    }
}

// Translated from 00a59c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__thiscall`, one argument (`RET 4`), the twin of `fn_00a59c60` with the slot +0xa8:
/// calls the virtual method at vtable `+0xa8` of `this` with (`argument`, 0), then, when the
/// pointer at `+0x18` of `this` is not null, the virtual method at vtable `+0xfc` of that
/// object. The slot names are not known.
pub fn fn_00a59c90(e: &mut Engine, this: Ptr, argument: u32) {
    e.vcall(this.addr(), 0xa8, &args![argument, 0u32]);
    let child = e.mem.u32(this.addr() + 0x18);
    if child != 0 {
        e.vcall(child, SECOND_SLOT, &args![]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a59c60, fn_00a59c60(Ptr, u32)),
        entry!(0x00a59c90, fn_00a59c90(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const THIS_VTABLE: u32 = 0x0300_0000;
    const CHILD_VTABLE: u32 = 0x0300_1000;
    const FIRST: u32 = 0x0070_0000;
    const SECOND: u32 = 0x0070_0010;

    /// An object whose vtable slots are the two doubles, with a child object
    /// when `with_child`.
    fn object(e: &mut Engine, with_child: bool) -> Ptr {
        let mut first_table = vec![0; 0xa4 / 4 + 1];
        first_table[0xa4 / 4] = FIRST;
        e.put_vtable(THIS_VTABLE, &first_table);
        let mut second_table = vec![0; 0xfc / 4 + 1];
        second_table[0xfc / 4] = SECOND;
        e.put_vtable(CHILD_VTABLE, &second_table);
        let this: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(this.addr(), THIS_VTABLE);
        if with_child {
            let child = e.mem.alloc(8);
            e.mem.set_u32(child, CHILD_VTABLE);
            e.mem.set_u32(this.addr() + 0x18, child);
        }
        this
    }

    fn quiet_doubles(e: &mut Engine) {
        e.register(FIRST, |_, _| Ret::default());
        e.register(SECOND, |_, _| Ret::default());
    }

    #[test]
    fn calls_the_first_slot_then_the_childs_slot() {
        let mut e = Engine::new();
        quiet_doubles(&mut e);
        let this = object(&mut e, true);
        let child = e.mem.u32(this.addr() + 0x18);
        e.call_log = Some(vec![]);
        e.call(0x00a5_9c60, &args![this, 0x55u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (FIRST, vec![this.addr(), 0x55, 0]));
        assert_eq!(log[2], (SECOND, vec![child]));
    }

    #[test]
    fn without_a_child_only_the_first_slot_is_called() {
        let mut e = Engine::new();
        quiet_doubles(&mut e);
        let this = object(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x00a5_9c60, &args![this, 0x55u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn fn_00a59c90_calls_slot_a8_then_the_childs_slot() {
        let mut e = Engine::new();
        quiet_doubles(&mut e);
        e.register(0x0070_0020, |_, _| Ret::default());
        let this = object(&mut e, true);
        let mut table = vec![0; 0xa8 / 4 + 1];
        table[0xa8 / 4] = 0x0070_0020;
        e.put_vtable(0x0300_2000, &table);
        e.mem.set_u32(this.addr(), 0x0300_2000);
        let child = e.mem.u32(this.addr() + 0x18);
        e.call_log = Some(vec![]);
        e.call(0x00a5_9c90, &args![this, 0x66u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|c| c.0 == 0x0070_0020 || c.0 == SECOND)
            .cloned()
            .collect();
        assert_eq!(
            hits,
            vec![
                (0x0070_0020, vec![this.addr(), 0x66, 0]),
                (SECOND, vec![child])
            ]
        );
        e.mem.set_u32(this.addr() + 0x18, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_9c90, &args![this, 0x66u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
