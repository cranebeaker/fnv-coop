//! `Havok SDK/hkstringptr.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkStringPtr` is one word: a `const char*` whose bit 0 says the string is owned (a copy
//! made by `strDup`/`strNdup`, stored as `pointer + 1`). The shared `assign` (`00cb0b10`)
//! takes its operands in registers (`EAX` the length, `-1` for the whole string, and `ECX`
//! the new string) and the `hkStringPtr` as a stack word; `strDup` takes its string in
//! `EDI` and `strNdup` its length in `EAX` and its string in `EBX`. Those registers come
//! first among the parameters here (the doc of each function says which).

#[allow(unused_imports)]
use crate::prelude::*;

/// The word every empty `hkStringPtr` holds (a global at `01268358`, read when needed).
const EMPTY_STRING_WORD: u32 = 0x0126_8358;
/// `hkString::strLen` (`00cb10d0`, cdecl `(text)`), `strCpy` (`00cb1090`, cdecl
/// `(destination, source)`) and `strNcpy` (`00cb10b0`, cdecl `(destination, source, count)`).
const STR_LEN: u32 = 0x00cb_10d0;
const STR_CPY: u32 = 0x00cb_1090;
const STR_NCPY: u32 = 0x00cb_10b0;
/// The calling thread's memory router (`00c85750`); its word at `+0x10` is the allocator.
const GET_MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkMemoryRouter::easyAlloc` (`00c85690`, cdecl `(allocator, size)`) and `easyFree`
/// (`00c856c0`, cdecl `(allocator, block)`).
const EASY_ALLOC: u32 = 0x00c8_5690;
const EASY_FREE: u32 = 0x00c8_56c0;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkString::vsnprintf` (`00cb0f50`, cdecl `(destination, count, format, va_list)`).
const VSNPRINTF: u32 = 0x00cb_0f50;
/// The size bits of an `hkArray`'s capacity word; bit 31 means it does not own its storage.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// The inline array `printf` formats into: data pointer, size, capacity word, then 256 bytes.
const PRINTF_BUFFER_SIZE: u32 = 0x10c;
/// Its initial capacity word: 255 bytes of inline storage that must not be deallocated.
const PRINTF_CAPACITY_WORD: u32 = 0x8000_00ff;

layout! {
    /// `hkStringPtr` (Xbox PDB), one word.
    pub struct HkStringPtr: 0x4 {
        /// `m_stringAndFlag` (Xbox PDB): the string, plus 1 when the object owns a copy.
        0x00 m_stringAndFlag: u32,
    }
}

/// The calling thread's allocator, the word at router `+0x10`.
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

// Translated from 00cb0a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStringPtr::hkStringPtr` (Xbox PDB), thiscall: the word becomes the empty-string word
/// (the global at `01268358`). Returns `this`.
pub fn hk_string_ptr_hk_string_ptr(e: &mut Engine, this: Ptr<HkStringPtr>) -> Ptr<HkStringPtr> {
    let empty: u32 = e.global(EMPTY_STRING_WORD);
    e.set(this, HkStringPtr::m_stringAndFlag, empty);
    this
}

// Translated from 00cb0a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, thiscall with one stack word that is not read: returns `this`
/// and touches nothing.
pub fn fn_00cb0a70(_e: &mut Engine, this: Ptr, _unused_0: u32) -> Ptr {
    this
}

// Translated from 00cb0a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStringPtr::getLength` (Xbox PDB), thiscall: `strLen` of the string (the word without
/// bit 0), or 0 when that is null.
pub fn hk_string_ptr_get_length(e: &mut Engine, this: Ptr<HkStringPtr>) -> i32 {
    let text = e.get(this, HkStringPtr::m_stringAndFlag) & 0xffff_fffe;
    if text == 0 {
        return 0;
    }
    e.call(STR_LEN, &args![text]).i32()
}

