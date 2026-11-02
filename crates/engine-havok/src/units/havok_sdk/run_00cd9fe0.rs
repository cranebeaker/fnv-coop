//! `Havok SDK/run_00cd9fe0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cd9fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets a table of words to 1 and flags the object (the class is unconfirmed): the count is
/// the word at `+0x1c` of the object `this` points to (`[this] + 0x1c`, signed); that many
/// words of the array whose data pointer is at `this + 0x1c` (read again for every word) are
/// set to 1; then the byte at `+0x29` is cleared and the byte at `+0x28` set. Returns
/// `this + 0x10`.
pub fn fn_00cd9fe0(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let owner = e.mem.u32(base);
    let count = e.mem.i32(owner + 0x1c);
    let mut index = 0;
    if count > 0 {
        loop {
            let data = e.mem.u32(base + 0x1c);
            e.mem.set_u32(data.wrapping_add(index * 4), 1);
            index += 1;
            if index as i32 >= count {
                break;
            }
        }
    }
    e.mem.set_u8(base + 0x29, 0);
    e.mem.set_u8(base + 0x28, 1);
    base + 0x10
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cd_9fe0, fn_00cd9fe0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The words are all set to 1, the two bytes written and `this + 0x10` returned; a count
    /// of zero or less sets no word.
    #[test]
    fn sets_every_word_to_one_and_flags_the_object() {
        let mut e = Engine::new();
        let owner = e.mem.alloc(0x40);
        let table = e.mem.alloc(0x40);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, owner);
        e.mem.set_u32(this + 0x1c, table);
        e.mem.set_u8(this + 0x29, 7);
        for i in 0..4 {
            e.mem.set_u32(table + 4 * i, 0xaaaa_aaaa);
        }
        e.mem.set_u32(owner + 0x1c, 3);
        let got = e.call(0x00cd_9fe0, &args![this]).u32();
        assert_eq!(got, this + 0x10);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(table + 4 * i)).collect();
        assert_eq!(words, vec![1, 1, 1, 0xaaaa_aaaa]);
        assert_eq!(e.mem.u8(this + 0x29), 0);
        assert_eq!(e.mem.u8(this + 0x28), 1);
        for count in [0u32, 0xffff_fffe] {
            e.mem.set_u32(table, 0xbbbb_bbbb);
            e.mem.set_u32(owner + 0x1c, count);
            e.call(0x00cd_9fe0, &args![this]);
            assert_eq!(e.mem.u32(table), 0xbbbb_bbbb);
        }
    }
}
