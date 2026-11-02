//! `BSMain/bsbound.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a7b3b0` (named `NiExtraData::LinkObject` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00A7B3B0: u32 = 0x00a7_b3b0;

// Translated from 00c54e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSBound` vtable word 010c2bb4, right after
/// `BSBound::LoadBinary` (010c2bb0). Forwards `this` and the stream to `00a7b3b0`
/// (`NiExtraData::LinkObject`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c54e70(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00A7B3B0, &args![this, stream]);
}

// Translated from 00c55320 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012031e8 without reading the receiver. The map names this body `BSBound::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c55320(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_31e8
}

/// `NiExtraData::LoadBinary` (`00a7b4a0`, engine map; thiscall, `RET 4`: the stream).
const EXTRA_DATA_LOAD_BINARY: u32 = 0x00a7_b4a0;
/// `00a59be0` (named `NiColor::LoadBinary` in the engine map; thiscall on three floats,
/// `RET 4`: the stream).
const THREE_FLOATS_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `NiNew` allocation (`00aa13e0`, cdecl: size).
const NI_NEW: u32 = 0x00aa_13e0;
/// The `NiExtraData` constructor taking a name (`00a7b440`, thiscall, `RET 4`).
const EXTRA_DATA_CONSTRUCTOR: u32 = 0x00a7_b440;
/// The name setter `00a5b950` (unit `NiMain/niobjectnet.cpp`; thiscall, `RET 4`).
const SET_NAME: u32 = 0x00a5_b950;
/// The word whose address `BSBound::CreateObject` passes to `00a5b950`.
const CREATED_NAME: u32 = 0x011f_49e0;
/// `NiExtraData` destructor body (`00a7b300`, thiscall).
const EXTRA_DATA_DESTRUCTOR: u32 = 0x00a7_b300;
/// `NiDelete` of a sized block (`00aa1460`, cdecl: the block, its size).
const NI_DELETE: u32 = 0x00aa_1460;
/// `NiObjectNET::GetExtraData` (`00a5bdd0`, engine map; thiscall, `RET 4`: the name).
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
/// `BSBound` vtable (RTTI).
const VTABLE_BS_BOUND: u32 = 0x010c_2b64;
/// `BSBound::ms_RTTI` (the address `00c55320` returns).
const BS_BOUND_RTTI: u32 = 0x0120_31e8;
/// The global holding `BSBound`'s extra-data name (an allocated `NiFixedString`).
const BS_BOUND_NAME: u32 = 0x0120_31e4;
/// `"BBX"`.
const BS_BOUND_NAME_TEXT: u32 = 0x010c_2c00;
/// The three words every new bound's center and extents start from.
const ZERO_POINT: u32 = 0x011f_426c;
/// The `MemoryManager` singleton.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager` allocation (`00aa3e40`, thiscall on the singleton: size).
const ALLOCATE: u32 = 0x00aa_3e40;
/// `MemoryManager` deallocation (`00aa4060`, thiscall on the singleton: the block).
const DEALLOCATE: u32 = 0x00aa_4060;
/// The `NiFixedString` maker (`00a5b690`, cdecl: the text; returns the handle).
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
/// The empty `NiFixedString` handle.
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
/// `InterlockedDecrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00c54e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBound::LoadBinary` (engine map): the `NiExtraData` part (`00a7b4a0`), then the center
/// (+0xc) and the extents (+0x18), three floats each (`00a59be0`).
pub fn bs_bound_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    e.call(EXTRA_DATA_LOAD_BINARY, &args![base, stream]);
    e.call(THREE_FLOATS_LOAD_BINARY, &args![base + 0xc, stream]);
    e.call(THREE_FLOATS_LOAD_BINARY, &args![base + 0x18, stream]);
}

// Translated from 00c55010 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c55010 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f5ff, state variable) is not translated
/// The `BSBound` factory (no arguments): a 0x24-byte `NiNew` block constructed as an
/// `NiExtraData` named by the `BSBound` name global (`00a7b440`), the `BSBound` vtable, and
/// the center (+0xc) and extents (+0x18) copied as words from `011f426c`; null when the
/// allocation fails. Then `00a5b950` runs on the result (also when it is null) with
/// `011f49e0`. Returns the result.
pub fn fn_00c55010(e: &mut Engine) -> u32 {
    let block = e.call(NI_NEW, &args![0x24u32]).u32();
    let object = if block == 0 {
        0
    } else {
        let name = e.global::<u32>(BS_BOUND_NAME);
        e.call(EXTRA_DATA_CONSTRUCTOR, &args![block, name]);
        e.mem.set_u32(block, VTABLE_BS_BOUND);
        for start in [0x0cu32, 0x18] {
            for i in 0..3u32 {
                let word = e.mem.u32(ZERO_POINT + 4 * i);
                e.mem.set_u32(block + start + 4 * i, word);
            }
        }
        block
    };
    e.call(SET_NAME, &args![object, CREATED_NAME]);
    object
}

