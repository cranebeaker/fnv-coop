//! `fallout shared/run_0047f940` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two classes of the actor base form (`TESActorBase`) are spread over this
//! range, next to the functions of their own `.cpp` units:
//!
//! - `TESAIForm` (the component at `TESActorBase+0x90`, confirmed by
//!   `TESActorBase`'s constructor calling `TESAIForm::TESAIForm` `0047ec40`
//!   on it): `0047f940` and `0047f980` write and read its `AIData` block
//!   when the save buffer's flags say so.
//! - `TESAnimation` (the component at `TESActorBase+0xc4`; vtable `0101bf50`
//!   has the RTTI name `.?AVTESAnimation@@`): constructor, destructor body,
//!   the three vtable functions (`ClearDataComponent`, `CopyComponent`,
//!   `CompareComponent`), the loader of the `KFFZ` chunk and the two
//!   helpers that search and add to the list of KF file names
//!   (`kfFilenames`, a `BSSimpleList<char const *>` at `+4`).
//!
//! The list head is a node inside the object (item at `+0`, next at `+4`);
//! it is empty when both are zero. The exception-unwinding frame of the
//! destructor is not translated.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleList;

/// `operator new` (cdecl, size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (cdecl, block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `__RTDynamicCast(object, 0, source type, target type, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `BaseFormComponent` constructor (stores the base vtable).
const BASE_FORM_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// The list constructor: zeroes the head node (`ECX` = list).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// The list destructor (`ECX` = list).
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// Whether the list is empty (`ECX` = list): no item and no next node.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// Returns `ECX` itself: the address of the node's item slot.
const LIST_ITEM_SLOT: u32 = 0x0068_15c0;
/// Removes the first item of the list (`ECX` = list).
const LIST_POP_FRONT: u32 = 0x0063_f7b0;
/// Returns the next node of `ECX`.
const LIST_NEXT: u32 = 0x0072_6070;
/// Appends the item whose address is passed to the list (`ECX` = list).
const LIST_PUSH_BACK: u32 = 0x0090_5820;
/// Case-insensitive string comparison (cdecl): zero when equal.
const STRING_COMPARE_NO_CASE: u32 = 0x0040_4dc0;
/// `strcpy_s` wrapper (cdecl): destination, size, source.
const STRING_COPY: u32 = 0x0040_6d30;
/// `strlen` wrapper (cdecl).
const STRING_LENGTH: u32 = 0x0044_a670;
/// Tests a flag mask against the dword at `ECX` (one stack argument).
const FLAGS_TEST: u32 = 0x0042_80f0;
/// Writes the save buffer's flags to the address passed and returns it
/// (`ECX` = buffer).
const BUFFER_FLAGS: u32 = 0x0042_8110;
/// `BGSSaveGameBuffer` raw write: `this`, data, size, 0.
const SAVE_RAW: u32 = 0x0086_5e50;
/// `BGSLoadGameBuffer` raw read: `this`, destination, size.
const LOAD_RAW: u32 = 0x0086_4980;
/// `TESFile::GetTESChunk` (Xbox PDB): the type of the current chunk.
const TES_FILE_GET_TES_CHUNK: u32 = 0x0047_26b0;
/// Reads `TESFile::m_actualChunkSize` (+0x25c) of the current chunk.
const TES_FILE_CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB): `this`, buffer, maximum size
/// (0 = no maximum).
const TES_FILE_GET_CHUNK_DATA: u32 = 0x0047_2890;

/// The flag of the save buffer that enables the AI block.
const SAVE_FLAG_AI_DATA: u32 = 8;
/// Size of the `AIDATA` block at `TESAIForm+4`.
const AI_DATA_SIZE: u32 = 0x14;
/// The vtable of `TESAnimation`.
const ANIMATION_VTABLE: u32 = 0x0101_bf50;
/// The chunk type "KFFZ": the list of KF file names.
const CHUNK_KF_FILE_NAMES: u32 = 0x5a46_464b;
/// RTTI type descriptors for the `dynamic_cast` of a component to
/// `TESAnimation` (source `BaseFormComponent`).
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_ANIMATION: u32 = 0x0118_584c;
/// `ClearDataComponent`'s slot (byte offset 4) in the component vtable.
const SLOT_CLEAR_DATA_COMPONENT: u32 = 4;

