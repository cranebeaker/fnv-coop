//! `NiAnimation/nistringpalette.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiStringPalette::ms_RTTI` (the class's `NiRTTI` record).
const NI_STRING_PALETTE_RTTI: u32 = 0x011f_3f30;

// Translated from 00a47140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStringPalette::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_string_palette_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_STRING_PALETTE_RTTI)
}

layout! {
    /// `NiStringPalette` (Xbox PDB), 0x14 bytes on PC (the same).
    pub struct NiStringPalette: 0x14 {
        /// `m_pcBuffer` (Xbox PDB): `char*`.
        0x08 m_pcBuffer: u32,
        /// `m_uiAllocatedSize` (Xbox PDB).
        0x0C m_uiAllocatedSize: u32,
        /// `m_uiEndOfBuffer` (Xbox PDB).
        0x10 m_uiEndOfBuffer: u32,
    }
}

/// `NiStringPalette`'s vtable.
const NI_STRING_PALETTE_VTABLE: u32 = 0x0109_832c;
/// `operator new` (`00aa13e0`, `cdecl`, the size), the sized delete (`00aa1460`, `cdecl`,
/// (block, size)), the `char` allocation (`00aa1070`, `cdecl`, size) and its free (`00aa10f0`,
/// `cdecl`, pointer).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
const CHAR_ALLOCATE: u32 = 0x00aa_1070;
const CHAR_FREE: u32 = 0x00aa_10f0;
/// `NiObject`'s constructor (`00a5d3a0`) and destructor body (`00a5d3d0`), `thiscall`
/// without stack words; `NiObject::IsEqual` (`00a5d470`; a bool), the base `SaveBinary`
/// `00a89af0` and `NiObject::LoadBinary` (`00a5d610`), `thiscall` with one word.
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
const OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const OBJECT_SAVE_BINARY: u32 = 0x00a8_9af0;
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;

/// One call of the stream's input (`NiStream::m_pkIstr` +0x24C, `m_pfnRead` +0x8) or output
/// (`m_pkOstr` +0x250, `m_pfnWrite` +0xC) function: `function(file, buffer, size, &component
/// size, 1)` with the component size word at `size_word` set to `component`, the bytes it
/// answers added to the file's position (+0x4).
fn stream_transfer(
    e: &mut Engine,
    (file_field, function_field): (u32, u32),
    stream: u32,
    buffer: u32,
    size: u32,
    (component, size_word): (u32, u32),
) {
    let file = e.mem.u32(stream.wrapping_add(file_field));
    let function = e.mem.u32(file.wrapping_add(function_field));
    e.mem.set_u32(size_word, component);
    let done = e
        .call(function, &args![file, buffer, size, size_word, 1u32])
        .u32();
    let position = e.mem.u32(file.wrapping_add(4));
    e.mem
        .set_u32(file.wrapping_add(4), position.wrapping_add(done));
}

fn stream_read(e: &mut Engine, stream: u32, buffer: u32, size: u32, component: (u32, u32)) {
    stream_transfer(e, (0x24c, 8), stream, buffer, size, component);
}

fn stream_write(e: &mut Engine, stream: u32, buffer: u32, size: u32, component: (u32, u32)) {
    stream_transfer(e, (0x250, 0xc), stream, buffer, size, component);
}

