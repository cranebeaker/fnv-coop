//! `fallout shared/run_004ff7e0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable the constructor installs (Ghidra labels `0102583c` `SaveGame`).
const SAVE_GAME_VTABLE: u32 = 0x0102_583c;

// Translated from 004ff7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__fastcall`: stores vtable `0102583c` at `this + 0` and
/// returns `this` (EAX still holds it).
pub fn fn_004ff7e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), SAVE_GAME_VTABLE);
    this
}

/// `00428110` (`thiscall`, `RET 4`): the change flags of the save buffer, through the word
/// it is given.
const CHANGE_FLAGS: u32 = 0x0042_8110;
/// `004280f0` (`thiscall`, `RET 4`): whether the flags have the bits of the mask.
const HAS_FLAGS: u32 = 0x0042_80f0;
/// The change flag the word at +0x90 is saved under.
const FLAG_FORM: u32 = 0x0080_0000;

// Translated from 004ff800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves into the save buffer `buffer` (`RET 4`): the base save `00511860`, then, when the
/// change flags (`00428110` into a local word) have 0x800000 (`004280f0`), the form at +0x90
/// (`BGSSaveGameBuffer::SaveFormID_ov2` with 0).
pub fn fn_004ff800(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0051_1860, &args![this, buffer]);
    let flagged = e.with_stack(4, |e, local| {
        let flags = e.call(CHANGE_FLAGS, &args![buffer, local]).u32();
        e.call(HAS_FLAGS, &args![flags, FLAG_FORM]).bool()
    });
    if flagged {
        let form = e.mem.u32(this.addr() + 0x90);
        e.call(0x0086_5df0, &args![buffer, form, 0u32]);
    }
}

// Translated from 004ff850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads from the load buffer `buffer` (`RET 4`): the base load `005118c0`, then, when the
/// change flags (`00428110` into a local word) have 0x800000 (`004280f0`), the form id into
/// +0x90 (`BGSLoadGameBuffer::LoadFormID_ov2`).
pub fn fn_004ff850(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0051_18c0, &args![this, buffer]);
    let flagged = e.with_stack(4, |e, local| {
        let flags = e.call(CHANGE_FLAGS, &args![buffer, local]).u32();
        e.call(HAS_FLAGS, &args![flags, FLAG_FORM]).bool()
    });
    if flagged {
        e.call(0x0086_48e0, &args![buffer, this.addr() + 0x90]);
    }
}

// Translated from 004ff8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves after a load (`RET 4`): the base step `004534f0`, then, when the change flags
/// (`00428110`) have 0x800000 (`004280f0`), the form id at +0x90 becomes its form
/// (`004839c0`) cast (`__RTDynamicCast` from `01183028` to `011846d4`), or stays null.
pub fn fn_004ff8a0(e: &mut Engine, this: Ptr, buffer: u32) {
    let base = this.addr();
    e.call(0x0045_34f0, &args![this, buffer]);
    let flagged = e.with_stack(4, |e, local| {
        let flags = e.call(CHANGE_FLAGS, &args![buffer, local]).u32();
        e.call(HAS_FLAGS, &args![flags, FLAG_FORM]).bool()
    });
    if flagged {
        let resolved = if e.mem.u32(base + 0x90) == 0 {
            0
        } else {
            let id = e.mem.u32(base + 0x90);
            let form = e.call(0x0048_39c0, &args![id]).u32();
            e.call(
                0x00ec_43fb,
                &args![form, 0u32, 0x0118_3028u32, 0x0118_46d4u32, 0u32],
            )
            .u32()
        };
        e.mem.set_u32(base + 0x90, resolved);
    }
}

