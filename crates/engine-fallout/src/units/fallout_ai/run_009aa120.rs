//! `fallout/ai/run_009aa120` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The add and remove of the pair lists the `CombatAction` and `CombatGoal`
//! classes keep (8-byte entries built by `004b0680`, compared by their first
//! word).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`, `cdecl(size)`) and `operator delete`
/// (`00401030`, `cdecl(block)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The entry constructor (`004b0680`, `thiscall(a, b)`).
const ENTRY_CONSTRUCT: u32 = 0x004b_0680;
/// Appends an entry to the list (`00905820`, `thiscall(entry pointer)`).
const LIST_APPEND: u32 = 0x0090_5820;
/// Returns the address of a node item (`006815c0`, `thiscall`).
const NODE_ITEM: u32 = 0x0068_15c0;
/// Returns the first word of an entry (`00559450`, `thiscall`).
const ENTRY_FIRST: u32 = 0x0055_9450;
/// Removes an entry from the list (`00905330`, `thiscall(entry pointer)`).
const LIST_REMOVE: u32 = 0x0090_5330;
/// Returns the next node (`0062d990`, `thiscall`).
const NODE_NEXT: u32 = 0x0062_d990;

// Translated from 009aa2b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009aa2b0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Unnamed in the engine map (`thiscall(a, b)`, `RET 8`): makes an 8-byte
/// entry `(a, b)` (`004b0680`; null when the allocation fails) and appends
/// it to the list at `this` (`00905820`, given the address of a local
/// holding the entry).
pub fn fn_009aa2b0(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let entry = if block != 0 {
        e.call(ENTRY_CONSTRUCT, &args![block, a, b]).u32()
    } else {
        0
    };
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), entry);
        e.call(LIST_APPEND, &args![this, slot]);
    });
}

// Translated from 009aa340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(value)`, `RET 4`): walks the list
/// from `this` (`0062d990` to the next node) and removes and frees the first
/// entry whose first word is `value` (`00905330` on the node holding it,
/// then `operator delete`). Nodes without an entry are skipped.
pub fn fn_009aa340(e: &mut Engine, this: Ptr, value: u32) {
    let mut node = this.addr();
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if entry != 0 && e.call(ENTRY_FIRST, &args![entry]).u32() == value {
            let entry = e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), entry);
                e.call(LIST_REMOVE, &args![node, local]);
                e.mem.u32(local.addr())
            });
            e.call(OPERATOR_DELETE, &args![entry]);
            return;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

/// The table of 8-byte state entries whose first word is the state's name
/// (`0x011a55b0`, indexed by the state kind).
const STATE_NAMES: u32 = 0x011a_55b0;
/// The value-name tables `fn_009aa1b0` indexes (one word per value).
const VALUE_NAMES_011A5700: u32 = 0x011a_5700;
const VALUE_NAMES_0118A8A8: u32 = 0x0118_a8a8;
const VALUE_NAMES_011A5768: u32 = 0x011a_5768;
/// `"True"` (`0x0108e9c4`) and `"False"` (`0x0108e9bc`).
const TEXT_TRUE: u32 = 0x0108_e9c4;
const TEXT_FALSE: u32 = 0x0108_e9bc;
/// `"%s: %s"` (`0x01012600`).
const NAME_VALUE_FORMAT: u32 = 0x0101_2600;
/// `00406d00` (unnamed): `cdecl` formatter (buffer, size, format, ...); here five words.
const FORMAT_INTO: u32 = 0x0040_6d00;
/// `00726070` (unnamed): `thiscall`, an entry's second word.
const ENTRY_SECOND: u32 = 0x0072_6070;
/// `009aad80` (unnamed, `combatworldstate.cpp`): `thiscall`, one word (`RET 4`); copies
/// a world-state array.
const COPY_FROM: u32 = 0x009a_ad80;
/// `00403d30` (unnamed): `cdecl` (destination, byte, size); fills the bytes.
const FILL_BYTES: u32 = 0x0040_3d30;

