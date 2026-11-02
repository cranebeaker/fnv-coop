//! `Havok SDK/run_00cf4f00` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The bit set is the list shape's `m_enabledChildren` (its words start at `+0x50`; see
//! `run_00cf5070.rs` for the shape): the 16-bit word at `+0x26` is raised by one when a bit is
//! cleared and lowered by one when a bit is set, and `00cf4f70` sets it to the word at `+0x1c`
//! (the child count) minus the number of set bits. The class is unconfirmed.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00d86680` (cdecl `(words, count)`): returns the number of set bits in `count` words.
/// Named by neither the map nor Ghidra; its body is a population count of the words.
const COUNT_SET_BITS: u32 = 0x00d8_6680;

// Translated from 00cf4f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears bit `index` (word `index >> 5` of the words at `+0x50`, bit `index & 31`) and, when it
/// was set, adds 1 to the 16-bit count at `+0x26`.
pub fn fn_00cf4f00(e: &mut Engine, this: Ptr, index: u32) {
    let word_at = this.addr() + 0x50 + (index >> 5) * 4;
    let before = e.mem.u32(word_at);
    let after = !(1u32 << (index & 0x1f)) & before;
    if before != after {
        e.mem.set_u32(word_at, after);
        let count = e.mem.u16(this.addr() + 0x26);
        e.mem.set_u16(this.addr() + 0x26, count.wrapping_add(1));
    }
}

// Translated from 00cf4f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets bit `index` (as `00cf4f00` addresses it) and, when it was clear, subtracts 1 from the
/// 16-bit count at `+0x26` (the exe adds `0xffff`).
pub fn fn_00cf4f30(e: &mut Engine, this: Ptr, index: u32) {
    let word_at = this.addr() + 0x50 + (index >> 5) * 4;
    let before = e.mem.u32(word_at);
    let after = (1u32 << (index & 0x1f)) | before;
    if before != after {
        e.mem.set_u32(word_at, after);
        let count = e.mem.u16(this.addr() + 0x26);
        e.mem
            .set_u16(this.addr() + 0x26, count.wrapping_add(0xffff));
    }
}

// Translated from 00cf4f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the words of an array (`source`: data `+0`, count `+4`, signed) to the words at `+0x50`
/// and sets the 16-bit count at `+0x26` to the bit count at `+0x1c` minus the number of set
/// bits in them (`00d86680(data, count)`, whose result's low word is used).
pub fn fn_00cf4f70(e: &mut Engine, this: Ptr, source: Ptr) {
    let base = this.addr();
    let source = source.addr();
    let mut index = 0;
    if e.mem.i32(source + 4) > 0 {
        let mut target = base + 0x50;
        loop {
            let data = e.mem.u32(source);
            let word = e.mem.u32(data.wrapping_add(index * 4));
            e.mem.set_u32(target, word);
            index += 1;
            target += 4;
            if index as i32 >= e.mem.i32(source + 4) {
                break;
            }
        }
    }
    let count = e.mem.u32(source + 4);
    let data = e.mem.u32(source);
    let total = e.mem.u32(base + 0x1c);
    let set = e.call(COUNT_SET_BITS, &args![data, count]).u32();
    e.mem.set_u16(base + 0x26, total.wrapping_sub(set) as u16);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_4f00, fn_00cf4f00(Ptr, u32)),
        entry!(0x00cf_4f30, fn_00cf4f30(Ptr, u32)),
        entry!(0x00cf_4f70, fn_00cf4f70(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Clearing a set bit raises the count; clearing a clear bit does nothing; setting a clear
    /// bit lowers it (also below zero, wrapping in 16 bits).
    #[test]
    fn bits_change_the_count_only_when_they_flip() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x50, 0b1010);
        e.mem.set_u32(this + 0x54, 1 << 31);
        e.mem.set_u16(this + 0x26, 5);
        e.call(0x00cf_4f00, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this + 0x50), 0b1000);
        assert_eq!(e.mem.u16(this + 0x26), 6);
        e.call(0x00cf_4f00, &args![this, 1u32]);
        assert_eq!(e.mem.u16(this + 0x26), 6);
        e.call(0x00cf_4f00, &args![this, 63u32]);
        assert_eq!(e.mem.u32(this + 0x54), 0);
        assert_eq!(e.mem.u16(this + 0x26), 7);
        e.call(0x00cf_4f30, &args![this, 2u32]);
        assert_eq!(e.mem.u32(this + 0x50), 0b1100);
        assert_eq!(e.mem.u16(this + 0x26), 6);
        e.call(0x00cf_4f30, &args![this, 2u32]);
        assert_eq!(e.mem.u16(this + 0x26), 6);
        // The index is taken modulo 32 within its word: 32 + 5 is bit 5 of the second word.
        e.mem.set_u16(this + 0x26, 0);
        e.call(0x00cf_4f30, &args![this, 37u32]);
        assert_eq!(e.mem.u32(this + 0x54), 0b10_0000);
        assert_eq!(e.mem.u16(this + 0x26), 0xffff);
    }

    /// The words are copied from the array and the count is the total minus the set bits.
    #[test]
    fn copying_the_words_recounts_the_clear_bits() {
        let mut e = Engine::new();
        e.register_double(COUNT_SET_BITS, |e, a| {
            let set: u32 = (0..a[1])
                .map(|i| e.mem.u32(a[0] + 4 * i).count_ones())
                .sum();
            Ret {
                eax: set,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x1c, 40);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, 0xff);
        e.mem.set_u32(data + 4, 0x0f);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 2);
        e.call(0x00cf_4f70, &args![this, array]);
        assert_eq!(e.mem.u32(this + 0x50), 0xff);
        assert_eq!(e.mem.u32(this + 0x54), 0x0f);
        assert_eq!(e.mem.u16(this + 0x26), 40 - 12);
        // An empty array copies nothing and the count is the total.
        e.mem.set_u32(array + 4, 0);
        e.call(0x00cf_4f70, &args![this, array]);
        assert_eq!(e.mem.u16(this + 0x26), 40);
    }
}
