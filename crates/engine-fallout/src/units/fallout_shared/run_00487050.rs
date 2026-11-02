//! `fallout shared/run_00487050` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesfile::TESFile;

/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`): the current chunk id.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The current chunk's size (`00401660`, `TESFile` `+0x25C`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`): `thiscall
/// (file; buffer, size)`, here with size 0.
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// The string setter at `00489100` (`thiscall (component; text)`; engine map
/// name `TESSoundFile::SetSoundFile`, a shared body).
const SET_STRING: u32 = 0x0048_9100;
/// The chunk id this loader reads: the bytes `FULL` as a little-endian word.
const FULL_CHUNK: u32 = 0x4c4c_5546;

// Translated from 00487050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00487050 - the /GS stack cookie check (00ec408c) is not translated.
/// `cdecl (component, file)`: with both non-null and the file's current
/// chunk a `FULL` chunk, reads the chunk into a stack buffer of the chunk's
/// size (`_alloca_probe_16`) and hands it to the component's string setter
/// (`00489100`); an empty chunk sets a null string. Any other chunk leaves
/// the component as it is. Other translations call it the `TESFullName`
/// loader; the engine map gives it no name (unconfirmed).
pub fn fn_00487050(e: &mut Engine, component: Ptr, file: Ptr<TESFile>) {
    if file.addr() == 0 || component.addr() == 0 {
        return;
    }
    let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
    if chunk != FULL_CHUNK {
        return;
    }
    if e.call(CHUNK_SIZE, &args![file]).u32() == 0 {
        e.call(SET_STRING, &args![component, 0u32]);
        return;
    }
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
    let reserved = size.wrapping_add(15) & !15;
    e.with_stack(reserved, |e, buffer| {
        e.call(GET_CHUNK_DATA, &args![file, buffer, 0u32]);
        e.call(SET_STRING, &args![component, buffer]);
    });
}

// Translated from 004871d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form component `TESHealthForm` (the engine map
/// gives it no name; the RTTI name of the vtable it stores, `0101cb94`, is
/// `.?AVTESHealthForm@@`): runs the base constructor (`00402d30`), stores the
/// vtable at `+0`, then calls `TESHealthForm::InitializeDataComponent`
/// (Xbox PDB name of `00487220`) directly. Returns `this`.
pub fn fn_004871d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_cb94);
    e.call(0x0048_7220, &args![this]);
    this
}

// Translated from 00487200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0101cb94` (RTTI `.?AVTESHealthForm@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00483710`. One word, plain `RET`, returns nothing.
pub fn fn_00487200(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_cb94);
    e.call(0x0048_3710, &args![this]);
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptor `.?AVBaseFormComponent@@`.
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
/// RTTI type descriptor `.?AVTESFullName@@`.
const TYPE_TES_FULL_NAME: u32 = 0x0118_3158;
/// `00408da0` (`thiscall`, no stack words; engine map
/// `MapMarkerData::GetLocationName`, a shared body): what `00559450` gives
/// for +4, or `01011584` (an empty string) when that is 0.
const GET_TEXT: u32 = 0x0040_8da0;
/// `004037f0` (`thiscall`, two words: text, then a word passed as 0): sets
/// a `BSStringT`.
const STRING_SET: u32 = 0x0040_37f0;

// Translated from 004870f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the name of another component (`thiscall (this; other)`; no Xbox
/// PDB name): when `other` (a `BaseFormComponent`) casts to `TESFullName`,
/// its text (`00408da0`) is set into the string at +4 of `this`
/// (`004037f0(text, 0)`).
pub fn fn_004870f0(e: &mut Engine, this: Ptr, other: Ptr) {
    let source = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_FULL_NAME,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return;
    }
    let text = e.call(GET_TEXT, &args![source]).u32();
    e.call(STRING_SET, &args![this.addr() + 4, text, 0u32]);
}

/// `0048cee0` (`thiscall`, no stack word): the length of the name string.
const NAME_LENGTH: u32 = 0x0048_cee0;
/// `BSStringT<char>::StrCmp` (engine map, `00408a80`, `thiscall`, `RET 8`:
/// text and a flag): 0 when the strings match.
const STRING_COMPARE: u32 = 0x0040_8a80;

