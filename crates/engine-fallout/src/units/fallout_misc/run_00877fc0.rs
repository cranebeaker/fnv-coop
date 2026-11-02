//! `fallout/misc/run_00877fc0` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit is a run of small functions between `main.cpp` and
//! `memorylevelmanager.cpp`:
//!
//! - `00877fc0`: the destructor of `SettingT<RegSettingCollection>`;
//! - `00878030`, `00878080`: the constructor of `MemoryLevelManager` (the
//!   Xbox PDB layout, five `u32` limits then a critical section at `+0x14`,
//!   matches the fields it writes) and a method that works under that
//!   object's lock;
//! - `00878110`, `00878130`: reader and clearer of the word at `+0x134` of
//!   the object `00401020` returns;
//! - `00878160`, `00878200`, `008781e0`, `008781f0`: a scope that frees
//!   memory while a global limit word (`011a95fc`) is raised, and the
//!   accessors of that word.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00401020`: the object whose word at `+0x134` is read and cleared here
/// (the memory manager singleton).
const GET_MEMORY_MANAGER: u32 = 0x0040_1020;
/// `0044fae0`: `InitializeCriticalSection` on `ECX`, returns it.
const INITIALIZE_CRITICAL_SECTION: u32 = 0x0044_fae0;
/// `0044b130`: `TryEnterCriticalSection` on `ECX`.
const TRY_ENTER_CRITICAL_SECTION: u32 = 0x0044_b130;
/// `0082f1f0`: `LeaveCriticalSection` on `ECX`.
const LEAVE_CRITICAL_SECTION: u32 = 0x0082_f1f0;
/// `00866a90` (`__thiscall` on the manager from `00401020`, three stack
/// words: a zero, the level, a pointer to a flag byte).
const FREE_LEVEL: u32 = 0x0086_6a90;
/// `00877950`: the `RegSettingCollection` singleton the setting removes
/// itself from.
const REG_SETTING_COLLECTION: u32 = 0x0087_7950;
/// `00404950`: the base class destructor of a setting.
const SETTING_BASE_DESTRUCTOR: u32 = 0x0040_4950;
/// The vtable `SettingT<RegSettingCollection>` installs while it is
/// destroyed (the RTTI name at its `-4` slot says which class it is).
const SETTING_VTABLE: u32 = 0x0108_39ac;
/// The global word `008781e0` writes and `008781f0` reads.
const LIMIT_WORD: u32 = 0x011a_95fc;
/// The global pointer whose object `00c3e310`/`00c3e340` are called on.
const IO_MANAGER: u32 = 0x0120_2d98;
/// `00c3e310` / `00c3e340` (`__thiscall` on the global at `01202d98`):
/// call a callback of the object when one is set (`+0x7c` / `+0x80`).
const IO_MANAGER_BEGIN: u32 = 0x00c3_e310;
const IO_MANAGER_END: u32 = 0x00c3_e340;
/// `00652160` / `00652190`: a pair of calls; the first returns a byte the
/// scope keeps, the second undoes it.
const PAIR_BEGIN: u32 = 0x0065_2160;
const PAIR_END: u32 = 0x0065_2190;
/// `0043c4b0`: returns the object `004a0370` / `004a03c0` are called on.
const PAIR_OBJECT: u32 = 0x0043_c4b0;
const PAIR_OBJECT_BEGIN: u32 = 0x004a_0370;
const PAIR_OBJECT_END: u32 = 0x004a_03c0;
/// `00703980`, `00a5b460`, `00aa7030`: argument-less calls made when a scope
/// opens (`00703980`) or closes (`00a5b460`, `00aa7030`).
const SCOPE_OPEN_CALL: u32 = 0x0070_3980;
const SCOPE_CLOSE_CALL_A: u32 = 0x00a5_b460;
const SCOPE_CLOSE_CALL_B: u32 = 0x00aa_7030;
/// The value the limit word holds while a scope is open.
const LIMIT_WHILE_OPEN: u32 = 0x7fff_ffff;

