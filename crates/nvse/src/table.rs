//! The script command table in guest memory (ADR-0016 §2; build step I-2), as xNVSE builds it in
//! `CommandTable::Init` (`CommandTable.cpp` (xNVSE)): a copy of the exe's 0x280 script records
//! (`0x1000..0x127f`, from `0x01190910`; no static initializer writes them), xNVSE's padding
//! command up to `0x1400`, then xNVSE's 654 commands. The engine's
//! [`ScriptTables`] then point at the copy, which is what xNVSE's `ApplyPatch` does to the exe's
//! immediates: every reader (`GetFunctionDef`, the expression tokenizer, the condition code, the
//! parameter code) sees the extended table.
//!
//! A record (`CommandInfo` (xNVSE), 0x28 bytes): `+0x00` long name, `+0x04` short name, `+0x08`
//! opcode, `+0x0C` help text, `+0x10` needs a reference (u16), `+0x12` parameter count (u16),
//! `+0x14` parameters (`ParamInfo`, 12 bytes each: name, type, optional), `+0x18` execute,
//! `+0x1C` parse, `+0x20` eval, `+0x24` flags. Return types are kept beside the table, by opcode,
//! as xNVSE keeps them (`CommandMetadata`).
//!
//! A command not built yet keeps its record (its names, so the script error log names it) with no
//! handler and no parameters: a line that calls it fails and the script stops for that run, as
//! it does without the DLL. Its record is completed when its command is built.

use crate::names::{FIRST_OPCODE, NAMES};
use crate::Nvse;
use engine_core::extend::{ScriptTables, EVAL_BASE, EXECUTE_BASE, ROUTINE_BASE};
use engine_core::mem::BUILTIN_DATA;
use engine_core::prelude::*;
use std::collections::HashMap;

/// Where the records go: the start of the built-ins' data.
pub const TABLE_AT: u32 = BUILTIN_DATA.start;
/// Room for records (0x3333 of them), then the strings and parameter lists.
pub const TABLE_ROOM: u32 = 0x8_0000;
/// The built-ins' strings and parameter lists, from here up.
pub const DATA_AT: u32 = TABLE_AT + TABLE_ROOM;

/// xNVSE's `Cmd_Default_Execute` (xNVSE): returns true. The execute word of its padding command.
pub const DEFAULT_EXECUTE: u32 = ROUTINE_BASE;
/// xNVSE's `Cmd_Default_Parse` (xNVSE): the game's default parse routine,
/// `g_defaultParseCommand` (`005b1ba0`), with the same four words. The parse word of xNVSE's
/// commands and of its padding command.
pub const DEFAULT_PARSE: u32 = ROUTINE_BASE + 4;
/// The exe's default parse routine.
const EXE_DEFAULT_PARSE: u32 = 0x005b_1ba0;

/// The console command table (`0x0118e8e0`, opcodes `0x100..0x1cc`): xNVSE re-exports 20 of
/// its records as script commands.
const CONSOLE_TABLE: u32 = 0x0118_e8e0;
const CONSOLE_COUNT: u32 = 0xcd;

/// The size of one record.
pub const RECORD: u32 = ScriptTables::RECORD;

/// A command's return type (`CommandReturnType` (xNVSE)): how `Let`, `eval` and the expression
/// evaluator read the result double.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Returns {
    /// A number (`kRetnType_Default`).
    #[default]
    Default,
    /// A form id in the low word (`kRetnType_Form`).
    Form,
    /// A string variable's id (`kRetnType_String`).
    String,
    /// An array variable's id (`kRetnType_Array`).
    Array,
    /// An array index (`kRetnType_ArrayIndex`).
    ArrayIndex,
    /// Decided per call (`kRetnType_Ambiguous`).
    Ambiguous,
}

/// One parameter (`ParamInfo` (xNVSE)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Param {
    /// `typeStr`: the parameter's name in the compiler's messages.
    pub name: &'static str,
    /// `typeID` (`kParamType_*`).
    pub kind: u32,
    /// `isOptional`.
    pub optional: bool,
}

/// A built command: its record's data and its Rust handlers.
#[derive(Clone, Copy, Debug)]
pub struct Command {
    pub opcode: u32,
    pub name: &'static str,
    /// Empty for none.
    pub short: &'static str,
    pub help: &'static str,
    pub needs_ref: bool,
    pub params: &'static [Param],
    /// `+0x18`: called with the eight words of [`crate::CommandArgs`].
    pub execute: AbiFn,
    /// `+0x20`: a condition's eval, called with [`crate::EvalArgs`]; `None` for a command that
    /// is not condition-capable.
    pub eval: Option<AbiFn>,
    pub returns: Returns,
}

