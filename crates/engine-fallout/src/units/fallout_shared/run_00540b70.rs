//! `fallout shared/run_00540b70` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0062a120` (thiscall, no stack words): passes the first two words of
/// `this` to `Error(first, second, 0)` (engine map name of its callee) and
/// clears the second.
const CLEAR_CONTENTS_0062A120: u32 = 0x0062_a120;
/// `0062a100` (thiscall): the word at +8 masked with `0x3fffffff`.
const CAPACITY_0062A100: u32 = 0x0062_a100;
/// `00540c20` (thiscall on the allocator: the buffer and its capacity;
/// calls the allocator's vtable slot +0x10 with them).
const ALLOCATOR_RELEASE_00540C20: u32 = 0x0054_0c20;
/// The buffer word (+0) and the capacity/flags word (+8; bit 31 set when
/// the buffer is not to be released).
const BUFFER: u32 = 0x00;
const CAPACITY_FLAGS: u32 = 0x08;
const NOT_OWNED: u32 = 0x8000_0000;

// Translated from 00540b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a buffer through `allocator` (thiscall, `RET 4`; unnamed in the
/// engine map): `0062a120(this)`; when bit 31 of the word at +8 is clear,
/// `00540c20(allocator, buffer, capacity)` with the capacity from
/// `0062a100`. Then the buffer word is cleared and the word at +8 becomes
/// `0x80000000`.
pub fn fn_00540b70(e: &mut Engine, this: Ptr, allocator: Ptr) {
    let base = this.addr();
    e.call(CLEAR_CONTENTS_0062A120, &args![this]);
    if e.mem.u32(base + CAPACITY_FLAGS) & NOT_OWNED == 0 {
        let capacity = e.call(CAPACITY_0062A100, &args![this]).u32();
        let buffer = e.mem.u32(base + BUFFER);
        e.call(
            ALLOCATOR_RELEASE_00540C20,
            &args![allocator, buffer, capacity],
        );
    }
    e.mem.set_u32(base + BUFFER, 0);
    e.mem.set_u32(base + CAPACITY_FLAGS, NOT_OWNED);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0054_0b70, fn_00540b70(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn setup(e: &mut Engine) {
        e.register(CLEAR_CONTENTS_0062A120, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(CAPACITY_0062A100, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8) & 0x3fff_ffff,
            ..Ret::default()
        });
        e.register(ALLOCATOR_RELEASE_00540C20, |_, _| Ret::default());
    }

    #[test]
    fn owned_buffer_is_released_with_its_capacity() {
        let mut e = Engine::new();
        setup(&mut e);
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this, 0x0abc_0000);
        e.mem.set_u32(this + 4, 7);
        e.mem.set_u32(this + 8, 0x4000_0020);
        e.call_log = Some(vec![]);
        e.call(0x0054_0b70, &args![this, 0x0555_0000u32]);
        assert_eq!(calls_to(&e, CLEAR_CONTENTS_0062A120), vec![vec![this]]);
        assert_eq!(
            calls_to(&e, ALLOCATOR_RELEASE_00540C20),
            vec![vec![0x0555_0000, 0x0abc_0000, 0x20]]
        );
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 8), 0x8000_0000);
    }

    #[test]
    fn unowned_buffer_is_only_forgotten() {
        let mut e = Engine::new();
        setup(&mut e);
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this, 0x0abc_0000);
        e.mem.set_u32(this + 8, 0x8000_0020);
        e.call_log = Some(vec![]);
        e.call(0x0054_0b70, &args![this, 0x0555_0000u32]);
        assert!(calls_to(&e, ALLOCATOR_RELEASE_00540C20).is_empty());
        assert!(calls_to(&e, CAPACITY_0062A100).is_empty());
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 8), 0x8000_0000);
    }
}
