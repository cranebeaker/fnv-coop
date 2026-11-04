//! `(unplaced)/run_00a29580` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Loads the base part of the same object (`00a3da20`, `RET 4`: `this`,
/// stream).
const LOAD_BASE: u32 = 0x00a3_da20;
/// `NiColorA::LoadBinary` (engine map), `00a69800` (`this`, stream).
const COLOR_LOAD_BINARY: u32 = 0x00a6_9800;

/// Reads `size` bytes from the stream's input through the read function
/// pointer at +8 of the stream object at `NiStream` +0x24C (`cdecl`:
/// input, buffer, size, &count, 1) and adds its result to the input's
/// counter at +4. `count` is the exe's local, set to `size` first.
fn stream_read(e: &mut Engine, stream: Ptr, buffer: u32, size: u32) {
    // NiStream +0x24C: the stream's input object (read function at +8,
    // bytes-read counter at +4).
    let input = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(input + 8);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), size);
        let n = e.call(read, &args![input, buffer, size, count, 1u32]).u32();
        let total = e.mem.u32(input + 4).wrapping_add(n);
        e.mem.set_u32(input + 4, total);
    });
}

// Translated from 00a29580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads a 4-byte value and a flag byte from the stream (no Xbox PDB name):
/// four bytes into `this` +0, then one byte into the exe's argument slot,
/// and `this` +4 becomes whether that byte is nonzero.
pub fn fn_00a29580(e: &mut Engine, this: Ptr, stream: Ptr) {
    stream_read(e, stream, this.addr(), 4);
    e.with_stack(4, |e, flag| {
        e.mem.set_u32(flag.addr(), stream.addr());
        stream_read(e, stream, flag.addr(), 1);
        let set = e.mem.u8(flag.addr()) != 0;
        e.mem.set_u8(this.addr() + 4, set as u8);
    });
}

