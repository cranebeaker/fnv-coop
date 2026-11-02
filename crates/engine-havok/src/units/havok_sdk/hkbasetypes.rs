//! `Havok SDK/hkbasetypes.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ca9360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkUFloat8::operator=` (Xbox PDB name from the engine map, the `=` shown as `_` there): `this`
/// in `ECX` and one stack word, `value`, the address of a float (`RET 4`). Reads the float's bits as
/// a signed integer: below `0x3c23d70a` (0.01) the byte at `this` becomes 0; otherwise a search
/// over the table of 256 integers at `010c7948`..`010c7d48` (whose last entry is at `010c7d44`)
/// starts in its second half (from `010c7b48`) when the value is above `0x4358cccd` and in its
/// first half otherwise, halves the range six times (the middle is `(low >> 1) + (high >> 1)`;
/// a value above the entry at the middle moves the low end there, any other moves the high end),
/// takes the middle once more, steps to the next entry when the value is above that one and the
/// middle is below the last entry's address, and stores the entry's index in the byte at `this`.
/// Returns `this`.
pub fn hk_ufloat8_operator_assign(e: &mut Engine, this: u32, value: u32) -> u32 {
    let bits = e.mem.i32(value);
    if bits < 0x3c23_d70a {
        e.mem.set_u8(this, 0);
        return this;
    }
    let (mut low, mut high) = if bits > 0x4358_cccd {
        (0x010c_7b48u32, 0x010c_7d48u32)
    } else {
        (0x010c_7948u32, 0x010c_7b48u32)
    };
    for _ in 0..6 {
        let middle = (high >> 1).wrapping_add(low >> 1);
        if bits > e.mem.i32(middle) {
            low = middle;
        } else {
            high = middle;
        }
    }
    let mut middle = (high >> 1).wrapping_add(low >> 1);
    if bits > e.mem.i32(middle) && middle < 0x010c_7d44 {
        middle = middle.wrapping_add(4);
    }
    e.mem
        .set_u8(this, (middle.wrapping_sub(0x010c_7948) as i32 >> 2) as u8);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ca_9360, hk_ufloat8_operator_assign(u32, u32) -> u32)]
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;

    /// Every table entry is `ENTRY_STEP` above the one before, starting at the first threshold; the
    /// entry at index 128 is then just below the second threshold, 0x4358cccd.
    const ENTRY_STEP: u32 = 0xe_69eb;

    fn table_entry(index: u32) -> u32 {
        0x3c23_d70a + ENTRY_STEP * index
    }

    /// The table at 010c7948: 256 ascending integers, set in mapped pages.
    fn ufloat_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x010c_7000, 0x2000);
        for index in 0..256u32 {
            e.mem.set_u32(0x010c_7948 + 4 * index, table_entry(index));
        }
        e
    }

    fn assign(e: &mut Engine, bits: u32) -> u8 {
        let this = e.mem.alloc(8);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, bits);
        e.mem.set_u8(this, 0x77);
        assert_eq!(e.call(0x00ca_9360, &args![this, value]).u32(), this);
        e.mem.u8(this)
    }

    /// Values below 0.01 give 0 without searching (also negative floats: a negative integer).
    #[test]
    fn hk_ufloat8_operator_assign_gives_zero_below_the_first_threshold() {
        let mut e = ufloat_engine();
        assert_eq!(assign(&mut e, 0x3c23_d709), 0);
        assert_eq!(assign(&mut e, 0xbf80_0000), 0);
        assert_eq!(assign(&mut e, 0), 0);
    }

    /// Between two entries the answer is the index of the first entry that is not below the value, in
    /// both halves of the table (the second one is searched for values above 0x4358cccd).
    #[test]
    fn hk_ufloat8_operator_assign_finds_the_first_entry_not_below_the_value() {
        let mut e = ufloat_engine();
        assert!(table_entry(128) < 0x4358_cccd && table_entry(129) - 0x1_0000 > 0x4358_cccd);
        for index in [2u32, 3, 17, 100, 127, 128, 129, 200, 254] {
            let entry = table_entry(index);
            // Just above the entry before: the entry itself.
            assert_eq!(
                u32::from(assign(&mut e, entry - 0x1_0000)),
                index,
                "below entry {index}"
            );
            // Exactly the entry: also that entry.
            assert_eq!(u32::from(assign(&mut e, entry)), index, "at entry {index}");
        }
    }

    /// Above the last entry the index stops at 255 (the middle is not below the last entry's address).
    #[test]
    fn hk_ufloat8_operator_assign_stops_at_the_last_entry() {
        let mut e = ufloat_engine();
        assert_eq!(assign(&mut e, 0x7f00_0000), 255);
        assert_eq!(assign(&mut e, table_entry(255)), 255);
    }
}
