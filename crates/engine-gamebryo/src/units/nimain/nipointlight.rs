//! `NiMain/nipointlight.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPointLight::ms_RTTI` (the class's `NiRTTI` record).
const NI_POINT_LIGHT_RTTI: u32 = 0x011f_4a98;

// Translated from 00a7d790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointLight::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_point_light_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_POINT_LIGHT_RTTI)
}

/// `NiAVObject::UpdateWorldData` (`00a68c60`, thiscall, update data).
const NI_AV_OBJECT_UPDATE_WORLD_DATA: u32 = 0x00a6_8c60;
/// `NiLight::LoadBinary` (`00a72080`, thiscall, stream).
const NI_LIGHT_LOAD_BINARY: u32 = 0x00a7_2080;
/// `NiStream`'s input file (+0x24C): read function at +8, position at +4.
const STREAM_INPUT: u32 = 0x24c;

// Translated from 00a7d7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointLight::UpdateWorldData` (Xbox PDB name in the engine map):
/// `NiAVObject::UpdateWorldData`, then increments the word at +0xA8.
pub fn ni_point_light_update_world_data(e: &mut Engine, this: Ptr, update: Ptr) {
    e.call(NI_AV_OBJECT_UPDATE_WORLD_DATA, &args![this, update]);
    let at = this.addr().wrapping_add(0xa8);
    let count = e.mem.u32(at).wrapping_add(1);
    e.mem.set_u32(at, count);
}

// Translated from 00a7d840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointLight::LoadBinary` (Xbox PDB name in the engine map):
/// `NiLight::LoadBinary`, then reads the three 4-byte attenuations
/// (+0xF0, +0xF4, +0xF8) through the stream input's read function (cdecl:
/// file, destination, 4, byte-count word, 1), each adding the bytes read to
/// the input's position. The stream argument's stack word is the
/// byte-count word.
pub fn ni_point_light_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_LIGHT_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        for offset in [0xf0u32, 0xf4, 0xf8] {
            let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
            let read = e.mem.u32(input.wrapping_add(8));
            e.mem.set_u32(count.addr(), 4);
            let destination = this.addr().wrapping_add(offset);
            let done = e
                .call(read, &args![input, destination, 4u32, count, 1u32])
                .u32();
            let position = e.mem.u32(input.wrapping_add(4)).wrapping_add(done);
            e.mem.set_u32(input.wrapping_add(4), position);
        }
    });
}

/// `NiLight::IsEqual` (Xbox PDB, `thiscall`, the other object).
const NI_LIGHT_IS_EQUAL: u32 = 0x00a7_2160;
/// `00aa13e0` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE: u32 = 0x00aa_13e0;
/// `00a71f60` (`thiscall`, unit `nilight.cpp`): the light base constructor.
const NI_LIGHT_CONSTRUCTOR: u32 = 0x00a7_1f60;
/// `00a71ec0` (`thiscall`, (clone, cloning process); unnamed in the engine map): the light
/// base's member copy.
const NI_LIGHT_COPY_MEMBERS: u32 = 0x00a7_1ec0;
/// `NiPointLight`'s vtable (its RTTI names `NiPointLight`).
const NI_POINT_LIGHT_VTABLE: u32 = 0x0109_dd0c;
/// `NiLight::SaveBinary` (Xbox PDB, `thiscall`, stream).
const NI_LIGHT_SAVE_BINARY: u32 = 0x00a7_20f0;
/// `NiStream`'s output file (+0x250): write function at +0xc, position at +4.
const STREAM_OUTPUT: u32 = 0x250;
/// `NiLight::GetViewerStrings` (Xbox PDB, `thiscall`, the string array).
const NI_LIGHT_GET_VIEWER_STRINGS: u32 = 0x00a7_21e0;
/// `NiGetViewerString` (Xbox PDB, `cdecl`, one word) and its `float` overload
/// `NiGetViewerString_ov9` (`cdecl`, name and the float's bits).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
/// `0096ad30` (`thiscall`, new capacity) and `0096ae90` (`thiscall`, index, pointer to the
/// element): the viewer-string array's set-size and set-at.
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `"m_fAtten0"`, `"m_fAtten1"`, `"m_fAtten2"`.
const NAME_ATTEN0: u32 = 0x0109_de00;
const NAME_ATTEN1: u32 = 0x0109_ddf4;
const NAME_ATTEN2: u32 = 0x0109_dde8;

