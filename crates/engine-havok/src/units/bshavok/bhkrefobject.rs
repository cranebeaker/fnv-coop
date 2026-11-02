//! `BSHavok/bhkrefobject.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c87dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRefObject::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c87d20`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0xc bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_ref_object_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c8_7d20;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xcu32]);
    }
    this
}

// Translated from 00c87c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRefObject::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8160.
pub fn bhk_ref_object_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8160
}

/// The per-thread word (`TLS + 0x2b4`) the cleaning functions set to 0x10 while they work.
const SCOPE_WORD: u32 = 0x2b4;
/// `hkReferencedObject::addReference` (`00c90510`) and `removeReference` (`00c905b0`), thiscall
/// on the Havok object.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The base constructor `004b5120` (thiscall) that `00c87c30`'s sibling `0056e6f0` runs, and the
/// base destructor `00a5d3d0` (thiscall) `~bhkRefObject` ends with.
const SHAPE_BASE_CONSTRUCT: u32 = 0x004b_5120;
const BASE_DESTRUCT: u32 = 0x00a5_d3d0;
/// Vtables stored: `bhkShape` (RTTI-confirmed) by `0056e6f0` and `bhkRefObject` by the
/// destructor; the live-object counter `0056e6f0` increments.
const VTABLE_SHAPE: u32 = 0x0103_0bf4;
const VTABLE_REF_OBJECT: u32 = 0x010c_5b54;
const COUNT_SHAPE: u32 = 0x0126_80f8;
/// Viewer-string helpers: `NiObject::GetViewerStrings` (`00a5d6e0`, thiscall `(strings)`),
/// `NiGetViewerString` (`00a7d330`, cdecl `(rtti)`), the string (`00a7d5e0`, cdecl `(name,
/// text)`) and int (`00a7d4d0`, cdecl `(name, value)`) overloads, and the bounded `sprintf`
/// (`00406d00`, cdecl `(buffer, size, format, value)`).
const NI_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
const NI_GET_VIEWER_STRING_INT: u32 = 0x00a7_d4d0;
const SPRINTF_BOUNDED: u32 = 0x0040_6d00;
/// The `NiRTTI` global whose viewer string heads the list, the object-address format, and the
/// names of the address and reference-count strings.
const REF_OBJECT_RTTI: u32 = 0x0126_8160;
const ADDRESS_FORMAT: u32 = 0x010c_54b0;
const ADDRESS_NAME: u32 = 0x010c_5bf4;
const REFERENCE_COUNT_NAME: u32 = 0x010c_5be8;
/// The viewer-string array's grow (`0096ad30`) and set-at (`0096ae90`) methods.
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

/// Sets the allocation-scope word to 0x10 and returns its address and old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    (word, saved)
}

