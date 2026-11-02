//! `fallout shared/bgsquestobjective.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A quest objective's record handling: its loader (`QOBJ` index, `NNAM`
//! display text), save, `InitItem` (owner quest and targets), the field
//! reset its constructor runs, and the display-text accessors. The
//! constructor itself (`005ec090`) is in `run_005ec040`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleList, BSStringT};

layout! {
    /// `BGSQuestObjective` (Xbox PDB), 0x24 bytes; the PC code uses the same
    /// offsets (vtable `01047088`, whose RTTI names `BGSQuestObjective`).
    pub struct BGSQuestObjective: 0x24 {
        /// `iIndex` (Xbox PDB): the objective number (`QOBJ`).
        0x04 iIndex: u32,
        /// `cDisplayText` (Xbox PDB): `BSStringT<char>` (`NNAM`).
        0x08 cDisplayText: Inline<BSStringT>,
        /// `pOwnerQuest` (Xbox PDB): `TESQuest*`.
        0x10 pOwnerQuest: u32,
        /// `ListTargets` (Xbox PDB): `BSSimpleList<TESQuestTarget *>`.
        0x14 ListTargets: Inline<BSSimpleList>,
        /// `bInitialized` (Xbox PDB).
        0x1C bInitialized: u8,
        /// `eState` (Xbox PDB): `QUEST_OBJECTIVE_STATE`.
        0x20 eState: u32,
    }
}

/// The chunk tags: `QOBJ` (the index) and `NNAM` (the display text).
const CHUNK_QOBJ: u32 = 0x4a42_4f51;
const CHUNK_NNAM: u32 = 0x4d41_4e4e;
/// The game's empty string (`01011584`, a zero byte).
const EMPTY_STRING: u32 = 0x0101_1584;

/// `BSStringT<char>` text assignment (`004037f0`, `ECX` = string; text,
/// minimum capacity; `RET 8`).
const STRING_SET: u32 = 0x0040_37f0;
/// `00438390` (`ECX` = string, one stack word: the text): runs
/// `004037f0(text, 0)`.
const STRING_CONSTRUCT_FROM: u32 = 0x0043_8390;
/// `004048e0` (`ECX` = string): the string's length.
const STRING_LENGTH: u32 = 0x0040_48e0;
/// `00559450` (`ECX` = string): the word at `+0`, the text pointer.
const STRING_TEXT: u32 = 0x0055_9450;
/// `00726070` (`ECX` = object): the word at `+4` (here `iIndex`; on a list
/// node, the next node).
const WORD_AT_4: u32 = 0x0072_6070;
/// `007d6bb0` (`ECX` = object): `this + 0x14`, the target list.
const TARGET_LIST: u32 = 0x007d_6bb0;
/// `008256d0` (`ECX` = node): true for the empty end node.
const NODE_IS_END: u32 = 0x0082_56d0;
/// `006815c0` (`ECX` = node): returns `this`, the address of the node's
/// item word.
const NODE_ITEM_SLOT: u32 = 0x0068_15c0;
/// `TESQuestTarget::InitItem` (`00610040`, `ECX` = target, one stack word:
/// the quest).
const TARGET_INIT_ITEM: u32 = 0x0061_0040;
/// `00483710` (`ECX` = target, no stack words): called for each target
/// while saving.
const TARGET_SAVE: u32 = 0x0048_3710;
/// A chunk holding one 32-bit value (`00485910`, cdecl: tag, value).
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// `TESForm::AddChunkArray` (`004856f0`, cdecl: tag, data, size).
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
/// `TESFile::GetTESChunk` (`004726b0`, `ECX` = file): the current chunk's
/// tag, 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `TESFile::GetChunkData` (`004727f0`, `ECX` = file, one stack word: the
/// destination of a 4-byte value).
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `00401660` (`ECX` = file): the word at `file + 0x25c`, which this loader
/// uses as the current chunk's data size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (`00472890`, `ECX` = file; destination,
/// size; `RET 8`). Called here with size 0.
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `004726f0` (`ECX` = file): moves to the next chunk; false at the end.
const NEXT_CHUNK: u32 = 0x0047_26f0;

