//! `BSMain/bspsysstripupdatemodifier.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c31650` (named `NiPSysModifier::LinkObject` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C31650: u32 = 0x00c3_1650;

// Translated from 00c5ff90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c3314 of the table whose `CreateClone` is
/// `BSPSysStripUpdateModifier::CreateClone` (010c330c), the slot after `LoadBinary`
/// (010c3310). Forwards `this` and the stream to `00c31650` (`NiPSysModifier::LinkObject`).
/// Returns nothing: Xbox PDB `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is
/// the `NiStream` reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c5ff90(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C31650, &args![this, stream]);
}

/// `00c31670` (named `NiPSysModifier::RegisterStreamables` in the engine map): the base
/// body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const REGISTER_STREAMABLES_TARGET_00C31670: u32 = 0x00c3_1670;

// Translated from 00c5ffb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c3318 of the `BSPSysStripUpdateModifier` table,
/// after the `LinkObject` word 010c3314. Forwards `this` and the stream to `00c31670`
/// (`NiPSysModifier::RegisterStreamables`). Returns the `bool` in `AL`: Xbox PDB
/// `?RegisterStreamables@NiObject@@UAA_NAAVNiStream@@@Z` returns `_N`; `stream` is the
/// `NiStream` reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c5ffb0(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(REGISTER_STREAMABLES_TARGET_00C31670, &args![this, stream])
        .bool()
}

// Translated from 00c600f0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01204388 without reading the receiver. The map names this body `BSPSysStripUpdateModifier::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c600f0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_4388
}

/// `BSPSysStripUpdateModifier` (Xbox PDB, 0x1C bytes): vtable `010c32c4`;
/// `fUpdateDeltaTime` (`f32`) at +0x18.
const VTABLE: u32 = 0x010c_32c4;
const MODIFIER_SIZE: u32 = 0x1c;
const UPDATE_DELTA_TIME: u32 = 0x18;
/// `NiPSysModifier`'s constructor (`00c31720`), destructor (`00c31750`), member copy
/// (`00c31780`, `(destination, cloning)`), `LoadBinary` (`00c318b0`), `SaveBinary`
/// (`00c31950`), `IsEqual` (`00c31830`) and `GetViewerStrings` (`00c319e0`), `thiscall`.
const MODIFIER_CONSTRUCT: u32 = 0x00c3_1720;
const MODIFIER_DESTRUCT: u32 = 0x00c3_1750;
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
const MODIFIER_SAVE_BINARY: u32 = 0x00c3_1950;
const MODIFIER_IS_EQUAL: u32 = 0x00c3_1830;
const MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_19e0;
/// `BSStripPSysData::InitializePoint` (`00c25d40`, `thiscall`, `(index)`), the point's
/// time `00c25de0` (`thiscall`, `(index)`, a float in `ST0`) and its update `00c26120`
/// (`thiscall`, `(index, time)`).
const STRIP_DATA_INITIALIZE_POINT: u32 = 0x00c2_5d40;
const STRIP_DATA_POINT_TIME: u32 = 0x00c2_5de0;
const STRIP_DATA_UPDATE_POINT: u32 = 0x00c2_6120;
/// `00aa13e0` (`cdecl`, one word): the Ni allocator; `00aa1460` (`cdecl`, `(block,
/// size)`): the sized delete.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// The stream word read (`00a26df0`) and `NiStreamSaveBinary<float>` (`00a3da50`), both
/// `cdecl`, `(stream, &word)`.
const STREAM_READ_WORD: u32 = 0x00a2_6df0;
const STREAM_SAVE_WORD: u32 = 0x00a3_da50;
/// `NiGetViewerString` (`00a7d330`, `(name)`), its `(label, float)` form `00a7d550` and the
/// viewer-string array's add (`0096a610`, `thiscall`, `(&string)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const VIEWER_STRINGS_ADD: u32 = 0x0096_a610;
/// The class's `ms_RTTI` (its first word is the class name) and `"fUpdateDeltaTime"`.
const RTTI: u32 = 0x0120_4388;
const UPDATE_DELTA_TIME_LABEL: u32 = 0x010c_3368;

