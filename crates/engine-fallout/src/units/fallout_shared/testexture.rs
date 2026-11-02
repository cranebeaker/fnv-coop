//! `fallout shared/testexture.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesfile::TESFile;

/// The current chunk's size (`00401660`, `TESFile` `+0x25C`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`): `thiscall
/// (file; buffer, size)`, here with size 0.
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// The CRT's `strlen` (`00ec6130`, cdecl).
const STRLEN: u32 = 0x00ec_6130;
/// The string setter at `00489100` (`thiscall (texture; text)`; engine map
/// name `TESSoundFile::SetSoundFile`, a shared body).
const SET_STRING: u32 = 0x0048_9100;

// Translated from 0048e3d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048e3d0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESTexture::LoadTextureChunk` (Xbox PDB), `cdecl (texture, file)`: with
/// both non-null, reads the current chunk into a stack buffer of the chunk's
/// size (`_alloca_probe_16`), drops one leading backslash (shifting the text,
/// its terminator included, one byte left, with `strlen` taken again on each
/// step as the game does) and hands the text to the texture's string setter
/// (`00489100`). An empty chunk sets a null string. The chunk id is not
/// checked.
pub fn tes_texture_load_texture_chunk(e: &mut Engine, texture: Ptr, file: Ptr<TESFile>) {
    if file.addr() == 0 || texture.addr() == 0 {
        return;
    }
    if e.call(CHUNK_SIZE, &args![file]).u32() == 0 {
        e.call(SET_STRING, &args![texture, 0u32]);
        return;
    }
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
    let reserved = size.wrapping_add(15) & !15;
    e.with_stack(reserved, |e, buffer| {
        let text = buffer.addr();
        e.call(GET_CHUNK_DATA, &args![file, text, 0u32]);
        if e.call(STRLEN, &args![text]).u32() != 0 && e.mem.u8(text) == b'\\' {
            let mut index = 0u32;
            while index < e.call(STRLEN, &args![text]).u32() {
                let next = e.mem.u8(text.wrapping_add(index).wrapping_add(1));
                e.mem.set_u8(text.wrapping_add(index), next);
                index = index.wrapping_add(1);
            }
        }
        e.call(SET_STRING, &args![texture, text]);
    });
}

/// `ICON`.
const CHUNK_ICON: u32 = 0x4e4f_4349;
/// `"%s_n.dds"`.
const NORMAL_FORMAT: u32 = 0x0101_d7e0;

// Translated from 0048e3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTexture::IsTextureChunk` (engine map), cdecl: whether the chunk is
/// `ICON`.
pub fn tes_texture_is_texture_chunk(_e: &mut Engine, chunk: u32) -> bool {
    chunk == CHUNK_ICON
}

// Translated from 0048e5b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048e5b0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESTexture::GetAsNormalFile` (engine map), cdecl: sets the string
/// `result` to `path` with its extension replaced by `_n.dds` (the path
/// copied with `00406d30`, the last `.` found with `0040ab30`, formatted
/// with `"%s_n.dds"`, set with `004037f0`) and answers its text
/// (`00559450`); answers 0 without a path or an extension. The stack
/// cookie check is not translated.
pub fn tes_texture_get_as_normal_file(e: &mut Engine, result: Ptr, path: u32) -> u32 {
    if path == 0 {
        return 0;
    }
    e.with_stack(0x210, |e, buffers| {
        let copy = buffers.addr();
        let normal = buffers.addr() + 0x108;
        e.call(0x0040_6d30, &args![copy, 0x104u32, path]);
        let dot = e.call(0x0040_ab30, &args![copy, 0x2eu32]).u32();
        if dot == 0 {
            return 0;
        }
        e.mem.set_u8(dot, 0);
        e.call(0x0040_6d00, &args![normal, 0x104u32, NORMAL_FORMAT, copy]);
        e.call(0x0040_37f0, &args![result, normal, 0u32]);
        e.call(0x0055_9450, &args![result]).u32()
    })
}

// Translated from 0048e580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The normal-map name of this texture: `TESTexture::GetAsNormalFile`
/// into `result` with the texture's path (`00408da0`, engine map name
/// `MapMarkerData::GetLocationName`), whose `EAX` it passes on.
/// `__thiscall`, `RET 4`. The engine map gives it no name.
pub fn fn_0048e580(e: &mut Engine, this: Ptr, result: Ptr) -> u32 {
    let path = e.call(0x0040_8da0, &args![this]).u32();
    tes_texture_get_as_normal_file(e, result, path)
}

