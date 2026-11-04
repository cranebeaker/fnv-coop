//! `fallout shared/tempeffects/bstempeffectparticle.cpp` (Xbox PDB source unit), subsystem `fallout shared/tempeffects`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00688f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectParticle` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00689720` and frees 0x74 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_temp_effect_particle_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0068_9720, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x74u32]);
    }
    this
}

// Translated from 00688f80 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d6b84 without reading the receiver. The map names this body `BSTempEffectParticle::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00688f80(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_6b84
}

// Translated from 0068a810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bstempeffectparticle.cpp`). `__fastcall` getter: the dword at
/// `+0x4c` of `this`.
pub fn fn_0068a810(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x4c))
}

/// `BSTempEffectParticle`'s vtable.
const VTABLE: u32 = 0x0106_9c5c;
/// `BSTempEffect`'s constructor (`006827b0`, thiscall: cell, lifetime).
const BS_TEMP_EFFECT_CONSTRUCT: u32 = 0x0068_27b0;
/// `BSTempEffect`'s destructor body (`00682860`).
const BS_TEMP_EFFECT_DESTROY: u32 = 0x0068_2860;
/// `BSTempEffect::Update` (`006828a0`, thiscall: one `float`).
const BS_TEMP_EFFECT_UPDATE: u32 = 0x0068_28a0;
/// `00559450` (thiscall on an `NiPointer`, returns the object; unnamed).
const POINTER_GET: u32 = 0x0055_9450;
/// `0066b0d0` (thiscall on an `NiPointer`, one word: assigns it; unnamed).
const POINTER_SET: u32 = 0x0066_b0d0;
/// `00633c90` (thiscall on an `NiPointer`, one word: constructs it; unnamed).
const POINTER_INIT: u32 = 0x0063_3c90;
/// `0045cec0` (thiscall on an `NiPointer`: destroys it; unnamed).
const POINTER_DESTROY: u32 = 0x0045_cec0;
/// `006e5cc0` (thiscall on an `NiPointer`, one word: assigns it from another
/// `NiPointer`; the map's name is a template instance's).
const POINTER_ASSIGN_POINTER: u32 = 0x006e_5cc0;
/// `00476a80` (thiscall: constructs an `NiTransform`; unnamed).
const TRANSFORM_INIT: u32 = 0x0047_6a80;
/// `NiTransform::MakeIdentity` (`00a86bf0`).
const NI_TRANSFORM_MAKE_IDENTITY: u32 = 0x00a8_6bf0;
/// `NiTransform::InvertNonUniform` (`004b4880`, thiscall: result).
const NI_TRANSFORM_INVERT_NON_UNIFORM: u32 = 0x004b_4880;
/// `0062c250` (thiscall on an `NiTransform`: result, other; their product).
const TRANSFORM_MULTIPLY_0062C250: u32 = 0x0062_c250;
/// `0041a250` (thiscall: constructs a sound handle; unnamed).
const SOUND_HANDLE_INIT: u32 = 0x0041_a250;
/// `00483710` (thiscall: the sound handle's destructor here; unnamed).
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;
/// `BSSoundHandle::IsValid`, `Stop`, `Release`, `SetPosition`.
const BS_SOUND_HANDLE_IS_VALID: u32 = 0x00ad_8ce0;
const BS_SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
const BS_SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
const BS_SOUND_HANDLE_SET_POSITION: u32 = 0x00ad_8b60;
/// `00aa13e0` (cdecl: size; the Ni allocator).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `004a4240` (no arguments, returns a `float` in `ST0`; unnamed).
const REAL_004A4240: u32 = 0x004a_4240;
/// `006815c0` (thiscall: constructs a local matrix here; unnamed).
const MATRIX_INIT_006815C0: u32 = 0x0068_15c0;
/// `004a0c90` (thiscall on that matrix, one `float`; unnamed).
const MATRIX_004A0C90: u32 = 0x004a_0c90;
/// `BSUtilities::GetRotationZToVectorTranspose` (`00c4b8a0`, cdecl: result, vector).
const GET_ROTATION_Z_TO_VECTOR_TRANSPOSE: u32 = 0x00c4_b8a0;
/// `NiMatrix3::operator*` (`0043f8d0`, thiscall: result, other).
const NI_MATRIX3_MULTIPLY: u32 = 0x0043_f8d0;
/// `00456fc0` (thiscall on the cell, one word; unnamed).
const CELL_NODE_00456FC0: u32 = 0x0045_6fc0;
/// `ProcessLists::AddTempEffect` (`00973fd0`, thiscall on `011e0e80`).
const PROCESS_LISTS_ADD_TEMP_EFFECT: u32 = 0x0097_3fd0;
/// The process lists `006890b0` adds the effect to.
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// `ModelLoader::LoadFile` (`00447080`) and `ModelLoader::ReleaseModel` (`00445300`),
/// thiscall on the loader at `011c3b3c`.
const MODEL_LOADER_LOAD_FILE: u32 = 0x0044_7080;
const MODEL_LOADER_RELEASE_MODEL: u32 = 0x0044_5300;
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The cache `TES::GetUnusedCachedParticleObject` (`0045aba0`) is asked on (`011dea10`).
const PARTICLE_CACHE_OWNER: u32 = 0x011d_ea10;
const TES_GET_UNUSED_CACHED_PARTICLE_OBJECT: u32 = 0x0045_aba0;
/// `00726070` (thiscall on an object, returns a word: compared with 1 and 2 as
/// a reference count; unnamed).
const COUNT_00726070: u32 = 0x0072_6070;
/// `004ad050` (thiscall: constructs an `NiCloningProcess` with a `float`).
const CLONING_PROCESS_INIT: u32 = 0x004a_d050;
/// `NiCloningProcess::~NiCloningProcess` (`004ad270`).
const CLONING_PROCESS_DESTROY: u32 = 0x004a_d270;
/// `NiObject::Clone` (`00a5d2c0`, thiscall: cloning process).
const NI_OBJECT_CLONE: u32 = 0x00a5_d2c0;
/// `TESObjectREFR::AddMasterParticleAddonNodes` / `RemoveMasterParticleAddonNodes`
/// (cdecl: the node).
const ADD_MASTER_PARTICLE_ADDON_NODES: u32 = 0x0057_8060;
const REMOVE_MASTER_PARTICLE_ADDON_NODES: u32 = 0x0057_8170;
/// `00440460` (translate), `0043fa80` (rotate), `00440490` (scale): thiscall on an
/// `NiAVObject`, one word each.
const SET_TRANSLATE: u32 = 0x0044_0460;
const SET_ROTATE: u32 = 0x0043_fa80;
const SET_SCALE: u32 = 0x0044_0490;
/// `008c7aa0` (no arguments, returns `AL`; unnamed).
const TEST_008C7AA0: u32 = 0x008c_7aa0;
/// `004537b0` (no arguments, returns the task queue; unnamed).
const TASK_QUEUE_004537B0: u32 = 0x0045_37b0;
/// `TaskQueueInterface::QueueNodeAttach` (`0087ac00`: node, parent, two words).
const TASK_QUEUE_QUEUE_NODE_ATTACH: u32 = 0x0087_ac00;
/// `NiAVObject::UpdateProperties` (`00a5a040`).
const NI_AV_OBJECT_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `NiTimeController::StartAnimations` (`00a6d2d0`, cdecl).
const NI_TIME_CONTROLLER_START_ANIMATIONS: u32 = 0x00a6_d2d0;
/// `00461130` (thiscall on an `NiAVObject`, returns its world transform; unnamed).
const WORLD_TRANSFORM_00461130: u32 = 0x0046_1130;
/// `009611e0` (thiscall on an `NiAVObject`, returns its parent; unnamed).
const PARENT_009611E0: u32 = 0x0096_11e0;
/// `0043b230` (first controller), `004a8a90` (next controller): thiscall; unnamed.
const FIRST_CONTROLLER: u32 = 0x0043_b230;
const NEXT_CONTROLLER: u32 = 0x004a_8a90;
/// `004a7bd0` and `0047c860` (thiscall on a controller, `float` in `ST0`; the map's
/// names come from folded getters): the end and start of its time range by their use.
const CONTROLLER_END_004A7BD0: u32 = 0x004a_7bd0;
const CONTROLLER_START_0047C860: u32 = 0x0047_c860;
/// `00404010` (cdecl, two `float`s, returns `ST0`; unnamed): the larger by its use.
const FLOAT_MAX_00404010: u32 = 0x0040_4010;
/// `00453470` (child count), `0043b4a0` (child at an index): thiscall on a node.
const CHILD_COUNT: u32 = 0x0045_3470;
const CHILD_AT: u32 = 0x0043_b4a0;
/// `0068abb0` (thiscall: constructs a local array: two words), `00658930` (its size),
/// `00877a30` (address of an element), `0068ab60` (destroys it); `0068a8f0` fills it.
const ARRAY_INIT_0068ABB0: u32 = 0x0068_abb0;
const ARRAY_SIZE_00658930: u32 = 0x0065_8930;
const ARRAY_AT_00877A30: u32 = 0x0087_7a30;
const ARRAY_DESTROY_0068AB60: u32 = 0x0068_ab60;
const COLLECT_0068A8F0: u32 = 0x0068_a8f0;
/// `NiMatrix3::Inverse` (`004b45b0`, thiscall: result).
const NI_MATRIX3_INVERSE: u32 = 0x004b_45b0;
/// `NiMatrix3::MakeYRotation` (`0043f850`, thiscall: angle).
const NI_MATRIX3_MAKE_Y_ROTATION: u32 = 0x0043_f850;
/// `-1.0` (`float`).
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// `-1.0` (`double`).
const MINUS_ONE: u32 = 0x0101_a6b0;
/// `0.1` (`double`, the `float` 0.1 widened).
const MINIMUM_LIFETIME: u32 = 0x0101_ffa0;
/// `0.1` (`float`).
const MINIMUM_LIFETIME_FLOAT: u32 = 0x0101_e2bc;
/// `3.1415927` (`float`).
const PI_FLOAT: u32 = 0x0102_b3c8;

// Translated from 00688fc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00688fc0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSTempEffectParticle` constructor (unnamed in the map; `RET 0x10`): the
/// `BSTempEffect` base with `cell` and `lifetime`, the vtable, the particle object and
/// follow object pointers (+0x18, +0x20), the follow offset transform (+0x24, made the
/// identity), both sound handles (+0x5c, +0x68); the follow object becomes `follow`, the
/// file name (+0x1c) `filename`, the decal impact data (+0x58) null. Returns `this`.
pub fn fn_00688fc0(
    e: &mut Engine,
    this: Ptr,
    cell: u32,
    lifetime: f32,
    filename: u32,
    follow: u32,
) -> Ptr {
    let base = this.addr();
    e.call(
        BS_TEMP_EFFECT_CONSTRUCT,
        &args![this, cell, x87_quiet_bits(lifetime.to_bits())],
    );
    e.mem.set_u32(base, VTABLE);
    e.call(POINTER_INIT, &args![base + 0x18, 0u32]);
    e.call(POINTER_INIT, &args![base + 0x20, 0u32]);
    e.call(TRANSFORM_INIT, &args![base + 0x24]);
    e.call(SOUND_HANDLE_INIT, &args![base + 0x5c]);
    e.call(SOUND_HANDLE_INIT, &args![base + 0x68]);
    e.call(POINTER_SET, &args![base + 0x18, 0u32]);
    e.call(POINTER_SET, &args![base + 0x20, follow]);
    e.call(NI_TRANSFORM_MAKE_IDENTITY, &args![base + 0x24]);
    e.mem.set_u32(base + 0x1c, filename);
    e.mem.set_u32(base + 0x58, 0);
    this
}

