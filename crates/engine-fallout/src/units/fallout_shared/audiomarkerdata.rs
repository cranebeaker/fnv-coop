//! `fallout shared/audiomarkerdata.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `AudioMarkerData` (Xbox PDB), 0x34 bytes; the fields `Load` sets.
    pub struct AudioMarkerData: 0x34 {
        /// `SecondLayerPercent` (Xbox PDB).
        0x0c SecondLayerPercent: f32,
        /// `ThirdLayerPercent` (Xbox PDB).
        0x10 ThirdLayerPercent: f32,
        /// `pController` (Xbox PDB).
        0x24 pController: u32,
        /// `bUseController` (Xbox PDB).
        0x28 bUseController: bool,
    }
}

/// `TESFile::GetTESChunk` (Xbox PDB name of `004726b0`; `ECX` = file).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (`ECX` = file): moves to the next chunk; true when there is one.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESFile::GetChunkData` (Xbox PDB name of `004727f0`; `ECX` = file, the
/// address of a word).
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `00487050` (cdecl: the full-name component, file): loads it.
const LOAD_FULL_NAME: u32 = 0x0048_7050;
/// `004579e0` (cdecl: a float): its result in `ST0`.
const FLOAT_CALL_004579E0: u32 = 0x0045_79e0;
/// The chunk tags, in the order the record must have them.
const CHUNK_MMRK: u32 = 0x4b52_4d4d;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_CNAM: u32 = 0x4d41_4e43;
const CHUNK_BNAM: u32 = 0x4d41_4e42;
const CHUNK_MNAM: u32 = 0x4d41_4e4d;
const CHUNK_NNAM: u32 = 0x4d41_4e4e;

/// Moves to the next chunk and checks its tag.
fn next_is(e: &mut Engine, file: Ptr, tag: u32) -> bool {
    if !e.call(NEXT_CHUNK, &args![file]).bool() {
        return false;
    }
    e.call(GET_TES_CHUNK, &args![file]).u32() == tag
}

// Translated from 00589600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AudioMarkerData::Load` (Xbox PDB; the file; `RET 4`): false for a null
/// file or as soon as the next chunk is missing or not the expected one.
/// The chunks must be `MMRK`, then `FULL` (the full name, `00487050` on
/// `this`), `CNAM` (a word into `pController`), `BNAM` (a word;
/// `bUseController` is whether it is 1), `MNAM` (into
/// `SecondLayerPercent`) and `NNAM` (into `ThirdLayerPercent`); true after
/// the last.
pub fn audio_marker_data_load(e: &mut Engine, this: Ptr<AudioMarkerData>, file: Ptr) -> bool {
    if file.is_null() {
        return false;
    }
    if e.call(GET_TES_CHUNK, &args![file]).u32() != CHUNK_MMRK {
        return false;
    }
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        if !next_is(e, file, CHUNK_FULL) {
            return false;
        }
        e.call(LOAD_FULL_NAME, &args![this, file]);
        if !next_is(e, file, CHUNK_CNAM) {
            return false;
        }
        e.call(GET_CHUNK_WORD, &args![file, word]);
        let controller = e.mem.u32(word.addr());
        e.set(this, AudioMarkerData::pController, controller);
        if !next_is(e, file, CHUNK_BNAM) {
            return false;
        }
        e.call(GET_CHUNK_WORD, &args![file, word]);
        let use_controller = e.mem.u32(word.addr()) == 1;
        e.set(this, AudioMarkerData::bUseController, use_controller);
        if !next_is(e, file, CHUNK_MNAM) {
            return false;
        }
        e.call(GET_CHUNK_WORD, &args![file, this.addr() + 0xc]);
        if !next_is(e, file, CHUNK_NNAM) {
            return false;
        }
        e.call(GET_CHUNK_WORD, &args![file, this.addr() + 0x10]);
        true
    })
}

// Translated from 00589850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `x * x + y * y` of the two floats at `this` (x87, rounded to a float),
/// handed to `004579e0`; returns what that leaves in `ST0`. The engine map
/// gives it no name.
pub fn fn_00589850(e: &mut Engine, this: Ptr) -> f64 {
    let x = e.mem.u32(this.addr());
    let y = e.mem.u32(this.addr() + 4);
    let xx = x87_mul(x87_load32(x), x87_m32(x));
    let yy = x87_mul(x87_load32(y), x87_m32(y));
    // FADDP: ST1 (x * x) + ST0 (y * y), then FSTP / FLD / FSTP float.
    let sum = x87_store32(x87_add(xx, yy));
    let pushed = x87_quiet_bits(sum);
    e.call(FLOAT_CALL_004579E0, &args![pushed]).f64()
}

/// The marker data's virtual slots: the base copy (`0x8`) and the base comparison
/// (`0xC`, `AL` true when different), each with the other marker data.
const SLOT_COPY_BASE: u32 = 0x8;
const SLOT_COMPARE_BASE: u32 = 0xc;

