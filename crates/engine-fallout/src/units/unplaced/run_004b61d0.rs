//! `(unplaced)/run_004b61d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006240d0` (`thiscall`, no stack word): constructs the 0x10-byte local of `004b61d0`.
const LOCAL_CONSTRUCT: u32 = 0x0062_40d0;
/// `0098ddd0` (`thiscall`, `RET 4`; unit `fallout/ai/combat/combatgroup.cpp`): given the
/// address of the three words passed by value.
const LOCAL_SET_POINT: u32 = 0x0098_ddd0;
/// `0063f790` (`thiscall`, `RET 4`: a float).
const LOCAL_SET_FLOAT: u32 = 0x0063_f790;
/// `004b5ff0` (`cdecl`, two words; unit `fallout shared/globalfunc.cpp`).
const TEST_WITH_LOCAL: u32 = 0x004b_5ff0;

// Translated from 004b61d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, five words: three words passed by value, a word and a
/// float): returns false when `object` is null. Otherwise builds a 0x10-byte local
/// (`006240d0`), gives it the address of the three by-value words (`0098ddd0`) and `value`
/// (`0063f790`, the float copied through the x87 stack), and returns what `004b5ff0` answers
/// for the local and `object` (low byte of `EAX`). The local is not destroyed.
pub fn fn_004b61d0(
    e: &mut Engine,
    point_x: u32,
    point_y: u32,
    point_z: u32,
    object: u32,
    value: f32,
) -> bool {
    if object == 0 {
        return false;
    }
    e.with_stack(0x1c, |e, frame| {
        let local = frame.addr();
        let point = local + 0x10;
        e.mem.set_u32(point, point_x);
        e.mem.set_u32(point + 4, point_y);
        e.mem.set_u32(point + 8, point_z);
        e.call(LOCAL_CONSTRUCT, &args![local]);
        e.call(LOCAL_SET_POINT, &args![local, point]);
        e.call(
            LOCAL_SET_FLOAT,
            &args![local, x87_quiet_bits(value.to_bits())],
        );
        e.call(TEST_WITH_LOCAL, &args![local, object]).bool()
    })
}

/// `0043b230` (`thiscall`, no stack word; unit `fallout shared/modelloader.cpp`).
const FORM_LOOKUP_KEY: u32 = 0x0043_b230;
/// `00653270` (`cdecl`, two words; unit `fallout shared/facegen/bsfacegenmanager.cpp`):
/// looks the key up in the table at `011f36ac`.
const TABLE_LOOKUP: u32 = 0x0065_3270;
/// The table `004b6220` looks up (`011f36ac`).
const LOOKUP_TABLE: u32 = 0x011f_36ac;
/// `00438170` (`thiscall`, `RET 4`) and `004381b0` (`thiscall`, no stack word): construct and
/// destroy the 4-byte local of `004b6220` (unit `fallout shared/extradataobjects.cpp`).
const NAME_LOCAL_CONSTRUCT: u32 = 0x0043_8170;
const NAME_LOCAL_DESTROY: u32 = 0x0043_81b0;
/// `0047a520` (`thiscall`, `RET 4`; unit `fallout shared/bgsopencloseform.cpp`).
const FIND_BY_NAME: u32 = 0x0047_a520;
/// `00700300` (`thiscall`, no stack word).
const GET_HOLDER: u32 = 0x0070_0300;
/// `00495f30` (`thiscall`, `RET 4`; unit `fallout shared/animation.cpp`): returns the array
/// of 8-byte entries and stores their count at the address given.
const GET_ENTRIES: u32 = 0x0049_5f30;
/// `0044a670` (`cdecl`, one word; unit `fallout shared/tesobject.cpp`): the length of the
/// text.
const TEXT_LENGTH: u32 = 0x0044_a670;
/// `00717e50` (`thiscall`, no stack word; engine map
/// `BaseProcess::GetActorPackageThatIsRunning`, a folded body).
const ENTRY_OBJECT: u32 = 0x0071_7e50;
/// `0043b1b0` (`thiscall`, no stack word; unit `fallout shared/modelloader.cpp`): the text of
/// the object.
const OBJECT_TEXT: u32 = 0x0043_b1b0;
/// `_strnicmp` (`00ec7ec0`, `cdecl`: two strings and a length).
const STRNICMP: u32 = 0x00ec_7ec0;
/// `006a7f50` (`thiscall`, no stack word; returns a float in ST0).
const ENTRY_VALUE: u32 = 0x006a_7f50;