// Translated from 006890b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006890b0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed (cdecl, twelve words): makes a particle effect in `cell` (nothing without a
/// cell): a new 0x74-byte `BSTempEffectParticle` (`fn_00688fc0` with `lifetime`,
/// `filename`, `follow`; null when the allocation fails), initialized (`Init`) under the
/// cell's node 4 (`00456fc0`) at `position`, rotated to face `direction` (the rotation
/// `GetRotationZToVectorTranspose` gives, times a matrix made from `004a4240`), with
/// `scale` and `flags`, then added to the process lists. Returns the effect.
pub fn fn_006890b0(
    e: &mut Engine,
    cell: u32,
    lifetime: f32,
    filename: u32,
    direction_x: u32,
    direction_y: u32,
    direction_z: u32,
    position_x: u32,
    position_y: u32,
    position_z: u32,
    scale: f32,
    flags: u32,
    follow: u32,
) -> u32 {
    if cell == 0 {
        return 0;
    }
    let block = e.call(NI_ALLOCATE, &args![0x74u32]).u32();
    let effect = if block == 0 {
        0
    } else {
        fn_00688fc0(e, Ptr::new(block), cell, lifetime, filename, follow).addr()
    };
    // The frame below `EBP`: `local(n)` is `EBP - n`.
    e.with_stack(0x90, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x90 - offset;
        let angle = e.call(REAL_004A4240, &[]).f64();
        let angle = x87_store32(angle);
        let spin = local(0x58);
        e.call(MATRIX_INIT_006815C0, &args![spin]);
        e.call(MATRIX_004A0C90, &args![spin, x87_quiet_bits(angle)]);
        let facing = e
            .call(
                GET_ROTATION_Z_TO_VECTOR_TRANSPOSE,
                &args![local(0x8c), direction_x, direction_y, direction_z],
            )
            .u32();
        let rotation = local(0x34);
        e.call(NI_MATRIX3_MULTIPLY, &args![facing, rotation, spin]);
        let node = e.call(CELL_NODE_00456FC0, &args![cell, 4u32]).u32();
        let r: Vec<u32> = (0..9).map(|i| e.mem.u32(rotation + 4 * i)).collect();
        bs_temp_effect_particle_init(
            e,
            Ptr::new(effect),
            node,
            r[0],
            r[1],
            r[2],
            r[3],
            r[4],
            r[5],
            r[6],
            r[7],
            r[8],
            position_x,
            position_y,
            position_z,
            f32::from_bits(x87_quiet_bits(scale.to_bits())),
            flags,
        );
    });
    e.call(PROCESS_LISTS_ADD_TEMP_EFFECT, &args![PROCESS_LISTS, effect]);
    effect
}

// Translated from 00689310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00689310 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSTempEffectParticle::Init` (Xbox PDB, `RET 0x3c`): loads the effect's model
/// (`ModelLoader::LoadFile`; nothing more without it). With flag 1 and the cache at
/// `011dea10` it takes an unused cached particle object; otherwise it uses the loaded
/// object while its count (`00726070`) is at most 2, or a clone of it with flag 2. With
/// an object: flag 4 sets the lifetime to the longest controller range (`fn_00689840`,
/// at least 0.1, unless it is -1); master particle add-on nodes are added for its slot
/// +0xc; it gets `position`, `rotation` and `scale`, is attached to `parent` (through the
/// task queue when `008c7aa0` says so), gets its emitters turned (`fn_0068aa50`) and its
/// animations started; with a follow object the follow offset becomes the follow
/// object's inverse world transform times the effect's.
pub fn bs_temp_effect_particle_init(
    e: &mut Engine,
    this: Ptr,
    parent: u32,
    rotation_0: u32,
    rotation_1: u32,
    rotation_2: u32,
    rotation_3: u32,
    rotation_4: u32,
    rotation_5: u32,
    rotation_6: u32,
    rotation_7: u32,
    rotation_8: u32,
    position_x: u32,
    position_y: u32,
    position_z: u32,
    scale: f32,
    flags: u32,
) {
    let base = this.addr();
    // The argument words as the exe has them on its stack (`EBP + 8` onwards), whose
    // addresses it passes on, and the frame below `EBP`.
    e.with_stack(0x150, |e, frame| {
        let argument = |offset: u32| frame.addr() + 0x110 + offset - 8;
        let local = |offset: u32| frame.addr() + 0x110 - offset;
        let words = [
            parent,
            rotation_0,
            rotation_1,
            rotation_2,
            rotation_3,
            rotation_4,
            rotation_5,
            rotation_6,
            rotation_7,
            rotation_8,
            position_x,
            position_y,
            position_z,
            scale.to_bits(),
            flags,
        ];
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(argument(8 + 4 * i as u32), *word);
        }
        let rotation = argument(0xc);
        let position = argument(0x30);
        let filename = e.mem.u32(base + 0x1c);
        let loader = e.global::<u32>(MODEL_LOADER);
        let model = e
            .call(
                MODEL_LOADER_LOAD_FILE,
                &args![loader, filename, 0u32, 1u32, 0u32, 0u32, 1u32],
            )
            .u32();
        let loaded = local(0x10);
        e.call(POINTER_INIT, &args![loaded, model]);
        if e.call(POINTER_GET, &args![loaded]).u32() == 0 {
            e.call(POINTER_DESTROY, &args![loaded]);
            return;
        }
        e.call(POINTER_SET, &args![base + 0x18, 0u32]);
        let mut cached = 0;
        let cache = e.global::<u32>(PARTICLE_CACHE_OWNER);
        if flags & 1 != 0 && cache != 0 {
            let object = e.call(POINTER_GET, &args![loaded]).u32();
            cached = e
                .call(
                    TES_GET_UNUSED_CACHED_PARTICLE_OBJECT,
                    &args![cache, object, base + 0x18],
                )
                .u32();
        }
        if cached == 0 {
            let object = e.call(POINTER_GET, &args![loaded]).u32();
            if e.call(COUNT_00726070, &args![object]).u32() <= 2 {
                e.call(POINTER_ASSIGN_POINTER, &args![base + 0x18, loaded]);
            } else if flags & 2 != 0 {
                let cloning = local(0x34);
                e.call(CLONING_PROCESS_INIT, &args![cloning, x87_store32(1.0)]);
                let object = e.call(POINTER_GET, &args![loaded]).u32();
                let clone = e.call(NI_OBJECT_CLONE, &args![object, cloning]).u32();
                e.call(POINTER_SET, &args![base + 0x18, clone]);
                e.call(CLONING_PROCESS_DESTROY, &args![cloning]);
            }
        }
        if e.call(POINTER_GET, &args![base + 0x18]).u32() == 0 {
            e.call(POINTER_DESTROY, &args![loaded]);
            return;
        }
        if flags & 4 != 0 {
            let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
            let longest = fn_00689840(e, this, object);
            let mut lifetime = x87_store32(f64::from(longest));
            let minus_one: f64 = e.global(MINUS_ONE);
            // FCOMP; TEST AH,0x44; JNP: kept unless equal.
            if x87_load32(lifetime) != minus_one {
                let minimum: f64 = e.global(MINIMUM_LIFETIME);
                // FCOMP; TEST AH,5; JP: raised only when below.
                if x87_load32(lifetime) < minimum {
                    lifetime = x87_quiet_bits(e.global::<u32>(MINIMUM_LIFETIME_FLOAT));
                }
                e.mem.set_u32(base + 8, x87_quiet_bits(lifetime));
            }
        }
        let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
        let node = e.vcall(object, 0xc, &[]).u32();
        if node != 0 {
            e.call(ADD_MASTER_PARTICLE_ADDON_NODES, &args![node]);
        }
        let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
        e.call(SET_TRANSLATE, &args![object, position]);
        let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
        e.call(SET_ROTATE, &args![object, rotation]);
        let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
        e.call(SET_SCALE, &args![object, x87_quiet_bits(scale.to_bits())]);
        if e.call(TEST_008C7AA0, &[]).bool() {
            let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
            let queue = e.call(TASK_QUEUE_004537B0, &[]).u32();
            e.call(
                TASK_QUEUE_QUEUE_NODE_ATTACH,
                &args![queue, object, parent, 1u32, 1u32],
            );
        } else {
            let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
            e.vcall(parent, 0xdc, &args![object, 1u32]);
            let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
            e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![object]);
        }
        let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
        fn_0068aa50(e, this, object);
        let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
        e.call(NI_TIME_CONTROLLER_START_ANIMATIONS, &args![object]);
        if e.call(POINTER_GET, &args![base + 0x20]).u32() != 0 {
            let transform = local(0xa0);
            e.call(TRANSFORM_INIT, &args![transform]);
            e.mem
                .set_u32(transform + 0x30, x87_quiet_bits(scale.to_bits()));
            for i in 0..9 {
                let word = e.mem.u32(rotation + 4 * i);
                e.mem.set_u32(transform + 4 * i, word);
            }
            for i in 0..3 {
                let word = e.mem.u32(position + 4 * i);
                e.mem.set_u32(transform + 0x24 + 4 * i, word);
            }
            let follow = e.call(POINTER_GET, &args![base + 0x20]).u32();
            let world = e.call(WORLD_TRANSFORM_00461130, &args![follow]).u32();
            let follow_world = local(0xd4);
            for i in 0..13 {
                let word = e.mem.u32(world + 4 * i);
                e.mem.set_u32(follow_world + 4 * i, word);
            }
            let inverse = local(0x6c);
            e.call(TRANSFORM_INIT, &args![inverse]);
            e.call(
                NI_TRANSFORM_INVERT_NON_UNIFORM,
                &args![follow_world, inverse],
            );
            let product = e
                .call(
                    TRANSFORM_MULTIPLY_0062C250,
                    &args![inverse, local(0x108), transform],
                )
                .u32();
            for i in 0..13 {
                let word = e.mem.u32(product + 4 * i);
                e.mem.set_u32(base + 0x24 + 4 * i, word);
            }
        }
        e.call(POINTER_DESTROY, &args![loaded]);
    });
}

// Translated from 006896c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectParticle::Detach` (Xbox PDB): removes the master particle add-on nodes
/// of the particle object's slot +0xc node, when there is one.
pub fn bs_temp_effect_particle_detach(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let node = if e.call(POINTER_GET, &args![base + 0x18]).u32() == 0 {
        0
    } else {
        let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
        e.vcall(object, 0xc, &[]).u32()
    };
    if node != 0 {
        e.call(REMOVE_MASTER_PARTICLE_ADDON_NODES, &args![node]);
    }
}

// Translated from 00689720 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00689720 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSTempEffectParticle::~BSTempEffectParticle` (Xbox PDB): stops the sounds, releases
/// the model (`ModelLoader::ReleaseModel(filename, 0, 1)`) when there is a file name,
/// detaches the particle object from its parent (slot +0xe8), clears it, then destroys
/// the sound handles, both pointers and the `BSTempEffect` base.
pub fn bs_temp_effect_particle_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    bs_temp_effect_particle_stop_sounds(e, this);
    let filename = e.mem.u32(base + 0x1c);
    if filename != 0 {
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(
            MODEL_LOADER_RELEASE_MODEL,
            &args![loader, filename, 0u32, 1u32],
        );
    }
    if e.call(POINTER_GET, &args![base + 0x18]).u32() != 0 {
        let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
        if e.call(PARENT_009611E0, &args![object]).u32() != 0 {
            let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
            let parent = e.call(PARENT_009611E0, &args![object]).u32();
            let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
            e.vcall(parent, 0xe8, &args![object]);
        }
    }
    e.call(POINTER_SET, &args![base + 0x18, 0u32]);
    e.call(SOUND_HANDLE_DESTROY, &args![base + 0x68]);
    e.call(SOUND_HANDLE_DESTROY, &args![base + 0x5c]);
    e.call(POINTER_DESTROY, &args![base + 0x20]);
    e.call(POINTER_DESTROY, &args![base + 0x18]);
    e.call(BS_TEMP_EFFECT_DESTROY, &args![this]);
}

// Translated from 00689840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`RET 4`, returns `ST0`): the longest time range of `object`'s controllers
/// and, through its slot +0xc node, of its children's (recursively): starting from -1,
/// each controller's `004a7bd0 - 0047c860` and each child's result go through
/// `00404010` with the running value.
pub fn fn_00689840(e: &mut Engine, this: Ptr, object: u32) -> f32 {
    let mut longest = x87_quiet_bits(e.global::<u32>(MINUS_ONE_FLOAT));
    let mut controller = e.call(FIRST_CONTROLLER, &args![object]).u32();
    while controller != 0 {
        let end = e.call(CONTROLLER_END_004A7BD0, &args![controller]).f64();
        let start = e.call(CONTROLLER_START_0047C860, &args![controller]).f64();
        let range = x87_store32(x87_sub(end, start));
        let larger = e
            .call(
                FLOAT_MAX_00404010,
                &args![x87_quiet_bits(longest), x87_quiet_bits(range)],
            )
            .f64();
        longest = x87_store32(larger);
        controller = e.call(NEXT_CONTROLLER, &args![controller]).u32();
    }
    let node = e.vcall(object, 0xc, &[]).u32();
    if node != 0 {
        let count = e.call(CHILD_COUNT, &args![node]).i32();
        let mut i = 0i32;
        while i < count {
            let child = e.call(CHILD_AT, &args![node, i]).u32();
            if child != 0 {
                let range = fn_00689840(e, this, child);
                let larger = e
                    .call(
                        FLOAT_MAX_00404010,
                        &args![x87_quiet_bits(longest), x87_store32(f64::from(range))],
                    )
                    .f64();
                longest = x87_store32(larger);
            }
            i += 1;
        }
    }
    f32::from_bits(x87_quiet_bits(longest))
}

