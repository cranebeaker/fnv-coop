//! `NiAnimation/bslookiknote.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSAnimNote::ms_RTTI` (the class's `NiRTTI` record).
const BS_ANIM_NOTE_RTTI: u32 = 0x011f_3ef8;

/// `BSLookIKNote::ms_RTTI` (the class's `NiRTTI` record).
const BS_LOOK_IK_NOTE_RTTI: u32 = 0x011f_3ee8;

// Translated from 00a43f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNote::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_anim_note_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_ANIM_NOTE_RTTI)
}

// Translated from 00a43f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSLookIKNote::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_look_ik_note_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_LOOK_IK_NOTE_RTTI)
}

// Translated from 00a43f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSLookIKNote::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a44830`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x18 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bs_look_ik_note_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a4_4830;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x18u32]);
    }
    this
}

/// `BSLookIKNote` (Xbox PDB, 0x18 bytes; same offsets on PC): the
/// `BSAnimNote` part (`eType` +8, `fTime` +0xC), then `fGain` +0x10 and
/// `eState` +0x14 (1 start, 2 stop).
const NOTE_TYPE: u32 = 0x8;
const NOTE_TIME: u32 = 0xc;
const GAIN: u32 = 0x10;
const STATE: u32 = 0x14;
const OBJECT_SIZE: u32 = 0x18;
/// The class's vtable.
const VTABLE: u32 = 0x0109_7c8c;
/// The `BSAnimNote` functions this class builds on (`bsanimnote.cpp`): the
/// member copy (`00a44850`), `IsEqual` (`00a44880`), `ParseTextKey`
/// (`00a44930`), `LoadBinary` (`00a44a30`), `SaveBinary` (`00a44a90`) and
/// `GetViewerStrings` (`00a44bf0`).
const ANIM_NOTE_COPY_MEMBERS: u32 = 0x00a4_4850;
const ANIM_NOTE_IS_EQUAL: u32 = 0x00a4_4880;
const ANIM_NOTE_PARSE_TEXT_KEY: u32 = 0x00a4_4930;
const ANIM_NOTE_LOAD_BINARY: u32 = 0x00a4_4a30;
const ANIM_NOTE_SAVE_BINARY: u32 = 0x00a4_4a90;
const ANIM_NOTE_GET_VIEWER_STRINGS: u32 = 0x00a4_4bf0;
/// `NiObject`'s constructor (`00a5d3a0`) and the Ni allocator (`00aa13e0`,
/// `cdecl`).
const NI_OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The CRT's `strncmp` (`00ec8a19`) and `strncpy` (`00ecc7b0`), both `cdecl
/// (a, b, n)`; `NiString`'s constructor from a C string (`00a88b80`,
/// `thiscall`), `ToFloat` (`00a88c00`, `thiscall (string, &float)`) and
/// destructor (`00a88bc0`).
const STRNCMP: u32 = 0x00ec_8a19;
const STRNCPY: u32 = 0x00ec_c7b0;
const NI_STRING_CONSTRUCTOR: u32 = 0x00a8_8b80;
const NI_STRING_TO_FLOAT: u32 = 0x00a8_8c00;
const NI_STRING_DESTRUCTOR: u32 = 0x00a8_8bc0;
/// The strings the text key is matched with: `"LookIK"`, `"Start"`,
/// `"Stop"`.
const LOOK_IK_TEXT: u32 = 0x0109_7d1c;
const START_TEXT: u32 = 0x0104_77f4;
const STOP_TEXT: u32 = 0x0106_39ec;
/// The viewer-string makers (`cdecl`): the float form (`00a7d550`) and the
/// string form (`00a7d5e0`); the names `"Gain"` and `"State"` and the values
/// `"Start"`, `"Stop"` and `"None"`.
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
const GAIN_NAME: u32 = 0x0109_7d24;
const STATE_NAME: u32 = 0x0104_4a7c;
const NONE_TEXT: u32 = 0x0102_909c;

