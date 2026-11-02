//! `NiMain/nistring.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ecbf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed body (the map puts `NiString::FindOneOf` on it, but it is the CRT's `strpbrk`,
/// cdecl, two words): the first character of `string` that occurs in `characters`, or null.
/// The code marks each byte of `characters` in a 256-bit set on its stack (the closing 0 is
/// never marked), then scans `string` until a marked byte or its end.
pub fn fn_00ecbf20(e: &mut Engine, string: Ptr, characters: Ptr) -> Ptr {
    let mut set = [false; 256];
    let mut p = characters.addr();
    loop {
        let byte = e.mem.u8(p);
        if byte == 0 {
            break;
        }
        p = p.wrapping_add(1);
        set[usize::from(byte)] = true;
    }
    let mut p = string.addr();
    loop {
        let byte = e.mem.u8(p);
        if byte == 0 {
            return Ptr::new(0);
        }
        p = p.wrapping_add(1);
        if set[usize::from(byte)] {
            return Ptr::new(p.wrapping_sub(1));
        }
    }
}

/// `00a88ac0` (cdecl, a size: the block size `NiString` asks for, rounded up to 0x20, 0x40,
/// 0x80, 0xff, 0x200 or 0x400, or one more above that), the allocation `00aa1070` and `NiFree`
/// (`00aa10f0`), `_memcpy` (`00ec44d0`), `atof` (`00eca573`, the result in `ST0`), the
/// `InterlockedDecrement` import slot and `"-0"` (`0109ec00`).
const BLOCK_SIZE_00A88AC0: u32 = 0x00a8_8ac0;
const NI_ALLOC: u32 = 0x00aa_1070;
const NI_FREE: u32 = 0x00aa_10f0;
const MEMCPY: u32 = 0x00ec_44d0;
const ATOF: u32 = 0x00ec_a573;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const MINUS_OR_ZERO: u32 = 0x0109_ec00;

/// `strlen` as the code inlines it.
fn string_length(e: &Engine, text: u32) -> u32 {
    e.mem.cstr(text).len() as u32
}

// Translated from 00a88b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, a C string): a new `NiString` buffer for it, or null for
/// null. The block (`00aa1070`) is the size `00a88ac0` gives for the length (1 for an empty
/// string) plus 0xd; it starts with that size, a reference count of 1 and the length (0, then
/// the string's length once copied with its terminator), and the text follows at +0xc, whose
/// address is answered (null when that address is null).
pub fn fn_00a88b10(e: &mut Engine, text: Ptr) -> Ptr {
    if text.is_null() {
        return Ptr::NULL;
    }
    let length = string_length(e, text.addr());
    let asked = if length == 0 { 1 } else { length };
    let size = e
        .call(BLOCK_SIZE_00A88AC0, &args![asked.wrapping_add(0xd)])
        .u32();
    let block = e.call(NI_ALLOC, &args![size]).u32();
    e.mem.set_u32(block, size);
    let data = block.wrapping_add(0xc);
    e.mem.set_u32(block.wrapping_add(4), 1);
    e.mem.set_u32(block.wrapping_add(8), 0);
    if data == 0 {
        return Ptr::NULL;
    }
    e.call(MEMCPY, &args![data, text, length.wrapping_add(1)]);
    e.mem.set_u32(data.wrapping_sub(4), length);
    Ptr::new(data)
}

// Translated from 00a88b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiString::NiString` (Xbox PDB; `__thiscall`, a C string): the buffer `00a88b10` makes for
/// it, its length word (-4) set again from the copied text. Answers `this`.
pub fn ni_string_ni_string(e: &mut Engine, this: Ptr, text: Ptr) -> Ptr {
    let data = fn_00a88b10(e, text).addr();
    e.mem.set_u32(this.addr(), data);
    if data != 0 {
        let length = string_length(e, data);
        e.mem.set_u32(data.wrapping_sub(4), length);
    }
    this
}

