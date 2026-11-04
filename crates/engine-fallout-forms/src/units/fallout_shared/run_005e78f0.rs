//! `fallout shared/run_005e78f0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e7a90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e7a90 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x30 (0x40 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005e7a90(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0047_9790, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0104_664c);
    e.mem.set_u32(base + 0x30, 0x0104_6638);
    e.call(0x004f_15a0, &args![this, 0x30]);
    this
}

// Translated from 005e7a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x24 (`SUB ECX,0x24`) and jumps to `005e7370`. `005e7370` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_005e7a80(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x005e_7370, &args![this.addr().wrapping_sub(0x24), flags])
        .u32()
}

// Translated from 005e7b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005e7b40` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e7b10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_7b40, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `TESForm::LoadForm` (`00485110`, `RET 4`).
const LOAD_FORM: u32 = 0x0048_5110;
/// `00484ab0` (`thiscall`, `RET 4`).
const FORM_SET_FLAG: u32 = 0x0048_4ab0;
/// `004013e0` (`thiscall`): a `bool` that makes `InitItem` do nothing.
const FORM_SKIPS_INIT: u32 = 0x0040_13e0;
/// `BGSIdleCollection::InitItem` (`00479b00`, `RET 4`).
const IDLE_COLLECTION_INIT_ITEM: u32 = 0x0047_9b00;
/// `004797c0`: the `BGSIdleCollection` destructor body.
const IDLE_COLLECTION_DESTRUCT: u32 = 0x0047_97c0;
/// `0050e9f0`: the form base destructor.
const FORM_BASE_DESTRUCT: u32 = 0x0050_e9f0;
/// `BGSIdleMarker`'s vtable (RTTI `.?AVBGSIdleMarker@@`) and its idle
/// collection's (+0x30).
const IDLE_MARKER_VTABLE: u32 = 0x0104_664c;
const IDLE_MARKER_COLLECTION_VTABLE: u32 = 0x0104_6638;
/// `TESFile` readers: `GetTESChunk` (`004726b0`), next chunk (`004726f0`),
/// `GetChunkData` (`004727f0`, `RET 4`), the sized overload (`00472890`,
/// `RET 8`), the chunk size (`00401660`) and whether ids need fixing up
/// (`00401680`).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
const NEXT_CHUNK: u32 = 0x0047_26f0;
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_FIXES_IDS: u32 = 0x0040_1680;
/// `00401080` (`cdecl`: a form id's address, a byte): fixes up one form id.
const FIX_UP_FORM_ID: u32 = 0x0040_1080;
/// `00401000` / `00401030`: allocate / free (`cdecl`).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// `00403d30` (`cdecl`: address, byte, count): fills memory.
const FILL_MEMORY: u32 = 0x0040_3d30;
/// The `PTDT` helpers: `0067ff10` fixes the 16-byte data up, `0067ffa0`
/// (`RET 4`) builds a second 16-byte local from it, `00483710` is then called
/// on that local.
const FIX_UP_TARGET_DATA: u32 = 0x0067_ff10;
const BUILD_TARGET: u32 = 0x0067_ffa0;
const TARGET_FINISH: u32 = 0x0048_3710;
/// The form vtable slots (byte offsets) the load calls: `0x134` with the
/// editor id read, `0xe0` with the file for `OBND`.
const SET_EDITOR_ID_SLOT: u32 = 0x134;
const LOAD_BOUNDS_SLOT: u32 = 0xe0;
/// Chunk tags.
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_IDLA: u32 = 0x414c_4449;
const CHUNK_IDLC: u32 = 0x434c_4449;
const CHUNK_IDLF: u32 = 0x464c_4449;
const CHUNK_IDLT: u32 = 0x544c_4449;
const CHUNK_PTDT: u32 = 0x5444_5450;

// Translated from 005e7a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether bit 0 of the byte at `this + 0x44` is
/// set (`MOVSX`, `AND 1`, `SETNZ`). The class is unconfirmed.
pub fn fn_005e7a60(e: &mut Engine, this: Ptr) -> bool {
    i32::from(e.mem.i8(this.addr().wrapping_add(0x44))) & 1 != 0
}

