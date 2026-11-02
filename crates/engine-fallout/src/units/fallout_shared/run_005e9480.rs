//! `fallout shared/run_005e9480` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The flag test both setters run on the object's flags: `004280f0`
/// (thiscall, `RET 4`) with the mask 0x80000000, on what the object's
/// getter (given a stack local) answers.
const FLAG_TEST: u32 = 0x0042_80f0;
const FLAG_MASK: u32 = 0x8000_0000;

// Translated from 005e9480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00484da0(object)` on `this` (`RET 4`: the object; the engine map
/// has no name), then sets the byte at +0x7D to 1 when `004280f0(0x80000000)`
/// answers true on what `00428110` (on the object, given a 4-byte stack
/// local) returns.
pub fn fn_005e9480(e: &mut Engine, this: Ptr, object: u32) {
    e.call(0x0048_4da0, &args![this, object]);
    let set = e.with_stack(4, |e, local| {
        let flags = e.call(0x0042_8110, &args![object, local]).u32();
        e.call(FLAG_TEST, &args![flags, FLAG_MASK]).bool()
    });
    if set {
        e.mem.set_u8(this.addr() + 0x7d, 1);
    }
}

// Translated from 005e94d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `004534f0(object)` on `this` (`RET 4`: the object; the engine map
/// has no name), then clears the byte at +0x7D when `004280f0(0x80000000)`
/// answers true on what `0042ce30` (on the object, given a 4-byte stack
/// local) returns.
pub fn fn_005e94d0(e: &mut Engine, this: Ptr, object: u32) {
    e.call(0x0045_34f0, &args![this, object]);
    let set = e.with_stack(4, |e, local| {
        let flags = e.call(0x0042_ce30, &args![object, local]).u32();
        e.call(FLAG_TEST, &args![flags, FLAG_MASK]).bool()
    });
    if set {
        e.mem.set_u8(this.addr() + 0x7d, 0);
    }
}

// Translated from 005e9520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x30 (`SUB ECX,0x30`) and jumps to
/// `005e8140` (a scalar deleting destructor, `RET 4`), passing the flags
/// word on; answers its `EAX`.
pub fn fn_005e9520(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x005e_8140, &args![this.addr().wrapping_sub(0x30), flags])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_9480, fn_005e9480(Ptr, u32)),
        entry!(0x005e_94d0, fn_005e94d0(Ptr, u32)),
        entry!(0x005e_9520, fn_005e9520(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn fn_005e9480_and_005e94d0_set_and_clear_the_byte() {
        let mut e = Engine::new();
        for a in [0x0048_4da0u32, 0x0045_34f0] {
            e.register(a, |_, _| Ret::default());
        }
        e.register(0x0042_8110, |_, a| ret(a[0] + 1));
        e.register(0x0042_ce30, |_, a| ret(a[0] + 2));
        // The test answers true for the first getter only.
        e.register(FLAG_TEST, |_, a| {
            assert_eq!(a[1], FLAG_MASK);
            ret(u32::from(a[0] & 3 == 1))
        });
        let this = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        e.call(0x005e_9480, &args![this, 0x1000u32]);
        assert_eq!(e.mem.u8(this + 0x7d), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0048_4da0, vec![this, 0x1000]));
        assert_eq!(log[2].0, 0x0042_8110);
        assert_eq!(log[2].1[0], 0x1000);
        assert_eq!(log[3], (FLAG_TEST, vec![0x1001, FLAG_MASK]));
        // The second getter answers a value the test rejects: kept.
        e.call(0x005e_94d0, &args![this, 0x1000u32]);
        assert_eq!(e.mem.u8(this + 0x7d), 1);
        e.register(FLAG_TEST, |_, _| ret(1));
        e.call(0x005e_94d0, &args![this, 0x1000u32]);
        assert_eq!(e.mem.u8(this + 0x7d), 0);
        e.register(FLAG_TEST, |_, _| ret(0));
        e.call(0x005e_9480, &args![this, 0x1000u32]);
        assert_eq!(e.mem.u8(this + 0x7d), 0);
    }

    #[test]
    fn fn_005e9520_moves_this_back_by_0x30() {
        let mut e = Engine::new();
        e.register(0x005e_8140, |_, a| ret(a[0] ^ a[1]));
        assert_eq!(e.call(0x005e_9520, &args![0x1030u32, 1u32]).u32(), 0x1001);
    }
}
