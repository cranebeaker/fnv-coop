//! `fallout shared/run_00599550` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00599c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00599c40 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x72 (0x30 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_449c`) is `TESAmmoEffect`, so this is the constructor of `TESAmmoEffect`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00599c40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_449c);
    e.mem.set_u32(base + 0x18, 0x0103_4484);
    // memset wrapper (cdecl, three words): the 0xC bytes at +0x24 cleared.
    e.call(0x0040_3d30, &args![base + 0x24, 0x0u32, 0xcu32]);
    e.call(0x004f_15a0, &args![this, 0x72u32]);
    this
}

// Translated from 00599cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00599d00` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00599cd0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_9d00, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The media set's track reset (`00597850`, on a track).
const TRACK_RESET: u32 = 0x0059_7850;
/// The media set's six tracks.
const TRACKS: [u32; 6] = [0x48, 0x58, 0x68, 0x78, 0x88, 0x98];
/// The list helpers on a node: its item's slot (`006815c0`) and the next
/// node (`00726070`).
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;

// Translated from 00599b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets a `MediaSet` (the engine map has no name): its six tracks
/// (`00597850`), the four floats at +0xAC .. +0xB8 to 0.0 and the two
/// sound words at +0xBC and +0xC0 to 0.
pub fn fn_00599b30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for track in TRACKS {
        e.call(TRACK_RESET, &args![base + track]);
    }
    for offset in [0xacu32, 0xb0, 0xb4, 0xb8] {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0xbc, 0);
    e.mem.set_u32(base + 0xc0, 0);
}

// Translated from 00599bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets a `MediaSet`'s six tracks (`00597850`) and its two sound words
/// at +0xBC and +0xC0 (the engine map has no name).
pub fn fn_00599bd0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for track in TRACKS {
        e.call(TRACK_RESET, &args![base + track]);
    }
    e.mem.set_u32(base + 0xbc, 0);
    e.mem.set_u32(base + 0xc0, 0);
}

