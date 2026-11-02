//! `(unplaced)/run_006546e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_006546e0` calls on the member (`00559450`).
const CALLEE_00559450: u32 = 0x0055_9450;

// Translated from 006546e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00559450` with `ECX` = `this + 0xd0` and passes on its result.
pub fn fn_006546e0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_00559450, &args![this.byte_add(0xd0)]).u32()
}

/// The `NiPointer` temporary's constructor (`00633c90`; `thiscall`: object) and destructor
/// (`0045cec0`; `thiscall`).
const POINTER_CONSTRUCTOR: u32 = 0x0063_3c90;
const POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
/// The global object `00655270` reads (`011d59e8`) and the offset of its member there.
const FACE_GEN_GLOBAL: u32 = 0x011d_59e8;
/// The member the global object keeps at `+0x1198`.
const GLOBAL_MEMBER: u32 = 0x1198;
/// The getter `00656d80` forwards to (`007d6bb0`; `thiscall` on the word at `+0x24`).
const MEMBER_GETTER: u32 = 0x007d_6bb0;

// Translated from 00656fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): copies the three words the pointer at
/// `this + 0` points to into `output`.
pub fn fn_00656fd0(e: &mut Engine, this: Ptr, output: Ptr) {
    let source = e.mem.u32(this.addr());
    for index in 0..3 {
        let word = e.mem.u32(source.wrapping_add(4 * index));
        e.mem.set_u32(output.addr() + 4 * index, word);
    }
}

// Translated from 00655270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00655270 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// Unnamed in the engine map (plain `RET`, no arguments): `00559450` on the member at
/// `+0x1198` of the object at `011d59e8`, or, without that object, on a null `NiPointer`
/// temporary (`00633c90`, destroyed afterwards by `0045cec0`). Answers what `00559450`
/// answers.
pub fn fn_00655270(e: &mut Engine) -> u32 {
    let global = e.global::<u32>(FACE_GEN_GLOBAL);
    if global != 0 {
        return e
            .call(CALLEE_00559450, &args![global.wrapping_add(GLOBAL_MEMBER)])
            .u32();
    }
    e.with_stack(4, |e, temporary| {
        let holder = e.call(POINTER_CONSTRUCTOR, &args![temporary, 0u32]).u32();
        let answer = e.call(CALLEE_00559450, &args![holder]).u32();
        e.call(POINTER_DESTRUCTOR, &args![temporary]);
        answer
    })
}

// Translated from 00656d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): 0 without the word at
/// `this + 0x24`, else what `007d6bb0` answers for it.
pub fn fn_00656d80(e: &mut Engine, this: Ptr) -> u32 {
    let member = e.mem.u32(this.addr() + 0x24);
    if member == 0 {
        return 0;
    }
    e.call(MEMBER_GETTER, &args![member]).u32()
}

/// The memory context scope (`00404eb0`; `thiscall`: id, flag, source file, line) and its
/// end (`00404ee0`; `thiscall`), and `"D:\_Fallout3\...\BSFaceGenManager.cpp"`
/// (`01065720`).
const MEMORY_SCOPE_BEGIN: u32 = 0x0040_4eb0;
const MEMORY_SCOPE_END: u32 = 0x0040_4ee0;
const FACE_GEN_MANAGER_SOURCE: u32 = 0x0106_5720;
/// The 8-byte string's constructor (`004037b0`) and destructor (`004037d0`); `thiscall`.
const STRING_CONSTRUCTOR: u32 = 0x0040_37b0;
const STRING_DESTRUCTOR: u32 = 0x0040_37d0;
/// The string's formatted assignment (`00406f60`; `cdecl`: string, format, one value).
const STRING_FORMAT: u32 = 0x0040_6f60;
/// `"Meshes\%s"` (`01019710`) and `"Textures\%s"` (`010657f8`).
const MESHES_FORMAT: u32 = 0x0101_9710;
const TEXTURES_FORMAT: u32 = 0x0106_57f8;
/// `NiPointer` assignment (`0066b0d0`; `thiscall` on the pointer: the new object).
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// Answers `this` (`MOV EAX,ECX`): the exe calls it as the empty constructor of stack
/// objects and to read a list iterator's node.
const IDENTITY: u32 = 0x0068_15c0;
/// The matrix setup (`0043f850`, `NiMatrix3::MakeYRotation` in the engine map; `thiscall`:
/// the angle) and the angle it gets (the float at `01016b78`).
const MATRIX_ROTATION: u32 = 0x0043_f850;
const HEAD_ROTATION_ANGLE: u32 = 0x0101_6b78;
/// The FaceGen file name builders (`cdecl`): `00653520` (EGM: string, path, index),
/// `006536d0` (TRI: string, path, index), `006537d0` (string, path), `00653a10` (low LOD
/// NIF: string, path).
const EGM_FILE_NAME: u32 = 0x0065_3520;
const TRI_FILE_NAME: u32 = 0x0065_36d0;
const ALTERNATE_FILE_NAME: u32 = 0x0065_37d0;
const LOW_LOD_FILE_NAME: u32 = 0x0065_3a10;
/// The archive look-up pair (`cdecl`): `00afd270` (path, two outputs) and `00af6540` (0,
/// both outputs, path).
const ARCHIVE_SPLIT_PATH: u32 = 0x00af_d270;
const ARCHIVE_FIND: u32 = 0x00af_6540;
/// The FaceGen model getter (`00651b50`; `cdecl`, seven words: EGM name, mesh path, TRI
/// name, a word, 1, an index, a flag).
const FACE_GEN_MODEL: u32 = 0x0065_1b50;
/// The model's clone into a geometry pointer (`0065a910`; `thiscall` on the model: the
/// FaceGen data, address of the pointer; answers whether it succeeded).
const MODEL_CLONE_GEOMETRY: u32 = 0x0065_a910;
/// The fixed string constructor (`00438170`; `thiscall`: text) and destructor
/// (`004381b0`).
const FIXED_STRING_CONSTRUCTOR: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCTOR: u32 = 0x0043_81b0;
/// The name setter called on the geometry (`00a5b950`; `thiscall`: the fixed string).
const SET_NAME: u32 = 0x00a5_b950;
/// The texture path getter (`00408da0`; `thiscall` on the texture form part).
const TEXTURE_PATH: u32 = 0x0040_8da0;
/// The texture loader (`004568c0`; `thiscall` on the object at `011dea10`: path, address of
/// the result pointer, two words).
const CREATE_TEXTURE: u32 = 0x0045_68c0;
const TEXTURE_LOADER: u32 = 0x011d_ea10;
/// `NiAVObject::GetProperty` (`00a59d30`; `thiscall`: property type), the shader type
/// reader (`00441110`; `thiscall`), `AttachProperty` (`00439410`), `RemoveProperty`
/// (`00a5b230`) and the shader's virtual slot `0xF0` (set texture: two words, the texture).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const SHADER_TYPE: u32 = 0x0044_1110;
const ATTACH_PROPERTY: u32 = 0x0043_9410;
const REMOVE_PROPERTY: u32 = 0x00a5_b230;
const SLOT_SET_TEXTURE: u32 = 0xf0;
/// The geometry steps of the attach: the skin test (`0043fad0`), the skin reset
/// (`004adde0`: 0), the translation set (`00440460`: the point at `011f426c`), the rotation
/// set (`0043fa80`: the matrix) and the node's virtual slot `0xDC` (attach child: the
/// geometry, 1).
const IS_SKINNED: u32 = 0x0043_fad0;
const RESET_SKIN: u32 = 0x004a_dde0;
const SET_TRANSLATE: u32 = 0x0044_0460;
const ZERO_POINT: u32 = 0x011f_426c;
const SET_ROTATE: u32 = 0x0043_fa80;
const SLOT_ATTACH_CHILD: u32 = 0xdc;
/// `00450f90` (`thiscall` on the geometry: 1), called for the second half of the head.
const SECOND_PART_SETUP: u32 = 0x0045_0f90;
/// `00657820` (`cdecl`: buffer, index): the part name `00655320` gives the geometry.
const PART_NAME: u32 = 0x0065_7820;
/// `00653470` (`cdecl`: geometry, float): applied with the FaceGen data's float at
/// `+0x88`.
const APPLY_PART_FLOAT: u32 = 0x0065_3470;
/// `"FaceGenEyeLeft"`/`"FaceGenEyeRight"` names (`0104aac0`, `0104aab0`) and the default
/// eye texture path (`01065804`).
const EYE_LEFT_NAME: u32 = 0x0104_aac0;
const EYE_RIGHT_NAME: u32 = 0x0104_aab0;
const DEFAULT_EYE_TEXTURE: u32 = 0x0106_5804;

/// The exe's stack frame of one of the large functions, addressed as `[EBP - offset]`.
struct Frame {
    top: u32,
}

impl Frame {
    fn at(&self, offset: u32) -> u32 {
        self.top - offset
    }
}

/// `00559450` on `holder`: the word it holds.
fn held(e: &mut Engine, holder: u32) -> u32 {
    e.call(CALLEE_00559450, &args![holder]).u32()
}

/// The shader texture step: with a texture (`held(texture) != 0`) the geometry's shader
/// property (type 3) whose shader type lies in 8..=12 gets the texture through its virtual
/// slot `0xF0` (0, 0, texture).
fn apply_shader_texture(e: &mut Engine, geometry: u32, texture: u32) {
    if held(e, texture) == 0 {
        return;
    }
    let object = held(e, geometry);
    let property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
    let qualifies = property != 0
        && e.call(SHADER_TYPE, &args![property]).i32() >= 8
        && e.call(SHADER_TYPE, &args![property]).i32() <= 0xc;
    let shader = if qualifies { property } else { 0 };
    if shader != 0 {
        let image = held(e, texture);
        e.vcall(shader, SLOT_SET_TEXTURE, &args![0u32, 0u32, image]);
    }
}

/// The node's attach (virtual slot `0xDC` of the object at `*holder`) of the geometry.
fn attach_to(e: &mut Engine, holder: u32, geometry: u32) {
    let object = held(e, geometry);
    let node = e.mem.u32(holder);
    e.vcall(node, SLOT_ATTACH_CHILD, &args![object, 1u32]);
}

/// The attach step `00655320` and `006558b0` share: a skinned geometry is reset, moved to
/// the origin, rotated by `matrix` and attached to `*head` when `reset` is set, else
/// attached to `*skinned` as it is; an unskinned one is rotated and attached to `*head`.
fn attach_geometry(e: &mut Engine, geometry: u32, matrix: u32, head: u32, skinned: u32, reset: u8) {
    let object = held(e, geometry);
    if e.call(IS_SKINNED, &args![object]).u32() != 0 {
        if reset != 0 {
            let object = held(e, geometry);
            e.call(RESET_SKIN, &args![object, 0u32]);
            let object = held(e, geometry);
            e.call(SET_TRANSLATE, &args![object, ZERO_POINT]);
            let object = held(e, geometry);
            e.call(SET_ROTATE, &args![object, matrix]);
            attach_to(e, head, geometry);
        } else {
            attach_to(e, skinned, geometry);
        }
    } else {
        let object = held(e, geometry);
        e.call(SET_ROTATE, &args![object, matrix]);
        attach_to(e, head, geometry);
    }
}

/// The geometry's name set from `text` through a fixed string temporary at `slot`.
fn name_geometry(e: &mut Engine, slot: u32, text: u32, geometry: u32) {
    let name = e.call(FIXED_STRING_CONSTRUCTOR, &args![slot, text]).u32();
    let object = held(e, geometry);
    e.call(SET_NAME, &args![object, name]);
    e.call(FIXED_STRING_DESTRUCTOR, &args![slot]);
}

/// The texture load into `texture` from `path`.
fn load_texture(e: &mut Engine, path: u32, texture: u32) {
    let loader = e.global::<u32>(TEXTURE_LOADER);
    e.call(CREATE_TEXTURE, &args![loader, path, texture, 0u32, 0u32]);
}

