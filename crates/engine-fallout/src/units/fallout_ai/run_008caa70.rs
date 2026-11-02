//! `fallout/ai/run_008caa70` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008cab00 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0044fb00`, which calls the import slot
/// `00fdf058` (Ghidra: `DeleteCriticalSection`) on `this`, and returns nothing. Parameters:
/// one word, `this`; the disassembly of the function ends in a plain `RET` and passes only
/// `ECX` on. `0044fb00` ends right after that call; `DeleteCriticalSection` returns nothing
/// and Ghidra types `0044fb00` void.
pub fn fn_008cab00(e: &mut Engine, this: Ptr) {
    e.call(0x0044_fb00, &args![this]);
}

/// The base constructor `0044fae0` (`this` only, plain `RET`).
const BASE_CONSTRUCTOR_0044FAE0: u32 = 0x0044_fae0;

// Translated from 008caab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `0044fae0` on `this`, then clears the words at `+0x44`
/// and `+0x40`. Returns `this`.
pub fn fn_008caab0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR_0044FAE0, &args![this]);
    e.mem.set_u32(this.addr() + 0x44, 0);
    e.mem.set_u32(this.addr() + 0x40, 0);
    this
}

// Translated from 008caae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the word at `+0xc`. Returns `this`.
pub fn fn_008caae0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr() + 0xc, 0);
    this
}

// Translated from 008cc6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores `value` in the word at `+0x8C`;
/// `RET 4`.
pub fn fn_008cc6d0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x8c, value);
}

// Translated from 008cc730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: does nothing with `this` and its four stack
/// words (`RET 0x10`); returns no value.
pub fn fn_008cc730(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) {
}

// Translated from 008cdd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the word at `+0xA8`.
pub fn fn_008cdd90(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xa8)
}

// ---- n028: `ArrowProjectile` (vtables `01085954`/`01085948`, RTTI
// `.?AVArrowProjectile@@` read at the locators) ----
//
// Field offsets of `ArrowProjectile` (Xbox PDB names; the PC build's
// `TESForm` is 0x10 bytes smaller, so every field sits 0x10 lower than the
// PDB says, which the code below confirms: `DeltaS`, an `NiPoint3`, is
// constructed at +0xB4 and `psCollisionData` is allocated at +0x88).

/// `MobileObject::pCurrentProcess` (Xbox PDB +0x78): the process.
const PROCESS: u32 = 0x68;
/// `ArrowProjectile::psCollisionData` (Xbox PDB +0x98): the collision record
/// (`ArrowProjectile::CollisionInfo`, 0x54 bytes on PC).
const COLLISION: u32 = 0x88;
/// `eState` (Xbox PDB +0x9C; `STATE_FLYING` 0, `STATE_COLLISION` 1,
/// `STATE_POST_COLLISION` 2, `STATE_DELETING` 3 in the PDB's enum).
const STATE: u32 = 0x8c;
/// `fFadeAlpha` (Xbox PDB +0xA0).
const FADE_ALPHA: u32 = 0x90;
/// `fAge` (Xbox PDB +0xA4).
const AGE: u32 = 0x94;
/// `fSpeed` (Xbox PDB +0xA8).
const SPEED: u32 = 0x98;
/// `fDamage` (Xbox PDB +0xAC).
const DAMAGE: u32 = 0x9c;
/// `fWaterSpeed` (Xbox PDB +0xB0); the update counts it down while the arrow
/// is under water.
const WATER_SPEED: u32 = 0xa0;
/// `pShooter` (Xbox PDB +0xB4).
const SHOOTER: u32 = 0xa4;
/// `DeltaS` (Xbox PDB +0xC4, an `NiPoint3`).
const DELTA: u32 = 0xb4;
/// `bNeedsFinishInitLoadGame` (Xbox PDB +0xD0).
const NEEDS_FINISH_INIT_LOAD_GAME: u32 = 0xc0;
/// `bAddedToInventory` (Xbox PDB +0xD1).
const ADDED_TO_INVENTORY: u32 = 0xc1;
/// `bPostForce` (Xbox PDB +0xD3).
const POST_FORCE: u32 = 0xc3;
/// `iLoadGameFlags` (Xbox PDB +0xD4).
const LOAD_GAME_FLAGS: u32 = 0xc4;

/// `ArrowProjectile::CollisionInfo` (Xbox PDB, 0x60 bytes there; the PC
/// allocation is 0x54): `eCollisionType`, `Point`, `Normal`, `Velocity`,
/// `pCollidee`, `pClosestObject`, `Rotation` (an `NiMatrix3`).
const COLLISION_TYPE: u32 = 0x00;
const COLLISION_POINT: u32 = 0x04;
const COLLISION_NORMAL: u32 = 0x10;
const COLLISION_VELOCITY: u32 = 0x1c;
const COLLISION_COLLIDEE: u32 = 0x28;
const COLLISION_CLOSEST_OBJECT: u32 = 0x2c;
const COLLISION_ROTATION: u32 = 0x30;
/// Size of the PC collision record (`operator new(0x54)`).
const COLLISION_SIZE: u32 = 0x54;

/// `ArrowProjectile`'s two vtables (primary at +0, the second at +0x18).
const VTABLE: u32 = 0x0108_5954;
const VTABLE_2: u32 = 0x0108_5948;
/// `ArrowProjectile::iArrowCount` (Xbox PDB static): the constructor adds
/// one, the destructor takes one.
const ARROW_COUNT: u32 = 0x011d_fc98;
/// The word `fn_008cddb0` caches the result of `005dbe10` in.
const CACHED_LAYER_WORD: u32 = 0x011d_fc9c;

/// The three-float vector at `011f426c` (zero in the file's data) the
/// collision record's points start from.
const ZERO_POINT: u32 = 0x011f_426c;
/// The nine floats at `011a9448` (an identity matrix in the file's data) the
/// record's rotation starts from.
const IDENTITY_ROTATION: u32 = 0x011a_9448;
/// The vector at `011a9478` (0, 1, 0 in the file's data).
const FORWARD_AXIS: u32 = 0x011a_9478;
/// `float` `FLT_MAX` (`01085910`): the velocity code compares the length
/// with its negation.
const FLOAT_MAX: u32 = 0x0108_5910;
/// The player (`PlayerCharacter*`, read from this word).
const PLAYER: u32 = 0x011d_ea3c;
/// `ProcessLists` (`011e0e80`).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The words holding the save/load game object, the data handler, `TES`
/// and the model loader.
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const DATA_HANDLER: u32 = 0x011c_3f2c;
const TES: u32 = 0x011d_ea10;
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The frame timer object `0084d030` reads (`011f6394`).
const FRAME_TIMER: u32 = 0x011f_6394;

/// Constants the code reads: `double` 0.0, 0.5, 2.0, 3.0, 8.0, 18.0 and
/// `float` 50.0, 0.1, -FLT_MAX, 24.0, -1.0, 2.0, 512.0, -1024.0, 200.0, 18.0
/// (their values in the file's data).
const DOUBLE_ZERO: u32 = 0x0101_2060;
const DOUBLE_HALF: u32 = 0x0101_1588;
const DOUBLE_TWO: u32 = 0x0101_1590;
const DOUBLE_THREE: u32 = 0x0102_1928;
const DOUBLE_EIGHT: u32 = 0x0102_40b0;
const DOUBLE_EIGHTEEN: u32 = 0x0103_56f0;
const FLOAT_FIFTY: u32 = 0x0101_b268;
const FLOAT_TENTH: u32 = 0x0101_e2bc;
const FLOAT_LOWEST: u32 = 0x0101_5f5c;
const FLOAT_TWENTY_FOUR: u32 = 0x0104_e0e8;
const FLOAT_MINUS_ONE: u32 = 0x0101_2054;
const FLOAT_TWO: u32 = 0x0101_62c0;
const FLOAT_512: u32 = 0x0102_31a0;
const FLOAT_MINUS_1024: u32 = 0x0108_5c58;
const FLOAT_200: u32 = 0x0102_226c;
const FLOAT_EIGHTEEN: u32 = 0x0104_7224;

/// The `Setting` objects whose value `00403e20` (`thiscall`, the address of
/// the value) gives: the arrow's age limit (`011ce684`), the impulse setting
/// (`011cf624`), the chance of keeping the arrow (`011cf510`, read as an
/// integer through `0043d4d0`), the bounce speed (`011ce1dc`) and the bounce
/// spin (`011cedac`). Their meanings are read from how this code uses them.
const SETTING_VALUE: u32 = 0x0040_3e20;
const SETTING_INT_VALUE: u32 = 0x0043_d4d0;
const SETTING_AGE_LIMIT: u32 = 0x011c_e684;
const SETTING_IMPULSE: u32 = 0x011c_f624;
const SETTING_KEEP_CHANCE: u32 = 0x011c_f510;
const SETTING_BOUNCE_SPEED: u32 = 0x011c_e1dc;
const SETTING_BOUNCE_SPIN: u32 = 0x011c_edac;

/// `operator new` / `operator delete` (cdecl, one word).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The no-op constructor of an `NiPoint3`/`NiMatrix3` (`006815c0`,
/// `thiscall`, returns `this`).
const CONSTRUCT_IN_PLACE: u32 = 0x0068_15c0;
/// `MobileObject::GetCharController` (Xbox PDB name of `009306d0`).
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `bhkCharacterProxy::GethkLinearVelocity` (engine map name of `00807080`).
const LINEAR_VELOCITY: u32 = 0x0080_7080;
/// `004a3c90` (`thiscall(source)`): fills the 16-byte vector at `this` from
/// the velocity; `00458700` (`thiscall(out, word)`) and `004586d0`
/// (`thiscall`, a float in ST0) give the length the code compares;
/// `00458620` (cdecl `(destination, vector)`) stores the vector as three
/// floats.
const VECTOR_LOAD: u32 = 0x004a_3c90;
const VECTOR_PREPARE: u32 = 0x0045_8700;
const VECTOR_LENGTH_OF: u32 = 0x0045_86d0;
const VECTOR_STORE: u32 = 0x0045_8620;
/// `0045bb20` (`thiscall(out, float)`): `this` scaled by the float, written to
/// `out`, returns `out`.
const POINT_SCALE: u32 = 0x0045_bb20;
/// `00461130` (`thiscall`): the node's world rotation.
const NODE_ROTATION: u32 = 0x0046_1130;
/// `004b4500` (`thiscall(out, vector)`): the matrix at `this` times the
/// vector, written to `out`, returns `out`.
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
/// `00439e90` / `00439ef0` (`thiscall(out, other)`): `this + other` and
/// `this - other` written to `out` (the engine's other units name them
/// so); return `out`.
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `FindFirstCollisionObject` (Xbox PDB name of `004b5260`, cdecl `(node)`).
const FIND_FIRST_COLLISION_OBJECT: u32 = 0x004b_5260;
/// `006fa820` (`thiscall`): what the code uses as the collision object's
/// rigid body.
const COLLISION_OBJECT_BODY: u32 = 0x006f_a820;
/// `0043fcd0` (`thiscall` on a reference): its loaded 3D.
const REFERENCE_3D: u32 = 0x0043_fcd0;
/// `MiddleHighProcess::GetSavedAcquireObject` (engine map name of the
/// folded `008d8520`): the reference's process.
const REFERENCE_PROCESS: u32 = 0x008d_8520;
/// `0045cd60` (`thiscall` on a process): its level (word at +0x28).
const PROCESS_LEVEL: u32 = 0x0045_cd60;
/// `008d6f30` (`thiscall` on a reference): its parent cell (word at +0x40).
const PARENT_CELL: u32 = 0x008d_6f30;
/// `bhkWorld::SetMotion` (Xbox PDB name of `00c6a350`, cdecl, five words).
const SET_MOTION: u32 = 0x00c6_a350;
/// `bhkWorld::RemoveObjects` (`00c69ee0`) and `bhkWorld::UpdatePosition`
/// (`00c69f50`), cdecl `(node, word, word)`.
const REMOVE_OBJECTS: u32 = 0x00c6_9ee0;
const UPDATE_POSITION: u32 = 0x00c6_9f50;
/// `00440460` / `0043fa80` (`thiscall(source)` on a node): set its
/// translation / rotation; `00440490` (`thiscall(float)`): its scale.
const NODE_SET_TRANSLATE: u32 = 0x0044_0460;
const NODE_SET_ROTATE: u32 = 0x0043_fa80;
const NODE_SET_SCALE: u32 = 0x0044_0490;
/// `0043d410` (`thiscall(float, word, word)`): fills an update record;
/// `00a59c60` (`thiscall(record)` on a node): updates the node with it.
const UPDATE_DATA: u32 = 0x0043_d410;
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// `TESObjectREFR::SetLocationOnReference` (Xbox PDB name of `00575830`).
const SET_LOCATION_ON_REFERENCE: u32 = 0x0057_5830;
/// `005b5e40` (cdecl, one word): prints the message whose address it gets.
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// The messages at `01085c5c`, `01085c90` and `01085cf0` (`"AI: No collision
/// data was found on this arrow"`, `"AI: An arrow thinks it is colliding with
/// an Actor, ..."`, `"AI: An arrow thinks it is colliding with an Reference,
/// ..."`).
const MESSAGE_NO_COLLISION_DATA: u32 = 0x0108_5c5c;
const MESSAGE_NO_ACTOR: u32 = 0x0108_5c90;
const MESSAGE_NO_REFERENCE: u32 = 0x0108_5cf0;
/// `00622290` (`thiscall`, three words): builds a collision filter word at
/// `this` from them (`006222c0`), returns `this`; `00626cb0` (`thiscall(word)` on a
/// rigid body): sets its filter word.
const MAKE_FILTER: u32 = 0x0062_2290;
const BODY_SET_FILTER: u32 = 0x0062_6cb0;
/// `0043b4f0` (`thiscall(out)` on a rigid body): its filter word into `out`,
/// returns `out`.
const BODY_GET_FILTER: u32 = 0x0043_b4f0;
/// `__RTDynamicCast` (cdecl, five words) and the type descriptors of
/// `TESObjectREFR` (`011841cc`) and `Actor` (`011846d4`; names read at the
/// descriptors).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const REFERENCE_TYPE: u32 = 0x0118_41cc;
const ACTOR_TYPE: u32 = 0x0118_46d4;
/// `00525340` (cdecl, one word): normalizes the vector at that address.
const NORMALIZE: u32 = 0x0052_5340;
/// `00457990` (`thiscall`): the length of the vector at `this` (ST0).
const POINT_LENGTH: u32 = 0x0045_7990;
/// `004a3760` (cdecl `(out, float, vector)`): the vector scaled, written to
/// `out`, returns `out`.
const SCALE_INTO: u32 = 0x004a_3760;
/// `004b46a0` (`thiscall(out)`): the inverse of the matrix at `this` into `out`.
const MATRIX_INVERSE: u32 = 0x004b_46a0;
/// `004a3a20` (`thiscall`): the upper 16 bits of the filter word at `this`.
const FILTER_HIGH_HALF: u32 = 0x004a_3a20;
/// `004a39f0` (`thiscall(value)`): replaces the low 7 bits of the filter word at
/// `this`; `0043b4d0` (`thiscall`): those low 7 bits.
const FILTER_SET_LOW_BITS: u32 = 0x004a_39f0;
const FILTER_LOW_BITS: u32 = 0x0043_b4d0;

/// The kinds (`006205a0`, `00457880`) for which the arrow takes the
/// `fn_008ccce0` path: the exe's byte map at `008cc6b8` / `008cccc4` sends
/// 0, 3, 5, 10, 11, 13 and 15 there and every other kind up to 15 past it.
fn takes_bounce_path(kind: u32) -> bool {
    matches!(kind, 0 | 3 | 5 | 10 | 11 | 13 | 15)
}

/// `count` words copied from `source` to `destination`, first to last.
fn copy_words(e: &mut Engine, destination: u32, source: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(destination + 4 * i, word);
    }
}

/// The collision record of the arrow (`psCollisionData`).
fn collision(e: &Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + COLLISION)
}

/// A new collision record: `operator new(0x54)`, constructed by
/// `fn_008cc6f0` when the allocation succeeded (null otherwise), stored in
/// `psCollisionData`.
fn new_collision(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![COLLISION_SIZE]).u32();
    let record = if block == 0 {
        0
    } else {
        fn_008cc6f0(e, Ptr::new(block)).addr()
    };
    e.mem.set_u32(this.addr() + COLLISION, record);
    record
}

/// The start shared by the record builders (`008cba30`, `008cc740`,
/// `008ccce0`, `008ccfb0`): with a character controller, the record's
/// velocity becomes the controller's linear velocity, or, when that
/// vector's length equals `-FLT_MAX`, the forward axis scaled by the speed
/// and turned by the arrow's 3D (when it has one).
fn take_controller_velocity(e: &mut Engine, this: Ptr) {
    if e.call(GET_CHAR_CONTROLLER, &args![this]).u32() == 0 {
        return;
    }
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    let velocity = e.call(LINEAR_VELOCITY, &args![controller]).u32();
    // Locals: the 16-byte vector (EBP-0x40), the prepared copy (-0x50) and
    // two points (-0x5c, -0x68).
    e.with_stack(0x38, |e, locals| {
        let vector = locals.addr();
        let prepared = vector + 0x10;
        let scaled = vector + 0x20;
        let turned = vector + 0x2c;
        e.call(VECTOR_LOAD, &args![vector, velocity]);
        let prepared = e.call(VECTOR_PREPARE, &args![vector, prepared, 0u32]).u32();
        let length = e.call(VECTOR_LENGTH_OF, &args![prepared]).f64();
        let lowest = -x87_load32(e.mem.u32(FLOAT_MAX));
        if lowest == length {
            let node = e.vcall(this.addr(), 0x1d0, &[]).u32();
            if node != 0 {
                let speed = x87_quiet_bits(e.mem.u32(this.addr() + SPEED));
                let along = e
                    .call(POINT_SCALE, &args![FORWARD_AXIS, scaled, speed])
                    .u32();
                let node = e.vcall(this.addr(), 0x1d0, &[]).u32();
                let rotation = e.call(NODE_ROTATION, &args![node]).u32();
                let result = e
                    .call(MATRIX_TIMES_POINT, &args![rotation, turned, along])
                    .u32();
                let record = collision(e, this);
                copy_words(e, record + COLLISION_VELOCITY, result, 3);
            }
        } else {
            let record = collision(e, this);
            e.call(VECTOR_STORE, &args![record + COLLISION_VELOCITY, vector]);
        }
    });
}

