//! `NiAnimation/bsgrabiknote.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSGrabIKNote::ms_RTTI` (the class's `NiRTTI` record).
const BS_GRAB_IK_NOTE_RTTI: u32 = 0x011f_3ef0;

// Translated from 00a44350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGrabIKNote::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_grab_ik_note_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_GRAB_IK_NOTE_RTTI)
}

/// `00a724d0`: the jump target of `00a95ca0`.
const JUMP_TARGET_00A95CA0: u32 = 0x00a7_24d0;

// Translated from 00a95ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGrabIKNote::LinkObject` (Xbox PDB): a 5-byte jump to `00a724d0` (engine map: `NiPSysEmitterCtlrData::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@BSGrabIKNote@@UAAXAAVNiStream@@@Z`.
pub fn bs_grab_ik_note_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A95CA0, &args![this, stream]);
}

/// `BSGrabIKNote` (Xbox PDB, 0x1C bytes; same offsets on PC): the
/// `BSAnimNote` part (`eType` +8, `fTime` +0xC), then `eArm` +0x10 (1 for
/// the right arm), `fGain` +0x14 and `eState` +0x18 (1 start, 2 stop).
const NOTE_TYPE: u32 = 0x8;
const NOTE_TIME: u32 = 0xc;
const ARM: u32 = 0x10;
const GAIN: u32 = 0x14;
const STATE: u32 = 0x18;
const OBJECT_SIZE: u32 = 0x1c;
/// The class's vtable.
const VTABLE: u32 = 0x0109_7d34;
/// The `BSAnimNote` functions this class builds on (`bsanimnote.cpp`): the
/// destructor body (`00a44830`), the member copy (`00a44850`), `IsEqual`
/// (`00a44880`), `ParseTextKey` (`00a44930`), `LoadBinary` (`00a44a30`),
/// `SaveBinary` (`00a44a90`), `GetViewerStrings` (`00a44bf0`) and
/// `RegisterStreamables` (`00a84c70`).
const ANIM_NOTE_DESTRUCTOR: u32 = 0x00a4_4830;
const ANIM_NOTE_COPY_MEMBERS: u32 = 0x00a4_4850;
const ANIM_NOTE_IS_EQUAL: u32 = 0x00a4_4880;
const ANIM_NOTE_PARSE_TEXT_KEY: u32 = 0x00a4_4930;
const ANIM_NOTE_LOAD_BINARY: u32 = 0x00a4_4a30;
const ANIM_NOTE_SAVE_BINARY: u32 = 0x00a4_4a90;
const ANIM_NOTE_GET_VIEWER_STRINGS: u32 = 0x00a4_4bf0;
const ANIM_NOTE_REGISTER_STREAMABLES: u32 = 0x00a8_4c70;
/// `NiObject`'s constructor (`00a5d3a0`), the Ni allocator (`00aa13e0`,
/// `cdecl`) and the sized delete (`00aa1460`, `cdecl (block, size)`).
const NI_OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// The CRT's `strncmp` (`00ec8a19`) and `strncpy` (`00ecc7b0`), both `cdecl
/// (a, b, n)`; `NiString`'s constructor from a C string (`00a88b80`,
/// `thiscall`), `ToFloat` (`00a88c00`, `thiscall (string, &float)`) and
/// destructor (`00a88bc0`).
const STRNCMP: u32 = 0x00ec_8a19;
const STRNCPY: u32 = 0x00ec_c7b0;
const NI_STRING_CONSTRUCTOR: u32 = 0x00a8_8b80;
const NI_STRING_TO_FLOAT: u32 = 0x00a8_8c00;
const NI_STRING_DESTRUCTOR: u32 = 0x00a8_8bc0;
/// The strings the text key is matched with: `"GrabIK"`, `"Start"`,
/// `"Stop"`.
const GRAB_IK_TEXT: u32 = 0x0109_7dc4;
const START_TEXT: u32 = 0x0104_77f4;
const STOP_TEXT: u32 = 0x0106_39ec;
/// The viewer-string makers (`cdecl`): the float form (`00a7d550`) and the
/// string form (`00a7d5e0`); the names `"Arm"`, `"Gain"` and `"State"`, the
/// values `"Left"`, `"Right"`, the empty string, `"Start"`, `"Stop"` and
/// `"None"`.
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
const ARM_NAME: u32 = 0x0109_7dcc;
const GAIN_NAME: u32 = 0x0109_7d24;
const STATE_NAME: u32 = 0x0104_4a7c;
const LEFT_TEXT: u32 = 0x0104_8444;
const RIGHT_TEXT: u32 = 0x0104_843c;
const EMPTY_TEXT: u32 = 0x0101_1584;
const NONE_TEXT: u32 = 0x0102_909c;