// Translated from 00655320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00655320 - the compiler's exception frame (FS:[0] unwind states) and the /GS cookie check (00ec408c) are not translated
/// Unnamed in the engine map (`cdecl`, four words: head holder, skinned holder, FaceGen
/// data, reset flag): attaches the two halves of the head part at `+0x80` of the FaceGen
/// data (its model's name, virtual slot `0x14` of the object at its `+0x24`, as
/// `Meshes\%s`). For each half (index 0, 1) the EGM and TRI names are built; when the
/// archive look-up fails for half 0 the whole part is taken once (index -1) instead of
/// the halves. The model (`00651b50`) is cloned into a geometry (`0065a910`), which is
/// named (`00657820`), given the data's float (`00653470`), the part's texture
/// (`Textures\%s`), the default material property when it has none (`00655270`), and
/// attached. Everything runs in a memory context scope (`BSFaceGenManager.cpp`, line
/// 0xB3B).
pub fn fn_00655320(e: &mut Engine, head: u32, skinned: u32, data: u32, reset: u8) {
    e.with_stack(0xc8, |e, base| {
        let frame = Frame {
            top: base.addr() + 0xc8,
        };
        let scope = frame.at(0x4c);
        let path = frame.at(0x64);
        let egm_name = frame.at(0x14);
        let tri_name = frame.at(0x1c);
        let unused_d = frame.at(0x24);
        let unused_e = frame.at(0x54);
        let texture = frame.at(0x5c);
        let geometry = frame.at(0x58);
        let matrix = frame.at(0x48);
        e.call(
            MEMORY_SCOPE_BEGIN,
            &args![scope, 0x37u32, 1u32, FACE_GEN_MANAGER_SOURCE, 0xb3bu32],
        );
        for string in [path, egm_name, tri_name, unused_d, unused_e] {
            e.call(STRING_CONSTRUCTOR, &args![string]);
        }
        e.call(POINTER_CONSTRUCTOR, &args![texture, 0u32]);
        e.call(POINTER_CONSTRUCTOR, &args![geometry, 0u32]);
        e.call(IDENTITY, &args![matrix]);
        let angle = x87_quiet_bits(e.global::<u32>(HEAD_ROTATION_ANGLE));
        e.call(MATRIX_ROTATION, &args![matrix, angle]);
        // FaceGen data +0x80: the head part.
        if e.mem.u32(data + 0x80) != 0 {
            let mut half: i32 = 0;
            while half < 2 {
                let part = e.mem.u32(data + 0x80);
                let model_name = e.vcall(part + 0x24, 0x14, &args![]).u32();
                e.call(STRING_FORMAT, &args![path, MESHES_FORMAT, model_name]);
                let text = held(e, path);
                let mut egm = e
                    .call(EGM_FILE_NAME, &args![egm_name, text, half as u32])
                    .u32();
                let text = held(e, path);
                let mut tri = e
                    .call(TRI_FILE_NAME, &args![tri_name, text, half as u32])
                    .u32();
                let mut model = 0u32;
                let first = frame.at(0x7c);
                let second = frame.at(0x70);
                e.call(IDENTITY, &args![first]);
                e.call(IDENTITY, &args![second]);
                e.call(ARCHIVE_SPLIT_PATH, &args![egm, first, second]);
                let found = e.call(ARCHIVE_FIND, &args![0u32, first, second, egm]).u32();
                if found == 0 {
                    if half == 0 {
                        let text = held(e, path);
                        egm = e
                            .call(EGM_FILE_NAME, &args![egm_name, text, 0xffff_ffffu32])
                            .u32();
                        let text = held(e, path);
                        tri = e
                            .call(TRI_FILE_NAME, &args![tri_name, text, 0xffff_ffffu32])
                            .u32();
                        let text = held(e, path);
                        model = e
                            .call(
                                FACE_GEN_MODEL,
                                &args![egm, text, tri, 0u32, 1u32, half as u32, 0u32],
                            )
                            .u32();
                        half = 2;
                    }
                } else {
                    let text = held(e, path);
                    model = e
                        .call(
                            FACE_GEN_MODEL,
                            &args![egm, text, tri, 0u32, 1u32, half as u32, 0u32],
                        )
                        .u32();
                }
                if model != 0 {
                    e.call(MODEL_CLONE_GEOMETRY, &args![model, data, geometry]);
                }
                if held(e, geometry) != 0 {
                    if half == 1 {
                        let object = held(e, geometry);
                        e.call(SECOND_PART_SETUP, &args![object, 1u32]);
                    }
                    let name = e.call(PART_NAME, &args![frame.at(0xac), half as u32]).u32();
                    name_geometry(e, frame.at(0xb8), name, geometry);
                    // FaceGen data +0x88: a float passed through the x87 stack.
                    let value = x87_quiet_bits(e.mem.u32(data + 0x88));
                    let object = held(e, geometry);
                    e.call(APPLY_PART_FLOAT, &args![object, value]);
                    let part = e.mem.u32(data + 0x80);
                    let texture_name = e.call(TEXTURE_PATH, &args![part + 0x3c]).u32();
                    e.call(STRING_FORMAT, &args![path, TEXTURES_FORMAT, texture_name]);
                    let text = held(e, path);
                    load_texture(e, text, texture);
                    apply_shader_texture(e, geometry, texture);
                    let object = held(e, geometry);
                    if e.call(GET_PROPERTY, &args![object, 0u32]).u32() == 0 {
                        let material = fn_00655270(e);
                        let object = held(e, geometry);
                        e.call(ATTACH_PROPERTY, &args![object, material]);
                    }
                    attach_geometry(e, geometry, matrix, head, skinned, reset);
                }
                e.call(POINTER_ASSIGN, &args![geometry, 0u32]);
                e.call(POINTER_ASSIGN, &args![texture, 0u32]);
                half += 1;
            }
        }
        e.call(POINTER_DESTRUCTOR, &args![geometry]);
        e.call(POINTER_DESTRUCTOR, &args![texture]);
        for string in [unused_e, unused_d, tri_name, egm_name, path] {
            e.call(STRING_DESTRUCTOR, &args![string]);
        }
        e.call(MEMORY_SCOPE_END, &args![scope]);
    });
}

/// One eye of [`fn_006558b0`]: the part at `data + part_offset`, named `name`.
fn attach_eye(
    e: &mut Engine,
    frame: &Frame,
    data: u32,
    part_offset: u32,
    name: u32,
    name_slot: u32,
    reset: (u32, u32, u8),
) {
    let (head, skinned, flag) = reset;
    let path = frame.at(0x68);
    let egm_name = frame.at(0x14);
    let tri_name = frame.at(0x1c);
    let texture = frame.at(0x60);
    let geometry = frame.at(0x5c);
    let matrix = frame.at(0x48);
    let part = e.mem.u32(data + part_offset);
    if part == 0 {
        return;
    }
    let model_name = e.vcall(part, 0x14, &args![]).u32();
    e.call(STRING_FORMAT, &args![path, MESHES_FORMAT, model_name]);
    let text = held(e, path);
    let tri = e
        .call(TRI_FILE_NAME, &args![tri_name, text, 0xffff_ffffu32])
        .u32();
    let mesh = held(e, path);
    let text = held(e, path);
    let egm = e
        .call(EGM_FILE_NAME, &args![egm_name, text, 0xffff_ffffu32])
        .u32();
    let model = e
        .call(
            FACE_GEN_MODEL,
            &args![egm, mesh, tri, 0u32, 1u32, 0xffff_ffffu32, 0u32],
        )
        .u32();
    if model != 0 {
        e.call(MODEL_CLONE_GEOMETRY, &args![model, data, geometry]);
    }
    if held(e, geometry) != 0 {
        name_geometry(e, name_slot, name, geometry);
        // FaceGen data +0x8C: the eye texture form.
        let eye_texture = e.mem.u32(data + 0x8c);
        if eye_texture != 0 {
            let texture_name = e.call(TEXTURE_PATH, &args![eye_texture + 0x24]).u32();
            e.call(STRING_FORMAT, &args![path, TEXTURES_FORMAT, texture_name]);
            let text = held(e, path);
            load_texture(e, text, texture);
        } else {
            load_texture(e, DEFAULT_EYE_TEXTURE, texture);
        }
        apply_shader_texture(e, geometry, texture);
        attach_geometry(e, geometry, matrix, head, skinned, flag);
    }
    e.call(POINTER_ASSIGN, &args![geometry, 0u32]);
    e.call(POINTER_ASSIGN, &args![texture, 0u32]);
}

// Translated from 006558b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006558b0 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// Unnamed in the engine map (`cdecl`, four words: head holder, skinned holder, FaceGen
/// data, reset flag): attaches the left and right eye parts (`+0xD8`, `+0xDC` of the
/// FaceGen data; model name from each part's virtual slot `0x14`) as `FaceGenEyeLeft` and
/// `FaceGenEyeRight`: the TRI and EGM names (index -1), the model (`00651b50`) cloned into
/// a geometry, its texture from the eye texture form at `+0x8C` (or the default path
/// `01065804`), then attached as the head half is. In a memory context scope
/// (`BSFaceGenManager.cpp`, line 0xBA7).
pub fn fn_006558b0(e: &mut Engine, head: u32, skinned: u32, data: u32, reset: u8) {
    e.with_stack(0xa0, |e, base| {
        let frame = Frame {
            top: base.addr() + 0xa0,
        };
        let scope = frame.at(0x4c);
        let path = frame.at(0x68);
        let strings = [
            path,
            frame.at(0x14),
            frame.at(0x1c),
            frame.at(0x24),
            frame.at(0x54),
        ];
        let texture = frame.at(0x60);
        let geometry = frame.at(0x5c);
        let matrix = frame.at(0x48);
        e.call(
            MEMORY_SCOPE_BEGIN,
            &args![scope, 0x37u32, 1u32, FACE_GEN_MANAGER_SOURCE, 0xba7u32],
        );
        for string in strings {
            e.call(STRING_CONSTRUCTOR, &args![string]);
        }
        e.call(POINTER_CONSTRUCTOR, &args![texture, 0u32]);
        e.call(POINTER_CONSTRUCTOR, &args![geometry, 0u32]);
        e.call(IDENTITY, &args![matrix]);
        let angle = x87_quiet_bits(e.global::<u32>(HEAD_ROTATION_ANGLE));
        e.call(MATRIX_ROTATION, &args![matrix, angle]);
        let targets = (head, skinned, reset);
        attach_eye(
            e,
            &frame,
            data,
            0xd8,
            EYE_LEFT_NAME,
            frame.at(0x7c),
            targets,
        );
        attach_eye(
            e,
            &frame,
            data,
            0xdc,
            EYE_RIGHT_NAME,
            frame.at(0x80),
            targets,
        );
        e.call(POINTER_DESTRUCTOR, &args![geometry]);
        e.call(POINTER_DESTRUCTOR, &args![texture]);
        for string in strings.iter().rev() {
            e.call(STRING_DESTRUCTOR, &args![*string]);
        }
        e.call(MEMORY_SCOPE_END, &args![scope]);
    });
}

/// The FaceGen data's head part lists (`+0x94` models, `+0xA4` textures, `+0xB4` names):
/// their count test (`00658930`; `thiscall`) and element (`00877a30`; `thiscall`: index,
/// answers the address of the element), and the form's validity test (`0048cee0`;
/// `thiscall`).
const PART_LIST_COUNT: u32 = 0x0065_8930;
const PART_LIST_AT: u32 = 0x0087_7a30;
const FORM_IS_VALID: u32 = 0x0048_cee0;
/// The setting `00654720` reads for the first part (`011d5adc`; its byte through
/// `00408d60`).
const HEAD_SETTING: u32 = 0x011d_5adc;
const SETTING_STRING: u32 = 0x0040_8d60;
/// The material steps for the first two parts: the material property type (`004af350`; no
/// arguments), the new material property (`00aa13e0` 0x4C bytes, `00a75650`) and the
/// material value its name is set from (`006041c0`; no arguments).
const MATERIAL_PROPERTY_TYPE: u32 = 0x004a_f350;
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const MATERIAL_PROPERTY_CONSTRUCTOR: u32 = 0x00a7_5650;
const MATERIAL_NAME: u32 = 0x0060_41c0;
/// The shader flag step (`00653270`; `cdecl`: the shader type at `01200464`, the shader
/// property) and its flag setter (`00441130`; `thiscall`: 1, 0).
const FIND_SHADER: u32 = 0x0065_3270;
const SHADER_KIND: u32 = 0x0120_0464;
const SET_SHADER_FLAG: u32 = 0x0044_1130;
/// The shader's update (`00b68d30`; `thiscall`).
const SHADER_UPDATE: u32 = 0x00b6_8d30;
/// The eye link test (`00605d20`; no arguments) and the eye link (`004b3e60`; `cdecl`:
/// both eyes, the float at `0101e2bc`, 0, 0).
const EYES_ENABLED: u32 = 0x0060_5d20;
const LINK_EYES: u32 = 0x004b_3e60;
const EYE_LINK_VALUE: u32 = 0x0101_e2bc;
/// The accessory list walk (`00726070`: next iterator), the texture swap (`0048afe0`;
/// `thiscall` on the accessory's model part: the geometry) and `"FaceGenAccessory"`
/// (`010657e4`).
const LIST_NEXT: u32 = 0x0072_6070;
const SWAP_TEXTURES: u32 = 0x0048_afe0;
const ACCESSORY_NAME: u32 = 0x0106_57e4;

