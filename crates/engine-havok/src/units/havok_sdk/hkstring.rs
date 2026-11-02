//! `Havok SDK/hkstring.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkString::memCpy`, `memMove` and `memSet` are each a single `JMP` to the C runtime
//! (`memcpy` `00ec44d0`, `memmove` `00ec7230`, `memset` `00ec61c0`), so they forward their
//! arguments and return the runtime's result.

#[allow(unused_imports)]
use crate::prelude::*;

/// The C runtime's `memcpy`, `memmove` and `memset`.
const CRT_MEMCPY: u32 = 0x00ec_44d0;
const CRT_MEMMOVE: u32 = 0x00ec_7230;
const CRT_MEMSET: u32 = 0x00ec_61c0;

// Translated from 00cb10d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::strLen` (Xbox PDB, cdecl): the number of bytes before the terminating zero.
pub fn hk_string_str_len(e: &mut Engine, string: Ptr) -> i32 {
    let mut length = 0u32;
    while e.mem.u8(string.addr().wrapping_add(length)) != 0 {
        length = length.wrapping_add(1);
    }
    length as i32
}

// Translated from 00cb1260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::memCpy` (Xbox PDB, cdecl): a jump to the runtime's `memcpy`
/// `(destination, source, size)`; returns its result.
pub fn hk_string_mem_cpy(e: &mut Engine, destination: Ptr, source: Ptr, size: u32) -> Ptr {
    e.call(CRT_MEMCPY, &args![destination, source, size]).ptr()
}

// Translated from 00cb1270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::memMove` (Xbox PDB, cdecl): a jump to the runtime's `memmove`
/// `(destination, source, size)`; returns its result.
pub fn hk_string_mem_move(e: &mut Engine, destination: Ptr, source: Ptr, size: u32) -> Ptr {
    e.call(CRT_MEMMOVE, &args![destination, source, size]).ptr()
}

// Translated from 00cb1280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::memSet` (Xbox PDB, cdecl): a jump to the runtime's `memset`
/// `(destination, value, size)`; returns its result.
pub fn hk_string_mem_set(e: &mut Engine, destination: Ptr, value: i32, size: u32) -> Ptr {
    e.call(CRT_MEMSET, &args![destination, value, size]).ptr()
}

/// `__cdecl` character mapper `strCasecmp` applies to each byte (`00cb0f30`:
/// one word in, the mapped byte in AL).
const MAP_CHARACTER: u32 = 0x00cb_0f30;
/// `__cdecl` copy of a string with an allocator (`00cb1160`: string, allocator).
const COPY_STRING: u32 = 0x00cb_1160;
/// `hkMemoryRouter::hkMemoryRouter` (`00c85770`, `__thiscall`, no stack arguments).
const MEMORY_ROUTER_CONSTRUCTOR: u32 = 0x00c8_5770;
/// The calling thread's memory router (`00c85750`).
const GET_MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The CRT `strrchr` (`00ec6e30`, `__cdecl`).
const CRT_STRRCHR: u32 = 0x00ec_6e30;
/// Size of the `hkMemoryRouter` that `strDup` keeps on its stack.
const MEMORY_ROUTER_SIZE: u32 = 0x24;

// Translated from 00cb0fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::strCasecmp` (Xbox PDB): compares two strings byte by byte
/// through the mapper `00cb0f30`, as signed bytes; -1, 0 or 1. Returns 0
/// when both strings end at the same position.
pub fn hk_string_str_casecmp(e: &mut Engine, left: u32, right: u32) -> i32 {
    let mut index = 0;
    loop {
        let left_byte = e.mem.u8(left + index);
        if left_byte == 0 && e.mem.u8(right + index) == 0 {
            return 0;
        }
        let mapped_left = e.call(MAP_CHARACTER, &args![left_byte as u32]).u32() as u8 as i8;
        let right_byte = e.mem.u8(right + index);
        let mapped_right = e.call(MAP_CHARACTER, &args![right_byte as u32]).u32() as u8 as i8;
        if mapped_left < mapped_right {
            return -1;
        }
        if mapped_left > mapped_right {
            return 1;
        }
        index += 1;
    }
}

// Translated from 00cb1150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::strRchr` (Xbox PDB): a jump to the CRT `strrchr`
/// (`00ec6e30`) with the same two arguments; returns its result.
pub fn hk_string_str_rchr(e: &mut Engine, text: u32, character: u32) -> u32 {
    e.call(CRT_STRRCHR, &args![text, character]).u32()
}

