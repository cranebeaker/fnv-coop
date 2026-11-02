//! `Havok SDK/run_00e46df0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkAgentNnMachine_FindAgent` (`00cfdd70`, engine map name): cdecl, two words.
const FIND_AGENT: u32 = 0x00cf_dd70;
/// `hkpWorldAgentUtil::removeAgentAndItsToiEvents` (Xbox PDB, `00cc1160`): cdecl, one word.
const REMOVE_AGENT: u32 = 0x00cc_1160;

// Translated from 00e46df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc` (no reference to it was found in the exe's data): writes a byte to
/// `out`. With the object at `+0x20` null the byte is 1. Otherwise virtual slot `4` of the
/// interface at `+0x14` of that object is called with `(slot, first, second)`, where `slot` is
/// a stack word the caller owns (it starts as `second`, and the slot function writes its answer
/// byte there and returns its address); the byte is that answer: 0 when it is 0, else 1.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool result returned
/// through a pointer), and virtual call sites test the flag through `EAX` (e.g. 00c90f6a).
pub fn fn_00e46df0(e: &mut Engine, this: Ptr, out: u32, first: u32, second: u32) -> u32 {
    let owner = e.mem.u32(this.addr().wrapping_add(0x20));
    if owner != 0 {
        let answer = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), second);
            let at = e
                .vcall(owner.wrapping_add(0x14), 4, &args![slot, first, second])
                .u32();
            e.mem.u8(at)
        });
        if answer == 0 {
            e.mem.set_u8(out, 0);
            return out;
        }
    }
    e.mem.set_u8(out, 1);
    out
}

// Translated from 00e46e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4` (virtual: referenced from the table at `010e7820`; `this` is not read):
/// `entity` is an object whose `+0xc` sub-object is asked for its virtual slot `0x20`. Unless the
/// answer is `0xb`, the agent between the objects at `+0x14` and `+0x18` (each `+0x10`) is
/// looked up with `00cfdd70` and, when found, removed with `00cc1160`.
pub fn fn_00e46e30(e: &mut Engine, _this: Ptr, entity: u32) {
    let object = e.mem.u32(entity.wrapping_add(0xc));
    let kind = e.vcall(object, 0x20, &[]).u32();
    if kind == 0xb {
        return;
    }
    let second = e.mem.u32(entity.wrapping_add(0x18)).wrapping_add(0x10);
    let first = e.mem.u32(entity.wrapping_add(0x14)).wrapping_add(0x10);
    let agent = e.call(FIND_AGENT, &args![first, second]).u32();
    if agent != 0 {
        e.call(REMOVE_AGENT, &args![agent]);
    }
}

