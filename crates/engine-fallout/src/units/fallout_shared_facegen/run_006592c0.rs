//! `fallout shared/facegen/run_006592c0` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `CreateSemaphoreA` import slot (`kernel32`, stdcall: attributes, initial count, maximum
/// count, name).
const CREATE_SEMAPHORE_A: u32 = 0x00fd_f0b4;

// Translated from 00659500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the two words at `+0` and `+4`, then creates a
/// semaphore with `CreateSemaphoreA(null, [+0], [+4], null)` (so the first word is the initial
/// count and the second the maximum count) and stores its handle at `+8`. Returns `this`.
pub fn fn_00659500(e: &mut Engine, this: Ptr, initial_count: u32, maximum_count: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, initial_count);
    e.mem.set_u32(base + 4, maximum_count);
    let initial = e.mem.u32(base);
    let maximum = e.mem.u32(base + 4);
    let handle = e
        .call(CREATE_SEMAPHORE_A, &args![0u32, initial, maximum, 0u32])
        .u32();
    e.mem.set_u32(base + 8, handle);
    this
}

/// The vtables of the face-model map's classes (RTTI): `NiTStringTemplateMap<NiTMap<char const
/// *, NiPointer<BSFaceGenModelMap::Entry>>, ...>`, `NiTMap<...>` and `NiTMapBase<DFALL<...>,
/// ...>`.
const STRING_TEMPLATE_MAP_VTABLE: u32 = 0x0106_5890;
const MAP_VTABLE: u32 = 0x0106_58b0;
const MAP_BASE_VTABLE: u32 = 0x0106_58d0;
/// `00aa1070` (`cdecl`, the size): allocates a block.
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
/// `00403d30` (`cdecl`: block, byte, count): the fill.
const MEMORY_FILL: u32 = 0x0040_3d30;
/// `operator new` (`00401000`) and `operator delete` (`00401030`), `cdecl`, one word.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `CloseHandle` import slot (`kernel32`, stdcall: the handle).
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
/// The string destructor (`0044a540`, `thiscall`) and the string from a C string
/// (`0044a4c0`, `thiscall`: the characters).
const STRING_DESTRUCT: u32 = 0x0044_a540;
const STRING_FROM_C: u32 = 0x0044_a4c0;
/// `00449680` (`thiscall`, no stack words): the destructor `006593e0` runs on the object's
/// start (the one other code runs on an `FR2MatrixVTC<float>`).
const BASE_DESTRUCT_00449680: u32 = 0x0044_9680;
/// `_eh_vector_constructor_iterator_` (`00ec782f`: block, element size, count, constructor,
/// destructor) and `_eh_vector_destructor_iterator_` (`00ec5fce`: block, element size,
/// count, destructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// The two element types of the morph-data pairs: constructor, destructor and the clean-up
/// the pair's destructor runs on each element first. The pairs of `00659560` use `0065cca0`,
/// `0065cd10`, `0065cd70`; those of `00659670` use `0065cdd0`, `0065ce40`, `0065cef0` (all
/// `thiscall`, no stack words).
const EGM_ELEMENT_CONSTRUCT: u32 = 0x0065_cca0;
const EGM_ELEMENT_DESTRUCT: u32 = 0x0065_cd10;
const EGM_ELEMENT_CLEAR: u32 = 0x0065_cd70;
const EGT_ELEMENT_CONSTRUCT: u32 = 0x0065_cdd0;
const EGT_ELEMENT_DESTRUCT: u32 = 0x0065_ce40;
const EGT_ELEMENT_CLEAR: u32 = 0x0065_cef0;
/// `fffReadEgmFile` (`00aafc80`) and `fffReadEgtFile` (`00ab2640`), both `cdecl` with four
/// words: the file name string, the object and its two element blocks (+0x04, +0x1C).
const READ_EGM_FILE: u32 = 0x00aa_fc80;
const READ_EGT_FILE: u32 = 0x00ab_2640;

// Translated from 006592f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; `this` is not read): deletes `element`
/// through `00659360`. Returns what that leaves in `EAX` (the element).
pub fn fn_006592f0(e: &mut Engine, _this: Ptr, element: Ptr) -> Ptr {
    fn_00659360(e, element)
}