// Translated from 008caa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three words at `source` to `this` (a 12-byte copy
/// constructor; `aitaskmanager.rs` uses it for `MobileObjectMessage`).
/// Returns `this`.
pub fn fn_008caa70(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    copy_words(e, this.addr(), source.addr(), 3);
    this
}

// Translated from 008cab20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008cab20 - the compiler's exception frame (FS:[0]) that destroys the half-built object when a step throws is not translated
/// The `ArrowProjectile` constructor: the `MobileObject` base
/// (`0092eb50`), the two vtables, the `DeltaS` point (`006815c0` at +0xB4),
/// `0046a010(1)`, a new `HighProcess` (`operator new(0x46c)`, `008d7510`)
/// as its process, cleared collision record and flags, and one more in
/// `iArrowCount`. Returns `this`.
pub fn fn_008cab20(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    e.call(0x0092_eb50, &args![this]);
    e.mem.set_u32(at, VTABLE);
    e.mem.set_u32(at + 0x18, VTABLE_2);
    e.call(CONSTRUCT_IN_PLACE, &args![at + DELTA]);
    e.call(0x0046_a010, &args![this, 1u32]);
    let block = e.call(OPERATOR_NEW, &args![0x46cu32]).u32();
    let process = if block == 0 {
        0
    } else {
        e.call(0x008d_7510, &args![block]).u32()
    };
    e.mem.set_u32(at + PROCESS, process);
    e.mem.set_u32(at + COLLISION, 0);
    e.mem.set_u8(at + NEEDS_FINISH_INIT_LOAD_GAME, 0);
    e.mem.set_u32(at + LOAD_GAME_FLAGS, 0);
    e.mem.set_u8(at + ADDED_TO_INVENTORY, 0);
    e.mem.set_u8(at + POST_FORCE, 0);
    let count = e.mem.u32(ARROW_COUNT);
    e.mem.set_u32(ARROW_COUNT, count.wrapping_add(1));
    this
}

// Translated from 008cac30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor: the destructor body (`fn_008cac60`), then
/// `operator delete(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_008cac30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_008cac60(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008cac60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008cac60 - the compiler's exception frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated
/// The `ArrowProjectile` destructor body: the vtables, one less in
/// `iArrowCount`, removal from the list `004610f0` gives on
/// `ProcessLists` (`00905330`), the force on the collidee's body
/// (`0062ab60`) when the record says it hit a reference
/// (`eCollisionType` 1) and the arrow has 3D, the collidee's process told
/// (slot 0x6A8) when it hit an actor (type 0) that slot 0x100 accepts and
/// the arrow is not waiting for its load, the record freed, no 3D
/// (`TESObjectREFR::Set3D(0, 1)`), the other mobile objects cleaned up
/// (`MobileObject::CleanUpOtherMobileObjects(0)`) unless the data handler's
/// byte (`004226e0`) is set, and `MobileObject::~MobileObject`.
pub fn fn_008cac60(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u32(at, VTABLE);
    e.mem.set_u32(at + 0x18, VTABLE_2);
    let count = e.mem.u32(ARROW_COUNT);
    e.mem.set_u32(ARROW_COUNT, count.wrapping_sub(1));

    let list = e.call(0x0046_10f0, &args![PROCESS_LISTS]).u32();
    if list != 0 {
        e.with_stack(4, |e, item| {
            e.mem.set_u32(item.addr(), at);
            e.call(0x0090_5330, &args![list, item]);
        });
    }

    let node = e.call(REFERENCE_3D, &args![this]).u32();
    if node != 0 {
        let record = collision(e, this);
        if record != 0 && e.mem.u32(record + COLLISION_TYPE) == 1 {
            let object = e.call(FIND_FIRST_COLLISION_OBJECT, &args![node]).u32();
            let body = if object == 0 {
                0
            } else {
                e.call(COLLISION_OBJECT_BODY, &args![object]).u32()
            };
            if body != 0 {
                e.with_stack(0x10, |e, point| {
                    e.call(CONSTRUCT_IN_PLACE, &args![point]);
                    e.vcall(body, 0xf0, &args![point]);
                    let record = collision(e, this);
                    let closest = e.mem.u32(record + COLLISION_CLOSEST_OBJECT);
                    e.call(0x0062_ab60, &args![point, closest]);
                });
            }
        }
    }

    if e.mem.u8(at + NEEDS_FINISH_INIT_LOAD_GAME) == 0 {
        let record = collision(e, this);
        if record != 0
            && e.mem.u32(record + COLLISION_TYPE) == 0
            && e.mem.u32(record + COLLISION_COLLIDEE) != 0
        {
            let collidee = e.mem.u32(record + COLLISION_COLLIDEE);
            if e.vcall(collidee, 0x100, &[]).bool() {
                let collidee = e.mem.u32(collision(e, this) + COLLISION_COLLIDEE);
                if collidee != 0 && e.call(REFERENCE_PROCESS, &args![collidee]).u32() != 0 {
                    let process = e.call(REFERENCE_PROCESS, &args![collidee]).u32();
                    e.vcall(process, 0x6a8, &args![this]);
                }
            }
        }
    }

    let record = collision(e, this);
    e.call(OPERATOR_DELETE, &args![record]);
    e.call(0x0057_02e0, &args![this, 0u32, 1u32]);
    let data_handler = e.mem.u32(DATA_HANDLER);
    if !e.call(0x0042_26e0, &args![data_handler]).bool() {
        e.call(0x0092_ed60, &args![this, 0u32]);
    }
    e.call(0x0092_ec40, &args![this]);
}

// Translated from 008caea0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008caea0 - the compiler's exception frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated
/// Gives the arrow its Havok character controller and puts it in its
/// cell's world. Nothing happens without a process or when the process
/// level (`0045cd60`) is not 0. Without a controller one is built
/// (`008d10e0`, 0x660 bytes from `00553300`) from a construction record
/// (`00c6da50` ... `00c6de10` on the 0xD8-byte local) holding the
/// arrow's 3D and position, the cell's world, a sphere shape of radius 0.1
/// (`0056e950`), the collision layer of `shooter`'s shape (`00931ed0`)
/// or of `fn_008cddb0`, and with a `shooter` the value `00646660(value,
/// 50.0)`; the process is given it (slot 0x290, as an `NiPointer` built by
/// `00633c90`). Then the controller is placed: the cell's water height
/// (`TESObjectCELL::GetWaterHeight`) or `-FLT_MAX`, the world
/// (`bhkCharacterController::SetWorld`), the 3D (`SetNiObject`, wireframe
/// and motion 4 when it changes), its position (8 higher when the world
/// changed) and the world's add (slot 0xD0) with the controller's layer.
pub fn fn_008caea0(e: &mut Engine, this: Ptr, shooter: Ptr, value: f32) {
    let at = this.addr();
    let process = e.mem.u32(at + PROCESS);
    if process == 0 || e.call(PROCESS_LEVEL, &args![process]).u32() != 0 {
        return;
    }
    let mut controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    let cell = e.call(PARENT_CELL, &args![this]).u32();
    let world = if cell == 0 {
        0
    } else {
        e.call(0x0045_43c0, &args![cell]).u32()
    };

    if controller == 0 {
        // The construction record (EBP-0xF0, 0xD8 bytes) and the word the
        // shape's filter is read into (EBP-0x100).
        controller = e.with_stack(0xe0, |e, locals| {
            let info = locals.addr();
            let filter = info + 0xd8;
            e.call(0x00c6_da50, &args![info, 1u32]);
            e.call(0x0081_0550, &args![info, 0.0f32]);
            if !shooter.is_null() {
                // The integer value is pushed only to reserve the stack slot
                // that the blend below overwrites.
                e.call(0x0066_ef20, &args![shooter.addr() + 0xa4, 0xbu32]);
                let fifty = x87_quiet_bits(e.mem.u32(FLOAT_FIFTY));
                let blended = e
                    .call(0x0064_6660, &args![x87_quiet(value), f32::from_bits(fifty)])
                    .f64();
                e.call(0x0040_7860, &args![info, x87_store32(blended)]);
            }
            let node = e.vcall(at, 0x1d0, &[]).u32();
            e.call(0x008a_c890, &args![info, node]);
            let position = e.vcall(at, 0x1f4, &[]).u32();
            e.call(0x0081_0510, &args![info, position]);
            e.call(0x0085_7230, &args![info, world]);
            let block = e.call(0x00aa_13e0, &args![0x14u32]).u32();
            let shape = if block == 0 {
                0
            } else {
                let radius = x87_quiet_bits(e.mem.u32(FLOAT_TENTH));
                e.call(0x0056_e950, &args![block, radius, 1u32]).u32()
            };
            e.call(0x0081_05c0, &args![info, shape]);
            e.call(0x0081_0530, &args![info, 0u32]);
            let layer = if !shooter.is_null() {
                let shape = e.call(0x0093_1ed0, &args![shooter, filter]).u32();
                e.call(FILTER_HIGH_HALF, &args![shape]).u32()
            } else {
                fn_008cddb0(e)
            };
            e.call(0x0081_0590, &args![info, 6u32, layer]);
            e.call(0x0081_0570, &args![info, 6u32]);
            let block = e.call(0x0055_3300, &args![0x660u32]).u32();
            let controller = if block == 0 {
                0
            } else {
                e.call(0x008d_10e0, &args![block, info, this]).u32()
            };
            e.call(0x0081_06b0, &args![controller, 1.0f32]);
            let flag = e.call(0x0052_5420, &[]).u8();
            e.call(0x0081_0610, &args![controller, u32::from(flag)]);
            e.call(0x0081_05e0, &args![controller, 1u32]);
            // The process takes the controller as an `NiPointer` by value,
            // built in its argument slot.
            let pointer = e.with_stack(4, |e, slot| {
                e.call(0x0063_3c90, &args![slot, controller]);
                e.mem.u32(slot.addr())
            });
            let process = e.mem.u32(at + PROCESS);
            e.vcall(process, 0x290, &args![pointer]);
            e.call(0x0081_0640, &args![controller, this]);
            e.call(0x00c6_de10, &args![info]);
            controller
        });
    }

    if controller == 0 {
        return;
    }
    if cell != 0 && world != 0 {
        let height = e.call(0x0054_71e0, &args![cell]).f64();
        e.call(0x0057_b1d0, &args![controller, x87_store32(height)]);
    } else {
        let lowest = x87_quiet_bits(e.mem.u32(FLOAT_LOWEST));
        e.call(0x0057_b1d0, &args![controller, lowest]);
    }
    let old_world = e.call(0x0062_1ad0, &args![controller]).u32();
    e.call(0x00c7_0c90, &args![controller, world]);
    let node = e.vcall(at, 0x1d0, &[]).u32();
    let current = e.call(0x0081_0660, &args![controller]).u32();
    let mut moved = true;
    if current != node {
        e.call(0x00c6_d7a0, &args![controller, node]);
        if world != 0 && e.call(0x004b_ae00, &args![world]).bool() {
            e.vcall(controller, 0xd0, &args![0u32]);
            let object = e.call(0x0081_0660, &args![controller]).u32();
            e.call(0x00b6_b910, &args![object, 1u32]);
        }
        e.call(SET_MOTION, &args![node, 4u32, 1u32, 0u32, 1u32]);
        moved = true;
    }
    // The position (EBP-0x130) and the filter word (EBP-0x134).
    e.with_stack(0x10, |e, locals| {
        let position = locals.addr();
        let filter = position + 0xc;
        let source = e.vcall(at, 0x1f4, &[]).u32();
        copy_words(e, position, source, 3);
        if old_world != world {
            if old_world != 0 {
                e.call(REMOVE_OBJECTS, &args![node, 1u32, 0u32]);
            }
            let z = x87_load32(e.mem.u32(position + 8));
            let eight = e.mem.f64(DOUBLE_EIGHT);
            e.mem.set_u32(position + 8, x87_store32(x87_add(z, eight)));
            moved = true;
        }
        if world == 0 {
            return;
        }
        e.call(0x0056_20e0, &args![controller, position]);
        // `moved` is set on every path above, so the exe's branch to
        // `bhkWorld::UpdatePosition` below never runs; it is kept as the
        // exe has it.
        if moved {
            e.call(0x0070_c440, &args![controller, filter]);
            let layer = e.call(FILTER_HIGH_HALF, &args![filter]).u32();
            e.vcall(world, 0xd0, &args![node, 1u32, 0u32, layer, 0u32]);
        } else {
            e.call(UPDATE_POSITION, &args![node, 1u32, 0u32]);
        }
    });
}

// Translated from 008cb380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls slot 0x2F8 of `this` with the frame time `0084d030` reads from
/// `011f6394`. The stack word (`RET 4`) is not read.
pub fn fn_008cb380(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let seconds = e.call(0x0084_d030, &args![FRAME_TIMER]).f64();
    e.vcall(this.addr(), 0x2f8, &args![x87_store32(seconds)]);
}

// Translated from 008cb3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The arrow's update for `seconds`. A pending load-game finish runs
/// first (slot 0x6C with `iLoadGameFlags`, then `0085f750` on the
/// save/load game) unless the record says it hit something whose 3D is
/// not loaded. With `seconds >= 0` the age grows and passing the age
/// setting makes the state 3; a test on the arrow (`00440d80`) ends the
/// update there. Flying with a controller (state 0): the move (slot
/// 0x250 with the velocity `(0, speed * seconds, 0)` and 0xF), the slowdown
/// under water (speed, damage and the controller's scale by
/// `(water - seconds) / water`; below 0.5 the record becomes type 4 and
/// `fn_008cd210` runs), a new state from a collision (`fn_008cd210`;
/// 3 stays 3, the rest become 2, and the arrow moves to the 3D's
/// location), else the 3D follows the position and points along the
/// motion (`DeltaS`, pitch `-asin(dz / |DeltaS|)`). In state 2 a stuck arrow
/// follows the actor it hit (or is dropped, slot 0xC4, when that actor's
/// 3D or cell is gone); in state 3 it fades out over three seconds and is
/// dropped at zero. Last the 3D is updated (`0043d410`, `00a59c60`).
pub fn fn_008cb3c0(e: &mut Engine, this: Ptr, seconds: f32) {
    let at = this.addr();
    let seconds_bits = seconds.to_bits();
    if e.mem.u8(at + NEEDS_FINISH_INIT_LOAD_GAME) != 0 {
        let mut finish = true;
        let record = collision(e, this);
        if record != 0 {
            let kind = e.mem.u32(record + COLLISION_TYPE);
            if (kind == 1 || kind == 0) && e.mem.u32(record + COLLISION_COLLIDEE) != 0 {
                let collidee = e.mem.u32(collision(e, this) + COLLISION_COLLIDEE);
                if e.call(REFERENCE_3D, &args![collidee]).u32() == 0 {
                    finish = false;
                }
            }
        }
        if finish {
            e.mem.set_u8(at + NEEDS_FINISH_INIT_LOAD_GAME, 0);
            let flags = e.mem.u32(at + LOAD_GAME_FLAGS);
            e.vcall(at, 0x6c, &args![flags, 0u32]);
            let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
            e.call(0x0085_f750, &args![save_load_game]);
        }
    }
    let node = e.vcall(at, 0x1d0, &[]).u32();
    // `FCOMP 0.0; TEST AH,1; JNZ`: below zero or unordered skips to the end.
    if x87_load32(seconds_bits) >= e.mem.f64(DOUBLE_ZERO) {
        let age = x87_add(x87_load32(e.mem.u32(at + AGE)), x87_m32(seconds_bits));
        e.mem.set_u32(at + AGE, x87_store32(age));
        let limit = e.call(SETTING_VALUE, &args![SETTING_AGE_LIMIT]).u32();
        if x87_load32(e.mem.u32(limit)) < x87_load32(e.mem.u32(at + AGE)) {
            e.mem.set_u32(at + STATE, 3);
        }
        if e.call(0x0044_0d80, &args![this]).bool() {
            return;
        }
        if e.mem.u32(at + STATE) == 0 && e.call(GET_CHAR_CONTROLLER, &args![this]).u32() != 0 {
            if update_flight(e, this, node, seconds_bits) {
                return;
            }
        } else if e.mem.u32(at + STATE) == 2 && collision(e, this) != 0 {
            follow_collidee(e, this, node);
        } else if e.mem.u32(at + STATE) == 3 {
            let step = x87_div(x87_load32(seconds_bits), e.mem.f64(DOUBLE_THREE));
            let alpha = x87_sub(x87_m32(e.mem.u32(at + FADE_ALPHA)), step);
            e.mem.set_u32(at + FADE_ALPHA, x87_store32(alpha));
            // `TEST AH,0x41; JP`: zero or below (not unordered) clears it.
            if x87_load32(e.mem.u32(at + FADE_ALPHA)) <= e.mem.f64(DOUBLE_ZERO) {
                e.mem.set_u32(at + FADE_ALPHA, 0.0f32.to_bits());
                e.vcall(at, 0xc4, &args![1u32]);
            }
            if node != 0 {
                let alpha = x87_quiet_bits(e.mem.u32(at + FADE_ALPHA));
                e.call(0x00b6_bb30, &args![node, alpha]);
            }
        }
    }
    if node != 0 {
        e.with_stack(0x10, |e, update| {
            let seconds = x87_quiet_bits(seconds_bits);
            e.call(UPDATE_DATA, &args![update, seconds, 1u32, 0u32]);
            e.call(NODE_UPDATE, &args![node, update]);
        });
    }
}

