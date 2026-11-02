//! `BSHavok/bhkshapephantom.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkShapePhantom` (vtable `010306b4`, RTTI-confirmed) is a `bhkPhantom` whose Havok object
//! (`+8`) keeps a 0x40-byte transform (four 16-byte vectors) at its `+0xb0`. The game's own
//! transform (a 3x3 matrix and a translation vector, 0x30 bytes) is built from it by `00624b10`
//! and `00458620`.
//!
//! The functions marked `Translation incomplete` carry an `FS:[0]` exception-unwinding frame or
//! a `/GS` stack-cookie check (`00ec408c`), neither of which is modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkSerializable::Kill` (`00c876a0`) and `~bhkPhantom` (`00c9f3b0`), engine map names.
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const PHANTOM_DESTRUCT: u32 = 0x00c9_f3b0;
/// `bhkPhantom::GetViewerStrings` (`00c9f480`) and `bhkWorldObject::CopyCinfo` (`00c85880`),
/// engine map names.
const PHANTOM_GET_VIEWER_STRINGS: u32 = 0x00c9_f480;
const WORLD_OBJECT_COPY_CINFO: u32 = 0x00c8_5880;
/// The empty lock call (`00a29680`) the PC build kept around the Havok accesses.
const EMPTY_LOCK: u32 = 0x00a2_9680;
/// The conversion of a 0x40-byte Havok transform to a 3x3 matrix (`00624b10`, cdecl:
/// destination, source; unnamed), the 3-float vector copy (`00458620`, cdecl: destination,
/// source; unnamed) and the 16-byte vector copy (`004b4db0`, thiscall on the destination, one
/// stack word: the source; unnamed).
const MATRIX_FROM_TRANSFORM: u32 = 0x0062_4b10;
const COPY_VECTOR3: u32 = 0x0045_8620;
const COPY_VECTOR: u32 = 0x004b_4db0;
/// `NiGetViewerString` (`00a7d330`, cdecl: name), `NiMatrix3::GetViewerString` (`00a598f0`) and
/// `NiPoint3::GetViewerString` (`00a59af0`) (thiscall on the matrix or point, one stack word:
/// the label), and the string array's resize (`0096ad30`) and store (`0096ae90`).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const MATRIX_GET_VIEWER_STRING: u32 = 0x00a5_98f0;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The vtable `~bhkShapePhantom` stores and the object counter it lowers.
const SHAPE_PHANTOM_VTABLE: u32 = 0x0103_06b4;
const SHAPE_PHANTOM_COUNT: u32 = 0x0126_8208;
/// The class name word (`01268224`) `GetViewerStrings` passes to `NiGetViewerString`, and the
/// labels (`"Rot"` at `010c604c`, `"Trans"` at `010c6050`) of the matrix and the translation.
const CLASS_RTTI: u32 = 0x0126_8224;
const ROTATION_LABEL: u32 = 0x010c_604c;
const TRANSLATION_LABEL: u32 = 0x010c_6050;
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

/// The transform in the Havok object: its offset and size (four 16-byte vectors).
const HAVOK_TRANSFORM: u32 = 0xb0;
const TRANSFORM_SIZE: u32 = 0x40;
/// The game's transform (a 3x3 matrix of 0x24 bytes, then the translation) and its size.
const MATRIX_SIZE: u32 = 0x24;
const GAME_TRANSFORM_SIZE: u32 = 0x30;

/// Sets the per-thread word `TLS + 0x2b4` to 0x10, answering the thread block and the old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    (thread, saved)
}

fn leave_scope(e: &mut Engine, scope: (u32, u32)) {
    e.mem.set_u32(scope.0 + SCOPE_WORD, scope.1);
}

/// Stores the string pointer `text` in the string array: grows the array by its grow-by
/// (`0096ad30`) when the size (`+0xa`) has reached the capacity (`+8`), then stores through
/// `0096ae90` (array, index = the old size, address of the pointer).
fn store_text(e: &mut Engine, array: u32, text: u32) {
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![array, grow_by + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRING_ARRAY_SET_AT, &args![array, size, slot]);
    });
}