// Translated from 00a43f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGrabIKNote::RegisterStreamables` (Xbox PDB): `BSAnimNote`'s, as a
/// bool (`SETNZ`).
pub fn bs_grab_ik_note_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(ANIM_NOTE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
}

// Translated from 00a44310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGrabIKNote::IsEqual` (Xbox PDB): `BSAnimNote::IsEqual`, then the same
/// arm, gain (x87 compare: a NaN is never equal) and state.
pub fn bs_grab_ik_note_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(ANIM_NOTE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let (this, other) = (this.addr(), other.addr());
    e.mem.u32(this + ARM) == e.mem.u32(other + ARM)
        && x87_load32(e.mem.u32(this + GAIN)) == x87_load32(e.mem.u32(other + GAIN))
        && e.mem.u32(this + STATE) == e.mem.u32(other + STATE)
}

// Translated from 00a44360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGrabIKNote::_scalar_deleting_destructor_` (Xbox PDB): the
/// `BSAnimNote` destructor body (`00a44830`), then the sized delete of 0x1C
/// bytes when bit 0 of `flags` is set. Answers `this`.
pub fn bs_grab_ik_note_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(ANIM_NOTE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, OBJECT_SIZE]);
    }
    this
}

/// The class factory's body: 0x1C bytes from the Ni allocator, the
/// `NiObject` constructor, the time and gain 0.0 (one `FLDZ`), the type 1,
/// the vtable and the state 0 (the arm is left as allocated). Null when the
/// allocation fails.
fn new_note(e: &mut Engine) -> u32 {
    let object = e.call(NI_ALLOCATE, &args![OBJECT_SIZE]).u32();
    if object == 0 {
        return 0;
    }
    e.call(NI_OBJECT_CONSTRUCTOR, &args![object]);
    e.mem.set_u32(object + NOTE_TIME, 0);
    e.mem.set_u32(object + NOTE_TYPE, 1);
    e.mem.set_u32(object + GAIN, 0);
    e.mem.set_u32(object, VTABLE);
    e.mem.set_u32(object + STATE, 0);
    object
}

// Translated from 00a44380 (decompiled, FalloutNV.exe 1.4.0.525)
/// The class factory (registered through `NiStream::RegisterLoader` at
/// `00a2d8ad`; Ghidra made no function of it): see `new_note`.
pub fn fn_00a44380(e: &mut Engine) -> Ptr {
    Ptr::new(new_note(e))
}

// Translated from 00a443c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGrabIKNote::CreateClone` (Xbox PDB): a new note as the factory makes
/// it (null when the allocation fails), `BSAnimNote`'s member copy into it,
/// then the arm, the gain (`FLD`/`FSTP`) and the state; the exe stores them
/// through the clone pointer even when it is null.
pub fn bs_grab_ik_note_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = new_note(e);
    e.call(ANIM_NOTE_COPY_MEMBERS, &args![this, clone, cloning]);
    let arm = e.mem.u32(this.addr() + ARM);
    e.mem.set_u32(clone.wrapping_add(ARM), arm);
    let gain = x87_quiet_bits(e.mem.u32(this.addr() + GAIN));
    e.mem.set_u32(clone.wrapping_add(GAIN), gain);
    let state = e.mem.u32(this.addr() + STATE);
    e.mem.set_u32(clone.wrapping_add(STATE), state);
    Ptr::new(clone)
}

