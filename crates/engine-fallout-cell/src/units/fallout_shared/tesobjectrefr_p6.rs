//! `fallout shared/tesobjectrefr.cpp` (Xbox PDB source unit), part 6: its functions from `0057cac0` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tesobjectrefr`]; anything public there may be used here.
//!
//! # Where this file stops
//!
//! This range holds five functions in all (three container helpers the
//! linker placed in the unit at `0057cac0` to `0057cb70`, and two
//! `TESObjectREFR` methods far from them, `005c1c10` and `0086d490`); this
//! file translates all of them, so nothing is left for a later session.
//!
//! Callees outside this file are called by address. The compiler-generated
//! exception-unwinding frames of the container helpers are not translated.

#[allow(unused_imports)]
use super::tesobjectrefr::*;
#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// Destructor body of the `NiTMapBase<NiTPointerAllocator<unsigned int>,
/// unsigned int, bool>` map (`thiscall(this)`): sets the vtable and releases
/// the buckets.
const NI_T_MAP_BASE_DESTRUCT: u32 = 0x0057_ca30;
/// `operator delete` wrapper (`cdecl(block)`): frees the object.
const FREE_OBJECT: u32 = 0x0040_1030;
/// `thiscall(out byte)` with the object as its one stack argument; the word
/// it returns is what [`CONTAINER_RELEASE`] receives.
const CONTAINER_RELEASE_ARGUMENT: u32 = 0x004a_4840;
/// `thiscall(word)`: releases the storage of the object it is called on.
const CONTAINER_RELEASE: u32 = 0x0057_cb20;
/// `cdecl(count)`: allocates `count * 4` bytes.
const ALLOCATE_POINTER_ARRAY: u32 = 0x0096_afc0;
/// Vtable of the `NiTArray` instance (`NiTArray<BSAnimNoteReceiver::
/// BSAnimNoteReceiverType *, NiTMallocInterface<...>>`) that `0057cb70`
/// constructs.
const ANIM_NOTE_RECEIVER_ARRAY_VTABLE: u32 = 0x0101_fc8c;
/// `TESObjectREFR::SetScale`'s worker: `thiscall(this, float scale)` on the
/// reference.
const REFERENCE_STORE_SCALE: u32 = 0x0056_7490;
/// Vtable slot (byte offset) the scale change calls after storing the new
/// scale, with the argument 1.
const REFERENCE_SCALE_CHANGED_SLOT: u32 = 0xc8;
/// `thiscall(this, x, y, z)` on the reference: stores `data.Angle` and
/// refreshes the reference.
const REFERENCE_STORE_ANGLE: u32 = 0x0057_5700;

// Translated from 0057cac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned_int>,unsigned_int,bool>::
/// scalar deleting destructor` (Xbox PDB): runs the destructor body, frees
/// the object when bit 0 of `flags` is set, and returns `this`.
pub fn ni_t_map_base_u32_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(NI_T_MAP_BASE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 0057caf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the storage of the container `this`: asks `004a4840` (with a
/// zeroed byte local as its `this`) for the word the release call takes,
/// then calls `0057cb20` on `this` with it. The map has no name for it
/// (linker-placed helper of this unit).
pub fn fn_0057caf0(e: &mut Engine, this: Ptr) {
    e.with_stack(4, |e, local| {
        e.mem.set_u8(local.addr(), 0);
        let argument = e
            .call(CONTAINER_RELEASE_ARGUMENT, &args![local, this])
            .u32();
        e.call(CONTAINER_RELEASE, &args![this, argument]);
    });
}

// Translated from 0057cb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `NiTArray` of `BSAnimNoteReceiver::
/// BSAnimNoteReceiverType *` (the vtable's type name; the map has no name
/// for it): sets the vtable, `m_usMaxSize` and `m_usGrowBy`, zeroes
/// `m_usSize` and `m_usESize`, and allocates `max_size` pointers for
/// `m_pBase` (null when `max_size` is 0). Returns `this`.
pub fn fn_0057cb70(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), ANIM_NOTE_RECEIVER_ARRAY_VTABLE);
    e.set(this, NiTArray::m_usMaxSize, max_size);
    e.set(this, NiTArray::m_usGrowBy, grow_by);
    e.set(this, NiTArray::m_usSize, 0u16);
    e.set(this, NiTArray::m_usESize, 0u16);
    let storage = if max_size == 0 {
        0
    } else {
        e.call(ALLOCATE_POINTER_ARRAY, &args![max_size as u32])
            .u32()
    };
    e.set(this, NiTArray::m_pBase, storage);
    this
}

// Translated from 005c1c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetScale` (Xbox PDB), `cdecl(reference, scale)`: unless
/// `reference` is null, stores the scale through `00567490` and then calls
/// the reference's vtable slot `+0xc8` with 1.
pub fn tes_object_refr_set_scale(e: &mut Engine, reference: Ptr<TESObjectREFR>, scale: f32) {
    if reference.addr() == 0 {
        return;
    }
    e.call(REFERENCE_STORE_SCALE, &args![reference, scale]);
    e.vcall(reference.addr(), REFERENCE_SCALE_CHANGED_SLOT, &args![1u32]);
}

