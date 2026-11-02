//! `NiXenonRenderer/run_00e93f50` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The globals `00e93f50` keeps: the renderer (`0126fa2c`) and its device (`0126fa28`,
/// counted with the device's `AddRef`/`Release`, slots 4 and 8).
const RENDERER_0126FA2C: u32 = 0x0126_fa2c;
const DEVICE_0126FA28: u32 = 0x0126_fa28;

// Translated from 00e93f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e93f50`), `cdecl`, the renderer: keeps it at
/// `0126fa2c`. With a renderer, the device held at `0126fa28` is released (slot 8) and
/// replaced by the renderer's device (+0x288; counted, slot 4, when not null); without one
/// the held device is released and the word cleared.
pub fn fn_00e93f50(e: &mut Engine, renderer: u32) {
    e.mem.set_u32(RENDERER_0126FA2C, renderer);
    let held = e.mem.u32(DEVICE_0126FA28);
    if renderer != 0 {
        let device = e.mem.u32(renderer.wrapping_add(0x288));
        if held != 0 {
            e.vcall(held, 8, &args![]);
        }
        e.mem.set_u32(DEVICE_0126FA28, device);
        if device != 0 {
            e.vcall(device, 4, &args![]);
        }
    } else {
        if held != 0 {
            e.vcall(held, 8, &args![]);
        }
        e.mem.set_u32(DEVICE_0126FA28, 0);
    }
}

/// The three floats `00e93fb0` copies into +0x10 to +0x18 and +0 to +8 (`011f4998`,
/// `011f499c`, `011f49a0`).
const FLOATS_011F4998: [u32; 3] = [0x011f_4998, 0x011f_499c, 0x011f_49a0];

/// A float moved by `FLD dword` / `FSTP dword` (a signalling NaN comes out quiet).
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

// Translated from 00e93fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e93f50`), `thiscall` with one word (a source record).
/// When the record's +0x44 equals `this`'s +0x44 answers 0 (nothing copied). Otherwise copies
/// the stamp +0x44, then the three floats at the record's +0x34 (or, when that word is null,
/// at the record's +0x28) into +0x30 to +0x38 and the record's float +0x3c into +0x3c, +0x1c,
/// +0xc and +0x2c; the floats at `011f4998` to `011f49a0` into +0x10 to +0x18 and again into
/// +0 to +8; the record's floats +0x1c to +0x24 into +0x20 to +0x28 and its +0x38 into +0x40.
/// Answers 1. Each float goes through the x87 stack (signalling NaNs come out quiet).
pub fn fn_00e93fb0(e: &mut Engine, this: Ptr, record: u32) -> bool {
    let base = this.addr();
    let stamp = e.mem.u32(record.wrapping_add(0x44));
    if e.mem.u32(base.wrapping_add(0x44)) == stamp {
        return false;
    }
    e.mem.set_u32(base.wrapping_add(0x44), stamp);
    let mut source = e.mem.u32(record.wrapping_add(0x34));
    if source == 0 {
        source = record.wrapping_add(0x28);
    }
    let fourth = x87_quiet_bits(e.mem.u32(record.wrapping_add(0x3c)));
    for index in 0..3u32 {
        copy_float(
            e,
            source.wrapping_add(4 * index),
            base.wrapping_add(0x30 + 4 * index),
        );
    }
    e.mem.set_u32(base.wrapping_add(0x3c), fourth);
    for (index, global) in FLOATS_011F4998.into_iter().enumerate() {
        copy_float(e, global, base.wrapping_add(0x10 + 4 * index as u32));
    }
    e.mem.set_u32(base.wrapping_add(0x1c), fourth);
    for (index, global) in FLOATS_011F4998.into_iter().enumerate() {
        copy_float(e, global, base.wrapping_add(4 * index as u32));
    }
    e.mem.set_u32(base.wrapping_add(0xc), fourth);
    for index in 0..3u32 {
        copy_float(
            e,
            record.wrapping_add(0x1c + 4 * index),
            base.wrapping_add(0x20 + 4 * index),
        );
    }
    e.mem.set_u32(base.wrapping_add(0x2c), fourth);
    copy_float(e, record.wrapping_add(0x38), base.wrapping_add(0x40));
    true
}

