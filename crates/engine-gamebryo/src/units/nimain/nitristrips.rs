//! `NiMain/nitristrips.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a71e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStrips::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a897c0`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f4a20`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_tri_strips_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a8_97c0;
    const CLASS_NAME_POINTER: u32 = 0x011f_4a20;
    const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;
    e.call(PARENT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CLASS_NAME_POINTER);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// `NiTriStrips::ms_RTTI` (the class's `NiRTTI` record).
const NI_TRI_STRIPS_RTTI: u32 = 0x011f_4a20;
/// `NiTriStrips`'s vtable (RTTI `.?AVNiTriStrips@@`).
const NI_TRI_STRIPS_VTABLE: u32 = 0x0109_cd44;
/// `NiGeometry::RenderImmediate` (`00a80610`, `RET 4`).
const GEOMETRY_RENDER_IMMEDIATE: u32 = 0x00a8_0610;
/// The renderer's word at +0x200 and byte at +0x208 that
/// `OnlyRenderImmediate` tests (unnamed: the renderer class is not in this
/// unit).
const RENDERER_MODE: u32 = 0x200;
const RENDERER_IMMEDIATE: u32 = 0x208;
/// The renderer's vtable slot +0x1B8 that draws a tri-strips object.
const RENDERER_DRAW_TRI_STRIPS_SLOT: u32 = 0x1b8;
/// `NiNew` (`00aa13e0`, `cdecl (size)`).
const NI_NEW: u32 = 0x00aa_13e0;
/// The constructor run on the clone (`00a89740`, `thiscall`, no
/// arguments; unnamed in the engine map).
const CLONE_BASE_CONSTRUCTOR: u32 = 0x00a8_9740;
/// `CopyMembers` (`00a9b140`, `RET 8`; the engine map's name,
/// `NiParticleMeshes::CopyMembers`, is a folded body).
const COPY_MEMBERS: u32 = 0x00a9_b140;
/// `NiTriStripsData`'s constructor (`00a75dc0`, `thiscall`, 11 words,
/// answers `this`).
const TRI_STRIPS_DATA_CONSTRUCTOR: u32 = 0x00a7_5dc0;
/// `NiTriBasedGeom::NiTriBasedGeom` (`00a89720`, `RET 4`: the data).
const TRI_BASED_GEOM_CONSTRUCTOR: u32 = 0x00a8_9720;

// Translated from 00a71d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStrips::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_tri_strips_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TRI_STRIPS_RTTI)
}

// Translated from 00a71d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStrips::OnlyRenderImmediate` (Xbox PDB): `NiGeometry::RenderImmediate`
/// on `renderer`, then, when the renderer's word at +0x200 is 1 or 2 and its
/// byte at +0x208 is 1, the renderer's vtable slot +0x1B8 with `this`.
pub fn ni_tri_strips_only_render_immediate(e: &mut Engine, this: Ptr, renderer: Ptr) {
    e.call(GEOMETRY_RENDER_IMMEDIATE, &args![this, renderer]);
    let mode = e.mem.u32(renderer.addr().wrapping_add(RENDERER_MODE));
    if (mode == 1 || mode == 2) && e.mem.u8(renderer.addr().wrapping_add(RENDERER_IMMEDIATE)) == 1 {
        e.vcall(renderer.addr(), RENDERER_DRAW_TRI_STRIPS_SLOT, &args![this]);
    }
}

// Translated from 00a71db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStrips::CreateClone` (Xbox PDB): a new 0xC4-byte object (`NiNew`);
/// when it was allocated, the constructor `00a89740` and the class's vtable.
/// `CopyMembers` (`this`, clone, `cloning`) runs either way (with a null
/// clone when the allocation failed). Answers the clone.
pub fn ni_tri_strips_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = e.call(NI_NEW, &args![0xc4u32]).u32();
    if clone != 0 {
        e.call(CLONE_BASE_CONSTRUCTOR, &args![clone]);
        e.mem.set_u32(clone, NI_TRI_STRIPS_VTABLE);
    }
    e.call(COPY_MEMBERS, &args![this, clone, cloning]);
    Ptr::new(clone)
}

