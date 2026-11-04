//! `(unplaced)/run_006c62d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006e5cc0` (thiscall, one word): `NiPointer` assignment (the engine map
/// names this instance `NiPointer<PathingDebugGeometryData>::operator=`).
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;

// Translated from 006c62d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: an `NiTMap` lookup whose values are
/// `NiPointer`s. Hashes `key` (vtable +4), walks the bucket's chain in the
/// table at `this + 8` (item: next at +0, key at +4, value at +8) comparing
/// keys (vtable +8); on a match assigns the item's value to the `NiPointer`
/// at `value_out` (`006e5cc0`) and returns true, otherwise false.
pub fn fn_006c62d0(e: &mut Engine, this: Ptr, key: u32, value_out: Ptr) -> bool {
    let t = this.addr();
    let bucket = e.vcall(t, 4, &args![key]).u32();
    let table = e.mem.u32(t.wrapping_add(8));
    let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u32(item.wrapping_add(4));
        if e.vcall(t, 8, &args![key, item_key]).bool() {
            e.call(NI_POINTER_ASSIGN, &args![value_out, item.wrapping_add(8)]);
            return true;
        }
        item = e.mem.u32(item);
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006c_62d0, fn_006c62d0(Ptr, u32, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_006c62d0_finds_the_key_in_its_bucket_chain() {
        let mut e = Engine::new();
        // Hash: key % 4; equality: plain comparison.
        e.register(0x7000_0004, |_, a| Ret {
            eax: a[1] % 4,
            ..Ret::default()
        });
        e.register(0x7000_0008, |_, a| Ret {
            eax: (a[1] == a[2]) as u32,
            ..Ret::default()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.put_vtable(0x9100_0000, &[0, 0x7000_0004, 0x7000_0008]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x9100_0000);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, table);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(second + 4, 9);
        e.mem.set_u32(second + 8, 0x0999);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 5);
        e.mem.set_u32(first + 8, 0x0555);
        e.mem.set_u32(table + 4, first);
        let out = e.mem.alloc(4);
        assert!(e.call(0x006c_62d0, &args![this, 9u32, out]).bool());
        assert_eq!(e.mem.u32(out), 0x0999);
        assert!(e.call(0x006c_62d0, &args![this, 5u32, out]).bool());
        assert_eq!(e.mem.u32(out), 0x0555);
        e.mem.set_u32(out, 0);
        assert!(!e.call(0x006c_62d0, &args![this, 13u32, out]).bool());
        assert!(!e.call(0x006c_62d0, &args![this, 6u32, out]).bool());
        assert_eq!(e.mem.u32(out), 0);
    }
}
