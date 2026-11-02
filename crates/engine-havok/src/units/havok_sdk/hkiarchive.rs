//! `Havok SDK/hkiarchive.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkIArchive` wraps a `hkStreamReader` (the pointer at `+8`; the Xbox PDB has no record of
//! the archive class, so the layout is described by offsets): its virtual slot `0xc` is
//! `isOk(flag)` and slot `0x10` is `read(buffer, size)`. The byte `+0xc` of the archive says
//! whether the data has to be byte-swapped.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkIArchive` (named by the engine map), as far as these functions know it.
    pub struct HkIArchive: 0x10 {
        /// The `hkStreamReader` the archive reads from.
        0x08 reader: Ptr,
        /// Whether the values read are byte-swapped.
        0x0c swapBytes: bool,
    }
}

// Translated from 00dd8fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkIArchive::readArrayGeneric` (Xbox PDB), thiscall, `RET 0xc`: reads `element_size *
/// count` bytes into `buffer` through the reader's slot `0x10` and, when the archive swaps
/// bytes, reverses the bytes of each of the `count` elements (signed count; element sizes 2,
/// 4 and 8 are swapped, any other size is left as read).
pub fn hk_i_archive_read_array_generic(
    e: &mut Engine,
    this: Ptr<HkIArchive>,
    buffer: u32,
    element_size: u32,
    count: i32,
) {
    let total = element_size.wrapping_mul(count as u32);
    let reader = e.get(this, HkIArchive::reader).addr();
    e.vcall(reader, 0x10, &args![buffer, total]);
    if !e.get(this, HkIArchive::swapBytes) {
        return;
    }
    if !matches!(element_size, 2 | 4 | 8) {
        return;
    }
    for index in 0..count.max(0) as u32 {
        let element = buffer.wrapping_add(index.wrapping_mul(element_size));
        let mut bytes = e.mem.bytes(element, element_size);
        bytes.reverse();
        e.mem.write(element, &bytes);
    }
}

// Translated from 00dd90d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkIArchive::readRaw` (Xbox PDB), thiscall: the exe jumps to the reader's slot `0x10`
/// (`read`) with the two stack words (the buffer and the byte count) passed through; returns
/// what it returns.
pub fn hk_i_archive_read_raw(e: &mut Engine, this: Ptr<HkIArchive>, buffer: u32, size: u32) -> u32 {
    let reader = e.get(this, HkIArchive::reader).addr();
    e.vcall(reader, 0x10, &args![buffer, size]).u32()
}

// Translated from 00dd90e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkIArchive::isOk` (Xbox PDB), thiscall, `RET 4`: asks the reader's slot `0xc` (`isOk`)
/// to store its answer through `flag`; returns `flag`.
pub fn hk_i_archive_is_ok(e: &mut Engine, this: Ptr<HkIArchive>, flag: Ptr) -> Ptr {
    let reader = e.get(this, HkIArchive::reader).addr();
    e.vcall(reader, 0xc, &args![flag]);
    flag
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00dd8fe0,
            hk_i_archive_read_array_generic(Ptr<HkIArchive>, u32, u32, i32)
        ),
        entry!(
            0x00dd90d0,
            hk_i_archive_read_raw(Ptr<HkIArchive>, u32, u32) -> u32
        ),
        entry!(0x00dd90e0, hk_i_archive_is_ok(Ptr<HkIArchive>, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const READ_DOUBLE: u32 = 0x7100_0010;
    const IS_OK_DOUBLE: u32 = 0x7100_000c;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(target, _)| *target == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An archive over a reader whose `read` answers 7 and `isOk` stores 1.
    fn archive(e: &mut Engine, swap: bool) -> u32 {
        e.put_vtable(0x7100_0000, &[0, 0, 0, IS_OK_DOUBLE, READ_DOUBLE]);
        e.register(READ_DOUBLE, |_, _| Ret {
            eax: 7,
            ..Ret::default()
        });
        e.register(IS_OK_DOUBLE, |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        let reader = e.mem.alloc(8);
        e.mem.set_u32(reader, 0x7100_0000);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, reader);
        e.mem.set_u8(this + 0xc, u8::from(swap));
        e.call_log = Some(vec![]);
        this
    }

    #[test]
    fn read_array_generic_swaps_each_element_when_asked() {
        for (size, swapped) in [
            (2u32, [2u8, 1, 4, 3, 5, 6]),
            (4, [4, 3, 2, 1, 8, 7]),
            (8, [8, 7, 6, 5, 4, 3]),
        ] {
            let mut e = Engine::new();
            let this = archive(&mut e, true);
            let buffer = e.mem.alloc(0x40);
            e.mem.write(
                buffer,
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            );
            e.call(0x00dd_8fe0, &args![this, buffer, size, 2u32]);
            let reader = e.mem.u32(this + 8);
            assert_eq!(
                calls_to(&e, READ_DOUBLE),
                vec![vec![reader, buffer, size * 2]]
            );
            assert_eq!(e.mem.bytes(buffer, 6), swapped.to_vec(), "size {size}");
        }
    }

    #[test]
    fn read_array_generic_leaves_other_sizes_and_unswapped_archives_alone() {
        let mut e = Engine::new();
        let this = archive(&mut e, true);
        let buffer = e.mem.alloc(0x40);
        e.mem.write(buffer, &[1, 2, 3, 4, 5, 6, 7, 8]);
        // A size that is not 2, 4 or 8.
        e.call(0x00dd_8fe0, &args![this, buffer, 3u32, 2u32]);
        assert_eq!(e.mem.bytes(buffer, 6), vec![1, 2, 3, 4, 5, 6]);
        // A count of zero or less swaps nothing.
        e.call(0x00dd_8fe0, &args![this, buffer, 4u32, 0u32]);
        e.call(0x00dd_8fe0, &args![this, buffer, 4u32, -1i32]);
        assert_eq!(e.mem.bytes(buffer, 4), vec![1, 2, 3, 4]);
        // An archive without the swap flag.
        let plain = archive(&mut e, false);
        e.call(0x00dd_8fe0, &args![plain, buffer, 2u32, 2u32]);
        assert_eq!(e.mem.bytes(buffer, 4), vec![1, 2, 3, 4]);
    }

    #[test]
    fn read_raw_passes_the_words_to_the_reader() {
        let mut e = Engine::new();
        let this = archive(&mut e, false);
        let reader = e.mem.u32(this + 8);
        assert_eq!(e.call(0x00dd_90d0, &args![this, 0x5000u32, 12u32]).u32(), 7);
        assert_eq!(calls_to(&e, READ_DOUBLE), vec![vec![reader, 0x5000, 12]]);
    }

    #[test]
    fn is_ok_asks_the_reader_and_returns_the_flag_pointer() {
        let mut e = Engine::new();
        let this = archive(&mut e, false);
        let flag = e.mem.alloc(4);
        assert_eq!(e.call(0x00dd_90e0, &args![this, flag]).u32(), flag);
        assert_eq!(e.mem.u8(flag), 1);
        let reader = e.mem.u32(this + 8);
        assert_eq!(calls_to(&e, IS_OK_DOUBLE), vec![vec![reader, flag]]);
    }
}
