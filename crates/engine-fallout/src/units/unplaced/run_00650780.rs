//! `(unplaced)/run_00650780` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The static lock and unlock around the face-gen map (`00652140` and
/// `00652190`, no arguments), the `NiPointer` local's constructor
/// (`00633c90`, `thiscall (this, object)`) and destructor (`0045cec0`,
/// `thiscall`), the map lookup (`006c62d0`, `thiscall (map, key,
/// &pointer)`, `bool` in `AL`), the pointer getter (`00559450`, `thiscall`,
/// answers the word at `this`) and the `thiscall` the found object gets
/// (`0065c400`).
const LOCK: u32 = 0x0065_2140;
const UNLOCK: u32 = 0x0065_2190;
const POINTER_CONSTRUCTOR: u32 = 0x0063_3c90;
const POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
const MAP_LOOKUP: u32 = 0x006c_62d0;
const POINTER_GET: u32 = 0x0055_9450;
const FOUND_OBJECT_CALL: u32 = 0x0065_c400;
/// The CRT string helpers (`cdecl`): copy (`00406d30`, `(dest, size,
/// source)`), find last character (`0040ab30`, `(text, character)`) and
/// append (`00406d50`, `(dest, size, source)`); the suffix appended
/// (`010656a4`).
const STRING_COPY: u32 = 0x0040_6d30;
const STRING_FIND_LAST: u32 = 0x0040_ab30;
const STRING_APPEND: u32 = 0x0040_6d50;
const NAME_SUFFIX: u32 = 0x0106_56a4;

// Translated from 00650780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00650780 - its C++ exception frame (FS:[0], the pointer local's unwind) and the /GS cookie check (00ec408c) are not translated
/// Looks `name` up in the map at +4 under the static lock (the engine map
/// has no name; `thiscall (name)`, `RET 4`); a null or empty name answers
/// false at once. When the name is not found, a copy of it (at most 0x104
/// bytes) cut at its last `.` and given the suffix at `010656a4` is looked
/// up instead (no second try without a `.`). When the `NiPointer` local
/// then holds an entry, the object at the entry's +8 gets `0065c400` and
/// true is answered.
pub fn fn_00650780(e: &mut Engine, this: Ptr, name: Ptr) -> bool {
    if name.is_null() || e.mem.i8(name.addr()) == 0 {
        return false;
    }
    e.with_stack(0x108, |e, frame| {
        let local = frame.addr();
        let buffer = local + 4;
        e.call(POINTER_CONSTRUCTOR, &args![local, 0u32]);
        e.call(LOCK, &args![]);
        let map = this.addr() + 4;
        if !e.call(MAP_LOOKUP, &args![map, name, local]).bool() {
            e.call(STRING_COPY, &args![buffer, 0x104u32, name]);
            let dot = e
                .call(STRING_FIND_LAST, &args![buffer, u32::from(b'.')])
                .u32();
            if dot != 0 {
                e.mem.set_u8(dot, 0);
                e.call(STRING_APPEND, &args![buffer, 0x104u32, NAME_SUFFIX]);
                e.call(MAP_LOOKUP, &args![map, buffer, local]);
            }
        }
        let mut found = false;
        if e.call(POINTER_GET, &args![local]).u32() != 0 {
            let entry = e.call(POINTER_GET, &args![local]).u32();
            let object = e.call(POINTER_GET, &args![entry.wrapping_add(8)]).u32();
            e.call(FOUND_OBJECT_CALL, &args![object]);
            found = true;
        }
        e.call(UNLOCK, &args![]);
        e.call(POINTER_DESTRUCTOR, &args![local]);
        found
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0065_0780, fn_00650780(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Doubles: the lookup finds the entry `0x5000` for the key `found`
    /// (compared as text), the string helpers act on game memory.
    fn named_engine(found: &'static [u8]) -> Engine {
        let mut e = Engine::new();
        for addr in [
            LOCK,
            UNLOCK,
            POINTER_CONSTRUCTOR,
            POINTER_DESTRUCTOR,
            FOUND_OBJECT_CALL,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register_double(MAP_LOOKUP, move |e, a| {
            let hit = e.mem.cstr(a[1]) == found;
            if hit {
                e.mem.set_u32(a[2], 0x5000);
            }
            u32::from(hit).into_ret()
        });
        e.register(STRING_COPY, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRING_FIND_LAST, |e, a| {
            let text = e.mem.cstr(a[0]);
            match text.iter().rposition(|&c| u32::from(c) == a[1]) {
                Some(i) => (a[0] + i as u32).into_ret(),
                None => Ret::default(),
            }
        });
        e.register(STRING_APPEND, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.map(NAME_SUFFIX, 8);
        e.mem.set_cstr(NAME_SUFFIX, b".egm");
        e.map(0x5000, 0x10);
        e.mem.set_u32(0x5008, 0x7000);
        e.call_log = Some(vec![]);
        e
    }

    fn found_calls(e: &mut Engine) -> Vec<Vec<u32>> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|call| call.0 == FOUND_OBJECT_CALL)
            .map(|call| call.1)
            .collect()
    }

    #[test]
    fn a_found_name_calls_the_entry_object() {
        let mut e = named_engine(b"head.nif");
        let this = e.mem.alloc(0x10);
        let name = e.mem.alloc(0x10);
        e.mem.set_cstr(name, b"head.nif");
        assert!(e.call(0x0065_0780, &args![this, name]).bool());
        assert_eq!(found_calls(&mut e), vec![vec![0x7000]]);
    }

    #[test]
    fn a_missing_name_retries_with_the_suffix() {
        let mut e = named_engine(b"head.egm");
        let this = e.mem.alloc(0x10);
        let name = e.mem.alloc(0x10);
        e.mem.set_cstr(name, b"head.nif");
        assert!(e.call(0x0065_0780, &args![this, name]).bool());
        assert_eq!(found_calls(&mut e), vec![vec![0x7000]]);
        // Without a dot there is no retry.
        let mut e = named_engine(b"head.egm");
        let this = e.mem.alloc(0x10);
        let name = e.mem.alloc(0x10);
        e.mem.set_cstr(name, b"head");
        assert!(!e.call(0x0065_0780, &args![this, name]).bool());
        assert!(found_calls(&mut e).is_empty());
    }

    #[test]
    fn a_null_or_empty_name_answers_false() {
        let mut e = named_engine(b"x");
        let this = e.mem.alloc(0x10);
        let name = e.mem.alloc(4);
        assert!(!e.call(0x0065_0780, &args![this, name]).bool());
        assert!(!e.call(0x0065_0780, &args![this, 0u32]).bool());
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
