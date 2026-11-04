//! `(unplaced)/run_00645c60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The fields read (offsets as the code reads them): `float`s at +0x178,
/// +0x14c and +0x154, a flag word at +0x12c.
const FLOAT_178: u32 = 0x178;
const FLOAT_14C: u32 = 0x14c;
const FLOAT_154: u32 = 0x154;
const FLAGS_12C: u32 = 0x12c;

/// The `float` at `address` as `FLD` loads it (a signalling NaN quieted).
fn load_float(e: &Engine, address: u32) -> f32 {
    x87_quiet(e.mem.f32(address))
}

// Translated from 00645c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at +0x178 of `this`, in ST0 (unnamed in the engine map).
pub fn fn_00645c60(e: &mut Engine, this: Ptr) -> f32 {
    load_float(e, this.addr() + FLOAT_178)
}

// Translated from 00645c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 9 (`0x200`) of the word at +0x12c of `this` is set
/// (unnamed in the engine map).
pub fn fn_00645c80(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + FLAGS_12C) & 0x200 != 0
}

// Translated from 00645ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at +0x14c of `this`, in ST0 (unnamed in the engine map).
pub fn fn_00645ca0(e: &mut Engine, this: Ptr) -> f32 {
    load_float(e, this.addr() + FLOAT_14C)
}

// Translated from 00645ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at +0x154 of `this`, in ST0 (unnamed in the engine map).
pub fn fn_00645ce0(e: &mut Engine, this: Ptr) -> f32 {
    load_float(e, this.addr() + FLOAT_154)
}

// Translated from 00645cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at +0x150 of `this`, in ST0 (unnamed in the engine map; called by
/// `00645380`).
pub fn fn_00645cc0(e: &mut Engine, this: Ptr) -> f32 {
    load_float(e, this.addr() + 0x150)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0064_5cc0, fn_00645cc0(Ptr) -> f32),
        entry!(0x0064_5c60, fn_00645c60(Ptr) -> f32),
        entry!(0x0064_5c80, fn_00645c80(Ptr) -> bool),
        entry!(0x0064_5ca0, fn_00645ca0(Ptr) -> f32),
        entry!(0x0064_5ce0, fn_00645ce0(Ptr) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_getters_load_their_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x180);
        e.mem.set_f32(this + FLOAT_178, 1.5);
        e.mem.set_f32(this + FLOAT_14C, -2.25);
        e.mem.set_u32(this + FLOAT_154, 0x7f80_0001);
        assert_eq!(e.call(0x0064_5c60, &args![this]).f64(), 1.5);
        assert_eq!(e.call(0x0064_5ca0, &args![this]).f64(), -2.25);
        let nan = e.call(0x0064_5ce0, &args![this]).f64();
        assert!(nan.is_nan());
        assert_eq!(x87_store32(nan), 0x7fc0_0001);
    }

    #[test]
    fn flag_getter_tests_bit_9() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x130);
        e.mem.set_u32(this + FLAGS_12C, 0xffff_fdff);
        assert_eq!(e.call(0x0064_5c80, &args![this]).u32() & 0xff, 0);
        e.mem.set_u32(this + FLAGS_12C, 0x200);
        assert_eq!(e.call(0x0064_5c80, &args![this]).u32() & 0xff, 1);
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    #[test]
    fn fn_00645cc0_loads_the_float_at_0x150() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x160);
        e.mem.set_f32(this + 0x150, 2.25);
        assert_eq!(e.call(0x0064_5cc0, &args![this]).f64(), 2.25);
        e.mem.set_u32(this + 0x150, 0x7f80_0001);
        assert_eq!(
            e.call(0x0064_5cc0, &args![this]).f32().to_bits(),
            0x7fc0_0001
        );
    }
}