// Translated from 009aa120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the name of the state whose kind is the word at
/// `this` (the first word of entry `kind` of the table at `0x011a55b0`).
pub fn fn_009aa120(e: &mut Engine, this: Ptr) -> u32 {
    let kind = e.mem.u32(this.addr());
    e.mem.u32(kind.wrapping_mul(8).wrapping_add(STATE_NAMES))
}

// Translated from 009aa140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): the name of state `kind` (the first word
/// of entry `kind` of the table at `0x011a55b0`).
pub fn fn_009aa140(e: &mut Engine, kind: u32) -> u32 {
    e.mem.u32(kind.wrapping_mul(8).wrapping_add(STATE_NAMES))
}

// Translated from 009aa150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`): writes `"<state>: <value>"` into
/// `buffer` (`00406d00` with `size` and `"%s: %s"`): the state's name (`fn_009aa120`) and
/// the name of `value` for its kind (`fn_009aa1b0`; -1 means the state's own value at
/// `+4`).
pub fn fn_009aa150(e: &mut Engine, this: Ptr, buffer: u32, size: u32, value: u32) {
    let mut value = value;
    if value == 0xffff_ffff {
        value = e.mem.u32(this.addr() + 0x4);
    }
    let kind = e.mem.u32(this.addr());
    let value_name = fn_009aa1b0(e, kind, value);
    let state_name = fn_009aa120(e, this);
    e.call(
        FORMAT_INTO,
        &args![buffer, size, NAME_VALUE_FORMAT, state_name, value_name],
    );
}

// Translated from 009aa1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words): the name of `value` for a state of
/// `kind`. The compiler's switch on `kind - 1` (byte map `009aa230`, `0 0 3 3 3 3 3 3 3 1
/// 1 1 0 3 3 3 3 2`, unsigned): kinds 1, 2 and 13 read the table at `0x011a5700`, kinds
/// 10 to 12 the one at `0x0118a8a8`, kind 18 the one at `0x011a5768` (no bounds check);
/// every other kind gives `"True"` for a non-zero value and `"False"` for 0.
pub fn fn_009aa1b0(e: &mut Engine, kind: u32, value: u32) -> u32 {
    let table = match kind.wrapping_sub(1) {
        0 | 1 | 12 => Some(VALUE_NAMES_011A5700),
        9..=11 => Some(VALUE_NAMES_0118A8A8),
        17 => Some(VALUE_NAMES_011A5768),
        _ => None,
    };
    match table {
        Some(table) => e.mem.u32(value.wrapping_mul(4).wrapping_add(table)),
        None if value != 0 => TEXT_TRUE,
        None => TEXT_FALSE,
    }
}

// Translated from 009aa250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): whether the state is of `kind` and
/// matches `value` (`fn_009aa280`).
pub fn fn_009aa250(e: &mut Engine, this: Ptr, kind: u32, value: u32) -> bool {
    if kind == e.mem.u32(this.addr()) {
        fn_009aa280(e, this, value)
    } else {
        false
    }
}

// Translated from 009aa280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether `value` matches the state's
/// value at `+4`: equal, or either is -1 (any).
pub fn fn_009aa280(e: &mut Engine, this: Ptr, value: u32) -> bool {
    let own = e.mem.u32(this.addr() + 0x4);
    value == own || value == 0xffff_ffff || own == 0xffff_ffff
}

