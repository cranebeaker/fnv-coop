//! `fallout/interface/menuconsole.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The debug console (`MenuConsole`, 0x914 bytes): two lists of strings (the
//! lines shown, `xStrings` at +4, and the commands typed, `xInputs` at +0x10),
//! the console font settings (`iConsoleFont`, `iConsoleVisibleLines`,
//! `iConsoleHistorySize`, `iConsoleTextXPos`, `iConsoleTextYPos`, all
//! `:Menu` settings) and a small batch recorder (`bRecordingBatch`, up to 20
//! recorded lines of 100 bytes at +0x40).
//!
//! `MenuConsole::Idle` handles one key message with the game's own unoptimised
//! code, which keeps the edited command line and its cursor (a `|` inside the
//! text) in a 0x800-byte stack buffer. The translation keeps that buffer and
//! the other address-taken locals in one block of game memory laid out at the
//! same distances from the frame pointer as the original stack frame
//! (`Frame`), so the offsets in the code can be checked against the
//! disassembly.
//!
//! The compiler's exception-unwinding frames and security cookie are not
//! translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// Allocator (`operator new` replacement): returns a block of the given size.
const ALLOCATE: u32 = 0x0040_1000;
/// `BSStringT` constructor (empty string) / destructor.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `memset(destination, value, count)`.
const MEMSET: u32 = 0x0040_3d30;
/// Copies a C string into a fixed-size record (`destination, source`).
const STRING_COPY: u32 = 0x0040_46f0;
/// Length of a `BSStringT` (its `sLen`).
const STRING_LENGTH: u32 = 0x0040_48e0;
/// `snprintf(buffer, size, format, ...)` (cdecl).
const SNPRINTF: u32 = 0x0040_6d00;
/// `strcpy_s`-style copy (`destination, size, source`).
const STRING_COPY_BOUNDED: u32 = 0x0040_6d30;
/// `strcat_s`-style append (`destination, size, source`).
const STRING_APPEND_BOUNDED: u32 = 0x0040_6d50;
/// `BSStringT::Format(format, ...)` with a `va_list` (`this`, format, list).
const STRING_FORMAT_LIST: u32 = 0x0040_6f90;
/// `BSStringT::Format(format, ...)` (cdecl, `this` first).
const STRING_FORMAT: u32 = 0x0040_6f60;
/// `BSStringT` constructor from a C string.
const STRING_FROM_C_STRING: u32 = 0x0040_c0e0;
/// `BSStringT::operator=(const char *)`.
const STRING_ASSIGN: u32 = 0x0043_8390;
/// Returns the pointer to the `int` value of the `Setting` in ECX.
const SETTING_INT_POINTER: u32 = 0x0043_d4d0;
/// Count of the list in ECX (the list's `m_uiCount` at +8).
const LIST_COUNT: u32 = 0x0044_ddc0;
/// `strlen`.
const STRING_LENGTH_C: u32 = 0x0044_a670;
/// `Interface` singleton getter (`[0x011d8a80]`).
const INTERFACE_GETTER: u32 = 0x004b_7210;
/// The first member of a list or string (`*this`): the head node of a list,
/// the character pointer of a `BSStringT`.
const FIRST_MEMBER: u32 = 0x0055_9450;
/// `NiTListItem` data address (node + 8).
const LIST_NODE_DATA: u32 = 0x0063_17a0;
/// Next node of a list (`this`, node).
const LIST_NEXT: u32 = 0x007b_52d0;
/// The array of `Font` pointers (`[0x011f33f8]`) and its element/float getters.
const FONT_ARRAY_GETTER: u32 = 0x005b_d5b0;
const FONT_ARRAY_ELEMENT: u32 = 0x005b_d5c0;
const FONT_HEIGHT: u32 = 0x005b_d580;
/// `_ftol2_sse`: truncates the float in ST0 (passed as an `f64`).
const FTOL: u32 = 0x00ec_62c0;
/// `DebugText::Instance(1)` and `DebugText::Print` (8 words).
const DEBUG_TEXT_INSTANCE: u32 = 0x00a0_d9e0;
const DEBUG_TEXT_PRINT: u32 = 0x00a0_f8b0;
/// `DebugText::DeleteAllDebugText`.
const DEBUG_TEXT_DELETE_ALL: u32 = 0x00a0_de80;
/// `BSFile` constructor (`this`, path, mode, buffer size, ...).
const FILE_CONSTRUCT: u32 = 0x00b0_0260;
/// `BSFile::Write` (`this`, buffer, length).
const FILE_WRITE: u32 = 0x0047_3180;
/// `BSFile::Close`.
const FILE_CLOSE: u32 = 0x00af_fd10;
/// Writes a line to the console through the console singleton (cdecl:
/// format, arguments).
const PRINT_TO_CONSOLE: u32 = 0x0070_3c00;

/// `iConsoleFont:Menu`, `iConsoleVisibleLines:Menu`,
/// `iConsoleHistorySize:Menu`, `iConsoleTextXPos:Menu` and
/// `iConsoleTextYPos:Menu` (`Setting` objects in `.data`).
const CONSOLE_FONT: u32 = 0x011d_8d38;
const CONSOLE_VISIBLE_LINES: u32 = 0x011d_8cfc;
const CONSOLE_HISTORY_SIZE: u32 = 0x011d_8cf0;
const CONSOLE_TEXT_X: u32 = 0x011d_8d44;
const CONSOLE_TEXT_Y: u32 = 0x011d_8d08;
/// `MenuConsole::pInstance`.
const CONSOLE_INSTANCE: u32 = 0x011d_8ce8;
/// A flag byte the console sets when a command line has been entered.
const COMMAND_ENTERED: u32 = 0x011d_8ce4;
/// The float the console passes to `DebugText::Print` as its 6th argument.
const PRINT_FLOAT_ARGUMENT: u32 = 0x0101_2054;
/// `""`.
const EMPTY_STRING: u32 = 0x0101_1584;
/// `"|"`: the text cursor.
const CURSOR_TEXT: u32 = 0x0106_fd64;

layout! {
    /// `MenuConsole` (Xbox PDB), 0x914 bytes (same on PC).
    pub struct MenuConsole: 0x914 {
        /// `pConsoleCompiler` (Xbox PDB): `ScriptCompiler*`.
        0x00 pConsoleCompiler: Ptr,
        /// `iInputsPos` (Xbox PDB): position in the command history.
        0x1C iInputsPos: i32,
        /// `iLastString` (Xbox PDB): the index of the last line shown.
        0x24 iLastString: i32,
        /// `iConsoleLineSpacing` (Xbox PDB).
        0x2C iConsoleLineSpacing: i32,
        /// `iConsoleXPos` (Xbox PDB).
        0x30 iConsoleXPos: i32,
        /// `iConsoleYPos` (Xbox PDB).
        0x34 iConsoleYPos: i32,
        /// `cVisible` (Xbox PDB): 0 hidden, 1 visible, 2 visible while a
        /// menu that takes the keyboard is open.
        0x38 cVisible: i8,
        /// `bIsActive` (Xbox PDB).
        0x39 bIsActive: bool,
        /// `bRecordingBatch` (Xbox PDB).
        0x3A bRecordingBatch: bool,
        /// `iRecordingLineCount` (Xbox PDB).
        0x3C iRecordingLineCount: i32,
        /// `cRecording` (Xbox PDB): 20 lines of 100 bytes.
        0x40 cRecording: u8,
    }
}

/// The lists inside the console: `xStrings` (the lines shown) at +4 and
/// `xInputs` (the commands typed) at +0x10; both are 0xC-byte lists.
const STRINGS_LIST: u32 = 0x04;
const INPUTS_LIST: u32 = 0x10;
/// Number of recorded lines the batch recorder holds, and their size.
const RECORDING_LINES: i32 = 20;
const RECORDING_LINE_SIZE: u32 = 100;

/// Value of an integer `Setting`.
fn setting_int(e: &mut Engine, setting: u32) -> i32 {
    let value = e.call(SETTING_INT_POINTER, &args![setting]).u32();
    e.mem.i32(value)
}

/// `DebugText::Instance(1)->Print(text, x, y, 1, -1, <float>, font, 0)`:
/// writes a line of text at the console's position.
fn debug_text_print(e: &mut Engine, text: u32, x: f32, y: f32, font: u32) {
    let instance = e.call(DEBUG_TEXT_INSTANCE, &args![1u32]).u32();
    let argument: f32 = e.global(PRINT_FLOAT_ARGUMENT);
    e.call(
        DEBUG_TEXT_PRINT,
        &args![instance, text, x, y, 1u32, -1i32, argument, font, 0u32],
    );
}

/// The console's position as the two floats `Print` takes.
fn console_position(e: &Engine, this: Ptr<MenuConsole>) -> (f32, f32) {
    (
        e.get(this, MenuConsole::iConsoleXPos) as f32,
        e.get(this, MenuConsole::iConsoleYPos) as f32,
    )
}

// Translated from 0071af10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuConsole::MenuConsole`: constructs the two string lists, clears the
/// state and reads the position and font metrics from the console settings
/// (line spacing is the console font's height plus 3). Returns `this`.
pub fn fn_0071af10(e: &mut Engine, this: Ptr<MenuConsole>) -> Ptr<MenuConsole> {
    e.call(0x0048_f200, &args![this.addr() + STRINGS_LIST]);
    e.call(0x0048_f200, &args![this.addr() + INPUTS_LIST]);
    e.set(this, MenuConsole::bIsActive, false);
    e.set(this, MenuConsole::pConsoleCompiler, Ptr::NULL);
    e.set(this, MenuConsole::iLastString, 0);
    e.set(this, MenuConsole::cVisible, 0);
    e.set(this, MenuConsole::iInputsPos, 0);
    let font = setting_int(e, CONSOLE_FONT);
    let fonts = e.call(FONT_ARRAY_GETTER, &args![]).u32();
    let font_object = e.call(FONT_ARRAY_ELEMENT, &args![fonts, font - 1]).u32();
    let height = e.call(FONT_HEIGHT, &args![font_object]).f32();
    let height = e.call(FTOL, &args![height as f64]).i32();
    e.set(
        this,
        MenuConsole::iConsoleLineSpacing,
        height.wrapping_add(3),
    );
    let x = setting_int(e, CONSOLE_TEXT_X);
    e.set(this, MenuConsole::iConsoleXPos, x);
    let y = setting_int(e, CONSOLE_TEXT_Y);
    e.set(this, MenuConsole::iConsoleYPos, y);
    e.set(this, MenuConsole::bRecordingBatch, false);
    e.set(this, MenuConsole::iRecordingLineCount, 0);
    // The output file name (+0x810, 260 bytes) is cleared.
    e.call(MEMSET, &args![this.addr() + 0x810, 0u32, 0x104u32]);
    this
}

// Translated from 0071b0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuConsole::Init` (Xbox PDB): creates the console's `ScriptCompiler`.
pub fn menu_console_init(e: &mut Engine, this: Ptr<MenuConsole>) {
    let memory = e.call(ALLOCATE, &args![1u32]).u32();
    let compiler = if memory != 0 {
        e.call(0x005a_ea50, &args![memory]).u32()
    } else {
        0
    };
    e.set(this, MenuConsole::pConsoleCompiler, Ptr::new(compiler));
}

// Translated from 0071b160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuConsole::Instance` (Xbox PDB): the console singleton. With `create`
/// set and no console yet, builds one and initialises it.
pub fn menu_console_instance(e: &mut Engine, create: u8) -> Ptr<MenuConsole> {
    let existing: u32 = e.global(CONSOLE_INSTANCE);
    if existing != 0 || create == 0 {
        return Ptr::new(e.global::<u32>(CONSOLE_INSTANCE));
    }
    let memory = e.call(ALLOCATE, &args![0x914u32]).u32();
    let console = if memory != 0 {
        fn_0071af10(e, Ptr::new(memory)).addr()
    } else {
        0
    };
    e.set_global(CONSOLE_INSTANCE, console);
    menu_console_init(e, Ptr::new(console));
    Ptr::new(e.global::<u32>(CONSOLE_INSTANCE))
}

// Translated from 0071d010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte at `this + 0x20`. The "new fonts" / "old fonts" commands
/// call it on the font array global (`[0x011f33f8]`) with 1 / 0.
pub fn fn_0071d010(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x20, value);
}

// Translated from 0071d030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuConsole::PrintLine` (Xbox PDB): `printf`-style line written to the
/// console; `arguments` are the words after the format string. Cdecl,
/// `this` is the first stack word.
///
/// The temporary `BSStringT` the original constructs and destroys is never
/// used and is kept for its calls.
pub fn menu_console_print_line(
    e: &mut Engine,
    this: Ptr<MenuConsole>,
    format: u32,
    arguments: &[u32],
) {
    // The exe's frame (`SUB ESP,0xc` under the exception record, then the record link):
    // the unused string at `EBP-0x14` (+8). The `va_list` is the argument words.
    e.with_stack(0x1c, |e, frame| {
        let scratch = frame.addr() + 0x8;
        e.call(STRING_CONSTRUCT, &args![scratch]);
        // The `va_list` is the address of the first argument word.
        let list = e.mem.alloc(4 * arguments.len().max(1) as u32);
        for (i, word) in arguments.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, *word);
        }
        fn_0071d0a0(e, this, format, list);
        e.mem.free(list);
        e.call(STRING_DESTRUCT, &args![scratch]);
    });
}

/// Registry form of [`menu_console_print_line`]: words are `this`, the
/// format, then the arguments.
fn print_line_entry(e: &mut Engine, words: &[u32]) -> Ret {
    menu_console_print_line(e, Ptr::new(words[0]), words[1], &words[2..]);
    Ret::default()
}

// Translated from 0071d0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Formats `format` with the `va_list` at `arguments`, wraps the text with
/// the console font to the console width and appends each wrapped line to
/// the list of lines shown (dropping the oldest lines above
/// `iConsoleHistorySize`). Redraws when the view was following the last line.
///
/// It does nothing while the console singleton check `00705ea0` holds and
/// the `0070edf0` check fails.
pub fn fn_0071d0a0(e: &mut Engine, this: Ptr<MenuConsole>, format: u32, arguments: u32) {
    if e.call(0x0070_5ea0, &args![]).bool() && !e.call(0x0070_edf0, &args![]).bool() {
        return;
    }
    // The exe's frame (`SUB ESP,0x840` under the exception record, then `ESI` and the record
    // link): the text at `EBP-0x1c` (+0x838), the width at `EBP-0x828` (+0x2c), the unused word
    // at `EBP-0x10` (+0x844), the line buffer at `EBP-0x824` (+0x30, 0x808 bytes up to the
    // text; the copy is not bounded), the line at `EBP-0x83c` (+0x18), the removed line at
    // `EBP-0x848` (+0xc).
    e.with_stack(0x854, |e, frame| {
        let text = frame.addr() + 0x838;
        e.call(STRING_CONSTRUCT, &args![text]);
        e.call(STRING_FORMAT_LIST, &args![text, format, arguments]);
        let c_text = e.call(FIRST_MEMBER, &args![text]).u32();
        e.call(0x0071_dd90, &args![this, c_text]);
        let size = frame.addr() + 0x2c;
        e.mem.set_u32(size, 0x4b0);
        let unused = frame.addr() + 0x844;
        let font = setting_int(e, CONSOLE_FONT);
        let fonts = e.call(FONT_ARRAY_GETTER, &args![]).u32();
        let font_object = e.call(FONT_ARRAY_ELEMENT, &args![fonts, font - 1]).u32();
        // `Font::LineBreakText` (Xbox PDB) breaks the text in place at the
        // width and returns the number of lines.
        let lines = e
            .call(
                0x00a1_60c0,
                &args![font_object, text, size, unused, 0u32, 0u32],
            )
            .i32();
        if lines == 0 {
            e.call(STRING_DESTRUCT, &args![text]);
            return;
        }
        // The wrapped text, with a NUL after each line, is copied into the stack
        // buffer (the exe does not bound the copy).
        let length = e.call(STRING_LENGTH, &args![text]).u32();
        let buffer = frame.addr() + 0x30;
        for i in 0..=length {
            let c_text = e.call(FIRST_MEMBER, &args![text]).u32();
            let byte = e.mem.u8(c_text + i);
            e.mem.set_u8(buffer + i, byte);
        }
        let mut cursor = buffer;
        let strings = this.addr() + STRINGS_LIST;
        for index in 0..lines {
            let mut was_last = false;
            let count = e.call(LIST_COUNT, &args![strings]).i32();
            if e.get(this, MenuConsole::iLastString) == count {
                was_last = true;
            }
            let line = frame.addr() + 0x18;
            e.call(STRING_CONSTRUCT, &args![line]);
            e.call(STRING_ASSIGN, &args![line, cursor]);
            // Appends the line to the list (`NiTList::AddTail`).
            e.call(0x0071_df60, &args![strings, line]);
            if was_last {
                let count = e.call(LIST_COUNT, &args![strings]).i32();
                e.set(this, MenuConsole::iLastString, count);
            }
            loop {
                let count = e.call(LIST_COUNT, &args![strings]).i32();
                let limit = setting_int(e, CONSOLE_HISTORY_SIZE);
                if count <= limit {
                    break;
                }
                // Removes the first line into a temporary string.
                let removed = frame.addr() + 0xc;
                e.call(0x0071_dfa0, &args![strings, removed]);
                e.call(STRING_DESTRUCT, &args![removed]);
                let last = e.get(this, MenuConsole::iLastString);
                e.set(this, MenuConsole::iLastString, last - 1);
            }
            if index + 1 < lines {
                // On to the next NUL-separated line.
                while e.mem.i8(cursor) != 0 {
                    cursor += 1;
                }
                cursor += 1;
            }
            e.call(STRING_DESTRUCT, &args![line]);
        }
        let count = e.call(LIST_COUNT, &args![strings]).i32();
        if e.get(this, MenuConsole::iLastString) == count {
            menu_console_refresh_lines(e, this);
        }
        e.call(STRING_DESTRUCT, &args![text]);
    });
}