// Translated from 00659330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the face-model map's `NiTStringTemplateMap` constructor
/// (`thiscall`: the hash size and the copy-keys byte): builds the `NiTMap` (`00659460`), sets
/// the class's vtable and stores the byte at +0x10. Returns `this`.
pub fn fn_00659330(e: &mut Engine, this: Ptr, hash_size: u32, copy_keys: u8) -> Ptr {
    fn_00659460(e, this, hash_size);
    e.mem.set_u32(this.addr(), STRING_TEMPLATE_MAP_VTABLE);
    e.mem.set_u8(this.addr() + 0x10, copy_keys);
    this
}

// Translated from 00659360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): runs the scalar deleting destructor
/// `00659380` on `element` with flag 0 (no free). Returns `element`.
pub fn fn_00659360(e: &mut Engine, element: Ptr) -> Ptr {
    fn_00659380(e, element, 0)
}

// Translated from 00659380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a scalar deleting destructor (`thiscall`, the flags): the
/// destructor `006593e0`, then `operator delete` with bit 0 of `flags`. Returns `this`.
pub fn fn_00659380(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006593e0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006593e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006593e0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a destructor: destroys the string at +0x20 (`0044a540`), then
/// runs `00449680` on the object.
pub fn fn_006593e0(e: &mut Engine, this: Ptr) {
    e.call(STRING_DESTRUCT, &args![this.addr() + 0x20]);
    e.call(BASE_DESTRUCT_00449680, &args![this]);
}

// Translated from 00659460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the face-model map's `NiTMap` constructor (`thiscall`, the hash
/// size): builds the `NiTMapBase` (`00659490`) and sets the `NiTMap` vtable. Returns `this`.
pub fn fn_00659460(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_00659490(e, this, hash_size);
    e.mem.set_u32(this.addr(), MAP_VTABLE);
    this
}

// Translated from 00659490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the face-model map's `NiTMapBase` constructor (`thiscall`, the
/// hash size): sets the vtable, `m_uiHashSize` (+4) and the count (+0xC, 0), allocates the
/// hash table of `hash_size` words (`00aa1070`, +8) and zeroes it (`00403d30`). Returns
/// `this`.
pub fn fn_00659490(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, MAP_BASE_VTABLE);
    e.mem.set_u32(base + 4, hash_size);
    e.mem.set_u32(base + 0xc, 0);
    let bytes = e.mem.u32(base + 4).wrapping_shl(2);
    let table = e.call(ALLOCATE_00AA1070, &args![bytes]).u32();
    e.mem.set_u32(base + 8, table);
    let bytes = e.mem.u32(base + 4).wrapping_shl(2);
    let table = e.mem.u32(base + 8);
    e.call(MEMORY_FILL, &args![table, 0u32, bytes]);
    this
}

// Translated from 00659540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: closes the handle at +8 (`CloseHandle`; the semaphore
/// `00659500` creates). Returns the import's result.
pub fn fn_00659540(e: &mut Engine, this: Ptr) -> u32 {
    let handle = e.mem.u32(this.addr() + 8);
    e.call(CLOSE_HANDLE, &args![handle]).u32()
}

/// The morph-data pair constructors `00659560` and `00659670`: the two 0x18-byte elements at
/// +0x04 (`_eh_vector_constructor_iterator_`), the file name as a temporary string, the file
/// read on (name, object, +0x04, +0x1C), and the temporary destroyed.
fn construct_and_read(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    construct: u32,
    destruct: u32,
    read: u32,
) -> Ptr {
    let base = this.addr();
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![base + 4, 0x18u32, 2u32, construct, destruct],
    );
    e.with_stack(0x1c, |e, string| {
        e.call(STRING_FROM_C, &args![string, name]);
        e.call(read, &args![string, base, base + 4, base + 0x1c]);
        e.call(STRING_DESTRUCT, &args![string]);
    });
    this
}

/// The pair destructors `00659600` and `00659710`: the clean-up on each element, then
/// `_eh_vector_destructor_iterator_` over both.
fn clear_and_destruct(e: &mut Engine, this: Ptr, clear: u32, destruct: u32) {
    let base = this.addr();
    e.call(clear, &args![base + 4]);
    e.call(clear, &args![base + 0x1c]);
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &args![base + 4, 0x18u32, 2u32, destruct],
    );
}

// Translated from 00659560 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00659560 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a morph-data constructor (`thiscall`, a C string: the file
/// name): constructs its two elements (`0065cca0` / `0065cd10`) and reads the file with
/// `fffReadEgmFile` (`00aafc80`). Returns `this`.
pub fn fn_00659560(e: &mut Engine, this: Ptr, name: u32) -> Ptr {
    construct_and_read(
        e,
        this,
        name,
        EGM_ELEMENT_CONSTRUCT,
        EGM_ELEMENT_DESTRUCT,
        READ_EGM_FILE,
    )
}