// Translated from 00a44430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGrabIKNote::ParseTextKey` (Xbox PDB): `BSAnimNote::ParseTextKey`
/// gives the first answer. The key's text (+4) must start with `"GrabIK"`
/// (else that answer stands); `" R"` or `" L"` next sets the arm (1 for R)
/// and makes the answer true. Then, with the answer true and a space at
/// offset 8, the four characters from offset 9 (`strncpy` into the exe's
/// local) become an `NiString` (in the key argument's stack word) whose
/// `ToFloat` sets the gain (false clears the answer); with the answer still
/// true and a space at offset 13, `"Start"` at offset 14 sets the state to 1
/// and `"Stop"` to 2. False when the arm part or the gain's space is
/// missing. The four copied characters have no terminator of their own: the
/// `NiString` constructor reads on into the next stack word (the return
/// address, which the engine's stack does not hold: it reads zeros there).
pub fn bs_grab_ik_note_parse_text_key(e: &mut Engine, this: Ptr, key: Ptr) -> bool {
    let object = this.addr();
    e.with_stack(0xc, |e, frame| {
        // The exe's local (which held `this`), the return address word, the
        // key argument's word.
        let local = frame.addr();
        let argument = local + 8;
        e.mem.set_u32(local, object);
        e.mem.set_u32(argument, key.addr());
        let mut answer = e.call(ANIM_NOTE_PARSE_TEXT_KEY, &args![this, key]).bool();
        let text = e.mem.u32(key.addr() + 4);
        if e.call(STRNCMP, &args![text, GRAB_IK_TEXT, 6u32]).u32() != 0 {
            return answer;
        }
        if e.mem.u8(text.wrapping_add(6)) == b' ' {
            let side = e.mem.u8(text.wrapping_add(7));
            if side == b'R' || side == b'L' {
                answer = true;
                e.mem.set_u32(object + ARM, u32::from(side == b'R'));
            }
        }
        if !answer || e.mem.u8(text.wrapping_add(8)) != b' ' {
            return false;
        }
        e.call(STRNCPY, &args![local, text.wrapping_add(9), 4u32]);
        e.call(NI_STRING_CONSTRUCTOR, &args![argument, local]);
        if !e
            .call(NI_STRING_TO_FLOAT, &args![argument, object + GAIN])
            .bool()
        {
            answer = false;
        }
        e.call(NI_STRING_DESTRUCTOR, &args![argument]);
        if !answer || e.mem.u8(text.wrapping_add(0xd)) != b' ' {
            return answer;
        }
        let state = text.wrapping_add(0xe);
        if e.call(STRNCMP, &args![state, START_TEXT, 5u32]).u32() == 0 {
            e.mem.set_u32(object + STATE, 1);
        }
        if e.call(STRNCMP, &args![state, STOP_TEXT, 4u32]).u32() == 0 {
            e.mem.set_u32(object + STATE, 2);
        }
        answer
    })
}

// Translated from 00a44530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGrabIKNote::LoadBinary` (Xbox PDB): `BSAnimNote::LoadBinary`, then
/// the arm, gain and state (4 bytes each; the component size in the stream
/// argument's stack word).
pub fn bs_grab_ik_note_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    use crate::units::nianimation::niblendinterpolator::stream_read;
    e.call(ANIM_NOTE_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, argument| {
        let argument = argument.addr();
        for field in [ARM, GAIN, STATE] {
            stream_read(e, stream, this.addr() + field, 4, argument);
        }
    });
}

// Translated from 00a445c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGrabIKNote::SaveBinary` (Xbox PDB): `BSAnimNote::SaveBinary`, then
/// the arm, gain and state (4 bytes each; the component size in the stream
/// argument's stack word).
pub fn bs_grab_ik_note_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    use crate::units::nianimation::niblendinterpolator::stream_write;
    e.call(ANIM_NOTE_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, argument| {
        let argument = argument.addr();
        for field in [ARM, GAIN, STATE] {
            stream_write(e, stream, this.addr() + field, 4, argument);
        }
    });
}

