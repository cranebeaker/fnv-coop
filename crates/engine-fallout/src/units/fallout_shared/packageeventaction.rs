//! `fallout shared/packageeventaction.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `PackageEventAction` (Xbox PDB), 0x10 bytes.
    pub struct PackageEventAction: 0x10 {
        /// `pIdle` (Xbox PDB): `TESIdleForm*`; a form id until `InitItem`.
        0x00 pIdle: u32,
        /// `pResultScript` (Xbox PDB): `Script*`.
        0x04 pResultScript: u32,
        /// `pTopic` (Xbox PDB): `TESTopic*`; a form id until `InitItem`.
        0x08 pTopic: u32,
    }
}

/// `00401000` (cdecl: size): allocates.
const ALLOCATE: u32 = 0x0040_1000;
/// `Script::Script` (Xbox PDB name of `005aa0f0`; `ECX` = memory).
const SCRIPT_CONSTRUCT: u32 = 0x005a_a0f0;
/// The size of a `Script` (0x54).
const SCRIPT_SIZE: u32 = 0x54;
/// `TESFile::GetTESChunk` (Xbox PDB name of `004726b0`; `ECX` = file).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (`ECX` = file): moves to the next chunk.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESFile::GetChunkData` (Xbox PDB name of `004727f0`; `ECX` = file, the
/// address of a word).
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`; `ECX` =
/// file; buffer, size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `00401660` (`ECX` = file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401680` (`ECX` = file): whether the data read needs its byte order
/// swapped.
const FILE_NEEDS_SWAP: u32 = 0x0040_1680;
/// `00403d30` (cdecl: destination, byte, count): fills memory.
const FILL_MEMORY: u32 = 0x0040_3d30;
/// `BSSimpleList<SCRIPT_REFERENCED_OBJECT_P>::AddTail` (Xbox PDB name of
/// `00500910`; `ECX` = memory): constructs a referenced-object entry there.
const REFERENCED_OBJECT_CONSTRUCT: u32 = 0x0050_0910;
/// `005d43c0` (`ECX` = script): its referenced-object list.
const SCRIPT_REFERENCED_OBJECTS: u32 = 0x005d_43c0;
/// `005d43e0` (`ECX` = script): its variable list.
const SCRIPT_VARIABLES: u32 = 0x005d_43e0;
/// `00905820` (`ECX` = list, the address of the entry; `RET 4`): appends.
const LIST_APPEND: u32 = 0x0090_5820;
/// `005e28b0` (`ECX` = memory): constructs a 0x20-byte script variable.
const VARIABLE_CONSTRUCT: u32 = 0x005e_28b0;
/// `ScriptVariable::Load` (Xbox PDB name of `005e2a20`; `ECX` = variable,
/// the file; `RET 4`).
const VARIABLE_LOAD: u32 = 0x005e_2a20;
/// The variable being loaded (`011d66ac`, a pointer global).
const CURRENT_VARIABLE: u32 = 0x011d_66ac;
/// `004037f0` (`ECX` = string; characters, a word; `RET 8`): sets the
/// string.
const STRING_SET: u32 = 0x0040_37f0;
/// `00500940` (`ECX` = script): its script data header.
const SCRIPT_HEADER: u32 = 0x0050_0940;
/// `00414220` (`ECX` = header): its byte swap.
const HEADER_SWAP: u32 = 0x0041_4220;
/// `Script::SetCompileData` (Xbox PDB name of `005aae70`; `ECX` = script;
/// size, data; `RET 8`).
const SET_COMPILE_DATA: u32 = 0x005a_ae70;
/// `00484ab0` (`ECX` = form, a word; `RET 4`).
const FORM_SET_484AB0: u32 = 0x0048_4ab0;
/// `00671d10` (`ECX` = script): whether it is kept.
const SCRIPT_KEPT: u32 = 0x0067_1d10;
/// The chunk tags.
const CHUNK_SCRO: u32 = 0x4f52_4353;
const CHUNK_INAM: u32 = 0x4d41_4e49;
const CHUNK_SCDA: u32 = 0x4144_4353;
const CHUNK_SLSD: u32 = 0x4453_4c53;
const CHUNK_TNAM: u32 = 0x4d41_4e54;
const CHUNK_SCHR: u32 = 0x5248_4353;
const CHUNK_SCVR: u32 = 0x5256_4353;
const CHUNK_SCRV: u32 = 0x5652_4353;
/// The most a variable name (`SCVR` without `SLSD`) is read (0x200 bytes).
const NAME_MAXIMUM: u32 = 0x200;
/// `TESForm::GetFile` (Xbox PDB name of `00484e60`; `ECX` = form, an index).
const GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB name of `00485d50`; cdecl: address
/// of a form id, file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (cdecl: form id): the form with that id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `TESForm` type descriptor (`01183028`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// The `TESIdleForm` type descriptor (`01186a18`).
const TYPE_TES_IDLE_FORM: u32 = 0x0118_6a18;
/// The `TESTopic` type descriptor (`01184720`).
const TYPE_TES_TOPIC: u32 = 0x0118_4720;
/// `00474cb0` (`ECX` = owner): whether to name it by its slot `0x130`.
const OWNER_NAMED: u32 = 0x0047_4cb0;
/// `0084e3a0` (`ECX` = form): its form id.
const FORM_ID: u32 = 0x0084_e3a0;
/// `005b5e40` (cdecl: format, arguments...): writes a warning line.
const LOG_WARNING: u32 = 0x005b_5e40;
/// The warnings for an idle and a topic that are not found, with the owner
/// by name (`01068c18`, `01068b98`) or by id (`01068bd8`, `01068b58`).
const FORMAT_NO_IDLE_NAMED: u32 = 0x0106_8c18;
const FORMAT_NO_IDLE: u32 = 0x0106_8bd8;
const FORMAT_NO_TOPIC_NAMED: u32 = 0x0106_8b98;
const FORMAT_NO_TOPIC: u32 = 0x0106_8b58;
/// `"MASTERFILE: Errors were encountered during InitItem for result script
/// on eventaction for package '%s' (%08X)..."` (`01068ac0`).
const FORMAT_SCRIPT_ERRORS: u32 = 0x0106_8ac0;
/// `0046e8a0` (no arguments): the count of warnings so far.
const WARNING_COUNT: u32 = 0x0046_e8a0;
/// `MessageHandler::IncDisableWarningCount` (Xbox PDB name of `0043b2b0`;
/// cdecl, a word).
const DISABLE_WARNINGS: u32 = 0x0043_b2b0;
/// `004fffe0` (cdecl, a word): sets the count of warnings.
const SET_WARNING_COUNT: u32 = 0x004f_ffe0;
/// `TESForm::SetTemporary` (Xbox PDB name of `00484490`; `ECX` = form).
const SET_TEMPORARY: u32 = 0x0048_4490;
/// `Script::InitItem` (Xbox PDB name of `005ab400`; `ECX` = script, the
/// owner; `RET 4`).
const SCRIPT_INIT_ITEM: u32 = 0x005a_b400;

