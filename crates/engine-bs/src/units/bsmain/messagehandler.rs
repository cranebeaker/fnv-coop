//! `BSMain/messagehandler.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// The memory manager singleton and its `Deallocate` (`00aa4060`,
/// `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00c4a890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiFrustumPlanes,1024>::_Deallocate` (engine-map name; the
/// body is the generic one, so identical code of other element types may
/// share it): gives `buffer` back to the memory manager. `this` is not read.
pub fn bs_simple_array_ni_frustum_planes_deallocate(e: &mut Engine, _this: Ptr, buffer: u32) {
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, buffer]);
}

// Translated from 00c3c220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageHandler::Output` (engine map): a variadic `cdecl` function (its callers push a
/// handler word, a format and its values, and clean the stack themselves) whose body only
/// answers 0; none of its words is read.
pub fn message_handler_output(_e: &mut Engine, _words: &[u32]) -> u32 {
    0
}

/// The uniform form of [`message_handler_output`]: every word the caller pushed.
fn message_handler_output_entry(e: &mut Engine, words: &[u32]) -> Ret {
    message_handler_output(e, words).into_ret()
}

/// The vtable `00c3a220` gives the array it constructs (`010c117c`).
const SIMPLE_ARRAY_VTABLE_010C117C: u32 = 0x010c_117c;

