//! xNVSE's commands built so far, one module per xNVSE `Commands_*.cpp` file, and what they share
//! (xNVSE's `GameAPI.cpp`, `GameForms.cpp` helpers they call).

pub mod algohol;
pub mod array;
pub mod inventory;
pub mod inventory_ref;
pub mod math;
pub mod printf;
pub mod scripting;
pub mod string;
pub mod weapons;

use crate::args::{extract_args_ex, Arg};
use crate::table::Command;
use crate::CommandArgs;
use engine_core::prelude::*;

/// Every built command, for [`crate::table::install`].
pub fn all() -> Vec<&'static Command> {
    inventory::COMMANDS
        .iter()
        .chain(inventory_ref::COMMANDS)
        .chain(weapons::COMMANDS)
        .chain(math::COMMANDS)
        .chain(algohol::COMMANDS)
        .chain(scripting::COMMANDS)
        .chain(string::COMMANDS)
        .chain(array::COMMANDS)
        .chain(crate::functions::COMMANDS)
        .chain(crate::tick::COMMANDS)
        .chain(crate::event_commands::COMMANDS)
        .collect()
}

/// The arguments a read stored, as xNVSE's output variables hold them after `ExtractArgs`: each
/// read argument in its slot, the rest at the value the command initialised it to.
pub struct Read {
    pub ok: bool,
    pub args: Vec<Arg>,
}

impl Read {
    pub fn of(e: &mut Engine, a: &CommandArgs) -> Read {
        let r = extract_args_ex(e, a);
        Read {
            ok: r.ok,
            args: r.args,
        }
    }
    /// Argument `i` as a `float` output (`default` when it was not read).
    pub fn float(&self, i: usize, default: f32) -> f32 {
        self.args.get(i).and_then(Arg::float).unwrap_or(default)
    }
    /// Argument `i` as an integer output (`UInt32`/`SInt32`, the same bits).
    pub fn int(&self, i: usize, default: u32) -> u32 {
        self.args
            .get(i)
            .and_then(Arg::int)
            .map_or(default, |v| v as u32)
    }
    /// Argument `i` as a form output (0 when it was not read).
    pub fn form(&self, i: usize) -> u32 {
        self.args.get(i).and_then(Arg::form).unwrap_or(0)
    }
    /// Argument `i` as a `ScriptLocal *` output (0 when it was not read).
    pub fn local(&self, i: usize) -> u32 {
        match self.args.get(i) {
            Some(Arg::Var(v)) => *v,
            _ => 0,
        }
    }
    /// Argument `i` as a string output.
    pub fn text(&self, i: usize) -> Vec<u8> {
        self.args
            .get(i)
            .and_then(Arg::text)
            .map(<[u8]>::to_vec)
            .unwrap_or_default()
    }
}