/// The table's state on the engine.
#[derive(Default)]
pub struct State {
    /// The next free byte of the built-ins' data, and the end of what is mapped of it.
    top: u32,
    mapped: u32,
    /// Strings already written (`""` once for every empty name).
    strings: HashMap<Vec<u8>, u32>,
    /// Return types by opcode, from `0x1000`.
    pub returns: Vec<Returns>,
}

/// Vanilla commands xNVSE records as returning forms (`AddCommandsV1`, `CommandTable.cpp`
/// (xNVSE)). Read only by xNVSE's evaluator and `Let`, which vanilla content never reaches: in
/// every preset (ADR-0016 §2).
const VANILLA_FORM_RESULTS: [u32; 13] = [
    0x1025, // PlaceAtMe
    0x10cd, // GetActionRef
    0x10ce, // GetSelf
    0x10cf, // GetContainer
    0x10e8, // GetCombatTarget
    0x10e9, // GetPackageTarget
    0x1113, // GetParentRef
    0x116b, // GetLinkedRef
    0x11bd, // PlaceAtMeHealthPercent
    0x11cf, // GetPlayerGrabbedRef
    0x124e, // GetOwnerLastTarget
    0x1265, // ObjectUnderTheReticule
    0x11d1, // PlaceLeveledActorAtMe
];

/// `Cmd_Default_Execute` (xNVSE): true.
fn default_execute(_: &mut Engine, _: &[u32]) -> Ret {
    crate::handled(true)
}

/// `Cmd_Default_Parse` (xNVSE): `g_defaultParseCommand(numParams, paramInfo, lineBuf,
/// scriptBuf)`, the exe's `005b1ba0`.
fn default_parse(e: &mut Engine, words: &[u32]) -> Ret {
    let mut four = [0u32; 4];
    for (to, from) in four.iter_mut().zip(words) {
        *to = *from;
    }
    e.call(EXE_DEFAULT_PARSE, &four)
}

/// `count` bytes of the built-ins' data, aligned to 4, mapped as needed.
pub fn alloc(e: &mut Engine, count: u32) -> u32 {
    let state = &mut e.ext::<Nvse>().table;
    if state.top == 0 {
        state.top = DATA_AT;
        state.mapped = DATA_AT;
    }
    let at = state.top;
    let end = at + count.next_multiple_of(4);
    assert!(end <= BUILTIN_DATA.end, "the built-ins' data is full");
    let mapped = state.mapped;
    state.top = end;
    if end > mapped {
        let to = end.next_multiple_of(0x1000);
        e.ext::<Nvse>().table.mapped = to;
        e.map(mapped, to - mapped);
    }
    at
}

/// A NUL-terminated copy of `text` in the built-ins' data, written once per text.
pub fn guest_str(e: &mut Engine, text: &[u8]) -> u32 {
    if let Some(&at) = e.ext::<Nvse>().table.strings.get(text) {
        return at;
    }
    let at = alloc(e, text.len() as u32 + 1);
    for (i, b) in text.iter().enumerate() {
        e.mem.set_u8(at + i as u32, *b);
    }
    e.mem.set_u8(at + text.len() as u32, 0);
    e.ext::<Nvse>().table.strings.insert(text.to_vec(), at);
    at
}

/// A parameter list in the built-ins' data (12 bytes each: name, type, optional); 0 for none.
fn guest_params(e: &mut Engine, params: &[Param]) -> u32 {
    if params.is_empty() {
        return 0;
    }
    let at = alloc(e, 12 * params.len() as u32);
    for (i, p) in params.iter().enumerate() {
        let name = guest_str(e, p.name.as_bytes());
        let slot = at + 12 * i as u32;
        e.mem.set_u32(slot, name);
        e.mem.set_u32(slot + 4, p.kind);
        e.mem.set_u32(slot + 8, u32::from(p.optional));
    }
    at
}

/// The fields of one record.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Record {
    pub long_name: u32,
    pub short_name: u32,
    pub opcode: u32,
    pub help: u32,
    pub needs_ref: u16,
    pub param_count: u16,
    pub params: u32,
    pub execute: u32,
    pub parse: u32,
    pub eval: u32,
    pub flags: u32,
}

