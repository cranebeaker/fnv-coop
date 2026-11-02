//! `Havok SDK/hkstringbuf.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkStringBuf` is an `hkInplaceArray<char, 128, hkContainerTempAllocator>` (Xbox PDB, 0x8c
//! bytes): the array's data pointer, element count (the text plus its terminating zero) and
//! capacity word (the size in the low 30 bits, bit 31 "does not own its storage"), then 128
//! inline bytes at `+0xc`. All growth goes through `hkArrayUtil::_reserve` (`00c908c0`, cdecl
//! `(allocator, array, count, element size)`) with the thread router's temporary allocator,
//! the word at router `+0xc` (the heap allocator at `+0x10` is the one the `hkStringPtr`
//! code uses).

#[allow(unused_imports)]
use crate::prelude::*;

/// The calling thread's memory router (`00c85750`); its word at `+0xc` is the allocator of
/// every function here.
const GET_MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkString::strLen` (`00cb10d0`, cdecl `(text)`) and `memCpy` (`00cb1260`, cdecl
/// `(destination, source, size)`).
const STR_LEN: u32 = 0x00cb_10d0;
const MEM_CPY: u32 = 0x00cb_1260;
/// `hkString::vsnprintf` (`00cb0f50`, cdecl `(destination, count, format, va_list)`).
const VSNPRINTF: u32 = 0x00cb_0f50;
/// The splice behind `00d62630` (`00d625a0`, thiscall on the array, five stack words
/// `(allocator, index, removed count, text, length)`).
const ARRAY_SPLICE: u32 = 0x00d6_25a0;
/// The size bits of the capacity word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// The capacity word of an array that starts in its 128 inline bytes.
const INPLACE_CAPACITY_WORD: u32 = 0x8000_0080;

layout! {
    /// `hkStringBuf` (Xbox PDB), 0x8c bytes: its one member `m_string` is an inplace array
    /// of 128 characters. The fields are the array's (`hkArrayBase` names, Xbox PDB).
    pub struct HkStringBuf: 0x8c {
        /// `m_data` (Xbox PDB `hkArrayBase`).
        0x00 m_data: Ptr,
        /// `m_size` (Xbox PDB `hkArrayBase`): the characters plus the terminating zero.
        0x04 m_size: i32,
        /// `m_capacityAndFlags` (Xbox PDB `hkArrayBase`).
        0x08 m_capacityAndFlags: u32,
    }
}

/// The calling thread's temporary allocator, the word at router `+0xc`.
fn temp_allocator(e: &mut Engine) -> u32 {
    let router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0xc)
}

/// `array.m_capacityAndFlags & 0x3fffffff`.
fn capacity(e: &Engine, this: Ptr<HkStringBuf>) -> i32 {
    (e.get(this, HkStringBuf::m_capacityAndFlags) & CAPACITY_MASK) as i32
}

/// The code the functions here share to make room for `wanted` bytes when the capacity is
/// below it: the new capacity is the larger of `wanted` and twice the capacity (compared
/// signed, `wanted` winning ties), reserved through `_reserve(allocator, array, count, 1)`.
fn reserve_for(e: &mut Engine, allocator: u32, this: Ptr<HkStringBuf>, wanted: i32) {
    let current = capacity(e, this);
    if current < wanted {
        let doubled = current.wrapping_add(current);
        let count = if wanted < doubled { doubled } else { wanted };
        e.call(ARRAY_RESERVE, &args![allocator, this, count, 1i32]);
    }
}

/// Stores `text` with its terminator: size `length + 1`, the zero at `data[length]` (written
/// before the bytes are copied, as the exe does), then `memCpy(data, text, length)`.
fn store_text(e: &mut Engine, this: Ptr<HkStringBuf>, text: Ptr, length: i32) {
    let wanted = length.wrapping_add(1);
    let allocator = temp_allocator(e);
    reserve_for(e, allocator, this, wanted);
    let data = e.get(this, HkStringBuf::m_data);
    e.set(this, HkStringBuf::m_size, wanted);
    e.mem.set_u8(data.addr().wrapping_add(length as u32), 0);
    let data = e.get(this, HkStringBuf::m_data);
    e.call(MEM_CPY, &args![data, text, length]);
}

/// Makes the buffer the empty string: when the capacity is below 1 (zero), the capacity
/// becomes `max(2 * capacity, 1)` signed (so 1); then size 1 and a zero first byte.
fn store_empty(e: &mut Engine, this: Ptr<HkStringBuf>) {
    let allocator = temp_allocator(e);
    let current = capacity(e, this);
    if current < 1 {
        let doubled = current.wrapping_add(current);
        let count = if doubled > 1 { doubled } else { 1 };
        e.call(ARRAY_RESERVE, &args![allocator, this, count, 1i32]);
    }
    let data = e.get(this, HkStringBuf::m_data);
    e.set(this, HkStringBuf::m_size, 1);
    e.mem.set_u8(data.addr(), 0);
}

/// The inplace array's starting state: the data pointer at the inline bytes (`this +
/// 0xc`), no elements, 128 inline characters that must not be deallocated.
fn init_inplace(e: &mut Engine, this: Ptr<HkStringBuf>) {
    e.set(this, HkStringBuf::m_data, Ptr::new(this.addr() + 0xc));
    e.set(this, HkStringBuf::m_size, 0);
    e.set(this, HkStringBuf::m_capacityAndFlags, INPLACE_CAPACITY_WORD);
}

