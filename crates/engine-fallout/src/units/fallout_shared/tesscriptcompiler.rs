//! `fallout shared/tesscriptcompiler.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The console command table: 0x28-byte command definitions for opcodes
/// 0x100 to 0x1cc.
const CONSOLE_COMMANDS: u32 = 0x0118_e8e0;
/// The script command table: definitions for opcodes 0x1000 to 0x127f.
const SCRIPT_COMMANDS: u32 = 0x0119_0910;

// Translated from 005b1120 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `ScriptCompiler::GetFunctionDef` (Xbox PDB), `cdecl`: the address of the
/// definition of a command opcode in the console table (0x100..=0x1cc) or
/// the script table (0x1000..=0x127f), signed comparisons; null otherwise.
pub fn script_compiler_get_function_def(_e: &mut Engine, opcode: i32) -> Ptr {
    if (0x100..0x1cd).contains(&opcode) {
        Ptr::new(CONSOLE_COMMANDS + (opcode - 0x100) as u32 * 0x28)
    } else if (0x1000..0x1280).contains(&opcode) {
        Ptr::new(SCRIPT_COMMANDS + (opcode - 0x1000) as u32 * 0x28)
    } else {
        Ptr::NULL
    }
}

/// The opcode word (+8) of the entry just past each table (0x1CD console
/// entries, 0x280 script entries): the tables' end markers.
const CONSOLE_END_OPCODE: u32 = 0x0119_08f0;
const SCRIPT_END_OPCODE: u32 = 0x0119_6d18;
/// `Error` (`0040fbe0`, engine map; `cdecl`, a format and its words) and the
/// two messages: "The script function list doesn't have all the names
/// filled in." (`010374a0`) and "The script console-command list doesn't
/// have all the names filled in." (`01037458`).
const ERROR: u32 = 0x0040_fbe0;
const SCRIPT_LIST_INCOMPLETE: u32 = 0x0103_74a0;
const CONSOLE_LIST_INCOMPLETE: u32 = 0x0103_7458;

// Translated from 005aea50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptCompiler::ScriptCompiler` (Xbox PDB; `thiscall`, no stack words):
/// checks that the script command table ends with opcode `0x1280` and the
/// console table with `0x1CD` (the entries past their ends), reporting each
/// mismatch through `Error`. Returns `this`.
pub fn script_compiler_script_compiler(e: &mut Engine, this: Ptr) -> Ptr {
    if e.mem.u32(SCRIPT_END_OPCODE) != 0x1280 {
        e.call(ERROR, &args![SCRIPT_LIST_INCOMPLETE]);
    }
    if e.mem.u32(CONSOLE_END_OPCODE) != 0x1cd {
        e.call(ERROR, &args![CONSOLE_LIST_INCOMPLETE]);
    }
    this
}

// ---- n045: the compile data, the line reader and the word reader ----------------------------
//
// Offsets used below (Xbox PDB names, the same on PC):
// - `ScriptCompileData` (0x58): +0x00 `input`, +0x04 `uiInputOffset`, +0x08 `eCompilerIndex`,
//   +0x0C `cScriptName` (`BSStringT`), +0x14 `eLastError`, +0x18 `bIsPartialScript`, +0x1C
//   `uiLastLineNumber`, +0x20 `pOutput`, +0x24 `uiOutputOffset`, +0x28 `header`
//   (`SCRIPT_HEADER`: +0x2C `refObjectCount`, +0x30 `dataSize`, +0x34 `m_uiLastID`, +0x38
//   `bIsQuestScript`, +0x39 `bIsMagicEffectScript`), +0x3C `listVariables`, +0x44
//   `listRefObjects`, +0x4C `pCurrentScript`, +0x50 `listLines`.
// - `SCRIPT_LINE` (0x41C): +0x000 `uiLineNumber`, +0x004 `pLine` (0x200 chars), +0x204
//   `uiSize`, +0x208 `uiOffset`, +0x20C `pOutput` (0x200 bytes), +0x40C `uiOutputSize`, +0x410
//   `sExpression` (the command's opcode), +0x414 `uiRefObjectIndex`, +0x418 `eScriptError`.
// - The word the compiler reads (0x214 bytes, built by `005af310`; no PDB name found): the text
//   (0x200 chars), +0x200 the referenced object's index, +0x204 the kind character (`'X'` a
//   command, `'G'` a global, `'s'`/`'f'` a reference or number variable, `'r'` ...), +0x208
//   the command's opcode, +0x20C the variable id, +0x210 the referenced form.
// - `SCRIPT_REFERENCED_OBJECT` (0x10): +0x00 `cEditorID`, +0x08 `pForm`, +0x0C `uiVariableID`.
// - `SCRIPT_FUNCTION` (0x28): +0x00 `pFunctionName`, +0x04 `pShortName`, +0x08 the opcode
//   (`eOutput` in the PDB), +0x10 `bReferenceFunction`, +0x12 `sParamCount`, +0x14
//   `pParameters`, +0x1C `pCompileFunction`.

/// `operator new` (`00401000`, `cdecl`) and `operator delete` (`00401030`, `cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The game's `memset` (`00403d30`) and `memcpy` (`00401460`) wrappers, `cdecl`.
const MEMSET: u32 = 0x0040_3d30;
const MEMCPY: u32 = 0x0040_1460;
/// `BSStringT<char>`: constructor `004037b0`, destructor `004037d0`, `Set(text, length)`
/// `004037f0` (`RET 8`), its text `00559450` and `00438390` (`RET 4`: sets it from a C
/// string).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
const STRING_SET: u32 = 0x0040_37f0;
const STRING_TEXT: u32 = 0x0055_9450;
const STRING_ASSIGN: u32 = 0x0043_8390;
/// `BSStringT<char>::StrCmp` (engine map, `00408a80`, `RET 8`: text, ignore case).
const STRING_COMPARE: u32 = 0x0040_8a80;
/// The list helpers (module notes of `tesscript.rs`): `006815c0` the item word's address,
/// `00726070` the next node, `008256d0` empty, `0063f7b0` remove the first item, `00905820`
/// add at the tail, `0096a2d0` construct, `0046ffb0` destruct.
const LIST_ITEM: u32 = 0x0068_15c0;
const LIST_NEXT: u32 = 0x0072_6070;
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_POP_FIRST: u32 = 0x0063_f7b0;
const LIST_ADD_TAIL: u32 = 0x0090_5820;
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// The addresses of a compile data's (or script's) lists: `005a8080` +0x3C (variables),
/// `005d43c0` +0x44 (referenced objects), `00613790` +0x50 (lines).
const VARIABLE_LIST_OF: u32 = 0x005a_8080;
const REF_OBJECT_LIST_OF: u32 = 0x005d_43c0;
const LINE_LIST_OF: u32 = 0x0061_3790;
/// The deleting destructors (`RET 4`, flags 1) of a referenced object (`005aa3f0`) and a
/// script variable (`0061cb30`).
const REF_OBJECT_DELETE: u32 = 0x005a_a3f0;
const VARIABLE_DELETE: u32 = 0x0061_cb30;
/// `005aaee0(count, size)` (`cdecl`): a `calloc` on the memory manager.
const CALLOCATE: u32 = 0x005a_aee0;
/// The C runtime and string helpers, `cdecl`: `strlen` (`00ec6130`), the game's length
/// (`0044a670`), `_strnicmp` (`00ec7ec0`), `isalnum` (`00ec89e9`), `isalpha` (`00ec86e1`),
/// `strchr` (`00ec7690`), `strtoul` (`00ec8d33`), `strcpy_s` (`00406d30`: destination, size,
/// source), `_stricmp` through `00404dc0`, `vsprintf` into a buffer (`00407000`: buffer,
/// format, arguments) and the string printf `00406f60` (string, format, ...).
const STRLEN: u32 = 0x00ec_6130;
const TEXT_LENGTH: u32 = 0x0044_a670;
const STRNICMP: u32 = 0x00ec_7ec0;
const ISALNUM: u32 = 0x00ec_89e9;
const ISALPHA: u32 = 0x00ec_86e1;
const STRCHR: u32 = 0x00ec_7690;
const STRTOUL: u32 = 0x00ec_8d33;
const STRCPY_S: u32 = 0x0040_6d30;
const STRICMP: u32 = 0x0040_4dc0;
const VSPRINTF: u32 = 0x0040_7000;
const STRING_PRINTF: u32 = 0x0040_6f60;
/// `00464f30(text, 0)` (`cdecl`), the console print `00703c00(text)` and the log
/// `005b5e40(format, ...)`, all `cdecl`.
const PREPARE_MESSAGE: u32 = 0x0046_4f30;
const CONSOLE_PRINT: u32 = 0x0070_3c00;
const SCRIPT_LOG: u32 = 0x005b_5e40;
/// `__RTDynamicCast` (`00ec43fb`) and the RTTI type descriptors of `TESForm` (`01183028`),
/// `TESObjectREFR` (`011841cc`) and `TESQuest` (`01186500`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const TYPE_TES_QUEST: u32 = 0x0118_6500;
/// `TESObjectREFR::InitScript` (`00565730`), `GetScriptPointer` (`005673c0`) and
/// `GetRefPersists` (`005653d0`), engine map names, `thiscall`.
const REFR_INIT_SCRIPT: u32 = 0x0056_5730;
const REFR_SCRIPT: u32 = 0x0056_73c0;
const REFR_PERSISTS: u32 = 0x0056_53d0;
/// `00401170` (`thiscall`): a form's type byte.
const FORM_TYPE: u32 = 0x0040_1170;
/// `TESScriptableForm::GetFormScript` (engine map, `004826d0`, `cdecl`: the form).
const FORM_SCRIPT: u32 = 0x0048_26d0;
/// `Script::FindVariable` (engine map, `005ac6a0`, `RET 8`: name, id output).
const SCRIPT_FIND_VARIABLE: u32 = 0x005a_c6a0;
/// `TESDataHandler::GetGlobal` (engine map, `00461700`, `RET 4`) on the object at
/// `011c3f2c`.
const GET_GLOBAL: u32 = 0x0046_1700;
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `TESForm::GetFormByEditorID` (`00483a00`) and `LookupFormByID` (`004839c0`), `cdecl`.
const FORM_BY_EDITOR_ID: u32 = 0x0048_3a00;
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `00500910` (`thiscall` on a new 0x10-byte block): constructs a
/// `SCRIPT_REFERENCED_OBJECT`.
const REF_OBJECT_CONSTRUCT: u32 = 0x0050_0910;
/// `00500940` (`thiscall` on a script): its header (`SCRIPT_HEADER`).
const SCRIPT_HEADER_OF: u32 = 0x0050_0940;
/// `007f7c90` (`thiscall` on a script, `RET 4`: a flag), called with 0 before a compile and
/// with 1 after a successful one.
const SCRIPT_COMPILED_FLAG: u32 = 0x007f_7c90;
/// `Script::Init` (engine map, `005aaf20`, `RET 4`: the compile data).
const SCRIPT_INIT: u32 = 0x005a_af20;
/// `0055b980` (`thiscall` on a script): its text.
const SCRIPT_TEXT: u32 = 0x0055_b980;
/// `005ab7f0(list, list, 0)` (`cdecl`): copies a script's referenced objects into the compile
/// data's list.
const COPY_REF_OBJECTS: u32 = 0x005a_b7f0;
/// `00474cb0` (`thiscall`): the length of a form's name (vtable slot 0x130), 0 without one.
const NAME_LENGTH: u32 = 0x0047_4cb0;
/// The player reference pointer (`011dea3c`).
const PLAYER_REF: u32 = 0x011d_ea3c;
/// The compiler names `CompilePartialScript` names the script after, by compiler index
/// (`0118ca7c`, the first "DefaultCompiler").
const COMPILER_NAMES: u32 = 0x0118_ca7c;
/// The block type table (`0118cb50`, 16 entries), searched with the name first.
const BLOCK_TABLE: u32 = 0x0118_cb50;

/// The messages the compiler reports (`ScriptError` formats).
const TEXT_SCRIPT_LINE: u32 = 0x0103_74e4;
const TEXT_START_WITH_SCRIPTNAME: u32 = 0x0103_7508;
const TEXT_SYNTAX_ERROR_LINE: u32 = 0x0103_75f0;
const TEXT_CONSOLE_ONLY: u32 = 0x0103_75c0;
const TEXT_COMMAND_NOT_FOUND: u32 = 0x0103_75a0;
const TEXT_MISSING_SCRIPT_NAME: u32 = 0x0103_757c;
const TEXT_NO_CONDITION: u32 = 0x0103_7540;
const TEXT_LINE_TOO_LONG: u32 = 0x0103_7620;
const TEXT_MISMATCHED_QUOTES: u32 = 0x0103_7798;
const TEXT_REFERENCE_NOT_ALLOWED: u32 = 0x0103_7760;
const TEXT_INVALID_REFERENCE_SYNTAX: u32 = 0x0103_7734;
const TEXT_UNKNOWN_OBJECT: u32 = 0x0103_771c;
const TEXT_UNKNOWN_VARIABLE_OR_FUNCTION: u32 = 0x0103_76f8;
const TEXT_ONLY_QUESTS_AND_REFERENCES: u32 = 0x0103_7690;
const TEXT_NO_FUNCTIONS_ON_QUESTS: u32 = 0x0103_7654;
const TEXT_UNKNOWN_REFERENCED_OBJECT: u32 = 0x0103_77bc;
const TEXT_PLAYER: u32 = 0x0101_8f54;
const TEXT_NON_PERSISTENT: u32 = 0x0103_77dc;
const TEXT_NESTED_BLOCKS: u32 = 0x0103_7bfc;
const TEXT_END_WITHOUT_BEGIN: u32 = 0x0103_7be0;
const TEXT_OUTSIDE_BLOCK: u32 = 0x0103_7ba8;
const TEXT_BEGIN_WITHOUT_END: u32 = 0x0103_7c58;
const TEXT_MISMATCHED_BLOCK: u32 = 0x0103_7c28;
const TEXT_MISMATCHED_IF: u32 = 0x0103_7cd8;
const TEXT_MAX_NESTING: u32 = 0x0103_7cbc;
const TEXT_MISMATCHED_IF_BLOCK: u32 = 0x0103_7c88;
const TEXT_UNDEFINED_FUNCTION: u32 = 0x0103_7d68;
const TEXT_NEEDS_EXPLICIT_REFERENCE: u32 = 0x0103_7d20;
const TEXT_UNKNOWN_COMMAND: u32 = 0x0103_7cf4;

/// Reads a word of a structure the exe addresses as `base + offset`.
fn word_at(e: &Engine, base: u32, offset: u32) -> u32 {
    e.mem.u32(base.wrapping_add(offset))
}

/// The item a list node holds (`006815c0`, then the word there).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let at = e.call(LIST_ITEM, &args![node]).u32();
    e.mem.u32(at)
}

/// The deleting step of the three list emptiers: while the list (from `list_of`) is not
/// empty, its first item is handed to `delete` and removed (`0063f7b0`).
fn empty_list_with(e: &mut Engine, this: Ptr, list_of: u32, delete: impl Fn(&mut Engine, u32)) {
    let list = e.call(list_of, &args![this]).u32();
    while list != 0 && !e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let item = node_item(e, list);
        delete(e, item);
        e.call(LIST_POP_FIRST, &args![list]);
    }
}

// Translated from 005ae490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ae490 - the FS:[0] exception frame (unwinding the members built so far) is not translated.
/// `ScriptCompileData::ScriptCompileData` (Xbox PDB; `ECX`, plain `RET`): constructs the name
/// string and the three lists, clears the line number, input offset and output offset,
/// allocates the 0x4000-byte output (`005aaee0(1, 0x4000)`) and clears its first word (twice)
/// and the header (0x14 bytes), clears the compiler index, the partial flag and the current
/// script, sets the name to the empty string `01011584` and clears the last error. Returns
/// `this`. The exception frame is not translated.
pub fn script_compile_data_script_compile_data(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(STRING_CONSTRUCT, &args![base.wrapping_add(0xc)]);
    e.call(LIST_CONSTRUCT, &args![base.wrapping_add(0x3c)]);
    e.call(LIST_CONSTRUCT, &args![base.wrapping_add(0x44)]);
    e.call(LIST_CONSTRUCT, &args![base.wrapping_add(0x50)]);
    e.mem.set_u32(base.wrapping_add(0x1c), 0);
    e.mem.set_u32(base.wrapping_add(4), 0);
    e.mem.set_u32(base.wrapping_add(0x24), 0);
    let output = e.call(CALLOCATE, &args![1u32, 0x4000u32]).u32();
    e.mem.set_u32(base.wrapping_add(0x20), output);
    let output = word_at(e, base, 0x20);
    e.call(MEMSET, &args![output, 0u32, 4u32]);
    e.call(MEMSET, &args![base.wrapping_add(0x28), 0u32, 0x14u32]);
    let output = word_at(e, base, 0x20);
    e.call(MEMSET, &args![output, 0u32, 4u32]);
    e.mem.set_u32(base.wrapping_add(8), 0);
    e.mem.set_u8(base.wrapping_add(0x18), 0);
    e.mem.set_u32(base.wrapping_add(0x4c), 0);
    e.call(
        STRING_SET,
        &args![base.wrapping_add(0xc), 0x0101_1584u32, 0u32],
    );
    e.mem.set_u32(base.wrapping_add(0x14), 0);
    this
}

// Translated from 005ae5c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ae5c0 - the FS:[0] exception frame (unwinding the members) is not translated.
/// `ScriptCompileData::~ScriptCompileData` (Xbox PDB; `ECX`, plain `RET`): deletes the lines
/// (`005ae670`), the referenced objects (`005ae6c0`) and the variables (`005ae730`), the output
/// buffer, then destructs the three lists (+0x50, +0x44, +0x3C) and the name string. The
/// exception frame is not translated.
pub fn script_compile_data_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    fn_005ae670(e, this);
    fn_005ae6c0(e, this);
    fn_005ae730(e, this);
    let output = word_at(e, base, 0x20);
    e.call(OPERATOR_DELETE, &args![output]);
    e.call(LIST_DESTRUCT, &args![base.wrapping_add(0x50)]);
    e.call(LIST_DESTRUCT, &args![base.wrapping_add(0x44)]);
    e.call(LIST_DESTRUCT, &args![base.wrapping_add(0x3c)]);
    e.call(STRING_DESTRUCT, &args![base.wrapping_add(0xc)]);
}

// Translated from 005ae670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX` = compile data): deletes every line (`operator delete`) of the
/// line list (`00613790`), removing the first item until the list is empty.
pub fn fn_005ae670(e: &mut Engine, this: Ptr) {
    empty_list_with(e, this, LINE_LIST_OF, |e, line| {
        e.call(OPERATOR_DELETE, &args![line]);
    });
}

// Translated from 005ae6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX` = compile data): deletes every referenced object (`005aa3f0`
/// with 1, skipping null items) of the referenced object list (`005d43c0`), removing the
/// first item until the list is empty.
pub fn fn_005ae6c0(e: &mut Engine, this: Ptr) {
    empty_list_with(e, this, REF_OBJECT_LIST_OF, |e, object| {
        if object != 0 {
            e.call(REF_OBJECT_DELETE, &args![object, 1u32]);
        }
    });
}

// Translated from 005ae730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX` = compile data): deletes every variable (`0061cb30` with 1,
/// skipping null items) of the variable list (`005a8080`), removing the first item until the
/// list is empty.
pub fn fn_005ae730(e: &mut Engine, this: Ptr) {
    empty_list_with(e, this, VARIABLE_LIST_OF, |e, variable| {
        if variable != 0 {
            e.call(VARIABLE_DELETE, &args![variable, 1u32]);
        }
    });
}

/// Clears the word's variable id (+0x20C).
fn clear_variable_id(e: &mut Engine, word: u32) {
    e.mem.set_u32(word.wrapping_add(0x20c), 0);
}

// Translated from 005ae7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptCompileData::GetVariable` (Xbox PDB, `RET 4`: a word): resolves the word as a
/// variable. With a referenced form (+0x210): the script of the reference (`InitScript`,
/// `GetScriptPointer`) or of a quest (type 0x47, `GetFormScript`) answers through
/// `Script::FindVariable` (kind at +0x204, id at +0x20C); none clears the id. Without one: a
/// word with a referenced object index (+0x200) is no variable; otherwise the compile data's
/// variables are searched (`_strnicmp` over the word's length, the variable's name then ending
/// in a character that is not alphanumeric, `-` or `_`; kind `'s'` when the variable's byte
/// at +0x10 is set, `'f'` otherwise), then the current script's (`FindVariable`), then a global
/// (`TESDataHandler::GetGlobal`) that `GetReferencedObject` accepts (kind `'G'`, id 0).
/// Not found: id and kind cleared, false.
pub fn script_compile_data_get_variable(e: &mut Engine, this: Ptr, word: u32) -> bool {
    let length = e.call(TEXT_LENGTH, &args![word]).u32();
    let form = word_at(e, word, 0x210);
    if form != 0 {
        let mut script = 0u32;
        let reference = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0u32],
            )
            .u32();
        if reference != 0 {
            e.call(REFR_INIT_SCRIPT, &args![reference]);
            script = e.call(REFR_SCRIPT, &args![reference]).u32();
        } else if word_at(e, word, 0x210) != 0 {
            let form = word_at(e, word, 0x210);
            if e.call(FORM_TYPE, &args![form]).u32() == 0x47 {
                let form = word_at(e, word, 0x210);
                script = e.call(FORM_SCRIPT, &args![form]).u32();
            }
        }
        if script == 0 {
            clear_variable_id(e, word);
            return false;
        }
        let kind = e
            .call(
                SCRIPT_FIND_VARIABLE,
                &args![script, word, word.wrapping_add(0x20c)],
            )
            .u8();
        e.mem.set_u8(word.wrapping_add(0x204), kind);
        if e.mem.i8(word.wrapping_add(0x204)) == 0 {
            clear_variable_id(e, word);
            return false;
        }
        return true;
    }
    if word_at(e, word, 0x200) != 0 {
        clear_variable_id(e, word);
        e.mem.set_u8(word.wrapping_add(0x204), 0);
        return false;
    }
    let mut node = e.call(VARIABLE_LIST_OF, &args![this]).u32();
    clear_variable_id(e, word);
    while node != 0 {
        let at = e.call(LIST_ITEM, &args![node]).u32();
        if e.mem.u32(at) == 0 {
            break;
        }
        let variable = node_item(e, node);
        let name = e
            .call(STRING_TEXT, &args![variable.wrapping_add(0x18)])
            .u32();
        if e.call(STRNICMP, &args![name, word, length]).u32() == 0 {
            let name = e
                .call(STRING_TEXT, &args![variable.wrapping_add(0x18)])
                .u32();
            let next = e.mem.i8(name.wrapping_add(length));
            if e.call(ISALNUM, &args![next as i32]).u32() == 0 && next != 0x2d && next != 0x5f {
                let kind = if e.mem.u8(variable.wrapping_add(0x10)) != 0 {
                    0x73
                } else {
                    0x66
                };
                e.mem.set_u8(word.wrapping_add(0x204), kind);
                let id = e.mem.u32(variable);
                e.mem.set_u32(word.wrapping_add(0x20c), id);
                return true;
            }
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    let current = word_at(e, this.addr(), 0x4c);
    if current != 0 {
        let kind = e
            .call(
                SCRIPT_FIND_VARIABLE,
                &args![current, word, word.wrapping_add(0x20c)],
            )
            .u8();
        e.mem.set_u8(word.wrapping_add(0x204), kind);
        if e.mem.i8(word.wrapping_add(0x204)) != 0 {
            return true;
        }
    }
    let handler = e.global::<u32>(DATA_HANDLER);
    if e.call(GET_GLOBAL, &args![handler, word]).u32() != 0
        && script_compile_data_get_referenced_object(e, this, word, 0, 0)
    {
        clear_variable_id(e, word);
        e.mem.set_u8(word.wrapping_add(0x204), 0x47);
        return true;
    }
    clear_variable_id(e, word);
    e.mem.set_u8(word.wrapping_add(0x204), 0);
    false
}

// Translated from 005aea90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005aea90 - the FS:[0] exception frame and the /GS stack cookie check (00ec408c) are not translated.
/// `ScriptCompiler::ScriptError` (Xbox PDB), `cdecl` with variable arguments (the compile
/// data, a format, its words): formats the message into a 0x108-byte buffer (`00407000` with
/// the words as its argument list), then into a string "SCRIPTS: Script '%s', line %d:\n%s"
/// with the script's name and last line number (`00406f60`), and prints it (through
/// `00464f30`) on the console (`00703c00`) for compiler index 1, else to the log
/// (`005b5e40`). The exception frame and the stack cookie check are not translated.
pub fn script_compiler_script_error(e: &mut Engine, data: Ptr, format: u32, rest: &[u32]) {
    let base = data.addr();
    let words = rest.len() as u32 * 4;
    e.with_stack(0x110 + words, |e, frame| {
        let buffer = frame.addr();
        let string = buffer + 0x108;
        let list = buffer + 0x110;
        for (i, word) in rest.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, *word);
        }
        e.call(VSPRINTF, &args![buffer, format, list]);
        e.call(STRING_CONSTRUCT, &args![string]);
        let line = word_at(e, base, 0x1c);
        let name = e.call(STRING_TEXT, &args![base.wrapping_add(0xc)]).u32();
        e.call(
            STRING_PRINTF,
            &args![string, TEXT_SCRIPT_LINE, name, line, buffer],
        );
        if word_at(e, base, 8) == 1 {
            let text = e.call(STRING_TEXT, &args![string]).u32();
            let message = e.call(PREPARE_MESSAGE, &args![text, 0u32]).u32();
            e.call(CONSOLE_PRINT, &args![message]);
        } else {
            let text = e.call(STRING_TEXT, &args![string]).u32();
            let message = e.call(PREPARE_MESSAGE, &args![text, 0u32]).u32();
            e.call(SCRIPT_LOG, &args![message]);
        }
        e.call(STRING_DESTRUCT, &args![string]);
    });
}

