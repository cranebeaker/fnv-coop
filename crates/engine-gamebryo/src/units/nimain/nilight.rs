//! `NiMain/nilight.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiLight::ms_RTTI` (the class's `NiRTTI` record).
const NI_LIGHT_RTTI: u32 = 0x011f_4a28;

// Translated from 00a72000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLight::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_light_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_LIGHT_RTTI)
}

/// `00a69070`: the jump target of `00a71f50`.
const JUMP_TARGET_00A71F50: u32 = 0x00a6_9070;

/// `00a6ff10`: the jump target of `00a90010`.
const JUMP_TARGET_00A90010: u32 = 0x00a6_ff10;

// Translated from 00a71f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLight::LinkObject` (Xbox PDB): a 5-byte jump to `00a69070` (engine map: `NiDynamicEffect::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiLight@@UAAXAAVNiStream@@@Z`.
pub fn ni_light_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A71F50, &args![this, stream]);
}

// Translated from 00a90010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLight::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a6ff10` (engine map: `NiCamera::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiLight@@UAA_NAAVNiStream@@@Z`.
pub fn ni_light_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A90010, &args![this, stream]).bool()
}

/// `NiDynamicEffect::NiDynamicEffect` (engine map, `00a69320`, `thiscall`, no words).
const NI_DYNAMIC_EFFECT_CONSTRUCT: u32 = 0x00a6_9320;
/// `NiLight`'s vtable (`0109ce4c`, named by its RTTI locator).
const NI_LIGHT_VTABLE: u32 = 0x0109_ce4c;

// Translated from 00a71f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `NiMain/nilight.cpp`): the `NiLight` constructor by its
/// body (it sets the vtable whose RTTI locator names `NiLight`). `thiscall`, no words. Runs
/// `NiDynamicEffect::NiDynamicEffect` (`00a69320`), sets the vtable `0109ce4c`, stores 0.0 in
/// the nine floats +0xc8..+0xe8 (the Xbox PDB's ambient, diffuse and specular `NiColor`s,
/// `m_kAmb`/`m_kDiff`/`m_kSpec`, 0x2c lower on PC), clears the word at +0xec
/// (`m_pvRendererData`), then stores 1.0 in the ten floats +0xc4..+0xe8 (`m_fDimmer` and the
/// three colours), in the exe's order. Answers `this`.
pub fn fn_00a71f60(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_DYNAMIC_EFFECT_CONSTRUCT, &args![this]);
    let base = this.addr();
    e.mem.set_u32(base, NI_LIGHT_VTABLE);
    for offset in (0xc8..=0xe8).step_by(4) {
        e.mem.set_f32(base.wrapping_add(offset), 0.0);
    }
    e.mem.set_u32(base.wrapping_add(0xec), 0);
    for offset in (0xc4..=0xe8).step_by(4) {
        e.mem.set_f32(base.wrapping_add(offset), 1.0);
    }
    this
}

/// The global object `NiLight::~NiLight` notifies (`011f4748`; called through
/// its vtable slot `+0xf8` with the light).
const LIGHT_LISTENER: u32 = 0x011f_4748;
/// `00a69030` (`nidynamiceffect.cpp`, `thiscall`, no stack words): the
/// `NiDynamicEffect` destructor body.
const NI_DYNAMIC_EFFECT_DESTRUCT: u32 = 0x00a6_9030;

// Translated from 00a72010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLight::~NiLight` (Xbox PDB), `thiscall`, no stack words: stores
/// `NiLight`'s vtable, tells the object in `011f4748` (when there is one)
/// through its vtable slot `+0xf8` with the light, then jumps to the
/// `NiDynamicEffect` destructor body `00a69030`.
pub fn ni_light_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_LIGHT_VTABLE);
    let listener = e.global::<u32>(LIGHT_LISTENER);
    if listener != 0 {
        e.vcall(listener, 0xf8, &args![this]);
    }
    e.call(NI_DYNAMIC_EFFECT_DESTRUCT, &args![this]);
}

/// `NiDynamicEffect::LoadBinary` (`00a68ea0`, `thiscall`, the stream).
const NI_DYNAMIC_EFFECT_LOAD_BINARY: u32 = 0x00a6_8ea0;
/// `NiColor::LoadBinary` (`00a59be0`, `thiscall` on the colour, the stream).
const NI_COLOR_LOAD_BINARY: u32 = 0x00a5_9be0;