// Translated from 00d61a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStringBuf::hkStringBuf` from a C string (Xbox PDB), thiscall, one stack word `text`:
/// an inplace array holding a copy of the text and its terminator (`strLen`, room for
/// `length + 1`, the zero stored, then `memCpy`), or just the terminator for a null `text`.
/// Returns `this`.
pub fn hk_string_buf_hk_string_buf(
    e: &mut Engine,
    this: Ptr<HkStringBuf>,
    text: Ptr,
) -> Ptr<HkStringBuf> {
    init_inplace(e, this);
    if text.is_null() {
        store_empty(e, this);
        return this;
    }
    let length = e.call(STR_LEN, &args![text]).i32();
    store_text(e, this, text, length);
    this
}

// Translated from 00d61b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the copy constructor), thiscall, one stack word `other` (an
/// `hkStringBuf`): an inplace array; when `other` holds more elements than the 128 inline
/// ones, storage for them comes from the temporary allocator's slot `0x0c` (`allocate(&size)`,
/// the size is in/out: the granted size becomes the capacity word) after freeing the
/// current storage through slot `0x10` when the array owns it (dead here: the inline flag is
/// set, so the exe never frees). Then the size is copied and the bytes copied forward one
/// by one. Returns `this`.
pub fn fn_00d61b80(
    e: &mut Engine,
    this: Ptr<HkStringBuf>,
    other: Ptr<HkStringBuf>,
) -> Ptr<HkStringBuf> {
    init_inplace(e, this);
    let allocator = temp_allocator(e);
    let word = e.get(this, HkStringBuf::m_capacityAndFlags);
    let other_size = e.get(other, HkStringBuf::m_size);
    if ((word & CAPACITY_MASK) as i32) < other_size {
        if (word as i32) >= 0 {
            // Unreachable in the exe: the capacity word was just set with bit 31.
            let data = e.get(this, HkStringBuf::m_data);
            e.vcall(allocator, 0x10, &args![data, word & CAPACITY_MASK]);
        }
        let granted = e.with_stack(4, |e, request| {
            e.mem.set_i32(request.addr(), other_size);
            let block = e.vcall(allocator, 0xc, &args![request]).ptr::<()>();
            e.set(this, HkStringBuf::m_data, block);
            e.mem.u32(request.addr())
        });
        e.set(this, HkStringBuf::m_capacityAndFlags, granted);
    }
    let count = e.get(other, HkStringBuf::m_size);
    let data = e.get(this, HkStringBuf::m_data);
    e.set(this, HkStringBuf::m_size, count);
    let source = e.get(other, HkStringBuf::m_data);
    if count > 0 {
        for index in 0..count as u32 {
            let byte = e.mem.u8(source.addr().wrapping_add(index));
            e.mem.set_u8(data.addr().wrapping_add(index), byte);
        }
    }
    this
}

