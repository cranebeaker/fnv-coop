//! Small PC helpers needed by the ExtraData list dependency chain.
//!
//! The engine map leaves these bodies in address-range groups, without a
//! source-unit name. `BSExtraData` layout names are Xbox PDB names; PC
//! disassembly confirms the offsets and stack cleanup. Native header
//! vectors were checked against FalloutNV.exe 1.4.0.525 on 2026-10-10.

use super::extradatalist::BSExtraData;
use crate::prelude::*;

const BASE_VTABLE: u32 = 0x0101_4210;
const CURRENT_THREAD_IMPORT: u32 = 0x00fd_f1d0;

// Translated from 0040ec80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSExtraData` (Xbox PDB) base constructor: vtable, type byte, null next.
/// The PC callee consumes one stack word (`RET 4`), reading its low byte.
pub fn construct(e: &mut Engine, this: Ptr<BSExtraData>, kind: u8) -> Ptr<BSExtraData> {
    e.mem.set_u32(this.addr(), BASE_VTABLE);
    e.set(this, BSExtraData::cEtype, kind);
    e.set(this, BSExtraData::pNext, Ptr::NULL);
    this
}

// Translated from 0040ecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSExtraData` (Xbox PDB) base destructor only restores the base vtable.
/// It does not release memory or change the type, link or subclass payload.
pub fn destruct(e: &mut Engine, this: Ptr<BSExtraData>) {
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

// Translated from 004f1540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads `BSExtraData::cEtype` (Xbox PDB). Only AL is a return value.
pub fn kind(e: &mut Engine, this: Ptr<BSExtraData>) -> u8 {
    e.get(this, BSExtraData::cEtype)
}

// Translated from 00403550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the word at +8, used as `BSExtraData::pNext` (Xbox PDB).
pub fn set_next(e: &mut Engine, this: Ptr<BSExtraData>, next: Ptr<BSExtraData>) {
    e.set(this, BSExtraData::pNext, next.cast());
}

// Translated from 0040fc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The game's no-argument wrapper around the GetCurrentThreadId import.
pub fn current_thread_id(e: &mut Engine) -> u32 {
    e.call(CURRENT_THREAD_IMPORT, &[]).u32()
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0040ec80,
            construct(Ptr<BSExtraData>, u8) -> Ptr<BSExtraData>
        ),
        entry!(0x004f1540, kind(Ptr<BSExtraData>) -> u8),
        entry!(0x0040ecb0, destruct(Ptr<BSExtraData>)),
        entry!(0x00403550, set_next(Ptr<BSExtraData>, Ptr<BSExtraData>)),
        entry!(0x0040fc90, current_thread_id() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Generated native cases: private exe SHA256
    // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f,
    // AMD Ryzen AI 9 HX 370, FCW 027F / MXCSR 1F80. Guard/padding bytes
    // are generated sentinels, not bytes copied from the executable.
    #[test]
    fn destructor_native_vector_changes_only_the_vtable() {
        let mut e = Engine::new();
        let object = e.mem.alloc(16);
        e.mem.write(object, &[0xaa; 16]);
        e.call(0x0040ecb0, &[object]);
        let mut expected = [0xaa; 16];
        expected[..4].copy_from_slice(&BASE_VTABLE.to_le_bytes());
        assert_eq!(e.mem.bytes(object, 16), expected);
    }

    #[test]
    fn constructor_native_vectors_preserve_padding_and_truncate_type_word() {
        let mut e = Engine::new();
        let object = e.mem.alloc(16);
        for value in [0u32, 67, 511] {
            e.mem.write(object, &[0xaa; 16]);
            assert_eq!(e.call(0x0040ec80, &[object, value]).u32(), object);
            let mut expected = [0xaa; 16];
            expected[..4].copy_from_slice(&BASE_VTABLE.to_le_bytes());
            expected[4] = value as u8;
            expected[8..12].fill(0);
            assert_eq!(e.mem.bytes(object, 16), expected);
        }
    }

    #[test]
    fn type_native_vectors_read_only_the_type_byte() {
        let mut e = Engine::new();
        let object = e.mem.alloc(16);
        for value in [0u8, 67, 255] {
            let mut bytes = [0xaa; 16];
            bytes[4] = value;
            e.mem.write(object, &bytes);
            assert_eq!(e.call(0x004f1540, &[object]).u8(), value);
            assert_eq!(e.mem.bytes(object, 16), bytes);
        }
    }

    #[test]
    fn next_native_vectors_change_only_the_link_word() {
        let mut e = Engine::new();
        let object = e.mem.alloc(16);
        for next in [0u32, 0x1234_5678] {
            e.mem.write(object, &[0xaa; 16]);
            e.call(0x00403550, &[object, next]);
            let mut expected = [0xaa; 16];
            expected[8..12].copy_from_slice(&next.to_le_bytes());
            assert_eq!(e.mem.bytes(object, 16), expected);
        }
    }

    #[test]
    fn thread_wrapper_forwards_the_import_return_value_without_arguments() {
        let mut e = Engine::new();
        e.register(CURRENT_THREAD_IMPORT, |_, args| {
            assert!(args.is_empty());
            0x7654_3210u32.into_ret()
        });
        assert_eq!(e.call(0x0040fc90, &[]).u32(), 0x7654_3210);
    }
}