// Translated from 00c3a350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<T,1024>` set-up of a pointer array (no map name; the body is the
/// template's initial set-up): clears `pBuffer`, `iSize` and `iReservedSize`, raises
/// `reserve` to `size` when it is smaller, allocates a buffer of `reserve` items through the
/// array's own allocator (vtable +0x04) and records it, then records `size`. The per-item
/// loop over the first `size` items only forms each item's address (pointer items need no
/// construction), so nothing is written there.
pub fn fn_00c3a350(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let mut reserve = reserve;
    if size > reserve {
        reserve = size;
    }
    if reserve != 0 {
        let buffer = e.vcall(this.addr(), 0x04, &args![reserve]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserve);
    }
    if size != 0 {
        for _ in 0..size {
            // The loop reads `pBuffer` for each item and forms the item's address only.
            let _ = e.get(this, BSSimpleArray::pBuffer);
        }
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 00c3a220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a `BSSimpleArray` of pointers (the engine map names it
/// `BSSimpleArray<ShadowSceneLight*,1024>::_InitialSetup`, but the body is the constructor):
/// puts the vtable `010c117c`, runs [`fn_00c3a350`] with no reserve and no items and
/// answers `this`.
pub fn fn_00c3a220(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), SIMPLE_ARRAY_VTABLE_010C117C);
    fn_00c3a350(e, this, 0, 0);
    this
}

// Translated from 00c3be40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageHandler::VSend` (engine map): `cdecl`, its one caller (`SettingWarning`) pushes
/// a message kind, a format and its `va_list`; the body reads none of them and answers 0.
pub fn message_handler_v_send(
    _e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
) -> u32 {
    0
}

// Translated from 00c3be50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, no callers found: sets up an exception frame and the stack cookie, then
/// answers 1 in AL (the rest of EAX holds the cookie the function computed) and leaves.
// Translation incomplete: 00c3be50 - its exception frame and /GS cookie check are not modelled
pub fn fn_00c3be50(_e: &mut Engine) -> bool {
    true
}

// Translated from 00c3c070 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method of the vtable around `010c14a8` (slot at `010c14b8`): `thiscall` with
/// one word, reads nothing and answers 0.
pub fn fn_00c3c070(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> u32 {
    0
}

// Translated from 00c3c080 (decompiled, FalloutNV.exe 1.4.0.525)
/// An empty function (no callers found): returns at once.
pub fn fn_00c3c080(_e: &mut Engine) {}

// Translated from 00c3c090 (decompiled, FalloutNV.exe 1.4.0.525)
/// An empty function (no callers found): returns at once.
pub fn fn_00c3c090(_e: &mut Engine) {}

// Translated from 00c3c0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An empty function (no callers found): returns at once.
pub fn fn_00c3c0a0(_e: &mut Engine) {}

// Translated from 00c3c0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An empty function (no callers found): returns at once.
pub fn fn_00c3c0b0(_e: &mut Engine) {}

/// The 0x18 flag bytes `00c3c0c0` clears (`011f6370`) and the byte after them it clears
/// too (`011af49c`).
const FLAG_BYTES_011F6370: u32 = 0x011f_6370;
const FLAG_BYTE_011AF49C: u32 = 0x011a_f49c;

// Translated from 00c3c0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, no callers found: clears the 0x18 bytes at `011f6370` one by one, then the
/// byte at `011af49c`.
pub fn fn_00c3c0c0(e: &mut Engine) {
    for i in 0..0x18u32 {
        e.mem.set_u8(FLAG_BYTES_011F6370 + i, 0);
    }
    e.mem.set_u8(FLAG_BYTE_011AF49C, 0);
}

/// The message-box callback word (`01202d5c`, `cdecl`, three words), the hook handle
/// `00c3c250` installs (`01202d70`) and the flag `00c3c180`/`00c3c1c0` raise around their
/// box (`01202d55`).
const MESSAGE_BOX_CALLBACK: u32 = 0x0120_2d5c;
const MESSAGE_BOX_HOOK: u32 = 0x0120_2d70;
const MESSAGE_BOX_ACTIVE: u32 = 0x0120_2d55;
/// `GetCurrentThreadId` and `SetWindowsHookExA` (import slots, as Ghidra names them).
const IMPORT_GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
const IMPORT_SET_WINDOWS_HOOK_EX_A: u32 = 0x00fd_f2d8;
/// The hook procedure `00c3c250` installs.
const HOOK_PROCEDURE: u32 = 0x00c3_c3a0;
/// The function `00c3c100` calls after a hooked box (`BSMain` range unit, untranslated here).
const AFTER_HOOKED_BOX: u32 = 0x00c3_c280;
/// The caption string both confirmation boxes pass (`010c14d0`, "Confirm").
const CONFIRM_CAPTION: u32 = 0x010c_14d0;

// Translated from 00c3c250 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: when no hook is installed yet (`01202d70` is 0), installs hook procedure
/// `00c3c3a0` of kind 5 for the current thread (`SetWindowsHookExA(5, 00c3c3a0, 0,
/// GetCurrentThreadId())`) and keeps the handle in `01202d70`.
pub fn fn_00c3c250(e: &mut Engine) {
    if e.global::<u32>(MESSAGE_BOX_HOOK) == 0 {
        let thread = e.call(IMPORT_GET_CURRENT_THREAD_ID, &args![]).u32();
        let hook = e
            .call(
                IMPORT_SET_WINDOWS_HOOK_EX_A,
                &args![5u32, HOOK_PROCEDURE, 0u32, thread],
            )
            .u32();
        e.set_global::<u32>(MESSAGE_BOX_HOOK, hook);
    }
}

// Translated from 00c3c100 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`cdecl`, three words): when a message-box callback is set (`01202d5c`),
/// calls it with `text`, `caption` and `flags` and answers its result (0 without a
/// callback). With any of the low two flag bits set, it first installs the hook
/// ([`fn_00c3c250`]); when a hook handle then exists, flag 0x1000 is cleared and 0x2000 set
/// for the call, and `00c3c280` runs after it.
pub fn fn_00c3c100(e: &mut Engine, text: u32, caption: u32, flags: u32) -> u32 {
    let mut result = 0;
    let callback = e.global::<u32>(MESSAGE_BOX_CALLBACK);
    if callback != 0 {
        let mut hooked = false;
        let mut flags = flags;
        if flags & 3 != 0 {
            fn_00c3c250(e);
            if e.global::<u32>(MESSAGE_BOX_HOOK) != 0 {
                flags = (flags & 0xffff_efff) | 0x2000;
                hooked = true;
            }
        }
        result = e.call(callback, &args![text, caption, flags]).u32();
        if hooked {
            e.call(AFTER_HOOKED_BOX, &args![]);
        }
    }
    result
}

// Translated from 00c3c180 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`cdecl`, one word): raises the flag at `01202d55`, shows `text` through
/// [`fn_00c3c100`] with the caption "Confirm" (`010c14d0`) and flags 0x2034, lowers the
/// flag and answers the box's result.
pub fn fn_00c3c180(e: &mut Engine, text: u32) -> u32 {
    e.set_global::<u8>(MESSAGE_BOX_ACTIVE, 1);
    let result = fn_00c3c100(e, text, CONFIRM_CAPTION, 0x2034);
    e.set_global::<u8>(MESSAGE_BOX_ACTIVE, 0);
    result
}

// Translated from 00c3c1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`cdecl`, one word): as [`fn_00c3c180`] with flags 0x2033.
pub fn fn_00c3c1c0(e: &mut Engine, text: u32) -> u32 {
    e.set_global::<u8>(MESSAGE_BOX_ACTIVE, 1);
    let result = fn_00c3c100(e, text, CONFIRM_CAPTION, 0x2033);
    e.set_global::<u8>(MESSAGE_BOX_ACTIVE, 0);
    result
}

// Translated from 00c3c200 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method of the vtable around `010c14a8` (slot at `010c14b4`): takes no word
/// besides `this`, reads nothing and answers 0.
pub fn fn_00c3c200(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_a350, fn_00c3a350(Ptr<BSSimpleArray>, u32, u32)),
        entry!(
            0x00c3_a220,
            fn_00c3a220(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x00c3_be40, message_handler_v_send(u32, u32, u32) -> u32),
        entry!(0x00c3_be50, fn_00c3be50() -> bool),
        entry!(0x00c3_c070, fn_00c3c070(Ptr, u32) -> u32),
        entry!(0x00c3_c080, fn_00c3c080()),
        entry!(0x00c3_c090, fn_00c3c090()),
        entry!(0x00c3_c0a0, fn_00c3c0a0()),
        entry!(0x00c3_c0b0, fn_00c3c0b0()),
        entry!(0x00c3_c0c0, fn_00c3c0c0()),
        entry!(0x00c3_c250, fn_00c3c250()),
        entry!(0x00c3_c100, fn_00c3c100(u32, u32, u32) -> u32),
        entry!(0x00c3_c180, fn_00c3c180(u32) -> u32),
        entry!(0x00c3_c1c0, fn_00c3c1c0(u32) -> u32),
        entry!(0x00c3_c200, fn_00c3c200(Ptr) -> u32),
        entry!(
            0x00c4_a890,
            bs_simple_array_ni_frustum_planes_deallocate(Ptr, u32)
        ),
        (0x00c3_c220, message_handler_output_entry as AbiFn),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deallocate_gives_the_buffer_to_the_memory_manager() {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_a890, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|c| c.0 == MEMORY_MANAGER_DEALLOCATE)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, [MEMORY_MANAGER, 0x2000]);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn output_answers_zero_whatever_it_is_given() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_c220, &args![]).u32(), 0);
        let words = args![0x0118_6084u32, 0x0101_7acc_u32, 1u32, 2u32, 3u32];
        assert_eq!(e.call(0x00c3_c220, &words).u32(), 0);
    }
}

