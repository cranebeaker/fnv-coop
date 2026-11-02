//! `fallout shared/run_00589450` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A constructor and a destructor body the engine map gives no name. The
//! offsets they set match `AudioMarkerData` in the Xbox PDB
//! (`LocationName` at +0, the layer percents at +0xc and +0x10, `Location`
//! at +0x18, `pController` at +0x24, `lBuoyMarkers` at +0x2c); the class is
//! not confirmed from the PC code.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00402d00` (`ECX` = object): the base at +0.
const BASE_CONSTRUCT: u32 = 0x0040_2d00;
/// `006815c0` (`ECX` = the point at +0x18).
const POINT_PREPARE: u32 = 0x0068_15c0;
/// `0096a2d0` (`ECX` = the list at +0x2c).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `0046ffb0` (`ECX` = the list at +0x2c).
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `00591300` (`ECX` = object): the base at +0.
const BASE_DESTRUCT: u32 = 0x0059_1300;
/// The floats stored at +0xc and +0x10 (`01026994`, `010320e4`).
const FIRST_FLOAT: u32 = 0x0102_6994;
const SECOND_FLOAT: u32 = 0x0103_20e4;
/// The point stored at +0x18 (`011f426c`, three floats).
const ORIGIN: u32 = 0x011f_426c;

// Translated from 00589450 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00589450 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs the object: the base (`00402d00`), `006815c0` on the point
/// at +0x18, the list at +0x2c (`0096a2d0`), the floats at `01026994` and
/// `010320e4` into +0xc and +0x10 (`FLD`/`FSTP`, a signalling NaN quieted),
/// the point at `011f426c` copied to +0x18, and +0x24 zero. Returns
/// `this`.
pub fn fn_00589450(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT, &args![this]);
    e.call(POINT_PREPARE, &args![base + 0x18]);
    e.call(LIST_CONSTRUCT, &args![base + 0x2c]);
    let first = x87_quiet_bits(e.mem.u32(FIRST_FLOAT));
    e.mem.set_u32(base + 0xc, first);
    let second = x87_quiet_bits(e.mem.u32(SECOND_FLOAT));
    e.mem.set_u32(base + 0x10, second);
    for i in 0..3 {
        let word = e.mem.u32(ORIGIN + 4 * i);
        e.mem.set_u32(base + 0x18 + 4 * i, word);
    }
    e.mem.set_u32(base + 0x24, 0);
    this
}

// Translated from 00589500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00589500 - the exception-unwinding frame (FS:[0]) is not translated.
/// The matching destructor body: the list at +0x2c (`0046ffb0`), then the
/// base (`00591300`).
pub fn fn_00589500(e: &mut Engine, this: Ptr) {
    e.call(LIST_DESTRUCT, &args![this.addr() + 0x2c]);
    e.call(BASE_DESTRUCT, &args![this]);
}

/// `0048cee0` (`thiscall`: the name's length), `MapMarkerData::GetLocationName`
/// (`00408da0`, engine map name; `thiscall`), `TESForm::AddChunkArray`
/// (`004856f0`, `cdecl`: chunk name, data, size) and the chunk writer
/// `00485910` (`cdecl`: chunk name, word).
const NAME_LENGTH: u32 = 0x0048_cee0;
const GET_LOCATION_NAME: u32 = 0x0040_8da0;
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
const WRITE_CHUNK_WORD: u32 = 0x0048_5910;

