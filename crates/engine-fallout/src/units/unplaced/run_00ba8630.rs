//! `(unplaced)/run_00ba8630` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// `00a5d470`: the jump target of `00ba87d0`.
const JUMP_TARGET_00BA87D0: u32 = 0x00a5_d470;

// Translated from 00ba87d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00a5d470` (engine map: `NiObject::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): the target `00a5d470` is `NiObject::IsEqual` (engine map), `bool` in the Xbox PDB.
pub fn fn_00ba87d0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00BA87D0, &args![this, other]).bool()
}

/// `BSShaderPPLightingProperty` code this class reuses (`bsshaderpplightingproperty.cpp`,
/// engine map): the base constructor `00b66f50` and destructor `00b67380` (`thiscall`, no
/// words), the slots +0x50 `00b66780` and +0x58 `00b68ed0` (`thiscall`, one word, `RET 4`),
/// `00b665c0` (`thiscall`, one word), the member copy `00b665d0` (`thiscall`, two words:
/// clone, cloning process) and `BSShaderPPLightingProperty::GetViewerStrings` `00b690d0`.
const BASE_CONSTRUCT_00B66F50: u32 = 0x00b6_6f50;
const BASE_DESTRUCT_00B67380: u32 = 0x00b6_7380;
const BASE_SLOT_50_00B66780: u32 = 0x00b6_6780;
const BASE_SLOT_58_00B68ED0: u32 = 0x00b6_8ed0;
const BASE_SLOT_5C_00B665C0: u32 = 0x00b6_65c0;
const BASE_COPY_MEMBERS_00B665D0: u32 = 0x00b6_65d0;
const BASE_GET_VIEWER_STRINGS: u32 = 0x00b6_90d0;
/// The Gamebryo allocator `00aa13e0` (`cdecl`, one word: size) and its sized free
/// `00aa1460` (`cdecl`, two words: block, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_FREE_SIZED: u32 = 0x00aa_1460;
/// `NiAVObject::UpdateProperties` (`00a5a040`, engine map; `thiscall`, no words).
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `NiGetViewerString` (`00a7d330`, engine map; `cdecl`, one word) and the `NiTArray`
/// growth `0096ad30` and element store `0096ae90` (`thiscall`, one and two words).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const ARRAY_SET_SIZE_0096AD30: u32 = 0x0096_ad30;
const ARRAY_SET_AT_0096AE90: u32 = 0x0096_ae90;
/// The class's virtual table and its `NiRTTI` record (built at start-up by `00fba150`
/// with the name `SpeedTreeBillboardShaderProperty`, `010b844c`).
const SPEEDTREE_BILLBOARD_VTABLE: u32 = 0x010b_8330;
const SPEEDTREE_BILLBOARD_RTTI: u32 = 0x0120_043c;
/// The object's size: what `CreateClone` allocates and the deleting destructor frees.
const OBJECT_SIZE: u32 = 0x104;

// Translated from 00ba8630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constructor of the class whose `NiRTTI` (`0120043c`)
/// is named `SpeedTreeBillboardShaderProperty` (`thiscall`, no words). Runs the base
/// constructor `00b66f50`, stores the virtual table `010b8330`, 1 at +0x58 and 0xb at
/// +0x1c (`iShader` and `iShaderPropertyType` at these offsets in the Xbox PDB's
/// `BSShaderPPLightingProperty`). Answers `this`.
pub fn fn_00ba8630(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT_00B66F50, &args![this]);
    e.mem.set_u32(this.addr(), SPEEDTREE_BILLBOARD_VTABLE);
    e.mem.set_u32(this.addr() + 0x58, 1);
    e.mem.set_u32(this.addr() + 0x1c, 0xb);
    this
}

// Translated from 00ba8650 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes): slot
/// +0x50 of `010b8330` (`thiscall`, one word), a jump to the base's `00b66780` with the
/// same words; answers its `EAX`.
pub fn fn_00ba8650(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    e.call(BASE_SLOT_50_00B66780, &args![this, stream]).u32()
}

// Translated from 00ba8660 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes): slot
/// +0x58 of `010b8330` (`thiscall`, one word), a jump to the base's `00b68ed0` with the
/// same words; answers its `EAX`.
pub fn fn_00ba8660(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    e.call(BASE_SLOT_58_00B68ED0, &args![this, stream]).u32()
}

// Translated from 00ba8670 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes): slot
/// +0x5c of `010b8330` (`IsEqual` in the Xbox PDB's slot order; `thiscall`, one word):
/// runs the base's `00b665c0` with the word, then clears `AL`: answers false whatever the
/// base answered.
pub fn fn_00ba8670(e: &mut Engine, this: Ptr, other: u32) -> bool {
    e.call(BASE_SLOT_5C_00B665C0, &args![this, other]);
    false
}

// Translated from 00ba8680 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes): slot
/// +8 of `010b8330` (`GetRTTI` in the Xbox PDB's slot order), no words: the address of
/// the `NiRTTI` record `0120043c` (`SpeedTreeBillboardShaderProperty`).
pub fn fn_00ba8680(_e: &mut Engine) -> u32 {
    SPEEDTREE_BILLBOARD_RTTI
}

