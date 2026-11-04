//! `fallout/magic/magicmodelhiteffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `MagicModelHitEffect` (Xbox PDB), 0x3c bytes on both builds (the scalar deleting
    /// destructor frees 0x3c): a `MagicHitEffect` (0x28) and its model.
    pub struct MagicModelHitEffect: 0x3c {
        /// `MagicHitEffect::pTarget` (Xbox PDB): `TESObjectREFR*`.
        0x1C pTarget: u32,
        /// `MagicHitEffect::fAge` (Xbox PDB).
        0x20 fAge: u32,
        /// `MagicHitEffect::bFinished` (Xbox PDB).
        0x24 bFinished: u8,
        /// `bOwnPath` (Xbox PDB).
        0x28 bOwnPath: u8,
        /// `b3rdPerson` (Xbox PDB).
        0x29 b3rdPerson: u8,
        /// `pFilename` (Xbox PDB): `const char*`.
        0x2C pFilename: u32,
        /// `spObject` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x30 spObject: u32,
        /// `spTarget` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x34 spTarget: u32,
        /// `LoadedDataSubBuffer` (Xbox PDB): a `BGSLoadGameSubBuffer`.
        0x38 LoadedDataSubBuffer: u32,
    }
}

/// `MagicModelHitEffect`'s vtable (`0107b24c`).
const MAGIC_MODEL_HIT_EFFECT_VTABLE: u32 = 0x0107_b24c;
/// `MagicHitEffect::MagicHitEffect` (`0081b700`, `thiscall`, two words).
const MAGIC_HIT_EFFECT_CONSTRUCTOR: u32 = 0x0081_b700;
/// `MagicHitEffect::~MagicHitEffect` (`0081b7b0`, `thiscall`).
const MAGIC_HIT_EFFECT_DESTRUCTOR: u32 = 0x0081_b7b0;
/// The `NiPointer` constructor `00633c90` (`thiscall`, one word: the object).
const POINTER_CONSTRUCTOR: u32 = 0x0063_3c90;
/// The `NiPointer` assignment `0066b0d0` (`thiscall`, one word: the object).
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The `NiPointer` destructor `0045cec0` (`thiscall`).
const POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
/// `00559450` (`thiscall`): the object a `NiPointer` holds.
const POINTER_GET: u32 = 0x0055_9450;
/// The sub-buffer constructor `0044dee0` and destructor `00435b20` (`thiscall`).
const SUB_BUFFER_CONSTRUCTOR: u32 = 0x0044_dee0;
const SUB_BUFFER_DESTRUCTOR: u32 = 0x0043_5b20;
/// `0044ddc0` (`thiscall`): the word at +8 (the active effect's `pSpell`).
const EFFECT_ITEM_0044DDC0: u32 = 0x0044_ddc0;
/// `MagicItem::GetAVEffect` (engine map, `0040a300`, `thiscall`, one word).
const GET_AV_EFFECT: u32 = 0x0040_a300;
/// `009611e0` (`thiscall`): the word at +0x18 (the hit effect's `pEffect`, an `NiAVObject`'s
/// parent).
const WORD_18_009611E0: u32 = 0x0096_11e0;
/// The model loader (`ModelLoader *`, `011c3b3c`).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// `ModelLoader::ReleaseModel` (engine map, `00445300`, `thiscall`, three words).
const RELEASE_MODEL: u32 = 0x0044_5300;
/// `ModelLoader::LoadFile` (engine map, `00447080`, `thiscall`, six words).
const LOAD_FILE: u32 = 0x0044_7080;
/// The free `00401030` (`cdecl`, one word).
const FREE: u32 = 0x0040_1030;
/// `006b9130` (`thiscall`): the float at +4 in ST0 (the active effect's `fElapsedSeconds`).
const START_AGE_006B9130: u32 = 0x006b_9130;
/// `007df1d0` (`thiscall`) and `00403e60` (`thiscall`, one word: a flag mask).
const OBJECT_007DF1D0: u32 = 0x007d_f1d0;
const HAS_FLAGS_00403E60: u32 = 0x0040_3e60;
/// The player (`PlayerCharacter *`, `011dea3c`) and `004eaf60` on it (a byte).
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_FLAG_004EAF60: u32 = 0x004e_af60;
/// `0043b230` (`thiscall`) and `00653270` (`cdecl`, two words) on `011f36ac`: the
/// controller manager of a model.
const MODEL_ROOT_0043B230: u32 = 0x0043_b230;
const CONTROLLER_MANAGER_00653270: u32 = 0x0065_3270;
const LOOKUP_ARGUMENT_011F36AC: u32 = 0x011f_36ac;
/// The string constructor `00438170` (`thiscall`, one word) and destructor `004381b0`.
const STRING_CONSTRUCTOR: u32 = 0x0043_8170;
const STRING_DESTRUCTOR: u32 = 0x0043_81b0;
/// `"SpecialIdle_HitEffect"`.
const HIT_EFFECT_IDLE: u32 = 0x0103_bdb8;
/// `0047a520` (`thiscall`, one word: the name): the controller sequence of that name.
const GET_SEQUENCE_0047A520: u32 = 0x0047_a520;
/// `MagicProjectile::Clone3D_ov2` (engine map, `0081e460`, `cdecl`, one word).
const CLONE_3D: u32 = 0x0081_e460;
/// `BSUtilities::ProcessSelectiveUpdate` (engine map, `00c4b500`, `cdecl`, one word).
const PROCESS_SELECTIVE_UPDATE: u32 = 0x00c4_b500;
/// `00825c00` (`thiscall`, no words) on `011f6394`: its word +0x14, divided by 1000.0 as a time.
const TIME_00825C00: u32 = 0x0082_5c00;
const TIMER_011F6394: u32 = 0x011f_6394;
/// `1000.0` (`double`, `01017b70`) and `0.0` (`double`, `01012060`).
const THOUSAND: u32 = 0x0101_7b70;
const ZERO: u32 = 0x0101_2060;
/// `0043d410` (`thiscall`, three words: a `float`, two words): fills an update block.
const UPDATE_BLOCK_0043D410: u32 = 0x0043_d410;
/// `00a59c60` (`thiscall`, one word: the update block).
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
/// `NiControllerManager::DeactivateAll` (engine map, `0048fef0`, `thiscall`, one `float`).
const DEACTIVATE_ALL: u32 = 0x0048_fef0;
/// `0047aab0` (`thiscall`, six words) and `0047aa40` (`thiscall`, one word).
const ACTIVATE_SEQUENCE_0047AAB0: u32 = 0x0047_aab0;
const SET_FLAG_0047AA40: u32 = 0x0047_aa40;
/// `NiControllerSequence::SetPhase` (engine map, `00a328b0`, `thiscall`, two words).
const SET_PHASE: u32 = 0x00a3_28b0;

// Translated from 0081c630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::MagicModelHitEffect` (engine map; `thiscall`, `RET 8`: the target
/// reference and the active effect): constructs the `MagicHitEffect` base, stores the
/// vtable, constructs `spObject` and `spTarget` (null) and the sub-buffer, assigns null to
/// both pointers, clears `b3rdPerson` and `pFilename`, then takes the file name from the
/// effect's magic item (`0044ddc0`, `MagicItem::GetAVEffect(0)`, the object at its +0x18
/// answering through its slot +0x14) and clears `bOwnPath`. Returns `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0081c630 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn magic_model_hit_effect_constructor(
    e: &mut Engine,
    this: Ptr<MagicModelHitEffect>,
    target: u32,
    effect: u32,
) -> Ptr<MagicModelHitEffect> {
    let base = this.addr();
    e.call(MAGIC_HIT_EFFECT_CONSTRUCTOR, &args![this, target, effect]);
    e.mem.set_u32(base, MAGIC_MODEL_HIT_EFFECT_VTABLE);
    e.call(POINTER_CONSTRUCTOR, &args![base + 0x30, 0u32]);
    e.call(POINTER_CONSTRUCTOR, &args![base + 0x34, 0u32]);
    e.call(SUB_BUFFER_CONSTRUCTOR, &args![base + 0x38]);
    e.call(POINTER_ASSIGN, &args![base + 0x30, 0u32]);
    e.call(POINTER_ASSIGN, &args![base + 0x34, 0u32]);
    e.set(this, MagicModelHitEffect::b3rdPerson, 0);
    e.set(this, MagicModelHitEffect::pFilename, 0);
    let item = e.call(EFFECT_ITEM_0044DDC0, &args![effect]).u32();
    let av_effect = e.call(GET_AV_EFFECT, &args![item, 0u32]).u32();
    let object = av_effect.wrapping_add(0x18);
    let name = e.vcall(object, 0x14, &args![]).u32();
    e.set(this, MagicModelHitEffect::pFilename, name);
    e.set(this, MagicModelHitEffect::bOwnPath, 0);
    this
}

// Translated from 0081c810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::~MagicModelHitEffect` (engine map; `thiscall`): stores the vtable;
/// when `spObject` holds an object, detaches it from its parent (`009611e0` on it: the
/// word at +0x18; the parent's slot +0xe8 with the object, when there is a parent) and
/// assigns null to `spObject`; assigns null to `spTarget`; releases the model of a non-empty `pFilename`
/// (`ModelLoader::ReleaseModel(name, 0, 1)`) and frees the name when `bOwnPath` is set;
/// clears `pFilename`; then destroys the sub-buffer, `spTarget`, `spObject` and the
/// `MagicHitEffect` base.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0081c810 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn magic_model_hit_effect_destructor(e: &mut Engine, this: Ptr<MagicModelHitEffect>) {
    let base = this.addr();
    e.mem.set_u32(base, MAGIC_MODEL_HIT_EFFECT_VTABLE);
    if e.call(POINTER_GET, &args![base + 0x30]).u32() != 0 {
        let object = e.call(POINTER_GET, &args![base + 0x30]).u32();
        let parent = e.call(WORD_18_009611E0, &args![object]).u32();
        if parent != 0 {
            let object = e.call(POINTER_GET, &args![base + 0x30]).u32();
            e.vcall(parent, 0xe8, &args![object]);
        }
        e.call(POINTER_ASSIGN, &args![base + 0x30, 0u32]);
    }
    e.call(POINTER_ASSIGN, &args![base + 0x34, 0u32]);
    let name = e.get(this, MagicModelHitEffect::pFilename);
    if name != 0 && e.mem.i8(name) != 0 {
        let loader = e.mem.u32(MODEL_LOADER);
        let name = e.get(this, MagicModelHitEffect::pFilename);
        e.call(RELEASE_MODEL, &args![loader, name, 0u32, 1u32]);
        if e.get(this, MagicModelHitEffect::bOwnPath) != 0 {
            let name = e.get(this, MagicModelHitEffect::pFilename);
            e.call(FREE, &args![name]);
        }
    }
    e.set(this, MagicModelHitEffect::pFilename, 0);
    e.call(SUB_BUFFER_DESTRUCTOR, &args![base + 0x38]);
    e.call(POINTER_DESTRUCTOR, &args![base + 0x34]);
    e.call(POINTER_DESTRUCTOR, &args![base + 0x30]);
    e.call(MAGIC_HIT_EFFECT_DESTRUCTOR, &args![this]);
}

/// The controller sequence named `"SpecialIdle_HitEffect"` of the controller manager
/// `manager` (`0047a520`, given a string the code builds and destroys on the stack).
fn hit_effect_sequence(e: &mut Engine, manager: u32) -> u32 {
    e.with_stack(4, |e, string| {
        let name = e
            .call(STRING_CONSTRUCTOR, &args![string, HIT_EFFECT_IDLE])
            .u32();
        let sequence = e.call(GET_SEQUENCE_0047A520, &args![manager, name]).u32();
        e.call(STRING_DESTRUCTOR, &args![string]);
        sequence
    })
}