/// `0048cee0` (thiscall on a texture): the length of its path (`004048e0` on
/// the string at +4).
const PATH_LENGTH: u32 = 0x0048_cee0;
/// `00408da0` (thiscall on a texture; engine map name
/// `MapMarkerData::GetLocationName`, a shared body): its path text.
const PATH_TEXT: u32 = 0x0040_8da0;
/// `TESForm::AddChunkArray` (engine map, `004856f0`, cdecl: chunk, data,
/// size).
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
/// `004037f0` (thiscall on a string, `RET 8`: text, length): sets it.
const STRING_SET: u32 = 0x0040_37f0;
/// `00404dc0` (cdecl, two strings): their comparison through `00ec68e4`.
const STRING_COMPARE: u32 = 0x0040_4dc0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and the RTTI type
/// descriptors of `BaseFormComponent` (`01183040`) and `TESTexture`
/// (`01183218`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_TES_TEXTURE: u32 = 0x0118_3218;

// Translated from 0048e370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTexture::Save` (Xbox PDB, `RET 4`: the chunk): when the path is not
/// empty (`0048cee0`), adds it as a chunk (`TESForm::AddChunkArray`) with its
/// terminator (length + 1).
pub fn tes_texture_save(e: &mut Engine, this: Ptr, chunk: u32) {
    if e.call(PATH_LENGTH, &args![this]).u32() == 0 {
        return;
    }
    let size = e.call(PATH_LENGTH, &args![this]).u32().wrapping_add(1);
    let text = e.call(PATH_TEXT, &args![this]).u32();
    e.call(ADD_CHUNK_ARRAY, &args![chunk, text, size]);
}

// Translated from 0048e4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`RET 4`: the other component; `TESTexture::CopyComponent`
/// by its shape): when the other component is a `TESTexture`
/// (`__RTDynamicCast` from `BaseFormComponent`), sets this path (the string
/// at +4, `004037f0` with length 0) to the other's text (`00408da0`).
pub fn fn_0048e4a0(e: &mut Engine, this: Ptr, other: u32) {
    let source = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_TEXTURE,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return;
    }
    let text = e.call(PATH_TEXT, &args![source]).u32();
    e.call(STRING_SET, &args![this.addr().wrapping_add(4), text, 0u32]);
}