// Translated from 00a43f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSLookIKNote::IsEqual` (Xbox PDB): `BSAnimNote::IsEqual`, then the same
/// gain (x87 compare: a NaN is never equal) and state.
pub fn bs_look_ik_note_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(ANIM_NOTE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let (this, other) = (this.addr(), other.addr());
    x87_load32(e.mem.u32(this + GAIN)) == x87_load32(e.mem.u32(other + GAIN))
        && e.mem.u32(this + STATE) == e.mem.u32(other + STATE)
}

/// The class factory's body: 0x18 bytes from the Ni allocator, the
/// `NiObject` constructor, the time and gain 0.0 (one `FLDZ`), the type 2,
/// the vtable and the state 0. Null when the allocation fails.
fn new_note(e: &mut Engine) -> u32 {
    let object = e.call(NI_ALLOCATE, &args![OBJECT_SIZE]).u32();
    if object == 0 {
        return 0;
    }
    e.call(NI_OBJECT_CONSTRUCTOR, &args![object]);
    e.mem.set_u32(object + NOTE_TIME, 0);
    e.mem.set_u32(object + NOTE_TYPE, 2);
    e.mem.set_u32(object + GAIN, 0);
    e.mem.set_u32(object, VTABLE);
    e.mem.set_u32(object + STATE, 0);
    object
}

// Translated from 00a43fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The class factory (registered through `NiStream::RegisterLoader` at
/// `00a2d8ad`; Ghidra made no function of it): see `new_note`.
pub fn fn_00a43fb0(e: &mut Engine) -> Ptr {
    Ptr::new(new_note(e))
}

// Translated from 00a43ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSLookIKNote::CreateClone` (Xbox PDB): a new note as the factory makes
/// it (null when the allocation fails), `BSAnimNote`'s member copy into it,
/// then the gain (`FLD`/`FSTP`) and the state; the exe stores them through
/// the clone pointer even when it is null.
pub fn bs_look_ik_note_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = new_note(e);
    e.call(ANIM_NOTE_COPY_MEMBERS, &args![this, clone, cloning]);
    let gain = x87_quiet_bits(e.mem.u32(this.addr() + GAIN));
    e.mem.set_u32(clone.wrapping_add(GAIN), gain);
    let state = e.mem.u32(this.addr() + STATE);
    e.mem.set_u32(clone.wrapping_add(STATE), state);
    Ptr::new(clone)
}

// Translated from 00a44050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSLookIKNote::ParseTextKey` (Xbox PDB): `BSAnimNote::ParseTextKey`
/// gives the answer. The key's text (+4) must start with `"LookIK"` (else
/// that answer stands); then, with the answer true and a space at offset 6
/// (else false), the four characters from offset 7 (`strncpy` into the
/// exe's local) become an `NiString` (in the key argument's stack word)
/// whose `ToFloat` sets the gain (false clears the answer); with the answer
/// still true and a space at offset 11, `"Start"` at offset 12 sets the
/// state to 1 and `"Stop"` to 2. The four copied characters have no
/// terminator of their own: the `NiString` constructor reads on into the
/// next stack word (the return address, which the engine's stack does not
/// hold: it reads zeros there).
pub fn bs_look_ik_note_parse_text_key(e: &mut Engine, this: Ptr, key: Ptr) -> bool {
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
        if e.call(STRNCMP, &args![text, LOOK_IK_TEXT, 6u32]).u32() != 0 {
            return answer;
        }
        if !answer || e.mem.u8(text.wrapping_add(6)) != b' ' {
            return false;
        }
        e.call(STRNCPY, &args![local, text.wrapping_add(7), 4u32]);
        e.call(NI_STRING_CONSTRUCTOR, &args![argument, local]);
        if !e
            .call(NI_STRING_TO_FLOAT, &args![argument, object + GAIN])
            .bool()
        {
            answer = false;
        }
        e.call(NI_STRING_DESTRUCTOR, &args![argument]);
        if !answer || e.mem.u8(text.wrapping_add(0xb)) != b' ' {
            return answer;
        }
        let state = text.wrapping_add(0xc);
        if e.call(STRNCMP, &args![state, START_TEXT, 5u32]).u32() == 0 {
            e.mem.set_u32(object + STATE, 1);
        }
        if e.call(STRNCMP, &args![state, STOP_TEXT, 4u32]).u32() == 0 {
            e.mem.set_u32(object + STATE, 2);
        }
        answer
    })
}