// Translated from 0081c950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::Init` (engine map; `thiscall`, plain `RET`; true in `AL` when the
/// hit effect model was attached):
///
/// - When `pEffect` (`009611e0`) is set, `fAge` takes the active effect's elapsed time
///   (`006b9130`) and `bFinished` is set when its `007df1d0` object lacks flag 0x400
///   (`00403e60`).
/// - It needs a target whose slot +0x1d0 answers non-zero, and a non-empty `pFilename`;
///   for the player `b3rdPerson` takes `004eaf60`.
/// - When the model loads (`ModelLoader::LoadFile(name, 0, 1, 0, 0, 0)`) and has a
///   controller manager (`0043b230`, `00653270` with `011f36ac`), that manager must have
///   a `"SpecialIdle_HitEffect"` sequence, or the call answers false.
/// - `spObject` becomes a clone of the model (`MagicProjectile::Clone3D_ov2`) and the model
///   is released (`ModelLoader::ReleaseModel(name, 0, 1)`). With a clone and the target
///   still answering, this effect's slot +0xcc runs, the clone gets a selective update
///   (`BSUtilities::ProcessSelectiveUpdate`) and an update at the time of `00825c00` on
///   `011f6394` divided by 1000.0 (`0043d410` fills a 12-byte update block with the time,
///   1 and 0; `00a59c60`). When the clone has a controller manager with that sequence,
///   the manager deactivates all (0.0), activates it (`0047aab0` with 0, 0, 1.0, 0.0, 0),
///   `0047aa40(1)` runs, and a positive `fAge` (`FCOMP` with `01012060`, not NaN) sets the
///   sequence's phase (`NiControllerSequence::SetPhase(fAge, 0)`): the answer is true.
///
/// Any other outcome answers false. The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0081c950 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn magic_model_hit_effect_init(e: &mut Engine, this: Ptr<MagicModelHitEffect>) -> bool {
    let base = this.addr();
    if e.call(WORD_18_009611E0, &args![this]).u32() != 0 {
        let effect = e.call(WORD_18_009611E0, &args![this]).u32();
        let age = e.call(START_AGE_006B9130, &args![effect]).f64();
        e.set(this, MagicModelHitEffect::fAge, x87_store32(age));
        let effect = e.call(WORD_18_009611E0, &args![this]).u32();
        let object = e.call(OBJECT_007DF1D0, &args![effect]).u32();
        let has = e.call(HAS_FLAGS_00403E60, &args![object, 0x400u32]).u8() != 0;
        e.set(this, MagicModelHitEffect::bFinished, u8::from(!has));
    }
    let target = e.get(this, MagicModelHitEffect::pTarget);
    if target == 0 || e.vcall(target, 0x1d0, &args![]).u32() == 0 {
        return false;
    }
    let player = e.mem.u32(PLAYER);
    if e.get(this, MagicModelHitEffect::pTarget) == player {
        let third_person = e.call(PLAYER_FLAG_004EAF60, &args![player]).u8();
        e.set(this, MagicModelHitEffect::b3rdPerson, third_person);
    }
    let name = e.get(this, MagicModelHitEffect::pFilename);
    if name == 0 || e.mem.i8(name) == 0 {
        return false;
    }
    let loader = e.mem.u32(MODEL_LOADER);
    let model = e
        .call(
            LOAD_FILE,
            &args![loader, name, 0u32, 1u32, 0u32, 0u32, 0u32],
        )
        .u32();
    if model != 0 {
        let root = e.call(MODEL_ROOT_0043B230, &args![model]).u32();
        let manager = e
            .call(
                CONTROLLER_MANAGER_00653270,
                &args![LOOKUP_ARGUMENT_011F36AC, root],
            )
            .u32();
        if manager != 0 && hit_effect_sequence(e, manager) == 0 {
            return false;
        }
    }
    let name = e.get(this, MagicModelHitEffect::pFilename);
    let clone = e.call(CLONE_3D, &args![name]).u32();
    e.call(POINTER_ASSIGN, &args![base + 0x30, clone]);
    let loader = e.mem.u32(MODEL_LOADER);
    let name = e.get(this, MagicModelHitEffect::pFilename);
    e.call(RELEASE_MODEL, &args![loader, name, 0u32, 1u32]);
    if e.call(POINTER_GET, &args![base + 0x30]).u32() == 0 {
        return false;
    }
    let target = e.get(this, MagicModelHitEffect::pTarget);
    if e.vcall(target, 0x1d0, &args![]).u32() == 0 {
        return false;
    }
    e.vcall(base, 0xcc, &args![]);
    let object = e.call(POINTER_GET, &args![base + 0x30]).u32();
    e.call(PROCESS_SELECTIVE_UPDATE, &args![object]);
    let milliseconds = e.call(TIME_00825C00, &args![TIMER_011F6394]).u32();
    let thousand = f64::from_bits(e.mem.u64(THOUSAND));
    let seconds = x87_store32(x87_div(f64::from(milliseconds), thousand));
    e.with_stack(0xc, |e, block| {
        e.call(
            UPDATE_BLOCK_0043D410,
            &args![block, x87_quiet_bits(seconds), 1u32, 0u32],
        );
        let object = e.call(POINTER_GET, &args![base + 0x30]).u32();
        e.call(UPDATE_00A59C60, &args![object, block]);
    });
    let object = e.call(POINTER_GET, &args![base + 0x30]).u32();
    let root = e.call(MODEL_ROOT_0043B230, &args![object]).u32();
    let manager = e
        .call(
            CONTROLLER_MANAGER_00653270,
            &args![LOOKUP_ARGUMENT_011F36AC, root],
        )
        .u32();
    if manager == 0 {
        return false;
    }
    let sequence = hit_effect_sequence(e, manager);
    if sequence == 0 {
        return false;
    }
    e.call(DEACTIVATE_ALL, &args![manager, 0.0f32]);
    e.call(
        ACTIVATE_SEQUENCE_0047AAB0,
        &args![manager, sequence, 0u32, 0u32, 1.0f32, 0.0f32, 0u32],
    );
    e.call(SET_FLAG_0047AA40, &args![manager, 1u32]);
    let age = x87_load32(e.get(this, MagicModelHitEffect::fAge));
    let zero = f64::from_bits(e.mem.u64(ZERO));
    // FCOMP double; TEST AH,0x41; JNZ: the phase is set only when fAge > 0.0.
    if age > zero {
        let age = x87_quiet_bits(e.get(this, MagicModelHitEffect::fAge));
        e.call(SET_PHASE, &args![sequence, age, 0u32]);
    }
    true
}

// Translated from 0081c600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `0081c810` and frees 0x3c bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn magic_model_hit_effect_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0081_c810, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x3cu32]);
    }
    this
}

// Translated from 0081c5f0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011dc724 without reading the receiver. The map names this body `MagicModelHitEffect::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0081c5f0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_c724
}

/// `BSTempEffect::fLifetime` (+8) and `MagicHitEffect::pEffect` (+0x18), the
/// base's fields this file's layout leaves out.
const LIFETIME: u32 = 0x08;
const EFFECT: u32 = 0x18;
/// The members' offsets: `spObject` (+0x30) and the sub-buffer (+0x38).
const OBJECT: u32 = 0x30;
const SUB_BUFFER: u32 = 0x38;
/// The `float` the second constructor stores for a negative lifetime
/// (`01016970`, the largest finite `float`) and -1.0 (`01012054`).
const LONGEST_LIFETIME: u32 = 0x0101_6970;
const MINUS_ONE: u32 = 0x0101_2054;
/// The `double` 2.0 (`01011590`) and 64.0 (`010240c0`).
const TWO_DOUBLE: u32 = 0x0101_1590;
const SIXTY_FOUR: u32 = 0x0102_40c0;
/// `MagicHitEffect::Update` (engine map, `0081b860`, `thiscall`, one
/// `float`), `GetSaveSize`, `SaveGame`, `LoadGame` (`0081bd30`,
/// `0081bd80`, `0081bde0`, two words each), `SaveGame_ov2`,
/// `LoadGame_ov2` (`0081be80`, `0081bed0`, one word) and `0081bc70`
/// (`thiscall`, one word).
const MAGIC_HIT_EFFECT_UPDATE: u32 = 0x0081_b860;
const MAGIC_HIT_EFFECT_GET_SAVE_SIZE: u32 = 0x0081_bd30;
const MAGIC_HIT_EFFECT_SAVE_GAME: u32 = 0x0081_bd80;
const MAGIC_HIT_EFFECT_LOAD_GAME: u32 = 0x0081_bde0;
const MAGIC_HIT_EFFECT_SAVE_GAME_OV2: u32 = 0x0081_be80;
const MAGIC_HIT_EFFECT_LOAD_GAME_OV2: u32 = 0x0081_bed0;
const MAGIC_HIT_EFFECT_0081BC70: u32 = 0x0081_bc70;
/// `00508120` and `00508100` (`thiscall` on the sequence: two times in
/// `ST0`), `008041a0` and `0059bb30` (`thiscall` on the sequence: its
/// words the code tests).
const SEQUENCE_END_00508120: u32 = 0x0050_8120;
const SEQUENCE_TIME_00508100: u32 = 0x0050_8100;
const SEQUENCE_TEST_008041A0: u32 = 0x0080_41a0;
const SEQUENCE_STATE_0059BB30: u32 = 0x0059_bb30;
/// `00950bb0` (`thiscall` on the player, one word: its 3D for a view),
/// `BGSSaveFormBuffer::GetForm` (the engine map's name for `007af430`: the
/// reference's base form), `00401170` (the form's type), the world bound
/// (`NiAVObject::GetWorldBound`, `0043d450`), `006815c0` (answers `this`),
/// `0045bb80` (the node's position), `006a9540` (the node's rotation),
/// `00440460` / `0043fa80` (`thiscall`, one word: set a translation /
/// rotation).
const PLAYER_NODE_00950BB0: u32 = 0x0095_0bb0;
const BASE_FORM_007AF430: u32 = 0x007a_f430;
const FORM_TYPE_00401170: u32 = 0x0040_1170;
const WORLD_BOUND: u32 = 0x0043_d450;
const SELF_006815C0: u32 = 0x0068_15c0;
const POSITION_0045BB80: u32 = 0x0045_bb80;
const ROTATION_006A9540: u32 = 0x006a_9540;
const SET_TRANSLATE_00440460: u32 = 0x0044_0460;
const SET_ROTATE_0043FA80: u32 = 0x0043_fa80;
/// `0043b480` (`thiscall`: a node's child count), `0043b4a0` (`thiscall`,
/// one word: a child), `0043c490` (`thiscall`) and `00439090` (`thiscall`,
/// one word) with `011f426c`: the test of the third level of children.
const CHILD_COUNT_0043B480: u32 = 0x0043_b480;
const CHILD_AT_0043B4A0: u32 = 0x0043_b4a0;
const NODE_DATA_0043C490: u32 = 0x0043_c490;
const COMPARE_00439090: u32 = 0x0043_9090;
const COMPARE_ARGUMENT_011F426C: u32 = 0x011f_426c;
/// `0055fdb0` (`cdecl`, the controller manager: its save size),
/// `0055fe70` / `0055ffa0` / `TESObjectREFR::SaveControllerManager`
/// (`00563f30`) / `00564010` (`cdecl`: save and load a controller
/// manager's state).
const CONTROLLER_SAVE_SIZE: u32 = 0x0055_fdb0;
const CONTROLLER_SAVE: u32 = 0x0055_fe70;
const CONTROLLER_LOAD: u32 = 0x0055_ffa0;
const SAVE_CONTROLLER_MANAGER: u32 = 0x0056_3f30;
const CONTROLLER_LOAD_BUFFER: u32 = 0x0056_4010;
/// `MiddleHighProcess::GetSavedAcquireObject` (engine map's name for
/// `008d8520`: the actor's process), `__RTDynamicCast` (`00ec43fb`) with
/// `0118a6b8` (`BaseProcess`) to `0118c64c` (`MiddleHighProcess`), and
/// `00453700` (`thiscall`: a `float` in `ST0`).
const PROCESS_OF: u32 = 0x008d_8520;
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_BASE_PROCESS: u32 = 0x0118_a6b8;
const TYPE_MIDDLE_HIGH_PROCESS: u32 = 0x0118_c64c;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const ANIMATION_TIME_00453700: u32 = 0x0045_3700;
/// The save/load game object (`011de45c`): its write and read (`008579b0`,
/// `008579e0`), `TESSaveLoadGame::SaveNumericID` / `LoadNumericID`
/// (`00857a10`, `00857aa0`), its position (`00825c00`) and position set
/// (`00600ab0`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const SAVE_WRITE: u32 = 0x0085_79b0;
const LOAD_READ: u32 = 0x0085_79e0;
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
const WORD_14_00825C00: u32 = 0x0082_5c00;
const SET_POSITION_00600AB0: u32 = 0x0060_0ab0;
/// `0084e3a0` (`thiscall`: an object's word the code takes as its form
/// id or effect item) and `ModelLoader::QueueModel` (engine map, `00444040`,
/// seven words).
const OBJECT_ID_0084E3A0: u32 = 0x0084_e3a0;
const QUEUE_MODEL: u32 = 0x0044_4040;
/// `operator new` (`00401000`), `00401460` (`cdecl`: copy), `00403d30`
/// (`cdecl`: fill) and the string length `0044a670` (`cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const COPY_00401460: u32 = 0x0040_1460;
const FILL_00403D30: u32 = 0x0040_3d30;
const STRING_LENGTH: u32 = 0x0044_a670;
/// `004839c0` (`cdecl`: the form of an id), `008d6f30` (`thiscall`: the
/// reference's cell) and `00450ff0` (`thiscall` on the cell, returns a
/// byte).
const LOOKUP_FORM_004839C0: u32 = 0x0048_39c0;
const REFERENCE_CELL_008D6F30: u32 = 0x008d_6f30;
const CELL_TEST_00450FF0: u32 = 0x0045_0ff0;
/// `BGSSaveGameBuffer::SaveString` (engine map, `00865e70`, two words),
/// `StartVariableSizedValue` (`00865f20`), `SaveVariableSizedValue_ov2`
/// (`00865ff0`, two words), `BGSLoadGameBuffer::LoadString` (`008649a0`,
/// one word), `BGSLoadGameSubBuffer::LoadGame` (`008655a0`, one word) and
/// `CreateLoadFormBuffer` (`00864e20`, one word).
const SAVE_STRING: u32 = 0x0086_5e70;
const START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5ff0;
const LOAD_STRING: u32 = 0x0086_49a0;
const SUB_BUFFER_LOAD_GAME: u32 = 0x0086_55a0;
const CREATE_LOAD_FORM_BUFFER: u32 = 0x0086_4e20;
/// `008d05e0` (`thiscall`, three words; the base's `FinishInitLoadGame`
/// as this class calls it), `009dace0` (`thiscall`, one word; the base's
/// `FinishLoadGame` as this class calls it) and `0081db60` (`thiscall`,
/// one word) on the load form buffer.
const BASE_FINISH_INIT_LOAD_GAME: u32 = 0x008d_05e0;
const BASE_FINISH_LOAD_GAME: u32 = 0x009d_ace0;
const LOAD_FORM_BUFFER_0081DB60: u32 = 0x0081_db60;

