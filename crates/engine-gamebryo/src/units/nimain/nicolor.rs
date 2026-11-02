//! `NiMain/nicolor.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// Moves `count` 4-byte words between `this` and the stream: through the input
/// (`m_pkIstr`, its read function) when `write` is false, the output (`m_pkOstr`, its write
/// function) otherwise; each call is `function(binary, this + 4 i, 4, &count, 1)` with the
/// stream argument's stack word, set to 4, as the byte-count word, and the answer is added
/// to the binary stream's position.
fn transfer_words(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>, count: u32, write: bool) {
    e.with_stack(4, |e, size| {
        for word in 0..count {
            let binary = if write {
                e.get(stream, NiStream::m_pkOstr)
            } else {
                e.get(stream, NiStream::m_pkIstr)
            };
            let binary = Ptr::<NiBinaryStream>::new(binary);
            let function = if write {
                e.get(binary, NiBinaryStream::m_pfnWrite)
            } else {
                e.get(binary, NiBinaryStream::m_pfnRead)
            };
            e.mem.set_u32(size.addr(), 4);
            let destination = this.addr() + 4 * word;
            let done = e
                .call(function, &args![binary, destination, 4u32, size, 1u32])
                .u32();
            let position = e.get(binary, NiBinaryStream::m_uiAbsoluteCurrentPos);
            e.set(
                binary,
                NiBinaryStream::m_uiAbsoluteCurrentPos,
                position.wrapping_add(done),
            );
        }
    });
}

// Translated from 00a59be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColor::LoadBinary` (Xbox PDB): reads the three 4-byte components `r g b` in place from
/// the stream's input (identical code also loads other three-float values, such as
/// `NiQuatTransform`'s translation).
pub fn ni_color_load_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    transfer_words(e, this, stream, 3, false);
}

// Translated from 00a69780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColor::SaveBinary` (Xbox PDB): writes the three 4-byte components to the stream's
/// output (`m_pkOstr` +0x250, its write function at +0xc).
pub fn ni_color_save_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    transfer_words(e, this, stream, 3, true);
}

// Translated from 00a69800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorA::LoadBinary` (Xbox PDB): reads the four 4-byte components `r g b a` in place
/// from the stream's input.
pub fn ni_color_a_load_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    transfer_words(e, this, stream, 4, false);
}

// Translated from 00a698a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorA::SaveBinary` (Xbox PDB): writes the four 4-byte components to the stream's
/// output (`m_pkOstr` +0x250, its write function at +0xc).
pub fn ni_color_a_save_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    transfer_words(e, this, stream, 4, true);
}

/// `00aa1070` (unnamed, `cdecl`, one word): allocates the text of a viewer string.
const ALLOCATE_TEXT: u32 = 0x00aa_1070;
/// `NiSprintf` (`00a305b0`, `cdecl`: buffer, size, format, then the format's arguments).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `"%s = (%5.3f,%5.3f,%5.3f)"` and `"%s = (%5.3f,%5.3f,%5.3f,%5.3f)"`.
const FORMAT_COLOR: u32 = 0x0109_c344;
const FORMAT_COLOR_A: u32 = 0x0109_c360;

/// The viewer string `"<name> = (c0,c1,...)"` of the first `count` floats at `this`: a text of
/// `strlen(name) + extra` bytes from `00aa1070`, written by `NiSprintf` with each float
/// widened to a `double` (`FLD dword`, `FSTP qword`).
fn components_viewer_string(
    e: &mut Engine,
    this: Ptr,
    name: Ptr,
    count: u32,
    extra: u32,
    format: u32,
) -> Ptr {
    let size = (e.mem.cstr(name.addr()).len() as u32).wrapping_add(extra);
    let text = e.call(ALLOCATE_TEXT, &args![size]).u32();
    let mut words = args![text, size, format, name];
    for component in 0..count {
        let value = x87_load32(e.mem.u32(this.addr() + 4 * component));
        words.extend(args![value]);
    }
    e.call(NI_SPRINTF, &words);
    Ptr::new(text)
}

// Translated from 00a696c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColor::GetViewerString` (Xbox PDB): `"<name> = (r,g,b)"` (format `0109c344`, each
/// `%5.3f`), in a text of `strlen(name) + 0x22` bytes.
pub fn ni_color_get_viewer_string(e: &mut Engine, this: Ptr, name: Ptr) -> Ptr {
    components_viewer_string(e, this, name, 3, 0x22, FORMAT_COLOR)
}

