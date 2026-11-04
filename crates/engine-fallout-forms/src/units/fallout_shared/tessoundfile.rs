//! `fallout shared/tessoundfile.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSStringT<char>` text assignment `(string; text, min_capacity)`
/// (`004037f0`).
const STRING_SET: u32 = 0x0040_37f0;
/// `__RTDynamicCast(object, 0, from, to, 0)` (CRT).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The RTTI type descriptors of `BaseFormComponent` and `TESSoundFile`
/// (their decorated names `.?AVBaseFormComponent@@` and `.?AVTESSoundFile@@`
/// sit at +8).
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_TES_SOUND_FILE: u32 = 0x0118_6b50;
/// The text of the string at `this + 4`, or the empty string `01011584`
/// when it has none (`00408da0`; the engine map's name for it,
/// `MapMarkerData::GetLocationName`, belongs to folded code).
const STRING_AT_4_TEXT: u32 = 0x0040_8da0;
/// The string length at `this + 4` (`0048cee0`, through `004048e0`).
const STRING_AT_4_LENGTH: u32 = 0x0048_cee0;
/// The case-insensitive compare `(a, b)` (0 when equal, `00404dc0`).
const STRICMP: u32 = 0x0040_4dc0;

// Translated from 00489100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names this `TESSoundFile::SetSoundFile` (Xbox PDB); the
/// code is shared by identical-code folding, so it is named by its body:
/// copies the C string `text` into the `BSStringT<char>` at `this + 4`
/// (`004037f0` with a minimum capacity of 0).
pub fn fn_00489100(e: &mut Engine, this: Ptr, text: Ptr) {
    e.call(STRING_SET, &args![this.addr() + 4, text, 0u32]);
}

// Translated from 0048cf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSoundFile::CopyComponent` (Xbox PDB): when `other` is a
/// `TESSoundFile` (a dynamic cast from `BaseFormComponent`), sets this
/// component's file name to `other`'s text through the virtual method at
/// slot `+0x10`.
pub fn tes_sound_file_copy_component(e: &mut Engine, this: Ptr, other: Ptr) {
    let source = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_SOUND_FILE,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return;
    }
    let text = e.call(STRING_AT_4_TEXT, &args![source]).u32();
    e.vcall(this.addr(), 0x10, &args![text]);
}

// Translated from 0048cf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSoundFile::CompareComponent` (Xbox PDB): answers true when the two
/// components differ. `other` that is not a `TESSoundFile` differs; two
/// empty names are equal; otherwise names of different lengths differ, and
/// names of the same length are compared without regard to case.
pub fn tes_sound_file_compare_component(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let source = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_SOUND_FILE,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return true;
    }
    if e.call(STRING_AT_4_LENGTH, &args![this]).u32() == 0
        && e.call(STRING_AT_4_LENGTH, &args![source]).u32() == 0
    {
        return false;
    }
    let own_length = e.call(STRING_AT_4_LENGTH, &args![this]).u32();
    if own_length != e.call(STRING_AT_4_LENGTH, &args![source]).u32() {
        return true;
    }
    let other_text = e.call(STRING_AT_4_TEXT, &args![source]).u32();
    let own_text = e.call(STRING_AT_4_TEXT, &args![this]).u32();
    e.call(STRICMP, &args![own_text, other_text]).u32() != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00489100, fn_00489100(Ptr, Ptr)),
        entry!(0x0048cf00, tes_sound_file_copy_component(Ptr, Ptr)),
        entry!(
            0x0048cf50,
            tes_sound_file_compare_component(Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Doubles: the cast answers `other + 0x100` unless `other` is 0x2000;
    /// a component's text is `component + 0x40`; its length is the `u32`
    /// at `component + 0x20`; the compare answers the `u32` at 0x3000.
    fn world() -> Engine {
        let mut e = Engine::new();
        e.map(0x2000, 0x2000);
        e.register(DYNAMIC_CAST, |_, a| {
            eax(if a[0] == 0x2000 { 0 } else { a[0] + 0x100 })
        });
        e.register(STRING_AT_4_TEXT, |_, a| eax(a[0] + 0x40));
        e.register(STRING_AT_4_LENGTH, |e, a| eax(e.mem.u32(a[0] + 0x20)));
        e.register(STRICMP, |e, _| eax(e.mem.u32(0x3000)));
        e
    }

    #[test]
    fn set_sound_file_assigns_the_string_at_plus_4() {
        let mut e = Engine::new();
        e.register(STRING_SET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_9100, &[0x2000, 0x5000]);
        assert_eq!(calls_to(&e, STRING_SET), [[0x2004, 0x5000, 0]]);
    }

    #[test]
    fn copy_component_takes_the_name_of_a_sound_file_only() {
        let mut e = world();
        e.put_vtable(0x4000, &[0, 0, 0, 0, 0x0090_0000]);
        e.mem.set_u32(0x2800, 0x4000);
        e.register(0x0090_0000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_cf00, &[0x2800, 0x2000]);
        assert!(calls_to(&e, 0x0090_0000).is_empty());
        e.call(0x0048_cf00, &[0x2800, 0x2400]);
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST)[1],
            [0x2400, 0, TYPE_BASE_FORM_COMPONENT, TYPE_TES_SOUND_FILE, 0]
        );
        assert_eq!(calls_to(&e, STRING_AT_4_TEXT), [[0x2500]]);
        assert_eq!(calls_to(&e, 0x0090_0000), [[0x2800, 0x2540]]);
    }

    #[test]
    fn compare_component_branches() {
        let mut e = world();
        let compare = |e: &mut Engine, other: u32| e.call(0x0048_cf50, &[0x2800, other]).bool();
        // Not a sound file: different.
        assert!(compare(&mut e, 0x2000));
        // Both empty: equal, no text compared.
        e.call_log = Some(vec![]);
        assert!(!compare(&mut e, 0x2400));
        assert!(calls_to(&e, STRICMP).is_empty());
        // Different lengths: different.
        e.mem.set_u32(0x2820, 3);
        assert!(compare(&mut e, 0x2400));
        // Same length: the case-insensitive compare decides.
        e.mem.set_u32(0x2520, 3);
        e.call_log = Some(vec![]);
        assert!(!compare(&mut e, 0x2400));
        assert_eq!(calls_to(&e, STRICMP), [[0x2840, 0x2540]]);
        e.mem.set_u32(0x3000, 1);
        assert!(compare(&mut e, 0x2400));
        // Only the other name is set: lengths differ.
        e.mem.set_u32(0x2820, 0);
        assert!(compare(&mut e, 0x2400));
    }
}