// Translated from 00a71cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStrips::NiTriStrips_ov2` (Xbox PDB): `NiTriBasedGeom`'s constructor
/// with `data`, then the class's vtable. Answers `this`.
pub fn ni_tri_strips_ni_tri_strips_ov2(e: &mut Engine, this: Ptr, data: Ptr) -> Ptr {
    e.call(TRI_BASED_GEOM_CONSTRUCTOR, &args![this, data]);
    e.mem.set_u32(this.addr(), NI_TRI_STRIPS_VTABLE);
    this
}

// Translated from 00a71c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStrips::NiTriStrips` (Xbox PDB): a new 0x50-byte `NiTriStripsData`
/// (`NiNew`) constructed from the eleven words (`00a75dc0`, which answers the
/// data), given to `NiTriBasedGeom`'s constructor (a null data when the
/// allocation failed, without constructing), then the class's vtable.
/// Answers `this`.
pub fn ni_tri_strips_ni_tri_strips(
    e: &mut Engine,
    this: Ptr,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
    word_7: u32,
    word_8: u32,
    word_9: u32,
    word_10: u32,
    word_11: u32,
) -> Ptr {
    let block = e.call(NI_NEW, &args![0x50u32]).u32();
    let data = if block != 0 {
        e.call(
            TRI_STRIPS_DATA_CONSTRUCTOR,
            &args![
                block, word_1, word_2, word_3, word_4, word_5, word_6, word_7, word_8, word_9,
                word_10, word_11
            ],
        )
        .u32()
    } else {
        0
    };
    e.call(TRI_BASED_GEOM_CONSTRUCTOR, &args![this, data]);
    e.mem.set_u32(this.addr(), NI_TRI_STRIPS_VTABLE);
    this
}

/// The renderer's vtable slot +0x1B0 that `RenderImmediate` calls for a tri-strips object.
const RENDERER_RENDER_TRI_STRIPS_SLOT: u32 = 0x1b0;
/// `NiGeometry::m_spModelData` (Xbox PDB) +0xB8, the geometry data's vertex array
/// (`NiGeometryData::m_pkVertex`, +0x20, twelve bytes per vertex) and the data's vtable slot
/// +0xAC that gives a triangle's three vertex indices (thiscall: the triangle, then three
/// out pointers to 16-bit indices; `NiTriStripsData::GetTriangleIndices` in the Xbox PDB's
/// slot order).
const MODEL_DATA: u32 = 0xb8;
const DATA_VERTICES: u32 = 0x20;
const DATA_TRIANGLE_INDICES_SLOT: u32 = 0xac;

// Translated from 00a71d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStrips::RenderImmediate` (Xbox PDB; thiscall, `RET 4`): `NiGeometry::RenderImmediate`
/// on `renderer`, then, when the renderer's word at +0x200 is 1 or 2 and its byte at +0x208
/// is 1, the renderer's vtable slot +0x1B0 with `this`.
pub fn ni_tri_strips_render_immediate(e: &mut Engine, this: Ptr, renderer: Ptr) {
    e.call(GEOMETRY_RENDER_IMMEDIATE, &args![this, renderer]);
    let mode = e.mem.u32(renderer.addr().wrapping_add(RENDERER_MODE));
    if (mode == 1 || mode == 2) && e.mem.u8(renderer.addr().wrapping_add(RENDERER_IMMEDIATE)) == 1 {
        e.vcall(
            renderer.addr(),
            RENDERER_RENDER_TRI_STRIPS_SLOT,
            &args![this],
        );
    }
}

