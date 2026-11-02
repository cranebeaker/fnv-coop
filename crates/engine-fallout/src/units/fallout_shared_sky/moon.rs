//! `fallout shared/sky/moon.cpp` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `Moon` is a sky object with two quads (the moon and its shadow, each
//! under its own `NiNode`), eight phase textures named after a prefix, and
//! the angles at which it fades in and out. The PC code uses the Xbox PDB's
//! offsets of the class.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `Moon` (Xbox PDB), 0x7C bytes: a `SkyObject` (vtable, `spRoot` +4)
    /// and the moon's nodes, meshes, textures and angles. Float fields are
    /// their bit patterns.
    pub struct Moon: 0x7c {
        /// `spRoot` (Xbox PDB, `SkyObject`): `NiPointer<NiNode>`.
        0x04 spRoot: u32,
        /// `spMoonNode` (Xbox PDB).
        0x08 spMoonNode: u32,
        /// `spShadowNode` (Xbox PDB).
        0x0C spShadowNode: u32,
        /// `spMoonMesh` (Xbox PDB): `NiPointer<NiTriShape>`.
        0x10 spMoonMesh: u32,
        /// `spShadowMesh` (Xbox PDB).
        0x14 spShadowMesh: u32,
        /// `strTextures` (Xbox PDB): eight `BSStringT<char>` (8 bytes each).
        0x18 strTextures: u32,
        /// `fAngleFadeStart` (Xbox PDB).
        0x58 fAngleFadeStart: u32,
        /// `fAngleFadeEnd` (Xbox PDB).
        0x5C fAngleFadeEnd: u32,
        /// `fAngleShadowEarlyFade` (Xbox PDB).
        0x60 fAngleShadowEarlyFade: u32,
        /// `fSpeed` (Xbox PDB).
        0x64 fSpeed: u32,
        /// `fZOffset` (Xbox PDB).
        0x68 fZOffset: u32,
        /// `iSize` (Xbox PDB).
        0x6C iSize: u32,
        /// `eUpdateMoonTexture` (Xbox PDB): `Moon::UpdateStatus`.
        0x70 eUpdateMoonTexture: u32,
        /// `fPos` (Xbox PDB): the moon's angle, degrees.
        0x74 fPos: u32,
        /// `fLastPosUpdate` (Xbox PDB): the hour of the last update.
        0x78 fLastPosUpdate: u32,
    }
}

/// `Moon`'s vtable (`0104ed28`, RTTI `.?AVMoon@@`).
const MOON_VTABLE: u32 = 0x0104_ed28;
/// `SkyObject`'s constructor (`0063f830`), destructor (`0063f8d0`) and
/// `SkyObject::Initialize` (`0063f940`, Xbox PDB name; one word).
const SKY_OBJECT_CONSTRUCTOR: u32 = 0x0063_f830;
const SKY_OBJECT_DESTRUCTOR: u32 = 0x0063_f8d0;
const SKY_OBJECT_INITIALIZE: u32 = 0x0063_f940;
/// `SkyObject::Update` as the moon calls it (`008d0600`; the engine map
/// gives the folded body the name `ProcessLists::PrintLists`; two words).
const SKY_OBJECT_UPDATE: u32 = 0x008d_0600;
/// The `NiPointer` constructor (`00633c90`), assignment (`0066b0d0`),
/// destructor (`0045cec0`) and read (`00559450`).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
const NI_POINTER_GET: u32 = 0x0055_9450;
/// The `BSStringT<char>` constructor (`004037b0`) and destructor
/// (`004037d0`), the formatted write (`00406f60`, cdecl), and the
/// `eh vector` iterators (`00ec782f` constructor, `00ec5fce` destructor;
/// stdcall).
const BS_STRING_CONSTRUCTOR: u32 = 0x0040_37b0;
const BS_STRING_DESTRUCTOR: u32 = 0x0040_37d0;
const BS_STRING_FORMAT: u32 = 0x0040_6f60;
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// The allocation scope the code opens around its work (`00404eb0`,
/// thiscall: 0x21, 1, file, line) and closes (`00404ee0`).
const SCOPE_OPEN: u32 = 0x0040_4eb0;
const SCOPE_CLOSE: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\Sky\Moon.cpp"`
/// (`0104ece4`).
const SOURCE_FILE: u32 = 0x0104_ece4;
/// `"\Textures\Sky\%s_%s.dds"` (`0104eccc`).
const TEXTURE_FORMAT: u32 = 0x0104_eccc;
/// The texture slot (offset in `Moon`) and the phase name the constructor
/// writes into it, in the code's order: `one_wan` (`0104ecc4`), `one_wax`
/// (`0104ecbc`), `half_wan` (`0104ecb0`), `half_wax` (`0104eca4`),
/// `three_wan` (`0104ec98`), `three_wax` (`0104ec8c`) and `full`
/// (`0104ec84`); the slot at +0x38 is not written.
const PHASE_NAMES: [(u32, u32); 7] = [
    (0x30, 0x0104_ecc4),
    (0x40, 0x0104_ecbc),
    (0x28, 0x0104_ecb0),
    (0x48, 0x0104_eca4),
    (0x20, 0x0104_ec98),
    (0x50, 0x0104_ec8c),
    (0x18, 0x0104_ec84),
];
/// `FLT_MAX` (`01016970`, a `float`): no position update yet.
const NO_UPDATE: u32 = 0x0101_6970;
/// `operator new` (`00401000`) and `operator delete` (`00401030`)
/// (cdecl), and `` `vector constructor iterator' `` (`00401050`,
/// stdcall: array, element size, count, constructor).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
/// The element constructors: `NiPoint3`/`NiPoint2` (`006815c0`) and
/// `NiColorA` (`004a7800`).
const POINT_ELEMENT_CONSTRUCTOR: u32 = 0x0068_15c0;
const COLOR_ELEMENT_CONSTRUCTOR: u32 = 0x004a_7800;
/// The value constructors: `NiPoint3` (`00416870`, three floats),
/// `NiPoint2` (`00452dc0`, two floats) and `NiColorA` (`00414430`, four
/// floats); each answers `this`.
const POINT3_CONSTRUCTOR: u32 = 0x0041_6870;
const POINT2_CONSTRUCTOR: u32 = 0x0045_2dc0;
const COLOR_A_CONSTRUCTOR: u32 = 0x0041_4430;
/// `-1.0` (`double`, `0101a6b0`).
const MINUS_ONE: u32 = 0x0101_a6b0;
/// Gamebryo's allocations (`00aa13e0` objects, `00aa1070` buffers).
const NI_NEW: u32 = 0x00aa_13e0;
const NI_ALLOC: u32 = 0x00aa_1070;
/// `NiTriShape::NiTriShape` (`00a74410`, Xbox PDB name; nine words) and
/// the `NiNode` constructor (`00a5ecb0`).
const NI_TRI_SHAPE_CONSTRUCTOR: u32 = 0x00a7_4410;
const NI_NODE_CONSTRUCTOR: u32 = 0x00a5_ecb0;
/// The node call the meshes and nodes are given 1 with (`004902f0`).
const NODE_SET_004902F0: u32 = 0x0049_02f0;
/// The alpha property: its constructor (`004391c0`, 0x1C bytes) and setters
/// (`0049ed90` with 1, `00439340` with 6, `00439390` with 0).
const ALPHA_PROPERTY_CONSTRUCTOR: u32 = 0x0043_91c0;
const ALPHA_SET_0049ED90: u32 = 0x0049_ed90;
const ALPHA_SET_00439340: u32 = 0x0043_9340;
const ALPHA_SET_00439390: u32 = 0x0043_9390;
/// `NiAVObject::AttachProperty` (`00439410`) and `GetProperty`
/// (`00a59d30`) (Xbox PDB names).
const ATTACH_PROPERTY: u32 = 0x0043_9410;
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// `BSShaderData::BSShaderData` (`00baf3c0`) and `BSShaderData::Convert`
/// (`00bb0180`, two words) (Xbox PDB names); the block is 0xB8 bytes.
const SHADER_DATA_CONSTRUCTOR: u32 = 0x00ba_f3c0;
const SHADER_DATA_CONVERT: u32 = 0x00bb_0180;
/// `Sky::GetInstance` (`0046dd00`, Xbox PDB name) and the sky's mode
/// (`00476c90`, thiscall).
const SKY_GET_INSTANCE: u32 = 0x0046_dd00;
const SKY_MODE: u32 = 0x0047_6c90;
/// `BSShaderManager::GetTexture` (`00b55840`, Xbox PDB name, cdecl: name,
/// 1, out pointer, 1, 0) and the texture's hand-over to the property
/// (`006348e0`, thiscall: texture).
const GET_TEXTURE: u32 = 0x00b5_5840;
const PROPERTY_SET_TEXTURE: u32 = 0x0063_48e0;
/// `"Textures\Sky\MoonShadow.dds"` (`0104ed3c`).
const MOON_SHADOW_TEXTURE: u32 = 0x0104_ed3c;
/// `NiMatrix3::MakeXRotation` (`00524ac0`) and the second rotation
/// (`004a0c90`), the translation (`004bc1f0`, three floats), the rotation
/// set (`0043fa80`) and the matrix product (`0043f8d0`, `NiMatrix3::
/// operator*`).
const MAKE_X_ROTATION: u32 = 0x0052_4ac0;
const MAKE_SECOND_ROTATION: u32 = 0x004a_0c90;
const SET_TRANSLATE: u32 = 0x004b_c1f0;
const SET_ROTATE: u32 = 0x0043_fa80;
const MATRIX_PRODUCT: u32 = 0x0043_f8d0;
/// `BSShaderManager::PrepareObject` (`00b57e30`, Xbox PDB name, cdecl).
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
/// The initial rotation (`01016b78`, a float: `-pi/2`) and height
/// (`010231a0`, a float: 512) of the meshes.
const INITIAL_ROTATION: u32 = 0x0101_6b78;
const MESH_HEIGHT: u32 = 0x0102_31a0;
/// The child attach slot of a node (`+0xDC`) and its detach slot
/// (`+0xE8`); a node's parent (`009611e0`).
const ATTACH_CHILD_SLOT: u32 = 0xdc;
const DETACH_CHILD_SLOT: u32 = 0xe8;
const GET_PARENT: u32 = 0x0096_11e0;
/// The flag setters `00635fe0` and `00636000` call (`0043b370` on the
/// object flags +0x30; `00633ba0` on the 16-bit flags +0x18).
const SET_OBJECT_FLAG: u32 = 0x0043_b370;
const SET_PROPERTY_FLAG: u32 = 0x0063_3ba0;

