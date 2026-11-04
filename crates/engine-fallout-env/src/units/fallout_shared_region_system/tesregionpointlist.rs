//! `fallout shared/region system/tesregionpointlist.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006815c0` (`thiscall`): a no-op constructor step (answers `this`); the
/// vector constructor below runs it on each point.
const TRIVIAL_CONSTRUCT: u32 = 0x0068_15c0;
/// `TESFile::GetTESChunk` (`004726b0`), the sized `GetChunkData`
/// (`00472890`, `RET 8`), the chunk size (`00401660`), whether form ids need
/// fixing up (`00401680`) and the file's name (`00891170`).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_FIXES_IDS: u32 = 0x0040_1680;
const FILE_NAME: u32 = 0x0089_1170;
/// `00401080` (`cdecl`: a word's address, a byte): the fix-up `LoadPoints`
/// applies to each float of each point.
const FIX_UP_WORD: u32 = 0x0040_1080;
/// `00401000` / `00401030`: allocate / free (`cdecl`).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// `` `vector constructor iterator' `` (`00401050`, `cdecl`: block, element
/// size, count, constructor).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
/// `005b5e40` (`cdecl`, variable): prints a warning; the format at
/// `01024180` takes the file's name.
const PRINT_WARNING: u32 = 0x005b_5e40;
const BAD_SIZE_FORMAT: u32 = 0x0102_4180;
/// `006a7f50` / `006b9130` (`thiscall`): a point's x / y (x87 `ST0`).
const POINT_X: u32 = 0x006a_7f50;
const POINT_Y: u32 = 0x006b_9130;
/// `005ae3d0` (`thiscall`, `RET 4`, the item's address): inserts into the
/// list.
const LIST_INSERT: u32 = 0x005a_e3d0;
/// The chunk tag `RPLD`.
const CHUNK_RPLD: u32 = 0x444c_5052;

/// The x87 condition bits (`AH` after `FNSTSW AX`: C0 0x01, C2 0x04, C3
/// 0x40) of comparing `st0` with `source`.
fn fcom_status(st0: f64, source: f64) -> u8 {
    if st0.is_nan() || source.is_nan() {
        0x45
    } else if st0 < source {
        0x01
    } else if st0 == source {
        0x40
    } else {
        0
    }
}

// Translated from 004f7000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a region point copied from another
/// (`thiscall`, `RET 4`): `006815c0`, then both words of the source. Answers
/// `this`.
pub fn fn_004f7000(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(TRIVIAL_CONSTRUCT, &args![this]);
    let x = e.mem.u32(source.addr());
    let y = e.mem.u32(source.addr().wrapping_add(4));
    e.mem.set_u32(this.addr(), x);
    e.mem.set_u32(this.addr().wrapping_add(4), y);
    this
}