// Translated from 0086d490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetAngleOnReference` (Xbox PDB): passes the three
/// `float`s at `angle` (an `NiPoint3`, copied by value) to `00575700` on
/// the same reference.
pub fn tes_object_refr_set_angle_on_reference(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    angle: Ptr,
) {
    let base = angle.addr();
    let x = e.mem.u32(base);
    let y = e.mem.u32(base + 4);
    let z = e.mem.u32(base + 8);
    e.call(REFERENCE_STORE_ANGLE, &args![this, x, y, z]);
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0057cac0,
            ni_t_map_base_u32_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0057caf0, fn_0057caf0(Ptr)),
        entry!(
            0x0057cb70,
            fn_0057cb70(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(
            0x005c1c10,
            tes_object_refr_set_scale(Ptr<TESObjectREFR>, f32)
        ),
        entry!(
            0x0086d490,
            tes_object_refr_set_angle_on_reference(Ptr<TESObjectREFR>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addr: u32) {
        e.register(addr, |_, _| Ret::default());
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_when_bit_zero_is_set() {
        let mut e = Engine::new();
        quiet(&mut e, NI_T_MAP_BASE_DESTRUCT);
        quiet(&mut e, FREE_OBJECT);
        let this = e.mem.alloc(0x10);

        e.call_log = Some(vec![]);
        let r = e.call(0x0057_cac0, &args![this, 3u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(r, this);
        assert_eq!(calls_to(&log, NI_T_MAP_BASE_DESTRUCT), vec![vec![this]]);
        assert_eq!(calls_to(&log, FREE_OBJECT), vec![vec![this]]);

        e.call_log = Some(vec![]);
        let r = e.call(0x0057_cac0, &args![this, 2u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(r, this);
        assert_eq!(calls_to(&log, NI_T_MAP_BASE_DESTRUCT), vec![vec![this]]);
        assert!(calls_to(&log, FREE_OBJECT).is_empty());
    }

    #[test]
    fn release_passes_the_helper_result_to_the_release_call() {
        let mut e = Engine::new();
        // The helper receives a zeroed byte local as its `this` and the
        // container as its argument.
        e.register(CONTAINER_RELEASE_ARGUMENT, |e, a| {
            assert_eq!(e.mem.u8(a[0]), 0);
            Ret {
                eax: 0x1234 + a[1],
                ..Ret::default()
            }
        });
        quiet(&mut e, CONTAINER_RELEASE);
        let this = e.mem.alloc(0x10);

        e.call_log = Some(vec![]);
        e.call(0x0057_caf0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, CONTAINER_RELEASE),
            vec![vec![this, 0x1234 + this]]
        );
    }

    #[test]
    fn array_constructor_allocates_storage_for_a_nonzero_size() {
        let mut e = Engine::new();
        e.register(ALLOCATE_POINTER_ARRAY, |e, a| Ret {
            eax: e.mem.alloc(a[0] * 4),
            ..Ret::default()
        });
        let this = e.new_object::<NiTArray>();
        // Stale values the constructor must overwrite.
        e.set(this, NiTArray::m_usSize, 7u16);
        e.set(this, NiTArray::m_usESize, 9u16);

        e.call_log = Some(vec![]);
        let r = e
            .call(0x0057_cb70, &args![this, 0x10u32, 4u32])
            .ptr::<NiTArray>();
        let log = e.call_log.take().unwrap();
        assert_eq!(r, this);
        assert_eq!(e.mem.u32(this.addr()), ANIM_NOTE_RECEIVER_ARRAY_VTABLE);
        assert_eq!(e.get(this, NiTArray::m_usMaxSize), 0x10);
        assert_eq!(e.get(this, NiTArray::m_usGrowBy), 4);
        assert_eq!(e.get(this, NiTArray::m_usSize), 0);
        assert_eq!(e.get(this, NiTArray::m_usESize), 0);
        assert_ne!(e.get(this, NiTArray::m_pBase), 0);
        assert_eq!(calls_to(&log, ALLOCATE_POINTER_ARRAY), vec![vec![0x10]]);
    }

    #[test]
    fn array_constructor_leaves_storage_null_for_size_zero() {
        let mut e = Engine::new();
        let this = e.new_object::<NiTArray>();
        e.set(this, NiTArray::m_pBase, 0xdead_beefu32);

        e.call_log = Some(vec![]);
        e.call(0x0057_cb70, &args![this, 0u32, 8u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(this, NiTArray::m_pBase), 0);
        assert_eq!(e.get(this, NiTArray::m_usGrowBy), 8);
        assert!(calls_to(&log, ALLOCATE_POINTER_ARRAY).is_empty());
    }

    #[test]
    fn set_scale_stores_then_notifies_unless_reference_is_null() {
        let mut e = Engine::new();
        quiet(&mut e, REFERENCE_STORE_SCALE);
        quiet(&mut e, 0x00ff_0001);
        e.put_vtable(0x00fe_0000, &[0; 0x33]);
        e.mem
            .set_u32(0x00fe_0000 + REFERENCE_SCALE_CHANGED_SLOT, 0x00ff_0001);
        let reference = e.mem.alloc(0x68);
        e.mem.set_u32(reference, 0x00fe_0000);

        e.call_log = Some(vec![]);
        e.call(0x005c_1c10, &args![reference, 2.5f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, REFERENCE_STORE_SCALE),
            vec![vec![reference, 2.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, 0x00ff_0001), vec![vec![reference, 1]]);

        e.call_log = Some(vec![]);
        e.call(0x005c_1c10, &args![0u32, 2.5f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1, "only the top-level call is logged");
    }

    #[test]
    fn set_angle_on_reference_forwards_the_three_floats() {
        let mut e = Engine::new();
        quiet(&mut e, REFERENCE_STORE_ANGLE);
        let reference = e.mem.alloc(0x68);
        let angle = e.mem.alloc(12);
        e.mem.set_f32(angle, 0.5);
        e.mem.set_f32(angle + 4, -1.25);
        e.mem.set_f32(angle + 8, 3.0);

        e.call_log = Some(vec![]);
        e.call(0x0086_d490, &args![reference, angle]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, REFERENCE_STORE_ANGLE),
            vec![vec![
                reference,
                0.5f32.to_bits(),
                (-1.25f32).to_bits(),
                3.0f32.to_bits()
            ]]
        );
    }
}
