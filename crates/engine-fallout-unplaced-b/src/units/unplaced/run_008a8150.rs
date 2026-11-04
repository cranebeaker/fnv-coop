//! `(unplaced)/run_008a8150` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `005d43c0` (`thiscall`): `this + 0x44` (here the object's extra data
/// list).
const EXTRA_LIST_005D43C0: u32 = 0x005d_43c0;
/// `BaseExtraList::GetExtraData` (`00410220`, engine map; `thiscall`, one
/// word: the extra data type).
const GET_EXTRA_DATA: u32 = 0x0041_0220;
/// `008a81c0` (`thiscall`; returns a `bool` in AL; the engine map's name,
/// `BGSLoadFormBuffer::GetVersion`, is unconfirmed for this body).
const EXTRA_TEST_008A81C0: u32 = 0x008a_81c0;

// Translated from 008a8150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte at +0x1C (`thiscall`, one word; no Xbox PDB name).
pub fn fn_008a8150(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x1c, value);
}

// Translated from 008a8170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall`, no Xbox PDB name: false unless the object's vtable +0x22C
/// answers true for 0; then the answer of `008a81c0` on its extra data of
/// type 0x5F (from the list `005d43c0` gives), false without one.
pub fn fn_008a8170(e: &mut Engine, this: Ptr) -> bool {
    if !e.vcall(this.addr(), 0x22c, &args![0u32]).bool() {
        return false;
    }
    let list = e.call(EXTRA_LIST_005D43C0, &args![this]).u32();
    let extra = e.call(GET_EXTRA_DATA, &args![list, 0x5fu32]).u32();
    if extra != 0 {
        e.call(EXTRA_TEST_008A81C0, &args![extra]).bool()
    } else {
        false
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008a_8150, fn_008a8150(Ptr, u8)),
        entry!(0x008a_8170, fn_008a8170(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLOT: u32 = 0x0001_0010;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn fn_008a8150_stores_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.call(0x008a_8150, &args![this, 0x1234_5601u32]);
        assert_eq!(e.mem.u8(this + 0x1c), 1);
        assert_eq!(e.mem.u8(this + 0x1d), 0);
    }

    #[test]
    fn fn_008a8170_tests_the_extra_data() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        let mut slots = vec![0u32; 0x22c / 4 + 1];
        slots[0x22c / 4] = SLOT;
        e.put_vtable(0x0002_0000, &slots);
        e.mem.set_u32(this, 0x0002_0000);
        e.register(SLOT, |_, a| {
            assert_eq!(a[1], 0);
            word(1)
        });
        e.register(EXTRA_LIST_005D43C0, |_, a| word(a[0] + 0x44));
        e.register_double(GET_EXTRA_DATA, move |_, a| {
            assert_eq!(a, &[this + 0x44, 0x5f]);
            word(0x7000)
        });
        e.register(EXTRA_TEST_008A81C0, |_, a| {
            assert_eq!(a[0], 0x7000);
            word(1)
        });
        assert!(e.call(0x008a_8170, &args![this]).bool());

        e.register(GET_EXTRA_DATA, |_, _| word(0));
        assert!(!e.call(0x008a_8170, &args![this]).bool());

        e.register(SLOT, |_, _| word(0));
        e.register(EXTRA_LIST_005D43C0, |_, _| panic!("not reached"));
        assert!(!e.call(0x008a_8170, &args![this]).bool());
    }
}
