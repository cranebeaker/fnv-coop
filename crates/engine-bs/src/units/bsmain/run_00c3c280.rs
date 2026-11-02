//! `BSMain/run_00c3c280` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `MemoryManager` singleton.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager` allocation (`00aa3e40`, thiscall on the singleton: size).
const ALLOCATE: u32 = 0x00aa_3e40;
/// `MemoryManager` deallocation (`00aa4060`, thiscall on the singleton: the block).
const DEALLOCATE: u32 = 0x00aa_4060;
/// The `MessageHandler` constructor (`00c3bcf0`, thiscall, `RET 4`: the flag byte).
const MESSAGE_HANDLER_CONSTRUCTOR: u32 = 0x00c3_bcf0;
/// The `MessageHandler` destructor (`00c3bda0`, thiscall).
const MESSAGE_HANDLER_DESTRUCTOR: u32 = 0x00c3_bda0;
/// The global holding the message handler.
const MESSAGE_HANDLER: u32 = 0x0120_2d74;

// Translated from 00c3c430 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MessageHandler`'s scalar deleting destructor (its vtable's class by RTTI): runs the
/// destructor (`00c3bda0`) and frees `this` through the memory manager when bit 0 of `flags`
/// is set. Returns `this`.
pub fn fn_00c3c430(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(MESSAGE_HANDLER_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00c3c460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3c460 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f109cb, state variable) is not translated
/// Makes the message handler: allocates 8 bytes from the memory manager, constructs a
/// `MessageHandler` in them (`00c3bcf0` with 0) unless the allocation failed, and stores the
/// result (or null) in `01202d74`.
pub fn fn_00c3c460(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 8u32]).u32();
    let handler = if block == 0 {
        0
    } else {
        e.call(MESSAGE_HANDLER_CONSTRUCTOR, &args![block, 0u32])
            .u32()
    };
    e.set_global::<u32>(MESSAGE_HANDLER, handler);
}

/// The message-box hook's handle (`01202d70`, `HHOOK`).
const HOOK_HANDLE: u32 = 0x0120_2d70;
/// Imported `user32` functions, by their import slots (`stdcall`): `UnhookWindowsHookEx`
/// (`00fdf2c8`), `GetWindowTextA` (`00fdf2cc`), `GetClassNameA` (`00fdf2d0`),
/// `SetWindowTextA` (`00fdf2dc`), `EnumChildWindows` (`00fdf2e4`) and `CallNextHookEx`
/// (`00fdf30c`).
const UNHOOK_WINDOWS_HOOK_EX: u32 = 0x00fd_f2c8;
const GET_WINDOW_TEXT_A: u32 = 0x00fd_f2cc;
const GET_CLASS_NAME_A: u32 = 0x00fd_f2d0;
const SET_WINDOW_TEXT_A: u32 = 0x00fd_f2dc;
const ENUM_CHILD_WINDOWS: u32 = 0x00fd_f2e4;
const CALL_NEXT_HOOK_EX: u32 = 0x00fd_f30c;
/// `memset` (`00ec61c0`) and `_mbsicmp` (`00ec858a`), both `cdecl`.
const MEMSET: u32 = 0x00ec_61c0;
const MBSICMP: u32 = 0x00ec_858a;
/// `"Button"` (`0105f010`), `"Cancel"` (`01059d30`) and `"Yes to all"` (`010c14d8`).
const BUTTON_TEXT: u32 = 0x0105_f010;
const CANCEL_TEXT: u32 = 0x0105_9d30;
const YES_TO_ALL_TEXT: u32 = 0x010c_14d8;
/// `HCBT_ACTIVATE`, the hook code the hook acts on.
const HOOK_ACTIVATE: i32 = 5;

// Translated from 00c3c280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the message-box hook (unnamed in the engine map; no arguments): when the handle
/// at `01202d70` is set, `UnhookWindowsHookEx` of it and the handle cleared.
pub fn fn_00c3c280(e: &mut Engine) {
    if e.mem.u32(HOOK_HANDLE) != 0 {
        let hook = e.mem.u32(HOOK_HANDLE);
        e.call(UNHOOK_WINDOWS_HOOK_EX, &args![hook]);
        e.mem.set_u32(HOOK_HANDLE, 0);
    }
}

