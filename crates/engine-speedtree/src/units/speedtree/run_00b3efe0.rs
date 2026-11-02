//! `SpeedTree/run_00b3efe0` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two small functions identified from the code (the engine map gives them no names): the peek of
//! the int at a `CTreeFileAccess` cursor (without advancing it) and the constructor of a class
//! with the `010a5890` vtable and three floats at `+4`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::billboardleaf::CTreeFileAccess;
use super::branch::INVALID_PARAMETER;

/// The vtable `00b3f030` stores (`010a5890`, the "scalar deleting destructor" table of the
/// object's class).
const SCALAR_DELETING_VTABLE: u32 = 0x010a_5890;

// Translated from 00b3efe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the int at the `CTreeFileAccess` cursor without advancing it (identified from the code;
/// `fastcall`): a cursor at or past the end of the file (`last - first` bytes) takes the
/// invalid-parameter path.
pub fn fn_00b3efe0(e: &mut Engine, this: Ptr<CTreeFileAccess>) -> i32 {
    let index = e.get(this, CTreeFileAccess::m_nIndex);
    let size = e
        .get(this, CTreeFileAccess::m_vMemoryLast)
        .addr()
        .wrapping_sub(e.get(this, CTreeFileAccess::m_vMemoryFirst).addr());
    if index >= size {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let first = e.get(this, CTreeFileAccess::m_vMemoryFirst).addr();
    e.mem.i32(first.wrapping_add(index))
}

// Translated from 00b3f030 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of a small class (identified from the code; class unconfirmed; `fastcall`):
/// stores the `010a5890` vtable at `+0` and clears the three floats at `+4`, `+8` and `+0xC`
/// (`FLDZ`, `FSTP`). Returns `this`.
pub fn fn_00b3f030(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), SCALAR_DELETING_VTABLE);
    for offset in [4u32, 8, 0x0C] {
        e.mem.set_f32(this.addr() + offset, 0.0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b3efe0, fn_00b3efe0(Ptr<CTreeFileAccess>) -> i32),
        entry!(0x00b3f030, fn_00b3f030(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn peek_reads_without_advancing() {
        let mut e = Engine::new();
        e.register(INVALID_PARAMETER, |_, _| panic!("invalid parameter"));
        let file: Ptr<CTreeFileAccess> = e.new_object();
        let memory = e.mem.alloc(8);
        e.mem.set_u32(memory, 0x1234_5678);
        e.mem.set_u32(memory + 4, 0xFFFF_FFFE);
        e.set(file, CTreeFileAccess::m_nIndex, 0);
        e.set(file, CTreeFileAccess::m_vMemoryFirst, Ptr::new(memory));
        e.set(file, CTreeFileAccess::m_vMemoryLast, Ptr::new(memory + 8));
        assert_eq!(e.call(0x00b3_efe0, &args![file]).i32(), 0x1234_5678);
        assert_eq!(e.get(file, CTreeFileAccess::m_nIndex), 0);
        e.set(file, CTreeFileAccess::m_nIndex, 4);
        assert_eq!(e.call(0x00b3_efe0, &args![file]).i32(), -2);
        // At the end of the file, the peek is an invalid parameter.
        e.set(file, CTreeFileAccess::m_nIndex, 8);
        let failed = catch_unwind(AssertUnwindSafe(|| {
            fn_00b3efe0(&mut e, file);
        }));
        assert!(failed.is_err());
    }

    #[test]
    fn constructor_stores_the_vtable_and_clears_the_floats() {
        let mut e = Engine::new();
        let at = e.mem.alloc(0x10);
        for word in 0..4 {
            e.mem.set_u32(at + 4 * word, 0xDEAD_BEEF);
        }
        let r = e.call(0x00b3_f030, &args![Ptr::<()>::new(at)]);
        assert_eq!(r.u32(), at);
        assert_eq!(e.mem.u32(at), SCALAR_DELETING_VTABLE);
        for offset in [4u32, 8, 0x0C] {
            assert_eq!(e.mem.u32(at + offset), 0);
        }
    }
}
