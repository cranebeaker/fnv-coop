//! `fallout shared/havok/bgsacousticspacelistener.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSAcousticSpaceListener` (Xbox PDB), 0x28 bytes; the PC code uses
    /// the same offsets (`0061f768`, `0061f87d`).
    pub struct BGSAcousticSpaceListener: 0x28 {
        /// `bBufferRemovalOperations` (Xbox PDB).
        0x14 bBufferRemovalOperations: bool,
    }
}

/// Offset of `RemovalQueueMap` (`NiTMap<TESObjectREFR *,int>`, Xbox PDB).
const REMOVAL_QUEUE_MAP: u32 = 0x18;
/// `BGSAcousticSpaceListener` vtable (RTTI `.?AVBGSAcousticSpaceListener@@`).
const LISTENER_VTABLE: u32 = 0x0104_d8e8;
/// `NiTMap<TESObjectREFR *,int>` vtable (RTTI).
const REF_MAP_VTABLE: u32 = 0x0104_d948;
/// `NiTMapBase<DFALL<NiTMapItem<TESObjectREFR *,int>>,TESObjectREFR *,int>`
/// vtable (RTTI).
const REF_MAP_BASE_VTABLE: u32 = 0x0104_d968;
/// The global reference these callbacks compare against (other units read
/// it as the player character; not confirmed here).
const REFERENCE_GLOBAL_011DEA3C: u32 = 0x011d_ea3c;

/// `TESHavokUtilities::FindCollidableRef` (Xbox PDB), another unit.
const FIND_COLLIDABLE_REF: u32 = 0x0062_b4e0;
/// `FalloutAudio::ClearAcousticSpace` (engine map name), called with one word.
const CLEAR_ACOUSTIC_SPACE: u32 = 0x0082_eb70;
/// Unnamed one-word callee taken on the add path (`0082eae0`).
const ADD_PATH_CALLEE: u32 = 0x0082_eae0;
/// Thiscall getter on a reference whose result the audio calls take
/// (`007af430`; the engine map's `BGSSaveFormBuffer::GetForm` is a folded name).
const REFERENCE_AUDIO_KEY: u32 = 0x007a_f430;
/// Map `SetAt(key, value)` (`00844700`, a folded `NiTMapBase<..>::SetAt` body).
const MAP_SET_AT: u32 = 0x0084_4700;
/// Map `RemoveAt(key)` (`00405430`, a folded `NiTMapBase<..>::RemoveAt` body).
const MAP_REMOVE_AT: u32 = 0x0040_5430;
/// Map first-position getter (`004b9ba0`).
const MAP_GET_FIRST_POS: u32 = 0x004b_9ba0;
/// Map `GetNext(&pos, &key, &value)` (`006b7f20`).
const MAP_GET_NEXT: u32 = 0x006b_7f20;
/// Map `RemoveAll` (`00438af0`).
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;

