//! `NiAnimation/run_00a57580` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a575b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words: an object and a value word).
/// Calls the function pointer at `object+8` (`cdecl`, five words) with the
/// object, the value word, 4, the address of a stack word holding 4, and 1,
/// and adds its result to the word at `object+4`.
pub fn fn_00a575b0(e: &mut Engine, object: Ptr, value: u32) {
    let object = object.addr();
    let function = e.mem.u32(object + 8);
    let done = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(function, &args![object, value, 4u32, size, 1u32])
            .u32()
    });
    let total = e.mem.u32(object + 4).wrapping_add(done);
    e.mem.set_u32(object + 4, total);
}

// Translated from 00a57580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words: an object and a destination). Calls the
/// function pointer at `object+8` (`cdecl`, five words) with the object, the destination, 1,
/// the address of a stack word holding 1, and 1, and adds its result to the word at
/// `object+4`: one byte read into the destination, on the stream layout `fn_00a575b0`
/// shares.
pub fn fn_00a57580(e: &mut Engine, object: Ptr, dest: u32) {
    let object = object.addr();
    let function = e.mem.u32(object + 8);
    let done = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 1);
        e.call(function, &args![object, dest, 1u32, size, 1u32])
            .u32()
    });
    let total = e.mem.u32(object + 4).wrapping_add(done);
    e.mem.set_u32(object + 4, total);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5_75b0, fn_00a575b0(Ptr, u32)),
        entry!(0x00a5_7580, fn_00a57580(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a575b0_calls_the_object_function_and_adds_its_result() {
        const FUNCTION: u32 = 0x0123_4560;
        let mut e = Engine::new();
        e.register(FUNCTION, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            7u32.into_ret()
        });
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 8, FUNCTION);
        e.mem.set_u32(object + 4, 10);
        e.call_log = Some(vec![]);
        e.call(0x00a5_75b0, &args![object, 0x5555u32]);
        assert_eq!(e.mem.u32(object + 4), 17);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == FUNCTION).collect();
        assert_eq!(calls.len(), 1);
        let w = &calls[0].1;
        assert_eq!((w[0], w[1], w[2], w[4]), (object, 0x5555, 4, 1));
    }
}

#[cfg(test)]
mod n007_tests {
    use super::*;

    #[test]
    fn reads_one_byte_and_advances() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0xc);
        e.mem.set_u32(object + 4, 10);
        e.mem.set_u32(object + 8, 0x034b_0000);
        e.register(0x034b_0000, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (1, 1, 1));
            e.mem.set_u8(a[1], 0x5a);
            1u32.into_ret()
        });
        let dest = e.mem.alloc(4);
        e.call(0x00a5_7580, &args![object, dest]);
        assert_eq!(e.mem.u8(dest), 0x5a);
        assert_eq!(e.mem.u32(object + 4), 11);
    }
}
