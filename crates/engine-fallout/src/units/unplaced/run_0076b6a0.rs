//! `(unplaced)/run_0076b6a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `0057bd10` (`this` plus one stack word, `RET 4`).
const BASE_CONSTRUCTOR_0057BD10: u32 = 0x0057_bd10;

// Translated from 0076b930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: calls `0057bd10(this, 0)`, then clears the word at `+0xc`.
/// Returns `this`.
pub fn fn_0076b930(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR_0057BD10, &args![this, 0u32]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    this
}

/// `0076b870` (`thiscall`, `RET 4`): run on each chained object with 1.
const CHAINED_DESTROY: u32 = 0x0076_b870;
/// `00418900` (`thiscall`, `RET 4`; unit `fallout shared/extradatalist.cpp`).
const ASSIGN_FROM: u32 = 0x0041_8900;
/// `00483710` (`thiscall`, no stack word; unit `fallout shared/tesform.cpp`): the base
/// destructor (also of the local).
const BASE_DESTRUCTOR: u32 = 0x0048_3710;

// Translated from 0076b7a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0076b7a0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f0b368, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): destroys the chain at +0xc (each
/// object's +0xc is the next one: unlinked, then `0076b870(1)` on it), then builds a local
/// with `0057bd10(0)`, passes it to `00418900` on `this`, and destroys the local
/// (`00483710`).
pub fn fn_0076b7a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    while e.mem.u32(base + 0xc) != 0 {
        let head = e.mem.u32(base + 0xc);
        let next = e.mem.u32(head + 0xc);
        e.mem.set_u32(head + 0xc, 0);
        let object = e.mem.u32(base + 0xc);
        if object != 0 {
            e.call(CHAINED_DESTROY, &args![object, 1u32]);
        }
        e.mem.set_u32(base + 0xc, next);
    }
    e.with_stack(0xc, |e, local| {
        let made = e.call(BASE_CONSTRUCTOR_0057BD10, &args![local, 0u32]).u32();
        e.call(ASSIGN_FROM, &args![this, made]);
        e.call(BASE_DESTRUCTOR, &args![local]);
    });
}

// Translated from 0076b8a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0076b8a0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f005f8, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word; Ghidra matches the body to a library
/// destructor pattern): [`fn_0076b7a0`], then the base destructor `00483710`.
pub fn fn_0076b8a0(e: &mut Engine, this: Ptr) {
    fn_0076b7a0(e, this);
    e.call(BASE_DESTRUCTOR, &args![this]);
}

/// `00559450` (`thiscall`, no stack word): the word at `this`.
const FIRST_WORD: u32 = 0x0055_9450;
/// `0076a3b0` (`thiscall`, one word; unit `fallout/interface/menus/hackingmenu.cpp`).
const CHECK_0076A3B0: u32 = 0x0076_a3b0;
/// The game's `operator new` (`00401000`, `cdecl`, the size) and `operator delete`
/// (`00401030`, `cdecl`, the block).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 0076b780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): whether `value` equals the word
/// `00559450` reads at `this`.
pub fn fn_0076b780(e: &mut Engine, this: Ptr, value: u32) -> bool {
    value == e.call(FIRST_WORD, &args![this]).u32()
}

