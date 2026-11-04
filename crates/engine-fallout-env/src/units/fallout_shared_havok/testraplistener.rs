//! `fallout shared/havok/testraplistener.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00620810` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const LIST_FIND_TARGET_00620810: u32 = 0x0062_0810;

// Translated from 0062db40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (only caller `0062dae5`): forwards `this` and one word to
/// `00620810`, which walks the chain starting at `this` (next link from `00726070`) until
/// `0044ddc0(node)` equals the word and returns that node, or null when the chain ends. The
/// wrapper returns it.
pub fn fn_0062db40(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.call(LIST_FIND_TARGET_00620810, &args![this, value]).ptr()
}

/// `0057c400` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const LIST_FIND_TARGET_0057C400: u32 = 0x0057_c400;

// Translated from 0062df60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (only caller `0062df33`): forwards `this` and one word to
/// `0057c400`, which walks the chain starting at `this` (next link from `00726070`) until
/// `0084e3a0(node)` equals the word and returns that node, or null when the chain ends. The
/// wrapper returns it.
pub fn fn_0062df60(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.call(LIST_FIND_TARGET_0057C400, &args![this, value]).ptr()
}

/// `00401000` (`cdecl`, one word: bytes) allocates a block; `00401030`
/// (`cdecl`, one word) frees one.
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// Folded word getters (`thiscall`, unnamed in the engine map): the words
/// at `+0x4` (`00726070`), `+0x8` (`0044ddc0`) and `+0xc` (`0084e3a0`; for a
/// reference `TESForm::iFormID`, which the reference lists keep).
const GET_WORD_4: u32 = 0x0072_6070;
const GET_WORD_8: u32 = 0x0044_ddc0;
const GET_WORD_C: u32 = 0x0084_e3a0;
/// `004839c0` (`cdecl`, one word: a form id): the form with that id, or
/// null.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `005d43c0` (`thiscall` on a reference): the word `Script::SetActionFlag`
/// gets as its second argument.
const REFERENCE_SCRIPT_WORD: u32 = 0x005d_43c0;
/// `Script::SetActionFlag` (Xbox PDB name of `005ac750`; `cdecl`: reference,
/// script word, flag), a byte result.
const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `TESHavokUtilities::FindCollidableRef` (Xbox PDB name of `0062b4e0`;
/// `cdecl`, one word).
const FIND_COLLIDABLE_REF: u32 = 0x0062_b4e0;
/// `0069b080` (`thiscall` on a `BSList`): whether its count at `+0xc` is 0.
const LIST_IS_EMPTY: u32 = 0x0069_b080;
/// `00622570` (`thiscall`, one word).
const STORE_00622570: u32 = 0x0062_2570;
/// The object `0042ce10` tests (`thiscall`, a byte) is held at `011ddf38`.
const OBJECT_POINTER_011DDF38: u32 = 0x011d_df38;
const OBJECT_TEST: u32 = 0x0042_ce10;
/// `00483710` (`thiscall`, `fallout shared/tesform.cpp`): run on the lock
/// word around the phantom calls of `0062cc90`, and by `0062e170`.
const CALL_00483710: u32 = 0x0048_3710;

/// The vtables of this unit's classes (RTTI names): `TESTrapListener`
/// (`0104e37c`, `0104e350`, `0104e338` at `+0`, `+0x8`, `+0x1c`),
/// `bhkTrapListener` (`0104e3a4`, `0104e38c` at `+0`, `+0x14`),
/// `hkpPhantomListener` (`0104e3d0`), `BSList<TriggerEntry::TriggerEvent*>`
/// (`0104e2f8`), `BSPointerListBase<..>` (`0104e308`), `BSListBase<..>`
/// (`0104e3f4`), `hkpAllCdBodyPairCollector` (`0104e318`) and
/// `hkpCdBodyPairCollector` (`0104e328`).
const TRAP_LISTENER_VTABLE: u32 = 0x0104_e37c;
const TRAP_LISTENER_VTABLE_8: u32 = 0x0104_e350;
const TRAP_LISTENER_VTABLE_1C: u32 = 0x0104_e338;
const BHK_TRAP_LISTENER_VTABLE: u32 = 0x0104_e3a4;
const BHK_TRAP_LISTENER_VTABLE_14: u32 = 0x0104_e38c;
const PHANTOM_LISTENER_VTABLE: u32 = 0x0104_e3d0;
const EVENT_LIST_VTABLE: u32 = 0x0104_e2f8;
const EVENT_POINTER_LIST_VTABLE: u32 = 0x0104_e308;
const EVENT_LIST_BASE_VTABLE: u32 = 0x0104_e3f4;
const ALL_PAIR_COLLECTOR_VTABLE: u32 = 0x0104_e318;
const PAIR_COLLECTOR_VTABLE: u32 = 0x0104_e328;

/// The reference list (`TriggerEntry::RefList`, the class of `0062c860` in
/// the Xbox PDB): entries at `+0` (8 bytes each: a form id, then a byte
/// flag), their count at `+4` and capacity at `+8`, and a `BSList` of
/// events at `+0xc`.
const ENTRY_SIZE: u32 = 8;
const REF_LIST_EVENTS: u32 = 0xc;
/// The reference list's place in a trap entry.
const ENTRY_REF_LIST: u32 = 0x10;

/// The address `offset` bytes into `object`.
fn at(object: Ptr, offset: u32) -> u32 {
    object.addr().wrapping_add(offset)
}

/// Entry `index` of the reference list at `list` (its address).
fn ref_entry(e: &Engine, list: Ptr, index: u32) -> u32 {
    e.mem
        .u32(list.addr())
        .wrapping_add(index.wrapping_mul(ENTRY_SIZE))
}

// Translated from 0062d6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `hkpPhantomListener` constructor (unnamed in the engine map;
/// `thiscall`, plain `RET`): stores the vtable `0104e3d0`. Returns `this`.
pub fn fn_0062d6f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), PHANTOM_LISTENER_VTABLE);
    this
}

// Translated from 0062d680 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `bhkTrapListener` constructor (unnamed in the engine map; `thiscall`,
/// plain `RET`): the `bhkEntityListener` constructor `0061f4d0` with 0, the
/// `hkpPhantomListener` constructor on `this + 0x14`, then the vtables
/// `0104e3a4` (`+0`) and `0104e38c` (`+0x14`). Returns `this`.
pub fn fn_0062d680(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0061_f4d0, &args![this, 0u32]);
    fn_0062d6f0(e, Ptr::new(at(this, 0x14)));
    e.mem.set_u32(this.addr(), BHK_TRAP_LISTENER_VTABLE);
    e.mem.set_u32(at(this, 0x14), BHK_TRAP_LISTENER_VTABLE_14);
    this
}

// Translated from 0062d5c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062d5c0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// A `TESTrapListener` constructor (unnamed in the engine map; `thiscall`,
/// plain `RET`): the base constructor `0061ffb0`, the `bhkTrapListener`
/// constructor on `this + 8`, the three `TESTrapListener` vtables at `+0`,
/// `+0x8` and `+0x1c`, and the entry list at `+0x20` cleared. Returns
/// `this`.
pub fn fn_0062d5c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0061_ffb0, &args![this]);
    fn_0062d680(e, Ptr::new(at(this, 8)));
    e.mem.set_u32(this.addr(), TRAP_LISTENER_VTABLE);
    e.mem.set_u32(at(this, 8), TRAP_LISTENER_VTABLE_8);
    e.mem.set_u32(at(this, 0x1c), TRAP_LISTENER_VTABLE_1C);
    e.mem.set_u32(at(this, 0x20), 0);
    this
}

// Translated from 0062d990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the next trap entry,
/// the word getter `00726070` on `this`.
pub fn fn_0062d990(e: &mut Engine, this: Ptr) -> u32 {
    e.call(GET_WORD_4, &args![this]).u32()
}

// Translated from 0062d2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): `this + 0x160`.
pub fn fn_0062d2f0(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(at(this, 0x160))
}

// Translated from 0062e150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the address of 16-byte
/// element `index` of the array whose base is at `this`.
pub fn fn_0062e150(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    index.wrapping_shl(4).wrapping_add(e.mem.u32(this.addr()))
}

// Translated from 0062e120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): `00622570` on `out`
/// with whether the word at `this + 4` is zero; returns `out`.
pub fn fn_0062e120(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let empty = u32::from(e.mem.u32(at(this, 4)) == 0);
    e.call(STORE_00622570, &args![out, empty]);
    out
}

// Translated from 0062dfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `BSListBase<TriggerEntry::TriggerEvent*>` constructor (unnamed in the
/// engine map; `thiscall`, plain `RET`): the vtable `0104e3f4`; count
/// (`+0xc`), head (`+4`) and tail (`+8`) cleared. Returns `this`.
pub fn fn_0062dfa0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), EVENT_LIST_BASE_VTABLE);
    e.mem.set_u32(at(this, 0xc), 0);
    e.mem.set_u32(at(this, 4), 0);
    e.mem.set_u32(at(this, 8), 0);
    this
}

// Translated from 0062c750 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `BSPointerListBase<TriggerEntry::TriggerEvent*>` constructor (unnamed
/// in the engine map; `thiscall`, plain `RET`): the base constructor, then
/// the vtable `0104e308`. Returns `this`.
pub fn fn_0062c750(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0062dfa0(e, this);
    e.mem.set_u32(this.addr(), EVENT_POINTER_LIST_VTABLE);
    this
}

// Translated from 0062c6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `BSList<TriggerEntry::TriggerEvent*>` constructor (unnamed in the
/// engine map; `thiscall`, plain `RET`): the base constructor, then the
/// vtable `0104e2f8`. Returns `this`.
pub fn fn_0062c6e0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0062c750(e, this);
    e.mem.set_u32(this.addr(), EVENT_LIST_VTABLE);
    this
}

// Translated from 0062c640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062c640 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The reference list constructor (unnamed in the engine map; `thiscall`,
/// plain `RET`): the event list at `+0xc`, capacity 0x14, count 0, and an
/// entry block of `capacity * 8` bytes (all ones when that overflows, as
/// the exe's `SETO`/`NEG`/`OR`). Returns `this`.
pub fn fn_0062c640(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0062c6e0(e, Ptr::new(at(this, REF_LIST_EVENTS)));
    e.mem.set_u32(at(this, 8), 0x14);
    e.mem.set_u32(at(this, 4), 0);
    let bytes = e
        .mem
        .u32(at(this, 8))
        .checked_mul(ENTRY_SIZE)
        .unwrap_or(u32::MAX);
    let block = e.call(ALLOCATE, &args![bytes]).u32();
    e.mem.set_u32(this.addr(), block);
    this
}

// Translated from 0062dc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// A trap entry constructor (unnamed in the engine map; `thiscall`,
/// `RET 8`): the base constructor `00620720` with both words, then the
/// reference list at `+0x10`. Returns `this`.
pub fn fn_0062dc50(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(0x0062_0720, &args![this, first, second]);
    fn_0062c640(e, Ptr::new(at(this, ENTRY_REF_LIST)));
    this
}

// Translated from 0062e2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): links `node` at the
/// tail of the list (node: next `+0`, previous `+4`; list: head `+4`, tail
/// `+8`, count `+0xc`) and counts it.
pub fn fn_0062e2f0(e: &mut Engine, this: Ptr, node: Ptr) {
    e.mem.set_u32(node.addr(), 0);
    let tail = e.mem.u32(at(this, 8));
    e.mem.set_u32(at(node, 4), tail);
    if e.mem.u32(at(this, 8)) != 0 {
        let tail = e.mem.u32(at(this, 8));
        e.mem.set_u32(tail, node.addr());
    } else {
        e.mem.set_u32(at(this, 4), node.addr());
    }
    e.mem.set_u32(at(this, 8), node.addr());
    let count = e.mem.u32(at(this, 0xc)).wrapping_add(1);
    e.mem.set_u32(at(this, 0xc), count);
}

// Translated from 0062e060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): a new node from the
/// list's virtual slot `+4`, the word at `value` as its item (`+8`), linked
/// at the tail (`0062e2f0`).
pub fn fn_0062e060(e: &mut Engine, this: Ptr, value: Ptr) {
    let node = e.vcall(this.addr(), 4, &[]).u32();
    let item = e.mem.u32(value.addr());
    e.mem.set_u32(node.wrapping_add(8), item);
    fn_0062e2f0(e, this, Ptr::new(node));
}

// Translated from 0062e0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): unlinks the head
/// node (the new head's previous link cleared, or the tail when the list
/// is now empty), hands the node to the list's virtual slot `+8`, lowers
/// the count and returns the node's item.
pub fn fn_0062e0a0(e: &mut Engine, this: Ptr) -> u32 {
    let node = e.mem.u32(at(this, 4));
    let next = e.mem.u32(node);
    e.mem.set_u32(at(this, 4), next);
    if e.mem.u32(at(this, 4)) != 0 {
        let head = e.mem.u32(at(this, 4));
        e.mem.set_u32(head.wrapping_add(4), 0);
    } else {
        e.mem.set_u32(at(this, 8), 0);
    }
    let item = e.mem.u32(node.wrapping_add(8));
    e.vcall(this.addr(), 8, &args![node]);
    let count = e.mem.u32(at(this, 0xc)).wrapping_sub(1);
    e.mem.set_u32(at(this, 0xc), count);
    item
}

