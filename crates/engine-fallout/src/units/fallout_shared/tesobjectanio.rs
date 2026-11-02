//! `fallout shared/tesobjectanio.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00511a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectANIO::InitItem` (Xbox PDB; `ECX` only, plain `RET`): unless `004013e0` says
/// otherwise, runs `0048aa80` (the model texture swap at +0x18) with the form; when the word
/// at +0x38 (the idle's form ID as loaded) is not 0, fixes it up in a stack slot through
/// `00485d50` (cdecl, the slot and the form's file from `00484e60` with -1), looks it up
/// with `004839c0`, casts it with `__RTDynamicCast` (`00ec43fb`: `.?AVTESForm@@` at
/// `0x0118_3028` to `.?AVTESIdleForm@@` at `0x0118_6a18`) and stores the result at +0x38.
/// Then `00484ab0` with 1.
pub fn tes_object_anio_init_item(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    e.call(0x0048_aa80, &args![base + 0x18, this]);
    let loaded = e.mem.u32(base + 0x38);
    if loaded != 0 {
        let id = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), loaded);
            let file = e.call(0x0048_4e60, &args![this, 0xffff_ffffu32]).u32();
            e.call(0x0048_5d50, &args![slot, file]);
            e.mem.u32(slot.addr())
        });
        let form = e.call(0x0048_39c0, &args![id]).u32();
        let idle = e
            .call(
                0x00ec_43fb,
                &args![form, 0u32, 0x0118_3028u32, 0x0118_6a18u32, 0u32],
            )
            .u32();
        e.mem.set_u32(base + 0x38, idle);
    }
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_1a90, tes_object_anio_init_item(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anio_init_item_resolves_the_idle() {
        for (checked, loaded) in [(true, 5u32), (false, 0), (false, 5)] {
            let mut e = Engine::new();
            let form = e.mem.alloc(0x40);
            e.mem.set_u32(form + 0x38, loaded);
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
                e.mem.set_u32(a[0], id + 0x100);
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
            e.register(0x0048_aa80, |_, _| Ret::default());
            e.register(0x0048_4ab0, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x0051_1a90, &args![form]);
            let log: Vec<_> = e.call_log.take().unwrap().into_iter().skip(1).collect();
            if checked {
                assert_eq!(log.len(), 1);
                assert_eq!(e.mem.u32(form + 0x38), loaded);
                continue;
            }
            assert_eq!(log[1], (0x0048_aa80, vec![form + 0x18, form]));
            assert_eq!(log.last().unwrap(), &(0x0048_4ab0, vec![form, 1]));
            if loaded == 0 {
                assert_eq!(log.len(), 3);
            } else {
                assert_eq!(e.mem.u32(form + 0x38), 0x1106);
                let cast: Vec<_> = log.iter().filter(|call| call.0 == 0x00ec_43fb).collect();
                assert_eq!(cast[0].1, vec![0x1105, 0, 0x0118_3028, 0x0118_6a18, 0]);
            }
        }
    }
}