// Translated from 00cb1410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::strDup` (Xbox PDB): constructs a temporary `hkMemoryRouter`
/// on the stack (`00c85770`), takes the calling thread's router (`00c85750`)
/// and copies the string with the word at router `+0x10` (`00cb1160`);
/// returns the copy.
pub fn hk_string_str_dup(e: &mut Engine, text: u32) -> u32 {
    e.with_stack(MEMORY_ROUTER_SIZE, |e, router| {
        e.call(MEMORY_ROUTER_CONSTRUCTOR, &args![router]);
        let thread_router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
        // hkMemoryRouter +0x10: the allocator word `strDup` hands to the copy.
        let allocator = e.mem.u32(thread_router + 0x10);
        e.call(COPY_STRING, &args![text, allocator]).u32()
    })
}

/// The CRT's `_vsnprintf` (`00ecddf1`, cdecl `(destination, count, format, va_list)`),
/// `strncmp` (`00ec8a19`, cdecl `(left, right, count)`), `strncpy` (`00ecc7b0`, cdecl
/// `(destination, source, count)`), `strtoul` (`00ec8d33`, cdecl `(text, end, base)`),
/// `strstr` (`00ec7750`, cdecl `(text, pattern)`) and `strchr` (`00ec7690`, cdecl
/// `(text, character)`).
const CRT_VSNPRINTF: u32 = 0x00ec_ddf1;
const CRT_STRNCMP: u32 = 0x00ec_8a19;
const CRT_STRNCPY: u32 = 0x00ec_c7b0;
const CRT_STRTOUL: u32 = 0x00ec_8d33;
const CRT_STRSTR: u32 = 0x00ec_7750;
const CRT_STRCHR: u32 = 0x00ec_7690;
/// `hkMemoryRouter::easyAlloc` (`00c85690`, cdecl `(allocator, size)`) and `easyFree`
/// (`00c856c0`, cdecl `(allocator, block)`).
const EASY_ALLOC: u32 = 0x00c8_5690;
const EASY_FREE: u32 = 0x00c8_56c0;

// Translated from 00cb0f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the character mapper `strCasecmp` uses; cdecl, one stack word
/// of which only the low byte is read. An upper-case ASCII letter (`'A'..='Z'`, tested as
/// `byte - 0x41 <= 0x19` unsigned) becomes lower case (`+ 0x20`); every other byte is
/// unchanged. The result is in AL.
pub fn fn_00cb0f30(_e: &mut Engine, character: u32) -> u8 {
    let byte = character as u8;
    if byte.wrapping_sub(0x41) <= 0x19 {
        byte.wrapping_add(0x20)
    } else {
        byte
    }
}

// Translated from 00cb0f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::vsnprintf` (Xbox PDB, cdecl `(destination, count, format, va_list)`): a jump
/// to the runtime's `_vsnprintf` with the same four words; returns its result.
pub fn hk_string_vsnprintf(
    e: &mut Engine,
    destination: Ptr,
    count: u32,
    format: Ptr,
    arguments: u32,
) -> i32 {
    e.call(CRT_VSNPRINTF, &args![destination, count, format, arguments])
        .i32()
}

// Translated from 00cb0f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::snprintf` (Xbox PDB, cdecl, variable arguments): passes the address of its
/// first variable argument (on the caller's stack in the exe; here the words are copied into
/// game memory, as the guide says for a `va_list`) to the runtime's `_vsnprintf` with
/// `(destination, count, format)`. Returns its result.
pub fn hk_string_snprintf(
    e: &mut Engine,
    destination: Ptr,
    count: u32,
    format: Ptr,
    arguments: &[u32],
) -> i32 {
    let block = e.mem.alloc((arguments.len().max(1) as u32) * 4);
    for (index, word) in arguments.iter().enumerate() {
        e.mem.set_u32(block + 4 * index as u32, *word);
    }
    let result = hk_string_vsnprintf(e, destination, count, format, block);
    e.mem.free(block);
    result
}

/// The registered form of `hkString::snprintf`: three fixed words, then the variable
/// arguments, however many the caller pushed.
fn hk_string_snprintf_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        words.len() >= 3,
        "hkString::snprintf needs 3 argument words, got {}",
        words.len()
    );
    let result = hk_string_snprintf(
        e,
        Ptr::new(words[0]),
        words[1],
        Ptr::new(words[2]),
        &words[3..],
    );
    result.into_ret()
}

