//! `NiMain/run_00a89810` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedIncrement` (kernel32 import slot `00fdf1c0`).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `InterlockedDecrement` (kernel32 import slot `00fdf1c4`).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The virtual slot a reference-counted object's last release calls.
const SLOT_DELETE_THIS: u32 = 0x4;
/// The number of reference-counted pointers the object holds.
const POINTER_COUNT: u32 = 7;

// Translated from 00a89810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): a copy constructor of
/// seven reference-counted pointers. It zeroes the seven words at `this`,
/// then assigns each from the word at the same place in `source`: when they
/// differ, the old object (if any) is released (`InterlockedDecrement` on
/// its count at `+4`, and virtual slot `+4` when that reaches zero), the
/// new one is stored and, if not null, gains a reference
/// (`InterlockedIncrement` on its `+4`). Returns `this`.
pub fn fn_00a89810(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    for i in 0..POINTER_COUNT {
        e.mem.set_u32(this.addr().wrapping_add(4 * i), 0);
    }
    for i in 0..POINTER_COUNT {
        let slot = this.addr().wrapping_add(4 * i);
        let from = source.addr().wrapping_add(4 * i);
        // Every slot was zeroed above and is written only here, so `old` is
        // always 0 and the release below never runs; the exe keeps it.
        let old = e.mem.u32(slot);
        if old == e.mem.u32(from) {
            continue;
        }
        if old != 0
            && e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
                .u32()
                == 0
        {
            e.vcall(old, SLOT_DELETE_THIS, &[]);
        }
        let new = e.mem.u32(from);
        e.mem.set_u32(slot, new);
        if new != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
        }
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_9810, fn_00a89810(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn counting(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
    }

    #[test]
    fn copies_seven_pointers_and_adds_a_reference_to_each() {
        let mut e = Engine::new();
        counting(&mut e);
        let source = e.mem.alloc(0x1c);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object + 4, 1);
        e.mem.set_u32(source, object);
        e.mem.set_u32(source + 0x18, object);
        let this = e.mem.alloc(0x1c);
        e.mem.write(this, &[0xcc; 0x1c]);
        assert_eq!(e.call(0x00a8_9810, &[this, source]).u32(), this);
        assert_eq!(e.mem.u32(this), object);
        assert_eq!(e.mem.u32(this + 0x18), object);
        for i in 1..6 {
            assert_eq!(e.mem.u32(this + 4 * i), 0);
        }
        assert_eq!(e.mem.u32(object + 4), 3);
    }

    #[test]
    fn a_source_overlapping_this_reads_the_words_already_copied() {
        // With `source == this - 4` each slot copies the slot before it,
        // which the previous step has just written.
        let mut e = Engine::new();
        counting(&mut e);
        let block = e.mem.alloc(0x20);
        let object = e.mem.alloc(8);
        e.mem.set_u32(block, object);
        e.call(0x00a8_9810, &[block + 4, block]);
        for i in 1..8 {
            assert_eq!(e.mem.u32(block + 4 * i), object);
        }
        assert_eq!(e.mem.u32(object + 4), 7);
    }
}
