//! `fallout/misc/run_008693c0` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The array's 16-bit element count (+0x0a, as both functions read it).
const SIZE: u32 = 0x0a;
/// The set-at-grow of one array instance (`00869640`, thiscall: index,
/// element; returns the index).
const SET_AT_GROW_00869640: u32 = 0x0086_9640;
/// The set-at-grow of another array instance (`00869680`, the same shape).
const SET_AT_GROW_00869680: u32 = 0x0086_9680;

// Translated from 008693c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An array's `Add` (thiscall, `RET 4`; unnamed in the engine map):
/// `00869640(this, size, element)` with the 16-bit count at +0x0a as the
/// index. The callee's EAX (the index) is left as the result.
pub fn fn_008693c0(e: &mut Engine, this: Ptr, element: u32) -> u32 {
    let size = u32::from(e.mem.u16(this.addr() + SIZE));
    e.call(SET_AT_GROW_00869640, &args![this, size, element])
        .u32()
}

// Translated from 008693f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same `Add` for the array instance of `00869680`.
pub fn fn_008693f0(e: &mut Engine, this: Ptr, element: u32) -> u32 {
    let size = u32::from(e.mem.u16(this.addr() + SIZE));
    e.call(SET_AT_GROW_00869680, &args![this, size, element])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0086_93c0, fn_008693c0(Ptr, u32) -> u32),
        entry!(0x0086_93f0, fn_008693f0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_add(address: u32, callee: u32) {
        let mut e = Engine::new();
        e.register(callee, |_, a| Ret {
            eax: a[1] + 0x100,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x10);
        // The count is the 16-bit field only.
        e.mem.set_u32(this + 8, 0x0007_0020);
        e.mem.set_u32(this + 0x0c, 0xffff_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(address, &args![this, 0x0abcu32]).u32(), 0x107);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == callee).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![this, 7, 0x0abc]);
    }

    #[test]
    fn fn_008693c0_adds_at_the_current_count() {
        check_add(0x0086_93c0, SET_AT_GROW_00869640);
    }

    #[test]
    fn fn_008693f0_adds_at_the_current_count() {
        check_add(0x0086_93f0, SET_AT_GROW_00869680);
    }
}