/// `_alloca_probe_16`: a stack buffer of `size` rounded up to 16 bytes.
fn alloca_size(size: u32) -> u32 {
    size.wrapping_add(15) & !15
}

/// A referenced-object entry (`SCRO` at +8, `SCRV` at +0xc of the 0x10-byte
/// entry): the chunk's word is stored at `offset` and the entry appended to
/// the script's referenced objects.
fn read_referenced_object(e: &mut Engine, this: Ptr<PackageEventAction>, file: Ptr, offset: u32) {
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    let entry = if block != 0 {
        e.call(REFERENCED_OBJECT_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.with_stack(8, |e, slots| {
        e.mem.set_u32(slots.addr(), entry);
        e.call(GET_CHUNK_WORD, &args![file, slots.addr() + 4]);
        let value = e.mem.u32(slots.addr() + 4);
        e.mem.set_u32(entry.wrapping_add(offset), value);
        let script = e.get(this, PackageEventAction::pResultScript);
        let list = e.call(SCRIPT_REFERENCED_OBJECTS, &args![script]).u32();
        e.call(LIST_APPEND, &args![list, slots]);
    });
}

/// Deletes the result script (its virtual slot `0x10` with 1) when there is
/// one, and clears the field.
fn drop_script(e: &mut Engine, this: Ptr<PackageEventAction>) {
    let script = e.get(this, PackageEventAction::pResultScript);
    if script != 0 {
        e.vcall(script, 0x10, &args![1u32]);
    }
    e.set(this, PackageEventAction::pResultScript, 0);
}

// Translated from 0067dd20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067dd20 - the exception-unwinding frame (FS:[0]) around the allocations and the security-cookie check (00ec408c) are not translated.
/// `PackageEventAction::Load` (Xbox PDB; the file; `RET 4`): clears the idle
/// and the topic, deletes an old result script, and makes a new one (0x54
/// bytes, `Script::Script`; 0 when the allocation fails). Then chunk after
/// chunk (`004726f0` between them; the loop ends only at `TNAM`, not when
/// there is no next chunk):
/// - `INAM`: the idle's form id; `TNAM`: the topic's, and the end;
/// - `SCHR`: the script data header (`00500940`), byte-swapped (`00414220`)
///   when the file needs it;
/// - `SCDA`: the compiled data, read into a stack buffer of the chunk's
///   size (cleared first), handed to `Script::SetCompileData`;
/// - `SCRO` and `SCRV`: a referenced-object entry (`00500910` on 0x10
///   bytes) with the chunk's word at +8 or +0xc, appended to the script's
///   list (`005d43c0`, `00905820`);
/// - `SLSD` and `SCVR`: when the current chunk (read again) is `SLSD`, a
///   new 0x20-byte variable (`005e28b0`) becomes `011d66ac`, is loaded
///   (`ScriptVariable::Load`) and appended to the script's variables
///   (`005d43e0`, `00905820` with the global's address); otherwise, with a
///   variable, its name (+0x18) is set from the chunk (at most 0x200 bytes,
///   into a stack buffer of the chunk's size).
///
/// After `TNAM`, `00484ab0` with 0 on the script and, when `00671d10` says
/// it is not kept, the script is deleted.
pub fn package_event_action_load(e: &mut Engine, this: Ptr<PackageEventAction>, file: Ptr) {
    e.set(this, PackageEventAction::pIdle, 0);
    e.set(this, PackageEventAction::pTopic, 0);
    if e.get(this, PackageEventAction::pResultScript) != 0 {
        drop_script(e, this);
    }
    let block = e.call(ALLOCATE, &args![SCRIPT_SIZE]).u32();
    let script = if block != 0 {
        e.call(SCRIPT_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.set(this, PackageEventAction::pResultScript, script);
    loop {
        let mut done = false;
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        match chunk {
            CHUNK_SCRO => read_referenced_object(e, this, file, 0x8),
            CHUNK_SCRV => read_referenced_object(e, this, file, 0xc),
            CHUNK_INAM => {
                let idle = e.with_stack(4, |e, word| {
                    e.call(GET_CHUNK_WORD, &args![file, word]);
                    e.mem.u32(word.addr())
                });
                e.set(this, PackageEventAction::pIdle, idle);
            }
            CHUNK_TNAM => {
                let topic = e.with_stack(4, |e, word| {
                    e.call(GET_CHUNK_WORD, &args![file, word]);
                    e.mem.u32(word.addr())
                });
                e.set(this, PackageEventAction::pTopic, topic);
                done = true;
            }
            CHUNK_SCDA => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(alloca_size(size), |e, buffer| {
                    e.call(FILL_MEMORY, &args![buffer, 0u32, size]);
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0u32]);
                    let script = e.get(this, PackageEventAction::pResultScript);
                    e.call(SET_COMPILE_DATA, &args![script, size, buffer]);
                });
            }
            CHUNK_SCHR => {
                let script = e.get(this, PackageEventAction::pResultScript);
                let header = e.call(SCRIPT_HEADER, &args![script]).u32();
                e.call(GET_CHUNK_DATA, &args![file, header, 0u32]);
                if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
                    let script = e.get(this, PackageEventAction::pResultScript);
                    let header = e.call(SCRIPT_HEADER, &args![script]).u32();
                    e.call(HEADER_SWAP, &args![header]);
                }
            }
            CHUNK_SLSD | CHUNK_SCVR => {
                if e.call(GET_TES_CHUNK, &args![file]).u32() == CHUNK_SLSD {
                    let block = e.call(ALLOCATE, &args![0x20u32]).u32();
                    let variable = if block != 0 {
                        e.call(VARIABLE_CONSTRUCT, &args![block]).u32()
                    } else {
                        0
                    };
                    e.mem.set_u32(CURRENT_VARIABLE, variable);
                    let current = e.mem.u32(CURRENT_VARIABLE);
                    e.call(VARIABLE_LOAD, &args![current, file]);
                    let script = e.get(this, PackageEventAction::pResultScript);
                    let list = e.call(SCRIPT_VARIABLES, &args![script]).u32();
                    e.call(LIST_APPEND, &args![list, CURRENT_VARIABLE]);
                } else if e.mem.u32(CURRENT_VARIABLE) != 0 {
                    let size = e.call(CHUNK_SIZE, &args![file]).u32();
                    e.with_stack(alloca_size(size), |e, buffer| {
                        e.call(GET_CHUNK_DATA, &args![file, buffer, NAME_MAXIMUM]);
                        let current = e.mem.u32(CURRENT_VARIABLE);
                        e.call(STRING_SET, &args![current + 0x18, buffer, 0u32]);
                    });
                }
            }
            _ => {}
        }
        if done {
            break;
        }
        e.call(NEXT_CHUNK, &args![file]);
    }
    let script = e.get(this, PackageEventAction::pResultScript);
    e.call(FORM_SET_484AB0, &args![script, 0u32]);
    let script = e.get(this, PackageEventAction::pResultScript);
    if e.call(SCRIPT_KEPT, &args![script]).u32() == 0 {
        drop_script(e, this);
    }
}