/// The form a base-form command works on: the given form's base form when it is a reference
/// (`TryGetREFRParent`), else the calling reference's base form; `None` with neither.
pub fn form_or_caller(e: &mut Engine, form: u32, this_obj: u32) -> Option<u32> {
    let form = try_get_refr_parent(e, form);
    if form != 0 {
        return Some(form);
    }
    (this_obj != 0).then(|| base_form(e, this_obj))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp: Extract_IntAndForm.
/// An integer and an optional form: the integer (0 when not read) and the form as
/// [`form_or_caller`] finds it, `None` when the read failed or there is no form.
pub fn extract_int_and_form(e: &mut Engine, a: &CommandArgs) -> (u32, Option<u32>) {
    let read = Read::of(e, a);
    let value = read.int(0, 0);
    if !read.ok {
        return (value, None);
    }
    (value, form_or_caller(e, read.form(1), a.this_obj))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Extract_FloatAndForm.
pub fn extract_float_and_form(e: &mut Engine, a: &CommandArgs) -> (f32, Option<u32>) {
    let read = Read::of(e, a);
    let value = read.float(0, 0.0);
    if !read.ok {
        return (value, None);
    }
    (value, form_or_caller(e, read.form(1), a.this_obj))
}

/// `__RTDynamicCast` (`Fallout_DynamicCast` (xNVSE), `0x00EC43FB`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;

/// RTTI type descriptors in the exe's data (`GameRTTI_1_4_0_525.inc` (xNVSE)).
pub mod rtti {
    pub const TES_FORM: u32 = 0x0118_3028;
    pub const TES_OBJECT_REFR: u32 = 0x0118_41cc;
    pub const TES_OBJECT_WEAP: u32 = 0x0118_3998;
    pub const TES_OBJECT_ARMO: u32 = 0x0118_3a34;
    pub const TES_BIPED_MODEL_FORM: u32 = 0x0118_3978;
    pub const TES_ENCHANTABLE_FORM: u32 = 0x0118_39b4;
    pub const BGS_AMMO_FORM: u32 = 0x0118_6890;
    pub const BGS_REPAIR_ITEM_LIST: u32 = 0x0118_6a64;
    pub const BGS_LIST_FORM: u32 = 0x0118_63d4;
    pub const BGS_PROJECTILE: u32 = 0x0118_6228;
    pub const MAGIC_ITEM: u32 = 0x0118_3140;
    pub const SPELL_ITEM: u32 = 0x0118_3060;
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
            rtti::TES_OBJECT_WEAP => (256, 0),
            rtti::TES_OBJECT_ARMO => (512, 0),
            rtti::TES_BIPED_MODEL_FORM => (1024, 0x700),
            rtti::TES_ENCHANTABLE_FORM => (2048, 0x780),
            rtti::BGS_AMMO_FORM => (4096, 0x7a0),
            rtti::BGS_REPAIR_ITEM_LIST => (8192, 0x7c0),
            rtti::BGS_LIST_FORM => (16384, 0),
            rtti::BGS_PROJECTILE => (32768, 0),
            rtti::SPELL_ITEM => (65536, 0),
            rtti::MAGIC_ITEM => (65536 | 131072, 0),
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

    /// A command call's guest data: the compiled line, its offset word, the result, the script
    /// (its reference list at `+0x44`, its variable list at `+0x4C`) and its event list.
    pub const DATA: u32 = 0x0300_0000;
    pub const OFFSET: u32 = 0x0300_0f00;
    pub const RESULT: u32 = 0x0300_0f08;
    pub const SCRIPT: u32 = 0x0300_1000;
    pub const LIST: u32 = 0x0300_2000;

    /// [`engine`] with the command table installed and room for a call's data.
    pub fn engine_with_table() -> Engine {
        let mut e = engine();
        e.map(0x0118_e000, 0x9000);
        e.map(0x0300_0000, 0x10000);
        crate::table::install(&mut e);
        e
    }

    /// Compiled arguments: an immediate double (`z`), an immediate integer (`n`), a string, a
    /// local (`f` and its id), a reference of the script's list (`r` and its index).
    pub fn z(v: f64) -> Vec<u8> {
        let mut b = vec![b'z'];
        b.extend_from_slice(&v.to_le_bytes());
        b
    }
    pub fn n(v: i32) -> Vec<u8> {
        let mut b = vec![b'n'];
        b.extend_from_slice(&v.to_le_bytes());
        b
    }
    pub fn s(text: &str) -> Vec<u8> {
        let mut b = (text.len() as u16).to_le_bytes().to_vec();
        b.extend_from_slice(text.as_bytes());
        b
    }
    pub fn local(id: u16) -> Vec<u8> {
        let mut b = vec![b'f'];
        b.extend_from_slice(&id.to_le_bytes());
        b
    }
    pub fn r(index: u16) -> Vec<u8> {
        let mut b = vec![b'r'];
        b.extend_from_slice(&index.to_le_bytes());
        b
    }

    /// Calls `opcode`'s execute through its record as `EvaluateLine` does, with the given
    /// arguments; answers the handler's `al` and the result double (it starts at 0, as the
    /// seams zero it).
    pub fn call(e: &mut Engine, opcode: u32, this_obj: u32, args: &[Vec<u8>]) -> (u32, f64) {
        e.mem.set_u16(DATA, opcode as u16);
        let bytes: Vec<u8> = args.concat();
        e.mem.set_u16(DATA + 2, bytes.len() as u16 + 2);
        e.mem.set_u16(DATA + 4, args.len() as u16);
        for (i, b) in bytes.iter().enumerate() {
            e.mem.set_u8(DATA + 6 + i as u32, *b);
        }
        e.mem.set_u32(OFFSET, 4);
        e.mem.set_f64(RESULT, 0.0);
        let record = crate::table::record_of(e, opcode);
        assert_ne!(record, 0, "{opcode:04x}");
        let params = e.mem.u32(record + 0x14);
        let execute = e.mem.u32(record + 0x18);
        let ok = e
            .call(
                execute,
                &[params, DATA, this_obj, 0, SCRIPT, LIST, RESULT, OFFSET],
            )
            .eax;
        (ok & 0xff, e.mem.f64(RESULT))
    }

    /// Calls `opcode`'s condition eval with `this_obj` and the two parameters.
    pub fn eval(e: &mut Engine, opcode: u32, this_obj: u32, arg1: u32, arg2: u32) -> f64 {
        let record = crate::table::record_of(e, opcode);
        let eval = e.mem.u32(record + 0x20);
        assert_ne!(eval, 0, "{opcode:04x} has no eval");
        assert_eq!(e.mem.u32(record + 0x24), 1, "a condition-capable record");
        e.mem.set_f64(RESULT, 99.0);
        e.call(eval, &[this_obj, arg1, arg2, RESULT]);
        e.mem.f64(RESULT)
    }

    /// Adds local `id` named `name` to the script's variable list and the event list, holding
    /// `value`; answers the `ScriptLocal`'s address.
    pub fn add_local(e: &mut Engine, id: u16, name: &str, value: f64) -> u32 {
        let id32 = u32::from(id);
        let local = 0x0300_6000 + id32 * 0x20;
        e.mem.set_u32(local, id32);
        e.mem.set_f64(local + 8, value);
        let node = 0x0300_4000 + id32 * 0x10;
        e.mem.set_u32(node, local);
        e.mem.set_u32(node + 4, 0);
        append(e, LIST + 0xc, node, false);
        let info = 0x0300_8000 + id32 * 0x40;
        e.mem.set_u32(info, id32);
        e.mem.set_u32(info + 0x18, info + 0x20);
        for (i, b) in name.bytes().enumerate() {
            e.mem.set_u8(info + 0x20 + i as u32, b);
        }
        e.mem.set_u8(info + 0x20 + name.len() as u32, 0);
        let var_node = 0x0300_c000 + id32 * 8;
        e.mem.set_u32(var_node, info);
        e.mem.set_u32(var_node + 4, 0);
        append(e, SCRIPT + 0x4c, var_node, true);
        local
    }

    /// Makes `form` reference `index` (from 1) of the script's reference list.
    pub fn set_script_ref(e: &mut Engine, index: u16, form: u32) {
        let i = u32::from(index);
        let ref_var = 0x0300_a000 + i * 0x10;
        e.mem.set_u32(ref_var + 8, form);
        e.mem.set_u32(ref_var + 0xc, 0);
        let mut node = SCRIPT + 0x44;
        for at in 1..i {
            let next = e.mem.u32(node + 4);
            node = if next == 0 {
                let made = 0x0300_b000 + (at + 1) * 8;
                e.mem.set_u32(node + 4, made);
                made
            } else {
                next
            };
        }
        e.mem.set_u32(node, ref_var);
    }

    /// Appends `node` to a list whose head is at `head`: a pointer to the first node, or (with
    /// `embedded`) the first node itself.
    fn append(e: &mut Engine, head: u32, node: u32, embedded: bool) {
        let mut at = if embedded {
            if e.mem.u32(head) == 0 {
                e.mem.set_u32(head, e.mem.u32(node));
                return;
            }
            head
        } else {
            let first = e.mem.u32(head);
            if first == 0 {
                e.mem.set_u32(head, node);
                return;
            }
            first
        };
        while e.mem.u32(at + 4) != 0 {
            at = e.mem.u32(at + 4);
        }
        e.mem.set_u32(at + 4, node);
    }

    /// The value a local holds.
    pub fn local_value(e: &Engine, local: u32) -> f64 {
        e.mem.f64(local + 8)
    }

    /// A value the real game printed times 2^40 (`real-results`): the value itself.
    pub fn scaled(v: f64) -> f64 {
        v / 1_099_511_627_776.0
    }
}
