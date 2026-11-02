//! `(unplaced)/run_0069d050` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0069d050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` and one stack word, `RET 4`): copies this object's
/// array of 6-byte entries (pointer at +4, count at +8) into `destination`. First
/// `0069cd60` on `destination` with the count and 1, then for each index the 4-byte word
/// and the 2-byte half of the entry go to the address `0069ccf0` on `destination` gives for
/// that index. The count is read again on each pass.
pub fn fn_0069d050(e: &mut Engine, this: Ptr, destination: Ptr) {
    let base = this.addr();
    let count = e.mem.u32(base + 8);
    e.call(0x0069_cd60, &args![destination, count, 1u32]);
    let mut index = 0u32;
    while index < e.mem.u32(base + 8) {
        let source = e.mem.u32(base + 4).wrapping_add(index.wrapping_mul(6));
        let target = e.call(0x0069_ccf0, &args![destination, index]).u32();
        let word = e.mem.u32(source);
        e.mem.set_u32(target, word);
        let half = e.mem.u16(source.wrapping_add(4));
        e.mem.set_u16(target.wrapping_add(4), half);
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0069_d050, fn_0069d050(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_0069d050_copies_each_six_byte_entry() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0xc);
        let entries = e.mem.alloc(18);
        for (index, byte) in (0..18u32).enumerate() {
            e.mem.set_u8(entries + index as u32, 0x10 + byte as u8);
        }
        e.mem.set_u32(object + 4, entries);
        e.mem.set_u32(object + 8, 3);
        let destination = e.mem.alloc(0x40);
        let slots = e.mem.alloc(3 * 8);
        e.register(0x0069_cd60, |_, _| Ret::default());
        e.register_double(0x0069_ccf0, move |_, a| Ret {
            eax: slots + 8 * a[1],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0069_d050, &args![object, destination]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0069_cd60, vec![destination, 3, 1]));
        assert_eq!(log.len(), 5);
        for index in 0..3u32 {
            assert_eq!(
                log[2 + index as usize],
                (0x0069_ccf0, vec![destination, index])
            );
            assert_eq!(e.mem.bytes(slots + 8 * index, 8), {
                let mut expected: Vec<u8> = (0..6).map(|k| 0x10 + (6 * index + k) as u8).collect();
                expected.extend([0, 0]);
                expected
            });
        }
    }
}
