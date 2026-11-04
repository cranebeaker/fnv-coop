//! `fallout shared/run_004a4810` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Template code the compiler emitted next to the decal manager: an
//! `hkArray` of 0x60-byte elements (destructor, buffer constructor,
//! clear-and-free, the allocator's block free), `NiTPointerList` helpers
//! for a list of `NiPointer`s (pop the head into a smart pointer, free a
//! node), and the `SettingT<RendererSettingCollection>` constructor.
//!
//! The `hkArray` fields (Xbox PDB `hkArrayBase`): `m_data` +0, `m_size`
//! +4, `m_capacityAndFlags` +8; bit 31 of the last means the array does
//! not free its buffer (the code skips the free when it is set), and the
//! low 30 bits are the capacity (`0062a100`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTListItem, NiTPointerList};

/// `hkArrayBase::m_capacityAndFlags` bit 31: the buffer is not freed.
const DONT_FREE_BUFFER: u32 = 0x8000_0000;
/// Element size of the array these functions handle.
const ELEMENT_SIZE: u32 = 0x60;
/// Returns the calling thread's object from the TLS slot at `01268108`
/// (`00c85750`, no arguments; the engine map puts it in Havok's
/// `hkmemoryrouter.cpp`).
const THREAD_MEMORY_ROUTER: u32 = 0x00c8_5750;
/// Returns the word at +0x10 of `this` (`0044edb0`, thiscall; the engine
/// map's name for it, `BaseProcess::GetCurrentProcedureIndex`, belongs to
/// another body folded onto it).
const WORD_AT_10: u32 = 0x0044_edb0;
/// Clears the array's elements (`00626870`, thiscall).
const ARRAY_CLEAR: u32 = 0x0062_6870;
/// The array's capacity, `m_capacityAndFlags & 0x3fffffff` (`0062a100`,
/// thiscall).
const ARRAY_CAPACITY: u32 = 0x0062_a100;
/// `NiPointer` copy constructor (`00559a40`, thiscall, source pointer).
const NI_POINTER_COPY: u32 = 0x0055_9a40;
/// `NiPointer` assignment from a raw pointer (`0066b0d0`, thiscall).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer` destructor (`0045cec0`, thiscall).
const NI_POINTER_RELEASE: u32 = 0x0045_cec0;
/// Frees a list node through the list's allocator at +8 (`0045cee0`,
/// thiscall, node).
const LIST_FREE_NODE: u32 = 0x0045_cee0;
/// The `Setting` base constructor (`00404920`, thiscall, two words).
const SETTING_INIT: u32 = 0x0040_4920;
/// The vtable of `SettingT<RendererSettingCollection>` (`010184fc`; its
/// RTTI type descriptor at `011860ac` names the class).
const RENDERER_SETTING_VTABLE: u32 = 0x0101_84fc;
/// Returns the collection the setting is added to (`0045d180`, no
/// arguments: calls `0045d190` and returns the word at `011f35a4`).
const RENDERER_SETTING_COLLECTION: u32 = 0x0045_d180;

// Translated from 004a4810 (decompiled, FalloutNV.exe 1.4.0.525)
/// The array's destructor: clears and frees it (`004a4990`) through the
/// allocator `004a4840` returns. The zeroed stack byte is the `this` of
/// `004a4840`.
pub fn fn_004a4810(e: &mut Engine, this: Ptr) {
    let allocator = e.with_stack(1, |e, local| fn_004a4840(e, local, this.addr()));
    fn_004a4990(e, this, allocator);
}

// Translated from 004a4840 (decompiled, FalloutNV.exe 1.4.0.525)
/// The heap allocator: the word at +0x10 (`0044edb0`) of the thread's
/// object from `00c85750`. Neither `this` nor the stack word is read.
pub fn fn_004a4840(e: &mut Engine, _this: Ptr, _unused_1: u32) -> u32 {
    let router = e.call(THREAD_MEMORY_ROUTER, &args![]).u32();
    e.call(WORD_AT_10, &args![router]).u32()
}

