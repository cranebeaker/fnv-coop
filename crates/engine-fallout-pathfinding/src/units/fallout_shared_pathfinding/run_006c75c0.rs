//! `fallout shared/pathfinding/run_006c75c0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00401030` (cdecl: block): frees a block.
const FREE_BLOCK: u32 = 0x0040_1030;
/// `00401000` (cdecl: byte count): allocates a block.
const ALLOCATE_BLOCK: u32 = 0x0040_1000;
/// `00401460` (cdecl: destination, source, byte count): copies bytes.
const COPY_BYTES: u32 = 0x0040_1460;
/// `00404eb0` (`thiscall` on a 4-byte local; four words; `RET 0x10`) and
/// `00404ee0` (`thiscall`): construct and destroy the scope object
/// [`fn_006c7610`] keeps while it runs.
const SCOPE_CONSTRUCT_00404EB0: u32 = 0x0040_4eb0;
const SCOPE_DESTRUCT_00404EE0: u32 = 0x0040_4ee0;
/// `"d:\_fallout3\platforms\common\code\bsmain\ThreadSafeStructures.inl"`,
/// passed to the scope object with the line 0xd0.
const THREAD_SAFE_STRUCTURES_FILE: u32 = 0x0101_73f8;
/// `0044f3e0` (`thiscall`): the first word of the object at `this + 0`
/// (through `00559450`), used as a count.
const COUNT_0044F3E0: u32 = 0x0044_f3e0;
/// `00726070` (`thiscall`): the word at the node's `+4` (the next node).
const NODE_FOLLOWING_00726070: u32 = 0x0072_6070;
/// `006ecd40` (`thiscall`, `RET 4`: one word): stores the word at the
/// node's `+4`. (The engine map's name for it,
/// `CombatProcedureAttackMelee::Initialize`, comes from folded code.)
const NODE_SET_FOLLOWING_006ECD40: u32 = 0x006e_cd40;

// Translated from 006c75c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (`thiscall`; flags; `RET 4`): the destructor
/// body `006c75f0`, then the object is freed (`00401030`) when bit 0 of
/// `flags` is set. Returns `this`. The engine map gives it no name.
pub fn fn_006c75c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006c75f0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_BLOCK, &args![this]);
    }
    this
}

// Translated from 006c75f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `006c7610` on `this` while the count at `+0xc` is not 0
/// (`thiscall`, plain `RET`). The engine map gives it no name.
pub fn fn_006c75f0(e: &mut Engine, this: Ptr) {
    while e.mem.u32(this.addr() + 0xc) != 0 {
        fn_006c7610(e, this);
    }
}

