//! `Havok SDK/run_00de4470` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The constructor of the primitive drawer at `this + 0xc0` (`00e10a00`, thiscall, no words).
const PRIMITIVE_DRAWER_CONSTRUCT: u32 = 0x00e1_0a00;
/// The `float`s `00de4470` copies through the x87 stack: `0.25` (`0101622c`), `0.75`
/// (`01016264`) and `0.5` (`01016248`) in the exe.
const FIRST_CONSTANT: u32 = 0x0101_622c;
const SECOND_CONSTANT: u32 = 0x0101_6264;
const THIRD_CONSTANT: u32 = 0x0101_6248;

// Translated from 00de4470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the constraint drawer (the engine map has no name; `hkpConstraintViewer`
/// calls it as the drawer's constructor): constructs the primitive drawer at `this + 0xc0`
/// (`00e10a00`), then stores `1.0` at `+0`, `+0xc` and `+0x1c`, the float at `0101622c` at
/// `+8` and `+0x14`, the float at `01016264` at `+4`, the float at `01016248` at `+0x10` and
/// the integer `0x18` at `+0x18`. The constants go through the x87 stack
/// (`FLD`/`FST`: a signalling NaN is stored quiet). Returns `this`.
pub fn fn_00de4470(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.call(PRIMITIVE_DRAWER_CONSTRUCT, &args![me.wrapping_add(0xc0)]);
    let first = x87_quiet_bits(e.mem.u32(FIRST_CONSTANT));
    e.mem.set_u32(me.wrapping_add(8), first);
    e.mem.set_u32(me.wrapping_add(0x18), 0x18);
    let second = x87_quiet_bits(e.mem.u32(SECOND_CONSTANT));
    e.mem.set_u32(me.wrapping_add(4), second);
    e.mem.set_f32(me, 1.0);
    e.mem.set_f32(me.wrapping_add(0xc), 1.0);
    let third = x87_quiet_bits(e.mem.u32(THIRD_CONSTANT));
    e.mem.set_u32(me.wrapping_add(0x10), third);
    e.mem.set_u32(me.wrapping_add(0x14), first);
    e.mem.set_f32(me.wrapping_add(0x1c), 1.0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00de_4470, fn_00de4470(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_constructor_stores_the_defaults_and_builds_the_primitive_drawer() {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.mem.set_u32(FIRST_CONSTANT, 0x3f00_0000);
        e.mem.set_u32(SECOND_CONSTANT, 0x3e80_0000);
        e.mem.set_u32(THIRD_CONSTANT, 0x3f40_0000);
        e.register(PRIMITIVE_DRAWER_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x100);
        let result = e.call(0x00de_4470, &args![this]);
        assert_eq!(result.u32(), this);
        let log = e.call_log.take().unwrap();
        let built: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == PRIMITIVE_DRAWER_CONSTRUCT)
            .collect();
        assert_eq!(built.len(), 1);
        assert_eq!(built[0].1, vec![this + 0xc0]);
        let fields: Vec<u32> = (0..8).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(
            fields,
            vec![
                0x3f80_0000,
                0x3e80_0000,
                0x3f00_0000,
                0x3f80_0000,
                0x3f40_0000,
                0x3f00_0000,
                0x18,
                0x3f80_0000
            ]
        );
    }

    #[test]
    fn a_signalling_constant_is_stored_quiet() {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.mem.set_u32(FIRST_CONSTANT, 0x7f80_0001);
        e.register(PRIMITIVE_DRAWER_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        e.call(0x00de_4470, &args![this]);
        assert_eq!(e.mem.u32(this + 8), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x14), 0x7fc0_0001);
    }
}