// Translated from 00c5fd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSPSysStripUpdateModifier` constructor.
/// `NiPSysModifier`'s constructor (`00c31720`) and the vtable. Returns `this`.
pub fn fn_00c5fd30(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MODIFIER_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    this
}

// Translated from 00c5fd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysStripUpdateModifier::~BSPSysStripUpdateModifier` (Xbox PDB, engine map): the
/// vtable, then `NiPSysModifier`'s destructor.
pub fn bs_psys_strip_update_modifier_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(MODIFIER_DESTRUCT, &args![this]);
}

// Translated from 00c5fd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysStripUpdateModifier::Initialize` (Xbox PDB, engine map; `(data, index)`):
/// `BSStripPSysData::InitializePoint(index)` on `data` (the 16-bit index).
pub fn bs_psys_strip_update_modifier_initialize(e: &mut Engine, _this: Ptr, data: u32, index: u16) {
    e.call(STRIP_DATA_INITIALIZE_POINT, &args![data, u32::from(index)]);
}

// Translated from 00c5fda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysStripUpdateModifier::Update` (Xbox PDB, engine map; `(time, data)`): for each of
/// the strip data's points (count, 16 bits, at +0x50), `time` minus the point's time
/// (`00c25de0`, x87 `FSUBR`, stored as a `float`) is compared with `fUpdateDeltaTime`; when
/// the delta time is below it (ordered), the point is updated (`00c26120` with the index
/// and `time`). The code also steps a pointer through the points (+0x70, 0x1C each) that
/// nothing reads.
pub fn bs_psys_strip_update_modifier_update(e: &mut Engine, this: Ptr, time: f32, data: u32) {
    let count = e.mem.u16(data.wrapping_add(0x50));
    let mut point = e.mem.u32(data.wrapping_add(0x70));
    let mut index = 0u16;
    while index < count {
        let point_time = e
            .call(STRIP_DATA_POINT_TIME, &args![data, u32::from(index)])
            .f64();
        let passed = x87_store32(x87_sub(x87_m32(time.to_bits()), point_time));
        let passed = x87_load32(passed);
        let delta = x87_load32(e.mem.u32(this.addr().wrapping_add(UPDATE_DELTA_TIME)));
        if delta < passed {
            e.call(
                STRIP_DATA_UPDATE_POINT,
                &args![data, u32::from(index), x87_quiet_bits(time.to_bits())],
            );
        }
        index = index.wrapping_add(1);
        point = point.wrapping_add(0x1c);
    }
    let _ = point;
}

// Translated from 00c5fe30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5fe30 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2ffc2, state variable) is not translated
/// `BSPSysStripUpdateModifier::CreateClone` (Xbox PDB, engine map): allocates 0x1C bytes,
/// constructs a modifier in it when that succeeds, copies the members into it (`00c5fec0`;
/// into address 0 when the allocation failed) and returns it.
pub fn bs_psys_strip_update_modifier_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![MODIFIER_SIZE]).u32();
    let clone = if block == 0 {
        Ptr::new(0)
    } else {
        fn_00c5fd30(e, Ptr::new(block))
    };
    fn_00c5fec0(e, this, clone, cloning);
    clone
}

// Translated from 00c5fec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `(destination, cloning)`, the member copy):
/// `NiPSysModifier`'s member copy only (`fUpdateDeltaTime` is not copied).
pub fn fn_00c5fec0(e: &mut Engine, this: Ptr, destination: Ptr, cloning: u32) {
    e.call(MODIFIER_COPY_MEMBERS, &args![this, destination, cloning]);
}

