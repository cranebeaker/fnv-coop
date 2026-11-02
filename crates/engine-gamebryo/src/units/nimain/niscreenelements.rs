//! `NiMain/niscreenelements.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiScreenElements::ms_RTTI` (the class's `NiRTTI` record).
const NI_SCREEN_ELEMENTS_RTTI: u32 = 0x011f_5b48;

// Translated from 00a87800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenElements::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_screen_elements_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SCREEN_ELEMENTS_RTTI)
}

// Translated from 00a878b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenElements::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a74690`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f5b48`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_screen_elements_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a7_4690;
    const CLASS_NAME_POINTER: u32 = 0x011f_5b48;
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

/// `NiTriShape::NiTriShape_ov2` (engine map, `00a74480`, `thiscall`, `RET 4`: the data).
const NI_TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4480;
/// `NiScreenElements`'s vtable (`0109e984`, RTTI `.?AVNiScreenElements@@`).
const NI_SCREEN_ELEMENTS_VTABLE: u32 = 0x0109_e984;

// Translated from 00a87810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenElements::NiScreenElements` (Xbox PDB), `thiscall`, one word (`RET 4`, the
/// `NiScreenElementsData`): runs `NiTriShape`'s constructor with the data, sets the vtable and
/// answers `this`.
pub fn ni_screen_elements_ni_screen_elements(e: &mut Engine, this: Ptr, data: Ptr) -> Ptr {
    e.call(NI_TRI_SHAPE_CONSTRUCT, &args![this, data]);
    e.mem.set_u32(this.addr(), NI_SCREEN_ELEMENTS_VTABLE);
    this
}

/// `00a89760` (`thiscall`, no stack words, unnamed in the engine map): the destructor body
/// the scalar deleting destructor runs after storing the vtable, and the sized delete
/// `00aa1460` (`cdecl`: block, size).
const DESTRUCTOR_BODY_00A89760: u32 = 0x00a8_9760;
const SIZED_DELETE_00AA1460: u32 = 0x00aa_1460;

// Translated from 00a87830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenElements::_scalar_deleting_destructor_` (Xbox PDB), `thiscall`, `RET 4`: stores
/// the vtable `0109e984`, runs the destructor body `00a89760` and, when bit 0 of `flags` is
/// set, the sized delete of 0xc4 bytes. Answers `this`.
pub fn ni_screen_elements_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_SCREEN_ELEMENTS_VTABLE);
    e.call(DESTRUCTOR_BODY_00A89760, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE_00AA1460, &args![this, 0xc4u32]);
    }
    this
}

// Translated from 00a87790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenElements::Draw` (Xbox PDB): a tail jump to the object's virtual +0xdc with the
/// same `this` and stack word (the renderer, as its callers push it); the callee's answer is
/// passed on.
pub fn ni_screen_elements_draw(e: &mut Engine, this: Ptr, renderer: Ptr) -> u32 {
    e.vcall(this.addr(), 0xdc, &args![renderer]).u32()
}

/// `00aa13e0` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE: u32 = 0x00aa_13e0;
/// `00a744a0` (`thiscall`, no stack word; unnamed in the engine map): the base constructor
/// the clone starts with.
const CLONE_BASE_CONSTRUCTOR: u32 = 0x00a7_44a0;
/// `00a744c0` (`thiscall`, (clone, cloning process); unnamed in the engine map): the base's
/// member copy.
const CLONE_COPY_MEMBERS: u32 = 0x00a7_44c0;

