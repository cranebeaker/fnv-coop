//! `fallout/misc/memorymanagerfile.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `MemoryManagerFile` (Xbox PDB, 0x2c bytes) is an `IMemoryManagerFile`
//! (the vtable word) holding a `BSSystemFile File` at +4; its methods
//! forward to that file.

#[allow(unused_imports)]
use crate::prelude::*;

/// `IMemoryManagerFile`'s vtable (RTTI `.?AVIMemoryManagerFile@@`).
const I_MEMORY_MANAGER_FILE_VTABLE: u32 = 0x0108_3b00;
/// `MemoryManagerFile`'s vtable (RTTI `.?AVMemoryManagerFile@@`).
const MEMORY_MANAGER_FILE_VTABLE: u32 = 0x0108_3ae8;
/// `MemoryManagerFile::File` (Xbox PDB): the `BSSystemFile` at +4.
const FILE: u32 = 4;

/// `operator delete` (`00401030`, cdecl `(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00b00950` (thiscall on `File`, no stack words): the destructor
/// `008784d0` runs on the embedded file.
const FILE_DESTRUCTOR: u32 = 0x00b0_0950;
/// `BSSystemFile::DoRead` (`008526d0`, engine map; thiscall
/// `(file, buffer, size_low, size_high, &read)`, `RET 0x10`).
const DO_READ: u32 = 0x0085_26d0;
/// `BSSystemFile::DoWrite` (`00856350`, engine map; same shape as `DoRead`).
const DO_WRITE: u32 = 0x0085_6350;
/// `BSSystemFile::DoGetSize` (`0065c290`, engine map; thiscall
/// `(file, &size)`, `RET 4`).
const DO_GET_SIZE: u32 = 0x0065_c290;
/// `BSSystemFile::DoSeek` (`008785e0`, engine map; thiscall
/// `(file, offset_low, offset_high, mode, &position)`, `RET 0x10`).
const DO_SEEK: u32 = 0x0087_85e0;

// Translated from 00878430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `IMemoryManagerFile` destructor's body,
/// which stores `IMemoryManagerFile`'s vtable (EAX is left holding `this`).
pub fn fn_00878430(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), I_MEMORY_MANAGER_FILE_VTABLE);
    this
}