#[cfg(test)]
mod n083_tests {
    use super::*;

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
    fn initial_setup_allocates_the_larger_of_reserve_and_size() {
        let mut e = Engine::new();
        let a = e.new_object::<BSSimpleArray>();
        e.put_vtable(0x0300_0000, &[0, 0x0300_1000]);
        e.mem.set_u32(a.addr(), 0x0300_0000);
        e.set(a, BSSimpleArray::iSize, 9);
        e.register(0x0300_1000, |_, w| Ret {
            eax: 0x0400_0000 + w[1],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c3_a350, &args![a, 2u32, 5u32]);
        assert_eq!(calls_to(&e, 0x0300_1000), [vec![a.addr(), 5]]);
        assert_eq!(e.get(a, BSSimpleArray::pBuffer), 0x0400_0005);
        assert_eq!(e.get(a, BSSimpleArray::iReservedSize), 5);
        assert_eq!(e.get(a, BSSimpleArray::iSize), 5);
        // Nothing to reserve: no allocation, everything cleared.
        e.call_log = Some(vec![]);
        e.call(0x00c3_a350, &args![a, 0u32, 0u32]);
        assert!(calls_to(&e, 0x0300_1000).is_empty());
        assert_eq!(e.get(a, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(a, BSSimpleArray::iReservedSize), 0);
        assert_eq!(e.get(a, BSSimpleArray::iSize), 0);
        // A reserve without items keeps the size 0.
        e.call(0x00c3_a350, &args![a, 3u32, 0u32]);
        assert_eq!(e.get(a, BSSimpleArray::iReservedSize), 3);
        assert_eq!(e.get(a, BSSimpleArray::iSize), 0);
    }

    #[test]
    fn constructor_puts_the_vtable_and_clears_the_array() {
        let mut e = Engine::new();
        let a = e.new_object::<BSSimpleArray>();
        e.set(a, BSSimpleArray::pBuffer, 7);
        e.set(a, BSSimpleArray::iSize, 7);
        e.set(a, BSSimpleArray::iReservedSize, 7);
        let r = e.call(0x00c3_a220, &args![a]).u32();
        assert_eq!(r, a.addr());
        assert_eq!(e.mem.u32(a.addr()), SIMPLE_ARRAY_VTABLE_010C117C);
        assert_eq!(e.get(a, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(a, BSSimpleArray::iSize), 0);
        assert_eq!(e.get(a, BSSimpleArray::iReservedSize), 0);
    }

    #[test]
    fn v_send_answers_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_be40, &args![1u32, 2u32, 3u32]).u32(), 0);
    }

    #[test]
    fn framed_function_answers_one() {
        let mut e = Engine::new();
        assert!(e.call(0x00c3_be50, &args![]).bool());
    }

