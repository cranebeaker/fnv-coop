//! `(unplaced)/run_006dba30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Callees: `00845460` (`thiscall`, no arguments, answers an index), `006dc3a0`
/// (`thiscall`, `(&slot, 1)`) and the slot's `0069a690` (`thiscall`, `(value)`).
const NEW_INDEX_00845460: u32 = 0x0084_5460;
const SLOT_PREPARE_006DC3A0: u32 = 0x006d_c3a0;
const SLOT_SET_0069A690: u32 = 0x0069_a690;

// Translated from 006dba30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(value)`; on an array of 8-byte slots whose buffer is
/// at `this + 4`): a new index from `00845460`, its slot prepared (`006dc3a0(&slot, 1)`)
/// and given `value` (`0069a690`). Answers the index.
pub fn fn_006dba30(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    let index = e.call(NEW_INDEX_00845460, &args![this]).u32();
    let slot = |e: &Engine| {
        e.mem
            .u32(this.addr() + 4)
            .wrapping_add(index.wrapping_mul(8))
    };
    let prepared = slot(e);
    e.call(SLOT_PREPARE_006DC3A0, &args![this, prepared, 1u32]);
    let target = slot(e);
    e.call(SLOT_SET_0069A690, &args![target, value]);
    index
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006dba30, fn_006dba30(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_006dba30_fills_the_new_slot() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(NEW_INDEX_00845460, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register(SLOT_PREPARE_006DC3A0, |_, _| Ret::default());
        e.register(SLOT_SET_0069A690, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x7000);
        assert_eq!(e.call(0x006dba30, &args![this, 0x55u32]).u32(), 3);
        let log = e.call_log.take().unwrap();
        let tail: Vec<_> = log.into_iter().skip(1).collect();
        assert_eq!(
            tail,
            vec![
                (NEW_INDEX_00845460, vec![this]),
                (SLOT_PREPARE_006DC3A0, vec![this, 0x7018, 1]),
                (SLOT_SET_0069A690, vec![0x7018, 0x55])
            ]
        );
    }
}
