//! `fallout shared/speedtree/bstreemodel.cpp` (Xbox PDB source unit), subsystem `fallout shared/speedtree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `BSTreeModel` wraps a SpeedTree (`CSpeedTreeRT`) for a `TESObjectTREE`:
//! the branch geometry per level of detail (`NiTriStripsData`), the leaf
//! geometry, their shader properties and textures, and the tree's
//! collision body. The PC code uses the Xbox PDB's offsets of the class.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006667d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeModel` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00666800` and frees 0x50 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_tree_model_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0066_6800, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x50u32]);
    }
    this
}

// Translated from 006694e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00633c90` on `this` with a null argument and returns its answer unchanged (the code
/// does not touch `EAX` afterwards; `00633c90` stores the argument in the pointer slot at
/// `this` and, when it is not null, runs `0040f6e0` on it, and returns `this`). The engine map
/// gives it no name, so the class is unconfirmed.
pub fn fn_006694e0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0063_3c90, &args![this, 0u32]).u32()
}

layout! {
    /// `BSTreeModel` (Xbox PDB), 0x50 bytes: an `NiRefObject` (vtable,
    /// reference count) and the tree's data. Float fields are their bit
    /// patterns.
    pub struct BSTreeModel: 0x50 {
        /// `eType` (Xbox PDB): `BSTreeModel::BSTREEMODEL_TYPE`.
        0x08 eType: u32,
        /// `pSpeedTree` (Xbox PDB): `CSpeedTreeRT*`.
        0x0C pSpeedTree: u32,
        /// `spTreeParent` (Xbox PDB): `NiPointer<BSTreeModel>`.
        0x10 spTreeParent: u32,
        /// `pspBranchData` (Xbox PDB): an array of `NiPointer<NiTriStripsData>`
        /// (its count in the word before it).
        0x14 pspBranchData: u32,
        /// `pspLeafData` (Xbox PDB): an array of `NiPointer<NiTriShapeData>`.
        0x18 pspLeafData: u32,
        /// `spBillboard` (Xbox PDB): `NiPointer<NiTriShape>`.
        0x1C spBillboard: u32,
        /// `spLeafShaderData` (Xbox PDB): `NiPointer<STLSPData>`.
        0x20 spLeafShaderData: u32,
        /// `pspBranchShaderProperties` (Xbox PDB): an array of pointers.
        0x24 pspBranchShaderProperties: u32,
        /// `pspLeafShaderProperties` (Xbox PDB): an array of pointers.
        0x28 pspLeafShaderProperties: u32,
        /// `spBranchTextureSet` (Xbox PDB): `NiPointer<BSShaderTextureSet>`.
        0x2C spBranchTextureSet: u32,
        /// `spLeafFrondTexture` (Xbox PDB): `NiPointer<NiSourceTexture>`.
        0x30 spLeafFrondTexture: u32,
        /// `spBillboardTextureSet` (Xbox PDB).
        0x34 spBillboardTextureSet: u32,
        /// `spRigidBody` (Xbox PDB): `NiPointer<bhkRigidBody>`.
        0x38 spRigidBody: u32,
        /// `fCurveScalar` (Xbox PDB).
        0x3C fCurveScalar: u32,
        /// `iSeed` (Xbox PDB).
        0x40 iSeed: u32,
        /// `bInitialized` (Xbox PDB).
        0x44 bInitialized: u8,
        /// `fTrunkHeight` (Xbox PDB).
        0x48 fTrunkHeight: u32,
        /// `fTrunkWidth` (Xbox PDB).
        0x4C fTrunkWidth: u32,
    }
}

/// `BSTreeModel`'s vtable (`01066768`, RTTI `.?AVBSTreeModel@@`).
const BS_TREE_MODEL_VTABLE: u32 = 0x0106_6768;
/// The `NiRefObject` destructor body (`00496910`).
const REF_OBJECT_DESTRUCTOR: u32 = 0x0049_6910;
/// The `CSpeedTreeRT` destructor (`00b03b30`).
const SPEED_TREE_DESTRUCTOR: u32 = 0x00b0_3b30;
/// `operator new` (`00401000`) and `operator delete` (`00401030`) (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The `NiPointer` constructor (`00633c90`), assignment (`0066b0d0`),
/// destructor (`0045cec0`) and read (`00559450`).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `` `eh vector constructor iterator' `` (`00ec782f`) and destructor
/// iterator (`00ec5fce`) (stdcall).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// A float setting's value (`00403e20`, thiscall on the setting: a pointer
/// to the value).
const SETTING_FLOAT_VALUE: u32 = 0x0040_3e20;
/// The settings the tree reads: the curve scalar (`011d5d14`), two wind
/// values (`011d5db8`, `011d5d24`) and the two angles (`011d5c9c`,
/// `011d5c80`).
const CURVE_SCALAR_SETTING: u32 = 0x011d_5d14;
const SETTING_011D5DB8: u32 = 0x011d_5db8;
const SETTING_011D5D24: u32 = 0x011d_5d24;
const SETTING_011D5C9C: u32 = 0x011d_5c9c;
const SETTING_011D5C80: u32 = 0x011d_5c80;
/// Gamebryo's object allocation (`00aa13e0`, cdecl).
const NI_NEW: u32 = 0x00aa_13e0;
/// The alpha property's constructor (`004391c0`, 0x1C bytes) and setters
/// (`0049ed90`, `004393c0`, `004393e0`, `0094db80`; one word each).
const ALPHA_PROPERTY_CONSTRUCTOR: u32 = 0x0043_91c0;
const ALPHA_SET_0049ED90: u32 = 0x0049_ed90;
const ALPHA_SET_004393C0: u32 = 0x0043_93c0;
const ALPHA_SET_004393E0: u32 = 0x0043_93e0;
const ALPHA_SET_0094DB80: u32 = 0x0094_db80;
/// `CSpeedTreeRT::GetNumBranchLodLevels` (`00b086f0`, Xbox PDB name) and
/// the leaf count (`00b08710`).
const SPEED_TREE_BRANCH_LODS: u32 = 0x00b0_86f0;
const SPEED_TREE_LEAF_LODS: u32 = 0x00b0_8710;
/// The tree manager's instance (`00664840`, cdecl: create flag).
const TREE_MANAGER_INSTANCE: u32 = 0x0066_4840;
/// The `CSpeedTreeRT` setters `ApplyBaseObject` uses (`00b07390`,
/// `00b073c0`, `00b07490`, `00b074c0`; one float each).
const SPEED_TREE_SET_00B07390: u32 = 0x00b0_7390;
const SPEED_TREE_SET_00B073C0: u32 = 0x00b0_73c0;
const SPEED_TREE_SET_00B07490: u32 = 0x00b0_7490;
const SPEED_TREE_SET_00B074C0: u32 = 0x00b0_74c0;
/// The allocation scope (`00404eb0`: 0x1E, 1, file, line; `00404ee0`).
const SCOPE_OPEN: u32 = 0x0040_4eb0;
const SCOPE_CLOSE: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\SpeedTree\BSTreeModel.cpp"`
/// (`010667b8`).
const SOURCE_FILE: u32 = 0x0106_67b8;
/// The texture-name holder (`00b02e20` clears its seven words,
/// `00b02e80` destroys it) and the tree's branch texture name into it
/// (`00b0d040`, thiscall on the SpeedTree).
const NAME_HOLDER_CONSTRUCTOR: u32 = 0x00b0_2e20;
const NAME_HOLDER_DESTRUCTOR: u32 = 0x00b0_2e80;
const SPEED_TREE_TEXTURE_NAMES: u32 = 0x00b0_d040;
/// `strcpy_s` (`00406d30`), `strcat_s` (`00406d50`) and `strlen`
/// (`0044a670`) wrappers (cdecl).
const STRCPY: u32 = 0x0040_6d30;
const STRCAT: u32 = 0x0040_6d50;
const STRLEN: u32 = 0x0044_a670;
/// `"Textures\Trees\Branches\"` (`0106679c`), `"Textures\Trees\Leaves\"`
/// (`01066784`) and `"_n"` (`0101f6b8`).
const BRANCHES_FOLDER: u32 = 0x0106_679c;
const LEAVES_FOLDER: u32 = 0x0106_6784;
const NORMAL_MAP_SUFFIX: u32 = 0x0101_f6b8;
/// `BSShaderManager::GetModifiedTextureFilename` (`00b4f5e0`, Xbox PDB
/// name, cdecl: out, name, suffix, 1).
const GET_MODIFIED_TEXTURE_FILENAME: u32 = 0x00b4_f5e0;
/// `BSShaderTextureSet::BSShaderTextureSet` (`0051d0d0`, Xbox PDB name;
/// 0x38 bytes) and its texture setter (virtual slot `+0x94`: index, name).
const TEXTURE_SET_CONSTRUCTOR: u32 = 0x0051_d0d0;
const TEXTURE_SET_SET_SLOT: u32 = 0x94;
/// The tree's leaf texture test and name (`0048cee0`, `00408da0`; the
/// engine map names the latter `MapMarkerData::GetLocationName`) on the
/// form's +0x48, `TES::CreateTextureImage` (`004568c0`, thiscall on the
/// word `011dea10`) and `TESObjectTREE::GetBillboardTextureFilename`
/// (`0051c3b0`) (Xbox PDB names).
const LEAF_TEXTURE_PRESENT: u32 = 0x0048_cee0;
const LEAF_TEXTURE_NAME: u32 = 0x0040_8da0;
const TES: u32 = 0x011d_ea10;
const CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;
const GET_BILLBOARD_TEXTURE_FILENAME: u32 = 0x0051_c3b0;
/// The tree form's virtual slots `ApplyBaseObject` reads floats from.
const TREE_SLOT_1AC: u32 = 0x1ac;
const TREE_SLOT_1B4: u32 = 0x1b4;
const TREE_SLOT_194: u32 = 0x194;
const TREE_SLOT_19C: u32 = 0x19c;
const TREE_SLOT_1A4: u32 = 0x1a4;
/// `0.0` (`01012060`), `1.0` (`01012070`) and `90.0` (`01066808`)
/// (`double`).
const ZERO: u32 = 0x0101_2060;
const ONE: u32 = 0x0101_2070;
const RIGHT_ANGLE: u32 = 0x0106_6808;
/// `CreateGeometry`'s calls: the branch (`00667470`) and leaf (`00668500`)
/// geometry, the tree's billboard test (`0051d250`) and billboard
/// (`00669500`), the model path (the form's model +0x30, virtual slot
/// `+0x14`), the collision file's stream (`BSStream`: constructor
/// `0043cfd0`, `Load` `00c3a8a0`, object count `0043b1e0`, object
/// `0043b200`, destructor `0043d090`), `FileFinder::Exist` (`00456a20`),
/// the cast to `NiAVObject` (`00653270` with `011f4280`), the collision
/// object (`004a8b00`) and its body (`006fa820`), and
/// `bhkUtilFunctions::CreateTree` (`00c81380`).
const BILLBOARD_PRESENT: u32 = 0x0051_d250;
const CREATE_BILLBOARD: u32 = 0x0066_9500;
const MODEL_PATH_SLOT: u32 = 0x14;
const STREAM_CONSTRUCTOR: u32 = 0x0043_cfd0;
const STREAM_LOAD: u32 = 0x00c3_a8a0;
const STREAM_OBJECT_COUNT: u32 = 0x0043_b1e0;
const STREAM_OBJECT: u32 = 0x0043_b200;
const STREAM_DESTRUCTOR: u32 = 0x0043_d090;
const FILE_EXISTS: u32 = 0x0045_6a20;
const DYNAMIC_CAST: u32 = 0x0065_3270;
const NI_AV_OBJECT_RTTI: u32 = 0x011f_4280;
const COLLISION_OBJECT: u32 = 0x004a_8b00;
const COLLISION_BODY: u32 = 0x006f_a820;
const CREATE_TREE_BODY: u32 = 0x00c8_1380;
/// The collision file's prefix `"Data\Meshes\Trees"` (`01066818`) and
/// extension `".NIF"` (`01066810`).
const COLLISION_PREFIX: u32 = 0x0106_6818;
const COLLISION_EXTENSION: u32 = 0x0106_6810;
/// `64.0` (`010240c0`) and `2.0` (`01011590`) (`double`): the smallest
/// trunk height that gets a body, and the trunk width's divisor.
const MINIMUM_TRUNK_HEIGHT: u32 = 0x0102_40c0;
const TWO: u32 = 0x0101_1590;

/// The word an `NiPointer` at `slot` holds (through `00559450`).
fn pointer(e: &mut Engine, slot: u32) -> u32 {
    e.call(NI_POINTER_GET, &args![slot]).u32()
}

/// A float setting's value in `ST0` (the float copied through the x87
/// stack).
fn setting_value(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_FLOAT_VALUE, &args![setting]).u32();
    f32::from_bits(x87_quiet_bits(e.mem.u32(value)))
}

// Translated from 00666800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeModel::~BSTreeModel` (Xbox PDB): clears the model
/// ([`bs_tree_model_clear_model`]), deletes the SpeedTree when there is one
/// ([`fn_00666910`] with 1), destroys the pointers and runs `NiRefObject`'s
/// destructor.
// Translation incomplete: 00666800 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn bs_tree_model_destructor(e: &mut Engine, this: Ptr<BSTreeModel>) {
    let t = this.addr();
    e.mem.set_u32(t, BS_TREE_MODEL_VTABLE);
    bs_tree_model_clear_model(e, this);
    if e.get(this, BSTreeModel::pSpeedTree) != 0 {
        let tree = e.get(this, BSTreeModel::pSpeedTree);
        if tree != 0 {
            fn_00666910(e, Ptr::new(tree), 1);
        }
        e.set(this, BSTreeModel::pSpeedTree, 0);
    }
    for offset in [0x38u32, 0x34, 0x30, 0x2c, 0x20, 0x1c, 0x10] {
        e.call(NI_POINTER_DESTRUCT, &args![t + offset]);
    }
    e.call(REF_OBJECT_DESTRUCTOR, &args![this]);
}