// Translated from 0062cb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`; a reference list):
/// the next event taken off the event list at `+0xc` (`0062e0a0`), or null
/// when it is empty (`0069b080`).
pub fn fn_0062cb60(e: &mut Engine, this: Ptr) -> u32 {
    let events = Ptr::new(at(this, REF_LIST_EVENTS));
    if e.call(LIST_IS_EMPTY, &args![events]).bool() {
        return 0;
    }
    fn_0062e0a0(e, events)
}

/// A new 8-byte event (`id` at `+0`, `kind` at `+4`) queued on the reference
/// list's event list through a stack word holding its address.
fn queue_event(e: &mut Engine, list: Ptr, id: u32, kind: u32) {
    let event = e.call(ALLOCATE, &args![8u32]).u32();
    e.mem.set_u32(event.wrapping_add(4), kind);
    e.mem.set_u32(event, id);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), event);
        fn_0062e060(e, Ptr::new(at(list, REF_LIST_EVENTS)), slot);
    });
}

// Translated from 0062cb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`; a reference list):
/// each entry whose flag is clear queues a kind-2 event with its id and is
/// removed (the later entries move down one); each entry whose flag is set
/// has it cleared.
pub fn fn_0062cb90(e: &mut Engine, this: Ptr) {
    let mut i = 0i32;
    while i < e.mem.i32(at(this, 4)) {
        let entry = ref_entry(e, this, i as u32);
        if e.mem.u8(entry.wrapping_add(4)) == 0 {
            let id = e.mem.u32(ref_entry(e, this, i as u32));
            queue_event(e, this, id, 2);
            let mut j = i;
            while j < e.mem.i32(at(this, 4)).wrapping_sub(1) {
                let from = ref_entry(e, this, (j as u32).wrapping_add(1));
                let to = ref_entry(e, this, j as u32);
                let id = e.mem.u32(from);
                e.mem.set_u32(to, id);
                let flag = e.mem.u8(from.wrapping_add(4));
                e.mem.set_u8(to.wrapping_add(4), flag);
                j = j.wrapping_add(1);
            }
            let count = e.mem.i32(at(this, 4)).wrapping_sub(1);
            e.mem.set_i32(at(this, 4), count);
        } else {
            e.mem.set_u8(entry.wrapping_add(4), 0);
            i = i.wrapping_add(1);
        }
    }
}

// Translated from 0062c9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; a reference list and a
/// reference): marks the reference as present. When its id (`0084e3a0`) is
/// listed: sets the entry's flag and answers true if it was clear, answers
/// false if it was set. Otherwise the id is appended with its flag set
/// (the entry block doubles when full: a new block, the old entries copied,
/// the old block freed), a kind-1 event with the id is queued, and it
/// answers false.
pub fn fn_0062c9d0(e: &mut Engine, this: Ptr, reference: Ptr) -> bool {
    let id = e.call(GET_WORD_C, &args![reference]).u32();
    let mut i = 0i32;
    while i < e.mem.i32(at(this, 4)) {
        let entry = ref_entry(e, this, i as u32);
        if e.mem.u32(entry) == id {
            if e.mem.u8(entry.wrapping_add(4)) == 0 {
                e.mem.set_u8(entry.wrapping_add(4), 1);
                return true;
            }
            return false;
        }
        i = i.wrapping_add(1);
    }
    if e.mem.u32(at(this, 4)) == e.mem.u32(at(this, 8)) {
        let capacity = e.mem.u32(at(this, 8)).wrapping_shl(1);
        let bytes = capacity.checked_mul(ENTRY_SIZE).unwrap_or(u32::MAX);
        let block = e.call(ALLOCATE, &args![bytes]).u32();
        let mut j = 0i32;
        while j < e.mem.i32(at(this, 8)) {
            let from = ref_entry(e, this, j as u32);
            let to = block.wrapping_add((j as u32).wrapping_mul(ENTRY_SIZE));
            let id = e.mem.u32(from);
            e.mem.set_u32(to, id);
            let flag = e.mem.u8(from.wrapping_add(4));
            e.mem.set_u8(to.wrapping_add(4), flag);
            j = j.wrapping_add(1);
        }
        e.mem.set_u32(at(this, 8), capacity);
        let old = e.mem.u32(this.addr());
        e.call(FREE, &args![old]);
        e.mem.set_u32(this.addr(), block);
    }
    let count = e.mem.u32(at(this, 4));
    let entry = ref_entry(e, this, count);
    e.mem.set_u32(entry, id);
    let count = e.mem.u32(at(this, 4));
    let entry = ref_entry(e, this, count);
    e.mem.set_u8(entry.wrapping_add(4), 1);
    let count = e.mem.u32(at(this, 4)).wrapping_add(1);
    e.mem.set_u32(at(this, 4), count);
    queue_event(e, this, id, 1);
    false
}

// Translated from 0062d570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; a reference list):
/// whether the reference's id (`0084e3a0`) is listed.
pub fn fn_0062d570(e: &mut Engine, this: Ptr, reference: Ptr) -> bool {
    let id = e.call(GET_WORD_C, &args![reference]).u32();
    let mut i = 0i32;
    while i < e.mem.i32(at(this, 4)) {
        if e.mem.u32(ref_entry(e, this, i as u32)) == id {
            return true;
        }
        i = i.wrapping_add(1);
    }
    false
}

// Translated from 0062d550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; a trap entry): whether
/// its reference list (`+0x10`) lists the reference (`0062d570`).
pub fn fn_0062d550(e: &mut Engine, this: Ptr, reference: Ptr) -> bool {
    fn_0062d570(e, Ptr::new(at(this, ENTRY_REF_LIST)), reference)
}

// Translated from 0062df20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`; a trap listener): finds
/// the entry for `key` in the chain at `+0x20` (`0062df60`) and answers
/// whether it lists `reference` (`0062d550`); false without an entry.
pub fn fn_0062df20(e: &mut Engine, this: Ptr, reference: Ptr, key: u32) -> bool {
    let chain = Ptr::new(e.mem.u32(at(this, 0x20)));
    let entry = fn_0062df60(e, chain, key);
    if entry.is_null() {
        return false;
    }
    fn_0062d550(e, entry, reference)
}

// Translated from 0062c860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TriggerEntry::RefList::ForceLeaveTrigger` (Xbox PDB; `thiscall`,
/// `RET 4`): nothing for an empty list. Otherwise copies the listed ids
/// into a new block, and for each id whose form (`004839c0`) exists and
/// passes its virtual slot `+0xf0`: `Script::SetActionFlag(form, word,
/// 0x40000000)` (`word` from `005d43c0` of `trigger`), `00572e10` on the
/// trigger with the form and `TESObjectREFR::RunScript` (`00565870`) on the
/// trigger. The block is freed.
pub fn trigger_entry_ref_list_force_leave_trigger(e: &mut Engine, this: Ptr, trigger: Ptr) {
    if e.mem.u32(at(this, 4)) == 0 {
        return;
    }
    let bytes = e.mem.u32(at(this, 4)).checked_mul(4).unwrap_or(u32::MAX);
    let ids = e.call(ALLOCATE, &args![bytes]).u32();
    let count = e.mem.u32(at(this, 4));
    let mut i = 0i32;
    while i < e.mem.i32(at(this, 4)) {
        let id = e.mem.u32(ref_entry(e, this, i as u32));
        e.mem
            .set_u32(ids.wrapping_add((i as u32).wrapping_mul(4)), id);
        i = i.wrapping_add(1);
    }
    let word = e.call(REFERENCE_SCRIPT_WORD, &args![trigger]).u32();
    let mut j = 0u32;
    while j < count {
        let id = e.mem.u32(ids.wrapping_add(j.wrapping_mul(4)));
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        if form != 0 && e.vcall(form, 0xf0, &[]).bool() {
            e.call(SET_ACTION_FLAG, &args![form, word, 0x4000_0000u32]);
            e.call(0x0057_2e10, &args![trigger, form]);
            e.call(0x0056_5870, &args![trigger]);
        }
        j = j.wrapping_add(1);
    }
    e.call(FREE, &args![ids]);
}

// Translated from 0062c980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`; a reference list):
/// clears the count and frees every queued event.
pub fn fn_0062c980(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(at(this, 4), 0);
    let events = Ptr::new(at(this, REF_LIST_EVENTS));
    while !e.call(LIST_IS_EMPTY, &args![events]).bool() {
        let event = fn_0062e0a0(e, events);
        e.call(FREE, &args![event]);
    }
}

// Translated from 0062e040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSListBase<TriggerEntry::TriggerEvent*>` destructor (unnamed in the
/// engine map; `thiscall`, plain `RET`): stores the vtable `0104e3f4`.
pub fn fn_0062e040(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), EVENT_LIST_BASE_VTABLE);
}

// Translated from 0062dfe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062dfe0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The `BSPointerListBase<TriggerEntry::TriggerEvent*>` destructor (unnamed
/// in the engine map; `thiscall`, plain `RET`): the vtable `0104e308`,
/// `0062e290` on the list, then the base destructor (`0062e040`).
pub fn fn_0062dfe0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), EVENT_POINTER_LIST_VTABLE);
    e.call(0x0062_e290, &args![this]);
    fn_0062e040(e, this);
}

// Translated from 0062c700 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSList<TriggerEntry::TriggerEvent*>` destructor (unnamed in the
/// engine map; `thiscall`, plain `RET`): forwards to `0062dfe0`.
pub fn fn_0062c700(e: &mut Engine, this: Ptr) {
    fn_0062dfe0(e, this);
}

// Translated from 0062c7a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062c7a0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The reference list destructor (unnamed in the engine map; `thiscall`,
/// plain `RET`): clears the count, frees the entry block (when not null)
/// and clears it, frees every queued event, then destroys the event list
/// (`0062c700`).
pub fn fn_0062c7a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(at(this, 4), 0);
    if e.mem.u32(this.addr()) != 0 {
        let block = e.mem.u32(this.addr());
        e.call(FREE, &args![block]);
    }
    e.mem.set_u32(this.addr(), 0);
    let events = Ptr::new(at(this, REF_LIST_EVENTS));
    while !e.call(LIST_IS_EMPTY, &args![events]).bool() {
        let event = fn_0062e0a0(e, events);
        e.call(FREE, &args![event]);
    }
    fn_0062c700(e, events);
}

// Translated from 0062d9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The trap entry destructor (unnamed in the engine map; `thiscall`, plain
/// `RET`): destroys the reference list at `+0x10` (`0062c7a0`).
pub fn fn_0062d9e0(e: &mut Engine, this: Ptr) {
    fn_0062c7a0(e, Ptr::new(at(this, ENTRY_REF_LIST)));
}

// Translated from 0062d9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The trap entry's scalar deleting destructor (unnamed in the engine map;
/// `thiscall`, `RET 4`): the destructor, then `00401030` frees `this` when
/// bit 0 of `flags` is set. Returns `this`.
pub fn fn_0062d9b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0062d9e0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0062d510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`; a trap entry): unless
/// `0042ce10` answers true for the object at `011ddf38`, its references are
/// forced out of the trigger (`ForceLeaveTrigger` with the entry's
/// `0084e3a0` word); then the list is emptied (`0062c980`).
pub fn fn_0062d510(e: &mut Engine, this: Ptr) {
    let object = e.mem.u32(OBJECT_POINTER_011DDF38);
    let list = Ptr::new(at(this, ENTRY_REF_LIST));
    if !e.call(OBJECT_TEST, &args![object]).bool() {
        let trigger = e.call(GET_WORD_C, &args![this]).ptr();
        trigger_entry_ref_list_force_leave_trigger(e, list, trigger);
    }
    fn_0062c980(e, list);
}

// Translated from 0062de90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTrapListener::ClearCurrentRefs` (Xbox PDB; `thiscall`, `RET 4`):
/// walks the entries from `+0x20` (`0062d990` for the next) and clears the
/// first whose `0084e3a0` word is `key` (`0062d510`).
pub fn tes_trap_listener_clear_current_refs(e: &mut Engine, this: Ptr, key: u32) {
    let mut entry = e.mem.u32(at(this, 0x20));
    while entry != 0 {
        if e.call(GET_WORD_C, &args![entry]).u32() == key {
            fn_0062d510(e, Ptr::new(entry));
            return;
        }
        entry = fn_0062d990(e, Ptr::new(entry));
    }
}

