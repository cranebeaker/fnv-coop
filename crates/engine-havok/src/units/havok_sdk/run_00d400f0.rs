//! `Havok SDK/run_00d400f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d400f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `record`): fills a record of function words: `+0` = `0x00d3fdf0`,
/// `+0x2c` = `0x00d3fe90`, `+0x28` = 0, `+8` = `0x00d3fe10`, `+0xc` = `0x00d3fd60`, `+0x10` =
/// `0x00d3fda0`, `+0x14` = `0x00d3fde0`, `+0x18` = 0, `+4` = `0x00d3fe70`, and the byte at `+0x30`
/// = 0.
pub fn fn_00d400f0(e: &mut Engine, record: Ptr) {
    let r = record.addr();
    e.mem.set_u32(r, 0x00d3_fdf0);
    e.mem.set_u32(r + 0x2c, 0x00d3_fe90);
    e.mem.set_u32(r + 0x28, 0);
    e.mem.set_u32(r + 8, 0x00d3_fe10);
    e.mem.set_u32(r + 0xc, 0x00d3_fd60);
    e.mem.set_u32(r + 0x10, 0x00d3_fda0);
    e.mem.set_u32(r + 0x14, 0x00d3_fde0);
    e.mem.set_u32(r + 0x18, 0);
    e.mem.set_u32(r + 4, 0x00d3_fe70);
    e.mem.set_u8(r + 0x30, 0);
}

// Translated from 00d40130 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `queue`; descriptions, not confirmed): builds a 0x40-byte record
/// on the stack (zero bytes at `+0x30` .. `+0x32` and zero words at `+0x18` .. `+0x28`, then
/// `00d400f0`; the rest uninitialised in the exe, zero here) and registers it with `00cfbd60` on
/// `queue` with `(record, 4, 4)`.
pub fn fn_00d40130(e: &mut Engine, queue: Ptr) {
    e.with_stack(0x40, |e, record| {
        let r = record.addr();
        e.mem.set_u8(r + 0x30, 0);
        e.mem.set_u8(r + 0x31, 0);
        e.mem.set_u8(r + 0x32, 0);
        for offset in [0x18u32, 0x1c, 0x20, 0x24, 0x28] {
            e.mem.set_u32(r + offset, 0);
        }
        fn_00d400f0(e, record);
        e.call(0x00cf_bd60, &args![queue, record, 4u32, 4u32]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_00f0, fn_00d400f0(Ptr)),
        entry!(0x00d4_0130, fn_00d40130(Ptr)),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    const EXPECTED: [u32; 13] = [
        0x00d3_fdf0,
        0x00d3_fe70,
        0x00d3_fe10,
        0x00d3_fd60,
        0x00d3_fda0,
        0x00d3_fde0,
        0,
        0,
        0,
        0,
        0,
        0x00d3_fe90,
        0,
    ];

    #[test]
    fn fn_00d400f0_fills_the_record() {
        let mut e = Engine::new();
        let record = e.mem.alloc(0x40);
        for word in 0..0x10 {
            e.mem.set_u32(record + word * 4, 0xffff_ffff);
        }
        e.call(0x00d4_00f0, &args![record]);
        // Words +0x1c, +0x20 and +0x24 are not written by this function.
        let expected = [
            0x00d3_fdf0,
            0x00d3_fe70,
            0x00d3_fe10,
            0x00d3_fd60,
            0x00d3_fda0,
            0x00d3_fde0,
            0,
            0xffff_ffff,
            0xffff_ffff,
            0xffff_ffff,
            0,
            0x00d3_fe90,
        ];
        for (word, value) in expected.iter().enumerate() {
            assert_eq!(e.mem.u32(record + word as u32 * 4), *value, "word {word}");
        }
        // The byte at +0x30 is cleared; the rest of that word is not touched.
        assert_eq!(e.mem.u32(record + 0x30), 0xffff_ff00);
    }

    #[test]
    fn fn_00d40130_registers_the_record() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<u32>::new()));
        let log = std::rc::Rc::clone(&seen);
        e.register_double(0x00cf_bd60, move |e, a| {
            let mut words = a.to_vec();
            for word in 0..12 {
                words.push(e.mem.u32(a[1] + word * 4));
            }
            words.push(e.mem.u32(a[1] + 0x30));
            *log.borrow_mut() = words;
            Ret::default()
        });
        let queue = e.mem.alloc(0x10);
        e.call(0x00d4_0130, &args![queue]);
        let seen = seen.borrow();
        assert_eq!(seen[0], queue);
        assert_eq!(seen[2..4], [4, 4]);
        assert_eq!(seen[4..], EXPECTED);
    }
}
