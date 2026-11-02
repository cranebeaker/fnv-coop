//! `fallout shared/run_00482b00` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00482c50` runs first (`00402d30`, `thiscall`).
const BASE_CONSTRUCTOR: u32 = 0x0040_2d30;
/// `TESDescription`'s vtable (RTTI `.?AVTESDescription@@`).
const TES_DESCRIPTION_VTABLE: u32 = 0x0101_c18c;
/// `TESDescription::InitializeDataComponent` (`00482c80`, `thiscall`).
const INITIALIZE_DATA_COMPONENT: u32 = 0x0048_2c80;
/// The allocator `00401000` (`cdecl (size)`), the list constructor
/// `0096a2d0` (`thiscall`, answers the list), the node's item address
/// `006815c0` (`thiscall`, answers `this`), the next node `00726070`
/// (`thiscall`), the append `005ae3d0` (`thiscall (list, &item)`) and the
/// removal `00905330` (`thiscall (list, &item)`).
const ALLOCATE: u32 = 0x0040_1000;
const LIST_CONSTRUCTOR: u32 = 0x0096_a2d0;
const ITEM_OF: u32 = 0x0068_15c0;
const NEXT_NODE: u32 = 0x0072_6070;
const APPEND: u32 = 0x005a_e3d0;
const REMOVE: u32 = 0x0090_5330;

// Translated from 00482c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `TESDescription`'s constructor: the base
/// constructor (`00402d30`), the class's vtable, then
/// `TESDescription::InitializeDataComponent`. Answers `this`.
pub fn fn_00482c50(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), TES_DESCRIPTION_VTABLE);
    e.call(INITIALIZE_DATA_COMPONENT, &args![this]);
    this
}

/// The node's item address (`006815c0`).
fn item(e: &mut Engine, node: u32) -> u32 {
    e.call(ITEM_OF, &args![node]).u32()
}

/// The word at the node's item address.
fn item_word(e: &mut Engine, node: u32) -> u32 {
    let at = item(e, node);
    e.mem.u32(at)
}

/// The next node (`00726070`).
fn next(e: &mut Engine, node: u32) -> u32 {
    e.call(NEXT_NODE, &args![node]).u32()
}

// Translated from 00482b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00482b00 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map: a new 8-byte list (`00401000`, constructed
/// by `0096a2d0` when allocated, else null) filled with the items of the
/// list at `this + 4` (each node's item `006815c0`, appended by `005ae3d0`
/// while the item is non-null), then made unique: for each node, every
/// later node whose item's word at +4 equals this one's is removed
/// (`00905330`, the scan restarting after the current node). Answers the
/// new list. The exception-unwinding frame is not translated.
pub fn fn_00482b00(e: &mut Engine, this: Ptr) -> Ptr {
    let block = e.call(ALLOCATE, &args![8u32]).u32();
    let list = if block != 0 {
        e.call(LIST_CONSTRUCTOR, &args![block]).u32()
    } else {
        0
    };
    let mut node = this.addr().wrapping_add(4);
    while node != 0 && item_word(e, node) != 0 {
        let entry = item(e, node);
        e.call(APPEND, &args![list, entry]);
        node = next(e, node);
    }
    let mut outer = list;
    while outer != 0 && item_word(e, outer) != 0 {
        let mut inner = next(e, outer);
        while inner != 0 && item_word(e, inner) != 0 {
            let first = item_word(e, outer);
            let second = item_word(e, inner);
            if e.mem.u32(first + 4) == e.mem.u32(second + 4) {
                let entry = item(e, inner);
                e.call(REMOVE, &args![outer, entry]);
                inner = next(e, outer);
            } else {
                inner = next(e, inner);
            }
        }
        outer = next(e, outer);
    }
    Ptr::new(list)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00482c50, fn_00482c50(Ptr) -> Ptr),
        entry!(0x00482b00, fn_00482b00(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_runs_the_base_then_initializes() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(INITIALIZE_DATA_COMPONENT, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00482c50, &args![this]).u32(), this);
        let order: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [0x00482c50, BASE_CONSTRUCTOR, INITIALIZE_DATA_COMPONENT]
        );
        assert_eq!(e.mem.u32(this), TES_DESCRIPTION_VTABLE);
    }

    /// Lists are nodes of (item, next); the doubles walk them in place, the
    /// append adds a node at the end, the removal unlinks the node holding
    /// the item.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LIST_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(ITEM_OF, |_, a| a[0].into_ret());
        e.register(NEXT_NODE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(APPEND, |e, a| {
            let mut node = a[0];
            if e.mem.u32(node) == 0 {
                e.mem.set_u32(node, e.mem.u32(a[1]));
                return Ret::default();
            }
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let fresh = e.mem.alloc(8);
            e.mem.set_u32(fresh, e.mem.u32(a[1]));
            e.mem.set_u32(node + 4, fresh);
            Ret::default()
        });
        e.register(REMOVE, |e, a| {
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                let next = e.mem.u32(node + 4);
                if next == a[1] {
                    e.mem.set_u32(node + 4, e.mem.u32(next + 4));
                    break;
                }
                node = next;
            }
            Ret::default()
        });
        e
    }

    #[test]
    fn copies_the_list_without_repeated_keys() {
        let mut e = engine();
        // Items whose key (word at +4) is 1, 2, 1.
        let items: Vec<u32> = [1u32, 2, 1]
            .iter()
            .map(|key| {
                let item = e.mem.alloc(8);
                e.mem.set_u32(item + 4, *key);
                item
            })
            .collect();
        let this = e.mem.alloc(0xc);
        let second = e.mem.alloc(8);
        let third = e.mem.alloc(8);
        e.mem.set_u32(this + 4, items[0]);
        e.mem.set_u32(this + 8, second);
        e.mem.set_u32(second, items[1]);
        e.mem.set_u32(second + 4, third);
        e.mem.set_u32(third, items[2]);
        let list = e.call(0x00482b00, &args![this]).u32();
        let mut keys = vec![];
        let mut node = list;
        while node != 0 {
            keys.push(e.mem.u32(e.mem.u32(node) + 4));
            node = e.mem.u32(node + 4);
        }
        assert_eq!(keys, [1, 2]);
    }
}
