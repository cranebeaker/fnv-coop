//! `(unplaced)/run_00cbc550` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00c87c30` (unit BSHavok; thiscall, no arguments).
const BASE_CTOR: u32 = 0x00c8_7c30;
/// `bhkAction`'s vtable (`0102016c`; RTTI `.?AVbhkAction@@`).
const VTABLE_BHK_ACTION: u32 = 0x0102_016c;
/// `bhkUnaryAction`'s vtable (`0102009c`; RTTI `.?AVbhkUnaryAction@@`).
const VTABLE_BHK_UNARY_ACTION: u32 = 0x0102_009c;
/// `bhkLiquidAction`'s vtable (`0108f1bc`; RTTI `.?AVbhkLiquidAction@@`).
const VTABLE_BHK_LIQUID_ACTION: u32 = 0x0108_f1bc;
/// The three instance counts the constructor increments, one per class level.
const COUNT_ACTION: u32 = 0x0126_8154;
const COUNT_UNARY_ACTION: u32 = 0x0126_8118;
const COUNT_LIQUID_ACTION: u32 = 0x0126_841c;
/// The three words copied twice into the object (+0x24 and +0x30).
const DEFAULT_TRIPLE: u32 = 0x011f_426c;
/// The object allocator `00aa13e0` (cdecl: size).
const OBJECT_ALLOCATE: u32 = 0x00aa_13e0;

/// Adds 1 to the 32-bit count at `addr` (`ADD dword ptr [..],ECX` with ECX = 1).
fn bump(e: &mut Engine, addr: u32) {
    let count = e.global::<u32>(addr);
    e.set_global::<u32>(addr, count.wrapping_add(1));
}

// Translated from 00cbc550 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bhkLiquidAction` constructor (unnamed in the engine map; class from the last
/// vtable it sets, 0x40 bytes per its create function `00cbc5e0`): the base constructor
/// (`00c87c30`), then the `bhkAction`, `bhkUnaryAction` and `bhkLiquidAction` vtables in
/// turn, each level's instance count (`01268154`, `01268118`, `0126841c`) incremented;
/// +0xc and +0x10 cleared, the floats +0x14..+0x20 set to 0 (`FLDZ`), the three words at
/// `011f426c` copied to +0x24 and again to +0x30, and the byte at +0x3c set to 1. Returns
/// `this`.
pub fn bhk_liquid_action_ctor(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CTOR, &args![base]);
    e.mem.set_u32(base, VTABLE_BHK_ACTION);
    e.mem.set_u32(base + 0xc, 0);
    bump(e, COUNT_ACTION);
    e.mem.set_u32(base, VTABLE_BHK_UNARY_ACTION);
    bump(e, COUNT_UNARY_ACTION);
    e.mem.set_f32(base + 0x14, 0.0);
    e.mem.set_f32(base + 0x18, 0.0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_f32(base + 0x1c, 0.0);
    e.mem.set_u32(base, VTABLE_BHK_LIQUID_ACTION);
    e.mem.set_f32(base + 0x20, 0.0);
    for copy in [0x24u32, 0x30] {
        for word in 0..3 {
            let value = e.global::<u32>(DEFAULT_TRIPLE + 4 * word);
            e.mem.set_u32(base + copy + 4 * word, value);
        }
    }
    e.mem.set_u8(base + 0x3c, 1);
    bump(e, COUNT_LIQUID_ACTION);
    this
}

// Translated from 00cbc5e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbc5e0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f337af, state variable) is not translated
/// Unnamed in the engine map: the `bhkLiquidAction` create function: 0x40 bytes from
/// `00aa13e0`, constructed (`00cbc550`); null when the allocation failed.
pub fn bhk_liquid_action_create(e: &mut Engine) -> Ptr {
    let block = e.call(OBJECT_ALLOCATE, &args![0x40u32]).u32();
    if block != 0 {
        bhk_liquid_action_ctor(e, Ptr::new(block))
    } else {
        Ptr::new(0)
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb_c550, bhk_liquid_action_ctor(Ptr) -> Ptr),
        entry!(0x00cb_c5e0, bhk_liquid_action_create() -> Ptr),
    ]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    fn globals(e: &mut Engine) {
        e.map(0x0126_8000, 0x1000);
        e.map(DEFAULT_TRIPLE, 12);
        for (i, v) in [0x11u32, 0x22, 0x33].iter().enumerate() {
            e.mem.set_u32(DEFAULT_TRIPLE + 4 * i as u32, *v);
        }
        e.mem.set_u32(COUNT_ACTION, 5);
        e.register(BASE_CTOR, |_, _| Ret::default());
    }

    #[test]
    fn ctor_fills_the_fields_and_counts() {
        let mut e = Engine::new();
        globals(&mut e);
        let this = e.mem.alloc(0x40);
        for off in (0..0x40).step_by(4) {
            e.mem.set_u32(this + off, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cb_c550, &args![this]).u32(), this);
        assert_eq!(e.call_log.take().unwrap()[1], (BASE_CTOR, vec![this]));
        assert_eq!(e.mem.u32(this), VTABLE_BHK_LIQUID_ACTION);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        for off in [0x14u32, 0x18, 0x1c, 0x20] {
            assert_eq!(e.mem.u32(this + off), 0);
        }
        for off in [0x24u32, 0x30] {
            assert_eq!(e.mem.u32(this + off), 0x11);
            assert_eq!(e.mem.u32(this + off + 4), 0x22);
            assert_eq!(e.mem.u32(this + off + 8), 0x33);
        }
        assert_eq!(e.mem.u8(this + 0x3c), 1);
        assert_eq!(e.mem.u8(this + 0x3d), 0xff);
        assert_eq!(e.mem.u32(this + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(COUNT_ACTION), 6);
        assert_eq!(e.mem.u32(COUNT_UNARY_ACTION), 1);
        assert_eq!(e.mem.u32(COUNT_LIQUID_ACTION), 1);
    }

    #[test]
    fn create_constructs_or_answers_null() {
        let mut e = Engine::new();
        globals(&mut e);
        e.register(OBJECT_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00cb_c5e0, &args![]).u32(), 0);
        let block = e.mem.alloc(0x40);
        e.register_double(OBJECT_ALLOCATE, move |_, a| {
            assert_eq!(a, [0x40]);
            block.into_ret()
        });
        assert_eq!(e.call(0x00cb_c5e0, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), VTABLE_BHK_LIQUID_ACTION);
    }
}
