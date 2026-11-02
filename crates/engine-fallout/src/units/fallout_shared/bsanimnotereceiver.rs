//! `fallout shared/bsanimnotereceiver.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSAnimNoteReceiver` keeps an array of 8-byte
//! `BSAnimNoteReceiverType` entries (Xbox PDB: `eType`, the
//! `BSAnimNote::AnimNoteType` at +0, and `pCallback` at +4) and calls the
//! callbacks of a note's type with its reference.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BSAnimNoteReceiver` (Xbox PDB), 0x18 bytes: the
    /// `IBSAnimNoteReceiver` vtable at +0 (slot 0 the scalar deleting
    /// destructor, slot 4 `Update`), the reference, and `CallTypeA`, an
    /// `NiTPrimitiveArray<BSAnimNoteReceiverType*>` at +0x08 (its count at
    /// +0x0A of the array).
    pub struct BSAnimNoteReceiver: 0x18 {
        /// `pObjRefr` (Xbox PDB): `TESObjectREFR*`.
        0x04 pObjRefr: Ptr,
    }
}

/// Offset of `CallTypeA` (Xbox PDB) in [`BSAnimNoteReceiver`].
const CALL_TYPES: u32 = 0x08;
/// `BSAnimNoteReceiver`'s vtable.
const RECEIVER_VTABLE: u32 = 0x0101_fc74;
/// `IBSAnimNoteReceiver`'s destructor (sets its vtable `0101fc80`).
const INTERFACE_DESTRUCT: u32 = 0x004b_0530;
/// The array's element count (the 16-bit word at +0x0A, zero-extended).
const ARRAY_COUNT: u32 = 0x0065_8930;
/// The address of element `index` of the array (`(array, index)`).
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// Removes and returns the array's last element (null when it is empty).
const ARRAY_REMOVE_LAST: u32 = 0x004b_07e0;
/// The array's destructor (`004b07b0`).
const ARRAY_DESTRUCT: u32 = 0x004b_07b0;
/// Appends the element `*value` to the array (`(array, value)`, through
/// `SetAtGrow` at the count).
const ARRAY_ADD: u32 = 0x0096_a610;
/// A note's type (the word at +8 of a `BSAnimNote`).
const NOTE_TYPE: u32 = 0x0044_ddc0;
/// `operator new` / `operator delete` (cdecl, one argument).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;

// Translated from 004b0480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNoteReceiver`'s destructor: its vtable, then each entry of
/// `CallTypeA` taken off the end and freed (as many as the count was at the
/// start), the array destroyed ([`fn_004b05b0`]) and the
/// `IBSAnimNoteReceiver` base (`004b0530`).
// Translation incomplete: 004b0480 - the C++ exception-unwinding frame (FS:[0]) that still destroys the array and the base when a free throws is not translated.
pub fn fn_004b0480(e: &mut Engine, this: Ptr<BSAnimNoteReceiver>) {
    let array = this.addr() + CALL_TYPES;
    e.mem.set_u32(this.addr(), RECEIVER_VTABLE);
    let count = e.call(ARRAY_COUNT, &args![array]).u32();
    for _ in 0..count {
        let entry = e.call(ARRAY_REMOVE_LAST, &args![array]).u32();
        e.call(FREE, &args![entry]);
    }
    fn_004b05b0(e, Ptr::new(array));
    e.call(INTERFACE_DESTRUCT, &args![this]);
}