/// `180.0` (`double`, `0104ed58`).
const HALF_TURN: u32 = 0x0104_ed58;
/// `FLT_MAX` (`double`, `010231b0`), `90.0` (`float`, `010249d8`), `24.0`
/// (`010356d8`), `60.0` (`01012638`), `360.0` (`0103c348`), `pi/180`
/// (`01023128`) and `0.0` (`01012060`) (`double`).
const NO_UPDATE_DOUBLE: u32 = 0x0102_31b0;
const START_POSITION: u32 = 0x0102_49d8;
const HOURS_PER_DAY: u32 = 0x0103_56d8;
const SPEED_SCALE: u32 = 0x0101_2638;
const FULL_TURN: u32 = 0x0103_c348;
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
const ZERO: u32 = 0x0101_2060;
/// The calls of `Update`: whether the days passed changed (`0047c850` on
/// the word `011de45c`), the days passed (`00867de0` on the calendar
/// `011de7b8`), the sky's hour (`00966a20`), the sky's object (`0059bb30`)
/// and its value (`0084d030`), the combination of the shadow fade with it
/// (`0040ebd0`, cdecl), the visibility test (`006366b0`), the hide flag
/// (`00450f90`), the property's type (`00441110`) and the sky's colour
/// (`005a8080`); the moon phase index (`011cca80`).
const DAYS_CHANGED_OWNER: u32 = 0x011d_e45c;
const DAYS_CHANGED: u32 = 0x0047_c850;
const CALENDAR: u32 = 0x011d_e7b8;
const DAYS_PASSED: u32 = 0x0086_7de0;
const SKY_HOUR: u32 = 0x0096_6a20;
const SKY_OBJECT: u32 = 0x0059_bb30;
const SKY_OBJECT_VALUE: u32 = 0x0084_d030;
const COMBINE_FADE: u32 = 0x0040_ebd0;
const IS_SHOWN: u32 = 0x0063_66b0;
const SET_HIDDEN: u32 = 0x0045_0f90;
const PROPERTY_TYPE: u32 = 0x0044_1110;
const SKY_COLOR: u32 = 0x005a_8080;
const PHASE_INDEX: u32 = 0x011c_ca80;

/// The word an `NiPointer` at `slot` holds (through `00559450`).
fn pointer(e: &mut Engine, slot: u32) -> u32 {
    e.call(NI_POINTER_GET, &args![slot]).u32()
}

// Translated from 00634a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Moon::Moon` (Xbox PDB): the `SkyObject` constructor, the vtable, the
/// node and mesh pointers and the eight texture names constructed, seven
/// of them written as `"\Textures\Sky\<prefix>_<phase>.dds"` (inside the
/// allocation scope); the angles, speed, height offset and size taken from
/// the arguments, the position 0 and the last update `FLT_MAX`, the
/// pointers emptied and the texture update off. Returns `this`.
// Translation incomplete: 00634a70 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn moon_constructor(
    e: &mut Engine,
    this: Ptr<Moon>,
    prefix: Ptr,
    fade_start: f32,
    fade_end: f32,
    shadow_early_fade: f32,
    speed: f32,
    z_offset: f32,
    size: u32,
) -> Ptr<Moon> {
    let t = this.addr();
    e.call(SKY_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(t, MOON_VTABLE);
    for offset in [0x08u32, 0x0c, 0x10, 0x14] {
        e.call(NI_POINTER_CONSTRUCT, &args![t + offset, 0u32]);
    }
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            t + 0x18,
            8u32,
            8u32,
            BS_STRING_CONSTRUCTOR,
            BS_STRING_DESTRUCTOR
        ],
    );
    e.with_stack(0x10, |e, scope| {
        e.call(
            SCOPE_OPEN,
            &args![scope, 0x21u32, 1u32, SOURCE_FILE, 0x32u32],
        );
        for (offset, phase) in PHASE_NAMES {
            e.call(
                BS_STRING_FORMAT,
                &args![t + offset, TEXTURE_FORMAT, prefix, phase],
            );
        }
        e.set(
            this,
            Moon::fAngleFadeStart,
            x87_quiet_bits(fade_start.to_bits()),
        );
        e.set(
            this,
            Moon::fAngleFadeEnd,
            x87_quiet_bits(fade_end.to_bits()),
        );
        e.set(
            this,
            Moon::fAngleShadowEarlyFade,
            x87_quiet_bits(shadow_early_fade.to_bits()),
        );
        e.set(this, Moon::fSpeed, x87_quiet_bits(speed.to_bits()));
        e.set(this, Moon::fZOffset, x87_quiet_bits(z_offset.to_bits()));
        e.set(this, Moon::iSize, size);
        e.set(this, Moon::fPos, 0.0f32.to_bits());
        let no_update = x87_quiet_bits(e.global::<u32>(NO_UPDATE));
        e.set(this, Moon::fLastPosUpdate, no_update);
        for offset in [0x08u32, 0x0c, 0x10, 0x14] {
            e.call(NI_POINTER_ASSIGN, &args![t + offset, 0u32]);
        }
        e.set(this, Moon::eUpdateMoonTexture, 0);
        e.call(SCOPE_CLOSE, &args![scope]);
    });
    this
}

