//! `BSShader/watershaderproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSShaderProperty::BSShaderProperty` (engine map), thiscall.
const SHADER_PROPERTY_CONSTRUCT: u32 = 0x00ba_8e50;
/// `WaterShaderProperty`'s vtable (the one the constructor sets).
const WATER_SHADER_PROPERTY_VTABLE: u32 = 0x010a_e438;
/// The float copied to `+0x124` (`010ae4fc`).
const VALUE_FOR_124: u32 = 0x010a_e4fc;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00b6abb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b6abb0 - the C++ exception-unwinding frame (FS:[0]) that destroys the base when a member constructor throws is not translated
/// `WaterShaderProperty::WaterShaderProperty` (Xbox PDB), thiscall
/// without stack words. On PC the class's fields sit 8 bytes below the
/// Xbox PDB's from `+0x60` on. Constructs the `BSShaderProperty` base,
/// sets the vtable, zeroes the 36 floats from `+0x88` to `+0x114` and the
/// words from `+0x128` to `+0x148`, sets `+0x6c`, `+0x11c` and `+0x120` to
/// 1.0, `+0x70` to 0.0 and `+0x124` to the float at `010ae4fc`
/// (`FLD`/`FSTP`: a signalling NaN comes out quiet), the bytes `+0x7f` to
/// `+0x81` to 1 and the other flags and words to 0, the shader number
/// (`+0x58`) to 0x11 and the shader property type (`+0x1c`) to 0xe.
/// Answers `this`.
pub fn water_shader_property_water_shader_property(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(SHADER_PROPERTY_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, WATER_SHADER_PROPERTY_VTABLE);
    let zero = x87_store32(0.0);
    let one = x87_store32(1.0);
    for offset in (0x88..=0x114u32).step_by(4) {
        e.mem.set_u32(base + offset, zero);
    }
    for offset in [
        0x130u32, 0x128, 0x12c, 0x134, 0x138, 0x13c, 0x140, 0x144, 0x148,
    ] {
        e.mem.set_u32(base + offset, 0);
    }
    // The code then releases the pointer it just cleared (`+0x148`); the
    // pointer it tests is the constant 0 it stored, so the release never
    // runs. It is translated as the code has it.
    let previous = 0u32;
    if previous != 0 {
        let count = e.call(INTERLOCKED_DECREMENT, &args![previous + 4]).u32();
        if count == 0 {
            e.vcall(previous, 4, &args![]);
        }
        e.mem.set_u32(base + 0x148, 0);
    }
    e.mem.set_u32(base + 0x6c, one);
    e.mem.set_u8(base + 0x7f, 1);
    e.mem.set_u8(base + 0x80, 1);
    e.mem.set_u32(base + 0x70, zero);
    e.mem.set_u8(base + 0x81, 1);
    let value = x87_quiet_bits(e.global::<u32>(VALUE_FOR_124));
    e.mem.set_u8(base + 0x60, 0);
    e.mem.set_u32(base + 0x124, value);
    for offset in [0x61u32, 0x62, 0x63] {
        e.mem.set_u8(base + offset, 0);
    }
    e.mem.set_u32(base + 0x11c, one);
    e.mem.set_u32(base + 0x68, 0);
    e.mem.set_u32(base + 0x120, one);
    e.mem.set_u32(base + 0x64, 0);
    for offset in [0x7du32, 0x7e, 0x7c] {
        e.mem.set_u8(base + offset, 0);
    }
    e.mem.set_u32(base + 0x14c, 0);
    e.mem.set_u8(base + 0x82, 0);
    e.mem.set_u8(base + 0x83, 0);
    e.mem.set_u32(base + 0x84, 0);
    e.mem.set_u32(base + 0x58, 0x11);
    e.mem.set_u32(base + 0x1c, 0xe);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00b6abb0,
        water_shader_property_water_shader_property(Ptr) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_the_base_and_sets_the_defaults() {
        let mut e = Engine::new();
        e.map(0x010a_e000, 0x1000);
        e.set_global(VALUE_FOR_124, 0x7f80_0001u32);
        e.register(SHADER_PROPERTY_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x158);
        for word in 0..0x56 {
            e.mem.set_u32(this + 4 * word, 0xdddd_dddd);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_abb0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|c| c.0 == SHADER_PROPERTY_CONSTRUCT && c.1 == vec![this]));
        assert_eq!(e.mem.u32(this), WATER_SHADER_PROPERTY_VTABLE);
        for offset in (0x88..=0x114u32).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x118), 0xdddd_dddd);
        for offset in (0x128..=0x14cu32).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.f32(this + 0x6c), 1.0);
        assert_eq!(e.mem.f32(this + 0x70), 0.0);
        assert_eq!(e.mem.f32(this + 0x11c), 1.0);
        assert_eq!(e.mem.f32(this + 0x120), 1.0);
        assert_eq!(e.mem.u32(this + 0x124), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x68), 0);
        assert_eq!(e.mem.u32(this + 0x7c), 0x0100_0000);
        assert_eq!(e.mem.u32(this + 0x80), 0x0000_0101);
        assert_eq!(e.mem.u32(this + 0x84), 0);
        assert_eq!(e.mem.u32(this + 0x58), 0x11);
        assert_eq!(e.mem.u32(this + 0x1c), 0xe);
    }
}
