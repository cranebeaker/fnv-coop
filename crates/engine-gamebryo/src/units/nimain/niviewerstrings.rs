//! `NiMain/niviewerstrings.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Every `NiGetViewerString` overload is `cdecl`: it measures the name (`strlen`), allocates
//! the text with `00aa1070` (the name's length plus room for the format's own characters and
//! the value), writes it with `NiSprintf` (`00a305b0`: buffer, size, format, the name, then
//! the value as the format takes it) and answers the text.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa1070` (unnamed, `cdecl`, one word): allocates the text of a viewer string.
const ALLOCATE_TEXT: u32 = 0x00aa_1070;
/// `NiSprintf` (`00a305b0`, `cdecl`: buffer, size, format, then the format's arguments).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// The formats: `"--- %s ---"`, `"%s = true"`, `"%s = false"`, `"%s = %c"`, `"%s = %u"`,
/// `"%s = %hd"`, `"%s = %hu"`, `"%s = %d"`, `"%s = %g"`, `"%s = %p"`, `"%s = %s"`,
/// `"%s = NULL"`.
const FORMAT_HEADING: u32 = 0x0109_dca0;
const FORMAT_TRUE: u32 = 0x0109_dcb8;
const FORMAT_FALSE: u32 = 0x0109_dcac;
const FORMAT_CHAR: u32 = 0x0109_dcc4;
const FORMAT_UNSIGNED: u32 = 0x0109_dccc;
const FORMAT_SHORT: u32 = 0x0109_dcd4;
const FORMAT_UNSIGNED_SHORT: u32 = 0x0109_dce0;
const FORMAT_INT: u32 = 0x0103_ab04;
const FORMAT_FLOAT: u32 = 0x0109_dcec;
const FORMAT_POINTER: u32 = 0x0109_dcf4;
const FORMAT_TEXT: u32 = 0x0109_655c;
const FORMAT_NULL: u32 = 0x0109_dcfc;

/// The length of the C string at `text` (the exe's inline `strlen` loop).
fn text_length(e: &Engine, text: Ptr) -> u32 {
    e.mem.cstr(text.addr()).len() as u32
}

/// Allocates `size` bytes for the text and writes it with `NiSprintf`, the format and the
/// words that follow it (`values`). Answers the text.
fn print(e: &mut Engine, size: u32, format: u32, values: &[u32]) -> Ptr {
    let text = e.call(ALLOCATE_TEXT, &args![size]).u32();
    let mut words = vec![text, size, format];
    words.extend_from_slice(values);
    e.call(NI_SPRINTF, &words);
    Ptr::new(text)
}

// Translated from 00a7d330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGetViewerString` (Xbox PDB): `"--- <name> ---"` (`strlen(name) + 9` bytes), the
/// heading line of a class's viewer strings.
pub fn ni_get_viewer_string(e: &mut Engine, name: Ptr) -> Ptr {
    let size = text_length(e, name).wrapping_add(9);
    print(e, size, FORMAT_HEADING, &args![name])
}

// Translated from 00a7d370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGetViewerString_ov2` (Xbox PDB), a `bool` (the low byte of its word):
/// `"<name> = true"` or `"<name> = false"` (`strlen(name) + 9` bytes).
pub fn ni_get_viewer_string_ov2(e: &mut Engine, name: Ptr, value: bool) -> Ptr {
    let size = text_length(e, name).wrapping_add(9);
    let format = if value { FORMAT_TRUE } else { FORMAT_FALSE };
    print(e, size, format, &args![name])
}

// Translated from 00a7d3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGetViewerString_ov3` (Xbox PDB), a `char`: `"<name> = %c"` (`strlen(name) + 5` bytes)
/// with the character sign-extended to a word.
pub fn ni_get_viewer_string_ov3(e: &mut Engine, name: Ptr, value: i8) -> Ptr {
    let size = text_length(e, name).wrapping_add(5);
    print(e, size, FORMAT_CHAR, &args![name, i32::from(value)])
}

// Translated from 00a7d410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGetViewerString_ov4` (Xbox PDB), an `unsigned char`: `"<name> = %u"` (`strlen(name) +
/// 7` bytes) with the byte zero-extended.
pub fn ni_get_viewer_string_ov4(e: &mut Engine, name: Ptr, value: u8) -> Ptr {
    let size = text_length(e, name).wrapping_add(7);
    print(e, size, FORMAT_UNSIGNED, &args![name, u32::from(value)])
}