/// The controller manager of `spObject` (`0043b230`, `00653270` with
/// `011f36ac`), or 0 without an object.
fn object_controller_manager(e: &mut Engine, this: Ptr<MagicModelHitEffect>) -> u32 {
    let holder = this.addr().wrapping_add(OBJECT);
    if e.call(POINTER_GET, &args![holder]).u32() == 0 {
        return 0;
    }
    let object = e.call(POINTER_GET, &args![holder]).u32();
    let root = e.call(MODEL_ROOT_0043B230, &args![object]).u32();
    e.call(
        CONTROLLER_MANAGER_00653270,
        &args![LOOKUP_ARGUMENT_011F36AC, root],
    )
    .u32()
}

/// The animation time an actor reference gives (`008d8520`'s process cast
/// to `MiddleHighProcess`, its slot +0x1B8, `00453700`), stored as a
/// `float`, or -1.0 (`01012054`); `reference` counts only when its slot
/// +0x100 answers true, and only with `required` set.
fn actor_animation_time(e: &mut Engine, reference: u32, required: u32) -> u32 {
    let mut time = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
    let actor = if reference != 0 && e.vcall(reference, 0x100, &[]).bool() {
        reference
    } else {
        0
    };
    if actor != 0 && required != 0 {
        let process = e.call(PROCESS_OF, &args![actor]).u32();
        let process = e
            .call(
                DYNAMIC_CAST,
                &args![
                    process,
                    0u32,
                    TYPE_BASE_PROCESS,
                    TYPE_MIDDLE_HIGH_PROCESS,
                    0u32
                ],
            )
            .u32();
        if process != 0 && e.vcall(process, 0x1b8, &[]).u32() != 0 {
            let animation = e.vcall(process, 0x1b8, &[]).u32();
            time = x87_store32(e.call(ANIMATION_TIME_00453700, &args![animation]).st0);
        }
    }
    time
}

/// The time of a buffer's biped (`buffer`'s slot `slot`, then +0x1E4,
/// then `00453700`), stored as a `float`, or -1.0 (`01012054`).
fn biped_time(e: &mut Engine, buffer: u32, slot: u32) -> u32 {
    let owner = e.vcall(buffer, slot, &[]).u32();
    let biped = if owner != 0 {
        e.vcall(owner, 0x1e4, &[]).u32()
    } else {
        0
    };
    if biped != 0 {
        x87_store32(e.call(ANIMATION_TIME_00453700, &args![biped]).st0)
    } else {
        x87_quiet_bits(e.global::<u32>(MINUS_ONE))
    }
}

/// Queues the effect setting's model (`0084e3a0`, `00825c00`, +0x18's slot
/// +0x14 for the path) with `ModelLoader::QueueModel(path, 5, 0, 0, 1, 0, 0)`
/// when it has one.
fn queue_effect_model(e: &mut Engine, effect: u32) {
    let item = e.call(OBJECT_ID_0084E3A0, &args![effect]).u32();
    let setting = e.call(WORD_14_00825C00, &args![item]).u32();
    if e.vcall(setting.wrapping_add(0x18), 0x14, &[]).u32() == 0 {
        return;
    }
    let item = e.call(OBJECT_ID_0084E3A0, &args![effect]).u32();
    let setting = e.call(WORD_14_00825C00, &args![item]).u32();
    let path = e.vcall(setting.wrapping_add(0x18), 0x14, &[]).u32();
    let loader = e.mem.u32(MODEL_LOADER);
    e.call(
        QUEUE_MODEL,
        &args![loader, path, 5u32, 0u32, 0u32, 1u32, 0u32, 0u32],
    );
}

// Translated from 0081c720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::MagicModelHitEffect_ov2` (engine map; `thiscall`,
/// `RET 0xC`: the target, the file name, the lifetime): the
/// `MagicHitEffect` base with the target and 0, the vtable, `spObject` and
/// `spTarget` constructed and assigned null, the sub-buffer constructed,
/// `b3rdPerson` and `bOwnPath` cleared, `pFilename` the name, and the
/// lifetime (a negative one becomes the largest `float`, `01016970`).
/// Returns `this`.
// Translation incomplete: 0081c720 - the FS:[0] exception frame (unwinding the members on a throw) is not translated
pub fn magic_model_hit_effect_magic_model_hit_effect_ov2(
    e: &mut Engine,
    this: Ptr<MagicModelHitEffect>,
    target: u32,
    name: u32,
    lifetime: f32,
) -> Ptr<MagicModelHitEffect> {
    let base = this.addr();
    e.call(MAGIC_HIT_EFFECT_CONSTRUCTOR, &args![base, target, 0u32]);
    e.mem.set_u32(base, MAGIC_MODEL_HIT_EFFECT_VTABLE);
    e.call(POINTER_CONSTRUCTOR, &args![base.wrapping_add(OBJECT), 0u32]);
    e.call(POINTER_CONSTRUCTOR, &args![base.wrapping_add(0x34), 0u32]);
    e.call(
        SUB_BUFFER_CONSTRUCTOR,
        &args![base.wrapping_add(SUB_BUFFER)],
    );
    e.call(POINTER_ASSIGN, &args![base.wrapping_add(OBJECT), 0u32]);
    e.call(POINTER_ASSIGN, &args![base.wrapping_add(0x34), 0u32]);
    e.set(this, MagicModelHitEffect::b3rdPerson, 0);
    e.set(this, MagicModelHitEffect::pFilename, name);
    e.set(this, MagicModelHitEffect::bOwnPath, 0);
    // FCOMP with 0.0, TEST AH,5, JP: the largest float only below 0.0.
    let stored = if x87_load32(lifetime.to_bits()) < e.global::<f64>(ZERO) {
        x87_quiet_bits(e.global::<u32>(LONGEST_LIFETIME))
    } else {
        x87_quiet_bits(lifetime.to_bits())
    };
    e.mem.set_u32(base.wrapping_add(LIFETIME), stored);
    this
}

// Translated from 0081cc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::Update` (engine map; `thiscall`, `RET 4`: the
/// elapsed seconds; answers a byte): false without a target whose slot
/// +0x1D0 answers, or when `MagicHitEffect::Update` answers false. With
/// `spObject`: this effect's slot +0xCC, then, when the object's
/// controller manager has the `"SpecialIdle_HitEffect"` sequence and the
/// sequence's `00508100` time is below its `00508120` time, `008041a0` is
/// set and either `0059bb30` is 2 or `bFinished` is already set:
/// `bFinished` is set, the sequence's slot +0x8C runs with (0.0, 0) and
/// the lifetime takes `fAge`. A finished effect answers false once `fAge`
/// is past the lifetime plus 2; otherwise true.
// Translation incomplete: 0081cc70 - the FS:[0] exception frame (destroying the fixed string on a throw) is not translated
pub fn magic_model_hit_effect_update(
    e: &mut Engine,
    this: Ptr<MagicModelHitEffect>,
    seconds: f32,
) -> u8 {
    let base = this.addr();
    let target = e.get(this, MagicModelHitEffect::pTarget);
    if target == 0 || e.vcall(target, 0x1d0, &[]).u32() == 0 {
        return 0;
    }
    let seconds = x87_quiet_bits(seconds.to_bits());
    if !e
        .call(MAGIC_HIT_EFFECT_UPDATE, &args![base, seconds])
        .bool()
    {
        return 0;
    }
    if e.call(POINTER_GET, &args![base.wrapping_add(OBJECT)]).u32() != 0 {
        e.vcall(base, 0xcc, &[]);
        let manager = object_controller_manager(e, this);
        if manager != 0 {
            let sequence = hit_effect_sequence(e, manager);
            if sequence != 0 {
                let end = e.call(SEQUENCE_END_00508120, &args![sequence]).st0;
                let time = e.call(SEQUENCE_TIME_00508100, &args![sequence]).st0;
                // FCOMP double; TEST AH,5; JP: on only when time < end.
                if time < end
                    && e.call(SEQUENCE_TEST_008041A0, &args![sequence]).u32() != 0
                    && (e.call(SEQUENCE_STATE_0059BB30, &args![sequence]).u32() == 2
                        || e.get(this, MagicModelHitEffect::bFinished) != 0)
                {
                    e.set(this, MagicModelHitEffect::bFinished, 1);
                    e.vcall(sequence, 0x8c, &args![0u32, 0u32]);
                    let age = x87_quiet_bits(e.get(this, MagicModelHitEffect::fAge));
                    e.mem.set_u32(base.wrapping_add(LIFETIME), age);
                }
            }
        }
    }
    if e.get(this, MagicModelHitEffect::bFinished) != 0 {
        let lifetime = x87_add(
            x87_load32(e.mem.u32(base.wrapping_add(LIFETIME))),
            e.global::<f64>(TWO_DOUBLE),
        );
        let age = x87_load32(e.get(this, MagicModelHitEffect::fAge));
        // FCOMPP; TEST AH,0x41; JNZ: false only when fAge > lifetime + 2.
        if age > lifetime {
            return 0;
        }
    }
    1
}

// Translated from 0081ce20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::UpdatePosition` (engine map; `thiscall`): the
/// target's 3D (the player's through `00950bb0` with 0, `b3rdPerson`
/// taking `004eaf60`; others' slot +0x1D0), `0081bc70` with `spObject`,
/// then, with 3D, the position: for an actor whose slot +0x22C (with 0)
/// answers true, or whose slot +0x230 does, the 3D's position, 64 lower
/// unless the node's third level of children passes `00439090` (with
/// `011f426c`); otherwise the 3D's position, or for a base form of type
/// 0x1C its world bound's centre at the 3D's height. `spObject` takes that
/// translation and the 3D's rotation.
pub fn magic_model_hit_effect_update_position(e: &mut Engine, this: Ptr<MagicModelHitEffect>) {
    let base = this.addr();
    let player = e.mem.u32(PLAYER);
    let node = if e.get(this, MagicModelHitEffect::pTarget) == player {
        let third_person = e.call(PLAYER_FLAG_004EAF60, &args![player]).u8();
        e.set(this, MagicModelHitEffect::b3rdPerson, third_person);
        let player = e.mem.u32(PLAYER);
        e.call(PLAYER_NODE_00950BB0, &args![player, 0u32]).u32()
    } else {
        let target = e.get(this, MagicModelHitEffect::pTarget);
        e.vcall(target, 0x1d0, &[]).u32()
    };
    let object = e.call(POINTER_GET, &args![base.wrapping_add(OBJECT)]).u32();
    e.call(MAGIC_HIT_EFFECT_0081BC70, &args![base, object]);
    if node == 0 {
        return;
    }
    e.with_stack(0xc, |e, point| {
        let point = point.addr();
        e.call(SELF_006815C0, &args![point]);
        let target = e.get(this, MagicModelHitEffect::pTarget);
        let knocked = e.vcall(target, 0x100, &[]).bool() && {
            let target = e.get(this, MagicModelHitEffect::pTarget);
            e.vcall(target, 0x22c, &args![0u32]).bool()
        };
        let lying = knocked || {
            let target = e.get(this, MagicModelHitEffect::pTarget);
            e.vcall(target, 0x230, &[]).bool()
        };
        let copy = |e: &mut Engine, source: u32| {
            for i in 0..3 {
                let word = e.mem.u32(source.wrapping_add(4 * i));
                e.mem.set_u32(point.wrapping_add(4 * i), word);
            }
        };
        if lying {
            let mut lower = true;
            let first = e.vcall(node, 0xc, &[]).u32();
            if first != 0 && e.call(CHILD_COUNT_0043B480, &args![first]).u32() != 0 {
                let child = e.call(CHILD_AT_0043B4A0, &args![first, 0u32]).u32();
                let second = if child != 0 {
                    e.vcall(child, 0xc, &[]).u32()
                } else {
                    0
                };
                if second != 0 && e.call(CHILD_COUNT_0043B480, &args![second]).u32() != 0 {
                    let child = e.call(CHILD_AT_0043B4A0, &args![second, 0u32]).u32();
                    let third = if child != 0 {
                        e.vcall(child, 0xc, &[]).u32()
                    } else {
                        0
                    };
                    if third != 0 {
                        let word = e.call(NODE_DATA_0043C490, &args![third]).u32();
                        if e.call(COMPARE_00439090, &args![word, COMPARE_ARGUMENT_011F426C])
                            .bool()
                        {
                            lower = false;
                        }
                    }
                }
            }
            let position = e.call(POSITION_0045BB80, &args![node]).u32();
            copy(e, position);
            if lower {
                let z = x87_load32(e.mem.u32(point.wrapping_add(8)));
                let z = x87_store32(x87_sub(z, e.global::<f64>(SIXTY_FOUR)));
                e.mem.set_u32(point.wrapping_add(8), z);
            }
        } else {
            let target = e.get(this, MagicModelHitEffect::pTarget);
            let bound = e.call(BASE_FORM_007AF430, &args![target]).u32() != 0 && {
                let target = e.get(this, MagicModelHitEffect::pTarget);
                let form = e.call(BASE_FORM_007AF430, &args![target]).u32();
                e.call(FORM_TYPE_00401170, &args![form]).u32() == 0x1c
            };
            if bound {
                let sphere = e.call(WORLD_BOUND, &args![node]).u32();
                let centre = e.call(SELF_006815C0, &args![sphere]).u32();
                copy(e, centre);
                let position = e.call(POSITION_0045BB80, &args![node]).u32();
                let z = x87_quiet_bits(e.mem.u32(position.wrapping_add(8)));
                e.mem.set_u32(point.wrapping_add(8), z);
            } else {
                let position = e.call(POSITION_0045BB80, &args![node]).u32();
                copy(e, position);
            }
        }
        let holder = base.wrapping_add(OBJECT);
        if e.call(POINTER_GET, &args![holder]).u32() != 0 {
            let object = e.call(POINTER_GET, &args![holder]).u32();
            e.call(SET_TRANSLATE_00440460, &args![object, point]);
            let rotation = e.call(ROTATION_006A9540, &args![node]).u32();
            let object = e.call(POINTER_GET, &args![holder]).u32();
            e.call(SET_ROTATE_0043FA80, &args![object, rotation]);
        }
    });
}