// Translated from 004a4860 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a4860 - the exception-unwinding frame (FS:[0], states 0-1) is not translated
/// Removes the list's head node and returns its element as an `NiPointer`
/// constructed at `result`: unlinks the head (the next node's previous
/// link, or the tail when the list becomes empty, set to null), copies the
/// node's `NiPointer` into a temporary, frees the node (`004a4930`, which
/// also releases the node's pointer), decrements the count, copy-constructs
/// `result` from the temporary and destroys the temporary. The list must
/// not be empty. Returns `result`.
pub fn fn_004a4860(e: &mut Engine, this: Ptr<NiTPointerList>, result: Ptr) -> Ptr {
    let node = e.get(this, NiTPointerList::m_pkHead);
    let next = e.mem.u32(node); // NiTListItem::m_pkNext
    e.set(this, NiTPointerList::m_pkHead, next);
    if next == 0 {
        e.set(this, NiTPointerList::m_pkTail, 0);
    } else {
        e.set(Ptr::<NiTListItem>::new(next), NiTListItem::m_pkPrev, 0);
    }
    e.with_stack(4, |e, temporary| {
        e.call(NI_POINTER_COPY, &args![temporary, node + 8]);
        fn_004a4930(e, this, node);
        let count = e.get(this, NiTPointerList::m_uiCount);
        e.set(this, NiTPointerList::m_uiCount, count.wrapping_sub(1));
        e.call(NI_POINTER_COPY, &args![result, temporary]);
        e.call(NI_POINTER_RELEASE, &args![temporary]);
    });
    result
}

// Translated from 004a4930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees a node of a list of `NiPointer`s: sets the node's pointer to null
/// (`0066b0d0`), then frees the node through the list's allocator
/// (`0045cee0` on `this` +8).
pub fn fn_004a4930(e: &mut Engine, this: Ptr<NiTPointerList>, node: u32) {
    e.call(NI_POINTER_ASSIGN, &args![node + 8, 0u32]);
    e.call(LIST_FREE_NODE, &args![this.addr() + 8, node]);
}

// Translated from 004a4960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The array over a caller's buffer: `m_data = data`, `m_size = size`,
/// `m_capacityAndFlags = capacity | 0x80000000` (not freed). Returns
/// `this`.
pub fn fn_004a4960(e: &mut Engine, this: Ptr, data: u32, size: u32, capacity: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, data);
    e.mem.set_u32(base + 4, size);
    e.mem.set_u32(base + 8, capacity | DONT_FREE_BUFFER);
    this
}

// Translated from 004a4990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the array (`00626870`) and, unless bit 31 of
/// `m_capacityAndFlags` is set, frees its buffer through `allocator`
/// (`004a4a10` with `m_data` and the capacity from `0062a100`); then
/// `m_data = 0` and `m_capacityAndFlags = 0x80000000`.
pub fn fn_004a4990(e: &mut Engine, this: Ptr, allocator: u32) {
    let base = this.addr();
    e.call(ARRAY_CLEAR, &args![this]);
    if e.mem.u32(base + 8) & DONT_FREE_BUFFER == 0 {
        let capacity = e.call(ARRAY_CAPACITY, &args![this]).u32();
        let data = e.mem.u32(base);
        fn_004a4a10(e, Ptr::new(allocator), data, capacity);
    }
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 8, DONT_FREE_BUFFER);
}

// Translated from 004a49e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees a node of a list of plain pointers: sets the node's element to 0,
/// then frees the node through the list's allocator (`0045cee0` on `this`
/// +8).
pub fn fn_004a49e0(e: &mut Engine, this: Ptr<NiTPointerList>, node: u32) {
    e.set(Ptr::<NiTListItem>::new(node), NiTListItem::m_element, 0);
    e.call(LIST_FREE_NODE, &args![this.addr() + 8, node]);
}

// Translated from 004a4a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees a buffer of `count` 0x60-byte elements: the allocator's virtual at
/// +0x10 with the pointer and `count * 0x60` bytes.
pub fn fn_004a4a10(e: &mut Engine, this: Ptr, data: u32, count: u32) {
    e.vcall(
        this.addr(),
        0x10,
        &args![data, count.wrapping_mul(ELEMENT_SIZE)],
    );
}

