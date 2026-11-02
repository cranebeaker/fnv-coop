//! `NiAnimation/nikeybasedinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiKeyBasedInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_KEY_BASED_INTERPOLATOR_RTTI: u32 = 0x011f_3740;

// Translated from 00a3ca40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyBasedInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_key_based_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_KEY_BASED_INTERPOLATOR_RTTI)
}

/// `00a88a40`: the jump target of `00aa0ac0`.
const JUMP_TARGET_00AA0AC0: u32 = 0x00a8_8a40;

/// `00a5d450`: the jump target of `00aa1000`.
const JUMP_TARGET_00AA1000: u32 = 0x00a5_d450;

// Translated from 00aa0ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyBasedInterpolator::LoadBinary` (Xbox PDB): a 5-byte jump to `00a88a40` (engine map: `NiInterpolator::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiKeyBasedInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn ni_key_based_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00AA0AC0, &args![this, stream]);
}

// Translated from 00aa1000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyBasedInterpolator::CopyMembers` (Xbox PDB): a 5-byte jump to `00a5d450`, passing on `this` and its two arguments. Return type (void): Xbox PDB public name `?CopyMembers@NiKeyBasedInterpolator@@IAAXPAV1@AAVNiCloningProcess@@@Z`.
pub fn ni_key_based_interpolator_copy_members(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    e.call(JUMP_TARGET_00AA1000, &args![this, first, second]);
}

/// The vtable of `NiKeyBasedInterpolator` (RTTI `.?AVNiKeyBasedInterpolator@@`).
const NI_KEY_BASED_INTERPOLATOR_VTABLE: u32 = 0x0109_7054;
/// `NiInterpolator::NiInterpolator` / `~NiInterpolator` (`00a56ff0` /
/// `00a57010`, `NiAnimation/niinterpolator.cpp`).
const NI_INTERPOLATOR_CONSTRUCT: u32 = 0x00a5_6ff0;
const NI_INTERPOLATOR_DESTRUCT: u32 = 0x00a5_7010;

// Translated from 00a3ca10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyBasedInterpolator::NiKeyBasedInterpolator` (Xbox PDB): the
/// `NiInterpolator` constructor, then this class's vtable. Returns `this`.
pub fn ni_key_based_interpolator_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_INTERPOLATOR_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_KEY_BASED_INTERPOLATOR_VTABLE);
    this
}

// Translated from 00a3ca30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyBasedInterpolator::~NiKeyBasedInterpolator` (unnamed in the engine
/// map): stores this class's vtable, then the `NiInterpolator` destructor
/// (`00a57010`, a tail jump).
pub fn ni_key_based_interpolator_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_KEY_BASED_INTERPOLATOR_VTABLE);
    e.call(NI_INTERPOLATOR_DESTRUCT, &args![this]);
}

/// The sized delete (`00aa1460`, cdecl, `(block, size)`) and this class's size.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
const NI_KEY_BASED_INTERPOLATOR_SIZE: u32 = 0x0c;

// Translated from 00a3ca50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyBasedInterpolator::_scalar_deleting_destructor_` (Xbox PDB): stores this class's
/// vtable, runs the `NiInterpolator` destructor (`00a57010`) and, when bit 0 of `flags` is
/// set, the sized delete of 0xC bytes. Returns `this`.
pub fn ni_key_based_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), NI_KEY_BASED_INTERPOLATOR_VTABLE);
    e.call(NI_INTERPOLATOR_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(
            NI_SIZED_DELETE,
            &args![this, NI_KEY_BASED_INTERPOLATOR_SIZE],
        );
    }
    this
}

/// Virtual slots of key-based interpolators: the channel count (`0xdc`, no arguments, 16
/// bits), a channel's key count (`0xe0`), key array (`0xec`) and key size (`0xf0`, a byte),
/// each with the channel.
const SLOT_CHANNEL_COUNT: u32 = 0xdc;
const SLOT_KEY_COUNT: u32 = 0xe0;
const SLOT_KEY_ARRAY: u32 = 0xec;
const SLOT_KEY_SIZE: u32 = 0xf0;