// Translated from 00a88bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiString::~NiString` (Xbox PDB): with a buffer, its reference count (-8) goes down
/// (`InterlockedDecrement`); when the count in memory is then 0 the block (-0xc) is freed.
/// The pointer is cleared.
pub fn ni_string_destructor(e: &mut Engine, this: Ptr) {
    let data = e.mem.u32(this.addr());
    if data == 0 {
        return;
    }
    let count = data.wrapping_sub(8);
    e.call(INTERLOCKED_DECREMENT, &args![count]);
    if e.mem.u32(count) == 0 {
        let data = e.mem.u32(this.addr());
        if data != 0 {
            e.call(NI_FREE, &args![data.wrapping_sub(0xc)]);
            e.mem.set_u32(this.addr(), 0);
        }
    }
    e.mem.set_u32(this.addr(), 0);
}

// Translated from 00a88c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiString::ToFloat` (Xbox PDB; `__thiscall`, the address of the result): false without a
/// buffer. Otherwise the text's `atof` rounded to a `float` is stored and the answer is true
/// unless it is 0 (an ordered compare: NaN answers true); a 0 counts only when the text holds
/// a `'-'` or `'0'` (`strpbrk` with `"-0"`, `00ecbf20`) at a non-negative offset.
pub fn ni_string_to_float(e: &mut Engine, this: Ptr, result: Ptr) -> bool {
    let text = e.mem.u32(this.addr());
    if text == 0 {
        return false;
    }
    let value = x87_store32(e.call(ATOF, &args![text]).f64());
    e.mem.set_u32(result.addr(), x87_quiet_bits(value));
    // `FLDZ; FUCOMPP; TEST AH,0x44; JP`: anything but an ordered 0 is a number.
    if x87_load32(value) != 0.0 {
        return true;
    }
    let text = e.mem.u32(this.addr());
    if text == 0 {
        return false;
    }
    let found = fn_00ecbf20(e, Ptr::new(text), Ptr::new(MINUS_OR_ZERO)).addr();
    if found == 0 {
        return false;
    }
    let offset = found.wrapping_sub(e.mem.u32(this.addr()));
    (offset as i32) >= 0 && offset != 0xffff_ffff
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ecbf20, fn_00ecbf20(Ptr, Ptr) -> Ptr),
        entry!(0x00a88b10, fn_00a88b10(Ptr) -> Ptr),
        entry!(0x00a88b80, ni_string_ni_string(Ptr, Ptr) -> Ptr),
        entry!(0x00a88bc0, ni_string_destructor(Ptr)),
        entry!(0x00a88c00, ni_string_to_float(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod b007_tests {
    use super::*;

    fn string(e: &mut Engine, bytes: &[u8]) -> u32 {
        let p = e.mem.alloc(bytes.len() as u32 + 1);
        for (i, b) in bytes.iter().enumerate() {
            e.mem.set_u8(p + i as u32, *b);
        }
        p
    }

    #[test]
    fn fn_00ecbf20_finds_the_first_listed_character() {
        let mut e = Engine::new();
        let s = string(&mut e, b"textures\\a/b.dds");
        let separators = string(&mut e, b"/\\");
        let none = string(&mut e, b"#");
        let empty = string(&mut e, b"");
        assert_eq!(e.call(0x00ecbf20, &args![s, separators]).u32(), s + 8);
        assert_eq!(e.call(0x00ecbf20, &args![s, none]).u32(), 0);
        assert_eq!(e.call(0x00ecbf20, &args![s, empty]).u32(), 0);
        assert_eq!(e.call(0x00ecbf20, &args![empty, separators]).u32(), 0);
    }
}

#[cfg(test)]
mod n019_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn string(e: &mut Engine, bytes: &[u8]) -> u32 {
        let p = e.mem.alloc(bytes.len() as u32 + 1);
        e.mem.set_cstr(p, bytes);
        p
    }

    fn doubles(e: &mut Engine) {
        e.register(BLOCK_SIZE_00A88AC0, |_, a| word(a[0].max(0x20)));
        e.register(NI_ALLOC, |e, a| word(e.mem.alloc(a[0])));
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            word(a[0])
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            word(count)
        });
    }

    #[test]
    fn buffers_hold_size_count_length_and_text() {
        let mut e = Engine::new();
        doubles(&mut e);
        let text = string(&mut e, b"bone");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_8b10, &args![0u32]).u32(), 0);
        let data = e.call(0x00a8_8b10, &args![text]).u32();
        assert_eq!(calls_to(&e, BLOCK_SIZE_00A88AC0), vec![vec![4 + 0xd]]);
        assert_eq!(e.mem.cstr(data), b"bone");
        let block = data - 0xc;
        assert_eq!(
            (e.mem.u32(block), e.mem.u32(block + 4), e.mem.u32(block + 8)),
            (0x20, 1, 4)
        );
        // The empty string asks for one byte.
        let empty = string(&mut e, b"");
        e.call(0x00a8_8b10, &args![empty]);
        assert_eq!(calls_to(&e, BLOCK_SIZE_00A88AC0)[1], vec![1 + 0xd]);
        // The constructor stores the buffer and its length.
        let holder = e.mem.alloc(4);
        assert_eq!(e.call(0x00a8_8b80, &args![holder, text]).u32(), holder);
        let data = e.mem.u32(holder);
        assert_eq!(e.mem.u32(data - 4), 4);
        e.call(0x00a8_8b80, &args![holder, 0u32]);
        assert_eq!(e.mem.u32(holder), 0);
    }

    #[test]
    fn destructor_frees_at_the_last_reference() {
        let mut e = Engine::new();
        doubles(&mut e);
        let text = string(&mut e, b"x");
        let holder = e.mem.alloc(4);
        e.call(0x00a8_8b80, &args![holder, text]);
        let data = e.mem.u32(holder);
        e.mem.set_u32(data - 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a8_8bc0, &args![holder]);
        assert_eq!(e.mem.u32(holder), 0);
        assert_eq!(calls_to(&e, NI_FREE).len(), 0);
        e.mem.set_u32(holder, data);
        e.call(0x00a8_8bc0, &args![holder]);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![data - 0xc]]);
        assert_eq!(e.mem.u32(holder), 0);
        // Nothing for an empty holder.
        e.call(0x00a8_8bc0, &args![holder]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 2);
    }

    #[test]
    fn to_float_accepts_numbers_and_spelled_zeros() {
        let mut e = Engine::new();
        e.map(MINUS_OR_ZERO, 4);
        e.mem.set_cstr(MINUS_OR_ZERO, b"-0");
        // `atof` answers 2.5 for "2.5", NaN for "nan" and 0 otherwise.
        e.register(ATOF, |e, a| {
            let text = e.mem.cstr(a[0]);
            let st0 = match &text[..] {
                b"2.5" => 2.5,
                b"nan" => f64::NAN,
                _ => 0.0,
            };
            Ret {
                st0,
                ..Ret::default()
            }
        });
        let holder = e.mem.alloc(4);
        let result = e.mem.alloc(4);
        assert!(!e.call(0x00a8_8c00, &args![holder, result]).bool());
        for (text, expected) in [
            (&b"2.5"[..], true),
            (b"nan", true),
            (b"abc", false),
            (b"0.0", true),
            (b"x-", true),
        ] {
            let s = string(&mut e, text);
            e.mem.set_u32(holder, s);
            assert_eq!(e.call(0x00a8_8c00, &args![holder, result]).bool(), expected);
        }
        let s = string(&mut e, b"2.5");
        e.mem.set_u32(holder, s);
        e.call(0x00a8_8c00, &args![holder, result]);
        assert_eq!(e.mem.f32(result), 2.5);
    }
}