// Translated from 00589770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies `other` into this audio marker data (`thiscall`, `RET 4`; unnamed in the engine
/// map): the base part through virtual slot `0x8`, then the controller, the
/// use-controller byte and the two layer percentages (`FLD`/`FSTP`, which quiets a
/// signalling NaN).
pub fn fn_00589770(e: &mut Engine, this: Ptr<AudioMarkerData>, other: Ptr<AudioMarkerData>) {
    e.vcall(this.addr(), SLOT_COPY_BASE, &args![other]);
    let controller = e.get(other, AudioMarkerData::pController);
    e.set(this, AudioMarkerData::pController, controller);
    let use_controller = e.mem.u8(other.addr() + 0x28);
    e.mem.set_u8(this.addr() + 0x28, use_controller);
    let second = e.mem.u32(other.addr() + 0xc);
    e.mem.set_u32(this.addr() + 0xc, x87_quiet_bits(second));
    let third = e.mem.u32(other.addr() + 0x10);
    e.mem.set_u32(this.addr() + 0x10, x87_quiet_bits(third));
}

// Translated from 005897c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AudioMarkerData::Compare` (Xbox PDB; `RET 4`, result in `AL`): true ("differs") when
/// the base comparison (virtual slot `0xC`) says so, or the controller, the
/// use-controller byte or either layer percentage differs (`FUCOMPP`: unordered
/// differs); false otherwise.
pub fn audio_marker_data_compare(
    e: &mut Engine,
    this: Ptr<AudioMarkerData>,
    other: Ptr<AudioMarkerData>,
) -> bool {
    if e.vcall(this.addr(), SLOT_COMPARE_BASE, &args![other]).u8() != 0 {
        return true;
    }
    if e.get(this, AudioMarkerData::pController) != e.get(other, AudioMarkerData::pController) {
        return true;
    }
    if e.mem.u8(this.addr() + 0x28) != e.mem.u8(other.addr() + 0x28) {
        return true;
    }
    for offset in [0xc, 0x10] {
        let ours = x87_m32(e.mem.u32(this.addr() + offset));
        let theirs = x87_m32(e.mem.u32(other.addr() + offset));
        if theirs != ours {
            return true;
        }
    }
    false
}

// Translated from 00589890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AudioBuoyMarkerData::Load` (Xbox PDB; `RET 4`, the file; result in `AL`): false for a
/// null file, otherwise whether the current chunk (`TESFile::GetTESChunk`) is `MMRK`.
pub fn audio_buoy_marker_data_load(e: &mut Engine, _this: Ptr, file: Ptr) -> bool {
    if file.is_null() {
        return false;
    }
    e.call(GET_TES_CHUNK, &args![file]).u32() == CHUNK_MMRK
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0058_9600,
            audio_marker_data_load(Ptr<AudioMarkerData>, Ptr) -> bool
        ),
        entry!(0x0058_9850, fn_00589850(Ptr) -> f64),
        entry!(
            0x0058_9770,
            fn_00589770(Ptr<AudioMarkerData>, Ptr<AudioMarkerData>)
        ),
        entry!(
            0x0058_97c0,
            audio_marker_data_compare(Ptr<AudioMarkerData>, Ptr<AudioMarkerData>) -> bool
        ),
        entry!(0x0058_9890, audio_buoy_marker_data_load(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Runs `Load` over `chunks`; each `GetChunkData` writes 0x100 plus its
    /// position in the list.
    fn load(chunks: Vec<u32>) -> (Engine, u32, bool) {
        let mut e = Engine::new();
        let position = Rc::new(Cell::new(0usize));
        let (list, at) = (chunks.clone(), position.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(list.get(at.get()).copied().unwrap_or(0))
        });
        let total = chunks.len();
        let next = position.clone();
        e.register_double(NEXT_CHUNK, move |_, _| {
            next.set(next.get() + 1);
            ret(u32::from(next.get() < total))
        });
        e.register_double(GET_CHUNK_WORD, move |e, a| {
            e.mem.set_u32(a[1], 0x100 + position.get() as u32);
            ret(0)
        });
        e.register(LOAD_FULL_NAME, |_, _| ret(0));
        let this = e.mem.alloc(0x34);
        let result = e.call(0x0058_9600, &args![this, 0x7000u32]).bool();
        (e, this, result)
    }

    const ALL: [u32; 6] = [
        CHUNK_MMRK, CHUNK_FULL, CHUNK_CNAM, CHUNK_BNAM, CHUNK_MNAM, CHUNK_NNAM,
    ];

    #[test]
    fn load_reads_every_chunk_in_order() {
        let (e, this, result) = load(ALL.to_vec());
        assert!(result);
        assert_eq!(e.mem.u32(this + 0x24), 0x102);
        // BNAM read 0x103, not 1.
        assert_eq!(e.mem.u8(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x104);
        assert_eq!(e.mem.u32(this + 0x10), 0x105);
    }

    #[test]
    fn load_stops_at_a_missing_or_wrong_chunk() {
        for cut in 1..ALL.len() {
            let mut chunks = ALL.to_vec();
            chunks[cut] = 0x1234;
            let (e, this, result) = load(chunks);
            assert!(!result);
            assert_eq!(e.mem.u32(this + 0x10), 0);
            let (_, _, result) = load(ALL[..cut].to_vec());
            assert!(!result);
        }
        let (_, _, result) = load(vec![CHUNK_FULL]);
        assert!(!result);
        let mut e = Engine::new();
        assert!(!e.call(0x0058_9600, &args![0x4000u32, 0u32]).bool());
    }

    #[test]
    fn use_controller_is_whether_bnam_is_one() {
        let mut e = Engine::new();
        let tags = Rc::new(Cell::new(0usize));
        let at = tags.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| ret(ALL[at.get()]));
        let next = tags.clone();
        e.register_double(NEXT_CHUNK, move |_, _| {
            next.set(next.get() + 1);
            ret(1)
        });
        e.register_double(GET_CHUNK_WORD, move |e, a| {
            e.mem.set_u32(a[1], u32::from(tags.get() == 3));
            ret(0)
        });
        e.register(LOAD_FULL_NAME, |_, _| ret(0));
        let this = e.mem.alloc(0x34);
        assert!(e.call(0x0058_9600, &args![this, 0x7000u32]).bool());
        assert_eq!(e.mem.u8(this + 0x28), 1);
    }

    #[test]
    fn at_00589850_passes_the_squared_length() {
        let mut e = Engine::new();
        e.register(FLOAT_CALL_004579E0, |_, a| {
            f64::from(f32::from_bits(a[0])).into_ret()
        });
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 3.0f32.to_bits());
        e.mem.set_u32(this + 4, (-4.0f32).to_bits());
        assert_eq!(e.call(0x0058_9850, &args![this]).f64(), 25.0);
        e.mem.set_u32(this, 0x7f80_0001);
        let result = e.call(0x0058_9850, &args![this]).f64();
        assert!(result.is_nan());
    }
}

