//! `BSHavok/run_00ccc2f0` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{enter_scope, stream_transfer, MEMORY_MANAGER, MEMORY_MANAGER_ALLOCATE};

/// `MemoryManager::Deallocate` (`00aa4060`, `this` = the manager, one argument).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// Offset of the stream's reader object, and the offset in it of the read function.
const STREAM_READER: u32 = 0x24c;
const READ_FUNCTION: u32 = 8;
/// Virtual slot of the object that answers the size of its data.
const SIZE_SLOT: u32 = 0xa8;

// Translated from 00ccc2f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccc2f0 - the FS:[0] exception frame (unwinding that restores the scope word) is not modeled.
/// `fn_00ccc2f0` (unnamed in the Xbox PDB): takes a stream. With the allocation-scope word set
/// to 0x10: asks the object for its data size (virtual slot 0xA8), allocates a buffer of that
/// size from the memory manager, reads that many bytes from the stream's reader (the object at
/// `+0x24c` of the stream, its function at `+8`, the reader's `+4` advanced by the answer),
/// frees the buffer, puts the scope word back and answers the size. The bytes read are not
/// used by this function.
pub fn fn_00ccc2f0(e: &mut Engine, this: u32, stream: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    let size = e.vcall(this, SIZE_SLOT, &args![]).u32();
    let buffer = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32();
    let reader = e.mem.u32(stream + STREAM_READER);
    stream_transfer(e, reader, READ_FUNCTION, buffer, size, None, 0);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, buffer]);
    e.mem.set_u32(word, saved);
    size
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ccc2f0, fn_00ccc2f0(u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{calls_to, logging_engine};
    use crate::units::bshavok::bhkentity::SCOPE_WORD;

    const SIZE_DOUBLE: u32 = 0x0900_00a8;
    const READ_DOUBLE: u32 = 0x0900_1000;

    #[test]
    fn reads_the_data_size_into_a_temporary_buffer() {
        let mut e = logging_engine();
        e.register_double(SIZE_DOUBLE, |e, _| {
            let word = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(word), 0x10);
            Ret {
                eax: 0x30,
                ..Ret::default()
            }
        });
        e.register_double(MEMORY_MANAGER_ALLOCATE, |_, _| Ret {
            eax: 0x6000,
            ..Ret::default()
        });
        e.register_double(READ_DOUBLE, |_, _| Ret {
            eax: 0x28,
            ..Ret::default()
        });
        e.register_double(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let table = e.mem.alloc(0x200);
        e.mem.set_u32(this, table);
        e.mem.set_u32(table + SIZE_SLOT, SIZE_DOUBLE);
        let stream = e.mem.alloc(0x260);
        let reader = e.mem.alloc(0x20);
        e.mem.set_u32(stream + STREAM_READER, reader);
        e.mem.set_u32(reader + READ_FUNCTION, READ_DOUBLE);
        e.mem.set_u32(reader + 4, 100);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 9);
        assert_eq!(e.call(0x00cc_c2f0, &args![this, stream]).u32(), 0x30);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            [[MEMORY_MANAGER, 0x30]]
        );
        assert_eq!(calls_to(&e, READ_DOUBLE), [[reader, 0x6000, 0x30, 0, 0]]);
        assert_eq!(e.mem.u32(reader + 4), 100 + 0x28);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE), [[MEMORY_MANAGER, 0x6000]]);
        assert_eq!(e.mem.u32(word), 9);
    }
}
