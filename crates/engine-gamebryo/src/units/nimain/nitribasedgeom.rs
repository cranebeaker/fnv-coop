//! `NiMain/nitribasedgeom.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTriBasedGeom::ms_RTTI` (the class's `NiRTTI` record).
const NI_TRI_BASED_GEOM_RTTI: u32 = 0x011f_5b70;

// Translated from 00a89780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriBasedGeom::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_tri_based_geom_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TRI_BASED_GEOM_RTTI)
}

// Translated from 00a897c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriBasedGeom::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a80e70`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f5b70`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_tri_based_geom_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a8_0e70;
    const CLASS_NAME_POINTER: u32 = 0x011f_5b70;
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

/// `NiGeometry::NiGeometry` (engine map, `00a804e0`, `thiscall`, `RET 4`: the model data).
const NI_GEOMETRY_CONSTRUCT: u32 = 0x00a8_04e0;
/// `NiTriBasedGeom`'s vtable (`0109ed94`, named by its RTTI locator).
const NI_TRI_BASED_GEOM_VTABLE: u32 = 0x0109_ed94;

// Translated from 00a89720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriBasedGeom::NiTriBasedGeom` (Xbox PDB): `thiscall`, `RET 4`, one word (the model
/// data). Runs `NiGeometry::NiGeometry` (`00a804e0`) with it, sets the class's vtable
/// (`0109ed94`) and answers `this`.
pub fn ni_tri_based_geom_ni_tri_based_geom(e: &mut Engine, this: Ptr, model_data: Ptr) -> Ptr {
    e.call(NI_GEOMETRY_CONSTRUCT, &args![this, model_data]);
    e.mem.set_u32(this.addr(), NI_TRI_BASED_GEOM_VTABLE);
    this
}

/// `NiGeometry::~NiGeometry` (`00a80580`, thiscall) and the sized delete (`00aa1460`,
/// cdecl `(pointer, size)`).
const NI_GEOMETRY_DESTRUCT: u32 = 0x00a8_0580;
const SIZED_DELETE: u32 = 0x00aa_1460;

// Translated from 00a89790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the body is `NiTriBasedGeom`'s scalar deleting destructor
/// (thiscall, `RET 4`): sets the class's vtable (`0109ed94`), runs
/// `NiGeometry::~NiGeometry`, then frees the 0xC4-byte object (`00aa1460`) when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_00a89790(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_TRI_BASED_GEOM_VTABLE);
    e.call(NI_GEOMETRY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xc4u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a89780, ni_tri_based_geom_get_rtti() -> Ptr),
        entry!(0x00a897c0, ni_tri_based_geom_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00a8_9720, ni_tri_based_geom_ni_tri_based_geom(Ptr, Ptr) -> Ptr),
        entry!(0x00a8_9790, fn_00a89790(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_tri_based_geom_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a89780, &args![]).u32(), NI_TRI_BASED_GEOM_RTTI);
    }

    #[test]
    fn ni_tri_based_geom_get_viewer_strings_appends_the_class_name_growing_a_full_array() {
        const PARENT: u32 = 0x00a8_0e70;
        const NAME_POINTER: u32 = 0x011f_5b70;
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
            ni_tri_based_geom_get_viewer_strings(&mut e, this, Ptr::new(strings));
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

    #[test]
    fn ni_tri_based_geom_ni_tri_based_geom_runs_the_base_then_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(NI_GEOMETRY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        let this = e.mem.alloc(0xc4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_9720, &args![this, 0x5000u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_GEOMETRY_CONSTRUCT)
            .collect();
        assert_eq!(base.len(), 1);
        assert_eq!(base[0].1, [this, 0x5000]);
        assert_eq!(e.mem.u32(this), NI_TRI_BASED_GEOM_VTABLE);
    }
}

#[cfg(test)]
mod n020_tests {
    use super::*;

    #[test]
    fn deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        e.register(NI_GEOMETRY_DESTRUCT, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0xc4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_9790, &args![this, 0xfeu32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_TRI_BASED_GEOM_VTABLE);
        e.call(0x00a8_9790, &args![this, 1u32]);
        let calls = |addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .iter()
                .flatten()
                .filter(|(a, _)| *a == addr)
                .map(|(_, w)| w.clone())
                .collect()
        };
        assert_eq!(calls(NI_GEOMETRY_DESTRUCT), vec![vec![this]; 2]);
        assert_eq!(calls(SIZED_DELETE), vec![vec![this, 0xc4]]);
    }
}