// Translated from 00666910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CSpeedTreeRT` scalar deleting destructor (no PDB name): the
/// destructor (`00b03b30`) and `operator delete` when `flags & 1`. Returns
/// `this`.
pub fn fn_00666910(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(SPEED_TREE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// A float from the tree form's virtual slot `slot`, stored as a float.
fn tree_value(e: &mut Engine, tree: Ptr, slot: u32) -> u32 {
    x87_store32(e.vcall(tree.addr(), slot, &args![]).f64())
}

/// A new `BSShaderTextureSet` (0x38 bytes; null when the allocation fails).
fn new_texture_set(e: &mut Engine) -> u32 {
    let block = e.call(NI_NEW, &args![0x38u32]).u32();
    if block == 0 {
        0
    } else {
        e.call(TEXTURE_SET_CONSTRUCTOR, &args![block]).u32()
    }
}

// Translated from 00666940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeModel::ApplyBaseObject` (Xbox PDB): with a tree form and a
/// SpeedTree of the first kind: takes two wind values from their settings
/// (or, out of `0 ..= 1`, from the form's slots `+0x1AC`, `+0x1B4`) into
/// the SpeedTree when they are in range; the curve scalar from its setting
/// (or, below 0, the slot `+0x194`) when not negative; two angles from
/// their settings (or, unless both are in `0 ..= 90` and ordered, the
/// slots `+0x19C`, `+0x1A4`) into the SpeedTree when they are. Then,
/// inside the allocation scope, gives the model what it lacks: a branch
/// texture set from the SpeedTree's branch texture (`"Textures\Trees\
/// Branches\<name>"` with the extension made `dds`, and its `_n` normal
/// map), the leaf frond texture (`"Textures\Trees\Leaves\<name>"`) when
/// the form has one, and a billboard texture set from the form's billboard
/// texture.
// Translation incomplete: 00666940 - the compiler's exception frame (FS:[0] chain) and the /GS stack cookie check (00ec408c) are not translated
pub fn bs_tree_model_apply_base_object(e: &mut Engine, this: Ptr<BSTreeModel>, tree: Ptr) {
    let t = this.addr();
    let zero = e.global::<f64>(ZERO);
    let one = e.global::<f64>(ONE);
    let right = e.global::<f64>(RIGHT_ANGLE);
    e.with_stack(4, |e, held| {
        e.call(NI_POINTER_CONSTRUCT, &args![held, 0u32]);
        if tree.is_null()
            || e.get(this, BSTreeModel::pSpeedTree) == 0
            || bs_tree_model_fn_00666f00(e, this)
        {
            e.call(NI_POINTER_DESTRUCT, &args![held]);
            return;
        }
        for (setting, slot, setter) in [
            (0x0066_6f40u32, TREE_SLOT_1AC, SPEED_TREE_SET_00B07390),
            (0x0066_6f60, TREE_SLOT_1B4, SPEED_TREE_SET_00B073C0),
        ] {
            let mut value = x87_store32(f64::from(e.call(setting, &args![]).f32()));
            let v = x87_load32(value);
            if v < zero || v > one {
                value = tree_value(e, tree, slot);
            }
            let v = x87_load32(value);
            if zero <= v && v <= one {
                let speed_tree = e.get(this, BSTreeModel::pSpeedTree);
                e.call(setter, &args![speed_tree, x87_quiet_bits(value)]);
            }
        }
        let mut curve = x87_store32(f64::from(fn_00666f20(e)));
        if x87_load32(curve) < zero {
            curve = tree_value(e, tree, TREE_SLOT_194);
        }
        if x87_load32(curve) >= zero {
            e.set(this, BSTreeModel::fCurveScalar, x87_quiet_bits(curve));
        }
        let mut low = x87_store32(f64::from(fn_00666f80(e)));
        let mut high = x87_store32(f64::from(fn_00666fa0(e)));
        let (a, b) = (x87_load32(low), x87_load32(high));
        if a < zero || a > right || b < zero || b > right || b < a {
            low = tree_value(e, tree, TREE_SLOT_19C);
            high = tree_value(e, tree, TREE_SLOT_1A4);
        }
        let (a, b) = (x87_load32(low), x87_load32(high));
        if zero <= a && a <= right && zero <= b && b <= right && b >= a {
            let speed_tree = e.get(this, BSTreeModel::pSpeedTree);
            e.call(
                SPEED_TREE_SET_00B07490,
                &args![speed_tree, x87_quiet_bits(low)],
            );
            let speed_tree = e.get(this, BSTreeModel::pSpeedTree);
            e.call(
                SPEED_TREE_SET_00B074C0,
                &args![speed_tree, x87_quiet_bits(high)],
            );
        }
        // Stack: +0 the scope, +0x10 the name holder (0x1C), +0x30 the
        // branch path, +0x134 its normal map, +0x238 the leaf path, +0x33C
        // the billboard path (0x104 bytes each).
        e.with_stack(0x440, |e, locals| {
            let scope = locals.addr();
            let names = scope + 0x10;
            let branch = scope + 0x30;
            let normal_map = scope + 0x134;
            let leaf = scope + 0x238;
            let billboard = scope + 0x33c;
            e.call(
                SCOPE_OPEN,
                &args![scope, 0x1eu32, 1u32, SOURCE_FILE, 0x91u32],
            );
            if pointer(e, t + 0x2c) == 0 {
                e.call(NAME_HOLDER_CONSTRUCTOR, &args![names]);
                let speed_tree = e.get(this, BSTreeModel::pSpeedTree);
                e.call(SPEED_TREE_TEXTURE_NAMES, &args![speed_tree, names]);
                e.call(STRCPY, &args![branch, 0x104u32, BRANCHES_FOLDER]);
                let name = e.mem.u32(names);
                e.call(STRCAT, &args![branch, 0x104u32, name]);
                let length = e.call(STRLEN, &args![branch]).u32();
                e.mem
                    .set_u8(branch.wrapping_add(length).wrapping_sub(3), b'd');
                e.mem
                    .set_u8(branch.wrapping_add(length).wrapping_sub(2), b'd');
                e.mem
                    .set_u8(branch.wrapping_add(length).wrapping_sub(1), b's');
                e.call(
                    GET_MODIFIED_TEXTURE_FILENAME,
                    &args![normal_map, branch, NORMAL_MAP_SUFFIX, 1u32],
                );
                let set = new_texture_set(e);
                e.call(NI_POINTER_ASSIGN, &args![t + 0x2c, set]);
                let set = pointer(e, t + 0x2c);
                e.vcall(set, TEXTURE_SET_SET_SLOT, &args![0u32, branch]);
                let set = pointer(e, t + 0x2c);
                e.vcall(set, TEXTURE_SET_SET_SLOT, &args![1u32, normal_map]);
                e.call(NAME_HOLDER_DESTRUCTOR, &args![names]);
            }
            if pointer(e, t + 0x30) == 0
                && e.call(LEAF_TEXTURE_PRESENT, &args![tree.addr() + 0x48])
                    .u32()
                    != 0
            {
                e.call(STRCPY, &args![leaf, 0x104u32, LEAVES_FOLDER]);
                let name = e.call(LEAF_TEXTURE_NAME, &args![tree.addr() + 0x48]).u32();
                e.call(STRCAT, &args![leaf, 0x104u32, name]);
                let tes = e.global::<u32>(TES);
                e.call(
                    CREATE_TEXTURE_IMAGE,
                    &args![tes, leaf, t + 0x30, 1u32, 0u32],
                );
            }
            if pointer(e, t + 0x34) == 0 {
                e.call(GET_BILLBOARD_TEXTURE_FILENAME, &args![tree, billboard]);
                let set = new_texture_set(e);
                e.call(NI_POINTER_ASSIGN, &args![t + 0x34, set]);
                let set = pointer(e, t + 0x34);
                e.vcall(set, TEXTURE_SET_SET_SLOT, &args![0u32, billboard]);
            }
            e.call(SCOPE_CLOSE, &args![scope]);
        });
        e.call(NI_POINTER_DESTRUCT, &args![held]);
    });
}

// Translated from 00666f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the model's type (`eType`) is 2 (no PDB name).
pub fn bs_tree_model_fn_00666f00(e: &mut Engine, this: Ptr<BSTreeModel>) -> bool {
    e.get(this, BSTreeModel::eType) == 2
}

// Translated from 00666f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The curve scalar setting's value (`011d5d14`).
pub fn fn_00666f20(e: &mut Engine) -> f32 {
    setting_value(e, CURVE_SCALAR_SETTING)
}

// Translated from 00666f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of the setting at `011d5db8`.
pub fn fn_00666f40(e: &mut Engine) -> f32 {
    setting_value(e, SETTING_011D5DB8)
}

// Translated from 00666f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of the setting at `011d5d24`.
pub fn fn_00666f60(e: &mut Engine) -> f32 {
    setting_value(e, SETTING_011D5D24)
}

// Translated from 00666f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of the setting at `011d5c9c`.
pub fn fn_00666f80(e: &mut Engine) -> f32 {
    setting_value(e, SETTING_011D5C9C)
}

// Translated from 00666fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of the setting at `011d5c80`.
pub fn fn_00666fa0(e: &mut Engine) -> f32 {
    setting_value(e, SETTING_011D5C80)
}

/// Deletes the pointer array in `slot` ([`fn_00667120`] with 3) and clears
/// the slot.
fn delete_pointer_array(e: &mut Engine, slot: u32) {
    if e.mem.u32(slot) != 0 {
        let array = e.mem.u32(slot);
        if array != 0 {
            fn_00667120(e, Ptr::new(array), 3);
        }
        e.mem.set_u32(slot, 0);
    }
}

// Translated from 00666fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeModel::ClearModel` (Xbox PDB): deletes the branch and leaf data
/// arrays and the branch and leaf shader property arrays, and drops the
/// billboard, the leaf shader data, the three texture sets and the rigid
/// body. (With branch data it also asks [`bs_tree_model_fn_00666f00`],
/// whose answer is not used.)
pub fn bs_tree_model_clear_model(e: &mut Engine, this: Ptr<BSTreeModel>) {
    let t = this.addr();
    if e.get(this, BSTreeModel::pspBranchData) != 0 {
        bs_tree_model_fn_00666f00(e, this);
        let array = e.get(this, BSTreeModel::pspBranchData);
        if array != 0 {
            fn_00667120(e, Ptr::new(array), 3);
        }
        e.set(this, BSTreeModel::pspBranchData, 0);
    }
    delete_pointer_array(e, t + 0x18);
    e.call(NI_POINTER_ASSIGN, &args![t + 0x1c, 0u32]);
    e.call(NI_POINTER_ASSIGN, &args![t + 0x20, 0u32]);
    delete_pointer_array(e, t + 0x24);
    delete_pointer_array(e, t + 0x28);
    for offset in [0x2cu32, 0x30, 0x34, 0x38] {
        e.call(NI_POINTER_ASSIGN, &args![t + offset, 0u32]);
    }
}

// Translated from 00667120 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `NiPointer` array's vector deleting destructor (no PDB name): with
/// `flags & 2`, destroys the elements (their count in the word before
/// `this`) and frees the block with its count when `flags & 1`, answering
/// the block; otherwise destroys the one pointer and frees it when
/// `flags & 1`, answering `this`.
pub fn fn_00667120(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let t = this.addr();
    if flags & 2 != 0 {
        let count = e.mem.u32(t.wrapping_sub(4));
        e.call(
            VECTOR_DESTRUCTOR_ITERATOR,
            &args![this, 4u32, count, NI_POINTER_DESTRUCT],
        );
        if flags & 1 != 0 {
            e.call(OPERATOR_DELETE, &args![t.wrapping_sub(4)]);
        }
        return Ptr::new(t.wrapping_sub(4));
    }
    e.call(NI_POINTER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00667190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeModel::CreateGeometry` (Xbox PDB): with a SpeedTree of the first
/// kind, builds the branch and leaf geometry (`00667470`, `00668500` with
/// the form) and the billboard when the form has one (`00669500`); then
/// drops the rigid body and loads the collision file
/// (`<prefix><model path without its last 4 characters>.NIF`) when it
/// exists: a stream with exactly one object whose collision object has a
/// body gives the rigid body. Without one, a trunk taller than 64 gets a
/// body from `bhkUtilFunctions::CreateTree(height, width / 2)`.
// Translation incomplete: 00667190 - the compiler's exception frame (FS:[0] chain) and the /GS stack cookie check (00ec408c) are not translated
pub fn bs_tree_model_create_geometry(e: &mut Engine, this: Ptr<BSTreeModel>, tree: Ptr) {
    let t = this.addr();
    if e.get(this, BSTreeModel::pSpeedTree) == 0 || bs_tree_model_fn_00666f00(e, this) {
        return;
    }
    e.call(0x0066_7470, &args![this]);
    e.call(0x0066_8500, &args![this, tree]);
    if e.call(BILLBOARD_PRESENT, &args![tree]).bool() {
        e.call(CREATE_BILLBOARD, &args![this, tree]);
    }
    e.call(NI_POINTER_ASSIGN, &args![t + 0x38, 0u32]);
    // Stack: +0 the model path, +0x104 the collision path, +0x208 the
    // stream (0x5D8 bytes, to the end of the frame's locals).
    e.with_stack(0x7e0, |e, locals| {
        let path = locals.addr();
        let collision = path + 0x104;
        let stream = path + 0x208;
        let model = tree.addr() + 0x30;
        let name = e.vcall(model, MODEL_PATH_SLOT, &args![]).u32();
        e.call(STRCPY, &args![path, 0x104u32, name]);
        let length = e.call(STRLEN, &args![path]).i32();
        if length > 4 {
            e.mem
                .set_u8(path.wrapping_add(length as u32).wrapping_sub(4), 0);
        }
        e.call(STREAM_CONSTRUCTOR, &args![stream]);
        e.call(STRCPY, &args![collision, 0x104u32, COLLISION_PREFIX]);
        e.call(STRCAT, &args![collision, 0x104u32, path]);
        e.call(STRCAT, &args![collision, 0x104u32, COLLISION_EXTENSION]);
        let found = e
            .call(FILE_EXISTS, &args![collision, 0u32, 6u32, 1u32])
            .u32()
            != 0;
        if found
            && e.call(STREAM_LOAD, &args![stream, collision, 0u32]).bool()
            && e.call(STREAM_OBJECT_COUNT, &args![stream]).u32() == 1
        {
            let object = e.call(STREAM_OBJECT, &args![stream, 0u32]).u32();
            let object = e
                .call(DYNAMIC_CAST, &args![NI_AV_OBJECT_RTTI, object])
                .u32();
            let collision_object = e.call(COLLISION_OBJECT, &args![object]).u32();
            let body = if collision_object == 0 {
                0
            } else {
                e.call(COLLISION_BODY, &args![collision_object]).u32()
            };
            if body != 0 {
                e.call(NI_POINTER_ASSIGN, &args![t + 0x38, body]);
            }
        }
        if pointer(e, t + 0x38) == 0
            && x87_load32(e.get(this, BSTreeModel::fTrunkHeight))
                > e.global::<f64>(MINIMUM_TRUNK_HEIGHT)
        {
            let width = e.get(this, BSTreeModel::fTrunkWidth);
            let radius = x87_store32(x87_div(x87_load32(width), e.global::<f64>(TWO)));
            let height = x87_quiet_bits(e.get(this, BSTreeModel::fTrunkHeight));
            let body = e
                .call(CREATE_TREE_BODY, &args![height, x87_quiet_bits(radius)])
                .u32();
            e.call(NI_POINTER_ASSIGN, &args![t + 0x38, body]);
        }
        e.call(STREAM_DESTRUCTOR, &args![stream]);
    });
}

// Translated from 006684d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a pointer (+8) and a 16-bit count (+0xC) in `this` (no PDB name;
/// the geometry code gives it the colour array and the vertex count).
pub fn fn_006684d0(e: &mut Engine, this: Ptr, data: u32, count: u16) {
    e.mem.set_u32(this.addr() + 8, data);
    e.mem.set_u16(this.addr() + 0x0c, count);
}

// Translated from 006694c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at `this + 0x14` (no PDB name).
pub fn fn_006694c0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0x14, x87_quiet_bits(value.to_bits()));
}

// Translated from 006698d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x6C` (through `00559450`).
pub fn fn_006698d0(e: &mut Engine, this: Ptr) -> u32 {
    pointer(e, this.addr() + 0x6c)
}

// Translated from 006698f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeModel::CreateAlphaProperty` (Xbox PDB): a new alpha property
/// (0x1C bytes; null when the allocation fails) given `0049ed90(0)`,
/// `004393c0(1)`, `004393e0(4)` and `0094db80(0x54)`.
// Translation incomplete: 006698f0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn bs_tree_model_create_alpha_property(e: &mut Engine) -> Ptr {
    let block = e.call(NI_NEW, &args![0x1cu32]).u32();
    let property = if block == 0 {
        0
    } else {
        e.call(ALPHA_PROPERTY_CONSTRUCTOR, &args![block]).u32()
    };
    e.call(ALPHA_SET_0049ED90, &args![property, 0u32]);
    e.call(ALPHA_SET_004393C0, &args![property, 1u32]);
    e.call(ALPHA_SET_004393E0, &args![property, 4u32]);
    e.call(ALPHA_SET_0094DB80, &args![property, 0x54u32]);
    Ptr::new(property)
}

// Translated from 00669990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeModel::GetNumBranchLODLevels` (Xbox PDB): the SpeedTree's branch
/// level count, 0 without a SpeedTree.
pub fn bs_tree_model_get_num_branch_lod_levels(e: &mut Engine, this: Ptr<BSTreeModel>) -> u16 {
    let tree = e.get(this, BSTreeModel::pSpeedTree);
    if tree == 0 {
        0
    } else {
        e.call(SPEED_TREE_BRANCH_LODS, &args![tree]).u16()
    }
}

// Translated from 006699d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The SpeedTree's leaf level count (`00b08710`), 0 without a SpeedTree
/// (no PDB name).
pub fn fn_006699d0(e: &mut Engine, this: Ptr<BSTreeModel>) -> u16 {
    let tree = e.get(this, BSTreeModel::pSpeedTree);
    if tree == 0 {
        0
    } else {
        e.call(SPEED_TREE_LEAF_LODS, &args![tree]).u16()
    }
}

// Translated from 00669fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x1B of the tree manager's instance (created when there is
/// none, `00664840(1)`).
pub fn fn_00669fb0(e: &mut Engine) -> u8 {
    let manager = e.call(TREE_MANAGER_INSTANCE, &args![1u32]).u32();
    e.mem.u8(manager + 0x1b)
}

// Translated from 0066a630 (decompiled, FalloutNV.exe 1.4.0.525)
/// The rigid body (`spRigidBody`, through `00559450`).
pub fn fn_0066a630(e: &mut Engine, this: Ptr<BSTreeModel>) -> u32 {
    pointer(e, this.addr() + 0x38)
}

/// `operator new` for an array of `count` `NiPointer`s with its count in
/// front (the size `count * 4 + 4`, all ones on overflow), the pointers
/// constructed null (`00ec782f` with `006694e0`, `0045cec0`). Answers the
/// first pointer, null when the allocation fails.
fn new_pointer_array(e: &mut Engine, count: u32) -> u32 {
    let (product, overflow) = count.overflowing_mul(4);
    let size = if overflow { u32::MAX } else { product };
    let (size, carry) = size.overflowing_add(4);
    let size = if carry { u32::MAX } else { size };
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![block + 4, 4u32, count, 0x0066_94e0u32, NI_POINTER_DESTRUCT],
    );
    block + 4
}

/// The tree's lock (`011d5df8`: `004538a0` with 0 takes it, `004538c0`
/// leaves it; thiscall).
const TREE_LOCK: u32 = 0x011d_5df8;
const LOCK_ENTER: u32 = 0x0045_38a0;
const LOCK_LEAVE: u32 = 0x0045_38c0;
/// `InitAsInstance`'s calls: the SpeedTree instance (`00b05210`, thiscall
/// on the base SpeedTree), the `NiPointer` copy (`006e5cc0`, thiscall:
/// source slot), the cloning process (`004ad050` with 1.0, destroyed by
/// `004ad270`), `NiObject::Clone` (`00a5d2c0`, with the process) and its
/// overload without one (`00a5d680`) (Xbox PDB names), and the byte
/// `011f4404` set while the billboard is cloned.
const SPEED_TREE_MAKE_INSTANCE: u32 = 0x00b0_5210;
const NI_POINTER_COPY: u32 = 0x006e_5cc0;
const CLONING_PROCESS_CONSTRUCTOR: u32 = 0x004a_d050;
const CLONING_PROCESS_DESTRUCTOR: u32 = 0x004a_d270;
const NI_OBJECT_CLONE_WITH: u32 = 0x00a5_d2c0;
const NI_OBJECT_CLONE: u32 = 0x00a5_d680;
const CLONING_BILLBOARD: u32 = 0x011f_4404;

/// Gives `this`'s array in `slot` a copy of `base`'s array's pointers
/// (`count` of them, the `NiPointer` copy) and its shader property array
/// in `properties` clones of `base`'s properties (null where `base` has
/// none).
fn copy_instance_arrays(
    e: &mut Engine,
    this: u32,
    base: u32,
    slot: u32,
    properties: u32,
    count: u16,
) {
    let data = new_pointer_array(e, u32::from(count));
    e.mem.set_u32(this + slot, data);
    let shaders = new_pointer_array(e, u32::from(count));
    e.mem.set_u32(this + properties, shaders);
    let mut index: u16 = 0;
    while u32::from(index) < u32::from(count) {
        let i = u32::from(index) * 4;
        let source = e.mem.u32(base + slot) + i;
        let target = e.mem.u32(this + slot) + i;
        e.call(NI_POINTER_COPY, &args![target, source]);
        let property = pointer(e, e.mem.u32(base + properties) + i);
        let target = e.mem.u32(this + properties) + i;
        if property != 0 {
            let property = pointer(e, e.mem.u32(base + properties) + i);
            let clone = e.call(NI_OBJECT_CLONE, &args![property]).u32();
            let target = e.mem.u32(this + properties) + i;
            e.call(NI_POINTER_ASSIGN, &args![target, clone]);
        } else {
            e.call(NI_POINTER_ASSIGN, &args![target, 0u32]);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 0066a650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeModel::InitAsInstance` (Xbox PDB): under the tree lock and inside
/// the allocation scope, makes an uninitialised model (`eType` 0) an
/// instance of `base` (one with a SpeedTree, not of the second kind):
/// an instance of its SpeedTree (false when there is none), `base` as the
/// parent, type 2, its texture sets and leaf shader data, curve scalar,
/// seed and trunk size, a clone of its rigid body, its branch and leaf data
/// with clones of their shader properties, and a clone of its billboard.
/// Answers whether it did.
// Translation incomplete: 0066a650 - the compiler's exception frame (FS:[0] chain) is not translated
pub fn bs_tree_model_init_as_instance(
    e: &mut Engine,
    this: Ptr<BSTreeModel>,
    base: Ptr<BSTreeModel>,
) -> bool {
    let t = this.addr();
    let b = base.addr();
    e.call(LOCK_ENTER, &args![TREE_LOCK, 0u32]);
    e.with_stack(0x38, |e, locals| {
        let scope = locals.addr();
        let process = scope + 4;
        e.call(
            SCOPE_OPEN,
            &args![scope, 0x1fu32, 1u32, SOURCE_FILE, 0x411u32],
        );
        if base.is_null()
            || e.get(base, BSTreeModel::pSpeedTree) == 0
            || bs_tree_model_fn_00666f00(e, base)
            || e.get(this, BSTreeModel::eType) != 0
        {
            e.call(LOCK_LEAVE, &args![TREE_LOCK]);
            e.call(SCOPE_CLOSE, &args![scope]);
            return false;
        }
        let base_tree = e.get(base, BSTreeModel::pSpeedTree);
        let instance = e.call(SPEED_TREE_MAKE_INSTANCE, &args![base_tree]).u32();
        e.set(this, BSTreeModel::pSpeedTree, instance);
        if e.get(this, BSTreeModel::pSpeedTree) == 0 {
            e.call(LOCK_LEAVE, &args![TREE_LOCK]);
            e.call(SCOPE_CLOSE, &args![scope]);
            return false;
        }
        e.call(NI_POINTER_ASSIGN, &args![t + 0x10, base]);
        e.set(this, BSTreeModel::eType, 2);
        for offset in [0x2cu32, 0x30, 0x34, 0x20] {
            e.call(NI_POINTER_COPY, &args![t + offset, b + offset]);
        }
        let curve = e.get(base, BSTreeModel::fCurveScalar);
        e.set(this, BSTreeModel::fCurveScalar, x87_quiet_bits(curve));
        let seed = e.get(base, BSTreeModel::iSeed);
        e.set(this, BSTreeModel::iSeed, seed);
        let height = e.get(base, BSTreeModel::fTrunkHeight);
        e.set(this, BSTreeModel::fTrunkHeight, x87_quiet_bits(height));
        let width = e.get(base, BSTreeModel::fTrunkWidth);
        e.set(this, BSTreeModel::fTrunkWidth, x87_quiet_bits(width));
        if pointer(e, b + 0x38) != 0 {
            e.call(
                CLONING_PROCESS_CONSTRUCTOR,
                &args![process, 1.0f32.to_bits()],
            );
            let body = pointer(e, b + 0x38);
            let clone = e.call(NI_OBJECT_CLONE_WITH, &args![body, process]).u32();
            e.call(NI_POINTER_ASSIGN, &args![t + 0x38, clone]);
            e.call(CLONING_PROCESS_DESTRUCTOR, &args![process]);
        } else {
            e.call(NI_POINTER_ASSIGN, &args![t + 0x38, 0u32]);
        }
        let count = bs_tree_model_get_num_branch_lod_levels(e, this);
        if count > 0
            && e.get(base, BSTreeModel::pspBranchData) != 0
            && e.get(base, BSTreeModel::pspBranchShaderProperties) != 0
        {
            copy_instance_arrays(e, t, b, 0x14, 0x24, count);
        }
        let count = fn_006699d0(e, this);
        if count > 0
            && e.get(base, BSTreeModel::pspLeafData) != 0
            && e.get(base, BSTreeModel::pspLeafShaderProperties) != 0
        {
            copy_instance_arrays(e, t, b, 0x18, 0x28, count);
        }
        if pointer(e, b + 0x1c) != 0 {
            e.mem.set_u8(CLONING_BILLBOARD, 1);
            let billboard = pointer(e, b + 0x1c);
            let clone = e.call(NI_OBJECT_CLONE, &args![billboard]).u32();
            e.call(NI_POINTER_ASSIGN, &args![t + 0x1c, clone]);
            e.mem.set_u8(CLONING_BILLBOARD, 0);
        }
        e.call(LOCK_LEAVE, &args![TREE_LOCK]);
        e.call(SCOPE_CLOSE, &args![scope]);
        true
    })
}

/// `InitFromBase`'s calls on the new SpeedTree (thiscall unless noted):
/// its constructor (`00b02ef0`, 0xA0 bytes), `LoadTree` (`00b05420`),
/// `SetNumLeafRockingGroups` (`00b071f0`), `00b071c0`, the size out
/// (`00b06260`), `SetTreeSize` (`00b06280`), `SetWindStrength`
/// (`00b07b00`), `SetLocalMatrices` (`00b08050`), `00984f60`, the wind
/// and lighting methods (`00b076e0`, `00b07900`, `00b07510`, `00b06660`,
/// `00b069c0`, `00b06810`), `00b08500`, `Compute` (`00b044a0`),
/// `00b087d0`, the seed (`00b06440`), the trunk height and width
/// (`00b07440`, `00b073f0`), `DeleteTransientData` (`00b053e0`),
/// `00b088d0` and `00b0a7a0` (Xbox PDB names where named).
const SPEED_TREE_CONSTRUCTOR: u32 = 0x00b0_2ef0;
const SPEED_TREE_LOAD_TREE: u32 = 0x00b0_5420;
const SPEED_TREE_SET_ROCKING_GROUPS: u32 = 0x00b0_71f0;
const SPEED_TREE_00B071C0: u32 = 0x00b0_71c0;
const SPEED_TREE_GET_SIZE: u32 = 0x00b0_6260;
const SPEED_TREE_SET_TREE_SIZE: u32 = 0x00b0_6280;
const SPEED_TREE_SET_WIND_STRENGTH: u32 = 0x00b0_7b00;
const SPEED_TREE_SET_LOCAL_MATRICES: u32 = 0x00b0_8050;
const SPEED_TREE_00984F60: u32 = 0x0098_4f60;
const SPEED_TREE_METHODS: [u32; 6] = [
    0x00b0_76e0,
    0x00b0_7900,
    0x00b0_7510,
    0x00b0_6660,
    0x00b0_69c0,
    0x00b0_6810,
];
const SPEED_TREE_00B08500: u32 = 0x00b0_8500;
const SPEED_TREE_COMPUTE: u32 = 0x00b0_44a0;
const SPEED_TREE_00B087D0: u32 = 0x00b0_87d0;
const SPEED_TREE_SEED: u32 = 0x00b0_6440;
const SPEED_TREE_TRUNK_HEIGHT: u32 = 0x00b0_7440;
const SPEED_TREE_TRUNK_WIDTH: u32 = 0x00b0_73f0;
const SPEED_TREE_DELETE_TRANSIENT_DATA: u32 = 0x00b0_53e0;
const SPEED_TREE_00B088D0: u32 = 0x00b0_88d0;
const SPEED_TREE_00B0A7A0: u32 = 0x00b0_a7a0;
/// `"Trees"` (`010668a0`), the too-many-leaf-maps message (`01066840`)
/// and the log call (`005b5e40`, cdecl).
const TREES_FOLDER: u32 = 0x0106_68a0;
const TOO_MANY_LEAF_MAPS: u32 = 0x0106_6840;
const LOG: u32 = 0x005b_5e40;
/// The size scale (`0066b0b0`) and the two values `00b08500` takes
/// (`006655c0`, `00665570`), cdecl, floats in `ST0`.
const TREE_SIZE_SCALE: u32 = 0x0066_b0b0;
const VALUE_006655C0: u32 = 0x0066_55c0;
const VALUE_00665570: u32 = 0x0066_5570;
/// `-1.0` (`01012054`, `float`).
const MINUS_ONE: u32 = 0x0101_2054;
/// The model's virtual slots `InitFromBase` calls with the form (`+0x10`
/// before the compute, `+0x18` after).
const MODEL_SLOT_10: u32 = 0x10;
const MODEL_SLOT_18: u32 = 0x18;

// Translated from 0066ac40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeModel::InitFromBase` (Xbox PDB): for an uninitialised model and a
/// form with a model path, loads `"Trees" + path` into a new SpeedTree
/// inside the allocation scope. When it loads: two leaf rocking groups
/// with at most three leaf maps (otherwise one, logging when there are
/// more than six), the size scaled by `0066b0b0`, no wind, local matrices,
/// the wind and lighting methods 0; the model takes it (type 1), runs its
/// `+0x10` slot with the form and computes the tree with `seed`; when that
/// works it reads the seed and trunk size, runs its `+0x18` slot and
/// drops the transient data. A tree that does not load is deleted.
/// Answers false when loading or computing fails, true otherwise.
// Translation incomplete: 0066ac40 - the compiler's exception frame (FS:[0] chain) and the /GS stack cookie check (00ec408c) are not translated
pub fn bs_tree_model_init_from_base(
    e: &mut Engine,
    this: Ptr<BSTreeModel>,
    form: Ptr,
    seed: u32,
) -> bool {
    let t = this.addr();
    if form.is_null() {
        return true;
    }
    let model = form.addr() + 0x30;
    if e.vcall(model, MODEL_PATH_SLOT, &args![]).u32() == 0 {
        return true;
    }
    let path = e.vcall(model, MODEL_PATH_SLOT, &args![]).u32();
    if e.mem.u8(path) == 0 || e.get(this, BSTreeModel::eType) != 0 {
        return true;
    }
    // Stack: +0 the scope, +4 the path (0x104), +0x108 the name holder
    // (0x1C), +0x124 the two size words.
    e.with_stack(0x130, |e, locals| {
        let scope = locals.addr();
        let name = scope + 4;
        let names = scope + 0x108;
        let size_a = scope + 0x124;
        let size_b = scope + 0x128;
        e.call(
            SCOPE_OPEN,
            &args![scope, 0x1fu32, 1u32, SOURCE_FILE, 0x473u32],
        );
        e.call(STRCPY, &args![name, 0x104u32, TREES_FOLDER]);
        let path = e.vcall(model, MODEL_PATH_SLOT, &args![]).u32();
        e.call(STRCAT, &args![name, 0x104u32, path]);
        let block = e.call(OPERATOR_NEW, &args![0xa0u32]).u32();
        let tree = if block == 0 {
            0
        } else {
            e.call(SPEED_TREE_CONSTRUCTOR, &args![block]).u32()
        };
        if !e.call(SPEED_TREE_LOAD_TREE, &args![tree, name]).bool() {
            if tree != 0 {
                fn_00666910(e, Ptr::new(tree), 1);
            }
            e.call(SCOPE_CLOSE, &args![scope]);
            return false;
        }
        e.call(NAME_HOLDER_CONSTRUCTOR, &args![names]);
        e.call(SPEED_TREE_TEXTURE_NAMES, &args![tree, names]);
        let leaf_maps = e.mem.u32(names + 4);
        if leaf_maps <= 3 {
            e.call(SPEED_TREE_SET_ROCKING_GROUPS, &args![tree, 2u32]);
        } else {
            e.call(SPEED_TREE_SET_ROCKING_GROUPS, &args![tree, 1u32]);
            if leaf_maps > 6 {
                let path = e.vcall(model, MODEL_PATH_SLOT, &args![]).u32();
                e.call(LOG, &args![TOO_MANY_LEAF_MAPS, path]);
            }
        }
        e.call(SPEED_TREE_00B071C0, &args![tree, 1u32]);
        e.call(SPEED_TREE_GET_SIZE, &args![tree, size_b, size_a]);
        let scale = e.call(TREE_SIZE_SCALE, &args![]).f64();
        let second = x87_store32(x87_mul(scale, x87_m32(e.mem.u32(size_a))));
        let scale = e.call(TREE_SIZE_SCALE, &args![]).f64();
        let first = x87_store32(x87_mul(scale, x87_m32(e.mem.u32(size_b))));
        e.call(
            SPEED_TREE_SET_TREE_SIZE,
            &args![tree, x87_quiet_bits(first), x87_quiet_bits(second)],
        );
        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
        e.call(
            SPEED_TREE_SET_WIND_STRENGTH,
            &args![tree, 0.0f32.to_bits(), minus_one, minus_one],
        );
        e.call(SPEED_TREE_SET_LOCAL_MATRICES, &args![tree, 0u32, 4u32]);
        e.call(SPEED_TREE_00984F60, &args![tree, 1u32]);
        for method in SPEED_TREE_METHODS {
            e.call(method, &args![tree, 0u32]);
        }
        let low = x87_store32(e.call(VALUE_006655C0, &args![]).f64());
        let high = x87_store32(e.call(VALUE_00665570, &args![]).f64());
        e.call(SPEED_TREE_00B08500, &args![tree, high, low]);
        e.call(NAME_HOLDER_DESTRUCTOR, &args![names]);
        e.set(this, BSTreeModel::eType, 1);
        e.set(this, BSTreeModel::pSpeedTree, tree);
        e.vcall(t, MODEL_SLOT_10, &args![form]);
        if !e
            .call(SPEED_TREE_COMPUTE, &args![tree, 0u32, seed, 1u32])
            .bool()
        {
            e.call(SCOPE_CLOSE, &args![scope]);
            return false;
        }
        e.call(SPEED_TREE_00B087D0, &args![tree]);
        let seed = e.call(SPEED_TREE_SEED, &args![tree]).u32();
        e.set(this, BSTreeModel::iSeed, seed);
        let height = x87_store32(e.call(SPEED_TREE_TRUNK_HEIGHT, &args![tree]).f64());
        e.set(this, BSTreeModel::fTrunkHeight, height);
        let width = x87_store32(e.call(SPEED_TREE_TRUNK_WIDTH, &args![tree]).f64());
        e.set(this, BSTreeModel::fTrunkWidth, width);
        e.vcall(t, MODEL_SLOT_18, &args![form]);
        e.call(SPEED_TREE_DELETE_TRANSIENT_DATA, &args![tree]);
        e.call(SPEED_TREE_00B088D0, &args![tree]);
        e.call(SPEED_TREE_00B0A7A0, &args![tree]);
        e.call(SCOPE_CLOSE, &args![scope]);
        true
    })
}

/// The SpeedTree geometry record (`00b029f0` constructs it, `00b02ac0`
/// destroys it; 0x134 bytes) and the point constructor (`006815c0`).
const GEOMETRY_CONSTRUCTOR: u32 = 0x00b0_29f0;
const GEOMETRY_DESTRUCTOR: u32 = 0x00b0_2ac0;
const POINT_CONSTRUCTOR: u32 = 0x0068_15c0;
/// `Update`'s calls: the node's slot `+0x100` (has it a parent),
/// `0054ee20` (cdecl, bool), the node's world translation (`0045bb80`),
/// the SpeedTree's camera position (`00b065e0`: x, y, z),
/// `SetLodLevel` (`00b08480`), `ComputeLodLevel` (`00b08200`),
/// `GetLodLevel` (`00b08440`), the geometry for that level (`00b0ad00`:
/// record, level), `GetDiscreteBranchLodLevel` (`00b08530`),
/// `BSTreeManager::AreLeavesVisible` (`005b9790`, cdecl), the wrapper
/// `00b051e0`, the leaf angles (`00b08af0`: out count) and the leaf shader
/// data's update (`00bb39e0`) (Xbox PDB names where named).
const NODE_HAS_PARENT_SLOT: u32 = 0x100;
const UPDATE_ALLOWED: u32 = 0x0054_ee20;
const NODE_WORLD_TRANSLATE: u32 = 0x0045_bb80;
const SPEED_TREE_SET_CAMERA: u32 = 0x00b0_65e0;
const SPEED_TREE_SET_LOD_LEVEL: u32 = 0x00b0_8480;
const SPEED_TREE_COMPUTE_LOD_LEVEL: u32 = 0x00b0_8200;
const SPEED_TREE_GET_LOD_LEVEL: u32 = 0x00b0_8440;
const SPEED_TREE_LOD_GEOMETRY: u32 = 0x00b0_ad00;
const SPEED_TREE_DISCRETE_BRANCH_LOD: u32 = 0x00b0_8530;
const ARE_LEAVES_VISIBLE: u32 = 0x005b_9790;
const SPEED_TREE_WRAPPER: u32 = 0x00b0_51e0;
const SPEED_TREE_LEAF_ANGLES: u32 = 0x00b0_8af0;
const LEAF_SHADER_DATA_UPDATE: u32 = 0x00bb_39e0;
/// The tree node's virtual slots `Update` drives (the engine map names
/// none): `+0x128` (branches shown), `+0x12C` (leaves shown), `+0x130`
/// (billboard shown), `+0x134` (branch level, alpha), `+0x138` (leaf
/// level, alpha) and `+0x13C` (billboard alpha).
const NODE_SLOT_128: u32 = 0x128;
const NODE_SLOT_12C: u32 = 0x12c;
const NODE_SLOT_130: u32 = 0x130;
const NODE_SLOT_134: u32 = 0x134;
const NODE_SLOT_138: u32 = 0x138;
const NODE_SLOT_13C: u32 = 0x13c;

/// The low byte of `FISTP` (truncating) of a float at `address`.
fn chopped_byte(e: &mut Engine, address: u32) -> u32 {
    crate::x87::fistp_chop(x87_load32(e.mem.u32(address))) as u32 & 0xff
}

// Translated from 00669a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeModel::Update` (Xbox PDB): for a node with a parent and a model
/// with a SpeedTree, when `0054ee20` allows: gives the SpeedTree the
/// node's world translation, sets the level of detail (1.0 when the tree
/// manager's byte +0x1B is set, computed otherwise) and reads the geometry
/// for it. Shows the billboard with its alpha when the record has one;
/// at level 0 or below hides branches and leaves. Otherwise shows the
/// discrete branch level with its alpha (the rest 0xFF), and, when leaves
/// are visible, the record's two leaf levels with their alphas (the rest
/// 0xFF) and updates the leaf shader data with the leaf angles.
// Translation incomplete: 00669a10 - the compiler's exception frame (FS:[0] chain) is not translated
pub fn bs_tree_model_update(e: &mut Engine, this: Ptr<BSTreeModel>, node: Ptr) {
    let t = this.addr();
    let n = node.addr();
    // Stack: +0 the record (0x134), +0x134 the point (0xC), +0x140 the
    // angle count.
    e.with_stack(0x144, |e, locals| {
        let record = locals.addr();
        let point = record + 0x134;
        let angles = record + 0x140;
        e.call(GEOMETRY_CONSTRUCTOR, &args![record]);
        e.call(POINT_CONSTRUCTOR, &args![point]);
        e.mem.set_u32(angles, 0);
        if node.is_null()
            || e.vcall(n, NODE_HAS_PARENT_SLOT, &args![]).u32() == 0
            || e.get(this, BSTreeModel::pSpeedTree) == 0
            || !e.call(UPDATE_ALLOWED, &args![]).bool()
        {
            e.call(GEOMETRY_DESTRUCTOR, &args![record]);
            return;
        }
        let translate = e.call(NODE_WORLD_TRANSLATE, &args![node]).u32();
        for i in 0..3 {
            let word = e.mem.u32(translate + i * 4);
            e.mem.set_u32(point + i * 4, word);
        }
        let x = x87_quiet_bits(e.mem.u32(point));
        let y = x87_quiet_bits(e.mem.u32(point + 4));
        let z = x87_quiet_bits(e.mem.u32(point + 8));
        let tree = e.get(this, BSTreeModel::pSpeedTree);
        e.call(SPEED_TREE_SET_CAMERA, &args![tree, x, y, z]);
        let level = if fn_00669fb0(e) != 0 {
            let tree = e.get(this, BSTreeModel::pSpeedTree);
            e.call(SPEED_TREE_SET_LOD_LEVEL, &args![tree, 1.0f32.to_bits()]);
            1.0f32.to_bits()
        } else {
            let tree = e.get(this, BSTreeModel::pSpeedTree);
            e.call(SPEED_TREE_COMPUTE_LOD_LEVEL, &args![tree]);
            let tree = e.get(this, BSTreeModel::pSpeedTree);
            x87_store32(e.call(SPEED_TREE_GET_LOD_LEVEL, &args![tree]).f64())
        };
        let tree = e.get(this, BSTreeModel::pSpeedTree);
        e.call(
            SPEED_TREE_LOD_GEOMETRY,
            &args![tree, record, x87_quiet_bits(level)],
        );
        let mut shown = 0u32;
        if e.mem.u8(record + 0xf0) != 0 {
            shown = 1;
            let alpha = chopped_byte(e, record + 0xfc);
            e.vcall(n, NODE_SLOT_13C, &args![alpha]);
        }
        e.vcall(n, NODE_SLOT_130, &args![shown]);
        let level = x87_load32(level);
        if level <= e.global::<f64>(ZERO) {
            e.vcall(n, NODE_SLOT_128, &args![0u32]);
            e.vcall(n, NODE_SLOT_12C, &args![0u32]);
            e.call(GEOMETRY_DESTRUCTOR, &args![record]);
            return;
        }
        shown = 0;
        let count = bs_tree_model_get_num_branch_lod_levels(e, this) as i16;
        if count > 0 {
            let tree = e.get(this, BSTreeModel::pSpeedTree);
            let current = x87_store32(e.call(SPEED_TREE_GET_LOD_LEVEL, &args![tree]).f64());
            let tree = e.get(this, BSTreeModel::pSpeedTree);
            let discrete = e
                .call(SPEED_TREE_DISCRETE_BRANCH_LOD, &args![tree, current])
                .u16() as i16;
            let mut index: i16 = 0;
            while index < count {
                let i = u32::from(index as u16);
                if discrete == index {
                    shown = 1;
                    let alpha = chopped_byte(e, record + 0x38);
                    e.vcall(n, NODE_SLOT_134, &args![i, alpha]);
                } else {
                    e.vcall(n, NODE_SLOT_134, &args![i, 0xffu32]);
                }
                index = index.wrapping_add(1);
            }
        }
        e.vcall(n, NODE_SLOT_128, &args![shown]);
        if !e.call(ARE_LEAVES_VISIBLE, &args![]).bool() {
            e.vcall(n, NODE_SLOT_12C, &args![0u32]);
            e.call(GEOMETRY_DESTRUCTOR, &args![record]);
            return;
        }
        shown = 0;
        let count = fn_006699d0(e, this) as i16;
        if count > 0 {
            let first = e.mem.u8(record + 0x78) != 0;
            let second = e.mem.u8(record + 0xb4) != 0;
            if first || second {
                let mut index: i16 = 0;
                while index < count {
                    let i = u32::from(index as u16);
                    if e.mem.u8(record + 0x78) != 0 && e.mem.i32(record + 0x80) == i32::from(index)
                    {
                        shown = 1;
                        let alpha = chopped_byte(e, record + 0x7c);
                        e.vcall(n, NODE_SLOT_138, &args![i, alpha]);
                    } else if e.mem.u8(record + 0xb4) != 0
                        && e.mem.i32(record + 0xbc) == i32::from(index)
                    {
                        shown = 1;
                        let alpha = chopped_byte(e, record + 0xb8);
                        e.vcall(n, NODE_SLOT_138, &args![i, alpha]);
                    } else {
                        e.vcall(n, NODE_SLOT_138, &args![i, 0xffu32]);
                    }
                    index = index.wrapping_add(1);
                }
            }
            if pointer(e, t + 0x20) != 0 {
                let tree = e.get(this, BSTreeModel::pSpeedTree);
                let angle_data = if e.call(SPEED_TREE_WRAPPER, &args![tree]).u32() != 0 {
                    let tree = e.get(this, BSTreeModel::pSpeedTree);
                    let wrapper = e.call(SPEED_TREE_WRAPPER, &args![tree]).u32();
                    e.call(SPEED_TREE_LEAF_ANGLES, &args![wrapper, angles])
                        .u32()
                } else {
                    let tree = e.get(this, BSTreeModel::pSpeedTree);
                    e.call(SPEED_TREE_LEAF_ANGLES, &args![tree, angles]).u32()
                };
                let count = e.mem.u32(angles);
                let data = pointer(e, t + 0x20);
                e.call(LEAF_SHADER_DATA_UPDATE, &args![data, angle_data, count]);
            }
        }
        e.vcall(n, NODE_SLOT_12C, &args![shown]);
        e.call(GEOMETRY_DESTRUCTOR, &args![record]);
    });
}

/// `CreateArt`'s calls: `BSTreeNode::BSTreeNode` (`0066b120`, 0xF8 bytes:
/// model, body), `BSShaderManager::GetShader` (`00b55560`, cdecl),
/// `NiTriStrips::NiTriStrips` (`00a71cc0`) and `NiTriShape::NiTriShape`
/// (`00a74480`) (0xC4 bytes, with the data), `NiAVObject::AttachProperty`
/// (`00439410`), `BSShaderProperty::FreeRenderPasses` (`00baa000`), the
/// shape's `005495f0` and `004a8a90`/`005f36f0` on its answer, the
/// shader property's slot `+0xD4` (with the shape) into `00539ef0`, the
/// shader into the shape (`00874460`), `BSShaderPPLightingProperty::
/// LoadTextureSet` (`00b66ad0`), `BSShaderManager::PrepareObject`
/// (`00b57e30`, cdecl), the model loader (`0043c4b0`) and its slot
/// `+0xEC`, `NiAVObject::UpdateProperties` (`00a5a040`), the update
/// record (`0043d410`: 0.0, 0, 0) and update (`00a59c90`), and
/// `BSTreeNode::MakeHavokLeaves` (`0066cb40`) (Xbox PDB names where
/// named).
const TREE_NODE_CONSTRUCTOR: u32 = 0x0066_b120;
const GET_SHADER: u32 = 0x00b5_5560;
const TRI_STRIPS_CONSTRUCTOR: u32 = 0x00a7_1cc0;
const TRI_SHAPE_CONSTRUCTOR: u32 = 0x00a7_4480;
const ATTACH_PROPERTY: u32 = 0x0043_9410;
const FREE_RENDER_PASSES: u32 = 0x00ba_a000;
const SHAPE_005495F0: u32 = 0x0054_95f0;
const TEST_004A8A90: u32 = 0x004a_8a90;
const TEST_005F36F0: u32 = 0x005f_36f0;
const PROPERTY_SLOT_D4: u32 = 0xd4;
const SHAPE_00539EF0: u32 = 0x0053_9ef0;
const SET_SHADER: u32 = 0x0087_4460;
const LOAD_TEXTURE_SET: u32 = 0x00b6_6ad0;
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
const MODEL_LOADER: u32 = 0x0043_c4b0;
const MODEL_LOADER_SLOT_EC: u32 = 0xec;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const UPDATE_RECORD_CONSTRUCTOR: u32 = 0x0043_d410;
const NODE_UPDATE: u32 = 0x00a5_9c90;
const MAKE_HAVOK_LEAVES: u32 = 0x0066_cb40;
/// `"STBranch"` (`01066830`).
const BRANCH_TEXTURE_NAME: u32 = 0x0106_6830;
/// The tree node's virtual slots `CreateArt` uses (the engine map names
/// none): `+0x104`, `+0x10C` (answers not used), `+0x110` (branch shape),
/// `+0x11C` (set branch), `+0x120` (set leaf), `+0x124` (set billboard).
const NODE_SLOT_104: u32 = 0x104;
const NODE_SLOT_10C: u32 = 0x10c;
const NODE_SLOT_110: u32 = 0x110;
const NODE_SLOT_11C: u32 = 0x11c;
const NODE_SLOT_120: u32 = 0x120;
const NODE_SLOT_124: u32 = 0x124;

/// Queues `shape` with the model loader (its slot `+0xEC`: shape, 0, 0, 0).
fn queue_shape(e: &mut Engine, shape: u32) {
    let loader = e.call(MODEL_LOADER, &args![]).u32();
    e.vcall(
        loader,
        MODEL_LOADER_SLOT_EC,
        &args![shape, 0u32, 0u32, 0u32],
    );
}

// Translated from 00669fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeModel::CreateArt` (Xbox PDB): a new `BSTreeNode` for the model at
/// `scale` (null without a SpeedTree), inside the allocation scope: its
/// body is the model's rigid body at scale 1, a new tree body at other
/// scales, none for a trunk of 64 or lower. Each branch level with data
/// gets a strips shape with a new alpha property (its `004393c0` 0) and
/// the level's shader property; each leaf level with data a shape with a
/// new alpha property, its shader property, shader 6 and the model
/// loader's queue. Then the billboard, the properties' update, an update
/// and the Havok leaves.
// Translation incomplete: 00669fd0 - the compiler's exception frame (FS:[0] chain) is not translated
pub fn bs_tree_model_create_art(e: &mut Engine, this: Ptr<BSTreeModel>, scale: f32) -> Ptr {
    let t = this.addr();
    let scale_bits = scale.to_bits();
    // Stack: +0 the scope, +4 the body pointer, +8 the update record (0x10).
    e.with_stack(0x18, |e, locals| {
        let scope = locals.addr();
        let body = scope + 4;
        let update = scope + 8;
        e.call(
            SCOPE_OPEN,
            &args![scope, 0x1eu32, 1u32, SOURCE_FILE, 0x385u32],
        );
        e.call(NI_POINTER_CONSTRUCT, &args![body, 0u32]);
        if e.get(this, BSTreeModel::pSpeedTree) == 0 {
            e.call(NI_POINTER_DESTRUCT, &args![body]);
            e.call(SCOPE_CLOSE, &args![scope]);
            return Ptr::new(0);
        }
        if x87_load32(e.get(this, BSTreeModel::fTrunkHeight))
            > e.global::<f64>(MINIMUM_TRUNK_HEIGHT)
        {
            if x87_load32(scale_bits) == e.global::<f64>(ONE) {
                let rigid = fn_0066a630(e, this);
                e.call(NI_POINTER_ASSIGN, &args![body, rigid]);
            } else {
                let width = e.get(this, BSTreeModel::fTrunkWidth);
                let radius = x87_store32(x87_div(
                    x87_mul(x87_load32(scale_bits), x87_m32(width)),
                    e.global::<f64>(TWO),
                ));
                let height = e.get(this, BSTreeModel::fTrunkHeight);
                let height = x87_store32(x87_mul(x87_load32(scale_bits), x87_m32(height)));
                let created = e
                    .call(
                        CREATE_TREE_BODY,
                        &args![x87_quiet_bits(height), x87_quiet_bits(radius)],
                    )
                    .u32();
                e.call(NI_POINTER_ASSIGN, &args![body, created]);
            }
        } else {
            e.call(NI_POINTER_ASSIGN, &args![body, 0u32]);
        }
        let block = e.call(NI_NEW, &args![0xf8u32]).u32();
        let node = if block == 0 {
            0
        } else {
            let rigid = pointer(e, body);
            e.call(TREE_NODE_CONSTRUCTOR, &args![block, this, rigid])
                .u32()
        };
        if e.get(this, BSTreeModel::pspBranchData) != 0
            && e.get(this, BSTreeModel::pspBranchShaderProperties) != 0
        {
            let count = bs_tree_model_get_num_branch_lod_levels(e, this);
            let shader = e.call(GET_SHADER, &args![4u32]).u32();
            e.vcall(node, NODE_SLOT_104, &args![]);
            let mut index: u16 = 0;
            while u32::from(index) < u32::from(count) {
                let i = u32::from(index) * 4;
                let slot = e.get(this, BSTreeModel::pspBranchData) + i;
                if pointer(e, slot) != 0 {
                    let block = e.call(NI_NEW, &args![0xc4u32]).u32();
                    let shape = if block == 0 {
                        0
                    } else {
                        let slot = e.get(this, BSTreeModel::pspBranchData) + i;
                        let data = pointer(e, slot);
                        e.call(TRI_STRIPS_CONSTRUCTOR, &args![block, data]).u32()
                    };
                    let alpha = bs_tree_model_create_alpha_property(e).addr();
                    if alpha != 0 {
                        e.call(ALPHA_SET_004393C0, &args![alpha, 0u32]);
                        e.call(ATTACH_PROPERTY, &args![shape, alpha]);
                    }
                    let properties = e.get(this, BSTreeModel::pspBranchShaderProperties);
                    if pointer(e, properties + i) != 0 {
                        let property = pointer(e, properties + i);
                        e.call(FREE_RENDER_PASSES, &args![property]);
                        let property = pointer(e, properties + i);
                        e.call(ATTACH_PROPERTY, &args![shape, property]);
                        let answer = e.call(SHAPE_005495F0, &args![shape]).u32();
                        if e.call(TEST_004A8A90, &args![answer]).u32() == 0 {
                            let property = pointer(e, properties + i);
                            let value = e.vcall(property, PROPERTY_SLOT_D4, &args![shape]).u32();
                            e.call(SHAPE_00539EF0, &args![shape, value]);
                        }
                    }
                    if bs_tree_model_fn_00666f00(e, this) {
                        e.call(SET_SHADER, &args![shape, shader]);
                    }
                    e.vcall(node, NODE_SLOT_11C, &args![u32::from(index), shape]);
                }
                index = index.wrapping_add(1);
            }
            if !bs_tree_model_fn_00666f00(e, this) {
                let mut index: u16 = 0;
                while u32::from(index) < u32::from(count) {
                    let i = u32::from(index) * 4;
                    let shape = e.vcall(node, NODE_SLOT_110, &args![u32::from(index)]).u32();
                    if shape != 0 {
                        let textures = pointer(e, t + 0x2c);
                        let properties = e.get(this, BSTreeModel::pspBranchShaderProperties);
                        let property = pointer(e, properties + i);
                        e.call(
                            LOAD_TEXTURE_SET,
                            &args![property, BRANCH_TEXTURE_NAME, textures],
                        );
                        e.call(PREPARE_OBJECT, &args![shape, 0u32, 0u32]);
                        let property = pointer(e, properties + i);
                        let value = e.vcall(property, PROPERTY_SLOT_D4, &args![shape]).u32();
                        e.call(SHAPE_00539EF0, &args![shape, value]);
                        let answer = e.call(SHAPE_005495F0, &args![shape]).u32();
                        if e.call(TEST_005F36F0, &args![answer]).u32() == 0 {
                            queue_shape(e, shape);
                        }
                    }
                    index = index.wrapping_add(1);
                }
            }
        }
        if e.get(this, BSTreeModel::pspLeafData) != 0
            && e.get(this, BSTreeModel::pspLeafShaderProperties) != 0
        {
            let count = fn_006699d0(e, this);
            let shader = e.call(GET_SHADER, &args![6u32]).u32();
            let mut index: u16 = 0;
            while u32::from(index) < u32::from(count) {
                let i = u32::from(index) * 4;
                let slot = e.get(this, BSTreeModel::pspLeafData) + i;
                if pointer(e, slot) != 0 {
                    let block = e.call(NI_NEW, &args![0xc4u32]).u32();
                    let shape = if block == 0 {
                        0
                    } else {
                        let slot = e.get(this, BSTreeModel::pspLeafData) + i;
                        let data = pointer(e, slot);
                        e.call(TRI_SHAPE_CONSTRUCTOR, &args![block, data]).u32()
                    };
                    let alpha = bs_tree_model_create_alpha_property(e).addr();
                    if alpha != 0 {
                        e.call(ATTACH_PROPERTY, &args![shape, alpha]);
                    }
                    let properties = e.get(this, BSTreeModel::pspLeafShaderProperties);
                    if pointer(e, properties + i) != 0 {
                        let property = pointer(e, properties + i);
                        e.call(FREE_RENDER_PASSES, &args![property]);
                        let property = pointer(e, properties + i);
                        e.call(ATTACH_PROPERTY, &args![shape, property]);
                    }
                    e.vcall(node, NODE_SLOT_120, &args![u32::from(index), shape]);
                    e.call(SET_SHADER, &args![shape, shader]);
                    queue_shape(e, shape);
                }
                index = index.wrapping_add(1);
            }
        }
        e.vcall(node, NODE_SLOT_10C, &args![]);
        if pointer(e, t + 0x1c) != 0 {
            let billboard = pointer(e, t + 0x1c);
            e.vcall(node, NODE_SLOT_124, &args![billboard]);
        }
        e.call(UPDATE_PROPERTIES, &args![node]);
        e.call(
            UPDATE_RECORD_CONSTRUCTOR,
            &args![update, 0.0f32.to_bits(), 0u32, 0u32],
        );
        e.call(NODE_UPDATE, &args![node, update]);
        let tree = e.get(this, BSTreeModel::pSpeedTree);
        e.call(
            MAKE_HAVOK_LEAVES,
            &args![node, tree, x87_quiet_bits(scale_bits)],
        );
        e.call(NI_POINTER_DESTRUCT, &args![body]);
        e.call(SCOPE_CLOSE, &args![scope]);
        Ptr::new(node)
    })
}

/// The geometry code's calls (no PDB names unless given): the SpeedTree's
/// geometry into a record (`00b0ab20`: record, kind, then three level
/// words), the block allocation (`00aa1070`, cdecl), the copy (`00401460`:
/// target, source, bytes) and fill (`00403d30`: target, byte, count), the
/// index set (`00474790`: 0, 1 constructs it, `0096a610` adds the word at
/// its argument, `00658930` its size, `00877a30` the address of an entry,
/// `00473f00` destroys it), the array constructor (`00401050`: array,
/// element size, count, constructor `006815c0`), the point (`00416870`:
/// x, y, z), texture coordinate (`00452dc0`: u, v) and colour (`00414430`:
/// four floats) constructors, `_isnan` (`00ec75b1`, a `double`), the
/// colour holder (`00bb33c0`, 0x10 bytes), the strips data
/// (`00a75dc0`, 0x50 bytes), `NiGeometryData::SetConsistency` (`00a67050`,
/// Xbox PDB), the data's `004410d0` and `004410f0`, the branch shader
/// property (`00bb3070`, 0x108 bytes, with the colour holder) and the
/// SpeedTree's `00b08750` and `00b08780` (after which the record's colours
/// and the two words at +0x30, +0x34 are cleared).
const SPEED_TREE_GEOMETRY: u32 = 0x00b0_ab20;
const BLOCK_ALLOCATE: u32 = 0x00aa_1070;
const MEMORY_COPY: u32 = 0x0040_1460;
const MEMORY_FILL: u32 = 0x0040_3d30;
const INDEX_SET_CONSTRUCTOR: u32 = 0x0047_4790;
const INDEX_SET_ADD: u32 = 0x0096_a610;
const INDEX_SET_SIZE: u32 = 0x0065_8930;
const INDEX_SET_AT: u32 = 0x0087_7a30;
const INDEX_SET_DESTRUCTOR: u32 = 0x0047_3f00;
const ARRAY_CONSTRUCTOR: u32 = 0x0040_1050;
const POINT3_CONSTRUCTOR: u32 = 0x0041_6870;
const POINT2_CONSTRUCTOR: u32 = 0x0045_2dc0;
const COLOR_CONSTRUCTOR: u32 = 0x0041_4430;
const IS_NAN: u32 = 0x00ec_75b1;
const COLOR_HOLDER_CONSTRUCTOR: u32 = 0x00bb_33c0;
const STRIPS_DATA_CONSTRUCTOR: u32 = 0x00a7_5dc0;
const SET_CONSISTENCY: u32 = 0x00a6_7050;
const DATA_004410D0: u32 = 0x0044_10d0;
const DATA_004410F0: u32 = 0x0044_10f0;
const BRANCH_PROPERTY_CONSTRUCTOR: u32 = 0x00bb_3070;
const SPEED_TREE_00B08750: u32 = 0x00b0_8750;
const SPEED_TREE_00B08780: u32 = 0x00b0_8780;
/// The normal a NaN normal is replaced with (`011a9484`, three floats).
const DEFAULT_NORMAL: u32 = 0x011a_9484;
/// `255.0` (`0101e568`, `double`).
const BYTE_SCALE: u32 = 0x0101_e568;

/// `count * size`, all ones when it overflows (the code's `operator new`
/// size).
fn array_size(count: u32, size: u32) -> u32 {
    let (product, overflow) = count.overflowing_mul(size);
    if overflow {
        u32::MAX
    } else {
        product
    }
}

/// A new array of `count` elements of `size` bytes, constructed by
/// `006815c0` (null when the allocation fails).
fn new_point_array(e: &mut Engine, count: u32, size: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![array_size(count, size)]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        ARRAY_CONSTRUCTOR,
        &args![block, size, count, POINT_CONSTRUCTOR],
    );
    block
}

/// Builds the point at `floats[index ..= index + 2]` through the point
/// constructor in `scratch` and copies it to `target`.
fn copy_point3(e: &mut Engine, scratch: u32, floats: u32, index: u32, target: u32) {
    let word = |e: &Engine, k: u32| {
        x87_quiet_bits(
            e.mem
                .u32(floats.wrapping_add(index.wrapping_add(k).wrapping_mul(4))),
        )
    };
    let (x, y, z) = (word(e, 0), word(e, 1), word(e, 2));
    let point = e.call(POINT3_CONSTRUCTOR, &args![scratch, x, y, z]).u32();
    for i in 0..3 {
        let value = e.mem.u32(point + i * 4);
        e.mem.set_u32(target.wrapping_add(i * 4), value);
    }
}

/// Whether `_isnan` answers non-zero for the float at `address` (widened
/// to a `double`).
fn float_is_nan(e: &mut Engine, address: u32) -> bool {
    let value = x87_load32(e.mem.u32(address));
    e.call(IS_NAN, &args![value]).u32() != 0
}

// Translated from 00667470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The branch geometry (no PDB name; `CreateGeometry` runs it): with a
/// SpeedTree of the first kind, deletes the branch data and shader
/// property arrays and, when the tree has branch levels and its branch
/// geometry has everything, makes new arrays of null pointers for them.
/// With tangents, NaN normals become the default normal. For each level
/// with vertices and strips: the first strip's indices are remapped to
/// the vertices it uses (in index-set order), and those vertices'
/// positions, normals (and tangents), texture coordinates and colours
/// (the float at +0x30, four times the byte at +0x34, the colour word's
/// second byte / 255, and 0) become a strips data with a shader property
/// for the level.
// Translation incomplete: 00667470 - the compiler's exception frame (FS:[0] chain) is not translated
pub fn bs_tree_model_fn_00667470(e: &mut Engine, this: Ptr<BSTreeModel>) {
    let t = this.addr();
    // Stack: +0 the record (0x134), +0x134 the index set (0x20), +0x154
    // the colour holder pointer, +0x158 the point scratch (0xC), +0x164
    // the texture coordinate scratch (8), +0x16C the colour scratch
    // (0x10), +0x17C the word added to the set.
    e.with_stack(0x180, |e, locals| {
        let record = locals.addr();
        let set = record + 0x134;
        let holder = record + 0x154;
        let point = record + 0x158;
        let uv = record + 0x164;
        let color = record + 0x16c;
        let value = record + 0x17c;
        let r = |offset: u32| record + offset;
        e.call(GEOMETRY_CONSTRUCTOR, &args![record]);
        if e.get(this, BSTreeModel::pSpeedTree) == 0 || bs_tree_model_fn_00666f00(e, this) {
            e.call(GEOMETRY_DESTRUCTOR, &args![record]);
            return;
        }
        delete_pointer_array(e, t + 0x14);
        delete_pointer_array(e, t + 0x24);
        let levels = bs_tree_model_get_num_branch_lod_levels(e, this);
        if levels == 0 {
            e.call(GEOMETRY_DESTRUCTOR, &args![record]);
            return;
        }
        let tree = e.get(this, BSTreeModel::pSpeedTree);
        e.call(
            SPEED_TREE_GEOMETRY,
            &args![tree, record, 1u32, u32::MAX, u32::MAX, u32::MAX],
        );
        if [0x24u32, 0x18, 0x28, 0x14, 0x30, 0x34]
            .iter()
            .any(|&offset| e.mem.u32(r(offset)) == 0)
        {
            e.call(GEOMETRY_DESTRUCTOR, &args![record]);
            return;
        }
        let vertices = e.mem.u16(r(0x10));
        if vertices == 0 {
            e.call(GEOMETRY_DESTRUCTOR, &args![record]);
            return;
        }
        let data = new_pointer_array(e, u32::from(levels));
        e.set(this, BSTreeModel::pspBranchData, data);
        let properties = new_pointer_array(e, u32::from(levels));
        e.set(this, BSTreeModel::pspBranchShaderProperties, properties);
        let mut level: u16 = 0;
        while level < levels {
            let i = u32::from(level) * 4;
            let slot = e.get(this, BSTreeModel::pspBranchData) + i;
            e.call(NI_POINTER_ASSIGN, &args![slot, 0u32]);
            let slot = e.get(this, BSTreeModel::pspBranchShaderProperties) + i;
            e.call(NI_POINTER_ASSIGN, &args![slot, 0u32]);
            level = level.wrapping_add(1);
        }
        if e.mem.u32(r(0x20)) != 0 && e.mem.u32(r(0x1c)) != 0 {
            let normals = e.mem.u32(r(0x18));
            let mut index: i32 = 0;
            while index < i32::from(vertices) {
                let entry = normals.wrapping_add((index as u32).wrapping_mul(0xc));
                if float_is_nan(e, entry)
                    || float_is_nan(e, entry + 4)
                    || float_is_nan(e, entry + 8)
                {
                    for k in 0..3 {
                        let word = e.mem.u32(DEFAULT_NORMAL + k * 4);
                        e.mem.set_u32(entry + k * 4, word);
                    }
                }
                index += 1;
            }
        }
        let mut level: u16 = 0;
        while level < levels {
            let tree = e.get(this, BSTreeModel::pSpeedTree);
            e.call(
                SPEED_TREE_GEOMETRY,
                &args![tree, record, 1u32, u32::from(level), u32::MAX, u32::MAX],
            );
            if e.mem.u16(r(0x10)) == 0 || e.mem.u32(r(8)) == 0 || e.mem.u32(r(0xc)) == 0 {
                level = level.wrapping_add(1);
                continue;
            }
            let length = e.call(BLOCK_ALLOCATE, &args![2u32]).u32();
            let first_length = e.mem.u16(e.mem.u32(r(8)));
            e.mem.set_u16(length, first_length);
            let bytes = u32::from(e.mem.u16(length)) << 1;
            let indices = e.call(BLOCK_ALLOCATE, &args![bytes]).u32();
            let bytes = u32::from(e.mem.u16(length)) << 1;
            let strip = e.mem.u32(e.mem.u32(r(0xc)));
            e.call(MEMORY_COPY, &args![indices, strip, bytes]);
            let vertex_count = u32::from(e.mem.u16(r(0x10)));
            let used = e.call(OPERATOR_NEW, &args![vertex_count]).u32();
            let vertex_count = u32::from(e.mem.u16(r(0x10)));
            e.call(MEMORY_FILL, &args![used, 0u32, vertex_count]);
            let mut k: u32 = 0;
            while k < u32::from(e.mem.u16(length)) {
                let vertex = u32::from(e.mem.u16(indices + k * 2));
                e.mem.set_u8(used.wrapping_add(vertex), 1);
                k += 1;
            }
            e.call(INDEX_SET_CONSTRUCTOR, &args![set, 0u32, 1u32]);
            let mut vertex: u16 = 0;
            while u32::from(vertex) < u32::from(e.mem.u16(r(0x10))) {
                if e.mem.u8(used + u32::from(vertex)) != 0 {
                    e.mem.set_u32(value, u32::from(vertex));
                    e.call(INDEX_SET_ADD, &args![set, value]);
                }
                vertex = vertex.wrapping_add(1);
            }
            e.call(OPERATOR_DELETE, &args![used]);
            let mut entry: u16 = 0;
            while u32::from(entry) < e.call(INDEX_SET_SIZE, &args![set]).u32() {
                let at = e.call(INDEX_SET_AT, &args![set, u32::from(entry)]).u32();
                let original = e.mem.u32(at);
                let mut k: u32 = 0;
                while k < u32::from(e.mem.u16(length)) {
                    let strip = e.mem.u32(e.mem.u32(r(0xc)));
                    if u32::from(e.mem.u16(strip.wrapping_add(k * 2))) == original {
                        e.mem.set_u16(indices + k * 2, entry);
                    }
                    k += 1;
                }
                entry = entry.wrapping_add(1);
            }
            let count = e.call(INDEX_SET_SIZE, &args![set]).u32();
            let positions = new_point_array(e, count, 0xc);
            let count = e.call(INDEX_SET_SIZE, &args![set]).u32();
            let coordinates = new_point_array(e, count, 8);
            let (normals, tangents, binormals, with_tangents) =
                if e.mem.u32(r(0x20)) != 0 && e.mem.u32(r(0x1c)) != 0 {
                    let count = e.call(INDEX_SET_SIZE, &args![set]).u32().wrapping_mul(3);
                    let normals = new_point_array(e, count, 0xc);
                    let count = e.call(INDEX_SET_SIZE, &args![set]).u32();
                    let tangents = normals.wrapping_add(count.wrapping_mul(0xc));
                    let count = e.call(INDEX_SET_SIZE, &args![set]).u32();
                    let binormals = normals.wrapping_add((count << 1).wrapping_mul(0xc));
                    (normals, tangents, binormals, true)
                } else {
                    let count = e.call(INDEX_SET_SIZE, &args![set]).u32();
                    (new_point_array(e, count, 0xc), 0, 0, false)
                };
            let mut entry: u16 = 0;
            while u32::from(entry) < e.call(INDEX_SET_SIZE, &args![set]).u32() {
                let at = e.call(INDEX_SET_AT, &args![set, u32::from(entry)]).u32();
                let original = e.mem.u32(at);
                let three = original.wrapping_mul(3);
                let two = original.wrapping_mul(2);
                let k = u32::from(entry);
                let source = e.mem.u32(r(0x24));
                copy_point3(e, point, source, three, positions.wrapping_add(k * 0xc));
                let source = e.mem.u32(r(0x18));
                copy_point3(e, point, source, three, normals.wrapping_add(k * 0xc));
                let source = e.mem.u32(r(0x28));
                let u = x87_quiet_bits(e.mem.u32(source.wrapping_add(two.wrapping_mul(4))));
                let v = x87_quiet_bits(
                    e.mem
                        .u32(source.wrapping_add(two.wrapping_add(1).wrapping_mul(4))),
                );
                let made = e.call(POINT2_CONSTRUCTOR, &args![uv, u, v]).u32();
                let (first, second) = (e.mem.u32(made), e.mem.u32(made + 4));
                e.mem.set_u32(coordinates.wrapping_add(k * 8), first);
                e.mem.set_u32(coordinates.wrapping_add(k * 8 + 4), second);
                if with_tangents {
                    let source = e.mem.u32(r(0x20));
                    copy_point3(e, point, source, three, binormals.wrapping_add(k * 0xc));
                    let source = e.mem.u32(r(0x1c));
                    copy_point3(e, point, source, three, tangents.wrapping_add(k * 0xc));
                }
                entry = entry.wrapping_add(1);
            }
            let count = e.call(INDEX_SET_SIZE, &args![set]).u32();
            let colors = new_point_array(e, count, 0x10);
            let mut entry: u16 = 0;
            while u32::from(entry) < e.call(INDEX_SET_SIZE, &args![set]).u32() {
                let at = e.call(INDEX_SET_AT, &args![set, u32::from(entry)]).u32();
                let original = e.mem.u32(at);
                let word = e
                    .mem
                    .u32(e.mem.u32(r(0x14)).wrapping_add(original.wrapping_mul(4)));
                let third = x87_store32(x87_div(
                    f64::from((word & 0xff00) >> 8),
                    e.global::<f64>(BYTE_SCALE),
                ));
                let byte = e.mem.u8(e.mem.u32(r(0x34)).wrapping_add(original));
                let second = x87_store32(f64::from(u32::from(byte) << 2));
                let first = x87_quiet_bits(
                    e.mem
                        .u32(e.mem.u32(r(0x30)).wrapping_add(original.wrapping_mul(4))),
                );
                let made = e
                    .call(
                        COLOR_CONSTRUCTOR,
                        &args![
                            color,
                            first,
                            second,
                            x87_quiet_bits(third),
                            0.0f32.to_bits()
                        ],
                    )
                    .u32();
                let target = colors.wrapping_add(u32::from(entry) << 4);
                for w in 0..4 {
                    let word = e.mem.u32(made + w * 4);
                    e.mem.set_u32(target.wrapping_add(w * 4), word);
                }
                entry = entry.wrapping_add(1);
            }
            let block = e.call(NI_NEW, &args![0x10u32]).u32();
            let made = if block == 0 {
                0
            } else {
                e.call(COLOR_HOLDER_CONSTRUCTOR, &args![block]).u32()
            };
            e.call(NI_POINTER_CONSTRUCT, &args![holder, made]);
            let count = e.call(INDEX_SET_SIZE, &args![set]).u32();
            let held = pointer(e, holder);
            fn_006684d0(e, Ptr::new(held), colors, count as u16);
            let block = e.call(NI_NEW, &args![0x50u32]).u32();
            let strips = if block == 0 {
                0
            } else {
                let triangles = u32::from(e.mem.u16(e.mem.u32(r(8)))).wrapping_sub(2);
                let flags = if with_tangents { 0x1000u32 } else { 0 };
                let count = e.call(INDEX_SET_SIZE, &args![set]).u32();
                e.call(
                    STRIPS_DATA_CONSTRUCTOR,
                    &args![
                        block,
                        count,
                        positions,
                        normals,
                        0u32,
                        coordinates,
                        1u32,
                        flags,
                        triangles,
                        1u32,
                        length,
                        indices
                    ],
                )
                .u32()
            };
            let i = u32::from(level) * 4;
            let slot = e.get(this, BSTreeModel::pspBranchData) + i;
            e.call(NI_POINTER_ASSIGN, &args![slot, strips]);
            let slot = e.get(this, BSTreeModel::pspBranchData) + i;
            let made = pointer(e, slot);
            e.call(SET_CONSISTENCY, &args![made, 0x4000u32]);
            let slot = e.get(this, BSTreeModel::pspBranchData) + i;
            let made = pointer(e, slot);
            e.call(DATA_004410D0, &args![made, 0x11u32]);
            let slot = e.get(this, BSTreeModel::pspBranchData) + i;
            let made = pointer(e, slot);
            e.call(DATA_004410F0, &args![made, 1u32]);
            let block = e.call(NI_NEW, &args![0x108u32]).u32();
            let property = if block == 0 {
                0
            } else {
                let held = pointer(e, holder);
                e.call(BRANCH_PROPERTY_CONSTRUCTOR, &args![block, held])
                    .u32()
            };
            let slot = e.get(this, BSTreeModel::pspBranchShaderProperties) + i;
            e.call(NI_POINTER_ASSIGN, &args![slot, property]);
            e.call(NI_POINTER_DESTRUCT, &args![holder]);
            e.call(INDEX_SET_DESTRUCTOR, &args![set]);
            level = level.wrapping_add(1);
        }
        let tree = e.get(this, BSTreeModel::pSpeedTree);
        e.call(SPEED_TREE_00B08750, &args![tree]);
        e.mem.set_u32(r(0x14), 0);
        let tree = e.get(this, BSTreeModel::pSpeedTree);
        e.call(SPEED_TREE_00B08780, &args![tree]);
        e.mem.set_u32(r(0x30), 0);
        e.mem.set_u32(r(0x34), 0);
        e.call(GEOMETRY_DESTRUCTOR, &args![record]);
    });
}

/// The leaf geometry's calls (no PDB names unless given): the SpeedTree's
/// `00b0cb10` (a pointer: its floats indexed by level), `0044edb0` (a
/// word; the engine map's name for it comes from identical-code folding)
/// and `00b088b0`, the leaf map names' comparison (`00408b20`, cdecl), the
/// leaf shader data (`00bb3950`, 0x20 bytes) and its setters (`0063f790`,
/// `00621b20`, `006694c0`, `00819a50`, `00804cb0`; one float each), the
/// floats `00598040` and `00792760` read from the SpeedTree's word, the
/// form's float slots `+0x1C4` and `+0x1CC`, `NiTriShapeData::
/// NiTriShapeData` (`00a7b630`, Xbox PDB; 0x58 bytes), the leaf shader
/// property (`00bb3ba0`, 0x90 bytes: level, colour holder, leaf shader
/// data) with its slots `+0xDC` (the leaf frond texture) and `+0xF4` (0),
/// whose answer goes to the data's `00a67260`.
const SPEED_TREE_LEAF_FLOATS: u32 = 0x00b0_cb10;
const SPEED_TREE_0044EDB0: u32 = 0x0044_edb0;
const SPEED_TREE_00B088B0: u32 = 0x00b0_88b0;
const NAMES_DIFFER: u32 = 0x0040_8b20;
const LEAF_SHADER_DATA_CONSTRUCTOR: u32 = 0x00bb_3950;
const LEAF_DATA_SET_0063F790: u32 = 0x0063_f790;
const LEAF_DATA_SET_00621B20: u32 = 0x0062_1b20;
const LEAF_DATA_SET_00819A50: u32 = 0x0081_9a50;
const LEAF_DATA_SET_00804CB0: u32 = 0x0080_4cb0;
const VALUE_00598040: u32 = 0x0059_8040;
const VALUE_00792760: u32 = 0x0079_2760;
const TREE_SLOT_1C4: u32 = 0x1c4;
const TREE_SLOT_1CC: u32 = 0x1cc;
const SHAPE_DATA_CONSTRUCTOR: u32 = 0x00a7_b630;
const LEAF_PROPERTY_CONSTRUCTOR: u32 = 0x00bb_3ba0;
const LEAF_PROPERTY_SLOT_DC: u32 = 0xdc;
const LEAF_PROPERTY_SLOT_F4: u32 = 0xf4;
const DATA_00A67260: u32 = 0x00a6_7260;
/// The shade a leaf's at least 1.0 shade becomes (`0106682c`, `float`).
const LEAF_SHADE_LIMIT: u32 = 0x0106_682c;

/// Copies the three words at `source` to `target`.
fn copy_words3(e: &mut Engine, source: u32, target: u32) {
    for i in 0..3 {
        let word = e.mem.u32(source.wrapping_add(i * 4));
        e.mem.set_u32(target.wrapping_add(i * 4), word);
    }
}

// Translated from 00668500 (decompiled, FalloutNV.exe 1.4.0.525)
/// The leaf geometry (no PDB name; `CreateGeometry` runs it with the
/// form): with a SpeedTree whose `00b0cb10` answers, of the first kind,
/// deletes the leaf data and shader property arrays; with leaf levels and
/// leaf maps makes new arrays of null pointers and a new leaf shader data
/// (the curve scalar, and with the SpeedTree's `0044edb0` word two values
/// from it and, with a form, two from the form). Each level's leaves
/// become four corners each (the leaf's position and normal, the corner's
/// texture coordinates, mirrored when `00408b20` answered non-zero for
/// any two leaf map names, and a colour) and two triangles in a shape
/// data with a leaf shader property.
// Translation incomplete: 00668500 - the compiler's exception frame (FS:[0] chain) is not translated
pub fn bs_tree_model_fn_00668500(e: &mut Engine, this: Ptr<BSTreeModel>, form: Ptr) {
    let t = this.addr();
    // Stack: +0 the record (0x134), +0x134 the name holder (0x1C), +0x150
    // the colour holder pointer, +0x154 the texture coordinate scratch
    // (8), +0x15C the colour scratch (0x10).
    e.with_stack(0x16c, |e, locals| {
        let record = locals.addr();
        let names = record + 0x134;
        let holder = record + 0x150;
        let uv = record + 0x154;
        let color = record + 0x15c;
        let r = |offset: u32| record + offset;
        e.call(GEOMETRY_CONSTRUCTOR, &args![record]);
        e.call(NAME_HOLDER_CONSTRUCTOR, &args![names]);
        e.call(NI_POINTER_CONSTRUCT, &args![holder, 0u32]);
        let finish = |e: &mut Engine| {
            e.call(NI_POINTER_DESTRUCT, &args![holder]);
            e.call(NAME_HOLDER_DESTRUCTOR, &args![names]);
            e.call(GEOMETRY_DESTRUCTOR, &args![record]);
        };
        let mut mirrored = false;
        let tree = e.get(this, BSTreeModel::pSpeedTree);
        if tree == 0
            || e.call(SPEED_TREE_LEAF_FLOATS, &args![tree]).u32() == 0
            || bs_tree_model_fn_00666f00(e, this)
        {
            finish(e);
            return;
        }
        delete_pointer_array(e, t + 0x18);
        delete_pointer_array(e, t + 0x28);
        let levels = fn_006699d0(e, this);
        if levels == 0 {
            finish(e);
            return;
        }
        let tree = e.get(this, BSTreeModel::pSpeedTree);
        e.call(SPEED_TREE_TEXTURE_NAMES, &args![tree, names]);
        let maps = e.mem.u16(names + 4);
        if maps == 0 {
            finish(e);
            return;
        }
        let mut i: u16 = 0;
        while i < maps {
            let mut j = (u32::from(i) + 1) as u16;
            while j < maps {
                let list = e.mem.u32(names + 8);
                let a = e.mem.u32(list + u32::from(i) * 4);
                let b = e.mem.u32(list + u32::from(j) * 4);
                if e.call(NAMES_DIFFER, &args![a, b]).u32() != 0 {
                    mirrored = true;
                }
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        let data = new_pointer_array(e, u32::from(levels));
        e.set(this, BSTreeModel::pspLeafData, data);
        let properties = new_pointer_array(e, u32::from(levels));
        e.set(this, BSTreeModel::pspLeafShaderProperties, properties);
        let mut level: u16 = 0;
        while level < levels {
            let i = u32::from(level) * 4;
            let slot = e.get(this, BSTreeModel::pspLeafData) + i;
            e.call(NI_POINTER_ASSIGN, &args![slot, 0u32]);
            let slot = e.get(this, BSTreeModel::pspLeafShaderProperties) + i;
            e.call(NI_POINTER_ASSIGN, &args![slot, 0u32]);
            level = level.wrapping_add(1);
        }
        let block = e.call(NI_NEW, &args![0x20u32]).u32();
        let shader_data = if block == 0 {
            0
        } else {
            e.call(LEAF_SHADER_DATA_CONSTRUCTOR, &args![block]).u32()
        };
        e.call(NI_POINTER_ASSIGN, &args![t + 0x20, shader_data]);
        let curve = x87_quiet_bits(e.get(this, BSTreeModel::fCurveScalar));
        let shader_data = pointer(e, t + 0x20);
        e.call(LEAF_DATA_SET_0063F790, &args![shader_data, curve]);
        let tree = e.get(this, BSTreeModel::pSpeedTree);
        if tree != 0 && e.call(SPEED_TREE_0044EDB0, &args![tree]).u32() != 0 {
            let tree = e.get(this, BSTreeModel::pSpeedTree);
            let word = e.call(SPEED_TREE_0044EDB0, &args![tree]).u32();
            let value = x87_store32(e.call(VALUE_00598040, &args![word]).f64());
            let shader_data = pointer(e, t + 0x20);
            e.call(LEAF_DATA_SET_00621B20, &args![shader_data, value]);
            let tree = e.get(this, BSTreeModel::pSpeedTree);
            let word = e.call(SPEED_TREE_0044EDB0, &args![tree]).u32();
            let value = x87_store32(e.call(VALUE_00792760, &args![word]).f64());
            let shader_data = pointer(e, t + 0x20);
            fn_006694c0(e, Ptr::new(shader_data), f32::from_bits(value));
            if !form.is_null() {
                let value = x87_store32(e.vcall(form.addr(), TREE_SLOT_1C4, &args![]).f64());
                let shader_data = pointer(e, t + 0x20);
                e.call(LEAF_DATA_SET_00819A50, &args![shader_data, value]);
                let value = x87_store32(e.vcall(form.addr(), TREE_SLOT_1CC, &args![]).f64());
                let shader_data = pointer(e, t + 0x20);
                e.call(LEAF_DATA_SET_00804CB0, &args![shader_data, value]);
            }
        }
        let mut level: u16 = 0;
        while level < levels {
            let tree = e.get(this, BSTreeModel::pSpeedTree);
            e.call(
                SPEED_TREE_GEOMETRY,
                &args![tree, record, 4u32, u32::MAX, u32::MAX, u32::from(level)],
            );
            let leaves = e.mem.u16(r(0x84));
            let corners = (u32::from(leaves) << 2) as u16;
            if leaves == 0 {
                level = level.wrapping_add(1);
                continue;
            }
            let positions_in = e.mem.u32(r(0x90));
            let normals_in = e.mem.u32(r(0xa0));
            let count = u32::from(corners);
            let positions = new_point_array(e, count, 0xc);
            let normals = new_point_array(e, count, 0xc);
            let coordinates = new_point_array(e, count, 8);
            let colors = new_point_array(e, count, 0x10);
            let indices = e
                .call(BLOCK_ALLOCATE, &args![(u32::from(leaves) * 6) << 1])
                .u32();
            let mut leaf: u16 = 0;
            while leaf < leaves {
                let base = (u32::from(leaf) << 2) as u16;
                let triangle = (u32::from(leaf) * 6) as u16;
                let b = u32::from(base);
                let l = u32::from(leaf);
                copy_words3(
                    e,
                    positions_in.wrapping_add(l * 0xc),
                    positions.wrapping_add((b + 3) * 0xc),
                );
                for k in [2u32, 1, 0] {
                    copy_words3(
                        e,
                        positions.wrapping_add((b + k + 1) * 0xc),
                        positions.wrapping_add((b + k) * 0xc),
                    );
                }
                copy_words3(
                    e,
                    normals_in.wrapping_add(l * 0xc),
                    normals.wrapping_add((b + 3) * 0xc),
                );
                for k in [2u32, 1, 0] {
                    copy_words3(
                        e,
                        normals.wrapping_add((b + k + 1) * 0xc),
                        normals.wrapping_add((b + k) * 0xc),
                    );
                }
                let word = e.mem.u32(e.mem.u32(r(0x9c)).wrapping_add(l * 4));
                let mut shade = x87_store32(x87_div(
                    f64::from((word & 0xff00) >> 8),
                    e.global::<f64>(BYTE_SCALE),
                ));
                if x87_load32(shade) >= e.global::<f64>(ONE) {
                    shade = x87_quiet_bits(e.global::<u32>(LEAF_SHADE_LIMIT));
                }
                let corner_table = e.mem.u32(r(0x94));
                let corner_uvs = e.mem.u32(corner_table.wrapping_add(l * 4));
                let mut corner: u16 = 0;
                while corner < 4 {
                    let k = u32::from(corner);
                    let float = |e: &Engine, index: u32| {
                        x87_quiet_bits(e.mem.u32(corner_uvs.wrapping_add(index.wrapping_mul(4))))
                    };
                    let (u, v) = if mirrored {
                        (float(e, k << 1), float(e, ((3 - k) << 1) + 1))
                    } else {
                        (float(e, k << 1), float(e, (k << 1) + 1))
                    };
                    let made = e.call(POINT2_CONSTRUCTOR, &args![uv, u, v]).u32();
                    let (first, second) = (e.mem.u32(made), e.mem.u32(made + 4));
                    let at = coordinates.wrapping_add((b + k) * 8);
                    e.mem.set_u32(at, first);
                    e.mem.set_u32(at + 4, second);
                    let byte = u32::from(e.mem.u8(e.mem.u32(r(0x8c)).wrapping_add(l)));
                    let step = ((k + 2) & 3) as i32;
                    let total = step.wrapping_add((byte << 2) as i32);
                    let total = x87_store32(f64::from(total));
                    let total = x87_store32(x87_add(x87_load32(total), x87_m32(shade)));
                    let tree = e.get(this, BSTreeModel::pSpeedTree);
                    let floats = e.call(SPEED_TREE_LEAF_FLOATS, &args![tree]).u32();
                    let last = x87_quiet_bits(e.mem.u32(floats.wrapping_add(u32::from(level) * 4)));
                    let byte = u32::from(e.mem.u8(e.mem.u32(r(0xb0)).wrapping_add(l)));
                    let second = x87_store32(f64::from((byte << 2) as i32));
                    let first = x87_quiet_bits(e.mem.u32(e.mem.u32(r(0xac)).wrapping_add(l * 4)));
                    let made = e
                        .call(
                            COLOR_CONSTRUCTOR,
                            &args![color, first, second, x87_quiet_bits(total), last],
                        )
                        .u32();
                    let target = colors.wrapping_add((b + k) << 4);
                    for w in 0..4 {
                        let word = e.mem.u32(made + w * 4);
                        e.mem.set_u32(target.wrapping_add(w * 4), word);
                    }
                    corner += 1;
                }
                let tri = indices.wrapping_add(u32::from(triangle) * 2);
                let order = [3u32, 1, 2, 0, 1, 3];
                for (n, offset) in order.iter().enumerate() {
                    let index = (b + offset) as u16;
                    e.mem.set_u16(tri + n as u32 * 2, index);
                }
                leaf = leaf.wrapping_add(1);
            }
            let block = e.call(NI_NEW, &args![0x58u32]).u32();
            let shape_data = if block == 0 {
                0
            } else {
                e.call(
                    SHAPE_DATA_CONSTRUCTOR,
                    &args![
                        block,
                        u32::from(corners),
                        positions,
                        normals,
                        0u32,
                        coordinates,
                        1u32,
                        0u32,
                        u32::from(leaves) << 1,
                        indices
                    ],
                )
                .u32()
            };
            let i = u32::from(level) * 4;
            let slot = e.get(this, BSTreeModel::pspLeafData) + i;
            e.call(NI_POINTER_ASSIGN, &args![slot, shape_data]);
            let block = e.call(NI_NEW, &args![0x10u32]).u32();
            let made = if block == 0 {
                0
            } else {
                e.call(COLOR_HOLDER_CONSTRUCTOR, &args![block]).u32()
            };
            e.call(NI_POINTER_ASSIGN, &args![holder, made]);
            let held = pointer(e, holder);
            fn_006684d0(e, Ptr::new(held), colors, corners);
            let block = e.call(NI_NEW, &args![0x90u32]).u32();
            let property = if block == 0 {
                0
            } else {
                let shader_data = pointer(e, t + 0x20);
                let held = pointer(e, holder);
                e.call(
                    LEAF_PROPERTY_CONSTRUCTOR,
                    &args![block, u32::from(level), held, shader_data],
                )
                .u32()
            };
            let slot = e.get(this, BSTreeModel::pspLeafShaderProperties) + i;
            e.call(NI_POINTER_ASSIGN, &args![slot, property]);
            let slot = e.get(this, BSTreeModel::pspLeafShaderProperties) + i;
            let made = pointer(e, slot);
            let frond = pointer(e, t + 0x30);
            e.vcall(made, LEAF_PROPERTY_SLOT_DC, &args![frond]);
            let slot = e.get(this, BSTreeModel::pspLeafShaderProperties) + i;
            let made = pointer(e, slot);
            let answer = e.vcall(made, LEAF_PROPERTY_SLOT_F4, &args![0u32]).u32();
            let slot = e.get(this, BSTreeModel::pspLeafData) + i;
            let shape_data = pointer(e, slot);
            e.call(DATA_00A67260, &args![shape_data, answer]);
            let slot = e.get(this, BSTreeModel::pspLeafData) + i;
            let shape_data = pointer(e, slot);
            e.call(SET_CONSISTENCY, &args![shape_data, 0x4000u32]);
            let slot = e.get(this, BSTreeModel::pspLeafData) + i;
            let shape_data = pointer(e, slot);
            e.call(DATA_004410D0, &args![shape_data, 0x11u32]);
            let slot = e.get(this, BSTreeModel::pspLeafData) + i;
            let shape_data = pointer(e, slot);
            e.call(DATA_004410F0, &args![shape_data, 1u32]);
            level = level.wrapping_add(1);
        }
        let tree = e.get(this, BSTreeModel::pSpeedTree);
        e.call(SPEED_TREE_00B088B0, &args![tree]);
        finish(e);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066_67d0, bs_tree_model_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0066_94e0, fn_006694e0(Ptr) -> u32),
        entry!(0x0066_6800, bs_tree_model_destructor(Ptr<BSTreeModel>)),
        entry!(0x0066_6910, fn_00666910(Ptr, u32) -> Ptr),
        entry!(
            0x0066_6940,
            bs_tree_model_apply_base_object(Ptr<BSTreeModel>, Ptr)
        ),
        entry!(
            0x0066_6f00,
            bs_tree_model_fn_00666f00(Ptr<BSTreeModel>) -> bool
        ),
        entry!(0x0066_6f20, fn_00666f20() -> f32),
        entry!(0x0066_6f40, fn_00666f40() -> f32),
        entry!(0x0066_6f60, fn_00666f60() -> f32),
        entry!(0x0066_6f80, fn_00666f80() -> f32),
        entry!(0x0066_6fa0, fn_00666fa0() -> f32),
        entry!(0x0066_6fc0, bs_tree_model_clear_model(Ptr<BSTreeModel>)),
        entry!(0x0066_7120, fn_00667120(Ptr, u32) -> Ptr),
        entry!(
            0x0066_7190,
            bs_tree_model_create_geometry(Ptr<BSTreeModel>, Ptr)
        ),
        entry!(0x0066_84d0, fn_006684d0(Ptr, u32, u16)),
        entry!(0x0066_94c0, fn_006694c0(Ptr, f32)),
        entry!(0x0066_98d0, fn_006698d0(Ptr) -> u32),
        entry!(0x0066_98f0, bs_tree_model_create_alpha_property() -> Ptr),
        entry!(
            0x0066_9990,
            bs_tree_model_get_num_branch_lod_levels(Ptr<BSTreeModel>) -> u16
        ),
        entry!(0x0066_99d0, fn_006699d0(Ptr<BSTreeModel>) -> u16),
        entry!(0x0066_9fb0, fn_00669fb0() -> u8),
        entry!(0x0066_a630, fn_0066a630(Ptr<BSTreeModel>) -> u32),
        entry!(0x0066_7470, bs_tree_model_fn_00667470(Ptr<BSTreeModel>)),
        entry!(
            0x0066_8500,
            bs_tree_model_fn_00668500(Ptr<BSTreeModel>, Ptr)
        ),
        entry!(0x0066_9a10, bs_tree_model_update(Ptr<BSTreeModel>, Ptr)),
        entry!(
            0x0066_9fd0,
            bs_tree_model_create_art(Ptr<BSTreeModel>, f32) -> Ptr
        ),
        entry!(
            0x0066_a650,
            bs_tree_model_init_as_instance(Ptr<BSTreeModel>, Ptr<BSTreeModel>) -> bool
        ),
        entry!(
            0x0066_ac40,
            bs_tree_model_init_from_base(Ptr<BSTreeModel>, Ptr, u32) -> bool
        ),
    ]
    .into_iter()
    .chain(funcs_n086())
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_tree_model_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0066_67d0, 0x0066_6800, 0x50);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;

    #[test]
    fn fn_006694e0_passes_a_null_target_and_returns_the_answer() {
        let mut e = Engine::new();
        e.register_double(0x0063_3c90, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0066_94e0, &args![Ptr::<()>::new(0x5000)]);
        assert_eq!(result.u32(), 0x1234);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0063_3c90, vec![0x5000, 0])]);
    }
}

#[cfg(test)]
mod tree_model_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// One vtable for every test object: slot `offset` calls `slot(offset)`.
    const VTABLE: u32 = 0x0003_0000;
    const TREE: u32 = 0x0000_7000;

    fn slot(offset: u32) -> u32 {
        0x0000_2000 + offset
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

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

    fn answer_first(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }
    }

    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    fn answer_float(e: &mut Engine, address: u32, value: f64) {
        e.register_double(address, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    fn object(e: &mut Engine, size: u32) -> u32 {
        let block = e.mem.alloc(size);
        e.mem.set_u32(block, VTABLE);
        block
    }

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let block = e.mem.alloc(4 * values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, bits(*v));
        }
        block
    }

    fn words(e: &mut Engine, values: &[u32]) -> u32 {
        let block = e.mem.alloc(4 * values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *v);
        }
        block
    }

    /// A pointer array as `new[]` lays it out: the count, then the words.
    fn pointer_array(e: &mut Engine, values: &[u32]) -> u32 {
        let mut all = vec![values.len() as u32];
        all.extend_from_slice(values);
        words(e, &all) + 4
    }

    fn string(e: &mut Engine, text: &[u8]) -> u32 {
        let block = e.mem.alloc(0x104);
        e.mem.set_cstr(block, text);
        block
    }

    /// Doubles every test needs: pointers on memory, allocators, constructors
    /// answering their block, settings at their own address, the constants.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x80).map(|i| slot(i * 4)).collect();
        e.put_vtable(VTABLE, &slots);
        e.register(NI_POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        for address in [NI_POINTER_ASSIGN, NI_POINTER_CONSTRUCT] {
            e.register(address, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        e.register(NI_POINTER_COPY, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        for address in [NI_NEW, OPERATOR_NEW, BLOCK_ALLOCATE] {
            e.register(address, |e, a| Ret {
                eax: e.mem.alloc(a[0].max(4)),
                ..Ret::default()
            });
        }
        e.register(SETTING_FLOAT_VALUE, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(STRCPY, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRCAT, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRLEN, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        for (address, value) in [
            (ZERO, 0.0f64),
            (ONE, 1.0),
            (RIGHT_ANGLE, 90.0),
            (MINIMUM_TRUNK_HEIGHT, 64.0),
            (TWO, 2.0),
            (BYTE_SCALE, 255.0),
        ] {
            e.map(address, 8);
            e.set_global(address, value);
        }
        for (address, value) in [(MINUS_ONE, -1.0f32), (LEAF_SHADE_LIMIT, 0.99)] {
            e.map(address, 4);
            e.set_global(address, value);
        }
        e.map(DEFAULT_NORMAL, 12);
        e.set_global(DEFAULT_NORMAL + 8, 1.0f32);
        quiet(
            &mut e,
            &[
                NI_POINTER_DESTRUCT,
                SCOPE_OPEN,
                SCOPE_CLOSE,
                VECTOR_CONSTRUCTOR_ITERATOR,
                OPERATOR_DELETE,
                GEOMETRY_CONSTRUCTOR,
                GEOMETRY_DESTRUCTOR,
                NAME_HOLDER_CONSTRUCTOR,
                NAME_HOLDER_DESTRUCTOR,
                ARRAY_CONSTRUCTOR,
            ],
        );
        answer_first(
            &mut e,
            &[
                VECTOR_DESTRUCTOR_ITERATOR,
                COLOR_HOLDER_CONSTRUCTOR,
                LEAF_SHADER_DATA_CONSTRUCTOR,
                ALPHA_PROPERTY_CONSTRUCTOR,
                TRI_STRIPS_CONSTRUCTOR,
                TRI_SHAPE_CONSTRUCTOR,
                STRIPS_DATA_CONSTRUCTOR,
                SHAPE_DATA_CONSTRUCTOR,
            ],
        );
        for address in [
            TREE_NODE_CONSTRUCTOR,
            BRANCH_PROPERTY_CONSTRUCTOR,
            LEAF_PROPERTY_CONSTRUCTOR,
            TEXTURE_SET_CONSTRUCTOR,
        ] {
            e.register(address, |e, a| {
                e.mem.set_u32(a[0], VTABLE);
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        for address in [POINT3_CONSTRUCTOR, POINT2_CONSTRUCTOR, COLOR_CONSTRUCTOR] {
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
        e
    }

    fn model(e: &mut Engine) -> Ptr<BSTreeModel> {
        let this = e.new_object::<BSTreeModel>();
        e.mem.set_u32(this.addr(), VTABLE);
        e.set(this, BSTreeModel::pSpeedTree, TREE);
        e.set(this, BSTreeModel::eType, 1);
        this
    }

    fn vcalls(e: &Engine, from: u32, to: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| (slot(from)..=slot(to)).contains(address))
            .map(|(address, words)| (address - slot(0), words[1..].to_vec()))
            .collect()
    }

    #[test]
    fn destructor_deletes_the_speed_tree_and_releases_the_pointers() {
        let mut e = engine();
        quiet(&mut e, &[SPEED_TREE_DESTRUCTOR, REF_OBJECT_DESTRUCTOR]);
        let this = model(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0066_6800, &args![this]);
        let t = this.addr();
        assert_eq!(e.mem.u32(t), BS_TREE_MODEL_VTABLE);
        assert_eq!(calls_to(&e, SPEED_TREE_DESTRUCTOR), vec![vec![TREE]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![TREE]]);
        assert_eq!(e.get(this, BSTreeModel::pSpeedTree), 0);
        let released: Vec<Vec<u32>> = [0x38u32, 0x34, 0x30, 0x2c, 0x20, 0x1c, 0x10]
            .iter()
            .map(|offset| vec![t + offset])
            .collect();
        assert_eq!(calls_to(&e, NI_POINTER_DESTRUCT), released);
        assert_eq!(calls_to(&e, REF_OBJECT_DESTRUCTOR), vec![vec![t]]);
        assert_eq!(calls_to(&e, NI_POINTER_ASSIGN).len(), 6);
    }

    #[test]
    fn speed_tree_deleting_destructor_frees_only_with_bit_zero() {
        let mut e = engine();
        quiet(&mut e, &[SPEED_TREE_DESTRUCTOR]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_6910, &args![TREE, 0u32]).u32(), TREE);
        assert_eq!(e.call(0x0066_6910, &args![TREE, 1u32]).u32(), TREE);
        assert_eq!(calls_to(&e, SPEED_TREE_DESTRUCTOR).len(), 2);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![TREE]]);
    }

    #[test]
    fn apply_base_object_takes_values_in_range_and_names_the_textures() {
        let mut e = engine();
        for (setting, value) in [
            (SETTING_011D5DB8, 2.0f32),
            (SETTING_011D5D24, 0.25),
            (CURVE_SCALAR_SETTING, -1.0),
            (SETTING_011D5C9C, 10.0),
            (SETTING_011D5C80, 5.0),
        ] {
            e.map(setting, 4);
            e.set_global(setting, value);
        }
        answer_float(&mut e, slot(TREE_SLOT_1AC), 0.5);
        answer_float(&mut e, slot(TREE_SLOT_194), 3.0);
        answer_float(&mut e, slot(TREE_SLOT_19C), 20.0);
        answer_float(&mut e, slot(TREE_SLOT_1A4), 30.0);
        quiet(
            &mut e,
            &[
                SPEED_TREE_SET_00B07390,
                SPEED_TREE_SET_00B073C0,
                SPEED_TREE_SET_00B07490,
                SPEED_TREE_SET_00B074C0,
                GET_MODIFIED_TEXTURE_FILENAME,
                GET_BILLBOARD_TEXTURE_FILENAME,
                slot(TEXTURE_SET_SET_SLOT),
            ],
        );
        e.register(SPEED_TREE_TEXTURE_NAMES, |e, a| {
            let name = e.mem.alloc(16);
            e.mem.set_cstr(name, b"oak.tga");
            e.mem.set_u32(a[1], name);
            Ret::default()
        });
        e.map(BRANCHES_FOLDER, 0x20);
        e.mem
            .set_cstr(BRANCHES_FOLDER, b"Textures\\Trees\\Branches\\");
        let names = Rc::new(RefCell::new(Vec::new()));
        let seen = names.clone();
        e.register_double(slot(TEXTURE_SET_SET_SLOT), move |e, a| {
            seen.borrow_mut().push(e.mem.cstr(a[2]));
            Ret::default()
        });
        let this = model(&mut e);
        e.mem.set_u32(this.addr() + 0x30, 0x8800);
        let tree = Ptr::<()>::new(object(&mut e, 0x60));
        e.call_log = Some(vec![]);
        e.call(0x0066_6940, &args![this, tree]);
        assert_eq!(
            calls_to(&e, SPEED_TREE_SET_00B07390),
            vec![vec![TREE, bits(0.5)]]
        );
        assert_eq!(
            calls_to(&e, SPEED_TREE_SET_00B073C0),
            vec![vec![TREE, bits(0.25)]]
        );
        assert_eq!(e.get(this, BSTreeModel::fCurveScalar), bits(3.0));
        assert_eq!(
            calls_to(&e, SPEED_TREE_SET_00B07490),
            vec![vec![TREE, bits(20.0)]]
        );
        assert_eq!(
            calls_to(&e, SPEED_TREE_SET_00B074C0),
            vec![vec![TREE, bits(30.0)]]
        );
        assert!(calls_to(&e, slot(TREE_SLOT_1B4)).is_empty());
        assert_eq!(
            calls_to(&e, SCOPE_OPEN)[0][1..],
            [0x1e, 1, SOURCE_FILE, 0x91]
        );
        assert_eq!(
            names.borrow()[0],
            b"Textures\\Trees\\Branches\\oak.dds".to_vec()
        );
        assert_eq!(names.borrow().len(), 3);
        assert!(calls_to(&e, LEAF_TEXTURE_PRESENT).is_empty());
        assert_ne!(e.mem.u32(this.addr() + 0x2c), 0);
        assert_ne!(e.mem.u32(this.addr() + 0x34), 0);
        assert_eq!(calls_to(&e, NI_POINTER_DESTRUCT).len(), 1);
    }

    #[test]
    fn fn_00666f00_tests_for_the_second_kind() {
        let mut e = engine();
        let this = model(&mut e);
        assert!(!e.call(0x0066_6f00, &args![this]).bool());
        e.set(this, BSTreeModel::eType, 2);
        assert!(e.call(0x0066_6f00, &args![this]).bool());
    }

    fn check_setting(address: u32, setting: u32) {
        let mut e = engine();
        e.map(setting, 4);
        e.set_global(setting, 1.5f32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(address, &args![]).f32(), 1.5);
        assert_eq!(calls_to(&e, SETTING_FLOAT_VALUE), vec![vec![setting]]);
    }

    #[test]
    fn fn_00666f20_reads_the_curve_scalar_setting() {
        check_setting(0x0066_6f20, CURVE_SCALAR_SETTING);
    }

    #[test]
    fn fn_00666f40_reads_its_setting() {
        check_setting(0x0066_6f40, SETTING_011D5DB8);
    }

    #[test]
    fn fn_00666f60_reads_its_setting() {
        check_setting(0x0066_6f60, SETTING_011D5D24);
    }

    #[test]
    fn fn_00666f80_reads_its_setting() {
        check_setting(0x0066_6f80, SETTING_011D5C9C);
    }

    #[test]
    fn fn_00666fa0_reads_its_setting() {
        check_setting(0x0066_6fa0, SETTING_011D5C80);
    }

    #[test]
    fn clear_model_deletes_the_arrays_and_drops_the_pointers() {
        let mut e = engine();
        let this = model(&mut e);
        let branch = pointer_array(&mut e, &[0, 0]);
        let leaf_properties = pointer_array(&mut e, &[0]);
        e.set(this, BSTreeModel::pspBranchData, branch);
        e.set(this, BSTreeModel::pspLeafShaderProperties, leaf_properties);
        e.mem.set_u32(this.addr() + 0x38, 0x8100);
        e.call_log = Some(vec![]);
        e.call(0x0066_6fc0, &args![this]);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCTOR_ITERATOR),
            vec![
                vec![branch, 4, 2, NI_POINTER_DESTRUCT],
                vec![leaf_properties, 4, 1, NI_POINTER_DESTRUCT]
            ]
        );
        assert_eq!(e.get(this, BSTreeModel::pspBranchData), 0);
        assert_eq!(e.get(this, BSTreeModel::pspLeafShaderProperties), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 0);
        assert_eq!(calls_to(&e, NI_POINTER_ASSIGN).len(), 6);
    }

    #[test]
    fn pointer_array_deleting_destructor_handles_arrays_and_single_pointers() {
        let mut e = engine();
        let array = pointer_array(&mut e, &[0, 0, 0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_7120, &args![array, 3u32]).u32(), array - 4);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCTOR_ITERATOR),
            vec![vec![array, 4, 3, NI_POINTER_DESTRUCT]]
        );
        assert_eq!(e.call(0x0066_7120, &args![array, 1u32]).u32(), array);
        assert_eq!(calls_to(&e, NI_POINTER_DESTRUCT), vec![vec![array]]);
        assert_eq!(
            calls_to(&e, OPERATOR_DELETE),
            vec![vec![array - 4], vec![array]]
        );
    }

    fn geometry_engine(found: bool) -> (Engine, Ptr<BSTreeModel>, Ptr) {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                0x0066_7470,
                0x0066_8500,
                CREATE_BILLBOARD,
                STREAM_CONSTRUCTOR,
                STREAM_DESTRUCTOR,
            ],
        );
        answer(&mut e, BILLBOARD_PRESENT, 1);
        e.map(COLLISION_EXTENSION, 0x20);
        e.mem.set_cstr(COLLISION_EXTENSION, b".NIF");
        e.mem.set_cstr(COLLISION_PREFIX, b"Data\\Meshes\\Trees");
        let path = string(&mut e, b"Trees\\Oak.spt");
        answer(&mut e, slot(MODEL_PATH_SLOT), path);
        e.register_double(FILE_EXISTS, move |e, a| Ret {
            eax: u32::from(found && e.mem.cstr(a[0]) == b"Data\\Meshes\\TreesTrees\\Oak.NIF"),
            ..Ret::default()
        });
        answer(&mut e, STREAM_LOAD, 1);
        answer(&mut e, STREAM_OBJECT_COUNT, 1);
        answer(&mut e, STREAM_OBJECT, 0x8000);
        e.register(DYNAMIC_CAST, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        answer(&mut e, COLLISION_OBJECT, 0x8100);
        answer(&mut e, COLLISION_BODY, 0x8200);
        answer(&mut e, CREATE_TREE_BODY, 0x8300);
        let this = model(&mut e);
        e.set(this, BSTreeModel::fTrunkHeight, bits(100.0));
        e.set(this, BSTreeModel::fTrunkWidth, bits(8.0));
        let tree = e.mem.alloc(0x60);
        e.mem.set_u32(tree + 0x30, VTABLE);
        (e, this, Ptr::new(tree))
    }

    #[test]
    fn create_geometry_takes_the_collision_file_body_or_makes_one() {
        let (mut e, this, tree) = geometry_engine(true);
        e.call_log = Some(vec![]);
        e.call(0x0066_7190, &args![this, tree]);
        assert_eq!(calls_to(&e, 0x0066_7470), vec![vec![this.addr()]]);
        assert_eq!(
            calls_to(&e, 0x0066_8500),
            vec![vec![this.addr(), tree.addr()]]
        );
        assert_eq!(
            calls_to(&e, CREATE_BILLBOARD),
            vec![vec![this.addr(), tree.addr()]]
        );
        assert_eq!(e.mem.u32(this.addr() + 0x38), 0x8200);
        assert!(calls_to(&e, CREATE_TREE_BODY).is_empty());
        let (mut e, this, tree) = geometry_engine(false);
        e.call_log = Some(vec![]);
        e.call(0x0066_7190, &args![this, tree]);
        assert_eq!(
            calls_to(&e, CREATE_TREE_BODY),
            vec![vec![bits(100.0), bits(4.0)]]
        );
        assert_eq!(e.mem.u32(this.addr() + 0x38), 0x8300);
        assert_eq!(calls_to(&e, STREAM_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn fn_006684d0_stores_the_pointer_and_count() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.call(0x0066_84d0, &args![this, 0x1234u32, 0x0001_0005u32]);
        assert_eq!(e.mem.u32(this + 8), 0x1234);
        assert_eq!(e.mem.u16(this + 0xc), 5);
    }

    #[test]
    fn fn_006694c0_stores_the_float_quieted() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        e.call(0x0066_94c0, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0x14), 0x7fc0_0001);
    }

    #[test]
    fn fn_006698d0_reads_the_pointer_at_0x6c() {
        let mut e = engine();
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x6c, 0x4321);
        assert_eq!(e.call(0x0066_98d0, &args![this]).u32(), 0x4321);
    }

    #[test]
    fn create_alpha_property_sets_up_the_property() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                ALPHA_SET_0049ED90,
                ALPHA_SET_004393C0,
                ALPHA_SET_004393E0,
                ALPHA_SET_0094DB80,
            ],
        );
        e.call_log = Some(vec![]);
        let property = e.call(0x0066_98f0, &args![]).u32();
        assert_ne!(property, 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x1c]]);
        assert_eq!(calls_to(&e, ALPHA_SET_0049ED90), vec![vec![property, 0]]);
        assert_eq!(calls_to(&e, ALPHA_SET_004393C0), vec![vec![property, 1]]);
        assert_eq!(calls_to(&e, ALPHA_SET_004393E0), vec![vec![property, 4]]);
        assert_eq!(calls_to(&e, ALPHA_SET_0094DB80), vec![vec![property, 0x54]]);
    }

    #[test]
    fn get_num_branch_lod_levels_asks_the_speed_tree() {
        let mut e = engine();
        answer(&mut e, SPEED_TREE_BRANCH_LODS, 0x0002_0003);
        let this = model(&mut e);
        assert_eq!(e.call(0x0066_9990, &args![this]).u16(), 3);
        e.set(this, BSTreeModel::pSpeedTree, 0);
        assert_eq!(e.call(0x0066_9990, &args![this]).u16(), 0);
    }

    #[test]
    fn fn_006699d0_asks_the_speed_tree_for_leaf_levels() {
        let mut e = engine();
        answer(&mut e, SPEED_TREE_LEAF_LODS, 4);
        let this = model(&mut e);
        assert_eq!(e.call(0x0066_99d0, &args![this]).u16(), 4);
        e.set(this, BSTreeModel::pSpeedTree, 0);
        assert_eq!(e.call(0x0066_99d0, &args![this]).u16(), 0);
    }

    #[test]
    fn fn_00669fb0_reads_the_tree_manager_byte() {
        let mut e = engine();
        let manager = e.mem.alloc(0x20);
        e.mem.set_u8(manager + 0x1b, 7);
        answer(&mut e, TREE_MANAGER_INSTANCE, manager);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_9fb0, &args![]).u8(), 7);
        assert_eq!(calls_to(&e, TREE_MANAGER_INSTANCE), vec![vec![1]]);
    }

    #[test]
    fn fn_0066a630_reads_the_rigid_body() {
        let mut e = engine();
        let this = model(&mut e);
        e.mem.set_u32(this.addr() + 0x38, 0x8100);
        assert_eq!(e.call(0x0066_a630, &args![this]).u32(), 0x8100);
    }

    /// An index set double: +0 the count, +4 the entries.
    fn index_set(e: &mut Engine) {
        e.register(INDEX_SET_CONSTRUCTOR, |e, a| {
            let entries = e.mem.alloc(0x40);
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, entries);
            Ret::default()
        });
        e.register(INDEX_SET_ADD, |e, a| {
            let count = e.mem.u32(a[0]);
            let entries = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(entries + count * 4, value);
            e.mem.set_u32(a[0], count + 1);
            Ret::default()
        });
        e.register(INDEX_SET_SIZE, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(INDEX_SET_AT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4) + a[1] * 4,
            ..Ret::default()
        });
        quiet(e, &[INDEX_SET_DESTRUCTOR]);
        e.register(MEMORY_COPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        e.register(MEMORY_FILL, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
    }

    #[test]
    fn branch_geometry_keeps_the_vertices_the_strip_uses() {
        let mut e = engine();
        index_set(&mut e);
        answer(&mut e, SPEED_TREE_BRANCH_LODS, 1);
        e.register(IS_NAN, |_, a| Ret {
            eax: u32::from(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).is_nan()),
            ..Ret::default()
        });
        quiet(
            &mut e,
            &[
                SET_CONSISTENCY,
                DATA_004410D0,
                DATA_004410F0,
                SPEED_TREE_00B08750,
                SPEED_TREE_00B08780,
            ],
        );
        let length = words(&mut e, &[3]);
        let strip = words(&mut e, &[0x0000_0002, 0x0000_0002]);
        let strips = words(&mut e, &[strip]);
        let colors = words(&mut e, &[0, 0, 0x0000_3300]);
        let normals = floats(&mut e, &[0.0, 1.0, 0.0, 0.0, 1.0, 0.0, f32::NAN, 0.0, 0.0]);
        let tangents = floats(&mut e, &[0.0; 9]);
        let binormals = floats(&mut e, &[1.0; 9]);
        let positions = floats(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let coordinates = floats(&mut e, &[0.1, 0.2, 0.3, 0.4, 0.5, 0.6]);
        let reds = floats(&mut e, &[0.0, 0.0, 0.75]);
        let bytes = words(&mut e, &[0x0003_0000]);
        let fields = [
            (8u32, length),
            (0xc, strips),
            (0x14, colors),
            (0x18, normals),
            (0x1c, tangents),
            (0x20, binormals),
            (0x24, positions),
            (0x28, coordinates),
            (0x30, reds),
            (0x34, bytes),
        ];
        e.register_double(SPEED_TREE_GEOMETRY, move |e, a| {
            for (offset, value) in fields {
                e.mem.set_u32(a[1] + offset, value);
            }
            e.mem.set_u16(a[1] + 0x10, 3);
            Ret::default()
        });
        let this = model(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0066_7470, &args![this]);
        let data = calls_to(&e, STRIPS_DATA_CONSTRUCTOR);
        assert_eq!(data.len(), 1);
        let data = &data[0];
        assert_eq!(data[1], 2);
        assert_eq!(data[4..], [0, data[5], 1, 0x1000, 1, 1, data[10], data[11]]);
        let indices = data[11];
        assert_eq!(
            [
                e.mem.u16(indices),
                e.mem.u16(indices + 2),
                e.mem.u16(indices + 4)
            ],
            [1, 0, 1]
        );
        let p = data[2];
        assert_eq!(e.mem.u32(p), bits(1.0));
        assert_eq!(e.mem.u32(p + 0xc), bits(7.0));
        let n = data[3];
        assert_eq!([e.mem.u32(n + 0xc), e.mem.u32(n + 0x14)], [0, bits(1.0)]);
        assert_eq!([e.mem.u32(n + 0x24), e.mem.u32(n + 0x30)], [0, bits(1.0)]);
        let uv = data[5];
        assert_eq!(e.mem.u32(uv + 8), bits(0.5));
        let holder = calls_to(&e, BRANCH_PROPERTY_CONSTRUCTOR)[0][1];
        let colors_out = e.mem.u32(holder + 8);
        assert_eq!(e.mem.u16(holder + 0xc), 2);
        assert_eq!(
            [
                e.mem.u32(colors_out + 0x10),
                e.mem.u32(colors_out + 0x14),
                e.mem.u32(colors_out + 0x18),
                e.mem.u32(colors_out + 0x1c)
            ],
            [bits(0.75), bits(12.0), bits((51.0f64 / 255.0) as f32), 0]
        );
        let branch = e.get(this, BSTreeModel::pspBranchData);
        assert_eq!(e.mem.u32(branch), data[0]);
        let properties = e.get(this, BSTreeModel::pspBranchShaderProperties);
        assert_ne!(e.mem.u32(properties), 0);
        assert_eq!(calls_to(&e, SET_CONSISTENCY), vec![vec![data[0], 0x4000]]);
        assert_eq!(calls_to(&e, SPEED_TREE_00B08780), vec![vec![TREE]]);
    }

    #[test]
    fn leaf_geometry_makes_four_corners_per_leaf() {
        let mut e = engine();
        answer(&mut e, SPEED_TREE_LEAF_LODS, 1);
        let level_floats = floats(&mut e, &[0.75]);
        answer(&mut e, SPEED_TREE_LEAF_FLOATS, level_floats);
        answer(&mut e, SPEED_TREE_0044EDB0, 0x9000);
        answer_float(&mut e, VALUE_00598040, 1.5);
        answer_float(&mut e, VALUE_00792760, 2.5);
        answer(&mut e, NAMES_DIFFER, 1);
        answer(&mut e, slot(LEAF_PROPERTY_SLOT_F4), 0x4444);
        quiet(
            &mut e,
            &[
                LEAF_DATA_SET_0063F790,
                LEAF_DATA_SET_00621B20,
                slot(LEAF_PROPERTY_SLOT_DC),
                DATA_00A67260,
                SET_CONSISTENCY,
                DATA_004410D0,
                DATA_004410F0,
                SPEED_TREE_00B088B0,
            ],
        );
        e.register(BLOCK_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        let name_list = words(&mut e, &[0x100, 0x200]);
        e.register_double(SPEED_TREE_TEXTURE_NAMES, move |e, a| {
            e.mem.set_u16(a[1] + 4, 2);
            e.mem.set_u32(a[1] + 8, name_list);
            Ret::default()
        });
        let positions = floats(&mut e, &[1.0, 2.0, 3.0]);
        let normals = floats(&mut e, &[0.0, 0.0, 1.0]);
        let colors = words(&mut e, &[0x0000_ff00]);
        let uvs = floats(&mut e, &[0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7]);
        let corners = words(&mut e, &[uvs]);
        let steps = words(&mut e, &[1]);
        let reds = floats(&mut e, &[0.5]);
        let greens = words(&mut e, &[2]);
        let fields = [
            (0x8cu32, steps),
            (0x90, positions),
            (0x94, corners),
            (0x9c, colors),
            (0xa0, normals),
            (0xac, reds),
            (0xb0, greens),
        ];
        e.register_double(SPEED_TREE_GEOMETRY, move |e, a| {
            for (offset, value) in fields {
                e.mem.set_u32(a[1] + offset, value);
            }
            e.mem.set_u16(a[1] + 0x84, 1);
            Ret::default()
        });
        let this = model(&mut e);
        e.set(this, BSTreeModel::fCurveScalar, bits(2.0));
        e.mem.set_u32(this.addr() + 0x30, 0x8800);
        e.call_log = Some(vec![]);
        e.call(0x0066_8500, &args![this, 0u32]);
        assert_eq!(calls_to(&e, NAMES_DIFFER), vec![vec![0x100, 0x200]]);
        assert_eq!(
            calls_to(&e, SPEED_TREE_GEOMETRY),
            vec![vec![
                TREE,
                calls_to(&e, GEOMETRY_CONSTRUCTOR)[0][0],
                4,
                u32::MAX,
                u32::MAX,
                0
            ]]
        );
        let shader_data = e.mem.u32(this.addr() + 0x20);
        assert_eq!(
            calls_to(&e, LEAF_DATA_SET_0063F790),
            vec![vec![shader_data, bits(2.0)]]
        );
        assert_eq!(
            calls_to(&e, LEAF_DATA_SET_00621B20),
            vec![vec![shader_data, bits(1.5)]]
        );
        assert_eq!(e.mem.u32(shader_data + 0x14), bits(2.5));
        let data = calls_to(&e, SHAPE_DATA_CONSTRUCTOR);
        let data = &data[0];
        assert_eq!(data[1], 4);
        assert_eq!(data[4..9], [0, data[5], 1, 0, 2]);
        let indices = data[9];
        let order: Vec<u16> = (0..6).map(|i| e.mem.u16(indices + i * 2)).collect();
        assert_eq!(order, [3, 1, 2, 0, 1, 3]);
        for corner in 0..4 {
            assert_eq!(e.mem.u32(data[2] + corner * 0xc + 4), bits(2.0));
            assert_eq!(e.mem.u32(data[3] + corner * 0xc + 8), bits(1.0));
        }
        let uv = data[5];
        assert_eq!([e.mem.u32(uv), e.mem.u32(uv + 4)], [bits(0.0), bits(0.7)]);
        assert_eq!(
            [e.mem.u32(uv + 8), e.mem.u32(uv + 0xc)],
            [bits(0.2), bits(0.5)]
        );
        let property = calls_to(&e, LEAF_PROPERTY_CONSTRUCTOR);
        assert_eq!(property[0][1..], [0, property[0][2], shader_data]);
        let holder = property[0][2];
        let colors_out = e.mem.u32(holder + 8);
        assert_eq!(e.mem.u16(holder + 0xc), 4);
        let shade = f64::from(0.99f32);
        assert_eq!(
            [
                e.mem.u32(colors_out),
                e.mem.u32(colors_out + 4),
                e.mem.u32(colors_out + 8),
                e.mem.u32(colors_out + 0xc)
            ],
            [bits(0.5), bits(8.0), bits((6.0 + shade) as f32), bits(0.75)]
        );
        assert_eq!(e.mem.u32(colors_out + 0x18), bits((7.0 + shade) as f32));
        assert_eq!(calls_to(&e, slot(LEAF_PROPERTY_SLOT_DC))[0][1], 0x8800);
        assert_eq!(calls_to(&e, DATA_00A67260), vec![vec![data[0], 0x4444]]);
        assert_eq!(calls_to(&e, SPEED_TREE_00B088B0), vec![vec![TREE]]);
    }

    #[test]
    fn update_shows_the_levels_the_record_names() {
        let mut e = engine();
        answer(&mut e, slot(NODE_HAS_PARENT_SLOT), 1);
        quiet(
            &mut e,
            &[
                slot(NODE_SLOT_128),
                slot(NODE_SLOT_12C),
                slot(NODE_SLOT_130),
                slot(NODE_SLOT_134),
                slot(NODE_SLOT_138),
                slot(NODE_SLOT_13C),
                POINT_CONSTRUCTOR,
                SPEED_TREE_SET_CAMERA,
                SPEED_TREE_COMPUTE_LOD_LEVEL,
            ],
        );
        answer(&mut e, UPDATE_ALLOWED, 1);
        let translate = floats(&mut e, &[1.0, 2.0, 3.0]);
        answer(&mut e, NODE_WORLD_TRANSLATE, translate);
        let manager = e.mem.alloc(0x20);
        answer(&mut e, TREE_MANAGER_INSTANCE, manager);
        answer_float(&mut e, SPEED_TREE_GET_LOD_LEVEL, 0.5);
        e.register(SPEED_TREE_LOD_GEOMETRY, |e, a| {
            let record = a[1];
            e.mem.set_u8(record + 0xf0, 1);
            e.mem.set_u32(record + 0xfc, 200.7f32.to_bits());
            e.mem.set_u32(record + 0x38, 100.9f32.to_bits());
            e.mem.set_u8(record + 0x78, 1);
            e.mem.set_u32(record + 0x80, 1);
            e.mem.set_u32(record + 0x7c, 50.2f32.to_bits());
            Ret::default()
        });
        answer(&mut e, SPEED_TREE_BRANCH_LODS, 2);
        answer(&mut e, SPEED_TREE_DISCRETE_BRANCH_LOD, 1);
        answer(&mut e, ARE_LEAVES_VISIBLE, 1);
        answer(&mut e, SPEED_TREE_LEAF_LODS, 2);
        let this = model(&mut e);
        let node = object(&mut e, 0x40);
        e.call_log = Some(vec![]);
        e.call(0x0066_9a10, &args![this, node]);
        assert_eq!(
            calls_to(&e, SPEED_TREE_SET_CAMERA),
            vec![vec![TREE, bits(1.0), bits(2.0), bits(3.0)]]
        );
        assert_eq!(calls_to(&e, SPEED_TREE_LOD_GEOMETRY)[0][2], bits(0.5));
        assert_eq!(
            vcalls(&e, NODE_SLOT_128, NODE_SLOT_13C),
            vec![
                (NODE_SLOT_13C, vec![200]),
                (NODE_SLOT_130, vec![1]),
                (NODE_SLOT_134, vec![0, 0xff]),
                (NODE_SLOT_134, vec![1, 100]),
                (NODE_SLOT_128, vec![1]),
                (NODE_SLOT_138, vec![0, 0xff]),
                (NODE_SLOT_138, vec![1, 50]),
                (NODE_SLOT_12C, vec![1]),
            ]
        );
        assert_eq!(calls_to(&e, GEOMETRY_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn create_art_builds_a_node_with_the_branch_shapes() {
        let mut e = engine();
        e.register(GET_SHADER, |_, a| Ret {
            eax: 0x40 + a[0],
            ..Ret::default()
        });
        answer(&mut e, slot(NODE_SLOT_110), 0x8b00);
        answer(&mut e, slot(PROPERTY_SLOT_D4), 0x77);
        answer(&mut e, SHAPE_005495F0, 0x8c00);
        answer(&mut e, TEST_004A8A90, 0);
        answer(&mut e, TEST_005F36F0, 0);
        let loader = object(&mut e, 0x10);
        answer(&mut e, MODEL_LOADER, loader);
        quiet(
            &mut e,
            &[
                slot(NODE_SLOT_104),
                slot(NODE_SLOT_10C),
                slot(NODE_SLOT_11C),
                slot(NODE_SLOT_124),
                slot(MODEL_LOADER_SLOT_EC),
                ALPHA_SET_0049ED90,
                ALPHA_SET_004393C0,
                ALPHA_SET_004393E0,
                ALPHA_SET_0094DB80,
                ATTACH_PROPERTY,
                FREE_RENDER_PASSES,
                SHAPE_00539EF0,
                LOAD_TEXTURE_SET,
                PREPARE_OBJECT,
                UPDATE_PROPERTIES,
                UPDATE_RECORD_CONSTRUCTOR,
                NODE_UPDATE,
                MAKE_HAVOK_LEAVES,
            ],
        );
        answer(&mut e, SPEED_TREE_BRANCH_LODS, 1);
        let this = model(&mut e);
        e.set(this, BSTreeModel::fTrunkHeight, bits(10.0));
        let property = object(&mut e, 0x10);
        let data = pointer_array(&mut e, &[0x8000]);
        let properties = pointer_array(&mut e, &[property]);
        e.set(this, BSTreeModel::pspBranchData, data);
        e.set(this, BSTreeModel::pspBranchShaderProperties, properties);
        e.mem.set_u32(this.addr() + 0x1c, 0x8900);
        e.mem.set_u32(this.addr() + 0x2c, 0x8a00);
        e.call_log = Some(vec![]);
        let node = e.call(0x0066_9fd0, &args![this, 2.0f32]).u32();
        assert_eq!(
            calls_to(&e, TREE_NODE_CONSTRUCTOR),
            vec![vec![node, this.addr(), 0]]
        );
        let shape = calls_to(&e, TRI_STRIPS_CONSTRUCTOR);
        assert_eq!(shape[0][1], 0x8000);
        let shape = shape[0][0];
        assert_eq!(
            calls_to(&e, slot(NODE_SLOT_11C)),
            vec![vec![node, 0, shape]]
        );
        assert_eq!(
            calls_to(&e, SHAPE_00539EF0),
            vec![vec![shape, 0x77], vec![0x8b00, 0x77]]
        );
        assert!(calls_to(&e, SET_SHADER).is_empty());
        assert_eq!(
            calls_to(&e, LOAD_TEXTURE_SET),
            vec![vec![property, BRANCH_TEXTURE_NAME, 0x8a00]]
        );
        assert_eq!(
            calls_to(&e, slot(MODEL_LOADER_SLOT_EC)),
            vec![vec![loader, 0x8b00, 0, 0, 0]]
        );
        assert_eq!(calls_to(&e, slot(NODE_SLOT_124)), vec![vec![node, 0x8900]]);
        assert_eq!(
            calls_to(&e, MAKE_HAVOK_LEAVES),
            vec![vec![node, TREE, bits(2.0)]]
        );
        assert_eq!(calls_to(&e, ATTACH_PROPERTY).len(), 2);
    }

    #[test]
    fn init_as_instance_shares_the_base_and_clones_its_parts() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                LOCK_ENTER,
                LOCK_LEAVE,
                CLONING_PROCESS_CONSTRUCTOR,
                CLONING_PROCESS_DESTRUCTOR,
            ],
        );
        answer(&mut e, SPEED_TREE_MAKE_INSTANCE, 0x7100);
        e.register(NI_OBJECT_CLONE_WITH, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.map(CLONING_BILLBOARD, 4);
        e.register(NI_OBJECT_CLONE, |e, a| Ret {
            eax: a[0] + 1 + 0x10 * u32::from(e.mem.u8(CLONING_BILLBOARD)),
            ..Ret::default()
        });
        answer(&mut e, SPEED_TREE_BRANCH_LODS, 1);
        answer(&mut e, SPEED_TREE_LEAF_LODS, 0);
        let base = model(&mut e);
        let b = base.addr();
        e.set(base, BSTreeModel::fCurveScalar, bits(2.5));
        e.set(base, BSTreeModel::iSeed, 7);
        e.set(base, BSTreeModel::fTrunkHeight, bits(100.0));
        e.set(base, BSTreeModel::fTrunkWidth, bits(8.0));
        e.mem.set_u32(b + 0x38, 0x8100);
        e.mem.set_u32(b + 0x1c, 0x8400);
        e.mem.set_u32(b + 0x2c, 0x8a00);
        let data = pointer_array(&mut e, &[0x8200]);
        let properties = pointer_array(&mut e, &[0x8300]);
        e.set(base, BSTreeModel::pspBranchData, data);
        e.set(base, BSTreeModel::pspBranchShaderProperties, properties);
        let this = e.new_object::<BSTreeModel>();
        let t = this.addr();
        e.call_log = Some(vec![]);
        assert!(e.call(0x0066_a650, &args![this, base]).bool());
        assert_eq!(e.get(this, BSTreeModel::pSpeedTree), 0x7100);
        assert_eq!(e.mem.u32(t + 0x10), b);
        assert_eq!(e.get(this, BSTreeModel::eType), 2);
        assert_eq!(e.mem.u32(t + 0x2c), 0x8a00);
        assert_eq!(e.get(this, BSTreeModel::iSeed), 7);
        assert_eq!(e.get(this, BSTreeModel::fTrunkWidth), bits(8.0));
        assert_eq!(e.mem.u32(t + 0x38), 0x8101);
        assert_eq!(e.mem.u32(e.get(this, BSTreeModel::pspBranchData)), 0x8200);
        assert_eq!(
            e.mem
                .u32(e.get(this, BSTreeModel::pspBranchShaderProperties)),
            0x8301
        );
        assert_eq!(e.mem.u32(t + 0x1c), 0x8411);
        assert_eq!(e.mem.u8(CLONING_BILLBOARD), 0);
        assert_eq!(calls_to(&e, LOCK_LEAVE).len(), 1);
        assert_eq!(
            calls_to(&e, SCOPE_OPEN)[0][1..],
            [0x1f, 1, SOURCE_FILE, 0x411]
        );
        assert!(!e.call(0x0066_a650, &args![this, base]).bool());
        assert_eq!(calls_to(&e, LOCK_LEAVE).len(), 2);
    }

    fn base_engine(loads: bool) -> (Engine, Ptr<BSTreeModel>, Ptr) {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                SPEED_TREE_DESTRUCTOR,
                SPEED_TREE_SET_ROCKING_GROUPS,
                LOG,
                SPEED_TREE_00B071C0,
                SPEED_TREE_SET_TREE_SIZE,
                SPEED_TREE_SET_WIND_STRENGTH,
                SPEED_TREE_SET_LOCAL_MATRICES,
                SPEED_TREE_00984F60,
                SPEED_TREE_00B08500,
                SPEED_TREE_00B087D0,
                SPEED_TREE_DELETE_TRANSIENT_DATA,
                SPEED_TREE_00B088D0,
                SPEED_TREE_00B0A7A0,
                slot(MODEL_SLOT_10),
                slot(MODEL_SLOT_18),
            ],
        );
        quiet(&mut e, &SPEED_TREE_METHODS);
        answer_first(&mut e, &[SPEED_TREE_CONSTRUCTOR]);
        e.register_double(SPEED_TREE_LOAD_TREE, move |e, a| Ret {
            eax: u32::from(loads && e.mem.cstr(a[1]) == b"TreesOak.spt"),
            ..Ret::default()
        });
        e.register(SPEED_TREE_TEXTURE_NAMES, |e, a| {
            e.mem.set_u32(a[1] + 4, 7);
            Ret::default()
        });
        e.register(SPEED_TREE_GET_SIZE, |e, a| {
            e.mem.set_u32(a[1], bits(3.0));
            e.mem.set_u32(a[2], bits(4.0));
            Ret::default()
        });
        answer_float(&mut e, TREE_SIZE_SCALE, 2.0);
        answer_float(&mut e, VALUE_006655C0, 10.0);
        answer_float(&mut e, VALUE_00665570, 20.0);
        answer(&mut e, SPEED_TREE_COMPUTE, 1);
        answer(&mut e, SPEED_TREE_SEED, 42);
        answer_float(&mut e, SPEED_TREE_TRUNK_HEIGHT, 120.0);
        answer_float(&mut e, SPEED_TREE_TRUNK_WIDTH, 9.0);
        e.map(TREES_FOLDER, 8);
        e.mem.set_cstr(TREES_FOLDER, b"Trees");
        let path = string(&mut e, b"Oak.spt");
        answer(&mut e, slot(MODEL_PATH_SLOT), path);
        let this = model(&mut e);
        e.set(this, BSTreeModel::eType, 0);
        e.set(this, BSTreeModel::pSpeedTree, 0);
        let form = e.mem.alloc(0x40);
        e.mem.set_u32(form + 0x30, VTABLE);
        (e, this, Ptr::new(form))
    }

    #[test]
    fn init_from_base_loads_and_computes_the_tree() {
        let (mut e, this, form) = base_engine(true);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0066_ac40, &args![this, form, 5u32]).bool());
        let tree = e.get(this, BSTreeModel::pSpeedTree);
        assert_ne!(tree, 0);
        assert_eq!(e.get(this, BSTreeModel::eType), 1);
        assert_eq!(
            calls_to(&e, SPEED_TREE_SET_ROCKING_GROUPS),
            vec![vec![tree, 1]]
        );
        assert_eq!(calls_to(&e, LOG).len(), 1);
        assert_eq!(
            calls_to(&e, SPEED_TREE_SET_TREE_SIZE),
            vec![vec![tree, bits(6.0), bits(8.0)]]
        );
        assert_eq!(
            calls_to(&e, SPEED_TREE_SET_WIND_STRENGTH),
            vec![vec![tree, 0, bits(-1.0), bits(-1.0)]]
        );
        assert_eq!(
            calls_to(&e, SPEED_TREE_00B08500),
            vec![vec![tree, bits(20.0), bits(10.0)]]
        );
        assert_eq!(calls_to(&e, SPEED_TREE_COMPUTE), vec![vec![tree, 0, 5, 1]]);
        assert_eq!(e.get(this, BSTreeModel::iSeed), 42);
        assert_eq!(e.get(this, BSTreeModel::fTrunkHeight), bits(120.0));
        assert_eq!(e.get(this, BSTreeModel::fTrunkWidth), bits(9.0));
        assert_eq!(
            calls_to(&e, slot(MODEL_SLOT_18)),
            vec![vec![this.addr(), form.addr()]]
        );
        let (mut e, this, form) = base_engine(false);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0066_ac40, &args![this, form, 5u32]).bool());
        assert_eq!(calls_to(&e, SPEED_TREE_DESTRUCTOR).len(), 1);
        assert_eq!(e.get(this, BSTreeModel::pSpeedTree), 0);
        assert!(e
            .call(0x0066_ac40, &args![this, Ptr::<()>::new(0), 0u32])
            .bool());
    }
}

