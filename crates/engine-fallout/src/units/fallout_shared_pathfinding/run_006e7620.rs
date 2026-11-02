//! `fallout shared/pathfinding/run_006e7620` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c85750` (no parameters): the running thread's TLS value (the thread's
/// heap data).
const THREAD_DATA: u32 = 0x00c8_5750;
/// `006815c0` (`thiscall`): run on the thread data, answers the object the
/// allocation is made on.
const THREAD_HEAP: u32 = 0x0068_15c0;
/// `005e0ad0` (`thiscall`, `RET 8`): allocates (size, second word).
const HEAP_ALLOCATE: u32 = 0x005e_0ad0;
/// The vtable `006e7650` stores.
const CACHE_VTABLE: u32 = 0x0106_d4bc;

// Translated from 006e7620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates `size` bytes rounded up to a multiple of 0x80 (`cdecl`, two
/// words; no name in the engine map) from the running thread's heap:
/// `005e0ad0` on `006815c0` of the thread data (`00c85750`), with `extra`
/// passed on. Answers the block (`EAX` of `005e0ad0`, which the callers
/// read).
pub fn fn_006e7620(e: &mut Engine, size: u32, extra: u32) -> u32 {
    let thread_data = e.call(THREAD_DATA, &[]).u32();
    let heap = e.call(THREAD_HEAP, &args![thread_data]).u32();
    e.call(
        HEAP_ALLOCATE,
        &args![heap, size.wrapping_add(0x7f) & 0xffff_ff80, extra],
    )
    .u32()
}

// Translated from 006e7650 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e7650 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// A constructor (no name in the engine map): the `NiRefObject` base
/// (`004968b0`), the vtable `0106d4bc`, the members at +8 (`006e9050`),
/// +0x20 (`006e9230`) and +0x30 (`00438660`), and the byte at +0x40 cleared.
/// Returns `this`.
pub fn fn_006e7650(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0049_68b0, &args![this]);
    e.mem.set_u32(base, CACHE_VTABLE);
    e.call(0x006e_9050, &args![base + 8]);
    e.call(0x006e_9230, &args![base + 0x20]);
    e.call(0x0043_8660, &args![base + 0x30]);
    e.mem.set_u8(base + 0x40, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_7620, fn_006e7620(u32, u32) -> u32),
        entry!(0x006e_7650, fn_006e7650(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
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

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn fn_006e7620_rounds_the_size_up_to_0x80() {
        let mut e = Engine::new();
        e.register(THREAD_DATA, |_, _| word(0x4000));
        e.register(THREAD_HEAP, |_, a| word(a[0] + 0x10));
        e.register(HEAP_ALLOCATE, |_, a| word(0x9000 + a[1]));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_7620, &args![0x81u32, 7u32]).u32(), 0x9100);
        assert_eq!(e.call(0x006e_7620, &args![0x80u32, 0u32]).u32(), 0x9080);
        assert_eq!(calls_to(&e, THREAD_HEAP)[0], vec![0x4000]);
        assert_eq!(
            calls_to(&e, HEAP_ALLOCATE),
            vec![vec![0x4010, 0x100, 7], vec![0x4010, 0x80, 0]]
        );
    }

    #[test]
    fn fn_006e7650_constructs_the_members() {
        let mut e = Engine::new();
        for callee in [0x0049_68b0, 0x006e_9050, 0x006e_9230, 0x0043_8660] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x44);
        e.mem.set_u8(this + 0x40, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_7650, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), CACHE_VTABLE);
        assert_eq!(e.mem.u8(this + 0x40), 0);
        assert_eq!(calls_to(&e, 0x0049_68b0), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x006e_9050), vec![vec![this + 8]]);
        assert_eq!(calls_to(&e, 0x006e_9230), vec![vec![this + 0x20]]);
        assert_eq!(calls_to(&e, 0x0043_8660), vec![vec![this + 0x30]]);
    }
}