// Translated from 00a72080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLight::LoadBinary` (Xbox PDB), `thiscall`, one stack word (the stream):
/// `NiDynamicEffect::LoadBinary`, then one 4-byte item into the dimmer at
/// `+0xc4` through the stream's binary reader (the object at `stream+0x24c`:
/// its function at `+8` is called `cdecl` with the reader, the destination, 4,
/// the address of a word holding 4 (the argument's own stack slot) and 1, and
/// its result is added to the reader's `+4`), then the three colours at
/// `+0xc8`, `+0xd4` and `+0xe0` (`NiColor::LoadBinary`).
pub fn ni_light_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let t = this.addr();
    e.call(NI_DYNAMIC_EFFECT_LOAD_BINARY, &args![this, stream]);
    // NiStream +0x24c: the binary reader.
    let reader = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(reader + 8);
    let done = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 4);
        e.call(read, &args![reader, t + 0xc4, 4u32, slot, 1u32])
            .u32()
    });
    let total = e.mem.u32(reader + 4).wrapping_add(done);
    e.mem.set_u32(reader + 4, total);
    for colour in [0xc8u32, 0xd4, 0xe0] {
        e.call(NI_COLOR_LOAD_BINARY, &args![t + colour, stream]);
    }
}

/// The sized delete (`00aa1460`, `cdecl`: block, size).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiDynamicEffect::SaveBinary` (`00a68f80`, the stream), `NiDynamicEffect::IsEqual`
/// (`00a68d80`, the other object) and `NiDynamicEffect::GetViewerStrings` (`00a69140`, the
/// string array), all `thiscall`.
const NI_DYNAMIC_EFFECT_SAVE_BINARY: u32 = 0x00a6_8f80;
const NI_DYNAMIC_EFFECT_IS_EQUAL: u32 = 0x00a6_8d80;
const NI_DYNAMIC_EFFECT_GET_VIEWER_STRINGS: u32 = 0x00a6_9140;
/// `NiColor::SaveBinary` (`00a69780`, the stream) and `NiColor::GetViewerString`
/// (`00a696c0`, a name, answering a new string), both `thiscall` on the colour.
const NI_COLOR_SAVE_BINARY: u32 = 0x00a6_9780;
const NI_COLOR_GET_VIEWER_STRING: u32 = 0x00a6_96c0;
/// `00439090` (`thiscall` on a colour, the other colour; unit `multiboundmarkerdata.cpp` in
/// the engine map): nonzero when the two colours differ.
const COLORS_DIFFER_00439090: u32 = 0x0043_9090;
/// `NiGetViewerString` (`00a7d330`, the class name) and `NiGetViewerString_ov9`
/// (`00a7d550`, a name and a float), both `cdecl`, answering a new string.
const VIEWER_STRING_CLASS: u32 = 0x00a7_d330;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
/// The string array's grow (`0096ad30`, `thiscall`, the new size) and set-at (`0096ae90`,
/// `thiscall`, (index, address of the string)), both unnamed in the engine map.
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The literals `"m_fDimmer"`, `"m_kAmb"`, `"m_kDiff"` and `"m_kSpec"`.
const TEXT_M_F_DIMMER: u32 = 0x0109_cf40;
const TEXT_M_K_AMB: u32 = 0x0109_cf38;
const TEXT_M_K_DIFF: u32 = 0x0109_cf30;
const TEXT_M_K_SPEC: u32 = 0x0109_cf28;
/// `NiLight` (Xbox PDB): the dimmer `m_fDimmer` (+0xc4) and the ambient, diffuse and
/// specular colours `m_kAmb`, `m_kDiff`, `m_kSpec` (three floats each at +0xc8, +0xd4,
/// +0xe0).
const LIGHT_DIMMER: u32 = 0xc4;
const LIGHT_COLORS: [u32; 3] = [0xc8, 0xd4, 0xe0];

