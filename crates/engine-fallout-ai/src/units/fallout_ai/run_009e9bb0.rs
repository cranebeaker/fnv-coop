//! `fallout/ai/run_009e9bb0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009e9c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `009e9c30`, which stores the vtable `01092930` at
/// `+0` and frees the block held at `+4` through the cdecl `004ede70`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `009e9c30`'s disassembly ends with the stack cleanup of that call
/// (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_009e9c10(e: &mut Engine, this: Ptr) {
    e.call(0x009e_9c30, &args![this]);
}

/// The base body `009e9c90` (`this` plus two stack words, `RET 8`).
const BASE_BODY_009E9C90: u32 = 0x009e_9c90;
/// The vtable stored at `+0` (RTTI class `NiTPrimitiveArray<Actor*>`).
const VTABLE_009E9BB0: u32 = 0x0109_2928;

// Translated from 009e9bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `009e9c90(this, first, second)`, then stores the vtable
/// `01092928` (RTTI class `NiTPrimitiveArray<Actor*>`) at `+0`. Returns `this`.
pub fn fn_009e9bb0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(BASE_BODY_009E9C90, &args![this, first, second]);
    e.mem.set_u32(this.addr(), VTABLE_009E9BB0);
    this
}

/// `009dad00` (the `ActorMover` constructor, as the decompiler names it): `thiscall`, `RET 4`.
const ACTOR_MOVER_CONSTRUCTOR: u32 = 0x009d_ad00;
/// `006815c0`: a function returning `this` (the compiler's empty `NiPoint3` and `NiMatrix3`
/// default constructors, folded into one body).
const EMPTY_CONSTRUCTOR: u32 = 0x0068_15c0;
/// The vtable this constructor stores.
const VTABLE_0109297C: u32 = 0x0109_297c;
/// Three consecutive dwords copied to `+0x88`.
const POINT_SOURCE: u32 = 0x011f_426c;
/// The float stored at `+0x9c` (`FLD`/`FSTP`).
const FLOAT_AT_9C: u32 = 0x0101_2054;

// Translated from 009e9d00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009e9d00 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f1bf58, state variable) is not translated
/// Unnamed in the engine map. Constructor of a class derived from the `ActorMover` of
/// `009dad00` (`thiscall`, `RET 4`, the word is passed on): runs that constructor, stores the
/// vtable `0109297c`, constructs the member at `+0x88` with `006815c0` and fills it with the three
/// dwords at `011f426c` (plain moves), clears `+0x94` and `+0x98`, and stores the float at
/// `01012054` at `+0x9c` (`FLD`/`FSTP`). Returns `this`. The class name is unconfirmed.
pub fn fn_009e9d00(e: &mut Engine, this: Ptr, base_argument: u32) -> Ptr {
    let base = this.addr();
    e.call(ACTOR_MOVER_CONSTRUCTOR, &args![this, base_argument]);
    e.mem.set_u32(base, VTABLE_0109297C);
    let member = base.wrapping_add(0x88);
    e.call(EMPTY_CONSTRUCTOR, &args![member]);
    for word in 0..3u32 {
        let value = e.mem.u32(POINT_SOURCE + 4 * word);
        e.mem.set_u32(member.wrapping_add(4 * word), value);
    }
    e.mem.set_u32(base.wrapping_add(0x94), 0);
    let bits = x87_quiet_bits(e.mem.u32(FLOAT_AT_9C));
    e.mem.set_u32(base.wrapping_add(0x9c), bits);
    e.mem.set_u32(base.wrapping_add(0x98), 0);
    this
}

/// `NiTArray<Actor *, NiTMallocInterface<..>>` vtable (RTTI).
const VTABLE_TARRAY_ACTOR: u32 = 0x0109_2930;
/// The array's allocation (`0096afc0`, cdecl: the element count).
const ALLOCATE_ELEMENTS: u32 = 0x0096_afc0;
/// The array's free (`004ede70`, cdecl: the block).
const FREE_ELEMENTS: u32 = 0x004e_de70;

