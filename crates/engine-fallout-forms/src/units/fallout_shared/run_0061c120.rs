//! `fallout shared/run_0061c120` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable `0061c120` stores: `NiTLargeArray<TESTopicInfo*,
/// NiTMallocInterface<TESTopicInfo*>>` by its RTTI name.
const TOPIC_INFO_ARRAY_VTABLE: u32 = 0x0104_d448;
/// `0096afc0` (cdecl: count): the storage for that many elements.
const ALLOCATE_ELEMENTS: u32 = 0x0096_afc0;

// Translated from 0061c120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an `NiTLargeArray<TESTopicInfo*>` (the vtable's RTTI name;
/// two words, `RET 8`): the vtable, the first word at +8 and the second at
/// +0x14, +0xc and +0x10 zero, and +4 from `0096afc0` with the word at +8,
/// or 0 when that is 0. Returns `this`. The engine map gives it no name.
pub fn fn_0061c120(e: &mut Engine, this: Ptr, maximum: u32, grow_by: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, TOPIC_INFO_ARRAY_VTABLE);
    e.mem.set_u32(base + 0x8, maximum);
    e.mem.set_u32(base + 0x14, grow_by);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0);
    let maximum = e.mem.u32(base + 0x8);
    if maximum == 0 {
        e.mem.set_u32(base + 0x4, 0);
    } else {
        let storage = e.call(ALLOCATE_ELEMENTS, &args![maximum]).u32();
        e.mem.set_u32(base + 0x4, storage);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0061_c120, fn_0061c120(Ptr, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0061c120_sets_up_the_array() {
        let mut e = Engine::new();
        e.register(ALLOCATE_ELEMENTS, |_, a| Ret {
            eax: 0x9000 + a[0],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x18);
        for i in 0..6 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        assert_eq!(e.call(0x0061_c120, &args![this, 4u32, 2u32]).u32(), this);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![TOPIC_INFO_ARRAY_VTABLE, 0x9004, 4, 0, 0, 2]);
        e.call_log = Some(vec![]);
        e.call(0x0061_c120, &args![this, 0u32, 2u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
        let allocations = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == ALLOCATE_ELEMENTS)
            .count();
        assert_eq!(allocations, 0);
    }
}