    #[test]
    fn virtual_stubs_answer_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_c070, &args![0x1000u32, 5u32]).u32(), 0);
        assert_eq!(e.call(0x00c3_c200, &args![0x1000u32]).u32(), 0);
    }

    #[test]
    fn empty_functions_return() {
        let mut e = Engine::new();
        for a in [0x00c3_c080u32, 0x00c3_c090, 0x00c3_c0a0, 0x00c3_c0b0] {
            e.call(a, &args![]);
        }
    }

    #[test]
    fn clears_the_flag_bytes() {
        let mut e = Engine::new();
        e.map(FLAG_BYTES_011F6370, 0x20);
        e.map(FLAG_BYTE_011AF49C, 4);
        for i in 0..0x20u32 {
            e.mem.set_u8(FLAG_BYTES_011F6370 + i, 0xaa);
        }
        e.mem.set_u8(FLAG_BYTE_011AF49C, 1);
        e.call(0x00c3_c0c0, &args![]);
        for i in 0..0x18u32 {
            assert_eq!(e.mem.u8(FLAG_BYTES_011F6370 + i), 0);
        }
        assert_eq!(e.mem.u8(FLAG_BYTES_011F6370 + 0x18), 0xaa);
        assert_eq!(e.mem.u8(FLAG_BYTE_011AF49C), 0);
    }

    fn box_globals(e: &mut Engine) {
        e.map(0x0120_2d40, 0x40);
        e.register(IMPORT_GET_CURRENT_THREAD_ID, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        e.register(IMPORT_SET_WINDOWS_HOOK_EX_A, |_, _| Ret {
            eax: 0x5555,
            ..Ret::default()
        });
        e.register(AFTER_HOOKED_BOX, |_, _| Ret::default());
        e.register(0x0300_2000, |e, _| Ret {
            eax: 6 + u32::from(e.global::<u8>(MESSAGE_BOX_ACTIVE)) * 0x100,
            ..Ret::default()
        });
    }

    #[test]
    fn hook_is_installed_once() {
        let mut e = Engine::new();
        box_globals(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c3_c250, &args![]);
        assert_eq!(
            calls_to(&e, IMPORT_SET_WINDOWS_HOOK_EX_A),
            [vec![5, HOOK_PROCEDURE, 0, 0x77]]
        );
        assert_eq!(e.global::<u32>(MESSAGE_BOX_HOOK), 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x00c3_c250, &args![]);
        assert!(calls_to(&e, IMPORT_SET_WINDOWS_HOOK_EX_A).is_empty());
    }

    #[test]
    fn message_box_goes_through_the_callback() {
        let mut e = Engine::new();
        box_globals(&mut e);
        // No callback: 0 and nothing called.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_c100, &args![1u32, 2u32, 3u32]).u32(), 0);
        assert!(calls_to(&e, IMPORT_SET_WINDOWS_HOOK_EX_A).is_empty());
        e.set_global::<u32>(MESSAGE_BOX_CALLBACK, 0x0300_2000);
        // Low bits clear: flags passed unchanged, no hook.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_c100, &args![1u32, 2u32, 0x1010u32]).u32(), 6);
        assert_eq!(calls_to(&e, 0x0300_2000), [vec![1, 2, 0x1010]]);
        assert!(calls_to(&e, AFTER_HOOKED_BOX).is_empty());
        // Low bits set and the hook installed: 0x1000 off, 0x2000 on, then 00c3c280.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_c100, &args![1u32, 2u32, 0x1001u32]).u32(), 6);
        assert_eq!(calls_to(&e, 0x0300_2000), [vec![1, 2, 0x2001]]);
        assert_eq!(calls_to(&e, AFTER_HOOKED_BOX).len(), 1);
        // The hook could not be installed: flags unchanged, no 00c3c280.
        e.set_global::<u32>(MESSAGE_BOX_HOOK, 0);
        e.register(IMPORT_SET_WINDOWS_HOOK_EX_A, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c3_c100, &args![1u32, 2u32, 0x1002u32]);
        assert_eq!(calls_to(&e, 0x0300_2000), [vec![1, 2, 0x1002]]);
        assert!(calls_to(&e, AFTER_HOOKED_BOX).is_empty());
    }

    #[test]
    fn confirm_boxes_raise_the_flag_around_the_box() {
        let mut e = Engine::new();
        box_globals(&mut e);
        e.set_global::<u32>(MESSAGE_BOX_CALLBACK, 0x0300_2000);
        e.set_global::<u32>(MESSAGE_BOX_HOOK, 0x5555);
        for (addr, flags) in [(0x00c3_c180u32, 0x2034u32), (0x00c3_c1c0, 0x2033)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![0x0300_3000u32]).u32(), 0x106);
            assert_eq!(
                calls_to(&e, 0x0300_2000),
                [vec![0x0300_3000, CONFIRM_CAPTION, flags]]
            );
            assert_eq!(e.global::<u8>(MESSAGE_BOX_ACTIVE), 0);
        }
    }
}
