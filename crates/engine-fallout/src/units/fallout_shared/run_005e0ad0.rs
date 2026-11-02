//! `fallout shared/run_005e0ad0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00537e90` (named `BGSUnloadedFormBuffer::BGSUnloadedFormBuffer` in the engine map):
/// the base body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const STORE_TARGET_00537E90: u32 = 0x0053_7e90;

// Translated from 005e0b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards `this` and one word to `00537e90` (the engine map names it
/// `BGSUnloadedFormBuffer::BGSUnloadedFormBuffer`; Xbox PDB
/// `??0BGSUnloadedFormBuffer@@QAA@PAD@Z` takes a `char *`), which stores the word at `this+0`
/// and leaves `this` in `EAX` (`MOV EAX,[EBP-4]` before the store). The wrapper returns that
/// `this`.
pub fn fn_005e0b80(e: &mut Engine, this: Ptr, data: Ptr) -> Ptr {
    e.call(STORE_TARGET_00537E90, &args![this, data]).ptr()
}

/// `00458b50` (unit `tes.cpp`, `thiscall`; a word): the value of a member
/// (the object's own at +0, and at +8).
const MEMBER_VALUE: u32 = 0x0045_8b50;
/// The member `005e0b10` reads besides the first (+0x08).
const SECOND_MEMBER: u32 = 0x08;
/// `00c85750` (unit `hkmemoryrouter.cpp`, no arguments): the thread's Havok
/// memory router; `006815c0` (`thiscall`) returns its object.
const THREAD_ROUTER: u32 = 0x00c8_5750;
const ROUTER_OBJECT: u32 = 0x0068_15c0;

// Translated from 005e0ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_005e0ad0`): runs `005e0b10`, takes
/// the member value (`00458b50`), stores that value plus `size`
/// (`005e0b50`) through `00537e90`, and returns the value. The second
/// argument word is not read.
pub fn fn_005e0ad0(e: &mut Engine, this: Ptr, size: u32, _unused_2: u32) -> u32 {
    fn_005e0b10(e, this);
    let value = e.call(MEMBER_VALUE, &args![this]).u32();
    let end = fn_005e0b50(e, value, size);
    e.call(STORE_TARGET_00537E90, &args![this, end]);
    value
}

// Translated from 005e0b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_005e0ad0`): the difference of the
/// member values at +0x08 and +0x00 (`005e0b40`); the result stays in `EAX`
/// and is returned.
pub fn fn_005e0b10(e: &mut Engine, this: Ptr) -> u32 {
    let second = e
        .call(
            MEMBER_VALUE,
            &args![this.addr().wrapping_add(SECOND_MEMBER)],
        )
        .u32();
    let first = e.call(MEMBER_VALUE, &args![this]).u32();
    fn_005e0b40(e, first, second)
}

// Translated from 005e0b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`): `b - a`.
pub fn fn_005e0b40(_e: &mut Engine, a: u32, b: u32) -> u32 {
    b.wrapping_sub(a)
}

// Translated from 005e0b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`): `a + b`.
pub fn fn_005e0b50(_e: &mut Engine, a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

// Translated from 005e0b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): hands `block` to the
/// object of the thread's Havok memory router (`00c85750`, `006815c0`)
/// through `005e0b80`; the word the caller pushed is the argument
/// `005e0b80` pops (`RET 4`). Returns what `005e0b80` returns.
pub fn fn_005e0b60(e: &mut Engine, block: Ptr) -> Ptr {
    let router = e.call(THREAD_ROUTER, &args![]).u32();
    let object = e.call(ROUTER_OBJECT, &args![router]).u32();
    fn_005e0b80(e, Ptr::new(object), block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_0b80, fn_005e0b80(Ptr, Ptr) -> Ptr),
        entry!(0x005e_0ad0, fn_005e0ad0(Ptr, u32, u32) -> u32),
        entry!(0x005e_0b10, fn_005e0b10(Ptr) -> u32),
        entry!(0x005e_0b40, fn_005e0b40(u32, u32) -> u32),
        entry!(0x005e_0b50, fn_005e0b50(u32, u32) -> u32),
        entry!(0x005e_0b60, fn_005e0b60(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_005e0b80_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(STORE_TARGET_00537E90, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x005e_0b80, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == STORE_TARGET_00537E90)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }

    #[test]
    fn fn_005e0ad0_stores_the_value_plus_the_size() {
        let mut e = Engine::new();
        e.register(MEMBER_VALUE, |_, a| (a[0] + 0x100).into_ret());
        e.register(STORE_TARGET_00537E90, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x005e_0ad0, &args![0x4000u32, 0x20u32, 9u32]).u32(),
            0x4100
        );
        let log = e.call_log.take().unwrap();
        let values: Vec<_> = log
            .iter()
            .filter(|c| c.0 == MEMBER_VALUE)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(values, [vec![0x4008], vec![0x4000], vec![0x4000]]);
        let store = log.iter().find(|c| c.0 == STORE_TARGET_00537E90).unwrap();
        assert_eq!(store.1, [0x4000, 0x4120]);
    }

    #[test]
    fn fn_005e0b10_returns_the_member_difference() {
        let mut e = Engine::new();
        e.register(MEMBER_VALUE, |e, a| e.mem.u32(a[0]).into_ret());
        let this = e.mem.alloc(12);
        e.mem.set_u32(this, 0x100);
        e.mem.set_u32(this + 8, 0x180);
        assert_eq!(e.call(0x005e_0b10, &args![this]).u32(), 0x80);
    }

    #[test]
    fn fn_005e0b40_and_fn_005e0b50_subtract_and_add() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x005e_0b40, &args![3u32, 10u32]).u32(), 7);
        assert_eq!(e.call(0x005e_0b40, &args![10u32, 3u32]).u32(), 0xffff_fff9);
        assert_eq!(
            e.call(0x005e_0b50, &args![0xffff_fff0u32, 0x20u32]).u32(),
            0x10
        );
    }

    #[test]
    fn fn_005e0b60_hands_the_block_to_the_router_object() {
        let mut e = Engine::new();
        e.register(THREAD_ROUTER, |_, _| 0x6000u32.into_ret());
        e.register(ROUTER_OBJECT, |_, a| (a[0] + 4).into_ret());
        e.register(STORE_TARGET_00537E90, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_0b60, &args![0x7000u32]).u32(), 0x6004);
        let log = e.call_log.take().unwrap();
        let store = log.iter().find(|c| c.0 == STORE_TARGET_00537E90).unwrap();
        assert_eq!(store.1, [0x6004, 0x7000]);
        let router = log.iter().find(|c| c.0 == THREAD_ROUTER).unwrap();
        assert!(router.1.is_empty());
    }
}