// Translated from 005e7b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSIdleMarker` destructor body (unnamed in the engine map; its
/// vtables by RTTI): sets both vtables, destroys the idle collection (+0x30,
/// `004797c0`), then the form base (`0050e9f0`).
// Translation incomplete: 005e7b40 - the SEH frame (FS:[0], unwind of the bases) is not translated.
pub fn fn_005e7b40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, IDLE_MARKER_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x30), IDLE_MARKER_COLLECTION_VTABLE);
    e.call(IDLE_COLLECTION_DESTRUCT, &args![base.wrapping_add(0x30)]);
    e.call(FORM_BASE_DESTRUCT, &args![this]);
}

// Translated from 005e7bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSIdleMarker`'s `InitItem` (unnamed in the engine map): unless
/// `004013e0` answers true, `BGSIdleCollection::InitItem` on the collection
/// (+0x30) with the marker, then `00484ab0` with 1.
pub fn fn_005e7bb0(e: &mut Engine, this: Ptr) {
    if e.call(FORM_SKIPS_INIT, &args![this]).bool() {
        return;
    }
    e.call(
        IDLE_COLLECTION_INIT_ITEM,
        &args![this.addr().wrapping_add(0x30), this],
    );
    e.call(FORM_SET_FLAG, &args![this, 1u32]);
}

