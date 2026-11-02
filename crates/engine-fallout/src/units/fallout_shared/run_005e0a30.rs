//! `fallout shared/run_005e0a30` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTArray<const char*,NiTMallocInterface<const char*>>`'s vtable (RTTI).
const ARRAY_VTABLE: u32 = 0x0103_d418;
/// `0096afc0` (cdecl, element count): allocates the array's data.
const ALLOCATE_DATA: u32 = 0x0096_afc0;

// Translated from 005e0a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiTArray<const char*>` constructor
/// (vtable `0103d418`): capacity `max_size` (+8), grow-by `grow_by` (+0xE),
/// size and used count 0 (+0xA, +0xC), and data from `0096afc0` for a
/// non-zero capacity, else null (+4). Returns `this`.
pub fn fn_005e0a30(e: &mut Engine, this: Ptr, max_size: u16, grow_by: u16) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, ARRAY_VTABLE);
    e.mem.set_u16(t + 8, max_size);
    e.mem.set_u16(t + 0xe, grow_by);
    e.mem.set_u16(t + 0xa, 0);
    e.mem.set_u16(t + 0xc, 0);
    if e.mem.u16(t + 8) as i32 > 0 {
        let count = e.mem.u16(t + 8) as u32;
        let data = e.call(ALLOCATE_DATA, &args![count]).u32();
        e.mem.set_u32(t + 4, data);
    } else {
        e.mem.set_u32(t + 4, 0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005e_0a30, fn_005e0a30(Ptr, u16, u16) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_005e0a30_builds_an_empty_array() {
        let mut e = Engine::new();
        e.register(ALLOCATE_DATA, |_, a| Ret {
            eax: 0x5000 + a[0],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xaa; 0x10]);
        assert_eq!(e.call(0x005e_0a30, &args![this, 3u32, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x5003);
        assert_eq!(e.mem.u16(this + 8), 3);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
        assert_eq!(e.mem.u16(this + 0xe), 2);
        e.call(0x005e_0a30, &args![this, 0u32, 2u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
    }
}