// Translated from 00634cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Moon`'s scalar deleting destructor (Xbox PDB): the destructor and
/// `operator delete` when `flags & 1`. Returns `this`.
pub fn moon_scalar_deleting_destructor(e: &mut Engine, this: Ptr<Moon>, flags: u32) -> Ptr<Moon> {
    moon_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00634ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Moon::~Moon` (Xbox PDB): drops the meshes and nodes, detaches the root
/// from its parent (the parent's slot `+0xE8`) when it has one, drops the
/// root, destroys the texture names and the pointers and runs the
/// `SkyObject` destructor.
// Translation incomplete: 00634ce0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn moon_destructor(e: &mut Engine, this: Ptr<Moon>) {
    let t = this.addr();
    e.mem.set_u32(t, MOON_VTABLE);
    for offset in [0x14u32, 0x10, 0x0c, 0x08] {
        e.call(NI_POINTER_ASSIGN, &args![t + offset, 0u32]);
    }
    let root = pointer(e, t + 4);
    if e.call(GET_PARENT, &args![root]).u32() != 0 {
        let root = pointer(e, t + 4);
        let parent = e.call(GET_PARENT, &args![root]).u32();
        let root = pointer(e, t + 4);
        e.vcall(parent, DETACH_CHILD_SLOT, &args![root]);
    }
    e.call(NI_POINTER_ASSIGN, &args![t + 4, 0u32]);
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &args![t + 0x18, 8u32, 8u32, BS_STRING_DESTRUCTOR],
    );
    for offset in [0x14u32, 0x10, 0x0c, 0x08] {
        e.call(NI_POINTER_DESTRUCT, &args![t + offset]);
    }
    e.call(SKY_OBJECT_DESTRUCTOR, &args![this]);
}

/// `new T[count]` of `size`-byte elements constructed by `constructor`
/// (null when the allocation fails).
fn new_array(e: &mut Engine, size: u32, count: u32, constructor: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![size * count]).u32();
    if block == 0 {
        return 0;
    }
    e.call(VECTOR_CONSTRUCTOR, &args![block, size, count, constructor]);
    block
}

/// A value built by `constructor` (`count` words) in a stack temporary and
/// copied out of the pointer the constructor answers.
fn build_value(e: &mut Engine, constructor: u32, words: &[u32]) -> Vec<u32> {
    e.with_stack(0x10, |e, value| {
        let mut call = vec![value.addr()];
        call.extend_from_slice(words);
        let at = e.call(constructor, &call).u32();
        (0..words.len() as u32)
            .map(|i| e.mem.u32(at + 4 * i))
            .collect()
    })
}

/// Writes the words to `address`.
fn put_words(e: &mut Engine, address: u32, words: &[u32]) {
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(address.wrapping_add(4 * i as u32), *word);
    }
}

/// The quad's corners `(-s, s), (-s, -s), (s, s), (s, -s)` (the moon's
/// size `s`) as `NiPoint3` temporaries copied into `vertices`.
fn put_quad(e: &mut Engine, this: Ptr<Moon>, vertices: u32) {
    let corners = [(-1i32, 1i32), (-1, -1), (1, 1), (1, -1)];
    for (i, (x_sign, y_sign)) in corners.iter().enumerate() {
        let coordinate = |e: &mut Engine, sign: i32| {
            let size = f64::from(e.get(this, Moon::iSize));
            if sign < 0 {
                let negated = x87_store32(x87_mul(size, e.global::<f64>(MINUS_ONE)));
                x87_quiet_bits(negated)
            } else {
                x87_store32(size)
            }
        };
        let y = coordinate(e, *y_sign);
        let x = coordinate(e, *x_sign);
        let point = build_value(e, POINT3_CONSTRUCTOR, &[x, y, 0.0f32.to_bits()]);
        put_words(e, vertices.wrapping_add(12 * i as u32), &point);
    }
}

/// The texture coordinates `(0,0) (0,1) (1,0) (1,1)` copied into `uvs`.
fn put_uvs(e: &mut Engine, uvs: u32) {
    let corners = [(0.0f32, 0.0f32), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)];
    for (i, (u, v)) in corners.iter().enumerate() {
        let point = build_value(e, POINT2_CONSTRUCTOR, &[u.to_bits(), v.to_bits()]);
        put_words(e, uvs.wrapping_add(8 * i as u32), &point);
    }
}

/// A new `NiTriShape` (0xC4 bytes, null when the allocation fails) of the
/// quad.
fn new_quad_shape(
    e: &mut Engine,
    vertices: u32,
    normals: u32,
    colors: u32,
    uvs: u32,
    triangles: u32,
) -> u32 {
    let block = e.call(NI_NEW, &args![0xc4u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        NI_TRI_SHAPE_CONSTRUCTOR,
        &args![block, 4u32, vertices, normals, colors, uvs, 1u32, 0u32, 2u32, triangles],
    )
    .u32()
}

/// A new `NiNode` (0xAC bytes, null when the allocation fails).
fn new_node(e: &mut Engine) -> u32 {
    let block = e.call(NI_NEW, &args![0xacu32]).u32();
    if block == 0 {
        0
    } else {
        e.call(NI_NODE_CONSTRUCTOR, &args![block, 0u32]).u32()
    }
}

/// The texture `load_texture` loads: a fixed name, or the string held
/// in a `BSStringT` slot (read when the name is needed).
enum TextureName {
    Fixed(u32),
    Slot(u32),
}

/// Gives the mesh at `mesh_slot` the texture `name` (through its
/// texturing property, type 3), sets its flag `0x20` when there is one,
/// and stores `status` as the texture update state.
fn load_texture(e: &mut Engine, this: Ptr<Moon>, mesh_slot: u32, name: TextureName, status: u32) {
    let mesh = pointer(e, this.addr() + mesh_slot);
    let property = e.call(GET_PROPERTY, &args![mesh, 3u32]).u32();
    e.with_stack(4, |e, texture| {
        e.call(NI_POINTER_CONSTRUCT, &args![texture, 0u32]);
        let name = match name {
            TextureName::Fixed(name) => name,
            TextureName::Slot(slot) => pointer(e, slot),
        };
        e.call(GET_TEXTURE, &args![name, 1u32, texture, 1u32, 0u32]);
        let loaded = pointer(e, texture.addr());
        if loaded != 0 {
            e.call(PROPERTY_SET_TEXTURE, &args![property, loaded]);
        }
        let mesh = pointer(e, this.addr() + mesh_slot);
        fn_00635fe0(e, Ptr::new(mesh), loaded != 0);
        e.set(this, Moon::eUpdateMoonTexture, status);
        e.call(NI_POINTER_DESTRUCT, &args![texture]);
    });
}

// Translated from 00634e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Moon::Initialize` (Xbox PDB): `SkyObject::Initialize` with `root`, then
/// (inside the allocation scope) builds the moon and shadow quads: vertex
/// corners `(+-size, +-size, 0)`, uninitialized normals, colours
/// `(1, 0, 0, 1)`, texture coordinates `(0,0) (0,1) (1,0) (1,1)` and
/// triangles `0 1 2 2 1 3`, each an `NiTriShape` given `004902f0(1)`; the
/// moon mesh gets a new alpha property (`0049ed90(1)`, `00439340(6)`,
/// `00439390(0)`, `00636000(1)`), both meshes are converted with shader
/// data 6 and 7; when the sky's mode is 3 or 2 the shadow takes
/// `"Textures\Sky\MoonShadow.dds"` and the texture update state becomes 2.
/// Then two new nodes hold the meshes and hang under the root, the meshes
/// are rotated by the angle at `01016b78` about the first axis and raised
/// to the height at `010231a0`, and the root is prepared for the shaders.
/// The second word is not read.
// Translation incomplete: 00634e20 - the compiler's exception frame (FS:[0] chain) and the /GS stack cookie check (00ec408c) are not translated
pub fn moon_initialize(e: &mut Engine, this: Ptr<Moon>, root: Ptr, _unused_2: u32) {
    let t = this.addr();
    e.with_stack(0x10, |e, scope| {
        e.call(
            SCOPE_OPEN,
            &args![scope, 0x21u32, 1u32, SOURCE_FILE, 0x66u32],
        );
        e.call(SKY_OBJECT_INITIALIZE, &args![this, root]);
        let normals = new_array(e, 0x0c, 4, POINT_ELEMENT_CONSTRUCTOR);
        let vertices = new_array(e, 0x0c, 4, POINT_ELEMENT_CONSTRUCTOR);
        let shadow_normals = new_array(e, 0x0c, 4, POINT_ELEMENT_CONSTRUCTOR);
        let shadow_vertices = new_array(e, 0x0c, 4, POINT_ELEMENT_CONSTRUCTOR);
        put_quad(e, this, vertices);
        put_quad(e, this, shadow_vertices);
        let uvs = new_array(e, 8, 4, POINT_ELEMENT_CONSTRUCTOR);
        let shadow_uvs = new_array(e, 8, 4, POINT_ELEMENT_CONSTRUCTOR);
        put_uvs(e, uvs);
        put_uvs(e, shadow_uvs);
        let colors = new_array(e, 0x10, 4, COLOR_ELEMENT_CONSTRUCTOR);
        let shadow_colors = new_array(e, 0x10, 4, COLOR_ELEMENT_CONSTRUCTOR);
        let one = 1.0f32.to_bits();
        let red = e.with_stack(0x10, |e, color| {
            e.call(COLOR_A_CONSTRUCTOR, &args![color, one, 0u32, 0u32, one]);
            (0..4)
                .map(|i| e.mem.u32(color.addr() + 4 * i))
                .collect::<Vec<_>>()
        });
        for array in [colors, shadow_colors] {
            for i in 0..4u32 {
                put_words(e, array.wrapping_add(0x10 * i), &red);
            }
        }
        let triangles = e.call(NI_ALLOC, &args![0x0cu32]).u32();
        let shadow_triangles = e.call(NI_ALLOC, &args![0x0cu32]).u32();
        for list in [triangles, shadow_triangles] {
            for (i, index) in [0u16, 1, 2, 2, 1, 3].iter().enumerate() {
                e.mem.set_u16(list.wrapping_add(2 * i as u32), *index);
            }
        }
        let shape = new_quad_shape(e, vertices, normals, colors, uvs, triangles);
        e.call(NI_POINTER_ASSIGN, &args![t + 0x10, shape]);
        let mesh = pointer(e, t + 0x10);
        e.call(NODE_SET_004902F0, &args![mesh, 1u32]);
        let shape = new_quad_shape(
            e,
            shadow_vertices,
            shadow_normals,
            shadow_colors,
            shadow_uvs,
            shadow_triangles,
        );
        e.call(NI_POINTER_ASSIGN, &args![t + 0x14, shape]);
        let mesh = pointer(e, t + 0x14);
        e.call(NODE_SET_004902F0, &args![mesh, 1u32]);

        // The moon mesh's alpha property, held in a local `NiPointer`.
        let block = e.call(NI_NEW, &args![0x1cu32]).u32();
        let alpha = if block == 0 {
            0
        } else {
            e.call(ALPHA_PROPERTY_CONSTRUCTOR, &args![block]).u32()
        };
        e.with_stack(4, |e, held| {
            e.call(NI_POINTER_CONSTRUCT, &args![held, alpha]);
            let property = pointer(e, held.addr());
            e.call(ALPHA_SET_0049ED90, &args![property, 1u32]);
            let property = pointer(e, held.addr());
            e.call(ALPHA_SET_00439340, &args![property, 6u32]);
            let property = pointer(e, held.addr());
            e.call(ALPHA_SET_00439390, &args![property, 0u32]);
            let property = pointer(e, held.addr());
            fn_00636000(e, Ptr::new(property), true);
            let property = pointer(e, held.addr());
            let mesh = pointer(e, t + 0x10);
            e.call(ATTACH_PROPERTY, &args![mesh, property]);
            e.call(NI_POINTER_ASSIGN, &args![held, 0u32]);
            e.call(NI_POINTER_DESTRUCT, &args![held]);
        });

        e.with_stack(0xb8, |e, data| {
            e.call(SHADER_DATA_CONSTRUCTOR, &args![data]);
            e.mem.set_u32(data.addr(), 6);
            e.mem.set_u32(data.addr() + 0x90, 6);
            let mesh = pointer(e, t + 0x10);
            e.call(SHADER_DATA_CONVERT, &args![data, mesh, 1u32]);
            e.mem.set_u32(data.addr() + 0x90, 7);
            let mesh = pointer(e, t + 0x14);
            e.call(SHADER_DATA_CONVERT, &args![data, mesh, 1u32]);
        });

        let sky = e.call(SKY_GET_INSTANCE, &args![]).u32();
        let mut shadowed = e.call(SKY_MODE, &args![sky]).u32() == 3;
        if !shadowed {
            let sky = e.call(SKY_GET_INSTANCE, &args![]).u32();
            shadowed = e.call(SKY_MODE, &args![sky]).u32() == 2;
        }
        if shadowed {
            load_texture(e, this, 0x14, TextureName::Fixed(MOON_SHADOW_TEXTURE), 2);
        }

        let node = new_node(e);
        e.call(NI_POINTER_ASSIGN, &args![t + 8, node]);
        let node = new_node(e);
        e.call(NI_POINTER_ASSIGN, &args![t + 0x0c, node]);
        for slot in [8u32, 0x0c] {
            let node = pointer(e, t + slot);
            e.call(NODE_SET_004902F0, &args![node, 1u32]);
        }
        for (parent, child) in [(8u32, 0x10u32), (0x0c, 0x14), (4, 0x0c), (4, 8)] {
            let parent = pointer(e, t + parent);
            let child = pointer(e, t + child);
            e.vcall(parent, ATTACH_CHILD_SLOT, &args![child, 1u32]);
        }
        e.with_stack(0x24, |e, rotation| {
            e.call(POINT_ELEMENT_CONSTRUCTOR, &args![rotation]);
            let angle = x87_quiet_bits(e.global::<u32>(INITIAL_ROTATION));
            e.call(MAKE_X_ROTATION, &args![rotation, angle]);
            for slot in [0x10u32, 0x14] {
                let height = x87_quiet_bits(e.global::<u32>(MESH_HEIGHT));
                let mesh = pointer(e, t + slot);
                e.call(SET_TRANSLATE, &args![mesh, 0.0f32, height, 0.0f32]);
                let mesh = pointer(e, t + slot);
                e.call(SET_ROTATE, &args![mesh, rotation]);
            }
        });
        let root = pointer(e, t + 4);
        e.call(PREPARE_OBJECT, &args![root, 0u32, 0u32]);
        e.call(SCOPE_CLOSE, &args![scope]);
    });
}

// Translated from 00635fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag `0x20` of the object `this` (its flags word +0x30,
/// through `0043b370`).
pub fn fn_00635fe0(e: &mut Engine, this: Ptr, set: bool) {
    e.call(SET_OBJECT_FLAG, &args![this, set, 0x20u32]);
}

// Translated from 00636000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit `0x2000` of the 16-bit flags at `this + 0x18`
/// (through `00633ba0`).
pub fn fn_00636000(e: &mut Engine, this: Ptr, set: bool) {
    e.call(SET_PROPERTY_FLAG, &args![this, set, 0x2000u32]);
}

/// Sets the hide flag of the node at `slot` (`00450f90`).
fn hide(e: &mut Engine, this: Ptr<Moon>, slot: u32, hidden: bool) {
    let node = pointer(e, this.addr() + slot);
    e.call(SET_HIDDEN, &args![node, hidden]);
}

/// The material property (type 3, of type `0xD`) of the mesh at `slot`, or
/// null.
fn material(e: &mut Engine, this: Ptr<Moon>, slot: u32) -> u32 {
    let mesh = pointer(e, this.addr() + slot);
    let has = if e.call(GET_PROPERTY, &args![mesh, 3u32]).u32() == 0 {
        false
    } else {
        let mesh = pointer(e, this.addr() + slot);
        let property = e.call(GET_PROPERTY, &args![mesh, 3u32]).u32();
        e.call(PROPERTY_TYPE, &args![property]).u32() == 0xd
    };
    if has {
        let mesh = pointer(e, this.addr() + slot);
        e.call(GET_PROPERTY, &args![mesh, 3u32]).u32()
    } else {
        0
    }
}

// Translated from 00636020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Moon::Update` (Xbox PDB), inside the allocation scope: `SkyObject`'s
/// update; starts the position at 90 degrees on the first update and when
/// the days passed changed (the last update then `-(days - 1) * 24`
/// hours); advances the position by `speed * 60` degrees per hour since the
/// last update (wrapped into `0 .. 360`) and keeps the hour; computes the
/// moon's and the shadow's fades ([`fn_006366d0`], [`fn_006367e0`], the
/// shadow's combined with the sky's value through `0040ebd0`); loads the
/// phase texture (the phase index at `011cca80`) when asked (state 2, or
/// state 1 with both fades `<= 0`). In sky modes 3 and 2, a shown root is
/// rotated by the position and the height offset; with both fades `<= 0`
/// it is hidden, else shown, and the moon and shadow nodes are shown with
/// their material alphas (the shadow's colour from the sky) when their
/// fade is positive and their mesh is shown, hidden otherwise. A root that
/// is not shown is hidden.
// Translation incomplete: 00636020 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn moon_update(e: &mut Engine, this: Ptr<Moon>, sky: Ptr, time: f32) {
    let t = this.addr();
    e.with_stack(0x24 + 0x24 + 0x24 + 0x10, |e, frame| {
        let scope = frame.addr();
        let rotation = scope + 0x10;
        let second = rotation + 0x24;
        let product = second + 0x24;
        e.call(
            SCOPE_OPEN,
            &args![scope, 0x21u32, 1u32, SOURCE_FILE, 0x124u32],
        );
        e.call(
            SKY_OBJECT_UPDATE,
            &args![this, sky, x87_quiet_bits(time.to_bits())],
        );
        if x87_load32(e.get(this, Moon::fLastPosUpdate)) == e.global::<f64>(NO_UPDATE_DOUBLE) {
            e.set(this, Moon::fLastPosUpdate, 0.0f32.to_bits());
            let position = x87_quiet_bits(e.global::<u32>(START_POSITION));
            e.set(this, Moon::fPos, position);
        }
        let owner = e.global::<u32>(DAYS_CHANGED_OWNER);
        if e.call(DAYS_CHANGED, &args![owner]).bool() {
            let days = e.call(DAYS_PASSED, &args![CALENDAR]).u32();
            // `FILD qword` of the 32-bit `days - 1` with a zero high word.
            let elapsed = f64::from(days.wrapping_sub(1));
            let hours = x87_mul(elapsed, e.global::<f64>(HOURS_PER_DAY));
            e.set(this, Moon::fLastPosUpdate, x87_store32(-hours));
            let position = x87_quiet_bits(e.global::<u32>(START_POSITION));
            e.set(this, Moon::fPos, position);
        }
        let hour = e.call(SKY_HOUR, &args![sky]).f64();
        let last = e.get(this, Moon::fLastPosUpdate);
        let mut passed = x87_store32(x87_sub(hour, x87_m32(last)));
        if x87_load32(passed) < e.global::<f64>(ZERO) {
            passed = x87_store32(x87_add(x87_load32(passed), e.global::<f64>(HOURS_PER_DAY)));
        }
        let speed = e.get(this, Moon::fSpeed);
        let step = x87_mul(
            x87_mul(x87_load32(speed), e.global::<f64>(SPEED_SCALE)),
            x87_m32(passed),
        );
        let position = e.get(this, Moon::fPos);
        e.set(
            this,
            Moon::fPos,
            x87_store32(x87_add(step, x87_m32(position))),
        );
        while x87_load32(e.get(this, Moon::fPos)) < e.global::<f64>(ZERO) {
            let position = x87_load32(e.get(this, Moon::fPos));
            let turned = x87_add(position, e.global::<f64>(FULL_TURN));
            e.set(this, Moon::fPos, x87_store32(turned));
        }
        // `TEST AH,1`: the loop ends on `pos < 360` and on NaN.
        while x87_load32(e.get(this, Moon::fPos)) >= e.global::<f64>(FULL_TURN) {
            let position = x87_load32(e.get(this, Moon::fPos));
            let turned = x87_sub(position, e.global::<f64>(FULL_TURN));
            e.set(this, Moon::fPos, x87_store32(turned));
        }
        let hour = e.call(SKY_HOUR, &args![sky]).f64();
        e.set(this, Moon::fLastPosUpdate, x87_store32(hour));
        let moon_fade = x87_store32(f64::from(fn_006366d0(e, this)));
        let shadow_fade = x87_store32(f64::from(fn_006367e0(e, this)));
        let object = e.call(SKY_OBJECT, &args![sky]).u32();
        let value = x87_store32(e.call(SKY_OBJECT_VALUE, &args![object]).f64());
        let shadow_fade = x87_store32(
            e.call(
                COMBINE_FADE,
                &args![x87_quiet_bits(shadow_fade), x87_quiet_bits(value)],
            )
            .f64(),
        );
        let zero = e.global::<f64>(ZERO);
        let faded = |fade: u32| x87_load32(fade) <= zero;
        let status = e.get(this, Moon::eUpdateMoonTexture);
        if status == 2 || (status == 1 && faded(moon_fade) && faded(shadow_fade)) {
            let phase = e.global::<u32>(PHASE_INDEX);
            let slot = t.wrapping_add(phase.wrapping_mul(8)).wrapping_add(0x18);
            load_texture(e, this, 0x10, TextureName::Slot(slot), 0);
        }
        let mut drawn = e.call(SKY_MODE, &args![sky]).u32() == 3;
        if !drawn {
            drawn = e.call(SKY_MODE, &args![sky]).u32() == 2;
        }
        if drawn {
            let root = pointer(e, t + 4);
            if !e.call(IS_SHOWN, &args![root]).bool() {
                hide(e, this, 4, true);
                e.call(SCOPE_CLOSE, &args![scope]);
                return;
            }
            e.call(POINT_ELEMENT_CONSTRUCTOR, &args![rotation]);
            e.call(POINT_ELEMENT_CONSTRUCTOR, &args![second]);
            let position = e.get(this, Moon::fPos);
            let angle = x87_store32(x87_mul(
                -x87_load32(position),
                e.global::<f64>(DEGREES_TO_RADIANS),
            ));
            e.call(MAKE_X_ROTATION, &args![rotation, x87_quiet_bits(angle)]);
            let offset = e.get(this, Moon::fZOffset);
            let angle = x87_store32(x87_mul(
                x87_load32(offset),
                e.global::<f64>(DEGREES_TO_RADIANS),
            ));
            e.call(MAKE_SECOND_ROTATION, &args![second, x87_quiet_bits(angle)]);
            let combined = e
                .call(MATRIX_PRODUCT, &args![rotation, product, second])
                .u32();
            let root = pointer(e, t + 4);
            e.call(SET_ROTATE, &args![root, combined]);
            if faded(moon_fade) && faded(shadow_fade) {
                hide(e, this, 4, true);
            } else {
                hide(e, this, 4, false);
                let mesh = pointer(e, t + 0x10);
                if faded(moon_fade) || !e.call(IS_SHOWN, &args![mesh]).bool() {
                    hide(e, this, 8, true);
                } else {
                    hide(e, this, 8, false);
                    let property = material(e, this, 0x10);
                    if property != 0 {
                        e.mem.set_u32(property + 0x6c, x87_quiet_bits(moon_fade));
                    }
                }
                let mesh = pointer(e, t + 0x14);
                if faded(shadow_fade) || !e.call(IS_SHOWN, &args![mesh]).bool() {
                    hide(e, this, 0x0c, true);
                } else {
                    hide(e, this, 0x0c, false);
                    let property = material(e, this, 0x14);
                    if property != 0 {
                        let mut color = [0u32; 3];
                        for (i, word) in [8u32, 4, 0].iter().enumerate() {
                            let sky_color = e.call(SKY_COLOR, &args![sky]).u32();
                            color[i] = x87_quiet_bits(e.mem.u32(sky_color + word));
                        }
                        let [blue, green, red] = color;
                        let alpha = x87_quiet_bits(shadow_fade);
                        let value = build_value(e, COLOR_A_CONSTRUCTOR, &[red, green, blue, alpha]);
                        put_words(e, property + 0x60, &value);
                    }
                }
            }
        }
        e.call(SCOPE_CLOSE, &args![scope]);
    });
}