// Translated from 005ec1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the objective (`ECX`, no stack words; unnamed in the map, run by
/// the constructor): `iIndex` 0, the display text set to the empty string
/// (`00438390`), `bInitialized` false, `eState` 0 and `pOwnerQuest` null.
pub fn fn_005ec1b0(e: &mut Engine, this: Ptr<BGSQuestObjective>) {
    e.set(this, BGSQuestObjective::iIndex, 0);
    let text = this.at(BGSQuestObjective::cDisplayText);
    e.call(STRING_CONSTRUCT_FROM, &args![text, EMPTY_STRING]);
    e.set(this, BGSQuestObjective::bInitialized, 0);
    e.set(this, BGSQuestObjective::eState, 0);
    e.set(this, BGSQuestObjective::pOwnerQuest, 0);
}

// Translated from 005ec210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The objective's `InitItem` (unnamed in the map; `ECX` = objective, one
/// stack word: the quest, `RET 4`): once (`bInitialized`), stores the quest
/// as `pOwnerQuest` and runs `TESQuestTarget::InitItem` (`00610040`) with
/// it on every target of the list, until a null node or the empty end
/// node.
pub fn fn_005ec210(e: &mut Engine, this: Ptr<BGSQuestObjective>, quest: u32) {
    if e.get(this, BGSQuestObjective::bInitialized) != 0 {
        return;
    }
    e.set(this, BGSQuestObjective::pOwnerQuest, quest);
    let mut node = e.call(TARGET_LIST, &args![this]).u32();
    while node != 0 && !e.call(NODE_IS_END, &args![node]).bool() {
        let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
        let target = e.mem.u32(slot);
        e.call(TARGET_INIT_ITEM, &args![target, quest]);
        node = e.call(WORD_AT_4, &args![node]).u32();
    }
    e.set(this, BGSQuestObjective::bInitialized, 1);
}

// Translated from 005ec280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSQuestObjective::Save` (Xbox PDB; `ECX`, no stack words): writes a
/// `QOBJ` chunk with the index (`00726070` reads it), an `NNAM` chunk with
/// the display text and its terminating zero (length + 1 bytes), then calls
/// `00483710` on every target.
pub fn bgs_quest_objective_save(e: &mut Engine, this: Ptr<BGSQuestObjective>) {
    let index = e.call(WORD_AT_4, &args![this]).u32();
    e.call(ADD_CHUNK_WORD, &args![CHUNK_QOBJ, index]);
    let length = fn_005ec310(e, this);
    let text = fn_005ec330(e, this);
    e.call(
        ADD_CHUNK_ARRAY,
        &args![CHUNK_NNAM, text, length.wrapping_add(1)],
    );
    let mut node = e.call(TARGET_LIST, &args![this]).u32();
    while node != 0 && !e.call(NODE_IS_END, &args![node]).bool() {
        let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
        let target = e.mem.u32(slot);
        e.call(TARGET_SAVE, &args![target]);
        node = e.call(WORD_AT_4, &args![node]).u32();
    }
}

// Translated from 005ec310 (decompiled, FalloutNV.exe 1.4.0.525)
/// The display text's length (`004048e0` on `cDisplayText`; unnamed in the
/// map; `ECX`, no stack words).
pub fn fn_005ec310(e: &mut Engine, this: Ptr<BGSQuestObjective>) -> u32 {
    let text = this.at(BGSQuestObjective::cDisplayText);
    e.call(STRING_LENGTH, &args![text]).u32()
}

// Translated from 005ec330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The display text (unnamed in the map; `ECX`, no stack words): the
/// string's text pointer (`00559450`, read twice as in the exe), or the
/// game's empty string (`01011584`) when it is null.
pub fn fn_005ec330(e: &mut Engine, this: Ptr<BGSQuestObjective>) -> u32 {
    let text = this.at(BGSQuestObjective::cDisplayText);
    if e.call(STRING_TEXT, &args![text]).u32() == 0 {
        EMPTY_STRING
    } else {
        e.call(STRING_TEXT, &args![text]).u32()
    }
}

