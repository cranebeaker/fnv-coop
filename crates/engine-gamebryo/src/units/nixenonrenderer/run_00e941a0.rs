//! `NiXenonRenderer/run_00e941a0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00e98610` (unnamed in the engine map; no arguments): runs first.
const RELEASE_00E98610: u32 = 0x00e9_8610;
/// The head of the singly linked list `fn_00e94290` frees (`0126fa30`; each node's next at
/// +0x3c, 0x44 bytes each).
const LIST_HEAD_0126FA30: u32 = 0x0126_fa30;
/// The node destructor body `00e98640` (unnamed; `thiscall`, no stack words).
const NODE_DESTRUCT: u32 = 0x00e9_8640;
/// The sized Ni free `00aa1460` (`cdecl`: block, size).
const NI_DELETE: u32 = 0x00aa_1460;

// Translated from 00e94290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): runs `00e98610`, then destroys and frees every
/// node of the list headed at `0126fa30` (the next node read before each one goes: +0x3c;
/// `00e98640`, then the 0x44 bytes freed) and clears the head.
pub fn fn_00e94290(e: &mut Engine) {
    e.call(RELEASE_00E98610, &args![]);
    let mut node = e.mem.u32(LIST_HEAD_0126FA30);
    while node != 0 {
        let next = e.mem.u32(node.wrapping_add(0x3c));
        e.call(NODE_DESTRUCT, &args![node]);
        e.call(NI_DELETE, &args![node, 0x44u32]);
        node = next;
    }
    e.mem.set_u32(LIST_HEAD_0126FA30, 0);
}

/// The dynamic-texture flag (`0126f0d0`, a byte).
const DYNAMIC_FLAG_0126F0D0: u32 = 0x0126_f0d0;

// Translated from 00e941a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e941a0`), `cdecl` with a buffer description: its +0x14
/// becomes 8, or 0x18 when bit 31 of +8 is set, and +0x18 becomes 1 when the byte at
/// `0126f0d0` is clear, else 0. Answers the description (left in `EAX`).
pub fn fn_00e941a0(e: &mut Engine, desc: u32) -> u32 {
    let flags = e.mem.u32(desc.wrapping_add(8));
    e.mem.set_u32(desc.wrapping_add(0x14), 8);
    if flags & 0x8000_0000 != 0 {
        e.mem.set_u32(desc.wrapping_add(0x14), 0x18);
    }
    let value = u32::from(e.mem.u8(DYNAMIC_FLAG_0126F0D0) == 0);
    e.mem.set_u32(desc.wrapping_add(0x18), value);
    desc
}

// Translated from 00e941d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e941a0`), `thiscall` with a size: takes from the list at
/// +0x2c (next +0x3c, previous +0x40; sorted by the size word +0x24) the first node of at
/// least that size: its neighbours are linked past it and the head moves on when it was the
/// head. Answers the node, or 0.
pub fn fn_00e941d0(e: &mut Engine, this: Ptr, size: u32) -> u32 {
    let base = this.addr();
    let mut node = e.mem.u32(base.wrapping_add(0x2c));
    while node != 0 {
        if e.mem.u32(node.wrapping_add(0x24)) >= size {
            let next = e.mem.u32(node.wrapping_add(0x3c));
            if next != 0 {
                let previous = e.mem.u32(node.wrapping_add(0x40));
                e.mem.set_u32(next.wrapping_add(0x40), previous);
            }
            let previous = e.mem.u32(node.wrapping_add(0x40));
            if previous != 0 {
                let next = e.mem.u32(node.wrapping_add(0x3c));
                e.mem.set_u32(previous.wrapping_add(0x3c), next);
            }
            if node == e.mem.u32(base.wrapping_add(0x2c)) {
                let next = e.mem.u32(node.wrapping_add(0x3c));
                e.mem.set_u32(base.wrapping_add(0x2c), next);
            }
            return node;
        }
        node = e.mem.u32(node.wrapping_add(0x3c));
    }
    0
}

