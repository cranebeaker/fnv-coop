//! `fallout shared/mapmarkerdata.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The text of a string object (`00559450`: the word at its `+0`).
const STRING_TEXT: u32 = 0x0055_9450;
/// The empty string the exe answers for a string without text
/// (`01011584`).
const EMPTY_TEXT: u32 = 0x0101_1584;

// Translated from 00408da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The text of the string at `this + 4`, or the exe's empty string
/// (`01011584`) when it has none. The engine map names it
/// `MapMarkerData::GetLocationName` (Xbox PDB); the body is shared by
/// identical-code folding (`TESForm`'s full-name getter `0048274c` calls it
/// too), so it is named by address: it reads `cFullName` of a
/// `TESFullName` (Xbox PDB, `+4`), which is `LocationName.cFullName` for a
/// `MapMarkerData`.
pub fn fn_00408da0(e: &mut Engine, this: Ptr) -> u32 {
    let string = this.addr() + 4;
    if e.call(STRING_TEXT, &args![string]).u32() == 0 {
        EMPTY_TEXT
    } else {
        e.call(STRING_TEXT, &args![string]).u32()
    }
}

layout! {
    /// `MapMarkerData` (Xbox PDB), 0x14 bytes (the PC offsets agree).
    pub struct MapMarkerData: 0x14 {
        /// `cFlags` (Xbox PDB): bit 0 visible, bit 1 travel location, bit 2
        /// hidden (the Xbox PDB names of the getters and setters).
        0x0C cFlags: u8,
        /// `cOriginalFlags` (Xbox PDB).
        0x0D cOriginalFlags: u8,
        /// `sType` (Xbox PDB).
        0x0E sType: u16,
        /// `pReputation` (Xbox PDB): `TESReputation*`.
        0x10 pReputation: u32,
    }
}

/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// Moves to the next chunk (`004726f0`, `bool`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`): buffer and size.
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `TESFile::GetChunkData_ov2` (Xbox PDB, `00472840`).
const GET_CHUNK_DATA_OV2: u32 = 0x0047_2840;
/// `TESFile::GetChunkData` (Xbox PDB, `004727f0`): one word.
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// The file's current chunk offset (`004724c0`).
const CHUNK_OFFSET: u32 = 0x0047_24c0;
/// `TESFile::SetOffsetChunk` (Xbox PDB, `004724f0`, `RET 4`).
const SET_OFFSET_CHUNK: u32 = 0x0047_24f0;
/// Loads a `TESFullName` from the file's `FULL` chunk (`00487050`, cdecl).
const FULL_NAME_LOAD: u32 = 0x0048_7050;

/// Chunk signatures, as the words the exe compares.
const SIG_XMRK: u32 = 0x4b52_4d58;
const SIG_FNAM: u32 = 0x4d41_4e46;
const SIG_FULL: u32 = 0x4c4c_5546;
const SIG_TNAM: u32 = 0x4d41_4e54;
const SIG_WMI1: u32 = 0x3149_4d57;

// Translated from 00438ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMarkerData::Load` (Xbox PDB): reads a map marker from `file` in a
/// fixed order: `XMRK`, an optional `FNAM` (one byte into `cFlags`, copied
/// to `cOriginalFlags`), `FULL` (the name, `00487050`), `TNAM` (`sType`,
/// `TESFile::GetChunkData_ov2`), then an optional `WMI1` (a reputation
/// word, stored only when not 0); without `WMI1` the file goes back to the
/// chunk offset taken after `TNAM` (`TESFile::SetOffsetChunk`). False for
/// no file, a missing required chunk or a missing next chunk; true at the
/// end.
pub fn map_marker_data_load(e: &mut Engine, this: Ptr<MapMarkerData>, file: Ptr) -> bool {
    if file.is_null() {
        return false;
    }
    let chunk = |e: &mut Engine| e.call(GET_TES_CHUNK, &args![file]).u32();
    let next = |e: &mut Engine| e.call(NEXT_CHUNK, &args![file]).bool();
    if chunk(e) != SIG_XMRK || !next(e) {
        return false;
    }
    if chunk(e) == SIG_FNAM {
        e.call(GET_CHUNK_DATA_SIZED, &args![file, this.byte_add(0xc), 1u32]);
        let flags = e.get(this, MapMarkerData::cFlags);
        e.set(this, MapMarkerData::cOriginalFlags, flags);
        if !next(e) {
            return false;
        }
    }
    if chunk(e) != SIG_FULL {
        return false;
    }
    e.call(FULL_NAME_LOAD, &args![this, file]);
    if !next(e) || chunk(e) != SIG_TNAM {
        return false;
    }
    e.call(GET_CHUNK_DATA_OV2, &args![file, this.byte_add(0xe)]);
    let offset = e.call(CHUNK_OFFSET, &args![file]).u32();
    if !next(e) {
        return false;
    }
    if chunk(e) == SIG_WMI1 {
        e.with_stack(4, |e, reputation_at| {
            e.mem.set_u32(reputation_at.addr(), 0);
            e.call(GET_CHUNK_DATA, &args![file, reputation_at]);
            let reputation = e.mem.u32(reputation_at.addr());
            if reputation != 0 {
                e.set(this, MapMarkerData::pReputation, reputation);
            }
        });
    } else {
        e.call(SET_OFFSET_CHUNK, &args![file, offset]);
    }
    true
}

