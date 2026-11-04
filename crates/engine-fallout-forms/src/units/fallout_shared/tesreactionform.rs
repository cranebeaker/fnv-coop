//! `fallout shared/tesreactionform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0048baf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0048bb10` (Ghidra name
/// `TESReactionForm::FreeReactionList`), which empties the reaction list node by node. The
/// Xbox PDB name `TESReactionForm::ClearDataComponent` is the class's vslot +0x004 (pdbdump
/// `type ... TESReactionForm`; engine map row `vt 822dd550`), and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `0048bb10`'s disassembly is a loop that ends on its exit test
/// without loading a result (Ghidra types it void).
pub fn tes_reaction_form_clear_data_component(e: &mut Engine, this: Ptr) {
    e.call(0x0048_bb10, &args![this]);
}

/// `00717e50` (`ECX` = form): the address of the reaction list (`this + 4`;
/// the engine map's name for this folded body is
/// `BaseProcess::GetActorPackageThatIsRunning`).
const REACTION_LIST: u32 = 0x0071_7e50;
/// `006815c0` (`ECX` = list node): the address of the node's item.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`ECX` = list node): the next node, or 0.
const NODE_NEXT: u32 = 0x0072_6070;
/// `0063f7b0` (`ECX` = list node): removes the node's item.
const NODE_REMOVE: u32 = 0x0063_f7b0;
/// `00905330` (`ECX` = list or node, the address of an item): removes that
/// item.
const LIST_REMOVE: u32 = 0x0090_5330;
/// `00905820` (`ECX` = list, the address of an item): adds the item.
const LIST_ADD: u32 = 0x0090_5820;
/// `00401000` (`cdecl`, size): the game's allocation.
const ALLOCATE: u32 = 0x0040_1000;
/// `00401030` (`cdecl`, block): the game's free.
const FREE: u32 = 0x0040_1030;
/// `"MASTERFILE: Form (%08X) not found for reaction of pForm (%08X) '%s'."`
const MISSING_REACTION_FORM: u32 = 0x0101_d480;

/// The address of the reaction list, through `00717e50` as the exe gets it.
fn reaction_list(e: &mut Engine, this: Ptr) -> u32 {
    e.call(REACTION_LIST, &args![this]).u32()
}

/// `*006815c0(node)`: the node's item.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let at = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(at)
}

// Translated from 0048bad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESReactionForm::InitializeDataComponent` (Xbox PDB): the byte at +0xC
/// becomes 8.
pub fn tes_reaction_form_initialize_data_component(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0xc, 8);
}

// Translated from 0048bb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESReactionForm::FreeReactionList` (Xbox PDB): while the list's first
/// item is not null, frees it (`00401030`) and removes it from the list
/// (`0063f7b0` on the list).
pub fn tes_reaction_form_free_reaction_list(e: &mut Engine, this: Ptr) {
    loop {
        let list = reaction_list(e, this);
        if node_item(e, list) == 0 {
            break;
        }
        let list = reaction_list(e, this);
        let item = node_item(e, list);
        e.call(FREE, &args![item]);
        let list = reaction_list(e, this);
        e.call(NODE_REMOVE, &args![list]);
    }
}

// Translated from 0048bc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESReactionForm::InitItem` (Xbox PDB): turns each reaction's form id
/// (the item's first word, given the compile index of `owner`'s file,
/// `TESForm::GetFile(-1)`, or of no file when `owner` is null) into the
/// form (`004839c0`). A missing form is reported ("MASTERFILE: Form (%08X)
/// not found for reaction of pForm (%08X) '%s'.", with the id, `owner`'s
/// form id and name, vtable slot 0x130) and its item removed and freed:
/// through `0063f7b0` on the node when the node has a next node or no
/// earlier node is known, otherwise through `00905330` on the earlier node
/// (with the address of the item), the walk going on from that node's
/// next. The exe never records an earlier node (its local stays 0), so the
/// second way is not taken; it is translated as the exe has it.
pub fn tes_reaction_form_init_item(e: &mut Engine, this: Ptr, owner: Ptr) {
    let mut node = reaction_list(e, this);
    let previous = 0u32;
    let file = if owner.is_null() {
        0
    } else {
        e.call(0x0048_4e60, &args![owner, u32::MAX]).u32()
    };
    e.with_stack(8, |e, locals| {
        let id_at = locals.addr();
        let item_at = locals.addr() + 4;
        while node != 0 && node_item(e, node) != 0 {
            let item = node_item(e, node);
            e.mem.set_u32(item_at, item);
            let id = e.mem.u32(item);
            e.mem.set_u32(id_at, id);
            e.call(0x0048_5d50, &args![id_at, file]);
            let id = e.mem.u32(id_at);
            let form = e.call(0x0048_39c0, &args![id]).u32();
            e.mem.set_u32(item, form);
            if e.mem.u32(item) != 0 {
                node = e.call(NODE_NEXT, &args![node]).u32();
                continue;
            }
            let name = e.vcall(owner.addr(), 0x130, &[]).u32();
            let owner_id = e.call(0x0084_e3a0, &args![owner]).u32();
            let id = e.mem.u32(id_at);
            e.call(
                0x005b_5e40,
                &args![MISSING_REACTION_FORM, id, owner_id, name],
            );
            let has_next = e.call(NODE_NEXT, &args![node]).u32() != 0;
            if has_next || previous == 0 {
                e.call(NODE_REMOVE, &args![node]);
            } else {
                e.call(LIST_REMOVE, &args![previous, item_at]);
                node = e.call(NODE_NEXT, &args![previous]).u32();
            }
            let freed = e.mem.u32(item_at);
            e.call(FREE, &args![freed]);
        }
    });
}