/// Appends the viewer string `text` to the array `strings` (size and capacity are 16-bit fields
/// at `+0xA` and `+8`; `+0xE` is the grow step): grows it when full, then sets the element at
/// the old size.
fn push_viewer_string(e: &mut Engine, strings: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 0056e6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bhkShape` constructor (the map names it `bhkRefObject::bhkRefObject`; the vtable it
/// stores, `01030bf4`, is RTTI-confirmed `bhkShape`): runs the base constructor `004b5120`,
/// stores the vtable, clears `+0x10` and increments the live-object counter `012680f8`.
/// Answers `this`.
pub fn fn_0056e6f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SHAPE_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_SHAPE);
    e.mem.set_u32(this.addr() + 0x10, 0);
    let count: u32 = e.global(COUNT_SHAPE);
    e.set_global(COUNT_SHAPE, count.wrapping_add(1));
    this
}

// Translated from 00c87c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRefObject::SethkObject` (Xbox PDB): when `object` differs from the Havok object at
/// `+8`, calls virtual slot `0x90` with 0, stores `object` and calls slot `0x90` with 1.
pub fn bhk_ref_object_set_hk_object(e: &mut Engine, this: Ptr, object: u32) {
    if e.mem.u32(this.addr() + 8) == object {
        return;
    }
    e.vcall(this.addr(), 0x90, &args![0u32]);
    e.mem.set_u32(this.addr() + 8, object);
    e.vcall(this.addr(), 0x90, &args![1u32]);
}

// Translated from 00c87c90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c87c90 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRefObject::Link` (Xbox PDB): with the allocation-scope word at 0x10 and a Havok object
/// at `+8`, takes a reference on it when `add` is not 0 (`00c90510`) and releases one
/// otherwise (`00c905b0`).
pub fn bhk_ref_object_link(e: &mut Engine, this: Ptr, add: u8) {
    let (word, saved) = enter_scope(e);
    let object = e.mem.u32(this.addr() + 8);
    if object != 0 {
        if add != 0 {
            e.call(ADD_REFERENCE, &args![object]);
        } else {
            e.call(REMOVE_REFERENCE, &args![object]);
        }
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c87d20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c87d20 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRefObject::~bhkRefObject` (Xbox PDB): stores the class vtable, releases the Havok object
/// at `+8` (`00c905b0`) with the allocation-scope word at 0x10, and runs the base destructor
/// `00a5d3d0`.
pub fn bhk_ref_object_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_REF_OBJECT);
    let (word, saved) = enter_scope(e);
    let object = e.mem.u32(this.addr() + 8);
    if object != 0 {
        e.call(REMOVE_REFERENCE, &args![object]);
    }
    e.mem.set_u32(word, saved);
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 00c87de0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c87de0 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `bhkRefObject::GetViewerStrings` (Xbox PDB): with the allocation-scope word at 0x10, runs
/// `NiObject::GetViewerStrings`, then appends the class's viewer string, the address of the
/// Havok object (`+8`, formatted into a 32-byte buffer) and, when there is one, the 16-bit
/// reference count at its `+6` (sign-extended).
pub fn bhk_ref_object_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(NI_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let rtti = e.global::<u32>(REF_OBJECT_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![rtti]).u32();
    push_viewer_string(e, strings, text);
    let object = e.mem.u32(this.addr() + 8);
    e.with_stack(0x20, |e, buffer| {
        e.call(
            SPRINTF_BOUNDED,
            &args![buffer, 0x20u32, ADDRESS_FORMAT, object],
        );
        let text = e
            .call(NI_GET_VIEWER_STRING_TEXT, &args![ADDRESS_NAME, buffer])
            .u32();
        push_viewer_string(e, strings, text);
    });
    let object = e.mem.u32(this.addr() + 8);
    if object != 0 {
        let count = i32::from(e.mem.i16(object + 6));
        let text = e
            .call(
                NI_GET_VIEWER_STRING_INT,
                &args![REFERENCE_COUNT_NAME, count],
            )
            .u32();
        push_viewer_string(e, strings, text);
    }
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0056e6f0, fn_0056e6f0(Ptr) -> Ptr),
        entry!(0x00c87c50, bhk_ref_object_set_hk_object(Ptr, u32)),
        entry!(0x00c87c80, bhk_ref_object_get_r_t_t_i() -> u32),
        entry!(0x00c87c90, bhk_ref_object_link(Ptr, u8)),
        entry!(0x00c87d20, bhk_ref_object_destruct(Ptr)),
        entry!(0x00c87dc0, bhk_ref_object_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00c87de0, bhk_ref_object_get_viewer_strings(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_ref_object_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c8_7d20, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00c87dc0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c87dc0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0xc]]);
        assert_eq!(calls_to(&e, 0x00c8_7d20), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_ref_object_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c8_7c80, &args![]).u32(), 0x0126_8160);
    }

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A logging double over `address` that answers `value` in `EAX`.
    fn returning(e: &mut Engine, log: &Log, address: u32, value: u32) {
        let kept = std::rc::Rc::clone(log);
        e.register_double(address, move |_, a| {
            kept.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
    }

    fn new_log() -> Log {
        Log::default()
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn shape_constructor_runs_the_base_stores_the_vtable_and_counts() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        let log = new_log();
        returning(&mut e, &log, SHAPE_BASE_CONSTRUCT, 0);
        e.set_global(COUNT_SHAPE, 41u32);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0x99);
        assert_eq!(e.call(0x0056e6f0, &args![this]).u32(), this);
        assert_eq!(calls_to(&log, SHAPE_BASE_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), VTABLE_SHAPE);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.global::<u32>(COUNT_SHAPE), 42);
    }

    #[test]
    fn set_hk_object_calls_slot_0x90_around_the_store_only_on_a_change() {
        let mut e = Engine::new();
        let log = new_log();
        returning(&mut e, &log, 0x0900_0090, 0);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x90, 0x0900_0090);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 8, 0x1111);
        e.call(0x00c87c50, &args![this, 0x1111u32]);
        assert!(log.borrow().is_empty());
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let kept = std::rc::Rc::clone(&seen);
        e.register_double(0x0900_0090, move |e, a| {
            kept.borrow_mut().push((a.to_vec(), e.mem.u32(a[0] + 8)));
            Ret::default()
        });
        e.call(0x00c87c50, &args![this, 0x2222u32]);
        assert_eq!(
            *seen.borrow(),
            vec![(vec![this, 0], 0x1111), (vec![this, 1], 0x2222)]
        );
        assert_eq!(e.mem.u32(this + 8), 0x2222);
    }

    #[test]
    fn link_adds_or_releases_the_reference_inside_the_scope() {
        let mut e = Engine::new();
        let log = new_log();
        for address in [ADD_REFERENCE, REMOVE_REFERENCE] {
            let kept = std::rc::Rc::clone(&log);
            e.register_double(address, move |e, a| {
                let word = e.tls() + SCOPE_WORD;
                assert_eq!(e.mem.u32(word), 0x10);
                kept.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 5);
        let this = e.mem.alloc(0x10);
        e.call(0x00c87c90, &args![this, 1u8]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(word), 5);
        e.mem.set_u32(this + 8, 0x4444);
        e.call(0x00c87c90, &args![this, 1u8]);
        e.call(0x00c87c90, &args![this, 0u8]);
        assert_eq!(calls_to(&log, ADD_REFERENCE), vec![vec![0x4444]]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), vec![vec![0x4444]]);
        assert_eq!(e.mem.u32(word), 5);
    }

    #[test]
    fn destructor_releases_the_object_then_runs_the_base() {
        let mut e = Engine::new();
        let log = new_log();
        returning(&mut e, &log, REMOVE_REFERENCE, 0);
        returning(&mut e, &log, BASE_DESTRUCT, 0);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 9);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x4444);
        e.call(0x00c87d20, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_REF_OBJECT);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order, vec![REMOVE_REFERENCE, BASE_DESTRUCT]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), vec![vec![0x4444]]);
        assert_eq!(calls_to(&log, BASE_DESTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(word), 9);
        log.borrow_mut().clear();
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c87d20, &args![this]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE).len(), 0);
        assert_eq!(calls_to(&log, BASE_DESTRUCT).len(), 1);
    }

    /// Installs doubles for the viewer-string helpers: the three string makers hand out
    /// 0x100, 0x101, ... and the array methods keep the size and the stored handles.
    fn install_viewer(e: &mut Engine, log: &Log) -> u32 {
        for address in [
            NI_OBJECT_GET_VIEWER_STRINGS,
            NI_GET_VIEWER_STRING,
            NI_GET_VIEWER_STRING_TEXT,
            NI_GET_VIEWER_STRING_INT,
            SPRINTF_BOUNDED,
        ] {
            let kept = std::rc::Rc::clone(log);
            e.register_double(address, move |e, a| {
                kept.borrow_mut().push((address, a.to_vec()));
                let made = e.mem.u32(0x0900_0000);
                e.mem.set_u32(0x0900_0000, made + 1);
                Ret {
                    eax: made,
                    ..Ret::default()
                }
            });
        }
        e.register_double(STRINGS_GROW, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register_double(STRINGS_SET_AT, |e, a| {
            let table = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(table + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let array = e.mem.alloc(0x10);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(array + 4, table);
        e.mem.set_u16(array + 8, 1);
        e.mem.set_u16(array + 0xe, 2);
        array
    }

    #[test]
    fn viewer_strings_append_class_address_and_reference_count() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.map(0x0900_0000, 0x1000);
        e.mem.set_u32(0x0900_0000, 0x100);
        let log = new_log();
        let array = install_viewer(&mut e, &log);
        e.set_global(REF_OBJECT_RTTI, 0x7777u32);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 3);
        let object = e.mem.alloc(0x10);
        e.mem.set_i16(object + 6, -2);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, object);
        e.call(0x00c87de0, &args![this, array]);
        assert_eq!(
            calls_to(&log, NI_OBJECT_GET_VIEWER_STRINGS),
            vec![vec![this, array]]
        );
        assert_eq!(calls_to(&log, NI_GET_VIEWER_STRING), vec![vec![0x7777]]);
        let formatted = calls_to(&log, SPRINTF_BOUNDED);
        assert_eq!(formatted.len(), 1);
        assert_eq!(formatted[0][1..], [0x20, ADDRESS_FORMAT, object]);
        let text = calls_to(&log, NI_GET_VIEWER_STRING_TEXT);
        assert_eq!(text, vec![vec![ADDRESS_NAME, formatted[0][0]]]);
        assert_eq!(
            calls_to(&log, NI_GET_VIEWER_STRING_INT),
            vec![vec![REFERENCE_COUNT_NAME, -2i32 as u32]]
        );
        // The string makers answered 0x100 + (helper call order): base 0x100, class 0x101,
        // address 0x103 (after the formatter's 0x102), count 0x104.
        let table = e.mem.u32(array + 4);
        assert_eq!(e.mem.u16(array + 0xa), 3);
        assert_eq!(
            [e.mem.u32(table), e.mem.u32(table + 4), e.mem.u32(table + 8)],
            [0x101, 0x103, 0x104]
        );
        assert_eq!(e.mem.u32(word), 3);
    }

    #[test]
    fn viewer_strings_without_a_havok_object_stop_after_the_address() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.map(0x0900_0000, 0x1000);
        let log = new_log();
        let array = install_viewer(&mut e, &log);
        let this = e.mem.alloc(0x10);
        e.call(0x00c87de0, &args![this, array]);
        assert_eq!(calls_to(&log, SPRINTF_BOUNDED)[0][3], 0);
        assert!(calls_to(&log, NI_GET_VIEWER_STRING_INT).is_empty());
        assert_eq!(e.mem.u16(array + 0xa), 2);
    }
}