// Translated from 0071d380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the last line shown by `delta`: the result is kept between
/// `iConsoleVisibleLines` and the number of lines.
pub fn fn_0071d380(e: &mut Engine, this: Ptr<MenuConsole>, delta: i32) {
    let count = e.call(LIST_COUNT, &args![this.addr() + STRINGS_LIST]).i32();
    let mut target = e.get(this, MenuConsole::iLastString).wrapping_add(delta);
    if target > count {
        target = count;
    }
    let visible = setting_int(e, CONSOLE_VISIBLE_LINES);
    if target - visible <= 0 {
        target = visible;
    }
    if target > count {
        target = count;
    }
    e.set(this, MenuConsole::iLastString, target);
}

// Translated from 0071d410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuConsole::RefreshLines` (Xbox PDB): redraws the visible lines. Lines
/// above the first existing one are blanked, then the lines up to
/// `iLastString` are drawn from the list, each one `iConsoleLineSpacing`
/// lower than the one before, ending at `iConsoleYPos`.
pub fn menu_console_refresh_lines(e: &mut Engine, this: Ptr<MenuConsole>) {
    let strings = this.addr() + STRINGS_LIST;
    let mut node = e.call(FIRST_MEMBER, &args![strings]).u32();
    let visible = setting_int(e, CONSOLE_VISIBLE_LINES);
    let spacing = e.get(this, MenuConsole::iConsoleLineSpacing);
    let mut y = e.get(this, MenuConsole::iConsoleYPos) - visible * spacing;
    let last = e.get(this, MenuConsole::iLastString);
    let mut index = last - setting_int(e, CONSOLE_VISIBLE_LINES);
    while index < 0 {
        let (x, _) = console_position(e, this);
        debug_text_print(e, 0, x, y as f32, 0);
        index += 1;
        y += e.get(this, MenuConsole::iConsoleLineSpacing);
    }
    let mut skipped = 0;
    while skipped < index {
        node = e.call(LIST_NEXT, &args![strings, node]).u32();
        skipped += 1;
    }
    while index < last {
        let font = setting_int(e, CONSOLE_FONT);
        let data = e.call(LIST_NODE_DATA, &args![strings, node]).u32();
        let line = e.call(FIRST_MEMBER, &args![data]).u32();
        let (x, _) = console_position(e, this);
        debug_text_print(e, line, x, y as f32, font as u32);
        node = e.call(LIST_NEXT, &args![strings, node]).u32();
        index += 1;
        y += e.get(this, MenuConsole::iConsoleLineSpacing);
    }
}

// Translated from 0071d580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuConsole::ToggleVisible` (Xbox PDB): hidden -> visible (returns 1).
/// Visible while the interface's `0070ecf0` test holds and the top two
/// entries of its menu stack are 3 and 0 -> state 2; every other case hides
/// the console. Returns 0 unless it was shown.
pub fn menu_console_toggle_visible(e: &mut Engine, this: Ptr<MenuConsole>) -> u8 {
    e.call(0x00ba_8b70, &args![]);
    let state = e.get(this, MenuConsole::cVisible);
    if state == 0 {
        fn_0071d620(e, this, 1);
        return 1;
    }
    if state == 1 {
        let interface = e.call(INTERFACE_GETTER, &args![]).u32();
        if e.call(0x0070_ecf0, &args![interface]).bool() {
            let interface = e.call(INTERFACE_GETTER, &args![]).u32();
            if e.call(0x0071_4f70, &args![interface, 0u32]).u32() == 3 {
                let interface = e.call(INTERFACE_GETTER, &args![]).u32();
                if e.call(0x0071_4f70, &args![interface, 1u32]).u32() == 0 {
                    fn_0071d620(e, this, 2);
                    return 0;
                }
            }
            fn_0071d620(e, this, 0);
            return 0;
        }
    }
    fn_0071d620(e, this, 0);
    0
}

// Translated from 0071d620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the visibility state. When it changes between hidden and shown:
/// showing draws the cursor (or a blank line when the interface's
/// `007023a0` test fails), scrolls to the last line and redraws; hiding
/// deletes all debug text and clears the interface's field at +0xF0.
pub fn fn_0071d620(e: &mut Engine, this: Ptr<MenuConsole>, state: i8) {
    let old = e.get(this, MenuConsole::cVisible);
    e.set(this, MenuConsole::cVisible, state);
    if (state > 0) == (old > 0) {
        return;
    }
    if state > 0 {
        e.call(0x0040_fbe0, &args![]);
        let interface = e.call(INTERFACE_GETTER, &args![]).u32();
        let (x, y) = console_position(e, this);
        if e.call(0x0070_23a0, &args![interface]).bool() {
            let font = setting_int(e, CONSOLE_FONT);
            debug_text_print(e, CURSOR_TEXT, x, y, font as u32);
            let count = e.call(LIST_COUNT, &args![this.addr() + STRINGS_LIST]).i32();
            fn_0071d380(e, this, count);
        } else {
            debug_text_print(e, EMPTY_STRING, x, y, 0);
        }
        menu_console_refresh_lines(e, this);
    } else {
        e.call(0x0040_fbe0, &args![]);
        let instance = e.call(DEBUG_TEXT_INSTANCE, &args![1u32]).u32();
        e.call(DEBUG_TEXT_DELETE_ALL, &args![instance]);
        let interface = e.call(INTERFACE_GETTER, &args![]).u32();
        fn_0071d750(e, Ptr::new(interface));
    }
}

// Translated from 0071d750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the dword at +0xF0 of the `Interface` (set when the console is
/// hidden).
pub fn fn_0071d750(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0xf0, 0);
}

// Translated from 0071d770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuConsole::OnEnterMenuMode` (Xbox PDB): with the console visible,
/// draws the cursor, scrolls to the last line, redraws and calls
/// `00703310(0)`.
pub fn menu_console_on_enter_menu_mode(e: &mut Engine, this: Ptr<MenuConsole>) {
    if e.get(this, MenuConsole::cVisible) > 0 {
        let font = setting_int(e, CONSOLE_FONT);
        let (x, y) = console_position(e, this);
        debug_text_print(e, CURSOR_TEXT, x, y, font as u32);
        let count = e.call(LIST_COUNT, &args![this.addr() + STRINGS_LIST]).i32();
        fn_0071d380(e, this, count);
        menu_console_refresh_lines(e, this);
        e.call(0x0070_3310, &args![0u32]);
    }
}

/// The `TES` singleton pointer variable (ECX of `00457070`, which returns the
/// current cell).
const TES_SINGLETON: u32 = 0x011d_ea10;
/// `"{unknown}"`.
const UNKNOWN_NAME: u32 = 0x0101_20d8;
/// The `Setting` byte (`00408d60` returns its pointer) that makes
/// `dump textures` list menu textures too.
const DUMP_MENU_TEXTURES: u32 = 0x011d_ec98;
const TEXTURES_FILE_NAME: u32 = 0x0106_fe78;
const TEXTURES_FILE_FAILED: u32 = 0x0106_fe4c;
const TEXTURES_HEADER_FORMAT: u32 = 0x0106_fde8;
const TEXTURES_FOOTER_FORMAT: u32 = 0x0106_fda8;
const TEXTURES_DUMPED_MESSAGE: u32 = 0x0106_fd68;
/// Run-time type descriptor of the texture class `00456510`'s items are
/// checked against (`0043b300(descriptor, item)`).
const TEXTURE_TYPE: u32 = 0x011f_444c;

// Translated from 0071d800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuConsole::DumpTextures` (Xbox PDB): collects up to 200 loaded
/// textures from the texture list (the ones `0071dca0` does not reject,
/// unless the `DUMP_MENU_TEXTURES` setting is on) and writes a header with
/// the current cell, grid and worldspace to `textures.txt`, then reports
/// the result on the console.
///
/// Faithful to the code: the position search for the slot calls `0059bb30`
/// on the occupied slots without comparing anything, so the first empty slot
/// is always the one taken (the list is not sorted); the per-texture loop
/// rewrites the header buffer instead of formatting a line; and the total
/// memory reported is always 0.
pub fn menu_console_dump_textures(e: &mut Engine, this: Ptr<MenuConsole>) {
    // The exe's frame (`SUB ESP,0x770` under the exception record, then the record link):
    // the texture slots at `EBP-0x32c` (+0x454), the text buffer at `EBP-0x754` (+0x2c).
    e.with_stack(0x780, |e, frame| {
        let slots = frame.addr() + 0x454;
        e.call(MEMSET, &args![slots, 0u32, 0x320u32]);
        let mut node = e.call(0x0045_6510, &args![]).u32();
        while node != 0 {
            if e.call(0x0043_b300, &args![TEXTURE_TYPE, node]).bool() {
                let texture = node;
                let size = e.call(0x0059_bb30, &args![texture]).u32();
                if size != 0 {
                    let flag = e.call(0x0040_8d60, &args![DUMP_MENU_TEXTURES]).u32();
                    let include_menu_textures = e.mem.u8(flag) != 0;
                    if include_menu_textures || !e.call(0x0071_dca0, &args![this, texture]).bool() {
                        let mut position = -1i32;
                        let mut i = 0i32;
                        while i < 200 && position == -1 {
                            let slot = e.mem.u32(slots + 4 * i as u32);
                            if slot == 0 {
                                position = i;
                            } else {
                                e.call(0x0059_bb30, &args![slot]);
                            }
                            i += 1;
                        }
                        if position != -1 {
                            let mut j = 199i32;
                            while position < j {
                                let moved = e.mem.u32(slots + 4 * (j - 1) as u32);
                                e.mem.set_u32(slots + 4 * j as u32, moved);
                                j -= 1;
                            }
                            e.mem.set_u32(slots + 4 * position as u32, texture);
                        }
                    }
                }
            }
            node = e.call(0x0055_b980, &args![node]).u32();
        }
        let memory = e.call(ALLOCATE, &args![0x158u32]).u32();
        let file = if memory != 0 {
            e.call(
                FILE_CONSTRUCT,
                &args![memory, TEXTURES_FILE_NAME, 1u32, 0x1000u32, 1u32],
            )
            .u32()
        } else {
            0
        };
        if file == 0 || !e.vcall(file, 0x20, &args![0u32, 0u32]).bool() {
            e.call(PRINT_TO_CONSOLE, &args![TEXTURES_FILE_FAILED]);
            return;
        }
        let total_memory = 0u32;
        let tes: u32 = e.global(TES_SINGLETON);
        // The worldspace name of the current cell, or "{unknown}".
        let world_name = if e.call(0x0045_7070, &args![tes]).u32() != 0 {
            let cell = e.call(0x0045_7070, &args![tes]).u32();
            if e.call(0x0054_ddd0, &args![cell]).u32() != 0 {
                let cell = e.call(0x0045_7070, &args![tes]).u32();
                let world = e.call(0x0054_ddd0, &args![cell]).u32();
                e.call(0x0040_8da0, &args![world + 0x18]).u32()
            } else {
                UNKNOWN_NAME
            }
        } else {
            UNKNOWN_NAME
        };
        // The name of the current cell, or "{unknown}".
        let cell_name = if e.call(0x0045_7070, &args![tes]).u32() != 0 {
            let cell = e.call(0x0045_7070, &args![tes]).u32();
            e.call(0x0040_8da0, &args![cell + 0x18]).u32()
        } else {
            UNKNOWN_NAME
        };
        let grid_y = e.call(0x0045_cd60, &args![tes]).u32();
        let grid_x = e.call(0x0059_bb30, &args![tes]).u32();
        let buffer = frame.addr() + 0x2c;
        e.call(
            SNPRINTF,
            &args![
                buffer,
                0x400u32,
                TEXTURES_HEADER_FORMAT,
                cell_name,
                grid_x,
                grid_y,
                world_name
            ],
        );
        let length = e.call(STRING_LENGTH_C, &args![buffer]).u32();
        e.call(FILE_WRITE, &args![file, buffer, length]);
        let mut i = 0u32;
        while i < 200 {
            let texture = e.mem.u32(slots + 4 * i);
            if texture == 0 {
                break;
            }
            e.call(0x0059_bb30, &args![texture]);
            let name = e.vcall(texture, 0x9c, &args![]).u32();
            e.call(0x0043_b1b0, &args![name]);
            let length = e.call(STRING_LENGTH_C, &args![buffer]).u32();
            e.call(FILE_WRITE, &args![file, buffer, length]);
            i += 1;
        }
        e.call(
            SNPRINTF,
            &args![buffer, 0x400u32, TEXTURES_FOOTER_FORMAT, total_memory],
        );
        let length = e.call(STRING_LENGTH_C, &args![buffer]).u32();
        e.call(FILE_WRITE, &args![file, buffer, length]);
        e.call(FILE_CLOSE, &args![file]);
        e.call(PRINT_TO_CONSOLE, &args![TEXTURES_DUMPED_MESSAGE]);
    });
}

// ---------------------------------------------------------------------------
// MenuConsole::Idle
// ---------------------------------------------------------------------------

/// Compares the first `count` characters of the command line with a command
/// name (two variants of the game's helper; 0 means they match).
const COMMAND_MATCHES_A: u32 = 0x0071_cff0;
const COMMAND_MATCHES_B: u32 = 0x0071_cfd0;
/// Whole-string comparisons used for `clr`, `cls` and `tag`.
const STRING_MATCHES_A: u32 = 0x0046_fd30;
const STRING_MATCHES_B: u32 = 0x0046_9880;
/// CRT `sscanf` (cdecl) and `sprintf`.
const SSCANF: u32 = 0x00ec_a4a6;
const SPRINTF: u32 = 0x00ec_623a;
/// CRT `strlen`.
const CRT_STRLEN: u32 = 0x00ec_6130;
/// `memset(destination, value, count)` of the CRT.
const CRT_MEMSET: u32 = 0x00ec_61c0;
/// Bounded copy used to take the typed text (`destination, source, count`).
const COPY_TEXT: u32 = 0x005c_bd90;
/// `DebugText::GetText`-style reader of the typed text (`this`, buffer, x, y,
/// 1); the buffer receives the text the user has typed so far.
const DEBUG_TEXT_READ: u32 = 0x00a1_03e0;
/// `__RTDynamicCast(object, offset, source type, target type, is reference)`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;