layout! {
    /// `TESAIForm` (Xbox PDB), 0x20 bytes: only the start of the `AIData`
    /// block is used here.
    pub struct TESAIForm: 0x20 {
        /// `AIData` (Xbox PDB): 0x14 bytes of `AIDATA`.
        0x04 AIData: u32,
    }

    /// `TESAnimation` (Xbox PDB), 0xC bytes.
    pub struct TESAnimation: 0x0c {
        /// `kfFilenames` (Xbox PDB): `BSSimpleList<char const *>`, the list
        /// head is inside the object.
        0x04 kfFilenames: crate::ptr::Inline<BSSimpleList>,
    }
}

/// Whether the save buffer's flags contain `mask` (`00428110` then
/// `004280f0`, the second with the mask left on the stack).
fn buffer_has_flag(e: &mut Engine, buffer: Ptr, mask: u32) -> bool {
    e.with_stack(4, |e, slot| {
        let flags: Ptr = e.call(BUFFER_FLAGS, &args![buffer, slot]).ptr();
        e.call(FLAGS_TEST, &args![flags, mask]).bool()
    })
}

// Translated from 0047f940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAIForm`'s save game step (the call from `TESActorBase`'s save at
/// `005f1f82`, with `this` = the component at `+0x90`): when the save
/// buffer's flags contain 8, writes the 0x14-byte `AIData` block.
pub fn fn_0047f940(e: &mut Engine, this: Ptr<TESAIForm>, buffer: Ptr) {
    if buffer_has_flag(e, buffer, SAVE_FLAG_AI_DATA) {
        let ai_data = this.addr() + 4;
        e.call(SAVE_RAW, &args![buffer, ai_data, AI_DATA_SIZE, 0]);
    }
}

// Translated from 0047f980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAIForm`'s load game step (the call from `TESActorBase`'s load at
/// `005f2041`): when the load buffer's flags contain 8, reads the 0x14-byte
/// `AIData` block.
pub fn fn_0047f980(e: &mut Engine, this: Ptr<TESAIForm>, buffer: Ptr) {
    if buffer_has_flag(e, buffer, SAVE_FLAG_AI_DATA) {
        let ai_data = this.addr() + 4;
        e.call(LOAD_RAW, &args![buffer, ai_data, AI_DATA_SIZE]);
    }
}

// Translated from 0047f9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESAnimation` constructor (called on `TESActorBase+0xc4`): the base
/// component constructor, the vtable `0101bf50`, and an empty file name
/// list.
pub fn fn_0047f9c0(e: &mut Engine, this: Ptr<TESAnimation>) -> Ptr<TESAnimation> {
    e.call(BASE_FORM_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), ANIMATION_VTABLE);
    let list = this.at(TESAnimation::kfFilenames);
    e.call(LIST_CONSTRUCT, &args![list]);
    this
}

// Translated from 0047f9f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0047f9f0 - C++ exception-unwinding frame (FS:[0] handler chain) not translated
/// The `TESAnimation` destructor body: stores the class's vtable, clears
/// the file name list (freeing the names) and destroys the list. The base
/// component is not destroyed here.
pub fn fn_0047f9f0(e: &mut Engine, this: Ptr<TESAnimation>) {
    e.mem.set_u32(this.addr(), ANIMATION_VTABLE);
    fn_0047fa50(e, this);
    let list = this.at(TESAnimation::kfFilenames);
    e.call(LIST_DESTRUCT, &args![list]);
}