// Translated from 00654720 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00654720 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// Unnamed in the engine map (`cdecl`, four words: head holder, skinned holder, FaceGen
/// data, reset flag), the FaceGen head build. For each of the eight head parts with a valid
/// model and texture form (lists at `+0x94`, `+0xA4`): the mesh path (`Meshes\%s` of the
/// model's virtual slot `0x14`; for part 0 without `+0xE0` replaced by the low LOD NIF when
/// the archive has it), the FaceGen model by part (the switch on the part index: part 0
/// with the alternate file when the data's byte `+0xD5` or the setting `011d5adc` asks,
/// part 1 with it always, parts 2 to 5 without; parts 6 and 7 keep the model word as it
/// is), cloned into a geometry, named (list `+0xB4`), textured (`Textures\%s`), given its
/// properties (material for parts 0 and 1, the default for 3 and 4), attached, and its
/// shader updated. Parts 0 and 1 are remembered and linked as the eyes (`004b3e60`); then
/// the head halves ([`fn_00655320`]), the eyes ([`fn_006558b0`]) and the accessories (list
/// `+0xE4`, flag 1, `FaceGenAccessory`). In a memory context scope
/// (`BSFaceGenManager.cpp`, line 0xA48). Answers true.
pub fn fn_00654720(e: &mut Engine, head: u32, skinned: u32, data: u32, reset: u8) -> bool {
    e.with_stack(0x100, |e, base| {
        let frame = Frame {
            top: base.addr() + 0x100,
        };
        let scope = frame.at(0x5c);
        let texture = frame.at(0x78);
        let geometry = frame.at(0x74);
        let matrix = frame.at(0x54);
        let path = frame.at(0x70);
        let egm_name = frame.at(0x14);
        let tri_name = frame.at(0x24);
        let alternate_name = frame.at(0x2c);
        let unused = frame.at(0x64);
        let low_lod_name = frame.at(0x1c);
        let strings = [
            path,
            egm_name,
            tri_name,
            alternate_name,
            unused,
            low_lod_name,
        ];
        e.call(
            MEMORY_SCOPE_BEGIN,
            &args![scope, 0x37u32, 1u32, FACE_GEN_MANAGER_SOURCE, 0xa48u32],
        );
        let mut model = 0u32;
        e.call(POINTER_CONSTRUCTOR, &args![texture, 0u32]);
        e.call(POINTER_CONSTRUCTOR, &args![geometry, 0u32]);
        let mut first_eye = 0u32;
        let mut second_eye = 0u32;
        e.call(IDENTITY, &args![matrix]);
        for string in strings {
            e.call(STRING_CONSTRUCTOR, &args![string]);
        }
        let angle = x87_quiet_bits(e.global::<u32>(HEAD_ROTATION_ANGLE));
        e.call(MATRIX_ROTATION, &args![matrix, angle]);
        let models = data + 0x94;
        let textures = data + 0xa4;
        for part in 0..8u32 {
            let valid = |e: &mut Engine, list: u32| {
                if e.call(PART_LIST_COUNT, &args![list]).u32() == 0 {
                    return false;
                }
                let at = e.call(PART_LIST_AT, &args![list, part]).u32();
                if e.mem.u32(at) == 0 {
                    return false;
                }
                let at = e.call(PART_LIST_AT, &args![list, part]).u32();
                let form = e.mem.u32(at);
                e.call(FORM_IS_VALID, &args![form]).u32() != 0
            };
            if !valid(e, models) || !valid(e, textures) {
                continue;
            }
            let at = e.call(PART_LIST_AT, &args![models, part]).u32();
            let form = e.mem.u32(at);
            let model_name = e.vcall(form, 0x14, &args![]).u32();
            e.call(STRING_FORMAT, &args![path, MESHES_FORMAT, model_name]);
            // FaceGen data +0xE0.
            if e.mem.u32(data + 0xe0) == 0 && part == 0 {
                let text = held(e, path);
                e.call(LOW_LOD_FILE_NAME, &args![low_lod_name, text]);
                let mut found = 0u32;
                if held(e, low_lod_name) != 0 {
                    let first = frame.at(0xa4);
                    let second = frame.at(0x9c);
                    e.call(IDENTITY, &args![first]);
                    e.call(IDENTITY, &args![second]);
                    let text = held(e, low_lod_name);
                    e.call(ARCHIVE_SPLIT_PATH, &args![text, first, second]);
                    let text = held(e, low_lod_name);
                    found = e
                        .call(ARCHIVE_FIND, &args![0u32, first, second, text])
                        .u32();
                }
                if found != 0 {
                    let text = held(e, path);
                    e.call(LOW_LOD_FILE_NAME, &args![path, text]);
                }
            }
            let mut alternate = 0u32;
            match part {
                0 => {
                    // FaceGen data +0xD5 (byte), or the setting's first character.
                    let asked = e.mem.u8(data + 0xd5) != 0 || {
                        let text = e.call(SETTING_STRING, &args![HEAD_SETTING]).u32();
                        e.mem.u8(text) != 0
                    };
                    if asked {
                        let text = held(e, path);
                        alternate = e
                            .call(ALTERNATE_FILE_NAME, &args![alternate_name, text])
                            .u32();
                    }
                    model = head_model(e, path, egm_name, tri_name, alternate, 0);
                }
                1 => {
                    let text = held(e, path);
                    let alternate = e
                        .call(ALTERNATE_FILE_NAME, &args![alternate_name, text])
                        .u32();
                    model = head_model(e, path, egm_name, tri_name, alternate, 0);
                }
                2..=5 => {
                    model = head_model(e, path, egm_name, tri_name, 0, 0);
                }
                // Parts 6 and 7: no case; the model word keeps its value.
                _ => {}
            }
            if model == 0 {
                continue;
            }
            if !e
                .call(MODEL_CLONE_GEOMETRY, &args![model, data, geometry])
                .bool()
            {
                continue;
            }
            if held(e, geometry) == 0 {
                continue;
            }
            let at = e.call(PART_LIST_AT, &args![data + 0xb4, part]).u32();
            let name = e.mem.u32(at);
            name_geometry(e, frame.at(0xc0), name, geometry);
            if part == 0 {
                first_eye = held(e, geometry);
            }
            if part == 1 {
                second_eye = held(e, geometry);
            }
            let at = e.call(PART_LIST_AT, &args![textures, part]).u32();
            let form = e.mem.u32(at);
            let texture_name = e.call(TEXTURE_PATH, &args![form]).u32();
            e.call(STRING_FORMAT, &args![path, TEXTURES_FORMAT, texture_name]);
            let text = held(e, path);
            load_texture(e, text, texture);
            apply_shader_texture(e, geometry, texture);
            if part == 4 || part == 3 {
                let object = held(e, geometry);
                if e.call(GET_PROPERTY, &args![object, 0u32]).u32() != 0 {
                    let object = held(e, geometry);
                    e.call(REMOVE_PROPERTY, &args![object, 0u32]);
                }
                let material = fn_00655270(e);
                let object = held(e, geometry);
                e.call(ATTACH_PROPERTY, &args![object, material]);
            }
            if part == 0 || part == 1 {
                let kind = e.call(MATERIAL_PROPERTY_TYPE, &args![]).u32();
                let object = held(e, geometry);
                let existing = e.call(GET_PROPERTY, &args![object, kind]).u32();
                if existing != 0 {
                    let name = e.call(MATERIAL_NAME, &args![]).u32();
                    e.call(SET_NAME, &args![existing, name]);
                } else {
                    let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
                    let material = if block == 0 {
                        0
                    } else {
                        e.call(MATERIAL_PROPERTY_CONSTRUCTOR, &args![block]).u32()
                    };
                    let name = e.call(MATERIAL_NAME, &args![]).u32();
                    e.call(SET_NAME, &args![material, name]);
                    let object = held(e, geometry);
                    e.call(ATTACH_PROPERTY, &args![object, material]);
                }
            }
            let object = held(e, geometry);
            if e.call(IS_SKINNED, &args![object]).u32() != 0 {
                if reset != 0 {
                    let object = held(e, geometry);
                    e.call(RESET_SKIN, &args![object, 0u32]);
                    let object = held(e, geometry);
                    e.call(SET_TRANSLATE, &args![object, ZERO_POINT]);
                    let object = held(e, geometry);
                    e.call(SET_ROTATE, &args![object, matrix]);
                    attach_to(e, head, geometry);
                    let object = held(e, geometry);
                    let shader = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
                    let found = e.call(FIND_SHADER, &args![SHADER_KIND, shader]).u32();
                    if found != 0 {
                        e.call(SET_SHADER_FLAG, &args![found, 1u32, 0u32]);
                    }
                } else {
                    attach_to(e, skinned, geometry);
                }
            } else {
                let object = held(e, geometry);
                e.call(SET_ROTATE, &args![object, matrix]);
                attach_to(e, head, geometry);
            }
            let object = held(e, geometry);
            let property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
            let qualifies = property != 0
                && e.call(SHADER_TYPE, &args![property]).i32() >= 8
                && e.call(SHADER_TYPE, &args![property]).i32() <= 0xc;
            if qualifies {
                e.call(SHADER_UPDATE, &args![property]);
            }
            model = 0;
            e.call(POINTER_ASSIGN, &args![texture, 0u32]);
            e.call(POINTER_ASSIGN, &args![geometry, 0u32]);
        }
        if e.call(EYES_ENABLED, &args![]).bool() && first_eye != 0 && second_eye != 0 {
            let value = x87_quiet_bits(e.global::<u32>(EYE_LINK_VALUE));
            e.call(LINK_EYES, &args![first_eye, second_eye, value, 0u32, 0u32]);
        }
        // FaceGen data +0x80: the head part; +0xD8 and +0xDC: the eyes.
        if e.mem.u32(data + 0x80) != 0 {
            fn_00655320(e, head, skinned, data, reset);
        }
        if e.mem.u32(data + 0xd8) != 0 && e.mem.u32(data + 0xdc) != 0 {
            fn_006558b0(e, head, skinned, data, reset);
        }
        // FaceGen data +0xE4: the accessory list.
        let mut iterator = data + 0xe4;
        while iterator != 0 {
            let node = e.call(IDENTITY, &args![iterator]).u32();
            if e.mem.u32(node) == 0 {
                break;
            }
            let node = e.call(IDENTITY, &args![iterator]).u32();
            let accessory = e.mem.u32(node).wrapping_add(0x24);
            let model_name = e.vcall(accessory, 0x14, &args![]).u32();
            e.call(STRING_FORMAT, &args![path, MESHES_FORMAT, model_name]);
            let text = held(e, path);
            let tri = e
                .call(TRI_FILE_NAME, &args![tri_name, text, 0xffff_ffffu32])
                .u32();
            let mesh = held(e, path);
            let text = held(e, path);
            let egm = e
                .call(EGM_FILE_NAME, &args![egm_name, text, 0xffff_ffffu32])
                .u32();
            model = e
                .call(
                    FACE_GEN_MODEL,
                    &args![egm, mesh, tri, 0u32, 1u32, 0xffff_ffffu32, 1u32],
                )
                .u32();
            if model != 0
                && e.call(MODEL_CLONE_GEOMETRY, &args![model, data, geometry])
                    .bool()
                && held(e, geometry) != 0
            {
                let object = held(e, geometry);
                e.call(SET_ROTATE, &args![object, matrix]);
                attach_to(e, head, geometry);
                let object = held(e, geometry);
                let node = e.call(IDENTITY, &args![iterator]).u32();
                let part = e.mem.u32(node).wrapping_add(0x24);
                e.call(SWAP_TEXTURES, &args![part, object]);
                name_geometry(e, frame.at(0xcc), ACCESSORY_NAME, geometry);
            }
            iterator = e.call(LIST_NEXT, &args![iterator]).u32();
        }
        for string in [
            low_lod_name,
            unused,
            alternate_name,
            tri_name,
            egm_name,
            path,
        ] {
            e.call(STRING_DESTRUCTOR, &args![string]);
        }
        e.call(POINTER_DESTRUCTOR, &args![geometry]);
        e.call(POINTER_DESTRUCTOR, &args![texture]);
        e.call(MEMORY_SCOPE_END, &args![scope]);
        true
    })
}