/// The reaction entry for `form` in the list, or 0.
fn find_reaction(e: &mut Engine, this: Ptr, form: u32) -> u32 {
    let mut node = reaction_list(e, this);
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        if e.mem.u32(item) == form {
            return item;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    0
}

// Translated from 0048bf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The second word (+4, the modifier) of the reaction to `form`, 0 when
/// `form` is null or has no reaction.
pub fn fn_0048bf50(e: &mut Engine, this: Ptr, form: u32) -> u32 {
    if form == 0 {
        return 0;
    }
    match find_reaction(e, this, form) {
        0 => 0,
        item => e.mem.u32(item + 4),
    }
}

// Translated from 0048c1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The third word (+8) of the reaction to `form`, 0 when `form` is null or
/// has no reaction.
pub fn fn_0048c1b0(e: &mut Engine, this: Ptr, form: u32) -> u32 {
    if form == 0 {
        return 0;
    }
    match find_reaction(e, this, form) {
        0 => 0,
        item => e.mem.u32(item + 8),
    }
}

/// A new 0xC-byte reaction (`00401000`; the exe does not check for null)
/// filled by `fill`, added to the list (`00905820` with the address of a
/// stack copy of the pointer).
fn add_reaction(e: &mut Engine, this: Ptr, fill: impl FnOnce(&mut Engine, u32)) {
    let item = e.call(ALLOCATE, &args![0xcu32]).u32();
    fill(e, item);
    e.with_stack(4, |e, item_at| {
        e.mem.set_u32(item_at.addr(), item);
        let list = reaction_list(e, this);
        e.call(LIST_ADD, &args![list, item_at]);
    });
}

// Translated from 0048bfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the modifier (+4) of the reaction to `form` (nothing for a null
/// `form`). An existing reaction gets `modifier`, unless `modifier` and
/// its +8 word are both 0: then it is removed from the list (`00905330` on
/// `this + 4`, with the address of a stack copy) and freed. Without one, a
/// nonzero `modifier` adds a reaction {`form`, `modifier`} (its +8 word not
/// written).
pub fn fn_0048bfc0(e: &mut Engine, this: Ptr, form: u32, modifier: u32) {
    if form == 0 {
        return;
    }
    let item = find_reaction(e, this, form);
    if item != 0 {
        if modifier == 0 && e.mem.u32(item + 8) == 0 {
            e.with_stack(4, |e, item_at| {
                e.mem.set_u32(item_at.addr(), item);
                e.call(LIST_REMOVE, &args![this.addr() + 4, item_at]);
                let freed = e.mem.u32(item_at.addr());
                e.call(FREE, &args![freed]);
            });
        } else {
            e.mem.set_u32(item + 4, modifier);
        }
        return;
    }
    if modifier != 0 {
        add_reaction(e, this, |e, item| {
            e.mem.set_u32(item, form);
            e.mem.set_u32(item + 4, modifier);
        });
    }
}

// Translated from 0048c220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the third word (+8) of the reaction to `form` (nothing for a null
/// `form`): an existing reaction gets `value`; without one a nonzero
/// `value` adds a reaction {`form`, 0, `value`}.
pub fn fn_0048c220(e: &mut Engine, this: Ptr, form: u32, value: u32) {
    if form == 0 {
        return;
    }
    let item = find_reaction(e, this, form);
    if item != 0 {
        e.mem.set_u32(item + 8, value);
        return;
    }
    if value != 0 {
        add_reaction(e, this, |e, item| {
            e.mem.set_u32(item, form);
            e.mem.set_u32(item + 4, 0);
            e.mem.set_u32(item + 8, value);
        });
    }
}

// Translated from 0048c9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the reactions into the save-game buffer `buffer` when bit 2 of its
/// flags word is set (`00428110` copies the word at +0x17 to a stack word,
/// `004280f0` tests it with 4): a variable-sized value
/// (`BGSSaveGameBuffer::StartVariableSizedValue`, `00865f20`), then for
/// each non-null item its form (`SaveFormID_ov2`, `00865df0`, with 0) and
/// its two words (`00865e50`, 4 bytes each, with 0), and finally the count
/// (`SaveVariableSizedValue_ov2`, `00865ff0`, with the count and the
/// start). The walk starts at `this + 4` itself.
pub fn fn_0048c9d0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let flagged = e.with_stack(4, |e, flags_at| {
        let flags = e.call(0x0042_8110, &args![buffer, flags_at]).u32();
        e.call(0x0042_80f0, &args![flags, 4u32]).bool()
    });
    if !flagged {
        return;
    }
    let mut count = 0u32;
    let start = e.call(0x0086_5f20, &args![buffer]).u32();
    let mut node = this.addr() + 4;
    while node != 0 {
        let item = node_item(e, node);
        if item != 0 {
            let form = e.mem.u32(item);
            e.call(0x0086_5df0, &args![buffer, form, 0u32]);
            e.call(0x0086_5e50, &args![buffer, item + 4, 4u32, 0u32]);
            e.call(0x0086_5e50, &args![buffer, item + 8, 4u32, 0u32]);
            count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(0x0086_5ff0, &args![buffer, count, start]);
}

/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and the type
/// descriptors `BaseFormComponent` (`01183040`) and `TESReactionForm`
/// (`01186b30`, name `.?AVTESReactionForm@@` read at +8).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_TES_REACTION_FORM: u32 = 0x0118_6b30;
/// `008256d0` (`ECX` = list node): whether the node holds no item.
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `005ae3d0` (`ECX` = list, the address of an item, `RET 4`): adds the item.
const LIST_ADD_ITEM: u32 = 0x005a_e3d0;
/// `005ae380` (`ECX` = list node): the number of nodes with an item.
const LIST_COUNT: u32 = 0x005a_e380;
/// `00440d80` (`ECX` = form): whether bit 0x20 of its flags word (+8) is set.
const FORM_FLAG_20: u32 = 0x0044_0d80;
/// The form-id getter (`0084e3a0`, `ECX` = form: the word at +0xc).
const FORM_ID: u32 = 0x0084_e3a0;
/// `00401500`: the byte at `011c54ba` (whether saved words are byte-swapped);
/// `005a6b10` (`ECX` = three words): byte-swaps each of them.
const SWAP_BYTES: u32 = 0x0040_1500;
const SWAP_THREE_WORDS: u32 = 0x005a_6b10;
/// `TESForm::__AddChunkData` (`00485990`, cdecl(chunk, data, size)) and the
/// chunk tag `XNAM`.
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
const CHUNK_XNAM: u32 = u32::from_le_bytes(*b"XNAM");
/// The form lookup by id (`004839c0`, cdecl).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// The save/load singleton (the word at `011de45c`) and its members
/// (`thiscall`): `UseSaveGameBlocks` (`00862110`), the buffer position
/// (`00825c00`), raw write (`008579b0`) and read (`008579e0`) of
/// `(address, size)`, `SaveNumericID` (`00857a10`) and `LoadNumericID`
/// (`00857aa0`) of `(address, size)`, the version (`008df040`, a byte), the
/// record of the form being saved (`004fd3e0`) and loaded (`004fd3c0`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
const SAVE_LOAD_POSITION: u32 = 0x0082_5c00;
const SAVE_LOAD_WRITE: u32 = 0x0085_79b0;
const SAVE_LOAD_READ: u32 = 0x0085_79e0;
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
const SAVE_LOAD_VERSION: u32 = 0x008d_f040;
const SAVED_FORM_RECORD: u32 = 0x004f_d3e0;
const LOADED_FORM_RECORD: u32 = 0x004f_d3c0;
/// The setting-like object at `011de4e8` whose byte (through `00408d60`)
/// turns on the size reports.
const SIZE_REPORT_SETTING: u32 = 0x011d_e4e8;
const SETTING_BYTE_ADDRESS: u32 = 0x0040_8d60;
/// `Error` (engine map, `0040fbe0`; cdecl, varargs) and the debug print
/// (`005b5e40`; cdecl, varargs).
const ERROR_REPORT: u32 = 0x0040_fbe0;
const DEBUG_PRINT: u32 = 0x005b_5e40;
/// A form's vtable slot `+0x130` (no stack words): its name.
const FORM_NAME_SLOT: u32 = 0x130;
/// The save-game block marker (`"KOLB"` as a word).
const BLOCK_MARKER: u32 = 0x424c_4f4b;
/// The reports' source file (`"D:\_Fallout3\...\TESReactionForm.cpp"`) and
/// line numbers.
const REACTION_FORM_SOURCE_FILE: u32 = 0x0101_d4c8;
const GET_SAVE_SIZE_LINE: u32 = 0x557;
const SAVE_GAME_LINE: u32 = 0x57b;
const LOAD_HEADER_LINE: u32 = 0x587;
const LOAD_SIZE_LINE: u32 = 0x599;
/// The report formats: `GetSaveSize()` with and without the form
/// (`01012cb0`, `01012c78`), `SaveGame()` (`010153a0`, `0101536c`), the
/// block size warning (`01015318`), the bad block header (`01015718`,
/// `010156a8`), the overrun (`01015588`, `010154a0`) and underrun
/// (`01015500`, `01015440`).
const GET_SAVE_SIZE_FORM_FORMAT: u32 = 0x0101_2cb0;
const GET_SAVE_SIZE_FORMAT: u32 = 0x0101_2c78;
const SAVE_GAME_FORM_FORMAT: u32 = 0x0101_53a0;
const SAVE_GAME_FORMAT: u32 = 0x0101_536c;
const BLOCK_TOO_BIG_FORMAT: u32 = 0x0101_5318;
const BAD_HEADER_FORM_FORMAT: u32 = 0x0101_5718;
const BAD_HEADER_FORMAT: u32 = 0x0101_56a8;
const OVERRUN_FORM_FORMAT: u32 = 0x0101_5588;
const OVERRUN_FORMAT: u32 = 0x0101_54a0;
const UNDERRUN_FORM_FORMAT: u32 = 0x0101_5500;
const UNDERRUN_FORMAT: u32 = 0x0101_5440;
/// `BGSLoadGameBuffer` members (engine map; `thiscall` on the buffer):
/// `LoadFormID_ov2` (`008648e0`; the address), the raw load (`00864980`;
/// address, size) and `LoadVariableSizedValue` (`00864a60`); `00428110`
/// (`RET 4`: an out word) gives the buffer's flags, which `004280f0`
/// (`RET 4`) tests with a mask.
const BUFFER_LOAD_FORM_ID_INTO: u32 = 0x0086_48e0;
const BUFFER_LOAD_BYTES: u32 = 0x0086_4980;
const BUFFER_LOAD_VARIABLE_SIZE: u32 = 0x0086_4a60;
const BUFFER_FLAGS: u32 = 0x0042_8110;
const FLAGS_HAVE: u32 = 0x0042_80f0;

fn save_load_object(e: &Engine) -> u32 {
    e.global::<u32>(SAVE_LOAD_GAME)
}

fn uses_save_game_blocks(e: &mut Engine) -> bool {
    let save = save_load_object(e);
    e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool()
}

fn save_load_position(e: &mut Engine) -> u32 {
    let save = save_load_object(e);
    e.call(SAVE_LOAD_POSITION, &args![save]).u32()
}

fn save_load_version(e: &mut Engine) -> u32 {
    let save = save_load_object(e);
    u32::from(e.call(SAVE_LOAD_VERSION, &args![save]).u8())
}

fn save_load_write(e: &mut Engine, address: u32, size: u32) {
    let save = save_load_object(e);
    e.call(SAVE_LOAD_WRITE, &args![save, address, size]);
}

fn save_load_read(e: &mut Engine, address: u32, size: u32) {
    let save = save_load_object(e);
    e.call(SAVE_LOAD_READ, &args![save, address, size]);
}

/// Whether the size reports are on (the byte at `00408d60(011de4e8)`).
fn size_reports_on(e: &mut Engine) -> bool {
    let at = e
        .call(SETTING_BYTE_ADDRESS, &args![SIZE_REPORT_SETTING])
        .u32();
    e.mem.u8(at) != 0
}

/// The size reports through `Error`: with the record of the form being
/// saved (`004fd3e0`) `form_format` with the size, the record's form id, the
/// form's name (no null check on the form) and the word at `+5`, else
/// `format` with the size; both with `line` and the source file.
fn report_saved_size(e: &mut Engine, (form_format, format): (u32, u32), size: u32, line: u32) {
    let save = save_load_object(e);
    let record = e.call(SAVED_FORM_RECORD, &args![save]).u32();
    if record != 0 {
        let id = e.mem.u32(record);
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let word = e.mem.u32(record + 5);
        let name = e.vcall(form, FORM_NAME_SLOT, &[]).u32();
        let id = e.mem.u32(record);
        e.call(
            ERROR_REPORT,
            &args![
                form_format,
                size,
                id,
                name,
                word,
                line,
                REACTION_FORM_SOURCE_FILE
            ],
        );
    } else {
        e.call(
            ERROR_REPORT,
            &args![format, size, line, REACTION_FORM_SOURCE_FILE],
        );
    }
}

/// `other` as a `TESReactionForm` (`__RTDynamicCast` from
/// `BaseFormComponent`), or 0.
fn as_reaction_form(e: &mut Engine, other: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![
            other,
            0u32,
            TYPE_BASE_FORM_COMPONENT,
            TYPE_TES_REACTION_FORM,
            0u32
        ],
    )
    .u32()
}

// Translated from 0048bb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reaction form's save (engine map: no name; no stack words): for each
/// reaction until the first null item, unless its form has flag 0x20
/// (`00440d80`), adds chunk `XNAM` of three words (the form's id, the
/// reaction's +4 and +8 words) from a stack copy (`TESForm::__AddChunkData`),
/// byte-swapping the copy before and after (`005a6b10`) when `00401500`
/// says so.
pub fn fn_0048bb70(e: &mut Engine, this: Ptr) {
    let mut node = reaction_list(e, this);
    e.with_stack(0xc, |e, record| {
        while node != 0 && node_item(e, node) != 0 {
            let item = node_item(e, node);
            let form = e.mem.u32(item);
            if !e.call(FORM_FLAG_20, &args![form]).bool() {
                let form = e.mem.u32(item);
                let id = e.call(FORM_ID, &args![form]).u32();
                e.mem.set_u32(record.addr(), id);
                let modifier = e.mem.u32(item + 4);
                e.mem.set_u32(record.addr() + 4, modifier);
                let third = e.mem.u32(item + 8);
                e.mem.set_u32(record.addr() + 8, third);
                if e.call(SWAP_BYTES, &[]).u32() as u8 != 0 {
                    e.call(SWAP_THREE_WORDS, &args![record]);
                }
                e.call(ADD_CHUNK_DATA, &args![CHUNK_XNAM, record, 0xcu32]);
                if e.call(SWAP_BYTES, &[]).u32() as u8 != 0 {
                    e.call(SWAP_THREE_WORDS, &args![record]);
                }
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
}

// Translated from 0048bd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESReactionForm::CopyComponent` (engine map, `RET 4`): when `other` is a
/// `TESReactionForm`, frees this reaction list (`FreeReactionList`) and
/// adds a copy of each of the other's reactions, until its first null item
/// (a new 0xc-byte block, not checked for null, `00905820` on the current
/// destination node with the address of a stack copy of the pointer); after
/// each add the destination moves on to its next node when there is one.
pub fn tes_reaction_form_copy_component(e: &mut Engine, this: Ptr, other: u32) {
    let source = as_reaction_form(e, other);
    if source == 0 {
        return;
    }
    tes_reaction_form_free_reaction_list(e, this);
    let mut target = reaction_list(e, this);
    let mut node = reaction_list(e, Ptr::new(source));
    e.with_stack(4, |e, copy_at| {
        while node != 0 && node_item(e, node) != 0 {
            let item = node_item(e, node);
            let copy = e.call(ALLOCATE, &args![0xcu32]).u32();
            e.mem.set_u32(copy_at.addr(), copy);
            for offset in [0u32, 4, 8] {
                let word = e.mem.u32(item + offset);
                e.mem.set_u32(copy + offset, word);
            }
            e.call(LIST_ADD, &args![target, copy_at]);
            if e.call(NODE_NEXT, &args![target]).u32() != 0 {
                target = e.call(NODE_NEXT, &args![target]).u32();
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
}

// Translated from 0048be40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESReactionForm::CompareComponent` (engine map, `RET 4`): true when the
/// two differ: when `other` is no `TESReactionForm`; when, walking both
/// lists side by side, the other runs out first, one item is null and the
/// other not, or two items differ in any of their three words; when after
/// this list ends the other's current node still holds an item (`008256d0`).
pub fn tes_reaction_form_compare_component(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let source = as_reaction_form(e, other);
    if source == 0 {
        return true;
    }
    let mut mine = reaction_list(e, this);
    let mut theirs = reaction_list(e, Ptr::new(source));
    while mine != 0 {
        if theirs == 0 {
            return true;
        }
        let a = node_item(e, mine);
        let b = node_item(e, theirs);
        if a != 0 && b != 0 {
            for offset in [0u32, 4, 8] {
                if e.mem.u32(a + offset) != e.mem.u32(b + offset) {
                    return true;
                }
            }
        } else if a != 0 || b != 0 {
            return true;
        }
        theirs = e.call(NODE_NEXT, &args![theirs]).u32();
        mine = e.call(NODE_NEXT, &args![mine]).u32();
    }
    theirs != 0 && !e.call(NODE_IS_EMPTY, &args![theirs]).bool()
}

// Translated from 0048c0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `delta` to the modifier (+4) of the reaction to `form` (`RET 8`;
/// nothing when `form` or `delta` is 0). Without a reaction, adds a new one
/// {`form`, `delta`} (its +8 word not written; `00905820` with the address
/// of a stack copy). When the sum is 0, the reaction is removed (`00905330`
/// on the list, with the address of a stack copy) and freed.
pub fn fn_0048c0b0(e: &mut Engine, this: Ptr, form: u32, delta: u32) {
    if form == 0 || delta == 0 {
        return;
    }
    let found = find_reaction(e, this, form);
    e.with_stack(4, |e, item_at| {
        if found == 0 {
            let item = e.call(ALLOCATE, &args![0xcu32]).u32();
            e.mem.set_u32(item_at.addr(), item);
            e.mem.set_u32(item, form);
            e.mem.set_u32(item + 4, delta);
            let list = reaction_list(e, this);
            e.call(LIST_ADD, &args![list, item_at]);
            return;
        }
        e.mem.set_u32(item_at.addr(), found);
        let sum = e.mem.u32(found + 4).wrapping_add(delta);
        e.mem.set_u32(found + 4, sum);
        if e.mem.u32(found + 4) == 0 {
            let list = reaction_list(e, this);
            e.call(LIST_REMOVE, &args![list, item_at]);
            let freed = e.mem.u32(item_at.addr());
            e.call(FREE, &args![freed]);
        }
    });
}

// Translated from 0048c2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reaction form's save-game size (engine map: no name; `RET 4`): with
/// bit 2 of `flags`, 4 + 2 for the block header when the save uses blocks,
/// 2 for the count and 8 per reaction (`005ae380`), all in 16 bits; with
/// the size reports on, `Error` reports it (line 0x557). 0 without the bit.
pub fn fn_0048c2e0(e: &mut Engine, this: Ptr, flags: u32) -> u16 {
    let mut size: u16 = 0;
    if flags & 4 == 0 {
        return size;
    }
    let start = size;
    if uses_save_game_blocks(e) {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    let list = reaction_list(e, this);
    size = size.wrapping_add(2);
    let count = e.call(LIST_COUNT, &args![list]).u32();
    size = (u32::from(size).wrapping_add(count.wrapping_mul(8))) as u16;
    if size_reports_on(e) {
        let reported = u32::from(size).wrapping_sub(u32::from(start));
        report_saved_size(
            e,
            (GET_SAVE_SIZE_FORM_FORMAT, GET_SAVE_SIZE_FORMAT),
            reported,
            GET_SAVE_SIZE_LINE,
        );
    }
    size
}

// Translated from 0048c400 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reaction form's save (engine map: no name; `RET 4`): with bit 2 of
/// `flags`: a block header when the save uses blocks (the marker and a
/// 16-bit size slot, its position kept), a 16-bit count slot, then for each
/// node with an item its form's id (`SaveNumericID`, 4 bytes) and its +4
/// word (4 bytes); the count goes into its slot (a 16-bit store at the
/// position the buffer gave). With the size reports on, `Error` reports
/// the bytes written (line 0x57b). With blocks, the size slot gets the
/// bytes after it (16 bits), after a warning (`005b5e40`) when they exceed
/// 0xffff. The words the game keeps on its stack are a block here.
pub fn fn_0048c400(e: &mut Engine, this: Ptr, flags: u32) {
    if flags & 4 == 0 {
        return;
    }
    e.with_stack(0x10, |e, locals| {
        let size_at = locals.addr();
        let marker_at = locals.addr() + 4;
        let count_at = locals.addr() + 8;
        let id_at = locals.addr() + 0xc;
        e.mem.set_u16(size_at, 0);
        let mut size_slot = 0u32;
        let mut start = save_load_position(e);
        if size_reports_on(e) {
            start = save_load_position(e);
        }
        if uses_save_game_blocks(e) {
            e.mem.set_u32(marker_at, BLOCK_MARKER);
            save_load_write(e, marker_at, 4);
            size_slot = save_load_position(e);
            save_load_write(e, size_at, 2);
        }
        e.mem.set_u16(count_at, 0);
        let count_slot = save_load_position(e);
        save_load_write(e, count_at, 2);
        let mut node = reaction_list(e, this);
        while node != 0 {
            let item = node_item(e, node);
            if item != 0 {
                let form = e.mem.u32(item);
                let id = e.call(FORM_ID, &args![form]).u32();
                e.mem.set_u32(id_at, id);
                let save = save_load_object(e);
                e.call(SAVE_NUMERIC_ID, &args![save, id_at, 4u32]);
                save_load_write(e, item + 4, 4);
                let count = e.mem.u16(count_at).wrapping_add(1);
                e.mem.set_u16(count_at, count);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        let count = e.mem.u16(count_at);
        e.mem.set_u16(count_slot, count);
        if size_reports_on(e) {
            let now = save_load_position(e);
            report_saved_size(
                e,
                (SAVE_GAME_FORM_FORMAT, SAVE_GAME_FORMAT),
                now.wrapping_sub(start),
                SAVE_GAME_LINE,
            );
        }
        if uses_save_game_blocks(e) {
            let now = save_load_position(e);
            let after = size_slot;
            if now > after.wrapping_add(0xffff) {
                e.call(
                    DEBUG_PRINT,
                    &args![
                        BLOCK_TOO_BIG_FORMAT,
                        REACTION_FORM_SOURCE_FILE,
                        SAVE_GAME_LINE
                    ],
                );
            }
            e.mem.set_u16(size_slot, now.wrapping_sub(after) as u16);
        }
    });
}

/// A load report through `005b5e40` with the record of the form being
/// loaded (`004fd3c0`) and its `form` (looked up by the caller): the
/// `leading` words, then the record's form id, the form's name (vtable slot
/// `+0x130`, no null check on the form), the record's byte at +9 and its
/// word at +5.
fn report_loaded_form(e: &mut Engine, record: u32, form: u32, leading: &[u32]) {
    let word = e.mem.u32(record + 5);
    let flag = u32::from(e.mem.u8(record + 9));
    let name = e.vcall(form, FORM_NAME_SLOT, &[]).u32();
    let id = e.mem.u32(record);
    let mut words = leading.to_vec();
    words.extend_from_slice(&[id, name, flag, word]);
    e.call(DEBUG_PRINT, &words);
}

// Translated from 0048c640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESReactionForm::LoadGame` (engine map; `RET 8`, the second word is not
/// read): with bit 2 of `flags`: when the save uses blocks, reads the block
/// marker (a wrong one is reported through `005b5e40`, line 0x587, with the
/// loading form's record or the save's version) and the 16-bit block size
/// (its position kept); frees the reaction list (`FreeReactionList`); reads
/// a 16-bit count and that many reactions (`LoadNumericID` of the form, 4
/// bytes of modifier) and sets each (`fn_0048bfc0`). With blocks, a read
/// position past or short of the block's end is reported (overrun,
/// underrun; line 0x599). The words the game keeps on its stack are a
/// block here.
pub fn tes_reaction_form_load_game(e: &mut Engine, this: Ptr, flags: u32, _unused_2: u32) {
    if flags & 4 == 0 {
        return;
    }
    e.with_stack(0x14, |e, locals| {
        let size_at = locals.addr();
        let marker_at = locals.addr() + 4;
        let count_at = locals.addr() + 8;
        let id_at = locals.addr() + 0xc;
        let modifier_at = locals.addr() + 0x10;
        e.mem.set_u16(size_at, 0);
        let mut start = 0u32;
        if uses_save_game_blocks(e) {
            save_load_read(e, marker_at, 4);
            if e.mem.u32(marker_at) != BLOCK_MARKER {
                let save = save_load_object(e);
                let record = e.call(LOADED_FORM_RECORD, &args![save]).u32();
                if record != 0 {
                    let id = e.mem.u32(record);
                    let form = e.call(LOOKUP_FORM, &args![id]).u32();
                    report_loaded_form(
                        e,
                        record,
                        form,
                        &[
                            BAD_HEADER_FORM_FORMAT,
                            REACTION_FORM_SOURCE_FILE,
                            LOAD_HEADER_LINE,
                        ],
                    );
                } else {
                    let version = save_load_version(e);
                    e.call(
                        DEBUG_PRINT,
                        &args![
                            BAD_HEADER_FORMAT,
                            REACTION_FORM_SOURCE_FILE,
                            LOAD_HEADER_LINE,
                            version
                        ],
                    );
                }
            }
            start = save_load_position(e);
            save_load_read(e, size_at, 2);
        }
        tes_reaction_form_free_reaction_list(e, this);
        save_load_read(e, count_at, 2);
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(count_at)) {
            let save = save_load_object(e);
            e.call(LOAD_NUMERIC_ID, &args![save, id_at, 4u32]);
            save_load_read(e, modifier_at, 4);
            let form = e.mem.u32(id_at);
            let modifier = e.mem.u32(modifier_at);
            fn_0048bfc0(e, this, form, modifier);
            index += 1;
        }
        if !uses_save_game_blocks(e) {
            return;
        }
        let now = save_load_position(e);
        let save = save_load_object(e);
        let record = e.call(LOADED_FORM_RECORD, &args![save]).u32();
        let end = u32::from(e.mem.u16(size_at)).wrapping_add(start);
        if record != 0 {
            let id = e.mem.u32(record);
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            if now > end {
                report_loaded_form(
                    e,
                    record,
                    form,
                    &[
                        OVERRUN_FORM_FORMAT,
                        now.wrapping_sub(end),
                        REACTION_FORM_SOURCE_FILE,
                        LOAD_SIZE_LINE,
                    ],
                );
            } else if now < end {
                report_loaded_form(
                    e,
                    record,
                    form,
                    &[
                        UNDERRUN_FORM_FORMAT,
                        end.wrapping_sub(now),
                        REACTION_FORM_SOURCE_FILE,
                        LOAD_SIZE_LINE,
                    ],
                );
            }
        } else if now > end {
            let version = save_load_version(e);
            e.call(
                DEBUG_PRINT,
                &args![
                    OVERRUN_FORMAT,
                    now.wrapping_sub(end),
                    REACTION_FORM_SOURCE_FILE,
                    LOAD_SIZE_LINE,
                    version
                ],
            );
        } else if now < end {
            let version = save_load_version(e);
            e.call(
                DEBUG_PRINT,
                &args![
                    UNDERRUN_FORMAT,
                    end.wrapping_sub(now),
                    REACTION_FORM_SOURCE_FILE,
                    LOAD_SIZE_LINE,
                    version
                ],
            );
        }
    });
}

// Translated from 0048c930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves the loaded reactions (engine map: no name; `RET 8`, the second
/// word is not read): with bit 2 of `flags`, turns each item's form id into
/// its form (`004839c0`); an item whose form is missing is removed
/// (`0063f7b0` on its node, which is then looked at again) and freed.
pub fn fn_0048c930(e: &mut Engine, this: Ptr, flags: u32, _unused_2: u32) {
    if flags & 4 == 0 {
        return;
    }
    let mut node = reaction_list(e, this);
    while node != 0 {
        let item = node_item(e, node);
        if item != 0 {
            let id = e.mem.u32(item);
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            e.mem.set_u32(item, form);
            if e.mem.u32(item) == 0 {
                e.call(NODE_REMOVE, &args![node]);
                e.call(FREE, &args![item]);
            } else {
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
        } else {
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
}

// Translated from 0048ca90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESReactionForm::LoadGame_ov2` (engine map; `RET 4`, the load buffer):
/// when the buffer's flags (`00428110`) have bit 2 (`004280f0`), frees the
/// reaction list (`FreeReactionList`) and reads the count
/// (`LoadVariableSizedValue`) and that many reactions: a new 0xc-byte
/// block (not checked for null), its form (`LoadFormID_ov2` into +0) and
/// its two words (4 bytes each), added to the list at +4 (`005ae3d0` with
/// the address of a stack copy).
pub fn tes_reaction_form_load_game_ov2(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let flagged = e.with_stack(4, |e, flags_at| {
        let flags = e.call(BUFFER_FLAGS, &args![buffer, flags_at]).u32();
        e.call(FLAGS_HAVE, &args![flags, 4u32]).bool()
    });
    if !flagged {
        return;
    }
    tes_reaction_form_free_reaction_list(e, this);
    let count = e.call(BUFFER_LOAD_VARIABLE_SIZE, &args![buffer]).u32();
    e.with_stack(4, |e, item_at| {
        let mut index = 0u32;
        while index < count {
            let item = e.call(ALLOCATE, &args![0xcu32]).u32();
            e.mem.set_u32(item_at.addr(), item);
            e.call(BUFFER_LOAD_FORM_ID_INTO, &args![buffer, item]);
            e.call(BUFFER_LOAD_BYTES, &args![buffer, item + 4, 4u32]);
            e.call(BUFFER_LOAD_BYTES, &args![buffer, item + 8, 4u32]);
            e.call(LIST_ADD_ITEM, &args![this.addr() + 4, item_at]);
            index += 1;
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_baf0, tes_reaction_form_clear_data_component(Ptr)),
        entry!(
            0x0048_bad0,
            tes_reaction_form_initialize_data_component(Ptr)
        ),
        entry!(0x0048_bb10, tes_reaction_form_free_reaction_list(Ptr)),
        entry!(0x0048_bc30, tes_reaction_form_init_item(Ptr, Ptr)),
        entry!(0x0048_bf50, fn_0048bf50(Ptr, u32) -> u32),
        entry!(0x0048_c1b0, fn_0048c1b0(Ptr, u32) -> u32),
        entry!(0x0048_bfc0, fn_0048bfc0(Ptr, u32, u32)),
        entry!(0x0048_c220, fn_0048c220(Ptr, u32, u32)),
        entry!(0x0048_c9d0, fn_0048c9d0(Ptr, Ptr)),
        entry!(0x0048_bb70, fn_0048bb70(Ptr)),
        entry!(0x0048_bd50, tes_reaction_form_copy_component(Ptr, u32)),
        entry!(
            0x0048_be40,
            tes_reaction_form_compare_component(Ptr, u32) -> bool
        ),
        entry!(0x0048_c0b0, fn_0048c0b0(Ptr, u32, u32)),
        entry!(0x0048_c2e0, fn_0048c2e0(Ptr, u32) -> u16),
        entry!(0x0048_c400, fn_0048c400(Ptr, u32)),
        entry!(0x0048_c640, tes_reaction_form_load_game(Ptr, u32, u32)),
        entry!(0x0048_c930, fn_0048c930(Ptr, u32, u32)),
        entry!(0x0048_ca90, tes_reaction_form_load_game_ov2(Ptr, Ptr)),
    ]
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
    fn fwd_a03_0048baf0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0048_bb10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_baf0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0048_bb10), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod b020_tests {
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

    /// An engine whose list is a chain of (item, next) nodes, the first at
    /// `this + 4`; removing a node's item moves its successor into it.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(REACTION_LIST, |_, a| ret(a[0] + 4));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(NODE_REMOVE, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        for a in [FREE, LIST_REMOVE, LIST_ADD] {
            e.register(a, |_, _| Ret::default());
        }
        e
    }

    /// A form with a list of `items` (each a 0xC-byte reaction {form, a, b}).
    fn form_with(e: &mut Engine, items: &[[u32; 3]]) -> (Ptr, Vec<u32>) {
        let this = e.mem.alloc(0x10);
        let mut node = this + 4;
        let mut made = vec![];
        for (i, words) in items.iter().enumerate() {
            let item = e.mem.alloc(0xc);
            for (k, w) in words.iter().enumerate() {
                e.mem.set_u32(item + 4 * k as u32, *w);
            }
            e.mem.set_u32(node, item);
            made.push(item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        (Ptr::new(this), made)
    }

    #[test]
    fn b020_0048bad0_sets_the_byte() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x10);
        e.call(0x0048_bad0, &args![p]);
        assert_eq!(e.mem.u8(p + 0xc), 8);
    }

    #[test]
    fn b020_0048bb10_frees_every_item() {
        let mut e = engine();
        let (this, items) = form_with(&mut e, &[[1, 0, 0], [2, 0, 0]]);
        e.call_log = Some(vec![]);
        e.call(0x0048_bb10, &args![this]);
        assert_eq!(calls_to(&e, FREE), vec![vec![items[0]], vec![items[1]]]);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
    }

    #[test]
    fn b020_0048bc30_resolves_and_drops_missing_forms() {
        let mut e = engine();
        e.register(0x0048_4e60, |_, _| ret(0x77));
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| {
            ret(if a[0] & 0xf == 0 { 0 } else { a[0] + 0x10 })
        });
        e.register(0x0084_e3a0, |_, _| ret(0x0400_0001));
        e.register(0x0000_1130, |_, _| ret(0x5000));
        e.register(0x005b_5e40, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0; 0x50]);
        e.mem.set_u32(0x0150_0130, 0x0000_1130);
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner, 0x0150_0000);
        let (this, items) = form_with(&mut e, &[[0x11, 1, 0], [0x20, 2, 0], [0x31, 3, 0]]);
        e.call_log = Some(vec![]);
        e.call(0x0048_bc30, &args![this, owner]);
        assert_eq!(e.mem.u32(items[0]), 0x0100_0021);
        assert_eq!(e.mem.u32(items[2]), 0x0100_0041);
        assert_eq!(
            calls_to(&e, 0x005b_5e40),
            vec![vec![
                MISSING_REACTION_FORM,
                0x0100_0020,
                0x0400_0001,
                0x5000
            ]]
        );
        assert_eq!(calls_to(&e, FREE), vec![vec![items[1]]]);
        assert_eq!(calls_to(&e, NODE_REMOVE).len(), 1);
        assert!(calls_to(&e, LIST_REMOVE).is_empty());
        assert_eq!(calls_to(&e, 0x0048_5d50)[0][1], 0x77);
    }

    #[test]
    fn b020_0048bc30_without_owner_uses_no_file() {
        let mut e = engine();
        e.register(0x0048_5d50, |_, _| Ret::default());
        e.register(0x0048_39c0, |_, a| ret(a[0] + 1));
        let (this, _) = form_with(&mut e, &[[0x11, 1, 0]]);
        e.call_log = Some(vec![]);
        e.call(0x0048_bc30, &args![this, 0u32]);
        assert_eq!(calls_to(&e, 0x0048_5d50)[0][1], 0);
        assert!(calls_to(&e, 0x0048_4e60).is_empty());
    }

    #[test]
    fn b020_0048bf50_and_0048c1b0_read_the_words() {
        let mut e = engine();
        let (this, _) = form_with(&mut e, &[[0x11, 5, 6], [0x22, 7, 8]]);
        assert_eq!(e.call(0x0048_bf50, &args![this, 0x22u32]).u32(), 7);
        assert_eq!(e.call(0x0048_c1b0, &args![this, 0x22u32]).u32(), 8);
        assert_eq!(e.call(0x0048_bf50, &args![this, 0x11u32]).u32(), 5);
        assert_eq!(e.call(0x0048_bf50, &args![this, 0x33u32]).u32(), 0);
        assert_eq!(e.call(0x0048_c1b0, &args![this, 0u32]).u32(), 0);
    }

    #[test]
    fn b020_0048bfc0_sets_removes_and_adds() {
        let mut e = engine();
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        let (this, items) = form_with(&mut e, &[[0x11, 5, 0], [0x22, 7, 8]]);
        e.call_log = Some(vec![]);
        e.call(0x0048_bfc0, &args![this, 0x22u32, 9u32]);
        assert_eq!(e.mem.u32(items[1] + 4), 9);
        e.call(0x0048_bfc0, &args![this, 0x22u32, 0u32]);
        assert_eq!(e.mem.u32(items[1] + 4), 0);
        assert!(calls_to(&e, LIST_REMOVE).is_empty());
        e.call(0x0048_bfc0, &args![this, 0x11u32, 0u32]);
        let removed = calls_to(&e, LIST_REMOVE);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], this.addr() + 4);
        assert_eq!(calls_to(&e, FREE), vec![vec![items[0]]]);
        e.call(0x0048_bfc0, &args![this, 0x33u32, 0u32]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        e.call(0x0048_bfc0, &args![this, 0x33u32, 4u32]);
        let added = calls_to(&e, LIST_ADD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], this.addr() + 4);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0xc]]);
        e.call(0x0048_bfc0, &args![this, 0u32, 4u32]);
        assert_eq!(calls_to(&e, ALLOCATE).len(), 1);
    }

    #[test]
    fn b020_0048c220_sets_or_adds() {
        let mut e = engine();
        let fresh = e.mem.alloc(0xc);
        e.mem.set_u32(fresh + 4, 0xffff);
        e.register_double(ALLOCATE, move |_, _| ret(fresh));
        let (this, items) = form_with(&mut e, &[[0x11, 5, 6]]);
        e.call_log = Some(vec![]);
        e.call(0x0048_c220, &args![this, 0x11u32, 0u32]);
        assert_eq!(e.mem.u32(items[0] + 8), 0);
        e.call(0x0048_c220, &args![this, 0x44u32, 0u32]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        e.call(0x0048_c220, &args![this, 0x44u32, 3u32]);
        assert_eq!(
            [e.mem.u32(fresh), e.mem.u32(fresh + 4), e.mem.u32(fresh + 8)],
            [0x44, 0, 3]
        );
        assert_eq!(calls_to(&e, LIST_ADD).len(), 1);
    }

    #[test]
    fn b020_0048c9d0_saves_when_flagged() {
        let mut e = engine();
        e.register(0x0042_8110, |e, a| {
            let flags = e.mem.u32(a[0] + 0x17);
            e.mem.set_u32(a[1], flags);
            ret(a[1])
        });
        e.register(0x0042_80f0, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) & a[1] != 0))
        });
        e.register(0x0086_5f20, |_, _| ret(0x55));
        for a in [0x0086_5df0, 0x0086_5e50, 0x0086_5ff0] {
            e.register(a, |_, _| Ret::default());
        }
        let buffer = e.mem.alloc(0x40);
        let (this, items) = form_with(&mut e, &[[0x11, 5, 6], [0, 0, 0], [0x22, 7, 8]]);
        // The middle node's item pointer is made null.
        let second = e.mem.u32(this.addr() + 8);
        e.mem.set_u32(second, 0);
        e.call_log = Some(vec![]);
        e.call(0x0048_c9d0, &args![this, buffer]);
        assert!(calls_to(&e, 0x0086_5f20).is_empty());
        e.mem.set_u32(buffer + 0x17, 4);
        e.call(0x0048_c9d0, &args![this, buffer]);
        assert_eq!(
            calls_to(&e, 0x0086_5df0),
            vec![vec![buffer, 0x11, 0], vec![buffer, 0x22, 0]]
        );
        assert_eq!(
            calls_to(&e, 0x0086_5e50),
            vec![
                vec![buffer, items[0] + 4, 4, 0],
                vec![buffer, items[0] + 8, 4, 0],
                vec![buffer, items[2] + 4, 4, 0],
                vec![buffer, items[2] + 8, 4, 0],
            ]
        );
        assert_eq!(calls_to(&e, 0x0086_5ff0), vec![vec![buffer, 2, 0x55]]);
    }
}

#[cfg(test)]
mod save_load_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    /// List doubles (a node is two words: item, next; the reaction list of a
    /// form is its `+4` node) and the allocator.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(REACTION_LIST, |_, a| ret(a[0] + 4));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(NODE_IS_EMPTY, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(LIST_COUNT, |e, a| {
            let (mut node, mut n) = (a[0], 0);
            while node != 0 {
                n += u32::from(e.mem.u32(node) != 0);
                node = e.mem.u32(node + 4);
            }
            ret(n)
        });
        fn append(e: &mut Engine, a: &[u32]) -> Ret {
            let item = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) == 0 {
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
        }
        e.register(LIST_ADD, append);
        e.register(LIST_ADD_ITEM, append);
        e.register(NODE_REMOVE, |e, a| {
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
        e.register(LIST_REMOVE, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) == item {
                    let next = e.mem.u32(node + 4);
                    if next != 0 {
                        let (i, after) = (e.mem.u32(next), e.mem.u32(next + 4));
                        e.mem.set_u32(node, i);
                        e.mem.set_u32(node + 4, after);
                    } else {
                        e.mem.set_u32(node, 0);
                    }
                    break;
                }
                node = e.mem.u32(node + 4);
            }
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(FREE, |_, _| Ret::default());
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e
    }

    fn reaction(e: &mut Engine, form: u32, modifier: u32, third: u32) -> u32 {
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, form);
        e.mem.set_u32(item + 4, modifier);
        e.mem.set_u32(item + 8, third);
        item
    }

    /// A reaction form (0x10 bytes) whose list (+4) holds `items`.
    fn reactions(e: &mut Engine, items: &[u32]) -> u32 {
        let this = e.mem.alloc(0x10);
        let mut node = this + 4;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        this
    }

    fn items(e: &Engine, this: u32) -> Vec<[u32; 3]> {
        let mut out = vec![];
        let mut node = this + 4;
        while node != 0 && e.mem.u32(node) != 0 {
            let item = e.mem.u32(node);
            out.push([item, item + 4, item + 8].map(|a| e.mem.u32(a)));
            node = e.mem.u32(node + 4);
        }
        out
    }

    /// A form with id `id` and flags word `flags` (+8).
    fn form(e: &mut Engine, id: u32, flags: u32) -> u32 {
        let form = e.mem.alloc(0x18);
        e.mem.set_u32(form + 8, flags);
        e.mem.set_u32(form + 0xc, id);
        form
    }

    #[test]
    fn save_writes_one_chunk_per_kept_reaction() {
        let mut e = engine();
        e.register(FORM_FLAG_20, |e, a| {
            ret(u32::from(e.mem.u32(a[0] + 8) & 0x20 != 0))
        });
        e.register(SWAP_BYTES, |_, _| ret(0));
        let seen = e.mem.alloc(0x18);
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(seen + 4 * i, word);
            }
            Ret::default()
        });
        let kept = form(&mut e, 0x111, 0);
        let skipped = form(&mut e, 0x222, 0x20);
        let a = reaction(&mut e, kept, 5, 6);
        let b = reaction(&mut e, skipped, 7, 8);
        let this = reactions(&mut e, &[a, b]);
        let (_, log) = run(&mut e, 0x0048_bb70, &args![this]);
        let added = only(&log, ADD_CHUNK_DATA);
        assert_eq!(added.len(), 1);
        assert_eq!((added[0][0], added[0][2]), (CHUNK_XNAM, 0xc));
        assert_eq!([0, 4, 8].map(|o| e.mem.u32(seen + o)), [0x111, 5, 6]);
        assert!(only(&log, SWAP_THREE_WORDS).is_empty());
        e.register(SWAP_BYTES, |_, _| ret(1));
        e.register(SWAP_THREE_WORDS, |_, _| Ret::default());
        let (_, log) = run(&mut e, 0x0048_bb70, &args![this]);
        assert_eq!(only(&log, SWAP_THREE_WORDS).len(), 2);
    }

    #[test]
    fn copy_replaces_the_list_with_copies() {
        let mut e = engine();
        let old = reaction(&mut e, 0x10, 1, 1);
        let this = reactions(&mut e, &[old]);
        let a = reaction(&mut e, 0x20, 2, 3);
        let b = reaction(&mut e, 0x30, 4, 5);
        let other = reactions(&mut e, &[a, b]);
        e.call(0x0048_bd50, &args![this, other]);
        assert_eq!(items(&e, this), vec![[0x20, 2, 3], [0x30, 4, 5]]);
        assert_ne!(e.mem.u32(this + 4), a);
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        let keep = reactions(&mut e, &[old]);
        e.call(0x0048_bd50, &args![keep, other]);
        assert_eq!(e.mem.u32(keep + 4), old);
    }

    #[test]
    fn compare_reports_each_difference() {
        let mut e = engine();
        let a = reaction(&mut e, 0x20, 2, 3);
        let same = reaction(&mut e, 0x20, 2, 3);
        let b = reaction(&mut e, 0x30, 4, 5);
        let other_third = reaction(&mut e, 0x20, 2, 4);
        let differs = |e: &mut Engine, mine: &[u32], theirs: &[u32]| {
            let this = reactions(e, mine);
            let other = reactions(e, theirs);
            e.call(0x0048_be40, &args![this, other]).bool()
        };
        assert!(!differs(&mut e, &[a, b], &[same, b]));
        assert!(!differs(&mut e, &[0], &[0]));
        assert!(differs(&mut e, &[a], &[other_third]));
        assert!(differs(&mut e, &[a], &[0]));
        assert!(differs(&mut e, &[a, b], &[same]));
        assert!(differs(&mut e, &[a], &[same, b]));
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(differs(&mut e, &[a], &[same]));
    }

    #[test]
    fn change_adds_sums_and_removes() {
        let mut e = engine();
        let a = reaction(&mut e, 0x20, 2, 9);
        let this = reactions(&mut e, &[a]);
        e.call(0x0048_c0b0, &args![this, 0x20u32, 3u32]);
        assert_eq!(items(&e, this), vec![[0x20, 5, 9]]);
        e.call(0x0048_c0b0, &args![this, 0x40u32, 7u32]);
        assert_eq!(items(&e, this)[1][..2], [0x40, 7]);
        let (_, log) = run(&mut e, 0x0048_c0b0, &args![this, 0x20u32, (-5i32) as u32]);
        assert_eq!(only(&log, FREE), vec![vec![a]]);
        assert_eq!(items(&e, this).len(), 1);
        let (_, log) = run(&mut e, 0x0048_c0b0, &args![this, 0x40u32, 0u32]);
        assert_eq!(log.len(), 1);
        let (_, log) = run(&mut e, 0x0048_c0b0, &args![this, 0u32, 1u32]);
        assert_eq!(log.len(), 1);
    }

    /// The save-load doubles on an object at `SAVE`: `+0` the buffer
    /// position (a pointer), `+4` whether blocks are used, `+8` the size
    /// report byte, `+0xc` the form record.
    const SAVE: u32 = 0x0e00_0000;

    fn save_engine(blocks: bool, reports: bool, record: u32) -> (Engine, u32) {
        let mut e = engine();
        e.map(SAVE_LOAD_GAME & !0xfff, 0x1000);
        e.set_global::<u32>(SAVE_LOAD_GAME, SAVE);
        e.map(SAVE, 0x1000);
        let buffer = e.mem.alloc(0x200);
        e.mem.set_u32(SAVE, buffer);
        e.mem.set_u32(SAVE + 4, u32::from(blocks));
        e.mem.set_u8(SAVE + 8, u8::from(reports));
        e.mem.set_u32(SAVE + 0xc, record);
        e.register(SAVE_LOAD_POSITION, |e, a| ret(e.mem.u32(a[0])));
        e.register(USE_SAVE_GAME_BLOCKS, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(SETTING_BYTE_ADDRESS, |_, _| ret(SAVE + 8));
        e.register(SAVE_LOAD_VERSION, |_, _| ret(0x55));
        e.register(SAVED_FORM_RECORD, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(LOADED_FORM_RECORD, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        fn write(e: &mut Engine, a: &[u32]) -> Ret {
            let at = e.mem.u32(a[0]);
            for i in 0..a[2] {
                let b = e.mem.u8(a[1] + i);
                e.mem.set_u8(at + i, b);
            }
            e.mem.set_u32(a[0], at + a[2]);
            Ret::default()
        }
        fn read(e: &mut Engine, a: &[u32]) -> Ret {
            let at = e.mem.u32(a[0]);
            for i in 0..a[2] {
                let b = e.mem.u8(at + i);
                e.mem.set_u8(a[1] + i, b);
            }
            e.mem.set_u32(a[0], at + a[2]);
            Ret::default()
        }
        e.register(SAVE_LOAD_WRITE, write);
        e.register(SAVE_NUMERIC_ID, write);
        e.register(SAVE_LOAD_READ, read);
        e.register(LOAD_NUMERIC_ID, read);
        e.register(ERROR_REPORT, |_, _| Ret::default());
        e.register(DEBUG_PRINT, |_, _| Ret::default());
        e.register(LOOKUP_FORM, |_, a| ret(a[0]));
        (e, buffer)
    }

    /// A record of a form (id 0x0d00_0070, word +5, byte +9) whose form (the id, as
    /// the lookup double answers it) is named 0x5150.
    fn record(e: &mut Engine) -> u32 {
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record, 0x0d00_0070);
        e.mem.set_u32(record + 5, 0x0102_0304);
        e.mem.set_u8(record + 9, 0x99);
        e.map(0x0d00_0000, 0x1000);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(0x0d00_0070, vtable);
        e.mem.set_u32(vtable + 0x130, 0x0f44_0130);
        e.register(0x0f44_0130, |_, _| ret(0x5150));
        record
    }

    #[test]
    fn save_size_counts_header_count_and_reactions() {
        let (mut e, _) = save_engine(true, false, 0);
        let a = reaction(&mut e, 1, 2, 3);
        let b = reaction(&mut e, 4, 5, 6);
        let this = reactions(&mut e, &[a, b]);
        assert_eq!(
            e.call(0x0048_c2e0, &args![this, 4u32]).u32() & 0xffff,
            6 + 2 + 16
        );
        assert_eq!(e.call(0x0048_c2e0, &args![this, 3u32]).u32() & 0xffff, 0);
        e.mem.set_u32(SAVE + 4, 0);
        e.mem.set_u8(SAVE + 8, 1);
        let (r, log) = run(&mut e, 0x0048_c2e0, &args![this, 4u32]);
        assert_eq!(r.eax & 0xffff, 18);
        assert_eq!(
            only(&log, ERROR_REPORT),
            vec![vec![
                GET_SAVE_SIZE_FORMAT,
                18,
                GET_SAVE_SIZE_LINE,
                REACTION_FORM_SOURCE_FILE
            ]]
        );
        let record = record(&mut e);
        e.mem.set_u32(SAVE + 0xc, record);
        let (_, log) = run(&mut e, 0x0048_c2e0, &args![this, 4u32]);
        assert_eq!(
            only(&log, ERROR_REPORT),
            vec![vec![
                GET_SAVE_SIZE_FORM_FORMAT,
                18,
                0x0d00_0070,
                0x5150,
                0x0102_0304,
                GET_SAVE_SIZE_LINE,
                REACTION_FORM_SOURCE_FILE
            ]]
        );
    }

    #[test]
    fn save_game_writes_a_block_then_loads_back() {
        let (mut e, buffer) = save_engine(true, true, 0);
        let first = form(&mut e, 0x111, 0);
        let second = form(&mut e, 0x222, 0);
        let a = reaction(&mut e, first, 5, 6);
        let b = reaction(&mut e, second, 7, 8);
        let this = reactions(&mut e, &[a, b]);
        let (_, log) = run(&mut e, 0x0048_c400, &args![this, 4u32]);
        assert_eq!(e.mem.u32(buffer), BLOCK_MARKER);
        assert_eq!(e.mem.u16(buffer + 4), 20);
        assert_eq!(e.mem.u16(buffer + 6), 2);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(buffer + 8 + 4 * i)).collect();
        assert_eq!(words, vec![0x111, 5, 0x222, 7]);
        assert_eq!(e.mem.u32(SAVE), buffer + 24);
        assert_eq!(
            only(&log, ERROR_REPORT),
            vec![vec![
                SAVE_GAME_FORMAT,
                24,
                SAVE_GAME_LINE,
                REACTION_FORM_SOURCE_FILE
            ]]
        );
        assert!(only(&log, DEBUG_PRINT).is_empty());
        let (_, log) = run(&mut e, 0x0048_c400, &args![this, 3u32]);
        assert_eq!(log.len(), 1);

        e.mem.set_u32(SAVE, buffer);
        let loaded = reactions(&mut e, &[0]);
        let (_, log) = run(&mut e, 0x0048_c640, &args![loaded, 4u32, 0u32]);
        assert_eq!(items(&e, loaded), vec![[0x111, 5, 0], [0x222, 7, 0]]);
        assert!(only(&log, DEBUG_PRINT).is_empty());
    }

    #[test]
    fn save_game_without_blocks_and_with_an_oversized_block() {
        let (mut e, buffer) = save_engine(false, false, 0);
        let target = form(&mut e, 0x111, 0);
        let a = reaction(&mut e, target, 5, 6);
        let this = reactions(&mut e, &[a]);
        e.call(0x0048_c400, &args![this, 4u32]);
        assert_eq!(e.mem.u16(buffer), 1);
        assert_eq!(e.mem.u32(SAVE), buffer + 10);
        let (mut e, _) = save_engine(true, false, 0);
        e.register(SAVE_LOAD_POSITION, |e, a| {
            let at = e.mem.u32(a[0]);
            ret(if e.mem.u32(a[0] + 0x10) != 0 {
                at + 0x10000
            } else {
                at
            })
        });
        let this = reactions(&mut e, &[0]);
        e.register_double(SAVE_LOAD_WRITE, |e, a| {
            let at = e.mem.u32(a[0]);
            for i in 0..a[2] {
                let b = e.mem.u8(a[1] + i);
                e.mem.set_u8(at + i, b);
            }
            e.mem.set_u32(a[0], at + a[2]);
            if a[2] == 2 && e.mem.u32(a[0] + 0x14) == 0 {
                e.mem.set_u32(a[0] + 0x14, 1);
            } else if a[2] == 2 {
                e.mem.set_u32(a[0] + 0x10, 1);
            }
            Ret::default()
        });
        let (_, log) = run(&mut e, 0x0048_c400, &args![this, 4u32]);
        assert_eq!(
            only(&log, DEBUG_PRINT),
            vec![vec![
                BLOCK_TOO_BIG_FORMAT,
                REACTION_FORM_SOURCE_FILE,
                SAVE_GAME_LINE
            ]]
        );
    }

    #[test]
    fn load_game_reports_bad_headers_and_size_mismatches() {
        let (mut e, buffer) = save_engine(true, false, 0);
        e.register(LOAD_NUMERIC_ID, |e, a| {
            let at = e.mem.u32(a[0]);
            let id = e.mem.u32(at);
            e.mem.set_u32(a[1], id);
            e.mem.set_u32(a[0], at + a[2]);
            Ret::default()
        });
        // A wrong marker, a size of 2 and one reaction (8 bytes): 8 over.
        e.mem.set_u32(buffer, 0x1234);
        e.mem.set_u16(buffer + 4, 2);
        e.mem.set_u16(buffer + 6, 1);
        e.mem.set_u32(buffer + 8, 0x500);
        e.mem.set_u32(buffer + 12, 3);
        let this = reactions(&mut e, &[0]);
        let (_, log) = run(&mut e, 0x0048_c640, &args![this, 4u32, 0u32]);
        assert_eq!(
            only(&log, DEBUG_PRINT),
            vec![
                vec![
                    BAD_HEADER_FORMAT,
                    REACTION_FORM_SOURCE_FILE,
                    LOAD_HEADER_LINE,
                    0x55
                ],
                vec![
                    OVERRUN_FORMAT,
                    10,
                    REACTION_FORM_SOURCE_FILE,
                    LOAD_SIZE_LINE,
                    0x55
                ]
            ]
        );
        assert_eq!(items(&e, this), vec![[0x500, 3, 0]]);
        // With the loading form's record and a size of 0x20: 0x16 short.
        let record = record(&mut e);
        e.mem.set_u32(SAVE + 0xc, record);
        e.mem.set_u32(SAVE, buffer);
        e.mem.set_u16(buffer + 4, 0x20);
        let (_, log) = run(&mut e, 0x0048_c640, &args![this, 4u32, 0u32]);
        assert_eq!(
            only(&log, DEBUG_PRINT),
            vec![
                vec![
                    BAD_HEADER_FORM_FORMAT,
                    REACTION_FORM_SOURCE_FILE,
                    LOAD_HEADER_LINE,
                    0x0d00_0070,
                    0x5150,
                    0x99,
                    0x0102_0304
                ],
                vec![
                    UNDERRUN_FORM_FORMAT,
                    0x14,
                    REACTION_FORM_SOURCE_FILE,
                    LOAD_SIZE_LINE,
                    0x0d00_0070,
                    0x5150,
                    0x99,
                    0x0102_0304
                ]
            ]
        );
        // Over with the record; then the flag bit missing does nothing.
        e.mem.set_u32(SAVE, buffer);
        e.mem.set_u32(buffer, BLOCK_MARKER);
        e.mem.set_u16(buffer + 4, 0);
        let (_, log) = run(&mut e, 0x0048_c640, &args![this, 4u32, 0u32]);
        assert_eq!(only(&log, DEBUG_PRINT)[0][..2], [OVERRUN_FORM_FORMAT, 12]);
        e.mem.set_u16(buffer + 4, 12);
        e.mem.set_u32(SAVE, buffer);
        let (_, log) = run(&mut e, 0x0048_c640, &args![this, 4u32, 0u32]);
        assert!(only(&log, DEBUG_PRINT).is_empty());
        let (_, log) = run(&mut e, 0x0048_c640, &args![this, 0u32, 0u32]);
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn resolve_turns_ids_into_forms_and_drops_missing_ones() {
        let mut e = engine();
        e.register(LOOKUP_FORM, |_, a| {
            ret(if a[0] == 0x99 { 0 } else { a[0] + 0x1000 })
        });
        let a = reaction(&mut e, 0x10, 1, 1);
        let missing = reaction(&mut e, 0x99, 2, 2);
        let b = reaction(&mut e, 0x20, 3, 3);
        let this = reactions(&mut e, &[a, missing, b]);
        let (_, log) = run(&mut e, 0x0048_c930, &args![this, 4u32, 0u32]);
        assert_eq!(items(&e, this), vec![[0x1010, 1, 1], [0x1020, 3, 3]]);
        assert_eq!(only(&log, FREE), vec![vec![missing]]);
        let (_, log) = run(&mut e, 0x0048_c930, &args![this, 0u32, 0u32]);
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn load_game_ov2_reads_the_reactions_when_flagged() {
        let mut e = engine();
        e.register(BUFFER_FLAGS, |e, a| {
            e.mem.set_u32(a[1], e.mem.u32(a[0]));
            ret(a[1])
        });
        e.register(FLAGS_HAVE, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) & a[1] != 0))
        });
        e.register(BUFFER_LOAD_VARIABLE_SIZE, |_, _| ret(2));
        e.register(BUFFER_LOAD_FORM_ID_INTO, |e, a| {
            let n = e.mem.u32(a[0] + 4) + 1;
            e.mem.set_u32(a[0] + 4, n);
            e.mem.set_u32(a[1], 0x700 + n);
            Ret::default()
        });
        e.register(BUFFER_LOAD_BYTES, |e, a| {
            let n = e.mem.u32(a[0] + 4) + 1;
            e.mem.set_u32(a[0] + 4, n);
            e.mem.set_u32(a[1], n);
            Ret::default()
        });
        let buffer = e.mem.alloc(8);
        let old = reaction(&mut e, 1, 1, 1);
        let this = reactions(&mut e, &[old]);
        let (_, log) = run(&mut e, 0x0048_ca90, &args![this, buffer]);
        assert!(only(&log, FREE).is_empty());
        assert_eq!(items(&e, this), vec![[1, 1, 1]]);
        e.mem.set_u32(buffer, 4);
        let (_, log) = run(&mut e, 0x0048_ca90, &args![this, buffer]);
        assert_eq!(only(&log, FREE), vec![vec![old]]);
        assert_eq!(items(&e, this), vec![[0x701, 2, 3], [0x704, 5, 6]]);
    }
}
