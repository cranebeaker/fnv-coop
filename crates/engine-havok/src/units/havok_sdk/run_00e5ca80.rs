//! `Havok SDK/run_00e5ca80` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{load_vec, store_vec};

/// `00e5c650`: thiscall, `(name)`, `RET 4`; the attribute object or 0.
const FIND_ATTRIBUTE: u32 = 0x00e5_c650;
/// `hkOstream::hkOstream` (`00cb1600`, thiscall on the stream, `(buffer, size)`, `RET 8`).
const OSTREAM_CONSTRUCTOR: u32 = 0x00cb_1600;
/// The stream's `operator<<` for text (`00cc01c0`, thiscall, one word, returns the stream), the
/// overload for an object pointer (`00cc05a0`, likewise) and the stream's destructor
/// (`00cc00e0`, thiscall, `this` only).
const STREAM_TEXT: u32 = 0x00cc_01c0;
const STREAM_POINTER: u32 = 0x00cc_05a0;
const STREAM_DESTRUCTOR: u32 = 0x00cc_00e0;
/// The error handler object pointer (`01268360`); its virtual slot `0xc` takes
/// `(1, 0xabbaab81, message, file, line)`.
const ERROR_HANDLER: u32 = 0x0126_8360;
/// The texts of the message: `"Float attribute "`, `" not found in "`, `" attribute group"` and
/// the source file name (`010ecd48`, `010eccfc`, `010ecd0c`, `010eccc8`).
const TEXT_ATTRIBUTE: u32 = 0x010e_cd48;
const TEXT_NOT_FOUND_IN: u32 = 0x010e_ccfc;
const TEXT_GROUP: u32 = 0x010e_cd0c;
const SOURCE_FILE: u32 = 0x010e_ccc8;