impl Record {
    pub fn read(e: &Engine, at: u32) -> Record {
        Record {
            long_name: e.mem.u32(at),
            short_name: e.mem.u32(at + 4),
            opcode: e.mem.u32(at + 8),
            help: e.mem.u32(at + 0xc),
            needs_ref: e.mem.u16(at + 0x10),
            param_count: e.mem.u16(at + 0x12),
            params: e.mem.u32(at + 0x14),
            execute: e.mem.u32(at + 0x18),
            parse: e.mem.u32(at + 0x1c),
            eval: e.mem.u32(at + 0x20),
            flags: e.mem.u32(at + 0x24),
        }
    }

    pub fn write(self, e: &mut Engine, at: u32) {
        e.mem.set_u32(at, self.long_name);
        e.mem.set_u32(at + 4, self.short_name);
        e.mem.set_u32(at + 8, self.opcode);
        e.mem.set_u32(at + 0xc, self.help);
        e.mem.set_u16(at + 0x10, self.needs_ref);
        e.mem.set_u16(at + 0x12, self.param_count);
        e.mem.set_u32(at + 0x14, self.params);
        e.mem.set_u32(at + 0x18, self.execute);
        e.mem.set_u32(at + 0x1c, self.parse);
        e.mem.set_u32(at + 0x20, self.eval);
        e.mem.set_u32(at + 0x24, self.flags);
    }
}

/// The NUL-terminated text at `at` (0: none).
pub fn guest_text(e: &Engine, at: u32) -> Vec<u8> {
    let mut out = Vec::new();
    if at == 0 {
        return out;
    }
    let mut p = at;
    loop {
        let b = e.mem.u8(p);
        if b == 0 {
            return out;
        }
        out.push(b);
        p += 1;
    }
}

/// The console record whose long name is `name`, compared without case (xNVSE's
/// `g_consoleCommands.GetByName`).
fn console_record(e: &Engine, name: &[u8]) -> Option<u32> {
    (0..CONSOLE_COUNT)
        .map(|i| CONSOLE_TABLE + i * RECORD)
        .find(|&at| guest_text(e, e.mem.u32(at)).eq_ignore_ascii_case(name))
}

/// The execute address of built command `opcode` of xNVSE's range.
pub fn execute_address(opcode: u32) -> u32 {
    EXECUTE_BASE + 4 * (opcode - FIRST_OPCODE)
}

/// The eval address of built command `opcode` of xNVSE's range.
pub fn eval_address(opcode: u32) -> u32 {
    EVAL_BASE + 4 * (opcode - FIRST_OPCODE)
}

