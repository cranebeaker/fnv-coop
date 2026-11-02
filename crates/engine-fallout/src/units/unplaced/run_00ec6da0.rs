//! `(unplaced)/run_00ec6da0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two functions of the statically linked C runtime (LIBCMT's assembly
//! `strcmp` and `strrchr`; the engine map names neither, the bodies are
//! those routines).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ec6e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `strrchr(string, character)` (LIBCMT, `cdecl`): the last occurrence of
/// the character's low byte in the string, its terminator included (so
/// `'\0'` finds the terminator), or null. The assembly measures the string
/// (`REPNE SCASB`), then scans backwards from the terminator over
/// `length + 1` bytes and checks the byte where the scan stopped.
pub fn strrchr(e: &mut Engine, string: Ptr, character: u32) -> Ptr {
    let wanted = character as u8;
    let mut cursor = string.addr();
    while e.mem.u8(cursor) != 0 {
        cursor = cursor.wrapping_add(1);
    }
    let mut count = cursor.wrapping_sub(string.addr()).wrapping_add(1);
    // `STD; REPNE SCASB`: compare, step back, until a match or ECX is 0.
    while count != 0 {
        let byte = e.mem.u8(cursor);
        cursor = cursor.wrapping_sub(1);
        count -= 1;
        if byte == wanted {
            break;
        }
    }
    let stop = cursor.wrapping_add(1);
    if e.mem.u8(stop) == wanted {
        Ptr::new(stop)
    } else {
        Ptr::NULL
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ec6e30, strrchr(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A copy of `text` (with its terminator) at `offset` bytes past an
    /// aligned block.
    fn put(e: &mut Engine, text: &[u8], offset: u32) -> u32 {
        let block = e.mem.alloc(64);
        let at = block + offset;
        e.mem.set_cstr(at, text);
        at
    }

    #[test]
    fn strcmp_orders_by_unsigned_bytes_at_every_alignment() {
        let mut e = Engine::new();
        let cases: [(&[u8], &[u8], i32); 7] = [
            (b"abcdefgh", b"abcdefgh", 0),
            (b"abcdefgh", b"abcdefgi", -1),
            (b"abcdefgi", b"abcdefgh", 1),
            (b"abc", b"abcd", -1),
            (b"abcd", b"abc", 1),
            (b"", b"", 0),
            (b"a\x80", b"a\x01", 1),
        ];
        for offset in 0..4 {
            for (left, right, want) in cases {
                let l = put(&mut e, left, offset);
                let r = put(&mut e, right, 1);
                assert_eq!(
                    e.call(0x00ec_6da0, &args![l, r]).i32(),
                    want,
                    "{left:?} {right:?} at +{offset}"
                );
            }
        }
    }

    #[test]
    fn strrchr_finds_the_last_occurrence() {
        let mut e = Engine::new();
        let s = put(&mut e, b"a/b/c", 0);
        assert_eq!(e.call(0x00ec_6e30, &args![s, b'/' as u32]).u32(), s + 3);
        assert_eq!(e.call(0x00ec_6e30, &args![s, b'a' as u32]).u32(), s);
        assert_eq!(e.call(0x00ec_6e30, &args![s, b'x' as u32]).u32(), 0);
        // The terminator, and only the low byte of the character counts.
        assert_eq!(e.call(0x00ec_6e30, &args![s, 0u32]).u32(), s + 5);
        assert_eq!(
            e.call(0x00ec_6e30, &args![s, 0x100u32 | b'c' as u32]).u32(),
            s + 4
        );
        let empty = put(&mut e, b"", 0);
        assert_eq!(e.call(0x00ec_6e30, &args![empty, b'a' as u32]).u32(), 0);
    }
}
