//! `NiXenonRenderer/run_00e94390` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The tables these functions store (by their RTTI): `NiTArray<NiVBChip *, ...>` (`010f0f7c`)
/// and `NiTArray<NiVBBlock *, ...>` (`010f0f84`).
const CHIP_ARRAY_VTABLE: u32 = 0x010f_0f7c;
const BLOCK_ARRAY_VTABLE: u32 = 0x010f_0f84;
/// The free `00aa10f0` (`cdecl`, a block), the memory manager singleton (`011f6238`) and its
/// free (`00aa4060`, `thiscall`, block).
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The chip clear `00e942d0` (`run_00e941a0`; `thiscall`) and the head of the free chip list
/// (`0126fa30`; next +0x3c, previous +0x40).
const CHIP_CLEAR_00E942D0: u32 = 0x00e9_42d0;
const FREE_CHIPS_0126FA30: u32 = 0x0126_fa30;

// Translated from 00e94390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of an `NiTArray<NiVBChip *>` (the
/// RTTI of the table it stores), `thiscall` with the flags: its table, the element array (+4)
/// freed (`00aa10f0`), and with bit 0 the block goes back to the memory manager (`00aa4060`).
/// Answers `this`.
pub fn fn_00e94390(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    let array = e.mem.u32(base.wrapping_add(4));
    e.mem.set_u32(base, CHIP_ARRAY_VTABLE);
    e.call(FREE_00AA10F0, &args![array]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, base]);
    }
    this
}

/// Puts a chip at the head of the free chip list (`0126fa30`) after linking its own
/// neighbours (next +0x3c, previous +0x40) past it.
fn push_free_chip(e: &mut Engine, chip: u32) {
    let next = e.mem.u32(chip.wrapping_add(0x3c));
    let previous = e.mem.u32(chip.wrapping_add(0x40));
    if next != 0 {
        e.mem.set_u32(next.wrapping_add(0x40), previous);
    }
    if previous != 0 {
        e.mem.set_u32(previous.wrapping_add(0x3c), next);
    }
    let head = e.mem.u32(FREE_CHIPS_0126FA30);
    if head != 0 {
        e.mem.set_u32(head.wrapping_add(0x40), chip);
    }
    let head = e.mem.u32(FREE_CHIPS_0126FA30);
    e.mem.set_u32(chip.wrapping_add(0x3c), head);
    e.mem.set_u32(chip.wrapping_add(0x40), 0);
    e.mem.set_u32(FREE_CHIPS_0126FA30, chip);
}