// Translated from 00cb0aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `strDup` (engine map name; `hkStringPtr`'s helper): **the string is in `EDI`** (the first
/// and only parameter here; there are no stack words, and `EDI` is left as it was). Allocates
/// `strlen + 1` bytes from the calling thread's allocator (`easyAlloc(allocator, size)`) and
/// copies the string. Returns the block.
pub fn str_dup(e: &mut Engine, edi: Ptr) -> Ptr {
    let length = e.call(STR_LEN, &args![edi]).i32();
    let allocator = allocator(e);
    let block = e
        .call(EASY_ALLOC, &args![allocator, length.wrapping_add(1)])
        .ptr::<()>();
    e.call(STR_CPY, &args![block, edi]);
    block
}

// Translated from 00cb0ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `strNdup` (engine map name; `hkStringPtr`'s helper): **the limit is in `EAX` and the string
/// in `EBX`** (the two parameters here, in that order; there are no stack words). The length
/// is `strlen`, cut to the limit (a signed compare); allocates `length + 1` bytes from the
/// calling thread's allocator, copies `length` bytes with `strNcpy` and ends the copy with a
/// zero byte. Returns the block.
pub fn str_ndup(e: &mut Engine, eax: i32, ebx: Ptr) -> Ptr {
    let mut length = e.call(STR_LEN, &args![ebx]).i32();
    if length > eax {
        length = eax;
    }
    let allocator = allocator(e);
    let block = e
        .call(EASY_ALLOC, &args![allocator, length.wrapping_add(1)])
        .ptr::<()>();
    e.call(STR_NCPY, &args![block, ebx, length]);
    e.mem.set_u8(block.addr().wrapping_add(length as u32), 0);
    block
}

// Translated from 00cb0b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `assign` (engine map name; the shared body of the `hkStringPtr` setters): **the length is
/// in `EAX`, the new string in `ECX`**, and the stack word is the `hkStringPtr` (cdecl; the
/// caller removes it). Nothing happens when the new string is the one already held (the word
/// without bit 0). Otherwise an owned old copy is freed (`easyFree(allocator, word - 1)`);
/// then a null string stores the empty-string word, a length of -1 stores a `strDup` copy
/// `+ 1`, and any other length a `strNdup` copy `+ 1`.
pub fn assign(e: &mut Engine, eax: i32, ecx: Ptr, string_ptr: Ptr<HkStringPtr>) {
    let old = e.get(string_ptr, HkStringPtr::m_stringAndFlag);
    if old & 0xffff_fffe == ecx.addr() {
        return;
    }
    if old & 1 != 0 {
        let allocator = allocator(e);
        e.call(EASY_FREE, &args![allocator, old.wrapping_sub(1)]);
    }
    if ecx.is_null() {
        let empty: u32 = e.global(EMPTY_STRING_WORD);
        e.set(string_ptr, HkStringPtr::m_stringAndFlag, empty);
        return;
    }
    let copy = if eax == -1 {
        str_dup(e, ecx)
    } else {
        str_ndup(e, eax, ecx)
    };
    e.set(
        string_ptr,
        HkStringPtr::m_stringAndFlag,
        copy.addr().wrapping_add(1),
    );
}

// Translated from 00cb0b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStringPtr::hkStringPtr` from a C string (Xbox PDB, overload 2), thiscall, one stack
/// word `text`: the word starts as the empty-string word, then `assign(-1, text)`. Returns
/// `this`.
pub fn hk_string_ptr_hk_string_ptr_ov2(
    e: &mut Engine,
    this: Ptr<HkStringPtr>,
    text: Ptr,
) -> Ptr<HkStringPtr> {
    let empty: u32 = e.global(EMPTY_STRING_WORD);
    e.set(this, HkStringPtr::m_stringAndFlag, empty);
    assign(e, -1, text, this);
    this
}

