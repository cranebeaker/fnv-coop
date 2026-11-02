//! `fallout/ai/run_008c6e90` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Number of entries in the table (0x4D, 8 bytes each: a "default" byte at
/// +0 and a float at +4).
const ENTRY_COUNT: i32 = 0x4d;

// Translated from 008c6eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: resets all 0x4D entries: float 0.0 (`FLDZ`)
/// at +4, byte 1 at +0.
pub fn fn_008c6eb0(e: &mut Engine, this: Ptr) {
    let mut index = 0i32;
    while index < ENTRY_COUNT {
        let entry = this.addr().wrapping_add((index as u32).wrapping_mul(8));
        e.mem.set_f32(entry + 4, 0.0);
        e.mem.set_u8(entry, 1);
        index += 1;
    }
}

// Translated from 008c6e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the table's constructor: resets it
/// (`008c6eb0`) and returns `this`.
pub fn fn_008c6e90(e: &mut Engine, this: Ptr) -> Ptr {
    fn_008c6eb0(e, this);
    this
}

// Translated from 008c6f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: for `index` in 0..0x4D, stores the float
/// `value` (through an x87 load and store) at the entry's +4 and clears its
/// byte at +0.
pub fn fn_008c6f00(e: &mut Engine, this: Ptr, index: i32, value: f32) {
    if !(0..ENTRY_COUNT).contains(&index) {
        return;
    }
    let entry = this.addr().wrapping_add((index as u32).wrapping_mul(8));
    e.mem.set_u32(entry + 4, x87_quiet_bits(value.to_bits()));
    e.mem.set_u8(entry, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008c_6eb0, fn_008c6eb0(Ptr)),
        entry!(0x008c_6e90, fn_008c6e90(Ptr) -> Ptr),
        entry!(0x008c_6f00, fn_008c6f00(Ptr, i32, f32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_008c6eb0_resets_every_entry() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x4d * 8 + 8);
        e.mem.write(this, &[0xaa; 0x4d * 8 + 8]);
        e.call(0x008c_6eb0, &args![this]);
        for i in 0..0x4d {
            assert_eq!(e.mem.u8(this + 8 * i), 1);
            assert_eq!(e.mem.u32(this + 8 * i + 4), 0);
        }
        assert_eq!(e.mem.u8(this + 0x4d * 8), 0xaa, "only 0x4d entries");
    }

    #[test]
    fn fn_008c6e90_resets_and_returns_this() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x4d * 8);
        assert_eq!(e.call(0x008c_6e90, &args![this]).u32(), this);
        assert_eq!(e.mem.u8(this + 8 * 0x4c), 1);
    }

    #[test]
    fn fn_008c6f00_sets_one_entry_in_range() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x4d * 8);
        e.call(0x008c_6e90, &args![this]);
        e.call(0x008c_6f00, &args![this, 3u32, 1.5f32.to_bits()]);
        assert_eq!(e.mem.u8(this + 24), 0);
        assert_eq!(e.mem.f32(this + 28), 1.5);
        e.call(0x008c_6f00, &args![this, 4u32, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 36), 0x7fc0_0001);
        e.call(0x008c_6f00, &args![this, 0x4du32, 2.0f32.to_bits()]);
        e.call(0x008c_6f00, &args![this, -1i32, 2.0f32.to_bits()]);
        assert_eq!(e.mem.u8(this), 1);
        assert_eq!(e.mem.u8(this + 8 * 0x4c), 1);
    }
}
