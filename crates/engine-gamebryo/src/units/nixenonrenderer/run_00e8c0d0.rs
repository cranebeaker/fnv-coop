//! `NiXenonRenderer/run_00e8c0d0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unlock that pairs with the lock `00e8bff0` (unit `run_00e8bfa0`) on the object the
//! renderer keeps at +0x8BC.

#[allow(unused_imports)]
use crate::prelude::*;

/// The object's staging fields (unnamed; read from the code, as in `run_00e8bfa0`): the
/// staging block +0x40, the locked address +0x48, the locked size +0x4C and the critical
/// section +0x80 the lock entered.
const STAGING_BLOCK: u32 = 0x40;
const LOCKED_ADDRESS: u32 = 0x48;
const LOCKED_SIZE: u32 = 0x4c;
const LOCK_SECTION: u32 = 0x80;

/// `memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;
/// `LeaveCriticalSection` (KERNEL32 import slot).
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `IDirect3DVertexBuffer9::Unlock` (slot 12).
const VERTEX_BUFFER_UNLOCK: u32 = 0x30;

// Translated from 00e8c0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies the staging block (+0x40) back to the locked address
/// (+0x48, size +0x4C), clears the locked address and size, leaves the critical section
/// (+0x80) and unlocks `buffer` (`Unlock`, slot +0x30). Answers whether the unlock succeeded
/// (a result of 0 or more).
pub fn fn_00e8c0d0(e: &mut Engine, this: Ptr, buffer: u32) -> bool {
    let base = this.addr();
    let size = e.mem.u32(base + LOCKED_SIZE);
    let block = e.mem.u32(base + STAGING_BLOCK);
    let locked = e.mem.u32(base + LOCKED_ADDRESS);
    e.call(MEMCPY, &args![locked, block, size]);
    e.mem.set_u32(base + LOCKED_ADDRESS, 0);
    e.mem.set_u32(base + LOCKED_SIZE, 0);
    e.call(LEAVE_CRITICAL_SECTION, &args![base + LOCK_SECTION]);
    e.vcall(buffer, VERTEX_BUFFER_UNLOCK, &[]).i32() >= 0
}

/// The cap on the texture-set count (`011bf558`).
const TEXTURE_SET_LIMIT_011BF558: u32 = 0x011b_f558;

// Translated from 00e8c120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: lays out a vertex format for a geometry data (`thiscall` on the
/// object at the renderer's +0x8BC, which it does not read; nine words: the data, a partition
/// or 0, and seven out words: the flags, the vertex size and the offsets of the position,
/// the blend weights, the normal, the colour and the texture coordinates). The flags and the
/// size start at 0 and the position is at offset 0; the size grows by the 12 position bytes
/// and `weights` gets the new size (12); with a partition
/// whose 16-bit +0x20 is non-zero the flags become 0xA and 12 more bytes follow, else flag 2
/// is set. Data with a word at +0x24 get flag 0x10 and 12 bytes at `normal`; with a word at
/// +0x28 flag 0x40 and 4 bytes at `colour`. The texture-coordinate count is 1 when bit 0 of
/// the data's 16-bit +0xC is clear, else that bit (1) capped by the word at `011bf558`
/// (nothing more when the cap makes it 0); it goes to the flags shifted by 8, its offset to
/// `coordinates` and 8 bytes per set to the size.
pub fn fn_00e8c120(
    e: &mut Engine,
    _this: Ptr,
    data: u32,
    partition: u32,
    flags: Ptr,
    size: Ptr,
    position: Ptr,
    weights: Ptr,
    normal: Ptr,
    colour: Ptr,
    coordinates: Ptr,
) {
    let (flags, size) = (flags.addr(), size.addr());
    e.mem.set_u32(flags, 0);
    e.mem.set_u32(size, 0);
    let weight_count = if partition != 0 {
        e.mem.u16(partition.wrapping_add(0x20))
    } else {
        0
    };
    e.mem.set_u32(position.addr(), 0);
    let grow = |e: &mut Engine, bytes: u32| {
        let value = e.mem.u32(size).wrapping_add(bytes);
        e.mem.set_u32(size, value);
    };
    let set_flag = |e: &mut Engine, bit: u32| {
        let value = e.mem.u32(flags) | bit;
        e.mem.set_u32(flags, value);
    };
    grow(e, 0xc);
    let offset = e.mem.u32(size);
    e.mem.set_u32(weights.addr(), offset);
    if weight_count == 0 {
        set_flag(e, 2);
    } else {
        e.mem.set_u32(flags, 0xa);
        grow(e, 0xc);
    }
    if e.mem.u32(data.wrapping_add(0x24)) != 0 {
        set_flag(e, 0x10);
        let offset = e.mem.u32(size);
        e.mem.set_u32(normal.addr(), offset);
        grow(e, 0xc);
    }
    if e.mem.u32(data.wrapping_add(0x28)) != 0 {
        set_flag(e, 0x40);
        let offset = e.mem.u32(size);
        e.mem.set_u32(colour.addr(), offset);
        grow(e, 4);
    }
    let mut count = u32::from(e.mem.u16(data.wrapping_add(0xc))) & 1;
    if count == 0 {
        count = 1;
    } else {
        let limit = e.mem.u32(TEXTURE_SET_LIMIT_011BF558);
        if count > limit {
            count = limit;
        }
        if count == 0 {
            return;
        }
    }
    set_flag(e, count << 8);
    let offset = e.mem.u32(size);
    e.mem.set_u32(coordinates.addr(), offset);
    grow(e, count.wrapping_mul(8));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_c0d0, fn_00e8c0d0(Ptr, u32) -> bool),
        entry!(
            0x00e8_c120,
            fn_00e8c120(Ptr, u32, u32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const UNLOCK_FN: u32 = 0x0040_4000;

    #[test]
    fn unlock_copies_back_leaves_and_unlocks() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, a| a[0].into_ret());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(UNLOCK_FN, |_, _| Ret::default());
        let mut slots = vec![0u32; 13];
        slots[12] = UNLOCK_FN;
        e.put_vtable(0x0090_6000, &slots);
        let buffer = e.mem.alloc(4);
        e.mem.set_u32(buffer, 0x0090_6000);
        let this: Ptr = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_u32(this.addr() + STAGING_BLOCK, 0x1111);
        e.mem.set_u32(this.addr() + LOCKED_ADDRESS, 0x2222);
        e.mem.set_u32(this.addr() + LOCKED_SIZE, 0x30);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e8_c0d0, &args![this, buffer]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MEMCPY), vec![vec![0x2222, 0x1111, 0x30]]);
        assert_eq!(
            calls_to(&log, LEAVE_CRITICAL_SECTION),
            vec![vec![this.addr() + LOCK_SECTION]]
        );
        assert_eq!(calls_to(&log, UNLOCK_FN), vec![vec![buffer]]);
        assert_eq!(e.mem.u32(this.addr() + LOCKED_ADDRESS), 0);
        assert_eq!(e.mem.u32(this.addr() + LOCKED_SIZE), 0);
        // A failed unlock answers false.
        e.register(UNLOCK_FN, |_, _| 0x8876_086cu32.into_ret());
        assert!(!e.call(0x00e8_c0d0, &args![this, buffer]).bool());
    }
}

