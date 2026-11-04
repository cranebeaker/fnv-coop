//! `fallout/dialogue/conversation.cpp` (Xbox PDB source unit), subsystem `fallout/dialogue`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `Conversation` (Xbox PDB, 0xC bytes): `lDialogueItems`
//! (`BSSimpleList<DialogueItem *>`, +0, first node embedded: item +0,
//! next +4) and `pCurrentItem` (+8, the node of the current item). The
//! list is walked as the code walks it: `006815c0` answers the node, whose
//! item is its first word, `00726070` the next node, `008256d0` whether a
//! node is empty (item and next both 0). The compiler's exception frames
//! (`FS:[0]`) are not translated; the functions that have them carry a
//! `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;

/// `pCurrentItem` (Xbox PDB).
const CURRENT_ITEM: u32 = 0x8;

/// The `BSSimpleList` constructor (`0096a2d0`, thiscall: clears item and next).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `BSSimpleList` destructor (thiscall).
const LIST_DESTROY: u32 = 0x0046_ffb0;
/// `TESTopic::CreateConversation` (engine map), cdecl (four words).
const TOPIC_CREATE_CONVERSATION: u32 = 0x0061_b440;
/// thiscall on a list node: whether it is empty.
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// thiscall on a list node: answers the node itself.
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
/// thiscall on a list node: the next node (+4).
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// thiscall on a list: removes the head node (`0063f7b0`).
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// The `DialogueItem` deleting destructor (`005c90d0`, thiscall (item, flags)).
const DIALOGUE_ITEM_DELETING_DESTRUCTOR: u32 = 0x005c_90d0;
/// cdecl (text): prints a message.
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// The message `GetDialogueItemIndex` prints when the item is not in the list.
const ITEM_NOT_FOUND_MESSAGE: u32 = 0x0107_da30;
/// `DialogueItem::SaveGame_ov2` (engine map), thiscall (item, buffer).
const DIALOGUE_ITEM_SAVE_GAME: u32 = 0x0083_ce40;
/// `BGSSaveGameBuffer::StartVariableSizedValue` (engine map), thiscall (buffer).
const SAVE_START_VARIABLE_SIZED: u32 = 0x0086_5f20;
/// `BGSSaveGameBuffer::SaveVariableSizedValue_ov2` (engine map), thiscall (buffer, value, mark).
const SAVE_VARIABLE_SIZED: u32 = 0x0086_5ff0;
/// thiscall (buffer, data, size, 0): writes bytes to the save buffer.
const SAVE_BUFFER_WRITE: u32 = 0x0086_5e50;
/// thiscall on a `DialogueItem`'s response list: precaches its sounds (`0083d130`).
const PRECACHE_RESPONSES: u32 = 0x0083_d130;

fn node_item(e: &mut Engine, node: u32) -> u32 {
    let at = e.call(LIST_NODE_ITEM, &args![node]).u32();
    e.mem.u32(at)
}

fn node_is_empty(e: &mut Engine, node: u32) -> bool {
    e.call(LIST_NODE_IS_EMPTY, &args![node]).bool()
}

// Translated from 0083b850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083b850 - the C++ exception-unwinding frame (FS:[0]) that destroys the list when the topic call throws is not translated
/// `Conversation::Conversation_ov2` (Xbox PDB), thiscall `(topic, speaker,
/// third)`: constructs the item list, clears `pCurrentItem` and fills the
/// conversation through `TESTopic::CreateConversation(topic, speaker,
/// this, third)`. Answers `this`.
pub fn conversation_conversation_ov2(
    e: &mut Engine,
    this: Ptr,
    topic: u32,
    speaker: u32,
    third: u32,
) -> Ptr {
    let base = this.addr();
    e.call(LIST_CONSTRUCT, &args![base]);
    e.mem.set_u32(base + CURRENT_ITEM, 0);
    e.call(
        TOPIC_CREATE_CONVERSATION,
        &args![topic, speaker, base, third],
    );
    this
}

// Translated from 0083b8d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083b8d0 - the C++ exception-unwinding frame (FS:[0]) that destroys the list when the clear throws is not translated
/// `Conversation::~Conversation` (unnamed in the engine map; the body is
/// the destructor): deletes the items (`0083b930`) and destroys the list.
pub fn fn_0083b8d0(e: &mut Engine, this: Ptr) {
    fn_0083b930(e, this);
    e.call(LIST_DESTROY, &args![this]);
}

// Translated from 0083b930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes every dialogue item (unnamed in the engine map): while the list
/// is not empty, deletes the head item (its deleting destructor with 1)
/// when there is one and removes the head node; then clears
/// `pCurrentItem`.
pub fn fn_0083b930(e: &mut Engine, this: Ptr) {
    let list = this.addr();
    while list != 0 && !node_is_empty(e, list) {
        let item = node_item(e, list);
        if item != 0 {
            e.call(DIALOGUE_ITEM_DELETING_DESTRUCTOR, &args![item, 1u32]);
        }
        e.call(LIST_REMOVE_HEAD, &args![list]);
    }
    e.mem.set_u32(list + CURRENT_ITEM, 0);
}

// Translated from 0083b9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Conversation::FirstItem` (Xbox PDB): makes the first node current
/// (`pCurrentItem = this`) and answers whether it holds an item.
pub fn conversation_first_item(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.mem.set_u32(base + CURRENT_ITEM, base);
    let current = e.mem.u32(base + CURRENT_ITEM);
    if current == 0 {
        return false;
    }
    let current = e.mem.u32(base + CURRENT_ITEM);
    node_item(e, current) != 0
}