// Translated from 0047fa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAnimation::ClearDataComponent` (slot 1 of the vtable; Xbox PDB):
/// removes every file name from the list and frees it.
pub fn fn_0047fa50(e: &mut Engine, this: Ptr<TESAnimation>) {
    let list = this.at(TESAnimation::kfFilenames);
    while !e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let slot = e.call(LIST_ITEM_SLOT, &args![list]).u32();
        let name = e.mem.u32(slot);
        e.call(LIST_POP_FRONT, &args![list]);
        e.call(OPERATOR_DELETE, &args![name]);
    }
}

// Translated from 0047faa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the "KFFZ" chunk of `file` into the file name list: nothing
/// happens unless both `component` (the caller passes `this` again, or null)
/// and `file` are set and the current chunk is "KFFZ". The chunk (sized by
/// `m_actualChunkSize`) holds NUL-terminated names ended by an empty one;
/// each is added with `fn_0047fd20`. The temporary buffer is freed.
pub fn fn_0047faa0(e: &mut Engine, this: Ptr<TESAnimation>, component: Ptr, file: Ptr) {
    if file.is_null() || component.is_null() {
        return;
    }
    if e.call(TES_FILE_GET_TES_CHUNK, &args![file]).u32() != CHUNK_KF_FILE_NAMES {
        return;
    }
    let size = e.call(TES_FILE_CHUNK_SIZE, &args![file]).u32();
    let buffer = e.call(OPERATOR_NEW, &args![size]).u32();
    e.call(TES_FILE_GET_CHUNK_DATA, &args![file, buffer, 0]);
    let mut name = buffer;
    while e.mem.i8(name) != 0 {
        fn_0047fd20(e, this, Ptr::new(name));
        let length = e.call(STRING_LENGTH, &args![name]).u32();
        name = name.wrapping_add(length).wrapping_add(1);
    }
    e.call(OPERATOR_DELETE, &args![buffer]);
}