// Translated from 00cb0bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStringPtr::hkStringPtr` copy constructor (Xbox PDB, overload 3), thiscall, one stack
/// word `other` (an `hkStringPtr`): the word starts as the empty-string word, then
/// `assign(-1, other's string)` (its word without bit 0). Returns `this`.
pub fn hk_string_ptr_hk_string_ptr_ov3(
    e: &mut Engine,
    this: Ptr<HkStringPtr>,
    other: Ptr<HkStringPtr>,
) -> Ptr<HkStringPtr> {
    let empty: u32 = e.global(EMPTY_STRING_WORD);
    e.set(this, HkStringPtr::m_stringAndFlag, empty);
    let text = e.get(other, HkStringPtr::m_stringAndFlag) & 0xffff_fffe;
    assign(e, -1, Ptr::new(text), this);
    this
}

// Translated from 00cb0c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStringPtr::~hkStringPtr` (Xbox PDB), thiscall: `assign(-1, null)` on `this` (frees an
/// owned copy and stores the empty-string word).
pub fn hk_string_ptr_dtor(e: &mut Engine, this: Ptr<HkStringPtr>) {
    assign(e, -1, Ptr::NULL, this);
}

// Translated from 00cb0c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStringPtr::operator=` from a C string (Xbox PDB), thiscall, one stack word `text`:
/// `assign(-1, text)`. Returns `this`.
pub fn hk_string_ptr_operator_assign(
    e: &mut Engine,
    this: Ptr<HkStringPtr>,
    text: Ptr,
) -> Ptr<HkStringPtr> {
    assign(e, -1, text, this);
    this
}

// Translated from 00cb0c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStringPtr::operator=` from another `hkStringPtr` (Xbox PDB, overload 2), thiscall, one
/// stack word `other`: `assign(-1, other's string)` (its word without bit 0). Returns
/// `this`.
pub fn hk_string_ptr_operator_assign_ov2(
    e: &mut Engine,
    this: Ptr<HkStringPtr>,
    other: Ptr<HkStringPtr>,
) -> Ptr<HkStringPtr> {
    let text = e.get(other, HkStringPtr::m_stringAndFlag) & 0xffff_fffe;
    assign(e, -1, Ptr::new(text), this);
    this
}

// Translated from 00cb0c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStringPtr::set` (Xbox PDB), thiscall, two stack words `(text, length)`:
/// `assign(length, text)`.
pub fn hk_string_ptr_set(e: &mut Engine, this: Ptr<HkStringPtr>, text: Ptr, length: i32) {
    assign(e, length, text, this);
}

