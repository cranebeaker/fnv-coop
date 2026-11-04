//! `fallout shared/run_0040f6e0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedIncrement` wrapper (`0040b460`, `cdecl`): one argument, the
/// address of the counter.
const INTERLOCKED_INCREMENT: u32 = 0x0040_b460;

// Translated from 0040f6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a reference to a counted object (`NiRefObject`): increments the
/// count at `+4` atomically (`0040b460`). The wrapper's result stays in
/// `EAX` but the decompiler shows none and callers ignore it.
pub fn fn_0040f6e0(e: &mut Engine, this: Ptr) {
    e.call(INTERLOCKED_INCREMENT, &args![this.addr() + 4]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0040f6e0, fn_0040f6e0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_the_count_at_4() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            Ret {
                eax: count,
                ..Ret::default()
            }
        });
        let object: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(object.addr() + 4, 1);
        e.call(0x0040_f6e0, &args![object]);
        assert_eq!(e.mem.u32(object.addr() + 4), 2);
        assert_eq!(e.mem.u32(object.addr()), 0);
    }
}
