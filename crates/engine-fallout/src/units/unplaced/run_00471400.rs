//! `(unplaced)/run_00471400` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesfile::TESFile;

/// `_byteswap_uint64` (`00ec3bf0`, cdecl: the value as two words, low
/// first; the result in `EDX:EAX`).
const BYTESWAP_UINT64: u32 = 0x00ec_3bf0;

/// `TESFile` helpers this function calls (all `ECX` = file unless said):
/// the reset before reading (`00470bc0`), `TESFile::OpenTES_ov2`
/// (`00470ca0`; path, file name, two zero words; answers a bool),
/// `TESFile::TESRewind` (`00471f80`, one word), `TESFile::GetChunkData_ov3`
/// (`00472890`: buffer, size), the current chunk's size (`00401660`), the
/// `bMustEndianConvert` getter (`00401680`), the next chunk (`004726f0`,
/// answers a bool), the compile-index setter (`004731c0`, one word),
/// `TESFile::GetTESForm` (`00472660`) and `TESFile::NextForm` (`00472150`,
/// one word).
const FILE_RESET: u32 = 0x0047_0bc0;
const FILE_OPEN: u32 = 0x0047_0ca0;
const FILE_REWIND: u32 = 0x0047_1f80;
const FILE_READ_CHUNK: u32 = 0x0047_2890;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_IS_BIG_ENDIAN: u32 = 0x0040_1680;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_SET_COMPILE_INDEX: u32 = 0x0047_31c0;
const FILE_GET_TES_FORM: u32 = 0x0047_2660;
const FILE_NEXT_FORM: u32 = 0x0047_2150;
/// `listMasters` (`00464df0`) and `listMastersData` (`00470b90`) of the
/// file (`ECX` = file), and the list append both take (`00905820`, `ECX` =
/// list, the address of the item word).
const MASTERS_LIST: u32 = 0x0046_4df0;
const MASTERS_DATA_LIST: u32 = 0x0047_0b90;
const LIST_APPEND: u32 = 0x0090_5820;
/// The allocator (`00401000`, `cdecl`: size).
const ALLOCATE: u32 = 0x0040_1000;
/// The string setter (`004037f0`, `ECX` = string; text, length 0).
const STRING_SET: u32 = 0x0040_37f0;
/// Byte-swaps the three words of the file header (`005a6b10`, `ECX`).
const HEADER_SWAP: u32 = 0x005a_6b10;
/// The word the first record's tag is compared with (`01187014`).
const FILE_RECORD_TAG: u32 = 0x0118_7014;
/// The `BSFile`'s virtual at `+0x28` (`ECX` = the `BSFile`): its size.
const SLOT_BSFILE_SIZE: u32 = 0x28;
/// Chunk tags (four characters read as a little-endian word).
const TAG_HEDR: u32 = 0x5244_4548;
const TAG_CNAM: u32 = 0x4d41_4e43;
const TAG_SNAM: u32 = 0x4d41_4e53;
const TAG_MAST: u32 = 0x5453_414d;
const TAG_DATA: u32 = 0x4154_4144;
/// The most bytes read of the `CNAM` and `SNAM` texts.
const TEXT_CHUNK_LIMIT: u32 = 0x200;
/// The least `iNextFormID` the header keeps.
const LEAST_NEXT_FORM_ID: u32 = 0x800;
/// The record header flags copied into `m_Flags` (bits 0, 4 and 7).
const COPIED_FLAGS: [u32; 3] = [0x1, 0x10, 0x80];
/// `TES_RETURN_CODE` (Xbox PDB) values it answers: `TES_ERROR_NONE`,
/// `TES_ERROR_NO_FILE`, `TES_ERROR_INVALID_FILE`.
const TES_ERROR_NONE: u32 = 0;
const TES_ERROR_NO_FILE: u32 = 2;
const TES_ERROR_INVALID_FILE: u32 = 0xb;

/// Reads a `CNAM`/`SNAM` text into the string at `string`: a stack buffer
/// of the chunk's size (the exe's `_alloca`), at most 0x200 bytes read
/// into it, then the string set from it.
fn read_text_chunk(e: &mut Engine, this: Ptr<TESFile>, string: u32) {
    let size = e.call(FILE_CHUNK_SIZE, &args![this]).u32();
    e.with_stack(size, |e, buffer| {
        e.call(FILE_READ_CHUNK, &args![this, buffer, TEXT_CHUNK_LIMIT]);
        e.call(STRING_SET, &args![string, buffer, 0u32]);
    });
}