// Translated from 00a470c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStringPalette::IsEqual` (Xbox PDB; `thiscall`, the other object): `NiObject::IsEqual`,
/// the allocated sizes and buffer ends equal, then the first `m_uiAllocatedSize` bytes of
/// the two buffers equal (a word at a time, the rest byte by byte).
pub fn ni_string_palette_is_equal(
    e: &mut Engine,
    this: Ptr<NiStringPalette>,
    other: Ptr<NiStringPalette>,
) -> bool {
    if !e.call(OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let size = e.get(this, NiStringPalette::m_uiAllocatedSize);
    if e.get(other, NiStringPalette::m_uiAllocatedSize) != size
        || e.get(other, NiStringPalette::m_uiEndOfBuffer)
            != e.get(this, NiStringPalette::m_uiEndOfBuffer)
    {
        return false;
    }
    if size == 0 {
        return true;
    }
    let mut mine = e.get(this, NiStringPalette::m_pcBuffer);
    let mut theirs = e.get(other, NiStringPalette::m_pcBuffer);
    let mut left = size;
    while left >= 4 {
        if e.mem.u32(theirs) != e.mem.u32(mine) {
            return false;
        }
        left -= 4;
        mine = mine.wrapping_add(4);
        theirs = theirs.wrapping_add(4);
    }
    for byte in 0..left {
        if e.mem.u8(mine.wrapping_add(byte)) != e.mem.u8(theirs.wrapping_add(byte)) {
            return false;
        }
    }
    true
}

// Translated from 00a47150 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiStringPalette`): 0x14 bytes from `operator new`; when that succeeds, the
/// `NiObject` constructor, the vtable `0109832c` and an empty buffer. Answers the object,
/// or null.
pub fn fn_00a47150(e: &mut Engine) -> Ptr<NiStringPalette> {
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(OBJECT_CONSTRUCT, &args![block]);
    let this = Ptr::<NiStringPalette>::new(block);
    e.mem.set_u32(block, NI_STRING_PALETTE_VTABLE);
    e.set(this, NiStringPalette::m_pcBuffer, 0);
    e.set(this, NiStringPalette::m_uiAllocatedSize, 0);
    e.set(this, NiStringPalette::m_uiEndOfBuffer, 0);
    this
}

// Translated from 00a47190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStringPalette::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`, the flags): the
/// vtable, the buffer freed (`00aa10f0`), the `NiObject` destructor body, and with bit 0 of
/// `flags` the sized delete of 0x14 bytes. Answers `this`.
pub fn ni_string_palette_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiStringPalette>,
    flags: u32,
) -> Ptr<NiStringPalette> {
    let buffer = e.get(this, NiStringPalette::m_pcBuffer);
    e.mem.set_u32(this.addr(), NI_STRING_PALETTE_VTABLE);
    e.call(CHAR_FREE, &args![buffer]);
    e.call(OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00a47210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStringPalette::SaveBinary` (Xbox PDB; `thiscall`, the stream): the base `SaveBinary`
/// (`00a89af0`), the buffer end (4 bytes, from a copy in the stream argument's own stack
/// slot), then for a non-zero end that many bytes of the buffer (component size 1), then
/// the buffer end again from the object; the component size word is the function's local.
pub fn ni_string_palette_save_binary(e: &mut Engine, this: Ptr<NiStringPalette>, stream: u32) {
    e.call(OBJECT_SAVE_BINARY, &args![this, stream]);
    e.with_stack(8, |e, slots| {
        let argument = slots.addr();
        let size_word = argument + 4;
        let end = e.get(this, NiStringPalette::m_uiEndOfBuffer);
        e.mem.set_u32(argument, end);
        stream_write(e, stream, argument, 4, (4, size_word));
        let end = e.mem.u32(argument);
        if end != 0 {
            let buffer = e.get(this, NiStringPalette::m_pcBuffer);
            stream_write(e, stream, buffer, end, (1, size_word));
        }
        stream_write(e, stream, this.addr() + 0x10, 4, (4, size_word));
    });
}

// Translated from 00a472b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStringPalette::LoadBinary` (Xbox PDB; `thiscall`, the stream): `NiObject::LoadBinary`,
/// the allocated size (4 bytes); for a non-zero size a buffer of that size (`00aa1070`)
/// filled from the stream (component size 1); then the buffer end (4 bytes). The component
/// size word is the stream argument's own stack slot.
pub fn ni_string_palette_load_binary(e: &mut Engine, this: Ptr<NiStringPalette>, stream: u32) {
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, slot| {
        let size_word = slot.addr();
        stream_read(e, stream, this.addr() + 0xc, 4, (4, size_word));
        let size = e.get(this, NiStringPalette::m_uiAllocatedSize);
        if size != 0 {
            let buffer = e.call(CHAR_ALLOCATE, &args![size]).u32();
            let size = e.get(this, NiStringPalette::m_uiAllocatedSize);
            e.set(this, NiStringPalette::m_pcBuffer, buffer);
            stream_read(e, stream, buffer, size, (1, size_word));
        }
        stream_read(e, stream, this.addr() + 0x10, 4, (4, size_word));
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a47140, ni_string_palette_get_rtti() -> Ptr),
        entry!(
            0x00a4_70c0,
            ni_string_palette_is_equal(Ptr<NiStringPalette>, Ptr<NiStringPalette>) -> bool
        ),
        entry!(0x00a4_7150, fn_00a47150() -> Ptr<NiStringPalette>),
        entry!(
            0x00a4_7190,
            ni_string_palette_scalar_deleting_destructor(
                Ptr<NiStringPalette>,
                u32,
            ) -> Ptr<NiStringPalette>
        ),
        entry!(
            0x00a4_7210,
            ni_string_palette_save_binary(Ptr<NiStringPalette>, u32)
        ),
        entry!(
            0x00a4_72b0,
            ni_string_palette_load_binary(Ptr<NiStringPalette>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_string_palette_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a47140, &args![]).u32(), NI_STRING_PALETTE_RTTI);
    }
}

#[cfg(test)]
mod n005_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn palette(e: &mut Engine, text: &[u8], end: u32) -> Ptr<NiStringPalette> {
        let this = e.new_object::<NiStringPalette>();
        let buffer = e.mem.alloc(text.len() as u32 + 4);
        e.mem.write(buffer, text);
        e.set(this, NiStringPalette::m_pcBuffer, buffer);
        e.set(this, NiStringPalette::m_uiAllocatedSize, text.len() as u32);
        e.set(this, NiStringPalette::m_uiEndOfBuffer, end);
        this
    }

    #[test]
    fn is_equal_compares_sizes_and_buffer_bytes() {
        let mut e = Engine::new();
        e.register(OBJECT_IS_EQUAL, |_, _| true.into_ret());
        let a = palette(&mut e, b"abcdefg", 5);
        let b = palette(&mut e, b"abcdefg", 5);
        let equal = |e: &mut Engine, x: Ptr<NiStringPalette>, y: Ptr<NiStringPalette>| {
            e.call(0x00a4_70c0, &[x.addr(), y.addr()]).bool()
        };
        assert!(equal(&mut e, a, b));
        // A difference in the word part and in the byte tail.
        let c = palette(&mut e, b"abXdefg", 5);
        assert!(!equal(&mut e, a, c));
        let d = palette(&mut e, b"abcdefX", 5);
        assert!(!equal(&mut e, a, d));
        let shorter = palette(&mut e, b"abcdef", 5);
        assert!(!equal(&mut e, a, shorter));
        let other_end = palette(&mut e, b"abcdefg", 6);
        assert!(!equal(&mut e, a, other_end));
        let empty_a = palette(&mut e, b"", 0);
        let empty_b = palette(&mut e, b"", 0);
        assert!(equal(&mut e, empty_a, empty_b));
        e.register(OBJECT_IS_EQUAL, |_, _| false.into_ret());
        assert!(!equal(&mut e, a, b));
    }

    #[test]
    fn create_object_builds_an_empty_palette() {
        let mut e = Engine::new();
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.set_u32(block + 8, 0xdead);
            block.into_ret()
        });
        let made = e.call(0x00a4_7150, &[]).u32();
        assert_eq!(e.mem.u32(made), NI_STRING_PALETTE_VTABLE);
        assert_eq!(e.mem.u32(made + 8), 0);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_7150, &[]).u32(), 0);
        assert!(calls_to(&e, OBJECT_CONSTRUCT).is_empty());
    }

    #[test]
    fn destructor_frees_the_buffer_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        e.register(CHAR_FREE, |_, _| Ret::default());
        e.register(OBJECT_DESTRUCT, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = palette(&mut e, b"ab", 2);
        let buffer = e.get(this, NiStringPalette::m_pcBuffer);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_7190, &[this.addr(), 0]).u32(), this.addr());
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        e.call(0x00a4_7190, &[this.addr(), 1]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this.addr(), 0x14]]);
        assert_eq!(calls_to(&e, CHAR_FREE), vec![vec![buffer], vec![buffer]]);
        assert_eq!(e.mem.u32(this.addr()), NI_STRING_PALETTE_VTABLE);
    }

    /// A stream whose input and output functions are `0x0300_0000` and `0x0300_0004`.
    fn stream(e: &mut Engine) -> (u32, u32) {
        let stream = e.mem.alloc(0x300);
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, 0x0300_0000);
        e.mem.set_u32(file + 0xc, 0x0300_0004);
        e.mem.set_u32(stream + 0x24c, file);
        e.mem.set_u32(stream + 0x250, file);
        (stream, file)
    }

    #[test]
    fn save_binary_writes_end_buffer_and_end() {
        let mut e = Engine::new();
        e.register(OBJECT_SAVE_BINARY, |_, _| Ret::default());
        e.register(0x0300_0004, |e, a| {
            // Every write's component size word.
            let component = e.mem.u32(a[3]);
            (a[2] * 0x100 + component).into_ret()
        });
        let (stream, file) = stream(&mut e);
        let this = palette(&mut e, b"abc", 3);
        e.call_log = Some(vec![]);
        e.call(0x00a4_7210, &[this.addr(), stream]);
        let writes = calls_to(&e, 0x0300_0004);
        let buffer = e.get(this, NiStringPalette::m_pcBuffer);
        assert_eq!(writes.len(), 3);
        assert_eq!(writes[1][1..3], [buffer, 3]);
        assert_eq!(writes[2][1..3], [this.addr() + 0x10, 4]);
        assert_eq!(e.mem.u32(file + 4), 0x404 + 0x301 + 0x404);
        // A zero end writes no buffer.
        e.set(this, NiStringPalette::m_uiEndOfBuffer, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_7210, &[this.addr(), stream]);
        assert_eq!(calls_to(&e, 0x0300_0004).len(), 2);
    }

    #[test]
    fn load_binary_reads_size_buffer_and_end() {
        let mut e = Engine::new();
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        let buffer = e.mem.alloc(8);
        e.register_double(CHAR_ALLOCATE, move |_, _| buffer.into_ret());
        let mut words = vec![3u32, 9];
        e.register_double(0x0300_0000, move |e, a| {
            if a[2] == 4 {
                assert_eq!(e.mem.u32(a[3]), 4);
                e.mem.set_u32(a[1], words.remove(0));
            } else {
                assert_eq!(e.mem.u32(a[3]), 1);
                e.mem.write(a[1], &b"xyz"[..a[2] as usize]);
            }
            a[2].into_ret()
        });
        let (stream, file) = stream(&mut e);
        let this = e.new_object::<NiStringPalette>();
        e.call_log = Some(vec![]);
        e.call(0x00a4_72b0, &[this.addr(), stream]);
        assert_eq!(e.get(this, NiStringPalette::m_uiAllocatedSize), 3);
        assert_eq!(e.get(this, NiStringPalette::m_pcBuffer), buffer);
        assert_eq!(e.mem.bytes(buffer, 3), b"xyz".to_vec());
        assert_eq!(e.get(this, NiStringPalette::m_uiEndOfBuffer), 9);
        assert_eq!(e.mem.u32(file + 4), 11);
        assert_eq!(calls_to(&e, CHAR_ALLOCATE), vec![vec![3]]);
    }
}