layout! {
    /// `MemoryLevelManager` (Xbox PDB), 0x3c bytes on the Xbox; PC fields
    /// verified in 00878030.
    pub struct MemoryLevelManager: 0x2c {
        /// `iLowMemoryLevelLoading` (Xbox PDB): 20 MB.
        0x00 iLowMemoryLevelLoading: u32,
        /// `iLowMemoryLevelIdle` (Xbox PDB): 15 MB.
        0x04 iLowMemoryLevelIdle: u32,
        /// `iLowMemoryLevelIdleCritical` (Xbox PDB): 10 MB.
        0x08 iLowMemoryLevelIdleCritical: u32,
        /// `iLowMemoryBufferSize` (Xbox PDB): 2 MB.
        0x0C iLowMemoryBufferSize: u32,
        /// `iLastCheckedMemLevel` (Xbox PDB): starts at -1.
        0x10 iLastCheckedMemLevel: u32,
    }

    /// The word-and-bytes record a memory-freeing scope keeps on the
    /// caller's stack (`00878160` opens it, `00878200` closes it); the
    /// callers' local is 0xc bytes. The field names say what the code does
    /// with each byte; the game's names are not known.
    pub struct FreeScope: 0x0c {
        /// The first flag argument: whether the `0043c4b0` pair was
        /// started.
        0x03 pair_started: u8,
        /// The second flag argument (stored, never read back by the scope).
        0x04 second_flag: u8,
        /// What `00652160` returned when the last argument was set, else 0.
        0x05 begin_result: u8,
        /// The limit word (`011a95fc`) before the scope raised it.
        0x08 saved_limit: u32,
    }
}

// Translated from 00877fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingT<RegSettingCollection>::~SettingT` (the RTTI of the vtable it
/// installs, `010839ac`, names the class; `00877f90`, the map's scalar
/// deleting destructor of that class, calls it): installs the vtable, has
/// the collection singleton (`00877950`) drop the setting through its
/// vtable slot `+8`, then runs the base setting destructor.
// Translation incomplete: 00877fc0 - the compiler's exception-unwind frame (FS:[0]) is not translated.
pub fn setting_t_reg_setting_collection_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SETTING_VTABLE);
    let collection = e.call(REG_SETTING_COLLECTION, &[]).u32();
    e.vcall(collection, 8, &args![this]);
    e.call(SETTING_BASE_DESTRUCTOR, &args![this]);
}

// Translated from 00878030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryLevelManager::MemoryLevelManager` (the Xbox PDB layout matches
/// the fields written; the static initializer `00fa3650` builds the global
/// at `011deffc` with it, which `00446ef0` returns): initializes the
/// critical section at `+0x14`, then sets the limits to 20 MB, 15 MB, 10 MB
/// and 2 MB and the last checked level to -1.
pub fn memory_level_manager_construct(
    e: &mut Engine,
    this: Ptr<MemoryLevelManager>,
) -> Ptr<MemoryLevelManager> {
    e.call(
        INITIALIZE_CRITICAL_SECTION,
        &args![this.addr().wrapping_add(0x14)],
    );
    e.set(
        this,
        MemoryLevelManager::iLowMemoryLevelLoading,
        0x0140_0000,
    );
    e.set(this, MemoryLevelManager::iLowMemoryLevelIdle, 0x00f0_0000);
    e.set(
        this,
        MemoryLevelManager::iLowMemoryLevelIdleCritical,
        0x00a0_0000,
    );
    e.set(this, MemoryLevelManager::iLowMemoryBufferSize, 0x0020_0000);
    e.set(this, MemoryLevelManager::iLastCheckedMemLevel, 0xffff_ffff);
    this
}

// Translated from 00878110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0x134` of `this` (the object `00401020` returns).
pub fn fn_00878110(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x134))
}

// Translated from 00878130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at `+0x134` of `this` and returns it (zero).
pub fn fn_00878130(e: &mut Engine, this: Ptr) -> u32 {
    let address = this.addr().wrapping_add(0x134);
    e.mem.set_u32(address, 0);
    e.mem.u32(address)
}

// Translated from 00878080 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of the `MemoryLevelManager` object `this` (its callers pass the
/// result of `00446ef0`; the map has no name for it): when the lock at
/// `+0x14` is free (`TryEnterCriticalSection`), and the memory manager's
/// word at `+0x134` is not zero, calls `00866a90` for every level from 0
/// up to and including that word (re-read each time, since `00866a90` can
/// change it), then clears the word and leaves the lock.
///
/// The one stack word the callers leave is popped by the `RET 4` and
/// never read.
pub fn fn_00878080(e: &mut Engine, this: Ptr<MemoryLevelManager>, _unused_1: u32) {
    let lock = this.addr().wrapping_add(0x14);
    if !e.call(TRY_ENTER_CRITICAL_SECTION, &args![lock]).bool() {
        return;
    }
    let manager = e.call(GET_MEMORY_MANAGER, &[]).ptr::<()>();
    if fn_00878110(e, manager) != 0 {
        e.with_stack(1, |e, flag| {
            e.mem.set_u8(flag.addr(), 1);
            let mut level = 0u32;
            loop {
                let manager = e.call(GET_MEMORY_MANAGER, &[]).ptr::<()>();
                if level > fn_00878110(e, manager) {
                    break;
                }
                let manager = e.call(GET_MEMORY_MANAGER, &[]).u32();
                e.call(FREE_LEVEL, &args![manager, 0u32, level, flag.addr()]);
                level += 1;
            }
        });
        let manager = e.call(GET_MEMORY_MANAGER, &[]).ptr::<()>();
        fn_00878130(e, manager);
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![lock]);
}

