//! `fallout/ai/run_009c6380` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0043b300` (unnamed, `modelloader.cpp`): `cdecl` (an `NiRTTI`, an object); whether the
/// object is of that class.
const IS_KIND_OF: u32 = 0x0043_b300;
/// The `NiRTTI` of `BSBlastNode` (`0x01202e7c`, by the name its static initializer gives it).
const RTTI_BS_BLAST_NODE: u32 = 0x0120_2e7c;
/// `0045bb80` (unnamed, `tes.cpp`): `thiscall`; the address of the node's point.
const NODE_POINT: u32 = 0x0045_bb80;
/// `0043b480` / `0043b4a0` (unnamed, `modelloader.cpp`): a node's child count and child
/// `index` (`RET 4`).
const CHILD_COUNT: u32 = 0x0043_b480;
const CHILD_AT: u32 = 0x0043_b4a0;

// Translated from 009c6380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`; `this` is only passed on): whether
/// `node` or a node below it is a `BSBlastNode` (`0043b300`); the first one found copies
/// its point (`0045bb80`, three words) to `out`. The children (slot `+0xc` node,
/// `0043b480`, `0043b4a0`) are searched until one is found. False for no node.
pub fn fn_009c6380(e: &mut Engine, this: Ptr, node: u32, out: u32) -> bool {
    if node == 0 {
        return false;
    }
    let mut found = false;
    if e.call(IS_KIND_OF, &args![RTTI_BS_BLAST_NODE, node]).u8() != 0 {
        found = true;
        let point = e.call(NODE_POINT, &args![node]).u32();
        let words = e.mem.bytes(point, 0xc);
        e.mem.write(out, &words);
    }
    let children = e.vcall(node, 0xc, &[]).u32();
    if children != 0 {
        let mut index = 0u32;
        while !found && index < e.call(CHILD_COUNT, &args![children]).u32() {
            let child = e.call(CHILD_AT, &args![children, index]).u32();
            found |= fn_009c6380(e, this, child, out);
            index = index.wrapping_add(1);
        }
    }
    found
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009c_6380, fn_009c6380(Ptr, u32, u32) -> bool)]
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

    /// A node: vtable slot `+0xc` gives its children list (`+0x80`; count then items),
    /// `+0x84` its class, `+0x88` its point.
    fn node(e: &mut Engine, class: u32, point: [f32; 3], children: &[u32]) -> u32 {
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table + 0xc, 0x700c);
        let p = e.mem.alloc(0x100);
        e.mem.set_u32(p, table);
        e.mem.set_u32(p + 0x84, class);
        for (i, v) in point.iter().enumerate() {
            e.mem.set_f32(p + 0x88 + 4 * i as u32, *v);
        }
        if !children.is_empty() {
            let list = e.mem.alloc(4 + 4 * children.len() as u32);
            e.mem.set_u32(list, children.len() as u32);
            for (i, c) in children.iter().enumerate() {
                e.mem.set_u32(list + 4 + 4 * i as u32, *c);
            }
            e.mem.set_u32(p + 0x80, list);
        }
        p
    }

    #[test]
    fn fn_009c6380_finds_the_first_blast_node() {
        let mut e = Engine::new();
        e.register(0x700c, |e, a| ret(e.mem.u32(a[0] + 0x80)));
        e.register(IS_KIND_OF, |e, a| {
            ret((e.mem.u32(a[1] + 0x84) == a[0]) as u32)
        });
        e.register(NODE_POINT, |_, a| ret(a[0] + 0x88));
        e.register(CHILD_COUNT, |e, a| ret(e.mem.u32(a[0])));
        e.register(CHILD_AT, |e, a| ret(e.mem.u32(a[0] + 4 + 4 * a[1])));
        let plain = node(&mut e, 0, [0.0; 3], &[]);
        let first = node(&mut e, RTTI_BS_BLAST_NODE, [1.0, 2.0, 3.0], &[]);
        let second = node(&mut e, RTTI_BS_BLAST_NODE, [4.0, 5.0, 6.0], &[]);
        let middle = node(&mut e, 0, [0.0; 3], &[plain, first]);
        let root = node(&mut e, 0, [0.0; 3], &[middle, second]);
        let out = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert!(e.call(0x009c_6380, &args![0u32, root, out]).bool());
        assert_eq!(e.mem.f32(out), 1.0);
        assert_eq!(e.mem.f32(out + 8), 3.0);
        // The search stops after the first find: the second is never asked.
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == IS_KIND_OF && c.1[1] == second));
        assert!(!e.call(0x009c_6380, &args![0u32, plain, out]).bool());
        assert!(!e.call(0x009c_6380, &args![0u32, 0u32, out]).bool());
    }
}
