//! `Havok SDK/hkpphantomagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d42770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantomAgent::linearCast` (map name, taken from a vtable). A virtual
/// method with five stack words (`RET 0x14`) whose body is empty.
pub fn hkp_phantom_agent_linear_cast(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
}

/// The shape type of a phantom callback shape (`0x1d`) in the two type words of the agent at
/// `+0x1c` and `+0x20`.
const PHANTOM_CALLBACK_TYPE: u32 = 0x1d;

// Translated from 00d426b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantomAgent::cleanup` (engine map): `ECX` = agent and one stack word the function does
/// not read. If the word at `+0x1c` is `0x1d`, calls the virtual function at vtable offset
/// `0x30` of the object at `+0x14` with `([+0xc], [+0x10])`; if the word at `+0x20` is `0x1d`,
/// the same on the object at `+0x18` with `([+0x10], [+0xc])`. Then calls the virtual function
/// of the agent itself at offset 0 (the deleting destructor) with 1.
pub fn hkp_phantom_agent_cleanup(e: &mut Engine, this: Ptr, _unused: u32) {
    let me = this.addr();
    if e.mem.u32(me + 0x1c) == PHANTOM_CALLBACK_TYPE {
        let target = e.mem.u32(me + 0x14);
        let (first, second) = (e.mem.u32(me + 0xc), e.mem.u32(me + 0x10));
        e.vcall(target, 0x30, &args![first, second]);
    }
    if e.mem.u32(me + 0x20) == PHANTOM_CALLBACK_TYPE {
        let target = e.mem.u32(me + 0x18);
        let (first, second) = (e.mem.u32(me + 0xc), e.mem.u32(me + 0x10));
        e.vcall(target, 0x30, &args![second, first]);
    }
    e.vcall(me, 0, &args![1u32]);
}

// Translated from 00d42700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantomAgent::processCollision` (engine map): four stack words after `this` (a virtual
/// method); the body is empty.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hkp_phantom_agent_process_collision(
    _e: &mut Engine,
    _this: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) {
}

// Translated from 00d42710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantomAgent::getPenetrations` (engine map): four stack words after `this` (a virtual
/// method): `a`, `b`, a word that is not read, and the collector. Calls the collector's virtual
/// function at offset 4 with `(a, b)`.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hkp_phantom_agent_get_penetrations(
    e: &mut Engine,
    _this: u32,
    a: u32,
    b: u32,
    _unused: u32,
    collector: u32,
) {
    e.vcall(collector, 4, &args![a, b]);
}

// Translated from 00d42750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantomAgent::getClosestPoints` (engine map): four stack words after `this` (a virtual
/// method); the body is empty.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hkp_phantom_agent_get_closest_points(
    _e: &mut Engine,
    _this: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) {
}

// Translated from 00d42730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `body_a`, `body_b`, `_unused_2`, `collector`;
/// the roles are descriptions, not confirmed): entry `+4` of the 0x14-byte agent record
/// `00d42960` registers (the static `getPenetrations` of the phantom pair). Hands the first two
/// words to the object `collector` through its virtual function at vtable offset 4
/// (`thiscall`, two stack words) and answers what it answers.
pub fn fn_00d42730(
    e: &mut Engine,
    body_a: u32,
    body_b: u32,
    _unused_2: u32,
    collector: Ptr,
) -> u32 {
    e.vcall(collector.addr(), 4, &args![body_a, body_b]).u32()
}

