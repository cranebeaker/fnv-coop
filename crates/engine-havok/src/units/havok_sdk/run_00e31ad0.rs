//! `Havok SDK/run_00e31ad0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e31ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: appends the 16-bit `value` to the byte `hkArray` at `array`:
/// `00e31800(2, array)` adds two zero bytes, then the last two bytes (at data + size - 2) are
/// overwritten with `value`. The function takes two stack words (the first read as a word) and
/// pops none (`RET`) and returns nothing.
pub fn fn_00e31ad0(e: &mut Engine, value: u16, array: Ptr) {
    e.call(0x00e3_1800, &args![2u32, array]);
    let size = e.mem.u32(array.addr() + 4);
    let data = e.mem.u32(array.addr());
    e.mem
        .set_u16(size.wrapping_add(data).wrapping_sub(2), value);
}

// Translated from 00e31b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: appends the float `value` to the byte `hkArray` at `array`:
/// `00e31800(4, array)` adds four zero bytes, then the last four bytes (at data + size - 4) are
/// overwritten with the float, which goes through the x87 stack (`FLD`, `FSTP`: a signalling
/// NaN is stored quiet). The function takes two stack words and pops none (`RET`) and returns
/// nothing.
pub fn fn_00e31b00(e: &mut Engine, value: f32, array: Ptr) {
    e.call(0x00e3_1800, &args![4u32, array]);
    let bits = x87_quiet_bits(value.to_bits());
    let size = e.mem.u32(array.addr() + 4);
    let data = e.mem.u32(array.addr());
    e.mem.set_u32(size.wrapping_add(data).wrapping_sub(4), bits);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_1ad0, fn_00e31ad0(u16, Ptr)),
        entry!(0x00e3_1b00, fn_00e31b00(f32, Ptr)),
    ]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// A byte array with 8 bytes of storage; the `00e31800` double adds `count` bytes by
    /// raising the size (the storage is already zero).
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register_double(0x00e3_1800, |e, a| {
            let size = e.mem.u32(a[1] + 4);
            e.mem.set_u32(a[1] + 4, size + a[0]);
            Ret::default()
        });
        let array = e.mem.alloc(0x10);
        let storage = e.mem.alloc(0x20);
        e.mem.write(storage, &[0xee; 0x20]);
        e.mem.set_u32(array, storage);
        e.mem.set_u32(array + 4, 3);
        e.mem.set_u32(array + 8, 0x20);
        (e, array)
    }

    /// The two new bytes (at offsets 3 and 4) hold the value, little-endian.
    #[test]
    fn fn_00e31ad0_stores_the_word_in_the_new_bytes() {
        let (mut e, array) = engine();
        e.call(0x00e3_1ad0, &args![0xbeefu32, array]);
        let data = e.mem.u32(array);
        assert_eq!(e.mem.u32(array + 4), 5);
        assert_eq!(e.mem.u8(data + 2), 0xee);
        assert_eq!(e.mem.u16(data + 3), 0xbeef);
        assert_eq!(e.mem.u8(data + 5), 0xee);
    }

    /// The float is stored in the four new bytes; a signalling NaN is stored quiet.
    #[test]
    fn fn_00e31b00_stores_the_float_in_the_new_bytes() {
        let (mut e, array) = engine();
        e.call(0x00e3_1b00, &args![2.5f32, array]);
        let data = e.mem.u32(array);
        assert_eq!(e.mem.u32(array + 4), 7);
        assert_eq!(e.mem.f32(data + 3), 2.5);
        assert_eq!(e.mem.u8(data + 7), 0xee);
        e.call(0x00e3_1b00, &args![f32::from_bits(0x7f80_0001), array]);
        assert_eq!(e.mem.u32(array + 4), 11);
        assert_eq!(e.mem.u32(data + 7), 0x7fc0_0001);
    }
}