/// The head part model of [`fn_00654720`]'s switch: the TRI name (index -1), the mesh path,
/// the EGM name (index -1), then `00651b50` (EGM, path, TRI, `alternate`, 1, -1, `flag`).
fn head_model(
    e: &mut Engine,
    path: u32,
    egm_name: u32,
    tri_name: u32,
    alternate: u32,
    flag: u32,
) -> u32 {
    let text = held(e, path);
    let tri = e
        .call(TRI_FILE_NAME, &args![tri_name, text, 0xffff_ffffu32])
        .u32();
    let mesh = held(e, path);
    let text = held(e, path);
    let egm = e
        .call(EGM_FILE_NAME, &args![egm_name, text, 0xffff_ffffu32])
        .u32();
    e.call(
        FACE_GEN_MODEL,
        &args![egm, mesh, tri, alternate, 1u32, 0xffff_ffffu32, flag],
    )
    .u32()
}

/// `"FaceGenHair"` (`0104aad0`), `""` (`01011584`) and `"Textures\"` (`0101d7fc`).
const HAIR_NAME: u32 = 0x0104_aad0;
const EMPTY_TEXT: u32 = 0x0101_1584;
const TEXTURES_FOLDER: u32 = 0x0101_d7fc;
/// The lock pair around the head update (`00652140` takes it, `00652190` releases it).
const LOCK: u32 = 0x0065_2140;
const UNLOCK: u32 = 0x0065_2190;
/// The detail level steps (`cdecl`): `00652440` (data, 0; answers in ST0), `00406ce0` (the
/// float; answers in ST0), then `_ftol2` (`00ec62c0`).
const DETAIL_VALUE: u32 = 0x0065_2440;
const DETAIL_ROUND: u32 = 0x0040_6ce0;
const FTOL: u32 = 0x00ec_62c0;
/// Character buffer helpers (`cdecl`): `strcpy_s`-like (`00406d30`: buffer, size, text),
/// `sprintf_s`-like (`00406d00`: buffer, size, format, value), the archive name look-up
/// (`00b4f5e0`: output, name, the text at `0106582c`, 1), `_strlen` (`00ec6130`)
/// and `strncmp` (`00ec7ec0`).
const COPY_TEXT: u32 = 0x0040_6d30;
const FORMAT_TEXT: u32 = 0x0040_6d00;
const FIND_ARCHIVE_NAME: u32 = 0x00b4_f5e0;
const STRLEN: u32 = 0x00ec_6130;
const STRNCMP: u32 = 0x00ec_7ec0;
/// The format at `01019f08` and the text at `0106582c` the tint path uses.
const TINT_FORMAT: u32 = 0x0101_9f08;
const TINT_SUFFIX: u32 = 0x0106_582c;
/// The string assignment (`004037f0`; `thiscall`: text, 0), the fixed string text getter
/// (`0043b1b0`; `thiscall`), the string's assign (`00438390`; `thiscall`: text) and append
/// (`00404820`; `thiscall`: text).
const STRING_SET: u32 = 0x0040_37f0;
const FIXED_STRING_TEXT: u32 = 0x0043_b1b0;
const STRING_ASSIGN: u32 = 0x0043_8390;
const STRING_APPEND: u32 = 0x0040_4820;
/// The node's child lookup by name (virtual slot `0x9C`), the geometry's model data
/// (virtual slot `0x1C`), the FaceGen model of a geometry's data (`006530e0`; `cdecl`) and
/// `BSShaderManager::PrepareObject` (`00b57e30`; `cdecl`: object, 0, 0).
const SLOT_GET_OBJECT_BY_NAME: u32 = 0x9c;
const SLOT_MODEL_DATA: u32 = 0x1c;
const GEOMETRY_FACE_GEN_MODEL: u32 = 0x0065_30e0;
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
/// The pointer assignment from a pointer reference (`006e5cc0`; `thiscall`).
const POINTER_ASSIGN_FROM: u32 = 0x006e_5cc0;
/// The FaceGen detail texture name (`00653880`; `cdecl`: string, race word, level, path).
const DETAIL_FILE_NAME: u32 = 0x0065_3880;
/// The coordinate texturing step (`0065b410`; `thiscall` on the model: data, address of the
/// pointer, `006151d0`'s answer).
const APPLY_COORDINATE_TEXTURING: u32 = 0x0065_b410;
const COORDINATE_SOURCE: u32 = 0x0061_51d0;
/// The normal map path (`0048e5b0`; `cdecl`: string, path).
const NORMAL_FILE_NAME: u32 = 0x0048_e5b0;
/// The shader's virtual slots: `0x100` (set texture: slot, texture), `0xFC` (set tint
/// texture: slot, texture), `0xC0` (update) and `0xF0` (set texture: slot, 0, texture).
const SLOT_SHADER_SET_TEXTURE: u32 = 0x100;
const SLOT_SHADER_SET_TINT: u32 = 0xfc;
const SLOT_SHADER_UPDATE: u32 = 0xc0;
/// The shader's default texture test (`00540570`; `thiscall`: 0), the default texture
/// (`00656db0`; no arguments) and the default normal texture (`004af8a0`; no arguments).
const SHADER_HAS_TEXTURE: u32 = 0x0054_0570;
const DEFAULT_TEXTURE: u32 = 0x0065_6db0;
const DEFAULT_NORMAL_TEXTURE: u32 = 0x004a_f8a0;
/// The property type 2 value test (`009a9350`; answers in ST0, compared with the double 1.0
/// at `01012070`) and its setter (`00654700`; `thiscall`: the float at `0101e570`).
const PROPERTY_VALUE: u32 = 0x009a_9350;
const ONE_DOUBLE: u32 = 0x0101_2070;
const PROPERTY_SET_VALUE: u32 = 0x0065_4700;
const PROPERTY_NEW_VALUE: u32 = 0x0101_e570;
/// `BSFaceGenManager::PrecacheFaceGeometry` (`00657010`; `cdecl`: geometry, kind, flag).
const PRECACHE_FACE_GEOMETRY: u32 = 0x0065_7010;
/// The node's child count (`00453470`; `thiscall`) and child (`0043b4a0`; `thiscall`:
/// index), the object's name holder (`00413f40`; answers `object + 8`) and the name
/// comparison (`009a3830`; `cdecl`: name, fixed string; answers a byte).
const CHILD_COUNT: u32 = 0x0045_3470;
const CHILD_AT: u32 = 0x0043_b4a0;
const NAME_HOLDER: u32 = 0x0041_3f40;
const NAME_EQUALS: u32 = 0x009a_3830;
/// The colour constructor (`00414430`; `thiscall`: four floats) and the divisor of the
/// colour bytes (the double at `0101e568`).
const COLOR_CONSTRUCTOR: u32 = 0x0041_4430;
const COLOR_DIVISOR: u32 = 0x0101_e568;
/// The node's property update (`00a5a040`, `NiAVObject::UpdateProperties`), the update
/// data constructor (`0043d410`; `thiscall`: float, two words) and the update with it
/// (`00a59c60`; `thiscall`: the data).
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const UPDATE_DATA_CONSTRUCTOR: u32 = 0x0043_d410;
const UPDATE_WITH: u32 = 0x00a5_9c60;

/// The colour of a hair or accessory shader: the FaceGen data's colour word (`+0x84`)
/// split into its three low bytes, each divided by the double at `0101e568` (x87, stored
/// as a float), with alpha 1.0, built in the temporary at `temporary` (`00414430`) and
/// copied into the shader's colour at `+0x84`; then the shader's update (slot `0xC0`).
fn set_shader_color(e: &mut Engine, data: u32, shader: u32, temporary: u32) {
    let color = e.mem.u32(data + 0x84);
    let divisor = f64::from_bits(e.mem.u64(COLOR_DIVISOR));
    let channel = |value: u32| x87_store32(x87_div(f64::from(value & 0xff), divisor));
    let third = channel(color >> 16);
    let second = channel(u32::from(color as u16) >> 8);
    let first = channel(color);
    let built = e
        .call(
            COLOR_CONSTRUCTOR,
            &args![temporary, first, second, third, 1.0f32.to_bits()],
        )
        .u32();
    for index in 0..4 {
        let word = e.mem.u32(built.wrapping_add(4 * index));
        e.mem.set_u32(shader + 0x84 + 4 * index, word);
    }
    e.vcall(shader, SLOT_SHADER_UPDATE, &args![]);
}

/// The shader property (type 3) of `object` when its shader type is `kind` (`00441110`).
fn shader_of_type(e: &mut Engine, property: u32, kind: i32) -> u32 {
    let matches = property != 0 && e.call(SHADER_TYPE, &args![property]).i32() == kind;
    if matches {
        property
    } else {
        0
    }
}

/// `00655fa0`'s coordinate texturing step: unless the data's list at `+0xC4` has an entry
/// for `part` holding something, the model (`0065b410`) fills the pointer at `target`.
fn coordinate_texturing(e: &mut Engine, data: u32, part: u32, model: u32, target: u32) {
    let list = data + 0xc4;
    if e.call(PART_LIST_COUNT, &args![list]).u32() != 0 {
        let at = e.call(PART_LIST_AT, &args![list, part]).u32();
        if held(e, at) != 0 {
            return;
        }
    }
    let source = e.call(COORDINATE_SOURCE, &args![]).u32();
    e.call(
        APPLY_COORDINATE_TEXTURING,
        &args![model, data, target, source],
    );
}

// Translated from 00655fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00655fa0 - the compiler's exception frame (FS:[0] unwind states) and the /GS cookie check (00ec408c) are not translated
/// Unnamed in the engine map (`cdecl`, two words: head node, FaceGen data): updates a built
/// head under the lock (`00652140` .. `00652190`); nothing without a node or data. The
/// detail level is `_ftol2` of `00406ce0` of `00652440`. For the head parts 0 to 5 found
/// in the node by name (list `+0xB4`, slot `0x9C`) with model data and a FaceGen model
/// and a shader of type 8..=12: the coordinate texturing, the detail or the part's own
/// texture, the normal map, the tint (with `+0xD4` for the skin parts 0 and 1, kind 0xE),
/// then `PrecacheFaceGeometry` and the shader update. Then every child of the node: the
/// eyes are only prepared, hair and accessories also get the data's colour on their
/// shader (type 8 or 12); each is precached. Last the node's properties and update.
pub fn fn_00655fa0(e: &mut Engine, node: u32, data: u32) {
    e.with_stack(0x458, |e, base| {
        let frame = Frame {
            top: base.addr() + 0x458,
        };
        let scope = frame.at(0x158);
        let eye_left = frame.at(0x188);
        let eye_right = frame.at(0x20);
        let accessory = frame.at(0x170);
        let hair = frame.at(0x154);
        let unused_first = frame.at(0x190);
        let unused_second = frame.at(0x1c);
        let unused_third = frame.at(0x168);
        let detail_path = frame.at(0x14);
        let texture_path = frame.at(0x30);
        let strings = [
            unused_first,
            unused_second,
            unused_third,
            detail_path,
            texture_path,
        ];
        let image = frame.at(0x160);
        let tint = frame.at(0x16c);
        let normal = frame.at(0x150);
        let mut kind = 0u32;
        e.call(
            MEMORY_SCOPE_BEGIN,
            &args![scope, 0x37u32, 1u32, FACE_GEN_MANAGER_SOURCE, 0xc2au32],
        );
        for (slot, text) in [
            (eye_left, EYE_LEFT_NAME),
            (eye_right, EYE_RIGHT_NAME),
            (accessory, ACCESSORY_NAME),
            (hair, HAIR_NAME),
        ] {
            e.call(FIXED_STRING_CONSTRUCTOR, &args![slot, text]);
        }
        for string in strings {
            e.call(STRING_CONSTRUCTOR, &args![string]);
        }
        for pointer in [image, tint, normal] {
            e.call(POINTER_CONSTRUCTOR, &args![pointer, 0u32]);
        }
        e.call(LOCK, &args![]);
        if node != 0 && data != 0 {
            update_head(e, &frame, node, data, &mut kind);
        }
        if node == 0 || data == 0 {
            e.call(UNLOCK, &args![]);
        }
        for pointer in [normal, tint, image] {
            e.call(POINTER_DESTRUCTOR, &args![pointer]);
        }
        for string in strings.iter().rev() {
            e.call(STRING_DESTRUCTOR, &args![*string]);
        }
        for slot in [hair, accessory, eye_right, eye_left] {
            e.call(FIXED_STRING_DESTRUCTOR, &args![slot]);
        }
        e.call(MEMORY_SCOPE_END, &args![scope]);
    });
}

