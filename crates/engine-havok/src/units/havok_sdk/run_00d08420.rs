//! `Havok SDK/run_00d08420` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00d9a040` (`this` = the object it builds, no arguments): writes the vtable words `010d4094`
/// at `+0` and `010d4088` at `+4`, zero at `+8` to `+0x18` and `1.0` at `+0x1c`.
const BUILD_HOLDER: u32 = 0x00d9_a040;
/// `00d98ca0` (cdecl, five words: an output byte, the collidable of `this`, two words, the
/// request record); named by neither the map nor Ghidra.
const CAST_AT_COLLIDABLE: u32 = 0x00d9_8ca0;
/// `00d1bd00` (cdecl, five words: the word at `this + 0xc`, two words, the holder part, a
/// float); named by neither the map nor Ghidra.
const CAST_IN_WORLD: u32 = 0x00d1_bd00;
/// The vtable word of the request record (`010ccfb8`).
const REQUEST_VTABLE: u32 = 0x010c_cfb8;

// Translated from 00d08420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs a query on the object `this` (the class is unconfirmed) with stack words `(a0, a1, a2)`:
/// two records are built in the exe's stack frame (offsets from the frame start `F`, as the
/// disassembly stores them):
///
/// - a holder at `F + 0x24` made by `00d9a040`, then `+8` = `a0`, `+0x18` = `a2`, `+0x0c` and `+0x10`
///   = `a1 + 0x14` and `a1 + 0x10` (both 0 when `a1` is 0), `+0x14` = `F + 0x44`, `+0x30` = `1.0`,
///   `+0x34` and `+0x40` = `-1` and `+0x60` and `+0x70` = 0;
/// - when the word at `this + 0x10` is not 0, a request record at `F + 0x0c`: the vtable word
///   `010ccfb8`, `a0` at `+4`, `a1 + 0x14` and `a1 + 0x10` at `+8` and `+0xc` (0 when `a1` is
///   0), `F + 0xa4` at `+0x10`, `a2` at `+0x14`, and at `F + 0xa4`: `1.0` at `+0x10`, `-1` at `+0x14`
///   and `+0x20`, 0 at `+0x40` and `+0x50`; then `00d98ca0(F + 0x0b, word at this + 0x10, a0,
///   a0 + 0x10, F + 0x0c)` is called;
/// - when the byte at `this + 0x24` is not 0, `00d1bd00(word at this + 0xc, a0, a0 + 0x10, F +
///   0x28, 1.0)` is called (`F + 0x28` is the holder's second table).
///
/// The 16-byte vectors at `F + 0x44` and `F + 0xa4` and the byte at `F + 0x0b` are left as the
/// exe leaves them (uninitialised stack; here zero) for the callees to fill.
pub fn fn_00d08420(e: &mut Engine, this: Ptr, a0: u32, a1: u32, a2: u32) {
    let base = this.addr();
    e.with_stack(0x104, |e, frame| {
        let f = frame.addr();
        e.call(BUILD_HOLDER, &args![f + 0x24]);
        e.mem.set_u32(f + 0x2c, a0);
        e.mem.set_u32(f + 0x3c, a2);
        let (end, start) = if a1 == 0 {
            (0, 0)
        } else {
            (a1.wrapping_add(0x14), a1.wrapping_add(0x10))
        };
        e.mem.set_u32(f + 0x30, end);
        e.mem.set_u32(f + 0x34, start);
        e.mem.set_u32(f + 0x58, 0xffff_ffff);
        e.mem.set_u32(f + 0x54, 0x3f80_0000);
        e.mem.set_u32(f + 0x64, 0xffff_ffff);
        e.mem.set_u32(f + 0x38, f + 0x44);
        let collidable = e.mem.u32(base + 0x10);
        e.mem.set_u32(f + 0x84, 0);
        e.mem.set_u32(f + 0x94, 0);
        if collidable != 0 {
            e.mem.set_u32(f + 0xc, REQUEST_VTABLE);
            e.mem.set_u32(f + 0x10, a0);
            e.mem.set_u32(f + 0x20, a2);
            e.mem.set_u32(f + 0x14, end);
            e.mem.set_u32(f + 0x18, start);
            e.mem.set_u32(f + 0xb4, 0x3f80_0000);
            e.mem.set_u32(f + 0xb8, 0xffff_ffff);
            e.mem.set_u32(f + 0xc4, 0xffff_ffff);
            e.mem.set_u32(f + 0x1c, f + 0xa4);
            e.mem.set_u32(f + 0xe4, 0);
            e.mem.set_u32(f + 0xf4, 0);
            e.call(
                CAST_AT_COLLIDABLE,
                &args![f + 0xb, collidable, a0, a0.wrapping_add(0x10), f + 0xc],
            );
        }
        if e.mem.u8(base + 0x24) != 0 {
            let world = e.mem.u32(base + 0xc);
            e.call(
                CAST_IN_WORLD,
                &args![world, a0, a0.wrapping_add(0x10), f + 0x28, 1.0f32],
            );
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d0_8420, fn_00d08420(Ptr, u32, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What the callees saw: the call's words and the words of the records they were given.
    type Seen = Rc<RefCell<Vec<(&'static str, Vec<u32>, Vec<u32>)>>>;

    fn words(e: &Engine, at: u32, offsets: &[u32]) -> Vec<u32> {
        offsets
            .iter()
            .map(|offset| e.mem.u32(at + offset))
            .collect()
    }

    /// Doubles of the three helpers recording the call words and the record contents the exe
    /// builds for them.
    fn rig() -> (Engine, Seen) {
        let mut e = Engine::new();
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(BUILD_HOLDER, move |_, a| {
            sink.borrow_mut().push(("holder", a.to_vec(), vec![]));
            Ret::default()
        });
        let sink = Rc::clone(&seen);
        e.register_double(CAST_AT_COLLIDABLE, move |e, a| {
            let request = a[4];
            let vectors = e.mem.u32(request + 0x10);
            let mut record = words(e, request, &[0, 4, 8, 0xc, 0x14]);
            record.push(vectors - request);
            record.extend(words(e, vectors, &[0x10, 0x14, 0x20, 0x40, 0x50]));
            sink.borrow_mut().push(("cast", a.to_vec(), record));
            Ret::default()
        });
        let sink = Rc::clone(&seen);
        e.register_double(CAST_IN_WORLD, move |e, a| {
            let table = a[3];
            // The holder starts 4 bytes before the table; its fields:
            let mut record = words(e, table, &[4, 8, 0xc, 0x14, 0x2c, 0x30, 0x3c, 0x5c, 0x6c]);
            let vectors = e.mem.u32(table + 0x10);
            record.push(vectors - table);
            sink.borrow_mut().push(("world", a.to_vec(), record));
            Ret::default()
        });
        (e, seen)
    }

    fn object(e: &mut Engine, collidable: u32, flag: u8) -> u32 {
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, 0x5555);
        e.mem.set_u32(this + 0x10, collidable);
        e.mem.set_u8(this + 0x24, flag);
        this
    }

    /// With a collidable and the flag set all three helpers run, with the records the exe builds
    /// (checked from inside the doubles).
    #[test]
    fn query_builds_the_records_and_runs_both_casts() {
        let (mut e, seen) = rig();
        let this = object(&mut e, 0x7777, 1);
        e.call(0x00d0_8420, &args![this, 0x1000u32, 0x2000u32, 0x3000u32]);
        let seen = seen.borrow();
        let names: Vec<&str> = seen.iter().map(|(name, _, _)| *name).collect();
        assert_eq!(names, vec!["holder", "cast", "world"]);
        // The request record: vtable, a0, a1 + 0x14, a1 + 0x10, a2, the distance of the
        // vector area from it (0xa4 - 0xc), then 1.0, -1, -1, 0, 0 in the vector area.
        let (_, call, record) = &seen[1];
        assert_eq!(call[1..4], [0x7777, 0x1000, 0x1010]);
        assert_eq!(call[0] + 1, call[4]);
        assert_eq!(
            *record,
            vec![
                REQUEST_VTABLE,
                0x1000,
                0x2014,
                0x2010,
                0x3000,
                0xa4 - 0xc,
                0x3f80_0000,
                0xffff_ffff,
                0xffff_ffff,
                0,
                0
            ]
        );
        // The world call: the word at this + 0xc, a0, a0 + 0x10, the holder's second table and
        // 1.0; the holder carries a0, a1 + 0x14, a1 + 0x10, a2, 1.0, -1, -1, 0, 0 and its vector
        // area pointer 0x44 - 0x28 beyond the table.
        let (_, call, record) = &seen[2];
        assert_eq!(call[0..3], [0x5555, 0x1000, 0x1010]);
        assert_eq!(call[4], 0x3f80_0000);
        assert_eq!(
            *record,
            vec![
                0x1000,
                0x2014,
                0x2010,
                0x3000,
                0x3f80_0000,
                0xffff_ffff,
                0xffff_ffff,
                0,
                0,
                0x44 - 0x28
            ]
        );
        // The holder was built at 0x24 before the table (0x28).
        assert_eq!(seen[0].1[0] + 4, call[3]);
    }

    /// A null `a1` zeroes the two range words; without a collidable there is no first cast and
    /// with a zero flag no second.
    #[test]
    fn missing_parts_skip_their_calls() {
        let (mut e, seen) = rig();
        let this = object(&mut e, 0, 0);
        e.call(0x00d0_8420, &args![this, 0x1000u32, 0u32, 0x3000u32]);
        assert_eq!(seen.borrow().len(), 1);
        seen.borrow_mut().clear();
        let this = object(&mut e, 0x7777, 0);
        e.call(0x00d0_8420, &args![this, 0x1000u32, 0u32, 0x3000u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[1].2[2..5], [0u32, 0, 0x3000]);
    }
}
