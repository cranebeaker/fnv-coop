//! `fallout/ai/run_0098f840` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006e6da0` (cdecl `(size, address)`): the placement `operator new`, which
/// answers `address`.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `memmove` (`00ec7230`, cdecl `(destination, source, size)`).
const MEMMOVE: u32 = 0x00ec_7230;
/// The size of the array's entries.
const ENTRY_SIZE: u32 = 0x68;

// Translated from 0098f840 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0098f840 - the compiler's exception frame (FS:[0]) is not translated
/// Constructs `count` 0x68-byte entries at `entries` (placement
/// `006e6da0`, then `fn_0098f8e0` on each that came back non-null; `RET 8`).
pub fn fn_0098f840(e: &mut Engine, _this: Ptr, entries: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let at = index.wrapping_mul(ENTRY_SIZE).wrapping_add(entries);
        let block = e.call(PLACEMENT_NEW, &args![ENTRY_SIZE, at]).u32();
        if block != 0 {
            fn_0098f8e0(e, Ptr::new(block));
        }
        index += 1;
    }
}

// Translated from 0098f8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An entry's constructor: `0097ede0` on its members at +0x8, +0x18, +0x28
/// and +0x38 and `004300a0` on the words at +0x4C to +0x60. Returns `this`.
pub fn fn_0098f8e0(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    for offset in [0x8, 0x18, 0x28, 0x38] {
        e.call(0x0097_ede0, &args![at + offset]);
    }
    for offset in [0x4c, 0x50, 0x54, 0x58, 0x5c, 0x60] {
        e.call(0x0043_00a0, &args![at + offset]);
    }
    this
}

// Translated from 0098f960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `count` 0x68-byte entries from `source` to `destination`
/// (`memmove` per entry, `RET 0xC`): front to back when the destination is
/// lower, back to front when it is higher, nothing when they are the same
/// or `count` is 0. The back-to-front index is signed, as in the exe.
pub fn fn_0098f960(e: &mut Engine, _this: Ptr, destination: u32, source: u32, count: u32) {
    if count == 0 {
        return;
    }
    let entry = |base: u32, index: u32| index.wrapping_mul(ENTRY_SIZE).wrapping_add(base);
    if destination < source {
        let mut index = 0u32;
        while index < count {
            let to = entry(destination, index);
            let from = entry(source, index);
            e.call(MEMMOVE, &args![to, from, ENTRY_SIZE]);
            index += 1;
        }
    } else if destination > source {
        let mut index = count.wrapping_sub(1) as i32;
        while index >= 0 {
            let to = entry(destination, index as u32);
            let from = entry(source, index as u32);
            e.call(MEMMOVE, &args![to, from, ENTRY_SIZE]);
            index -= 1;
        }
    }
}

// Translated from 0098fa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the array `this` (allocator vtable at +0, buffer +4, capacity +0xC)
/// room for `capacity` entries (`RET 8`): without a buffer a new one (slot
/// 4) and the capacity; with one of `old_capacity` the buffer is resized in
/// place (slot 0xC with buffer and capacity); otherwise a new buffer takes
/// the `old_capacity` entries (`fn_0098f960`) and the old one is released
/// (`006a8500`).
pub fn fn_0098fa00(e: &mut Engine, this: Ptr, capacity: u32, old_capacity: u32) {
    let at = this.addr();
    if e.mem.u32(at + 4) == 0 {
        let buffer = e.vcall(at, 0x4, &args![capacity]).u32();
        e.mem.set_u32(at + 4, buffer);
        e.mem.set_u32(at + 0xc, capacity);
        return;
    }
    if old_capacity == e.mem.u32(at + 0xc) {
        let buffer = e.mem.u32(at + 4);
        let resized = e.vcall(at, 0xc, &args![buffer, capacity]).u32();
        e.mem.set_u32(at + 4, resized);
    } else {
        let buffer = e.vcall(at, 0x4, &args![capacity]).u32();
        let old = e.mem.u32(at + 4);
        fn_0098f960(e, this, buffer, old, old_capacity);
        e.call(0x006a_8500, &args![this]);
        e.mem.set_u32(at + 4, buffer);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0098_f840, fn_0098f840(Ptr, u32, u32)),
        entry!(0x0098_f8e0, fn_0098f8e0(Ptr) -> Ptr),
        entry!(0x0098_f960, fn_0098f960(Ptr, u32, u32, u32)),
        entry!(0x0098_fa00, fn_0098fa00(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod n028_tests {
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
    fn fn_0098f840_constructs_each_entry() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(0x0097_ede0, |_, _| Ret::default());
        e.register(0x0043_00a0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0098_f840, &[0, 0x1000, 2]);
        assert_eq!(
            calls_to(&e, PLACEMENT_NEW),
            vec![vec![0x68, 0x1000], vec![0x68, 0x1068]]
        );
        let members = calls_to(&e, 0x0097_ede0);
        assert_eq!(members.len(), 8);
        assert_eq!(members[4], vec![0x1070]);
        let words = calls_to(&e, 0x0043_00a0);
        assert_eq!(words.len(), 12);
        assert_eq!(words[11], vec![0x1068 + 0x60]);
    }

    #[test]
    fn fn_0098f960_moves_in_the_safe_direction() {
        let mut e = Engine::new();
        e.register(MEMMOVE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0098_f960, &[0, 0x1000, 0x2000, 2]);
        assert_eq!(
            calls_to(&e, MEMMOVE),
            vec![vec![0x1000, 0x2000, 0x68], vec![0x1068, 0x2068, 0x68]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0098_f960, &[0, 0x2000, 0x1000, 2]);
        assert_eq!(
            calls_to(&e, MEMMOVE),
            vec![vec![0x2068, 0x1068, 0x68], vec![0x2000, 0x1000, 0x68]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0098_f960, &[0, 0x2000, 0x2000, 2]);
        e.call(0x0098_f960, &[0, 0x1000, 0x2000, 0]);
        assert!(calls_to(&e, MEMMOVE).is_empty());
    }

    #[test]
    fn fn_0098fa00_allocates_resizes_or_moves() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table + 4, 0x7100_0004);
        e.mem.set_u32(table + 0xc, 0x7100_000c);
        e.mem.set_u32(this, table);
        e.register(0x7100_0004, |_, a| Ret {
            eax: 0x5000 + a[1],
            ..Ret::default()
        });
        e.register(0x7100_000c, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        e.register(MEMMOVE, |_, _| Ret::default());
        e.register(0x006a_8500, |_, _| Ret::default());
        e.call(0x0098_fa00, &[this, 4, 0]);
        assert_eq!(e.mem.u32(this + 4), 0x5004);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        e.call(0x0098_fa00, &[this, 8, 4]);
        assert_eq!(e.mem.u32(this + 4), 0x5005);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        e.call_log = Some(vec![]);
        e.call(0x0098_fa00, &[this, 8, 2]);
        assert_eq!(e.mem.u32(this + 4), 0x5008);
        assert_eq!(calls_to(&e, MEMMOVE)[0], vec![0x5070, 0x506d, 0x68]);
        assert_eq!(calls_to(&e, 0x006a_8500), vec![vec![this]]);
    }
}