// Translated from 0061f5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAcousticSpaceListener::_scalar_deleting_destructor_` (Xbox PDB).
pub fn bgs_acoustic_space_listener_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BGSAcousticSpaceListener>,
    flags: u32,
) -> Ptr<BGSAcousticSpaceListener> {
    fn_0061f5d0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translation incomplete: 0061f5d0 - FS:[0] exception frame (unwind of the removal map) not translated.
// Translated from 0061f5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAcousticSpaceListener` destructor body: sets its vtable, destroys the
/// removal queue map and runs the base destructor (`005545f0`).
/// open: the exception frame's unwind state is not modelled.
pub fn fn_0061f5d0(e: &mut Engine, this: Ptr<BGSAcousticSpaceListener>) {
    e.mem.set_u32(this.addr(), LISTENER_VTABLE);
    fn_0061fae0(e, Ptr::new(this.addr() + REMOVAL_QUEUE_MAP));
    e.call(0x0055_45f0, &args![this]);
}

// Translated from 0061f630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAcousticSpaceListener::entityRemovedCallback` (Xbox PDB): clears the
/// acoustic space when the removed entity belongs to the reference in the
/// global at `011dea3c`.
pub fn bgs_acoustic_space_listener_entity_removed_callback(
    e: &mut Engine,
    _this: Ptr<BGSAcousticSpaceListener>,
    entity: Ptr,
) {
    let reference: u32 = e.global(REFERENCE_GLOBAL_011DEA3C);
    if reference == 0 {
        return;
    }
    if e.call(0x004e_af60, &args![reference]).bool() {
        let removed = e.call(0x0046_0140, &args![entity]).u32();
        let reference: u32 = e.global(REFERENCE_GLOBAL_011DEA3C);
        let holder = fn_0061f6c0(e, Ptr::new(reference));
        let own = e.call(0x0043_b560, &args![holder]).u32();
        if removed == own {
            e.call(CLEAR_ACOUSTIC_SPACE, &args![0u32]);
        }
    } else {
        let collidable = e.call(0x0046_0140, &args![entity]).u32();
        let found = e.call(FIND_COLLIDABLE_REF, &args![collidable]).u32();
        if found == e.global::<u32>(REFERENCE_GLOBAL_011DEA3C) {
            e.call(CLEAR_ACOUSTIC_SPACE, &args![0u32]);
        }
    }
}

// Translated from 0061f6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the pointer held at `this + 0xdec` through `00559450`.
pub fn fn_0061f6c0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0055_9450, &args![this.addr() + 0xdec]).u32()
}

/// The reference on the other side of a contact with the global reference,
/// as both contact callbacks compute it (0 when the contact is not with it).
fn contact_partner(e: &mut Engine, event: Ptr) -> u32 {
    let reference: u32 = e.global(REFERENCE_GLOBAL_011DEA3C);
    let mut partner = 0;
    if e.call(0x004e_af60, &args![reference]).bool() {
        let reference: u32 = e.global(REFERENCE_GLOBAL_011DEA3C);
        let holder = fn_0061f6c0(e, Ptr::new(reference));
        let own = e.call(0x0043_b560, &args![holder]).u32();
        if e.mem.u32(event.addr() + 0x48) == own {
            let other = e.mem.u32(event.addr() + 0x20);
            partner = e.call(FIND_COLLIDABLE_REF, &args![other]).u32();
        }
    } else {
        let first = e.mem.u32(event.addr() + 0x48);
        let found = e.call(FIND_COLLIDABLE_REF, &args![first]).u32();
        if found == e.global::<u32>(REFERENCE_GLOBAL_011DEA3C) {
            let other = e.mem.u32(event.addr() + 0x20);
            partner = e.call(FIND_COLLIDABLE_REF, &args![other]).u32();
        }
    }
    partner
}

// Translated from 0061f6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAcousticSpaceListener::contactPointAddedCallback` (Xbox PDB): finds
/// the reference touching the global reference (event words +0x48 and
/// +0x20); unless a buffered removal of it was cancelled, passes its key to
/// `0082eae0`. True when such a reference was found.
pub fn bgs_acoustic_space_listener_contact_point_added_callback(
    e: &mut Engine,
    this: Ptr<BGSAcousticSpaceListener>,
    event: Ptr,
) -> bool {
    let partner = contact_partner(e, event);
    if partner == 0 {
        return false;
    }
    let cancelled = e.get(this, BGSAcousticSpaceListener::bBufferRemovalOperations)
        && fn_0061f890(e, this, Ptr::new(partner), false);
    if !cancelled {
        let key = e.call(REFERENCE_AUDIO_KEY, &args![partner]).u32();
        e.call(ADD_PATH_CALLEE, &args![key]);
    }
    true
}