// Translated from 00438df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a map marker: the name through vtable slot 8 with `source`, then
/// `cFlags`, `sType` and `pReputation` (`cOriginalFlags` is not copied).
pub fn fn_00438df0(e: &mut Engine, this: Ptr<MapMarkerData>, source: Ptr<MapMarkerData>) {
    e.vcall(this.addr(), 8, &args![source]);
    let flags = e.get(source, MapMarkerData::cFlags);
    e.set(this, MapMarkerData::cFlags, flags);
    let kind = e.get(source, MapMarkerData::sType);
    e.set(this, MapMarkerData::sType, kind);
    let reputation = e.get(source, MapMarkerData::pReputation);
    e.set(this, MapMarkerData::pReputation, reputation);
}

// Translated from 00438e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMarkerData::Compare` (Xbox PDB): true when the markers differ: the
/// name (vtable slot 0xC with `other`), `cFlags`, `sType` or
/// `pReputation`.
pub fn map_marker_data_compare(
    e: &mut Engine,
    this: Ptr<MapMarkerData>,
    other: Ptr<MapMarkerData>,
) -> bool {
    e.vcall(this.addr(), 0xc, &args![other]).bool()
        || e.get(this, MapMarkerData::cFlags) != e.get(other, MapMarkerData::cFlags)
        || e.get(this, MapMarkerData::sType) != e.get(other, MapMarkerData::sType)
        || e.get(this, MapMarkerData::pReputation) != e.get(other, MapMarkerData::pReputation)
}

// Translated from 00438eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00408da0` (the location name's text) and leaves its answer in
/// `EAX` (Ghidra types it `void`; the register is untouched after the
/// call).
pub fn fn_00438eb0(e: &mut Engine, this: Ptr<MapMarkerData>) -> u32 {
    fn_00408da0(e, this.cast())
}

fn flag(e: &Engine, this: Ptr<MapMarkerData>, bit: u8) -> bool {
    e.get(this, MapMarkerData::cFlags) & bit != 0
}

fn set_flag(e: &mut Engine, this: Ptr<MapMarkerData>, bit: u8, on: bool) {
    let flags = e.get(this, MapMarkerData::cFlags);
    let flags = if on { flags | bit } else { flags & !bit };
    e.set(this, MapMarkerData::cFlags, flags);
}

// Translated from 00438ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMarkerData::GetVisible` (Xbox PDB): `cFlags` bit 0.
pub fn map_marker_data_get_visible(e: &mut Engine, this: Ptr<MapMarkerData>) -> bool {
    flag(e, this, 1)
}

// Translated from 00438ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMarkerData::GetTravelLoc` (Xbox PDB): `cFlags` bit 1.
pub fn map_marker_data_get_travel_loc(e: &mut Engine, this: Ptr<MapMarkerData>) -> bool {
    flag(e, this, 2)
}

// Translated from 00438f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMarkerData::GetHidden` (Xbox PDB): `cFlags` bit 2.
pub fn map_marker_data_get_hidden(e: &mut Engine, this: Ptr<MapMarkerData>) -> bool {
    flag(e, this, 4)
}

// Translated from 0044de40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMarkerData::SetVisible` (Xbox PDB): sets or clears `cFlags` bit 0.
pub fn map_marker_data_set_visible(e: &mut Engine, this: Ptr<MapMarkerData>, visible: bool) {
    set_flag(e, this, 1, visible);
}