/// `InitItem`'s lookup of one form id (the idle or the topic): the id gets
/// the owner's file's compile index and is looked up and cast to
/// `target`. Found, it replaces the id. Not found, a nonzero id warns
/// (with the owner by its slot `0x130` when `00474cb0` says so, else by its
/// form id) and the field is cleared.
fn resolve(e: &mut Engine, field: u32, owner: Ptr, target: u32, named: u32, by_id: u32) {
    let id = e.mem.u32(field);
    let (found, id) = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), id);
        let file = e.call(GET_FILE, &args![owner, 0xffff_ffffu32]).u32();
        e.call(ADD_COMPILE_INDEX, &args![slot, file]);
        let id = e.mem.u32(slot.addr());
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let found = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
            )
            .u32();
        (found, id)
    });
    if found != 0 {
        e.mem.set_u32(field, found);
    } else if id != 0 {
        if e.call(OWNER_NAMED, &args![owner]).u32() != 0 {
            let name = e.vcall(owner.addr(), 0x130, &[]).u32();
            e.call(LOG_WARNING, &args![named, id, name]);
        } else {
            let owner_id = e.call(FORM_ID, &args![owner]).u32();
            e.call(LOG_WARNING, &args![by_id, id, owner_id]);
        }
        e.mem.set_u32(field, 0);
    }
}