// Translated from 0061f7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAcousticSpaceListener::contactPointRemovedCallback` (Xbox PDB): for
/// the reference touching the global reference, queues its removal when
/// removals are buffered, else clears its acoustic space at once. Returns
/// false (`AL` = 0; the upper bytes of `EAX` are left from the last call).
pub fn bgs_acoustic_space_listener_contact_point_removed_callback(
    e: &mut Engine,
    this: Ptr<BGSAcousticSpaceListener>,
    event: Ptr,
) -> bool {
    let partner = contact_partner(e, event);
    if partner != 0 {
        if e.get(this, BGSAcousticSpaceListener::bBufferRemovalOperations) {
            fn_0061f870(e, this, Ptr::new(partner));
        } else {
            let key = e.call(REFERENCE_AUDIO_KEY, &args![partner]).u32();
            e.call(CLEAR_ACOUSTIC_SPACE, &args![key]);
        }
    }
    false
}

// Translated from 0061f870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues a reference in the removal map with a count of 5.
pub fn fn_0061f870(e: &mut Engine, this: Ptr<BGSAcousticSpaceListener>, reference: Ptr) {
    e.call(
        MAP_SET_AT,
        &args![this.addr() + REMOVAL_QUEUE_MAP, reference, 5u32],
    );
}

// Translated from 0061f890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes a reference from the removal map; when it was there and `clear`
/// is set, clears its acoustic space. Returns whether it was removed.
pub fn fn_0061f890(
    e: &mut Engine,
    this: Ptr<BGSAcousticSpaceListener>,
    reference: Ptr,
    clear: bool,
) -> bool {
    let removed = e
        .call(
            MAP_REMOVE_AT,
            &args![this.addr() + REMOVAL_QUEUE_MAP, reference],
        )
        .bool();
    if removed && clear {
        let key = e.call(REFERENCE_AUDIO_KEY, &args![reference]).u32();
        e.call(CLEAR_ACOUSTIC_SPACE, &args![key]);
    }
    removed
}

// Translated from 0061f8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the acoustic space of every queued reference, then empties the
/// removal map. The stack word it takes (`RET 4`) is never read.
pub fn fn_0061f8e0(e: &mut Engine, this: Ptr<BGSAcousticSpaceListener>, _unused_1: u32) {
    let map = this.addr() + REMOVAL_QUEUE_MAP;
    e.with_stack(12, |e, locals| {
        let position = locals.addr();
        let key = position + 4;
        let value = position + 8;
        let first = e.call(MAP_GET_FIRST_POS, &args![map]).u32();
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.call(MAP_GET_NEXT, &args![map, position, key, value]);
            let audio_key = e.call(REFERENCE_AUDIO_KEY, &args![e.mem.u32(key)]).u32();
            e.call(CLEAR_ACOUSTIC_SPACE, &args![audio_key]);
        }
    });
    e.call(MAP_REMOVE_ALL, &args![map]);
}

// Translated from 0061f940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAcousticSpaceListener::Update` (Xbox PDB): counts every queued
/// removal down by one, then repeatedly removes the first entry whose count
/// is 0 and clears its acoustic space, restarting the scan after each. The
/// stack word it takes (`RET 4`) is never read.
pub fn bgs_acoustic_space_listener_update(
    e: &mut Engine,
    this: Ptr<BGSAcousticSpaceListener>,
    _unused_1: u32,
) {
    let map = this.addr() + REMOVAL_QUEUE_MAP;
    e.with_stack(24, |e, locals| {
        let position = locals.addr();
        let key = position + 4;
        let value = position + 8;
        let first = e.call(MAP_GET_FIRST_POS, &args![map]).u32();
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.call(MAP_GET_NEXT, &args![map, position, key, value]);
            let count = e.mem.u32(value).wrapping_sub(1);
            e.mem.set_u32(value, count);
            e.call(MAP_SET_AT, &args![map, e.mem.u32(key), count]);
        }
        let scan = position + 12;
        let scan_key = position + 16;
        let scan_value = position + 20;
        let mut again = true;
        while again {
            again = false;
            let first = e.call(MAP_GET_FIRST_POS, &args![map]).u32();
            e.mem.set_u32(scan, first);
            while e.mem.u32(scan) != 0 {
                e.call(MAP_GET_NEXT, &args![map, scan, scan_key, scan_value]);
                if e.mem.u32(scan_value) == 0 {
                    let reference = e.mem.u32(scan_key);
                    e.call(MAP_REMOVE_AT, &args![map, reference]);
                    let reference = e.mem.u32(scan_key);
                    let audio_key = e.call(REFERENCE_AUDIO_KEY, &args![reference]).u32();
                    e.call(CLEAR_ACOUSTIC_SPACE, &args![audio_key]);
                    again = true;
                    break;
                }
            }
        }
    });
}