// Translated from 009aa3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): whether any entry of the list from the
/// node `this` (`006815c0`, `0062d990`) is a state of `kind` matching `value`
/// (`fn_009aa250`); nodes without an entry are skipped.
pub fn fn_009aa3b0(e: &mut Engine, this: Ptr, kind: u32, value: u32) -> bool {
    let mut node = this.addr();
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if entry != 0 && fn_009aa250(e, Ptr::new(entry), kind, value) {
            return true;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 009aa410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): finds the first entry of the list from
/// the node `this` whose first word (`00559450`) is `kind` and stores its second word
/// (`00726070`) at `out`; whether one was found.
pub fn fn_009aa410(e: &mut Engine, this: Ptr, kind: u32, out: u32) -> bool {
    let mut node = this.addr();
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if entry != 0 && e.call(ENTRY_FIRST, &args![entry]).u32() == kind {
            let value = e.call(ENTRY_SECOND, &args![entry]).u32();
            e.mem.set_u32(out, value);
            return true;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 009aa470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): clears the world-state array (`fn_009aa4b0`)
/// and returns `this`.
pub fn fn_009aa470(e: &mut Engine, this: Ptr) -> Ptr {
    fn_009aa4b0(e, this);
    this
}

// Translated from 009aa490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): copies `other` into `this`
/// (`009aad80`) and returns `this`.
pub fn fn_009aa490(e: &mut Engine, this: Ptr, other: u32) -> Ptr {
    e.call(COPY_FROM, &args![this, other]);
    this
}

// Translated from 009aa4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): zeroes the 0x50 bytes at `this` and the 0xc
/// bytes at `+0x50` (`00403d30` twice).
pub fn fn_009aa4b0(e: &mut Engine, this: Ptr) {
    e.call(FILL_BYTES, &args![this, 0u32, 0x50u32]);
    e.call(FILL_BYTES, &args![this.addr() + 0x50, 0u32, 0xcu32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009a_a2b0, fn_009aa2b0(Ptr, u32, u32)),
        entry!(0x009a_a340, fn_009aa340(Ptr, u32)),
        entry!(0x009a_a120, fn_009aa120(Ptr) -> u32),
        entry!(0x009a_a140, fn_009aa140(u32) -> u32),
        entry!(0x009a_a150, fn_009aa150(Ptr, u32, u32, u32)),
        entry!(0x009a_a1b0, fn_009aa1b0(u32, u32) -> u32),
        entry!(0x009a_a250, fn_009aa250(Ptr, u32, u32) -> bool),
        entry!(0x009a_a280, fn_009aa280(Ptr, u32) -> bool),
        entry!(0x009a_a3b0, fn_009aa3b0(Ptr, u32, u32) -> bool),
        entry!(0x009a_a410, fn_009aa410(Ptr, u32, u32) -> bool),
        entry!(0x009a_a470, fn_009aa470(Ptr) -> Ptr),
        entry!(0x009a_a490, fn_009aa490(Ptr, u32) -> Ptr),
        entry!(0x009a_a4b0, fn_009aa4b0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn add_makes_an_entry_and_appends_it() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ENTRY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        let appended = e.mem.alloc(4);
        e.register_double(LIST_APPEND, move |e, a| {
            let entry = e.mem.u32(a[1]);
            e.mem.set_u32(appended, entry);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009a_a2b0, &args![0x5000u32, 3u32, 4u32]);
        let entry = e.mem.u32(appended);
        assert_eq!(calls(&e, ENTRY_CONSTRUCT), vec![vec![entry, 3, 4]]);
        assert_eq!(e.mem.u32(entry + 4), 4);
        assert_eq!(calls(&e, LIST_APPEND)[0][0], 0x5000);
        // A failed allocation appends a null entry.
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call(0x009a_a2b0, &args![0x5000u32, 3u32, 4u32]);
        assert_eq!(e.mem.u32(appended), 0);
    }

    #[test]
    fn remove_frees_the_first_matching_entry() {
        let mut e = Engine::new();
        // Nodes: item word at +0, next at +4. Entries: first word at +0.
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ENTRY_FIRST, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LIST_REMOVE, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let entry = |e: &mut Engine, v: u32| {
            let p = e.mem.alloc(8);
            e.mem.set_u32(p, v);
            p
        };
        let a = entry(&mut e, 1);
        let b = entry(&mut e, 2);
        let c = entry(&mut e, 2);
        let mut next = 0;
        for item in [c, b, 0, a] {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        let head = next;
        let third = e.mem.u32(e.mem.u32(head + 4) + 4);
        e.call_log = Some(vec![]);
        e.call(0x009a_a340, &args![head, 2u32]);
        assert_eq!(calls(&e, LIST_REMOVE)[0][0], third);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![b]]);
        // No match: nothing removed.
        e.call_log = Some(vec![]);
        e.call(0x009a_a340, &args![head, 9u32]);
        assert!(calls(&e, LIST_REMOVE).is_empty());
        assert_eq!(calls(&e, NODE_NEXT).len(), 4);
    }
}

#[cfg(test)]
mod state_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn state(e: &mut Engine, kind: u32, value: u32) -> u32 {
        let p = e.mem.alloc(8);
        e.mem.set_u32(p, kind);
        e.mem.set_u32(p + 4, value);
        p
    }

    fn name_tables(e: &mut Engine) {
        for table in [
            STATE_NAMES,
            VALUE_NAMES_011A5700,
            VALUE_NAMES_0118A8A8,
            VALUE_NAMES_011A5768,
        ] {
            e.map(table & !0xfff, 0x2000);
        }
        e.mem.set_u32(STATE_NAMES + 3 * 8, 0x5003);
        e.mem.set_u32(STATE_NAMES + 13 * 8, 0x5013);
        e.mem.set_u32(VALUE_NAMES_011A5700 + 8, 0x6002);
        e.mem.set_u32(VALUE_NAMES_0118A8A8 + 4, 0x6101);
        e.mem.set_u32(VALUE_NAMES_011A5768 + 12, 0x6203);
    }

    #[test]
    fn fn_009aa120_and_fn_009aa140_read_the_state_name() {
        let mut e = Engine::new();
        name_tables(&mut e);
        let s = state(&mut e, 3, 0);
        assert_eq!(e.call(0x009a_a120, &args![s]).u32(), 0x5003);
        assert_eq!(e.call(0x009a_a140, &args![13u32]).u32(), 0x5013);
    }

    #[test]
    fn fn_009aa1b0_picks_the_value_table_by_kind() {
        let mut e = Engine::new();
        name_tables(&mut e);
        let name =
            |e: &mut Engine, kind: u32, value: u32| e.call(0x009a_a1b0, &args![kind, value]).u32();
        for kind in [1u32, 2, 13] {
            assert_eq!(name(&mut e, kind, 2), 0x6002);
        }
        for kind in [10u32, 11, 12] {
            assert_eq!(name(&mut e, kind, 1), 0x6101);
        }
        assert_eq!(name(&mut e, 18, 3), 0x6203);
        for kind in [0u32, 3, 9, 14, 17, 19, 0x100] {
            assert_eq!(name(&mut e, kind, 5), TEXT_TRUE);
            assert_eq!(name(&mut e, kind, 0), TEXT_FALSE);
        }
    }

    #[test]
    fn fn_009aa150_formats_name_and_value() {
        let mut e = Engine::new();
        name_tables(&mut e);
        e.register(FORMAT_INTO, |_, _| Ret::default());
        let s = state(&mut e, 13, 2);
        e.call_log = Some(vec![]);
        e.call(0x009a_a150, &args![s, 0x7000u32, 0x40u32, 0xffff_ffffu32]);
        e.call(0x009a_a150, &args![s, 0x7000u32, 0x40u32, 0u32]);
        assert_eq!(
            calls(&e, FORMAT_INTO),
            vec![
                vec![0x7000, 0x40, NAME_VALUE_FORMAT, 0x5013, 0x6002],
                vec![
                    0x7000,
                    0x40,
                    NAME_VALUE_FORMAT,
                    0x5013,
                    e.mem.u32(VALUE_NAMES_011A5700)
                ]
            ]
        );
    }

    #[test]
    fn fn_009aa250_and_fn_009aa280_match_kind_and_value() {
        let mut e = Engine::new();
        let s = state(&mut e, 4, 7);
        let any = state(&mut e, 4, 0xffff_ffff);
        let matches = |e: &mut Engine, s: u32, kind: u32, value: u32| {
            e.call(0x009a_a250, &args![s, kind, value]).bool()
        };
        assert!(matches(&mut e, s, 4, 7));
        assert!(matches(&mut e, s, 4, 0xffff_ffff));
        assert!(!matches(&mut e, s, 4, 8));
        assert!(!matches(&mut e, s, 5, 7));
        assert!(matches(&mut e, any, 4, 8));
        assert!(e.call(0x009a_a280, &args![s, 7u32]).bool());
        assert!(!e.call(0x009a_a280, &args![s, 6u32]).bool());
    }

    /// A list of nodes `[entry, next]` holding `entries` (0 for an empty node).
    fn list(e: &mut Engine, entries: &[u32]) -> u32 {
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let mut next = 0;
        for entry in entries.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *entry);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    #[test]
    fn fn_009aa3b0_finds_a_matching_state() {
        let mut e = Engine::new();
        let a = state(&mut e, 1, 2);
        let b = state(&mut e, 3, 4);
        let head = list(&mut e, &[a, 0, b]);
        assert!(e.call(0x009a_a3b0, &args![head, 3u32, 4u32]).bool());
        assert!(e
            .call(0x009a_a3b0, &args![head, 1u32, 0xffff_ffffu32])
            .bool());
        assert!(!e.call(0x009a_a3b0, &args![head, 3u32, 5u32]).bool());
        assert!(!e.call(0x009a_a3b0, &args![0u32, 3u32, 4u32]).bool());
    }

    #[test]
    fn fn_009aa410_reads_the_first_value_of_a_kind() {
        let mut e = Engine::new();
        e.register(ENTRY_FIRST, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ENTRY_SECOND, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let a = state(&mut e, 1, 2);
        let b = state(&mut e, 3, 4);
        let c = state(&mut e, 3, 5);
        let head = list(&mut e, &[a, 0, b, c]);
        let out = e.mem.alloc(4);
        assert!(e.call(0x009a_a410, &args![head, 3u32, out]).bool());
        assert_eq!(e.mem.u32(out), 4);
        e.mem.set_u32(out, 0x99);
        assert!(!e.call(0x009a_a410, &args![head, 9u32, out]).bool());
        assert_eq!(e.mem.u32(out), 0x99);
    }

    #[test]
    fn fn_009aa470_fn_009aa490_and_fn_009aa4b0_clear_and_copy() {
        let mut e = Engine::new();
        e.register(FILL_BYTES, |e, a| {
            let bytes = vec![a[1] as u8; a[2] as usize];
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(COPY_FROM, |_, _| Ret::default());
        let array = e.mem.alloc(0x60);
        e.mem.write(array, &[0xcd; 0x60]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009a_a470, &args![array]).u32(), array);
        assert_eq!(
            calls(&e, FILL_BYTES),
            vec![vec![array, 0, 0x50], vec![array + 0x50, 0, 0xc]]
        );
        assert!(e.mem.bytes(array, 0x5c).iter().all(|b| *b == 0));
        assert_eq!(e.mem.u32(array + 0x5c), 0xcdcd_cdcd);
        assert_eq!(e.call(0x009a_a490, &args![array, 0x4000u32]).u32(), array);
        assert_eq!(calls(&e, COPY_FROM), vec![vec![array, 0x4000]]);
        e.call_log = Some(vec![]);
        e.call(0x009a_a4b0, &args![array]);
        assert_eq!(calls(&e, FILL_BYTES).len(), 2);
    }
}