/// Messages `Idle` receives for keys it handles itself; any other value is a
/// character to insert. Named by what the code does with them.
const MESSAGE_DELETE_BEFORE_CURSOR: u32 = 0x8000_0000;
const MESSAGE_CURSOR_LEFT: u32 = 0x8000_0001;
const MESSAGE_CURSOR_RIGHT: u32 = 0x8000_0002;
const MESSAGE_HISTORY_OLDER: u32 = 0x8000_0003;
const MESSAGE_HISTORY_NEWER: u32 = 0x8000_0004;
const MESSAGE_CURSOR_TO_START: u32 = 0x8000_0005;
const MESSAGE_CURSOR_TO_END: u32 = 0x8000_0006;
const MESSAGE_DELETE_AT_CURSOR: u32 = 0x8000_0007;
const MESSAGE_ENTER_LINE: u32 = 0x8000_0008;
const MESSAGE_SCROLL_BACK: u32 = 0x8000_0009;
const MESSAGE_SCROLL_FORWARD: u32 = 0x8000_000a;

/// Text cursor character inside the command line.
const CURSOR: u8 = b'|';

// Command names, formats and messages in the exe's data.
const DOF_COMMAND: u32 = 0x0106_fd60;
const DOF_FORMAT: u32 = 0x0106_fd58;
const USZ_COMMAND: u32 = 0x0106_fd54;
const USZ_FORMAT: u32 = 0x0106_fd48;
const CLR_COMMAND: u32 = 0x0106_fd44;
const CLS_COMMAND: u32 = 0x0106_fd40;
const TAG_COMMAND: u32 = 0x0106_fd3c;
const RELOAD_STRINGS_COMMAND: u32 = 0x0106_fd2c;
const RELOAD_HUD_RETICLE_COMMAND: u32 = 0x0106_fd18;
const RELOAD_HUD_SAFE_ZONE_COMMAND: u32 = 0x0106_fd04;
const RELOAD_MAIN_MENU_COMMAND: u32 = 0x0106_fcf4;
const RELOAD_COMMAND: u32 = 0x0106_fcec;
const RELOAD_FORMAT: u32 = 0x0106_fce0;
const DELETE_COMMAND: u32 = 0x0106_fcd8;
const DELETE_FORMAT: u32 = 0x0106_fccc;
const FRONT_COMMAND: u32 = 0x0106_fcc4;
const FRONTMOST_FORMAT: u32 = 0x0106_fcb4;
const STACK_COMMAND: u32 = 0x0106_fcac;
const STACK_ENTRY_FORMAT: u32 = 0x0106_fca4;
const VISIBLE_COMMAND: u32 = 0x0106_fc9c;
const VISIBLE_SCAN_FORMAT: u32 = 0x0106_fc90;
const VISIBLE_REPORT_FORMAT: u32 = 0x0106_fc80;
const EXIT_MENUS_COMMAND: u32 = 0x0106_fc74;
const EMERGENCY_TEXTURE_RELEASE_COMMAND: u32 = 0x0106_fc58;
const TEXTURE_RELEASE_MESSAGE: u32 = 0x0106_fc24;
const SET_TRAIT_COMMAND: u32 = 0x0106_fc18;
const SET_TRAIT_FORMAT: u32 = 0x0106_fc04;
const TRAIT_SET_MESSAGE: u32 = 0x0106_fbf8;
const UNKNOWN_TRAIT_FORMAT: u32 = 0x0106_fbe4;
const TILE_NOT_FOUND_FORMAT: u32 = 0x0106_fbcc;
const MAX_DEPTH_FORMAT: u32 = 0x0106_fbbc;
const DEPTH_COMMAND: u32 = 0x0103_c034;
const CURSOR_DEPTH_FORMAT: u32 = 0x0106_fba8;
const NEW_FONTS_COMMAND: u32 = 0x0106_fb9c;
const OLD_FONTS_COMMAND: u32 = 0x0106_fb90;
const ISO_COMMAND: u32 = 0x0106_fb8c;
const RESTORE_MENUS_COMMAND: u32 = 0x0106_fb7c;
const USE_WORKBOOK_COMMAND: u32 = 0x0106_fb6c;
const WORKBOOK_NEEDS_BOOK_MESSAGE: u32 = 0x0106_fb30;
const DUMP_TEXTURES_COMMAND: u32 = 0x0106_fb20;
const BAT_COMMAND: u32 = 0x0106_fb1c;
const BAT_FORMAT: u32 = 0x0106_fb14;
const FILENAME_REQUIRED_MESSAGE: u32 = 0x0106_faf8;
const TXT_EXTENSION: u32 = 0x0103_9788;
const FILE_NOT_OPENED_FORMAT: u32 = 0x0106_faa8;
const NEITHER_FILE_OPENED_FORMAT: u32 = 0x0106_facc;
const SCRIPT_ECHO_FORMAT: u32 = 0x0106_faa0;
const RBAT_COMMAND: u32 = 0x0106_fa98;
const RBAT_FORMAT: u32 = 0x0106_fa28;
const RECORDING_STARTED_MESSAGE: u32 = 0x0106_fa7c;
const RECORDING_HELP_MESSAGE: u32 = 0x0106_fa30;
const RECORDING_NO_FILENAME_MESSAGE: u32 = 0x0106_f9fc;
const RECORDING_EMPTY_MESSAGE: u32 = 0x0106_f9d0;
const RECORDED_LINE_FORMAT: u32 = 0x0106_f9c8;
const BATCH_CREATED_FORMAT: u32 = 0x0106_f9ac;
const RECORDED_MESSAGE: u32 = 0x0106_f9a0;
const RECORDING_FULL_MESSAGE: u32 = 0x0106_f950;
const GLOBALS_XML: u32 = 0x0106_eff8;
const STRING_FORMAT_S: u32 = 0x0101_9f08;
const TAG_FORMAT: u32 = 0x0103_a1c8;

// Run-time type descriptors for the `use workbook` casts.
const TYPE_TILE_MENU_BASE: u32 = 0x0119_f4cc;
const TYPE_TILE_MENU: u32 = 0x0119_f49c;
const TYPE_MENU: u32 = 0x0119_f34c;
const TYPE_BOOK_MENU: u32 = 0x0119_f400;
/// A global pointer (zero in the exe's data) that `tag` passes to `00653270`.
const TAG_LOOKUP_TABLE: u32 = 0x011f_4428;

/// Stack frame of `Idle`: the original keeps these locals at fixed distances
/// below its frame pointer. The values are those distances.
mod frame {
    /// Bytes of the exe's frame (`_chkstk` of 0x28c8 under the exception record, then
    /// `ESI`, `EDI` and the record link) and where `EBP` sits in them: at its top.
    pub const SIZE: u32 = 0x28e0;
    pub const POINTER: u32 = 0x28e0;
    /// The text read back from the debug text (0x808 bytes).
    pub const TYPED_TEXT: i32 = -0x1024;
    /// The command line being edited (0x800 bytes), with its cursor.
    pub const LINE: i32 = -0x81c;
    /// `BSStringT` of the entered line added to the history.
    pub const ENTERED_LINE: i32 = -0x1038;
    /// `dof`: the float read and the 0x1c-byte parameter block.
    pub const DOF_VALUE: i32 = -0x105c;
    pub const DOF_BLOCK: i32 = -0x1058;
    /// `usz`: the two integers read.
    pub const USZ_WIDTH: i32 = -0x1060;
    pub const USZ_HEIGHT: i32 = -0x1064;
    /// `tag`: the `BSStringT` it formats into.
    pub const TAG_TEXT: i32 = -0x1070;
    /// `reload <name>` / `delete <name>` name buffers (0x400 bytes).
    pub const RELOAD_NAME: i32 = -0x147c;
    pub const DELETE_NAME: i32 = -0x1884;
    /// `visible`: the integer read.
    pub const VISIBLE_VALUE: i32 = -0x188c;
    /// `set trait`: tile name (0x80), trait name (0x80) and the float.
    pub const TRAIT_TILE: i32 = -0x199c;
    pub const TRAIT_NAME: i32 = -0x191c;
    pub const TRAIT_VALUE: i32 = -0x1898;
    /// `bat` file name (0x80), the two paths (0x104 each).
    pub const BAT_NAME: i32 = -0x1a2c;
    pub const BAT_PATH: i32 = -0x1b34;
    pub const BAT_PATH_WITH_EXTENSION: i32 = -0x1c44;
    /// `bat`: one line of the file (0xa28 bytes) and the `Script` that runs it.
    pub const BAT_LINE: i32 = -0x266c;
    pub const BAT_SCRIPT: i32 = -0x26cc;
    /// `rbat` file name (0x80) and one output line (0x64).
    pub const RBAT_NAME: i32 = -0x2754;
    pub const RBAT_LINE: i32 = -0x27c4;
    /// The `Script` that runs a plain command line, and the `BSStringT`s used
    /// for the history.
    pub const COMMAND_SCRIPT: i32 = -0x281c;
    pub const HISTORY_TEXT: i32 = -0x2828;
    pub const RECALLED_TEXT: i32 = -0x2848;
    pub const DROPPED_TEXT: i32 = -0x28a4;
}

fn at(ebp: u32, offset: i32) -> u32 {
    ebp.wrapping_add(offset as u32)
}

fn line_byte(e: &Engine, ebp: u32, index: i32) -> u8 {
    e.mem.u8(at(ebp, frame::LINE + index))
}

fn set_line_byte(e: &mut Engine, ebp: u32, index: i32, value: u8) {
    e.mem.set_u8(at(ebp, frame::LINE + index), value);
}

/// `COMMAND_MATCHES_A(line, name, length) == 0`.
fn starts_with_a(e: &mut Engine, ebp: u32, name: u32, length: u32) -> bool {
    e.call(
        COMMAND_MATCHES_A,
        &args![at(ebp, frame::LINE), name, length],
    )
    .i32()
        == 0
}

/// `COMMAND_MATCHES_B(line, name, length) == 0`.
fn starts_with_b(e: &mut Engine, ebp: u32, name: u32, length: u32) -> bool {
    e.call(
        COMMAND_MATCHES_B,
        &args![at(ebp, frame::LINE), name, length],
    )
    .i32()
        == 0
}

/// Replaces every line feed in the NUL-terminated text at `text` by a space.
fn replace_line_feeds(e: &mut Engine, text: u32) {
    let mut i = 0u32;
    loop {
        let length = e.call(CRT_STRLEN, &args![text]).u32();
        if i >= length {
            break;
        }
        if e.mem.i8(text + i) == 0x0a {
            e.mem.set_u8(text + i, b' ');
        }
        i += 1;
    }
}

/// Runs the text as a script with the console's compiler (`Script::Script`,
/// `SetTemporary`, `SetText`, `CompileAndRun`, destructor).
fn run_console_script(e: &mut Engine, this: Ptr<MenuConsole>, script: u32, text: u32) {
    e.call(0x005a_a0f0, &args![script]);
    e.call(0x0048_4490, &args![script]);
    e.call(0x005a_be50, &args![script, text]);
    let compiler = e.get(this, MenuConsole::pConsoleCompiler);
    let interface = e.call(INTERFACE_GETTER, &args![]).u32();
    let reference = e.call(0x0070_31c0, &args![interface]).u32();
    e.call(0x005a_c400, &args![script, compiler, 1u32, reference]);
    e.call(0x005a_a1a0, &args![script]);
}

/// Opens `path` for the batch commands: allocates a `BSFile` and constructs
/// it (0 when the allocation fails).
fn open_file(e: &mut Engine, path: u32, mode: u32, buffer_size: u32, write: u32) -> u32 {
    let memory = e.call(ALLOCATE, &args![0x158u32]).u32();
    if memory != 0 {
        e.call(
            FILE_CONSTRUCT,
            &args![memory, path, mode, buffer_size, write],
        )
        .u32()
    } else {
        0
    }
}

/// `file != 0 && file->IsOpen(0, 0)` (virtual slot +0x20).
fn file_is_open(e: &mut Engine, file: u32) -> bool {
    file != 0 && e.vcall(file, 0x20, &args![0u32, 0u32]).bool()
}

/// Deletes an object through its virtual destructor (slot 0, flag 1).
fn delete_object(e: &mut Engine, object: u32) {
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
}

// Translated from 0071b210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuConsole::Idle` (Xbox PDB): handles one key message for the console.
/// Returns 0 when the console is not visible (or the interface's
/// `007023a0` test fails), otherwise 1.
///
/// The command line lives in a stack buffer with the text cursor `|` inside
/// it. A message of 0x80000000 and up edits the line (see the `MESSAGE_*`
/// constants), any other message inserts that character. On Enter
/// (0x80000008) the line has its cursor moved to the end and its line feeds
/// replaced, is echoed with `PrintLine`, added to the command history and
/// matched against the built-in commands: `dof`, `usz`, `clr`/`cls`, `tag`,
/// `reload ...`, `delete ...`, `front`, `stack`, `visible`, `exit menus`,
/// `emergency texture release`, `set trait`, `depth`, `new fonts`,
/// `old fonts`, `iso`, `restore menus`, `use workbook`, `dump textures`,
/// `bat`, `rbat`; anything else runs as a script, and is recorded when the
/// batch recorder (`rbat`) is on. After any other message the edited line is
/// stored as the newest history entry and drawn.
///
/// A line longer than the 0x800-byte stack buffer overflows the original's
/// stack; here it writes past the buffer inside the frame block.
pub fn menu_console_idle(e: &mut Engine, this: Ptr<MenuConsole>, message: u32) -> u8 {
    if e.get(this, MenuConsole::cVisible) <= 0 {
        return 0;
    }
    let interface = e.call(INTERFACE_GETTER, &args![]).u32();
    if !e.call(0x0070_23a0, &args![interface]).bool() {
        return 0;
    }
    if message == 0 {
        return 1;
    }
    e.with_stack(frame::SIZE, |e, block| {
        let ebp = block.addr() + frame::POINTER;
        idle_message(e, this, message, ebp);
    });
    1
}