// Translated from 00689a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectParticle::Update` (Xbox PDB): the `BSTempEffect` update with `time`;
/// drops the follow object once only this effect holds it (count 1); with a particle
/// object and a follow object, places the particle object at the follow object's world
/// transform times the follow offset. Returns the base update's answer.
pub fn bs_temp_effect_particle_update(e: &mut Engine, this: Ptr, time: f32) -> bool {
    let base = this.addr();
    let answer = e
        .call(
            BS_TEMP_EFFECT_UPDATE,
            &args![this, x87_quiet_bits(time.to_bits())],
        )
        .u8();
    if e.call(POINTER_GET, &args![base + 0x20]).u32() != 0 {
        let follow = e.call(POINTER_GET, &args![base + 0x20]).u32();
        if e.call(COUNT_00726070, &args![follow]).u32() == 1 {
            e.call(POINTER_SET, &args![base + 0x20, 0u32]);
        }
    }
    if e.call(POINTER_GET, &args![base + 0x18]).u32() != 0
        && e.call(POINTER_GET, &args![base + 0x20]).u32() != 0
    {
        e.with_stack(0x38, |e, transform| {
            let transform = transform.addr();
            let follow = e.call(POINTER_GET, &args![base + 0x20]).u32();
            let world = e.call(WORLD_TRANSFORM_00461130, &args![follow]).u32();
            e.call(
                TRANSFORM_MULTIPLY_0062C250,
                &args![world, transform, base + 0x24],
            );
            let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
            e.call(SET_TRANSLATE, &args![object, transform + 0x24]);
            let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
            e.call(SET_ROTATE, &args![object, transform]);
            let scale = x87_quiet_bits(e.mem.u32(transform + 0x30));
            let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
            e.call(SET_SCALE, &args![object, scale]);
        });
    }
    answer != 0
}

// Translated from 0068a7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (thiscall on a `BSSoundHandle`, `RET 4`): `SetPosition` with the three
/// `float`s at `position` (loaded and pushed, so a signalling NaN arrives quiet).
pub fn fn_0068a7d0(e: &mut Engine, this: Ptr, position: u32) {
    let x = x87_quiet_bits(e.mem.u32(position));
    let y = x87_quiet_bits(e.mem.u32(position + 4));
    let z = x87_quiet_bits(e.mem.u32(position + 8));
    e.call(BS_SOUND_HANDLE_SET_POSITION, &args![this, x, y, z]);
}

// Translated from 0068a830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed getter: the word at `this + 0x50`.
pub fn fn_0068a830(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x50)
}

// Translated from 0068a850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectParticle::StopSounds` (Xbox PDB): stops each valid sound (`Sound1` at
/// +0x5c, `Sound2` at +0x68) and releases it when it is still valid afterwards.
pub fn bs_temp_effect_particle_stop_sounds(e: &mut Engine, this: Ptr) {
    for offset in [0x5cu32, 0x68] {
        let sound = this.addr() + offset;
        if e.call(BS_SOUND_HANDLE_IS_VALID, &args![sound]).bool() {
            e.call(BS_SOUND_HANDLE_STOP, &args![sound]);
            if e.call(BS_SOUND_HANDLE_IS_VALID, &args![sound]).bool() {
                e.call(BS_SOUND_HANDLE_RELEASE, &args![sound]);
            }
        }
    }
}

// Translated from 0068aa50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0068aa50 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed (`RET 4`): collects objects of `node` into a local array (`0068a8f0`; array
/// made with 10 and 1) and turns each one: its rotation becomes the inverse of its
/// parent's world rotation times a half turn about Y (`MakeYRotation(pi)`).
pub fn fn_0068aa50(e: &mut Engine, this: Ptr, node: u32) {
    // The frame below `EBP`: `local(n)` is `EBP - n`.
    e.with_stack(0x98, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x98 - offset;
        let array = local(0x1c);
        e.call(ARRAY_INIT_0068ABB0, &args![array, 10u32, 1u32]);
        let count = local(0x20);
        e.mem.set_u32(count, 0);
        e.call(COLLECT_0068A8F0, &args![this, node, array, count]);
        let mut i = 0u32;
        while i < e.call(ARRAY_SIZE_00658930, &args![array]).u32() {
            let slot = e.call(ARRAY_AT_00877A30, &args![array, i]).u32();
            let item = e.mem.u32(slot);
            if item != 0 {
                let inverse = local(0x4c);
                let parent = e.call(PARENT_009611E0, &args![item]).u32();
                let world = e.call(WORLD_TRANSFORM_00461130, &args![parent]).u32();
                e.call(NI_MATRIX3_INVERSE, &args![world, inverse]);
                let turn = local(0x70);
                e.call(MATRIX_INIT_006815C0, &args![turn]);
                let pi = x87_quiet_bits(e.global::<u32>(PI_FLOAT));
                e.call(NI_MATRIX3_MAKE_Y_ROTATION, &args![turn, pi]);
                let product = e
                    .call(NI_MATRIX3_MULTIPLY, &args![inverse, local(0x94), turn])
                    .u32();
                e.call(SET_ROTATE, &args![item, product]);
            }
            i = i.wrapping_add(1);
        }
        e.call(ARRAY_DESTROY_0068AB60, &args![array]);
    });
}

use crate::units::fallout_shared_tempeffects::bstempeffectgeometrydecal::{
    buffer_position, load_bad_header, load_block_end, load_bytes, report_flag, save_block_begin,
    save_block_end, save_bytes, save_report, uses_blocks, BLOCK_TAG, SAVE_REPORT, SAVE_REPORT_FORM,
    SIZE_REPORT, SIZE_REPORT_FORM,
};

/// The source file name the save reports give (`...\BSTempEffectParticle.cpp`).
const PARTICLE_SOURCE_FILE: u32 = 0x0106_9d20;
/// The controller manager lookup (`00653270`, cdecl: the RTTI at `011f36ac`
/// and the first controller): the particle object's controller manager.
const FIND_CONTROLLER_00653270: u32 = 0x0065_3270;
const CONTROLLER_MANAGER_RTTI: u32 = 0x011f_36ac;
/// `00438170` (`RET 4`) and `004381b0`: a fixed-string local's
/// constructor and destructor.
const FIXED_STRING_INIT_00438170: u32 = 0x0043_8170;
const FIXED_STRING_DESTROY_004381B0: u32 = 0x0043_81b0;
/// `0047a520` (`RET 4`): the manager's sequence of a name.
const MANAGER_SEQUENCE_0047A520: u32 = 0x0047_a520;
/// `0047aab0` (`RET 0x18`): activates a sequence (sequence, 0, 0, 1.0, 0.0, 0).
const MANAGER_ACTIVATE_0047AAB0: u32 = 0x0047_aab0;
/// `00508100` (ST0): the sequence's length.
const SEQUENCE_LENGTH_00508100: u32 = 0x0050_8100;
/// `0044a670` (cdecl): the file name's length.
const STRING_LENGTH_0044A670: u32 = 0x0044_a670;
/// `0055fdb0` (cdecl, the manager; `u16`): the manager's saved sequence count.
const MANAGER_SAVE_COUNT_0055FDB0: u32 = 0x0055_fdb0;
/// `0055fe70` (cdecl: the manager, a `float`): saves the manager's state.
const MANAGER_SAVE_0055FE70: u32 = 0x0055_fe70;
/// `0055ffa0` (cdecl: the manager, the object, a `float`): loads it.
const MANAGER_LOAD_0055FFA0: u32 = 0x0055_ffa0;
/// `006a9540`, `0043c490`, `009cdae0` (no stack word): the object's
/// rotation, translation and scale (ST0).
const OBJECT_ROTATION_006A9540: u32 = 0x006a_9540;
const OBJECT_TRANSLATION_0043C490: u32 = 0x0043_c490;
const OBJECT_SCALE_009CDAE0: u32 = 0x009c_dae0;
/// `NiQuaternion::FromRotation` and `ToRotation` (engine map; `RET 4`).
const QUATERNION_FROM_ROTATION: u32 = 0x00a6_df40;
const QUATERNION_TO_ROTATION: u32 = 0x004f_0180;
/// `BSTempEffect::SaveGame` / `LoadGame` (`00682920`, `00682990`) and its
/// save size (`006828e0`).
const BS_TEMP_EFFECT_SAVE_GAME: u32 = 0x0068_2920;
const BS_TEMP_EFFECT_LOAD_GAME: u32 = 0x0068_2990;
const BS_TEMP_EFFECT_SAVE_SIZE: u32 = 0x0068_28e0;
/// `00403d30` (cdecl: memset).
const FILL_MEMORY_00403D30: u32 = 0x0040_3d30;
/// `NiObject::Clone_ov2` (`00a5d680`, engine map).
const NI_OBJECT_CLONE_OV2: u32 = 0x00a5_d680;
/// `ModelLoader::FindModel` (`004472a0`, engine map; `RET 8`).
const MODEL_LOADER_FIND_MODEL: u32 = 0x0044_72a0;
/// `0040c110`: the model pointer local's destructor.
const MODEL_POINTER_DESTROY_0040C110: u32 = 0x0040_c110;
/// `005497a0` (no stack word): the cell's node the object is attached to
/// (slot +0xDC with the object and 1).
const CELL_ATTACH_NODE_005497A0: u32 = 0x0054_97a0;
const SLOT_ATTACH_CHILD: u32 = 0xdc;
/// `0043d410` (`RET 0xC`: a `float`, two words) and `00a59c60` (`RET 4`):
/// the update data made from the time and the object's update with it.
const UPDATE_DATA_INIT_0043D410: u32 = 0x0043_d410;
const OBJECT_UPDATE_00A59C60: u32 = 0x00a5_9c60;
/// The timer object whose `00825c00` gives milliseconds, and 1000.0 (`double`).
const TIMER_011F6394: u32 = 0x011f_6394;
const THOUSAND: u32 = 0x0101_7b70;
/// `00857bd0` (`RET 4`): skips bytes of the save.
const SKIP_BYTES_00857BD0: u32 = 0x0085_7bd0;
/// `00825c00` (no stack word): reused on the timer above.
const POSITION_00825C00: u32 = 0x0082_5c00;
/// `BSAudio::QInstance` (`00453a70`) and `GetSoundHandleByNumericID`
/// (`00ad73b0`, `RET 0xC`: result, id, flags).
const BS_AUDIO_INSTANCE: u32 = 0x0045_3a70;
const BS_AUDIO_GET_SOUND_HANDLE: u32 = 0x00ad_73b0;
/// `00418900` (`RET 4`): assigns a sound handle.
const SOUND_HANDLE_ASSIGN_00418900: u32 = 0x0041_8900;
/// `0045bb80` (no stack word): the object's world position.
const WORLD_POSITION_0045BB80: u32 = 0x0045_bb80;
/// `BSSoundHandle::SetObjectToFollow` and `Play` (`RET 4` each).
const BS_SOUND_HANDLE_SET_OBJECT_TO_FOLLOW: u32 = 0x00ad_8f20;
const BS_SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// `0084e3a0` (no stack word): the form id (+0xC).
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;