/// The Ni allocation `00aa13e0` (`cdecl`, size) and free `00aa1460` (`cdecl`: block, size), and
/// the global record `011f4a64` `00e94130` and `00e940a0` use.
const NI_ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;
const RECORD_011F4A64: u32 = 0x011f_4a64;

/// The 0x48-byte copy a record holds at +0x48 freed (when there is one): the record's word
/// cleared, `this`'s +0 cleared when it pointed at the copy, then the copy freed.
fn free_copy(e: &mut Engine, this: u32, record: u32) {
    if record == 0 {
        return;
    }
    let copy = e.mem.u32(record.wrapping_add(0x48));
    if copy == 0 {
        return;
    }
    e.mem.set_u32(record.wrapping_add(0x48), 0);
    if copy == e.mem.u32(this) {
        e.mem.set_u32(this, 0);
    }
    e.call(NI_FREE_00AA1460, &args![copy, 0x48u32]);
}

// Translated from 00e94070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e93f50`), `thiscall` with one word (a record): frees the
/// record's 0x48-byte copy (+0x48; nothing for a null record or copy), clearing the record's
/// word and `this`'s +0 when it pointed at it.
pub fn fn_00e94070(e: &mut Engine, this: Ptr, record: u32) {
    free_copy(e, this.addr(), record);
}

// Translated from 00e940a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e93f50`), `thiscall`, no stack words: `00e94070` inlined
/// on the global record `011f4a64`.
pub fn fn_00e940a0(e: &mut Engine, this: Ptr) {
    let record = e.mem.u32(RECORD_011F4A64);
    free_copy(e, this.addr(), record);
}

// Translated from 00e940d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e93f50`), `thiscall` with one word (a record). A record
/// without a copy (+0x48) gets one: 0x48 bytes (`00aa13e0`) whose stamp +0x44 is cleared and
/// which `00e93fb0` fills from the record (null when the allocation fails), and the byte +4
/// is set. Then `00e93fb0` updates the copy (set +4 when it copied) and the copy becomes +0
/// (setting +4) when +0 held another. The copy the code works on is the one it read or made in
/// ECX (a failed allocation leaves 0, which `00e93fb0` then reads through).
pub fn fn_00e940d0(e: &mut Engine, this: Ptr, record: u32) {
    let base = this.addr();
    let mut copy = e.mem.u32(record.wrapping_add(0x48));
    if copy == 0 {
        copy = e.call(NI_ALLOCATE_00AA13E0, &args![0x48u32]).u32();
        if copy != 0 {
            e.mem.set_u32(copy + 0x44, 0);
            fn_00e93fb0(e, Ptr::new(copy), record);
        }
        e.mem.set_u32(record.wrapping_add(0x48), copy);
        e.mem.set_u8(base.wrapping_add(4), 1);
    }
    if fn_00e93fb0(e, Ptr::new(copy), record) {
        e.mem.set_u8(base.wrapping_add(4), 1);
    }
    if copy != e.mem.u32(base) {
        e.mem.set_u8(base.wrapping_add(4), 1);
        e.mem.set_u32(base, copy);
    }
}

// Translated from 00e94130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e93f50`), `thiscall`, no stack words: clears +0, runs
/// `00e940d0` with the global record `011f4a64` and sets the byte +4.
pub fn fn_00e94130(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0);
    let record = e.mem.u32(RECORD_011F4A64);
    fn_00e940d0(e, this, record);
    e.mem.set_u8(this.addr().wrapping_add(4), 1);
}

