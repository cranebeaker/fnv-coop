//! `fallout shared/region system/run_004f5d90` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f5d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the float argument at `+4` (unnamed in the map; `ECX`, one stack
/// word, `RET 4`; an `FLD`/`FSTP` pair, so a signalling NaN is stored
/// quiet). `TESForm::LoadData` (`004861f0`) calls it as the setter of the
/// form's `TESWeightForm` component (the cast it makes first).
pub fn fn_004f5d90(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 4, x87_quiet_bits(value.to_bits()));
}

/// `0096a2d0` (unnamed in the engine map): the list constructor run on +4.
const LIST_CTOR_0096A2D0: u32 = 0x0096_a2d0;
/// The vtable of `TESRegionGrassObjectList` (RTTI `.?AVTESRegionGrassObjectList@@`, read at the word before it).
const TES_REGION_GRASS_OBJECT_LIST_VTABLE: u32 = 0x0102_4094;

// Translated from 004f5db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionGrassObjectList` constructor (unnamed in the engine map; the
/// vtable's RTTI names the class; `thiscall`, `RET 4`): constructs the list
/// at +4 (`0096a2d0`), stores the vtable, stores the byte `flag` at +0xC
/// and clears the word at +0x10. Returns `this`.
pub fn fn_004f5db0(e: &mut Engine, this: Ptr, flag: u8) -> Ptr {
    e.call(LIST_CTOR_0096A2D0, &args![this.addr() + 4]);
    e.mem
        .set_u32(this.addr(), TES_REGION_GRASS_OBJECT_LIST_VTABLE);
    e.mem.set_u8(this.addr() + 0xc, flag);
    e.mem.set_u32(this.addr() + 0x10, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f5d90, fn_004f5d90(Ptr, f32)),
        entry!(0x004f_5db0, fn_004f5db0(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod grass_list_tests {
    use super::*;

    #[test]
    fn at_004f5db0_builds_the_list_and_stores_the_flag() {
        let mut e = Engine::new();
        e.register(LIST_CTOR_0096A2D0, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0xc, 0xffff_ffff);
        e.mem.set_u32(this + 0x10, 0xffff_ffff);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x004f_5db0, &args![this, 0x0000_0101u32]).u32(),
            this
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_GRASS_OBJECT_LIST_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0xffff_ff01);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        let ctor: Vec<_> = log.iter().filter(|c| c.0 == LIST_CTOR_0096A2D0).collect();
        assert_eq!(ctor[0].1, vec![this + 4]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_float_and_quiets_a_signalling_nan() {
        let mut e = Engine::new();
        let component = e.mem.alloc(8);
        let this = Ptr::<()>::new(component);
        e.call(0x004f_5d90, &args![this, 2.5f32.to_bits()]);
        assert_eq!(e.mem.u32(component + 4), 2.5f32.to_bits());
        e.call(0x004f_5d90, &args![this, 0xff80_0001u32]);
        assert_eq!(e.mem.u32(component + 4), 0xffc0_0001);
        e.call(0x004f_5d90, &args![this, 0x7fc0_1234u32]);
        assert_eq!(e.mem.u32(component + 4), 0x7fc0_1234);
        assert_eq!(e.mem.u32(component), 0);
    }
}