/// The fade of a moon part between angles: 0 above 180 degrees; while
/// rising (`end <= pos <= start`) `(pos - end) / (start - end)`; while
/// setting (`180 - start <= pos <= 180 - end`) `(180 - end - pos) / ((180
/// - end) - (180 - start))`; 1 between `start` and `180 - start`; else 0.
/// The comparisons and the arithmetic are the code's, NaN included.
fn fade(position: u32, start: u32, end: u32, half_turn: f64) -> f32 {
    let rise_end = x87_store32(x87_sub(half_turn, x87_load32(start)));
    let set_end = x87_store32(x87_sub(half_turn, x87_load32(end)));
    let pos = x87_load32(position);
    if pos > half_turn {
        return 0.0;
    }
    let (s, n) = (x87_load32(start), x87_load32(end));
    if !(s < pos || s.is_nan() || pos.is_nan()) && n <= pos {
        let ratio = x87_div(x87_sub(pos, x87_m32(end)), x87_sub(s, x87_m32(end)));
        return f32::from_bits(x87_quiet_bits(x87_store32(ratio)));
    }
    let (a, b) = (x87_load32(rise_end), x87_load32(set_end));
    if a <= pos && !(b < pos || b.is_nan() || pos.is_nan()) {
        let ratio = x87_div(x87_sub(b, x87_m32(position)), x87_sub(b, x87_m32(rise_end)));
        return f32::from_bits(x87_quiet_bits(x87_store32(ratio)));
    }
    if s < pos && a > pos {
        return 1.0;
    }
    0.0
}