/// Appends `text` to a viewer-string array as the exe's inlined `Add` does: when the 16-bit
/// size (+0xa) has reached the capacity (+0x8) the array grows to its step (+0xe) plus the
/// size, then the string is stored at the old size (through a stack word holding it).
fn push_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00a72040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLight`'s scalar deleting destructor (unnamed in the engine map; `thiscall`, the
/// flags): the destructor body (vtable, the light listener `011f4748`'s slot +0xf8, the
/// `NiDynamicEffect` destructor) inlined, then the sized delete of 0xf0 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn fn_00a72040(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    ni_light_destructor(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xf0u32]);
    }
    this
}

// Translated from 00a720f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLight::SaveBinary` (Xbox PDB): `NiDynamicEffect::SaveBinary`, the dimmer (4 bytes
/// through the stream's output `m_pkOstr`, its write function with a component size word
/// of 4 and one component, the position advanced by the result), then the three colours
/// save themselves (`NiColor::SaveBinary`).
pub fn ni_light_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let t = this.addr();
    e.call(NI_DYNAMIC_EFFECT_SAVE_BINARY, &args![this, stream]);
    let writer = e.mem.u32(stream.addr() + 0x250);
    let write = e.mem.u32(writer + 0xc);
    let done = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 4);
        e.call(write, &args![writer, t + LIGHT_DIMMER, 4u32, slot, 1u32])
            .u32()
    });
    let total = e.mem.u32(writer + 4).wrapping_add(done);
    e.mem.set_u32(writer + 4, total);
    for colour in LIGHT_COLORS {
        e.call(NI_COLOR_SAVE_BINARY, &args![t + colour, stream]);
    }
}

// Translated from 00a72160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLight::IsEqual` (Xbox PDB): true when `NiDynamicEffect::IsEqual` holds, the dimmers
/// compare equal (`FUCOMPP`, a NaN is not equal) and none of the three colours differs
/// (`00439090`, checked in order, stopping at the first that differs).
pub fn ni_light_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e
        .call(NI_DYNAMIC_EFFECT_IS_EQUAL, &args![this, other])
        .bool()
    {
        return false;
    }
    let t = this.addr();
    let o = other.addr();
    let mine = x87_load32(e.mem.u32(t + LIGHT_DIMMER));
    let theirs = x87_load32(e.mem.u32(o.wrapping_add(LIGHT_DIMMER)));
    if theirs != mine {
        return false;
    }
    for colour in LIGHT_COLORS {
        if e.call(
            COLORS_DIFFER_00439090,
            &args![t + colour, o.wrapping_add(colour)],
        )
        .bool()
        {
            return false;
        }
    }
    true
}

// Translated from 00a721e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLight::GetViewerStrings` (Xbox PDB): `NiDynamicEffect::GetViewerStrings`, then the
/// class name (`NiGetViewerString` of `ms_RTTI`'s name word), `"m_fDimmer"` with the
/// dimmer and the three colours (`NiColor::GetViewerString` with `"m_kAmb"`, `"m_kDiff"`,
/// `"m_kSpec"`), each appended to the array.
pub fn ni_light_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let t = this.addr();
    e.call(NI_DYNAMIC_EFFECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(NI_LIGHT_RTTI);
    let text = e.call(VIEWER_STRING_CLASS, &args![name]).u32();
    push_viewer_string(e, strings, text);
    let dimmer = x87_quiet_bits(e.mem.u32(t + LIGHT_DIMMER));
    let text = e
        .call(VIEWER_STRING_FLOAT, &args![TEXT_M_F_DIMMER, dimmer])
        .u32();
    push_viewer_string(e, strings, text);
    for (colour, label) in
        LIGHT_COLORS
            .into_iter()
            .zip([TEXT_M_K_AMB, TEXT_M_K_DIFF, TEXT_M_K_SPEC])
    {
        let text = e
            .call(NI_COLOR_GET_VIEWER_STRING, &args![t + colour, label])
            .u32();
        push_viewer_string(e, strings, text);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_1f60, fn_00a71f60(Ptr) -> Ptr),
        entry!(0x00a72000, ni_light_get_rtti() -> Ptr),
        entry!(0x00a7_1f50, ni_light_link_object(Ptr, Ptr)),
        entry!(0x00a9_0010, ni_light_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a7_2010, ni_light_destructor(Ptr)),
        entry!(0x00a7_2080, ni_light_load_binary(Ptr, Ptr)),
        entry!(0x00a7_2040, fn_00a72040(Ptr, u32) -> Ptr),
        entry!(0x00a7_20f0, ni_light_save_binary(Ptr, Ptr)),
        entry!(0x00a7_2160, ni_light_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a7_21e0, ni_light_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_light_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a72000, &args![]).u32(), NI_LIGHT_RTTI);
    }

    #[test]
    fn thunk_00a71f50_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_1f50;
        const TARGET: u32 = 0x00a6_9070;
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
    fn thunk_00a90010_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_0010;
        const TARGET: u32 = 0x00a6_ff10;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn fn_00a71f60_sets_the_dimmer_and_colours_to_one() {
        let mut e = Engine::new();
        e.register(NI_DYNAMIC_EFFECT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            e.mem.set_u32(a[0] + 0xec, 0xdead);
            Ret::default()
        });
        let this = e.mem.alloc(0xf0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_1f60, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_DYNAMIC_EFFECT_CONSTRUCT)
            .collect();
        assert_eq!(base.len(), 1);
        assert_eq!(base[0].1, [this]);
        assert_eq!(e.mem.u32(this), NI_LIGHT_VTABLE);
        for offset in (0xc4..=0xe8).step_by(4) {
            assert_eq!(e.mem.f32(this + offset).to_bits(), 1.0f32.to_bits());
        }
        assert_eq!(e.mem.u32(this + 0xec), 0);
    }
}

