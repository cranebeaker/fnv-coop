//! `BSSystem/bssystemfile_xenon.cpp` (Xbox PDB source unit), subsystem `BSSystem`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md). Each `Do...`
//! keeps the answer of the file call it wraps in the word at +0 and answers that word.

#[allow(unused_imports)]
use crate::prelude::*;

/// The wrapped calls (`thiscall` on the same object): the size (`00aff000`, one word), the
/// read (`00afecd0`), the write (`00afede0`) and the seek (`00afeef0`), four words each.
const GET_SIZE: u32 = 0x00af_f000;
const READ: u32 = 0x00af_ecd0;
const WRITE: u32 = 0x00af_ede0;
const SEEK: u32 = 0x00af_eef0;

/// The four-word wrappers' shared body: the call's answer stored at +0 and read back.
fn keep(e: &mut Engine, this: u32, answer: u32) -> u32 {
    e.mem.set_u32(this, answer);
    e.mem.u32(this)
}

// Translated from 0065c290 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSSystemFile::DoGetSize` (engine map; `thiscall`, one word: the address of the 64-bit
/// size, `RET 4`): `00aff000(size)`, its answer kept at +0 and answered.
pub fn bs_system_file_do_get_size(e: &mut Engine, this: u32, size: u32) -> u32 {
    let answer = e.call(GET_SIZE, &args![this, size]).u32();
    keep(e, this, answer)
}

// Translated from 008526d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSSystemFile::DoRead` (engine map; `thiscall`, four words passed on in order, `RET 0x10`):
/// `00afecd0` with them, its answer kept at +0 and answered.
pub fn bs_system_file_do_read(e: &mut Engine, this: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    let answer = e.call(READ, &args![this, a, b, c, d]).u32();
    keep(e, this, answer)
}

// Translated from 00856350 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSSystemFile::DoWrite` (engine map; `thiscall`, four words passed on in order,
/// `RET 0x10`): `00afede0` with them, its answer kept at +0 and answered.
pub fn bs_system_file_do_write(e: &mut Engine, this: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    let answer = e.call(WRITE, &args![this, a, b, c, d]).u32();
    keep(e, this, answer)
}

// Translated from 008785e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSSystemFile::DoSeek` (engine map; `thiscall`, four words passed on in order, `RET 0x10`):
/// `00afeef0` with them, its answer kept at +0 and answered.
pub fn bs_system_file_do_seek(e: &mut Engine, this: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    let answer = e.call(SEEK, &args![this, a, b, c, d]).u32();
    keep(e, this, answer)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_c290, bs_system_file_do_get_size(u32, u32) -> u32),
        entry!(
            0x0085_26d0,
            bs_system_file_do_read(u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x0085_6350,
            bs_system_file_do_write(u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x0087_85e0,
            bs_system_file_do_seek(u32, u32, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_call_passes_its_words_and_keeps_the_answer() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        for (addr, answer) in [(GET_SIZE, 4u32), (READ, 5), (WRITE, 6), (SEEK, 7)] {
            e.register_double(addr, move |_, _| answer.into_ret());
        }
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x0065_c290, &args![this, 0x10u32]).u32(), 4);
        assert_eq!(e.mem.u32(this), 4);
        let four = args![this, 1u32, 2u32, 3u32, 4u32];
        assert_eq!(e.call(0x0085_26d0, &four).u32(), 5);
        assert_eq!(e.mem.u32(this), 5);
        assert_eq!(e.call(0x0085_6350, &four).u32(), 6);
        assert_eq!(e.call(0x0087_85e0, &four).u32(), 7);
        assert_eq!(e.mem.u32(this), 7);
        let log = e.call_log.take().unwrap();
        for addr in [READ, WRITE, SEEK] {
            let calls: Vec<_> = log.iter().filter(|c| c.0 == addr).collect();
            assert_eq!(calls.len(), 1);
            assert_eq!(calls[0].1, vec![this, 1, 2, 3, 4]);
        }
        let sizes: Vec<_> = log.iter().filter(|c| c.0 == GET_SIZE).collect();
        assert_eq!(sizes[0].1, vec![this, 0x10]);
    }
}
