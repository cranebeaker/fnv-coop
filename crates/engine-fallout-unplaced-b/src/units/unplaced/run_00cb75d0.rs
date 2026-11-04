//! `(unplaced)/run_00cb75d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `bhkWaterListener` vtable (RTTI `.?AVbhkWaterListener@@`, read
/// through its complete object locator).
const WATER_LISTENER_VTABLE: u32 = 0x010c_88b8;

layout! {
    /// `bhkWaterListener` (Xbox PDB), 0x14 bytes; the PC constructor writes
    /// the same fields at the same offsets.
    pub struct BhkWaterListener: 0x14 {
        /// `iFlag` (Xbox PDB, `bhkEntityListener`).
        0x04 iFlag: u32,
        /// `bEnabled` (Xbox PDB, `bhkEntityListener`).
        0x08 bEnabled: bool,
        /// `Entities` (Xbox PDB): a `BSSimpleList<NiPointer<bhkEntity>>`,
        /// its item word.
        0x0C Entities_item: u32,
        /// The list's next pointer.
        0x10 Entities_next: u32,
    }
}

// Translated from 00cb75d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bhkWaterListener` constructor (no Xbox PDB name at this address):
/// clears the entity list (+0x0C, +0x10) and `iFlag`, sets `bEnabled` and
/// stores the `bhkWaterListener` vtable. Returns `this` (in `EAX`).
pub fn fn_00cb75d0(e: &mut Engine, this: Ptr<BhkWaterListener>) -> Ptr<BhkWaterListener> {
    e.set(this, BhkWaterListener::Entities_item, 0);
    e.set(this, BhkWaterListener::Entities_next, 0);
    e.set(this, BhkWaterListener::iFlag, 0);
    e.set(this, BhkWaterListener::bEnabled, true);
    e.mem.set_u32(this.addr(), WATER_LISTENER_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cb_75d0,
        fn_00cb75d0(Ptr<BhkWaterListener>) -> Ptr<BhkWaterListener>
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_clears_the_list_and_enables() {
        let mut e = Engine::new();
        let this: Ptr<BhkWaterListener> = e.new_object();
        for offset in (0..0x14).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xffff_ffff);
        }
        let back = e.call(0x00cb_75d0, &args![this]).u32();
        assert_eq!(back, this.addr());
        assert_eq!(e.mem.u32(this.addr()), WATER_LISTENER_VTABLE);
        assert_eq!(e.get(this, BhkWaterListener::iFlag), 0);
        // Only the byte is written: the other three bytes of the word stay.
        assert_eq!(e.mem.u32(this.addr() + 8), 0xffff_ff01);
        assert_eq!(e.get(this, BhkWaterListener::Entities_item), 0);
        assert_eq!(e.get(this, BhkWaterListener::Entities_next), 0);
    }
}