// Translated from 0081d0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::GetSaveSize` (engine map; `thiscall`, `RET 8`):
/// the base's size plus 2, plus the controller manager's save size
/// (`0055fdb0`) when `spObject` has one (16-bit sums).
pub fn magic_model_hit_effect_get_save_size(
    e: &mut Engine,
    this: Ptr<MagicModelHitEffect>,
    first: u32,
    second: u32,
) -> u16 {
    let mut size = e
        .call(MAGIC_HIT_EFFECT_GET_SAVE_SIZE, &args![this, first, second])
        .u16();
    size = size.wrapping_add(2);
    let manager = object_controller_manager(e, this);
    if manager != 0 {
        let more = e.call(CONTROLLER_SAVE_SIZE, &args![manager]).u16();
        size = size.wrapping_add(more);
    }
    size
}

// Translated from 0081d150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::SaveGame` (engine map; `thiscall`, `RET 8`: the
/// active effect and the reference): the base's save, then the controller
/// manager's save size as a 16-bit word (0 without one), and with a size
/// the manager's state (`0055fe70`) at the reference's animation time
/// (-1.0 unless an actor with a middle-high process gives one).
pub fn magic_model_hit_effect_save_game(
    e: &mut Engine,
    this: Ptr<MagicModelHitEffect>,
    effect: u32,
    reference: u32,
) {
    e.call(MAGIC_HIT_EFFECT_SAVE_GAME, &args![this, effect, reference]);
    let manager = object_controller_manager(e, this);
    e.with_stack(4, |e, size| {
        let size = size.addr();
        e.mem.set_u16(size, 0);
        if manager != 0 {
            let value = e.call(CONTROLLER_SAVE_SIZE, &args![manager]).u16();
            e.mem.set_u16(size, value);
        }
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(SAVE_WRITE, &args![save_load, size, 2u32]);
        if e.mem.u16(size) == 0 {
            return;
        }
        let time = actor_animation_time(e, reference, effect);
        e.call(CONTROLLER_SAVE, &args![manager, x87_quiet_bits(time)]);
    });
}

// Translated from 0081d2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::LoadGame` (engine map; `thiscall`, `RET 8`: the
/// active effect and a word): the base's load, the effect setting's model
/// queued (`queue_effect_model`) when there is an active effect, then a
/// 16-bit length and, when it is not 0, a new block of the length plus 2
/// holding the length and the bytes read after it, kept in `pFilename`.
pub fn magic_model_hit_effect_load_game(
    e: &mut Engine,
    this: Ptr<MagicModelHitEffect>,
    effect: u32,
    second: u32,
) {
    e.call(MAGIC_HIT_EFFECT_LOAD_GAME, &args![this, effect, second]);
    if effect != 0 {
        queue_effect_model(e, effect);
    }
    e.with_stack(4, |e, length| {
        let length = length.addr();
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(LOAD_READ, &args![save_load, length, 2u32]);
        let count = u32::from(e.mem.u16(length));
        if count == 0 {
            return;
        }
        let block = e.call(OPERATOR_NEW, &args![count + 2]).u32();
        e.call(COPY_00401460, &args![block, length, 2u32]);
        let count = u32::from(e.mem.u16(length));
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(LOAD_READ, &args![save_load, block.wrapping_add(2), count]);
        e.set(this, MagicModelHitEffect::pFilename, block);
    });
}

// Translated from 0081d3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::FinishInitLoadGame` (engine map; `thiscall`,
/// `RET 0xC`: the active effect, the reference, a file name): the base's
/// (`008d05e0` with the two words and 0); `pFilename` becomes the effect
/// setting's model path (with an active effect) or the third word; this
/// effect's slot +0xC4 runs with `fAge` and `bFinished` kept; the save
/// position is moved to the old `pFilename` word (`00600ab0`) for a 16-bit
/// read and, when `spObject` has a controller manager, its state is loaded
/// (`0055ffa0`) at the reference's animation time; then the position is
/// restored, the old word freed and slot +0x94 runs with 0.0.
pub fn magic_model_hit_effect_finish_init_load_game(
    e: &mut Engine,
    this: Ptr<MagicModelHitEffect>,
    effect: u32,
    reference: u32,
    name: u32,
) {
    let base = this.addr();
    e.call(
        BASE_FINISH_INIT_LOAD_GAME,
        &args![base, effect, reference, 0u32],
    );
    let old = e.get(this, MagicModelHitEffect::pFilename);
    if effect != 0 {
        let item = e.call(OBJECT_ID_0084E3A0, &args![effect]).u32();
        let setting = e.call(WORD_14_00825C00, &args![item]).u32();
        let path = e.vcall(setting.wrapping_add(0x18), 0x14, &[]).u32();
        e.set(this, MagicModelHitEffect::pFilename, path);
    } else {
        e.set(this, MagicModelHitEffect::pFilename, name);
    }
    let age = x87_quiet_bits(e.get(this, MagicModelHitEffect::fAge));
    let finished = e.get(this, MagicModelHitEffect::bFinished);
    e.vcall(base, 0xc4, &[]);
    e.set(this, MagicModelHitEffect::fAge, x87_quiet_bits(age));
    e.set(this, MagicModelHitEffect::bFinished, finished);
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    let saved = e.call(WORD_14_00825C00, &args![save_load]).u32();
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    e.call(SET_POSITION_00600AB0, &args![save_load, old]);
    e.with_stack(4, |e, count| {
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(LOAD_READ, &args![save_load, count, 2u32]);
    });
    let manager = object_controller_manager(e, this);
    if manager != 0 {
        let time = actor_animation_time(e, reference, effect);
        let object = e.call(POINTER_GET, &args![base.wrapping_add(OBJECT)]).u32();
        e.call(
            CONTROLLER_LOAD,
            &args![manager, object, x87_quiet_bits(time)],
        );
    }
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    e.call(SET_POSITION_00600AB0, &args![save_load, saved]);
    e.call(FREE, &args![old]);
    e.vcall(base, 0x94, &args![0u32]);
}

// Translated from 0081d5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::CheckShouldSave` (engine map; `thiscall`): true
/// only without an active effect (+0x18) and with a target and a file
/// name.
pub fn magic_model_hit_effect_check_should_save(
    e: &mut Engine,
    this: Ptr<MagicModelHitEffect>,
) -> u8 {
    if e.mem.u32(this.addr().wrapping_add(EFFECT)) != 0 {
        return 0;
    }
    if e.get(this, MagicModelHitEffect::pTarget) == 0
        || e.get(this, MagicModelHitEffect::pFilename) == 0
    {
        return 0;
    }
    1
}

// Translated from 0081d5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::GetSaveSize_ov2` (engine map; `thiscall`): 5 plus
/// the file name's length plus this effect's slot +0xD0 with (0, target)
/// (16-bit sums).
pub fn magic_model_hit_effect_get_save_size_ov2(
    e: &mut Engine,
    this: Ptr<MagicModelHitEffect>,
) -> u16 {
    let mut size: u16 = 4;
    size = size.wrapping_add(1);
    let name = e.get(this, MagicModelHitEffect::pFilename);
    let length = e.call(STRING_LENGTH, &args![name]).u32();
    size = (u32::from(size).wrapping_add(length)) as u16;
    let target = e.get(this, MagicModelHitEffect::pTarget);
    let more = e.vcall(this.addr(), 0xd0, &args![0u32, target]).u16();
    size.wrapping_add(more)
}

// Translated from 0081d640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::SaveGame_ov2` (engine map; `thiscall`): the
/// target's id (`0084e3a0`) through `TESSaveLoadGame::SaveNumericID`, the
/// file name's length as a byte and its bytes, then this effect's slot
/// +0xD4 with (0, target).
pub fn magic_model_hit_effect_save_game_ov2(e: &mut Engine, this: Ptr<MagicModelHitEffect>) {
    e.with_stack(8, |e, frame| {
        // `[EBP-4]` (the id) and `[EBP-5]` (the length byte).
        let id = frame.addr().wrapping_add(4);
        let length = frame.addr().wrapping_add(3);
        let target = e.get(this, MagicModelHitEffect::pTarget);
        let value = e.call(OBJECT_ID_0084E3A0, &args![target]).u32();
        e.mem.set_u32(id, value);
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(SAVE_NUMERIC_ID, &args![save_load, id, 4u32]);
        let name = e.get(this, MagicModelHitEffect::pFilename);
        let count = e.call(STRING_LENGTH, &args![name]).u32();
        e.mem.set_u8(length, count as u8);
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(SAVE_WRITE, &args![save_load, length, 1u32]);
        let count = u32::from(e.mem.u8(length));
        let name = e.get(this, MagicModelHitEffect::pFilename);
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(SAVE_WRITE, &args![save_load, name, count]);
    });
    let target = e.get(this, MagicModelHitEffect::pTarget);
    e.vcall(this.addr(), 0xd4, &args![0u32, target]);
}

// Translated from 0081d6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::LoadGame_ov2` (engine map; `thiscall`; answers a
/// byte): reads the target's id (`TESSaveLoadGame::LoadNumericID`) and
/// takes the form cast to `TESObjectREFR` as the target; the answer is
/// true when the target has a cell (`008d6f30`) that passes `00450ff0`.
/// Reads the file name (a length byte, a zeroed new block of length + 1,
/// the bytes), sets `bOwnPath`, calls this effect's slot +0xD8 with (0,
/// target) and, when the answer is true, slots +0xDC with (0, target) and
/// +0xE0 with (0, target, name).
pub fn magic_model_hit_effect_load_game_ov2(e: &mut Engine, this: Ptr<MagicModelHitEffect>) -> u8 {
    let base = this.addr();
    e.with_stack(0x10, |e, frame| {
        // `[EBP-8]` (the id) and `[EBP-0xD]` (the length byte).
        let id = frame.addr().wrapping_add(8);
        let length = frame.addr().wrapping_add(3);
        let mut loaded = 1u8;
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(LOAD_NUMERIC_ID, &args![save_load, id, 4u32]);
        let id_value = e.mem.u32(id);
        let form = e.call(LOOKUP_FORM_004839C0, &args![id_value]).u32();
        let target = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0u32],
            )
            .u32();
        e.set(this, MagicModelHitEffect::pTarget, target);
        let target = e.get(this, MagicModelHitEffect::pTarget);
        let good = target != 0 && e.call(REFERENCE_CELL_008D6F30, &args![target]).u32() != 0 && {
            let target = e.get(this, MagicModelHitEffect::pTarget);
            let cell = e.call(REFERENCE_CELL_008D6F30, &args![target]).u32();
            e.call(CELL_TEST_00450FF0, &args![cell]).bool()
        };
        if !good {
            loaded = 0;
        }
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(LOAD_READ, &args![save_load, length, 1u32]);
        let size = u32::from(e.mem.u8(length)) + 1;
        let name = e.call(OPERATOR_NEW, &args![size]).u32();
        let size = u32::from(e.mem.u8(length)) + 1;
        e.call(FILL_00403D30, &args![name, 0u32, size]);
        let count = u32::from(e.mem.u8(length));
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(LOAD_READ, &args![save_load, name, count]);
        e.set(this, MagicModelHitEffect::bOwnPath, 1);
        let target = e.get(this, MagicModelHitEffect::pTarget);
        e.vcall(base, 0xd8, &args![0u32, target]);
        if loaded != 0 {
            let target = e.get(this, MagicModelHitEffect::pTarget);
            e.vcall(base, 0xdc, &args![0u32, target]);
            let target = e.get(this, MagicModelHitEffect::pTarget);
            e.vcall(base, 0xe0, &args![0u32, target, name]);
        }
        loaded
    })
}

