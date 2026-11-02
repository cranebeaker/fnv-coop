//! `(unplaced)/run_00c312a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Import slot of `InterlockedDecrement` (`stdcall`, one word: the address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The handle of the empty `NiFixedString`, which is never counted.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// `00a6d290`: the base destructor, reached with a `JMP` (same `this`).
const BASE_DESTRUCTOR: u32 = 0x00a6_d290;

// Translated from 00c312a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): a destructor. Sets the vtable `010c08ac`, releases the fixed-string
/// handle at +0x38 (unless it is the empty string's), drops one reference of the object at
/// +0x34 when set (`InterlockedDecrement` of its count at +4; at zero its vtable slot +4
/// runs), then continues in the base destructor `00a6d290` (a `JMP`; its result is passed
/// on). The decompiler shows the empty-string comparison as a comparison with 0; the
/// disassembly compares with the global `0109b220`.
pub fn fn_00c312a0(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    e.mem.set_u32(base, 0x010c_08ac);
    let name = e.mem.u32(base + 0x38);
    if name != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
    }
    let object = e.mem.u32(base + 0x34);
    if object != 0 {
        let count = e
            .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32();
        if count == 0 {
            e.vcall(object, 4, &args![]);
        }
    }
    e.call(BASE_DESTRUCTOR, &args![this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c3_12a0, fn_00c312a0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn releases_the_name_and_the_object_then_runs_the_base() {
        let mut e = Engine::new();
        e.map(EMPTY_FIXED_STRING & !0xfff, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x4444);
        e.register(BASE_DESTRUCTOR, |_, _| 0x31u32.into_ret());
        e.put_vtable(0x0150_0000, &[0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        let object = e.mem.alloc(8);
        e.register_double(INTERLOCKED_DECREMENT, move |e, a| {
            if a[0] == object + 4 {
                e.mem.u32(a[0]).wrapping_sub(1).into_ret()
            } else {
                1u32.into_ret()
            }
        });
        e.mem.set_u32(object, 0x0150_0000);
        e.mem.set_u32(object + 4, 1);
        e.mem.set_u32(this + 0x34, object);
        e.mem.set_u32(this + 0x38, 0x1008);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_12a0, &args![this]).u32(), 0x31);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (INTERLOCKED_DECREMENT, vec![0x1000]),
                (INTERLOCKED_DECREMENT, vec![object + 4]),
                (0x0150_1000, vec![object]),
                (BASE_DESTRUCTOR, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x010c_08ac);

        // Empty name, no object.
        e.mem.set_u32(this + 0x34, 0);
        e.mem.set_u32(this + 0x38, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x00c3_12a0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(BASE_DESTRUCTOR, vec![this])]);
    }
}
