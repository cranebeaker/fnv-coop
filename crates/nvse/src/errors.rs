//! xNVSE's run-time script errors (`ShowRuntimeError`, `vShowRuntimeError`, `Utilities.cpp`
//! (xNVSE)): a header naming the script, its editor id and its mod, then the message, printed to
//! the game's console (split in 500-character pieces past 511, `Console_Print_Long`). Like
//! xNVSE's `Console_Print`, nothing is printed before the first frame of the main loop
//! recorded the main thread ([`crate::tick`]). xNVSE also writes the text to `nvse.log`; the
//! port writes no `nvse.log`.

use crate::Nvse;
use engine_core::prelude::*;

/// `DataHandler`'s singleton (`0x011C3F2C`) and its mod list (`modList`, `+0x210`: the list of
/// files, then the loaded count at `+0x218`); a file's name is at `+0x20` (`ModInfo`).
const DATA_HANDLER: u32 = 0x011c_3f2c;
const MOD_LIST: u32 = 0x210;
const LOADED_COUNT: u32 = 0x218;
const MOD_NAME: u32 = 0x20;

/// `GetModName(form)`: the form's first file, else by its mod index (`"In-game console"` for a
/// script of index `0xFF`, `"Dynamic form"` for another form), `"Unknown"` for an index with no
/// name.
pub fn mod_name(e: &Engine, form: u32) -> Vec<u8> {
    if form == 0 {
        return b"Unknown or deleted script".to_vec();
    }
    let first = e.mem.u32(form + 0x10);
    if first != 0 {
        return crate::table::guest_text(e, first + MOD_NAME);
    }
    let index = e.mem.u32(form + 0xc) >> 24;
    if index == 0xff {
        return if e.mem.u8(form + 4) == 0x11 {
            b"In-game console".to_vec()
        } else {
            b"Dynamic form".to_vec()
        };
    }
    let name = nth_mod_name(e, index);
    if name.is_empty() {
        b"Unknown".to_vec()
    } else {
        name
    }
}

/// `DataHandler::GetNthModName`: the `index`-th file of the list (xNVSE's `GetActiveModList`
/// counts every file of the list, all of which it takes as loaded) when below the loaded count.
pub fn nth_mod_name(e: &Engine, index: u32) -> Vec<u8> {
    let handler = e.mem.u32(DATA_HANDLER);
    if handler == 0 || index >= e.mem.u8(handler + LOADED_COUNT).into() {
        return Vec::new();
    }
    let mut node = handler + MOD_LIST;
    for _ in 0..index {
        node = e.mem.u32(node + 4);
        if node == 0 {
            return Vec::new();
        }
    }
    let info = e.mem.u32(node);
    if info == 0 {
        Vec::new()
    } else {
        crate::table::guest_text(e, info + MOD_NAME)
    }
}

/// A form's name (`TESForm::GetName`, the `+0x130` virtual function: a script's editor id).
fn form_name(e: &mut Engine, form: u32) -> Vec<u8> {
    let at = e.vcall(form, 0x130, &args![]).u32();
    crate::table::guest_text(e, at)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Utilities.cpp: vShowRuntimeError.
/// The header and the message, as xNVSE prints them.
pub fn text(e: &mut Engine, script: u32, message: &str) -> String {
    let modname = String::from_utf8_lossy(&mod_name(e, script)).into_owned();
    let (mut id, mut name) = if script != 0 {
        (e.mem.u32(script + 0xc), form_name(e, script))
    } else {
        (0, Vec::new())
    };
    if script != 0 && crate::lambda::is_lambda(e, script) {
        let parent_list = crate::lambda::parent_event_list(e, script);
        let parent = if parent_list != 0 {
            e.mem.u32(parent_list)
        } else {
            0
        };
        if parent != 0 {
            id = e.mem.u32(parent + 0xc);
            name = form_name(e, parent);
        }
    }
    if name.is_empty() {
        format!("Error in script {id:08X} in mod {modname}\n{message}")
    } else {
        let name = String::from_utf8_lossy(&name);
        format!("Error in script {id:08X} ({name}) in mod {modname}\n{message}")
    }
}

/// `ShowRuntimeError(script, ...)`.
pub fn show(e: &mut Engine, script: u32, message: &str) {
    // Made only for the console, which takes nothing before the first frame.
    if !e.ext::<Nvse>().main_thread_recorded {
        return;
    }
    let text = text(e, script, message);
    print_long(e, text.as_bytes());
}

/// `Console_Print_Str`: below 512 bytes in one line, else `Console_Print_Long`'s pieces.
pub fn print_long(e: &mut Engine, text: &[u8]) {
    if !e.ext::<Nvse>().main_thread_recorded {
        return;
    }
    if text.len() < 512 {
        print(e, "%s", text);
        return;
    }
    let lines = text.len() / 500;
    for i in 0..lines {
        print(e, "%s ...", &text[i * 500..(i + 1) * 500]);
    }
    print(e, "%s", &text[lines * 500..]);
}

/// `Console_Print(format, text)`, the text copied to the guest stack for the call.
fn print(e: &mut Engine, format: &str, text: &[u8]) {
    let size = (text.len() as u32 + 1).next_multiple_of(4);
    e.with_stack(size, |e, at| {
        e.mem.write(at.addr(), text);
        e.mem.set_u8(at.addr() + text.len() as u32, 0);
        crate::commands::console_print(e, format, &[at.addr()]);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const AREA: u32 = 0x0200_0000;
    const NAME_FN: u32 = 0x7eff_e100;

    fn editor_id(_: &mut Engine, _: &[u32]) -> Ret {
        Ret {
            eax: AREA + 0x800,
            ..Ret::default()
        }
    }

    /// The header as xNVSE writes it: the id, the editor id in brackets when there is one, the
    /// first file's name; for no script, `Unknown or deleted script` (the real game's
    /// `Error in script 00000000 in mod Unknown or deleted script`, `real-results/udf3.txt`).
    #[test]
    fn the_header_names_the_script_and_its_file() {
        let mut e = Engine::new();
        e.map(AREA, 0x1000);
        let (script, file, vtable) = (AREA, AREA + 0x100, AREA + 0x400);
        e.mem.set_u32(script, vtable);
        e.mem.set_u32(vtable + 0x130, NAME_FN);
        e.register(NAME_FN, editor_id);
        e.mem.write(AREA + 0x800, b"MyQuestScript\0");
        e.mem.set_u32(script + 0xc, 0x0500_1234);
        e.mem.set_u32(script + 0x10, file);
        e.mem.write(file + MOD_NAME, b"Test.esp\0");
        assert_eq!(
            text(&mut e, script, "Max nest depth 30 exceeded in function call."),
            "Error in script 05001234 (MyQuestScript) in mod Test.esp\nMax nest depth 30 exceeded in function call."
        );
        assert_eq!(
            text(&mut e, 0, "Max nest depth 30 exceeded in function call."),
            "Error in script 00000000 in mod Unknown or deleted script\nMax nest depth 30 exceeded in function call."
        );
    }
}