// Translated from 00c55170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object's `BSBound` extra data: null for a null object; otherwise the extra data named
/// by the `BSBound` name global (`NiObjectNET::GetExtraData`), kept only when its RTTI chain
/// (the virtual at +8, then each record's parent at +4) reaches `BSBound`'s (`012031e8`).
pub fn fn_00c55170(e: &mut Engine, object: Ptr) -> u32 {
    if object.addr() == 0 {
        return 0;
    }
    let name = e.global::<u32>(BS_BOUND_NAME);
    let data = e.call(GET_EXTRA_DATA, &args![object, name]).u32();
    if data == 0 {
        return 0;
    }
    let mut rtti = e.vcall(data, 8, &args![]).u32();
    while rtti != 0 {
        if rtti == BS_BOUND_RTTI {
            return data;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    0
}

// Translated from 00c55200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c55200 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f625, state variable) is not translated
/// The start-up initializer of the `BSBound` name: a 4-byte `NiFixedString` from the memory
/// manager set to `"BBX"` (`00a5b690`); the compiler kept a test of the constant text's
/// address (`MOV EAX,imm; TEST; JNZ`), so the branch that stores the empty string
/// (`0109b220`) never runs (translated anyway). The string (or null when the allocation
/// failed) goes to `012031e4`.
pub fn fn_00c55200(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    let stored = if block == 0 {
        0
    } else {
        let text = std::hint::black_box(BS_BOUND_NAME_TEXT);
        let handle = if text != 0 {
            e.call(FIXED_STRING_CREATE, &args![BS_BOUND_NAME_TEXT])
                .u32()
        } else {
            e.global::<u32>(FIXED_STRING_EMPTY)
        };
        e.mem.set_u32(block, handle);
        block
    };
    e.set_global::<u32>(BS_BOUND_NAME, stored);
}

// Translated from 00c552a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Its `atexit` destructor: when the name exists, releases its handle (unless it is the
/// empty string `0109b220`: `InterlockedDecrement` of the count 8 bytes before the text),
/// frees the 4 bytes through the memory manager (behind a folded `1 & 1` test, always true),
/// and clears the global.
pub fn fn_00c552a0(e: &mut Engine) {
    let string = e.global::<u32>(BS_BOUND_NAME);
    if string != 0 {
        let handle = e.mem.u32(string);
        if handle != e.global::<u32>(FIXED_STRING_EMPTY) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
        if std::hint::black_box(1u32) & 1 != 0 {
            e.call(DEALLOCATE, &args![MEMORY_MANAGER, string]);
        }
    }
    e.set_global::<u32>(BS_BOUND_NAME, 0);
}

// Translated from 00c55330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBound`'s scalar deleting destructor (engine map; the decompiler's library match is a
/// folded body): the `NiExtraData` destructor (`00a7b300`), then, for bit 0 of `flags`,
/// `00aa1460(this, 0x24)`. Returns `this`.
pub fn bs_bound_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(EXTRA_DATA_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x24u32]);
    }
    this
}

/// `NiExtraData::RegisterStreamables` (`00a7b290`), `SaveBinary`
/// (`00a7b2c0`), `IsEqual` (`00a7b3d0`) and `GetViewerStrings` (`00a7b580`):
/// `thiscall`, one word each.
const EXTRA_DATA_REGISTER_STREAMABLES: u32 = 0x00a7_b290;
const EXTRA_DATA_SAVE_BINARY: u32 = 0x00a7_b2c0;
const EXTRA_DATA_IS_EQUAL: u32 = 0x00a7_b3d0;
const EXTRA_DATA_GET_VIEWER_STRINGS: u32 = 0x00a7_b580;
/// `NiColor::SaveBinary` (`00a69780`, `thiscall` on three floats, the
/// stream) and `NiPoint3::GetViewerString` (`00a59af0`, `thiscall` on the
/// point, the name).
const THREE_FLOATS_SAVE_BINARY: u32 = 0x00a6_9780;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
/// `NiGetViewerString` (`00a7d330`, `cdecl`, the class name) and the
/// viewer-string array's `Add` (`0096a610`, `thiscall` on the array, the
/// string's address).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_ADD: u32 = 0x0096_a610;
/// `Center` (+0xC) and `Extents` (+0x18), `NiPoint3`s (Xbox PDB), and the
/// names "Center" (`010c1040`) and "Extents" (`010c2bf8`).
const CENTER: u32 = 0x0c;
const EXTENTS: u32 = 0x18;
const CENTER_NAME: u32 = 0x010c_1040;
const EXTENTS_NAME: u32 = 0x010c_2bf8;