// Translated from 0081d800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::SaveGame_ov3` (engine map; `thiscall`, `RET 4`:
/// the save buffer): the base's save, the file name
/// (`BGSSaveGameBuffer::SaveString` with 0), then a variable-sized value
/// holding, when `spObject` has a controller manager, its state
/// (`TESObjectREFR::SaveControllerManager` with the buffer and -1.0; the
/// buffer's slot +8 biped time is computed and not used).
pub fn magic_model_hit_effect_save_game_ov3(
    e: &mut Engine,
    this: Ptr<MagicModelHitEffect>,
    buffer: u32,
) {
    e.call(MAGIC_HIT_EFFECT_SAVE_GAME_OV2, &args![this, buffer]);
    let name = e.get(this, MagicModelHitEffect::pFilename);
    e.call(SAVE_STRING, &args![buffer, name, 0u32]);
    let manager = object_controller_manager(e, this);
    let start = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let before = e.call(OBJECT_ID_0084E3A0, &args![buffer]).u32();
    if manager != 0 {
        biped_time(e, buffer, 8);
        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
        e.call(SAVE_CONTROLLER_MANAGER, &args![manager, buffer, minus_one]);
    }
    let after = e.call(OBJECT_ID_0084E3A0, &args![buffer]).u32();
    e.call(
        SAVE_VARIABLE_SIZED_VALUE,
        &args![buffer, after.wrapping_sub(before), start],
    );
}

// Translated from 0081d920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::LoadGame_ov3` (engine map; `thiscall`, `RET 4`:
/// the load buffer): the base's load, the file name
/// (`BGSLoadGameBuffer::LoadString` with 0) with `bOwnPath` set, the
/// sub-buffer loaded, and the effect setting's model queued when there is
/// an active effect.
pub fn magic_model_hit_effect_load_game_ov3(
    e: &mut Engine,
    this: Ptr<MagicModelHitEffect>,
    buffer: u32,
) {
    let base = this.addr();
    e.call(MAGIC_HIT_EFFECT_LOAD_GAME_OV2, &args![base, buffer]);
    let name = e.call(LOAD_STRING, &args![buffer, 0u32]).u32();
    e.set(this, MagicModelHitEffect::pFilename, name);
    e.set(this, MagicModelHitEffect::bOwnPath, 1);
    e.call(
        SUB_BUFFER_LOAD_GAME,
        &args![base.wrapping_add(SUB_BUFFER), buffer],
    );
    let effect = e.mem.u32(base.wrapping_add(EFFECT));
    if effect != 0 {
        queue_effect_model(e, effect);
    }
}

// Translated from 0081d9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::FinishLoadGame` (engine map; `thiscall`, `RET 4`:
/// the load buffer): the base's (`009dace0`), this effect's slot +0xC4
/// with `fAge` and `bFinished` kept, then, when the sub-buffer holds data
/// and `spObject` has a controller manager, its state is loaded
/// (`00564010`) from a load form buffer (`CreateLoadFormBuffer` with 0) at
/// the buffer's biped time (slot +0xC) and the form buffer released
/// (`0081db60` with 1); the sub-buffer is freed (`00435b20`) and slot +0x94
/// runs with 0.0.
pub fn magic_model_hit_effect_finish_load_game(
    e: &mut Engine,
    this: Ptr<MagicModelHitEffect>,
    buffer: u32,
) {
    let base = this.addr();
    e.call(BASE_FINISH_LOAD_GAME, &args![base, buffer]);
    let age = x87_quiet_bits(e.get(this, MagicModelHitEffect::fAge));
    let finished = e.get(this, MagicModelHitEffect::bFinished);
    e.vcall(base, 0xc4, &[]);
    e.set(this, MagicModelHitEffect::fAge, x87_quiet_bits(age));
    e.set(this, MagicModelHitEffect::bFinished, finished);
    let sub_buffer = base.wrapping_add(SUB_BUFFER);
    if e.call(POINTER_GET, &args![sub_buffer]).u32() != 0 {
        let manager = object_controller_manager(e, this);
        if manager != 0 {
            let time = x87_quiet_bits(biped_time(e, buffer, 0xc));
            let form_buffer = e
                .call(CREATE_LOAD_FORM_BUFFER, &args![sub_buffer, 0u32])
                .u32();
            e.call(
                CONTROLLER_LOAD_BUFFER,
                &args![manager, form_buffer, x87_quiet_bits(time)],
            );
            if form_buffer != 0 {
                e.call(LOAD_FORM_BUFFER_0081DB60, &args![form_buffer, 1u32]);
            }
        }
    }
    e.call(SUB_BUFFER_DESTRUCTOR, &args![sub_buffer]);
    e.vcall(base, 0x94, &args![0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0081_c5f0, fn_0081c5f0(Ptr) -> u32),
        entry!(0x0081_c600, magic_model_hit_effect_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x0081_c630,
            magic_model_hit_effect_constructor(
                Ptr<MagicModelHitEffect>,
                u32,
                u32,
            ) -> Ptr<MagicModelHitEffect>
        ),
        entry!(
            0x0081_c810,
            magic_model_hit_effect_destructor(Ptr<MagicModelHitEffect>)
        ),
        entry!(
            0x0081_c950,
            magic_model_hit_effect_init(Ptr<MagicModelHitEffect>) -> bool
        ),
        entry!(
            0x0081_c720,
            magic_model_hit_effect_magic_model_hit_effect_ov2(
                Ptr<MagicModelHitEffect>,
                u32,
                u32,
                f32,
            )
                -> Ptr<MagicModelHitEffect>
        ),
        entry!(
            0x0081_cc70,
            magic_model_hit_effect_update(Ptr<MagicModelHitEffect>, f32) -> u8
        ),
        entry!(
            0x0081_ce20,
            magic_model_hit_effect_update_position(Ptr<MagicModelHitEffect>)
        ),
        entry!(
            0x0081_d0b0,
            magic_model_hit_effect_get_save_size(Ptr<MagicModelHitEffect>, u32, u32) -> u16
        ),
        entry!(
            0x0081_d150,
            magic_model_hit_effect_save_game(Ptr<MagicModelHitEffect>, u32, u32)
        ),
        entry!(
            0x0081_d2a0,
            magic_model_hit_effect_load_game(Ptr<MagicModelHitEffect>, u32, u32)
        ),
        entry!(
            0x0081_d3b0,
            magic_model_hit_effect_finish_init_load_game(Ptr<MagicModelHitEffect>, u32, u32, u32)
        ),
        entry!(
            0x0081_d5a0,
            magic_model_hit_effect_check_should_save(Ptr<MagicModelHitEffect>) -> u8
        ),
        entry!(
            0x0081_d5d0,
            magic_model_hit_effect_get_save_size_ov2(Ptr<MagicModelHitEffect>) -> u16
        ),
        entry!(
            0x0081_d640,
            magic_model_hit_effect_save_game_ov2(Ptr<MagicModelHitEffect>)
        ),
        entry!(
            0x0081_d6c0,
            magic_model_hit_effect_load_game_ov2(Ptr<MagicModelHitEffect>) -> u8
        ),
        entry!(
            0x0081_d800,
            magic_model_hit_effect_save_game_ov3(Ptr<MagicModelHitEffect>, u32)
        ),
        entry!(
            0x0081_d920,
            magic_model_hit_effect_load_game_ov3(Ptr<MagicModelHitEffect>, u32)
        ),
        entry!(
            0x0081_d9f0,
            magic_model_hit_effect_finish_load_game(Ptr<MagicModelHitEffect>, u32)
        ),
    ]
}

#[cfg(test)]
mod model_hit_effect_tests {
    use super::*;

