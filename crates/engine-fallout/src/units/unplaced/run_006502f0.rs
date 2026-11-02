//! `(unplaced)/run_006502f0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The static lock and unlock around the face-gen map (`00652140` and
/// `00652190`, no arguments), the `NiPointer` local's constructor
/// (`00633c90`, `thiscall (this, object)`) and destructor (`0045cec0`,
/// `thiscall`), the map lookup (`006c62d0`, `thiscall (map, key,
/// &pointer)`, `bool` in `AL`), the pointer getter (`00559450`, `thiscall`,
/// answers the word at `this`), the `NiPointer` assignment (`006e5cc0`,
/// `thiscall (pointer, &source)`), the getter `0044ddc0` (`thiscall`) and
/// the setter `0044b300` (`thiscall (this, value)`).
const LOCK: u32 = 0x0065_2140;
const UNLOCK: u32 = 0x0065_2190;
const POINTER_CONSTRUCTOR: u32 = 0x0063_3c90;
const POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
const MAP_LOOKUP: u32 = 0x006c_62d0;
const POINTER_GET: u32 = 0x0055_9450;
const POINTER_ASSIGN: u32 = 0x006e_5cc0;
const GET_VALUE: u32 = 0x0044_ddc0;
const SET_VALUE: u32 = 0x0044_b300;

// Translated from 006502f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006502f0 - its C++ exception frame (FS:[0], the pointer local's unwind) is not translated
/// Looks `key` up in the map at +4 under the static lock (the engine map
/// has no name; `thiscall (key, &pointer, target)`, `RET 0xC`): when found,
/// `pointer` is assigned from the found entry's +8, and `target` is given
/// (`0044b300`) the value `0044ddc0` reads from the pointed object.
/// Answers whether it was found. The `NiPointer` local is constructed
/// empty and destroyed at the end, after the unlock.
pub fn fn_006502f0(e: &mut Engine, this: Ptr, key: u32, pointer: Ptr, target: Ptr) -> bool {
    e.call(LOCK, &args![]);
    e.with_stack(4, |e, local| {
        e.call(POINTER_CONSTRUCTOR, &args![local, 0u32]);
        let mut found = false;
        if e.call(MAP_LOOKUP, &args![this.addr() + 4, key, local])
            .bool()
        {
            let entry = e.call(POINTER_GET, &args![local]).u32();
            e.call(POINTER_ASSIGN, &args![pointer, entry.wrapping_add(8)]);
            let object = e.call(POINTER_GET, &args![pointer]).u32();
            let value = e.call(GET_VALUE, &args![object]).u32();
            e.call(SET_VALUE, &args![target, value]);
            found = true;
        }
        e.call(UNLOCK, &args![]);
        e.call(POINTER_DESTRUCTOR, &args![local]);
        found
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0065_02f0, fn_006502f0(Ptr, u32, Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lookup_engine(found: bool) -> Engine {
        let mut e = Engine::new();
        for addr in [
            LOCK,
            UNLOCK,
            POINTER_CONSTRUCTOR,
            POINTER_DESTRUCTOR,
            POINTER_ASSIGN,
            SET_VALUE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(GET_VALUE, |_, a| (a[0] + 1).into_ret());
        e.register_double(MAP_LOOKUP, move |e, a| {
            if found {
                e.mem.set_u32(a[2], 0x5000);
            }
            u32::from(found).into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn found_entries_fill_the_pointer_and_the_target() {
        let mut e = lookup_engine(true);
        let this = e.mem.alloc(0x10);
        let pointer = e.mem.alloc(4);
        e.mem.set_u32(pointer, 0x6000);
        assert!(e
            .call(0x0065_02f0, &args![this, 0x77u32, pointer, 0x8000u32])
            .bool());
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|call| call.0).collect();
        assert_eq!(
            order,
            vec![
                LOCK,
                POINTER_CONSTRUCTOR,
                MAP_LOOKUP,
                POINTER_GET,
                POINTER_ASSIGN,
                POINTER_GET,
                GET_VALUE,
                SET_VALUE,
                UNLOCK,
                POINTER_DESTRUCTOR
            ]
        );
        assert_eq!(log[3].1[..2], [this + 4, 0x77]);
        assert_eq!(log[5].1, vec![pointer, 0x5008]);
        assert_eq!(log[8].1, vec![0x8000, 0x6001]);
    }

    #[test]
    fn missing_entries_answer_false() {
        let mut e = lookup_engine(false);
        let this = e.mem.alloc(0x10);
        assert!(!e
            .call(0x0065_02f0, &args![this, 0x77u32, 0x6000u32, 0x8000u32])
            .bool());
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|call| call.0 == SET_VALUE));
        assert!(log.iter().any(|call| call.0 == UNLOCK));
    }
}
