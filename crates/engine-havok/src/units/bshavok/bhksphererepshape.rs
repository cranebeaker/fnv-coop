//! `BSHavok/bhksphererepshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhknicollisionobject::{enter_context, push_viewer_string};

/// The class's vtable (`01030b0c`), the class's static object count (`01268274`, which the
/// destructor decrements) and the `NiRTTI` pointer word (`01268278`) that `GetViewerStrings`
/// shows as the class name.
const VTABLE: u32 = 0x0103_0b0c;
const OBJECT_COUNT: u32 = 0x0126_8274;
const CLASS_RTTI: u32 = 0x0126_8278;
/// The `bhkSerializable` kill (`00c876a0`), the destructor body of the base (`00c84c50`), the
/// viewer strings of the base (`00c851c0`, `(this, strings)`) and the class name builder
/// (`00a7d330`, cdecl).
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const BASE_DESTRUCTOR_BODY: u32 = 0x00c8_4c50;
const BASE_GET_VIEWER_STRINGS: u32 = 0x00c8_51c0;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;

// Translated from 00ca0540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca0540 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkSphereRepShape::~bhkSphereRepShape` (engine map): the destructor body. Sets the vtable,
/// kills the serializable base (`00c876a0`), decrements the class's object count and runs the
/// base destructor body (`00c84c50`).
pub fn bhk_sphere_rep_shape_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(OBJECT_COUNT);
    e.mem.set_u32(OBJECT_COUNT, count.wrapping_sub(1));
    e.call(BASE_DESTRUCTOR_BODY, &args![this]);
}

// Translated from 00ca05b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca05b0 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkSphereRepShape::GetViewerStrings` (engine map): with the thread block's `+0x2b4` word at
/// 0x10, appends to `strings` the base class strings (`00c851c0`) and the class name
/// (`00a7d330` on the `NiRTTI` pointer at `01268278`).
pub fn bhk_sphere_rep_shape_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let (word, saved) = enter_context(e);
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(CLASS_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings.addr(), text);
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ca0540, bhk_sphere_rep_shape_destructor(Ptr)),
        entry!(
            0x00ca05b0,
            bhk_sphere_rep_shape_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Words = Vec<u32>;

    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Words> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn destructor_kills_the_base_counts_down_and_runs_the_base_body() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(OBJECT_COUNT, 8);
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(BASE_DESTRUCTOR_BODY, |_, _| Ret::default());
        e.mem.set_u32(OBJECT_COUNT, 5);
        let this = e.mem.alloc(0x14);
        e.call(0x00ca0540, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 4);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, BASE_DESTRUCTOR_BODY), vec![vec![this]]);
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name_after_the_base_strings() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(CLASS_RTTI, 4);
        e.mem.set_u32(CLASS_RTTI, 0x5151_0001);
        e.register(BASE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| ret(0xa000));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        let stored = Rc::new(RefCell::new(Vec::new()));
        let sink = stored.clone();
        e.register_double(STRINGS_SET_AT, move |e, a| {
            sink.borrow_mut().push((a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        let this = e.mem.alloc(0x14);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.mem.set_u16(strings + 0xa, 1);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x42);

        e.call(0x00ca05b0, &args![this, strings]);

        assert_eq!(
            calls_to(&e, BASE_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x5151_0001]]);
        assert_eq!(*stored.borrow(), vec![(1, 0xa000)]);
        assert!(calls_to(&e, STRINGS_GROW).is_empty());
        assert_eq!(e.mem.u32(word), 0x42);
        // A full array is grown first (size 4, step 3).
        e.mem.set_u16(strings + 0xa, 4);
        e.mem.set_u16(strings + 0xe, 3);
        e.call(0x00ca05b0, &args![this, strings]);
        assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 7]]);
    }
}