    fn called(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == target)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// `NiPointer` doubles over plain words, and silent doubles for the rest.
    fn pointer_engine() -> Engine {
        let mut e = Engine::new();
        e.register(POINTER_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(WORD_18_009611E0, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        for address in [
            MAGIC_HIT_EFFECT_CONSTRUCTOR,
            MAGIC_HIT_EFFECT_DESTRUCTOR,
            POINTER_DESTRUCTOR,
            SUB_BUFFER_CONSTRUCTOR,
            SUB_BUFFER_DESTRUCTOR,
            RELEASE_MODEL,
            FREE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.map(0x011c_3000, 0x1000);
        e.mem.set_u32(MODEL_LOADER, 0x3b3c);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn constructor_takes_the_file_name_from_the_base_effect() {
        let mut e = pointer_engine();
        e.register(EFFECT_ITEM_0044DDC0, |_, _| 0x4400u32.into_ret());
        let av_effect = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x18);
        e.mem.set_u32(vtable + 0x14, 0x7014);
        e.register(0x7014, |_, _| 0x0abcu32.into_ret());
        e.mem.set_u32(av_effect + 0x18, vtable);
        e.register_double(GET_AV_EFFECT, move |_, _| av_effect.into_ret());
        let this = e.mem.alloc(0x3c);
        e.mem.write(this, &[0xee; 0x3c]);
        assert_eq!(e.call(0x0081_c630, &[this, 0x10, 0x20]).u32(), this);
        assert_eq!(
            called(&e, MAGIC_HIT_EFFECT_CONSTRUCTOR),
            vec![vec![this, 0x10, 0x20]]
        );
        assert_eq!(e.mem.u32(this), MAGIC_MODEL_HIT_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(called(&e, SUB_BUFFER_CONSTRUCTOR), vec![vec![this + 0x38]]);
        assert_eq!(called(&e, EFFECT_ITEM_0044DDC0), vec![vec![0x20]]);
        assert_eq!(called(&e, GET_AV_EFFECT), vec![vec![0x4400, 0]]);
        assert_eq!(called(&e, 0x7014), vec![vec![av_effect + 0x18]]);
        assert_eq!(e.mem.u32(this + 0x2c), 0x0abc);
        assert_eq!(e.mem.u8(this + 0x28), 0);
        assert_eq!(e.mem.u8(this + 0x29), 0);
    }

    #[test]
    fn destructor_detaches_releases_and_frees_an_owned_name() {
        for owned in [0u8, 1] {
            let mut e = pointer_engine();
            let parent_vtable = e.mem.alloc(0xec);
            e.mem.set_u32(parent_vtable + 0xe8, 0x70e8);
            e.register(0x70e8, |_, _| Ret::default());
            let parent = e.mem.alloc(8);
            e.mem.set_u32(parent, parent_vtable);
            let object = e.mem.alloc(0x20);
            e.mem.set_u32(object + 0x18, parent);
            let name = e.mem.alloc(4);
            e.mem.set_u8(name, b'a');
            let this = e.mem.alloc(0x3c);
            e.mem.set_u32(this + 0x30, object);
            e.mem.set_u32(this + 0x34, 0x3434);
            e.mem.set_u32(this + 0x2c, name);
            e.mem.set_u8(this + 0x28, owned);
            e.call(0x0081_c810, &[this]);
            assert_eq!(e.mem.u32(this), MAGIC_MODEL_HIT_EFFECT_VTABLE);
            assert_eq!(called(&e, 0x70e8), vec![vec![parent, object]]);
            assert_eq!(e.mem.u32(this + 0x30), 0);
            assert_eq!(e.mem.u32(this + 0x34), 0);
            assert_eq!(called(&e, RELEASE_MODEL), vec![vec![0x3b3c, name, 0, 1]]);
            assert_eq!(called(&e, FREE).len(), usize::from(owned));
            assert_eq!(e.mem.u32(this + 0x2c), 0);
            let order: Vec<u32> = e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .map(|(a, _)| *a)
                .filter(|a| {
                    [
                        SUB_BUFFER_DESTRUCTOR,
                        POINTER_DESTRUCTOR,
                        MAGIC_HIT_EFFECT_DESTRUCTOR,
                    ]
                    .contains(a)
                })
                .collect();
            assert_eq!(
                order,
                vec![
                    SUB_BUFFER_DESTRUCTOR,
                    POINTER_DESTRUCTOR,
                    POINTER_DESTRUCTOR,
                    MAGIC_HIT_EFFECT_DESTRUCTOR
                ]
            );
        }
        // Nothing held, an empty name: no detach, no release.
        let mut e = pointer_engine();
        let this = e.mem.alloc(0x3c);
        let name = e.mem.alloc(4);
        e.mem.set_u32(this + 0x2c, name);
        e.call(0x0081_c810, &[this]);
        assert!(called(&e, WORD_18_009611E0).is_empty());
        assert!(called(&e, RELEASE_MODEL).is_empty());
    }

    struct Init {
        e: Engine,
        this: u32,
        clone: u32,
    }

    /// An effect whose target answers `target_answer` from slot +0x1d0, a file name, a
    /// clone with a controller manager whose sequence lookup answers `sequence`, and the
    /// loaded model answering `model` (0: not loaded).
    fn init_world(target_answer: u32, model: u32, sequence: u32) -> Init {
        let mut e = pointer_engine();
        for page in [
            0x0101_2000,
            0x0101_7000,
            0x011d_e000,
            0x011f_3000,
            0x011f_6000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f64(THOUSAND, 1000.0);
        e.mem.set_f64(ZERO, 0.0);
        let target_vtable = e.mem.alloc(0x1d4);
        e.mem.set_u32(target_vtable + 0x1d0, 0x71d0);
        e.register_double(0x71d0, move |_, _| target_answer.into_ret());
        let target = e.mem.alloc(8);
        e.mem.set_u32(target, target_vtable);
        let vtable = e.mem.alloc(0xd0);
        e.mem.set_u32(vtable + 0xcc, 0x70cc);
        e.register(0x70cc, |_, _| Ret::default());
        let this = e.mem.alloc(0x3c);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x1c, target);
        let name = e.mem.alloc(4);
        e.mem.set_u8(name, b'm');
        e.mem.set_u32(this + 0x2c, name);
        let clone = e.mem.alloc(0x20);
        e.register_double(LOAD_FILE, move |_, _| model.into_ret());
        e.register_double(CLONE_3D, move |_, _| clone.into_ret());
        e.register(MODEL_ROOT_0043B230, |_, a| (a[0] + 1).into_ret());
        e.register(CONTROLLER_MANAGER_00653270, |_, a| {
            (a[1] + 0x100).into_ret()
        });
        e.register(STRING_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(STRING_DESTRUCTOR, |_, _| Ret::default());
        e.register_double(GET_SEQUENCE_0047A520, move |_, _| sequence.into_ret());
        e.register(PROCESS_SELECTIVE_UPDATE, |_, _| Ret::default());
        e.register(TIME_00825C00, |_, _| 2500u32.into_ret());
        e.register(UPDATE_BLOCK_0043D410, |_, _| Ret::default());
        e.register(UPDATE_00A59C60, |_, _| Ret::default());
        e.register(DEACTIVATE_ALL, |_, _| Ret::default());
        e.register(ACTIVATE_SEQUENCE_0047AAB0, |_, _| Ret::default());
        e.register(SET_FLAG_0047AA40, |_, _| Ret::default());
        e.register(SET_PHASE, |_, _| Ret::default());
        Init { e, this, clone }
    }

    #[test]
    fn init_attaches_the_clone_and_starts_the_sequence() {
        let mut w = init_world(1, 0, 0x5e9);
        let this = w.this;
        // pEffect with an elapsed time of 0.75 and flag 0x400 present.
        let effect = w.e.mem.alloc(8);
        w.e.mem.set_f32(effect + 4, 0.75);
        w.e.mem.set_u32(this + 0x18, effect);
        w.e.register(START_AGE_006B9130, |e, a| {
            f64::from(e.mem.f32(a[0] + 4)).into_ret()
        });
        w.e.register(OBJECT_007DF1D0, |_, _| 0x7df1u32.into_ret());
        w.e.register(HAS_FLAGS_00403E60, |_, _| 1u32.into_ret());
        assert!(w.e.call(0x0081_c950, &[this]).bool());
        assert_eq!(w.e.mem.f32(this + 0x20), 0.75);
        assert_eq!(w.e.mem.u8(this + 0x24), 0);
        assert_eq!(w.e.mem.u32(this + 0x30), w.clone);
        assert_eq!(called(&w.e, HAS_FLAGS_00403E60), vec![vec![0x7df1, 0x400]]);
        let block = called(&w.e, UPDATE_BLOCK_0043D410);
        assert_eq!(block[0][1..], [2.5f32.to_bits(), 1, 0]);
        assert_eq!(called(&w.e, UPDATE_00A59C60)[0], vec![w.clone, block[0][0]]);
        let manager = w.clone + 1 + 0x100;
        assert_eq!(called(&w.e, DEACTIVATE_ALL), vec![vec![manager, 0]]);
        assert_eq!(
            called(&w.e, ACTIVATE_SEQUENCE_0047AAB0),
            vec![vec![manager, 0x5e9, 0, 0, 1.0f32.to_bits(), 0, 0]]
        );
        assert_eq!(called(&w.e, SET_FLAG_0047AA40), vec![vec![manager, 1]]);
        assert_eq!(
            called(&w.e, SET_PHASE),
            vec![vec![0x5e9, 0.75f32.to_bits(), 0]]
        );
        assert_eq!(called(&w.e, RELEASE_MODEL)[0][2..], [0, 1]);
    }

    #[test]
    fn init_without_age_skips_the_phase_and_marks_finished() {
        let mut w = init_world(1, 0, 0x5e9);
        let this = w.this;
        let effect = w.e.mem.alloc(8);
        w.e.mem.set_u32(this + 0x18, effect);
        w.e.register(START_AGE_006B9130, |_, _| 0.0f64.into_ret());
        w.e.register(OBJECT_007DF1D0, |_, _| 0x7df1u32.into_ret());
        w.e.register(HAS_FLAGS_00403E60, |_, _| 0u32.into_ret());
        assert!(w.e.call(0x0081_c950, &[this]).bool());
        assert_eq!(w.e.mem.u8(this + 0x24), 1);
        assert!(called(&w.e, SET_PHASE).is_empty());
    }

    #[test]
    fn init_answers_false_early() {
        // The target answering 0.
        let mut w = init_world(0, 0, 0x5e9);
        assert!(!w.e.call(0x0081_c950, &[w.this]).bool());
        assert!(called(&w.e, LOAD_FILE).is_empty());
        // An empty file name.
        let mut w = init_world(1, 0, 0x5e9);
        let name = w.e.mem.u32(w.this + 0x2c);
        w.e.mem.set_u8(name, 0);
        assert!(!w.e.call(0x0081_c950, &[w.this]).bool());
        assert!(called(&w.e, LOAD_FILE).is_empty());
        // A loaded model with a controller manager but no hit-effect sequence.
        let mut w = init_world(1, 0x6000, 0);
        assert!(!w.e.call(0x0081_c950, &[w.this]).bool());
        assert!(called(&w.e, CLONE_3D).is_empty());
        // The clone's manager has no sequence.
        let mut w = init_world(1, 0, 0);
        assert!(!w.e.call(0x0081_c950, &[w.this]).bool());
        assert!(called(&w.e, DEACTIVATE_ALL).is_empty());
        assert_eq!(called(&w.e, 0x70cc).len(), 1);
    }

    #[test]
    fn init_records_third_person_for_the_player() {
        let mut w = init_world(1, 0, 0x5e9);
        let target = w.e.mem.u32(w.this + 0x1c);
        w.e.mem.set_u32(PLAYER, target);
        w.e.register(PLAYER_FLAG_004EAF60, |_, _| 1u32.into_ret());
        assert!(w.e.call(0x0081_c950, &[w.this]).bool());
        assert_eq!(w.e.mem.u8(w.this + 0x29), 1);
    }
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
    fn magic_model_hit_effect_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0081_c600, 0x0081_c810, 0x3c);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_0081c5f0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0081_c5f0, &args![0x1000u32]).u32(), 0x011d_c724);
        assert_eq!(e.call(0x0081_c5f0, &args![0u32]).u32(), 0x011d_c724);
    }
}

#[cfg(test)]
mod tests_n048 {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn zero(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    /// An object whose vtable sends each listed slot to `code + slot`, a
    /// double answering the paired word.
    fn object(e: &mut Engine, code: u32, slots: &[(u32, u32)]) -> u32 {
        let size = slots.iter().map(|s| s.0).max().unwrap_or(0) + 4;
        let vtable = e.mem.alloc(size);
        for &(slot, answer) in slots {
            e.mem.set_u32(vtable + slot, code + slot);
            e.register_double(code + slot, move |_, _| ret(answer));
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    const TARGET: u32 = 0x0300_0000;
    const EFFECT_CODE: u32 = 0x0310_0000;
    const SEQUENCE: u32 = 0x0320_0000;
    const NODE: u32 = 0x0330_0000;
    const CHILD: u32 = 0x0340_0000;
    const BUFFER: u32 = 0x0350_0000;
    const SETTING: u32 = 0x0360_0000;
    const PROCESS: u32 = 0x0370_0000;

    /// Pointers holding their object in their first word, the controller
    /// manager `manager` for any object, the save/load game object at
    /// 0x5a1d and the constants mapped.
    fn rig(e: &mut Engine, manager: u32) {
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(MODEL_ROOT_0043B230, |_, a| ret(a[0] + 1));
        e.register_double(CONTROLLER_MANAGER_00653270, move |_, _| ret(manager));
        e.map(SAVE_LOAD_GAME, 4);
        e.mem.set_u32(SAVE_LOAD_GAME, 0x5a1d);
        e.map(MODEL_LOADER, 4);
        e.mem.set_u32(MODEL_LOADER, 0x10ad);
        for (addr, bits) in [
            (MINUS_ONE, (-1.0f32).to_bits()),
            (LONGEST_LIFETIME, f32::MAX.to_bits()),
        ] {
            e.map(addr, 4);
            e.set_global::<u32>(addr, bits);
        }
        for (addr, value) in [(ZERO, 0.0f64), (TWO_DOUBLE, 2.0), (SIXTY_FOUR, 64.0)] {
            e.map(addr, 8);
            e.set_global::<f64>(addr, value);
        }
    }

    fn effect(e: &mut Engine, slots: &[(u32, u32)]) -> Ptr<MagicModelHitEffect> {
        let this = object(e, EFFECT_CODE, slots);
        Ptr::new(this)
    }

    #[test]
    fn second_constructor_keeps_a_lifetime() {
        let mut e = Engine::new();
        rig(&mut e, 0);
        zero(
            &mut e,
            &[
                MAGIC_HIT_EFFECT_CONSTRUCTOR,
                POINTER_CONSTRUCTOR,
                SUB_BUFFER_CONSTRUCTOR,
            ],
        );
        let this = e.new_object::<MagicModelHitEffect>();
        e.mem.write(this.addr(), &[0xffu8; 0x3c]);
        let (r, log) = run(
            &mut e,
            0x0081_c720,
            &args![this, 0x7a7au32, 0x4a4au32, 3.0f32.to_bits()],
        );
        assert_eq!(r.u32(), this.addr());
        assert_eq!(
            calls(&log, MAGIC_HIT_EFFECT_CONSTRUCTOR),
            vec![vec![this.addr(), 0x7a7a, 0]]
        );
        assert_eq!(e.mem.u32(this.addr()), MAGIC_MODEL_HIT_EFFECT_VTABLE);
        assert_eq!(e.get(this, MagicModelHitEffect::spObject), 0);
        assert_eq!(e.get(this, MagicModelHitEffect::spTarget), 0);
        assert_eq!(e.get(this, MagicModelHitEffect::b3rdPerson), 0);
        assert_eq!(e.get(this, MagicModelHitEffect::bOwnPath), 0);
        assert_eq!(e.get(this, MagicModelHitEffect::pFilename), 0x4a4a);
        assert_eq!(e.mem.u32(this.addr() + 8), 3.0f32.to_bits());
        assert_eq!(
            calls(&log, SUB_BUFFER_CONSTRUCTOR),
            vec![vec![this.addr() + 0x38]]
        );
        e.call(
            0x0081_c720,
            &args![this, 0x7a7au32, 0x4a4au32, (-1.0f32).to_bits()],
        );
        assert_eq!(e.mem.u32(this.addr() + 8), f32::MAX.to_bits());
        // NaN is not below 0.0: kept (quiet).
        e.call(
            0x0081_c720,
            &args![this, 0x7a7au32, 0x4a4au32, 0x7f80_0001u32],
        );
        assert_eq!(e.mem.u32(this.addr() + 8), 0x7fc0_0001);
    }

    /// An effect with a target whose slot +0x1D0 answers, `spObject` set,
    /// a manager with a sequence (`SEQUENCE` slots).
    fn update_world(e: &mut Engine, state: u32) -> (Ptr<MagicModelHitEffect>, u32) {
        let sequence = object(e, SEQUENCE, &[(0x8c, 0)]);
        rig(e, 0x3a3a);
        let target = object(e, TARGET, &[(0x1d0, 0x0d0d)]);
        let this = effect(e, &[(0xcc, 0)]);
        e.set(this, MagicModelHitEffect::pTarget, target);
        e.set(this, MagicModelHitEffect::spObject, 0x0b0b);
        e.register(MAGIC_HIT_EFFECT_UPDATE, |_, _| ret(1));
        e.register(STRING_CONSTRUCTOR, |_, a| ret(a[0]));
        zero(e, &[STRING_DESTRUCTOR]);
        e.register_double(GET_SEQUENCE_0047A520, move |_, _| ret(sequence));
        e.register(SEQUENCE_END_00508120, |_, _| st0(2.0));
        e.register(SEQUENCE_TIME_00508100, |_, _| st0(1.0));
        e.register(SEQUENCE_TEST_008041A0, |_, _| ret(1));
        e.register_double(SEQUENCE_STATE_0059BB30, move |_, _| ret(state));
        (this, sequence)
    }

    #[test]
    fn update_finishes_with_the_sequence() {
        let mut e = Engine::new();
        let (this, sequence) = update_world(&mut e, 2);
        e.set(this, MagicModelHitEffect::fAge, 1.5f32.to_bits());
        let (r, log) = run(&mut e, 0x0081_cc70, &args![this, 0.25f32.to_bits()]);
        assert_eq!(r.u32(), 1);
        assert_eq!(
            calls(&log, MAGIC_HIT_EFFECT_UPDATE),
            vec![vec![this.addr(), 0.25f32.to_bits()]]
        );
        assert_eq!(calls(&log, EFFECT_CODE + 0xcc), vec![vec![this.addr()]]);
        assert_eq!(e.get(this, MagicModelHitEffect::bFinished), 1);
        assert_eq!(calls(&log, SEQUENCE + 0x8c), vec![vec![sequence, 0, 0]]);
        assert_eq!(e.mem.u32(this.addr() + 8), 1.5f32.to_bits());
        // Past the lifetime plus 2: false.
        e.set(this, MagicModelHitEffect::fAge, 4.0f32.to_bits());
        e.mem.set_u32(this.addr() + 8, 1.5f32.to_bits());
        e.register(SEQUENCE_TIME_00508100, |_, _| st0(2.0));
        let (r, _) = run(&mut e, 0x0081_cc70, &args![this, 0.25f32.to_bits()]);
        assert_eq!(r.u32(), 0);
        e.set(this, MagicModelHitEffect::fAge, 3.5f32.to_bits());
        assert_eq!(
            e.call(0x0081_cc70, &args![this, 0.25f32.to_bits()]).u32(),
            1
        );
    }

    #[test]
    fn update_answers_false_early() {
        let mut e = Engine::new();
        let (this, _) = update_world(&mut e, 1);
        // State 1 and not finished: no change.
        let (r, log) = run(&mut e, 0x0081_cc70, &args![this, 0.25f32.to_bits()]);
        assert_eq!(r.u32(), 1);
        assert!(calls(&log, SEQUENCE + 0x8c).is_empty());
        e.register(MAGIC_HIT_EFFECT_UPDATE, |_, _| ret(0x100));
        assert_eq!(
            e.call(0x0081_cc70, &args![this, 0.25f32.to_bits()]).u32(),
            0
        );
        e.register(TARGET + 0x1d0, |_, _| ret(0));
        let (r, log) = run(&mut e, 0x0081_cc70, &args![this, 0.25f32.to_bits()]);
        assert_eq!(r.u32(), 0);
        assert!(calls(&log, MAGIC_HIT_EFFECT_UPDATE).is_empty());
    }

    /// A target (`TARGET` slots) answering the listed words, its 3D a node
    /// whose position is (1, 2, 3).
    fn position_world(
        e: &mut Engine,
        actor: u32,
        knocked: u32,
        lying: u32,
    ) -> (Ptr<MagicModelHitEffect>, u32, u32) {
        rig(e, 0);
        let node = object(e, NODE, &[(0xc, 0)]);
        let target = object(
            e,
            TARGET,
            &[
                (0x1d0, node),
                (0x100, actor),
                (0x22c, knocked),
                (0x230, lying),
            ],
        );
        let this = effect(e, &[]);
        e.set(this, MagicModelHitEffect::pTarget, target);
        e.set(this, MagicModelHitEffect::spObject, 0x0b0b);
        e.map(PLAYER, 4);
        zero(
            e,
            &[
                MAGIC_HIT_EFFECT_0081BC70,
                SET_TRANSLATE_00440460,
                SET_ROTATE_0043FA80,
            ],
        );
        e.register(SELF_006815C0, |_, a| ret(a[0]));
        let position = e.mem.alloc(12);
        for (i, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_u32(position + 4 * i as u32, value.to_bits());
        }
        e.register_double(POSITION_0045BB80, move |_, _| ret(position));
        e.register(ROTATION_006A9540, |_, a| ret(a[0] + 0x40));
        (this, target, node)
    }

    fn translated(e: &Engine, log: &Log) -> Vec<u32> {
        let point = calls(log, SET_TRANSLATE_00440460)[0][1];
        (0..3).map(|i| e.mem.u32(point + 4 * i)).collect()
    }

    #[test]
    fn update_position_lowers_a_lying_actor() {
        let mut e = Engine::new();
        let (this, _, node) = position_world(&mut e, 1, 1, 0);
        let (_, log) = run(&mut e, 0x0081_ce20, &args![this]);
        assert_eq!(
            translated(&e, &log),
            vec![1.0f32.to_bits(), 2.0f32.to_bits(), (-61.0f32).to_bits()]
        );
        assert_eq!(
            calls(&log, MAGIC_HIT_EFFECT_0081BC70),
            vec![vec![this.addr(), 0x0b0b]]
        );
        assert_eq!(
            calls(&log, SET_ROTATE_0043FA80),
            vec![vec![0x0b0b, node + 0x40]]
        );
        // Three levels of children whose test passes: not lowered.
        let second = object(&mut e, CHILD + 0x2000, &[]);
        let child = object(&mut e, CHILD, &[(0xc, second)]);
        e.register(NODE + 0xc, move |_, _| ret(0x6000));
        e.register(CHILD_COUNT_0043B480, |_, _| ret(1));
        e.register_double(CHILD_AT_0043B4A0, move |_, _| ret(child));
        e.register(NODE_DATA_0043C490, |_, a| ret(a[0] + 4));
        e.register(COMPARE_00439090, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0081_ce20, &args![this]);
        assert_eq!(translated(&e, &log)[2], 3.0f32.to_bits());
        assert_eq!(
            calls(&log, COMPARE_00439090),
            vec![vec![second + 4, COMPARE_ARGUMENT_011F426C]]
        );
    }

    #[test]
    fn update_position_uses_the_bound_for_type_0x1c() {
        let mut e = Engine::new();
        let (this, target, node) = position_world(&mut e, 0, 0, 0);
        e.register(BASE_FORM_007AF430, |_, a| ret(a[0] + 8));
        e.register(FORM_TYPE_00401170, |_, _| ret(0x1c));
        let bound = e.mem.alloc(12);
        for (i, value) in [7.0f32, 8.0, 9.0].iter().enumerate() {
            e.mem.set_u32(bound + 4 * i as u32, value.to_bits());
        }
        e.register_double(WORLD_BOUND, move |_, _| ret(bound));
        let (_, log) = run(&mut e, 0x0081_ce20, &args![this]);
        assert_eq!(calls(&log, FORM_TYPE_00401170), vec![vec![target + 8]]);
        assert_eq!(calls(&log, WORLD_BOUND), vec![vec![node]]);
        assert_eq!(
            translated(&e, &log),
            vec![7.0f32.to_bits(), 8.0f32.to_bits(), 3.0f32.to_bits()]
        );
        e.register(FORM_TYPE_00401170, |_, _| ret(0x1b));
        let (_, log) = run(&mut e, 0x0081_ce20, &args![this]);
        assert_eq!(
            translated(&e, &log),
            vec![1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
    }

    #[test]
    fn update_position_of_the_player_and_without_3d() {
        let mut e = Engine::new();
        let (this, target, _) = position_world(&mut e, 0, 0, 1);
        e.mem.set_u32(PLAYER, target);
        e.register(PLAYER_FLAG_004EAF60, |_, _| ret(1));
        let node = object(&mut e, NODE + 0x1000, &[(0xc, 0)]);
        e.register_double(PLAYER_NODE_00950BB0, move |_, _| ret(node));
        let (_, log) = run(&mut e, 0x0081_ce20, &args![this]);
        assert_eq!(e.get(this, MagicModelHitEffect::b3rdPerson), 1);
        assert_eq!(calls(&log, PLAYER_NODE_00950BB0), vec![vec![target, 0]]);
        assert_eq!(translated(&e, &log)[2], (-61.0f32).to_bits());
        e.register(PLAYER_NODE_00950BB0, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0081_ce20, &args![this]);
        assert_eq!(calls(&log, MAGIC_HIT_EFFECT_0081BC70).len(), 1);
        assert!(calls(&log, SET_TRANSLATE_00440460).is_empty());
    }

    #[test]
    fn save_size_and_save_with_a_controller_manager() {
        let mut e = Engine::new();
        rig(&mut e, 0x3a3a);
        let this = effect(&mut e, &[]);
        e.set(this, MagicModelHitEffect::spObject, 0x0b0b);
        e.register(MAGIC_HIT_EFFECT_GET_SAVE_SIZE, |_, _| ret(0x10));
        e.register(CONTROLLER_SAVE_SIZE, |_, _| ret(0x2_0020));
        let (r, log) = run(&mut e, 0x0081_d0b0, &args![this, 1u32, 2u32]);
        assert_eq!(r.u32() & 0xffff, 0x32);
        assert_eq!(
            calls(&log, MAGIC_HIT_EFFECT_GET_SAVE_SIZE),
            vec![vec![this.addr(), 1, 2]]
        );
        assert_eq!(
            calls(&log, CONTROLLER_MANAGER_00653270),
            vec![vec![LOOKUP_ARGUMENT_011F36AC, 0x0b0c]]
        );
        zero(&mut e, &[MAGIC_HIT_EFFECT_SAVE_GAME, CONTROLLER_SAVE]);
        e.register(SAVE_WRITE, |e, a| {
            assert_eq!(e.mem.u16(a[1]), 0x20);
            Ret::default()
        });
        // An actor reference with a middle-high process and animation.
        let reference = object(&mut e, TARGET, &[(0x100, 1)]);
        let process = object(&mut e, PROCESS, &[(0x1b8, 0xa1)]);
        e.register(PROCESS_OF, |_, a| ret(a[0] + 1));
        e.register_double(DYNAMIC_CAST, move |_, _| ret(process));
        e.register(ANIMATION_TIME_00453700, |_, _| st0(0.75));
        let (_, log) = run(&mut e, 0x0081_d150, &args![this, 0xefu32, reference]);
        assert_eq!(
            calls(&log, DYNAMIC_CAST),
            vec![vec![
                reference + 1,
                0,
                TYPE_BASE_PROCESS,
                TYPE_MIDDLE_HIGH_PROCESS,
                0
            ]]
        );
        assert_eq!(calls(&log, ANIMATION_TIME_00453700), vec![vec![0xa1]]);
        assert_eq!(
            calls(&log, CONTROLLER_SAVE),
            vec![vec![0x3a3a, 0.75f32.to_bits()]]
        );
        // Without an active effect the time stays -1.
        let (_, log) = run(&mut e, 0x0081_d150, &args![this, 0u32, reference]);
        assert_eq!(
            calls(&log, CONTROLLER_SAVE),
            vec![vec![0x3a3a, (-1.0f32).to_bits()]]
        );
        // No manager: a zero size and nothing more.
        e.register(CONTROLLER_MANAGER_00653270, |_, _| ret(0));
        e.register(SAVE_WRITE, |e, a| {
            assert_eq!(e.mem.u16(a[1]), 0);
            Ret::default()
        });
        let (_, log) = run(&mut e, 0x0081_d150, &args![this, 0xefu32, reference]);
        assert_eq!(calls(&log, SAVE_WRITE).len(), 1);
        assert!(calls(&log, CONTROLLER_SAVE).is_empty());
    }

    /// An active effect whose effect setting's model path is `path`.
    fn effect_setting(e: &mut Engine, path: u32) -> u32 {
        let setting = e.mem.alloc(0x20);
        let model = object(e, SETTING, &[(0x14, path)]);
        let vtable = e.mem.u32(model);
        e.mem.set_u32(setting + 0x18, vtable);
        e.register(OBJECT_ID_0084E3A0, |_, a| ret(a[0] + 0xc));
        e.register_double(WORD_14_00825C00, move |_, _| ret(setting));
        0xac7e
    }

    #[test]
    fn load_game_reads_the_name_block() {
        let mut e = Engine::new();
        rig(&mut e, 0);
        let this = effect(&mut e, &[]);
        let active = effect_setting(&mut e, 0x9a7);
        zero(
            &mut e,
            &[MAGIC_HIT_EFFECT_LOAD_GAME, QUEUE_MODEL, COPY_00401460],
        );
        let block = e.mem.alloc(0x10);
        e.register_double(OPERATOR_NEW, move |_, _| ret(block));
        e.register(LOAD_READ, |e, a| {
            if a[2] == 2 {
                e.mem.set_u16(a[1], 5);
            }
            Ret::default()
        });
        let (_, log) = run(&mut e, 0x0081_d2a0, &args![this, active, 9u32]);
        assert_eq!(
            calls(&log, QUEUE_MODEL),
            vec![vec![0x10ad, 0x9a7, 5, 0, 0, 1, 0, 0]]
        );
        assert_eq!(calls(&log, OPERATOR_NEW), vec![vec![7]]);
        let reads = calls(&log, LOAD_READ);
        assert_eq!(reads[1], vec![0x5a1d, block + 2, 5]);
        assert_eq!(calls(&log, COPY_00401460)[0][0], block);
        assert_eq!(e.get(this, MagicModelHitEffect::pFilename), block);
        // No path: nothing queued; a zero length: no block.
        let active = effect_setting(&mut e, 0);
        e.register(LOAD_READ, |_, _| Ret::default());
        e.set(this, MagicModelHitEffect::pFilename, 0);
        let (_, log) = run(&mut e, 0x0081_d2a0, &args![this, active, 9u32]);
        assert!(calls(&log, QUEUE_MODEL).is_empty());
        assert!(calls(&log, OPERATOR_NEW).is_empty());
        assert_eq!(e.get(this, MagicModelHitEffect::pFilename), 0);
    }

    #[test]
    fn finish_init_load_game_restores_the_controller_state() {
        let mut e = Engine::new();
        rig(&mut e, 0x3a3a);
        let this = effect(&mut e, &[(0xc4, 0), (0x94, 0)]);
        e.register(EFFECT_CODE + 0xc4, |e, a| {
            e.mem.set_u32(a[0] + 0x20, 0);
            e.mem.set_u8(a[0] + 0x24, 0);
            Ret::default()
        });
        e.set(this, MagicModelHitEffect::spObject, 0x0b0b);
        e.set(this, MagicModelHitEffect::pFilename, 0x01d0);
        e.set(this, MagicModelHitEffect::fAge, 2.5f32.to_bits());
        e.set(this, MagicModelHitEffect::bFinished, 1);
        zero(
            &mut e,
            &[
                BASE_FINISH_INIT_LOAD_GAME,
                SET_POSITION_00600AB0,
                LOAD_READ,
                CONTROLLER_LOAD,
                FREE,
            ],
        );
        e.register(WORD_14_00825C00, |_, a| {
            ret(if a[0] == 0x5a1d { 0x9051 } else { 0 })
        });
        let (_, log) = run(&mut e, 0x0081_d3b0, &args![this, 0u32, 0u32, 0x4a4au32]);
        assert_eq!(
            calls(&log, BASE_FINISH_INIT_LOAD_GAME),
            vec![vec![this.addr(), 0, 0, 0]]
        );
        assert_eq!(e.get(this, MagicModelHitEffect::pFilename), 0x4a4a);
        assert_eq!(e.get(this, MagicModelHitEffect::fAge), 2.5f32.to_bits());
        assert_eq!(e.get(this, MagicModelHitEffect::bFinished), 1);
        assert_eq!(
            calls(&log, SET_POSITION_00600AB0),
            vec![vec![0x5a1d, 0x01d0], vec![0x5a1d, 0x9051]]
        );
        assert_eq!(calls(&log, LOAD_READ)[0][2], 2);
        assert_eq!(
            calls(&log, CONTROLLER_LOAD),
            vec![vec![0x3a3a, 0x0b0b, (-1.0f32).to_bits()]]
        );
        assert_eq!(calls(&log, FREE), vec![vec![0x01d0]]);
        assert_eq!(calls(&log, EFFECT_CODE + 0x94), vec![vec![this.addr(), 0]]);
        // With an active effect the name is its model path; no manager, no load.
        let active = effect_setting(&mut e, 0x9a7);
        e.register(CONTROLLER_MANAGER_00653270, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0081_d3b0, &args![this, active, 0u32, 0x4a4au32]);
        assert_eq!(e.get(this, MagicModelHitEffect::pFilename), 0x9a7);
        assert!(calls(&log, CONTROLLER_LOAD).is_empty());
    }

    #[test]
    fn check_should_save() {
        let mut e = Engine::new();
        let this = e.new_object::<MagicModelHitEffect>();
        assert_eq!(e.call(0x0081_d5a0, &args![this]).u32(), 0);
        e.set(this, MagicModelHitEffect::pTarget, 1);
        e.set(this, MagicModelHitEffect::pFilename, 2);
        assert_eq!(e.call(0x0081_d5a0, &args![this]).u32(), 1);
        e.mem.set_u32(this.addr() + 0x18, 3);
        assert_eq!(e.call(0x0081_d5a0, &args![this]).u32(), 0);
    }

    #[test]
    fn second_save_size_save_and_load() {
        let mut e = Engine::new();
        rig(&mut e, 0);
        let this = effect(
            &mut e,
            &[(0xd0, 0x30), (0xd4, 0), (0xd8, 0), (0xdc, 0), (0xe0, 0)],
        );
        e.set(this, MagicModelHitEffect::pTarget, 0x7a7a);
        e.set(this, MagicModelHitEffect::pFilename, 0x4a4a);
        e.register(STRING_LENGTH, |_, _| ret(0x107));
        let (r, log) = run(&mut e, 0x0081_d5d0, &args![this]);
        assert_eq!(r.u32() & 0xffff, 5 + 0x107 + 0x30);
        assert_eq!(
            calls(&log, EFFECT_CODE + 0xd0),
            vec![vec![this.addr(), 0, 0x7a7a]]
        );
        e.register(OBJECT_ID_0084E3A0, |_, _| ret(0x00ab_cdef));
        e.register(SAVE_NUMERIC_ID, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x00ab_cdef);
            Ret::default()
        });
        e.register(SAVE_WRITE, |e, a| {
            if a[2] == 1 {
                assert_eq!(e.mem.u8(a[1]), 7);
            }
            Ret::default()
        });
        let (_, log) = run(&mut e, 0x0081_d640, &args![this]);
        assert_eq!(calls(&log, SAVE_WRITE)[1], vec![0x5a1d, 0x4a4a, 7]);
        assert_eq!(
            calls(&log, EFFECT_CODE + 0xd4),
            vec![vec![this.addr(), 0, 0x7a7a]]
        );
        // Load: the id's form is a reference in a cell that passes.
        e.register(LOAD_NUMERIC_ID, |e, a| {
            e.mem.set_u32(a[1], 0x0001_0203);
            Ret::default()
        });
        e.register(LOOKUP_FORM_004839C0, |_, a| ret(a[0] + 1));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0] + 1));
        e.register(REFERENCE_CELL_008D6F30, |_, _| ret(0xce11));
        e.register(CELL_TEST_00450FF0, |_, _| ret(1));
        e.register(LOAD_READ, |e, a| {
            if a[2] == 1 {
                e.mem.set_u8(a[1], 4);
            }
            Ret::default()
        });
        e.register(OPERATOR_NEW, |_, _| ret(0x8a8e));
        zero(&mut e, &[FILL_00403D30]);
        let (r, log) = run(&mut e, 0x0081_d6c0, &args![this]);
        assert_eq!(r.u32(), 1);
        assert_eq!(e.get(this, MagicModelHitEffect::pTarget), 0x0001_0205);
        assert_eq!(
            calls(&log, DYNAMIC_CAST),
            vec![vec![0x0001_0204, 0, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0]]
        );
        assert_eq!(calls(&log, OPERATOR_NEW), vec![vec![5]]);
        assert_eq!(calls(&log, FILL_00403D30), vec![vec![0x8a8e, 0, 5]]);
        assert_eq!(calls(&log, LOAD_READ)[1], vec![0x5a1d, 0x8a8e, 4]);
        assert_eq!(e.get(this, MagicModelHitEffect::bOwnPath), 1);
        let target = 0x0001_0205;
        assert_eq!(
            calls(&log, EFFECT_CODE + 0xd8),
            vec![vec![this.addr(), 0, target]]
        );
        assert_eq!(
            calls(&log, EFFECT_CODE + 0xdc),
            vec![vec![this.addr(), 0, target]]
        );
        assert_eq!(
            calls(&log, EFFECT_CODE + 0xe0),
            vec![vec![this.addr(), 0, target, 0x8a8e]]
        );
        e.register(CELL_TEST_00450FF0, |_, _| ret(0));
        let (r, log) = run(&mut e, 0x0081_d6c0, &args![this]);
        assert_eq!(r.u32(), 0);
        assert_eq!(calls(&log, EFFECT_CODE + 0xd8).len(), 1);
        assert!(calls(&log, EFFECT_CODE + 0xdc).is_empty());
        e.register(DYNAMIC_CAST, |_, _| ret(0));
        let (r, log) = run(&mut e, 0x0081_d6c0, &args![this]);
        assert_eq!(r.u32(), 0);
        assert!(calls(&log, REFERENCE_CELL_008D6F30).is_empty());
    }