// Translated from 00599d00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00599d00 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor body of `TESAmmoEffect` (the vtables `00599c40` stores):
/// puts back the vtables, destroys the name at +0x18 (`00591300`) and the
/// `TESForm` base (`00483630`).
pub fn fn_00599d00(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_449c);
    e.mem.set_u32(base + 0x18, 0x0103_4484);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 00599d70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00599d70 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESAmmoEffect`'s record reader (`RET 4`: the `TESFile`; the engine map
/// has no name): false unless the form type byte is 0x72; otherwise
/// `TESForm::LoadForm`, `00484ab0(0)` and each chunk: `EDID` (vtable
/// +0x134), `OBND` (vtable +0xE0), `FULL` (+0x18) and `DATA` (0xC bytes at
/// +0x24, swapped by `005a6b10`). Returns true.
pub fn fn_00599d70(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let form_type = e.call(0x0047_2660, &args![file]).u8();
    if form_type != 0x72 {
        return false;
    }
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            // OBND
            0x444e_424f => {
                e.vcall(base, 0xe0, &args![file]);
            }
            // DATA
            0x4154_4144 => {
                e.call(0x0048_61f0, &args![this, file, base + 0x24, 0xcu32]);
                if e.call(0x0040_1680, &args![file]).bool() {
                    e.call(0x005a_6b10, &args![base + 0x24]);
                }
            }
            // EDID
            0x4449_4445 => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(0x0047_2890, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            // FULL
            0x4c4c_5546 => {
                let full = if this.is_null() { 0 } else { base + 0x18 };
                e.call(0x0048_7050, &args![full, file]);
            }
            _ => {}
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 0059a030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059a030 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Applies the ammo effects of the list `effects` of type `kind` to
/// `value` (cdecl; the engine map has no name): the effects are first
/// copied into a new list (8 bytes, `0096a2d0`; those whose operation,
/// `0045cd60`, is 1 through `005ae3d0`, the others through `00905820`),
/// then each effect whose type (`0059bb30`) is `kind` adds (operation 0),
/// multiplies by (1) or subtracts (2) its amount (`00759450`) in x87
/// arithmetic stored as a `float`; the copy is deleted (`004702f0(1)`). No
/// list: `value` itself.
pub fn fn_0059a030(e: &mut Engine, kind: u32, effects: u32, value: f32) -> f32 {
    let mut result = x87_quiet_bits(value.to_bits());
    if effects == 0 {
        return f32::from_bits(result);
    }
    let block = e.call(0x0040_1000, &args![8u32]).u32();
    let copy = if block == 0 {
        0
    } else {
        e.call(0x0096_a2d0, &args![block]).u32()
    };
    let mut node = effects;
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let effect = e.mem.u32(slot);
        let operation = e.call(0x0045_cd60, &args![effect]).u32();
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if operation == 1 {
            e.call(0x005a_e3d0, &args![copy, slot]);
        } else {
            e.call(0x0090_5820, &args![copy, slot]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let mut node = copy;
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let effect = e.mem.u32(slot);
        node = e.call(NODE_NEXT, &args![node]).u32();
        let effect_kind = e.call(0x0059_bb30, &args![effect]).u32();
        let operation = e.call(0x0045_cd60, &args![effect]).u32();
        let amount = x87_store32(e.call(0x0075_9450, &args![effect]).f64());
        if effect_kind != kind {
            continue;
        }
        let current = x87_load32(result);
        let operand = x87_m32(amount);
        match operation {
            0 => result = x87_store32(x87_add(current, operand)),
            1 => result = x87_store32(x87_mul(current, operand)),
            2 => result = x87_store32(x87_sub(current, operand)),
            _ => {}
        }
    }
    if copy != 0 {
        e.call(0x0047_02f0, &args![copy, 1u32]);
    }
    f32::from_bits(x87_quiet_bits(result))
}

/// `__RTDynamicCast` (`00ec43fb`, cdecl), the source type descriptor these
/// functions pass, and the targets: the media set's (`01186168`) and the
/// ammo effect's (`01186474`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE: u32 = 0x0118_3028;
const MEDIA_SET_TYPE: u32 = 0x0118_6168;
const AMMO_EFFECT_TYPE: u32 = 0x0118_6474;
/// `TESForm::CopyAllComponents` and `TESForm::CompareAllComponents`.
const COPY_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_COMPONENTS: u32 = 0x0048_5270;
/// A track's string: its text (`00559450`), set (`004037f0`: text,
/// length), copy (`004047f0`), compare (`BSStringT<char>::StrCmp`,
/// `00408a80`) and destructor (`007d9ec0`).
const STRING_TEXT: u32 = 0x0055_9450;
const STRING_SET: u32 = 0x0040_37f0;
const STRING_COPY: u32 = 0x0040_47f0;
const STRING_COMPARE: u32 = 0x0040_8a80;
const STRING_DESTROY: u32 = 0x007d_9ec0;

/// Puts a track passed by value (its four words) into a stack block for
/// `f`, as the exe has it in its argument area.
fn track_on_stack<R>(
    e: &mut Engine,
    text: u32,
    lengths: u32,
    volume: f32,
    other: f32,
    f: impl FnOnce(&mut Engine, u32) -> R,
) -> R {
    e.with_stack(0x10, |e, track| {
        let t = track.addr();
        e.mem.set_u32(t, text);
        e.mem.set_u32(t + 4, lengths);
        e.mem.set_u32(t + 8, volume.to_bits());
        e.mem.set_u32(t + 0xc, other.to_bits());
        f(e, t)
    })
}

/// A media set track copied into a stack temporary (`00599780`) from
/// `source`, handed to `f` as its four words.
fn copied_track<R>(e: &mut Engine, source: u32, f: impl FnOnce(&mut Engine, [u32; 4]) -> R) -> R {
    e.with_stack(0x10, |e, temp| {
        fn_00599780(e, temp, source);
        let t = temp.addr();
        let words = [
            e.mem.u32(t),
            e.mem.u32(t + 4),
            e.mem.u32(t + 8),
            e.mem.u32(t + 0xc),
        ];
        f(e, words)
    })
}

// Translated from 00599550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MediaSet`'s copy (`RET 4`: the source form; the engine map has no
/// name): the source through `__RTDynamicCast(source, 0, 01183028,
/// 01186168, 0)`; nothing without it. Otherwise the word at +0x44, each of
/// the six tracks (a copy of the source's track, `00599780`, assigned by
/// value through `00599700`), the byte at +0xA8, the floats +0xAC .. +0xB8
/// (FLD/FSTP), the sound words +0xBC and +0xC0, then
/// `TESForm::CopyAllComponents`.
pub fn fn_00599550(e: &mut Engine, this: Ptr, source: u32) {
    let base = this.addr();
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, CAST_SOURCE, MEDIA_SET_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return;
    }
    let word = e.mem.u32(other + 0x44);
    e.mem.set_u32(base + 0x44, word);
    for track in TRACKS {
        copied_track(e, other + track, |e, w| {
            fn_00599700(
                e,
                Ptr::new(base + track),
                w[0],
                w[1],
                f32::from_bits(w[2]),
                f32::from_bits(w[3]),
            );
        });
    }
    let byte = e.mem.u8(other + 0xa8);
    e.mem.set_u8(base + 0xa8, byte);
    for offset in [0xacu32, 0xb0, 0xb4, 0xb8] {
        let value = x87_quiet_bits(e.mem.u32(other + offset));
        e.mem.set_u32(base + offset, value);
    }
    for offset in [0xbcu32, 0xc0] {
        let word = e.mem.u32(other + offset);
        e.mem.set_u32(base + offset, word);
    }
    e.call(COPY_COMPONENTS, &args![this, other]);
}

