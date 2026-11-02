//! `Havok SDK/run_00d33600` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00ec44d0` (cdecl `(destination, source, size)`): the copy of 0x1008 bytes both functions end
/// with when the target record has a buffer pointer.
const COPY_BYTES: u32 = 0x00ec_44d0;

// Translated from 00d33600 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 0x10`: four stack words, only `target` and `notifier` are read;
/// descriptions, not confirmed): when the float at `this + 0x1030` differs from the one at
/// `target + 0x3030` (a NaN counts as different: `FUCOMPP` unordered), calls slot `0x20` of the
/// object `notifier` with `(*(target + 4), target + 0x3050)` and `00d33330` on `target + 0x3010`
/// with `this + 0x1010`. Then copies the word at `this` to `target + 0`, and, when the word at
/// `target + 0x3080` is not null, copies 0x1008 bytes from `this + 4` to it with `00ec44d0`.
pub fn fn_00d33600(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    target: Ptr,
    notifier: Ptr,
) {
    let me = this.addr();
    let record = target.addr();
    let theirs = x87_load32(e.mem.u32(record + 0x3030));
    let ours = x87_load32(e.mem.u32(me + 0x1030));
    if ours != theirs {
        let key = e.mem.u32(record + 4);
        e.vcall(notifier.addr(), 0x20, &args![key, record + 0x3050]);
        e.call(0x00d3_3330, &args![record + 0x3010, me + 0x1010]);
    }
    let head = e.mem.u32(me);
    e.mem.set_u32(record, head);
    let buffer = e.mem.u32(record + 0x3080);
    if buffer != 0 {
        e.call(COPY_BYTES, &args![buffer, me + 4, 0x1008u32]);
    }
}