/// The scope local `00669500` keeps on its stack: constructor `00b029f0`
/// and destructor `00b02ac0` (`thiscall`, no words).
const SCOPE_00B029F0: u32 = 0x00b0_29f0;
const SCOPE_END_00B02AC0: u32 = 0x00b0_2ac0;
/// `CSpeedTreeRT::GetCamera` (`00b06b70`) and `CSpeedTreeRT::SetCamera`
/// (`00b06c10`) (engine map; `cdecl`: position, direction), and
/// `CSpeedTreeRT::SetLodLevel` (`00b08480`, `thiscall`, one float).
const SPEED_TREE_GET_CAMERA: u32 = 0x00b0_6b70;
const SPEED_TREE_SET_CAMERA_00B06C10: u32 = 0x00b0_6c10;
/// The float the camera direction's second component starts as
/// (`01012054`).
const CAMERA_DIRECTION_Y_01012054: u32 = 0x0101_2054;
/// `TESObjectTREE::CreateBillboardGeomData` (`0051c490`, engine map;
/// `thiscall`, two words).
const CREATE_BILLBOARD_GEOM_DATA: u32 = 0x0051_c490;
/// `NiGeometryData::SetConsistency` (`00a67050`, engine map) and the setters
/// `004410d0` and `004410f0` (`thiscall`, one word each).
const GEOMETRY_SET_CONSISTENCY: u32 = 0x00a6_7050;
const GEOMETRY_SET_004410D0: u32 = 0x0044_10d0;
const GEOMETRY_SET_004410F0: u32 = 0x0044_10f0;
/// `NiTriShape::NiTriShape_ov2` (`00a74480`, engine map; `thiscall`, one
/// word: the data), 0xc4 bytes.
const NI_TRI_SHAPE_CONSTRUCTOR: u32 = 0x00a7_4480;
/// `00ba8630` (`thiscall`, no words): the constructor of the 0x104-byte
/// shader property.
const SHADER_PROPERTY_CONSTRUCTOR_00BA8630: u32 = 0x00ba_8630;
/// The text `BSShaderPPLightingProperty::LoadTextureSet` (`00b66ad0`) gets
/// here (`0102bf34`, `"STBB"`).
const TEXT_STBB: u32 = 0x0102_bf34;
/// `00656db0` (`cdecl`, no words): the word passed to the property's
/// virtual slot `+0x100`.
const PROPERTY_WORD_00656DB0: u32 = 0x0065_6db0;
/// `BSShaderProperty::SetFlag` (`00441130`, engine map; `thiscall`, two
/// words).
const SHADER_PROPERTY_SET_FLAG: u32 = 0x0044_1130;
/// `BSShaderManager::PrepareObject` (`00b57e30`, engine map; `cdecl`, three
/// words).
const SHADER_MANAGER_PREPARE_OBJECT: u32 = 0x00b5_7e30;
/// `004030b0` (`thiscall`, no words): an object of the shape, kept when its
/// `008d8520` (`thiscall`, no words) is 1 to 5.
const SHAPE_OBJECT_004030B0: u32 = 0x0040_30b0;
const OBJECT_COUNT_008D8520: u32 = 0x008d_8520;
/// `0043c4b0` (`cdecl`, no words): the object whose virtual slot `+0xec`
/// takes the shape, and on which `00e74120` (`thiscall`) runs.
const RENDERER_0043C4B0: u32 = 0x0043_c4b0;
const RENDERER_00E74120: u32 = 0x00e7_4120;

