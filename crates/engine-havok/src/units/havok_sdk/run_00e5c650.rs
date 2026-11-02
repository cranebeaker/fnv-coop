//! `Havok SDK/run_00e5c650` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkxAttributeHolder::findAttributeObjectByName` (Xbox PDB, `00e5c540`): thiscall on the holder,
/// `(name, group name)`, `RET 8`.
const FIND_ATTRIBUTE_OBJECT: u32 = 0x00e5_c540;
/// The group name searched for (`0126ea38`).
const GROUP_NAME: u32 = 0x0126_ea38;

// Translated from 00e5c650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4`: `hkxAttributeHolder::findAttributeObjectByName` (`00e5c540`) on `this` with
/// `name` and the string at `0126ea38`; returns its result.
pub fn fn_00e5c650(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    e.call(FIND_ATTRIBUTE_OBJECT, &args![this, name, GROUP_NAME])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e5_c650, fn_00e5c650(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn the_holder_is_asked_with_the_fixed_group_name() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(FIND_ATTRIBUTE_OBJECT, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: 0x4242,
                ..Ret::default()
            }
        });
        assert_eq!(
            e.call(0x00e5_c650, &args![0x1000u32, 0x2000u32]).u32(),
            0x4242
        );
        assert_eq!(*seen.borrow(), vec![vec![0x1000, 0x2000, 0x0126_ea38]]);
    }
}
