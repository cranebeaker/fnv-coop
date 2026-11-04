//! `(unplaced)/run_00a305e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Four small release-build helpers on objects whose first words are
//! pooled-string handles (`NiFixedString`-style: the reference count sits
//! at `handle - 8`) or reference-counted pointers. The engine map names
//! none of them.

#[allow(unused_imports)]
use crate::prelude::*;

/// The import slot of `InterlockedDecrement` (`00fdf1c4`).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The pooled-string handle whose count is never released (`0109b220`).
const NO_STRING_HANDLE: u32 = 0x0109_b220;
/// `00a5b690`: makes a pooled-string handle from a C string (cdecl).
const MAKE_HANDLE: u32 = 0x00a5_b690;
/// Reference-counted object: the count at +4, `DeleteThis` in vtable slot
/// +0x04.
const REF_COUNT_OFFSET: u32 = 4;
const SLOT_DELETE_THIS: u32 = 0x04;

// Translated from 00a305e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the string at `this[0]` equals `text`: the same pointer is
/// equal, a null on either side is not, otherwise the bytes are compared
/// up to the terminator (the compiler's inlined `strcmp`).
pub fn fn_00a305e0(e: &mut Engine, this: Ptr, text: Ptr) -> bool {
    let mut own = e.mem.u32(this.addr());
    let mut other = text.addr();
    if own == other {
        return true;
    }
    if other == 0 || own == 0 {
        return false;
    }
    loop {
        let a = e.mem.u8(own);
        if a != e.mem.u8(other) {
            return false;
        }
        if a == 0 {
            return true;
        }
        own = own.wrapping_add(1);
        other = other.wrapping_add(1);
    }
}

// Translated from 00a30640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the five handles at +0x00..+0x10: each set one is replaced by
/// the handle `00a5b690(null)` makes, and the old one's count is released
/// unless it is the handle at `0109b220`.
pub fn fn_00a30640(e: &mut Engine, this: Ptr) {
    for i in 0..5 {
        let slot = this.addr() + 4 * i;
        let old = e.mem.u32(slot);
        if old == 0 {
            continue;
        }
        let handle = e.call(MAKE_HANDLE, &args![0u32]).u32();
        e.mem.set_u32(slot, handle);
        if old != e.global::<u32>(NO_STRING_HANDLE) {
            e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
        }
    }
}

// Translated from 00a30700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the two reference-counted pointers at +4 and +0, in that
/// order: decrements each one's count and calls its `DeleteThis` (vtable
/// slot +0x04) when it reaches zero.
pub fn fn_00a30700(e: &mut Engine, this: Ptr) {
    for off in [4, 0] {
        let object = e.mem.u32(this.addr() + off);
        if object == 0 {
            continue;
        }
        let count = e
            .call(INTERLOCKED_DECREMENT, &args![object + REF_COUNT_OFFSET])
            .u32();
        if count == 0 {
            e.vcall(object, SLOT_DELETE_THIS, &args![]);
        }
    }
}

// Translated from 00a30750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the counts of the five handles at +0x10 down to +0x00 (no
/// null test: every handle but the one at `0109b220` is released).
pub fn fn_00a30750(e: &mut Engine, this: Ptr) {
    for i in (0..5).rev() {
        let handle = e.mem.u32(this.addr() + 4 * i);
        if handle != e.global::<u32>(NO_STRING_HANDLE) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a305e0, fn_00a305e0(Ptr, Ptr) -> bool),
        entry!(0x00a30640, fn_00a30640(Ptr)),
        entry!(0x00a30700, fn_00a30700(Ptr)),
        entry!(0x00a30750, fn_00a30750(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The decrement double decrements the word and returns the new value.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.map(0x0109_b000, 0x1000);
        e
    }

    fn string(e: &mut Engine, text: &[u8]) -> u32 {
        let p = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(p, text);
        p
    }

    #[test]
    fn string_equality() {
        let mut e = engine();
        let holder: Ptr = Ptr::new(e.mem.alloc(4));
        let own = string(&mut e, b"Bip01 Head");
        e.mem.set_u32(holder.addr(), own);
        let same = string(&mut e, b"Bip01 Head");
        let longer = string(&mut e, b"Bip01 Head2");
        let other = string(&mut e, b"Bip01 Neck");
        let eq = |e: &mut Engine, p: u32| e.call(0x00a3_05e0, &args![holder, p]).bool();
        assert!(eq(&mut e, own));
        assert!(eq(&mut e, same));
        assert!(!eq(&mut e, longer));
        assert!(!eq(&mut e, other));
        assert!(!eq(&mut e, 0));
        e.mem.set_u32(holder.addr(), 0);
        assert!(!eq(&mut e, same));
        assert!(eq(&mut e, 0));
    }

    #[test]
    fn reset_replaces_set_handles_and_releases_them() {
        let mut e = engine();
        e.register(MAKE_HANDLE, |_, _| 0x0009_0000u32.into_ret());
        let none = e.mem.alloc(16) + 8;
        e.set_global(NO_STRING_HANDLE, none);
        let counted = e.mem.alloc(16) + 8;
        e.mem.set_u32(counted - 8, 3);
        let this: Ptr = Ptr::new(e.mem.alloc(20));
        e.mem.set_u32(this.addr(), counted);
        e.mem.set_u32(this.addr() + 8, none);
        e.call_log = Some(vec![]);
        e.call(0x00a3_0640, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, MAKE_HANDLE).len(), 2);
        assert_eq!(e.mem.u32(this.addr()), 0x0009_0000);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        assert_eq!(e.mem.u32(this.addr() + 8), 0x0009_0000);
        assert_eq!(calls(&log, INTERLOCKED_DECREMENT), vec![vec![counted - 8]]);
        assert_eq!(e.mem.u32(counted - 8), 2);
    }

    #[test]
    fn release_two_pointers_deletes_at_zero() {
        let mut e = engine();
        e.register(0x0001_0000, |_, _| Ret::default());
        e.put_vtable(0x0002_0000, &[0, 0x0001_0000]);
        let this: Ptr = Ptr::new(e.mem.alloc(8));
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 0x0002_0000);
        e.mem.set_u32(first + 4, 1);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x0002_0000);
        e.mem.set_u32(second + 4, 2);
        e.mem.set_u32(this.addr(), first);
        e.mem.set_u32(this.addr() + 4, second);
        e.call_log = Some(vec![]);
        e.call(0x00a3_0700, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, INTERLOCKED_DECREMENT),
            vec![vec![second + 4], vec![first + 4]]
        );
        assert_eq!(calls(&log, 0x0001_0000), vec![vec![first]]);
    }

    #[test]
    fn release_five_handles_in_reverse_order() {
        let mut e = engine();
        let none = 0x0050_0008;
        e.set_global(NO_STRING_HANDLE, none);
        let this: Ptr = Ptr::new(e.mem.alloc(20));
        let mut handles = vec![];
        for i in 0..5 {
            let h = e.mem.alloc(16) + 8;
            e.mem.set_u32(h - 8, 10);
            e.mem.set_u32(this.addr() + 4 * i, h);
            handles.push(h);
        }
        e.mem.set_u32(this.addr() + 4, none);
        e.call_log = Some(vec![]);
        e.call(0x00a3_0750, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, INTERLOCKED_DECREMENT),
            vec![
                vec![handles[4] - 8],
                vec![handles[3] - 8],
                vec![handles[2] - 8],
                vec![handles[0] - 8]
            ]
        );
    }
}