// Translated from 00599700 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00599700 - the exception-unwinding frame (FS:[0]) around the by-value track is not translated.
/// A media set track's assignment from a track passed by value (`RET
/// 0x10`: its string's two words and its floats at +8 and +0xC; the engine
/// map has no name): the string set to the argument's text (`004037f0`
/// with length 0), the two floats copied (FLD/FSTP), then the argument's
/// string destroyed (`007d9ec0`), as the callee owns it.
pub fn fn_00599700(e: &mut Engine, this: Ptr, text: u32, lengths: u32, volume: f32, other: f32) {
    let base = this.addr();
    track_on_stack(e, text, lengths, volume, other, |e, track| {
        let name = e.call(STRING_TEXT, &args![track]).u32();
        e.call(STRING_SET, &args![this, name, 0u32]);
        let value = x87_quiet_bits(e.mem.u32(track + 8));
        e.mem.set_u32(base + 8, value);
        let value = x87_quiet_bits(e.mem.u32(track + 0xc));
        e.mem.set_u32(base + 0xc, value);
        e.call(STRING_DESTROY, &args![track]);
    });
}

// Translated from 00599780 (decompiled, FalloutNV.exe 1.4.0.525)
/// A media set track's copy constructor (`RET 4`: the source track; the
/// engine map has no name): the string (`004047f0`), then the floats at
/// +8 and +0xC (FLD/FSTP). Returns `this`.
pub fn fn_00599780(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    let base = this.addr();
    e.call(STRING_COPY, &args![this, source]);
    let value = x87_quiet_bits(e.mem.u32(source + 8));
    e.mem.set_u32(base + 8, value);
    let value = x87_quiet_bits(e.mem.u32(source + 0xc));
    e.mem.set_u32(base + 0xc, value);
    this
}

// Translated from 005997c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MediaSet`'s compare (`RET 4`: the other form; true when they differ;
/// the engine map has no name): the other through `__RTDynamicCast` as in
/// `00599550`; true without it, when the words at +0x44 differ, when
/// `00599a50` answers true for one of the six tracks (given a copy of the
/// other's track, `00599780`), when the bytes at +0xA8, the floats +0xAC ..
/// +0xB8 (unordered counts as different) or the words +0xBC, +0xC0 differ,
/// or when `TESForm::CompareAllComponents` answers true.
pub fn fn_005997c0(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let base = this.addr();
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, CAST_SOURCE, MEDIA_SET_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return true;
    }
    if e.mem.u32(other + 0x44) != e.mem.u32(base + 0x44) {
        return true;
    }
    for track in TRACKS {
        let differs = copied_track(e, other + track, |e, w| {
            fn_00599a50(
                e,
                Ptr::new(base + track),
                w[0],
                w[1],
                f32::from_bits(w[2]),
                f32::from_bits(w[3]),
            )
        });
        if differs {
            return true;
        }
    }
    if e.mem.i8(base + 0xa8) != e.mem.i8(other + 0xa8) {
        return true;
    }
    for offset in [0xacu32, 0xb0, 0xb4, 0xb8] {
        let mine = x87_load32(e.mem.u32(base + offset));
        let theirs = x87_load32(e.mem.u32(other + offset));
        // FUCOMPP; TEST AH,0x44; JNP: on only when equal and ordered.
        if theirs != mine {
            return true;
        }
    }
    for offset in [0xbcu32, 0xc0] {
        if e.mem.u32(base + offset) != e.mem.u32(other + offset) {
            return true;
        }
    }
    e.call(COMPARE_COMPONENTS, &args![this, other]).bool()
}