// Translated from 00a44650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGrabIKNote::GetViewerStrings` (Xbox PDB): `BSAnimNote`'s strings,
/// then the arm as the compiled `switch` falls through: arm 0 gives `"Arm"`
/// `"Left"`, `"Right"` and `""`, arm 1 `"Right"` and `""`, any other only
/// `""`; then `"Gain"` with the gain; then the state the same way: 1 gives
/// `"State"` `"Start"`, `"Stop"` and `"None"`, 2 `"Stop"` and `"None"`, any
/// other only `"None"` (all through the argument's stack word).
pub fn bs_grab_ik_note_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    use crate::units::nianimation::niblendinterpolator::append_viewer_string;
    e.call(ANIM_NOTE_GET_VIEWER_STRINGS, &args![this, strings]);
    let object = this.addr();
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let text = |e: &mut Engine, name: u32, value: u32| {
            let made = e.call(VIEWER_STRING_TEXT, &args![name, value]).u32();
            append_viewer_string(e, strings, slot, made);
        };
        let arm = e.mem.u32(object + ARM);
        if arm == 0 {
            text(e, ARM_NAME, LEFT_TEXT);
        }
        if arm == 0 || arm == 1 {
            text(e, ARM_NAME, RIGHT_TEXT);
        }
        text(e, ARM_NAME, EMPTY_TEXT);
        let gain = x87_quiet_bits(e.mem.u32(object + GAIN));
        let made = e.call(VIEWER_STRING_FLOAT, &args![GAIN_NAME, gain]).u32();
        append_viewer_string(e, strings, slot, made);
        let state = e.mem.u32(object + STATE);
        if state == 1 {
            text(e, STATE_NAME, START_TEXT);
        }
        if state == 1 || state == 2 {
            text(e, STATE_NAME, STOP_TEXT);
        }
        text(e, STATE_NAME, NONE_TEXT);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a44350, bs_grab_ik_note_get_rtti() -> Ptr),
        entry!(0x00a9_5ca0, bs_grab_ik_note_link_object(Ptr, Ptr)),
        entry!(
            0x00a4_3f10,
            bs_grab_ik_note_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00a4_4310, bs_grab_ik_note_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a4_4360,
            bs_grab_ik_note_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a4_4380, fn_00a44380() -> Ptr),
        entry!(0x00a4_43c0, bs_grab_ik_note_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a4_4430,
            bs_grab_ik_note_parse_text_key(Ptr, Ptr) -> bool
        ),
        entry!(0x00a4_4530, bs_grab_ik_note_load_binary(Ptr, Ptr)),
        entry!(0x00a4_45c0, bs_grab_ik_note_save_binary(Ptr, Ptr)),
        entry!(0x00a4_4650, bs_grab_ik_note_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_grab_ik_note_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a44350, &args![]).u32(), BS_GRAB_IK_NOTE_RTTI);
    }

    #[test]
    fn thunk_00a95ca0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_5ca0;
        const TARGET: u32 = 0x00a7_24d0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }
}

#[cfg(test)]
mod a07_tests {
    use super::*;
    use crate::units::nianimation::niblendinterpolator::a07_test_support::{
        calls, stream, viewer_strings,
    };