// Translated from 00669500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00669500 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The billboard maker `BSTreeModel` calls with its tree form (unnamed in
/// the engine map; `thiscall`, `RET 4`). Inside a scope local
/// (`00b029f0`/`00b02ac0`), with a SpeedTree (`+0xc`) that is not of the
/// second kind (`00666f00`) and a tree: the SpeedTree camera is saved and
/// set to the origin looking along `(0, [01012054], 0)`, the level of
/// detail set to 0, the tree's billboard geometry data made into a stack
/// `NiPointer` (`CreateBillboardGeomData`) and given its consistency
/// `0x4000`, `0x11` and 1; a new `NiTriShape` of it (null when the
/// allocation fails) becomes `spBillboard`, which gets the new alpha
/// property (`CreateAlphaProperty`, when set) and a new 0x104-byte shader
/// property (`00ba8630`; null when the allocation fails) with the
/// billboard texture set (`LoadTextureSet` with `"STBB"`), its slots
/// `+0x100` (0, `00656db0`) and `+0xe4` (0), flags `0xd` and `0x16`;
/// `PrepareObject` on the shape; when the shape's `004030b0` object has a
/// `008d8520` count of 1 to 5, the renderer (`0043c4b0`) gets the shape
/// with that object's `0x6c` pointer (slot `+0xec`) and `00e74120` runs;
/// then the camera is restored and the stack pointers destroyed.
pub fn fn_00669500(e: &mut Engine, this: Ptr<BSTreeModel>, tree: u32) {
    let t = this.addr();
    e.with_stack(0x1a0, |e, frame| {
        // The frame is EBP - 0x1a0.
        let f = frame.addr();
        let scope = f + 0x34;
        let saved_direction = f + 0x1c;
        let direction = f + 0x28;
        let saved_position = f + 0x174;
        let data = f + 0x180;
        let alpha = f + 0x184;
        let position = f + 0x188;
        e.call(SCOPE_00B029F0, &args![scope]);
        for word in 0..3 {
            e.mem.set_u32(position + word * 4, 0.0f32.to_bits());
        }
        e.mem.set_u32(direction, 0.0f32.to_bits());
        let y = x87_quiet_bits(e.mem.u32(CAMERA_DIRECTION_Y_01012054));
        e.mem.set_u32(direction + 4, y);
        e.mem.set_u32(direction + 8, 0.0f32.to_bits());
        if e.get(this, BSTreeModel::pSpeedTree) == 0
            || bs_tree_model_fn_00666f00(e, this)
            || tree == 0
        {
            e.call(SCOPE_END_00B02AC0, &args![scope]);
            return;
        }
        e.call(
            SPEED_TREE_GET_CAMERA,
            &args![saved_position, saved_direction],
        );
        e.call(SPEED_TREE_SET_CAMERA_00B06C10, &args![position, direction]);
        let speed_tree = e.get(this, BSTreeModel::pSpeedTree);
        e.call(
            SPEED_TREE_SET_LOD_LEVEL,
            &args![speed_tree, 0.0f32.to_bits()],
        );
        e.call(NI_POINTER_CONSTRUCT, &args![data, 0u32]);
        e.call(CREATE_BILLBOARD_GEOM_DATA, &args![tree, 0u32, data]);
        let geometry = pointer(e, data);
        e.call(GEOMETRY_SET_CONSISTENCY, &args![geometry, 0x4000u32]);
        let geometry = pointer(e, data);
        e.call(GEOMETRY_SET_004410D0, &args![geometry, 0x11u32]);
        let geometry = pointer(e, data);
        e.call(GEOMETRY_SET_004410F0, &args![geometry, 1u32]);
        let block = e.call(NI_NEW, &args![0xc4u32]).u32();
        let shape = if block != 0 {
            let geometry = pointer(e, data);
            e.call(NI_TRI_SHAPE_CONSTRUCTOR, &args![block, geometry])
                .u32()
        } else {
            0
        };
        e.call(NI_POINTER_ASSIGN, &args![t + 0x1c, shape]);
        let property = bs_tree_model_create_alpha_property(e).addr();
        e.call(NI_POINTER_CONSTRUCT, &args![alpha, property]);
        if pointer(e, alpha) != 0 {
            let property = pointer(e, alpha);
            let shape = pointer(e, t + 0x1c);
            e.call(ATTACH_PROPERTY, &args![shape, property]);
        }
        let block = e.call(NI_NEW, &args![0x104u32]).u32();
        let shader = if block != 0 {
            e.call(SHADER_PROPERTY_CONSTRUCTOR_00BA8630, &args![block])
                .u32()
        } else {
            0
        };
        let shape = pointer(e, t + 0x1c);
        e.call(ATTACH_PROPERTY, &args![shape, shader]);
        let texture_set = pointer(e, t + 0x34);
        e.call(LOAD_TEXTURE_SET, &args![shader, TEXT_STBB, texture_set]);
        let word = e.call(PROPERTY_WORD_00656DB0, &[]).u32();
        e.vcall(shader, 0x100, &args![0u32, word]);
        e.vcall(shader, 0xe4, &args![0u32]);
        e.call(SHADER_PROPERTY_SET_FLAG, &args![shader, 0xdu32, 1u32]);
        e.call(SHADER_PROPERTY_SET_FLAG, &args![shader, 0x16u32, 1u32]);
        let shape = pointer(e, t + 0x1c);
        e.call(SHADER_MANAGER_PREPARE_OBJECT, &args![shape, 0u32, 0u32]);
        let shape = pointer(e, t + 0x1c);
        let object = e.call(SHAPE_OBJECT_004030B0, &args![shape]).u32();
        // Kept when its count is 1 to 5 (the count asked again for the
        // upper bound).
        let kept = if object != 0
            && e.call(OBJECT_COUNT_008D8520, &args![object]).i32() >= 1
            && e.call(OBJECT_COUNT_008D8520, &args![object]).i32() <= 5
        {
            object
        } else {
            0
        };
        if kept != 0 {
            let renderer = e.call(RENDERER_0043C4B0, &[]).u32();
            let word = fn_006698d0(e, Ptr::new(kept));
            let shape = pointer(e, t + 0x1c);
            e.vcall(renderer, 0xec, &args![shape, 0u32, 0u32, word]);
            let renderer = e.call(RENDERER_0043C4B0, &[]).u32();
            e.call(RENDERER_00E74120, &args![renderer]);
        }
        e.call(
            SPEED_TREE_SET_CAMERA_00B06C10,
            &args![saved_position, saved_direction],
        );
        e.call(NI_POINTER_DESTRUCT, &args![alpha]);
        e.call(NI_POINTER_DESTRUCT, &args![data]);
        e.call(SCOPE_END_00B02AC0, &args![scope]);
    });
}