// Translated from 004a4a40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a4a40 - the exception-unwinding frame (FS:[0], state 0) is not translated
/// The `SettingT<RendererSettingCollection>` constructor (vtable
/// `010184fc`): the `Setting` base (`00404920` with the two words), the
/// vtable, then the setting adds itself to its collection (`0045d180`'s
/// result, virtual +4 with `this`). Returns `this`.
pub fn fn_004a4a40(e: &mut Engine, this: Ptr, name: u32, value: u32) -> Ptr {
    e.call(SETTING_INIT, &args![this, name, value]);
    e.mem.set_u32(this.addr(), RENDERER_SETTING_VTABLE);
    let collection = e.call(RENDERER_SETTING_COLLECTION, &args![]).u32();
    e.vcall(collection, 4, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004a4810, fn_004a4810(Ptr)),
        entry!(0x004a4840, fn_004a4840(Ptr, u32) -> u32),
        entry!(0x004a4860, fn_004a4860(Ptr<NiTPointerList>, Ptr) -> Ptr),
        entry!(0x004a4930, fn_004a4930(Ptr<NiTPointerList>, u32)),
        entry!(0x004a4960, fn_004a4960(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x004a4990, fn_004a4990(Ptr, u32)),
        entry!(0x004a49e0, fn_004a49e0(Ptr<NiTPointerList>, u32)),
        entry!(0x004a4a10, fn_004a4a10(Ptr, u32, u32)),
        entry!(0x004a4a40, fn_004a4a40(Ptr, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn quiet_double(e: &mut Engine, addr: u32) {
        e.register(addr, |_, _| Ret::default());
    }

    /// An allocator object whose virtual +0x10 is `00ab0010`.
    fn allocator(e: &mut Engine) -> u32 {
        e.put_vtable(0x0102_0000, &[0, 0, 0, 0, 0x00ab_0010]);
        quiet_double(e, 0x00ab_0010);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x0102_0000);
        object
    }

    fn array_engine() -> Engine {
        let mut e = Engine::new();
        quiet_double(&mut e, ARRAY_CLEAR);
        e.register(ARRAY_CAPACITY, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8) & 0x3fff_ffff,
            ..Ret::default()
        });
        e
    }

    #[test]
    fn buffer_constructor_marks_not_freed() {
        let mut e = Engine::new();
        let array = e.mem.alloc(0xc);
        assert_eq!(
            fn_004a4960(&mut e, Ptr::new(array), 0x1000, 3, 8).addr(),
            array
        );
        assert_eq!(e.mem.u32(array), 0x1000);
        assert_eq!(e.mem.u32(array + 4), 3);
        assert_eq!(e.mem.u32(array + 8), 0x8000_0008);
    }

    #[test]
    fn clear_frees_owned_buffer_only() {
        let mut e = array_engine();
        let alloc = allocator(&mut e);
        let array = e.mem.alloc(0xc);
        e.mem.set_u32(array, 0x5000);
        e.mem.set_u32(array + 8, 4);
        e.call_log = Some(vec![]);
        fn_004a4990(&mut e, Ptr::new(array), alloc);
        let log: CallLog = e.call_log.take().unwrap();
        let frees: Vec<_> = log.iter().filter(|c| c.0 == 0x00ab_0010).collect();
        assert_eq!(frees[0].1, [alloc, 0x5000, 4 * 0x60]);
        assert!(log.iter().any(|c| c.0 == ARRAY_CLEAR));
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(array + 8), 0x8000_0000);

        // Not owned: no free.
        e.mem.set_u32(array, 0x5000);
        e.mem.set_u32(array + 8, 0x8000_0004);
        e.call_log = Some(vec![]);
        fn_004a4990(&mut e, Ptr::new(array), alloc);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == 0x00ab_0010));
        assert_eq!(e.mem.u32(array), 0);
    }

    #[test]
    fn destructor_uses_the_threads_heap() {
        let mut e = array_engine();
        let alloc = allocator(&mut e);
        let router = e.mem.alloc(0x14);
        e.mem.set_u32(router + 0x10, alloc);
        e.register_double(THREAD_MEMORY_ROUTER, move |_, a| {
            assert!(a.is_empty());
            Ret {
                eax: router,
                ..Ret::default()
            }
        });
        e.register(WORD_AT_10, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x10),
            ..Ret::default()
        });
        let array = e.mem.alloc(0xc);
        e.mem.set_u32(array, 0x6000);
        e.mem.set_u32(array + 8, 2);
        e.call_log = Some(vec![]);
        fn_004a4810(&mut e, Ptr::new(array));
        let log = e.call_log.take().unwrap();
        let frees: Vec<_> = log.iter().filter(|c| c.0 == 0x00ab_0010).collect();
        assert_eq!(frees[0].1, [alloc, 0x6000, 0xc0]);
        assert_eq!(fn_004a4840(&mut e, Ptr::new(0), 0), alloc);
    }

    #[test]
    fn block_free_passes_bytes() {
        let mut e = Engine::new();
        let alloc = allocator(&mut e);
        e.call_log = Some(vec![]);
        fn_004a4a10(&mut e, Ptr::new(alloc), 0x7000, 5);
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![(0x00ab_0010, vec![alloc, 0x7000, 5 * 0x60])]
        );
    }

    fn list_with(e: &mut Engine, elements: &[u32]) -> (Ptr<NiTPointerList>, Vec<u32>) {
        let list = e.new_object::<NiTPointerList>();
        let nodes: Vec<u32> = elements.iter().map(|_| e.mem.alloc(0xc)).collect();
        for (i, &node) in nodes.iter().enumerate() {
            e.mem.set_u32(node + 8, elements[i]);
            e.mem.set_u32(node, nodes.get(i + 1).copied().unwrap_or(0));
            e.mem
                .set_u32(node + 4, if i == 0 { 0 } else { nodes[i - 1] });
        }
        e.set(list, NiTPointerList::m_pkHead, nodes[0]);
        e.set(list, NiTPointerList::m_pkTail, *nodes.last().unwrap());
        e.set(list, NiTPointerList::m_uiCount, nodes.len() as u32);
        (list, nodes)
    }

    #[test]
    fn pop_head_returns_the_pointer() {
        let mut e = Engine::new();
        // Copies are plain word copies here.
        e.register(NI_POINTER_COPY, |e, a| {
            let v = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], v);
            Ret::default()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        quiet_double(&mut e, NI_POINTER_RELEASE);
        quiet_double(&mut e, LIST_FREE_NODE);
        let (list, nodes) = list_with(&mut e, &[0x111, 0x222]);
        let result = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(fn_004a4860(&mut e, list, Ptr::new(result)).addr(), result);
        assert_eq!(e.mem.u32(result), 0x111);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), nodes[1]);
        assert_eq!(e.mem.u32(nodes[1] + 4), 0);
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), nodes[1]);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 1);
        let log = e.call_log.take().unwrap();
        let freed: Vec<_> = log.iter().filter(|c| c.0 == LIST_FREE_NODE).collect();
        assert_eq!(freed[0].1, [list.addr() + 8, nodes[0]]);
        assert_eq!(e.mem.u32(nodes[0] + 8), 0);

        // The last node: the tail becomes null too.
        fn_004a4860(&mut e, list, Ptr::new(result));
        assert_eq!(e.mem.u32(result), 0x222);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), 0);
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), 0);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 0);
    }

    #[test]
    fn node_frees() {
        let mut e = Engine::new();
        quiet_double(&mut e, NI_POINTER_ASSIGN);
        quiet_double(&mut e, LIST_FREE_NODE);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, 0x99);
        e.call_log = Some(vec![]);
        fn_004a4930(&mut e, Ptr::new(0x4000), node);
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![
                (NI_POINTER_ASSIGN, vec![node + 8, 0]),
                (LIST_FREE_NODE, vec![0x4008, node]),
            ]
        );

        e.call_log = Some(vec![]);
        fn_004a49e0(&mut e, Ptr::new(0x4000), node);
        assert_eq!(e.mem.u32(node + 8), 0);
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![(LIST_FREE_NODE, vec![0x4008, node])]
        );
    }

    #[test]
    fn renderer_setting_registers_itself() {
        let mut e = Engine::new();
        quiet_double(&mut e, SETTING_INIT);
        e.put_vtable(0x0103_0000, &[0, 0x00ab_0004]);
        quiet_double(&mut e, 0x00ab_0004);
        let collection = e.mem.alloc(4);
        e.mem.set_u32(collection, 0x0103_0000);
        e.register_double(RENDERER_SETTING_COLLECTION, move |_, _| Ret {
            eax: collection,
            ..Ret::default()
        });
        let setting = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(
            fn_004a4a40(&mut e, Ptr::new(setting), 0x0106_0000, 7).addr(),
            setting
        );
        assert_eq!(e.mem.u32(setting), RENDERER_SETTING_VTABLE);
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![
                (SETTING_INIT, vec![setting, 0x0106_0000, 7]),
                (RENDERER_SETTING_COLLECTION, vec![]),
                (0x00ab_0004, vec![collection, setting]),
            ]
        );
    }
}