// Translated from 00a3ca80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nikeybasedinterpolator.cpp`; thiscall, two words): the
/// address of key `index` of `channel`: the channel's key array (slot `0xec`) plus `index`
/// times its key size (slot `0xf0`, asked first).
pub fn fn_00a3ca80(e: &mut Engine, this: Ptr, index: u32, channel: u32) -> u32 {
    let size = e.vcall(this.addr(), SLOT_KEY_SIZE, &args![channel]).u8();
    let keys = e.vcall(this.addr(), SLOT_KEY_ARRAY, &args![channel]).u32();
    keys.wrapping_add(u32::from(size).wrapping_mul(index))
}

/// The floats the active time range starts from (`0109704c` for the start, `01015f5c` for
/// the end) and the doubles it compares them with at the end (`010231b0`, `010241b0`).
const RANGE_START_FLOAT: u32 = 0x0109_704c;
const RANGE_END_FLOAT: u32 = 0x0101_5f5c;
const RANGE_START_DOUBLE: u32 = 0x0102_31b0;
const RANGE_END_DOUBLE: u32 = 0x0102_41b0;

// Translated from 00a3cac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyBasedInterpolator::GetActiveTimeRange` (Xbox PDB): the range starts from the floats
/// at `0109704c` and `01015f5c`; for each channel (a 16-bit count from slot `0xdc`, asked
/// again after each), the time of its first key (key array, slot `0xec`) lowers the start and
/// the time of its last key (count - 1 keys of the key size further, slots `0xe0`, `0xf0`,
/// `0xec`) raises the end, when both keys exist (NaN changes nothing). The key size is asked
/// once more first (its answer unused, as the exe does). A range still at both starting
/// values (compared with the doubles at `010231b0` and `010241b0`) becomes 0.0 to 0.0.
pub fn ni_key_based_interpolator_get_active_time_range(
    e: &mut Engine,
    this: Ptr,
    start: Ptr,
    end: Ptr,
) {
    let object = this.addr();
    let start = start.addr();
    let end = end.addr();
    let value = x87_quiet_bits(e.mem.u32(RANGE_START_FLOAT));
    e.mem.set_u32(start, value);
    let value = x87_quiet_bits(e.mem.u32(RANGE_END_FLOAT));
    e.mem.set_u32(end, value);
    let mut channel = 0u32;
    let count = e.vcall(object, SLOT_CHANNEL_COUNT, &args![]).u16();
    if count != 0 {
        loop {
            let index = channel & 0xffff;
            e.vcall(object, SLOT_KEY_SIZE, &args![index]);
            let first = e.vcall(object, SLOT_KEY_ARRAY, &args![index]).u32();
            let last_index = e
                .vcall(object, SLOT_KEY_COUNT, &args![index])
                .u32()
                .wrapping_sub(1);
            let size = e.vcall(object, SLOT_KEY_SIZE, &args![index]).u8();
            let keys = e.vcall(object, SLOT_KEY_ARRAY, &args![index]).u32();
            let last = keys.wrapping_add(u32::from(size).wrapping_mul(last_index));
            if first != 0 && last != 0 {
                let time = x87_load32(e.mem.u32(first));
                if x87_load32(e.mem.u32(start)) > time {
                    let bits = x87_quiet_bits(e.mem.u32(first));
                    e.mem.set_u32(start, bits);
                }
                let time = x87_load32(e.mem.u32(last));
                if x87_load32(e.mem.u32(end)) < time {
                    let bits = x87_quiet_bits(e.mem.u32(last));
                    e.mem.set_u32(end, bits);
                }
            }
            channel = channel.wrapping_add(1);
            let count = u32::from(e.vcall(object, SLOT_CHANNEL_COUNT, &args![]).u16());
            if channel >= count {
                break;
            }
        }
    }
    let unchanged_start = f64::from_bits(e.mem.u64(RANGE_START_DOUBLE));
    if x87_load32(e.mem.u32(start)) == unchanged_start {
        let unchanged_end = f64::from_bits(e.mem.u64(RANGE_END_DOUBLE));
        if x87_load32(e.mem.u32(end)) == unchanged_end {
            e.mem.set_f32(start, 0.0);
            e.mem.set_f32(end, 0.0);
        }
    }
}