// Translated from 00cb0fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::strCmp` (Xbox PDB, cdecl): compares the two strings two bytes per step as
/// unsigned bytes and answers -1 or 1 at the first difference (the code's `SBB` pair turns
/// the borrow of the byte compare into -1, otherwise 1), 0 when they end together.
pub fn hk_string_str_cmp(e: &mut Engine, left: Ptr, right: Ptr) -> i32 {
    let mut a = left.addr();
    let mut b = right.addr();
    loop {
        let first = e.mem.u8(a);
        let other = e.mem.u8(b);
        if first != other {
            return if first < other { -1 } else { 1 };
        }
        if first == 0 {
            return 0;
        }
        let first = e.mem.u8(a.wrapping_add(1));
        let other = e.mem.u8(b.wrapping_add(1));
        if first != other {
            return if first < other { -1 } else { 1 };
        }
        a = a.wrapping_add(2);
        b = b.wrapping_add(2);
        if first == 0 {
            return 0;
        }
    }
}

// Translated from 00cb0fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl `(left, right, count)`): a jump to the runtime's
/// `strncmp` with the same three words; returns its result.
pub fn fn_00cb0fd0(e: &mut Engine, left: Ptr, right: Ptr, count: u32) -> i32 {
    e.call(CRT_STRNCMP, &args![left, right, count]).i32()
}

// Translated from 00cb1090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::strCpy` (Xbox PDB, cdecl `(destination, source)`): copies the source string
/// including its terminating zero, byte by byte.
pub fn hk_string_str_cpy(e: &mut Engine, destination: Ptr, source: Ptr) {
    let mut from = source.addr();
    let mut to = destination.addr();
    loop {
        let byte = e.mem.u8(from);
        e.mem.set_u8(to, byte);
        from = from.wrapping_add(1);
        to = to.wrapping_add(1);
        if byte == 0 {
            break;
        }
    }
}

// Translated from 00cb10b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::strNcpy` (Xbox PDB, cdecl `(destination, source, count)`): nothing for a zero
/// count (the zero count is what the function leaves in EAX); otherwise a jump to the
/// runtime's `strncpy` with the same three words, whose result it returns.
pub fn hk_string_str_ncpy(e: &mut Engine, destination: Ptr, source: Ptr, count: u32) -> Ptr {
    if count == 0 {
        return Ptr::NULL;
    }
    e.call(CRT_STRNCPY, &args![destination, source, count])
        .ptr()
}

// Translated from 00cb10f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::atoi` (Xbox PDB, cdecl `(text, base)`): the runtime's `strtoul(text, 0,
/// base)`; returns its result.
pub fn hk_string_atoi(e: &mut Engine, text: Ptr, base: i32) -> i32 {
    e.call(CRT_STRTOUL, &args![text, 0u32, base]).i32()
}

// Translated from 00cb1130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::strStr` (Xbox PDB, cdecl `(text, pattern)`): a jump to the runtime's `strstr`
/// with the same two words; returns its result.
pub fn hk_string_str_str(e: &mut Engine, text: Ptr, pattern: Ptr) -> Ptr {
    e.call(CRT_STRSTR, &args![text, pattern]).ptr()
}

// Translated from 00cb1140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::strChr` (Xbox PDB, cdecl `(text, character)`): a jump to the runtime's
/// `strchr` with the same two words; returns its result.
pub fn hk_string_str_chr(e: &mut Engine, text: Ptr, character: i32) -> Ptr {
    e.call(CRT_STRCHR, &args![text, character]).ptr()
}

// Translated from 00cb1160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (string copy into a router allocation), cdecl `(text,
/// allocator)`: allocates `strlen + 1` bytes with `hkMemoryRouter::easyAlloc(allocator,
/// size)` (`00c85690`) and copies the string with `strCpy`. Returns the new block.
pub fn fn_00cb1160(e: &mut Engine, text: Ptr, allocator: u32) -> Ptr {
    let length = hk_string_str_len(e, text);
    let block = e
        .call(EASY_ALLOC, &args![allocator, length.wrapping_add(1)])
        .ptr::<()>();
    hk_string_str_cpy(e, block, text);
    block
}

// Translated from 00cb1190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (frees a router allocation), cdecl `(block, allocator)`:
/// `hkMemoryRouter::easyFree(allocator, block)` (`00c856c0`).
pub fn fn_00cb1190(e: &mut Engine, block: Ptr, allocator: u32) {
    e.call(EASY_FREE, &args![allocator, block]);
}