#[cfg(test)]
mod tests_b008 {
    use super::*;

    #[test]
    fn ni_light_destructor_notifies_the_listener_when_there_is_one() {
        const NOTIFY: u32 = 0x0123_4000;
        let mut e = Engine::new();
        e.map(LIGHT_LISTENER, 4);
        e.register(NOTIFY, |_, _| Ret::default());
        e.register(NI_DYNAMIC_EFFECT_DESTRUCT, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0xf8, NOTIFY);
        let listener = e.mem.alloc(4);
        e.mem.set_u32(listener, vtable);
        let this = e.mem.alloc(0xfc);
        let run = |e: &mut Engine| -> Vec<(u32, Vec<u32>)> {
            e.call_log = Some(vec![]);
            e.call(0x00a7_2010, &args![this]);
            e.call_log.take().unwrap()[1..].to_vec()
        };
        assert_eq!(run(&mut e), vec![(NI_DYNAMIC_EFFECT_DESTRUCT, vec![this])]);
        assert_eq!(e.mem.u32(this), NI_LIGHT_VTABLE);
        e.set_global::<u32>(LIGHT_LISTENER, listener);
        assert_eq!(
            run(&mut e),
            vec![
                (NOTIFY, vec![listener, this]),
                (NI_DYNAMIC_EFFECT_DESTRUCT, vec![this])
            ]
        );
    }

    #[test]
    fn ni_light_load_binary_reads_the_dimmer_and_three_colours() {
        const READ: u32 = 0x0123_5000;
        let mut e = Engine::new();
        e.register(NI_DYNAMIC_EFFECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(NI_COLOR_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], 0x3f00_0000);
            4u32.into_ret()
        });
        let reader = e.mem.alloc(0x10);
        e.mem.set_u32(reader + 8, READ);
        e.mem.set_u32(reader + 4, 100);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, reader);
        let this = e.mem.alloc(0xfc);
        e.call_log = Some(vec![]);
        e.call(0x00a7_2080, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0xc4), 0x3f00_0000);
        assert_eq!(e.mem.u32(reader + 4), 104);
        let log: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .map(|c| c.0)
            .collect();
        assert_eq!(
            log,
            vec![
                0x00a7_2080,
                NI_DYNAMIC_EFFECT_LOAD_BINARY,
                READ,
                NI_COLOR_LOAD_BINARY,
                NI_COLOR_LOAD_BINARY,
                NI_COLOR_LOAD_BINARY
            ]
        );
    }
}