/// Builds the table (xNVSE's `CommandTable::Init`, the script table's part) and moves the
/// engine's [`ScriptTables`] onto it.
pub fn install(e: &mut Engine) {
    e.register(DEFAULT_EXECUTE, default_execute);
    e.register(DEFAULT_PARSE, default_parse);
    let exe = ScriptTables::EXE;
    let vanilla = (exe.max_opcode - ScriptTables::FIRST_OPCODE) as usize;
    let max_opcode = FIRST_OPCODE + NAMES.len() as u32;
    let count = max_opcode - ScriptTables::FIRST_OPCODE;
    // The records and the one past the end, which the exe's `ScriptCompiler` constructor
    // (`005aea50`) reads (`end + 8`).
    let size = (count + 1) * RECORD;
    assert!(size <= TABLE_ROOM);
    e.map(TABLE_AT, size.next_multiple_of(0x1000));
    // `g_scriptCommands.Read`: the exe's records as they are.
    let bytes = e.mem.bytes(exe.start, vanilla as u32 * RECORD);
    for (i, b) in bytes.iter().enumerate() {
        e.mem.set_u8(TABLE_AT + i as u32, *b);
    }
    let mut returns = vec![Returns::Default; count as usize];
    for opcode in VANILLA_FORM_RESULTS {
        returns[(opcode - ScriptTables::FIRST_OPCODE) as usize] = Returns::Form;
    }
    // `PadTo(kNVSEOpcodeStart)`: xNVSE's padding command.
    let empty = guest_str(e, b"");
    let padding_help = guest_str(e, b"command used for padding");
    for opcode in exe.max_opcode..FIRST_OPCODE {
        Record {
            long_name: empty,
            short_name: empty,
            opcode,
            help: padding_help,
            execute: DEFAULT_EXECUTE,
            parse: DEFAULT_PARSE,
            ..Record::default()
        }
        .write(e, TABLE_AT + (opcode - ScriptTables::FIRST_OPCODE) * RECORD);
    }
    // xNVSE's commands, in opcode order.
    let built = crate::commands::all();
    for (i, (name, short)) in NAMES.iter().enumerate() {
        let opcode = FIRST_OPCODE + i as u32;
        let at = TABLE_AT + (opcode - ScriptTables::FIRST_OPCODE) * RECORD;
        let record = if let Some(cmd) = built.iter().find(|c| c.opcode == opcode) {
            assert_eq!(
                (cmd.name, cmd.short),
                (*name, *short),
                "{opcode:04x}: a command's names differ from xNVSE's"
            );
            e.register(execute_address(opcode), cmd.execute);
            let eval = cmd.eval.map_or(0, |f| {
                e.register(eval_address(opcode), f);
                eval_address(opcode)
            });
            returns[(opcode - ScriptTables::FIRST_OPCODE) as usize] = cmd.returns;
            Record {
                long_name: guest_str(e, cmd.name.as_bytes()),
                short_name: guest_str(e, cmd.short.as_bytes()),
                opcode,
                help: guest_str(e, cmd.help.as_bytes()),
                needs_ref: u16::from(cmd.needs_ref),
                param_count: cmd.params.len() as u16,
                params: guest_params(e, cmd.params),
                execute: execute_address(opcode),
                parse: DEFAULT_PARSE,
                eval,
                // `DEFINE_CMD_ALT_COND_ANY` (xNVSE) marks a condition-capable command with 1.
                flags: u32::from(cmd.eval.is_some()),
            }
        } else if let Some(console) = name
            .strip_prefix("con_")
            .and_then(|n| console_record(e, n.as_bytes()))
        {
            // `ImportConsoleCommand` (xNVSE): the console record with the new long name, no
            // short name and the script opcode.
            Record {
                long_name: guest_str(e, name.as_bytes()),
                short_name: empty,
                opcode,
                ..Record::read(e, console)
            }
        } else {
            Record {
                long_name: guest_str(e, name.as_bytes()),
                short_name: guest_str(e, short.as_bytes()),
                opcode,
                help: empty,
                parse: DEFAULT_PARSE,
                ..Record::default()
            }
        };
        record.write(e, at);
    }
    // The record past the end holds the end opcode the constructor checks.
    Record {
        opcode: max_opcode,
        ..Record::default()
    }
    .write(e, TABLE_AT + count * RECORD);
    e.ext::<Nvse>().table.returns = returns;
    e.set_script_tables(ScriptTables {
        start: TABLE_AT,
        end: TABLE_AT + count * RECORD,
        max_opcode,
    });
}

/// The return type of `opcode` (xNVSE's `GetReturnType`): `Default` outside the table.
pub fn return_type(e: &Engine, opcode: u32) -> Returns {
    e.ext_get::<Nvse>()
        .and_then(|n| {
            n.table
                .returns
                .get(opcode.wrapping_sub(ScriptTables::FIRST_OPCODE) as usize)
                .copied()
        })
        .unwrap_or_default()
}