#[cfg(test)]
mod tests_n036 {
    use super::*;

    /// A marker data with a vtable whose slots `0x8` and `0xC` go to `0x4008` and
    /// `0x400C`.
    fn marker(e: &mut Engine, controller: u32, flag: u8, second: f32, third: f32) -> u32 {
        e.put_vtable(0x0180_0000, &[0, 0, 0x4008, 0x400c]);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this, 0x0180_0000);
        e.mem.set_u32(this + 0x24, controller);
        e.mem.set_u8(this + 0x28, flag);
        e.mem.set_u32(this + 0xc, second.to_bits());
        e.mem.set_u32(this + 0x10, third.to_bits());
        this
    }

    #[test]
    fn copy_takes_the_base_and_the_fields() {
        let mut e = Engine::new();
        e.register(0x4008, |_, _| Ret::default());
        let this = marker(&mut e, 0, 0, 0.0, 0.0);
        let other = marker(&mut e, 0x77, 1, 0.5, 0.25);
        e.mem.set_u32(other + 0x10, 0xff80_0001);
        e.call_log = Some(vec![]);
        e.call(0x0058_9770, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x4008, vec![this, other])));
        assert_eq!(e.mem.u32(this + 0x24), 0x77);
        assert_eq!(e.mem.u8(this + 0x28), 1);
        assert_eq!(e.mem.u32(this + 0xc), 0.5f32.to_bits());
        assert_eq!(e.mem.u32(this + 0x10), 0xffc0_0001);
    }

    #[test]
    fn compare_checks_the_base_and_every_field() {
        let mut e = Engine::new();
        e.register(0x400c, |_, _| 0u32.into_ret());
        let this = marker(&mut e, 5, 1, 0.5, 0.25);
        let same = marker(&mut e, 5, 1, 0.5, 0.25);
        assert!(!e.call(0x0058_97c0, &args![this, same]).bool());
        for other in [
            marker(&mut e, 6, 1, 0.5, 0.25),
            marker(&mut e, 5, 0, 0.5, 0.25),
            marker(&mut e, 5, 1, 0.75, 0.25),
            marker(&mut e, 5, 1, 0.5, f32::NAN),
        ] {
            assert!(e.call(0x0058_97c0, &args![this, other]).bool());
        }
        e.register(0x400c, |_, _| 1u32.into_ret());
        assert!(e.call(0x0058_97c0, &args![this, same]).bool());
    }

    #[test]
    fn buoy_load_accepts_an_mmrk_chunk() {
        let mut e = Engine::new();
        e.register(GET_TES_CHUNK, |e, a| e.mem.u32(a[0]).into_ret());
        let file = e.mem.alloc(4);
        assert!(!e.call(0x0058_9890, &args![0x10u32, 0u32]).bool());
        e.mem.set_u32(file, CHUNK_MMRK);
        assert!(e.call(0x0058_9890, &args![0x10u32, file]).bool());
        e.mem.set_u32(file, CHUNK_FULL);
        assert!(!e.call(0x0058_9890, &args![0x10u32, file]).bool());
    }
}