// Translated from 00659600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00659600 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: the destructor of `00659560`'s object: `0065cd70` on each
/// element, then destroys both (`0065cd10`).
pub fn fn_00659600(e: &mut Engine, this: Ptr) {
    clear_and_destruct(e, this, EGM_ELEMENT_CLEAR, EGM_ELEMENT_DESTRUCT);
}

// Translated from 00659670 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00659670 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: the other morph-data constructor (`thiscall`, a C string: the
/// file name): constructs its two elements (`0065cdd0` / `0065ce40`) and reads the file with
/// `fffReadEgtFile` (`00ab2640`). Returns `this`.
pub fn fn_00659670(e: &mut Engine, this: Ptr, name: u32) -> Ptr {
    construct_and_read(
        e,
        this,
        name,
        EGT_ELEMENT_CONSTRUCT,
        EGT_ELEMENT_DESTRUCT,
        READ_EGT_FILE,
    )
}

// Translated from 00659710 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00659710 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: the destructor of `00659670`'s object: `0065cef0` on each
/// element, then destroys both (`0065ce40`).
pub fn fn_00659710(e: &mut Engine, this: Ptr) {
    clear_and_destruct(e, this, EGT_ELEMENT_CLEAR, EGT_ELEMENT_DESTRUCT);
}

// Translated from 00659780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00659780 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, a C string: the file name): allocates 0x34 bytes
/// (`operator new`) and constructs them with `00659670`; null when the allocation fails.
pub fn fn_00659780(e: &mut Engine, name: u32) -> Ptr {
    let block: Ptr = e.call(OPERATOR_NEW, &args![0x34u32]).ptr();
    if block.is_null() {
        return Ptr::NULL;
    }
    fn_00659670(e, block, name)
}

/// `00658d30` (`thiscall`, no stack words): the destructor `006592c0` runs.
const DESTRUCT_00658D30: u32 = 0x0065_8d30;