// Translated from 00599a50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00599a50 - the exception-unwinding frame (FS:[0]) around the by-value track is not translated.
/// A media set track's compare with a track passed by value (`RET 0x10`,
/// as `00599700`; the engine map has no name): true when a float (+8,
/// +0xC) differs from the argument's (unordered counts as different);
/// otherwise the strings through `BSStringT<char>::StrCmp(text, 0)`, true
/// when it answers 0 and false otherwise (the exe answers true for equal
/// tracks). The argument's string is destroyed (`007d9ec0`) before each
/// answer.
pub fn fn_00599a50(
    e: &mut Engine,
    this: Ptr,
    text: u32,
    lengths: u32,
    volume: f32,
    other: f32,
) -> bool {
    let base = this.addr();
    track_on_stack(e, text, lengths, volume, other, |e, track| {
        for offset in [8u32, 0xc] {
            let mine = x87_load32(e.mem.u32(base + offset));
            let theirs = x87_load32(e.mem.u32(track + offset));
            // FUCOMPP; TEST AH,0x44; JNP: on only when equal and ordered.
            if theirs != mine {
                e.call(STRING_DESTROY, &args![track]);
                return true;
            }
        }
        let name = e.call(STRING_TEXT, &args![track]).u32();
        let order = e.call(STRING_COMPARE, &args![this, name, 0u32]).u32();
        e.call(STRING_DESTROY, &args![track]);
        order == 0
    })
}

// Translated from 00599ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAmmoEffect`'s record writer (no stack words; the engine map has no
/// name): `TESForm::StartForm`, `TESFullName::Save` on +0x18, then the
/// 0xC bytes at +0x24 as `TESForm::__SaveData(+0x24, 0xC)`, each side of
/// it swapped by `005a6b10` when `00401500` answers true, then
/// `TESForm::CloseForm`.
pub fn fn_00599ef0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0048_55a0, &args![this]);
    e.call(0x0048_7010, &args![base + 0x18]);
    if e.call(0x0040_1500, &args![]).bool() {
        e.call(0x005a_6b10, &args![base + 0x24]);
    }
    e.call(0x0048_5e20, &args![this, base + 0x24, 0xcu32]);
    if e.call(0x0040_1500, &args![]).bool() {
        e.call(0x005a_6b10, &args![base + 0x24]);
    }
    e.call(0x0048_5680, &args![this]);
}

// Translated from 00599f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAmmoEffect`'s copy (`RET 4`: the source form; the engine map has no
/// name): the source through `__RTDynamicCast(source, 0, 01183028,
/// 01186474, 0)`; nothing without it, otherwise the 0xC bytes at +0x24
/// (`00401460`, cdecl: destination, source, size) and
/// `TESForm::CopyAllComponents` given the uncast source.
pub fn fn_00599f60(e: &mut Engine, this: Ptr, source: u32) {
    let base = this.addr();
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, CAST_SOURCE, AMMO_EFFECT_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return;
    }
    e.call(0x0040_1460, &args![base + 0x24, other + 0x24, 0xcu32]);
    e.call(COPY_COMPONENTS, &args![this, source]);
}