// Translated from 00c9eb30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9eb30 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkShapePhantom::~bhkShapePhantom` (engine map name): stores the vtable, runs `Kill`
/// (`00c876a0`), lowers the object counter (`01268208`) and runs `~bhkPhantom` (`00c9f3b0`).
pub fn bhk_shape_phantom_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SHAPE_PHANTOM_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(SHAPE_PHANTOM_COUNT);
    e.mem.set_u32(SHAPE_PHANTOM_COUNT, count.wrapping_sub(1));
    e.call(PHANTOM_DESTRUCT, &args![this]);
}

// Translated from 00c9ec00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `bhkshapephantom.obj`). `(out)`: if `this` and its Havok
/// object (`+8`) are set, copies the Havok object's four 16-byte vectors at `+0xb0` to `out`,
/// between two empty-lock calls (`00a29680`).
pub fn fn_00c9ec00(e: &mut Engine, this: Ptr, out: Ptr) {
    if this.addr() == 0 {
        return;
    }
    let object = e.mem.u32(this.addr() + 8);
    if object == 0 {
        return;
    }
    e.call(EMPTY_LOCK, &args![this]);
    for word in 0..TRANSFORM_SIZE / 4 {
        let value = e.mem.u32(object + HAVOK_TRANSFORM + 4 * word);
        e.mem.set_u32(out.addr() + 4 * word, value);
    }
    e.call(EMPTY_LOCK, &args![this]);
}

// Translated from 00c9ec60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9ec60 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `bhkShapePhantom::GetTransform` (engine map name), `(out)`: takes the Havok transform
/// (`00c9ec00`) and builds the game's transform in `out`: the matrix from it (`00624b10`, into
/// `out`) and the translation from its fourth vector (`00458620`, into `out + 0x24`).
pub fn bhk_shape_phantom_get_transform(e: &mut Engine, this: Ptr, out: Ptr) {
    e.with_stack(TRANSFORM_SIZE, |e, transform| {
        fn_00c9ec00(e, this, transform);
        e.call(MATRIX_FROM_TRANSFORM, &args![out, transform]);
        e.call(
            COPY_VECTOR3,
            &args![out.addr() + MATRIX_SIZE, transform.addr() + 0x30],
        );
    });
}

// Translated from 00c9ecb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9ecb0 - the /GS stack-cookie check (00ec408c) is not modeled.
/// No map name (the map places it in `bhkshapephantom.obj`). `(cinfo)`: runs
/// `bhkWorldObject::CopyCinfo` (`00c85880`) with the Cinfo, takes the Havok transform
/// (`00c9ec00`) and copies its four vectors (`004b4db0`) to the Cinfo's `+0x20`, `+0x30`,
/// `+0x40` and `+0x50`.
pub fn fn_00c9ecb0(e: &mut Engine, this: Ptr, cinfo: Ptr) {
    e.call(WORLD_OBJECT_COPY_CINFO, &args![this, cinfo]);
    e.with_stack(TRANSFORM_SIZE, |e, transform| {
        fn_00c9ec00(e, this, transform);
        for vector in 0..4 {
            e.call(
                COPY_VECTOR,
                &args![
                    cinfo.addr() + 0x20 + 0x10 * vector,
                    transform.addr() + 0x10 * vector
                ],
            );
        }
    });
}