// Translated from 00589560 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`; the form's save): a `FULL` chunk with the name
/// (`00408da0`) and its length plus one (`0048cee0`), a `CNAM` chunk with the
/// word at +0x24 when it is set, `BNAM` with whether the byte at +0x28 is
/// set (0 or 1), then `MNAM` and `NNAM` with the floats at +0xC and +0x10
/// (copied through the x87 stack).
pub fn fn_00589560(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    let size = e.call(NAME_LENGTH, &args![this]).u32().wrapping_add(1);
    let name = e.call(GET_LOCATION_NAME, &args![this]).u32();
    e.call(ADD_CHUNK_ARRAY, &args![0x4c4c_5546u32, name, size]);
    let word = e.mem.u32(at + 0x24);
    if word != 0 {
        e.call(WRITE_CHUNK_WORD, &args![0x4d41_4e43u32, word]);
    }
    let set = u32::from(e.mem.u8(at + 0x28) != 0);
    e.call(WRITE_CHUNK_WORD, &args![0x4d41_4e42u32, set]);
    let value = x87_quiet_bits(e.mem.u32(at + 0xc));
    e.call(WRITE_CHUNK_WORD, &args![0x4d41_4e4du32, value]);
    let value = x87_quiet_bits(e.mem.u32(at + 0x10));
    e.call(WRITE_CHUNK_WORD, &args![0x4d41_4e4eu32, value]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_9450, fn_00589450(Ptr) -> Ptr),
        entry!(0x0058_9500, fn_00589500(Ptr)),
        entry!(0x0058_9560, fn_00589560(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    #[test]
    fn at_00589450_constructs_the_parts_and_sets_the_fields() {
        let mut e = Engine::new();
        for addr in [BASE_CONSTRUCT, POINT_PREPARE, LIST_CONSTRUCT] {
            e.register(addr, |_, _| Ret::default());
        }
        e.map(FIRST_FLOAT, 4);
        e.map(SECOND_FLOAT, 4);
        e.map(ORIGIN, 0xc);
        e.mem.set_u32(FIRST_FLOAT, 0.33f32.to_bits());
        e.mem.set_u32(SECOND_FLOAT, 0xff80_0002);
        for i in 0..3 {
            e.mem.set_u32(ORIGIN + 4 * i, 0x100 + i);
        }
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 0x24, 0xffff_ffff);
        let (result, log) = run(&mut e, 0x0058_9450, &[this]);
        assert_eq!(result.u32(), this);
        assert_eq!(
            log,
            vec![
                (BASE_CONSTRUCT, vec![this]),
                (POINT_PREPARE, vec![this + 0x18]),
                (LIST_CONSTRUCT, vec![this + 0x2c]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0xc), 0.33f32.to_bits());
        assert_eq!(e.mem.u32(this + 0x10), 0xffc0_0002);
        let point: Vec<u32> = (0..3).map(|i| e.mem.u32(this + 0x18 + 4 * i)).collect();
        assert_eq!(point, vec![0x100, 0x101, 0x102]);
        assert_eq!(e.mem.u32(this + 0x24), 0);
    }

    #[test]
    fn at_00589500_destroys_the_list_then_the_base() {
        let mut e = Engine::new();
        for addr in [LIST_DESTRUCT, BASE_DESTRUCT] {
            e.register(addr, |_, _| Ret::default());
        }
        let (_, log) = run(&mut e, 0x0058_9500, &[0x4000]);
        assert_eq!(
            log,
            vec![(LIST_DESTRUCT, vec![0x402c]), (BASE_DESTRUCT, vec![0x4000])]
        );
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    #[test]
    fn fn_00589560_saves_the_chunks() {
        let mut e = Engine::new();
        e.register(NAME_LENGTH, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        e.register(GET_LOCATION_NAME, |_, _| Ret {
            eax: 0x0077_0000,
            ..Ret::default()
        });
        e.register(ADD_CHUNK_ARRAY, |_, _| Ret::default());
        e.register(WRITE_CHUNK_WORD, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        e.mem.set_u8(this + 0x28, 0x40);
        e.mem.set_f32(this + 0xc, 1.5);
        e.mem.set_u32(this + 0x10, 0x7f80_0001);
        let targets = [ADD_CHUNK_ARRAY, WRITE_CHUNK_WORD];
        e.call_log = Some(vec![]);
        e.call(0x0058_9560, &[this]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| targets.contains(a))
            .collect();
        assert_eq!(
            log,
            vec![
                (ADD_CHUNK_ARRAY, vec![0x4c4c_5546, 0x0077_0000, 6]),
                (WRITE_CHUNK_WORD, vec![0x4d41_4e42, 1]),
                (WRITE_CHUNK_WORD, vec![0x4d41_4e4d, 1.5f32.to_bits()]),
                (WRITE_CHUNK_WORD, vec![0x4d41_4e4e, 0x7fc0_0001]),
            ]
        );
        e.mem.set_u32(this + 0x24, 0x1234);
        e.mem.set_u8(this + 0x28, 0);
        e.call_log = Some(vec![]);
        e.call(0x0058_9560, &[this]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == WRITE_CHUNK_WORD)
            .collect();
        assert_eq!(log[0].1, vec![0x4d41_4e43, 0x1234]);
        assert_eq!(log[1].1, vec![0x4d41_4e42, 0]);
    }
}
