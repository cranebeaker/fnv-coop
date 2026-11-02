//! `Havok SDK/run_00dba610` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dba610 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: cdecl, no stack words. A function-local static: when bit 0 of the guard
/// word at `01269b80` is clear, sets it, stores the 16-bit word 1 at `01269b7e` and the vtable
/// word `010d6260` at `01269b78` and registers the destructor `00fde2a0` with `_atexit`
/// (`00ec658f`). Returns the address of the static, `01269b78`. Ghidra made no function here; the address is stored in the data word at 011b794c.
pub fn fn_00dba610(e: &mut Engine) -> u32 {
    let guard: u32 = e.global(0x0126_9b80);
    if guard & 1 == 0 {
        e.set_global(0x0126_9b80, guard | 1);
        e.mem.set_u16(0x0126_9b7e, 1);
        e.mem.set_u32(0x0126_9b78, 0x010d_6260);
        e.call(0x00ec_658f, &args![0x00fd_e2a0u32]);
    }
    0x0126_9b78
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00db_a610, fn_00dba610() -> u32)]
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00dba610_initialises_the_static_once() {
        let mut e = Engine::new();
        e.map(0x0126_9000, 0x1000);
        let registered = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&registered);
        e.register_double(0x00ec_658f, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        assert_eq!(e.call(0x00db_a610, &args![]).u32(), 0x0126_9b78);
        assert_eq!(e.mem.u32(0x0126_9b78), 0x010d_6260);
        assert_eq!(e.mem.u16(0x0126_9b7e), 1);
        assert_eq!(e.global::<u32>(0x0126_9b80) & 1, 1);
        assert_eq!(*registered.borrow(), vec![vec![0x00fd_e2a0]]);
        // The second call finds the guard set and registers nothing more.
        e.mem.set_u32(0x0126_9b78, 0);
        assert_eq!(e.call(0x00db_a610, &args![]).u32(), 0x0126_9b78);
        assert_eq!(e.mem.u32(0x0126_9b78), 0);
        assert_eq!(registered.borrow().len(), 1);
    }
}
