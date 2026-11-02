//! `fallout shared/run_004fd400` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004fd400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): the word at +0xa0.
pub fn fn_004fd400(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xa0)
}

// Translated from 004fd420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` and one stack word, `RET 4`): whether the word at the
/// address `004ea950` gives for `this` has any of the bits of `mask` (`SETNZ AL`; the rest
/// of `EAX` keeps the masked word's upper bits, which callers do not read).
pub fn fn_004fd420(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    let flags = e.call(0x004e_a950, &args![this]).u32();
    e.mem.u32(flags) & mask != 0
}

// Translated from 004fd4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): the word at +0x60 masked with
/// `0x1f0000`.
pub fn fn_004fd4f0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x60) & 0x001f_0000
}

// Translated from 004fd440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Handles `reference` (`RET 4`; nothing for null): for the kinds 0x40000 and 0x100000
/// (`004fd4f0`) and for references without a non-empty list (`009d9f00`, `0044ddc0`) it goes
/// to `0050fbf0`; otherwise the value `0050fd90` gives (with 0) for that list's
/// `0044ddc0` goes to `0045a5e0` on the object at `011c3b3c`.
pub fn fn_004fd440(e: &mut Engine, this: Ptr, reference: u32) {
    if reference == 0 {
        return;
    }
    let kind = fn_004fd4f0(e, this);
    if kind == 0x40000 || kind == 0x0010_0000 {
        e.call(0x0050_fbf0, &args![this, reference]);
        return;
    }
    if reference != 0 && e.call(0x009d_9f00, &args![reference]).u32() != 0 {
        let list = e.call(0x009d_9f00, &args![reference]).u32();
        if e.call(0x0044_ddc0, &args![list]).u32() != 0 {
            let list = e.call(0x009d_9f00, &args![reference]).u32();
            let first = e.call(0x0044_ddc0, &args![list]).u32();
            let value = e.call(0x0050_fd90, &args![first, 0u32]).u32();
            let owner = e.global::<u32>(0x011c_3b3c);
            e.call(0x0045_a5e0, &args![owner, value]);
            return;
        }
    }
    e.call(0x0050_fbf0, &args![this, reference]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_d400, fn_004fd400(Ptr) -> u32),
        entry!(0x004f_d420, fn_004fd420(Ptr, u32) -> bool),
        entry!(0x004f_d4f0, fn_004fd4f0(Ptr) -> u32),
        entry!(0x004f_d440, fn_004fd440(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_004fd400_and_fn_004fd4f0_read_their_words() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0xa4);
        e.mem.set_u32(object + 0xa0, 0x5555_6666);
        e.mem.set_u32(object + 0x60, 0xffff_ffff);
        assert_eq!(e.call(0x004f_d400, &args![object]).u32(), 0x5555_6666);
        assert_eq!(e.call(0x004f_d4f0, &args![object]).u32(), 0x001f_0000);
        e.mem.set_u32(object + 0x60, 0x0012_3456);
        assert_eq!(e.call(0x004f_d4f0, &args![object]).u32(), 0x0012_0000);
    }

    #[test]
    fn fn_004fd420_tests_the_mask_against_the_flags_word() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        let flags = e.mem.alloc(4);
        e.mem.set_u32(flags, 0x0000_0102);
        e.register_double(0x004e_a950, move |_, _| Ret {
            eax: flags,
            ..Ret::default()
        });
        for (mask, expected) in [
            (2u32, true),
            (1, false),
            (0x100, true),
            (0xffff_0000, false),
        ] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x004f_d420, &args![object, mask]).bool(), expected);
            let log = e.call_log.take().unwrap();
            assert_eq!(log[1], (0x004e_a950, vec![object]));
        }
    }
}

#[cfg(test)]
mod n039_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_004fd440_routes_by_kind_and_list() {
        let mut e = Engine::new();
        for address in [0x0050_fbf0, 0x0045_a5e0] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x009d_9f00, |_, a| (a[0] + 1).into_ret());
        e.register(0x0044_ddc0, |_, a| (a[0] + 1).into_ret());
        e.register(0x0050_fd90, |_, a| (a[0] + 1).into_ret());
        e.map(0x011c_3b3c, 4);
        e.set_global::<u32>(0x011c_3b3c, 0x6000);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x60, 0x0004_0000);
        e.call_log = Some(vec![]);
        e.call(0x004f_d440, &args![this, 0x100u32]);
        assert_eq!(calls_to(&e, 0x0050_fbf0), vec![vec![this, 0x100]]);
        e.mem.set_u32(this + 0x60, 0x0001_0000);
        e.call_log = Some(vec![]);
        e.call(0x004f_d440, &args![this, 0x100u32]);
        assert_eq!(calls_to(&e, 0x0050_fd90), vec![vec![0x102, 0]]);
        assert_eq!(calls_to(&e, 0x0045_a5e0), vec![vec![0x6000, 0x103]]);
        assert!(calls_to(&e, 0x0050_fbf0).is_empty());
        e.register(0x0044_ddc0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004f_d440, &args![this, 0x100u32]);
        assert_eq!(calls_to(&e, 0x0050_fbf0), vec![vec![this, 0x100]]);
        e.call_log = Some(vec![]);
        e.call(0x004f_d440, &args![this, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