// Translated from 00a87860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenElements::CreateClone` (Xbox PDB): a new screen elements object (0xc4 bytes
/// from `00aa13e0`, the base constructor `00a744a0`, the vtable `0109e984`) given this
/// one's members (`00a744c0`). When the allocation fails the member copy still runs on
/// null and the answer is null.
pub fn ni_screen_elements_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let clone = e.call(ALLOCATE, &args![0xc4u32]).u32();
    if clone != 0 {
        e.call(CLONE_BASE_CONSTRUCTOR, &args![clone]);
        e.mem.set_u32(clone, NI_SCREEN_ELEMENTS_VTABLE);
        e.call(CLONE_COPY_MEMBERS, &args![this, clone, cloning_process]);
        return Ptr::new(clone);
    }
    e.call(CLONE_COPY_MEMBERS, &args![this, 0u32, cloning_process]);
    Ptr::new(0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_7790, ni_screen_elements_draw(Ptr, Ptr) -> u32),
        entry!(
            0x00a8_7860,
            ni_screen_elements_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00a8_7830,
            ni_screen_elements_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a8_7810,
            ni_screen_elements_ni_screen_elements(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00a87800, ni_screen_elements_get_rtti() -> Ptr),
        entry!(0x00a878b0, ni_screen_elements_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_screen_elements_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a87800, &args![]).u32(), NI_SCREEN_ELEMENTS_RTTI);
    }

    #[test]
    fn ni_screen_elements_get_viewer_strings_appends_the_class_name_growing_a_full_array() {
        const PARENT: u32 = 0x00a7_4690;
        const NAME_POINTER: u32 = 0x011f_5b48;
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
            ni_screen_elements_get_viewer_strings(&mut e, this, Ptr::new(strings));
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
mod b000_tests {
    use super::*;

    #[test]
    fn ni_screen_elements_constructor_passes_the_data_and_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(NI_TRI_SHAPE_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0xc4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_7810, &args![this, 0x5000u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let shape: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_TRI_SHAPE_CONSTRUCT)
            .collect();
        assert_eq!(shape.len(), 1);
        assert_eq!(shape[0].1, vec![this, 0x5000]);
        assert_eq!(e.mem.u32(this), NI_SCREEN_ELEMENTS_VTABLE);
    }
}

#[cfg(test)]
mod batch_b011_tests {
    use super::*;

    #[test]
    fn scalar_deleting_destructor_sets_the_vtable_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        e.register(DESTRUCTOR_BODY_00A89760, |e, a| {
            // The vtable is the class's own when the body runs.
            assert_eq!(e.mem.u32(a[0]), NI_SCREEN_ELEMENTS_VTABLE);
            Ret::default()
        });
        e.register(SIZED_DELETE_00AA1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xc4);
        assert_eq!(e.call(0x00a8_7830, &args![this, 2u32]).u32(), this);
        assert_eq!(e.call(0x00a8_7830, &args![this, 3u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let deletes: Vec<_> = log
            .iter()
            .filter(|c| c.0 == SIZED_DELETE_00AA1460)
            .collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![this, 0xc4]);
        assert_eq!(
            log.iter()
                .filter(|c| c.0 == DESTRUCTOR_BODY_00A89760)
                .count(),
            2
        );
    }
}

#[cfg(test)]
mod n018_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn draw_forwards_to_the_virtual_slot() {
        let mut e = Engine::new();
        e.put_vtable(0x0300_0000, &[0; 0x38]);
        e.mem.set_u32(0x0300_00dc, 0x0300_1000);
        e.register(0x0300_1000, |_, _| 0x55u32.into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xc4);
        e.mem.set_u32(this, 0x0300_0000);
        assert_eq!(e.call(0x00a8_7790, &args![this, 0x4444u32]).u32(), 0x55);
        assert_eq!(calls_to(&e, 0x0300_1000), vec![vec![this, 0x4444]]);
    }

    #[test]
    fn create_clone_builds_and_copies() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CLONE_BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(CLONE_COPY_MEMBERS, |e, a| {
            // The vtable is in place when the members are copied.
            assert_eq!(e.mem.u32(a[1]), NI_SCREEN_ELEMENTS_VTABLE);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a8_7860, &args![0x7000u32, 0x4444u32]).u32();
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0xc4]]);
        assert_eq!(calls_to(&e, CLONE_BASE_CONSTRUCTOR), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, CLONE_COPY_MEMBERS),
            vec![vec![0x7000, clone, 0x4444]]
        );
    }

    #[test]
    fn create_clone_copies_onto_null_when_the_allocation_fails() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.register(CLONE_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_7860, &args![0x7000u32, 0x4444u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, CLONE_COPY_MEMBERS),
            vec![vec![0x7000, 0, 0x4444]]
        );
    }
}