fn idle_message(e: &mut Engine, this: Ptr<MenuConsole>, message: u32, ebp: u32) {
    // Reads the text typed so far and takes it as the command line.
    let (x, y) = console_position(e, this);
    let instance = e.call(DEBUG_TEXT_INSTANCE, &args![1u32]).u32();
    e.call(
        DEBUG_TEXT_READ,
        &args![instance, at(ebp, frame::TYPED_TEXT), x, y, 1u32],
    );
    e.call(
        COPY_TEXT,
        &args![at(ebp, frame::LINE), at(ebp, frame::TYPED_TEXT), 0x7feu32],
    );
    let mut length = 0i32;
    while line_byte(e, ebp, length) as i8 != 0 {
        length += 1;
    }
    // A trailing cursor is dropped, then the cursor is looked up and put at
    // the end when the line has none.
    if line_byte(e, ebp, length - 1) == CURSOR {
        length -= 1;
        set_line_byte(e, ebp, length, 0);
    }
    let mut cursor = 0i32;
    while cursor < length && line_byte(e, ebp, cursor) != CURSOR {
        cursor += 1;
    }
    if cursor == length {
        set_line_byte(e, ebp, cursor, CURSOR);
        length += 1;
    }
    if message == MESSAGE_ENTER_LINE {
        idle_enter_line(e, this, ebp, length);
        return;
    }
    // Editing keys.
    match message {
        MESSAGE_DELETE_BEFORE_CURSOR => {
            if length != 0 && cursor != 0 {
                length -= 1;
                let mut i = cursor;
                while i < length {
                    let next = line_byte(e, ebp, i + 1);
                    set_line_byte(e, ebp, i, next);
                    i += 1;
                }
                set_line_byte(e, ebp, length, 0);
                set_line_byte(e, ebp, cursor - 1, CURSOR);
            }
        }
        MESSAGE_DELETE_AT_CURSOR => {
            if cursor < length {
                length -= 1;
                let mut i = cursor;
                while i < length {
                    let next = line_byte(e, ebp, i + 1);
                    set_line_byte(e, ebp, i, next);
                    i += 1;
                }
                set_line_byte(e, ebp, length, 0);
                set_line_byte(e, ebp, cursor, CURSOR);
            }
        }
        MESSAGE_SCROLL_BACK => {
            let lines = setting_int(e, CONSOLE_VISIBLE_LINES);
            fn_0071d380(e, this, -lines + 1);
            menu_console_refresh_lines(e, this);
        }
        MESSAGE_SCROLL_FORWARD => {
            let lines = setting_int(e, CONSOLE_VISIBLE_LINES);
            fn_0071d380(e, this, lines - 1);
            menu_console_refresh_lines(e, this);
        }
        MESSAGE_HISTORY_OLDER => {
            let inputs = this.addr() + INPUTS_LIST;
            if e.call(LIST_COUNT, &args![inputs]).i32() != 0 {
                let position = e.get(this, MenuConsole::iInputsPos) + 1;
                e.set(this, MenuConsole::iInputsPos, position);
                let count = e.call(LIST_COUNT, &args![inputs]).i32();
                let position = e.call(0x004a_8f20, &args![position, count - 1]).i32();
                e.set(this, MenuConsole::iInputsPos, position);
                let mut node = e.call(FIRST_MEMBER, &args![inputs]).u32();
                let mut k = 0;
                while k < e.get(this, MenuConsole::iInputsPos) {
                    node = e.call(LIST_NEXT, &args![inputs, node]).u32();
                    k += 1;
                }
                let data = e.call(LIST_NODE_DATA, &args![inputs, node]).u32();
                let text = e.call(FIRST_MEMBER, &args![data]).u32();
                e.call(SPRINTF, &args![at(ebp, frame::LINE), STRING_FORMAT_S, text]);
            }
        }
        MESSAGE_HISTORY_NEWER => {
            let inputs = this.addr() + INPUTS_LIST;
            if e.call(LIST_COUNT, &args![inputs]).i32() != 0 {
                let position = e.get(this, MenuConsole::iInputsPos) - 1;
                e.set(this, MenuConsole::iInputsPos, position);
                let position = e.call(0x0064_7b70, &args![position, 0u32]).i32();
                e.set(this, MenuConsole::iInputsPos, position);
                let recalled = at(ebp, frame::RECALLED_TEXT);
                e.call(STRING_CONSTRUCT, &args![recalled]);
                let mut node = 0u32;
                if e.get(this, MenuConsole::iInputsPos) != 0 {
                    node = e.call(FIRST_MEMBER, &args![inputs]).u32();
                    let mut k = 0;
                    while k < e.get(this, MenuConsole::iInputsPos) {
                        node = e.call(LIST_NEXT, &args![inputs, node]).u32();
                        k += 1;
                    }
                }
                if node != 0 {
                    let data = e.call(LIST_NODE_DATA, &args![inputs, node]).u32();
                    e.call(0x0050_1c10, &args![recalled, data]);
                } else {
                    e.call(STRING_ASSIGN, &args![recalled, EMPTY_STRING]);
                }
                let text = if e.call(STRING_LENGTH, &args![recalled]).u32() != 0 {
                    e.call(FIRST_MEMBER, &args![recalled]).u32()
                } else {
                    EMPTY_STRING
                };
                e.call(SPRINTF, &args![at(ebp, frame::LINE), STRING_FORMAT_S, text]);
                e.call(STRING_DESTRUCT, &args![recalled]);
            }
        }
        MESSAGE_CURSOR_LEFT => {
            if length != 0 && cursor != 0 {
                let before = line_byte(e, ebp, cursor - 1);
                set_line_byte(e, ebp, cursor, before);
                set_line_byte(e, ebp, cursor - 1, CURSOR);
            }
        }
        MESSAGE_CURSOR_RIGHT => {
            if length != 0 && cursor + 1 < length {
                let after = line_byte(e, ebp, cursor + 1);
                set_line_byte(e, ebp, cursor, after);
                set_line_byte(e, ebp, cursor + 1, CURSOR);
            }
        }
        MESSAGE_CURSOR_TO_START => {
            if length != 0 && cursor != 0 {
                let mut i = cursor;
                while i > 0 {
                    let before = line_byte(e, ebp, i - 1);
                    set_line_byte(e, ebp, i, before);
                    i -= 1;
                }
                set_line_byte(e, ebp, 0, CURSOR);
            }
        }
        MESSAGE_CURSOR_TO_END => {
            if length != 0 && cursor + 1 < length {
                let mut i = cursor;
                while i < length - 1 {
                    let after = line_byte(e, ebp, i + 1);
                    set_line_byte(e, ebp, i, after);
                    i += 1;
                }
                set_line_byte(e, ebp, length - 1, CURSOR);
            }
        }
        _ => {
            // Insert the character at the cursor.
            set_line_byte(e, ebp, cursor, message as u8);
            length += 1;
            let mut i = length;
            while i > cursor {
                let moved = line_byte(e, ebp, i);
                set_line_byte(e, ebp, i + 1, moved);
                i -= 1;
            }
            set_line_byte(e, ebp, length + 1, 0);
            set_line_byte(e, ebp, cursor + 1, CURSOR);
        }
    }
    // The line must contain a cursor; the edited line becomes the newest
    // history entry and is drawn.
    let mut found = false;
    let mut i = 0i32;
    while line_byte(e, ebp, i) as i8 != 0 {
        if line_byte(e, ebp, i) == CURSOR {
            found = true;
            break;
        }
        i += 1;
    }
    if !found {
        set_line_byte(e, ebp, i, CURSOR);
        set_line_byte(e, ebp, i + 1, 0);
    }
    let history = at(ebp, frame::HISTORY_TEXT);
    e.call(STRING_FROM_C_STRING, &args![history, at(ebp, frame::LINE)]);
    let inputs = this.addr() + INPUTS_LIST;
    if e.call(LIST_COUNT, &args![inputs]).i32() == 0 {
        e.call(0x0071_df20, &args![inputs, history]);
    } else {
        let dropped = at(ebp, frame::DROPPED_TEXT);
        e.call(0x0071_dfa0, &args![inputs, dropped]);
        e.call(STRING_DESTRUCT, &args![dropped]);
        e.call(0x0071_df20, &args![inputs, history]);
    }
    let font = setting_int(e, CONSOLE_FONT);
    let (x, y) = console_position(e, this);
    debug_text_print(e, at(ebp, frame::LINE), x, y, font as u32);
    e.call(STRING_DESTRUCT, &args![history]);
}

/// Enter: moves the cursor to the end of the line, turns line feeds into
/// spaces (a `-` before one is dropped with it), then echoes, remembers and
/// executes the line.
fn idle_enter_line(e: &mut Engine, this: Ptr<MenuConsole>, ebp: u32, length: i32) {
    let mut i = 0i32;
    while i < length + 1 {
        if line_byte(e, ebp, i) == CURSOR {
            let next = line_byte(e, ebp, i + 1);
            set_line_byte(e, ebp, i, next);
            set_line_byte(e, ebp, i + 1, CURSOR);
        }
        i += 1;
    }
    i = 0;
    while i < length + 1 {
        if line_byte(e, ebp, i) == 0x0a {
            if line_byte(e, ebp, i - 1) == b'-' {
                let mut j = i - 1;
                while j < length + 1 {
                    let moved = line_byte(e, ebp, j + 2);
                    set_line_byte(e, ebp, j, moved);
                    j += 1;
                }
            } else {
                set_line_byte(e, ebp, i, b' ');
            }
        }
        if line_byte(e, ebp, i) == 0x0a {
            set_line_byte(e, ebp, i, b' ');
        }
        i += 1;
    }
    if line_byte(e, ebp, 0) == 0 {
        return;
    }
    e.mem.set_u8(COMMAND_ENTERED, 1);
    let interface = e.call(INTERFACE_GETTER, &args![]).u32();
    let count = e.call(LIST_COUNT, &args![this.addr() + STRINGS_LIST]).i32();
    fn_0071d380(e, this, count);
    menu_console_print_line(e, this, at(ebp, frame::LINE), &[]);
    let entered = at(ebp, frame::ENTERED_LINE);
    e.call(STRING_FROM_C_STRING, &args![entered, at(ebp, frame::LINE)]);
    e.call(0x0071_df20, &args![this.addr() + INPUTS_LIST, entered]);
    e.set(this, MenuConsole::iInputsPos, 0);
    let font = setting_int(e, CONSOLE_FONT);
    let (x, y) = console_position(e, this);
    debug_text_print(e, CURSOR_TEXT, x, y, font as u32);
    idle_run_command(e, this, ebp, interface);
    e.call(STRING_DESTRUCT, &args![entered]);
}

/// The command dispatch of an entered line.
fn idle_run_command(e: &mut Engine, this: Ptr<MenuConsole>, ebp: u32, interface: u32) {
    let line = at(ebp, frame::LINE);
    if starts_with_a(e, ebp, DOF_COMMAND, 3) {
        // dof <float>: sets the depth-of-field parameter block's 6th float.
        e.mem.set_f32(at(ebp, frame::DOF_VALUE), 0.0);
        e.call(SSCANF, &args![line, DOF_FORMAT, at(ebp, frame::DOF_VALUE)]);
        let a = e.call(0x007f_a950, &args![interface]).u32();
        let b = e.call(0x0066_29f0, &args![a]).u32();
        let source = e.call(0x0045_bbe0, &args![b]).u32();
        for word in 0..7 {
            let value = e.mem.u32(source + 4 * word);
            e.mem.set_u32(at(ebp, frame::DOF_BLOCK) + 4 * word, value);
        }
        let value = e.mem.f32(at(ebp, frame::DOF_VALUE));
        e.mem.set_f32(at(ebp, frame::DOF_BLOCK) + 0x14, value);
        let a = e.call(0x007f_a950, &args![interface]).u32();
        let b = e.call(0x0066_29f0, &args![a]).u32();
        e.call(0x00a6_faf0, &args![b, at(ebp, frame::DOF_BLOCK)]);
        return;
    }
    if starts_with_a(e, ebp, USZ_COMMAND, 3) {
        e.mem.set_u32(at(ebp, frame::USZ_WIDTH), 0);
        e.mem.set_u32(at(ebp, frame::USZ_HEIGHT), 0);
        e.call(
            SSCANF,
            &args![
                line,
                USZ_FORMAT,
                at(ebp, frame::USZ_WIDTH),
                at(ebp, frame::USZ_HEIGHT)
            ],
        );
        let width = e.mem.u32(at(ebp, frame::USZ_WIDTH));
        let height = e.mem.u32(at(ebp, frame::USZ_HEIGHT));
        e.call(0x0070_30c0, &args![width, height]);
        return;
    }
    if e.call(STRING_MATCHES_A, &args![line, CLR_COMMAND]).i32() == 0
        || e.call(STRING_MATCHES_B, &args![line, CLS_COMMAND]).i32() == 0
    {
        e.call(0x0071_e070, &args![this.addr() + STRINGS_LIST]);
        e.set(this, MenuConsole::iLastString, 0);
        menu_console_refresh_lines(e, this);
        return;
    }
    if e.call(STRING_MATCHES_A, &args![line, TAG_COMMAND]).i32() == 0 {
        idle_tag(e, ebp, interface);
        return;
    }
    if starts_with_a(e, ebp, RELOAD_STRINGS_COMMAND, 0xe) {
        let object = e.call(0x004f_d400, &args![interface]).u32();
        delete_object(e, object);
        e.call(0x00a1_ca20, &args![GLOBALS_XML, 0u32]);
        let tile = e.call(0x0058_6150, &args![interface]).u32();
        let font = e.call(0x00a0_1b00, &args![tile, GLOBALS_XML]).u32();
        e.call(0x00a0_87d0, &args![font, 0u32, 0u32]);
        e.call(0x0050_0a20, &args![interface, font]);
        return;
    }
    if starts_with_a(e, ebp, RELOAD_HUD_RETICLE_COMMAND, 0x11) {
        e.call(0x00a1_c800, &args![]);
        return;
    }
    if starts_with_a(e, ebp, RELOAD_HUD_SAFE_ZONE_COMMAND, 0x12) {
        e.call(0x00a1_c800, &args![]);
        e.call(0x0070_38e0, &args![3u32]);
        return;
    }
    if e.call(STRING_MATCHES_A, &args![line, RELOAD_MAIN_MENU_COMMAND])
        .i32()
        == 0
    {
        e.call(0x00a1_c800, &args![]);
        let menu_id = e.call(0x0070_ede0, &args![]).u32();
        let menu = e.call(0x00a0_9030, &args![menu_id]).u32();
        delete_object(e, menu);
        e.call(0x007c_b7d0, &args![0u32, 0u32]);
        return;
    }
    if starts_with_a(e, ebp, RELOAD_COMMAND, 7) {
        e.call(0x00a1_c800, &args![]);
        let name = at(ebp, frame::RELOAD_NAME);
        e.call(SSCANF, &args![line, RELOAD_FORMAT, name]);
        // The name gets "&" in front and ";" behind unless it starts with "&".
        if e.mem.i8(name) != 0x26 {
            let mut length = 0u32;
            while e.mem.i8(name + length) != 0 {
                length += 1;
            }
            e.mem.set_u8(name + length + 2, 0);
            e.mem.set_u8(name + length + 1, b';');
            while length > 0 {
                let before = e.mem.u8(name + length - 1);
                e.mem.set_u8(name + length, before);
                length -= 1;
            }
            e.mem.set_u8(name, b'&');
        }
        let key = e.call(0x00a0_1860, &args![name]).u32();
        let manager = e.call(0x0071_e290, &args![1u32]).u32();
        e.call(0x0072_0aa0, &args![manager, key]);
        return;
    }
    if starts_with_a(e, ebp, DELETE_COMMAND, 7) {
        let name = at(ebp, frame::DELETE_NAME);
        e.call(SSCANF, &args![line, DELETE_FORMAT, name]);
        let key = e.call(0x00a0_1860, &args![name]).u32();
        let manager = e.call(0x0071_e290, &args![1u32]).u32();
        e.call(0x0072_0e00, &args![manager, key]);
        return;
    }
    if starts_with_b(e, ebp, FRONT_COMMAND, 5) {
        let manager = e.call(0x0071_e290, &args![1u32]).u32();
        let a = e.call(0x0072_0e60, &args![manager]).u32();
        let b = e.call(0x0072_6070, &args![a]).u32();
        let name = e.call(0x0070_0300, &args![b]).u32();
        menu_console_print_line(e, this, FRONTMOST_FORMAT, &[name]);
        return;
    }
    if starts_with_b(e, ebp, STACK_COMMAND, 5) {
        let mut index = 0u32;
        loop {
            let entry = e.call(0x0071_4f70, &args![interface, index]).u32();
            index += 1;
            if entry == 0 {
                break;
            }
            let next = e.call(0x0071_4f70, &args![interface, index]).u32();
            menu_console_print_line(e, this, STACK_ENTRY_FORMAT, &[index, next]);
        }
        return;
    }
    if starts_with_b(e, ebp, VISIBLE_COMMAND, 7) {
        e.mem.set_u32(at(ebp, frame::VISIBLE_VALUE), 0);
        e.call(
            SSCANF,
            &args![line, VISIBLE_SCAN_FORMAT, at(ebp, frame::VISIBLE_VALUE)],
        );
        let menu_id = e.mem.u32(at(ebp, frame::VISIBLE_VALUE));
        let manager = e.call(0x0071_e290, &args![1u32]).u32();
        let visible = e.call(0x0072_0f90, &args![manager, menu_id]).u8();
        menu_console_print_line(e, this, VISIBLE_REPORT_FORMAT, &[visible as u32]);
        return;
    }
    if starts_with_b(e, ebp, EXIT_MENUS_COMMAND, 0xa) {
        let mut top = e.call(0x0071_4f00, &args![interface]).u32();
        while top != 0 {
            e.call(0x0071_4fd0, &args![interface, top, 0u32]);
            top = e.call(0x0071_4f00, &args![interface]).u32();
        }
        return;
    }
    if starts_with_b(e, ebp, EMERGENCY_TEXTURE_RELEASE_COMMAND, 0x19) {
        e.call(0x0070_3920, &args![]);
        menu_console_print_line(e, this, TEXTURE_RELEASE_MESSAGE, &[]);
        return;
    }
    if starts_with_b(e, ebp, SET_TRAIT_COMMAND, 9) {
        idle_set_trait(e, this, ebp);
        return;
    }
    if starts_with_b(e, ebp, DEPTH_COMMAND, 5) {
        let depth = e.call(0x00a0_1530, &args![]).f32();
        let depth = e.call(FTOL, &args![depth as f64]).u32();
        menu_console_print_line(e, this, MAX_DEPTH_FORMAT, &[depth]);
        let interface = e.call(INTERFACE_GETTER, &args![]).u32();
        let tile = e.call(0x0045_cd60, &args![interface]).u32();
        let depth = e.call(0x00a0_11b0, &args![tile, 0xfadu32]).f32();
        let depth = e.call(FTOL, &args![depth as f64]).u32();
        menu_console_print_line(e, this, CURSOR_DEPTH_FORMAT, &[depth]);
        return;
    }
    if starts_with_b(e, ebp, NEW_FONTS_COMMAND, 9) {
        let fonts = e.call(FONT_ARRAY_GETTER, &args![]).u32();
        fn_0071d010(e, Ptr::new(fonts), 1);
        return;
    }
    if starts_with_b(e, ebp, OLD_FONTS_COMMAND, 9) {
        let fonts = e.call(FONT_ARRAY_GETTER, &args![]).u32();
        fn_0071d010(e, Ptr::new(fonts), 0);
        return;
    }
    if starts_with_b(e, ebp, ISO_COMMAND, 3) {
        let mut first = 0u32;
        let mut second = 0u32;
        if line_byte(e, ebp, 3) == b'1' {
            first = 1;
        }
        if line_byte(e, ebp, 4) == b'1' {
            second = 1;
        }
        e.call(0x0070_2c80, &args![first, second]);
        return;
    }
    if starts_with_b(e, ebp, RESTORE_MENUS_COMMAND, 0xd) {
        e.call(0x0070_2dd0, &args![]);
        return;
    }
    if starts_with_b(e, ebp, USE_WORKBOOK_COMMAND, 0xc) {
        idle_use_workbook(e, this);
        return;
    }
    if starts_with_b(e, ebp, DUMP_TEXTURES_COMMAND, 0xd) {
        menu_console_dump_textures(e, this);
        return;
    }
    if starts_with_b(e, ebp, BAT_COMMAND, 3) {
        idle_play_batch(e, this, ebp);
        return;
    }
    if starts_with_b(e, ebp, RBAT_COMMAND, 3) {
        idle_record_batch(e, this, ebp);
        return;
    }
    // Any other line runs as a script, and is recorded while recording.
    if e.get(this, MenuConsole::bRecordingBatch) {
        let recorded = e.get(this, MenuConsole::iRecordingLineCount);
        if recorded < RECORDING_LINES {
            let slot = this.addr() + 0x40 + recorded as u32 * RECORDING_LINE_SIZE;
            e.set(this, MenuConsole::iRecordingLineCount, recorded + 1);
            e.call(STRING_COPY, &args![slot, line]);
            e.call(PRINT_TO_CONSOLE, &args![RECORDED_MESSAGE]);
        } else {
            e.call(PRINT_TO_CONSOLE, &args![RECORDING_FULL_MESSAGE]);
        }
    }
    replace_line_feeds(e, line);
    run_console_script(e, this, at(ebp, frame::COMMAND_SCRIPT), line);
}