/// This unit's n086 translations, by exe address.
fn funcs_n086() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0066_9500, fn_00669500(Ptr<BSTreeModel>, u32))]
}

#[cfg(test)]
mod n086_tests {
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

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    fn setup(e: &mut Engine) -> (u32, u32) {
        quiet(
            e,
            &[
                SCOPE_00B029F0,
                SCOPE_END_00B02AC0,
                SPEED_TREE_GET_CAMERA,
                SPEED_TREE_SET_CAMERA_00B06C10,
                SPEED_TREE_SET_LOD_LEVEL,
                CREATE_BILLBOARD_GEOM_DATA,
                GEOMETRY_SET_CONSISTENCY,
                GEOMETRY_SET_004410D0,
                GEOMETRY_SET_004410F0,
                ATTACH_PROPERTY,
                LOAD_TEXTURE_SET,
                SHADER_PROPERTY_SET_FLAG,
                SHADER_MANAGER_PREPARE_OBJECT,
                RENDERER_00E74120,
                NI_POINTER_DESTRUCT,
                ALPHA_PROPERTY_CONSTRUCTOR,
                ALPHA_SET_0049ED90,
                ALPHA_SET_004393C0,
                ALPHA_SET_004393E0,
                ALPHA_SET_0094DB80,
                0x7300_0100,
                0x7300_00e4,
                0x7300_00ec,
            ],
        );
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.map(CAMERA_DIRECTION_Y_01012054, 4);
        e.mem.set_f32(CAMERA_DIRECTION_Y_01012054, -1.0);
        // A shader property with the slots the code calls.
        let shader = e.mem.alloc(0x104);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(shader, vtable);
        e.mem.set_u32(vtable + 0x100, 0x7300_0100);
        e.mem.set_u32(vtable + 0xe4, 0x7300_00e4);
        e.register_double(SHADER_PROPERTY_CONSTRUCTOR_00BA8630, move |_, _| {
            shader.into_ret()
        });
        e.register(NI_NEW, |_, a| (0x0090_0000 + a[0]).into_ret());
        e.register(ALPHA_PROPERTY_CONSTRUCTOR, |_, _| 0x0aaau32.into_ret());
        e.register(NI_TRI_SHAPE_CONSTRUCTOR, |_, _| 0x0555u32.into_ret());
        e.register(PROPERTY_WORD_00656DB0, |_, _| 0x66u32.into_ret());
        let model = e.mem.alloc(0x80);
        e.mem.set_u32(model + 0x0c, 0x0fff);
        e.mem.set_u32(model + 0x08, 0);
        e.mem.set_u32(model + 0x34, 0x0333);
        (model, shader)
    }