/// The body of [`fn_00655fa0`] with node and data present (ends with the release of the
/// lock).
fn update_head(e: &mut Engine, frame: &Frame, node: u32, data: u32, kind: &mut u32) {
    let detail_path = frame.at(0x14);
    let texture_path = frame.at(0x30);
    let image = frame.at(0x160);
    let tint = frame.at(0x16c);
    let normal = frame.at(0x150);
    // 0065619f: the cdecl `00652440` takes three words (data, 0, 0); ADD ESP,8 pops two
    // and the third slot is reused for the next call's argument.
    let value = e.call(DETAIL_VALUE, &args![data, 0u32, 0u32]).f64();
    let rounded = e.call(DETAIL_ROUND, &args![x87_store32(value)]).f64();
    let level = e.call(FTOL, &args![rounded]).u32() as u8;
    e.call(
        COPY_TEXT,
        &args![frame.at(0x144), 0x104u32, TEXTURES_FOLDER],
    );
    for part in 0..8u32 {
        e.call(STRING_SET, &args![detail_path, EMPTY_TEXT, 0u32]);
        if part == 6 || part == 7 {
            continue;
        }
        *kind = if part == 0 || part == 1 { 0xe } else { 1 };
        let at = e.call(PART_LIST_AT, &args![data + 0x94, part]).u32();
        if e.mem.u32(at) == 0 {
            continue;
        }
        let at = e.call(PART_LIST_AT, &args![data + 0xa4, part]).u32();
        if e.mem.u32(at) == 0 {
            continue;
        }
        let at = e.call(PART_LIST_AT, &args![data + 0xb4, part]).u32();
        let name_text = e.mem.u32(at);
        let name_slot = frame.at(0x3e4);
        let name = e
            .call(FIXED_STRING_CONSTRUCTOR, &args![name_slot, name_text])
            .u32();
        let geometry = e.vcall(node, SLOT_GET_OBJECT_BY_NAME, &args![name]).u32();
        e.call(FIXED_STRING_DESTRUCTOR, &args![name_slot]);
        if geometry == 0 {
            continue;
        }
        let model_data = e.vcall(geometry, SLOT_MODEL_DATA, &args![]).u32();
        if model_data == 0 {
            continue;
        }
        let model = e.call(GEOMETRY_FACE_GEN_MODEL, &args![model_data]).u32();
        if model == 0 {
            continue;
        }
        e.call(PREPARE_OBJECT, &args![geometry, 0u32, 0u32]);
        let property = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
        let qualifies = property != 0
            && e.call(SHADER_TYPE, &args![property]).i32() >= 8
            && e.call(SHADER_TYPE, &args![property]).i32() <= 0xc;
        let shader = if qualifies { property } else { 0 };
        if shader == 0 {
            continue;
        }
        // FaceGen data +0xC4: per-part tint pointers; +0xD4: the tint flag (byte).
        if e.call(PART_LIST_COUNT, &args![data + 0xc4]).u32() != 0 && e.mem.u8(data + 0xd4) != 0 {
            let at = e.call(PART_LIST_AT, &args![data + 0xc4, part]).u32();
            e.call(POINTER_ASSIGN_FROM, &args![tint, at]);
        }
        let mut detailed = false;
        if part == 0 && (level as i8) >= 0 {
            let at = e.call(PART_LIST_AT, &args![data + 0xa4, part]).u32();
            let form = e.mem.u32(at);
            let path = e.call(TEXTURE_PATH, &args![form]).u32();
            // FaceGen data +0x90: the word the detail texture name is built from.
            let race = e.mem.u32(data + 0x90);
            let found = e
                .call(
                    DETAIL_FILE_NAME,
                    &args![detail_path, race, u32::from(level), path],
                )
                .u32();
            if found != 0 {
                coordinate_texturing(e, data, part, model, tint);
                let text = held(e, detail_path);
                let loader = e.global::<u32>(TEXTURE_LOADER);
                e.call(CREATE_TEXTURE, &args![loader, text, normal, 1u32, 1u32]);
                let text = held(e, detail_path);
                e.call(NORMAL_FILE_NAME, &args![texture_path, text]);
                detailed = true;
            }
        }
        if !detailed {
            coordinate_texturing(e, data, part, model, tint);
            let own = frame.at(0x1a0);
            e.call(STRING_CONSTRUCTOR, &args![own]);
            let at = e.call(PART_LIST_AT, &args![data + 0xa4, part]).u32();
            let form = e.mem.u32(at);
            e.vcall(form, 0x14, &args![own]);
            if held(e, own) != 0 {
                e.call(STRING_ASSIGN, &args![texture_path, TEXTURES_FOLDER]);
                let text = held(e, own);
                e.call(STRING_APPEND, &args![texture_path, text]);
            }
            e.call(STRING_DESTRUCTOR, &args![own]);
        }
        if held(e, texture_path) != 0 {
            let text = held(e, texture_path);
            let loader = e.global::<u32>(TEXTURE_LOADER);
            e.call(CREATE_TEXTURE, &args![loader, text, image, 1u32, 1u32]);
        }
        if held(e, image) != 0 {
            let texture = held(e, image);
            e.vcall(shader, SLOT_SHADER_SET_TEXTURE, &args![0u32, texture]);
            let texture = held(e, image);
            let holder = fn_00656d80(e, Ptr::new(texture));
            let count = e.call(LIST_NEXT, &args![holder]).u32();
            let flag = u32::from(count == 5 || count == 6 || count == 1);
            e.mem.set_u8(frame.at(0x145), flag as u8);
            e.call(SET_SHADER_FLAG, &args![shader, 0u32, flag]);
        } else if e.call(SHADER_HAS_TEXTURE, &args![shader, 0u32]).u32() == 0 {
            let texture = e.call(DEFAULT_TEXTURE, &args![]).u32();
            e.vcall(shader, SLOT_SHADER_SET_TEXTURE, &args![0u32, texture]);
        }
        if e.mem.u8(data + 0xd4) != 0 && *kind == 0xe {
            let texture = held(e, tint);
            e.vcall(shader, SLOT_SHADER_SET_TINT, &args![1u32, texture]);
            let texture = if held(e, normal) != 0 {
                held(e, normal)
            } else {
                e.call(DEFAULT_NORMAL_TEXTURE, &args![]).u32()
            };
            e.vcall(shader, SLOT_SHADER_SET_TEXTURE, &args![1u32, texture]);
            e.call(SET_SHADER_FLAG, &args![shader, 0xau32, 1u32]);
            let second = e.call(GET_PROPERTY, &args![geometry, 2u32]).u32();
            if second != 0 {
                let current = e.call(PROPERTY_VALUE, &args![second]).f64();
                // FCOMP with the double 1.0: below it (ordered) sets the new value.
                if current < f64::from_bits(e.mem.u64(ONE_DOUBLE)) {
                    let value = x87_quiet_bits(e.global::<u32>(PROPERTY_NEW_VALUE));
                    e.call(PROPERTY_SET_VALUE, &args![second, value]);
                }
            }
            if held(e, texture_path) != 0 {
                let text = held(e, texture_path);
                let formatted = frame.at(0x3bc);
                let found = frame.at(0x2b4);
                e.call(FORMAT_TEXT, &args![formatted, 0x104u32, TINT_FORMAT, text]);
                e.call(
                    FIND_ARCHIVE_NAME,
                    &args![found, formatted, TINT_SUFFIX, 1u32],
                );
                if e.mem.u8(found) as i8 != 0 {
                    let temporary = frame.at(0x3c0);
                    e.call(POINTER_CONSTRUCTOR, &args![temporary, 0u32]);
                    let loader = e.global::<u32>(TEXTURE_LOADER);
                    e.call(CREATE_TEXTURE, &args![loader, found, temporary, 1u32, 1u32]);
                    let texture = held(e, temporary);
                    e.vcall(shader, SLOT_SET_TEXTURE, &args![2u32, 0u32, texture]);
                    e.call(POINTER_DESTRUCTOR, &args![temporary]);
                }
            }
        }
        let cleared = e.call(POINTER_ASSIGN, &args![normal, 0u32]).u32();
        e.call(POINTER_ASSIGN_FROM, &args![tint, cleared]);
        e.call(POINTER_ASSIGN, &args![image, 0u32]);
        if crate::units::fallout_shared_facegen::bsfacegenmodel::face_check::enabled() {
            // Diagnostic only (`NV_FACE_CHECK`, not exe behaviour).
            crate::units::fallout_shared_facegen::bsfacegenmodel::face_check::shader_textures(
                e, part, geometry, shader,
            );
        }
        e.call(PRECACHE_FACE_GEOMETRY, &args![geometry, *kind, 1u32]);
        e.vcall(shader, SLOT_SHADER_UPDATE, &args![]);
    }
    let eye_left = frame.at(0x188);
    let eye_right = frame.at(0x20);
    let accessory = frame.at(0x170);
    let hair = frame.at(0x154);
    let mut index = 0u32;
    while index < e.call(CHILD_COUNT, &args![node]).u32() {
        let child = e.call(CHILD_AT, &args![node, index]).u32();
        if child != 0 {
            let mut keep = 1u32;
            let is_named = |e: &mut Engine, name: u32| {
                let holder = e.call(NAME_HOLDER, &args![child]).u32();
                e.call(NAME_EQUALS, &args![holder, name]).bool()
            };
            if is_named(e, eye_left) || is_named(e, eye_right) {
                keep = 0;
                e.call(PREPARE_OBJECT, &args![child, 0u32, 0u32]);
            } else {
                let text = e.call(FIXED_STRING_TEXT, &args![hair]).u32();
                let length = e.call(STRLEN, &args![text]).u32();
                let hair_text = e.call(FIXED_STRING_TEXT, &args![hair]).u32();
                let holder = e.call(NAME_HOLDER, &args![child]).u32();
                let child_text = e.call(FIXED_STRING_TEXT, &args![holder]).u32();
                let differs = e.call(STRNCMP, &args![child_text, hair_text, length]).u32() != 0;
                if !differs || is_named(e, accessory) {
                    e.call(PREPARE_OBJECT, &args![child, 0u32, 0u32]);
                    let property = e.call(GET_PROPERTY, &args![child, 3u32]).u32();
                    let shader = shader_of_type(e, property, 8);
                    if shader != 0 {
                        set_shader_color(e, data, shader, frame.at(0x3f4));
                    } else {
                        let shader = shader_of_type(e, property, 0xc);
                        if shader != 0 {
                            set_shader_color(e, data, shader, frame.at(0x404));
                        }
                    }
                }
            }
            e.call(PRECACHE_FACE_GEOMETRY, &args![child, *kind, keep]);
        }
        index = index.wrapping_add(1);
    }
    e.call(UPDATE_PROPERTIES, &args![node]);
    let update = frame.at(0x180);
    e.call(
        UPDATE_DATA_CONSTRUCTOR,
        &args![update, 0.0f32.to_bits(), 0u32, 0u32],
    );
    e.call(UPDATE_WITH, &args![node, update]);
    e.call(UNLOCK, &args![]);
}

/// `004bc480` (unnamed; `thiscall`, one word): what the value setter runs with 1 after the
/// store.
const AFTER_SET_004BC480: u32 = 0x004b_c480;

// Translated from 00654700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores `value` at +0x38 (`FLD`/`FSTP`,
/// a signalling NaN stored quiet), then `004bc480` on `this` with 1.
pub fn fn_00654700(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0x38, x87_quiet_bits(value.to_bits()));
    e.call(AFTER_SET_004BC480, &args![this, 1u32]);
}

/// The pointer object `00656db0` reads (`011f9510`).
const POINTER_GLOBAL_011F9510: u32 = 0x011f_9510;

// Translated from 00656db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): `00559450` on the global at `011f9510`
/// (the word it holds).
pub fn fn_00656db0(e: &mut Engine) -> u32 {
    e.call(CALLEE_00559450, &args![POINTER_GLOBAL_011F9510])
        .u32()
}