/// `tag`: describes the object under the cursor (name and form id) with
/// `a10c00` on the tile found for its form type.
fn idle_tag(e: &mut Engine, ebp: u32, interface: u32) {
    let object = e.call(0x0070_31c0, &args![interface]).u32();
    if object == 0 {
        return;
    }
    let text = at(ebp, frame::TAG_TEXT);
    e.call(STRING_CONSTRUCT, &args![text]);
    let form_id = e.call(0x0084_e3a0, &args![object]).u32();
    let name = e.call(0x0055_d520, &args![object]).u32();
    e.call(STRING_FORMAT, &args![text, TAG_FORMAT, name, form_id]);
    let key = e.vcall(object, 0x1d0, &args![]).u32();
    let mut found = e.call(0x0065_3270, &args![TAG_LOOKUP_TABLE, key]).u32();
    if found == 0 {
        let key = e.vcall(object, 0x1d0, &args![]).u32();
        let fallback = e.call(0x0096_11e0, &args![key]).u32();
        found = e
            .call(0x0065_3270, &args![TAG_LOOKUP_TABLE, fallback])
            .u32();
    }
    let argument: f32 = e.global(PRINT_FLOAT_ARGUMENT);
    let c_text = e.call(FIRST_MEMBER, &args![text]).u32();
    e.call(0x00a1_0c00, &args![found, c_text, 0u32, argument]);
    e.call(STRING_DESTRUCT, &args![text]);
}

/// `set trait <tile> <trait> <value>`.
fn idle_set_trait(e: &mut Engine, this: Ptr<MenuConsole>, ebp: u32) {
    let line = at(ebp, frame::LINE);
    let tile_name = at(ebp, frame::TRAIT_TILE);
    let trait_name = at(ebp, frame::TRAIT_NAME);
    let value = at(ebp, frame::TRAIT_VALUE);
    e.mem.set_f32(value, 0.0);
    e.call(
        SSCANF,
        &args![line, SET_TRAIT_FORMAT, tile_name, trait_name, value],
    );
    let tile = e.call(0x00a0_8b20, &args![0u32, tile_name]).u32();
    let trait_id = e.call(0x00a0_1860, &args![trait_name]).i32();
    if tile != 0 && trait_id > 0 {
        menu_console_print_line(e, this, TRAIT_SET_MESSAGE, &[]);
        let number = e.mem.f32(value);
        e.call(0x00a0_12d0, &args![tile, trait_id, number, 1u32]);
    } else if tile != 0 {
        // The original passes the address of the name as the `va_list`.
        fn_0071d0a0(e, this, UNKNOWN_TRAIT_FORMAT, trait_name);
    } else {
        fn_0071d0a0(e, this, TILE_NOT_FOUND_FORMAT, tile_name);
    }
}

/// `use workbook`: with a book (or scroll) menu open, calls `0073b500` on it.
fn idle_use_workbook(e: &mut Engine, this: Ptr<MenuConsole>) {
    let menu = e.call(0x00a0_9030, &args![0x402u32]).u32();
    let menu = e
        .call(
            DYNAMIC_CAST,
            &args![menu, 0u32, TYPE_TILE_MENU_BASE, TYPE_TILE_MENU, 0u32],
        )
        .u32();
    if menu == 0 {
        menu_console_print_line(e, this, WORKBOOK_NEEDS_BOOK_MESSAGE, &[]);
        return;
    }
    let parent = e.call(0x0063_9b40, &args![menu]).u32();
    let book = e
        .call(
            DYNAMIC_CAST,
            &args![parent, 0u32, TYPE_MENU, TYPE_BOOK_MENU, 0u32],
        )
        .u32();
    if book != 0 {
        e.call(0x0073_b500, &args![book]);
    }
}

/// `bat <file>`: runs each line of the file (or of `<file>.txt`) as a script.
fn idle_play_batch(e: &mut Engine, this: Ptr<MenuConsole>, ebp: u32) {
    let line = at(ebp, frame::LINE);
    let name = at(ebp, frame::BAT_NAME);
    let first = e.mem.u8(EMPTY_STRING);
    e.mem.set_u8(name, first);
    e.call(CRT_MEMSET, &args![name + 1, 0u32, 0x7fu32]);
    e.call(SSCANF, &args![line, BAT_FORMAT, name]);
    if e.call(CRT_STRLEN, &args![name]).u32() == 0 {
        e.call(PRINT_TO_CONSOLE, &args![FILENAME_REQUIRED_MESSAGE]);
        return;
    }
    let path = at(ebp, frame::BAT_PATH);
    let path_text = at(ebp, frame::BAT_PATH_WITH_EXTENSION);
    let first = e.mem.u8(EMPTY_STRING);
    e.mem.set_u8(path, first);
    e.call(CRT_MEMSET, &args![path + 1, 0u32, 0x103u32]);
    e.mem.set_u8(path_text, first);
    e.call(CRT_MEMSET, &args![path_text + 1, 0u32, 0x103u32]);
    e.call(STRING_APPEND_BOUNDED, &args![path, 0x104u32, name]);
    let mut file = open_file(e, path, 0, 0x4000, 0);
    let name_length = e.call(STRING_LENGTH_C, &args![name]).u32();
    let opened = file_is_open(e, file);
    let try_with_extension = !opened
        && !(name_length > 4
            && e.call(
                0x0045_64f0,
                &args![name + name_length - 4, TXT_EXTENSION, 4u32],
            )
            .i32()
                == 0);
    if try_with_extension {
        delete_object(e, file);
        e.call(STRING_COPY_BOUNDED, &args![path_text, 0x104u32, path]);
        e.call(
            STRING_APPEND_BOUNDED,
            &args![path_text, 0x104u32, TXT_EXTENSION],
        );
        file = open_file(e, path_text, 0, 0x4000, 0);
    }
    if !file_is_open(e, file) {
        if e.call(STRING_LENGTH_C, &args![path_text]).u32() != 0 {
            e.call(
                PRINT_TO_CONSOLE,
                &args![NEITHER_FILE_OPENED_FORMAT, path, path_text],
            );
        } else {
            e.call(PRINT_TO_CONSOLE, &args![FILE_NOT_OPENED_FORMAT, path]);
        }
    } else {
        let text = at(ebp, frame::BAT_LINE);
        loop {
            let read = e.vcall(file, 0x34, &args![text, 0xa28u32, 0xdu32]).u32();
            if read == 0 {
                break;
            }
            replace_line_feeds(e, text);
            if e.call(CRT_STRLEN, &args![text]).u32() > 1 {
                e.call(PRINT_TO_CONSOLE, &args![SCRIPT_ECHO_FORMAT, text]);
                run_console_script(e, this, at(ebp, frame::BAT_SCRIPT), text);
            }
        }
    }
    delete_object(e, file);
}