#[cfg(test)]
mod n011_tests {
    use super::*;

    /// Runs the layout and answers the seven out words.
    fn layout(e: &mut Engine, data: u32, partition: u32) -> [u32; 7] {
        let out = e.mem.alloc(0x1c);
        for i in 0..7u32 {
            e.mem.set_u32(out + 4 * i, 0xeeee_eeee);
        }
        let mut words = vec![0x0e00_0000, data, partition];
        words.extend((0..7u32).map(|i| out + 4 * i));
        e.call(0x00e8_c120, &words);
        let mut result = [0u32; 7];
        for (i, word) in result.iter_mut().enumerate() {
            *word = e.mem.u32(out + 4 * i as u32);
        }
        result
    }

    #[test]
    fn fn_00e8c120_lays_out_every_component() {
        let mut e = Engine::new();
        e.map(TEXTURE_SET_LIMIT_011BF558, 4);
        e.set_global(TEXTURE_SET_LIMIT_011BF558, 4u32);
        let data = e.mem.alloc(0x30);
        e.mem.set_u32(data + 0x24, 1);
        e.mem.set_u32(data + 0x28, 1);
        e.mem.set_u16(data + 0xc, 3);
        let partition = e.mem.alloc(0x24);
        e.mem.set_u16(partition + 0x20, 2);
        // flags, size, position, weights, normal, colour, coordinates.
        assert_eq!(
            layout(&mut e, data, partition),
            [0x15a, 0x30, 0, 0xc, 0x18, 0x24, 0x28]
        );
    }

    #[test]
    fn fn_00e8c120_without_parts_and_with_a_zero_cap() {
        let mut e = Engine::new();
        e.map(TEXTURE_SET_LIMIT_011BF558, 4);
        let data = e.mem.alloc(0x30);
        // Bit 0 clear: one texture set regardless of the cap.
        assert_eq!(
            layout(&mut e, data, 0),
            [0x102, 0x14, 0, 0xc, 0xeeee_eeee, 0xeeee_eeee, 0xc]
        );
        // Bit 0 set and a cap of 0: no texture set.
        e.mem.set_u16(data + 0xc, 1);
        assert_eq!(
            layout(&mut e, data, 0),
            [2, 0xc, 0, 0xc, 0xeeee_eeee, 0xeeee_eeee, 0xeeee_eeee]
        );
    }
}