// Translated from 00c3c2b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3c2b0 - the /GS stack cookie check (00ec408c) is not translated
/// The child-window callback of the message-box hook (unnamed in the engine map;
/// `stdcall`, `RET 8`: window, address of the found window). For a window whose class name
/// (`GetClassNameA` into a cleared 0x104-byte stack buffer) is `"Button"` and whose text
/// (`GetWindowTextA`, the buffer cleared again) is non-empty and `"Cancel"` (`_mbsicmp`, so
/// in any case), the window is stored and the answer is 0 (stop); otherwise 1.
pub fn fn_00c3c2b0(e: &mut Engine, window: u32, found: Ptr) -> u32 {
    let mut keep_going = 1u8;
    if window != 0 {
        e.with_stack(0x104, |e, text| {
            e.call(MEMSET, &args![text, 0u32, 0x104u32]);
            if e.call(GET_CLASS_NAME_A, &args![window, text, 0x104u32])
                .u32()
                == 0
            {
                return;
            }
            if e.call(MBSICMP, &args![text, BUTTON_TEXT]).u32() != 0 {
                return;
            }
            e.call(MEMSET, &args![text, 0u32, 0x104u32]);
            if e.call(GET_WINDOW_TEXT_A, &args![window, text, 0x104u32])
                .u32()
                == 0
            {
                return;
            }
            if e.mem.u8(text.addr()) == 0 {
                return;
            }
            if e.call(MBSICMP, &args![text, CANCEL_TEXT]).u32() == 0 {
                e.mem.set_u32(found.addr(), window);
                keep_going = 0;
            }
        });
    }
    u32::from(keep_going)
}