// Translated from 0062d920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`; a trap listener):
/// deletes every entry of the chain at `+0x20` (`0062d9b0` with 1, the next
/// one read first) and clears the chain.
pub fn fn_0062d920(e: &mut Engine, this: Ptr) {
    let mut entry = e.mem.u32(at(this, 0x20));
    while entry != 0 {
        let next = fn_0062d990(e, Ptr::new(entry));
        if entry != 0 {
            fn_0062d9b0(e, Ptr::new(entry), 1);
        }
        entry = next;
    }
    e.mem.set_u32(at(this, 0x20), 0);
}

// Translated from 0062d740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `hkpPhantomListener` destructor (unnamed in the engine map;
/// `thiscall`, plain `RET`): stores the vtable `0104e3d0`.
pub fn fn_0062d740(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), PHANTOM_LISTENER_VTABLE);
}

// Translated from 0062d760 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062d760 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The `bhkTrapListener` destructor (unnamed in the engine map; `thiscall`,
/// plain `RET`): the `hkpPhantomListener` destructor on `this + 0x14`, then
/// the base destructor `005545f0`.
pub fn fn_0062d760(e: &mut Engine, this: Ptr) {
    fn_0062d740(e, Ptr::new(at(this, 0x14)));
    e.call(0x0055_45f0, &args![this]);
}

// Translated from 0062d8a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062d8a0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The `TESTrapListener` destructor (unnamed in the engine map; `thiscall`,
/// plain `RET`): the three vtables, the entries deleted (`0062d920`), the
/// `bhkTrapListener` destructor on `this + 8`, then the base destructor
/// `00620110`.
pub fn fn_0062d8a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TRAP_LISTENER_VTABLE);
    e.mem.set_u32(at(this, 8), TRAP_LISTENER_VTABLE_8);
    e.mem.set_u32(at(this, 0x1c), TRAP_LISTENER_VTABLE_1C);
    fn_0062d920(e, this);
    fn_0062d760(e, Ptr::new(at(this, 8)));
    e.call(0x0062_0110, &args![this]);
}

// Translated from 0062d870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTrapListener::scalar deleting destructor` (Xbox PDB; `thiscall`,
/// `RET 4`): the destructor, then `00401030` frees `this` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn tes_trap_listener_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0062d8a0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0062d330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCdBodyPairCollector::reset` (Xbox PDB; `thiscall`, plain `RET`):
/// `00622570` on `this + 4` with 0.
pub fn hkp_cd_body_pair_collector_reset(e: &mut Engine, this: Ptr) {
    e.call(STORE_00622570, &args![at(this, 4), 0u32]);
}

// Translated from 0062d3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `hkpCdBodyPairCollector` constructor (unnamed in the engine map;
/// `thiscall`, plain `RET`): the vtable `0104e328`, `006815c0` on
/// `this + 4`, then `reset`. Returns `this`.
pub fn fn_0062d3c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), PAIR_COLLECTOR_VTABLE);
    e.call(0x0068_15c0, &args![at(this, 4)]);
    hkp_cd_body_pair_collector_reset(e, this);
    this
}

// Translated from 0062e1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): `0062e230` on `this`
/// with its inline storage `this + 0xc`, `value` and 16. Returns `this`.
pub fn fn_0062e1d0(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.call(0x0062_e230, &args![this, at(this, 0xc), value, 0x10u32]);
    this
}

// Translated from 0062d310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAllCdBodyPairCollector::reset` (Xbox PDB; `thiscall`, plain `RET`):
/// `00626870` on the hit array at `this + 8`, then the base `reset`.
pub fn hkp_all_cd_body_pair_collector_reset(e: &mut Engine, this: Ptr) {
    e.call(0x0062_6870, &args![at(this, 8)]);
    hkp_cd_body_pair_collector_reset(e, this);
}

// Translated from 0062d350 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062d350 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// An `hkpAllCdBodyPairCollector` constructor (unnamed in the engine map;
/// `thiscall`, plain `RET`): the base constructor, the vtable `0104e318`,
/// the hit array at `+8` (`0062e1d0` with 0), then `reset`. Returns `this`.
pub fn fn_0062d350(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0062d3c0(e, this);
    e.mem.set_u32(this.addr(), ALL_PAIR_COLLECTOR_VTABLE);
    fn_0062e1d0(e, Ptr::new(at(this, 8)), 0);
    hkp_all_cd_body_pair_collector_reset(e, this);
    this
}

// Translated from 0062e260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): `0062e350` on
/// `this` with what `004a4840` answers for a zeroed stack byte and `this`.
pub fn fn_0062e260(e: &mut Engine, this: Ptr) {
    e.with_stack(4, |e, byte| {
        e.mem.set_u8(byte.addr(), 0);
        let value = e.call(0x004a_4840, &args![byte, this]).u32();
        e.call(0x0062_e350, &args![this, value]);
    });
}

// Translated from 0062e170 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062e170 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The hit array's destructor body (unnamed in the engine map; `thiscall`,
/// plain `RET`): `0062e260`, then `00483710` on `this`.
pub fn fn_0062e170(e: &mut Engine, this: Ptr) {
    fn_0062e260(e, this);
    e.call(CALL_00483710, &args![this]);
}

// Translated from 0062df80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): forwards to
/// `0062e170`.
pub fn fn_0062df80(e: &mut Engine, this: Ptr) {
    fn_0062e170(e, this);
}

// Translated from 0062d420 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `hkpCdBodyPairCollector` destructor (unnamed in the engine map;
/// `thiscall`, plain `RET`): stores the vtable `0104e328`.
pub fn fn_0062d420(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), PAIR_COLLECTOR_VTABLE);
}

// Translated from 0062d4b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062d4b0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `hkpAllCdBodyPairCollector::~hkpAllCdBodyPairCollector` (Xbox PDB;
/// `thiscall`, plain `RET`): the vtable `0104e318`, the hit array at `+8`
/// destroyed (`0062df80`), then the base destructor (`0062d420`).
pub fn hkp_all_cd_body_pair_collector_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ALL_PAIR_COLLECTOR_VTABLE);
    fn_0062df80(e, Ptr::new(at(this, 8)));
    fn_0062d420(e, this);
}

// Translated from 0062ddb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): `008d6f30` of the object,
/// `004543c0` of that, `007af430` of that, less 8 (null stays null); null
/// as soon as one step answers null.
pub fn fn_0062ddb0(e: &mut Engine, object: Ptr) -> u32 {
    if object.is_null() {
        return 0;
    }
    let first = e.call(0x008d_6f30, &args![object]).u32();
    if first == 0 {
        return 0;
    }
    let second = e.call(0x0045_43c0, &args![first]).u32();
    if second == 0 {
        return 0;
    }
    let third = e.call(0x007a_f430, &args![second]).u32();
    if third != 0 {
        third.wrapping_sub(8)
    } else {
        0
    }
}

/// The trigger kinds (`0043b4d0` of the `008c71b0` value of the object's
/// `0043b540`) that `0062db60` and `0062da90` act on, and those `0062da00`
/// acts on.
fn is_trap_kind(kind: u32) -> bool {
    kind == 0xc || kind == 0x16 || kind == 0x18
}

/// `00460140` of `source` (the object), then `0043b540` of it through
/// `008c71b0` into a stack word, and `0043b4d0` of that word (the kind).
/// `before_kind` runs between the stack word's set-up and the kind read; a
/// `false` from it ends the call without reading the kind.
fn object_and_kind(
    e: &mut Engine,
    source: u32,
    before_kind: impl FnOnce(&mut Engine) -> bool,
) -> Option<(u32, u32)> {
    let object = e.call(0x0046_0140, &args![source]).u32();
    e.with_stack(4, |e, word| {
        let value = e.call(0x0043_b540, &args![object]).u32();
        e.call(0x008c_71b0, &args![word, value]);
        if !before_kind(e) {
            return None;
        }
        let kind = e.call(0x0043_b4d0, &args![word]).u32();
        Some((object, kind))
    })
}

// Translated from 0062db60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062db60 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`thiscall`, `RET 4`; the listener at
/// `TESTrapListener + 8`, its entry chain at `+4`): for an object of kind
/// 0xc, 0x16 or 0x18 with a collidable reference (`FindCollidableRef`), a
/// new trap entry (0x2c bytes, `0062dc50` with the reference and the
/// object; null when the allocation fails) is put in front of the chain
/// (`006ecd40` on the entry with the old head) and becomes its head.
pub fn fn_0062db60(e: &mut Engine, this: Ptr, source: u32) {
    let Some((object, kind)) = object_and_kind(e, source, |_| true) else {
        return;
    };
    if !is_trap_kind(kind) {
        return;
    }
    let reference = e.call(FIND_COLLIDABLE_REF, &args![object]).u32();
    if reference == 0 {
        return;
    }
    let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
    let entry = if block != 0 {
        fn_0062dc50(e, Ptr::new(block), reference, object).addr()
    } else {
        0
    };
    let head = e.mem.u32(at(this, 4));
    e.call(0x006e_cd40, &args![entry, head]);
    e.mem.set_u32(at(this, 4), entry);
}

// Translated from 0062da90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; the same listener): with
/// an entry chain at `+4` and an object of kind 0xc, 0x16 or 0x18, the
/// object's entry (`0062db40`) is unlinked (`006207a0` answers the new
/// head) and deleted (`0062d9b0` with 1).
pub fn fn_0062da90(e: &mut Engine, this: Ptr, source: u32) {
    let Some((object, kind)) = object_and_kind(e, source, |e| e.mem.u32(at(this, 4)) != 0) else {
        return;
    };
    if !is_trap_kind(kind) {
        return;
    }
    let chain = Ptr::new(e.mem.u32(at(this, 4)));
    let entry = fn_0062db40(e, chain, object);
    if entry.is_null() {
        return;
    }
    let head = e.mem.u32(at(this, 4));
    let new_head = e.call(0x0062_07a0, &args![entry, head]).u32();
    e.mem.set_u32(at(this, 4), new_head);
    // The exe tests the entry again before deleting it (always non-null
    // here).
    if !entry.is_null() {
        fn_0062d9b0(e, entry, 1);
    }
}

// Translated from 0062da00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; the listener at
/// `+8` of its owner): for an object of kind 0xe, 0x10 or 0x1e with a
/// collidable reference, the owner (`this - 8`) gets `00576800` (kinds 0xe
/// and 0x10) or `BGSZoneTargetListener::RemoveTarget` (`00620130`) with
/// the reference.
pub fn fn_0062da00(e: &mut Engine, this: Ptr, source: u32) {
    let Some((object, kind)) = object_and_kind(e, source, |_| true) else {
        return;
    };
    if kind != 0xe && kind != 0x10 && kind != 0x1e {
        return;
    }
    let reference = e.call(FIND_COLLIDABLE_REF, &args![object]).u32();
    if reference == 0 {
        return;
    }
    let owner = this.addr().wrapping_sub(8);
    if kind == 0xe || kind == 0x10 {
        e.call(0x0057_6800, &args![owner, reference]);
    } else {
        e.call(0x0062_0130, &args![owner, reference]);
    }
}

/// One collided body of `0062cc90`: its collidable reference, when the
/// reference's slot `+0x100` and `004b59f0` do not both pass while neither
/// slot `+0x22c` (with 0) nor `+0x230` does, is marked present
/// (`0062c9d0`); a newly present reference gets
/// `Script::SetActionFlag(.., 0x10000000)`, whose byte is returned (0
/// otherwise).
fn trap_body_entered(e: &mut Engine, this: Ptr, body: u32) -> u8 {
    if body == 0 {
        return 0;
    }
    let reference = e.call(FIND_COLLIDABLE_REF, &args![body]).u32();
    if reference == 0 {
        return 0;
    }
    let ignorable =
        e.vcall(reference, 0x100, &[]).bool() && e.call(0x004b_59f0, &args![body]).u32() != 0;
    let forced =
        e.vcall(reference, 0x22c, &args![0u32]).bool() || e.vcall(reference, 0x230, &[]).bool();
    if ignorable && !forced {
        return 0;
    }
    if !fn_0062c9d0(e, Ptr::new(at(this, ENTRY_REF_LIST)), Ptr::new(reference)) {
        return 0;
    }
    let owner = e.call(GET_WORD_C, &args![this]).u32();
    let word = e.call(REFERENCE_SCRIPT_WORD, &args![owner]).u32();
    e.call(SET_ACTION_FLAG, &args![reference, word, 0x1000_0000u32])
        .u8()
}

/// `0062e120` on `list` into a stack byte, then `005609f0` on the result:
/// true when there is nothing to look at.
fn nothing_to_check(e: &mut Engine, list: u32) -> bool {
    e.with_stack(4, |e, flag| {
        let out = fn_0062e120(e, Ptr::new(list), flag);
        e.call(0x0056_09f0, &args![out]).bool()
    })
}