// Translated from 00ba8690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot 0 of `010b8330`, the deleting destructor (`thiscall`,
/// one word: flags). Stores the virtual table, runs the base destructor `00b67380`, and
/// with bit 0 of the flags frees the 0x104-byte object (`00aa1460`). Answers `this`.
pub fn fn_00ba8690(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), SPEEDTREE_BILLBOARD_VTABLE);
    e.call(BASE_DESTRUCT_00B67380, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, OBJECT_SIZE]);
    }
    this
}

// Translated from 00ba86c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0xb0 of `010b8330` (`ClarifyShader` in the Xbox
/// PDB's slot order; `thiscall`, three words, the last two not read). When the first
/// word plus 0x9c is zero (the compiler's null test of a member address) runs
/// `NiAVObject::UpdateProperties` on the first word; then when the first word of the
/// object at +0xac (`spTextureSet` in the Xbox PDB) is zero, clears +0x58. Answers 0
/// (`EAX` cleared for the compare).
pub fn fn_00ba86c0(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    _unused_2: u32,
    _unused_3: u32,
) -> u32 {
    if geometry.wrapping_add(0x9c) == 0 {
        e.call(UPDATE_PROPERTIES, &args![geometry]);
    }
    let texture_set = e.mem.u32(this.addr() + 0xac);
    if e.mem.u32(texture_set) == 0 {
        e.mem.set_u32(this.addr() + 0x58, 0);
    }
    0
}