/// The flying part of `fn_008cb3c0` (state 0 with a controller). Returns
/// whether the update ends there (a collision changed the state).
fn update_flight(e: &mut Engine, this: Ptr, node: u32, seconds_bits: u32) -> bool {
    let at = this.addr();
    // Locals at their EBP offsets: velocity -0x20, start -0x14, position
    // -0x2c, rotation -0x5c (9 floats), angles -0x64 / -0x38 / -0x6c,
    // the difference -0x8c.
    e.with_stack(0x90, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x90 - offset;
        let velocity = local(0x20);
        let start = local(0x14);
        let position = local(0x2c);
        let rotation = local(0x5c);
        let first_angle = local(0x64);
        let pitch = local(0x38);
        let last_angle = local(0x6c);
        let difference = local(0x8c);

        let step = x87_mul(x87_load32(e.mem.u32(at + SPEED)), x87_m32(seconds_bits));
        let step = x87_store32(step);
        e.call(0x0041_6870, &args![velocity, 0.0f32, step, 0.0f32]);
        let source = e.vcall(at, 0x1f4, &[]).u32();
        copy_words(e, start, source, 3);
        let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
        let level = e.call(0x0081_0a80, &args![controller]).f64();
        let height = e.call(0x0045_87d0, &args![x87_store32(level)]).f64();
        let source = e.vcall(at, 0x1f4, &[]).u32();
        // `TEST AH,0x41; JP`: at or below the water height.
        if x87_load32(e.mem.u32(source + 8)) <= height {
            let spent = x87_mul(x87_load32(seconds_bits), 1.0);
            let water = e.mem.u32(at + WATER_SPEED);
            let left = x87_sub(x87_m32(water), spent);
            let factor = x87_store32(x87_div(left, x87_m32(water)));
            let speed = x87_mul(x87_load32(e.mem.u32(at + SPEED)), x87_m32(factor));
            e.mem.set_u32(at + SPEED, x87_store32(speed));
            let damage = x87_mul(x87_load32(e.mem.u32(at + DAMAGE)), x87_m32(factor));
            e.mem.set_u32(at + DAMAGE, x87_store32(damage));
            let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
            e.call(0x0081_06b0, &args![controller, x87_quiet_bits(factor)]);
            let spent = x87_mul(x87_load32(seconds_bits), 1.0);
            let water = x87_sub(x87_m32(e.mem.u32(at + WATER_SPEED)), spent);
            e.mem.set_u32(at + WATER_SPEED, x87_store32(water));
            if x87_load32(e.mem.u32(at + WATER_SPEED)) < e.mem.f64(DOUBLE_HALF) {
                fn_008ccfb0(e, this, Ptr::NULL);
                fn_008cd210(e, this);
            }
        }
        let seconds = x87_quiet_bits(seconds_bits);
        e.vcall(at, 0x250, &args![seconds, velocity, 0xfu32]);
        if e.mem.u32(at + STATE) != 0 {
            let next = u32::from(e.mem.u32(at + STATE) == 3) + 2;
            if fn_008cd210(e, this) {
                if node != 0 {
                    let location = e.call(0x0043_c490, &args![node]).u32();
                    e.call(SET_LOCATION_ON_REFERENCE, &args![this, location]);
                }
                e.mem.set_u32(at + STATE, next);
                return true;
            }
        }
        let source = e.vcall(at, 0x1f4, &[]).u32();
        copy_words(e, position, source, 3);
        if node != 0 {
            e.call(NODE_SET_TRANSLATE, &args![node, position]);
            let current = e.call(0x006a_9540, &args![node]).u32();
            copy_words(e, rotation, current, 9);
            e.call(
                0x00a5_9400,
                &args![rotation, first_angle, pitch, last_angle],
            );
            let moved = e
                .call(POINT_SUBTRACT, &args![position, difference, start])
                .u32();
            copy_words(e, at + DELTA, moved, 3);
            let rise = x87_quiet_bits(e.mem.u32(at + DELTA + 8));
            let length = e.call(POINT_LENGTH, &args![at + DELTA]).f64();
            let length = x87_store32(length);
            let sine = x87_store32(x87_div(x87_load32(rise), x87_m32(length)));
            let angle = e.call(0x004b_54c0, &args![x87_quiet_bits(sine)]).f64();
            e.mem.set_u32(pitch, x87_store32(-angle));
            let first = x87_quiet_bits(e.mem.u32(first_angle));
            let second = x87_quiet_bits(e.mem.u32(pitch));
            let last = x87_quiet_bits(e.mem.u32(last_angle));
            e.call(0x00a5_9660, &args![rotation, first, second, last]);
            e.call(NODE_SET_ROTATE, &args![node, rotation]);
        }
        false
    })
}

/// State 2 of `fn_008cb3c0`: the arrow stuck in what it hit.
fn follow_collidee(e: &mut Engine, this: Ptr, node: u32) {
    let at = this.addr();
    let kind = e.mem.u32(collision(e, this) + COLLISION_TYPE);
    if node != 0 && kind == 0 {
        let location = e.call(0x0045_bb80, &args![node]).u32();
        e.call(SET_LOCATION_ON_REFERENCE, &args![this, location]);
        let mut cell = 0;
        let collidee = e.mem.u32(collision(e, this) + COLLISION_COLLIDEE);
        if collidee != 0 {
            let collidee = e.mem.u32(collision(e, this) + COLLISION_COLLIDEE);
            cell = e.call(PARENT_CELL, &args![collidee]).u32();
        }
        if cell != e.call(PARENT_CELL, &args![this]).u32() {
            if cell == 0 {
                e.vcall(at, 0xc4, &args![1u32]);
            } else {
                e.call(0x0096_e870, &args![PROCESS_LISTS, this]);
            }
        }
    }
    if node != 0 && kind == 1 {
        let record = collision(e, this);
        let gone = record == 0 || {
            let collidee = e.mem.u32(record + COLLISION_COLLIDEE);
            collidee == 0 || {
                let collidee = e.mem.u32(collision(e, this) + COLLISION_COLLIDEE);
                e.call(REFERENCE_3D, &args![collidee]).u32() == 0
            }
        };
        if gone {
            e.vcall(at, 0xc4, &args![1u32]);
        }
    }
}

// Translated from 008cba30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008cba30 - the compiler's exception frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated
/// The arrow hits `target` at `point` (surface `normal`): a new collision
/// record of type 0 (point, normal, identity rotation, the controller's
/// velocity) and state 1. With a `target`: the shooter's process hears of
/// it (`008d0600`), the target is recorded and told (`008a5300`,
/// `005ac750`); when the shooter is the player (or `00493bb0` holds for
/// it), the target's slot 0x22C (with 0) answers false and then true, and
/// slot 0x21C is false or slot 0x38C true, the target's 3D is pushed
/// (`0062b660`) with `006466e0(impulse setting, 1)` from 1024 units behind
/// the arrow along its forward axis. Then a ray (`008cfbc0`, 9 words) from
/// 512 units behind the arrow along its heading (slot 0x2BC) and pitch
/// (`00931d70`) finds a node of the target's 3D: the record gets the point
/// 18 units deeper and the rotation in that node's space, and the node's
/// collision body is kept. When nothing was found, or `00894d60`, the
/// target's process and `009a6ae0` hold and the point is at or above the
/// target's position plus half its height (`008853a0`), a second search
/// (`00c7ff40` with 7 in the latter case, else `00c806b0` with a 200-unit
/// pick at `point`) gives the node; without one the arrow falls
/// (`fn_008ccfb0`) and stops. A found body takes the hit (`008d0020`). A
/// kind (`006205a0` of the node's body's `004ae6a0`) that the exe's byte
/// map sends there makes the arrow bounce (`fn_008ccce0`); when slot 0x21C
/// holds and slot 0x38C does not, the arrow falls and drops
/// (`fn_008ccfb0`, `fn_008cc6d0(3)`); otherwise, without an enchantment,
/// `00487f50() % 100` below the keep-chance setting gives the target the
/// arrow's item (slot 0x190 with it, 0, 1) and sets `bAddedToInventory`.
pub fn fn_008cba30(e: &mut Engine, this: Ptr, point: Ptr, normal: Ptr, target: Ptr) {
    let at = this.addr();
    let record = new_collision(e, this);
    e.mem.set_u32(record + COLLISION_TYPE, 0);
    let record = collision(e, this);
    e.mem.set_u32(record + COLLISION_CLOSEST_OBJECT, 0);
    let record = collision(e, this);
    e.mem.set_u32(record + COLLISION_COLLIDEE, 0);
    let record = collision(e, this);
    copy_words(e, record + COLLISION_NORMAL, normal.addr(), 3);
    let record = collision(e, this);
    copy_words(e, record + COLLISION_POINT, point.addr(), 3);
    let record = collision(e, this);
    copy_words(e, record + COLLISION_ROTATION, IDENTITY_ROTATION, 9);
    let record = collision(e, this);
    copy_words(e, record + COLLISION_VELOCITY, ZERO_POINT, 3);
    take_controller_velocity(e, this);
    e.mem.set_u32(at + STATE, 1);
    if target.is_null() {
        return;
    }
    // The frame from EBP-0x31C up (offsets are the exe's `EBP - n`).
    e.with_stack(0x320, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x320 - offset;
        actor_hit(e, this, point, normal, target, &local);
    });
}

/// The part of `fn_008cba30` after the record is built (`target` not
/// null); `local(n)` is the address of the exe's `EBP - n`.
fn actor_hit(
    e: &mut Engine,
    this: Ptr,
    point: Ptr,
    normal: Ptr,
    target: Ptr,
    local: &dyn Fn(u32) -> u32,
) {
    let at = this.addr();
    let target_at = target.addr();
    let node = e.vcall(at, 0x1d0, &[]).u32();
    let shooter = e.mem.u32(at + SHOOTER);
    if shooter != 0 && e.vcall(shooter, 0x428, &[]).u32() != 0 {
        let position = e.vcall(target_at, 0x1f4, &[]).u32();
        let shooter = e.mem.u32(at + SHOOTER);
        let process = e.vcall(shooter, 0x428, &[]).u32();
        e.call(0x008d_0600, &args![process, position, 1u32]);
    }
    let record = collision(e, this);
    e.mem.set_u32(record + COLLISION_COLLIDEE, target_at);
    e.call(0x008a_5300, &args![target]);
    let collidee = e.mem.u32(collision(e, this) + COLLISION_COLLIDEE);
    let collidee_word = e.call(0x005d_43c0, &args![collidee]).u32();
    let item = e.call(0x007a_f430, &args![this]).u32();
    e.call(0x005a_c750, &args![item, collidee_word, 0x100u32]);

    let shooter = e.mem.u32(at + SHOOTER);
    if shooter != 0
        && (shooter == e.mem.u32(PLAYER) || e.call(0x0049_3bb0, &args![shooter]).bool())
        && !e.vcall(target_at, 0x22c, &args![0u32]).bool()
        && e.vcall(target_at, 0x22c, &args![0u32]).bool()
        && (!e.vcall(target_at, 0x21c, &[]).bool() || e.vcall(target_at, 0x38c, &[]).bool())
    {
        let target_node = e.call(REFERENCE_3D, &args![target]).u32();
        if target_node != 0 {
            push_target(e, this, node, target_node, local);
        }
    }

    let mut body = 0u32;
    let player = e.mem.u32(PLAYER);
    let ray_root = if target_at == player {
        e.call(0x0095_0bb0, &args![player, 0u32]).u32()
    } else {
        e.call(REFERENCE_3D, &args![target]).u32()
    };
    let distance = local(0xcc);
    e.mem
        .set_u32(distance, x87_quiet_bits(e.mem.u32(FLOAT_512)));
    let direction = local(0xd8);
    let heading = local(0xfc);
    let pitch = local(0x120);
    e.call(CONSTRUCT_IN_PLACE, &args![direction]);
    e.call(CONSTRUCT_IN_PLACE, &args![heading]);
    e.call(CONSTRUCT_IN_PLACE, &args![pitch]);
    let angle = e.vcall(at, 0x2bc, &args![0u32]).f64();
    e.call(0x004a_0c90, &args![heading, x87_store32(angle)]);
    let looking = e.call(0x0093_1d70, &args![this]).f64();
    e.call(0x0052_4ac0, &args![pitch, x87_store32(looking)]);
    let product = e
        .call(0x0043_f8d0, &args![heading, local(0x144), pitch])
        .u32();
    copy_words(e, heading, product, 9);
    e.call(0x0043_9f50, &args![heading, 1u32, direction]);
    e.call(NORMALIZE, &args![direction]);
    let two = x87_quiet_bits(e.mem.u32(FLOAT_TWO));
    let reach = x87_quiet_bits(e.mem.u32(distance));
    let ahead = e
        .call(POINT_SCALE, &args![direction, local(0x15c), reach])
        .u32();
    let back = e.call(0x0053_d280, &args![ahead, local(0x150), two]).u32();
    let position = e.vcall(at, 0x1f4, &[]).u32();
    let start = local(0x168);
    e.call(POINT_SUBTRACT, &args![position, start, back]);

    let mut found = false;
    let alive = !e.vcall(target_at, 0x22c, &args![1u32]).bool();
    let target_node = e.vcall(target_at, 0x1d0, &[]).u32();
    let mut ray = vec![target_node];
    for i in 0..3 {
        ray.push(e.mem.u32(start + 4 * i));
    }
    for i in 0..3 {
        ray.push(e.mem.u32(direction + 4 * i));
    }
    ray.push(distance);
    ray.push(u32::from(alive));
    let mut bone = e.call(0x008c_fbc0, &ray).u32();
    if bone != 0 {
        let depth = x87_add(x87_load32(e.mem.u32(distance)), e.mem.f64(DOUBLE_EIGHTEEN));
        let depth = x87_store32(depth);
        e.call(
            POINT_SCALE,
            &args![direction, local(0x180), x87_quiet_bits(depth)],
        );
        place_on_bone(
            e,
            this,
            node,
            bone,
            Some(start),
            local(0x180),
            local,
            BoneLocals::FIRST,
        );
        let object = e.call(FIND_FIRST_COLLISION_OBJECT, &args![bone]).u32();
        if object != 0 {
            body = e.call(COLLISION_OBJECT_BODY, &args![object]).u32();
        }
        found = true;
    }

    let mut blocked = false;
    if e.call(0x0089_4d60, &args![target]).bool()
        && e.call(REFERENCE_PROCESS, &args![target]).u32() != 0
        && e.call(0x009a_6ae0, &args![target, this, 0u32]).bool()
    {
        let source = e.call(0x0043_6aa0, &args![target]).u32();
        let feet = local(0x228);
        copy_words(e, feet, source, 3);
        let height = e.call(0x0088_53a0, &args![target]).f64();
        let half = x87_div(height, e.mem.f64(DOUBLE_TWO));
        let top = x87_store32(x87_add(half, x87_m32(e.mem.u32(feet + 8))));
        // `FCOMPP; TEST AH,1; JNZ`: the point's height at or above the
        // middle (not unordered) marks it blocked.
        if x87_load32(e.mem.u32(point.addr() + 8)) >= x87_load32(top) {
            blocked = true;
        }
    }

    if !found || blocked {
        let pick = local(0x240);
        let from = local(0x250);
        e.call(0x0062_40d0, &args![pick]);
        e.call(CONSTRUCT_IN_PLACE, &args![from]);
        e.call(0x004a_3e00, &args![from, point]);
        e.call(0x008a_50f0, &args![pick, from]);
        let radius = x87_quiet_bits(e.mem.u32(FLOAT_200));
        e.call(0x008a_50d0, &args![pick, radius]);
        let picked = if blocked {
            e.call(0x00c7_ff40, &args![ray_root, 7u32]).u32()
        } else {
            let alive = !e.vcall(target_at, 0x22c, &args![1u32]).bool();
            e.call(0x00c8_06b0, &args![ray_root, pick, u32::from(alive)])
                .u32()
        };
        if picked != 0 {
            body = picked;
            let object = e.call(0x004a_e750, &args![body]).u32();
            let owner = e.call(0x004b_5a20, &args![object]).u32();
            if owner != 0 {
                bone = e.call(0x0044_ddc0, &args![owner]).u32();
                let depth = x87_quiet_bits(e.mem.u32(FLOAT_EIGHTEEN));
                e.call(POINT_SCALE, &args![direction, local(0x264), depth]);
                place_on_bone(
                    e,
                    this,
                    node,
                    bone,
                    None,
                    local(0x264),
                    local,
                    BoneLocals::SECOND,
                );
            }
        }
        if e.mem.u32(collision(e, this) + COLLISION_CLOSEST_OBJECT) == 0 {
            fn_008ccfb0(e, this, target);
            return;
        }
    }

    if body != 0 {
        let rotation = e.call(NODE_ROTATION, &args![node]).u32();
        let forward = e
            .call(
                MATRIX_TIMES_POINT,
                &args![rotation, local(0x304), FORWARD_AXIS],
            )
            .u32();
        let mut words = Vec::with_capacity(7);
        for i in 0..3 {
            words.push(e.mem.u32(point.addr() + 4 * i));
        }
        for i in 0..3 {
            words.push(e.mem.u32(forward + 4 * i));
        }
        words.push(body);
        let mut hit = vec![at];
        hit.extend(words);
        e.call(0x008d_0020, &hit);
    }

    let closest = e.mem.u32(collision(e, this) + COLLISION_CLOSEST_OBJECT);
    if closest != 0 {
        let closest = e.mem.u32(collision(e, this) + COLLISION_CLOSEST_OBJECT);
        let object = e.call(0x0043_b610, &args![closest]).u32();
        let rigid = if object == 0 {
            0
        } else {
            e.call(COLLISION_OBJECT_BODY, &args![object]).u32()
        };
        let material = if rigid == 0 {
            0
        } else {
            e.call(0x004a_e6a0, &args![rigid]).u32()
        };
        if material != 0 {
            let kind = e.call(0x0062_05a0, &args![material]).u32();
            if takes_bounce_path(kind) {
                fn_008ccce0(e, this, point, normal);
                return;
            }
        }
    }

    if e.vcall(target_at, 0x21c, &[]).bool() && !e.vcall(target_at, 0x38c, &[]).bool() {
        fn_008ccfb0(e, this, target);
        fn_008cc6d0(e, this, 3);
        return;
    }
    if fn_008cdd90(e, this) == 0 {
        let roll = e.call(0x0048_7f50, &[]).u32() % 100;
        let chance = e.call(SETTING_INT_VALUE, &args![SETTING_KEEP_CHANCE]).u32();
        if roll < e.mem.u32(chance) {
            let item = e.call(0x007a_f430, &args![this]).u32();
            e.vcall(target_at, 0x190, &args![item, 0u32, 1u32]);
            e.mem.set_u8(at + ADDED_TO_INVENTORY, 1);
        }
    }
}

/// The locals one bone placement of `fn_008cba30` uses (the exe's `EBP - n`):
/// the inverse transform, the sum, the result point, the inverse rotation
/// and the product.
struct BoneLocals {
    transform: u32,
    sum: u32,
    result: u32,
    rotation: u32,
    product: u32,
}

impl BoneLocals {
    const FIRST: BoneLocals = BoneLocals {
        transform: 0x1b4,
        sum: 0x1c0,
        result: 0x1cc,
        rotation: 0x1f0,
        product: 0x214,
    };
    const SECOND: BoneLocals = BoneLocals {
        transform: 0x298,
        sum: 0x2a4,
        result: 0x2b0,
        rotation: 0x2d4,
        product: 0x2f8,
    };
}

