//! `(unplaced)/run_00ebf7d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`MemoryManager`) the tile set is allocated from (`00aa3e40`) and freed
/// to (`00aa4060`).
const MEMORY_MANAGER_011F6238: u32 = 0x011f_6238;
/// The byte that chooses the retry texture's usage and pool in `00ebf7d0`.
const RETRY_DYNAMIC_0126F0D0: u32 = 0x0126_f0d0;
/// `memset` (`00ec61c0`, `cdecl`).
const MEMSET: u32 = 0x00ec_61c0;

// Translated from 00ebf7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words; the third, a row pitch the caller passes, is
/// not read): creates a `width` x `height` texture of `format` through the device's
/// `IDirect3DDevice9` vtable slot `0x5c` (`CreateTexture(width, height, 1, 0x200, format, 0,
/// &texture, 0)`). On failure (a negative result) it retries with usage 0x200 and pool 0 when
/// the byte at `0126f0d0` is set, usage 0 and pool 1 otherwise; a second failure returns
/// whatever the texture word holds. Then the texture is locked (slot `0x4c`: `LockRect(0, &rect,
/// 0, 0)`; a failure returns the texture), each of the `height` rows of `width *
/// bytes_per_pixel` bytes is cleared (`memset`, stepping by the locked pitch) and it is
/// unlocked (slot `0x50`). Returns the texture.
pub fn fn_00ebf7d0(
    e: &mut Engine,
    device: u32,
    format: u32,
    _unused_3: u32,
    bytes_per_pixel: u32,
    width: u32,
    height: u32,
) -> u32 {
    e.with_stack(0xc, |e, locals| {
        let texture_slot = locals.addr();
        let rect = texture_slot + 4;
        e.mem.set_u32(texture_slot, 0);
        let created = e
            .vcall(
                device,
                0x5c,
                &args![
                    width,
                    height,
                    1u32,
                    0x200u32,
                    format,
                    0u32,
                    texture_slot,
                    0u32
                ],
            )
            .i32();
        if created < 0 {
            let dynamic = e.mem.u8(RETRY_DYNAMIC_0126F0D0) != 0;
            let usage = if dynamic { 0x200u32 } else { 0 };
            let pool = if dynamic { 0u32 } else { 1 };
            let retried = e
                .vcall(
                    device,
                    0x5c,
                    &args![width, height, 1u32, usage, format, pool, texture_slot, 0u32],
                )
                .i32();
            if retried < 0 {
                return e.mem.u32(texture_slot);
            }
        }
        let texture = e.mem.u32(texture_slot);
        if e.vcall(texture, 0x4c, &args![0u32, rect, 0u32, 0u32]).i32() < 0 {
            return e.mem.u32(texture_slot);
        }
        let pitch = e.mem.u32(rect);
        let mut row = e.mem.u32(rect + 4);
        let row_bytes = width.wrapping_mul(bytes_per_pixel);
        for _ in 0..height {
            e.call(MEMSET, &args![row, 0u32, row_bytes]);
            row = row.wrapping_add(pitch);
        }
        let texture = e.mem.u32(texture_slot);
        e.vcall(texture, 0x50, &args![0u32]);
        e.mem.u32(texture_slot)
    })
}

// Translated from 00ebfc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`): the smallest power of two from 4 to 0x400 that is at
/// least `value` (0x400 for anything above 0x200); a value of 2 or less is returned as it is.
pub fn fn_00ebfc80(_e: &mut Engine, value: u32) -> u32 {
    if value > 0x10 {
        if value > 0x40 {
            if value > 0x80 {
                if value > 0x100 {
                    if value > 0x200 {
                        0x400
                    } else {
                        0x200
                    }
                } else {
                    0x100
                }
            } else {
                0x80
            }
        } else if value > 0x20 {
            0x40
        } else {
            0x20
        }
    } else if value > 4 {
        if value > 8 {
            0x10
        } else {
            8
        }
    } else if value > 2 {
        4
    } else {
        value
    }
}