// Translated from 00a71e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStrips::GetModelTriangle` (Xbox PDB; thiscall, `RET 0x10`: the triangle and three
/// out pointers): the model data's vertex array is read first, then the data's slot +0xAC
/// gives the triangle's three indices (the first written over the triangle argument's own
/// stack word, the other two into locals), and each out pointer gets the address of its
/// vertex (twelve bytes per vertex).
pub fn ni_tri_strips_get_model_triangle(
    e: &mut Engine,
    this: Ptr,
    triangle: u32,
    first: u32,
    second: u32,
    third: u32,
) {
    let data = e.mem.u32(this.addr() + MODEL_DATA);
    let vertices = e.mem.u32(data + DATA_VERTICES);
    e.with_stack(12, |e, frame| {
        // The triangle's stack word, then the two locals.
        let slot = frame.addr();
        e.mem.set_u32(slot, triangle);
        e.vcall(
            data,
            DATA_TRIANGLE_INDICES_SLOT,
            &args![triangle, slot, slot + 4, slot + 8],
        );
        for (index_slot, out) in [(slot, first), (slot + 4, second), (slot + 8, third)] {
            let index = u32::from(e.mem.u16(index_slot));
            e.mem.set_u32(out, vertices.wrapping_add(index * 12));
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a71e70, ni_tri_strips_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00a71d20, ni_tri_strips_get_rtti() -> Ptr),
        entry!(0x00a71d70, ni_tri_strips_only_render_immediate(Ptr, Ptr)),
        entry!(0x00a71db0, ni_tri_strips_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a71cc0, ni_tri_strips_ni_tri_strips_ov2(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a71c40,
            ni_tri_strips_ni_tri_strips(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
        entry!(0x00a7_1d30, ni_tri_strips_render_immediate(Ptr, Ptr)),
        entry!(
            0x00a7_1e00,
            ni_tri_strips_get_model_triangle(Ptr, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_tri_strips_get_viewer_strings_appends_the_class_name_growing_a_full_array() {
        const PARENT: u32 = 0x00a8_97c0;
        const NAME_POINTER: u32 = 0x011f_4a20;
        const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
        const GROW: u32 = 0x0096_ad30;
        const SET_AT: u32 = 0x0096_ae90;
        // (capacity, size): room left, then a full array.
        for (capacity, size) in [(4u16, 2u16), (3, 3)] {
            let mut e = Engine::new();
            e.map(NAME_POINTER, 4);
            e.set_global(NAME_POINTER, 0x5151_0001u32);
            let this = Ptr::new(e.mem.alloc(0x40));
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, size);
            e.mem.set_u16(strings + 0xe, 5);
            e.register(PARENT, |_, _| Ret::default());
            e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
                eax: 0x7777_0001,
                ..Ret::default()
            });
            e.register(GROW, |_, _| Ret::default());
            let stored = Rc::new(Cell::new(0u32));
            let seen = stored.clone();
            e.register_double(SET_AT, move |e, a| {
                seen.set(e.mem.u32(a[2]));
                Ret::default()
            });
            e.call_log = Some(vec![]);
            ni_tri_strips_get_viewer_strings(&mut e, this, Ptr::new(strings));
            let log = e.call_log.take().unwrap();
            let order: Vec<u32> = log.iter().map(|c| c.0).collect();
            let size = u32::from(size);
            if capacity > 3 {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, SET_AT]);
            } else {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, GROW, SET_AT]);
                let grow = log.iter().find(|c| c.0 == GROW).unwrap();
                assert_eq!(grow.1, [strings, 5 + size]);
            }
            let parent = log.iter().find(|c| c.0 == PARENT).unwrap();
            assert_eq!(parent.1, [this.addr(), strings]);
            let name = log.iter().find(|c| c.0 == NI_GET_VIEWER_STRING).unwrap();
            assert_eq!(name.1, [0x5151_0001]);
            let set = log.iter().find(|c| c.0 == SET_AT).unwrap();
            assert_eq!(&set.1[..2], [strings, size]);
            assert_eq!(stored.get(), 0x7777_0001);
        }
    }
}

