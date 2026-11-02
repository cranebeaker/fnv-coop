//! `Havok SDK/run_00d2d3e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d2d3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, seven words, the first never read; the names are descriptions, not
/// confirmed; the address is used as a function pointer by `00d2d9d0`): asks the object at the
/// word `*holder` (its slot `0x10`) for a value and builds a local record of eight words: `+0` =
/// `second`, `+4` = `holder`, `+0xc` = the value, `+0x10` = `fourth`, `+0x14` = `fifth`, `+0x18` =
/// `sixth` (the words at `+8` and `+0x1c` are never written by the exe: zero here); then answers
/// the result of `00da48e0(first, &record)` (cdecl).
pub fn fn_00d2d3e0(
    e: &mut Engine,
    _unused_1: u32,
    first: u32,
    second: u32,
    holder: Ptr,
    fourth: u32,
    fifth: u32,
    sixth: u32,
) -> u32 {
    let object = e.mem.u32(holder.addr());
    let value = e.vcall(object, 0x10, &args![]).u32();
    e.with_stack(0x20, |e, record| {
        let r = record.addr();
        e.mem.set_u32(r, second);
        e.mem.set_u32(r + 0x10, fourth);
        e.mem.set_u32(r + 0x14, fifth);
        e.mem.set_u32(r + 0xc, value);
        e.mem.set_u32(r + 0x18, sixth);
        e.mem.set_u32(r + 4, holder.addr());
        e.call(0x00da_48e0, &args![first, record]).u32()
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d2_d3e0, fn_00d2d3e0(u32, u32, u32, Ptr, u32, u32, u32) -> u32)]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00d2d3e0_passes_the_record_to_00da48e0() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, 0x0900_0071);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(holder, object);
        e.register(0x0900_0071, |_, a| {
            assert_eq!(a.len(), 1);
            Ret {
                eax: 0x5555,
                ..Ret::default()
            }
        });
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00da_48e0, move |e, a| {
            let mut words = vec![a[0]];
            for offset in 0..8 {
                words.push(e.mem.u32(a[1] + offset * 4));
            }
            log.borrow_mut().push(words);
            Ret {
                eax: 0x9999,
                ..Ret::default()
            }
        });
        let result = e
            .call(
                0x00d2_d3e0,
                &args![0u32, 0x11u32, 0x22u32, holder, 0x44u32, 0x55u32, 0x66u32],
            )
            .u32();
        assert_eq!(result, 0x9999);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(
            seen[0],
            vec![0x11, 0x22, holder, 0, 0x5555, 0x44, 0x55, 0x66, 0]
        );
    }
}
