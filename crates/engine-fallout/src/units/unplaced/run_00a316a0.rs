//! `(unplaced)/run_00a316a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiObject`'s constructor (`thiscall`, no stack words).
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// `NiControllerSequence`'s vtable (by its RTTI type descriptor).
const NI_CONTROLLER_SEQUENCE_VTABLE: u32 = 0x0109_6584;
/// The word holding the handle of the empty `NiFixedString` (stored uncounted).
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// The float the begin key time starts as (`FLT_MAX`, `7f7fffff`, in the exe).
const BEGIN_KEY_TIME_FLOAT: u32 = 0x0109_6548;
/// The float `-FLT_MAX` (`ff7fffff`) the unset times start as.
const NO_TIME_FLOAT: u32 = 0x0101_5f5c;
/// `00a312d0` (unit `nicontrollersequence.cpp`, `thiscall`, five pointers to fixed strings):
/// `IDTag`'s constructor from five fixed strings.
const ID_TAG_CONSTRUCT: u32 = 0x00a3_12d0;
/// The static `NiFixedString` at `011f49e0`.
const STATIC_FIXED_STRING: u32 = 0x011f_49e0;

// Translated from 00a316a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (by its body `NiControllerSequence`'s empty constructor; the
/// field names are the Xbox PDB's): the `NiObject` constructor, the vtable `01096584`, the
/// name the empty fixed string (uncounted), weight and frequency 1, no interpolator or tag
/// arrays (size, growth, pointers 0), no text keys, cycle type 0, the begin key time the
/// float at `01096548`, the end key, last, weighted last, last scaled, offset, start, end and
/// destination frame times `-FLT_MAX` (`01015f5c`), no owner, state 0, no partner sequence
/// (+0x58), the accumulation root name the empty fixed string (uncounted), the words at
/// +0x60, +0x64 and +0x6c 0, the 16-bit words at +0x68 `0xffff` and +0x70 0, the byte at
/// +0x72 0. Answers `this`.
pub fn fn_00a316a0(e: &mut Engine, this: Ptr) -> Ptr {
    let a = this.addr();
    e.call(OBJECT_CONSTRUCT, &args![this]);
    let one = 1.0f32.to_bits();
    e.mem.set_u32(a, NI_CONTROLLER_SEQUENCE_VTABLE);
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.mem.set_u32(a + 0x08, empty);
    e.mem.set_u32(a + 0x1c, one);
    for offset in [0x0cu32, 0x10, 0x14, 0x18, 0x20] {
        e.mem.set_u32(a + offset, 0);
    }
    e.mem.set_u32(a + 0x28, one);
    let begin = x87_store32(x87_load32(e.mem.u32(BEGIN_KEY_TIME_FLOAT)));
    e.mem.set_u32(a + 0x24, 0);
    e.mem.set_u32(a + 0x2c, begin);
    e.mem.set_u32(a + 0x40, 0);
    let no_time = x87_store32(x87_load32(e.mem.u32(NO_TIME_FLOAT)));
    e.mem.set_u32(a + 0x44, 0);
    e.mem.set_u32(a + 0x30, no_time);
    e.mem.set_u32(a + 0x58, 0);
    for offset in [0x34u32, 0x38, 0x3c, 0x48, 0x4c, 0x50, 0x54] {
        e.mem.set_u32(a + offset, no_time);
    }
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.mem.set_u32(a + 0x5c, empty);
    e.mem.set_u32(a + 0x60, 0);
    e.mem.set_u32(a + 0x64, 0);
    e.mem.set_u32(a + 0x6c, 0);
    e.mem.set_u8(a + 0x72, 0);
    e.mem.set_u16(a + 0x68, 0xffff);
    e.mem.set_u16(a + 0x70, 0);
    this
}

// Translated from 00a31680 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `thiscall`, no stack words):
/// `IDTag`'s default constructor: `00a312d0` on `this` with the static fixed string at
/// `011f49e0` for all five names; answers its result.
pub fn fn_00a31680(e: &mut Engine, this: Ptr) -> Ptr {
    let s = STATIC_FIXED_STRING;
    e.call(ID_TAG_CONSTRUCT, &args![this, s, s, s, s, s]).ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3_16a0, fn_00a316a0(Ptr) -> Ptr),
        entry!(0x00a3_1680, fn_00a31680(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a316a0_sets_up_an_empty_sequence() {
        let mut e = Engine::new();
        e.map(0x0109_6000, 0x6000);
        e.map(0x0101_5000, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x9000);
        e.mem.set_u32(BEGIN_KEY_TIME_FLOAT, 0x7f7f_ffff);
        e.mem.set_u32(NO_TIME_FLOAT, 0xff7f_ffff);
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x74);
        e.mem.write(this, &[0xee; 0x74]);
        assert_eq!(e.call(0x00a3_16a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_CONTROLLER_SEQUENCE_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0xeeee_eeee);
        assert_eq!(e.mem.u32(this + 8), 0x9000);
        assert_eq!(e.mem.u32(this + 0x5c), 0x9000);
        assert_eq!(e.mem.f32(this + 0x1c), 1.0);
        assert_eq!(e.mem.f32(this + 0x28), 1.0);
        assert_eq!(e.mem.u32(this + 0x2c), 0x7f7f_ffff);
        for offset in [0x30u32, 0x34, 0x38, 0x3c, 0x48, 0x4c, 0x50, 0x54] {
            assert_eq!(e.mem.u32(this + offset), 0xff7f_ffff);
        }
        for offset in [
            0x0cu32, 0x10, 0x14, 0x18, 0x20, 0x24, 0x40, 0x44, 0x58, 0x60, 0x64, 0x6c,
        ] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u16(this + 0x68), 0xffff);
        assert_eq!(e.mem.u16(this + 0x70), 0);
        assert_eq!(e.mem.u8(this + 0x72), 0);
        assert_eq!(e.mem.u8(this + 0x73), 0xee);
    }

    #[test]
    fn fn_00a31680_builds_the_tag_from_the_static_string() {
        let mut e = Engine::new();
        e.register(ID_TAG_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a3_1680, &args![0x5000u32]).u32(), 0x5000);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == ID_TAG_CONSTRUCT).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(
            calls[0].1,
            vec![
                0x5000,
                STATIC_FIXED_STRING,
                STATIC_FIXED_STRING,
                STATIC_FIXED_STRING,
                STATIC_FIXED_STRING,
                STATIC_FIXED_STRING
            ]
        );
    }
}