// Translated from 00a69720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorA::GetViewerString` (Xbox PDB): `"<name> = (r,g,b,a)"` (format `0109c360`), in a
/// text of `strlen(name) + 0x24` bytes.
pub fn ni_color_a_get_viewer_string(e: &mut Engine, this: Ptr, name: Ptr) -> Ptr {
    components_viewer_string(e, this, name, 4, 0x24, FORMAT_COLOR_A)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5_9be0, ni_color_load_binary(Ptr, Ptr<NiStream>)),
        entry!(0x00a6_9780, ni_color_save_binary(Ptr, Ptr<NiStream>)),
        entry!(0x00a6_9800, ni_color_a_load_binary(Ptr, Ptr<NiStream>)),
        entry!(0x00a6_98a0, ni_color_a_save_binary(Ptr, Ptr<NiStream>)),
        entry!(0x00a6_96c0, ni_color_get_viewer_string(Ptr, Ptr) -> Ptr),
        entry!(0x00a6_9720, ni_color_a_get_viewer_string(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A stream whose input and output are one binary stream: reads answer words counting up
    /// from 1, writes record the words.
    fn stream(e: &mut Engine) -> (Ptr<NiStream>, Ptr<NiBinaryStream>) {
        let stream = e.new_object::<NiStream>();
        let binary = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, binary.addr());
        e.set(stream, NiStream::m_pkOstr, binary.addr());
        e.set(binary, NiBinaryStream::m_pfnRead, 0x0300_0000);
        e.set(binary, NiBinaryStream::m_pfnWrite, 0x0300_0004);
        let mut next = 1u32;
        e.register_double(0x0300_0000, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], next);
            next += 1;
            4u32.into_ret()
        });
        e.register(0x0300_0004, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            4u32.into_ret()
        });
        (stream, binary)
    }

    #[test]
    fn load_binary_reads_three_words() {
        let mut e = Engine::new();
        let (stream, binary) = stream(&mut e);
        let color = e.mem.alloc(0x10);
        e.call(0x00a5_9be0, &args![color, stream]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(color + 4 * i)).collect();
        assert_eq!(words, vec![1, 2, 3, 0]);
        assert_eq!(e.get(binary, NiBinaryStream::m_uiAbsoluteCurrentPos), 12);
    }

    #[test]
    fn save_binary_writes_three_words() {
        let mut e = Engine::new();
        let (stream, binary) = stream(&mut e);
        let color = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        e.call(0x00a6_9780, &args![color, stream]);
        let log = e.call_log.take().unwrap();
        let writes: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == 0x0300_0004)
            .map(|c| c.1[1])
            .collect();
        assert_eq!(writes, vec![color, color + 4, color + 8]);
        assert_eq!(e.get(binary, NiBinaryStream::m_uiAbsoluteCurrentPos), 12);
    }

    #[test]
    fn color_a_load_binary_reads_four_words() {
        let mut e = Engine::new();
        let (stream, binary) = stream(&mut e);
        let color = e.mem.alloc(0x10);
        e.call(0x00a6_9800, &args![color, stream]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(color + 4 * i)).collect();
        assert_eq!(words, vec![1, 2, 3, 4]);
        assert_eq!(e.get(binary, NiBinaryStream::m_uiAbsoluteCurrentPos), 16);
    }
}

#[cfg(test)]
mod n015_tests {
    use super::*;

    #[test]
    fn color_a_save_binary_writes_four_words() {
        let mut e = Engine::new();
        let stream = e.new_object::<NiStream>();
        let binary = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkOstr, binary.addr());
        e.set(binary, NiBinaryStream::m_pfnWrite, 0x0300_0004);
        e.register(0x0300_0004, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            4u32.into_ret()
        });
        let color = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00a6_98a0, &args![color, stream]);
        let log = e.call_log.take().unwrap();
        let writes: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == 0x0300_0004)
            .map(|c| c.1[1])
            .collect();
        assert_eq!(writes, vec![color, color + 4, color + 8, color + 0xc]);
        assert_eq!(e.get(binary, NiBinaryStream::m_uiAbsoluteCurrentPos), 16);
    }

    fn sprintf_calls(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == NI_SPRINTF)
            .map(|c| c.1)
            .collect()
    }

    #[test]
    fn viewer_strings_pass_the_components_as_doubles() {
        let mut e = Engine::new();
        e.register(ALLOCATE_TEXT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"m_k");
        let color = e.mem.alloc(0x10);
        e.mem.set_f32(color, 0.5);
        e.mem.set_f32(color + 4, -1.25);
        e.mem.set_f32(color + 8, 3.0);
        e.mem.set_u32(color + 0xc, 0x7f80_0001);
        e.call_log = Some(vec![]);
        let text = e.call(0x00a6_96c0, &args![color, name]).u32();
        let mut expected = args![text, 0x25u32, FORMAT_COLOR, name];
        expected.extend(args![0.5f64, -1.25f64, 3.0f64]);
        assert_eq!(sprintf_calls(&e), vec![expected]);

        e.call_log = Some(vec![]);
        let text = e.call(0x00a6_9720, &args![color, name]).u32();
        let calls = sprintf_calls(&e);
        assert_eq!(calls[0][..4], [text, 0x27, FORMAT_COLOR_A, name]);
        assert_eq!(calls[0].len(), 12);
        // The signalling NaN alpha is quiet once loaded.
        let alpha = f64::from_bits(u64::from(calls[0][10]) | u64::from(calls[0][11]) << 32);
        assert!(alpha.is_nan());
        assert_eq!(alpha.to_bits(), 0x7ff8_0000_2000_0000);
    }
}