/// `rbat [<file>]`: starts recording the commands entered, or (while
/// recording) saves them to the file, or cancels without a file name.
fn idle_record_batch(e: &mut Engine, this: Ptr<MenuConsole>, ebp: u32) {
    let line = at(ebp, frame::LINE);
    if !e.get(this, MenuConsole::bRecordingBatch) {
        e.call(PRINT_TO_CONSOLE, &args![RECORDING_STARTED_MESSAGE]);
        e.call(PRINT_TO_CONSOLE, &args![RECORDING_HELP_MESSAGE]);
        e.set(this, MenuConsole::bRecordingBatch, true);
        e.set(this, MenuConsole::iRecordingLineCount, 0);
        for i in 0..RECORDING_LINES as u32 {
            e.call(
                STRING_COPY,
                &args![this.addr() + 0x40 + i * RECORDING_LINE_SIZE, EMPTY_STRING],
            );
        }
        return;
    }
    let name = at(ebp, frame::RBAT_NAME);
    let first = e.mem.u8(EMPTY_STRING);
    e.mem.set_u8(name, first);
    e.call(CRT_MEMSET, &args![name + 1, 0u32, 0x7fu32]);
    e.call(SSCANF, &args![line, RBAT_FORMAT, name]);
    if e.call(CRT_STRLEN, &args![name]).u32() == 0 {
        e.call(PRINT_TO_CONSOLE, &args![RECORDING_NO_FILENAME_MESSAGE]);
    } else if e.get(this, MenuConsole::iRecordingLineCount) <= 0 {
        e.call(PRINT_TO_CONSOLE, &args![RECORDING_EMPTY_MESSAGE]);
    } else {
        let file = open_file(e, name, 1, 0x1000, 1);
        if !file_is_open(e, file) {
            e.call(PRINT_TO_CONSOLE, &args![FILE_NOT_OPENED_FORMAT, name]);
        } else {
            let output = at(ebp, frame::RBAT_LINE);
            let mut i = 0i32;
            while i < e.get(this, MenuConsole::iRecordingLineCount) {
                let recorded = this.addr() + 0x40 + i as u32 * RECORDING_LINE_SIZE;
                e.call(
                    SNPRINTF,
                    &args![output, 0x64u32, RECORDED_LINE_FORMAT, recorded],
                );
                let length = e.call(STRING_LENGTH_C, &args![output]).u32();
                e.call(FILE_WRITE, &args![file, output, length]);
                i += 1;
            }
            e.call(FILE_CLOSE, &args![file]);
            e.call(PRINT_TO_CONSOLE, &args![BATCH_CREATED_FORMAT, name]);
        }
        delete_object(e, file);
    }
    e.set(this, MenuConsole::bRecordingBatch, false);
    e.set(this, MenuConsole::iRecordingLineCount, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0071af10,
            fn_0071af10(Ptr<MenuConsole>) -> Ptr<MenuConsole>
        ),
        entry!(0x0071b0e0, menu_console_init(Ptr<MenuConsole>)),
        entry!(0x0071b160, menu_console_instance(u8) -> Ptr<MenuConsole>),
        entry!(0x0071b210, menu_console_idle(Ptr<MenuConsole>, u32) -> u8),
        entry!(0x0071d010, fn_0071d010(Ptr, u8)),
        (0x0071d030, print_line_entry as AbiFn),
        entry!(0x0071d0a0, fn_0071d0a0(Ptr<MenuConsole>, u32, u32)),
        entry!(0x0071d380, fn_0071d380(Ptr<MenuConsole>, i32)),
        entry!(0x0071d410, menu_console_refresh_lines(Ptr<MenuConsole>)),
        entry!(
            0x0071d580,
            menu_console_toggle_visible(Ptr<MenuConsole>) -> u8
        ),
        entry!(0x0071d620, fn_0071d620(Ptr<MenuConsole>, i8)),
        entry!(0x0071d750, fn_0071d750(Ptr)),
        entry!(
            0x0071d770,
            menu_console_on_enter_menu_mode(Ptr<MenuConsole>)
        ),
        entry!(0x0071d800, menu_console_dump_textures(Ptr<MenuConsole>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What `DebugText::Print` was asked to draw: text, x, y, font.
    type Prints = Rc<RefCell<Vec<(String, f32, f32, u32)>>>;

    const INTERFACE_SLOT: u32 = 0x011d_8a80;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Writes a C string into game memory (mapping the page first when the
    /// address is in the exe's data).
    fn put_string(e: &mut Engine, addr: u32, text: &str) {
        if !e.mem.is_mapped(addr) {
            e.map(addr & !0xfff, 0x1000);
        }
        e.mem.set_cstr(addr, text.as_bytes());
    }

    fn read_string(e: &Engine, addr: u32) -> String {
        if addr == 0 {
            return String::new();
        }
        String::from_utf8_lossy(&e.mem.cstr(addr)).into_owned()
    }

    /// A new heap copy of a C string.
    fn heap_string(e: &mut Engine, text: &str) -> u32 {
        let block = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(block, text.as_bytes());
        block
    }

    /// Sets the C string a `BSStringT` object holds (and its length).
    fn set_text(e: &mut Engine, string: u32, text: &str) {
        let block = heap_string(e, text);
        e.mem.set_u32(string, block);
        e.mem.set_u16(string + 4, text.len() as u16);
    }

    fn text_of(e: &Engine, string: u32) -> String {
        read_string(e, e.mem.u32(string))
    }

    /// The engine every console test starts from: the console's globals
    /// mapped, and doubles for the string, list, setting and debug text
    /// helpers.
    ///
    /// Strings are `BSStringT` objects (`pString`, `sLen`); lists are three
    /// words (head, tail, count) with 16-byte nodes (next, previous, the
    /// string inline at +8).
    fn console_engine() -> (Engine, Prints) {
        let mut e = Engine::new();
        for page in [
            0x0101_1000u32,
            0x0101_2000,
            0x011d_8000,
            0x011d_e000,
            0x0106_f000,
            0x0103_c000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(CONSOLE_FONT, 2i32);
        e.set_global(CONSOLE_VISIBLE_LINES, 4i32);
        e.set_global(CONSOLE_HISTORY_SIZE, 50i32);
        e.set_global(CONSOLE_TEXT_X, 30i32);
        e.set_global(CONSOLE_TEXT_Y, 100i32);
        e.set_global(PRINT_FLOAT_ARGUMENT, -1.0f32);
        // Settings: the value sits in the setting object itself.
        e.register(SETTING_INT_POINTER, |_, a| ret(a[0]));
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_DESTRUCT, |_, _| Ret::default());
        e.register(FIRST_MEMBER, |e, a| ret(e.mem.u32(a[0])));
        e.register(LIST_COUNT, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(LIST_NODE_DATA, |_, a| ret(a[1] + 8));
        e.register(LIST_NEXT, |e, a| {
            ret(if a[1] == 0 { 0 } else { e.mem.u32(a[1]) })
        });
        e.register(STRING_LENGTH, |e, a| ret(e.mem.u16(a[0] + 4) as u32));
        e.register(STRING_ASSIGN, |e, a| {
            set_text(e, a[0], &read_string(e, a[1]));
            ret(a[0])
        });
        e.register(STRING_FROM_C_STRING, |e, a| {
            set_text(e, a[0], &read_string(e, a[1]));
            ret(a[0])
        });
        e.register(STRING_FORMAT_LIST, |e, a| {
            set_text(e, a[0], &read_string(e, a[1]));
            Ret::default()
        });
        // Appending copies the string into a new node; removing the first
        // line copies it out.
        fn append(e: &mut Engine, a: &[u32]) -> Ret {
            let node = e.mem.alloc(16);
            let (head, tail) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
            if head == 0 {
                e.mem.set_u32(a[0], node);
            } else {
                e.mem.set_u32(tail, node);
                e.mem.set_u32(node + 4, tail);
            }
            e.mem.set_u32(a[0] + 4, node);
            let count = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, count + 1);
            let (pointer, length) = (e.mem.u32(a[1]), e.mem.u16(a[1] + 4));
            e.mem.set_u32(node + 8, pointer);
            e.mem.set_u16(node + 12, length);
            ret(node)
        }
        e.register(0x0071_df60, append);
        e.register(0x0071_df20, append);
        e.register(0x0071_dfa0, |e, a| {
            let head = e.mem.u32(a[0]);
            let (pointer, length) = (e.mem.u32(head + 8), e.mem.u16(head + 12));
            e.mem.set_u32(a[1], pointer);
            e.mem.set_u16(a[1] + 4, length);
            let next = e.mem.u32(head);
            e.mem.set_u32(a[0], next);
            if next == 0 {
                e.mem.set_u32(a[0] + 4, 0);
            }
            let count = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, count - 1);
            Ret::default()
        });
        e.register(0x0071_e070, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
        let interface = e.mem.alloc(0x200);
        e.set_global(INTERFACE_SLOT, interface);
        e.register(
            INTERFACE_GETTER,
            |e, _| ret(e.global::<u32>(INTERFACE_SLOT)),
        );
        let prints: Prints = Rc::new(RefCell::new(Vec::new()));
        e.register(DEBUG_TEXT_INSTANCE, |_, _| ret(0x0bad_0000));
        let log = prints.clone();
        e.register_double(DEBUG_TEXT_PRINT, move |e, a| {
            log.borrow_mut().push((
                read_string(e, a[1]),
                f32::from_bits(a[2]),
                f32::from_bits(a[3]),
                a[7],
            ));
            Ret::default()
        });
        // The wrapping, the hooks and the console flags used by PrintLine.
        e.register(0x0070_5ea0, |_, _| ret(0));
        e.register(0x0070_edf0, |_, _| ret(1));
        e.register(0x0071_dd90, |_, _| Ret::default());
        e.register(FONT_ARRAY_GETTER, |_, _| ret(0x0004_0000));
        e.register(FONT_ARRAY_ELEMENT, |_, a| ret(a[0] + 4 * a[1]));
        e.register(0x00a1_60c0, |_, _| ret(1));
        e.register(0x0070_23a0, |_, _| ret(1));
        e.register(0x0040_fbe0, |_, _| Ret::default());
        e.register(0x00a0_de80, |_, _| Ret::default());
        (e, prints)
    }

    fn new_console(e: &mut Engine) -> Ptr<MenuConsole> {
        e.new_object()
    }

    /// Adds a line to the console's list of lines the way the game does.
    fn push_line(e: &mut Engine, console: Ptr<MenuConsole>, text: &str) {
        let string = e.mem.alloc(8);
        set_text(e, string, text);
        e.call(0x0071_df60, &args![console.addr() + STRINGS_LIST, string]);
    }

    fn lines_of(e: &Engine, console: Ptr<MenuConsole>) -> Vec<String> {
        let mut out = Vec::new();
        let mut node = e.mem.u32(console.addr() + STRINGS_LIST);
        while node != 0 {
            out.push(text_of(e, node + 8));
            node = e.mem.u32(node);
        }
        out
    }

    #[test]
    fn constructor_reads_the_settings_and_clears_the_state() {
        let (mut e, _) = console_engine();
        e.register(0x0048_f200, |_, _| Ret::default());
        e.register(FONT_ARRAY_GETTER, |_, _| ret(0x4_0000));
        e.register(FONT_ARRAY_ELEMENT, |_, a| ret(a[0] + 4 * a[1]));
        e.register(FONT_HEIGHT, |_, _| 12.75f32.into_ret());
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
        e.register(MEMSET, |_, _| Ret::default());
        let console = new_console(&mut e);
        e.mem.set_u8(console.addr() + 0x39, 1);
        e.set(console, MenuConsole::cVisible, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0071_af10, &args![console]).u32(), console.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(console, MenuConsole::iConsoleLineSpacing), 12 + 3);
        assert_eq!(e.get(console, MenuConsole::iConsoleXPos), 30);
        assert_eq!(e.get(console, MenuConsole::iConsoleYPos), 100);
        assert_eq!(e.get(console, MenuConsole::cVisible), 0);
        assert!(!e.get(console, MenuConsole::bIsActive));
        assert!(!e.get(console, MenuConsole::bRecordingBatch));
        // Both lists are constructed, the font element is the one before the
        // font setting's index, and the file name is cleared.
        let lists: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == 0x0048_f200)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(lists, [console.addr() + 4, console.addr() + 0x10]);
        let element = log.iter().find(|(a, _)| *a == FONT_ARRAY_ELEMENT).unwrap();
        assert_eq!(element.1, [0x4_0000, 1]);
        assert!(log
            .iter()
            .any(|(a, w)| *a == MEMSET && w[..] == [console.addr() + 0x810, 0, 0x104]));
    }

    #[test]
    fn init_creates_the_script_compiler() {
        let (mut e, _) = console_engine();
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x005a_ea50, |_, a| ret(a[0] + 0x1000));
        let console = new_console(&mut e);
        e.call(0x0071_b0e0, &args![console]);
        let compiler = e.get(console, MenuConsole::pConsoleCompiler).addr();
        assert_ne!(compiler, 0);
        // A failed allocation leaves no compiler.
        e.register(ALLOCATE, |_, _| ret(0));
        e.call(0x0071_b0e0, &args![console]);
        assert_eq!(e.get(console, MenuConsole::pConsoleCompiler).addr(), 0);
    }

    #[test]
    fn instance_creates_once_when_asked() {
        let (mut e, _) = console_engine();
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        // (The constructor and `Init` are direct calls: their callees are doubled.)
        e.register(0x0048_f200, |_, _| Ret::default());
        e.register(FONT_HEIGHT, |_, _| 8.0f32.into_ret());
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
        e.register(MEMSET, |_, _| Ret::default());
        e.register(0x005a_ea50, |_, a| ret(a[0] + 0x1000));
        assert_eq!(e.call(0x0071_b160, &args![0u8]).u32(), 0);
        let created = e.call(0x0071_b160, &args![1u8]).u32();
        assert_ne!(created, 0);
        assert_eq!(e.global::<u32>(CONSOLE_INSTANCE), created);
        // Later calls return it, with or without the flag.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0071_b160, &args![1u8]).u32(), created);
        assert_eq!(e.call(0x0071_b160, &args![0u8]).u32(), created);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn fn_0071d010_stores_a_byte() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x40);
        e.call(0x0071_d010, &args![block, 1u8]);
        assert_eq!(e.mem.u8(block + 0x20), 1);
        e.call(0x0071_d010, &args![block, 0u8]);
        assert_eq!(e.mem.u8(block + 0x20), 0);
    }

    #[test]
    fn print_line_formats_with_the_extra_words_as_a_va_list() {
        let (mut e, _) = console_engine();
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(vec![]));
        let log = seen.clone();
        e.register_double(STRING_FORMAT_LIST, move |e, a| {
            log.borrow_mut()
                .push((0..2).map(|i| e.mem.u32(a[2] + 4 * i)).collect());
            set_text(e, a[0], &read_string(e, a[1]));
            Ret::default()
        });
        put_string(&mut e, 0x0106_fcb4, "Frontmost: x");
        let console = new_console(&mut e);
        e.call(0x0071_d030, &args![console, 0x0106_fcb4u32, 7u32, 9u32]);
        assert_eq!(*seen.borrow(), vec![vec![7, 9]]);
        assert_eq!(lines_of(&e, console), ["Frontmost: x"]);
    }

    #[test]
    fn format_line_skips_while_the_hook_says_so() {
        let (mut e, _) = console_engine();
        e.register(0x0070_5ea0, |_, _| ret(1));
        e.register(0x0070_edf0, |_, _| ret(0));
        let console = new_console(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0071_d0a0, &args![console, 0x0106_fcb4u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 3); // the call itself, 705ea0 and 70edf0
        assert!(lines_of(&e, console).is_empty());
    }

    #[test]
    fn format_line_wraps_into_lines_and_follows_the_last_one() {
        let (mut e, prints) = console_engine();
        put_string(&mut e, 0x0106_fcb4, "ab cd");
        // The wrapping breaks "ab cd" into "ab" and "cd" in place.
        e.register(0x00a1_60c0, |e, a| {
            let text = e.mem.u32(a[1]);
            e.mem.set_u8(text + 2, 0);
            ret(2)
        });
        let console = new_console(&mut e);
        e.call(0x0071_d0a0, &args![console, 0x0106_fcb4u32, 0u32]);
        assert_eq!(lines_of(&e, console), ["ab", "cd"]);
        // The view followed the last line and was redrawn.
        assert_eq!(e.get(console, MenuConsole::iLastString), 2);
        let drawn: Vec<String> = prints.borrow().iter().map(|p| p.0.clone()).collect();
        assert_eq!(drawn, ["", "", "ab", "cd"]);
    }

    #[test]
    fn format_line_keeps_the_view_when_scrolled_back() {
        let (mut e, prints) = console_engine();
        put_string(&mut e, 0x0106_fcb4, "new");
        let console = new_console(&mut e);
        push_line(&mut e, console, "one");
        push_line(&mut e, console, "two");
        e.set(console, MenuConsole::iLastString, 1);
        e.call(0x0071_d0a0, &args![console, 0x0106_fcb4u32, 0u32]);
        assert_eq!(lines_of(&e, console), ["one", "two", "new"]);
        assert_eq!(e.get(console, MenuConsole::iLastString), 1);
        assert!(prints.borrow().is_empty());
    }

    #[test]
    fn format_line_drops_the_oldest_lines_above_the_history_size() {
        let (mut e, _) = console_engine();
        e.set_global(CONSOLE_HISTORY_SIZE, 2i32);
        put_string(&mut e, 0x0106_fcb4, "new");
        let console = new_console(&mut e);
        push_line(&mut e, console, "one");
        push_line(&mut e, console, "two");
        e.set(console, MenuConsole::iLastString, 2);
        e.call(0x0071_d0a0, &args![console, 0x0106_fcb4u32, 0u32]);
        assert_eq!(lines_of(&e, console), ["two", "new"]);
        // It followed the last line: 3 after the append, minus the dropped one.
        assert_eq!(e.get(console, MenuConsole::iLastString), 2);
    }

    #[test]
    fn scroll_keeps_the_last_line_between_the_visible_count_and_the_total() {
        let (mut e, _) = console_engine();
        let console = new_console(&mut e);
        for line in ["a", "b", "c", "d", "e", "f"] {
            push_line(&mut e, console, line);
        }
        e.set(console, MenuConsole::iLastString, 5);
        e.call(0x0071_d380, &args![console, 1i32]);
        assert_eq!(e.get(console, MenuConsole::iLastString), 6);
        // Past the end: clamped to the number of lines.
        e.call(0x0071_d380, &args![console, 10i32]);
        assert_eq!(e.get(console, MenuConsole::iLastString), 6);
        // Back past the visible lines (4): clamped to 4.
        e.call(0x0071_d380, &args![console, -10i32]);
        assert_eq!(e.get(console, MenuConsole::iLastString), 4);
    }

    #[test]
    fn refresh_blanks_missing_lines_then_draws_each_line_lower() {
        let (mut e, prints) = console_engine();
        let console = new_console(&mut e);
        e.set(console, MenuConsole::iConsoleLineSpacing, 15);
        e.set(console, MenuConsole::iConsoleXPos, 30);
        e.set(console, MenuConsole::iConsoleYPos, 100);
        for line in ["a", "b", "c"] {
            push_line(&mut e, console, line);
        }
        e.set(console, MenuConsole::iLastString, 3);
        e.call(0x0071_d410, &args![console]);
        // Four visible lines end at y = 100 (the first one is 4 spacings
        // up); one is missing and is blanked with font 0.
        assert_eq!(
            *prints.borrow(),
            vec![
                (String::new(), 30.0, 40.0, 0),
                ("a".to_string(), 30.0, 55.0, 2),
                ("b".to_string(), 30.0, 70.0, 2),
                ("c".to_string(), 30.0, 85.0, 2),
            ]
        );
    }

    #[test]
    fn refresh_skips_the_lines_scrolled_off_the_top() {
        let (mut e, prints) = console_engine();
        let console = new_console(&mut e);
        e.set(console, MenuConsole::iConsoleLineSpacing, 10);
        e.set(console, MenuConsole::iConsoleXPos, 1);
        e.set(console, MenuConsole::iConsoleYPos, 50);
        for line in ["a", "b", "c", "d", "e", "f"] {
            push_line(&mut e, console, line);
        }
        e.set(console, MenuConsole::iLastString, 6);
        e.call(0x0071_d410, &args![console]);
        let drawn: Vec<String> = prints.borrow().iter().map(|p| p.0.clone()).collect();
        assert_eq!(drawn, ["c", "d", "e", "f"]);
        assert_eq!(prints.borrow()[0].2, 10.0);
    }

    #[test]
    fn toggle_shows_hidden_console_and_hides_it_otherwise() {
        let (mut e, prints) = console_engine();
        put_string(&mut e, CURSOR_TEXT, "|");
        e.register(0x00ba_8b70, |_, _| Ret::default());
        e.register(0x0070_ecf0, |_, _| ret(0));
        let console = new_console(&mut e);
        e.set(console, MenuConsole::iConsoleXPos, 30);
        e.set(console, MenuConsole::iConsoleYPos, 100);
        // Hidden -> shown: returns 1; the cursor is drawn (the interface
        // test passes) and the lines are redrawn.
        assert_eq!(e.call(0x0071_d580, &args![console]).u32() & 0xff, 1);
        assert_eq!(e.get(console, MenuConsole::cVisible), 1);
        assert_eq!(prints.borrow()[0].0, "|");
        // Shown, interface test fails: hidden, returns 0.
        assert_eq!(e.call(0x0071_d580, &args![console]).u32() & 0xff, 0);
        assert_eq!(e.get(console, MenuConsole::cVisible), 0);
    }

    #[test]
    fn toggle_goes_to_state_2_when_the_menu_stack_says_so() {
        let (mut e, _) = console_engine();
        e.register(0x00ba_8b70, |_, _| Ret::default());
        e.register(0x0070_ecf0, |_, _| ret(1));
        e.register(0x0071_4f70, |_, a| ret(if a[1] == 0 { 3 } else { 0 }));
        let console = new_console(&mut e);
        e.set(console, MenuConsole::cVisible, 1);
        assert_eq!(e.call(0x0071_d580, &args![console]).u32() & 0xff, 0);
        assert_eq!(e.get(console, MenuConsole::cVisible), 2);
        // A different second entry hides it instead.
        e.register(0x0071_4f70, |_, a| ret(if a[1] == 0 { 3 } else { 5 }));
        e.set(console, MenuConsole::cVisible, 1);
        e.call(0x0071_d580, &args![console]);
        assert_eq!(e.get(console, MenuConsole::cVisible), 0);
        // State 2 goes straight to hidden.
        e.set(console, MenuConsole::cVisible, 2);
        e.call(0x0071_d580, &args![console]);
        assert_eq!(e.get(console, MenuConsole::cVisible), 0);
    }

    #[test]
    fn set_visibility_only_acts_when_shown_or_hidden_changes() {
        let (mut e, prints) = console_engine();
        let console = new_console(&mut e);
        e.set(console, MenuConsole::cVisible, 1);
        // 1 -> 2 is still shown: just the field changes.
        e.call_log = Some(vec![]);
        e.call(0x0071_d620, &args![console, 2i8]);
        assert_eq!(e.get(console, MenuConsole::cVisible), 2);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // Shown with the interface test failing draws an empty line, font 0.
        e.register(0x0070_23a0, |_, _| ret(0));
        e.set(console, MenuConsole::cVisible, 0);
        e.call(0x0071_d620, &args![console, 1i8]);
        assert_eq!(prints.borrow()[0].0, "");
        assert_eq!(prints.borrow()[0].3, 0);
        // Hidden: debug text is deleted and the interface field is cleared.
        let interface = e.global::<u32>(INTERFACE_SLOT);
        e.mem.set_u32(interface + 0xf0, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0071_d620, &args![console, 0i8]);
        let order: Vec<u32> = e.call_log.take().unwrap().iter().map(|(a, _)| *a).collect();
        assert!(order.contains(&0x00a0_de80));
        assert_eq!(e.mem.u32(interface + 0xf0), 0);
    }

    #[test]
    fn fn_0071d750_clears_the_field() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x100);
        e.mem.set_u32(block + 0xf0, 7);
        e.call(0x0071_d750, &args![block]);
        assert_eq!(e.mem.u32(block + 0xf0), 0);
    }

    #[test]
    fn entering_menu_mode_redraws_only_while_visible() {
        let (mut e, prints) = console_engine();
        put_string(&mut e, CURSOR_TEXT, "|");
        e.register(0x0070_3310, |_, _| Ret::default());
        let console = new_console(&mut e);
        push_line(&mut e, console, "a");
        e.call_log = Some(vec![]);
        e.call(0x0071_d770, &args![console]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert!(prints.borrow().is_empty());
        e.set(console, MenuConsole::cVisible, 1);
        e.call_log = Some(vec![]);
        e.call(0x0071_d770, &args![console]);
        let log = e.call_log.take().unwrap();
        assert_eq!(prints.borrow()[0].0, "|");
        // The last line moved to the end of the list and `00703310(0)` ran.
        assert_eq!(e.get(console, MenuConsole::iLastString), 1);
        assert!(log.iter().any(|(a, w)| *a == 0x0070_3310 && w[..] == [0]));
    }

    /// The textures `dump textures` collects: a list of two items that pass
    /// the type check, one of which `0071dca0` rejects.
    #[test]
    fn dump_textures_writes_the_header_and_reports() {
        let (mut e, _) = console_engine();
        e.set_global(TES_SINGLETON, 0x0120_0100u32);
        let items: [u32; 3] = [e.mem.alloc(0x40), e.mem.alloc(0x40), e.mem.alloc(0x40)];
        // The items link to each other through offset 0x38 and share a
        // vtable whose +0x9C answers with a name.
        let vtable = e.mem.alloc(0xc0);
        e.mem.set_u32(vtable + 0x9c, 0x0120_0a00);
        e.register(0x0120_0a00, |_, _| ret(0x1234));
        for item in items {
            e.mem.set_u32(item, vtable);
        }
        e.mem.set_u32(items[0] + 0x38, items[1]);
        e.mem.set_u32(items[1] + 0x38, items[2]);
        e.set_global(0x011d_ee00u32, items[0]);
        e.map(0x011d_e000, 0x1000);
        e.register(0x0045_6510, |e, _| ret(e.global::<u32>(0x011d_ee00)));
        e.register(0x0055_b980, |e, a| ret(e.mem.u32(a[0] + 0x38)));
        e.register(0x0043_b1b0, |_, _| Ret::default());
        e.register(0x0043_b300, |_, _| ret(1));
        e.register(0x0059_bb30, |_, a| {
            ret(if a[0] == 0x0120_0100 { 5 } else { 100 })
        });
        // The menu-texture setting byte is 0, so 0071dca0 decides: it
        // rejects the second item.
        let flag = e.mem.alloc(8);
        e.set_global(0x011d_ef00u32, flag);
        e.register(0x0040_8d60, |e, _| ret(e.global::<u32>(0x011d_ef00)));
        e.register(0x0071_dca0, |e, a| {
            ret(if a[1] == e.global::<u32>(0x011d_ee04) {
                1
            } else {
                0
            })
        });
        e.set_global(0x011d_ee04u32, items[1]);
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        // The file object: virtual slot 0x20 says it is open.
        let vtable = e.mem.alloc(0xc0);
        e.mem.set_u32(vtable + 0x20, 0x0120_0900);
        e.set_global(0x011d_ef10u32, vtable);
        e.register(0x0120_0900, |_, _| ret(1));
        e.register(FILE_CONSTRUCT, |e, a| {
            let vtable = e.global::<u32>(0x011d_ef10);
            e.mem.set_u32(a[0], vtable);
            ret(a[0])
        });
        e.register(0x0045_7070, |_, _| ret(0));
        e.register(0x0045_cd60, |_, _| ret(7));
        e.register(SNPRINTF, |e, a| {
            e.mem.set_cstr(a[0], b"header");
            ret(6)
        });
        e.register(STRING_LENGTH_C, |e, a| ret(e.mem.cstr(a[0]).len() as u32));
        e.register(FILE_WRITE, |_, _| Ret::default());
        e.register(FILE_CLOSE, |_, _| Ret::default());
        e.register(PRINT_TO_CONSOLE, |_, _| Ret::default());
        let console = new_console(&mut e);
        // The 0x780-byte frame: the slots at +0x454, the text buffer at +0x2c.
        let frame = next_frame(&mut e, 0x780);
        e.call(0x0071_d800, &args![console]);
        let log = e.call_log.take().unwrap();
        assert_eq!(first_words(&log, MEMSET), vec![frame + 0x454]);
        assert!(first_words(&log, SNPRINTF)
            .iter()
            .all(|a| *a == frame + 0x2c));
        // The header is formatted with the unknown cell and world names and
        // the grid, then one write per kept texture plus header and footer.
        let format = log.iter().find(|(a, _)| *a == SNPRINTF).unwrap();
        assert_eq!(
            format.1[2..],
            [TEXTURES_HEADER_FORMAT, UNKNOWN_NAME, 5, 7, UNKNOWN_NAME]
        );
        let writes = log.iter().filter(|(a, _)| *a == FILE_WRITE).count();
        assert_eq!(writes, 1 + 2 + 1);
        let last = log.last().unwrap();
        assert_eq!(*last, (PRINT_TO_CONSOLE, vec![TEXTURES_DUMPED_MESSAGE]));
    }

    #[test]
    fn dump_textures_reports_a_file_that_cannot_be_opened() {
        let (mut e, _) = console_engine();
        e.register(0x0045_6510, |_, _| ret(0));
        e.register(ALLOCATE, |_, _| ret(0));
        e.register(PRINT_TO_CONSOLE, |_, _| Ret::default());
        let console = new_console(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0071_d800, &args![console]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            *log.last().unwrap(),
            (PRINT_TO_CONSOLE, vec![TEXTURES_FILE_FAILED])
        );
    }

    // ---- MenuConsole::Idle ----------------------------------------------

    const TYPED_SLOT: u32 = 0x011d_8b00;

    /// An engine for `Idle`: the text "typed" is what the debug text gives
    /// back, command comparison works on the text in game memory, and
    /// `sprintf(dst, "%s", src)` copies.
    fn idle_engine(typed: &str) -> (Engine, Prints, Ptr<MenuConsole>) {
        let (mut e, prints) = console_engine();
        put_string(&mut e, CURSOR_TEXT, "|");
        let text = heap_string(&mut e, typed);
        e.set_global(TYPED_SLOT, text);
        e.register(DEBUG_TEXT_READ, |e, a| {
            let text = e.mem.cstr(e.global::<u32>(TYPED_SLOT));
            e.mem.set_cstr(a[1], &text);
            Ret::default()
        });
        e.register(COPY_TEXT, |e, a| {
            let text = e.mem.cstr(a[1]);
            let count = text.len().min(a[2] as usize);
            e.mem.set_cstr(a[0], &text[..count]);
            Ret::default()
        });
        fn prefix(e: &mut Engine, a: &[u32]) -> Ret {
            let (line, name) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            let n = a[2] as usize;
            let equal = (0..n).all(|i| line.get(i) == name.get(i));
            ret(!equal as u32)
        }
        fn whole(e: &mut Engine, a: &[u32]) -> Ret {
            ret((e.mem.cstr(a[0]) != e.mem.cstr(a[1])) as u32)
        }
        e.register(COMMAND_MATCHES_A, prefix);
        e.register(COMMAND_MATCHES_B, prefix);
        e.register(STRING_MATCHES_A, whole);
        e.register(STRING_MATCHES_B, whole);
        e.register(CRT_STRLEN, |e, a| ret(e.mem.cstr(a[0]).len() as u32));
        e.register(SPRINTF, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            ret(text.len() as u32)
        });
        e.register(0x004a_8f20, |_, a| {
            ret((a[0] as i32).min(a[1] as i32) as u32)
        });
        e.register(0x0064_7b70, |_, a| {
            ret((a[0] as i32).max(a[1] as i32) as u32)
        });
        e.register(0x0070_31c0, |_, _| ret(0x00c0_0000));
        let console = new_console(&mut e);
        e.set(console, MenuConsole::cVisible, 1);
        e.set(console, MenuConsole::iConsoleXPos, 30);
        e.set(console, MenuConsole::iConsoleYPos, 100);
        e.set(console, MenuConsole::iConsoleLineSpacing, 15);
        (e, prints, console)
    }

    /// The text of the last thing drawn (the edited line).
    fn last_drawn(prints: &Prints) -> String {
        prints.borrow().last().unwrap().0.clone()
    }

    fn inputs_of(e: &Engine, console: Ptr<MenuConsole>) -> Vec<String> {
        let mut out = Vec::new();
        let mut node = e.mem.u32(console.addr() + INPUTS_LIST);
        while node != 0 {
            out.push(text_of(e, node + 8));
            node = e.mem.u32(node);
        }
        out
    }

    /// Doubles for the script object `Idle` builds to run a line.
    fn script_doubles(e: &mut Engine) {
        for address in [
            0x005a_a0f0u32,
            0x0048_4490,
            0x005a_be50,
            0x005a_c400,
            0x005a_a1a0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn copy_string(e: &mut Engine, a: &[u32]) -> Ret {
        let text = e.mem.cstr(a[1]);
        e.mem.set_cstr(a[0], &text);
        Ret::default()
    }

    #[test]
    fn idle_ignores_messages_while_hidden() {
        let (mut e, prints, console) = idle_engine("ab|");
        e.set(console, MenuConsole::cVisible, 0);
        assert_eq!(e.call(0x0071_b210, &args![console, b'x' as u32]).u32(), 0);
        assert!(prints.borrow().is_empty());
        // Visible, but the interface test fails.
        e.set(console, MenuConsole::cVisible, 1);
        e.register(0x0070_23a0, |_, _| ret(0));
        assert_eq!(e.call(0x0071_b210, &args![console, b'x' as u32]).u32(), 0);
        assert!(prints.borrow().is_empty());
    }

    #[test]
    fn idle_message_zero_does_nothing_and_returns_one() {
        let (mut e, prints, console) = idle_engine("ab|");
        assert_eq!(e.call(0x0071_b210, &args![console, 0u32]).u32(), 1);
        assert!(prints.borrow().is_empty());
    }

    /// Runs `Idle` over a typed line and returns the line it drew afterwards
    /// and the command history.
    fn edit(typed: &str, message: u32) -> (String, Vec<String>) {
        let (mut e, prints, console) = idle_engine(typed);
        assert_eq!(e.call(0x0071_b210, &args![console, message]).u32(), 1);
        let drawn = last_drawn(&prints);
        (drawn, inputs_of(&e, console))
    }

    #[test]
    fn idle_inserts_characters_at_the_cursor() {
        assert_eq!(edit("ab|", b'c' as u32).0, "abc|");
        assert_eq!(edit("a|b", b'x' as u32).0, "ax|b");
        assert_eq!(edit("|", b'q' as u32).0, "q|");
    }

    #[test]
    fn idle_stores_the_edited_line_as_the_newest_history_entry() {
        let (drawn, inputs) = edit("ab|", b'c' as u32);
        assert_eq!(drawn, "abc|");
        assert_eq!(inputs, ["abc|"]);
    }

    #[test]
    fn idle_deletes_before_and_at_the_cursor() {
        assert_eq!(edit("ab|", MESSAGE_DELETE_BEFORE_CURSOR).0, "a|");
        assert_eq!(edit("|ab", MESSAGE_DELETE_BEFORE_CURSOR).0, "|ab");
        assert_eq!(edit("a|b", MESSAGE_DELETE_AT_CURSOR).0, "a|");
        assert_eq!(edit("ab|", MESSAGE_DELETE_AT_CURSOR).0, "ab|");
    }

    #[test]
    fn idle_moves_the_cursor() {
        assert_eq!(edit("ab|", MESSAGE_CURSOR_LEFT).0, "a|b");
        assert_eq!(edit("|ab", MESSAGE_CURSOR_LEFT).0, "|ab");
        assert_eq!(edit("a|b", MESSAGE_CURSOR_RIGHT).0, "ab|");
        assert_eq!(edit("ab|", MESSAGE_CURSOR_RIGHT).0, "ab|");
        assert_eq!(edit("ab|", MESSAGE_CURSOR_TO_START).0, "|ab");
        assert_eq!(edit("a|bc", MESSAGE_CURSOR_TO_START).0, "|abc");
        assert_eq!(edit("|ab", MESSAGE_CURSOR_TO_END).0, "ab|");
        assert_eq!(edit("a|bc", MESSAGE_CURSOR_TO_END).0, "abc|");
    }

    #[test]
    fn idle_recalls_history_with_the_arrow_messages() {
        let (mut e, prints, console) = idle_engine("");
        for line in ["first", "second"] {
            let string = e.mem.alloc(8);
            set_text(&mut e, string, line);
            e.call(0x0071_df20, &args![console.addr() + INPUTS_LIST, string]);
        }
        // One step back: position 1 of the history.
        e.call(0x0071_b210, &args![console, MESSAGE_HISTORY_OLDER]);
        assert_eq!(e.get(console, MenuConsole::iInputsPos), 1);
        assert_eq!(last_drawn(&prints), "second|");
        // The position stops at the last entry.
        e.call(0x0071_b210, &args![console, MESSAGE_HISTORY_OLDER]);
        assert_eq!(e.get(console, MenuConsole::iInputsPos), 1);
        // Forward again: position 0, no entry: the line is empty.
        e.call(0x0071_b210, &args![console, MESSAGE_HISTORY_NEWER]);
        assert_eq!(e.get(console, MenuConsole::iInputsPos), 0);
        assert_eq!(last_drawn(&prints), "|");
    }

    #[test]
    fn idle_scrolls_the_lines_with_the_page_messages() {
        let (mut e, prints, console) = idle_engine("");
        for line in ["a", "b", "c", "d", "e", "f"] {
            push_line(&mut e, console, line);
        }
        e.set(console, MenuConsole::iLastString, 6);
        // Back by visible lines - 1 = 3 from line 6: line 3, clamped to 4.
        e.call(0x0071_b210, &args![console, MESSAGE_SCROLL_BACK]);
        assert_eq!(e.get(console, MenuConsole::iLastString), 4);
        let drawn: Vec<String> = prints.borrow().iter().map(|p| p.0.clone()).collect();
        assert_eq!(drawn[..4], ["a", "b", "c", "d"]);
        e.call(0x0071_b210, &args![console, MESSAGE_SCROLL_FORWARD]);
        assert_eq!(e.get(console, MenuConsole::iLastString), 6);
    }

    #[test]
    fn idle_enter_echoes_remembers_and_runs_the_line_as_a_script() {
        let (mut e, prints, console) = idle_engine("foo|");
        let compiler = Ptr::new(0x00d0_0000);
        e.set(console, MenuConsole::pConsoleCompiler, compiler);
        script_doubles(&mut e);
        let compiled = Rc::new(RefCell::new(String::new()));
        let heard = compiled.clone();
        e.register_double(0x005a_be50, move |e, a| {
            *heard.borrow_mut() = read_string(e, a[1]);
            Ret::default()
        });
        // The 0x28e0-byte frame with `EBP` at its top: the command script at `EBP-0x281c`.
        let frame = next_frame(&mut e, 0x28e0);
        assert_eq!(
            e.call(0x0071_b210, &args![console, MESSAGE_ENTER_LINE])
                .u32(),
            1
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            first_words(&log, 0x005a_be50),
            vec![frame + 0x28e0 - 0x281c]
        );
        // The line without the cursor was echoed, remembered and compiled.
        assert_eq!(lines_of(&e, console), ["foo"]);
        assert_eq!(inputs_of(&e, console), ["foo"]);
        assert_eq!(*compiled.borrow(), "foo");
        assert_eq!(e.get(console, MenuConsole::iInputsPos), 0);
        assert_eq!(e.mem.u8(COMMAND_ENTERED), 1);
        // The cursor is drawn again at the console position.
        assert!(prints
            .borrow()
            .iter()
            .any(|p| *p == ("|".to_string(), 30.0, 100.0, 2)));
        let run = log.iter().find(|(a, _)| *a == 0x005a_c400).unwrap();
        assert_eq!(run.1[1..], [compiler.addr(), 1, 0x00c0_0000]);
    }

    #[test]
    fn idle_enter_on_an_empty_line_does_nothing() {
        let (mut e, prints, console) = idle_engine("|");
        e.call_log = Some(vec![]);
        e.call(0x0071_b210, &args![console, MESSAGE_ENTER_LINE]);
        let log = e.call_log.take().unwrap();
        assert!(lines_of(&e, console).is_empty());
        assert!(prints.borrow().is_empty());
        assert!(!log.iter().any(|(a, _)| *a == 0x005a_c400));
    }

    #[test]
    fn idle_clr_empties_the_lines() {
        let (mut e, _, console) = idle_engine("clr|");
        put_string(&mut e, CLR_COMMAND, "clr");
        push_line(&mut e, console, "old");
        e.set(console, MenuConsole::iLastString, 1);
        e.call(0x0071_b210, &args![console, MESSAGE_ENTER_LINE]);
        assert!(lines_of(&e, console).is_empty());
        assert_eq!(e.get(console, MenuConsole::iLastString), 0);
    }

    #[test]
    fn idle_front_names_the_frontmost_menu() {
        let (mut e, _, console) = idle_engine("front|");
        put_string(&mut e, FRONT_COMMAND, "front");
        put_string(&mut e, FRONTMOST_FORMAT, "Frontmost: %s");
        e.register(0x0071_e290, |_, a| ret(0x1000 + a[0]));
        e.register(0x0072_0e60, |_, a| ret(a[0] + 1));
        e.register(0x0072_6070, |_, a| ret(a[0] + 1));
        e.register(0x0070_0300, |_, a| ret(a[0] + 1));
        let words = Rc::new(RefCell::new(Vec::<u32>::new()));
        let seen = words.clone();
        e.register_double(STRING_FORMAT_LIST, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[2]));
            set_text(e, a[0], &read_string(e, a[1]));
            Ret::default()
        });
        e.call(0x0071_b210, &args![console, MESSAGE_ENTER_LINE]);
        // The name goes through the chain 71e290(1) -> 720e60 -> 726070 ->
        // 700300 and is the argument of the last line printed.
        assert_eq!(*words.borrow().last().unwrap(), 0x1001 + 3);
        assert_eq!(lines_of(&e, console).last().unwrap(), "Frontmost: %s");
    }

    #[test]
    fn idle_stack_lists_the_menu_stack() {
        let (mut e, _, console) = idle_engine("stack|");
        put_string(&mut e, STACK_COMMAND, "stack");
        put_string(&mut e, STACK_ENTRY_FORMAT, "%i: %i");
        e.register(0x0071_4f70, |_, a| ret([5, 6, 0][a[1] as usize]));
        let words = Rc::new(RefCell::new(Vec::<(u32, u32)>::new()));
        let seen = words.clone();
        e.register_double(STRING_FORMAT_LIST, move |e, a| {
            seen.borrow_mut()
                .push((e.mem.u32(a[2]), e.mem.u32(a[2] + 4)));
            set_text(e, a[0], &read_string(e, a[1]));
            Ret::default()
        });
        e.call(0x0071_b210, &args![console, MESSAGE_ENTER_LINE]);
        // (The first line printed is the echo of the entered line.)
        let entries: Vec<(u32, u32)> = words.borrow()[1..].to_vec();
        assert_eq!(entries, [(1, 6), (2, 0)]);
    }

    #[test]
    fn idle_new_and_old_fonts_set_the_flag_on_the_font_array() {
        let (mut e, _, console) = idle_engine("new fonts|");
        put_string(&mut e, NEW_FONTS_COMMAND, "new fonts");
        put_string(&mut e, OLD_FONTS_COMMAND, "old fonts");
        let table = e.mem.alloc(0x40);
        e.set_global(0x011d_8b10u32, table);
        e.register(FONT_ARRAY_GETTER, |e, _| ret(e.global::<u32>(0x011d_8b10)));
        e.call(0x0071_b210, &args![console, MESSAGE_ENTER_LINE]);
        assert_eq!(e.mem.u8(table + 0x20), 1);
        let text = heap_string(&mut e, "old fonts|");
        e.set_global(TYPED_SLOT, text);
        e.call(0x0071_b210, &args![console, MESSAGE_ENTER_LINE]);
        assert_eq!(e.mem.u8(table + 0x20), 0);
    }

    #[test]
    fn idle_records_entered_lines_while_the_batch_recorder_is_on() {
        let (mut e, _, console) = idle_engine("foo|");
        script_doubles(&mut e);
        e.register(STRING_COPY, copy_string);
        e.register(PRINT_TO_CONSOLE, |_, _| Ret::default());
        e.set(console, MenuConsole::bRecordingBatch, true);
        e.set(console, MenuConsole::iRecordingLineCount, 19);
        e.call_log = Some(vec![]);
        e.call(0x0071_b210, &args![console, MESSAGE_ENTER_LINE]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(console, MenuConsole::iRecordingLineCount), 20);
        assert_eq!(read_string(&e, console.addr() + 0x40 + 19 * 100), "foo");
        assert!(log
            .iter()
            .any(|(a, w)| *a == PRINT_TO_CONSOLE && w[..] == [RECORDED_MESSAGE]));
        // The recorder is full now: the next line is not kept.
        let text = heap_string(&mut e, "bar|");
        e.set_global(TYPED_SLOT, text);
        e.call_log = Some(vec![]);
        e.call(0x0071_b210, &args![console, MESSAGE_ENTER_LINE]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(console, MenuConsole::iRecordingLineCount), 20);
        assert!(log
            .iter()
            .any(|(a, w)| *a == PRINT_TO_CONSOLE && w[..] == [RECORDING_FULL_MESSAGE]));
    }

    #[test]
    fn idle_rbat_starts_recording_and_clears_the_slots() {
        let (mut e, _, console) = idle_engine("rbat|");
        put_string(&mut e, RBAT_COMMAND, "rbat");
        e.register(PRINT_TO_CONSOLE, |_, _| Ret::default());
        e.register(STRING_COPY, copy_string);
        e.mem.set_cstr(console.addr() + 0x40, b"stale");
        e.call_log = Some(vec![]);
        e.call(0x0071_b210, &args![console, MESSAGE_ENTER_LINE]);
        let log = e.call_log.take().unwrap();
        assert!(e.get(console, MenuConsole::bRecordingBatch));
        assert_eq!(e.get(console, MenuConsole::iRecordingLineCount), 0);
        assert_eq!(read_string(&e, console.addr() + 0x40), "");
        let messages: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == PRINT_TO_CONSOLE)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(
            messages,
            [RECORDING_STARTED_MESSAGE, RECORDING_HELP_MESSAGE]
        );
    }

    #[test]
    fn idle_rbat_without_a_name_cancels() {
        let (mut e, _, console) = idle_engine("rbat|");
        put_string(&mut e, RBAT_COMMAND, "rbat");
        e.register(PRINT_TO_CONSOLE, |_, _| Ret::default());
        e.register(SSCANF, |_, _| Ret::default());
        e.register(CRT_MEMSET, |_, _| Ret::default());
        e.set(console, MenuConsole::bRecordingBatch, true);
        e.set(console, MenuConsole::iRecordingLineCount, 3);
        e.call_log = Some(vec![]);
        e.call(0x0071_b210, &args![console, MESSAGE_ENTER_LINE]);
        let log = e.call_log.take().unwrap();
        assert!(!e.get(console, MenuConsole::bRecordingBatch));
        assert_eq!(e.get(console, MenuConsole::iRecordingLineCount), 0);
        assert!(log
            .iter()
            .any(|(a, w)| *a == PRINT_TO_CONSOLE && w[..] == [RECORDING_NO_FILENAME_MESSAGE]));
    }

    #[test]
    fn idle_bat_without_a_name_asks_for_one() {
        let (mut e, _, console) = idle_engine("bat|");
        put_string(&mut e, BAT_COMMAND, "bat");
        e.register(PRINT_TO_CONSOLE, |_, _| Ret::default());
        e.register(SSCANF, |_, _| Ret::default());
        e.register(CRT_MEMSET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0071_b210, &args![console, MESSAGE_ENTER_LINE]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(a, w)| *a == PRINT_TO_CONSOLE && w[..] == [FILENAME_REQUIRED_MESSAGE]));
    }

    #[test]
    fn idle_bat_runs_every_line_of_the_file() {
        let (mut e, _, console) = idle_engine("bat x|");
        put_string(&mut e, BAT_COMMAND, "bat");
        e.register(PRINT_TO_CONSOLE, |_, _| Ret::default());
        // sscanf("bat %s") gives the name "x".
        e.register(SSCANF, |e, a| {
            e.mem.set_cstr(a[2], b"x");
            Ret::default()
        });
        e.register(CRT_MEMSET, |_, _| Ret::default());
        e.register(STRING_APPEND_BOUNDED, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRING_LENGTH_C, |e, a| ret(e.mem.cstr(a[0]).len() as u32));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        // A file object that opens and gives three lines, one of them a
        // single character.
        let vtable = e.mem.alloc(0xc0);
        e.mem.set_u32(vtable, 0x0120_0920);
        e.mem.set_u32(vtable + 0x20, 0x0120_0900);
        e.mem.set_u32(vtable + 0x34, 0x0120_0910);
        e.set_global(0x011d_ef10u32, vtable);
        e.set_global(0x011d_ef14u32, 0u32);
        e.register(0x0120_0900, |_, _| ret(1));
        e.register(0x0120_0910, |e, a| {
            let n = e.global::<u32>(0x011d_ef14);
            e.set_global(0x011d_ef14u32, n + 1);
            let lines: [&[u8]; 3] = [b"one\n", b"x", b"two"];
            match lines.get(n as usize) {
                Some(line) => {
                    e.mem.set_cstr(a[1], line);
                    ret(1)
                }
                None => ret(0),
            }
        });
        e.register(0x0120_0920, |_, _| Ret::default());
        e.register(FILE_CONSTRUCT, |e, a| {
            let vtable = e.global::<u32>(0x011d_ef10);
            e.mem.set_u32(a[0], vtable);
            ret(a[0])
        });
        let compiled = Rc::new(RefCell::new(Vec::<String>::new()));
        script_doubles(&mut e);
        let seen = compiled.clone();
        e.register_double(0x005a_be50, move |e, a| {
            seen.borrow_mut().push(read_string(e, a[1]));
            Ret::default()
        });
        e.call(0x0071_b210, &args![console, MESSAGE_ENTER_LINE]);
        // The line feed became a space; the one-character line is skipped.
        assert_eq!(*compiled.borrow(), ["one ", "two"]);
    }

    // ---- the exe's stack frames ----

    /// The local the next `with_stack(size)` on the running thread takes (the called function's
    /// own frame, taken at the stack's current top), with the call log switched on.
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        e.call_log = Some(vec![]);
        e.next_stack_local(size)
    }

    fn first_words(log: &[(u32, Vec<u32>)], address: u32) -> Vec<u32> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w[0])
            .collect()
    }

    #[test]
    fn format_line_keeps_its_locals_at_the_exe_offsets() {
        let (mut e, _) = console_engine();
        put_string(&mut e, 0x0106_fcb4, "ab cd");
        e.register(0x00a1_60c0, |e, a| {
            let text = e.mem.u32(a[1]);
            e.mem.set_u8(text + 2, 0);
            ret(2)
        });
        let console = new_console(&mut e);
        // 0071d0a0 (0x854 bytes): the text at +0x838, the width at +0x2c, the unused word at
        // +0x844, the lines at +0x18.
        let frame = next_frame(&mut e, 0x854);
        e.call(0x0071_d0a0, &args![console, 0x0106_fcb4u32, 0u32]);
        let log = e.call_log.take().unwrap();
        let wrap: Vec<u32> = log.iter().find(|(a, _)| *a == 0x00a1_60c0).unwrap().1[1..4].to_vec();
        assert_eq!(wrap, vec![frame + 0x838, frame + 0x2c, frame + 0x844]);
        assert_eq!(
            log.iter()
                .filter(|(a, _)| *a == 0x0071_df60)
                .map(|(_, w)| w[1])
                .collect::<Vec<u32>>(),
            vec![frame + 0x18; 2]
        );
        assert_eq!(lines_of(&e, console), ["ab", "cd"]);
        // 0071d030 (0x1c bytes): the unused string at +8.
        let frame = next_frame(&mut e, 0x1c);
        e.call(0x0071_d030, &args![console, 0x0106_fcb4u32, 7u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(first_words(&log, STRING_CONSTRUCT)[0], frame + 8);
    }
}