// Translated from 00e5ca80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc`: the vector value of the attribute named `name` in the group `this`. The
/// attribute object comes from `00e5c650`; when found, the 16 bytes at `*(object + 8)` are copied
/// to `out` and 0 is returned. When not found and `report` is nonzero, the message `"Float
/// attribute <name> not found in <this> attribute group"` is written to an `hkOstream` on a
/// 0x200-byte stack buffer (`00cb1600`, then the `<<` calls `00cc01c0` and `00cc05a0`, each on
/// the stream the previous one returned) and handed to the error handler's virtual slot `0xc`
/// with `(1, 0xabbaab81, message, ".\Attributes\hkxAttributeGroup.cpp", 0xa2)`; the stream is
/// destroyed (`00cc00e0`). Returns 1 when not found.
pub fn fn_00e5ca80(e: &mut Engine, this: Ptr, name: u32, report: u8, out: u32) -> u32 {
    let this = this.addr();
    let found = e.call(FIND_ATTRIBUTE, &args![this, name]).u32();
    if found != 0 {
        let value = e.mem.u32(found.wrapping_add(8));
        let vector = load_vec(e, value);
        store_vec(e, out, vector);
        return 0;
    }
    if report != 0 {
        e.with_stack(0x220, |e, frame| {
            let stream = frame.addr().wrapping_add(0x14);
            let buffer = frame.addr().wrapping_add(0x20);
            e.call(OSTREAM_CONSTRUCTOR, &args![stream, buffer, 0x200u32]);
            let next = e.call(STREAM_TEXT, &args![stream, TEXT_ATTRIBUTE]).u32();
            let next = e.call(STREAM_TEXT, &args![next, name]).u32();
            let next = e.call(STREAM_TEXT, &args![next, TEXT_NOT_FOUND_IN]).u32();
            let next = e.call(STREAM_POINTER, &args![next, this]).u32();
            e.call(STREAM_TEXT, &args![next, TEXT_GROUP]);
            let handler = e.mem.u32(ERROR_HANDLER);
            e.vcall(
                handler,
                0xc,
                &args![1u32, 0xabba_ab81u32, buffer, SOURCE_FILE, 0xa2u32],
            );
            e.call(STREAM_DESTRUCTOR, &args![stream]);
        });
    }
    1
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e5_ca80, fn_00e5ca80(Ptr, u32, u8, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> (Engine, Calls) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in [OSTREAM_CONSTRUCTOR, STREAM_DESTRUCTOR] {
            let log = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        // The `<<` calls answer with the stream they were given.
        for address in [STREAM_TEXT, STREAM_POINTER] {
            let log = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax: a[0] + 1,
                    ..Ret::default()
                }
            });
        }
        let log = Rc::clone(&calls);
        e.register_double(0x0ff0_000c, move |_, a| {
            log.borrow_mut().push((0xc, a.to_vec()));
            Ret::default()
        });
        e.put_vtable(0x0126_0300, &[0, 0, 0, 0x0ff0_000c]);
        e.mem.set_u32(0x0126_0400, 0x0126_0300);
        e.mem.set_u32(ERROR_HANDLER, 0x0126_0400);
        (e, calls)
    }

    #[test]
    fn found_attribute_copies_its_vector() {
        let (mut e, calls) = engine();
        let object = e.mem.alloc(0x20);
        let value = e.mem.alloc(16);
        store_vec(&mut e, value, [1.0, 2.0, f32::from_bits(0x7f80_0001), 4.0]);
        e.mem.set_u32(object + 8, value);
        e.register_double(FIND_ATTRIBUTE, move |_, _| Ret {
            eax: object,
            ..Ret::default()
        });
        let out = e.mem.alloc(16);
        assert_eq!(
            e.call(0x00e5_ca80, &args![0x1000u32, 0x2000u32, 1u8, out])
                .u32(),
            0
        );
        assert_eq!(
            load_vec(&e, out).map(f32::to_bits),
            [
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                0x7f80_0001,
                4.0f32.to_bits()
            ]
        );
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn missing_attribute_without_report_only_returns_one() {
        let (mut e, calls) = engine();
        e.register(FIND_ATTRIBUTE, |_, _| Ret::default());
        let out = e.mem.alloc(16);
        assert_eq!(
            e.call(0x00e5_ca80, &args![0x1000u32, 0x2000u32, 0u8, out])
                .u32(),
            1
        );
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn missing_attribute_with_report_builds_and_reports_the_message() {
        let (mut e, calls) = engine();
        e.register(FIND_ATTRIBUTE, |_, _| Ret::default());
        let out = e.mem.alloc(16);
        assert_eq!(
            e.call(0x00e5_ca80, &args![0x1000u32, 0x2000u32, 1u8, out])
                .u32(),
            1
        );
        let calls = calls.borrow();
        let order: Vec<u32> = calls.iter().map(|call| call.0).collect();
        assert_eq!(
            order,
            vec![
                OSTREAM_CONSTRUCTOR,
                STREAM_TEXT,
                STREAM_TEXT,
                STREAM_TEXT,
                STREAM_POINTER,
                STREAM_TEXT,
                0xc,
                STREAM_DESTRUCTOR
            ]
        );
        let stream = calls[0].1[0];
        let buffer = calls[0].1[1];
        assert_eq!(calls[0].1[2], 0x200);
        // Each `<<` is applied to the stream the previous call returned.
        assert_eq!(calls[1].1, vec![stream, TEXT_ATTRIBUTE]);
        assert_eq!(calls[2].1, vec![stream + 1, 0x2000]);
        assert_eq!(calls[3].1, vec![stream + 2, TEXT_NOT_FOUND_IN]);
        assert_eq!(calls[4].1, vec![stream + 3, 0x1000]);
        assert_eq!(calls[5].1, vec![stream + 4, TEXT_GROUP]);
        assert_eq!(
            calls[6].1,
            vec![0x0126_0400, 1, 0xabba_ab81, buffer, SOURCE_FILE, 0xa2]
        );
        assert_eq!(calls[7].1, vec![stream]);
    }
}
