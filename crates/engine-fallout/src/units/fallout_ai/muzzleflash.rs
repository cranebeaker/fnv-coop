//! `fallout/ai/combat/muzzleflash.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `MuzzleFlash` (layout in `run_009ba550.rs`, where its constructor
//! `009bacb0` is translated): the flash node (`spAVObject`, +0x0C) and its
//! light (`spLight`, +0x10) shown for a while after a shot.
//!
//! `NiPointer` reads go through `00559450` (the pointer the smart pointer
//! holds) and assignments through `0066b0d0`; the getter `009611e0` reads
//! the word at +0x18 of whatever it is given (`pWeaponSource` of a muzzle
//! flash, the parent of a scene graph node). Functions that open a C++
//! exception frame (`FS:[0]`) carry a `Translation incomplete` marker: the
//! unwinding states are not translated.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::fallout_ai::run_009ba550::MuzzleFlash;

/// `NiPointer<T>` read: the pointer it holds (`this` is the smart pointer).
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer<T>` assignment `(this, pointer)`.
const NI_POINTER_SET: u32 = 0x0066_b0d0;
/// `NiPointer<T>` destructor (`this` is the smart pointer).
const NI_POINTER_DESTROY: u32 = 0x0045_cec0;
/// The word at +0x18 of `this` (see the module note).
const WORD_AT_18: u32 = 0x0096_11e0;
/// Deallocator (cdecl `(block)`).
const FREE: u32 = 0x0040_1030;
/// cdecl `(0)`: the scene's `ShadowSceneNode` (map: `fallout shared/tes.cpp`).
const SHADOW_SCENE_NODE: u32 = 0x0045_0b80;
/// `ShadowSceneNode::RemoveLight` (Xbox PDB) `(this, light)`.
const SHADOW_SCENE_NODE_REMOVE_LIGHT: u32 = 0x00b5_eed0;
/// `ShadowSceneNode::GetLight` (Xbox PDB) `(this, light)`.
const SHADOW_SCENE_NODE_GET_LIGHT: u32 = 0x00b5_b4a0;
/// `ShadowSceneNode::AddLight` (Xbox PDB) `(this, light, 1)`.
const SHADOW_SCENE_NODE_ADD_LIGHT: u32 = 0x00b5_c940;
/// `(light, float)`: given 1.0 while the flash is enabled, 0.0 otherwise.
const LIGHT_SET_VALUE: u32 = 0x0050_dd20;
/// `(projectile)`: the projectile's light form (or null).
const PROJECTILE_LIGHT: u32 = 0x004f_d3a0;
/// `TESObjectLIGH::GenDynamic` (Xbox PDB) `(this, 0, node, 1)`.
const LIGHT_GEN_DYNAMIC: u32 = 0x0050_d810;
/// `(projectile)`: a float in ST0, the flash duration.
const PROJECTILE_FLASH_DURATION: u32 = 0x0050_7e10;
/// `(node, word)`: given 1 when the flash is disabled (map: `tes.cpp`).
const NODE_SET_HIDDEN: u32 = 0x0045_0f90;
/// `TESObjectREFR::AddMasterParticleAddonNodes` (Xbox PDB), cdecl `(node)`.
const ADD_MASTER_PARTICLE_ADDON_NODES: u32 = 0x0057_8060;
/// `TESObjectREFR::RemoveMasterParticleAddonNodes` (Xbox PDB), cdecl `(node)`.
const REMOVE_MASTER_PARTICLE_ADDON_NODES: u32 = 0x0057_8170;
/// `(0x011f2250)`: a mode word of the object at `011f2250`; 4 selects the
/// time scaling below.
const MODE_OBJECT: u32 = 0x011f_2250;
const MODE_GET: u32 = 0x0044_ddc0;
/// A float in ST0 (map: `interfacemanager.cpp`) the elapsed time is divided by.
const TIME_DIVISOR: u32 = 0x0071_6440;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// The `float` `fEnableTimer` is reset to when a flash ends.
const ENABLE_TIMER_RESET: u32 = 0x0101_e2bc;
/// The player (`PlayerCharacter*` global).
const PLAYER: u32 = 0x011d_ea3c;
/// `(reference)`: the word at +0x40 of a reference.
const REFERENCE_WORD_40: u32 = 0x008d_6f30;
/// `TESObjectWEAP::GetFireNode` (Xbox PDB) `(this, node)`.
const WEAPON_GET_FIRE_NODE: u32 = 0x0052_5700;
/// `NiTransform` constructor (`this`), 0x34 bytes: rotation, translation,
/// scale.
const TRANSFORM_CONSTRUCT: u32 = 0x0047_6a80;
/// `(player)`: a flag selecting how the player's node and animation are found.
const PLAYER_FLAG: u32 = 0x0052_4d10;
/// `(player, 1)`: the player's node when the flag is clear.
const PLAYER_NODE: u32 = 0x0095_0bb0;
/// `(node)`: the node's world transform.
const NODE_WORLD_TRANSFORM: u32 = 0x0046_1130;
/// `NiMatrix3::Transpose` (Xbox PDB) `(this, result)`.
const MATRIX_TRANSPOSE: u32 = 0x0047_68c0;
/// `(point, result)`: point operation into `result` (map: `bgsdecalmanager.cpp`).
const POINT_NEGATED: u32 = 0x004a_0bd0;
/// `(matrix, result, point)`: matrix times point.
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
/// `(point, float)`: scales the point in place.
const POINT_SCALE: u32 = 0x0043_9180;
/// `(transform, result, other)`: transform product.
const TRANSFORM_TIMES: u32 = 0x0062_c250;
/// `PlayerCharacter::GetAnimation` (Xbox PDB) `(player, 1)`.
const PLAYER_GET_ANIMATION: u32 = 0x0095_0a60;
/// `(animation)`: a float in ST0, the time handed to the node update.
const ANIMATION_TIME: u32 = 0x0045_3700;
/// `(cell)`: the node `Position3D` attaches the flash to for other references.
const CELL_NODE: u32 = 0x0054_97a0;
/// `(void)`: true when not on the main thread (see `Position3D`).
const OFF_MAIN_THREAD: u32 = 0x008c_7aa0;
/// `(void)`: the task queue interface.
const TASK_QUEUE_INTERFACE: u32 = 0x0045_37b0;
/// `TaskQueueInterface::QueueNodeAttach` (Xbox PDB) `(this, node, parent, 1, 0)`.
const QUEUE_NODE_ATTACH: u32 = 0x0087_ac00;
/// `(this, node)` of the task queue interface, used by the destructor.
const QUEUE_NODE_DETACH: u32 = 0x0087_ac60;
/// `BSShaderUtil::RecursiveSetPropertyFadeAlpha` (Xbox PDB), cdecl `(node, float)`.
const RECURSIVE_SET_FADE_ALPHA: u32 = 0x00b6_bb30;
/// Node setters `(node, &translation)`, `(node, &rotation)`, `(node, scale)`.
const NODE_SET_TRANSLATE: u32 = 0x0044_0460;
const NODE_SET_ROTATE: u32 = 0x0043_fa80;
const NODE_SET_SCALE: u32 = 0x0044_0490;
/// Update data constructor `(this, float time, 0, 0)` and the node update
/// `(node, &data)`.
const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// cdecl `(text)`: prints a warning.
const PRINT_WARNING: u32 = 0x005b_5e40;
/// `"Trying to update a Muzzle Flash when not in the main thread"`.
const NOT_MAIN_THREAD_TEXT: u32 = 0x0109_0010;
/// Byte flag: the warning above is still to be printed (cleared once printed).
const NOT_MAIN_THREAD_WARN: u32 = 0x011a_59bc;
/// Process lists object and its `(this, source, reference)` method
/// `00978740` for references other than actors.
const PROCESS_LISTS: u32 = 0x011e_0e80;
const PROCESS_LISTS_MUZZLE_FLASH: u32 = 0x0097_8740;
/// `MiddleHighProcess::GetSavedAcquireObject` (map name of `008d8520`;
/// called on the reference).
const REFERENCE_PROCESS: u32 = 0x008d_8520;
/// The word at +0x14 of `this`.
const WORD_AT_14: u32 = 0x0082_5c00;
/// `(source)`: compared with the muzzle flash's word at +0x14.
const SOURCE_WORD_A: u32 = 0x0041_81e0;
/// `(source)`: the word at +0xF8, compared with the muzzle flash's +0x18.
const SOURCE_WORD_B: u32 = 0x0047_6c90;
/// Destructor callees: `009baee0(this)`, `0048cee0(obj)`, the global
/// `011c3b3c` and its `(this, value)` method `0045a5e0`.
const DESTRUCTOR_OBJECT: u32 = 0x009b_aee0;
const DESTRUCTOR_OBJECT_CHECK: u32 = 0x0048_cee0;
const DESTRUCTOR_TARGET: u32 = 0x011c_3b3c;
const DESTRUCTOR_TARGET_CALL: u32 = 0x0045_a5e0;

/// `FCOMP double 0.0; FNSTSW; TEST AH,0x41; JP skip`: runs when the value is
/// below or equal to the constant (false for NaN).
fn at_most_zero(e: &Engine, value: f64) -> bool {
    let zero: f64 = e.global(ZERO);
    value <= zero
}

fn flash_node(e: &mut Engine, this: Ptr<MuzzleFlash>) -> u32 {
    e.call(NI_POINTER_GET, &args![this.byte_add(0x0c)]).u32()
}

fn flash_light(e: &mut Engine, this: Ptr<MuzzleFlash>) -> u32 {
    e.call(NI_POINTER_GET, &args![this.byte_add(0x10)]).u32()
}

// Translated from 008d9f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MuzzleFlash::~MuzzleFlash` (Xbox PDB name; the body is the scalar
/// deleting destructor): runs the destructor `009badc0`, frees `this` when
/// bit 0 of `flags` is set, returns `this`.
pub fn muzzle_flash_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<MuzzleFlash>,
    flags: u32,
) -> Ptr<MuzzleFlash> {
    fn_009badc0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 009badc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009badc0 - C++ exception frame (FS:[0] unwinding states) not translated
/// The `MuzzleFlash` destructor: hands the value of `009baee0`'s object's
/// slot 0x14 to `0045a5e0` on the global at `011c3b3c` when `0048cee0`
/// says so; detaches the flash node from its parent (directly through the
/// parent's slot 0xE8 on the main thread, otherwise through the task queue
/// interface `0087ac60`); clears the node pointer, removes the light
/// (`009bb850`) and destroys both smart pointers.
pub fn fn_009badc0(e: &mut Engine, this: Ptr<MuzzleFlash>) {
    let object = e.call(DESTRUCTOR_OBJECT, &args![this]).u32();
    if e.call(DESTRUCTOR_OBJECT_CHECK, &args![object]).u32() != 0 {
        let value = e.vcall(object, 0x14, &args![]).u32();
        let target: u32 = e.global(DESTRUCTOR_TARGET);
        e.call(DESTRUCTOR_TARGET_CALL, &args![target, value]);
    }
    if flash_node(e, this) != 0 {
        let node = flash_node(e, this);
        if e.call(WORD_AT_18, &args![node]).u32() != 0 {
            if !e.call(OFF_MAIN_THREAD, &args![]).bool() {
                let node = flash_node(e, this);
                let parent = e.call(WORD_AT_18, &args![node]).u32();
                let node = flash_node(e, this);
                e.vcall(parent, 0xe8, &args![node]);
            } else {
                let node = flash_node(e, this);
                let queue = e.call(TASK_QUEUE_INTERFACE, &args![]).u32();
                e.call(QUEUE_NODE_DETACH, &args![queue, node]);
            }
        }
    }
    e.call(NI_POINTER_SET, &args![this.byte_add(0x0c), 0u32]);
    fn_009bb850(e, this);
    e.call(NI_POINTER_DESTROY, &args![this.byte_add(0x10)]);
    e.call(NI_POINTER_DESTROY, &args![this.byte_add(0x0c)]);
}

// Translated from 009bb080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MuzzleFlash::Update` (Xbox PDB): with a flash node, positions it
/// (`Position3D`), hides it while disabled, adds or removes the master
/// particle addon nodes when `bEnabled` changes against `bMPSEnabled`,
/// updates the light when asked, then counts the timers down by `elapsed`
/// (divided by `00716440`'s value when the mode word of `011f2250` is 4 and
/// that value is positive, or when `reference` answers slot 0x100 and its
/// +0xA4 object's slot 8 gives a positive count for 0x33). When the
/// duration runs out the flash is disabled, `fDurationTimer` reloaded from
/// the projectile, `fEnableTimer` reset and the light update requested.
pub fn muzzle_flash_update(e: &mut Engine, this: Ptr<MuzzleFlash>, elapsed: f32, reference: Ptr) {
    if flash_node(e, this) == 0 {
        return;
    }
    muzzle_flash_position_3d(e, this, reference);
    let hidden = !e.get(this, MuzzleFlash::bEnabled) as u32;
    let node = flash_node(e, this);
    e.call(NODE_SET_HIDDEN, &args![node, hidden]);

    if e.get(this, MuzzleFlash::bEnabled) && !e.get(this, MuzzleFlash::bMPSEnabled) {
        let node = flash_node(e, this);
        let root = e.vcall(node, 0x0c, &args![]).u32();
        e.call(ADD_MASTER_PARTICLE_ADDON_NODES, &args![root]);
        e.set(this, MuzzleFlash::bMPSEnabled, true);
    }
    if !e.get(this, MuzzleFlash::bEnabled) && e.get(this, MuzzleFlash::bMPSEnabled) {
        let node = flash_node(e, this);
        let root = e.vcall(node, 0x0c, &args![]).u32();
        e.call(REMOVE_MASTER_PARTICLE_ADDON_NODES, &args![root]);
        e.set(this, MuzzleFlash::bMPSEnabled, false);
    }
    if e.get(this, MuzzleFlash::bUpdateLight) {
        muzzle_flash_update_light(e, this);
    }

    let mut elapsed = elapsed;
    let scaled_by_mode = e.call(MODE_GET, &args![MODE_OBJECT]).u32() == 4 && {
        let divisor = e.call(TIME_DIVISOR, &args![]).f64();
        !at_most_zero(e, divisor)
    };
    if scaled_by_mode {
        let divisor = e.call(TIME_DIVISOR, &args![]).f64();
        elapsed = (elapsed as f64 / divisor) as f32;
    } else if e.vcall(reference.addr(), 0x100, &args![]).bool() {
        // `ECX = reference + 0xA4`, its own vtable's slot 8.
        let count = e
            .vcall(reference.addr().wrapping_add(0xa4), 0x08, &args![0x33u32])
            .i32();
        if count > 0 {
            let divisor = e.call(TIME_DIVISOR, &args![]).f64();
            elapsed = (elapsed as f64 / divisor) as f32;
        }
    }

    let enable_timer = e.get(this, MuzzleFlash::fEnableTimer);
    e.set(
        this,
        MuzzleFlash::fEnableTimer,
        (enable_timer as f64 - elapsed as f64) as f32,
    );
    if !e.get(this, MuzzleFlash::bEnabled) {
        return;
    }
    let duration = e.get(this, MuzzleFlash::fDurationTimer);
    let duration = (duration as f64 - elapsed as f64) as f32;
    e.set(this, MuzzleFlash::fDurationTimer, duration);
    let duration = e.get(this, MuzzleFlash::fDurationTimer) as f64;
    if at_most_zero(e, duration) {
        let projectile = e.get(this, MuzzleFlash::pProjectile);
        let reload = e.call(PROJECTILE_FLASH_DURATION, &args![projectile]).f64() as f32;
        e.set(this, MuzzleFlash::fDurationTimer, reload);
        let reset = x87_quiet(e.global(ENABLE_TIMER_RESET));
        e.set(this, MuzzleFlash::fEnableTimer, reset);
        e.set(this, MuzzleFlash::bEnabled, false);
        e.set(this, MuzzleFlash::bUpdateLight, true);
    }
}

/// `Position3D`'s stack frame (0x110 bytes below `EBP`): the locals it
/// passes by address, at their offsets from `EBP`.
struct Frame {
    base: u32,
}

impl Frame {
    const SIZE: u32 = 0x110;
    /// The address of `[EBP - offset]`.
    fn at(&self, offset: u32) -> u32 {
        self.base + Self::SIZE - offset
    }
}

/// `NiTransform` the flash node gets (`EBP-0x40`): rotation, translation at
/// +0x24, scale at +0x30.
const LOCAL_TRANSFORM: u32 = 0x40;
/// Inverse of the player node's world transform (`EBP-0x78`).
const INVERSE_TRANSFORM: u32 = 0x78;
/// Temporaries: the transposed rotation, the point results, the transform
/// product, the update data.
const TRANSPOSED: u32 = 0xac;
const NEGATED: u32 = 0xb8;
const ROTATED: u32 = 0xc4;
const PRODUCT: u32 = 0xf8;
const UPDATE_DATA: u32 = 0x88;

fn copy_words(e: &mut Engine, to: u32, from: u32, words: u32) {
    let bytes = e.mem.bytes(from, 4 * words);
    e.mem.write(to, &bytes);
}

// Translated from 009bb240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MuzzleFlash::Position3D` (Xbox PDB): puts the flash node at the fire
/// node of `reference`'s weapon. For an actor (slot 0x100) the fire node is
/// its slot 0x20C, otherwise `TESObjectWEAP::GetFireNode` of the weapon
/// source with `reference`'s slot 0x1D0 node. For the player the node's
/// transform is the fire node's world transform seen from the player's node
/// (inverse of its world transform: transposed rotation, inverted scale,
/// negated and rotated translation) and the node is updated with the
/// animation time afterwards; otherwise it is the world transform and the
/// parent is `005497a0` of `reference`'s word at +0x40. A node not yet under
/// that parent is attached (on the main thread through the parent's slot
/// 0xDC, otherwise queued) and its fade alpha set to 1.0. Off the main
/// thread the player update is skipped and a warning printed once.
pub fn muzzle_flash_position_3d(e: &mut Engine, this: Ptr<MuzzleFlash>, reference: Ptr) {
    if flash_node(e, this) == 0
        || reference.is_null()
        || e.call(REFERENCE_WORD_40, &args![reference]).u32() == 0
        || e.vcall(reference.addr(), 0x1d0, &args![]).u32() == 0
    {
        return;
    }
    let fire_node = if e.vcall(reference.addr(), 0x100, &args![]).bool() {
        e.vcall(reference.addr(), 0x20c, &args![]).u32()
    } else if e.call(WORD_AT_18, &args![this]).u32() != 0 {
        let root = e.vcall(reference.addr(), 0x1d0, &args![]).u32();
        let weapon = e.call(WORD_AT_18, &args![this]).u32();
        e.call(WEAPON_GET_FIRE_NODE, &args![weapon, root]).u32()
    } else {
        0
    };
    if fire_node == 0 {
        return;
    }
    e.with_stack(Frame::SIZE, |e, block| {
        let frame = Frame { base: block.addr() };
        position_with_frame(e, this, reference, fire_node, &frame);
    });
}

fn position_with_frame(
    e: &mut Engine,
    this: Ptr<MuzzleFlash>,
    reference: Ptr,
    fire_node: u32,
    frame: &Frame,
) {
    let local = frame.at(LOCAL_TRANSFORM);
    let inverse = frame.at(INVERSE_TRANSFORM);
    let mut player_update = false;
    let mut time = 0.0f32;

    e.call(TRANSFORM_CONSTRUCT, &args![local]);
    let player: u32 = e.global(PLAYER);
    let parent = if reference.addr() == player {
        let parent = if !e.call(PLAYER_FLAG, &args![player]).bool() {
            e.call(PLAYER_NODE, &args![player, 1u32]).u32()
        } else {
            e.vcall(player, 0x1d0, &args![]).u32()
        };
        e.call(TRANSFORM_CONSTRUCT, &args![inverse]);
        if parent != 0 {
            let world = e.call(NODE_WORLD_TRANSFORM, &args![parent]).u32();
            let transposed = e
                .call(MATRIX_TRANSPOSE, &args![world, frame.at(TRANSPOSED)])
                .u32();
            copy_words(e, inverse, transposed, 9);
            let world = e.call(NODE_WORLD_TRANSFORM, &args![parent]).u32();
            let scale = (1.0 / e.mem.f32(world + 0x30) as f64) as f32;
            e.mem.set_f32(inverse + 0x30, scale);
            let world = e.call(NODE_WORLD_TRANSFORM, &args![parent]).u32();
            let negated = e
                .call(POINT_NEGATED, &args![world + 0x24, frame.at(NEGATED)])
                .u32();
            copy_words(e, inverse + 0x24, negated, 3);
            let rotated = e
                .call(
                    MATRIX_TIMES_POINT,
                    &args![inverse, frame.at(ROTATED), inverse + 0x24],
                )
                .u32();
            copy_words(e, inverse + 0x24, rotated, 3);
            let scale = x87_quiet(e.mem.f32(inverse + 0x30));
            e.call(POINT_SCALE, &args![inverse + 0x24, scale]);
        }
        let world = e.call(NODE_WORLD_TRANSFORM, &args![fire_node]).u32();
        let product = e
            .call(TRANSFORM_TIMES, &args![inverse, frame.at(PRODUCT), world])
            .u32();
        copy_words(e, local, product, 13);
        player_update = true;
        if !e.call(PLAYER_FLAG, &args![player]).bool() {
            time = if e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32() != 0 {
                let animation = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
                e.call(ANIMATION_TIME, &args![animation]).f64() as f32
            } else {
                0.0
            };
        } else {
            time = if e.vcall(player, 0x1e4, &args![]).u32() != 0 {
                let animation = e.vcall(player, 0x1e4, &args![]).u32();
                e.call(ANIMATION_TIME, &args![animation]).f64() as f32
            } else {
                0.0
            };
        }
        parent
    } else {
        let world = e.call(NODE_WORLD_TRANSFORM, &args![fire_node]).u32();
        copy_words(e, local, world, 13);
        let cell = e.call(REFERENCE_WORD_40, &args![reference]).u32();
        e.call(CELL_NODE, &args![cell]).u32()
    };

    if parent != 0 {
        let node = flash_node(e, this);
        if parent != e.call(WORD_AT_18, &args![node]).u32() {
            if !e.call(OFF_MAIN_THREAD, &args![]).bool() {
                let node = flash_node(e, this);
                e.vcall(parent, 0xdc, &args![node, 1u32]);
            } else {
                let node = flash_node(e, this);
                let queue = e.call(TASK_QUEUE_INTERFACE, &args![]).u32();
                e.call(QUEUE_NODE_ATTACH, &args![queue, node, parent, 1u32, 0u32]);
            }
            let node = flash_node(e, this);
            let root = e.vcall(node, 0x0c, &args![]).u32();
            e.call(RECURSIVE_SET_FADE_ALPHA, &args![root, 1.0f32]);
        }
    }

    let node = flash_node(e, this);
    e.call(NODE_SET_TRANSLATE, &args![node, local + 0x24]);
    let node = flash_node(e, this);
    e.call(NODE_SET_ROTATE, &args![node, local]);
    let scale = x87_quiet(e.mem.f32(local + 0x30));
    let node = flash_node(e, this);
    e.call(NODE_SET_SCALE, &args![node, scale]);

    if player_update {
        if !e.call(OFF_MAIN_THREAD, &args![]).bool() {
            let data = frame.at(UPDATE_DATA);
            e.call(
                UPDATE_DATA_CONSTRUCT,
                &args![data, x87_quiet(time), 0u32, 0u32],
            );
            let node = flash_node(e, this);
            e.call(NODE_UPDATE, &args![node, data]);
        } else if e.global::<u8>(NOT_MAIN_THREAD_WARN) != 0 {
            e.call(PRINT_WARNING, &args![NOT_MAIN_THREAD_TEXT]);
            e.set_global(NOT_MAIN_THREAD_WARN, 0u8);
        }
    }
}

// Translated from 009bb690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fires the flash once `fEnableTimer` has run out (at most 0.0, not NaN):
/// sets `bEnabled` and `bUpdateLight` and loads `fDurationTimer` from the
/// projectile (`00507e10`).
pub fn fn_009bb690(e: &mut Engine, this: Ptr<MuzzleFlash>) {
    let enable_timer = e.get(this, MuzzleFlash::fEnableTimer) as f64;
    if !at_most_zero(e, enable_timer) {
        return;
    }
    e.set(this, MuzzleFlash::bEnabled, true);
    e.set(this, MuzzleFlash::bUpdateLight, true);
    let projectile = e.get(this, MuzzleFlash::pProjectile);
    let duration = e.call(PROJECTILE_FLASH_DURATION, &args![projectile]).f64() as f32;
    e.set(this, MuzzleFlash::fDurationTimer, duration);
}

// Translated from 009bb6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(reference, source)`: with both non-null, for a non-actor
/// (slot 0x100 false) hands them to the process lists (`00978740(source,
/// reference)`). For an actor, takes its process (`008d8520`); unless the
/// process's muzzle flash (slot 0x6B8) already matches `source` (its +0x14
/// word equals `004181e0(source)` and its +0x18 word equals
/// `00476c90(source)`), calls the process's slot 0x6C4 and slot 0x6B4
/// `(source, reference)`; then fires the process's muzzle flash
/// (`009bb690`) if it has one.
pub fn fn_009bb6d0(e: &mut Engine, reference: Ptr, source: Ptr) {
    if reference.is_null() || source.is_null() {
        return;
    }
    if !e.vcall(reference.addr(), 0x100, &args![]).bool() {
        e.call(
            PROCESS_LISTS_MUZZLE_FLASH,
            &args![PROCESS_LISTS, source, reference],
        );
        return;
    }
    let process = e.call(REFERENCE_PROCESS, &args![reference]).u32();
    if process == 0 {
        return;
    }
    let mut matches = false;
    if e.vcall(process, 0x6b8, &args![]).u32() != 0 {
        let flash = e.vcall(process, 0x6b8, &args![]).u32();
        let flash_word = e.call(WORD_AT_14, &args![flash]).u32();
        if flash_word == e.call(SOURCE_WORD_A, &args![source]).u32() {
            let flash = e.vcall(process, 0x6b8, &args![]).u32();
            let flash_word = e.call(WORD_AT_18, &args![flash]).u32();
            matches = flash_word == e.call(SOURCE_WORD_B, &args![source]).u32();
        }
    }
    if !matches {
        e.vcall(process, 0x6c4, &args![]);
        e.vcall(process, 0x6b4, &args![source, reference]);
    }
    if e.vcall(process, 0x6b8, &args![]).u32() != 0 {
        let flash = e.vcall(process, 0x6b8, &args![]).u32();
        fn_009bb690(e, Ptr::new(flash));
    }
}

// Translated from 009bb7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the flash light: with a flash node and a light form on the
/// projectile (`004fd3a0`), `TESObjectLIGH::GenDynamic(form, 0, node, 1)`
/// into `spLight`.
pub fn fn_009bb7f0(e: &mut Engine, this: Ptr<MuzzleFlash>) {
    if flash_node(e, this) == 0 {
        return;
    }
    let projectile = e.get(this, MuzzleFlash::pProjectile);
    if e.call(PROJECTILE_LIGHT, &args![projectile]).u32() == 0 {
        return;
    }
    let node = flash_node(e, this);
    let projectile = e.get(this, MuzzleFlash::pProjectile);
    let form = e.call(PROJECTILE_LIGHT, &args![projectile]).u32();
    let light = e
        .call(LIGHT_GEN_DYNAMIC, &args![form, 0u32, node, 1u32])
        .u32();
    e.call(NI_POINTER_SET, &args![this.byte_add(0x10), light]);
}

// Translated from 009bb850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the flash light: with a light, takes it off the shadow scene node
/// and clears `spLight`.
pub fn fn_009bb850(e: &mut Engine, this: Ptr<MuzzleFlash>) {
    if flash_light(e, this) == 0 {
        return;
    }
    let light = flash_light(e, this);
    let scene = e.call(SHADOW_SCENE_NODE, &args![0u32]).u32();
    e.call(SHADOW_SCENE_NODE_REMOVE_LIGHT, &args![scene, light]);
    e.call(NI_POINTER_SET, &args![this.byte_add(0x10), 0u32]);
}

// Translated from 009bb8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MuzzleFlash::UpdateLight` (Xbox PDB): with a light, adds it to the
/// shadow scene node while the flash is enabled (if not there yet) and
/// gives it 1.0 when enabled, 0.0 otherwise (`0050dd20`).
pub fn muzzle_flash_update_light(e: &mut Engine, this: Ptr<MuzzleFlash>) {
    if flash_light(e, this) == 0 {
        return;
    }
    if e.get(this, MuzzleFlash::bEnabled) {
        let light = flash_light(e, this);
        let scene = e.call(SHADOW_SCENE_NODE, &args![0u32]).u32();
        if e.call(SHADOW_SCENE_NODE_GET_LIGHT, &args![scene, light])
            .u32()
            == 0
        {
            let light = flash_light(e, this);
            let scene = e.call(SHADOW_SCENE_NODE, &args![0u32]).u32();
            e.call(SHADOW_SCENE_NODE_ADD_LIGHT, &args![scene, light, 1u32]);
        }
    }
    let value = if e.get(this, MuzzleFlash::bEnabled) {
        1.0f32
    } else {
        0.0
    };
    let light = flash_light(e, this);
    e.call(LIGHT_SET_VALUE, &args![light, value]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x008d9f70,
            muzzle_flash_scalar_deleting_destructor(Ptr<MuzzleFlash>, u32) -> Ptr<MuzzleFlash>
        ),
        entry!(0x009badc0, fn_009badc0(Ptr<MuzzleFlash>)),
        entry!(0x009bb080, muzzle_flash_update(Ptr<MuzzleFlash>, f32, Ptr)),
        entry!(0x009bb240, muzzle_flash_position_3d(Ptr<MuzzleFlash>, Ptr)),
        entry!(0x009bb690, fn_009bb690(Ptr<MuzzleFlash>)),
        entry!(0x009bb6d0, fn_009bb6d0(Ptr, Ptr)),
        entry!(0x009bb7f0, fn_009bb7f0(Ptr<MuzzleFlash>)),
        entry!(0x009bb850, fn_009bb850(Ptr<MuzzleFlash>)),
        entry!(0x009bb8a0, muzzle_flash_update_light(Ptr<MuzzleFlash>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| ret(value));
    }

    fn stub_float(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    /// Doubles for every callee outside this file (0 by default; the smart
    /// pointer read returns the word it points at, the +0x18 getter the word
    /// there) and the globals' pages.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            NI_POINTER_SET,
            NI_POINTER_DESTROY,
            FREE,
            SHADOW_SCENE_NODE,
            SHADOW_SCENE_NODE_REMOVE_LIGHT,
            SHADOW_SCENE_NODE_GET_LIGHT,
            SHADOW_SCENE_NODE_ADD_LIGHT,
            LIGHT_SET_VALUE,
            PROJECTILE_LIGHT,
            LIGHT_GEN_DYNAMIC,
            PROJECTILE_FLASH_DURATION,
            NODE_SET_HIDDEN,
            ADD_MASTER_PARTICLE_ADDON_NODES,
            REMOVE_MASTER_PARTICLE_ADDON_NODES,
            MODE_GET,
            TIME_DIVISOR,
            REFERENCE_WORD_40,
            WEAPON_GET_FIRE_NODE,
            TRANSFORM_CONSTRUCT,
            PLAYER_FLAG,
            PLAYER_NODE,
            MATRIX_TRANSPOSE,
            POINT_NEGATED,
            MATRIX_TIMES_POINT,
            POINT_SCALE,
            TRANSFORM_TIMES,
            PLAYER_GET_ANIMATION,
            ANIMATION_TIME,
            CELL_NODE,
            OFF_MAIN_THREAD,
            TASK_QUEUE_INTERFACE,
            QUEUE_NODE_ATTACH,
            QUEUE_NODE_DETACH,
            RECURSIVE_SET_FADE_ALPHA,
            NODE_SET_TRANSLATE,
            NODE_SET_ROTATE,
            NODE_SET_SCALE,
            UPDATE_DATA_CONSTRUCT,
            NODE_UPDATE,
            PRINT_WARNING,
            PROCESS_LISTS_MUZZLE_FLASH,
            REFERENCE_PROCESS,
            WORD_AT_14,
            SOURCE_WORD_A,
            SOURCE_WORD_B,
            DESTRUCTOR_OBJECT,
            DESTRUCTOR_OBJECT_CHECK,
            DESTRUCTOR_TARGET_CALL,
            NODE_WORLD_TRANSFORM,
        ] {
            stub(&mut e, addr, 0);
        }
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(WORD_AT_18, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        for page in [
            0x0101_2000,
            0x0101_e000,
            0x011d_e000,
            0x011a_5000,
            0x011c_3000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO, 0.0f64);
        e.set_global(ENABLE_TIMER_RESET, 0.1f32);
        e
    }

    fn slot_target(vtable: u32, slot: u32) -> u32 {
        0x7000_0000 + (vtable & 0xffff) * 0x1000 + slot
    }

    /// An object (0x100 bytes) whose vtable at `vtable` has the given slots,
    /// each answered by a double returning the paired value.
    fn object(e: &mut Engine, vtable: u32, answers: &[(u32, u32)]) -> u32 {
        let size = answers.iter().map(|(slot, _)| slot + 4).max().unwrap_or(4);
        e.map(vtable, size);
        for (slot, value) in answers {
            let target = slot_target(vtable, *slot);
            e.mem.set_u32(vtable + slot, target);
            stub(e, target, *value);
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn flash(e: &mut Engine, node: u32, light: u32) -> Ptr<MuzzleFlash> {
        let this: Ptr<MuzzleFlash> = e.new_object();
        e.set(this, MuzzleFlash::spAVObject, Ptr::new(node));
        e.set(this, MuzzleFlash::spLight, Ptr::new(light));
        e.set(this, MuzzleFlash::pProjectile, Ptr::new(0x5000));
        this
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_0() {
        let mut e = engine();
        let this = flash(&mut e, 0, 0);
        e.call_log = Some(vec![]);
        let back = e.call(0x008d_9f70, &args![this, 1u32]).u32();
        assert_eq!(back, this.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, FREE), vec![vec![this.addr()]]);
        assert_eq!(calls(&log, NI_POINTER_DESTROY).len(), 2);

        e.call_log = Some(vec![]);
        e.call(0x008d_9f70, &args![this, 2u32]);
        assert!(calls(&e.call_log.take().unwrap(), FREE).is_empty());
    }

    #[test]
    fn destructor_detaches_the_node_and_removes_the_light() {
        let mut e = engine();
        let parent = object(&mut e, 0x0200_1000, &[(0xe8, 0)]);
        let node = e.mem.alloc(0x20);
        e.mem.set_u32(node + 0x18, parent);
        let this = flash(&mut e, node, 0x6000);
        let holder = object(&mut e, 0x0200_2000, &[(0x14, 0x77)]);
        stub(&mut e, DESTRUCTOR_OBJECT, holder);
        stub(&mut e, DESTRUCTOR_OBJECT_CHECK, 1);
        e.set_global(DESTRUCTOR_TARGET, 0x4321u32);
        stub(&mut e, SHADOW_SCENE_NODE, 0x9000);
        e.call_log = Some(vec![]);
        e.call(0x009b_adc0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, DESTRUCTOR_TARGET_CALL),
            vec![vec![0x4321, 0x77]]
        );
        // Main thread: the parent's slot 0xE8 with the node.
        assert_eq!(
            calls(&log, slot_target(0x0200_1000, 0xe8)),
            vec![vec![parent, node]]
        );
        assert!(calls(&log, QUEUE_NODE_DETACH).is_empty());
        // The light goes off the scene and both pointers are cleared.
        assert_eq!(
            calls(&log, SHADOW_SCENE_NODE_REMOVE_LIGHT),
            vec![vec![0x9000, 0x6000]]
        );
        let sets = calls(&log, NI_POINTER_SET);
        assert_eq!(sets[0], vec![this.addr() + 0x0c, 0]);
        assert_eq!(sets[1], vec![this.addr() + 0x10, 0]);
        assert_eq!(
            calls(&log, NI_POINTER_DESTROY),
            vec![vec![this.addr() + 0x10], vec![this.addr() + 0x0c]]
        );

        // Off the main thread the task queue detaches it.
        stub(&mut e, OFF_MAIN_THREAD, 1);
        stub(&mut e, TASK_QUEUE_INTERFACE, 0x8800);
        e.call_log = Some(vec![]);
        e.call(0x009b_adc0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, QUEUE_NODE_DETACH), vec![vec![0x8800, node]]);
        assert!(calls(&log, slot_target(0x0200_1000, 0xe8)).is_empty());
    }

    #[test]
    fn update_counts_timers_down_and_ends_the_flash() {
        let mut e = engine();
        let node = object(&mut e, 0x0200_3000, &[(0x0c, 0x3333)]);
        let reference = object(&mut e, 0x0200_4000, &[(0x100, 0), (0x1d0, 0)]);
        let this = flash(&mut e, node, 0);
        e.set(this, MuzzleFlash::bEnabled, true);
        e.set(this, MuzzleFlash::fEnableTimer, 1.0);
        e.set(this, MuzzleFlash::fDurationTimer, 0.5);
        stub_float(&mut e, PROJECTILE_FLASH_DURATION, 0.25);
        e.call_log = Some(vec![]);
        e.call(0x009b_b080, &args![this, 0.5f32, Ptr::<()>::new(reference)]);
        let log = e.call_log.take().unwrap();
        // Shown (0), the addon nodes added once.
        assert_eq!(calls(&log, NODE_SET_HIDDEN), vec![vec![node, 0]]);
        assert_eq!(
            calls(&log, ADD_MASTER_PARTICLE_ADDON_NODES),
            vec![vec![0x3333]]
        );
        assert!(e.get(this, MuzzleFlash::bMPSEnabled));
        // The duration ran out (0.5 - 0.5 = 0): reloaded and disabled.
        assert_eq!(e.get(this, MuzzleFlash::fDurationTimer), 0.25);
        assert_eq!(e.get(this, MuzzleFlash::fEnableTimer), 0.1);
        assert!(!e.get(this, MuzzleFlash::bEnabled));
        assert!(e.get(this, MuzzleFlash::bUpdateLight));

        // Disabled now: hidden, addon nodes removed, only the enable timer
        // runs; mode 4 with a positive divisor scales the time.
        stub(&mut e, MODE_GET, 4);
        stub_float(&mut e, TIME_DIVISOR, 2.0);
        e.set(this, MuzzleFlash::bUpdateLight, false);
        e.call_log = Some(vec![]);
        e.call(0x009b_b080, &args![this, 0.5f32, Ptr::<()>::new(reference)]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, NODE_SET_HIDDEN), vec![vec![node, 1]]);
        assert_eq!(
            calls(&log, REMOVE_MASTER_PARTICLE_ADDON_NODES),
            vec![vec![0x3333]]
        );
        assert!(!e.get(this, MuzzleFlash::bMPSEnabled));
        assert_eq!(
            e.get(this, MuzzleFlash::fEnableTimer),
            (0.1f32 as f64 - 0.25) as f32
        );
        assert_eq!(e.get(this, MuzzleFlash::fDurationTimer), 0.25);
    }

    #[test]
    fn update_scales_time_for_actors_with_a_positive_count() {
        let mut e = engine();
        let node = object(&mut e, 0x0200_5000, &[(0x0c, 0)]);
        let reference = object(&mut e, 0x0200_6000, &[(0x100, 1), (0x1d0, 0)]);
        // The +0xA4 sub-object with its own vtable (slot 8).
        let sub_vtable = 0x0200_7000;
        e.map(sub_vtable, 0x10);
        e.mem.set_u32(sub_vtable + 8, slot_target(sub_vtable, 8));
        stub(&mut e, slot_target(sub_vtable, 8), 2);
        e.mem.set_u32(reference + 0xa4, sub_vtable);
        let this = flash(&mut e, node, 0);
        e.set(this, MuzzleFlash::fEnableTimer, 1.0);
        // Mode 4 but the divisor is not positive: the actor branch decides.
        stub(&mut e, MODE_GET, 4);
        stub_float(&mut e, TIME_DIVISOR, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x009b_b080, &args![this, 0.5f32, Ptr::<()>::new(reference)]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, slot_target(sub_vtable, 8)),
            vec![vec![reference + 0xa4, 0x33]]
        );
        // 0.5 / 0.0 is infinity.
        assert_eq!(e.get(this, MuzzleFlash::fEnableTimer), f32::NEG_INFINITY);

        // Without a node nothing happens.
        let bare = flash(&mut e, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x009b_b080, &args![bare, 0.5f32, Ptr::<()>::new(reference)]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn position_3d_places_the_node_at_another_references_fire_node() {
        let mut e = engine();
        let node = object(&mut e, 0x0200_e000, &[(0x0c, 0x0c0c)]);
        let reference = object(
            &mut e,
            0x0200_8000,
            &[(0x100, 1), (0x1d0, 0x1234), (0x20c, 0x2222)],
        );
        let parent = object(&mut e, 0x0200_9000, &[(0xdc, 0)]);
        let this = flash(&mut e, node, 0);
        stub(&mut e, REFERENCE_WORD_40, 0x4040);
        stub(&mut e, CELL_NODE, parent);
        // The fire node's world transform: 13 words 1..13.
        let world = e.mem.alloc(0x34);
        for i in 0..13 {
            e.mem.set_f32(world + 4 * i, (i + 1) as f32);
        }
        stub(&mut e, NODE_WORLD_TRANSFORM, world);
        e.call_log = Some(vec![]);
        e.call(0x009b_b240, &args![this, Ptr::<()>::new(reference)]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, NODE_WORLD_TRANSFORM), vec![vec![0x2222]]);
        assert_eq!(calls(&log, CELL_NODE), vec![vec![0x4040]]);
        assert_eq!(
            calls(&log, slot_target(0x0200_9000, 0xdc)),
            vec![vec![parent, node, 1]]
        );
        assert_eq!(
            calls(&log, RECURSIVE_SET_FADE_ALPHA),
            vec![vec![0x0c0c, 1.0f32.to_bits()]]
        );
        let rotate = calls(&log, NODE_SET_ROTATE);
        let translate = calls(&log, NODE_SET_TRANSLATE);
        assert_eq!(translate[0][1], rotate[0][1] + 0x24);
        assert_eq!(e.mem.f32(rotate[0][1]), 1.0);
        assert_eq!(
            calls(&log, NODE_SET_SCALE),
            vec![vec![node, 13.0f32.to_bits()]]
        );
        // Not the player: no node update.
        assert!(calls(&log, NODE_UPDATE).is_empty());

        // Already under that parent: not attached again.
        e.mem.set_u32(node + 0x18, parent);
        e.call_log = Some(vec![]);
        e.call(0x009b_b240, &args![this, Ptr::<()>::new(reference)]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, slot_target(0x0200_9000, 0xdc)).is_empty());

        // No word at +0x40: nothing.
        stub(&mut e, REFERENCE_WORD_40, 0);
        e.call_log = Some(vec![]);
        e.call(0x009b_b240, &args![this, Ptr::<()>::new(reference)]);
        assert!(calls(&e.call_log.take().unwrap(), NODE_SET_SCALE).is_empty());
    }

    #[test]
    fn position_3d_for_the_player_uses_the_inverse_node_transform() {
        let mut e = engine();
        let node = object(&mut e, 0x0200_f000, &[(0x0c, 0)]);
        let player = object(&mut e, 0x0200_a000, &[(0x100, 0), (0x1d0, 0x1234)]);
        e.set_global(PLAYER, player);
        let player_node = object(&mut e, 0x0201_0000, &[(0xdc, 0)]);
        let weapon = 0x7777;
        let this = flash(&mut e, node, 0);
        e.set(this, MuzzleFlash::pWeaponSource, Ptr::new(weapon));
        stub(&mut e, REFERENCE_WORD_40, 0x4040);
        stub(&mut e, WEAPON_GET_FIRE_NODE, 0x2222);
        stub(&mut e, PLAYER_NODE, player_node);
        stub(&mut e, PLAYER_GET_ANIMATION, 0x3030);
        stub_float(&mut e, ANIMATION_TIME, 1.5);
        let world = e.mem.alloc(0x34);
        e.mem.set_f32(world + 0x30, 4.0);
        stub(&mut e, NODE_WORLD_TRANSFORM, world);
        let scratch = e.mem.alloc(0x34);
        e.mem.set_f32(scratch + 0x30, 2.0);
        for addr in [
            MATRIX_TRANSPOSE,
            POINT_NEGATED,
            MATRIX_TIMES_POINT,
            TRANSFORM_TIMES,
        ] {
            stub(&mut e, addr, scratch);
        }
        e.call_log = Some(vec![]);
        e.call(0x009b_b240, &args![this, Ptr::<()>::new(player)]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, WEAPON_GET_FIRE_NODE),
            vec![vec![weapon, 0x1234]]
        );
        assert_eq!(calls(&log, PLAYER_NODE), vec![vec![player, 1]]);
        assert_eq!(
            calls(&log, slot_target(0x0201_0000, 0xdc)),
            vec![vec![player_node, node, 1]]
        );
        // The inverse scale 1/4 scales the inverse translation.
        let scale = calls(&log, POINT_SCALE);
        assert_eq!(scale[0][1], 0.25f32.to_bits());
        let product = calls(&log, TRANSFORM_TIMES);
        assert_eq!(product[0][2], world);
        assert_eq!(scale[0][0], product[0][0] + 0x24);
        // The product (scale 2.0) becomes the node's transform.
        assert_eq!(
            calls(&log, NODE_SET_SCALE),
            vec![vec![node, 2.0f32.to_bits()]]
        );
        // The player's update with the animation time.
        let data = calls(&log, UPDATE_DATA_CONSTRUCT);
        assert_eq!(&data[0][1..], &[1.5f32.to_bits(), 0, 0]);
        assert_eq!(calls(&log, NODE_UPDATE), vec![vec![node, data[0][0]]]);

        // Off the main thread: queued attach, warning once, no update.
        stub(&mut e, OFF_MAIN_THREAD, 1);
        stub(&mut e, TASK_QUEUE_INTERFACE, 0x8800);
        e.set_global(NOT_MAIN_THREAD_WARN, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x009b_b240, &args![this, Ptr::<()>::new(player)]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, QUEUE_NODE_ATTACH),
            vec![vec![0x8800, node, player_node, 1, 0]]
        );
        assert_eq!(calls(&log, PRINT_WARNING), vec![vec![NOT_MAIN_THREAD_TEXT]]);
        assert_eq!(e.global::<u8>(NOT_MAIN_THREAD_WARN), 0);
        assert!(calls(&log, NODE_UPDATE).is_empty());
    }

    #[test]
    fn fire_starts_when_the_enable_timer_ran_out() {
        let mut e = engine();
        stub_float(&mut e, PROJECTILE_FLASH_DURATION, 0.75);
        for (timer, fires) in [
            (0.0f32, true),
            (-1.0, true),
            (0.5, false),
            (f32::NAN, false),
        ] {
            let this = flash(&mut e, 0, 0);
            e.set(this, MuzzleFlash::fEnableTimer, timer);
            e.call(0x009b_b690, &args![this]);
            assert_eq!(e.get(this, MuzzleFlash::bEnabled), fires);
            assert_eq!(e.get(this, MuzzleFlash::bUpdateLight), fires);
            let duration = if fires { 0.75 } else { 0.0 };
            assert_eq!(e.get(this, MuzzleFlash::fDurationTimer), duration);
        }
    }

    #[test]
    fn start_flash_goes_to_the_process_or_the_process_lists() {
        let mut e = engine();
        // Not an actor: the process lists get (source, reference).
        let thing = object(&mut e, 0x0200_b000, &[(0x100, 0)]);
        e.call_log = Some(vec![]);
        e.call(0x009b_b6d0, &args![thing, 0x5555u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, PROCESS_LISTS_MUZZLE_FLASH),
            vec![vec![PROCESS_LISTS, 0x5555, thing]]
        );

        // An actor whose process has a matching muzzle flash: only fired.
        let flash_ptr = flash(&mut e, 0, 0);
        e.set(flash_ptr, MuzzleFlash::fEnableTimer, -1.0);
        e.mem.set_u32(flash_ptr.addr() + 0x18, 0x18);
        let process = object(
            &mut e,
            0x0200_c000,
            &[(0x6b4, 0), (0x6b8, flash_ptr.addr()), (0x6c4, 0)],
        );
        let actor = object(&mut e, 0x0200_d000, &[(0x100, 1)]);
        stub(&mut e, REFERENCE_PROCESS, process);
        stub(&mut e, WORD_AT_14, 0x14);
        stub(&mut e, SOURCE_WORD_A, 0x14);
        stub(&mut e, SOURCE_WORD_B, 0x18);
        e.call_log = Some(vec![]);
        e.call(0x009b_b6d0, &args![actor, 0x5555u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, slot_target(0x0200_c000, 0x6b4)).is_empty());
        assert!(e.get(flash_ptr, MuzzleFlash::bEnabled));

        // Not matching: the process makes a new one (slot 0x6B4).
        stub(&mut e, SOURCE_WORD_B, 0x19);
        e.call_log = Some(vec![]);
        e.call(0x009b_b6d0, &args![actor, 0x5555u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, slot_target(0x0200_c000, 0x6c4)).len(), 1);
        assert_eq!(
            calls(&log, slot_target(0x0200_c000, 0x6b4)),
            vec![vec![process, 0x5555, actor]]
        );

        // A null source: nothing.
        e.call_log = Some(vec![]);
        e.call(0x009b_b6d0, &args![actor, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn make_light_generates_it_from_the_projectile() {
        let mut e = engine();
        let this = flash(&mut e, 0x4444, 0);
        stub(&mut e, PROJECTILE_LIGHT, 0x6666);
        stub(&mut e, LIGHT_GEN_DYNAMIC, 0x7070);
        e.call_log = Some(vec![]);
        e.call(0x009b_b7f0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, LIGHT_GEN_DYNAMIC),
            vec![vec![0x6666, 0, 0x4444, 1]]
        );
        assert_eq!(
            calls(&log, NI_POINTER_SET),
            vec![vec![this.addr() + 0x10, 0x7070]]
        );
        // No light form: nothing generated.
        stub(&mut e, PROJECTILE_LIGHT, 0);
        e.call_log = Some(vec![]);
        e.call(0x009b_b7f0, &args![this]);
        assert!(calls(&e.call_log.take().unwrap(), LIGHT_GEN_DYNAMIC).is_empty());
    }

    #[test]
    fn remove_light_takes_it_off_the_scene() {
        let mut e = engine();
        stub(&mut e, SHADOW_SCENE_NODE, 0x9000);
        let this = flash(&mut e, 0, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x009b_b850, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SHADOW_SCENE_NODE), vec![vec![0]]);
        assert_eq!(
            calls(&log, SHADOW_SCENE_NODE_REMOVE_LIGHT),
            vec![vec![0x9000, 0x6000]]
        );
        assert_eq!(
            calls(&log, NI_POINTER_SET),
            vec![vec![this.addr() + 0x10, 0]]
        );
        let bare = flash(&mut e, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x009b_b850, &args![bare]);
        assert!(calls(&e.call_log.take().unwrap(), NI_POINTER_SET).is_empty());
    }

    #[test]
    fn update_light_adds_it_while_enabled_and_sets_its_value() {
        let mut e = engine();
        stub(&mut e, SHADOW_SCENE_NODE, 0x9000);
        let this = flash(&mut e, 0, 0x6000);
        e.set(this, MuzzleFlash::bEnabled, true);
        e.call_log = Some(vec![]);
        e.call(0x009b_b8a0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, SHADOW_SCENE_NODE_ADD_LIGHT),
            vec![vec![0x9000, 0x6000, 1]]
        );
        assert_eq!(
            calls(&log, LIGHT_SET_VALUE),
            vec![vec![0x6000, 1.0f32.to_bits()]]
        );
        // Already in the scene: not added again.
        stub(&mut e, SHADOW_SCENE_NODE_GET_LIGHT, 1);
        e.call_log = Some(vec![]);
        e.call(0x009b_b8a0, &args![this]);
        assert!(calls(&e.call_log.take().unwrap(), SHADOW_SCENE_NODE_ADD_LIGHT).is_empty());
        // Disabled: 0.0, no scene lookup.
        e.set(this, MuzzleFlash::bEnabled, false);
        e.call_log = Some(vec![]);
        e.call(0x009b_b8a0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, SHADOW_SCENE_NODE_GET_LIGHT).is_empty());
        assert_eq!(calls(&log, LIGHT_SET_VALUE), vec![vec![0x6000, 0]]);
    }
}