// Translated from 0047fb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAnimation::CopyComponent` (slot 2 of the vtable; Xbox PDB): when the
/// source component is a `TESAnimation`, clears this one (virtual slot 4)
/// and adds each non-null file name of the source's list.
pub fn fn_0047fb50(e: &mut Engine, this: Ptr<TESAnimation>, source: Ptr) {
    let other: Ptr<TESAnimation> = e
        .call(
            RT_DYNAMIC_CAST,
            &args![source, 0, TYPE_BASE_FORM_COMPONENT, TYPE_ANIMATION, 0],
        )
        .ptr();
    if other.is_null() {
        return;
    }
    e.vcall(this.addr(), SLOT_CLEAR_DATA_COMPONENT, &args![]);
    let mut node = other.at(TESAnimation::kfFilenames).addr();
    while node != 0 {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let name = e.mem.u32(slot);
        if name != 0 {
            fn_0047fd20(e, this, Ptr::new(name));
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
}

// Translated from 0047fbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAnimation::CompareComponent` (slot 3 of the vtable; Xbox PDB): true
/// when the other component differs (or is not a `TESAnimation`). The two
/// lists are walked together: a null name against a set one differs, two
/// set names must be equal ignoring case, and both lists must end together.
pub fn fn_0047fbd0(e: &mut Engine, this: Ptr<TESAnimation>, other: Ptr) -> bool {
    let other: Ptr<TESAnimation> = e
        .call(
            RT_DYNAMIC_CAST,
            &args![other, 0, TYPE_BASE_FORM_COMPONENT, TYPE_ANIMATION, 0],
        )
        .ptr();
    if other.is_null() {
        return true;
    }
    let mut mine = this.at(TESAnimation::kfFilenames).addr();
    let mut theirs = other.at(TESAnimation::kfFilenames).addr();
    while mine != 0 && theirs != 0 {
        let slot = e.call(LIST_ITEM_SLOT, &args![mine]).u32();
        let name = e.mem.u32(slot);
        let slot = e.call(LIST_ITEM_SLOT, &args![theirs]).u32();
        let other_name = e.mem.u32(slot);
        if (name == 0 && other_name != 0) || (name != 0 && other_name == 0) {
            return true;
        }
        if name != 0
            && other_name != 0
            && e.call(STRING_COMPARE_NO_CASE, &args![name, other_name])
                .i32()
                != 0
        {
            return true;
        }
        mine = e.call(LIST_NEXT, &args![mine]).u32();
        theirs = e.call(LIST_NEXT, &args![theirs]).u32();
    }
    mine != 0 || theirs != 0
}

// Translated from 0047fcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the file name list holds `name` (compared ignoring case); false
/// for an empty list.
pub fn fn_0047fcb0(e: &mut Engine, this: Ptr<TESAnimation>, name: Ptr) -> bool {
    let list = this.at(TESAnimation::kfFilenames);
    if e.call(LIST_IS_EMPTY, &args![list]).bool() {
        return false;
    }
    let mut node = list.addr();
    while node != 0 {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        if e.call(STRING_COMPARE_NO_CASE, &args![item, name]).i32() == 0 {
            return true;
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 0047fd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a KF file name to the list unless it is already there: makes a copy
/// of the string and appends it.
pub fn fn_0047fd20(e: &mut Engine, this: Ptr<TESAnimation>, name: Ptr) {
    if fn_0047fcb0(e, this, name) {
        return;
    }
    let size = e.call(STRING_LENGTH, &args![name]).u32() + 1;
    let copy = e.call(OPERATOR_NEW, &args![size]).u32();
    // The game keeps the copy's address in a stack slot and passes that
    // slot's address to the list.
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), copy);
        e.call(STRING_COPY, &args![copy, size, name]);
        let list = this.at(TESAnimation::kfFilenames);
        e.call(LIST_PUSH_BACK, &args![list, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047f940, fn_0047f940(Ptr<TESAIForm>, Ptr)),
        entry!(0x0047f980, fn_0047f980(Ptr<TESAIForm>, Ptr)),
        entry!(
            0x0047f9c0,
            fn_0047f9c0(Ptr<TESAnimation>) -> Ptr<TESAnimation>
        ),
        entry!(0x0047f9f0, fn_0047f9f0(Ptr<TESAnimation>)),
        entry!(0x0047fa50, fn_0047fa50(Ptr<TESAnimation>)),
        entry!(0x0047faa0, fn_0047faa0(Ptr<TESAnimation>, Ptr, Ptr)),
        entry!(0x0047fb50, fn_0047fb50(Ptr<TESAnimation>, Ptr)),
        entry!(0x0047fbd0, fn_0047fbd0(Ptr<TESAnimation>, Ptr) -> bool),
        entry!(0x0047fcb0, fn_0047fcb0(Ptr<TESAnimation>, Ptr) -> bool),
        entry!(0x0047fd20, fn_0047fd20(Ptr<TESAnimation>, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A simple model of the list functions in memory: the head node is
    /// inside the object, further nodes are 8-byte blocks (item, next).
    fn engine_with_lists() -> Engine {
        let mut e = Engine::new();
        e.register(LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(LIST_IS_EMPTY, |e, a| {
            let empty = e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0;
            Ret {
                eax: empty as u32,
                ..Ret::default()
            }
        });
        e.register(LIST_ITEM_SLOT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(LIST_NEXT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(LIST_POP_FRONT, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        e.register(LIST_PUSH_BACK, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            if e.mem.u32(node) == 0 {
                e.mem.set_u32(node, item);
            } else {
                let fresh = e.mem.alloc(8);
                e.mem.set_u32(fresh, item);
                e.mem.set_u32(node + 4, fresh);
            }
            Ret::default()
        });
        e.register(STRING_LENGTH, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        e.register(STRING_COPY, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRING_COMPARE_NO_CASE, |e, a| {
            let left = e.mem.cstr(a[0]).to_ascii_lowercase();
            let right = e.mem.cstr(a[1]).to_ascii_lowercase();
            Ret {
                eax: (left != right) as u32,
                ..Ret::default()
            }
        });
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e
    }

    fn animation(e: &mut Engine) -> Ptr<TESAnimation> {
        let this: Ptr<TESAnimation> = e.new_object();
        e.call(0x0047_f9c0, &args![this]);
        this
    }

    fn string(e: &mut Engine, text: &str) -> u32 {
        let block = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(block, text.as_bytes());
        block
    }

    /// The names in the list, in order.
    fn names(e: &Engine, this: Ptr<TESAnimation>) -> Vec<String> {
        let mut result = Vec::new();
        let mut node = this.addr() + 4;
        while node != 0 {
            let item = e.mem.u32(node);
            if item != 0 {
                result.push(String::from_utf8(e.mem.cstr(item)).unwrap());
            }
            node = e.mem.u32(node + 4);
        }
        result
    }

    #[test]
    fn ai_data_is_saved_only_when_the_buffer_flag_is_set() {
        let mut e = Engine::new();
        let this = Ptr::<TESAIForm>::new(e.mem.alloc(0x20));
        let buffer = e.mem.alloc(0x20);
        // The flags double writes the buffer's flags to the address it is
        // given and returns that address; the test double of the flag test
        // checks the mask.
        e.register(BUFFER_FLAGS, |e, a| {
            e.mem.set_u32(a[1], e.mem.u32(a[0] + 0x10));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(FLAGS_TEST, |e, a| Ret {
            eax: (e.mem.u32(a[0]) & a[1] != 0) as u32,
            ..Ret::default()
        });
        e.register(SAVE_RAW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_f940, &args![this, buffer]);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .iter()
            .all(|(addr, _)| *addr != SAVE_RAW));

        e.mem.set_u32(buffer + 0x10, 8);
        e.call_log = Some(vec![]);
        e.call(0x0047_f940, &args![this, buffer]);
        let log = e.call_log.take().unwrap();
        let write = log.iter().find(|(addr, _)| *addr == SAVE_RAW).unwrap();
        assert_eq!(write.1[..], [buffer, this.addr() + 4, 0x14, 0]);
        let test = log.iter().find(|(addr, _)| *addr == FLAGS_TEST).unwrap();
        assert_eq!(test.1[1], 8);
    }

    #[test]
    fn ai_data_is_loaded_only_when_the_buffer_flag_is_set() {
        let mut e = Engine::new();
        let this = Ptr::<TESAIForm>::new(e.mem.alloc(0x20));
        let buffer = e.mem.alloc(0x20);
        e.register(BUFFER_FLAGS, |e, a| {
            e.mem.set_u32(a[1], e.mem.u32(a[0] + 0x10));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(FLAGS_TEST, |e, a| Ret {
            eax: (e.mem.u32(a[0]) & a[1] != 0) as u32,
            ..Ret::default()
        });
        e.register(LOAD_RAW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_f980, &args![this, buffer]);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .iter()
            .all(|(addr, _)| *addr != LOAD_RAW));

        e.mem.set_u32(buffer + 0x10, 0xff);
        e.call_log = Some(vec![]);
        e.call(0x0047_f980, &args![this, buffer]);
        let log = e.call_log.take().unwrap();
        let read = log.iter().find(|(addr, _)| *addr == LOAD_RAW).unwrap();
        assert_eq!(read.1[..], [buffer, this.addr() + 4, 0x14]);
    }

    #[test]
    fn constructor_builds_the_base_vtable_and_an_empty_list() {
        let mut e = engine_with_lists();
        e.register(BASE_FORM_COMPONENT_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this: Ptr<TESAnimation> = e.new_object();
        e.mem.set_u32(this.addr() + 4, 0x1234);
        let back = e.call(0x0047_f9c0, &args![this]).ptr::<()>();
        assert_eq!(back.addr(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), ANIMATION_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        assert_eq!(e.mem.u32(this.addr() + 8), 0);
    }

    #[test]
    fn destructor_body_clears_the_names_and_destroys_the_list() {
        let mut e = engine_with_lists();
        e.register(BASE_FORM_COMPONENT_CONSTRUCT, |_, _| Ret::default());
        let this = animation(&mut e);
        for name in ["a.kf", "b.kf"] {
            let text = string(&mut e, name);
            fn_0047fd20(&mut e, this, Ptr::new(text));
        }
        let deleted = Rc::new(RefCell::new(Vec::new()));
        let sink = deleted.clone();
        e.register_double(OPERATOR_DELETE, move |e, a| {
            sink.borrow_mut().push(e.mem.cstr(a[0]));
            Ret::default()
        });
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_f9f0, &args![this]);
        assert_eq!(*deleted.borrow(), vec![b"a.kf".to_vec(), b"b.kf".to_vec()]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == LIST_DESTRUCT && a[0] == this.addr() + 4));
        assert_eq!(e.mem.u32(this.addr()), ANIMATION_VTABLE);
    }

    #[test]
    fn clear_on_an_empty_list_does_nothing() {
        let mut e = engine_with_lists();
        e.register(BASE_FORM_COMPONENT_CONSTRUCT, |_, _| Ret::default());
        let this = animation(&mut e);
        e.register(OPERATOR_DELETE, |_, _| panic!("nothing to free"));
        e.call(0x0047_fa50, &args![this]);
    }

    #[test]
    fn kff_chunk_adds_every_name() {
        let mut e = engine_with_lists();
        e.register(BASE_FORM_COMPONENT_CONSTRUCT, |_, _| Ret::default());
        let this = animation(&mut e);
        let file = e.mem.alloc(8);
        e.register(TES_FILE_GET_TES_CHUNK, |_, _| Ret {
            eax: CHUNK_KF_FILE_NAMES,
            ..Ret::default()
        });
        e.register(TES_FILE_CHUNK_SIZE, |_, _| Ret {
            eax: 16,
            ..Ret::default()
        });
        let block = e.mem.alloc(16);
        e.register_double(OPERATOR_NEW, move |e, a| {
            // The chunk buffer is the first allocation, copies follow.
            let _ = a;
            Ret {
                eax: e.mem.alloc(a[0]),
                ..Ret::default()
            }
        });
        let _ = block;
        e.register(TES_FILE_GET_CHUNK_DATA, |e, a| {
            assert_eq!(a[2], 0);
            e.mem.write(a[1], b"One.kf\0Two.kf\0\0");
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.call(0x0047_faa0, &args![this, this, file]);
        assert_eq!(names(&e, this), vec!["One.kf", "Two.kf"]);
    }

    #[test]
    fn kff_chunk_is_ignored_for_other_chunks_and_missing_arguments() {
        let mut e = engine_with_lists();
        e.register(BASE_FORM_COMPONENT_CONSTRUCT, |_, _| Ret::default());
        let this = animation(&mut e);
        let file = e.mem.alloc(8);
        e.register(TES_FILE_GET_TES_CHUNK, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.register(TES_FILE_CHUNK_SIZE, |_, _| panic!("not the KFFZ chunk"));
        e.call(0x0047_faa0, &args![this, this, file]);
        e.call(0x0047_faa0, &args![this, 0u32, file]);
        e.call(0x0047_faa0, &args![this, this, 0u32]);
        assert!(names(&e, this).is_empty());
    }

    #[test]
    fn contains_and_add_ignore_case_and_skip_duplicates() {
        let mut e = engine_with_lists();
        e.register(BASE_FORM_COMPONENT_CONSTRUCT, |_, _| Ret::default());
        let this = animation(&mut e);
        e.register_double(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        let lower = string(&mut e, "walk.kf");
        let upper = string(&mut e, "WALK.KF");
        let other = string(&mut e, "run.kf");
        assert!(!e.call(0x0047_fcb0, &args![this, lower]).bool());
        e.call(0x0047_fd20, &args![this, lower]);
        assert!(e.call(0x0047_fcb0, &args![this, upper]).bool());
        e.call(0x0047_fd20, &args![this, upper]);
        assert_eq!(names(&e, this), vec!["walk.kf"]);
        assert!(!e.call(0x0047_fcb0, &args![this, other]).bool());
        e.call(0x0047_fd20, &args![this, other]);
        assert_eq!(names(&e, this), vec!["walk.kf", "run.kf"]);
        // The stored name is a copy.
        let first = e.mem.u32(this.addr() + 4);
        assert_ne!(first, lower);
    }

    #[test]
    fn copy_replaces_the_names_with_the_sources() {
        let mut e = engine_with_lists();
        e.register(BASE_FORM_COMPONENT_CONSTRUCT, |_, _| Ret::default());
        e.register_double(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this = animation(&mut e);
        let source = animation(&mut e);
        for name in ["a.kf", "b.kf"] {
            let text = string(&mut e, name);
            fn_0047fd20(&mut e, source, Ptr::new(text));
        }
        let old = string(&mut e, "old.kf");
        fn_0047fd20(&mut e, this, Ptr::new(old));
        // The virtual ClearDataComponent slot (+4) of the vtable.
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0047_fa50);
        e.mem.set_u32(this.addr(), vtable);
        e.call(0x0047_fb50, &args![this, source]);
        assert_eq!(names(&e, this), vec!["a.kf", "b.kf"]);
        assert_eq!(names(&e, source), vec!["a.kf", "b.kf"]);
    }

    #[test]
    fn copy_from_another_kind_of_component_does_nothing() {
        let mut e = engine_with_lists();
        e.register(BASE_FORM_COMPONENT_CONSTRUCT, |_, _| Ret::default());
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        let this = animation(&mut e);
        let old = string(&mut e, "old.kf");
        e.register_double(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        fn_0047fd20(&mut e, this, Ptr::new(old));
        e.call(0x0047_fb50, &args![this, 0x1000u32]);
        assert_eq!(names(&e, this), vec!["old.kf"]);
    }

    #[test]
    fn compare_walks_both_lists() {
        let mut e = engine_with_lists();
        e.register(BASE_FORM_COMPONENT_CONSTRUCT, |_, _| Ret::default());
        e.register_double(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let left = animation(&mut e);
        let right = animation(&mut e);
        // Two empty lists are equal.
        assert!(!e.call(0x0047_fbd0, &args![left, right]).bool());
        for (animation, name) in [(left, "a.kf"), (right, "A.KF")] {
            let text = string(&mut e, name);
            fn_0047fd20(&mut e, animation, Ptr::new(text));
        }
        // Equal ignoring case.
        assert!(!e.call(0x0047_fbd0, &args![left, right]).bool());
        // A different name.
        let extra = string(&mut e, "b.kf");
        fn_0047fd20(&mut e, left, Ptr::new(extra));
        let other = string(&mut e, "c.kf");
        fn_0047fd20(&mut e, right, Ptr::new(other));
        assert!(e.call(0x0047_fbd0, &args![left, right]).bool());
    }

    #[test]
    fn compare_differs_by_length_and_for_other_components() {
        let mut e = engine_with_lists();
        e.register(BASE_FORM_COMPONENT_CONSTRUCT, |_, _| Ret::default());
        e.register_double(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let left = animation(&mut e);
        let right = animation(&mut e);
        for name in ["a.kf", "b.kf"] {
            let text = string(&mut e, name);
            fn_0047fd20(&mut e, left, Ptr::new(text));
        }
        let text = string(&mut e, "a.kf");
        fn_0047fd20(&mut e, right, Ptr::new(text));
        // The shorter list ends first.
        assert!(e.call(0x0047_fbd0, &args![left, right]).bool());
        assert!(e.call(0x0047_fbd0, &args![right, left]).bool());
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        assert!(e.call(0x0047_fbd0, &args![left, right]).bool());
    }
}
