//! `fallout shared/tesquesttarget.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESQuestTarget` (0x48 bytes, the same on PC): a flags byte, the
//! target's conditions (+4), the target reference (+0xc, a form ID until
//! `InitItem` resolves it) and its path (+0x10). The constructor is in
//! `run_0060ff70.rs`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::run_0060bef0::TESQuest;

/// The file reader (`TESFile`): chunk type (`004726b0`), chunk data
/// (`00472890`: buffer, size; 0 reads the whole chunk), byte-swap flag
/// (`00401680`).
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_GET_CHUNK_DATA: u32 = 0x0047_2890;
const FILE_SWAP_FLAG: u32 = 0x0040_1680;
/// The `memset` wrapper (`00403d30`, cdecl: destination, value, count).
const MEMSET: u32 = 0x0040_3d30;
/// `00503210` (this = the chunk buffer): swaps its first word in place
/// (through `00401080`).
const SWAP_TARGET_DATA: u32 = 0x0050_3210;
/// `00717e50`: `this + 4` (the target's `TESCondition`);
/// `TESCondition::InitItem` (`00680a20`, one stack word: the form).
const TARGET_CONDITIONS: u32 = 0x0071_7e50;
const CONDITION_INIT_ITEM: u32 = 0x0068_0a20;
/// `TESForm::GetFile` (`00484e60`, one stack word: the index, -1 the
/// last), `TESForm::AddCompileIndex` (`00485d50`, cdecl: address of the
/// form ID, file) and the form lookup by ID (`004839c0`, cdecl).
const FORM_GET_FILE: u32 = 0x0048_4e60;
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, 0, source type, target
/// type, 0) and the RTTI type descriptors of `TESForm` and
/// `TESObjectREFR`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// `0084e3a0`: the word at `this + 0xc` (the form ID); `TESQuest` vtable
/// slot `0x130`: its editor ID string.
const FORM_ID: u32 = 0x0084_e3a0;
const SLOT_EDITOR_ID: u32 = 0x130;
/// The warning printer (`005b5e40`, cdecl, a format and its arguments) and
/// its formats (`.rdata`).
const WARNING: u32 = 0x005b_5e40;
const MSG_REFERENCE_NOT_FOUND: u32 = 0x0104_aff0;
const MSG_NO_REFERENCE: u32 = 0x0104_afb4;
/// `00440d80`: bit 0x20 of a form's flags word (+8); `005d43c0`:
/// `this + 0x44` (a reference's extra data list);
/// `ExtraDataList::GetReferencePointer` (`0041c8d0`).
const FORM_FLAG_0X20: u32 = 0x0044_0d80;
const REFERENCE_EXTRA_DATA: u32 = 0x005d_43c0;
const EXTRA_REFERENCE_POINTER: u32 = 0x0041_c8d0;

/// The `QSTA` chunk tag, as the little-endian word the file reader returns.
const CHUNK_QSTA: u32 = 0x4154_5351;

layout! {
    /// `TESQuestTarget` (Xbox PDB), 0x48 bytes; the PC constructor and
    /// loader use the same offsets.
    pub struct TESQuestTarget: 0x48 {
        /// `cFlags` (Xbox PDB).
        0x00 cFlags: u8,
        /// `m_pTargetRef` (Xbox PDB): `TESObjectREFR*` (a form ID while
        /// loading).
        0x0C m_pTargetRef: Ptr,
    }
}