// Translated from 00487140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFullName`'s compare (`thiscall`, `RET 4`; the engine map gives it no
/// name): true (they differ) when `other` is not a `TESFullName`
/// (`dynamic_cast` from `BaseFormComponent`). When either name has a length
/// (`0048cee0`), true when the lengths differ or the string at +4 compares
/// unequal (`00408a80` with the other's text, `00408da0`, and 0); false
/// otherwise.
pub fn fn_00487140(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let cast = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_FULL_NAME,
                0u32
            ],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    if e.call(NAME_LENGTH, &args![this]).u32() == 0 && e.call(NAME_LENGTH, &args![cast]).u32() == 0
    {
        return false;
    }
    let mine = e.call(NAME_LENGTH, &args![this]).u32();
    let theirs = e.call(NAME_LENGTH, &args![cast]).u32();
    if mine != theirs {
        return true;
    }
    let text = e.call(GET_TEXT, &args![cast]).u32();
    e.call(
        STRING_COMPARE,
        &args![this.addr().wrapping_add(4), text, 0u32],
    )
    .u32()
        != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_7140, fn_00487140(Ptr, Ptr) -> bool),
        entry!(0x0048_7200, fn_00487200(Ptr)),
        entry!(0x00487050, fn_00487050(Ptr, Ptr<TESFile>)),
        entry!(0x004871d0, fn_004871d0(Ptr) -> Ptr),
        entry!(0x0048_70f0, fn_004870f0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Runs the loader on a file whose current chunk is `chunk` with `size`
    /// bytes "NAME\0"; returns the log and the text the setter saw.
    fn run(component: u32, file: u32, chunk: u32, size: u32) -> (CallLog, Option<String>) {
        let mut e = Engine::new();
        e.register_double(GET_TES_CHUNK, move |_, _| chunk.into_ret());
        e.register_double(CHUNK_SIZE, move |_, _| size.into_ret());
        e.register(GET_CHUNK_DATA, |e, a| {
            e.mem.write(a[1], b"NAME\0");
            Ret::default()
        });
        let seen = std::rc::Rc::new(std::cell::RefCell::new(None));
        let record = seen.clone();
        e.register_double(SET_STRING, move |e, a| {
            let text = if a[1] == 0 {
                String::from("<null>")
            } else {
                String::from_utf8(e.mem.bytes(a[1], 4)).unwrap()
            };
            *record.borrow_mut() = Some(text);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0048_7050, &args![component, file]);
        let log = e.call_log.take().unwrap();
        let text = seen.borrow().clone();
        (log, text)
    }

    #[test]
    fn reads_a_full_chunk_into_the_component() {
        let (log, text) = run(0x5000, 0x6000, FULL_CHUNK, 5);
        assert_eq!(text.as_deref(), Some("NAME"));
        let reads = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(reads.len(), 1);
        assert_eq!((reads[0][0], reads[0][2]), (0x6000, 0));
        assert_eq!(calls_to(&log, SET_STRING)[0][0], 0x5000);
        assert_eq!(calls_to(&log, CHUNK_SIZE).len(), 2);
    }

    #[test]
    fn an_empty_full_chunk_sets_a_null_string() {
        let (log, text) = run(0x5000, 0x6000, FULL_CHUNK, 0);
        assert_eq!(text.as_deref(), Some("<null>"));
        assert_eq!(calls_to(&log, SET_STRING), vec![vec![0x5000, 0]]);
        assert!(calls_to(&log, GET_CHUNK_DATA).is_empty());
    }

    #[test]
    fn other_chunks_and_null_arguments_do_nothing() {
        let (log, text) = run(0x5000, 0x6000, 0x4c4c_5547, 5);
        assert!(text.is_none());
        assert!(calls_to(&log, CHUNK_SIZE).is_empty());
        let (log, _) = run(0, 0x6000, FULL_CHUNK, 5);
        assert!(calls_to(&log, GET_TES_CHUNK).is_empty());
        let (log, _) = run(0x5000, 0, FULL_CHUNK, 5);
        assert!(calls_to(&log, GET_TES_CHUNK).is_empty());
    }
}

#[cfg(test)]
mod component_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004871d0_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0048_71d0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)]), (0x0048_7220, &[A::T(0x0)])],
            stores: &[(0x0, 4, 0x0101_cb94)],
        });
    }

    #[test]
    fn fn_00487200_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0048_3710, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0048_7200, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0048_3710)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0101_cb94);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0101_cb94,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod copy_tests {
    use super::*;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn fn_004870f0_copies_the_name_of_a_full_name_component() {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |_, a| {
            assert_eq!(a[1..], [0, TYPE_BASE_FORM_COMPONENT, TYPE_TES_FULL_NAME, 0]);
            word(a[0])
        });
        e.register(GET_TEXT, |_, a| word(a[0] + 0x10));
        e.register(STRING_SET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_70f0, &args![0x100u32, 0x200u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap(), &(STRING_SET, vec![0x104, 0x210, 0]));

        e.register(RT_DYNAMIC_CAST, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x0048_70f0, &args![0x100u32, 0x200u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn at_00487140_compares_lengths_then_text() {
        let mut e = Engine::new();
        e.mem.map(0x0f00_0000, 8);
        e.register(RT_DYNAMIC_CAST, |e, _| word(e.mem.u32(0x0f00_0000)));
        e.register(NAME_LENGTH, |e, a| word(e.mem.u32(a[0] + 8)));
        e.register(GET_TEXT, |_, a| word(a[0] + 0x100));
        e.register(STRING_COMPARE, |e, _| word(e.mem.u32(0x0f00_0004)));
        let this = e.mem.alloc(0xc);
        let other = e.mem.alloc(0xc);
        let run = |e: &mut Engine| e.call(0x0048_7140, &args![this, other]).bool();
        assert!(run(&mut e));
        e.mem.set_u32(0x0f00_0000, other);
        // Both empty.
        assert!(!run(&mut e));
        // Lengths differ (only one set, either side).
        e.mem.set_u32(other + 8, 3);
        assert!(run(&mut e));
        e.mem.set_u32(other + 8, 0);
        e.mem.set_u32(this + 8, 3);
        assert!(run(&mut e));
        // Same length: the string compare decides.
        e.mem.set_u32(other + 8, 3);
        e.call_log = Some(vec![]);
        assert!(!run(&mut e));
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.last().unwrap(),
            &(STRING_COMPARE, vec![this + 4, other + 0x100, 0])
        );
        e.mem.set_u32(0x0f00_0004, 0xffff_ffff);
        assert!(run(&mut e));
    }
}