    #[test]
    fn third_save_and_load() {
        let mut e = Engine::new();
        rig(&mut e, 0x3a3a);
        let this = effect(&mut e, &[]);
        e.set(this, MagicModelHitEffect::spObject, 0x0b0b);
        e.set(this, MagicModelHitEffect::pFilename, 0x4a4a);
        let owner = object(&mut e, PROCESS, &[(0x1e4, 0)]);
        let buffer = object(&mut e, BUFFER, &[(8, owner)]);
        zero(
            &mut e,
            &[
                MAGIC_HIT_EFFECT_SAVE_GAME_OV2,
                SAVE_STRING,
                SAVE_CONTROLLER_MANAGER,
                SAVE_VARIABLE_SIZED_VALUE,
            ],
        );
        e.register(START_VARIABLE_SIZED_VALUE, |_, _| ret(0x57a7));
        let mut position = 0x100;
        e.register_double(OBJECT_ID_0084E3A0, move |_, _| {
            position += 0x40;
            ret(position)
        });
        let (_, log) = run(&mut e, 0x0081_d800, &args![this, buffer]);
        assert_eq!(
            calls(&log, MAGIC_HIT_EFFECT_SAVE_GAME_OV2),
            vec![vec![this.addr(), buffer]]
        );
        assert_eq!(calls(&log, SAVE_STRING), vec![vec![buffer, 0x4a4a, 0]]);
        assert_eq!(calls(&log, BUFFER + 8), vec![vec![buffer]]);
        assert_eq!(
            calls(&log, SAVE_CONTROLLER_MANAGER),
            vec![vec![0x3a3a, buffer, (-1.0f32).to_bits()]]
        );
        assert_eq!(
            calls(&log, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![buffer, 0x40, 0x57a7]]
        );
        // Load.
        zero(
            &mut e,
            &[
                MAGIC_HIT_EFFECT_LOAD_GAME_OV2,
                SUB_BUFFER_LOAD_GAME,
                QUEUE_MODEL,
            ],
        );
        e.register(LOAD_STRING, |_, _| ret(0x5717));
        let active = effect_setting(&mut e, 0x9a7);
        e.mem.set_u32(this.addr() + 0x18, active);
        let (_, log) = run(&mut e, 0x0081_d920, &args![this, buffer]);
        assert_eq!(calls(&log, LOAD_STRING), vec![vec![buffer, 0]]);
        assert_eq!(e.get(this, MagicModelHitEffect::pFilename), 0x5717);
        assert_eq!(e.get(this, MagicModelHitEffect::bOwnPath), 1);
        assert_eq!(
            calls(&log, SUB_BUFFER_LOAD_GAME),
            vec![vec![this.addr() + 0x38, buffer]]
        );
        assert_eq!(calls(&log, QUEUE_MODEL).len(), 1);
        e.mem.set_u32(this.addr() + 0x18, 0);
        let (_, log) = run(&mut e, 0x0081_d920, &args![this, buffer]);
        assert!(calls(&log, QUEUE_MODEL).is_empty());
    }