// Translated from 00a7d450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGetViewerString_ov5` (Xbox PDB), a `short`: `"<name> = %hd"` (`strlen(name) + 10`
/// bytes) with the value sign-extended.
pub fn ni_get_viewer_string_ov5(e: &mut Engine, name: Ptr, value: i16) -> Ptr {
    let size = text_length(e, name).wrapping_add(10);
    print(e, size, FORMAT_SHORT, &args![name, i32::from(value)])
}

// Translated from 00a7d490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGetViewerString_ov6` (Xbox PDB), an `unsigned short`: `"<name> = %hu"`
/// (`strlen(name) + 9` bytes) with the value zero-extended.
pub fn ni_get_viewer_string_ov6(e: &mut Engine, name: Ptr, value: u16) -> Ptr {
    let size = text_length(e, name).wrapping_add(9);
    print(
        e,
        size,
        FORMAT_UNSIGNED_SHORT,
        &args![name, u32::from(value)],
    )
}

// Translated from 00a7d4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGetViewerString_ov7` (Xbox PDB), an `int`: `"<name> = %d"` (`strlen(name) + 15`
/// bytes).
pub fn ni_get_viewer_string_ov7(e: &mut Engine, name: Ptr, value: i32) -> Ptr {
    let size = text_length(e, name).wrapping_add(0xf);
    print(e, size, FORMAT_INT, &args![name, value])
}

// Translated from 00a7d510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGetViewerString_ov8` (Xbox PDB), an `unsigned int`: `"<name> = %u"` (`strlen(name) +
/// 14` bytes).
pub fn ni_get_viewer_string_ov8(e: &mut Engine, name: Ptr, value: u32) -> Ptr {
    let size = text_length(e, name).wrapping_add(0xe);
    print(e, size, FORMAT_UNSIGNED, &args![name, value])
}

// Translated from 00a7d550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGetViewerString_ov9` (Xbox PDB), a `float`: `"<name> = %g"` (`strlen(name) + 20`
/// bytes) with the value passed as a `double` (`FLD float` then `FSTP double`, which quiets
/// a signalling NaN; two words, low first).
pub fn ni_get_viewer_string_ov9(e: &mut Engine, name: Ptr, value: f32) -> Ptr {
    let size = text_length(e, name).wrapping_add(0x14);
    let value = x87_load32(value.to_bits());
    print(e, size, FORMAT_FLOAT, &args![name, value])
}

// Translated from 00a7d5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGetViewerString_ov10` (Xbox PDB), a pointer: `"<name> = %p"` (`strlen(name) + 16`
/// bytes).
pub fn ni_get_viewer_string_ov10(e: &mut Engine, name: Ptr, value: u32) -> Ptr {
    let size = text_length(e, name).wrapping_add(0x10);
    print(e, size, FORMAT_POINTER, &args![name, value])
}