/// Callees of [`fn_00656dc0`]: the bound's centre setter (`0098ddd0`; `thiscall`: point
/// address) and radius setter (`0063f790`; `thiscall`: float), the point default
/// constructor (`006815c0`), the iterator's point at an index (`00534570`; `thiscall`:
/// index, out point), its advance (`00657be0`), point addition and subtraction
/// (`00439e90` / `00439ef0`; `thiscall`: out point, other point; answer the out point),
/// scalar times point (`004a3760`; cdecl: out point, float, point), `NiPoint3::Dot`
/// (`004b6190`, engine map; `thiscall`: other point; ST0) and the square root
/// (`004579e0`; cdecl float; ST0).
const BOUND_SET_CENTER: u32 = 0x0098_ddd0;
const BOUND_SET_RADIUS: u32 = 0x0063_f790;
const POINT_DEFAULT: u32 = 0x0068_15c0;
const ITERATOR_POINT_AT: u32 = 0x0053_4570;
const ITERATOR_ADVANCE: u32 = 0x0065_7be0;
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
const POINT_SCALE: u32 = 0x004a_3760;
const POINT_DOT: u32 = 0x004b_6190;
const SQUARE_ROOT: u32 = 0x0045_79e0;
/// The `float` factor of the centre (`01016248`, read at run time).
const CENTER_FACTOR: u32 = 0x0101_6248;
/// The point the empty bound is centred on (`011f426c`).
const BOUND_ZERO_POINT: u32 = 0x011f_426c;

// Translated from 00656dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: bound, point iterator, count). With `count < 1`
/// (signed) the bound gets the point at `011f426c` as centre and radius 0.0. Otherwise
/// the box of the `count` points (the first through `00534570(0)`, the rest by copying
/// the iterator and stepping it, `00656fd0` / `00657be0`; a coordinate replaces the
/// minimum when below it and the maximum when above it, never for a NaN) gives the
/// centre `(min + max) * factor` (`01016248`), and the radius is the square root
/// (`004579e0`) of the largest `Dot(p - centre, p - centre)` over all points (start 0.0,
/// replaced when below, as a `float`).
pub fn fn_00656dc0(e: &mut Engine, bound: u32, iterator: Ptr, count: i32) {
    if count < 1 {
        e.call(BOUND_SET_CENTER, &args![bound, BOUND_ZERO_POINT]);
        e.call(BOUND_SET_RADIUS, &args![bound, 0u32]);
        return;
    }
    // The frame from EBP - 0x78; `at(k)` is EBP - k.
    e.with_stack(0x78, |e, frame| {
        let at = |k: u32| frame.addr() + 0x78 - k;
        let (center, best) = (at(0x10), at(0x4));
        let (min, point, max) = (at(0x44), at(0x38), at(0x50));
        let (walk, walk_again) = (at(0x20), at(0x2c));
        e.call(POINT_DEFAULT, &args![center]);
        e.mem.set_u32(best, 0);
        e.call(POINT_DEFAULT, &args![min]);
        e.call(POINT_DEFAULT, &args![point]);
        e.call(ITERATOR_POINT_AT, &args![iterator, 0u32, min]);
        for word in [0, 4, 8] {
            let value = e.mem.u32(min + word);
            e.mem.set_u32(max + word, value);
        }
        for word in [0, 4, 8] {
            let value = e.mem.u32(iterator.addr().wrapping_add(word));
            e.mem.set_u32(walk + word, value);
        }
        let mut index = 1;
        while index < count {
            fn_00656fd0(e, Ptr::new(walk), Ptr::new(point));
            for word in [0, 4, 8] {
                let p = e.mem.u32(point + word);
                if x87_load32(p) < x87_load32(e.mem.u32(min + word)) {
                    e.mem.set_u32(min + word, x87_quiet_bits(p));
                }
            }
            for word in [0, 4, 8] {
                let p = e.mem.u32(point + word);
                if x87_load32(p) > x87_load32(e.mem.u32(max + word)) {
                    e.mem.set_u32(max + word, x87_quiet_bits(p));
                }
            }
            index += 1;
            e.call(ITERATOR_ADVANCE, &args![walk]);
        }
        let sum = e.call(POINT_ADD, &args![min, at(0x6c), max]).u32();
        let factor = x87_quiet_bits(e.mem.u32(CENTER_FACTOR));
        let scaled = e.call(POINT_SCALE, &args![at(0x78), factor, sum]).u32();
        for word in [0, 4, 8] {
            let value = e.mem.u32(scaled.wrapping_add(word));
            e.mem.set_u32(center + word, value);
        }
        for word in [0, 4, 8] {
            let value = e.mem.u32(iterator.addr().wrapping_add(word));
            e.mem.set_u32(walk_again + word, value);
        }
        let mut index = 0;
        while index < count {
            fn_00656fd0(e, Ptr::new(walk_again), Ptr::new(point));
            let difference = at(0x60);
            e.call(POINT_SUBTRACT, &args![point, difference, center]);
            let dot = x87_store32(e.call(POINT_DOT, &args![difference, difference]).st0);
            e.mem.set_u32(at(0x54), dot);
            if x87_load32(e.mem.u32(best)) < x87_load32(dot) {
                e.mem.set_u32(best, x87_quiet_bits(dot));
            }
            index += 1;
            e.call(ITERATOR_ADVANCE, &args![walk_again]);
        }
        e.call(BOUND_SET_CENTER, &args![bound, center]);
        let largest = x87_quiet_bits(e.mem.u32(best));
        let radius = x87_store32(e.call(SQUARE_ROOT, &args![largest]).st0);
        e.call(BOUND_SET_RADIUS, &args![bound, radius]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_46e0, fn_006546e0(Ptr) -> u32),
        entry!(0x0065_4700, fn_00654700(Ptr, f32)),
        entry!(0x0065_6fd0, fn_00656fd0(Ptr, Ptr)),
        entry!(0x0065_5270, fn_00655270() -> u32),
        entry!(0x0065_6d80, fn_00656d80(Ptr) -> u32),
        entry!(0x0065_5320, fn_00655320(u32, u32, u32, u8)),
        entry!(0x0065_58b0, fn_006558b0(u32, u32, u32, u8)),
        entry!(0x0065_4720, fn_00654720(u32, u32, u32, u8) -> bool),
        entry!(0x0065_5fa0, fn_00655fa0(u32, u32)),
        entry!(0x0065_6db0, fn_00656db0() -> u32),
        entry!(0x0065_6dc0, fn_00656dc0(u32, Ptr, i32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_006546e0_passes_this_plus_d0_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00559450, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_46e0, &args![0x4100u32]).u32(), 0x1485);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00559450).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0xd0]);
    }
}

#[cfg(test)]
mod b037_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn point_copy_reads_through_the_pointer() {
        let mut e = Engine::new();
        let source = e.mem.alloc(12);
        for index in 0..3 {
            e.mem.set_u32(source + 4 * index, 0x10 + index);
        }
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, source);
        let output = e.mem.alloc(16);
        e.call(0x0065_6fd0, &args![this, output]);
        assert_eq!(
            [
                e.mem.u32(output),
                e.mem.u32(output + 4),
                e.mem.u32(output + 8)
            ],
            [0x10, 0x11, 0x12]
        );
        assert_eq!(e.mem.u32(output + 12), 0);
    }

    #[test]
    fn global_member_or_a_null_temporary() {
        let mut e = Engine::new();
        e.register(CALLEE_00559450, |_, a| (a[0] ^ 1).into_ret());
        e.register(POINTER_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(POINTER_DESTRUCTOR, |_, _| Ret::default());
        e.map(FACE_GEN_GLOBAL, 4);
        e.set_global(FACE_GEN_GLOBAL, 0x8000u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_5270, &args![]).u32(), 0x9199);
        assert!(calls(&e, POINTER_CONSTRUCTOR).is_empty());
        e.set_global(FACE_GEN_GLOBAL, 0u32);
        let answer = e.call(0x0065_5270, &args![]).u32();
        let temporary = calls(&e, POINTER_CONSTRUCTOR)[0][0];
        assert_eq!(answer, temporary ^ 1);
        assert_eq!(calls(&e, POINTER_DESTRUCTOR), vec![vec![temporary]]);
    }

    #[test]
    fn member_getter_answers_zero_without_a_member() {
        let mut e = Engine::new();
        e.register(MEMBER_GETTER, |_, a| (a[0] + 2).into_ret());
        let this = e.mem.alloc(0x28);
        assert_eq!(e.call(0x0065_6d80, &args![this]).u32(), 0);
        e.mem.set_u32(this + 0x24, 0x500);
        assert_eq!(e.call(0x0065_6d80, &args![this]).u32(), 0x502);
    }
}