// Translated from 0067e190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageEventAction::InitItem` (Xbox PDB; the owning package; `RET 4`):
/// resolves the idle (`TESIdleForm`) and the topic (`TESTopic`) form ids.
/// With a result script: the warning count (`0046e8a0`) is kept, warnings
/// are disabled (`0043b2b0` with 1), the count set to 0 (`004fffe0`), the
/// script made temporary and initialised (`Script::InitItem` with the
/// owner), the byte at +0x10 of its header (`00500940`) cleared, warnings
/// enabled again, and the count restored; when the script's initialisation
/// left warnings, one more names the package (its slot `0x130`) and its
/// form id.
pub fn package_event_action_init_item(e: &mut Engine, this: Ptr<PackageEventAction>, owner: Ptr) {
    let base = this.addr();
    resolve(
        e,
        base,
        owner,
        TYPE_TES_IDLE_FORM,
        FORMAT_NO_IDLE_NAMED,
        FORMAT_NO_IDLE,
    );
    resolve(
        e,
        base + 0x8,
        owner,
        TYPE_TES_TOPIC,
        FORMAT_NO_TOPIC_NAMED,
        FORMAT_NO_TOPIC,
    );
    if e.get(this, PackageEventAction::pResultScript) == 0 {
        return;
    }
    let kept = e.call(WARNING_COUNT, &[]).u32();
    e.call(DISABLE_WARNINGS, &args![1u32]);
    e.call(SET_WARNING_COUNT, &args![0u32]);
    let script = e.get(this, PackageEventAction::pResultScript);
    e.call(SET_TEMPORARY, &args![script]);
    let script = e.get(this, PackageEventAction::pResultScript);
    e.call(SCRIPT_INIT_ITEM, &args![script, owner]);
    let script = e.get(this, PackageEventAction::pResultScript);
    let header = e.call(SCRIPT_HEADER, &args![script]).u32();
    e.mem.set_u8(header + 0x10, 0);
    e.call(DISABLE_WARNINGS, &args![0u32]);
    let errors = e.call(WARNING_COUNT, &[]).u32();
    e.call(SET_WARNING_COUNT, &args![kept]);
    if errors != 0 {
        let id = e.call(FORM_ID, &args![owner]).u32();
        let name = e.vcall(owner.addr(), 0x130, &[]).u32();
        e.call(LOG_WARNING, &args![FORMAT_SCRIPT_ERRORS, name, id]);
    }
}

