//! `BSMenu/run_00a1eeb0` (Xbox PDB source unit), subsystem `BSMenu`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// The `NiTArray<Tile *, NiTMallocInterface<Tile *>>` vtable (RTTI name
/// `.?AV?$NiTArray@PAVTile@@V?$NiTMallocInterface@PAVTile@@@@@@`).
const TILE_ARRAY_VTABLE: u32 = 0x0109_568c;
/// The Gamebryo `malloc` (`run_00aa1010.rs`), cdecl (size).
const NI_MALLOC: u32 = 0x00aa_1070;

// Translated from 00a1eeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray<Tile *, NiTMallocInterface<Tile *>>` constructor (RTTI of
/// the vtable it stores): `m_usMaxSize` and `m_usGrowBy` from the two
/// arguments (their low 16 bits), `m_usSize` and `m_usESize` cleared, and
/// `m_pBase` a block of `4 * max size` bytes from `00aa1070`, or null for
/// a zero size. Returns `this`.
pub fn fn_00a1eeb0(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u32,
    grow_by: u32,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), TILE_ARRAY_VTABLE);
    e.set(this, NiTArray::m_usMaxSize, max_size as u16);
    e.set(this, NiTArray::m_usGrowBy, grow_by as u16);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    if e.get(this, NiTArray::m_usMaxSize) > 0 {
        let size = u32::from(e.get(this, NiTArray::m_usMaxSize));
        let base = e.call(NI_MALLOC, &args![size << 2]).u32();
        e.set(this, NiTArray::m_pBase, base);
    } else {
        e.set(this, NiTArray::m_pBase, 0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a1_eeb0,
        fn_00a1eeb0(Ptr<NiTArray>, u32, u32) -> Ptr<NiTArray>
    )]
}

#[cfg(test)]
mod tile_array_tests {
    use super::*;

    #[test]
    fn fn_00a1eeb0_sets_the_sizes_and_allocates() {
        let mut e = Engine::new();
        e.register(NI_MALLOC, |_, a| Ret {
            eax: 0x4000 + a[0],
            ..Ret::default()
        });
        let this: Ptr<NiTArray> = Ptr::new(e.mem.alloc(0x10));
        for offset in (0..0x10).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a1_eeb0, &args![this, 0x0001_0003u32, 0x0002_0005u32])
                .u32(),
            this.addr()
        );
        assert_eq!(e.mem.u32(this.addr()), TILE_ARRAY_VTABLE);
        assert_eq!(e.get(this, NiTArray::m_usMaxSize), 3);
        assert_eq!(e.get(this, NiTArray::m_usGrowBy), 5);
        assert_eq!(e.get(this, NiTArray::m_usSize), 0);
        assert_eq!(e.get(this, NiTArray::m_usESize), 0);
        assert_eq!(e.get(this, NiTArray::m_pBase), 0x400c);
        let mallocs = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == NI_MALLOC)
            .count();
        assert_eq!(mallocs, 1);

        e.call_log = Some(vec![]);
        e.call(0x00a1_eeb0, &args![this, 0x0001_0000u32, 1u32]);
        assert_eq!(e.get(this, NiTArray::m_pBase), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}