// Translated from 00ba86f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x48 of `010b8330` (`CreateClone` in the Xbox PDB's
/// slot order; `thiscall`, one word: the cloning process). Allocates 0x104 bytes
/// (`00aa13e0`); a block is constructed as `fn_00ba8630` does (inlined in the exe), none
/// leaves a null clone. Then the base member copy `00b665d0` runs on `this` with the
/// clone and the process, and the clone is answered.
// Translation incomplete: 00ba86f0 - the C++ exception frame (FS:[0] chain, unwind state 0 freeing the block if the constructor throws) is not translated.
pub fn fn_00ba86f0(e: &mut Engine, this: Ptr, process: u32) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![OBJECT_SIZE]).u32();
    let clone = if block != 0 {
        fn_00ba8630(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.call(BASE_COPY_MEMBERS_00B665D0, &args![this, clone, process]);
    clone
}

// Translated from 00ba8780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x60 of `010b8330` (`GetViewerStrings` in the Xbox
/// PDB's slot order; `thiscall`, one word: an `NiTArray` of strings). Runs the base's
/// `BSShaderPPLightingProperty::GetViewerStrings`, makes the string of the class's
/// `NiRTTI` name (`NiGetViewerString` on the word at `0120043c`) and appends it: with
/// `m_usSize` at least `m_usMaxSize` the array grows to `m_usSize + m_usGrowBy`
/// (`0096ad30`), then `0096ae90` stores it at the old size (the string passed by
/// address, in the exe's argument slot). Ends in that call, so its `EAX` is answered.
pub fn fn_00ba8780(e: &mut Engine, this: Ptr, strings: Ptr<NiTArray>) -> u32 {
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.global::<u32>(SPEEDTREE_BILLBOARD_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    let size = u32::from(e.get(strings, NiTArray::m_usSize));
    let max = u32::from(e.get(strings, NiTArray::m_usMaxSize));
    if size >= max {
        let grow = u32::from(e.get(strings, NiTArray::m_usGrowBy));
        e.call(ARRAY_SET_SIZE_0096AD30, &args![strings, grow + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(ARRAY_SET_AT_0096AE90, &args![strings, size, slot])
            .u32()
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_87d0, fn_00ba87d0(Ptr, Ptr) -> bool),
        entry!(0x00ba_8630, fn_00ba8630(Ptr) -> Ptr),
        entry!(0x00ba_8650, fn_00ba8650(Ptr, u32) -> u32),
        entry!(0x00ba_8660, fn_00ba8660(Ptr, u32) -> u32),
        entry!(0x00ba_8670, fn_00ba8670(Ptr, u32) -> bool),
        entry!(0x00ba_8680, fn_00ba8680() -> u32),
        entry!(0x00ba_8690, fn_00ba8690(Ptr, u32) -> Ptr),
        entry!(0x00ba_86c0, fn_00ba86c0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00ba_86f0, fn_00ba86f0(Ptr, u32) -> u32),
        entry!(0x00ba_8780, fn_00ba8780(Ptr, Ptr<NiTArray>) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00ba87d0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00ba_87d0;
        const TARGET: u32 = 0x00a5_d470;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    fn answer(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn constructor_runs_the_base_then_sets_the_table_and_kinds() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT_00B66F50, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            e.mem.set_u32(a[0] + 0x58, 7);
            Ret::default()
        });
        let this = e.mem.alloc(OBJECT_SIZE);
        assert_eq!(e.call(0x00ba_8630, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SPEEDTREE_BILLBOARD_VTABLE);
        assert_eq!(e.mem.u32(this + 0x58), 1);
        assert_eq!(e.mem.u32(this + 0x1c), 0xb);
    }

    #[test]
    fn forwarding_slots_pass_their_words_and_answers() {
        let mut e = Engine::new();
        e.register(BASE_SLOT_50_00B66780, |_, a| answer(a[1] + 1));
        e.register(BASE_SLOT_58_00B68ED0, |_, a| answer(a[1] + 2));
        e.register(BASE_SLOT_5C_00B665C0, |_, _| answer(0x0101));
        assert_eq!(e.call(0x00ba_8650, &args![0x10u32, 0x20u32]).u32(), 0x21);
        assert_eq!(e.call(0x00ba_8660, &args![0x10u32, 0x20u32]).u32(), 0x22);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00ba_8670, &args![0x10u32, 0x20u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_SLOT_5C_00B665C0, vec![0x10, 0x20])));
        assert_eq!(e.call(0x00ba_8680, &[]).u32(), SPEEDTREE_BILLBOARD_RTTI);
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(BASE_DESTRUCT_00B67380, |_, _| Ret::default());
        e.register(NI_FREE_SIZED, |_, _| Ret::default());
        let this = e.mem.alloc(OBJECT_SIZE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ba_8690, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), SPEEDTREE_BILLBOARD_VTABLE);
        e.call(0x00ba_8690, &args![this, 3u32]);
        let log = e.call_log.take().unwrap();
        let frees: Vec<_> = log.iter().filter(|c| c.0 == NI_FREE_SIZED).collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, vec![this, OBJECT_SIZE]);
        assert_eq!(
            log.iter().filter(|c| c.0 == BASE_DESTRUCT_00B67380).count(),
            2
        );
    }

    #[test]
    fn clarify_clears_the_shader_without_a_texture_set_word() {
        let mut e = Engine::new();
        e.register(UPDATE_PROPERTIES, |_, _| Ret::default());
        let this = e.mem.alloc(OBJECT_SIZE);
        let set = e.mem.alloc(8);
        e.mem.set_u32(this + 0xac, set);
        e.mem.set_u32(this + 0x58, 1);
        e.mem.set_u32(set, 0x55);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ba_86c0, &args![this, 0x4000u32, 0u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(this + 0x58), 1);
        e.mem.set_u32(set, 0);
        e.call(0x00ba_86c0, &args![this, 0xffff_ff64u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0x58), 0);
        let log = e.call_log.take().unwrap();
        let updates: Vec<_> = log.iter().filter(|c| c.0 == UPDATE_PROPERTIES).collect();
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].1, vec![0xffff_ff64]);
    }

    #[test]
    fn clone_is_constructed_then_copied_into() {
        let mut e = Engine::new();
        let block = e.mem.alloc(OBJECT_SIZE);
        e.register_double(NI_ALLOCATE, move |_, a| {
            assert_eq!(a, &[OBJECT_SIZE]);
            answer(block)
        });
        e.register(BASE_CONSTRUCT_00B66F50, |_, _| Ret::default());
        e.register(BASE_COPY_MEMBERS_00B665D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ba_86f0, &args![0x7000u32, 0x8000u32]).u32(),
            block
        );
        assert_eq!(e.mem.u32(block), SPEEDTREE_BILLBOARD_VTABLE);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_COPY_MEMBERS_00B665D0, vec![0x7000, block, 0x8000])));
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ba_86f0, &args![0x7000u32, 0x8000u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_COPY_MEMBERS_00B665D0, vec![0x7000, 0, 0x8000])));
        assert!(!log.iter().any(|c| c.0 == BASE_CONSTRUCT_00B66F50));
    }

    #[test]
    fn viewer_strings_append_the_class_name_growing_a_full_array() {
        let mut e = Engine::new();
        e.map(0x0120_0000, 0x1000);
        e.set_global(SPEEDTREE_BILLBOARD_RTTI, 0x010b_844cu32);
        e.register(BASE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, a| answer(a[0] + 1));
        e.register(ARRAY_SET_SIZE_0096AD30, |_, _| Ret::default());
        e.register(ARRAY_SET_AT_0096AE90, |e, a| answer(e.mem.u32(a[2])));
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0x8, 4);
        e.mem.set_u16(strings + 0xa, 4);
        e.mem.set_u16(strings + 0xe, 3);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ba_8780, &args![0x7000u32, strings]).u32(),
            0x010b_844d
        );
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_GET_VIEWER_STRINGS, vec![0x7000, strings])));
        assert!(log.contains(&(ARRAY_SET_SIZE_0096AD30, vec![strings, 7])));
        let stores: Vec<_> = log
            .iter()
            .filter(|c| c.0 == ARRAY_SET_AT_0096AE90)
            .collect();
        assert_eq!(stores[0].1[..2], [strings, 4]);
        e.mem.set_u16(strings + 0x8, 8);
        e.call_log = Some(vec![]);
        e.call(0x00ba_8780, &args![0x7000u32, strings]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == ARRAY_SET_SIZE_0096AD30));
    }
}
