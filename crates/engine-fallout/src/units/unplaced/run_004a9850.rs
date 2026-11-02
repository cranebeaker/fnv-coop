//! `(unplaced)/run_004a9850` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable the destructor stores (`0101ecec`; RTTI
/// `.?AV?$NiTMap@PAVNiTexture@@PAUTEX_USER_DATA@@@@`).
const TEXTURE_USER_DATA_MAP_VTABLE: u32 = 0x0101_ecec;
/// `00438af0` (unnamed; `thiscall`, no stack words): the map body's clean-up the
/// destructor runs first, and `004a98b0` (unnamed; `thiscall`, no stack words): the base
/// destructor it runs after.
const MAP_CLEAN_UP: u32 = 0x0043_8af0;
const MAP_BASE_DESTRUCT: u32 = 0x004a_98b0;

// Translated from 004a9850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a9850 - the FS:[0] exception frame (unwind state 0 for the base destructor) is not translated
/// `NiTMap<NiTexture*, TEX_USER_DATA*>`'s destructor (by its vtable's RTTI; the engine map
/// does not name it; `thiscall`, no stack words): stores the class's vtable (`0101ecec`),
/// runs `00438af0`, then the base destructor `004a98b0`.
pub fn fn_004a9850(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TEXTURE_USER_DATA_MAP_VTABLE);
    e.call(MAP_CLEAN_UP, &args![this]);
    e.call(MAP_BASE_DESTRUCT, &args![this]);
}

/// `00aa10f0` (`cdecl`, one word): frees a block.
const FREE: u32 = 0x00aa_10f0;
/// The base vtable [`fn_004a98b0`] stores (`0101ed2c`; RTTI
/// `NiTMapBase<DFALL<NiTMapItem<NiTexture*,TEX_USER_DATA*>>,...>`).
const TEXTURE_USER_DATA_MAP_BASE_VTABLE: u32 = 0x0101_ed2c;

// Translated from 004a98b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` destructor body under [`fn_004a9850`] (by its vtable's RTTI; the engine
/// map does not name it; `ECX`, plain `RET`): stores the base vtable (`0101ed2c`), removes
/// every item (`00438af0`) and frees the bucket array at +8 (`00aa10f0`).
pub fn fn_004a98b0(e: &mut Engine, this: Ptr) {
    e.mem
        .set_u32(this.addr(), TEXTURE_USER_DATA_MAP_BASE_VTABLE);
    e.call(MAP_CLEAN_UP, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(FREE, &args![buckets]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004a_9850, fn_004a9850(Ptr)),
        entry!(0x004a_98b0, fn_004a98b0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructor_sets_the_vtable_then_cleans_up_and_destroys_the_base() {
        let mut e = Engine::new();
        e.register(MAP_CLEAN_UP, |e, a| {
            assert_eq!(e.mem.u32(a[0]), TEXTURE_USER_DATA_MAP_VTABLE);
            Ret::default()
        });
        e.register(MAP_BASE_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x004a_9850, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(MAP_CLEAN_UP, vec![this]), (MAP_BASE_DESTRUCT, vec![this])]
        );
    }
}

#[cfg(test)]
mod base_tests {
    use super::*;

    #[test]
    fn fn_004a98b0_stores_the_base_vtable_and_frees_the_buckets() {
        let mut e = Engine::new();
        e.register(MAP_CLEAN_UP, |e, a| {
            assert_eq!(e.mem.u32(a[0]), TEXTURE_USER_DATA_MAP_BASE_VTABLE);
            Ret::default()
        });
        e.register(FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x5550);
        e.call_log = Some(vec![]);
        e.call(0x004a_98b0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(MAP_CLEAN_UP, vec![this]), (FREE, vec![0x5550])]);
    }
}
