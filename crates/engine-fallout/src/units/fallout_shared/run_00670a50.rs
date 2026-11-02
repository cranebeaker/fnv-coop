//! `fallout shared/run_00670a50` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00670a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): a destructor body. Each owned object
/// that is not null is destroyed and freed with the flag 1: the one at +0x2c through
/// `00670b30`, +0x30 through `007b3fa0`, +0x34 through `00670b60`, and +0x28 through its
/// vtable slot +0. Then `00680fe0` runs on the address `0087eaa0` gives for this object,
/// and `00483710` on the object.
pub fn fn_00670a50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, destructor) in [
        (0x2cu32, 0x0067_0b30u32),
        (0x30, 0x007b_3fa0),
        (0x34, 0x0067_0b60),
    ] {
        let owned = e.mem.u32(base + offset);
        if owned != 0 {
            e.call(destructor, &args![owned, 1u32]);
        }
    }
    let owned = e.mem.u32(base + 0x28);
    if owned != 0 {
        e.vcall(owned, 0, &args![1u32]);
    }
    let member = e.call(0x0087_eaa0, &args![this]).u32();
    e.call(0x0068_0fe0, &args![member]);
    e.call(0x0048_3710, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0067_0a50, fn_00670a50(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00670a50_destroys_the_owned_objects_present() {
        for present in [[false; 4], [true; 4], [true, false, true, false]] {
            let mut e = Engine::new();
            let object = e.mem.alloc(0x38);
            let virtual_object = e.mem.alloc(4);
            e.put_vtable(0x0300_0000, &[0x0300_1000]);
            e.mem.set_u32(virtual_object, 0x0300_0000);
            let owned = [0x6000u32, 0x6100, 0x6200, virtual_object];
            for (index, offset) in [0x2cu32, 0x30, 0x34, 0x28].into_iter().enumerate() {
                if present[index] {
                    e.mem.set_u32(object + offset, owned[index]);
                }
            }
            for address in [
                0x0067_0b30u32,
                0x007b_3fa0,
                0x0067_0b60,
                0x0300_1000,
                0x0068_0fe0,
                0x0048_3710,
            ] {
                e.register(address, |_, _| Ret::default());
            }
            e.register(0x0087_eaa0, |_, a| Ret {
                eax: a[0] + 0x40,
                ..Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x0067_0a50, &args![object]);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![];
            for (index, destructor) in [0x0067_0b30u32, 0x007b_3fa0, 0x0067_0b60, 0x0300_1000]
                .into_iter()
                .enumerate()
            {
                if present[index] {
                    expected.push((destructor, vec![owned[index], 1]));
                }
            }
            expected.extend([
                (0x0087_eaa0, vec![object]),
                (0x0068_0fe0, vec![object + 0x40]),
                (0x0048_3710, vec![object]),
            ]);
            assert_eq!(log[1..], expected);
        }
    }
}
