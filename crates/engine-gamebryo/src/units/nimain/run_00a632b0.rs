//! `NiMain/run_00a632b0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The two object pointers `00a632b0` disposes of, in its order.
const FIRST_OBJECT_SLOT: u32 = 0x011f_4784;
const SECOND_OBJECT_SLOT: u32 = 0x011f_4788;

// Translated from 00a632b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; one of the `NiMain` shutdown steps of
/// `00a8a280`): for `011f4784` and then `011f4788`, when the pointer holds an object, calls
/// its vtable slot 0 with 1 (`thiscall`). The pointers are not cleared.
pub fn fn_00a632b0(e: &mut Engine) {
    for slot in [FIRST_OBJECT_SLOT, SECOND_OBJECT_SLOT] {
        let object = e.mem.u32(slot);
        if object != 0 {
            e.vcall(object, 0, &args![1u32]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a6_32b0, fn_00a632b0())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destroys_each_held_object_and_keeps_the_pointers() {
        let mut e = Engine::new();
        e.map(FIRST_OBJECT_SLOT & !0xfff, 0x1000);
        e.put_vtable(0x0150_0000, &[0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0150_0000);
        e.mem.set_u32(SECOND_OBJECT_SLOT, object);
        e.call_log = Some(vec![]);
        e.call(0x00a6_32b0, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0150_1000, vec![object, 1])]);
        assert_eq!(e.mem.u32(SECOND_OBJECT_SLOT), object);
        e.mem.set_u32(FIRST_OBJECT_SLOT, object);
        e.call_log = Some(vec![]);
        e.call(0x00a6_32b0, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0150_1000, vec![object, 1]),
                (0x0150_1000, vec![object, 1])
            ]
        );
    }
}