// Translated from 00a7d730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointLight::IsEqual` (Xbox PDB): `NiLight::IsEqual`, then the three attenuations
/// (+0xF0, +0xF4, +0xF8) compared in turn (`FUCOMPP`, equal only when ordered and equal, so
/// a NaN is never equal).
pub fn ni_point_light_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(NI_LIGHT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    for offset in [0xf0u32, 0xf4, 0xf8] {
        let mine = x87_load32(e.mem.u32(this.addr().wrapping_add(offset)));
        let theirs = x87_load32(e.mem.u32(other.addr().wrapping_add(offset)));
        if theirs != mine {
            return false;
        }
    }
    true
}

// Translated from 00a7d7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointLight::CreateClone` (Xbox PDB): a new point light (0xfc bytes from `00aa13e0`;
/// the light base constructor, the vtable `0109dd0c`, the byte +0x9D set to 2, attenuations
/// 0, 1, 0; null when the allocation fails), given the base's members (`00a71ec0`, called
/// on null too) and this one's three attenuations (`FLD`/`FSTP` copies, written even through
/// a null clone, as the exe does). Answers the clone.
pub fn ni_point_light_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let block = e.call(ALLOCATE, &args![0xfcu32]).u32();
    let clone = if block == 0 {
        0
    } else {
        e.call(NI_LIGHT_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block + 0xf0, x87_store32(0.0));
        e.mem.set_u32(block, NI_POINT_LIGHT_VTABLE);
        e.mem.set_u8(block + 0x9d, 2);
        e.mem.set_u32(block + 0xf4, x87_store32(1.0));
        e.mem.set_u32(block + 0xf8, x87_store32(0.0));
        block
    };
    e.call(NI_LIGHT_COPY_MEMBERS, &args![this, clone, cloning_process]);
    for offset in [0xf0u32, 0xf4, 0xf8] {
        let value = x87_quiet_bits(e.mem.u32(this.addr().wrapping_add(offset)));
        e.mem.set_u32(clone.wrapping_add(offset), value);
    }
    Ptr::new(clone)
}

// Translated from 00a7d8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointLight::SaveBinary` (Xbox PDB): `NiLight::SaveBinary`, then writes the three
/// 4-byte attenuations (+0xF0, +0xF4, +0xF8) through the stream output's write function
/// (cdecl: file, source, 4, byte-count word, 1), each adding the answer to the output's
/// position. The stream argument's stack word is the byte-count word.
pub fn ni_point_light_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_LIGHT_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        for offset in [0xf0u32, 0xf4, 0xf8] {
            let output = e.mem.u32(stream.addr().wrapping_add(STREAM_OUTPUT));
            let write = e.mem.u32(output.wrapping_add(0xc));
            e.mem.set_u32(count.addr(), 4);
            let source = this.addr().wrapping_add(offset);
            let done = e
                .call(write, &args![output, source, 4u32, count, 1u32])
                .u32();
            let position = e.mem.u32(output.wrapping_add(4)).wrapping_add(done);
            e.mem.set_u32(output.wrapping_add(4), position);
        }
    });
}

/// Appends the string pointer held in the word at `slot` to the viewer-string array: when
/// the count (`m_usSize`, +0xa) is not below the capacity (`m_usMaxSize`, +8) the array first
/// grows to count + `m_usGrowBy` (+0xe, `0096ad30`); then `0096ae90` stores it at the old
/// count.
fn append_viewer_string(e: &mut Engine, strings: Ptr, slot: u32) {
    let count = e.mem.u16(strings.addr() + 0xa) as u32;
    let capacity = e.mem.u16(strings.addr() + 8) as u32;
    if count >= capacity {
        let grow_by = e.mem.u16(strings.addr() + 0xe) as u32;
        e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
    }
    e.call(ARRAY_SET_AT, &args![strings, count, slot]);
}

