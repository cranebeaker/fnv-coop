//! `(unplaced)/run_00505dd0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00505dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`; an `InitItem` shape): unless
/// `004013e0` says otherwise, reads the word `005a29b0` returns for the form; when it is not
/// 0, fixes it up in a stack slot through `00485d50` (cdecl, the slot and the form's file
/// from `00484e60` with -1), looks the result up with `004839c0` (cdecl), casts that with
/// `__RTDynamicCast` (`00ec43fb`: the pointer, 0, `.?AVTESForm@@` at `0x0118_3028`,
/// `.?AVBGSDebris@@` at `0x0118_61f4`, 0) and gives the cast to `00505e50`. Then
/// `00484ab0` with 1.
pub fn fn_00505dd0(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    let word = e.call(0x005a_29b0, &args![this]).u32();
    if word != 0 {
        let id = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), word);
            let file = e.call(0x0048_4e60, &args![this, 0xffff_ffffu32]).u32();
            e.call(0x0048_5d50, &args![slot, file]);
            e.mem.u32(slot.addr())
        });
        let form = e.call(0x0048_39c0, &args![id]).u32();
        let debris = e
            .call(
                0x00ec_43fb,
                &args![form, 0u32, 0x0118_3028u32, 0x0118_61f4u32, 0u32],
            )
            .u32();
        e.call(0x0050_5e50, &args![this, debris]);
    }
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0050_5dd0, fn_00505dd0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00505dd0_resolves_the_debris_unless_checked() {
        for (checked, word) in [(true, 7u32), (false, 0), (false, 7)] {
            let mut e = Engine::new();
            let form = e.mem.alloc(0x40);
            e.register_double(0x0040_13e0, move |_, _| Ret {
                eax: u32::from(checked),
                ..Ret::default()
            });
            e.register_double(0x005a_29b0, move |_, _| Ret {
                eax: word,
                ..Ret::default()
            });
            e.register(0x0048_4e60, |_, _| Ret {
                eax: 0x9000,
                ..Ret::default()
            });
            e.register(0x0048_5d50, |e, a| {
                let value = e.mem.u32(a[0]);
                e.mem.set_u32(a[0], value + 0x100);
                Ret::default()
            });
            e.register(0x0048_39c0, |_, a| Ret {
                eax: a[0] + 0x1000,
                ..Ret::default()
            });
            e.register(0x00ec_43fb, |_, a| Ret {
                eax: a[0] + 1,
                ..Ret::default()
            });
            e.register(0x0050_5e50, |_, _| Ret::default());
            e.register(0x0048_4ab0, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x0050_5dd0, &args![form]);
            let log: Vec<_> = e.call_log.take().unwrap().into_iter().skip(1).collect();
            if checked {
                assert_eq!(log, vec![(0x0040_13e0, vec![form])]);
                continue;
            }
            assert_eq!(log.last().unwrap(), &(0x0048_4ab0, vec![form, 1]));
            let set: Vec<_> = log.iter().filter(|call| call.0 == 0x0050_5e50).collect();
            if word == 0 {
                assert!(set.is_empty());
                assert_eq!(log.len(), 3);
            } else {
                assert_eq!(set[0].1, vec![form, 0x1108]);
                let cast: Vec<_> = log.iter().filter(|call| call.0 == 0x00ec_43fb).collect();
                assert_eq!(cast[0].1, vec![0x1107, 0, 0x0118_3028, 0x0118_61f4, 0]);
                let fixup: Vec<_> = log.iter().filter(|call| call.0 == 0x0048_5d50).collect();
                assert_eq!(fixup[0].1[1], 0x9000);
            }
        }
    }
}
