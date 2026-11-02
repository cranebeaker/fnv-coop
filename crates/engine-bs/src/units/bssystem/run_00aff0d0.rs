//! `BSSystem/run_00aff0d0` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function-local static `BSWin32FileCache` (`011f81e8`, its RTTI name
/// by the vtable its constructor `00b01c00` stores) and the guard word
/// whose bit 0 says it was constructed.
const FILE_CACHE: u32 = 0x011f_81e8;
const FILE_CACHE_GUARD: u32 = 0x011f_824c;
/// The `atexit` destructor of the cache (`00fdc3e0`) and `_atexit`
/// (`00ec658f`).
const FILE_CACHE_ATEXIT: u32 = 0x00fd_c3e0;
const ATEXIT: u32 = 0x00ec_658f;
/// `BSFile`'s vtable (its RTTI name).
const BS_FILE_VTABLE: u32 = 0x010a_4764;

// Translated from 00aff100 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aff100 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// The file cache singleton: on the first call constructs the static
/// `BSWin32FileCache` at `011f81e8` (`00b01c00`) and registers its
/// destructor with `_atexit`; returns its address.
pub fn fn_00aff100(e: &mut Engine) -> u32 {
    let guard: u32 = e.global(FILE_CACHE_GUARD);
    if guard & 1 == 0 {
        let guard: u32 = e.global(FILE_CACHE_GUARD);
        e.set_global(FILE_CACHE_GUARD, guard | 1);
        e.call(0x00b0_1c00, &args![FILE_CACHE]);
        e.call(ATEXIT, &args![FILE_CACHE_ATEXIT]);
    }
    FILE_CACHE
}

// Translated from 00aff180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFile`'s constructor (its vtable `010a4764`): the base (`00aa14b0`),
/// then the vtable and the start values of its fields (all 0 but the word
/// at `+0x38`, -1). Returns `this`.
pub fn fn_00aff180(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x00aa_14b0, &args![this]);
    let base = this.addr();
    e.mem.set_u32(base, BS_FILE_VTABLE);
    for offset in [0x28u32, 0x10, 0x14, 0x18, 0x20, 0x24] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    e.mem.set_u8(base.wrapping_add(0x44), 0);
    for offset in [0x14cu32, 0x148, 0x150] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    e.mem.set_u8(base.wrapping_add(0x30), 0);
    e.mem.set_u32(base.wrapping_add(0x34), 0);
    e.mem.set_u32(base.wrapping_add(0x38), 0xffff_ffff);
    e.mem.set_u32(base.wrapping_add(0x3c), 0);
    e.mem.set_u32(base.wrapping_add(0x40), 0);
    e.mem.set_u8(base.wrapping_add(0x2c), 0);
    this
}

/// `DeleteFileA` (KERNEL32 import slot).
const IMPORT_DELETE_FILE_A: u32 = 0x00fd_f0d4;

// Translated from 00aff0b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSSystemFile::DeleteFileA` (engine map; `cdecl`, one word): `DeleteFileA(path)`; 0 when it
/// succeeded, 4 when it failed (`NEG`, `SBB`, `AND 0xfffffffc`, `ADD 4`).
pub fn bs_system_file_delete_file_a(e: &mut Engine, path: Ptr) -> u32 {
    if e.call(IMPORT_DELETE_FILE_A, &args![path]).u32() != 0 {
        0
    } else {
        4
    }
}

/// `CopyFileA` (KERNEL32 import slot).
const IMPORT_COPY_FILE_A: u32 = 0x00fd_f1b8;

// Translated from 00aff0d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words: the source path, the destination path and
/// a flag whose low byte is passed zero-extended): `CopyFileA(source, destination, flag)`; 0
/// when it succeeded, 4 when it failed (`NEG`, `SBB`, `AND 0xfffffffc`, `ADD 4`).
pub fn fn_00aff0d0(e: &mut Engine, source: u32, destination: u32, fail_if_exists: u32) -> u32 {
    let flag = u32::from(fail_if_exists as u8);
    if e.call(IMPORT_COPY_FILE_A, &args![source, destination, flag])
        .u32()
        != 0
    {
        0
    } else {
        4
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_f0d0, fn_00aff0d0(u32, u32, u32) -> u32),
        entry!(0x00af_f100, fn_00aff100() -> u32),
        entry!(0x00af_f0b0, bs_system_file_delete_file_a(Ptr) -> u32),
        entry!(0x00af_f180, fn_00aff180(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_cache_is_constructed_once() {
        let mut e = Engine::new();
        e.map(0x011f_8000, 0x1000);
        e.register(0x00b0_1c00, |_, a| a[0].into_ret());
        e.register(ATEXIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00af_f100, &[]).u32(), FILE_CACHE);
        assert_eq!(e.call(0x00af_f100, &[]).u32(), FILE_CACHE);
        let log = e.call_log.take().unwrap();
        let built: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00b0_1c00).collect();
        assert_eq!(built.len(), 1);
        assert!(log.contains(&(ATEXIT, vec![FILE_CACHE_ATEXIT])));
        assert_eq!(e.global::<u32>(FILE_CACHE_GUARD) & 1, 1);
    }

    #[test]
    fn bs_file_constructor_sets_its_fields() {
        let mut e = Engine::new();
        e.register(0x00aa_14b0, |_, _| Ret::default());
        let file = e.mem.alloc(0x154);
        for offset in (0..0x154).step_by(4) {
            e.mem.set_u32(file + offset, 0x5555_5555);
        }
        assert_eq!(e.call(0x00af_f180, &args![file]).u32(), file);
        assert_eq!(e.mem.u32(file), BS_FILE_VTABLE);
        assert_eq!(e.mem.u32(file + 0x38), 0xffff_ffff);
        assert_eq!(e.mem.u32(file + 0x10), 0);
        assert_eq!(e.mem.u32(file + 0x150), 0);
        assert_eq!(e.mem.u8(file + 0x44), 0);
        assert_eq!(e.mem.u8(file + 0x45), 0x55);
        assert_eq!(e.mem.u32(file + 0x1c), 0x5555_5555);
    }
}

#[cfg(test)]
mod delete_tests {
    use super::*;

    #[test]
    fn delete_answers_0_on_success_and_4_on_failure() {
        let mut e = Engine::new();
        e.register(IMPORT_DELETE_FILE_A, |_, a| {
            u32::from(a[0] == 0x10).into_ret()
        });
        assert_eq!(e.call(0x00af_f0b0, &args![0x10u32]).u32(), 0);
        assert_eq!(e.call(0x00af_f0b0, &args![0x20u32]).u32(), 4);
    }
}

#[cfg(test)]
mod copy_tests {
    use super::*;

    #[test]
    fn copy_passes_the_flag_byte_and_answers_0_or_4() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(IMPORT_COPY_FILE_A, |_, a| {
            u32::from(a[0] == 0x10).into_ret()
        });
        assert_eq!(
            e.call(0x00af_f0d0, &args![0x10u32, 0x20u32, 0x301u32])
                .u32(),
            0
        );
        assert_eq!(fn_00aff0d0(&mut e, 0x11, 0x20, 0), 4);
        let log = e.call_log.take().unwrap();
        let copies: Vec<_> = log.iter().filter(|c| c.0 == IMPORT_COPY_FILE_A).collect();
        assert_eq!(copies[0].1, vec![0x10, 0x20, 1]);
        assert_eq!(copies[1].1, vec![0x11, 0x20, 0]);
    }
}