// Translated from 0048e4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`RET 4`: the other component; `TESTexture::CompareComponent`
/// by its shape): true ("different") when the other component is not a
/// `TESTexture`; false when both paths are empty; otherwise true when the
/// lengths (`0048cee0`) differ or the texts compare unequal (`00404dc0`,
/// this text first).
pub fn fn_0048e4f0(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let source = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_TEXTURE,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return true;
    }
    if e.call(PATH_LENGTH, &args![this]).u32() == 0
        && e.call(PATH_LENGTH, &args![source]).u32() == 0
    {
        return false;
    }
    let length = e.call(PATH_LENGTH, &args![this]).u32();
    if length != e.call(PATH_LENGTH, &args![source]).u32() {
        return true;
    }
    let theirs = e.call(PATH_TEXT, &args![source]).u32();
    let mine = e.call(PATH_TEXT, &args![this]).u32();
    e.call(STRING_COMPARE, &args![mine, theirs]).u32() != 0
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0048e3d0,
            tes_texture_load_texture_chunk(Ptr, Ptr<TESFile>)
        ),
        entry!(0x0048_e3b0, tes_texture_is_texture_chunk(u32) -> bool),
        entry!(0x0048_e5b0, tes_texture_get_as_normal_file(Ptr, u32) -> u32),
        entry!(0x0048_e580, fn_0048e580(Ptr, Ptr) -> u32),
        entry!(0x0048_e370, tes_texture_save(Ptr, u32)),
        entry!(0x0048_e4a0, fn_0048e4a0(Ptr, u32)),
        entry!(0x0048_e4f0, fn_0048e4f0(Ptr, u32) -> bool),
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

    /// Loads a chunk holding `data` (its size is `data.len()`); returns the
    /// log and the text the setter saw.
    fn run(texture: u32, file: u32, data: &'static [u8]) -> (CallLog, Option<String>) {
        let mut e = Engine::new();
        let size = data.len() as u32;
        e.register_double(CHUNK_SIZE, move |_, _| size.into_ret());
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            e.mem.write(a[1], data);
            Ret::default()
        });
        e.register(STRLEN, |e, a| {
            let mut n = 0;
            while e.mem.u8(a[0] + n) != 0 {
                n += 1;
            }
            n.into_ret()
        });
        let seen = std::rc::Rc::new(std::cell::RefCell::new(None));
        let record = seen.clone();
        e.register_double(SET_STRING, move |e, a| {
            let text = if a[1] == 0 {
                String::from("<null>")
            } else {
                let mut bytes = Vec::new();
                let mut at = a[1];
                while e.mem.u8(at) != 0 {
                    bytes.push(e.mem.u8(at));
                    at += 1;
                }
                String::from_utf8(bytes).unwrap()
            };
            *record.borrow_mut() = Some(text);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0048_e3d0, &args![texture, file]);
        let log = e.call_log.take().unwrap();
        let text = seen.borrow().clone();
        (log, text)
    }

    #[test]
    fn drops_one_leading_backslash() {
        let (log, text) = run(0x5000, 0x6000, b"\\\\a\\b.dds\0");
        assert_eq!(text.as_deref(), Some("\\a\\b.dds"));
        let reads = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!((reads[0][0], reads[0][2]), (0x6000, 0));
        assert_eq!(calls_to(&log, SET_STRING)[0][0], 0x5000);
        // Once for the test, then once per step (nine, the terminator moves
        // at the last) and once more to end the loop.
        assert_eq!(calls_to(&log, STRLEN).len(), 1 + 9 + 1);
    }

    #[test]
    fn keeps_a_path_without_a_leading_backslash() {
        let (log, text) = run(0x5000, 0x6000, b"a\\b.dds\0");
        assert_eq!(text.as_deref(), Some("a\\b.dds"));
        assert_eq!(calls_to(&log, STRLEN).len(), 1);
        let (_, text) = run(0x5000, 0x6000, b"\0");
        assert_eq!(text.as_deref(), Some(""));
    }

    #[test]
    fn an_empty_chunk_sets_a_null_string_and_null_arguments_do_nothing() {
        let (log, text) = run(0x5000, 0x6000, b"");
        assert_eq!(text.as_deref(), Some("<null>"));
        assert!(calls_to(&log, GET_CHUNK_DATA).is_empty());
        let (log, text) = run(0, 0x6000, b"x\0");
        assert!(text.is_none());
        assert!(calls_to(&log, CHUNK_SIZE).is_empty());
        let (log, _) = run(0x5000, 0, b"x\0");
        assert!(calls_to(&log, CHUNK_SIZE).is_empty());
    }
}