#[cfg(test)]
mod face_gen_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    const NAME_GETTER: u32 = 0x0330_0000;
    const ATTACH_CHILD: u32 = 0x0330_0010;
    const SHADER_SET: u32 = 0x0330_0020;
    const SHADER_SLOT_100: u32 = 0x0330_0030;
    const SHADER_SLOT_FC: u32 = 0x0330_0040;
    const SHADER_SLOT_C0: u32 = 0x0330_0050;
    const BY_NAME: u32 = 0x0330_0060;
    const MODEL_DATA_GETTER: u32 = 0x0330_0070;
    const TEXTURE_NAME_GETTER: u32 = 0x0330_0080;

    /// An engine with every callee of the FaceGen functions doubled: the holder getter and
    /// the identity behave as in the exe, everything else answers 0 unless a test says
    /// otherwise.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let quiet = [
            MEMORY_SCOPE_BEGIN,
            MEMORY_SCOPE_END,
            STRING_CONSTRUCTOR,
            STRING_DESTRUCTOR,
            STRING_FORMAT,
            POINTER_CONSTRUCTOR,
            POINTER_DESTRUCTOR,
            POINTER_ASSIGN,
            MATRIX_ROTATION,
            EGM_FILE_NAME,
            TRI_FILE_NAME,
            ALTERNATE_FILE_NAME,
            LOW_LOD_FILE_NAME,
            ARCHIVE_SPLIT_PATH,
            ARCHIVE_FIND,
            FACE_GEN_MODEL,
            MODEL_CLONE_GEOMETRY,
            FIXED_STRING_CONSTRUCTOR,
            FIXED_STRING_DESTRUCTOR,
            SET_NAME,
            TEXTURE_PATH,
            CREATE_TEXTURE,
            GET_PROPERTY,
            SHADER_TYPE,
            ATTACH_PROPERTY,
            REMOVE_PROPERTY,
            IS_SKINNED,
            RESET_SKIN,
            SET_TRANSLATE,
            SET_ROTATE,
            SECOND_PART_SETUP,
            PART_NAME,
            APPLY_PART_FLOAT,
            PART_LIST_COUNT,
            PART_LIST_AT,
            FORM_IS_VALID,
            SETTING_STRING,
            MATERIAL_PROPERTY_TYPE,
            OPERATOR_NEW,
            MATERIAL_PROPERTY_CONSTRUCTOR,
            MATERIAL_NAME,
            FIND_SHADER,
            SET_SHADER_FLAG,
            SHADER_UPDATE,
            EYES_ENABLED,
            LINK_EYES,
            LIST_NEXT,
            SWAP_TEXTURES,
            LOCK,
            UNLOCK,
            DETAIL_VALUE,
            DETAIL_ROUND,
            COPY_TEXT,
            FORMAT_TEXT,
            FIND_ARCHIVE_NAME,
            STRLEN,
            STRNCMP,
            STRING_SET,
            FIXED_STRING_TEXT,
            STRING_ASSIGN,
            STRING_APPEND,
            GEOMETRY_FACE_GEN_MODEL,
            PREPARE_OBJECT,
            POINTER_ASSIGN_FROM,
            DETAIL_FILE_NAME,
            APPLY_COORDINATE_TEXTURING,
            COORDINATE_SOURCE,
            NORMAL_FILE_NAME,
            SHADER_HAS_TEXTURE,
            DEFAULT_TEXTURE,
            DEFAULT_NORMAL_TEXTURE,
            PROPERTY_VALUE,
            PROPERTY_SET_VALUE,
            PRECACHE_FACE_GEOMETRY,
            CHILD_COUNT,
            CHILD_AT,
            NAME_EQUALS,
            COLOR_CONSTRUCTOR,
            UPDATE_PROPERTIES,
            UPDATE_DATA_CONSTRUCTOR,
            UPDATE_WITH,
            ATTACH_CHILD,
            SHADER_SET,
            SHADER_SLOT_100,
            SHADER_SLOT_FC,
            SHADER_SLOT_C0,
            BY_NAME,
            MEMBER_GETTER,
        ];
        for address in quiet {
            e.register(address, |_, _| Ret::default());
        }
        e.register(CALLEE_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(IDENTITY, |_, a| a[0].into_ret());
        e.register(POINTER_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(NAME_HOLDER, |_, a| (a[0] + 8).into_ret());
        e.register(FIXED_STRING_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(NAME_GETTER, |_, a| (a[0] | 0x0100_0000).into_ret());
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (value as i32 as u32).into_ret()
        });
        e.map(FACE_GEN_GLOBAL, 4);
        e.map(TEXTURE_LOADER, 4);
        e.set_global(TEXTURE_LOADER, 0x4400u32);
        e.map(HEAD_ROTATION_ANGLE, 4);
        e.set_global(HEAD_ROTATION_ANGLE, (-1.5f32).to_bits());
        e.call_log = Some(vec![]);
        e
    }

    /// An object whose virtual table holds `slots` (offset, function) pairs.
    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x110);
        for (offset, function) in slots {
            e.mem.set_u32(vtable + offset, *function);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Head and skinned holders, each holding a node whose slot `0xDC` is recorded.
    fn holders(e: &mut Engine) -> (u32, u32) {
        let head_node = object(e, 8, &[(0xdc, ATTACH_CHILD)]);
        let skinned_node = object(e, 8, &[(0xdc, ATTACH_CHILD)]);
        let head = e.mem.alloc(4);
        e.mem.set_u32(head, head_node);
        let skinned = e.mem.alloc(4);
        e.mem.set_u32(skinned, skinned_node);
        (head, skinned)
    }

    /// The clone double: stores `geometry` into the pointer and succeeds.
    fn clone_into(e: &mut Engine, geometry: u32) {
        e.register_double(MODEL_CLONE_GEOMETRY, move |e, a| {
            e.mem.set_u32(a[2], geometry);
            1u32.into_ret()
        });
    }

    #[test]
    fn head_halves_are_built_and_attached() {
        let mut e = engine();
        let (head, skinned) = holders(&mut e);
        let data = e.mem.alloc(0x100);
        let part = e.mem.alloc(0x40);
        let model_object = object(&mut e, 4, &[(0x14, NAME_GETTER)]);
        e.mem.set_u32(part + 0x24, e.mem.u32(model_object));
        e.mem.set_u32(data + 0x80, part);
        e.mem.set_u32(data + 0x88, 0.75f32.to_bits());
        e.register(ARCHIVE_FIND, |_, _| 1u32.into_ret());
        e.register(FACE_GEN_MODEL, |_, a| (0x9000 + a[5]).into_ret());
        clone_into(&mut e, 0x5000);
        e.call(0x0065_5320, &args![head, skinned, data, 0u32]);
        let models = calls(&e, FACE_GEN_MODEL);
        assert_eq!(models.len(), 2);
        assert_eq!((models[0][5], models[1][5]), (0, 1));
        assert_eq!(calls(&e, MODEL_CLONE_GEOMETRY).len(), 2);
        assert_eq!(calls(&e, SECOND_PART_SETUP), vec![vec![0x5000, 1]]);
        assert_eq!(
            calls(&e, PART_NAME)
                .iter()
                .map(|c| c[1])
                .collect::<Vec<_>>(),
            vec![0, 1]
        );
        assert_eq!(
            calls(&e, APPLY_PART_FLOAT)[0],
            vec![0x5000, 0.75f32.to_bits()]
        );
        assert_eq!(calls(&e, ATTACH_CHILD).len(), 2);
        assert_eq!(calls(&e, ATTACH_CHILD)[0][1..], [0x5000, 1]);
        assert_eq!(calls(&e, ATTACH_CHILD)[0][0], e.mem.u32(head));
        // No material property yet: the default one is attached each time.
        assert_eq!(calls(&e, ATTACH_PROPERTY).len(), 2);
        let scope = &calls(&e, MEMORY_SCOPE_BEGIN)[0];
        assert_eq!(scope[1..], [0x37, 1, FACE_GEN_MANAGER_SOURCE, 0xb3b]);
        assert_eq!(calls(&e, STRING_DESTRUCTOR).len(), 5);
        assert_eq!(calls(&e, MEMORY_SCOPE_END).len(), 1);
    }

    #[test]
    fn head_without_halves_in_the_archive_takes_the_whole_part() {
        let mut e = engine();
        let (head, skinned) = holders(&mut e);
        let data = e.mem.alloc(0x100);
        let part = e.mem.alloc(0x40);
        let model_object = object(&mut e, 4, &[(0x14, NAME_GETTER)]);
        e.mem.set_u32(part + 0x24, e.mem.u32(model_object));
        e.mem.set_u32(data + 0x80, part);
        e.register(FACE_GEN_MODEL, |_, _| 0x9000u32.into_ret());
        e.call(0x0065_5320, &args![head, skinned, data, 0u32]);
        let indices: Vec<u32> = calls(&e, EGM_FILE_NAME).iter().map(|c| c[2]).collect();
        assert_eq!(indices, vec![0, 0xffff_ffff]);
        assert_eq!(calls(&e, FACE_GEN_MODEL).len(), 1);
        // The clone left no geometry: nothing is attached.
        assert!(calls(&e, ATTACH_CHILD).is_empty());
        e.mem.set_u32(data + 0x80, 0);
        e.call(0x0065_5320, &args![head, skinned, data, 0u32]);
        assert_eq!(calls(&e, FACE_GEN_MODEL).len(), 1);
    }

    #[test]
    fn eyes_get_their_names_and_default_textures() {
        let mut e = engine();
        let (head, skinned) = holders(&mut e);
        let data = e.mem.alloc(0x100);
        let left = object(&mut e, 8, &[(0x14, NAME_GETTER)]);
        let right = object(&mut e, 8, &[(0x14, NAME_GETTER)]);
        e.mem.set_u32(data + 0xd8, left);
        e.mem.set_u32(data + 0xdc, right);
        e.register(FACE_GEN_MODEL, |_, _| 0x9000u32.into_ret());
        clone_into(&mut e, 0x5000);
        e.register(IS_SKINNED, |_, _| 1u32.into_ret());
        e.call(0x0065_58b0, &args![head, skinned, data, 1u32]);
        let names: Vec<u32> = calls(&e, FIXED_STRING_CONSTRUCTOR)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(names, vec![EYE_LEFT_NAME, EYE_RIGHT_NAME]);
        let textures = calls(&e, CREATE_TEXTURE);
        assert_eq!(textures.len(), 2);
        assert_eq!(textures[0][..2], [0x4400, DEFAULT_EYE_TEXTURE]);
        assert_eq!(calls(&e, RESET_SKIN), vec![vec![0x5000, 0]; 2]);
        assert_eq!(calls(&e, SET_TRANSLATE), vec![vec![0x5000, ZERO_POINT]; 2]);
        assert_eq!(calls(&e, ATTACH_CHILD).len(), 2);
        let models = calls(&e, FACE_GEN_MODEL);
        assert_eq!(models[0][3..], [0, 1, 0xffff_ffff, 0]);
    }

    #[test]
    fn eyes_without_a_reset_attach_to_the_skinned_node() {
        let mut e = engine();
        let (head, skinned) = holders(&mut e);
        let data = e.mem.alloc(0x100);
        let left = object(&mut e, 8, &[(0x14, NAME_GETTER)]);
        e.mem.set_u32(data + 0xd8, left);
        let texture_form = e.mem.alloc(0x30);
        e.mem.set_u32(data + 0x8c, texture_form);
        e.register(FACE_GEN_MODEL, |_, _| 0x9000u32.into_ret());
        clone_into(&mut e, 0x5000);
        e.register(IS_SKINNED, |_, _| 1u32.into_ret());
        e.call(0x0065_58b0, &args![head, skinned, data, 0u32]);
        assert_eq!(calls(&e, TEXTURE_PATH), vec![vec![texture_form + 0x24]]);
        let attached = calls(&e, ATTACH_CHILD);
        assert_eq!(attached.len(), 1);
        assert_eq!(attached[0][0], e.mem.u32(skinned));
        assert!(calls(&e, RESET_SKIN).is_empty());
    }

    #[test]
    fn head_build_runs_every_valid_part_and_links_the_eyes() {
        let mut e = engine();
        let (head, skinned) = holders(&mut e);
        let data = e.mem.alloc(0x100);
        let form = object(&mut e, 8, &[(0x14, NAME_GETTER)]);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, form);
        e.register_double(PART_LIST_AT, move |_, _| element.into_ret());
        e.register(PART_LIST_COUNT, |_, _| 1u32.into_ret());
        e.register(FORM_IS_VALID, |_, _| 1u32.into_ret());
        let setting = e.mem.alloc(4);
        e.mem.set_u8(setting, 1);
        e.register_double(SETTING_STRING, move |_, _| setting.into_ret());
        e.register(ALTERNATE_FILE_NAME, |_, _| 0x7777u32.into_ret());
        e.register(FACE_GEN_MODEL, |_, _| 0x9000u32.into_ret());
        clone_into(&mut e, 0x5000);
        e.register(MATERIAL_PROPERTY_TYPE, |_, _| 5u32.into_ret());
        e.register(OPERATOR_NEW, |_, _| 0x6600u32.into_ret());
        e.register(MATERIAL_PROPERTY_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(EYES_ENABLED, |_, _| 1u32.into_ret());
        e.map(EYE_LINK_VALUE, 4);
        e.set_global(EYE_LINK_VALUE, 0.1f32.to_bits());
        let answer = e.call(0x0065_4720, &args![head, skinned, data, 0u32]);
        assert!(answer.bool());
        let models = calls(&e, FACE_GEN_MODEL);
        assert_eq!(models.len(), 6);
        // Parts 0 and 1 use the alternate file, parts 2 to 5 do not.
        assert_eq!(models[0][3], 0x7777);
        assert_eq!(models[1][3], 0x7777);
        assert!(models[2..].iter().all(|m| m[3] == 0));
        assert_eq!(calls(&e, LOW_LOD_FILE_NAME).len(), 1);
        assert_eq!(calls(&e, ATTACH_CHILD).len(), 6);
        // Materials for parts 0 and 1; the default material for parts 3 and 4.
        assert_eq!(
            calls(&e, MATERIAL_PROPERTY_CONSTRUCTOR),
            vec![vec![0x6600]; 2]
        );
        assert_eq!(calls(&e, ATTACH_PROPERTY).len(), 4);
        assert_eq!(
            calls(&e, LINK_EYES),
            vec![vec![0x5000, 0x5000, 0.1f32.to_bits(), 0, 0]]
        );
        let scope = &calls(&e, MEMORY_SCOPE_BEGIN)[0];
        assert_eq!(scope[4], 0xa48);
        assert_eq!(calls(&e, MEMORY_SCOPE_BEGIN).len(), 1);
    }

    #[test]
    fn head_build_runs_the_halves_eyes_and_accessories() {
        let mut e = engine();
        let (head, skinned) = holders(&mut e);
        let data = e.mem.alloc(0x100);
        let named = object(&mut e, 0x40, &[(0x14, NAME_GETTER)]);
        e.mem.set_u32(named + 0x24, e.mem.u32(named));
        e.mem.set_u32(data + 0x80, named);
        e.mem.set_u32(data + 0xd8, named);
        e.mem.set_u32(data + 0xdc, named);
        // One accessory: the list at +0xE4 holds the accessory; the walk ends after it.
        e.mem.set_u32(data + 0xe4, named);
        e.register(FACE_GEN_MODEL, |_, _| 0x9000u32.into_ret());
        clone_into(&mut e, 0x5000);
        assert!(e
            .call(0x0065_4720, &args![head, skinned, data, 0u32])
            .bool());
        let lines: Vec<u32> = calls(&e, MEMORY_SCOPE_BEGIN).iter().map(|c| c[4]).collect();
        assert_eq!(lines, vec![0xa48, 0xb3b, 0xba7]);
        assert_eq!(calls(&e, SWAP_TEXTURES), vec![vec![named + 0x24, 0x5000]]);
        assert_eq!(calls(&e, LIST_NEXT), vec![vec![data + 0xe4]]);
        let last_model = calls(&e, FACE_GEN_MODEL).last().unwrap().clone();
        assert_eq!(last_model[3..], [0, 1, 0xffff_ffff, 1]);
    }

    #[test]
    fn head_update_needs_a_node_and_data() {
        let mut e = engine();
        e.call(0x0065_5fa0, &args![0u32, 0x100u32]);
        assert_eq!(calls(&e, LOCK).len(), 1);
        assert_eq!(calls(&e, UNLOCK).len(), 1);
        assert!(calls(&e, DETAIL_VALUE).is_empty());
        assert_eq!(calls(&e, FIXED_STRING_DESTRUCTOR).len(), 4);
        assert_eq!(calls(&e, MEMORY_SCOPE_END).len(), 1);
    }

    #[test]
    fn head_update_colours_hair_and_skips_eyes() {
        let mut e = engine();
        let node = object(&mut e, 8, &[(0x9c, BY_NAME)]);
        let data = e.mem.alloc(0x100);
        e.mem.set_u32(data + 0x84, 0x00ff_8000);
        e.map(COLOR_DIVISOR, 8);
        e.mem.set_u64(COLOR_DIVISOR, 255.0f64.to_bits());
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, 0x7100);
        e.register_double(PART_LIST_AT, move |_, _| element.into_ret());
        let shader = object(&mut e, 0x100, &[(0xc0, SHADER_SLOT_C0)]);
        e.register(CHILD_COUNT, |_, _| 2u32.into_ret());
        e.register(CHILD_AT, |_, a| (0x8000 + a[1]).into_ret());
        // Child 0 is the left eye; child 1 is the hair.
        e.register(NAME_EQUALS, |_, a| u32::from(a[0] == 0x8008).into_ret());
        e.register(STRNCMP, |_, a| u32::from(a[0] != 0x8009).into_ret());
        e.register(FIXED_STRING_TEXT, |_, a| a[0].into_ret());
        e.register_double(GET_PROPERTY, move |_, a| {
            (if a[0] == 0x8001 { shader } else { 0 }).into_ret()
        });
        e.register(SHADER_TYPE, |_, _| 8u32.into_ret());
        e.register(COLOR_CONSTRUCTOR, |e, a| {
            for index in 0..4 {
                e.mem.set_u32(a[0] + 4 * index, a[1 + index as usize]);
            }
            a[0].into_ret()
        });
        e.register(DETAIL_VALUE, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        e.register(DETAIL_ROUND, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).floor(),
            ..Ret::default()
        });
        e.call(0x0065_5fa0, &args![node, data]);
        assert_eq!(calls(&e, DETAIL_ROUND), vec![vec![2.5f32.to_bits()]]);
        // No part is found by name: nothing is precached for the parts.
        assert_eq!(calls(&e, BY_NAME).len(), 6);
        let precached = calls(&e, PRECACHE_FACE_GEOMETRY);
        assert_eq!(precached, vec![vec![0x8000, 1, 0], vec![0x8001, 1, 1]]);
        assert_eq!(
            calls(&e, PREPARE_OBJECT),
            vec![vec![0x8000, 0, 0], vec![0x8001, 0, 0]]
        );
        let colors = calls(&e, COLOR_CONSTRUCTOR);
        assert_eq!(
            colors[0][1..],
            [
                0,
                x87_store32(128.0 / 255.0),
                1.0f32.to_bits(),
                1.0f32.to_bits()
            ]
        );
        assert_eq!(e.mem.f32(shader + 0x8c), 1.0);
        assert_eq!(calls(&e, SHADER_SLOT_C0), vec![vec![shader]]);
        assert_eq!(calls(&e, UPDATE_PROPERTIES), vec![vec![node]]);
        let update = &calls(&e, UPDATE_DATA_CONSTRUCTOR)[0];
        assert_eq!(update[1..], [0, 0, 0]);
        assert_eq!(calls(&e, UPDATE_WITH), vec![vec![node, update[0]]]);
        assert_eq!(calls(&e, UNLOCK).len(), 1);
    }

    #[test]
    fn head_update_textures_a_found_part() {
        let mut e = engine();
        let geometry = object(&mut e, 8, &[(0x1c, MODEL_DATA_GETTER)]);
        e.register(MODEL_DATA_GETTER, |_, _| 0x7200u32.into_ret());
        e.register(GEOMETRY_FACE_GEN_MODEL, |_, _| 0x7300u32.into_ret());
        // Only part 2's name finds the geometry.
        let names = e.mem.alloc(8 * 4);
        e.register_double(BY_NAME, move |e, a| {
            (if e.mem.u32(a[1]) == 0x7002 {
                geometry
            } else {
                0
            })
            .into_ret()
        });
        e.register(FIXED_STRING_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        let node = object(&mut e, 8, &[(0x9c, BY_NAME)]);
        let shader = object(
            &mut e,
            0x100,
            &[(0x100, SHADER_SLOT_100), (0xc0, SHADER_SLOT_C0)],
        );
        e.register_double(GET_PROPERTY, move |_, _| shader.into_ret());
        e.register(SHADER_TYPE, |_, _| 10u32.into_ret());
        // The part's texture form names its texture; the path then loads an image.
        let texture_form = object(&mut e, 8, &[(0x14, TEXTURE_NAME_GETTER)]);
        e.register_double(TEXTURE_NAME_GETTER, move |e, a| {
            e.mem.set_u32(a[1], 0x6100);
            Ret::default()
        });
        e.register(STRING_APPEND, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let image = e.mem.alloc(0x28);
        e.mem.set_u32(image + 0x24, 0x6300);
        e.register_double(CREATE_TEXTURE, move |e, a| {
            e.mem.set_u32(a[2], image);
            Ret::default()
        });
        e.register(MEMBER_GETTER, |_, _| 0x6400u32.into_ret());
        e.register(LIST_NEXT, |_, _| 5u32.into_ret());
        let data = e.mem.alloc(0x100);
        // The texture list (+0xA4) holds the texture form for every part.
        let texture_list = e.mem.alloc(4);
        e.mem.set_u32(texture_list, texture_form);
        let lists = names;
        e.register_double(PART_LIST_AT, move |e, a| {
            if a[0] == data + 0xa4 {
                return texture_list.into_ret();
            }
            e.mem.set_u32(lists + 4 * a[1], 0x7000 + a[1]);
            (lists + 4 * a[1]).into_ret()
        });
        e.call(0x0065_5fa0, &args![node, data]);
        assert_eq!(calls(&e, PREPARE_OBJECT), vec![vec![geometry, 0, 0]]);
        assert_eq!(
            calls(&e, APPLY_COORDINATE_TEXTURING),
            vec![vec![0x7300, data, calls(&e, POINTER_ASSIGN_FROM)[0][0], 0]]
        );
        assert_eq!(calls(&e, STRING_APPEND)[0][1], 0x6100);
        let set = calls(&e, SHADER_SLOT_100);
        assert_eq!(set, vec![vec![shader, 0, image]]);
        assert_eq!(calls(&e, MEMBER_GETTER), vec![vec![0x6300]]);
        assert_eq!(calls(&e, LIST_NEXT), vec![vec![0x6400]]);
        assert_eq!(calls(&e, SET_SHADER_FLAG), vec![vec![shader, 0, 1]]);
        assert_eq!(
            calls(&e, PRECACHE_FACE_GEOMETRY),
            vec![vec![geometry, 1, 1]]
        );
        assert_eq!(calls(&e, SHADER_SLOT_C0), vec![vec![shader]]);
    }

    #[test]
    fn value_setter_stores_quiet_then_marks() {
        let mut e = Engine::new();
        e.register(AFTER_SET_004BC480, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x0065_4700, &args![this, 0x7f80_0005u32]);
        assert_eq!(e.mem.u32(this + 0x38), 0x7fc0_0005);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(AFTER_SET_004BC480, vec![this, 1])]);
        e.call(0x0065_4700, &args![this, 0.75f32]);
        assert_eq!(e.mem.u32(this + 0x38), 0.75f32.to_bits());
    }
}