/// The uniform entry of `ScriptCompiler::ScriptError` (variable words).
fn script_compiler_script_error_entry(e: &mut Engine, words: &[u32]) -> Ret {
    script_compiler_script_error(e, Ptr::new(words[0]), words[1], &words[2..]);
    Ret::default()
}

/// `ScriptError` with no words after the format.
fn report(e: &mut Engine, data: Ptr, format: u32) {
    script_compiler_script_error(e, data, format, &[]);
}

// Translated from 005aeb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptCompiler::Compile` (Xbox PDB, `RET 8`: the script and the compile data): false for
/// a null script or compile data. Clears the script's compiled flag (`007f7c90` with 0) and
/// takes the quest, magic effect and last id fields of its header (`00500940`). With input,
/// reads every expression (`GetNextExpression` until 0xFFFF); an error count above 0 fails;
/// the line number is reset and, unless the script is partial, the first line must be a
/// `scriptname` (opcode 0x1D; an empty list or another opcode is reported, 0xFFFF fails
/// quietly) and the blocks must verify (`VerifyBlocks`). Each line is then compiled
/// (`CompileLine`) with its number as the last line number; the data size takes the output
/// offset, and when the referenced objects verify the script is initialised (`Script::Init`)
/// and marked compiled (`007f7c90` with 1): true.
pub fn script_compiler_compile(e: &mut Engine, this: Ptr, script: u32, data: Ptr) -> bool {
    let base = data.addr();
    if script == 0 || base == 0 {
        return false;
    }
    e.call(SCRIPT_COMPILED_FLAG, &args![script, 0u32]);
    let header = e.call(SCRIPT_HEADER_OF, &args![script]).u32();
    let quest = e.mem.u8(header.wrapping_add(0x10));
    e.mem.set_u8(base.wrapping_add(0x38), quest);
    let header = e.call(SCRIPT_HEADER_OF, &args![script]).u32();
    let magic = e.mem.u8(header.wrapping_add(0x11));
    e.mem.set_u8(base.wrapping_add(0x39), magic);
    let header = e.call(SCRIPT_HEADER_OF, &args![script]).u32();
    let last_id = word_at(e, header, 0xc);
    e.mem.set_u32(base.wrapping_add(0x34), last_id);
    if word_at(e, base, 0) != 0 {
        while script_compiler_get_next_expression(e, this, data) != 0xffff {}
        if e.mem.i32(base.wrapping_add(0x14)) > 0 {
            return false;
        }
        e.mem.set_u32(base.wrapping_add(0x1c), 0);
        if e.mem.u8(base.wrapping_add(0x18)) == 0 {
            let lines = e.call(LINE_LIST_OF, &args![data]).u32();
            if lines == 0 || e.call(LIST_IS_EMPTY, &args![lines]).bool() {
                report(e, data, TEXT_START_WITH_SCRIPTNAME);
                return false;
            }
            let line = node_item(e, lines);
            let opcode = word_at(e, line, 0x410);
            if opcode == 0xffff {
                return false;
            }
            if opcode != 0x1d {
                report(e, data, TEXT_START_WITH_SCRIPTNAME);
                return false;
            }
            if !script_compiler_verify_blocks(e, this, data) {
                return false;
            }
        }
    }
    let mut node = e.call(LINE_LIST_OF, &args![data]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let line = node_item(e, node);
        let number = e.mem.u32(line);
        e.mem.set_u32(base.wrapping_add(0x1c), number);
        if !script_compiler_compile_line(e, this, script, data, line) {
            return false;
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    let size = word_at(e, base, 0x24);
    e.mem.set_u32(base.wrapping_add(0x30), size);
    if !script_compile_data_verify_referenced_object_list(e, data) {
        return false;
    }
    e.call(SCRIPT_INIT, &args![script, data]);
    e.call(SCRIPT_COMPILED_FLAG, &args![script, 1u32]);
    true
}

// Translated from 005aedb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005aedb0 - the FS:[0] exception frame (destroying the compile data on unwinding) is not translated.
/// `ScriptCompiler::CompilePartialScript` (Xbox PDB, `RET 0x10`: script, reference, compiler
/// index, a word it does not read): compiles the script's text as a partial script in a
/// compile data on the stack (`005ae490`, compiler index set, partial flag 1). With a
/// reference that has a script (`GetScriptPointer`), that script's header (0x14 bytes) and
/// referenced objects (`005ab7f0`) are copied in and it names the result; otherwise the script
/// does. Unless the compiler index is 1, a named form (`00474cb0`) gives the name (vtable slot
/// 0x130); else the compiler's name from the table at `0118ca7c`. The input is the script's
/// text (`0055b980`); `Compile` answers; the compile data is destructed. The exception frame is
/// not translated.
pub fn script_compiler_compile_partial_script(
    e: &mut Engine,
    this: Ptr,
    script: u32,
    reference: u32,
    compiler: u32,
    _unused_4: u32,
) -> bool {
    e.with_stack(0x58, |e, data| {
        let base = data.addr();
        script_compile_data_script_compile_data(e, data);
        e.mem.set_u32(base + 8, compiler);
        e.mem.set_u8(base + 0x18, 1);
        let named = if reference != 0 && e.call(REFR_SCRIPT, &args![reference]).u32() != 0 {
            let source = e.call(REFR_SCRIPT, &args![reference]).u32();
            let header = e.call(SCRIPT_HEADER_OF, &args![source]).u32();
            e.call(MEMCPY, &args![base + 0x28, header, 0x14u32]);
            let list = e.call(REF_OBJECT_LIST_OF, &args![source]).u32();
            e.call(COPY_REF_OBJECTS, &args![list, base + 0x44, 0u32]);
            source
        } else {
            script
        };
        if compiler != 1 && e.call(NAME_LENGTH, &args![named]).u32() != 0 {
            let name = e.vcall(named, 0x130, &[]).u32();
            e.call(STRING_SET, &args![base + 0xc, name, 0u32]);
        } else {
            let index = e.mem.u32(base + 8);
            let compiler_name = e
                .mem
                .u32(COMPILER_NAMES.wrapping_add(index.wrapping_mul(4)));
            e.call(STRING_ASSIGN, &args![base + 0xc, compiler_name]);
        }
        let text = e.call(SCRIPT_TEXT, &args![script]).u32();
        e.mem.set_u32(base, text);
        let compiled = script_compiler_compile(e, this, script, data);
        script_compile_data_destructor(e, data);
        compiled
    })
}

/// Deletes a line (`operator delete`) and answers 0xFFFF, `GetNextExpression`'s "no line".
fn drop_line(e: &mut Engine, line: u32) -> u32 {
    e.call(OPERATOR_DELETE, &args![line]);
    0xffff
}

// Translated from 005aeed0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005aeed0 - the FS:[0] exception frame and the /GS stack cookie check (00ec408c) are not translated.
/// `ScriptCompiler::GetNextExpression` (Xbox PDB, `RET 4`: the compile data): reads the next
/// line into a new `SCRIPT_LINE` (numbered after the last line) and answers its command's
/// opcode, or 0xFFFF when the input is used up or the line is dropped. The line's first word
/// (`GetNextWord`, references allowed) must be a command (kind `'X'`); console commands
/// (0x100..0x1CC) only for compiler index 1; unknown commands are reported (error 0xD, line
/// number counted). The command word is cut from the line, the last line number updated; a
/// `scriptname` (0x1D) sets the script's name from the next word (error 1 when missing); an
/// `if`/`elseif` (0x16, 0x18) needs a condition (error 6). The line is appended to the line
/// list. Dropped lines are deleted. The exception frame and the stack cookie check are not
/// translated.
pub fn script_compiler_get_next_expression(e: &mut Engine, _this: Ptr, data: Ptr) -> u32 {
    let base = data.addr();
    let input = word_at(e, base, 0);
    let length = e.call(STRLEN, &args![input]).u32();
    if word_at(e, base, 4) >= length {
        return 0xffff;
    }
    let block = e.call(OPERATOR_NEW, &args![0x41cu32]).u32();
    let line = if block != 0 {
        script_line_script_line(e, Ptr::new(block)).addr()
    } else {
        0
    };
    let number = word_at(e, base, 0x1c).wrapping_add(1);
    e.mem.set_u32(line, number);
    let read = script_compiler_get_next_line(e, data, Ptr::new(line));
    if read == 0 {
        let error = word_at(e, line, 0x418);
        e.mem.set_u32(base.wrapping_add(0x14), error);
        return drop_line(e, line);
    }
    let offset = word_at(e, base, 4).wrapping_add(read);
    e.mem.set_u32(base.wrapping_add(4), offset);
    e.with_stack(0x430, |e, words| {
        let word = words.addr();
        fn_005af310(e, Ptr::new(word));
        let found = script_compiler_get_next_word(
            e,
            data,
            word,
            line.wrapping_add(4),
            line.wrapping_add(0x208),
            1,
            1,
        );
        if found == 0 {
            script_compiler_script_error(e, data, TEXT_SYNTAX_ERROR_LINE, &[line.wrapping_add(4)]);
            return drop_line(e, line);
        }
        let reference = word_at(e, word, 0x200);
        e.mem.set_u32(line.wrapping_add(0x414), reference);
        let mut command = e.mem.i8(word.wrapping_add(0x204)) == 0x58;
        if command {
            let opcode = e.mem.i32(word.wrapping_add(0x208));
            if (0x100..0x1cd).contains(&opcode) && word_at(e, base, 8) != 1 {
                command = false;
                script_compiler_script_error(e, data, TEXT_CONSOLE_ONLY, &[word]);
            }
        } else {
            let number = word_at(e, base, 0x1c).wrapping_add(1);
            e.mem.set_u32(base.wrapping_add(0x1c), number);
            script_compiler_script_error(e, data, TEXT_COMMAND_NOT_FOUND, &[word]);
            e.mem.set_u32(base.wrapping_add(0x14), 0xd);
        }
        if !command {
            return drop_line(e, line);
        }
        let opcode = word_at(e, word, 0x208);
        e.mem.set_u32(line.wrapping_add(0x410), opcode);
        let rest = line.wrapping_add(word_at(e, line, 0x208)).wrapping_add(4);
        e.call(STRCPY_S, &args![line.wrapping_add(4), 0x200u32, rest]);
        let size = word_at(e, line, 0x204).wrapping_sub(word_at(e, line, 0x208));
        e.mem.set_u32(line.wrapping_add(0x204), size);
        let number = e.mem.u32(line);
        e.mem.set_u32(base.wrapping_add(0x1c), number);
        if word_at(e, line, 0x410) == 0x1d {
            // The second word and its offset (0x214 + 4 bytes) after the first word.
            let name = word.wrapping_add(0x218);
            let offset = word.wrapping_add(0x214);
            e.mem.set_u32(offset, 0);
            fn_005af310(e, Ptr::new(name));
            if script_compiler_get_next_word(e, data, name, line.wrapping_add(4), offset, 0, 0) != 0
            {
                e.call(STRING_SET, &args![base.wrapping_add(0xc), name, 0u32]);
            } else {
                report(e, data, TEXT_MISSING_SCRIPT_NAME);
                e.mem.set_u32(base.wrapping_add(0x14), 1);
                return drop_line(e, line);
            }
        }
        let opcode = word_at(e, line, 0x410);
        if (opcode == 0x16 || opcode == 0x18) && e.mem.i8(line.wrapping_add(4)) == 0 {
            report(e, data, TEXT_NO_CONDITION);
            e.mem.set_u32(base.wrapping_add(0x14), 6);
            return drop_line(e, line);
        }
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), line);
            let lines = e.call(LINE_LIST_OF, &args![data]).u32();
            e.call(LIST_ADD_TAIL, &args![lines, slot]);
        });
        word_at(e, line, 0x410)
    })
}

// Translated from 005af310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX`; the word constructor): clears the word's fields (+0x200 index,
/// +0x20C id, +0x204 kind, +0x208 opcode, +0x210 form) and its text (0x200 bytes,
/// `memset`). Returns `this`.
pub fn fn_005af310(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base.wrapping_add(0x200), 0);
    e.mem.set_u32(base.wrapping_add(0x20c), 0);
    e.mem.set_u8(base.wrapping_add(0x204), 0);
    e.mem.set_u32(base.wrapping_add(0x208), 0);
    e.mem.set_u32(base.wrapping_add(0x210), 0);
    e.call(MEMSET, &args![base, 0u32, 0x200u32]);
    this
}

// Translated from 005af370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SCRIPT_LINE::SCRIPT_LINE` (Xbox PDB; `ECX`): clears the line number, size, offset, output
/// size and error, and the text and output buffers (0x200 bytes each). Returns `this`.
pub fn script_line_script_line(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base.wrapping_add(0x204), 0);
    e.mem.set_u32(base.wrapping_add(0x208), 0);
    e.mem.set_u32(base.wrapping_add(0x40c), 0);
    e.mem.set_u32(base.wrapping_add(0x418), 0);
    e.call(MEMSET, &args![base.wrapping_add(4), 0u32, 0x200u32]);
    e.call(MEMSET, &args![base.wrapping_add(0x20c), 0u32, 0x200u32]);
    this
}

// Translated from 005af3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptCompiler::GetNextLine` (Xbox PDB), `cdecl` (compile data, line): copies the next
/// line of the input (from the input offset) into the line's text and answers the number of
/// input bytes read. Characters before the first alphanumeric or `"` are skipped; a `;`
/// comment runs to the line feed; a carriage return (or the end of a comment) takes a
/// following line feed with it. At the end of a line the trailing blanks and tabs are cut;
/// an empty line counts a line number and starts again (also past the input's end, as the
/// exe does). The end of the input before any text answers 0; a line of 0x1FF characters or
/// more is reported (error 0x10) and answers 0.
pub fn script_compiler_get_next_line(e: &mut Engine, data: Ptr, line: Ptr) -> u32 {
    let base = data.addr();
    let line = line.addr();
    let start = word_at(e, base, 0).wrapping_add(word_at(e, base, 4));
    let mut read = 0u32;
    let mut skipping = true;
    loop {
        let mut c = e.mem.u8(start.wrapping_add(read));
        read = read.wrapping_add(1);
        let at_end = if c == 0 {
            true
        } else if c == 0xd || c == 0x3b {
            if c == 0x3b {
                while c != 0 && c != 0xa {
                    c = e.mem.u8(start.wrapping_add(read));
                    read = read.wrapping_add(1);
                }
            }
            if c != 0 && e.mem.i8(start.wrapping_add(read)) == 0xa {
                read = read.wrapping_add(1);
            }
            true
        } else {
            false
        };
        if at_end {
            if !skipping {
                loop {
                    let size = word_at(e, line, 0x204);
                    if size == 0 {
                        break;
                    }
                    let last = e.mem.i8(line.wrapping_add(size).wrapping_add(3));
                    if last != 0x20 && last != 0x9 {
                        break;
                    }
                    let size = size.wrapping_sub(1);
                    e.mem.set_u32(line.wrapping_add(0x204), size);
                    e.mem.set_u8(line.wrapping_add(size).wrapping_add(4), 0);
                }
                let size = word_at(e, line, 0x204);
                if size != 0 {
                    e.mem.set_u8(line.wrapping_add(size).wrapping_add(4), 0);
                    return read;
                }
            } else if c == 0 {
                return 0;
            }
            let number = e.mem.u32(line).wrapping_add(1);
            e.mem.set_u32(line, number);
            e.mem.set_u32(line.wrapping_add(0x204), 0);
            e.call(MEMSET, &args![line.wrapping_add(4), 0u32, 0x200u32]);
            continue;
        }
        if word_at(e, line, 0x204).wrapping_add(1) >= 0x200 {
            script_compiler_script_error(e, data, TEXT_LINE_TOO_LONG, &[0x200]);
            e.mem.set_u32(line.wrapping_add(0x418), 0x10);
            return 0;
        }
        if e.call(ISALNUM, &args![u32::from(c)]).u32() != 0 || c == 0x22 {
            skipping = false;
        }
        if !skipping {
            let size = word_at(e, line, 0x204);
            e.mem.set_u8(line.wrapping_add(size).wrapping_add(4), c);
            e.mem
                .set_u32(line.wrapping_add(0x204), size.wrapping_add(1));
        }
    }
}

/// `GetNextWord`'s step after a word that ends in `.`: references must be allowed (else
/// reported, 0) and the word must not already name one (else reported, 0); the word must be a
/// referenced object (`GetReferencedObject`, else reported, 0); then the next word is read
/// with references allowed.
#[allow(clippy::too_many_arguments)]
fn read_after_reference(
    e: &mut Engine,
    data: Ptr,
    word: u32,
    text: u32,
    position: u32,
    allow_reference: u8,
    force: u8,
    skip_dot: bool,
    length: u32,
) -> u32 {
    if allow_reference == 0 {
        report(e, data, TEXT_REFERENCE_NOT_ALLOWED);
        return 0;
    }
    if word_at(e, word, 0x200) != 0 {
        report(e, data, TEXT_INVALID_REFERENCE_SYNTAX);
        return 0;
    }
    if skip_dot {
        let at = e.mem.u32(position).wrapping_add(1);
        e.mem.set_u32(position, at);
    } else {
        e.mem.set_u8(word.wrapping_add(length).wrapping_sub(1), 0);
    }
    if !script_compile_data_get_referenced_object(e, data, word, 0, force) {
        script_compiler_script_error(e, data, TEXT_UNKNOWN_OBJECT, &[word]);
        return 0;
    }
    script_compiler_get_next_word(e, data, word, text, position, 1, force)
}

// Translated from 005af5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptCompiler::GetNextWord` (Xbox PDB), `cdecl` (compile data, word, text, address of
/// the position, references allowed, force): reads the next word of the text at the position
/// (after blanks, tabs and commas, `005b1b60`) into the word and answers its length. A quoted
/// word needs matched quotes (`005b1b00`, else reported) and is copied without them; a
/// following `.` makes it a reference. An unquoted word runs over alphanumerics, `.`, `-` and
/// `_`; a `.` after a word starting with a letter makes it a reference. A reference is
/// resolved (`GetReferencedObject`) and the word after it read instead. With references
/// allowed, a word naming a reference must be a variable or a function of it (else
/// reported), the reference must be a reference or a quest unless forced, and commands are
/// not allowed on quests; a word without a kind is looked up as a variable, then a function.
/// An empty quoted word goes on as an unquoted one, as the exe does.
pub fn script_compiler_get_next_word(
    e: &mut Engine,
    data: Ptr,
    word: u32,
    text: u32,
    position: u32,
    allow_reference: u8,
    force: u8,
) -> u32 {
    fn_005b1b60(e, text, position);
    let mut length = 0u32;
    let at = |e: &Engine| text.wrapping_add(e.mem.u32(position));
    if e.mem.i8(at(e)) == 0x22 {
        let from = e.mem.u32(position);
        if !fn_005b1b00(e, text, from) {
            report(e, data, TEXT_MISMATCHED_QUOTES);
            return 0;
        }
        let first = e.call(STRCHR, &args![at(e), 0x22u32]).u32();
        e.mem.set_u8(first, 0x20);
        let second = e.call(STRCHR, &args![at(e), 0x22u32]).u32();
        e.mem.set_u8(second, 0);
        e.call(STRCPY_S, &args![word, 0x200u32, first.wrapping_add(1)]);
        length = e.call(TEXT_LENGTH, &args![word]).u32();
        e.mem.set_u8(first, 0x22);
        e.mem.set_u8(second, 0x22);
        let moved = length.wrapping_add(e.mem.u32(position)).wrapping_add(2);
        e.mem.set_u32(position, moved);
        if e.mem.i8(at(e)) == 0x2e {
            return read_after_reference(
                e,
                data,
                word,
                text,
                position,
                allow_reference,
                force,
                true,
                length,
            );
        }
        e.mem.set_u8(word.wrapping_add(length), 0);
    }
    if length == 0 {
        loop {
            let c = e.mem.i8(at(e));
            if e.call(ISALNUM, &args![c as i32]).u32() == 0 && c != 0x2e && c != 0x2d && c != 0x5f {
                break;
            }
            let c = e.mem.u8(at(e));
            e.mem.set_u8(word.wrapping_add(length), c);
            length = length.wrapping_add(1);
            let moved = e.mem.u32(position).wrapping_add(1);
            e.mem.set_u32(position, moved);
            if e.mem.i8(word.wrapping_add(length).wrapping_sub(1)) == 0x2e {
                let first = e.mem.i8(word);
                if e.call(ISALPHA, &args![first as i32]).u32() != 0 {
                    return read_after_reference(
                        e,
                        data,
                        word,
                        text,
                        position,
                        allow_reference,
                        force,
                        false,
                        length,
                    );
                }
            }
            e.mem.set_u8(word.wrapping_add(length), 0);
        }
    }
    if allow_reference == 0 {
        return length;
    }
    if word_at(e, word, 0x200) != 0 {
        if !script_compile_data_get_variable(e, data, word)
            && !script_compiler_get_function(e, word)
        {
            script_compiler_script_error(e, data, TEXT_UNKNOWN_VARIABLE_OR_FUNCTION, &[word]);
            return 0;
        }
        let index = word_at(e, word, 0x200);
        let object = script_compile_data_get_referenced_object_ov2(e, data, index);
        let form = word_at(e, object, 8);
        let reference = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0u32],
            )
            .u32();
        let form = word_at(e, object, 8);
        let quest = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_QUEST, 0u32],
            )
            .u32();
        if reference == 0 && quest == 0 && force == 0 {
            let form = word_at(e, object, 8);
            let name = e.vcall(form, 0x130, &[]).u32();
            script_compiler_script_error(e, data, TEXT_ONLY_QUESTS_AND_REFERENCES, &[name]);
            return 0;
        }
        if quest != 0 && e.mem.i8(word.wrapping_add(0x204)) == 0x58 {
            report(e, data, TEXT_NO_FUNCTIONS_ON_QUESTS);
            return 0;
        }
    }
    if e.mem.i8(word.wrapping_add(0x204)) == 0 && !script_compile_data_get_variable(e, data, word) {
        script_compiler_get_function(e, word);
    }
    length
}