// Translated from 00599fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAmmoEffect`'s compare (`RET 4`: the other form; true when they
/// differ; the engine map has no name): true when the cast as in
/// `00599f60` fails or `_memcmp` of the 0xC bytes at +0x24 is not 0,
/// otherwise `TESForm::CompareAllComponents`.
pub fn fn_00599fc0(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let base = this.addr();
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, CAST_SOURCE, AMMO_EFFECT_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return true;
    }
    if e.call(0x00ec_4835, &args![base + 0x24, other + 0x24, 0xcu32])
        .u32()
        != 0
    {
        return true;
    }
    e.call(COMPARE_COMPONENTS, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_9550, fn_00599550(Ptr, u32)),
        entry!(0x0059_9700, fn_00599700(Ptr, u32, u32, f32, f32)),
        entry!(0x0059_9780, fn_00599780(Ptr, u32) -> Ptr),
        entry!(0x0059_97c0, fn_005997c0(Ptr, u32) -> bool),
        entry!(0x0059_9a50, fn_00599a50(Ptr, u32, u32, f32, f32) -> bool),
        entry!(0x0059_9ef0, fn_00599ef0(Ptr)),
        entry!(0x0059_9f60, fn_00599f60(Ptr, u32)),
        entry!(0x0059_9fc0, fn_00599fc0(Ptr, u32) -> bool),
        entry!(0x0059_9c40, fn_00599c40(Ptr) -> Ptr),
        entry!(0x0059_9cd0, fn_00599cd0(Ptr, u32) -> Ptr),
        entry!(0x0059_9b30, fn_00599b30(Ptr)),
        entry!(0x0059_9bd0, fn_00599bd0(Ptr)),
        entry!(0x0059_9d00, fn_00599d00(Ptr)),
        entry!(0x0059_9d70, fn_00599d70(Ptr, Ptr) -> bool),
        entry!(0x0059_a030, fn_0059a030(u32, u32, f32) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00599c40_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_9c40,
            size: 0x30,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0040_3d30, &[A::T(0x24), A::V(0x0), A::V(0xc)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x72)]),
            ],
            stores: &[(0x0, 4, 0x0103_449c), (0x18, 4, 0x0103_4484)],
        });
    }
    #[test]
    fn at_00599cd0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0059_9d00, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0059_9cd0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0059_9cd0)
                .collect();
            let mut expected = vec![(0x0059_9d00, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod media_set_and_ammo_effect_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
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

    #[test]
    fn media_set_resets() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc4);
        for offset in (0xac..0xc4).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        quiet(&mut e, &[TRACK_RESET]);
        e.call_log = Some(vec![]);
        e.call(0x0059_9bd0, &args![this]);
        let tracks: Vec<u32> = calls(&e, TRACK_RESET).iter().map(|c| c[0] - this).collect();
        assert_eq!(tracks, TRACKS.to_vec());
        assert_eq!(e.mem.u32(this + 0xac), 0xffff_ffff);
        assert_eq!((e.mem.u32(this + 0xbc), e.mem.u32(this + 0xc0)), (0, 0));

        e.call_log = Some(vec![]);
        e.call(0x0059_9b30, &args![this]);
        assert_eq!(calls(&e, TRACK_RESET).len(), 6);
        for offset in (0xac..0xc4).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }

    #[test]
    fn ammo_effect_destructor() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        quiet(&mut e, &[0x0059_1300, 0x0048_3630]);
        e.call_log = Some(vec![]);
        e.call(0x0059_9d00, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0103_449c);
        assert_eq!(e.mem.u32(this + 0x18), 0x0103_4484);
        assert_eq!(calls(&e, 0x0059_1300), vec![vec![this + 0x18]]);
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![this]]);
    }

    fn load(form_type: u32, swap: bool) -> (Engine, u32, u32, bool) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0xe0, 0x7000_00e0);
        e.mem.set_u32(vtable + 0x134, 0x7000_0134);
        e.mem.set_u32(this, vtable);
        let file = e.mem.alloc(0x10);
        let chunks = Rc::new(RefCell::new((
            vec![0x4449_4445u32, 0x444e_424f, 0x4c4c_5546, 0x4154_4144],
            0usize,
        )));
        e.register_double(0x0047_2660, move |_, _| ret(form_type));
        let c = chunks.clone();
        e.register_double(0x0047_26b0, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).copied().unwrap_or(0))
        });
        let c = chunks;
        e.register_double(0x0047_26f0, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        e.register(0x0040_1660, |_, _| ret(8));
        if swap {
            e.register(0x0040_1680, |_, _| ret(1));
        } else {
            e.register(0x0040_1680, |_, _| ret(0));
        }
        quiet(
            &mut e,
            &[
                0x7000_00e0,
                0x7000_0134,
                0x0047_2890,
                0x0048_5110,
                0x0048_4ab0,
                0x0048_7050,
                0x0048_61f0,
                0x005a_6b10,
            ],
        );
        e.call_log = Some(vec![]);
        let result = e.call(0x0059_9d70, &args![this, file]).bool();
        (e, this, file, result)
    }

    #[test]
    fn ammo_effect_load() {
        let (e, this, file, result) = load(0x72, true);
        assert!(result);
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_7050), vec![vec![this + 0x18, file]]);
        assert_eq!(
            calls(&e, 0x0048_61f0),
            vec![vec![this, file, this + 0x24, 0xc]]
        );
        assert_eq!(calls(&e, 0x005a_6b10), vec![vec![this + 0x24]]);

        let (e, _, _, result) = load(0x72, false);
        assert!(result);
        assert!(calls(&e, 0x005a_6b10).is_empty());

        let (e, _, _, result) = load(0x71, false);
        assert!(!result);
        assert!(calls(&e, 0x0048_5110).is_empty());
    }

    /// Effects: words at `effect` (type), `effect + 4` (operation),
    /// `effect + 8` (amount); the source list's nodes are the addresses of
    /// a word array ending in 0, and the copy is a separate list.
    fn effects(e: &mut Engine, list: &[(u32, u32, f32)]) -> u32 {
        let nodes = e.mem.alloc(4 * (list.len() as u32 + 1));
        for (i, (kind, operation, amount)) in list.iter().enumerate() {
            let effect = e.mem.alloc(0xc);
            e.mem.set_u32(effect, *kind);
            e.mem.set_u32(effect + 4, *operation);
            e.mem.set_f32(effect + 8, *amount);
            e.mem.set_u32(nodes + 4 * i as u32, effect);
        }
        let copy = Rc::new(RefCell::new(Vec::<u32>::new()));
        let copy_nodes = e.mem.alloc(0x40);
        e.register(0x0040_1000, |_, _| ret(0x5000));
        e.register_double(0x0096_a2d0, move |_, _| ret(copy_nodes));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |_, a| ret(a[0] + 4));
        let c = copy.clone();
        let add = move |e: &mut Engine, a: &[u32]| {
            let mut c = c.borrow_mut();
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 4 * c.len() as u32, item);
            c.push(item);
            Ret::default()
        };
        e.register_double(0x005a_e3d0, add.clone());
        e.register_double(0x0090_5820, add);
        e.register(0x0045_cd60, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0059_bb30, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0075_9450, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 8)),
            ..Ret::default()
        });
        quiet(e, &[0x0047_02f0]);
        nodes
    }

    #[test]
    fn ammo_effects_apply_in_order() {
        let mut e = Engine::new();
        let list = effects(
            &mut e,
            &[
                (1, 1, 2.0),
                (2, 0, 100.0),
                (1, 0, 3.0),
                (1, 2, 0.5),
                (1, 7, 9.0),
            ],
        );
        e.call_log = Some(vec![]);
        let result = e.call(0x0059_a030, &args![1u32, list, 10.0f32]).f32();
        // ((10 * 2) + 3) - 0.5; the other kind and operation 7 change nothing.
        assert_eq!(result, 22.5);
        assert_eq!(calls(&e, 0x005a_e3d0).len(), 1);
        assert_eq!(calls(&e, 0x0090_5820).len(), 4);
        assert_eq!(calls(&e, 0x0047_02f0).len(), 1);

        // No list: the value itself, nothing allocated.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0059_a030, &args![1u32, 0u32, 4.0f32]).f32(), 4.0);
        assert!(calls(&e, 0x0040_1000).is_empty());
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

    /// String doubles: the text is the first word; a copy copies the two
    /// words; the compare answers 0 when the texts match.
    fn string_doubles(e: &mut Engine) {
        e.register(STRING_TEXT, |e, a| ret(e.mem.u32(a[0])));
        e.register(STRING_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(STRING_COPY, |e, a| {
            let (text, lengths) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            e.mem.set_u32(a[0], text);
            e.mem.set_u32(a[0] + 4, lengths);
            Ret::default()
        });
        e.register(STRING_COMPARE, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) != a[1]))
        });
        e.register(STRING_DESTROY, |_, _| Ret::default());
    }

    #[test]
    fn fn_00599780_and_00599700_copy_a_track() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, 0x1234);
        e.mem.set_u32(source + 4, 0x0004_0004);
        e.mem.set_u32(source + 8, 0x7f80_0001);
        e.mem.set_f32(source + 0xc, 2.5);
        let copy = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0059_9780, &args![copy, source]).u32(), copy);
        assert_eq!(calls(&e, STRING_COPY), vec![vec![copy, source]]);
        assert_eq!(e.mem.u32(copy + 8), 0x7fc0_0001);
        assert_eq!(e.mem.f32(copy + 0xc), 2.5);

        let track = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(
            0x0059_9700,
            &args![track, 0x5678u32, 9u32, 0xff80_0002u32, 1.5f32],
        );
        assert_eq!(e.mem.u32(track), 0x5678);
        assert_eq!(e.mem.u32(track + 8), 0xffc0_0002);
        assert_eq!(e.mem.f32(track + 0xc), 1.5);
        assert_eq!(calls(&e, STRING_SET), vec![vec![track, 0x5678, 0]]);
        // The argument copy is the one destroyed.
        let text = calls(&e, STRING_TEXT)[0][0];
        assert_ne!(text, track);
        assert_eq!(calls(&e, STRING_DESTROY), vec![vec![text]]);
    }

    #[test]
    fn fn_00599a50_answers_true_for_equal_tracks() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        let track = e.mem.alloc(0x10);
        e.mem.set_u32(track, 0x77);
        e.mem.set_f32(track + 8, 1.0);
        e.mem.set_f32(track + 0xc, 2.0);
        let check = |e: &mut Engine, text: u32, a: f32, b: f32| {
            e.call_log = Some(vec![]);
            let r = e.call(0x0059_9a50, &args![track, text, 0u32, a, b]).bool();
            assert_eq!(calls(e, STRING_DESTROY).len(), 1);
            (r, calls(e, STRING_COMPARE).len())
        };
        assert_eq!(check(&mut e, 0x77, 1.0, 2.0), (true, 1));
        assert_eq!(check(&mut e, 0x78, 1.0, 2.0), (false, 1));
        assert_eq!(check(&mut e, 0x78, 1.5, 2.0), (true, 0));
        assert_eq!(check(&mut e, 0x78, 1.0, f32::NAN), (true, 0));
    }

    /// Two media sets alike: the tracks' texts differ, so `00599a50`
    /// answers false for each.
    fn media_sets(e: &mut Engine) -> (u32, u32) {
        string_doubles(e);
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(COMPARE_COMPONENTS, |_, _| ret(0));
        e.register(COPY_COMPONENTS, |_, _| Ret::default());
        let this = e.mem.alloc(0xc4);
        let other = e.mem.alloc(0xc4);
        for track in TRACKS {
            e.mem.set_u32(this + track, 0x100 + track);
            e.mem.set_u32(other + track, 0x200 + track);
        }
        (this, other)
    }

    #[test]
    fn fn_005997c0_compares_media_sets() {
        let mut e = Engine::new();
        let (this, other) = media_sets(&mut e);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0059_97c0, &args![this, other]).bool());
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![other, 0, CAST_SOURCE, MEDIA_SET_TYPE, 0]]
        );
        assert_eq!(calls(&e, STRING_COPY).len(), 6);
        assert_eq!(calls(&e, STRING_DESTROY).len(), 6);
        assert!(e.call(0x0059_97c0, &args![this, 0u32]).bool());
        for (offset, value) in [(0x44u32, 1u32), (0xa8, 1), (0xb0, 0x7fc0_0000), (0xc0, 3)] {
            let (this, other) = media_sets(&mut e);
            e.mem.set_u32(this + offset, value);
            assert!(
                e.call(0x0059_97c0, &args![this, other]).bool(),
                "{offset:x}"
            );
        }
        // A track whose text matches: 00599a50 answers true.
        let (this, other) = media_sets(&mut e);
        e.mem.set_u32(this + 0x88, 0x288);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0059_97c0, &args![this, other]).bool());
        assert_eq!(calls(&e, STRING_COPY).len(), 5);
        let (this, other) = media_sets(&mut e);
        e.register(COMPARE_COMPONENTS, |_, _| ret(1));
        assert!(e.call(0x0059_97c0, &args![this, other]).bool());
    }

    #[test]
    fn fn_00599550_copies_a_media_set() {
        let mut e = Engine::new();
        let (this, other) = media_sets(&mut e);
        for offset in [0x44u32, 0xac, 0xb0, 0xb4, 0xbc, 0xc0] {
            e.mem.set_u32(other + offset, offset);
        }
        e.mem.set_u32(other + 0xb8, 0x7f80_0003);
        e.mem.set_u8(other + 0xa8, 0x5a);
        e.mem.set_f32(other + 0x58 + 8, 0.25);
        e.call_log = Some(vec![]);
        e.call(0x0059_9550, &args![this, other]);
        for offset in [0x44u32, 0xac, 0xb0, 0xb4, 0xbc, 0xc0] {
            assert_eq!(e.mem.u32(this + offset), offset);
        }
        assert_eq!(e.mem.u32(this + 0xb8), 0x7fc0_0003);
        assert_eq!(e.mem.u8(this + 0xa8), 0x5a);
        for track in TRACKS {
            assert_eq!(e.mem.u32(this + track), 0x200 + track);
        }
        assert_eq!(e.mem.f32(this + 0x58 + 8), 0.25);
        assert_eq!(calls(&e, COPY_COMPONENTS), vec![vec![this, other]]);
        // No cast: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0059_9550, &args![this, 0u32]);
        assert!(calls(&e, COPY_COMPONENTS).is_empty());
    }

    #[test]
    fn ammo_effect_save_copy_and_compare() {
        let mut e = Engine::new();
        for a in [
            0x0048_55a0u32,
            0x0048_7010,
            0x005a_6b10,
            0x0048_5e20,
            0x0048_5680,
            0x0040_1460,
            COPY_COMPONENTS,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x30);
        for swap in [0u32, 1] {
            e.register_double(0x0040_1500, move |_, _| ret(swap));
            e.call_log = Some(vec![]);
            e.call(0x0059_9ef0, &args![this]);
            assert_eq!(calls(&e, 0x0048_7010), vec![vec![this + 0x18]]);
            assert_eq!(calls(&e, 0x0048_5e20), vec![vec![this, this + 0x24, 0xc]]);
            assert_eq!(calls(&e, 0x005a_6b10).len(), 2 * swap as usize);
            assert_eq!(calls(&e, 0x0048_5680), vec![vec![this]]);
        }

        e.register(DYNAMIC_CAST, |_, a| ret(a[0] & !0xf));
        let source = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x0059_9f60, &args![this, source + 1]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![source + 1, 0, CAST_SOURCE, AMMO_EFFECT_TYPE, 0]]
        );
        assert_eq!(
            calls(&e, 0x0040_1460),
            vec![vec![this + 0x24, source + 0x24, 0xc]]
        );
        assert_eq!(calls(&e, COPY_COMPONENTS), vec![vec![this, source + 1]]);
        e.call_log = Some(vec![]);
        e.call(0x0059_9f60, &args![this, 1u32]);
        assert!(calls(&e, 0x0040_1460).is_empty());

        for (order, components, differs) in [(0u32, 0u32, false), (1, 0, true), (0, 1, true)] {
            e.register_double(0x00ec_4835, move |_, _| ret(order));
            e.register_double(COMPARE_COMPONENTS, move |_, _| ret(components));
            assert_eq!(e.call(0x0059_9fc0, &args![this, source]).bool(), differs);
        }
        assert!(e.call(0x0059_9fc0, &args![this, 2u32]).bool());
    }
}