// Translated from 00a29640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads an object holding an `NiColorA` at +4 (no Xbox PDB name):
/// `00a3da20` on `this`, then `NiColorA::LoadBinary` (`00a69800`) on
/// `this` +4, both with the stream.
pub fn fn_00a29640(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LOAD_BASE, &args![this, stream]);
    e.call(COLOR_LOAD_BINARY, &args![this.addr() + 4, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a29580, fn_00a29580(Ptr, Ptr)),
        entry!(0x00a29640, fn_00a29640(Ptr, Ptr)),
        entry!(0x00a2_95f0, fn_00a295f0(Ptr, Ptr)),
        entry!(0x00a2_9660, fn_00a29660(Ptr, Ptr) -> u32),
        entry!(0x00a2_9690, fn_00a29690(u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const READ: u32 = 0x00de_ad00;

    /// A stream whose input object reads from `bytes` through a double at
    /// `READ`, recording each request.
    fn stream(e: &mut Engine, bytes: Vec<u8>) -> Ptr {
        let stream = Ptr::new(e.mem.alloc(0x250));
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream.addr() + 0x24c, input);
        e.mem.set_u32(input + 8, READ);
        e.mem.set_u32(input + 4, 7);
        let mut at = 0usize;
        e.register_double(READ, move |e, a| {
            assert_eq!(a[4], 1);
            assert_eq!(e.mem.u32(a[3]), a[2]);
            let n = a[2] as usize;
            e.mem.write(a[1], &bytes[at..at + n]);
            at += n;
            Ret {
                eax: n as u32,
                ..Ret::default()
            }
        });
        stream
    }

    #[test]
    fn load_reads_the_value_then_the_flag() {
        let mut e = Engine::new();
        let s = stream(&mut e, vec![0x78, 0x56, 0x34, 0x12, 0x02, 0, 0, 0, 0, 0]);
        let this: Ptr = Ptr::new(e.mem.alloc(8));
        e.call(0x00a2_9580, &args![this, s]);
        assert_eq!(e.mem.u32(this.addr()), 0x1234_5678);
        assert_eq!(e.mem.u8(this.addr() + 4), 1);
        let input = e.mem.u32(s.addr() + 0x24c);
        assert_eq!(e.mem.u32(input + 4), 7 + 5);
        e.call(0x00a2_9580, &args![this, s]);
        assert_eq!(e.mem.u32(this.addr()), 0);
        assert_eq!(e.mem.u8(this.addr() + 4), 0);
    }

    #[test]
    fn color_holder_loads_its_base_then_the_color() {
        let mut e = Engine::new();
        e.register(LOAD_BASE, |_, _| Ret::default());
        e.register(COLOR_LOAD_BINARY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a2_9640, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == LOAD_BASE || *a == COLOR_LOAD_BINARY)
            .cloned()
            .collect();
        assert_eq!(
            calls,
            vec![
                (LOAD_BASE, vec![0x1000, 0x2000]),
                (COLOR_LOAD_BINARY, vec![0x1004, 0x2000]),
            ]
        );
    }
}

/// Saves the base part of a key (`00a3da50`, cdecl: stream, key; the
/// engine map names it `NiStreamSaveBinary<float>`).
const SAVE_BASE: u32 = 0x00a3_da50;
/// `NiColorA::SaveBinary` (engine map), `00a698a0` (thiscall: the colour,
/// the stream).
const COLOR_SAVE_BINARY: u32 = 0x00a6_98a0;
/// The 16 bytes `00a29690` copies out (an `NiColorA`, four floats).
const DEFAULT_COLOR_011A9BD0: u32 = 0x011a_9bd0;

// Translated from 00a295f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves a key holding a flag byte at +4 (cdecl; no Xbox PDB name; called
/// for each 8-byte key by `00a2cc60`): `00a3da50(stream, key)`, then one
/// byte, whether the key's byte +4 is set, through the write function at
/// +0x0C of the stream's output object (`NiStream` +0x250; cdecl: output,
/// the byte, 1, &count, 1, with `count` 1), whose result is added to the
/// output's counter at +4.
pub fn fn_00a295f0(e: &mut Engine, stream: Ptr, key: Ptr) {
    e.call(SAVE_BASE, &args![stream, key]);
    let set = e.mem.u8(key.addr() + 4) != 0;
    // NiStream +0x250: the stream's output object (write function at +0x0C,
    // bytes-written counter at +4).
    let output = e.mem.u32(stream.addr() + 0x250);
    // The byte goes into the low byte of the key argument's slot and the
    // count into the stream argument's slot.
    e.with_stack(8, |e, slots| {
        let flag = slots.addr();
        let count = slots.addr() + 4;
        e.mem.set_u32(flag, key.addr());
        e.mem.set_u8(flag, set as u8);
        e.mem.set_u32(count, 1);
        let write = e.mem.u32(output + 0xc);
        let n = e.call(write, &[output, flag, 1, count, 1]).u32();
        let total = e.mem.u32(output + 4).wrapping_add(n);
        e.mem.set_u32(output + 4, total);
    });
}

// Translated from 00a29660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves a key holding an `NiColorA` at +4 (cdecl; no Xbox PDB name; called
/// by `00a2cfd0`): `00a3da50(stream, key)`, then `NiColorA::SaveBinary`
/// (`00a698a0`) on the key's +4 with the stream, whose EAX it passes on.
pub fn fn_00a29660(e: &mut Engine, stream: Ptr, key: Ptr) -> u32 {
    e.call(SAVE_BASE, &args![stream, key]);
    e.call(COLOR_SAVE_BINARY, &args![key.addr() + 4, stream])
        .u32()
}

// Translated from 00a29690 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A colour-key interpolation entry of the table at `011f3cd8` (stored by
/// `00a299f0`, called by `NiColorKey::GenInterp` with the time, two keys and
/// the result; Ghidra made no function of it): ignores the first three
/// words and copies the 16 bytes at `011a9bd0` into `result`.
pub fn fn_00a29690(e: &mut Engine, _unused_0: u32, _unused_1: u32, _unused_2: u32, result: u32) {
    for offset in [0, 4, 8, 0xc] {
        let word = e.mem.u32(DEFAULT_COLOR_011A9BD0 + offset);
        e.mem.set_u32(result + offset, word);
    }
}

#[cfg(test)]
mod n079_tests {
    use super::*;

    const WRITE: u32 = 0x00de_ae00;

    /// A stream whose output object records each byte written through a
    /// double at `WRITE`.
    fn stream(e: &mut Engine) -> (Ptr, std::rc::Rc<std::cell::RefCell<Vec<u8>>>) {
        let stream = Ptr::new(e.mem.alloc(0x254));
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(stream.addr() + 0x250, output);
        e.mem.set_u32(output + 0xc, WRITE);
        e.mem.set_u32(output + 4, 7);
        let written = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let record = written.clone();
        e.register_double(WRITE, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (1, 1, 1));
            record.borrow_mut().push(e.mem.u8(a[1]));
            1u32.into_ret()
        });
        (stream, written)
    }

    #[test]
    fn flag_key_saves_its_base_then_the_flag() {
        let mut e = Engine::new();
        e.register(SAVE_BASE, |_, _| Ret::default());
        let (s, written) = stream(&mut e);
        let key: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u8(key.addr() + 4, 5);
        e.call_log = Some(vec![]);
        e.call(0x00a2_95f0, &args![s, key]);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log.iter().filter(|(a, _)| *a == SAVE_BASE).collect();
        assert_eq!(base, vec![&(SAVE_BASE, vec![s.addr(), key.addr()])]);
        e.mem.set_u8(key.addr() + 4, 0);
        e.call(0x00a2_95f0, &args![s, key]);
        assert_eq!(*written.borrow(), vec![1, 0]);
        let output = e.mem.u32(s.addr() + 0x250);
        assert_eq!(e.mem.u32(output + 4), 9);
    }

    #[test]
    fn color_key_saves_its_base_then_the_color() {
        let mut e = Engine::new();
        e.register(SAVE_BASE, |_, _| Ret::default());
        e.register(COLOR_SAVE_BINARY, |_, _| 0x77u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a2_9660, &args![0x2000u32, 0x1000u32]).u32(),
            0x77
        );
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == SAVE_BASE || *a == COLOR_SAVE_BINARY)
            .cloned()
            .collect();
        assert_eq!(
            calls,
            vec![
                (SAVE_BASE, vec![0x2000, 0x1000]),
                (COLOR_SAVE_BINARY, vec![0x1004, 0x2000]),
            ]
        );
    }

    #[test]
    fn color_entry_copies_the_constant() {
        let mut e = Engine::new();
        e.map(0x011a_9000, 0x1000);
        for (i, v) in [1.0f32, 0.5, 0.25, 2.0].into_iter().enumerate() {
            e.mem
                .set_u32(DEFAULT_COLOR_011A9BD0 + 4 * i as u32, v.to_bits());
        }
        let result = e.mem.alloc(0x14);
        e.call(0x00a2_9690, &[0x3f80_0000, 1, 2, result]);
        assert_eq!(e.mem.f32(result + 8), 0.25);
        assert_eq!(
            e.mem.bytes(result, 16),
            e.mem.bytes(DEFAULT_COLOR_011A9BD0, 16)
        );
        assert_eq!(e.mem.u32(result + 16), 0);
    }
}