// Translated from 00cb11b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (bounded string copy into a router allocation), cdecl `(text,
/// limit, allocator)`: the length is `strlen`, cut to `limit` (a signed compare); allocates
/// `length + 1` bytes with `easyAlloc(allocator, size)`, copies `length` bytes with
/// `strNcpy` and ends the copy with a zero byte. Returns the new block.
pub fn fn_00cb11b0(e: &mut Engine, text: Ptr, limit: i32, allocator: u32) -> Ptr {
    let mut length = hk_string_str_len(e, text);
    if length > limit {
        length = limit;
    }
    let block = e
        .call(EASY_ALLOC, &args![allocator, length.wrapping_add(1)])
        .ptr::<()>();
    hk_string_str_ncpy(e, block, text, length as u32);
    e.mem.set_u8(block.addr().wrapping_add(length as u32), 0);
    block
}

// Translated from 00cb1290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::memCmp` (Xbox PDB, cdecl `(left, right, count)`): compares four bytes per
/// step while at least four remain, then byte by byte (the first four bytes of a differing
/// word: a zero difference in the fourth byte is not tested and gives 1, as in the code).
/// Returns -1 or 1 at the first difference (the sign of the byte difference, `(d >> 31) |
/// 1`), 0 when equal.
pub fn hk_string_mem_cmp(e: &mut Engine, left: Ptr, right: Ptr, count: u32) -> i32 {
    let mut a = left.addr();
    let mut b = right.addr();
    let mut remaining = count;
    let mut differs = false;
    while remaining >= 4 {
        if e.mem.u32(a) != e.mem.u32(b) {
            differs = true;
            break;
        }
        remaining -= 4;
        b = b.wrapping_add(4);
        a = a.wrapping_add(4);
    }
    if !differs && remaining == 0 {
        return 0;
    }
    let mut difference = 0i32;
    for index in 0..4u32 {
        if index > 0 && remaining <= index {
            return 0;
        }
        difference =
            i32::from(e.mem.u8(a.wrapping_add(index))) - i32::from(e.mem.u8(b.wrapping_add(index)));
        // The fourth byte falls straight into the sign extraction.
        if difference != 0 || index == 3 {
            break;
        }
    }
    (difference >> 31) | 1
}

// Translated from 00cb13c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::strFree` (Xbox PDB, cdecl `(block)`): frees the block through the calling
/// thread's router: `00cb1190(block, router + 0x10)`.
pub fn hk_string_str_free(e: &mut Engine, block: Ptr) {
    let router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
    // hkMemoryRouter +0x10: the allocator word.
    let allocator = e.mem.u32(router + 0x10);
    fn_00cb1190(e, block, allocator);
}

