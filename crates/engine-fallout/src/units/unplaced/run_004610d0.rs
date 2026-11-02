//! `(unplaced)/run_004610d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Five member-address getters of one object (its class is not confirmed):
//! `this + 0x28`, `+ 0x70`, `+ 0x78`, `+ 0x68` and `+ 0x98`.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004610d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The address of the member at +0x28 (`thiscall`).
pub fn fn_004610d0(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x28))
}

// Translated from 004610f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The address of the member at +0x70 (`thiscall`).
pub fn fn_004610f0(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x70))
}

// Translated from 00461110 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The address of the member at +0x78 (`thiscall`).
pub fn fn_00461110(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x78))
}

// Translated from 00461130 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The address of the member at +0x68 (`thiscall`).
pub fn fn_00461130(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x68))
}

// Translated from 00461150 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The address of the member at +0x98 (`thiscall`).
pub fn fn_00461150(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x98))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004610d0, fn_004610d0(Ptr) -> Ptr),
        entry!(0x004610f0, fn_004610f0(Ptr) -> Ptr),
        entry!(0x00461110, fn_00461110(Ptr) -> Ptr),
        entry!(0x00461130, fn_00461130(Ptr) -> Ptr),
        entry!(0x00461150, fn_00461150(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_getter_adds_its_offset() {
        let mut e = Engine::new();
        for (addr, off) in [
            (0x0046_10d0, 0x28),
            (0x0046_10f0, 0x70),
            (0x0046_1110, 0x78),
            (0x0046_1130, 0x68),
            (0x0046_1150, 0x98),
        ] {
            assert_eq!(e.call(addr, &[0x1000]).u32(), 0x1000 + off);
            assert_eq!(e.call(addr, &[0xffff_ffff]).u32(), off - 1);
        }
    }
}