// Translated from 009e9c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<Actor *, NiTMallocInterface<..>>` constructor (class by the vtable's RTTI):
/// `m_usMaxSize` (+8) = `max_size`, `m_usGrowBy` (+0xe) = `grow_by`, `m_usSize` (+0xa) and
/// `m_usESize` (+0xc) = 0; `m_pBase` (+4) is `0096afc0(max_size)`, or null when `max_size`
/// (read back from +8) is 0. Returns `this`.
pub fn ni_t_array_actor_constructor(e: &mut Engine, this: Ptr, max_size: u16, grow_by: u16) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_TARRAY_ACTOR);
    e.mem.set_u16(base + 8, max_size);
    e.mem.set_u16(base + 0xe, grow_by);
    e.mem.set_u16(base + 0xa, 0);
    e.mem.set_u16(base + 0xc, 0);
    let count = u32::from(e.mem.u16(base + 8));
    if count == 0 {
        e.mem.set_u32(base + 4, 0);
    } else {
        let block = e.call(ALLOCATE_ELEMENTS, &args![count]).u32();
        e.mem.set_u32(base + 4, block);
    }
    this
}

// Translated from 009e9c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<Actor *, NiTMallocInterface<..>>` destructor (class by the vtable's RTTI; the
/// decompiler's library match `~basic_streambuf` is a folded body): stores the vtable and
/// frees `m_pBase` (+4) through `004ede70`.
pub fn ni_t_array_actor_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_TARRAY_ACTOR);
    let data = e.mem.u32(this.addr() + 4);
    e.call(FREE_ELEMENTS, &args![data]);
}

