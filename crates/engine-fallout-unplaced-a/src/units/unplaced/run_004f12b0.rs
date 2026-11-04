//! `(unplaced)/run_004f12b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `TESDataHandler` singleton pointer.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `004169d0` (`ECX` = the data handler): the object whose list at +4 is counted.
const LIST_OWNER_004169D0: u32 = 0x0041_69d0;
/// `005ae380` (`ECX` = a list; the engine map's `VATS::GetCount`): its entry count.
const LIST_COUNT: u32 = 0x005a_e380;

// Translated from 004f12b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A save-size count (unnamed in the engine map; no arguments, plain `RET`; called by
/// `SaveGlobalData`): 2 plus 8 for each entry of the list at +4 of the object `004169d0`
/// answers for the data handler, as a 16-bit value (`AX`).
pub fn fn_004f12b0(e: &mut Engine) -> u16 {
    let size: u16 = 0;
    let size = size.wrapping_add(2);
    let handler: u32 = e.global(DATA_HANDLER);
    let owner = e.call(LIST_OWNER_004169D0, &args![handler]).u32();
    let count = e.call(LIST_COUNT, &args![owner.wrapping_add(4)]).u32();
    u32::from(size).wrapping_add(count.wrapping_mul(8)) as u16
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f_12b0, fn_004f12b0() -> u16)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_004f12b0_counts_eight_bytes_per_entry_plus_two() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.set_global(DATA_HANDLER, 0x9000u32);
        e.register(LIST_OWNER_004169D0, |_, a| (a[0] + 0x100).into_ret());
        e.register(LIST_COUNT, |_, a| {
            assert_eq!(a[0], 0x9104);
            5u32.into_ret()
        });
        assert_eq!(e.call(0x004f_12b0, &[]).u32() & 0xffff, 42);
        e.register(LIST_COUNT, |_, _| 0x2000u32.into_ret());
        // Only the low 16 bits are kept.
        assert_eq!(e.call(0x004f_12b0, &[]).u32() & 0xffff, 2);
    }
}