// Translated from 00689210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00689210 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed (cdecl, eighteen words): makes a particle effect in `cell`
/// (nothing, null, without a cell) with a given rotation: a new 0x74-byte
/// `BSTempEffectParticle` ([`fn_00688fc0`] with `lifetime`, `filename`,
/// `follow`; null when the allocation fails), initialized (`Init`) under
/// the cell's node 4 (`00456fc0`) with the nine rotation words, the
/// position, `scale` and `flags`, then added to the process lists. Returns
/// the effect.
pub fn fn_00689210(
    e: &mut Engine,
    cell: u32,
    lifetime: f32,
    filename: u32,
    rotation: [u32; 9],
    position: [u32; 3],
    scale: f32,
    flags: u32,
    follow: u32,
) -> u32 {
    if cell == 0 {
        return 0;
    }
    let block = e.call(NI_ALLOCATE, &args![0x74u32]).u32();
    let effect = if block == 0 {
        0
    } else {
        fn_00688fc0(e, Ptr::new(block), cell, lifetime, filename, follow).addr()
    };
    let node = e.call(CELL_NODE_00456FC0, &args![cell, 4u32]).u32();
    let r = rotation;
    bs_temp_effect_particle_init(
        e,
        Ptr::new(effect),
        node,
        r[0],
        r[1],
        r[2],
        r[3],
        r[4],
        r[5],
        r[6],
        r[7],
        r[8],
        position[0],
        position[1],
        position[2],
        f32::from_bits(x87_quiet_bits(scale.to_bits())),
        flags,
    );
    e.call(PROCESS_LISTS_ADD_TEMP_EFFECT, &args![PROCESS_LISTS, effect]);
    effect
}

/// The uniform form of [`fn_00689210`] (eighteen words).
fn fn_00689210_entry(e: &mut Engine, a: &[u32]) -> Ret {
    let rotation = [a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11]];
    let effect = fn_00689210(
        e,
        a[0],
        f32::from_bits(a[1]),
        a[2],
        rotation,
        [a[12], a[13], a[14]],
        f32::from_bits(a[15]),
        a[16],
        a[17],
    );
    Ret {
        eax: effect,
        ..Default::default()
    }
}

/// The controller manager of the particle object (`00653270` on its first
/// controller), or 0.
fn controller_manager(e: &mut Engine, this: Ptr) -> u32 {
    let object = e.call(POINTER_GET, &args![this.addr() + 0x18]).u32();
    let controller = e.call(FIRST_CONTROLLER, &args![object]).u32();
    e.call(
        FIND_CONTROLLER_00653270,
        &args![CONTROLLER_MANAGER_RTTI, controller],
    )
    .u32()
}

// Translated from 00689920 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00689920 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed (`RET 4`): plays the particle object's sequence `name`: with a
/// controller manager and its sequence of that name (through a
/// fixed-string local), the sequence is activated (`0047aab0` with 0, 0,
/// 1.0, 0.0, 0) and, when the lifetime (+8) is below the sequence's length
/// (ordered), the lifetime becomes the length.
pub fn fn_00689920(e: &mut Engine, this: Ptr, name: u32) {
    let manager = controller_manager(e, this);
    if manager == 0 {
        return;
    }
    let sequence = e.with_stack(4, |e, local| {
        let text = e
            .call(FIXED_STRING_INIT_00438170, &args![local, name])
            .u32();
        let sequence = e
            .call(MANAGER_SEQUENCE_0047A520, &args![manager, text])
            .u32();
        e.call(FIXED_STRING_DESTROY_004381B0, &args![local]);
        sequence
    });
    if sequence == 0 {
        return;
    }
    e.call(
        MANAGER_ACTIVATE_0047AAB0,
        &args![
            manager,
            sequence,
            0u32,
            0u32,
            x87_store32(1.0),
            x87_store32(0.0),
            0u32
        ],
    );
    let length = e.call(SEQUENCE_LENGTH_00508100, &args![sequence]).f64();
    let length = x87_store32(length);
    let lifetime = x87_load32(e.mem.u32(this.addr() + 8));
    if lifetime < x87_m32(length) {
        e.mem.set_u32(this.addr() + 8, x87_quiet_bits(length));
    }
}

// Translated from 00689af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectParticle::CheckShouldSave` (engine map): true with a file
/// name (+0x1C) and a particle object (+0x18).
pub fn bs_temp_effect_particle_check_should_save(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x1c) != 0 && e.call(POINTER_GET, &args![this.addr() + 0x18]).u32() != 0
}

// Translated from 00689b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectParticle::GetSaveSize` (engine map; a `u16`): 6 for a
/// block header and size when the save uses blocks, the `BSTempEffect`
/// size (`006828e0`), 1 and the file name's length, 16 (rotation), 12
/// (translation), 4 (scale), 2 (sequence count), plus the controller
/// manager's own size (`0055fdb0`) when there is one. With the report flag
/// the size is reported (line 0x174).
pub fn bs_temp_effect_particle_get_save_size(e: &mut Engine, this: Ptr) -> u16 {
    let mut size: u16 = 0;
    let start = size;
    if uses_blocks(e) {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    let base = e.call(BS_TEMP_EFFECT_SAVE_SIZE, &args![this]).u16();
    size = size.wrapping_add(base);
    size = size.wrapping_add(1);
    let filename = e.mem.u32(this.addr() + 0x1c);
    let length = e.call(STRING_LENGTH_0044A670, &args![filename]).u32();
    size = (u32::from(size).wrapping_add(length)) as u16;
    size = size.wrapping_add(0x10);
    size = size.wrapping_add(0xc);
    size = size.wrapping_add(4);
    size = size.wrapping_add(2);
    let manager = controller_manager(e, this);
    if manager != 0 {
        let extra = e.call(MANAGER_SAVE_COUNT_0055FDB0, &args![manager]).u16();
        size = size.wrapping_add(extra);
    }
    if report_flag(e) {
        let difference = (i32::from(size) - i32::from(start)) as u32;
        save_report(
            e,
            SIZE_REPORT_FORM,
            SIZE_REPORT,
            difference,
            0x174,
            PARTICLE_SOURCE_FILE,
        );
    }
    size
}

// Translated from 00689cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectParticle::SaveGame` (engine map): within a save block
/// (when the save uses blocks; line 0x19B), the `BSTempEffect` part, the
/// file name's length (one byte) and the name, the particle object's
/// rotation as a quaternion (16 bytes), its translation (12) and scale
/// (4), the controller manager's count (2 bytes, 0 without a manager) and,
/// when it is not 0, the manager's state (`0055fe70` with -1.0). With the
/// report flag the bytes written are reported.
pub fn bs_temp_effect_particle_save_game(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    // EBP-0x3C .. EBP-4: the header word, the size, the translation, the
    // length byte, the quaternion, the count and the scale.
    e.with_stack(0x40, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x40 - offset;
        let mut placeholder = 0;
        let mut start = buffer_position(e);
        if report_flag(e) {
            start = buffer_position(e);
        }
        if uses_blocks(e) {
            placeholder = save_block_begin(e, local(0x3c), local(0x30));
        }
        e.call(BS_TEMP_EFFECT_SAVE_GAME, &args![this]);
        let filename = e.mem.u32(base + 0x1c);
        let length = e.call(STRING_LENGTH_0044A670, &args![filename]).u8();
        e.mem.set_u8(local(0x1d), length);
        save_bytes(e, local(0x1d), 1);
        let filename = e.mem.u32(base + 0x1c);
        save_bytes(e, filename, u32::from(length));
        let quaternion = local(0x1c);
        e.call(MATRIX_INIT_006815C0, &args![quaternion]);
        let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
        let rotation = e.call(OBJECT_ROTATION_006A9540, &args![object]).u32();
        e.call(QUATERNION_FROM_ROTATION, &args![quaternion, rotation]);
        save_bytes(e, quaternion, 0x10);
        let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
        let translation = e.call(OBJECT_TRANSLATION_0043C490, &args![object]).u32();
        for i in 0..3 {
            let word = e.mem.u32(translation + 4 * i);
            e.mem.set_u32(local(0x2c) + 4 * i, word);
        }
        save_bytes(e, local(0x2c), 0xc);
        let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
        let scale = e.call(OBJECT_SCALE_009CDAE0, &args![object]).f64();
        e.mem.set_u32(local(8), x87_store32(scale));
        save_bytes(e, local(8), 4);
        e.mem.set_u16(local(0xc), 0);
        let manager = controller_manager(e, this);
        if manager != 0 {
            let count = e.call(MANAGER_SAVE_COUNT_0055FDB0, &args![manager]).u16();
            e.mem.set_u16(local(0xc), count);
        }
        save_bytes(e, local(0xc), 2);
        if e.mem.u16(local(0xc)) != 0 {
            let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE_FLOAT));
            e.call(MANAGER_SAVE_0055FE70, &args![manager, minus_one]);
        }
        if report_flag(e) {
            let end = buffer_position(e);
            save_report(
                e,
                SAVE_REPORT_FORM,
                SAVE_REPORT,
                end.wrapping_sub(start),
                0x19b,
                PARTICLE_SOURCE_FILE,
            );
        }
        if uses_blocks(e) {
            save_block_end(e, placeholder, 0x19b, PARTICLE_SOURCE_FILE);
        }
    });
}

// Translated from 00689f80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00689f80 - the exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated.
/// `BSTempEffectParticle::LoadGame` (engine map): within a save block
/// (header checked, line 0x1A4; end checked, line 0x1EC) reads what
/// `SaveGame` wrote: the `BSTempEffect` part (its answer is the result),
/// the file name into a cleared 0x104-byte buffer, the quaternion (turned
/// into a rotation), the translation, the scale and the sequence count.
/// When the base loaded, the model is loaded (`ModelLoader::LoadFile`) and
/// cloned into the particle object (+0x18); the file name (+0x1C) becomes
/// the found model's name; the object gets the translation, rotation and
/// scale, is attached to the cell's node (slot +0xDC with 1), updates its
/// properties and is updated at the timer's milliseconds / 1000; its
/// animations start; with a count, the controller manager's state is
/// loaded (`0055ffa0` with -1.0), and a missing manager fails the load. A
/// missing model or object fails it too. A failed load with a count skips
/// the manager's bytes (`00857bd0`).
pub fn bs_temp_effect_particle_load_game(e: &mut Engine, this: Ptr) -> u8 {
    let base = this.addr();
    // The frame from EBP-0x1B8.
    e.with_stack(0x1c0, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x1c0 - offset;
        let tag = local(0x178);
        let size = local(0x170);
        let name = local(0x16c);
        let length = local(0x4d);
        let quaternion = local(0x4c);
        let rotation = local(0x34);
        let translation = local(0x5c);
        let scale = local(0x38);
        let count = local(0x3c);
        let model_pointer = local(0x188);
        let update_data = local(0x198);
        let mut start = 0;
        if uses_blocks(e) {
            load_bytes(e, tag, 4);
            if e.mem.u32(tag) != BLOCK_TAG {
                load_bad_header(e, 0x1a4, PARTICLE_SOURCE_FILE);
            }
            start = buffer_position(e);
            load_bytes(e, size, 2);
        }
        let mut loaded = e.call(BS_TEMP_EFFECT_LOAD_GAME, &args![this]).u8();
        load_bytes(e, length, 1);
        e.call(FILL_MEMORY_00403D30, &args![name, 0u32, 0x104u32]);
        let name_length = u32::from(e.mem.u8(length));
        load_bytes(e, name, name_length);
        e.call(MATRIX_INIT_006815C0, &args![quaternion]);
        load_bytes(e, quaternion, 0x10);
        e.call(MATRIX_INIT_006815C0, &args![rotation]);
        e.call(QUATERNION_TO_ROTATION, &args![quaternion, rotation]);
        e.call(MATRIX_INIT_006815C0, &args![translation]);
        load_bytes(e, translation, 0xc);
        load_bytes(e, scale, 4);
        load_bytes(e, count, 2);
        if loaded != 0 {
            let loader = e.global::<u32>(MODEL_LOADER);
            let model = e
                .call(
                    MODEL_LOADER_LOAD_FILE,
                    &args![loader, name, 0u32, 1u32, 0u32, 0u32, 0u32],
                )
                .u32();
            if model == 0 {
                loaded = 0;
            } else {
                let clone = e.call(NI_OBJECT_CLONE_OV2, &args![model]).u32();
                e.call(POINTER_SET, &args![base + 0x18, clone]);
                if e.call(POINTER_GET, &args![base + 0x18]).u32() == 0 {
                    loaded = 0;
                } else {
                    e.call(POINTER_INIT, &args![model_pointer, 0u32]);
                    let loader = e.global::<u32>(MODEL_LOADER);
                    e.call(MODEL_LOADER_FIND_MODEL, &args![loader, name, model_pointer]);
                    let found = e.call(POINTER_GET, &args![model_pointer]).u32();
                    let found_name = e.call(POINTER_GET, &args![found]).u32();
                    e.mem.set_u32(base + 0x1c, found_name);
                    let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
                    e.call(SET_TRANSLATE, &args![object, translation]);
                    let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
                    e.call(SET_ROTATE, &args![object, rotation]);
                    let scale_bits = x87_quiet_bits(e.mem.u32(scale));
                    let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
                    e.call(SET_SCALE, &args![object, scale_bits]);
                    // `BSTempEffect::pCell` +0x0C.
                    let cell = e.mem.u32(base + 0xc);
                    let node = e.call(CELL_ATTACH_NODE_005497A0, &args![cell]).u32();
                    let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
                    e.vcall(node, SLOT_ATTACH_CHILD, &args![object, 1u32]);
                    let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
                    e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![object]);
                    let milliseconds = e.call(POSITION_00825C00, &args![TIMER_011F6394]).u32();
                    let thousand: f64 = e.global(THOUSAND);
                    let time = x87_store32(x87_div(f64::from(milliseconds), thousand));
                    e.call(
                        UPDATE_DATA_INIT_0043D410,
                        &args![update_data, x87_quiet_bits(time), 1u32, 0u32],
                    );
                    let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
                    e.call(OBJECT_UPDATE_00A59C60, &args![object, update_data]);
                    let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
                    e.call(NI_TIME_CONTROLLER_START_ANIMATIONS, &args![object]);
                    if e.mem.u16(count) != 0 {
                        let manager = controller_manager(e, this);
                        if manager != 0 {
                            let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE_FLOAT));
                            let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
                            e.call(MANAGER_LOAD_0055FFA0, &args![manager, object, minus_one]);
                        } else {
                            loaded = 0;
                        }
                    }
                    e.call(MODEL_POINTER_DESTROY_0040C110, &args![model_pointer]);
                }
            }
        }
        if loaded == 0 && e.mem.u16(count) != 0 {
            let skip = u32::from(e.mem.u16(count));
            let game = e.global::<u32>(SAVE_LOAD_GAME);
            e.call(SKIP_BYTES_00857BD0, &args![game, skip]);
        }
        if uses_blocks(e) {
            let block_size = e.mem.u16(size);
            load_block_end(e, start, block_size, 0x1ec, PARTICLE_SOURCE_FILE);
        }
        loaded
    })
}