/// `NiInterpolator::GetViewerStrings` (`00a570b0`, thiscall, the array), `NiGetViewerString`
/// (`00a7d330`, cdecl) and the viewer-string array's resize (`0096ad30`) and `SetAt`
/// (`0096ae90`).
const NI_INTERPOLATOR_GET_VIEWER_STRINGS: u32 = 0x00a5_70b0;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRINGS_GROW: u32 = 0x0096_ad30;
const VIEWER_STRINGS_SET_AT: u32 = 0x0096_ae90;

// Translated from 00a3cbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyBasedInterpolator::GetViewerStrings` (Xbox PDB): `NiInterpolator::GetViewerStrings`,
/// then the class name (the first word of the `NiRTTI` record at `011f3740`, through
/// `NiGetViewerString`) appended to the array (grown by its step when full), held in the
/// argument's own stack word.
pub fn ni_key_based_interpolator_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(NI_INTERPOLATOR_GET_VIEWER_STRINGS, &args![this, strings]);
    let strings = strings.addr();
    let class_name = e.mem.u32(NI_KEY_BASED_INTERPOLATOR_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(VIEWER_STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(VIEWER_STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a3_ca50,
            ni_key_based_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a3_ca80, fn_00a3ca80(Ptr, u32, u32) -> u32),
        entry!(
            0x00a3_cac0,
            ni_key_based_interpolator_get_active_time_range(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00a3_cbf0,
            ni_key_based_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a3_ca10, ni_key_based_interpolator_construct(Ptr) -> Ptr),
        entry!(0x00a3_ca30, ni_key_based_interpolator_destruct(Ptr)),
        entry!(0x00a3ca40, ni_key_based_interpolator_get_rtti() -> Ptr),
        entry!(0x00aa_0ac0, ni_key_based_interpolator_load_binary(Ptr, Ptr)),
        entry!(
            0x00aa_1000,
            ni_key_based_interpolator_copy_members(Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_key_based_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a3ca40, &args![]).u32(),
            NI_KEY_BASED_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn thunk_00aa0ac0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00aa_0ac0;
        const TARGET: u32 = 0x00a8_8a40;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00aa1000_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00aa_1000;
        const TARGET: u32 = 0x00a5_d450;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32, 0x3000u32]);
    }

    #[test]
    fn constructor_and_destructor_chain_to_the_interpolator() {
        let mut e = Engine::new();
        e.register(NI_INTERPOLATOR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        e.register(NI_INTERPOLATOR_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x0c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a3_ca10, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_KEY_BASED_INTERPOLATOR_VTABLE);
        e.mem.set_u32(this, 0);
        e.call(0x00a3_ca30, &args![this]);
        assert_eq!(e.mem.u32(this), NI_KEY_BASED_INTERPOLATOR_VTABLE);
        let log = e.call_log.take().unwrap();
        let callees: Vec<_> = log
            .iter()
            .filter(|call| {
                call.0 == NI_INTERPOLATOR_CONSTRUCT || call.0 == NI_INTERPOLATOR_DESTRUCT
            })
            .cloned()
            .collect();
        assert_eq!(
            callees,
            [
                (NI_INTERPOLATOR_CONSTRUCT, vec![this]),
                (NI_INTERPOLATOR_DESTRUCT, vec![this]),
            ]
        );
    }
}

#[cfg(test)]
mod n004_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
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
    fn scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_INTERPOLATOR_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        let this = e.mem.alloc(0x0c);
        assert_eq!(e.call(0x00a3_ca50, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_KEY_BASED_INTERPOLATOR_VTABLE);
        assert!(calls(&log, NI_SIZED_DELETE).is_empty());
        e.call(0x00a3_ca50, &args![this, 1u32]);
        assert_eq!(
            calls(&log, NI_INTERPOLATOR_DESTRUCT),
            [vec![this], vec![this]]
        );
        assert_eq!(calls(&log, NI_SIZED_DELETE), [vec![this, 0x0c]]);
    }

    /// A key-based interpolator double: channel count, key counts, key arrays and key sizes.
    fn interpolator(e: &mut Engine, channels: Vec<(u32, u32, u8)>) -> u32 {
        let count = channels.len() as u32;
        let vtable = e.mem.alloc(0x100);
        for (slot, target) in [
            (SLOT_CHANNEL_COUNT, 0x00ee_2000u32),
            (SLOT_KEY_COUNT, 0x00ee_2010),
            (SLOT_KEY_ARRAY, 0x00ee_2020),
            (SLOT_KEY_SIZE, 0x00ee_2030),
        ] {
            e.mem.set_u32(vtable + slot, target);
        }
        let shared = Rc::new(channels);
        e.register_double(0x00ee_2000, move |_, _| ret(0xabcd_0000 | count));
        let list = shared.clone();
        e.register_double(0x00ee_2010, move |_, a| ret(list[a[1] as usize].1));
        let list = shared.clone();
        e.register_double(0x00ee_2020, move |_, a| ret(list[a[1] as usize].0));
        let list = shared;
        e.register_double(0x00ee_2030, move |_, a| {
            ret(0xff00 | u32::from(list[a[1] as usize].2))
        });
        let this = e.mem.alloc(0x0c);
        e.mem.set_u32(this, vtable);
        this
    }

    #[test]
    fn fn_00a3ca80_steps_by_the_key_size() {
        let mut e = Engine::new();
        let this = interpolator(&mut e, vec![(0x4000, 3, 8), (0x5000, 2, 12)]);
        assert_eq!(e.call(0x00a3_ca80, &args![this, 2u32, 1u32]).u32(), 0x5018);
        assert_eq!(e.call(0x00a3_ca80, &args![this, 1u32, 0u32]).u32(), 0x4008);
    }

    #[test]
    fn get_active_time_range_spans_the_first_and_last_keys() {
        let mut e = Engine::new();
        e.map(RANGE_START_FLOAT, 4);
        e.map(RANGE_END_FLOAT, 4);
        e.map(RANGE_START_DOUBLE, 8);
        e.map(RANGE_END_DOUBLE, 8);
        e.mem.set_f32(RANGE_START_FLOAT, f32::MAX);
        e.mem.set_f32(RANGE_END_FLOAT, -f32::MAX);
        e.mem.set_f64(RANGE_START_DOUBLE, f64::from(f32::MAX));
        e.mem.set_f64(RANGE_END_DOUBLE, f64::from(-f32::MAX));
        let keys_a = e.mem.alloc(0x20);
        e.mem.set_f32(keys_a, 1.0);
        e.mem.set_f32(keys_a + 0x10, 3.0);
        let keys_b = e.mem.alloc(0x20);
        e.mem.set_f32(keys_b, 0.5);
        e.mem.set_f32(keys_b + 8, 2.0);
        let this = interpolator(&mut e, vec![(keys_a, 3, 8), (0, 2, 4), (keys_b, 2, 8)]);
        let range = e.mem.alloc(8);
        e.call(0x00a3_cac0, &args![this, range, range + 4]);
        assert_eq!(e.mem.f32(range), 0.5);
        assert_eq!(e.mem.f32(range + 4), 3.0);
        // No channels: the starting values become 0.0 to 0.0.
        let empty = interpolator(&mut e, vec![]);
        e.call(0x00a3_cac0, &args![empty, range, range + 4]);
        assert_eq!(e.mem.u32(range), 0);
        assert_eq!(e.mem.u32(range + 4), 0);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_name() {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(NI_KEY_BASED_INTERPOLATOR_RTTI, 4);
        e.mem.set_u32(NI_KEY_BASED_INTERPOLATOR_RTTI, 0x0155_0000);
        logged(&mut e, &log, NI_INTERPOLATOR_GET_VIEWER_STRINGS, 0);
        logged(&mut e, &log, NI_GET_VIEWER_STRING, 0x0166_0000);
        logged(&mut e, &log, VIEWER_STRINGS_GROW, 0);
        e.register(VIEWER_STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x0166_0000);
            ret(a[1])
        });
        let this = e.mem.alloc(0x0c);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xa, 2);
        e.mem.set_u16(strings + 0xe, 3);
        e.call(0x00a3_cbf0, &args![this, strings]);
        assert_eq!(
            calls(&log, NI_INTERPOLATOR_GET_VIEWER_STRINGS),
            [vec![this, strings]]
        );
        assert_eq!(calls(&log, NI_GET_VIEWER_STRING), [vec![0x0155_0000]]);
        assert_eq!(calls(&log, VIEWER_STRINGS_GROW), [vec![strings, 5]]);
    }
}