#[cfg(test)]
mod n016_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        for double in [
            SIZED_DELETE,
            NI_DYNAMIC_EFFECT_DESTRUCT,
            NI_DYNAMIC_EFFECT_SAVE_BINARY,
            NI_DYNAMIC_EFFECT_GET_VIEWER_STRINGS,
            NI_COLOR_SAVE_BINARY,
            STRINGS_GROW,
            STRINGS_SET_AT,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn scalar_deleting_destructor_tells_the_listener_and_deletes_on_bit_0() {
        let mut e = engine();
        let listener = e.mem.alloc(4);
        let mut slots = vec![0u32; 0x40];
        slots[0xf8 / 4] = 0x0e00_00f8;
        e.put_vtable(0x0e10_0000, &slots);
        e.mem.set_u32(listener, 0x0e10_0000);
        e.register(0x0e00_00f8, |_, _| Ret::default());
        e.mem.set_u32(LIGHT_LISTENER, listener);
        let this = e.mem.alloc(0xf0);
        assert_eq!(e.call(0x00a7_2040, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_LIGHT_VTABLE);
        assert_eq!(calls_to(&e, 0x0e00_00f8), vec![vec![listener, this]]);
        assert_eq!(calls_to(&e, NI_DYNAMIC_EFFECT_DESTRUCT), vec![vec![this]]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0xf0]]);
        e.mem.set_u32(LIGHT_LISTENER, 0);
        e.call(0x00a7_2040, &args![this, 0u32]);
        assert_eq!(calls_to(&e, 0x0e00_00f8).len(), 1);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
        assert_eq!(calls_to(&e, NI_DYNAMIC_EFFECT_DESTRUCT).len(), 2);
    }

    #[test]
    fn save_binary_writes_the_dimmer_and_colours() {
        let mut e = engine();
        let stream = e.mem.alloc(0x260);
        let writer = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x250, writer);
        e.mem.set_u32(writer + 0xc, 0x0300_0000);
        e.register(0x0300_0000, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            4u32.into_ret()
        });
        let this = e.mem.alloc(0xf0);
        e.call(0x00a7_20f0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, NI_DYNAMIC_EFFECT_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(calls_to(&e, 0x0300_0000)[0][..2], [writer, this + 0xc4]);
        assert_eq!(e.mem.u32(writer + 4), 4);
        assert_eq!(
            calls_to(&e, NI_COLOR_SAVE_BINARY),
            vec![
                vec![this + 0xc8, stream],
                vec![this + 0xd4, stream],
                vec![this + 0xe0, stream]
            ]
        );
    }

    #[test]
    fn is_equal_checks_the_base_dimmer_and_colours() {
        let mut e = engine();
        e.register(NI_DYNAMIC_EFFECT_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(COLORS_DIFFER_00439090, |e, a| {
            u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        let this = e.mem.alloc(0xf0);
        let other = e.mem.alloc(0xf0);
        e.mem.set_f32(this + 0xc4, 0.5);
        e.mem.set_f32(other + 0xc4, 0.5);
        assert!(e.call(0x00a7_2160, &args![this, other]).bool());
        assert_eq!(calls_to(&e, COLORS_DIFFER_00439090).len(), 3);
        e.mem.set_f32(other + 0xd4, 1.0);
        assert!(!e.call(0x00a7_2160, &args![this, other]).bool());
        assert_eq!(calls_to(&e, COLORS_DIFFER_00439090).len(), 5);
        e.mem.set_f32(other + 0xd4, 0.0);
        e.mem.set_f32(other + 0xc4, 0.25);
        assert!(!e.call(0x00a7_2160, &args![this, other]).bool());
        assert_eq!(calls_to(&e, COLORS_DIFFER_00439090).len(), 5);
        e.register(NI_DYNAMIC_EFFECT_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a7_2160, &args![this, this]).bool());
    }

    #[test]
    fn get_viewer_strings_lists_the_dimmer_and_colours() {
        let mut e = engine();
        e.mem.set_u32(NI_LIGHT_RTTI, 0x0123_0000);
        e.register(VIEWER_STRING_CLASS, |_, _| 0x0e00_0001u32.into_ret());
        e.register(VIEWER_STRING_FLOAT, |_, _| 0x0e00_0002u32.into_ret());
        e.register(NI_COLOR_GET_VIEWER_STRING, |_, _| 0x0e00_0003u32.into_ret());
        let this = e.mem.alloc(0xf0);
        e.mem.set_f32(this + 0xc4, 0.75);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 8);
        e.call(0x00a7_21e0, &args![this, strings]);
        assert_eq!(
            calls_to(&e, NI_DYNAMIC_EFFECT_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls_to(&e, VIEWER_STRING_CLASS), vec![vec![0x0123_0000]]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_FLOAT),
            vec![vec![TEXT_M_F_DIMMER, 0.75f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, NI_COLOR_GET_VIEWER_STRING),
            vec![
                vec![this + 0xc8, TEXT_M_K_AMB],
                vec![this + 0xd4, TEXT_M_K_DIFF],
                vec![this + 0xe0, TEXT_M_K_SPEC]
            ]
        );
        assert_eq!(calls_to(&e, STRINGS_SET_AT).len(), 5);
    }
}