// Translated from 00ebf8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: `device`, `width`, `height`, `bytes_per_pixel`, `tile`,
/// `format`): makes a set of `tile`-sized textures covering `width` x `height`. The set
/// (0x48 bytes and a pointer per tile, from the memory manager `011f6238` through `00aa3e40`;
/// null when that fails) holds the device (`+0`), format (`+4`), bytes per pixel (`+8`), size
/// (`+0xc`, `+0x10`), the row pitch `(width * bpp + 15) & !15` (`+0x1c`), the tile count
/// `ceil(w / tile) * ceil(h / tile)` (`+0x20`), the full tiles across and down (`+0x24`,
/// `+0x28`), the remainders rounded up to a power of two (`00ebfc80`, `+0x2c`, `+0x30`) and as
/// they are (`+0x34`, `+0x38`), the tile size (`+0x3c`) and the texture table (`+0x40`, right
/// after the header). Each row of full tiles gets its textures (`00ebf7d0`) and a remainder
/// column; a remainder row follows. When any texture is missing, all the others are released
/// (vtable slot 8) and the set is freed (`00aa4060`), giving null; otherwise `+0x18` is cleared
/// and the set returned. (The exe also computes the widest and tallest tile into locals it
/// never reads.)
pub fn fn_00ebf8e0(
    e: &mut Engine,
    device: u32,
    width: u32,
    height: u32,
    bytes_per_pixel: u32,
    tile: u32,
    format: u32,
) -> u32 {
    let pitch = width.wrapping_mul(bytes_per_pixel).wrapping_add(0xf) & 0xffff_fff0;
    let extra_width = fn_00ebfc80(e, width % tile);
    let extra_height = fn_00ebfc80(e, height % tile);
    let across = width.wrapping_add(tile).wrapping_sub(1) / tile;
    let down = height.wrapping_add(tile).wrapping_sub(1) / tile;
    let count = across.wrapping_mul(down);
    let set = e
        .call(
            0x00aa_3e40,
            &args![
                MEMORY_MANAGER_011F6238,
                count.wrapping_mul(4).wrapping_add(0x48)
            ],
        )
        .u32();
    if set == 0 {
        return 0;
    }
    e.call(MEMSET, &args![set, 0u32, 0x48u32]);
    e.mem.set_u32(set + 0x40, set + 0x48);
    e.mem.set_u32(set, device);
    e.mem.set_u32(set + 0x20, count);
    e.mem.set_u32(set + 0x1c, pitch);
    e.mem.set_u32(set + 0xc, width);
    e.mem.set_u32(set + 0x10, height);
    e.mem.set_u32(set + 8, bytes_per_pixel);
    e.mem.set_u32(set + 0x24, width / tile);
    e.mem.set_u32(set + 0x28, height / tile);
    e.mem.set_u32(set + 0x2c, extra_width);
    e.mem.set_u32(set + 0x30, extra_height);
    e.mem.set_u32(set + 0x34, width % tile);
    e.mem.set_u32(set + 0x38, height % tile);
    e.mem.set_u32(set + 0x3c, tile);
    e.mem.set_u32(set + 4, format);
    let mut slot = e.mem.u32(set + 0x40);
    let mut make = |e: &mut Engine, tile_width: u32, tile_height: u32| {
        let format = e.mem.u32(set + 4);
        let pitch = e.mem.u32(set + 0x1c);
        let bytes = e.mem.u32(set + 8);
        let texture = fn_00ebf7d0(e, device, format, pitch, bytes, tile_width, tile_height);
        e.mem.set_u32(slot, texture);
        slot = slot.wrapping_add(4);
    };
    let mut row = 0u32;
    while row < e.mem.u32(set + 0x28) {
        let mut column = 0u32;
        while column < e.mem.u32(set + 0x24) {
            let size = e.mem.u32(set + 0x3c);
            make(e, size, size);
            column += 1;
        }
        if e.mem.u32(set + 0x2c) != 0 {
            let tile_width = e.mem.u32(set + 0x2c);
            let size = e.mem.u32(set + 0x3c);
            make(e, tile_width, size);
        }
        row += 1;
    }
    if e.mem.u32(set + 0x30) != 0 {
        let mut column = 0u32;
        while column < e.mem.u32(set + 0x24) {
            let size = e.mem.u32(set + 0x3c);
            let tile_height = e.mem.u32(set + 0x30);
            make(e, size, tile_height);
            column += 1;
        }
        if e.mem.u32(set + 0x2c) != 0 {
            let tile_width = e.mem.u32(set + 0x2c);
            let tile_height = e.mem.u32(set + 0x30);
            make(e, tile_width, tile_height);
        }
    }
    let mut index = 0u32;
    while index < e.mem.u32(set + 0x20) {
        let table = e.mem.u32(set + 0x40);
        if e.mem.u32(table + 4 * index) == 0 {
            let mut release = 0u32;
            while release < e.mem.u32(set + 0x20) {
                let table = e.mem.u32(set + 0x40);
                let texture = e.mem.u32(table + 4 * release);
                if texture != 0 {
                    e.vcall(texture, 8, &args![]);
                    let table = e.mem.u32(set + 0x40);
                    e.mem.set_u32(table + 4 * release, 0);
                }
                release += 1;
            }
            e.call(0x00aa_4060, &args![MEMORY_MANAGER_011F6238, set]);
            return 0;
        }
        index += 1;
    }
    e.mem.set_u32(set + 0x18, 0);
    set
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00eb_f7d0, fn_00ebf7d0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00eb_f8e0, fn_00ebf8e0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00eb_fc80, fn_00ebfc80(u32) -> u32),
    ]
}

