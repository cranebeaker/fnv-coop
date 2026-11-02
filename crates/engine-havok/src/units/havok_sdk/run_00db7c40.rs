//! `Havok SDK/run_00db7c40` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00db7c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x50-byte frame stores the vtable word 0x010c_ce28 at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x38 (argument 0) and
/// returns the word at the start of the frame. Nothing is destroyed.
pub fn fn_00db7c40(e: &mut Engine) -> u32 {
    e.with_stack(0x50, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010c_ce28);
        e.call(0x00cb_0a70, &args![base + 0x38, 0u32]);
        e.mem.u32(base)
    })
}

// Translated from 00db7c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Thiscall (`RET 8`: two stack words, the first never read:
/// `_unused_0`; `this` is not read either, its one caller `00db7c90` passes the uid cache
/// object). Answers the second stack word.
pub fn fn_00db7c70(_e: &mut Engine, _this: Ptr, _unused_0: u32, value: u32) -> u32 {
    value
}

// Translated from 00db7c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word `object`): calls the object's virtual function at
/// vtable offset 0 (the destructor) with the one stack word 0 and answers what it answers. It is
/// the cleanup function of a type-info record (word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject`
/// calls it with `(object)`): the start-up initializer at `00fbff59` stores its address there.
pub fn fn_00db7c00(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00db7c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words `(object, extra)`): with a non-null `object`
/// stores the vtable word 0x010c_ce28 in its first word and runs the constructor at `00cb0a70` on the
/// member at `object + 0x38` with `extra` (`thiscall`, one stack word). Nothing is destroyed. It
/// is the finish function of a type-info record (word `+4`): the start-up initializer at `00fbff4f`
/// stores its address there.
pub fn fn_00db7c20(e: &mut Engine, object: Ptr, extra: u32) {
    if object.is_null() {
        return;
    }
    e.mem.set_u32(object.addr(), 0x010c_ce28);
    e.call(0x00cb_0a70, &args![object.addr() + 0x38, extra]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00db7c00, fn_00db7c00(Ptr) -> u32),
        entry!(0x00db7c20, fn_00db7c20(Ptr, u32)),
        entry!(0x00db7c40, fn_00db7c40() -> u32),
        entry!(0x00db7c70, fn_00db7c70(Ptr, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00db7c00_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        e.put_vtable(0x0900_0000, &[0x0900_0100]);
        e.mem.set_u32(object, 0x0900_0000);
        e.register(0x0900_0100, |_, a| Ret {
            eax: a[0].wrapping_add(a[1]).wrapping_add(7),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00db_7c00, &args![object]).u32();
        assert_eq!(answer, object + 7);
        let seen: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0900_0100)
            .cloned()
            .collect();
        assert_eq!(seen, vec![(0x0900_0100, vec![object, 0])]);
    }

    #[test]
    fn fn_00db7c20_stores_the_vtable_and_constructs_the_member() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.register(0x00cb_0a70, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00db_7c20, &args![object, 0x55u32]);
        assert_eq!(e.mem.u32(object), 0x010c_ce28);
        let seen: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00cb_0a70)
            .cloned()
            .collect();
        assert_eq!(seen, vec![(0x00cb_0a70, vec![object + 0x38, 0x55])]);
    }

    #[test]
    fn fn_00db7c20_does_nothing_for_a_null_object() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00db_7c20, &args![0u32, 0x55u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

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
    fn fn_00db7c40_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x38], false);
        let result = e.call(0x00db_7c40, &args![]).u32();
        assert_eq!(result, 0x010c_ce28);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_ce28);
        }
    }

    #[test]
    fn fn_00db7c70_answers_its_second_word() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00db_7c70, &args![0x3333u32, 0x1111u32, 0x2222u32])
                .u32(),
            0x2222
        );
    }
}