// Translated from 0062cc90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062cc90 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`thiscall`, `RET 4`; a trap entry; the float
/// word it is given is never read). Answers 0 when its reference (the
/// `+0xc` word) is null, when `00440da0` or `00440d80` passes for it or its
/// slot `+0x1d0` answers null. Otherwise, for the entry's phantom
/// (`004b59d0` of its `+8` word; the phantom's `+8` word, when not null,
/// gets `00483710` before and after each call to the phantom):
/// - a phantom whose slot `+0x14` answers 1: unless `0062e120`/`005609f0`
///   on `phantom + 0x160` says there is nothing to check, a stack
///   `hkpAllCdBodyPairCollector` collects its pairs (slot `+0x44`) and the
///   body (`+8`) of each 16-byte hit (from `00413f40`) is checked;
/// - one whose slot `+0x14` (asked again) answers 0: the bodies from
///   `00629ab0` of `0045c650`'s list are checked.
///
/// A checked body may mark its reference present (`0062c9d0`) and raise
/// `Script::SetActionFlag(.., 0x10000000)`. Then the reference list's
/// entries are aged (`0062cb90`), and one queued event (`0062cb60`) is
/// handled: its form (`004839c0`) gets `SetActionFlag` with `0x20000000`
/// (kind 1) or `0x40000000`, then `00572e10` of the entry's reference with
/// it; the event is freed first. The answer ORs the bytes those
/// `SetActionFlag` calls return.
pub fn fn_0062cc90(e: &mut Engine, this: Ptr, _unused_1: u32) -> u8 {
    let mut changed = 0u8;
    let reference = e.call(GET_WORD_C, &args![this]).u32();
    if reference == 0
        || e.call(0x0044_0da0, &args![reference]).bool()
        || e.call(0x0044_0d80, &args![reference]).bool()
        || e.vcall(reference, 0x1d0, &[]).u32() == 0
    {
        return changed;
    }
    let world = e.call(GET_WORD_8, &args![this]).u32();
    let phantom = e.call(0x004b_59d0, &args![world]).u32();
    let lock = e.call(GET_WORD_8, &args![phantom]).u32();
    let lock_step = |e: &mut Engine| {
        if lock != 0 {
            e.call(CALL_00483710, &args![lock]);
        }
    };
    lock_step(e);
    let kind = e.vcall(phantom, 0x14, &[]).u32();
    lock_step(e);
    if kind == 1 {
        let shape = fn_0062d2f0(e, Ptr::new(phantom));
        if !nothing_to_check(e, shape.addr()) {
            e.with_stack(0x11c, |e, collector| {
                fn_0062d350(e, collector);
                hkp_all_cd_body_pair_collector_reset(e, collector);
                lock_step(e);
                e.vcall(phantom, 0x44, &args![collector, 0u32]);
                lock_step(e);
                let hits = e.call(0x0041_3f40, &args![collector]).u32();
                if !nothing_to_check(e, hits) {
                    let count = e.call(GET_WORD_4, &args![hits]).i32();
                    let mut i = 0i32;
                    while i < count {
                        let hit = fn_0062e150(e, Ptr::new(hits), i as u32);
                        let body = e.mem.u32(hit.wrapping_add(8));
                        changed |= trap_body_entered(e, this, body);
                        i = i.wrapping_add(1);
                    }
                }
                hkp_all_cd_body_pair_collector_destructor(e, collector);
            });
        }
    } else if e.vcall(phantom, 0x14, &[]).u32() == 0 {
        let list = e.call(0x0045_c650, &args![phantom]).u32();
        if !nothing_to_check(e, list) {
            let count = e.call(GET_WORD_4, &args![list]).i32();
            let mut i = 0i32;
            while i < count {
                let slot = e.call(0x0062_9ab0, &args![list, i as u32]).u32();
                let body = e.mem.u32(slot);
                changed |= trap_body_entered(e, this, body);
                i = i.wrapping_add(1);
            }
        }
    }
    let list = Ptr::new(at(this, ENTRY_REF_LIST));
    fn_0062cb90(e, list);
    let event = fn_0062cb60(e, list);
    if event != 0 {
        let flag = if e.mem.u32(event.wrapping_add(4)) == 1 {
            0x2000_0000u32
        } else {
            0x4000_0000
        };
        let id = e.mem.u32(event);
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        e.call(FREE, &args![event]);
        if form != 0 {
            let owner = e.call(GET_WORD_C, &args![this]).u32();
            let word = e.call(REFERENCE_SCRIPT_WORD, &args![owner]).u32();
            changed |= e.call(SET_ACTION_FLAG, &args![form, word, flag]).u8();
            let owner = e.call(GET_WORD_C, &args![this]).u32();
            e.call(0x0057_2e10, &args![owner, form]);
        }
    }
    changed
}

// Translated from 0062de20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; a float): updates every
/// trap entry of the chain at `+0x18` (`0062cc90` with the float, quieted
/// by the exe's `FLD`/`FSTP`), then calls virtual slot `+4` of every item of
/// the chain whose head is at `this - 4` (`00620180` for the next).
pub fn fn_0062de20(e: &mut Engine, this: Ptr, seconds: f32) {
    let mut entry = e.mem.u32(at(this, 0x18));
    while entry != 0 {
        fn_0062cc90(e, Ptr::new(entry), x87_quiet_bits(seconds.to_bits()));
        entry = fn_0062d990(e, Ptr::new(entry));
    }
    let mut item = e.mem.u32(this.addr().wrapping_sub(4));
    while item != 0 {
        e.vcall(item, 4, &[]);
        item = e.call(0x0062_0180, &args![item]).u32();
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_db40, fn_0062db40(Ptr, u32) -> Ptr),
        entry!(0x0062_df60, fn_0062df60(Ptr, u32) -> Ptr),
        entry!(0x0062_d6f0, fn_0062d6f0(Ptr) -> Ptr),
        entry!(0x0062_d680, fn_0062d680(Ptr) -> Ptr),
        entry!(0x0062_d5c0, fn_0062d5c0(Ptr) -> Ptr),
        entry!(0x0062_d990, fn_0062d990(Ptr) -> u32),
        entry!(0x0062_d2f0, fn_0062d2f0(Ptr) -> Ptr),
        entry!(0x0062_e150, fn_0062e150(Ptr, u32) -> u32),
        entry!(0x0062_e120, fn_0062e120(Ptr, Ptr) -> Ptr),
        entry!(0x0062_dfa0, fn_0062dfa0(Ptr) -> Ptr),
        entry!(0x0062_c750, fn_0062c750(Ptr) -> Ptr),
        entry!(0x0062_c6e0, fn_0062c6e0(Ptr) -> Ptr),
        entry!(0x0062_c640, fn_0062c640(Ptr) -> Ptr),
        entry!(0x0062_dc50, fn_0062dc50(Ptr, u32, u32) -> Ptr),
        entry!(0x0062_e2f0, fn_0062e2f0(Ptr, Ptr)),
        entry!(0x0062_e060, fn_0062e060(Ptr, Ptr)),
        entry!(0x0062_e0a0, fn_0062e0a0(Ptr) -> u32),
        entry!(0x0062_cb60, fn_0062cb60(Ptr) -> u32),
        entry!(0x0062_cb90, fn_0062cb90(Ptr)),
        entry!(0x0062_c9d0, fn_0062c9d0(Ptr, Ptr) -> bool),
        entry!(0x0062_d570, fn_0062d570(Ptr, Ptr) -> bool),
        entry!(0x0062_d550, fn_0062d550(Ptr, Ptr) -> bool),
        entry!(0x0062_df20, fn_0062df20(Ptr, Ptr, u32) -> bool),
        entry!(
            0x0062_c860,
            trigger_entry_ref_list_force_leave_trigger(Ptr, Ptr)
        ),
        entry!(0x0062_c980, fn_0062c980(Ptr)),
        entry!(0x0062_e040, fn_0062e040(Ptr)),
        entry!(0x0062_dfe0, fn_0062dfe0(Ptr)),
        entry!(0x0062_c700, fn_0062c700(Ptr)),
        entry!(0x0062_c7a0, fn_0062c7a0(Ptr)),
        entry!(0x0062_d9e0, fn_0062d9e0(Ptr)),
        entry!(0x0062_d9b0, fn_0062d9b0(Ptr, u32) -> Ptr),
        entry!(0x0062_d510, fn_0062d510(Ptr)),
        entry!(0x0062_de90, tes_trap_listener_clear_current_refs(Ptr, u32)),
        entry!(0x0062_d920, fn_0062d920(Ptr)),
        entry!(0x0062_d740, fn_0062d740(Ptr)),
        entry!(0x0062_d760, fn_0062d760(Ptr)),
        entry!(0x0062_d8a0, fn_0062d8a0(Ptr)),
        entry!(
            0x0062_d870,
            tes_trap_listener_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0062_d330, hkp_cd_body_pair_collector_reset(Ptr)),
        entry!(0x0062_d3c0, fn_0062d3c0(Ptr) -> Ptr),
        entry!(0x0062_e1d0, fn_0062e1d0(Ptr, u32) -> Ptr),
        entry!(0x0062_d310, hkp_all_cd_body_pair_collector_reset(Ptr)),
        entry!(0x0062_d350, fn_0062d350(Ptr) -> Ptr),
        entry!(0x0062_e260, fn_0062e260(Ptr)),
        entry!(0x0062_e170, fn_0062e170(Ptr)),
        entry!(0x0062_df80, fn_0062df80(Ptr)),
        entry!(0x0062_d420, fn_0062d420(Ptr)),
        entry!(0x0062_d4b0, hkp_all_cd_body_pair_collector_destructor(Ptr)),
        entry!(0x0062_ddb0, fn_0062ddb0(Ptr) -> u32),
        entry!(0x0062_db60, fn_0062db60(Ptr, u32)),
        entry!(0x0062_da90, fn_0062da90(Ptr, u32)),
        entry!(0x0062_da00, fn_0062da00(Ptr, u32)),
        entry!(0x0062_cc90, fn_0062cc90(Ptr, u32) -> u8),
        entry!(0x0062_de20, fn_0062de20(Ptr, f32)),
    ]
    .into_iter()
    .chain(funcs_n086())
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_0062db40_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LIST_FIND_TARGET_00620810, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0062_db40, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LIST_FIND_TARGET_00620810)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }

    #[test]
    fn forwarder_fn_0062df60_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LIST_FIND_TARGET_0057C400, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0062_df60, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LIST_FIND_TARGET_0057C400)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }
}

