//! `Havok SDK/run_00ce9840` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ce9840 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(destination, source, count)`: for `count` (signed) records of `0x30` bytes, copies the
/// 16 bytes at `+0x20` of each source record over the same bytes of the destination record.
pub fn fn_00ce9840(e: &mut Engine, destination: Ptr, source: Ptr, count: i32) {
    let mut remaining = count;
    if remaining > 0 {
        let mut to = destination.addr().wrapping_add(0x20);
        let mut from = source.addr().wrapping_add(0x20);
        loop {
            for lane in 0..4 {
                let word = e.mem.u32(from.wrapping_add(4 * lane));
                e.mem.set_u32(to.wrapping_add(4 * lane), word);
            }
            to = to.wrapping_add(0x30);
            from = from.wrapping_add(0x30);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ce_9840, fn_00ce9840(Ptr, Ptr, i32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Only the last vector of each record is copied; a count of zero or less copies nothing.
    #[test]
    fn copies_the_third_vector_of_each_record() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0x30 * 3);
        let destination = e.mem.alloc(0x30 * 3);
        for i in 0..36 {
            e.mem.set_u32(source + 4 * i, 0x100 + i);
            e.mem.set_u32(destination + 4 * i, 0x900 + i);
        }
        e.call(0x00ce_9840, &args![destination, source, 2i32]);
        for record in 0..3u32 {
            for word in 0..12u32 {
                let at = destination + 0x30 * record + 4 * word;
                let copied = record < 2 && (8..12).contains(&word);
                let expected = if copied {
                    0x100 + 12 * record + word
                } else {
                    0x900 + 12 * record + word
                };
                assert_eq!(e.mem.u32(at), expected, "record {record} word {word}");
            }
        }
        for count in [0i32, -1] {
            e.mem.set_u32(destination + 0x20, 0x5555);
            e.call(0x00ce_9840, &args![destination, source, count]);
            assert_eq!(e.mem.u32(destination + 0x20), 0x5555);
        }
    }
}