// Translated from 00a7d5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGetViewerString_ov11` (Xbox PDB), a C string: `"<name> = <value>"` (`strlen(value) +
/// strlen(name) + 4` bytes), or `"<name> = NULL"` (`strlen(name) + 8` bytes) for a null
/// string.
pub fn ni_get_viewer_string_ov11(e: &mut Engine, name: Ptr, value: Ptr) -> Ptr {
    if value.is_null() {
        let size = text_length(e, name).wrapping_add(8);
        return print(e, size, FORMAT_NULL, &args![name]);
    }
    let value_length = text_length(e, value);
    let size = text_length(e, name)
        .wrapping_add(value_length)
        .wrapping_add(4);
    print(e, size, FORMAT_TEXT, &args![name, value])
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_d330, ni_get_viewer_string(Ptr) -> Ptr),
        entry!(0x00a7_d370, ni_get_viewer_string_ov2(Ptr, bool) -> Ptr),
        entry!(0x00a7_d3d0, ni_get_viewer_string_ov3(Ptr, i8) -> Ptr),
        entry!(0x00a7_d410, ni_get_viewer_string_ov4(Ptr, u8) -> Ptr),
        entry!(0x00a7_d450, ni_get_viewer_string_ov5(Ptr, i16) -> Ptr),
        entry!(0x00a7_d490, ni_get_viewer_string_ov6(Ptr, u16) -> Ptr),
        entry!(0x00a7_d4d0, ni_get_viewer_string_ov7(Ptr, i32) -> Ptr),
        entry!(0x00a7_d510, ni_get_viewer_string_ov8(Ptr, u32) -> Ptr),
        entry!(0x00a7_d550, ni_get_viewer_string_ov9(Ptr, f32) -> Ptr),
        entry!(0x00a7_d5a0, ni_get_viewer_string_ov10(Ptr, u32) -> Ptr),
        entry!(0x00a7_d5e0, ni_get_viewer_string_ov11(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(ALLOCATE_TEXT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"abc");
        e.call_log = Some(vec![]);
        (e, name)
    }

    /// The `NiSprintf` call of the last run, and checks the allocation matched its size.
    fn printed(e: &mut Engine, text: u32) -> Vec<u32> {
        let log = e.call_log.replace(vec![]).unwrap();
        let sizes: Vec<_> = log.iter().filter(|c| c.0 == ALLOCATE_TEXT).collect();
        let prints: Vec<_> = log.iter().filter(|c| c.0 == NI_SPRINTF).collect();
        assert_eq!((sizes.len(), prints.len()), (1, 1));
        assert_eq!(prints[0].1[0], text);
        assert_eq!(prints[0].1[1], sizes[0].1[0]);
        prints[0].1[1..].to_vec()
    }

    #[test]
    fn heading() {
        let (mut e, name) = engine();
        let text = e.call(0x00a7_d330, &args![name]).u32();
        assert_eq!(printed(&mut e, text), vec![12, FORMAT_HEADING, name]);
    }

    #[test]
    fn bool_reads_only_the_low_byte() {
        let (mut e, name) = engine();
        let text = e.call(0x00a7_d370, &args![name, 0x100u32]).u32();
        assert_eq!(printed(&mut e, text), vec![12, FORMAT_FALSE, name]);
        let text = e.call(0x00a7_d370, &args![name, 0x101u32]).u32();
        assert_eq!(printed(&mut e, text), vec![12, FORMAT_TRUE, name]);
    }

    #[test]
    fn small_integers_are_extended_by_their_sign() {
        let (mut e, name) = engine();
        let text = e.call(0x00a7_d3d0, &args![name, 0x1234_56ffu32]).u32();
        assert_eq!(
            printed(&mut e, text),
            vec![8, FORMAT_CHAR, name, 0xffff_ffff]
        );
        let text = e.call(0x00a7_d410, &args![name, 0x1234_56ffu32]).u32();
        assert_eq!(printed(&mut e, text), vec![10, FORMAT_UNSIGNED, name, 0xff]);
        let text = e.call(0x00a7_d450, &args![name, 0x1234_8000u32]).u32();
        assert_eq!(
            printed(&mut e, text),
            vec![13, FORMAT_SHORT, name, 0xffff_8000]
        );
        let text = e.call(0x00a7_d490, &args![name, 0x1234_8000u32]).u32();
        assert_eq!(
            printed(&mut e, text),
            vec![12, FORMAT_UNSIGNED_SHORT, name, 0x8000]
        );
    }

    #[test]
    fn words_pass_unchanged() {
        let (mut e, name) = engine();
        let text = e.call(0x00a7_d4d0, &args![name, 0xffff_fff0u32]).u32();
        assert_eq!(
            printed(&mut e, text),
            vec![18, FORMAT_INT, name, 0xffff_fff0]
        );
        let text = e.call(0x00a7_d510, &args![name, 7u32]).u32();
        assert_eq!(printed(&mut e, text), vec![17, FORMAT_UNSIGNED, name, 7]);
        let text = e.call(0x00a7_d5a0, &args![name, 0x0100_0000u32]).u32();
        assert_eq!(
            printed(&mut e, text),
            vec![19, FORMAT_POINTER, name, 0x0100_0000]
        );
    }

    #[test]
    fn float_is_passed_as_a_double() {
        let (mut e, name) = engine();
        let text = e.call(0x00a7_d550, &args![name, 1.5f32]).u32();
        let bits = 1.5f64.to_bits();
        assert_eq!(
            printed(&mut e, text),
            vec![23, FORMAT_FLOAT, name, bits as u32, (bits >> 32) as u32]
        );
        // A signalling NaN is quieted by the load.
        let text = e.call(0x00a7_d550, &args![name, 0x7f80_0001u32]).u32();
        let words = printed(&mut e, text);
        let double = f64::from_bits(u64::from(words[3]) | (u64::from(words[4]) << 32));
        assert_eq!(double.to_bits(), 0x7ff8_0000_2000_0000);
    }

    #[test]
    fn text_or_null() {
        let (mut e, name) = engine();
        let value = e.mem.alloc(8);
        e.mem.set_cstr(value, b"hello");
        let text = e.call(0x00a7_d5e0, &args![name, value]).u32();
        assert_eq!(printed(&mut e, text), vec![12, FORMAT_TEXT, name, value]);
        let text = e.call(0x00a7_d5e0, &args![name, 0u32]).u32();
        assert_eq!(printed(&mut e, text), vec![11, FORMAT_NULL, name]);
    }
}