// Translated from 005e7bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSIdleMarker`'s load (unnamed in the engine map), `thiscall`, `RET 4`:
/// the file; answers true. `TESForm::LoadForm`, `00484ab0` with 0, then each
/// chunk until the tag is 0 or the record ends: `EDID` (at most 0x200 bytes
/// into a stack block of the chunk's size, `__alloca_probe_16`, then vtable
/// `+0x134`), `OBND` (vtable `+0xe0`), and the idle collection's own chunks
/// read at the marker's offsets: `IDLF` (+0x34), `IDLC` (+0x35, one byte or
/// a word's low byte; a non-zero count frees the old array at +0x38 and
/// allocates a cleared `count * 4`), `IDLA` (the ids when the chunk has
/// exactly `count * 4` bytes, each fixed up by `00401080` when `00401680`
/// says so), `IDLT` (+0x3c); `PTDT` reads 16 cleared bytes (fixed up by
/// `0067ff10` when asked), builds a second local from them (`0067ffa0`) and
/// calls `00483710` on it.
// Translation incomplete: 005e7bf0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_005e7bf0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.call(LOAD_FORM, &args![this, file]);
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    loop {
        let tag = e.call(GET_TES_CHUNK, &args![file]).u32();
        if tag == 0 {
            break;
        }
        match tag {
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size.wrapping_add(15) & !15, |e, block| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, block, 0x200u32]);
                    e.vcall(base, SET_EDITOR_ID_SLOT, &args![block]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, LOAD_BOUNDS_SLOT, &args![file]);
            }
            CHUNK_IDLF => {
                e.call(
                    GET_CHUNK_DATA_SIZED,
                    &args![file, base.wrapping_add(0x34), 1u32],
                );
            }
            CHUNK_IDLC => {
                if e.call(CHUNK_SIZE, &args![file]).u32() == 1 {
                    e.call(
                        GET_CHUNK_DATA_SIZED,
                        &args![file, base.wrapping_add(0x35), 1u32],
                    );
                } else {
                    e.with_stack(4, |e, word| {
                        e.mem.set_u32(word.addr(), 0);
                        e.call(GET_CHUNK_DATA, &args![file, word]);
                        let count = e.mem.u8(word.addr());
                        e.mem.set_u8(base.wrapping_add(0x35), count);
                    });
                }
                if e.mem.u8(base.wrapping_add(0x35)) != 0 {
                    let old = e.mem.u32(base.wrapping_add(0x38));
                    if old != 0 {
                        e.call(FREE, &args![old]);
                    }
                    let count = u32::from(e.mem.u8(base.wrapping_add(0x35)));
                    let size = count.checked_mul(4).unwrap_or(0xffff_ffff);
                    let array = e.call(ALLOCATE, &args![size]).u32();
                    e.mem.set_u32(base.wrapping_add(0x38), array);
                    let bytes = u32::from(e.mem.u8(base.wrapping_add(0x35))) << 2;
                    let array = e.mem.u32(base.wrapping_add(0x38));
                    e.call(FILL_MEMORY, &args![array, 0u32, bytes]);
                }
            }
            CHUNK_IDLA => {
                if e.mem.u32(base.wrapping_add(0x38)) != 0 {
                    let bytes = u32::from(e.mem.u8(base.wrapping_add(0x35))) << 2;
                    if e.call(CHUNK_SIZE, &args![file]).u32() == bytes {
                        let array = e.mem.u32(base.wrapping_add(0x38));
                        e.call(GET_CHUNK_DATA_SIZED, &args![file, array, bytes]);
                        if e.call(FILE_FIXES_IDS, &args![file]).bool() {
                            let mut index: i32 = 0;
                            while index < i32::from(e.mem.u8(base.wrapping_add(0x35))) {
                                let slot = e
                                    .mem
                                    .u32(base.wrapping_add(0x38))
                                    .wrapping_add((index as u32).wrapping_mul(4));
                                e.call(FIX_UP_FORM_ID, &args![slot, 0u32]);
                                index = index.wrapping_add(1);
                            }
                        }
                    }
                }
            }
            CHUNK_IDLT => {
                e.call(GET_CHUNK_DATA, &args![file, base.wrapping_add(0x3c)]);
            }
            CHUNK_PTDT => {
                e.with_stack(0x20, |e, locals| {
                    let target = locals.addr();
                    let data = target + 0x10;
                    e.call(FILL_MEMORY, &args![data, 0u32, 0x10u32]);
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, data, 0x10u32]);
                    if e.call(FILE_FIXES_IDS, &args![file]).bool() {
                        e.call(FIX_UP_TARGET_DATA, &args![data]);
                    }
                    e.call(BUILD_TARGET, &args![target, data]);
                    e.call(TARGET_FINISH, &args![target]);
                });
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// `__RTDynamicCast` (`00ec43fb`, cdecl), the source type descriptor and
/// the target `005e78f0` and `005e79a0` pass (`011863b8`, the type
/// descriptor named `.?AVBGSHeadPart@@`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE: u32 = 0x0118_3028;
const CAST_TARGET: u32 = 0x0118_63b8;
/// The form's list (`00624700`, thiscall) and the list helpers: empty it
/// (`00470470`), whether a node is empty (`008256d0`), a node's item slot
/// (`006815c0`) and the next node (`00726070`).
const FORM_LIST: u32 = 0x0062_4700;
const LIST_CLEAR: u32 = 0x0047_0470;
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_ITEM: u32 = 0x0068_15c0;
const LIST_NEXT: u32 = 0x0072_6070;

// Translated from 005e78f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSHeadPart`'s copy (`RET 4`: the source form; the engine map has no name):
/// the source through `__RTDynamicCast(source, 0, 01183028, 011863b8, 0)`;
/// nothing without it. Otherwise `TESForm::CopyAllComponents` given the
/// uncast source, the byte at +0x44, then this form's list (`00624700`)
/// emptied and each item of the source's list, up to an empty node,
/// added with `005e7790` (the next node is taken before the add).
pub fn fn_005e78f0(e: &mut Engine, this: Ptr, source: u32) {
    let base = this.addr();
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, CAST_SOURCE, CAST_TARGET, 0u32],
        )
        .u32();
    if other == 0 {
        return;
    }
    e.call(0x0048_51b0, &args![this, source]);
    let byte = e.mem.u8(other + 0x44);
    e.mem.set_u8(base + 0x44, byte);
    let list = e.call(FORM_LIST, &args![this]).u32();
    e.call(LIST_CLEAR, &args![list]);
    let mut node = e.call(FORM_LIST, &args![other]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_ITEM, &args![node]).u32();
        let item = e.mem.u32(slot);
        node = e.call(LIST_NEXT, &args![node]).u32();
        e.call(0x005e_7790, &args![this, item]);
    }
}