// Translated from 005afa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptCompileData::VerifyReferencedObjectList` (Xbox PDB; `ECX`): true unless a
/// referenced object (until an empty item) has neither a form nor a variable id, which is
/// reported with its name ("Unknown referenced object").
pub fn script_compile_data_verify_referenced_object_list(e: &mut Engine, this: Ptr) -> bool {
    let mut node = this.addr().wrapping_add(0x44);
    loop {
        if node == 0 {
            return true;
        }
        let at = e.call(LIST_ITEM, &args![node]).u32();
        if e.mem.u32(at) == 0 {
            return true;
        }
        let object = node_item(e, node);
        node = e.call(LIST_NEXT, &args![node]).u32();
        if word_at(e, object, 8) == 0 && word_at(e, object, 0xc) == 0 {
            let name = e.call(STRING_TEXT, &args![object]).u32();
            script_compiler_script_error(e, this, TEXT_UNKNOWN_REFERENCED_OBJECT, &[name]);
            return false;
        }
    }
}

// Translated from 005afad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005afad0 - the FS:[0] exception frame (deleting the new object on unwinding) is not translated.
/// `ScriptCompileData::GetReferencedObject` (Xbox PDB, `RET 0xC`: a word, by-variable flag,
/// force flag): finds the word among the referenced objects (`BSStringT::StrCmp`, ignoring
/// case; until an empty item), setting its index (from 1) and form in the word and trading the
/// variable id (the word's to the object with the flag, the object's to the word without).
/// Not found, a new object (`00500910`) is made: by variable it is accepted; otherwise its form
/// is the player for "Player", else the editor id's form (a type 3 form does not count), else
/// the form of the word read as a hex form id; a found form that is a reference must persist
/// (`GetRefPersists`, unless compiler index 1), else it is reported. An accepted or forced
/// object takes the word's form or variable id and its name, is counted and appended, and its
/// index goes to the word; a refused one is deleted (`005aa3f0`). Answers whether it was
/// found or accepted. The exception frame is not translated.
pub fn script_compile_data_get_referenced_object(
    e: &mut Engine,
    this: Ptr,
    word: u32,
    by_variable: u8,
    force: u8,
) -> bool {
    let base = this.addr();
    let mut index = 1u32;
    let mut node = base.wrapping_add(0x44);
    while node != 0 {
        let at = e.call(LIST_ITEM, &args![node]).u32();
        if e.mem.u32(at) == 0 {
            break;
        }
        let object = node_item(e, node);
        if e.call(STRING_COMPARE, &args![object, word, 1u32]).u32() == 0 {
            e.mem.set_u32(word.wrapping_add(0x200), index);
            let object = node_item(e, node);
            let form = word_at(e, object, 8);
            e.mem.set_u32(word.wrapping_add(0x210), form);
            if by_variable != 0 {
                let object = node_item(e, node);
                let id = word_at(e, word, 0x20c);
                e.mem.set_u32(object.wrapping_add(0xc), id);
            } else {
                let object = node_item(e, node);
                let id = word_at(e, object, 0xc);
                e.mem.set_u32(word.wrapping_add(0x20c), id);
            }
            return true;
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
        index = index.wrapping_add(1);
    }
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let object = if block != 0 {
        e.call(REF_OBJECT_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    let mut found = false;
    if by_variable != 0 {
        found = true;
    } else {
        if e.call(STRICMP, &args![word, TEXT_PLAYER]).u32() == 0 {
            let player = e.global::<u32>(PLAYER_REF);
            e.mem.set_u32(object.wrapping_add(8), player);
            found = true;
        }
        if !found {
            let form = e.call(FORM_BY_EDITOR_ID, &args![word]).u32();
            e.mem.set_u32(object.wrapping_add(8), form);
            if word_at(e, object, 8) != 0 {
                let form = word_at(e, object, 8);
                if e.call(FORM_TYPE, &args![form]).u32() == 3 {
                    e.mem.set_u32(object.wrapping_add(8), 0);
                } else {
                    found = true;
                }
            }
        }
        if !found {
            found = e.with_stack(4, |e, end| {
                let id = fn_005afe10(e, word, end.addr(), 0x10);
                let end = e.mem.u32(end.addr());
                if id != 0 && end != 0 && e.mem.i8(end) == 0 {
                    let form = e.call(LOOKUP_FORM, &args![id]).u32();
                    e.mem.set_u32(object.wrapping_add(8), form);
                    return word_at(e, object, 8) != 0;
                }
                false
            });
        }
        if found && word_at(e, base, 8) != 1 {
            let form = word_at(e, object, 8);
            let reference = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0u32],
                )
                .u32();
            if reference != 0 && !e.call(REFR_PERSISTS, &args![reference]).bool() {
                script_compiler_script_error(e, this, TEXT_NON_PERSISTENT, &[word]);
                found = false;
            }
        }
    }
    if !found && force == 0 {
        if object != 0 {
            e.call(REF_OBJECT_DELETE, &args![object, 1u32]);
        }
        return false;
    }
    if by_variable != 0 {
        e.mem.set_u32(object.wrapping_add(8), 0);
        let id = word_at(e, word, 0x20c);
        e.mem.set_u32(object.wrapping_add(0xc), id);
    } else {
        let form = word_at(e, object, 8);
        e.mem.set_u32(word.wrapping_add(0x210), form);
        e.mem.set_u32(object.wrapping_add(0xc), 0);
    }
    e.call(STRING_SET, &args![object, word, 0u32]);
    let count = word_at(e, base, 0x2c).wrapping_add(1);
    e.mem.set_u32(base.wrapping_add(0x2c), count);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), object);
        let list = e.call(REF_OBJECT_LIST_OF, &args![this]).u32();
        e.call(LIST_ADD_TAIL, &args![list, slot]);
    });
    let count = word_at(e, base, 0x2c);
    e.mem.set_u32(word.wrapping_add(0x200), count);
    true
}

// Translated from 005afe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, `cdecl` (text, end address, base): `strtoul` (`00ec8d33`) with the
/// same words, whose result it answers.
pub fn fn_005afe10(e: &mut Engine, text: u32, end: u32, radix: u32) -> u32 {
    e.call(STRTOUL, &args![text, end, radix]).u32()
}