// Translated from 006592c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a scalar deleting destructor (`thiscall`, one word: the
/// flags): `00658d30`, then `operator delete` when bit 0 of `flags` is set. Answers `this`.
pub fn fn_006592c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCT_00658D30, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00659310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a destroy callback (`thiscall`, one word: the element; `this`
/// is not read): [`fn_00659370`] on the element.
pub fn fn_00659310(e: &mut Engine, _this: Ptr, element: Ptr) {
    fn_00659370(e, element);
}

// Translated from 00659370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): [`fn_006593b0`] on `element` with flags 0
/// (destroys without freeing).
pub fn fn_00659370(e: &mut Engine, element: Ptr) {
    fn_006593b0(e, element, 0);
}

// Translated from 006593b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a scalar deleting destructor (`thiscall`, one word: the
/// flags): [`fn_00659440`], then `operator delete` when bit 0 of `flags` is set. Answers
/// `this`.
pub fn fn_006593b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00659440(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00659440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra's library match, `SubAllocator::~SubAllocator`, is
/// not this body): destroys the four 0x20-byte elements at `+8`
/// (`_eh_vector_destructor_iterator_` with `00449680`).
pub fn fn_00659440(e: &mut Engine, this: Ptr) {
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &args![this.addr() + 8, 0x20u32, 4u32, BASE_DESTRUCT_00449680],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_9500, fn_00659500(Ptr, u32, u32) -> Ptr),
        entry!(0x0065_92f0, fn_006592f0(Ptr, Ptr) -> Ptr),
        entry!(0x0065_9330, fn_00659330(Ptr, u32, u8) -> Ptr),
        entry!(0x0065_9360, fn_00659360(Ptr) -> Ptr),
        entry!(0x0065_9380, fn_00659380(Ptr, u32) -> Ptr),
        entry!(0x0065_93e0, fn_006593e0(Ptr)),
        entry!(0x0065_9460, fn_00659460(Ptr, u32) -> Ptr),
        entry!(0x0065_9490, fn_00659490(Ptr, u32) -> Ptr),
        entry!(0x0065_9540, fn_00659540(Ptr) -> u32),
        entry!(0x0065_9560, fn_00659560(Ptr, u32) -> Ptr),
        entry!(0x0065_9600, fn_00659600(Ptr)),
        entry!(0x0065_9670, fn_00659670(Ptr, u32) -> Ptr),
        entry!(0x0065_9710, fn_00659710(Ptr)),
        entry!(0x0065_9780, fn_00659780(u32) -> Ptr),
        entry!(0x0065_92c0, fn_006592c0(Ptr, u32) -> Ptr),
        entry!(0x0065_9310, fn_00659310(Ptr, Ptr)),
        entry!(0x0065_9370, fn_00659370(Ptr)),
        entry!(0x0065_93b0, fn_006593b0(Ptr, u32) -> Ptr),
        entry!(0x0065_9440, fn_00659440(Ptr)),
    ]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn creates_the_semaphore_from_the_two_words() {
        let mut e = Engine::new();
        e.register(CREATE_SEMAPHORE_A, |_, _| Ret {
            eax: 0xabc,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_9500, &args![this, 1u32, 5u32]).u32(), this);
        assert_eq!(calls_to(&mut e, CREATE_SEMAPHORE_A), vec![vec![0, 1, 5, 0]]);
        assert_eq!(e.mem.u32(this), 1);
        assert_eq!(e.mem.u32(this + 4), 5);
        assert_eq!(e.mem.u32(this + 8), 0xabc);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn deleting_destructor_chain_destroys_and_frees_with_bit_0() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[STRING_DESTRUCT, BASE_DESTRUCT_00449680, OPERATOR_DELETE],
        );
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_9380, &args![object, 1u32]).u32(), object);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, STRING_DESTRUCT), vec![vec![object + 0x20]]);
        assert_eq!(calls_to(&log, BASE_DESTRUCT_00449680), vec![vec![object]]);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![object]]);
        // 006592f0 and 00659360 destroy without freeing.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_92f0, &args![0u32, object]).u32(), object);
        assert_eq!(e.call(0x0065_9360, &args![object]).u32(), object);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, STRING_DESTRUCT).len(), 2);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn destructor_destroys_the_string_then_the_base() {
        let mut e = Engine::new();
        quiet(&mut e, &[STRING_DESTRUCT, BASE_DESTRUCT_00449680]);
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x0065_93e0, &args![object]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|(a, _)| *a).collect();
        assert_eq!(order, vec![STRING_DESTRUCT, BASE_DESTRUCT_00449680]);
    }

    fn map_engine() -> Engine {
        let mut e = Engine::new();
        e.register(ALLOCATE_00AA1070, |e, a| {
            e.mem.alloc(a[0].max(4)).into_ret()
        });
        e.register(MEMORY_FILL, |e, a| {
            let fill = vec![a[1] as u8; a[2] as usize];
            e.mem.write(a[0], &fill);
            Ret::default()
        });
        e
    }

    #[test]
    fn map_base_allocates_and_clears_the_table() {
        let mut e = map_engine();
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 0xc, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_9490, &args![map, 37u32]).u32(), map);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ALLOCATE_00AA1070), vec![vec![148]]);
        let table = e.mem.u32(map + 8);
        assert_eq!(calls_to(&log, MEMORY_FILL), vec![vec![table, 0, 148]]);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 37);
        assert_eq!(e.mem.u32(map + 0xc), 0);
    }

    #[test]
    fn map_sets_its_vtable() {
        let mut e = map_engine();
        let map = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0065_9460, &args![map, 5u32]).u32(), map);
        assert_eq!(e.mem.u32(map), MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 5);
    }

    #[test]
    fn string_template_map_keeps_the_copy_byte() {
        let mut e = map_engine();
        let map = e.mem.alloc(0x14);
        assert_eq!(e.call(0x0065_9330, &args![map, 5u32, 0x101u32]).u32(), map);
        assert_eq!(e.mem.u32(map), STRING_TEMPLATE_MAP_VTABLE);
        assert_eq!(e.mem.u8(map + 0x10), 1);
        assert_eq!(e.mem.u32(map + 4), 5);
    }

    #[test]
    fn close_handle_closes_the_third_word() {
        let mut e = Engine::new();
        e.register(CLOSE_HANDLE, |_, _| 1u32.into_ret());
        let object = e.mem.alloc(0xc);
        e.mem.set_u32(object + 8, 0x1234);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_9540, &args![object]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CLOSE_HANDLE), vec![vec![0x1234]]);
    }

    fn pair_engine() -> Engine {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                VECTOR_CONSTRUCTOR_ITERATOR,
                VECTOR_DESTRUCTOR_ITERATOR,
                STRING_FROM_C,
                STRING_DESTRUCT,
                READ_EGM_FILE,
                READ_EGT_FILE,
                EGM_ELEMENT_CLEAR,
                EGT_ELEMENT_CLEAR,
            ],
        );
        e
    }

    fn check_constructor(addr: u32, construct: u32, destruct: u32, read: u32) {
        let mut e = pair_engine();
        let object = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(addr, &args![object, 0x0100_0000u32]).u32(), object);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![object + 4, 0x18, 2, construct, destruct]]
        );
        let string = calls_to(&log, STRING_FROM_C)[0][0];
        assert_eq!(calls_to(&log, STRING_FROM_C)[0][1], 0x0100_0000);
        assert_eq!(
            calls_to(&log, read),
            vec![vec![string, object, object + 4, object + 0x1c]]
        );
        assert_eq!(calls_to(&log, STRING_DESTRUCT), vec![vec![string]]);
    }

    #[test]
    fn egm_pair_constructor_reads_the_file() {
        check_constructor(
            0x0065_9560,
            EGM_ELEMENT_CONSTRUCT,
            EGM_ELEMENT_DESTRUCT,
            READ_EGM_FILE,
        );
    }

    #[test]
    fn egt_pair_constructor_reads_the_file() {
        check_constructor(
            0x0065_9670,
            EGT_ELEMENT_CONSTRUCT,
            EGT_ELEMENT_DESTRUCT,
            READ_EGT_FILE,
        );
    }

    fn check_destructor(addr: u32, clear: u32, destruct: u32) {
        let mut e = pair_engine();
        let object = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        e.call(addr, &args![object]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, clear),
            vec![vec![object + 4], vec![object + 0x1c]]
        );
        assert_eq!(
            calls_to(&log, VECTOR_DESTRUCTOR_ITERATOR),
            vec![vec![object + 4, 0x18, 2, destruct]]
        );
    }

    #[test]
    fn egm_pair_destructor_clears_then_destroys() {
        check_destructor(0x0065_9600, EGM_ELEMENT_CLEAR, EGM_ELEMENT_DESTRUCT);
    }

    #[test]
    fn egt_pair_destructor_clears_then_destroys() {
        check_destructor(0x0065_9710, EGT_ELEMENT_CLEAR, EGT_ELEMENT_DESTRUCT);
    }

    #[test]
    fn egt_loader_allocates_and_constructs() {
        let mut e = pair_engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        let object = e.call(0x0065_9780, &args![0x0100_0000u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x34]]);
        assert_eq!(calls_to(&log, READ_EGT_FILE)[0][1], object);
        // A failed allocation constructs nothing.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_9780, &args![0x0100_0000u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, VECTOR_CONSTRUCTOR_ITERATOR).is_empty());
    }
}

