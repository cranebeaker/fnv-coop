//! `BSSystem/run_00affd80` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The smallest buffer size the code allows for (0x4000 bytes).
const MIN_BUFFER_SIZE: u32 = 0x4000;
/// The file's buffer size (`NiFile::m_uiBufferAllocSize` in the Xbox PDB) +0x10.
const FILE_BUFFER_SIZE: u32 = 0x10;
/// The file's buffer (`NiFile::m_pBuffer` in the Xbox PDB) +0x20.
const FILE_BUFFER: u32 = 0x20;
/// A word of the file object at +0x150 that the code clears after a new buffer (its name is
/// not confirmed on PC).
const FILE_WORD_150: u32 = 0x150;
/// The virtual at +0x28 of the file, `__thiscall` without arguments: a size the minimum is
/// applied to (the slot's name is not confirmed on PC).
const SIZE_SLOT: u32 = 0x28;
/// The thread block's word at `+0x2b4` (the allocation context), set to 0x17 while the buffer
/// is replaced.
const TLS_CONTEXT: u32 = 0x2b4;
const BUFFER_CONTEXT: u32 = 0x17;

// Callees outside this unit.
/// `00aa16d0` (`__thiscall`, two words; `NiSystem`, unnamed): called as `(this, 0, seek mode)`.
const FILE_SEEK_00AA16D0: u32 = 0x00aa_16d0;
/// `00aa15a0` (`__thiscall`, no words; `NiSystem`, unnamed): writes out or drops the buffered
/// bytes.
const FILE_FLUSH_00AA15A0: u32 = 0x00aa_15a0;
/// The seek mode word passed to `00aa16d0` (`.rdata`).
const SEEK_MODE: u32 = 0x010a_2480;
/// The fallback buffer size when the first allocation fails (`.data`).
const FALLBACK_BUFFER_SIZE: u32 = 0x011a_cff8;
/// `MemoryManager::Allocate` (`00aa3e40`) and `Deallocate` (`00aa4060`), `__thiscall` on the
/// memory manager at `011f6238`, one word.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const ALLOCATE: u32 = 0x00aa_3e40;
const DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00affd80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00affd80 - its FS:[0] exception frame (the unwind state that restores the allocation context) is not translated
/// Unnamed (`__thiscall`, one word; called by `OpenMasterArchives` and `00af7eb0`): sets a
/// file's buffer size. The size asked for is capped at the larger of 0x4000 and the file's
/// virtual at +0x28; when it equals the current size (+0x10) nothing changes and the result
/// is true. Otherwise, with the thread's allocation context (TLS +0x2b4) set to 0x17: calls
/// `00aa16d0(this, 0, [010a2480])` and `00aa15a0(this)`, frees the old buffer (+0x20),
/// stores the new size and allocates the buffer; when that fails, the size becomes the word
/// at `011acff8`, that much is allocated instead and the result is false. Clears +0x150 and
/// puts the allocation context back.
pub fn fn_00affd80(e: &mut Engine, this: Ptr, size: u32) -> bool {
    let mut result = true;
    let limit = e.vcall(this.addr(), SIZE_SLOT, &args![]).u32();
    let limit = if limit < MIN_BUFFER_SIZE {
        MIN_BUFFER_SIZE
    } else {
        limit
    };
    let size = if size > limit { limit } else { size };
    if size == e.mem.u32(this.addr() + FILE_BUFFER_SIZE) {
        return result;
    }
    let context = e.tls() + TLS_CONTEXT;
    let saved = e.mem.u32(context);
    e.mem.set_u32(context, BUFFER_CONTEXT);
    let seek_mode = e.mem.u32(SEEK_MODE);
    e.call(FILE_SEEK_00AA16D0, &args![this, 0u32, seek_mode]);
    e.call(FILE_FLUSH_00AA15A0, &args![this]);
    let old = e.mem.u32(this.addr() + FILE_BUFFER);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, old]);
    e.mem.set_u32(this.addr() + FILE_BUFFER_SIZE, size);
    let buffer = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    e.mem.set_u32(this.addr() + FILE_BUFFER, buffer);
    if e.mem.u32(this.addr() + FILE_BUFFER) == 0 {
        let fallback = e.mem.u32(FALLBACK_BUFFER_SIZE);
        e.mem.set_u32(this.addr() + FILE_BUFFER_SIZE, fallback);
        let fallback = e.mem.u32(this.addr() + FILE_BUFFER_SIZE);
        let buffer = e.call(ALLOCATE, &args![MEMORY_MANAGER, fallback]).u32();
        e.mem.set_u32(this.addr() + FILE_BUFFER, buffer);
        result = false;
    }
    e.mem.set_u32(this.addr() + FILE_WORD_150, 0);
    let context = e.tls() + TLS_CONTEXT;
    e.mem.set_u32(context, saved);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00affd80, fn_00affd80(Ptr, u32) -> bool)]
}

