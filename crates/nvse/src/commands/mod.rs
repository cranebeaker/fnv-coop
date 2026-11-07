//! xNVSE's commands built so far, one module per xNVSE `Commands_*.cpp` file, and what they share
//! (xNVSE's `GameAPI.cpp`, `GameForms.cpp` helpers they call).

pub mod inventory;

use crate::table::Command;
use engine_core::prelude::*;

/// Every built command, for [`crate::table::install`].
pub fn all() -> Vec<&'static Command> {
    inventory::COMMANDS.iter().collect()
}

/// `__RTDynamicCast` (`Fallout_DynamicCast` (xNVSE), `0x00EC43FB`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;

/// RTTI type descriptors in the exe's data (`GameRTTI_1_4_0_525.inc` (xNVSE)).
pub mod rtti {
    pub const TES_FORM: u32 = 0x0118_3028;
    pub const TES_OBJECT_REFR: u32 = 0x0118_41cc;
    pub const TES_WEIGHT_FORM: u32 = 0x0118_3274;
    pub const TES_HEALTH_FORM: u32 = 0x0118_6c3c;
    pub const TES_VALUE_FORM: u32 = 0x0118_6b6c;
    pub const TES_AMMO: u32 = 0x0118_40c4;
    pub const BGS_EQUIP_TYPE: u32 = 0x0118_3290;
    pub const BGS_CLIP_ROUNDS_FORM: u32 = 0x0118_68d4;
    pub const TES_ATTACK_DAMAGE_FORM: u32 = 0x0118_6c90;
}

/// `DYNAMIC_CAST(obj, from, to)` (xNVSE): the exe's `__RTDynamicCast`; 0 for a null object or
/// another class.
pub fn dynamic_cast(e: &mut Engine, object: u32, from: u32, to: u32) -> u32 {
    if object == 0 {
        return 0;
    }
    e.call(DYNAMIC_CAST, &args![object, 0u32, from, to, 0u32])
        .u32()
}

/// `TESObjectREFR::baseForm` (xNVSE), `+0x20`.
pub fn base_form(e: &Engine, reference: u32) -> u32 {
    e.mem.u32(reference + 0x20)
}

/// `TESForm::TryGetREFRParent` (xNVSE, `GameForms.cpp`): a reference's base form, any other form
/// itself; 0 stays 0.
pub fn try_get_refr_parent(e: &mut Engine, form: u32) -> u32 {
    if form == 0 {
        return 0;
    }
    let refr = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_OBJECT_REFR);
    if refr != 0 && base_form(e, refr) != 0 {
        base_form(e, refr)
    } else {
        form
    }
}

/// `g_consoleOpen` (xNVSE), the byte at `0x11DEA2E`.
const CONSOLE_OPEN: u32 = 0x011d_ea2e;
/// The byte of the running thread's TLS block that is set while a console line runs.
const TLS_CONSOLE_LINE: u32 = 0x268;

/// `IsConsoleMode` (xNVSE, `GameAPI.cpp`): the console is open and the running thread is running
/// a console line (its TLS block's byte `+0x268`).
pub fn is_console_mode(e: &mut Engine) -> bool {
    if e.mem.u8(CONSOLE_OPEN) == 0 {
        return false;
    }
    let tls = e.tls();
    tls != 0 && e.mem.u8(tls + TLS_CONSOLE_LINE) != 0
}

/// `ConsoleManager_GetSingleton` (xNVSE, `0x0071B160`, `MenuConsole::Instance`) and
/// `ConsoleManager::Print` (`0x0071D0A0`, `thiscall`: the format and a `va_list`).
const CONSOLE_INSTANCE: u32 = 0x0071_b160;
const CONSOLE_PRINT: u32 = 0x0071_d0a0;

/// `Console_Print(fmt, ...)` (xNVSE): the game's console prints `format` with `words` (a
/// `va_list` on the guest stack), through the console's own formatting. xNVSE prints nothing
/// before it has recorded the main thread at the first frame; a console line can only run after
/// that, and every caller here prints only in console mode.
pub fn console_print(e: &mut Engine, format: &str, words: &[u32]) {
    let console = e.call(CONSOLE_INSTANCE, &args![1u32]).u32();
    if console == 0 {
        return;
    }
    let text = crate::table::guest_str(e, format.as_bytes());
    let size = 4 * words.len().max(1) as u32;
    e.with_stack(size, |e, list| {
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(list.addr() + 4 * i as u32, *w);
        }
        e.call(CONSOLE_PRINT, &args![console, text, list]);
    });
}

/// A double as the two words a `va_list` holds it in.
pub fn double_words(v: f64) -> [u32; 2] {
    let bits = v.to_bits();
    [bits as u32, (bits >> 32) as u32]
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    /// A fake `__RTDynamicCast`: an object whose word at `+0x7f0` lists (as a bit per class
    /// below) what it is answers the cast to those classes, at the class's place in it
    /// ([`component`]).
    pub fn fake_cast(e: &mut Engine, a: &[u32]) -> Ret {
        let (object, to) = (a[0], a[3]);
        let (bit, at) = component(to);
        let is = e.mem.u32(object + 0x7f0) & bit != 0;
        Ret {
            eax: if is { object + at } else { 0 },
            ..Ret::default()
        }
    }

    /// The fake cast's bit for a class and where the class lies in an object.
    pub fn component(rtti: u32) -> (u32, u32) {
        match rtti {
            rtti::TES_OBJECT_REFR => (1, 0),
            rtti::TES_WEIGHT_FORM => (2, 0x100),
            rtti::TES_HEALTH_FORM => (4, 0x200),
            rtti::TES_VALUE_FORM => (8, 0x300),
            rtti::TES_AMMO => (16, 0),
            rtti::BGS_EQUIP_TYPE => (32, 0x400),
            rtti::BGS_CLIP_ROUNDS_FORM => (64, 0x500),
            rtti::TES_ATTACK_DAMAGE_FORM => (128, 0x600),
            _ => (0, 0),
        }
    }

    /// An engine with the fake cast, a closed console and memory for forms at `0x0200_0000`.
    pub fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0200_0000, 0x10000);
        e.map(CONSOLE_OPEN & !0xfff, 0x1000);
        e.register(DYNAMIC_CAST, fake_cast);
        e
    }
}