/// One bone placement of `fn_008cba30`: the record's point becomes
/// `origin + depth` in the bone's space (`origin` the ray start, or with `None` the bone's own position `0045bb80`, read where the exe reads it) (`00476a80`, the inverse of the
/// bone's transform `004b4880`, `00439e90`, `00524c40`), its rotation the
/// inverse of the bone's rotation (`004b46a0`) times the arrow's
/// (`0043f8d0`), and its closest object the bone.
#[allow(clippy::too_many_arguments)]
fn place_on_bone(
    e: &mut Engine,
    this: Ptr,
    node: u32,
    bone: u32,
    origin: Option<u32>,
    depth: u32,
    local: &dyn Fn(u32) -> u32,
    locals: BoneLocals,
) {
    let transform = local(locals.transform);
    e.call(0x0047_6a80, &args![transform]);
    let world = e.call(NODE_ROTATION, &args![bone]).u32();
    e.call(0x004b_4880, &args![world, transform]);
    let origin = match origin {
        Some(origin) => origin,
        None => e.call(0x0045_bb80, &args![bone]).u32(),
    };
    let sum = e
        .call(POINT_ADD, &args![origin, local(locals.sum), depth])
        .u32();
    let result = e
        .call(0x0052_4c40, &args![transform, local(locals.result), sum])
        .u32();
    let record = collision(e, this);
    copy_words(e, record + COLLISION_POINT, result, 3);
    let rotation = local(locals.rotation);
    e.call(CONSTRUCT_IN_PLACE, &args![rotation]);
    let world = e.call(NODE_ROTATION, &args![bone]).u32();
    e.call(MATRIX_INVERSE, &args![world, rotation]);
    let own = e.call(NODE_ROTATION, &args![node]).u32();
    let product = e
        .call(0x0043_f8d0, &args![rotation, local(locals.product), own])
        .u32();
    let record = collision(e, this);
    copy_words(e, record + COLLISION_ROTATION, product, 9);
    let record = collision(e, this);
    e.mem.set_u32(record + COLLISION_CLOSEST_OBJECT, bone);
}

/// The impulse part of `fn_008cba30`: the target's 3D is released from its
/// force (`00cb9650`) and set to motion 1 (`bhkWorld::SetMotion`), updated,
/// and pushed (`0062b660`) along the arrow's forward axis from 1024 units
/// behind its position with the impulse `006466e0(setting, 1)`.
fn push_target(e: &mut Engine, this: Ptr, node: u32, target_node: u32, local: &dyn Fn(u32) -> u32) {
    let at = this.addr();
    let setting = e.call(SETTING_VALUE, &args![SETTING_IMPULSE]).u32();
    let value = x87_quiet_bits(e.mem.u32(setting));
    let impulse = e.call(0x0064_66e0, &args![value, 1u32]).f64();
    let impulse = x87_store32(impulse);
    e.call(0x00cb_9650, &args![target_node]);
    e.call(SET_MOTION, &args![target_node, 1u32, 1u32, 0u32, 1u32]);
    let update = local(0x84);
    e.call(UPDATE_DATA, &args![update, 0.0f32, 0u32, 0u32]);
    e.call(NODE_UPDATE, &args![target_node, update]);
    let forward = local(0x90);
    let rotation = e.call(NODE_ROTATION, &args![node]).u32();
    e.call(MATRIX_TIMES_POINT, &args![rotation, forward, FORWARD_AXIS]);
    e.call(0x004a_0c10, &args![forward]);
    let back = x87_quiet_bits(e.mem.u32(FLOAT_MINUS_1024));
    let offset = e
        .call(POINT_SCALE, &args![forward, local(0x9c), back])
        .u32();
    let position = e.vcall(at, 0x1f4, &[]).u32();
    let from = e
        .call(POINT_ADD, &args![position, local(0xa8), offset])
        .u32();
    copy_words(e, forward, from, 3);
    let origin = local(0xc0);
    e.call(CONSTRUCT_IN_PLACE, &args![origin]);
    e.call(0x004a_3e00, &args![origin, forward]);
    e.call(
        0x0062_b660,
        &args![target_node, origin, x87_quiet_bits(impulse), 0u32],
    );
}

// Translated from 008cc6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The collision record's constructor: the in-place constructor
/// (`006815c0`) on its point (+0x4), normal (+0x10), velocity (+0x1C) and
/// rotation (+0x30). Returns `this`.
pub fn fn_008cc6f0(e: &mut Engine, this: Ptr) -> Ptr {
    for offset in [
        COLLISION_POINT,
        COLLISION_NORMAL,
        COLLISION_VELOCITY,
        COLLISION_ROTATION,
    ] {
        e.call(CONSTRUCT_IN_PLACE, &args![this.addr() + offset]);
    }
    this
}

// Translated from 008cc740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008cc740 - the compiler's exception frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated
/// The arrow hits the ground or a static at `point` (surface `normal`): a
/// new record of type 2 whose point is `point` plus 24 units along the
/// normalized `DeltaS`, identity rotation and the controller's velocity; a
/// kind at `point` that the exe's byte map sends there (`00457880` on `TES`) also takes the
/// bounce path (`fn_008ccce0`). State 1, the hit sound
/// (`Actor::FakeWeaponHitSound`, `008cfad0`, with the velocity's length),
/// the shooter's process told (`008d0600`), and for each of the arrow's
/// enchantment (`fn_008cdd90`, its effects at +0x18), `009ee040`'s (+0x30)
/// and `004f9bf0`'s (+0x18) the shooter's magic caster (+0x88) casts it at
/// the item (slots 0x40, 0x48, `MagicCaster::ReleaseCast`). With an
/// enchantment the arrow then drops (`fn_008cc6d0(3)`).
pub fn fn_008cc740(e: &mut Engine, this: Ptr, point: Ptr, normal: Ptr) {
    let at = this.addr();
    new_collision(e, this);
    e.with_stack(0x18, |e, locals| {
        // The direction (EBP-0x30) and the sum (EBP-0x3C).
        let direction = locals.addr();
        let sum = direction + 0xc;
        copy_words(e, direction, at + DELTA, 3);
        e.call(NORMALIZE, &args![direction]);
        let length = x87_quiet_bits(e.mem.u32(FLOAT_TWENTY_FOUR));
        e.call(0x0043_9180, &args![direction, length]);
        let record = collision(e, this);
        e.mem.set_u32(record + COLLISION_TYPE, 2);
        let record = collision(e, this);
        e.mem.set_u32(record + COLLISION_CLOSEST_OBJECT, 0);
        let record = collision(e, this);
        e.mem.set_u32(record + COLLISION_COLLIDEE, 0);
        let record = collision(e, this);
        copy_words(e, record + COLLISION_NORMAL, normal.addr(), 3);
        let moved = e.call(POINT_ADD, &args![point, sum, direction]).u32();
        let record = collision(e, this);
        copy_words(e, record + COLLISION_POINT, moved, 3);
    });
    let record = collision(e, this);
    copy_words(e, record + COLLISION_ROTATION, IDENTITY_ROTATION, 9);
    let record = collision(e, this);
    copy_words(e, record + COLLISION_VELOCITY, ZERO_POINT, 3);
    take_controller_velocity(e, this);
    let tes = e.mem.u32(TES);
    let kind = e.call(0x0045_7880, &args![tes, point]).u32();
    if takes_bounce_path(kind) {
        fn_008ccce0(e, this, point, normal);
    }
    e.mem.set_u32(at + STATE, 1);
    let velocity = collision(e, this) + COLLISION_VELOCITY;
    let speed = e.call(POINT_LENGTH, &args![velocity]).f64();
    let mut sound = vec![at, x87_store32(speed)];
    for i in 0..3 {
        sound.push(e.mem.u32(point.addr() + 4 * i));
    }
    sound.push(0);
    sound.push(kind);
    e.call(0x008c_fad0, &sound);
    let shooter = e.mem.u32(at + SHOOTER);
    if shooter != 0 && e.vcall(shooter, 0x428, &[]).u32() != 0 {
        let shooter = e.mem.u32(at + SHOOTER);
        let process = e.vcall(shooter, 0x428, &[]).u32();
        e.call(0x008d_0600, &args![process, point, 0u32]);
    }
    if e.mem.u32(at + SHOOTER) != 0 {
        if fn_008cdd90(e, this) != 0 {
            let item = fn_008cdd90(e, this);
            cast_on_hit(e, this, if item == 0 { 0 } else { item + 0x18 });
        }
        if e.call(0x009e_e040, &args![this]).u32() != 0 {
            let item = e.call(0x009e_e040, &args![this]).u32();
            cast_on_hit(e, this, if item == 0 { 0 } else { item + 0x30 });
        }
        if e.call(0x004f_9bf0, &args![this]).u32() != 0 {
            let item = e.call(0x004f_9bf0, &args![this]).u32();
            cast_on_hit(e, this, if item == 0 { 0 } else { item + 0x18 });
        }
    }
    if fn_008cdd90(e, this) != 0 {
        fn_008cc6d0(e, this, 3);
    }
}

/// One cast of `fn_008cc740`: on the shooter's magic caster (`+0x88`),
/// slot 0x40 with `spell`, slot 0x48 with 0, then
/// `MagicCaster::ReleaseCast` (`00815870`) with the arrow's item
/// (`007af430`).
fn cast_on_hit(e: &mut Engine, this: Ptr, spell: u32) {
    let caster = e.mem.u32(this.addr() + SHOOTER) + 0x88;
    e.vcall(caster, 0x40, &args![spell]);
    let caster = e.mem.u32(this.addr() + SHOOTER) + 0x88;
    e.vcall(caster, 0x48, &args![0u32]);
    let item = e.call(0x007a_f430, &args![this]).u32();
    let caster = e.mem.u32(this.addr() + SHOOTER) + 0x88;
    e.call(0x0081_5870, &args![caster, item]);
}

// Translated from 008ccce0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008ccce0 - the compiler's exception frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated
/// Makes the collision a bounce (type 3) at `point` with `normal`. An
/// existing record keeps its point and normal unless the new ones pass
/// `00439090` against the zero vector, loses its collidee and keeps the
/// state; otherwise a new record (point, normal, identity rotation, the
/// controller's velocity) is built and the state becomes 1.
pub fn fn_008ccce0(e: &mut Engine, this: Ptr, point: Ptr, normal: Ptr) {
    let at = this.addr();
    if collision(e, this) != 0 {
        if e.call(0x0043_9090, &args![normal, ZERO_POINT]).bool() {
            let record = collision(e, this);
            copy_words(e, record + COLLISION_NORMAL, normal.addr(), 3);
        }
        if e.call(0x0043_9090, &args![point, ZERO_POINT]).bool() {
            let record = collision(e, this);
            copy_words(e, record + COLLISION_POINT, point.addr(), 3);
        }
        let record = collision(e, this);
        e.mem.set_u32(record + COLLISION_TYPE, 3);
        let record = collision(e, this);
        e.mem.set_u32(record + COLLISION_COLLIDEE, 0);
        return;
    }
    let record = new_collision(e, this);
    e.mem.set_u32(record + COLLISION_TYPE, 3);
    let record = collision(e, this);
    e.mem.set_u32(record + COLLISION_CLOSEST_OBJECT, 0);
    let record = collision(e, this);
    e.mem.set_u32(record + COLLISION_COLLIDEE, 0);
    let record = collision(e, this);
    copy_words(e, record + COLLISION_NORMAL, normal.addr(), 3);
    let record = collision(e, this);
    copy_words(e, record + COLLISION_POINT, point.addr(), 3);
    let record = collision(e, this);
    copy_words(e, record + COLLISION_ROTATION, IDENTITY_ROTATION, 9);
    let record = collision(e, this);
    copy_words(e, record + COLLISION_VELOCITY, ZERO_POINT, 3);
    take_controller_velocity(e, this);
    e.mem.set_u32(at + STATE, 1);
}

// Translated from 008ccfb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008ccfb0 - the compiler's exception frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated
/// Makes the arrow fall (record type 4): an existing record only changes
/// its type; otherwise a new one with `collidee`, zero point and normal,
/// identity rotation and the controller's velocity is built. The state
/// becomes 1 either way.
pub fn fn_008ccfb0(e: &mut Engine, this: Ptr, collidee: Ptr) {
    let at = this.addr();
    if collision(e, this) != 0 {
        let record = collision(e, this);
        e.mem.set_u32(record + COLLISION_TYPE, 4);
    } else {
        let record = new_collision(e, this);
        e.mem.set_u32(record + COLLISION_TYPE, 4);
        let record = collision(e, this);
        e.mem.set_u32(record + COLLISION_CLOSEST_OBJECT, 0);
        let record = collision(e, this);
        e.mem.set_u32(record + COLLISION_COLLIDEE, collidee.addr());
        let record = collision(e, this);
        copy_words(e, record + COLLISION_NORMAL, ZERO_POINT, 3);
        let record = collision(e, this);
        copy_words(e, record + COLLISION_POINT, ZERO_POINT, 3);
        let record = collision(e, this);
        copy_words(e, record + COLLISION_ROTATION, IDENTITY_ROTATION, 9);
        let record = collision(e, this);
        copy_words(e, record + COLLISION_VELOCITY, ZERO_POINT, 3);
        take_controller_velocity(e, this);
    }
    e.mem.set_u32(at + STATE, 1);
}

// Translated from 008cd210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies the collision record: without one, false. Without a collision
/// object on the arrow's 3D the message `"AI: No collision data was found
/// on this arrow"` is printed, the record freed and false returned.
/// Otherwise the record's type picks the handler (0 `fn_008cd390`, 1
/// `fn_008cd520`, 2 `fn_008cd6d0`, 3 `fn_008cd780`, 4 `fn_008cdaf0`, each
/// with the object's rigid body; another type does nothing and counts as
/// false); when it fails the record is freed and the state set to 0.
/// Returns the handler's result.
pub fn fn_008cd210(e: &mut Engine, this: Ptr) -> bool {
    let at = this.addr();
    if collision(e, this) == 0 {
        return false;
    }
    let object = if e.vcall(at, 0x1d0, &[]).u32() == 0 {
        0
    } else {
        let node = e.vcall(at, 0x1d0, &[]).u32();
        e.call(FIND_FIRST_COLLISION_OBJECT, &args![node]).u32()
    };
    if object == 0 {
        e.call(PRINT_MESSAGE, &args![MESSAGE_NO_COLLISION_DATA]);
        let record = collision(e, this);
        e.call(OPERATOR_DELETE, &args![record]);
        e.mem.set_u32(at + COLLISION, 0);
        return false;
    }
    let body = Ptr::new(e.call(COLLISION_OBJECT_BODY, &args![object]).u32());
    let result = match e.mem.u32(collision(e, this) + COLLISION_TYPE) {
        0 => fn_008cd390(e, this, body),
        1 => fn_008cd520(e, this, body),
        2 => fn_008cd6d0(e, this, body),
        3 => fn_008cd780(e, this, body),
        4 => fn_008cdaf0(e, this, body),
        _ => false,
    };
    if !result {
        let record = collision(e, this);
        if record != 0 {
            e.call(OPERATOR_DELETE, &args![record]);
            e.mem.set_u32(at + COLLISION, 0);
        }
        e.mem.set_u32(at + STATE, 0);
    }
    result
}

// Translated from 008cd390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collision type 0 (the record `fn_008cba30` builds for an actor): the arrow's own physics stops (slot 0x2A0)
/// and, when the collidee passes slot 0x100 and the record has a bone, the
/// arrow's 3D is taken out of the world (`bhkWorld::RemoveObjects`,
/// `TESHavokUtilities::RemoveHavokFromSceneGraph`), placed at the record's
/// point and rotation, scaled by `009cdae0(3D) / 008d01e0(bone)`, moved
/// from the shadow scene node (`00450b80(0)`) under the bone (slot 0xDC) and
/// back, and the collidee's process told (slot 0x6A4). Without a bone:
/// false. Without an actor collidee: the message `"AI: An arrow thinks it
/// is colliding with an Actor, ..."` and false. `body` (`RET 4`) is not
/// read.
pub fn fn_008cd390(e: &mut Engine, this: Ptr, _unused_1: Ptr) -> bool {
    let at = this.addr();
    let node = e.vcall(at, 0x1d0, &[]).u32();
    e.vcall(at, 0x2a0, &[]);
    let collidee = e.mem.u32(collision(e, this) + COLLISION_COLLIDEE);
    if collidee == 0 || !e.vcall(collidee, 0x100, &[]).bool() {
        e.call(PRINT_MESSAGE, &args![MESSAGE_NO_ACTOR]);
        return false;
    }
    let bone = e.mem.u32(collision(e, this) + COLLISION_CLOSEST_OBJECT);
    if bone == 0 {
        return false;
    }
    e.call(REMOVE_OBJECTS, &args![node, 1u32, 0u32]);
    e.call(0x0062_bc10, &args![node]);
    let point = collision(e, this) + COLLISION_POINT;
    e.call(NODE_SET_TRANSLATE, &args![node, point]);
    let rotation = collision(e, this) + COLLISION_ROTATION;
    e.call(NODE_SET_ROTATE, &args![node, rotation]);
    let own = e.call(0x009c_dae0, &args![node]).f64();
    let other = e.call(0x008d_01e0, &args![bone]).f64();
    let scale = x87_store32(x87_div(own, other));
    e.call(NODE_SET_SCALE, &args![node, x87_quiet_bits(scale)]);
    let shadow = e.call(0x0045_0b80, &args![0u32]).u32();
    e.call(0x00b5_b1c0, &args![shadow, node]);
    e.vcall(bone, 0xdc, &args![node, 1u32]);
    e.call(0x00b5_eeb0, &args![shadow, node]);
    let collidee = e.mem.u32(collision(e, this) + COLLISION_COLLIDEE);
    if collidee != 0 && e.call(REFERENCE_PROCESS, &args![collidee]).u32() != 0 {
        let process = e.call(REFERENCE_PROCESS, &args![collidee]).u32();
        e.vcall(process, 0x6a4, &args![this]);
    }
    true
}