#[cfg(test)]
mod b014_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn ni_tri_strips_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a71d20, &args![]).u32(), NI_TRI_STRIPS_RTTI);
    }

    #[test]
    fn only_render_immediate_draws_for_modes_1_and_2_when_immediate() {
        const DRAW: u32 = 0x7200_0000;
        for (mode, flag, draws) in [
            (1u32, 1u8, true),
            (2, 1, true),
            (3, 1, false),
            (1, 2, false),
        ] {
            let mut e = Engine::new();
            e.register(GEOMETRY_RENDER_IMMEDIATE, |_, _| Ret::default());
            e.register(DRAW, |_, _| Ret::default());
            e.put_vtable(0x7000_0000, &[0; 0x1bc / 4]);
            e.mem.set_u32(0x7000_0000 + 0x1b8, DRAW);
            let renderer = e.mem.alloc(0x210);
            e.mem.set_u32(renderer, 0x7000_0000);
            e.mem.set_u32(renderer + 0x200, mode);
            e.mem.set_u8(renderer + 0x208, flag);
            e.call_log = Some(vec![]);
            e.call(0x00a71d70, &args![0x1234u32, renderer]);
            assert_eq!(
                calls(&e, GEOMETRY_RENDER_IMMEDIATE),
                vec![vec![0x1234, renderer]]
            );
            let expected = if draws {
                vec![vec![renderer, 0x1234]]
            } else {
                vec![]
            };
            assert_eq!(calls(&e, DRAW), expected);
        }
    }

    #[test]
    fn create_clone_constructs_when_allocated_and_copies_either_way() {
        for allocated in [true, false] {
            let mut e = Engine::new();
            let block = if allocated { e.mem.alloc(0xc4) } else { 0 };
            e.map(0x7300_0000, 4);
            e.mem.set_u32(0x7300_0000, block);
            e.register(NI_NEW, |e, _| Ret {
                eax: e.mem.u32(0x7300_0000),
                ..Ret::default()
            });
            e.register(CLONE_BASE_CONSTRUCTOR, |_, _| Ret::default());
            e.register(COPY_MEMBERS, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            let clone = e.call(0x00a71db0, &args![0x1000u32, 0x2000u32]).u32();
            assert_eq!(clone, block);
            assert_eq!(calls(&e, NI_NEW), vec![vec![0xc4]]);
            assert_eq!(calls(&e, COPY_MEMBERS), vec![vec![0x1000, block, 0x2000]]);
            if allocated {
                assert_eq!(calls(&e, CLONE_BASE_CONSTRUCTOR), vec![vec![block]]);
                assert_eq!(e.mem.u32(block), NI_TRI_STRIPS_VTABLE);
            } else {
                assert!(calls(&e, CLONE_BASE_CONSTRUCTOR).is_empty());
            }
        }
    }

    #[test]
    fn constructor_from_data_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(TRI_BASED_GEOM_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0xc4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a71cc0, &args![this, 0x5555u32]).u32(), this);
        assert_eq!(
            calls(&e, TRI_BASED_GEOM_CONSTRUCTOR),
            vec![vec![this, 0x5555]]
        );
        assert_eq!(e.mem.u32(this), NI_TRI_STRIPS_VTABLE);
    }

    #[test]
    fn constructor_builds_the_data_from_the_eleven_words() {
        for allocated in [true, false] {
            let mut e = Engine::new();
            let block = if allocated { e.mem.alloc(0x50) } else { 0 };
            e.map(0x7300_0000, 4);
            e.mem.set_u32(0x7300_0000, block);
            e.register(NI_NEW, |e, _| Ret {
                eax: e.mem.u32(0x7300_0000),
                ..Ret::default()
            });
            e.register(TRI_STRIPS_DATA_CONSTRUCTOR, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
            e.register(TRI_BASED_GEOM_CONSTRUCTOR, |_, _| Ret::default());
            let this = e.mem.alloc(0xc4);
            let mut words = vec![this];
            words.extend(1..=11u32);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00a71c40, &words).u32(), this);
            assert_eq!(calls(&e, NI_NEW), vec![vec![0x50]]);
            if allocated {
                let mut expected = vec![block];
                expected.extend(1..=11u32);
                assert_eq!(calls(&e, TRI_STRIPS_DATA_CONSTRUCTOR), vec![expected]);
            } else {
                assert!(calls(&e, TRI_STRIPS_DATA_CONSTRUCTOR).is_empty());
            }
            assert_eq!(
                calls(&e, TRI_BASED_GEOM_CONSTRUCTOR),
                vec![vec![this, block]]
            );
            assert_eq!(e.mem.u32(this), NI_TRI_STRIPS_VTABLE);
        }
    }
}