// Translated from 00e46e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual, `RET 4`: does nothing; the stack word is not read.
pub fn fn_00e46e70(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00e46e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall: clears the five words at `this` and returns `this`.
pub fn fn_00e46e80(e: &mut Engine, this: Ptr) -> u32 {
    let this = this.addr();
    for word in 0..5u32 {
        e.mem.set_u32(this.wrapping_add(4 * word), 0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_6df0, fn_00e46df0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00e4_6e30, fn_00e46e30(Ptr, u32)),
        entry!(0x00e4_6e70, fn_00e46e70(Ptr, u32)),
        entry!(0x00e4_6e80, fn_00e46e80(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0010_0000);
        e
    }

    /// An object at `+0x20` of the result whose `+0x14` interface has vtable slot 1 answering
    /// with `answer` (written to the given slot), recording the words it received.
    fn filter_owner(e: &mut Engine, answer: u8) -> (u32, Calls) {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&calls);
        e.register_double(0x0ff0_0004, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            e.mem.set_u8(a[1], answer);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.put_vtable(0x0126_0000, &[0, 0x0ff0_0004]);
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner + 0x14, 0x0126_0000);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x20, owner);
        (this, calls)
    }

    fn run(e: &mut Engine, this: u32) -> u8 {
        let out = e.mem.alloc(8);
        e.mem.set_u8(out, 0xee);
        e.call(0x00e4_6df0, &args![this, out, 0x11u32, 0x22u32]);
        assert_eq!(e.mem.u8(out + 1), 0);
        e.mem.u8(out)
    }

    #[test]
    fn no_owner_gives_one() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        assert_eq!(run(&mut e, this), 1);
    }

    #[test]
    fn the_filter_answers_its_out_pointer() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        let out = e.mem.alloc(8);
        // EAX is `out` at both RETs; virtual call sites test the flag through it (00c90f6a).
        assert_eq!(
            e.call(0x00e4_6df0, &args![this, out, 0x11u32, 0x22u32])
                .u32(),
            out
        );
    }

    #[test]
    fn owner_answer_decides_the_byte() {
        let mut e = engine();
        let (this, calls) = filter_owner(&mut e, 0);
        assert_eq!(run(&mut e, this), 0);
        let owner = e.mem.u32(this + 0x20);
        let words = calls.borrow()[0].clone();
        assert_eq!(words[0], owner + 0x14);
        assert_eq!(&words[2..], &[0x11, 0x22]);
        let (this, _) = filter_owner(&mut e, 5);
        assert_eq!(run(&mut e, this), 1);
    }

    fn removal_doubles(e: &mut Engine, kind: u32, agent: u32) -> Rc<RefCell<Vec<(u32, Vec<u32>)>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        e.register_double(0x0ff0_0020, move |_, _| Ret {
            eax: kind,
            ..Ret::default()
        });
        e.put_vtable(0x0126_0100, &[0; 8]);
        e.mem.set_u32(0x0126_0100 + 0x20, 0x0ff0_0020);
        let log = Rc::clone(&seen);
        e.register_double(FIND_AGENT, move |_, a| {
            log.borrow_mut().push((FIND_AGENT, a.to_vec()));
            Ret {
                eax: agent,
                ..Ret::default()
            }
        });
        let log = Rc::clone(&seen);
        e.register_double(REMOVE_AGENT, move |_, a| {
            log.borrow_mut().push((REMOVE_AGENT, a.to_vec()));
            Ret::default()
        });
        seen
    }

    fn entity(e: &mut Engine) -> u32 {
        let entity = e.mem.alloc(0x20);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0126_0100);
        e.mem.set_u32(entity + 0xc, object);
        e.mem.set_u32(entity + 0x14, 0x1000);
        e.mem.set_u32(entity + 0x18, 0x2000);
        entity
    }

    #[test]
    fn found_agent_is_removed() {
        let mut e = engine();
        let seen = removal_doubles(&mut e, 3, 0x7777);
        let entity = entity(&mut e);
        e.call(0x00e4_6e30, &args![0u32, entity]);
        assert_eq!(
            *seen.borrow(),
            vec![
                (FIND_AGENT, vec![0x1010, 0x2010]),
                (REMOVE_AGENT, vec![0x7777])
            ]
        );
    }

    #[test]
    fn missing_agent_or_kind_0xb_removes_nothing() {
        let mut e = engine();
        let seen = removal_doubles(&mut e, 3, 0);
        let entity = entity(&mut e);
        e.call(0x00e4_6e30, &args![0u32, entity]);
        assert_eq!(*seen.borrow(), vec![(FIND_AGENT, vec![0x1010, 0x2010])]);
        let seen = removal_doubles(&mut e, 0xb, 0x7777);
        e.call(0x00e4_6e30, &args![0u32, entity]);
        assert!(seen.borrow().is_empty());
    }

    #[test]
    fn empty_virtual_and_clear_function() {
        let mut e = engine();
        e.call(0x00e4_6e70, &args![0u32, 1u32]);
        let this = e.mem.alloc(0x20);
        for word in 0..8u32 {
            e.mem.set_u32(this + 4 * word, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00e4_6e80, &args![this]).u32(), this);
        for word in 0..5u32 {
            assert_eq!(e.mem.u32(this + 4 * word), 0);
        }
        assert_eq!(e.mem.u32(this + 20), 0xffff_ffff);
    }
}
