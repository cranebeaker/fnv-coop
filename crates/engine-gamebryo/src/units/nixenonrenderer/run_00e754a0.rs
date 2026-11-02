//! `NiXenonRenderer/run_00e754a0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor (`00a62af0`, unnamed; `thiscall`, no stack words), the class's virtual
/// table (RTTI `NiDX9Renderer`) and the set-up that ends the constructor (`00e6b990`,
/// `nixenonrenderer.cpp`; `thiscall`, no stack words).
const BASE_CONSTRUCT: u32 = 0x00a6_2af0;
const NI_DX9_RENDERER_VTABLE: u32 = 0x010e_e4bc;
const FINISH_00E6B990: u32 = 0x00e6_b990;
/// The bucket allocation `00aa1070` (`cdecl`, size) and `memset` (`00ec61c0`).
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const MEMSET: u32 = 0x00ec_61c0;

/// The renderer's six maps in the exe's order: (offset, base virtual table, final virtual
/// table, bucket count, bytes allocated). The final tables' RTTI: `NiTPointerMap<NiVBBlock *,
/// PrePackObject *>`, `NiTPointerMap<HWND, NiPointer<NiRenderTargetGroup>>`,
/// `NiTPointerMap<NiRenderedTexture *, NiDX9RenderedTextureData *>`,
/// `NiTPointerMap<NiRenderedCubeMap *, NiDX9RenderedCubeMapData *>`,
/// `NiTPointerMap<NiDynamicTexture *, NiDX9DynamicTextureData *>`, `NiTMap<_D3DFORMAT,
/// NiPixelFormat *>`.
type MapSpec = (u32, u32, u32, u32, u32);
const FIRST_MAP: MapSpec = (0x610, 0x010e_da24, 0x010e_dafc, 0x25, 0x94);
const MIDDLE_MAPS: [MapSpec; 4] = [
    (0x890, 0x010e_da44, 0x010e_db1c, 0x25, 0x94),
    (0x8cc, 0x010e_da64, 0x010e_db3c, 0x25, 0x94),
    (0x8dc, 0x010e_da84, 0x010e_db5c, 0x25, 0x94),
    (0x8ec, 0x010e_daa4, 0x010e_db7c, 0x25, 0x94),
];
const LAST_MAP: MapSpec = (0xb14, 0x010e_dac4, 0x010e_db9c, 7, 0x1c);
/// The three small arrays' virtual tables (RTTI `NiTPrimitiveArray<bool (*)(bool, void *)>`,
/// `NiTPrimitiveArray<void *>`, `NiTPrimitiveArray<bool (*)(void *)>`).
const CALLBACK_ARRAY_VTABLE: u32 = 0x010e_de18;
const POINTER_ARRAY_VTABLE: u32 = 0x010e_de20;
const PLAIN_CALLBACK_ARRAY_VTABLE: u32 = 0x010e_de28;

/// The inlined map constructor: base virtual table, bucket count, item count 0, the buckets
/// (`00aa1070` of the pushed size) cleared by `memset` with the count re-read, the final
/// virtual table.
fn construct_map(e: &mut Engine, this: u32, spec: MapSpec) {
    let (offset, base_vtable, vtable, buckets, bytes) = spec;
    let map = this.wrapping_add(offset);
    e.mem.set_u32(map, base_vtable);
    e.mem.set_u32(map.wrapping_add(4), buckets);
    e.mem.set_u32(map.wrapping_add(0xc), 0);
    let table = e.call(ALLOCATE_00AA1070, &args![bytes]).u32();
    e.mem.set_u32(map.wrapping_add(8), table);
    let size = e.mem.u32(map.wrapping_add(4)).wrapping_mul(4);
    e.call(MEMSET, &args![table, 0u32, size]);
    e.mem.set_u32(map, vtable);
}