// Translated from 00cb0c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStringPtr::printf` (Xbox PDB, cdecl, variable arguments; the `hkStringPtr` is the first
/// stack word, `format` the second): formats into a local inline array of 255 bytes with
/// `hkString::vsnprintf`. A negative result (the text did not fit) doubles the array's
/// capacity (`_reserve`); a result at least the capacity grows it to the larger of the
/// result and double the capacity when the capacity is below the result, and tries again;
/// otherwise `operator=(const char*)` (`00cb0c10`) stores the text, which the exe does
/// twice, and the array is released through the allocator when it owns its storage. The
/// variable argument words are copied into game memory as the `va_list`.
///
/// A result exactly equal to the capacity leaves the capacity as it is and repeats the same
/// call, as the exe does.
pub fn hk_string_ptr_printf(
    e: &mut Engine,
    this: Ptr<HkStringPtr>,
    format: Ptr,
    arguments: &[u32],
) {
    let va_list = e.mem.alloc((arguments.len().max(1) as u32) * 4);
    for (index, word) in arguments.iter().enumerate() {
        e.mem.set_u32(va_list + 4 * index as u32, *word);
    }
    e.with_stack(PRINTF_BUFFER_SIZE, |e, array| {
        let array_address = array.addr();
        // The array: data pointer, size, capacity word; the inline bytes follow.
        e.mem.set_u32(array_address, array_address + 0xc);
        e.mem.set_u32(array_address + 4, 0);
        e.mem.set_u32(array_address + 8, PRINTF_CAPACITY_WORD);
        loop {
            let capacity = e.mem.u32(array_address + 8) & CAPACITY_MASK;
            let data = e.mem.u32(array_address);
            let result = e
                .call(VSNPRINTF, &args![data, capacity, format, va_list])
                .i32();
            if result < 0 {
                let mut wanted = (e.mem.u32(array_address + 8) & CAPACITY_MASK) as i32;
                wanted = wanted.wrapping_add(wanted);
                let allocator = allocator(e);
                let current = (e.mem.u32(array_address + 8) & CAPACITY_MASK) as i32;
                if current < wanted {
                    let doubled = current.wrapping_add(current);
                    if wanted < doubled {
                        wanted = doubled;
                    }
                    e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 1i32]);
                }
                continue;
            }
            if result >= capacity as i32 {
                let mut wanted = result;
                let allocator = allocator(e);
                let current = (e.mem.u32(array_address + 8) & CAPACITY_MASK) as i32;
                if current < wanted {
                    let doubled = current.wrapping_add(current);
                    if wanted < doubled {
                        wanted = doubled;
                    }
                    e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 1i32]);
                }
                continue;
            }
            break;
        }
        let data = e.mem.u32(array_address);
        hk_string_ptr_operator_assign(e, this, Ptr::new(data));
        let data = e.mem.u32(array_address);
        hk_string_ptr_operator_assign(e, this, Ptr::new(data));
        let allocator = allocator(e);
        let capacity_word = e.mem.u32(array_address + 8);
        e.mem.set_u32(array_address + 4, 0);
        if (capacity_word as i32) >= 0 {
            let data = e.mem.u32(array_address);
            e.vcall(allocator, 0x10, &args![data, capacity_word & CAPACITY_MASK]);
        }
    });
    e.mem.free(va_list);
}

