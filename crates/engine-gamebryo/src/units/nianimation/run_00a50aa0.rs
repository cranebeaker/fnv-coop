//! `NiAnimation/run_00a50aa0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplineInterpolator::CopyMembers` (`00a57fd0`, `thiscall (this,
/// dest, cloning process)`; the engine map has no name).
const BASE_COPY_MEMBERS: u32 = 0x00a5_7fd0;

// Translated from 00a50aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineFloatInterpolator::CopyMembers` (body; the engine map has no
/// name): the base class's `CopyMembers`, then the `float` at +0x1C
/// (through the FPU) and the control-point handle at +0x20 copied to
/// `dest`.
pub fn fn_00a50aa0(e: &mut Engine, this: Ptr, dest: Ptr, cloning: Ptr) {
    e.call(BASE_COPY_MEMBERS, &args![this, dest, cloning]);
    let value = x87_quiet_bits(e.mem.u32(this.addr() + 0x1c));
    e.mem.set_u32(dest.addr().wrapping_add(0x1c), value);
    let handle = e.mem.u32(this.addr() + 0x20);
    e.mem.set_u32(dest.addr().wrapping_add(0x20), handle);
}

/// `00aa13e0` (`cdecl`, the size): the Gamebryo allocation the objects are made with;
/// `00a57f20` (unit `nibsplineinterpolator.cpp`, `thiscall`, two words): the base
/// constructor; and `NiBSplineFloatInterpolator`'s vtable (`010997f4`, by its RTTI type
/// descriptor).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const BASE_CONSTRUCT: u32 = 0x00a5_7f20;
const B_SPLINE_FLOAT_INTERPOLATOR_VTABLE: u32 = 0x0109_97f4;

// Translated from 00a50ad0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; no arguments; by its body
/// `NiBSplineFloatInterpolator`'s creation function): 0x24 bytes from `00aa13e0`, the base
/// constructor (`00a57f20` with 0, 0), the vtable `010997f4` and 0xffff at +0x20. Answers the
/// object, null when the allocation fails.
pub fn fn_00a50ad0(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![0x24u32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(BASE_CONSTRUCT, &args![object, 0u32, 0u32]);
    e.mem.set_u32(object, B_SPLINE_FLOAT_INTERPOLATOR_VTABLE);
    e.mem.set_u32(object + 0x20, 0xffff);
    Ptr::new(object)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5_0aa0, fn_00a50aa0(Ptr, Ptr, Ptr)),
        entry!(0x00a5_0ad0, fn_00a50ad0() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn b_spline_float_copy_members_copies_the_value_and_handle() {
        let mut e = Engine::new();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x24);
        let dest = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x1c, 0x7f80_0001);
        e.mem.set_u32(this + 0x20, 0x1234);
        e.call(0x00a5_0aa0, &args![this, dest, 0x7000u32]);
        assert_eq!(e.mem.u32(dest + 0x1c), 0x7fc0_0001);
        assert_eq!(e.mem.u32(dest + 0x20), 0x1234);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == BASE_COPY_MEMBERS && call.1 == vec![this, dest, 0x7000]));
    }
}

#[cfg(test)]
mod n007_tests {
    use super::*;

    #[test]
    fn creation_function_allocates_and_constructs() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let object = e.call(0x00a5_0ad0, &[]).u32();
        assert_eq!(e.mem.u32(object), B_SPLINE_FLOAT_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(object + 0x20), 0xffff);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|c| c.0 == BASE_CONSTRUCT && c.1 == vec![object, 0, 0]));
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a5_0ad0, &[]).u32(), 0);
    }
}