// Translated from 005e79a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSHeadPart`'s compare (`RET 4`: the other form; true when they differ; the
/// engine map has no name): true when the cast as in `005e78f0` fails,
/// `TESForm::CompareAllComponents` answers true or the bytes at +0x44
/// differ; then for each item of the other's list (`00624700`), up to an
/// empty node, true when this form's list does not hold it (`005f65d0`,
/// given the item's address on the stack). False otherwise.
pub fn fn_005e79a0(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let base = this.addr();
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, CAST_SOURCE, CAST_TARGET, 0u32],
        )
        .u32();
    if other == 0 {
        return true;
    }
    if e.call(0x0048_5270, &args![this, other]).bool() {
        return true;
    }
    if e.mem.i8(other + 0x44) != e.mem.i8(base + 0x44) {
        return true;
    }
    let mut node = e.call(FORM_LIST, &args![other]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_ITEM, &args![node]).u32();
        let item = e.mem.u32(slot);
        node = e.call(LIST_NEXT, &args![node]).u32();
        let held = e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), item);
            let list = e.call(FORM_LIST, &args![this]).u32();
            e.call(0x005f_65d0, &args![list, local]).bool()
        });
        if !held {
            return true;
        }
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_78f0, fn_005e78f0(Ptr, u32)),
        entry!(0x005e_79a0, fn_005e79a0(Ptr, u32) -> bool),
        entry!(0x005e_7a90, fn_005e7a90(Ptr) -> Ptr),
        entry!(0x005e_7a80, fn_005e7a80(Ptr, u32) -> u32),
        entry!(0x005e_7b10, fn_005e7b10(Ptr, u32) -> Ptr),
        entry!(0x005e_7a60, fn_005e7a60(Ptr) -> bool),
        entry!(0x005e_7b40, fn_005e7b40(Ptr)),
        entry!(0x005e_7bb0, fn_005e7bb0(Ptr)),
        entry!(0x005e_7bf0, fn_005e7bf0(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005e7a90_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005e_7a90,
            size: 0x40,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0047_9790, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x30)]),
            ],
            stores: &[(0x0, 4, 0x0104_664c), (0x30, 4, 0x0104_6638)],
        });
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_005e7a80() {
        assert_eq!(
            check(0x005e_7a80, 0x005e_7370, 0x24, &[0x1]).u32(),
            0xdead_beef
        );
    }
    #[test]
    fn at_005e7b10_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_7b40, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_7b10, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_7b10)
                .collect();
            let mut expected = vec![(0x005e_7b40, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b025_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FORM_VTABLE: u32 = 0x00ab_0000;
    const EDITOR_ID_FN: u32 = 0x00ab_1000;
    const BOUNDS_FN: u32 = 0x00ab_1004;

    type Chunks = Rc<RefCell<(usize, Vec<(u32, Vec<u8>)>)>>;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine(chunks: Vec<(u32, Vec<u8>)>) -> Engine {
        let mut e = Engine::new();
        let state: Chunks = Rc::new(RefCell::new((0, chunks)));
        let s = state.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            let s = s.borrow();
            s.1.get(s.0).map_or(0, |c| c.0).into_ret()
        });
        let s = state.clone();
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut s = s.borrow_mut();
            s.0 += 1;
            u32::from(s.0 < s.1.len()).into_ret()
        });
        let s = state.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            let s = s.borrow();
            (s.1[s.0].1.len() as u32).into_ret()
        });
        let s = state.clone();
        e.register_double(GET_CHUNK_DATA_SIZED, move |e, a| {
            let data = s.borrow().1[s.borrow().0].1.clone();
            e.mem.write(a[1], &data[..data.len().min(a[2] as usize)]);
            Ret::default()
        });
        let s = state;
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let data = s.borrow().1[s.borrow().0].1.clone();
            e.mem.write(a[1], &data);
            Ret::default()
        });
        e.register(FILL_MEMORY, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FILE_FIXES_IDS, |_, _| 1u32.into_ret());
        for addr in [
            LOAD_FORM,
            FORM_SET_FLAG,
            FREE,
            FIX_UP_FORM_ID,
            FIX_UP_TARGET_DATA,
            BUILD_TARGET,
            TARGET_FINISH,
            EDITOR_ID_FN,
            BOUNDS_FN,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let mut slots = vec![0u32; 0x4e];
        slots[0x134 / 4] = EDITOR_ID_FN;
        slots[0xe0 / 4] = BOUNDS_FN;
        e.put_vtable(FORM_VTABLE, &slots);
        e
    }

    #[test]
    fn fn_005e7bf0_loads_the_marker_and_its_idles() {
        let mut e = engine(vec![
            (CHUNK_EDID, b"Marker\0".to_vec()),
            (CHUNK_OBND, vec![0; 12]),
            (CHUNK_IDLF, vec![1]),
            (CHUNK_IDLC, vec![2, 0, 0, 0]),
            (CHUNK_IDLA, vec![7; 8]),
            (CHUNK_IDLT, 1.5f32.to_bits().to_le_bytes().to_vec()),
            (CHUNK_PTDT, vec![9; 16]),
        ]);
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this, FORM_VTABLE);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_7bf0, &args![this, 0x9000u32]).bool());
        assert_eq!(calls_to(&e, LOAD_FORM), vec![vec![this, 0x9000]]);
        assert_eq!(calls_to(&e, FORM_SET_FLAG), vec![vec![this, 0]]);
        assert_eq!(calls_to(&e, EDITOR_ID_FN).len(), 1);
        assert_eq!(calls_to(&e, BOUNDS_FN), vec![vec![this, 0x9000]]);
        assert_eq!(e.mem.u8(this + 0x34), 1);
        assert_eq!(e.mem.u8(this + 0x35), 2);
        let array = e.mem.u32(this + 0x38);
        assert_eq!(e.mem.u32(array + 4), 0x0707_0707);
        assert_eq!(
            calls_to(&e, FIX_UP_FORM_ID),
            vec![vec![array, 0], vec![array + 4, 0]]
        );
        assert_eq!(e.mem.f32(this + 0x3c), 1.5);
        let build = calls_to(&e, BUILD_TARGET);
        assert_eq!(build[0][1], build[0][0] + 0x10);
        assert_eq!(calls_to(&e, FIX_UP_TARGET_DATA), vec![vec![build[0][1]]]);
        assert_eq!(calls_to(&e, TARGET_FINISH), vec![vec![build[0][0]]]);
        // A one-byte count over an old array frees it; a zero tag stops.
        let mut e = engine(vec![
            (CHUNK_IDLC, vec![1]),
            (0, vec![]),
            (CHUNK_IDLF, vec![5]),
        ]);
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this, FORM_VTABLE);
        e.mem.set_u8(this + 0x34, 1);
        e.mem.set_u32(this + 0x38, 0x4444);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_7bf0, &args![this, 0x9000u32]).bool());
        assert_eq!(calls_to(&e, FREE), vec![vec![0x4444]]);
        assert_eq!(e.mem.u8(this + 0x34), 1);
    }

    #[test]
    fn fn_005e7a60_tests_bit_0_at_0x44() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x48);
        e.mem.set_u8(this + 0x44, 0x81);
        assert!(e.call(0x005e_7a60, &args![this]).bool());
        e.mem.set_u8(this + 0x44, 0x80);
        assert!(!e.call(0x005e_7a60, &args![this]).bool());
    }

    #[test]
    fn idle_marker_destructor_and_init_item() {
        let mut e = Engine::new();
        for addr in [
            IDLE_COLLECTION_DESTRUCT,
            FORM_BASE_DESTRUCT,
            IDLE_COLLECTION_INIT_ITEM,
            FORM_SET_FLAG,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(FORM_SKIPS_INIT, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        e.call(0x005e_7b40, &args![this]);
        assert_eq!(
            [e.mem.u32(this), e.mem.u32(this + 0x30)],
            [IDLE_MARKER_VTABLE, IDLE_MARKER_COLLECTION_VTABLE]
        );
        assert_eq!(
            calls_to(&e, IDLE_COLLECTION_DESTRUCT),
            vec![vec![this + 0x30]]
        );
        assert_eq!(calls_to(&e, FORM_BASE_DESTRUCT), vec![vec![this]]);
        e.call(0x005e_7bb0, &args![this]);
        assert_eq!(
            calls_to(&e, IDLE_COLLECTION_INIT_ITEM),
            vec![vec![this + 0x30, this]]
        );
        assert_eq!(calls_to(&e, FORM_SET_FLAG), vec![vec![this, 1]]);
        e.register(FORM_SKIPS_INIT, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x005e_7bb0, &args![this]);
        assert!(calls_to(&e, IDLE_COLLECTION_INIT_ITEM).is_empty());
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Forms whose list head is at +0x48 (nodes `{item, next}`; a node is
    /// empty when its item is 0).
    fn doubles(e: &mut Engine) {
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(FORM_LIST, |_, a| ret(a[0] + 0x48));
        e.register(LIST_IS_EMPTY, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(LIST_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
    }

    fn two_items(e: &mut Engine, form: u32, a: u32, b: u32) {
        let second = e.mem.alloc(8);
        e.mem.set_u32(form + 0x48, a);
        e.mem.set_u32(form + 0x4c, second);
        e.mem.set_u32(second, b);
    }

    #[test]
    fn fn_005e78f0_copies_the_byte_and_the_list() {
        let mut e = Engine::new();
        doubles(&mut e);
        for a in [0x0048_51b0u32, LIST_CLEAR, 0x005e_7790] {
            e.register(a, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x50);
        let other = e.mem.alloc(0x50);
        e.mem.set_u8(other + 0x44, 9);
        two_items(&mut e, other, 0x11, 0x22);
        e.call_log = Some(vec![]);
        e.call(0x005e_78f0, &args![this, other]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![other, 0, CAST_SOURCE, CAST_TARGET, 0]]
        );
        assert_eq!(calls(&e, 0x0048_51b0), vec![vec![this, other]]);
        assert_eq!(e.mem.u8(this + 0x44), 9);
        assert_eq!(calls(&e, LIST_CLEAR), vec![vec![this + 0x48]]);
        assert_eq!(
            calls(&e, 0x005e_7790),
            vec![vec![this, 0x11], vec![this, 0x22]]
        );
        e.register(DYNAMIC_CAST, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x005e_78f0, &args![this, other]);
        assert!(calls(&e, 0x0048_51b0).is_empty());
    }

    #[test]
    fn fn_005e79a0_checks_each_item_is_held() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(0x0048_5270, |_, _| ret(0));
        // Held when the item is 0x11 or 0x22.
        e.register(0x005f_65d0, |e, a| {
            let item = e.mem.u32(a[1]);
            ret(u32::from(item == 0x11 || item == 0x22))
        });
        let this = e.mem.alloc(0x50);
        let other = e.mem.alloc(0x50);
        two_items(&mut e, other, 0x11, 0x22);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005e_79a0, &args![this, other]).bool());
        let held = calls(&e, 0x005f_65d0);
        assert_eq!(held.len(), 2);
        assert!(held.iter().all(|c| c[0] == this + 0x48));
        two_items(&mut e, other, 0x11, 0x33);
        assert!(e.call(0x005e_79a0, &args![this, other]).bool());
        e.mem.set_u32(other + 0x48, 0);
        assert!(!e.call(0x005e_79a0, &args![this, other]).bool());
        e.mem.set_u8(other + 0x44, 1);
        assert!(e.call(0x005e_79a0, &args![this, other]).bool());
        e.mem.set_u8(other + 0x44, 0);
        e.register(0x0048_5270, |_, _| ret(1));
        assert!(e.call(0x005e_79a0, &args![this, other]).bool());
        assert!(e.call(0x005e_79a0, &args![this, 0u32]).bool());
    }
}