/// The `TESSaveLoadGame` singleton pointer.
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;

/// The impact data's first (`0068a810`) or second (`0068a830`) sound.
fn impact_sound(e: &mut Engine, data: u32, second: bool) -> u32 {
    if second {
        fn_0068a830(e, Ptr::new(data))
    } else {
        fn_0068a810(e, Ptr::new(data))
    }
}

// Translated from 0068a600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0068a600 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed: restarts the impact sounds. The sounds are stopped
/// ([`bs_temp_effect_particle_stop_sounds`]); with a particle object and
/// impact data (+0x58), each of the data's two sounds (`0068a810`,
/// `0068a830`) that is set gets a handle by its form id (flags 0x102,
/// through a temporary handle), placed at the object's world position,
/// made to follow the object and played (`Sound1` +0x5C, `Sound2` +0x68).
pub fn fn_0068a600(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    bs_temp_effect_particle_stop_sounds(e, this);
    if e.call(POINTER_GET, &args![base + 0x18]).u32() == 0 {
        return;
    }
    let data = e.mem.u32(base + 0x58);
    if data == 0 {
        return;
    }
    for (second, handle_offset) in [(false, 0x5cu32), (true, 0x68)] {
        let data = e.mem.u32(base + 0x58);
        if impact_sound(e, data, second) == 0 {
            continue;
        }
        let data = e.mem.u32(base + 0x58);
        let sound = impact_sound(e, data, second);
        let id = e.call(FORM_ID_0084E3A0, &args![sound]).u32();
        e.with_stack(0xc, |e, temporary| {
            let audio = e.call(BS_AUDIO_INSTANCE, &args![]).u32();
            let handle = e
                .call(
                    BS_AUDIO_GET_SOUND_HANDLE,
                    &args![audio, temporary, id, 0x102u32],
                )
                .u32();
            e.call(
                SOUND_HANDLE_ASSIGN_00418900,
                &args![base + handle_offset, handle],
            );
            e.call(SOUND_HANDLE_DESTROY, &args![temporary]);
        });
        let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
        let position = e.call(WORLD_POSITION_0045BB80, &args![object]).u32();
        fn_0068a7d0(e, Ptr::new(base + handle_offset), position);
        let object = e.call(POINTER_GET, &args![base + 0x18]).u32();
        e.call(
            BS_SOUND_HANDLE_SET_OBJECT_TO_FOLLOW,
            &args![base + handle_offset, object],
        );
        e.call(BS_SOUND_HANDLE_PLAY, &args![base + handle_offset, 0u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x0068_9210, fn_00689210_entry as AbiFn),
        entry!(0x0068_9920, fn_00689920(Ptr, u32)),
        entry!(
            0x0068_9af0,
            bs_temp_effect_particle_check_should_save(Ptr) -> bool
        ),
        entry!(
            0x0068_9b20,
            bs_temp_effect_particle_get_save_size(Ptr) -> u16
        ),
        entry!(0x0068_9cb0, bs_temp_effect_particle_save_game(Ptr)),
        entry!(0x0068_9f80, bs_temp_effect_particle_load_game(Ptr) -> u8),
        entry!(0x0068_a600, fn_0068a600(Ptr)),
        entry!(0x0068_8fc0, fn_00688fc0(Ptr, u32, f32, u32, u32) -> Ptr),
        entry!(
            0x0068_90b0,
            fn_006890b0(u32, f32, u32, u32, u32, u32, u32, u32, u32, f32, u32, u32) -> u32
        ),
        entry!(
            0x0068_9310,
            bs_temp_effect_particle_init(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, f32, u32
            )
        ),
        entry!(0x0068_96c0, bs_temp_effect_particle_detach(Ptr)),
        entry!(0x0068_9720, bs_temp_effect_particle_destructor(Ptr)),
        entry!(0x0068_9840, fn_00689840(Ptr, u32) -> f32),
        entry!(0x0068_9a10, bs_temp_effect_particle_update(Ptr, f32) -> bool),
        entry!(0x0068_a7d0, fn_0068a7d0(Ptr, u32)),
        entry!(0x0068_a830, fn_0068a830(Ptr) -> u32),
        entry!(0x0068_a850, bs_temp_effect_particle_stop_sounds(Ptr)),
        entry!(0x0068_aa50, fn_0068aa50(Ptr, u32)),
        entry!(0x0068_a810, fn_0068a810(Ptr) -> u32),
        entry!(0x0068_8f80, fn_00688f80(Ptr) -> u32),
        entry!(0x0068_8f90, bs_temp_effect_particle_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
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
    fn bs_temp_effect_particle_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0068_8f90, 0x0068_9720, 0x74);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00688f80_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0068_8f80, &args![0x1000u32]).u32(), 0x011d_6b84);
        assert_eq!(e.call(0x0068_8f80, &args![0u32]).u32(), 0x011d_6b84);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_0068a810_answers_the_dword_at_offset_0x4c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x48, 1);
        e.mem.set_u32(this + 0x4c, 0x0bad_f00d);
        e.mem.set_u32(this + 0x50, 2);
        assert_eq!(e.call(0x0068_a810, &args![this]).u32(), 0x0bad_f00d);
    }
}

