//! `NiXenonRenderer/run_00e944f0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The allocation `00aa13e0` (`cdecl`, size), the chip clear `00e942d0` (`run_00e941a0`;
/// `thiscall`), the `NiTPrimitiveArray<NiVBChip *>` table (`010f0f8c`, by its RTTI), the head
/// of the free chip list (`0126fa30`; next +0x3c, previous +0x40) and the number of chips one
/// refill makes (`011c0cdc`).
const ALLOCATE: u32 = 0x00aa_13e0;
const CHIP_CLEAR_00E942D0: u32 = 0x00e9_42d0;
const CHIP_PRIMITIVE_ARRAY_VTABLE: u32 = 0x010f_0f8c;
const FREE_CHIPS_0126FA30: u32 = 0x0126_fa30;
const REFILL_COUNT_011C0CDC: u32 = 0x011c_0cdc;

// Translated from 00e944f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: refills the free chip list (`cdecl`, no arguments). As many
/// times as the word at `011c0cdc` says, a 0x44-byte chip (`00aa13e0`) gets its array member
/// (+0x2c: the `NiTPrimitiveArray<NiVBChip *>` table, no elements, growth 1 at +0x3a), is
/// cleared (`00e942d0`), its neighbours (next +0x3c, previous +0x40) are linked past it and
/// it goes to the head of the free list `0126fa30`. A failed allocation leaves a null chip,
/// whose words the exe then reads.
pub fn fn_00e944f0(e: &mut Engine) {
    let mut made = 0u32;
    while made < e.mem.u32(REFILL_COUNT_011C0CDC) {
        let block = e.call(ALLOCATE, &args![0x44u32]).u32();
        let chip = if block != 0 {
            e.mem.set_u16(block.wrapping_add(0x3a), 1);
            e.mem.set_u16(block.wrapping_add(0x34), 0);
            e.mem.set_u16(block.wrapping_add(0x36), 0);
            e.mem.set_u16(block.wrapping_add(0x38), 0);
            e.mem.set_u32(block.wrapping_add(0x30), 0);
            e.mem
                .set_u32(block.wrapping_add(0x2c), CHIP_PRIMITIVE_ARRAY_VTABLE);
            e.call(CHIP_CLEAR_00E942D0, &args![block]);
            block
        } else {
            0
        };
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
        made += 1;
        e.mem.set_u32(FREE_CHIPS_0126FA30, chip);
    }
}

/// The device's `CreateVertexBuffer` (slot 0x68, by the interface's order) and the array add
/// `00a64330` (`thiscall` on the block's chip array at +0x1c, the address of the chip word;
/// answers its index; the engine map's `NiUnionBV::AddBoundingVolume` is a folded body).
const CREATE_VERTEX_BUFFER: u32 = 0x68;
const ARRAY_ADD_00A64330: u32 = 0x00a6_4330;