// Translated from 005ec370 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ec370 - the security-cookie check (00ec408c) is not translated.
/// `BGSQuestObjective::Load` (Xbox PDB; `ECX`, one stack word: the file,
/// `RET 4`): reads chunks until the file has none left. `QOBJ` reads the
/// index into `iIndex` (`004727f0`) and goes on to the next chunk; `NNAM`
/// ends the loop after setting the display text (`005ec440`): with a
/// nonzero data size (`00401660`) the data is read (`00472890`, size
/// argument 0) into a stack buffer of that size (`_alloca_probe_16`) and
/// that buffer is the text, otherwise the text is null. Any other chunk is
/// skipped. The loop also ends when `004726f0` answers false.
///
/// The alloca block is a fresh block of the data size (the exe's is
/// 16-byte aligned on its stack; the block's contents are what
/// `00472890` writes).
pub fn bgs_quest_objective_load(e: &mut Engine, this: Ptr<BGSQuestObjective>, file: Ptr) {
    loop {
        let tag = e.call(GET_TES_CHUNK, &args![file]).u32();
        if tag == 0 {
            return;
        }
        if tag == CHUNK_QOBJ {
            // The address of `iIndex` (+4).
            let index = this.byte_add(4);
            e.call(GET_CHUNK_WORD, &args![file, index]);
        } else if tag == CHUNK_NNAM {
            if e.call(CHUNK_SIZE, &args![file]).u32() != 0 {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0u32]);
                    fn_005ec440(e, this, buffer.addr());
                });
            } else {
                fn_005ec440(e, this, 0);
            }
            return;
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            return;
        }
    }
}

// Translated from 005ec440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the display text (unnamed in the map; `ECX`, one stack word: the C
/// string, `RET 4`): `004037f0(text, 0)` on `cDisplayText`.
pub fn fn_005ec440(e: &mut Engine, this: Ptr<BGSQuestObjective>, text: u32) {
    let string = this.at(BGSQuestObjective::cDisplayText);
    e.call(STRING_SET, &args![string, text, 0u32]);
}

// Translated from 005ec1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `005ec460`, which walks a list and deletes its
/// items, and returns nothing. Parameters: one word, `this`; the disassembly of the function
/// ends in a plain `RET` and passes only `ECX` on. `005ec460`'s disassembly is a loop that
/// ends on its exit test without loading a result (Ghidra types it void).
pub fn fn_005ec1f0(e: &mut Engine, this: Ptr) {
    e.call(0x005e_c460, &args![this]);
}

// Translated from 005ec110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSQuestObjective` scalar deleting destructor (engine map name): runs the destructor body `005ec140` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn bgs_quest_objective_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x005e_c140, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `005ec1f0` (`thiscall`, no stack word; unit `bgsquestobjective.cpp`).
const OBJECTIVE_CLEAR: u32 = 0x005e_c1f0;
/// `0046ffb0` (`thiscall`, no stack word): the target list's destructor.
const TARGET_LIST_DESTRUCTOR: u32 = 0x0046_ffb0;
/// `004037d0` (`thiscall`, no stack word): the `BSStringT` destructor.
const STRING_DESTRUCTOR: u32 = 0x0040_37d0;
/// `005ec4d0` (`thiscall`, `RET 4`): run with 1 on each target.
const TARGET_DELETE: u32 = 0x005e_c4d0;
/// `0063f7b0` (`thiscall`, no stack word): run on the list after each target.
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;

// Translated from 005ec140 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ec140 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00efa316, state variable) is not translated
/// Unnamed in the engine map (unit `bgsquestobjective.cpp`, `thiscall`, no stack word): the
/// `BGSQuestObjective` destructor. Sets the vtable `01047088`, calls `005ec1f0` on `this`,
/// destroys `ListTargets` (+0x14, `0046ffb0`) and `cDisplayText` (+8, `004037d0`).
pub fn fn_005ec140(e: &mut Engine, this: Ptr<BGSQuestObjective>) {
    e.mem.set_u32(this.addr(), 0x0104_7088);
    e.call(OBJECTIVE_CLEAR, &args![this]);
    e.call(TARGET_LIST_DESTRUCTOR, &args![this.addr() + 0x14]);
    e.call(STRING_DESTRUCTOR, &args![this.addr() + 8]);
}