/// Appends the address of a word holding `block` to the list `list_getter`
/// answers for the file (`00905820`).
fn append_block(e: &mut Engine, this: Ptr<TESFile>, list_getter: u32, block: u32) {
    e.with_stack(4, |e, item| {
        e.mem.set_u32(item.addr(), block);
        let list = e.call(list_getter, &args![this]).u32();
        e.call(LIST_APPEND, &args![list, item]);
    });
}

// Translated from 00471400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00471400 - the /GS cookie check (00ec408c) is not translated; the CNAM/SNAM buffers from __alloca_probe_16 (00ec5ec0) are with_stack blocks of the chunk size (their 16-byte alignment padding is not modelled)
/// Reads a plugin file's header (unnamed in the map; `ECX` = `TESFile`, no
/// stack words; answers a `TES_RETURN_CODE`). Resets the file (`00470bc0`),
/// opens it (`OpenTES_ov2` with `m_Path`, `m_Filename`, 0, 0; `NO_FILE`
/// when that fails) or rewinds it when `m_pFile` is already set, stores
/// the `BSFile`'s size (its virtual at `+0x28`) in `m_filesize`, and
/// answers `INVALID_FILE` unless the first record's tag is the word at
/// `01187014`. Then, chunk by chunk until `004726f0` answers false:
/// - `HEDR`: the 12 bytes into `fileHeaderInfo` (byte-swapped by
///   `005a6b10` when the file needs it); `iNextFormID` is raised to 0x800
///   and its top byte replaced by `cCompileIndex`;
/// - `CNAM` / `SNAM`: the text into `cCreatedBy` / `cSummary`;
/// - `MAST`: a block of the chunk's size, read whole, appended to
///   `listMasters`, and `iMasterCount` counted up;
/// - `DATA`: an 8-byte block, read and byte-swapped when needed
///   ([`fn_004717f0`]), appended to `listMastersData`.
///
/// Afterwards bits 0, 4 and 7 of the record's flags are copied into
/// `m_Flags`, `004731c0` gets the top byte of `m_Flags`, records of kind 1
/// are skipped (`NextForm(1)`), [`fn_00471820`] stores whether the next
/// kind is 0 or 2, and the file is rewound (`TESRewind(1)`). Answers
/// `NONE`.
pub fn fn_00471400(e: &mut Engine, this: Ptr<TESFile>) -> u32 {
    let base = this.addr();
    e.call(FILE_RESET, &args![this]);
    if e.get(this, TESFile::m_pFile).addr() == 0 {
        let path = base + TESFile::m_Path.off;
        let name = base + TESFile::m_Filename.off;
        let opened = e
            .call(FILE_OPEN, &args![this, path, name, 0u32, 0u32])
            .bool();
        if !opened {
            return TES_ERROR_NO_FILE;
        }
    } else {
        e.call(FILE_REWIND, &args![this, 1u32]);
    }
    let file = e.get(this, TESFile::m_pFile).addr();
    let size = e.vcall(file, SLOT_BSFILE_SIZE, &args![]).u32();
    e.set(this, TESFile::m_filesize, size);
    let record = base + TESFile::m_currentform.off;
    if e.mem.u32(record) != e.global::<u32>(FILE_RECORD_TAG) {
        return TES_ERROR_INVALID_FILE;
    }
    loop {
        match e.get(this, TESFile::m_currentchunkID) {
            TAG_HEDR => {
                let header = base + TESFile::fileHeaderInfo.off;
                e.call(FILE_READ_CHUNK, &args![this, header, 0xcu32]);
                if e.call(FILE_IS_BIG_ENDIAN, &args![this]).bool() {
                    e.call(HEADER_SWAP, &args![header]);
                }
                // `FileHeader::iNextFormID` (Xbox PDB) +8.
                let next_id_at = header + 8;
                if e.mem.u32(next_id_at) < LEAST_NEXT_FORM_ID {
                    e.mem.set_u32(next_id_at, LEAST_NEXT_FORM_ID);
                }
                let index = e.get(this, TESFile::cCompileIndex) as u32;
                let next_id = e.mem.u32(next_id_at);
                e.mem
                    .set_u32(next_id_at, (index << 24) | (next_id & 0x00ff_ffff));
            }
            TAG_CNAM => read_text_chunk(e, this, base + TESFile::cCreatedBy.off),
            TAG_SNAM => read_text_chunk(e, this, base + TESFile::cSummary.off),
            TAG_MAST => {
                let size = e.call(FILE_CHUNK_SIZE, &args![this]).u32();
                let block = e.call(ALLOCATE, &args![size]).u32();
                e.call(FILE_READ_CHUNK, &args![this, block, 0u32]);
                append_block(e, this, MASTERS_LIST, block);
                let count = e.get(this, TESFile::iMasterCount).wrapping_add(1);
                e.set(this, TESFile::iMasterCount, count);
            }
            TAG_DATA => {
                let block = e.call(ALLOCATE, &args![8u32]).u32();
                e.call(FILE_READ_CHUNK, &args![this, block, 8u32]);
                if e.call(FILE_IS_BIG_ENDIAN, &args![this]).bool() {
                    fn_004717f0(e, Ptr::new(block), 0);
                }
                append_block(e, this, MASTERS_DATA_LIST, block);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![this]).bool() {
            break;
        }
    }
    // `FORM::flags` (Xbox PDB) +8 of `m_currentform`.
    let record_flags = e.mem.u32(record + 8);
    for bit in COPIED_FLAGS {
        let flags = e.get(this, TESFile::m_Flags);
        let flags = if record_flags & bit != 0 {
            flags | bit
        } else {
            flags & !bit
        };
        e.set(this, TESFile::m_Flags, flags);
    }
    let compile_index = e.get(this, TESFile::m_Flags) >> 24;
    e.call(FILE_SET_COMPILE_INDEX, &args![this, compile_index]);
    while e.call(FILE_GET_TES_FORM, &args![this]).u32() == 1 {
        e.call(FILE_NEXT_FORM, &args![this, 1u32]);
    }
    let kind = e.call(FILE_GET_TES_FORM, &args![this]).u32() as u8;
    fn_00471820(e, this, u8::from(kind == 2 || kind == 0));
    e.call(FILE_REWIND, &args![this, 1u32]);
    TES_ERROR_NONE
}

// Translated from 004717f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the eight bytes at `value` in place (`_byteswap_uint64`).
/// cdecl with two words, as its callers push them (`005e2870`, `00471400`):
/// the address and a flag byte the body never reads.
pub fn fn_004717f0(e: &mut Engine, value: Ptr, _unused_1: u32) {
    let low = e.mem.u32(value.addr());
    let high = e.mem.u32(value.addr() + 4);
    let swapped = e.call(BYTESWAP_UINT64, &args![low, high]);
    e.mem.set_u32(value.addr(), swapped.eax);
    e.mem.set_u32(value.addr() + 4, swapped.edx);
}

// Translated from 00471820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte argument in `TESFile::bHasGroups` (`+0x298`, Xbox PDB;
/// unnamed in the map; `ECX` = file, one stack word, `RET 4`). The file's
/// header reader (`00471400`) calls it after the header, with 1 when
/// `TESFile::GetTESForm` then answers 0 or 2 and 0 otherwise.
pub fn fn_00471820(e: &mut Engine, this: Ptr<TESFile>, has_groups: u8) {
    // The byte as given (`MOV byte`), not normalised to 0 or 1.
    e.mem
        .set_u8(this.addr() + TESFile::bHasGroups.off, has_groups);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00471400, fn_00471400(Ptr<TESFile>) -> u32),
        entry!(0x004717f0, fn_004717f0(Ptr, u32)),
        entry!(0x00471820, fn_00471820(Ptr<TESFile>, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_swapped_edx_eax() {
        let mut e = Engine::new();
        e.register(BYTESWAP_UINT64, |_, a| Ret {
            eax: a[0] ^ 0x1111_1111,
            edx: a[1] ^ 0x2222_2222,
            ..Ret::default()
        });
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 0x0102_0304);
        e.mem.set_u32(value + 4, 0x0506_0708);
        e.call_log = Some(vec![]);
        e.call(0x0047_17f0, &[value, 0]);
        assert_eq!(e.mem.u32(value), 0x0102_0304 ^ 0x1111_1111);
        assert_eq!(e.mem.u32(value + 4), 0x0506_0708 ^ 0x2222_2222);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(BYTESWAP_UINT64, vec![0x0102_0304, 0x0506_0708])]
        );
    }

    #[test]
    fn stores_the_has_groups_byte_only() {
        let mut e = Engine::new();
        let file = e.new_object::<TESFile>();
        e.mem.set_u32(file.addr() + 0x298, 0xaaaa_aa00);
        e.call(0x0047_1820, &args![file, 0x1234_5601u32]);
        assert_eq!(e.mem.u32(file.addr() + 0x298), 0xaaaa_aa01);
        e.call(0x0047_1820, &args![file, 2u32]);
        assert_eq!(e.mem.u32(file.addr() + 0x298), 0xaaaa_aa02);
        e.call(0x0047_1820, &args![file, 0u32]);
        assert_eq!(e.mem.u32(file.addr() + 0x298), 0xaaaa_aa00);
    }

    /// The tag `"TES4"` the test puts at `01187014`.
    const TES4: u32 = 0x3453_4554;
    const BSFILE: u32 = 0x0c00_0000;
    const BSFILE_VTABLE: u32 = 0x0c10_0000;
    const BSFILE_SIZE_FN: u32 = 0x0c20_0000;
    const MASTERS: u32 = 0x0d00_0000;
    const MASTERS_DATA: u32 = 0x0d00_0100;
    const BLOCKS: u32 = 0x0e00_0000;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// An engine with doubles for every callee: `chunks` are the chunk tags
    /// in order (the first is current when the function starts), `kinds`
    /// the answers of `GetTESForm` in order, `opens` what `OpenTES_ov2`
    /// answers.
    fn header_engine(chunks: Vec<u32>, kinds: Vec<u32>, opens: bool) -> (Engine, Ptr<TESFile>) {
        let mut e = Engine::new();
        e.map(FILE_RECORD_TAG & !0xfff, 0x1000);
        e.set_global(FILE_RECORD_TAG, TES4);
        e.put_vtable(
            BSFILE_VTABLE,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, BSFILE_SIZE_FN],
        );
        e.map(BSFILE, 0x1000);
        e.mem.set_u32(BSFILE, BSFILE_VTABLE);
        e.map(BLOCKS, 0x1000);
        e.register(BSFILE_SIZE_FN, |_, _| ret(0x1234));
        for addr in [
            FILE_RESET,
            FILE_REWIND,
            FILE_READ_CHUNK,
            FILE_SET_COMPILE_INDEX,
            FILE_NEXT_FORM,
            LIST_APPEND,
            STRING_SET,
            HEADER_SWAP,
            BYTESWAP_UINT64,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register_double(FILE_OPEN, move |e, a| {
            if opens {
                e.mem.set_u32(a[0] + 0x10, BSFILE);
            }
            ret(u32::from(opens))
        });
        e.register(FILE_CHUNK_SIZE, |_, _| ret(0x30));
        e.register(FILE_IS_BIG_ENDIAN, |_, _| ret(1));
        e.register(MASTERS_LIST, |_, _| ret(MASTERS));
        e.register(MASTERS_DATA_LIST, |_, _| ret(MASTERS_DATA));
        let mut next_block = BLOCKS;
        e.register_double(ALLOCATE, move |_, _| {
            next_block += 0x100;
            ret(next_block)
        });
        let mut rest = chunks.clone().into_iter().skip(1);
        e.register_double(FILE_NEXT_CHUNK, move |e, a| match rest.next() {
            Some(tag) => {
                e.mem.set_u32(a[0] + 0x258, tag);
                ret(1)
            }
            None => ret(0),
        });
        let mut kinds = kinds.into_iter();
        e.register_double(FILE_GET_TES_FORM, move |_, _| ret(kinds.next().unwrap()));
        let file = e.new_object::<TESFile>();
        e.mem.set_u32(file.addr() + 0x240, TES4);
        e.mem.set_u32(file.addr() + 0x258, chunks[0]);
        e.call_log = Some(vec![]);
        (e, file)
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn header_answers_no_file_when_the_open_fails() {
        let (mut e, file) = header_engine(vec![TAG_HEDR], vec![], false);
        let back = e.call(0x0047_1400, &args![file]).u32();
        assert_eq!(back, TES_ERROR_NO_FILE);
        let base = file.addr();
        assert_eq!(calls_to(&e, FILE_RESET), vec![vec![base]]);
        assert_eq!(
            calls_to(&e, FILE_OPEN),
            vec![vec![base, base + 0x124, base + 0x20, 0, 0]]
        );
        assert!(calls_to(&e, FILE_READ_CHUNK).is_empty());
        assert_eq!(e.mem.u32(base + 0x260), 0);
    }

    #[test]
    fn header_answers_invalid_file_for_another_first_record() {
        let (mut e, file) = header_engine(vec![TAG_HEDR], vec![], true);
        e.mem.set_u32(file.addr() + 0x240, 0x5550_5247);
        let back = e.call(0x0047_1400, &args![file]).u32();
        assert_eq!(back, TES_ERROR_INVALID_FILE);
        assert_eq!(e.mem.u32(file.addr() + 0x260), 0x1234);
        assert!(calls_to(&e, FILE_READ_CHUNK).is_empty());
        assert!(calls_to(&e, FILE_REWIND).is_empty());
    }

    #[test]
    fn header_reads_every_chunk_kind_and_the_flags() {
        let chunks = vec![
            TAG_HEDR,
            TAG_CNAM,
            TAG_SNAM,
            TAG_MAST,
            TAG_DATA,
            0x4d41_4e58,
        ];
        let (mut e, file) = header_engine(chunks, vec![1, 1, 0, 2], true);
        let base = file.addr();
        e.mem.set_u32(base + 0x10, BSFILE);
        e.mem.set_u32(base + 0x3e4, 0x0000_0100);
        e.mem.set_u8(base + 0x40c, 3);
        e.mem.set_u32(base + 0x3fc, 4);
        e.mem.set_u32(base + 0x248, 0x0000_0011);
        e.mem.set_u32(base + 0x3e8, 0x0500_0080);
        let back = e.call(0x0047_1400, &args![file]).u32();
        assert_eq!(back, TES_ERROR_NONE);
        // Already open: rewound at the start and at the end, not opened.
        assert!(calls_to(&e, FILE_OPEN).is_empty());
        assert_eq!(
            calls_to(&e, FILE_REWIND),
            vec![vec![base, 1], vec![base, 1]]
        );
        assert_eq!(e.mem.u32(base + 0x260), 0x1234);
        // HEDR: read, swapped, the next id raised and given the index byte.
        assert_eq!(calls_to(&e, HEADER_SWAP), vec![vec![base + 0x3dc]]);
        assert_eq!(e.mem.u32(base + 0x3e4), 0x0300_0800);
        // CNAM and SNAM: stack buffers, at most 0x200 bytes, then the strings.
        let reads = calls_to(&e, FILE_READ_CHUNK);
        assert_eq!(reads.len(), 5);
        assert_eq!(reads[0], vec![base, base + 0x3dc, 0xc]);
        assert_eq!(reads[1][2], 0x200);
        assert_eq!(reads[2][2], 0x200);
        let strings = calls_to(&e, STRING_SET);
        assert_eq!(strings.len(), 2);
        assert_eq!(strings[0][0], base + 0x410);
        assert_eq!(strings[0][1], reads[1][1]);
        assert_eq!(strings[0][2], 0);
        assert_eq!(strings[1][0], base + 0x418);
        // MAST: a block of the chunk size, read whole, appended, counted.
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x30], vec![8]]);
        assert_eq!(reads[3], vec![base, BLOCKS + 0x100, 0]);
        assert_eq!(e.mem.u32(base + 0x3fc), 5);
        // DATA: eight bytes, swapped, appended.
        assert_eq!(reads[4], vec![base, BLOCKS + 0x200, 8]);
        assert_eq!(calls_to(&e, BYTESWAP_UINT64).len(), 1);
        let appends = calls_to(&e, LIST_APPEND);
        assert_eq!(appends.len(), 2);
        assert_eq!(appends[0][0], MASTERS);
        assert_eq!(appends[1][0], MASTERS_DATA);
        // The flags: bits 0 and 4 set, bit 7 cleared; the top byte passed on.
        assert_eq!(e.mem.u32(base + 0x3e8), 0x0500_0011);
        assert_eq!(calls_to(&e, FILE_SET_COMPILE_INDEX), vec![vec![base, 5]]);
        // Kind 1 skipped twice; the next kind (2) gives bHasGroups 1.
        assert_eq!(
            calls_to(&e, FILE_NEXT_FORM),
            vec![vec![base, 1], vec![base, 1]]
        );
        assert_eq!(e.mem.u8(base + 0x298), 1);
    }

    #[test]
    fn header_keeps_a_large_next_id_and_clears_has_groups_for_other_kinds() {
        let (mut e, file) = header_engine(vec![TAG_HEDR], vec![0, 4], true);
        let base = file.addr();
        e.register(FILE_IS_BIG_ENDIAN, |_, _| ret(0));
        e.mem.set_u32(base + 0x3e4, 0x1234_5678);
        e.mem.set_u8(base + 0x40c, 0xab);
        e.mem.set_u8(base + 0x298, 1);
        e.mem.set_u32(base + 0x3e8, 0x0000_0091);
        let back = e.call(0x0047_1400, &args![file]).u32();
        assert_eq!(back, TES_ERROR_NONE);
        assert!(calls_to(&e, HEADER_SWAP).is_empty());
        assert_eq!(e.mem.u32(base + 0x3e4), 0xab34_5678);
        assert_eq!(e.mem.u32(base + 0x3e8), 0);
        assert!(calls_to(&e, FILE_NEXT_FORM).is_empty());
        assert_eq!(e.mem.u8(base + 0x298), 0);
    }
}