// Translated from 00d33670 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, one stack word `source`; descriptions, not confirmed): copies a record
/// of `source` into `this`: the word at `+0`; the vectors at `+0x3010` and `+0x3020` to `+0x1010`
/// and `+0x1020`; the words at `+0x3030`, `+0x3034`, `+0x3040`..`+0x304c` to `+0x1030`, `+0x1034`,
/// `+0x1040`..`+0x104c`; and 0x30 bytes from `+0x3050` to `+0x1050` (the exe first stores the float
/// at `0104da48` to `+0x1030` and zero to `+0x1050` and `+0x1054`, which the copies overwrite).
/// When the word at `source + 0x3080` is not null, copies 0x1008 bytes from it to `this + 4` with
/// `00ec44d0`. Returns `this`.
pub fn fn_00d33670(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let me = this.addr();
    let from = source.addr();
    let head = e.mem.u32(from);
    let large = x87_quiet_bits(e.mem.u32(0x0104_da48));
    e.mem.set_u32(me + 0x1030, large);
    e.mem.set_u32(me, head);
    e.mem.set_u32(me + 0x1050, 0);
    e.mem.set_u32(me + 0x1054, 0);
    for (source_offset, our_offset) in [(0x3010u32, 0x1010u32), (0x3020, 0x1020)] {
        for lane in 0..4 {
            let value = e.mem.u32(from + source_offset + lane * 4);
            e.mem.set_u32(me + our_offset + lane * 4, value);
        }
    }
    for (source_offset, our_offset) in [
        (0x3030u32, 0x1030u32),
        (0x3034, 0x1034),
        (0x3040, 0x1040),
        (0x3044, 0x1044),
        (0x3048, 0x1048),
        (0x304c, 0x104c),
    ] {
        let value = e.mem.u32(from + source_offset);
        e.mem.set_u32(me + our_offset, value);
    }
    for word in 0..12 {
        let value = e.mem.u32(from + 0x3050 + word * 4);
        e.mem.set_u32(me + 0x1050 + word * 4, value);
    }
    let buffer = e.mem.u32(from + 0x3080);
    if buffer != 0 {
        e.call(COPY_BYTES, &args![me + 4, buffer, 0x1008u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d3_3600, fn_00d33600(Ptr, u32, u32, Ptr, Ptr)),
        entry!(0x00d3_3670, fn_00d33670(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d33600_notifies_only_when_the_floats_differ() {
        for (ours, theirs, notified) in [
            (1.0f32, 1.0f32, false),
            (1.0, 2.0, true),
            (f32::NAN, f32::NAN, true),
            (0.0, -0.0, false),
        ] {
            let mut e = Engine::new();
            let this = e.mem.alloc(0x1100);
            let target = e.mem.alloc(0x3100);
            e.mem.set_u32(this, 0xabcd);
            e.mem.set_f32(this + 0x1030, ours);
            e.mem.set_f32(target + 0x3030, theirs);
            e.mem.set_u32(target + 4, 0x4444);
            let buffer = e.mem.alloc(0x1010);
            e.mem.set_u32(target + 0x3080, buffer);
            let vtable = e.mem.alloc(0x40);
            e.mem.set_u32(vtable + 0x20, 0x0900_0091);
            let notifier = e.mem.alloc(0x10);
            e.mem.set_u32(notifier, vtable);
            for address in [0x0900_0091, 0x00d3_3330, COPY_BYTES] {
                e.register(address, |_, _| Ret::default());
            }
            e.call_log = Some(vec![]);
            e.call(0x00d3_3600, &args![this, 1u32, 2u32, target, notifier]);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![];
            if notified {
                expected.push((0x0900_0091, vec![notifier, 0x4444, target + 0x3050]));
                expected.push((0x00d3_3330, vec![target + 0x3010, this + 0x1010]));
            }
            expected.push((COPY_BYTES, vec![buffer, this + 4, 0x1008]));
            assert_eq!(log[1..], expected[..]);
            assert_eq!(e.mem.u32(target), 0xabcd);
        }
    }

    #[test]
    fn fn_00d33600_skips_the_copy_without_a_buffer() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1100);
        let target = e.mem.alloc(0x3100);
        e.mem.set_u32(this, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x00d3_3600, &args![this, 0u32, 0u32, target, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u32(target), 0x77);
    }

    #[test]
    fn fn_00d33670_copies_the_record() {
        let mut e = Engine::new();
        e.map(0x0104_d000, 0x1000);
        e.mem.set_u32(0x0104_da48, 0x7f7f_ffee);
        let source = e.mem.alloc(0x3100);
        let this = e.mem.alloc(0x1100);
        e.mem.set_u32(source, 0x1234);
        for word in 0..32 {
            e.mem.set_u32(source + 0x3010 + word * 4, 0x9000 + word);
        }
        let buffer = e.mem.alloc(0x1010);
        e.mem.set_u32(source + 0x3080, buffer);
        e.register(COPY_BYTES, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let result = e.call(0x00d3_3670, &args![this, source]).u32();
        assert_eq!(result, this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(COPY_BYTES, vec![this + 4, buffer, 0x1008])]);
        assert_eq!(e.mem.u32(this), 0x1234);
        // Vectors at +0x1010 and +0x1020.
        for lane in 0..8 {
            assert_eq!(e.mem.u32(this + 0x1010 + lane * 4), 0x9000 + lane);
        }
        // +0x1030 and +0x1034, then the hole, then +0x1040..+0x104c.
        assert_eq!(e.mem.u32(this + 0x1030), 0x9008);
        assert_eq!(e.mem.u32(this + 0x1034), 0x9009);
        assert_eq!(e.mem.u32(this + 0x1038), 0);
        for word in 0..4 {
            assert_eq!(e.mem.u32(this + 0x1040 + word * 4), 0x900c + word);
        }
        // The 0x30-byte block overwrote +0x1050 and +0x1054.
        for word in 0..12 {
            assert_eq!(e.mem.u32(this + 0x1050 + word * 4), 0x9010 + word);
        }
    }

    #[test]
    fn fn_00d33670_without_a_buffer_copies_nothing_more() {
        let mut e = Engine::new();
        e.map(0x0104_d000, 0x1000);
        let source = e.mem.alloc(0x3100);
        let this = e.mem.alloc(0x1100);
        e.call_log = Some(vec![]);
        e.call(0x00d3_3670, &args![this, source]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}