// Translated from 00cb13e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the bounded duplicate of `strDup`'s kind), cdecl `(text,
/// limit)`: constructs a temporary `hkMemoryRouter` on the stack (`00c85770`), takes the
/// calling thread's router (`00c85750`) and calls `00cb11b0(text, limit, router + 0x10)`;
/// returns the copy.
pub fn fn_00cb13e0(e: &mut Engine, text: Ptr, limit: i32) -> Ptr {
    e.with_stack(MEMORY_ROUTER_SIZE, |e, router| {
        e.call(MEMORY_ROUTER_CONSTRUCTOR, &args![router]);
        let thread_router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
        // hkMemoryRouter +0x10: the allocator word.
        let allocator = e.mem.u32(thread_router + 0x10);
        fn_00cb11b0(e, text, limit, allocator)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb_10d0, hk_string_str_len(Ptr) -> i32),
        entry!(0x00cb_1260, hk_string_mem_cpy(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00cb_1270, hk_string_mem_move(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00cb_1280, hk_string_mem_set(Ptr, i32, u32) -> Ptr),
        entry!(0x00cb0fe0, hk_string_str_casecmp(u32, u32) -> i32),
        entry!(0x00cb1150, hk_string_str_rchr(u32, u32) -> u32),
        entry!(0x00cb1410, hk_string_str_dup(u32) -> u32),
        entry!(0x00cb_0f30, fn_00cb0f30(u32) -> u8),
        entry!(0x00cb_0f50, hk_string_vsnprintf(Ptr, u32, Ptr, u32) -> i32),
        (0x00cb_0f60, hk_string_snprintf_entry as AbiFn),
        entry!(0x00cb_0fa0, hk_string_str_cmp(Ptr, Ptr) -> i32),
        entry!(0x00cb_0fd0, fn_00cb0fd0(Ptr, Ptr, u32) -> i32),
        entry!(0x00cb_1090, hk_string_str_cpy(Ptr, Ptr)),
        entry!(0x00cb_10b0, hk_string_str_ncpy(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00cb_10f0, hk_string_atoi(Ptr, i32) -> i32),
        entry!(0x00cb_1130, hk_string_str_str(Ptr, Ptr) -> Ptr),
        entry!(0x00cb_1140, hk_string_str_chr(Ptr, i32) -> Ptr),
        entry!(0x00cb_1160, fn_00cb1160(Ptr, u32) -> Ptr),
        entry!(0x00cb_1190, fn_00cb1190(Ptr, u32)),
        entry!(0x00cb_11b0, fn_00cb11b0(Ptr, i32, u32) -> Ptr),
        entry!(0x00cb_1290, hk_string_mem_cmp(Ptr, Ptr, u32) -> i32),
        entry!(0x00cb_13c0, hk_string_str_free(Ptr)),
        entry!(0x00cb_13e0, fn_00cb13e0(Ptr, i32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_len_counts_bytes_before_the_zero() {
        let mut e = Engine::new();
        let text = e.mem.alloc(16);
        e.mem.set_cstr(text, b"hello");
        assert_eq!(e.call(0x00cb_10d0, &args![text]).i32(), 5);
        e.mem.set_u8(text, 0);
        assert_eq!(e.call(0x00cb_10d0, &args![text]).i32(), 0);
    }

    /// Each wrapper passes its three words on unchanged and returns the runtime's result.
    fn forwards(wrapper: u32, runtime: u32) {
        let mut e = Engine::new();
        e.register(runtime, |_, a| Ret {
            eax: a[0] ^ a[1] ^ a[2],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let got = e.call(wrapper, &args![0x1000u32, 0x20u32, 0x300u32]).u32();
        assert_eq!(got, 0x1000 ^ 0x20 ^ 0x300);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == runtime).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x1000, 0x20, 0x300]);
    }

    #[test]
    fn mem_functions_forward_to_the_runtime() {
        forwards(0x00cb_1260, CRT_MEMCPY);
        forwards(0x00cb_1270, CRT_MEMMOVE);
        forwards(0x00cb_1280, CRT_MEMSET);
    }
}

#[cfg(test)]
mod tests_b {
    use super::*;

    fn put_string(e: &mut Engine, text: &[u8]) -> u32 {
        let address = e.mem.alloc(text.len() as u32 + 1);
        for (i, byte) in text.iter().enumerate() {
            e.mem.set_u8(address + i as u32, *byte);
        }
        address
    }

    #[test]
    fn str_casecmp_orders_by_mapped_bytes() {
        let mut e = Engine::new();
        e.register(MAP_CHARACTER, |_, a| Ret {
            eax: (a[0] as u8).to_ascii_lowercase() as u32,
            ..Ret::default()
        });
        let hello = put_string(&mut e, b"Hello");
        let shout = put_string(&mut e, b"HELLO");
        let longer = put_string(&mut e, b"HELLOS");
        let bigger = put_string(&mut e, b"hellp");
        assert_eq!(e.call(0x00cb_0fe0, &args![hello, shout]).i32(), 0);
        assert_eq!(e.call(0x00cb_0fe0, &args![hello, longer]).i32(), -1);
        assert_eq!(e.call(0x00cb_0fe0, &args![longer, hello]).i32(), 1);
        assert_eq!(e.call(0x00cb_0fe0, &args![bigger, hello]).i32(), 1);
        assert_eq!(e.call(0x00cb_0fe0, &args![hello, bigger]).i32(), -1);
    }

    #[test]
    fn str_casecmp_compares_mapped_bytes_as_signed() {
        let mut e = Engine::new();
        // 'a' maps to 0x80 (signed -128), anything else to 1.
        e.register(MAP_CHARACTER, |_, a| Ret {
            eax: if a[0] == b'a' as u32 { 0x80 } else { 1 },
            ..Ret::default()
        });
        let a = put_string(&mut e, b"a");
        let b = put_string(&mut e, b"b");
        assert_eq!(e.call(0x00cb_0fe0, &args![a, b]).i32(), -1);
    }

    #[test]
    fn str_rchr_forwards_to_the_crt() {
        let mut e = Engine::new();
        e.register(CRT_STRRCHR, |_, a| Ret {
            eax: a[0] + a[1],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00cb_1150, &args![0x1000u32, 0x2fu32]).u32(),
            0x102f
        );
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(CRT_STRRCHR, vec![0x1000, 0x2f])));
    }

    #[test]
    fn str_dup_copies_with_the_thread_routers_word() {
        let mut e = Engine::new();
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0x10, 0xa110c);
        e.register(MEMORY_ROUTER_CONSTRUCTOR, |_, _| Ret::default());
        e.register_double(GET_MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(COPY_STRING, |_, a| Ret {
            eax: a[0] ^ a[1],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00cb_1410, &args![0x5000u32]).u32(),
            0x5000 ^ 0xa110c
        );
        let log = e.call_log.clone().unwrap();
        assert!(log.iter().any(|c| c.0 == MEMORY_ROUTER_CONSTRUCTOR));
        assert!(log.contains(&(COPY_STRING, vec![0x5000, 0xa110c])));
    }
}

#[cfg(test)]
mod tests_c {
    use super::*;

    fn put_string(e: &mut Engine, text: &[u8]) -> u32 {
        let address = e.mem.alloc(text.len() as u32 + 8);
        for (i, byte) in text.iter().enumerate() {
            e.mem.set_u8(address + i as u32, *byte);
        }
        e.mem.set_u8(address + text.len() as u32, 0);
        address
    }

    /// Makes `address` answer `value` in EAX.
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

    #[test]
    fn character_mapper_lowers_ascii_capitals_only() {
        let mut e = Engine::new();
        for (input, expected) in [
            (b'A' as u32, b'a'),
            (b'M' as u32, b'm'),
            (b'Z' as u32, b'z'),
            (0x40, 0x40),
            (0x5b, 0x5b),
            (b'a' as u32, b'a'),
            (0xc1, 0xc1),
            (0, 0),
            // Only the low byte of the stack word is read.
            (0x1234_5641, b'a'),
        ] {
            assert_eq!(
                e.call(0x00cb_0f30, &args![input]).u8(),
                expected,
                "{input:x}"
            );
        }
    }

    #[test]
    fn vsnprintf_forwards_its_four_words() {
        let mut e = Engine::new();
        e.register(CRT_VSNPRINTF, |_, a| Ret {
            eax: a[0] + a[1] + a[2] + a[3],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let got = e
            .call(0x00cb_0f50, &args![0x1000u32, 0x20u32, 0x300u32, 0x4u32])
            .u32();
        assert_eq!(got, 0x1324);
        assert_eq!(
            calls_to(&e, CRT_VSNPRINTF),
            vec![vec![0x1000, 0x20, 0x300, 4]]
        );
    }

    #[test]
    fn snprintf_passes_the_variable_arguments_as_a_va_list_in_memory() {
        let mut e = Engine::new();
        e.register_double(CRT_VSNPRINTF, |e, a| {
            // The va_list points at the caller's variable words.
            Ret {
                eax: e.mem.u32(a[3]) * 100 + e.mem.u32(a[3] + 4),
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        let words = [0x1000u32, 0x20, 0x300, 7, 9];
        let got = e.call(0x00cb_0f60, &words).u32();
        assert_eq!(got, 709);
        let log = calls_to(&e, CRT_VSNPRINTF);
        assert_eq!(log.len(), 1);
        assert_eq!(log[0][..3], [0x1000, 0x20, 0x300]);
        // Without variable arguments the call still has a (readable) list pointer.
        e.register(CRT_VSNPRINTF, |_, a| Ret {
            eax: a[3],
            ..Ret::default()
        });
        assert_ne!(e.call(0x00cb_0f60, &[0x1000u32, 0x20, 0x300]).u32(), 0);
    }

    #[test]
    fn str_cmp_answers_minus_one_zero_or_one() {
        let mut e = Engine::new();
        let cases: [(&[u8], &[u8], i32); 9] = [
            (b"hello", b"hello", 0),
            (b"", b"", 0),
            (b"a", b"a", 0),
            (b"ab", b"ab", 0),
            (b"abc", b"abd", -1),
            (b"abd", b"abc", 1),
            (b"abcd", b"abc", 1),
            (b"abc", b"abcd", -1),
            (b"b", b"ab", 1),
        ];
        for (left, right, expected) in cases {
            let l = put_string(&mut e, left);
            let r = put_string(&mut e, right);
            assert_eq!(e.call(0x00cb_0fa0, &args![l, r]).i32(), expected);
        }
    }

    #[test]
    fn str_cmp_compares_the_second_byte_of_a_pair_and_unsigned() {
        let mut e = Engine::new();
        let l = put_string(&mut e, b"xA");
        let r = put_string(&mut e, b"xB");
        assert_eq!(e.call(0x00cb_0fa0, &args![l, r]).i32(), -1);
        assert_eq!(e.call(0x00cb_0fa0, &args![r, l]).i32(), 1);
        // 0x80 is above 0x7f unsigned.
        let high = put_string(&mut e, &[0x80]);
        let low = put_string(&mut e, &[0x7f]);
        assert_eq!(e.call(0x00cb_0fa0, &args![high, low]).i32(), 1);
        assert_eq!(e.call(0x00cb_0fa0, &args![low, high]).i32(), -1);
    }

    #[test]
    fn the_runtime_wrappers_forward_their_words() {
        let mut e = Engine::new();
        for address in [
            CRT_STRNCMP,
            CRT_STRSTR,
            CRT_STRCHR,
            CRT_STRTOUL,
            CRT_STRNCPY,
        ] {
            e.register(address, |_, a| Ret {
                eax: a.iter().fold(0x1000, |sum, word| sum + word),
                ..Ret::default()
            });
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cb_0fd0, &args![1u32, 2u32, 3u32]).u32(), 0x1006);
        assert_eq!(calls_to(&e, CRT_STRNCMP), vec![vec![1, 2, 3]]);
        assert_eq!(e.call(0x00cb_1130, &args![1u32, 2u32]).u32(), 0x1003);
        assert_eq!(calls_to(&e, CRT_STRSTR), vec![vec![1, 2]]);
        assert_eq!(e.call(0x00cb_1140, &args![1u32, 0x2fu32]).u32(), 0x1030);
        assert_eq!(calls_to(&e, CRT_STRCHR), vec![vec![1, 0x2f]]);
        // atoi is strtoul(text, 0, base).
        assert_eq!(e.call(0x00cb_10f0, &args![0x50u32, 16i32]).u32(), 0x1060);
        assert_eq!(calls_to(&e, CRT_STRTOUL), vec![vec![0x50, 0, 16]]);
        // strNcpy: a zero count calls nothing and answers 0.
        assert_eq!(e.call(0x00cb_10b0, &args![1u32, 2u32, 0u32]).u32(), 0);
        assert!(calls_to(&e, CRT_STRNCPY).is_empty());
        assert_eq!(e.call(0x00cb_10b0, &args![1u32, 2u32, 5u32]).u32(), 0x1008);
        assert_eq!(calls_to(&e, CRT_STRNCPY), vec![vec![1, 2, 5]]);
    }

    #[test]
    fn str_cpy_copies_the_terminator_too() {
        let mut e = Engine::new();
        let source = put_string(&mut e, b"abc");
        let target = e.mem.alloc(8);
        for i in 0..8 {
            e.mem.set_u8(target + i, 0xee);
        }
        e.call(0x00cb_1090, &args![target, source]);
        assert_eq!(e.mem.bytes(target, 5), vec![b'a', b'b', b'c', 0, 0xee]);
        // An empty string copies just the zero.
        let empty = put_string(&mut e, b"");
        e.call(0x00cb_1090, &args![target, empty]);
        assert_eq!(e.mem.u8(target), 0);
        assert_eq!(e.mem.u8(target + 1), b'b');
    }

    #[test]
    fn fn_00cb1160_allocates_length_plus_one_and_copies() {
        let mut e = Engine::new();
        let block = e.mem.alloc(16);
        answer(&mut e, EASY_ALLOC, block);
        e.call_log = Some(vec![]);
        let text = put_string(&mut e, b"hello");
        let got = e.call(0x00cb_1160, &args![text, 0xa110cu32]).u32();
        assert_eq!(got, block);
        assert_eq!(calls_to(&e, EASY_ALLOC), vec![vec![0xa110c, 6]]);
        assert_eq!(e.mem.cstr(block), b"hello".to_vec());
    }

    #[test]
    fn fn_00cb1190_frees_through_the_router_with_swapped_arguments() {
        let mut e = Engine::new();
        answer(&mut e, EASY_FREE, 0);
        e.call_log = Some(vec![]);
        e.call(0x00cb_1190, &args![0x5000u32, 0xa110cu32]);
        assert_eq!(calls_to(&e, EASY_FREE), vec![vec![0xa110c, 0x5000]]);
    }

    #[test]
    fn fn_00cb11b0_cuts_the_copy_to_the_limit_and_terminates_it() {
        let mut e = Engine::new();
        let block = e.mem.alloc(16);
        for i in 0..16 {
            e.mem.set_u8(block + i, 0xee);
        }
        answer(&mut e, EASY_ALLOC, block);
        e.register(CRT_STRNCPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        let text = put_string(&mut e, b"hello");
        let got = e.call(0x00cb_11b0, &args![text, 3i32, 0xa110cu32]).u32();
        assert_eq!(got, block);
        assert_eq!(calls_to(&e, EASY_ALLOC), vec![vec![0xa110c, 4]]);
        assert_eq!(e.mem.bytes(block, 5), vec![b'h', b'e', b'l', 0, 0xee]);
        // A limit above the length copies the whole string.
        e.call_log = Some(vec![]);
        e.call(0x00cb_11b0, &args![text, 99i32, 0xa110cu32]);
        assert_eq!(calls_to(&e, EASY_ALLOC), vec![vec![0xa110c, 6]]);
        assert_eq!(e.mem.cstr(block), b"hello".to_vec());
    }

    fn compare(e: &mut Engine, left: &[u8], right: &[u8], count: u32) -> i32 {
        let l = put_string(e, left);
        let r = put_string(e, right);
        e.call(0x00cb_1290, &args![l, r, count]).i32()
    }

    #[test]
    fn mem_cmp_orders_by_the_first_differing_byte() {
        let mut e = Engine::new();
        assert_eq!(compare(&mut e, b"abcdefgh", b"abcdefgh", 8), 0);
        assert_eq!(compare(&mut e, b"abcdefgh", b"abcdefgi", 8), -1);
        assert_eq!(compare(&mut e, b"abcdefgi", b"abcdefgh", 8), 1);
        // Differences in each byte of the first word.
        assert_eq!(compare(&mut e, b"Xbcd", b"abcd", 4), -1);
        assert_eq!(compare(&mut e, b"aXcd", b"abcd", 4), -1);
        assert_eq!(compare(&mut e, b"abXd", b"abcd", 4), -1);
        assert_eq!(compare(&mut e, b"abcX", b"abcd", 4), -1);
        assert_eq!(compare(&mut e, b"abcz", b"abcd", 4), 1);
        // Bytes are unsigned.
        assert_eq!(compare(&mut e, &[0x80], &[0x01], 1), 1);
        assert_eq!(compare(&mut e, &[0x01], &[0x80], 1), -1);
    }

    #[test]
    fn mem_cmp_stops_at_the_count() {
        let mut e = Engine::new();
        assert_eq!(compare(&mut e, b"abcX", b"abcY", 0), 0);
        assert_eq!(compare(&mut e, b"abX", b"abY", 2), 0);
        assert_eq!(compare(&mut e, b"abX", b"abY", 3), -1);
        assert_eq!(compare(&mut e, b"aX", b"aY", 1), 0);
        assert_eq!(compare(&mut e, b"abcdefX", b"abcdefY", 6), 0);
        assert_eq!(compare(&mut e, b"abcdefX", b"abcdefY", 7), -1);
        assert_eq!(compare(&mut e, b"abcdXfg", b"abcdYfg", 7), -1);
    }

    #[test]
    fn str_free_frees_through_the_thread_routers_allocator() {
        let mut e = Engine::new();
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0x10, 0xa110c);
        answer(&mut e, GET_MEMORY_ROUTER, router);
        answer(&mut e, EASY_FREE, 0);
        e.call_log = Some(vec![]);
        e.call(0x00cb_13c0, &args![0x5000u32]);
        assert_eq!(calls_to(&e, EASY_FREE), vec![vec![0xa110c, 0x5000]]);
    }

    #[test]
    fn fn_00cb13e0_copies_with_the_thread_routers_allocator() {
        let mut e = Engine::new();
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0x10, 0xa110c);
        answer(&mut e, GET_MEMORY_ROUTER, router);
        answer(&mut e, MEMORY_ROUTER_CONSTRUCTOR, 0);
        let block = e.mem.alloc(16);
        answer(&mut e, EASY_ALLOC, block);
        e.register(CRT_STRNCPY, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let text = put_string(&mut e, b"hi");
        let got = e.call(0x00cb_13e0, &args![text, 10i32]).u32();
        assert_eq!(got, block);
        assert_eq!(calls_to(&e, MEMORY_ROUTER_CONSTRUCTOR).len(), 1);
        assert_eq!(calls_to(&e, EASY_ALLOC), vec![vec![0xa110c, 3]]);
        assert_eq!(calls_to(&e, CRT_STRNCPY), vec![vec![block, text, 2]]);
        assert_eq!(e.mem.u8(block + 2), 0);
    }
}
