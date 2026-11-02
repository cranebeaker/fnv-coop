//! `fallout shared/tesloadscreentype.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005a72f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a72f0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESLoadScreenType`'s record reader (`RET 4`: the `TESFile`; the engine
/// map has no name; the form type is not checked): `TESForm::LoadForm`,
/// `00484ab0(0)` and each chunk: `EDID` (vtable +0x134), `OBND` (vtable
/// +0xE0) and `DATA` (0x58 bytes at +0x18, `TESForm::LoadData`); after the
/// last chunk the data's words are swapped (`005a7400`) when the file asks.
/// Returns true.
pub fn fn_005a72f0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            // DATA
            0x4154_4144 => {
                e.call(0x0048_61f0, &args![this, file, base + 0x18, 0x58u32]);
            }
            // EDID
            0x4449_4445 => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(0x0047_2890, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            // OBND
            0x444e_424f => {
                e.vcall(base, 0xe0, &args![file]);
            }
            _ => {}
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    if e.call(0x0040_1680, &args![file]).bool() {
        e.call(0x005a_7400, &args![base + 0x18]);
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005a_72f0, fn_005a72f0(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn load(swap: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x70);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0xe0, 0x7000_00e0);
        e.mem.set_u32(vtable + 0x134, 0x7000_0134);
        e.mem.set_u32(this, vtable);
        let file = e.mem.alloc(0x10);
        let chunks = Rc::new(RefCell::new((
            vec![0x4449_4445u32, 0x444e_424f, 0x4154_4144, 0x5858_5858],
            0usize,
        )));
        let c = chunks.clone();
        e.register_double(0x0047_26b0, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).copied().unwrap_or(0))
        });
        let c = chunks;
        e.register_double(0x0047_26f0, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        e.register(0x0040_1660, |_, _| ret(8));
        if swap {
            e.register(0x0040_1680, |_, _| ret(1));
        } else {
            e.register(0x0040_1680, |_, _| ret(0));
        }
        for addr in [
            0x7000_00e0u32,
            0x7000_0134,
            0x0047_2890,
            0x0048_5110,
            0x0048_4ab0,
            0x0048_61f0,
            0x005a_7400,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        assert!(e.call(0x005a_72f0, &args![this, file]).bool());
        (e, this, file)
    }

    #[test]
    fn load_reads_the_chunks_then_swaps_the_data() {
        let (e, this, file) = load(true);
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, file]]);
        assert_eq!(
            calls(&e, 0x0048_61f0),
            vec![vec![this, file, this + 0x18, 0x58]]
        );
        assert_eq!(calls(&e, 0x0040_1680).len(), 1);
        assert_eq!(calls(&e, 0x005a_7400), vec![vec![this + 0x18]]);

        let (e, _, _) = load(false);
        assert!(calls(&e, 0x005a_7400).is_empty());
    }
}