    #[test]
    fn finish_load_game_loads_the_controller_state() {
        let mut e = Engine::new();
        rig(&mut e, 0x3a3a);
        let this = effect(&mut e, &[(0xc4, 0), (0x94, 0)]);
        e.set(this, MagicModelHitEffect::spObject, 0x0b0b);
        e.set(this, MagicModelHitEffect::LoadedDataSubBuffer, 0xda7a);
        let biped = e.mem.alloc(4);
        let owner = object(&mut e, PROCESS, &[(0x1e4, biped)]);
        let buffer = object(&mut e, BUFFER, &[(0xc, owner)]);
        zero(
            &mut e,
            &[
                BASE_FINISH_LOAD_GAME,
                CONTROLLER_LOAD_BUFFER,
                LOAD_FORM_BUFFER_0081DB60,
                SUB_BUFFER_DESTRUCTOR,
            ],
        );
        e.register(ANIMATION_TIME_00453700, |_, _| st0(1.25));
        e.register(CREATE_LOAD_FORM_BUFFER, |_, _| ret(0xf0b0));
        let (_, log) = run(&mut e, 0x0081_d9f0, &args![this, buffer]);
        assert_eq!(
            calls(&log, BASE_FINISH_LOAD_GAME),
            vec![vec![this.addr(), buffer]]
        );
        assert_eq!(calls(&log, EFFECT_CODE + 0xc4), vec![vec![this.addr()]]);
        assert_eq!(calls(&log, ANIMATION_TIME_00453700), vec![vec![biped]]);
        assert_eq!(
            calls(&log, CREATE_LOAD_FORM_BUFFER),
            vec![vec![this.addr() + 0x38, 0]]
        );
        assert_eq!(
            calls(&log, CONTROLLER_LOAD_BUFFER),
            vec![vec![0x3a3a, 0xf0b0, 1.25f32.to_bits()]]
        );
        assert_eq!(
            calls(&log, LOAD_FORM_BUFFER_0081DB60),
            vec![vec![0xf0b0, 1]]
        );
        assert_eq!(
            calls(&log, SUB_BUFFER_DESTRUCTOR),
            vec![vec![this.addr() + 0x38]]
        );
        assert_eq!(calls(&log, EFFECT_CODE + 0x94), vec![vec![this.addr(), 0]]);
        // No form buffer: no release; an empty sub-buffer: no load.
        e.register(CREATE_LOAD_FORM_BUFFER, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0081_d9f0, &args![this, buffer]);
        assert!(calls(&log, LOAD_FORM_BUFFER_0081DB60).is_empty());
        e.set(this, MagicModelHitEffect::LoadedDataSubBuffer, 0);
        let (_, log) = run(&mut e, 0x0081_d9f0, &args![this, buffer]);
        assert!(calls(&log, CONTROLLER_LOAD_BUFFER).is_empty());
        assert_eq!(calls(&log, SUB_BUFFER_DESTRUCTOR).len(), 1);
    }
}