// Translated from 00c54e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBound::RegisterStreamables` (Xbox PDB): `NiExtraData`'s
/// (`00a7b290`); the answer in EAX is 0 when that answered nonzero in AL
/// and 1 otherwise (`NEG`, `SBB`, `ADD 1`, as the exe computes it).
pub fn bs_bound_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> u32 {
    let registered = e
        .call(EXTRA_DATA_REGISTER_STREAMABLES, &args![this, stream])
        .u8();
    u32::from(registered == 0)
}

// Translated from 00c54ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBound::SaveBinary` (Xbox PDB): `NiExtraData`'s (`00a7b2c0`), then the
/// centre and the extents (`NiColor::SaveBinary` on each).
pub fn bs_bound_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EXTRA_DATA_SAVE_BINARY, &args![this, stream]);
    e.call(
        THREE_FLOATS_SAVE_BINARY,
        &args![this.addr() + CENTER, stream],
    );
    e.call(
        THREE_FLOATS_SAVE_BINARY,
        &args![this.addr() + EXTENTS, stream],
    );
}

// Translated from 00c54f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBound::IsEqual` (Xbox PDB): `NiExtraData`'s (`00a7b3d0`, AL); when
/// that holds, the centres and the extents compared component by component
/// (`FUCOMPP`: a NaN is unequal), both pairs evaluated, their results
/// combined with `AND`.
pub fn bs_bound_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> u8 {
    let base = e.call(EXTRA_DATA_IS_EQUAL, &args![this, other]).u8();
    if base == 0 {
        return base;
    }
    let same = |e: &Engine, off: u32| {
        (0..3).all(|i| {
            let mine = e.mem.f32(this.addr() + off + 4 * i);
            let theirs = e.mem.f32(other.addr() + off + 4 * i);
            mine == theirs
        })
    };
    let centers = u8::from(same(e, CENTER));
    let extents = u8::from(same(e, EXTENTS));
    centers & extents
}

