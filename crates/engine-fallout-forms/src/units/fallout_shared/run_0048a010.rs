//! `fallout shared/run_0048a010` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0048a0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00488ff0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0048a0a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0048_8ff0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0048a010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`/`thiscall` on `ECX`, no stack words).
/// On a two-word array header (+0 count, +4 element pointer): when the
/// element pointer is set, each non-null element `0..count` is destroyed
/// and freed (`0048a0a0` with flags 1, its result unused), the element block is
/// freed (`00401030`) and the pointer cleared; the count is cleared in any case.
pub fn fn_0048a010(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 4) != 0 {
        let mut index: u32 = 0;
        while index < e.mem.u32(base) {
            let element = e
                .mem
                .u32(e.mem.u32(base + 4).wrapping_add(index.wrapping_mul(4)));
            if element != 0 {
                fn_0048a0a0(e, Ptr::new(element), 1);
            }
            index = index.wrapping_add(1);
        }
        let block = e.mem.u32(base + 4);
        e.call(0x0040_1030, &args![block]);
        e.mem.set_u32(base + 4, 0);
    }
    e.mem.set_u32(base, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_a0a0, fn_0048a0a0(Ptr, u32) -> Ptr),
        entry!(0x0048_a010, fn_0048a010(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0048a0a0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0048_8ff0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0048_a0a0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0048_a0a0)
                .collect();
            let mut expected = vec![(0x0048_8ff0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b025_tests {
    use super::*;

    #[test]
    fn fn_0048a010_destroys_each_element_then_frees_the_block() {
        let mut e = Engine::new();
        e.register(0x0048_8ff0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        let block = e.mem.alloc(12);
        e.mem.set_u32(block, 0x500);
        e.mem.set_u32(block + 8, 0x700);
        e.mem.set_u32(this, 3);
        e.mem.set_u32(this + 4, block);
        e.call_log = Some(vec![]);
        e.call(0x0048_a010, &args![this]);
        let log: Vec<(u32, Vec<u32>)> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != 0x0048_a010)
            .collect();
        assert_eq!(
            log,
            vec![
                (0x0048_8ff0, vec![0x500]),
                (0x0040_1030, vec![0x500]),
                (0x0048_8ff0, vec![0x700]),
                (0x0040_1030, vec![0x700]),
                (0x0040_1030, vec![block]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        // Without elements only the count is cleared.
        e.mem.set_u32(this, 5);
        e.call_log = Some(vec![]);
        e.call(0x0048_a010, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this), 0);
    }
}