// Translated from 005ec460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bgsquestobjective.cpp`, `thiscall`, no stack word):
/// empties the objective's target list (`007d6bb0`): while the list is not null and its node
/// is not the end (`008256d0`), the target in the node (`006815c0`) is deleted
/// (`005ec4d0(1)`, skipped for null) and `0063f7b0` runs on the list.
pub fn fn_005ec460(e: &mut Engine, this: Ptr<BGSQuestObjective>) {
    let list = e.call(TARGET_LIST, &args![this]).u32();
    while list != 0 && !e.call(NODE_IS_END, &args![list]).bool() {
        let slot = e.call(NODE_ITEM_SLOT, &args![list]).u32();
        let target = e.mem.u32(slot);
        if target != 0 {
            e.call(TARGET_DELETE, &args![target, 1u32]);
        }
        e.call(LIST_REMOVE_HEAD, &args![list]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_c1f0, fn_005ec1f0(Ptr)),
        entry!(0x005ec1b0, fn_005ec1b0(Ptr<BGSQuestObjective>)),
        entry!(0x005ec210, fn_005ec210(Ptr<BGSQuestObjective>, u32)),
        entry!(0x005ec280, bgs_quest_objective_save(Ptr<BGSQuestObjective>)),
        entry!(0x005ec310, fn_005ec310(Ptr<BGSQuestObjective>) -> u32),
        entry!(0x005ec330, fn_005ec330(Ptr<BGSQuestObjective>) -> u32),
        entry!(
            0x005ec370,
            bgs_quest_objective_load(Ptr<BGSQuestObjective>, Ptr)
        ),
        entry!(0x005ec440, fn_005ec440(Ptr<BGSQuestObjective>, u32)),
        entry!(0x005e_c110, bgs_quest_objective_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x005e_c140, fn_005ec140(Ptr<BGSQuestObjective>)),
        entry!(0x005e_c460, fn_005ec460(Ptr<BGSQuestObjective>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Doubles for every callee outside this file. A string's text pointer
    /// is its first word and its length its second; the target list is at
    /// `+0x14` and a node is `{item, next}`.
    fn setup() -> Engine {
        let mut e = Engine::new();
        e.register(STRING_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(1)
        });
        e.register(STRING_CONSTRUCT_FROM, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(STRING_LENGTH, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(STRING_TEXT, |e, a| ret(e.mem.u32(a[0])));
        e.register(WORD_AT_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(TARGET_LIST, |_, a| ret(a[0] + 0x14));
        e.register(NODE_IS_END, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(NODE_ITEM_SLOT, |_, a| ret(a[0]));
        e.register(TARGET_INIT_ITEM, |_, _| ret(0));
        e.register(TARGET_SAVE, |_, _| ret(0));
        e.register(ADD_CHUNK_WORD, |_, _| ret(0));
        e.register(ADD_CHUNK_ARRAY, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e
    }

    /// An objective with two targets (0x31, 0x32) in its list.
    fn objective(e: &mut Engine) -> Ptr<BGSQuestObjective> {
        let this = e.new_object::<BGSQuestObjective>();
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x32);
        e.mem.set_u32(this.addr() + 0x14, 0x31);
        e.mem.set_u32(this.addr() + 0x18, second);
        this
    }

    #[test]
    fn reset_clears_the_fields_and_empties_the_text() {
        let mut e = setup();
        let this = e.new_object::<BGSQuestObjective>();
        for offset in [0x04, 0x10, 0x1c, 0x20] {
            e.mem.set_u32(this.addr() + offset, 0xffff_ffff);
        }
        e.call(0x005e_c1b0, &args![this]);
        assert_eq!(e.get(this, BGSQuestObjective::iIndex), 0);
        assert_eq!(e.get(this, BGSQuestObjective::pOwnerQuest), 0);
        assert_eq!(e.get(this, BGSQuestObjective::bInitialized), 0);
        assert_eq!(e.get(this, BGSQuestObjective::eState), 0);
        // Only the flag byte of the word at +0x1c is written.
        assert_eq!(e.mem.u32(this.addr() + 0x1c), 0xffff_ff00);
        assert_eq!(
            calls_to(&e, STRING_CONSTRUCT_FROM),
            vec![vec![this.addr() + 8, EMPTY_STRING]]
        );
    }

    #[test]
    fn init_item_runs_once_and_initializes_every_target() {
        let mut e = setup();
        let this = objective(&mut e);
        e.call(0x005e_c210, &args![this, 0x900u32]);
        assert_eq!(e.get(this, BGSQuestObjective::pOwnerQuest), 0x900);
        assert_eq!(e.get(this, BGSQuestObjective::bInitialized), 1);
        assert_eq!(
            calls_to(&e, TARGET_INIT_ITEM),
            vec![vec![0x31, 0x900], vec![0x32, 0x900]]
        );
        // A second call does nothing.
        e.call_log = Some(vec![]);
        e.call(0x005e_c210, &args![this, 0x901u32]);
        assert_eq!(e.get(this, BGSQuestObjective::pOwnerQuest), 0x900);
        assert!(calls_to(&e, TARGET_INIT_ITEM).is_empty());
    }

    #[test]
    fn save_writes_the_index_and_the_text_with_its_zero() {
        let mut e = setup();
        let this = objective(&mut e);
        e.set(this, BGSQuestObjective::iIndex, 20);
        e.mem.set_u32(this.addr() + 8, 0x5000);
        e.mem.set_u32(this.addr() + 0xc, 7);
        e.call(0x005e_c280, &args![this]);
        assert_eq!(calls_to(&e, ADD_CHUNK_WORD), vec![vec![CHUNK_QOBJ, 20]]);
        assert_eq!(
            calls_to(&e, ADD_CHUNK_ARRAY),
            vec![vec![CHUNK_NNAM, 0x5000, 8]]
        );
        assert_eq!(calls_to(&e, TARGET_SAVE), vec![vec![0x31], vec![0x32]]);
    }

    #[test]
    fn the_text_falls_back_to_the_empty_string() {
        let mut e = setup();
        let this = e.new_object::<BGSQuestObjective>();
        assert_eq!(e.call(0x005e_c330, &args![this]).u32(), EMPTY_STRING);
        e.mem.set_u32(this.addr() + 8, 0x5000);
        assert_eq!(e.call(0x005e_c330, &args![this]).u32(), 0x5000);
        e.mem.set_u32(this.addr() + 0xc, 3);
        assert_eq!(e.call(0x005e_c310, &args![this]).u32(), 3);
    }

    #[test]
    fn set_text_assigns_with_no_minimum_capacity() {
        let mut e = setup();
        let this = e.new_object::<BGSQuestObjective>();
        e.call(0x005e_c440, &args![this, 0x6000u32]);
        assert_eq!(
            calls_to(&e, STRING_SET),
            vec![vec![this.addr() + 8, 0x6000, 0]]
        );
    }

    /// A file whose chunks are given as `(tag, size, word)`: the current
    /// chunk index is the file's first word; the size sits at `+0x25c` as
    /// `00401660` reads it.
    fn file(e: &mut Engine, chunks: &[(u32, u32, u32)]) -> Ptr {
        let f = e.mem.alloc(0x300);
        let table = e.mem.alloc(12 * chunks.len() as u32 + 12);
        for (i, &(tag, size, word)) in chunks.iter().enumerate() {
            let at = table + 12 * i as u32;
            e.mem.set_u32(at, tag);
            e.mem.set_u32(at + 4, size);
            e.mem.set_u32(at + 8, word);
        }
        e.mem.set_u32(f + 4, table);
        e.mem.set_u32(f + 8, chunks.len() as u32);
        Ptr::new(f)
    }

    fn current(e: &Engine, f: u32) -> u32 {
        e.mem.u32(f + 4) + 12 * e.mem.u32(f)
    }

    fn file_doubles(e: &mut Engine) {
        e.register(GET_TES_CHUNK, |e, a| {
            if e.mem.u32(a[0]) >= e.mem.u32(a[0] + 8) {
                return ret(0);
            }
            ret(e.mem.u32(current(e, a[0])))
        });
        e.register(CHUNK_SIZE, |e, a| {
            let size = e.mem.u32(current(e, a[0]) + 4);
            e.mem.set_u32(a[0] + 0x25c, size);
            ret(e.mem.u32(a[0] + 0x25c))
        });
        e.register(GET_CHUNK_WORD, |e, a| {
            let word = e.mem.u32(current(e, a[0]) + 8);
            e.mem.set_u32(a[1], word);
            ret(1)
        });
        e.register(GET_CHUNK_DATA, |e, a| {
            let word = e.mem.u32(current(e, a[0]) + 8);
            e.mem.set_u32(a[1], word);
            ret(1)
        });
        e.register(NEXT_CHUNK, |e, a| {
            let next = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], next);
            ret((next < e.mem.u32(a[0] + 8)) as u32)
        });
        // The text assignment records the first word of the text it gets.
        e.register(STRING_SET, |e, a| {
            let first = if a[1] == 0 { 0xdead } else { e.mem.u32(a[1]) };
            e.mem.set_u32(a[0], first);
            ret(1)
        });
    }

    #[test]
    fn load_reads_the_index_skips_other_chunks_and_stops_after_the_text() {
        let mut e = setup();
        file_doubles(&mut e);
        let this = e.new_object::<BGSQuestObjective>();
        let f = file(
            &mut e,
            &[
                (CHUNK_QOBJ, 4, 12),
                (0x4154_4451, 4, 0),
                (CHUNK_NNAM, 6, 0x6968),
                (CHUNK_QOBJ, 4, 99),
            ],
        );
        e.call(0x005e_c370, &args![this, f]);
        assert_eq!(e.get(this, BGSQuestObjective::iIndex), 12);
        assert_eq!(e.mem.u32(this.addr() + 8), 0x6968);
        let reads = calls_to(&e, GET_CHUNK_DATA);
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0][2], 0);
        // The loop ended on NNAM: the last QOBJ was never read.
        assert_eq!(calls_to(&e, NEXT_CHUNK).len(), 2);
    }

    #[test]
    fn load_with_an_empty_text_sets_a_null_text() {
        let mut e = setup();
        file_doubles(&mut e);
        let this = e.new_object::<BGSQuestObjective>();
        let f = file(&mut e, &[(CHUNK_NNAM, 0, 0)]);
        e.call(0x005e_c370, &args![this, f]);
        assert!(calls_to(&e, GET_CHUNK_DATA).is_empty());
        assert_eq!(calls_to(&e, STRING_SET), vec![vec![this.addr() + 8, 0, 0]]);
    }

    #[test]
    fn load_stops_at_the_end_of_the_file() {
        let mut e = setup();
        file_doubles(&mut e);
        let this = e.new_object::<BGSQuestObjective>();
        let f = file(&mut e, &[(CHUNK_QOBJ, 4, 5)]);
        e.call(0x005e_c370, &args![this, f]);
        assert_eq!(e.get(this, BGSQuestObjective::iIndex), 5);
        assert!(calls_to(&e, STRING_SET).is_empty());
        // No chunks at all: nothing is read.
        let empty = file(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x005e_c370, &args![this, empty]);
        assert!(calls_to(&e, NEXT_CHUNK).is_empty());
    }
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_005ec1f0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x005e_c460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005e_c1f0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x005e_c460), vec![vec![0x5000]]);
    }
    #[test]
    fn at_005ec110_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_c140, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_c110, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_c110)
                .collect();
            let mut expected = vec![(0x005e_c140, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn fn_005ec140_destroys_the_members() {
        let mut e = Engine::new();
        for callee in [OBJECTIVE_CLEAR, TARGET_LIST_DESTRUCTOR, STRING_DESTRUCTOR] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        e.call(0x005e_c140, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (OBJECTIVE_CLEAR, vec![this]),
                (TARGET_LIST_DESTRUCTOR, vec![this + 0x14]),
                (STRING_DESTRUCTOR, vec![this + 8]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0104_7088);
    }

    #[test]
    fn fn_005ec460_deletes_each_target_until_the_list_is_empty() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        let slot = e.mem.alloc(4);
        let left = Rc::new(Cell::new(3u32));
        let (empty, slots, remove) = (left.clone(), left.clone(), left.clone());
        e.register(TARGET_LIST, |_, a| (a[0] + 0x14).into_ret());
        e.register_double(NODE_IS_END, move |_, _| (empty.get() == 0).into_ret());
        // The second target slot holds null.
        e.register_double(NODE_ITEM_SLOT, move |e, _| {
            let target = if slots.get() == 2 {
                0
            } else {
                0x7000 + slots.get()
            };
            e.mem.set_u32(slot, target);
            slot.into_ret()
        });
        e.register_double(LIST_REMOVE_HEAD, move |_, _| {
            remove.set(remove.get() - 1);
            Ret::default()
        });
        e.register(TARGET_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005e_c460, &args![this]);
        let log = e.call_log.take().unwrap();
        let deleted: Vec<_> = log
            .iter()
            .filter(|c| c.0 == TARGET_DELETE)
            .cloned()
            .collect();
        assert_eq!(
            deleted,
            [
                (TARGET_DELETE, vec![0x7003, 1]),
                (TARGET_DELETE, vec![0x7001, 1]),
            ]
        );
        assert_eq!(left.get(), 0);
        assert_eq!(log.iter().filter(|c| c.0 == LIST_REMOVE_HEAD).count(), 3);
    }
}