// Translated from 00c550f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBound::GetViewerStrings` (Xbox PDB): `NiExtraData`'s (`00a7b580`),
/// then the class name, "Center" with the centre and "Extents" with the
/// extents (`NiPoint3::GetViewerString`), each added through `0096a610`
/// with the address of its own local.
pub fn bs_bound_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(EXTRA_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    let add = |e: &mut Engine, text: u32| {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), text);
            e.call(STRINGS_ADD, &args![strings, slot]);
        });
    };
    let class_name = e.global::<u32>(BS_BOUND_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    add(e, text);
    let text = e
        .call(
            POINT_GET_VIEWER_STRING,
            &args![this.addr() + CENTER, CENTER_NAME],
        )
        .u32();
    add(e, text);
    let text = e
        .call(
            POINT_GET_VIEWER_STRING,
            &args![this.addr() + EXTENTS, EXTENTS_NAME],
        )
        .u32();
    add(e, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5_4e90, bs_bound_register_streamables(Ptr, Ptr) -> u32),
        entry!(0x00c5_4ec0, bs_bound_save_binary(Ptr, Ptr)),
        entry!(0x00c5_4f00, bs_bound_is_equal(Ptr, Ptr) -> u8),
        entry!(0x00c5_50f0, bs_bound_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00c5_5320, fn_00c55320(Ptr) -> u32),
        entry!(0x00c5_4e70, fn_00c54e70(Ptr, Ptr)),
        entry!(0x00c5_4e30, bs_bound_load_binary(Ptr, Ptr)),
        entry!(0x00c5_5010, fn_00c55010() -> u32),
        entry!(0x00c5_5170, fn_00c55170(Ptr) -> u32),
        entry!(0x00c5_5200, fn_00c55200()),
        entry!(0x00c5_52a0, fn_00c552a0()),
        entry!(
            0x00c5_5330,
            bs_bound_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c54e70_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00A7B3B0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_4e70, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00A7B3B0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c55320_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_5320, &args![0x1000u32]).u32(), 0x0120_31e8);
        assert_eq!(e.call(0x00c5_5320, &args![0u32]).u32(), 0x0120_31e8);
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn log_of(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != top)
            .collect()
    }

    #[test]
    fn load_binary_reads_the_base_then_center_and_extents() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_LOAD_BINARY, |_, _| Ret::default());
        e.register(THREE_FLOATS_LOAD_BINARY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_4e30, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            log_of(&mut e, 0x00c5_4e30),
            vec![
                (EXTRA_DATA_LOAD_BINARY, vec![0x1000, 0x2000]),
                (THREE_FLOATS_LOAD_BINARY, vec![0x100c, 0x2000]),
                (THREE_FLOATS_LOAD_BINARY, vec![0x1018, 0x2000]),
            ]
        );
    }

    #[test]
    fn factory_builds_a_bound_or_passes_null_on() {
        for allocated in [true, false] {
            let mut e = Engine::new();
            e.map(BS_BOUND_NAME, 4);
            e.mem.set_u32(BS_BOUND_NAME, 0x4321);
            e.map(ZERO_POINT, 0xc);
            for i in 0..3 {
                e.mem.set_u32(ZERO_POINT + 4 * i, 0x10 + i);
            }
            let block = if allocated { e.mem.alloc(0x24) } else { 0 };
            e.register_double(NI_NEW, move |_, _| block.into_ret());
            e.register(EXTRA_DATA_CONSTRUCTOR, |e, a| {
                e.mem.set_u32(a[0], 0x5555);
                a[0].into_ret()
            });
            e.register(SET_NAME, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00c5_5010, &args![]).u32(), block);
            let mut want = vec![(NI_NEW, vec![0x24])];
            if allocated {
                want.push((EXTRA_DATA_CONSTRUCTOR, vec![block, 0x4321]));
                assert_eq!(e.mem.u32(block), VTABLE_BS_BOUND);
                for start in [0x0cu32, 0x18] {
                    for i in 0..3 {
                        assert_eq!(e.mem.u32(block + start + 4 * i), 0x10 + i);
                    }
                }
            }
            want.push((SET_NAME, vec![block, CREATED_NAME]));
            assert_eq!(log_of(&mut e, 0x00c5_5010), want);
        }
    }

    #[test]
    fn cast_keeps_extra_data_whose_rtti_chain_reaches_bs_bound() {
        let mut e = Engine::new();
        e.map(BS_BOUND_NAME, 4);
        e.mem.set_u32(BS_BOUND_NAME, 0x4321);
        // A derived RTTI record whose parent is BSBound's, and an unrelated one.
        let derived = e.mem.alloc(8);
        e.mem.set_u32(derived + 4, BS_BOUND_RTTI);
        let other = e.mem.alloc(8);
        let data = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, 0x00aa_0008);
        e.mem.set_u32(data, vtable);
        for (rtti, found) in [(derived, true), (BS_BOUND_RTTI, true), (other, false)] {
            e.register_double(0x00aa_0008, move |_, _| rtti.into_ret());
            e.register_double(GET_EXTRA_DATA, move |_, a| {
                assert_eq!(a[1], 0x4321);
                data.into_ret()
            });
            let want = if found { data } else { 0 };
            assert_eq!(e.call(0x00c5_5170, &args![0x900u32]).u32(), want);
        }
        e.register(GET_EXTRA_DATA, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00c5_5170, &args![0x900u32]).u32(), 0);
        e.register(GET_EXTRA_DATA, |_, _| {
            panic!("a null object is not searched")
        });
        assert_eq!(e.call(0x00c5_5170, &args![0u32]).u32(), 0);
    }

    #[test]
    fn name_initializer_and_destructor() {
        let mut e = Engine::new();
        e.map(BS_BOUND_NAME, 4);
        e.map(FIXED_STRING_EMPTY, 4);
        e.mem.set_u32(FIXED_STRING_EMPTY, 0x4444);
        let block = e.mem.alloc(4);
        e.register_double(ALLOCATE, move |_, _| block.into_ret());
        e.register(FIXED_STRING_CREATE, |_, _| 0x8008u32.into_ret());
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_5200, &args![]);
        assert_eq!(
            log_of(&mut e, 0x00c5_5200),
            vec![
                (ALLOCATE, vec![MEMORY_MANAGER, 4]),
                (FIXED_STRING_CREATE, vec![BS_BOUND_NAME_TEXT]),
            ]
        );
        assert_eq!(e.mem.u32(BS_BOUND_NAME), block);
        assert_eq!(e.mem.u32(block), 0x8008);
        e.call_log = Some(vec![]);
        e.call(0x00c5_52a0, &args![]);
        assert_eq!(
            log_of(&mut e, 0x00c5_52a0),
            vec![
                (INTERLOCKED_DECREMENT, vec![0x8000]),
                (DEALLOCATE, vec![MEMORY_MANAGER, block]),
            ]
        );
        assert_eq!(e.mem.u32(BS_BOUND_NAME), 0);
        // A failed allocation stores null; an empty name is not released.
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00c5_5200, &args![]);
        assert_eq!(e.mem.u32(BS_BOUND_NAME), 0);
        e.mem.set_u32(block, 0x4444);
        e.mem.set_u32(BS_BOUND_NAME, block);
        e.call_log = Some(vec![]);
        e.call(0x00c5_52a0, &args![]);
        assert_eq!(
            log_of(&mut e, 0x00c5_52a0),
            vec![(DEALLOCATE, vec![MEMORY_MANAGER, block])]
        );
    }

    #[test]
    fn deleting_destructor_frees_on_bit_0() {
        for (flags, frees) in [(0u32, false), (1, true)] {
            let mut e = Engine::new();
            e.register(EXTRA_DATA_DESTRUCTOR, |_, _| Ret::default());
            e.register(NI_DELETE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00c5_5330, &args![0x700u32, flags]).u32(), 0x700);
            let mut want = vec![(EXTRA_DATA_DESTRUCTOR, vec![0x700])];
            if frees {
                want.push((NI_DELETE, vec![0x700, 0x24]));
            }
            assert_eq!(log_of(&mut e, 0x00c5_5330), want);
        }
    }
}