// Translated from 00e94050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e93f50`), `thiscall` with the device: when the byte +4 is
/// set it is cleared and the device's slot 0xc4 (by the interface's order `SetMaterial`)
/// gets the word +0 (the record's address).
pub fn fn_00e94050(e: &mut Engine, this: Ptr, device: u32) {
    let base = this.addr();
    if e.mem.u8(base.wrapping_add(4)) == 0 {
        return;
    }
    e.mem.set_u8(base.wrapping_add(4), 0);
    let record = e.mem.u32(base);
    e.vcall(device, 0xc4, &args![record]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_4050, fn_00e94050(Ptr, u32)),
        entry!(0x00e9_3f50, fn_00e93f50(u32)),
        entry!(0x00e9_3fb0, fn_00e93fb0(Ptr, u32) -> bool),
        entry!(0x00e9_4070, fn_00e94070(Ptr, u32)),
        entry!(0x00e9_40a0, fn_00e940a0(Ptr)),
        entry!(0x00e9_40d0, fn_00e940d0(Ptr, u32)),
        entry!(0x00e9_4130, fn_00e94130(Ptr)),
    ]
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00e93f50_swaps_the_held_device() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.put_vtable(0x6000, &[0, 0x6104, 0x6108]);
        e.register(0x6104, |_, _| Ret::default());
        e.register(0x6108, |_, _| Ret::default());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x6000);
        let renderer = e.mem.alloc(0x300);
        e.mem.set_u32(renderer + 0x288, device);
        e.call_log = Some(vec![]);
        e.call(0x00e9_3f50, &args![renderer]);
        assert_eq!(e.mem.u32(RENDERER_0126FA2C), renderer);
        assert_eq!(e.mem.u32(DEVICE_0126FA28), device);
        e.call(0x00e9_3f50, &args![0u32]);
        assert_eq!(e.mem.u32(DEVICE_0126FA28), 0);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(0x6104, vec![device])));
        assert!(log.contains(&(0x6108, vec![device])));
    }
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    const SIGNALLING: u32 = 0x7f80_0001;
    const QUIET: u32 = 0x7fc0_0001;

    /// A record with distinct float bits at +0x1c to +0x44 and the three globals set.
    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        for (index, global) in FLOATS_011F4998.into_iter().enumerate() {
            e.mem.set_u32(global, 0x4000_0000 + index as u32);
        }
        let record = e.mem.alloc(0x4c);
        for offset in (0x1c..0x44).step_by(4) {
            e.mem.set_u32(record + offset, 0x3f80_0000 + offset);
        }
        e.mem.set_u32(record + 0x34, 0);
        e.mem.set_u32(record + 0x44, 7);
        (e, record)
    }

    #[test]
    fn fn_00e93fb0_copies_once_per_stamp() {
        let (mut e, record) = rig();
        e.mem.set_u32(record + 0x3c, SIGNALLING);
        let this = e.mem.alloc(0x48);
        assert!(e.call(0x00e9_3fb0, &args![this, record]).bool());
        assert_eq!(e.mem.u32(this + 0x44), 7);
        // +0x34 is null: the floats come from +0x28.
        for index in 0..3 {
            assert_eq!(e.mem.u32(this + 0x30 + 4 * index), 0x3f80_0028 + 4 * index);
            assert_eq!(e.mem.u32(this + 0x10 + 4 * index), 0x4000_0000 + index);
            assert_eq!(e.mem.u32(this + 4 * index), 0x4000_0000 + index);
            assert_eq!(e.mem.u32(this + 0x20 + 4 * index), 0x3f80_001c + 4 * index);
        }
        for offset in [0x3c, 0x1c, 0xc, 0x2c] {
            assert_eq!(e.mem.u32(this + offset), QUIET);
        }
        assert_eq!(e.mem.u32(this + 0x40), 0x3f80_0038);
        // The same stamp: nothing copied.
        e.mem.set_u32(this + 0x40, 0);
        assert!(!e.call(0x00e9_3fb0, &args![this, record]).bool());
        assert_eq!(e.mem.u32(this + 0x40), 0);
        // A source block at +0x34.
        let block = e.mem.alloc(0xc);
        e.mem.set_u32(block, 0x1111);
        e.mem.set_u32(record + 0x34, block);
        e.mem.set_u32(record + 0x44, 8);
        assert!(e.call(0x00e9_3fb0, &args![this, record]).bool());
        assert_eq!(e.mem.u32(this + 0x30), 0x1111);
    }

    #[test]
    fn fn_00e94070_and_00e940a0_free_the_copy() {
        let (mut e, record) = rig();
        e.register(NI_FREE_00AA1460, |_, _| Ret::default());
        let copy = e.mem.alloc(0x48);
        e.mem.set_u32(record + 0x48, copy);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, copy);
        e.call_log = Some(vec![]);
        e.call(0x00e9_4070, &args![this, record]);
        assert_eq!(e.mem.u32(record + 0x48), 0);
        assert_eq!(e.mem.u32(this), 0);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(NI_FREE_00AA1460, vec![copy, 0x48])));
        // Null record, and no copy: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00e9_4070, &args![this, 0u32]);
        e.call(0x00e9_4070, &args![this, record]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        // The global record; `this` keeps another copy.
        e.mem.set_u32(RECORD_011F4A64, record);
        e.mem.set_u32(record + 0x48, copy);
        e.mem.set_u32(this, 0x5555);
        e.call(0x00e9_40a0, &args![this]);
        assert_eq!(e.mem.u32(record + 0x48), 0);
        assert_eq!(e.mem.u32(this), 0x5555);
    }

    #[test]
    fn fn_00e940d0_makes_and_updates_the_copy() {
        let (mut e, record) = rig();
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.mem.alloc(8);
        e.call(0x00e9_40d0, &args![this, record]);
        let copy = e.mem.u32(record + 0x48);
        assert_ne!(copy, 0);
        assert_eq!(e.mem.u32(this), copy);
        assert_eq!(e.mem.u8(this + 4), 1);
        assert_eq!(e.mem.u32(copy + 0x44), 7);
        // Nothing new: +4 stays clear.
        e.mem.set_u8(this + 4, 0);
        e.call(0x00e9_40d0, &args![this, record]);
        assert_eq!(e.mem.u8(this + 4), 0);
        // A new stamp marks it.
        e.mem.set_u32(record + 0x44, 9);
        e.call(0x00e9_40d0, &args![this, record]);
        assert_eq!(e.mem.u8(this + 4), 1);
        // Another copy at +0: replaced and marked.
        e.mem.set_u8(this + 4, 0);
        e.mem.set_u32(this, 0x7777);
        e.call(0x00e9_40d0, &args![this, record]);
        assert_eq!(e.mem.u32(this), copy);
        assert_eq!(e.mem.u8(this + 4), 1);
    }

    #[test]
    fn fn_00e94130_uses_the_global_record() {
        let (mut e, record) = rig();
        e.mem.set_u32(RECORD_011F4A64, record);
        let copy = e.mem.alloc(0x48);
        e.mem.set_u32(copy + 0x44, 7);
        e.mem.set_u32(record + 0x48, copy);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, copy);
        e.call(0x00e9_4130, &args![this]);
        // +0 was cleared first, so the copy is stored again; +4 set at the end.
        assert_eq!(e.mem.u32(this), copy);
        assert_eq!(e.mem.u8(this + 4), 1);
    }
}

#[cfg(test)]
mod n011_tests {
    use super::*;

    #[test]
    fn fn_00e94050_sends_the_record_once() {
        let mut e = Engine::new();
        e.put_vtable(0x6000, &[0; 0x31]);
        e.mem.set_u32(0x6000 + 0xc4, 0x60c4);
        e.register(0x60c4, |_, _| Ret::default());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x6000);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x0e0e_0000);
        e.mem.set_u8(this + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e9_4050, &args![this, device]);
        e.call(0x00e9_4050, &args![this, device]);
        let log = e.call_log.take().unwrap();
        let sent: Vec<_> = log.iter().filter(|c| c.0 == 0x60c4).collect();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].1, vec![device, 0x0e0e_0000]);
        assert_eq!(e.mem.u8(this + 4), 0);
    }
}