// Translated from 004b6220 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004b6220 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef0ab8, state variable) is not translated
/// Unnamed in the engine map (`cdecl`, three words; returns a float in ST0): 0.0 for a null
/// `form`. Otherwise looks the form's key (`0043b230`) up in the table at `011f36ac`
/// (`00653270`); with a result, builds a 4-byte local from `name` (`00438170`), finds the
/// object for it (`0047a520` on the result), destroys the local (`004381b0`); with an object,
/// gets its holder (`00700300`) and the holder's array of 8-byte entries and their count
/// (`00495f30`). When `text` is not empty (`0044a670`), returns the value (`006a7f50`) of the
/// first entry whose object's text (`00717e50`, then `0043b1b0`) is set and matches `text`
/// in its first `length` characters, ignoring case (`_strnicmp`). Every other path returns
/// 0.0.
pub fn fn_004b6220(e: &mut Engine, form: u32, name: u32, text: u32) -> f32 {
    if form == 0 {
        return 0.0;
    }
    let key = e.call(FORM_LOOKUP_KEY, &args![form]).u32();
    let found = e.call(TABLE_LOOKUP, &args![LOOKUP_TABLE, key]).u32();
    if found == 0 {
        return 0.0;
    }
    let object = e.with_stack(4, |e, local| {
        let made = e.call(NAME_LOCAL_CONSTRUCT, &args![local, name]).u32();
        let object = e.call(FIND_BY_NAME, &args![found, made]).u32();
        e.call(NAME_LOCAL_DESTROY, &args![local]);
        object
    });
    if object == 0 {
        return 0.0;
    }
    let holder = e.call(GET_HOLDER, &args![object]).u32();
    let (entries, count) = e.with_stack(4, |e, count| {
        let entries = e.call(GET_ENTRIES, &args![holder, count]).u32();
        (entries, e.mem.u32(count.addr()))
    });
    let length = e.call(TEXT_LENGTH, &args![text]).u32();
    if length == 0 {
        return 0.0;
    }
    let mut index = 0u32;
    while index < count {
        let entry = entries.wrapping_add(index.wrapping_mul(8));
        let entry_object = e.call(ENTRY_OBJECT, &args![entry]).u32();
        let entry_text = e.call(OBJECT_TEXT, &args![entry_object]).u32();
        if entry_text != 0 && e.call(STRNICMP, &args![entry_text, text, length]).u32() == 0 {
            return e.call(ENTRY_VALUE, &args![entry]).f32();
        }
        index = index.wrapping_add(1);
    }
    0.0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004b_61d0, fn_004b61d0(u32, u32, u32, u32, f32) -> bool),
        entry!(0x004b_6220, fn_004b6220(u32, u32, u32) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_004b61d0_builds_the_local_then_answers_the_test() {
        let mut e = Engine::new();
        e.register(LOCAL_CONSTRUCT, |_, _| Ret::default());
        e.register(LOCAL_SET_POINT, |e, a| {
            assert_eq!(e.mem.bytes(a[1], 12), [1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]);
            Ret::default()
        });
        e.register(LOCAL_SET_FLOAT, |_, _| Ret::default());
        e.register(TEST_WITH_LOCAL, |_, _| 0x0101u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x004b_61d0, &args![1u32, 2u32, 3u32, 0u32, 1.5f32])
            .bool());
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call_log = Some(vec![]);
        // A signalling NaN is quieted by the FLD/FSTP copy.
        let result = e.call(
            0x004b_61d0,
            &args![1u32, 2u32, 3u32, 0x77u32, 0x7f80_0001u32],
        );
        assert!(result.bool());
        let log = e.call_log.take().unwrap();
        let local = log[1].1[0];
        assert_eq!(
            log[1..],
            [
                (LOCAL_CONSTRUCT, vec![local]),
                (LOCAL_SET_POINT, vec![local, local + 0x10]),
                (LOCAL_SET_FLOAT, vec![local, 0x7fc0_0001]),
                (TEST_WITH_LOCAL, vec![local, 0x77]),
            ]
        );
    }

    fn lookup_fixture(texts: [u32; 2]) -> Engine {
        let mut e = Engine::new();
        e.register(FORM_LOOKUP_KEY, |_, _| 0x51u32.into_ret());
        e.register(TABLE_LOOKUP, |_, _| 0x5200u32.into_ret());
        e.register(NAME_LOCAL_CONSTRUCT, |_, _| 0x5300u32.into_ret());
        e.register(NAME_LOCAL_DESTROY, |_, _| Ret::default());
        e.register(FIND_BY_NAME, |_, _| 0x5400u32.into_ret());
        e.register(GET_HOLDER, |_, _| 0x5500u32.into_ret());
        e.register(GET_ENTRIES, |e, a| {
            e.mem.set_u32(a[1], 2);
            0x9000u32.into_ret()
        });
        e.register(TEXT_LENGTH, |_, _| 4u32.into_ret());
        e.register(ENTRY_OBJECT, |_, a| (a[0] + 1).into_ret());
        e.register_double(OBJECT_TEXT, move |_, a| {
            texts[((a[0] - 0x9001) / 8) as usize].into_ret()
        });
        e.register(STRNICMP, |_, a| (a[0] != 0x6000).into_ret());
        e.register(ENTRY_VALUE, |_, a| (a[0] as f32).into_ret());
        e
    }

    #[test]
    fn fn_004b6220_returns_the_value_of_the_first_matching_entry() {
        let mut e = lookup_fixture([0, 0x6000]);
        e.call_log = Some(vec![]);
        let value = e.call(0x004b_6220, &args![0x10u32, 0x20u32, 0x30u32]).f32();
        assert_eq!(value, 0x9008 as f32);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(TABLE_LOOKUP, vec![LOOKUP_TABLE, 0x51])));
        assert!(log.contains(&(FIND_BY_NAME, vec![0x5200, 0x5300])));
        assert!(log.contains(&(STRNICMP, vec![0x6000, 0x30, 4])));
        assert_eq!(log.iter().filter(|c| c.0 == STRNICMP).count(), 1);

        // No match: 0.0; a null form: 0.0 without calls.
        let mut e = lookup_fixture([0x7000, 0x7000]);
        assert_eq!(
            e.call(0x004b_6220, &args![0x10u32, 0x20u32, 0x30u32]).f32(),
            0.0
        );
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x004b_6220, &args![0u32, 0x20u32, 0x30u32]).f32(),
            0.0
        );
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.register(TABLE_LOOKUP, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x004b_6220, &args![0x10u32, 0x20u32, 0x30u32]).f32(),
            0.0
        );
        assert!(e
            .call_log
            .take()
            .unwrap()
            .iter()
            .all(|c| c.0 != NAME_LOCAL_CONSTRUCT));
    }
}