/// The other action's fields, through the folded getters the exe calls:
/// `00559450` (the word at +0), `0044ddc0` (the same) and `00726070` (the
/// word at +4), each `thiscall` with no stack word.
const OTHER_IDLE: u32 = 0x0055_9450;
const OTHER_TOPIC: u32 = 0x0044_ddc0;
const OTHER_SCRIPT: u32 = 0x0072_6070;
/// `Script::CompareResultScripts` (engine map, `005ab240`, `thiscall`, `RET
/// 4`) and `Script::SaveResultScript` (engine map, `005abe00`, `thiscall`).
const COMPARE_RESULT_SCRIPTS: u32 = 0x005a_b240;
const SAVE_RESULT_SCRIPT: u32 = 0x005a_be00;
/// `TESForm::AddChunk` (engine map, `004856d0`, `cdecl`: the type) and
/// `00485910` (`cdecl`: type and a word).
const ADD_CHUNK: u32 = 0x0048_56d0;
const ADD_CHUNK_WORD: u32 = 0x0048_5910;

// Translated from 0067dbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageEventAction::Compare` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): true (they differ) when the idles differ, the
/// topics differ (the other's read by `00559450` and `0044ddc0` against
/// +0 and +8), or exactly one has a result script, or both have and
/// `Script::CompareResultScripts` (`005ab240`) says they differ.
pub fn package_event_action_compare(
    e: &mut Engine,
    this: Ptr<PackageEventAction>,
    other: u32,
) -> bool {
    if e.get(this, PackageEventAction::pIdle) != e.call(OTHER_IDLE, &args![other]).u32() {
        return true;
    }
    if e.get(this, PackageEventAction::pTopic) != e.call(OTHER_TOPIC, &args![other]).u32() {
        return true;
    }
    if e.get(this, PackageEventAction::pResultScript) == 0
        && e.call(OTHER_SCRIPT, &args![other]).u32() == 0
    {
        return false;
    }
    if e.get(this, PackageEventAction::pResultScript) == 0
        || e.call(OTHER_SCRIPT, &args![other]).u32() == 0
    {
        return true;
    }
    let theirs = e.call(OTHER_SCRIPT, &args![other]).u32();
    let mine = e.get(this, PackageEventAction::pResultScript);
    e.call(COMPARE_RESULT_SCRIPTS, &args![mine, theirs]).bool()
}