// Translated from 00e943c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a vertex buffer block's destructor body (`thiscall`). Each slot
/// of its chip array (+0x20, 16-bit size +0x26) is cleared; a chip counts the 16-bit +0x28
/// down; clearing the last slot shrinks the size; the chip's buffer (+8) is released (slot 8)
/// and cleared, the chip cleared (`00e942d0`) and put on the free chip list. Then the block's
/// buffer (+0x10) is released when set and cleared, the array member at +0x1c gets the
/// `NiTArray<NiVBBlock *>` table and its elements (+0x20) are freed (`00aa10f0`).
pub fn fn_00e943c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let size = |e: &Engine| u32::from(e.mem.u16(base.wrapping_add(0x26)));
    if size(e) != 0 {
        let mut index = 0u32;
        loop {
            if index < size(e) {
                let slot = e
                    .mem
                    .u32(base.wrapping_add(0x20))
                    .wrapping_add(index.wrapping_mul(4));
                let chip = e.mem.u32(slot);
                e.mem.set_u32(slot, 0);
                if chip != 0 {
                    let used = e.mem.u16(base.wrapping_add(0x28)).wrapping_add(0xffff);
                    e.mem.set_u16(base.wrapping_add(0x28), used);
                }
                let last = e.mem.u16(base.wrapping_add(0x26)).wrapping_sub(1);
                if index == u32::from(last) {
                    e.mem.set_u16(base.wrapping_add(0x26), last);
                }
                if chip != 0 {
                    let buffer = e.mem.u32(chip.wrapping_add(8));
                    if buffer != 0 {
                        e.vcall(buffer, 8, &args![]);
                        e.mem.set_u32(chip.wrapping_add(8), 0);
                    }
                    e.call(CHIP_CLEAR_00E942D0, &args![chip]);
                    push_free_chip(e, chip);
                }
            }
            index += 1;
            if index >= size(e) {
                break;
            }
        }
    }
    let buffer = e.mem.u32(base.wrapping_add(0x10));
    if buffer != 0 {
        e.vcall(buffer, 8, &args![]);
    }
    e.mem.set_u32(base.wrapping_add(0x10), 0);
    let elements = e.mem.u32(base.wrapping_add(0x20));
    e.mem.set_u32(base.wrapping_add(0x1c), BLOCK_ARRAY_VTABLE);
    e.call(FREE_00AA10F0, &args![elements]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_4390, fn_00e94390(Ptr, u32) -> Ptr),
        entry!(0x00e9_43c0, fn_00e943c0(Ptr)),
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
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00e94390_frees_the_elements_and_the_block_on_bit_zero() {
        for (flags, frees) in [(0u32, false), (1, true)] {
            let mut e = Engine::new();
            e.register(FREE_00AA10F0, |_, _| Ret::default());
            e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 4, 0x0e0a_0000);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00e9_4390, &args![this, flags]).u32(), this);
            assert_eq!(e.mem.u32(this), CHIP_ARRAY_VTABLE);
            assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x0e0a_0000]]);
            assert_eq!(calls(&e, MEMORY_DEALLOCATE).len(), usize::from(frees));
        }
    }

    #[test]
    fn fn_00e943c0_returns_every_chip_to_the_free_list() {
        let mut e = Engine::new();
        e.map(FREE_CHIPS_0126FA30 & !0xfff, 0x1000);
        e.put_vtable(0x6000, &[0, 0, 0x6008]);
        e.register(0x6008, |_, _| Ret::default());
        e.register(CHIP_CLEAR_00E942D0, |_, _| Ret::default());
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        let buffer = e.mem.alloc(4);
        e.mem.set_u32(buffer, 0x6000);
        let old = e.mem.alloc(0x44);
        e.mem.set_u32(FREE_CHIPS_0126FA30, old);
        let chips: Vec<u32> = (0..2).map(|_| e.mem.alloc(0x44)).collect();
        e.mem.set_u32(chips[0] + 8, buffer);
        let array = e.mem.alloc(0xc);
        e.mem.set_u32(array, chips[0]);
        e.mem.set_u32(array + 8, chips[1]);
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + 0x20, array);
        e.mem.set_u16(this + 0x26, 3);
        e.mem.set_u16(this + 0x28, 2);
        e.mem.set_u32(this + 0x10, buffer);
        e.call_log = Some(vec![]);
        e.call(0x00e9_43c0, &args![this]);
        assert_eq!(e.mem.u16(this + 0x28), 0);
        // The last slot shrank the size to 2.
        assert_eq!(e.mem.u16(this + 0x26), 2);
        assert_eq!(
            calls(&e, CHIP_CLEAR_00E942D0),
            vec![vec![chips[0]], vec![chips[1]]]
        );
        // The first chip's buffer and the block's own released.
        assert_eq!(calls(&e, 0x6008).len(), 2);
        assert_eq!(e.mem.u32(chips[0] + 8), 0);
        assert_eq!(e.mem.u32(FREE_CHIPS_0126FA30), chips[1]);
        assert_eq!(e.mem.u32(chips[1] + 0x3c), chips[0]);
        assert_eq!(e.mem.u32(chips[0] + 0x3c), old);
        assert_eq!(e.mem.u32(old + 0x40), chips[0]);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x1c), BLOCK_ARRAY_VTABLE);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![array]]);
    }
}