#[cfg(test)]
mod b035_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const CREATE: u32 = 0x7100_005c;
    const LOCK: u32 = 0x7100_004c;
    const UNLOCK: u32 = 0x7100_0050;
    const RELEASE: u32 = 0x7100_0008;

    fn com_object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x60);
        for slot in (0..0x60).step_by(4) {
            e.mem.set_u32(vtable + slot, 0x7100_0000 + slot);
        }
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn fn_00ebfc80_rounds_up_to_a_power_of_two() {
        let mut e = Engine::new();
        for (value, expected) in [
            (0u32, 0u32),
            (2, 2),
            (3, 4),
            (5, 8),
            (9, 0x10),
            (0x10, 0x10),
            (0x11, 0x20),
            (0x21, 0x40),
            (0x41, 0x80),
            (0x81, 0x100),
            (0x101, 0x200),
            (0x201, 0x400),
            (0x9999, 0x400),
        ] {
            assert_eq!(
                e.call(0x00eb_fc80, &args![value]).u32(),
                expected,
                "{value:#x}"
            );
        }
    }

    #[test]
    fn fn_00ebf7d0_creates_clears_and_unlocks() {
        let mut e = Engine::new();
        e.map(RETRY_DYNAMIC_0126F0D0 & !0xfff, 0x1000);
        let device = com_object(&mut e);
        let texture = com_object(&mut e);
        let bits = e.mem.alloc(0x100);
        // The first creation fails, the retry succeeds.
        let tries = Rc::new(RefCell::new(0));
        let count = tries.clone();
        e.register_double(CREATE, move |e, a| {
            *count.borrow_mut() += 1;
            if *count.borrow() == 1 {
                return 0x8000_0000u32.into_ret();
            }
            e.mem.set_u32(a[7], texture);
            0u32.into_ret()
        });
        e.register_double(LOCK, move |e, a| {
            e.mem.set_u32(a[2], 0x20);
            e.mem.set_u32(a[2] + 4, bits);
            0u32.into_ret()
        });
        e.register(UNLOCK, |_, _| Ret::default());
        e.register(MEMSET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let result = e
            .call(
                0x00eb_f7d0,
                &args![device, 0x16u32, 0x40u32, 4u32, 6u32, 3u32],
            )
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, texture);
        let created = calls(&log, CREATE);
        assert_eq!(created[0][1..7], [6, 3, 1, 0x200, 0x16, 0]);
        // The byte at 0126f0d0 is 0: usage 0, pool 1.
        assert_eq!(created[1][1..7], [6, 3, 1, 0, 0x16, 1]);
        assert_eq!(
            calls(&log, MEMSET),
            vec![
                vec![bits, 0, 24],
                vec![bits + 0x20, 0, 24],
                vec![bits + 0x40, 0, 24]
            ]
        );
        assert_eq!(calls(&log, UNLOCK), vec![vec![texture, 0]]);
        // Both creations failing return the (empty) texture word.
        e.register(CREATE, |_, _| 0x8000_0000u32.into_ret());
        assert_eq!(
            e.call(
                0x00eb_f7d0,
                &args![device, 0x16u32, 0x40u32, 4u32, 6u32, 3u32]
            )
            .u32(),
            0
        );
    }

    #[test]
    fn fn_00ebf8e0_tiles_the_area() {
        let mut e = Engine::new();
        e.map(RETRY_DYNAMIC_0126F0D0 & !0xfff, 0x1000);
        let device = com_object(&mut e);
        e.register(0x00aa_3e40, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(0x00aa_4060, |_, _| Ret::default());
        e.register(MEMSET, |_, _| Ret::default());
        let pool = [com_object(&mut e), com_object(&mut e)];
        let made = Rc::new(RefCell::new(vec![]));
        let record = made.clone();
        e.register_double(CREATE, move |e, a| {
            let texture = pool[record.borrow().len()];
            record.borrow_mut().push((a[1], a[2]));
            e.mem.set_u32(a[7], texture);
            0u32.into_ret()
        });
        // Locking fails: the texture is returned as it is.
        e.register(LOCK, |_, _| 0x8000_0000u32.into_ret());
        // 300 x 200 in tiles of 0x100: one full tile across and none down; the remainders
        // 44 and 200 round up to 0x40 and 0x100.
        let set = e
            .call(
                0x00eb_f8e0,
                &args![device, 300u32, 200u32, 4u32, 0x100u32, 0x16u32],
            )
            .u32();
        assert_ne!(set, 0);
        assert_eq!(e.mem.u32(set), device);
        assert_eq!(e.mem.u32(set + 4), 0x16);
        assert_eq!(e.mem.u32(set + 0x1c), 1200);
        assert_eq!(e.mem.u32(set + 0x20), 2);
        assert_eq!((e.mem.u32(set + 0x24), e.mem.u32(set + 0x28)), (1, 0));
        assert_eq!(
            (e.mem.u32(set + 0x2c), e.mem.u32(set + 0x30)),
            (0x40, 0x100)
        );
        assert_eq!((e.mem.u32(set + 0x34), e.mem.u32(set + 0x38)), (44, 200));
        assert_eq!(e.mem.u32(set + 0x40), set + 0x48);
        // No full row; the remainder row: one full-width tile and the corner.
        assert_eq!(*made.borrow(), vec![(0x100, 0x100), (0x40, 0x100)]);
        assert_eq!(
            (e.mem.u32(set + 0x48), e.mem.u32(set + 0x4c)),
            (pool[0], pool[1])
        );
        // A missing texture releases the others and frees the set.
        let fails = Rc::new(RefCell::new(0));
        let count = fails.clone();
        let textures = [com_object(&mut e), com_object(&mut e)];
        e.register_double(CREATE, move |e, a| {
            *count.borrow_mut() += 1;
            if (2..=3).contains(&*count.borrow()) {
                return 0x8000_0000u32.into_ret();
            }
            e.mem.set_u32(a[7], textures[0]);
            0u32.into_ret()
        });
        e.register(RELEASE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let set = e
            .call(
                0x00eb_f8e0,
                &args![device, 300u32, 200u32, 4u32, 0x100u32, 0x16u32],
            )
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(set, 0);
        assert_eq!(calls(&log, RELEASE), vec![vec![textures[0]]]);
        assert_eq!(calls(&log, 0x00aa_4060).len(), 1);
    }
}
