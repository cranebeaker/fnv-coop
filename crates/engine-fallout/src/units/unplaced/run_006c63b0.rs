//! `(unplaced)/run_006c63b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_006c63e0` calls on the member (`006c72f0`).
const CALLEE_006C72F0: u32 = 0x006c_72f0;

// Translated from 006c63e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `006c72f0` with `ECX` = `this + 0xc` and passes on its pointer result.
pub fn fn_006c63e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CALLEE_006C72F0, &args![this.byte_add(0xc)]).ptr()
}

/// `00438af0` (`thiscall`, no stack word): empties the map.
const EMPTY_MAP_00438AF0: u32 = 0x0043_8af0;
/// `00aa10f0` (`cdecl`, one word): frees a block.
const FREE_BLOCK_00AA10F0: u32 = 0x00aa_10f0;

// Translated from 006c63b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): a map destructor. Sets the vtable
/// `0106c654`, empties the map (`00438af0`) and frees the bucket block at +8 (`00aa10f0`).
pub fn fn_006c63b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_c654);
    e.call(EMPTY_MAP_00438AF0, &args![this]);
    let block = e.mem.u32(this.addr() + 8);
    e.call(FREE_BLOCK_00AA10F0, &args![block]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006c_63e0, fn_006c63e0(Ptr) -> Ptr),
        entry!(0x006c_63b0, fn_006c63b0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_006c63e0_passes_this_plus_c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_006C72F0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x006c_63e0, &args![0x4100u32]).ptr::<()>().addr(),
            0x1459
        );
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_006C72F0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0xc]);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_006c63b0_empties_and_frees_the_buckets() {
        let mut e = Engine::new();
        e.register(EMPTY_MAP_00438AF0, |_, _| Ret::default());
        e.register(FREE_BLOCK_00AA10F0, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x006c_63b0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (EMPTY_MAP_00438AF0, vec![this]),
                (FREE_BLOCK_00AA10F0, vec![0x6000]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0106_c654);
    }
}