    /// Doubles for the CRT string functions (on the engine's memory), an
    /// `NiString` that keeps the C string's address and parses it with
    /// `str::parse` (false on failure), and the base note functions.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(STRNCMP, |e, a| {
            let (x, y) = (c_string(e, a[0], a[2]), c_string(e, a[1], a[2]));
            u32::from(x != y).into_ret()
        });
        e.register(STRNCPY, |e, a| {
            for i in 0..a[2] {
                let b = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, b);
            }
            a[0].into_ret()
        });
        e.register(NI_STRING_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_STRING_TO_FLOAT, |e, a| {
            let chars = c_string(e, e.mem.u32(a[0]), 64);
            match String::from_utf8(chars)
                .ok()
                .and_then(|s| s.parse::<f32>().ok())
            {
                Some(v) => {
                    e.mem.set_f32(a[1], v);
                    1u32.into_ret()
                }
                None => 0u32.into_ret(),
            }
        });
        e.register(NI_STRING_DESTRUCTOR, |_, _| Ret::default());
        e.register(ANIM_NOTE_PARSE_TEXT_KEY, |_, _| 1u32.into_ret());
        e.register(ANIM_NOTE_IS_EQUAL, |_, _| 1u32.into_ret());
        for f in [
            ANIM_NOTE_COPY_MEMBERS,
            ANIM_NOTE_LOAD_BINARY,
            ANIM_NOTE_SAVE_BINARY,
            ANIM_NOTE_GET_VIEWER_STRINGS,
            NI_OBJECT_CONSTRUCTOR,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e
    }

    /// A text key whose text is `text` (a C string), and its parsing.
    fn parse(e: &mut Engine, address: u32, this: u32, text: &str) -> bool {
        let chars = e.mem.alloc(text.len() as u32 + 1);
        for (i, b) in text.bytes().enumerate() {
            e.mem.set_u8(chars + i as u32, b);
        }
        let key = e.mem.alloc(8);
        e.mem.set_u32(key + 4, chars);
        e.call(address, &args![this, key]).bool()
    }

    /// Writes the C strings the code compares with.
    fn strings(e: &mut Engine, at: &[(u32, &str)]) {
        for (address, text) in at {
            e.map(*address & !0xfff, 0x1000);
            for (i, b) in text.bytes().enumerate() {
                e.mem.set_u8(address + i as u32, b);
            }
            e.mem.set_u8(address + text.len() as u32, 0);
        }
    }

    /// Up to `n` bytes of the C string at `at`.
    fn c_string(e: &Engine, mut at: u32, n: u32) -> Vec<u8> {
        let mut out = vec![];
        while out.len() < n as usize {
            let b = e.mem.u8(at);
            if b == 0 {
                break;
            }
            out.push(b);
            at += 1;
        }
        out
    }

    #[test]
    fn register_streamables_is_the_note_base_as_a_bool() {
        let mut e = engine();
        e.register(ANIM_NOTE_REGISTER_STREAMABLES, |_, _| 0x200u32.into_ret());
        assert!(!e.call(0x00a4_3f10, &args![1u32, 2u32]).bool());
        e.register(ANIM_NOTE_REGISTER_STREAMABLES, |_, _| 4u32.into_ret());
        assert_eq!(e.call(0x00a4_3f10, &args![1u32, 2u32]).u32(), 1);
    }

    #[test]
    fn is_equal_compares_arm_gain_and_state() {
        let mut e = engine();
        let (a, b) = (e.mem.alloc(OBJECT_SIZE), e.mem.alloc(OBJECT_SIZE));
        for p in [a, b] {
            e.mem.set_u32(p + ARM, 1);
            e.mem.set_f32(p + GAIN, 0.5);
            e.mem.set_u32(p + STATE, 2);
        }
        assert!(e.call(0x00a4_4310, &args![a, b]).bool());
        e.mem.set_u32(b + STATE, 1);
        assert!(!e.call(0x00a4_4310, &args![a, b]).bool());
        e.mem.set_u32(b + STATE, 2);
        e.mem.set_u32(b + ARM, 0);
        assert!(!e.call(0x00a4_4310, &args![a, b]).bool());
        e.register(ANIM_NOTE_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a4_4310, &args![a, a]).bool());
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = engine();
        e.register(ANIM_NOTE_DESTRUCTOR, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_4360, &args![0x500u32, 0u32]).u32(), 0x500);
        assert!(calls(&e, SIZED_DELETE).is_empty());
        e.call(0x00a4_4360, &args![0x500u32, 1u32]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![0x500, OBJECT_SIZE]]);
        assert_eq!(calls(&e, ANIM_NOTE_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn factory_and_clone_build_grab_notes() {
        let mut e = engine();
        let object = e.call(0x00a4_4380, &args![]).u32();
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + NOTE_TYPE), 1);
        let this = e.mem.alloc(OBJECT_SIZE);
        e.mem.set_u32(this + ARM, 1);
        e.mem.set_f32(this + GAIN, 0.25);
        e.mem.set_u32(this + STATE, 2);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a4_43c0, &args![this, 0x60u32]).u32();
        assert_eq!(
            calls(&e, ANIM_NOTE_COPY_MEMBERS),
            vec![vec![this, clone, 0x60]]
        );
        assert_eq!(e.mem.u32(clone + ARM), 1);
        assert_eq!(e.mem.f32(clone + GAIN), 0.25);
        assert_eq!(e.mem.u32(clone + STATE), 2);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_4380, &args![]).u32(), 0);
    }

    #[test]
    fn parse_text_key_reads_arm_gain_and_state() {
        let mut e = engine();
        strings(
            &mut e,
            &[
                (GRAB_IK_TEXT, "GrabIK"),
                (START_TEXT, "Start"),
                (STOP_TEXT, "Stop"),
            ],
        );
        let this = e.mem.alloc(OBJECT_SIZE);
        assert!(parse(&mut e, 0x00a4_4430, this, "GrabIK R 0.50 Start"));
        assert_eq!(e.mem.u32(this + ARM), 1);
        assert_eq!(e.mem.f32(this + GAIN), 0.5);
        assert_eq!(e.mem.u32(this + STATE), 1);
        assert!(parse(&mut e, 0x00a4_4430, this, "GrabIK L 1.25 Stop"));
        assert_eq!(e.mem.u32(this + ARM), 0);
        assert_eq!(e.mem.f32(this + GAIN), 1.25);
        assert_eq!(e.mem.u32(this + STATE), 2);
        assert!(
            !parse(&mut e, 0x00a4_4430, this, "GrabIK R x.yz Stop"),
            "no gain"
        );
        // Without an arm the base's answer (true here) carries on.
        assert!(parse(&mut e, 0x00a4_4430, this, "GrabIK X 0.50"));
        assert_eq!(e.mem.u32(this + ARM), 1, "left as the last parse set it");
        e.register(ANIM_NOTE_PARSE_TEXT_KEY, |_, _| 0u32.into_ret());
        assert!(!parse(&mut e, 0x00a4_4430, this, "GrabIK X 0.50"), "no arm");
        assert!(parse(&mut e, 0x00a4_4430, this, "GrabIK R 0.50"));
        assert!(!parse(&mut e, 0x00a4_4430, this, "LookIK 0.50"));
    }

    #[test]
    fn load_and_save_binary_move_the_three_fields() {
        let mut e = engine();
        let mut bytes = vec![];
        bytes.extend(1u32.to_le_bytes());
        bytes.extend(0.75f32.to_le_bytes());
        bytes.extend(2u32.to_le_bytes());
        let (s, written) = stream(&mut e, 0, &bytes);
        let this = e.mem.alloc(OBJECT_SIZE);
        e.call(0x00a4_4530, &args![this, s]);
        assert_eq!(e.mem.u32(this + ARM), 1);
        assert_eq!(e.mem.f32(this + GAIN), 0.75);
        assert_eq!(e.mem.u32(this + STATE), 2);
        e.call(0x00a4_45c0, &args![this, s]);
        assert_eq!(*written.borrow(), bytes);
    }

    #[test]
    fn get_viewer_strings_falls_through_the_switches() {
        let mut e = engine();
        let (strings, stored) = viewer_strings(&mut e);
        let this = e.mem.alloc(OBJECT_SIZE);
        e.call_log = Some(vec![]);
        e.call(0x00a4_4650, &args![this, strings]);
        let values: Vec<u32> = calls(&e, VIEWER_STRING_TEXT).iter().map(|c| c[1]).collect();
        assert_eq!(values, [LEFT_TEXT, RIGHT_TEXT, EMPTY_TEXT, NONE_TEXT]);
        assert_eq!(stored.borrow().len(), 5);
        e.mem.set_u32(this + ARM, 1);
        e.mem.set_u32(this + STATE, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a4_4650, &args![this, strings]);
        let values: Vec<u32> = calls(&e, VIEWER_STRING_TEXT).iter().map(|c| c[1]).collect();
        assert_eq!(
            values,
            [RIGHT_TEXT, EMPTY_TEXT, START_TEXT, STOP_TEXT, NONE_TEXT]
        );
        e.mem.set_u32(this + ARM, 5);
        e.mem.set_u32(this + STATE, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a4_4650, &args![this, strings]);
        let values: Vec<u32> = calls(&e, VIEWER_STRING_TEXT).iter().map(|c| c[1]).collect();
        assert_eq!(values, [EMPTY_TEXT, STOP_TEXT, NONE_TEXT]);
    }
}