// Translated from 00d42760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl; entry `+8` of the 0x14-byte agent record `00d42960`
/// registers, the static `getClosestPoints` slot): a bare `RET`, no instruction reads a stack
/// word. Declared with the four words the sibling statics of that slot take (`00d42730` here,
/// `hkpBoxBoxAgent::staticGetClosestPoints` elsewhere), none of them read. It does nothing.
pub fn fn_00d42760(
    _e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) {
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d42730, fn_00d42730(u32, u32, u32, Ptr) -> u32),
        entry!(0x00d42760, fn_00d42760(u32, u32, u32, u32)),
        entry!(0x00d426b0, hkp_phantom_agent_cleanup(Ptr, u32)),
        entry!(
            0x00d42700,
            hkp_phantom_agent_process_collision(u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00d42710,
            hkp_phantom_agent_get_penetrations(u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00d42750,
            hkp_phantom_agent_get_closest_points(u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00d42770,
            hkp_phantom_agent_linear_cast(Ptr, u32, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00d42730_hands_the_first_two_words_to_the_collectors_slot_one() {
        let mut e = Engine::new();
        e.put_vtable(0x0900_0000, &[0, 0x0900_0100]);
        let collector = e.mem.alloc(8);
        e.mem.set_u32(collector, 0x0900_0000);
        e.register(0x0900_0100, |_, a| Ret {
            eax: a[0].wrapping_add(a[1]).wrapping_add(a[2]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x00d4_2730, &args![10u32, 20u32, 30u32, collector])
            .u32();
        assert_eq!(answer, collector + 30);
        let seen: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0900_0100)
            .cloned()
            .collect();
        assert_eq!(seen, vec![(0x0900_0100, vec![collector, 10, 20])]);
    }

    #[test]
    fn fn_00d42760_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00d4_2760, &args![1u32, 2u32, 3u32, 4u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    type Log = Vec<(u32, Vec<u32>)>;

    const DESTRUCTOR: u32 = 0x7000_0000;
    const BODY_A_CLEANUP: u32 = 0x7000_0030;
    const BODY_B_CLEANUP: u32 = 0x7000_0040;
    const ADD_PAIR: u32 = 0x7000_0004;

    /// An agent whose type words are `types`, with fake objects at `+0x14` and `+0x18`.
    fn agent(e: &mut Engine, types: (u32, u32)) -> (u32, u32, u32) {
        let this = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable, DESTRUCTOR);
        let body_a = e.mem.alloc(0x40);
        let body_b = e.mem.alloc(0x40);
        for (body, handler) in [(body_a, BODY_A_CLEANUP), (body_b, BODY_B_CLEANUP)] {
            let table = e.mem.alloc(0x40);
            e.mem.set_u32(body, table);
            e.mem.set_u32(table + 0x30, handler);
        }
        e.mem.set_u32(this + 0xc, 0xaaaa);
        e.mem.set_u32(this + 0x10, 0xbbbb);
        e.mem.set_u32(this + 0x14, body_a);
        e.mem.set_u32(this + 0x18, body_b);
        e.mem.set_u32(this + 0x1c, types.0);
        e.mem.set_u32(this + 0x20, types.1);
        for handler in [DESTRUCTOR, BODY_A_CLEANUP, BODY_B_CLEANUP] {
            e.register(handler, |_, _| Ret::default());
        }
        (this, body_a, body_b)
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn hkp_phantom_agent_linear_cast_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(
            0x00d4_2770,
            &args![Ptr::<()>::new(0x10), 1u32, 2u32, 3u32, 4u32, 5u32],
        );
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn cleanup_tells_both_phantom_callback_bodies_with_the_words_swapped() {
        let mut e = Engine::new();
        let (this, body_a, body_b) = agent(&mut e, (0x1d, 0x1d));
        e.call_log = Some(vec![]);
        e.call(0x00d4_26b0, &args![this, 0u32]);
        let log: Log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, BODY_A_CLEANUP),
            vec![args![body_a, 0xaaaau32, 0xbbbbu32]]
        );
        assert_eq!(
            calls_to(&log, BODY_B_CLEANUP),
            vec![args![body_b, 0xbbbbu32, 0xaaaau32]]
        );
        assert_eq!(calls_to(&log, DESTRUCTOR), vec![args![this, 1u32]]);
        assert_eq!(log.last().unwrap().0, DESTRUCTOR);
    }

    #[test]
    fn cleanup_skips_bodies_that_are_not_phantom_callbacks() {
        let mut e = Engine::new();
        let (this, _, body_b) = agent(&mut e, (0x1c, 0x1d));
        e.call_log = Some(vec![]);
        e.call(0x00d4_26b0, &args![this, 0u32]);
        let log: Log = e.call_log.take().unwrap();
        assert!(calls_to(&log, BODY_A_CLEANUP).is_empty());
        assert_eq!(calls_to(&log, BODY_B_CLEANUP).len(), 1);
        assert_eq!(calls_to(&log, BODY_B_CLEANUP)[0][0], body_b);
        let (this, _, _) = agent(&mut e, (0x1c, 0x1e));
        e.call_log = Some(vec![]);
        e.call(0x00d4_26b0, &args![this, 0u32]);
        let log: Log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(calls_to(&log, DESTRUCTOR), vec![args![this, 1u32]]);
    }

    #[test]
    fn get_penetrations_adds_the_pair_to_the_collector() {
        let mut e = Engine::new();
        let collector = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(collector, vtable);
        e.mem.set_u32(vtable + 4, ADD_PAIR);
        e.register(ADD_PAIR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(
            0x00d4_2710,
            &args![VCALL_THIS, 0x11u32, 0x22u32, 0x33u32, collector],
        );
        let log: Log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ADD_PAIR),
            vec![args![collector, 0x11u32, 0x22u32]]
        );
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn process_collision_and_get_closest_points_do_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00d4_2700, &args![VCALL_THIS, 1u32, 2u32, 3u32, 4u32]);
        e.call(0x00d4_2750, &args![VCALL_THIS, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