// Translated from 004f79f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionPointList::LoadPoints` (engine map name), `thiscall`, `RET 4`:
/// the file. False without a file, for a chunk other than `RPLD`, for a size
/// not a multiple of 8 (with a warning naming the file) and for an empty
/// chunk. Otherwise the points are read into a temporary array (`count * 8`
/// bytes, `MUL` overflow giving `0xffffffff`, each point constructed by the
/// vector constructor), the count (+0x24) restarts at 0, and each point,
/// its two words fixed up when `00401680` says so, is copied into a new
/// region point (`004f7000`; null when the allocation failed) that widens
/// the bounds (+0x10, +0x14 minimum x and y; +0x18, +0x1c maximum, each
/// stored from a second getter call) and goes into the list (`005ae3d0`),
/// the count growing by one. The array is freed; answers true.
// Translation incomplete: 004f79f0 - the SEH frame (FS:[0], unwind of the two allocations) is not translated.
pub fn tes_region_point_list_load_points(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if file.is_null() || e.call(GET_TES_CHUNK, &args![file]).u32() != CHUNK_RPLD {
        return false;
    }
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    let count = size >> 3;
    if size % 8 != 0 {
        let name = e.call(FILE_NAME, &args![file]).u32();
        e.call(PRINT_WARNING, &args![BAD_SIZE_FORMAT, name]);
        return false;
    }
    if size == 0 || count == 0 {
        return false;
    }
    let bytes = count.checked_mul(8).unwrap_or(0xffff_ffff);
    let block = e.call(ALLOCATE, &args![bytes]).u32();
    let points = if block != 0 {
        e.call(
            VECTOR_CONSTRUCT,
            &args![block, 8u32, count, TRIVIAL_CONSTRUCT],
        );
        block
    } else {
        0
    };
    e.call(GET_CHUNK_DATA_SIZED, &args![file, points, size]);
    e.mem.set_u32(base.wrapping_add(0x24), 0);
    let mut index: u32 = 0;
    while index < count {
        let at = points.wrapping_add(index.wrapping_mul(8));
        if e.call(FILE_FIXES_IDS, &args![file]).bool() {
            e.call(FIX_UP_WORD, &args![at, 0u32]);
            e.call(FIX_UP_WORD, &args![at.wrapping_add(4), 0u32]);
        }
        let node = e.call(ALLOCATE, &args![8u32]).u32();
        let point = if node != 0 {
            fn_004f7000(e, Ptr::new(node), Ptr::new(at)).addr()
        } else {
            0
        };
        // Minimum x and y: stored when the bound is above the point's.
        for (offset, getter) in [(0x10u32, POINT_X), (0x14, POINT_Y)] {
            let value = e.call(getter, &args![point]).f64();
            let bound = x87_load32(e.mem.u32(base.wrapping_add(offset)));
            if fcom_status(bound, value) & 0x41 == 0 {
                let value = e.call(getter, &args![point]).f64();
                e.mem.set_u32(base.wrapping_add(offset), x87_store32(value));
            }
        }
        // Maximum x and y: stored when the bound is below the point's.
        for (offset, getter) in [(0x18u32, POINT_X), (0x1c, POINT_Y)] {
            let value = e.call(getter, &args![point]).f64();
            let bound = x87_load32(e.mem.u32(base.wrapping_add(offset)));
            if (fcom_status(bound, value) & 0x05).count_ones() % 2 == 1 {
                let value = e.call(getter, &args![point]).f64();
                e.mem.set_u32(base.wrapping_add(offset), x87_store32(value));
            }
        }
        e.with_stack(4, |e, item| {
            e.mem.set_u32(item.addr(), point);
            e.call(LIST_INSERT, &args![this, item]);
        });
        let total = e.mem.u32(base.wrapping_add(0x24));
        e.mem
            .set_u32(base.wrapping_add(0x24), total.wrapping_add(1));
        index = index.wrapping_add(1);
    }
    e.call(FREE, &args![points]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_7000, fn_004f7000(Ptr, Ptr) -> Ptr),
        entry!(0x004f_79f0, tes_region_point_list_load_points(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod b025_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_004f7000_copies_both_words() {
        let mut e = Engine::new();
        e.register(TRIVIAL_CONSTRUCT, |_, a| a[0].into_ret());
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 0x7f80_0001);
        e.mem.set_u32(source + 4, 0x1234);
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x004f_7000, &args![this, source]).u32(), this);
        assert_eq!(
            (e.mem.u32(this), e.mem.u32(this + 4)),
            (0x7f80_0001, 0x1234)
        );
    }

    type Chunk = Rc<RefCell<(u32, Vec<u8>)>>;

    fn engine() -> (Engine, Chunk) {
        let mut e = Engine::new();
        let chunk: Chunk = Rc::new(RefCell::new((CHUNK_RPLD, vec![])));
        let c = chunk.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| c.borrow().0.into_ret());
        let c = chunk.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            (c.borrow().1.len() as u32).into_ret()
        });
        let c = chunk.clone();
        e.register_double(GET_CHUNK_DATA_SIZED, move |e, a| {
            let data = c.borrow().1.clone();
            e.mem.write(a[1], &data[..(a[2] as usize).min(data.len())]);
            Ret::default()
        });
        e.register(FILE_FIXES_IDS, |_, _| 0u32.into_ret());
        e.register(FILE_NAME, |_, _| 0x5555u32.into_ret());
        e.register(PRINT_WARNING, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FREE, |_, _| Ret::default());
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.register(TRIVIAL_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(FIX_UP_WORD, |_, _| Ret::default());
        e.register(POINT_X, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0])),
            ..Ret::default()
        });
        e.register(POINT_Y, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 4)),
            ..Ret::default()
        });
        e.register(LIST_INSERT, |_, _| Ret::default());
        (e, chunk)
    }

    fn points(values: &[(f32, f32)]) -> Vec<u8> {
        values
            .iter()
            .flat_map(|(x, y)| x.to_le_bytes().into_iter().chain(y.to_le_bytes()))
            .collect()
    }

    #[test]
    fn load_points_reads_points_and_bounds() {
        let (mut e, chunk) = engine();
        let this = e.mem.alloc(0x28);
        e.mem.set_f32(this + 0x10, 100.0);
        e.mem.set_f32(this + 0x14, 100.0);
        e.mem.set_f32(this + 0x18, -100.0);
        e.mem.set_f32(this + 0x1c, -100.0);
        e.mem.set_u32(this + 0x24, 9);
        chunk.borrow_mut().1 = points(&[(1.0, 5.0), (-3.0, 2.0), (4.0, -1.0)]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_79f0, &args![this, 0x9000u32]).bool());
        assert_eq!(e.mem.u32(this + 0x24), 3);
        assert_eq!(
            [0x10, 0x14, 0x18, 0x1c].map(|o| e.mem.f32(this + o)),
            [-3.0, -1.0, 4.0, 5.0]
        );
        assert_eq!(calls_to(&e, LIST_INSERT).len(), 3);
        let array = calls_to(&e, ALLOCATE)[0].clone();
        assert_eq!(array, vec![0x18]);
        let construct = calls_to(&e, VECTOR_CONSTRUCT);
        assert_eq!(&construct[0][1..], [8, 3, TRIVIAL_CONSTRUCT]);
        assert_eq!(calls_to(&e, FREE), vec![vec![construct[0][0]]]);
        assert!(calls_to(&e, FIX_UP_WORD).is_empty());
        // With fix-ups: two per point.
        e.register(FILE_FIXES_IDS, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_79f0, &args![this, 0x9000u32]).bool());
        assert_eq!(calls_to(&e, FIX_UP_WORD).len(), 6);
    }

    #[test]
    fn load_points_refuses_bad_chunks() {
        let (mut e, chunk) = engine();
        let this = e.mem.alloc(0x28);
        assert!(!e.call(0x004f_79f0, &args![this, 0u32]).bool());
        chunk.borrow_mut().0 = 0x1234;
        assert!(!e.call(0x004f_79f0, &args![this, 0x9000u32]).bool());
        chunk.borrow_mut().0 = CHUNK_RPLD;
        chunk.borrow_mut().1 = vec![0; 12];
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_79f0, &args![this, 0x9000u32]).bool());
        assert_eq!(
            calls_to(&e, PRINT_WARNING),
            vec![vec![BAD_SIZE_FORMAT, 0x5555]]
        );
        chunk.borrow_mut().1 = vec![];
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_79f0, &args![this, 0x9000u32]).bool());
        assert!(calls_to(&e, ALLOCATE).is_empty());
    }
}