// Translated from 00e94580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a vertex buffer block's new chip (`thiscall`, the size). Unless
/// bit 28 of the block's usage word (+8) is set the size is at least the block's +0xc. The
/// device (+0x10) creates a vertex buffer (`CreateVertexBuffer`: the size, the usage +0x14,
/// the vertex format +4, the pool +0x18, the out word, 0); failing, 0 is answered. A chip comes
/// off the free list (refilled by `00e944f0` when empty) and gets the block (+0), the buffer
/// (+8), the size (+0xc, +0x24, +0x28), the block's +4, +8, +0x18 and +0x14 (at +0x10, +0x14,
/// +0x1c, +0x18) and its index in the block's chip array (+4, `00a64330`). Answers the chip.
///
/// The exe's out word is the stack word of the size argument.
pub fn fn_00e94580(e: &mut Engine, this: Ptr, size: u32) -> u32 {
    let base = this.addr();
    let mut size = size;
    if e.mem.u32(base.wrapping_add(8)) & 0x1000_0000 == 0 {
        let least = e.mem.u32(base.wrapping_add(0xc));
        if size <= least {
            size = least;
        }
    }
    e.with_stack(8, |e, words| {
        let out = words.addr();
        let local = out + 4;
        let device = e.mem.u32(base.wrapping_add(0x10));
        let pool = e.mem.u32(base.wrapping_add(0x18));
        let format = e.mem.u32(base.wrapping_add(4));
        let usage = e.mem.u32(base.wrapping_add(0x14));
        e.mem.set_u32(out, 0);
        if e.vcall(
            device,
            CREATE_VERTEX_BUFFER,
            &args![size, usage, format, pool, out, 0u32],
        )
        .i32()
            < 0
        {
            return 0;
        }
        let mut chip = e.mem.u32(FREE_CHIPS_0126FA30);
        if chip == 0 {
            fn_00e944f0(e);
            chip = e.mem.u32(FREE_CHIPS_0126FA30);
        }
        let next = e.mem.u32(chip.wrapping_add(0x3c));
        e.mem.set_u32(FREE_CHIPS_0126FA30, next);
        if next != 0 {
            e.mem.set_u32(next.wrapping_add(0x40), 0);
        }
        e.mem.set_u32(chip.wrapping_add(0x3c), 0);
        e.mem.set_u32(chip.wrapping_add(0x40), 0);
        e.mem.set_u32(chip, base);
        let buffer = e.mem.u32(out);
        e.mem.set_u32(chip.wrapping_add(8), buffer);
        e.mem.set_u32(chip.wrapping_add(0xc), size);
        e.mem.set_u32(chip.wrapping_add(0x24), size);
        e.mem.set_u32(chip.wrapping_add(0x28), size);
        for (from, to) in [(4u32, 0x10u32), (8, 0x14), (0x18, 0x1c), (0x14, 0x18)] {
            let word = e.mem.u32(base.wrapping_add(from));
            e.mem.set_u32(chip.wrapping_add(to), word);
        }
        e.mem.set_u32(local, chip);
        let index = e
            .call(ARRAY_ADD_00A64330, &args![base.wrapping_add(0x1c), local])
            .u32();
        e.mem.set_u32(chip.wrapping_add(4), index);
        chip
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_44f0, fn_00e944f0()),
        entry!(0x00e9_4580, fn_00e94580(Ptr, u32) -> u32),
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

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(FREE_CHIPS_0126FA30 & !0xfff, 0x1000);
        e.map(REFILL_COUNT_011C0CDC & !0xfff, 0x1000);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CHIP_CLEAR_00E942D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00e944f0_puts_new_chips_at_the_head() {
        let mut e = engine();
        e.mem.set_u32(REFILL_COUNT_011C0CDC, 2);
        let old = e.mem.alloc(0x44);
        e.mem.set_u32(FREE_CHIPS_0126FA30, old);
        e.call(0x00e9_44f0, &args![]);
        let made: Vec<u32> = calls(&e, CHIP_CLEAR_00E942D0)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(made.len(), 2);
        assert_eq!(e.mem.u32(FREE_CHIPS_0126FA30), made[1]);
        assert_eq!(e.mem.u32(made[1] + 0x3c), made[0]);
        assert_eq!(e.mem.u32(made[0] + 0x3c), old);
        assert_eq!(e.mem.u32(old + 0x40), made[0]);
        assert_eq!(e.mem.u32(made[0] + 0x2c), CHIP_PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(made[0] + 0x3a), 1);
    }

    #[test]
    fn fn_00e94580_takes_a_chip_for_a_new_buffer() {
        let mut e = engine();
        e.mem.set_u32(REFILL_COUNT_011C0CDC, 1);
        e.put_vtable(0x6000, &[0; 0x1b]);
        e.mem.set_u32(0x6000 + CREATE_VERTEX_BUFFER, 0x6068);
        e.register(0x6068, |e, a| {
            e.mem.set_u32(a[5], 0x0e0b_0000);
            Ret::default()
        });
        e.register(ARRAY_ADD_00A64330, |_, _| 7u32.into_ret());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x6000);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 4, 0x142);
        e.mem.set_u32(this + 8, 0x208);
        e.mem.set_u32(this + 0xc, 0x1000);
        e.mem.set_u32(this + 0x10, device);
        e.mem.set_u32(this + 0x14, 0x200);
        e.mem.set_u32(this + 0x18, 1);
        // The free list is empty: one refill, and that chip is taken.
        let chip = e.call(0x00e9_4580, &args![this, 0x40u32]).u32();
        assert_eq!(calls(&e, CHIP_CLEAR_00E942D0), vec![vec![chip]]);
        assert_eq!(e.mem.u32(FREE_CHIPS_0126FA30), 0);
        assert_eq!(calls(&e, 0x6068)[0][..5], [device, 0x1000, 0x200, 0x142, 1]);
        assert_eq!(e.mem.u32(chip), this);
        assert_eq!(e.mem.u32(chip + 8), 0x0e0b_0000);
        for at in [0xc, 0x24, 0x28] {
            assert_eq!(e.mem.u32(chip + at), 0x1000);
        }
        assert_eq!(e.mem.u32(chip + 0x10), 0x142);
        assert_eq!(e.mem.u32(chip + 0x14), 0x208);
        assert_eq!(e.mem.u32(chip + 0x18), 0x200);
        assert_eq!(e.mem.u32(chip + 0x1c), 1);
        assert_eq!(e.mem.u32(chip + 4), 7);
        assert_eq!(calls(&e, ARRAY_ADD_00A64330)[0][0], this + 0x1c);
        // Bit 28 keeps the asked size; a failed create answers 0.
        e.mem.set_u32(this + 8, 0x1000_0000);
        e.call_log = Some(vec![]);
        e.call(0x00e9_4580, &args![this, 0x40u32]);
        assert_eq!(calls(&e, 0x6068)[0][1], 0x40);
        e.register(0x6068, |_, _| 0x8876_0001u32.into_ret());
        assert_eq!(e.call(0x00e9_4580, &args![this, 0x40u32]).u32(), 0);
    }
}