// Translated from 004ff930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reverts (`RET 4`): the base step `004534f0`, then, when the flags `0042ce30` gives (into a
/// local word) have 0x800000 (`004280f0`), clears +0x90.
pub fn fn_004ff930(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0045_34f0, &args![this, buffer]);
    let flagged = e.with_stack(4, |e, local| {
        let flags = e.call(0x0042_ce30, &args![buffer, local]).u32();
        e.call(HAS_FLAGS, &args![flags, FLAG_FORM]).bool()
    });
    if flagged {
        e.mem.set_u32(this.addr() + 0x90, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004ff7e0, fn_004ff7e0(Ptr) -> Ptr),
        entry!(0x004f_f800, fn_004ff800(Ptr, u32)),
        entry!(0x004f_f850, fn_004ff850(Ptr, u32)),
        entry!(0x004f_f8a0, fn_004ff8a0(Ptr, u32)),
        entry!(0x004f_f930, fn_004ff930(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installs_the_vtable_and_returns_this() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(8));
        assert_eq!(e.call(0x004f_f7e0, &args![this]).ptr::<()>(), this);
        assert_eq!(e.mem.u32(this.addr()), SAVE_GAME_VTABLE);
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

    /// Change-flag doubles: the flags are the local word's address, `004280f0` answers
    /// `flagged`.
    fn flags(e: &mut Engine, flagged: bool) {
        e.register(CHANGE_FLAGS, |_, a| a[1].into_ret());
        e.register(0x0042_ce30, |_, a| a[1].into_ret());
        e.register_double(HAS_FLAGS, move |_, a| {
            assert_eq!(a[1], FLAG_FORM);
            flagged.into_ret()
        });
    }

    #[test]
    fn fn_004ff800_saves_the_form_when_flagged() {
        let mut e = Engine::new();
        e.register(0x0051_1860, |_, _| Ret::default());
        e.register(0x0086_5df0, |_, _| Ret::default());
        flags(&mut e, true);
        let this = e.mem.alloc(0x94);
        e.mem.set_u32(this + 0x90, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x004f_f800, &args![this, 0x5000u32]);
        assert_eq!(calls_to(&e, 0x0051_1860), vec![vec![this, 0x5000]]);
        assert_eq!(calls_to(&e, 0x0086_5df0), vec![vec![0x5000, 0x77, 0]]);
        flags(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x004f_f800, &args![this, 0x5000u32]);
        assert!(calls_to(&e, 0x0086_5df0).is_empty());
    }

    #[test]
    fn fn_004ff850_loads_the_form_id_when_flagged() {
        let mut e = Engine::new();
        e.register(0x0051_18c0, |_, _| Ret::default());
        e.register(0x0086_48e0, |_, _| Ret::default());
        flags(&mut e, true);
        let this = e.mem.alloc(0x94);
        e.call_log = Some(vec![]);
        e.call(0x004f_f850, &args![this, 0x5000u32]);
        assert_eq!(calls_to(&e, 0x0051_18c0), vec![vec![this, 0x5000]]);
        assert_eq!(calls_to(&e, 0x0086_48e0), vec![vec![0x5000, this + 0x90]]);
        flags(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x004f_f850, &args![this, 0x5000u32]);
        assert!(calls_to(&e, 0x0086_48e0).is_empty());
    }

    #[test]
    fn fn_004ff8a0_resolves_the_form() {
        let mut e = Engine::new();
        e.register(0x0045_34f0, |_, _| Ret::default());
        e.register(0x0048_39c0, |_, a| (a[0] + 0x1000).into_ret());
        e.register(0x00ec_43fb, |_, a| (a[0] + 1).into_ret());
        flags(&mut e, true);
        let this = e.mem.alloc(0x94);
        e.mem.set_u32(this + 0x90, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x004f_f8a0, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(this + 0x90), 0x1078);
        assert_eq!(
            calls_to(&e, 0x00ec_43fb),
            vec![vec![0x1077, 0, 0x0118_3028, 0x0118_46d4, 0]]
        );
        e.mem.set_u32(this + 0x90, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_f8a0, &args![this, 0x5000u32]);
        assert!(calls_to(&e, 0x0048_39c0).is_empty());
        assert_eq!(e.mem.u32(this + 0x90), 0);
        flags(&mut e, false);
        e.mem.set_u32(this + 0x90, 0x77);
        e.call(0x004f_f8a0, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(this + 0x90), 0x77);
    }

    #[test]
    fn fn_004ff930_clears_the_form_when_flagged() {
        let mut e = Engine::new();
        e.register(0x0045_34f0, |_, _| Ret::default());
        flags(&mut e, false);
        let this = e.mem.alloc(0x94);
        e.mem.set_u32(this + 0x90, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x004f_f930, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(this + 0x90), 0x77);
        assert_eq!(calls_to(&e, 0x0045_34f0), vec![vec![this, 0x5000]]);
        assert_eq!(calls_to(&e, 0x0042_ce30).len(), 1);
        flags(&mut e, true);
        e.call(0x004f_f930, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(this + 0x90), 0);
    }
}