/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 009e9be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the scalar deleting destructor of the
/// `NiTPrimitiveArray<Actor*>` (vtable `01092928`): [`fn_009e9c10`], then `operator delete`
/// on `this` for bit 0 of `flags`. Returns `this`.
pub fn fn_009e9be0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009e9c10(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009e9c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the scalar deleting destructor of the
/// `NiTArray<Actor*>`: [`ni_t_array_actor_destructor`], then `operator delete` on `this`
/// for bit 0 of `flags`. Returns `this`.
pub fn fn_009e9c60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    ni_t_array_actor_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009e_9be0, fn_009e9be0(Ptr, u32) -> Ptr),
        entry!(0x009e_9c60, fn_009e9c60(Ptr, u32) -> Ptr),
        entry!(0x009e_9c90, ni_t_array_actor_constructor(Ptr, u16, u16) -> Ptr),
        entry!(0x009e_9c30, ni_t_array_actor_destructor(Ptr)),
        entry!(0x009e_9d00, fn_009e9d00(Ptr, u32) -> Ptr),
        entry!(0x009e_9c10, fn_009e9c10(Ptr)),
        entry!(0x009e_9bb0, fn_009e9bb0(Ptr, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_009e9c10_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x009e_9c30, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009e_9c10, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x009e_9c30), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn runs_the_body_then_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(BASE_BODY_009E9C90, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009e_9bb0, &args![this, 8u32, 9u32]).u32(), this);
        assert_eq!(calls_to(&mut e, BASE_BODY_009E9C90), vec![vec![this, 8, 9]]);
        assert_eq!(e.mem.u32(this), VTABLE_009E9BB0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_009e9d00_runs_the_base_constructor_then_fills_in_the_members() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(POINT_SOURCE, 0x10);
        e.map(FLOAT_AT_9C, 4);
        e.mem.set_u32(POINT_SOURCE, 0x1111_1111);
        e.mem.set_u32(POINT_SOURCE + 4, 0x2222_2222);
        e.mem.set_u32(POINT_SOURCE + 8, 0x3333_3333);
        e.mem.set_u32(FLOAT_AT_9C, (-1.0f32).to_bits());
        e.register(ACTOR_MOVER_CONSTRUCTOR, |e, a| {
            // The base constructor leaves its own vtable and some junk in the members.
            e.mem.set_u32(a[0], 0x0aaa_0000);
            e.mem.set_u32(a[0] + 0x94, 0x7777_7777);
            e.mem.set_u32(a[0] + 0x98, 0x8888_8888);
            a[0].into_ret()
        });
        e.register(EMPTY_CONSTRUCTOR, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0xa8);
        let result = e.call(0x009e_9d00, &args![this, 0x4242u32]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), VTABLE_0109297C);
        assert_eq!(e.mem.u32(this + 0x88), 0x1111_1111);
        assert_eq!(e.mem.u32(this + 0x8c), 0x2222_2222);
        assert_eq!(e.mem.u32(this + 0x90), 0x3333_3333);
        assert_eq!(e.mem.u32(this + 0x94), 0);
        assert_eq!(e.mem.u32(this + 0x98), 0);
        assert_eq!(e.mem.u32(this + 0x9c), (-1.0f32).to_bits());
        let log = e.call_log.take().unwrap();
        let order: Vec<(u32, Vec<u32>)> = log.into_iter().filter(|c| c.0 != 0x009e_9d00).collect();
        assert_eq!(
            order,
            vec![
                (ACTOR_MOVER_CONSTRUCTOR, vec![this, 0x4242]),
                (EMPTY_CONSTRUCTOR, vec![this + 0x88]),
            ]
        );
    }

    #[test]
    fn at_009e9d00_quiets_a_signalling_float() {
        let mut e = Engine::new();
        e.map(POINT_SOURCE, 0x10);
        e.map(FLOAT_AT_9C, 4);
        e.mem.set_u32(FLOAT_AT_9C, 0x7f80_0001);
        e.register(ACTOR_MOVER_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(EMPTY_CONSTRUCTOR, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0xa8);
        e.call(0x009e_9d00, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x9c), 0x7fc0_0001);
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn actor_array_constructor_allocates_unless_the_size_is_zero() {
        let mut e = Engine::new();
        e.register(ALLOCATE_ELEMENTS, |_, a| (0x6000 + a[0]).into_ret());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let r = e.call(0x009e_9c90, &args![this, 0xaaaa_0005u32, 0xbbbb_0003u32]);
        assert_eq!(r.u32(), this);
        assert_eq!(calls_to(&e, ALLOCATE_ELEMENTS), vec![vec![5]]);
        assert_eq!(e.mem.u32(this), VTABLE_TARRAY_ACTOR);
        assert_eq!(e.mem.u32(this + 4), 0x6005);
        assert_eq!(e.mem.u16(this + 8), 5);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
        assert_eq!(e.mem.u16(this + 0xe), 3);
        e.call_log = Some(vec![]);
        e.call(0x009e_9c90, &args![this, 0u32, 1u32]);
        assert!(calls_to(&e, ALLOCATE_ELEMENTS).is_empty());
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    #[test]
    fn actor_array_destructor_frees_the_elements() {
        let mut e = Engine::new();
        e.register(FREE_ELEMENTS, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x7070);
        e.call_log = Some(vec![]);
        e.call(0x009e_9c30, &args![this]);
        assert_eq!(calls_to(&e, FREE_ELEMENTS), vec![vec![0x7070]]);
        assert_eq!(e.mem.u32(this), VTABLE_TARRAY_ACTOR);
    }
}

#[cfg(test)]
mod n029_tests {
    use super::*;

    fn frees(e: &mut Engine, addr: u32, this: u32, flags: u32) -> usize {
        e.call_log = Some(vec![]);
        assert_eq!(e.call(addr, &args![this, flags]).u32(), this);
        let log = e.call_log.take().unwrap();
        log.iter().filter(|c| c.0 == OPERATOR_DELETE).count()
    }

    #[test]
    fn fn_009e9be0_destroys_and_frees_for_bit_0() {
        let mut e = Engine::new();
        e.register(0x009e_9c30, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        assert_eq!(frees(&mut e, 0x009e_9be0, this, 0), 0);
        assert_eq!(frees(&mut e, 0x009e_9be0, this, 3), 1);
    }

    #[test]
    fn fn_009e9c60_destroys_and_frees_for_bit_0() {
        let mut e = Engine::new();
        e.register(FREE_ELEMENTS, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x7070);
        assert_eq!(frees(&mut e, 0x009e_9c60, this, 2), 0);
        assert_eq!(frees(&mut e, 0x009e_9c60, this, 1), 1);
        assert_eq!(e.mem.u32(this), VTABLE_TARRAY_ACTOR);
    }
}