#[cfg(test)]
mod n076_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn pointer_global_is_read_through_the_getter() {
        let mut e = Engine::new();
        e.register(CALLEE_00559450, |_, a| (a[0] + 1).into_ret());
        assert_eq!(e.call(0x0065_6db0, &[]).u32(), 0x011f_9511);
    }

    /// Doubles doing what the point helpers do, on floats in memory.
    fn bound_engine(points: &[[f32; 3]]) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.mem.set_f32(CENTER_FACTOR, 0.5);
        for address in [BOUND_SET_CENTER, BOUND_SET_RADIUS, POINT_DEFAULT] {
            e.register(address, |_, _| Ret::default());
        }
        // The iterator: word 0 points at the current point (12 bytes each).
        let data = e.mem.alloc(12 * points.len() as u32 + 12);
        for (i, p) in points.iter().enumerate() {
            for (j, v) in p.iter().enumerate() {
                e.mem.set_f32(data + 12 * i as u32 + 4 * j as u32, *v);
            }
        }
        e.register(ITERATOR_POINT_AT, |e, a| {
            let source = e.mem.u32(a[0]) + 12 * a[1];
            for w in 0..3 {
                let v = e.mem.u32(source + 4 * w);
                e.mem.set_u32(a[2] + 4 * w, v);
            }
            Ret::default()
        });
        e.register(ITERATOR_ADVANCE, |e, a| {
            let next = e.mem.u32(a[0]) + 12;
            e.mem.set_u32(a[0], next);
            Ret::default()
        });
        e.register(POINT_ADD, |e, a| {
            for w in 0..3 {
                let v = e.mem.f32(a[0] + 4 * w) + e.mem.f32(a[2] + 4 * w);
                e.mem.set_f32(a[1] + 4 * w, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_SUBTRACT, |e, a| {
            for w in 0..3 {
                let v = e.mem.f32(a[0] + 4 * w) - e.mem.f32(a[2] + 4 * w);
                e.mem.set_f32(a[1] + 4 * w, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_SCALE, |e, a| {
            for w in 0..3 {
                let v = f32::from_bits(a[1]) * e.mem.f32(a[2] + 4 * w);
                e.mem.set_f32(a[0] + 4 * w, v);
            }
            a[0].into_ret()
        });
        e.register(POINT_DOT, |e, a| {
            let dot: f32 = (0..3)
                .map(|w| e.mem.f32(a[0] + 4 * w) * e.mem.f32(a[1] + 4 * w))
                .sum();
            f64::from(dot).into_ret()
        });
        e.register(SQUARE_ROOT, |_, a| {
            f64::from(f32::from_bits(a[0]).sqrt()).into_ret()
        });
        let iterator = e.mem.alloc(12);
        e.mem.set_u32(iterator, data);
        (e, iterator)
    }

    #[test]
    fn bound_of_points_is_the_box_centre_and_farthest_point() {
        let points = [[0.0, 0.0, 0.0], [4.0, -2.0, 0.0], [2.0, 2.0, 6.0]];
        let (mut e, iterator) = bound_engine(&points);
        e.call_log = Some(vec![]);
        e.call(0x0065_6dc0, &args![0x5000u32, iterator, 3i32]);
        // The box walk starts at the iterator's own position, so it sees points 0, 0, 1:
        // box (0, -2, 0) .. (4, 0, 0), centre (2, -1, 0); the farthest is point 2 (0+9+36).
        let center = calls(&e, BOUND_SET_CENTER);
        assert_eq!(center.len(), 1);
        assert_eq!(center[0][0], 0x5000);
        assert_eq!(calls(&e, SQUARE_ROOT), vec![vec![45.0f32.to_bits()]]);
        assert_eq!(
            calls(&e, BOUND_SET_RADIUS),
            vec![vec![0x5000, 45.0f32.sqrt().to_bits()]]
        );
        assert_eq!(calls(&e, POINT_SCALE)[0][1], 0.5f32.to_bits());
        assert_eq!(calls(&e, ITERATOR_ADVANCE).len(), 5);
    }

    #[test]
    fn empty_bound_takes_the_zero_point() {
        let (mut e, iterator) = bound_engine(&[]);
        e.call_log = Some(vec![]);
        e.call(0x0065_6dc0, &args![0x5000u32, iterator, 0i32]);
        assert_eq!(
            calls(&e, BOUND_SET_CENTER),
            vec![vec![0x5000, BOUND_ZERO_POINT]]
        );
        assert_eq!(calls(&e, BOUND_SET_RADIUS), vec![vec![0x5000, 0]]);
        assert!(calls(&e, ITERATOR_POINT_AT).is_empty());
    }
}