// Translated from 00c9ed30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9ed30 - the FS:[0] exception-unwinding frame and the /GS stack-cookie check (00ec408c) are not modeled.
/// `bhkShapePhantom::GetViewerStrings` (engine map name), `(strings)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, the phantom's strings (`00c9f480`), the text `NiGetViewerString`
/// makes of the word at `01268224`, then, for the game's transform built as `GetTransform`
/// does, the text `NiMatrix3::GetViewerString` makes of the matrix (label `"Rot"`) and the text
/// `NiPoint3::GetViewerString` makes of the translation (label `"Trans"`), each stored in the
/// string array (grown by `0096ad30` when its size, `+0xa`, has reached its capacity, `+8`).
pub fn bhk_shape_phantom_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let scope = enter_scope(e);
    e.call(PHANTOM_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(CLASS_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![name]).u32();
    store_text(e, strings.addr(), text);
    e.with_stack(TRANSFORM_SIZE, |e, transform| {
        e.with_stack(GAME_TRANSFORM_SIZE, |e, game| {
            fn_00c9ec00(e, this, transform);
            e.call(MATRIX_FROM_TRANSFORM, &args![game, transform]);
            e.call(
                COPY_VECTOR3,
                &args![game.addr() + MATRIX_SIZE, transform.addr() + 0x30],
            );
            let text = e
                .call(MATRIX_GET_VIEWER_STRING, &args![game, ROTATION_LABEL])
                .u32();
            store_text(e, strings.addr(), text);
            let text = e
                .call(
                    POINT_GET_VIEWER_STRING,
                    &args![game.addr() + MATRIX_SIZE, TRANSLATION_LABEL],
                )
                .u32();
            store_text(e, strings.addr(), text);
        });
    });
    leave_scope(e, scope);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9eb30, bhk_shape_phantom_destructor(Ptr)),
        entry!(0x00c9ec00, fn_00c9ec00(Ptr, Ptr)),
        entry!(0x00c9ec60, bhk_shape_phantom_get_transform(Ptr, Ptr)),
        entry!(0x00c9ecb0, fn_00c9ecb0(Ptr, Ptr)),
        entry!(0x00c9ed30, bhk_shape_phantom_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn order_of(e: &Engine, addresses: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| addresses.contains(a))
            .collect()
    }

    /// A phantom whose Havok object holds the words `1..=16` at `+0xb0`.
    fn phantom(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0x100);
        for word in 0..16u32 {
            e.mem.set_u32(object + HAVOK_TRANSFORM + 4 * word, word + 1);
        }
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, object);
        this
    }

    /// Doubles for the matrix conversion (copies the first nine words) and the vector copies.
    fn conversion_doubles(e: &mut Engine) {
        e.register(MATRIX_FROM_TRANSFORM, |e, a| {
            for word in 0..9 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        e.register(COPY_VECTOR3, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        e.register(EMPTY_LOCK, |_, _| Ret::default());
    }

    #[test]
    fn bhk_shape_phantom_destructor_runs_kill_lowers_the_counter_and_runs_the_phantom() {
        let mut e = logged_engine();
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(PHANTOM_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(SHAPE_PHANTOM_COUNT, 2);
        e.call(0x00c9_eb30, &args![this]);
        assert_eq!(e.mem.u32(this), SHAPE_PHANTOM_VTABLE);
        assert_eq!(e.mem.u32(SHAPE_PHANTOM_COUNT), 1);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, PHANTOM_DESTRUCT), vec![vec![this]]);
        assert_eq!(
            order_of(&e, &[SERIALIZABLE_KILL, PHANTOM_DESTRUCT]),
            vec![SERIALIZABLE_KILL, PHANTOM_DESTRUCT]
        );
    }

    #[test]
    fn fn_00c9ec00_copies_the_transform_between_two_locks() {
        let mut e = logged_engine();
        e.register(EMPTY_LOCK, |_, _| Ret::default());
        let this = phantom(&mut e);
        let out = e.mem.alloc(0x50);
        e.mem.set_u32(out + 0x40, 0xaa);
        e.call(0x00c9_ec00, &args![this, out]);
        let copied: Vec<u32> = (0..16).map(|word| e.mem.u32(out + 4 * word)).collect();
        assert_eq!(copied, (1..=16).collect::<Vec<u32>>());
        assert_eq!(e.mem.u32(out + 0x40), 0xaa);
        assert_eq!(calls_to(&e, EMPTY_LOCK), vec![vec![this], vec![this]]);
    }

    #[test]
    fn fn_00c9ec00_does_nothing_without_a_phantom_or_a_havok_object() {
        let mut e = logged_engine();
        e.register(EMPTY_LOCK, |_, _| Ret::default());
        let out = e.mem.alloc(0x40);
        let bare = e.mem.alloc(0x20);
        e.call(0x00c9_ec00, &args![0u32, out]);
        e.call(0x00c9_ec00, &args![bare, out]);
        assert!(calls_to(&e, EMPTY_LOCK).is_empty());
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn bhk_shape_phantom_get_transform_builds_the_matrix_and_the_translation() {
        let mut e = logged_engine();
        conversion_doubles(&mut e);
        let this = phantom(&mut e);
        let out = e.mem.alloc(0x30);
        e.call(0x00c9_ec60, &args![this, out]);
        let words: Vec<u32> = (0..12).map(|word| e.mem.u32(out + 4 * word)).collect();
        assert_eq!(words, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 13, 14, 15]);
        let matrix = calls_to(&e, MATRIX_FROM_TRANSFORM);
        assert_eq!(matrix.len(), 1);
        assert_eq!(matrix[0][0], out);
        assert_eq!(
            calls_to(&e, COPY_VECTOR3),
            vec![vec![out + 0x24, matrix[0][1] + 0x30]]
        );
    }

    #[test]
    fn fn_00c9ecb0_copies_the_cinfo_then_the_four_transform_vectors() {
        let mut e = logged_engine();
        conversion_doubles(&mut e);
        e.register(WORLD_OBJECT_COPY_CINFO, |_, _| Ret::default());
        e.register(COPY_VECTOR, |e, a| {
            for word in 0..4 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        let this = phantom(&mut e);
        let cinfo = e.mem.alloc(0x70);
        e.call(0x00c9_ecb0, &args![this, cinfo]);
        assert_eq!(
            calls_to(&e, WORLD_OBJECT_COPY_CINFO),
            vec![vec![this, cinfo]]
        );
        let copied: Vec<u32> = (0..16)
            .map(|word| e.mem.u32(cinfo + 0x20 + 4 * word))
            .collect();
        assert_eq!(copied, (1..=16).collect::<Vec<u32>>());
        let destinations: Vec<u32> = calls_to(&e, COPY_VECTOR).iter().map(|c| c[0]).collect();
        assert_eq!(
            destinations,
            vec![cinfo + 0x20, cinfo + 0x30, cinfo + 0x40, cinfo + 0x50]
        );
        assert_eq!(
            order_of(&e, &[WORLD_OBJECT_COPY_CINFO, COPY_VECTOR])[0],
            WORLD_OBJECT_COPY_CINFO
        );
    }

    #[test]
    fn bhk_shape_phantom_get_viewer_strings_stores_the_class_name_matrix_and_translation() {
        let mut e = logged_engine();
        conversion_doubles(&mut e);
        e.register(PHANTOM_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x1111u32.into_ret());
        e.register(MATRIX_GET_VIEWER_STRING, |_, _| 0x2222u32.into_ret());
        e.register(POINT_GET_VIEWER_STRING, |_, _| 0x3333u32.into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, (a[1] + 1) as u16);
            Ret::default()
        });
        e.mem.set_u32(CLASS_RTTI, 0x4242);
        let this = phantom(&mut e);
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u16(array + 8, 2);
        e.mem.set_u16(array + 0xe, 4);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00c9_ed30, &args![this, array]);
        assert_eq!(
            calls_to(&e, PHANTOM_GET_VIEWER_STRINGS),
            vec![vec![this, array]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x4242]]);
        let matrix = calls_to(&e, MATRIX_GET_VIEWER_STRING);
        assert_eq!(matrix, vec![vec![matrix[0][0], ROTATION_LABEL]]);
        assert_eq!(
            calls_to(&e, POINT_GET_VIEWER_STRING),
            vec![vec![matrix[0][0] + 0x24, TRANSLATION_LABEL]]
        );
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![array, 6]]);
        let stored: Vec<u32> = (0..3).map(|n| e.mem.u32(base + 4 * n)).collect();
        assert_eq!(stored, vec![0x1111, 0x2222, 0x3333]);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }
}
