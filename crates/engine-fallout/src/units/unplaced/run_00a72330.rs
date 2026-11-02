//! `(unplaced)/run_00a72330` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a72330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends the component sizes of a data type (thiscall; called by
/// `00a728d0` in `NiMain/niadditionalgeometrydata.cpp`; that `this` is an
/// `NiAdditionalGeometryData::NiAGDDataStream`, `m_uiType` at +4, is not
/// confirmed): for the type code at +4, writes its component count of words
/// into `out` at `*index`, each the component size, advancing `*index` once
/// per word. Codes 1 to 12 have 1 to 4 components of 4 bytes (repeating
/// every 4 codes), 13 to 20 1 to 4 of 2 bytes, 21 to 32 1 to 4 of 1 byte;
/// other codes write nothing. The third stack word is not read. The exe's
/// byte map and jump table are the `match` arms here.
pub fn fn_00a72330(e: &mut Engine, this: u32, out: u32, index: u32, _unused_3: u32) {
    let (count, size) = match e.mem.u32(this + 4) {
        1 | 5 | 9 => (1, 4),
        2 | 6 | 10 => (2, 4),
        3 | 7 | 11 => (3, 4),
        4 | 8 | 12 => (4, 4),
        13 | 17 => (1, 2),
        14 | 18 => (2, 2),
        15 | 19 => (3, 2),
        16 | 20 => (4, 2),
        21 | 25 | 29 => (1, 1),
        22 | 26 | 30 => (2, 1),
        23 | 27 | 31 => (3, 1),
        24 | 28 | 32 => (4, 1),
        _ => (0, 0),
    };
    for _ in 0..count {
        let at = e.mem.u32(index);
        e.mem.set_u32(out.wrapping_add(at.wrapping_mul(4)), size);
        e.mem.set_u32(index, at.wrapping_add(1));
    }
}

// Translated from 00a72440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two data blocks hold the same bytes (thiscall; called by
/// `NiAdditionalGeometryData::IsEqual`, `00a727f0`; the Xbox PDB's
/// `NiAGDDataBlock` has `m_uiDataBlockSize` at +4 and `m_pucDataBlock` at
/// +8, which this matches but which is not confirmed here): false when the
/// sizes (+4) differ; otherwise the inlined `memcmp` of the two buffers
/// (+8), four bytes at a time and then byte by byte, equal.
pub fn fn_00a72440(e: &mut Engine, this: u32, other: u32) -> bool {
    let size = e.mem.u32(this + 4);
    if size != e.mem.u32(other + 4) {
        return false;
    }
    let mut left = e.mem.u32(this + 8);
    let mut right = e.mem.u32(other + 8);
    let mut remaining = size;
    while remaining >= 4 {
        if e.mem.u32(left) != e.mem.u32(right) {
            break;
        }
        remaining -= 4;
        left = left.wrapping_add(4);
        right = right.wrapping_add(4);
    }
    if remaining == 0 {
        return true;
    }
    // The tail (or the first differing word) byte by byte.
    for offset in 0..3u32 {
        if e.mem.u8(left.wrapping_add(offset)) != e.mem.u8(right.wrapping_add(offset)) {
            return false;
        }
        if remaining <= offset + 1 {
            return true;
        }
    }
    // The fourth byte is only reached inside a differing word: the code
    // reads it and answers false whatever it holds (`SAR`/`OR 1` is never
    // zero).
    let _fourth = (
        e.mem.u8(left.wrapping_add(3)),
        e.mem.u8(right.wrapping_add(3)),
    );
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_2330, fn_00a72330(u32, u32, u32, u32)),
        entry!(0x00a7_2440, fn_00a72440(u32, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The words type `code` appends after two already there.
    fn sizes(code: u32) -> Vec<u32> {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, code);
        let out = e.mem.alloc(0x20);
        let index = e.mem.alloc(4);
        e.mem.set_u32(index, 2);
        e.call(0x00a7_2330, &[this, out, index, 0]);
        let n = e.mem.u32(index);
        (2..n).map(|i| e.mem.u32(out + 4 * i)).collect()
    }

    #[test]
    fn type_codes_give_their_component_sizes() {
        assert_eq!(sizes(1), vec![4]);
        assert_eq!(sizes(8), vec![4; 4]);
        assert_eq!(sizes(11), vec![4; 3]);
        assert_eq!(sizes(13), vec![2]);
        assert_eq!(sizes(18), vec![2; 2]);
        assert_eq!(sizes(21), vec![1]);
        assert_eq!(sizes(31), vec![1; 3]);
        assert_eq!(sizes(32), vec![1; 4]);
        assert_eq!(sizes(0), Vec::<u32>::new());
        assert_eq!(sizes(33), Vec::<u32>::new());
    }

    #[test]
    fn blocks_compare_size_then_bytes() {
        let mut e = Engine::new();
        let a = e.mem.alloc(12);
        let b = e.mem.alloc(12);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.write(first, &[1, 2, 3, 4, 5, 6, 7, 8]);
        e.mem.write(second, &[1, 2, 3, 4, 5, 6, 7, 9]);
        e.mem.set_u32(a + 8, first);
        e.mem.set_u32(b + 8, second);
        for (size_a, size_b, equal) in [
            (7u32, 7u32, true),
            (8, 8, false),
            (4, 4, true),
            (0, 0, true),
            (7, 8, false),
        ] {
            e.mem.set_u32(a + 4, size_a);
            e.mem.set_u32(b + 4, size_b);
            assert_eq!(e.call(0x00a7_2440, &[a, b]).bool(), equal, "{size_a}");
        }
        // A difference in the first word is found byte by byte.
        e.mem.set_u8(second + 1, 0);
        e.mem.set_u32(a + 4, 8);
        e.mem.set_u32(b + 4, 8);
        assert!(!e.call(0x00a7_2440, &[a, b]).bool());
    }
}