// Translated from 00c5fee0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5fee0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2ffec, state variable) is not translated
/// Unnamed in the engine map (the `BSPSysStripUpdateModifier` loader `BSStream::InitLoaders`
/// registers, no arguments): allocates 0x1C bytes and constructs a modifier in it when that
/// succeeds. Returns it, or 0.
pub fn fn_00c5fee0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![MODIFIER_SIZE]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_00c5fd30(e, Ptr::new(block))
}

// Translated from 00c5ff60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the vtable's `LoadBinary` place): `NiPSysModifier::LoadBinary`,
/// then `fUpdateDeltaTime` (4 bytes through `00a26df0`).
pub fn fn_00c5ff60(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(MODIFIER_LOAD_BINARY, &args![this, stream]);
    e.call(
        STREAM_READ_WORD,
        &args![stream, this.addr().wrapping_add(UPDATE_DELTA_TIME)],
    );
}

// Translated from 00c5ffd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the vtable's `SaveBinary` place): `NiPSysModifier::SaveBinary`,
/// then `fUpdateDeltaTime` (`NiStreamSaveBinary<float>`).
pub fn fn_00c5ffd0(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(MODIFIER_SAVE_BINARY, &args![this, stream]);
    e.call(
        STREAM_SAVE_WORD,
        &args![stream, this.addr().wrapping_add(UPDATE_DELTA_TIME)],
    );
}

// Translated from 00c60000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysStripUpdateModifier::IsEqual` (Xbox PDB, engine map): `NiPSysModifier::IsEqual`,
/// and when that holds, whether the two `fUpdateDeltaTime` compare equal (x87 `FUCOMPP`:
/// unordered is not equal).
pub fn bs_psys_strip_update_modifier_is_equal(e: &mut Engine, this: Ptr, other: u32) -> bool {
    if e.call(MODIFIER_IS_EQUAL, &args![this, other]).u8() == 0 {
        return false;
    }
    let theirs = x87_load32(e.mem.u32(other.wrapping_add(UPDATE_DELTA_TIME)));
    let mine = x87_load32(e.mem.u32(this.addr().wrapping_add(UPDATE_DELTA_TIME)));
    mine == theirs
}

// Translated from 00c60050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysStripUpdateModifier::GetViewerStrings` (Xbox PDB, engine map):
/// `NiPSysModifier::GetViewerStrings`, the class name, then `"fUpdateDeltaTime"` with its
/// value (the float copied through the x87 stack), each added to the list.
pub fn bs_psys_strip_update_modifier_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(8, |e, slots| {
        let name = e.global::<u32>(RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
        let slot = slots.addr().wrapping_add(4);
        e.mem.set_u32(slot, text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
        let delta = x87_quiet_bits(e.mem.u32(this.addr().wrapping_add(UPDATE_DELTA_TIME)));
        let text = e
            .call(
                NI_GET_VIEWER_STRING_FLOAT,
                &args![UPDATE_DELTA_TIME_LABEL, delta],
            )
            .u32();
        e.mem.set_u32(slots.addr(), text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, slots]);
    });
}