// Translated from 00d61c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStringBuf::operator=` from a C string (Xbox PDB), thiscall, one stack word `text`:
/// stores the text and its terminator as the constructor does (growing the array when
/// needed), or just the terminator for a null `text`. Returns `this`.
pub fn hk_string_buf_operator_assign(
    e: &mut Engine,
    this: Ptr<HkStringBuf>,
    text: Ptr,
) -> Ptr<HkStringBuf> {
    if text.is_null() {
        store_empty(e, this);
        return this;
    }
    let length = e.call(STR_LEN, &args![text]).i32();
    store_text(e, this, text, length);
    this
}

// Translated from 00d61cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (makes the buffer the empty string), thiscall: room for one
/// byte when the capacity is zero, size 1 and a zero first byte.
pub fn fn_00d61cc0(e: &mut Engine, this: Ptr<HkStringBuf>) {
    store_empty(e, this);
}

/// Growth for `printf`: the capacity becomes at least `needed`, as in [`reserve_for`] (the
/// larger of `needed` and twice the capacity), then the size is `needed` and the byte at
/// `terminator` is zero.
fn grow_and_terminate(e: &mut Engine, this: Ptr<HkStringBuf>, needed: i32, terminator: i32) {
    let allocator = temp_allocator(e);
    reserve_for(e, allocator, this, needed);
    let data = e.get(this, HkStringBuf::m_data);
    e.set(this, HkStringBuf::m_size, needed);
    e.mem.set_u8(data.addr().wrapping_add(terminator as u32), 0);
}

// Translated from 00d61d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`hkStringBuf::printf`; cdecl, variable arguments: the buffer
/// is the first stack word, the format the second): formats into the array with
/// `hkString::vsnprintf(data, capacity, format, va_list)`. A negative result (it did not fit)
/// grows the array to `max(2 * capacity, 255) + 1` bytes, with the terminator stored at
/// the index one less; a result at least the capacity grows it to `result + 1`; both formats
/// again. A result that fits sets the size to `result + 1` and stores the terminator at
/// `data[result]`. The variable argument words are copied into game memory as the
/// `va_list`.
pub fn fn_00d61d00(e: &mut Engine, this: Ptr<HkStringBuf>, format: Ptr, arguments: &[u32]) {
    let va_list = e.mem.alloc((arguments.len().max(1) as u32) * 4);
    for (index, word) in arguments.iter().enumerate() {
        e.mem.set_u32(va_list + 4 * index as u32, *word);
    }
    loop {
        let capacity = capacity(e, this);
        let data = e.get(this, HkStringBuf::m_data);
        let result = e
            .call(VSNPRINTF, &args![data, capacity, format, va_list])
            .i32();
        if result < 0 {
            let doubled = capacity.wrapping_add(capacity);
            let index = if doubled > 0xff { doubled } else { 0xff };
            grow_and_terminate(e, this, index.wrapping_add(1), index);
            continue;
        }
        if result >= capacity {
            grow_and_terminate(e, this, result.wrapping_add(1), result);
            continue;
        }
        grow_and_terminate(e, this, result.wrapping_add(1), result);
        break;
    }
    e.mem.free(va_list);
}

/// The registered form of `00d61d00`: two fixed words, then the variable arguments, however
/// many the caller pushed.
fn fn_00d61d00_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        words.len() >= 2,
        "00d61d00 needs 2 argument words, got {}",
        words.len()
    );
    fn_00d61d00(e, Ptr::new(words[0]), Ptr::new(words[1]), &words[2..]);
    Ret::default()
}