// Translated from 004b0550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IBSAnimNoteReceiver::scalar deleting destructor` (Xbox PDB): the
/// destructor (`004b0530`), then `operator delete` when bit 0 of `flags`
/// is set. Returns `this`.
pub fn ibs_anim_note_receiver_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(INTERFACE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 004b0580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNoteReceiver::scalar deleting destructor` (Xbox PDB): the
/// destructor ([`fn_004b0480`]), then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn bs_anim_note_receiver_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSAnimNoteReceiver>,
    flags: u32,
) -> Ptr<BSAnimNoteReceiver> {
    fn_004b0480(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 004b05b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the `CallTypeA` array (`004b07b0` on it).
pub fn fn_004b05b0(e: &mut Engine, array: Ptr) {
    e.call(ARRAY_DESTRUCT, &args![array]);
}

// Translated from 004b05d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a receiver for note type `kind` with `callback` unless the same
/// pair is already there ([`fn_004b06b0`]): a new 8-byte entry (null if
/// the allocation fails, else built by [`fn_004b0680`]) appended to
/// `CallTypeA` through a stack word holding it (`0096a610`). Returns 0
/// (a `bool` the function never sets).
// Translation incomplete: 004b05d0 - the C++ exception-unwinding frame (FS:[0]) that frees the new entry when its constructor throws is not translated.
pub fn fn_004b05d0(
    e: &mut Engine,
    this: Ptr<BSAnimNoteReceiver>,
    kind: u32,
    callback: u32,
) -> bool {
    if fn_004b06b0(e, this, kind, callback) == u32::MAX {
        let block = e.call(ALLOCATE, &args![8u32]).u32();
        let entry = if block == 0 {
            0
        } else {
            fn_004b0680(e, Ptr::new(block), kind, callback).addr()
        };
        e.with_stack(4, |e, value| {
            e.mem.set_u32(value.addr(), entry);
            e.call(ARRAY_ADD, &args![this.addr() + CALL_TYPES, value]);
        });
    }
    false
}

// Translated from 004b0680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a pair of words: `first` at +0, `second` at +4. Returns
/// `this`. (One body shared by many 8-byte pairs; here the
/// `BSAnimNoteReceiverType`.)
pub fn fn_004b0680(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.mem.set_u32(this.addr(), first);
    e.mem.set_u32(this.addr() + 4, second);
    this
}

// Translated from 004b06b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Index of the last `CallTypeA` entry with this type and callback, or
/// 0xFFFFFFFF when there is none (every entry is checked).
pub fn fn_004b06b0(e: &mut Engine, this: Ptr<BSAnimNoteReceiver>, kind: u32, callback: u32) -> u32 {
    let array = this.addr() + CALL_TYPES;
    let mut found = u32::MAX;
    let mut index = 0u32;
    while index < e.call(ARRAY_COUNT, &args![array]).u32() {
        let slot = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
        let entry = e.mem.u32(slot);
        if e.mem.u32(entry) == kind {
            let slot = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
            let entry = e.mem.u32(slot);
            if e.mem.u32(entry + 4) == callback {
                found = index;
            }
        }
        index += 1;
    }
    found
}

// Translated from 004b0730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNoteReceiver::Update` (Xbox PDB): for every `CallTypeA` entry
/// whose type is `note`'s (`0044ddc0`), calls its callback (cdecl) with
/// `pObjRefr` and `note`.
pub fn bs_anim_note_receiver_update(e: &mut Engine, this: Ptr<BSAnimNoteReceiver>, note: Ptr) {
    let array = this.addr() + CALL_TYPES;
    let mut index = 0u32;
    while index < e.call(ARRAY_COUNT, &args![array]).u32() {
        let slot = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
        let entry = e.mem.u32(slot);
        let kind = e.call(NOTE_TYPE, &args![note]).u32();
        if e.mem.u32(entry) == kind {
            let reference = e.get(this, BSAnimNoteReceiver::pObjRefr);
            let slot = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
            let entry = e.mem.u32(slot);
            let callback = e.mem.u32(entry + 4);
            e.call(callback, &args![reference, note]);
        }
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004b0480, fn_004b0480(Ptr<BSAnimNoteReceiver>)),
        entry!(
            0x004b0550,
            ibs_anim_note_receiver_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x004b0580,
            bs_anim_note_receiver_scalar_deleting_destructor(
                Ptr<BSAnimNoteReceiver>,
                u32,
            )
                -> Ptr<BSAnimNoteReceiver>
        ),
        entry!(0x004b05b0, fn_004b05b0(Ptr)),
        entry!(
            0x004b05d0,
            fn_004b05d0(Ptr<BSAnimNoteReceiver>, u32, u32) -> bool
        ),
        entry!(0x004b0680, fn_004b0680(Ptr, u32, u32) -> Ptr),
        entry!(
            0x004b06b0,
            fn_004b06b0(Ptr<BSAnimNoteReceiver>, u32, u32) -> u32
        ),
        entry!(
            0x004b0730,
            bs_anim_note_receiver_update(Ptr<BSAnimNoteReceiver>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Doubles of the array helpers, doing what their bodies do on an
    /// array whose data is at +4 and count at +0x0A.
    fn double_array(e: &mut Engine) {
        e.register(ARRAY_COUNT, |e, a| {
            (e.mem.u16(a[0] + 0xa) as u32).into_ret()
        });
        e.register(ARRAY_ELEMENT, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        e.register(ARRAY_ADD, |e, a| {
            let count = e.mem.u16(a[0] + 0xa) as u32;
            let data = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(data + 4 * count, value);
            e.mem.set_u16(a[0] + 0xa, count as u16 + 1);
            Ret::default()
        });
        e.register(NOTE_TYPE, |e, a| e.mem.u32(a[0] + 8).into_ret());
    }

    /// A receiver with `entries` (type, callback) in its array.
    fn receiver(e: &mut Engine, entries: &[(u32, u32)]) -> Ptr<BSAnimNoteReceiver> {
        let this = e.new_object::<BSAnimNoteReceiver>();
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 0xc, data);
        e.mem.set_u16(this.addr() + 0x12, entries.len() as u16);
        for (i, &(kind, callback)) in entries.iter().enumerate() {
            let entry = e.mem.alloc(8);
            e.mem.set_u32(entry, kind);
            e.mem.set_u32(entry + 4, callback);
            e.mem.set_u32(data + 4 * i as u32, entry);
        }
        this
    }

    #[test]
    fn find_returns_the_last_matching_index() {
        let mut e = Engine::new();
        double_array(&mut e);
        let this = receiver(&mut e, &[(1, 0x100), (2, 0x200), (1, 0x100), (1, 0x300)]);
        let find = |e: &mut Engine, kind: u32, callback: u32| {
            e.call(0x004b_06b0, &args![this, kind, callback]).u32()
        };
        assert_eq!(find(&mut e, 1, 0x100), 2);
        assert_eq!(find(&mut e, 2, 0x200), 1);
        assert_eq!(find(&mut e, 2, 0x100), u32::MAX);
    }

    #[test]
    fn adding_appends_only_a_new_pair() {
        let mut e = Engine::new();
        double_array(&mut e);
        let block = e.mem.alloc(8);
        e.register_double(ALLOCATE, move |_, _| block.into_ret());
        let this = receiver(&mut e, &[(1, 0x100)]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004b_05d0, &args![this, 1u32, 0x100u32]).bool());
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert!(!e.call(0x004b_05d0, &args![this, 3u32, 0x400u32]).bool());
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![8]]);
        assert_eq!(e.mem.u16(this.addr() + 0x12), 2);
        let data = e.mem.u32(this.addr() + 0xc);
        assert_eq!(e.mem.u32(data + 4), block);
        assert_eq!((e.mem.u32(block), e.mem.u32(block + 4)), (3, 0x400));
    }

    #[test]
    fn adding_with_a_failed_allocation_appends_null() {
        let mut e = Engine::new();
        double_array(&mut e);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        let this = receiver(&mut e, &[]);
        e.call(0x004b_05d0, &args![this, 3u32, 0x400u32]);
        assert_eq!(e.mem.u16(this.addr() + 0x12), 1);
        assert_eq!(e.mem.u32(e.mem.u32(this.addr() + 0xc)), 0);
    }

    #[test]
    fn update_calls_the_callbacks_of_the_notes_type() {
        let mut e = Engine::new();
        double_array(&mut e);
        e.register(0x0150_0000, |_, _| Ret::default());
        e.register(0x0150_0100, |_, _| Ret::default());
        let this = receiver(
            &mut e,
            &[(4, 0x0150_0000), (5, 0x0150_0100), (4, 0x0150_0100)],
        );
        e.set(this, BSAnimNoteReceiver::pObjRefr, Ptr::new(0x0700_0000));
        let note = e.mem.alloc(0x10);
        e.mem.set_u32(note + 8, 4);
        e.call_log = Some(vec![]);
        e.call(0x004b_0730, &args![this, note]);
        assert_eq!(calls_to(&e, 0x0150_0000), vec![vec![0x0700_0000, note]]);
        assert_eq!(calls_to(&e, 0x0150_0100), vec![vec![0x0700_0000, note]]);
        assert_eq!(calls_to(&e, NOTE_TYPE).len(), 3);
    }

    #[test]
    fn destructor_frees_every_entry_then_the_array_and_base() {
        let mut e = Engine::new();
        double_array(&mut e);
        e.register(ARRAY_REMOVE_LAST, |e, a| {
            let count = e.mem.u16(a[0] + 0xa) as u32 - 1;
            e.mem.set_u16(a[0] + 0xa, count as u16);
            e.mem.u32(e.mem.u32(a[0] + 4) + 4 * count).into_ret()
        });
        for addr in [FREE, ARRAY_DESTRUCT, INTERFACE_DESTRUCT] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = receiver(&mut e, &[(1, 2), (3, 4)]);
        let data = e.mem.u32(this.addr() + 0xc);
        let (first, second) = (e.mem.u32(data), e.mem.u32(data + 4));
        e.call_log = Some(vec![]);
        let result = e.call(0x004b_0580, &args![this, 1u32]).u32();
        assert_eq!(result, this.addr());
        assert_eq!(e.mem.u32(this.addr()), RECEIVER_VTABLE);
        assert_eq!(
            calls_to(&e, FREE),
            vec![vec![second], vec![first], vec![this.addr()]]
        );
        assert_eq!(calls_to(&e, ARRAY_DESTRUCT), vec![vec![this.addr() + 8]]);
        assert_eq!(calls_to(&e, INTERFACE_DESTRUCT), vec![vec![this.addr()]]);
    }

    #[test]
    fn interface_deleting_destructor_and_pair() {
        let mut e = Engine::new();
        e.register(INTERFACE_DESTRUCT, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        let object = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_0550, &[object, 0]).u32(), object);
        assert!(calls_to(&e, FREE).is_empty());
        e.call(0x004b_0550, &[object, 1]);
        assert_eq!(calls_to(&e, FREE), vec![vec![object]]);
        assert_eq!(e.call(0x004b_0680, &[object, 7, 9]).u32(), object);
        assert_eq!((e.mem.u32(object), e.mem.u32(object + 4)), (7, 9));
        e.register(ARRAY_DESTRUCT, |_, _| Ret::default());
        e.call(0x004b_05b0, &[object]);
        assert_eq!(calls_to(&e, ARRAY_DESTRUCT), vec![vec![object]]);
    }
}
