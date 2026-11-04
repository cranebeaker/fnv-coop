//! `(unplaced)/run_004726f0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesfile::{Form, TESFile};

/// `TESFile`: true when the current record is compressed (`00472100`).
const IS_COMPRESSED: u32 = 0x0047_2100;
/// `TESFile`: the size of the decompressed form buffer (`004727d0`; engine
/// map name `HighProcess::GetPostAnimationActions`, a folded body that
/// answers the word at `+0x424`).
const DECOMPRESSED_FORM_SIZE: u32 = 0x0047_27d0;
/// `TESFile`: the size of the record header that precedes the chunks
/// (`00471840`; engine map name `bhkLiquidAction::QCinfoSize`, folded).
const FORM_HEADER_SIZE: u32 = 0x0047_1840;
/// `TESFile`: clears the current chunk id and size (`004720d0`).
const CLEAR_CURRENT_CHUNK: u32 = 0x0047_20d0;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The `whence` word the seek passes (global at `010a2480`).
const SEEK_MODE: u32 = 0x010a_2480;
/// The `BSFile` virtual slot the seek calls: `(offset, whence)`.
const FILE_SEEK_SLOT: u32 = 0x14;

// Translated from 004726f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves a `TESFile` to the next chunk of the current record. The cursor
/// `m_formoffset` moves past the current chunk (its six-byte header and
/// `m_actualChunkSize`). While it is still below the end of the record
/// (`m_currentform.length`, or the decompressed size for a compressed
/// record), it seeks the `BSFile` there (unless compressed), zeroes
/// `m_chunkoffset`, clears the current chunk and reads the next chunk header
/// through `TESFile::GetTESChunk`, answering true; otherwise it only clears
/// the current chunk and answers false. The engine map gives it no name.
pub fn fn_004726f0(e: &mut Engine, this: Ptr<TESFile>) -> bool {
    let form = this.at(TESFile::m_currentform);
    let mut end = e.get(form, Form::length);
    if e.call(IS_COMPRESSED, &args![this]).bool() {
        end = e.call(DECOMPRESSED_FORM_SIZE, &args![this]).u32();
    }
    let offset = e
        .get(this, TESFile::m_actualChunkSize)
        .wrapping_add(e.get(this, TESFile::m_formoffset))
        .wrapping_add(6);
    e.set(this, TESFile::m_formoffset, offset);
    if e.get(this, TESFile::m_formoffset) >= end {
        e.call(CLEAR_CURRENT_CHUNK, &args![this]);
        return false;
    }
    let header_size = e.call(FORM_HEADER_SIZE, &args![this]).u32();
    let position = e
        .get(this, TESFile::m_fileoffset)
        .wrapping_add(header_size)
        .wrapping_add(e.get(this, TESFile::m_formoffset));
    if !e.call(IS_COMPRESSED, &args![this]).bool() {
        let whence: u32 = e.global(SEEK_MODE);
        let file = e.get(this, TESFile::m_pFile);
        e.vcall(file.addr(), FILE_SEEK_SLOT, &args![position, whence]);
    }
    e.set(this, TESFile::m_chunkoffset, 0);
    e.call(CLEAR_CURRENT_CHUNK, &args![this]);
    e.call(GET_TES_CHUNK, &args![this]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004726f0, fn_004726f0(Ptr<TESFile>) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE_VTABLE: u32 = 0x0170_0000;
    const SEEK: u32 = 0x0170_1000;

    type CallLog = Vec<(u32, Vec<u32>)>;

    /// A file at record offset 0x1000 with a 0x40-byte record, the cursor at
    /// 0x10 and a 0x0A-byte chunk; `compressed` answers the compression test.
    fn setup(compressed: bool) -> (Engine, Ptr<TESFile>) {
        let mut e = Engine::new();
        let file = e.new_object::<TESFile>();
        let bs_file: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.put_vtable(FILE_VTABLE, &[0, 0, 0, 0, 0, SEEK]);
        e.mem.set_u32(bs_file.addr(), FILE_VTABLE);
        e.set(file, TESFile::m_pFile, bs_file);
        e.set(file.at(TESFile::m_currentform), Form::length, 0x40);
        e.set(file, TESFile::m_fileoffset, 0x1000);
        e.set(file, TESFile::m_formoffset, 0x10);
        e.set(file, TESFile::m_actualChunkSize, 0x0a);
        e.set(file, TESFile::m_chunkoffset, 7);
        e.map(SEEK_MODE, 4);
        e.set_global::<u32>(SEEK_MODE, 2);
        if compressed {
            e.register(IS_COMPRESSED, |_, _| true.into_ret());
        } else {
            e.register(IS_COMPRESSED, |_, _| false.into_ret());
        }
        e.register(DECOMPRESSED_FORM_SIZE, |_, _| 0x20u32.into_ret());
        e.register(FORM_HEADER_SIZE, |_, _| 0x18u32.into_ret());
        e.register(CLEAR_CURRENT_CHUNK, |_, _| Ret::default());
        e.register(GET_TES_CHUNK, |_, _| 0u32.into_ret());
        e.register(SEEK, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, file)
    }

    fn calls_to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn seeks_to_the_next_chunk_of_a_plain_record() {
        let (mut e, file) = setup(false);
        assert!(e.call(0x0047_26f0, &args![file]).bool());
        // 0x10 + 6 + 0x0a.
        assert_eq!(e.get(file, TESFile::m_formoffset), 0x20);
        assert_eq!(e.get(file, TESFile::m_chunkoffset), 0);
        let log = e.call_log.take().unwrap();
        let bs_file = e.get(file, TESFile::m_pFile).addr();
        assert_eq!(
            calls_to(&log, SEEK),
            vec![vec![bs_file, 0x1000 + 0x18 + 0x20, 2]]
        );
        assert_eq!(calls_to(&log, CLEAR_CURRENT_CHUNK).len(), 1);
        assert_eq!(calls_to(&log, GET_TES_CHUNK), vec![vec![file.addr()]]);
        assert!(calls_to(&log, DECOMPRESSED_FORM_SIZE).is_empty());
    }

    #[test]
    fn a_compressed_record_ends_at_its_decompressed_size_and_does_not_seek() {
        let (mut e, file) = setup(true);
        // The decompressed size 0x20 is reached: no next chunk.
        assert!(!e.call(0x0047_26f0, &args![file]).bool());
        assert_eq!(e.get(file, TESFile::m_formoffset), 0x20);
        assert_eq!(e.get(file, TESFile::m_chunkoffset), 7);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CLEAR_CURRENT_CHUNK).len(), 1);
        assert!(calls_to(&log, GET_TES_CHUNK).is_empty());
        assert!(calls_to(&log, SEEK).is_empty());

        // Below it, the next chunk is read without a seek.
        let (mut e, file) = setup(true);
        e.set(file, TESFile::m_actualChunkSize, 2);
        assert!(e.call(0x0047_26f0, &args![file]).bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SEEK).is_empty());
        assert_eq!(calls_to(&log, GET_TES_CHUNK).len(), 1);
    }

    #[test]
    fn the_record_end_stops_a_plain_record() {
        let (mut e, file) = setup(false);
        e.set(file, TESFile::m_actualChunkSize, 0x2a);
        // 0x10 + 6 + 0x2a = 0x40: not below the record length.
        assert!(!e.call(0x0047_26f0, &args![file]).bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SEEK).is_empty());
        assert!(calls_to(&log, FORM_HEADER_SIZE).is_empty());
    }
}