/// The record of `opcode` in the current table, or 0 past its ends (xNVSE's `GetByOpcode`).
pub fn record_of(e: &Engine, opcode: u32) -> u32 {
    let t = e.script_tables();
    if (ScriptTables::FIRST_OPCODE..t.max_opcode).contains(&opcode) {
        t.record(opcode)
    } else {
        0
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// An engine with a stand-in of the exe's script and console tables: record `i` named
    /// `V<i>` with its opcode and an execute word, console records named `C<i>`, `SetVel` at
    /// console index 3.
    pub(crate) fn engine_with_exe_tables() -> Engine {
        let mut e = Engine::new();
        let exe = ScriptTables::EXE;
        e.map(0x0118_e000, 0x9000);
        e.map(0x0100_0000, 0x1_0000);
        let mut name_at = 0x0100_0000;
        let mut name = |e: &mut Engine, s: &str| {
            let at = name_at;
            for (i, b) in s.bytes().enumerate() {
                e.mem.set_u8(at + i as u32, b);
            }
            e.mem.set_u8(at + s.len() as u32, 0);
            name_at += s.len() as u32 + 1;
            at
        };
        for i in 0..0x280 {
            let at = exe.start + i * RECORD;
            let n = name(&mut e, &format!("V{i}"));
            e.mem.set_u32(at, n);
            e.mem.set_u32(at + 8, 0x1000 + i);
            e.mem.set_u32(at + 0x18, 0x0050_0000 + i);
        }
        // The record past the exe's table, as the constructor reads it.
        e.mem.set_u32(exe.end + 8, 0x1280);
        for i in 0..CONSOLE_COUNT {
            let at = CONSOLE_TABLE + i * RECORD;
            let text = if i == 3 {
                "SetVel".to_string()
            } else {
                format!("C{i}")
            };
            let n = name(&mut e, &text);
            e.mem.set_u32(at, n);
            e.mem.set_u32(at + 4, n);
            e.mem.set_u32(at + 8, 0x100 + i);
            e.mem.set_u16(at + 0x10, 1);
            e.mem.set_u32(at + 0x18, 0x0060_0000 + i);
        }
        e
    }

    /// The exe's records are copied unchanged; the table and its descriptor move; padding fills
    /// `0x1280..0x1400` as xNVSE's padding command.
    #[test]
    fn the_table_holds_the_exes_records_then_padding_then_xnvses() {
        let mut e = engine_with_exe_tables();
        assert_eq!(e.script_tables(), ScriptTables::EXE);
        install(&mut e);
        let t = e.script_tables();
        assert_eq!(t.start, TABLE_AT);
        assert_eq!(t.max_opcode, 0x1400 + 654);
        assert_eq!(t.end, TABLE_AT + (0x400 + 654) * RECORD);
        for i in [0u32, 0x59, 0x27f] {
            let copy = Record::read(&e, t.record(0x1000 + i));
            let exe = Record::read(&e, ScriptTables::EXE.record(0x1000 + i));
            assert_eq!(copy, exe, "{i:x}");
        }
        for opcode in [0x1280, 0x13ff] {
            let r = Record::read(&e, t.record(opcode));
            assert_eq!(
                (r.opcode, r.execute, r.parse),
                (opcode, DEFAULT_EXECUTE, DEFAULT_PARSE)
            );
            assert_eq!(guest_text(&e, r.long_name), b"");
            assert_eq!(guest_text(&e, r.help), b"command used for padding");
            assert_eq!(e.call(r.execute, &[0; 8]).eax, 1);
        }
        // The record past the end answers the constructor's check.
        assert_eq!(e.mem.u32(t.end + 8), t.max_opcode);
        assert_eq!(record_of(&e, t.max_opcode), 0);
        assert_eq!(record_of(&e, 0x1400), t.record(0x1400));
    }

    /// Every xNVSE opcode has its names; a console re-export copies the console record with the
    /// script opcode, the `con_` name and no short name.
    #[test]
    fn xnvse_records_have_their_names_and_the_console_re_exports() {
        let mut e = engine_with_exe_tables();
        install(&mut e);
        let t = e.script_tables();
        for (i, (name, short)) in NAMES.iter().enumerate() {
            let r = Record::read(&e, t.record(0x1400 + i as u32));
            assert_eq!(r.opcode, 0x1400 + i as u32);
            assert_eq!(guest_text(&e, r.long_name), name.as_bytes());
            if !name.starts_with("con_") {
                assert_eq!(guest_text(&e, r.short_name), short.as_bytes());
            }
        }
        let set_vel = NAMES.iter().position(|(n, _)| *n == "con_SetVel").unwrap() as u32;
        let r = Record::read(&e, t.record(0x1400 + set_vel));
        let console = Record::read(&e, CONSOLE_TABLE + 3 * RECORD);
        assert_eq!(
            r,
            Record {
                long_name: r.long_name,
                short_name: guest_str(&mut e, b""),
                opcode: 0x1400 + set_vel,
                ..console
            }
        );
        assert_eq!(r.execute, 0x0060_0003);
    }

    /// Return types: the 13 vanilla form commands, and numbers elsewhere.
    #[test]
    fn vanilla_form_commands_return_forms() {
        let mut e = engine_with_exe_tables();
        install(&mut e);
        assert_eq!(return_type(&e, 0x1025), Returns::Form);
        assert_eq!(return_type(&e, 0x1026), Returns::Default);
        assert_eq!(return_type(&e, 0x9000), Returns::Default);
    }

    /// Strings are written once.
    #[test]
    fn guest_strings_are_shared() {
        let mut e = engine_with_exe_tables();
        let a = guest_str(&mut e, b"abc");
        assert_eq!(guest_str(&mut e, b"abc"), a);
        assert_eq!(guest_text(&e, a), b"abc");
        assert_ne!(guest_str(&mut e, b"abd"), a);
    }
}
