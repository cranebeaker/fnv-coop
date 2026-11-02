//! `SpeedTree/leaflod.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `std::basic_string<char>::basic_string(const char*)` (`thiscall`).
const STRING_FROM_C_STRING: u32 = 0x0044_a4c0;
/// `std::runtime_error::runtime_error(const std::string&)` (`thiscall`).
const RUNTIME_ERROR_FROM_STRING: u32 = 0x0044_a310;
/// `_CxxThrowException(object, throw_info)`.
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// `"vector<bool> too long"`.
const MESSAGE_TOO_LONG: u32 = 0x010a_5950;
/// The `std::length_error` vtable, which the code stores over the
/// `runtime_error` it just built.
const LENGTH_ERROR_VTABLE: u32 = 0x0101_73dc;
/// The throw information of the `length_error`.
const LENGTH_ERROR_THROW_INFO: u32 = 0x0114_0cb0;

// Translated from 00b429d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b429d0 - exception unwinding frame (FS:[0], /GS cookie) is not translated; the throw is.
/// `std::vector<bool>::_Xlen` (the engine map calls the body
/// `std::vector<bool,std::allocator<bool> >::_Trim`, but nothing is trimmed:
/// the body only throws): builds a `runtime_error` from `"vector<bool> too
/// long"`, gives it `std::length_error`'s vtable and throws it. Does not
/// return. `this` is not read.
pub fn fn_00b429d0(e: &mut Engine, _this: Ptr) {
    e.with_stack(0x1c, |e, text| {
        e.call(STRING_FROM_C_STRING, &args![text, MESSAGE_TOO_LONG]);
        e.with_stack(0x28, |e, exception| {
            e.call(RUNTIME_ERROR_FROM_STRING, &args![exception, text]);
            e.mem.set_u32(exception.addr(), LENGTH_ERROR_VTABLE);
            e.call(
                CXX_THROW_EXCEPTION,
                &args![exception, LENGTH_ERROR_THROW_INFO],
            );
        });
    });
    unreachable!("__CxxThrowException does not return (00ec7894)")
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b429d0, fn_00b429d0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn throws_a_length_error_for_vector_bool() {
        let mut e = Engine::new();
        let log = Rc::new(RefCell::new(Vec::new()));
        let record = log.clone();
        e.register_double(STRING_FROM_C_STRING, move |_, a| {
            record.borrow_mut().push((STRING_FROM_C_STRING, a.to_vec()));
            Ret::default()
        });
        let record = log.clone();
        e.register_double(RUNTIME_ERROR_FROM_STRING, move |_, a| {
            record
                .borrow_mut()
                .push((RUNTIME_ERROR_FROM_STRING, a.to_vec()));
            Ret::default()
        });
        let record = log.clone();
        e.register_double(CXX_THROW_EXCEPTION, move |e, a| {
            // The vtable was replaced before the throw.
            assert_eq!(e.mem.u32(a[0]), LENGTH_ERROR_VTABLE);
            record.borrow_mut().push((CXX_THROW_EXCEPTION, a.to_vec()));
            panic!("thrown")
        });
        let thrown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x00b4_29d0, &args![0u32]);
        }));
        assert!(thrown.is_err());
        let log = log.borrow();
        assert_eq!(log.len(), 3);
        assert_eq!(log[0].1[1], MESSAGE_TOO_LONG);
        // The exception is built from the string, then thrown with the
        // length_error throw information.
        assert_eq!(log[1].1[1], log[0].1[0]);
        assert_eq!(log[2].1, vec![log[1].1[0], LENGTH_ERROR_THROW_INFO]);
    }
}
