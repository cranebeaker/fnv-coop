//! `NiAnimation/run_00a48990` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiKeyBasedInterpolator::LoadBinary` (`00aa0ac0`), `NiColorA::LoadBinary` (`00a69800`) and
/// `NiStream::ResolveLinkID` (`00a64620`), engine map; all `thiscall`, the first two on the
/// object with the stream, the last on the stream.
const KEY_BASED_INTERPOLATOR_LOAD: u32 = 0x00aa_0ac0;
const COLOR_LOAD: u32 = 0x00a6_9800;
const RESOLVE_LINK_ID: u32 = 0x00a6_4620;
/// `InterlockedIncrement` and `InterlockedDecrement` (KERNEL32.DLL import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00a48990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the stream, `RET 4`; a `LoadBinary` by its body):
/// the key-based interpolator's part (`00aa0ac0`), the colour at +0xc (`NiColorA::
/// LoadBinary`), then the object the stream's next link names (`NiStream::ResolveLinkID`)
/// into the smart pointer at +0x1c (unless it holds it already: the old one's count (+4)
/// down, vtable +4 at zero; the new one, when not null, counted up).
pub fn fn_00a48990(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(KEY_BASED_INTERPOLATOR_LOAD, &args![this, stream]);
    e.call(COLOR_LOAD, &args![this.addr() + 0xc, stream]);
    let object = e.call(RESOLVE_LINK_ID, &args![stream]).u32();
    let old = e.mem.u32(this.addr() + 0x1c);
    if old == object {
        return;
    }
    if old != 0 && e.call(INTERLOCKED_DECREMENT, &args![old + 4]).u32() == 0 {
        e.vcall(old, 4, &[]);
    }
    e.mem.set_u32(this.addr() + 0x1c, object);
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a4_8990, fn_00a48990(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_reads_the_parts_and_links_the_object() {
        let mut e = Engine::new();
        e.register(KEY_BASED_INTERPOLATOR_LOAD, |_, _| Ret::default());
        e.register(COLOR_LOAD, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.i32(a[0]).wrapping_add(1);
            e.mem.set_i32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.i32(a[0]).wrapping_sub(1);
            e.mem.set_i32(a[0], count);
            count.into_ret()
        });
        e.register(0x034a_0000, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x034a_0000);
        let (old, new) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(old, vtable);
        e.mem.set_i32(old + 4, 1);
        e.register_double(RESOLVE_LINK_ID, move |_, _| new.into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x1c, old);
        e.call(0x00a4_8990, &args![this, 0x9000u32]);
        assert_eq!(e.mem.u32(this + 0x1c), new);
        assert_eq!(e.mem.i32(new + 4), 1);
        let log = e.call_log.take().unwrap();
        let calls: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .filter(|c| {
                [
                    KEY_BASED_INTERPOLATOR_LOAD,
                    COLOR_LOAD,
                    RESOLVE_LINK_ID,
                    0x034a_0000,
                ]
                .contains(&c.0)
            })
            .collect();
        assert_eq!(
            calls,
            vec![
                (KEY_BASED_INTERPOLATOR_LOAD, vec![this, 0x9000]),
                (COLOR_LOAD, vec![this + 0xc, 0x9000]),
                (RESOLVE_LINK_ID, vec![0x9000]),
                (0x034a_0000, vec![old]),
            ]
        );
    }
}