#[cfg(test)]
mod b066_tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet_all(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            answer(e, *a, 0);
        }
    }

    /// An object whose vtable has `slots` (byte offset, target).
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        this
    }

    fn pointers(e: &mut Engine) {
        e.register(POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_INIT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_ASSIGN_POINTER, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        answer(e, POINTER_DESTROY, 0);
    }

    const CONSTRUCTOR_CALLEES: [u32; 4] = [
        BS_TEMP_EFFECT_CONSTRUCT,
        TRANSFORM_INIT,
        SOUND_HANDLE_INIT,
        NI_TRANSFORM_MAKE_IDENTITY,
    ];

    #[test]
    fn constructor_builds_the_effect() {
        let mut e = Engine::new();
        pointers(&mut e);
        quiet_all(&mut e, &CONSTRUCTOR_CALLEES);
        let this = e.mem.alloc(0x74);
        e.mem.set_u32(this + 0x58, 0x77);
        e.call_log = Some(vec![]);
        let result = e.call(
            0x0068_8fc0,
            &args![this, 0x4000u32, 2.5f32, 0x4100u32, 0x4200u32],
        );
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(
            calls(&e, BS_TEMP_EFFECT_CONSTRUCT),
            vec![vec![this, 0x4000, 2.5f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, POINTER_INIT),
            vec![vec![this + 0x18, 0], vec![this + 0x20, 0]]
        );
        assert_eq!(
            calls(&e, SOUND_HANDLE_INIT),
            vec![vec![this + 0x5c], vec![this + 0x68]]
        );
        assert_eq!(
            calls(&e, NI_TRANSFORM_MAKE_IDENTITY),
            vec![vec![this + 0x24]]
        );
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0x4200);
        assert_eq!(e.mem.u32(this + 0x1c), 0x4100);
        assert_eq!(e.mem.u32(this + 0x58), 0);
    }

    fn make_engine(model: u32) -> Engine {
        let mut e = Engine::new();
        pointers(&mut e);
        quiet_all(&mut e, &CONSTRUCTOR_CALLEES);
        e.map(MODEL_LOADER, 4);
        e.mem.set_u32(MODEL_LOADER, 0x4300);
        e.map(PARTICLE_CACHE_OWNER, 4);
        answer(&mut e, MODEL_LOADER_LOAD_FILE, model);
        e
    }

    #[test]
    fn make_needs_a_cell_and_adds_the_effect() {
        let mut e = make_engine(0);
        e.call_log = Some(vec![]);
        let words = [0u32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(e.call(0x0068_90b0, &words).u32(), 0);
        assert!(calls(&e, NI_ALLOCATE).is_empty());
        let block = e.mem.alloc(0x74);
        answer(&mut e, NI_ALLOCATE, block);
        e.register(REAL_004A4240, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        answer(&mut e, GET_ROTATION_Z_TO_VECTOR_TRANSPOSE, 0x4400);
        quiet_all(
            &mut e,
            &[
                MATRIX_INIT_006815C0,
                MATRIX_004A0C90,
                NI_MATRIX3_MULTIPLY,
                PROCESS_LISTS_ADD_TEMP_EFFECT,
            ],
        );
        answer(&mut e, CELL_NODE_00456FC0, 0x4500);
        let words = [
            0x4000u32,
            1.5f32.to_bits(),
            0x4100,
            1,
            2,
            3,
            4,
            5,
            6,
            2.0f32.to_bits(),
            7,
            0x4200,
        ];
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_90b0, &words).u32(), block);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x74]]);
        assert_eq!(
            calls(&e, BS_TEMP_EFFECT_CONSTRUCT),
            vec![vec![block, 0x4000, 1.5f32.to_bits()]]
        );
        assert_eq!(e.mem.u32(block + 0x1c), 0x4100);
        assert_eq!(e.mem.u32(block + 0x20), 0x4200);
        let facing = calls(&e, GET_ROTATION_Z_TO_VECTOR_TRANSPOSE);
        assert_eq!(facing[0][1..], [1, 2, 3]);
        let spin = calls(&e, MATRIX_004A0C90);
        assert_eq!(spin[0][1], 0.5f32.to_bits());
        let multiply = calls(&e, NI_MATRIX3_MULTIPLY);
        assert_eq!(multiply[0][0], 0x4400);
        assert_eq!(multiply[0][2], spin[0][0]);
        assert_eq!(calls(&e, CELL_NODE_00456FC0), vec![vec![0x4000, 4]]);
        // Init ran (and stopped without a model).
        assert_eq!(
            calls(&e, MODEL_LOADER_LOAD_FILE),
            vec![vec![0x4300, 0x4100, 0, 1, 0, 0, 1]]
        );
        assert_eq!(
            calls(&e, PROCESS_LISTS_ADD_TEMP_EFFECT),
            vec![vec![PROCESS_LISTS, block]]
        );
    }

    const PARENT_SLOT: u32 = 0x0320_0000;
    const NODE_SLOT: u32 = 0x0320_0004;

    /// `Init` with a model of count `count` whose controllers span `range`.
    fn init_engine(count: u32) -> (Engine, u32, u32) {
        let mut e = make_engine(0);
        let model = object(&mut e, &[(0xc, NODE_SLOT)]);
        answer(&mut e, MODEL_LOADER_LOAD_FILE, model);
        answer(&mut e, NODE_SLOT, 0);
        answer(&mut e, COUNT_00726070, count);
        let parent = object(&mut e, &[(0xdc, PARENT_SLOT)]);
        quiet_all(
            &mut e,
            &[
                PARENT_SLOT,
                SET_TRANSLATE,
                SET_ROTATE,
                SET_SCALE,
                TEST_008C7AA0,
                NI_AV_OBJECT_UPDATE_PROPERTIES,
                NI_TIME_CONTROLLER_START_ANIMATIONS,
                ARRAY_INIT_0068ABB0,
                COLLECT_0068A8F0,
                ARRAY_SIZE_00658930,
                ARRAY_DESTROY_0068AB60,
                FIRST_CONTROLLER,
                CLONING_PROCESS_INIT,
                CLONING_PROCESS_DESTROY,
                TRANSFORM_INIT,
                NI_TRANSFORM_INVERT_NON_UNIFORM,
            ],
        );
        (e, model, parent)
    }

    fn init_words(effect: u32, parent: u32, flags: u32) -> Vec<u32> {
        let mut words = vec![effect, parent];
        words.extend(0x10..0x19);
        words.extend([0x20, 0x21, 0x22, 3.0f32.to_bits(), flags]);
        words
    }

    #[test]
    fn init_stops_without_a_model() {
        let mut e = make_engine(0);
        let effect = e.mem.alloc(0x74);
        e.call_log = Some(vec![]);
        e.call(0x0068_9310, &init_words(effect, 0x100, 0));
        assert_eq!(calls(&e, POINTER_DESTROY).len(), 1);
        assert!(calls(&e, POINTER_SET).is_empty());
    }

    #[test]
    fn init_places_and_attaches_the_loaded_object() {
        let (mut e, model, parent) = init_engine(1);
        let effect = e.mem.alloc(0x74);
        e.call_log = Some(vec![]);
        e.call(0x0068_9310, &init_words(effect, parent, 0));
        assert_eq!(e.mem.u32(effect + 0x18), model);
        let translate = calls(&e, SET_TRANSLATE);
        assert_eq!(translate[0][0], model);
        let rotate = calls(&e, SET_ROTATE);
        assert_eq!(rotate[0][0], model);
        assert_eq!(rotate[0][1] + 0x24, translate[0][1]);
        assert_eq!(calls(&e, SET_SCALE), vec![vec![model, 3.0f32.to_bits()]]);
        assert_eq!(calls(&e, PARENT_SLOT), vec![vec![parent, model, 1]]);
        assert_eq!(calls(&e, NI_AV_OBJECT_UPDATE_PROPERTIES), vec![vec![model]]);
        assert_eq!(calls(&e, COLLECT_0068A8F0)[0][..2], [effect, model]);
        assert_eq!(
            calls(&e, NI_TIME_CONTROLLER_START_ANIMATIONS),
            vec![vec![model]]
        );
        assert!(calls(&e, TRANSFORM_INIT).is_empty());
    }

    #[test]
    fn init_passes_its_argument_words_by_address() {
        let (mut e, _, parent) = init_engine(1);
        e.register(SET_TRANSLATE, |e, a| {
            assert_eq!(
                (0..3).map(|i| e.mem.u32(a[1] + 4 * i)).collect::<Vec<_>>(),
                vec![0x20, 0x21, 0x22]
            );
            Ret::default()
        });
        e.register(SET_ROTATE, |e, a| {
            assert_eq!(
                (0..9).map(|i| e.mem.u32(a[1] + 4 * i)).collect::<Vec<_>>(),
                (0x10..0x19).collect::<Vec<u32>>()
            );
            Ret::default()
        });
        let effect = e.mem.alloc(0x74);
        e.call_log = Some(vec![]);
        e.call(0x0068_9310, &init_words(effect, parent, 0));
        assert_eq!(calls(&e, SET_ROTATE).len(), 1);
    }

    #[test]
    fn init_clones_a_shared_object_and_reads_the_cache_first() {
        // Count 3 without flag 2: no object.
        let (mut e, _, parent) = init_engine(3);
        let effect = e.mem.alloc(0x74);
        e.call_log = Some(vec![]);
        e.call(0x0068_9310, &init_words(effect, parent, 0));
        assert!(calls(&e, SET_TRANSLATE).is_empty());
        // With flag 2: a clone.
        let clone = object(&mut e, &[(0xc, NODE_SLOT)]);
        answer(&mut e, NI_OBJECT_CLONE, clone);
        e.call_log = Some(vec![]);
        e.call(0x0068_9310, &init_words(effect, parent, 2));
        assert_eq!(e.mem.u32(effect + 0x18), clone);
        assert_eq!(calls(&e, CLONING_PROCESS_INIT)[0][1], 1.0f32.to_bits());
        assert_eq!(calls(&e, CLONING_PROCESS_DESTROY).len(), 1);
        // Flag 1 and a cache: the cached object is used.
        e.mem.set_u32(PARTICLE_CACHE_OWNER, 0x4700);
        let cached = object(&mut e, &[(0xc, NODE_SLOT)]);
        e.register_double(TES_GET_UNUSED_CACHED_PARTICLE_OBJECT, move |e, a| {
            e.mem.set_u32(a[2], cached);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x0068_9310, &init_words(effect, parent, 1));
        let cache = calls(&e, TES_GET_UNUSED_CACHED_PARTICLE_OBJECT);
        assert_eq!(cache[0][0], 0x4700);
        assert_eq!(cache[0][2], effect + 0x18);
        assert!(calls(&e, COUNT_00726070).is_empty());
    }

    #[test]
    fn init_sets_the_lifetime_and_the_follow_offset() {
        for (range, lifetime) in [(-1.0f32, 9.0f32), (0.05, 0.1), (4.0, 4.0)] {
            let (mut e, _, parent) = init_engine(1);
            let effect = e.mem.alloc(0x74);
            e.mem.set_f32(effect + 8, 9.0);
            for (addr, value) in [(MINUS_ONE, -1.0f64), (MINIMUM_LIFETIME, f64::from(0.1f32))] {
                e.map(addr, 8);
                e.mem.set_f64(addr, value);
            }
            e.map(MINUS_ONE_FLOAT, 4);
            e.mem.set_f32(MINUS_ONE_FLOAT, -1.0);
            e.map(MINIMUM_LIFETIME_FLOAT, 4);
            e.mem.set_f32(MINIMUM_LIFETIME_FLOAT, 0.1);
            // One controller spanning `range`.
            let controller = e.mem.alloc(4);
            answer(&mut e, FIRST_CONTROLLER, controller);
            answer(&mut e, NEXT_CONTROLLER, 0);
            e.register_double(CONTROLLER_END_004A7BD0, move |_, _| Ret {
                st0: f64::from(range),
                ..Ret::default()
            });
            e.register(CONTROLLER_START_0047C860, |_, _| Ret::default());
            e.register(FLOAT_MAX_00404010, |_, a| Ret {
                st0: f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))),
                ..Ret::default()
            });
            // A follow object whose world transform is all 0x55.
            e.mem.set_u32(effect + 0x20, 0x4900);
            let world = e.mem.alloc(0x34);
            for i in 0..13 {
                e.mem.set_u32(world + 4 * i, 0x55);
            }
            answer(&mut e, WORLD_TRANSFORM_00461130, world);
            let product = e.mem.alloc(0x34);
            for i in 0..13 {
                e.mem.set_u32(product + 4 * i, 0x60 + i);
            }
            answer(&mut e, TRANSFORM_MULTIPLY_0062C250, product);
            e.call_log = Some(vec![]);
            e.call(0x0068_9310, &init_words(effect, parent, 4));
            assert_eq!(e.mem.f32(effect + 8), lifetime, "range {range}");
            let invert = calls(&e, NI_TRANSFORM_INVERT_NON_UNIFORM);
            let multiply = calls(&e, TRANSFORM_MULTIPLY_0062C250);
            assert_eq!(multiply[0][0], invert[0][1]);
            for i in 0..13 {
                assert_eq!(e.mem.u32(effect + 0x24 + 4 * i), 0x60 + i);
            }
        }
    }

    #[test]
    fn init_queues_the_attach_when_asked() {
        let (mut e, model, parent) = init_engine(1);
        answer(&mut e, TEST_008C7AA0, 1);
        answer(&mut e, TASK_QUEUE_004537B0, 0x4a00);
        answer(&mut e, TASK_QUEUE_QUEUE_NODE_ATTACH, 0);
        let effect = e.mem.alloc(0x74);
        e.call_log = Some(vec![]);
        e.call(0x0068_9310, &init_words(effect, parent, 0));
        assert_eq!(
            calls(&e, TASK_QUEUE_QUEUE_NODE_ATTACH),
            vec![vec![0x4a00, model, parent, 1, 1]]
        );
        assert!(calls(&e, PARENT_SLOT).is_empty());
    }

    #[test]
    fn detach_removes_the_addon_nodes() {
        let mut e = Engine::new();
        pointers(&mut e);
        answer(&mut e, REMOVE_MASTER_PARTICLE_ADDON_NODES, 0);
        let effect = e.mem.alloc(0x74);
        e.call_log = Some(vec![]);
        e.call(0x0068_96c0, &args![effect]);
        assert!(calls(&e, REMOVE_MASTER_PARTICLE_ADDON_NODES).is_empty());
        let object = object(&mut e, &[(0xc, NODE_SLOT)]);
        answer(&mut e, NODE_SLOT, 0x4b00);
        e.mem.set_u32(effect + 0x18, object);
        e.call(0x0068_96c0, &args![effect]);
        assert_eq!(
            calls(&e, REMOVE_MASTER_PARTICLE_ADDON_NODES),
            vec![vec![0x4b00]]
        );
    }

    #[test]
    fn destructor_releases_and_detaches_everything() {
        let mut e = Engine::new();
        pointers(&mut e);
        e.map(MODEL_LOADER, 4);
        e.mem.set_u32(MODEL_LOADER, 0x4300);
        let parent = object(&mut e, &[(0xe8, PARENT_SLOT)]);
        answer(&mut e, PARENT_009611E0, parent);
        quiet_all(
            &mut e,
            &[
                PARENT_SLOT,
                MODEL_LOADER_RELEASE_MODEL,
                SOUND_HANDLE_DESTROY,
                BS_TEMP_EFFECT_DESTROY,
                BS_SOUND_HANDLE_IS_VALID,
            ],
        );
        let effect = e.mem.alloc(0x74);
        e.mem.set_u32(effect + 0x1c, 0x4100);
        e.mem.set_u32(effect + 0x18, 0x4c00);
        e.call_log = Some(vec![]);
        e.call(0x0068_9720, &args![effect]);
        assert_eq!(e.mem.u32(effect), VTABLE);
        assert_eq!(
            calls(&e, MODEL_LOADER_RELEASE_MODEL),
            vec![vec![0x4300, 0x4100, 0, 1]]
        );
        assert_eq!(calls(&e, PARENT_SLOT), vec![vec![parent, 0x4c00]]);
        assert_eq!(e.mem.u32(effect + 0x18), 0);
        assert_eq!(
            calls(&e, SOUND_HANDLE_DESTROY),
            vec![vec![effect + 0x68], vec![effect + 0x5c]]
        );
        assert_eq!(
            calls(&e, POINTER_DESTROY),
            vec![vec![effect + 0x20], vec![effect + 0x18]]
        );
        assert_eq!(calls(&e, BS_TEMP_EFFECT_DESTROY), vec![vec![effect]]);
        assert_eq!(calls(&e, BS_SOUND_HANDLE_IS_VALID).len(), 2);
    }

    #[test]
    fn longest_range_covers_controllers_and_children() {
        let mut e = Engine::new();
        e.map(MINUS_ONE_FLOAT, 4);
        e.mem.set_f32(MINUS_ONE_FLOAT, -1.0);
        // Each object holds its controller count at +4 and its node at +8.
        e.register(FIRST_CONTROLLER, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(NEXT_CONTROLLER, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(CONTROLLER_END_004A7BD0, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 4)),
            ..Ret::default()
        });
        e.register(CONTROLLER_START_0047C860, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 8)),
            ..Ret::default()
        });
        e.register(FLOAT_MAX_00404010, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        e.register(0x0320_0100, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        e.register(CHILD_COUNT, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(CHILD_AT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4 + 4 * a[1]),
            ..Ret::default()
        });
        let make = |e: &mut Engine, end: f32, start: f32, node: u32| {
            let o = object(e, &[(0xc, 0x0320_0100)]);
            let controller = e.mem.alloc(12);
            e.mem.set_f32(controller + 4, end);
            e.mem.set_f32(controller + 8, start);
            e.mem.set_u32(o + 4, controller);
            e.mem.set_u32(o + 8, node);
            o
        };
        let child = make(&mut e, 9.0, 1.0, 0);
        let node = e.mem.alloc(12);
        e.mem.set_u32(node, 2);
        e.mem.set_u32(node + 4, 0);
        e.mem.set_u32(node + 8, child);
        let root = make(&mut e, 3.0, 1.0, node);
        assert_eq!(e.call(0x0068_9840, &args![0u32, root]).f32(), 8.0);
        let lone = make(&mut e, 3.0, 1.0, 0);
        assert_eq!(e.call(0x0068_9840, &args![0u32, lone]).f32(), 2.0);
    }

    #[test]
    fn update_follows_the_follow_object() {
        let mut e = Engine::new();
        pointers(&mut e);
        answer(&mut e, BS_TEMP_EFFECT_UPDATE, 1);
        quiet_all(&mut e, &[SET_TRANSLATE, SET_ROTATE, SET_SCALE]);
        answer(&mut e, WORLD_TRANSFORM_00461130, 0x4d00);
        e.register(TRANSFORM_MULTIPLY_0062C250, |e, a| {
            e.mem.set_f32(a[1] + 0x30, 2.0);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let effect = e.mem.alloc(0x74);
        e.mem.set_u32(effect + 0x18, 0x4c00);
        e.mem.set_u32(effect + 0x20, 0x4e00);
        answer(&mut e, COUNT_00726070, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0068_9a10, &args![effect, 0.25f32]).bool());
        assert_eq!(
            calls(&e, BS_TEMP_EFFECT_UPDATE),
            vec![vec![effect, 0.25f32.to_bits()]]
        );
        let multiply = calls(&e, TRANSFORM_MULTIPLY_0062C250);
        assert_eq!(multiply[0][0], 0x4d00);
        assert_eq!(multiply[0][2], effect + 0x24);
        let transform = multiply[0][1];
        assert_eq!(
            calls(&e, SET_TRANSLATE),
            vec![vec![0x4c00, transform + 0x24]]
        );
        assert_eq!(calls(&e, SET_ROTATE), vec![vec![0x4c00, transform]]);
        assert_eq!(calls(&e, SET_SCALE), vec![vec![0x4c00, 2.0f32.to_bits()]]);
        // Held only by the effect: dropped, and nothing is placed.
        answer(&mut e, COUNT_00726070, 1);
        answer(&mut e, BS_TEMP_EFFECT_UPDATE, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0068_9a10, &args![effect, 0.25f32]).bool());
        assert_eq!(e.mem.u32(effect + 0x20), 0);
        assert!(calls(&e, SET_TRANSLATE).is_empty());
    }

    #[test]
    fn sound_position_getter_and_stop_sounds() {
        let mut e = Engine::new();
        answer(&mut e, BS_SOUND_HANDLE_SET_POSITION, 0);
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        e.mem.set_u32(position + 4, 0x7f80_0001);
        e.mem.set_f32(position + 8, 3.0);
        e.call_log = Some(vec![]);
        e.call(0x0068_a7d0, &args![0x4f00u32, position]);
        assert_eq!(
            calls(&e, BS_SOUND_HANDLE_SET_POSITION),
            vec![vec![
                0x4f00,
                1.0f32.to_bits(),
                0x7fc0_0001,
                3.0f32.to_bits()
            ]]
        );
        let effect = e.mem.alloc(0x74);
        e.mem.set_u32(effect + 0x50, 0x1234);
        assert_eq!(e.call(0x0068_a830, &args![effect]).u32(), 0x1234);
        // Sound 1 stays valid after stopping (released); sound 2 is invalid.
        let first = effect + 0x5c;
        e.register_double(BS_SOUND_HANDLE_IS_VALID, move |_, a| Ret {
            eax: u32::from(a[0] == first),
            ..Ret::default()
        });
        quiet_all(&mut e, &[BS_SOUND_HANDLE_STOP, BS_SOUND_HANDLE_RELEASE]);
        e.call_log = Some(vec![]);
        e.call(0x0068_a850, &args![effect]);
        assert_eq!(calls(&e, BS_SOUND_HANDLE_STOP), vec![vec![first]]);
        assert_eq!(calls(&e, BS_SOUND_HANDLE_RELEASE), vec![vec![first]]);
        assert_eq!(calls(&e, BS_SOUND_HANDLE_IS_VALID).len(), 3);
    }

    #[test]
    fn emitters_are_turned_half_about_y() {
        let mut e = Engine::new();
        e.map(PI_FLOAT, 4);
        e.mem.set_f32(PI_FLOAT, std::f32::consts::PI);
        let items = e.mem.alloc(8);
        e.mem.set_u32(items, 0x5100);
        e.mem.set_u32(items + 4, 0);
        e.register(ARRAY_SIZE_00658930, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.register_double(ARRAY_AT_00877A30, move |_, a| Ret {
            eax: items + 4 * a[1],
            ..Ret::default()
        });
        answer(&mut e, PARENT_009611E0, 0x5200);
        answer(&mut e, WORLD_TRANSFORM_00461130, 0x5300);
        answer(&mut e, NI_MATRIX3_MULTIPLY, 0x5400);
        quiet_all(
            &mut e,
            &[
                ARRAY_INIT_0068ABB0,
                COLLECT_0068A8F0,
                ARRAY_DESTROY_0068AB60,
                NI_MATRIX3_INVERSE,
                MATRIX_INIT_006815C0,
                NI_MATRIX3_MAKE_Y_ROTATION,
                SET_ROTATE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0068_aa50, &args![0x5000u32, 0x5500u32]);
        let array = calls(&e, ARRAY_INIT_0068ABB0);
        assert_eq!(array[0][1..], [10, 1]);
        let collect = calls(&e, COLLECT_0068A8F0);
        assert_eq!(collect[0][..3], [0x5000, 0x5500, array[0][0]]);
        assert_eq!(calls(&e, ARRAY_AT_00877A30).len(), 2);
        assert_eq!(calls(&e, NI_MATRIX3_INVERSE)[0][0], 0x5300);
        assert_eq!(
            calls(&e, NI_MATRIX3_MAKE_Y_ROTATION)[0][1],
            std::f32::consts::PI.to_bits()
        );
        assert_eq!(calls(&e, SET_ROTATE), vec![vec![0x5100, 0x5400]]);
        assert_eq!(calls(&e, ARRAY_DESTROY_0068AB60), vec![vec![array[0][0]]]);
    }
}

#[cfg(test)]
mod tests_n073 {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Default::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Default::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn noop(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn pointer_doubles(e: &mut Engine) {
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_INIT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
    }

    #[test]
    fn fn_00689210_makes_and_initializes_the_effect() {
        let mut e = Engine::new();
        // A null cell: nothing.
        e.call_log = Some(vec![]);
        let words: Vec<u32> = (0..18).map(|i| if i == 0 { 0 } else { i }).collect();
        assert_eq!(e.call(0x0068_9210, &words).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // A new effect, constructed, then Init (no model loads: it stops).
        let block = e.mem.alloc(0x74);
        e.register_double(NI_ALLOCATE, move |_, _| ret(block));
        pointer_doubles(&mut e);
        noop(
            &mut e,
            &[
                BS_TEMP_EFFECT_CONSTRUCT,
                TRANSFORM_INIT,
                SOUND_HANDLE_INIT,
                NI_TRANSFORM_MAKE_IDENTITY,
                POINTER_DESTROY,
                MODEL_LOADER_LOAD_FILE,
            ],
        );
        e.map(MODEL_LOADER & !0xfff, 0x1000);
        e.register(CELL_NODE_00456FC0, |_, _| ret(0x4e0d));
        e.register(PROCESS_LISTS_ADD_TEMP_EFFECT, |_, _| Ret::default());
        let mut words: Vec<u32> = (0..18).map(|i| 0x100 + i).collect();
        words[15] = 2.0f32.to_bits();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_9210, &words).u32(), block);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CELL_NODE_00456FC0), vec![vec![0x100, 4]]);
        assert_eq!(
            calls_to(&log, BS_TEMP_EFFECT_CONSTRUCT),
            vec![vec![block, 0x100, 0x101]]
        );
        assert_eq!(e.mem.u32(block + 0x1c), 0x102);
        assert_eq!(e.mem.u32(block + 0x20), 0x111);
        assert_eq!(
            calls_to(&log, PROCESS_LISTS_ADD_TEMP_EFFECT),
            vec![vec![PROCESS_LISTS, block]]
        );
    }

    #[test]
    fn fn_00689920_plays_the_sequence_and_extends_the_lifetime() {
        let mut e = Engine::new();
        pointer_doubles(&mut e);
        e.register(FIRST_CONTROLLER, |_, _| ret(0xc0));
        e.register(FIND_CONTROLLER_00653270, |_, _| ret(0));
        let this = e.mem.alloc(0x74);
        e.call_log = Some(vec![]);
        e.call(0x0068_9920, &args![this, 0x5e9u32]);
        assert!(calls_to(&e.call_log.take().unwrap(), MANAGER_SEQUENCE_0047A520).is_empty());
        e.register(FIND_CONTROLLER_00653270, |_, _| ret(0x3a3a));
        e.register(FIXED_STRING_INIT_00438170, |_, a| ret(a[0]));
        noop(
            &mut e,
            &[FIXED_STRING_DESTROY_004381B0, MANAGER_ACTIVATE_0047AAB0],
        );
        e.register(MANAGER_SEQUENCE_0047A520, |_, _| ret(0x5e00));
        e.register(SEQUENCE_LENGTH_00508100, |_, _| st0(3.0));
        e.mem.set_f32(this + 8, 2.0);
        e.call_log = Some(vec![]);
        e.call(0x0068_9920, &args![this, 0x5e9u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MANAGER_ACTIVATE_0047AAB0),
            vec![vec![0x3a3a, 0x5e00, 0, 0, 1.0f32.to_bits(), 0, 0]]
        );
        assert_eq!(e.mem.f32(this + 8), 3.0);
        // A longer lifetime stays.
        e.mem.set_f32(this + 8, 4.0);
        e.call(0x0068_9920, &args![this, 0x5e9u32]);
        assert_eq!(e.mem.f32(this + 8), 4.0);
    }

    #[test]
    fn check_should_save_needs_name_and_object() {
        let mut e = Engine::new();
        pointer_doubles(&mut e);
        let this = e.mem.alloc(0x74);
        assert!(!e.call(0x0068_9af0, &args![this]).bool());
        e.mem.set_u32(this + 0x1c, 0x1234);
        assert!(!e.call(0x0068_9af0, &args![this]).bool());
        e.mem.set_u32(this + 0x18, 0x5678);
        assert!(e.call(0x0068_9af0, &args![this]).bool());
    }

    fn save_globals(e: &mut Engine, blocks: u32) {
        e.map(SAVE_LOAD_GAME & !0xfff, 0x1000);
        e.set_global(SAVE_LOAD_GAME, 0x5a5au32);
        e.register_double(0x0086_2110, move |_, _| ret(blocks));
        let flag = e.mem.alloc(4);
        e.register_double(0x0040_8d60, move |_, _| ret(flag));
    }

    #[test]
    fn save_size_adds_the_parts() {
        let mut e = Engine::new();
        pointer_doubles(&mut e);
        save_globals(&mut e, 1);
        e.register(BS_TEMP_EFFECT_SAVE_SIZE, |_, _| ret(12));
        e.register(STRING_LENGTH_0044A670, |_, _| ret(9));
        e.register(FIRST_CONTROLLER, |_, _| ret(0xc0));
        e.register(FIND_CONTROLLER_00653270, |_, _| ret(0x3a3a));
        e.register(MANAGER_SAVE_COUNT_0055FDB0, |_, _| ret(0x20));
        let this = e.mem.alloc(0x74);
        // 6 + 12 + 1 + 9 + 16 + 12 + 4 + 2 + 0x20.
        assert_eq!(e.call(0x0068_9b20, &args![this]).u16(), 94);
        e.register(FIND_CONTROLLER_00653270, |_, _| ret(0));
        assert_eq!(e.call(0x0068_9b20, &args![this]).u16(), 62);
    }

    #[test]
    fn save_game_writes_the_parts_in_order() {
        let mut e = Engine::new();
        pointer_doubles(&mut e);
        save_globals(&mut e, 1);
        let buffer = e.mem.alloc(0x100);
        let position = std::rc::Rc::new(std::cell::Cell::new(buffer));
        let p = position.clone();
        e.register_double(0x0082_5c00, move |_, _| ret(p.get()));
        let p = position.clone();
        let written = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let w = written.clone();
        e.register_double(0x0085_79b0, move |e, a| {
            w.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            p.set(p.get() + a[2]);
            Ret::default()
        });
        noop(
            &mut e,
            &[
                BS_TEMP_EFFECT_SAVE_GAME,
                MATRIX_INIT_006815C0,
                MANAGER_SAVE_0055FE70,
            ],
        );
        e.register(STRING_LENGTH_0044A670, |_, _| ret(3));
        e.register(OBJECT_ROTATION_006A9540, |_, _| ret(0x7070));
        e.register(QUATERNION_FROM_ROTATION, |e, a| {
            e.mem.set_f32(a[0], 0.5);
            Ret::default()
        });
        let translation = e.mem.alloc(0xc);
        e.mem.set_f32(translation, 7.0);
        e.register_double(OBJECT_TRANSLATION_0043C490, move |_, _| ret(translation));
        e.register(OBJECT_SCALE_009CDAE0, |_, _| st0(1.5));
        e.register(FIRST_CONTROLLER, |_, _| ret(0xc0));
        e.register(FIND_CONTROLLER_00653270, |_, _| ret(0x3a3a));
        e.register(MANAGER_SAVE_COUNT_0055FDB0, |_, _| ret(4));
        e.map(MINUS_ONE_FLOAT & !0xfff, 0x1000);
        e.set_global(MINUS_ONE_FLOAT, (-1.0f32).to_bits());
        let this = e.mem.alloc(0x74);
        let name = e.mem.alloc(4);
        e.mem.write(name, b"abc");
        e.mem.set_u32(this + 0x1c, name);
        e.mem.set_u32(this + 0x18, 0x0b1e);
        e.call_log = Some(vec![]);
        e.call(0x0068_9cb0, &args![this]);
        let log = e.call_log.take().unwrap();
        let w = written.borrow();
        assert_eq!(w.len(), 8);
        assert_eq!(w[0], b"KOLB".to_vec());
        assert_eq!(w[2], vec![3]);
        assert_eq!(w[3], b"abc".to_vec());
        assert_eq!(w[4][..4], 0.5f32.to_le_bytes());
        assert_eq!(w[5][..4], 7.0f32.to_le_bytes());
        assert_eq!(w[6], 1.5f32.to_le_bytes().to_vec());
        assert_eq!(w[7], vec![4, 0]);
        assert_eq!(
            calls_to(&log, MANAGER_SAVE_0055FE70),
            vec![vec![0x3a3a, (-1.0f32).to_bits()]]
        );
        assert_eq!(calls_to(&log, QUATERNION_FROM_ROTATION)[0][1], 0x7070);
        // The block size placeholder (at buffer + 4) counts what followed it.
        assert_eq!(
            u32::from(e.mem.u16(buffer + 4)),
            position.get() - (buffer + 4)
        );
    }

    /// The load setting: the save returns `bytes` in order for each read.
    fn load_setup(e: &mut Engine, reads: Vec<Vec<u8>>) {
        save_globals(e, 0);
        let queue = std::rc::Rc::new(std::cell::RefCell::new(reads));
        e.register_double(0x0085_79e0, move |e, a| {
            let next = queue.borrow_mut().remove(0);
            assert_eq!(next.len() as u32, a[2]);
            e.mem.write(a[1], &next);
            Ret::default()
        });
        e.register(FILL_MEMORY_00403D30, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        noop(
            e,
            &[
                MATRIX_INIT_006815C0,
                QUATERNION_TO_ROTATION,
                SET_TRANSLATE,
                SET_ROTATE,
                SET_SCALE,
                NI_AV_OBJECT_UPDATE_PROPERTIES,
                UPDATE_DATA_INIT_0043D410,
                OBJECT_UPDATE_00A59C60,
                NI_TIME_CONTROLLER_START_ANIMATIONS,
                MODEL_POINTER_DESTROY_0040C110,
                SKIP_BYTES_00857BD0,
                MANAGER_LOAD_0055FFA0,
            ],
        );
    }

    fn reads(count: u16) -> Vec<Vec<u8>> {
        vec![
            vec![2],
            b"ab".to_vec(),
            vec![0; 16],
            vec![0; 12],
            1.0f32.to_le_bytes().to_vec(),
            count.to_le_bytes().to_vec(),
        ]
    }

    #[test]
    fn load_game_fails_without_base_or_model_and_skips_the_manager() {
        let mut e = Engine::new();
        pointer_doubles(&mut e);
        load_setup(&mut e, reads(5));
        e.register(BS_TEMP_EFFECT_LOAD_GAME, |_, _| ret(0));
        let this = e.mem.alloc(0x74);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_9f80, &args![this]).u8(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SKIP_BYTES_00857BD0), vec![vec![0x5a5a, 5]]);
        assert!(calls_to(&log, MODEL_LOADER_LOAD_FILE).is_empty());
        // The base loads but the model does not.
        let mut e = Engine::new();
        pointer_doubles(&mut e);
        load_setup(&mut e, reads(0));
        e.register(BS_TEMP_EFFECT_LOAD_GAME, |_, _| ret(1));
        e.map(MODEL_LOADER & !0xfff, 0x1000);
        e.register(MODEL_LOADER_LOAD_FILE, |e, a| {
            assert_eq!(e.mem.cstr(a[1]), b"ab".to_vec());
            ret(0)
        });
        let this = e.mem.alloc(0x74);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_9f80, &args![this]).u8(), 0);
        assert!(calls_to(&e.call_log.take().unwrap(), SKIP_BYTES_00857BD0).is_empty());
    }

    #[test]
    fn load_game_rebuilds_the_object() {
        let mut e = Engine::new();
        pointer_doubles(&mut e);
        load_setup(&mut e, reads(3));
        e.register(BS_TEMP_EFFECT_LOAD_GAME, |_, _| ret(1));
        e.map(MODEL_LOADER & !0xfff, 0x1000);
        e.register(MODEL_LOADER_LOAD_FILE, |_, _| ret(0x0de1));
        e.register(NI_OBJECT_CLONE_OV2, |_, _| ret(0xc10e));
        let found = e.mem.alloc(4);
        e.mem.set_u32(found, 0xfa5e);
        e.register_double(MODEL_LOADER_FIND_MODEL, move |e, a| {
            e.mem.set_u32(a[2], found);
            Ret::default()
        });
        let node = e.mem.alloc(4);
        let mut slots = vec![0u32; 0x38];
        slots[0xdc / 4] = 0x0150_10dc;
        e.put_vtable(0x0150_0000, &slots);
        e.mem.set_u32(node, 0x0150_0000);
        e.register(0x0150_10dc, |_, _| Ret::default());
        e.register_double(CELL_ATTACH_NODE_005497A0, move |_, _| ret(node));
        e.register(POSITION_00825C00, |_, _| ret(2500));
        e.map(THOUSAND & !0xfff, 0x1000);
        e.set_global(THOUSAND, 1000.0f64);
        e.map(MINUS_ONE_FLOAT & !0xfff, 0x1000);
        e.set_global(MINUS_ONE_FLOAT, (-1.0f32).to_bits());
        e.register(FIRST_CONTROLLER, |_, _| ret(0xc0));
        e.register(FIND_CONTROLLER_00653270, |_, _| ret(0x3a3a));
        let this = e.mem.alloc(0x74);
        e.mem.set_u32(this + 0xc, 0xce11);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_9f80, &args![this]).u8(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0x18), 0xc10e);
        assert_eq!(e.mem.u32(this + 0x1c), 0xfa5e);
        assert_eq!(calls_to(&log, 0x0150_10dc), vec![vec![node, 0xc10e, 1]]);
        assert_eq!(
            calls_to(&log, CELL_ATTACH_NODE_005497A0),
            vec![vec![0xce11]]
        );
        assert_eq!(
            calls_to(&log, UPDATE_DATA_INIT_0043D410)[0][1..],
            [2.5f32.to_bits(), 1, 0]
        );
        assert_eq!(
            calls_to(&log, MANAGER_LOAD_0055FFA0),
            vec![vec![0x3a3a, 0xc10e, (-1.0f32).to_bits()]]
        );
        assert_eq!(calls_to(&log, SET_SCALE)[0][1], 1.0f32.to_bits());
        // Without a manager the load fails and the bytes are skipped.
        load_setup(&mut e, reads(3));
        e.register(BS_TEMP_EFFECT_LOAD_GAME, |_, _| ret(1));
        e.register(FIND_CONTROLLER_00653270, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_9f80, &args![this]).u8(), 0);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), SKIP_BYTES_00857BD0),
            vec![vec![0x5a5a, 3]]
        );
    }

    #[test]
    fn fn_0068a600_restarts_the_impact_sounds() {
        let mut e = Engine::new();
        pointer_doubles(&mut e);
        e.register(BS_SOUND_HANDLE_IS_VALID, |_, _| ret(0));
        e.register(BS_AUDIO_INSTANCE, |_, _| ret(0xa0d1));
        e.register(BS_AUDIO_GET_SOUND_HANDLE, |_, a| ret(a[1]));
        noop(
            &mut e,
            &[
                SOUND_HANDLE_ASSIGN_00418900,
                SOUND_HANDLE_DESTROY,
                BS_SOUND_HANDLE_SET_POSITION,
                BS_SOUND_HANDLE_SET_OBJECT_TO_FOLLOW,
                BS_SOUND_HANDLE_PLAY,
            ],
        );
        let position = e.mem.alloc(0xc);
        e.register_double(WORLD_POSITION_0045BB80, move |_, _| ret(position));
        e.register(FORM_ID_0084E3A0, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        let this = e.mem.alloc(0x74);
        // No object: only the stop.
        e.call_log = Some(vec![]);
        e.call(0x0068_a600, &args![this]);
        assert!(calls_to(&e.call_log.take().unwrap(), BS_AUDIO_INSTANCE).is_empty());
        e.mem.set_u32(this + 0x18, 0x0b1e);
        let data = e.mem.alloc(0x54);
        e.mem.set_u32(this + 0x58, data);
        let sound = e.mem.alloc(0x10);
        e.mem.set_u32(sound + 0xc, 0x50d);
        e.mem.set_u32(data + 0x50, sound);
        e.call_log = Some(vec![]);
        e.call(0x0068_a600, &args![this]);
        let log = e.call_log.take().unwrap();
        let handles = calls_to(&log, BS_AUDIO_GET_SOUND_HANDLE);
        assert_eq!(handles.len(), 1);
        assert_eq!((handles[0][2], handles[0][3]), (0x50d, 0x102));
        assert_eq!(
            calls_to(&log, BS_SOUND_HANDLE_PLAY),
            vec![vec![this + 0x68, 0]]
        );
        assert_eq!(
            calls_to(&log, BS_SOUND_HANDLE_SET_OBJECT_TO_FOLLOW),
            vec![vec![this + 0x68, 0x0b1e]]
        );
        // Both sounds.
        e.mem.set_u32(data + 0x4c, sound);
        e.call_log = Some(vec![]);
        e.call(0x0068_a600, &args![this]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), BS_SOUND_HANDLE_PLAY),
            vec![vec![this + 0x5c, 0], vec![this + 0x68, 0]]
        );
    }
}