#[cfg(test)]
mod b015_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == callee)
            .map(|c| c.1.clone())
            .collect()
    }

    fn take_log(e: &mut Engine) -> Log {
        e.call_log.take().unwrap()
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    /// The event list vtable of the test lists: slot 4 allocates a node,
    /// slot 8 frees one.
    const NODE_ALLOC: u32 = 0x0091_1004;
    const NODE_FREE: u32 = 0x0091_1008;

    /// Doubles for the allocator, the word getters and the list test.
    fn heap(e: &mut Engine) {
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(FREE, |_, _| Ret::default());
        e.register(GET_WORD_4, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(GET_WORD_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(GET_WORD_C, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(LIST_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0] + 0xc) == 0).into_ret()
        });
        e.put_vtable(0x0091_1000, &[0, NODE_ALLOC, NODE_FREE]);
        e.register(NODE_ALLOC, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(NODE_FREE, |_, _| Ret::default());
    }

    /// A reference list (at `+0x10` of a 0x2c-byte entry when `entry`) with
    /// room for `capacity` entries.
    fn ref_list(e: &mut Engine, capacity: u32) -> Ptr {
        let list = e.mem.alloc(0x1c);
        let block = e.mem.alloc(8 * capacity.max(1));
        e.mem.set_u32(list, block);
        e.mem.set_u32(list + 8, capacity);
        e.mem.set_u32(list + 0xc, 0x0091_1000);
        Ptr::new(list)
    }

    fn entries(e: &Engine, list: Ptr) -> Vec<(u32, u8)> {
        (0..e.mem.u32(list.addr() + 4))
            .map(|i| {
                let at = ref_entry(e, list, i);
                (e.mem.u32(at), e.mem.u8(at + 4))
            })
            .collect()
    }

    /// The queued events of a reference list, as (id, kind).
    fn events(e: &Engine, list: Ptr) -> Vec<(u32, u32)> {
        let mut out = vec![];
        let mut node = e.mem.u32(list.addr() + 0x10);
        while node != 0 {
            let event = e.mem.u32(node + 8);
            out.push((e.mem.u32(event), e.mem.u32(event + 4)));
            node = e.mem.u32(node);
        }
        out
    }

    /// A reference whose `+0xc` word (its form id) is `id`.
    fn reference(e: &mut Engine, id: u32) -> Ptr {
        let r = e.mem.alloc(0x10);
        e.mem.set_u32(r + 0xc, id);
        Ptr::new(r)
    }

    #[test]
    fn listener_constructors_chain_and_overlap_their_vtables() {
        let mut e = Engine::new();
        noop(&mut e, &[0x0061_ffb0, 0x0061_f4d0]);
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x20, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_d5c0, &[this]).u32(), this);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0061_ffb0), vec![vec![this]]);
        assert_eq!(calls_to(&log, 0x0061_f4d0), vec![vec![this + 8, 0]]);
        assert_eq!(e.mem.u32(this), TRAP_LISTENER_VTABLE);
        assert_eq!(e.mem.u32(this + 8), TRAP_LISTENER_VTABLE_8);
        // +0x1c is the bhkTrapListener's +0x14: the last store wins.
        assert_eq!(e.mem.u32(this + 0x1c), TRAP_LISTENER_VTABLE_1C);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        let other = e.mem.alloc(0x18);
        assert_eq!(e.call(0x0062_d680, &[other]).u32(), other);
        assert_eq!(e.mem.u32(other), BHK_TRAP_LISTENER_VTABLE);
        assert_eq!(e.mem.u32(other + 0x14), BHK_TRAP_LISTENER_VTABLE_14);
        assert_eq!(e.call(0x0062_d6f0, &[other]).u32(), other);
        assert_eq!(e.mem.u32(other), PHANTOM_LISTENER_VTABLE);
    }

    #[test]
    fn small_getters() {
        let mut e = Engine::new();
        heap(&mut e);
        e.register(STORE_00622570, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x1000);
        e.mem.set_u32(this + 4, 0x55);
        assert_eq!(e.call(0x0062_d990, &[this]).u32(), 0x55);
        assert_eq!(e.call(0x0062_d2f0, &[this]).u32(), this + 0x160);
        assert_eq!(e.call(0x0062_e150, &[this, 3]).u32(), 0x1030);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x0062_e120, &[this, out]).u32(), out);
        assert_eq!(e.mem.u32(out), 0);
        e.mem.set_u32(this + 4, 0);
        e.call(0x0062_e120, &[this, out]);
        assert_eq!(e.mem.u32(out), 1);
    }

    #[test]
    fn list_and_ref_list_constructors() {
        let mut e = Engine::new();
        heap(&mut e);
        noop(&mut e, &[0x0062_0720]);
        let list = e.mem.alloc(0x10);
        e.mem.write(list, &[0xff; 0x10]);
        assert_eq!(e.call(0x0062_c6e0, &[list]).u32(), list);
        assert_eq!(e.mem.u32(list), EVENT_LIST_VTABLE);
        for offset in [4, 8, 0xc] {
            assert_eq!(e.mem.u32(list + offset), 0);
        }
        e.call(0x0062_c750, &[list]);
        assert_eq!(e.mem.u32(list), EVENT_POINTER_LIST_VTABLE);
        e.call(0x0062_dfa0, &[list]);
        assert_eq!(e.mem.u32(list), EVENT_LIST_BASE_VTABLE);
        let entry = e.mem.alloc(0x2c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_dc50, &[entry, 0x11, 0x22]).u32(), entry);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0062_0720), vec![vec![entry, 0x11, 0x22]]);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0x14 * 8]]);
        assert_eq!(e.mem.u32(entry + 0x18), 0x14);
        assert_eq!(e.mem.u32(entry + 0x14), 0);
        assert_ne!(e.mem.u32(entry + 0x10), 0);
        assert_eq!(e.mem.u32(entry + 0x1c), EVENT_LIST_VTABLE);
    }

    #[test]
    fn event_list_links_at_the_tail_and_pops_the_head() {
        let mut e = Engine::new();
        heap(&mut e);
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list, 0x0091_1000);
        for value in [7u32, 8, 9] {
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, value);
            e.call(0x0062_e060, &[list, slot]);
        }
        assert_eq!(e.mem.u32(list + 0xc), 3);
        let head = e.mem.u32(list + 4);
        let tail = e.mem.u32(list + 8);
        assert_eq!(e.mem.u32(head + 4), 0);
        assert_eq!(e.mem.u32(tail), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_e0a0, &[list]).u32(), 7);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, NODE_FREE), vec![vec![list, head]]);
        assert_eq!(e.mem.u32(e.mem.u32(list + 4) + 4), 0);
        assert_eq!(e.call(0x0062_e0a0, &[list]).u32(), 8);
        assert_eq!(e.call(0x0062_e0a0, &[list]).u32(), 9);
        assert_eq!(e.mem.u32(list + 4), 0);
        assert_eq!(e.mem.u32(list + 8), 0);
        assert_eq!(e.mem.u32(list + 0xc), 0);
        // Linking by node directly.
        let node = e.mem.alloc(0xc);
        e.mem.write(node, &[0xee; 8]);
        e.call(0x0062_e2f0, &[list, node]);
        assert_eq!(e.mem.u32(list + 4), node);
        assert_eq!(e.mem.u32(node), 0);
        assert_eq!(e.mem.u32(node + 4), 0);
    }

    #[test]
    fn marking_present_appends_flags_and_queues_entry_events() {
        let mut e = Engine::new();
        heap(&mut e);
        let list = ref_list(&mut e, 1);
        let a = reference(&mut e, 0xa);
        let b = reference(&mut e, 0xb);
        // New: appended with the flag set, an "entered" (kind 1) event.
        assert!(!e.call(0x0062_c9d0, &[list.addr(), a.addr()]).bool());
        assert_eq!(entries(&e, list), vec![(0xa, 1)]);
        // Listed with the flag set: false; with it clear: set, true.
        assert!(!e.call(0x0062_c9d0, &[list.addr(), a.addr()]).bool());
        e.mem.set_u8(ref_entry(&e, list, 0) + 4, 0);
        assert!(e.call(0x0062_c9d0, &[list.addr(), a.addr()]).bool());
        // Full: the block doubles.
        let old_block = e.mem.u32(list.addr());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0062_c9d0, &[list.addr(), b.addr()]).bool());
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, ALLOCATE)[0], vec![2 * 8]);
        assert_eq!(calls_to(&log, FREE), vec![vec![old_block]]);
        assert_eq!(e.mem.u32(list.addr() + 8), 2);
        assert_eq!(entries(&e, list), vec![(0xa, 1), (0xb, 1)]);
        assert_eq!(events(&e, list), vec![(0xa, 1), (0xb, 1)]);
        assert!(e.call(0x0062_d570, &[list.addr(), b.addr()]).bool());
        let c = reference(&mut e, 0xc);
        assert!(!e.call(0x0062_d570, &[list.addr(), c.addr()]).bool());
    }

    #[test]
    fn aging_drops_unmarked_entries_with_a_leave_event() {
        let mut e = Engine::new();
        heap(&mut e);
        let list = ref_list(&mut e, 4);
        for (i, (id, flag)) in [(1u32, 1u8), (2, 0), (3, 0), (4, 1)].iter().enumerate() {
            let at = ref_entry(&e, list, i as u32);
            e.mem.set_u32(at, *id);
            e.mem.set_u8(at + 4, *flag);
        }
        e.mem.set_u32(list.addr() + 4, 4);
        e.call(0x0062_cb90, &[list.addr()]);
        assert_eq!(entries(&e, list), vec![(1, 0), (4, 0)]);
        assert_eq!(events(&e, list), vec![(2, 2), (3, 2)]);
        // The next event comes off the queue; none when empty.
        let first = e.call(0x0062_cb60, &[list.addr()]).u32();
        assert_eq!(e.mem.u32(first), 2);
        e.call(0x0062_cb60, &[list.addr()]);
        assert_eq!(e.call(0x0062_cb60, &[list.addr()]).u32(), 0);
    }

    #[test]
    fn entry_lookups_go_through_the_chain() {
        let mut e = Engine::new();
        heap(&mut e);
        let entry = e.mem.alloc(0x2c);
        let list = ref_list(&mut e, 2);
        e.mem.write(entry + 0x10, &e.mem.bytes(list.addr(), 0x1c));
        let r = reference(&mut e, 0x77);
        let at = ref_entry(&e, Ptr::new(entry + 0x10), 0);
        e.mem.set_u32(at, 0x77);
        e.mem.set_u32(entry + 0x14, 1);
        assert!(e.call(0x0062_d550, &[entry, r.addr()]).bool());
        let listener = e.mem.alloc(0x24);
        e.mem.set_u32(listener + 0x20, 0x0099_0000);
        let found = std::rc::Rc::new(std::cell::Cell::new(entry));
        let f = found.clone();
        e.register_double(LIST_FIND_TARGET_0057C400, move |_, _| f.get().into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0062_df20, &[listener, r.addr(), 0x5]).bool());
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, LIST_FIND_TARGET_0057C400),
            vec![vec![0x0099_0000, 5]]
        );
        found.set(0);
        assert!(!e.call(0x0062_df20, &[listener, r.addr(), 0x5]).bool());
    }

    #[test]
    fn force_leave_raises_the_leave_flag_for_live_forms() {
        let mut e = Engine::new();
        heap(&mut e);
        let list = ref_list(&mut e, 3);
        for (i, id) in [0x10u32, 0x20, 0x30].iter().enumerate() {
            e.mem.set_u32(ref_entry(&e, list, i as u32), *id);
        }
        e.mem.set_u32(list.addr() + 4, 3);
        // 0x10: a form passing +0xf0; 0x20: one failing it; 0x30: no form.
        e.put_vtable(0x0092_0000, &[0x0092_1000; 0x3d]);
        let live = e.mem.alloc(8);
        e.mem.set_u32(live, 0x0092_0000);
        e.mem.set_u32(live + 4, 1);
        let dead = e.mem.alloc(8);
        e.mem.set_u32(dead, 0x0092_0000);
        e.register(0x0092_1000, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register_double(LOOKUP_FORM, move |_, a| {
            match a[0] {
                0x10 => live,
                0x20 => dead,
                _ => 0,
            }
            .into_ret()
        });
        e.register(REFERENCE_SCRIPT_WORD, |_, _| 0x0055u32.into_ret());
        noop(&mut e, &[SET_ACTION_FLAG, 0x0057_2e10, 0x0056_5870]);
        let trigger = 0x0093_0000u32;
        e.call_log = Some(vec![]);
        e.call(0x0062_c860, &[list.addr(), trigger]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![12]]);
        assert_eq!(
            calls_to(&log, SET_ACTION_FLAG),
            vec![vec![live, 0x55, 0x4000_0000]]
        );
        assert_eq!(calls_to(&log, 0x0057_2e10), vec![vec![trigger, live]]);
        assert_eq!(calls_to(&log, 0x0056_5870), vec![vec![trigger]]);
        assert_eq!(calls_to(&log, FREE).len(), 1);
        // An empty list does nothing.
        e.mem.set_u32(list.addr() + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0062_c860, &[list.addr(), trigger]);
        assert_eq!(take_log(&mut e).len(), 1);
    }

    #[test]
    fn clearing_and_destroying_a_ref_list_frees_its_events() {
        let mut e = Engine::new();
        heap(&mut e);
        noop(&mut e, &[0x0062_e290]);
        let list = ref_list(&mut e, 2);
        let a = reference(&mut e, 1);
        let b = reference(&mut e, 2);
        e.call(0x0062_c9d0, &[list.addr(), a.addr()]);
        e.call(0x0062_c9d0, &[list.addr(), b.addr()]);
        e.call_log = Some(vec![]);
        e.call(0x0062_c980, &[list.addr()]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, FREE).len(), 2);
        assert_eq!(e.mem.u32(list.addr() + 4), 0);
        assert_eq!(e.mem.u32(list.addr() + 0x18), 0);
        e.call(0x0062_c9d0, &[list.addr(), a.addr()]);
        let block = e.mem.u32(list.addr());
        e.call_log = Some(vec![]);
        e.call(0x0062_c7a0, &[list.addr()]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, FREE)[0], vec![block]);
        assert_eq!(calls_to(&log, FREE).len(), 2);
        assert_eq!(e.mem.u32(list.addr()), 0);
        assert_eq!(calls_to(&log, 0x0062_e290), vec![vec![list.addr() + 0xc]]);
        assert_eq!(e.mem.u32(list.addr() + 0xc), EVENT_LIST_BASE_VTABLE);
        // A null block is not freed.
        e.call_log = Some(vec![]);
        e.call(0x0062_c7a0, &[list.addr()]);
        assert!(calls_to(&take_log(&mut e), FREE).is_empty());
    }

    #[test]
    fn entry_and_listener_destructors() {
        let mut e = Engine::new();
        heap(&mut e);
        noop(&mut e, &[0x0062_e290, 0x0055_45f0, 0x0062_0110]);
        // Two entries chained through their +4 word.
        let second = e.mem.alloc(0x2c);
        let first = e.mem.alloc(0x2c);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(first + 0x1c, 0x0091_1000);
        e.mem.set_u32(second + 0x1c, 0x0091_1000);
        let listener = e.mem.alloc(0x24);
        e.mem.set_u32(listener + 0x20, first);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_d870, &[listener, 1]).u32(), listener);
        let log = take_log(&mut e);
        let freed = calls_to(&log, FREE);
        assert!(freed.contains(&vec![first]));
        assert!(freed.contains(&vec![second]));
        assert_eq!(freed.last().unwrap(), &vec![listener]);
        assert_eq!(e.mem.u32(listener + 0x20), 0);
        assert_eq!(e.mem.u32(listener), TRAP_LISTENER_VTABLE);
        assert_eq!(e.mem.u32(listener + 8), TRAP_LISTENER_VTABLE_8);
        assert_eq!(e.mem.u32(listener + 0x1c), PHANTOM_LISTENER_VTABLE);
        assert_eq!(calls_to(&log, 0x0055_45f0), vec![vec![listener + 8]]);
        assert_eq!(calls_to(&log, 0x0062_0110), vec![vec![listener]]);
        // An entry deleted without the flag stays allocated.
        let entry = e.mem.alloc(0x2c);
        e.mem.set_u32(entry + 0x1c, 0x0091_1000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_d9b0, &[entry, 0]).u32(), entry);
        let log = take_log(&mut e);
        assert!(!calls_to(&log, FREE).contains(&vec![entry]));
        e.call(0x0062_d740, &[entry]);
        assert_eq!(e.mem.u32(entry), PHANTOM_LISTENER_VTABLE);
    }

    #[test]
    fn pair_collector_lifecycle() {
        let mut e = Engine::new();
        noop(
            &mut e,
            &[
                STORE_00622570,
                0x0068_15c0,
                0x0062_e230,
                0x0062_6870,
                0x004a_4840,
                0x0062_e350,
                CALL_00483710,
            ],
        );
        let c = e.mem.alloc(0x11c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_d350, &[c]).u32(), c);
        let log = take_log(&mut e);
        assert_eq!(e.mem.u32(c), ALL_PAIR_COLLECTOR_VTABLE);
        assert_eq!(calls_to(&log, 0x0068_15c0), vec![vec![c + 4]]);
        assert_eq!(
            calls_to(&log, 0x0062_e230),
            vec![vec![c + 8, c + 0x14, 0, 0x10]]
        );
        assert_eq!(calls_to(&log, 0x0062_6870), vec![vec![c + 8]]);
        assert_eq!(
            calls_to(&log, STORE_00622570),
            vec![vec![c + 4, 0], vec![c + 4, 0]]
        );
        assert_eq!(e.call(0x0062_e1d0, &[c, 9]).u32(), c);
        e.register(0x004a_4840, |_, _| 0x66u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_d4b0, &[c]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0062_e350), vec![vec![c + 8, 0x66]]);
        assert_eq!(calls_to(&log, CALL_00483710), vec![vec![c + 8]]);
        assert_eq!(calls_to(&log, 0x004a_4840)[0][1], c + 8);
        assert_eq!(e.mem.u32(c), PAIR_COLLECTOR_VTABLE);
        e.call(0x0062_d330, &[c]);
        e.call(0x0062_d310, &[c]);
        e.call(0x0062_df80, &[c]);
        e.call(0x0062_e170, &[c]);
        e.call(0x0062_e260, &[c]);
        e.call(0x0062_d3c0, &[c]);
        e.mem.set_u32(c, 0);
        e.call(0x0062_d420, &[c]);
        assert_eq!(e.mem.u32(c), PAIR_COLLECTOR_VTABLE);
    }

    #[test]
    fn owner_reference_chain_stops_at_the_first_null() {
        let mut e = Engine::new();
        e.register(0x008d_6f30, |_, a| (a[0] + 1).into_ret());
        e.register(0x0045_43c0, |_, a| (a[0] + 1).into_ret());
        e.register(0x007a_f430, |_, a| (a[0] + 0x100).into_ret());
        assert_eq!(e.call(0x0062_ddb0, &[0x1000]).u32(), 0x1102 - 8);
        assert_eq!(e.call(0x0062_ddb0, &[0]).u32(), 0);
        e.register(0x007a_f430, |_, _| Ret::default());
        assert_eq!(e.call(0x0062_ddb0, &[0x1000]).u32(), 0);
        e.register(0x0045_43c0, |_, _| Ret::default());
        assert_eq!(e.call(0x0062_ddb0, &[0x1000]).u32(), 0);
    }

    /// Doubles for the object and kind lookups; the kind is the source word.
    fn kinds(e: &mut Engine) {
        e.register(0x0046_0140, |_, a| (a[0] | 0x0100_0000).into_ret());
        e.register(0x0043_b540, |_, a| (a[0] & 0xff).into_ret());
        e.register(0x008c_71b0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(0x0043_b4d0, |e, a| e.mem.u32(a[0]).into_ret());
    }

    #[test]
    fn trap_objects_get_and_lose_entries() {
        let mut e = Engine::new();
        heap(&mut e);
        kinds(&mut e);
        noop(&mut e, &[0x0062_0720, 0x006e_cd40, 0x0062_e290]);
        e.register(FIND_COLLIDABLE_REF, |_, a| (a[0] + 0x10).into_ret());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x0033_0000);
        e.call_log = Some(vec![]);
        e.call(0x0062_db60, &[this, 0x0c]);
        let log = take_log(&mut e);
        let entry = e.mem.u32(this + 4);
        assert_eq!(calls_to(&log, ALLOCATE)[0], vec![0x2c]);
        assert_eq!(
            calls_to(&log, 0x0062_0720),
            vec![vec![entry, 0x0100_001c, 0x0100_000c]]
        );
        assert_eq!(calls_to(&log, 0x006e_cd40), vec![vec![entry, 0x0033_0000]]);
        // Another kind adds nothing.
        e.call_log = Some(vec![]);
        e.call(0x0062_db60, &[this, 0x0d]);
        assert!(calls_to(&take_log(&mut e), FIND_COLLIDABLE_REF).is_empty());
        // Removal: the entry found for the object is unlinked and deleted.
        let found = e.mem.alloc(0x2c);
        e.mem.set_u32(found + 0x1c, 0x0091_1000);
        e.register_double(LIST_FIND_TARGET_00620810, move |_, _| found.into_ret());
        e.register(0x0062_07a0, |_, _| 0x0044_0000u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_da90, &[this, 0x18]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, LIST_FIND_TARGET_00620810),
            vec![vec![entry, 0x0100_0018]]
        );
        assert_eq!(calls_to(&log, 0x0062_07a0), vec![vec![found, entry]]);
        assert_eq!(e.mem.u32(this + 4), 0x0044_0000);
        assert!(calls_to(&log, FREE).contains(&vec![found]));
        // Without a chain the kind is not even read.
        e.mem.set_u32(this + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0062_da90, &[this, 0x18]);
        assert!(calls_to(&take_log(&mut e), 0x0043_b4d0).is_empty());
    }

    #[test]
    fn zone_objects_go_to_the_owner() {
        let mut e = Engine::new();
        kinds(&mut e);
        noop(&mut e, &[0x0057_6800, 0x0062_0130]);
        e.register(FIND_COLLIDABLE_REF, |_, a| (a[0] + 0x10).into_ret());
        let owner = e.mem.alloc(0x10);
        let this = owner + 8;
        for (kind, target) in [
            (0x0e, 0x0057_6800),
            (0x10, 0x0057_6800),
            (0x1e, 0x0062_0130),
        ] {
            e.call_log = Some(vec![]);
            e.call(0x0062_da00, &[this, kind]);
            let log = take_log(&mut e);
            assert_eq!(
                calls_to(&log, target),
                vec![vec![owner, 0x0100_0010 + kind]]
            );
        }
        e.call_log = Some(vec![]);
        e.call(0x0062_da00, &[this, 0x0c]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0057_6800).is_empty());
        assert!(calls_to(&log, 0x0062_0130).is_empty());
    }

    #[test]
    fn clearing_an_entry_forces_its_references_out_unless_blocked() {
        let mut e = Engine::new();
        heap(&mut e);
        e.map(OBJECT_POINTER_011DDF38, 4);
        e.register(OBJECT_TEST, |_, _| Ret::default());
        e.register(LOOKUP_FORM, |_, _| Ret::default());
        e.register(REFERENCE_SCRIPT_WORD, |_, _| Ret::default());
        let listener = e.mem.alloc(0x24);
        let second = e.mem.alloc(0x2c);
        let first = e.mem.alloc(0x2c);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(first + 0xc, 0x0aaa);
        e.mem.set_u32(second + 0xc, 0x0bbb);
        let list = ref_list(&mut e, 1);
        e.mem.write(second + 0x10, &e.mem.bytes(list.addr(), 0x1c));
        e.mem.set_u32(ref_entry(&e, Ptr::new(second + 0x10), 0), 5);
        e.mem.set_u32(second + 0x14, 1);
        e.mem.set_u32(listener + 0x20, first);
        e.call_log = Some(vec![]);
        e.call(0x0062_de90, &[listener, 0x0bbb]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, LOOKUP_FORM), vec![vec![5]]);
        assert_eq!(calls_to(&log, REFERENCE_SCRIPT_WORD), vec![vec![0x0bbb]]);
        assert_eq!(e.mem.u32(second + 0x14), 0);
        // Blocked: only the clearing.
        e.register(OBJECT_TEST, |_, _| 1u32.into_ret());
        e.mem.set_u32(second + 0x14, 1);
        e.call_log = Some(vec![]);
        e.call(0x0062_d510, &[second]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, LOOKUP_FORM).is_empty());
        assert_eq!(e.mem.u32(second + 0x14), 0);
    }

    /// A trap entry for `0062cc90`: its reference passes the checks, its
    /// phantom answers `kind`, and its reference list has room for 4.
    fn trap_entry(e: &mut Engine, kind: u32) -> (Ptr, u32) {
        heap(e);
        noop(
            e,
            &[
                0x0044_0da0,
                0x0044_0d80,
                CALL_00483710,
                0x0056_09f0,
                STORE_00622570,
            ],
        );
        e.put_vtable(0x0094_0000, &[0x0094_1000; 0x120]);
        e.register(0x0094_1000, |_, _| 1u32.into_ret());
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, 0x0094_0000);
        let phantom = e.mem.alloc(0x170);
        let phantom_table = e.mem.alloc(0x50);
        e.mem.set_u32(phantom, phantom_table);
        e.mem.set_u32(phantom_table + 0x14, 0x0095_0014);
        e.register_double(0x0095_0014, move |_, _| kind.into_ret());
        e.mem.set_u32(phantom + 8, 0x0096_0000);
        e.register_double(0x004b_59d0, move |_, _| phantom.into_ret());
        let entry = e.mem.alloc(0x2c);
        e.mem.set_u32(entry + 0xc, owner);
        let list = ref_list(e, 4);
        e.mem.write(entry + 0x10, &e.mem.bytes(list.addr(), 0x1c));
        e.register(REFERENCE_SCRIPT_WORD, |_, _| 0x0055u32.into_ret());
        e.register(SET_ACTION_FLAG, |_, a| {
            u32::from(a[2] != 0x4000_0000).into_ret()
        });
        e.register(LOOKUP_FORM, |_, a| a[0].into_ret());
        noop(e, &[0x0057_2e10]);
        (Ptr::new(entry), phantom)
    }

    /// A body whose collidable reference has the id `id` and whose
    /// reference passes or fails slot `+0x100` and the force slots.
    fn body(e: &mut Engine, id: u32) -> u32 {
        let reference = e.mem.alloc(0x10);
        e.mem.set_u32(reference, 0x0097_0000);
        e.mem.set_u32(reference + 0xc, id);
        let body = e.mem.alloc(4);
        e.mem.set_u32(body, reference);
        body
    }

    fn reference_doubles(e: &mut Engine, ignorable: bool) {
        e.put_vtable(0x0097_0000, &[0x0097_1000; 0x8d]);
        let answer = u32::from(ignorable);
        e.register_double(0x0097_1000, move |_, _| answer.into_ret());
        e.register(FIND_COLLIDABLE_REF, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x004b_59f0, |_, _| 1u32.into_ret());
    }

    #[test]
    fn a_shape_phantom_marks_the_bodies_it_collects() {
        let mut e = Engine::new();
        let (entry, phantom) = trap_entry(&mut e, 1);
        reference_doubles(&mut e, false);
        noop(
            &mut e,
            &[
                STORE_00622570,
                0x0068_15c0,
                0x0062_e230,
                0x0062_6870,
                0x004a_4840,
                0x0062_e350,
            ],
        );
        // The collector's hits: two 16-byte elements with bodies at +8.
        let hits = e.mem.alloc(0x10);
        let elements = e.mem.alloc(0x20);
        let first = body(&mut e, 0x41);
        e.mem.set_u32(elements + 8, first);
        e.mem.set_u32(elements + 0x18, 0);
        e.mem.set_u32(hits, elements);
        e.mem.set_u32(hits + 4, 2);
        e.register_double(0x0041_3f40, move |_, _| hits.into_ret());
        let phantom_table = e.mem.u32(phantom);
        e.mem.set_u32(phantom_table + 0x44, 0x0095_0044);
        e.register(0x0095_0044, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_cc90, &[entry.addr(), 0]).u32(), 1);
        let log = take_log(&mut e);
        let list = Ptr::new(entry.addr() + 0x10);
        // Marked present; the aging pass then clears its flag.
        assert_eq!(entries(&e, list), vec![(0x41, 0)]);
        // A new reference raises nothing in the loop; its entered event is
        // taken off and handled: SetActionFlag with 0x20000000, then
        // 00572e10.
        let owner = e.mem.u32(entry.addr() + 0xc);
        assert_eq!(
            calls_to(&log, SET_ACTION_FLAG),
            vec![vec![0x41, 0x55, 0x2000_0000]]
        );
        assert_eq!(calls_to(&log, 0x0057_2e10), vec![vec![owner, 0x41]]);
        assert_eq!(calls_to(&log, 0x0095_0044).len(), 1);
        // The lock word before and after both phantom calls (the hit
        // array's destructor runs 00483710 once more, on itself).
        let locks = calls_to(&log, CALL_00483710);
        assert_eq!(locks.iter().filter(|w| w[0] == 0x0096_0000).count(), 4);
        assert_eq!(locks.len(), 5);
        assert!(events(&e, list).is_empty());
    }

    #[test]
    fn a_list_phantom_checks_its_bodies_and_skips_ignorable_ones() {
        let mut e = Engine::new();
        let (entry, phantom) = trap_entry(&mut e, 0);
        reference_doubles(&mut e, true);
        let list = e.mem.alloc(8);
        let slots = e.mem.alloc(8);
        let first = body(&mut e, 0x42);
        e.mem.set_u32(slots, first);
        e.mem.set_u32(list, slots);
        e.mem.set_u32(list + 4, 1);
        e.register_double(0x0045_c650, move |_, _| list.into_ret());
        e.register(0x0062_9ab0, |e, a| (e.mem.u32(a[0]) + 4 * a[1]).into_ret());
        // Already listed with its flag clear: marking it answers true and
        // raises SetActionFlag with 0x10000000. Ignorable (slot +0x100 and
        // 004b59f0 pass) but also forced (the force slots answer 1 too).
        let listed = Ptr::new(entry.addr() + 0x10);
        e.mem.set_u32(ref_entry(&e, listed, 0), 0x42);
        e.mem.set_u32(entry.addr() + 0x14, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_cc90, &[entry.addr(), 0]).u32(), 1);
        let log = take_log(&mut e);
        let reference = e.mem.u32(first);
        assert_eq!(
            calls_to(&log, SET_ACTION_FLAG),
            vec![vec![reference, 0x55, 0x1000_0000]]
        );
        assert_eq!(calls_to(&log, 0x0045_c650), vec![vec![phantom]]);
        assert_eq!(entries(&e, Ptr::new(entry.addr() + 0x10)), vec![(0x42, 0)]);
        // A phantom answering 2 checks nothing.
        let (entry, _) = trap_entry(&mut e, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_cc90, &[entry.addr(), 0]).u32(), 0);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0045_c650).is_empty());
        assert!(calls_to(&log, FIND_COLLIDABLE_REF).is_empty());
    }

    #[test]
    fn an_entry_without_a_usable_reference_does_nothing() {
        let mut e = Engine::new();
        let (entry, _) = trap_entry(&mut e, 1);
        e.register(0x0044_0da0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_cc90, &[entry.addr(), 0]).u32(), 0);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x004b_59d0).is_empty());
        e.mem.set_u32(entry.addr() + 0xc, 0);
        assert_eq!(e.call(0x0062_cc90, &[entry.addr(), 0]).u32(), 0);
    }

    #[test]
    fn listener_update_runs_entries_then_the_item_chain() {
        let mut e = Engine::new();
        let (entry, _) = trap_entry(&mut e, 2);
        let this = e.mem.alloc(0x20) + 4;
        e.mem.set_u32(this + 0x18, entry.addr());
        let item = e.mem.alloc(8);
        e.put_vtable(0x0098_0000, &[0, 0x0098_1000]);
        e.mem.set_u32(item, 0x0098_0000);
        e.register(0x0098_1000, |_, _| Ret::default());
        e.register(0x0062_0180, |_, _| Ret::default());
        e.mem.set_u32(this - 4, item);
        e.call_log = Some(vec![]);
        e.call(0x0062_de20, &[this, 0x7f80_0001]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0062_cc90), Vec::<Vec<u32>>::new());
        assert_eq!(calls_to(&log, 0x004b_59d0).len(), 1);
        assert_eq!(calls_to(&log, 0x0098_1000), vec![vec![item]]);
        assert_eq!(calls_to(&log, 0x0062_0180), vec![vec![item]]);
    }
}