// Translated from 00e754a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e754a0`): the `NiDX9Renderer` constructor (the RTTI of
/// the virtual table it stores), `thiscall`, no stack words. The base constructor, the class's
/// virtual table, the map at +0x610, +0x630 and +0x870 to +0x87c cleared, the maps at +0x890,
/// +0x8cc, +0x8dc and +0x8ec, the words +0x8fc to +0x93c and +0xad8 cleared, three primitive
/// arrays at +0xad4, +0xae4 and +0xaf4 (+0xb04's virtual table too) with their 16-bit counts
/// (grow steps 1, the rest 0) and element pointers cleared, the format map at +0xb14 (7
/// buckets), then `00e6b990`. Answers `this`.
pub fn fn_00e754a0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, NI_DX9_RENDERER_VTABLE);
    construct_map(e, base, FIRST_MAP);
    e.mem.set_u32(base.wrapping_add(0x630), 0);
    for offset in [0x870u32, 0x874, 0x878, 0x87c] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    for spec in MIDDLE_MAPS {
        construct_map(e, base, spec);
    }
    for offset in [0x8fcu32, 0x908, 0x900, 0x904, 0x914, 0x90c, 0x910] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    for offset in (0x918u32..=0x93c).step_by(4) {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    let word =
        |e: &mut Engine, offset: u32, value: u32| e.mem.set_u32(base.wrapping_add(offset), value);
    let half =
        |e: &mut Engine, offset: u32, value: u16| e.mem.set_u16(base.wrapping_add(offset), value);
    word(e, 0xad8, 0);
    word(e, 0xad4, CALLBACK_ARRAY_VTABLE);
    half(e, 0xadc, 0);
    half(e, 0xae0, 0);
    half(e, 0xae2, 1);
    half(e, 0xade, 0);
    half(e, 0xaee, 0);
    word(e, 0xae8, 0);
    half(e, 0xaec, 0);
    half(e, 0xaf0, 0);
    word(e, 0xae4, POINTER_ARRAY_VTABLE);
    half(e, 0xaf2, 1);
    word(e, 0xaf8, 0);
    word(e, 0xaf4, PLAIN_CALLBACK_ARRAY_VTABLE);
    half(e, 0xb02, 1);
    half(e, 0xb00, 0);
    half(e, 0xafc, 0);
    half(e, 0xafe, 0);
    half(e, 0xb12, 1);
    half(e, 0xb0c, 0);
    half(e, 0xb0e, 0);
    half(e, 0xb10, 0);
    word(e, 0xb08, 0);
    word(e, 0xb04, POINTER_ARRAY_VTABLE);
    construct_map(e, base, LAST_MAP);
    e.call(FINISH_00E6B990, &args![base]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e7_54a0, fn_00e754a0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod b000t_tests {
    use super::*;

    #[test]
    fn fn_00e754a0_builds_the_renderer_maps_and_arrays() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(FINISH_00E6B990, |_, _| Ret::default());
        e.register(ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xb80);
        for i in 0..0xb80 / 4 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00e7_54a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DX9_RENDERER_VTABLE);
        let log = e.call_log.clone().unwrap();
        let allocations: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == ALLOCATE_00AA1070)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(allocations, vec![0x94, 0x94, 0x94, 0x94, 0x94, 0x1c]);
        let clears: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == MEMSET)
            .map(|(_, w)| w[2])
            .collect();
        assert_eq!(clears, vec![0x94, 0x94, 0x94, 0x94, 0x94, 0x1c]);
        for spec in [FIRST_MAP, MIDDLE_MAPS[0], MIDDLE_MAPS[3], LAST_MAP] {
            let map = this + spec.0;
            assert_eq!(e.mem.u32(map), spec.2);
            assert_eq!(e.mem.u32(map + 4), spec.3);
            assert_eq!(e.mem.u32(map + 0xc), 0);
            assert_ne!(e.mem.u32(map + 8), 0xffff_ffff);
        }
        assert_eq!(e.mem.u32(this + 0x630), 0);
        assert_eq!(e.mem.u32(this + 0x93c), 0);
        assert_eq!(e.mem.u32(this + 0xad4), CALLBACK_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(this + 0xae2), 1);
        assert_eq!(e.mem.u32(this + 0xb04), POINTER_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(this + 0xb12), 1);
        assert_eq!(e.mem.u32(this + 0x940), 0xffff_ffff);
        assert_eq!(log.last().unwrap(), &(FINISH_00E6B990, vec![this]));
    }
}
