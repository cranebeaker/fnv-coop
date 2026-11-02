//! `Havok SDK/hktagfilereader.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkBinaryTagfile::isBinaryMagic` (`00d483a0`, cdecl: result byte, two words; answers the
/// result pointer).
const IS_BINARY_MAGIC: u32 = 0x00d4_83a0;

/// The format word `detectFormat` stores when the stream gave fewer than 16 bytes, and for a
/// binary tagfile and for anything else.
const FORMAT_UNKNOWN: u32 = 0;
const FORMAT_OTHER: u32 = 1;
const FORMAT_BINARY: u32 = 2;

// Translated from 00dcca60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTagfileReader::detectFormat` (Xbox PDB), cdecl, `(out, stream)`: marks the stream (slot
/// `0x1c`, argument 16), reads 16 bytes into a local buffer (slot `0x10`) and stores 0 at the word
/// `out` when it did not get all 16. Otherwise it rewinds to the mark (slot `0x20`) and stores 2
/// when the first two words of the buffer are the binary tagfile magic
/// (`hkBinaryTagfile::isBinaryMagic`, `00d483a0`, its result byte going to a scratch word), else 1.
/// Answers `out`.
pub fn hk_tagfile_reader_detect_format(e: &mut Engine, out: u32, stream: u32) -> u32 {
    e.vcall(stream, 0x1c, &args![0x10u32]);
    // The exe's frame (`SUB ESP,0x10`, one push) taken with the arguments: the 16 bytes read at
    // +0x4 (00dcca7a); the magic test's byte goes to the `stream` argument's own slot, +0x1c
    // past the return address at +0x14 (00dccaaa).
    let read_words = e.with_stack(0x20, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x18, out);
        e.mem.set_u32(frame + 0x1c, stream);
        let buffer = frame + 4;
        let read = e.vcall(stream, 0x10, &args![buffer, 0x10u32]).u32();
        if read != 0x10 {
            return None;
        }
        e.vcall(stream, 0x20, &args![]);
        let first = e.mem.u32(buffer);
        let second = e.mem.u32(buffer + 4);
        let flag = frame + 0x1c;
        let at = e.call(IS_BINARY_MAGIC, &args![flag, first, second]).u32();
        Some(e.mem.u8(at) != 0)
    });
    let Some(binary) = read_words else {
        e.mem.set_u32(out, FORMAT_UNKNOWN);
        return out;
    };
    e.mem
        .set_u32(out, if binary { FORMAT_BINARY } else { FORMAT_OTHER });
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00dcca60, hk_tagfile_reader_detect_format(u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A stream whose slot `0x10` copies `data` (and answers how many bytes it had up to the
    /// request) and whose slots `0x1c` and `0x20` log `(slot, arguments)`.
    fn stream(e: &mut Engine, data: &[u8]) -> (u32, Rc<RefCell<Vec<(u32, Vec<u32>)>>>) {
        let log = Rc::new(RefCell::new(Vec::new()));
        for slot in [0x1cu32, 0x20] {
            let l = Rc::clone(&log);
            e.register_double(0x0b10_0000 + slot, move |_, a| {
                l.borrow_mut().push((slot, a[1..].to_vec()));
                Ret::default()
            });
        }
        let bytes = data.to_vec();
        e.register_double(0x0b10_0010, move |e, a| {
            let count = bytes.len().min(a[2] as usize);
            e.mem.write(a[1], &bytes[..count]);
            Ret {
                eax: count as u32,
                ..Ret::default()
            }
        });
        let vtable = e.mem.alloc(0x40);
        for slot in [0x10u32, 0x1c, 0x20] {
            e.mem.set_u32(vtable + slot, 0x0b10_0000 + slot);
        }
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        (object, log)
    }

    #[test]
    fn detect_format_tells_binary_tagfiles_from_other_streams() {
        let mut e = Engine::new();
        // The native magic: 0xcab00d1e and 0xd011face, then 8 more bytes.
        let mut magic = vec![];
        magic.extend(0xcab0_0d1eu32.to_le_bytes());
        magic.extend(0xd011_faceu32.to_le_bytes());
        magic.extend([0u8; 8]);
        let (binary, log) = stream(&mut e, &magic);
        let out = e.mem.alloc(4);
        // The 16 bytes are read to +0x4 of the exe's frame; the magic byte lands in the
        // `stream` argument's slot (+0x1c). The frame is the next stack local; its bytes stay on
        // the stack after the call until another local is taken there.
        let frame = e.next_stack_local(0x20);
        assert_eq!(e.call(0x00dc_ca60, &args![out, binary]).u32(), out);
        assert_eq!(e.mem.u32(out), 2);
        assert_eq!(e.mem.u32(frame + 4), 0xcab0_0d1e);
        assert_eq!(e.mem.u8(frame + 0x1c), 1);
        // Marked with 16, rewound once.
        assert_eq!(*log.borrow(), vec![(0x1c, vec![0x10]), (0x20, vec![])]);
        let (other, _) = stream(&mut e, &[7u8; 16]);
        assert_eq!(e.call(0x00dc_ca60, &args![out, other]).u32(), out);
        assert_eq!(e.mem.u32(out), 1);
    }

    #[test]
    fn detect_format_answers_0_for_a_short_stream_without_rewinding() {
        let mut e = Engine::new();
        let (short, log) = stream(&mut e, &[1u8; 15]);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 9);
        assert_eq!(e.call(0x00dc_ca60, &args![out, short]).u32(), out);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(*log.borrow(), vec![(0x1c, vec![0x10])]);
    }
}