// Translated from 00610040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESQuestTarget::InitItem` (Xbox PDB): resolves the target reference's
/// form ID (adjusted to the quest's file with `TESForm::AddCompileIndex`,
/// looked up and cast to `TESObjectREFR`), with a warning naming the form
/// ID and the quest (form ID, editor ID from vtable slot `0x130`) when it is
/// not found, or a warning naming the quest when there is no reference.
/// Then initializes the target's conditions with the quest when it has
/// them.
pub fn tes_quest_target_init_item(e: &mut Engine, this: Ptr<TESQuestTarget>, quest: Ptr<TESQuest>) {
    let reference = e.get(this, TESQuestTarget::m_pTargetRef).addr();
    if reference == 0 {
        let editor_id = e.vcall(quest.addr(), SLOT_EDITOR_ID, &args![]).u32();
        let form_id = e.call(FORM_ID, &args![quest]).u32();
        e.call(WARNING, &args![MSG_NO_REFERENCE, form_id, editor_id]);
    } else {
        e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), reference);
            let file = e.call(FORM_GET_FILE, &args![quest, 0xffff_ffffu32]).u32();
            e.call(FORM_ADD_COMPILE_INDEX, &args![cell, file]);
            let form_id = e.mem.u32(cell.addr());
            let form = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
            let found = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0u32],
                )
                .u32();
            e.set(this, TESQuestTarget::m_pTargetRef, Ptr::new(found));
            if e.get(this, TESQuestTarget::m_pTargetRef) == Ptr::NULL {
                let editor_id = e.vcall(quest.addr(), SLOT_EDITOR_ID, &args![]).u32();
                let quest_id = e.call(FORM_ID, &args![quest]).u32();
                let form_id = e.mem.u32(cell.addr());
                e.call(
                    WARNING,
                    &args![MSG_REFERENCE_NOT_FOUND, form_id, quest_id, editor_id],
                );
            }
        });
    }
    if e.call(TARGET_CONDITIONS, &args![this]).u32() != 0 {
        let conditions = e.call(TARGET_CONDITIONS, &args![this]).u32();
        e.call(CONDITION_INIT_ITEM, &args![conditions, quest]);
    }
}

// Translated from 00610120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The target's chunk loader (`TESQuest`'s loader calls it for `QSTA`):
/// with a file whose current chunk is `QSTA`, reads it into a zeroed
/// 8-byte buffer (swapped by `00503210` when the reader swaps) and takes
/// the target reference's form ID from its first word and the flags from
/// its fifth byte. Anything else is ignored.
pub fn fn_00610120(e: &mut Engine, this: Ptr<TESQuestTarget>, file: Ptr) {
    if file == Ptr::NULL || e.call(FILE_CHUNK_TYPE, &args![file]).u32() != CHUNK_QSTA {
        return;
    }
    e.with_stack(8, |e, buffer| {
        e.call(MEMSET, &args![buffer, 0u32, 8u32]);
        e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0u32]);
        if e.call(FILE_SWAP_FLAG, &args![file]).bool() {
            e.call(SWAP_TARGET_DATA, &args![buffer]);
        }
        let reference = e.mem.u32(buffer.addr());
        e.set(this, TESQuestTarget::m_pTargetRef, Ptr::new(reference));
        let flags = e.mem.u8(buffer.addr() + 4);
        e.set(this, TESQuestTarget::cFlags, flags);
    });
}

// Translated from 00610190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0 of the target's flags byte (`cFlags`).
pub fn fn_00610190(e: &mut Engine, this: Ptr<TESQuestTarget>) -> bool {
    e.get(this, TESQuestTarget::cFlags) & 1 != 0
}

