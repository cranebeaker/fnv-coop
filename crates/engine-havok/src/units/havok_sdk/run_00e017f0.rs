//! `Havok SDK/run_00e017f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e01830 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x2c-byte frame stores the vtable word 0x010e_95b4 at
/// +0xC, runs the constructor at `00cb0a70` (argument 0) on the members at frame+0x14, 0x18, 0x1c in that
/// order and returns the vtable word. Nothing is destroyed.
pub fn fn_00e01830(e: &mut Engine) -> u32 {
    e.with_stack(0x2c, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base + 0xc, 0x010e_95b4);
        e.call(0x00cb_0a70, &args![base + 0x14, 0u32]);
        e.call(0x00cb_0a70, &args![base + 0x18, 0u32]);
        e.call(0x00cb_0a70, &args![base + 0x1c, 0u32]);
        e.mem.u32(base + 0xc)
    })
}

// Translated from 00e017f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010e95b4` and
/// runs `00cb0a70` (an `hkStringPtr` function, one stack word `extra`) on `object + 8`, then on
/// `object + 0xc` and on `object + 0x10`. The function takes two stack words and pops none
/// (`RET`), and returns nothing. The class is unconfirmed.
pub fn fn_00e017f0(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_95b4);
        e.call(0x00cb_0a70, &args![object.addr() + 0x8, extra]);
        e.call(0x00cb_0a70, &args![object.addr() + 0xc, extra]);
        e.call(0x00cb_0a70, &args![object.addr() + 0x10, extra]);
    }
}

// Translated from 00e017d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns what it returns (`EAX`). Ghidra made no function here; the address is an immediate operand at 00fc6699.
pub fn fn_00e017d0(e: &mut Engine, object: u32) -> u32 {
    e.vcall(object, 0, &args![0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e01830, fn_00e01830() -> u32),
        entry!(0x00e0_17f0, fn_00e017f0(Ptr, u32)),
        entry!(0x00e0_17d0, fn_00e017d0(u32) -> u32),
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
    fn fn_00e01830_stores_the_vtable_and_constructs_its_members() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x8, 0xc, 0x10], false);
        let result = e.call(0x00e0_1830, &args![]).u32();
        assert_eq!(result, 0x010e_95b4);
        let calls = log.borrow();
        assert_eq!(calls.len(), 3);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010e_95b4);
        }
        assert_eq!(calls[1].0 - calls[0].0, 0x4);
        assert_eq!(calls[2].0 - calls[0].0, 0x8);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A null object does nothing; otherwise the string function runs on the three members.
    #[test]
    fn fn_00e017f0_runs_the_string_function_three_times() {
        let mut e = Engine::new();
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let string_log = Rc::clone(&log);
        e.register_double(0x00cb_0a70, move |_, a| {
            string_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00e0_17f0, &args![0u32, 4u32]);
        assert!(log.borrow().is_empty());
        let object = e.mem.alloc(0x30);
        e.call(0x00e0_17f0, &args![object, 4u32]);
        assert_eq!(e.mem.u32(object), 0x010e_95b4);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![object + 8, 4],
                vec![object + 0xc, 4],
                vec![object + 0x10, 4]
            ]
        );
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The virtual function in slot 0 gets the object and the word 0; its answer comes back.
    #[test]
    fn fn_00e017d0_calls_slot_zero_with_zero() {
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
        assert_eq!(e.call(0x00e0_17d0, &args![object]).u32(), 0x77);
        assert_eq!(*seen.borrow(), vec![vec![object, 0]]);
    }
}