// Translated from 008cd520 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008cd520 - the /GS cookie check (00ec408c) is not translated
/// Collision type 1 (a reference, as the exe's message calls it): the arrow's physics stops (slot 0x2A0);
/// without a collidee the message `"AI: An arrow thinks it is colliding
/// with an Reference, ..."` and false; without a collision object on the
/// record's bone, false. Otherwise the arrow's 3D gets motion 4
/// (`bhkWorld::SetMotion`), `body` takes the filter built from (6, the upper half
/// of the hit body's filter, 0) (`00622290`), its low 7 bits set to 0xF unless
/// the hit body's are 4, the arrow joins the list `004610f0` gives on the
/// process lists (`005ae3d0`), and when `body` is not null and the hit body's
/// dynamic mass (`bhkRigidBody::GetDynamicMass`) is above zero, `body`'s
/// velocity (slot 0xF0) goes to `0062a7e0` with the bone. Returns true.
pub fn fn_008cd520(e: &mut Engine, this: Ptr, body: Ptr) -> bool {
    let at = this.addr();
    e.vcall(at, 0x2a0, &[]);
    if e.mem.u32(collision(e, this) + COLLISION_COLLIDEE) == 0 {
        e.call(PRINT_MESSAGE, &args![MESSAGE_NO_REFERENCE]);
        return false;
    }
    let bone = e.mem.u32(collision(e, this) + COLLISION_CLOSEST_OBJECT);
    let object = e.call(FIND_FIRST_COLLISION_OBJECT, &args![bone]).u32();
    if object == 0 {
        return false;
    }
    let hit_body = e.call(COLLISION_OBJECT_BODY, &args![object]).u32();
    let node = e.vcall(at, 0x1d0, &[]).u32();
    e.call(SET_MOTION, &args![node, 4u32, 1u32, 0u32, 1u32]);
    e.with_stack(0x40, |e, frame| {
        // Locals at EBP-0x14, -0x18, -0x1c, -0x24 and the velocity at -0x40.
        let local = |offset: u32| frame.addr() + 0x40 - offset;
        let hit_filter = e.call(BODY_GET_FILTER, &args![hit_body, local(0x14)]).u32();
        let layer = e.call(FILTER_HIGH_HALF, &args![hit_filter]).u32();
        let filter = local(0x18);
        e.call(MAKE_FILTER, &args![filter, 6u32, layer, 0u32]);
        let hit_filter = e.call(BODY_GET_FILTER, &args![hit_body, local(0x1c)]).u32();
        if e.call(FILTER_LOW_BITS, &args![hit_filter]).u32() != 4 {
            e.call(FILTER_SET_LOW_BITS, &args![filter, 0xfu32]);
        }
        let word = e.mem.u32(filter);
        e.call(BODY_SET_FILTER, &args![body, word]);
        let list = e.call(0x0046_10f0, &args![PROCESS_LISTS]).u32();
        if list != 0 {
            let item = local(0x24);
            e.mem.set_u32(item, at);
            e.call(0x005a_e3d0, &args![list, item]);
        }
        if !body.is_null() {
            let mass = e.call(0x0062_ab20, &args![hit_body]).f64();
            // `FCOMP 0.0; TEST AH,0x41; JNZ`: only above zero (ordered).
            if mass > e.mem.f64(DOUBLE_ZERO) {
                let velocity = local(0x40);
                e.call(CONSTRUCT_IN_PLACE, &args![velocity]);
                e.vcall(body.addr(), 0xf0, &args![velocity]);
                let bone = e.mem.u32(collision(e, this) + COLLISION_CLOSEST_OBJECT);
                e.call(0x0062_a7e0, &args![velocity, bone]);
            }
        }
    });
    true
}

// Translated from 008cd6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards `first` to `00cb2d90` on `this` and `second` to `004a3f10` on
/// `this + 0x30` (`RET 8`).
pub fn fn_008cd6a0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(0x00cb_2d90, &args![this, first]);
    e.call(0x004a_3f10, &args![this.addr() + 0x30, second]);
}

// Translated from 008cd6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collision type 2 (the record `fn_008cc740` builds): the arrow's physics stops
/// (slot 0x2A0), its 3D and location go to the record's point, and `body`'s
/// filter gets the low bits 0xF and `00810f90(0)`, then the filter built from
/// (0xF, `fn_008cddb0`, 0) (`00622290`). Returns true.
pub fn fn_008cd6d0(e: &mut Engine, this: Ptr, body: Ptr) -> bool {
    let at = this.addr();
    e.vcall(at, 0x2a0, &[]);
    let point = collision(e, this) + COLLISION_POINT;
    let node = e.vcall(at, 0x1d0, &[]).u32();
    e.call(NODE_SET_TRANSLATE, &args![node, point]);
    let point = collision(e, this) + COLLISION_POINT;
    e.call(SET_LOCATION_ON_REFERENCE, &args![this, point]);
    e.with_stack(8, |e, frame| {
        // The filter word (EBP-0x4) and the new one (EBP-0x8).
        let filter = frame.addr() + 4;
        let fresh = frame.addr();
        e.call(BODY_GET_FILTER, &args![body, filter]);
        e.call(FILTER_SET_LOW_BITS, &args![filter, 0xfu32]);
        e.call(0x0081_0f90, &args![filter, 0u32]);
        let word = e.mem.u32(filter);
        e.call(BODY_SET_FILTER, &args![body, word]);
        let group = fn_008cddb0(e);
        let made = e
            .call(MAKE_FILTER, &args![fresh, 0xfu32, group, 0u32])
            .u32();
        let word = e.mem.u32(made);
        e.call(BODY_SET_FILTER, &args![body, word]);
    });
    true
}

// Translated from 008cd780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collision type 3 (the record `fn_008ccce0` builds): the arrow's physics stops (slot 0x2A0);
/// unless the save/load game's test (`0047c850`) holds, its 3D moves to the
/// record's point and is updated. Its 3D gets motion 1
/// (`bhkWorld::SetMotion`) and `body` the filter built from (4,
/// `fn_008cddb0`, 0). Unless that test holds again, `body` is sent off: the
/// reflection of `-DeltaS` (normalized, against the normalized record normal:
/// `2 (n . d) n - d`, `0063c8a0`) times the bounce-speed setting and the
/// speed (`005615d0`), and a spin about the arrow's axes (`00561630`) from
/// the record normal in the 3D's space (`|z|` replaced by `|y| * 00631070(z)`
/// when below `|y|`, `y` zeroed, normalized) times the bounce-spin setting.
/// Last the 3D's collision object is woken (`00573ed0(1)`). Returns true.
pub fn fn_008cd780(e: &mut Engine, this: Ptr, body: Ptr) -> bool {
    let at = this.addr();
    e.vcall(at, 0x2a0, &[]);
    let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
    if !e.call(0x0047_c850, &args![save_load_game]).bool() {
        let point = collision(e, this) + COLLISION_POINT;
        let node = e.vcall(at, 0x1d0, &[]).u32();
        e.call(NODE_SET_TRANSLATE, &args![node, point]);
        e.with_stack(0x10, |e, update| {
            e.call(UPDATE_DATA, &args![update, 0.0f32, 0u32, 0u32]);
            let node = e.vcall(at, 0x1d0, &[]).u32();
            e.call(NODE_UPDATE, &args![node, update]);
        });
    }
    let node = e.vcall(at, 0x1d0, &[]).u32();
    e.call(SET_MOTION, &args![node, 1u32, 1u32, 0u32, 1u32]);
    e.with_stack(0xc0, |e, frame| {
        let local = |offset: u32| frame.addr() + 0xc0 - offset;
        let group = fn_008cddb0(e);
        let made = e
            .call(MAKE_FILTER, &args![local(0x6c), 4u32, group, 0u32])
            .u32();
        let word = e.mem.u32(made);
        e.call(BODY_SET_FILTER, &args![body, word]);
        let save_load_game = e.mem.u32(SAVE_LOAD_GAME);
        if e.call(0x0047_c850, &args![save_load_game]).bool() {
            return;
        }
        let back = local(0x68);
        let minus_one = x87_quiet_bits(e.mem.u32(FLOAT_MINUS_ONE));
        e.call(SCALE_INTO, &args![back, minus_one, at + DELTA]);
        e.call(NORMALIZE, &args![back]);
        let normal = collision(e, this) + COLLISION_NORMAL;
        e.call(NORMALIZE, &args![normal]);
        let normal = collision(e, this) + COLLISION_NORMAL;
        let dot = e.call(0x004b_6190, &args![normal, back]).f64();
        let dot = x87_store32(dot);
        let normal = collision(e, this) + COLLISION_NORMAL;
        let along = e
            .call(SCALE_INTO, &args![local(0x78), x87_quiet_bits(dot), normal])
            .u32();
        let rest = e
            .call(POINT_SUBTRACT, &args![along, local(0x84), back])
            .u32();
        let two = x87_quiet_bits(e.mem.u32(FLOAT_TWO));
        e.call(SCALE_INTO, &args![local(0x44), two, rest]);
        e.call(0x0063_c8a0, &args![back, local(0x44)]);
        let setting = e.call(SETTING_VALUE, &args![SETTING_BOUNCE_SPEED]).u32();
        let speed = x87_mul(
            x87_load32(e.mem.u32(setting)),
            x87_m32(e.mem.u32(at + SPEED)),
        );
        let speed = x87_store32(speed);
        let launch = e
            .call(SCALE_INTO, &args![local(0x90), x87_quiet_bits(speed), back])
            .u32();
        e.call(0x0056_15d0, &args![body, launch]);

        let inverse = local(0x34);
        e.call(CONSTRUCT_IN_PLACE, &args![inverse]);
        let node = e.vcall(at, 0x1d0, &[]).u32();
        let rotation = e.call(NODE_ROTATION, &args![node]).u32();
        e.call(MATRIX_INVERSE, &args![rotation, inverse]);
        let local_normal = local(0x5c);
        let normal = collision(e, this) + COLLISION_NORMAL;
        e.call(MATRIX_TIMES_POINT, &args![inverse, local_normal, normal]);
        let y = local_normal + 4;
        let z = local_normal + 8;
        let side = e
            .call(0x0040_8820, &args![x87_quiet_bits(e.mem.u32(y))])
            .f64();
        let up = e
            .call(0x0040_8820, &args![x87_quiet_bits(e.mem.u32(z))])
            .f64();
        // `FCOMP; TEST AH,5; JP`: only when `|z|` is below `|y|`.
        if up < side {
            let side = e
                .call(0x0040_8820, &args![x87_quiet_bits(e.mem.u32(y))])
                .f64();
            let sign = e
                .call(0x0063_1070, &args![x87_quiet_bits(e.mem.u32(z))])
                .f64();
            e.mem.set_u32(z, x87_store32(x87_mul(sign, side)));
        }
        e.mem.set_u32(y, 0.0f32.to_bits());
        e.call(NORMALIZE, &args![local_normal]);
        let setting = e.call(SETTING_VALUE, &args![SETTING_BOUNCE_SPIN]).u32();
        let x = -x87_load32(e.mem.u32(local_normal));
        let third = x87_store32(x87_mul(x, x87_m32(e.mem.u32(setting))));
        let setting = e.call(SETTING_VALUE, &args![SETTING_BOUNCE_SPIN]).u32();
        let first = x87_mul(
            x87_load32(e.mem.u32(setting)),
            x87_m32(e.mem.u32(local_normal + 8)),
        );
        let first = x87_store32(first);
        let spin = local(0x50);
        e.call(
            0x0041_6870,
            &args![spin, x87_quiet_bits(first), 0.0f32, x87_quiet_bits(third)],
        );
        let node = e.vcall(at, 0x1d0, &[]).u32();
        let rotation = e.call(NODE_ROTATION, &args![node]).u32();
        let turned = e
            .call(MATRIX_TIMES_POINT, &args![rotation, local(0x9c), spin])
            .u32();
        e.call(0x0056_1630, &args![body, turned]);
    });
    let node = e.vcall(at, 0x1d0, &[]).u32();
    let object = e.call(FIND_FIRST_COLLISION_OBJECT, &args![node]).u32();
    if object != 0 {
        e.call(0x0057_3ed0, &args![object, 1u32]);
    }
    true
}

// Translated from 008cdaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collision type 4 (the record `fn_008ccfb0` builds): the arrow's physics stops (slot 0x2A0), its
/// 3D gets motion 1 (`bhkWorld::SetMotion`), and `body` the filter built
/// from (4, the group, 0), the group being that of the collidee's
/// character controller (`00819250`) when the collidee is an `Actor`
/// (`__RTDynamicCast`) with one, else `fn_008cddb0`'s. `body` takes the
/// record's velocity (`005615d0`) and the object of `00620b80`'s result
/// (`004b5a20`) is woken (`00573ed0(1)`). Returns true.
pub fn fn_008cdaf0(e: &mut Engine, this: Ptr, body: Ptr) -> bool {
    let at = this.addr();
    e.vcall(at, 0x2a0, &[]);
    let node = e.vcall(at, 0x1d0, &[]).u32();
    e.call(SET_MOTION, &args![node, 1u32, 1u32, 0u32, 1u32]);
    let collidee = e.mem.u32(collision(e, this) + COLLISION_COLLIDEE);
    let actor = e
        .call(
            RT_DYNAMIC_CAST,
            &args![collidee, 0u32, REFERENCE_TYPE, ACTOR_TYPE, 0u32],
        )
        .u32();
    e.with_stack(8, |e, frame| {
        // The filter words (EBP-0xC and EBP-0x10).
        if actor != 0 && e.call(GET_CHAR_CONTROLLER, &args![actor]).u32() != 0 {
            let controller = e.call(GET_CHAR_CONTROLLER, &args![actor]).u32();
            let group = e.call(0x0081_9250, &args![controller]).u32();
            let made = e
                .call(MAKE_FILTER, &args![frame.addr() + 4, 4u32, group, 0u32])
                .u32();
            let word = e.mem.u32(made);
            e.call(BODY_SET_FILTER, &args![body, word]);
        } else {
            let group = fn_008cddb0(e);
            let made = e
                .call(MAKE_FILTER, &args![frame.addr(), 4u32, group, 0u32])
                .u32();
            let word = e.mem.u32(made);
            e.call(BODY_SET_FILTER, &args![body, word]);
        }
    });
    let velocity = collision(e, this) + COLLISION_VELOCITY;
    e.call(0x0056_15d0, &args![body, velocity]);
    let owner = e.call(0x0062_0b80, &args![body]).u32();
    let object = e.call(0x004b_5a20, &args![owner]).u32();
    if object != 0 {
        e.call(0x0057_3ed0, &args![object, 1u32]);
    }
    true
}

// Translated from 008cdbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the arrow to the process level its cell calls for. The wanted
/// level is 0 when `TES::IsCellLoaded(cell, 1)`, else 1 when
/// `IsCellLoaded(cell, 0)`, else 3; it is only worked out when the arrow
/// has 3D or the model loader's test (`00445750`) holds. The cell is the
/// arrow's, or for a stuck arrow (state 2) the collidee's; an arrow stuck in
/// the player only moves to high (slot 0x240) when not already there and
/// returns true. Same level: true. Level 0: slot 0x240 (to high). Level 1
/// while not flying (state not 0): slot 0x24C (to middle high). Otherwise
/// the arrow deletes itself (slot 0x10 with 1) and false is returned.
pub fn fn_008cdbf0(e: &mut Engine, this: Ptr) -> bool {
    let at = this.addr();
    let process = e.mem.u32(at + PROCESS);
    let level = e.call(PROCESS_LEVEL, &args![process]).u32();
    let mut wanted = 3u32;
    let mut cell = e.call(PARENT_CELL, &args![this]).u32();
    if e.mem.u32(at + STATE) == 2 && collision(e, this) != 0 {
        let collidee = e.mem.u32(collision(e, this) + COLLISION_COLLIDEE);
        if collidee != 0 {
            if collidee == e.mem.u32(PLAYER) {
                if level != 0 {
                    e.vcall(at, 0x240, &[]);
                }
                return true;
            }
            cell = e.call(PARENT_CELL, &args![collidee]).u32();
        }
    }
    let has_3d = e.vcall(at, 0x1d0, &[]).u32() != 0;
    let model_loader = e.mem.u32(MODEL_LOADER);
    if has_3d || e.call(0x0044_5750, &args![model_loader, this]).bool() {
        let tes = e.mem.u32(TES);
        if e.call(0x0045_11e0, &args![tes, cell, 1u32]).bool() {
            wanted = 0;
        } else {
            let tes = e.mem.u32(TES);
            if e.call(0x0045_11e0, &args![tes, cell, 0u32]).bool() {
                wanted = 1;
            }
        }
    }
    let flying = e.mem.u32(at + STATE) == 0;
    if level == wanted {
        return true;
    }
    match wanted {
        0 => {
            e.vcall(at, 0x240, &[]);
            true
        }
        1 if !flying => {
            e.vcall(at, 0x24c, &[]);
            true
        }
        _ => {
            // `this` is never null here; the exe still tests it.
            if at != 0 {
                e.vcall(at, 0x10, &args![1u32]);
            }
            false
        }
    }
}

// Translated from 008cddb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value `005dbe10` returns, asked once and cached in the word at
/// `011dfc9c` (asked again while it is 0).
pub fn fn_008cddb0(e: &mut Engine) -> u32 {
    if e.mem.u32(CACHED_LAYER_WORD) == 0 {
        let value = e.call(0x005d_be10, &[]).u32();
        e.mem.set_u32(CACHED_LAYER_WORD, value);
    }
    e.mem.u32(CACHED_LAYER_WORD)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008c_ab00, fn_008cab00(Ptr)),
        entry!(0x008c_aab0, fn_008caab0(Ptr) -> Ptr),
        entry!(0x008c_aae0, fn_008caae0(Ptr) -> Ptr),
        entry!(0x008c_c6d0, fn_008cc6d0(Ptr, u32)),
        entry!(0x008c_c730, fn_008cc730(Ptr, u32, u32, u32, u32)),
        entry!(0x008c_dd90, fn_008cdd90(Ptr) -> u32),
        entry!(0x008c_aa70, fn_008caa70(Ptr, Ptr) -> Ptr),
        entry!(0x008c_ab20, fn_008cab20(Ptr) -> Ptr),
        entry!(0x008c_ac30, fn_008cac30(Ptr, u32) -> Ptr),
        entry!(0x008c_ac60, fn_008cac60(Ptr)),
        entry!(0x008c_aea0, fn_008caea0(Ptr, Ptr, f32)),
        entry!(0x008c_b380, fn_008cb380(Ptr, u32)),
        entry!(0x008c_b3c0, fn_008cb3c0(Ptr, f32)),
        entry!(0x008c_ba30, fn_008cba30(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x008c_c6f0, fn_008cc6f0(Ptr) -> Ptr),
        entry!(0x008c_c740, fn_008cc740(Ptr, Ptr, Ptr)),
        entry!(0x008c_cce0, fn_008ccce0(Ptr, Ptr, Ptr)),
        entry!(0x008c_cfb0, fn_008ccfb0(Ptr, Ptr)),
        entry!(0x008c_d210, fn_008cd210(Ptr) -> bool),
        entry!(0x008c_d390, fn_008cd390(Ptr, Ptr) -> bool),
        entry!(0x008c_d520, fn_008cd520(Ptr, Ptr) -> bool),
        entry!(0x008c_d6a0, fn_008cd6a0(Ptr, u32, u32)),
        entry!(0x008c_d6d0, fn_008cd6d0(Ptr, Ptr) -> bool),
        entry!(0x008c_d780, fn_008cd780(Ptr, Ptr) -> bool),
        entry!(0x008c_daf0, fn_008cdaf0(Ptr, Ptr) -> bool),
        entry!(0x008c_dbf0, fn_008cdbf0(Ptr) -> bool),
        entry!(0x008c_ddb0, fn_008cddb0() -> u32),
    ]
}

