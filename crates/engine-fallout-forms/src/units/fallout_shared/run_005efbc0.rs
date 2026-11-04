//! `fallout shared/run_005efbc0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;
use crate::units::fallout_shared::run_005ef650::{construct_elements, resize_with};

// Translated from 005efbc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005efbc0 - the exception-unwinding frame (FS:[0]) around each element's construction is not translated.
/// Constructs `count` 6-byte grab IK entries from `elements` (`RET 8`;
/// `this` is not read; the engine map has no name): each placed by
/// `006e6da0(6, address)` and constructed by [`fn_005efc60`].
pub fn fn_005efbc0(e: &mut Engine, _this: Ptr, elements: u32, count: u32) {
    construct_elements(e, elements, count, 6, fn_005efc60);
}

// Translated from 005efc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a 6-byte grab IK entry (the engine map has no name):
/// the three `u16` at +0, +2, +4 set to 0xFFFF. Returns `this`.
pub fn fn_005efc60(e: &mut Engine, this: Ptr) -> Ptr {
    for offset in [0u32, 2, 4] {
        e.mem.set_u16(this.addr() + offset, 0xffff);
    }
    this
}

// Translated from 005efc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resizes the grab IK entry array's buffer (`RET 8`: capacity, count;
/// the engine map has no name), as `005ef860` does, moving the elements
/// with `005eff10`.
pub fn fn_005efc90(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, count: u32) {
    resize_with(e, this, capacity, count, 0x005e_ff10);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_fbc0, fn_005efbc0(Ptr, u32, u32)),
        entry!(0x005e_fc60, fn_005efc60(Ptr) -> Ptr),
        entry!(0x005e_fc90, fn_005efc90(Ptr<BSSimpleArray>, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn grab_entries_are_constructed_in_place() {
        let mut e = Engine::new();
        let block = e.mem.alloc(6 * 3);
        e.register_double(0x006e_6da0, move |_, a| {
            assert_eq!(a[0], 6);
            ret(if a[1] == block + 6 { 0 } else { a[1] })
        });
        e.call(0x005e_fbc0, &args![0u32, block, 3u32]);
        assert_eq!(e.mem.u16(block + 4), 0xffff);
        assert_eq!(e.mem.u16(block + 6), 0);
        assert_eq!(e.mem.u16(block + 16), 0xffff);
        let entry = e.mem.alloc(8);
        assert_eq!(e.call(0x005e_fc60, &args![entry]).u32(), entry);
        assert_eq!(e.mem.u32(entry + 4), 0x0000_ffff);
        assert_eq!(e.mem.u32(entry), 0xffff_ffff);
    }

    #[test]
    fn grab_resize_moves_with_005eff10() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x7000_0004);
        e.register(0x7000_0004, |_, a| ret(0x9000 + a[1]));
        e.register(0x005e_ff10, |_, _| Ret::default());
        e.register(0x006a_8500, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 4, 0x8000);
        e.mem.set_u32(this + 0xc, 4);
        e.call_log = Some(vec![]);
        e.call(0x005e_fc90, &args![this, 6u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x005e_ff10, vec![this, 0x9006, 0x8000, 2])));
        assert_eq!(e.mem.u32(this + 4), 0x9006);
    }
}