// Translated from 006c7610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c7610 - the exception-unwinding frame (FS:[0]) that destroys the scope object on unwind is not translated.
/// Filters the linked list whose head is at `+0x10` and count at `+0xc`
/// (`thiscall`, plain `RET`), under a scope object (`00404eb0` with 6, 1,
/// the `ThreadSafeStructures.inl` file name and line 0xd0; `00404ee0` at
/// the end). Twice the number `0044f3e0` gives for the object at `+0` of
/// words are copied (`00401460`) from that object's `+0x10` into a new
/// block (`00401000`; the byte count saturates to 0xFFFFFFFF on overflow).
/// Each node is taken off the head (the head becomes its `00726070`); a
/// node found among those words is kept: its next word is set
/// (`006ecd40`) to the previously kept node, and it counts; any other gets
/// 0 as its next word and is freed (`00401030`). Then the block is freed,
/// the head is the last kept node and the count the number kept. The
/// engine map gives it no name.
pub fn fn_006c7610(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    // The scope object `[EBP-0x18]`.
    e.with_stack(4, |e, scope| {
        e.call(
            SCOPE_CONSTRUCT_00404EB0,
            &args![scope, 6u32, 1u32, THREAD_SAFE_STRUCTURES_FILE, 0xd0u32],
        );
        let owner = e.mem.u32(base);
        let words = e.call(COUNT_0044F3E0, &args![owner]).u32() << 1;
        let bytes = u64::from(words) * 4;
        let size = if bytes > u64::from(u32::MAX) {
            0xffff_ffff
        } else {
            bytes as u32
        };
        let copy = e.call(ALLOCATE_BLOCK, &args![size]).u32();
        let owner = e.mem.u32(base);
        let source = e.mem.u32(owner + 0x10);
        e.call(COPY_BYTES, &args![copy, source, words << 2]);
        let mut kept = 0u32;
        let mut kept_count = 0u32;
        while e.mem.u32(base + 0x10) != 0 {
            let node = e.mem.u32(base + 0x10);
            let next = e.call(NODE_FOLLOWING_00726070, &args![node]).u32();
            e.mem.set_u32(base + 0x10, next);
            let mut found = false;
            let mut index = 0u32;
            while index < words {
                if node == e.mem.u32(copy.wrapping_add(index.wrapping_mul(4))) {
                    found = true;
                    break;
                }
                index += 1;
            }
            if found {
                e.call(NODE_SET_FOLLOWING_006ECD40, &args![node, kept]);
                kept = node;
                kept_count = kept_count.wrapping_add(1);
            } else {
                e.call(NODE_SET_FOLLOWING_006ECD40, &args![node, 0u32]);
                e.call(FREE_BLOCK, &args![node]);
            }
        }
        e.call(FREE_BLOCK, &args![copy]);
        e.mem.set_u32(base + 0x10, kept);
        e.mem.set_u32(base + 0xc, kept_count);
        e.call(SCOPE_DESTRUCT_00404EE0, &args![scope]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006c_75c0, fn_006c75c0(Ptr, u32) -> Ptr),
        entry!(0x006c_75f0, fn_006c75f0(Ptr)),
        entry!(0x006c_7610, fn_006c7610(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A list object whose `+0` points at an owner holding `kept.len() / 2`
    /// at `+0` and the address of `kept` words at `+0x10`, with nodes
    /// (8 bytes, next at `+4`) linked from `+0x10`.
    fn list_case(e: &mut Engine, nodes: usize, kept: &[usize]) -> (u32, Vec<u32>) {
        e.register(SCOPE_CONSTRUCT_00404EB0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(SCOPE_DESTRUCT_00404EE0, |_, _| Ret::default());
        e.register(COUNT_0044F3E0, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(ALLOCATE_BLOCK, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(COPY_BYTES, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.register(NODE_FOLLOWING_00726070, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(NODE_SET_FOLLOWING_006ECD40, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        let list: Vec<u32> = (0..nodes).map(|_| e.mem.alloc(8)).collect();
        for pair in list.windows(2) {
            e.mem.set_u32(pair[0] + 4, pair[1]);
        }
        let words = e.mem.alloc(0x40);
        for (i, k) in kept.iter().enumerate() {
            e.mem.set_u32(words + 4 * i as u32, list[*k]);
        }
        let owner = e.mem.alloc(0x14);
        e.mem.set_u32(owner, (kept.len() / 2) as u32);
        e.mem.set_u32(owner + 0x10, words);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this, owner);
        e.mem.set_u32(this + 0xc, nodes as u32);
        e.mem
            .set_u32(this + 0x10, list.first().copied().unwrap_or(0));
        (this, list)
    }

    #[test]
    fn at_006c7610_keeps_the_listed_nodes_in_reverse() {
        let mut e = Engine::new();
        let (this, list) = list_case(&mut e, 4, &[2, 0]);
        e.call_log = Some(vec![]);
        e.call(0x006c_7610, &args![this]);
        let log = e.call_log.take().unwrap();
        // Nodes 0 and 2 kept, linked last first; 1 and 3 freed.
        assert_eq!(e.mem.u32(this + 0x10), list[2]);
        assert_eq!(e.mem.u32(list[2] + 4), list[0]);
        assert_eq!(e.mem.u32(list[0] + 4), 0);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        let freed: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == FREE_BLOCK)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(freed.len(), 3);
        assert_eq!(&freed[..2], &[list[1], list[3]]);
        // The block of two words: 8 bytes, then the scope object closes.
        let allocation = log.iter().find(|(a, _)| *a == ALLOCATE_BLOCK).unwrap();
        assert_eq!(allocation.1, vec![8]);
        let scope = log
            .iter()
            .find(|(a, _)| *a == SCOPE_CONSTRUCT_00404EB0)
            .unwrap();
        assert_eq!(&scope.1[1..], &[6, 1, THREAD_SAFE_STRUCTURES_FILE, 0xd0]);
        assert_eq!(log.last().unwrap().0, SCOPE_DESTRUCT_00404EE0);
    }

    #[test]
    fn at_006c7610_saturates_the_block_size() {
        let mut e = Engine::new();
        let (this, _) = list_case(&mut e, 0, &[]);
        // 0x8000_0000 words of 4 bytes overflow: the allocation asks for
        // 0xFFFFFFFF; the copy gets the wrapped byte count 0.
        let owner = e.mem.u32(this);
        e.mem.set_u32(owner, 0x4000_0000);
        e.register(ALLOCATE_BLOCK, |_, _| Ret {
            eax: 0x5000,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006c_7610, &args![this]);
        let log = e.call_log.take().unwrap();
        let allocation = log.iter().find(|(a, _)| *a == ALLOCATE_BLOCK).unwrap();
        assert_eq!(allocation.1, vec![0xffff_ffff]);
        let copy = log.iter().find(|(a, _)| *a == COPY_BYTES).unwrap();
        assert_eq!(copy.1[2], 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn at_006c75f0_filters_until_the_count_is_zero() {
        let mut e = Engine::new();
        let (this, _) = list_case(&mut e, 3, &[]);
        e.call_log = Some(vec![]);
        e.call(0x006c_75f0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        // One pass empties the list.
        let passes = log
            .iter()
            .filter(|(a, _)| *a == SCOPE_CONSTRUCT_00404EB0)
            .count();
        assert_eq!(passes, 1);
        // Nothing to do with a count of 0.
        e.call_log = Some(vec![]);
        e.call(0x006c_75f0, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn at_006c75c0_destroys_and_frees_on_bit_0() {
        let mut e = Engine::new();
        let (this, _) = list_case(&mut e, 0, &[]);
        e.mem.set_u32(this + 0xc, 0);
        for (flags, frees) in [(0u32, false), (1, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006c_75c0, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                log.iter().any(|(a, w)| *a == FREE_BLOCK && w[0] == this),
                frees
            );
        }
    }
}