#[cfg(test)]
mod n066_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn register_streamables_inverts_the_base_answer() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_REGISTER_STREAMABLES, |_, _| word(0x1234_5601));
        assert_eq!(e.call(0x00c5_4e90, &args![0x1000u32, 0x2000u32]).u32(), 0);
        e.register(EXTRA_DATA_REGISTER_STREAMABLES, |_, _| word(0x1234_5600));
        assert_eq!(e.call(0x00c5_4e90, &args![0x1000u32, 0x2000u32]).u32(), 1);
    }

    #[test]
    fn save_binary_writes_the_base_the_centre_and_the_extents() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_SAVE_BINARY, |_, _| Ret::default());
        e.register(THREE_FLOATS_SAVE_BINARY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_4ec0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EXTRA_DATA_SAVE_BINARY, vec![0x1000, 0x2000]),
                (THREE_FLOATS_SAVE_BINARY, vec![0x100c, 0x2000]),
                (THREE_FLOATS_SAVE_BINARY, vec![0x1018, 0x2000]),
            ]
        );
    }

    #[test]
    fn is_equal_compares_centre_and_extents_after_the_base() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_IS_EQUAL, |_, _| word(1));
        let this = e.mem.alloc(0x24);
        let other = e.mem.alloc(0x24);
        for (i, v) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0].iter().enumerate() {
            e.mem.set_f32(this + 0xc + 4 * i as u32, *v);
            e.mem.set_f32(other + 0xc + 4 * i as u32, *v);
        }
        assert_eq!(e.call(0x00c5_4f00, &args![this, other]).u32(), 1);
        e.mem.set_f32(other + 0x20, 7.0);
        assert_eq!(e.call(0x00c5_4f00, &args![this, other]).u32(), 0);
        e.mem.set_f32(other + 0x20, 6.0);
        e.mem.set_u32(other + 0x10, 0x7fc0_0000);
        e.mem.set_u32(this + 0x10, 0x7fc0_0000);
        assert_eq!(e.call(0x00c5_4f00, &args![this, other]).u32(), 0);
        e.register(EXTRA_DATA_IS_EQUAL, |_, _| word(0x5500));
        assert_eq!(e.call(0x00c5_4f00, &args![this, this]).u32(), 0);
    }

    #[test]
    fn get_viewer_strings_adds_the_name_the_centre_and_the_extents() {
        let mut e = Engine::new();
        e.map(BS_BOUND_RTTI, 4);
        e.set_global(BS_BOUND_RTTI, 0x0555_0000u32);
        e.register(EXTRA_DATA_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x7000_0001));
        e.register(POINT_GET_VIEWER_STRING, |_, a| word(a[1]));
        let added = Rc::new(RefCell::new(vec![]));
        let seen = added.clone();
        e.register_double(STRINGS_ADD, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c5_50f0, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            *added.borrow(),
            vec![
                (0x2000, 0x7000_0001),
                (0x2000, CENTER_NAME),
                (0x2000, EXTENTS_NAME)
            ]
        );
        assert_eq!(
            calls_to(&e, POINT_GET_VIEWER_STRING),
            vec![vec![0x100c, CENTER_NAME], vec![0x1018, EXTENTS_NAME]]
        );
    }
}