/// `00825c00` (`thiscall`, plain `RET`): the first item of a trap entry's
/// target chain.
const FIRST_TARGET_00825C00: u32 = 0x0082_5c00;
/// `006210d0` (`cdecl`, one word): frees the block (the
/// `hkpCdBodyPairCollector` deleting destructor's free).
const FREE_006210D0: u32 = 0x0062_10d0;
/// `00c85750` (`cdecl`, no words; `Havok SDK/hkmemoryrouter.cpp`): the
/// memory router.
const MEMORY_ROUTER_00C85750: u32 = 0x00c8_5750;
/// `0044edb0` (`thiscall`, plain `RET`; named
/// `BaseProcess::GetCurrentProcedureIndex` in the map, a folded getter): the
/// router's allocator, whose virtual slot `+8` frees a block of a size.
const ROUTER_ALLOCATOR_0044EDB0: u32 = 0x0044_edb0;
/// The `TrapEntry` vtable (RTTI `.?AVTrapEntry@@`).
const TRAP_ENTRY_VTABLE: u32 = 0x0104_e3e8;
/// `0061fc10` (`thiscall`, `RET 8`): the base constructor of a trap entry.
const TRAP_ENTRY_BASE_CONSTRUCTOR: u32 = 0x0061_fc10;
/// `00620340` (`thiscall`, `RET 0x14`) and `00620680` (`thiscall`,
/// `RET 8`): run on the listener (`this - 8`) for a body pair.
const LISTENER_PAIR_00620340: u32 = 0x0062_0340;
const LISTENER_PAIR_00620680: u32 = 0x0062_0680;