// Translated from 00e94220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e941a0`), `thiscall` with a node: puts it into the list
/// at +0x2c (next +0x3c, previous +0x40) before the first node whose size (+0x24) is greater,
/// or at the end; into an empty list as the only node; a node put first becomes the head.
pub fn fn_00e94220(e: &mut Engine, this: Ptr, node: u32) {
    let base = this.addr();
    let head = e.mem.u32(base.wrapping_add(0x2c));
    if head == 0 {
        e.mem.set_u32(base.wrapping_add(0x2c), node);
        e.mem.set_u32(node.wrapping_add(0x3c), 0);
        let head = e.mem.u32(base.wrapping_add(0x2c));
        e.mem.set_u32(head.wrapping_add(0x40), 0);
        return;
    }
    let size = e.mem.u32(node.wrapping_add(0x24));
    let mut previous = 0u32;
    let mut current = head;
    while current != 0 {
        if e.mem.u32(current.wrapping_add(0x24)) > size {
            if previous != 0 {
                e.mem.set_u32(previous.wrapping_add(0x3c), node);
            }
            e.mem.set_u32(current.wrapping_add(0x40), node);
            e.mem.set_u32(node.wrapping_add(0x3c), current);
            e.mem.set_u32(node.wrapping_add(0x40), previous);
            if current == e.mem.u32(base.wrapping_add(0x2c)) {
                e.mem.set_u32(base.wrapping_add(0x2c), node);
            }
            return;
        }
        previous = current;
        current = e.mem.u32(current.wrapping_add(0x3c));
    }
    e.mem.set_u32(previous.wrapping_add(0x3c), node);
    e.mem.set_u32(node.wrapping_add(0x40), previous);
    e.mem.set_u32(node.wrapping_add(0x3c), current);
}

// Translated from 00e942d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e941a0`), `thiscall`: clears the words +0 to +0x28, the
/// first 16-bit +0x36 words of the array at +0x30, the 16-bit counts +0x36 and +0x38 and the
/// links +0x3c and +0x40.
pub fn fn_00e942d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for offset in [0u32, 8, 0xc, 0x10, 0x14, 4, 0x18, 0x1c, 0x20, 0x24, 0x28] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(base.wrapping_add(0x36))) {
        let array = e.mem.u32(base.wrapping_add(0x30));
        e.mem.set_u32(array.wrapping_add(index * 4), 0);
        index = (index + 1) & 0xffff;
    }
    e.mem.set_u16(base.wrapping_add(0x36), 0);
    e.mem.set_u16(base.wrapping_add(0x38), 0);
    e.mem.set_u32(base.wrapping_add(0x3c), 0);
    e.mem.set_u32(base.wrapping_add(0x40), 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_41a0, fn_00e941a0(u32) -> u32),
        entry!(0x00e9_41d0, fn_00e941d0(Ptr, u32) -> u32),
        entry!(0x00e9_4220, fn_00e94220(Ptr, u32)),
        entry!(0x00e9_42d0, fn_00e942d0(Ptr)),
        entry!(0x00e9_4290, fn_00e94290()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00e94290_frees_every_node_and_clears_the_head() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        for callee in [RELEASE_00E98610, NODE_DESTRUCT] {
            e.register(callee, |_, _| Ret::default());
        }
        // The free clobbers the node's next word: the loop read it before.
        e.register(NI_DELETE, |e, a| {
            e.mem.set_u32(a[0] + 0x3c, 0xdead_beef);
            Ret::default()
        });
        let first = e.mem.alloc(0x44);
        let second = e.mem.alloc(0x44);
        e.mem.set_u32(first + 0x3c, second);
        e.set_global(LIST_HEAD_0126FA30, first);
        e.call_log = Some(vec![]);
        e.call(0x00e9_4290, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (RELEASE_00E98610, vec![]),
                (NODE_DESTRUCT, vec![first]),
                (NI_DELETE, vec![first, 0x44]),
                (NODE_DESTRUCT, vec![second]),
                (NI_DELETE, vec![second, 0x44]),
            ]
        );
        assert_eq!(e.mem.u32(LIST_HEAD_0126FA30), 0);

        // An empty list: only the first call.
        e.call_log = Some(vec![]);
        e.call(0x00e9_4290, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}

