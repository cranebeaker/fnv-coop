//! `BSMain/run_00c60100` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The pooled `NiFixedString` maker (`00a5b690`, `cdecl`: text) and the empty handle
/// (`0109b220`, never counted).
const FIXED_STRING_FROM_TEXT: u32 = 0x00a5_b690;
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiPSysModifier`'s constructor (`00c316a0`; `thiscall`, `RET 8`: address of the name
/// handle, order).
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_16a0;
/// The `BSPSysSimpleColorModifier` virtual table (`010c3384`, by RTTI).
const SIMPLE_COLOR_MODIFIER_VTABLE: u32 = 0x010c_3384;

// Translated from 00c60100 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c60100 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f30018, state variable) is not translated
/// The `BSPSysSimpleColorModifier` constructor (by its virtual table; unnamed in the engine
/// map; `thiscall`, `RET 0x28`: name text, three colours by address, six `float`s). The
/// name becomes a pooled handle in a stack local (`00a5b690`; the empty handle for a null
/// text), `NiPSysModifier`'s constructor (`00c316a0`) gets its address and order 3000, then
/// the handle is released (unless it is the empty one). Then the class's virtual table, the
/// three 4-word colours at +0x18, +0x28 and +0x38 and the six `float`s at +0x48 to +0x5C
/// (copied through the x87 stack). Answers `this`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00c60100(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    first_color: Ptr,
    second_color: Ptr,
    third_color: Ptr,
    first: f32,
    second: f32,
    third: f32,
    fourth: f32,
    fifth: f32,
    sixth: f32,
) -> Ptr {
    e.with_stack(4, |e, slot| {
        let handle = if name == 0 {
            e.mem.u32(FIXED_STRING_EMPTY)
        } else {
            e.call(FIXED_STRING_FROM_TEXT, &args![name]).u32()
        };
        e.mem.set_u32(slot.addr(), handle);
        e.call(MODIFIER_CONSTRUCTOR, &args![this, slot, 0xbb8u32]);
        let handle = e.mem.u32(slot.addr());
        if handle != e.mem.u32(FIXED_STRING_EMPTY) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
    });
    let address = this.addr();
    e.mem.set_u32(address, SIMPLE_COLOR_MODIFIER_VTABLE);
    for (offset, color) in [
        (0x18u32, first_color),
        (0x28, second_color),
        (0x38, third_color),
    ] {
        for k in 0..4u32 {
            let word = e.mem.u32(color.addr() + 4 * k);
            e.mem.set_u32(address + offset + 4 * k, word);
        }
    }
    for (k, value) in [first, second, third, fourth, fifth, sixth]
        .iter()
        .enumerate()
    {
        e.mem.set_u32(
            address + 0x48 + 4 * k as u32,
            x87_quiet_bits(value.to_bits()),
        );
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c6_0100,
        fn_00c60100(Ptr, u32, Ptr, Ptr, Ptr, f32, f32, f32, f32, f32, f32) -> Ptr
    )]
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn colours_and_floats_follow_the_base() {
        let mut e = Engine::new();
        e.map(FIXED_STRING_EMPTY, 4);
        e.mem.set_u32(FIXED_STRING_EMPTY, 0x0500_0008);
        e.register(FIXED_STRING_FROM_TEXT, |_, _| 0x0600_0008u32.into_ret());
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        let colors: Vec<u32> = (0..3)
            .map(|c| {
                let block = e.mem.alloc(16);
                for k in 0..4 {
                    e.mem.set_u32(block + 4 * k, 0x10 * (c + 1) + k);
                }
                block
            })
            .collect();
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        let result = e.call(
            0x00c6_0100,
            &args![
                this,
                0x0700_0000u32,
                colors[0],
                colors[1],
                colors[2],
                1.0f32,
                2.0f32,
                0x7f80_0001u32,
                4.0f32,
                5.0f32,
                6.0f32
            ],
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(result.u32(), this);
        assert!(log
            .iter()
            .any(|c| c.0 == MODIFIER_CONSTRUCTOR && c.1[2] == 3000));
        assert!(log.contains(&(INTERLOCKED_DECREMENT, vec![0x0600_0000])));
        assert_eq!(e.mem.u32(this), SIMPLE_COLOR_MODIFIER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), 0x10);
        assert_eq!(e.mem.u32(this + 0x34), 0x23);
        assert_eq!(e.mem.u32(this + 0x44), 0x33);
        assert_eq!(e.mem.f32(this + 0x48), 1.0);
        assert_eq!(e.mem.u32(this + 0x50), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0x5c), 6.0);
    }
}
