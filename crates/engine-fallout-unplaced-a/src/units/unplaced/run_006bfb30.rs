//! `(unplaced)/run_006bfb30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006bfb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (cdecl, two pointers; passed as a sort comparison by `006be950`,
/// `UndoNavMeshFromObstacleUndoData` and `BuildSearchTeleportDoorArray`): compares the
/// words they point to, unsigned: 1 when the first is greater, -1 when it is less, else 0.
pub fn fn_006bfb30(e: &mut Engine, first: u32, second: u32) -> i32 {
    let (a, b) = (e.mem.u32(first), e.mem.u32(second));
    if a > b {
        1
    } else if b > a {
        -1
    } else {
        0
    }
}

// Translated from 006bfb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (cdecl, two pointers; passed as an equality test by `006be950` and
/// `UndoNavMeshFromObstacleUndoData`): [`fn_006bfb90`] on `first` with `second`.
pub fn fn_006bfb70(e: &mut Engine, first: u32, second: u32) -> u8 {
    fn_006bfb90(e, Ptr::new(first), Ptr::new(second))
}

// Translated from 006bfb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 4`): whether two 0xC-byte handles are equal (by
/// its body: the words at +0 and +8 and the 16-bit value at +4; the Xbox PDB's
/// `NavMeshEdgeHandle` is 0xC bytes, not confirmed as this type).
pub fn fn_006bfb90(e: &mut Engine, this: Ptr, other: Ptr) -> u8 {
    let (a, b) = (this.addr(), other.addr());
    let equal = e.mem.u32(a) == e.mem.u32(b)
        && e.mem.u16(a + 4) == e.mem.u16(b + 4)
        && e.mem.u32(a + 8) == e.mem.u32(b + 8);
    u8::from(equal)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b_fb30, fn_006bfb30(u32, u32) -> i32),
        entry!(0x006b_fb70, fn_006bfb70(u32, u32) -> u8),
        entry!(0x006b_fb90, fn_006bfb90(Ptr, Ptr) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparison_orders_unsigned_words() {
        let mut e = Engine::new();
        let a = e.mem.alloc(4);
        let b = e.mem.alloc(4);
        e.mem.set_u32(a, 0xffff_0000);
        e.mem.set_u32(b, 1);
        assert_eq!(e.call(0x006b_fb30, &args![a, b]).u32() as i32, 1);
        assert_eq!(e.call(0x006b_fb30, &args![b, a]).u32() as i32, -1);
        assert_eq!(e.call(0x006b_fb30, &args![a, a]).u32(), 0);
    }

    #[test]
    fn handles_are_equal_in_all_three_parts() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0xc);
        let b = e.mem.alloc(0xc);
        for (at, value) in [(0u32, 7u32), (8, 9)] {
            e.mem.set_u32(a + at, value);
            e.mem.set_u32(b + at, value);
        }
        e.mem.set_u32(a + 4, 0x0001_0005);
        e.mem.set_u32(b + 4, 0x0002_0005);
        assert_eq!(e.call(0x006b_fb70, &args![a, b]).u8(), 1);
        e.mem.set_u16(b + 4, 6);
        assert_eq!(e.call(0x006b_fb90, &args![a, b]).u8(), 0);
        e.mem.set_u16(b + 4, 5);
        e.mem.set_u32(b + 8, 1);
        assert_eq!(e.call(0x006b_fb90, &args![a, b]).u8(), 0);
    }
}
