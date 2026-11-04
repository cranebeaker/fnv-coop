//! `(unplaced)/run_00eea650` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The handle of the empty `NiFixedString` (the word at `0109b220`), which is never counted;
/// a counted handle's count is the word 8 bytes before it.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// Import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// The inlined `NiFixedString` assignment the four setters share: the new handle is the word
/// `source` points to; when it differs from the field's, the new handle's count is raised
/// (unless it is the empty handle), the old one's dropped (likewise), and the field set.
/// `[0109b220]` is read again for each test, as the exe does.
fn assign_fixed_string(e: &mut Engine, this: Ptr, offset: u32, source: Ptr) {
    let field = this.addr().wrapping_add(offset);
    let new = e.mem.u32(source.addr());
    if e.mem.u32(field) == new {
        return;
    }
    if new != e.mem.u32(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_sub(8)]);
    }
    let old = e.mem.u32(field);
    if old != e.mem.u32(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
    }
    e.mem.set_u32(field, new);
}

// Translated from 00eea650 (decompiled, FalloutNV.exe 1.4.0.525)
/// A string setter (unnamed in the map; `thiscall`, one word: the address of an
/// `NiFixedString`): assigns it to the handle at `this + 0xc` (`NiMaterialResource::
/// m_kSemantic` (Xbox PDB) on the resource the GPU program cache sets).
pub fn fn_00eea650(e: &mut Engine, this: Ptr, source: Ptr) {
    assign_fixed_string(e, this, 0xc, source);
}

// Translated from 00eea690 (decompiled, FalloutNV.exe 1.4.0.525)
/// A string setter (unnamed in the map; `thiscall`, one word: the address of an
/// `NiFixedString`): assigns it to the handle at `this + 0x10` (`NiMaterialResource::
/// m_kLabel` (Xbox PDB) on the resource the GPU program cache sets).
pub fn fn_00eea690(e: &mut Engine, this: Ptr, source: Ptr) {
    assign_fixed_string(e, this, 0x10, source);
}

// Translated from 00eea6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A string setter (unnamed in the map; `thiscall`, one word: the address of an
/// `NiFixedString`): assigns it to the handle at `this + 0x14` (`NiMaterialResource::
/// m_kVariable` (Xbox PDB) on the resource the GPU program cache sets).
pub fn fn_00eea6d0(e: &mut Engine, this: Ptr, source: Ptr) {
    assign_fixed_string(e, this, 0x14, source);
}

// Translated from 00eea710 (decompiled, FalloutNV.exe 1.4.0.525)
/// A string setter (unnamed in the map; `thiscall`, one word: the address of an
/// `NiFixedString`): assigns it to the handle at `this + 0x18` (`NiMaterialResource::
/// m_kDefaultValue` (Xbox PDB) on the resource the GPU program cache sets).
pub fn fn_00eea710(e: &mut Engine, this: Ptr, source: Ptr) {
    assign_fixed_string(e, this, 0x18, source);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee_a650, fn_00eea650(Ptr, Ptr)),
        entry!(0x00ee_a690, fn_00eea690(Ptr, Ptr)),
        entry!(0x00ee_a6d0, fn_00eea6d0(Ptr, Ptr)),
        entry!(0x00ee_a710, fn_00eea710(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: u32 = 0x0300_0008;

    /// An engine with the empty handle set and two counted strings (counts 5 and 9, handles
    /// at `0x0310_0008` and `0x0320_0008`).
    fn setup() -> Engine {
        let mut e = Engine::new();
        // The imports are registered only with the exe: counting doubles.
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
        e.map(EMPTY_FIXED_STRING & !0xfff, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, EMPTY);
        for (base, count) in [(0x0310_0000u32, 5u32), (0x0320_0000, 9)] {
            e.map(base, 0x1000);
            e.mem.set_u32(base, count);
        }
        e
    }

    /// Runs the setter at `addr` (field at `offset`) through its three paths.
    fn check(addr: u32, offset: u32) {
        let mut e = setup();
        let this = e.mem.alloc(0x2c);
        let source = e.mem.alloc(4);
        // Counted to counted: the new one raised, the old one dropped.
        e.mem.set_u32(this + offset, 0x0310_0008);
        e.mem.set_u32(source, 0x0320_0008);
        e.call(addr, &args![this, source]);
        assert_eq!(e.mem.u32(this + offset), 0x0320_0008);
        assert_eq!(e.mem.u32(0x0310_0000), 4);
        assert_eq!(e.mem.u32(0x0320_0000), 10);
        // Same handle: nothing changes.
        e.call(addr, &args![this, source]);
        assert_eq!(e.mem.u32(0x0320_0000), 10);
        // To the empty handle: only the old one is dropped.
        e.mem.set_u32(source, EMPTY);
        e.call_log = Some(vec![]);
        e.call(addr, &args![this, source]);
        assert_eq!(e.mem.u32(this + offset), EMPTY);
        assert_eq!(e.mem.u32(0x0320_0000), 9);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == INTERLOCKED_INCREMENT));
        // From the empty handle: only the new one is raised.
        e.mem.set_u32(source, 0x0310_0008);
        e.call_log = Some(vec![]);
        e.call(addr, &args![this, source]);
        assert_eq!(e.mem.u32(this + offset), 0x0310_0008);
        assert_eq!(e.mem.u32(0x0310_0000), 5);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == INTERLOCKED_DECREMENT));
    }

    #[test]
    fn fn_00eea650_assigns_the_handle_at_0xc() {
        check(0x00ee_a650, 0xc);
    }

    #[test]
    fn fn_00eea690_assigns_the_handle_at_0x10() {
        check(0x00ee_a690, 0x10);
    }

    #[test]
    fn fn_00eea6d0_assigns_the_handle_at_0x14() {
        check(0x00ee_a6d0, 0x14);
    }

    #[test]
    fn fn_00eea710_assigns_the_handle_at_0x18() {
        check(0x00ee_a710, 0x18);
    }
}