#[cfg(test)]
mod n011_tests {
    use super::*;

    #[test]
    fn fn_00e941a0_sets_the_description_words() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let desc = e.mem.alloc(0x1c);
        assert_eq!(e.call(0x00e9_41a0, &args![desc]).u32(), desc);
        assert_eq!([e.mem.u32(desc + 0x14), e.mem.u32(desc + 0x18)], [8, 1]);
        e.mem.set_u32(desc + 8, 0x8000_0000);
        e.mem.set_u8(DYNAMIC_FLAG_0126F0D0, 1);
        e.call(0x00e9_41a0, &args![desc]);
        assert_eq!([e.mem.u32(desc + 0x14), e.mem.u32(desc + 0x18)], [0x18, 0]);
    }

    fn node(e: &mut Engine, size: u32) -> u32 {
        let node = e.mem.alloc(0x44);
        e.mem.set_u32(node + 0x24, size);
        node
    }

    /// The sizes along the list from the head, checking the back links.
    fn sizes(e: &Engine, this: u32) -> Vec<u32> {
        let mut out = vec![];
        let mut previous = 0;
        let mut node = e.mem.u32(this + 0x2c);
        while node != 0 {
            assert_eq!(e.mem.u32(node + 0x40), previous);
            out.push(e.mem.u32(node + 0x24));
            previous = node;
            node = e.mem.u32(node + 0x3c);
        }
        out
    }

    #[test]
    fn fn_00e94220_keeps_the_list_sorted_and_fn_00e941d0_takes_the_first_fit() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x44);
        let nodes: Vec<u32> = [30u32, 10, 20, 40, 5]
            .iter()
            .map(|&s| node(&mut e, s))
            .collect();
        for &n in &nodes {
            e.call(0x00e9_4220, &args![this, n]);
        }
        assert_eq!(sizes(&e, this), vec![5, 10, 20, 30, 40]);
        // The first node of at least 15: the 20 from the middle.
        assert_eq!(e.call(0x00e9_41d0, &args![this, 15u32]).u32(), nodes[2]);
        assert_eq!(sizes(&e, this), vec![5, 10, 30, 40]);
        // The head and the tail.
        assert_eq!(e.call(0x00e9_41d0, &args![this, 0u32]).u32(), nodes[4]);
        assert_eq!(e.call(0x00e9_41d0, &args![this, 35u32]).u32(), nodes[3]);
        assert_eq!(sizes(&e, this), vec![10, 30]);
        assert_eq!(e.call(0x00e9_41d0, &args![this, 99u32]).u32(), 0);
    }

    #[test]
    fn fn_00e942d0_clears_the_record() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x44);
        for offset in (0..0x44).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        let array = e.mem.alloc(0x10);
        for i in 0..4u32 {
            e.mem.set_u32(array + 4 * i, 0x77);
        }
        e.mem.set_u32(this + 0x30, array);
        e.mem.set_u16(this + 0x36, 3);
        e.call(0x00e9_42d0, &args![this]);
        for offset in (0..0x2c).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x2c), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x34), 0x0000_ffff);
        assert_eq!(e.mem.u32(this + 0x38), 0xffff_0000);
        assert_eq!([e.mem.u32(this + 0x3c), e.mem.u32(this + 0x40)], [0, 0]);
        assert_eq!(
            (0..4u32)
                .map(|i| e.mem.u32(array + 4 * i))
                .collect::<Vec<_>>(),
            vec![0, 0, 0, 0x77]
        );
    }
}
