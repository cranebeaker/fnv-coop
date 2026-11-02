//! `BSHavok/run_00c87c30` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base-class constructor `00a5d3a0` (`ECX` = the object).
const BASE_CONSTRUCT: u32 = 0x00a5_d3a0;
/// The vtable the constructor installs.
const VTABLE: u32 = 0x010c_5b54;

// Translated from 00c87c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `ECX` = the block: the `bhkRefObject` constructor. Runs the base-class
/// constructor (`00a5d3a0`), sets the vtable `010c5b54`, clears `+8` and answers the block.
pub fn fn_00c87c30(e: &mut Engine, this: Ptr) -> u32 {
    let me = this.addr();
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(me, VTABLE);
    e.mem.set_u32(me + 8, 0);
    me
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c87c30, fn_00c87c30(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_constructor_runs_the_base_then_sets_the_vtable_and_clears_the_word() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0xdead);
            e.mem.set_u32(a[0] + 8, 0xbeef);
            Ret::default()
        });
        let block = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00c8_7c30, &args![block]).u32(), block);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.mem.u32(block + 8), 0);
    }
}