#[cfg(test)]
mod n020_tests {
    use super::*;

    const RENDERER_VTABLE: u32 = 0x0710_0000;
    const RENDER_DOUBLE: u32 = 0x0720_0000;
    const DATA_VTABLE: u32 = 0x0730_0000;
    const INDICES_DOUBLE: u32 = 0x0740_0000;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn render_immediate_asks_renderers_in_modes_1_and_2() {
        let mut e = Engine::new();
        e.register(GEOMETRY_RENDER_IMMEDIATE, |_, _| Ret::default());
        e.register(RENDER_DOUBLE, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x200 / 4];
        slots[RENDERER_RENDER_TRI_STRIPS_SLOT as usize / 4] = RENDER_DOUBLE;
        e.put_vtable(RENDERER_VTABLE, &slots);
        let renderer = e.mem.alloc(0x210);
        e.mem.set_u32(renderer, RENDERER_VTABLE);
        let this = e.mem.alloc(0xc4);
        e.call_log = Some(vec![]);
        for (mode, flag) in [(1u32, 1u8), (2, 1), (0, 1), (3, 1), (1, 0)] {
            e.mem.set_u32(renderer + RENDERER_MODE, mode);
            e.mem.set_u8(renderer + RENDERER_IMMEDIATE, flag);
            e.call(0x00a7_1d30, &args![this, renderer]);
        }
        assert_eq!(calls_to(&e, GEOMETRY_RENDER_IMMEDIATE).len(), 5);
        assert_eq!(calls_to(&e, RENDER_DOUBLE), vec![vec![renderer, this]; 2]);
    }

    #[test]
    fn model_triangle_points_at_the_three_vertices() {
        let mut e = Engine::new();
        e.register(INDICES_DOUBLE, |e, a| {
            // The first out pointer is the triangle's own word.
            assert_eq!(e.mem.u32(a[2]), a[1]);
            e.mem.set_u16(a[2], 4);
            e.mem.set_u16(a[3], 0xffff);
            e.mem.set_u16(a[4], 2);
            Ret::default()
        });
        let mut slots = vec![0u32; 0xb0 / 4];
        slots[DATA_TRIANGLE_INDICES_SLOT as usize / 4] = INDICES_DOUBLE;
        e.put_vtable(DATA_VTABLE, &slots);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data, DATA_VTABLE);
        e.mem.set_u32(data + DATA_VERTICES, 0x0100_0000);
        let this = e.mem.alloc(0xc4);
        e.mem.set_u32(this + MODEL_DATA, data);
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        e.call(
            0x00a7_1e00,
            &args![this, 0x0003_0007u32, out, out + 4, out + 8],
        );
        assert_eq!(calls_to(&e, INDICES_DOUBLE)[0][..2], [data, 0x0003_0007]);
        assert_eq!(e.mem.u32(out), 0x0100_0000 + 4 * 12);
        assert_eq!(e.mem.u32(out + 4), 0x0100_0000 + 0xffff * 12);
        assert_eq!(e.mem.u32(out + 8), 0x0100_0000 + 2 * 12);
    }
}