// Translated from 00c600c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysStripUpdateModifier::scalar deleting destructor` (Xbox PDB, engine map): the
/// destructor (`00c5fd50`) and the sized delete of 0x1C bytes with bit 0 of `flags`.
/// Returns `this`.
pub fn bs_psys_strip_update_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    bs_psys_strip_update_modifier_destructor(e, this);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, MODIFIER_SIZE]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5_fd30, fn_00c5fd30(Ptr) -> Ptr),
        entry!(0x00c5_fd50, bs_psys_strip_update_modifier_destructor(Ptr)),
        entry!(
            0x00c5_fd70,
            bs_psys_strip_update_modifier_initialize(Ptr, u32, u16)
        ),
        entry!(
            0x00c5_fda0,
            bs_psys_strip_update_modifier_update(Ptr, f32, u32)
        ),
        entry!(
            0x00c5_fe30,
            bs_psys_strip_update_modifier_create_clone(Ptr, u32) -> Ptr
        ),
        entry!(0x00c5_fec0, fn_00c5fec0(Ptr, Ptr, u32)),
        entry!(0x00c5_fee0, fn_00c5fee0() -> Ptr),
        entry!(0x00c5_ff60, fn_00c5ff60(Ptr, Ptr)),
        entry!(0x00c5_ffd0, fn_00c5ffd0(Ptr, Ptr)),
        entry!(
            0x00c6_0000,
            bs_psys_strip_update_modifier_is_equal(Ptr, u32) -> bool
        ),
        entry!(
            0x00c6_0050,
            bs_psys_strip_update_modifier_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(
            0x00c6_00c0,
            bs_psys_strip_update_modifier_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c6_00f0, fn_00c600f0(Ptr) -> u32),
        entry!(0x00c5_ff90, fn_00c5ff90(Ptr, Ptr)),
        entry!(0x00c5_ffb0, fn_00c5ffb0(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c5ff90_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C31650, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_ff90, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C31650)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c5ffb0_forwards_this_and_the_argument_and_returns_the_byte() {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(REGISTER_STREAMABLES_TARGET_00C31670, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x00c5_ffb0, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == REGISTER_STREAMABLES_TARGET_00C31670)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c600f0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_00f0, &args![0x1000u32]).u32(), 0x0120_4388);
        assert_eq!(e.call(0x00c6_00f0, &args![0u32]).u32(), 0x0120_4388);
    }
}