// Translated from 0062c5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TrapEntry::Run` (Xbox PDB; `thiscall`, plain `RET`): from the first
/// target (`00825c00`), each target's virtual slot `+0` runs with `this`;
/// the next target is `00726070` of the target's `+0x10` link less `0x10`
/// (null stays null).
pub fn trap_entry_run(e: &mut Engine, this: Ptr) {
    let mut target = e.call(FIRST_TARGET_00825C00, &args![this]).u32();
    while target != 0 {
        e.vcall(target, 0, &args![this]);
        let next = e.call(GET_WORD_4, &args![target.wrapping_add(0x10)]).u32();
        target = if next == 0 {
            0
        } else {
            next.wrapping_sub(0x10)
        };
    }
}

// Translated from 0062c600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TrapTargetEntry::Run` (Xbox PDB; `thiscall`, `RET 4`): the target
/// (`0084e3a0` of `this + 0x10`) runs its virtual slot `+0x18c` with
/// `trap` and `this`.
pub fn trap_target_entry_run(e: &mut Engine, this: Ptr, trap: u32) {
    let target = e.call(GET_WORD_C, &args![at(this, 0x10)]).u32();
    e.vcall(target, 0x18c, &args![trap, this]);
}

// Translated from 0062c720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSList<TriggerEntry::TriggerEvent*>` scalar deleting destructor
/// (unnamed in the engine map; `thiscall`, `RET 4`): the destructor
/// (`0062c700`), then `00401030` frees `this` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_0062c720(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0062c700(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0062c770 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSPointerListBase<TriggerEntry::TriggerEvent*>` scalar deleting
/// destructor (unnamed in the engine map; `thiscall`, `RET 4`): the
/// destructor (`0062dfe0`), then `00401030` frees `this` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_0062c770(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0062dfe0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0062d3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `hkpCdBodyPairCollector` scalar deleting destructor (unnamed in the
/// engine map; `thiscall`, `RET 4`): the destructor (`0062d420`), then
/// `006210d0` frees `this` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0062d3f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0062d420(e, this);
    if flags & 1 != 0 {
        e.call(FREE_006210D0, &args![this]);
    }
    this
}

// Translated from 0062d440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `hkpAllCdBodyPairCollector` scalar deleting destructor (Xbox PDB;
/// `thiscall`, `RET 4`): the destructor (`0062d4b0`), then `0062d470` frees
/// `this` when bit 0 of `flags` is set. Returns `this`.
pub fn hkp_all_cd_body_pair_collector_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    hkp_all_cd_body_pair_collector_destructor(e, this);
    if flags & 1 != 0 {
        fn_0062d470(e, this.addr());
    }
    this
}

// Translated from 0062d470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): a non-null `block` goes
/// back to the memory router's allocator (`00c85750`, then `0044edb0` on
/// it), whose virtual slot `+8` takes the block and its size `0x114`.
pub fn fn_0062d470(e: &mut Engine, block: u32) {
    if block != 0 {
        let router = e.call(MEMORY_ROUTER_00C85750, &[]).u32();
        let allocator = e.call(ROUTER_ALLOCATOR_0044EDB0, &args![router]).u32();
        e.vcall(allocator, 8, &args![block, 0x114u32]);
    }
}

// Translated from 0062d640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantomListener::phantomShapeSetCallback` (Xbox PDB; `thiscall`,
/// `RET 4`): when the phantom's word at `+8` (`0044ddc0`) is not zero, the
/// listener's virtual slots `+8` and then `+4` run with the phantom.
pub fn hkp_phantom_listener_phantom_shape_set_callback(e: &mut Engine, this: Ptr, phantom: u32) {
    if e.call(GET_WORD_8, &args![phantom]).u32() != 0 {
        e.vcall(this.addr(), 8, &args![phantom]);
        e.vcall(this.addr(), 4, &args![phantom]);
    }
}