// Translated from 008781e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the global limit word `011a95fc`.
pub fn fn_008781e0(e: &mut Engine, value: u32) {
    e.set_global(LIMIT_WORD, value);
}

// Translated from 008781f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The global limit word `011a95fc`.
pub fn fn_008781f0(e: &mut Engine) -> u32 {
    e.global::<u32>(LIMIT_WORD)
}

// Translated from 00878160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Opens a memory-freeing scope in the 0xc-byte record `scope` (cdecl,
/// four stack words): runs the begin callback of the global object at
/// `01202d98`; keeps `00652160`'s byte in `+5` when `begin_pair` is not zero
/// (else 0); when `first_flag` is not zero starts the `0043c4b0` pair
/// (`004a0370`); stores both flags at `+3` and `+4`, saves the limit word at
/// `+8`, raises the limit word to `0x7fffffff`, and calls `00703980`.
/// `00878200` undoes it.
pub fn fn_00878160(
    e: &mut Engine,
    scope: Ptr<FreeScope>,
    first_flag: u8,
    second_flag: u8,
    begin_pair: u8,
) {
    let io_manager = e.global::<u32>(IO_MANAGER);
    e.call(IO_MANAGER_BEGIN, &args![io_manager]);
    if begin_pair != 0 {
        let result = e.call(PAIR_BEGIN, &[]).u8();
        e.set(scope, FreeScope::begin_result, result);
    } else {
        e.set(scope, FreeScope::begin_result, 0);
    }
    if first_flag != 0 {
        let object = e.call(PAIR_OBJECT, &[]).u32();
        e.call(PAIR_OBJECT_BEGIN, &args![object]);
    }
    e.set(scope, FreeScope::pair_started, first_flag);
    e.set(scope, FreeScope::second_flag, second_flag);
    let limit = fn_008781f0(e);
    e.set(scope, FreeScope::saved_limit, limit);
    fn_008781e0(e, LIMIT_WHILE_OPEN);
    e.call(SCOPE_OPEN_CALL, &[]);
}

