//! `BSMain/run_00c64640` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`) and its free (`00aa4060`; `thiscall`: block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// The point the group's sixteen points start as (`011f426c`, three words).
const DEFAULT_POINT: u32 = 0x011f_426c;

// Translated from 00c64640 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of a decal placement group (`DECAL_PLACEMENT_VECTOR_GROUP_DATA`, Xbox
/// PDB name of the type; the function is unnamed in the engine map; `thiscall`, plain
/// `RET`): the 16-bit count at `+0` cleared and both arrays of eight points (`+0x4` and
/// `+0x64`, 12 bytes each) set to the three words at `011f426c`. The two loops before that
/// count down over the arrays' (empty) point constructors and change nothing. Answers
/// `this`.
pub fn fn_00c64640(e: &mut Engine, this: Ptr) -> Ptr {
    let object = this.addr();
    e.mem.set_u16(object, 0);
    for index in 0..8 {
        for (first, part) in [
            (0x4u32, 0u32),
            (0x4, 1),
            (0x4, 2),
            (0x64, 0),
            (0x64, 1),
            (0x64, 2),
        ] {
            let word = e.global::<u32>(DEFAULT_POINT + 4 * part);
            e.mem.set_u32(object + first + 12 * index + 4 * part, word);
        }
    }
    this
}

// Translated from 00c64710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties a singly linked node list (head at `+0`, tail at
/// `+4`, count at `+8`; `thiscall`, plain `RET`). Each node (next pointer at `+0`) has its
/// word at `+8` cleared and is freed by the memory manager (`00aa4060`); then count, head
/// and tail are cleared.
pub fn fn_00c64710(e: &mut Engine, this: Ptr) {
    let list = this.addr();
    let mut node = e.mem.u32(list);
    while node != 0 {
        let next = e.mem.u32(node);
        e.mem.set_u32(node + 8, 0);
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, node]);
        node = next;
    }
    e.mem.set_u32(list + 8, 0);
    e.mem.set_u32(list, 0);
    e.mem.set_u32(list + 4, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_4640, fn_00c64640(Ptr) -> Ptr),
        entry!(0x00c6_4710, fn_00c64710(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_constructor_fills_both_point_arrays() {
        let mut e = Engine::new();
        e.map(DEFAULT_POINT, 12);
        e.set_global(DEFAULT_POINT, 0x11u32);
        e.set_global(DEFAULT_POINT + 4, 0x22u32);
        e.set_global(DEFAULT_POINT + 8, 0x33u32);
        let this = e.mem.alloc(0xc4);
        e.mem.set_u32(this, 0xffff_ffff);
        assert_eq!(e.call(0x00c6_4640, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this), 0);
        assert_eq!(e.mem.u16(this + 2), 0xffff);
        for index in 0..8 {
            for base in [0x4, 0x64] {
                let point = this + base + 12 * index;
                assert_eq!(
                    (e.mem.u32(point), e.mem.u32(point + 4), e.mem.u32(point + 8)),
                    (0x11, 0x22, 0x33)
                );
            }
        }
    }

    #[test]
    fn list_clear_frees_every_node() {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        let second = e.mem.alloc(0xc);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, 0x99);
        e.mem.set_u32(second + 8, 0x98);
        let list = e.mem.alloc(0xc);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(list + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x00c6_4710, &args![list]);
        let log = e.call_log.take().unwrap();
        let frees: Vec<_> = log
            .iter()
            .filter(|c| c.0 == MEMORY_MANAGER_FREE)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            frees,
            vec![vec![MEMORY_MANAGER, first], vec![MEMORY_MANAGER, second]]
        );
        assert_eq!((e.mem.u32(first + 8), e.mem.u32(second + 8)), (0, 0));
        assert_eq!(
            (e.mem.u32(list), e.mem.u32(list + 4), e.mem.u32(list + 8)),
            (0, 0, 0)
        );
    }
}