#[cfg(test)]
mod b023_tests {
    use super::*;

    #[test]
    fn setter_getter_and_the_empty_body() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xac);
        e.call(0x008c_c6d0, &[this, 0x1234_5678]);
        assert_eq!(e.mem.u32(this + 0x8c), 0x1234_5678);
        e.mem.set_u32(this + 0xa8, 0x9abc);
        assert_eq!(e.call(0x008c_dd90, &[this]).u32(), 0x9abc);
        let before = e.mem.bytes(this, 0xac);
        let r = e.call(0x008c_c730, &[this, 1, 2, 3, 4]);
        assert_eq!(r, Ret::default());
        assert_eq!(e.mem.bytes(this, 0xac), before);
    }
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_008cab00_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0044_fb00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008c_ab00, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0044_fb00), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn fn_008caab0_runs_the_base_then_clears_the_two_words() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR_0044FAE0, |e, a| {
            e.mem.set_u32(a[0] + 0x40, 0x11);
            e.mem.set_u32(a[0] + 0x44, 0x22);
            Ret::default()
        });
        let this = e.mem.alloc(0x50);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008c_aab0, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_0044FAE0),
            vec![vec![this]]
        );
        assert_eq!(e.mem.u32(this + 0x40), 0);
        assert_eq!(e.mem.u32(this + 0x44), 0);
    }

    #[test]
    fn fn_008caae0_clears_the_word_at_0c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0x77);
        e.mem.set_u32(this + 0x10, 0x88);
        assert_eq!(e.call(0x008c_aae0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x88);
    }
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ret_f(st0: f64) -> Ret {
        Ret {
            st0,
            ..Ret::default()
        }
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

    /// Doubles that return nothing for every listed callee.
    fn stub_all(e: &mut Engine, callees: &[u32]) {
        for &callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
    }

    /// A double with a fixed integer result.
    fn stub(e: &mut Engine, callee: u32, eax: u32) {
        e.register_double(callee, move |_, _| ret(eax));
    }

    /// A double that returns its argument word `index` (an `out` pointer).
    fn stub_returning_arg(e: &mut Engine, callee: u32, index: usize) {
        e.register_double(callee, move |_, a| ret(a[index]));
    }

    /// An object of `size` bytes whose vtable sends slot `n` to `base + n`.
    fn object(e: &mut Engine, size: u32, base: u32) -> u32 {
        let this = e.mem.alloc(size);
        let table = e.mem.alloc(0x800);
        for slot in (0..0x800).step_by(4) {
            e.mem.set_u32(table + slot, base + slot);
        }
        e.mem.set_u32(this, table);
        this
    }

    const ARROW_SLOTS: u32 = 0x7100_0000;
    const TARGET_SLOTS: u32 = 0x7200_0000;
    const OTHER_SLOTS: u32 = 0x7300_0000;

    /// Maps the exe's data pages the code reads and writes the values the
    /// file holds there.
    fn map_data(e: &mut Engine) {
        e.map(0x0101_0000, 0x0008_0000);
        e.map(0x011a_0000, 0x0006_0000);
        for (address, value) in [
            (DOUBLE_ZERO, 0.0f64),
            (DOUBLE_HALF, 0.5),
            (DOUBLE_TWO, 2.0),
            (DOUBLE_THREE, 3.0),
            (DOUBLE_EIGHT, 8.0),
            (DOUBLE_EIGHTEEN, 18.0),
        ] {
            e.mem.set_f64(address, value);
        }
        for (address, value) in [
            (FLOAT_MAX, f32::MAX),
            (FLOAT_FIFTY, 50.0f32),
            (FLOAT_TENTH, 0.1),
            (FLOAT_LOWEST, f32::MIN),
            (FLOAT_TWENTY_FOUR, 24.0),
            (FLOAT_MINUS_ONE, -1.0),
            (FLOAT_TWO, 2.0),
            (FLOAT_512, 512.0),
            (FLOAT_MINUS_1024, -1024.0),
            (FLOAT_200, 200.0),
            (FLOAT_EIGHTEEN, 18.0),
        ] {
            e.mem.set_f32(address, value);
        }
        for i in 0..9 {
            let value = if i % 4 == 0 { 1.0f32 } else { 0.0 };
            e.mem.set_f32(IDENTITY_ROTATION + 4 * i, value);
        }
        e.mem.set_f32(FORWARD_AXIS + 4, 1.0);
    }

    fn arrow(e: &mut Engine) -> u32 {
        object(e, 0xc8, ARROW_SLOTS)
    }

    fn point(e: &mut Engine, x: f32, y: f32, z: f32) -> u32 {
        let p = e.mem.alloc(12);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        e.mem.set_f32(p + 8, z);
        p
    }

    fn new_allocates(e: &mut Engine) {
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
    }

    #[test]
    fn fn_008caa70_copies_three_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(12);
        let source = point(&mut e, 1.0, 2.0, 3.0);
        assert_eq!(e.call(0x008c_aa70, &[this, source]).u32(), this);
        assert_eq!(e.mem.bytes(this, 12), e.mem.bytes(source, 12));
    }

    #[test]
    fn fn_008cab20_builds_the_arrow_and_counts_it() {
        let mut e = Engine::new();
        map_data(&mut e);
        new_allocates(&mut e);
        stub_all(&mut e, &[0x0092_eb50, CONSTRUCT_IN_PLACE, 0x0046_a010]);
        stub_returning_arg(&mut e, 0x008d_7510, 0);
        e.mem.set_u32(ARROW_COUNT, 4);
        let this = e.mem.alloc(0xc8);
        e.mem.set_u8(this + ADDED_TO_INVENTORY, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008c_ab20, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), VTABLE_2);
        assert_eq!(calls_to(&e, CONSTRUCT_IN_PLACE), vec![vec![this + DELTA]]);
        assert_eq!(calls_to(&e, 0x0046_a010), vec![vec![this, 1]]);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x46c]]);
        let process = calls_to(&e, 0x008d_7510)[0][0];
        assert_eq!(e.mem.u32(this + PROCESS), process);
        assert_eq!(e.mem.u8(this + ADDED_TO_INVENTORY), 0);
        assert_eq!(e.mem.u32(ARROW_COUNT), 5);
    }

    /// Doubles for the destructor body with nothing to tell.
    fn quiet_destructor(e: &mut Engine) {
        stub_all(
            e,
            &[
                0x0046_10f0,
                REFERENCE_3D,
                OPERATOR_DELETE,
                0x0057_02e0,
                0x0042_26e0,
                0x0092_ed60,
                0x0092_ec40,
            ],
        );
    }

    #[test]
    fn fn_008cac30_frees_only_with_bit_0() {
        let mut e = Engine::new();
        map_data(&mut e);
        quiet_destructor(&mut e);
        let this = arrow(&mut e);
        e.mem.set_u8(this + NEEDS_FINISH_INIT_LOAD_GAME, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008c_ac30, &[this, 0]).u32(), this);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0]]);
        e.call_log = Some(vec![]);
        e.call(0x008c_ac30, &[this, 1]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0], vec![this]]);
        assert_eq!(calls_to(&e, 0x0092_ec40), vec![vec![this]]);
    }

    #[test]
    fn fn_008cac60_releases_list_force_and_record() {
        let mut e = Engine::new();
        map_data(&mut e);
        quiet_destructor(&mut e);
        let this = arrow(&mut e);
        let record = e.mem.alloc(COLLISION_SIZE);
        e.mem.set_u32(this + COLLISION, record);
        e.mem.set_u32(record + COLLISION_TYPE, 1);
        e.mem.set_u32(record + COLLISION_CLOSEST_OBJECT, 0x4444);
        e.mem.set_u32(ARROW_COUNT, 2);
        stub(&mut e, 0x0046_10f0, 0x5150);
        e.register(0x0090_5330, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(0x0101_0000, item);
            Ret::default()
        });
        stub(&mut e, REFERENCE_3D, 0x3d00);
        stub(&mut e, FIND_FIRST_COLLISION_OBJECT, 0x0b00);
        let body = object(&mut e, 0x10, OTHER_SLOTS);
        stub(&mut e, COLLISION_OBJECT_BODY, body);
        stub_all(
            &mut e,
            &[OTHER_SLOTS + 0xf0, CONSTRUCT_IN_PLACE, 0x0062_ab60],
        );
        e.call_log = Some(vec![]);
        e.call(0x008c_ac60, &[this]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(ARROW_COUNT), 1);
        assert_eq!(calls_to(&e, 0x0090_5330)[0][0], 0x5150);
        assert_eq!(e.mem.u32(0x0101_0000), this);
        assert_eq!(
            calls_to(&e, FIND_FIRST_COLLISION_OBJECT),
            vec![vec![0x3d00]]
        );
        let velocity = calls_to(&e, OTHER_SLOTS + 0xf0)[0][1];
        assert_eq!(calls_to(&e, 0x0062_ab60), vec![vec![velocity, 0x4444]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![record]]);
        assert_eq!(calls_to(&e, 0x0057_02e0), vec![vec![this, 0, 1]]);
        assert_eq!(calls_to(&e, 0x0092_ed60), vec![vec![this, 0]]);
    }

    #[test]
    fn fn_008cac60_tells_the_struck_actor() {
        let mut e = Engine::new();
        map_data(&mut e);
        quiet_destructor(&mut e);
        let this = arrow(&mut e);
        let record = e.mem.alloc(COLLISION_SIZE);
        let actor = object(&mut e, 0x100, TARGET_SLOTS);
        let process = object(&mut e, 0x10, OTHER_SLOTS);
        e.mem.set_u32(this + COLLISION, record);
        e.mem.set_u32(record + COLLISION_COLLIDEE, actor);
        stub(&mut e, TARGET_SLOTS + 0x100, 1);
        stub(&mut e, REFERENCE_PROCESS, process);
        stub(&mut e, 0x0042_26e0, 1);
        stub_all(&mut e, &[OTHER_SLOTS + 0x6a8]);
        e.call_log = Some(vec![]);
        e.call(0x008c_ac60, &[this]);
        assert_eq!(calls_to(&e, OTHER_SLOTS + 0x6a8), vec![vec![process, this]]);
        assert!(calls_to(&e, 0x0092_ed60).is_empty());
    }

    /// The arrow with a process (level 0) and the callees `fn_008caea0` makes.
    fn controller_setup(e: &mut Engine) -> (u32, u32) {
        map_data(e);
        let this = arrow(e);
        let process = object(e, 0x30, OTHER_SLOTS);
        e.mem.set_u32(this + PROCESS, process);
        stub_all(
            e,
            &[
                PROCESS_LEVEL,
                0x00c6_da50,
                0x0081_0550,
                0x0066_ef20,
                0x0040_7860,
                0x008a_c890,
                0x0081_0510,
                0x0085_7230,
                0x0081_05c0,
                0x0081_0530,
                0x0081_0590,
                0x0081_0570,
                0x0081_06b0,
                0x0081_0610,
                0x0081_05e0,
                OTHER_SLOTS + 0x290,
                0x0081_0640,
                0x00c6_de10,
                0x0057_b1d0,
                0x00c7_0c90,
                0x00c6_d7a0,
                SET_MOTION,
                REMOVE_OBJECTS,
                0x0056_20e0,
                0x0070_c440,
                0x004b_ae00,
            ],
        );
        stub(e, PARENT_CELL, 0xce11);
        stub(e, 0x0045_43c0, 0);
        stub(e, ARROW_SLOTS + 0x1d0, 0x3d00);
        let position = point(e, 1.0, 2.0, 3.0);
        stub(e, ARROW_SLOTS + 0x1f4, position);
        stub(e, 0x00aa_13e0, 0x5a00);
        stub(e, 0x0056_e950, 0x5a01);
        stub(e, 0x0093_1ed0, 0x6100);
        stub(e, FILTER_HIGH_HALF, 0x0007);
        stub(e, 0x0055_3300, 0x6600);
        stub(e, 0x008d_10e0, 0x6601);
        stub(e, 0x0052_5420, 0x0301);
        e.register(0x0063_3c90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(0x0064_6660, |_, a| {
            ret_f(f64::from(f32::from_bits(a[0])) + f64::from(f32::from_bits(a[1])))
        });
        stub(e, GET_CHAR_CONTROLLER, 0);
        stub(e, 0x0062_1ad0, 0);
        stub(e, 0x0081_0660, 0x3d00);
        (this, process)
    }

    #[test]
    fn fn_008caea0_builds_a_controller_and_hands_it_over() {
        let mut e = Engine::new();
        let (this, process) = controller_setup(&mut e);
        let shooter = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        e.call(0x008c_aea0, &args![this, shooter, 1.5f32]);
        let info = calls_to(&e, 0x00c6_da50)[0][0];
        assert_eq!(calls_to(&e, 0x0066_ef20), vec![vec![shooter + 0xa4, 0xb]]);
        assert_eq!(
            calls_to(&e, 0x0040_7860),
            vec![vec![info, 51.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, 0x0056_e950),
            vec![vec![0x5a00, 0.1f32.to_bits(), 1]]
        );
        assert_eq!(calls_to(&e, 0x0081_0590), vec![vec![info, 6, 7]]);
        assert_eq!(calls_to(&e, 0x008d_10e0), vec![vec![0x6600, info, this]]);
        assert_eq!(calls_to(&e, 0x0081_0610), vec![vec![0x6601, 1]]);
        assert_eq!(
            calls_to(&e, OTHER_SLOTS + 0x290),
            vec![vec![process, 0x6601]]
        );
        // No world: the lowest float as water height, nothing added.
        assert_eq!(
            calls_to(&e, 0x0057_b1d0),
            vec![vec![0x6601, f32::MIN.to_bits()]]
        );
        assert!(calls_to(&e, 0x0056_20e0).is_empty());
    }

    #[test]
    fn fn_008caea0_places_an_existing_controller_in_a_new_world() {
        let mut e = Engine::new();
        let (this, _) = controller_setup(&mut e);
        let world = object(&mut e, 0x10, TARGET_SLOTS);
        stub(&mut e, GET_CHAR_CONTROLLER, 0x6601);
        stub(&mut e, 0x0045_43c0, world);
        e.register(0x0054_71e0, |_, _| ret_f(-7.0));
        stub(&mut e, 0x0062_1ad0, 0x0ad0);
        stub(&mut e, 0x0081_0660, 0x1234);
        stub(&mut e, 0x0070_c440, 0);
        e.register(FILTER_HIGH_HALF, |e, a| ret(e.mem.u32(a[0]) >> 16));
        e.register(0x0070_c440, |e, a| {
            e.mem.set_u32(a[1], 0x0009_0000);
            Ret::default()
        });
        stub_all(&mut e, &[TARGET_SLOTS + 0xd0]);
        e.call_log = Some(vec![]);
        e.call(0x008c_aea0, &args![this, 0u32, 0.0f32]);
        assert!(calls_to(&e, 0x00c6_da50).is_empty());
        assert_eq!(
            calls_to(&e, 0x0057_b1d0),
            vec![vec![0x6601, (-7.0f32).to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x00c6_d7a0), vec![vec![0x6601, 0x3d00]]);
        assert_eq!(calls_to(&e, SET_MOTION), vec![vec![0x3d00, 4, 1, 0, 1]]);
        assert_eq!(calls_to(&e, REMOVE_OBJECTS), vec![vec![0x3d00, 1, 0]]);
        let position = calls_to(&e, 0x0056_20e0)[0][1];
        assert_eq!(e.mem.f32(position + 8), 11.0);
        assert_eq!(
            calls_to(&e, TARGET_SLOTS + 0xd0),
            vec![vec![world, 0x3d00, 1, 0, 9, 0]]
        );
    }

    #[test]
    fn fn_008caea0_does_nothing_off_level_0() {
        let mut e = Engine::new();
        let (this, _) = controller_setup(&mut e);
        stub(&mut e, PROCESS_LEVEL, 1);
        e.call_log = Some(vec![]);
        e.call(0x008c_aea0, &args![this, 0u32, 0.0f32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn fn_008cb380_passes_the_frame_time() {
        let mut e = Engine::new();
        let this = arrow(&mut e);
        e.register(0x0084_d030, |_, _| ret_f(0.25));
        stub_all(&mut e, &[ARROW_SLOTS + 0x2f8]);
        e.call_log = Some(vec![]);
        e.call(0x008c_b380, &[this, 0x1234]);
        assert_eq!(calls_to(&e, 0x0084_d030), vec![vec![FRAME_TIMER]]);
        assert_eq!(
            calls_to(&e, ARROW_SLOTS + 0x2f8),
            vec![vec![this, 0.25f32.to_bits()]]
        );
    }

    /// An arrow for the update, with the age limit setting and the node
    /// update doubled.
    fn update_setup(e: &mut Engine) -> u32 {
        map_data(e);
        let this = arrow(e);
        let limit = e.mem.alloc(4);
        e.mem.set_f32(limit, 10.0);
        e.register_double(SETTING_VALUE, move |_, _| ret(limit));
        stub(e, ARROW_SLOTS + 0x1d0, 0x3d00);
        stub_all(
            e,
            &[
                0x0044_0d80,
                UPDATE_DATA,
                NODE_UPDATE,
                ARROW_SLOTS + 0xc4,
                0x00b6_bb30,
            ],
        );
        stub(e, GET_CHAR_CONTROLLER, 0);
        this
    }

    #[test]
    fn fn_008cb3c0_fades_a_deleting_arrow() {
        let mut e = Engine::new();
        let this = update_setup(&mut e);
        e.mem.set_u32(this + STATE, 3);
        e.mem.set_f32(this + FADE_ALPHA, 0.5);
        e.mem.set_f32(this + AGE, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x008c_b3c0, &args![this, 0.75f32]);
        assert_eq!(e.mem.f32(this + AGE), 1.75);
        assert_eq!(e.mem.f32(this + FADE_ALPHA), 0.25);
        assert!(calls_to(&e, ARROW_SLOTS + 0xc4).is_empty());
        assert_eq!(
            calls_to(&e, 0x00b6_bb30),
            vec![vec![0x3d00, 0.25f32.to_bits()]]
        );
        let update = calls_to(&e, UPDATE_DATA)[0][0];
        assert_eq!(
            calls_to(&e, UPDATE_DATA),
            vec![vec![update, 0.75f32.to_bits(), 1, 0]]
        );
        assert_eq!(calls_to(&e, NODE_UPDATE), vec![vec![0x3d00, update]]);
        e.call(0x008c_b3c0, &args![this, 0.75f32]);
        assert_eq!(e.mem.f32(this + FADE_ALPHA), 0.0);
        assert_eq!(calls_to(&e, ARROW_SLOTS + 0xc4), vec![vec![this, 1]]);
    }

    #[test]
    fn fn_008cb3c0_ages_out_and_stops_on_the_reference_test() {
        let mut e = Engine::new();
        let this = update_setup(&mut e);
        e.mem.set_f32(this + AGE, 9.5);
        stub(&mut e, 0x0044_0d80, 1);
        e.call_log = Some(vec![]);
        e.call(0x008c_b3c0, &args![this, 1.0f32]);
        assert_eq!(e.mem.u32(this + STATE), 3);
        assert!(calls_to(&e, NODE_UPDATE).is_empty());
        // Below zero only the node update runs.
        e.call(0x008c_b3c0, &args![this, -1.0f32]);
        assert_eq!(e.mem.f32(this + AGE), 10.5);
        assert_eq!(calls_to(&e, NODE_UPDATE).len(), 1);
    }

    #[test]
    fn fn_008cb3c0_slows_a_flying_arrow_under_water() {
        let mut e = Engine::new();
        let this = update_setup(&mut e);
        stub(&mut e, GET_CHAR_CONTROLLER, 0x6601);
        let position = point(&mut e, 1.0, 2.0, 3.0);
        stub(&mut e, ARROW_SLOTS + 0x1f4, position);
        e.register(0x0081_0a80, |_, _| ret_f(5.0));
        e.register(0x0045_87d0, |_, a| ret_f(f64::from(f32::from_bits(a[0]))));
        stub_all(
            &mut e,
            &[
                0x0041_6870,
                0x0081_06b0,
                ARROW_SLOTS + 0x250,
                NODE_SET_TRANSLATE,
                0x00a5_9400,
                0x00a5_9660,
                NODE_SET_ROTATE,
                0x004b_54c0,
            ],
        );
        let rotation = e.mem.alloc(36);
        stub(&mut e, 0x006a_9540, rotation);
        let difference = point(&mut e, 0.0, 3.0, 4.0);
        stub(&mut e, POINT_SUBTRACT, difference);
        e.register(POINT_LENGTH, |_, _| ret_f(5.0));
        e.mem.set_f32(this + SPEED, 100.0);
        e.mem.set_f32(this + DAMAGE, 8.0);
        e.mem.set_f32(this + WATER_SPEED, 4.0);
        e.call_log = Some(vec![]);
        e.call(0x008c_b3c0, &args![this, 1.0f32]);
        // (4 - 1) / 4 = 0.75 scales speed and damage; 3 is left.
        assert_eq!(e.mem.f32(this + SPEED), 75.0);
        assert_eq!(e.mem.f32(this + DAMAGE), 6.0);
        assert_eq!(e.mem.f32(this + WATER_SPEED), 3.0);
        assert_eq!(
            calls_to(&e, 0x0081_06b0),
            vec![vec![0x6601, 0.75f32.to_bits()]]
        );
        let velocity = calls_to(&e, 0x0041_6870)[0].clone();
        assert_eq!(velocity[1..], [0, 100.0f32.to_bits(), 0]);
        assert_eq!(
            calls_to(&e, ARROW_SLOTS + 0x250),
            vec![vec![this, 1.0f32.to_bits(), velocity[0], 0xf]]
        );
        assert_eq!(e.mem.bytes(this + DELTA, 12), e.mem.bytes(difference, 12));
        assert_eq!(calls_to(&e, 0x004b_54c0), vec![vec![0.8f32.to_bits()]]);
        assert_eq!(calls_to(&e, NODE_SET_ROTATE).len(), 1);
        assert_eq!(calls_to(&e, NODE_UPDATE).len(), 1);
    }

    /// Doubles for the record builders' controller step (no controller).
    fn record_setup(e: &mut Engine) -> u32 {
        map_data(e);
        new_allocates(e);
        stub_all(e, &[CONSTRUCT_IN_PLACE]);
        stub(e, GET_CHAR_CONTROLLER, 0);
        arrow(e)
    }

    #[test]
    fn take_controller_velocity_uses_the_controller_or_the_axis() {
        let mut e = Engine::new();
        let this = record_setup(&mut e);
        let record = e.mem.alloc(COLLISION_SIZE);
        e.mem.set_u32(this + COLLISION, record);
        stub(&mut e, GET_CHAR_CONTROLLER, 0x6601);
        stub(&mut e, LINEAR_VELOCITY, 0x7e10);
        stub_all(&mut e, &[VECTOR_LOAD, VECTOR_STORE]);
        stub_returning_arg(&mut e, VECTOR_PREPARE, 1);
        e.register(VECTOR_LENGTH_OF, |_, _| ret_f(2.0));
        e.call_log = Some(vec![]);
        take_controller_velocity(&mut e, Ptr::new(this));
        let vector = calls_to(&e, VECTOR_LOAD)[0][0];
        assert_eq!(calls_to(&e, VECTOR_LOAD), vec![vec![vector, 0x7e10]]);
        assert_eq!(
            calls_to(&e, VECTOR_STORE),
            vec![vec![record + COLLISION_VELOCITY, vector]]
        );
        e.register(VECTOR_LENGTH_OF, |_, _| ret_f(-f64::from(f32::MAX)));
        stub(&mut e, ARROW_SLOTS + 0x1d0, 0x3d00);
        stub(&mut e, NODE_ROTATION, 0x7070);
        stub_returning_arg(&mut e, POINT_SCALE, 1);
        let turned = point(&mut e, 4.0, 5.0, 6.0);
        stub(&mut e, MATRIX_TIMES_POINT, turned);
        e.mem.set_f32(this + SPEED, 9.0);
        e.call_log = Some(vec![]);
        take_controller_velocity(&mut e, Ptr::new(this));
        assert!(calls_to(&e, VECTOR_STORE).is_empty());
        let scaled = calls_to(&e, POINT_SCALE)[0][1];
        assert_eq!(
            calls_to(&e, POINT_SCALE),
            vec![vec![FORWARD_AXIS, scaled, 9.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, MATRIX_TIMES_POINT)[0][2], scaled);
        assert_eq!(
            e.mem.bytes(record + COLLISION_VELOCITY, 12),
            e.mem.bytes(turned, 12)
        );
    }

    #[test]
    fn fn_008cc6f0_constructs_the_four_members() {
        let mut e = Engine::new();
        stub_all(&mut e, &[CONSTRUCT_IN_PLACE]);
        let record = e.mem.alloc(COLLISION_SIZE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008c_c6f0, &[record]).u32(), record);
        assert_eq!(
            calls_to(&e, CONSTRUCT_IN_PLACE),
            vec![
                vec![record + 4],
                vec![record + 0x10],
                vec![record + 0x1c],
                vec![record + 0x30]
            ]
        );
    }

    #[test]
    fn fn_008ccfb0_reuses_or_builds_a_falling_record() {
        let mut e = Engine::new();
        let this = record_setup(&mut e);
        e.call(0x008c_cfb0, &[this, 0x7a00]);
        let record = e.mem.u32(this + COLLISION);
        assert_eq!(e.mem.u32(record + COLLISION_TYPE), 4);
        assert_eq!(e.mem.u32(record + COLLISION_COLLIDEE), 0x7a00);
        assert_eq!(e.mem.f32(record + COLLISION_ROTATION), 1.0);
        assert_eq!(e.mem.u32(this + STATE), 1);
        e.mem.set_u32(record + COLLISION_TYPE, 2);
        e.mem.set_u32(this + STATE, 0);
        e.call(0x008c_cfb0, &[this, 0x7b00]);
        assert_eq!(e.mem.u32(this + COLLISION), record);
        assert_eq!(e.mem.u32(record + COLLISION_TYPE), 4);
        assert_eq!(e.mem.u32(record + COLLISION_COLLIDEE), 0x7a00);
        assert_eq!(e.mem.u32(this + STATE), 1);
    }

    #[test]
    fn fn_008ccce0_builds_or_updates_a_bounce() {
        let mut e = Engine::new();
        let this = record_setup(&mut e);
        let at = point(&mut e, 1.0, 2.0, 3.0);
        let normal = point(&mut e, 0.0, 0.0, 1.0);
        e.call(0x008c_cce0, &[this, at, normal]);
        let record = e.mem.u32(this + COLLISION);
        assert_eq!(e.mem.u32(record + COLLISION_TYPE), 3);
        assert_eq!(
            e.mem.bytes(record + COLLISION_POINT, 12),
            e.mem.bytes(at, 12)
        );
        assert_eq!(
            e.mem.bytes(record + COLLISION_NORMAL, 12),
            e.mem.bytes(normal, 12)
        );
        assert_eq!(e.mem.u32(this + STATE), 1);
        // An existing record: only the normal passes the test here.
        e.mem.set_u32(this + STATE, 2);
        e.mem.set_u32(record + COLLISION_COLLIDEE, 0x7a00);
        let second = point(&mut e, 7.0, 8.0, 9.0);
        let normal_2 = point(&mut e, 1.0, 0.0, 0.0);
        e.register_double(0x0043_9090, move |_, a| ret(u32::from(a[0] == normal_2)));
        e.call_log = Some(vec![]);
        e.call(0x008c_cce0, &[this, second, normal_2]);
        assert_eq!(
            calls_to(&e, 0x0043_9090),
            vec![vec![normal_2, ZERO_POINT], vec![second, ZERO_POINT]]
        );
        assert_eq!(
            e.mem.bytes(record + COLLISION_NORMAL, 12),
            e.mem.bytes(normal_2, 12)
        );
        assert_eq!(
            e.mem.bytes(record + COLLISION_POINT, 12),
            e.mem.bytes(at, 12)
        );
        assert_eq!(e.mem.u32(record + COLLISION_COLLIDEE), 0);
        assert_eq!(e.mem.u32(this + STATE), 2);
    }

    #[test]
    fn fn_008cc740_records_a_ground_hit_and_casts() {
        let mut e = Engine::new();
        let this = record_setup(&mut e);
        e.mem.set_u32(TES, 0x7e5);
        let at = point(&mut e, 1.0, 2.0, 3.0);
        let normal = point(&mut e, 0.0, 0.0, 1.0);
        e.mem.set_f32(this + DELTA, 6.0);
        stub_all(
            &mut e,
            &[
                NORMALIZE,
                0x0043_9180,
                0x008c_fad0,
                0x0081_5870,
                0x0043_9090,
            ],
        );
        let moved = point(&mut e, 9.0, 9.0, 9.0);
        stub(&mut e, POINT_ADD, moved);
        stub(&mut e, 0x0045_7880, 3);
        e.register(POINT_LENGTH, |_, _| ret_f(12.0));
        let shooter = object(&mut e, 0x100, TARGET_SLOTS);
        e.mem.set_u32(this + SHOOTER, shooter);
        let caster = shooter + 0x88;
        let caster_table = e.mem.alloc(0x100);
        for slot in (0..0x100).step_by(4) {
            e.mem.set_u32(caster_table + slot, OTHER_SLOTS + slot);
        }
        e.mem.set_u32(caster, caster_table);
        stub(&mut e, TARGET_SLOTS + 0x428, 0);
        stub_all(&mut e, &[OTHER_SLOTS + 0x40, OTHER_SLOTS + 0x48]);
        let enchantment = 0x00e0_0000;
        e.mem.set_u32(this + 0xa8, enchantment);
        stub(&mut e, 0x009e_e040, 0);
        stub(&mut e, 0x004f_9bf0, 0);
        stub(&mut e, 0x007a_f430, 0x1a00);
        e.call_log = Some(vec![]);
        e.call(0x008c_c740, &[this, at, normal]);
        let record = e.mem.u32(this + COLLISION);
        let direction = calls_to(&e, NORMALIZE)[0][0];
        assert_eq!(
            calls_to(&e, 0x0043_9180),
            vec![vec![direction, 24.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, POINT_ADD)[0][0], at);
        // Kind 3 also takes the bounce path on the record just built.
        assert_eq!(e.mem.u32(record + COLLISION_TYPE), 3);
        assert_eq!(
            e.mem.bytes(record + COLLISION_POINT, 12),
            e.mem.bytes(moved, 12)
        );
        assert_eq!(
            calls_to(&e, 0x008c_fad0),
            vec![vec![
                this,
                12.0f32.to_bits(),
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                0,
                3
            ]]
        );
        assert_eq!(
            calls_to(&e, OTHER_SLOTS + 0x40),
            vec![vec![caster, enchantment + 0x18]]
        );
        assert_eq!(calls_to(&e, OTHER_SLOTS + 0x48), vec![vec![caster, 0]]);
        assert_eq!(calls_to(&e, 0x0081_5870), vec![vec![caster, 0x1a00]]);
        // The enchantment makes the arrow drop (`fn_008cc6d0(3)` sets the state).
        assert_eq!(e.mem.u32(this + STATE), 3);
    }

    /// The arrow with a 3D, a collision object and its body for
    /// `fn_008cd210` and the handlers.
    fn handler_setup(e: &mut Engine, kind: u32) -> (u32, u32) {
        map_data(e);
        let this = arrow(e);
        let record = e.mem.alloc(COLLISION_SIZE);
        e.mem.set_u32(this + COLLISION, record);
        e.mem.set_u32(record + COLLISION_TYPE, kind);
        stub(e, ARROW_SLOTS + 0x1d0, 0x3d00);
        stub(e, FIND_FIRST_COLLISION_OBJECT, 0x0b00);
        let body = object(e, 0x10, OTHER_SLOTS);
        stub(e, COLLISION_OBJECT_BODY, body);
        stub_all(
            e,
            &[
                ARROW_SLOTS + 0x2a0,
                PRINT_MESSAGE,
                OPERATOR_DELETE,
                NODE_SET_TRANSLATE,
                SET_LOCATION_ON_REFERENCE,
                FILTER_SET_LOW_BITS,
                0x0081_0f90,
                BODY_SET_FILTER,
                SET_MOTION,
            ],
        );
        stub_returning_arg(e, BODY_GET_FILTER, 1);
        stub_returning_arg(e, MAKE_FILTER, 0);
        stub(e, 0x005d_be10, 0x0042);
        (this, body)
    }

    #[test]
    fn fn_008cd210_needs_a_record_and_a_collision_object() {
        let mut e = Engine::new();
        let (this, _) = handler_setup(&mut e, 2);
        let record = e.mem.u32(this + COLLISION);
        stub(&mut e, FIND_FIRST_COLLISION_OBJECT, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x008c_d210, &[this]).bool());
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![MESSAGE_NO_COLLISION_DATA]]
        );
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![record]]);
        assert_eq!(e.mem.u32(this + COLLISION), 0);
        assert!(!e.call(0x008c_d210, &[this]).bool());
    }

    #[test]
    fn fn_008cd210_runs_the_ground_handler() {
        let mut e = Engine::new();
        let (this, body) = handler_setup(&mut e, 2);
        e.mem.set_u32(this + STATE, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x008c_d210, &[this]).bool());
        let record = e.mem.u32(this + COLLISION);
        assert_ne!(record, 0);
        assert_eq!(e.mem.u32(this + STATE), 1);
        assert_eq!(
            calls_to(&e, NODE_SET_TRANSLATE),
            vec![vec![0x3d00, record + COLLISION_POINT]]
        );
        assert_eq!(
            calls_to(&e, SET_LOCATION_ON_REFERENCE),
            vec![vec![this, record + COLLISION_POINT]]
        );
        let filter = calls_to(&e, BODY_GET_FILTER)[0][1];
        assert_eq!(calls_to(&e, FILTER_SET_LOW_BITS), vec![vec![filter, 0xf]]);
        assert_eq!(calls_to(&e, 0x0081_0f90), vec![vec![filter, 0]]);
        let fresh = calls_to(&e, MAKE_FILTER)[0][0];
        assert_eq!(calls_to(&e, MAKE_FILTER), vec![vec![fresh, 0xf, 0x42, 0]]);
        assert_eq!(calls_to(&e, BODY_SET_FILTER).len(), 2);
        assert_eq!(calls_to(&e, BODY_SET_FILTER)[0][0], body);
    }

    #[test]
    fn fn_008cd210_drops_the_record_when_the_handler_fails() {
        let mut e = Engine::new();
        let (this, _) = handler_setup(&mut e, 0);
        let record = e.mem.u32(this + COLLISION);
        e.mem.set_u32(this + STATE, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x008c_d210, &[this]).bool());
        assert_eq!(calls_to(&e, PRINT_MESSAGE), vec![vec![MESSAGE_NO_ACTOR]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![record]]);
        assert_eq!(e.mem.u32(this + COLLISION), 0);
        assert_eq!(e.mem.u32(this + STATE), 0);
        // A type past 4 also fails.
        let (this, _) = handler_setup(&mut e, 7);
        assert!(!e.call(0x008c_d210, &[this]).bool());
    }

    #[test]
    fn fn_008cd390_attaches_the_arrow_to_the_bone() {
        let mut e = Engine::new();
        let (this, _) = handler_setup(&mut e, 0);
        let record = e.mem.u32(this + COLLISION);
        let actor = object(&mut e, 0x100, TARGET_SLOTS);
        let bone = object(&mut e, 0x10, 0x7400_0000);
        e.mem.set_u32(record + COLLISION_COLLIDEE, actor);
        stub(&mut e, TARGET_SLOTS + 0x100, 1);
        stub_all(
            &mut e,
            &[
                REMOVE_OBJECTS,
                0x0062_bc10,
                NODE_SET_ROTATE,
                NODE_SET_SCALE,
                0x00b5_b1c0,
                0x7400_00dc,
                0x00b5_eeb0,
                0x7500_06a4,
            ],
        );
        assert!(!e.call(0x008c_d390, &[this, 0]).bool());
        e.mem.set_u32(record + COLLISION_CLOSEST_OBJECT, bone);
        e.register(0x009c_dae0, |_, _| ret_f(3.0));
        e.register(0x008d_01e0, |_, _| ret_f(2.0));
        stub(&mut e, 0x0045_0b80, 0x5500);
        let process = object(&mut e, 0x10, 0x7500_0000);
        stub(&mut e, REFERENCE_PROCESS, process);
        e.call_log = Some(vec![]);
        assert!(e.call(0x008c_d390, &[this, 0]).bool());
        assert_eq!(calls_to(&e, REMOVE_OBJECTS), vec![vec![0x3d00, 1, 0]]);
        assert_eq!(
            calls_to(&e, NODE_SET_SCALE),
            vec![vec![0x3d00, 1.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x0045_0b80), vec![vec![0]]);
        assert_eq!(calls_to(&e, 0x7400_00dc), vec![vec![bone, 0x3d00, 1]]);
        assert_eq!(calls_to(&e, 0x00b5_eeb0), vec![vec![0x5500, 0x3d00]]);
        assert_eq!(calls_to(&e, 0x7500_06a4), vec![vec![process, this]]);
    }

    #[test]
    fn fn_008cd520_sticks_into_a_reference() {
        let mut e = Engine::new();
        let (this, body) = handler_setup(&mut e, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x008c_d520, &[this, body]).bool());
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![MESSAGE_NO_REFERENCE]]
        );
        let record = e.mem.u32(this + COLLISION);
        e.mem.set_u32(record + COLLISION_COLLIDEE, 0x7a00);
        e.mem.set_u32(record + COLLISION_CLOSEST_OBJECT, 0x0c00);
        let hit_body = 0x00b0_d000;
        stub(&mut e, COLLISION_OBJECT_BODY, hit_body);
        stub(&mut e, FILTER_HIGH_HALF, 0x0033);
        stub(&mut e, FILTER_LOW_BITS, 5);
        stub(&mut e, 0x0046_10f0, 0x5150);
        e.register(0x005a_e3d0, |e, a| ret(e.mem.u32(a[1])));
        e.register(0x0062_ab20, |_, _| ret_f(2.0));
        stub_all(
            &mut e,
            &[CONSTRUCT_IN_PLACE, OTHER_SLOTS + 0xf0, 0x0062_a7e0],
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x008c_d520, &[this, body]).bool());
        assert_eq!(
            calls_to(&e, FIND_FIRST_COLLISION_OBJECT),
            vec![vec![0x0c00]]
        );
        assert_eq!(calls_to(&e, SET_MOTION), vec![vec![0x3d00, 4, 1, 0, 1]]);
        let filter = calls_to(&e, MAKE_FILTER)[0][0];
        assert_eq!(calls_to(&e, MAKE_FILTER), vec![vec![filter, 6, 0x33, 0]]);
        assert_eq!(calls_to(&e, FILTER_SET_LOW_BITS), vec![vec![filter, 0xf]]);
        assert_eq!(calls_to(&e, BODY_SET_FILTER)[0][0], body);
        assert_eq!(calls_to(&e, 0x0062_ab20), vec![vec![hit_body]]);
        let velocity = calls_to(&e, OTHER_SLOTS + 0xf0)[0][1];
        assert_eq!(calls_to(&e, 0x0062_a7e0), vec![vec![velocity, 0x0c00]]);
        // No dynamic mass: no velocity handed on.
        e.register(0x0062_ab20, |_, _| ret_f(0.0));
        stub(&mut e, FILTER_LOW_BITS, 4);
        e.call_log = Some(vec![]);
        assert!(e.call(0x008c_d520, &[this, body]).bool());
        assert!(calls_to(&e, FILTER_SET_LOW_BITS).is_empty());
        assert!(calls_to(&e, 0x0062_a7e0).is_empty());
    }

    #[test]
    fn fn_008cd6a0_forwards_both_words() {
        let mut e = Engine::new();
        stub_all(&mut e, &[0x00cb_2d90, 0x004a_3f10]);
        e.call_log = Some(vec![]);
        e.call(0x008c_d6a0, &[0x9000, 1, 2]);
        assert_eq!(calls_to(&e, 0x00cb_2d90), vec![vec![0x9000, 1]]);
        assert_eq!(calls_to(&e, 0x004a_3f10), vec![vec![0x9030, 2]]);
    }

    #[test]
    fn fn_008cd780_sends_the_bounce_off() {
        let mut e = Engine::new();
        let (this, body) = handler_setup(&mut e, 3);
        e.mem.set_u32(SAVE_LOAD_GAME, 0x5a1e);
        e.mem.set_f32(this + SPEED, 10.0);
        stub(&mut e, 0x0047_c850, 0);
        let speed_setting = e.mem.alloc(4);
        e.mem.set_f32(speed_setting, 0.5);
        let spin_setting = e.mem.alloc(4);
        e.mem.set_f32(spin_setting, 2.0);
        e.register_double(SETTING_VALUE, move |_, a| {
            ret(if a[0] == SETTING_BOUNCE_SPEED {
                speed_setting
            } else {
                spin_setting
            })
        });
        stub_all(
            &mut e,
            &[
                UPDATE_DATA,
                NODE_UPDATE,
                NORMALIZE,
                0x0063_c8a0,
                0x0056_15d0,
                CONSTRUCT_IN_PLACE,
                MATRIX_INVERSE,
                0x0041_6870,
                0x0056_1630,
                0x0057_3ed0,
            ],
        );
        stub_returning_arg(&mut e, SCALE_INTO, 0);
        e.register(0x004b_6190, |_, _| ret_f(0.5));
        stub_returning_arg(&mut e, POINT_SUBTRACT, 1);
        stub(&mut e, NODE_ROTATION, 0x7070);
        // The record normal in the arrow's space: (3, 1, -0.5).
        e.register(MATRIX_TIMES_POINT, |e, a| {
            if a[0] != 0x7070 {
                e.mem.set_f32(a[1], 3.0);
                e.mem.set_f32(a[1] + 4, 1.0);
                e.mem.set_f32(a[1] + 8, -0.5);
            }
            ret(a[1])
        });
        e.register(0x0040_8820, |_, a| {
            ret_f(f64::from(f32::from_bits(a[0]).abs()))
        });
        e.register(0x0063_1070, |_, a| {
            ret_f(if f32::from_bits(a[0]) < 0.0 {
                -1.0
            } else {
                1.0
            })
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x008c_d780, &[this, body]).bool());
        let record = e.mem.u32(this + COLLISION);
        assert_eq!(
            calls_to(&e, NODE_SET_TRANSLATE),
            vec![vec![0x3d00, record + COLLISION_POINT]]
        );
        assert_eq!(calls_to(&e, SET_MOTION), vec![vec![0x3d00, 1, 1, 0, 1]]);
        assert_eq!(calls_to(&e, MAKE_FILTER)[0][1..], [4, 0x42, 0]);
        let scales = calls_to(&e, SCALE_INTO);
        assert_eq!(scales[0][1..], [(-1.0f32).to_bits(), this + DELTA]);
        assert_eq!(scales[1][1], 0.5f32.to_bits());
        assert_eq!(scales[2][1], 2.0f32.to_bits());
        assert_eq!(scales[3][1], 5.0f32.to_bits());
        assert_eq!(calls_to(&e, 0x0056_15d0), vec![vec![body, scales[3][0]]]);
        // |z| 0.5 is below |y| 1: z becomes |y| * sign(z) = -1, y 0.
        let spin = calls_to(&e, 0x0041_6870)[0].clone();
        assert_eq!(spin[1..], [(-2.0f32).to_bits(), 0, (-6.0f32).to_bits()]);
        assert_eq!(calls_to(&e, 0x0056_1630)[0][0], body);
        assert_eq!(calls_to(&e, 0x0057_3ed0), vec![vec![0x0b00, 1]]);
        // With the save/load test holding, only the filter is set.
        stub(&mut e, 0x0047_c850, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x008c_d780, &[this, body]).bool());
        assert!(calls_to(&e, NODE_SET_TRANSLATE).is_empty());
        assert!(calls_to(&e, SCALE_INTO).is_empty());
        assert_eq!(calls_to(&e, BODY_SET_FILTER).len(), 1);
    }

    #[test]
    fn fn_008cdaf0_takes_the_actor_controller_group() {
        let mut e = Engine::new();
        let (this, body) = handler_setup(&mut e, 4);
        let record = e.mem.u32(this + COLLISION);
        e.mem.set_u32(record + COLLISION_COLLIDEE, 0x7a00);
        stub(&mut e, RT_DYNAMIC_CAST, 0x7a00);
        stub(&mut e, GET_CHAR_CONTROLLER, 0x6601);
        stub(&mut e, 0x0081_9250, 0x0077);
        stub_all(&mut e, &[0x0056_15d0, 0x0057_3ed0]);
        stub(&mut e, 0x0062_0b80, 0x0b0b);
        stub(&mut e, 0x004b_5a20, 0x0c0c);
        e.call_log = Some(vec![]);
        assert!(e.call(0x008c_daf0, &[this, body]).bool());
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST),
            vec![vec![0x7a00, 0, REFERENCE_TYPE, ACTOR_TYPE, 0]]
        );
        assert_eq!(calls_to(&e, MAKE_FILTER)[0][1..], [4, 0x77, 0]);
        assert_eq!(
            calls_to(&e, 0x0056_15d0),
            vec![vec![body, record + COLLISION_VELOCITY]]
        );
        assert_eq!(calls_to(&e, 0x004b_5a20), vec![vec![0x0b0b]]);
        assert_eq!(calls_to(&e, 0x0057_3ed0), vec![vec![0x0c0c, 1]]);
        stub(&mut e, RT_DYNAMIC_CAST, 0);
        e.call_log = Some(vec![]);
        e.call(0x008c_daf0, &[this, body]);
        assert_eq!(calls_to(&e, MAKE_FILTER)[0][1..], [4, 0x42, 0]);
    }

    #[test]
    fn fn_008cdbf0_moves_to_the_level_its_cell_wants() {
        let mut e = Engine::new();
        map_data(&mut e);
        let this = arrow(&mut e);
        let process = e.mem.alloc(0x30);
        e.mem.set_u32(this + PROCESS, process);
        e.mem.set_u32(TES, 0x7e5);
        stub(&mut e, PROCESS_LEVEL, 1);
        stub(&mut e, PARENT_CELL, 0xce11);
        stub(&mut e, ARROW_SLOTS + 0x1d0, 0x3d00);
        stub_all(
            &mut e,
            &[ARROW_SLOTS + 0x240, ARROW_SLOTS + 0x24c, ARROW_SLOTS + 0x10],
        );
        // Loaded with the flag: level 0 wanted, to high.
        stub(&mut e, 0x0045_11e0, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x008c_dbf0, &[this]).bool());
        assert_eq!(calls_to(&e, 0x0045_11e0), vec![vec![0x7e5, 0xce11, 1]]);
        assert_eq!(calls_to(&e, ARROW_SLOTS + 0x240), vec![vec![this]]);
        // Not loaded at all: level 3, the arrow deletes itself.
        stub(&mut e, 0x0045_11e0, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x008c_dbf0, &[this]).bool());
        assert_eq!(calls_to(&e, ARROW_SLOTS + 0x10), vec![vec![this, 1]]);
        // Level 1 wanted and already there.
        e.register(0x0045_11e0, |_, a| ret(u32::from(a[2] == 0)));
        e.call_log = Some(vec![]);
        assert!(e.call(0x008c_dbf0, &[this]).bool());
        assert!(calls_to(&e, ARROW_SLOTS + 0x24c).is_empty());
        // Level 1 wanted from level 0 while stuck: to middle high.
        stub(&mut e, PROCESS_LEVEL, 0);
        e.mem.set_u32(this + STATE, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x008c_dbf0, &[this]).bool());
        assert_eq!(calls_to(&e, ARROW_SLOTS + 0x24c), vec![vec![this]]);
        // Stuck in the player: to high when not there, true.
        let record = e.mem.alloc(COLLISION_SIZE);
        e.mem.set_u32(this + COLLISION, record);
        e.mem.set_u32(this + STATE, 2);
        e.mem.set_u32(PLAYER, 0x9a7e);
        e.mem.set_u32(record + COLLISION_COLLIDEE, 0x9a7e);
        stub(&mut e, PROCESS_LEVEL, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x008c_dbf0, &[this]).bool());
        assert_eq!(calls_to(&e, ARROW_SLOTS + 0x240), vec![vec![this]]);
        assert!(calls_to(&e, 0x0045_11e0).is_empty());
    }

    #[test]
    fn fn_008cddb0_asks_once() {
        let mut e = Engine::new();
        map_data(&mut e);
        stub(&mut e, 0x005d_be10, 0x0042);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008c_ddb0, &[]).u32(), 0x42);
        assert_eq!(e.call(0x008c_ddb0, &[]).u32(), 0x42);
        assert_eq!(calls_to(&e, 0x005d_be10).len(), 1);
    }

    /// The callees of the actor hit, all doubled; returns the arrow and the
    /// target.
    fn actor_hit_setup(e: &mut Engine) -> (u32, u32) {
        let this = record_setup(e);
        let target = object(e, 0x100, TARGET_SLOTS);
        stub(e, ARROW_SLOTS + 0x1d0, 0x3d00);
        let position = point(e, 1.0, 2.0, 3.0);
        stub(e, ARROW_SLOTS + 0x1f4, position);
        e.register(ARROW_SLOTS + 0x2bc, |_, _| ret_f(0.0));
        e.register(0x0093_1d70, |_, _| ret_f(0.0));
        stub(e, TARGET_SLOTS + 0x1d0, 0x7d00);
        stub_all(
            e,
            &[
                0x008a_5300,
                0x005d_43c0,
                0x005a_c750,
                REFERENCE_3D,
                0x004a_0c90,
                0x0052_4ac0,
                0x0043_9f50,
                NORMALIZE,
                TARGET_SLOTS + 0x22c,
                TARGET_SLOTS + 0x21c,
                TARGET_SLOTS + 0x38c,
                TARGET_SLOTS + 0x190,
                0x0047_6a80,
                0x004b_4880,
                MATRIX_INVERSE,
                0x0089_4d60,
                0x008d_0020,
                0x0043_b610,
                0x0048_7f50,
            ],
        );
        stub(e, 0x007a_f430, 0x1a00);
        let product = e.mem.alloc(36);
        e.mem.set_f32(product + 32, 7.0);
        stub(e, 0x0043_f8d0, product);
        stub_returning_arg(e, POINT_SCALE, 1);
        stub_returning_arg(e, 0x0053_d280, 1);
        stub_returning_arg(e, POINT_SUBTRACT, 1);
        stub_returning_arg(e, POINT_ADD, 1);
        stub_returning_arg(e, MATRIX_TIMES_POINT, 1);
        let local_point = point(e, 4.0, 5.0, 6.0);
        stub(e, 0x0052_4c40, local_point);
        stub(e, NODE_ROTATION, 0x7070);
        stub(e, FIND_FIRST_COLLISION_OBJECT, 0x0b00);
        stub(e, COLLISION_OBJECT_BODY, 0xb0d1);
        let chance = e.mem.alloc(4);
        e.mem.set_u32(chance, 50);
        stub(e, SETTING_INT_VALUE, chance);
        (this, target)
    }

    #[test]
    fn fn_008cba30_without_a_target_only_builds_the_record() {
        let mut e = Engine::new();
        let this = record_setup(&mut e);
        let at = point(&mut e, 1.0, 2.0, 3.0);
        let normal = point(&mut e, 0.0, 0.0, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x008c_ba30, &[this, at, normal, 0]);
        let record = e.mem.u32(this + COLLISION);
        assert_eq!(e.mem.u32(record + COLLISION_TYPE), 0);
        assert_eq!(
            e.mem.bytes(record + COLLISION_POINT, 12),
            e.mem.bytes(at, 12)
        );
        assert_eq!(
            e.mem.bytes(record + COLLISION_NORMAL, 12),
            e.mem.bytes(normal, 12)
        );
        assert_eq!(e.mem.f32(record + COLLISION_ROTATION + 16), 1.0);
        assert_eq!(e.mem.u32(this + STATE), 1);
        assert!(calls_to(&e, ARROW_SLOTS + 0x1d0).is_empty());
    }

    #[test]
    fn fn_008cba30_lodges_in_the_bone_the_ray_finds() {
        let mut e = Engine::new();
        let (this, target) = actor_hit_setup(&mut e);
        stub(&mut e, 0x008c_fbc0, 0xb0e0);
        let at = point(&mut e, 1.0, 2.0, 3.0);
        let normal = point(&mut e, 0.0, 0.0, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x008c_ba30, &[this, at, normal, target]);
        let record = e.mem.u32(this + COLLISION);
        assert_eq!(e.mem.u32(record + COLLISION_COLLIDEE), target);
        assert_eq!(e.mem.u32(record + COLLISION_CLOSEST_OBJECT), 0xb0e0);
        assert_eq!(e.mem.f32(record + COLLISION_POINT + 8), 6.0);
        assert_eq!(e.mem.f32(record + COLLISION_ROTATION + 32), 7.0);
        assert_eq!(calls_to(&e, 0x005a_c750), vec![vec![0x1a00, 0, 0x100]]);
        let ray = calls_to(&e, 0x008c_fbc0)[0].clone();
        assert_eq!(ray.len(), 9);
        assert_eq!(ray[0], 0x7d00);
        assert_eq!(e.mem.f32(ray[7]), 512.0);
        assert_eq!(ray[8], 1);
        let deeper = calls_to(&e, POINT_SCALE);
        assert_eq!(deeper[1][2], 530.0f32.to_bits());
        // The body found takes the hit with the point and forward axis.
        let hit = calls_to(&e, 0x008d_0020)[0].clone();
        assert_eq!(hit[0], this);
        assert_eq!(
            hit[1..4],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
        assert_eq!(hit[7], 0xb0d1);
        // `00487f50` gave 0, below the chance of 50: the item goes over.
        assert_eq!(
            calls_to(&e, TARGET_SLOTS + 0x190),
            vec![vec![target, 0x1a00, 0, 1]]
        );
        assert_eq!(e.mem.u8(this + ADDED_TO_INVENTORY), 1);
    }

    #[test]
    fn fn_008cba30_falls_when_no_bone_is_found() {
        let mut e = Engine::new();
        let (this, target) = actor_hit_setup(&mut e);
        stub(&mut e, 0x008c_fbc0, 0);
        stub_all(
            &mut e,
            &[0x0062_40d0, 0x004a_3e00, 0x008a_50f0, 0x008a_50d0],
        );
        stub(&mut e, 0x00c8_06b0, 0);
        let at = point(&mut e, 1.0, 2.0, 3.0);
        let normal = point(&mut e, 0.0, 0.0, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x008c_ba30, &[this, at, normal, target]);
        let pick = calls_to(&e, 0x0062_40d0)[0][0];
        assert_eq!(
            calls_to(&e, 0x008a_50d0),
            vec![vec![pick, 200.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x00c8_06b0), vec![vec![0, pick, 1]]);
        let record = e.mem.u32(this + COLLISION);
        assert_eq!(e.mem.u32(record + COLLISION_TYPE), 4);
        assert!(calls_to(&e, 0x008d_0020).is_empty());
        assert!(calls_to(&e, TARGET_SLOTS + 0x190).is_empty());
    }
}