#[cfg(test)]
mod n068_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn constructor_destructor_factory_and_deleting_destructor() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, MODIFIER_CONSTRUCT, 0);
        logged(&mut e, &log, MODIFIER_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        let block = e.mem.alloc(0x1c);
        logged(&mut e, &log, NI_ALLOCATE, block);
        assert_eq!(e.call(0x00c5_fee0, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(calls(&log, MODIFIER_CONSTRUCT), vec![vec![block]]);
        e.mem.set_u32(block, 0);
        assert_eq!(e.call(0x00c6_00c0, &args![block, 1u32]).u32(), block);
        assert_eq!(e.mem.u32(block), VTABLE);
        e.call(0x00c6_00c0, &args![block, 0u32]);
        assert_eq!(calls(&log, MODIFIER_DESTRUCT).len(), 2);
        assert_eq!(calls(&log, NI_SIZED_DELETE), vec![vec![block, 0x1c]]);
        logged(&mut e, &log, NI_ALLOCATE, 0);
        assert_eq!(e.call(0x00c5_fee0, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_copies_only_the_base_members() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, MODIFIER_CONSTRUCT, 0);
        logged(&mut e, &log, MODIFIER_COPY_MEMBERS, 0);
        let block = e.mem.alloc(0x1c);
        logged(&mut e, &log, NI_ALLOCATE, block);
        let this = e.mem.alloc(0x1c);
        e.mem.set_f32(this + UPDATE_DELTA_TIME, 0.5);
        assert_eq!(e.call(0x00c5_fe30, &args![this, 0x5555u32]).u32(), block);
        assert_eq!(
            calls(&log, MODIFIER_COPY_MEMBERS),
            vec![vec![this, block, 0x5555]]
        );
        assert_eq!(e.mem.u32(block + UPDATE_DELTA_TIME), 0);
    }

    #[test]
    fn initialize_sets_up_the_point() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, STRIP_DATA_INITIALIZE_POINT, 0);
        e.call(0x00c5_fd70, &args![0x1000u32, 0x2000u32, 0x0005_0007u32]);
        assert_eq!(
            calls(&log, STRIP_DATA_INITIALIZE_POINT),
            vec![vec![0x2000, 7]]
        );
    }

    #[test]
    fn update_moves_points_older_than_the_delta() {
        let mut e = Engine::new();
        let log = Log::default();
        e.register(STRIP_DATA_POINT_TIME, |_, a| {
            // Point times 1.0, 2.5 and NaN.
            [1.0f64, 2.5, f64::NAN][a[1] as usize].into_ret()
        });
        logged(&mut e, &log, STRIP_DATA_UPDATE_POINT, 0);
        let data = e.mem.alloc(0x80);
        e.mem.set_u16(data + 0x50, 3);
        let this = e.mem.alloc(0x1c);
        e.mem.set_f32(this + UPDATE_DELTA_TIME, 0.5);
        e.call(0x00c5_fda0, &args![this, 3.0f32, data]);
        // 3.0 - 1.0 = 2.0 > 0.5 moves; 3.0 - 2.5 = 0.5 is not above; NaN does not.
        assert_eq!(
            calls(&log, STRIP_DATA_UPDATE_POINT),
            vec![vec![data, 0, 3.0f32.to_bits()]]
        );
    }

    #[test]
    fn load_and_save_carry_the_delta_time() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, MODIFIER_LOAD_BINARY, 0);
        logged(&mut e, &log, MODIFIER_SAVE_BINARY, 0);
        logged(&mut e, &log, STREAM_READ_WORD, 0);
        logged(&mut e, &log, STREAM_SAVE_WORD, 0);
        e.call(0x00c5_ff60, &args![0x1000u32, 0x6000u32]);
        e.call(0x00c5_ffd0, &args![0x1000u32, 0x6000u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (MODIFIER_LOAD_BINARY, vec![0x1000, 0x6000]),
                (STREAM_READ_WORD, vec![0x6000, 0x1018]),
                (MODIFIER_SAVE_BINARY, vec![0x1000, 0x6000]),
                (STREAM_SAVE_WORD, vec![0x6000, 0x1018]),
            ]
        );
    }

    #[test]
    fn is_equal_compares_the_delta_times() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, MODIFIER_IS_EQUAL, 1);
        let this = e.mem.alloc(0x1c);
        let other = e.mem.alloc(0x1c);
        e.mem.set_f32(this + UPDATE_DELTA_TIME, 0.25);
        e.mem.set_f32(other + UPDATE_DELTA_TIME, 0.25);
        assert!(e.call(0x00c6_0000, &args![this, other]).bool());
        e.mem.set_f32(other + UPDATE_DELTA_TIME, 0.5);
        assert!(!e.call(0x00c6_0000, &args![this, other]).bool());
        e.mem.set_u32(this + UPDATE_DELTA_TIME, 0x7fc0_0000);
        e.mem.set_u32(other + UPDATE_DELTA_TIME, 0x7fc0_0000);
        assert!(!e.call(0x00c6_0000, &args![this, other]).bool());
        logged(&mut e, &log, MODIFIER_IS_EQUAL, 0);
        e.mem.set_f32(this + UPDATE_DELTA_TIME, 0.5);
        assert!(!e.call(0x00c6_0000, &args![this, other]).bool());
    }

    #[test]
    fn viewer_strings_add_the_name_and_the_delta_time() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, MODIFIER_GET_VIEWER_STRINGS, 0);
        e.map(RTTI, 4);
        e.set_global::<u32>(RTTI, 0x0101_0000);
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        logged(&mut e, &log, NI_GET_VIEWER_STRING_FLOAT, 0x7000);
        let l = log.clone();
        e.register_double(VIEWER_STRINGS_ADD, move |e, a| {
            l.borrow_mut().push((0, vec![a[0], e.mem.u32(a[1])]));
            Ret::default()
        });
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + UPDATE_DELTA_TIME, 0x7f80_0001);
        e.call(0x00c6_0050, &args![this, 0x6000u32]);
        assert_eq!(
            calls(&log, NI_GET_VIEWER_STRING_FLOAT),
            vec![vec![UPDATE_DELTA_TIME_LABEL, 0x7fc0_0001]]
        );
        assert_eq!(
            calls(&log, 0),
            vec![vec![0x6000, 0x0101_0001], vec![0x6000, 0x7000]]
        );
    }
}