// Translated from 00d61df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (appends a C string), thiscall, one stack word `text`:
/// `00d62630(allocator, size - 1, text, strLen(text))` with the temporary allocator.
/// Returns `this`.
pub fn fn_00d61df0(e: &mut Engine, this: Ptr<HkStringBuf>, text: Ptr) -> Ptr<HkStringBuf> {
    let length = e.call(STR_LEN, &args![text]).i32();
    let size = e.get(this, HkStringBuf::m_size);
    let allocator = temp_allocator(e);
    fn_00d62630(e, this, allocator, size.wrapping_sub(1), text, length);
    this
}

// Translated from 00d61e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStringBuf::appendJoin` (Xbox PDB), thiscall, six stack words: up to six C strings (the
/// list ends at the first null one; the exe keeps the six words in a local array followed
/// by a null). Their lengths are summed; the array grows to hold the old text, all the
/// strings and the terminator (the larger of the new size and twice the capacity when the
/// capacity is below it), the size becomes the new size, the terminator is stored at the
/// end and each string is copied behind the old text with `memCpy`.
pub fn hk_string_buf_append_join(e: &mut Engine, this: Ptr<HkStringBuf>, strings: [u32; 6]) {
    // The local array: the six words, then the null that ends the list.
    let mut list = [0u32; 7];
    list[..6].copy_from_slice(&strings);
    let mut lengths = [0i32; 6];
    let mut end = e.get(this, HkStringBuf::m_size).wrapping_sub(1);
    let base = end;
    let mut count = 0usize;
    if list[0] != 0 {
        loop {
            let length = e.call(STR_LEN, &args![list[count]]).i32();
            lengths[count] = length;
            end = end.wrapping_add(length);
            count += 1;
            if list[count] == 0 {
                break;
            }
        }
    }
    let new_size = end.wrapping_add(1);
    let allocator = temp_allocator(e);
    reserve_for(e, allocator, this, new_size);
    let data = e.get(this, HkStringBuf::m_data);
    e.set(this, HkStringBuf::m_size, new_size);
    e.mem.set_u8(data.addr().wrapping_add(end as u32), 0);
    if list[0] != 0 {
        let mut offset = base;
        let mut index = 0usize;
        loop {
            let length = lengths[index];
            let data = e.get(this, HkStringBuf::m_data);
            e.call(
                MEM_CPY,
                &args![data.addr().wrapping_add(offset as u32), list[index], length],
            );
            index += 1;
            offset = offset.wrapping_add(length);
            if list[index] == 0 {
                break;
            }
        }
    }
}

// Translated from 00d62630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (an `hkArray<char>` insert; thiscall, four stack words
/// `(allocator, index, text, length)`): the splice `00d625a0` with nothing removed,
/// `00d625a0(this, allocator, index, 0, text, length)`.
pub fn fn_00d62630(
    e: &mut Engine,
    this: Ptr<HkStringBuf>,
    allocator: u32,
    index: i32,
    text: Ptr,
    length: i32,
) {
    e.call(
        ARRAY_SPLICE,
        &args![this, allocator, index, 0u32, text, length],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d6_1a50,
            hk_string_buf_hk_string_buf(Ptr<HkStringBuf>, Ptr) -> Ptr<HkStringBuf>
        ),
        entry!(
            0x00d6_1b80,
            fn_00d61b80(Ptr<HkStringBuf>, Ptr<HkStringBuf>) -> Ptr<HkStringBuf>
        ),
        entry!(
            0x00d6_1c10,
            hk_string_buf_operator_assign(Ptr<HkStringBuf>, Ptr) -> Ptr<HkStringBuf>
        ),
        entry!(0x00d6_1cc0, fn_00d61cc0(Ptr<HkStringBuf>)),
        (0x00d6_1d00, fn_00d61d00_entry as AbiFn),
        entry!(
            0x00d6_1df0,
            fn_00d61df0(Ptr<HkStringBuf>, Ptr) -> Ptr<HkStringBuf>
        ),
        (0x00d6_1e30, hk_string_buf_append_join_entry as AbiFn),
        entry!(
            0x00d6_2630,
            fn_00d62630(Ptr<HkStringBuf>, u32, i32, Ptr, i32)
        ),
    ]
}