#[cfg(test)]
mod tests {
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

    const SIZE_DOUBLE: u32 = 0x0070_0000;

    /// A file whose virtual at +0x28 answers `limit`, with a buffer of size 0x8000 at 0x1234.
    fn file(e: &mut Engine, limit: u32) -> u32 {
        let this = e.mem.alloc(0x200);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + SIZE_SLOT, SIZE_DOUBLE);
        e.register_double(SIZE_DOUBLE, move |_, _| Ret {
            eax: limit,
            ..Ret::default()
        });
        e.mem.set_u32(this + FILE_BUFFER_SIZE, 0x8000);
        e.mem.set_u32(this + FILE_BUFFER, 0x1234);
        e.mem.set_u32(this + FILE_WORD_150, 9);
        e.map(SEEK_MODE, 4);
        e.mem.set_u32(SEEK_MODE, 1);
        e.map(FALLBACK_BUFFER_SIZE, 4);
        e.mem.set_u32(FALLBACK_BUFFER_SIZE, 0x2000);
        e.register(FILE_SEEK_00AA16D0, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.register(FILE_FLUSH_00AA15A0, |e, _| {
            let context = e.tls() + TLS_CONTEXT;
            assert_eq!(e.mem.u32(context), BUFFER_CONTEXT);
            Ret::default()
        });
        this
    }

    #[test]
    fn fn_00affd80_keeps_an_equal_size() {
        let mut e = Engine::new();
        // 0x9000 is capped to the limit 0x8000, the current size.
        let this = file(&mut e, 0x8000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00affd80, &args![this, 0x9000u32]).bool());
        assert!(calls_to(&e, FILE_FLUSH_00AA15A0).is_empty());
        assert_eq!(e.mem.u32(this + FILE_WORD_150), 9);
    }

    #[test]
    fn fn_00affd80_replaces_the_buffer() {
        let mut e = Engine::new();
        // The limit 0x100 is raised to 0x4000.
        let this = file(&mut e, 0x100);
        e.register(ALLOCATE, |_, _| Ret {
            eax: 0x5000,
            ..Ret::default()
        });
        let context = e.tls() + TLS_CONTEXT;
        e.mem.set_u32(context, 3);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00affd80, &args![this, 0x9000u32]).bool());
        assert_eq!(calls_to(&e, FILE_SEEK_00AA16D0), vec![vec![this, 0, 1]]);
        assert_eq!(calls_to(&e, FILE_FLUSH_00AA15A0), vec![vec![this]]);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, 0x1234]]);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![MEMORY_MANAGER, 0x4000]]);
        assert_eq!(e.mem.u32(this + FILE_BUFFER_SIZE), 0x4000);
        assert_eq!(e.mem.u32(this + FILE_BUFFER), 0x5000);
        assert_eq!(e.mem.u32(this + FILE_WORD_150), 0);
        assert_eq!(e.mem.u32(context), 3);
    }

    #[test]
    fn fn_00affd80_falls_back_when_the_allocation_fails() {
        let mut e = Engine::new();
        let this = file(&mut e, 0x10000);
        let mut answers = vec![0x6000u32, 0];
        e.register_double(ALLOCATE, move |_, _| Ret {
            eax: answers.pop().unwrap(),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00affd80, &args![this, 0xc000u32]).bool());
        assert_eq!(
            calls_to(&e, ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0xc000], vec![MEMORY_MANAGER, 0x2000]]
        );
        assert_eq!(e.mem.u32(this + FILE_BUFFER_SIZE), 0x2000);
        assert_eq!(e.mem.u32(this + FILE_BUFFER), 0x6000);
        assert_eq!(e.mem.u32(this + FILE_WORD_150), 0);
    }
}
