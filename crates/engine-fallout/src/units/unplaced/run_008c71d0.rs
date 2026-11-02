//! `(unplaced)/run_008c71d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00ec782f` (`RET 0x14`: array, element size, count, element constructor, element
/// destructor): constructs an array of elements.
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// The element constructor and destructor handed to it.
const ELEMENT_CONSTRUCTOR_008C7250: u32 = 0x008c_7250;
const ELEMENT_DESTRUCTOR_008C7270: u32 = 0x008c_7270;

// Translated from 008c71d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: constructs three 0xc-byte elements at `this + 0x18` through
/// `00ec782f` (constructor `008c7250`, destructor `008c7270`), clears the three words at `+0`
/// and the three words at `+0xc`, and stores the addresses `this + 0x3c` at `+0x8c` and
/// `this + 0x6c` at `+0x90`. Returns `this`.
pub fn fn_008c71d0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            base + 0x18,
            0xcu32,
            3u32,
            ELEMENT_CONSTRUCTOR_008C7250,
            ELEMENT_DESTRUCTOR_008C7270
        ],
    );
    for index in 0..3u32 {
        e.mem.set_u32(base + index * 4, 0);
        e.mem.set_u32(base + 0xc + index * 4, 0);
    }
    e.mem.set_u32(base + 0x8c, base + 0x3c);
    e.mem.set_u32(base + 0x90, base + 0x6c);
    this
}

/// `0086c6a0` (`thiscall`, `RET 8`, `fallout/misc/main.cpp`): the element's
/// constructor body, given 0 and 0x14.
const ELEMENT_CONSTRUCT_BODY: u32 = 0x0086_c6a0;
/// `0055a2d0` (`thiscall`, plain `RET`): the element's destructor body.
const ELEMENT_DESTRUCT_BODY: u32 = 0x0055_a2d0;

// Translated from 008c7250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The element constructor `008c71d0` hands to the vector constructor
/// (unoptimized code): `0086c6a0(this, 0, 0x14)`; returns `this`.
pub fn fn_008c7250(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(ELEMENT_CONSTRUCT_BODY, &args![this, 0u32, 0x14u32]);
    this
}

// Translated from 008c7270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The element destructor `008c71d0` hands to the vector constructor
/// (unoptimized code): `0055a2d0(this)`.
pub fn fn_008c7270(e: &mut Engine, this: Ptr) {
    e.call(ELEMENT_DESTRUCT_BODY, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008c_71d0, fn_008c71d0(Ptr) -> Ptr),
        entry!(0x008c_7250, fn_008c7250(Ptr) -> Ptr),
        entry!(0x008c_7270, fn_008c7270(Ptr)),
    ]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn constructs_the_elements_and_clears_the_words() {
        let mut e = Engine::new();
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| Ret::default());
        let this = e.mem.alloc(0xa0);
        for i in 0..6u32 {
            e.mem.set_u32(this + i * 4, 0xffff_ffff);
        }
        e.mem.set_u32(this + 0x18, 0xeeee_eeee);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008c_71d0, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![this + 0x18, 0xc, 3, 0x008c_7250, 0x008c_7270]]
        );
        for i in 0..6u32 {
            assert_eq!(e.mem.u32(this + i * 4), 0);
        }
        assert_eq!(e.mem.u32(this + 0x18), 0xeeee_eeee);
        assert_eq!(e.mem.u32(this + 0x8c), this + 0x3c);
        assert_eq!(e.mem.u32(this + 0x90), this + 0x6c);
    }

    #[test]
    fn element_constructor_and_destructor_run_their_bodies() {
        let mut e = Engine::new();
        e.register(ELEMENT_CONSTRUCT_BODY, |_, _| Ret::default());
        e.register(ELEMENT_DESTRUCT_BODY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008c_7250, &args![0x1000u32]).u32(), 0x1000);
        e.call(0x008c_7270, &args![0x1000u32]);
        assert_eq!(
            calls_to(&mut e, ELEMENT_CONSTRUCT_BODY),
            vec![vec![0x1000, 0, 0x14]]
        );
        assert_eq!(calls_to(&mut e, ELEMENT_DESTRUCT_BODY), vec![vec![0x1000]]);
    }
}
