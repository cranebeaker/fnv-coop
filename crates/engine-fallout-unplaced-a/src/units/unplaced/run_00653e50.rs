//! `(unplaced)/run_00653e50` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00633c90` (thiscall, object): a smart-pointer constructor (stores the
/// object and adds a reference when it is not null).
const SMART_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `0057c5d0` (thiscall: out smart pointer, smart pointer; unit
/// `tesobjectrefr.cpp` in the engine map).
const CALLEE_0057C5D0: u32 = 0x0057_c5d0;
/// `0045cec0` (thiscall): the smart-pointer destructor (releases a
/// non-null object).
const SMART_POINTER_DESTRUCT: u32 = 0x0045_cec0;

// Translated from 00654640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: wraps `object` in a smart pointer on the
/// stack (`00633c90`), passes it with a second, out smart pointer to
/// `0057c5d0` on the member at `this + 0x24`, then destroys the out pointer
/// and the wrapper (`0045cec0`).
// Translation incomplete: 00654640 - SEH exception unwinding (the smart pointers' destructors on unwind) is unsupported.
pub fn fn_00654640(e: &mut Engine, this: Ptr, object: u32) {
    // Locals: the wrapper at +0, the out pointer at +4.
    e.with_stack(8, |e, locals| {
        let wrapper = locals.addr();
        let out = wrapper + 4;
        e.call(SMART_POINTER_CONSTRUCT, &args![wrapper, object]);
        e.call(CALLEE_0057C5D0, &args![this.byte_add(0x24), out, wrapper]);
        e.call(SMART_POINTER_DESTRUCT, &args![out]);
        e.call(SMART_POINTER_DESTRUCT, &args![wrapper]);
    });
}

/// The callees of [`fn_00653e50`] (thiscall unless noted): the node's child count
/// (`0043b480`) and child (`0043b4a0`, index), the child's name (`0043b1b0` on what
/// `00413f40` answers), the cdecl string compare `00408b20` (0 when equal), the child's
/// model (its slot +0x1C) and the cdecl lookup `006530e0` on it, `006505d0` on what that
/// answers, the cdecl base-name builder `00653620` (string, word), the string constructor
/// (`004037b0`) and destructor (`004037d0`), the eh vector constructor (`00ec782f`,
/// stdcall: array, size, count, constructor `00449610`, destructor `00449680`) and
/// destructor (`00ec5fce`, stdcall: array, size, count, destructor),
/// `BSFaceGenManager::GetFaceGenCoord` (`006521e0`, no arguments) and
/// `BSFaceGenManager::CopyFaceGenCoord` (`006529a0`, cdecl: coordinates, out, 0, 0),
/// `BSFaceGenManager::GetAsNIFFileLowLOD` (`00653a10`) and the other LOD's name
/// (`00653c40`; both cdecl: out string, base name), the word an object holds (`00559450`:
/// a string's text, a pointer's object) and the trivial constructor `006815c0` (answers
/// `this`) run on two path buffers.
const CHILD_COUNT: u32 = 0x0043_b480;
const CHILD_AT: u32 = 0x0043_b4a0;
const CHILD_NAME_OWNER: u32 = 0x0041_3f40;
const NAME_OF: u32 = 0x0043_b1b0;
const STRING_COMPARE: u32 = 0x0040_8b20;
const SLOT_MODEL: u32 = 0x1c;
const MODEL_LOOKUP_006530E0: u32 = 0x0065_30e0;
const BASE_NAME_SOURCE_006505D0: u32 = 0x0065_05d0;
const BASE_NAME: u32 = 0x0065_3620;
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
const COORD_ELEMENT_CONSTRUCT: u32 = 0x0044_9610;
const COORD_ELEMENT_DESTRUCT: u32 = 0x0044_9680;
const GET_FACE_GEN_COORD: u32 = 0x0065_21e0;
const COPY_FACE_GEN_COORD: u32 = 0x0065_29a0;
const LOW_LOD_NIF_NAME: u32 = 0x0065_3a10;
const HIGH_LOD_NIF_NAME: u32 = 0x0065_3c40;
const HELD_WORD: u32 = 0x0055_9450;
const TRIVIAL_CONSTRUCT_006815C0: u32 = 0x0068_15c0;
/// The archive side: the cdecl path split (`00afd270`: path, out folder, out file) and
/// find (`00af6540`: 0, folder, file, path), the cdecl file names `00653520`
/// (`BSFaceGenManager::GetAsEGMFile`), `006536d0` (`BSFaceGenManager::GetAsTRIFile`) and
/// `006537d0`, the cdecl model loader `00651b50` (seven words), and the model's geometry
/// clone (`0065a910`: coordinates, out pointer; `AL`).
const SPLIT_PATH: u32 = 0x00af_d270;
const ARCHIVE_FIND: u32 = 0x00af_6540;
const EGM_FILE_NAME: u32 = 0x0065_3520;
const TRI_FILE_NAME: u32 = 0x0065_36d0;
const NAME_006537D0: u32 = 0x0065_37d0;
const FACE_GEN_MODEL: u32 = 0x0065_1b50;
const MODEL_CLONE_GEOMETRY: u32 = 0x0065_a910;
/// The new geometry's set-up: the fixed string constructor (`00438170`: text) and
/// destructor (`004381b0`), the name setter (`00a5b950`), the child's translate
/// (`0043c490`), rotate (`006a9540`) and scale (`009cdae0`, ST0) getters and the
/// geometry's setters (`00440460`, `0043fa80`, `00440490`), `BSShaderManager::PrepareObject`
/// (`00b57e30`, cdecl: object, 0, 0), `NiAVObject::GetProperty` (`00a59d30`: kind),
/// the cloning process's constructor (`004ad0c0`: 0x101) and destructor (`004ad1d0`),
/// `NiObject::Clone` (`00a5d2c0`: the process), `00539f20` (the shader property's word),
/// [`fn_00654640`], `NiAVObject::AttachProperty` (`00439410`), `00874460` (the child's
/// `004030b0` word), the geometry's vertex count (`004a8090`), its data (`005495f0`),
/// the data's tests and fill (`00a67360` with 1, `00a67410` with the iterator), the
/// iterator's constructor (`00534550`), the data's bound (`00460140`) and the bound
/// builder `00656dc0` (cdecl: bound, iterator, count).
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
const SET_NAME: u32 = 0x00a5_b950;
const GET_TRANSLATE: u32 = 0x0043_c490;
const SET_TRANSLATE: u32 = 0x0044_0460;
const GET_ROTATE: u32 = 0x006a_9540;
const SET_ROTATE: u32 = 0x0043_fa80;
const GET_SCALE: u32 = 0x009c_dae0;
const SET_SCALE: u32 = 0x0044_0490;
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
const GET_PROPERTY: u32 = 0x00a5_9d30;
const CLONING_PROCESS_CONSTRUCT: u32 = 0x004a_d0c0;
const CLONING_PROCESS_DESTRUCT: u32 = 0x004a_d1d0;
const CLONE: u32 = 0x00a5_d2c0;
const SET_SHADER_WORD_00539F20: u32 = 0x0053_9f20;
const SHADER_WORD_006546E0: u32 = 0x0065_46e0;
const ATTACH_PROPERTY: u32 = 0x0043_9410;
const CHILD_WORD_004030B0: u32 = 0x0040_30b0;
const SET_WORD_00874460: u32 = 0x0087_4460;
const VERTEX_COUNT: u32 = 0x004a_8090;
const GEOMETRY_DATA: u32 = 0x0054_95f0;
const DATA_TEST_00A67360: u32 = 0x00a6_7360;
const ITERATOR_CONSTRUCT: u32 = 0x0053_4550;
const DATA_FILL_00A67410: u32 = 0x00a6_7410;
const DATA_BOUND: u32 = 0x0046_0140;
const BUILD_BOUND: u32 = 0x0065_6dc0;
/// The finishing calls: the alpha property's value (`009a9350`, ST0) compared with the
/// `double` 1.0 (`01012070`) and its setter `00654700` with the `float` at `0101e570`, the
/// node's slots +0xF8 (index, geometry) and +0x128 (`009611e0`'s word, 0),
/// `NiAVObject::UpdateProperties` (`00a5a040`) and the node's LOD flag setter `006546c0`.
const ALPHA_VALUE: u32 = 0x009a_9350;
const DOUBLE_ONE: u32 = 0x0101_2070;
const SET_ALPHA_00654700: u32 = 0x0065_4700;
const ALPHA_LIMIT: u32 = 0x0101_e570;
const SLOT_SET_CHILD: u32 = 0xf8;
const SLOT_0128: u32 = 0x128;
const NODE_WORD_009611E0: u32 = 0x0096_11e0;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const SET_LOD_FLAG: u32 = 0x0065_46c0;
/// The child name the function looks for (`0104b3b4`, `"FaceGenFace"`).
const FACE_GEN_FACE: u32 = 0x0104_b3b4;

/// The locals of one child of [`fn_00653e50`] at their frame offsets (EBP - k).
struct FaceLocals {
    geometry_pointer: u32,
    strings: [u32; 6],
    coordinates: u32,
}

/// Destroys the per-child locals in the exe's order (the coordinates, the six strings
/// from the last made, the geometry pointer).
fn destroy_face_locals(e: &mut Engine, locals: &FaceLocals) {
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &args![locals.coordinates, 0x20u32, 4u32, COORD_ELEMENT_DESTRUCT],
    );
    for string in locals.strings.iter().rev() {
        e.call(STRING_DESTRUCT, &args![*string]);
    }
    e.call(SMART_POINTER_DESTRUCT, &args![locals.geometry_pointer]);
}

