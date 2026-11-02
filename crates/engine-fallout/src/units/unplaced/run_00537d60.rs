//! `(unplaced)/run_00537d60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_13e0;
/// The six one-word `thiscall` setters (`RET 4`) both functions call on the 0x20-byte
/// object, in their order. The engine map's names for them come from identical-code folding
/// (`BGSUnloadedFormBuffer::BGSUnloadedFormBuffer`, `CombatProcedureAttackMelee::Initialize`,
/// an unnamed one, `NonActorMagicCaster::SetCurrentSpell`, an unnamed one, and one in
/// `extradataobjects.cpp`); which fields they set is not confirmed here.
const SETTERS: [u32; 6] = [
    0x0053_7e90,
    0x006e_cd40,
    0x0040_3550,
    0x0041_fd00,
    0x0060_0ab0,
    0x0043_7730,
];

// Translated from 00537d60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00537d60 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef5a1f, state variable) is not translated
/// Unnamed in the engine map (`cdecl`, five words): allocates 0x20 bytes (`00aa13e0`) and
/// constructs them ([`fn_00537e30`]; a failed allocation gives null); for a non-null object
/// calls the six setters of [`SETTERS`] with, in order, `fifth`, `first`, `second`, `third`,
/// `fourth` and 0. Returns the object.
pub fn fn_00537d60(
    e: &mut Engine,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
) -> Ptr {
    let block = e.call(ALLOCATE_BLOCK, &args![0x20u32]).u32();
    let object = if block != 0 {
        fn_00537e30(e, Ptr::new(block))
    } else {
        Ptr::new(0)
    };
    if !object.is_null() {
        for (setter, value) in SETTERS
            .into_iter()
            .zip([fifth, first, second, third, fourth, 0])
        {
            e.call(setter, &args![object, value]);
        }
    }
    object
}

// Translated from 00537e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): the constructor of the 0x20-byte
/// object `00537d60` makes. Calls each of the six setters of [`SETTERS`] with 0, then clears
/// the words at +0x18 and +0x1c. Returns `this`.
pub fn fn_00537e30(e: &mut Engine, this: Ptr) -> Ptr {
    for setter in SETTERS {
        e.call(setter, &args![this, 0u32]);
    }
    e.mem.set_u32(this.addr() + 0x18, 0);
    e.mem.set_u32(this.addr() + 0x1c, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0053_7d60, fn_00537d60(u32, u32, u32, u32, u32) -> Ptr),
        entry!(0x0053_7e30, fn_00537e30(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Engine {
        let mut e = Engine::new();
        for setter in SETTERS {
            e.register(setter, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn constructor_clears_through_the_setters() {
        let mut e = fixture();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x18, 5);
        e.mem.set_u32(this + 0x1c, 6);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0053_7e30, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let expected: Vec<_> = SETTERS.iter().map(|&s| (s, vec![this, 0])).collect();
        assert_eq!(log[1..], expected[..]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
    }

    #[test]
    fn maker_sets_the_five_values_in_the_exe_order() {
        let mut e = fixture();
        let block = e.mem.alloc(0x20);
        e.register_double(ALLOCATE_BLOCK, move |_, _| block.into_ret());
        e.call_log = Some(vec![]);
        let made = e
            .call(0x0053_7d60, &args![11u32, 22u32, 33u32, 44u32, 55u32])
            .u32();
        assert_eq!(made, block);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ALLOCATE_BLOCK, vec![0x20]));
        let setters: Vec<_> = log.iter().skip(8).cloned().collect();
        let expected: Vec<_> = SETTERS
            .iter()
            .zip([55u32, 11, 22, 33, 44, 0])
            .map(|(&s, v)| (s, vec![block, v]))
            .collect();
        assert_eq!(setters, expected);

        // A failed allocation: null and no setter.
        e.register(ALLOCATE_BLOCK, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0053_7d60, &args![1u32, 2u32, 3u32, 4u32, 5u32])
                .u32(),
            0
        );
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