// Translated from 0061fa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESObjectREFR *,int>` constructor: base map with `hash_size`
/// buckets, then the `NiTMap` vtable.
pub fn fn_0061fa10(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_0061fa70(e, this, hash_size);
    e.mem.set_u32(this.addr(), REF_MAP_VTABLE);
    this
}

// Translated from 0061fa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESObjectREFR *,int>::_scalar_deleting_destructor_` (Xbox PDB).
pub fn ni_t_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0061fae0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0061fa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..TESObjectREFR *,int>` constructor: stores the hash size,
/// zero count, allocates the bucket table and clears it.
pub fn fn_0061fa70(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    e.mem.set_u32(this.addr(), REF_MAP_BASE_VTABLE);
    e.mem.set_u32(this.addr() + 4, hash_size);
    e.mem.set_u32(this.addr() + 0xc, 0);
    let bytes = e.mem.u32(this.addr() + 4) << 2;
    let table = e.call(0x00aa_1070, &args![bytes]).u32();
    e.mem.set_u32(this.addr() + 8, table);
    // The size and the table are both reloaded after the allocation.
    let clear_bytes = e.mem.u32(this.addr() + 4) << 2;
    let table = e.mem.u32(this.addr() + 8);
    e.call(0x0040_3d30, &args![table, 0u32, clear_bytes]);
    this
}

// Translation incomplete: 0061fae0 - FS:[0] exception frame (unwind into the base destructor) not translated.
// Translated from 0061fae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESObjectREFR *,int>` destructor body: `RemoveAll`, then the base
/// destructor. open: the exception frame's unwind state is not modelled.
pub fn fn_0061fae0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), REF_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_0061fb40(e, this);
}

// Translated from 0061fb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..TESObjectREFR *,int>` destructor body: `RemoveAll`, then
/// frees the bucket table.
pub fn fn_0061fb40(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), REF_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(0x00aa_10f0, &args![e.mem.u32(this.addr() + 8)]);
}