// Translated from 0044de80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMarkerData::SetTravelLoc` (Xbox PDB): sets or clears `cFlags` bit 1.
pub fn map_marker_data_set_travel_loc(e: &mut Engine, this: Ptr<MapMarkerData>, travel: bool) {
    set_flag(e, this, 2, travel);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00408da0, fn_00408da0(Ptr) -> u32),
        entry!(
            0x0043_8ca0,
            map_marker_data_load(Ptr<MapMarkerData>, Ptr) -> bool
        ),
        entry!(
            0x0043_8df0,
            fn_00438df0(Ptr<MapMarkerData>, Ptr<MapMarkerData>)
        ),
        entry!(
            0x0043_8e40,
            map_marker_data_compare(Ptr<MapMarkerData>, Ptr<MapMarkerData>) -> bool
        ),
        entry!(0x0043_8eb0, fn_00438eb0(Ptr<MapMarkerData>) -> u32),
        entry!(
            0x0043_8ed0,
            map_marker_data_get_visible(Ptr<MapMarkerData>) -> bool
        ),
        entry!(
            0x0043_8ef0,
            map_marker_data_get_travel_loc(Ptr<MapMarkerData>) -> bool
        ),
        entry!(
            0x0043_8f10,
            map_marker_data_get_hidden(Ptr<MapMarkerData>) -> bool
        ),
        entry!(
            0x0044_de40,
            map_marker_data_set_visible(Ptr<MapMarkerData>, bool)
        ),
        entry!(
            0x0044_de80,
            map_marker_data_set_travel_loc(Ptr<MapMarkerData>, bool)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_text_or_the_empty_string() {
        let mut e = Engine::new();
        e.register(STRING_TEXT, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        let name = e.mem.alloc(0xc);
        e.mem.set_u32(name + 4, 0x7000);
        assert_eq!(e.call(0x0040_8da0, &[name]).u32(), 0x7000);
        e.mem.set_u32(name + 4, 0);
        assert_eq!(e.call(0x0040_8da0, &[name]).u32(), EMPTY_TEXT);
        e.call_log = Some(vec![]);
        e.call(0x0040_8da0, &[name]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .skip(1)
            .all(|(a, w)| *a == STRING_TEXT && w == &[name + 4]));
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A file double: `chunks` is the sequence of signatures (moving past
    /// the last one is "no next chunk"); data reads write
    /// fixed values.
    fn file_engine(chunks: &[u32]) -> Engine {
        let mut e = Engine::new();
        let list: Vec<u32> = chunks.to_vec();
        let at = std::rc::Rc::new(std::cell::Cell::new(0usize));
        let (a1, a2) = (at.clone(), at);
        let l1 = list.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| ret(l1[a1.get()]));
        e.register_double(NEXT_CHUNK, move |_, _| {
            a2.set(a2.get() + 1);
            ret((a2.get() < list.len()) as u32)
        });
        e.register(GET_CHUNK_DATA_SIZED, |e, a| {
            e.mem.set_u8(a[1], 0x05);
            Ret::default()
        });
        e.register(GET_CHUNK_DATA_OV2, |e, a| {
            e.mem.set_u16(a[1], 0x0007);
            Ret::default()
        });
        e.register(GET_CHUNK_DATA, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            e.mem.set_u32(a[1], 0x0abc);
            Ret::default()
        });
        e.register(CHUNK_OFFSET, |_, _| ret(0x4321));
        e.register(SET_OFFSET_CHUNK, |_, _| Ret::default());
        e.register(FULL_NAME_LOAD, |_, _| Ret::default());
        e
    }

    #[test]
    fn load_reads_the_marker_chunks_in_order() {
        let file = 0x0500_0000;
        let mut e = file_engine(&[SIG_XMRK, SIG_FNAM, SIG_FULL, SIG_TNAM, SIG_WMI1]);
        let marker = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0043_8ca0, &[marker, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            (e.mem.u8(marker + 0xc), e.mem.u8(marker + 0xd)),
            (0x05, 0x05)
        );
        assert_eq!(e.mem.u16(marker + 0xe), 7);
        assert_eq!(e.mem.u32(marker + 0x10), 0x0abc);
        assert_eq!(
            calls_to(&log, GET_CHUNK_DATA_SIZED),
            vec![vec![file, marker + 0xc, 1]]
        );
        assert_eq!(calls_to(&log, FULL_NAME_LOAD), vec![vec![marker, file]]);
        assert!(calls_to(&log, SET_OFFSET_CHUNK).is_empty());

        // No FNAM, no WMI1: the file goes back to the offset after TNAM.
        let mut e = file_engine(&[SIG_XMRK, SIG_FULL, SIG_TNAM, 0x1111_1111]);
        let marker = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0043_8ca0, &[marker, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SET_OFFSET_CHUNK), vec![vec![file, 0x4321]]);
        assert_eq!(e.mem.u8(marker + 0xc), 0);
        assert_eq!(e.mem.u32(marker + 0x10), 0);
    }

    #[test]
    fn load_fails_on_a_missing_chunk() {
        let file = 0x0500_0000;
        let mut e = file_engine(&[SIG_XMRK]);
        let marker = e.mem.alloc(0x14);
        assert!(!e.call(0x0043_8ca0, &[marker, 0]).bool());
        assert!(!e.call(0x0043_8ca0, &[marker, file]).bool());
        for chunks in [
            &[SIG_FULL][..],
            &[SIG_XMRK, SIG_TNAM][..],
            &[SIG_XMRK, SIG_FNAM][..],
            &[SIG_XMRK, SIG_FULL][..],
            &[SIG_XMRK, SIG_FULL, SIG_FULL][..],
            &[SIG_XMRK, SIG_FULL, SIG_TNAM][..],
        ] {
            let mut e = file_engine(chunks);
            let marker = e.mem.alloc(0x14);
            assert!(!e.call(0x0043_8ca0, &[marker, file]).bool(), "{chunks:x?}");
        }
    }

    fn marker(e: &mut Engine, flags: u8, original: u8, kind: u16, reputation: u32) -> u32 {
        let m = e.mem.alloc(0x14);
        e.mem.set_u32(m, 0x0600_0000);
        e.mem.set_u8(m + 0xc, flags);
        e.mem.set_u8(m + 0xd, original);
        e.mem.set_u16(m + 0xe, kind);
        e.mem.set_u32(m + 0x10, reputation);
        m
    }

    #[test]
    fn copy_takes_name_flags_type_and_reputation() {
        let mut e = Engine::new();
        e.put_vtable(0x0600_0000, &[0, 0, 0x7000_0008]);
        e.register(0x7000_0008, |_, _| Ret::default());
        let source = marker(&mut e, 3, 9, 0x22, 0x0333);
        let this = marker(&mut e, 0, 1, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_8df0, &[this, source]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x7000_0008), vec![vec![this, source]]);
        assert_eq!(
            (
                e.mem.u8(this + 0xc),
                e.mem.u8(this + 0xd),
                e.mem.u16(this + 0xe),
                e.mem.u32(this + 0x10)
            ),
            (3, 1, 0x22, 0x0333)
        );
    }

    #[test]
    fn compare_reports_any_difference() {
        let mut e = Engine::new();
        e.put_vtable(0x0600_0000, &[0, 0, 0, 0x7000_000c]);
        e.register(0x7000_000c, |e, a| ret(e.mem.u32(a[1] + 4)));
        let this = marker(&mut e, 1, 0, 2, 3);
        let same = marker(&mut e, 1, 7, 2, 3);
        assert!(!e.call(0x0043_8e40, &[this, same]).bool());
        for other in [
            marker(&mut e, 2, 0, 2, 3),
            marker(&mut e, 1, 0, 0x102, 3),
            marker(&mut e, 1, 0, 2, 4),
        ] {
            assert!(e.call(0x0043_8e40, &[this, other]).bool());
        }
        e.mem.set_u32(same + 4, 0xffff_ff01);
        assert!(e.call(0x0043_8e40, &[this, same]).bool());
    }

    #[test]
    fn location_name_wrapper_returns_the_text() {
        let mut e = Engine::new();
        e.register(STRING_TEXT, |e, a| ret(e.mem.u32(a[0])));
        let m = marker(&mut e, 0, 0, 0, 0);
        e.mem.set_u32(m + 4, 0x7000);
        assert_eq!(e.call(0x0043_8eb0, &[m]).u32(), 0x7000);
    }

    #[test]
    fn flag_getters_and_setters() {
        let mut e = Engine::new();
        let m = marker(&mut e, 0, 0, 0, 0);
        let get = |e: &mut Engine, addr: u32| e.call(addr, &[m]).bool();
        for addr in [0x0043_8ed0, 0x0043_8ef0, 0x0043_8f10] {
            assert!(!get(&mut e, addr));
        }
        e.call(0x0044_de40, &[m, 0xff01]);
        assert_eq!(e.mem.u8(m + 0xc), 1);
        assert!(get(&mut e, 0x0043_8ed0));
        e.call(0x0044_de80, &[m, 1]);
        assert_eq!(e.mem.u8(m + 0xc), 3);
        assert!(get(&mut e, 0x0043_8ef0));
        e.mem.set_u8(m + 0xc, 0xff);
        assert!(get(&mut e, 0x0043_8f10));
        e.call(0x0044_de40, &[m, 0xff00]);
        assert_eq!(e.mem.u8(m + 0xc), 0xfe);
        e.call(0x0044_de80, &[m, 0]);
        assert_eq!(e.mem.u8(m + 0xc), 0xfc);
    }
}
