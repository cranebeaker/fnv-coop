//! `(unplaced)/run_00b9fba0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00aa13e0`, `cdecl (size)`), the `NiTriShape`
/// constructor (`00a74480`, `thiscall (this, data)`), the pooled-string
/// maker (`00a5b690`, `cdecl (text)`), `NiObjectNET`'s name setter
/// (`00a5b950`, `thiscall (object, &name)`) and the particle-system call
/// given the shape's slot (`00c1b4a0`, `thiscall (this, &slot)`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const TRI_SHAPE_CONSTRUCTOR: u32 = 0x00a7_4480;
const FIXED_STRING_FROM_TEXT: u32 = 0x00a5_b690;
const SET_NAME: u32 = 0x00a5_b950;
const ATTACH_SHAPE: u32 = 0x00c1_b4a0;
/// The shape's name (`010b7ec0`).
const SHAPE_NAME: u32 = 0x010b_7ec0;
/// `InterlockedIncrement` and `InterlockedDecrement` (import slots, one word:
/// the address).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The pooled empty `NiFixedString` handle (`0109b220`), never counted.
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;

// Translated from 00b9fba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b9fba0 - its C++ exception frame (FS:[0], the new shape's and the name's unwind) is not translated
/// Makes sure the object has its shape (the engine map has no name): when
/// the `NiPointer` at +0x13C is empty, a new `NiTriShape` (0xC4 bytes,
/// constructed with no data; null when the allocation fails) is assigned
/// to it (old one released, new one counted) and named with the string
/// at `010b7ec0` (the temporary handle released after). Then `00c1b4a0` is
/// called on +0xE0 with the slot's address and the word at +0xE0 copied to
/// +0x138.
pub fn fn_00b9fba0(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    let slot = object + 0x13c;
    if e.mem.u32(slot) == 0 {
        let block = e.call(OPERATOR_NEW, &args![0xc4u32]).u32();
        let shape = if block != 0 {
            e.call(TRI_SHAPE_CONSTRUCTOR, &args![block, 0u32]).u32()
        } else {
            0
        };
        let old = e.mem.u32(slot);
        if old != shape {
            if old != 0
                && e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
                    .u32()
                    == 0
            {
                e.vcall(old, 4, &args![]);
            }
            e.mem.set_u32(slot, shape);
            if shape != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![shape.wrapping_add(4)]);
            }
        }
        let name = e.call(FIXED_STRING_FROM_TEXT, &args![SHAPE_NAME]).u32();
        e.with_stack(4, |e, handle| {
            e.mem.set_u32(handle.addr(), name);
            let shape = e.mem.u32(slot);
            e.call(SET_NAME, &args![shape, handle]);
            let name = e.mem.u32(handle.addr());
            if name != e.global::<u32>(FIXED_STRING_EMPTY) {
                e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
            }
        });
    }
    e.call(ATTACH_SHAPE, &args![object + 0xe0, slot]);
    let value = e.mem.u32(object + 0xe0);
    e.mem.set_u32(object + 0x138, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b9_fba0, fn_00b9fba0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shape_engine() -> Engine {
        let mut e = Engine::new();
        e.map(FIXED_STRING_EMPTY, 4);
        e.set_global(FIXED_STRING_EMPTY, 0x0500_0008u32);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(TRI_SHAPE_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(SET_NAME, |_, _| Ret::default());
        e.register(ATTACH_SHAPE, |e, a| {
            e.mem.set_u32(a[0], 0x4444);
            Ret::default()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e
    }

    #[test]
    fn a_missing_shape_is_made_named_and_attached() {
        let mut e = shape_engine();
        let name = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(name - 8, 1);
        e.register_double(FIXED_STRING_FROM_TEXT, move |_, _| name.into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x140);
        e.call(0x00b9_fba0, &args![this]);
        let shape = e.mem.u32(this + 0x13c);
        assert_ne!(shape, 0);
        assert_eq!(e.mem.u32(shape + 4), 1);
        assert_eq!(e.mem.u32(name - 8), 0);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == FIXED_STRING_FROM_TEXT && call.1 == vec![SHAPE_NAME]));
        assert!(log
            .iter()
            .any(|call| call.0 == SET_NAME && call.1[0] == shape));
        assert!(log
            .iter()
            .any(|call| call.0 == ATTACH_SHAPE && call.1 == vec![this + 0xe0, this + 0x13c]));
        assert_eq!(e.mem.u32(this + 0x138), 0x4444);
    }

    #[test]
    fn an_existing_shape_is_only_attached() {
        let mut e = shape_engine();
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x140);
        e.mem.set_u32(this + 0x13c, 0x9000);
        e.call(0x00b9_fba0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|call| call.0 == OPERATOR_NEW));
        assert_eq!(e.mem.u32(this + 0x138), 0x4444);
    }
}
