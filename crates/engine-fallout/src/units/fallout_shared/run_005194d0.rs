//! `fallout shared/run_005194d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00519550` calls on the member (`00470470`).
const CALLEE_00470470: u32 = 0x0047_0470;

// Translated from 00519550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00470470` with `ECX` = `this + 0x88` and passes on nothing (the callee returns no value).
pub fn fn_00519550(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_00470470, &args![this.byte_add(0x88)]);
}

/// `008256d0` (`thiscall` on the member at +0x88, a byte answer) and
/// `005f65d0` (`thiscall` on it, one word: the address of a form pointer).
const MEMBER_008256D0: u32 = 0x0082_56d0;
const MEMBER_005F65D0: u32 = 0x005f_65d0;
/// `00401170` (`thiscall`: the form's type), `__RTDynamicCast`
/// (`00ec43fb`) from `.?AVTESForm@@` (`01183028`) to `.?AVTESObjectCELL@@`
/// (`01183fb4`), and `00425fd0` (`thiscall` on the cell, a byte answer).
const FORM_TYPE: u32 = 0x0040_1170;
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_CELL: u32 = 0x0118_3fb4;
const CELL_00425FD0: u32 = 0x0042_5fd0;

// Translated from 005194d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: whether `008256d0` on the member at +0x88 answers false.
pub fn fn_005194d0(e: &mut Engine, this: Ptr) -> bool {
    !e.call(MEMBER_008256D0, &args![this.byte_add(0x88)]).bool()
}

// Translated from 00519500 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one word, `RET 4`): whether `form` is set,
/// passes `00519570` and `005f65d0` on the member at +0x88, given the
/// address of the stack word holding `form`, answers true.
pub fn fn_00519500(e: &mut Engine, this: Ptr, form: u32) -> bool {
    if form == 0 {
        return false;
    }
    if !fn_00519570(e, form) {
        return false;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), form);
        e.call(MEMBER_005F65D0, &args![this.byte_add(0x88), slot])
            .bool()
    })
}

// Translated from 00519570 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`cdecl`, one word): whether `form` is set and either of
/// type 0x41, or of type 0x39, casts to `TESObjectCELL` and `00425fd0` on
/// it answers true.
pub fn fn_00519570(e: &mut Engine, form: u32) -> bool {
    if form == 0 {
        return false;
    }
    match e.call(FORM_TYPE, &args![form]).u32() {
        0x39 => {
            let cell = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_CELL, 0u32],
                )
                .u32();
            cell != 0 && e.call(CELL_00425FD0, &args![cell]).bool()
        }
        0x41 => true,
        _ => false,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_9550, fn_00519550(Ptr)),
        entry!(0x0051_94d0, fn_005194d0(Ptr) -> bool),
        entry!(0x0051_9500, fn_00519500(Ptr, u32) -> bool),
        entry!(0x0051_9570, fn_00519570(u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00519550_passes_this_plus_88_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00470470, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0051_9550, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00470470).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x88]);
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    #[test]
    fn fn_005194d0_negates_the_member_check() {
        let mut e = Engine::new();
        e.register(MEMBER_008256D0, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x100);
        assert!(e.call(0x0051_94d0, &[this]).bool());
        e.mem.set_u32(this + 0x88, 0x0100);
        assert!(e.call(0x0051_94d0, &[this]).bool());
        e.mem.set_u32(this + 0x88, 1);
        assert!(!e.call(0x0051_94d0, &[this]).bool());
    }

    /// Forms: type at +0x10, cast answers the form when +0xc is zero,
    /// `00425fd0` answers the word at +0x14.
    fn setup(e: &mut Engine) {
        e.register(FORM_TYPE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x10),
            ..Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |e, a| {
            assert_eq!(a[1..], [0, TYPE_TES_FORM, TYPE_TES_OBJECT_CELL, 0]);
            let form = if e.mem.u32(a[0] + 0xc) == 0 { a[0] } else { 0 };
            Ret {
                eax: form,
                ..Ret::default()
            }
        });
        e.register(CELL_00425FD0, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x14),
            ..Ret::default()
        });
    }

    #[test]
    fn fn_00519570_accepts_type_0x41_and_checked_cells() {
        let mut e = Engine::new();
        setup(&mut e);
        let form = e.mem.alloc(0x20);
        assert!(!e.call(0x0051_9570, &[0]).bool());
        e.mem.set_u32(form + 0x10, 0x41);
        assert!(e.call(0x0051_9570, &[form]).bool());
        e.mem.set_u32(form + 0x10, 0x40);
        assert!(!e.call(0x0051_9570, &[form]).bool());
        e.mem.set_u32(form + 0x10, 0x39);
        assert!(!e.call(0x0051_9570, &[form]).bool());
        e.mem.set_u32(form + 0x14, 1);
        assert!(e.call(0x0051_9570, &[form]).bool());
        e.mem.set_u32(form + 0xc, 1);
        assert!(!e.call(0x0051_9570, &[form]).bool());
    }

    #[test]
    fn fn_00519500_checks_the_form_then_the_member() {
        let mut e = Engine::new();
        setup(&mut e);
        e.register(MEMBER_005F65D0, |e, a| {
            let form = e.mem.u32(a[1]);
            Ret {
                eax: e.mem.u32(form + 0x18),
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x100);
        let form = e.mem.alloc(0x20);
        e.mem.set_u32(form + 0x10, 0x41);
        assert!(!e.call(0x0051_9500, &[this, 0]).bool());
        assert!(!e.call(0x0051_9500, &[this, form]).bool());
        e.mem.set_u32(form + 0x18, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0051_9500, &[this, form]).bool());
        let log = e.call_log.take().unwrap();
        let member: Vec<_> = log.iter().filter(|(a, _)| *a == MEMBER_005F65D0).collect();
        assert_eq!(member[0].1[0], this + 0x88);
        e.mem.set_u32(form + 0x10, 0x40);
        assert!(!e.call(0x0051_9500, &[this, form]).bool());
    }
}