// Translated from 0061fb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<TESObjectREFR *,int>>,TESObjectREFR *,int>::_scalar_deleting_destructor_`
/// (Xbox PDB).
pub fn ni_t_map_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0061fb40(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0061f5a0,
            bgs_acoustic_space_listener_scalar_deleting_destructor(
                Ptr<BGSAcousticSpaceListener>,
                u32,
            )
                -> Ptr<BGSAcousticSpaceListener>
        ),
        entry!(0x0061f5d0, fn_0061f5d0(Ptr<BGSAcousticSpaceListener>)),
        entry!(
            0x0061f630,
            bgs_acoustic_space_listener_entity_removed_callback(Ptr<BGSAcousticSpaceListener>, Ptr)
        ),
        entry!(0x0061f6c0, fn_0061f6c0(Ptr) -> u32),
        entry!(
            0x0061f6e0,
            bgs_acoustic_space_listener_contact_point_added_callback(
                Ptr<BGSAcousticSpaceListener>,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x0061f7b0,
            bgs_acoustic_space_listener_contact_point_removed_callback(
                Ptr<BGSAcousticSpaceListener>,
                Ptr,
            ) -> bool
        ),
        entry!(0x0061f870, fn_0061f870(Ptr<BGSAcousticSpaceListener>, Ptr)),
        entry!(
            0x0061f890,
            fn_0061f890(Ptr<BGSAcousticSpaceListener>, Ptr, bool) -> bool
        ),
        entry!(0x0061f8e0, fn_0061f8e0(Ptr<BGSAcousticSpaceListener>, u32)),
        entry!(
            0x0061f940,
            bgs_acoustic_space_listener_update(Ptr<BGSAcousticSpaceListener>, u32)
        ),
        entry!(0x0061fa10, fn_0061fa10(Ptr, u32) -> Ptr),
        entry!(0x0061fa40, ni_t_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0061fa70, fn_0061fa70(Ptr, u32) -> Ptr),
        entry!(0x0061fae0, fn_0061fae0(Ptr)),
        entry!(0x0061fb40, fn_0061fb40(Ptr)),
        entry!(0x0061fb70, ni_t_map_base_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn noop(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn set_reference(e: &mut Engine, value: u32) {
        e.map(REFERENCE_GLOBAL_011DEA3C, 4);
        e.set_global(REFERENCE_GLOBAL_011DEA3C, value);
    }

    /// The map callees: first position and next entries come from `entries`
    /// (key, value); next writes position 0 after the last.
    fn map_doubles(e: &mut Engine, entries: Vec<(u32, u32)>) {
        let first = if entries.is_empty() { 0 } else { 1 };
        e.register_double(MAP_GET_FIRST_POS, move |_, _| ret(first));
        e.register_double(MAP_GET_NEXT, move |e, a| {
            let index = e.mem.u32(a[1]) as usize - 1;
            let (key, value) = entries[index];
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            let next = if index + 1 < entries.len() {
                index as u32 + 2
            } else {
                0
            };
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_flag() {
        let mut e = Engine::new();
        let listener: Ptr<BGSAcousticSpaceListener> = e.new_object();
        noop(
            &mut e,
            &[0x0055_45f0, MAP_REMOVE_ALL, 0x00aa_10f0, 0x0040_1030],
        );
        e.call_log = Some(vec![]);
        e.call(0x0061_f5a0, &args![listener, 0u32]);
        assert!(calls(&e, 0x0040_1030).is_empty());
        let result = e.call(0x0061_f5a0, &args![listener, 1u32]).u32();
        assert_eq!(result, listener.addr());
        assert_eq!(calls(&e, 0x0040_1030), vec![vec![listener.addr()]]);
    }

    #[test]
    fn destructor_sets_vtables_and_destroys_map_then_base() {
        let mut e = Engine::new();
        let listener: Ptr<BGSAcousticSpaceListener> = e.new_object();
        e.mem.set_u32(listener.addr() + 0x20, 0x4444);
        noop(&mut e, &[0x0055_45f0, MAP_REMOVE_ALL, 0x00aa_10f0]);
        e.call_log = Some(vec![]);
        e.call(0x0061_f5d0, &args![listener]);
        assert_eq!(e.mem.u32(listener.addr()), LISTENER_VTABLE);
        assert_eq!(
            e.mem.u32(listener.addr() + REMOVAL_QUEUE_MAP),
            REF_MAP_BASE_VTABLE
        );
        assert_eq!(calls(&e, 0x00aa_10f0), vec![vec![0x4444]]);
        assert_eq!(calls(&e, 0x0055_45f0), vec![vec![listener.addr()]]);
    }

    #[test]
    fn entity_removed_compares_collidable_reference_or_own_entity() {
        let mut e = Engine::new();
        let listener: Ptr<BGSAcousticSpaceListener> = e.new_object();
        set_reference(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x0061_f630, &args![listener, 0x10u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);

        set_reference(&mut e, 0x5000);
        e.register(0x004e_af60, |_, _| ret(0x100));
        e.register(0x0046_0140, |_, a| ret(a[0] + 1));
        e.register(FIND_COLLIDABLE_REF, |_, a| {
            ret(if a[0] == 0x11 { 0x5000 } else { 0 })
        });
        noop(&mut e, &[CLEAR_ACOUSTIC_SPACE]);
        e.call(0x0061_f630, &args![listener, 0x10u32]);
        assert_eq!(calls(&e, CLEAR_ACOUSTIC_SPACE), vec![vec![0]]);

        e.register(0x004e_af60, |_, _| ret(1));
        e.register(0x0055_9450, |_, a| ret(a[0]));
        e.register(0x0043_b560, |_, a| ret(a[0] - 0x5dec + 0x21));
        e.call_log = Some(vec![]);
        e.call(0x0061_f630, &args![listener, 0x20u32]);
        assert_eq!(calls(&e, CLEAR_ACOUSTIC_SPACE), vec![vec![0]]);
        assert_eq!(calls(&e, 0x0055_9450), vec![vec![0x5dec]]);
        e.call(0x0061_f630, &args![listener, 0x30u32]);
        assert_eq!(calls(&e, CLEAR_ACOUSTIC_SPACE).len(), 1);
    }

    #[test]
    fn holder_getter_reads_offset_dec() {
        let mut e = Engine::new();
        e.register(0x0055_9450, |_, a| ret(a[0] + 1));
        assert_eq!(e.call(0x0061_f6c0, &args![0x1000u32]).u32(), 0x1dec + 1);
    }

    fn contact_doubles(e: &mut Engine, own_entity_path: bool) {
        set_reference(e, 0x5000);
        e.register_double(0x004e_af60, move |_, _| ret(own_entity_path as u32));
        e.register(0x0055_9450, |_, _| ret(0x7000));
        e.register(0x0043_b560, |_, _| ret(0x48));
        e.register(FIND_COLLIDABLE_REF, |_, a| {
            ret(match a[0] {
                0x48 => 0x5000,
                0x20 => 0x6000,
                _ => 0,
            })
        });
        e.register(REFERENCE_AUDIO_KEY, |_, a| ret(a[0] + 1));
        noop(e, &[ADD_PATH_CALLEE, CLEAR_ACOUSTIC_SPACE, MAP_SET_AT]);
    }

    fn event(e: &mut Engine, first: u32, second: u32) -> Ptr {
        let event = e.mem.alloc(0x4c);
        e.mem.set_u32(event + 0x48, first);
        e.mem.set_u32(event + 0x20, second);
        Ptr::new(event)
    }

    #[test]
    fn contact_added_reports_partner_unless_buffered_removal_was_cancelled() {
        let mut e = Engine::new();
        let listener: Ptr<BGSAcousticSpaceListener> = e.new_object();
        contact_doubles(&mut e, false);
        let touching = event(&mut e, 0x48, 0x20);
        let other = event(&mut e, 0x99, 0x20);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0061_f6e0, &args![listener, other]).bool());
        assert!(e.call(0x0061_f6e0, &args![listener, touching]).bool());
        assert_eq!(calls(&e, ADD_PATH_CALLEE), vec![vec![0x6001]]);

        e.set(
            listener,
            BGSAcousticSpaceListener::bBufferRemovalOperations,
            true,
        );
        e.register(MAP_REMOVE_AT, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0061_f6e0, &args![listener, touching]).bool());
        assert!(calls(&e, ADD_PATH_CALLEE).is_empty());
        assert!(calls(&e, CLEAR_ACOUSTIC_SPACE).is_empty());
        assert_eq!(
            calls(&e, MAP_REMOVE_AT),
            vec![vec![listener.addr() + 0x18, 0x6000]]
        );
    }

    #[test]
    fn contact_added_own_entity_path_compares_event_word() {
        let mut e = Engine::new();
        let listener: Ptr<BGSAcousticSpaceListener> = e.new_object();
        contact_doubles(&mut e, true);
        let touching = event(&mut e, 0x48, 0x20);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0061_f6e0, &args![listener, touching]).bool());
        assert_eq!(calls(&e, 0x0055_9450), vec![vec![0x5dec]]);
        assert_eq!(calls(&e, FIND_COLLIDABLE_REF), vec![vec![0x20]]);
    }

    #[test]
    fn contact_removed_queues_or_clears() {
        let mut e = Engine::new();
        let listener: Ptr<BGSAcousticSpaceListener> = e.new_object();
        contact_doubles(&mut e, false);
        let touching = event(&mut e, 0x48, 0x20);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0061_f7b0, &args![listener, touching]).bool());
        assert_eq!(calls(&e, CLEAR_ACOUSTIC_SPACE), vec![vec![0x6001]]);
        e.set(
            listener,
            BGSAcousticSpaceListener::bBufferRemovalOperations,
            true,
        );
        e.call(0x0061_f7b0, &args![listener, touching]);
        assert_eq!(
            calls(&e, MAP_SET_AT),
            vec![vec![listener.addr() + 0x18, 0x6000, 5]]
        );
    }

    #[test]
    fn queue_removal_sets_count_five() {
        let mut e = Engine::new();
        let listener: Ptr<BGSAcousticSpaceListener> = e.new_object();
        noop(&mut e, &[MAP_SET_AT]);
        e.call_log = Some(vec![]);
        e.call(0x0061_f870, &args![listener, 0x77u32]);
        assert_eq!(
            calls(&e, MAP_SET_AT),
            vec![vec![listener.addr() + 0x18, 0x77, 5]]
        );
    }

    #[test]
    fn cancel_removal_clears_only_when_removed_and_asked() {
        let mut e = Engine::new();
        let listener: Ptr<BGSAcousticSpaceListener> = e.new_object();
        e.register(MAP_REMOVE_AT, |_, a| ret((a[1] == 0x77) as u32));
        e.register(REFERENCE_AUDIO_KEY, |_, a| ret(a[0] + 1));
        noop(&mut e, &[CLEAR_ACOUSTIC_SPACE]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0061_f890, &args![listener, 0x55u32, true]).bool());
        assert!(e.call(0x0061_f890, &args![listener, 0x77u32, false]).bool());
        assert!(calls(&e, CLEAR_ACOUSTIC_SPACE).is_empty());
        assert!(e.call(0x0061_f890, &args![listener, 0x77u32, true]).bool());
        assert_eq!(calls(&e, CLEAR_ACOUSTIC_SPACE), vec![vec![0x78]]);
    }

    #[test]
    fn flush_clears_every_queued_reference_then_empties_map() {
        let mut e = Engine::new();
        let listener: Ptr<BGSAcousticSpaceListener> = e.new_object();
        map_doubles(&mut e, vec![(0x10, 3), (0x20, 1)]);
        e.register(REFERENCE_AUDIO_KEY, |_, a| ret(a[0] + 1));
        noop(&mut e, &[CLEAR_ACOUSTIC_SPACE, MAP_REMOVE_ALL]);
        e.call_log = Some(vec![]);
        e.call(0x0061_f8e0, &args![listener, 0u32]);
        assert_eq!(
            calls(&e, CLEAR_ACOUSTIC_SPACE),
            vec![vec![0x11], vec![0x21]]
        );
        assert_eq!(
            calls(&e, MAP_REMOVE_ALL),
            vec![vec![listener.addr() + 0x18]]
        );
    }

    #[test]
    fn update_counts_down_and_removes_expired_entries() {
        let mut e = Engine::new();
        let listener: Ptr<BGSAcousticSpaceListener> = e.new_object();
        // A small real map: entries (key, count); RemoveAt deletes the key.
        let entries = std::rc::Rc::new(std::cell::RefCell::new(vec![(0x10u32, 1u32), (0x20, 3)]));
        let list = entries.clone();
        e.register_double(MAP_GET_FIRST_POS, move |_, _| {
            ret(!list.borrow().is_empty() as u32)
        });
        let list = entries.clone();
        e.register_double(MAP_GET_NEXT, move |e, a| {
            let list = list.borrow();
            let index = e.mem.u32(a[1]) as usize - 1;
            e.mem.set_u32(a[2], list[index].0);
            e.mem.set_u32(a[3], list[index].1);
            let next = if index + 1 < list.len() {
                index as u32 + 2
            } else {
                0
            };
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        let list = entries.clone();
        e.register_double(MAP_SET_AT, move |_, a| {
            for entry in list.borrow_mut().iter_mut() {
                if entry.0 == a[1] {
                    entry.1 = a[2];
                }
            }
            Ret::default()
        });
        let list = entries.clone();
        e.register_double(MAP_REMOVE_AT, move |_, a| {
            list.borrow_mut().retain(|entry| entry.0 != a[1]);
            ret(1)
        });
        e.register(REFERENCE_AUDIO_KEY, |_, a| ret(a[0] + 1));
        noop(&mut e, &[CLEAR_ACOUSTIC_SPACE]);
        e.call_log = Some(vec![]);
        e.call(0x0061_f940, &args![listener, 0u32]);
        assert_eq!(*entries.borrow(), vec![(0x20, 2)]);
        assert_eq!(calls(&e, CLEAR_ACOUSTIC_SPACE), vec![vec![0x11]]);
        // Two full scans: one that removes, one that finds nothing.
        assert_eq!(calls(&e, MAP_GET_FIRST_POS).len(), 3);
    }

    #[test]
    fn map_constructor_sets_map_vtable_last() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x00aa_1070, |e, a| ret(e.mem.alloc(a[0])));
        noop(&mut e, &[0x0040_3d30]);
        assert_eq!(e.call(0x0061_fa10, &args![map, 37u32]).u32(), map.addr());
        assert_eq!(e.mem.u32(map.addr()), REF_MAP_VTABLE);
        assert_eq!(e.mem.u32(map.addr() + 4), 37);
    }

    #[test]
    fn map_scalar_deleting_destructor_frees_only_with_flag() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        noop(&mut e, &[MAP_REMOVE_ALL, 0x00aa_10f0, 0x0040_1030]);
        e.call_log = Some(vec![]);
        e.call(0x0061_fa40, &args![map, 0u32]);
        assert!(calls(&e, 0x0040_1030).is_empty());
        assert_eq!(e.mem.u32(map.addr()), REF_MAP_BASE_VTABLE);
        e.call(0x0061_fa40, &args![map, 1u32]);
        assert_eq!(calls(&e, 0x0040_1030), vec![vec![map.addr()]]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL).len(), 4);
    }

    #[test]
    fn base_constructor_reloads_size_and_table_after_allocation() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        e.register_double(0x00aa_1070, move |e, a| {
            assert_eq!(a, &[8]);
            e.mem.set_u32(map.addr() + 4, 3);
            ret(0x1234)
        });
        noop(&mut e, &[0x0040_3d30]);
        e.call_log = Some(vec![]);
        e.call(0x0061_fa70, &args![map, 2u32]);
        assert_eq!(e.mem.u32(map.addr()), REF_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(map.addr() + 8), 0x1234);
        assert_eq!(e.mem.u32(map.addr() + 0xc), 0);
        assert_eq!(calls(&e, 0x0040_3d30), vec![vec![0x1234, 0, 12]]);
    }

    #[test]
    fn map_destructor_removes_all_then_runs_base() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        noop(&mut e, &[MAP_REMOVE_ALL, 0x00aa_10f0]);
        e.call_log = Some(vec![]);
        e.call(0x0061_fae0, &args![map]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL).len(), 2);
        assert_eq!(e.mem.u32(map.addr()), REF_MAP_BASE_VTABLE);
    }

    #[test]
    fn base_destructor_frees_bucket_table() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        e.mem.set_u32(map.addr() + 8, 0x9999);
        noop(&mut e, &[MAP_REMOVE_ALL, 0x00aa_10f0]);
        e.call_log = Some(vec![]);
        e.call(0x0061_fb40, &args![map]);
        assert_eq!(e.mem.u32(map.addr()), REF_MAP_BASE_VTABLE);
        assert_eq!(calls(&e, MAP_REMOVE_ALL), vec![vec![map.addr()]]);
        assert_eq!(calls(&e, 0x00aa_10f0), vec![vec![0x9999]]);
    }

    #[test]
    fn base_scalar_deleting_destructor_frees_only_with_flag() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        noop(&mut e, &[MAP_REMOVE_ALL, 0x00aa_10f0, 0x0040_1030]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_fb70, &args![map, 2u32]).u32(), map.addr());
        assert!(calls(&e, 0x0040_1030).is_empty());
        e.call(0x0061_fb70, &args![map, 3u32]);
        assert_eq!(calls(&e, 0x0040_1030), vec![vec![map.addr()]]);
    }
}
