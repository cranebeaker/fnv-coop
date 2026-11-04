//! `fallout/misc/main.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! This is the unit of the game's `main` (the Windows entry point, `0086a850`) and of the
//! helpers around it: the shader timer, the message-box wrapper, the renderer's off-screen
//! frame guards, small global setters and the deleting destructors of the objects `main`
//! creates.
//!
//! The unit is split into part files by address range (`main_p2`, ...); the constants and
//! helpers a part may need are `pub(crate)` here.
//!
//! Not translated: the compiler's stack-cookie check at the end of `main`
//! (`__security_check_cookie`, `00ec408c`) and the `INT3` of the debugger-present branch of the
//! message-menu callback.
//!
//! Calling-convention note: the decompiler hangs a pushed argument on the wrong call all over
//! `main` (`PUSH x; CALL getter; MOV ECX,EAX; CALL method` is `method(getter(), x)`), so the
//! arguments below are read from the disassembly and checked against each callee's `RET n`.

#[allow(unused_imports)]
use crate::prelude::*;

// ---------------------------------------------------------------------------------------------
// Callees and globals shared by the functions of this file.
// ---------------------------------------------------------------------------------------------

/// `Error` (Xbox PDB name): the game's logging function. In this build its body is empty
/// (`PUSH EBP; MOV EBP,ESP; POP EBP; RET`); the calls are kept, with their arguments.
pub(crate) const ERROR_LOG: u32 = 0x0040_fbe0;
/// The renderer singleton's getter (reads the global at `011f4748`).
pub(crate) const GET_RENDERER: u32 = 0x0043_c4b0;
/// Setting object method: address of the setting's one-byte value.
pub(crate) const SETTING_BYTE_PTR: u32 = 0x0040_8d60;
/// Setting object method: address of the setting's 32-bit value.
pub(crate) const SETTING_INT_PTR: u32 = 0x0043_d4d0;
/// Frees a heap object (cdecl, one argument).
pub(crate) const FREE_OBJECT: u32 = 0x0040_1030;
/// Allocates a heap block (cdecl, size).
pub(crate) const ALLOCATE_OBJECT: u32 = 0x0040_1000;
/// `Interface::IsInMenuMode` (Xbox PDB).
pub(crate) const IS_IN_MENU_MODE: u32 = 0x0070_2360;
/// `Calendar::GetHour` (Xbox PDB), called on the calendar at [`CALENDAR`].
pub(crate) const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
/// `BSTimer::Disable` and `BSTimer::Enable` (Xbox PDB), called on [`TIMER`].
pub(crate) const TIMER_DISABLE: u32 = 0x00aa_50a0;
pub(crate) const TIMER_ENABLE: u32 = 0x00aa_5040;
/// Methods of the timer at [`TIMER`]: a `float` in ST0 (`0084d030`) and a millisecond count in
/// EAX (`00825c00`).
pub(crate) const TIMER_GET_SECONDS: u32 = 0x0084_d030;
pub(crate) const TIMER_GET_MILLISECONDS: u32 = 0x0082_5c00;
/// `BaseProcess::GetCurrentProcedureIndex` (Xbox PDB), called on the main object: the id of the
/// thread that runs the main loop.
pub(crate) const GET_MAIN_THREAD_ID: u32 = 0x0044_edb0;
/// Called on the main object: a pointer to its helper object (or 0).
pub(crate) const MAIN_OBJECT_GET_HELPER: u32 = 0x0087_7720;
/// Called on the helper: non-zero when bit 3 of the word at +4 is set.
pub(crate) const HELPER_HAS_FLAG: u32 = 0x0071_1e00;
/// Called on the helper with one argument (`1`).
pub(crate) const HELPER_SET_FLAG: u32 = 0x00a2_3c00;
/// `Interface::GetMessageMenuResult` (Xbox PDB): the button used in the warning menu.
pub(crate) const GET_MESSAGE_MENU_RESULT: u32 = 0x0070_3fa0;
/// Closes the warning menu.
pub(crate) const CLOSE_MESSAGE_MENU: u32 = 0x0070_3fd0;
/// Opens the warning menu: text, two zero words, the result callback, a word, the context, two
/// floats, three pooled button texts and a word (cdecl, 12 words). Non-zero when it was opened.
pub(crate) const SHOW_MESSAGE_MENU: u32 = 0x0070_3e80;
/// Returns the text of a string-pool object (`__thiscall`, no argument).
pub(crate) const GET_POOLED_TEXT: u32 = 0x0040_3df0;
/// Returns the byte setting at `011c77b4` (AL).
pub(crate) const GET_WINDOW_SETTING: u32 = 0x0044_6e10;
/// Returns the global at `011c6fbc`.
pub(crate) const GET_WINDOW_STATE: u32 = 0x004d_c1e0;

/// The timer object [`shader_timer`], `BSTimer::Disable` and `BSTimer::Enable` work on.
pub(crate) const TIMER: u32 = 0x011f_6394;
/// The global `Calendar` instance.
pub(crate) const CALENDAR: u32 = 0x011d_e7b8;
/// `1000.0` (`double`).
const MILLISECONDS_PER_SECOND: u32 = 0x0101_7b70;
/// `60.0` (`double`).
const SIXTY: u32 = 0x0101_2638;
/// Setting (byte): when set, [`fn_0086a580`] answers without showing a box.
const SETTING_NO_MESSAGE_BOXES: u32 = 0x011d_eb3c;
/// The `Main` object (0xA4 bytes, built by `Main::Main`, `0086c160`); its first byte is the
/// "background loading thread is suspended" flag. Null before `main` creates it.
pub(crate) const MAIN_OBJECT: u32 = 0x011d_ea0c;
/// Byte flag: the warning menu may be used for the next message box.
const WARNING_MENU_AVAILABLE: u32 = 0x011a_3015;
/// Global returned by `0086a820`: the context word handed to the warning menu.
const WARNING_MENU_CONTEXT: u32 = 0x011f_638c;
/// The pooled button texts of the warning menu.
const BUTTON_TEXT_THIRD: u32 = 0x011d_12b4;
const BUTTON_TEXT_SECOND: u32 = 0x011d_0cb4;
const BUTTON_TEXT_FIRST: u32 = 0x011d_06f4;

/// Imports, called by the address of their import slot.
pub(crate) const API_CREATE_MUTEX: u32 = 0x00fd_f0fc;
pub(crate) const API_GET_LAST_ERROR: u32 = 0x00fd_f094;
pub(crate) const API_MESSAGE_BOX: u32 = 0x00fd_f2f4;
pub(crate) const API_GET_SYSTEM_METRICS: u32 = 0x00fd_f2b0;
pub(crate) const API_GET_SYSTEM_INFO: u32 = 0x00fd_f0f8;
pub(crate) const API_FIND_WINDOW: u32 = 0x00fd_f304;
pub(crate) const API_SET_FOREGROUND_WINDOW: u32 = 0x00fd_f300;
pub(crate) const API_GET_COMMAND_LINE: u32 = 0x00fd_f0f4;
pub(crate) const API_COMMAND_LINE_TO_ARGV: u32 = 0x00fd_f270;
pub(crate) const API_LOCAL_FREE: u32 = 0x00fd_f08c;
pub(crate) const API_SHELL_EXECUTE: u32 = 0x00fd_f278;
pub(crate) const API_LOAD_ICON: u32 = 0x00fd_f280;
pub(crate) const API_GET_STOCK_OBJECT: u32 = 0x00fd_f03c;
pub(crate) const API_REGISTER_CLASS: u32 = 0x00fd_f2b4;
pub(crate) const API_ADJUST_WINDOW_RECT: u32 = 0x00fd_f310;
pub(crate) const API_CREATE_WINDOW: u32 = 0x00fd_f2b8;
pub(crate) const API_PEEK_MESSAGE: u32 = 0x00fd_f29c;
pub(crate) const API_TRANSLATE_MESSAGE: u32 = 0x00fd_f298;
pub(crate) const API_DISPATCH_MESSAGE: u32 = 0x00fd_f294;
pub(crate) const API_GET_ACTIVE_WINDOW: u32 = 0x00fd_f2fc;
pub(crate) const API_SLEEP: u32 = 0x00fd_f104;
pub(crate) const API_GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
pub(crate) const API_GET_WINDOW_LONG: u32 = 0x00fd_f2e8;
pub(crate) const API_ADJUST_WINDOW_RECT_EX: u32 = 0x00fd_f2f8;
pub(crate) const API_SET_WINDOW_POS: u32 = 0x00fd_f2a4;
pub(crate) const API_IS_DEBUGGER_PRESENT: u32 = 0x00fd_f0f0;
pub(crate) const API_EXIT_PROCESS: u32 = 0x00fd_f0ec;

// ---------------------------------------------------------------------------------------------
// The renderer's off-screen frame guards (`NiRenderer`).
// ---------------------------------------------------------------------------------------------

/// `NiRenderer::m_eFrameState` (Xbox PDB) at +0x200 of the renderer.
const RENDERER_FRAME_STATE: u32 = 0x200;
/// The renderer's two pairs of parallel `NiTArray`s at +0xAD4 / +0xAE4 and +0xAF4 / +0xB04:
/// the first of a pair holds keys (addresses of registered functions), the second their data.
const KEY_ARRAY_A: u32 = 0xad4;
const DATA_ARRAY_A: u32 = 0xae4;
const KEY_ARRAY_B: u32 = 0xaf4;
const DATA_ARRAY_B: u32 = 0xb04;
/// Array methods: add an element given by the address of its value, returning its index
/// (`00877a50`); address of the element at an index (`00877a30`); element count (`00658930`,
/// the 16 bits at +0x0A); `SetAtGrow(index, address of the value)` (`00470000`); remove the
/// element at an index (`009e98d0`).
const ARRAY_ADD: u32 = 0x0087_7a50;
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
const ARRAY_COUNT: u32 = 0x0065_8930;
const ARRAY_SET_AT_GROW: u32 = 0x0047_0000;
const ARRAY_REMOVE_AT: u32 = 0x009e_98d0;

// Translated from 0086a480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShaderTimer` (Xbox PDB): the time value the shader system reads. Mode 0 is the game hour
/// in seconds, mode 1 the timer's seconds (0 while a menu is open), mode 2 the timer's
/// milliseconds as seconds (0 while a menu is open), mode 3 the timer's seconds; any other
/// mode gives 0.
pub fn shader_timer(e: &mut Engine, mode: i32) -> f32 {
    match mode {
        3 => e.call(TIMER_GET_SECONDS, &args![TIMER]).f32(),
        2 => {
            if e.call(IS_IN_MENU_MODE, &args![]).bool() {
                return 0.0;
            }
            let milliseconds = e.call(TIMER_GET_MILLISECONDS, &args![TIMER]).u32();
            let per_second: f64 = e.global(MILLISECONDS_PER_SECOND);
            (milliseconds as f64 / per_second) as f32
        }
        1 => {
            if e.call(IS_IN_MENU_MODE, &args![]).bool() {
                return 0.0;
            }
            e.call(TIMER_GET_SECONDS, &args![TIMER]).f32()
        }
        0 => {
            let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f32();
            let sixty: f64 = e.global(SIXTY);
            (hour as f64 * sixty * sixty) as f32
        }
        _ => 0.0,
    }
}

// Translated from 0086a530 (decompiled, FalloutNV.exe 1.4.0.525)
/// The result callback of the warning menu opened by [`fn_0086a580`]: results 1 and 3 log
/// "Warning Response was to Exit the Process" and end the process (a breakpoint first when a
/// debugger is attached, not translated); result 2 closes the menu.
pub fn fn_0086a530(e: &mut Engine) {
    let result = e.call(GET_MESSAGE_MENU_RESULT, &args![]).u8();
    match result {
        1 | 3 => {
            e.call(ERROR_LOG, &args![0x0108_28c0u32]);
            // `IsDebuggerPresent`, then `INT3` when it is non-zero: no equivalent here.
            e.call(API_IS_DEBUGGER_PRESENT, &args![]);
            // `ExitProcess` does not return.
            e.call(API_EXIT_PROCESS, &args![0u32]);
        }
        2 => {
            e.call(CLOSE_MESSAGE_MENU, &args![]);
        }
        _ => {}
    }
}

/// Shows a Windows message box. When the main object exists and the calling thread is not its
/// thread, the helper of the main object gets its flag set first (unless it has it).
fn show_windows_message_box(e: &mut Engine, text: u32, caption: u32, flags: u32) -> i32 {
    let main_object = e.global::<u32>(MAIN_OBJECT);
    if main_object != 0 {
        let thread = e.call(API_GET_CURRENT_THREAD_ID, &args![]).u32();
        let main_thread = e.call(GET_MAIN_THREAD_ID, &args![main_object]).u32();
        if thread != main_thread && e.call(MAIN_OBJECT_GET_HELPER, &args![main_object]).u32() != 0 {
            let helper = e.call(MAIN_OBJECT_GET_HELPER, &args![main_object]).u32();
            if e.call(HELPER_HAS_FLAG, &args![helper]).u32() == 0 {
                let helper = e.call(MAIN_OBJECT_GET_HELPER, &args![main_object]).u32();
                e.call(HELPER_SET_FLAG, &args![helper, 1u32]);
            }
        }
    }
    e.call(API_MESSAGE_BOX, &args![0u32, text, caption, flags])
        .i32()
}

/// Whether the window setting is on and the window state global is non-zero: the cases in
/// which [`fn_0086a580`] does not show a Windows message box.
fn message_box_suppressed(e: &mut Engine) -> bool {
    e.call(GET_WINDOW_SETTING, &args![]).bool() && e.call(GET_WINDOW_STATE, &args![]).u32() != 0
}

// Translated from 0086a580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The game's `MessageBox` wrapper (`text`, `caption`, `MB_*` flags; cdecl). Returns the button
/// id. With the no-message-boxes setting it answers without showing anything: 5 (`IDIGNORE`)
/// for `MB_ABORTRETRYIGNORE` and 6 (`IDYES`) otherwise. Otherwise the timer is disabled around
/// the box. When the warning menu is available and the main object has its helper, boxes of
/// styles other than `MB_OK` and `MB_ABORTRETRYIGNORE` are opened as the game's warning menu
/// (button texts for styles 3 and 4), answered by [`fn_0086a530`], and the menu is marked
/// unavailable meanwhile; if that fails, or in every other case, a Windows message box is
/// shown unless [`message_box_suppressed`], in which case the answer is 5 for style 2 and 6 for
/// styles 3 and 4 (6 is also the answer when the box is suppressed without the menu).
pub fn fn_0086a580(e: &mut Engine, text: u32, caption: u32, flags: u32) -> i32 {
    let style = (flags & 0xf) as u8;
    let setting = e
        .call(SETTING_BYTE_PTR, &args![SETTING_NO_MESSAGE_BOXES])
        .u32();
    if e.mem.u8(setting) != 0 {
        return if style == 2 { 5 } else { 6 };
    }
    e.call(TIMER_DISABLE, &args![TIMER]);
    let mut result: i32 = 6;

    let main_object = e.global::<u32>(MAIN_OBJECT);
    let use_menu = e.global::<u8>(WARNING_MENU_AVAILABLE) != 0
        && main_object != 0
        && e.call(MAIN_OBJECT_GET_HELPER, &args![main_object]).u32() != 0;

    if !use_menu {
        if !message_box_suppressed(e) {
            result = show_windows_message_box(e, text, caption, flags);
        }
        e.call(TIMER_ENABLE, &args![TIMER]);
        return result;
    }

    e.set_global(WARNING_MENU_AVAILABLE, 0u8);
    let mut first_text = 0u32;
    let mut second_text = 0u32;
    let mut third_text = 0u32;
    if style == 3 {
        third_text = e.call(GET_POOLED_TEXT, &args![BUTTON_TEXT_THIRD]).u32();
    }
    if style == 3 || style == 4 {
        second_text = e.call(GET_POOLED_TEXT, &args![BUTTON_TEXT_SECOND]).u32();
        first_text = e.call(GET_POOLED_TEXT, &args![BUTTON_TEXT_FIRST]).u32();
    }
    let opened = if style != 2 && style != 0 {
        let context = fn_0086a820(e);
        e.call(
            SHOW_MESSAGE_MENU,
            &args![
                text,
                0u32,
                0u32,
                0x0086_a530u32,
                0u32,
                context,
                0.0f32,
                0.0f32,
                second_text,
                first_text,
                third_text,
                0u32
            ],
        )
        .bool()
    } else {
        false
    };
    if !opened {
        if !message_box_suppressed(e) {
            result = show_windows_message_box(e, text, caption, flags);
        } else if style == 2 {
            result = 5;
        } else if style > 2 && style <= 4 {
            result = 6;
        }
    }
    e.set_global(WARNING_MENU_AVAILABLE, 1u8);
    e.call(TIMER_ENABLE, &args![TIMER]);
    result
}

// Translated from 0086a820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the global at `011f638c`: the context word handed to the warning menu.
pub fn fn_0086a820(e: &mut Engine) -> u32 {
    e.global(WARNING_MENU_CONTEXT)
}

// Translated from 0086a830 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `__thiscall` method with one stack argument that ignores both and returns 0.
pub fn fn_0086a830(_e: &mut Engine, _unused_0: u32, _unused_1: u32) -> u8 {
    0
}

// ---------------------------------------------------------------------------------------------
// `main`.
// ---------------------------------------------------------------------------------------------

/// Size of `main`'s stack frame.
const MAIN_FRAME_SIZE: u32 = 0x378;

// `main`'s locals, as offsets from its frame pointer.
const LOCAL_MUTEX: i32 = -0x20;
const LOCAL_WINDOW: i32 = -0x24;
const LOCAL_FOUND_WINDOW: i32 = -0x4;
/// Passed by address to `Main_InitStartCell`, then its three words are passed by value to
/// `00877260`.
const LOCAL_START_POSITION: i32 = -0x1c8;
const LOCAL_START_CELL: i32 = -0x8;
const LOCAL_SYSTEM_INFO: i32 = -0x1bc;
const LOCAL_PROCESSOR_COUNT: i32 = -0x1a8;
const LOCAL_PATH_BUFFER: i32 = -0x130;
const LOCAL_LAUNCHER_PATH: i32 = -0x2e8;
const LOCAL_ARG_COUNT: i32 = -0x1d4;
const LOCAL_ARGS: i32 = -0x1d0;
const LOCAL_WINDOW_CLASS: i32 = -0x194;
const LOCAL_WINDOW_RECT: i32 = -0x1c;
const LOCAL_MESSAGE: i32 = -0x16c;
const LOCAL_SCOPE: i32 = -0x150;
const LOCAL_EXIT_DATA: i32 = -0x14c;
const LOCAL_ACTIVE_WINDOW: i32 = -0x134;
const LOCAL_RESIZE_RECT_ACTIVE: i32 = -0x300;
const LOCAL_RESIZE_RECT_IDLE: i32 = -0x314;

/// Argument of `GetSystemMetrics` for "is this a remote session".
const SM_REMOTESESSION: u32 = 0x1000;
const ERROR_ALREADY_EXISTS: u32 = 0xb7;
/// `GWL_STYLE`.
const GWL_STYLE: u32 = (-0x10i32) as u32;

/// The value of a setting object that holds a byte (through [`SETTING_BYTE_PTR`]).
fn setting_byte(e: &mut Engine, setting: u32) -> u32 {
    let address = e.call(SETTING_BYTE_PTR, &args![setting]).u32();
    e.mem.u8(address) as u32
}

/// Starts the log files (`c3bd30` with three settings, the error-handler address and `flag`,
/// then the names of the four text logs).
fn open_log_files(e: &mut Engine, flag: u32) {
    let first = setting_byte(e, 0x011d_ed74);
    let second = setting_byte(e, 0x011d_ee20);
    let third = setting_byte(e, 0x011d_ec88);
    e.call(
        0x00c3_bd30,
        &args![flag, third, second, first, 0x0086_a580u32],
    );
    e.call(0x00c3_bb90, &args![0x0108_2a38u32]);
    e.call(0x00c3_bbe0, &args![0x0108_2a28u32]);
    e.call(0x00c3_bc30, &args![0x0108_2a10u32]);
    e.call(0x00c3_bc80, &args![0x0108_2a00u32]);
}

/// When the processor-count setting (`011c3ea4`) is 1, makes it 2.
fn raise_single_processor_count(e: &mut Engine) {
    let count = e.call(SETTING_INT_PTR, &args![0x011c_3ea4u32]).u32();
    if e.mem.u32(count) == 1 {
        let count = e.call(SETTING_INT_PTR, &args![0x011c_3ea4u32]).u32();
        e.mem.set_u32(count, 2);
    }
}

/// Resizes the game window to the renderer's screen size: the client rectangle
/// (`0, 0, 004dc1f0(), 004dc200()`) grown by the window's style, then `SetWindowPos` with the
/// rectangle's left and bottom as the position and (right - left, top - bottom) as the size,
/// exactly as the game passes them. `rect` is a 20-byte local (the rectangle and the style).
fn resize_window_to_screen(e: &mut Engine, window: u32, rect: u32) {
    e.mem.set_u32(rect, 0);
    e.mem.set_u32(rect + 4, 0);
    let width = e.call(0x004d_c1f0, &args![]).u32();
    e.mem.set_u32(rect + 8, width);
    let height = e.call(0x004d_c200, &args![]).u32();
    e.mem.set_u32(rect + 12, height);
    e.mem.set_u32(rect + 16, 0);
    let style = e.call(API_GET_WINDOW_LONG, &args![window, GWL_STYLE]).u32();
    e.mem.set_u32(rect + 16, style);
    e.call(API_ADJUST_WINDOW_RECT_EX, &args![rect, style, 0u32, 0u32]);
    let left = e.mem.u32(rect);
    let top = e.mem.u32(rect + 4);
    let right = e.mem.u32(rect + 8);
    let bottom = e.mem.u32(rect + 12);
    e.call(
        API_SET_WINDOW_POS,
        &args![
            window,
            0u32,
            left,
            bottom,
            right.wrapping_sub(left),
            top.wrapping_sub(bottom),
            0u32
        ],
    );
}

/// Returns a rendered texture held in the object at `holder` (`00559450` reads it) to the
/// texture manager and clears the holder.
fn release_held_texture(e: &mut Engine, holder: u32) {
    if e.call(0x0055_9450, &args![holder]).u32() != 0 {
        let texture = e.call(0x0055_9450, &args![holder]).u32();
        let manager = e.call(0x004a_0ea0, &args![]).u32();
        // BSTextureManager::ReturnRenderedTexture (Xbox PDB)
        e.call(0x00b6_da10, &args![manager, texture]);
        e.call(0x0066_b0d0, &args![holder, 0u32]);
    }
}

/// Starts or restarts one of the profiling-style scope objects `main` keeps at
/// [`LOCAL_SCOPE`]: `00404f00(category, 1, source file, line)`.
fn scope_begin(e: &mut Engine, scope: u32, category: u32, line: u32) {
    e.call(
        0x0040_4f00,
        &args![scope, category, 1u32, 0x0108_29c4u32, line],
    );
}

// Translated from 0086a850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `main` (Xbox PDB): the game's `WinMain`. `instance` is the module handle; the other three
/// words (previous instance, command line, show command) are not read. Returns 0.
///
/// Sequence: single-instance mutex and remote-session checks; log files; processor count;
/// activates an already running window instead of starting; singletons and system
/// initialisation; `-nocloud` on the command line; the INI paths; optionally hands over to
/// `FalloutNVLauncher.exe`; window class and window; `Main`; the renderer, shader and scene
/// graph initialisation; the world, the player and the start cell; the message loop (active
/// frames call `Main`'s per-frame function, inactive ones sleep 50 ms and suspend audio); and
/// the shutdown of everything in reverse.
pub fn main(e: &mut Engine, instance: u32, _unused_1: u32, _unused_2: u32, _unused_3: u32) -> u32 {
    e.with_stack(MAIN_FRAME_SIZE, |e, frame| {
        let ebp = frame.addr() + MAIN_FRAME_SIZE;
        run_main(e, ebp, instance)
    })
}

/// `main`'s body on its frame, whose frame pointer is `ebp` (locals at negative offsets).
fn run_main(e: &mut Engine, ebp: u32, instance: u32) -> u32 {
    let at = |offset: i32| ebp.wrapping_add(offset as u32);

    let mutex = e
        .call(API_CREATE_MUTEX, &args![0u32, 0u32, 0x0108_2ab8u32])
        .u32();
    e.mem.set_u32(at(LOCAL_MUTEX), mutex);
    if e.call(API_GET_LAST_ERROR, &args![]).u32() == ERROR_ALREADY_EXISTS {
        e.call(
            API_MESSAGE_BOX,
            &args![0u32, 0x0108_2a7cu32, 0x0104_df38u32, 0u32],
        );
        return 0;
    }
    if e.call(API_GET_SYSTEM_METRICS, &args![SM_REMOTESESSION])
        .u32()
        != 0
    {
        e.call(
            API_MESSAGE_BOX,
            &args![0u32, 0x0108_2a48u32, 0x0104_df38u32, 0u32],
        );
        return 0;
    }

    open_log_files(e, 1);
    e.call(API_GET_SYSTEM_INFO, &args![at(LOCAL_SYSTEM_INFO)]);
    let processors = e.mem.u32(at(LOCAL_PROCESSOR_COUNT));
    let setting = e.call(SETTING_INT_PTR, &args![0x011c_3ea4u32]).u32();
    e.mem.set_u32(setting, processors);
    e.call(0x0087_6ad0, &args![at(LOCAL_PATH_BUFFER)]);
    raise_single_processor_count(e);

    let window_class_name = e.global::<u32>(0x011a_2fe8);
    let running = e
        .call(API_FIND_WINDOW, &args![window_class_name, 0u32])
        .u32();
    e.mem.set_u32(at(LOCAL_FOUND_WINDOW), running);
    if running != 0 {
        e.call(API_SET_FOREGROUND_WINDOW, &args![running]);
        return 0;
    }

    e.call(0x004b_4a30, &args![window_class_name]);
    // `PUSH x; CALL 00af2640; MOV ECX,EAX; CALL method(x)`: 00af2640 takes no argument.
    let system_utility = e.call(0x00af_2640, &args![]).u32();
    e.call(0x004f_b090, &args![system_utility, 0x0086_9f30u32]);
    let system_utility = e.call(0x00af_2640, &args![]).u32();
    let dialog_utility = e.call(0x0085_26c0, &args![system_utility]).u32();
    e.call(0x006e_cd40, &args![dialog_utility, 0x0086_a020u32]);
    e.call(0x00aa_20b0, &args![0u32]);
    e.call(0x00c5_d4e0, &args![]);
    e.call(0x004b_7920, &args![]);
    e.call(0x00a1_c6f0, &args![]);
    e.call(0x00bc_2940, &args![]);
    e.call(0x0045_b020, &args![]);
    e.call(0x0040_4f30, &args![0x3bu32]);
    fn_0086bde0(e, 0x0086_a050);
    fn_0086bcd0(e, 0x00b4_dfb0);
    fn_0086bcc0(e, 1);
    e.call(0x00c3_a6d0, &args![]);
    fn_0086bd70(e);

    let scope = at(LOCAL_SCOPE);
    e.call(
        0x0040_4eb0,
        &args![scope, 0x10u32, 1u32, 0x0108_29c4u32, 0x8e0u32],
    );
    e.call(0x0045_3720, &args![]);
    e.call(0x0040_4f70, &args![scope]);
    scope_begin(e, scope, 0xf, 0x8e6);
    e.call(0x00a6_1ad0, &args![]);
    e.call(0x0040_4f70, &args![scope]);
    e.call(0x006f_f610, &args![]);

    // The command line: `-nocloud` switches two settings off.
    e.mem.set_u32(at(LOCAL_ARG_COUNT), 0);
    let command_line = e.call(API_GET_COMMAND_LINE, &args![]).u32();
    let arguments = e
        .call(
            API_COMMAND_LINE_TO_ARGV,
            &args![command_line, at(LOCAL_ARG_COUNT)],
        )
        .u32();
    e.mem.set_u32(at(LOCAL_ARGS), arguments);
    if arguments != 0 {
        let count = e.mem.i32(at(LOCAL_ARG_COUNT));
        let mut index = 0i32;
        while index < count {
            let argument = e.mem.u32(arguments + 4 * index as u32);
            // A wide string starting with '-'.
            if argument != 0 && e.mem.u16(argument) == 0x2d {
                // `__wcsicmp(argument + 1 character, L"nocloud")`
                let matched = e
                    .call(0x00ec_c071, &args![argument + 2, 0x0108_29b4u32])
                    .u32();
                if matched == 0 {
                    // `PUSH 0; CALL 006ff580; MOV ECX,EAX; CALL 006ff880`
                    let object = e.call(0x006f_f580, &args![]).u32();
                    e.call(0x006f_f880, &args![object, 0u32]);
                }
            }
            index += 1;
        }
        e.call(API_LOCAL_FREE, &args![arguments]);
    }

    // The registry key and the INI file paths.
    let path = at(LOCAL_PATH_BUFFER);
    let key_owner = e.call(0x0087_7950, &args![]).u32();
    e.call(0x005e_0200, &args![key_owner, 0x0108_2990u32]);
    let object = e.call(0x0044_f560, &args![]).u32();
    e.call(0x005e_0200, &args![object, path]);
    let source = e.call(0x004d_c110, &args![]).u32();
    e.call(0x0040_6d30, &args![path, 0x104u32, source]);
    let ini_name = e.global::<u32>(0x011a_2ff0);
    e.call(0x0040_6d50, &args![path, 0x104u32, ini_name]);
    let object = e.call(0x004d_e490, &args![]).u32();
    e.call(0x005e_0200, &args![object, path]);
    let source = e.call(0x004d_c110, &args![]).u32();
    e.call(0x0040_6d30, &args![path, 0x104u32, source]);
    let prefs_name = e.global::<u32>(0x011a_2ff8);
    e.call(0x0040_6d50, &args![path, 0x104u32, prefs_name]);
    let object = e.call(0x0045_d180, &args![]).u32();
    e.call(0x005e_0200, &args![object, path]);
    let object = e.call(0x004d_e490, &args![]).u32();
    e.call(0x005e_0200, &args![object, path]);
    raise_single_processor_count(e);

    // Hand over to the launcher when the launcher is wanted and starts.
    if e.call(0x004d_a570, &args![]).bool() {
        e.call(0x004d_a4d0, &args![1u32]);
        let tasklet_manager = e.call(0x00b0_0a00, &args![]).u32();
        e.vcall(tasklet_manager, 8, &args![]);
        let launcher = at(LOCAL_LAUNCHER_PATH);
        e.call(0x0087_6a70, &args![launcher, 0x104u32]);
        e.call(0x0040_6d50, &args![launcher, 0x104u32, 0x0108_2978u32]);
        let started = e
            .call(
                API_SHELL_EXECUTE,
                &args![0u32, 0u32, launcher, 0u32, 0u32, 1u32],
            )
            .u32();
        if started != 0 {
            e.call(0x0040_4ee0, &args![scope]);
            return 0;
        }
    }

    let text = e.call(GET_POOLED_TEXT, &args![0x011d_ed2cu32]).u32();
    let other_text = e.call(GET_POOLED_TEXT, &args![0x011d_ee78u32]).u32();
    let system = e.call(0x00af_f100, &args![]).u32();
    e.call(0x00b0_12f0, &args![system, other_text, text]);
    let block = e.call(ALLOCATE_OBJECT, &args![0x10u32]).u32();
    if block != 0 {
        let setting = e.call(SETTING_INT_PTR, &args![0x011d_ee6cu32]).u32();
        let setting_value = e.mem.u32(setting);
        let pooled = e.call(GET_POOLED_TEXT, &args![0x011d_ece4u32]).u32();
        e.call(0x00af_dc60, &args![block, pooled, setting_value, 0u32]);
    }
    fn_0086bce0(e, 0x0105_bb14);
    open_log_files(e, 1);

    scope_begin(e, scope, 0x18, 0x992);
    e.call(ERROR_LOG, &args![]);
    let system = e.call(0x00af_f100, &args![]).u32();
    fn_0086bda0(e, Ptr::new(system));
    e.call(0x0040_4f70, &args![scope]);
    scope_begin(e, scope, 0x13, 0x99c);
    e.call(0x0087_6d20, &args![]);
    e.call(0x0040_4f70, &args![scope]);

    // The window class and the window.
    let class = at(LOCAL_WINDOW_CLASS);
    let icon = e.call(API_LOAD_ICON, &args![instance, 0x65u32]).u32();
    let background = e.call(API_GET_STOCK_OBJECT, &args![4u32]).u32();
    e.mem.set_u32(class, 3); // style
    e.mem.set_u32(class + 4, 0x0086_a0a0); // window procedure
    e.mem.set_u32(class + 8, 0); // extra class bytes
    e.mem.set_u32(class + 12, 0); // extra window bytes
    e.mem.set_u32(class + 16, instance);
    e.mem.set_u32(class + 20, icon);
    e.mem.set_u32(class + 24, 0); // cursor
    e.mem.set_u32(class + 28, background);
    e.mem.set_u32(class + 32, 0); // menu name
    e.mem.set_u32(class + 36, window_class_name);
    e.call(API_REGISTER_CLASS, &args![class]);
    let rect = at(LOCAL_WINDOW_RECT);
    e.mem.set_u32(rect, 0);
    e.mem.set_u32(rect + 4, 0);
    e.mem.set_u32(rect + 8, 0x140);
    e.mem.set_u32(rect + 12, 0xf0);
    e.call(API_ADJUST_WINDOW_RECT, &args![rect, 0x1000_0000u32, 0u32]);
    let width = e.mem.u32(rect + 8).wrapping_sub(e.mem.u32(rect));
    let height = e.mem.u32(rect + 12).wrapping_sub(e.mem.u32(rect + 4));
    let window = e
        .call(
            API_CREATE_WINDOW,
            &args![
                0u32,
                window_class_name,
                window_class_name,
                0x1000_0000u32,
                0u32,
                0u32,
                width,
                height,
                0u32,
                0u32,
                instance,
                0u32
            ],
        )
        .u32();
    e.mem.set_u32(at(LOCAL_WINDOW), window);

    // `Main` (0xA4 bytes).
    let block = e.call(ALLOCATE_OBJECT, &args![0xa4u32]).u32();
    let main_object = if block != 0 {
        e.call(0x0086_c160, &args![block, window, instance]).u32()
    } else {
        0
    };
    e.set_global(MAIN_OBJECT, main_object);
    scope_begin(e, scope, 0xd, 0x9f8);
    let block = e.call(ALLOCATE_OBJECT, &args![8u32]).u32();
    let second_object = if block != 0 {
        e.call(0x0070_05d0, &args![block]).u32()
    } else {
        0
    };
    e.set_global(0x011d_8804, second_object);
    e.call(0x0040_4f70, &args![scope]);

    e.call(ERROR_LOG, &args![0x0108_295cu32]);
    let main_object = e.global::<u32>(MAIN_OBJECT);
    e.call(0x0086_d500, &args![main_object]);
    e.set_global(0x011f_4509, 1u8);
    let renderer = e.call(GET_RENDERER, &args![]).u32();
    e.call(0x004a_0370, &args![renderer]);
    let renderer = e.call(GET_RENDERER, &args![]).u32();
    fn_0086bbd0(e, Ptr::new(renderer), 0x005d_4a40, 0);
    let renderer = e.call(GET_RENDERER, &args![]).u32();
    fn_0086bae0(e, Ptr::new(renderer), 0x0087_2570, 0);
    main_shader_init(e);
    let main_object = e.global::<u32>(MAIN_OBJECT);
    e.call(0x0086_d590, &args![main_object]);
    e.call(0x0070_2250, &args![1u32]);
    e.call(0x0065_19f0, &args![]);
    let main_object = e.global::<u32>(MAIN_OBJECT);
    e.call(0x0087_1c90, &args![main_object, 1u32]);
    e.call(0x006a_2920, &args![]);
    fn_0086c010(e);
    let renderer = e.call(GET_RENDERER, &args![]).u32();
    e.call(0x004a_03c0, &args![renderer]);
    e.call(0x0086_6ff0, &args![]);
    let pooled = e.call(GET_POOLED_TEXT, &args![0x011d_e72cu32]).u32();
    let movie_player = e.global::<u32>(0x0126_fac4);
    // MoviePlayer::StartSequence (Xbox PDB): text, 0, -1, 1, 1, 0.0f
    e.call(
        0x00ec_16d0,
        &args![movie_player, pooled, 0u32, u32::MAX, 1u32, 1u32, 0.0f32],
    );
    e.call(0x0040_4f70, &args![scope]);
    let object = e.call(0x0045_c670, &args![]).u32();
    let main_object = e.global::<u32>(MAIN_OBJECT);
    e.call(0x0086_cf20, &args![main_object, object]);
    let object = e.call(0x0045_c670, &args![]).u32();
    let animation_data = e.call(0x0066_29f0, &args![object]).u32();
    fn_0086be90(e, animation_data);
    let object = e.call(0x0045_c670, &args![]).u32();
    let animation_data = e.call(0x0066_29f0, &args![object]).u32();
    fn_0086be80(e, animation_data);
    let processors = e.call(SETTING_INT_PTR, &args![0x011c_3ea4u32]).u32();
    if e.mem.i32(processors) > 1 {
        let manager = e.call(0x0071_3d80, &args![]).u32();
        e.call(0x008c_7290, &args![manager]);
    }
    e.call(0x006d_0870, &args![]);
    let path_manager = e.call(0x0047_d0b0, &args![]).u32();
    e.call(0x006e_b8c0, &args![path_manager]);
    if setting_byte(e, 0x011d_73e4) != 0 {
        let obstacles = e.call(0x006c_0720, &args![]).u32();
        e.call(0x006c_07d0, &args![obstacles]);
    }
    e.call(0x004e_0540, &args![]);
    e.call(0x004d_eb30, &args![]);

    e.mem.set_u32(at(LOCAL_START_CELL), 0);
    let size_a: f32 = e.global(0x0101_8bfc);
    let size_b: f32 = e.global(0x0101_e704);
    e.call(
        0x0041_6870,
        &args![at(LOCAL_START_POSITION), size_a, size_a, size_b],
    );
    let data_handler = e.global::<u32>(0x011c_3f2c);
    let object = e.call(0x0046_0140, &args![data_handler]).u32();
    let world_space = e.call(0x0068_15c0, &args![object]).u32();
    let world_space = e.mem.u32(world_space);
    let tes = e.global::<u32>(0x011d_ea10);
    e.call(0x0045_8200, &args![tes, world_space]);
    let player = e.global::<u32>(0x011d_ea3c);
    e.call(0x0095_ee80, &args![player]);
    let object = e.global::<u32>(0x011d_e45c);
    e.call(0x008a_8150, &args![object, 1u32]);
    e.call(ERROR_LOG, &args![0x0108_293cu32]);
    e.call(0x0097_0d50, &args![0x011e_0e80u32]);
    let data_handler = e.global::<u32>(0x011c_3f2c);
    e.call(0x0046_14e0, &args![data_handler]);
    let object = e.global::<u32>(0x011d_df38);
    e.call(0x007d_6bd0, &args![object, 1u32]);
    e.call(ERROR_LOG, &args![0x0108_2924u32]);
    let start_cell = e.mem.u32(at(LOCAL_START_CELL));
    e.call(0x0087_6dd0, &args![start_cell, at(LOCAL_START_POSITION)]);
    e.call(ERROR_LOG, &args![0x0108_2910u32]);
    // The start position is passed by value: three words.
    let x = e.mem.u32(at(LOCAL_START_POSITION));
    let y = e.mem.u32(at(LOCAL_START_POSITION + 4));
    let z = e.mem.u32(at(LOCAL_START_POSITION + 8));
    e.call(0x0087_7260, &args![x, y, z]);
    e.call(0x0070_22c0, &args![]);
    let player = e.global::<u32>(0x011d_ea3c);
    e.call(0x0048_3710, &args![player]);
    let main_object = e.global::<u32>(MAIN_OBJECT);
    e.call(0x004f_15a0, &args![main_object, 1u32]);
    e.call(0x00af_4540, &args![]);
    let player = e.global::<u32>(0x011d_ea3c);
    if e.call(0x008d_6f30, &args![player]).u32() == 0 {
        let data_handler = e.global::<u32>(0x011c_3f2c);
        // TESDataHandler::SetMasterFileLargeBuffer (Xbox PDB)
        e.call(0x0046_50a0, &args![data_handler, u32::MAX]);
        e.call(ERROR_LOG, &args![]);
        e.call(0x0070_53f0, &args![]);
        // Controls::SetCurrentController (Xbox PDB)
        let controls = e.global::<u32>(0x0126_fac4);
        e.call(0x00ec_1800, &args![controls]);
        e.call(ERROR_LOG, &args![]);
    } else {
        let controls = e.global::<u32>(0x0126_fac4);
        e.call(0x00ec_1800, &args![controls]);
    }
    let io_manager = e.global::<u32>(0x0120_2d98);
    // IOManager::LoadQueuedPriority (Xbox PDB)
    e.call(0x0045_6520, &args![io_manager]);
    e.call(ERROR_LOG, &args![0x0108_28fcu32]);

    // The message loop.
    let message = at(LOCAL_MESSAGE);
    e.call(0x0040_3d30, &args![message, 0u32, 0x1cu32]);
    let main_object = e.global::<u32>(MAIN_OBJECT);
    let active_window = e.call(0x0044_ddc0, &args![main_object]).u32();
    e.mem.set_u32(at(LOCAL_ACTIVE_WINDOW), active_window);
    // "The game is in the background": audio and the loading thread are suspended.
    let mut background = false;
    loop {
        while e
            .call(API_PEEK_MESSAGE, &args![message, 0u32, 0u32, 0u32, 1u32])
            .u32()
            != 0
        {
            e.call(API_TRANSLATE_MESSAGE, &args![message]);
            e.call(API_DISPATCH_MESSAGE, &args![message]);
        }
        let foreground = e.call(API_GET_ACTIVE_WINDOW, &args![]).u32() == active_window
            || {
                let tes = e.global::<u32>(0x011d_ea10);
                e.call(0x0045_1530, &args![tes]).bool()
            }
            || e.call(0x0055_9450, &args![0x011d_eb30u32]).u32() != 0
            || setting_byte(e, 0x011d_eed8) != 0;
        if foreground {
            let renderer = e.call(GET_RENDERER, &args![]).u32();
            let device = e.call(0x004d_c020, &args![renderer]).u32();
            // `TestCooperativeLevel` (0086b3a6: only the device, as the COM `this`).
            let frame_result = e.vcall(device, 0xc, &args![]).i32();
            if frame_result < 0 {
                let renderer = e.call(GET_RENDERER, &args![]).u32();
                fn_0086ba10(e, Ptr::new(renderer));
                let renderer = e.call(GET_RENDERER, &args![]).u32();
                fn_0086ba90(e, Ptr::new(renderer));
                e.call(API_SLEEP, &args![0x32u32]);
            } else {
                let main_object = e.global::<u32>(MAIN_OBJECT);
                e.call(0x0086_e650, &args![main_object]);
            }
            if background {
                background = false;
                let audio = e.call(0x0045_3a70, &args![]).u32();
                e.call(0x00ad_8740, &args![audio]);
                e.call(0x0083_0660, &args![]);
                e.call(0x0083_2ad0, &args![0u32]);
                let audio = e.call(0x0045_3a70, &args![]).u32();
                e.call(0x00ad_7740, &args![audio, 1u32]);
                if e.call(GET_WINDOW_SETTING, &args![]).bool() {
                    resize_window_to_screen(e, window, at(LOCAL_RESIZE_RECT_ACTIVE));
                }
            }
            let main_object = e.global::<u32>(MAIN_OBJECT);
            if !background && e.mem.u8(main_object) != 0 && frame_result >= 0 {
                // LoadingMenu::ResumeBackgroundThread (Xbox PDB)
                e.call(0x0078_d020, &args![]);
                let main_object = e.global::<u32>(MAIN_OBJECT);
                e.mem.set_u8(main_object, 0);
            }
        } else {
            if !background {
                background = true;
                if fn_0086bdf0(e) == 0 {
                    let thread = e.call(API_GET_CURRENT_THREAD_ID, &args![]).u32();
                    let main_object = e.global::<u32>(MAIN_OBJECT);
                    let main_thread = e.call(GET_MAIN_THREAD_ID, &args![main_object]).u32();
                    if thread == main_thread {
                        // LoadingMenu::SuspendBackgroundThread (Xbox PDB)
                        e.call(0x0078_cfc0, &args![]);
                        let main_object = e.global::<u32>(MAIN_OBJECT);
                        e.mem.set_u8(main_object, 1);
                    }
                }
                let audio = e.call(0x0045_3a70, &args![]).u32();
                e.call(0x00ad_8700, &args![audio]);
                e.call(0x0083_0640, &args![]);
                e.call(0x0083_2ad0, &args![0u32]);
                let audio = e.call(0x0045_3a70, &args![]).u32();
                e.call(0x00ad_7740, &args![audio, 1u32]);
                let main_object = e.global::<u32>(MAIN_OBJECT);
                let object = e.call(0x004e_2190, &args![main_object]).u32();
                e.call(0x00b6_3620, &args![object]);
                let object = e.call(0x0049_fef0, &args![]).u32();
                e.call(0x004a_4340, &args![object]);
                let object = e.call(0x0070_5910, &args![]).u32();
                e.call(0x0080_26b0, &args![object]);
                if e.call(GET_WINDOW_SETTING, &args![]).bool() {
                    resize_window_to_screen(e, window, at(LOCAL_RESIZE_RECT_IDLE));
                }
            }
            e.call(API_SLEEP, &args![0x32u32]);
        }
        let main_object = e.global::<u32>(MAIN_OBJECT);
        if e.call(0x005d_c960, &args![main_object]).bool() {
            break;
        }
        let main_object = e.global::<u32>(MAIN_OBJECT);
        if e.call(0x0067_8ce0, &args![main_object]).bool() {
            let main_object = e.global::<u32>(MAIN_OBJECT);
            e.call(0x0087_74a0, &args![main_object, 1u32]);
        }
    }

    // Shutdown.
    let main_object = e.global::<u32>(MAIN_OBJECT);
    e.call(0x004f_15a0, &args![main_object, 0u32]);
    e.call(0x006f_f620, &args![]);
    e.call(0x0070_3610, &args![]);
    e.call(0x0070_6320, &args![0u32]);
    release_held_texture(e, 0x011d_ed3c);
    release_held_texture(e, 0x011d_ec64);
    release_held_texture(e, 0x011d_eb38);
    fn_0086bea0(e, 0);
    if e.call(0x0068_3a60, &args![]).u32() != 0 {
        e.call(0x00a8_19e0, &args![]);
    }
    let object = e.global::<u32>(0x011c_3b3c);
    e.call(0x0044_8420, &args![object]);
    let processors = e.call(SETTING_INT_PTR, &args![0x011c_3ea4u32]).u32();
    if e.mem.i32(processors) > 1 {
        let manager = e.call(0x0071_3d80, &args![]).u32();
        e.call(0x008c_73a0, &args![manager]);
    }
    e.call(0x0086_7090, &args![]);
    e.call(0x004d_c560, &args![]);
    e.call(0x004d_c590, &args![]);
    e.call(0x0070_2330, &args![]);
    let tes = e.global::<u32>(0x011d_ea10);
    if e.call(0x008d_8520, &args![tes]).u32() != 0 {
        let tes = e.global::<u32>(0x011d_ea10);
        let object = e.call(0x008d_8520, &args![tes]).u32();
        fn_0086be30(e, object);
    }
    e.call(0x004e_0800, &args![]);
    e.call(0x004d_ef20, &args![]);
    e.call(0x007f_fe00, &args![]);
    e.call(0x0083_2010, &args![]);
    e.call(0x0083_04a0, &args![]);
    e.call(0x0082_f9c0, &args![]);
    let audio = e.call(0x0045_3a70, &args![]).u32();
    e.vcall(audio, 8, &args![]);
    e.call(ERROR_LOG, &args![]);
    e.call(0x0049_ff80, &args![]);
    e.call(0x0062_2bc0, &args![]);
    e.call(0x0062_6f10, &args![]);
    if setting_byte(e, 0x011d_73e4) != 0 {
        let obstacles = e.call(0x006c_0720, &args![]).u32();
        e.call(0x006c_0900, &args![obstacles]);
    }
    let tes = e.global::<u32>(0x011d_ea10);
    if tes != 0 {
        fn_0086bee0(e, Ptr::new(tes), 1);
    }
    fn_0086beb0(e);
    let path_manager = e.call(0x0047_d0b0, &args![]).u32();
    e.call(0x006e_b980, &args![path_manager]);
    e.call(0x0065_1ab0, &args![]);
    e.call(0x0099_0ff0, &args![]);
    let tasklet_manager = e.call(0x00b0_0a00, &args![]).u32();
    e.vcall(tasklet_manager, 8, &args![]);
    e.call(0x0066_4940, &args![]);
    e.call(0x0052_6eb0, &args![]);
    // EffectSettingCollection::Reset (Xbox PDB)
    e.call(0x0040_8de0, &args![]);
    fn_0086bdd0(e);
    let second_object = e.global::<u32>(0x011d_8804);
    if second_object != 0 {
        fn_0086bf10(e, Ptr::new(second_object), 1);
    }
    let main_object = e.global::<u32>(MAIN_OBJECT);
    if main_object != 0 {
        fn_0086bf40(e, Ptr::new(main_object), 1);
    }
    e.set_global(MAIN_OBJECT, 0u32);
    let system = e.call(0x00af_f100, &args![]).u32();
    e.vcall(system, 8, &args![]);
    let object = e.call(0x004d_e490, &args![]).u32();
    let package = e.call(0x0071_7e50, &args![object]).u32();
    let object = e.call(0x004d_e490, &args![]).u32();
    e.call(0x005e_01b0, &args![object, package]);
    let renderer = e.call(GET_RENDERER, &args![]).u32();
    fn_0086bb20(e, Ptr::new(renderer), 0x0087_2570);
    let renderer = e.call(GET_RENDERER, &args![]).u32();
    fn_0086bc10(e, Ptr::new(renderer), 0x005d_4a40);
    e.call(0x006a_29a0, &args![]);
    e.call(0x00a6_2090, &args![]);
    e.call(0x00bc_2960, &args![]);
    // Renderer::Kill (Xbox PDB)
    e.call(0x004d_a4d0, &args![1u32]);
    fn_0086bd00(e);
    e.call(0x0045_b050, &args![]);
    e.call(0x00c5_d520, &args![]);
    e.call(0x004b_8cf0, &args![]);
    e.call(0x00a1_c790, &args![]);
    e.call(0x00aa_2170, &args![]);
    let exit_data = at(LOCAL_EXIT_DATA);
    e.call(0x0087_83c0, &args![exit_data, 0x0108_28ecu32]);
    let object = e.call(0x0040_1020, &args![]).u32();
    e.call(0x00aa_47c0, &args![object, exit_data, 0u32]);
    e.call(0x0087_84d0, &args![exit_data]);
    e.call(0x0040_4ee0, &args![scope]);
    0
}

// ---------------------------------------------------------------------------------------------
// The renderer's callback arrays and off-screen frame guards.
// ---------------------------------------------------------------------------------------------

// Translated from 0086ba10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts an off-screen frame on the renderer `this`: only when its frame state is 0 and its
/// virtual method at +0x17C accepts, the state becomes 2. Returns whether it did.
pub fn fn_0086ba10(e: &mut Engine, this: Ptr) -> bool {
    if !fn_0086ba60(e, this, 0x0108_2acc, 0) {
        return false;
    }
    if !e.vcall(this.addr(), 0x17c, &args![]).bool() {
        return false;
    }
    e.mem.set_u32(this.addr() + RENDERER_FRAME_STATE, 2);
    true
}

// Translated from 0086ba60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the renderer's frame state (`NiRenderer::m_eFrameState`, +0x200) is `expected`.
/// The first argument (the name of the caller's operation, for a message) is not used.
pub fn fn_0086ba60(e: &mut Engine, this: Ptr, _unused_1: u32, expected: u32) -> bool {
    e.mem.u32(this.addr() + RENDERER_FRAME_STATE) == expected
}

// Translated from 0086ba90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ends an off-screen frame on the renderer `this`: only when its frame state is 2 and its
/// virtual method at +0x180 accepts, the state becomes 0. Returns whether it did.
pub fn fn_0086ba90(e: &mut Engine, this: Ptr) -> bool {
    if !fn_0086ba60(e, this, 0x0108_2ae0, 2) {
        return false;
    }
    if !e.vcall(this.addr(), 0x180, &args![]).bool() {
        return false;
    }
    e.mem.set_u32(this.addr() + RENDERER_FRAME_STATE, 0);
    true
}

/// Adds `first` to the array at `this + key_array` and `second` at the same index of the array
/// at `this + data_array` (both methods take the address of the value); returns the index.
fn add_to_parallel_arrays(
    e: &mut Engine,
    this: Ptr,
    key_array: u32,
    data_array: u32,
    first: u32,
    second: u32,
) -> u32 {
    e.with_stack(8, |e, values| {
        e.mem.set_u32(values.addr(), first);
        e.mem.set_u32(values.addr() + 4, second);
        let index = e
            .call(ARRAY_ADD, &args![this.addr() + key_array, values])
            .u32();
        e.call(
            ARRAY_SET_AT_GROW,
            &args![this.addr() + data_array, index, values.addr() + 4],
        );
        index
    })
}

/// Index of the first element of the array at `this + key_array` equal to `key`, or
/// `u32::MAX`.
fn find_in_array(e: &mut Engine, this: Ptr, key_array: u32, key: u32) -> u32 {
    let array = this.addr() + key_array;
    let mut index = 0u32;
    while index < e.call(ARRAY_COUNT, &args![array]).u32() {
        let element = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
        if e.mem.u32(element) == key {
            return index;
        }
        index += 1;
    }
    u32::MAX
}

// Translated from 0086bae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers `first` in the renderer's first key array and `second` at the same index of the
/// data array after it (+0xAD4 / +0xAE4); returns the index.
pub fn fn_0086bae0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    add_to_parallel_arrays(e, this, KEY_ARRAY_A, DATA_ARRAY_A, first, second)
}

// Translated from 0086bb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `key` and its partner from the renderer's first pair of arrays (+0xAD4 / +0xAE4).
/// Returns whether it was found.
pub fn fn_0086bb20(e: &mut Engine, this: Ptr, key: u32) -> bool {
    let index = fn_0086bb70(e, this, key);
    if index == u32::MAX {
        return false;
    }
    e.call(ARRAY_REMOVE_AT, &args![this.addr() + KEY_ARRAY_A, index]);
    e.call(ARRAY_REMOVE_AT, &args![this.addr() + DATA_ARRAY_A, index]);
    true
}

// Translated from 0086bb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Index of `key` in the renderer's first key array (+0xAD4), or `0xFFFFFFFF`.
pub fn fn_0086bb70(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    find_in_array(e, this, KEY_ARRAY_A, key)
}

// Translated from 0086bbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers `first` in the renderer's second key array and `second` at the same index of the
/// data array after it (+0xAF4 / +0xB04); returns the index.
pub fn fn_0086bbd0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    add_to_parallel_arrays(e, this, KEY_ARRAY_B, DATA_ARRAY_B, first, second)
}

// Translated from 0086bc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `key` and its partner from the renderer's second pair of arrays (+0xAF4 / +0xB04).
/// Returns whether it was found.
pub fn fn_0086bc10(e: &mut Engine, this: Ptr, key: u32) -> bool {
    let index = fn_0086bc60(e, this, key);
    if index == u32::MAX {
        return false;
    }
    e.call(ARRAY_REMOVE_AT, &args![this.addr() + KEY_ARRAY_B, index]);
    e.call(ARRAY_REMOVE_AT, &args![this.addr() + DATA_ARRAY_B, index]);
    true
}

// Translated from 0086bc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Index of `key` in the renderer's second key array (+0xAF4), or `0xFFFFFFFF`.
pub fn fn_0086bc60(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    find_in_array(e, this, KEY_ARRAY_B, key)
}

// ---------------------------------------------------------------------------------------------
// Global setters and small wrappers.
// ---------------------------------------------------------------------------------------------

// Translated from 0086bcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte global at `011f4448`.
pub fn fn_0086bcc0(e: &mut Engine, value: u8) {
    e.set_global(0x011f_4448, value);
}

// Translated from 0086bcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the 32-bit global at `011f6e08`.
pub fn fn_0086bcd0(e: &mut Engine, value: u32) {
    e.set_global(0x011f_6e08, value);
}

/// The object at `011f81dc`, built in `main` by `00afdc60`.
const OBJECT_AT_011F81DC: u32 = 0x011f_81dc;

// Translated from 0086bce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00afdda0(argument)` on the object at `011f81dc`, when there is one.
pub fn fn_0086bce0(e: &mut Engine, argument: u32) {
    let object = e.global::<u32>(OBJECT_AT_011F81DC);
    if object != 0 {
        e.call(0x00af_dda0, &args![object, argument]);
    }
}

// Translated from 0086bd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys and frees the object at `011f81dc` when there is one (the global itself is left as
/// it was).
pub fn fn_0086bd00(e: &mut Engine) {
    let object = e.global::<u32>(OBJECT_AT_011F81DC);
    if object != 0 {
        fn_0086bd40(e, Ptr::new(object), 1);
    }
}

// Translated from 0086bd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of the object at `011f81dc`: runs `00afdcf0`, then frees the object when
/// bit 0 of `flags` is set. Returns `this`.
pub fn fn_0086bd40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    deleting_destructor(e, 0x00af_dcf0, this, flags)
}

// Translated from 0086bd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `00b013f0` into the global at `011f6224` (through [`fn_0086bd90`]).
pub fn fn_0086bd70(e: &mut Engine) {
    fn_0086bd90(e, 0x00b0_13f0);
}

// Translated from 0086bd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the 32-bit global at `011f6224`.
pub fn fn_0086bd90(e: &mut Engine, value: u32) {
    e.set_global(0x011f_6224, value);
}

// Translated from 0086bda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the state word at +0x5C from 1 or 2 to 3 (`main` calls it on the object from
/// `00aff100`).
pub fn fn_0086bda0(e: &mut Engine, this: Ptr) {
    let state = e.mem.u32(this.addr() + 0x5c);
    if state == 1 || state == 2 {
        e.mem.set_u32(this.addr() + 0x5c, 3);
    }
}

// Translated from 0086bdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00438af0` on the object at `011dd948`.
pub fn fn_0086bdd0(e: &mut Engine) {
    e.call(0x0043_8af0, &args![0x011d_d948u32]);
}

// Translated from 0086bde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the 32-bit global at `011f9134`.
pub fn fn_0086bde0(e: &mut Engine, value: u32) {
    e.set_global(0x011f_9134, value);
}

// Translated from 0086bdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// 1 when the object at `011da0c4` exists and its method `007cfd00` says yes, else 0.
pub fn fn_0086bdf0(e: &mut Engine) -> u8 {
    let object = e.global::<u32>(0x011d_a0c4);
    if object != 0 && e.call(0x007c_fd00, &args![object]).bool() {
        1
    } else {
        0
    }
}

/// The object at `011ccb78`.
const OBJECT_AT_011CCB78: u32 = 0x011c_cb78;

// Translated from 0086be30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the object at `011ccb78` through its first virtual method (the deleting destructor)
/// when there is one, and clears the global. The word in ECX (`main` passes the result of
/// `008d8520`) is not read.
pub fn fn_0086be30(e: &mut Engine, _unused_0: u32) {
    let object = e.global::<u32>(OBJECT_AT_011CCB78);
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
    e.set_global(OBJECT_AT_011CCB78, 0u32);
}

// Translated from 0086be80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the 32-bit global at `011d5c44`.
pub fn fn_0086be80(e: &mut Engine, value: u32) {
    e.set_global(0x011d_5c44, value);
}

// Translated from 0086be90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the 32-bit global at `011d59ec`.
pub fn fn_0086be90(e: &mut Engine, value: u32) {
    e.set_global(0x011d_59ec, value);
}

// Translated from 0086bea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte global at `011a31f4`.
pub fn fn_0086bea0(e: &mut Engine, value: u8) {
    e.set_global(0x011a_31f4, value);
}

// Translated from 0086beb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0066b0d0(0)` on the three objects at `011d86bc`, `011d86a8` and `011d8690`.
pub fn fn_0086beb0(e: &mut Engine) {
    for object in [0x011d_86bcu32, 0x011d_86a8, 0x011d_8690] {
        e.call(0x0066_b0d0, &args![object, 0u32]);
    }
}

/// Shape shared by the deleting destructors: runs the destructor body, frees the object when
/// bit 0 of `flags` is set and returns it.
fn deleting_destructor(e: &mut Engine, body: u32, this: Ptr, flags: u32) -> Ptr {
    e.call(body, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 0086bee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: body `00450770`, free when bit 0 of `flags` is set; returns `this`.
pub fn fn_0086bee0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    deleting_destructor(e, 0x0045_0770, this, flags)
}

// Translated from 0086bf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: body `00700650`, free when bit 0 of `flags` is set; returns `this`.
pub fn fn_0086bf10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    deleting_destructor(e, 0x0070_0650, this, flags)
}

// Translated from 0086bf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of `Main`: body `0086c880`, free when bit 0 of `flags` is set; returns
/// `this`.
pub fn fn_0086bf40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    deleting_destructor(e, 0x0086_c880, this, flags)
}

// Translated from 0086bf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main_ShaderInit` (Xbox PDB): logs, calls `005c5a60(0)`, installs [`shader_timer`] as the
/// timer callback, and copies four settings into the shader globals (bytes at `011f94a7` and
/// `011f9443`, floats at `011f9484` and `011f9480`).
pub fn main_shader_init(e: &mut Engine) {
    e.call(ERROR_LOG, &args![0x0108_2af4u32]);
    e.call(0x005c_5a60, &args![0u32]);
    fn_0086bff0(e, 0x0086_a480);
    let value = setting_byte(e, 0x011d_ed68) as u8;
    fn_0086c000(e, value);
    let value = setting_byte(e, 0x011d_eea8) as u8;
    e.set_global(0x011f_9443, value);
    let value = e.call(0x0045_0410, &args![0x011d_ec18u32]).f32();
    e.set_global(0x011f_9484, value);
    let value = e.call(0x0045_0410, &args![0x011d_eb70u32]).f32();
    e.set_global(0x011f_9480, value);
}

// Translated from 0086bff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the 32-bit global at `011f91f0` (the shader timer callback).
pub fn fn_0086bff0(e: &mut Engine, value: u32) {
    e.set_global(0x011f_91f0, value);
}

// Translated from 0086c000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte global at `011f94a7`.
pub fn fn_0086c000(e: &mut Engine, value: u8) {
    e.set_global(0x011f_94a7, value);
}

// Translated from 0086c010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads float settings through the `0086c0xx` / `0086c1xx` getters: when the byte at
/// `011f941e` is 0, `0086c0c0` and `0086c0d0` go to `0086c140` and `0086c150` and `0086c0e0`
/// and `0086c0f0` to the globals `011adbcc` / `011adbd0`; when it is set, `0086c120` and
/// `0086c130` go to the globals and `0086c100` and `0086c110` to `0086c140` and `0086c150`.
/// Then the object `004e3270` returns is asked (`004ebbc0`, index 0xC) for an entry, and the
/// first float of the object `00403e20` returns for `011cf8a4` is passed to `005b9f80` on that
/// entry (`004e3270` takes no argument; the two stack words belong to the next two calls).
pub fn fn_0086c010(e: &mut Engine) {
    if e.global::<u8>(0x011f_941e) != 0 {
        let value = e.call(0x0086_c120, &args![]).f32();
        e.set_global(0x011a_dbcc, value);
        let value = e.call(0x0086_c130, &args![]).f32();
        e.set_global(0x011a_dbd0, value);
        let value = e.call(0x0086_c100, &args![]).f32();
        e.call(0x0086_c140, &args![value]);
        let value = e.call(0x0086_c110, &args![]).f32();
        e.call(0x0086_c150, &args![value]);
    } else {
        let value = fn_0086c0c0(e);
        e.call(0x0086_c140, &args![value]);
        let value = fn_0086c0d0(e);
        e.call(0x0086_c150, &args![value]);
        let value = e.call(0x0086_c0e0, &args![]).f32();
        e.set_global(0x011a_dbcc, value);
        let value = e.call(0x0086_c0f0, &args![]).f32();
        e.set_global(0x011a_dbd0, value);
    }
    let float_address = e.call(0x0040_3e20, &args![0x011c_f8a4u32]).u32();
    let value = e.mem.f32(float_address);
    let object = e.call(0x004e_3270, &args![]).u32();
    let entry = e.call(0x004e_bbc0, &args![object, 0xcu32]).u32();
    e.call(0x005b_9f80, &args![entry, value]);
}

// Translated from 0086c0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the float setting at `011c72a0` (through `00450410`).
pub fn fn_0086c0c0(e: &mut Engine) -> f32 {
    e.call(0x0045_0410, &args![0x011c_72a0u32]).f32()
}

// Translated from 0086c0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the float setting at `011c774c` (through `00450410`).
pub fn fn_0086c0d0(e: &mut Engine) -> f32 {
    e.call(0x0045_0410, &args![0x011c_774cu32]).f32()
}

// ---------------------------------------------------------------------------------------------
// The `Main` object (`0086c160`), its task queues and the start-up of the game's world
// (`0086c0e0` - `0086e580`).
// ---------------------------------------------------------------------------------------------

/// Allocates a block for a scene-graph object (cdecl, size).
const ALLOCATE_SCENE_OBJECT: u32 = 0x00aa_13e0;
/// Reads the pointer held by the object it is called on (`this`, no argument).
const POINTER_GET: u32 = 0x0055_9450;
/// Assigns the pointer held by the object it is called on (`this`, the new pointer).
const POINTER_SET: u32 = 0x0066_b0d0;
/// A scope object (4 bytes, a local of the caller): `00404eb0` constructs it with a category,
/// a flag, the source file name and a line; `00404f70` ends it and `00404ee0` ends and
/// destroys it ([`scope_begin`] starts it again).
const SCOPE_CONSTRUCT: u32 = 0x0040_4eb0;
const SCOPE_END: u32 = 0x0040_4f70;
const SCOPE_DESTROY: u32 = 0x0040_4ee0;
/// `CreateSemaphoreA`, called by the address of its import slot.
const API_CREATE_SEMAPHORE: u32 = 0x00fd_f0b4;
/// Bits of `1.0f` and `0.0f`, as the game pushes them on the stack.
const FLOAT_ONE: u32 = 0x3f80_0000;
const FLOAT_ZERO: u32 = 0;

/// Global: the task object (0x1C04 bytes, built by `00a22660`) of [`fn_0086c790`].
const TASK_OBJECT: u32 = 0x011f_35cc;
/// Global: an object whose slot 0 (a deleting destructor) [`fn_0086cd80`] calls when its byte
/// at +4 is 0.
const SHUTDOWN_OBJECT: u32 = 0x0120_2d74;
/// Globals set by [`main_init_scene_graph`] and read by [`main_init_tes`].
const TES_OBJECT: u32 = 0x011d_ea10;
const LOD_ROOT_NODE: u32 = 0x011d_ea14;
const OBJECT_LOD_ROOT_NODE: u32 = 0x011d_ea18;
const WATER_LOD_NODE: u32 = 0x011d_ea1c;
const SKY_OBJECT: u32 = 0x011d_ea20;
const SCENE_FLAG_BYTE: u32 = 0x011d_ea28;
const PLAYER_OBJECT: u32 = 0x011d_ea3c;
/// Holders (objects that keep one pointer, read with [`POINTER_GET`]) of the scene graph.
const SCENE_GRAPH_HOLDER: u32 = 0x011d_eb7c;
const FOG_HOLDER: u32 = 0x011d_eb00;
const SKY_NODE_HOLDER: u32 = 0x011d_eb34;
const WEATHER_NODE_HOLDER: u32 = 0x011d_eda4;
const PARTICLE_SYSTEMS_HOLDER: u32 = 0x011d_ed58;
const SCREEN_ELEMENT_HOLDER: u32 = 0x011d_ec94;
const SCREEN_TEXTURE_HOLDER: u32 = 0x011d_ed3c;
/// A holder `0086c880` clears besides the scene-graph ones above.
const CLEARED_HOLDER: u32 = 0x011d_6e5c;
/// Tables of heap objects (pointers; zero when absent) with their element counts, destroyed
/// one by one by [`fn_0086c880`] through slot 0 of their vtables (the deleting destructor).
const OBJECT_TABLES: [(u32, u32); 9] = [
    (0x011d_5730, 4),
    (0x011d_52f0, 0xee),
    (0x011d_5240, 9),
    (0x011d_51b0, 8),
    (0x011d_51d0, 0x1c),
    (0x011d_5128, 0xe),
    (0x011d_5160, 0x14),
    (0x011d_5268, 0x22),
    (0x011d_56a8, 0x22),
];
/// The "processor count" setting (a setting object whose value [`SETTING_INT_PTR`] points at).
const PROCESSOR_COUNT_SETTING: u32 = 0x011c_3ea4;

/// Allocates `size` bytes with `allocator` (cdecl) and, when that worked, constructs the object
/// there with `construct(block)`, which returns the object. Returns 0 when the allocation
/// failed.
fn allocate_and_construct(
    e: &mut Engine,
    allocator: u32,
    size: u32,
    construct: impl FnOnce(&mut Engine, u32) -> u32,
) -> u32 {
    let block = e.call(allocator, &args![size]).u32();
    if block == 0 {
        0
    } else {
        construct(e, block)
    }
}

/// The pointer held by `holder`.
fn pointer_get(e: &mut Engine, holder: u32) -> u32 {
    e.call(POINTER_GET, &args![holder]).u32()
}

/// Makes `holder` hold `value`.
fn pointer_set(e: &mut Engine, holder: u32, value: u32) {
    e.call(POINTER_SET, &args![holder, value]);
}

/// Names `object` (`00a5b950`) with a temporary 4-byte name object built from the text at
/// `text` (`00438170`, destroyed again by `004381b0`). `object` is evaluated after the name is
/// built, as the game does.
fn name_object(e: &mut Engine, text: u32, object: impl FnOnce(&mut Engine) -> u32) {
    e.with_stack(4, |e, name| {
        let built = e.call(0x0043_8170, &args![name, text]).u32();
        let target = object(e);
        e.call(0x00a5_b950, &args![target, built]);
        e.call(0x0043_81b0, &args![name]);
    });
}

/// Logs a start-up message (`Error`, cdecl, one argument).
fn log_message(e: &mut Engine, text: u32) {
    e.call(ERROR_LOG, &args![text]);
}

// Translated from 0086c0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the float setting at `011c7314` (through `00450410`).
pub fn fn_0086c0e0(e: &mut Engine) -> f32 {
    e.call(0x0045_0410, &args![0x011c_7314u32]).f32()
}

// Translated from 0086c0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the float setting at `011c768c` (through `00450410`).
pub fn fn_0086c0f0(e: &mut Engine) -> f32 {
    e.call(0x0045_0410, &args![0x011c_768cu32]).f32()
}

// Translated from 0086c100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the float setting at `011c7758` (through `00450410`).
pub fn fn_0086c100(e: &mut Engine) -> f32 {
    e.call(0x0045_0410, &args![0x011c_7758u32]).f32()
}

// Translated from 0086c110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the float setting at `011c75bc` (through `00450410`).
pub fn fn_0086c110(e: &mut Engine) -> f32 {
    e.call(0x0045_0410, &args![0x011c_75bcu32]).f32()
}

// Translated from 0086c120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the float setting at `011c7458` (through `00450410`).
pub fn fn_0086c120(e: &mut Engine) -> f32 {
    e.call(0x0045_0410, &args![0x011c_7458u32]).f32()
}

// Translated from 0086c130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the float setting at `011c74a0` (through `00450410`).
pub fn fn_0086c130(e: &mut Engine) -> f32 {
    e.call(0x0045_0410, &args![0x011c_74a0u32]).f32()
}

// Translated from 0086c140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float in the global at `011ff8ac`.
pub fn fn_0086c140(e: &mut Engine, value: f32) {
    e.set_global(0x011f_f8ac, value);
}

// Translated from 0086c150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float in the global at `011ff8b0`.
pub fn fn_0086c150(e: &mut Engine, value: f32) {
    e.set_global(0x011f_f8b0, value);
}

// Translated from 0086c160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main::Main` (Xbox PDB): constructs the `Main` object (0xA4 bytes) at `this`; `window` and
/// `instance` are kept at +8 and +0xC. Two task-queue pairs are built first (a base object at
/// +0x18 / +0x50 and a semaphore-guarded queue at +0x28 / +0x60 over it, [`fn_0086c620`]),
/// then six pointer holders at +0x88 .. +0xA0 (`00633c90`) and seven flag bytes at +1 .. +7 are
/// cleared, as is the holder at `011dec64`. Five accumulator objects (0x280 bytes, `00b660d0`)
/// go into the holders at +0x88 (arguments 0x63, 1, 0x2f7), +0x8C and +0x90 (the same, each
/// then given two settings, `004a1040(0xC)` and `00936aa0(1)`), +0x94 (0x64, 1, 4;
/// `004a1040(0xD)`) and +0x98 (0x63, 1, 0x2f7); the byte at +0x9C is cleared and the holder at
/// +0xA0 gets an object built by `00a712f0` (0x114 bytes). With more than one processor the two
/// queues are started (`0087b8d0`, then [`bs_packed_task_queue_thread_begin_input`] on each).
/// Then the thread id (`0040fc90`), the window and the instance are stored (+0x10, +8, +0xC;
/// +0x14 is cleared), the thread is named (`00aa2740`, "Main"), the task object
/// ([`fn_0086c790`]), the object `007fdf30` returns (`00a22ef0`) and the audio set-up
/// ([`fn_0086ce40`]) follow, each inside a scope object. Returns `this`. (The SEH frame of the
/// original is not translated.)
pub fn main_main(e: &mut Engine, this: Ptr, window: u32, instance: u32) -> Ptr {
    let base = this.addr();
    e.call(0x00aa_53f0, &args![base + 0x18]);
    fn_0086c620(e, Ptr::new(base + 0x28), base + 0x18, 0x0087_b990);
    e.call(0x00aa_53f0, &args![base + 0x50]);
    fn_0086c620(e, Ptr::new(base + 0x60), base + 0x50, 0x0087_b990);
    for offset in [0x88u32, 0x8c, 0x90, 0x94, 0x98, 0xa0] {
        e.call(0x0063_3c90, &args![base + offset, 0u32]);
    }
    for offset in 1..=7u32 {
        e.mem.set_u8(base + offset, 0);
    }
    pointer_set(e, 0x011d_ec64, 0);

    let accumulator = |e: &mut Engine, kind: u32, third: u32| {
        allocate_and_construct(e, ALLOCATE_SCENE_OBJECT, 0x280, |e, block| {
            e.call(0x00b6_60d0, &args![block, kind, 1u32, third]).u32()
        })
    };
    let object = accumulator(e, 0x63, 0x2f7);
    pointer_set(e, base + 0x88, object);

    let object = accumulator(e, 0x63, 0x2f7);
    pointer_set(e, base + 0x8c, object);
    let held = pointer_get(e, base + 0x8c);
    e.call(0x004a_1040, &args![held, 0xcu32]);
    let held = pointer_get(e, base + 0x8c);
    e.call(0x0093_6aa0, &args![held, 1u32]);

    let object = accumulator(e, 0x63, 0x2f7);
    pointer_set(e, base + 0x90, object);
    let held = pointer_get(e, base + 0x90);
    e.call(0x004a_1040, &args![held, 0xcu32]);
    let held = pointer_get(e, base + 0x90);
    e.call(0x0093_6aa0, &args![held, 1u32]);

    let object = accumulator(e, 0x64, 4);
    pointer_set(e, base + 0x94, object);
    let held = pointer_get(e, base + 0x94);
    e.call(0x004a_1040, &args![held, 0xdu32]);

    let object = accumulator(e, 0x63, 0x2f7);
    pointer_set(e, base + 0x98, object);
    e.mem.set_u8(base + 0x9c, 0);

    let object = allocate_and_construct(e, ALLOCATE_SCENE_OBJECT, 0x114, |e, block| {
        e.call(0x00a7_12f0, &args![block]).u32()
    });
    pointer_set(e, base + 0xa0, object);

    let count = e
        .call(SETTING_INT_PTR, &args![PROCESSOR_COUNT_SETTING])
        .u32();
    if e.mem.i32(count) > 1 {
        e.call(0x0087_b8d0, &args![base + 0x28, base + 0x60]);
        bs_packed_task_queue_thread_begin_input(e, Ptr::new(base + 0x28));
        bs_packed_task_queue_thread_begin_input(e, Ptr::new(base + 0x60));
    }
    // The setting is looked up once more and the result is not used.
    e.call(SETTING_INT_PTR, &args![PROCESSOR_COUNT_SETTING]);
    let thread = e.call(0x0040_fc90, &args![]).u32();
    e.mem.set_u32(base + 0x10, thread);
    e.mem.set_u32(base + 8, window);
    e.mem.set_u32(base + 0xc, instance);
    e.mem.set_u32(base + 0x14, 0);
    e.call(0x00aa_2740, &args![thread, 0x0105_c2b0u32]);

    e.with_stack(4, |e, scope| {
        let scope = scope.addr();
        e.call(
            SCOPE_CONSTRUCT,
            &args![scope, 0xdu32, 1u32, 0x0108_29c4u32, 0xdc6u32],
        );
        fn_0086c790(e, instance);
        let object = e.call(0x007f_df30, &args![]).u32();
        e.call(0x00a2_2ef0, &args![object]);
        e.call(SCOPE_END, &args![scope]);
        scope_begin(e, scope, 0xb, 0xdd5);
        fn_0086ce40(e, this);
        e.call(SCOPE_END, &args![scope]);
        e.call(SCOPE_DESTROY, &args![scope]);
    });
    this
}

// Translated from 0086c600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPackedTaskQueue::ThreadBeginInput` (Xbox PDB): releases the semaphore wrapper at +0x14
/// (`00442550`).
pub fn bs_packed_task_queue_thread_begin_input(e: &mut Engine, this: Ptr) {
    e.call(0x0044_2550, &args![this.addr() + 0x14]);
}

// Translated from 0086c620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a semaphore-guarded task queue at `this`: the base part from `base`
/// (`00877a80`), the semaphore wrapper at +0x14 with 0 as initial count and 100 as maximum
/// ([`fn_0086c6a0`]), `value` at +0x20 and the byte at +0x24 cleared. Returns `this`. (The SEH
/// frame is not translated.)
pub fn fn_0086c620(e: &mut Engine, this: Ptr, base: u32, value: u32) -> Ptr {
    e.call(0x0087_7a80, &args![this, base]);
    fn_0086c6a0(e, Ptr::new(this.addr() + 0x14), 0, 100);
    e.mem.set_u32(this.addr() + 0x20, value);
    e.mem.set_u8(this.addr() + 0x24, 0);
    this
}

// Translated from 0086c6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a semaphore wrapper at `this`: the initial count at +0, the handle at +4 (from
/// `CreateSemaphoreA(NULL, initial, maximum, NULL)`) and the maximum count at +8. Returns
/// `this`.
pub fn fn_0086c6a0(e: &mut Engine, this: Ptr, initial: u32, maximum: u32) -> Ptr {
    e.mem.set_u32(this.addr(), initial);
    e.mem.set_u32(this.addr() + 8, maximum);
    let handle = e
        .call(API_CREATE_SEMAPHORE, &args![0u32, initial, maximum, 0u32])
        .u32();
    e.mem.set_u32(this.addr() + 4, handle);
    this
}

// Translated from 0086c6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys a task queue made by [`fn_0086c620`]: drains it ([`fn_0086c750`]), runs the
/// destructor of the semaphore wrapper at +0x14 (`0055a2d0`) and then the base destructor
/// (`00877ac0`). The SEH frame of the original is not translated.
pub fn fn_0086c6e0(e: &mut Engine, this: Ptr) {
    fn_0086c750(e, this);
    e.call(0x0055_a2d0, &args![this.addr() + 0x14]);
    e.call(0x0087_7ac0, &args![this]);
}

// Translated from 0086c750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls slot 0x10 of the queue's vtable with the address of a 32-byte local until it answers
/// false. The stack-cookie check is not translated.
pub fn fn_0086c750(e: &mut Engine, this: Ptr) {
    e.with_stack(32, |e, entry| {
        while e.vcall(this.addr(), 0x10, &args![entry]).bool() {}
    });
}

// Translated from 0086c790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the task object at `011f35cc`: the old one is destroyed ([`fn_0086c850`], flag
/// set) and a new 0x1C04-byte one built with `00a22660(argument)`; a failed allocation leaves
/// 0. (The SEH frame is not translated.)
pub fn fn_0086c790(e: &mut Engine, argument: u32) {
    let old = e.global::<u32>(TASK_OBJECT);
    if old != 0 {
        fn_0086c850(e, Ptr::new(old), 1);
    }
    let created = allocate_and_construct(e, ALLOCATE_OBJECT, 0x1c04, |e, block| {
        e.call(0x00a2_2660, &args![block, argument]).u32()
    });
    e.set_global(TASK_OBJECT, created);
}

// Translated from 0086c850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of the task object of [`fn_0086c790`]: body `00a22bf0`, free when bit 0
/// of `flags` is set; returns `this`.
pub fn fn_0086c850(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    deleting_destructor(e, 0x00a2_2bf0, this, flags)
}

// Translated from 0086c880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `Main` (its deleting destructor is [`fn_0086bf40`]): `00af5820`, then
/// `00a22e10` on the object `007fdf30` returns; the globals `011dea14`, `011dea18` and
/// `011dea1c` are cleared; the task object is destroyed ([`fn_0086cdf0`]); `00483710` acts
/// on the scene graph; eight holders are cleared; [`fn_0086cd80`] runs. Then every non-null
/// object of the nine tables [`OBJECT_TABLES`] is destroyed through slot 0 of its vtable with
/// flag 1. With more than one processor the two queues are waited on ([`fn_0086cdd0`] at +0x28
/// and +0x60) and `0087b950` runs. The holders at +0x88, +0x8C, +0x94, +0x98 and +0xA0 are
/// cleared and all six holders (+0xA0 down to +0x88) destroyed (`0045cec0`); finally the queue
/// at +0x60 ([`fn_0086c6e0`]), the base at +0x50 (`00aa5460`), the queue at +0x28 and the base
/// at +0x18.
pub fn fn_0086c880(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x00af_5820, &args![]);
    let object = e.call(0x007f_df30, &args![]).u32();
    e.call(0x00a2_2e10, &args![object]);
    e.set_global(LOD_ROOT_NODE, 0u32);
    e.set_global(OBJECT_LOD_ROOT_NODE, 0u32);
    e.set_global(WATER_LOD_NODE, 0u32);
    fn_0086cdf0(e);
    let held = pointer_get(e, SCENE_GRAPH_HOLDER);
    e.call(0x0048_3710, &args![held]);
    for holder in [
        SCENE_GRAPH_HOLDER,
        CLEARED_HOLDER,
        SKY_NODE_HOLDER,
        WEATHER_NODE_HOLDER,
        PARTICLE_SYSTEMS_HOLDER,
        FOG_HOLDER,
        SCREEN_TEXTURE_HOLDER,
        SCREEN_ELEMENT_HOLDER,
    ] {
        pointer_set(e, holder, 0);
    }
    fn_0086cd80(e);
    for (table, count) in OBJECT_TABLES {
        for index in 0..count {
            let object = e.mem.u32(table + index * 4);
            if object != 0 {
                e.vcall(object, 0, &args![1u32]);
            }
        }
    }
    let count = e
        .call(SETTING_INT_PTR, &args![PROCESSOR_COUNT_SETTING])
        .u32();
    if e.mem.i32(count) > 1 {
        fn_0086cdd0(e, Ptr::new(base + 0x28));
        fn_0086cdd0(e, Ptr::new(base + 0x60));
        e.call(0x0087_b950, &args![]);
    }
    for offset in [0x88u32, 0x8c, 0x94, 0x98, 0xa0] {
        pointer_set(e, base + offset, 0);
    }
    for offset in [0xa0u32, 0x98, 0x94, 0x90, 0x8c, 0x88] {
        e.call(0x0045_cec0, &args![base + offset]);
    }
    fn_0086c6e0(e, Ptr::new(base + 0x60));
    e.call(0x00aa_5460, &args![base + 0x50]);
    fn_0086c6e0(e, Ptr::new(base + 0x28));
    e.call(0x00aa_5460, &args![base + 0x18]);
}

// Translated from 0086cd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the object at `01202d74` exists and its byte at +4 is 0, calls its slot 0 with 1 (a
/// deleting destructor).
pub fn fn_0086cd80(e: &mut Engine) {
    let object = e.global::<u32>(SHUTDOWN_OBJECT);
    if object != 0 && e.mem.u8(object + 4) == 0 {
        let object = e.global::<u32>(SHUTDOWN_OBJECT);
        if object != 0 {
            e.vcall(object, 0, &args![1u32]);
        }
    }
}

// Translated from 0086cdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Waits on the semaphore wrapper at +0x14 of the queue (`004424e0`).
pub fn fn_0086cdd0(e: &mut Engine, this: Ptr) {
    e.call(0x0044_24e0, &args![this.addr() + 0x14]);
}

// Translated from 0086cdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the task object at `011f35cc` ([`fn_0086c850`] with flag 1) if there is one and
/// clears the global.
pub fn fn_0086cdf0(e: &mut Engine) {
    let object = e.global::<u32>(TASK_OBJECT);
    if object != 0 {
        fn_0086c850(e, Ptr::new(object), 1);
    }
    e.set_global(TASK_OBJECT, 0u32);
}

// Translated from 0086ce40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Audio set-up of `Main`: `00aea020`, then six callbacks are registered on the object
/// `00453a70` returns (`0050f9c0` with `0082d150`, `007037c0` with `0082d280`, `0061cc40` with
/// `005e3630`, [`fn_0086cf00`] with `0082d400`, `005f4bb0` with `00832c40`, `008d7dc0` with
/// `00832c80`), its slot 0xC is called with `0082d740`, its slot 4 with the word at +8 of the
/// `Main` object, and `0082f830` runs.
pub fn fn_0086ce40(e: &mut Engine, this: Ptr) {
    e.call(0x00ae_a020, &args![]);
    for (callee, callback) in [
        (0x0050_f9c0u32, 0x0082_d150u32),
        (0x0070_37c0, 0x0082_d280),
        (0x0061_cc40, 0x005e_3630),
    ] {
        let audio = e.call(0x0045_3a70, &args![]).u32();
        e.call(callee, &args![audio, callback]);
    }
    let audio = e.call(0x0045_3a70, &args![]).u32();
    fn_0086cf00(e, Ptr::new(audio), 0x0082_d400);
    for (callee, callback) in [(0x005f_4bb0u32, 0x0083_2c40u32), (0x008d_7dc0, 0x0083_2c80)] {
        let audio = e.call(0x0045_3a70, &args![]).u32();
        e.call(callee, &args![audio, callback]);
    }
    let audio = e.call(0x0045_3a70, &args![]).u32();
    e.vcall(audio, 0xc, &args![0x0082_d740u32]);
    let audio = e.call(0x0045_3a70, &args![]).u32();
    let word = e.mem.u32(this.addr() + 8);
    e.vcall(audio, 4, &args![word]);
    e.call(0x0082_f830, &args![]);
}

// Translated from 0086cf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at +0x2C of the object.
pub fn fn_0086cf00(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x2c, value);
}

// Translated from 0086cf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main::InitTES` (Xbox PDB): builds the `TES` object and loads the game's data, logging each
/// step. `node` is the object that gets `00a5a040` and a 12-byte value built by `0043d410(0.0,
/// 0, 0)` handed to `00a59c60` (the call order of the original).
///
/// In order: the sky object (`0046dd00`, kept at `011dea20`) and `00877730`; the helper object
/// `007a1480` returns (if any, it is given to the sky object, `004505a0` / `00a89af0`, and
/// replaced by the result of slot 0xC of the object `0045bc00(helper, 0)` returns, or 0); the
/// `TES` object (0xC4 bytes, `0044fb20` with `010724e8`, the value [`fn_0086d470`] returns, the
/// helper, the sky object and `011dea1c`) is kept at `011dea10`; tree manager, tasklet
/// threads; the ten file-name holders are looked up (`00403df0` text, `00464f30`, then
/// `00462f40` on `011c3f2c`) and each file found is activated (`00471d00(file, 1)`); the
/// plugins list is read (`00872430`) and when neither gave a file `Fallout.esm` is activated;
/// the player object (0xE50 bytes, `00938180`) is built into `011dea3c` and its slot 0x128 is
/// called with (0x14, 1); files, addon nodes and forms are loaded; the player is given its
/// object reference (`00575690` with the dynamic cast of `004839c0(7)` to the type at
/// `01183a1c`), its slot 0x2a8 gets a value built by `00416870`, `0086d490` and slot 0x88
/// run, `008c26e0(0)` is called, and when its object at +0xA4 answers 0 to slot 8 with 0x10 the
/// "Health value is 0" error is logged with the file name of the reference. Finally the scripts
/// are initialised (`00465040`) and, when the setting object at `011dee60` holds a value
/// between 1 and 199, [`fn_0086d4c0`] on the timer is given that setting's value. The whole
/// body is inside a scope object. The SEH frame is not translated.
pub fn main_init_tes(e: &mut Engine, this: Ptr, node: u32) {
    e.with_stack(4, |e, scope| {
        init_tes_body(e, this.addr(), node, scope.addr())
    });
}

/// The ten file-name holders [`main_init_tes`] walks.
const FILE_NAME_HOLDERS: [u32; 10] = [
    0x011d_ebf0,
    0x011d_eb88,
    0x011d_ecf8,
    0x011d_ee9c,
    0x011d_ec34,
    0x011d_ee0c,
    0x011d_ed14,
    0x011d_ee48,
    0x011d_ea70,
    0x011d_ed48,
];
/// The data handler (`011c3f2c`) whose method `00462f40(name)` finds a file.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The pointer to the plugins list file name used by [`main_init_tes`].
const PLUGINS_LIST_NAME: u32 = 0x011a_2ffc;

fn init_tes_body(e: &mut Engine, this: u32, node: u32, scope: u32) {
    e.call(
        SCOPE_CONSTRUCT,
        &args![scope, 0xau32, 1u32, 0x0108_29c4u32, 0xe4fu32],
    );
    scope_begin(e, scope, 0xa, 0xe51);
    let sky = e.call(0x0046_dd00, &args![]).u32();
    e.set_global(SKY_OBJECT, sky);
    e.call(0x0087_7730, &args![]);
    let tes_argument = fn_0086d470(e);
    let mut helper = e.call(0x007a_1480, &args![]).u32();
    if helper != 0 {
        let sky = e.global::<u32>(SKY_OBJECT);
        let result = e.call(0x0045_05a0, &args![sky]).u32();
        e.call(0x00a8_9af0, &args![result, helper]);
        let object = e.call(0x0045_bc00, &args![helper, 0u32]).u32();
        helper = if object != 0 {
            e.vcall(object, 0xc, &args![]).u32()
        } else {
            0
        };
    }
    log_message(e, 0x0108_2be8);
    let tes = allocate_and_construct(e, ALLOCATE_OBJECT, 0xc4, |e, block| {
        let sky = e.global::<u32>(SKY_OBJECT);
        let water_lod = e.global::<u32>(WATER_LOD_NODE);
        e.call(
            0x0044_fb20,
            &args![block, 0x0107_24e8u32, tes_argument, helper, sky, water_lod],
        )
        .u32()
    });
    e.set_global(TES_OBJECT, tes);
    e.call(SCOPE_END, &args![scope]);
    let tes = e.global::<u32>(TES_OBJECT);
    e.call(0x0045_0c50, &args![tes]);
    log_message(e, 0x0108_2bcc);
    e.call(0x0066_4870, &args![0u32]);
    log_message(e, 0x0108_2bac);
    let tasklets = e.call(0x00b0_0a00, &args![]).u32();
    e.call(0x00b0_0df0, &args![tasklets]);
    e.call(0x0052_6e10, &args![]);
    e.call(0x0099_0f20, &args![]);
    scope_begin(e, scope, 0x19, 0xe85);
    e.call(0x00a5_a040, &args![node]);
    e.with_stack(12, |e, value| {
        e.call(0x0043_d410, &args![value, FLOAT_ZERO, 0u32, 0u32]);
        e.call(0x00a5_9c60, &args![node, value]);
    });
    e.call(SCOPE_END, &args![scope]);

    let mut found_file = false;
    for holder in FILE_NAME_HOLDERS {
        if e.call(GET_POOLED_TEXT, &args![holder]).u32() != 0 {
            let text = e.call(GET_POOLED_TEXT, &args![holder]).u32();
            if e.call(0x00ec_6130, &args![text]).u32() != 0 {
                let text = e.call(GET_POOLED_TEXT, &args![holder]).u32();
                let name = e.call(0x0046_4f30, &args![text, 0u32]).u32();
                let handler = e.global::<u32>(DATA_HANDLER);
                let file = e.call(0x0046_2f40, &args![handler, name]).u32();
                if file != 0 {
                    e.call(0x0047_1d00, &args![file, 1u32]);
                    found_file = true;
                }
            }
        }
    }
    let list_name = e.global::<u32>(PLUGINS_LIST_NAME);
    let list_file = fn_0086d480(e);
    if e.call(0x0087_2430, &args![this, list_file, list_name])
        .bool()
    {
        found_file = true;
    }
    if !found_file {
        let handler = e.global::<u32>(DATA_HANDLER);
        let file = e.call(0x0046_2f40, &args![handler, 0x0108_2ba0u32]).u32();
        if file != 0 {
            e.call(0x0047_1d00, &args![file, 1u32]);
        }
    }
    scope_begin(e, scope, 0x34, 0xecb);
    let player = allocate_and_construct(e, ALLOCATE_OBJECT, 0xe50, |e, block| {
        e.call(0x0093_8180, &args![block]).u32()
    });
    e.set_global(PLAYER_OBJECT, player);
    let player = e.global::<u32>(PLAYER_OBJECT);
    e.vcall(player, 0x128, &args![0x14u32, 1u32]);
    e.call(SCOPE_END, &args![scope]);
    log_message(e, 0x0108_2b8c);
    let handler = e.global::<u32>(DATA_HANDLER);
    e.call(0x0046_3070, &args![handler, 0u32]);
    e.call(0x005f_5880, &args![]);
    let tes = e.global::<u32>(TES_OBJECT);
    e.call(0x0045_9f10, &args![tes]);
    let tes = e.global::<u32>(TES_OBJECT);
    e.call(0x0045_a370, &args![tes]);
    let setting = e.call(SETTING_BYTE_PTR, &args![0x011c_7adcu32]).u32();
    if e.mem.u8(setting) != 0 {
        let tes = e.global::<u32>(TES_OBJECT);
        e.call(0x0045_a600, &args![tes]);
    }
    let tes = e.global::<u32>(TES_OBJECT);
    e.call(0x0045_a1e0, &args![tes]);
    e.call(0x0068_b3e0, &args![]);
    log_message(e, 0x0108_2b74);
    scope_begin(e, scope, 0x34, 0xf00);
    let form = e.call(0x0048_39c0, &args![7u32]).u32();
    // `__RTDynamicCast(object, 0, source type, target type, 0)`.
    let reference = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, 0x0118_3028u32, 0x0118_3a1cu32, 0u32],
        )
        .u32();
    let player = e.global::<u32>(PLAYER_OBJECT);
    e.call(0x0057_5690, &args![player, reference]);
    let first_bits = e.global::<u32>(0x0101_8bfc);
    e.with_stack(12, |e, buffer| {
        let second_bits = e.global::<u32>(0x0101_8bfc);
        let position = e
            .call(
                0x0041_6870,
                &args![buffer, first_bits, second_bits, FLOAT_ZERO],
            )
            .u32();
        let player = e.global::<u32>(PLAYER_OBJECT);
        e.vcall(player, 0x2a8, &args![position]);
    });
    let player = e.global::<u32>(PLAYER_OBJECT);
    // TESObjectREFR::SetAngleOnReference (Xbox PDB)
    e.call(0x0086_d490, &args![player, 0x011f_426cu32]);
    let player = e.global::<u32>(PLAYER_OBJECT);
    e.vcall(player, 0x88, &args![]);
    let player = e.global::<u32>(PLAYER_OBJECT);
    e.call(0x008c_26e0, &args![player, 0u32]);
    let player = e.global::<u32>(PLAYER_OBJECT);
    let health = e.vcall(player + 0xa4, 8, &args![0x10u32]).u32();
    if health == 0 {
        let reference_file = e.call(0x0048_4e60, &args![reference, 0u32]).u32();
        let file_name = e.call(0x0089_1170, &args![reference_file]).u32();
        e.call(ERROR_LOG, &args![0x0108_2b30u32, file_name]);
    }
    log_message(e, 0x0108_2b14);
    let handler = e.global::<u32>(DATA_HANDLER);
    e.call(0x0046_5040, &args![handler]);
    let setting = 0x011d_ee60u32;
    if e.call(0x0045_03f0, &args![setting]).i32() > 0
        && e.call(0x0045_03f0, &args![setting]).i32() < 200
    {
        let value = e.call(SETTING_INT_PTR, &args![setting]).u32();
        let value = e.mem.u32(value);
        fn_0086d4c0(e, Ptr::new(TIMER), value);
    }
    e.call(SCOPE_DESTROY, &args![scope]);
}

// Translated from 0086d470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the global at `011dea18` (the object-LOD root node [`main_init_scene_graph`] makes).
pub fn fn_0086d470(e: &mut Engine) -> u32 {
    e.global::<u32>(OBJECT_LOD_ROOT_NODE)
}

// Translated from 0086d480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the address `01202e98`, a fixed object of the data section.
pub fn fn_0086d480(_e: &mut Engine) -> u32 {
    0x0120_2e98
}

// Translated from 0086d4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `1000.0 / count` (the `double` at `01017b70` divided by the unsigned count, rounded
/// to `float`) at +4 of the object, or 0.0 when `count` is 0.
pub fn fn_0086d4c0(e: &mut Engine, this: Ptr, count: u32) {
    let value: f64 = if count != 0 {
        let per_second: f64 = e.global(MILLISECONDS_PER_SECOND);
        per_second / count as f64
    } else {
        0.0
    };
    e.mem.set_f32(this.addr() + 4, value as f32);
}

// Translated from 0086d500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checks the renderer start-up: `004da670(this[8], this[0xC])`; when it answers 0 a Windows
/// message box titled "Fallout" says "Failed to initialize renderer." followed by the text
/// `0086d580` returns, and the process ends (`ExitProcess(0)`). The stack-cookie check is not
/// translated.
pub fn fn_0086d500(e: &mut Engine, this: Ptr) {
    let first = e.mem.u32(this.addr() + 8);
    let second = e.mem.u32(this.addr() + 0xc);
    if e.call(0x004d_a670, &args![first, second]).u32() == 0 {
        let detail = e.call(0x0086_d580, &args![]).u32();
        e.with_stack(0x204, |e, message| {
            // `sprintf(message, "Failed to initialize renderer.\n%s", detail)`
            e.call(0x00ec_623a, &args![message, 0x0108_2c04u32, detail]);
            e.call(API_MESSAGE_BOX, &args![0u32, message, 0x0108_2bfcu32, 0u32]);
        });
        e.call(API_EXIT_PROCESS, &args![0u32]);
    }
}

// Translated from 0086d590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main::InitSceneGraph` (Xbox PDB): builds the world's scene graph.
///
/// The `World` scene graph (0xC0 bytes, `00878610("World", 0, 0)`) goes into the holder at
/// `011deb7c`; a root node (0x200 bytes, `00b5e0f0`) is made; the shader path
/// `data\shaders\` and the node are handed on (`0086e520`, `0086e4e0`, `00b57420`,
/// `00712e60`, `0070b760`, `00b4f2f0`) and the node is stored in the holders at +0x88, +0x8C,
/// +0x90, +0x94 and +0x98 of `Main` (`004a1020`; the last one gets the result of `00450b80(1)`
/// instead); the byte settings are folded into the flag word given to `004bc3d0` (bit 1 for
/// `0086e540`, 2 for `0086e560`, 4 for `0086e5a0`, 8 for `0086e580`, always 0x10, 0x20 when
/// `004bc400` and `004dc060(0) >= 2`) and the one at `011ad82c` (8 for `0086e5c0`, 2 for
/// `0086e5e0`); `0086e500(0)`, `0086e510(0)` and `0086e4d0(00564360)` set globals. Then the
/// nodes are created, attached to the root with slot 0xDC / 0xF8 of its vtable and named: fog
/// (0x64 bytes, `00bb8180`, holder `011deb00`), the "Sky" node (`00c46970`, holder `011deb34`,
/// child 0), "Weather" (`00a5ecb0`, holder `011deda4`, child 1), "LODRoot" (`00bc2980`, global
/// `011dea14`, child 2), under it "LandLOD", "DistantRefLOD", "WaterLOD" (global `011dea1c`) and
/// "LOD Trees" (`0086e620` and `0086e630` get it), "ObjectLODRoot" (global `011dea18`, child 3)
/// and "Master Particle Systems" (`00c503d0`, holder `011ded58`, child 5). The root is placed
/// (`004bc1f0` with `0.0, 0.0` and the float at `0101e704`), updated and given the value
/// `0043d410(0.0, 0, 0)`; the renderer gets the accumulator `00b4f5c0` returns (`004dc540`);
/// the byte at `011dea28` is `0086e600()`. When the holder at `011dec94` is empty a screen
/// quad is built there: a screen-elements object (`00a881e0`, `00a87810`) with one element of
/// four vertices (`0086e3f0`), the rectangle (0, 0, 1, 1) (`0086e420`) and the texture
/// coordinates (0, 0, 1, 1) (`0086e460`), a material property (`00a75650`, `004bc450`) and a
/// texturing property (`00a6aa40`, the texture `004bc320(011ded3c, 0)` as base, clamp mode 0),
/// both attached (`00439410`), and the quad updated like the root. Finally `0057cf00` (grass
/// set-up) runs. Everything is inside a scope object; the SEH frame is not translated.
pub fn main_init_scene_graph(e: &mut Engine, this: Ptr) {
    e.with_stack(4, |e, scope| {
        init_scene_graph_body(e, this.addr(), scope.addr())
    });
}

/// A plain node of 0xAC bytes (`00a5ecb0(0)`), or 0.
fn new_plain_node(e: &mut Engine) -> u32 {
    allocate_and_construct(e, ALLOCATE_SCENE_OBJECT, 0xac, |e, block| {
        e.call(0x00a5_ecb0, &args![block, 0u32]).u32()
    })
}

/// Marks a node (`00546780(1)` and `005467c0(1)`).
fn mark_node(e: &mut Engine, node: u32) {
    e.call(0x0054_6780, &args![node, 1u32]);
    e.call(0x0054_67c0, &args![node, 1u32]);
}

/// Attaches `child` as child number `slot` of the node `parent` (slot 0xF8 of its vtable).
fn set_child(e: &mut Engine, parent: u32, slot: u32, child: u32) {
    e.vcall(parent, 0xf8, &args![slot, child]);
}

/// Attaches `child` to the node `parent` (slot 0xDC of its vtable, with 1).
fn attach_child(e: &mut Engine, parent: u32, child: u32) {
    e.vcall(parent, 0xdc, &args![child, 1u32]);
}

/// Updates the object held by `holder` (`00a5a040`) and hands it a 12-byte value built by
/// `0043d410(0.0, 0, 0)` (`00a59c60`).
fn update_held_object(e: &mut Engine, holder: u32) {
    let held = pointer_get(e, holder);
    e.call(0x00a5_a040, &args![held]);
    e.with_stack(12, |e, value| {
        e.call(0x0043_d410, &args![value, FLOAT_ZERO, 0u32, 0u32]);
        let held = pointer_get(e, holder);
        e.call(0x00a5_9c60, &args![held, value]);
    });
}

fn init_scene_graph_body(e: &mut Engine, this: u32, scope: u32) {
    e.call(
        SCOPE_CONSTRUCT,
        &args![scope, 0x19u32, 1u32, 0x0108_29c4u32, 0xf4cu32],
    );
    let graph = allocate_and_construct(e, ALLOCATE_SCENE_OBJECT, 0xc0, |e, block| {
        e.call(0x0087_8610, &args![block, 0x0108_2c9cu32, 0u32, 0u32])
            .u32()
    });
    pointer_set(e, SCENE_GRAPH_HOLDER, graph);
    let root = allocate_and_construct(e, ALLOCATE_SCENE_OBJECT, 0x200, |e, block| {
        e.call(0x00b5_e0f0, &args![block]).u32()
    });
    let shader_path = e.call(0x004b_c3f0, &args![]).u32();
    fn_0086e520(e, shader_path);
    fn_0086e4e0(e, 0x0108_2c8c);
    e.call(0x00b5_7420, &args![]);
    let held = pointer_get(e, SCENE_GRAPH_HOLDER);
    let held = e.call(0x0066_29f0, &args![held]).u32();
    e.call(0x0071_2e60, &args![held, 0x011a_d840u32]);
    e.call(0x0070_b760, &args![root, 0u32]);
    e.call(0x00b4_f2f0, &args![0u32, root]);
    for offset in [0x88u32, 0x8c, 0x90, 0x94] {
        let held = pointer_get(e, this + offset);
        e.call(0x004a_1020, &args![held, root]);
    }
    let value = e.call(0x0045_0b80, &args![1u32]).u32();
    let held = pointer_get(e, this + 0x98);
    e.call(0x004a_1020, &args![held, value]);
    fn_0086e500(e, 0);
    fn_0086e510(e, 0);

    // The flag word handed to 004bc3d0: one bit per byte setting that equals 1.
    let mut flags = (fn_0086e540(e) == 1) as u32;
    flags |= if fn_0086e560(e) == 1 { 2 } else { 0 };
    flags |= if e.call(0x0086_e5a0, &args![]).u8() == 1 {
        4
    } else {
        0
    };
    flags |= if fn_0086e580(e) == 1 { 8 } else { 0 };
    flags |= 0x10;
    let extra =
        if e.call(0x004b_c400, &args![]).bool() && e.call(0x004d_c060, &args![0u32]).i32() >= 2 {
            0x20
        } else {
            0
        };
    flags |= extra;
    e.call(0x004b_c3d0, &args![flags]);
    let mut state = if e.call(0x0086_e5c0, &args![]).u8() != 0 {
        8
    } else {
        0
    };
    state |= if e.call(0x0086_e5e0, &args![]).u8() != 0 {
        2
    } else {
        0
    };
    e.set_global(0x011a_d82c, state);
    fn_0086e4d0(e, 0x0056_4360);
    e.set_global(0x011f_d870, FLOAT_ZERO);

    let held = pointer_get(e, SCENE_GRAPH_HOLDER);
    let held = e.call(0x0066_29f0, &args![held]).u32();
    pointer_set(e, 0x011f_95d8, held);
    let graph = pointer_get(e, SCENE_GRAPH_HOLDER);
    attach_child(e, graph, root);

    // Fog.
    let fog = allocate_and_construct(e, ALLOCATE_SCENE_OBJECT, 0x64, |e, block| {
        e.call(0x00bb_8180, &args![block]).u32()
    });
    pointer_set(e, FOG_HOLDER, fog);
    let held = pointer_get(e, FOG_HOLDER);
    e.call(0x0049_ed90, &args![held, 1u32]);
    let held = pointer_get(e, FOG_HOLDER);
    e.call(0x00bb_8140, &args![held, FLOAT_ONE]);
    let held = pointer_get(e, FOG_HOLDER);
    e.call(0x00b5_5540, &args![0u32, held]);

    // The "Sky" node.
    let node = allocate_and_construct(e, ALLOCATE_SCENE_OBJECT, 0xb4, |e, block| {
        e.call(0x00c4_6970, &args![block]).u32()
    });
    pointer_set(e, SKY_NODE_HOLDER, node);
    let held = pointer_get(e, SKY_NODE_HOLDER);
    e.call(0x0054_68d0, &args![held, 1u32]);
    let held = pointer_get(e, SKY_NODE_HOLDER);
    e.call(0x0054_67c0, &args![held, 1u32]);
    name_object(e, 0x0108_2c88, |e| pointer_get(e, SKY_NODE_HOLDER));
    let held = pointer_get(e, SKY_NODE_HOLDER);
    set_child(e, root, 0, held);

    // The "Weather" node.
    let node = new_plain_node(e);
    pointer_set(e, WEATHER_NODE_HOLDER, node);
    let held = pointer_get(e, WEATHER_NODE_HOLDER);
    e.call(0x0054_6780, &args![held, 1u32]);
    let held = pointer_get(e, WEATHER_NODE_HOLDER);
    e.call(0x0054_67c0, &args![held, 1u32]);
    name_object(e, 0x0101_fe78, |e| pointer_get(e, WEATHER_NODE_HOLDER));
    let held = pointer_get(e, WEATHER_NODE_HOLDER);
    set_child(e, root, 1, held);
    e.call(0x004d_c5c0, &args![]);
    e.call(0x004d_c650, &args![]);

    // The LOD nodes.
    let lod_root = allocate_and_construct(e, ALLOCATE_SCENE_OBJECT, 0xb8, |e, block| {
        e.call(0x00bc_2980, &args![block]).u32()
    });
    e.set_global(LOD_ROOT_NODE, lod_root);
    name_object(e, 0x0108_2c80, |e| e.global::<u32>(LOD_ROOT_NODE));
    let lod_root = e.global::<u32>(LOD_ROOT_NODE);
    set_child(e, root, 2, lod_root);

    let land_lod = new_plain_node(e);
    mark_node(e, land_lod);
    name_object(e, 0x0108_2c78, |_| land_lod);
    let lod_root = e.global::<u32>(LOD_ROOT_NODE);
    attach_child(e, lod_root, land_lod);
    // The float the object `00403e20(011dedc8)` points at.
    let setting = e.call(0x0040_3e20, &args![0x011d_edc8u32]).u32();
    let value = e.mem.u32(setting);
    e.set_global(0x011a_d808, value);

    let distant_lod = new_plain_node(e);
    mark_node(e, distant_lod);
    name_object(e, 0x0108_2c68, |_| distant_lod);
    let lod_root = e.global::<u32>(LOD_ROOT_NODE);
    attach_child(e, lod_root, distant_lod);

    let water_lod = new_plain_node(e);
    e.set_global(WATER_LOD_NODE, water_lod);
    let water_lod = e.global::<u32>(WATER_LOD_NODE);
    mark_node(e, water_lod);
    name_object(e, 0x0108_2c5c, |e| e.global::<u32>(WATER_LOD_NODE));
    let water_lod = e.global::<u32>(WATER_LOD_NODE);
    let lod_root = e.global::<u32>(LOD_ROOT_NODE);
    attach_child(e, lod_root, water_lod);

    let trees = new_plain_node(e);
    mark_node(e, trees);
    name_object(e, 0x0108_2c50, |_| trees);
    // The "LOD Trees" node is attached to the "DistantRefLOD" node, the last one held in the
    // local the game reuses for the LOD children.
    attach_child(e, distant_lod, trees);
    e.call(0x0086_e620, &args![trees]);
    e.call(0x0086_e630, &args![trees]);

    let object_lod_root = new_plain_node(e);
    e.set_global(OBJECT_LOD_ROOT_NODE, object_lod_root);
    mark_node(e, object_lod_root);
    name_object(e, 0x0108_2c40, |e| e.global::<u32>(OBJECT_LOD_ROOT_NODE));
    let object_lod_root = e.global::<u32>(OBJECT_LOD_ROOT_NODE);
    set_child(e, root, 3, object_lod_root);

    // Master particle systems.
    let particles = allocate_and_construct(e, ALLOCATE_SCENE_OBJECT, 0xb8, |e, block| {
        e.call(0x00c5_03d0, &args![block]).u32()
    });
    pointer_set(e, PARTICLE_SYSTEMS_HOLDER, particles);
    name_object(e, 0x0108_2c28, |e| pointer_get(e, PARTICLE_SYSTEMS_HOLDER));
    let held = pointer_get(e, PARTICLE_SYSTEMS_HOLDER);
    set_child(e, root, 5, held);

    // Place and update the root.
    let limit_bits = e.global::<u32>(0x0101_e704);
    let held = pointer_get(e, SCENE_GRAPH_HOLDER);
    let held = e.call(0x0055_8310, &args![held]).u32();
    e.call(
        0x004b_c1f0,
        &args![held, FLOAT_ZERO, FLOAT_ZERO, limit_bits],
    );
    update_held_object(e, SCENE_GRAPH_HOLDER);
    let accumulator = e.call(0x00b4_f5c0, &args![]).u32();
    let renderer = e.call(GET_RENDERER, &args![]).u32();
    e.call(0x004d_c540, &args![renderer, accumulator]);
    let flag = e.call(0x0086_e600, &args![]).u8();
    e.set_global(SCENE_FLAG_BYTE, flag);

    // The screen quad.
    if pointer_get(e, SCREEN_ELEMENT_HOLDER) == 0 {
        let outer = allocate_and_construct(e, ALLOCATE_SCENE_OBJECT, 0xc4, |e, block| {
            let data = allocate_and_construct(e, ALLOCATE_SCENE_OBJECT, 0x70, |e, data_block| {
                e.call(
                    0x00a8_81e0,
                    &args![data_block, 0u32, 0u32, 1u32, 1u32, 1u32, 4u32, 1u32, 2u32, 1u32],
                )
                .u32()
            });
            e.call(0x00a8_7810, &args![block, data]).u32()
        });
        pointer_set(e, SCREEN_ELEMENT_HOLDER, outer);
        let held = pointer_get(e, SCREEN_ELEMENT_HOLDER);
        fn_0086e3f0(e, Ptr::new(held), 4, 0, 0);
        let held = pointer_get(e, SCREEN_ELEMENT_HOLDER);
        fn_0086e420(
            e,
            Ptr::new(held),
            0,
            f32::from_bits(FLOAT_ZERO),
            f32::from_bits(FLOAT_ZERO),
            f32::from_bits(FLOAT_ONE),
            f32::from_bits(FLOAT_ONE),
        );
        let held = pointer_get(e, SCREEN_ELEMENT_HOLDER);
        fn_0086e4b0(e, Ptr::new(held));
        let held = pointer_get(e, SCREEN_ELEMENT_HOLDER);
        fn_0086e460(
            e,
            Ptr::new(held),
            0,
            0,
            f32::from_bits(FLOAT_ZERO),
            f32::from_bits(FLOAT_ZERO),
            f32::from_bits(FLOAT_ONE),
            f32::from_bits(FLOAT_ONE),
        );
        let material = allocate_and_construct(e, ALLOCATE_SCENE_OBJECT, 0x4c, |e, block| {
            e.call(0x00a7_5650, &args![block]).u32()
        });
        e.call(0x004b_c450, &args![material, 0x011a_9b7cu32]);
        let held = pointer_get(e, SCREEN_ELEMENT_HOLDER);
        e.call(0x0043_9410, &args![held, material]);
        let texturing = allocate_and_construct(e, ALLOCATE_SCENE_OBJECT, 0x30, |e, block| {
            e.call(0x00a6_aa40, &args![block]).u32()
        });
        let held = pointer_get(e, SCREEN_TEXTURE_HOLDER);
        let texture = e.call(0x004b_c320, &args![held, 0u32]).u32();
        e.call(0x005b_8fc0, &args![texturing, texture]);
        e.call(0x0060_aeb0, &args![texturing, 0u32]);
        e.call(0x004f_3200, &args![texturing, 0u32]);
        let held = pointer_get(e, SCREEN_ELEMENT_HOLDER);
        e.call(0x0043_9410, &args![held, texturing]);
        update_held_object(e, SCREEN_ELEMENT_HOLDER);
    }
    e.call(0x0057_cf00, &args![]);
    e.call(SCOPE_DESTROY, &args![scope]);
}

// Translated from 0086e3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `00a87ae0` on the object `005495f0` (the pointer held at +0xB8) returns, with
/// the same three arguments (`NiScreenElementsData::Insert` in the decompiler's naming). Returns
/// what it returns.
pub fn fn_0086e3f0(e: &mut Engine, this: Ptr, first: u16, second: u16, third: u32) -> u32 {
    let data = e.call(0x0054_95f0, &args![this]).u32();
    e.call(0x00a8_7ae0, &args![data, first, second, third])
        .u32()
}

// Translated from 0086e420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `00a88020` on the object `005495f0` returns, with the same five arguments (an
/// index and four floats; `NiScreenElementsData::SetRectangle` in the decompiler's naming).
/// Returns what it returns.
pub fn fn_0086e420(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    first: f32,
    second: f32,
    third: f32,
    fourth: f32,
) -> u32 {
    let data = e.call(0x0054_95f0, &args![this]).u32();
    e.call(
        0x00a8_8020,
        &args![data, index, first, second, third, fourth],
    )
    .u32()
}

// Translated from 0086e460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `00a88130` on the object `005495f0` returns, with the same six arguments (an
/// index, a 16-bit value and four floats; `NiScreenElementsData::SetTextures` in the
/// decompiler's naming). Returns what it returns.
#[allow(clippy::too_many_arguments)]
pub fn fn_0086e460(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    set: u16,
    first: f32,
    second: f32,
    third: f32,
    fourth: f32,
) -> u32 {
    let data = e.call(0x0054_95f0, &args![this]).u32();
    e.call(
        0x00a8_8130,
        &args![data, index, set, first, second, third, fourth],
    )
    .u32()
}

// Translated from 0086e4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `00a87900` on the object `005495f0` returns (`NiScreenElementsData::UpdateBound`
/// in the decompiler's naming).
pub fn fn_0086e4b0(e: &mut Engine, this: Ptr) {
    let data = e.call(0x0054_95f0, &args![this]).u32();
    e.call(0x00a8_7900, &args![data]);
}

// Translated from 0086e4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a word in the global at `011f91ec`.
pub fn fn_0086e4d0(e: &mut Engine, value: u32) {
    e.set_global(0x011f_91ec, value);
}

// Translated from 0086e4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the string at `text` into the 0x104-byte buffer at `011f9308` (`00406d30`).
pub fn fn_0086e4e0(e: &mut Engine, text: u32) {
    e.call(0x0040_6d30, &args![0x011f_9308u32, 0x104u32, text]);
}

// Translated from 0086e500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte in the global at `011f91de`.
pub fn fn_0086e500(e: &mut Engine, value: u8) {
    e.set_global(0x011f_91de, value);
}

// Translated from 0086e510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte in the global at `011f91dd`.
pub fn fn_0086e510(e: &mut Engine, value: u8) {
    e.set_global(0x011f_91dd, value);
}

// Translated from 0086e520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the holder at `011f9508` hold `value` (`0066b0d0`).
pub fn fn_0086e520(e: &mut Engine, value: u32) {
    pointer_set(e, 0x011f_9508, value);
}

// Translated from 0086e540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte setting at `011c7154`.
pub fn fn_0086e540(e: &mut Engine) -> u8 {
    setting_byte(e, 0x011c_7154) as u8
}

// Translated from 0086e560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte setting at `011c7544`.
pub fn fn_0086e560(e: &mut Engine) -> u8 {
    setting_byte(e, 0x011c_7544) as u8
}

// Translated from 0086e580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte setting at `011c712c`.
pub fn fn_0086e580(e: &mut Engine) -> u8 {
    setting_byte(e, 0x011c_712c) as u8
}

// ---------------------------------------------------------------------------------------------
// The per-frame idle update (`0086e650`) and the small functions around it.
// ---------------------------------------------------------------------------------------------

/// `Main::bFlyCamera` and `Main::bFreezeTime` (Xbox PDB): byte flags at +6 and +7 of the `Main`
/// object.
const MAIN_FLY_CAMERA: u32 = 6;
const MAIN_FREEZE_TIME: u32 = 7;
/// The four byte flags the idle update refreshes from the interface: a menu is up
/// (`Interface::IsInMenuMode` or `Interface::IsPipboyOpening`), the dialog test (`007050d0`),
/// the fader test (`00701450` with 1) and the console test (`Interface::IsConsoleVisible`).
const IN_MENU_FLAG: u32 = 0x011d_ea2b;
const IN_DIALOG_FLAG: u32 = 0x011d_ea2c;
const FADER_ONE_FLAG: u32 = 0x011d_ea2d;
const CONSOLE_VISIBLE_FLAG: u32 = 0x011d_ea2e;
/// A byte flag several steps of the idle update read (the byte `00525420` returns is the one
/// after it, `011dea2a`).
const SCENE_FLAG_29: u32 = 0x011d_ea29;
/// The fader manager object (`00701450`, `007010e0`, ... are called on it).
const FADER_MANAGER: u32 = 0x011d_8804;
/// The `ProcessLists` instance (the methods at `0096....` are called on it).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The data handler object the cell lookups (`00461bc0`) are called on.
const DATA_HANDLER_OBJECT: u32 = 0x011c_3f2c;
/// The idle update's frame counter.
const FRAME_COUNTER: u32 = 0x011a_2fe0;
/// `0044ddc0` is called on this object: it returns the word at +8.
const MODE_OBJECT: u32 = 0x011f_2250;
/// Reads the holder at [`SCENE_GRAPH_HOLDER`] and passes its value to `006629f0` (cdecl, no
/// argument).
const SCENE_DATA_GETTER: u32 = 0x0052_4c90;
/// The actor value the idle update asks the player for (virtual method +8 of the object at
/// +0xA4) before turning it into an alignment.
const KARMA_ACTOR_VALUE: u32 = 0x17;
/// `GetAsyncKeyState` (import slot).
pub(crate) const API_GET_ASYNC_KEY_STATE: u32 = 0x00fd_f308;
/// `CreateDirectoryA` (import slot).
pub(crate) const API_CREATE_DIRECTORY: u32 = 0x00fd_f0b8;

/// Reads a byte flag of the `Main` object.
fn main_flag(e: &Engine, this: Ptr, offset: u32) -> bool {
    e.mem.u8(this.addr() + offset) != 0
}

/// Reads a byte global as a flag.
fn byte_flag(e: &Engine, address: u32) -> bool {
    e.global::<u8>(address) != 0
}

/// The processor-count setting as the signed number the code compares.
fn processor_count(e: &mut Engine) -> i32 {
    let address = e
        .call(SETTING_INT_PTR, &args![PROCESSOR_COUNT_SETTING])
        .u32();
    e.mem.u32(address) as i32
}

/// The seconds of the timer at [`TIMER`] (`0084d030`, a `float` in `ST0`).
fn timer_seconds(e: &mut Engine) -> f32 {
    e.call(TIMER_GET_SECONDS, &args![TIMER]).f32()
}

/// The frame seconds of the timer at [`TIMER`] (`007013e0`, a `float` in `ST0`).
fn timer_frame_seconds(e: &mut Engine) -> f32 {
    e.call(0x0070_13e0, &args![TIMER]).f32()
}

/// Reads the `float` a setting object (`00403e20`) points at.
fn float_setting(e: &mut Engine, setting: u32) -> f32 {
    let address = e.call(0x0040_3e20, &args![setting]).u32();
    e.mem.f32(address)
}

/// `GetAsyncKeyState(virtual_key)`: whether the key is down (the high bit of the result).
fn key_is_down(e: &mut Engine, virtual_key: u32) -> bool {
    e.call(API_GET_ASYNC_KEY_STATE, &args![virtual_key]).u16() & 0x8000 != 0
}

/// Calls `Interface::IsTopMenuID` (`00702450`, cdecl).
fn is_top_menu(e: &mut Engine, menu_id: u32) -> bool {
    e.call(0x0070_2450, &args![menu_id]).u8() != 0
}

/// Calls `Interface::IsMenuIDVisible` (`00702680`, cdecl: menu id, then a second word).
fn is_menu_visible(e: &mut Engine, menu_id: u32, second: u32) -> bool {
    e.call(0x0070_2680, &args![menu_id, second]).u8() != 0
}

/// Refreshes the four interface flags: the menu flag is 1 when `Interface::IsInMenuMode` or
/// `Interface::IsPipboyOpening` says so (the second is only asked when the first says no).
fn refresh_interface_flags(e: &mut Engine) {
    let in_menu =
        e.call(IS_IN_MENU_MODE, &args![]).u8() != 0 || e.call(0x0070_9bc0, &args![]).u8() != 0;
    e.set_global(IN_MENU_FLAG, in_menu as u8);
    let in_dialog = e.call(0x0070_50d0, &args![]).u8();
    e.set_global(IN_DIALOG_FLAG, in_dialog);
    let fader = e.global::<u32>(FADER_MANAGER);
    let fader_one = e.call(0x0070_1450, &args![fader, 1u32]).u8();
    e.set_global(FADER_ONE_FLAG, fader_one);
    let console = e.call(0x0070_3d50, &args![]).u8();
    e.set_global(CONSOLE_VISIBLE_FLAG, console);
}

/// The test the idle update repeats before the world-facing steps: no menu is up (or the fader
/// flag is set), the console is not visible and the time is not frozen.
fn world_steps_allowed(e: &Engine, this: Ptr) -> bool {
    (!byte_flag(e, IN_MENU_FLAG) || byte_flag(e, FADER_ONE_FLAG))
        && !byte_flag(e, CONSOLE_VISIBLE_FLAG)
        && !main_flag(e, this, MAIN_FREEZE_TIME)
}

// Translated from 0086e5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte setting at `011c7180`.
pub fn fn_0086e5a0(e: &mut Engine) -> u8 {
    setting_byte(e, 0x011c_7180) as u8
}

// Translated from 0086e5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte setting at `011c7368`.
pub fn fn_0086e5c0(e: &mut Engine) -> u8 {
    setting_byte(e, 0x011c_7368) as u8
}

// Translated from 0086e5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte setting at `011c7380`.
pub fn fn_0086e5e0(e: &mut Engine) -> u8 {
    setting_byte(e, 0x011c_7380) as u8
}

// Translated from 0086e600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte setting at `011c7550`.
pub fn fn_0086e600(e: &mut Engine) -> u8 {
    setting_byte(e, 0x011c_7550) as u8
}

// Translated from 0086e620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a word in the global at `011f9644`.
pub fn fn_0086e620(e: &mut Engine, value: u32) {
    e.set_global(0x011f_9644, value);
}

// Translated from 0086e630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the holder at `011d86bc` hold `value` (`0066b0d0`).
pub fn fn_0086e630(e: &mut Engine, value: u32) {
    pointer_set(e, 0x011d_86bc, value);
}

// Translated from 0086e650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The per-frame idle update of the `Main` object: refreshes the interface flags, updates the
/// player, the controls, the process lists, the cells, the menus, the renderer and the audio, in
/// the order the game does, and counts the frames. `this` is the `Main` object.
pub fn fn_0086e650(e: &mut Engine, this: Ptr) {
    let frame_budget = e.global::<u32>(0x011a_3010);
    fn_0086a830(e, this.addr(), frame_budget);
    // Alt+Tab: skip the whole update.
    if key_is_down(e, 9) && key_is_down(e, 0x12) {
        return;
    }
    let system = e.call(0x00af_2640, &args![]).u32();
    e.vcall(system, 0x0c, &args![]);

    refresh_interface_flags(e);
    // A rendered menu other than the Pipboy is up.
    let other_menu = if e.call(0x0070_7ad0, &args![]).u32() != 0 {
        let menu = e.call(0x0070_7ad0, &args![]).u32();
        let pipboy = e.call(0x0070_5990, &args![]).u32();
        menu != pipboy
    } else {
        false
    };
    let object = e.call(0x004b_7210, &args![]).u32();
    let kind = e.call(0x0042_4940, &args![object]).u8() as i8;
    if kind == 3 && !other_menu {
        e.call(0x0087_82b0, &args![]);
    }
    main_on_idle_update_player(e, this);
    let helper = e.call(0x006f_f580, &args![]).u32();
    e.call(0x006f_f860, &args![helper]);
    let running = !byte_flag(e, IN_MENU_FLAG) && !main_flag(e, this, MAIN_FREEZE_TIME);
    main_on_idle_update_image_space(e, this, running as u8);

    refresh_interface_flags(e);
    if byte_flag(e, 0x011d_eefc) {
        // `Pathing::ProfilePathing(process, value of virtual +0x1f4, 2, 100)` (cdecl).
        let player = e.global::<u32>(PLAYER_OBJECT);
        let value = e.vcall(player, 0x1f4, &args![]).u32();
        let process = e.call(0x008d_6f30, &args![player]).u32();
        e.call(0x006d_a7c0, &args![process, value, 2u32, 100u32]);
    }
    let saves = e.global::<u32>(0x011d_e134);
    e.call(0x0085_1d90, &args![saves]);
    fn_0086ef30(e);
    fn_0086ef90(e);
    fn_0086f190(e, this);
    e.call(0x0048_3710, &args![this]);
    e.call(0x004e_1610, &args![]);
    let frames = e.global::<u32>(FRAME_COUNTER);
    e.set_global(FRAME_COUNTER, frames.wrapping_add(1));
    e.set_global(0x011d_f674, 0u32);
    if !byte_flag(e, IN_MENU_FLAG) {
        fn_0086ef40(e);
    }
    fn_0086f260(e, this);
    main_on_idle_poll_controls(e, this);
    let input = e.global::<u32>(0x0120_2d98);
    e.call(0x00c3_dbf0, &args![input]);
    if fn_0086efa0(e) == 0 {
        if e.call(0x0070_edf0, &args![]).u8() != 0 {
            e.call(0x0078_cfc0, &args![]);
        } else if e.call(0x0070_5ea0, &args![]).u8() != 0 {
            let tes = e.global::<u32>(TES_OBJECT);
            e.call(0x0045_7d70, &args![tes, 0u32, 0u32, 0u32]);
        }
    }
    fn_0086efe0(e, this);
    let menu = byte_flag(e, IN_MENU_FLAG) as u32;
    let scene_data = e.call(SCENE_DATA_GETTER, &args![]).u32();
    let manager = e.call(0x004a_0ea0, &args![]).u32();
    e.call(0x00b6_dd00, &args![manager, scene_data, menu]);
    main_on_idle_handle_menu_background(e, this);
    let fader = e.global::<u32>(FADER_MANAGER);
    e.call(0x0070_11d0, &args![fader, 1u32]);
    if !byte_flag(e, IN_MENU_FLAG) && !main_flag(e, this, MAIN_FREEZE_TIME) {
        e.call(0x004e_0110, &args![]);
        e.call(0x004d_e600, &args![]);
    }
    if e.call(0x0068_3a60, &args![]).u32() != 0 {
        e.call(0x00a8_1a20, &args![]);
    }
    if !byte_flag(e, IN_MENU_FLAG) && !main_flag(e, this, MAIN_FREEZE_TIME) {
        let tes = e.global::<u32>(TES_OBJECT);
        if e.call(0x0045_1530, &args![tes]).u8() != 0 && fn_0086ef70(e, Ptr::new(tes)) == 0 {
            e.call(0x0045_56d0, &args![tes, 0u32]);
        }
        let scene = e.call(POINTER_GET, &args![SCENE_GRAPH_HOLDER]).u32();
        let seconds = timer_seconds(e);
        e.vcall(scene, 0x104, &args![seconds]);
        let seconds = timer_seconds(e);
        e.call(0x0086_7a40, &args![CALENDAR, seconds]);
        if processor_count(e) == 1 {
            let tes = e.global::<u32>(TES_OBJECT);
            e.call(0x0045_5640, &args![tes]);
        }
    }
    e.call(0x0040_fbf0, &args![0x011f_11a0u32, 0u32]);
    e.call(0x0097_8550, &args![PROCESS_LISTS]);
    if processor_count(e) == 1 {
        if (!byte_flag(e, IN_MENU_FLAG) || byte_flag(e, FADER_ONE_FLAG))
            && !byte_flag(e, CONSOLE_VISIBLE_FLAG)
            && !main_flag(e, this, MAIN_FREEZE_TIME)
        {
            e.call(0x0097_77a0, &args![PROCESS_LISTS]);
            e.call(0x008d_0600, &args![PROCESS_LISTS, 0.0f32, 0u32]);
            e.call(0x0096_eb40, &args![PROCESS_LISTS]);
            e.call(0x0096_e9b0, &args![PROCESS_LISTS]);
        }
    } else if world_steps_allowed(e, this) {
        e.call(0x0097_77a0, &args![PROCESS_LISTS]);
        e.call(0x0096_eb40, &args![PROCESS_LISTS]);
        e.call(0x0096_e9b0, &args![PROCESS_LISTS]);
    }
    e.call(0x0040_fba0, &args![0x011f_11a0u32]);
    let object = e.call(0x0044_6ef0, &args![]).u32();
    e.call(0x0087_8080, &args![object, 1u32]);
    if fn_0086ef60(e) != 0 {
        e.call(0x00a6_1cd0, &args![]);
    }
    e.call(0x0086_8850, &args![]);
    e.call(0x0086_8d10, &args![]);
    let paused = byte_flag(e, IN_MENU_FLAG) || main_flag(e, this, MAIN_FREEZE_TIME);
    let scene_data = e.call(SCENE_DATA_GETTER, &args![]).u32();
    e.call(0x0066_52e0, &args![scene_data, paused as u32]);
    fn_0086fbe0(e, this);
    if processor_count(e) == 1 {
        fn_0086fd70(e, this);
    }
    e.call(0x0048_3710, &args![this]);
    let object = e.call(0x0049_fef0, &args![]).u32();
    e.call(0x0049_fff0, &args![object]);
    let scene_data = e.call(SCENE_DATA_GETTER, &args![]).u32();
    e.call(0x0071_2e60, &args![scene_data, 0x011a_d840u32]);
    if e.call(0x0044_ddc0, &args![MODE_OBJECT]).u32() != 4 {
        let player = e.global::<u32>(PLAYER_OBJECT);
        let field_of_view = e.call(0x0071_0ab0, &args![player]).f32();
        let scene = e.call(0x0045_c670, &args![]).u32();
        e.call(0x00c5_2020, &args![scene, field_of_view, 0u32, 0u32, 1u32]);
    }
    let player = e.global::<u32>(PLAYER_OBJECT);
    let field_of_view = e.call(0x0071_0ab0, &args![player]).f32();
    e.call(0x00b5_4000, &args![field_of_view]);
    let scene = e.call(POINTER_GET, &args![SCENE_GRAPH_HOLDER]).u32();
    // The game also keeps a local that is always 0 here and would be preferred when set.
    let animation_data = e.call(0x0066_29f0, &args![scene]).u32();
    let tes = e.global::<u32>(TES_OBJECT);
    e.call(0x0045_bc80, &args![tes, 1u32, 1u32]);
    let object = e.call(0x0045_0b80, &args![0u32]).u32();
    e.call(0x00b5_ac90, &args![object, animation_data]);
    let tes = e.global::<u32>(TES_OBJECT);
    e.call(0x0045_b070, &args![tes, animation_data]);
    let render_world = (!byte_flag(e, IN_MENU_FLAG) || byte_flag(e, FADER_ONE_FLAG))
        && !byte_flag(e, CONSOLE_VISIBLE_FLAG)
        && !main_flag(e, this, MAIN_FREEZE_TIME);
    if render_world {
        if processor_count(e) > 1 {
            e.call(0x008c_80e0, &args![1u32]);
            let object = e.call(0x0071_3d80, &args![]).u32();
            e.call(0x008c_78c0, &args![object]);
        } else {
            e.call(0x008c_a070, &args![0x011e_0fe0u32]);
        }
    }
    fn_0086fc60(e, this);
    if processor_count(e) > 1 && !render_world {
        fn_0086fd70(e, this);
        if e.call(IS_IN_MENU_MODE, &args![]).u8() != 0 {
            e.call(0x0070_58e0, &args![]);
        }
    }
    if !main_flag(e, this, MAIN_FREEZE_TIME) {
        let object = e.call(0x0047_d0b0, &args![]).u32();
        e.call(0x006e_bc50, &args![object]);
    }
    let object = e.call(0x0055_2ba0, &args![]).u32();
    e.call(0x006a_61b0, &args![object]);
    if processor_count(e) == 1 {
        let address = e.call(SETTING_BYTE_PTR, &args![0x011d_73e4u32]).u32();
        if e.mem.u8(address) != 0 {
            let object = e.call(0x006c_0720, &args![]).u32();
            e.call(0x006c_3640, &args![object]);
        }
        let hold = byte_flag(e, IN_MENU_FLAG)
            || byte_flag(e, FADER_ONE_FLAG)
            || main_flag(e, this, MAIN_FREEZE_TIME);
        let object = e.global::<u32>(0x011f_1958);
        e.call(0x0099_1500, &args![object, hold as u32]);
    }
    if e.call(0x0071_4a00, &args![]).u8() != 0 {
        e.call(0x00a8_1a80, &args![]);
    }
    if e.call(0x0070_23c0, &args![]).u32() == 0x3f4 && e.call(0x0070_56f0, &args![]).u8() != 0 {
        e.call(0x0087_1dc0, &args![this]);
    }
    e.call(0x0057_ab70, &args![]);
    e.call(0x00b6_0040, &args![]);
    // The karma of the player turned into an alignment (cdecl, one `float`); the result is
    // stored in a local the function never reads.
    let player = e.global::<u32>(PLAYER_OBJECT);
    let karma = e.vcall(player + 0xa4, 8, &args![KARMA_ACTOR_VALUE]).i32();
    e.call(0x0047_e040, &args![karma as f32]);
    fn_0086ff70(e, this);
    e.call(0x0087_05d0, &args![this]);
    if byte_flag(e, 0x011c_6fbb) {
        e.call(0x004d_c360, &args![]);
        e.set_global(0x011c_6fbb, 0u8);
    }
    if world_steps_allowed(e, this) {
        if processor_count(e) > 1 {
            let object = e.call(0x0071_3d80, &args![]).u32();
            e.call(0x008c_7990, &args![object]);
        } else {
            e.call(0x008c_a300, &args![0x011e_0fe0u32]);
        }
    }
    main_update_non_render_safe_ai_tasks(e, this);
    e.call(0x0087_0610, &args![this]);
    if e.call(0x0070_ed10, &args![]).u8() != 0 {
        e.call(0x0070_ed20, &args![0u32]);
        e.call(0x0070_3e10, &args![]);
    }
    e.call(0x00a2_9680, &args![]);
    e.call(0x005a_e270, &args![]);
    e.call(0x005a_9d60, &args![]);
    // Seconds gathered since the 256-step counter last ticked.
    let seconds = timer_seconds(e) as f64;
    let accumulated = (seconds + e.global::<f32>(0x011d_eef8) as f64) as f32;
    e.set_global(0x011d_eef8, accumulated);
    let player = e.global::<u32>(PLAYER_OBJECT);
    let tick = e.call(IS_IN_MENU_MODE, &args![]).u8() != 0
        || (player != 0 && e.call(0x0095_0090, &args![player]).u8() != 0)
        || (accumulated as f64) >= e.global::<f64>(0x0103_57e8);
    if tick {
        e.set_global(0x011d_eef8, 0.0f32);
        let step = e.global::<u8>(0x011d_eef5);
        e.set_global(0x011d_eef5, step.wrapping_add(1));
        e.call(0x00aa_7290, &args![step as u32]);
        // The game also resets the counter when it reaches 0x100, which a byte never does.
    }
}

// Translated from 0086ef30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the byte at `012677a2`.
pub fn fn_0086ef30(e: &mut Engine) {
    e.set_global(0x0126_77a2, 0u8);
}

// Translated from 0086ef40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds one to the word at `011f9138`.
pub fn fn_0086ef40(e: &mut Engine) {
    let value = e.global::<u32>(0x011f_9138);
    e.set_global(0x011f_9138, value.wrapping_add(1));
}

// Translated from 0086ef60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at `011f4461`.
pub fn fn_0086ef60(e: &mut Engine) -> u8 {
    e.global::<u8>(0x011f_4461)
}

// Translated from 0086ef70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at +0x52 of `this`.
pub fn fn_0086ef70(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x52)
}

// Translated from 0086ef90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `0.0` in the `float` at `01267c1c`.
pub fn fn_0086ef90(e: &mut Engine) {
    e.set_global(0x0126_7c1c, 0.0f32);
}

// Translated from 0086efa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// 1 when the object at `011daac0` exists and its method `004a4080(0x10000)` says yes, else 0.
pub fn fn_0086efa0(e: &mut Engine) -> u8 {
    let object = e.global::<u32>(0x011d_aac0);
    if object != 0 && e.call(0x004a_4080, &args![object, 0x1_0000u32]).u8() != 0 {
        1
    } else {
        0
    }
}

/// The three `float` setting objects [`fn_0086efe0`] hands to its setters, or `None` for the
/// default `1.0` triple.
fn light_settings(e: &mut Engine) -> Option<[u32; 3]> {
    if e.call(0x0052_5420, &args![]).u8() != 0 {
        return Some([0x011d_ec24, 0x011d_eb10, 0x011d_ed8c]);
    }
    let tes = e.global::<u32>(TES_OBJECT);
    if e.call(0x004f_d3e0, &args![tes]).u32() != 0 {
        let tes = e.global::<u32>(TES_OBJECT);
        let world_space = e.call(0x004f_d3e0, &args![tes]).u32();
        if e.call(0x0058_6390, &args![world_space, 1u32]).u32() != 0 {
            return Some([0x011d_ec7c, 0x011d_ee54, 0x011d_ed98]);
        }
    }
    let player = e.global::<u32>(PLAYER_OBJECT);
    if player != 0 && e.call(0x008d_6f30, &args![player]).u32() != 0 {
        let process = e.call(0x008d_6f30, &args![player]).u32();
        if e.call(0x0042_5fd0, &args![process]).u8() != 0 {
            return Some([0x011d_ea80, 0x011d_eddc, 0x011d_ecc8]);
        }
    }
    None
}

// Translated from 0086efe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks one of three triples of `float` settings and hands them to the setters
/// [`fn_0086f160`], [`fn_0086f170`] and [`fn_0086f180`]: the first when `00525420` says so, the
/// second when the current world space (`004fd3e0`, then `00586390(.., 1)`) qualifies, the
/// third when the player's object `008d6f30` returns exists and `00425fd0` says yes; otherwise
/// `1.0` three times.
pub fn fn_0086efe0(e: &mut Engine, _this: Ptr) {
    match light_settings(e) {
        Some([first, second, third]) => {
            let value = float_setting(e, first);
            fn_0086f160(e, value);
            let value = float_setting(e, second);
            fn_0086f170(e, value);
            let value = float_setting(e, third);
            fn_0086f180(e, value);
        }
        None => {
            fn_0086f160(e, 1.0);
            fn_0086f170(e, 1.0);
            fn_0086f180(e, 1.0);
        }
    }
}

// Translated from 0086f160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a `float` in the global at `011ad804`.
pub fn fn_0086f160(e: &mut Engine, value: f32) {
    e.set_global(0x011a_d804, value);
}

// Translated from 0086f170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a `float` in the global at `011ad800`.
pub fn fn_0086f170(e: &mut Engine, value: f32) {
    e.set_global(0x011a_d800, value);
}

// Translated from 0086f180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a `float` in the global at `011ad7fc`.
pub fn fn_0086f180(e: &mut Engine, value: f32) {
    e.set_global(0x011a_d7fc, value);
}

// Translated from 0086f190 (decompiled, FalloutNV.exe 1.4.0.525)
/// While a menu is up (the flag at `011dea2b`), the first call after it came up runs through the
/// list `00717e50` returns for [`PROCESS_LISTS`]: for every entry that exists, whose current
/// process type (`MobileObject::GetCurrentProcessType`, `00931850`) is 0 and whose virtual
/// method at +0x1d0 answers non-zero, calls `00483710` on the object `008d8520` returns; then
/// the same for the player. Without a menu it only re-arms the flag at `011deefd`.
pub fn fn_0086f190(e: &mut Engine, _this: Ptr) {
    if !byte_flag(e, IN_MENU_FLAG) {
        e.set_global(0x011d_eefd, 1u8);
        return;
    }
    if byte_flag(e, 0x011d_eefd) {
        let list = e.call(0x0071_7e50, &args![PROCESS_LISTS]).u32();
        let mut index = 0u32;
        while index < e.call(0x005b_e5c0, &args![list, 0u32]).u32() {
            let actor = e.call(0x0096_8670, &args![list, index]).u32();
            if actor != 0
                && e.call(0x0093_1850, &args![actor]).u32() == 0
                && e.vcall(actor, 0x1d0, &args![]).u32() != 0
            {
                let object = e.call(0x008d_8520, &args![actor]).u32();
                e.call(0x0048_3710, &args![object]);
            }
            index += 1;
        }
        let player = e.global::<u32>(PLAYER_OBJECT);
        let object = e.call(0x008d_8520, &args![player]).u32();
        e.call(0x0048_3710, &args![object]);
    }
    e.set_global(0x011d_eefd, 0u8);
}

// Translated from 0086f260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the timer: sets its mode from the byte setting at `011dedbc` (`008a8150`), starts the
/// frame clock unless a menu is up (`00aa4e40`), copies a `float` setting (`011d1320`) to
/// `011afe60`, runs the physics step `00c66760(seconds, 00525420(), mode == 4)`, refreshes the
/// values with [`fn_0086f330`] and stores in `011dea30` the seconds times the scale setting
/// `011c5724` (0 while a menu is up).
pub fn fn_0086f260(e: &mut Engine, _this: Ptr) {
    let address = e.call(SETTING_BYTE_PTR, &args![0x011d_edbcu32]).u32();
    let mode_byte = e.mem.u8(address) as u32;
    e.call(0x008a_8150, &args![TIMER, mode_byte]);
    if !byte_flag(e, IN_MENU_FLAG) {
        e.call(0x00aa_4e40, &args![]);
    }
    e.call(0x0077_1520, &args![TIMER]);
    let value = float_setting(e, 0x011d_1320);
    e.set_global(0x011a_fe60, value);
    let mode = e.call(0x0044_ddc0, &args![MODE_OBJECT]).u32();
    let stepping = (mode == 4) as u32;
    let flag = e.call(0x0052_5420, &args![]).u8() as u32;
    let seconds = timer_seconds(e);
    e.call(0x00c6_6760, &args![seconds, flag, stepping]);
    fn_0086f330(e);
    let scaled = if byte_flag(e, IN_MENU_FLAG) {
        0.0
    } else {
        let seconds = timer_seconds(e) as f64;
        let scale = float_setting(e, 0x011c_5724) as f64;
        (scale * seconds) as f32
    };
    e.set_global(0x011d_ea30, scaled);
}

// Translated from 0086f330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the four `float`s at `011f940c` with `f(0)` .. `f(3)`, where `f` is the function whose
/// address is stored at `011f91f0` (cdecl, one index, result in `ST0`); 0.0 without one.
pub fn fn_0086f330(e: &mut Engine) {
    for index in 0..4u32 {
        let function = e.global::<u32>(0x011f_91f0);
        let value = if function != 0 {
            e.call(function, &args![index]).f32()
        } else {
            0.0
        };
        e.set_global(0x011f_940c + index * 4, value);
    }
}

// Translated from 0086f390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main::OnIdle_PollControls` (Xbox PDB): polls the controls (`00a23010`) and tells them the
/// frame time (`00a257c0`); then, unless the camera flies, the player's virtual method +0x22c
/// says no, the Pipboy test passes, neither the menu 0x3e9 is visible nor `004a4040` says yes,
/// clears the user actions (`00a253d0`).
pub fn main_on_idle_poll_controls(e: &mut Engine, this: Ptr) {
    let controls = e.call(0x007f_df30, &args![]).u32();
    e.call(0x00a2_3010, &args![controls]);
    let seconds = timer_frame_seconds(e);
    let controls = e.call(0x007f_df30, &args![]).u32();
    e.call(0x00a2_57c0, &args![controls, seconds]);
    if main_flag(e, this, MAIN_FLY_CAMERA) {
        return;
    }
    let player = e.global::<u32>(PLAYER_OBJECT);
    if e.vcall(player, 0x22c, &args![0u32]).u8() != 0 {
        return;
    }
    let mode = e.call(0x0044_ddc0, &args![MODE_OBJECT]).u32();
    let proceed = if mode == 4 && e.call(0x007d_1360, &args![player]).u8() == 0 {
        true
    } else {
        e.call(0x0070_9bc0, &args![]).u8() != 0
    };
    if !proceed {
        return;
    }
    if is_menu_visible(e, 0x3e9, 0) {
        return;
    }
    if e.call(0x004a_4040, &args![]).u8() != 0 {
        return;
    }
    let controls = e.call(0x007f_df30, &args![]).u32();
    e.call(0x00a2_53d0, &args![controls]);
}

// Translated from 0086f450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main::OnIdle_HandleMenuBackground` (Xbox PDB): while a menu is up and none of the menus that
/// bring their own background is open, renders the menu background (`00871dc0`) if the byte at
/// `011dea28` is set; otherwise, when the byte at `011dea29` is set and no menu that needs the
/// background is shown, kills the menu background texture (`00877430`).
pub fn main_on_idle_handle_menu_background(e: &mut Engine, this: Ptr) {
    if byte_flag(e, IN_MENU_FLAG)
        && !is_top_menu(e, 0x3f1)
        && e.call(0x0070_edf0, &args![]).u8() == 0
        && !is_menu_visible(e, 0x420, 0xb)
        && !is_menu_visible(e, 0x3e9, 0xb)
        && e.call(0x0070_5a00, &args![]).u8() == 0
        && e.call(0x0070_3d50, &args![]).u8() == 0
        && !byte_flag(e, SCENE_FLAG_29)
        && byte_flag(e, SCENE_FLAG_BYTE)
    {
        e.call(0x0087_1dc0, &args![this]);
        return;
    }
    if !byte_flag(e, SCENE_FLAG_29) {
        return;
    }
    let background_may_go = !byte_flag(e, IN_MENU_FLAG)
        || e.call(0x0070_edf0, &args![]).u8() != 0
        || (e.call(0x0070_3d50, &args![]).u8() != 0 && e.call(0x0070_79b0, &args![]).u8() == 0)
        || is_top_menu(e, 0x3f1)
        || is_top_menu(e, 0x420);
    if !background_may_go {
        return;
    }
    let fader = e.global::<u32>(FADER_MANAGER);
    if e.call(0x0070_14a0, &args![fader, 1u32]).u8() != 0 {
        return;
    }
    for menu_id in [0x41e, 0x3f6, 0x424, 0x432, 0x438, 0x439, 0x43a, 0x43b] {
        if is_menu_visible(e, menu_id, 0) {
            return;
        }
    }
    e.call(0x0087_7430, &args![this]);
}

// Translated from 0086f640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the audio: `00ad7740(audio, 1)` on the object `00453a70` returns, then
/// `00832ad0(0)`, `0082fb70` and `0082d7c0`.
pub fn fn_0086f640(e: &mut Engine, _this: Ptr) {
    let audio = e.call(0x0045_3a70, &args![]).u32();
    e.call(0x00ad_7740, &args![audio, 1u32]);
    e.call(0x0083_2ad0, &args![0u32]);
    e.call(0x0082_fb70, &args![]);
    e.call(0x0082_d7c0, &args![]);
}

// Translated from 0086f670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the bytes at `012682f8`, `011df678` and `011dea2a`, then calls `005a38e0` and
/// `00455490` on the object at [`TES_OBJECT`].
pub fn fn_0086f670(e: &mut Engine, _this: Ptr) {
    e.set_global(0x0126_82f8, 0u8);
    e.set_global(0x011d_f678, 0u8);
    e.set_global(0x011d_ea2a, 0u8);
    e.call(0x005a_38e0, &args![]);
    let tes = e.global::<u32>(TES_OBJECT);
    e.call(0x0045_5490, &args![tes]);
}

// Translated from 0086f6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main::UpdateNonRenderSafeAITasks` (Xbox PDB): resets a dialog menu's timer when a dialog is
/// open, then updates the process lists, the Pipboy and the task managers for the frame.
pub fn main_update_non_render_safe_ai_tasks(e: &mut Engine, this: Ptr) {
    if e.call(0x0070_50d0, &args![]).u8() != 0
        && e.call(0x0070_3d50, &args![]).u8() == 0
        && fn_0086f860(e) == 0
    {
        let dialog = e.call(0x0076_24d0, &args![]).u32();
        if dialog != 0 && e.vcall(dialog, 0x100, &args![]).u8() != 0 {
            e.vcall(dialog, 0x404, &args![0.0f32]);
        }
    }
    e.call(0x0096_c240, &args![PROCESS_LISTS, 0.0f32]);
    if processor_count(e) > 1 {
        e.call(0x0096_c970, &args![PROCESS_LISTS]);
    } else {
        e.call(0x0096_c860, &args![PROCESS_LISTS]);
    }
    e.call(0x0096_c710, &args![PROCESS_LISTS]);
    let seconds = timer_seconds(e);
    e.call(0x0097_81d0, &args![PROCESS_LISTS, seconds]);
    let pipboy = e.call(0x0070_5990, &args![]).u32();
    if fn_0086f840(e, Ptr::new(pipboy)) != 0 {
        e.call(0x007f_a990, &args![pipboy]);
    }
    if processor_count(e) > 1 {
        e.call(0x0054_ae30, &args![]);
        let object = e.call(0x0045_37b0, &args![]).u32();
        e.call(0x0087_a6d0, &args![object]);
        let object = e.call(0x0045_37b0, &args![]).u32();
        e.call(0x0087_a790, &args![object]);
        e.call(0x0055_2570, &args![]);
        let value = e.global::<f32>(0x011c_3c08);
        fn_0086f830(e, value);
        let object = e.call(0x0045_37b0, &args![]).u32();
        e.call(0x0087_a6b0, &args![object]);
    }
    let address = e.call(SETTING_INT_PTR, &args![0x011d_10d4u32]).u32();
    let value = e.mem.u32(address);
    e.call(0x0070_34c0, &args![value]);
    e.call(0x0070_3490, &args![]);
    let hold = byte_flag(e, IN_MENU_FLAG)
        || byte_flag(e, FADER_ONE_FLAG)
        || main_flag(e, this, MAIN_FREEZE_TIME);
    let object = e.global::<u32>(0x011f_1958);
    e.call(0x0099_1600, &args![object, hold as u32]);
}

// Translated from 0086f830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a `float` in the global at `011c40ec`.
pub fn fn_0086f830(e: &mut Engine, value: f32) {
    e.set_global(0x011c_40ec, value);
}

// Translated from 0086f840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at +0x16c of `this`.
pub fn fn_0086f840(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x16c)
}

// Translated from 0086f860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at +0x131 of the object at `011d9510`, or 0 without one.
pub fn fn_0086f860(e: &mut Engine) -> u8 {
    let object = e.global::<u32>(0x011d_9510);
    if object != 0 {
        e.mem.u8(object + 0x131)
    } else {
        0
    }
}

// Translated from 0086f890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main::OnIdle_UpdateProcessLists` (Xbox PDB): with one processor runs `0096eb40` on the
/// process lists first; then, when the world steps are allowed, runs `008c94e0` and `00991dc0`
/// (with one processor) and `ProcessLists::UpdateProcessLists` (`0096d810`); otherwise, while a
/// menu is up or time is frozen, `PlayerCharacter::UpdateAutoAimActor` (`00964260`).
pub fn main_on_idle_update_process_lists(e: &mut Engine, this: Ptr) {
    if processor_count(e) == 1 {
        e.call(0x0096_eb40, &args![PROCESS_LISTS]);
    }
    if world_steps_allowed(e, this) {
        if processor_count(e) == 1 {
            e.call(0x008c_94e0, &args![0x011e_0fe0u32]);
            let object = e.global::<u32>(0x011f_1958);
            e.call(0x0099_1dc0, &args![object]);
        }
        e.call(0x0096_d810, &args![PROCESS_LISTS]);
    } else if byte_flag(e, IN_MENU_FLAG) || main_flag(e, this, MAIN_FREEZE_TIME) {
        let player = e.global::<u32>(PLAYER_OBJECT);
        e.call(0x0096_4260, &args![player]);
    }
}

// Translated from 0086f940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main::OnIdle_UpdatePlayer` (Xbox PDB): lets the player handle a position request
/// (`0093bea0`); else, while a menu is up without the Pipboy opening, runs `009481d0`; else with
/// the fly camera updates the player (`009466d0`); otherwise runs the player's per-frame step
/// (virtual +0x2f8 with the VATS time multiplier) and, when the player stands in a cell that is
/// not interior and not yet loaded (`00550200`), finds the cell under it (`00461bc0`), points
/// the current-cell tracker at the position (`00452580`), attaches the player to that cell
/// (`00548230`) and notifies the shader accumulator (`00b655b0`).
pub fn main_on_idle_update_player(e: &mut Engine, this: Ptr) {
    let player = e.global::<u32>(PLAYER_OBJECT);
    if e.call(0x0093_bea0, &args![player]).u8() != 0 {
        return;
    }
    if byte_flag(e, IN_MENU_FLAG) && e.call(0x0070_9bc0, &args![]).u8() == 0 {
        e.call(0x0094_81d0, &args![player]);
        return;
    }
    if main_flag(e, this, MAIN_FLY_CAMERA) {
        if e.vcall(player, 0x1d0, &args![]).u32() != 0 {
            let freeze = e.mem.u8(this.addr() + MAIN_FREEZE_TIME) as u32;
            let seconds = timer_seconds(e);
            e.call(0x0094_66d0, &args![player, seconds, freeze]);
        }
        return;
    }
    if e.vcall(player, 0x1d0, &args![]).u32() != 0 {
        let depth = e.global::<u8>(0x011e_07a8);
        e.set_global(0x011e_07a8, depth.wrapping_add(1));
        let seconds = timer_seconds(e) as f64;
        let multiplier = e.call(0x009c_8cc0, &args![MODE_OBJECT]).f32() as f64;
        let step = (multiplier * seconds) as f32;
        e.vcall(player, 0x2f8, &args![step]);
        let depth = e.global::<u8>(0x011e_07a8);
        e.set_global(0x011e_07a8, depth.wrapping_sub(1));
    }
    let cell = e.call(0x008d_6f30, &args![player]).u32();
    let position = e.call(0x0043_6aa0, &args![player]).u32();
    let coordinates = [
        e.mem.u32(position),
        e.mem.u32(position + 4),
        e.mem.u32(position + 8),
    ];
    if cell == 0 || e.call(0x0042_5fd0, &args![cell]).u8() != 0 {
        return;
    }
    e.with_stack(12, |e, buffer| {
        for (index, word) in coordinates.iter().enumerate() {
            e.mem.set_u32(buffer.addr() + index as u32 * 4, *word);
        }
        if e.call(0x0055_0200, &args![cell, buffer]).u8() != 0 {
            return;
        }
        let world_space = e.call(0x0054_ddd0, &args![cell]).u32();
        let handler = e.global::<u32>(DATA_HANDLER_OBJECT);
        let grid_cell = e
            .call(
                0x0046_1bc0,
                &args![
                    handler,
                    f32::from_bits(coordinates[0]),
                    f32::from_bits(coordinates[1]),
                    world_space,
                    1u32
                ],
            )
            .u32();
        if grid_cell == 0 {
            return;
        }
        if e.call(0x0045_0fb0, &args![grid_cell]).u8() == 0
            && e.call(0x0045_0ff0, &args![grid_cell]).u8() == 0
        {
            let tes = e.global::<u32>(TES_OBJECT);
            e.call(0x0045_2580, &args![tes, buffer, 1u32]);
            let tes = e.global::<u32>(TES_OBJECT);
            if e.call(0x0045_1530, &args![tes]).u8() != 0 {
                e.call(0x0045_7d70, &args![tes, 0u32, 0u32, 0u32]);
            }
        }
        fn_0086fba0(e, 1);
        fn_0086fbc0(e, Ptr::new(player), 1);
        e.call(0x0054_8230, &args![grid_cell, player, 0u32]);
        let acoustic_space = e.call(0x0054_7590, &args![grid_cell]).u32();
        fn_0086fbb0(e, acoustic_space);
        fn_0086fbc0(e, Ptr::new(player), 0);
        fn_0086fba0(e, 0);
        let accumulator = e.call(0x00b4_f5c0, &args![]).u32();
        if accumulator != 0 {
            e.call(0x00b6_55b0, &args![accumulator]);
        }
    });
}

// Translated from 0086fba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte in the global at `011dcfa6`.
pub fn fn_0086fba0(e: &mut Engine, value: u8) {
    e.set_global(0x011d_cfa6, value);
}

// Translated from 0086fbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a word in the global at `011dcfb8`.
pub fn fn_0086fbb0(e: &mut Engine, value: u32) {
    e.set_global(0x011d_cfb8, value);
}

// Translated from 0086fbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte at +0x5f9 of `this` (the player).
pub fn fn_0086fbc0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x5f9, value);
}

// Translated from 0086fbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// While no menu is up, the byte at `011d8907` is clear and time is not frozen: points the
/// current-cell tracker (`00452580`) at the player's position (`00436aa0`) and, when it has a
/// cell (`00451530`) and `005f36f0` says there is none, refreshes the player's world space
/// (`00575d70`) and the tracker's (`004fd3e0`).
pub fn fn_0086fbe0(e: &mut Engine, this: Ptr) {
    if byte_flag(e, IN_MENU_FLAG)
        || byte_flag(e, 0x011d_8907)
        || main_flag(e, this, MAIN_FREEZE_TIME)
    {
        return;
    }
    let player = e.global::<u32>(PLAYER_OBJECT);
    let position = e.call(0x0043_6aa0, &args![player]).u32();
    let tes = e.global::<u32>(TES_OBJECT);
    e.call(0x0045_2580, &args![tes, position, 1u32]);
    let tes = e.global::<u32>(TES_OBJECT);
    if e.call(0x0045_1530, &args![tes]).u8() == 0 {
        return;
    }
    if e.call(0x005f_36f0, &args![tes]).u32() != 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER_OBJECT);
    e.call(0x0057_5d70, &args![player]);
    let tes = e.global::<u32>(TES_OBJECT);
    e.call(0x004f_d3e0, &args![tes]);
}

// Translated from 0086fc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// When time is not frozen and (no menu is up, or a dialog or the fader flag is set): updates the
/// process lists for the frame (`009746c0` with several processors, `00974420` otherwise), the
/// current cell (`00453550` with the scaled seconds at `011dea30` with one processor and no
/// menu, `004537c0` otherwise) and the particle systems, with a temporary object built by
/// `0043d410(seconds, 1, 0)`.
pub fn fn_0086fc60(e: &mut Engine, this: Ptr) {
    let active =
        !byte_flag(e, IN_MENU_FLAG) || byte_flag(e, IN_DIALOG_FLAG) || byte_flag(e, FADER_ONE_FLAG);
    if !active || main_flag(e, this, MAIN_FREEZE_TIME) {
        return;
    }
    let seconds = timer_seconds(e);
    if processor_count(e) > 1 {
        e.call(0x0097_46c0, &args![PROCESS_LISTS, seconds]);
    } else {
        e.call(0x0097_4420, &args![PROCESS_LISTS, seconds]);
    }
    let tes = e.global::<u32>(TES_OBJECT);
    if processor_count(e) == 1 && !byte_flag(e, IN_MENU_FLAG) {
        let scaled = e.global::<f32>(0x011d_ea30);
        e.call(0x0045_3550, &args![tes, scaled]);
    } else {
        e.call(0x0045_37c0, &args![tes]);
    }
    let scaled = e.global::<f32>(0x011d_ea30);
    e.with_stack(12, |e, buffer| {
        e.call(0x0043_d410, &args![buffer, scaled, 1u32, 0u32]);
        // The game tests the processor count again here, but both arms run the same two calls.
        // 0086fd43: `0045a190` takes no word (plain RET, no stack read); the pushed buffer
        // is the word of `00c50610` (RET 4).
        let holder = e.call(0x0045_a190, &args![]).u32();
        e.call(0x00c5_0610, &args![holder, buffer]);
    });
}

// Translated from 0086fd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `007027e0`, `00702810` and `00702840`.
pub fn fn_0086fd70(e: &mut Engine, _this: Ptr) {
    e.call(0x0070_27e0, &args![]);
    e.call(0x0070_2810, &args![]);
    e.call(0x0070_2840, &args![]);
}

// Translated from 0086fd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main::OnIdle_UpdateImageSpace` (Xbox PDB): updates the image-space manager (`00b8b500`,
/// `00b8b9a0`), adds the seconds to the shader time when `running` ([`fn_0086ff50`]), then walks
/// the list `0043b5d0` returns for the current cell: nodes whose virtual +0x8c says yes are
/// taken out of the list (`00633c90` builds a temporary, `00631620` removes it); every fourth
/// node counted by its byte at +8 ends a batch (`00b8aea0` when none of the batch had a positive
/// value at virtual +0x90, then `00b8ccb0`); the manager is finished with `00b8d020` and
/// `00b8b440(0)`.
///
/// Not translated: the exception-unwinding state of the frame (the temporary built by
/// `00633c90` is destroyed by `0045cec0` on the normal path).
pub fn main_on_idle_update_image_space(e: &mut Engine, _this: Ptr, running: u8) {
    let manager = e.call(0x004e_3270, &args![]).u32();
    e.call(0x00b8_b500, &args![manager]);
    let manager = e.call(0x004e_3270, &args![]).u32();
    e.call(0x00b8_b9a0, &args![manager]);
    if running != 0 {
        let seconds = timer_seconds(e);
        fn_0086ff50(e, seconds);
    }
    let tes = e.global::<u32>(TES_OBJECT);
    let mut list = e.call(0x0043_b5d0, &args![tes]).u32();
    let mut counted = 0u32;
    let mut positive = 0u32;
    while list != 0 {
        let holder = e.call(0x0068_15c0, &args![list]).u32();
        let node = e.call(POINTER_GET, &args![holder]).u32();
        if node != 0 {
            if e.mem.u8(node + 8) != 0 {
                counted += 1;
            }
            if e.vcall(node, 0x8c, &args![]).u8() != 0 {
                let current = list;
                e.with_stack(4, |e, temporary| {
                    e.call(0x0063_3c90, &args![temporary, node]);
                    e.call(0x0063_1620, &args![current, temporary]);
                    e.call(0x0045_cec0, &args![temporary]);
                });
                continue;
            }
            if fn_0086ff40(e) != 0 {
                e.vcall(node, 0x90, &args![]);
                let value = e.call(TIMER_GET_SECONDS, &args![node]).f32() as f64;
                if value > e.global::<f64>(0x0101_2060) && e.mem.u8(node + 8) != 0 {
                    positive += 1;
                }
            }
        }
        list = e.call(0x0072_6070, &args![list]).u32();
        if counted == 4 {
            if positive == 0 {
                let manager = e.call(0x004e_3270, &args![]).u32();
                e.call(0x00b8_aea0, &args![manager]);
            }
            let manager = e.call(0x004e_3270, &args![]).u32();
            e.call(0x00b8_ccb0, &args![manager]);
            counted = 0;
        }
    }
    let manager = e.call(0x004e_3270, &args![]).u32();
    e.call(0x00b8_d020, &args![manager]);
    let manager = e.call(0x004e_3270, &args![]).u32();
    e.call(0x00b8_b440, &args![manager, 0u32]);
}

// Translated from 0086ff40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at `0118abb0`.
pub fn fn_0086ff40(e: &mut Engine) -> u8 {
    e.global::<u8>(0x0118_abb0)
}

// Translated from 0086ff50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `seconds` to the `float` at `011c96fc`.
pub fn fn_0086ff50(e: &mut Engine, seconds: f32) {
    let total = e.global::<f32>(0x011c_96fc) as f64 + seconds as f64;
    e.set_global(0x011c_96fc, total as f32);
}

/// The eight menus whose being visible stops the frame's world update in [`fn_0086ff70`].
const COVERING_MENUS: [u32; 8] = [0x41e, 0x3f6, 0x424, 0x432, 0x438, 0x439, 0x43a, 0x43b];

// Translated from 0086ff70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The second half of the idle update (rendering and menus): flushes the renderer, redraws the
/// menu background after a menu change, ages the fade-in counter (`011def00`/`011def04`) and
/// then, depending on whether the game world or a menu is shown, runs the world update
/// (`008706b0`) or the menu update (`00871a50`, `00872940`, `00874b90`); toggles a debug
/// counter on the keys of the actions 0x1e / 0x9d (making a numbered directory); and closes the
/// frame (`00c458f0`, ...).
///
/// Not translated: the stack-cookie check at the end (`00ec408c`).
pub fn fn_0086ff70(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let renderer = e.call(GET_RENDERER, &args![]).u32();
    e.call(0x004a_0370, &args![renderer]);
    if byte_flag(e, 0x011d_890a) {
        e.set_global(0x011d_890a, 0u8);
        let player = e.global::<u32>(PLAYER_OBJECT);
        let target = e.call(0x0095_0bb0, &args![player, 1u32]).u32();
        let saved_first = e.call(0x0045_6610, &args![target]).u8();
        let target = e.call(0x0095_0bb0, &args![player, 0u32]).u32();
        let saved_second = e.call(0x0045_6610, &args![target]).u8();
        let target = e.call(0x0095_0bb0, &args![player, 1u32]).u32();
        e.call(0x0045_0f90, &args![target, 1u32]);
        let target = e.call(0x0095_0bb0, &args![player, 0u32]).u32();
        e.call(0x0045_0f90, &args![target, 1u32]);
        e.call(0x0087_1dc0, &args![me]);
        let target = e.call(0x0095_0bb0, &args![player, 1u32]).u32();
        e.call(0x0045_0f90, &args![target, saved_first as u32]);
        let target = e.call(0x0095_0bb0, &args![player, 0u32]).u32();
        e.call(0x0045_0f90, &args![target, saved_second as u32]);
    }
    let renderer_again = e.call(GET_RENDERER, &args![]).u32();
    e.call(0x0055_85e0, &args![renderer_again]);
    e.call(0x00a2_9680, &args![]);
    e.call(ERROR_LOG, &args![]);
    let threads = e.call(0x004e_a970, &args![]).u32();
    e.call(0x00ba_2f30, &args![threads]);
    if byte_flag(e, 0x011c_6fb8) {
        e.call(0x004d_cef0, &args![]);
    }
    let fader = e.global::<u32>(FADER_MANAGER);
    if e.call(0x0070_1450, &args![fader, 1u32]).u8() != 0
        || e.call(0x0070_1450, &args![fader, 2u32]).u8() != 0
    {
        if e.global::<u32>(0x011d_ef04) == 0 {
            let tes = e.global::<u32>(TES_OBJECT);
            e.call(0x0045_7d70, &args![tes, 0u32, 0u32, 0u32]);
            e.call(ERROR_LOG, &args![]);
            e.call(0x0096_cfa0, &args![PROCESS_LISTS]);
        }
        let seconds = timer_frame_seconds(e) as f64;
        let elapsed = (seconds + e.global::<f32>(0x011d_ef00) as f64) as f32;
        e.set_global(0x011d_ef00, elapsed);
        let frames = e.global::<u32>(0x011d_ef04).wrapping_add(1);
        e.set_global(0x011d_ef04, frames);
        if (elapsed as f64) >= e.global::<f64>(0x0101_2070) && frames as i32 >= 10 {
            let fader = e.global::<u32>(FADER_MANAGER);
            e.call(0x0070_10e0, &args![fader, 1u32, 0u32]);
            e.call(0x0070_10e0, &args![fader, 2u32, 0u32]);
            let object = e.global::<u32>(0x011d_e45c);
            e.call(0x0045_34f0, &args![object, 0u32]);
            let object = e.global::<u32>(0x011d_e45c);
            if e.call(0x0047_c850, &args![object]).u8() != 0 {
                let object = e.global::<u32>(0x011d_e45c);
                e.call(0x0045_34f0, &args![object, 0u32]);
                let object = e.global::<u32>(0x011d_e45c);
                e.call(0x0048_3710, &args![object]);
            }
            e.call(0x0087_7430, &args![me]);
        }
    } else {
        e.set_global(0x011d_ef00, 0.0f32);
        e.set_global(0x011d_ef04, 0u32);
    }
    // A render target the game keeps in a local that nothing ever sets.
    let pending_target = 0u32;
    let player = e.global::<u32>(PLAYER_OBJECT);
    let fader = e.global::<u32>(FADER_MANAGER);
    let tes = e.global::<u32>(TES_OBJECT);
    if !byte_flag(e, SCENE_FLAG_29)
        && e.call(0x0070_edf0, &args![]).u8() == 0
        && e.vcall(player, 0x1d0, &args![]).u32() != 0
        && e.call(0x0070_1400, &args![fader, 1u32]).u8() == 0
    {
        if e.call(0x005f_36f0, &args![tes]).u32() == 0 {
            let world_space = e.call(0x004f_d3e0, &args![tes]).u32();
            let cell_data = e.call(0x0054_8210, &args![world_space]).u32();
            if (e.mem.u8(cell_data) as i8) != 0 && e.call(0x0044_ddc0, &args![tes]).u32() != 0 {
                e.call(0x0044_ddc0, &args![tes]);
                if e.call(0x0087_05c0, &args![]).u8() != 0 {
                    let mode = e.call(0x0044_ddc0, &args![tes]).u32();
                    e.call(0x004b_af10, &args![mode]);
                }
            }
        }
        e.call(0x0087_06b0, &args![me, 0u32, 0u32, 0u32]);
    } else {
        let rendered = e.call(0x0070_79b0, &args![]).u8() != 0;
        let menu_up = (!rendered && e.call(IS_IN_MENU_MODE, &args![]).u8() != 0)
            || is_menu_visible(e, 0x3f5, 0);
        if menu_up {
            for stage in 1..0x17u32 {
                let threads = e.call(0x004e_a970, &args![]).u32();
                e.call(0x00ba_30f0, &args![threads, 0u32, stage]);
                let threads = e.call(0x004e_a970, &args![]).u32();
                e.call(0x00ba_3130, &args![threads, 1u32, stage]);
            }
            e.call(0x0087_1a50, &args![me]);
            let covered = COVERING_MENUS
                .into_iter()
                .any(|menu_id| is_menu_visible(e, menu_id, 0));
            if covered {
                e.call(0x0087_2940, &args![me, 0u32]);
            } else if e.call(POINTER_GET, &args![0x011d_ed3cu32]).u32() != 0 {
                let flag = e.call(0x004d_c310, &args![]).u8() as u32;
                let manager = e.call(0x004e_3270, &args![]).u32();
                e.call(0x0087_4b90, &args![me, manager, renderer, flag, 0u32]);
            }
        } else if e.vcall(player, 0x1d0, &args![]).u32() != 0 {
            let rendered = e.call(0x0070_79b0, &args![]).u8() as u32;
            e.call(0x0087_06b0, &args![me, 0u32, rendered, 0u32]);
        }
    }
    if e.call(0x005d_4a40, &args![]).u8() == 1 {
        e.call(0x00b5_5a10, &args![]);
    }
    e.call(0x0070_9b40, &args![]);
    let holder_value = e.call(POINTER_GET, &args![0x011d_ec64u32]).u32();
    e.call(0x0070_28b0, &args![pending_target, holder_value]);
    let threads = e.call(0x004e_a970, &args![]).u32();
    e.call(0x00ba_2fa0, &args![threads]);
    if e.call(POINTER_GET, &args![0x011d_ec64u32]).u32() != 0 {
        let value = e.call(POINTER_GET, &args![0x011d_ec64u32]).u32();
        let manager = e.call(0x004a_0ea0, &args![]).u32();
        e.call(0x00b6_da10, &args![manager, value]);
        pointer_set(e, 0x011d_ec64, 0);
    }
    let controls = e.call(0x007f_df30, &args![]).u32();
    if e.call(0x00a2_4660, &args![controls, 0x1eu32, 1u32]).u32() != 0 {
        let controls = e.call(0x007f_df30, &args![]).u32();
        let pressed = e.call(0x00a2_4180, &args![controls, 0x9du32, 0u32]).u32() != 0;
        if pressed || byte_flag(e, 0x011d_ea40) {
            let shown = !byte_flag(e, 0x011d_ea40);
            e.set_global(0x011d_ea40, shown as u8);
            if shown {
                e.set_global(0x011d_ea44, 0u32);
                let address = e.call(SETTING_INT_PTR, &args![0x011d_eeccu32]).u32();
                let count = e.mem.u32(address);
                e.call(0x0045_ce80, &args![0x011d_eeccu32, count.wrapping_add(1)]);
                let address = e.call(SETTING_INT_PTR, &args![0x011d_eeccu32]).u32();
                let count = e.mem.u32(address);
                let text = e.call(0x0040_3df0, &args![0x011d_eb04u32]).u32();
                e.with_stack(0x104, |e, buffer| {
                    e.call(
                        0x0040_6d00,
                        &args![buffer, 0x104u32, 0x0108_2ca4u32, text, count],
                    );
                    e.call(API_CREATE_DIRECTORY, &args![buffer, 0u32]);
                });
                let address = e.call(SETTING_INT_PTR, &args![0x011d_ee00u32]).u32();
                let value = e.mem.u32(address);
                fn_0086d4c0(e, Ptr::new(TIMER), value);
            } else {
                fn_0086d4c0(e, Ptr::new(TIMER), 0);
            }
        } else {
            e.call(0x0087_8860, &args![0u32]);
        }
    }
    e.call(0x00c4_58f0, &args![]);
    if e.call(0x004e_9530, &args![renderer]).u32() != 0 {
        e.call(0x00b6_b730, &args![]);
    }
    if pending_target != 0 {
        let manager = e.call(0x004a_0ea0, &args![]).u32();
        e.call(0x00b6_da10, &args![manager, pending_target]);
    }
    let holder_value = e.call(POINTER_GET, &args![SCENE_GRAPH_HOLDER]).u32();
    e.call(0x00c5_1f20, &args![holder_value]);
    e.call(0x004a_03c0, &args![renderer]);
    if processor_count(e) > 1 {
        e.call(0x008c_80e0, &args![0u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0086a480, shader_timer(i32) -> f32),
        entry!(0x0086a530, fn_0086a530()),
        entry!(0x0086a580, fn_0086a580(u32, u32, u32) -> i32),
        entry!(0x0086a820, fn_0086a820() -> u32),
        entry!(0x0086a830, fn_0086a830(u32, u32) -> u8),
        entry!(0x0086a850, main(u32, u32, u32, u32) -> u32),
        entry!(0x0086ba10, fn_0086ba10(Ptr) -> bool),
        entry!(0x0086ba60, fn_0086ba60(Ptr, u32, u32) -> bool),
        entry!(0x0086ba90, fn_0086ba90(Ptr) -> bool),
        entry!(0x0086bae0, fn_0086bae0(Ptr, u32, u32) -> u32),
        entry!(0x0086bb20, fn_0086bb20(Ptr, u32) -> bool),
        entry!(0x0086bb70, fn_0086bb70(Ptr, u32) -> u32),
        entry!(0x0086bbd0, fn_0086bbd0(Ptr, u32, u32) -> u32),
        entry!(0x0086bc10, fn_0086bc10(Ptr, u32) -> bool),
        entry!(0x0086bc60, fn_0086bc60(Ptr, u32) -> u32),
        entry!(0x0086bcc0, fn_0086bcc0(u8)),
        entry!(0x0086bcd0, fn_0086bcd0(u32)),
        entry!(0x0086bce0, fn_0086bce0(u32)),
        entry!(0x0086bd00, fn_0086bd00()),
        entry!(0x0086bd40, fn_0086bd40(Ptr, u32) -> Ptr),
        entry!(0x0086bd70, fn_0086bd70()),
        entry!(0x0086bd90, fn_0086bd90(u32)),
        entry!(0x0086bda0, fn_0086bda0(Ptr)),
        entry!(0x0086bdd0, fn_0086bdd0()),
        entry!(0x0086bde0, fn_0086bde0(u32)),
        entry!(0x0086bdf0, fn_0086bdf0() -> u8),
        entry!(0x0086be30, fn_0086be30(u32)),
        entry!(0x0086be80, fn_0086be80(u32)),
        entry!(0x0086be90, fn_0086be90(u32)),
        entry!(0x0086bea0, fn_0086bea0(u8)),
        entry!(0x0086beb0, fn_0086beb0()),
        entry!(0x0086bee0, fn_0086bee0(Ptr, u32) -> Ptr),
        entry!(0x0086bf10, fn_0086bf10(Ptr, u32) -> Ptr),
        entry!(0x0086bf40, fn_0086bf40(Ptr, u32) -> Ptr),
        entry!(0x0086bf70, main_shader_init()),
        entry!(0x0086bff0, fn_0086bff0(u32)),
        entry!(0x0086c000, fn_0086c000(u8)),
        entry!(0x0086c010, fn_0086c010()),
        entry!(0x0086c0c0, fn_0086c0c0() -> f32),
        entry!(0x0086c0d0, fn_0086c0d0() -> f32),
        entry!(0x0086c0e0, fn_0086c0e0() -> f32),
        entry!(0x0086c0f0, fn_0086c0f0() -> f32),
        entry!(0x0086c100, fn_0086c100() -> f32),
        entry!(0x0086c110, fn_0086c110() -> f32),
        entry!(0x0086c120, fn_0086c120() -> f32),
        entry!(0x0086c130, fn_0086c130() -> f32),
        entry!(0x0086c140, fn_0086c140(f32)),
        entry!(0x0086c150, fn_0086c150(f32)),
        entry!(0x0086c160, main_main(Ptr, u32, u32) -> Ptr),
        entry!(0x0086c600, bs_packed_task_queue_thread_begin_input(Ptr)),
        entry!(0x0086c620, fn_0086c620(Ptr, u32, u32) -> Ptr),
        entry!(0x0086c6a0, fn_0086c6a0(Ptr, u32, u32) -> Ptr),
        entry!(0x0086c6e0, fn_0086c6e0(Ptr)),
        entry!(0x0086c750, fn_0086c750(Ptr)),
        entry!(0x0086c790, fn_0086c790(u32)),
        entry!(0x0086c850, fn_0086c850(Ptr, u32) -> Ptr),
        entry!(0x0086c880, fn_0086c880(Ptr)),
        entry!(0x0086cd80, fn_0086cd80()),
        entry!(0x0086cdd0, fn_0086cdd0(Ptr)),
        entry!(0x0086cdf0, fn_0086cdf0()),
        entry!(0x0086ce40, fn_0086ce40(Ptr)),
        entry!(0x0086cf00, fn_0086cf00(Ptr, u32)),
        entry!(0x0086cf20, main_init_tes(Ptr, u32)),
        entry!(0x0086d470, fn_0086d470() -> u32),
        entry!(0x0086d480, fn_0086d480() -> u32),
        entry!(0x0086d4c0, fn_0086d4c0(Ptr, u32)),
        entry!(0x0086d500, fn_0086d500(Ptr)),
        entry!(0x0086d590, main_init_scene_graph(Ptr)),
        entry!(0x0086e3f0, fn_0086e3f0(Ptr, u16, u16, u32) -> u32),
        entry!(0x0086e420, fn_0086e420(Ptr, u32, f32, f32, f32, f32) -> u32),
        entry!(0x0086e460, fn_0086e460(Ptr, u32, u16, f32, f32, f32, f32) -> u32),
        entry!(0x0086e4b0, fn_0086e4b0(Ptr)),
        entry!(0x0086e4d0, fn_0086e4d0(u32)),
        entry!(0x0086e4e0, fn_0086e4e0(u32)),
        entry!(0x0086e500, fn_0086e500(u8)),
        entry!(0x0086e510, fn_0086e510(u8)),
        entry!(0x0086e520, fn_0086e520(u32)),
        entry!(0x0086e540, fn_0086e540() -> u8),
        entry!(0x0086e560, fn_0086e560() -> u8),
        entry!(0x0086e580, fn_0086e580() -> u8),
        entry!(0x0086e5a0, fn_0086e5a0() -> u8),
        entry!(0x0086e5c0, fn_0086e5c0() -> u8),
        entry!(0x0086e5e0, fn_0086e5e0() -> u8),
        entry!(0x0086e600, fn_0086e600() -> u8),
        entry!(0x0086e620, fn_0086e620(u32)),
        entry!(0x0086e630, fn_0086e630(u32)),
        entry!(0x0086e650, fn_0086e650(Ptr)),
        entry!(0x0086ef30, fn_0086ef30()),
        entry!(0x0086ef40, fn_0086ef40()),
        entry!(0x0086ef60, fn_0086ef60() -> u8),
        entry!(0x0086ef70, fn_0086ef70(Ptr) -> u8),
        entry!(0x0086ef90, fn_0086ef90()),
        entry!(0x0086efa0, fn_0086efa0() -> u8),
        entry!(0x0086efe0, fn_0086efe0(Ptr)),
        entry!(0x0086f160, fn_0086f160(f32)),
        entry!(0x0086f170, fn_0086f170(f32)),
        entry!(0x0086f180, fn_0086f180(f32)),
        entry!(0x0086f190, fn_0086f190(Ptr)),
        entry!(0x0086f260, fn_0086f260(Ptr)),
        entry!(0x0086f330, fn_0086f330()),
        entry!(0x0086f390, main_on_idle_poll_controls(Ptr)),
        entry!(0x0086f450, main_on_idle_handle_menu_background(Ptr)),
        entry!(0x0086f640, fn_0086f640(Ptr)),
        entry!(0x0086f670, fn_0086f670(Ptr)),
        entry!(0x0086f6a0, main_update_non_render_safe_ai_tasks(Ptr)),
        entry!(0x0086f830, fn_0086f830(f32)),
        entry!(0x0086f840, fn_0086f840(Ptr) -> u8),
        entry!(0x0086f860, fn_0086f860() -> u8),
        entry!(0x0086f890, main_on_idle_update_process_lists(Ptr)),
        entry!(0x0086f940, main_on_idle_update_player(Ptr)),
        entry!(0x0086fba0, fn_0086fba0(u8)),
        entry!(0x0086fbb0, fn_0086fbb0(u32)),
        entry!(0x0086fbc0, fn_0086fbc0(Ptr, u8)),
        entry!(0x0086fbe0, fn_0086fbe0(Ptr)),
        entry!(0x0086fc60, fn_0086fc60(Ptr)),
        entry!(0x0086fd70, fn_0086fd70(Ptr)),
        entry!(0x0086fd90, main_on_idle_update_image_space(Ptr, u8)),
        entry!(0x0086ff40, fn_0086ff40() -> u8),
        entry!(0x0086ff50, fn_0086ff50(f32)),
        entry!(0x0086ff70, fn_0086ff70(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Every callee of this file's functions that lives elsewhere (each is a no-op returning 0
    /// until a test replaces it).
    const CALLEES: [u32; 208] = [
        0x0040_1000,
        0x0040_1020,
        0x0040_1030,
        0x0040_3d30,
        0x0040_3df0,
        0x0040_3e20,
        0x0040_4eb0,
        0x0040_4ee0,
        0x0040_4f00,
        0x0040_4f30,
        0x0040_4f70,
        0x0040_6d30,
        0x0040_6d50,
        0x0040_8d60,
        0x0040_8de0,
        0x0040_fbe0,
        0x0041_6870,
        0x0043_8af0,
        0x0043_c4b0,
        0x0043_d4d0,
        0x0044_6e10,
        0x0044_8420,
        0x0044_ddc0,
        0x0044_edb0,
        0x0044_f560,
        0x0045_0410,
        0x0045_0770,
        0x0045_1530,
        0x0045_3720,
        0x0045_3a70,
        0x0045_6520,
        0x0045_8200,
        0x0045_b020,
        0x0045_b050,
        0x0045_c670,
        0x0045_d180,
        0x0046_0140,
        0x0046_14e0,
        0x0046_50a0,
        0x0047_0000,
        0x0047_d0b0,
        0x0048_3710,
        0x0049_fef0,
        0x0049_ff80,
        0x004a_0370,
        0x004a_03c0,
        0x004a_0ea0,
        0x004a_4340,
        0x004b_4a30,
        0x004b_7920,
        0x004b_8cf0,
        0x004d_a4d0,
        0x004d_a570,
        0x004d_c020,
        0x004d_c110,
        0x004d_c1e0,
        0x004d_c1f0,
        0x004d_c200,
        0x004d_c560,
        0x004d_c590,
        0x004d_e490,
        0x004d_eb30,
        0x004d_ef20,
        0x004e_0540,
        0x004e_0800,
        0x004e_2190,
        0x004e_3270,
        0x004e_bbc0,
        0x004f_15a0,
        0x004f_b090,
        0x0052_6eb0,
        0x0055_9450,
        0x005b_9f80,
        0x005c_5a60,
        0x005d_c960,
        0x005e_01b0,
        0x005e_0200,
        0x0062_2bc0,
        0x0062_6f10,
        0x0065_19f0,
        0x0065_1ab0,
        0x0065_8930,
        0x0066_29f0,
        0x0066_4940,
        0x0066_b0d0,
        0x0067_8ce0,
        0x0068_15c0,
        0x0068_3a60,
        0x006a_2920,
        0x006a_29a0,
        0x006c_0720,
        0x006c_07d0,
        0x006c_0900,
        0x006d_0870,
        0x006e_b8c0,
        0x006e_b980,
        0x006e_cd40,
        0x006f_f580,
        0x006f_f610,
        0x006f_f620,
        0x006f_f880,
        0x0070_05d0,
        0x0070_0650,
        0x0070_2250,
        0x0070_22c0,
        0x0070_2330,
        0x0070_2360,
        0x0070_3610,
        0x0070_3e80,
        0x0070_3fa0,
        0x0070_3fd0,
        0x0070_53f0,
        0x0070_5910,
        0x0070_6320,
        0x0071_1e00,
        0x0071_3d80,
        0x0071_7e50,
        0x0078_cfc0,
        0x0078_d020,
        0x007c_fd00,
        0x007d_6bd0,
        0x007f_fe00,
        0x0080_26b0,
        0x0082_5c00,
        0x0082_f9c0,
        0x0083_04a0,
        0x0083_0640,
        0x0083_0660,
        0x0083_2010,
        0x0083_2ad0,
        0x0084_d030,
        0x0085_26c0,
        0x0086_6ff0,
        0x0086_7090,
        0x0086_7da0,
        0x0086_c0e0,
        0x0086_c0f0,
        0x0086_c100,
        0x0086_c110,
        0x0086_c120,
        0x0086_c130,
        0x0086_c140,
        0x0086_c150,
        0x0086_c160,
        0x0086_c880,
        0x0086_cf20,
        0x0086_d500,
        0x0086_d590,
        0x0086_e650,
        0x0087_1c90,
        0x0087_6a70,
        0x0087_6ad0,
        0x0087_6d20,
        0x0087_6dd0,
        0x0087_7260,
        0x0087_74a0,
        0x0087_7720,
        0x0087_7950,
        0x0087_7a30,
        0x0087_7a50,
        0x0087_83c0,
        0x0087_84d0,
        0x008a_8150,
        0x008c_7290,
        0x008c_73a0,
        0x008d_6f30,
        0x008d_8520,
        0x0095_ee80,
        0x0097_0d50,
        0x0099_0ff0,
        0x009e_98d0,
        0x00a1_c6f0,
        0x00a1_c790,
        0x00a2_3c00,
        0x00a6_1ad0,
        0x00a6_2090,
        0x00a8_19e0,
        0x00aa_20b0,
        0x00aa_2170,
        0x00aa_47c0,
        0x00aa_5040,
        0x00aa_50a0,
        0x00ad_7740,
        0x00ad_8700,
        0x00ad_8740,
        0x00af_2640,
        0x00af_4540,
        0x00af_dc60,
        0x00af_dcf0,
        0x00af_dda0,
        0x00af_f100,
        0x00b0_0a00,
        0x00b0_12f0,
        0x00b6_3620,
        0x00b6_da10,
        0x00bc_2940,
        0x00bc_2960,
        0x00c3_a6d0,
        0x00c3_bb90,
        0x00c3_bbe0,
        0x00c3_bc30,
        0x00c3_bc80,
        0x00c3_bd30,
        0x00c5_d4e0,
        0x00c5_d520,
        0x00ec_16d0,
        0x00ec_1800,
        0x00ec_c071,
    ];
    const IMPORTS: [u32; 27] = [
        API_CREATE_MUTEX,
        API_GET_LAST_ERROR,
        API_MESSAGE_BOX,
        API_GET_SYSTEM_METRICS,
        API_GET_SYSTEM_INFO,
        API_FIND_WINDOW,
        API_SET_FOREGROUND_WINDOW,
        API_GET_COMMAND_LINE,
        API_COMMAND_LINE_TO_ARGV,
        API_LOCAL_FREE,
        API_SHELL_EXECUTE,
        API_LOAD_ICON,
        API_GET_STOCK_OBJECT,
        API_REGISTER_CLASS,
        API_ADJUST_WINDOW_RECT,
        API_CREATE_WINDOW,
        API_PEEK_MESSAGE,
        API_TRANSLATE_MESSAGE,
        API_DISPATCH_MESSAGE,
        API_GET_ACTIVE_WINDOW,
        API_SLEEP,
        API_GET_CURRENT_THREAD_ID,
        API_GET_WINDOW_LONG,
        API_ADJUST_WINDOW_RECT_EX,
        API_SET_WINDOW_POS,
        API_IS_DEBUGGER_PRESENT,
        API_EXIT_PROCESS,
    ];

    /// A byte that stays 0 and a 32-bit value: what the setting getters point at by default.
    const ZERO_BYTE: u32 = 0x0100_0100;
    const ZERO_WORD: u32 = 0x0100_0200;
    /// A function registered as a no-op, for vtable slots.
    const NOTHING: u32 = 0x0000_1000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        // The exe's constants, strings and globals.
        e.map(0x0100_0000, 0x0028_0000);
        for address in CALLEES.iter().chain(IMPORTS.iter()) {
            e.register(*address, |_, _| Ret::default());
        }
        e.register(NOTHING, |_, _| Ret::default());
        e.register(SETTING_BYTE_PTR, |_, _| ZERO_BYTE.into_ret());
        e.register(SETTING_INT_PTR, |_, _| ZERO_WORD.into_ret());
        e
    }

    fn take_log(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap()
    }

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn position(log: &[(u32, Vec<u32>)], address: u32) -> usize {
        log.iter()
            .position(|(a, _)| *a == address)
            .unwrap_or_else(|| panic!("{address:08x} was not called"))
    }

    /// An object whose vtable has the given (byte offset, function) slots.
    fn object_with_vtable(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x200);
        for (offset, function) in slots {
            e.mem.set_u32(table + offset, *function);
        }
        let object = e.mem.alloc(0x300);
        e.mem.set_u32(object, table);
        object
    }

    fn ret_f32(value: f32) -> Ret {
        value.into_ret()
    }

    // ----- shader_timer ---------------------------------------------------------------------

    #[test]
    fn shader_timer_mode_3_is_the_timer_seconds() {
        let mut e = engine();
        e.register(TIMER_GET_SECONDS, |_, _| ret_f32(2.5));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0086_a480, &args![3i32]).f32(), 2.5);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, TIMER_GET_SECONDS), vec![vec![TIMER]]);
    }

    #[test]
    fn shader_timer_mode_2_is_milliseconds_as_seconds_unless_in_a_menu() {
        let mut e = engine();
        e.set_global(MILLISECONDS_PER_SECOND, 1000.0f64);
        e.register(TIMER_GET_MILLISECONDS, |_, _| 1500u32.into_ret());
        assert_eq!(e.call(0x0086_a480, &args![2i32]).f32(), 1.5);
        e.register(IS_IN_MENU_MODE, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x0086_a480, &args![2i32]).f32(), 0.0);
    }

    #[test]
    fn shader_timer_mode_1_is_the_timer_seconds_unless_in_a_menu() {
        let mut e = engine();
        e.register(TIMER_GET_SECONDS, |_, _| ret_f32(7.25));
        assert_eq!(e.call(0x0086_a480, &args![1i32]).f32(), 7.25);
        e.register(IS_IN_MENU_MODE, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x0086_a480, &args![1i32]).f32(), 0.0);
    }

    #[test]
    fn shader_timer_mode_0_is_the_game_hour_in_seconds() {
        let mut e = engine();
        e.set_global(SIXTY, 60.0f64);
        e.register(CALENDAR_GET_HOUR, |_, _| ret_f32(12.5));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0086_a480, &args![0i32]).f32(), 45000.0);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, CALENDAR_GET_HOUR), vec![vec![CALENDAR]]);
    }

    #[test]
    fn shader_timer_other_modes_are_zero() {
        let mut e = engine();
        assert_eq!(e.call(0x0086_a480, &args![4i32]).f32(), 0.0);
        assert_eq!(e.call(0x0086_a480, &args![-1i32]).f32(), 0.0);
    }

    // ----- the message box --------------------------------------------------------------------

    #[test]
    fn warning_menu_callback_exits_for_results_1_and_3() {
        for result in [1u32, 3] {
            let mut e = engine();
            e.register_double(GET_MESSAGE_MENU_RESULT, move |_, _| result.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_a530, &args![]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, ERROR_LOG), vec![vec![0x0108_28c0]]);
            assert_eq!(calls_to(&log, API_EXIT_PROCESS), vec![vec![0]]);
            assert!(calls_to(&log, CLOSE_MESSAGE_MENU).is_empty());
        }
    }

    #[test]
    fn warning_menu_callback_closes_the_menu_for_result_2_and_ignores_others() {
        let mut e = engine();
        e.register(GET_MESSAGE_MENU_RESULT, |_, _| 2u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0086_a530, &args![]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, CLOSE_MESSAGE_MENU).len(), 1);
        assert!(calls_to(&log, API_EXIT_PROCESS).is_empty());

        e.register(GET_MESSAGE_MENU_RESULT, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0086_a530, &args![]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, CLOSE_MESSAGE_MENU).is_empty());
        assert!(calls_to(&log, API_EXIT_PROCESS).is_empty());
    }

    #[test]
    fn message_box_answers_without_showing_when_the_setting_is_on() {
        let mut e = engine();
        e.mem.set_u8(ZERO_BYTE, 1);
        e.call_log = Some(vec![]);
        // MB_ABORTRETRYIGNORE: IDIGNORE; anything else: IDYES.
        assert_eq!(e.call(0x0086_a580, &args![1u32, 2u32, 2u32]).i32(), 5);
        assert_eq!(e.call(0x0086_a580, &args![1u32, 2u32, 4u32]).i32(), 6);
        assert_eq!(e.call(0x0086_a580, &args![1u32, 2u32, 0x34u32]).i32(), 6);
        let log = take_log(&mut e);
        assert!(calls_to(&log, API_MESSAGE_BOX).is_empty());
        assert!(calls_to(&log, TIMER_DISABLE).is_empty());
        assert_eq!(
            calls_to(&log, SETTING_BYTE_PTR)[0],
            vec![SETTING_NO_MESSAGE_BOXES]
        );
    }

    #[test]
    fn message_box_shows_a_windows_box_around_a_disabled_timer() {
        let mut e = engine();
        e.register(API_MESSAGE_BOX, |_, _| 7u32.into_ret());
        e.call_log = Some(vec![]);
        let result = e
            .call(0x0086_a580, &args![0x111u32, 0x222u32, 0x4u32])
            .i32();
        assert_eq!(result, 7);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, API_MESSAGE_BOX)[0], vec![0, 0x111, 0x222, 4]);
        assert!(position(&log, TIMER_DISABLE) < position(&log, API_MESSAGE_BOX));
        assert!(position(&log, API_MESSAGE_BOX) < position(&log, TIMER_ENABLE));
        assert_eq!(calls_to(&log, TIMER_DISABLE), vec![vec![TIMER]]);
        assert_eq!(calls_to(&log, TIMER_ENABLE), vec![vec![TIMER]]);
        // No warning menu is available, so it is not touched.
        assert!(calls_to(&log, SHOW_MESSAGE_MENU).is_empty());
    }

    #[test]
    fn message_box_is_not_shown_when_the_window_setting_and_state_say_so() {
        let mut e = engine();
        e.register(GET_WINDOW_SETTING, |_, _| 1u32.into_ret());
        e.register(GET_WINDOW_STATE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0086_a580, &args![1u32, 2u32, 4u32]).i32(), 6);
        let log = take_log(&mut e);
        assert!(calls_to(&log, API_MESSAGE_BOX).is_empty());
        assert_eq!(calls_to(&log, TIMER_ENABLE).len(), 1);
        // With the setting on but the state 0 it is shown after all.
        e.register(GET_WINDOW_STATE, |_, _| 0u32.into_ret());
        e.register(API_MESSAGE_BOX, |_, _| 3u32.into_ret());
        assert_eq!(e.call(0x0086_a580, &args![1u32, 2u32, 4u32]).i32(), 3);
    }

    #[test]
    fn message_box_from_another_thread_sets_the_helper_flag_first() {
        let mut e = engine();
        e.set_global(MAIN_OBJECT, 0x5000u32);
        e.register(API_GET_CURRENT_THREAD_ID, |_, _| 10u32.into_ret());
        e.register(GET_MAIN_THREAD_ID, |_, _| 20u32.into_ret());
        e.register(MAIN_OBJECT_GET_HELPER, |_, _| 0x6000u32.into_ret());
        e.register(HELPER_HAS_FLAG, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0086_a580, &args![1u32, 2u32, 0u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, GET_MAIN_THREAD_ID), vec![vec![0x5000]]);
        assert_eq!(calls_to(&log, HELPER_SET_FLAG), vec![vec![0x6000, 1]]);
        assert!(position(&log, HELPER_SET_FLAG) < position(&log, API_MESSAGE_BOX));

        // A helper that already has the flag, or the main thread: nothing is set.
        e.register(HELPER_HAS_FLAG, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0086_a580, &args![1u32, 2u32, 0u32]);
        assert!(calls_to(&take_log(&mut e), HELPER_SET_FLAG).is_empty());
        e.register(GET_MAIN_THREAD_ID, |_, _| 10u32.into_ret());
        e.register(HELPER_HAS_FLAG, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0086_a580, &args![1u32, 2u32, 0u32]);
        assert!(calls_to(&take_log(&mut e), HELPER_SET_FLAG).is_empty());
    }

    /// An engine where the warning menu is available and the main object has a helper.
    fn warning_menu_engine() -> Engine {
        let mut e = engine();
        e.set_global(WARNING_MENU_AVAILABLE, 1u8);
        e.set_global(MAIN_OBJECT, 0x5000u32);
        e.set_global(WARNING_MENU_CONTEXT, 0xabcdu32);
        e.register(MAIN_OBJECT_GET_HELPER, |_, _| 0x6000u32.into_ret());
        e.register(API_GET_CURRENT_THREAD_ID, |_, _| 10u32.into_ret());
        e.register(GET_MAIN_THREAD_ID, |_, _| 10u32.into_ret());
        // Pooled texts are numbered after the pool object they come from.
        e.register(GET_POOLED_TEXT, |_, a| (a[0] + 1).into_ret());
        e
    }

    #[test]
    fn message_box_opens_the_warning_menu_with_button_texts_for_style_3() {
        let mut e = warning_menu_engine();
        e.register(SHOW_MESSAGE_MENU, |e, _| {
            // While the menu is open it is marked unavailable.
            assert_eq!(e.global::<u8>(WARNING_MENU_AVAILABLE), 0);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e
            .call(0x0086_a580, &args![0x111u32, 0x222u32, 0x33u32])
            .i32();
        assert_eq!(result, 6);
        let log = take_log(&mut e);
        let menu = calls_to(&log, SHOW_MESSAGE_MENU);
        assert_eq!(menu.len(), 1);
        assert_eq!(
            menu[0],
            vec![
                0x111,
                0,
                0,
                0x0086_a530,
                0,
                0xabcd,
                0,
                0,
                BUTTON_TEXT_SECOND + 1,
                BUTTON_TEXT_FIRST + 1,
                BUTTON_TEXT_THIRD + 1,
                0
            ]
        );
        assert!(calls_to(&log, API_MESSAGE_BOX).is_empty());
        assert_eq!(e.global::<u8>(WARNING_MENU_AVAILABLE), 1);
        assert!(position(&log, TIMER_ENABLE) > position(&log, SHOW_MESSAGE_MENU));
    }

    #[test]
    fn message_box_style_4_fetches_two_button_texts() {
        let mut e = warning_menu_engine();
        e.register(SHOW_MESSAGE_MENU, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0086_a580, &args![0x111u32, 0x222u32, 0x4u32]);
        let log = take_log(&mut e);
        let texts: Vec<u32> = calls_to(&log, GET_POOLED_TEXT)
            .iter()
            .map(|a| a[0])
            .collect();
        assert_eq!(texts, vec![BUTTON_TEXT_SECOND, BUTTON_TEXT_FIRST]);
        let menu = &calls_to(&log, SHOW_MESSAGE_MENU)[0];
        assert_eq!(
            &menu[8..],
            &[BUTTON_TEXT_SECOND + 1, BUTTON_TEXT_FIRST + 1, 0, 0]
        );
    }

    #[test]
    fn message_box_styles_0_and_2_skip_the_menu() {
        for style in [0u32, 2] {
            let mut e = warning_menu_engine();
            e.register(API_MESSAGE_BOX, |_, _| 9u32.into_ret());
            e.call_log = Some(vec![]);
            let result = e.call(0x0086_a580, &args![0x111u32, 0x222u32, style]).i32();
            assert_eq!(result, 9);
            let log = take_log(&mut e);
            assert!(calls_to(&log, SHOW_MESSAGE_MENU).is_empty());
            assert!(calls_to(&log, GET_POOLED_TEXT).is_empty());
            assert_eq!(e.global::<u8>(WARNING_MENU_AVAILABLE), 1);
        }
    }

    #[test]
    fn message_box_falls_back_when_the_menu_does_not_open() {
        let mut e = warning_menu_engine();
        e.register(SHOW_MESSAGE_MENU, |_, _| 0u32.into_ret());
        e.register(API_MESSAGE_BOX, |_, _| 4u32.into_ret());
        assert_eq!(e.call(0x0086_a580, &args![1u32, 2u32, 4u32]).i32(), 4);
        // Suppressed: styles 3 and 4 answer 6, and so does any other style.
        e.register(GET_WINDOW_SETTING, |_, _| 1u32.into_ret());
        e.register(GET_WINDOW_STATE, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x0086_a580, &args![1u32, 2u32, 4u32]).i32(), 6);
        assert_eq!(e.call(0x0086_a580, &args![1u32, 2u32, 3u32]).i32(), 6);
        assert_eq!(e.call(0x0086_a580, &args![1u32, 2u32, 1u32]).i32(), 6);
        // Style 2 never opens the menu but is answered 5 when suppressed.
        assert_eq!(e.call(0x0086_a580, &args![1u32, 2u32, 2u32]).i32(), 5);
        assert_eq!(e.global::<u8>(WARNING_MENU_AVAILABLE), 1);
    }

    #[test]
    fn warning_menu_context_getter_and_the_method_that_ignores_its_arguments() {
        let mut e = engine();
        e.set_global(WARNING_MENU_CONTEXT, 0x1234_5678u32);
        assert_eq!(e.call(0x0086_a820, &args![]).u32(), 0x1234_5678);
        assert_eq!(e.call(0x0086_a830, &args![0x4000u32, 99u32]).u8(), 0);
    }

    // ----- the renderer's frame guards and arrays -----------------------------------------------

    fn renderer_with(e: &mut Engine, state: u32, accept: u32) -> u32 {
        // Slots 0x17c / 0x180 answer `accept`.
        let begin = 0x0000_2000u32;
        let end = 0x0000_2004u32;
        e.register_double(begin, move |_, _| accept.into_ret());
        e.register_double(end, move |_, _| accept.into_ret());
        let renderer = object_with_vtable(e, &[(0x17c, begin), (0x180, end)]);
        e.mem.set_u32(renderer + RENDERER_FRAME_STATE, state);
        renderer
    }

    #[test]
    fn frame_state_check_compares_the_state_word() {
        let mut e = engine();
        let renderer = e.mem.alloc(0x300);
        e.mem.set_u32(renderer + 0x200, 2);
        assert!(e
            .call(0x0086_ba60, &args![renderer, 0x1111u32, 2u32])
            .bool());
        assert!(!e
            .call(0x0086_ba60, &args![renderer, 0x1111u32, 0u32])
            .bool());
    }

    #[test]
    fn begin_off_screen_frame_needs_state_0_and_an_accepting_method() {
        let mut e = engine();
        let renderer = renderer_with(&mut e, 0, 1);
        assert!(e.call(0x0086_ba10, &args![renderer]).bool());
        assert_eq!(e.mem.u32(renderer + 0x200), 2);

        let renderer = renderer_with(&mut e, 1, 1);
        assert!(!e.call(0x0086_ba10, &args![renderer]).bool());
        assert_eq!(e.mem.u32(renderer + 0x200), 1);

        let renderer = renderer_with(&mut e, 0, 0);
        assert!(!e.call(0x0086_ba10, &args![renderer]).bool());
        assert_eq!(e.mem.u32(renderer + 0x200), 0);
    }

    #[test]
    fn end_off_screen_frame_needs_state_2_and_an_accepting_method() {
        let mut e = engine();
        let renderer = renderer_with(&mut e, 2, 1);
        assert!(e.call(0x0086_ba90, &args![renderer]).bool());
        assert_eq!(e.mem.u32(renderer + 0x200), 0);

        let renderer = renderer_with(&mut e, 0, 1);
        assert!(!e.call(0x0086_ba90, &args![renderer]).bool());

        let renderer = renderer_with(&mut e, 2, 0);
        assert!(!e.call(0x0086_ba90, &args![renderer]).bool());
        assert_eq!(e.mem.u32(renderer + 0x200), 2);
    }

    /// Doubles for the array count and element methods over keys.
    fn arrays_with_keys(e: &mut Engine, keys: Vec<u32>) {
        let keys = Rc::new(RefCell::new(keys));
        let for_count = keys.clone();
        e.register_double(ARRAY_COUNT, move |_, _| {
            (for_count.borrow().len() as u32).into_ret()
        });
        e.register_double(ARRAY_ELEMENT, move |e, a| {
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, keys.borrow()[a[1] as usize]);
            slot.into_ret()
        });
    }

    #[test]
    fn finding_a_key_returns_its_index_or_minus_one() {
        let mut e = engine();
        arrays_with_keys(&mut e, vec![5, 9, 12]);
        let renderer = Ptr::<()>::new(0x7000);
        assert_eq!(e.call(0x0086_bb70, &args![renderer, 9u32]).u32(), 1);
        assert_eq!(e.call(0x0086_bb70, &args![renderer, 12u32]).u32(), 2);
        assert_eq!(e.call(0x0086_bb70, &args![renderer, 3u32]).u32(), u32::MAX);
        assert_eq!(e.call(0x0086_bc60, &args![renderer, 5u32]).u32(), 0);
        e.call_log = Some(vec![]);
        e.call(0x0086_bb70, &args![renderer, 5u32]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, ARRAY_COUNT),
            vec![vec![0x7000 + KEY_ARRAY_A]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0086_bc60, &args![renderer, 5u32]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, ARRAY_COUNT),
            vec![vec![0x7000 + KEY_ARRAY_B]]
        );
    }

    #[test]
    fn removing_a_key_removes_it_from_both_arrays() {
        let mut e = engine();
        arrays_with_keys(&mut e, vec![5, 9, 12]);
        let renderer = Ptr::<()>::new(0x7000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0086_bb20, &args![renderer, 12u32]).bool());
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, ARRAY_REMOVE_AT),
            vec![
                vec![0x7000 + KEY_ARRAY_A, 2],
                vec![0x7000 + DATA_ARRAY_A, 2]
            ]
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0086_bc10, &args![renderer, 5u32]).bool());
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, ARRAY_REMOVE_AT),
            vec![
                vec![0x7000 + KEY_ARRAY_B, 0],
                vec![0x7000 + DATA_ARRAY_B, 0]
            ]
        );
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0086_bb20, &args![renderer, 77u32]).bool());
        assert!(!e.call(0x0086_bc10, &args![renderer, 77u32]).bool());
        assert!(calls_to(&take_log(&mut e), ARRAY_REMOVE_AT).is_empty());
    }

    #[test]
    fn registering_a_pair_adds_the_key_then_sets_the_data_at_its_index() {
        for (entry, key_array, data_array) in [
            (0x0086_bae0u32, KEY_ARRAY_A, DATA_ARRAY_A),
            (0x0086_bbd0, KEY_ARRAY_B, DATA_ARRAY_B),
        ] {
            let mut e = engine();
            let seen = Rc::new(RefCell::new(vec![]));
            let record = seen.clone();
            e.register_double(ARRAY_ADD, move |e, a| {
                record.borrow_mut().push(("add", a[0], e.mem.u32(a[1])));
                4u32.into_ret()
            });
            let record = seen.clone();
            e.register_double(ARRAY_SET_AT_GROW, move |e, a| {
                record
                    .borrow_mut()
                    .push(("set", a[0], a[1] * 1000 + e.mem.u32(a[2])));
                Ret::default()
            });
            let index = e
                .call(entry, &args![Ptr::<()>::new(0x7000), 0xaaaau32, 0xbbbbu32])
                .u32();
            assert_eq!(index, 4);
            assert_eq!(
                *seen.borrow(),
                vec![
                    ("add", 0x7000 + key_array, 0xaaaa),
                    ("set", 0x7000 + data_array, 4 * 1000 + 0xbbbb)
                ]
            );
        }
    }

    // ----- globals and small wrappers -------------------------------------------------------------

    #[test]
    fn global_setters_store_their_argument() {
        let mut e = engine();
        e.call(0x0086_bcc0, &args![0x5u32]);
        assert_eq!(e.global::<u8>(0x011f_4448), 5);
        e.call(0x0086_bcd0, &args![0x1234_5678u32]);
        assert_eq!(e.global::<u32>(0x011f_6e08), 0x1234_5678);
        e.call(0x0086_bd90, &args![0x1111u32]);
        assert_eq!(e.global::<u32>(0x011f_6224), 0x1111);
        e.call(0x0086_bde0, &args![0x2222u32]);
        assert_eq!(e.global::<u32>(0x011f_9134), 0x2222);
        e.call(0x0086_be80, &args![0x3333u32]);
        assert_eq!(e.global::<u32>(0x011d_5c44), 0x3333);
        e.call(0x0086_be90, &args![0x4444u32]);
        assert_eq!(e.global::<u32>(0x011d_59ec), 0x4444);
        e.call(0x0086_bea0, &args![0x7u32]);
        assert_eq!(e.global::<u8>(0x011a_31f4), 7);
        e.call(0x0086_bff0, &args![0x5555u32]);
        assert_eq!(e.global::<u32>(0x011f_91f0), 0x5555);
        e.call(0x0086_c000, &args![0x9u32]);
        assert_eq!(e.global::<u8>(0x011f_94a7), 9);
    }

    #[test]
    fn the_default_for_011f6224_is_a_function_address() {
        let mut e = engine();
        e.call(0x0086_bd70, &args![]);
        assert_eq!(e.global::<u32>(0x011f_6224), 0x00b0_13f0);
    }

    #[test]
    fn call_on_the_object_at_011f81dc_only_when_there_is_one() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0086_bce0, &args![0x77u32]);
        assert!(calls_to(&take_log(&mut e), 0x00af_dda0).is_empty());
        e.set_global(OBJECT_AT_011F81DC, 0x8000u32);
        e.call_log = Some(vec![]);
        e.call(0x0086_bce0, &args![0x77u32]);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x00af_dda0),
            vec![vec![0x8000, 0x77]]
        );
    }

    #[test]
    fn destroying_the_object_at_011f81dc_runs_its_destructor_and_frees_it() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0086_bd00, &args![]);
        let log = take_log(&mut e);
        assert_eq!(log.len(), 1, "only the top-level call: {log:?}");

        e.set_global(OBJECT_AT_011F81DC, 0x8000u32);
        e.call_log = Some(vec![]);
        e.call(0x0086_bd00, &args![]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x00af_dcf0), vec![vec![0x8000]]);
        assert_eq!(calls_to(&log, FREE_OBJECT), vec![vec![0x8000]]);
        // The global is left as it was.
        assert_eq!(e.global::<u32>(OBJECT_AT_011F81DC), 0x8000);
    }

    #[test]
    fn deleting_destructors_free_only_when_bit_0_of_the_flags_is_set() {
        for (entry, body) in [
            (0x0086_bd40u32, 0x00af_dcf0u32),
            (0x0086_bee0, 0x0045_0770),
            (0x0086_bf10, 0x0070_0650),
            (0x0086_bf40, 0x0086_c880),
        ] {
            let mut e = engine();
            e.call_log = Some(vec![]);
            let this = Ptr::<()>::new(0x9000);
            assert_eq!(e.call(entry, &args![this, 1u32]).u32(), 0x9000);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, body), vec![vec![0x9000]]);
            assert_eq!(calls_to(&log, FREE_OBJECT), vec![vec![0x9000]]);

            e.call_log = Some(vec![]);
            assert_eq!(e.call(entry, &args![this, 0u32]).u32(), 0x9000);
            e.call(entry, &args![this, 2u32]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, body).len(), 2);
            assert!(calls_to(&log, FREE_OBJECT).is_empty());
        }
    }

    #[test]
    fn state_5c_moves_from_1_or_2_to_3_only() {
        let mut e = engine();
        let object = e.mem.alloc(0x80);
        for (before, after) in [(0u32, 0u32), (1, 3), (2, 3), (3, 3), (4, 4)] {
            e.mem.set_u32(object + 0x5c, before);
            e.call(0x0086_bda0, &args![Ptr::<()>::new(object)]);
            assert_eq!(e.mem.u32(object + 0x5c), after);
        }
    }

    #[test]
    fn call_on_the_object_at_011dd948() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0086_bdd0, &args![]);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x0043_8af0),
            vec![vec![0x011d_d948]]
        );
    }

    #[test]
    fn query_on_the_object_at_011da0c4() {
        let mut e = engine();
        assert_eq!(e.call(0x0086_bdf0, &args![]).u8(), 0);
        e.set_global(0x011d_a0c4u32, 0x8000u32);
        e.register(0x007c_fd00, |_, a| {
            assert_eq!(a[0], 0x8000);
            0u32.into_ret()
        });
        assert_eq!(e.call(0x0086_bdf0, &args![]).u8(), 0);
        e.register(0x007c_fd00, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x0086_bdf0, &args![]).u8(), 1);
    }

    #[test]
    fn deleting_the_object_at_011ccb78_through_its_first_virtual_method() {
        let mut e = engine();
        // Nothing there: the global stays 0.
        e.call(0x0086_be30, &args![0u32]);
        assert_eq!(e.global::<u32>(OBJECT_AT_011CCB78), 0);

        let seen = Rc::new(RefCell::new(vec![]));
        let record = seen.clone();
        e.register_double(0x0000_3000, move |_, a| {
            record.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        let object = object_with_vtable(&mut e, &[(0, 0x0000_3000)]);
        e.set_global(OBJECT_AT_011CCB78, object);
        e.call(0x0086_be30, &args![0x1234u32]);
        assert_eq!(*seen.borrow(), vec![(object, 1)]);
        assert_eq!(e.global::<u32>(OBJECT_AT_011CCB78), 0);
    }

    #[test]
    fn three_objects_get_zero_in_a_row() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0086_beb0, &args![]);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x0066_b0d0),
            vec![
                vec![0x011d_86bc, 0],
                vec![0x011d_86a8, 0],
                vec![0x011d_8690, 0]
            ]
        );
    }

    #[test]
    fn shader_init_copies_four_settings_and_installs_the_timer_callback() {
        let mut e = engine();
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u8(first, 0x11);
        e.mem.set_u8(second, 0x22);
        e.register_double(SETTING_BYTE_PTR, move |_, a| match a[0] {
            0x011d_ed68 => first.into_ret(),
            0x011d_eea8 => second.into_ret(),
            other => panic!("unexpected setting {other:08x}"),
        });
        e.register(0x0045_0410, |_, a| match a[0] {
            0x011d_ec18 => ret_f32(1.5),
            0x011d_eb70 => ret_f32(2.5),
            other => panic!("unexpected setting {other:08x}"),
        });
        e.call_log = Some(vec![]);
        e.call(0x0086_bf70, &args![]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, ERROR_LOG), vec![vec![0x0108_2af4]]);
        assert_eq!(calls_to(&log, 0x005c_5a60), vec![vec![0]]);
        assert_eq!(e.global::<u32>(0x011f_91f0), 0x0086_a480);
        assert_eq!(e.global::<u8>(0x011f_94a7), 0x11);
        assert_eq!(e.global::<u8>(0x011f_9443), 0x22);
        assert_eq!(e.global::<f32>(0x011f_9484), 1.5);
        assert_eq!(e.global::<f32>(0x011f_9480), 2.5);
    }

    #[test]
    fn float_setting_getters_forward_the_result() {
        let mut e = engine();
        e.register(0x0045_0410, |_, a| match a[0] {
            0x011c_72a0 => ret_f32(0.25),
            0x011c_774c => ret_f32(0.75),
            other => panic!("unexpected setting {other:08x}"),
        });
        assert_eq!(e.call(0x0086_c0c0, &args![]).f32(), 0.25);
        assert_eq!(e.call(0x0086_c0d0, &args![]).f32(), 0.75);
    }

    /// Doubles for the float getters, the two float consumers and the entry chain at the end of
    /// `0086c010`.
    fn viewport_engine() -> Engine {
        let mut e = engine();
        e.register(0x0045_0410, |_, a| match a[0] {
            0x011c_72a0 => ret_f32(10.0),
            0x011c_774c => ret_f32(20.0),
            other => panic!("unexpected setting {other:08x}"),
        });
        e.register(0x0086_c0e0, |_, _| ret_f32(30.0));
        e.register(0x0086_c0f0, |_, _| ret_f32(40.0));
        e.register(0x0086_c100, |_, _| ret_f32(50.0));
        e.register(0x0086_c110, |_, _| ret_f32(60.0));
        e.register(0x0086_c120, |_, _| ret_f32(70.0));
        e.register(0x0086_c130, |_, _| ret_f32(80.0));
        let float_cell = e.mem.alloc(4);
        e.mem.set_f32(float_cell, 0.5);
        e.register_double(0x0040_3e20, move |_, a| {
            assert_eq!(a[0], 0x011c_f8a4);
            float_cell.into_ret()
        });
        e.register(0x004e_3270, |_, _| 0xaa00u32.into_ret());
        e.register(0x004e_bbc0, |_, a| {
            assert_eq!(a, &[0xaa00, 0xc]);
            0xbb00u32.into_ret()
        });
        e
    }

    #[test]
    fn viewport_setup_with_the_first_set_of_getters() {
        let mut e = viewport_engine();
        e.call_log = Some(vec![]);
        e.call(0x0086_c010, &args![]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0086_c140), vec![vec![10.0f32.to_bits()]]);
        assert_eq!(calls_to(&log, 0x0086_c150), vec![vec![20.0f32.to_bits()]]);
        assert_eq!(e.global::<f32>(0x011a_dbcc), 30.0);
        assert_eq!(e.global::<f32>(0x011a_dbd0), 40.0);
        assert_eq!(
            calls_to(&log, 0x005b_9f80),
            vec![vec![0xbb00, 0.5f32.to_bits()]]
        );
        assert!(calls_to(&log, 0x0086_c100).is_empty());
    }

    #[test]
    fn viewport_setup_with_the_second_set_of_getters() {
        let mut e = viewport_engine();
        e.set_global(0x011f_941eu32, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x0086_c010, &args![]);
        let log = take_log(&mut e);
        assert_eq!(e.global::<f32>(0x011a_dbcc), 70.0);
        assert_eq!(e.global::<f32>(0x011a_dbd0), 80.0);
        assert_eq!(calls_to(&log, 0x0086_c140), vec![vec![50.0f32.to_bits()]]);
        assert_eq!(calls_to(&log, 0x0086_c150), vec![vec![60.0f32.to_bits()]]);
        assert_eq!(calls_to(&log, 0x005b_9f80).len(), 1);
        assert!(calls_to(&log, 0x0086_c0e0).is_empty());
    }

    // ----- main -------------------------------------------------------------------------------

    const INSTANCE: u32 = 0x0040_0000;
    const MAIN_ENTRY: u32 = 0x0086_a850;
    const WINDOW_CLASS_NAME: u32 = 0x0108_3000;

    fn run_main_entry(e: &mut Engine) -> u32 {
        e.call(MAIN_ENTRY, &args![INSTANCE, 0u32, 0u32, 0u32]).u32()
    }

    /// An engine where `main` can run to the end of its loop once: allocation works, the
    /// objects `main` calls through vtables exist, and the first loop test ends the loop.
    fn main_engine() -> Engine {
        let mut e = engine();
        e.set_global(0x011a_2fe8u32, WINDOW_CLASS_NAME);
        e.register(ALLOCATE_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FREE_OBJECT, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(0x0086_c160, |_, a| a[0].into_ret());
        e.register(0x0068_15c0, |_, _| ZERO_WORD.into_ret());
        e.register(0x0040_3e20, |_, _| ZERO_WORD.into_ret());
        e.register(0x0070_05d0, |_, a| a[0].into_ret());
        let device = object_with_vtable(&mut e, &[(0xc, NOTHING)]);
        e.register_double(0x004d_c020, move |_, _| device.into_ret());
        let with_destructor = object_with_vtable(&mut e, &[(8, NOTHING)]);
        for address in [0x0045_3a70u32, 0x00b0_0a00, 0x00af_f100] {
            e.register_double(address, move |_, _| with_destructor.into_ret());
        }
        // The loop ends after its first pass.
        e.register(0x005d_c960, |_, _| 1u32.into_ret());
        e
    }

    #[test]
    fn main_stops_when_the_mutex_already_exists() {
        let mut e = main_engine();
        e.register(API_GET_LAST_ERROR, |_, _| 0xb7u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(run_main_entry(&mut e), 0);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, API_CREATE_MUTEX),
            vec![vec![0, 0, 0x0108_2ab8]]
        );
        assert_eq!(
            calls_to(&log, API_MESSAGE_BOX),
            vec![vec![0, 0x0108_2a7c, 0x0104_df38, 0]]
        );
        assert!(calls_to(&log, API_GET_SYSTEM_METRICS).is_empty());
    }

    #[test]
    fn main_stops_in_a_remote_desktop_session() {
        let mut e = main_engine();
        e.register(API_GET_SYSTEM_METRICS, |_, a| {
            assert_eq!(a[0], 0x1000);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(run_main_entry(&mut e), 0);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, API_MESSAGE_BOX),
            vec![vec![0, 0x0108_2a48, 0x0104_df38, 0]]
        );
        assert!(calls_to(&log, 0x00c3_bd30).is_empty());
    }

    #[test]
    fn main_activates_a_running_instance_and_stops() {
        let mut e = main_engine();
        e.register(API_FIND_WINDOW, |_, _| 0x1234u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(run_main_entry(&mut e), 0);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, API_FIND_WINDOW),
            vec![vec![WINDOW_CLASS_NAME, 0]]
        );
        assert_eq!(
            calls_to(&log, API_SET_FOREGROUND_WINDOW),
            vec![vec![0x1234]]
        );
        assert!(calls_to(&log, API_CREATE_WINDOW).is_empty());
        // The log files were set up before looking for the window: settings read in order, the
        // error handler last.
        assert_eq!(
            calls_to(&log, 0x00c3_bd30),
            vec![vec![1, 0, 0, 0, 0x0086_a580]]
        );
    }

    #[test]
    fn main_stores_the_processor_count_and_raises_one_to_two() {
        let mut e = main_engine();
        e.register(API_GET_SYSTEM_INFO, |e, a| {
            // dwNumberOfProcessors is at +0x14 of SYSTEM_INFO.
            e.mem.set_u32(a[0] + 0x14, 1);
            Ret::default()
        });
        e.register(API_FIND_WINDOW, |_, _| 0x1234u32.into_ret());
        run_main_entry(&mut e);
        assert_eq!(e.mem.u32(ZERO_WORD), 2);
    }

    #[test]
    fn main_hands_over_to_the_launcher_when_it_starts() {
        let mut e = main_engine();
        e.register(0x004d_a570, |_, _| 1u32.into_ret());
        e.register(API_SHELL_EXECUTE, |_, _| 33u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(run_main_entry(&mut e), 0);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x004d_a4d0), vec![vec![1]]);
        let launcher = &calls_to(&log, API_SHELL_EXECUTE)[0];
        assert_eq!(&launcher[..2], &[0, 0]);
        assert_eq!(&launcher[3..], &[0, 0, 1]);
        assert_eq!(calls_to(&log, 0x0040_6d50).last().unwrap()[2], 0x0108_2978);
        assert_eq!(calls_to(&log, 0x0040_4ee0).len(), 1);
        assert!(calls_to(&log, API_CREATE_WINDOW).is_empty());
    }

    #[test]
    fn main_goes_on_when_the_launcher_does_not_start() {
        let mut e = main_engine();
        e.register(0x004d_a570, |_, _| 1u32.into_ret());
        e.register(API_SHELL_EXECUTE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(run_main_entry(&mut e), 0);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, API_CREATE_WINDOW).len(), 1);
    }

    /// A UTF-16 string in memory.
    fn wide(e: &mut Engine, text: &str) -> u32 {
        let units: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        let address = e.mem.alloc(units.len() as u32 * 2);
        for (i, unit) in units.iter().enumerate() {
            e.mem.set_u16(address + 2 * i as u32, *unit);
        }
        address
    }

    #[test]
    fn main_turns_cloud_off_for_the_nocloud_argument_only() {
        let mut e = main_engine();
        let arguments = [
            wide(&mut e, "FalloutNV.exe"),
            wide(&mut e, "-NoCloud"),
            wide(&mut e, "nocloud"),
            wide(&mut e, "-other"),
        ];
        let argv = e.mem.alloc(16);
        for (i, argument) in arguments.iter().enumerate() {
            e.mem.set_u32(argv + 4 * i as u32, *argument);
        }
        e.register(API_GET_COMMAND_LINE, |_, _| 0xc0de_0000u32.into_ret());
        e.register_double(API_COMMAND_LINE_TO_ARGV, move |e, a| {
            assert_eq!(a[0], 0xc0de_0000);
            e.mem.set_i32(a[1], 4);
            argv.into_ret()
        });
        for (i, unit) in "nocloud".encode_utf16().enumerate() {
            e.mem.set_u16(0x0108_29b4 + 2 * i as u32, unit);
        }
        // `__wcsicmp`: compares case-insensitively.
        e.register(0x00ec_c071, |e, a| {
            let read = |e: &Engine, mut p: u32| {
                let mut text = String::new();
                loop {
                    let unit = e.mem.u16(p);
                    if unit == 0 {
                        return text;
                    }
                    text.push(char::from_u32(unit as u32).unwrap());
                    p += 2;
                }
            };
            let left = read(e, a[0]).to_lowercase();
            let right = read(e, a[1]);
            assert_eq!(right, "nocloud");
            (left != right).into_ret()
        });
        e.register(0x006f_f580, |_, _| 0x7700u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(run_main_entry(&mut e), 0);
        let log = take_log(&mut e);
        // One match ("-NoCloud"): the second setting object is the one `006ff580` returned.
        assert_eq!(calls_to(&log, 0x006f_f580).len(), 1);
        assert_eq!(calls_to(&log, 0x006f_f880), vec![vec![0x7700, 0]]);
        assert_eq!(calls_to(&log, API_LOCAL_FREE), vec![vec![argv]]);
        // Compared: "-NoCloud" and "-other" start with '-', the text after it is compared.
        assert_eq!(calls_to(&log, 0x00ec_c071).len(), 2);
    }

    #[test]
    fn main_registers_the_window_class_and_creates_the_window_and_main_object() {
        let mut e = main_engine();
        e.register(API_LOAD_ICON, |_, a| {
            assert_eq!(a, &[INSTANCE, 0x65]);
            0x77u32.into_ret()
        });
        e.register(API_GET_STOCK_OBJECT, |_, a| {
            assert_eq!(a[0], 4);
            0x88u32.into_ret()
        });
        let class_words = Rc::new(RefCell::new(vec![]));
        let record = class_words.clone();
        e.register_double(API_REGISTER_CLASS, move |e, a| {
            for i in 0..10 {
                record.borrow_mut().push(e.mem.u32(a[0] + 4 * i));
            }
            Ret::default()
        });
        // AdjustWindowRect grows the rectangle by (4, 20) in total.
        e.register(API_ADJUST_WINDOW_RECT, |e, a| {
            assert_eq!(&a[1..], &[0x1000_0000, 0]);
            e.mem.set_u32(a[0] + 8, e.mem.u32(a[0] + 8) + 4);
            e.mem.set_u32(a[0] + 12, e.mem.u32(a[0] + 12) + 20);
            Ret::default()
        });
        e.register(API_CREATE_WINDOW, |_, _| 0x4321u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(run_main_entry(&mut e), 0);
        let log = take_log(&mut e);
        assert_eq!(
            *class_words.borrow(),
            vec![
                3,
                0x0086_a0a0,
                0,
                0,
                INSTANCE,
                0x77,
                0,
                0x88,
                0,
                WINDOW_CLASS_NAME
            ]
        );
        assert_eq!(
            calls_to(&log, API_CREATE_WINDOW),
            vec![vec![
                0,
                WINDOW_CLASS_NAME,
                WINDOW_CLASS_NAME,
                0x1000_0000,
                0,
                0,
                0x144,
                0x104,
                0,
                0,
                INSTANCE,
                0
            ]]
        );
        // `Main` is allocated with 0xA4 bytes and built with the window and the instance.
        let main_allocation = calls_to(&log, ALLOCATE_OBJECT)
            .iter()
            .position(|a| a[0] == 0xa4)
            .unwrap();
        assert_eq!(main_allocation, 1);
        let constructor = &calls_to(&log, 0x0086_c160)[0];
        assert_eq!(&constructor[1..], &[0x4321, INSTANCE]);
        // After the run the global is cleared again.
        assert_eq!(e.global::<u32>(MAIN_OBJECT), 0);
    }

    #[test]
    fn main_runs_one_frame_and_shuts_down_in_order() {
        let mut e = main_engine();
        e.register(API_CREATE_WINDOW, |_, _| 0x4321u32.into_ret());
        // The start position `Main_InitStartCell` writes is passed on by value.
        e.register(0x0087_6dd0, |e, a| {
            assert_eq!(a[0], 0);
            e.mem.set_u32(a[1], 11);
            e.mem.set_u32(a[1] + 4, 22);
            e.mem.set_u32(a[1] + 8, 33);
            Ret::default()
        });
        e.set_global(0x0126_fac4u32, 0x5150u32);
        e.register(GET_POOLED_TEXT, |_, a| (a[0] + 1).into_ret());
        e.register(0x0066_29f0, |_, a| (a[0] + 5).into_ret());
        e.register(0x0045_c670, |_, _| 0x3000u32.into_ret());
        e.set_global(0x011f_4509u32, 0u8);
        e.call_log = Some(vec![]);
        assert_eq!(run_main_entry(&mut e), 0);
        let log = take_log(&mut e);

        assert_eq!(calls_to(&log, 0x0087_7260), vec![vec![11, 22, 33]]);
        assert_eq!(
            calls_to(&log, 0x00ec_16d0),
            vec![vec![0x5150, 0x011d_e72c + 1, 0, u32::MAX, 1, 1, 0]]
        );
        assert_eq!(calls_to(&log, 0x0086_cf20)[0][1], 0x3000);
        // The animation data of the object goes to two globals.
        assert_eq!(e.global::<u32>(0x011d_59ec), 0x3005);
        assert_eq!(e.global::<u32>(0x011d_5c44), 0x3005);
        assert_eq!(e.global::<u8>(0x011f_4509), 1);
        // Large buffer is set for the master file when `008d6f30` says 0.
        assert_eq!(calls_to(&log, 0x0046_50a0), vec![vec![0, u32::MAX]]);
        // Where the Main object is used.
        let main_object = calls_to(&log, 0x0086_e650)[0][0];
        assert_ne!(main_object, 0);
        assert_eq!(calls_to(&log, 0x0086_e650).len(), 1);
        assert_eq!(calls_to(&log, 0x005d_c960), vec![vec![main_object]]);
        assert_eq!(calls_to(&log, 0x0086_c880), vec![vec![main_object]]);
        // Order of the main phases.
        let order = [
            API_CREATE_MUTEX,
            API_REGISTER_CLASS,
            0x0086_c160,
            0x0086_d500,
            0x0086_d590,
            0x0087_6dd0,
            0x0087_7260,
            0x00ec_1800,
            API_PEEK_MESSAGE,
            0x0086_e650,
            0x005d_c960,
            0x0070_3610,
            0x0086_c880,
            0x0087_83c0,
            0x0087_84d0,
        ];
        for pair in order.windows(2) {
            assert!(
                position(&log, pair[0]) < position(&log, pair[1]),
                "{:08x} before {:08x}",
                pair[0],
                pair[1]
            );
        }
        // The Main object was freed by its deleting destructor.
        assert!(calls_to(&log, FREE_OBJECT)
            .iter()
            .any(|a| a[0] == main_object));
        // The tasklet manager, the system object and the audio object were asked to shut down
        // through their vtables.
        assert!(!calls_to(&log, 0x00b0_0a00).is_empty());
        assert!(!calls_to(&log, 0x00af_f100).is_empty());
    }

    #[test]
    fn main_posts_pending_window_messages_before_each_frame() {
        let mut e = main_engine();
        let pending = Rc::new(RefCell::new(2u32));
        let counter = pending.clone();
        e.register_double(API_PEEK_MESSAGE, move |_, a| {
            assert_eq!(&a[1..], &[0, 0, 0, 1]);
            let mut left = counter.borrow_mut();
            if *left > 0 {
                *left -= 1;
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.call_log = Some(vec![]);
        run_main_entry(&mut e);
        let log = take_log(&mut e);
        let translated = calls_to(&log, API_TRANSLATE_MESSAGE);
        assert_eq!(translated.len(), 2);
        assert_eq!(translated, calls_to(&log, API_DISPATCH_MESSAGE));
        assert_eq!(calls_to(&log, API_PEEK_MESSAGE).len(), 3);
    }

    #[test]
    fn main_frame_with_a_lost_device_runs_the_off_screen_guards_and_sleeps() {
        let mut e = main_engine();
        // The device's vtable slot 0xC answers -1.
        let device = object_with_vtable(&mut e, &[(0xc, 0x0000_3100)]);
        e.register_double(0x0000_3100, |_, a| {
            // TestCooperativeLevel (0086b3a6): only the device, as the COM `this`.
            assert_eq!(a.len(), 1, "the device is passed once");
            u32::MAX.into_ret()
        });
        e.register_double(0x004d_c020, move |_, _| device.into_ret());
        let renderer = renderer_with(&mut e, 0, 1);
        e.register_double(GET_RENDERER, move |_, _| renderer.into_ret());
        e.call_log = Some(vec![]);
        run_main_entry(&mut e);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0086_e650).is_empty());
        assert!(calls_to(&log, API_SLEEP).contains(&vec![0x32]));
        // The begin guard moved the state to 2 and the end guard back to 0.
        assert_eq!(e.mem.u32(renderer + RENDERER_FRAME_STATE), 0);
    }

    /// Makes the window inactive on the first pass of the loop and active on the second.
    fn background_then_foreground(e: &mut Engine, background_calls: u32) {
        let windows = Rc::new(RefCell::new(0u32));
        let count = windows.clone();
        e.register(0x0044_ddc0, |_, _| 6u32.into_ret());
        e.register_double(API_GET_ACTIVE_WINDOW, move |_, _| {
            let mut n = count.borrow_mut();
            *n += 1;
            if *n <= background_calls {
                5u32.into_ret()
            } else {
                6u32.into_ret()
            }
        });
        let loops = Rc::new(RefCell::new(0u32));
        let count = loops.clone();
        // The loop ends after `background_calls + 1` passes.
        e.register_double(0x005d_c960, move |_, _| {
            let mut n = count.borrow_mut();
            *n += 1;
            (*n > background_calls).into_ret()
        });
    }

    #[test]
    fn main_in_the_background_suspends_audio_and_the_loading_thread_then_resumes() {
        let mut e = main_engine();
        background_then_foreground(&mut e, 1);
        e.register(API_GET_CURRENT_THREAD_ID, |_, _| 7u32.into_ret());
        e.register(GET_MAIN_THREAD_ID, |_, _| 7u32.into_ret());
        let audio = object_with_vtable(&mut e, &[(8, NOTHING)]);
        e.register_double(0x0045_3a70, move |_, _| audio.into_ret());
        e.register(0x004e_2190, |_, a| (a[0] + 1).into_ret());
        e.register(0x0049_fef0, |_, _| 0x2222u32.into_ret());
        e.register(0x0070_5910, |_, _| 0x3333u32.into_ret());
        e.call_log = Some(vec![]);
        run_main_entry(&mut e);
        let log = take_log(&mut e);

        // First pass (background): suspend everything, sleep.
        let suspend = position(&log, 0x0078_cfc0);
        let audio_down = position(&log, 0x00ad_8700);
        let resume_thread = position(&log, 0x0078_d020);
        assert!(suspend < audio_down);
        assert!(audio_down < resume_thread);
        assert_eq!(calls_to(&log, 0x00ad_8700), vec![vec![audio]]);
        assert_eq!(calls_to(&log, 0x00ad_8740), vec![vec![audio]]);
        assert_eq!(calls_to(&log, 0x0083_2ad0).len(), 2);
        assert_eq!(
            calls_to(&log, 0x00ad_7740),
            vec![vec![audio, 1], vec![audio, 1]]
        );
        assert_eq!(calls_to(&log, 0x004a_4340), vec![vec![0x2222]]);
        assert_eq!(calls_to(&log, 0x0080_26b0), vec![vec![0x3333]]);
        assert!(calls_to(&log, API_SLEEP).contains(&vec![0x32]));
        // Second pass (foreground): the frame runs once and the loading thread is resumed.
        assert_eq!(calls_to(&log, 0x0086_e650).len(), 1);
        assert_eq!(calls_to(&log, 0x0078_d020).len(), 1);
    }

    #[test]
    fn main_in_the_background_from_another_thread_leaves_the_loading_thread_alone() {
        let mut e = main_engine();
        background_then_foreground(&mut e, 1);
        e.register(API_GET_CURRENT_THREAD_ID, |_, _| 7u32.into_ret());
        e.register(GET_MAIN_THREAD_ID, |_, _| 8u32.into_ret());
        e.call_log = Some(vec![]);
        run_main_entry(&mut e);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0078_cfc0).is_empty());
        assert!(calls_to(&log, 0x0078_d020).is_empty());
        assert_eq!(calls_to(&log, 0x00ad_8700).len(), 1);
    }

    #[test]
    fn main_in_the_background_skips_the_loading_thread_when_the_object_says_so() {
        let mut e = main_engine();
        background_then_foreground(&mut e, 1);
        e.set_global(0x011d_a0c4u32, 0x8000u32);
        e.register(0x007c_fd00, |_, _| 1u32.into_ret());
        e.register(API_GET_CURRENT_THREAD_ID, |_, _| 7u32.into_ret());
        e.register(GET_MAIN_THREAD_ID, |_, _| 7u32.into_ret());
        e.call_log = Some(vec![]);
        run_main_entry(&mut e);
        assert!(calls_to(&take_log(&mut e), 0x0078_cfc0).is_empty());
    }

    #[test]
    fn main_resizes_the_window_when_it_goes_to_the_background_and_comes_back() {
        let mut e = main_engine();
        background_then_foreground(&mut e, 1);
        e.register(GET_WINDOW_SETTING, |_, _| 1u32.into_ret());
        e.register(0x004d_c1f0, |_, _| 800u32.into_ret());
        e.register(0x004d_c200, |_, _| 600u32.into_ret());
        e.register(API_CREATE_WINDOW, |_, _| 0x4321u32.into_ret());
        e.register(API_GET_WINDOW_LONG, |_, a| {
            assert_eq!(a, &[0x4321, (-0x10i32) as u32]);
            0x1234u32.into_ret()
        });
        e.register(API_ADJUST_WINDOW_RECT_EX, |e, a| {
            assert_eq!(&a[1..], &[0x1234, 0, 0]);
            // The rectangle (0, 0, 800, 600) grows by 8 on each side.
            e.mem.set_u32(a[0], (-8i32) as u32);
            e.mem.set_u32(a[0] + 4, (-8i32) as u32);
            e.mem.set_u32(a[0] + 8, 808);
            e.mem.set_u32(a[0] + 12, 608);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        run_main_entry(&mut e);
        let log = take_log(&mut e);
        // Once going to the background and once coming back.
        let moves = calls_to(&log, API_SET_WINDOW_POS);
        assert_eq!(moves.len(), 2);
        // Left and bottom are the position, (right - left, top - bottom) the size.
        let expected = vec![0x4321, 0, (-8i32) as u32, 608, 816, (-8i32 - 608) as u32, 0];
        assert_eq!(moves[0], expected);
        assert_eq!(moves[1], expected);
    }

    #[test]
    fn main_returns_held_textures_and_deletes_what_it_created() {
        let mut e = main_engine();
        // The first holder (011ded3c) holds a texture; the others do not.
        e.register(0x0055_9450, |_, a| {
            if a[0] == 0x011d_ed3c {
                0x9999u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(0x004a_0ea0, |_, _| 0x4a00u32.into_ret());
        e.call_log = Some(vec![]);
        run_main_entry(&mut e);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x00b6_da10), vec![vec![0x4a00, 0x9999]]);
        assert_eq!(calls_to(&log, 0x0066_b0d0).len(), 4);
        assert_eq!(calls_to(&log, 0x0066_b0d0)[0], vec![0x011d_ed3c, 0]);
        // The renderer's callbacks are looked up again by key (none are registered).
        assert_eq!(
            calls_to(&log, ARRAY_COUNT),
            vec![vec![KEY_ARRAY_A], vec![KEY_ARRAY_B]]
        );
        assert_eq!(e.global::<u8>(0x011a_31f4), 0);
    }

    // ----- the Main object, its queues and the start-up of the world (0086c0e0 - 0086e580) ----

    /// Callees of the functions from `0086c0e0` to `0086e580` that [`engine`] does not register
    /// (each is a no-op returning 0 until a test replaces it).
    const INIT_CALLEES: &[u32] = &[
        0x0040_3e20,
        0x0040_fc90,
        0x0041_6870,
        0x0043_8170,
        0x0043_81b0,
        0x0043_9410,
        0x0043_d410,
        0x0044_24e0,
        0x0044_2550,
        0x0044_fb20,
        0x0045_03f0,
        0x0045_05a0,
        0x0045_0b80,
        0x0045_0c50,
        0x0045_9f10,
        0x0045_a1e0,
        0x0045_a370,
        0x0045_a600,
        0x0045_bc00,
        0x0045_cec0,
        0x0046_2f40,
        0x0046_3070,
        0x0046_4f30,
        0x0046_5040,
        0x0046_dd00,
        0x0047_1d00,
        0x0048_3710,
        0x0048_39c0,
        0x0048_4e60,
        0x0049_ed90,
        0x004a_1020,
        0x004a_1040,
        0x004b_c1f0,
        0x004b_c320,
        0x004b_c3d0,
        0x004b_c3f0,
        0x004b_c400,
        0x004b_c450,
        0x004d_a670,
        0x004d_c060,
        0x004d_c540,
        0x004d_c5c0,
        0x004d_c650,
        0x004f_3200,
        0x0050_f9c0,
        0x0052_6e10,
        0x0054_6780,
        0x0054_67c0,
        0x0054_68d0,
        0x0054_95f0,
        0x0055_8310,
        0x0055_a2d0,
        0x0057_5690,
        0x0057_cf00,
        0x005b_8fc0,
        0x005f_4bb0,
        0x005f_5880,
        0x0060_aeb0,
        0x0061_cc40,
        0x0063_3c90,
        0x0066_29f0,
        0x0066_4870,
        0x0068_b3e0,
        0x0070_37c0,
        0x0070_b760,
        0x0071_2e60,
        0x007a_1480,
        0x007f_df30,
        0x0082_f830,
        0x0086_d490,
        0x0086_d580,
        0x0086_e5a0,
        0x0086_e5c0,
        0x0086_e5e0,
        0x0086_e600,
        0x0086_e620,
        0x0086_e630,
        0x0087_2430,
        0x0087_7730,
        0x0087_7a80,
        0x0087_7ac0,
        0x0087_8610,
        0x0087_b8d0,
        0x0087_b950,
        0x0089_1170,
        0x008c_26e0,
        0x008d_7dc0,
        0x0093_6aa0,
        0x0093_8180,
        0x0099_0f20,
        0x00a2_2660,
        0x00a2_2bf0,
        0x00a2_2e10,
        0x00a2_2ef0,
        0x00a5_9c60,
        0x00a5_a040,
        0x00a5_b950,
        0x00a5_ecb0,
        0x00a6_aa40,
        0x00a7_12f0,
        0x00a7_5650,
        0x00a8_7810,
        0x00a8_7900,
        0x00a8_7ae0,
        0x00a8_8020,
        0x00a8_8130,
        0x00a8_81e0,
        0x00a8_9af0,
        0x00aa_2740,
        0x00aa_53f0,
        0x00aa_5460,
        0x00ae_a020,
        0x00af_5820,
        0x00b0_0a00,
        0x00b0_0df0,
        0x00b4_f2f0,
        0x00b4_f5c0,
        0x00b5_5540,
        0x00b5_7420,
        0x00b5_e0f0,
        0x00b6_60d0,
        0x00bb_8140,
        0x00bb_8180,
        0x00bc_2980,
        0x00c4_6970,
        0x00c5_03d0,
        0x00ec_43fb,
        0x00ec_6130,
        0x00ec_623a,
        0x00fd_f0b4,
        0x0040_1000,
        0x0040_1030,
        0x0040_3df0,
        0x0040_4eb0,
        0x0040_4ee0,
        0x0040_4f00,
        0x0040_4f70,
        0x0040_6d30,
        0x0040_fbe0,
        0x0043_c4b0,
        0x0045_0410,
        0x0045_3a70,
        0x0055_9450,
        0x0066_b0d0,
        0x0000_3000,
        0x0000_3100,
        0x0000_3200,
        0x0000_3300,
        0x0000_3400,
        0x0000_3500,
        0x0000_3600,
        0x0000_3700,
        0x0000_3800,
    ];

    /// The engine for the start-up tests: [`INIT_CALLEES`] are no-ops, the allocators work and
    /// the holders behave (`00559450` reads the pointer a holder keeps, `0066b0d0` sets it).
    fn init_engine() -> Engine {
        let mut e = engine();
        for address in INIT_CALLEES.iter().copied() {
            e.register(address, |_, _| Ret::default());
        }
        e.register(ALLOCATE_SCENE_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ALLOCATE_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        // The addresses of this file's own functions that the earlier tests stub out are the
        // real translations again.
        for (address, function) in funcs() {
            e.register(address, function);
        }
        e
    }

    /// Makes every constructor in `addresses` return its `this`.
    fn constructors_return_this(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, a| a[0].into_ret());
        }
    }

    /// Registers a double at `address` that records its argument words in the returned list.
    fn recorder(e: &mut Engine, address: u32, result: u32) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = seen.clone();
        e.register_double(address, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            result.into_ret()
        });
        seen
    }

    /// A text in the test memory.
    fn text_at(e: &mut Engine, address: u32, text: &[u8]) -> u32 {
        e.mem.set_cstr(address, text);
        address
    }

    // ----- 0086c0e0 .. 0086c150 --------------------------------------------------------------

    /// The float getters: the setting object whose address they pass answers with a float
    /// derived from it.
    fn float_getter_result(function: fn(&mut Engine) -> f32, setting: u32) -> f32 {
        let mut e = init_engine();
        e.register(0x0045_0410, |_, a| ret_f32((a[0] & 0xffff) as f32));
        e.call_log = Some(vec![]);
        let value = function(&mut e);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0045_0410), vec![vec![setting]]);
        value
    }

    #[test]
    fn float_getter_0086c0e0_reads_its_setting() {
        assert_eq!(float_getter_result(fn_0086c0e0, 0x011c_7314), 0x7314 as f32);
    }

    #[test]
    fn float_getter_0086c0f0_reads_its_setting() {
        assert_eq!(float_getter_result(fn_0086c0f0, 0x011c_768c), 0x768c as f32);
    }

    #[test]
    fn float_getter_0086c100_reads_its_setting() {
        assert_eq!(float_getter_result(fn_0086c100, 0x011c_7758), 0x7758 as f32);
    }

    #[test]
    fn float_getter_0086c110_reads_its_setting() {
        assert_eq!(float_getter_result(fn_0086c110, 0x011c_75bc), 0x75bc as f32);
    }

    #[test]
    fn float_getter_0086c120_reads_its_setting() {
        assert_eq!(float_getter_result(fn_0086c120, 0x011c_7458), 0x7458 as f32);
    }

    #[test]
    fn float_getter_0086c130_reads_its_setting() {
        assert_eq!(float_getter_result(fn_0086c130, 0x011c_74a0), 0x74a0 as f32);
    }

    #[test]
    fn float_setters_store_their_argument() {
        let mut e = init_engine();
        e.call(0x0086_c140, &args![1.5f32]);
        e.call(0x0086_c150, &args![-2.25f32]);
        assert_eq!(e.global::<f32>(0x011f_f8ac), 1.5);
        assert_eq!(e.global::<f32>(0x011f_f8b0), -2.25);
    }

    // ----- Main::Main ------------------------------------------------------------------------

    /// An engine where `Main::Main` can run: constructors return `this`, the thread id is
    /// 0x1234 and the audio object exists.
    fn main_main_engine() -> (Engine, u32) {
        let mut e = init_engine();
        constructors_return_this(
            &mut e,
            &[
                0x00aa_53f0,
                0x00b6_60d0,
                0x00a7_12f0,
                0x00a2_2660,
                0x0087_7a80,
            ],
        );
        e.register(API_CREATE_SEMAPHORE, |_, _| 0x5151u32.into_ret());
        e.register(0x0040_fc90, |_, _| 0x1234u32.into_ret());
        e.register(0x007f_df30, |_, _| 0x7777u32.into_ret());
        let audio = object_with_vtable(&mut e, &[(0xc, NOTHING), (4, NOTHING)]);
        e.register_double(0x0045_3a70, move |_, _| audio.into_ret());
        let this = e.mem.alloc(0xa4);
        (e, this)
    }

    #[test]
    fn main_main_builds_the_queues_the_accumulators_and_the_fields() {
        let (mut e, this) = main_main_engine();
        for offset in 1..=7 {
            e.mem.set_u8(this + offset, 0xff);
        }
        e.mem.set_u8(this + 0x9c, 0xff);
        e.mem.set_u32(this + 0x14, 0xffff);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x0086_c160, &args![this, 0xaaaau32, 0xbbbbu32])
            .u32();
        let log = take_log(&mut e);
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this + 8), 0xaaaa);
        assert_eq!(e.mem.u32(this + 0xc), 0xbbbb);
        assert_eq!(e.mem.u32(this + 0x10), 0x1234);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        for offset in 1..=7 {
            assert_eq!(e.mem.u8(this + offset), 0);
        }
        assert_eq!(e.mem.u8(this + 0x9c), 0);
        // The two queue pairs: bases at +0x18 / +0x50, queues at +0x28 / +0x60 over them.
        assert_eq!(
            calls_to(&log, 0x0087_7a80),
            vec![
                vec![this + 0x28, this + 0x18],
                vec![this + 0x60, this + 0x50]
            ]
        );
        assert_eq!(e.mem.u32(this + 0x28 + 0x20), 0x0087_b990);
        assert_eq!(e.mem.u8(this + 0x28 + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x28 + 0x14 + 4), 0x5151);
        assert_eq!(e.mem.u32(this + 0x28 + 0x14 + 8), 100);
        assert_eq!(
            calls_to(&log, API_CREATE_SEMAPHORE),
            vec![vec![0, 0, 100, 0]; 2]
        );
        // Six holders constructed with 0.
        let holders: Vec<Vec<u32>> = [0x88u32, 0x8c, 0x90, 0x94, 0x98, 0xa0]
            .iter()
            .map(|offset| vec![this + offset, 0])
            .collect();
        assert_eq!(calls_to(&log, 0x0063_3c90), holders);
        // Five accumulators.
        let accumulators: Vec<Vec<u32>> = calls_to(&log, 0x00b6_60d0)
            .iter()
            .map(|call| call[1..].to_vec())
            .collect();
        assert_eq!(
            accumulators,
            vec![
                vec![0x63, 1, 0x2f7],
                vec![0x63, 1, 0x2f7],
                vec![0x63, 1, 0x2f7],
                vec![0x64, 1, 4],
                vec![0x63, 1, 0x2f7],
            ]
        );
        let held = |e: &Engine, offset: u32| e.mem.u32(this + offset);
        assert_eq!(
            calls_to(&log, 0x004a_1040),
            vec![
                vec![held(&e, 0x8c), 0xc],
                vec![held(&e, 0x90), 0xc],
                vec![held(&e, 0x94), 0xd]
            ]
        );
        assert_eq!(
            calls_to(&log, 0x0093_6aa0),
            vec![vec![held(&e, 0x8c), 1], vec![held(&e, 0x90), 1]]
        );
        assert!(held(&e, 0xa0) != 0);
        // One processor: the queues are not started.
        assert!(calls_to(&log, 0x0087_b8d0).is_empty());
        assert!(calls_to(&log, 0x0044_2550).is_empty());
        // Thread name, task object, scope objects.
        assert_eq!(calls_to(&log, 0x00aa_2740), vec![vec![0x1234, 0x0105_c2b0]]);
        assert_eq!(calls_to(&log, 0x00a2_2660).len(), 1);
        assert_eq!(calls_to(&log, 0x00a2_2660)[0][1], 0xbbbb);
        assert_ne!(e.global::<u32>(TASK_OBJECT), 0);
        assert_eq!(calls_to(&log, 0x00a2_2ef0), vec![vec![0x7777]]);
        let scope = calls_to(&log, 0x0040_4eb0)[0][0];
        assert_eq!(
            calls_to(&log, 0x0040_4eb0),
            vec![vec![scope, 0xd, 1, 0x0108_29c4, 0xdc6]]
        );
        assert_eq!(
            calls_to(&log, 0x0040_4f00),
            vec![vec![scope, 0xb, 1, 0x0108_29c4, 0xdd5]]
        );
        assert_eq!(calls_to(&log, 0x0040_4f70).len(), 2);
        assert_eq!(calls_to(&log, 0x0040_4ee0), vec![vec![scope]]);
        // The audio set-up ran (with the window word at +8).
        assert_eq!(calls_to(&log, 0x0050_f9c0).len(), 1);
    }

    #[test]
    fn main_main_with_several_processors_starts_both_queues() {
        let (mut e, this) = main_main_engine();
        e.mem.set_u32(ZERO_WORD, 2);
        e.call_log = Some(vec![]);
        e.call(0x0086_c160, &args![this, 1u32, 2u32]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, 0x0087_b8d0),
            vec![vec![this + 0x28, this + 0x60]]
        );
        assert_eq!(
            calls_to(&log, 0x0044_2550),
            vec![vec![this + 0x28 + 0x14], vec![this + 0x60 + 0x14]]
        );
    }

    // ----- the task queues -------------------------------------------------------------------

    #[test]
    fn thread_begin_input_releases_the_semaphore_wrapper() {
        let mut e = init_engine();
        e.call_log = Some(vec![]);
        e.call(0x0086_c600, &args![0x4000u32]);
        assert_eq!(calls_to(&take_log(&mut e), 0x0044_2550), vec![vec![0x4014]]);
    }

    #[test]
    fn task_queue_constructor_sets_up_the_base_the_semaphore_and_the_fields() {
        let mut e = init_engine();
        e.register(API_CREATE_SEMAPHORE, |_, _| 0x6161u32.into_ret());
        let queue = e.mem.alloc(0x30);
        e.mem.set_u8(queue + 0x24, 0xff);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x0086_c620, &args![queue, 0x7000u32, 0x0087_b990u32])
            .u32();
        let log = take_log(&mut e);
        assert_eq!(result, queue);
        assert_eq!(calls_to(&log, 0x0087_7a80), vec![vec![queue, 0x7000]]);
        assert_eq!(
            calls_to(&log, API_CREATE_SEMAPHORE),
            vec![vec![0, 0, 100, 0]]
        );
        assert_eq!(e.mem.u32(queue + 0x14), 0);
        assert_eq!(e.mem.u32(queue + 0x18), 0x6161);
        assert_eq!(e.mem.u32(queue + 0x1c), 100);
        assert_eq!(e.mem.u32(queue + 0x20), 0x0087_b990);
        assert_eq!(e.mem.u8(queue + 0x24), 0);
    }

    #[test]
    fn semaphore_wrapper_constructor_creates_the_semaphore() {
        let mut e = init_engine();
        e.register(API_CREATE_SEMAPHORE, |_, _| 0x9999u32.into_ret());
        let wrapper = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let result = e.call(0x0086_c6a0, &args![wrapper, 3u32, 8u32]).u32();
        let log = take_log(&mut e);
        assert_eq!(result, wrapper);
        assert_eq!(calls_to(&log, API_CREATE_SEMAPHORE), vec![vec![0, 3, 8, 0]]);
        assert_eq!(e.mem.u32(wrapper), 3);
        assert_eq!(e.mem.u32(wrapper + 4), 0x9999);
        assert_eq!(e.mem.u32(wrapper + 8), 8);
    }

    #[test]
    fn task_queue_destructor_drains_then_destroys_the_semaphore_and_the_base() {
        let mut e = init_engine();
        let queue = object_with_vtable(&mut e, &[(0x10, 0x3000)]);
        e.register(0x0000_3000, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0086_c6e0, &args![queue]);
        let log = take_log(&mut e);
        assert!(position(&log, 0x0000_3000) < position(&log, 0x0055_a2d0));
        assert!(position(&log, 0x0055_a2d0) < position(&log, 0x0087_7ac0));
        assert_eq!(calls_to(&log, 0x0055_a2d0), vec![vec![queue + 0x14]]);
        assert_eq!(calls_to(&log, 0x0087_7ac0), vec![vec![queue]]);
    }

    #[test]
    fn queue_drain_calls_slot_0x10_until_it_answers_false() {
        let mut e = init_engine();
        let queue = object_with_vtable(&mut e, &[(0x10, 0x3000)]);
        let answers = Rc::new(RefCell::new(vec![0u32, 1, 1]));
        let seen = Rc::new(RefCell::new(Vec::new()));
        let (script, record) = (answers.clone(), seen.clone());
        e.register_double(0x0000_3000, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            script.borrow_mut().pop().unwrap().into_ret()
        });
        e.call(0x0086_c750, &args![queue]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 3);
        assert!(seen.iter().all(|call| call[0] == queue && call.len() == 2));
        assert!(seen.iter().all(|call| call[1] == seen[0][1]));
    }

    #[test]
    fn task_object_is_replaced_and_the_old_one_destroyed() {
        let mut e = init_engine();
        constructors_return_this(&mut e, &[0x00a2_2660]);
        e.set_global(TASK_OBJECT, 0x4242u32);
        e.call_log = Some(vec![]);
        e.call(0x0086_c790, &args![0x77u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x00a2_2bf0), vec![vec![0x4242]]);
        assert_eq!(calls_to(&log, FREE_OBJECT), vec![vec![0x4242]]);
        assert_eq!(calls_to(&log, ALLOCATE_OBJECT), vec![vec![0x1c04]]);
        let new = e.global::<u32>(TASK_OBJECT);
        assert_ne!(new, 0);
        assert_eq!(calls_to(&log, 0x00a2_2660), vec![vec![new, 0x77]]);
    }

    #[test]
    fn task_object_first_time_and_failed_allocation() {
        let mut e = init_engine();
        constructors_return_this(&mut e, &[0x00a2_2660]);
        e.call_log = Some(vec![]);
        e.call(0x0086_c790, &args![1u32]);
        assert!(calls_to(&take_log(&mut e), 0x00a2_2bf0).is_empty());
        assert_ne!(e.global::<u32>(TASK_OBJECT), 0);
        // The allocator fails: the global ends up 0 and the constructor is not called.
        e.register(ALLOCATE_OBJECT, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0086_c790, &args![1u32]);
        let log = take_log(&mut e);
        assert_eq!(e.global::<u32>(TASK_OBJECT), 0);
        assert!(calls_to(&log, 0x00a2_2660).is_empty());
    }

    #[test]
    fn task_object_deleting_destructor_frees_only_with_bit_0() {
        let mut e = init_engine();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0086_c850, &args![0x5000u32, 0u32]).u32(), 0x5000);
        assert!(calls_to(&take_log(&mut e), FREE_OBJECT).is_empty());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0086_c850, &args![0x5000u32, 3u32]).u32(), 0x5000);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x00a2_2bf0), vec![vec![0x5000]]);
        assert_eq!(calls_to(&log, FREE_OBJECT), vec![vec![0x5000]]);
    }

    // ----- Main::~Main -----------------------------------------------------------------------

    /// Sets the nonzero marks `fn_0086c880` clears and returns a `Main` object (0xA4 bytes)
    /// whose two queues have a vtable whose slot 0x10 answers false.
    fn destroyed_main(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0xa4);
        let queue = object_with_vtable(e, &[(0x10, 0x3000)]);
        let vtable = e.mem.u32(queue);
        e.mem.set_u32(this + 0x28, vtable);
        e.mem.set_u32(this + 0x60, vtable);
        for offset in [0x88u32, 0x8c, 0x90, 0x94, 0x98, 0xa0] {
            e.mem.set_u32(this + offset, 0x9000 + offset);
        }
        this
    }

    #[test]
    fn main_destructor_destroys_the_tables_the_holders_and_the_queues() {
        let mut e = init_engine();
        let this = destroyed_main(&mut e);
        e.set_global(LOD_ROOT_NODE, 1u32);
        e.set_global(OBJECT_LOD_ROOT_NODE, 2u32);
        e.set_global(WATER_LOD_NODE, 3u32);
        e.set_global(TASK_OBJECT, 0x4242u32);
        e.mem.set_u32(SCENE_GRAPH_HOLDER, 0x1111);
        for holder in [CLEARED_HOLDER, SKY_NODE_HOLDER, SCREEN_ELEMENT_HOLDER] {
            e.mem.set_u32(holder, 0x2222);
        }
        // Objects in the tables: first and fourth of the first table, the last of the second
        // and the last of the ninth.
        let objects = Rc::new(RefCell::new(Vec::new()));
        let record = objects.clone();
        e.register_double(0x0000_3100, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let mut placed = Vec::new();
        for (table, index) in [
            (OBJECT_TABLES[0].0, 0u32),
            (OBJECT_TABLES[0].0, 3),
            (OBJECT_TABLES[1].0, 0xed),
            (OBJECT_TABLES[8].0, 0x21),
        ] {
            let object = object_with_vtable(&mut e, &[(0, 0x0000_3100)]);
            e.mem.set_u32(table + index * 4, object);
            placed.push(vec![object, 1]);
        }
        e.call_log = Some(vec![]);
        e.call(0x0086_c880, &args![this]);
        let log = take_log(&mut e);
        assert_eq!(*objects.borrow(), placed);
        assert_eq!(e.global::<u32>(LOD_ROOT_NODE), 0);
        assert_eq!(e.global::<u32>(OBJECT_LOD_ROOT_NODE), 0);
        assert_eq!(e.global::<u32>(WATER_LOD_NODE), 0);
        assert_eq!(e.global::<u32>(TASK_OBJECT), 0);
        assert_eq!(calls_to(&log, 0x00a2_2bf0), vec![vec![0x4242]]);
        // The scene graph is told about the holder's object before the holder is cleared.
        assert_eq!(calls_to(&log, 0x0048_3710), vec![vec![0x1111]]);
        for holder in [
            SCENE_GRAPH_HOLDER,
            CLEARED_HOLDER,
            SKY_NODE_HOLDER,
            SCREEN_ELEMENT_HOLDER,
        ] {
            assert_eq!(e.mem.u32(holder), 0);
        }
        // Holders of `Main`: +0x90 is destroyed but not cleared.
        for offset in [0x88u32, 0x8c, 0x94, 0x98, 0xa0] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x90), 0x9090);
        assert_eq!(
            calls_to(&log, 0x0045_cec0),
            [0xa0u32, 0x98, 0x94, 0x90, 0x8c, 0x88]
                .iter()
                .map(|offset| vec![this + offset])
                .collect::<Vec<_>>()
        );
        // One processor: nothing is waited on.
        assert!(calls_to(&log, 0x0044_24e0).is_empty());
        assert!(calls_to(&log, 0x0087_b950).is_empty());
        // The queues and their bases are destroyed back to front.
        assert_eq!(
            calls_to(&log, 0x0055_a2d0),
            vec![vec![this + 0x60 + 0x14], vec![this + 0x28 + 0x14]]
        );
        assert_eq!(
            calls_to(&log, 0x0087_7ac0),
            vec![vec![this + 0x60], vec![this + 0x28]]
        );
        assert_eq!(
            calls_to(&log, 0x00aa_5460),
            vec![vec![this + 0x50], vec![this + 0x18]]
        );
    }

    #[test]
    fn main_destructor_with_several_processors_waits_for_both_queues() {
        let mut e = init_engine();
        let this = destroyed_main(&mut e);
        e.mem.set_u32(ZERO_WORD, 2);
        e.call_log = Some(vec![]);
        e.call(0x0086_c880, &args![this]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, 0x0044_24e0),
            vec![vec![this + 0x28 + 0x14], vec![this + 0x60 + 0x14]]
        );
        assert_eq!(calls_to(&log, 0x0087_b950).len(), 1);
    }

    #[test]
    fn shutdown_object_is_destroyed_unless_it_is_missing_or_marked() {
        let mut e = init_engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = seen.clone();
        e.register_double(0x0000_3100, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        // No object.
        e.call(0x0086_cd80, &args![]);
        assert!(seen.borrow().is_empty());
        // Marked (byte at +4 non-zero).
        let object = object_with_vtable(&mut e, &[(0, 0x0000_3100)]);
        e.mem.set_u8(object + 4, 1);
        e.set_global(SHUTDOWN_OBJECT, object);
        e.call(0x0086_cd80, &args![]);
        assert!(seen.borrow().is_empty());
        // Unmarked.
        e.mem.set_u8(object + 4, 0);
        e.call(0x0086_cd80, &args![]);
        assert_eq!(*seen.borrow(), vec![vec![object, 1]]);
    }

    #[test]
    fn queue_wait_waits_on_the_semaphore_wrapper() {
        let mut e = init_engine();
        e.call_log = Some(vec![]);
        e.call(0x0086_cdd0, &args![0x6000u32]);
        assert_eq!(calls_to(&take_log(&mut e), 0x0044_24e0), vec![vec![0x6014]]);
    }

    #[test]
    fn task_object_is_cleared_with_its_destructor() {
        let mut e = init_engine();
        e.call_log = Some(vec![]);
        e.call(0x0086_cdf0, &args![]);
        assert!(calls_to(&take_log(&mut e), 0x00a2_2bf0).is_empty());
        e.set_global(TASK_OBJECT, 0x4242u32);
        e.call_log = Some(vec![]);
        e.call(0x0086_cdf0, &args![]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x00a2_2bf0), vec![vec![0x4242]]);
        assert_eq!(calls_to(&log, FREE_OBJECT), vec![vec![0x4242]]);
        assert_eq!(e.global::<u32>(TASK_OBJECT), 0);
    }

    #[test]
    fn audio_setup_registers_the_callbacks_on_the_audio_object() {
        let mut e = init_engine();
        let slot_c = recorder(&mut e, 0x0000_3100, 0);
        let slot_4 = recorder(&mut e, 0x0000_3200, 0);
        let audio = object_with_vtable(&mut e, &[(0xc, 0x0000_3100), (4, 0x0000_3200)]);
        e.register_double(0x0045_3a70, move |_, _| audio.into_ret());
        let main = e.mem.alloc(0xa4);
        e.mem.set_u32(main + 8, 0xabcd);
        e.call_log = Some(vec![]);
        e.call(0x0086_ce40, &args![main]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0050_f9c0), vec![vec![audio, 0x0082_d150]]);
        assert_eq!(calls_to(&log, 0x0070_37c0), vec![vec![audio, 0x0082_d280]]);
        assert_eq!(calls_to(&log, 0x0061_cc40), vec![vec![audio, 0x005e_3630]]);
        // `0086cf00` stored its callback at +0x2c of the audio object.
        assert_eq!(e.mem.u32(audio + 0x2c), 0x0082_d400);
        assert_eq!(calls_to(&log, 0x005f_4bb0), vec![vec![audio, 0x0083_2c40]]);
        assert_eq!(calls_to(&log, 0x008d_7dc0), vec![vec![audio, 0x0083_2c80]]);
        assert_eq!(*slot_c.borrow(), vec![vec![audio, 0x0082_d740]]);
        assert_eq!(*slot_4.borrow(), vec![vec![audio, 0xabcd]]);
        assert!(position(&log, 0x00ae_a020) < position(&log, 0x0050_f9c0));
        assert!(position(&log, 0x008d_7dc0) < position(&log, 0x0082_f830));
    }

    #[test]
    fn audio_callback_setter_stores_at_0x2c() {
        let mut e = init_engine();
        let object = e.mem.alloc(0x40);
        e.call(0x0086_cf00, &args![object, 0x1234u32]);
        assert_eq!(e.mem.u32(object + 0x2c), 0x1234);
    }

    // ----- Main::InitTES ---------------------------------------------------------------------

    /// The pieces of an engine in which `Main::InitTES` can run.
    struct TesWorld {
        player_calls: Rc<RefCell<Vec<Vec<u32>>>>,
        health_calls: Rc<RefCell<Vec<Vec<u32>>>>,
    }

    const HOLDER_TEXT: u32 = 0x0100_0400;

    /// An engine for `Main::InitTES`: the sky, the helper, the player and the file lookups
    /// exist. `health` is what the player's slot 8 (at +0xA4) answers. The second file-name
    /// holder has the text "A.esm", the fourth an empty text.
    fn tes_engine(health: u32) -> (Engine, TesWorld) {
        let mut e = init_engine();
        e.register(0x0046_dd00, |_, _| 0x5000u32.into_ret());
        e.register(0x007a_1480, |_, _| 0x6000u32.into_ret());
        e.register(0x0045_05a0, |_, a| (a[0] + 1).into_ret());
        let helper_object = object_with_vtable(&mut e, &[(0xc, 0x0000_3800)]);
        e.register(0x0000_3800, |_, _| 0x7000u32.into_ret());
        e.register_double(0x0045_bc00, move |_, _| helper_object.into_ret());
        constructors_return_this(&mut e, &[0x0044_fb20]);
        e.set_global(OBJECT_LOD_ROOT_NODE, 0x8200u32);
        e.set_global(WATER_LOD_NODE, 0x8100u32);
        e.set_global(DATA_HANDLER, 0x9000u32);
        let player_calls = recorder(&mut e, 0x0000_3300, 0);
        let health_calls = recorder(&mut e, 0x0000_3500, health);
        let player_table = e.mem.alloc(0x400);
        for slot in [0x128u32, 0x2a8, 0x88] {
            e.mem.set_u32(player_table + slot, 0x0000_3300);
        }
        let embedded_table = e.mem.alloc(0x40);
        e.mem.set_u32(embedded_table + 8, 0x0000_3500);
        e.register_double(0x0093_8180, move |e, a| {
            e.mem.set_u32(a[0], player_table);
            e.mem.set_u32(a[0] + 0xa4, embedded_table);
            a[0].into_ret()
        });
        // Texts of the file-name holders.
        let first = text_at(&mut e, HOLDER_TEXT, b"A.esm");
        let empty = text_at(&mut e, HOLDER_TEXT + 0x20, b"");
        e.register_double(0x0040_3df0, move |_, a| {
            if a[0] == FILE_NAME_HOLDERS[1] {
                first.into_ret()
            } else if a[0] == FILE_NAME_HOLDERS[3] {
                empty.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(0x00ec_6130, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        e.register(0x0046_4f30, |_, a| a[0].into_ret());
        e.register(0x0046_2f40, |_, a| (a[1] + 0x100).into_ret());
        e.register(0x0048_39c0, |_, _| 0x6100u32.into_ret());
        e.register(0x00ec_43fb, |_, _| 0x6200u32.into_ret());
        e.register(0x0041_6870, |_, a| a[0].into_ret());
        (
            e,
            TesWorld {
                player_calls,
                health_calls,
            },
        )
    }

    #[test]
    fn init_tes_builds_the_tes_object_activates_files_and_initialises_the_player() {
        let (mut e, world) = tes_engine(100);
        e.set_global(PLUGINS_LIST_NAME, 0x4242u32);
        e.set_global(0x0101_8bfc, 0x4500_0000u32);
        let main = e.mem.alloc(0xa4);
        e.call_log = Some(vec![]);
        e.call(0x0086_cf20, &args![main, 0x3333u32]);
        let log = take_log(&mut e);

        // Sky, helper and the TES object.
        assert_eq!(e.global::<u32>(SKY_OBJECT), 0x5000);
        assert_eq!(calls_to(&log, 0x0045_05a0), vec![vec![0x5000]]);
        assert_eq!(calls_to(&log, 0x00a8_9af0), vec![vec![0x5001, 0x6000]]);
        assert_eq!(calls_to(&log, 0x0045_bc00), vec![vec![0x6000, 0]]);
        let tes = e.global::<u32>(TES_OBJECT);
        assert_eq!(
            calls_to(&log, 0x0044_fb20),
            vec![vec![tes, 0x0107_24e8, 0x8200, 0x7000, 0x5000, 0x8100]]
        );
        assert_eq!(calls_to(&log, 0x0045_0c50), vec![vec![tes]]);
        assert_eq!(calls_to(&log, 0x0045_9f10), vec![vec![tes]]);
        assert_eq!(calls_to(&log, 0x0045_a370), vec![vec![tes]]);
        assert_eq!(calls_to(&log, 0x0045_a1e0), vec![vec![tes]]);
        // The setting byte (zero) keeps `0045a600` away.
        assert!(calls_to(&log, 0x0045_a600).is_empty());
        // One file was found; the list of plugins did not add one, but the fallback is skipped.
        assert_eq!(calls_to(&log, 0x0046_2f40), vec![vec![0x9000, HOLDER_TEXT]]);
        assert_eq!(
            calls_to(&log, 0x0047_1d00),
            vec![vec![HOLDER_TEXT + 0x100, 1]]
        );
        assert_eq!(
            calls_to(&log, 0x0087_2430),
            vec![vec![main, 0x0120_2e98, 0x4242]]
        );
        // The player.
        let player = e.global::<u32>(PLAYER_OBJECT);
        assert_ne!(player, 0);
        let player_calls = world.player_calls.borrow();
        assert_eq!(player_calls.len(), 3);
        assert_eq!(player_calls[0], vec![player, 0x14, 1]);
        assert_eq!(calls_to(&log, 0x0057_5690), vec![vec![player, 0x6200]]);
        assert_eq!(
            calls_to(&log, 0x00ec_43fb),
            vec![vec![0x6100, 0, 0x0118_3028, 0x0118_3a1c, 0]]
        );
        let buffer = calls_to(&log, 0x0041_6870)[0][0];
        assert_eq!(
            calls_to(&log, 0x0041_6870),
            vec![vec![buffer, 0x4500_0000, 0x4500_0000, 0]]
        );
        assert_eq!(player_calls[1], vec![player, buffer]);
        assert_eq!(player_calls[2], vec![player]);
        assert_eq!(calls_to(&log, 0x0086_d490), vec![vec![player, 0x011f_426c]]);
        assert_eq!(calls_to(&log, 0x008c_26e0), vec![vec![player, 0]]);
        // Health is non-zero: no error.
        assert_eq!(
            *world.health_calls.borrow(),
            vec![vec![player + 0xa4, 0x10]]
        );
        assert!(calls_to(&log, ERROR_LOG)
            .iter()
            .all(|call| call[0] != 0x0108_2b30));
        // Node update and scopes.
        let scope = calls_to(&log, 0x0040_4eb0)[0][0];
        assert_eq!(
            calls_to(&log, 0x0040_4eb0),
            vec![vec![scope, 0xa, 1, 0x0108_29c4, 0xe4f]]
        );
        let scope_lines: Vec<u32> = calls_to(&log, 0x0040_4f00)
            .iter()
            .map(|call| call[4])
            .collect();
        assert_eq!(scope_lines, vec![0xe51, 0xe85, 0xecb, 0xf00]);
        assert_eq!(calls_to(&log, 0x0040_4ee0), vec![vec![scope]]);
        assert_eq!(calls_to(&log, 0x00a5_a040), vec![vec![0x3333]]);
        let value = calls_to(&log, 0x0043_d410)[0][0];
        assert_eq!(calls_to(&log, 0x0043_d410), vec![vec![value, 0, 0, 0]]);
        assert_eq!(calls_to(&log, 0x00a5_9c60), vec![vec![0x3333, value]]);
        // Messages, in order.
        let messages: Vec<u32> = calls_to(&log, ERROR_LOG)
            .iter()
            .map(|call| call[0])
            .collect();
        assert_eq!(
            messages,
            vec![
                0x0108_2be8,
                0x0108_2bcc,
                0x0108_2bac,
                0x0108_2b8c,
                0x0108_2b74,
                0x0108_2b14
            ]
        );
        assert_eq!(calls_to(&log, 0x0046_3070), vec![vec![0x9000, 0]]);
        assert_eq!(calls_to(&log, 0x0046_5040), vec![vec![0x9000]]);
    }

    #[test]
    fn init_tes_without_a_helper_object_and_without_files_activates_fallout_esm() {
        let (mut e, _world) = tes_engine(100);
        e.register(0x007a_1480, |_, _| 0u32.into_ret());
        // No holder has a text.
        e.register(0x0040_3df0, |_, _| 0u32.into_ret());
        let main = e.mem.alloc(0xa4);
        e.call_log = Some(vec![]);
        e.call(0x0086_cf20, &args![main, 0u32]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0045_05a0).is_empty());
        assert!(calls_to(&log, 0x0045_bc00).is_empty());
        let tes = e.global::<u32>(TES_OBJECT);
        assert_eq!(
            calls_to(&log, 0x0044_fb20),
            vec![vec![tes, 0x0107_24e8, 0x8200, 0, 0x5000, 0x8100]]
        );
        assert_eq!(calls_to(&log, 0x0046_2f40), vec![vec![0x9000, 0x0108_2ba0]]);
        assert_eq!(
            calls_to(&log, 0x0047_1d00),
            vec![vec![0x0108_2ba0 + 0x100, 1]]
        );
    }

    #[test]
    fn init_tes_skips_fallout_esm_when_the_plugin_list_loads() {
        let (mut e, _world) = tes_engine(100);
        e.register(0x0040_3df0, |_, _| 0u32.into_ret());
        e.register(0x0087_2430, |_, _| 1u32.into_ret());
        let main = e.mem.alloc(0xa4);
        e.call_log = Some(vec![]);
        e.call(0x0086_cf20, &args![main, 0u32]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0046_2f40).is_empty());
        assert!(calls_to(&log, 0x0047_1d00).is_empty());
    }

    #[test]
    fn init_tes_loads_extra_data_when_the_setting_is_on_and_logs_a_zero_health() {
        let (mut e, world) = tes_engine(0);
        e.mem.set_u8(ZERO_BYTE, 1);
        e.register(0x0048_4e60, |_, a| (a[0] + 1).into_ret());
        e.register(0x0089_1170, |_, a| (a[0] + 1).into_ret());
        let main = e.mem.alloc(0xa4);
        e.call_log = Some(vec![]);
        e.call(0x0086_cf20, &args![main, 0u32]);
        let log = take_log(&mut e);
        let tes = e.global::<u32>(TES_OBJECT);
        assert_eq!(calls_to(&log, 0x0045_a600), vec![vec![tes]]);
        assert_eq!(world.health_calls.borrow().len(), 1);
        assert_eq!(calls_to(&log, 0x0048_4e60), vec![vec![0x6200, 0]]);
        assert_eq!(calls_to(&log, 0x0089_1170), vec![vec![0x6201]]);
        assert!(calls_to(&log, ERROR_LOG).contains(&vec![0x0108_2b30, 0x6202]));
    }

    #[test]
    fn init_tes_sets_the_timer_step_from_the_setting_when_it_is_in_range() {
        let (mut e, _world) = tes_engine(100);
        e.set_global(MILLISECONDS_PER_SECOND, 1000.0f64);
        e.register(0x0045_03f0, |_, _| 100u32.into_ret());
        e.mem.set_u32(ZERO_WORD, 500);
        let main = e.mem.alloc(0xa4);
        e.call_log = Some(vec![]);
        e.call(0x0086_cf20, &args![main, 0u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0045_03f0).len(), 2);
        assert_eq!(e.mem.f32(TIMER + 4), 2.0);
        // Out of range (200 or more): left alone.
        e.mem.set_f32(TIMER + 4, 0.0);
        e.register(0x0045_03f0, |_, _| 200u32.into_ret());
        e.call(0x0086_cf20, &args![main, 0u32]);
        assert_eq!(e.mem.f32(TIMER + 4), 0.0);
        // Zero: left alone after one check.
        e.register(0x0045_03f0, |_, _| 0u32.into_ret());
        e.call(0x0086_cf20, &args![main, 0u32]);
        assert_eq!(e.mem.f32(TIMER + 4), 0.0);
    }

    // ----- the small helpers of the start-up -------------------------------------------------

    #[test]
    fn object_lod_root_getter_and_constant_getter() {
        let mut e = init_engine();
        e.set_global(OBJECT_LOD_ROOT_NODE, 0x1357u32);
        assert_eq!(e.call(0x0086_d470, &args![]).u32(), 0x1357);
        assert_eq!(e.call(0x0086_d480, &args![]).u32(), 0x0120_2e98);
    }

    #[test]
    fn timer_step_is_a_thousandth_divided_by_the_count() {
        let mut e = init_engine();
        e.set_global(MILLISECONDS_PER_SECOND, 1000.0f64);
        let object = e.mem.alloc(0x10);
        e.call(0x0086_d4c0, &args![object, 4u32]);
        assert_eq!(e.mem.f32(object + 4), 250.0);
        e.call(0x0086_d4c0, &args![object, 0u32]);
        assert_eq!(e.mem.f32(object + 4), 0.0);
        // The count is unsigned.
        e.call(0x0086_d4c0, &args![object, 0x8000_0000u32]);
        assert_eq!(e.mem.f32(object + 4), (1000.0f64 / 2147483648.0) as f32);
    }

    #[test]
    fn renderer_check_does_nothing_when_the_renderer_starts() {
        let mut e = init_engine();
        e.register(0x004d_a670, |_, _| 1u32.into_ret());
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + 8, 0x11);
        e.mem.set_u32(object + 0xc, 0x22);
        e.call_log = Some(vec![]);
        e.call(0x0086_d500, &args![object]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x004d_a670), vec![vec![0x11, 0x22]]);
        assert!(calls_to(&log, API_MESSAGE_BOX).is_empty());
        assert!(calls_to(&log, API_EXIT_PROCESS).is_empty());
    }

    #[test]
    fn renderer_check_reports_the_failure_and_ends_the_process() {
        let mut e = init_engine();
        e.register(0x004d_a670, |_, _| 0u32.into_ret());
        e.register(0x0086_d580, |_, _| 0x1234u32.into_ret());
        e.register(0x00ec_623a, |e, a| {
            e.mem.set_cstr(a[0], b"formatted");
            Ret::default()
        });
        let shown = Rc::new(RefCell::new(Vec::new()));
        let record = shown.clone();
        e.register_double(API_MESSAGE_BOX, move |e, a| {
            record.borrow_mut().push((a.to_vec(), e.mem.cstr(a[1])));
            Ret::default()
        });
        let object = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0086_d500, &args![object]);
        let log = take_log(&mut e);
        let format = calls_to(&log, 0x00ec_623a);
        assert_eq!(format.len(), 1);
        assert_eq!(format[0][1..], [0x0108_2c04, 0x1234]);
        let shown = shown.borrow();
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0].0, vec![0, format[0][0], 0x0108_2bfc, 0]);
        assert_eq!(shown[0].1, b"formatted".to_vec());
        assert!(position(&log, API_MESSAGE_BOX) < position(&log, API_EXIT_PROCESS));
        assert_eq!(calls_to(&log, API_EXIT_PROCESS), vec![vec![0]]);
    }

    // ----- Main::InitSceneGraph --------------------------------------------------------------

    /// What a scene-graph test needs to look at.
    struct SceneWorld {
        graph: u32,
        root: u32,
        lod_root: u32,
        nodes: Rc<RefCell<Vec<u32>>>,
        attached: Rc<RefCell<Vec<Vec<u32>>>>,
        children: Rc<RefCell<Vec<Vec<u32>>>>,
    }

    /// An engine for `Main::InitSceneGraph`: the constructors return nodes that share one
    /// vtable (slot 0xDC attaches, slot 0xF8 sets a numbered child, both recorded).
    fn scene_engine() -> (Engine, SceneWorld, u32) {
        let mut e = init_engine();
        let attached = recorder(&mut e, 0x0000_3600, 0);
        let children = recorder(&mut e, 0x0000_3700, 0);
        let table = e.mem.alloc(0x400);
        e.mem.set_u32(table + 0xdc, 0x0000_3600);
        e.mem.set_u32(table + 0xf8, 0x0000_3700);
        let node = move |e: &mut Engine| {
            let node = e.mem.alloc(0x40);
            e.mem.set_u32(node, table);
            node
        };
        let graph = node(&mut e);
        let root = node(&mut e);
        let lod_root = node(&mut e);
        e.register_double(0x0087_8610, move |_, _| graph.into_ret());
        e.register_double(0x00b5_e0f0, move |_, _| root.into_ret());
        e.register_double(0x00bc_2980, move |_, _| lod_root.into_ret());
        let nodes = Rc::new(RefCell::new(Vec::new()));
        let made = nodes.clone();
        e.register_double(0x00a5_ecb0, move |e, _| {
            let created = node(e);
            made.borrow_mut().push(created);
            created.into_ret()
        });
        constructors_return_this(
            &mut e,
            &[
                0x00bb_8180,
                0x00c4_6970,
                0x00c5_03d0,
                0x00a8_81e0,
                0x00a8_7810,
                0x00a7_5650,
                0x00a6_aa40,
            ],
        );
        e.register(0x0043_8170, |_, a| a[0].into_ret());
        e.register(0x0066_29f0, |_, a| (a[0] + 1).into_ret());
        e.register(0x0045_0b80, |_, _| 0x5555u32.into_ret());
        e.register(0x004b_c3f0, |_, _| 0x6666u32.into_ret());
        e.register(0x0055_8310, |_, a| (a[0] + 2).into_ret());
        e.register(0x00b4_f5c0, |_, _| 0x7171u32.into_ret());
        e.register(0x0086_e600, |_, _| 0x81u32.into_ret());
        // The LOD distance setting: the float the object `00403e20` points at.
        e.register(0x0040_3e20, |_, _| 0x0100_0300u32.into_ret());
        e.mem.set_u32(0x0100_0300, 0x4049_0fdb);
        e.register(0x0054_95f0, |_, _| 0x4444u32.into_ret());
        let main = e.mem.alloc(0xa4);
        for offset in [0x88u32, 0x8c, 0x90, 0x94, 0x98] {
            e.mem.set_u32(main + offset, 0xa000 + offset);
        }
        (
            e,
            SceneWorld {
                graph,
                root,
                lod_root,
                nodes,
                attached,
                children,
            },
            main,
        )
    }

    #[test]
    fn scene_graph_is_built_with_its_nodes_in_order() {
        let (mut e, world, main) = scene_engine();
        e.call_log = Some(vec![]);
        e.call(0x0086_d590, &args![main]);
        let log = take_log(&mut e);

        // The graph is named "World"; the root is stored in the holders of `Main`.
        let world_calls = calls_to(&log, 0x0087_8610);
        assert_eq!(world_calls.len(), 1);
        assert_eq!(world_calls[0][1..], [0x0108_2c9c, 0, 0]);
        assert_eq!(e.mem.u32(SCENE_GRAPH_HOLDER), world.graph);
        assert_eq!(calls_to(&log, ALLOCATE_SCENE_OBJECT)[0], vec![0xc0]);
        assert_eq!(calls_to(&log, ALLOCATE_SCENE_OBJECT)[1], vec![0x200]);
        assert_eq!(e.mem.u32(0x011f_9508), 0x6666);
        assert_eq!(
            calls_to(&log, 0x0040_6d30),
            vec![vec![0x011f_9308, 0x104, 0x0108_2c8c]]
        );
        assert_eq!(
            calls_to(&log, 0x0071_2e60),
            vec![vec![world.graph + 1, 0x011a_d840]]
        );
        assert_eq!(calls_to(&log, 0x0070_b760), vec![vec![world.root, 0]]);
        assert_eq!(calls_to(&log, 0x00b4_f2f0), vec![vec![0, world.root]]);
        assert_eq!(
            calls_to(&log, 0x004a_1020),
            vec![
                vec![0xa088, world.root],
                vec![0xa08c, world.root],
                vec![0xa090, world.root],
                vec![0xa094, world.root],
                vec![0xa098, 0x5555],
            ]
        );
        assert_eq!(calls_to(&log, 0x0045_0b80), vec![vec![1]]);
        assert_eq!(e.mem.u32(0x011f_95d8), world.graph + 1);

        // Settings: everything off.
        assert_eq!(calls_to(&log, 0x004b_c3d0), vec![vec![0x10]]);
        assert_eq!(e.global::<u32>(0x011a_d82c), 0);
        assert_eq!(e.global::<u32>(0x011f_91ec), 0x0056_4360);
        assert_eq!(e.global::<u32>(0x011f_d870), 0);
        assert_eq!(e.global::<u32>(0x011a_d808), 0x4049_0fdb);
        assert_eq!(calls_to(&log, 0x0040_3e20), vec![vec![0x011d_edc8]]);
        assert_eq!(e.global::<u8>(0x011f_91de), 0);
        assert_eq!(e.global::<u8>(0x011f_91dd), 0);

        // Attachments (slot 0xDC) and numbered children (slot 0xF8).
        let nodes = world.nodes.borrow().clone();
        // Weather, LandLOD, DistantRefLOD, WaterLOD, LOD Trees, ObjectLODRoot.
        assert_eq!(nodes.len(), 6);
        let (weather, land, distant, water, trees, object_lod) =
            (nodes[0], nodes[1], nodes[2], nodes[3], nodes[4], nodes[5]);
        let sky = e.mem.u32(SKY_NODE_HOLDER);
        let particles = e.mem.u32(PARTICLE_SYSTEMS_HOLDER);
        assert_eq!(
            *world.attached.borrow(),
            vec![
                vec![world.graph, world.root, 1],
                vec![world.lod_root, land, 1],
                vec![world.lod_root, distant, 1],
                vec![world.lod_root, water, 1],
                vec![distant, trees, 1],
            ]
        );
        assert_eq!(
            *world.children.borrow(),
            vec![
                vec![world.root, 0, sky],
                vec![world.root, 1, weather],
                vec![world.root, 2, world.lod_root],
                vec![world.root, 3, object_lod],
                vec![world.root, 5, particles],
            ]
        );
        assert_eq!(e.global::<u32>(LOD_ROOT_NODE), world.lod_root);
        assert_eq!(e.global::<u32>(WATER_LOD_NODE), water);
        assert_eq!(e.global::<u32>(OBJECT_LOD_ROOT_NODE), object_lod);
        assert_eq!(e.mem.u32(WEATHER_NODE_HOLDER), weather);
        assert_eq!(calls_to(&log, 0x0086_e620), vec![vec![trees]]);
        assert_eq!(calls_to(&log, 0x0086_e630), vec![vec![trees]]);

        // Names, in creation order, each given to its node.
        let texts: Vec<u32> = calls_to(&log, 0x0043_8170)
            .iter()
            .map(|call| call[1])
            .collect();
        assert_eq!(
            texts,
            vec![
                0x0108_2c88,
                0x0101_fe78,
                0x0108_2c80,
                0x0108_2c78,
                0x0108_2c68,
                0x0108_2c5c,
                0x0108_2c50,
                0x0108_2c40,
                0x0108_2c28,
            ]
        );
        let named: Vec<u32> = calls_to(&log, 0x00a5_b950)
            .iter()
            .map(|call| call[0])
            .collect();
        assert_eq!(
            named,
            vec![
                sky,
                weather,
                world.lod_root,
                land,
                distant,
                water,
                trees,
                object_lod,
                particles
            ]
        );
        assert_eq!(calls_to(&log, 0x0043_81b0).len(), 9);

        // Fog.
        let fog = e.mem.u32(FOG_HOLDER);
        assert_eq!(calls_to(&log, 0x0049_ed90), vec![vec![fog, 1]]);
        assert_eq!(calls_to(&log, 0x00bb_8140), vec![vec![fog, 0x3f80_0000]]);
        assert_eq!(calls_to(&log, 0x00b5_5540), vec![vec![0, fog]]);
        // The sky and the weather nodes are marked.
        assert_eq!(calls_to(&log, 0x0054_68d0), vec![vec![sky, 1]]);
        assert!(calls_to(&log, 0x0054_67c0).contains(&vec![sky, 1]));
        let marked: Vec<Vec<u32>> = calls_to(&log, 0x0054_6780);
        assert_eq!(
            marked,
            vec![
                vec![weather, 1],
                vec![land, 1],
                vec![distant, 1],
                vec![water, 1],
                vec![trees, 1],
                vec![object_lod, 1]
            ]
        );
        // The root is placed, updated, handed to the renderer.
        assert_eq!(
            calls_to(&log, 0x004b_c1f0),
            vec![vec![world.graph + 2, 0, 0, 0]]
        );
        assert_eq!(calls_to(&log, 0x004d_c540), vec![vec![0, 0x7171]]);
        assert_eq!(e.global::<u8>(SCENE_FLAG_BYTE), 0x81);
        assert_eq!(calls_to(&log, 0x0057_cf00).len(), 1);
        // Scope.
        let scope = calls_to(&log, 0x0040_4eb0)[0][0];
        assert_eq!(
            calls_to(&log, 0x0040_4eb0),
            vec![vec![scope, 0x19, 1, 0x0108_29c4, 0xf4c]]
        );
        assert_eq!(calls_to(&log, 0x0040_4ee0), vec![vec![scope]]);
    }

    #[test]
    fn scene_graph_builds_the_screen_quad_when_the_holder_is_empty() {
        let (mut e, _world, main) = scene_engine();
        e.register(0x00a8_7ae0, |_, _| 77u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0086_d590, &args![main]);
        let log = take_log(&mut e);
        let quad = e.mem.u32(SCREEN_ELEMENT_HOLDER);
        assert_ne!(quad, 0);
        // The data (0x70 bytes) and the object (0xC4 bytes) are allocated in that nesting.
        let allocations: Vec<u32> = calls_to(&log, ALLOCATE_SCENE_OBJECT)
            .iter()
            .map(|call| call[0])
            .collect();
        let at_outer = allocations.iter().position(|size| *size == 0xc4).unwrap();
        assert_eq!(allocations[at_outer + 1], 0x70);
        let data = calls_to(&log, 0x00a8_81e0);
        assert_eq!(data.len(), 1);
        assert_eq!(data[0][1..], [0, 0, 1, 1, 1, 4, 1, 2, 1]);
        assert_eq!(calls_to(&log, 0x00a8_7810), vec![vec![quad, data[0][0]]]);
        // The wrappers reach the data object `005495f0` returns.
        assert_eq!(calls_to(&log, 0x00a8_7ae0), vec![vec![0x4444, 4, 0, 0]]);
        assert_eq!(
            calls_to(&log, 0x00a8_8020),
            vec![vec![0x4444, 0, 0, 0, 0x3f80_0000, 0x3f80_0000]]
        );
        assert_eq!(calls_to(&log, 0x00a8_7900), vec![vec![0x4444]]);
        assert_eq!(
            calls_to(&log, 0x00a8_8130),
            vec![vec![0x4444, 0, 0, 0, 0, 0x3f80_0000, 0x3f80_0000]]
        );
        // The material and the texturing property are attached to the quad.
        let material = calls_to(&log, 0x004b_c450);
        assert_eq!(material.len(), 1);
        assert_eq!(material[0][1], 0x011a_9b7c);
        let texturing = calls_to(&log, 0x005b_8fc0)[0][0];
        assert_eq!(
            calls_to(&log, 0x0043_9410),
            vec![vec![quad, material[0][0]], vec![quad, texturing]]
        );
        assert_eq!(calls_to(&log, 0x004b_c320), vec![vec![0, 0]]);
        assert_eq!(calls_to(&log, 0x0060_aeb0), vec![vec![texturing, 0]]);
        assert_eq!(calls_to(&log, 0x004f_3200), vec![vec![texturing, 0]]);
        // The quad is updated like the root: two updates in all.
        assert_eq!(calls_to(&log, 0x00a5_a040).len(), 2);
        assert_eq!(calls_to(&log, 0x00a5_a040)[1], vec![quad]);
        assert_eq!(calls_to(&log, 0x0043_d410).len(), 2);
        assert_eq!(calls_to(&log, 0x00a5_9c60).len(), 2);
        assert_eq!(calls_to(&log, 0x00a5_9c60)[1][0], quad);
    }

    #[test]
    fn scene_graph_keeps_an_existing_screen_quad() {
        let (mut e, _world, main) = scene_engine();
        e.mem.set_u32(SCREEN_ELEMENT_HOLDER, 0x2468);
        e.call_log = Some(vec![]);
        e.call(0x0086_d590, &args![main]);
        let log = take_log(&mut e);
        assert_eq!(e.mem.u32(SCREEN_ELEMENT_HOLDER), 0x2468);
        assert!(calls_to(&log, 0x00a8_81e0).is_empty());
        assert!(calls_to(&log, 0x00a8_7ae0).is_empty());
        assert_eq!(calls_to(&log, 0x00a5_a040).len(), 1);
        assert_eq!(calls_to(&log, 0x0057_cf00).len(), 1);
    }

    #[test]
    fn scene_graph_folds_the_settings_into_the_flag_words() {
        let (mut e, _world, main) = scene_engine();
        // Every byte setting is 1 (the three read through the setting pointer and the three
        // read by address).
        e.mem.set_u8(ZERO_BYTE, 1);
        e.register(0x0086_e5a0, |_, _| 1u32.into_ret());
        e.register(0x0086_e5c0, |_, _| 1u32.into_ret());
        e.register(0x0086_e5e0, |_, _| 7u32.into_ret());
        e.register(0x004b_c400, |_, _| 1u32.into_ret());
        e.register(0x004d_c060, |_, a| {
            assert_eq!(a, &[0]);
            2u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0086_d590, &args![main]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x004b_c3d0), vec![vec![0x3f]]);
        assert_eq!(e.global::<u32>(0x011a_d82c), 10);
    }

    #[test]
    fn scene_graph_leaves_out_the_multi_display_bit_with_one_display_or_none() {
        let (mut e, _world, main) = scene_engine();
        e.register(0x004b_c400, |_, _| 1u32.into_ret());
        e.register(0x004d_c060, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0086_d590, &args![main]);
        assert_eq!(calls_to(&take_log(&mut e), 0x004b_c3d0), vec![vec![0x10]]);
        // Settings that are 2 do not count as 1; the display count is not even asked when
        // the first check fails.
        let (mut e, _world, main) = scene_engine();
        e.mem.set_u8(ZERO_BYTE, 2);
        e.register(0x004d_c060, |_, _| panic!("not asked"));
        e.call_log = Some(vec![]);
        e.call(0x0086_d590, &args![main]);
        assert_eq!(calls_to(&take_log(&mut e), 0x004b_c3d0), vec![vec![0x10]]);
    }

    // ----- the screen-element wrappers and the global setters --------------------------------

    #[test]
    fn insert_wrapper_forwards_to_the_data_object() {
        let mut e = init_engine();
        e.register(0x0054_95f0, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x4444u32.into_ret()
        });
        e.register(0x00a8_7ae0, |_, _| 9u32.into_ret());
        e.call_log = Some(vec![]);
        let result = e.call(0x0086_e3f0, &args![0x1000u32, 4u16, 0xffffu16, 0x2000u32]);
        assert_eq!(result.u32(), 9);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x00a8_7ae0),
            vec![vec![0x4444, 4, 0xffff, 0x2000]]
        );
    }

    #[test]
    fn rectangle_wrapper_forwards_to_the_data_object() {
        let mut e = init_engine();
        e.register(0x0054_95f0, |_, _| 0x4444u32.into_ret());
        e.register(0x00a8_8020, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        let result = e.call(
            0x0086_e420,
            &args![0x1000u32, 3u32, 0.5f32, 1.5f32, 2.5f32, -3.5f32],
        );
        assert_eq!(result.u32(), 1);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x00a8_8020),
            vec![vec![
                0x4444,
                3,
                0.5f32.to_bits(),
                1.5f32.to_bits(),
                2.5f32.to_bits(),
                (-3.5f32).to_bits()
            ]]
        );
    }

    #[test]
    fn textures_wrapper_forwards_to_the_data_object() {
        let mut e = init_engine();
        e.register(0x0054_95f0, |_, _| 0x4444u32.into_ret());
        e.register(0x00a8_8130, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        let result = e.call(
            0x0086_e460,
            &args![
                0x1000u32,
                3u32,
                0x1_0002u32,
                0.25f32,
                0.5f32,
                0.75f32,
                1.0f32
            ],
        );
        assert_eq!(result.u32(), 1);
        // The 16-bit argument is cut to 16 bits.
        assert_eq!(
            calls_to(&take_log(&mut e), 0x00a8_8130),
            vec![vec![
                0x4444,
                3,
                2,
                0.25f32.to_bits(),
                0.5f32.to_bits(),
                0.75f32.to_bits(),
                1.0f32.to_bits()
            ]]
        );
    }

    #[test]
    fn update_bound_wrapper_forwards_to_the_data_object() {
        let mut e = init_engine();
        e.register(0x0054_95f0, |_, _| 0x4444u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0086_e4b0, &args![0x1000u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0054_95f0), vec![vec![0x1000]]);
        assert_eq!(calls_to(&log, 0x00a8_7900), vec![vec![0x4444]]);
    }

    #[test]
    fn global_setters_store_their_arguments() {
        let mut e = init_engine();
        e.call(0x0086_e4d0, &args![0xdead_beefu32]);
        assert_eq!(e.global::<u32>(0x011f_91ec), 0xdead_beef);
        e.call(0x0086_e500, &args![7u8]);
        assert_eq!(e.global::<u8>(0x011f_91de), 7);
        e.call(0x0086_e510, &args![9u8]);
        assert_eq!(e.global::<u8>(0x011f_91dd), 9);
        // Each setter touches only its own global.
        assert_eq!(e.global::<u8>(0x011f_91df), 0);
    }

    #[test]
    fn path_setter_copies_into_the_buffer() {
        let mut e = init_engine();
        e.call_log = Some(vec![]);
        e.call(0x0086_e4e0, &args![0x1234u32]);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x0040_6d30),
            vec![vec![0x011f_9308, 0x104, 0x1234]]
        );
    }

    #[test]
    fn holder_setter_assigns_the_holder() {
        let mut e = init_engine();
        e.call(0x0086_e520, &args![0x7777u32]);
        assert_eq!(e.mem.u32(0x011f_9508), 0x7777);
    }

    #[test]
    fn byte_setting_getters_read_their_settings() {
        let mut e = init_engine();
        e.register(0x0040_8d60, |_, a| {
            // The setting object's address picks one of three bytes.
            let index = match a[0] {
                0x011c_7154 => 0,
                0x011c_7544 => 1,
                0x011c_712c => 2,
                other => panic!("unexpected setting {other:08x}"),
            };
            (ZERO_BYTE + index).into_ret()
        });
        e.mem.set_u8(ZERO_BYTE, 11);
        e.mem.set_u8(ZERO_BYTE + 1, 22);
        e.mem.set_u8(ZERO_BYTE + 2, 33);
        assert_eq!(e.call(0x0086_e540, &args![]).u8(), 11);
        assert_eq!(e.call(0x0086_e560, &args![]).u8(), 22);
        assert_eq!(e.call(0x0086_e580, &args![]).u8(), 33);
    }

    /// Every callee of the idle-update functions that lives outside this file.
    const IDLE_CALLEES: [u32; 228] = [
        0x0040_3df0,
        0x0040_3e20,
        0x0040_6d00,
        0x0040_fba0,
        0x0040_fbf0,
        0x0042_4940,
        0x0042_5fd0,
        0x0043_6aa0,
        0x0043_b5d0,
        0x0043_d410,
        0x0044_6ef0,
        0x0044_ddc0,
        0x0045_0b80,
        0x0045_0f90,
        0x0045_0fb0,
        0x0045_0ff0,
        0x0045_1530,
        0x0045_2580,
        0x0045_34f0,
        0x0045_3550,
        0x0045_37b0,
        0x0045_37c0,
        0x0045_3a70,
        0x0045_5490,
        0x0045_5640,
        0x0045_56d0,
        0x0045_6610,
        0x0045_7d70,
        0x0045_a190,
        0x0045_b070,
        0x0045_bc80,
        0x0045_c670,
        0x0045_ce80,
        0x0045_cec0,
        0x0046_1bc0,
        0x0047_c850,
        0x0047_d0b0,
        0x0047_e040,
        0x0048_3710,
        0x0049_fef0,
        0x0049_fff0,
        0x004a_0370,
        0x004a_03c0,
        0x004a_0ea0,
        0x004a_4040,
        0x004a_4080,
        0x004b_7210,
        0x004b_af10,
        0x004d_c310,
        0x004d_c360,
        0x004d_cef0,
        0x004d_e600,
        0x004e_0110,
        0x004e_1610,
        0x004e_3270,
        0x004e_9530,
        0x004e_a970,
        0x004f_d3e0,
        0x0052_4c90,
        0x0052_5420,
        0x0054_7590,
        0x0054_8210,
        0x0054_8230,
        0x0054_ae30,
        0x0054_ddd0,
        0x0055_0200,
        0x0055_2570,
        0x0055_2ba0,
        0x0055_85e0,
        0x0057_5d70,
        0x0057_ab70,
        0x0058_6390,
        0x005a_38e0,
        0x005a_9d60,
        0x005a_e270,
        0x005b_e5c0,
        0x005d_4a40,
        0x005f_36f0,
        0x0063_1620,
        0x0063_3c90,
        0x0066_29f0,
        0x0066_52e0,
        0x0068_15c0,
        0x0068_3a60,
        0x006a_61b0,
        0x006c_0720,
        0x006c_3640,
        0x006d_a7c0,
        0x006e_bc50,
        0x006f_f580,
        0x006f_f860,
        0x0070_10e0,
        0x0070_11d0,
        0x0070_13e0,
        0x0070_1400,
        0x0070_1450,
        0x0070_14a0,
        0x0070_23c0,
        0x0070_2450,
        0x0070_2680,
        0x0070_27e0,
        0x0070_2810,
        0x0070_2840,
        0x0070_28b0,
        0x0070_3490,
        0x0070_34c0,
        0x0070_3d50,
        0x0070_3e10,
        0x0070_50d0,
        0x0070_56f0,
        0x0070_58e0,
        0x0070_5990,
        0x0070_5a00,
        0x0070_5ea0,
        0x0070_79b0,
        0x0070_7ad0,
        0x0070_9b40,
        0x0070_9bc0,
        0x0070_ed10,
        0x0070_ed20,
        0x0070_edf0,
        0x0071_0ab0,
        0x0071_2e60,
        0x0071_3d80,
        0x0071_4a00,
        0x0071_7e50,
        0x0072_6070,
        0x0076_24d0,
        0x0077_1520,
        0x0078_cfc0,
        0x007d_1360,
        0x007f_a990,
        0x007f_df30,
        0x0082_d7c0,
        0x0082_fb70,
        0x0083_2ad0,
        0x0085_1d90,
        0x0086_7a40,
        0x0086_8850,
        0x0086_8d10,
        0x0087_05c0,
        0x0087_05d0,
        0x0087_0610,
        0x0087_06b0,
        0x0087_1a50,
        0x0087_1dc0,
        0x0087_2940,
        0x0087_4b90,
        0x0087_7430,
        0x0087_8080,
        0x0087_82b0,
        0x0087_8860,
        0x0087_a6b0,
        0x0087_a6d0,
        0x0087_a790,
        0x008a_8150,
        0x008c_78c0,
        0x008c_7990,
        0x008c_80e0,
        0x008c_94e0,
        0x008c_a070,
        0x008c_a300,
        0x008d_0600,
        0x008d_6f30,
        0x008d_8520,
        0x0093_1850,
        0x0093_bea0,
        0x0094_66d0,
        0x0094_81d0,
        0x0095_0090,
        0x0095_0bb0,
        0x0096_4260,
        0x0096_8670,
        0x0096_c240,
        0x0096_c710,
        0x0096_c860,
        0x0096_c970,
        0x0096_cfa0,
        0x0096_d810,
        0x0096_e9b0,
        0x0096_eb40,
        0x0097_4420,
        0x0097_46c0,
        0x0097_77a0,
        0x0097_81d0,
        0x0097_8550,
        0x0099_1500,
        0x0099_1600,
        0x0099_1dc0,
        0x009c_8cc0,
        0x00a2_3010,
        0x00a2_4180,
        0x00a2_4660,
        0x00a2_53d0,
        0x00a2_57c0,
        0x00a2_9680,
        0x00a6_1cd0,
        0x00a8_1a20,
        0x00a8_1a80,
        0x00aa_4e40,
        0x00aa_7290,
        0x00ad_7740,
        0x00af_2640,
        0x00b4_f5c0,
        0x00b5_4000,
        0x00b5_5a10,
        0x00b5_ac90,
        0x00b6_0040,
        0x00b6_55b0,
        0x00b6_b730,
        0x00b6_da10,
        0x00b6_dd00,
        0x00b8_aea0,
        0x00b8_b440,
        0x00b8_b500,
        0x00b8_b9a0,
        0x00b8_ccb0,
        0x00b8_d020,
        0x00ba_2f30,
        0x00ba_2fa0,
        0x00ba_30f0,
        0x00ba_3130,
        0x00c3_dbf0,
        0x00c4_58f0,
        0x00c5_0610,
        0x00c5_1f20,
        0x00c5_2020,
        0x00c6_6760,
    ];

    /// The idle update's functions (`0086e5a0` .. `0086ff70`).
    mod idle {
        use super::*;

        /// A function address registered as a double that answers 1.
        const YES: u32 = 0x0000_2000;
        /// An object whose vtable (large enough for the idle update's slots) has the given
        /// (byte offset, function) slots.
        fn object_with_vtable(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
            let table = e.mem.alloc(0x500);
            for (offset, function) in slots {
                e.mem.set_u32(table + offset, *function);
            }
            let object = e.mem.alloc(0x700);
            e.mem.set_u32(object, table);
            object
        }

        fn idle_engine() -> Engine {
            let mut e = engine();
            let extra = [
                API_GET_ASYNC_KEY_STATE,
                API_CREATE_DIRECTORY,
                IS_IN_MENU_MODE,
                GET_RENDERER,
                ERROR_LOG,
            ];
            for address in IDLE_CALLEES.iter().copied().chain(extra) {
                e.register(address, |_, _| Ret::default());
            }
            e.register(YES, |_, _| 1u32.into_ret());
            e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
            e.register(POINTER_SET, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                Ret::default()
            });
            let system = object_with_vtable(&mut e, &[(0xc, NOTHING)]);
            e.register_double(0x00af_2640, move |_, _| system.into_ret());
            let slots = [
                (8, NOTHING),
                (0x1d0, NOTHING),
                (0x1f4, NOTHING),
                (0x22c, NOTHING),
                (0x2f8, NOTHING),
            ];
            let player = object_with_vtable(&mut e, &slots);
            let table = e.mem.u32(player);
            e.mem.set_u32(player + 0xa4, table);
            e.set_global(PLAYER_OBJECT, player);
            let scene = object_with_vtable(&mut e, &[(0x104, NOTHING)]);
            e.mem.set_u32(SCENE_GRAPH_HOLDER, scene);
            // The Pipboy, the player's position and the cell's flag byte are objects with zeroed
            // fields, and the setting objects are their own float cells.
            let pipboy = e.mem.alloc(0x200);
            e.register_double(0x0070_5990, move |_, _| pipboy.into_ret());
            let position = e.mem.alloc(12);
            e.register_double(0x0043_6aa0, move |_, _| position.into_ret());
            let cell_flags = e.mem.alloc(4);
            e.register_double(0x0054_8210, move |_, _| cell_flags.into_ret());
            e.register(0x0040_3e20, |_, a| a[0].into_ret());
            for (address, function) in funcs() {
                e.register(address, function);
            }
            e
        }

        fn main_object(e: &mut Engine) -> Ptr {
            Ptr::new(e.mem.alloc(0xb4))
        }

        fn set_processors(e: &mut Engine, count: u32) {
            e.mem.set_u32(ZERO_WORD, count);
        }

        fn floats(e: &mut Engine, address: u32, value: f32) {
            e.mem.set_f32(address, value);
        }

        // ----- small functions ---------------------------------------------------------------

        fn check_byte_getter(function: u32, setting: u32) {
            let mut e = engine();
            e.register_double(SETTING_BYTE_PTR, move |_, a| {
                assert_eq!(a[0], setting);
                ZERO_BYTE.into_ret()
            });
            e.mem.set_u8(ZERO_BYTE, 0x5a);
            assert_eq!(e.call(function, &args![]).u8(), 0x5a);
        }

        #[test]
        fn byte_setting_0086e5a0_reads_011c7180() {
            check_byte_getter(0x0086_e5a0, 0x011c_7180);
        }

        #[test]
        fn byte_setting_0086e5c0_reads_011c7368() {
            check_byte_getter(0x0086_e5c0, 0x011c_7368);
        }

        #[test]
        fn byte_setting_0086e5e0_reads_011c7380() {
            check_byte_getter(0x0086_e5e0, 0x011c_7380);
        }

        #[test]
        fn byte_setting_0086e600_reads_011c7550() {
            check_byte_getter(0x0086_e600, 0x011c_7550);
        }

        #[test]
        fn word_setter_0086e620_stores_its_argument() {
            let mut e = engine();
            e.call(0x0086_e620, &args![0xcafe_f00du32]);
            assert_eq!(e.global::<u32>(0x011f_9644), 0xcafe_f00d);
        }

        #[test]
        fn holder_setter_0086e630_assigns_the_holder_at_011d86bc() {
            let mut e = engine();
            let seen = recorder(&mut e, POINTER_SET, 0);
            e.call(0x0086_e630, &args![0x4321u32]);
            assert_eq!(*seen.borrow(), vec![vec![0x011d_86bc, 0x4321]]);
        }

        #[test]
        fn clearing_setter_0086ef30_zeroes_its_byte() {
            let mut e = engine();
            e.set_global(0x0126_77a2, 9u8);
            e.call(0x0086_ef30, &args![]);
            assert_eq!(e.global::<u8>(0x0126_77a2), 0);
        }

        #[test]
        fn counter_0086ef40_adds_one() {
            let mut e = engine();
            e.set_global(0x011f_9138, 41u32);
            e.call(0x0086_ef40, &args![]);
            assert_eq!(e.global::<u32>(0x011f_9138), 42);
        }

        #[test]
        fn getter_0086ef60_returns_its_byte() {
            let mut e = engine();
            e.set_global(0x011f_4461, 3u8);
            assert_eq!(e.call(0x0086_ef60, &args![]).u8(), 3);
        }

        #[test]
        fn getter_0086ef70_reads_offset_0x52() {
            let mut e = engine();
            let object = e.mem.alloc(0x80);
            e.mem.set_u8(object + 0x52, 7);
            assert_eq!(e.call(0x0086_ef70, &args![object]).u8(), 7);
        }

        #[test]
        fn clearing_setter_0086ef90_zeroes_its_float() {
            let mut e = engine();
            e.set_global(0x0126_7c1c, 5.5f32);
            e.call(0x0086_ef90, &args![]);
            assert_eq!(e.global::<f32>(0x0126_7c1c), 0.0);
        }

        #[test]
        fn object_test_0086efa0_needs_the_object_and_its_yes() {
            let mut e = engine();
            // No object: 0 without asking.
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0086_efa0, &args![]).u8(), 0);
            assert!(calls_to(&take_log(&mut e), 0x004a_4080).is_empty());
            // An object that says no, then yes.
            e.set_global(0x011d_aac0, 0x5000u32);
            let seen = recorder(&mut e, 0x004a_4080, 0);
            assert_eq!(e.call(0x0086_efa0, &args![]).u8(), 0);
            e.register(0x004a_4080, |_, _| 1u32.into_ret());
            assert_eq!(e.call(0x0086_efa0, &args![]).u8(), 1);
            assert_eq!(*seen.borrow(), vec![vec![0x5000, 0x1_0000]]);
        }

        #[test]
        fn float_setters_store_their_arguments() {
            let mut e = engine();
            e.call(0x0086_f160, &args![1.5f32]);
            e.call(0x0086_f170, &args![2.5f32]);
            e.call(0x0086_f180, &args![3.5f32]);
            assert_eq!(e.global::<f32>(0x011a_d804), 1.5);
            assert_eq!(e.global::<f32>(0x011a_d800), 2.5);
            assert_eq!(e.global::<f32>(0x011a_d7fc), 3.5);
            e.call(0x0086_f830, &args![4.5f32]);
            assert_eq!(e.global::<f32>(0x011c_40ec), 4.5);
        }

        #[test]
        fn getter_0086f840_reads_offset_0x16c() {
            let mut e = engine();
            let object = e.mem.alloc(0x200);
            e.mem.set_u8(object + 0x16c, 1);
            assert_eq!(e.call(0x0086_f840, &args![object]).u8(), 1);
        }

        #[test]
        fn getter_0086f860_reads_the_object_or_answers_zero() {
            let mut e = engine();
            assert_eq!(e.call(0x0086_f860, &args![]).u8(), 0);
            let object = e.mem.alloc(0x200);
            e.mem.set_u8(object + 0x131, 4);
            e.set_global(0x011d_9510, object);
            assert_eq!(e.call(0x0086_f860, &args![]).u8(), 4);
        }

        #[test]
        fn getter_0086ff40_returns_its_byte() {
            let mut e = engine();
            e.set_global(0x0118_abb0, 1u8);
            assert_eq!(e.call(0x0086_ff40, &args![]).u8(), 1);
        }

        #[test]
        fn adder_0086ff50_adds_seconds_to_the_float() {
            let mut e = engine();
            e.set_global(0x011c_96fc, 1.25f32);
            e.call(0x0086_ff50, &args![0.5f32]);
            assert_eq!(e.global::<f32>(0x011c_96fc), 1.75);
        }

        #[test]
        fn player_byte_setters_store_where_the_game_does() {
            let mut e = engine();
            e.call(0x0086_fba0, &args![1u8]);
            assert_eq!(e.global::<u8>(0x011d_cfa6), 1);
            e.call(0x0086_fbb0, &args![0x77u32]);
            assert_eq!(e.global::<u32>(0x011d_cfb8), 0x77);
            let player = e.mem.alloc(0x600);
            e.call(0x0086_fbc0, &args![player, 1u8]);
            assert_eq!(e.mem.u8(player + 0x5f9), 1);
        }

        #[test]
        fn sequence_0086fd70_runs_the_three_interface_calls() {
            let mut e = idle_engine();
            e.call_log = Some(vec![]);
            e.call(0x0086_fd70, &args![0u32]);
            let log = take_log(&mut e);
            let addresses: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
            assert_eq!(
                addresses,
                vec![0x0086_fd70, 0x0070_27e0, 0x0070_2810, 0x0070_2840]
            );
        }

        #[test]
        fn audio_update_0086f640_passes_the_audio_object() {
            let mut e = idle_engine();
            e.register(0x0045_3a70, |_, _| 0x1234u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f640, &args![0u32]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x00ad_7740), vec![vec![0x1234, 1]]);
            assert_eq!(calls_to(&log, 0x0083_2ad0), vec![vec![0]]);
            assert!(position(&log, 0x0083_2ad0) < position(&log, 0x0082_fb70));
            assert!(position(&log, 0x0082_fb70) < position(&log, 0x0082_d7c0));
        }

        #[test]
        fn reset_0086f670_clears_bytes_and_calls_the_cell_object() {
            let mut e = idle_engine();
            for address in [0x0126_82f8u32, 0x011d_f678, 0x011d_ea2a] {
                e.set_global(address, 1u8);
            }
            e.set_global(TES_OBJECT, 0x6000u32);
            e.call_log = Some(vec![]);
            e.call(0x0086_f670, &args![0u32]);
            let log = take_log(&mut e);
            for address in [0x0126_82f8u32, 0x011d_f678, 0x011d_ea2a] {
                assert_eq!(e.global::<u8>(address), 0);
            }
            assert_eq!(calls_to(&log, 0x0045_5490), vec![vec![0x6000]]);
            assert!(position(&log, 0x005a_38e0) < position(&log, 0x0045_5490));
        }

        // ----- lights and counters -----------------------------------------------------------

        fn light_engine() -> Engine {
            let mut e = idle_engine();
            // Setting objects are their own float cells.
            e.register(0x0040_3e20, |_, a| a[0].into_ret());
            for (address, value) in [
                (0x011d_ec24u32, 1.0f32),
                (0x011d_eb10, 2.0),
                (0x011d_ed8c, 3.0),
                (0x011d_ec7c, 4.0),
                (0x011d_ee54, 5.0),
                (0x011d_ed98, 6.0),
                (0x011d_ea80, 7.0),
                (0x011d_eddc, 8.0),
                (0x011d_ecc8, 9.0),
            ] {
                floats(&mut e, address, value);
            }
            e.set_global(PLAYER_OBJECT, 0x7000u32);
            e
        }

        fn lights(e: &Engine) -> [f32; 3] {
            [
                e.global::<f32>(0x011a_d804),
                e.global::<f32>(0x011a_d800),
                e.global::<f32>(0x011a_d7fc),
            ]
        }

        #[test]
        fn lights_0086efe0_first_triple_when_00525420_says_so() {
            let mut e = light_engine();
            e.register(0x0052_5420, |_, _| 1u32.into_ret());
            e.call(0x0086_efe0, &args![0u32]);
            assert_eq!(lights(&e), [1.0, 2.0, 3.0]);
        }

        #[test]
        fn lights_0086efe0_second_triple_for_a_qualifying_world_space() {
            let mut e = light_engine();
            e.register(0x004f_d3e0, |_, _| 0x8000u32.into_ret());
            let seen = recorder(&mut e, 0x0058_6390, 1);
            e.call(0x0086_efe0, &args![0u32]);
            assert_eq!(lights(&e), [4.0, 5.0, 6.0]);
            assert_eq!(*seen.borrow(), vec![vec![0x8000, 1]]);
        }

        #[test]
        fn lights_0086efe0_third_triple_for_the_players_process() {
            let mut e = light_engine();
            e.register(0x008d_6f30, |_, _| 0x9000u32.into_ret());
            let seen = recorder(&mut e, 0x0042_5fd0, 1);
            e.call(0x0086_efe0, &args![0u32]);
            assert_eq!(lights(&e), [7.0, 8.0, 9.0]);
            assert_eq!(*seen.borrow(), vec![vec![0x9000]]);
        }

        #[test]
        fn lights_0086efe0_default_is_one_three_times() {
            let mut e = light_engine();
            // A world space that the second test rejects, and a process that is not wanted.
            e.register(0x004f_d3e0, |_, _| 0x8000u32.into_ret());
            e.register(0x008d_6f30, |_, _| 0x9000u32.into_ret());
            e.call(0x0086_efe0, &args![0u32]);
            assert_eq!(lights(&e), [1.0, 1.0, 1.0]);
            // Without a player the process is not even asked for.
            e.set_global(PLAYER_OBJECT, 0u32);
            e.set_global(0x011a_d804, 0.0f32);
            e.call(0x0086_efe0, &args![0u32]);
            assert_eq!(lights(&e), [1.0, 1.0, 1.0]);
        }

        // ----- 0086f190 ----------------------------------------------------------------------

        #[test]
        fn menu_actors_0086f190_rearms_without_a_menu() {
            let mut e = idle_engine();
            e.call(0x0086_f190, &args![0u32]);
            assert_eq!(e.global::<u8>(0x011d_eefd), 1);
        }

        #[test]
        fn menu_actors_0086f190_visits_listed_actors_once() {
            let mut e = idle_engine();
            e.set_global(IN_MENU_FLAG, 1u8);
            e.set_global(0x011d_eefd, 1u8);
            e.set_global(PLAYER_OBJECT, 0xaa0u32);
            e.register(0x0071_7e50, |_, _| 0x4000u32.into_ret());
            e.register(0x005b_e5c0, |_, _| 3u32.into_ret());
            let wanted = object_with_vtable(&mut e, &[(0x1d0, YES)]);
            let unwanted = object_with_vtable(&mut e, &[(0x1d0, NOTHING)]);
            e.register_double(0x0096_8670, move |_, a| match a[1] {
                0 => wanted.into_ret(),
                1 => 0u32.into_ret(),
                _ => unwanted.into_ret(),
            });
            e.register(0x008d_8520, |_, a| (a[0] + 1).into_ret());
            e.register(0x0093_1850, |_, _| 0u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f190, &args![0u32]);
            let log = take_log(&mut e);
            assert_eq!(
                calls_to(&log, 0x0048_3710),
                vec![vec![wanted + 1], vec![0xaa1]]
            );
            assert_eq!(calls_to(&log, 0x005b_e5c0).len(), 4);
            assert_eq!(e.global::<u8>(0x011d_eefd), 0);
        }

        #[test]
        fn menu_actors_0086f190_skips_actors_with_a_process_type() {
            let mut e = idle_engine();
            e.set_global(IN_MENU_FLAG, 1u8);
            e.set_global(0x011d_eefd, 1u8);
            e.set_global(PLAYER_OBJECT, 0xaa0u32);
            e.register(0x005b_e5c0, |_, _| 1u32.into_ret());
            let actor = object_with_vtable(&mut e, &[(0x1d0, YES)]);
            e.register_double(0x0096_8670, move |_, _| actor.into_ret());
            e.register(0x0093_1850, |_, _| 5u32.into_ret());
            e.register(0x008d_8520, |_, a| a[0].into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f190, &args![0u32]);
            // Only the player is visited.
            assert_eq!(calls_to(&take_log(&mut e), 0x0048_3710), vec![vec![0xaa0]]);
        }

        // ----- 0086f260 / 0086f330 -----------------------------------------------------------

        fn timer_engine() -> Engine {
            let mut e = idle_engine();
            e.register(0x0040_3e20, |_, a| a[0].into_ret());
            e.register(TIMER_GET_SECONDS, |_, _| ret_f32(2.0));
            floats(&mut e, 0x011d_1320, 7.0);
            floats(&mut e, 0x011c_5724, 0.5);
            e
        }

        #[test]
        fn timer_update_0086f260_runs_the_physics_step_and_scales_the_seconds() {
            let mut e = timer_engine();
            e.register(0x0044_ddc0, |_, _| 4u32.into_ret());
            e.register(0x0052_5420, |_, _| 1u32.into_ret());
            e.mem.set_u8(ZERO_BYTE, 6);
            e.call_log = Some(vec![]);
            e.call(0x0086_f260, &args![0u32]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x008a_8150), vec![vec![TIMER, 6]]);
            assert_eq!(calls_to(&log, 0x00aa_4e40).len(), 1);
            assert_eq!(calls_to(&log, 0x0077_1520), vec![vec![TIMER]]);
            assert_eq!(
                calls_to(&log, 0x00c6_6760),
                vec![vec![2.0f32.to_bits(), 1, 1]]
            );
            assert_eq!(e.global::<f32>(0x011a_fe60), 7.0);
            assert_eq!(e.global::<f32>(0x011d_ea30), 1.0);
        }

        #[test]
        fn timer_update_0086f260_in_a_menu_skips_the_clock_and_zeroes_the_scale() {
            let mut e = timer_engine();
            e.set_global(IN_MENU_FLAG, 1u8);
            e.set_global(0x011d_ea30, 5.0f32);
            e.call_log = Some(vec![]);
            e.call(0x0086_f260, &args![0u32]);
            let log = take_log(&mut e);
            assert!(calls_to(&log, 0x00aa_4e40).is_empty());
            assert_eq!(
                calls_to(&log, 0x00c6_6760),
                vec![vec![2.0f32.to_bits(), 0, 0]]
            );
            assert_eq!(e.global::<f32>(0x011d_ea30), 0.0);
        }

        #[test]
        fn timer_values_0086f330_ask_the_function_for_each_index() {
            let mut e = idle_engine();
            e.register(0x0300_0000, |_, a| ret_f32(a[0] as f32 * 1.5));
            e.set_global(0x011f_91f0, 0x0300_0000u32);
            e.call(0x0086_f330, &args![]);
            for index in 0..4u32 {
                assert_eq!(e.global::<f32>(0x011f_940c + index * 4), index as f32 * 1.5);
            }
            // Without a function the values are 0.0.
            e.set_global(0x011f_91f0, 0u32);
            e.call(0x0086_f330, &args![]);
            for index in 0..4u32 {
                assert_eq!(e.global::<f32>(0x011f_940c + index * 4), 0.0);
            }
        }

        // ----- 0086f390 ----------------------------------------------------------------------

        fn poll_engine() -> (Engine, Ptr) {
            let mut e = idle_engine();
            e.register(0x007f_df30, |_, _| 0x5100u32.into_ret());
            e.register(TIMER_GET_SECONDS, |_, _| ret_f32(0.25));
            e.register(0x0070_13e0, |_, _| ret_f32(0.125));
            let main = main_object(&mut e);
            (e, main)
        }

        #[test]
        fn poll_controls_0086f390_polls_and_clears_the_user_actions() {
            let (mut e, main) = poll_engine();
            // The Pipboy test passes (mode is not 4).
            e.register(0x0070_9bc0, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f390, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x00a2_3010), vec![vec![0x5100]]);
            assert_eq!(
                calls_to(&log, 0x00a2_57c0),
                vec![vec![0x5100, 0.125f32.to_bits()]]
            );
            assert_eq!(calls_to(&log, 0x00a2_53d0), vec![vec![0x5100]]);
            assert_eq!(calls_to(&log, 0x0070_2680), vec![vec![0x3e9, 0]]);
        }

        #[test]
        fn poll_controls_0086f390_does_not_clear_while_the_camera_flies() {
            let (mut e, main) = poll_engine();
            e.register(0x0070_9bc0, |_, _| 1u32.into_ret());
            e.mem.set_u8(main.addr() + MAIN_FLY_CAMERA, 1);
            e.call_log = Some(vec![]);
            e.call(0x0086_f390, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x00a2_3010).len(), 1);
            assert!(calls_to(&log, 0x00a2_53d0).is_empty());
        }

        #[test]
        fn poll_controls_0086f390_stops_at_each_blocking_test() {
            // The player's method +0x22c says yes.
            let (mut e, main) = poll_engine();
            e.register(0x0070_9bc0, |_, _| 1u32.into_ret());
            let player = object_with_vtable(&mut e, &[(0x22c, YES)]);
            e.set_global(PLAYER_OBJECT, player);
            e.call_log = Some(vec![]);
            e.call(0x0086_f390, &args![main]);
            assert!(calls_to(&take_log(&mut e), 0x00a2_53d0).is_empty());
            // Mode 4 with the start-menu test saying yes needs the Pipboy test, which says no.
            let (mut e, main) = poll_engine();
            e.register(0x0044_ddc0, |_, _| 4u32.into_ret());
            e.register(0x007d_1360, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f390, &args![main]);
            assert!(calls_to(&take_log(&mut e), 0x00a2_53d0).is_empty());
            // Mode 4 with the start-menu test saying no goes on.
            let (mut e, main) = poll_engine();
            e.register(0x0044_ddc0, |_, _| 4u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f390, &args![main]);
            assert_eq!(calls_to(&take_log(&mut e), 0x00a2_53d0).len(), 1);
            // The menu 0x3e9 being visible, and `004a4040` saying yes, each stop it.
            let (mut e, main) = poll_engine();
            e.register(0x0070_9bc0, |_, _| 1u32.into_ret());
            e.register(0x0070_2680, |_, a| (a[0] == 0x3e9).into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f390, &args![main]);
            assert!(calls_to(&take_log(&mut e), 0x00a2_53d0).is_empty());
            let (mut e, main) = poll_engine();
            e.register(0x0070_9bc0, |_, _| 1u32.into_ret());
            e.register(0x004a_4040, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f390, &args![main]);
            assert!(calls_to(&take_log(&mut e), 0x00a2_53d0).is_empty());
        }

        // ----- 0086f450 ----------------------------------------------------------------------

        #[test]
        fn menu_background_0086f450_renders_when_nothing_else_covers_it() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.set_global(IN_MENU_FLAG, 1u8);
            e.set_global(SCENE_FLAG_BYTE, 1u8);
            e.call_log = Some(vec![]);
            e.call(0x0086_f450, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x0087_1dc0), vec![vec![main.addr()]]);
            assert!(calls_to(&log, 0x0087_7430).is_empty());
            // The menu 0x420 being visible (second word 0xb) blocks it.
            e.register(0x0070_2680, |_, a| {
                (a[0] == 0x420 && a[1] == 0xb).into_ret()
            });
            e.call_log = Some(vec![]);
            e.call(0x0086_f450, &args![main]);
            assert!(calls_to(&take_log(&mut e), 0x0087_1dc0).is_empty());
        }

        #[test]
        fn menu_background_0086f450_kills_the_texture_when_no_menu_is_up() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.set_global(SCENE_FLAG_29, 1u8);
            e.call_log = Some(vec![]);
            e.call(0x0086_f450, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x0087_7430), vec![vec![main.addr()]]);
            assert_eq!(calls_to(&log, 0x0070_14a0), vec![vec![0, 1]]);
            // All eight covering menus are asked, in order.
            let asked: Vec<u32> = calls_to(&log, 0x0070_2680).iter().map(|a| a[0]).collect();
            assert_eq!(
                asked,
                vec![0x41e, 0x3f6, 0x424, 0x432, 0x438, 0x439, 0x43a, 0x43b]
            );
        }

        #[test]
        fn menu_background_0086f450_without_the_flag_does_nothing() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.call_log = Some(vec![]);
            e.call(0x0086_f450, &args![main]);
            assert!(calls_to(&take_log(&mut e), 0x0087_7430).is_empty());
        }

        #[test]
        fn menu_background_0086f450_in_a_menu_needs_a_top_menu_to_kill_the_texture() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.set_global(SCENE_FLAG_29, 1u8);
            e.set_global(IN_MENU_FLAG, 1u8);
            // In a menu with nothing special: the background stays.
            e.call_log = Some(vec![]);
            e.call(0x0086_f450, &args![main]);
            assert!(calls_to(&take_log(&mut e), 0x0087_7430).is_empty());
            // The top menu being 0x420 lets it go.
            e.register(0x0070_2450, |_, a| (a[0] == 0x420).into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f450, &args![main]);
            assert_eq!(calls_to(&take_log(&mut e), 0x0087_7430).len(), 1);
            // So does the console being visible while no rendered menu is up.
            e.register(0x0070_2450, |_, _| Ret::default());
            e.register(0x0070_3d50, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f450, &args![main]);
            assert_eq!(calls_to(&take_log(&mut e), 0x0087_7430).len(), 1);
            // A fader that is active blocks it.
            e.register(0x0070_14a0, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f450, &args![main]);
            assert!(calls_to(&take_log(&mut e), 0x0087_7430).is_empty());
        }

        // ----- 0086f6a0 / 0086f890 -----------------------------------------------------------

        #[test]
        fn ai_tasks_0086f6a0_updates_lists_by_processor_count() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(TIMER_GET_SECONDS, |_, _| ret_f32(0.5));
            let pipboy = e.mem.alloc(0x200);
            e.register_double(0x0070_5990, move |_, _| pipboy.into_ret());
            e.mem.set_u32(ZERO_WORD, 1);
            e.call_log = Some(vec![]);
            e.call(0x0086_f6a0, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x0096_c240), vec![vec![PROCESS_LISTS, 0]]);
            assert_eq!(calls_to(&log, 0x0096_c860).len(), 1);
            assert!(calls_to(&log, 0x0096_c970).is_empty());
            assert_eq!(calls_to(&log, 0x0096_c710).len(), 1);
            assert_eq!(
                calls_to(&log, 0x0097_81d0),
                vec![vec![PROCESS_LISTS, 0.5f32.to_bits()]]
            );
            assert!(calls_to(&log, 0x0054_ae30).is_empty());
            assert_eq!(calls_to(&log, 0x0099_1600), vec![vec![0, 0]]);
        }

        #[test]
        fn ai_tasks_0086f6a0_with_several_processors_runs_the_task_managers() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            set_processors(&mut e, 2);
            floats(&mut e, 0x011c_3c08, 9.5);
            e.register(0x0045_37b0, |_, _| 0x7700u32.into_ret());
            e.set_global(IN_MENU_FLAG, 1u8);
            e.call_log = Some(vec![]);
            e.call(0x0086_f6a0, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x0096_c970).len(), 1);
            assert!(calls_to(&log, 0x0096_c860).is_empty());
            assert_eq!(calls_to(&log, 0x0054_ae30).len(), 1);
            assert_eq!(calls_to(&log, 0x0087_a6d0), vec![vec![0x7700]]);
            assert_eq!(calls_to(&log, 0x0087_a790), vec![vec![0x7700]]);
            assert_eq!(calls_to(&log, 0x0087_a6b0), vec![vec![0x7700]]);
            assert_eq!(e.global::<f32>(0x011c_40ec), 9.5);
            // The menu flag makes the hold flag 1.
            assert_eq!(calls_to(&log, 0x0099_1600), vec![vec![0, 1]]);
        }

        #[test]
        fn ai_tasks_0086f6a0_resets_the_dialog_timer_for_an_open_dialog() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(0x0070_50d0, |_, _| 1u32.into_ret());
            let seen = recorder(&mut e, 0x0000_2100, 0);
            let dialog = object_with_vtable(&mut e, &[(0x100, YES), (0x404, 0x0000_2100)]);
            e.register_double(0x0076_24d0, move |_, _| dialog.into_ret());
            e.call(0x0086_f6a0, &args![main]);
            assert_eq!(*seen.borrow(), vec![vec![dialog, 0]]);
            // A byte at +0x131 of the object at 011d9510 suppresses it.
            let object = e.mem.alloc(0x200);
            e.mem.set_u8(object + 0x131, 1);
            e.set_global(0x011d_9510, object);
            e.call(0x0086_f6a0, &args![main]);
            assert_eq!(seen.borrow().len(), 1);
        }

        #[test]
        fn process_lists_0086f890_runs_the_lists_when_allowed() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            set_processors(&mut e, 1);
            e.call_log = Some(vec![]);
            e.call(0x0086_f890, &args![main]);
            let log = take_log(&mut e);
            let order: Vec<u32> = [0x0096_eb40, 0x008c_94e0, 0x0099_1dc0, 0x0096_d810]
                .iter()
                .map(|a| position(&log, *a) as u32)
                .collect();
            assert!(order.windows(2).all(|w| w[0] < w[1]));
            assert!(calls_to(&log, 0x0096_4260).is_empty());
        }

        #[test]
        fn process_lists_0086f890_updates_the_player_while_a_menu_blocks() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            set_processors(&mut e, 2);
            e.set_global(IN_MENU_FLAG, 1u8);
            let player = e.global::<u32>(PLAYER_OBJECT);
            e.call_log = Some(vec![]);
            e.call(0x0086_f890, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x0096_4260), vec![vec![player]]);
            assert!(calls_to(&log, 0x0096_d810).is_empty());
            assert!(calls_to(&log, 0x0096_eb40).is_empty());
            // Frozen time with no menu does the same; neither does nothing.
            e.set_global(IN_MENU_FLAG, 0u8);
            e.call_log = Some(vec![]);
            e.call(0x0086_f890, &args![main]);
            assert_eq!(calls_to(&take_log(&mut e), 0x0096_d810).len(), 1);
            e.mem.set_u8(main.addr() + MAIN_FREEZE_TIME, 1);
            e.call_log = Some(vec![]);
            e.call(0x0086_f890, &args![main]);
            assert_eq!(calls_to(&take_log(&mut e), 0x0096_4260).len(), 1);
        }

        // ----- 0086f940 ----------------------------------------------------------------------

        #[test]
        fn update_player_0086f940_stops_after_a_position_request() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(0x0093_bea0, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f940, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(log.len(), 2);
        }

        #[test]
        fn update_player_0086f940_in_a_menu_runs_009481d0() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.set_global(IN_MENU_FLAG, 1u8);
            let player = e.global::<u32>(PLAYER_OBJECT);
            e.call_log = Some(vec![]);
            e.call(0x0086_f940, &args![main]);
            assert_eq!(calls_to(&take_log(&mut e), 0x0094_81d0), vec![vec![player]]);
            // With the Pipboy opening it does not.
            e.register(0x0070_9bc0, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f940, &args![main]);
            assert!(calls_to(&take_log(&mut e), 0x0094_81d0).is_empty());
        }

        #[test]
        fn update_player_0086f940_fly_camera_updates_with_the_freeze_flag() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.mem.set_u8(main.addr() + MAIN_FLY_CAMERA, 1);
            e.mem.set_u8(main.addr() + MAIN_FREEZE_TIME, 1);
            e.register(TIMER_GET_SECONDS, |_, _| ret_f32(0.75));
            let player = object_with_vtable(&mut e, &[(0x1d0, YES)]);
            e.set_global(PLAYER_OBJECT, player);
            e.call_log = Some(vec![]);
            e.call(0x0086_f940, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(
                calls_to(&log, 0x0094_66d0),
                vec![vec![player, 0.75f32.to_bits(), 1]]
            );
            assert!(calls_to(&log, 0x0045_2580).is_empty());
        }

        #[test]
        fn update_player_0086f940_steps_the_player_with_the_vats_multiplier() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(TIMER_GET_SECONDS, |_, _| ret_f32(0.5));
            e.register(0x009c_8cc0, |_, _| ret_f32(3.0));
            let depth_seen = Rc::new(RefCell::new(Vec::new()));
            let depth_record = depth_seen.clone();
            e.register_double(0x0000_2200, move |e, a| {
                depth_record
                    .borrow_mut()
                    .push((a.to_vec(), e.global::<u8>(0x011e_07a8)));
                Ret::default()
            });
            let player = object_with_vtable(&mut e, &[(0x1d0, YES), (0x2f8, 0x0000_2200)]);
            e.set_global(PLAYER_OBJECT, player);
            e.set_global(0x011e_07a8, 4u8);
            e.call(0x0086_f940, &args![main]);
            assert_eq!(
                *depth_seen.borrow(),
                vec![(vec![player, 1.5f32.to_bits()], 5u8)]
            );
            assert_eq!(e.global::<u8>(0x011e_07a8), 4);
        }

        fn cell_engine() -> (Engine, u32, Ptr) {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            let player = e.global::<u32>(PLAYER_OBJECT);
            e.set_global(TES_OBJECT, 0x6100u32);
            e.set_global(DATA_HANDLER_OBJECT, 0x6200u32);
            let position = e.mem.alloc(12);
            e.mem.set_f32(position, 10.0);
            e.mem.set_f32(position + 4, 20.0);
            e.mem.set_f32(position + 8, 30.0);
            e.register_double(0x0043_6aa0, move |_, _| position.into_ret());
            e.register(0x008d_6f30, |_, _| 0x700u32.into_ret());
            e.register(0x0054_ddd0, |_, _| 0x55u32.into_ret());
            e.register(0x0046_1bc0, |_, _| 0x800u32.into_ret());
            e.register(0x0054_7590, |_, _| 0x99u32.into_ret());
            e.register(0x00b4_f5c0, |_, _| 0x1111u32.into_ret());
            (e, player, main)
        }

        #[test]
        fn update_player_0086f940_loads_the_cell_under_the_player() {
            let (mut e, player, main) = cell_engine();
            let tracker = recorder(&mut e, 0x0045_2580, 0);
            let order = Rc::new(RefCell::new(Vec::new()));
            let flag_order = order.clone();
            e.register_double(0x0054_8230, move |e, a| {
                flag_order.borrow_mut().push((
                    a.to_vec(),
                    e.global::<u8>(0x011d_cfa6),
                    e.mem.u8(a[1] + 0x5f9),
                ));
                Ret::default()
            });
            e.register(0x0045_1530, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f940, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(
                calls_to(&log, 0x0046_1bc0),
                vec![vec![0x6200, 10.0f32.to_bits(), 20.0f32.to_bits(), 0x55, 1]]
            );
            let tracker = tracker.borrow();
            assert_eq!(tracker.len(), 1);
            assert_eq!((tracker[0][0], tracker[0][2]), (0x6100, 1));
            assert_eq!(calls_to(&log, 0x0045_7d70), vec![vec![0x6100, 0, 0, 0]]);
            // While the cell is attached the two flags are set; afterwards they are cleared.
            assert_eq!(*order.borrow(), vec![(vec![0x800, player, 0], 1, 1)]);
            assert_eq!(e.global::<u8>(0x011d_cfa6), 0);
            assert_eq!(e.mem.u8(player + 0x5f9), 0);
            assert_eq!(e.global::<u32>(0x011d_cfb8), 0x99);
            assert_eq!(calls_to(&log, 0x00b6_55b0), vec![vec![0x1111]]);
        }

        #[test]
        fn update_player_0086f940_leaves_a_loaded_or_interior_cell_alone() {
            let (mut e, _player, main) = cell_engine();
            e.register(0x0055_0200, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f940, &args![main]);
            assert!(calls_to(&take_log(&mut e), 0x0046_1bc0).is_empty());
            let (mut e, _player, main) = cell_engine();
            e.register(0x0042_5fd0, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f940, &args![main]);
            let log = take_log(&mut e);
            assert!(calls_to(&log, 0x0055_0200).is_empty());
            // No grid cell found: nothing is attached.
            let (mut e, _player, main) = cell_engine();
            e.register(0x0046_1bc0, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x0086_f940, &args![main]);
            assert!(calls_to(&take_log(&mut e), 0x0054_8230).is_empty());
            // A grid cell that is already loaded is attached without moving the tracker.
            let (mut e, _player, main) = cell_engine();
            e.register(0x0045_0fb0, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_f940, &args![main]);
            let log = take_log(&mut e);
            assert!(calls_to(&log, 0x0045_2580).is_empty());
            assert_eq!(calls_to(&log, 0x0054_8230).len(), 1);
        }

        /// The function address the player's slot +0x2f8 (`PlayerCharacter::Update`) holds in
        /// the order test: a double that does nothing.
        const UPDATE_DOUBLE: u32 = 0x0000_2300;

        /// `0086f940` with the inputs of `world::frame::player::PlayerState` set through its
        /// callees' doubles: the calls it makes, in order.
        fn update_player_calls(s: &world::frame::player::PlayerState) -> Vec<u32> {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            let flag = |b: bool| u32::from(b);
            let (request, opening) = (flag(s.position_request), flag(s.pipboy_opening));
            e.register_double(0x0093_bea0, move |_, _| request.into_ret());
            e.register_double(0x0070_9bc0, move |_, _| opening.into_ret());
            e.set_global(IN_MENU_FLAG, u8::from(s.menu_flag));
            e.mem
                .set_u8(main.addr() + MAIN_FLY_CAMERA, u8::from(s.fly_camera));
            e.register(UPDATE_DOUBLE, |_, _| Ret::default());
            let three_d = if s.has_3d { YES } else { NOTHING };
            let player = object_with_vtable(&mut e, &[(0x1d0, three_d), (0x2f8, UPDATE_DOUBLE)]);
            e.set_global(PLAYER_OBJECT, player);
            let cell = if s.in_cell { 0x700u32 } else { 0 };
            e.register_double(0x008d_6f30, move |_, _| cell.into_ret());
            let (interior, inside) = (flag(s.interior), flag(!s.left_cell));
            e.register_double(0x0042_5fd0, move |_, _| interior.into_ret());
            e.register_double(0x0055_0200, move |_, _| inside.into_ret());
            e.register(0x0054_ddd0, |_, _| 0x55u32.into_ret());
            let found = if s.grid_cell_found { 0x800u32 } else { 0 };
            e.register_double(0x0046_1bc0, move |_, _| found.into_ret());
            let state = s.grid_cell_state;
            e.register_double(0x0045_0fb0, move |_, _| u32::from(state == 3).into_ret());
            e.register_double(0x0045_0ff0, move |_, _| u32::from(state == 6).into_ret());
            let tests = flag(s.cell_tests);
            e.register_double(0x0045_1530, move |_, _| tests.into_ret());
            let accumulator = if s.accumulator { 0x1111u32 } else { 0 };
            e.register_double(0x00b4_f5c0, move |_, _| accumulator.into_ret());
            for address in [
                0x0094_81d0,
                0x0094_66d0,
                0x009c_8cc0,
                0x0045_2580,
                0x0045_7d70,
                0x0054_8230,
                0x0054_7590,
                0x00b6_55b0,
            ] {
                e.register(address, |_, _| Ret::default());
            }
            e.call_log = Some(vec![]);
            e.call(0x0086_f940, &args![main]);
            let to_model = |a: u32| match a {
                YES | NOTHING => world::frame::player::player_slot(0x1d0),
                UPDATE_DOUBLE => world::frame::player::UPDATE_ADDRESS,
                other => other,
            };
            // The log starts with the call of `0086f940` itself.
            take_log(&mut e)
                .into_iter()
                .skip(1)
                .map(|(a, _)| to_model(a))
                .collect()
        }

        /// The calls `0086f940` makes are `world::frame::player::UPDATE_PLAYER`'s that its gates
        /// let through, in its order, for each of its tests (docs/FRAME_SKELETON.md, PR 4).
        #[test]
        fn update_player_0086f940_follows_the_frame_model() {
            use world::frame::player::{calls_run, PlayerState, UPDATE_PLAYER};
            let game = PlayerState::default();
            let left = PlayerState {
                left_cell: true,
                ..game
            };
            let states = [
                game,
                PlayerState {
                    position_request: true,
                    ..game
                },
                PlayerState {
                    menu_flag: true,
                    ..game
                },
                PlayerState {
                    menu_flag: true,
                    pipboy_opening: true,
                    ..game
                },
                PlayerState {
                    pipboy_opening: true,
                    ..game
                },
                PlayerState {
                    fly_camera: true,
                    ..game
                },
                PlayerState {
                    fly_camera: true,
                    has_3d: false,
                    ..game
                },
                PlayerState {
                    has_3d: false,
                    ..game
                },
                PlayerState {
                    in_cell: false,
                    ..game
                },
                PlayerState {
                    interior: true,
                    ..left
                },
                left,
                PlayerState {
                    cell_tests: true,
                    ..left
                },
                PlayerState {
                    grid_cell_state: 3,
                    ..left
                },
                PlayerState {
                    grid_cell_state: 6,
                    ..left
                },
                PlayerState {
                    grid_cell_found: false,
                    ..left
                },
                PlayerState {
                    accumulator: false,
                    ..left
                },
            ];
            // The translation calls this file's three small setters as Rust functions, not
            // through the engine, so they aren't in the log.
            let direct = [0x0086_fba0, 0x0086_fbb0, 0x0086_fbc0];
            for s in states {
                let want: Vec<u32> = calls_run(&s)
                    .into_iter()
                    .map(|i| UPDATE_PLAYER[i].callee.address())
                    .filter(|a| !direct.contains(a))
                    .collect();
                assert_eq!(update_player_calls(&s), want, "{s:?}");
            }
        }

        /// The calls `0086fbe0` makes are `world::frame::world_time`'s model of it, exactly, for
        /// each of its tests (docs/FRAME_SKELETON.md, PR 5).
        #[test]
        fn update_current_grid_cell_0086fbe0_follows_the_frame_model() {
            use world::frame::world_time::{function, steps_run, Callee, WorldState};
            let model = function(0x0086_fbe0).expect("modelled");
            let game = WorldState::default();
            let tests = WorldState {
                cell_tests: true,
                ..game
            };
            let states = [
                game,
                WorldState {
                    menu_flag: true,
                    ..game
                },
                WorldState {
                    new_game_loading: true,
                    ..game
                },
                WorldState {
                    world_frozen: true,
                    ..game
                },
                tests,
                WorldState {
                    interior_loaded: true,
                    ..tests
                },
            ];
            for s in states {
                let mut e = idle_engine();
                let main = main_object(&mut e);
                e.set_global(IN_MENU_FLAG, u8::from(s.menu_flag));
                e.set_global(0x011d_8907, u8::from(s.new_game_loading));
                e.mem
                    .set_u8(main.addr() + MAIN_FREEZE_TIME, u8::from(s.world_frozen));
                let tests = u32::from(s.cell_tests);
                e.register_double(0x0045_1530, move |_, _| tests.into_ret());
                let interior = if s.interior_loaded { 0x6200u32 } else { 0 };
                e.register_double(0x005f_36f0, move |_, _| interior.into_ret());
                e.call_log = Some(vec![]);
                e.call(0x0086_fbe0, &args![main]);
                let calls: Vec<u32> = take_log(&mut e)
                    .into_iter()
                    .skip(1)
                    .map(|(a, _)| a)
                    .collect();
                let want: Vec<u32> = steps_run(&s, model)
                    .into_iter()
                    .map(|i| match model.steps[i].callee {
                        Callee::Direct(a) => a,
                        other => panic!("{other:?}"),
                    })
                    .collect();
                assert_eq!(calls, want, "{s:?}");
            }
        }

        /// The calls `0086fc60` makes follow `world::frame::ai_stage`'s model of it for each of its
        /// tests: the thread count, menu mode with and without the dialogue and fader flags, the
        /// frozen world (docs/FRAME_SKELETON.md, PR 6).
        #[test]
        fn update_animations_and_effects_0086fc60_follows_the_frame_model() {
            use world::frame::ai_stage::{follows, function, steps_run, AiState, Callee};
            let model = function(0x0086_fc60).expect("modelled");
            let game = AiState::default();
            let menu = AiState {
                menu_flag: true,
                in_menu_mode: true,
                ..game
            };
            let mut states = Vec::new();
            for threads in [1, 2, 4] {
                for s in [
                    game,
                    menu,
                    AiState {
                        in_dialog: true,
                        ..menu
                    },
                    AiState {
                        fader_visible: true,
                        ..menu
                    },
                    AiState {
                        world_frozen: true,
                        ..game
                    },
                ] {
                    states.push(AiState { threads, ..s });
                }
            }
            for s in states {
                let mut e = idle_engine();
                let main = main_object(&mut e);
                set_processors(&mut e, s.threads as u32);
                e.set_global(TES_OBJECT, 0x6100u32);
                e.set_global(IN_MENU_FLAG, u8::from(s.menu_flag));
                e.set_global(IN_DIALOG_FLAG, u8::from(s.in_dialog));
                e.set_global(FADER_ONE_FLAG, u8::from(s.fader_visible));
                e.mem
                    .set_u8(main.addr() + MAIN_FREEZE_TIME, u8::from(s.world_frozen));
                e.call_log = Some(vec![]);
                e.call(0x0086_fc60, &args![main]);
                let log: Vec<Callee> = take_log(&mut e)
                    .into_iter()
                    .skip(1)
                    .map(|(a, _)| Callee::Direct(a))
                    .collect();
                follows(model, &s, &log, &[]).unwrap_or_else(|why| panic!("{s:?}: {why}"));
                // Exactly the reached calls (the model has no own tests here).
                let seen = log
                    .iter()
                    .filter(|c| model.steps.iter().any(|st| st.callee == **c))
                    .count();
                assert_eq!(seen, steps_run(&s, model).len(), "{s:?}");
            }
        }

        // ----- the interface and render stages (docs/FRAME_SKELETON.md, PR 7) ----------------

        /// The calls `address` made, as the model's callees, without its own entry.
        fn logged(e: &mut Engine) -> Vec<world::frame::interface::Callee> {
            take_log(e)
                .into_iter()
                .skip(1)
                .map(|(a, _)| world::frame::interface::Callee::Direct(a))
                .collect()
        }

        fn check(
            address: u32,
            s: &world::frame::interface::InterfaceState,
            log: &[world::frame::interface::Callee],
        ) {
            use world::frame::interface::{follows, function};
            let model = function(address).expect("modelled");
            follows(model, s, log, &[]).unwrap_or_else(|why| panic!("{address:08x} {s:?}: {why}"));
        }

        /// `0086fd70` makes its three calls, in order, every time.
        #[test]
        fn interface_idle_0086fd70_follows_the_frame_model() {
            let mut e = idle_engine();
            e.call_log = Some(vec![]);
            e.call(0x0086_fd70, &args![0u32]);
            let log = logged(&mut e);
            check(0x0086_fd70, &Default::default(), &log);
            assert_eq!(log.len(), 3);
        }

        /// `0086f390` polls every frame and clears the user actions only in V.A.T.S.'s
        /// playback or as the Pip-Boy comes up, and not with the fly camera.
        #[test]
        fn poll_controls_0086f390_follows_the_frame_model() {
            use world::frame::interface::InterfaceState;
            let game = InterfaceState::default();
            let cases = [
                game,
                InterfaceState {
                    pipboy_opening: true,
                    ..game
                },
                InterfaceState {
                    vats_playback: true,
                    ..game
                },
                InterfaceState {
                    vats_playback: true,
                    vats_test: true,
                    ..game
                },
                InterfaceState {
                    pipboy_opening: true,
                    fly_camera: true,
                    ..game
                },
            ];
            for s in cases {
                let (mut e, main) = poll_engine();
                let opening = s.pipboy_opening;
                e.register_double(0x0070_9bc0, move |_, _| u32::from(opening).into_ret());
                let mode = if s.vats_playback { 4u32 } else { 0 };
                e.register_double(0x0044_ddc0, move |_, _| mode.into_ret());
                let test = s.vats_test;
                e.register_double(0x007d_1360, move |_, _| u32::from(test).into_ret());
                e.mem
                    .set_u8(main.addr() + MAIN_FLY_CAMERA, u8::from(s.fly_camera));
                e.call_log = Some(vec![]);
                e.call(0x0086_f390, &args![main]);
                let log = logged(&mut e);
                check(0x0086_f390, &s, &log);
            }
        }

        /// `0086f890` (in `Main::PostSwapProcess`) for one and more threads, in menu mode,
        /// with fader 1, the console and the frozen world.
        #[test]
        fn process_lists_0086f890_follows_the_frame_model() {
            use world::frame::interface::InterfaceState;
            for threads in [1, 2] {
                let game = InterfaceState {
                    threads,
                    ..InterfaceState::default()
                };
                for s in [
                    game,
                    InterfaceState {
                        menu_flag: true,
                        ..game
                    },
                    InterfaceState {
                        menu_flag: true,
                        fader_visible: true,
                        ..game
                    },
                    InterfaceState {
                        console_visible: true,
                        ..game
                    },
                    InterfaceState {
                        world_frozen: true,
                        ..game
                    },
                ] {
                    let mut e = idle_engine();
                    let main = main_object(&mut e);
                    set_processors(&mut e, threads as u32);
                    e.set_global(IN_MENU_FLAG, u8::from(s.menu_flag));
                    e.set_global(FADER_ONE_FLAG, u8::from(s.fader_visible));
                    e.set_global(CONSOLE_VISIBLE_FLAG, u8::from(s.console_visible));
                    e.mem
                        .set_u8(main.addr() + MAIN_FREEZE_TIME, u8::from(s.world_frozen));
                    e.call_log = Some(vec![]);
                    e.call(0x0086_f890, &args![main]);
                    let log = logged(&mut e);
                    check(0x0086_f890, &s, &log);
                    let model = world::frame::interface::function(0x0086_f890).unwrap();
                    let seen = log
                        .iter()
                        .filter(|c| model.steps.iter().any(|st| st.callee == **c))
                        .count();
                    assert_eq!(
                        seen,
                        world::frame::interface::steps_run(&s, model).len(),
                        "{s:?}"
                    );
                }
            }
        }

        /// `0086f6a0` for one and more threads, and in dialogue.
        #[test]
        fn non_render_safe_0086f6a0_follows_the_frame_model() {
            use world::frame::interface::InterfaceState;
            for threads in [1, 2] {
                for in_dialog in [false, true] {
                    let mut e = idle_engine();
                    let main = main_object(&mut e);
                    set_processors(&mut e, threads);
                    if in_dialog {
                        e.register(0x0070_50d0, |_, _| 1u32.into_ret());
                        let dialog = object_with_vtable(&mut e, &[(0x100, YES), (0x404, NOTHING)]);
                        e.register_double(0x0076_24d0, move |_, _| dialog.into_ret());
                    }
                    e.call_log = Some(vec![]);
                    e.call(0x0086_f6a0, &args![main]);
                    let log = logged(&mut e);
                    let s = InterfaceState {
                        threads: threads as i32,
                        in_dialog,
                        ..InterfaceState::default()
                    };
                    check(0x0086_f6a0, &s, &log);
                }
            }
        }

        /// `Main::Swap` (`0086ff70`): the world drawn, or the menus over the held background,
        /// the menu change's background, the faders' loading screen, the threads' release.
        #[test]
        fn swap_0086ff70_follows_the_frame_model() {
            use world::frame::interface::InterfaceState;
            let game = InterfaceState::default();
            let cases = [
                game,
                InterfaceState { threads: 1, ..game },
                InterfaceState {
                    background_held: true,
                    menus_on_screen: true,
                    menu_flag: true,
                    ..game
                },
                InterfaceState {
                    background_held: true,
                    ..game
                },
                InterfaceState {
                    menu_changed: true,
                    ..game
                },
                InterfaceState {
                    faders_up: true,
                    ..game
                },
            ];
            for s in cases {
                let mut e = idle_engine();
                let main = main_object(&mut e);
                set_processors(&mut e, s.threads as u32);
                // The player's 3D is there.
                let player = e.global::<u32>(PLAYER_OBJECT);
                let table = e.mem.u32(player);
                e.mem.set_u32(table + 0x1d0, YES);
                e.set_global(SCENE_FLAG_29, u8::from(s.background_held));
                let menus = s.menus_on_screen;
                e.register_double(IS_IN_MENU_MODE, move |_, _| u32::from(menus).into_ret());
                e.set_global(0x011d_890a, u8::from(s.menu_changed));
                // The background's own drawing is checked on its own.
                e.register(0x0087_1dc0, |_, _| Ret::default());
                let faders = s.faders_up;
                e.register_double(0x0070_1450, move |_, a| (faders && a[1] == 1).into_ret());
                e.call_log = Some(vec![]);
                e.call(0x0086_ff70, &args![main]);
                let log = logged(&mut e);
                check(0x0086_ff70, &s, &log);
            }
        }

        // ----- 0086fbe0 / 0086fc60 -----------------------------------------------------------

        #[test]
        fn player_cell_0086fbe0_refreshes_the_world_spaces() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            let player = e.global::<u32>(PLAYER_OBJECT);
            e.set_global(TES_OBJECT, 0x6100u32);
            e.register(0x0043_6aa0, |_, _| 0x3300u32.into_ret());
            e.register(0x0045_1530, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_fbe0, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x0045_2580), vec![vec![0x6100, 0x3300, 1]]);
            assert_eq!(calls_to(&log, 0x0057_5d70), vec![vec![player]]);
            assert_eq!(calls_to(&log, 0x004f_d3e0), vec![vec![0x6100]]);
        }

        #[test]
        fn player_cell_0086fbe0_stops_at_each_condition() {
            // A menu, the byte at 011d8907 and frozen time all stop it before anything is called.
            for stop in 0..3 {
                let mut e = idle_engine();
                let main = main_object(&mut e);
                match stop {
                    0 => e.set_global(IN_MENU_FLAG, 1u8),
                    1 => e.set_global(0x011d_8907, 1u8),
                    _ => e.mem.set_u8(main.addr() + MAIN_FREEZE_TIME, 1),
                }
                e.call_log = Some(vec![]);
                e.call(0x0086_fbe0, &args![main]);
                assert_eq!(take_log(&mut e).len(), 1);
            }
            // No cell: the tracker is moved but nothing else happens.
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.call_log = Some(vec![]);
            e.call(0x0086_fbe0, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x0045_2580).len(), 1);
            assert!(calls_to(&log, 0x0057_5d70).is_empty());
            // A cell that `005f36f0` already knows leaves the world spaces alone.
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(0x0045_1530, |_, _| 1u32.into_ret());
            e.register(0x005f_36f0, |_, _| 5u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_fbe0, &args![main]);
            assert!(calls_to(&take_log(&mut e), 0x0057_5d70).is_empty());
        }

        #[test]
        fn frame_update_0086fc60_scales_the_cell_update_with_one_processor() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            set_processors(&mut e, 1);
            e.set_global(TES_OBJECT, 0x6100u32);
            e.set_global(0x011d_ea30, 0.5f32);
            e.register(TIMER_GET_SECONDS, |_, _| ret_f32(2.0));
            e.register(0x0045_a190, |_, a| {
                assert!(a.is_empty());
                0x7704u32.into_ret()
            });
            let built = recorder(&mut e, 0x0043_d410, 0);
            e.call_log = Some(vec![]);
            e.call(0x0086_fc60, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(
                calls_to(&log, 0x0097_4420),
                vec![vec![PROCESS_LISTS, 2.0f32.to_bits()]]
            );
            assert_eq!(
                calls_to(&log, 0x0045_3550),
                vec![vec![0x6100, 0.5f32.to_bits()]]
            );
            let built = built.borrow();
            assert_eq!(built.len(), 1);
            assert_eq!(built[0][1..], [0.5f32.to_bits(), 1, 0]);
            let held = calls_to(&log, 0x00c5_0610);
            assert_eq!(held.len(), 1);
            assert_eq!(held[0][0], 0x7704);
            assert_eq!(held[0][1], built[0][0]);
        }

        #[test]
        fn frame_update_0086fc60_uses_the_other_calls_with_several_processors() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            set_processors(&mut e, 2);
            e.set_global(TES_OBJECT, 0x6100u32);
            e.call_log = Some(vec![]);
            e.call(0x0086_fc60, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x0097_46c0).len(), 1);
            assert!(calls_to(&log, 0x0097_4420).is_empty());
            assert_eq!(calls_to(&log, 0x0045_37c0), vec![vec![0x6100]]);
            assert!(calls_to(&log, 0x0045_3550).is_empty());
        }

        #[test]
        fn frame_update_0086fc60_waits_for_a_dialog_or_fader_flag_in_a_menu() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.set_global(IN_MENU_FLAG, 1u8);
            e.call_log = Some(vec![]);
            e.call(0x0086_fc60, &args![main]);
            assert!(calls_to(&take_log(&mut e), 0x0097_4420).is_empty());
            e.set_global(IN_DIALOG_FLAG, 1u8);
            e.call_log = Some(vec![]);
            e.call(0x0086_fc60, &args![main]);
            assert_eq!(calls_to(&take_log(&mut e), 0x0097_4420).len(), 1);
            // Frozen time always stops it.
            e.mem.set_u8(main.addr() + MAIN_FREEZE_TIME, 1);
            e.call_log = Some(vec![]);
            e.call(0x0086_fc60, &args![main]);
            assert!(calls_to(&take_log(&mut e), 0x0097_4420).is_empty());
        }

        // ----- 0086fd90 ----------------------------------------------------------------------

        /// A list of `count` nodes: a list entry holds its node at +0, and `00726070` follows the
        /// chain. Returns the first entry and the nodes.
        fn node_list(e: &mut Engine, count: usize, slots: &[(u32, u32)]) -> (u32, Vec<u32>) {
            let entries: Vec<u32> = (0..count).map(|_| e.mem.alloc(8)).collect();
            let mut nodes = Vec::new();
            for entry in &entries {
                let node = object_with_vtable(e, slots);
                e.mem.set_u8(node + 8, 1);
                e.mem.set_u32(*entry, node);
                nodes.push(node);
            }
            let chain = entries.clone();
            e.register_double(0x0072_6070, move |_, a| {
                let at = chain.iter().position(|entry| *entry == a[0]).unwrap();
                chain.get(at + 1).copied().unwrap_or(0).into_ret()
            });
            e.register(0x0068_15c0, |_, a| a[0].into_ret());
            let first = entries[0];
            e.register_double(0x0043_b5d0, move |_, _| first.into_ret());
            (first, nodes)
        }

        #[test]
        fn image_space_0086fd90_adds_seconds_only_when_running() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(TIMER_GET_SECONDS, |_, _| ret_f32(0.5));
            e.set_global(0x011c_96fc, 1.0f32);
            e.call(0x0086_fd90, &args![main, 0u8]);
            assert_eq!(e.global::<f32>(0x011c_96fc), 1.0);
            e.call_log = Some(vec![]);
            e.call(0x0086_fd90, &args![main, 1u8]);
            let log = take_log(&mut e);
            assert_eq!(e.global::<f32>(0x011c_96fc), 1.5);
            // The manager is brought up, finished, and the fader value asked with zero.
            assert!(position(&log, 0x00b8_b500) < position(&log, 0x00b8_b9a0));
            assert!(position(&log, 0x00b8_d020) < position(&log, 0x00b8_b440));
            assert_eq!(calls_to(&log, 0x00b8_b440).len(), 1);
        }

        #[test]
        fn image_space_0086fd90_takes_removable_nodes_out_of_the_list() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            let removals = Rc::new(RefCell::new(0));
            let counter = removals.clone();
            e.register_double(0x0000_2300, move |_, _| {
                let mut count = counter.borrow_mut();
                *count += 1;
                (*count == 1).into_ret()
            });
            let (first, nodes) = node_list(&mut e, 1, &[(0x8c, 0x0000_2300), (0x90, NOTHING)]);
            e.call_log = Some(vec![]);
            e.call(0x0086_fd90, &args![main, 0u8]);
            let log = take_log(&mut e);
            // The first visit removes the node (temporary built from it, removal from the list
            // entry), the entry is then visited again and kept.
            let built = calls_to(&log, 0x0063_3c90);
            assert_eq!(built.len(), 1);
            assert_eq!(built[0][1], nodes[0]);
            assert_eq!(calls_to(&log, 0x0063_1620), vec![vec![first, built[0][0]]]);
            assert_eq!(calls_to(&log, 0x0045_cec0), vec![vec![built[0][0]]]);
            assert_eq!(*removals.borrow(), 2);
        }

        #[test]
        fn image_space_0086fd90_ends_a_batch_after_four_counted_nodes() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            let (_, _) = node_list(&mut e, 4, &[(0x8c, NOTHING), (0x90, NOTHING)]);
            e.call_log = Some(vec![]);
            e.call(0x0086_fd90, &args![main, 0u8]);
            let log = take_log(&mut e);
            // No node had a positive value: the batch starts with 00b8aea0, then 00b8ccb0.
            assert_eq!(calls_to(&log, 0x00b8_aea0).len(), 1);
            assert_eq!(calls_to(&log, 0x00b8_ccb0).len(), 1);
            assert!(position(&log, 0x00b8_aea0) < position(&log, 0x00b8_ccb0));
            assert_eq!(calls_to(&log, 0x0072_6070).len(), 4);
        }

        #[test]
        fn image_space_0086fd90_skips_the_first_call_when_a_node_has_a_positive_value() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            let (_, _) = node_list(&mut e, 4, &[(0x8c, NOTHING), (0x90, NOTHING)]);
            e.set_global(0x0118_abb0, 1u8);
            e.mem.set_f64(0x0101_2060, 0.0);
            e.register(TIMER_GET_SECONDS, |_, _| ret_f32(1.0));
            e.call_log = Some(vec![]);
            e.call(0x0086_fd90, &args![main, 0u8]);
            let log = take_log(&mut e);
            assert!(calls_to(&log, 0x00b8_aea0).is_empty());
            assert_eq!(calls_to(&log, 0x00b8_ccb0).len(), 1);
            // A value that is not above the limit does not count.
            let mut e = idle_engine();
            let main = main_object(&mut e);
            let (_, _) = node_list(&mut e, 4, &[(0x8c, NOTHING), (0x90, NOTHING)]);
            e.set_global(0x0118_abb0, 1u8);
            e.mem.set_f64(0x0101_2060, 1.0);
            e.register(TIMER_GET_SECONDS, |_, _| ret_f32(1.0));
            e.call_log = Some(vec![]);
            e.call(0x0086_fd90, &args![main, 0u8]);
            assert_eq!(calls_to(&take_log(&mut e), 0x00b8_aea0).len(), 1);
        }

        // ----- 0086ff70 ----------------------------------------------------------------------

        #[test]
        fn frame_end_0086ff70_redraws_the_menu_background_once() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.set_global(0x011d_890a, 1u8);
            e.register(0x0095_0bb0, |_, a| (0x100 + a[1]).into_ret());
            e.register(0x0045_6610, |_, a| (a[0] & 0xff).into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_ff70, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(e.global::<u8>(0x011d_890a), 0);
            assert_eq!(
                calls_to(&log, 0x0045_0f90),
                vec![
                    vec![0x101, 1],
                    vec![0x100, 1],
                    vec![0x101, 1],
                    vec![0x100, 0]
                ]
            );
            let render = position(&log, 0x0087_1dc0);
            assert_eq!(calls_to(&log, 0x0087_1dc0), vec![vec![main.addr()]]);
            assert!(render > position(&log, 0x0045_6610));
            // Without the flag nothing of this happens.
            e.call_log = Some(vec![]);
            e.call(0x0086_ff70, &args![main]);
            assert!(calls_to(&take_log(&mut e), 0x0045_0f90).is_empty());
        }

        #[test]
        fn frame_end_0086ff70_ages_the_fade_counter_and_removes_the_faders() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(0x0070_1450, |_, a| (a[1] == 1).into_ret());
            e.register(0x0070_13e0, |_, _| ret_f32(0.5));
            e.mem.set_f64(0x0101_2070, 1.0);
            e.set_global(0x011d_e45c, 0x7100u32);
            e.register(0x0047_c850, |_, _| 1u32.into_ret());
            let fader = e.global::<u32>(FADER_MANAGER);
            e.call_log = Some(vec![]);
            // The first frame also resets the process lists.
            e.call(0x0086_ff70, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x0096_cfa0).len(), 1);
            assert_eq!(e.global::<u32>(0x011d_ef04), 1);
            assert_eq!(e.global::<f32>(0x011d_ef00), 0.5);
            assert!(calls_to(&log, 0x0070_10e0).is_empty());
            // Once a second and ten frames have passed the faders are removed.
            e.set_global(0x011d_ef04, 9u32);
            e.call_log = Some(vec![]);
            e.call(0x0086_ff70, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(
                calls_to(&log, 0x0070_10e0),
                vec![vec![fader, 1, 0], vec![fader, 2, 0]]
            );
            assert_eq!(
                calls_to(&log, 0x0045_34f0),
                vec![vec![0x7100, 0], vec![0x7100, 0]]
            );
            assert_eq!(calls_to(&log, 0x0048_3710), vec![vec![0x7100]]);
            assert_eq!(calls_to(&log, 0x0087_7430), vec![vec![main.addr()]]);
            assert!(calls_to(&log, 0x0096_cfa0).is_empty());
        }

        #[test]
        fn frame_end_0086ff70_resets_the_fade_counter_without_faders() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.set_global(0x011d_ef00, 3.0f32);
            e.set_global(0x011d_ef04, 8u32);
            e.call(0x0086_ff70, &args![main]);
            assert_eq!(e.global::<f32>(0x011d_ef00), 0.0);
            assert_eq!(e.global::<u32>(0x011d_ef04), 0);
        }

        #[test]
        fn frame_end_0086ff70_world_path_updates_the_world() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            let player = object_with_vtable(&mut e, &[(0x1d0, YES)]);
            e.set_global(PLAYER_OBJECT, player);
            e.set_global(TES_OBJECT, 0x6100u32);
            e.call_log = Some(vec![]);
            e.call(0x0086_ff70, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(
                calls_to(&log, 0x0087_06b0),
                vec![vec![main.addr(), 0, 0, 0]]
            );
            assert!(calls_to(&log, 0x0087_1a50).is_empty());
        }

        #[test]
        fn frame_end_0086ff70_world_path_loads_the_map_when_the_cell_has_one() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            let player = object_with_vtable(&mut e, &[(0x1d0, YES)]);
            e.set_global(PLAYER_OBJECT, player);
            e.set_global(TES_OBJECT, 0x6100u32);
            let cell_data = e.mem.alloc(4);
            e.mem.set_u8(cell_data, 0x80);
            e.register(0x004f_d3e0, |_, _| 0x5555u32.into_ret());
            e.register_double(0x0054_8210, move |_, _| cell_data.into_ret());
            e.register(0x0044_ddc0, |_, _| 0x66u32.into_ret());
            e.register(0x0087_05c0, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_ff70, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x0054_8210), vec![vec![0x5555]]);
            assert_eq!(calls_to(&log, 0x004b_af10), vec![vec![0x66]]);
        }

        #[test]
        fn frame_end_0086ff70_menu_path_runs_the_stages_and_the_menu_update() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(IS_IN_MENU_MODE, |_, _| 1u32.into_ret());
            e.register(0x004e_a970, |_, _| 0x1200u32.into_ret());
            e.register(0x004e_3270, |_, _| 0x1300u32.into_ret());
            e.register(0x004d_c310, |_, _| 1u32.into_ret());
            e.register(GET_RENDERER, |_, _| 0x1400u32.into_ret());
            e.mem.set_u32(0x011d_ed3c, 0x1500);
            e.call_log = Some(vec![]);
            e.call(0x0086_ff70, &args![main]);
            let log = take_log(&mut e);
            let first_stages = calls_to(&log, 0x00ba_30f0);
            assert_eq!(first_stages.len(), 22);
            assert_eq!(first_stages[0], vec![0x1200, 0, 1]);
            assert_eq!(first_stages[21], vec![0x1200, 0, 22]);
            let second_stages = calls_to(&log, 0x00ba_3130);
            assert_eq!(second_stages.len(), 22);
            assert_eq!(second_stages[0], vec![0x1200, 1, 1]);
            assert_eq!(calls_to(&log, 0x0087_1a50), vec![vec![main.addr()]]);
            assert_eq!(
                calls_to(&log, 0x0087_4b90),
                vec![vec![main.addr(), 0x1300, 0x1400, 1, 0]]
            );
            assert!(calls_to(&log, 0x0087_2940).is_empty());
        }

        #[test]
        fn frame_end_0086ff70_menu_path_with_a_covering_menu_runs_00872940() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(IS_IN_MENU_MODE, |_, _| 1u32.into_ret());
            e.register(0x0070_2680, |_, a| (a[0] == 0x438).into_ret());
            e.mem.set_u32(0x011d_ed3c, 0x1500);
            e.call_log = Some(vec![]);
            e.call(0x0086_ff70, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x0087_2940), vec![vec![main.addr(), 0]]);
            assert!(calls_to(&log, 0x0087_4b90).is_empty());
        }

        #[test]
        fn frame_end_0086ff70_menu_path_when_the_menu_0x3f5_is_visible_without_menu_mode() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(0x0070_2680, |_, a| (a[0] == 0x3f5).into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_ff70, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x0087_1a50).len(), 1);
            // A rendered menu makes the menu-mode test not count.
            let mut e = idle_engine();
            e.register(IS_IN_MENU_MODE, |_, _| 1u32.into_ret());
            e.register(0x0070_79b0, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_ff70, &args![main]);
            let log = take_log(&mut e);
            assert!(calls_to(&log, 0x0087_1a50).is_empty());
            // Nor does the world update run: the player's virtual +0x1d0 says no.
            assert!(calls_to(&log, 0x0087_06b0).is_empty());
        }

        #[test]
        fn frame_end_0086ff70_non_menu_path_passes_the_rendered_menu_test() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            let player = object_with_vtable(&mut e, &[(0x1d0, YES)]);
            e.set_global(PLAYER_OBJECT, player);
            // The world test fails because the byte at 011dea29 is set; no menu is up, so the
            // rendered-menu test is passed on.
            e.set_global(SCENE_FLAG_29, 1u8);
            e.register(0x0070_79b0, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_ff70, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(
                calls_to(&log, 0x0087_06b0),
                vec![vec![main.addr(), 0, 1, 0]]
            );
        }

        #[test]
        fn frame_end_0086ff70_closes_the_frame() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            set_processors(&mut e, 2);
            e.register(GET_RENDERER, |_, _| 0x1400u32.into_ret());
            e.register(0x004e_9530, |_, _| 1u32.into_ret());
            e.register(0x005d_4a40, |_, _| 1u32.into_ret());
            e.set_global(0x011c_6fb8, 1u8);
            e.mem.set_u32(0x011d_ec64, 0x4343);
            e.call_log = Some(vec![]);
            e.call(0x0086_ff70, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x004a_0370), vec![vec![0x1400]]);
            assert_eq!(calls_to(&log, 0x0055_85e0), vec![vec![0x1400]]);
            assert_eq!(calls_to(&log, 0x004d_cef0).len(), 1);
            assert_eq!(calls_to(&log, 0x00b5_5a10).len(), 1);
            assert_eq!(calls_to(&log, 0x0070_28b0), vec![vec![0, 0x4343]]);
            // The holder at 011dec64 is handed to 00b6da10 and cleared.
            assert_eq!(calls_to(&log, 0x00b6_da10), vec![vec![0, 0x4343]]);
            assert_eq!(e.mem.u32(0x011d_ec64), 0);
            assert_eq!(calls_to(&log, 0x00b6_b730).len(), 1);
            assert_eq!(calls_to(&log, 0x004a_03c0), vec![vec![0x1400]]);
            assert_eq!(calls_to(&log, 0x008c_80e0), vec![vec![0]]);
            let last = log.last().unwrap();
            assert_eq!(last.0, 0x008c_80e0);
        }

        #[test]
        fn frame_end_0086ff70_toggles_the_debug_counter_and_makes_a_directory() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(0x007f_df30, |_, _| 0x5100u32.into_ret());
            e.register(0x00a2_4660, |_, _| 1u32.into_ret());
            e.register(0x00a2_4180, |_, a| (a[2] == 0).into_ret());
            e.register(0x0040_3df0, |_, _| 0x0107_0000u32.into_ret());
            e.mem.set_u32(ZERO_WORD, 7);
            let text = recorder(&mut e, 0x0040_6d00, 0);
            let made = recorder(&mut e, API_CREATE_DIRECTORY, 0);
            e.call_log = Some(vec![]);
            e.call(0x0086_ff70, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(e.global::<u8>(0x011d_ea40), 1);
            assert_eq!(e.global::<u32>(0x011d_ea44), 0);
            // The setting is raised by one and then read again for the text.
            assert_eq!(calls_to(&log, 0x0045_ce80), vec![vec![0x011d_eecc, 8]]);
            let text = text.borrow();
            assert_eq!(text.len(), 1);
            assert_eq!(text[0][1..], [0x104, 0x0108_2ca4, 0x0107_0000, 7]);
            let made = made.borrow();
            assert_eq!(made.len(), 1);
            assert_eq!((made[0][0], made[0][1]), (text[0][0], 0));
            // The timer object is told the value of the setting at 011dee00.
            assert_eq!(calls_to(&log, 0x00a2_4660), vec![vec![0x5100, 0x1e, 1]]);
            // Pressed again, the counter turns off.
            e.call_log = Some(vec![]);
            e.call(0x0086_ff70, &args![main]);
            assert_eq!(e.global::<u8>(0x011d_ea40), 0);
            let log = take_log(&mut e);
            assert!(calls_to(&log, 0x0040_6d00).is_empty());
        }

        #[test]
        fn frame_end_0086ff70_without_the_second_key_it_calls_00878860() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(0x00a2_4660, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_ff70, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, 0x0087_8860), vec![vec![0]]);
            assert_eq!(e.global::<u8>(0x011d_ea40), 0);
            // Without the first key nothing happens.
            let mut e = idle_engine();
            e.call_log = Some(vec![]);
            e.call(0x0086_ff70, &args![main]);
            assert!(calls_to(&take_log(&mut e), 0x0087_8860).is_empty());
        }

        // ----- 0086e650 ----------------------------------------------------------------------

        #[test]
        fn idle_update_0086e650_does_nothing_when_alt_and_tab_are_down() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(API_GET_ASYNC_KEY_STATE, |_, _| 0x8000u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_e650, &args![main]);
            let log = take_log(&mut e);
            let addresses: Vec<(u32, Vec<u32>)> = log;
            assert_eq!(
                addresses,
                vec![
                    (0x0086_e650, vec![main.addr()]),
                    (API_GET_ASYNC_KEY_STATE, vec![9]),
                    (API_GET_ASYNC_KEY_STATE, vec![0x12]),
                ]
            );
        }

        #[test]
        fn idle_update_0086e650_tab_alone_does_not_skip_the_frame() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(API_GET_ASYNC_KEY_STATE, |_, a| {
                if a[0] == 9 { 0x8000u32 } else { 0 }.into_ret()
            });
            e.call_log = Some(vec![]);
            e.call(0x0086_e650, &args![main]);
            assert_eq!(calls_to(&take_log(&mut e), 0x00af_2640).len(), 1);
        }

        #[test]
        fn idle_update_0086e650_runs_the_steps_in_order_and_counts_the_frame() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.set_global(0x011a_2fe0, 10u32);
            e.set_global(0x011d_f674, 5u32);
            e.set_global(0x011f_9138, 2u32);
            e.set_global(0x0126_77a2, 1u8);
            e.call_log = Some(vec![]);
            e.call(0x0086_e650, &args![main]);
            let log = take_log(&mut e);
            let order = [
                0x00af_2640u32, // the system object
                0x0093_bea0,    // player update
                0x00b8_b500,    // image space
                0x0085_1d90,    // queued saves
                0x00a2_3010,    // controls poll
                0x00c3_dbf0,
                0x00b6_dd00,
                0x0097_8550,
                0x0086_8850, // garbage collector
                0x0066_52e0,
                0x004a_0370, // second half
                0x0087_05d0,
                0x0087_0610,
                0x005a_9d60,
            ];
            let positions: Vec<usize> = order.iter().map(|a| position(&log, *a)).collect();
            assert!(positions.windows(2).all(|w| w[0] < w[1]), "{positions:?}");
            assert_eq!(e.global::<u32>(0x011a_2fe0), 11);
            assert_eq!(e.global::<u32>(0x011d_f674), 0);
            assert_eq!(e.global::<u32>(0x011f_9138), 3);
            assert_eq!(e.global::<u8>(0x0126_77a2), 0);
            assert_eq!(e.global::<u8>(IN_MENU_FLAG), 0);
        }

        #[test]
        fn idle_update_0086e650_refreshes_the_interface_flags_twice() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(0x0070_9bc0, |_, _| 1u32.into_ret());
            e.register(0x0070_50d0, |_, _| 5u32.into_ret());
            e.register(0x0070_1450, |_, a| (a[1] + 1).into_ret());
            e.register(0x0070_3d50, |_, _| 3u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_e650, &args![main]);
            assert_eq!(e.global::<u8>(IN_MENU_FLAG), 1);
            assert_eq!(e.global::<u8>(IN_DIALOG_FLAG), 5);
            assert_eq!(e.global::<u8>(FADER_ONE_FLAG), 2);
            assert_eq!(e.global::<u8>(CONSOLE_VISIBLE_FLAG), 3);
            let log = take_log(&mut e);
            // Two refreshes, and the dialog test of the AI task step.
            assert_eq!(calls_to(&log, 0x0070_50d0).len(), 3);
            // In a menu the frame counter at 011f9138 does not move.
            assert_eq!(e.global::<u32>(0x011f_9138), 0);
        }

        #[test]
        fn interface_flags_ask_the_pipboy_only_when_the_menu_test_says_no() {
            let mut e = idle_engine();
            e.register(IS_IN_MENU_MODE, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            refresh_interface_flags(&mut e);
            assert_eq!(e.global::<u8>(IN_MENU_FLAG), 1);
            assert!(calls_to(&take_log(&mut e), 0x0070_9bc0).is_empty());
            e.register(IS_IN_MENU_MODE, |_, _| Ret::default());
            e.register(0x0070_9bc0, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            refresh_interface_flags(&mut e);
            assert_eq!(e.global::<u8>(IN_MENU_FLAG), 1);
            assert_eq!(calls_to(&take_log(&mut e), 0x0070_9bc0).len(), 1);
            e.register(0x0070_9bc0, |_, _| Ret::default());
            refresh_interface_flags(&mut e);
            assert_eq!(e.global::<u8>(IN_MENU_FLAG), 0);
        }

        #[test]
        fn idle_update_0086e650_profiles_the_pathing_when_the_flag_is_set() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            let player = e.global::<u32>(PLAYER_OBJECT);
            e.set_global(0x011d_eefc, 1u8);
            e.register(0x0000_2400, |_, _| 0x321u32.into_ret());
            let table = e.mem.u32(player);
            e.mem.set_u32(table + 0x1f4, 0x0000_2400);
            e.register(0x008d_6f30, |_, _| 0x654u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_e650, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(
                calls_to(&log, 0x006d_a7c0),
                vec![vec![0x654, 0x321, 2, 100]]
            );
        }

        #[test]
        fn idle_update_0086e650_ends_with_the_256_step_counter() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(TIMER_GET_SECONDS, |_, _| ret_f32(1.0));
            e.mem.set_f64(0x0103_57e8, 45.0);
            e.set_global(0x011d_eef5, 7u8);
            e.call_log = Some(vec![]);
            // 1.0 + 0.0 < 45: nothing happens, the seconds are kept.
            e.call(0x0086_e650, &args![main]);
            assert_eq!(e.global::<f32>(0x011d_eef8), 1.0);
            assert_eq!(e.global::<u8>(0x011d_eef5), 7);
            assert!(calls_to(&take_log(&mut e), 0x00aa_7290).is_empty());
            // Past 45 seconds the counter ticks and the byte (the old value) is passed on.
            e.set_global(0x011d_eef8, 44.5f32);
            e.call_log = Some(vec![]);
            e.call(0x0086_e650, &args![main]);
            assert_eq!(e.global::<f32>(0x011d_eef8), 0.0);
            assert_eq!(e.global::<u8>(0x011d_eef5), 8);
            assert_eq!(calls_to(&take_log(&mut e), 0x00aa_7290), vec![vec![7]]);
            // The byte wraps.
            e.set_global(0x011d_eef8, 50.0f32);
            e.set_global(0x011d_eef5, 255u8);
            e.call(0x0086_e650, &args![main]);
            assert_eq!(e.global::<u8>(0x011d_eef5), 0);
        }

        #[test]
        fn idle_update_0086e650_ticks_in_a_menu_and_when_the_player_asks() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(IS_IN_MENU_MODE, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_e650, &args![main]);
            assert_eq!(calls_to(&take_log(&mut e), 0x00aa_7290).len(), 1);
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(0x0095_0090, |_, _| 1u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_e650, &args![main]);
            assert_eq!(calls_to(&take_log(&mut e), 0x00aa_7290).len(), 1);
        }

        #[test]
        fn idle_update_0086e650_passes_the_karma_as_a_float() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            let player = e.global::<u32>(PLAYER_OBJECT);
            let table = e.mem.u32(player);
            e.mem.set_u32(table + 8, 0x0000_2500);
            e.register(0x0000_2500, |_, a| {
                assert_eq!(a[1], 0x17);
                50u32.into_ret()
            });
            let alignment = recorder(&mut e, 0x0047_e040, 0);
            e.call(0x0086_e650, &args![main]);
            assert_eq!(*alignment.borrow(), vec![vec![50.0f32.to_bits()]]);
        }

        #[test]
        fn idle_update_0086e650_world_steps_depend_on_the_processor_count() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            set_processors(&mut e, 1);
            e.call_log = Some(vec![]);
            e.call(0x0086_e650, &args![main]);
            let log = take_log(&mut e);
            // One processor: the lists are printed, the cell is tested and the AI manager is
            // updated through 008ca070 / 008ca300.
            assert_eq!(calls_to(&log, 0x008d_0600), vec![vec![PROCESS_LISTS, 0, 0]]);
            assert_eq!(calls_to(&log, 0x008c_a070), vec![vec![0x011e_0fe0]]);
            assert_eq!(calls_to(&log, 0x008c_a300), vec![vec![0x011e_0fe0]]);
            assert!(calls_to(&log, 0x008c_80e0).is_empty());
            let mut e = idle_engine();
            let main = main_object(&mut e);
            set_processors(&mut e, 4);
            e.register(0x0071_3d80, |_, _| 0x6d80u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_e650, &args![main]);
            let log = take_log(&mut e);
            assert!(calls_to(&log, 0x008d_0600).is_empty());
            assert_eq!(calls_to(&log, 0x008c_80e0), vec![vec![1], vec![0]]);
            assert_eq!(calls_to(&log, 0x008c_78c0), vec![vec![0x6d80]]);
            assert_eq!(calls_to(&log, 0x008c_7990), vec![vec![0x6d80]]);
        }

        #[test]
        fn idle_update_0086e650_sets_the_field_of_view_unless_the_mode_is_4() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(0x0071_0ab0, |_, _| ret_f32(75.0));
            e.register(0x0045_c670, |_, _| 0x6c67u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_e650, &args![main]);
            let log = take_log(&mut e);
            assert_eq!(
                calls_to(&log, 0x00c5_2020),
                vec![vec![0x6c67, 75.0f32.to_bits(), 0, 0, 1]]
            );
            assert_eq!(calls_to(&log, 0x00b5_4000), vec![vec![75.0f32.to_bits()]]);
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(0x0044_ddc0, |_, _| 4u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0086_e650, &args![main]);
            let log = take_log(&mut e);
            assert!(calls_to(&log, 0x00c5_2020).is_empty());
            assert_eq!(calls_to(&log, 0x00b5_4000).len(), 1);
        }

        #[test]
        fn idle_update_0086e650_hands_the_scene_data_to_the_image_space_calls() {
            let mut e = idle_engine();
            let main = main_object(&mut e);
            e.register(0x0066_29f0, |_, a| (a[0] + 1).into_ret());
            e.register(0x004a_0ea0, |_, _| 0x0e0au32.into_ret());
            e.set_global(IN_MENU_FLAG, 0u8);
            e.call_log = Some(vec![]);
            e.call(0x0086_e650, &args![main]);
            let log = take_log(&mut e);
            let scene = e.mem.u32(SCENE_GRAPH_HOLDER);
            // 00524c90 reads the holder; it is a double here, so only the callers are checked.
            assert_eq!(calls_to(&log, 0x00b6_dd00).len(), 1);
            assert_eq!(calls_to(&log, 0x00b6_dd00)[0][0], 0x0e0a);
            assert_eq!(calls_to(&log, 0x00b6_dd00)[0][2], 0);
            assert_eq!(calls_to(&log, 0x0066_29f0), vec![vec![scene]]);
            assert_eq!(calls_to(&log, 0x0045_b070)[0][1], scene + 1);
        }
    }
}

#[cfg(test)]
mod renderer_text_tests {
    #[test]
    fn fn_0086d580_answers_the_error_text_buffer() {
        let mut e = crate::Engine::new();
        assert_eq!(e.call(0x0086_d580, &[]).u32(), 0x011c_6fc8);
    }
}