// Translated from 006101b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESQuestTarget::GetReference` (Xbox PDB): the target reference; with
/// `follow` set and the reference's flag 0x20 (`00440d80`) set, the
/// reference its extra data points to (`ExtraDataList::GetReferencePointer`)
/// when there is one.
pub fn tes_quest_target_get_reference(
    e: &mut Engine,
    this: Ptr<TESQuestTarget>,
    follow: u8,
) -> Ptr {
    let mut reference = e.get(this, TESQuestTarget::m_pTargetRef).addr();
    if reference != 0 && e.call(FORM_FLAG_0X20, &args![reference]).bool() && follow != 0 {
        let extra = e.call(REFERENCE_EXTRA_DATA, &args![reference]).u32();
        let pointed = e.call(EXTRA_REFERENCE_POINTER, &args![extra]).u32();
        if pointed != 0 {
            reference = pointed;
        }
    }
    Ptr::new(reference)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00610040,
            tes_quest_target_init_item(Ptr<TESQuestTarget>, Ptr<TESQuest>)
        ),
        entry!(0x00610120, fn_00610120(Ptr<TESQuestTarget>, Ptr)),
        entry!(0x00610190, fn_00610190(Ptr<TESQuestTarget>) -> bool),
        entry!(
            0x006101b0,
            tes_quest_target_get_reference(Ptr<TESQuestTarget>, u8) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| eax(value));
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const QUEST_EDITOR_ID: u32 = 0x00ab_0000;

    fn init_world(
        reference: u32,
        found: u32,
        conditions: bool,
    ) -> (Engine, Ptr<TESQuestTarget>, Ptr<TESQuest>) {
        let mut e = Engine::new();
        e.register_double(TARGET_CONDITIONS, move |_, a| {
            eax(if conditions { a[0] + 4 } else { 0 })
        });
        stub(&mut e, FORM_GET_FILE, 0x4400);
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            eax(0)
        });
        e.register(LOOKUP_FORM_BY_ID, |_, _| eax(0x9100));
        stub(&mut e, DYNAMIC_CAST, found);
        e.register(FORM_ID, |e, a| eax(e.mem.u32(a[0] + 0xc)));
        stub(&mut e, QUEST_EDITOR_ID, 0x6000);
        stub(&mut e, WARNING, 0);
        stub(&mut e, CONDITION_INIT_ITEM, 0);
        let target: Ptr<TESQuestTarget> = e.new_object();
        e.set(target, TESQuestTarget::m_pTargetRef, Ptr::new(reference));
        let quest: Ptr<TESQuest> = e.new_object();
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + SLOT_EDITOR_ID, QUEST_EDITOR_ID);
        e.mem.set_u32(quest.addr(), vtable);
        e.mem.set_u32(quest.addr() + 0xc, 0x0012_3456);
        e.call_log = Some(vec![]);
        (e, target, quest)
    }

    #[test]
    fn init_item_resolves_the_reference_and_its_conditions() {
        let (mut e, target, quest) = init_world(0x42, 0x9200, true);
        tes_quest_target_init_item(&mut e, target, quest);
        assert_eq!(
            calls_to(&e, FORM_GET_FILE),
            vec![vec![quest.addr(), 0xffff_ffff]]
        );
        assert_eq!(calls_to(&e, LOOKUP_FORM_BY_ID), vec![vec![0x0100_0042]]);
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST),
            vec![vec![0x9100, 0, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0]]
        );
        assert_eq!(e.get(target, TESQuestTarget::m_pTargetRef).addr(), 0x9200);
        assert!(calls_to(&e, WARNING).is_empty());
        assert_eq!(
            calls_to(&e, CONDITION_INIT_ITEM),
            vec![vec![target.addr() + 4, quest.addr()]]
        );
    }

    #[test]
    fn init_item_warns_for_a_missing_or_unknown_reference() {
        let (mut e, target, quest) = init_world(0x42, 0, false);
        tes_quest_target_init_item(&mut e, target, quest);
        assert_eq!(
            calls_to(&e, WARNING),
            vec![vec![
                MSG_REFERENCE_NOT_FOUND,
                0x0100_0042,
                0x0012_3456,
                0x6000
            ]]
        );
        assert!(calls_to(&e, CONDITION_INIT_ITEM).is_empty());

        let (mut e, target, quest) = init_world(0, 0, false);
        tes_quest_target_init_item(&mut e, target, quest);
        assert_eq!(
            calls_to(&e, WARNING),
            vec![vec![MSG_NO_REFERENCE, 0x0012_3456, 0x6000]]
        );
        assert!(calls_to(&e, LOOKUP_FORM_BY_ID).is_empty());
    }

    fn load_world(tag: u32, swap: bool) -> (Engine, Ptr<TESQuestTarget>) {
        let mut e = Engine::new();
        stub(&mut e, FILE_CHUNK_TYPE, tag);
        e.register(FILE_GET_CHUNK_DATA, |e, a| {
            e.mem.write(a[1], &[0x11, 0x22, 0x33, 0x44, 0x05, 0, 0, 0]);
            eax(1)
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            eax(a[0])
        });
        stub(&mut e, FILE_SWAP_FLAG, swap as u32);
        e.register(SWAP_TARGET_DATA, |e, a| {
            let word = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], word.swap_bytes());
            eax(0)
        });
        let target: Ptr<TESQuestTarget> = e.new_object();
        e.call_log = Some(vec![]);
        (e, target)
    }

    #[test]
    fn loader_reads_reference_and_flags_from_qsta() {
        let (mut e, target) = load_world(CHUNK_QSTA, false);
        fn_00610120(&mut e, target, Ptr::new(0x5000));
        assert_eq!(
            e.get(target, TESQuestTarget::m_pTargetRef).addr(),
            0x4433_2211
        );
        assert_eq!(e.get(target, TESQuestTarget::cFlags), 5);
        assert_eq!(calls_to(&e, FILE_GET_CHUNK_DATA)[0][2], 0);
        assert_eq!(calls_to(&e, MEMSET)[0][1..], [0, 8]);
        assert!(calls_to(&e, SWAP_TARGET_DATA).is_empty());

        let (mut e, target) = load_world(CHUNK_QSTA, true);
        fn_00610120(&mut e, target, Ptr::new(0x5000));
        assert_eq!(
            e.get(target, TESQuestTarget::m_pTargetRef).addr(),
            0x1122_3344
        );
    }

    #[test]
    fn loader_ignores_other_chunks_and_a_null_file() {
        let (mut e, target) = load_world(0x1111, false);
        fn_00610120(&mut e, target, Ptr::new(0x5000));
        assert!(calls_to(&e, FILE_GET_CHUNK_DATA).is_empty());
        let (mut e, target) = load_world(CHUNK_QSTA, false);
        fn_00610120(&mut e, target, Ptr::NULL);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        assert_eq!(e.get(target, TESQuestTarget::m_pTargetRef), Ptr::NULL);
    }

    #[test]
    fn flag_bit_0() {
        let mut e = Engine::new();
        let target: Ptr<TESQuestTarget> = e.new_object();
        for (flags, set) in [(0, false), (1, true), (0xfe, false), (0xff, true)] {
            e.set(target, TESQuestTarget::cFlags, flags);
            assert_eq!(fn_00610190(&mut e, target), set);
        }
    }

    #[test]
    fn get_reference_follows_the_extra_data_only_when_asked_and_flagged() {
        let world = |reference: u32, flagged: bool, pointed: u32| {
            let mut e = Engine::new();
            e.register_double(FORM_FLAG_0X20, move |_, _| eax(flagged as u32));
            e.register(REFERENCE_EXTRA_DATA, |_, a| eax(a[0] + 0x44));
            e.register_double(EXTRA_REFERENCE_POINTER, move |_, _| eax(pointed));
            let target: Ptr<TESQuestTarget> = e.new_object();
            e.set(target, TESQuestTarget::m_pTargetRef, Ptr::new(reference));
            (e, target)
        };
        let (mut e, target) = world(0x700, true, 0x800);
        assert_eq!(
            tes_quest_target_get_reference(&mut e, target, 1).addr(),
            0x800
        );
        assert_eq!(
            tes_quest_target_get_reference(&mut e, target, 0).addr(),
            0x700
        );
        let (mut e, target) = world(0x700, false, 0x800);
        assert_eq!(
            tes_quest_target_get_reference(&mut e, target, 1).addr(),
            0x700
        );
        let (mut e, target) = world(0x700, true, 0);
        assert_eq!(
            tes_quest_target_get_reference(&mut e, target, 1).addr(),
            0x700
        );
        let (mut e, target) = world(0, true, 0x800);
        assert_eq!(tes_quest_target_get_reference(&mut e, target, 1), Ptr::NULL);
    }
}
