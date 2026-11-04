//! `(unplaced)/run_00b9ef30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSShaderLightingProperty::RemoveLight` (engine map name of `00b717a0`;
/// `thiscall`, one word).
const REMOVE_LIGHT: u32 = 0x00b7_17a0;
/// `00e74d40` (`thiscall`; the engine map names it
/// `NiTPointerListBase<..>::RemoveAll`): empties a pointer list.
const LIST_REMOVE_ALL: u32 = 0x00e7_4d40;

// Translated from 00b9ef30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`; it ends in a jump to
/// `00e74d40`). Walks the list whose head is at `this + 0xe0` (each node:
/// next at `+0`, item at `+8`); for each item whose word at `+0xa8` is not
/// null and whose `+0x1c` word, less one, is at most 11 (unsigned, so 1 to
/// 12), it calls `00b717a0` on that `+0xa8` object with `this`. Then it
/// empties the lists at `this + 0xe0` and `this + 0x12c` (`00e74d40`).
pub fn fn_00b9ef30(e: &mut Engine, this: Ptr) {
    let head = this.addr().wrapping_add(0xe0);
    let mut node = e.mem.u32(head);
    while node != 0 {
        let item = e.mem.u32(node.wrapping_add(8));
        node = e.mem.u32(node);
        if item == 0 {
            continue;
        }
        let property = e.mem.u32(item.wrapping_add(0xa8));
        if property == 0 {
            continue;
        }
        // `DEC; CMP 0xb; SBB; INC; NEG; SBB; AND`: the property when its
        // `+0x1c` word minus one is at most 11, else 0.
        let kind = e.mem.u32(property.wrapping_add(0x1c)).wrapping_sub(1);
        let target = if kind <= 0xb { property } else { 0 };
        if target != 0 {
            e.call(REMOVE_LIGHT, &args![target, this]);
        }
    }
    e.call(LIST_REMOVE_ALL, &args![head]);
    e.call(LIST_REMOVE_ALL, &args![this.addr().wrapping_add(0x12c)]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b9_ef30, fn_00b9ef30(Ptr))]
}

#[cfg(test)]
mod tests {
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
    fn removes_the_lights_of_kinds_1_to_12_then_empties_both_lists() {
        let mut e = Engine::new();
        e.register(REMOVE_LIGHT, |_, _| Ret::default());
        e.register(LIST_REMOVE_ALL, |_, _| Ret::default());
        let this = e.mem.alloc(0x140);
        // Five nodes: kinds 1, 12, 0, 13, then an item without a property,
        // then a null item.
        let mut properties = vec![];
        let mut next = 0u32;
        for kind in [
            None,
            Some(None),
            Some(Some(13u32)),
            Some(Some(0)),
            Some(Some(12)),
            Some(Some(1)),
        ] {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node, next);
            if let Some(property_kind) = kind {
                let item = e.mem.alloc(0xb0);
                if let Some(k) = property_kind {
                    let property = e.mem.alloc(0x20);
                    e.mem.set_u32(property + 0x1c, k);
                    e.mem.set_u32(item + 0xa8, property);
                    properties.push((k, property));
                }
                e.mem.set_u32(node + 8, item);
            }
            next = node;
        }
        e.mem.set_u32(this + 0xe0, next);
        e.call_log = Some(vec![]);
        e.call(0x00b9_ef30, &[this]);
        let expected: Vec<Vec<u32>> = [1u32, 12]
            .iter()
            .map(|k| {
                let p = properties.iter().find(|(kind, _)| kind == k).unwrap().1;
                vec![p, this]
            })
            .collect();
        assert_eq!(calls_to(&e, REMOVE_LIGHT), expected);
        assert_eq!(
            calls_to(&e, LIST_REMOVE_ALL),
            vec![vec![this + 0xe0], vec![this + 0x12c]]
        );
    }
}