// Translated from 00a44130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSLookIKNote::LoadBinary` (Xbox PDB): `BSAnimNote::LoadBinary`, then
/// the gain and the state (4 bytes each; the component size in the stream
/// argument's stack word).
pub fn bs_look_ik_note_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    use crate::units::nianimation::niblendinterpolator::stream_read;
    e.call(ANIM_NOTE_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, argument| {
        let argument = argument.addr();
        for field in [GAIN, STATE] {
            stream_read(e, stream, this.addr() + field, 4, argument);
        }
    });
}

// Translated from 00a44190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSLookIKNote::SaveBinary` (Xbox PDB): `BSAnimNote::SaveBinary`, then
/// the gain and the state (4 bytes each; the component size in the stream
/// argument's stack word).
pub fn bs_look_ik_note_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    use crate::units::nianimation::niblendinterpolator::stream_write;
    e.call(ANIM_NOTE_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, argument| {
        let argument = argument.addr();
        for field in [GAIN, STATE] {
            stream_write(e, stream, this.addr() + field, 4, argument);
        }
    });
}

// Translated from 00a441f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSLookIKNote::GetViewerStrings` (Xbox PDB): `BSAnimNote`'s strings,
/// `"Gain"` with the gain, then the state as the compiled `switch` falls
/// through: 1 gives `"State"` `"Start"`, `"Stop"` and `"None"`, 2 `"Stop"`
/// and `"None"`, any other only `"None"` (all through the argument's stack
/// word).
pub fn bs_look_ik_note_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    use crate::units::nianimation::niblendinterpolator::append_viewer_string;
    e.call(ANIM_NOTE_GET_VIEWER_STRINGS, &args![this, strings]);
    let object = this.addr();
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let gain = x87_quiet_bits(e.mem.u32(object + GAIN));
        let made = e.call(VIEWER_STRING_FLOAT, &args![GAIN_NAME, gain]).u32();
        append_viewer_string(e, strings, slot, made);
        let text = |e: &mut Engine, value: u32| {
            let made = e.call(VIEWER_STRING_TEXT, &args![STATE_NAME, value]).u32();
            append_viewer_string(e, strings, slot, made);
        };
        let state = e.mem.u32(object + STATE);
        if state == 1 {
            text(e, START_TEXT);
        }
        if state == 1 || state == 2 {
            text(e, STOP_TEXT);
        }
        text(e, NONE_TEXT);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a43f70, bs_anim_note_get_rtti() -> Ptr),
        entry!(0x00a43f80, bs_look_ik_note_get_rtti() -> Ptr),
        entry!(0x00a43f90, bs_look_ik_note_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a4_3f30, bs_look_ik_note_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a4_3fb0, fn_00a43fb0() -> Ptr),
        entry!(0x00a4_3ff0, bs_look_ik_note_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a4_4050,
            bs_look_ik_note_parse_text_key(Ptr, Ptr) -> bool
        ),
        entry!(0x00a4_4130, bs_look_ik_note_load_binary(Ptr, Ptr)),
        entry!(0x00a4_4190, bs_look_ik_note_save_binary(Ptr, Ptr)),
        entry!(0x00a4_41f0, bs_look_ik_note_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_anim_note_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a43f70, &args![]).u32(), BS_ANIM_NOTE_RTTI);
    }

    #[test]
    fn bs_look_ik_note_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a43f80, &args![]).u32(), BS_LOOK_IK_NOTE_RTTI);
    }

    #[test]
    fn bs_look_ik_note_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a4_4830, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a43f90, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a43f90, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x18]]);
        assert_eq!(calls_to(&e, 0x00a4_4830), vec![vec![this]; 2]);
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
    fn is_equal_compares_gain_and_state() {
        let mut e = engine();
        let (a, b) = (e.mem.alloc(OBJECT_SIZE), e.mem.alloc(OBJECT_SIZE));
        for p in [a, b] {
            e.mem.set_f32(p + GAIN, 0.5);
            e.mem.set_u32(p + STATE, 1);
        }
        assert!(e.call(0x00a4_3f30, &args![a, b]).bool());
        e.mem.set_f32(b + GAIN, 0.25);
        assert!(!e.call(0x00a4_3f30, &args![a, b]).bool());
        e.mem.set_f32(b + GAIN, 0.5);
        e.mem.set_u32(b + STATE, 2);
        assert!(!e.call(0x00a4_3f30, &args![a, b]).bool());
    }

    #[test]
    fn factory_and_clone_build_look_notes() {
        let mut e = engine();
        let object = e.call(0x00a4_3fb0, &args![]).u32();
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + NOTE_TYPE), 2);
        let this = e.mem.alloc(OBJECT_SIZE);
        e.mem.set_f32(this + GAIN, 0.25);
        e.mem.set_u32(this + STATE, 2);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a4_3ff0, &args![this, 0x60u32]).u32();
        assert_eq!(
            calls(&e, ANIM_NOTE_COPY_MEMBERS),
            vec![vec![this, clone, 0x60]]
        );
        assert_eq!(e.mem.f32(clone + GAIN), 0.25);
        assert_eq!(e.mem.u32(clone + STATE), 2);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_3fb0, &args![]).u32(), 0);
    }

    #[test]
    fn parse_text_key_reads_gain_and_state() {
        let mut e = engine();
        strings(
            &mut e,
            &[
                (LOOK_IK_TEXT, "LookIK"),
                (START_TEXT, "Start"),
                (STOP_TEXT, "Stop"),
            ],
        );
        let this = e.mem.alloc(OBJECT_SIZE);
        assert!(parse(&mut e, 0x00a4_4050, this, "LookIK 0.50 Stop"));
        assert_eq!(e.mem.f32(this + GAIN), 0.5);
        assert_eq!(e.mem.u32(this + STATE), 2);
        assert!(parse(&mut e, 0x00a4_4050, this, "LookIK 1.00 Start"));
        assert_eq!(e.mem.u32(this + STATE), 1);
        assert!(
            parse(&mut e, 0x00a4_4050, this, "GrabIK"),
            "not a look note"
        );
        assert!(!parse(&mut e, 0x00a4_4050, this, "LookIK_0.5"), "no space");
        assert!(
            !parse(&mut e, 0x00a4_4050, this, "LookIK abcd Stop"),
            "no gain"
        );
    }

    #[test]
    fn load_and_save_binary_move_gain_and_state() {
        let mut e = engine();
        let mut bytes = vec![];
        bytes.extend(0.75f32.to_le_bytes());
        bytes.extend(1u32.to_le_bytes());
        let (s, written) = stream(&mut e, 0, &bytes);
        let this = e.mem.alloc(OBJECT_SIZE);
        e.call(0x00a4_4130, &args![this, s]);
        assert_eq!(e.mem.f32(this + GAIN), 0.75);
        assert_eq!(e.mem.u32(this + STATE), 1);
        e.call(0x00a4_4190, &args![this, s]);
        assert_eq!(*written.borrow(), bytes);
    }

    #[test]
    fn get_viewer_strings_falls_through_the_state_switch() {
        let mut e = engine();
        let (strings, stored) = viewer_strings(&mut e);
        let this = e.mem.alloc(OBJECT_SIZE);
        e.mem.set_u32(this + STATE, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a4_41f0, &args![this, strings]);
        let values: Vec<u32> = calls(&e, VIEWER_STRING_TEXT).iter().map(|c| c[1]).collect();
        assert_eq!(values, [START_TEXT, STOP_TEXT, NONE_TEXT]);
        assert_eq!(stored.borrow()[0], GAIN_NAME);
        e.mem.set_u32(this + STATE, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_41f0, &args![this, strings]);
        let values: Vec<u32> = calls(&e, VIEWER_STRING_TEXT).iter().map(|c| c[1]).collect();
        assert_eq!(values, [NONE_TEXT]);
    }
}