// Translated from 00653e50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00653e50 - the compiler's exception frame (FS:[0] unwind states 0..10) and the security-cookie word it pushes are not translated
/// Unnamed in the map (cdecl: the node, the LOD kind; called from `BSFaceGenNiNode` code at
/// `00663450`): swaps each `"FaceGenFace"` child of `node` for the other LOD's geometry. For
/// such a child the model's base name (`006530e0`, `006505d0`, `00653620`) gives the NIF
/// name of the kind (0: `GetAsNIFFileLowLOD`, 1: `00653c40`); without one, or when the
/// archive does not have it (`00afd270`, `00af6540`), or the model (`00651b50` with the
/// EGM, TRI and `006537d0` names) yields no geometry (`0065a910`), the function returns.
/// The new geometry is named `"FaceGenFace"`, takes the child's transform, is prepared
/// (`BSShaderManager::PrepareObject`), gets a clone of the child's property 3 (with the
/// shader word of its own, `00539f20`, [`fn_00654640`], `NiAVObject::AttachProperty`) and
/// the child's `004030b0` word, a bound from its vertices when its data allows
/// (`00a67360`, `00a67410`, `00656dc0`), its alpha raised to `0101e570` when below 1.0,
/// replaces the child (slot +0xF8), and the node's properties and LOD flag (1 for kind 0,
/// 0 for kind 1) are updated.
pub fn fn_00653e50(e: &mut Engine, node: u32, kind: u32) {
    // The frame from EBP - 0x140; `at(k)` is EBP - k.
    e.with_stack(0x140, |e, frame| {
        let at = |k: u32| frame.addr() + 0x140 - k;
        let mut index = 0u32;
        while index < e.call(CHILD_COUNT, &args![node]).u32() {
            let child = e.call(CHILD_AT, &args![node, index]).u32();
            index += 1;
            if child == 0 {
                continue;
            }
            let owner = e.call(CHILD_NAME_OWNER, &args![child]).u32();
            let name = e.call(NAME_OF, &args![owner]).u32();
            if e.call(STRING_COMPARE, &args![name, FACE_GEN_FACE]).u32() != 0 {
                continue;
            }
            let child_index = index - 1;
            let model = e.vcall(child, SLOT_MODEL, &[]).u32();
            let looked_up = e.call(MODEL_LOOKUP_006530E0, &args![model]).u32();
            let locals = FaceLocals {
                geometry_pointer: at(0x11c),
                strings: [at(0x118), at(0x110), at(0x24), at(0xbc), at(0xd8), at(0xec)],
                coordinates: at(0xb4),
            };
            e.call(
                SMART_POINTER_CONSTRUCT,
                &args![locals.geometry_pointer, 0u32],
            );
            e.mem.set_u32(at(0xdc), 0);
            for string in locals.strings {
                e.call(STRING_CONSTRUCT, &args![string]);
            }
            e.call(
                VECTOR_CONSTRUCTOR_ITERATOR,
                &args![
                    locals.coordinates,
                    0x20u32,
                    4u32,
                    COORD_ELEMENT_CONSTRUCT,
                    COORD_ELEMENT_DESTRUCT
                ],
            );
            let coordinates = e.call(GET_FACE_GEN_COORD, &[]).u32();
            e.call(
                COPY_FACE_GEN_COORD,
                &args![coordinates, locals.coordinates, 0u32, 0u32],
            );
            let source = e.call(BASE_NAME_SOURCE_006505D0, &args![looked_up]).u32();
            let base_name = at(0x118);
            e.call(BASE_NAME, &args![base_name, source]);
            let nif = at(0x110);
            match kind {
                0 => {
                    let text = e.call(HELD_WORD, &args![base_name]).u32();
                    e.call(LOW_LOD_NIF_NAME, &args![nif, text]);
                }
                1 => {
                    let text = e.call(HELD_WORD, &args![base_name]).u32();
                    e.call(HIGH_LOD_NIF_NAME, &args![nif, text]);
                }
                _ => {}
            }
            if e.call(HELD_WORD, &args![nif]).u32() == 0 {
                destroy_face_locals(e, &locals);
                return;
            }
            let (folder, file) = (at(0xd0), at(0x30));
            e.call(TRIVIAL_CONSTRUCT_006815C0, &args![folder]);
            e.call(TRIVIAL_CONSTRUCT_006815C0, &args![file]);
            let path = e.call(HELD_WORD, &args![nif]).u32();
            e.call(SPLIT_PATH, &args![path, folder, file]);
            let path = e.call(HELD_WORD, &args![nif]).u32();
            let found = e.call(ARCHIVE_FIND, &args![0u32, folder, file, path]).u32();
            if found == 0 {
                destroy_face_locals(e, &locals);
                return;
            }
            let path = e.call(HELD_WORD, &args![nif]).u32();
            let alternate = e.call(NAME_006537D0, &args![at(0xd8), path]).u32();
            let path = e.call(HELD_WORD, &args![nif]).u32();
            let tri = e
                .call(TRI_FILE_NAME, &args![at(0xbc), path, u32::MAX])
                .u32();
            let path_again = e.call(HELD_WORD, &args![nif]).u32();
            let path = e.call(HELD_WORD, &args![nif]).u32();
            let egm = e
                .call(EGM_FILE_NAME, &args![at(0x24), path, u32::MAX])
                .u32();
            let model = e
                .call(
                    FACE_GEN_MODEL,
                    &args![egm, path_again, tri, alternate, 1u32, u32::MAX, 0u32],
                )
                .u32();
            let model_pointer = at(0x108);
            e.call(SMART_POINTER_CONSTRUCT, &args![model_pointer, model]);
            let held = e.call(HELD_WORD, &args![model_pointer]).u32();
            let cloned = e
                .call(
                    MODEL_CLONE_GEOMETRY,
                    &args![held, locals.coordinates, locals.geometry_pointer],
                )
                .u8()
                != 0;
            if !cloned || e.call(HELD_WORD, &args![locals.geometry_pointer]).u32() == 0 {
                e.call(SMART_POINTER_DESTRUCT, &args![model_pointer]);
                destroy_face_locals(e, &locals);
                return;
            }
            let fixed = at(0x12c);
            let label = e
                .call(FIXED_STRING_CONSTRUCT, &args![fixed, FACE_GEN_FACE])
                .u32();
            let geometry = e.call(HELD_WORD, &args![locals.geometry_pointer]).u32();
            e.call(SET_NAME, &args![geometry, label]);
            e.call(FIXED_STRING_DESTRUCT, &args![fixed]);
            let geometry = e.call(HELD_WORD, &args![locals.geometry_pointer]).u32();
            e.mem.set_u32(at(0xdc), geometry);
            let translate = e.call(GET_TRANSLATE, &args![child]).u32();
            e.call(SET_TRANSLATE, &args![geometry, translate]);
            let rotate = e.call(GET_ROTATE, &args![child]).u32();
            e.call(SET_ROTATE, &args![geometry, rotate]);
            let scale = x87_store32(e.call(GET_SCALE, &args![child]).st0);
            e.call(SET_SCALE, &args![geometry, scale]);
            e.call(PREPARE_OBJECT, &args![geometry, 0u32, 0u32]);
            let own = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
            let theirs = e.call(GET_PROPERTY, &args![child, 3u32]).u32();
            let process = at(0x104);
            e.call(CLONING_PROCESS_CONSTRUCT, &args![process, 0x101u32]);
            let clone = e.call(CLONE, &args![theirs, process]).u32();
            let word = e.call(SHADER_WORD_006546E0, &args![own]).u32();
            e.call(SET_SHADER_WORD_00539F20, &args![clone, word]);
            fn_00654640(e, Ptr::new(geometry), own);
            e.call(ATTACH_PROPERTY, &args![geometry, clone]);
            let word = e.call(CHILD_WORD_004030B0, &args![child]).u32();
            e.call(SET_WORD_00874460, &args![geometry, word]);
            let vertices = e.call(VERTEX_COUNT, &args![geometry]).u32() as u16;
            let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
            if e.call(DATA_TEST_00A67360, &args![data, 1u32]).u8() != 0 {
                let iterator = at(0x128);
                e.call(ITERATOR_CONSTRUCT, &args![iterator]);
                let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
                e.call(DATA_FILL_00A67410, &args![data, iterator]);
                if e.call(HELD_WORD, &args![iterator]).u32() != 0 {
                    let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
                    let bound = e.call(DATA_BOUND, &args![data]).u32();
                    e.call(BUILD_BOUND, &args![bound, iterator, u32::from(vertices)]);
                }
            }
            let alpha = e.call(GET_PROPERTY, &args![geometry, 2u32]).u32();
            if alpha != 0 {
                let value = e.call(ALPHA_VALUE, &args![alpha]).st0;
                if value < e.global::<f64>(DOUBLE_ONE) {
                    let limit = x87_quiet_bits(e.mem.u32(ALPHA_LIMIT));
                    e.call(SET_ALPHA_00654700, &args![alpha, limit]);
                }
            }
            e.vcall(node, SLOT_SET_CHILD, &args![child_index, geometry]);
            e.call(UPDATE_PROPERTIES, &args![geometry]);
            let word = e.call(NODE_WORD_009611E0, &args![node]).u32();
            e.vcall(node, SLOT_0128, &args![word, 0u32]);
            match kind {
                0 => {
                    e.call(SET_LOD_FLAG, &args![node, 1u32]);
                }
                1 => {
                    e.call(SET_LOD_FLAG, &args![node, 0u32]);
                }
                _ => {}
            }
            e.call(CLONING_PROCESS_DESTRUCT, &args![process]);
            e.call(SMART_POINTER_DESTRUCT, &args![model_pointer]);
            destroy_face_locals(e, &locals);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_4640, fn_00654640(Ptr, u32)),
        entry!(0x0065_3e50, fn_00653e50(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00654640_passes_the_wrapped_object_and_destroys_both_pointers() {
        let mut e = Engine::new();
        e.register(SMART_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(CALLEE_0057C5D0, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x7777);
            Ret::default()
        });
        e.register(SMART_POINTER_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0065_4640, &args![0x6000u32, 0x7777u32]);
        let log = e.call_log.take().unwrap();
        let wrapper = log[1].1[0];
        assert_eq!(log[1], (SMART_POINTER_CONSTRUCT, vec![wrapper, 0x7777]));
        let out = log[2].1[1];
        assert_eq!(log[2], (CALLEE_0057C5D0, vec![0x6024, out, wrapper]));
        assert_eq!(log[3], (SMART_POINTER_DESTRUCT, vec![out]));
        assert_eq!(log[4], (SMART_POINTER_DESTRUCT, vec![wrapper]));
    }
}

#[cfg(test)]
mod n076_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Every callee answers 0x7000 unless set below; strings and pointers hold a word.
    fn engine(children: &'static [u32]) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x0002_0000);
        e.set_global(DOUBLE_ONE, 1.0f64);
        e.mem.set_f32(ALPHA_LIMIT, 0.5);
        for address in [
            CHILD_NAME_OWNER,
            NAME_OF,
            MODEL_LOOKUP_006530E0,
            BASE_NAME_SOURCE_006505D0,
            BASE_NAME,
            COPY_FACE_GEN_COORD,
            GET_FACE_GEN_COORD,
            VECTOR_CONSTRUCTOR_ITERATOR,
            VECTOR_DESTRUCTOR_ITERATOR,
            STRING_DESTRUCT,
            SMART_POINTER_DESTRUCT,
            TRIVIAL_CONSTRUCT_006815C0,
            SPLIT_PATH,
            ARCHIVE_FIND,
            EGM_FILE_NAME,
            TRI_FILE_NAME,
            NAME_006537D0,
            FACE_GEN_MODEL,
            FIXED_STRING_CONSTRUCT,
            FIXED_STRING_DESTRUCT,
            SET_NAME,
            GET_TRANSLATE,
            SET_TRANSLATE,
            GET_ROTATE,
            SET_ROTATE,
            SET_SCALE,
            PREPARE_OBJECT,
            GET_PROPERTY,
            CLONING_PROCESS_CONSTRUCT,
            CLONING_PROCESS_DESTRUCT,
            CLONE,
            SET_SHADER_WORD_00539F20,
            SHADER_WORD_006546E0,
            ATTACH_PROPERTY,
            CHILD_WORD_004030B0,
            SET_WORD_00874460,
            VERTEX_COUNT,
            GEOMETRY_DATA,
            DATA_TEST_00A67360,
            ITERATOR_CONSTRUCT,
            DATA_FILL_00A67410,
            DATA_BOUND,
            BUILD_BOUND,
            SET_ALPHA_00654700,
            UPDATE_PROPERTIES,
            NODE_WORD_009611E0,
            SET_LOD_FLAG,
            CALLEE_0057C5D0,
        ] {
            e.register(address, |_, _| 0x7000u32.into_ret());
        }
        e.register(STRING_COMPARE, |_, _| Ret::default());
        e.register(GET_SCALE, |_, _| 1.0f64.into_ret());
        e.register(ALPHA_VALUE, |_, _| 0.25f64.into_ret());
        e.register(HELD_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(SMART_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        for address in [LOW_LOD_NIF_NAME, HIGH_LOD_NIF_NAME] {
            e.register(address, |e, a| {
                e.mem.set_u32(a[0], 0x5a5a);
                a[0].into_ret()
            });
        }
        e.register(MODEL_CLONE_GEOMETRY, |e, a| {
            e.mem.set_u32(a[2], 0x6600);
            1u32.into_ret()
        });
        e.register(DATA_TEST_00A67360, |_, _| 1u32.into_ret());
        e.register(DATA_FILL_00A67410, |e, a| {
            e.mem.set_u32(a[1], 0x8000);
            Ret::default()
        });
        // The node: its vtable slots +0xF8 and +0x128 are recorded.
        let mut table = vec![0u32; 0x4c];
        table[0xf8 / 4] = 0x0155_00f8;
        table[0x128 / 4] = 0x0155_0128;
        e.put_vtable(0x0155_0000, &table);
        e.register(0x0155_00f8, |_, _| Ret::default());
        e.register(0x0155_0128, |_, _| Ret::default());
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, 0x0155_0000);
        // The children's model slot +0x1C.
        let mut child_table = vec![0u32; 8];
        child_table[7] = 0x0156_001c;
        e.put_vtable(0x0156_0000, &child_table);
        e.register(0x0156_001c, |_, _| 0x4400u32.into_ret());
        let made: Vec<u32> = children
            .iter()
            .map(|c| {
                if *c == 0 {
                    0
                } else {
                    let child = e.mem.alloc(8);
                    e.mem.set_u32(child, 0x0156_0000);
                    child
                }
            })
            .collect();
        let count = made.len() as u32;
        e.register_double(CHILD_COUNT, move |_, _| count.into_ret());
        e.register_double(CHILD_AT, move |_, a| made[a[1] as usize].into_ret());
        e.call_log = Some(vec![]);
        (e, node)
    }

    #[test]
    fn swaps_each_face_child_for_the_other_lod() {
        let (mut e, node) = engine(&[0, 1]);
        e.call(0x0065_3e50, &args![node, 0u32]);
        assert_eq!(calls_to(&e, LOW_LOD_NIF_NAME).len(), 1);
        assert!(calls_to(&e, HIGH_LOD_NIF_NAME).is_empty());
        let set = calls_to(&e, 0x0155_00f8);
        assert_eq!(set, vec![vec![node, 1, 0x6600]]);
        assert_eq!(calls_to(&e, SET_NAME), vec![vec![0x6600, 0x7000]]);
        assert_eq!(calls_to(&e, SET_LOD_FLAG), vec![vec![node, 1]]);
        assert_eq!(calls_to(&e, SET_SCALE)[0][1], 1.0f32.to_bits());
        assert_eq!(
            calls_to(&e, SET_ALPHA_00654700),
            vec![vec![0x7000, 0.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x0155_0128), vec![vec![node, 0x7000, 0]]);
        let model = &calls_to(&e, FACE_GEN_MODEL)[0];
        assert_eq!(model[4..], [1, u32::MAX, 0]);
        assert_eq!(calls_to(&e, BUILD_BOUND)[0][2], 0x7000 & 0xffff);
        assert_eq!(calls_to(&e, STRING_DESTRUCT).len(), 6);
    }

    #[test]
    fn missing_names_or_files_end_the_swap() {
        let (mut e, node) = engine(&[1, 1]);
        e.call(0x0065_3e50, &args![node, 2u32]);
        assert!(calls_to(&e, SPLIT_PATH).is_empty());
        assert_eq!(calls_to(&e, CHILD_AT).len(), 1);
        assert_eq!(calls_to(&e, STRING_DESTRUCT).len(), 6);
        let (mut e, node) = engine(&[1]);
        e.register(ARCHIVE_FIND, |_, _| Ret::default());
        e.call(0x0065_3e50, &args![node, 1u32]);
        assert_eq!(calls_to(&e, HIGH_LOD_NIF_NAME).len(), 1);
        assert!(calls_to(&e, FACE_GEN_MODEL).is_empty());
        let (mut e, node) = engine(&[1]);
        e.register(MODEL_CLONE_GEOMETRY, |_, _| Ret::default());
        e.call(0x0065_3e50, &args![node, 1u32]);
        assert!(calls_to(&e, SET_NAME).is_empty());
        assert_eq!(calls_to(&e, SMART_POINTER_DESTRUCT).len(), 2);
    }

    #[test]
    fn other_children_are_left_alone() {
        let (mut e, node) = engine(&[1]);
        e.register(STRING_COMPARE, |_, _| 1u32.into_ret());
        e.call(0x0065_3e50, &args![node, 0u32]);
        assert!(calls_to(&e, 0x0156_001c).is_empty());
        assert!(calls_to(&e, STRING_CONSTRUCT).is_empty());
    }
}
