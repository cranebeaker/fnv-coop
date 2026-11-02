//! `Havok SDK/run_00dd95b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router getter (`00c85750`): its word at `+0x10` is the container allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `00cb1fb0` (`thiscall` on the member at `this + 8`, `RET 4`: one byte as a word) and
/// `00cb1870` (`thiscall` on the same member, `RET 0xc`: the allocator, a byte as a word, a
/// pointer). Both appear to add a byte to a growing array; the engine map has no names for
/// them.
const ADD_BYTE: u32 = 0x00cb_1fb0;
const ADD_BYTE_WITH_ALLOCATOR: u32 = 0x00cb_1870;

/// Asks `source` (virtual function at `+8`, arguments: a pointer to receive a pointer, a pointer
/// to receive a count; both start as 0) for its bytes and answers the block holding the two
/// words (`+0` the count, `+4` the pointer).
fn ask_for_bytes(e: &mut Engine, source: u32) -> u32 {
    let block = e.mem.alloc(8);
    e.vcall(source, 8, &args![block + 4, block]);
    block
}

// Translated from 00dd95b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `source`). Asks `source` for a pointer
/// and a count of bytes (virtual function `+8`, with a pointer for the pointer and one for the
/// count, both starting as 0) and, when the count is above 0, adds every byte one after the
/// other (`00cb1fb0` on `this + 8`).
pub fn fn_00dd95b0(e: &mut Engine, this: u32, source: u32) {
    let block = ask_for_bytes(e, source);
    if e.mem.i32(block) > 0 {
        let mut index = 0u32;
        loop {
            let data = e.mem.u32(block + 4);
            let byte = e.mem.u8(data.wrapping_add(index));
            e.call(ADD_BYTE, &args![this + 8, u32::from(byte)]);
            index += 1;
            if index as i32 >= e.mem.i32(block) {
                break;
            }
        }
    }
    e.mem.free(block);
}

// Translated from 00dd9600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `source`). As `00dd95b0`, but each
/// byte is added with `00cb1870` on `this + 8`, passing the memory router's allocator (its word at
/// `+0x10`), the byte and `source`.
pub fn fn_00dd9600(e: &mut Engine, this: u32, source: u32) {
    let block = ask_for_bytes(e, source);
    if e.mem.i32(block) > 0 {
        let mut index = 0u32;
        loop {
            let data = e.mem.u32(block + 4);
            let byte = e.mem.u8(data.wrapping_add(index));
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.call(
                ADD_BYTE_WITH_ALLOCATOR,
                &args![this + 8, allocator, u32::from(byte), source],
            );
            index += 1;
            if index as i32 >= e.mem.i32(block) {
                break;
            }
        }
    }
    e.mem.free(block);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd95b0, fn_00dd95b0(u32, u32)),
        entry!(0x00dd9600, fn_00dd9600(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const SOURCE_DOUBLE: u32 = 0x007a_0100;

    /// A source object whose virtual function at `+8` hands out `bytes` (or nothing when `None`),
    /// and a log of the argument words the byte functions see.
    fn setup(e: &mut Engine, bytes: Option<&[u8]>) -> (u32, Rc<RefCell<Vec<Vec<u32>>>>) {
        e.put_vtable(0x007a_0000, &[0, 0, SOURCE_DOUBLE]);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 0x007a_0000);
        let data = e.mem.alloc(16);
        if let Some(b) = bytes {
            for (i, v) in b.iter().enumerate() {
                e.mem.set_u8(data + i as u32, *v);
            }
        }
        let count = bytes.map_or(0, |b| b.len() as u32);
        e.register_double(SOURCE_DOUBLE, move |e, a| {
            // The pointer and the count start as 0.
            assert_eq!((e.mem.u32(a[1]), e.mem.u32(a[2])), (0, 0));
            if count > 0 {
                e.mem.set_u32(a[1], data);
                e.mem.set_u32(a[2], count);
            }
            Ret::default()
        });
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0xa110);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        for address in [ADD_BYTE, ADD_BYTE_WITH_ALLOCATOR] {
            let seen = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                seen.borrow_mut().push(a.to_vec());
                Ret::default()
            });
        }
        (source, log)
    }

    #[test]
    fn every_byte_is_added_in_order() {
        let mut e = Engine::new();
        let (source, log) = setup(&mut e, Some(b"abc"));
        e.call(0x00dd_95b0, &args![0x1000u32, source]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![0x1008, u32::from(b'a')],
                vec![0x1008, u32::from(b'b')],
                vec![0x1008, u32::from(b'c')]
            ]
        );
    }

    #[test]
    fn the_allocator_form_passes_the_allocator_and_the_source() {
        let mut e = Engine::new();
        let (source, log) = setup(&mut e, Some(&[0x80, 7]));
        e.call(0x00dd_9600, &args![0x1000u32, source]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![0x1008, 0xa110, 0x80, source],
                vec![0x1008, 0xa110, 7, source]
            ]
        );
    }

    #[test]
    fn no_bytes_means_no_calls() {
        let mut e = Engine::new();
        let (source, log) = setup(&mut e, None);
        e.call(0x00dd_95b0, &args![0x1000u32, source]);
        e.call(0x00dd_9600, &args![0x1000u32, source]);
        assert!(log.borrow().is_empty());
    }
}