// Translated from 005afe30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptCompileData::GetReferencedObject_ov2` (Xbox PDB, `RET 4`: an index from 1): the
/// referenced object at that index, 0 for index 0 or one above the object count; the walk
/// stops early at an empty list and then answers that node's item.
pub fn script_compile_data_get_referenced_object_ov2(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    if index == 0 || index > word_at(e, this.addr(), 0x2c) {
        return 0;
    }
    let mut at = 1u32;
    let mut node = e.call(REF_OBJECT_LIST_OF, &args![this]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() && at < index {
        at = at.wrapping_add(1);
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    node_item(e, node)
}

// Translated from 005afeb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005afeb0 - the /GS stack cookie check (00ec408c) is not translated.
/// `ScriptCompiler::IsValidVariableName` (Xbox PDB), `cdecl` (a name): false for a null or
/// empty name, an editor id of a form, or a function name (`GetFunction` on a word holding
/// it); true otherwise. The stack cookie check is not translated.
pub fn script_compiler_is_valid_variable_name(e: &mut Engine, name: u32) -> bool {
    if name == 0 || e.mem.i8(name) == 0 {
        return false;
    }
    if e.call(FORM_BY_EDITOR_ID, &args![name]).u32() != 0 {
        return false;
    }
    e.with_stack(0x214, |e, word| {
        fn_005af310(e, word);
        e.call(STRCPY_S, &args![word, 0x200u32, name]);
        !script_compiler_get_function(e, word.addr())
    })
}

/// Reports `format` with the line's number set as the last line number, for the block
/// checks.
fn report_at_line(e: &mut Engine, data: Ptr, line: u32, format: u32) -> bool {
    let number = e.mem.u32(line);
    e.mem.set_u32(data.addr().wrapping_add(0x1c), number);
    report(e, data, format);
    false
}

// Translated from 005b1180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptCompiler::VerifyBlocks` (Xbox PDB, `RET 4`: the compile data): walks the lines: a
/// `begin` (0x10) inside a block, an `end` (0x11) outside one, or any command but the
/// declarations (0x12, 0x13, 0x14, 0x1F) and `scriptname` (0x1D) outside a block is reported
/// (with that line as the last line) and fails. The exe's jump table at `005b129c` with the
/// byte map at `005b12ac` is the `match`.
pub fn script_compiler_verify_blocks(e: &mut Engine, _this: Ptr, data: Ptr) -> bool {
    let mut inside = false;
    let mut node = e.call(LINE_LIST_OF, &args![data]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let line = node_item(e, node);
        match word_at(e, line, 0x410) {
            0x10 => {
                if inside {
                    return report_at_line(e, data, line, TEXT_NESTED_BLOCKS);
                }
                inside = true;
            }
            0x11 => {
                if !inside {
                    return report_at_line(e, data, line, TEXT_END_WITHOUT_BEGIN);
                }
                inside = false;
            }
            0x12 | 0x13 | 0x14 | 0x1d | 0x1f => {}
            _ => {
                if !inside {
                    return report_at_line(e, data, line, TEXT_OUTSIDE_BLOCK);
                }
            }
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    true
}

// Translated from 005b12c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptCompiler::CountBlockLines` (Xbox PDB, `RET 8`: the compile data, a line): counts
/// the lines after the given one (by number) up to its block's `end` (0x11), skipping the
/// declarations (0x12..=0x14). A nested `begin` (0x10), a line with opcode 0xFFFF ("Begin
/// without matching end", with the line's number) or no `end` ("Mismatched begin/end block")
/// is reported and answers -1.
pub fn script_compiler_count_block_lines(e: &mut Engine, _this: Ptr, data: Ptr, line: u32) -> i32 {
    let mut count = 0i32;
    let mut done = false;
    let mut node = e.call(LINE_LIST_OF, &args![data]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() && !done {
        let other = node_item(e, node);
        if e.mem.u32(other) > e.mem.u32(line) {
            let opcode = e.mem.i32(other.wrapping_add(0x410));
            if opcode > 0x14 {
                if opcode == 0xffff {
                    let number = e.mem.u32(line);
                    script_compiler_script_error(e, data, TEXT_BEGIN_WITHOUT_END, &[number]);
                    return -1;
                }
                count = count.wrapping_add(1);
            } else if opcode >= 0x12 {
            } else if opcode == 0x10 {
                report(e, data, TEXT_NESTED_BLOCKS);
                return -1;
            } else if opcode == 0x11 {
                done = true;
            } else {
                count = count.wrapping_add(1);
            }
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    if !done {
        let number = e.mem.u32(line);
        script_compiler_script_error(e, data, TEXT_MISMATCHED_BLOCK, &[number]);
        return -1;
    }
    count
}

// Translated from 005b13e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptCompiler::CountIfLines` (Xbox PDB, `RET 8`: the compile data, a line): counts the
/// lines after the given `if` up to its matching `elseif`/`else` (0x17, 0x18) or `endif`
/// (0x19) at depth 0, skipping the declarations (0x12..=0x14); nested `if`s (0x16) count and
/// deepen (10 or more is reported, -1), their `else`s and `endif`s count. A line with opcode
/// 0xFFFF ("Mismatched if/else/endif.") or no end ("Mismatched if/else/endif block", with the
/// line's number) is reported and answers -1.
pub fn script_compiler_count_if_lines(e: &mut Engine, _this: Ptr, data: Ptr, line: u32) -> i32 {
    let mut count = 0i32;
    let mut depth = 0i32;
    let mut done = false;
    let mut node = e.call(LINE_LIST_OF, &args![data]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() && !done {
        let other = node_item(e, node);
        if e.mem.u32(other) > e.mem.u32(line) {
            let opcode = e.mem.i32(other.wrapping_add(0x410));
            if opcode > 0x18 {
                if opcode == 0x19 {
                    if depth == 0 {
                        done = true;
                    } else {
                        depth -= 1;
                        count = count.wrapping_add(1);
                    }
                } else if opcode == 0xffff {
                    report(e, data, TEXT_MISMATCHED_IF);
                    return -1;
                } else {
                    count = count.wrapping_add(1);
                }
            } else if opcode >= 0x17 {
                if depth == 0 {
                    done = true;
                } else {
                    count = count.wrapping_add(1);
                }
            } else if opcode < 0x12 {
                count = count.wrapping_add(1);
            } else if opcode <= 0x14 {
            } else if opcode == 0x16 {
                count = count.wrapping_add(1);
                depth += 1;
                if depth >= 10 {
                    script_compiler_script_error(e, data, TEXT_MAX_NESTING, &[10]);
                    return -1;
                }
            } else {
                count = count.wrapping_add(1);
            }
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    if !done {
        let number = e.mem.u32(line);
        script_compiler_script_error(e, data, TEXT_MISMATCHED_IF_BLOCK, &[number]);
        return -1;
    }
    count
}

/// Appends a byte to the output at `out + *count`.
fn put_u8(e: &mut Engine, out: u32, count: &mut u32, value: u8) {
    e.mem.set_u8(out.wrapping_add(*count), value);
    *count = count.wrapping_add(1);
}

/// Appends a 16-bit word to the output at `out + *count`.
fn put_u16(e: &mut Engine, out: u32, count: &mut u32, value: u16) {
    e.mem.set_u16(out.wrapping_add(*count), value);
    *count = count.wrapping_add(2);
}

// Translated from 005b1570 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005b1570 - the /GS stack cookie check (00ec408c) is not translated.
/// `ScriptCompiler::ReplaceVariablesAndFunctions` (Xbox PDB, `RET 0x10`: the compile data,
/// the address of a line number, the text, the output): copies the text into the output with
/// every word starting with a letter or `"` (`GetNextWord`, references allowed, forced)
/// replaced by its code: a reference prefix (`'G'` for a global, else `'r'`, and the 16-bit
/// index); then a variable (its kind and 16-bit id), or a command (`'X'`, the opcode; its
/// definition must exist and, in a quest script, a reference command needs an explicit
/// reference; with parameters, a `SCRIPT_LINE` on the stack holding the rest of the text and
/// the line number goes to the definition's compile function, whose output size (16 bits)
/// and output are appended and whose offset is skipped; without, a 16-bit 0), or a
/// referenced object (`'Z'` and its index; reported when unknown). Other characters are
/// copied. Answers the output length (a 0 after it), 0 on an error. The stack cookie check is
/// not translated.
pub fn script_compiler_replace_variables_and_functions(
    e: &mut Engine,
    _this: Ptr,
    data: Ptr,
    line_number: u32,
    text: u32,
    out: u32,
) -> u32 {
    let length = e.call(TEXT_LENGTH, &args![text]).u32();
    let mut count = 0u32;
    // Locals: +0 the position, +4 the word (0x214), +0x218 the line (0x41C).
    e.with_stack(0x634, |e, frame| {
        let position = frame.addr();
        let word = position + 4;
        let line = position + 0x218;
        e.mem.set_u32(position, 0);
        while e.mem.u32(position) < length {
            let c = e.mem.i8(text.wrapping_add(e.mem.u32(position)));
            if e.call(ISALPHA, &args![c as i32]).u32() != 0 || c == 0x22 {
                fn_005af310(e, Ptr::new(word));
                if script_compiler_get_next_word(e, data, word, text, position, 1, 1) == 0 {
                    return 0;
                }
                if word_at(e, word, 0x200) != 0 {
                    let prefix = if e.mem.i8(word + 0x204) == 0x47 {
                        0x47
                    } else {
                        0x72
                    };
                    put_u8(e, out, &mut count, prefix);
                    let index = e.mem.u16(word + 0x200);
                    put_u16(e, out, &mut count, index);
                }
                if word_at(e, word, 0x20c) != 0 {
                    let kind = e.mem.u8(word + 0x204);
                    put_u8(e, out, &mut count, kind);
                    let id = e.mem.u16(word + 0x20c);
                    put_u16(e, out, &mut count, id);
                } else if e.mem.i8(word + 0x204) == 0x58 {
                    put_u8(e, out, &mut count, 0x58);
                    let opcode = e.mem.u16(word + 0x208);
                    put_u16(e, out, &mut count, opcode);
                    let opcode = e.mem.i32(word + 0x208);
                    let definition = script_compiler_get_function_def(e, opcode).addr();
                    if definition == 0 {
                        script_compiler_script_error(e, data, TEXT_UNDEFINED_FUNCTION, &[word]);
                        return 0;
                    }
                    if e.mem.u8(definition.wrapping_add(0x10)) != 0
                        && e.mem.u8(data.addr().wrapping_add(0x38)) != 0
                        && word_at(e, word, 0x200) == 0
                    {
                        script_compiler_script_error(
                            e,
                            data,
                            TEXT_NEEDS_EXPLICIT_REFERENCE,
                            &[word],
                        );
                        return 0;
                    }
                    if word_at(e, definition, 0x14) != 0 {
                        let parameters = word_at(e, definition, 0x14);
                        let compile = word_at(e, definition, 0x1c);
                        let parameter_count = e.mem.u16(definition.wrapping_add(0x12));
                        script_line_script_line(e, Ptr::new(line));
                        let rest = text.wrapping_add(e.mem.u32(position));
                        e.call(STRCPY_S, &args![line + 4, 0x200u32, rest]);
                        e.mem.set_u32(line + 0x40c, 0);
                        let number = e.mem.u32(line_number);
                        e.mem.set_u32(line, number);
                        let ok = e
                            .call(
                                compile,
                                &args![u32::from(parameter_count), parameters, line, data],
                            )
                            .bool();
                        if !ok {
                            return 0;
                        }
                        let size = e.mem.u16(line + 0x40c);
                        put_u16(e, out, &mut count, size);
                        let size = e.mem.u32(line + 0x40c);
                        e.call(MEMCPY, &args![out.wrapping_add(count), line + 0x20c, size]);
                        count = count.wrapping_add(e.mem.u32(line + 0x40c));
                        let moved = e.mem.u32(position).wrapping_add(e.mem.u32(line + 0x208));
                        e.mem.set_u32(position, moved);
                    } else {
                        put_u16(e, out, &mut count, 0);
                    }
                } else if e.mem.i8(word + 0x204) != 0x47 {
                    if word_at(e, word, 0x200) == 0
                        && script_compile_data_get_referenced_object(e, data, word, 0, 0)
                    {
                        put_u8(e, out, &mut count, 0x5a);
                        let index = e.mem.u16(word + 0x200);
                        put_u16(e, out, &mut count, index);
                    } else {
                        script_compiler_script_error(e, data, TEXT_UNKNOWN_COMMAND, &[word]);
                        return 0;
                    }
                }
            }
            if e.mem.u32(position) < length {
                let c = e.mem.u8(text.wrapping_add(e.mem.u32(position)));
                put_u8(e, out, &mut count, c);
            }
            let moved = e.mem.u32(position).wrapping_add(1);
            e.mem.set_u32(position, moved);
        }
        e.mem.set_u8(out.wrapping_add(count).wrapping_add(1), 0);
        count
    })
}

/// Looks the word up in one command table (`count` entries of 0x28 bytes: long name, short
/// name, opcode); `name_first` passes the table's name as the first `_stricmp` word.
fn find_in_table(e: &mut Engine, word: u32, table: u32, count: i32, name_first: bool) -> bool {
    let mut index = 0i32;
    while index < count {
        let entry = table.wrapping_add((index as u32).wrapping_mul(0x28));
        let mut matched = false;
        for name_at in [entry, entry.wrapping_add(4)] {
            let name = e.mem.u32(name_at);
            let words = if name_first {
                args![name, word]
            } else {
                args![word, name]
            };
            if e.call(STRICMP, &words).u32() == 0 {
                matched = true;
                break;
            }
        }
        if matched {
            e.mem.set_u8(word.wrapping_add(0x204), 0x58);
            let opcode = e.mem.u32(entry.wrapping_add(8));
            e.mem.set_u32(word.wrapping_add(0x208), opcode);
            return true;
        }
        index += 1;
    }
    false
}

// Translated from 005b1910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptCompiler::GetFunction` (Xbox PDB), `cdecl` (a word): finds the word by long or short
/// name (`_stricmp`) in the block table (`0118cb50`, 16 entries, compared name first), the
/// script command table (`01190910`, 0x280 entries) and the console command table
/// (`0118e8e0`, 0xCD entries); a match makes it a command (kind `'X'`, the entry's opcode):
/// true.
pub fn script_compiler_get_function(e: &mut Engine, word: u32) -> bool {
    find_in_table(e, word, BLOCK_TABLE, 0x10, true)
        || find_in_table(e, word, SCRIPT_COMMANDS, 0x280, false)
        || find_in_table(e, word, CONSOLE_COMMANDS, 0xcd, false)
}

// Translated from 005b1a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, `cdecl` (text, start): whether the parentheses from the start to the
/// end of the text balance (`(` counts up, `)` down, the sum 0).
pub fn fn_005b1a90(e: &mut Engine, text: u32, start: u32) -> bool {
    let mut depth = 0i32;
    let mut at = start;
    loop {
        let c = e.mem.u8(text.wrapping_add(at));
        if c == 0 {
            break;
        }
        if c == 0x28 {
            depth += 1;
        } else if c == 0x29 {
            depth -= 1;
        }
        at = at.wrapping_add(1);
    }
    depth == 0
}

// Translated from 005b1b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, `cdecl` (text, start): whether the text from the start holds an even
/// number (at least 2) of `"`.
pub fn fn_005b1b00(e: &mut Engine, text: u32, start: u32) -> bool {
    let mut quotes = 0i32;
    let mut at = start;
    loop {
        let c = e.mem.u8(text.wrapping_add(at));
        if c == 0 {
            break;
        }
        if c == 0x22 {
            quotes += 1;
        }
        at = at.wrapping_add(1);
    }
    quotes >= 2 && quotes & 1 == 0
}

// Translated from 005b1b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, `cdecl` (text, address of a position): moves the position past tabs,
/// blanks and commas.
pub fn fn_005b1b60(e: &mut Engine, text: u32, position: u32) {
    loop {
        let c = e.mem.u8(text.wrapping_add(e.mem.u32(position)));
        if c != 0x9 && c != 0x20 && c != 0x2c {
            break;
        }
        let moved = e.mem.u32(position).wrapping_add(1);
        e.mem.set_u32(position, moved);
    }
}

/// `CExpression::CExpression` (engine map, `00593490`, `ECX`), the conversion to postfix
/// `00593500` (`ECX` = expression, `RET 8`: infix text, address of its length; answers the
/// postfix bytes) and `00483710` (`ECX`; an empty body the exe calls as the expression's
/// destructor).
const EXPRESSION_CONSTRUCT: u32 = 0x0059_3490;
const EXPRESSION_TO_POSTFIX: u32 = 0x0059_3500;
const EXPRESSION_DESTRUCT: u32 = 0x0048_3710;
/// The expression error messages, by error code (`0118c594`, "Expression Error: NOERROR"
/// first).
const EXPRESSION_ERRORS: u32 = 0x0118_c594;
/// The `begin` block type table (`0118e2f0`, 0x26 `SCRIPT_FUNCTION` entries, "GameMode"
/// first).
const BLOCK_TYPES: u32 = 0x0118_e2f0;
/// The output offset of the jump word of the `begin` block being compiled (`011cadbc`), 0
/// outside a block.
const BLOCK_JUMP: u32 = 0x011c_adbc;
/// `005e28b0` (`ECX` = a new 0x20-byte block): constructs a `ScriptVariable`.
const VARIABLE_CONSTRUCT: u32 = 0x005e_28b0;
/// `tolower` (`00ec67aa`, `cdecl`).
const TOLOWER: u32 = 0x00ec_67aa;
/// The 0x4000-byte limit of a compiled script.
const MAX_SCRIPT_SIZE: u32 = 0x4000;

const TEXT_MISMATCHED_BEGIN_END: u32 = 0x0103_7b8c;
const TEXT_BLOCK_NOT_FOR_QUESTS: u32 = 0x0103_7b64;
const TEXT_INVALID_BLOCK_TYPE: u32 = 0x0103_7b2c;
const TEXT_MISSING_BLOCK_TYPE: u32 = 0x0103_7af4;
const TEXT_NO_JUMP_BLOCK: u32 = 0x0103_7ab4;
const TEXT_INVALID_VARIABLE_NAME: u32 = 0x0103_7a7c;
const TEXT_MISSING_VARIABLE_NAME: u32 = 0x0103_7a4c;
const TEXT_SET_MISSING_VARIABLE: u32 = 0x0103_7a24;
const TEXT_UNKNOWN_VARIABLE: u32 = 0x0103_7a0c;
const TEXT_MISSING_TO: u32 = 0x0103_79e0;
const TEXT_SET_MISMATCHED_PARENTHESES: u32 = 0x0103_79a8;
const TEXT_SET_MISSING_EXPRESSION: u32 = 0x0103_7974;
const TEXT_INFIX_TO_POSTFIX: u32 = 0x0103_795c;
const TEXT_MISMATCHED_PARENTHESES: u32 = 0x0103_7944;
const TEXT_MISMATCHED_IF_THEN_ELSE: u32 = 0x0103_7924;
const TEXT_UNKNOWN_FUNCTION_CODE: u32 = 0x0103_7908;
const TEXT_FUNCTION_NEEDS_REFERENCE: u32 = 0x0103_78c8;
const TEXT_ONLY_REFERENCES: u32 = 0x0103_7850;
const TEXT_MAX_SCRIPT_SIZE: u32 = 0x0103_7818;

/// Appends a byte to a line's output (`pOutput + uiOutputSize`).
fn line_put_u8(e: &mut Engine, line: u32, value: u8) {
    let size = word_at(e, line, 0x40c);
    e.mem
        .set_u8(line.wrapping_add(size).wrapping_add(0x20c), value);
    e.mem
        .set_u32(line.wrapping_add(0x40c), size.wrapping_add(1));
}

/// Appends a 16-bit word to a line's output.
fn line_put_u16(e: &mut Engine, line: u32, value: u16) {
    let size = word_at(e, line, 0x40c);
    e.mem
        .set_u16(line.wrapping_add(size).wrapping_add(0x20c), value);
    e.mem
        .set_u32(line.wrapping_add(0x40c), size.wrapping_add(2));
}

/// Appends `size` bytes from `source` to a line's output (`00401460`).
fn line_put_bytes(e: &mut Engine, line: u32, source: u32, size: u32) {
    let at = line
        .wrapping_add(word_at(e, line, 0x40c))
        .wrapping_add(0x20c);
    e.call(MEMCPY, &args![at, source, size]);
    let total = word_at(e, line, 0x40c).wrapping_add(size);
    e.mem.set_u32(line.wrapping_add(0x40c), total);
}

/// Runs a block type's compile function (`SCRIPT_FUNCTION` +0x1C) on a stack `SCRIPT_LINE`
/// holding the rest of `line`'s text (from its offset) and its number, as `CompileLine` does
/// for a `begin`; answers the compile function's result and appends its output to `line`'s.
fn compile_parameters_on_copy(e: &mut Engine, data: Ptr, line: u32, definition: u32) -> bool {
    let parameters = word_at(e, definition, 0x14);
    let compile = word_at(e, definition, 0x1c);
    let count = e.mem.u16(definition.wrapping_add(0x12));
    e.with_stack(0x41c, |e, copy| {
        let copy = copy.addr();
        script_line_script_line(e, Ptr::new(copy));
        let rest = line.wrapping_add(word_at(e, line, 0x208)).wrapping_add(4);
        e.call(STRCPY_S, &args![copy + 4, 0x200u32, rest]);
        e.mem.set_u32(copy + 0x40c, 0);
        let number = e.mem.u32(line);
        e.mem.set_u32(copy, number);
        if !e
            .call(compile, &args![u32::from(count), parameters, copy, data])
            .bool()
        {
            return false;
        }
        let size = e.mem.u32(copy + 0x40c);
        line_put_bytes(e, line, copy + 0x20c, size);
        true
    })
}

/// `CompileLine`'s expression step: clears the 0x200-byte infix buffer (`memset`) and its
/// length, and replaces the variables and functions of the line's text from `start`
/// (`ReplaceVariablesAndFunctions`, the line itself giving the line number) into it; false
/// when that gives no bytes.
fn compile_expression(
    e: &mut Engine,
    this: Ptr,
    data: Ptr,
    line: u32,
    start: u32,
    infix: u32,
    infix_length: u32,
) -> bool {
    e.call(MEMSET, &args![infix, 0u32, 0x200u32]);
    e.mem.set_u32(infix_length, 0);
    let text = line.wrapping_add(start).wrapping_add(4);
    let length = script_compiler_replace_variables_and_functions(e, this, data, line, text, infix);
    e.mem.set_u32(infix_length, length);
    length != 0
}

// Translated from 005aff40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005aff40 - the FS:[0] exception frame and the /GS stack cookie check (00ec408c) are not translated.
/// `ScriptCompiler::CompileLine` (Xbox PDB, `RET 0xC`: the script, the compile data, the
/// line): compiles one line into its output (`pOutput`, size `uiOutputSize`) and appends it to
/// the script's output. By opcode (the exe's jump table at `005b10d8`):
/// - `begin` (0x10): the block type word must be in the block type table (`0118e2f0`, long or
///   short name); its code, a jump word 0x55555555 whose output offset `011cadbc` keeps
///   (the block's line count `CountBlockLines` must not fail), and, for a block type with
///   parameters, the compile function's output; block types marked so are refused in quest
///   scripts. The jump word then holds the output offset after this line.
/// - `end` (0x11): the open block's jump word becomes the bytes from it to here; no open
///   block is reported.
/// - declarations (0x12, 0x13, 0x14, 0x1F): a valid name (`IsValidVariableName`) makes a new
///   variable (`005e28b0`; the byte at +0x10 set except for 0x14 and 0x1F), numbered after the
///   last id unless the script already has it (`Script::FindVariable`), appended to the
///   variables; a reference variable (0x1F) also becomes a referenced object.
/// - `set` (0x15): a variable (reference prefix and kind/id), `to`, balanced parentheses and
///   an expression, replaced (`ReplaceVariablesAndFunctions`) and converted to postfix
///   (`00593500`), its 16-bit length and bytes.
/// - `if`/`elseif` (0x16, 0x18): the condition the same way (its error message from the table
///   at `0118c594`), preceded by the line count to the matching branch (`CountIfLines`).
/// - `else` (0x17): the line count; `endif` (0x19), 0x1C, `scriptname` (0x1D), 0x1E: nothing.
/// - other commands: the definition must exist; reference commands need an explicit
///   reference in quest scripts, and their reference must be a reference or a variable;
///   parameters go through the definition's compile function on this line.
///
/// Except for declarations, the line is then written to the script's output (the limit is
/// 0x4000 bytes with 10 to spare): a reference (0x1C and its index), the opcode, the output
/// size and the output. Every exit calls the expression's empty destructor `00483710`. The
/// stack probe `__alloca_probe` (`00ec8660`) of the frame is left out (the frame is a stack
/// block of the engine); the exception frame and the stack cookie check are not translated.
pub fn script_compiler_compile_line(
    e: &mut Engine,
    this: Ptr,
    script: u32,
    data: Ptr,
    line: u32,
) -> bool {
    let base = data.addr();
    // Locals: +0 the word (0x214), +0x214 the condition position, +0x218 a second word
    // (0x214), +0x42c the expression (0x720), +0xb4c the infix buffer (0x200), +0xd4c its
    // length, +0xd50 the new variable.
    e.with_stack(0xd54, |e, frame| {
        let word = frame.addr();
        let position = word + 0x214;
        let second = word + 0x218;
        let expression = word + 0x42c;
        let infix = word + 0xb4c;
        let infix_length = word + 0xd4c;
        let variable_at = word + 0xd50;
        fn_005af310(e, Ptr::new(word));
        e.mem.set_u32(position, 0);
        e.call(EXPRESSION_CONSTRUCT, &args![expression]);
        let fail = |e: &mut Engine| {
            e.call(EXPRESSION_DESTRUCT, &args![expression]);
            false
        };
        let fail_with = |e: &mut Engine, format: u32| {
            report(e, data, format);
            e.call(EXPRESSION_DESTRUCT, &args![expression]);
            false
        };
        e.mem.set_u32(line.wrapping_add(0x40c), 0);
        e.mem.set_u32(line.wrapping_add(0x208), 0);
        let text = line.wrapping_add(4);
        let offset = line.wrapping_add(0x208);
        let opcode = word_at(e, line, 0x410);
        match opcode.wrapping_sub(0x10) {
            0x0 => {
                if script_compiler_get_next_word(e, data, word, text, offset, 0, 0) == 0 {
                    return fail_with(e, TEXT_MISSING_BLOCK_TYPE);
                }
                let mut found = false;
                let mut index = 0i32;
                while index < 0x26 {
                    let entry = BLOCK_TYPES.wrapping_add((index as u32).wrapping_mul(0x28));
                    let name = e.mem.u32(entry);
                    if e.call(STRICMP, &args![name, word]).u32() == 0 {
                        found = true;
                        break;
                    }
                    let short = e.mem.u32(entry.wrapping_add(4));
                    if e.call(STRICMP, &args![short, word]).u32() == 0 {
                        found = true;
                        break;
                    }
                    index += 1;
                }
                if !found {
                    return fail_with(e, TEXT_INVALID_BLOCK_TYPE);
                }
                let entry = BLOCK_TYPES.wrapping_add((index as u32).wrapping_mul(0x28));
                let code = e.mem.u16(entry.wrapping_add(8));
                line_put_u16(e, line, code);
                if script_compiler_count_block_lines(e, this, data, line) == -1 {
                    return fail_with(e, TEXT_MISMATCHED_BEGIN_END);
                }
                let size = word_at(e, line, 0x40c);
                e.mem
                    .set_u32(line.wrapping_add(size).wrapping_add(0x20c), 0x5555_5555);
                let jump = word_at(e, base, 0x24).wrapping_add(word_at(e, line, 0x40c));
                e.set_global::<u32>(BLOCK_JUMP, jump);
                let size = word_at(e, line, 0x40c).wrapping_add(4);
                e.mem.set_u32(line.wrapping_add(0x40c), size);
                if entry != 0 {
                    if e.mem.u8(entry.wrapping_add(0x10)) != 0
                        && e.mem.u8(base.wrapping_add(0x38)) != 0
                    {
                        return fail_with(e, TEXT_BLOCK_NOT_FOR_QUESTS);
                    }
                    if entry != 0
                        && word_at(e, entry, 0x14) != 0
                        && !compile_parameters_on_copy(e, data, line, entry)
                    {
                        return fail(e);
                    }
                }
                let jump = e.global::<u32>(BLOCK_JUMP);
                if jump != 0 {
                    let value = word_at(e, base, 0x24).wrapping_add(word_at(e, line, 0x40c));
                    let at = jump
                        .wrapping_sub(word_at(e, base, 0x24))
                        .wrapping_add(line)
                        .wrapping_add(0x20c);
                    e.mem.set_u32(at, value);
                }
            }
            0x1 => {
                let jump = e.global::<u32>(BLOCK_JUMP);
                if jump == 0 {
                    return fail_with(e, TEXT_NO_JUMP_BLOCK);
                }
                let at = jump.wrapping_add(word_at(e, base, 0x20)).wrapping_add(4);
                let start = e.mem.u32(at);
                let length = word_at(e, base, 0x24).wrapping_sub(start);
                let at = e
                    .global::<u32>(BLOCK_JUMP)
                    .wrapping_add(word_at(e, base, 0x20))
                    .wrapping_add(4);
                e.mem.set_u32(at, length);
                e.set_global::<u32>(BLOCK_JUMP, 0);
            }
            0x2 | 0x3 | 0x4 | 0xf => {
                if script_compiler_get_next_word(e, data, word, text, offset, 0, 0) == 0 {
                    return fail_with(e, TEXT_MISSING_VARIABLE_NAME);
                }
                if !script_compiler_is_valid_variable_name(e, word) {
                    script_compiler_script_error(e, data, TEXT_INVALID_VARIABLE_NAME, &[word]);
                    return fail(e);
                }
                let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
                let variable = if block != 0 {
                    e.call(VARIABLE_CONSTRUCT, &args![block]).u32()
                } else {
                    0
                };
                e.mem.set_u32(variable_at, variable);
                let opcode = word_at(e, line, 0x410);
                let flag = u8::from(opcode != 0x14 && opcode != 0x1f);
                e.mem.set_u8(variable.wrapping_add(0x10), flag);
                e.call(STRING_SET, &args![variable.wrapping_add(0x18), word, 0u32]);
                let name = e
                    .call(STRING_TEXT, &args![variable.wrapping_add(0x18)])
                    .u32();
                let known = e
                    .call(SCRIPT_FIND_VARIABLE, &args![script, name, variable])
                    .u8();
                if known == 0 {
                    let id = word_at(e, base, 0x34).wrapping_add(1);
                    e.mem.set_u32(base.wrapping_add(0x34), id);
                    e.mem.set_u32(variable, id);
                }
                let list = e.call(VARIABLE_LIST_OF, &args![data]).u32();
                e.call(LIST_ADD_TAIL, &args![list, variable_at]);
                if word_at(e, line, 0x410) == 0x1f {
                    let variable = e.mem.u32(variable_at);
                    let id = e.mem.u32(variable);
                    e.mem.set_u32(word + 0x20c, id);
                    script_compile_data_get_referenced_object(e, data, word, 1, 0);
                }
            }
            0x5 => {
                if script_compiler_get_next_word(e, data, word, text, offset, 1, 1) == 0 {
                    return fail_with(e, TEXT_SET_MISSING_VARIABLE);
                }
                let kind = e.mem.i8(word + 0x204);
                if kind == 0 || kind == 0x58 {
                    script_compiler_script_error(e, data, TEXT_UNKNOWN_VARIABLE, &[word]);
                    return fail(e);
                }
                fn_005af310(e, Ptr::new(second));
                script_compiler_get_next_word(e, data, second, text, offset, 0, 0);
                let first = e.mem.i8(second);
                let mut is_to = e.call(TOLOWER, &args![first as i32]).u32() == 0x74;
                if is_to {
                    let next = e.mem.i8(second + 1);
                    is_to = e.call(TOLOWER, &args![next as i32]).u32() == 0x6f;
                }
                if !is_to {
                    return fail_with(e, TEXT_MISSING_TO);
                }
                if word_at(e, word, 0x200) != 0 {
                    let prefix = if e.mem.i8(word + 0x204) == 0x47 {
                        0x47
                    } else {
                        0x72
                    };
                    line_put_u8(e, line, prefix);
                    let index = e.mem.u16(word + 0x200);
                    line_put_u16(e, line, index);
                }
                if word_at(e, word, 0x20c) != 0 {
                    let kind = e.mem.u8(word + 0x204);
                    line_put_u8(e, line, kind);
                    let id = e.mem.u16(word + 0x20c);
                    line_put_u16(e, line, id);
                }
                let start = word_at(e, line, 0x208);
                if !fn_005b1a90(e, text, start) {
                    return fail_with(e, TEXT_SET_MISMATCHED_PARENTHESES);
                }
                fn_005b1b60(e, text, offset);
                let start = word_at(e, line, 0x208);
                if !compile_expression(e, this, data, line, start, infix, infix_length) {
                    return fail_with(e, TEXT_SET_MISSING_EXPRESSION);
                }
                let postfix = e
                    .call(
                        EXPRESSION_TO_POSTFIX,
                        &args![expression, infix, infix_length],
                    )
                    .u32();
                if e.call(STRING_TEXT, &args![expression]).u32() != 0 {
                    return fail_with(e, TEXT_INFIX_TO_POSTFIX);
                }
                let length = e.mem.u16(infix_length);
                line_put_u16(e, line, length);
                let length = e.mem.u32(infix_length);
                line_put_bytes(e, line, postfix, length);
            }
            0x6 | 0x8 => {
                e.mem.set_u32(position, 0);
                let start = e.mem.u32(position);
                if !fn_005b1a90(e, text, start) {
                    return fail_with(e, TEXT_MISMATCHED_PARENTHESES);
                }
                fn_005b1b60(e, text, position);
                let start = e.mem.u32(position);
                if !compile_expression(e, this, data, line, start, infix, infix_length) {
                    return fail(e);
                }
                let postfix = e
                    .call(
                        EXPRESSION_TO_POSTFIX,
                        &args![expression, infix, infix_length],
                    )
                    .u32();
                if e.call(STRING_TEXT, &args![expression]).u32() != 0 {
                    let code = e.call(STRING_TEXT, &args![expression]).u32();
                    let message = e
                        .mem
                        .u32(EXPRESSION_ERRORS.wrapping_add(code.wrapping_mul(4)));
                    report(e, data, message);
                    return fail(e);
                }
                let lines = script_compiler_count_if_lines(e, this, data, line);
                if lines == -1 {
                    return fail_with(e, TEXT_MISMATCHED_IF_THEN_ELSE);
                }
                line_put_u16(e, line, lines as u16);
                let length = e.mem.u16(infix_length);
                line_put_u16(e, line, length);
                let length = e.mem.u32(infix_length);
                line_put_bytes(e, line, postfix, length);
            }
            0x7 => {
                let lines = script_compiler_count_if_lines(e, this, data, line);
                if lines == -1 {
                    return fail_with(e, TEXT_MISMATCHED_IF_THEN_ELSE);
                }
                line_put_u16(e, line, lines as u16);
            }
            0x9 | 0xc | 0xd | 0xe => {}
            _ => {
                let opcode = word_at(e, line, 0x410);
                let definition = script_compiler_get_function_def(e, opcode as i32).addr();
                if definition == 0 {
                    script_compiler_script_error(e, data, TEXT_UNKNOWN_FUNCTION_CODE, &[opcode]);
                    return fail(e);
                }
                // The exe tests the definition for null once more (never null here).
                let checked = definition != 0;
                if checked
                    && e.mem.u8(definition.wrapping_add(0x10)) != 0
                    && e.mem.u8(base.wrapping_add(0x38)) != 0
                    && word_at(e, line, 0x414) == 0
                {
                    return fail_with(e, TEXT_FUNCTION_NEEDS_REFERENCE);
                }
                if checked
                    && e.mem.u8(definition.wrapping_add(0x10)) != 0
                    && word_at(e, line, 0x414) != 0
                {
                    let index = word_at(e, line, 0x414);
                    let object = script_compile_data_get_referenced_object_ov2(e, data, index);
                    let form = word_at(e, object, 8);
                    let reference = e
                        .call(
                            RT_DYNAMIC_CAST,
                            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0u32],
                        )
                        .u32();
                    if reference == 0 && word_at(e, object, 0xc) == 0 {
                        let name = e.call(STRING_TEXT, &args![object]).u32();
                        script_compiler_script_error(e, data, TEXT_ONLY_REFERENCES, &[name]);
                        return fail(e);
                    }
                }
                if checked && word_at(e, definition, 0x14) != 0 {
                    let parameters = word_at(e, definition, 0x14);
                    let compile = word_at(e, definition, 0x1c);
                    let count = e.mem.u16(definition.wrapping_add(0x12));
                    if !e
                        .call(compile, &args![u32::from(count), parameters, line, data])
                        .bool()
                    {
                        return fail(e);
                    }
                }
            }
        }
        let opcode = e.mem.i32(line.wrapping_add(0x410));
        let declaration = opcode >= 0x12 && (opcode <= 0x14 || opcode == 0x1f);
        if !declaration {
            let end = word_at(e, base, 0x24)
                .wrapping_add(word_at(e, line, 0x40c))
                .wrapping_add(0xa);
            if end >= MAX_SCRIPT_SIZE {
                return fail_with(e, TEXT_MAX_SCRIPT_SIZE);
            }
            let put = |e: &mut Engine, value: u16| {
                let at = word_at(e, base, 0x20).wrapping_add(word_at(e, base, 0x24));
                e.mem.set_u16(at, value);
                let offset = word_at(e, base, 0x24).wrapping_add(2);
                e.mem.set_u32(base.wrapping_add(0x24), offset);
            };
            if word_at(e, line, 0x414) != 0 {
                put(e, 0x1c);
                let index = e.mem.u16(line.wrapping_add(0x414));
                put(e, index);
            }
            let opcode = e.mem.u16(line.wrapping_add(0x410));
            put(e, opcode);
            let size = e.mem.u16(line.wrapping_add(0x40c));
            put(e, size);
            let at = word_at(e, base, 0x20).wrapping_add(word_at(e, base, 0x24));
            let size = word_at(e, line, 0x40c);
            e.call(MEMCPY, &args![at, line.wrapping_add(0x20c), size]);
            let offset = word_at(e, base, 0x24).wrapping_add(word_at(e, line, 0x40c));
            e.mem.set_u32(base.wrapping_add(0x24), offset);
        }
        e.call(EXPRESSION_DESTRUCT, &args![expression]);
        true
    })
}

/// The script parameter type table (`0118cdd4`, 8 bytes per type): +0 whether the parameter
/// may be a variable (references allowed when reading it), +1 whether it names an object.
const PARAMETER_TYPES: u32 = 0x0118_cdd4;
/// `TESContainer::ContainerCanHoldType` (engine map, `00481f30`, `cdecl`: a form type).
const CONTAINER_CAN_HOLD_TYPE: u32 = 0x0048_1f30;
/// `TESObjectREFR::HasContainer` (engine map, `0055d310`, `thiscall`).
const REFR_HAS_CONTAINER: u32 = 0x0055_d310;
/// `007af430` (`thiscall` on a reference; engine map name `BGSSaveFormBuffer::GetForm`) and
/// the word at `011ca224` it must equal for a map marker.
const REFR_BASE_FORM: u32 = 0x007a_f430;
const MAP_MARKER_FORM: u32 = 0x011c_a224;
/// `00425fd0` (`thiscall` on a cell): the test an interior cell parameter must pass.
const CELL_TEST: u32 = 0x0042_5fd0;
/// `IsAnInteger` (`004b1230`) and `IsAFloat` (`004b12d0`), engine map names, `cdecl`.
const IS_AN_INTEGER: u32 = 0x004b_1230;
const IS_A_FLOAT: u32 = 0x004b_12d0;
/// `00eca6bd` (`cdecl`, one word: a text; the engine map says `strtol`): the text's integer.
const TEXT_TO_INTEGER: u32 = 0x00ec_a6bd;
/// `atof` (`00eca573`, `cdecl`; the double in `ST0`).
const ATOF: u32 = 0x00ec_a573;
/// `toupper` (`00eca7f4`, `cdecl`).
const TOUPPER: u32 = 0x00ec_a7f4;
/// `0066eb40` (`cdecl`, a name): the actor value of a name, 0x4D for none.
const ACTOR_VALUE_BY_NAME: u32 = 0x0066_eb40;
/// `MiscStatManager::GetIDByName` (engine map, `004d5eb0`, `cdecl`): 0x2B for none.
const MISC_STAT_BY_NAME: u32 = 0x004d_5eb0;
/// The vector constructor iterator (`00401050`, `stdcall`: array, element size, count,
/// constructor).
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
/// The name tables `StandardCompile` searches: animation groups (`011977d8`, 0xF5 entries of
/// 0x24 bytes, the name first), the sexes (`01199e8c` "Male", `01199e90` "Female"), the form
/// type names (`0118a2d8`, 0x57) with their type bytes (`0118a598`), alignments (`01186cf4`,
/// 5), equip types (`011869bc`, 0xE) and critical stages (`0119bbb0`, 5).
const ANIMATION_GROUPS: u32 = 0x0119_77d8;
const SEX_MALE: u32 = 0x0119_9e8c;
const SEX_FEMALE: u32 = 0x0119_9e90;
const FORM_TYPE_NAMES: u32 = 0x0118_a2d8;
const FORM_TYPE_CODES: u32 = 0x0118_a598;
const ALIGNMENT_NAMES: u32 = 0x0118_6cf4;
const EQUIP_TYPE_NAMES: u32 = 0x0118_69bc;
const CRITICAL_STAGE_NAMES: u32 = 0x0119_bbb0;
/// RTTI type descriptors (their names read from the exe): `SpellItem`, `TESObjectBOOK`,
/// `TESObjectCELL`, `MagicItem`, `TESObject`, `TESActorBase`, `TESWorldSpace`, `TESPackage`,
/// `TESCombatStyle`, `EffectSetting`, `BGSListForm`, `BGSMenuIcon`, `BGSPerk`, `BGSNote`,
/// `BGSEncounterZone`, `TESIdleForm`, `BGSMessage`.
const TYPE_SPELL_ITEM: u32 = 0x0118_3060;
const TYPE_TES_OBJECT_BOOK: u32 = 0x0118_a650;
const TYPE_TES_OBJECT_CELL: u32 = 0x0118_3fb4;
const TYPE_MAGIC_ITEM: u32 = 0x0118_3140;
const TYPE_TES_OBJECT: u32 = 0x0118_3128;
const TYPE_TES_ACTOR_BASE: u32 = 0x0118_46e8;
const TYPE_TES_WORLD_SPACE: u32 = 0x0118_3fd0;
const TYPE_TES_PACKAGE: u32 = 0x0118_46a0;
const TYPE_TES_COMBAT_STYLE: u32 = 0x0118_627c;
const TYPE_EFFECT_SETTING: u32 = 0x0118_37c4;
const TYPE_BGS_LIST_FORM: u32 = 0x0118_63d4;
const TYPE_BGS_MENU_ICON: u32 = 0x0118_63f0;
const TYPE_BGS_PERK: u32 = 0x0118_61dc;
const TYPE_BGS_NOTE: u32 = 0x0118_640c;
const TYPE_BGS_ENCOUNTER_ZONE: u32 = 0x0118_40dc;
const TYPE_TES_IDLE_FORM: u32 = 0x0118_6a18;
const TYPE_BGS_MESSAGE: u32 = 0x0118_6130;

/// `StandardCompile`'s and `MessageBoxCompile`'s messages (the formats as the exe holds
/// them).
const TEXT_MISSING_PARAMETER: u32 = 0x0103_9080;
const TEXT_PARAMETER_NOT_VARIABLE: u32 = 0x0103_9040;
const TEXT_ITEM_NOT_FOUND: u32 = 0x0103_8ff8;
const TEXT_REFERENCE_TYPE_UNIMPLEMENTED: u32 = 0x0103_8178;
const TEXT_TYPE_UNIMPLEMENTED: u32 = 0x0103_7dc8;
const TEXT_EXPECTED_END_OF_LINE: u32 = 0x0103_7d90;
const TEXT_UNKNOWN_VARIABLE_FOR_PARAMETER: u32 = 0x0103_8130;
const TEXT_INVALID_ACTOR_VALUE: u32 = 0x0103_80e8;
const TEXT_AXIS_REQUIRED: u32 = 0x0103_7f00;
const TEXT_ANIMATION_GROUP: u32 = 0x0103_7e60;
const TEXT_SEX_REQUIRED: u32 = 0x0103_7eb0;
const TEXT_INVALID_CRIME_TYPE: u32 = 0x0103_7f48;
const TEXT_FORM_TYPE_NOT_FOUND: u32 = 0x0103_7e18;
const TEXT_INVALID_MISC_STAT: u32 = 0x0103_7fc0;
const TEXT_INVALID_ALIGNMENT: u32 = 0x0103_80a0;
const TEXT_INVALID_EQUIP_TYPE: u32 = 0x0103_8008;
const TEXT_INVALID_CRITICAL_STAGE: u32 = 0x0103_8050;
const TEXT_INVALID_INVENTORY_OBJECT: u32 = 0x0103_8fa8;
const TEXT_INVALID_OBJECT: u32 = 0x0103_8f60;
const TEXT_INVALID_OBJECT_REFERENCE: u32 = 0x0103_8f10;
const TEXT_INVALID_CONTAINER_REFERENCE: u32 = 0x0103_8ec0;
const TEXT_INVALID_MAP_MARKER: u32 = 0x0103_8e78;
const TEXT_INVALID_ACTOR: u32 = 0x0103_8e30;
const TEXT_INVALID_SPELL_ITEM: u32 = 0x0103_8de8;
const TEXT_INVALID_INTERIOR_CELL: u32 = 0x0103_8d98;
const TEXT_INVALID_ENCOUNTER_ZONE: u32 = 0x0103_8d48;
const TEXT_INVALID_IDLE_FORM: u32 = 0x0103_8d00;
const TEXT_INVALID_FORM_LIST: u32 = 0x0103_8cb8;
const TEXT_INVALID_MENU_ICON: u32 = 0x0103_8c70;
const TEXT_INVALID_PERK: u32 = 0x0103_8c30;
const TEXT_INVALID_NOTE: u32 = 0x0103_8bf0;
const TEXT_INVALID_WORLDSPACE: u32 = 0x0103_8ba8;
const TEXT_INVALID_MAGIC_ITEM: u32 = 0x0103_8b60;
const TEXT_INVALID_EFFECT_SETTING: u32 = 0x0103_8b10;
const TEXT_INVALID_SOUND: u32 = 0x0103_8ac8;
const TEXT_INVALID_MUSIC_TYPE: u32 = 0x0103_8a80;
const TEXT_INVALID_TOPIC: u32 = 0x0103_8a38;
const TEXT_INVALID_INFO: u32 = 0x0103_89f8;
const TEXT_INVALID_RACE: u32 = 0x0103_89b8;
const TEXT_INVALID_CLASS: u32 = 0x0103_8970;
const TEXT_INVALID_FACTION: u32 = 0x0103_8928;
const TEXT_INVALID_REPUTATION: u32 = 0x0103_88e0;
const TEXT_INVALID_CHALLENGE: u32 = 0x0103_8898;
const TEXT_INVALID_CASINO: u32 = 0x0103_8850;
const TEXT_INVALID_CASINO_CHIP: u32 = 0x0103_8808;
const TEXT_INVALID_CARAVAN_MONEY: u32 = 0x0103_87b8;
const TEXT_INVALID_CARAVAN_CARD: u32 = 0x0103_8770;
const TEXT_INVALID_REGION: u32 = 0x0103_8728;
const TEXT_INVALID_GLOBAL: u32 = 0x0103_86e0;
const TEXT_INVALID_FURNITURE: u32 = 0x0103_8688;
const TEXT_INVALID_ACTOR_BASE: u32 = 0x0103_8640;
const TEXT_INVALID_PACKAGE: u32 = 0x0103_85f8;
const TEXT_INVALID_COMBAT_STYLE: u32 = 0x0103_85b0;
const TEXT_INVALID_WEATHER: u32 = 0x0103_8568;
const TEXT_INVALID_IMAGESPACE_MODIFIER: u32 = 0x0103_8518;
const TEXT_INVALID_IMAGESPACE: u32 = 0x0103_84d0;
const TEXT_INVALID_NPC: u32 = 0x0103_848c;
const TEXT_INVALID_OWNER: u32 = 0x0103_8448;
const TEXT_INVALID_EFFECT_SHADER: u32 = 0x0103_83f8;
const TEXT_INVALID_MESSAGE: u32 = 0x0103_83b0;
const TEXT_INVALID_NPC_OR_LEVELED: u32 = 0x0103_8358;
const TEXT_INVALID_CREATURE_OR_LEVELED: u32 = 0x0103_8300;
const TEXT_INVALID_LEVELED_CHARACTER: u32 = 0x0103_82b0;
const TEXT_INVALID_LEVELED_CREATURE: u32 = 0x0103_8260;
const TEXT_INVALID_LEVELED_ITEM: u32 = 0x0103_8218;
const TEXT_INVALID_FORM: u32 = 0x0103_81d8;
const TEXT_NO_MESSAGE_TEXT: u32 = 0x0103_91dc;
const TEXT_MAX_VARIABLES: u32 = 0x0103_91bc;
const TEXT_TOO_FEW_VARIABLES: u32 = 0x0103_9178;
const TEXT_UNKNOWN_MESSAGE_VARIABLE: u32 = 0x0103_9148;
const TEXT_MESSAGE_VARIABLE_ERROR: u32 = 0x0103_90f8;
const TEXT_MESSAGE_TIME: u32 = 0x0103_90b4;

/// The parameter being compiled and the word read for it, for the checks of
/// `StandardCompile`.
struct Parameter {
    data: Ptr,
    /// The parameter's name (`SCRIPT_PARAMETER` +0).
    name: u32,
    /// The word: its variable id (+0x20C) and form (+0x210) are what the checks read.
    word: u32,
}

impl Parameter {
    fn variable(&self, e: &Engine) -> u32 {
        word_at(e, self.word, 0x20c)
    }

    fn form(&self, e: &Engine) -> u32 {
        word_at(e, self.word, 0x210)
    }

    /// Reports `format` with the word and the parameter's name; false.
    fn fail(&self, e: &mut Engine, format: u32) -> bool {
        script_compiler_script_error(e, self.data, format, &[self.word, self.name]);
        false
    }

    /// Without a variable: the form must exist and have one of `types` (`00401170`, asked
    /// again for each type tried).
    fn check_form_type(&self, e: &mut Engine, types: &[u32], format: u32) -> bool {
        if self.variable(e) != 0 {
            return true;
        }
        let form = self.form(e);
        if form != 0 {
            for kind in types {
                if e.call(FORM_TYPE, &args![form]).u32() == *kind {
                    return true;
                }
            }
        }
        self.fail(e, format)
    }

    /// The form cast from `TESForm` to `target` (`__RTDynamicCast`).
    fn cast(&self, e: &mut Engine, target: u32) -> u32 {
        let form = self.form(e);
        e.call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
        )
        .u32()
    }

    /// Without a variable: the form must exist and cast to `target`.
    fn check_cast(&self, e: &mut Engine, target: u32, format: u32) -> bool {
        if self.variable(e) != 0 {
            return true;
        }
        if self.form(e) != 0 && self.cast(e, target) != 0 {
            return true;
        }
        self.fail(e, format)
    }

    /// The form is cast to `target` first (also with a variable); without a variable the cast
    /// must succeed.
    fn check_cast_first(&self, e: &mut Engine, target: u32, format: u32) -> bool {
        let cast = self.cast(e, target);
        if self.variable(e) == 0 && cast == 0 {
            return self.fail(e, format);
        }
        true
    }
}

/// `StandardCompile`'s checks of an object parameter by type (the exe's jump table at
/// `005b3ac4`, types 3 to 0x45); `None` for a type it has no check for (the log of
/// `0103 8178` follows).
fn check_object_parameter(e: &mut Engine, parameter: &Parameter, kind: u32) -> Option<bool> {
    let p = parameter;
    let ok = match kind {
        3 | 0x32 => {
            if p.variable(e) != 0 {
                return Some(true);
            }
            let form = p.form(e);
            if form != 0 {
                let form_type = e.call(FORM_TYPE, &args![form]).u32();
                if e.call(CONTAINER_CAN_HOLD_TYPE, &args![form_type]).bool() {
                    return Some(true);
                }
                if kind == 0x32 && e.call(FORM_TYPE, &args![form]).u32() == 0x55 {
                    return Some(true);
                }
            }
            p.fail(e, TEXT_INVALID_INVENTORY_OBJECT)
        }
        0x35 => {
            if p.variable(e) != 0 {
                return Some(true);
            }
            let form = p.form(e);
            if form != 0
                && (e.vcall(form, 0xe8, &[]).bool()
                    || e.call(FORM_TYPE, &args![form]).u32() == 0x55)
            {
                return Some(true);
            }
            p.fail(e, TEXT_INVALID_OBJECT)
        }
        4 | 6 | 0x18 | 0x1a => {
            if p.variable(e) != 0 {
                return Some(true);
            }
            let reference = p.cast(e, TYPE_TES_OBJECT_REFR);
            if reference == 0 {
                return Some(p.fail(e, TEXT_INVALID_OBJECT_REFERENCE));
            }
            match kind {
                6 => {
                    if !e.vcall(reference, 0x100, &[]).bool() {
                        return Some(p.fail(e, TEXT_INVALID_ACTOR));
                    }
                    true
                }
                0x18 => {
                    let base = e.call(REFR_BASE_FORM, &args![reference]).u32();
                    if base != e.global::<u32>(MAP_MARKER_FORM) {
                        return Some(p.fail(e, TEXT_INVALID_MAP_MARKER));
                    }
                    true
                }
                0x1a => {
                    if e.call(REFR_HAS_CONTAINER, &args![reference]).u32() == 0 {
                        return Some(p.fail(e, TEXT_INVALID_CONTAINER_REFERENCE));
                    }
                    true
                }
                _ => true,
            }
        }
        7 => {
            if p.variable(e) != 0 {
                return Some(true);
            }
            if p.form(e) != 0
                && (p.cast(e, TYPE_SPELL_ITEM) != 0 || p.cast(e, TYPE_TES_OBJECT_BOOK) != 0)
            {
                return Some(true);
            }
            p.fail(e, TEXT_INVALID_SPELL_ITEM)
        }
        9 => {
            let cell = p.cast(e, TYPE_TES_OBJECT_CELL);
            if p.variable(e) == 0 && (cell == 0 || !e.call(CELL_TEST, &args![cell]).bool()) {
                return Some(p.fail(e, TEXT_INVALID_INTERIOR_CELL));
            }
            true
        }
        0xb => p.check_cast(e, TYPE_MAGIC_ITEM, TEXT_INVALID_MAGIC_ITEM),
        0xc => p.check_form_type(e, &[0xd], TEXT_INVALID_SOUND),
        0xd => p.check_form_type(e, &[0x45], TEXT_INVALID_TOPIC),
        0xe => p.check_form_type(e, &[0x47], TEXT_INVALID_INFO),
        0xf => p.check_form_type(e, &[0xc], TEXT_INVALID_RACE),
        0x10 => p.check_form_type(e, &[7], TEXT_INVALID_CLASS),
        0x11 => p.check_form_type(e, &[8], TEXT_INVALID_FACTION),
        0x13 => p.check_form_type(e, &[6], TEXT_INVALID_GLOBAL),
        0x14 => p.check_form_type(e, &[0x27, 0x55], TEXT_INVALID_FURNITURE),
        0x15 => p.check_cast(e, TYPE_TES_OBJECT, TEXT_INVALID_OBJECT),
        0x19 => p.check_cast(e, TYPE_TES_ACTOR_BASE, TEXT_INVALID_ACTOR_BASE),
        0x1b => p.check_cast(e, TYPE_TES_WORLD_SPACE, TEXT_INVALID_WORLDSPACE),
        0x1d => p.check_cast(e, TYPE_TES_PACKAGE, TEXT_INVALID_PACKAGE),
        0x1e => p.check_cast(e, TYPE_TES_COMBAT_STYLE, TEXT_INVALID_COMBAT_STYLE),
        0x1f => p.check_cast(e, TYPE_EFFECT_SETTING, TEXT_INVALID_EFFECT_SETTING),
        0x21 => p.check_form_type(e, &[0x35], TEXT_INVALID_WEATHER),
        0x22 => p.check_form_type(e, &[0x2a], TEXT_INVALID_NPC),
        0x23 => p.check_form_type(e, &[0x2a, 8], TEXT_INVALID_OWNER),
        0x24 => p.check_form_type(e, &[0x4f], TEXT_INVALID_EFFECT_SHADER),
        0x25 => p.check_cast_first(e, TYPE_BGS_LIST_FORM, TEXT_INVALID_FORM_LIST),
        0x26 => p.check_cast_first(e, TYPE_BGS_MENU_ICON, TEXT_INVALID_MENU_ICON),
        0x27 => p.check_cast_first(e, TYPE_BGS_PERK, TEXT_INVALID_PERK),
        0x28 => p.check_cast_first(e, TYPE_BGS_NOTE, TEXT_INVALID_NOTE),
        0x2a => p.check_form_type(e, &[0x54], TEXT_INVALID_IMAGESPACE_MODIFIER),
        0x2b => p.check_form_type(e, &[0x53], TEXT_INVALID_IMAGESPACE),
        0x2f => p.check_cast_first(e, TYPE_BGS_ENCOUNTER_ZONE, TEXT_INVALID_ENCOUNTER_ZONE),
        0x30 => p.check_cast_first(e, TYPE_TES_IDLE_FORM, TEXT_INVALID_IDLE_FORM),
        0x31 => p.check_form_type(e, &[0x62], TEXT_INVALID_MESSAGE),
        0x36 => p.check_form_type(e, &[0x66], TEXT_INVALID_MUSIC_TYPE),
        0x38 => p.check_form_type(e, &[0x2a, 0x2d], TEXT_INVALID_NPC_OR_LEVELED),
        0x39 => p.check_form_type(e, &[0x2b, 0x2c], TEXT_INVALID_CREATURE_OR_LEVELED),
        0x3a => p.check_form_type(e, &[0x2d], TEXT_INVALID_LEVELED_CHARACTER),
        0x3b => p.check_form_type(e, &[0x2c], TEXT_INVALID_LEVELED_CREATURE),
        0x3c => p.check_form_type(e, &[0x34], TEXT_INVALID_LEVELED_ITEM),
        0x3d => {
            if p.variable(e) == 0 && p.form(e) == 0 {
                return Some(p.fail(e, TEXT_INVALID_FORM));
            }
            true
        }
        0x3e => p.check_form_type(e, &[0x68], TEXT_INVALID_REPUTATION),
        0x3f => p.check_form_type(e, &[0x6d], TEXT_INVALID_CASINO),
        0x40 => p.check_form_type(e, &[0x6c], TEXT_INVALID_CASINO_CHIP),
        0x41 => p.check_form_type(e, &[0x71], TEXT_INVALID_CHALLENGE),
        0x42 => p.check_form_type(e, &[0x74], TEXT_INVALID_CARAVAN_MONEY),
        0x43 => p.check_form_type(e, &[0x73], TEXT_INVALID_CARAVAN_CARD),
        0x44 => p.check_form_type(e, &[0x75], TEXT_INVALID_CARAVAN_CARD),
        0x45 => p.check_form_type(e, &[0x37], TEXT_INVALID_REGION),
        _ => return None,
    };
    Some(ok)
}

/// The index (0 to `count - 1`, a 16-bit counter) of the first name of `table` (one pointer
/// each) that `_stricmp(name, word)` matches, else `count`.
fn index_in_names(e: &mut Engine, table: u32, count: i16, word: u32) -> i16 {
    let mut index = 0i16;
    while index < count {
        let name = e
            .mem
            .u32(table.wrapping_add((index as u32).wrapping_mul(4)));
        if e.call(STRICMP, &args![name, word]).u32() == 0 {
            break;
        }
        index += 1;
    }
    index
}

/// `StandardCompile`'s handling of a parameter that is not an object (the exe's jump table at
/// `005b3bd0` with the byte map at `005b3c04`); `None` for a type it does not handle (the log
/// of `0103 7dc8` follows).
fn compile_value_parameter(
    e: &mut Engine,
    parameter: &Parameter,
    kind: u32,
    line: u32,
    length: u32,
) -> Option<bool> {
    let p = parameter;
    let word = p.word;
    match kind {
        0 => {
            line_put_u16(e, line, length as u16);
            line_put_bytes(e, line, word, length);
        }
        1 | 2 | 0x17 => {
            if word_at(e, word, 0x200) != 0 {
                let prefix = if e.mem.i8(word + 0x204) == 0x47 {
                    0x47
                } else {
                    0x72
                };
                line_put_u8(e, line, prefix);
                let index = e.mem.u16(word + 0x200);
                line_put_u16(e, line, index);
            }
            if e.mem.i8(word + 0x204) == 0x47 {
                return Some(true);
            }
            if word_at(e, word, 0x20c) != 0 {
                let kind_byte = e.mem.u8(word + 0x204);
                line_put_u8(e, line, kind_byte);
                let id = e.mem.u16(word + 0x20c);
                line_put_u16(e, line, id);
            } else if kind == 1 || kind == 0x17 {
                if e.call(IS_AN_INTEGER, &args![word]).u32() == 0 {
                    return Some(p.fail(e, TEXT_UNKNOWN_VARIABLE_FOR_PARAMETER));
                }
                line_put_u8(e, line, 0x6e);
                let value = e.call(TEXT_TO_INTEGER, &args![word]).u32();
                let at = line
                    .wrapping_add(word_at(e, line, 0x40c))
                    .wrapping_add(0x20c);
                e.mem.set_u32(at, value);
                let size = word_at(e, line, 0x40c).wrapping_add(4);
                e.mem.set_u32(line.wrapping_add(0x40c), size);
            } else {
                if e.call(IS_A_FLOAT, &args![word]).u32() == 0 {
                    return Some(p.fail(e, TEXT_UNKNOWN_VARIABLE_FOR_PARAMETER));
                }
                line_put_u8(e, line, 0x7a);
                let value = e.call(ATOF, &args![word]).f64();
                let at = line
                    .wrapping_add(word_at(e, line, 0x40c))
                    .wrapping_add(0x20c);
                e.mem.set_f64(at, value);
                let size = word_at(e, line, 0x40c).wrapping_add(8);
                e.mem.set_u32(line.wrapping_add(0x40c), size);
            }
        }
        5 => {
            let value = e.call(ACTOR_VALUE_BY_NAME, &args![word]).u32();
            if value == 0x4d {
                return Some(p.fail(e, TEXT_INVALID_ACTOR_VALUE));
            }
            line_put_u16(e, line, value as u16);
        }
        8 => {
            let first = e.mem.i8(word);
            let axis = e.call(TOUPPER, &args![first as i32]).u8();
            if axis != 0x58 && axis != 0x59 && axis != 0x5a {
                script_compiler_script_error(e, p.data, TEXT_AXIS_REQUIRED, &[p.name]);
                return Some(false);
            }
            line_put_u8(e, line, axis);
        }
        0xa => {
            let mut found = 0xffu32;
            let mut index = 0i32;
            while index < 0xf5 {
                let name = e
                    .mem
                    .u32(ANIMATION_GROUPS.wrapping_add((index as u32).wrapping_mul(0x24)));
                if e.call(STRICMP, &args![word, name]).u32() == 0 {
                    found = index as u32;
                    break;
                }
                index += 1;
            }
            if found == 0xff {
                return Some(p.fail(e, TEXT_ANIMATION_GROUP));
            }
            line_put_u16(e, line, found as u16);
        }
        0x12 => {
            let mut sex = 2u32;
            let male = e.mem.u32(SEX_MALE);
            if e.call(STRICMP, &args![word, male]).u32() == 0 {
                sex = 0;
            } else {
                let female = e.mem.u32(SEX_FEMALE);
                if e.call(STRICMP, &args![word, female]).u32() == 0 {
                    sex = 1;
                }
            }
            if sex == 2 {
                script_compiler_script_error(e, p.data, TEXT_SEX_REQUIRED, &[p.name]);
                return Some(false);
            }
            line_put_u16(e, line, sex as u16);
        }
        0x1c => {
            let crime_error = |e: &mut Engine| {
                script_compiler_script_error(
                    e,
                    p.data,
                    TEXT_INVALID_CRIME_TYPE,
                    &[word, p.name, 4],
                );
                Some(false)
            };
            if e.call(IS_AN_INTEGER, &args![word]).u32() == 0 {
                return crime_error(e);
            }
            let crime = e.call(TEXT_TO_INTEGER, &args![word]).i32();
            if !(0..5).contains(&crime) {
                return crime_error(e);
            }
            line_put_u16(e, line, crime as u16);
        }
        0x20 => {
            let mut code = 0u8;
            let mut index = 0i32;
            while index < 0x57 {
                let name = e
                    .mem
                    .u32(FORM_TYPE_NAMES.wrapping_add((index as u32).wrapping_mul(4)));
                if e.call(STRICMP, &args![word, name]).u32() == 0 {
                    code = e.mem.u8(FORM_TYPE_CODES.wrapping_add(index as u32));
                    break;
                }
                index += 1;
            }
            if code == 0 {
                return Some(p.fail(e, TEXT_FORM_TYPE_NOT_FOUND));
            }
            line_put_u16(e, line, u16::from(code));
        }
        0x29 => {
            let stat = e.call(MISC_STAT_BY_NAME, &args![word]).u32();
            if stat == 0x2b {
                return Some(p.fail(e, TEXT_INVALID_MISC_STAT));
            }
            line_put_u16(e, line, stat as u16);
        }
        0x33 | 0x34 | 0x37 => {
            let (table, count, message) = match kind {
                0x33 => (ALIGNMENT_NAMES, 5, TEXT_INVALID_ALIGNMENT),
                0x34 => (EQUIP_TYPE_NAMES, 0xe, TEXT_INVALID_EQUIP_TYPE),
                _ => (CRITICAL_STAGE_NAMES, 5, TEXT_INVALID_CRITICAL_STAGE),
            };
            let index = index_in_names(e, table, count, word);
            if index == count {
                return Some(p.fail(e, message));
            }
            line_put_u16(e, line, index as u16);
        }
        _ => return None,
    }
    Some(true)
}

// Translated from 005b1ba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005b1ba0 - the /GS stack cookie check (00ec408c) is not translated.
/// `ScriptCompiler::StandardCompile` (Xbox PDB), `cdecl` (parameter count, the parameters
/// (`SCRIPT_PARAMETER`, 0xC bytes: name, type, optional), the line, the compile data): reads
/// the line's parameters into its output after a 16-bit count. Each parameter's word is read
/// (`GetNextWord`, references allowed when its type (table `0118cdd4`) may be a variable); a
/// missing optional parameter sets the count to the parameters read so far and ends (true),
/// a missing required one is reported. A word naming a variable where none is allowed is
/// reported. An object parameter must be a referenced object (`GetReferencedObject`) and pass
/// its type's check (`check_object_parameter`), then is written as `'r'` and its index; other
/// parameters are written by `compile_value_parameter`. An unhandled type is logged
/// (`005b5e40`) and fails. At the end, text left on the line is reported ("Expected end of
/// line", with the name one past the last parameter, as the exe reads it). The stack cookie
/// check is not translated.
pub fn script_compiler_standard_compile(
    e: &mut Engine,
    count: u16,
    parameters: u32,
    line: u32,
    data: Ptr,
) -> bool {
    e.mem.set_u32(line.wrapping_add(0x208), 0);
    let count_at = line
        .wrapping_add(word_at(e, line, 0x40c))
        .wrapping_add(0x20c);
    e.mem.set_u16(count_at, count);
    let size = word_at(e, line, 0x40c).wrapping_add(2);
    e.mem.set_u32(line.wrapping_add(0x40c), size);
    e.with_stack(0x214, |e, word| {
        let word = word.addr();
        let mut index = 0i16;
        loop {
            let entry = parameters.wrapping_add((index as i32 as u32).wrapping_mul(0xc));
            if i32::from(index) >= i32::from(count) {
                if word_at(e, line, 0x208) < word_at(e, line, 0x204) {
                    let name = e.mem.u32(entry);
                    script_compiler_script_error(e, data, TEXT_EXPECTED_END_OF_LINE, &[name]);
                    return false;
                }
                return true;
            }
            let kind = word_at(e, entry, 4);
            let allow = e.mem.u8(PARAMETER_TYPES.wrapping_add(kind.wrapping_mul(8)));
            fn_005af310(e, Ptr::new(word));
            let length = script_compiler_get_next_word(
                e,
                data,
                word,
                line.wrapping_add(4),
                line.wrapping_add(0x208),
                allow,
                0,
            );
            if length == 0 {
                if e.mem.u8(entry.wrapping_add(8)) == 0 {
                    let name = e.mem.u32(entry);
                    script_compiler_script_error(e, data, TEXT_MISSING_PARAMETER, &[name]);
                    return false;
                }
                e.mem.set_u16(count_at, index as u16);
                return true;
            }
            if allow == 0 && (word_at(e, word, 0x200) != 0 || e.mem.i8(word + 0x204) != 0) {
                let name = e.mem.u32(entry);
                script_compiler_script_error(e, data, TEXT_PARAMETER_NOT_VARIABLE, &[name]);
                return false;
            }
            let object_at = PARAMETER_TYPES
                .wrapping_add(kind.wrapping_mul(8))
                .wrapping_add(1);
            let parameter = Parameter {
                data,
                name: e.mem.u32(entry),
                word,
            };
            if e.mem.u8(object_at) != 0
                && (!script_compile_data_get_referenced_object(e, data, word, 0, 0)
                    || word_at(e, word, 0x200) == 0)
            {
                return parameter.fail(e, TEXT_ITEM_NOT_FOUND);
            }
            let kind = word_at(e, entry, 4);
            if e.mem.u8(object_at) != 0 {
                match check_object_parameter(e, &parameter, kind) {
                    Some(true) => {}
                    Some(false) => return false,
                    None => {
                        e.call(SCRIPT_LOG, &args![TEXT_REFERENCE_TYPE_UNIMPLEMENTED, kind]);
                        return false;
                    }
                }
                line_put_u8(e, line, 0x72);
                let index = e.mem.u16(word + 0x200);
                line_put_u16(e, line, index);
            } else {
                match compile_value_parameter(e, &parameter, kind, line, length) {
                    Some(true) => {}
                    Some(false) => return false,
                    None => {
                        e.call(SCRIPT_LOG, &args![TEXT_TYPE_UNIMPLEMENTED, kind]);
                        return false;
                    }
                }
            }
            index = index.wrapping_add(1);
        }
    })
}

/// Writes a word's variable for a message box: a reference prefix (`'G'` for a global, else
/// `'r'`, and the 16-bit index), then, unless it is a global, its kind and 16-bit id (an id of
/// 0 is reported); false on an error.
fn put_message_variable(e: &mut Engine, data: Ptr, line: u32, word: u32) -> bool {
    if word_at(e, word, 0x200) != 0 {
        let prefix = if e.mem.i8(word + 0x204) == 0x47 {
            0x47
        } else {
            0x72
        };
        line_put_u8(e, line, prefix);
        let index = e.mem.u16(word + 0x200);
        line_put_u16(e, line, index);
    }
    if e.mem.i8(word + 0x204) == 0x47 {
        return true;
    }
    if word_at(e, word, 0x20c) == 0 {
        script_compiler_script_error(e, data, TEXT_MESSAGE_VARIABLE_ERROR, &[word]);
        return false;
    }
    let kind = e.mem.u8(word + 0x204);
    line_put_u8(e, line, kind);
    let id = e.mem.u16(word + 0x20c);
    line_put_u16(e, line, id);
    true
}

// Translated from 005b3cd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005b3cd0 - the /GS stack cookie check (00ec408c) is not translated.
/// `ScriptCompiler::MessageBoxCompile` (Xbox PDB), `cdecl` (parameter count (not read), the
/// parameters, the line, the compile data, buttons flag, text flag): writes a 16-bit 1, then
/// with the text flag the message text (16-bit length and bytes; none is reported), without
/// it a `BGSMessage` object parameter (as `StandardCompile` checks one: a variable is refused,
/// it must be a referenced object, and a message unless it is a variable; `'r'` and its index).
/// The `%` conversions of the text (the text itself, or the message's description from its
/// component at +0x24, slot 0x10 with 0 and `DESC`; `%%` does not count) give the variable
/// count, below 10 (else reported), written as 16 bits and followed by each variable
/// (references allowed). Without the text flag, then either (buttons flag) up to 10 button
/// words, their count and each as a 16-bit 1, its length and its text; or a display time (an
/// integer word, 0 without one) as 32 bits. Answers true, false on an error. The stack probe
/// `__alloca_probe` (`00ec8660`) of the frame is left out (the frame is a stack block of the
/// engine); the stack cookie check is not translated.
pub fn script_compiler_message_box_compile(
    e: &mut Engine,
    _unused_1: u32,
    parameters: u32,
    line: u32,
    data: Ptr,
    buttons: u8,
    text_only: u8,
) -> bool {
    e.mem.set_u32(line.wrapping_add(0x208), 0);
    line_put_u16(e, line, 1);
    let text = line.wrapping_add(4);
    let offset = line.wrapping_add(0x208);
    // Locals: +0 the text word, +0x214 the message word, +0x428 a variable word, +0x63c ten
    // button words (0x14c8), +0x1b04 the time word.
    e.with_stack(0x1d18, |e, frame| {
        let text_word = frame.addr();
        let message_word = text_word + 0x214;
        let variable_word = text_word + 0x428;
        let button_words = text_word + 0x63c;
        let time_word = text_word + 0x1b04;
        fn_005af310(e, Ptr::new(text_word));
        let mut message = 0u32;
        if text_only != 0 {
            let length = script_compiler_get_next_word(e, data, text_word, text, offset, 0, 0);
            if length == 0 {
                report(e, data, TEXT_NO_MESSAGE_TEXT);
                return false;
            }
            line_put_u16(e, line, length as u16);
            line_put_bytes(e, line, text_word, length);
        } else {
            fn_005af310(e, Ptr::new(message_word));
            let length = script_compiler_get_next_word(e, data, message_word, text, offset, 0, 0);
            let name = e.mem.u32(parameters);
            if length == 0 {
                script_compiler_script_error(e, data, TEXT_MISSING_PARAMETER, &[name]);
                return false;
            }
            if word_at(e, message_word, 0x200) != 0 || e.mem.i8(message_word + 0x204) != 0 {
                script_compiler_script_error(e, data, TEXT_PARAMETER_NOT_VARIABLE, &[name]);
                return false;
            }
            if !script_compile_data_get_referenced_object(e, data, message_word, 0, 0)
                || word_at(e, message_word, 0x200) == 0
            {
                let name = e.mem.u32(parameters);
                script_compiler_script_error(e, data, TEXT_ITEM_NOT_FOUND, &[message_word, name]);
                return false;
            }
            let form = word_at(e, message_word, 0x210);
            message = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_BGS_MESSAGE, 0u32],
                )
                .u32();
            if word_at(e, message_word, 0x20c) == 0 && message == 0 {
                let name = e.mem.u32(parameters);
                script_compiler_script_error(e, data, TEXT_INVALID_MESSAGE, &[message_word, name]);
                return false;
            }
            line_put_u8(e, line, 0x72);
            let index = e.mem.u16(message_word + 0x200);
            line_put_u16(e, line, index);
        }
        let mut at = if text_only != 0 {
            text_word
        } else if message != 0 {
            e.vcall(
                message.wrapping_add(0x24),
                0x10,
                &args![0u32, 0x4353_4544u32],
            )
            .u32()
        } else {
            0
        };
        let mut conversions = 0i32;
        while at != 0 && e.mem.i8(at) != 0 {
            if e.mem.i8(at) == 0x25 {
                if e.mem.i8(at.wrapping_add(1)) == 0x25 {
                    at = at.wrapping_add(1);
                } else {
                    conversions += 1;
                }
            }
            at = at.wrapping_add(1);
        }
        if conversions >= 10 {
            script_compiler_script_error(e, data, TEXT_MAX_VARIABLES, &[10]);
            return false;
        }
        line_put_u16(e, line, conversions as u16);
        let mut index = 0i32;
        while index < conversions {
            fn_005af310(e, Ptr::new(variable_word));
            if script_compiler_get_next_word(e, data, variable_word, text, offset, 1, 0) == 0 {
                script_compiler_script_error(
                    e,
                    data,
                    TEXT_TOO_FEW_VARIABLES,
                    &[conversions as u32, index as u32],
                );
                return false;
            }
            if e.mem.i8(variable_word + 0x204) == 0 {
                script_compiler_script_error(
                    e,
                    data,
                    TEXT_UNKNOWN_MESSAGE_VARIABLE,
                    &[variable_word],
                );
                return false;
            }
            if !put_message_variable(e, data, line, variable_word) {
                return false;
            }
            index += 1;
        }
        if text_only != 0 {
            return true;
        }
        if buttons != 0 {
            e.call(
                VECTOR_CONSTRUCTOR,
                &args![button_words, 0x214u32, 0xau32, 0x005a_f310u32],
            );
            let mut read = 0i32;
            loop {
                let button = button_words.wrapping_add((read as u32).wrapping_mul(0x214));
                let length = script_compiler_get_next_word(e, data, button, text, offset, 0, 0);
                if length as i32 > 0 {
                    read += 1;
                }
                if length as i32 <= 0 || read >= 10 {
                    break;
                }
            }
            line_put_u16(e, line, read as u16);
            let mut button = 0i32;
            while button < read {
                line_put_u16(e, line, 1);
                let at = button_words.wrapping_add((button as u32).wrapping_mul(0x214));
                let length = e.call(TEXT_LENGTH, &args![at]).u32();
                line_put_u16(e, line, length as u16);
                line_put_bytes(e, line, at, length);
                button += 1;
            }
        } else {
            let mut time = 0u32;
            fn_005af310(e, Ptr::new(time_word));
            let length = script_compiler_get_next_word(e, data, time_word, text, offset, 0, 0);
            if length as i32 > 0 {
                if e.call(IS_AN_INTEGER, &args![time_word]).u32() == 0 {
                    report(e, data, TEXT_MESSAGE_TIME);
                    return false;
                }
                time = e.call(TEXT_TO_INTEGER, &args![time_word]).u32();
            }
            let at = line
                .wrapping_add(word_at(e, line, 0x40c))
                .wrapping_add(0x20c);
            e.mem.set_u32(at, time);
            let size = word_at(e, line, 0x40c).wrapping_add(4);
            e.mem.set_u32(line.wrapping_add(0x40c), size);
        }
        true
    })
}

// Translated from 005b3c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, a compile function (`cdecl`: parameter count, parameters, line,
/// compile data): `MessageBoxCompile` with buttons (flags 1, 0); its `AL` passes through.
pub fn fn_005b3c40(e: &mut Engine, count: u16, parameters: u32, line: u32, data: Ptr) -> bool {
    script_compiler_message_box_compile(e, u32::from(count), parameters, line, data, 1, 0)
}

// Translated from 005b3c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, a compile function: `MessageBoxCompile` with a display time (flags 0,
/// 0).
pub fn fn_005b3c70(e: &mut Engine, count: u16, parameters: u32, line: u32, data: Ptr) -> bool {
    script_compiler_message_box_compile(e, u32::from(count), parameters, line, data, 0, 0)
}

// Translated from 005b3ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, a compile function: `MessageBoxCompile` with a message text (flags 0,
/// 1).
pub fn fn_005b3ca0(e: &mut Engine, count: u16, parameters: u32, line: u32, data: Ptr) -> bool {
    script_compiler_message_box_compile(e, u32::from(count), parameters, line, data, 0, 1)
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x005b1120,
            script_compiler_get_function_def(i32) -> Ptr
        ),
        entry!(0x005aea50, script_compiler_script_compiler(Ptr) -> Ptr),
        entry!(0x005a_e490, script_compile_data_script_compile_data(Ptr) -> Ptr),
        entry!(0x005a_e5c0, script_compile_data_destructor(Ptr)),
        entry!(0x005a_e670, fn_005ae670(Ptr)),
        entry!(0x005a_e6c0, fn_005ae6c0(Ptr)),
        entry!(0x005a_e730, fn_005ae730(Ptr)),
        entry!(0x005a_e7a0, script_compile_data_get_variable(Ptr, u32) -> bool),
        (0x005a_ea90, script_compiler_script_error_entry as AbiFn),
        entry!(0x005a_eb90, script_compiler_compile(Ptr, u32, Ptr) -> bool),
        entry!(
            0x005a_edb0,
            script_compiler_compile_partial_script(Ptr, u32, u32, u32, u32) -> bool
        ),
        entry!(0x005a_eed0, script_compiler_get_next_expression(Ptr, Ptr) -> u32),
        entry!(0x005a_f310, fn_005af310(Ptr) -> Ptr),
        entry!(0x005a_f370, script_line_script_line(Ptr) -> Ptr),
        entry!(0x005a_f3f0, script_compiler_get_next_line(Ptr, Ptr) -> u32),
        entry!(
            0x005a_f5f0,
            script_compiler_get_next_word(Ptr, u32, u32, u32, u8, u8) -> u32
        ),
        entry!(
            0x005a_fa50,
            script_compile_data_verify_referenced_object_list(Ptr) -> bool
        ),
        entry!(
            0x005a_fad0,
            script_compile_data_get_referenced_object(Ptr, u32, u8, u8) -> bool
        ),
        entry!(0x005a_fe10, fn_005afe10(u32, u32, u32) -> u32),
        entry!(
            0x005a_fe30,
            script_compile_data_get_referenced_object_ov2(Ptr, u32) -> u32
        ),
        entry!(0x005a_feb0, script_compiler_is_valid_variable_name(u32) -> bool),
        entry!(
            0x005a_ff40,
            script_compiler_compile_line(Ptr, u32, Ptr, u32) -> bool
        ),
        entry!(0x005b_1180, script_compiler_verify_blocks(Ptr, Ptr) -> bool),
        entry!(0x005b_12c0, script_compiler_count_block_lines(Ptr, Ptr, u32) -> i32),
        entry!(0x005b_13e0, script_compiler_count_if_lines(Ptr, Ptr, u32) -> i32),
        entry!(
            0x005b_1570,
            script_compiler_replace_variables_and_functions(Ptr, Ptr, u32, u32, u32) -> u32
        ),
        entry!(0x005b_1910, script_compiler_get_function(u32) -> bool),
        entry!(0x005b_1a90, fn_005b1a90(u32, u32) -> bool),
        entry!(0x005b_1b00, fn_005b1b00(u32, u32) -> bool),
        entry!(0x005b_1b60, fn_005b1b60(u32, u32)),
        entry!(
            0x005b_1ba0,
            script_compiler_standard_compile(u16, u32, u32, Ptr) -> bool
        ),
        entry!(0x005b_3c40, fn_005b3c40(u16, u32, u32, Ptr) -> bool),
        entry!(0x005b_3c70, fn_005b3c70(u16, u32, u32, Ptr) -> bool),
        entry!(0x005b_3ca0, fn_005b3ca0(u16, u32, u32, Ptr) -> bool),
        entry!(
            0x005b_3cd0,
            script_compiler_message_box_compile(u32, u32, u32, Ptr, u8, u8) -> bool
        ),
    ]
}

#[cfg(test)]
mod constructor_tests {
    use super::*;

    #[test]
    fn reports_tables_without_their_end_markers() {
        let mut e = Engine::new();
        e.register(ERROR, |_, _| Ret::default());
        e.map(0x0119_0000, 0x7000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005a_ea50, &args![0x40u32]).u32(), 0x40);
        let errors: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == ERROR)
            .map(|c| c.1)
            .collect();
        assert_eq!(
            errors,
            vec![vec![SCRIPT_LIST_INCOMPLETE], vec![CONSOLE_LIST_INCOMPLETE]]
        );
        e.mem.set_u32(SCRIPT_END_OPCODE, 0x1280);
        e.mem.set_u32(CONSOLE_END_OPCODE, 0x1cd);
        e.call_log = Some(vec![]);
        e.call(0x005a_ea50, &args![0x40u32]);
        assert!(!e.call_log.take().unwrap().iter().any(|c| c.0 == ERROR));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_both_opcode_ranges_and_rejects_the_rest() {
        let mut e = Engine::new();
        let cases: &[(i32, u32)] = &[
            (0xff, 0),
            (0x100, 0x0118_e8e0),
            (0x101, 0x0118_e908),
            (0x1cc, 0x0118_e8e0 + 0xcc * 0x28),
            (0x1cd, 0),
            (0xfff, 0),
            (0x1000, 0x0119_0910),
            (0x1059, 0x0119_0910 + 0x59 * 0x28),
            (0x127f, 0x0119_0910 + 0x27f * 0x28),
            (0x1280, 0),
            (-1, 0),
            (i32::MIN, 0),
            (0x10100, 0),
        ];
        for &(opcode, expected) in cases {
            assert_eq!(
                e.call(0x005b1120, &args![opcode]).u32(),
                expected,
                "{opcode:x}"
            );
        }
    }
}

#[cfg(test)]
mod n045_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn lower(text: &[u8]) -> Vec<u8> {
        text.iter().map(|c| c.to_ascii_lowercase()).collect()
    }

    /// The formats `ScriptError` was given (from its `vsprintf` call).
    fn errors(e: &Engine) -> Vec<u32> {
        only(e.call_log.as_ref().unwrap(), VSPRINTF)
            .into_iter()
            .map(|w| w[1])
            .collect()
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let at = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(at, s);
        at
    }

    /// Doubles for the list, string, C runtime and log helpers the compiler calls.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x0118_0000, 0x2_0000);
        e.map(0x011c_0000, 0x2_0000);
        e.map(0x0101_0000, 0x1_0000);
        e.mem.set_cstr(TEXT_PLAYER, b"Player");
        e.register(LIST_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_IS_EMPTY, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        e.register(LIST_POP_FIRST, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let (item, after) = (e.mem.u32(next), e.mem.u32(next + 4));
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            Ret::default()
        });
        e.register(LIST_ADD_TAIL, |e, a| {
            let item = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0 {
                e.mem.set_u32(a[0], item);
                return Ret::default();
            }
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let new = e.mem.alloc(8);
            e.mem.set_u32(new, item);
            e.mem.set_u32(node + 4, new);
            Ret::default()
        });
        e.register(LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(VARIABLE_LIST_OF, |_, a| ret(a[0] + 0x3c));
        e.register(REF_OBJECT_LIST_OF, |_, a| ret(a[0] + 0x44));
        e.register(LINE_LIST_OF, |_, a| ret(a[0] + 0x50));
        e.register(STRING_TEXT, |e, a| ret(e.mem.u32(a[0])));
        e.register(STRING_SET, |e, a| {
            let s = e.mem.cstr(a[1]);
            let copy = e.mem.alloc(s.len() as u32 + 1);
            e.mem.set_cstr(copy, &s);
            e.mem.set_u32(a[0], copy);
            Ret::default()
        });
        e.register(STRING_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(STRING_COMPARE, |e, a| {
            let mine = e.mem.cstr(e.mem.u32(a[0]));
            ret(u32::from(lower(&mine) != lower(&e.mem.cstr(a[1]))))
        });
        e.register(STRICMP, |e, a| {
            if a[0] == 0 || a[1] == 0 {
                return ret(1);
            }
            ret(u32::from(
                lower(&e.mem.cstr(a[0])) != lower(&e.mem.cstr(a[1])),
            ))
        });
        e.register(STRNICMP, |e, a| {
            let x = lower(&e.mem.cstr(a[0]));
            let y = lower(&e.mem.cstr(a[1]));
            let n = a[2] as usize;
            let cut = |v: &[u8]| v.iter().take(n).copied().collect::<Vec<u8>>();
            ret(u32::from(cut(&x) != cut(&y)))
        });
        e.register(ISALNUM, |_, a| {
            let c = a[0] as i32;
            ret(u32::from(
                (0..256).contains(&c) && (c as u8).is_ascii_alphanumeric(),
            ))
        });
        e.register(ISALPHA, |_, a| {
            let c = a[0] as i32;
            ret(u32::from(
                (0..256).contains(&c) && (c as u8).is_ascii_alphabetic(),
            ))
        });
        e.register(TOLOWER, |_, a| {
            ret(u32::from((a[0] as u8).to_ascii_lowercase()))
        });
        e.register(TOUPPER, |_, a| {
            ret(u32::from((a[0] as u8).to_ascii_uppercase()))
        });
        e.register(STRCHR, |e, a| {
            let s = e.mem.cstr(a[0]);
            match s.iter().position(|&c| u32::from(c) == a[1]) {
                Some(i) => ret(a[0] + i as u32),
                None => ret(0),
            }
        });
        e.register(STRCPY_S, |e, a| {
            let s = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &s);
            Ret::default()
        });
        e.register(TEXT_LENGTH, |e, a| ret(e.mem.cstr(a[0]).len() as u32));
        e.register(STRLEN, |e, a| ret(e.mem.cstr(a[0]).len() as u32));
        e.register(MEMSET, |e, a| {
            let bytes = vec![a[1] as u8; a[2] as usize];
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(CALLOCATE, |e, a| ret(e.mem.alloc(a[0] * a[1])));
        for addr in [
            OPERATOR_DELETE,
            STRING_CONSTRUCT,
            STRING_DESTRUCT,
            LIST_DESTRUCT,
            VSPRINTF,
            STRING_PRINTF,
            CONSOLE_PRINT,
            SCRIPT_LOG,
            REF_OBJECT_DELETE,
            VARIABLE_DELETE,
            EXPRESSION_CONSTRUCT,
            EXPRESSION_DESTRUCT,
        ] {
            e.register(addr, ok);
        }
        e.register(PREPARE_MESSAGE, |_, a| ret(a[0]));
        e.register(FORM_BY_EDITOR_ID, |_, _| ret(0));
        e.register(LOOKUP_FORM, |_, _| ret(0));
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        e.register(FORM_TYPE, |e, a| ret(u32::from(e.mem.u8(a[0] + 4))));
        e.register(GET_GLOBAL, |_, _| ret(0));
        e.register(STRTOUL, |e, a| {
            let s = e.mem.cstr(a[0]);
            let digits: Vec<u8> = s
                .iter()
                .take_while(|c| c.is_ascii_hexdigit())
                .copied()
                .collect();
            e.mem.set_u32(a[1], a[0] + digits.len() as u32);
            ret(u32::from_str_radix(std::str::from_utf8(&digits).unwrap_or("0"), 16).unwrap_or(0))
        });
        e.register(REF_OBJECT_CONSTRUCT, |e, a| {
            e.mem.write(a[0], &[0; 0x10]);
            ret(a[0])
        });
        e
    }

    /// A compile data made by its own constructor.
    fn data(e: &mut Engine) -> u32 {
        let block = e.mem.alloc(0x58);
        e.call(0x005a_e490, &[block]).u32()
    }

    /// A `SCRIPT_LINE` holding `text`, numbered `number`, with `opcode`.
    fn line(e: &mut Engine, number: u32, opcode: u32, text: &[u8]) -> u32 {
        let block = e.mem.alloc(0x41c);
        e.call(0x005a_f370, &[block]);
        e.mem.set_u32(block, number);
        e.mem.set_cstr(block + 4, text);
        e.mem.set_u32(block + 0x204, text.len() as u32);
        e.mem.set_u32(block + 0x410, opcode);
        block
    }

    fn push(e: &mut Engine, list: u32, item: u32) {
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, item);
        e.call(LIST_ADD_TAIL, &[list, slot]);
    }

    /// A script variable named `name` with id `id` and its byte at +0x10.
    fn variable(e: &mut Engine, name: &[u8], id: u32, flag: u8) -> u32 {
        let v = e.mem.alloc(0x20);
        e.mem.set_u32(v, id);
        e.mem.set_u8(v + 0x10, flag);
        let t = text(e, name);
        e.mem.set_u32(v + 0x18, t);
        v
    }

    /// A word holding `text`.
    fn word(e: &mut Engine, s: &[u8]) -> u32 {
        let w = e.mem.alloc(0x214);
        e.call(0x005a_f310, &[w]);
        e.mem.set_cstr(w, s);
        w
    }

    /// Puts a command at index 0 of the script command table (`01190910`).
    fn command(e: &mut Engine, name: &[u8], opcode: u32) {
        let t = text(e, name);
        e.mem.set_u32(SCRIPT_COMMANDS, t);
        e.mem.set_u32(SCRIPT_COMMANDS + 8, opcode);
    }

    #[test]
    fn construct_and_destroy_the_compile_data() {
        let mut e = rig();
        e.call_log = Some(vec![]);
        let d = data(&mut e);
        let output = e.mem.u32(d + 0x20);
        assert_ne!(output, 0);
        assert_eq!(e.mem.u32(d + 0x14), 0);
        assert_eq!(e.mem.u8(d + 0x18), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, CALLOCATE), vec![vec![1, 0x4000]]);
        assert_eq!(only(&log, STRING_SET), vec![vec![d + 0xc, 0x0101_1584, 0]]);
        let l = line(&mut e, 1, 0x10, b"x");
        push(&mut e, d + 0x50, l);
        let r = e.mem.alloc(0x10);
        push(&mut e, d + 0x44, r);
        push(&mut e, d + 0x44, 0);
        let v = e.mem.alloc(0x20);
        push(&mut e, d + 0x3c, v);
        e.call_log = Some(vec![]);
        e.call(0x005a_e5c0, &[d]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, OPERATOR_DELETE), vec![vec![l], vec![output]]);
        assert_eq!(only(&log, REF_OBJECT_DELETE), vec![vec![r, 1]]);
        assert_eq!(only(&log, VARIABLE_DELETE), vec![vec![v, 1]]);
        assert_eq!(
            only(&log, LIST_DESTRUCT),
            vec![vec![d + 0x50], vec![d + 0x44], vec![d + 0x3c]]
        );
        assert_eq!(only(&log, STRING_DESTRUCT), vec![vec![d + 0xc]]);
        assert_eq!(e.mem.u32(d + 0x50), 0);
    }

    #[test]
    fn get_variable_searches_variables_script_and_globals() {
        let mut e = rig();
        let d = data(&mut e);
        let speed = variable(&mut e, b"speed", 7, 1);
        let rate = variable(&mut e, b"rate", 9, 0);
        push(&mut e, d + 0x3c, speed);
        push(&mut e, d + 0x3c, rate);
        let w = word(&mut e, b"speed");
        assert!(e.call(0x005a_e7a0, &[d, w]).bool());
        assert_eq!((e.mem.u8(w + 0x204), e.mem.u32(w + 0x20c)), (0x73, 7));
        let w = word(&mut e, b"rate");
        assert!(e.call(0x005a_e7a0, &[d, w]).bool());
        assert_eq!((e.mem.u8(w + 0x204), e.mem.u32(w + 0x20c)), (0x66, 9));
        // A longer variable name sharing the prefix does not match.
        let w = word(&mut e, b"spee");
        assert!(!e.call(0x005a_e7a0, &[d, w]).bool());
        // A referenced object index makes no variable.
        let w = word(&mut e, b"speed");
        e.mem.set_u32(w + 0x200, 1);
        assert!(!e.call(0x005a_e7a0, &[d, w]).bool());
        // The current script answers next.
        e.mem.set_u32(d + 0x4c, 0x7700);
        e.register(SCRIPT_FIND_VARIABLE, |e, a| {
            e.mem.set_u32(a[2], 0x31);
            ret(0x66)
        });
        let w = word(&mut e, b"other");
        assert!(e.call(0x005a_e7a0, &[d, w]).bool());
        assert_eq!(e.mem.u32(w + 0x20c), 0x31);
        // A global is accepted when it is a referenced object.
        e.mem.set_u32(d + 0x4c, 0);
        let g = e.mem.alloc(0x10);
        e.register_double(GET_GLOBAL, move |_, _| ret(g));
        e.register_double(FORM_BY_EDITOR_ID, move |_, _| ret(g));
        e.mem.set_u8(g + 4, 6);
        let w = word(&mut e, b"gTimer");
        assert!(e.call(0x005a_e7a0, &[d, w]).bool());
        assert_eq!(e.mem.u8(w + 0x204), 0x47);
        // A word with a referenced form asks that form's script.
        let w = word(&mut e, b"done");
        e.mem.set_u32(w + 0x210, g);
        e.mem.set_u8(g + 4, 0x47);
        e.register(FORM_SCRIPT, |_, _| ret(0x7800));
        e.register(SCRIPT_FIND_VARIABLE, |_, a| {
            ret(u32::from(a[0] == 0x7800) * 0x73)
        });
        assert!(e.call(0x005a_e7a0, &[d, w]).bool());
        e.register(FORM_SCRIPT, |_, _| ret(0));
        assert!(!e.call(0x005a_e7a0, &[d, w]).bool());
    }

    #[test]
    fn script_error_prints_to_the_console_or_the_log() {
        let mut e = rig();
        let d = data(&mut e);
        e.mem.set_u32(d + 0x1c, 12);
        e.call_log = Some(vec![]);
        e.call(0x005a_ea90, &[d, 0x0103_7600, 0x55, 0x66]);
        let log = e.call_log.take().unwrap();
        let v = only(&log, VSPRINTF);
        assert_eq!(v[0][1], 0x0103_7600);
        assert_eq!(e.mem.u32(v[0][2]), 0x55);
        assert_eq!(e.mem.u32(v[0][2] + 4), 0x66);
        let p = only(&log, STRING_PRINTF);
        assert_eq!(p[0][1], TEXT_SCRIPT_LINE);
        assert_eq!(p[0][3], 12);
        assert_eq!(p[0][4], v[0][0]);
        assert_eq!(only(&log, SCRIPT_LOG).len(), 1);
        e.mem.set_u32(d + 8, 1);
        e.call_log = Some(vec![]);
        e.call(0x005a_ea90, &[d, 0x0103_7600]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, CONSOLE_PRINT).len(), 1);
        assert!(only(&log, SCRIPT_LOG).is_empty());
    }

    fn script(e: &mut Engine) -> u32 {
        let header = e.mem.alloc(0x14);
        e.mem.set_u8(header + 0x10, 1);
        e.mem.set_u32(header + 0xc, 40);
        e.register_double(SCRIPT_HEADER_OF, move |_, _| ret(header));
        e.register(SCRIPT_COMPILED_FLAG, ok);
        e.register(SCRIPT_INIT, ok);
        0x9900
    }

    #[test]
    fn compile_reads_the_header_and_initialises_the_script() {
        let mut e = rig();
        let s = script(&mut e);
        let d = data(&mut e);
        assert!(!e.call(0x005a_eb90, &[0, s, 0]).bool());
        e.call_log = Some(vec![]);
        assert!(e.call(0x005a_eb90, &[0, s, d]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u8(d + 0x38), 1);
        assert_eq!(e.mem.u32(d + 0x34), 40);
        assert_eq!(
            only(&log, SCRIPT_COMPILED_FLAG),
            vec![vec![s, 0], vec![s, 1]]
        );
        assert_eq!(only(&log, SCRIPT_INIT), vec![vec![s, d]]);
        // An unknown command fails the compile.
        let input = text(&mut e, b"zzz\r\n");
        e.mem.set_u32(d, input);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005a_eb90, &[0, s, d]).bool());
        assert_eq!(errors(&e), vec![TEXT_COMMAND_NOT_FOUND]);
        assert_eq!(e.mem.u32(d + 0x14), 0xd);
    }

    #[test]
    fn compile_needs_a_scriptname_first() {
        let mut e = rig();
        let s = script(&mut e);
        command(&mut e, b"set", 0x15);
        let d = data(&mut e);
        let input = text(&mut e, b"set x to 1\n");
        e.mem.set_u32(d, input);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005a_eb90, &[0, s, d]).bool());
        assert_eq!(errors(&e), vec![TEXT_START_WITH_SCRIPTNAME]);
    }

    #[test]
    fn compile_partial_script_names_and_runs_the_compile() {
        let mut e = rig();
        let s = script(&mut e);
        e.register(SCRIPT_TEXT, |_, _| ret(0));
        e.register(NAME_LENGTH, |_, _| ret(0));
        e.mem.set_u32(COMPILER_NAMES + 8, 0x0103_0000);
        let seen = std::rc::Rc::new(std::cell::Cell::new((0u32, 0u8)));
        let s2 = seen.clone();
        e.register_double(SCRIPT_INIT, move |e, a| {
            s2.set((e.mem.u32(a[1] + 8), e.mem.u8(a[1] + 0x18)));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x005a_edb0, &[0, s, 0, 2, 0]).bool());
        let log = e.call_log.take().unwrap();
        let assigned = only(&log, STRING_ASSIGN);
        assert_eq!(assigned[0][1], 0x0103_0000);
        assert_eq!(seen.get(), (2, 1));
        // A reference's script gives its header, objects and name.
        let source = e.mem.alloc(0x200);
        let vt = e.mem.alloc(0x140);
        e.mem.set_u32(source, vt);
        e.mem.set_u32(vt + 0x130, 0x0050_0000);
        let name = text(&mut e, b"RefScript");
        e.register_double(0x0050_0000, move |_, _| ret(name));
        e.register_double(REFR_SCRIPT, move |_, _| ret(source));
        e.register(NAME_LENGTH, |_, _| ret(9));
        e.register(COPY_REF_OBJECTS, ok);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005a_edb0, &[0, s, 0x4500, 0, 0]).bool());
        let log = e.call_log.take().unwrap();
        let copy = only(&log, COPY_REF_OBJECTS);
        assert_eq!(copy[0][0], source + 0x44);
        assert_eq!(copy[0][2], 0);
        let set = only(&log, STRING_SET);
        assert_eq!(set.last().unwrap()[1], name);
        assert_eq!(only(&log, MEMCPY)[0][2], 0x14);
    }

    #[test]
    fn get_next_expression_reads_command_lines() {
        let mut e = rig();
        command(&mut e, b"set", 0x15);
        let d = data(&mut e);
        let input = text(&mut e, b"  set x to 1\r\n");
        e.mem.set_u32(d, input);
        assert_eq!(e.call(0x005a_eed0, &[0, d]).u32(), 0x15);
        let l = e.mem.u32(d + 0x50);
        assert_eq!(e.mem.cstr(l + 4), b" x to 1".to_vec());
        assert_eq!(e.mem.u32(l + 0x204), 7);
        assert_eq!(e.mem.u32(l), 1);
        assert_eq!(e.mem.u32(d + 0x1c), 1);
        assert_eq!(e.call(0x005a_eed0, &[0, d]).u32(), 0xffff);
        // scriptname takes the next word as the name; an if needs a condition.
        let mut e = rig();
        command(&mut e, b"scn", 0x1d);
        let d = data(&mut e);
        let input = text(&mut e, b"scn MyScript\n");
        e.mem.set_u32(d, input);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005a_eed0, &[0, d]).u32(), 0x1d);
        assert_eq!(e.mem.cstr(e.mem.u32(d + 0xc)), b"MyScript".to_vec());
        let mut e = rig();
        command(&mut e, b"if", 0x16);
        let d = data(&mut e);
        // A line feed alone does not end a line: it stays in the condition.
        let input = text(&mut e, b"if\n");
        e.mem.set_u32(d, input);
        assert_eq!(e.call(0x005a_eed0, &[0, d]).u32(), 0x16);
        let d = data(&mut e);
        let input = text(&mut e, b"if\r\n");
        e.mem.set_u32(d, input);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005a_eed0, &[0, d]).u32(), 0xffff);
        assert_eq!(errors(&e), vec![TEXT_NO_CONDITION]);
        assert_eq!(e.mem.u32(d + 0x14), 6);
        // Console commands only for compiler index 1.
        let mut e = rig();
        let t = text(&mut e, b"tgm");
        e.mem.set_u32(CONSOLE_COMMANDS, t);
        e.mem.set_u32(CONSOLE_COMMANDS + 8, 0x120);
        let d = data(&mut e);
        let input = text(&mut e, b"tgm\n");
        e.mem.set_u32(d, input);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005a_eed0, &[0, d]).u32(), 0xffff);
        assert_eq!(errors(&e), vec![TEXT_CONSOLE_ONLY]);
    }

    #[test]
    fn word_and_line_constructors_clear_their_fields() {
        let mut e = rig();
        let w = e.mem.alloc(0x214);
        e.mem.write(w, &[0xaa; 0x214]);
        assert_eq!(e.call(0x005a_f310, &[w]).u32(), w);
        assert_eq!(e.mem.bytes(w, 0x205), vec![0; 0x205]);
        assert_eq!(e.mem.u8(w + 0x205), 0xaa);
        assert_eq!(e.mem.bytes(w + 0x208, 0xc), vec![0; 0xc]);
        let l = e.mem.alloc(0x41c);
        e.mem.write(l, &[0xaa; 0x41c]);
        assert_eq!(e.call(0x005a_f370, &[l]).u32(), l);
        for at in [0, 4, 0x203, 0x204, 0x208, 0x20c, 0x40b, 0x40c, 0x418] {
            assert_eq!(e.mem.u8(l + at), 0, "{at:x}");
        }
        assert_eq!(e.mem.u8(l + 0x410), 0xaa);
    }

    #[test]
    fn get_next_line_skips_comments_and_blank_lines() {
        let mut e = rig();
        let d = data(&mut e);
        let input = text(&mut e, b"; note\r\n\r\n  set x  \t\r\nend");
        e.mem.set_u32(d, input);
        let l = line(&mut e, 0, 0, b"");
        let read = e.call(0x005a_f3f0, &[d, l]).u32();
        assert_eq!(e.mem.cstr(l + 4), b"set x".to_vec());
        assert_eq!(e.mem.u32(l + 0x204), 5);
        assert_eq!(e.mem.u32(l), 2);
        assert_eq!(read, 22);
        e.mem.set_u32(d + 4, read);
        let l = line(&mut e, 0, 0, b"");
        assert_eq!(e.call(0x005a_f3f0, &[d, l]).u32(), 4);
        assert_eq!(e.mem.cstr(l + 4), b"end".to_vec());
        let l = line(&mut e, 0, 0, b"");
        e.mem.set_u32(d + 4, 25);
        assert_eq!(e.call(0x005a_f3f0, &[d, l]).u32(), 0);
        // A line of 0x1FF characters is too long.
        let long = vec![b'a'; 0x200];
        let input = text(&mut e, &long);
        e.mem.set_u32(d, input);
        e.mem.set_u32(d + 4, 0);
        let l = line(&mut e, 0, 0, b"");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005a_f3f0, &[d, l]).u32(), 0);
        assert_eq!(errors(&e), vec![TEXT_LINE_TOO_LONG]);
        assert_eq!(e.mem.u32(l + 0x418), 0x10);
    }

    #[test]
    fn get_next_word_reads_plain_quoted_and_reference_words() {
        let mut e = rig();
        let d = data(&mut e);
        let t = text(&mut e, b"  abc_1, \"two words\" x");
        let pos = e.mem.alloc(4);
        let w = word(&mut e, b"");
        assert_eq!(e.call(0x005a_f5f0, &[d, w, t, pos, 0, 0]).u32(), 5);
        assert_eq!(e.mem.cstr(w), b"abc_1".to_vec());
        assert_eq!(e.mem.u32(pos), 7);
        let w = word(&mut e, b"");
        assert_eq!(e.call(0x005a_f5f0, &[d, w, t, pos, 0, 0]).u32(), 9);
        assert_eq!(e.mem.cstr(w), b"two words".to_vec());
        assert_eq!(e.mem.u32(pos), 20);
        assert_eq!(e.mem.cstr(t), b"  abc_1, \"two words\" x".to_vec());
        // Mismatched quotes, and a reference where none is allowed.
        let t = text(&mut e, b"\"open");
        e.mem.set_u32(pos, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005a_f5f0, &[d, w, t, pos, 0, 0]).u32(), 0);
        assert_eq!(errors(&e), vec![TEXT_MISMATCHED_QUOTES]);
        let t = text(&mut e, b"Player.GetAV");
        e.mem.set_u32(pos, 0);
        let w = word(&mut e, b"");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005a_f5f0, &[d, w, t, pos, 0, 0]).u32(), 0);
        assert_eq!(errors(&e), vec![TEXT_REFERENCE_NOT_ALLOWED]);
        // With references allowed the reference is resolved and the next word read.
        e.mem.set_u32(PLAYER_REF, 0x14);
        command(&mut e, b"GetAV", 0x1014);
        e.register(RT_DYNAMIC_CAST, |_, a| {
            ret(u32::from(a[3] == TYPE_TES_OBJECT_REFR) * a[0])
        });
        e.register(REFR_PERSISTS, |_, _| ret(1));
        e.register(REFR_INIT_SCRIPT, ok);
        e.register(REFR_SCRIPT, |_, _| ret(0));
        e.mem.set_u32(pos, 0);
        let w = word(&mut e, b"");
        assert_eq!(e.call(0x005a_f5f0, &[d, w, t, pos, 1, 0]).u32(), 5);
        assert_eq!(e.mem.cstr(w), b"GetAV".to_vec());
        assert_eq!(e.mem.u32(w + 0x200), 1);
        assert_eq!(e.mem.u8(w + 0x204), 0x58);
        assert_eq!(e.mem.u32(w + 0x208), 0x1014);
    }

    fn ref_object(e: &mut Engine, name: &[u8], form: u32, variable: u32) -> u32 {
        let o = e.mem.alloc(0x10);
        let t = text(e, name);
        e.mem.set_u32(o, t);
        e.mem.set_u32(o + 8, form);
        e.mem.set_u32(o + 0xc, variable);
        o
    }

    #[test]
    fn verify_referenced_object_list_needs_a_form_or_variable() {
        let mut e = rig();
        let d = data(&mut e);
        let a = ref_object(&mut e, b"A", 0x10, 0);
        let b = ref_object(&mut e, b"B", 0, 3);
        push(&mut e, d + 0x44, a);
        push(&mut e, d + 0x44, b);
        assert!(e.call(0x005a_fa50, &[d]).bool());
        let c = ref_object(&mut e, b"C", 0, 0);
        push(&mut e, d + 0x44, c);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005a_fa50, &[d]).bool());
        assert_eq!(errors(&e), vec![TEXT_UNKNOWN_REFERENCED_OBJECT]);
    }

    #[test]
    fn get_referenced_object_finds_or_adds_objects() {
        let mut e = rig();
        let d = data(&mut e);
        let a = ref_object(&mut e, b"Door", 0x5000, 7);
        push(&mut e, d + 0x44, a);
        let w = word(&mut e, b"door");
        assert!(e.call(0x005a_fad0, &[d, w, 0, 0]).bool());
        assert_eq!(
            (
                e.mem.u32(w + 0x200),
                e.mem.u32(w + 0x210),
                e.mem.u32(w + 0x20c)
            ),
            (1, 0x5000, 7)
        );
        e.mem.set_u32(w + 0x20c, 9);
        assert!(e.call(0x005a_fad0, &[d, w, 1, 0]).bool());
        assert_eq!(e.mem.u32(a + 0xc), 9);
        // "Player" becomes a new object with the player.
        e.mem.set_u32(PLAYER_REF, 0x14);
        let w = word(&mut e, b"player");
        assert!(e.call(0x005a_fad0, &[d, w, 0, 0]).bool());
        assert_eq!((e.mem.u32(w + 0x200), e.mem.u32(w + 0x210)), (1, 0x14));
        assert_eq!(e.mem.u32(d + 0x2c), 1);
        // A hex form id; a non-persistent reference is refused and deleted.
        e.register(LOOKUP_FORM, |_, a| ret(u32::from(a[0] != 0) * 0x6000));
        let w = word(&mut e, b"abc");
        assert!(e.call(0x005a_fad0, &[d, w, 0, 0]).bool());
        assert_eq!(e.mem.u32(w + 0x210), 0x6000);
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(REFR_PERSISTS, |_, _| ret(0));
        let w = word(&mut e, b"abd");
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005a_fad0, &[d, w, 0, 0]).bool());
        assert_eq!(errors(&e), vec![TEXT_NON_PERSISTENT]);
        assert_eq!(
            only(e.call_log.as_ref().unwrap(), REF_OBJECT_DELETE).len(),
            1
        );
        // Forced, an unknown word is still added; a type 3 form does not count.
        let form = e.mem.alloc(0x10);
        e.register_double(FORM_BY_EDITOR_ID, move |_, _| ret(form));
        e.mem.set_u8(form + 4, 3);
        let w = word(&mut e, b"Thing");
        assert!(e.call(0x005a_fad0, &[d, w, 0, 1]).bool());
        assert_eq!(e.mem.u32(w + 0x210), 0);
        assert_eq!(e.mem.u32(w + 0x200), e.mem.u32(d + 0x2c));
    }

    #[test]
    fn strtoul_wrapper_and_object_by_index() {
        let mut e = rig();
        let t = text(&mut e, b"1f");
        let end = e.mem.alloc(4);
        assert_eq!(e.call(0x005a_fe10, &[t, end, 0x10]).u32(), 0x1f);
        assert_eq!(e.mem.u32(end), t + 2);
        let d = data(&mut e);
        let a = ref_object(&mut e, b"A", 1, 0);
        let b = ref_object(&mut e, b"B", 2, 0);
        push(&mut e, d + 0x44, a);
        push(&mut e, d + 0x44, b);
        e.mem.set_u32(d + 0x2c, 2);
        assert_eq!(e.call(0x005a_fe30, &[d, 0]).u32(), 0);
        assert_eq!(e.call(0x005a_fe30, &[d, 1]).u32(), a);
        assert_eq!(e.call(0x005a_fe30, &[d, 2]).u32(), b);
        assert_eq!(e.call(0x005a_fe30, &[d, 3]).u32(), 0);
    }

    #[test]
    fn valid_variable_names_are_not_forms_or_functions() {
        let mut e = rig();
        command(&mut e, b"set", 0x15);
        let n = text(&mut e, b"count");
        assert!(e.call(0x005a_feb0, &[n]).bool());
        let n = text(&mut e, b"Set");
        assert!(!e.call(0x005a_feb0, &[n]).bool());
        let n = text(&mut e, b"");
        assert!(!e.call(0x005a_feb0, &[n]).bool());
        assert!(!e.call(0x005a_feb0, &[0]).bool());
        e.register(FORM_BY_EDITOR_ID, |_, _| ret(0x10));
        let n = text(&mut e, b"count");
        assert!(!e.call(0x005a_feb0, &[n]).bool());
    }

    fn lines(e: &mut Engine, d: u32, opcodes: &[u32]) -> Vec<u32> {
        let mut out = vec![];
        for (i, op) in opcodes.iter().enumerate() {
            let l = line(e, i as u32 + 1, *op, b"");
            push(e, d + 0x50, l);
            out.push(l);
        }
        out
    }

    #[test]
    fn verify_blocks_checks_begin_and_end() {
        let mut e = rig();
        let d = data(&mut e);
        lines(&mut e, d, &[0x1d, 0x12, 0x10, 0x15, 0x11]);
        assert!(e.call(0x005b_1180, &[0, d]).bool());
        for (ops, error) in [
            (vec![0x10, 0x10], TEXT_NESTED_BLOCKS),
            (vec![0x11], TEXT_END_WITHOUT_BEGIN),
            (vec![0x15], TEXT_OUTSIDE_BLOCK),
        ] {
            let mut e = rig();
            let d = data(&mut e);
            let ls = lines(&mut e, d, &ops);
            e.call_log = Some(vec![]);
            assert!(!e.call(0x005b_1180, &[0, d]).bool());
            assert_eq!(errors(&e), vec![error]);
            assert_eq!(e.mem.u32(d + 0x1c), e.mem.u32(*ls.last().unwrap()));
        }
    }

    #[test]
    fn count_block_and_if_lines() {
        let mut e = rig();
        let d = data(&mut e);
        let ls = lines(
            &mut e,
            d,
            &[0x10, 0x12, 0x15, 0x16, 0x15, 0x17, 0x15, 0x19, 0x11],
        );
        assert_eq!(e.call(0x005b_12c0, &[0, d, ls[0]]).i32(), 6);
        assert_eq!(e.call(0x005b_13e0, &[0, d, ls[3]]).i32(), 1);
        assert_eq!(e.call(0x005b_13e0, &[0, d, ls[5]]).i32(), 1);
        let mut e = rig();
        let d = data(&mut e);
        let ls = lines(&mut e, d, &[0x10, 0x15]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005b_12c0, &[0, d, ls[0]]).i32(), -1);
        assert_eq!(errors(&e), vec![TEXT_MISMATCHED_BLOCK]);
        let mut e = rig();
        let d = data(&mut e);
        let ls = lines(&mut e, d, &[0x16, 0x16, 0x15, 0x19, 0x19]);
        assert_eq!(e.call(0x005b_13e0, &[0, d, ls[0]]).i32(), 3);
        let mut ops = vec![0x16; 11];
        ops.push(0x19);
        let mut e = rig();
        let d = data(&mut e);
        let ls = lines(&mut e, d, &ops);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005b_13e0, &[0, d, ls[0]]).i32(), -1);
        assert_eq!(errors(&e), vec![TEXT_MAX_NESTING]);
    }

    #[test]
    fn replace_variables_and_functions_codes_words() {
        let mut e = rig();
        let d = data(&mut e);
        let v = variable(&mut e, b"x", 3, 0);
        push(&mut e, d + 0x3c, v);
        let t = text(&mut e, b"x + 1");
        let out = e.mem.alloc(0x40);
        let number = e.mem.alloc(4);
        let n = e.call(0x005b_1570, &[0, d, number, t, out]).u32();
        assert_eq!(
            e.mem.bytes(out, n),
            vec![0x66, 3, 0, b' ', b'+', b' ', b'1']
        );
        // A command without parameters gets a 16-bit 0; an unknown word is reported.
        command(&mut e, b"GetX", 0x1010);
        let t = text(&mut e, b"GetX");
        let n = e.call(0x005b_1570, &[0, d, number, t, out]).u32();
        assert_eq!(e.mem.bytes(out, n), vec![0x58, 0x10, 0x10, 0, 0]);
        let t = text(&mut e, b"zork");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005b_1570, &[0, d, number, t, out]).u32(), 0);
        assert_eq!(errors(&e), vec![TEXT_UNKNOWN_COMMAND]);
    }

    #[test]
    fn get_function_searches_the_three_tables() {
        let mut e = rig();
        let b = text(&mut e, b"GameMode");
        e.mem.set_u32(BLOCK_TABLE + 0x28 * 2 + 4, b);
        e.mem.set_u32(BLOCK_TABLE + 0x28 * 2 + 8, 0x10);
        let c = text(&mut e, b"tgm");
        e.mem.set_u32(CONSOLE_COMMANDS + 0x28 * 5, c);
        e.mem.set_u32(CONSOLE_COMMANDS + 0x28 * 5 + 8, 0x105);
        let w = word(&mut e, b"gamemode");
        assert!(e.call(0x005b_1910, &[w]).bool());
        assert_eq!((e.mem.u8(w + 0x204), e.mem.u32(w + 0x208)), (0x58, 0x10));
        let w = word(&mut e, b"TGM");
        assert!(e.call(0x005b_1910, &[w]).bool());
        assert_eq!(e.mem.u32(w + 0x208), 0x105);
        let w = word(&mut e, b"nothing");
        assert!(!e.call(0x005b_1910, &[w]).bool());
        assert_eq!(e.mem.u8(w + 0x204), 0);
    }

    #[test]
    fn parentheses_quotes_and_blanks() {
        let mut e = rig();
        let t = text(&mut e, b"(a (b)) c)");
        assert!(!e.call(0x005b_1a90, &[t, 0]).bool());
        assert!(!e.call(0x005b_1a90, &[t, 1]).bool());
        let t = text(&mut e, b"(a (b))");
        assert!(e.call(0x005b_1a90, &[t, 0]).bool());
        let t = text(&mut e, b"\"a\" \"b");
        assert!(!e.call(0x005b_1b00, &[t, 0]).bool());
        assert!(e.call(0x005b_1b00, &[t, 1]).bool());
        let t = text(&mut e, b"\"a\" \"b\"");
        assert!(e.call(0x005b_1b00, &[t, 0]).bool());
        assert!(!e.call(0x005b_1b00, &[t, 5]).bool());
        let t = text(&mut e, b" \t,,x");
        let pos = e.mem.alloc(4);
        e.call(0x005b_1b60, &[t, pos]);
        assert_eq!(e.mem.u32(pos), 4);
    }

    #[test]
    fn compile_line_writes_set_lines_to_the_output() {
        let mut e = rig();
        let d = data(&mut e);
        let v = variable(&mut e, b"x", 3, 0);
        push(&mut e, d + 0x3c, v);
        e.register(EXPRESSION_TO_POSTFIX, |e, a| {
            let p = e.mem.alloc(4);
            e.mem.write(p, &[1, 2, 3]);
            e.mem.set_u32(a[2], 3);
            ret(p)
        });
        let l = line(&mut e, 1, 0x15, b" x to 5");
        assert!(e.call(0x005a_ff40, &[0, 0x9900, d, l]).bool());
        let output = e.mem.u32(d + 0x20);
        assert_eq!(e.mem.u32(d + 0x24), 4 + 3 + 2 + 3);
        assert_eq!(
            e.mem.bytes(output, 12),
            vec![0x15, 0, 8, 0, 0x66, 3, 0, 3, 0, 1, 2, 3]
        );
        // A missing "to" is reported.
        let l = line(&mut e, 2, 0x15, b" x 5");
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005a_ff40, &[0, 0x9900, d, l]).bool());
        assert_eq!(errors(&e), vec![TEXT_MISSING_TO]);
    }

    #[test]
    fn compile_line_begin_end_and_declarations() {
        let mut e = rig();
        let d = data(&mut e);
        let name = text(&mut e, b"GameMode");
        e.mem.set_u32(BLOCK_TYPES, name);
        e.mem.set_u16(BLOCK_TYPES + 8, 0);
        let begin = line(&mut e, 1, 0x10, b" GameMode");
        let end = line(&mut e, 2, 0x11, b"");
        push(&mut e, d + 0x50, begin);
        push(&mut e, d + 0x50, end);
        assert!(e.call(0x005a_ff40, &[0, 0x9900, d, begin]).bool());
        let output = e.mem.u32(d + 0x20);
        // opcode, size 6, block code 0, jump word (the offset after the line).
        assert_eq!(e.mem.u16(output), 0x10);
        assert_eq!(e.mem.u16(output + 2), 6);
        assert_eq!(e.mem.u32(output + 6), 6);
        assert_eq!(e.global::<u32>(BLOCK_JUMP), 2);
        assert!(e.call(0x005a_ff40, &[0, 0x9900, d, end]).bool());
        assert_eq!(e.mem.u32(output + 6), 4);
        assert_eq!(e.global::<u32>(BLOCK_JUMP), 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005a_ff40, &[0, 0x9900, d, end]).bool());
        assert_eq!(errors(&e), vec![TEXT_NO_JUMP_BLOCK]);
        // A declaration adds a numbered variable and writes nothing.
        e.register(VARIABLE_CONSTRUCT, |e, a| {
            e.mem.write(a[0], &[0; 0x20]);
            ret(a[0])
        });
        e.register(SCRIPT_FIND_VARIABLE, |_, _| ret(0));
        e.mem.set_u32(d + 0x34, 4);
        let size = e.mem.u32(d + 0x24);
        let decl = line(&mut e, 3, 0x14, b" speed");
        assert!(e.call(0x005a_ff40, &[0, 0x9900, d, decl]).bool());
        let v = e.mem.u32(d + 0x3c);
        assert_eq!((e.mem.u32(v), e.mem.u8(v + 0x10)), (5, 0));
        assert_eq!(e.mem.u32(d + 0x24), size);
    }

    fn parameters(e: &mut Engine, list: &[(u32, u8)]) -> u32 {
        let p = e.mem.alloc(0xc * (list.len() as u32 + 1));
        for (i, (kind, optional)) in list.iter().enumerate() {
            let at = p + 0xc * i as u32;
            e.mem.set_u32(at, 0x0103_0000 + i as u32);
            e.mem.set_u32(at + 4, *kind);
            e.mem.set_u8(at + 8, *optional);
        }
        p
    }

    #[test]
    fn standard_compile_writes_values_and_objects() {
        let mut e = rig();
        let d = data(&mut e);
        // Type 1 (integer) may be a variable; type 0x13 (global) names an object.
        e.mem.set_u8(PARAMETER_TYPES + 8, 1);
        e.mem.set_u8(PARAMETER_TYPES + 0x13 * 8 + 1, 1);
        e.register(IS_AN_INTEGER, |_, _| ret(1));
        e.register(TEXT_TO_INTEGER, |_, _| ret(42));
        let form = e.mem.alloc(0x10);
        e.register_double(FORM_BY_EDITOR_ID, move |_, _| ret(form));
        e.mem.set_u8(form + 4, 6);
        let p = parameters(&mut e, &[(1, 0), (0x13, 0), (1, 1)]);
        let l = line(&mut e, 1, 0x1000, b" 42 gValue");
        assert!(e.call(0x005b_1ba0, &[3, p, l, d]).bool());
        assert_eq!(
            e.mem.bytes(l + 0x20c, e.mem.u32(l + 0x40c)),
            vec![2, 0, 0x6e, 42, 0, 0, 0, 0x72, 1, 0]
        );
        // A required parameter missing, an object of the wrong type, text left over.
        let l = line(&mut e, 1, 0x1000, b"");
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005b_1ba0, &[1, p, l, d]).bool());
        assert_eq!(errors(&e), vec![TEXT_MISSING_PARAMETER]);
        e.mem.set_u8(form + 4, 7);
        let p2 = parameters(&mut e, &[(0x13, 0)]);
        let l = line(&mut e, 1, 0x1000, b" gOther");
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005b_1ba0, &[1, p2, l, d]).bool());
        assert_eq!(errors(&e), vec![TEXT_INVALID_GLOBAL]);
        let p3 = parameters(&mut e, &[(0, 0)]);
        let l = line(&mut e, 1, 0x1000, b" a b");
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005b_1ba0, &[1, p3, l, d]).bool());
        assert_eq!(errors(&e), vec![TEXT_EXPECTED_END_OF_LINE]);
        assert_eq!(e.mem.bytes(l + 0x20c, 5), vec![1, 0, 1, 0, b'a']);
        // An unhandled value type is logged.
        let p4 = parameters(&mut e, &[(3, 0)]);
        let l = line(&mut e, 1, 0x1000, b" a");
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005b_1ba0, &[1, p4, l, d]).bool());
        assert_eq!(
            only(e.call_log.as_ref().unwrap(), SCRIPT_LOG),
            vec![vec![TEXT_TYPE_UNIMPLEMENTED, 3]]
        );
    }

    #[test]
    fn message_box_compile_text_variables_and_time() {
        let mut e = rig();
        let d = data(&mut e);
        let v = variable(&mut e, b"x", 3, 0);
        push(&mut e, d + 0x3c, v);
        let p = parameters(&mut e, &[(0, 0)]);
        let l = line(&mut e, 1, 0x1000, b" \"Hits: %.0f %%\" x");
        assert!(e.call(0x005b_3ca0, &[1, p, l, d]).bool());
        let mut want = vec![1, 0, 13, 0];
        want.extend_from_slice(b"Hits: %.0f %%");
        want.extend_from_slice(&[1, 0, 0x66, 3, 0]);
        let size = e.mem.u32(l + 0x40c);
        assert_eq!(e.mem.bytes(l + 0x20c, size), want);
        // Too few variables.
        let l = line(&mut e, 1, 0x1000, b" \"%g %g\" x");
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005b_3ca0, &[1, p, l, d]).bool());
        assert_eq!(errors(&e), vec![TEXT_TOO_FEW_VARIABLES]);
        // A message form with no conversions, then a time or buttons.
        let message = e.mem.alloc(0x40);
        let vt = e.mem.alloc(0x20);
        e.mem.set_u32(message + 0x24, vt);
        e.mem.set_u32(vt + 0x10, 0x0050_0000);
        let description = text(&mut e, b"No values");
        e.register_double(0x0050_0000, move |_, _| ret(description));
        e.register_double(FORM_BY_EDITOR_ID, move |_, _| ret(message));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(REFR_PERSISTS, |_, _| ret(1));
        e.register(IS_AN_INTEGER, |_, _| ret(1));
        e.register(TEXT_TO_INTEGER, |_, _| ret(5));
        let l = line(&mut e, 1, 0x1000, b" MyMessage 5");
        assert!(e.call(0x005b_3c70, &[1, p, l, d]).bool());
        let size = e.mem.u32(l + 0x40c);
        assert_eq!(
            e.mem.bytes(l + 0x20c, size),
            vec![1, 0, 0x72, 1, 0, 0, 0, 5, 0, 0, 0]
        );
        e.register(VECTOR_CONSTRUCTOR, |e, a| {
            for i in 0..a[2] {
                e.call(a[3], &[a[0] + i * a[1]]);
            }
            Ret::default()
        });
        let l = line(&mut e, 1, 0x1000, b" MyMessage Yes No");
        assert!(e.call(0x005b_3c40, &[1, p, l, d]).bool());
        let size = e.mem.u32(l + 0x40c);
        assert_eq!(
            e.mem.bytes(l + 0x20c, size),
            vec![
                1, 0, 0x72, 1, 0, 0, 0, 2, 0, 1, 0, 3, 0, b'Y', b'e', b's', 1, 0, 2, 0, b'N', b'o'
            ]
        );
    }
}