    #[test]
    fn billboard_needs_a_speed_tree_and_a_tree() {
        let mut e = Engine::new();
        let (model, _) = setup(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0066_9500, &args![model, 0u32]);
        assert_eq!(calls(&e, SCOPE_00B029F0).len(), 1);
        assert_eq!(calls(&e, SCOPE_END_00B02AC0).len(), 1);
        assert!(calls(&e, SPEED_TREE_GET_CAMERA).is_empty());
        e.mem.set_u32(model + 0x0c, 0);
        e.call_log = Some(vec![]);
        e.call(0x0066_9500, &args![model, 0x0777u32]);
        assert!(calls(&e, SPEED_TREE_GET_CAMERA).is_empty());
    }

    #[test]
    fn billboard_is_made_and_dressed() {
        let mut e = Engine::new();
        let (model, shader) = setup(&mut e);
        let renderer = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(renderer, vtable);
        e.mem.set_u32(vtable + 0xec, 0x7300_00ec);
        e.register_double(RENDERER_0043C4B0, move |_, _| renderer.into_ret());
        let object = e.mem.alloc(0x70);
        e.mem.set_u32(object + 0x6c, 0x0bbb);
        e.register_double(SHAPE_OBJECT_004030B0, move |_, _| object.into_ret());
        e.register(OBJECT_COUNT_008D8520, |_, _| 3u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0066_9500, &args![model, 0x0777u32]);
        let saved = calls(&e, SPEED_TREE_GET_CAMERA);
        assert_eq!(saved.len(), 1);
        let set = calls(&e, SPEED_TREE_SET_CAMERA_00B06C10);
        assert_eq!(set.len(), 2);
        assert_eq!(set[1], saved[0]);
        let direction = set[0][1];
        assert_eq!(e.mem.f32(direction + 4), -1.0);
        assert_eq!(calls(&e, SPEED_TREE_SET_LOD_LEVEL), vec![vec![0x0fff, 0]]);
        let geometry = calls(&e, CREATE_BILLBOARD_GEOM_DATA);
        assert_eq!(geometry[0][..2], [0x0777, 0]);
        assert_eq!(e.mem.u32(model + 0x1c), 0x0555);
        assert_eq!(
            calls(&e, ATTACH_PROPERTY),
            vec![vec![0x0555, 0x0aaa], vec![0x0555, shader]]
        );
        assert_eq!(
            calls(&e, LOAD_TEXTURE_SET),
            vec![vec![shader, TEXT_STBB, 0x0333]]
        );
        assert_eq!(calls(&e, 0x7300_0100), vec![vec![shader, 0, 0x66]]);
        assert_eq!(calls(&e, 0x7300_00e4), vec![vec![shader, 0]]);
        assert_eq!(
            calls(&e, SHADER_PROPERTY_SET_FLAG),
            vec![vec![shader, 0xd, 1], vec![shader, 0x16, 1]]
        );
        assert_eq!(
            calls(&e, SHADER_MANAGER_PREPARE_OBJECT),
            vec![vec![0x0555, 0, 0]]
        );
        assert_eq!(
            calls(&e, 0x7300_00ec),
            vec![vec![renderer, 0x0555, 0, 0, 0x0bbb]]
        );
        assert_eq!(calls(&e, RENDERER_00E74120), vec![vec![renderer]]);
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT).len(), 2);

        // A count outside 1..=5: no renderer call.
        e.register(OBJECT_COUNT_008D8520, |_, _| 6u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0066_9500, &args![model, 0x0777u32]);
        assert!(calls(&e, RENDERER_0043C4B0).is_empty());
        e.register(OBJECT_COUNT_008D8520, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0066_9500, &args![model, 0x0777u32]);
        assert_eq!(calls(&e, OBJECT_COUNT_008D8520).len(), 1);
        assert!(calls(&e, RENDERER_0043C4B0).is_empty());
    }
}
