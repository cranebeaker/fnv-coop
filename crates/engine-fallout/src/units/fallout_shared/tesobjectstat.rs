//! `fallout shared/tesobjectstat.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0051b530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectSTAT::InitItem` (Xbox PDB; `ECX` only, plain `RET`): unless `004013e0` says
/// otherwise, runs `0048aa80` (the model texture swap at +0x30) with the form; when the word
/// at +0x54 (the looping/random sound's form ID as loaded) is not 0, fixes it up in a stack
/// slot through `00485d50` (cdecl, the slot and the form's file from `00484e60` with -1),
/// looks it up with `004839c0`, casts it with `__RTDynamicCast` (`00ec43fb`: `.?AVTESForm@@`
/// at `0x0118_3028` to `.?AVTESSound@@` at `0x0118_32fc`) and stores the result at +0x54;
/// a null result prints "MASTERFILE: Could not find looping/random sound (%08X) for static
/// '%s' (%08X)." (`0x0102_bbb0`, through `005b5e40`) with the ID, the form's name (vtable
/// slot +0x130) and the word `0084e3a0` gives. Then `00484ab0` with 1.
pub fn tes_object_stat_init_item(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    e.call(0x0048_aa80, &args![base + 0x30, this]);
    let loaded = e.mem.u32(base + 0x54);
    if loaded != 0 {
        let id = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), loaded);
            let file = e.call(0x0048_4e60, &args![this, 0xffff_ffffu32]).u32();
            e.call(0x0048_5d50, &args![slot, file]);
            e.mem.u32(slot.addr())
        });
        let form = e.call(0x0048_39c0, &args![id]).u32();
        let sound = e
            .call(
                0x00ec_43fb,
                &args![form, 0u32, 0x0118_3028u32, 0x0118_32fcu32, 0u32],
            )
            .u32();
        e.mem.set_u32(base + 0x54, sound);
        if e.mem.u32(base + 0x54) == 0 {
            let form_id = e.call(0x0084_e3a0, &args![this]).u32();
            let name = e.vcall(base, 0x130, &args![]).u32();
            e.call(0x005b_5e40, &args![0x0102_bbb0u32, id, name, form_id]);
        }
    }
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_b530, tes_object_stat_init_item(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Doubles for an `InitItem` that resolves one sound: IDs below 0x2000 are found (the cast
    /// gives the form + 0x100), the fix-up adds 1, the name is 0x7777 and the form ID 0xbeef.
    fn sound_case(e: &mut Engine, checked: bool) -> u32 {
        let form = e.mem.alloc(0xb0);
        e.put_vtable(0x0300_0000, &[0; 0x50]);
        e.mem.set_u32(0x0300_0000 + 0x130, 0x0300_1130);
        e.mem.set_u32(form, 0x0300_0000);
        e.register(0x0300_1130, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        e.register_double(0x0040_13e0, move |_, _| Ret {
            eax: u32::from(checked),
            ..Ret::default()
        });
        e.register(0x0048_4e60, |_, _| Ret {
            eax: 0x9000,
            ..Ret::default()
        });
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 1);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x00ec_43fb, |_, a| Ret {
            eax: if a[0] < 0x2000 { a[0] + 0x100 } else { 0 },
            ..Ret::default()
        });
        e.register(0x0084_e3a0, |_, _| Ret {
            eax: 0xbeef,
            ..Ret::default()
        });
        for address in [0x005b_5e40u32, 0x0048_4ab0, 0x0048_aa80] {
            e.register(address, |_, _| Ret::default());
        }
        form
    }

    #[test]
    fn stat_init_item_resolves_the_sound_or_warns() {
        for (loaded, stored, warns) in
            [(0u32, 0u32, false), (0x10, 0x111, false), (0x3000, 0, true)]
        {
            let mut e = Engine::new();
            let form = sound_case(&mut e, false);
            e.mem.set_u32(form + 0x54, loaded);
            e.call_log = Some(vec![]);
            e.call(0x0051_b530, &args![form]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(form + 0x54), stored);
            assert_eq!(log[2], (0x0048_aa80, vec![form + 0x30, form]));
            let warnings: Vec<_> = log.iter().filter(|call| call.0 == 0x005b_5e40).collect();
            if warns {
                assert_eq!(warnings[0].1, vec![0x0102_bbb0, 0x3001, 0x7777, 0xbeef]);
            } else {
                assert!(warnings.is_empty());
            }
            let casts: Vec<_> = log.iter().filter(|call| call.0 == 0x00ec_43fb).collect();
            assert_eq!(casts.len(), usize::from(loaded != 0));
            if loaded != 0 {
                assert_eq!(casts[0].1[2..], [0x0118_3028, 0x0118_32fc, 0]);
            }
            assert_eq!(log.last().unwrap(), &(0x0048_4ab0, vec![form, 1]));
        }
        let mut e = Engine::new();
        let form = sound_case(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0051_b530, &args![form]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
