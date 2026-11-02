//! `Havok SDK/run_00dfbdf0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfbe20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0xe0-byte frame stores the vtable word 0x010c_c2fc at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x14 (argument 0) and
/// returns the constant 0x010d_f34c (the stored word is not read back). Nothing is destroyed.
pub fn fn_00dfbe20(e: &mut Engine) -> u32 {
    e.with_stack(0xe0, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![base + 0x14, 0u32]);
        0x010d_f34c
    })
}

// Translated from 00dfbdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010cc2fc`,
/// runs `00cb0a70` (an `hkStringPtr` function, `this` = `object + 0x14`, one stack word `extra`)
/// and writes the vtable word `010df34c`. The function takes two stack words and pops none
/// (`RET`), and returns nothing. The class is unconfirmed.
pub fn fn_00dfbdf0(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![object.addr() + 0x14, extra]);
        e.mem.set_u32(object.addr(), 0x010d_f34c);
    }
}

// Translated from 00dfbdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns what it returns (`EAX`). Ghidra made no function here; the address is an immediate operand at 00fc1a69.
pub fn fn_00dfbdd0(e: &mut Engine, object: u32) -> u32 {
    e.vcall(object, 0, &args![0u32]).u32()
}
// Translated from 00dfbdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011ba504.
pub fn fn_00dfbdb0(_e: &mut Engine) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfbe20, fn_00dfbe20() -> u32),
        entry!(0x00df_bdf0, fn_00dfbdf0(Ptr, u32)),
        entry!(0x00df_bdd0, fn_00dfbdd0(u32) -> u32),
        entry!(0x00df_bdb0, fn_00dfbdb0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What the probe saw on one call: `this`, the argument, the word `behind` bytes before `this`
    /// (the frame's vtable slot) and, when asked for, the `u32` at +0x20, `u16` at +0x24 and `u16`
    /// at +0x26 of the frame.
    type ProbeCall = (u32, u32, u32, [u32; 3]);
    type ProbeLog = Rc<RefCell<Vec<ProbeCall>>>;

    /// Double for the member constructor: logs, per call, what `ProbeCall` describes, with the
    /// vtable slot `behind[call]` bytes before `this`.
    fn frame_probe(e: &mut Engine, address: u32, behind: &[u32], with_tail: bool) -> ProbeLog {
        let log: ProbeLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        let behind = behind.to_vec();
        e.register_double(address, move |e, a| {
            let call = seen.borrow().len();
            let slot = a[0] - behind[call];
            let mut tail = [0; 3];
            if with_tail {
                tail = [
                    e.mem.u32(slot + 0x20),
                    u32::from(e.mem.u16(slot + 0x24)),
                    u32::from(e.mem.u16(slot + 0x26)),
                ];
            }
            seen.borrow_mut().push((a[0], a[1], e.mem.u32(slot), tail));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn fn_00dfbe20_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x14], false);
        let result = e.call(0x00df_be20, &args![]).u32();
        assert_eq!(result, 0x010d_f34c);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_c2fc);
        }
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// A null object does nothing.
    #[test]
    fn fn_00dfbdf0_null_object_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00df_bdf0, &args![0u32, 5u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    /// The vtable word ends at the second value, and the string function runs in between on
    /// `object + 0x14` with the extra word, seeing the first vtable value.
    #[test]
    fn fn_00dfbdf0_runs_the_string_function_between_the_vtable_writes() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.register_double(0x00cb_0a70, move |e, a| {
            assert_eq!(a, [object + 0x14, 7]);
            assert_eq!(e.mem.u32(object), 0x010c_c2fc);
            Ret::default()
        });
        e.call(0x00df_bdf0, &args![object, 7u32]);
        assert_eq!(e.mem.u32(object), 0x010d_f34c);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The virtual function in slot 0 gets the object and the word 0; its answer comes back.
    #[test]
    fn fn_00dfbdd0_calls_slot_zero_with_zero() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable, 0x0900_0002);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x0900_0002, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: 0x77,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x00df_bdd0, &args![object]).u32(), 0x77);
        assert_eq!(*seen.borrow(), vec![vec![object, 0]]);
    }
    #[test]
    fn fn_00dfbdb0_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00df_bdb0, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