// Translated from 00c3c3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The message-box hook (unnamed in the engine map; `stdcall`, `RET 0xC`: code, wparam,
/// lparam). For code 5 (`HCBT_ACTIVATE`) the activated window's children are searched for
/// the `"Cancel"` button (`EnumChildWindows` with [`fn_00c3c2b0`]); a found button is
/// renamed `"Yes to all"` (`SetWindowTextA`) and the hook removed ([`fn_00c3c280`]). Every
/// code is passed on (`CallNextHookEx` with the handle at `01202d70`), whose answer is
/// returned.
pub fn fn_00c3c3a0(e: &mut Engine, code: i32, wparam: u32, lparam: u32) -> u32 {
    // A negative code goes straight to `CallNextHookEx`, as every other code does after
    // the activation handling.
    if code == HOOK_ACTIVATE {
        e.with_stack(4, |e, button| {
            e.mem.set_u32(button.addr(), 0);
            e.call(ENUM_CHILD_WINDOWS, &args![wparam, 0x00c3_c2b0u32, button]);
            let found = e.mem.u32(button.addr());
            if found != 0 {
                e.call(SET_WINDOW_TEXT_A, &args![found, YES_TO_ALL_TEXT]);
                fn_00c3c280(e);
            }
        });
    }
    let hook = e.mem.u32(HOOK_HANDLE);
    e.call(CALL_NEXT_HOOK_EX, &args![hook, code, wparam, lparam])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_c280, fn_00c3c280()),
        entry!(0x00c3_c2b0, fn_00c3c2b0(u32, Ptr) -> u32),
        entry!(0x00c3_c3a0, fn_00c3c3a0(i32, u32, u32) -> u32),
        entry!(0x00c3_c430, fn_00c3c430(Ptr, u32) -> Ptr),
        entry!(0x00c3_c460, fn_00c3c460()),
    ]
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn log_of(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != top)
            .collect()
    }

    #[test]
    fn deleting_destructor_frees_on_bit_0() {
        for (flags, frees) in [(0u32, false), (2, false), (1, true), (3, true)] {
            let mut e = Engine::new();
            e.register(MESSAGE_HANDLER_DESTRUCTOR, |_, _| Ret::default());
            e.register(DEALLOCATE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00c3_c430, &args![0x4000u32, flags]).u32(), 0x4000);
            let mut want = vec![(MESSAGE_HANDLER_DESTRUCTOR, vec![0x4000])];
            if frees {
                want.push((DEALLOCATE, vec![MEMORY_MANAGER, 0x4000]));
            }
            assert_eq!(log_of(&mut e, 0x00c3_c430), want);
        }
    }

    #[test]
    fn makes_the_handler_or_stores_null() {
        for block in [0u32, 0x5000] {
            let mut e = Engine::new();
            e.map(MESSAGE_HANDLER, 4);
            e.mem.set_u32(MESSAGE_HANDLER, 0xffff);
            e.register_double(ALLOCATE, move |_, _| block.into_ret());
            e.register(MESSAGE_HANDLER_CONSTRUCTOR, |_, a| (a[0] + 1).into_ret());
            e.call_log = Some(vec![]);
            e.call(0x00c3_c460, &args![]);
            let mut want = vec![(ALLOCATE, vec![MEMORY_MANAGER, 8])];
            if block != 0 {
                want.push((MESSAGE_HANDLER_CONSTRUCTOR, vec![block, 0]));
            }
            assert_eq!(log_of(&mut e, 0x00c3_c460), want);
            let stored = if block == 0 { 0 } else { block + 1 };
            assert_eq!(e.mem.u32(MESSAGE_HANDLER), stored);
        }
    }
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    fn hook_engine() -> Engine {
        let mut e = Engine::new();
        e.map(HOOK_HANDLE, 4);
        e.mem.set_u32(HOOK_HANDLE, 0x77);
        e.register(UNHOOK_WINDOWS_HOOK_EX, |_, _| 1u32.into_ret());
        e.register(CALL_NEXT_HOOK_EX, |_, a| {
            a[1].wrapping_add(0x100).into_ret()
        });
        e.register(SET_WINDOW_TEXT_A, |_, _| 1u32.into_ret());
        e
    }

    #[test]
    fn unhooking_clears_the_handle_once() {
        let mut e = hook_engine();
        e.call_log = Some(vec![]);
        e.call(0x00c3_c280, &args![]);
        e.call(0x00c3_c280, &args![]);
        let log = e.call_log.take().unwrap();
        let unhooks: Vec<_> = log
            .iter()
            .filter(|c| c.0 == UNHOOK_WINDOWS_HOOK_EX)
            .collect();
        assert_eq!(unhooks.len(), 1);
        assert_eq!(unhooks[0].1, vec![0x77]);
        assert_eq!(e.mem.u32(HOOK_HANDLE), 0);
    }

    /// Runs the child callback with the given class name and window text.
    fn callback(class: &'static [u8], text: &'static [u8]) -> (u32, u32) {
        let mut e = Engine::new();
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, 0);
            }
            a[0].into_ret()
        });
        e.register_double(GET_CLASS_NAME_A, move |e, a| {
            e.mem.set_cstr(a[1], class);
            (class.len() as u32).into_ret()
        });
        e.register_double(GET_WINDOW_TEXT_A, move |e, a| {
            e.mem.set_cstr(a[1], text);
            (text.len() as u32).into_ret()
        });
        e.map(BUTTON_TEXT, 8);
        e.mem.set_cstr(BUTTON_TEXT, b"Button");
        e.map(CANCEL_TEXT, 8);
        e.mem.set_cstr(CANCEL_TEXT, b"Cancel");
        e.register(MBSICMP, |e, a| {
            let left = e.mem.cstr(a[0]).to_ascii_lowercase();
            let right = e.mem.cstr(a[1]).to_ascii_lowercase();
            u32::from(left != right).into_ret()
        });
        let found = e.mem.alloc(4);
        let answer = e.call(0x00c3_c2b0, &args![0x55u32, found]).u32();
        (answer, e.mem.u32(found))
    }

    #[test]
    fn only_the_cancel_button_is_found() {
        assert_eq!(callback(b"button", b"CANCEL"), (0, 0x55));
        assert_eq!(callback(b"Button", b"OK"), (1, 0));
        assert_eq!(callback(b"Static", b"Cancel"), (1, 0));
        assert_eq!(callback(b"Button", b""), (1, 0));
        assert_eq!(callback(b"", b"Cancel"), (1, 0));
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_c2b0, &args![0u32, 0u32]).u32(), 1);
    }

    #[test]
    fn activation_renames_the_found_button_and_unhooks() {
        let mut e = hook_engine();
        e.register(ENUM_CHILD_WINDOWS, |e, a| {
            assert_eq!(a[1], 0x00c3_c2b0);
            e.mem.set_u32(a[2], 0x99);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00c3_c3a0, &args![5u32, 0x44u32, 0x33u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(answer, 0x105);
        assert!(log.contains(&(SET_WINDOW_TEXT_A, vec![0x99, YES_TO_ALL_TEXT])));
        assert!(log.contains(&(UNHOOK_WINDOWS_HOOK_EX, vec![0x77])));
        assert!(log.contains(&(CALL_NEXT_HOOK_EX, vec![0, 5, 0x44, 0x33])));
    }

    #[test]
    fn other_codes_are_only_passed_on() {
        let mut e = hook_engine();
        e.register(ENUM_CHILD_WINDOWS, |_, _| 1u32.into_ret());
        for code in [-1i32, 0, 3] {
            e.call_log = Some(vec![]);
            let answer = e
                .call(0x00c3_c3a0, &args![code as u32, 0x44u32, 0x33u32])
                .u32();
            let log = e.call_log.take().unwrap();
            assert_eq!(answer, (code as u32).wrapping_add(0x100));
            assert_eq!(log.len(), 2);
            assert!(log.contains(&(CALL_NEXT_HOOK_EX, vec![0x77, code as u32, 0x44, 0x33])));
        }
        // Activation without a button: nothing renamed, the hook stays.
        e.call_log = Some(vec![]);
        e.call(0x00c3_c3a0, &args![5u32, 0x44u32, 0x33u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == SET_WINDOW_TEXT_A));
        assert_eq!(e.mem.u32(HOOK_HANDLE), 0x77);
    }
}
