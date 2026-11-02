//! `Havok SDK/hkxvertexdescription.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e3dee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxVertexDescription::operator_` (Xbox PDB, the comparison operator): two descriptions are
/// equal when their element arrays (`this`: data `+0`, size `+4`; elements are `0xc` bytes)
/// have the same size and every element matches in the word at `+0`, the 16-bit words at `+4`
/// and `+6` and the word at `+8`. Returns false at the first difference, true when all match
/// (also for two empty arrays).
pub fn hkx_vertex_description_operator_eq(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let (this, other) = (this.addr(), other.addr());
    let size = e.mem.i32(this + 4);
    if size != e.mem.i32(other + 4) {
        return false;
    }
    let mut index = 0;
    if size > 0 {
        let mut mine = e.mem.u32(this);
        let mut theirs = e.mem.u32(other);
        loop {
            if e.mem.u16(mine + 4) != e.mem.u16(theirs + 4)
                || e.mem.u16(mine + 6) != e.mem.u16(theirs + 6)
                || e.mem.u32(mine) != e.mem.u32(theirs)
                || e.mem.u32(mine + 8) != e.mem.u32(theirs + 8)
            {
                return false;
            }
            index += 1;
            theirs = theirs.wrapping_add(0xc);
            mine = mine.wrapping_add(0xc);
            if index >= size {
                break;
            }
        }
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e3_dee0,
        hkx_vertex_description_operator_eq(Ptr, Ptr) -> bool
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A description with the given elements: (offset, type, usage, stride).
    fn description(e: &mut Engine, elements: &[(u32, u16, u16, u32)]) -> u32 {
        let this = e.mem.alloc(0x10);
        let data = e.mem.alloc(0xc * elements.len().max(1) as u32);
        for (i, &(offset, kind, usage, stride)) in elements.iter().enumerate() {
            let at = data + 0xc * i as u32;
            e.mem.set_u32(at, offset);
            e.mem.set_u16(at + 4, kind);
            e.mem.set_u16(at + 6, usage);
            e.mem.set_u32(at + 8, stride);
        }
        e.mem.set_u32(this, data);
        e.mem.set_u32(this + 4, elements.len() as u32);
        this
    }

    fn equal(e: &mut Engine, a: u32, b: u32) -> bool {
        e.call(0x00e3_dee0, &args![a, b]).bool()
    }

    /// Different sizes differ, empty arrays are equal, and a difference in any one of the
    /// four compared fields of any element makes the descriptions unequal.
    #[test]
    fn equality_compares_size_and_every_field() {
        let mut e = Engine::new();
        let base = [(0, 6, 1, 16), (16, 4, 2, 4)];
        let a = description(&mut e, &base);
        let same = description(&mut e, &base);
        assert!(equal(&mut e, a, same));
        let shorter = description(&mut e, &base[..1]);
        assert!(!equal(&mut e, a, shorter));
        let empty_a = description(&mut e, &[]);
        let empty_b = description(&mut e, &[]);
        assert!(equal(&mut e, empty_a, empty_b));
        for (index, field) in [(0, 0), (1, 1), (1, 2), (0, 3)] {
            let mut changed = base;
            match field {
                0 => changed[index].0 += 4,
                1 => changed[index].1 += 1,
                2 => changed[index].2 += 1,
                _ => changed[index].3 += 4,
            }
            let other = description(&mut e, &changed);
            assert!(!equal(&mut e, a, other), "field {field} of element {index}");
        }
    }
}