// Translated from 00a7d960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointLight::GetViewerStrings` (Xbox PDB): `NiLight::GetViewerStrings`, then four
/// strings appended to `strings`: `NiGetViewerString` of the class name (the first word of
/// `NiPointLight::ms_RTTI`, `011f4a98`) and `m_fAtten0`..`m_fAtten2` (`NiGetViewerString_ov9`
/// of each attenuation, pushed through `FLD`/`FSTP`). Each string pointer is kept in the
/// `strings` argument's stack word, whose address is what the array's set-at receives.
pub fn ni_point_light_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(NI_LIGHT_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.global::<u32>(NI_POINT_LIGHT_RTTI);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);
        for (name, offset) in [
            (NAME_ATTEN0, 0xf0u32),
            (NAME_ATTEN1, 0xf4),
            (NAME_ATTEN2, 0xf8),
        ] {
            let value = x87_quiet_bits(e.mem.u32(this.addr().wrapping_add(offset)));
            let text = e.call(GET_VIEWER_STRING_FLOAT, &args![name, value]).u32();
            e.mem.set_u32(slot, text);
            append_viewer_string(e, strings, slot);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7d790, ni_point_light_get_rtti() -> Ptr),
        entry!(0x00a7_d7a0, ni_point_light_update_world_data(Ptr, Ptr)),
        entry!(0x00a7_d840, ni_point_light_load_binary(Ptr, Ptr)),
        entry!(0x00a7_d730, ni_point_light_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a7_d7c0, ni_point_light_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a7_d8d0, ni_point_light_save_binary(Ptr, Ptr)),
        entry!(0x00a7_d960, ni_point_light_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_point_light_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a7d790, &args![]).u32(), NI_POINT_LIGHT_RTTI);
    }

    #[test]
    fn update_world_data_bumps_the_counter_at_a8() {
        let mut e = Engine::new();
        e.register(NI_AV_OBJECT_UPDATE_WORLD_DATA, |_, _| Ret::default());
        let this = e.mem.alloc(0xfc);
        e.mem.set_u32(this + 0xa8, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x00a7_d7a0, &args![this, 0x4444u32]);
        assert_eq!(e.mem.u32(this + 0xa8), 0);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (NI_AV_OBJECT_UPDATE_WORLD_DATA, vec![this, 0x4444])
        );
    }

    #[test]
    fn load_binary_reads_the_three_attenuations() {
        let mut e = Engine::new();
        e.register(NI_LIGHT_LOAD_BINARY, |_, _| Ret::default());
        // The read function fills the destination with 4 bytes of 0x11 * n
        // and answers 4.
        let mut n = 0u8;
        e.register_double(0x7000_0008, move |e, a| {
            n += 1;
            assert_eq!(e.mem.u32(a[3]), 4);
            assert_eq!(a[4], 1);
            e.mem.write(a[1], &[0x11 * n; 4]);
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, 0x7000_0008);
        e.mem.set_u32(input + 4, 0x100);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, input);
        let this = e.mem.alloc(0xfc);
        e.call(0x00a7_d840, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0xf0), 0x1111_1111);
        assert_eq!(e.mem.u32(this + 0xf4), 0x2222_2222);
        assert_eq!(e.mem.u32(this + 0xf8), 0x3333_3333);
        assert_eq!(e.mem.u32(input + 4), 0x10c);
    }
}