// Translated from 006366d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The moon's fade at its position (no PDB name): between
/// `fAngleFadeStart` and `fAngleFadeEnd` (see [`fade`]).
pub fn fn_006366d0(e: &mut Engine, this: Ptr<Moon>) -> f32 {
    let half_turn = e.global::<f64>(HALF_TURN);
    let position = e.get(this, Moon::fPos);
    let start = e.get(this, Moon::fAngleFadeStart);
    let end = e.get(this, Moon::fAngleFadeEnd);
    fade(position, start, end, half_turn)
}

// Translated from 006367e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The shadow's fade at the moon's position (no PDB name): between
/// `fAngleFadeEnd` and `fAngleFadeEnd - fAngleShadowEarlyFade` (see
/// [`fade`]).
pub fn fn_006367e0(e: &mut Engine, this: Ptr<Moon>) -> f32 {
    let half_turn = e.global::<f64>(HALF_TURN);
    let position = e.get(this, Moon::fPos);
    let start = x87_quiet_bits(e.get(this, Moon::fAngleFadeEnd));
    let early = e.get(this, Moon::fAngleShadowEarlyFade);
    let end = x87_store32(x87_sub(
        x87_load32(e.get(this, Moon::fAngleFadeEnd)),
        x87_m32(early),
    ));
    fade(position, start, end, half_turn)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0063_4a70,
            moon_constructor(Ptr<Moon>, Ptr, f32, f32, f32, f32, f32, u32) -> Ptr<Moon>
        ),
        entry!(
            0x0063_4cb0,
            moon_scalar_deleting_destructor(Ptr<Moon>, u32) -> Ptr<Moon>
        ),
        entry!(0x0063_4ce0, moon_destructor(Ptr<Moon>)),
        entry!(0x0063_4e20, moon_initialize(Ptr<Moon>, Ptr, u32)),
        entry!(0x0063_5fe0, fn_00635fe0(Ptr, bool)),
        entry!(0x0063_6000, fn_00636000(Ptr, bool)),
        entry!(0x0063_6020, moon_update(Ptr<Moon>, Ptr, f32)),
        entry!(0x0063_66d0, fn_006366d0(Ptr<Moon>) -> f32),
        entry!(0x0063_67e0, fn_006367e0(Ptr<Moon>) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ATTACH: u32 = 0x0000_10dc;
    const DETACH: u32 = 0x0000_10e8;
    const OBJECT_VTABLE: u32 = 0x0003_0000;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    /// Doubles every test needs: pointer reads and assignments that work on
    /// memory, constructors that answer their block, a node vtable.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NI_POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        for address in [NI_POINTER_ASSIGN, NI_POINTER_CONSTRUCT] {
            e.register(address, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                Ret::default()
            });
        }
        for address in [NI_NEW, OPERATOR_NEW, NI_ALLOC] {
            e.register(address, |e, a| Ret {
                eax: e.mem.alloc(a[0]),
                ..Ret::default()
            });
        }
        for address in [
            NI_TRI_SHAPE_CONSTRUCTOR,
            NI_NODE_CONSTRUCTOR,
            ALPHA_PROPERTY_CONSTRUCTOR,
        ] {
            e.register(address, |e, a| {
                e.mem.set_u32(a[0], OBJECT_VTABLE);
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        for address in [POINT3_CONSTRUCTOR, POINT2_CONSTRUCTOR, COLOR_A_CONSTRUCTOR] {
            e.register(address, |e, a| {
                for (i, word) in a[1..].iter().enumerate() {
                    e.mem.set_u32(a[0] + 4 * i as u32, *word);
                }
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        let mut slots = vec![0u32; 0x40];
        slots[(ATTACH_CHILD_SLOT / 4) as usize] = ATTACH;
        slots[(DETACH_CHILD_SLOT / 4) as usize] = DETACH;
        e.put_vtable(OBJECT_VTABLE, &slots);
        quiet(
            &mut e,
            &[ATTACH, DETACH, SCOPE_OPEN, SCOPE_CLOSE, NI_POINTER_DESTRUCT],
        );
        e
    }

    fn moon(e: &mut Engine) -> Ptr<Moon> {
        let this = e.new_object::<Moon>();
        e.set(this, Moon::iSize, 64);
        this
    }

    #[test]
    fn moon_constructor_names_the_textures() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                SKY_OBJECT_CONSTRUCTOR,
                VECTOR_CONSTRUCTOR_ITERATOR,
                BS_STRING_FORMAT,
            ],
        );
        e.map(NO_UPDATE, 4);
        e.set_global(NO_UPDATE, f32::MAX);
        let this = e.new_object::<Moon>();
        e.set(this, Moon::eUpdateMoonTexture, 2);
        e.call_log = Some(vec![]);
        let result = e.call(
            0x0063_4a70,
            &args![this, 0x7000u32, 10.0f32, 20.0f32, 5.0f32, 2.0f32, 15.0f32, 128u32],
        );
        assert_eq!(result.u32(), this.addr());
        let t = this.addr();
        assert_eq!(e.mem.u32(t), MOON_VTABLE);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![
                t + 0x18,
                8,
                8,
                BS_STRING_CONSTRUCTOR,
                BS_STRING_DESTRUCTOR
            ]]
        );
        let names = calls_to(&e, BS_STRING_FORMAT);
        assert_eq!(names.len(), 7);
        assert_eq!(
            names[0],
            vec![t + 0x30, TEXTURE_FORMAT, 0x7000, 0x0104_ecc4]
        );
        assert_eq!(
            names[6],
            vec![t + 0x18, TEXTURE_FORMAT, 0x7000, 0x0104_ec84]
        );
        assert_eq!(e.get(this, Moon::fAngleFadeStart), bits(10.0));
        assert_eq!(e.get(this, Moon::fAngleShadowEarlyFade), bits(5.0));
        assert_eq!(e.get(this, Moon::fZOffset), bits(15.0));
        assert_eq!(e.get(this, Moon::iSize), 128);
        assert_eq!(e.get(this, Moon::fLastPosUpdate), bits(f32::MAX));
        assert_eq!(e.get(this, Moon::eUpdateMoonTexture), 0);
        assert_eq!(
            calls_to(&e, SCOPE_OPEN)[0][1..],
            [0x21, 1, SOURCE_FILE, 0x32]
        );
    }

    #[test]
    fn moon_destructors_detach_and_release() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                VECTOR_DESTRUCTOR_ITERATOR,
                SKY_OBJECT_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        e.register(GET_PARENT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x18),
            ..Ret::default()
        });
        let this = moon(&mut e);
        let root = e.mem.alloc(0x20);
        let parent = e.mem.alloc(0x20);
        e.mem.set_u32(parent, OBJECT_VTABLE);
        e.mem.set_u32(root + 0x18, parent);
        e.set(this, Moon::spRoot, root);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_4cb0, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, DETACH), vec![vec![parent, root]]);
        assert_eq!(e.get(this, Moon::spRoot), 0);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCTOR_ITERATOR),
            vec![vec![this.addr() + 0x18, 8, 8, BS_STRING_DESTRUCTOR]]
        );
        assert_eq!(calls_to(&e, NI_POINTER_DESTRUCT).len(), 4);
        assert_eq!(calls_to(&e, SKY_OBJECT_DESTRUCTOR), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this.addr()]]);
        // A root without a parent: no detach; no delete without bit 0.
        let root = e.mem.alloc(0x20);
        e.set(this, Moon::spRoot, root);
        e.call_log = Some(vec![]);
        e.call(0x0063_4cb0, &args![this, 0u32]);
        assert!(calls_to(&e, DETACH).is_empty());
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn fn_00635fe0_and_fn_00636000_set_their_flags() {
        let mut e = Engine::new();
        quiet(&mut e, &[SET_OBJECT_FLAG, SET_PROPERTY_FLAG]);
        e.call_log = Some(vec![]);
        e.call(0x0063_5fe0, &args![0x5000u32, 1u32]);
        e.call(0x0063_6000, &args![0x6000u32, 0u32]);
        assert_eq!(calls_to(&e, SET_OBJECT_FLAG), vec![vec![0x5000, 1, 0x20]]);
        assert_eq!(
            calls_to(&e, SET_PROPERTY_FLAG),
            vec![vec![0x6000, 0, 0x2000]]
        );
    }

    fn initialize_engine(mode: u32) -> Engine {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                SKY_OBJECT_INITIALIZE,
                VECTOR_CONSTRUCTOR,
                NODE_SET_004902F0,
                ALPHA_SET_0049ED90,
                ALPHA_SET_00439340,
                ALPHA_SET_00439390,
                SET_PROPERTY_FLAG,
                SET_OBJECT_FLAG,
                ATTACH_PROPERTY,
                SHADER_DATA_CONSTRUCTOR,
                PROPERTY_SET_TEXTURE,
                POINT_ELEMENT_CONSTRUCTOR,
                MAKE_X_ROTATION,
                SET_TRANSLATE,
                SET_ROTATE,
                PREPARE_OBJECT,
            ],
        );
        e.register(SKY_GET_INSTANCE, |_, _| Ret {
            eax: 0x0000_7700,
            ..Ret::default()
        });
        e.register_double(SKY_MODE, move |_, _| Ret {
            eax: mode,
            ..Ret::default()
        });
        e.register(GET_PROPERTY, |_, a| Ret {
            eax: a[0] + 0x80,
            ..Ret::default()
        });
        e.register(GET_TEXTURE, |e, a| {
            e.mem.set_u32(a[2], 0x0000_7e00);
            Ret::default()
        });
        e.map(MINUS_ONE, 8);
        e.set_global(MINUS_ONE, -1.0f64);
        e.map(INITIAL_ROTATION, 4);
        e.set_global(INITIAL_ROTATION, -std::f32::consts::FRAC_PI_2);
        e.map(MESH_HEIGHT, 4);
        e.set_global(MESH_HEIGHT, 512.0f32);
        e
    }

    #[test]
    fn moon_initialize_builds_the_quads_and_nodes() {
        let mut e = initialize_engine(3);
        let datas = Rc::new(RefCell::new(vec![]));
        let seen = datas.clone();
        e.register_double(SHADER_DATA_CONVERT, move |e, a| {
            seen.borrow_mut()
                .push((e.mem.u32(a[0]), e.mem.u32(a[0] + 0x90), a[1], a[2]));
            Ret::default()
        });
        let this = moon(&mut e);
        let root = e.mem.alloc(0xd0);
        e.mem.set_u32(root, OBJECT_VTABLE);
        e.set(this, Moon::spRoot, root);
        e.call_log = Some(vec![]);
        e.call(0x0063_4e20, &args![this, 0x4444u32, 9u32]);
        assert_eq!(
            calls_to(&e, SKY_OBJECT_INITIALIZE),
            vec![vec![this.addr(), 0x4444]]
        );
        let shapes = calls_to(&e, NI_TRI_SHAPE_CONSTRUCTOR);
        assert_eq!(shapes.len(), 2);
        let s = &shapes[0];
        assert_eq!(s[1], 4);
        assert_eq!(s[6..9], [1, 0, 2]);
        let floats =
            |e: &Engine, at: u32, n: u32| (0..n).map(|i| e.mem.f32(at + 4 * i)).collect::<Vec<_>>();
        assert_eq!(
            floats(&e, s[2], 12),
            vec![-64.0, 64.0, 0.0, -64.0, -64.0, 0.0, 64.0, 64.0, 0.0, 64.0, -64.0, 0.0]
        );
        assert_eq!(
            floats(&e, s[5], 8),
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0]
        );
        assert_eq!(
            floats(&e, s[4], 8),
            vec![1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0]
        );
        let indices: Vec<u16> = (0..6).map(|i| e.mem.u16(s[9] + 2 * i)).collect();
        assert_eq!(indices, vec![0, 1, 2, 2, 1, 3]);
        let mesh = e.get(this, Moon::spMoonMesh);
        let shadow = e.get(this, Moon::spShadowMesh);
        assert_eq!(mesh, s[0]);
        assert_eq!(shadow, shapes[1][0]);
        let alpha = calls_to(&e, ATTACH_PROPERTY)[0].clone();
        assert_eq!(alpha[0], mesh);
        assert_eq!(calls_to(&e, ALPHA_SET_00439340), vec![vec![alpha[1], 6]]);
        assert_eq!(
            calls_to(&e, SET_PROPERTY_FLAG),
            vec![vec![alpha[1], 1, 0x2000]]
        );
        assert_eq!(*datas.borrow(), vec![(6, 6, mesh, 1), (6, 7, shadow, 1)]);
        // Mode 3: the shadow texture.
        assert_eq!(calls_to(&e, GET_TEXTURE)[0][..2], [MOON_SHADOW_TEXTURE, 1]);
        assert_eq!(
            calls_to(&e, PROPERTY_SET_TEXTURE),
            vec![vec![shadow + 0x80, 0x7e00]]
        );
        assert_eq!(calls_to(&e, SET_OBJECT_FLAG), vec![vec![shadow, 1, 0x20]]);
        assert_eq!(e.get(this, Moon::eUpdateMoonTexture), 2);
        let (node, shadow_node) = (
            e.get(this, Moon::spMoonNode),
            e.get(this, Moon::spShadowNode),
        );
        assert_eq!(
            calls_to(&e, ATTACH),
            vec![
                vec![node, mesh, 1],
                vec![shadow_node, shadow, 1],
                vec![root, shadow_node, 1],
                vec![root, node, 1]
            ]
        );
        assert_eq!(
            calls_to(&e, SET_TRANSLATE),
            vec![
                vec![mesh, 0, bits(512.0), 0],
                vec![shadow, 0, bits(512.0), 0]
            ]
        );
        assert_eq!(calls_to(&e, PREPARE_OBJECT), vec![vec![root, 0, 0]]);
    }

    #[test]
    fn moon_initialize_skips_the_shadow_in_other_modes() {
        let mut e = initialize_engine(1);
        quiet(&mut e, &[SHADER_DATA_CONVERT]);
        let this = moon(&mut e);
        let root = e.mem.alloc(0xd0);
        e.mem.set_u32(root, OBJECT_VTABLE);
        e.set(this, Moon::spRoot, root);
        e.call_log = Some(vec![]);
        e.call(0x0063_4e20, &args![this, 0u32, 0u32]);
        assert_eq!(calls_to(&e, SKY_MODE).len(), 2);
        assert!(calls_to(&e, GET_TEXTURE).is_empty());
        assert_eq!(e.get(this, Moon::eUpdateMoonTexture), 0);
    }

    fn fade_moon(e: &mut Engine, start: f32, end: f32, early: f32, position: f32) -> Ptr<Moon> {
        let this = e.new_object::<Moon>();
        e.set(this, Moon::fAngleFadeStart, bits(start));
        e.set(this, Moon::fAngleFadeEnd, bits(end));
        e.set(this, Moon::fAngleShadowEarlyFade, bits(early));
        e.set(this, Moon::fPos, bits(position));
        this
    }

    #[test]
    fn fn_006366d0_fades_between_the_angles() {
        let mut e = Engine::new();
        e.map(HALF_TURN, 8);
        e.set_global(HALF_TURN, 180.0f64);
        let fade_at = |e: &mut Engine, position: f32| {
            let this = fade_moon(e, 20.0, 10.0, 0.0, position);
            e.call(0x0063_66d0, &args![this]).f64() as f32
        };
        assert_eq!(fade_at(&mut e, 190.0), 0.0);
        assert_eq!(fade_at(&mut e, 5.0), 0.0);
        assert_eq!(fade_at(&mut e, 15.0), 0.5);
        assert_eq!(fade_at(&mut e, 90.0), 1.0);
        assert_eq!(fade_at(&mut e, 165.0), 0.5);
        assert_eq!(fade_at(&mut e, 175.0), 0.0);
        assert_eq!(fade_at(&mut e, 180.0), 0.0);
        assert!(fade_at(&mut e, f32::NAN) == 0.0);
    }

    #[test]
    fn fn_006367e0_fades_the_shadow_earlier() {
        let mut e = Engine::new();
        e.map(HALF_TURN, 8);
        e.set_global(HALF_TURN, 180.0f64);
        let fade_at = |e: &mut Engine, position: f32| {
            let this = fade_moon(e, 30.0, 20.0, 8.0, position);
            e.call(0x0063_67e0, &args![this]).f64() as f32
        };
        // Between 12 (20 - 8) and 20.
        assert_eq!(fade_at(&mut e, 16.0), 0.5);
        assert_eq!(fade_at(&mut e, 10.0), 0.0);
        assert_eq!(fade_at(&mut e, 90.0), 1.0);
        assert_eq!(fade_at(&mut e, 164.0), 0.5);
    }

    struct Update {
        e: Engine,
        this: Ptr<Moon>,
        root: u32,
        sky: u32,
    }

    fn update_engine(mode: u32, hour: f64, shown: bool) -> Update {
        let mut e = engine();
        for (address, value) in [
            (NO_UPDATE_DOUBLE, f64::from(f32::MAX)),
            (HOURS_PER_DAY, 24.0),
            (SPEED_SCALE, 60.0),
            (FULL_TURN, 360.0),
            (DEGREES_TO_RADIANS, std::f64::consts::PI / 180.0),
            (ZERO, 0.0),
            (HALF_TURN, 180.0),
        ] {
            e.map(address, 8);
            e.set_global(address, value);
        }
        e.map(START_POSITION, 4);
        e.set_global(START_POSITION, 90.0f32);
        e.map(DAYS_CHANGED_OWNER, 4);
        e.map(PHASE_INDEX, 4);
        e.set_global(PHASE_INDEX, 2u32);
        e.register(DAYS_CHANGED, |_, _| Ret::default());
        e.register_double(SKY_HOUR, move |_, _| Ret {
            st0: hour,
            ..Ret::default()
        });
        e.register(SKY_OBJECT, |_, _| Ret::default());
        e.register(SKY_OBJECT_VALUE, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        // The combination answers the shadow's fade unchanged.
        e.register(COMBINE_FADE, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        e.register_double(SKY_MODE, move |_, _| Ret {
            eax: mode,
            ..Ret::default()
        });
        e.register_double(IS_SHOWN, move |_, _| Ret {
            eax: shown as u32,
            ..Ret::default()
        });
        quiet(
            &mut e,
            &[
                SKY_OBJECT_UPDATE,
                SET_HIDDEN,
                POINT_ELEMENT_CONSTRUCTOR,
                MAKE_X_ROTATION,
                MAKE_SECOND_ROTATION,
                SET_ROTATE,
                GET_TEXTURE,
                SET_OBJECT_FLAG,
            ],
        );
        e.register(MATRIX_PRODUCT, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(GET_PROPERTY, |_, _| Ret::default());
        let this = e.new_object::<Moon>();
        let root = e.mem.alloc(0x40);
        e.set(this, Moon::spRoot, root);
        for (slot, value) in [(8u32, 0x81u32), (0x0c, 0x82), (0x10, 0x83), (0x14, 0x84)] {
            e.mem.set_u32(this.addr() + slot, value);
        }
        e.set(this, Moon::fAngleFadeStart, bits(20.0));
        e.set(this, Moon::fAngleFadeEnd, bits(10.0));
        e.set(this, Moon::fAngleShadowEarlyFade, bits(5.0));
        e.set(this, Moon::fSpeed, bits(0.5));
        e.set(this, Moon::fZOffset, bits(15.0));
        let sky = 0x0000_9900;
        Update { e, this, root, sky }
    }

    fn hidden(e: &Engine) -> Vec<Vec<u32>> {
        calls_to(e, SET_HIDDEN)
    }

    #[test]
    fn moon_update_starts_and_advances_the_position() {
        let Update {
            mut e,
            this,
            root,
            sky,
        } = update_engine(3, 6.0, true);
        e.set(this, Moon::fLastPosUpdate, bits(f32::MAX));
        e.call_log = Some(vec![]);
        e.call(0x0063_6020, &args![this, sky, 0.25f32]);
        assert_eq!(
            calls_to(&e, SKY_OBJECT_UPDATE),
            vec![vec![this.addr(), sky, bits(0.25)]]
        );
        // From 90 degrees: 6 hours at 0.5 * 60 per hour.
        assert_eq!(e.get(this, Moon::fPos), bits(270.0));
        assert_eq!(e.get(this, Moon::fLastPosUpdate), bits(6.0));
        let rotation = calls_to(&e, MAKE_X_ROTATION)[0][1];
        assert_eq!(
            f32::from_bits(rotation),
            (-270.0f64 * (std::f64::consts::PI / 180.0)) as f32
        );
        assert_eq!(calls_to(&e, SET_ROTATE)[0][0], root);
        // At 270 degrees both fades are 0: the root is hidden.
        assert_eq!(hidden(&e), vec![vec![root, 1]]);
    }

    #[test]
    fn moon_update_wraps_and_shows_the_visible_parts() {
        let Update {
            mut e,
            this,
            root,
            sky,
        } = update_engine(2, 13.0, true);
        e.set(this, Moon::fLastPosUpdate, bits(1.0));
        e.set(this, Moon::fPos, bits(-280.0));
        let properties = e.mem.alloc(0x80);
        e.register_double(GET_PROPERTY, move |_, a| Ret {
            eax: if a[0] == 0x83 || a[0] == 0x84 {
                properties
            } else {
                0
            },
            ..Ret::default()
        });
        e.register(PROPERTY_TYPE, |_, _| Ret {
            eax: 0xd,
            ..Ret::default()
        });
        let color = e.mem.alloc(0x10);
        for (i, v) in [0.1f32, 0.2, 0.3].iter().enumerate() {
            e.mem.set_f32(color + 4 * i as u32, *v);
        }
        e.register_double(SKY_COLOR, move |_, _| Ret {
            eax: color,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0063_6020, &args![this, sky, 0.0f32]);
        // -280 + 12 hours * 30 = 80 degrees: fully shown.
        assert_eq!(e.get(this, Moon::fPos), bits(80.0));
        assert_eq!(
            hidden(&e),
            vec![vec![root, 0], vec![0x81, 0], vec![0x82, 0]]
        );
        assert_eq!(e.mem.f32(properties + 0x6c), 1.0);
        let colour: Vec<f32> = (0..4)
            .map(|i| e.mem.f32(properties + 0x60 + 4 * i))
            .collect();
        assert_eq!(colour, vec![0.1, 0.2, 0.3, 1.0]);
    }

    #[test]
    fn moon_update_loads_the_phase_texture_and_hides_an_unshown_root() {
        let Update {
            mut e,
            this,
            root,
            sky,
        } = update_engine(3, 0.0, false);
        e.set(this, Moon::fLastPosUpdate, bits(0.0));
        e.set(this, Moon::eUpdateMoonTexture, 2);
        e.mem.set_u32(this.addr() + 0x18 + 2 * 8, 0x0000_7100);
        e.register(GET_PROPERTY, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        e.register(GET_TEXTURE, |e, a| {
            e.mem.set_u32(a[2], 0x7e00);
            Ret::default()
        });
        quiet(&mut e, &[PROPERTY_SET_TEXTURE]);
        e.call_log = Some(vec![]);
        e.call(0x0063_6020, &args![this, sky, 0.0f32]);
        assert_eq!(calls_to(&e, GET_TEXTURE)[0][..2], [0x7100, 1]);
        assert_eq!(
            calls_to(&e, PROPERTY_SET_TEXTURE),
            vec![vec![0x83 + 0x100, 0x7e00]]
        );
        assert_eq!(e.get(this, Moon::eUpdateMoonTexture), 0);
        assert_eq!(hidden(&e), vec![vec![root, 1]]);
        assert!(calls_to(&e, MAKE_X_ROTATION).is_empty());
        // Another sky mode: nothing is shown or hidden.
        let Update {
            mut e, this, sky, ..
        } = update_engine(1, 0.0, true);
        e.set(this, Moon::fLastPosUpdate, bits(0.0));
        e.call_log = Some(vec![]);
        e.call(0x0063_6020, &args![this, sky, 0.0f32]);
        assert!(hidden(&e).is_empty());
        assert_eq!(calls_to(&e, SCOPE_CLOSE).len(), 1);
    }
}
