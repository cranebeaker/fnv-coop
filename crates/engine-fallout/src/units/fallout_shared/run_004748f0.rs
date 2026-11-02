//! `fallout shared/run_004748f0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// This class's vtable (RTTI `.?AVBGSAmmoForm@@`).
const VTABLE: u32 = 0x0101_a3b8;
/// The `InitializeDataComponent` body the constructor calls (`008c6060`: a shared empty body; the engine map names it ActorPackage::NullInstanceData because the linker folded it).
const INITIALIZE_DATA: u32 = 0x008c_6060;

// Translated from 004748f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form component `BGSAmmoForm` (the engine map gives it
/// no name; the RTTI name of the vtable it stores is `.?AVBGSAmmoForm@@`): runs the
/// base constructor, stores the vtable at `+0`, then calls the component's
/// `InitializeDataComponent` body directly. Returns `this`.
pub fn fn_004748f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(INITIALIZE_DATA, &args![this]);
    this
}

/// `00401170` (thiscall): the form's type byte (`TESForm` +0x04).
const FORM_TYPE: u32 = 0x0040_1170;

/// The component's form (+0x04) when it has form type `form_type`, else 0.
fn form_of_type(e: &mut Engine, this: Ptr, form_type: u32) -> u32 {
    let form = e.mem.u32(this.addr() + 4);
    if form != 0 {
        let form = e.mem.u32(this.addr() + 4);
        if e.call(FORM_TYPE, &args![form]).u32() == form_type {
            return e.mem.u32(this.addr() + 4);
        }
    }
    0
}

// Translated from 00474a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The form at +0x04 when its type (`00401170`) is 0x29, else 0 (the form
/// type number is not confirmed as ammunition here).
pub fn fn_00474a00(e: &mut Engine, this: Ptr) -> u32 {
    form_of_type(e, this, 0x29)
}

// Translated from 00474a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The form at +0x04 when its type is 0x55, else 0.
pub fn fn_00474a40(e: &mut Engine, this: Ptr) -> u32 {
    form_of_type(e, this, 0x55)
}

// Translated from 00474920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The form the component stands for: the type-0x29 form at +0x04
/// (`00474a00`, called again for the result) or, when it is a type-0x55 form
/// (`00474a40`), an item of that form's list: `007af430` gives an index and
/// `00500940` the list's first node; when the list's count (`005ae380`) is
/// above the index, the node `index` steps on (`00726070` each step),
/// otherwise the first node; the word at the address `006815c0` gives for
/// that node is returned. 0 when the form is neither.
pub fn fn_00474920(e: &mut Engine, this: Ptr) -> u32 {
    if fn_00474a00(e, this) != 0 {
        return fn_00474a00(e, this);
    }
    if fn_00474a40(e, this) == 0 {
        return 0;
    }
    let list = fn_00474a40(e, this);
    let index = e.call(0x007a_f430, &args![list]).u32();
    let list = fn_00474a40(e, this);
    let first = e.call(0x0050_0940, &args![list]).u32();
    let count = e.call(0x005a_e380, &args![first]).u32();
    let item = if count > index {
        let list = fn_00474a40(e, this);
        let mut node = e.call(0x0050_0940, &args![list]).u32();
        let mut step = 0u32;
        while step < index {
            node = e.call(0x0072_6070, &args![node]).u32();
            step += 1;
        }
        e.call(0x0068_15c0, &args![node]).u32()
    } else {
        let list = fn_00474a40(e, this);
        let node = e.call(0x0050_0940, &args![list]).u32();
        e.call(0x0068_15c0, &args![node]).u32()
    };
    e.mem.u32(item)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004748f0, fn_004748f0(Ptr) -> Ptr),
        entry!(0x0047_4a00, fn_00474a00(Ptr) -> u32),
        entry!(0x0047_4a40, fn_00474a40(Ptr) -> u32),
        entry!(0x0047_4920, fn_00474920(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004748f0_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004748f0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)]), (0x008c_6060, &[A::T(0x0)])],
            stores: &[(0x0, 4, 0x0101_a3b8)],
        });
    }
}

#[cfg(test)]
mod b024_tests {
    use crate::prelude::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// A component whose +0x04 form has type `form_type` (0: no form).
    fn component(e: &mut Engine, form_type: u32) -> u32 {
        let this = e.mem.alloc(0x10);
        if form_type != 0 {
            let form = e.mem.alloc(0x20);
            e.mem.set_u32(form + 4, form_type);
            e.mem.set_u32(this + 4, form);
        }
        e.register(0x0040_1170, |e, a| ret(e.mem.u32(a[0] + 4)));
        this
    }

    #[test]
    fn at_00474a00_gives_a_type_29_form() {
        let mut e = Engine::new();
        let this = component(&mut e, 0x29);
        let form = e.mem.u32(this + 4);
        assert_eq!(e.call(0x0047_4a00, &[this]).u32(), form);
        let other = component(&mut e, 0x55);
        assert_eq!(e.call(0x0047_4a00, &[other]).u32(), 0);
        let none = component(&mut e, 0);
        assert_eq!(e.call(0x0047_4a00, &[none]).u32(), 0);
    }

    #[test]
    fn at_00474a40_gives_a_type_55_form() {
        let mut e = Engine::new();
        let this = component(&mut e, 0x55);
        let form = e.mem.u32(this + 4);
        assert_eq!(e.call(0x0047_4a40, &[this]).u32(), form);
        let other = component(&mut e, 0x29);
        assert_eq!(e.call(0x0047_4a40, &[other]).u32(), 0);
        let none = component(&mut e, 0);
        assert_eq!(e.call(0x0047_4a40, &[none]).u32(), 0);
    }

    /// A type-0x55 component over a list of three nodes; returns the result
    /// and how many node steps ran.
    fn from_list(index: u32, count: u32) -> (u32, usize) {
        let mut e = Engine::new();
        let this = component(&mut e, 0x55);
        // Nodes 0x0060_0000 + 0x10 * n; each node's item word at +0 is 0x100 + n.
        e.map(0x0060_0000, 0x1000);
        for n in 0..4u32 {
            e.mem.set_u32(0x0060_0000 + 0x10 * n, 0x100 + n);
        }
        e.register_double(0x007a_f430, move |_, _| ret(index));
        e.register(0x0050_0940, |_, _| ret(0x0060_0000));
        e.register_double(0x005a_e380, move |_, _| ret(count));
        e.register(0x0072_6070, |_, a| ret(a[0] + 0x10));
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.call_log = Some(vec![]);
        let r = e.call(0x0047_4920, &[this]).u32();
        let steps = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == 0x0072_6070)
            .count();
        (r, steps)
    }

    #[test]
    fn at_00474920_resolves_the_form_or_the_list_item() {
        let mut e = Engine::new();
        let this = component(&mut e, 0x29);
        let form = e.mem.u32(this + 4);
        assert_eq!(e.call(0x0047_4920, &[this]).u32(), form);
        let none = component(&mut e, 0x30);
        assert_eq!(e.call(0x0047_4920, &[none]).u32(), 0);
        assert_eq!(from_list(2, 3), (0x102, 2));
        assert_eq!(from_list(0, 3), (0x100, 0));
        // The index is not below the count: the first item.
        assert_eq!(from_list(3, 3), (0x100, 0));
        assert_eq!(from_list(0xffff_ffff, 3), (0x100, 0));
    }
}