// Translated from 0083ba50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Conversation::GetDialogueItemIndex` (Xbox PDB), thiscall `(item)`: the
/// index (16 bits, in `AX`) of the item in the list, counting until an
/// empty node; when it is not found, prints the message at `0107da30` and
/// answers 0.
pub fn conversation_get_dialogue_item_index(e: &mut Engine, this: Ptr, item: u32) -> u16 {
    let mut index = 0u16;
    let mut node = this.addr();
    while node != 0 && !node_is_empty(e, node) {
        if item == node_item(e, node) {
            return index;
        }
        index = index.wrapping_add(1);
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    e.call(PRINT_MESSAGE, &args![ITEM_NOT_FOUND_MESSAGE]);
    0
}

// Translated from 0083c270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Conversation::SaveGame` (Xbox PDB), thiscall `(buffer)`: saves every
/// item of the list (each node with an item) as a variable-sized count,
/// then the current item's index as two bytes (-1 when there is no current
/// item).
pub fn conversation_save_game(e: &mut Engine, this: Ptr, buffer: u32) {
    let base = this.addr();
    let mut count = 0u32;
    let mark = e.call(SAVE_START_VARIABLE_SIZED, &args![buffer]).u32();
    let mut node = base;
    while node != 0 {
        let item = node_item(e, node);
        if item != 0 {
            e.call(DIALOGUE_ITEM_SAVE_GAME, &args![item, buffer]);
            count += 1;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZED, &args![buffer, count, mark]);
    let index = if e.mem.u32(base + CURRENT_ITEM) != 0 {
        let current = e.mem.u32(base + CURRENT_ITEM);
        let item = node_item(e, current);
        conversation_get_dialogue_item_index(e, this, item)
    } else {
        0xffff
    };
    e.with_stack(4, |e, value| {
        e.mem.set_u16(value.addr(), index);
        e.call(SAVE_BUFFER_WRITE, &args![buffer, value, 2u32, 0u32]);
    });
}

// Translated from 0083c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Conversation::PrecacheSounds` (Xbox PDB): for each item of the list
/// (until an empty node), precaches its responses' sounds (`0083d130` on
/// the item).
pub fn conversation_precache_sounds(e: &mut Engine, this: Ptr) {
    let mut node = this.addr();
    while node != 0 && !node_is_empty(e, node) {
        if node != 0 && node_item(e, node) != 0 {
            let item = node_item(e, node);
            e.call(PRECACHE_RESPONSES, &args![item]);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 0083b9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Conversation::NextItem` (engine map; `ECX` only, the answer in `AL`): moves
/// `pCurrentItem` to the next node (`00726070`) when there is one, then answers whether
/// there is a node whose item (`006815c0`, the word it points at) is not null.
pub fn conversation_next_item(e: &mut Engine, this: Ptr) -> bool {
    let at = this.addr() + CURRENT_ITEM;
    if e.mem.u32(at) != 0 {
        let node = e.mem.u32(at);
        let next = e.call(LIST_NODE_NEXT, &args![node]).u32();
        e.mem.set_u32(at, next);
    }
    if e.mem.u32(at) == 0 {
        return false;
    }
    let node = e.mem.u32(at);
    let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
    e.mem.u32(item) != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0083_b9f0, conversation_next_item(Ptr) -> bool),
        entry!(
            0x0083b850,
            conversation_conversation_ov2(Ptr, u32, u32, u32) -> Ptr
        ),
        entry!(0x0083b8d0, fn_0083b8d0(Ptr)),
        entry!(0x0083b930, fn_0083b930(Ptr)),
        entry!(0x0083b9a0, conversation_first_item(Ptr) -> bool),
        entry!(
            0x0083ba50,
            conversation_get_dialogue_item_index(Ptr, u32) -> u16
        ),
        entry!(0x0083c270, conversation_save_game(Ptr, u32)),
        entry!(0x0083c470, conversation_precache_sounds(Ptr)),
    ]
    .into_iter()
    .chain(funcs_n086())
    .collect()
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

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An engine with the list helpers doubled as the game's.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(LIST_NODE_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        e
    }

    /// A conversation whose list holds `items` (first node embedded).
    fn conversation(e: &mut Engine, items: &[u32]) -> u32 {
        let this = e.mem.alloc(0xc);
        let mut node = this;
        for (index, &item) in items.iter().enumerate() {
            e.mem.set_u32(node, item);
            if index + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        this
    }

    #[test]
    fn constructor_builds_the_list_through_the_topic() {
        let mut e = engine();
        e.register(LIST_CONSTRUCT, |_, _| Ret::default());
        e.register(TOPIC_CREATE_CONVERSATION, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 5);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0083_b850, &args![this, 1u32, 2u32, 3u32]).u32(),
            this
        );
        assert_eq!(calls(&e, LIST_CONSTRUCT), vec![vec![this]]);
        assert_eq!(
            calls(&e, TOPIC_CREATE_CONVERSATION),
            vec![vec![1, 2, this, 3]]
        );
        assert_eq!(e.mem.u32(this + 8), 0);
    }

    #[test]
    fn destructor_deletes_each_item_and_the_list() {
        let mut e = engine();
        e.register(DIALOGUE_ITEM_DELETING_DESTRUCTOR, |_, _| Ret::default());
        e.register(LIST_DESTROY, |_, _| Ret::default());
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let (item, after) = if next != 0 {
                (e.mem.u32(next), e.mem.u32(next + 4))
            } else {
                (0, 0)
            };
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        let this = conversation(&mut e, &[0x10, 0, 0x30]);
        e.mem.set_u32(this + 8, this);
        e.call_log = Some(vec![]);
        e.call(0x0083_b8d0, &args![this]);
        assert_eq!(
            calls(&e, DIALOGUE_ITEM_DELETING_DESTRUCTOR),
            vec![vec![0x10, 1], vec![0x30, 1]]
        );
        assert_eq!(calls(&e, LIST_REMOVE_HEAD).len(), 3);
        assert_eq!(calls(&e, LIST_DESTROY), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 8), 0);
    }

    #[test]
    fn first_item_makes_the_head_current() {
        let mut e = engine();
        let this = conversation(&mut e, &[0x10]);
        assert!(e.call(0x0083_b9a0, &args![this]).bool());
        assert_eq!(e.mem.u32(this + 8), this);
        let empty = conversation(&mut e, &[0]);
        assert!(!e.call(0x0083_b9a0, &args![empty]).bool());
    }

    #[test]
    fn item_index_counts_nodes_and_reports_a_missing_item() {
        let mut e = engine();
        e.register(PRINT_MESSAGE, |_, _| Ret::default());
        let this = conversation(&mut e, &[0x10, 0x20, 0x30]);
        assert_eq!(e.call(0x0083_ba50, &args![this, 0x30u32]).u32(), 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_ba50, &args![this, 0x40u32]).u32(), 0);
        assert_eq!(calls(&e, PRINT_MESSAGE), vec![vec![ITEM_NOT_FOUND_MESSAGE]]);
    }

    #[test]
    fn save_game_saves_the_items_and_the_current_index() {
        let mut e = engine();
        e.register(SAVE_START_VARIABLE_SIZED, |_, _| ret(0x77));
        e.register(DIALOGUE_ITEM_SAVE_GAME, |_, _| Ret::default());
        e.register(SAVE_VARIABLE_SIZED, |_, _| Ret::default());
        let written = std::rc::Rc::new(std::cell::Cell::new(0u16));
        let seen = written.clone();
        e.register_double(SAVE_BUFFER_WRITE, move |e, a| {
            seen.set(e.mem.u16(a[1]));
            Ret::default()
        });
        let this = conversation(&mut e, &[0x10, 0, 0x30]);
        let third = e.mem.u32(e.mem.u32(this + 4) + 4);
        e.mem.set_u32(this + 8, third);
        e.call_log = Some(vec![]);
        e.call(0x0083_c270, &args![this, 0x900u32]);
        assert_eq!(
            calls(&e, DIALOGUE_ITEM_SAVE_GAME),
            vec![vec![0x10, 0x900], vec![0x30, 0x900]]
        );
        assert_eq!(calls(&e, SAVE_VARIABLE_SIZED), vec![vec![0x900, 2, 0x77]]);
        assert_eq!(calls(&e, SAVE_BUFFER_WRITE)[0][2..], [2, 0]);
        assert_eq!(written.get(), 2);
        e.mem.set_u32(this + 8, 0);
        e.call(0x0083_c270, &args![this, 0x900u32]);
        assert_eq!(written.get(), 0xffff);
    }

    #[test]
    fn precache_sounds_visits_each_item() {
        let mut e = engine();
        e.register(PRECACHE_RESPONSES, |_, _| Ret::default());
        let this = conversation(&mut e, &[0x10, 0, 0x30]);
        e.call_log = Some(vec![]);
        e.call(0x0083_c470, &args![this]);
        assert_eq!(calls(&e, PRECACHE_RESPONSES), vec![vec![0x10], vec![0x30]]);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    /// List nodes of two words (item, next): `006815c0` answers the node itself (the item
    /// word's address), `00726070` its next word.
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(LIST_NODE_ITEM, |_, a| a[0].into_ret());
        e.register(LIST_NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e
    }

    #[test]
    fn next_item_moves_on_and_answers_whether_it_has_an_item() {
        let mut e = list_engine();
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        let third = e.mem.alloc(8);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, 0x7000);
        e.mem.set_u32(second + 4, third);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, first);
        assert!(e.call(0x0083_b9f0, &args![this]).bool());
        assert_eq!(e.mem.u32(this + 8), second);
        // The third node has a null item.
        assert!(!e.call(0x0083_b9f0, &args![this]).bool());
        assert_eq!(e.mem.u32(this + 8), third);
        // Past the end: no node.
        assert!(!e.call(0x0083_b9f0, &args![this]).bool());
        assert_eq!(e.mem.u32(this + 8), 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0083_b9f0, &args![this]).bool());
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}

/// The message `GetDialogueItemByIndex` prints when the index is past the
/// list (`0107daa0`).
const INDEX_TOO_LARGE_MESSAGE: u32 = 0x0107_daa0;
/// The save/load game object (`011de45c`) and the calls on it (`thiscall`):
/// whether block headers are written (`00862110`, a byte), the buffer
/// position (`00825c00`), writing (`008579b0`) and reading (`008579e0`)
/// bytes (two words: the data's address, the size), the form being saved
/// (`004fd3e0`) or loaded (`004fd3c0`), and a byte for the load messages
/// (`008df040`).
const SAVE_LOAD_GAME_011DE45C: u32 = 0x011d_e45c;
const SAVE_LOAD_HAS_BLOCK_HEADERS: u32 = 0x0086_2110;
const SAVE_LOAD_POSITION: u32 = 0x0082_5c00;
const SAVE_LOAD_WRITE: u32 = 0x0085_79b0;
const SAVE_LOAD_READ: u32 = 0x0085_79e0;
const SAVE_LOAD_SAVED_FORM: u32 = 0x004f_d3e0;
const SAVE_LOAD_LOADED_FORM: u32 = 0x004f_d3c0;
const SAVE_LOAD_BYTE_008DF040: u32 = 0x008d_f040;
/// The setting (`011de4e8`) whose byte (`00408d60` answers its address)
/// turns on the save size traces.
const TRACE_SETTING_011DE4E8: u32 = 0x011d_e4e8;
const SETTING_BYTE_ADDRESS: u32 = 0x0040_8d60;
/// The form lookup by id (`004839c0`, `cdecl`).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// The trace print (`0040fbe0`, `cdecl`, a format and its words).
const TRACE_PRINT: u32 = 0x0040_fbe0;
/// The block header word `"KOLB"` (`0x424c4f4b`).
const BLOCK_HEADER: u32 = 0x424c_4f4b;
/// This file's name (`0107db20`) and the trace and load messages
/// (`.rdata` texts: `GetSaveSize()` with and without the form, `SaveGame()`
/// with and without the form, the block too large for a short, the wrong
/// block header with and without the form, the buffer overrun and underrun
/// with and without the form).
const SOURCE_FILE_0107DB20: u32 = 0x0107_db20;
const TEXT_SAVE_SIZE_FORM: u32 = 0x0101_2cb0;
const TEXT_SAVE_SIZE: u32 = 0x0101_2c78;
const TEXT_SAVE_GAME_FORM: u32 = 0x0101_53a0;
const TEXT_SAVE_GAME: u32 = 0x0101_536c;
const TEXT_BLOCK_TOO_LARGE: u32 = 0x0101_5318;
const TEXT_BAD_HEADER_FORM: u32 = 0x0101_5718;
const TEXT_BAD_HEADER: u32 = 0x0101_56a8;
const TEXT_OVERRUN_FORM: u32 = 0x0101_5588;
const TEXT_UNDERRUN_FORM: u32 = 0x0101_5500;
const TEXT_OVERRUN: u32 = 0x0101_54a0;
const TEXT_UNDERRUN: u32 = 0x0101_5440;
/// The `DialogueItem` calls (`thiscall`): its save size (`0083c9e0`, 16
/// bits), its save (`0083ca90`), the call `0083c220` makes (`0083cdf0`),
/// its constructor (`0083c4d0`, 0x1c bytes), its loads (`0083cc10`, and
/// `0083cf40` with a buffer) and the one `0083c420` makes (`0083d0d0`, one
/// word).
const ITEM_SAVE_SIZE: u32 = 0x0083_c9e0;
const ITEM_SAVE: u32 = 0x0083_ca90;
const ITEM_0083CDF0: u32 = 0x0083_cdf0;
const ITEM_CONSTRUCT: u32 = 0x0083_c4d0;
const ITEM_LOAD: u32 = 0x0083_cc10;
const ITEM_LOAD_FROM: u32 = 0x0083_cf40;
const ITEM_0083D0D0: u32 = 0x0083_d0d0;
/// `00905820` (`thiscall`, one word: the address of the item): adds an item
/// to the list; `0083c8a0` (`thiscall`, one word) makes an item current.
const LIST_ADD: u32 = 0x0090_5820;
const SET_CURRENT_ITEM: u32 = 0x0083_c8a0;
/// `00401000` (`cdecl`, one word): allocates a block.
const ALLOCATE: u32 = 0x0040_1000;
/// The load buffer's count (`00864a60`, `thiscall`) and read (`00864980`,
/// `thiscall`, two words: address, size).
const BUFFER_COUNT: u32 = 0x0086_4a60;
const BUFFER_READ: u32 = 0x0086_4980;

/// The save/load game object.
fn save_load(e: &Engine) -> u32 {
    e.mem.u32(SAVE_LOAD_GAME_011DE45C)
}

/// Whether the save/load game object writes block headers.
fn has_block_headers(e: &mut Engine) -> bool {
    let game = save_load(e);
    e.call(SAVE_LOAD_HAS_BLOCK_HEADERS, &args![game]).u32() & 0xff != 0
}

/// The save/load game object's buffer position.
fn position(e: &mut Engine) -> u32 {
    let game = save_load(e);
    e.call(SAVE_LOAD_POSITION, &args![game]).u32()
}

/// Whether the trace setting's byte is set.
fn tracing(e: &mut Engine) -> bool {
    let at = e
        .call(SETTING_BYTE_ADDRESS, &args![TRACE_SETTING_011DE4E8])
        .u32();
    e.mem.u8(at) != 0
}

/// The form id at `info`, its form's name (virtual slot `+0x130`, no
/// words) and the word at `info + 5`: the words the traces print.
fn form_words(e: &mut Engine, info: u32) -> (u32, u32, u32) {
    let id = e.mem.u32(info);
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    let flags = e.mem.u32(info.wrapping_add(5));
    let name = e.vcall(form, 0x130, &[]).u32();
    (e.mem.u32(info), name, flags)
}

// Translated from 0083b830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Conversation::Conversation` (engine map; `thiscall`, plain `RET`): the
/// item list constructed (`0096a2d0`) and `pCurrentItem` cleared. Returns
/// `this`.
pub fn conversation_conversation(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(LIST_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + CURRENT_ITEM, 0);
    this
}

// Translated from 0083bac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Conversation::GetDialogueItemByIndex` (engine map; `thiscall`, `RET 4`):
/// the item of node `index` (both compared as signed 16-bit numbers),
/// counting until an empty node; past the list the message at `0107daa0`
/// and null.
pub fn conversation_get_dialogue_item_by_index(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let mut at = 0i16;
    let mut node = this.addr();
    while node != 0 && !node_is_empty(e, node) {
        if at == index as i16 {
            return node_item(e, node);
        }
        at = at.wrapping_add(1);
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    e.call(PRINT_MESSAGE, &args![INDEX_TOO_LARGE_MESSAGE]);
    0
}

// Translated from 0083bb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The conversation's save size (unnamed in the engine map; its trace says
/// `GetSaveSize()`; `thiscall`, plain `RET`, 16 bits in `AX`): 6 more with
/// block headers, 2, each item's size (`0083c9e0`) until an empty node, and
/// 2. With the trace setting (`011de4e8`) on, prints the size (`0040fbe0`)
/// with the saved form's id, name and flags when there is one.
pub fn fn_0083bb30(e: &mut Engine, this: Ptr) -> u16 {
    let mut size = 0u16;
    let start = size;
    if has_block_headers(e) {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    size = size.wrapping_add(2);
    let mut node = this.addr();
    while node != 0 && !node_is_empty(e, node) {
        let item = node_item(e, node);
        let item_size = e.call(ITEM_SAVE_SIZE, &args![item]).u32() as u16;
        size = size.wrapping_add(item_size);
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    size = size.wrapping_add(2);
    if tracing(e) {
        let game = save_load(e);
        let info = e.call(SAVE_LOAD_SAVED_FORM, &args![game]).u32();
        let length = u32::from(size).wrapping_sub(u32::from(start));
        if info != 0 {
            let (id, name, flags) = form_words(e, info);
            e.call(
                TRACE_PRINT,
                &args![
                    TEXT_SAVE_SIZE_FORM,
                    length,
                    id,
                    name,
                    flags,
                    0xc0u32,
                    SOURCE_FILE_0107DB20
                ],
            );
        } else {
            e.call(
                TRACE_PRINT,
                &args![TEXT_SAVE_SIZE, length, 0xc0u32, SOURCE_FILE_0107DB20],
            );
        }
    }
    size
}

// Translated from 0083bc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The conversation's save (unnamed in the engine map; its trace says
/// `SaveGame()`; `thiscall`, plain `RET`), through the save/load game object
/// (`011de45c`): with block headers the header `"KOLB"` and a 16-bit block
/// size (0 for now, its place kept); the 16-bit item count (written over
/// its place when the items, until an empty node, are saved: `0083ca90`);
/// the current item's index (`GetDialogueItemIndex`, -1 without one). The
/// trace setting prints the bytes written (from the position before the
/// block); with block headers the block size goes into its place (a
/// message when it is over 0xffff).
pub fn fn_0083bc80(e: &mut Engine, this: Ptr) {
    e.with_stack(0x20, |e, locals| {
        let l = locals.addr();
        let header = l;
        let count = l + 4;
        let index = l + 8;
        let block_size = l + 0xc;
        e.mem.set_u16(block_size, 0);
        let mut size_place = 0u32;
        let mut start = position(e);
        if tracing(e) {
            start = position(e);
        }
        if has_block_headers(e) {
            e.mem.set_u32(header, BLOCK_HEADER);
            let game = save_load(e);
            e.call(SAVE_LOAD_WRITE, &args![game, header, 4u32]);
            size_place = position(e);
            let game = save_load(e);
            e.call(SAVE_LOAD_WRITE, &args![game, block_size, 2u32]);
        }
        e.mem.set_u16(count, 0);
        let count_place = position(e);
        let game = save_load(e);
        e.call(SAVE_LOAD_WRITE, &args![game, count, 2u32]);
        let mut node = this.addr();
        while node != 0 && !node_is_empty(e, node) {
            let item = node_item(e, node);
            e.call(ITEM_SAVE, &args![item]);
            let n = e.mem.u16(count).wrapping_add(1);
            e.mem.set_u16(count, n);
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
        let n = e.mem.u16(count);
        e.mem.set_u16(count_place, n);
        e.mem.set_u16(index, 0xffff);
        if e.mem.u32(this.addr() + CURRENT_ITEM) != 0 {
            let current = e.mem.u32(this.addr() + CURRENT_ITEM);
            let item = node_item(e, current);
            let at = conversation_get_dialogue_item_index(e, this, item);
            e.mem.set_u16(index, at);
        }
        let game = save_load(e);
        e.call(SAVE_LOAD_WRITE, &args![game, index, 2u32]);
        if tracing(e) {
            let end = position(e);
            let game = save_load(e);
            let info = e.call(SAVE_LOAD_SAVED_FORM, &args![game]).u32();
            let length = end.wrapping_sub(start);
            if info != 0 {
                let (id, name, flags) = form_words(e, info);
                e.call(
                    TRACE_PRINT,
                    &args![
                        TEXT_SAVE_GAME_FORM,
                        length,
                        id,
                        name,
                        flags,
                        0xe1u32,
                        SOURCE_FILE_0107DB20
                    ],
                );
            } else {
                e.call(
                    TRACE_PRINT,
                    &args![TEXT_SAVE_GAME, length, 0xe1u32, SOURCE_FILE_0107DB20],
                );
            }
        }
        if has_block_headers(e) {
            let end = position(e);
            if end > size_place.wrapping_add(0xffff) {
                e.call(
                    PRINT_MESSAGE,
                    &args![TEXT_BLOCK_TOO_LARGE, SOURCE_FILE_0107DB20, 0xe1u32],
                );
            }
            e.mem
                .set_u16(size_place, end.wrapping_sub(size_place) as u16);
        }
    });
}

/// Reads `items` dialogue items: each a new 0x1c-byte `DialogueItem`
/// (`0083c4d0`; null when the allocation fails) loaded by `load` and added
/// to the list (`00905820`, through a stack word holding it).
fn load_items(e: &mut Engine, this: Ptr, items: u32, slot: u32, load: &dyn Fn(&mut Engine, u32)) {
    for _ in 0..items {
        let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
        let item = if block != 0 {
            e.call(ITEM_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(slot, item);
        load(e, item);
        e.call(LIST_ADD, &args![this, slot]);
    }
}

// Translated from 0083bec0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083bec0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `Conversation::LoadGame` (engine map; `thiscall`, plain `RET`), through
/// the save/load game object (`011de45c`): with block headers the 4-byte
/// header (a message when it is not `"KOLB"`, with the loaded form's id,
/// name, byte `+9` and word `+5`, or the byte `008df040`), then the 16-bit
/// block size (its position kept); the 16-bit item count and that many
/// items (`0083cc10`); the 16-bit current index (`GetDialogueItemByIndex`
/// and `0083c8a0`, or `pCurrentItem` cleared for -1). With block headers,
/// a message when the position is past or short of the block's end.
pub fn conversation_load_game(e: &mut Engine, this: Ptr) {
    e.with_stack(0x20, |e, locals| {
        let l = locals.addr();
        let header = l;
        let block_size = l + 4;
        let count = l + 8;
        let index = l + 0xc;
        let slot = l + 0x10;
        e.mem.set_u16(block_size, 0);
        let mut start = 0u32;
        if has_block_headers(e) {
            let game = save_load(e);
            e.call(SAVE_LOAD_READ, &args![game, header, 4u32]);
            if e.mem.u32(header) != BLOCK_HEADER {
                let game = save_load(e);
                let info = e.call(SAVE_LOAD_LOADED_FORM, &args![game]).u32();
                if info != 0 {
                    let id = e.mem.u32(info);
                    let form = e.call(LOOKUP_FORM, &args![id]).u32();
                    let word = e.mem.u32(info.wrapping_add(5));
                    let byte = u32::from(e.mem.u8(info.wrapping_add(9)));
                    let name = e.vcall(form, 0x130, &[]).u32();
                    let id = e.mem.u32(info);
                    e.call(
                        PRINT_MESSAGE,
                        &args![
                            TEXT_BAD_HEADER_FORM,
                            SOURCE_FILE_0107DB20,
                            0xe7u32,
                            id,
                            name,
                            byte,
                            word
                        ],
                    );
                } else {
                    let game = save_load(e);
                    let byte = e.call(SAVE_LOAD_BYTE_008DF040, &args![game]).u32() & 0xff;
                    e.call(
                        PRINT_MESSAGE,
                        &args![TEXT_BAD_HEADER, SOURCE_FILE_0107DB20, 0xe7u32, byte],
                    );
                }
            }
            start = position(e);
            let game = save_load(e);
            e.call(SAVE_LOAD_READ, &args![game, block_size, 2u32]);
        }
        let game = save_load(e);
        e.call(SAVE_LOAD_READ, &args![game, count, 2u32]);
        let items = u32::from(e.mem.u16(count));
        load_items(e, this, items, slot, &|e, item| {
            e.call(ITEM_LOAD, &args![item]);
        });
        let game = save_load(e);
        e.call(SAVE_LOAD_READ, &args![game, index, 2u32]);
        if e.mem.u16(index) as i16 != -1 {
            let at = u32::from(e.mem.u16(index));
            let item = conversation_get_dialogue_item_by_index(e, this, at);
            e.call(SET_CURRENT_ITEM, &args![this, item]);
        } else {
            e.mem.set_u32(this.addr() + CURRENT_ITEM, 0);
        }
        if !has_block_headers(e) {
            return;
        }
        let end = position(e);
        let game = save_load(e);
        let info = e.call(SAVE_LOAD_LOADED_FORM, &args![game]).u32();
        let expected = u32::from(e.mem.u16(block_size)).wrapping_add(start);
        if info != 0 {
            let id = e.mem.u32(info);
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            let (text, amount) = if end > expected {
                (TEXT_OVERRUN_FORM, end.wrapping_sub(expected))
            } else if end < expected {
                (TEXT_UNDERRUN_FORM, expected.wrapping_sub(end))
            } else {
                return;
            };
            let word = e.mem.u32(info.wrapping_add(5));
            let byte = u32::from(e.mem.u8(info.wrapping_add(9)));
            let name = e.vcall(form, 0x130, &[]).u32();
            let id = e.mem.u32(info);
            e.call(
                PRINT_MESSAGE,
                &args![
                    text,
                    amount,
                    SOURCE_FILE_0107DB20,
                    0xfdu32,
                    id,
                    name,
                    byte,
                    word
                ],
            );
        } else {
            let (text, amount) = if end > expected {
                (TEXT_OVERRUN, end.wrapping_sub(expected))
            } else if end < expected {
                (TEXT_UNDERRUN, expected.wrapping_sub(end))
            } else {
                return;
            };
            let game = save_load(e);
            let byte = e.call(SAVE_LOAD_BYTE_008DF040, &args![game]).u32() & 0xff;
            e.call(
                PRINT_MESSAGE,
                &args![text, amount, SOURCE_FILE_0107DB20, 0xfdu32, byte],
            );
        }
    });
}

// Translated from 0083c220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): `0083cdf0` on each
/// item until an empty node.
pub fn fn_0083c220(e: &mut Engine, this: Ptr) {
    let mut node = this.addr();
    while node != 0 && !node_is_empty(e, node) {
        let item = node_item(e, node);
        e.call(ITEM_0083CDF0, &args![item]);
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 0083c330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083c330 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `Conversation::LoadGame_ov2` (engine map; `thiscall`, `RET 4`: the load
/// buffer): the buffer's count (`00864a60`) of items, each a new
/// `DialogueItem` loaded from the buffer (`0083cf40`) and added; then the
/// 16-bit current index read (`00864980`), made current when it is not -1.
pub fn conversation_load_game_ov2(e: &mut Engine, this: Ptr, buffer: u32) {
    e.with_stack(8, |e, locals| {
        let slot = locals.addr();
        let index = locals.addr() + 4;
        let items = e.call(BUFFER_COUNT, &args![buffer]).u32();
        load_items(e, this, items, slot, &|e, item| {
            e.call(ITEM_LOAD_FROM, &args![item, buffer]);
        });
        e.mem.set_u16(index, 0xffff);
        e.call(BUFFER_READ, &args![buffer, index, 2u32]);
        if e.mem.u16(index) as i16 != -1 {
            let at = u32::from(e.mem.u16(index));
            let item = conversation_get_dialogue_item_by_index(e, this, at);
            e.call(SET_CURRENT_ITEM, &args![this, item]);
        }
    });
}

// Translated from 0083c420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): `0083d0d0` with `value`
/// on every item of the list (every node, empty or not, whose item is
/// set).
pub fn fn_0083c420(e: &mut Engine, this: Ptr, value: u32) {
    let mut node = this.addr();
    while node != 0 {
        let item = node_item(e, node);
        if item != 0 {
            e.call(ITEM_0083D0D0, &args![item, value]);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

/// This unit's n086 translations, by exe address.
fn funcs_n086() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0083_b830, conversation_conversation(Ptr) -> Ptr),
        entry!(
            0x0083_bac0,
            conversation_get_dialogue_item_by_index(Ptr, u32) -> u32
        ),
        entry!(0x0083_bb30, fn_0083bb30(Ptr) -> u16),
        entry!(0x0083_bc80, fn_0083bc80(Ptr)),
        entry!(0x0083_bec0, conversation_load_game(Ptr)),
        entry!(0x0083_c220, fn_0083c220(Ptr)),
        entry!(0x0083_c330, conversation_load_game_ov2(Ptr, u32)),
        entry!(0x0083_c420, fn_0083c420(Ptr, u32)),
    ]
}

#[cfg(test)]
mod n086_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A list of nodes (item +0, next +4) holding `items`, the last node
    /// empty when `trailing_empty`.
    fn list(e: &mut Engine, items: &[u32], trailing_empty: bool) -> u32 {
        let count = items.len() + usize::from(trailing_empty);
        let nodes: Vec<u32> = (0..count).map(|_| e.mem.alloc(0xc)).collect();
        for (i, &node) in nodes.iter().enumerate() {
            e.mem.set_u32(node, items.get(i).copied().unwrap_or(0));
            e.mem
                .set_u32(node + 4, nodes.get(i + 1).copied().unwrap_or(0));
        }
        nodes[0]
    }

    fn list_walking(e: &mut Engine) {
        e.register(LIST_NODE_ITEM, |_, a| a[0].into_ret());
        e.register(LIST_NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(PRINT_MESSAGE, |_, _| Ret::default());
        e.register(TRACE_PRINT, |_, _| Ret::default());
    }

    /// The save/load game object with block headers `headers`, a position
    /// counter that each write and read advances, and the trace byte.
    fn save_load_game(e: &mut Engine, headers: bool, trace: bool) -> u32 {
        let game = e.mem.alloc(0x10);
        e.map(SAVE_LOAD_GAME_011DE45C, 4);
        e.mem.set_u32(SAVE_LOAD_GAME_011DE45C, game);
        e.mem.set_u32(game + 4, u32::from(headers));
        e.register(SAVE_LOAD_HAS_BLOCK_HEADERS, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(SAVE_LOAD_POSITION, |e, a| e.mem.u32(a[0]).into_ret());
        let advance = |e: &mut Engine, a: &[u32]| {
            let p = e.mem.u32(a[0]) + a[2];
            e.mem.set_u32(a[0], p);
            Ret::default()
        };
        e.register(SAVE_LOAD_WRITE, advance);
        e.register(SAVE_LOAD_READ, |e, a| {
            // Reads come from the queue at game +8 (two words).
            let p = e.mem.u32(a[0]) + a[2];
            e.mem.set_u32(a[0], p);
            let queue = e.mem.u32(a[0] + 8);
            let value = e.mem.u32(queue);
            e.mem.set_u32(a[0] + 8, queue + 4);
            if a[2] == 4 {
                e.mem.set_u32(a[1], value);
            } else {
                e.mem.set_u16(a[1], value as u16);
            }
            Ret::default()
        });
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, u8::from(trace));
        e.register_double(SETTING_BYTE_ADDRESS, move |_, _| flag.into_ret());
        e.register(SAVE_LOAD_SAVED_FORM, |_, _| 0u32.into_ret());
        e.register(SAVE_LOAD_LOADED_FORM, |_, _| 0u32.into_ret());
        e.register(SAVE_LOAD_BYTE_008DF040, |_, _| 0x1207u32.into_ret());
        game
    }

    #[test]
    fn constructor_and_item_by_index() {
        let mut e = Engine::new();
        list_walking(&mut e);
        e.register(LIST_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 5);
        assert_eq!(e.call(0x0083_b830, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this + 8), 0);
        let head = list(&mut e, &[0xa1, 0xa2], true);
        assert_eq!(e.call(0x0083_bac0, &args![head, 1u32]).u32(), 0xa2);
        assert_eq!(e.call(0x0083_bac0, &args![head, 0x1_0000u32]).u32(), 0xa1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_bac0, &args![head, 2u32]).u32(), 0);
        assert_eq!(
            calls(&e, PRINT_MESSAGE),
            vec![vec![INDEX_TOO_LARGE_MESSAGE]]
        );
    }

    #[test]
    fn save_size_counts_the_items() {
        let mut e = Engine::new();
        list_walking(&mut e);
        save_load_game(&mut e, true, true);
        e.register(ITEM_SAVE_SIZE, |_, a| (0xffff_0000 | a[0]).into_ret());
        let head = list(&mut e, &[0x10, 0x20], true);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0083_bb30, &args![head]).u32() & 0xffff,
            6 + 2 + 0x30 + 2
        );
        assert_eq!(
            calls(&e, TRACE_PRINT),
            vec![vec![TEXT_SAVE_SIZE, 0x3a, 0xc0, SOURCE_FILE_0107DB20]]
        );
        save_load_game(&mut e, false, false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_bb30, &args![head]).u32() & 0xffff, 0x34);
        assert!(calls(&e, TRACE_PRINT).is_empty());
    }

    #[test]
    fn save_writes_header_count_and_index() {
        let mut e = Engine::new();
        list_walking(&mut e);
        let game = save_load_game(&mut e, true, false);
        e.register(ITEM_SAVE, |_, _| Ret::default());
        let head = list(&mut e, &[0x10, 0x20], true);
        let this = head;
        // The buffer positions are addresses the count and size go into.
        let buffer = e.mem.alloc(0x20);
        e.mem.set_u32(game, buffer);
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x0083_bc80, &args![this]);
        let writes = calls(&e, SAVE_LOAD_WRITE);
        assert_eq!(writes.len(), 4);
        assert_eq!(
            writes.iter().map(|w| w[2]).collect::<Vec<_>>(),
            vec![4, 2, 2, 2]
        );
        assert_eq!(calls(&e, ITEM_SAVE), vec![vec![0x10], vec![0x20]]);
        // The count over its place (after header and size), the block size
        // from its place to the end.
        assert_eq!(e.mem.u16(buffer + 6), 2);
        assert_eq!(e.mem.u16(buffer + 4), 6);
    }

    #[test]
    fn load_game_reads_the_items_and_the_index() {
        let mut e = Engine::new();
        list_walking(&mut e);
        let game = save_load_game(&mut e, true, false);
        e.register(ALLOCATE, |_, _| 0x9000u32.into_ret());
        e.register(ITEM_CONSTRUCT, |_, a| (a[0] + 1).into_ret());
        e.register(ITEM_LOAD, |_, _| Ret::default());
        e.register(LIST_ADD, |_, _| Ret::default());
        e.register(SET_CURRENT_ITEM, |_, _| Ret::default());
        let queue = e.mem.alloc(0x20);
        // Header, block size, count, index.
        for (i, v) in (0u32..).zip([BLOCK_HEADER, 6, 2, 0xffff]) {
            e.mem.set_u32(queue + i * 4, v);
        }
        e.mem.set_u32(game + 8, queue);
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x0083_bec0, &args![this]);
        assert!(calls(&e, PRINT_MESSAGE).is_empty());
        assert_eq!(calls(&e, ITEM_LOAD), vec![vec![0x9001], vec![0x9001]]);
        assert_eq!(calls(&e, LIST_ADD).len(), 2);
        assert_eq!(e.mem.u32(this + 8), 0);

        // A wrong header and an overrun.
        e.mem.set_u32(game, 0);
        for (i, v) in (0u32..).zip([0x1234, 1, 0, 0]) {
            e.mem.set_u32(queue + i * 4, v);
        }
        e.mem.set_u32(game + 8, queue);
        e.mem.set_u32(this, 0);
        e.mem.set_u32(this + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0083_bec0, &args![this]);
        let messages = calls(&e, PRINT_MESSAGE);
        assert_eq!(
            messages[0],
            vec![TEXT_BAD_HEADER, SOURCE_FILE_0107DB20, 0xe7, 0x07]
        );
        // Index 0 of an empty list: the not-found message, then made current.
        assert_eq!(messages[1], vec![INDEX_TOO_LARGE_MESSAGE]);
        assert_eq!(calls(&e, SET_CURRENT_ITEM), vec![vec![this, 0]]);
        // Start 4, size 1: the end (10) is 5 past.
        assert_eq!(
            messages[2],
            vec![TEXT_OVERRUN, 5, SOURCE_FILE_0107DB20, 0xfd, 0x07]
        );
    }

    #[test]
    fn load_game_ov2_reads_from_the_buffer() {
        let mut e = Engine::new();
        list_walking(&mut e);
        e.register(BUFFER_COUNT, |_, _| 3u32.into_ret());
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.register(ITEM_LOAD_FROM, |_, _| Ret::default());
        e.register(LIST_ADD, |_, _| Ret::default());
        e.register(SET_CURRENT_ITEM, |_, _| Ret::default());
        e.register(BUFFER_READ, |e, a| {
            e.mem.set_u16(a[1], 1);
            Ret::default()
        });
        let this = list(&mut e, &[0xa1, 0xa2], false);
        e.call_log = Some(vec![]);
        e.call(0x0083_c330, &args![this, 0x4400u32]);
        assert_eq!(calls(&e, ITEM_LOAD_FROM), vec![vec![0, 0x4400]; 3]);
        assert!(calls(&e, ITEM_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, SET_CURRENT_ITEM), vec![vec![this, 0xa2]]);
        e.register(BUFFER_READ, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0083_c330, &args![this, 0x4400u32]);
        assert!(calls(&e, SET_CURRENT_ITEM).is_empty());
    }

    #[test]
    fn item_walks() {
        let mut e = Engine::new();
        list_walking(&mut e);
        e.register(ITEM_0083CDF0, |_, _| Ret::default());
        e.register(ITEM_0083D0D0, |_, _| Ret::default());
        let head = list(&mut e, &[0x10, 0, 0x30], false);
        e.call_log = Some(vec![]);
        e.call(0x0083_c220, &args![head]);
        // A node without an item but with a next one is not empty.
        assert_eq!(
            calls(&e, ITEM_0083CDF0),
            vec![vec![0x10], vec![0], vec![0x30]]
        );
        e.call(0x0083_c420, &args![head, 9u32]);
        assert_eq!(calls(&e, ITEM_0083D0D0), vec![vec![0x10, 9], vec![0x30, 9]]);
    }
}
