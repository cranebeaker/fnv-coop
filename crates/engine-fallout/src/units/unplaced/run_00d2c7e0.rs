//! `(unplaced)/run_00d2c7e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The constructor `00d2c7e0` (`this` plus one stack word, `RET 4`).
const CONSTRUCTOR_00D2C7E0: u32 = 0x00d2_c7e0;
/// `__security_check_cookie` (`00ec408c`, fastcall: the cookie in `ECX`).
const SECURITY_CHECK_COOKIE: u32 = 0x00ec_408c;
/// The security cookie (`.data`).
const SECURITY_COOKIE_011C16BC: u32 = 0x011c_16bc;
/// The constant the function returns (an address in `.data`; what it points at is unconfirmed).
const RETURNED_ADDRESS_010CE740: u32 = 0x010c_e740;

// Translated from 00d2c8a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d2c8a0 - the compiler's exception frame (FS:[0] chain) is not
// translated; the stack cookie check is (through `__security_check_cookie`).
/// Unnamed in the engine map: builds a 0x38-byte object in a local, on the stack (preceded
/// by a pointer to it), with `00d2c7e0(object, 0)`, checks the stack cookie and returns the
/// constant `010ce740`. The object is not destroyed here. The stack-aligned locals are the
/// engine's `with_stack` block.
pub fn fn_00d2c8a0(e: &mut Engine) -> u32 {
    e.with_stack(0x3c, |e, frame| {
        let object = frame.addr() + 4;
        e.mem.set_u32(frame.addr(), object);
        e.call(CONSTRUCTOR_00D2C7E0, &args![object, 0u32]);
    });
    // The cookie stored in the frame is the global XOR the frame address, and the check
    // XORs the frame address back: the value checked is the global itself.
    let cookie = e.global::<u32>(SECURITY_COOKIE_011C16BC);
    e.call(SECURITY_CHECK_COOKIE, &args![cookie]);
    RETURNED_ADDRESS_010CE740
}

/// `hkpAction` vtable (RTTI).
const VTABLE_HKP_ACTION: u32 = 0x010c_c2fc;
/// `00cb0a70` (unit `Havok SDK/hkstringptr.cpp` in the engine map; thiscall, `RET 4`).
const STRING_POINTER_INIT: u32 = 0x00cb_0a70;

// Translated from 00d2c7e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d2c7e0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f34868, state variable) is not translated
/// A constructor that stores the `hkpAction` vtable (RTTI) and runs `00cb0a70` on the
/// member at +0x14 with `argument`. Returns `this`.
pub fn fn_00d2c7e0(e: &mut Engine, this: Ptr, argument: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_HKP_ACTION);
    e.call(STRING_POINTER_INIT, &args![base + 0x14, argument]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_c8a0, fn_00d2c8a0() -> u32),
        entry!(0x00d2_c7e0, fn_00d2c7e0(Ptr, u32) -> Ptr),
        entry!(0x00d2_c7d0, fn_00d2c7d0(Ptr) -> u32),
        entry!(0x00d2_c840, fn_00d2c840(Ptr, u32)),
    ]
}

/// The vtable `00d2c840` installs after the constructor (`010ce740`, the address
/// `fn_00d2c8a0` answers).
const VTABLE_010CE740: u32 = 0x010c_e740;

// Translated from 00d2c7d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the Ghidra project (bytes disassembled; referenced as data from
/// `00fbeb19`): cdecl, one word: calls the object's vtable slot +0 with 0 (the deleting
/// destructor's shape, without freeing) and passes on its `EAX`.
pub fn fn_00d2c7d0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0x0, &args![0u32]).u32()
}

// Translated from 00d2c840 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d2c840 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f348a1, state variable) is not translated
/// Unnamed in the engine map (cdecl, two words; referenced as data from `00fbeb0f`): when
/// `block` is not null, constructs it in place (`00d2c7e0` with `argument`) and sets the
/// vtable `010ce740`. Nothing is answered (`EAX` is meaningless on the null path).
pub fn fn_00d2c840(e: &mut Engine, block: Ptr, argument: u32) {
    if block.addr() != 0 {
        fn_00d2c7e0(e, block, argument);
        e.mem.set_u32(block.addr(), VTABLE_010CE740);
    }
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn fn_00d2c7d0_calls_slot_0_with_zero_and_passes_on_eax() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, 0x0079_0000);
        e.register(0x0079_0000, |_, a| (a[0] + a[1] + 1).into_ret());
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, vtable);
        assert_eq!(e.call(0x00d2_c7d0, &args![object]).u32(), object + 1);
    }

    #[test]
    fn fn_00d2c840_builds_only_a_real_block() {
        let mut e = Engine::new();
        e.register(STRING_POINTER_INIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00d2_c840, &args![0u32, 5u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        let block = e.mem.alloc(0x38);
        e.call_log = Some(vec![]);
        e.call(0x00d2_c840, &args![block, 5u32]);
        assert_eq!(e.mem.u32(block), VTABLE_010CE740);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (STRING_POINTER_INIT, vec![block + 0x14, 5]));
    }
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
    fn constructs_the_object_checks_the_cookie_and_returns_the_address() {
        let mut e = Engine::new();
        e.map(0x011c_1000, 0x1000);
        e.set_global(SECURITY_COOKIE_011C16BC, 0xdead_beefu32);
        e.register(CONSTRUCTOR_00D2C7E0, |e, a| {
            e.mem.set_u32(a[0], 0x42);
            Ret::default()
        });
        e.register(SECURITY_CHECK_COOKIE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d2_c8a0, &args![]).u32(), 0x010c_e740);
        let built = calls_to(&mut e, CONSTRUCTOR_00D2C7E0);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1], 0);
        assert_ne!(built[0][0], 0);
        assert_eq!(
            calls_to(&mut e, SECURITY_CHECK_COOKIE),
            vec![vec![0xdead_beef]]
        );
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    #[test]
    fn constructor_sets_the_vtable_and_initializes_the_member() {
        let mut e = Engine::new();
        e.register(STRING_POINTER_INIT, |e, a| {
            assert_eq!(e.mem.u32(a[0] - 0x14), VTABLE_HKP_ACTION);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d2_c7e0, &args![this, 0x77u32]).u32(), this);
        let calls: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == STRING_POINTER_INIT)
            .collect();
        assert_eq!(calls, vec![(STRING_POINTER_INIT, vec![this + 0x14, 0x77])]);
    }
}