// Translated from 00878450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IMemoryManagerFile::scalar deleting destructor` (Xbox PDB): runs the
/// destructor and frees the object if bit 0 of `flags` is set. Returns
/// `this`.
pub fn i_memory_manager_file_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00878430(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00878480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryManagerFile::scalar deleting destructor` (Xbox PDB): runs the
/// destructor (`008784d0`) and frees the object if bit 0 of `flags` is set.
/// Returns `this`.
pub fn memory_manager_file_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_008784d0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008784b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `IMemoryManagerFile` constructor (stores
/// its vtable). Returns `this`.
pub fn fn_008784b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), I_MEMORY_MANAGER_FILE_VTABLE);
    this
}

// Translated from 008784d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra matches a library destructor
/// signature): the `MemoryManagerFile` destructor. Stores
/// `MemoryManagerFile`'s vtable, destroys `File` (`00b00950`), then runs the
/// `IMemoryManagerFile` part (`00878430`).
// Translation incomplete: 008784d0 - FS:[0] exception frame (unwind state around the File destructor) is not translated.
pub fn fn_008784d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MEMORY_MANAGER_FILE_VTABLE);
    e.call(FILE_DESTRUCTOR, &args![this.addr().wrapping_add(FILE)]);
    fn_00878430(e, this);
}

// Translated from 00878530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryManagerFile::Read` (Xbox PDB): `File.DoRead(buffer, size)` (the
/// size's high word 0); returns the low word of the count it reports.
pub fn memory_manager_file_read(e: &mut Engine, this: Ptr, buffer: u32, size: u32) -> u32 {
    e.with_stack(8, |e, count| {
        e.call(
            DO_READ,
            &args![this.addr().wrapping_add(FILE), buffer, size, 0u32, count],
        );
        e.mem.u32(count.addr())
    })
}

// Translated from 00878560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryManagerFile::Write` (Xbox PDB): `File.DoWrite(buffer, size)` (the
/// size's high word 0); returns the low word of the count it reports.
pub fn memory_manager_file_write(e: &mut Engine, this: Ptr, buffer: u32, size: u32) -> u32 {
    e.with_stack(8, |e, count| {
        e.call(
            DO_WRITE,
            &args![this.addr().wrapping_add(FILE), buffer, size, 0u32, count],
        );
        e.mem.u32(count.addr())
    })
}

// Translated from 00878590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryManagerFile::Size` (Xbox PDB): the low word of
/// `File.DoGetSize`'s size.
pub fn memory_manager_file_size(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(8, |e, size| {
        e.call(DO_GET_SIZE, &args![this.addr().wrapping_add(FILE), size]);
        e.mem.u32(size.addr())
    })
}

// Translated from 008785b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryManagerFile::Seek` (Xbox PDB): `File.DoSeek(offset, mode 0)` with
/// the offset sign-extended to 64 bits (`CDQ`). The function sets no result;
/// EAX is left as `DoSeek`'s, returned here.
pub fn memory_manager_file_seek(e: &mut Engine, this: Ptr, offset: i32) -> u32 {
    let high = (offset >> 31) as u32;
    e.with_stack(8, |e, position| {
        e.call(
            DO_SEEK,
            &args![
                this.addr().wrapping_add(FILE),
                offset as u32,
                high,
                0u32,
                position
            ],
        )
        .u32()
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00878430, fn_00878430(Ptr) -> Ptr),
        entry!(
            0x00878450,
            i_memory_manager_file_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00878480,
            memory_manager_file_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x008784b0, fn_008784b0(Ptr) -> Ptr),
        entry!(0x008784d0, fn_008784d0(Ptr)),
        entry!(0x00878530, memory_manager_file_read(Ptr, u32, u32) -> u32),
        entry!(0x00878560, memory_manager_file_write(Ptr, u32, u32) -> u32),
        entry!(0x00878590, memory_manager_file_size(Ptr) -> u32),
        entry!(0x008785b0, memory_manager_file_seek(Ptr, i32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn interface_constructor_and_destructor_store_its_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x2c);
        assert_eq!(e.call(0x008784b0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), I_MEMORY_MANAGER_FILE_VTABLE);
        e.mem.set_u32(this, 0);
        assert_eq!(e.call(0x00878430, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), I_MEMORY_MANAGER_FILE_VTABLE);
    }

    #[test]
    fn scalar_deleting_destructors_free_on_bit_zero() {
        let mut e = Engine::new();
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(FILE_DESTRUCTOR, |e, a| {
            // The vtable is MemoryManagerFile's while File is destroyed.
            assert_eq!(e.mem.u32(a[0] - 4), MEMORY_MANAGER_FILE_VTABLE);
            Ret::default()
        });
        let this = e.mem.alloc(0x2c);
        for (addr, flags, frees) in [
            (0x00878450u32, 1u32, true),
            (0x00878450, 2, false),
            (0x00878480, 3, true),
            (0x00878480, 0, false),
        ] {
            e.mem.set_u32(this, 0);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this), I_MEMORY_MANAGER_FILE_VTABLE);
            let deletes = calls_to(&log, OPERATOR_DELETE);
            assert_eq!(deletes, if frees { vec![vec![this]] } else { vec![] });
            let file_destroyed = calls_to(&log, FILE_DESTRUCTOR);
            if addr == 0x00878480 {
                assert_eq!(file_destroyed, vec![vec![this + 4]]);
            } else {
                assert!(file_destroyed.is_empty());
            }
        }
    }

    #[test]
    fn read_write_size_forward_to_the_file() {
        let mut e = Engine::new();
        e.register(DO_READ, |e, a| {
            e.mem.set_u32(a[4], a[2] - 1);
            e.mem.set_u32(a[4] + 4, 0xffff);
            Ret::default()
        });
        e.register(DO_WRITE, |e, a| {
            e.mem.set_u32(a[4], a[2] + 1);
            Ret::default()
        });
        e.register(DO_GET_SIZE, |e, a| {
            e.mem.set_u32(a[1], 0x1234);
            e.mem.set_u32(a[1] + 4, 1);
            Ret::default()
        });
        let this = e.mem.alloc(0x2c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00878530, &args![this, 0x500u32, 10u32]).u32(), 9);
        assert_eq!(e.call(0x00878560, &args![this, 0x600u32, 10u32]).u32(), 11);
        assert_eq!(e.call(0x00878590, &args![this]).u32(), 0x1234);
        let log = e.call_log.take().unwrap();
        let read = &calls_to(&log, DO_READ)[0];
        assert_eq!(read[..4], [this + 4, 0x500, 10, 0]);
        let write = &calls_to(&log, DO_WRITE)[0];
        assert_eq!(write[..4], [this + 4, 0x600, 10, 0]);
        assert_eq!(calls_to(&log, DO_GET_SIZE)[0][0], this + 4);
    }

    #[test]
    fn seek_sign_extends_the_offset() {
        let mut e = Engine::new();
        e.register(DO_SEEK, |_, _| 0x77u32.into_ret());
        let this = e.mem.alloc(0x2c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008785b0, &args![this, -5i32]).u32(), 0x77);
        e.call(0x008785b0, &args![this, 5i32]);
        let log = e.call_log.take().unwrap();
        let seeks = calls_to(&log, DO_SEEK);
        assert_eq!(seeks[0][..4], [this + 4, (-5i32) as u32, 0xffff_ffff, 0]);
        assert_eq!(seeks[1][..4], [this + 4, 5, 0, 0]);
    }
}