// Translated from 00878200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Closes the scope `00878160` opened in `scope`: calls `00a5b460` and
/// `00aa7030`, ends the `00652160` pair when `+5` is not zero, restores the
/// limit word from `+8`, ends the `0043c4b0` pair (`004a03c0`) when `+3` is
/// not zero, and runs the end callback of the global object at `01202d98`.
pub fn fn_00878200(e: &mut Engine, scope: Ptr<FreeScope>) {
    e.call(SCOPE_CLOSE_CALL_A, &[]);
    e.call(SCOPE_CLOSE_CALL_B, &[]);
    if e.get(scope, FreeScope::begin_result) != 0 {
        e.call(PAIR_END, &[]);
    }
    let limit = e.get(scope, FreeScope::saved_limit);
    fn_008781e0(e, limit);
    if e.get(scope, FreeScope::pair_started) != 0 {
        let object = e.call(PAIR_OBJECT, &[]).u32();
        e.call(PAIR_OBJECT_END, &args![object]);
    }
    let io_manager = e.global::<u32>(IO_MANAGER);
    e.call(IO_MANAGER_END, &args![io_manager]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00877fc0, setting_t_reg_setting_collection_destruct(Ptr)),
        entry!(
            0x00878030,
            memory_level_manager_construct(Ptr<MemoryLevelManager>) -> Ptr<MemoryLevelManager>
        ),
        entry!(0x00878080, fn_00878080(Ptr<MemoryLevelManager>, u32)),
        entry!(0x00878110, fn_00878110(Ptr) -> u32),
        entry!(0x00878130, fn_00878130(Ptr) -> u32),
        entry!(0x00878160, fn_00878160(Ptr<FreeScope>, u8, u8, u8)),
        entry!(0x008781e0, fn_008781e0(u32)),
        entry!(0x008781f0, fn_008781f0() -> u32),
        entry!(0x00878200, fn_00878200(Ptr<FreeScope>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word_ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn setting_destructor_removes_itself_from_the_collection_then_runs_the_base() {
        let mut e = Engine::new();
        let setting = e.mem.alloc(16);
        // The collection singleton: an object whose vtable slot +8 is a double.
        e.put_vtable(0x0300_0000, &[0, 0, 0x0300_0100]);
        e.mem.map(0x0400_0000, 4);
        e.mem.set_u32(0x0400_0000, 0x0300_0000);
        e.register(0x0087_7950, |_, _| word_ret(0x0400_0000));
        e.register(0x0300_0100, |_, _| Ret::default());
        e.register(0x0040_4950, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0087_7fc0, &[setting]);
        assert_eq!(e.mem.u32(setting), 0x0108_39ac);
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![
                (0x0087_7fc0, vec![setting]),
                (0x0087_7950, vec![]),
                (0x0300_0100, vec![0x0400_0000, setting]),
                (0x0040_4950, vec![setting]),
            ]
        );
    }

    #[test]
    fn memory_level_manager_constructor_sets_the_limits() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x2c);
        e.mem.write(object, &[0xaa; 0x2c]);
        e.register(0x0044_fae0, |_, a| word_ret(a[0]));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_8030, &[object]).u32(), object);
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![
                (0x0087_8030, vec![object]),
                (0x0044_fae0, vec![object + 0x14])
            ]
        );
        let words: Vec<u32> = (0..5).map(|i| e.mem.u32(object + 4 * i)).collect();
        assert_eq!(
            words,
            [
                0x0140_0000,
                0x00f0_0000,
                0x00a0_0000,
                0x0020_0000,
                0xffff_ffff
            ]
        );
        // The critical section and the bytes after it are not touched.
        assert_eq!(e.mem.u32(object + 0x14), 0xaaaa_aaaa);
    }

    #[test]
    fn word_at_0x134_is_read_and_cleared() {
        let mut e = Engine::new();
        let manager = e.mem.alloc(0x140);
        e.mem.set_u32(manager + 0x134, 6);
        assert_eq!(e.call(0x0087_8110, &[manager]).u32(), 6);
        assert_eq!(e.call(0x0087_8130, &[manager]).u32(), 0);
        assert_eq!(e.mem.u32(manager + 0x134), 0);
        assert_eq!(e.call(0x0087_8110, &[manager]).u32(), 0);
    }

    #[test]
    fn free_levels_run_up_to_the_word_which_is_reread_every_time() {
        let mut e = Engine::new();
        let level_manager = e.mem.alloc(0x2c);
        let manager = e.mem.alloc(0x140);
        e.mem.set_u32(manager + 0x134, 3);
        e.register(0x0044_b130, |_, _| word_ret(1));
        e.register(0x0082_f1f0, |_, _| Ret::default());
        e.register_double(0x0040_1020, move |_, _| word_ret(manager));
        // The first level lowers the word to 1: levels 0 and 1 run, not 2.
        e.register_double(0x0086_6a90, move |e, a| {
            assert_eq!(e.mem.u8(a[3]), 1);
            if a[2] == 0 {
                e.mem.set_u32(manager + 0x134, 1);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0087_8080, &[level_manager, 0xdead]);
        let log = e.call_log.take().unwrap();
        let levels: Vec<(u32, u32, u32)> = log
            .iter()
            .filter(|(address, _)| *address == 0x0086_6a90)
            .map(|(_, a)| (a[0], a[1], a[2]))
            .collect();
        assert_eq!(levels, [(manager, 0, 0), (manager, 0, 1)]);
        assert_eq!(e.mem.u32(manager + 0x134), 0);
        assert_eq!(log.first().unwrap().0, 0x0087_8080);
        assert_eq!(
            log.last().unwrap(),
            &(0x0082_f1f0, vec![level_manager + 0x14])
        );
        assert_eq!(log[1], (0x0044_b130, vec![level_manager + 0x14]));
    }

    #[test]
    fn free_levels_do_nothing_when_the_lock_is_taken_or_the_word_is_zero() {
        let mut e = Engine::new();
        let level_manager = e.mem.alloc(0x2c);
        let manager = e.mem.alloc(0x140);
        e.register(0x0044_b130, |_, _| word_ret(0));
        e.register(0x0082_f1f0, |_, _| Ret::default());
        e.register_double(0x0040_1020, move |_, _| word_ret(manager));
        e.call_log = Some(vec![]);
        e.call(0x0087_8080, &[level_manager, 0]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);

        // Lock free, nothing to free: the lock is still left.
        e.register(0x0044_b130, |_, _| word_ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0087_8080, &[level_manager, 0]);
        let addresses: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            addresses,
            [0x0087_8080, 0x0044_b130, 0x0040_1020, 0x0082_f1f0]
        );
    }

    #[test]
    fn limit_word_accessors() {
        let mut e = Engine::new();
        e.map(0x011a_95fc, 4);
        e.call(0x0087_81e0, &[0x1234]);
        assert_eq!(e.mem.u32(0x011a_95fc), 0x1234);
        assert_eq!(e.call(0x0087_81f0, &[]).u32(), 0x1234);
    }

    fn scope_doubles(e: &mut Engine) {
        e.map(0x011a_95fc, 4);
        e.map(0x0120_2d98, 4);
        e.mem.set_u32(0x0120_2d98, 0x0500_0000);
        for address in [
            0x00c3_e310,
            0x00c3_e340,
            0x0065_2190,
            0x004a_0370,
            0x004a_03c0,
            0x0070_3980,
            0x00a5_b460,
            0x00aa_7030,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x0065_2160, |_, _| word_ret(0x1234_5601));
        e.register(0x0043_c4b0, |_, _| word_ret(0x0600_0000));
    }

    #[test]
    fn opening_a_scope_records_the_flags_and_raises_the_limit() {
        let mut e = Engine::new();
        scope_doubles(&mut e);
        e.mem.set_u32(0x011a_95fc, 77);
        let scope = e.mem.alloc(0x0c);
        e.call_log = Some(vec![]);
        e.call(0x0087_8160, &[scope, 1, 2, 1]);
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![
                (0x0087_8160, vec![scope, 1, 2, 1]),
                (0x00c3_e310, vec![0x0500_0000]),
                (0x0065_2160, vec![]),
                (0x0043_c4b0, vec![]),
                (0x004a_0370, vec![0x0600_0000]),
                (0x0070_3980, vec![]),
            ]
        );
        assert_eq!(e.mem.u8(scope + 3), 1);
        assert_eq!(e.mem.u8(scope + 4), 2);
        assert_eq!(e.mem.u8(scope + 5), 1); // Low byte of 00652160's result.
        assert_eq!(e.mem.u32(scope + 8), 77);
        assert_eq!(e.mem.u32(0x011a_95fc), 0x7fff_ffff);

        // No pair and no result kept when the flags are zero.
        let plain = e.mem.alloc(0x0c);
        e.mem.write(plain, &[0xff; 0x0c]);
        e.call_log = Some(vec![]);
        e.call(0x0087_8160, &[plain, 0, 9, 0]);
        let addresses: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(addresses, [0x0087_8160, 0x00c3_e310, 0x0070_3980]);
        assert_eq!(e.mem.u8(plain + 3), 0);
        assert_eq!(e.mem.u8(plain + 4), 9);
        assert_eq!(e.mem.u8(plain + 5), 0);
    }

    #[test]
    fn closing_a_scope_undoes_what_was_opened() {
        let mut e = Engine::new();
        scope_doubles(&mut e);
        e.mem.set_u32(0x011a_95fc, 0x7fff_ffff);
        let scope = e.mem.alloc(0x0c);
        e.mem.set_u8(scope + 3, 1);
        e.mem.set_u8(scope + 5, 1);
        e.mem.set_u32(scope + 8, 77);
        e.call_log = Some(vec![]);
        e.call(0x0087_8200, &[scope]);
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![
                (0x0087_8200, vec![scope]),
                (0x00a5_b460, vec![]),
                (0x00aa_7030, vec![]),
                (0x0065_2190, vec![]),
                (0x0043_c4b0, vec![]),
                (0x004a_03c0, vec![0x0600_0000]),
                (0x00c3_e340, vec![0x0500_0000]),
            ]
        );
        assert_eq!(e.mem.u32(0x011a_95fc), 77);

        // Nothing was started: only the unconditional calls run.
        let idle = e.mem.alloc(0x0c);
        e.call_log = Some(vec![]);
        e.call(0x0087_8200, &[idle]);
        let addresses: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            addresses,
            [0x0087_8200, 0x00a5_b460, 0x00aa_7030, 0x00c3_e340]
        );
        assert_eq!(e.mem.u32(0x011a_95fc), 0);
    }
}