#[cfg(test)]
mod n018_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    fn light(e: &mut Engine, attenuations: [f32; 3]) -> u32 {
        let light = e.mem.alloc(0xfc);
        for (i, value) in attenuations.iter().enumerate() {
            e.mem.set_f32(light + 0xf0 + 4 * i as u32, *value);
        }
        light
    }

    #[test]
    fn is_equal_needs_the_base_and_all_three_attenuations() {
        let mut e = Engine::new();
        e.register(NI_LIGHT_IS_EQUAL, |_, _| 1u32.into_ret());
        let this = light(&mut e, [1.0, 2.0, 3.0]);
        let same = light(&mut e, [1.0, 2.0, 3.0]);
        assert!(e.call(0x00a7_d730, &args![this, same]).bool());
        for i in 0..3 {
            let mut values = [1.0, 2.0, 3.0];
            values[i] = 9.0;
            let other = light(&mut e, values);
            assert!(!e.call(0x00a7_d730, &args![this, other]).bool(), "{i}");
        }
        let nan = light(&mut e, [f32::NAN, 2.0, 3.0]);
        assert!(!e.call(0x00a7_d730, &args![nan, nan]).bool());
        e.register(NI_LIGHT_IS_EQUAL, |_, _| 0x100u32.into_ret());
        assert!(!e.call(0x00a7_d730, &args![this, same]).bool());
    }

    #[test]
    fn create_clone_copies_the_attenuations() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_LIGHT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(NI_LIGHT_COPY_MEMBERS, |e, a| {
            // The defaults are in place when the base copy runs.
            assert_eq!(e.mem.f32(a[1] + 0xf4), 1.0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = light(&mut e, [0.5, 0.25, 0.125]);
        // A signalling NaN comes out quiet.
        e.mem.set_u32(this + 0xf8, 0x7f80_0001);
        let clone = e.call(0x00a7_d7c0, &args![this, 0x4444u32]).u32();
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0xfc]]);
        assert_eq!(calls_to(&e, NI_LIGHT_CONSTRUCTOR), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, NI_LIGHT_COPY_MEMBERS),
            vec![vec![this, clone, 0x4444]]
        );
        assert_eq!(e.mem.u32(clone), NI_POINT_LIGHT_VTABLE);
        assert_eq!(e.mem.u8(clone + 0x9d), 2);
        assert_eq!(e.mem.f32(clone + 0xf0), 0.5);
        assert_eq!(e.mem.f32(clone + 0xf4), 0.25);
        assert_eq!(e.mem.u32(clone + 0xf8), 0x7fc0_0001);
    }

    #[test]
    fn save_binary_writes_the_three_attenuations() {
        let mut e = Engine::new();
        e.register(NI_LIGHT_SAVE_BINARY, |_, _| Ret::default());
        e.register(0x7000_000c, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            a[2].into_ret()
        });
        e.call_log = Some(vec![]);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, 0x7000_000c);
        e.mem.set_u32(output + 4, 0x20);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + 0x250, output);
        let this = light(&mut e, [1.0, 2.0, 3.0]);
        e.call(0x00a7_d8d0, &args![this, stream]);
        assert_eq!(calls_to(&e, NI_LIGHT_SAVE_BINARY), vec![vec![this, stream]]);
        let writes = calls_to(&e, 0x7000_000c);
        let sources: Vec<_> = writes.iter().map(|w| (w[0], w[1], w[2], w[4])).collect();
        assert_eq!(
            sources,
            vec![
                (output, this + 0xf0, 4, 1),
                (output, this + 0xf4, 4, 1),
                (output, this + 0xf8, 4, 1)
            ]
        );
        assert_eq!(e.mem.u32(output + 4), 0x2c);
    }

    #[test]
    fn viewer_strings_list_the_class_and_attenuations() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(NI_POINT_LIGHT_RTTI, 0x0109_d000);
        e.register(NI_LIGHT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x5000u32.into_ret());
        e.register_double(GET_VIEWER_STRING_FLOAT, {
            let mut n = 0u32;
            move |_, _| {
                n += 1;
                (0x6000 + n).into_ret()
            }
        });
        e.register(ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xe, 2);
        let this = light(&mut e, [1.0, 0.5, 0.25]);
        e.call(0x00a7_d960, &args![this, strings]);
        assert_eq!(
            calls_to(&e, NI_LIGHT_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x0109_d000]]);
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_FLOAT),
            vec![
                vec![NAME_ATTEN0, 1.0f32.to_bits()],
                vec![NAME_ATTEN1, 0.5f32.to_bits()],
                vec![NAME_ATTEN2, 0.25f32.to_bits()]
            ]
        );
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), vec![vec![strings, 4]]);
        let held: Vec<u32> = (0..4).map(|i| e.mem.u32(base + 4 * i)).collect();
        assert_eq!(held, vec![0x5000, 0x6001, 0x6002, 0x6003]);
    }
}