/// The registered form of `hkStringPtr::printf`: two fixed words, then the variable
/// arguments, however many the caller pushed.
fn hk_string_ptr_printf_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        words.len() >= 2,
        "hkStringPtr::printf needs 2 argument words, got {}",
        words.len()
    );
    hk_string_ptr_printf(e, Ptr::new(words[0]), Ptr::new(words[1]), &words[2..]);
    Ret::default()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cb_0a60,
            hk_string_ptr_hk_string_ptr(Ptr<HkStringPtr>) -> Ptr<HkStringPtr>
        ),
        entry!(0x00cb_0a70, fn_00cb0a70(Ptr, u32) -> Ptr),
        entry!(
            0x00cb_0a80,
            hk_string_ptr_get_length(Ptr<HkStringPtr>) -> i32
        ),
        entry!(0x00cb_0aa0, str_dup(Ptr) -> Ptr),
        entry!(0x00cb_0ad0, str_ndup(i32, Ptr) -> Ptr),
        entry!(0x00cb_0b10, assign(i32, Ptr, Ptr<HkStringPtr>)),
        entry!(
            0x00cb_0b80,
            hk_string_ptr_hk_string_ptr_ov2(Ptr<HkStringPtr>, Ptr) -> Ptr<HkStringPtr>
        ),
        entry!(
            0x00cb_0bd0,
            hk_string_ptr_hk_string_ptr_ov3(Ptr<HkStringPtr>, Ptr<HkStringPtr>) -> Ptr<HkStringPtr>
        ),
        entry!(0x00cb_0c00, hk_string_ptr_dtor(Ptr<HkStringPtr>)),
        entry!(
            0x00cb_0c10,
            hk_string_ptr_operator_assign(Ptr<HkStringPtr>, Ptr) -> Ptr<HkStringPtr>
        ),
        entry!(
            0x00cb_0c30,
            hk_string_ptr_operator_assign_ov2(
                Ptr<HkStringPtr>,
                Ptr<HkStringPtr>,
            ) -> Ptr<HkStringPtr>
        ),
        entry!(0x00cb_0c50, hk_string_ptr_set(Ptr<HkStringPtr>, Ptr, i32)),
        (0x00cb_0c70, hk_string_ptr_printf_entry as AbiFn),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const EMPTY: u32 = 0x00ab_0010;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(EMPTY_STRING_WORD, EMPTY);
        e
    }

    fn put_string(e: &mut Engine, text: &[u8]) -> u32 {
        let address = e.mem.alloc(text.len() as u32 + 8);
        for (i, byte) in text.iter().enumerate() {
            e.mem.set_u8(address + i as u32, *byte);
        }
        e.mem.set_u8(address + text.len() as u32, 0);
        address
    }

    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Doubles for what the copy helpers call: the thread router (`router + 0x10` is the
    /// allocator `0xa110c`), `strLen` (counts for real), `easyAlloc` (answers `block`),
    /// `strCpy`/`strNcpy` (copy for real) and `easyFree`. Returns the block.
    fn install_copy_doubles(e: &mut Engine) -> u32 {
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0x10, 0xa110c);
        answer(e, GET_MEMORY_ROUTER, router);
        let block = e.mem.alloc(0x40);
        answer(e, EASY_ALLOC, block);
        answer(e, EASY_FREE, 0);
        e.register(STR_LEN, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        e.register(STR_CPY, |e, a| {
            let text = e.mem.cstr(a[1]);
            e.mem.write(a[0], &text);
            e.mem.set_u8(a[0] + text.len() as u32, 0);
            Ret::default()
        });
        e.register(STR_NCPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        block
    }

    #[test]
    fn constructor_stores_the_empty_string_word() {
        let mut e = engine();
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x1234);
        assert_eq!(e.call(0x00cb_0a60, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), EMPTY);
    }

    #[test]
    fn fn_00cb0a70_returns_this_and_changes_nothing() {
        let mut e = engine();
        assert_eq!(
            e.call(0x00cb_0a70, &args![0xdead_0000u32, 5u32]).u32(),
            0xdead_0000
        );
    }

    #[test]
    fn get_length_measures_the_string_without_the_owned_bit() {
        let mut e = engine();
        e.register(STR_LEN, |_, a| Ret {
            eax: a[0] ^ 0x100,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x5001);
        assert_eq!(e.call(0x00cb_0a80, &args![this]).u32(), 0x5100);
        assert_eq!(calls_to(&e, STR_LEN), vec![vec![0x5000]]);
        // A null string, with or without the bit, is 0 and calls nothing.
        e.call_log = Some(vec![]);
        for word in [0u32, 1] {
            e.mem.set_u32(this, word);
            assert_eq!(e.call(0x00cb_0a80, &args![this]).u32(), 0);
        }
        assert!(calls_to(&e, STR_LEN).is_empty());
    }

    #[test]
    fn str_dup_copies_with_the_calling_threads_allocator() {
        let mut e = engine();
        let block = install_copy_doubles(&mut e);
        e.call_log = Some(vec![]);
        let text = put_string(&mut e, b"hello");
        assert_eq!(e.call(0x00cb_0aa0, &args![text]).u32(), block);
        assert_eq!(calls_to(&e, EASY_ALLOC), vec![vec![0xa110c, 6]]);
        assert_eq!(e.mem.cstr(block), b"hello".to_vec());
    }

    #[test]
    fn str_ndup_cuts_the_copy_to_the_limit_and_terminates_it() {
        let mut e = engine();
        let block = install_copy_doubles(&mut e);
        for i in 0..8 {
            e.mem.set_u8(block + i, 0xee);
        }
        e.call_log = Some(vec![]);
        let text = put_string(&mut e, b"hello");
        assert_eq!(e.call(0x00cb_0ad0, &args![3i32, text]).u32(), block);
        assert_eq!(calls_to(&e, EASY_ALLOC), vec![vec![0xa110c, 4]]);
        assert_eq!(e.mem.bytes(block, 5), vec![b'h', b'e', b'l', 0, 0xee]);
        // A limit above the length copies it all.
        e.call_log = Some(vec![]);
        e.call(0x00cb_0ad0, &args![99i32, text]);
        assert_eq!(calls_to(&e, EASY_ALLOC), vec![vec![0xa110c, 6]]);
        assert_eq!(e.mem.cstr(block), b"hello".to_vec());
    }

    #[test]
    fn assign_does_nothing_for_the_string_already_held() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(4);
        // The held word carries the owned bit; the string compared is without it.
        e.mem.set_u32(this, 0x5001);
        e.call(0x00cb_0b10, &args![-1i32, 0x5000u32, this]);
        assert_eq!(e.mem.u32(this), 0x5001);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn assign_frees_an_owned_copy_and_stores_the_empty_word_for_null() {
        let mut e = engine();
        install_copy_doubles(&mut e);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x5001);
        e.call(0x00cb_0b10, &args![-1i32, 0u32, this]);
        assert_eq!(calls_to(&e, EASY_FREE), vec![vec![0xa110c, 0x5000]]);
        assert_eq!(e.mem.u32(this), EMPTY);
        // An unowned word is not freed.
        e.call_log = Some(vec![]);
        e.mem.set_u32(this, 0x6000);
        e.call(0x00cb_0b10, &args![-1i32, 0u32, this]);
        assert!(calls_to(&e, EASY_FREE).is_empty());
        assert_eq!(e.mem.u32(this), EMPTY);
    }

    #[test]
    fn assign_stores_a_whole_copy_for_minus_one_and_a_bounded_one_otherwise() {
        let mut e = engine();
        let block = install_copy_doubles(&mut e);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, EMPTY);
        let text = put_string(&mut e, b"hello");
        e.call(0x00cb_0b10, &args![-1i32, text, this]);
        assert_eq!(e.mem.u32(this), block + 1);
        assert_eq!(e.mem.cstr(block), b"hello".to_vec());
        // A length: the bounded copy; the old copy is freed first.
        e.call_log = Some(vec![]);
        e.call(0x00cb_0b10, &args![2i32, text, this]);
        assert_eq!(calls_to(&e, EASY_FREE), vec![vec![0xa110c, block]]);
        assert_eq!(e.mem.u32(this), block + 1);
        assert_eq!(e.mem.cstr(block), b"he".to_vec());
    }

    #[test]
    fn the_constructors_and_setters_all_assign() {
        let mut e = engine();
        let block = install_copy_doubles(&mut e);
        let text = put_string(&mut e, b"abc");
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x7777);
        // From a C string: starts empty, then copies (the old word 0x7777 is not owned).
        assert_eq!(e.call(0x00cb_0b80, &args![this, text]).u32(), this);
        assert_eq!(e.mem.u32(this), block + 1);
        // Copy constructor: from another hkStringPtr's string (its word without the bit).
        let copy = e.mem.alloc(4);
        e.mem.set_u32(copy, 0x7777);
        assert_eq!(e.call(0x00cb_0bd0, &args![copy, this]).u32(), copy);
        assert_eq!(e.mem.u32(copy), block + 1);
        assert_eq!(e.mem.cstr(block), b"abc".to_vec());
        // operator= (C string): returns this.
        let other_text = put_string(&mut e, b"xyz");
        assert_eq!(e.call(0x00cb_0c10, &args![copy, other_text]).u32(), copy);
        assert_eq!(e.mem.cstr(block), b"xyz".to_vec());
        // operator= (hkStringPtr).
        let third = e.mem.alloc(4);
        e.mem.set_u32(third, 0x7777);
        assert_eq!(e.call(0x00cb_0c30, &args![third, copy]).u32(), third);
        assert_eq!(e.mem.u32(third), block + 1);
        // set with a length.
        e.call(0x00cb_0c50, &args![third, text, 1i32]);
        assert_eq!(e.mem.cstr(block), b"a".to_vec());
        // The destructor releases and resets.
        e.call_log = Some(vec![]);
        e.call(0x00cb_0c00, &args![third]);
        assert_eq!(calls_to(&e, EASY_FREE), vec![vec![0xa110c, block]]);
        assert_eq!(e.mem.u32(third), EMPTY);
    }

    /// A `vsnprintf` double: answers the scripted results in turn and, for one that fits,
    /// writes `text` at the destination. Logs `(destination, count, va_list words)`.
    type Formats = Rc<RefCell<Vec<(u32, u32, Vec<u32>)>>>;

    fn install_formatter(e: &mut Engine, results: Vec<i32>, text: &'static [u8]) -> Formats {
        let log: Formats = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        let mut results = results.into_iter();
        e.register_double(VSNPRINTF, move |e, a| {
            let words = vec![e.mem.u32(a[3]), e.mem.u32(a[3] + 4)];
            seen.borrow_mut().push((a[0], a[1], words));
            let result = results.next().expect("more vsnprintf calls than scripted");
            if result >= 0 && (result as u32) < a[1] {
                e.mem.write(a[0], text);
                e.mem.set_u8(a[0] + text.len() as u32, 0);
            }
            Ret {
                eax: result as u32,
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn printf_formats_into_the_inline_array_and_stores_the_text() {
        let mut e = engine();
        let block = install_copy_doubles(&mut e);
        let log = install_formatter(&mut e, vec![5], b"hello");
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, EMPTY);
        let format = put_string(&mut e, b"%s");
        e.call_log = Some(vec![]);
        e.call(0x00cb_0c70, &[this, format, 11, 22]);
        assert_eq!(log.borrow().len(), 1);
        // The inline array holds 255 bytes; the va_list reads the caller's two words.
        assert_eq!(log.borrow()[0].1, 255);
        assert_eq!(log.borrow()[0].2, vec![11, 22]);
        assert_eq!(e.mem.u32(this), block + 1);
        assert_eq!(e.mem.cstr(block), b"hello".to_vec());
        // The inline storage is not deallocated, and no reserve was needed.
        assert!(calls_to(&e, ARRAY_RESERVE).is_empty());
    }

    #[test]
    fn printf_grows_the_array_after_a_negative_or_too_large_result() {
        let mut e = engine();
        let block = install_copy_doubles(&mut e);
        // -1: double the capacity (510); 600 is at least the capacity: grow to the larger of 600
        // and double the capacity (1020); then the text fits.
        let log = install_formatter(&mut e, vec![-1, 600, 5], b"hello");
        // The reserve double enlarges the array: capacity word and a new data block.
        let grown = e.mem.alloc(0x400);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            e.mem.set_u32(a[1], grown);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let freed = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&freed);
        let allocator_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(allocator_vtable + 0x10, 0x0900_0030);
        let allocator_object = e.mem.alloc(8);
        e.mem.set_u32(allocator_object, allocator_vtable);
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0x10, allocator_object);
        answer(&mut e, GET_MEMORY_ROUTER, router);
        e.register_double(0x0900_0030, move |_, a| {
            seen.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, EMPTY);
        let format = put_string(&mut e, b"%s");
        e.call_log = Some(vec![]);
        e.call(0x00cb_0c70, &[this, format]);
        let counts: Vec<u32> = log.borrow().iter().map(|c| c.1).collect();
        // 255, then 510 after -1, then the capacity the reserve gave for 1020.
        assert_eq!(counts, vec![255, 510, 1020]);
        let reserves = calls_to(&e, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 2);
        assert_eq!(reserves[0][2..], [510, 1]);
        assert_eq!(reserves[1][2..], [1020, 1]);
        assert_eq!(reserves[0][0], allocator_object);
        assert_eq!(e.mem.u32(this), block + 1);
        // The grown array owned its storage: it is freed with its capacity as the size.
        assert_eq!(*freed.borrow(), vec![(grown, 1020)]);
    }
}