// Translated from 0067dc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageEventAction::Save` (Xbox PDB name in the engine map; `thiscall`,
/// `RET 4`, the chunk type): an empty chunk of `kind` (`004856d0`), the
/// idle's id (`0084e3a0`, 0 without one) as `INAM`, the result script's
/// save (`005abe00`) when there is one, then the topic's id (taken before
/// the script's save) as `TNAM`.
pub fn package_event_action_save(e: &mut Engine, this: Ptr<PackageEventAction>, kind: u32) {
    e.call(ADD_CHUNK, &args![kind]);
    let idle = e.get(this, PackageEventAction::pIdle);
    let idle_id = if idle != 0 {
        e.call(FORM_ID, &args![idle]).u32()
    } else {
        0
    };
    e.call(ADD_CHUNK_WORD, &args![CHUNK_INAM, idle_id]);
    let topic = e.get(this, PackageEventAction::pTopic);
    let topic_id = if topic != 0 {
        e.call(FORM_ID, &args![topic]).u32()
    } else {
        0
    };
    let script = e.get(this, PackageEventAction::pResultScript);
    if script != 0 {
        e.call(SAVE_RESULT_SCRIPT, &args![script]);
    }
    e.call(ADD_CHUNK_WORD, &args![CHUNK_TNAM, topic_id]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0067_dbf0,
            package_event_action_compare(Ptr<PackageEventAction>, u32) -> bool
        ),
        entry!(
            0x0067_dc80,
            package_event_action_save(Ptr<PackageEventAction>, u32)
        ),
        entry!(
            0x0067_dd20,
            package_event_action_load(Ptr<PackageEventAction>, Ptr)
        ),
        entry!(
            0x0067_e190,
            package_event_action_init_item(Ptr<PackageEventAction>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> Log {
        e.call_log = Some(vec![]);
        e.call(addr, words);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect()
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A script object at a fresh address whose vtable (`0f00_0000`) slot
    /// `0x10` is the double `0f00_1010`.
    fn script(e: &mut Engine) -> u32 {
        let script = e.mem.alloc(SCRIPT_SIZE);
        e.mem.set_u32(script, 0x0f00_0000);
        script
    }

    /// The engine for `Load` with the chunks given; each chunk word read is
    /// 0x100 plus the chunk's position, each size 0x21.
    fn load_engine(chunks: Vec<u32>, kept: u32) -> Engine {
        let mut e = Engine::new();
        for addr in [
            FILL_MEMORY,
            GET_CHUNK_DATA,
            HEADER_SWAP,
            SET_COMPILE_DATA,
            LIST_APPEND,
            VARIABLE_LOAD,
            STRING_SET,
            FORM_SET_484AB0,
            0x0f00_1010,
        ] {
            e.register(addr, |_, _| ret(0));
        }
        e.put_vtable(0x0f00_0000, &[0, 0, 0, 0, 0x0f00_1010]);
        e.map(CURRENT_VARIABLE, 4);
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(SCRIPT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0f00_0000);
            ret(a[0])
        });
        e.register(REFERENCED_OBJECT_CONSTRUCT, |_, a| ret(a[0]));
        e.register(VARIABLE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(SCRIPT_REFERENCED_OBJECTS, |_, a| ret(a[0] + 0x100));
        e.register(SCRIPT_VARIABLES, |_, a| ret(a[0] + 0x200));
        e.register(SCRIPT_HEADER, |_, a| ret(a[0] + 0x300));
        e.register(CHUNK_SIZE, |_, _| ret(0x21));
        e.register(FILE_NEEDS_SWAP, |_, _| ret(1));
        e.register_double(SCRIPT_KEPT, move |_, _| ret(kept));
        let position = Rc::new(Cell::new(0usize));
        let (list, at) = (chunks, position.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(list.get(at.get()).copied().unwrap_or(0))
        });
        let next = position.clone();
        e.register_double(NEXT_CHUNK, move |_, _| {
            next.set(next.get() + 1);
            ret(1)
        });
        e.register_double(GET_CHUNK_WORD, move |e, a| {
            e.mem.set_u32(a[1], 0x100 + position.get() as u32);
            ret(0)
        });
        e
    }

    #[test]
    fn load_reads_every_chunk_until_tnam() {
        let chunks = vec![
            CHUNK_INAM, CHUNK_SCHR, CHUNK_SCDA, CHUNK_SCRO, CHUNK_SCRV, CHUNK_SCVR, CHUNK_SLSD,
            CHUNK_SCVR, 0x1234, CHUNK_TNAM, CHUNK_INAM,
        ];
        let mut e = load_engine(chunks, 1);
        let old = script(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, old);
        let log = run(&mut e, 0x0067_dd20, &[this, 0x7000]);
        assert_eq!(calls(&log, 0x0f00_1010), vec![vec![old, 1]]);
        let new = e.mem.u32(this + 4);
        assert_eq!(calls(&log, SCRIPT_CONSTRUCT), vec![vec![new]]);
        assert_eq!((e.mem.u32(this), e.mem.u32(this + 8)), (0x100, 0x109));
        assert_eq!(calls(&log, HEADER_SWAP), vec![vec![new + 0x300]]);
        let data = calls(&log, GET_CHUNK_DATA);
        assert_eq!(data[0], vec![0x7000, new + 0x300, 0]);
        assert_eq!(calls(&log, FILL_MEMORY), vec![vec![data[1][1], 0, 0x21]]);
        assert_eq!(data[1][2], 0);
        assert_eq!(
            calls(&log, SET_COMPILE_DATA),
            vec![vec![new, 0x21, data[1][1]]]
        );
        // `SCVR` before any variable reads nothing; `SLSD` makes one; the
        // `SCVR` after it names it.
        let variable = calls(&log, VARIABLE_CONSTRUCT)[0][0];
        assert_eq!(e.mem.u32(CURRENT_VARIABLE), variable);
        assert_eq!(calls(&log, VARIABLE_LOAD), vec![vec![variable, 0x7000]]);
        assert_eq!(data[2], vec![0x7000, data[2][1], NAME_MAXIMUM]);
        assert_eq!(
            calls(&log, STRING_SET),
            vec![vec![variable + 0x18, data[2][1], 0]]
        );
        let appends = calls(&log, LIST_APPEND);
        assert_eq!(appends.len(), 3);
        assert_eq!(appends[0][0], new + 0x100);
        assert_eq!(appends[1][0], new + 0x100);
        assert_eq!(appends[2], vec![new + 0x200, CURRENT_VARIABLE]);
        let entries = calls(&log, REFERENCED_OBJECT_CONSTRUCT);
        assert_eq!(e.mem.u32(entries[0][0] + 8), 0x103);
        assert_eq!(e.mem.u32(entries[1][0] + 0xc), 0x104);
        assert_eq!(calls(&log, FORM_SET_484AB0), vec![vec![new, 0]]);
        assert_eq!(e.mem.u32(this + 4), new);
        assert_eq!(calls(&log, NEXT_CHUNK).len(), 9);
    }

    #[test]
    fn load_drops_a_script_that_is_not_kept() {
        let mut e = load_engine(vec![CHUNK_TNAM], 0);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x55);
        let log = run(&mut e, 0x0067_dd20, &[this, 0x7000]);
        let new = calls(&log, SCRIPT_CONSTRUCT)[0][0];
        assert_eq!(calls(&log, 0x0f00_1010), vec![vec![new, 1]]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this), 0);
        assert!(calls(&log, NEXT_CHUNK).is_empty());
        // A failed allocation leaves no script to delete.
        e.register(ALLOCATE, |_, _| ret(0));
        let log = run(&mut e, 0x0067_dd20, &[this, 0x7000]);
        assert!(calls(&log, SCRIPT_CONSTRUCT).is_empty());
        assert_eq!(calls(&log, FORM_SET_484AB0), vec![vec![0, 0]]);
        assert!(calls(&log, 0x0f00_1010).is_empty());
    }

    /// The engine for `InitItem`: ids gain 0x0100_0000; ids ending in 1
    /// are found (as `id + 0x1000`).
    fn init_engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            LOG_WARNING,
            DISABLE_WARNINGS,
            SET_WARNING_COUNT,
            SET_TEMPORARY,
            SCRIPT_INIT_ITEM,
        ] {
            e.register(addr, |_, _| ret(0));
        }
        e.register(GET_FILE, |_, _| ret(0x7100));
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            if id != 0 {
                e.mem.set_u32(a[0], id | 0x0100_0000);
            }
            ret(0)
        });
        e.register(LOOKUP_FORM, |_, a| ret(a[0]));
        e.register(DYNAMIC_CAST, |_, a| {
            ret(if a[0] & 0xf == 1 { a[0] + 0x1000 } else { 0 })
        });
        e.register(FORM_ID, |_, _| ret(0x00ab_cdef));
        e.register(0x0f00_1130, |_, _| ret(0x6600));
        let mut slots = vec![0u32; 0x134 / 4];
        slots[0x130 / 4] = 0x0f00_1130;
        e.put_vtable(0x0f00_0000, &slots);
        e
    }

    #[test]
    fn init_item_resolves_and_warns() {
        let mut e = init_engine();
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, 0x0f00_0000);
        e.register(OWNER_NAMED, |_, _| ret(1));
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x21);
        e.mem.set_u32(this + 8, 0x32);
        let log = run(&mut e, 0x0067_e190, &[this, owner]);
        assert_eq!(
            calls(&log, DYNAMIC_CAST),
            vec![
                vec![0x0100_0021, 0, TYPE_TES_FORM, TYPE_TES_IDLE_FORM, 0],
                vec![0x0100_0032, 0, TYPE_TES_FORM, TYPE_TES_TOPIC, 0],
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0100_1021);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(
            calls(&log, LOG_WARNING),
            vec![vec![FORMAT_NO_TOPIC_NAMED, 0x0100_0032, 0x6600]]
        );
        assert!(calls(&log, WARNING_COUNT).is_empty());
        // By form id when `00474cb0` says no.
        e.register(OWNER_NAMED, |_, _| ret(0));
        e.mem.set_u32(this, 0x42);
        e.mem.set_u32(this + 8, 0x31);
        let log = run(&mut e, 0x0067_e190, &[this, owner]);
        assert_eq!(
            calls(&log, LOG_WARNING),
            vec![vec![FORMAT_NO_IDLE, 0x0100_0042, 0x00ab_cdef]]
        );
        assert_eq!(e.mem.u32(this + 8), 0x0100_1031);
    }

    #[test]
    fn init_item_initialises_the_result_script() {
        let mut e = init_engine();
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, 0x0f00_0000);
        let header = e.mem.alloc(0x14);
        e.mem.set_u8(header + 0x10, 0xff);
        e.register_double(SCRIPT_HEADER, move |_, _| ret(header));
        let counts = Rc::new(Cell::new(0u32));
        let seen = counts.clone();
        e.register_double(WARNING_COUNT, move |_, _| {
            seen.set(seen.get() + 1);
            ret(if seen.get() == 1 { 7 } else { 2 })
        });
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x5400);
        let log = run(&mut e, 0x0067_e190, &[this, owner]);
        // An id of 0 that is not found does not warn about itself.
        assert_eq!(calls(&log, DISABLE_WARNINGS), vec![vec![1], vec![0]]);
        assert_eq!(calls(&log, SET_WARNING_COUNT), vec![vec![0], vec![7]]);
        assert_eq!(calls(&log, SET_TEMPORARY), vec![vec![0x5400]]);
        assert_eq!(calls(&log, SCRIPT_INIT_ITEM), vec![vec![0x5400, owner]]);
        assert_eq!(e.mem.u8(header + 0x10), 0);
        assert_eq!(
            calls(&log, LOG_WARNING),
            vec![vec![FORMAT_SCRIPT_ERRORS, 0x6600, 0x00ab_cdef]]
        );
        assert_eq!(counts.get(), 2);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn compare_checks_idle_topic_and_scripts() {
        let mut e = Engine::new();
        e.mem.map(0x0f00_0000, 4);
        e.register(OTHER_IDLE, |e, a| word(e.mem.u32(a[0])));
        e.register(OTHER_TOPIC, |e, a| word(e.mem.u32(a[0] + 8)));
        e.register(OTHER_SCRIPT, |e, a| word(e.mem.u32(a[0] + 4)));
        e.register(COMPARE_RESULT_SCRIPTS, |e, _| word(e.mem.u32(0x0f00_0000)));
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        let compare = |e: &mut Engine| e.call(0x0067_dbf0, &args![this, other]).bool();
        assert!(!compare(&mut e));
        e.mem.set_u32(other, 1);
        assert!(compare(&mut e));
        e.mem.set_u32(other, 0);
        e.mem.set_u32(other + 8, 1);
        assert!(compare(&mut e));
        e.mem.set_u32(other + 8, 0);
        e.mem.set_u32(other + 4, 0x5000);
        assert!(compare(&mut e));
        e.mem.set_u32(this + 4, 0x6000);
        e.call_log = Some(vec![]);
        assert!(!compare(&mut e));
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.last().unwrap(),
            &(COMPARE_RESULT_SCRIPTS, vec![0x6000, 0x5000])
        );
        e.mem.set_u32(0x0f00_0000, 1);
        assert!(compare(&mut e));
        e.mem.set_u32(other + 4, 0);
        assert!(compare(&mut e));
    }

    #[test]
    fn save_writes_idle_script_and_topic() {
        let mut e = Engine::new();
        e.register(FORM_ID, |_, a| word(a[0] + 1));
        for addr in [ADD_CHUNK, ADD_CHUNK_WORD, SAVE_RESULT_SCRIPT] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0067_dc80, &args![this, 0x3144_5450u32]);
        let log: Vec<_> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (ADD_CHUNK, vec![0x3144_5450]),
                (ADD_CHUNK_WORD, vec![CHUNK_INAM, 0]),
                (ADD_CHUNK_WORD, vec![CHUNK_TNAM, 0]),
            ]
        );
        e.mem.set_u32(this, 0x5000);
        e.mem.set_u32(this + 4, 0x6000);
        e.mem.set_u32(this + 8, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x0067_dc80, &args![this, 0x3144_5450u32]);
        let log: Vec<_> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (ADD_CHUNK, vec![0x3144_5450]),
                (FORM_ID, vec![0x5000]),
                (ADD_CHUNK_WORD, vec![CHUNK_INAM, 0x5001]),
                (FORM_ID, vec![0x7000]),
                (SAVE_RESULT_SCRIPT, vec![0x6000]),
                (ADD_CHUNK_WORD, vec![CHUNK_TNAM, 0x7001]),
            ]
        );
    }
}