#[cfg(test)]
mod n063_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn deleting_destructor_006592c0_frees_on_bit_0() {
        let mut e = Engine::new();
        e.register(DESTRUCT_00658D30, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        for (flags, freed) in [(1u32, 1usize), (2, 0)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0065_92c0, &args![0x5000u32, flags]).u32(), 0x5000);
            assert_eq!(calls(&e, DESTRUCT_00658D30), vec![vec![0x5000]]);
            assert_eq!(calls(&e, OPERATOR_DELETE).len(), freed);
        }
    }

    #[test]
    fn element_destructors_destroy_the_four_entries() {
        let mut e = Engine::new();
        e.register(VECTOR_DESTRUCTOR_ITERATOR, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        // The callback and its cdecl forwarder never free.
        e.call_log = Some(vec![]);
        e.call(0x0065_9310, &args![0x1234u32, 0x6000u32]);
        e.call(0x0065_9370, &args![0x7000u32]);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCTOR_ITERATOR),
            vec![
                vec![0x6008, 0x20, 4, BASE_DESTRUCT_00449680],
                vec![0x7008, 0x20, 4, BASE_DESTRUCT_00449680]
            ]
        );
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        // The deleting form frees with bit 0.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_93b0, &args![0x8000u32, 1u32]).u32(), 0x8000);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x8000]]);
        e.call_log = Some(vec![]);
        e.call(0x0065_9440, &args![0x9000u32]);
        assert_eq!(calls(&e, VECTOR_DESTRUCTOR_ITERATOR)[0][0], 0x9008);
    }
}