#[cfg(test)]
mod b022_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn only_icon_is_a_texture_chunk() {
        let mut e = Engine::new();
        assert!(e.call(0x0048_e3b0, &[CHUNK_ICON]).bool());
        assert!(!e.call(0x0048_e3b0, &[0x4c44_4f4d]).bool());
    }

    /// Doubles that behave as the CRT helpers: copy, last `.`, format
    /// (`%s_n.dds`), set (records the text).
    fn engine() -> (Engine, Rc<RefCell<Vec<u8>>>) {
        let mut e = Engine::new();
        e.register(0x0040_6d30, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(0x0040_ab30, |e, a| {
            let text = e.mem.cstr(a[0]);
            ret(text
                .iter()
                .rposition(|&c| c == a[1] as u8)
                .map_or(0, |i| a[0] + i as u32))
        });
        e.register(0x0040_6d00, |e, a| {
            let mut text = e.mem.cstr(a[3]);
            text.extend(b"_n.dds");
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        let set = Rc::new(RefCell::new(vec![]));
        let seen = set.clone();
        e.register_double(0x0040_37f0, move |e, a| {
            *seen.borrow_mut() = e.mem.cstr(a[1]);
            Ret::default()
        });
        e.register(0x0055_9450, |_, a| ret(a[0] + 0x1000));
        e.call_log = Some(vec![]);
        (e, set)
    }

    #[test]
    fn the_normal_map_name_replaces_the_extension() {
        let (mut e, set) = engine();
        let path = e.mem.alloc(0x40);
        e.mem.set_cstr(path, b"textures/a.b/rock.dds");
        let result = e.mem.alloc(8);
        assert_eq!(e.call(0x0048_e5b0, &[result, path]).u32(), result + 0x1000);
        assert_eq!(*set.borrow(), b"textures/a.b/rock_n.dds");
        let formats: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == 0x0040_6d00)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(formats[0][1..3], [0x104, NORMAL_FORMAT]);
    }

    #[test]
    fn no_path_or_no_extension_answers_zero() {
        let (mut e, set) = engine();
        let result = e.mem.alloc(8);
        assert_eq!(e.call(0x0048_e5b0, &[result, 0]).u32(), 0);
        let path = e.mem.alloc(0x40);
        e.mem.set_cstr(path, b"textures/rock");
        assert_eq!(e.call(0x0048_e5b0, &[result, path]).u32(), 0);
        assert!(set.borrow().is_empty());
    }

    #[test]
    fn the_texture_passes_its_own_path() {
        let (mut e, set) = engine();
        let path = e.mem.alloc(0x40);
        e.mem.set_cstr(path, b"x.dds");
        e.register_double(0x0040_8da0, move |_, _| ret(path));
        let result = e.mem.alloc(8);
        assert_eq!(
            e.call(0x0048_e580, &[0x5000, result]).u32(),
            result + 0x1000
        );
        assert_eq!(*set.borrow(), b"x_n.dds");
    }
}

#[cfg(test)]
mod n045_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A texture is a length word at +8 and a text pointer at +4 here.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(PATH_LENGTH, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(PATH_TEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ADD_CHUNK_ARRAY, |_, _| Ret::default());
        e.register(STRING_SET, |_, _| Ret::default());
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(STRING_COMPARE, |e, a| {
            u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])).into_ret()
        });
        e
    }

    fn texture(e: &mut Engine, text: &[u8]) -> u32 {
        let t = e.mem.alloc(0x10);
        let s = e.mem.alloc(0x20);
        e.mem.set_cstr(s, text);
        e.mem.set_u32(t + 4, s);
        e.mem.set_u32(t + 8, text.len() as u32);
        t
    }

    #[test]
    fn save_adds_the_path_with_its_terminator() {
        let mut e = engine();
        let empty = texture(&mut e, b"");
        let t = texture(&mut e, b"a.dds");
        e.call_log = Some(vec![]);
        e.call(0x0048_e370, &[empty, 0x4e4f_4349]);
        e.call(0x0048_e370, &[t, 0x4e4f_4349]);
        let log = e.call_log.take().unwrap();
        let text = e.mem.u32(t + 4);
        assert_eq!(
            only(&log, ADD_CHUNK_ARRAY),
            vec![vec![0x4e4f_4349, text, 6]]
        );
    }

    #[test]
    fn copy_sets_this_path_from_a_texture() {
        let mut e = engine();
        let t = texture(&mut e, b"x");
        let other = texture(&mut e, b"b.dds");
        e.call_log = Some(vec![]);
        e.call(0x0048_e4a0, &[t, 0]);
        e.call(0x0048_e4a0, &[t, other]);
        let log = e.call_log.take().unwrap();
        let text = e.mem.u32(other + 4);
        assert_eq!(only(&log, STRING_SET), vec![vec![t + 4, text, 0]]);
    }

    #[test]
    fn compare_checks_cast_lengths_and_text() {
        let mut e = engine();
        let a = texture(&mut e, b"abc");
        let b = texture(&mut e, b"abc");
        let c = texture(&mut e, b"abd");
        let d = texture(&mut e, b"ab");
        let empty1 = texture(&mut e, b"");
        let empty2 = texture(&mut e, b"");
        assert!(e.call(0x0048_e4f0, &[a, 0]).bool());
        assert!(!e.call(0x0048_e4f0, &[empty1, empty2]).bool());
        assert!(!e.call(0x0048_e4f0, &[a, b]).bool());
        assert!(e.call(0x0048_e4f0, &[a, c]).bool());
        assert!(e.call(0x0048_e4f0, &[a, d]).bool());
        assert!(e.call(0x0048_e4f0, &[empty1, d]).bool());
    }
}