// Translated from 0062d6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bhkTrapListener` scalar deleting destructor (unnamed in the engine
/// map; `thiscall`, `RET 4`): the destructor (`0062d760`), then `00401030`
/// frees `this` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0062d6c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0062d760(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0062d710 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `hkpPhantomListener` scalar deleting destructor (Xbox PDB;
/// `thiscall`, `RET 4`): the destructor (`0062d740`), then `00401030` frees
/// `this` when bit 0 of `flags` is set. Returns `this`.
pub fn hkp_phantom_listener_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0062d740(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0062d7c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062d7c0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `TESTrapListener::CreateZoneEntry` (Xbox PDB; `thiscall`, `RET 8`): a
/// new `0x18`-byte block (`00401000`) made a `TrapEntry` with both words
/// (`0062d840`), or null when the allocation fails. `this` is not read.
pub fn tes_trap_listener_create_zone_entry(
    e: &mut Engine,
    _this: Ptr,
    first: u32,
    second: u32,
) -> u32 {
    let block = e.call(ALLOCATE, &args![0x18u32]).u32();
    if block == 0 {
        return 0;
    }
    fn_0062d840(e, Ptr::new(block), first, second).addr()
}

// Translated from 0062d840 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `TrapEntry` constructor (unnamed in the engine map; `thiscall`,
/// `RET 8`): the base constructor `0061fc10` with both words, then the
/// vtable `0104e3e8`. Returns `this`.
pub fn fn_0062d840(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(TRAP_ENTRY_BASE_CONSTRUCTOR, &args![this, first, second]);
    e.mem.set_u32(this.addr(), TRAP_ENTRY_VTABLE);
    this
}

/// The two references of a body pair: `FindCollidableRef` of the words at
/// `+0x48` and `+0x20`, when both are set and the second's virtual slot
/// `+0x100` answers true (its low byte).
fn pair_references(e: &mut Engine, pair: u32) -> Option<(u32, u32)> {
    let first_word = e.mem.u32(pair.wrapping_add(0x48));
    let first = e.call(FIND_COLLIDABLE_REF, &args![first_word]).u32();
    let second_word = e.mem.u32(pair.wrapping_add(0x20));
    let second = e.call(FIND_COLLIDABLE_REF, &args![second_word]).u32();
    if first == 0 || second == 0 {
        return None;
    }
    if e.vcall(second, 0x100, &[]).u32() & 0xff == 0 {
        return None;
    }
    Some((first, second))
}

// Translated from 0062dc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the listener's `+8` part,
/// `RET 4`): for the pair's two references (`FindCollidableRef` of its
/// `+0x48` and `+0x20` words), when both are set and the second's virtual
/// slot `+0x100` answers true, whether `00620340` on the listener
/// (`this - 8`) with the pair, both references, 0 and 1 answers non-zero;
/// otherwise false.
pub fn fn_0062dc80(e: &mut Engine, this: Ptr, pair: u32) -> bool {
    match pair_references(e, pair) {
        Some((first, second)) => {
            let listener = this.addr().wrapping_sub(8);
            e.call(
                LISTENER_PAIR_00620340,
                &args![listener, pair, first, second, 0u32, 1u32],
            )
            .u32()
                != 0
        }
        None => false,
    }
}

// Translated from 0062dd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the listener's `+8` part,
/// `RET 4`): for the pair's two references as in `0062dc80`, when both are
/// set and the second's virtual slot `+0x100` answers true, `00620680` on
/// the listener (`this - 8`) with the pair's `+0x48` word (read again) and
/// the second reference, and true; otherwise false.
pub fn fn_0062dd20(e: &mut Engine, this: Ptr, pair: u32) -> bool {
    match pair_references(e, pair) {
        Some((_, second)) => {
            let word = e.mem.u32(pair.wrapping_add(0x48));
            let listener = this.addr().wrapping_sub(8);
            e.call(LISTENER_PAIR_00620680, &args![listener, word, second]);
            true
        }
        None => false,
    }
}

// Translated from 0062dee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; a `TESTrapListener`):
/// whether one of the trap entries (from `+0x20`, the next from
/// `0062d990`) has `key` as its `0084e3a0` word.
pub fn fn_0062dee0(e: &mut Engine, this: Ptr, key: u32) -> bool {
    let mut entry = e.mem.u32(at(this, 0x20));
    while entry != 0 {
        if e.call(GET_WORD_C, &args![entry]).u32() == key {
            return true;
        }
        entry = fn_0062d990(e, Ptr::new(entry));
    }
    false
}

// Translated from 0062e200 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSListBase<TriggerEntry::TriggerEvent*>` scalar deleting destructor
/// (Xbox PDB; `thiscall`, `RET 4`): the destructor (`0062e040`), then
/// `00401030` frees `this` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_list_base_trigger_event_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0062e040(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's n086 translations, by exe address.
fn funcs_n086() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_c5a0, trap_entry_run(Ptr)),
        entry!(0x0062_c600, trap_target_entry_run(Ptr, u32)),
        entry!(0x0062_c720, fn_0062c720(Ptr, u32) -> Ptr),
        entry!(0x0062_c770, fn_0062c770(Ptr, u32) -> Ptr),
        entry!(0x0062_d3f0, fn_0062d3f0(Ptr, u32) -> Ptr),
        entry!(
            0x0062_d440,
            hkp_all_cd_body_pair_collector_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0062_d470, fn_0062d470(u32)),
        entry!(
            0x0062_d640,
            hkp_phantom_listener_phantom_shape_set_callback(Ptr, u32)
        ),
        entry!(0x0062_d6c0, fn_0062d6c0(Ptr, u32) -> Ptr),
        entry!(
            0x0062_d710,
            hkp_phantom_listener_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0062_d7c0,
            tes_trap_listener_create_zone_entry(Ptr, u32, u32) -> u32
        ),
        entry!(0x0062_d840, fn_0062d840(Ptr, u32, u32) -> Ptr),
        entry!(0x0062_dc80, fn_0062dc80(Ptr, u32) -> bool),
        entry!(0x0062_dd20, fn_0062dd20(Ptr, u32) -> bool),
        entry!(0x0062_dee0, fn_0062dee0(Ptr, u32) -> bool),
        entry!(
            0x0062_e200,
            bs_list_base_trigger_event_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
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

    /// An object whose vtable has `slots` entries at `0x7100_0000 + offset`,
    /// each doubled to answer `answer`.
    fn object_with_slots(e: &mut Engine, offsets: &[u32]) -> u32 {
        let object = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(object, vtable);
        for &offset in offsets {
            e.mem.set_u32(vtable + offset, 0x7100_0000 + offset);
        }
        object
    }

    #[test]
    fn trap_entry_run_walks_the_target_chain() {
        let mut e = Engine::new();
        let first = object_with_slots(&mut e, &[0]);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(second, e.mem.u32(first));
        e.mem.set_u32(first + 0x10, second + 0x10);
        e.mem.set_u32(second + 0x10, 0);
        e.register_double(FIRST_TARGET_00825C00, move |_, _| first.into_ret());
        e.register(GET_WORD_4, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x7100_0000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0062_c5a0, &args![0x5000u32]);
        assert_eq!(
            calls(&e, 0x7100_0000),
            vec![vec![first, 0x5000], vec![second, 0x5000]]
        );
        assert_eq!(
            calls(&e, GET_WORD_4),
            vec![vec![first + 0x10], vec![second + 0x10]]
        );

        // No first target: nothing runs.
        e.register(FIRST_TARGET_00825C00, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_c5a0, &args![0x5000u32]);
        assert!(calls(&e, 0x7100_0000).is_empty());
    }

    #[test]
    fn trap_target_entry_run_calls_the_target() {
        let mut e = Engine::new();
        let target = object_with_slots(&mut e, &[0x18c]);
        e.register_double(GET_WORD_C, move |_, _| target.into_ret());
        e.register(0x7100_018c, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0062_c600, &args![0x6000u32, 0x77u32]);
        assert_eq!(calls(&e, GET_WORD_C), vec![vec![0x6010]]);
        assert_eq!(calls(&e, 0x7100_018c), vec![vec![target, 0x77, 0x6000]]);
    }

    #[test]
    fn scalar_deleting_destructors_free_on_bit_0() {
        let mut e = Engine::new();
        e.register(FREE, |_, _| Ret::default());
        e.register(FREE_006210D0, |_, _| Ret::default());
        e.register(0x0062_e290, |_, _| Ret::default());
        e.register(0x0055_45f0, |_, _| Ret::default());
        e.register(0x004a_4840, |_, _| Ret::default());
        e.register(0x0062_e350, |_, _| Ret::default());
        e.register(CALL_00483710, |_, _| Ret::default());
        e.register(MEMORY_ROUTER_00C85750, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        let cases: [(u32, u32, u32); 6] = [
            (0x0062_c720, EVENT_LIST_BASE_VTABLE, FREE),
            (0x0062_c770, EVENT_LIST_BASE_VTABLE, FREE),
            (0x0062_d3f0, PAIR_COLLECTOR_VTABLE, FREE_006210D0),
            (0x0062_d6c0, BHK_TRAP_LISTENER_VTABLE, FREE),
            (0x0062_d710, PHANTOM_LISTENER_VTABLE, FREE),
            (0x0062_e200, EVENT_LIST_BASE_VTABLE, FREE),
        ];
        for (addr, vtable, free) in cases {
            e.mem.set_u32(this, BHK_TRAP_LISTENER_VTABLE);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![this, 2u32]).u32(), this);
            assert!(calls(&e, free).is_empty());
            if addr != 0x0062_d6c0 {
                assert_eq!(e.mem.u32(this), vtable);
            }
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, 1u32]);
            assert_eq!(calls(&e, free), vec![vec![this]]);
        }

        // hkpAllCdBodyPairCollector: its free goes to the router (0062d470).
        let allocator = object_with_slots(&mut e, &[8]);
        e.register_double(ROUTER_ALLOCATOR_0044EDB0, move |_, _| allocator.into_ret());
        e.register(0x7100_0008, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_d440, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), PAIR_COLLECTOR_VTABLE);
        assert!(calls(&e, 0x7100_0008).is_empty());
        e.call(0x0062_d440, &args![this, 1u32]);
        assert_eq!(calls(&e, 0x7100_0008), vec![vec![allocator, this, 0x114]]);
    }

    #[test]
    fn fn_0062d470_frees_through_the_router_unless_null() {
        let mut e = Engine::new();
        let allocator = object_with_slots(&mut e, &[8]);
        e.register(MEMORY_ROUTER_00C85750, |_, _| 0x4444u32.into_ret());
        e.register_double(ROUTER_ALLOCATOR_0044EDB0, move |_, _| allocator.into_ret());
        e.register(0x7100_0008, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0062_d470, &args![0x9000u32]);
        assert_eq!(calls(&e, ROUTER_ALLOCATOR_0044EDB0), vec![vec![0x4444]]);
        assert_eq!(calls(&e, 0x7100_0008), vec![vec![allocator, 0x9000, 0x114]]);
        e.call_log = Some(vec![]);
        e.call(0x0062_d470, &args![0u32]);
        assert!(calls(&e, MEMORY_ROUTER_00C85750).is_empty());
    }

    #[test]
    fn phantom_shape_set_callback_needs_the_phantom_word() {
        let mut e = Engine::new();
        let this = object_with_slots(&mut e, &[4, 8]);
        e.register(0x7100_0004, |_, _| Ret::default());
        e.register(0x7100_0008, |_, _| Ret::default());
        e.register(GET_WORD_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        let phantom = e.mem.alloc(0x10);
        e.mem.set_u32(phantom + 8, 1);
        e.call_log = Some(vec![]);
        e.call(0x0062_d640, &args![this, phantom]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|&a| a == 0x7100_0004 || a == 0x7100_0008)
            .collect();
        assert_eq!(order, vec![0x7100_0008, 0x7100_0004]);
        assert_eq!(calls(&e, 0x7100_0004), vec![vec![this, phantom]]);
        e.mem.set_u32(phantom + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x0062_d640, &args![this, phantom]);
        assert!(calls(&e, 0x7100_0008).is_empty());
    }

    #[test]
    fn create_zone_entry_constructs_a_trap_entry() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x18);
        e.register_double(ALLOCATE, move |_, _| block.into_ret());
        e.register(TRAP_ENTRY_BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let result = e.call(0x0062_d7c0, &args![0x1234u32, 7u32, 9u32]).u32();
        assert_eq!(result, block);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x18]]);
        assert_eq!(
            calls(&e, TRAP_ENTRY_BASE_CONSTRUCTOR),
            vec![vec![block, 7, 9]]
        );
        assert_eq!(e.mem.u32(block), TRAP_ENTRY_VTABLE);

        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_d7c0, &args![0x1234u32, 7u32, 9u32]).u32(), 0);
        assert!(calls(&e, TRAP_ENTRY_BASE_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn constructor_0062d840_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(TRAP_ENTRY_BASE_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_d840, &args![this, 1u32, 2u32]).u32(), this);
        assert_eq!(
            calls(&e, TRAP_ENTRY_BASE_CONSTRUCTOR),
            vec![vec![this, 1, 2]]
        );
        assert_eq!(e.mem.u32(this), TRAP_ENTRY_VTABLE);
    }

    /// A pair with words 0x11 (`+0x48`) and 0x22 (`+0x20`); references
    /// found for them are `first` and `second` (null for zero).
    fn pair_setup(e: &mut Engine, first: u32, flag: u32) -> (u32, u32) {
        let pair = e.mem.alloc(0x50);
        e.mem.set_u32(pair + 0x48, 0x11);
        e.mem.set_u32(pair + 0x20, 0x22);
        let second = object_with_slots(e, &[0x100]);
        e.register_double(FIND_COLLIDABLE_REF, move |_, a| {
            match a[0] {
                0x11 => first,
                _ => second,
            }
            .into_ret()
        });
        e.register_double(0x7100_0100, move |_, _| (0xff00 | flag).into_ret());
        (pair, second)
    }

    #[test]
    fn fn_0062dc80_enters_when_both_references_qualify() {
        let mut e = Engine::new();
        let (pair, second) = pair_setup(&mut e, 0x3000, 1);
        e.register(LISTENER_PAIR_00620340, |_, _| 5u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0062_dc80, &args![0x1008u32, pair]).bool());
        assert_eq!(
            calls(&e, LISTENER_PAIR_00620340),
            vec![vec![0x1000, pair, 0x3000, second, 0, 1]]
        );
        e.register(LISTENER_PAIR_00620340, |_, _| 0u32.into_ret());
        assert!(!e.call(0x0062_dc80, &args![0x1008u32, pair]).bool());
        // The slot's low byte is 0: nothing.
        let (pair, _) = pair_setup(&mut e, 0x3000, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0062_dc80, &args![0x1008u32, pair]).bool());
        assert!(calls(&e, LISTENER_PAIR_00620340).is_empty());
        // No first reference.
        let (pair, _) = pair_setup(&mut e, 0, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0062_dc80, &args![0x1008u32, pair]).bool());
        assert!(calls(&e, 0x7100_0100).is_empty());
    }

    #[test]
    fn fn_0062dd20_leaves_when_both_references_qualify() {
        let mut e = Engine::new();
        let (pair, second) = pair_setup(&mut e, 0x3000, 1);
        e.register(LISTENER_PAIR_00620680, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0062_dd20, &args![0x1008u32, pair]).bool());
        assert_eq!(
            calls(&e, LISTENER_PAIR_00620680),
            vec![vec![0x1000, 0x11, second]]
        );
        let (pair, _) = pair_setup(&mut e, 0x3000, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0062_dd20, &args![0x1008u32, pair]).bool());
        assert!(calls(&e, LISTENER_PAIR_00620680).is_empty());
    }

    #[test]
    fn fn_0062dee0_finds_an_entry_by_key() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x20, first);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(first + 0xc, 0xa);
        e.mem.set_u32(second + 0xc, 0xb);
        e.register(GET_WORD_4, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(GET_WORD_C, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        assert!(e.call(0x0062_dee0, &args![this, 0xbu32]).bool());
        assert!(e.call(0x0062_dee0, &args![this, 0xau32]).bool());
        assert!(!e.call(0x0062_dee0, &args![this, 0xcu32]).bool());
        e.mem.set_u32(this + 0x20, 0);
        assert!(!e.call(0x0062_dee0, &args![this, 0xau32]).bool());
    }
}