// Translated from 0076b900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): a constructor of the 0x10-byte chain
/// node: `00418900(this, source)`, then the next link at +0xC cleared. Returns `this`.
pub fn fn_0076b900(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    e.call(ASSIGN_FROM, &args![this, source]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    this
}

// Translated from 0076b870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): the scalar deleting destructor of the
/// chain node: [`fn_0076b8a0`], then `operator delete` (`00401030`) when bit 0 of `flags`
/// is set. Returns `this`.
pub fn fn_0076b870(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0076b8a0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0076b6a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0076b6a0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eeb0fb, state variable around the node's construction) is not translated
/// Unnamed in the engine map (`thiscall`, one word): nothing when `other`'s first word is 0
/// ([`fn_0076b780`] with 0). Otherwise, when `0076a3b0(this, 0)` answers true, a new
/// 0x10-byte node (`operator new`, then [`fn_0076b900`] with `this`; 0 when the allocation
/// fails) is linked in front of the chain at `this + 0xC` before `00418900(this, other)`;
/// when it answers false only `00418900(this, other)` runs.
pub fn fn_0076b6a0(e: &mut Engine, this: Ptr, other: Ptr) {
    if fn_0076b780(e, other, 0) {
        return;
    }
    if e.call(CHECK_0076A3B0, &args![this, 0u32]).bool() {
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let node = if block != 0 {
            fn_0076b900(e, Ptr::new(block), this.addr()).addr()
        } else {
            0
        };
        let head = e.mem.u32(this.addr() + 0xc);
        e.mem.set_u32(node + 0xc, head);
        e.mem.set_u32(this.addr() + 0xc, node);
        e.call(ASSIGN_FROM, &args![this, other]);
    } else {
        e.call(ASSIGN_FROM, &args![this, other]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0076_b930, fn_0076b930(Ptr) -> Ptr),
        entry!(0x0076_b7a0, fn_0076b7a0(Ptr)),
        entry!(0x0076_b8a0, fn_0076b8a0(Ptr)),
        entry!(0x0076_b780, fn_0076b780(Ptr, u32) -> bool),
        entry!(0x0076_b900, fn_0076b900(Ptr, u32) -> Ptr),
        entry!(0x0076_b870, fn_0076b870(Ptr, u32) -> Ptr),
        entry!(0x0076_b6a0, fn_0076b6a0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests_n077 {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_0076b780_compares_with_the_first_word() {
        let mut e = Engine::new();
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 9);
        assert!(e.call(0x0076_b780, &args![this, 9u32]).bool());
        assert!(!e.call(0x0076_b780, &args![this, 0u32]).bool());
    }

    #[test]
    fn fn_0076b900_assigns_and_clears_the_link() {
        let mut e = Engine::new();
        e.register(ASSIGN_FROM, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_b900, &args![this, 0x4444u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ASSIGN_FROM), vec![vec![this, 0x4444]]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn fn_0076b870_deletes_only_with_bit_0() {
        let mut e = Engine::new();
        for addr in [ASSIGN_FROM, BASE_DESTRUCTOR, OPERATOR_DELETE] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(BASE_CONSTRUCTOR_0057BD10, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_b870, &args![this, 2u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&log, BASE_DESTRUCTOR).last(), Some(&vec![this]));
        e.call_log = Some(vec![]);
        e.call(0x0076_b870, &args![this, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn fn_0076b6a0_links_a_node_only_when_asked() {
        let mut e = Engine::new();
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ASSIGN_FROM, |_, _| Ret::default());
        e.register(CHECK_0076A3B0, |e, a| (e.mem.u32(a[0]) == 1).into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0x7777);
        // An empty `other`: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0076_b6a0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(FIRST_WORD, vec![other])]);
        // The check answers false: only the assignment.
        e.mem.set_u32(other, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x0076_b6a0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ASSIGN_FROM), vec![vec![this, other]]);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // It answers true: a node goes in front of the chain.
        e.mem.set_u32(this, 1);
        e.call_log = Some(vec![]);
        e.call(0x0076_b6a0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        let node = e.mem.u32(this + 0xc);
        assert_eq!(e.mem.u32(node + 0xc), 0x7777);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(
            calls_to(&log, ASSIGN_FROM),
            vec![vec![node, this], vec![this, other]]
        );
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
    fn calls_the_base_then_clears_the_word() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR_0057BD10, |e, a| {
            e.mem.set_u32(a[0] + 0xc, 0x5555);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_b930, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_0057BD10),
            vec![vec![this, 0]]
        );
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_0076b8a0_destroys_the_chain_then_resets_through_a_local() {
        let mut e = Engine::new();
        e.register(CHAINED_DESTROY, |e, a| {
            assert_eq!(e.mem.u32(a[0] + 0xc), 0);
            Ret::default()
        });
        e.register(BASE_CONSTRUCTOR_0057BD10, |_, a| a[0].into_ret());
        e.register(ASSIGN_FROM, |_, _| Ret::default());
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, first);
        e.mem.set_u32(first + 0xc, second);
        e.call_log = Some(vec![]);
        e.call(0x0076_b8a0, &args![this]);
        let log = e.call_log.take().unwrap();
        let local = log
            .iter()
            .find(|c| c.0 == BASE_CONSTRUCTOR_0057BD10)
            .unwrap()
            .1[0];
        assert_eq!(
            log[1..],
            [
                (CHAINED_DESTROY, vec![first, 1]),
                (CHAINED_DESTROY, vec![second, 1]),
                (BASE_CONSTRUCTOR_0057BD10, vec![local, 0]),
                (ASSIGN_FROM, vec![this, local]),
                (BASE_DESTRUCTOR, vec![local]),
                (BASE_DESTRUCTOR, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }
}