/// The registered form of `appendJoin`: `this` and the six string words.
fn hk_string_buf_append_join_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        words.len() == 7,
        "hkStringBuf::appendJoin needs 7 argument words, got {}",
        words.len()
    );
    let mut strings = [0u32; 6];
    strings.copy_from_slice(&words[1..7]);
    hk_string_buf_append_join(e, Ptr::new(words[0]), strings);
    Ret::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The thread router's allocator word at `+0xc`.
    const TEMP_ALLOCATOR: u32 = 0x00a1_10c0;

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

    fn put_string(e: &mut Engine, text: &[u8]) -> u32 {
        let address = e.mem.alloc(text.len() as u32 + 8);
        e.mem.write(address, text);
        e.mem.set_u8(address + text.len() as u32, 0);
        address
    }

    /// Doubles for the callees: the router (allocator at `+0xc`), `strLen` and `memCpy` (real
    /// behaviour), and `_reserve`, which really moves the array into a larger block (capacity
    /// word = the requested count, no flags).
    fn install(e: &mut Engine) {
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0xc, TEMP_ALLOCATOR);
        answer(e, GET_MEMORY_ROUTER, router);
        e.register(STR_LEN, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        e.register(MEM_CPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(ARRAY_RESERVE, |e, a| {
            let (array, count) = (a[1], a[2]);
            let block = e.mem.alloc(count);
            let size = e.mem.u32(array + 4);
            let old = e.mem.u32(array);
            for i in 0..size.min(count) {
                let byte = e.mem.u8(old + i);
                e.mem.set_u8(block + i, byte);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, count);
            Ret::default()
        });
    }

    fn buffer(e: &mut Engine) -> u32 {
        e.mem.alloc(0x8c)
    }

    fn text_of(e: &Engine, this: u32) -> Vec<u8> {
        let data = e.mem.u32(this);
        let size = e.mem.u32(this + 4);
        e.mem.bytes(data, size)
    }

    #[test]
    fn constructor_copies_the_text_and_terminator_inline() {
        let mut e = Engine::new();
        install(&mut e);
        e.call_log = Some(vec![]);
        let this = buffer(&mut e);
        let text = put_string(&mut e, b"hello");
        assert_eq!(e.call(0x00d6_1a50, &args![this, text]).u32(), this);
        assert_eq!(e.mem.u32(this), this + 0xc);
        assert_eq!(e.mem.u32(this + 8), 0x8000_0080);
        assert_eq!(text_of(&e, this), b"hello\0".to_vec());
        assert!(calls_to(&e, ARRAY_RESERVE).is_empty());
    }

    #[test]
    fn constructor_reserves_the_larger_of_the_size_and_double_the_capacity() {
        let mut e = Engine::new();
        install(&mut e);
        e.call_log = Some(vec![]);
        let this = buffer(&mut e);
        // 200 characters: 201 bytes against twice the capacity, 256.
        let long = vec![b'x'; 200];
        let text = put_string(&mut e, &long);
        e.call(0x00d6_1a50, &args![this, text]);
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE),
            vec![vec![TEMP_ALLOCATOR, this, 256, 1]]
        );
        assert_eq!(e.mem.i32(this + 4), 201);
        assert_eq!(e.mem.u8(e.mem.u32(this) + 200), 0);
        assert_eq!(e.mem.bytes(e.mem.u32(this), 200), long);
        // 300 characters: 301 bytes is above 256, so it wins.
        e.call_log = Some(vec![]);
        let that = buffer(&mut e);
        let longer = vec![b'y'; 300];
        let text = put_string(&mut e, &longer);
        e.call(0x00d6_1a50, &args![that, text]);
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE),
            vec![vec![TEMP_ALLOCATOR, that, 301, 1]]
        );
    }

    #[test]
    fn constructor_with_a_null_text_holds_just_the_terminator() {
        let mut e = Engine::new();
        install(&mut e);
        e.call_log = Some(vec![]);
        let this = buffer(&mut e);
        e.call(0x00d6_1a50, &args![this, 0u32]);
        assert_eq!(e.mem.i32(this + 4), 1);
        assert_eq!(text_of(&e, this), vec![0]);
        assert!(calls_to(&e, ARRAY_RESERVE).is_empty());
    }

    #[test]
    fn copy_constructor_copies_a_small_buffer_into_the_inline_storage() {
        let mut e = Engine::new();
        install(&mut e);
        e.call_log = Some(vec![]);
        let other = buffer(&mut e);
        let text = put_string(&mut e, b"abcd");
        e.call(0x00d6_1a50, &args![other, text]);
        let this = buffer(&mut e);
        assert_eq!(e.call(0x00d6_1b80, &args![this, other]).u32(), this);
        assert_eq!(e.mem.u32(this), this + 0xc);
        assert_eq!(text_of(&e, this), b"abcd\0".to_vec());
        assert_eq!(e.mem.u32(this + 8), 0x8000_0080);
    }

    #[test]
    fn copy_constructor_allocates_a_block_for_a_large_buffer() {
        let mut e = Engine::new();
        install(&mut e);
        // The temporary allocator: slot 0xc (allocate, in/out size) grants 1024 bytes.
        let allocator_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(allocator_vtable + 0xc, 0x0900_0040);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, allocator_vtable);
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0xc, allocator);
        answer(&mut e, GET_MEMORY_ROUTER, router);
        let block = e.mem.alloc(0x400);
        let requests = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&requests);
        e.register_double(0x0900_0040, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[1]));
            e.mem.set_u32(a[1], 1024);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        // `other` is a heap buffer of 200 bytes.
        let other = buffer(&mut e);
        let data = e.mem.alloc(200);
        for i in 0..200 {
            e.mem.set_u8(data + i, (i % 251) as u8);
        }
        e.mem.set_u32(other, data);
        e.mem.set_u32(other + 4, 200);
        let this = buffer(&mut e);
        assert_eq!(e.call(0x00d6_1b80, &args![this, other]).u32(), this);
        assert_eq!(*requests.borrow(), vec![200]);
        assert_eq!(e.mem.u32(this), block);
        assert_eq!(e.mem.u32(this + 8), 1024);
        assert_eq!(e.mem.i32(this + 4), 200);
        assert_eq!(e.mem.bytes(block, 200), e.mem.bytes(data, 200));
    }

    #[test]
    fn operator_assign_replaces_the_text_and_null_empties_it() {
        let mut e = Engine::new();
        install(&mut e);
        let this = buffer(&mut e);
        let first = put_string(&mut e, b"hello world");
        e.call(0x00d6_1a50, &args![this, first]);
        let second = put_string(&mut e, b"hi");
        assert_eq!(e.call(0x00d6_1c10, &args![this, second]).u32(), this);
        assert_eq!(e.mem.i32(this + 4), 3);
        assert_eq!(e.mem.cstr(e.mem.u32(this)), b"hi".to_vec());
        e.call(0x00d6_1c10, &args![this, 0u32]);
        assert_eq!(e.mem.i32(this + 4), 1);
        assert_eq!(e.mem.u8(e.mem.u32(this)), 0);
        // A text longer than the capacity grows the array.
        e.call_log = Some(vec![]);
        let long = vec![b'z'; 150];
        let text = put_string(&mut e, &long);
        e.call(0x00d6_1c10, &args![this, text]);
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE),
            vec![vec![TEMP_ALLOCATOR, this, 256, 1]]
        );
        assert_eq!(e.mem.i32(this + 4), 151);
    }

    #[test]
    fn fn_00d61cc0_reserves_one_byte_only_for_a_zero_capacity() {
        let mut e = Engine::new();
        install(&mut e);
        e.call_log = Some(vec![]);
        let this = buffer(&mut e);
        // Zero capacity, no storage.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00d6_1cc0, &args![this]);
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE),
            vec![vec![TEMP_ALLOCATOR, this, 1, 1]]
        );
        assert_eq!(e.mem.i32(this + 4), 1);
        assert_eq!(e.mem.u8(e.mem.u32(this)), 0);
        // With storage already there nothing is reserved.
        e.call_log = Some(vec![]);
        let data = e.mem.alloc(4);
        e.mem.set_u8(data, 0x55);
        e.mem.set_u32(this, data);
        e.mem.set_u32(this + 8, 4);
        e.mem.set_u32(this + 4, 3);
        e.call(0x00d6_1cc0, &args![this]);
        assert!(calls_to(&e, ARRAY_RESERVE).is_empty());
        assert_eq!(e.mem.i32(this + 4), 1);
        assert_eq!(e.mem.u8(data), 0);
    }

    type Formats = Rc<RefCell<Vec<(u32, u32, Vec<u32>)>>>;

    /// A `vsnprintf` double: answers the scripted results in turn; for one that fits it writes
    /// `text` at the destination. Logs `(destination, count, va_list words)`.
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
            }
            Ret {
                eax: result as u32,
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn printf_sets_the_size_to_the_result_plus_one_and_terminates() {
        let mut e = Engine::new();
        install(&mut e);
        let log = install_formatter(&mut e, vec![5], b"hello");
        let this = buffer(&mut e);
        e.call(0x00d6_1a50, &args![this, 0u32]);
        let format = put_string(&mut e, b"%s");
        e.call(0x00d6_1d00, &[this, format, 11, 22]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(log.borrow()[0].1, 128);
        assert_eq!(log.borrow()[0].2, vec![11, 22]);
        assert_eq!(e.mem.i32(this + 4), 6);
        assert_eq!(text_of(&e, this), b"hello\0".to_vec());
    }

    #[test]
    fn printf_grows_after_a_negative_result_and_after_a_result_that_does_not_fit() {
        let mut e = Engine::new();
        install(&mut e);
        let log = install_formatter(&mut e, vec![-1, 300, 5], b"hello");
        let this = buffer(&mut e);
        e.call(0x00d6_1a50, &args![this, 0u32]);
        let format = put_string(&mut e, b"%s");
        e.call_log = Some(vec![]);
        e.call(0x00d6_1d00, &[this, format]);
        // -1 at capacity 128: max(2 * 128, 255) = 256 index, 257 bytes (reserved: max(257,
        // 256) = 257). Then 300 does not fit in 257: 301 bytes; the new capacity is
        // max(301, 514) = 514. Then 5 fits.
        let counts: Vec<u32> = log.borrow().iter().map(|c| c.1).collect();
        assert_eq!(counts, vec![128, 257, 514]);
        let reserves = calls_to(&e, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 2);
        assert_eq!(reserves[0][2..], [257, 1]);
        assert_eq!(reserves[1][2..], [514, 1]);
        assert_eq!(e.mem.i32(this + 4), 6);
        assert_eq!(e.mem.bytes(e.mem.u32(this), 6), b"hello\0".to_vec());
    }

    #[test]
    fn printf_with_a_negative_result_at_a_large_capacity_doubles_it() {
        let mut e = Engine::new();
        install(&mut e);
        install_formatter(&mut e, vec![-1, 0], b"");
        let this = buffer(&mut e);
        e.call(0x00d6_1a50, &args![this, 0u32]);
        // A heap block of capacity 400.
        let data = e.mem.alloc(400);
        e.mem.set_u32(this, data);
        e.mem.set_u32(this + 8, 400);
        let format = put_string(&mut e, b"x");
        e.call_log = Some(vec![]);
        e.call(0x00d6_1d00, &[this, format]);
        // 2 * 400 = 800 is above 255: index 800, 801 bytes.
        assert_eq!(calls_to(&e, ARRAY_RESERVE)[0][2..], [801, 1]);
    }

    #[test]
    fn fn_00d61df0_appends_through_the_splice() {
        let mut e = Engine::new();
        install(&mut e);
        e.register_double(ARRAY_SPLICE, |_, _| Ret::default());
        let this = buffer(&mut e);
        let first = put_string(&mut e, b"ab");
        e.call(0x00d6_1a50, &args![this, first]);
        e.call_log = Some(vec![]);
        let text = put_string(&mut e, b"cde");
        assert_eq!(e.call(0x00d6_1df0, &args![this, text]).u32(), this);
        // (this, allocator, size - 1, removed count, text, length)
        assert_eq!(
            calls_to(&e, ARRAY_SPLICE),
            vec![vec![this, TEMP_ALLOCATOR, 2, 0, text, 3]]
        );
    }

    #[test]
    fn fn_00d62630_inserts_with_nothing_removed() {
        let mut e = Engine::new();
        e.register_double(ARRAY_SPLICE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00d6_2630, &args![0x10u32, 0x20u32, 3i32, 0x40u32, 7i32]);
        assert_eq!(
            calls_to(&e, ARRAY_SPLICE),
            vec![vec![0x10, 0x20, 3, 0, 0x40, 7]]
        );
    }

    #[test]
    fn append_join_appends_every_string_up_to_the_first_null() {
        let mut e = Engine::new();
        install(&mut e);
        let this = buffer(&mut e);
        let start = put_string(&mut e, b"xy");
        e.call(0x00d6_1a50, &args![this, start]);
        let a = put_string(&mut e, b"ab");
        let b = put_string(&mut e, b"cde");
        let c = put_string(&mut e, b"f");
        let ignored = put_string(&mut e, b"ignored");
        e.call(0x00d6_1e30, &[this, a, b, c, 0, ignored, 0]);
        assert_eq!(e.mem.i32(this + 4), 9);
        assert_eq!(text_of(&e, this), b"xyabcdef\0".to_vec());
    }

    #[test]
    fn append_join_with_six_strings_and_with_none() {
        let mut e = Engine::new();
        install(&mut e);
        let this = buffer(&mut e);
        let start = put_string(&mut e, b"-");
        e.call(0x00d6_1a50, &args![this, start]);
        let parts: Vec<u32> = [b"1", b"2", b"3", b"4", b"5", b"6"]
            .iter()
            .map(|p| put_string(&mut e, *p))
            .collect();
        let mut words = vec![this];
        words.extend(parts);
        e.call(0x00d6_1e30, &words);
        assert_eq!(text_of(&e, this), b"-123456\0".to_vec());
        // No strings: the size and the terminator are rewritten as they were.
        let before = text_of(&e, this);
        e.call(0x00d6_1e30, &[this, 0, 0, 0, 0, 0, 0]);
        assert_eq!(text_of(&e, this), before);
    }

    #[test]
    fn append_join_grows_the_array_for_the_total_length() {
        let mut e = Engine::new();
        install(&mut e);
        let this = buffer(&mut e);
        let start = put_string(&mut e, b"a");
        e.call(0x00d6_1a50, &args![this, start]);
        let big = vec![b'q'; 200];
        let text = put_string(&mut e, &big);
        e.call_log = Some(vec![]);
        e.call(0x00d6_1e30, &[this, text, 0, 0, 0, 0, 0]);
        // 1 + 200 + 1 = 202 bytes against twice the capacity, 256.
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE),
            vec![vec![TEMP_ALLOCATOR, this, 256, 1]]
        );
        assert_eq!(e.mem.i32(this + 4), 202);
    }
}
