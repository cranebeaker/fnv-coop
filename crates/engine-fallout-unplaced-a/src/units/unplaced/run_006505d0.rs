//! `(unplaced)/run_006505d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00559450`: answers the pointer an `NiPointer`-like holder keeps (`thiscall`).
const POINTER_GET: u32 = 0x0055_9450;

// Translated from 006505d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): 0 without a holder at `this + 8`,
/// else what `00559450` answers for it.
pub fn fn_006505d0(e: &mut Engine, this: Ptr) -> u32 {
    let holder = e.mem.u32(this.addr() + 8);
    if holder == 0 {
        return 0;
    }
    e.call(POINTER_GET, &args![holder]).u32()
}

// Translated from 00650600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): 0 without a holder at
/// `this + 0xC`, else what `00559450` answers for it.
pub fn fn_00650600(e: &mut Engine, this: Ptr) -> u32 {
    let holder = e.mem.u32(this.addr() + 0xc);
    if holder == 0 {
        return 0;
    }
    e.call(POINTER_GET, &args![holder]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_05d0, fn_006505d0(Ptr) -> u32),
        entry!(0x0065_0600, fn_00650600(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holders_answer_their_pointer_or_zero() {
        let mut e = Engine::new();
        e.register(POINTER_GET, |_, a| (a[0] + 1).into_ret());
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0065_05d0, &args![this]).u32(), 0);
        assert_eq!(e.call(0x0065_0600, &args![this]).u32(), 0);
        e.mem.set_u32(this + 8, 0x100);
        e.mem.set_u32(this + 0xc, 0x200);
        assert_eq!(e.call(0x0065_05d0, &args![this]).u32(), 0x101);
        assert_eq!(e.call(0x0065_0600, &args![this]).u32(), 0x201);
    }
}
