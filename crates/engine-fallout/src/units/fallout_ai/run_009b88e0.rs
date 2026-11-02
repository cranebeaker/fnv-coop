//! `fallout/ai/run_009b88e0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Callees: a flag test `004fd420` (`thiscall`, `(mask)`, answers a byte), the list at
/// `this + 0x88` (`008256d0` "empty", `006815c0` its first item; `thiscall`, no arguments),
/// `004181e0` (`thiscall`, no arguments, answers an object), the pointer conversion
/// `004ae750` (`thiscall` on the pointer, no arguments; the map's name for it is a folded
/// one), `00444d00` (`thiscall`, `(bit)`, answers a byte), and the others named where used.
const TEST_FLAGS_004FD420: u32 = 0x004f_d420;
const LIST_EMPTY_008256D0: u32 = 0x0082_56d0;
const LIST_FIRST_006815C0: u32 = 0x0068_15c0;
const OBJECT_004181E0: u32 = 0x0041_81e0;
const CONVERT_004AE750: u32 = 0x004a_e750;
const TEST_BIT_00444D00: u32 = 0x0044_4d00;
/// `fn_009b8900`'s callees: `0043b540` and `0043b4d0` (`thiscall`, no arguments), `008c71b0`
/// (`thiscall` on a local, one word, answers the local), the test `009c2c80` (`cdecl`, four
/// words) and `Projectile::AddImpact` (`009bf5a0`, `thiscall`, six words).
const KIND_SOURCE_0043B540: u32 = 0x0043_b540;
const KIND_LOCAL_008C71B0: u32 = 0x008c_71b0;
const KIND_OF_0043B4D0: u32 = 0x0043_b4d0;
const IMPACT_TEST_009C2C80: u32 = 0x009c_2c80;
const PROJECTILE_ADD_IMPACT: u32 = 0x009b_f5a0;
/// `fn_009b8a20`'s callees: `009b7c90` (`thiscall`, no arguments, answers a byte) and
/// `009d9f00` (`thiscall`, no arguments, answers a word).
const OBJECT_TEST_009B7C90: u32 = 0x009b_7c90;
const TEST_009D9F00: u32 = 0x009d_9f00;
/// `fn_009b8b10`'s callees: the floats (ST0) `00979260`, `009669c0`, `00644660` and
/// `00508070` (`thiscall`, no arguments), `00408840` (`cdecl`, `(float)`, ST0),
/// `Projectile::ProcessImpacts` (`009c1b70`, answers a byte), the object at `011f2250` and
/// its `00441110` / `007af430` getters and `009c8b90` / `009c8c40` setters (`(0)`), and the
/// four per-state updates `009bc8f0`, `009b8c90`, `009b90b0`, `009ba660` (`thiscall`, no
/// arguments). The `double` constants: 1/30 as a float (`0108fdd0`) and 0 (`01012060`).
const VALUE_00979260: u32 = 0x0097_9260;
const VALUE_009669C0: u32 = 0x0096_69c0;
const VALUE_00644660: u32 = 0x0064_4660;
const VALUE_00508070: u32 = 0x0050_8070;
const FLOAT_OF_00408840: u32 = 0x0040_8840;
const PROJECTILE_PROCESS_IMPACTS: u32 = 0x009c_1b70;
const HOLDER_011F2250: u32 = 0x011f_2250;
const HOLDER_GET_00441110: u32 = 0x0044_1110;
const HOLDER_GET_007AF430: u32 = 0x007a_f430;
const HOLDER_SET_009C8B90: u32 = 0x009c_8b90;
const HOLDER_SET_009C8C40: u32 = 0x009c_8c40;
const STATE_1_UPDATE_009BC8F0: u32 = 0x009b_c8f0;
const STATE_2_UPDATE_009B8C90: u32 = 0x009b_8c90;
const STATE_3_UPDATE_009B90B0: u32 = 0x009b_90b0;
const STATE_4_UPDATE_009BA660: u32 = 0x009b_a660;
const ONE_THIRTIETH: u32 = 0x0108_fdd0;
const ZERO: u32 = 0x0101_2060;
/// The state word at +0x150.
const STATE: u32 = 0x150;

// Translated from 009b88e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, no arguments): `004fd420(0x800)` on `this`; answers what
/// it leaves in EAX.
pub fn fn_009b88e0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(TEST_FLAGS_004FD420, &args![this, 0x800u32]).u32()
}

// Translated from 009ba350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(pointer)`): the word at `this + 0xC` becomes
/// `004ae750` on `pointer`, or 0 for a null pointer.
pub fn fn_009ba350(e: &mut Engine, this: Ptr, pointer: u32) {
    let value = if pointer == 0 {
        0
    } else {
        e.call(CONVERT_004AE750, &args![pointer]).u32()
    };
    e.mem.set_u32(this.addr() + 0xc, value);
}

// Translated from 009ba390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(pointer)`): the word at `this + 0x10` becomes
/// `004ae750` on `pointer`, or 0 for a null pointer.
pub fn fn_009ba390(e: &mut Engine, this: Ptr, pointer: u32) {
    let value = if pointer == 0 {
        0
    } else {
        e.call(CONVERT_004AE750, &args![pointer]).u32()
    };
    e.mem.set_u32(this.addr() + 0x10, value);
}

// Translated from 009ba320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(first, second)`; the decompiler's library match
/// `CRect::MoveToXY` is wrong): `fn_009ba350(first)` and `fn_009ba390(second)`.
pub fn fn_009ba320(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    fn_009ba350(e, this, first);
    fn_009ba390(e, this, second);
}

// Translated from 009b8a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, no arguments): the state at +0x150 moves on: 3 becomes
/// 4; otherwise, when the list at +0x88 is not empty, it becomes 1, then 3 when
/// `009b7c90` answers true for the `004181e0` object, then 0 when bit 1 is set and bit 2 is
/// not (`00444d00`); when bit 1 is not set it becomes 2 if the object's `004fd420(0x10)`
/// answers true or `009d9f00` answers non-zero.
pub fn fn_009b8a20(e: &mut Engine, this: Ptr) {
    let state = this.addr() + STATE;
    if e.mem.u32(state) == 3 {
        e.mem.set_u32(state, 4);
        return;
    }
    if e.call(LIST_EMPTY_008256D0, &args![this.addr() + 0x88])
        .bool()
    {
        return;
    }
    e.mem.set_u32(state, 1);
    let object = e.call(OBJECT_004181E0, &args![this]).u32();
    if e.call(OBJECT_TEST_009B7C90, &args![object]).bool() {
        e.mem.set_u32(state, 3);
    }
    if e.call(TEST_BIT_00444D00, &args![this, 1u32]).bool()
        && !e.call(TEST_BIT_00444D00, &args![this, 2u32]).bool()
    {
        e.mem.set_u32(state, 0);
    }
    if !e.call(TEST_BIT_00444D00, &args![this, 1u32]).bool() {
        let object = e.call(OBJECT_004181E0, &args![this]).u32();
        if e.call(TEST_FLAGS_004FD420, &args![object, 0x10u32]).bool()
            || e.call(TEST_009D9F00, &args![this]).u32() != 0
        {
            e.mem.set_u32(state, 2);
        }
    }
}

// Translated from 009b8900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, six words): the impact is allowed when the list at +0x88
/// is empty or its first item's byte at +0x28 is set. The fourth word's kind (via
/// `0043b540`, `008c71b0` on a local and `0043b4d0`; 0 without it) is read. When allowed and
/// `009c2c80(first, fourth, this, 004181e0 object)` answers true, and the state (+0x150) is
/// not 3 or the kind is 1, 2, 9, 0x11, 0xD or 0x14, `Projectile::AddImpact` with the six
/// words; when that answers true the state moves on (`fn_009b8a20`) and the answer is true.
/// Otherwise false.
pub fn fn_009b8900(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
) -> bool {
    let list = this.addr() + 0x88;
    let allowed = e.call(LIST_EMPTY_008256D0, &args![list]).bool() || {
        let item = e.call(LIST_FIRST_006815C0, &args![list]).u32();
        let record = e.mem.u32(item);
        e.mem.u8(record.wrapping_add(0x28)) != 0
    };
    let kind = if fourth != 0 {
        let source = e.call(KIND_SOURCE_0043B540, &args![fourth]).u32();
        e.with_stack(4, |e, local| {
            let filled = e.call(KIND_LOCAL_008C71B0, &args![local, source]).u32();
            e.call(KIND_OF_0043B4D0, &args![filled]).u32()
        })
    } else {
        0
    };
    if !allowed {
        return false;
    }
    let object = e.call(OBJECT_004181E0, &args![this]).u32();
    if !e
        .call(IMPACT_TEST_009C2C80, &args![first, fourth, this, object])
        .bool()
    {
        return false;
    }
    let state = e.mem.u32(this.addr() + STATE);
    if state == 3 && ![1u32, 2, 9, 0x11, 0xd, 0x14].contains(&kind) {
        return false;
    }
    if !e
        .call(
            PROJECTILE_ADD_IMPACT,
            &args![this, first, second, third, fourth, fifth, sixth],
        )
        .bool()
    {
        return false;
    }
    fn_009b8a20(e, this);
    true
}

// Translated from 009b8b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, no arguments): when the byte at +0x90 is clear and the
/// list at +0x88 is not empty, the impacts are processed (`Projectile::ProcessImpacts`,
/// its byte the answer) unless bit 1 (`00444d00`) is set and both the float `00644660` is
/// above 0 (or NaN) and `00979260 - 00408840(009669c0 * 1/30)` (as a float) is not below
/// `00508070`. When processed: the holder at `011f2250` drops this object from its two
/// slots it holds it in (`009c8b90(0)`, `009c8c40(0)`), and the state (+0x150) 1 to 4 runs
/// its update (`009bc8f0`, `009b8c90`, `009b90b0`, `009ba660`). Answers whether processed.
pub fn fn_009b8b10(e: &mut Engine, this: Ptr) -> bool {
    let mut processed = false;
    if e.mem.u8(this.addr() + 0x90) == 0
        && !e
            .call(LIST_EMPTY_008256D0, &args![this.addr() + 0x88])
            .bool()
    {
        let mut go = true;
        if e.call(TEST_BIT_00444D00, &args![this, 1u32]).bool() {
            let total = e.call(VALUE_00979260, &args![this]).f64();
            let rate = e.call(VALUE_009669C0, &args![this]).f64();
            let scale = e.global::<f64>(ONE_THIRTIETH);
            let scaled = x87_quiet_bits(x87_store32(x87_mul(rate, scale)));
            let step = e.call(FLOAT_OF_00408840, &args![scaled]).f64();
            let left = x87_store32(x87_sub(total, step));
            let limit = e.call(VALUE_00644660, &args![this]).f64();
            go = if limit <= e.global::<f64>(ZERO) {
                true
            } else {
                let bound = e.call(VALUE_00508070, &args![this]).f64();
                x87_load32(left) < bound
            };
        }
        if go {
            processed = e.call(PROJECTILE_PROCESS_IMPACTS, &args![this]).bool();
        }
    }
    if processed {
        if e.call(HOLDER_GET_00441110, &args![HOLDER_011F2250]).u32() == this.addr() {
            e.call(HOLDER_SET_009C8B90, &args![HOLDER_011F2250, 0u32]);
        }
        if e.call(HOLDER_GET_007AF430, &args![HOLDER_011F2250]).u32() == this.addr() {
            e.call(HOLDER_SET_009C8C40, &args![HOLDER_011F2250, 0u32]);
        }
        let update = match e.mem.u32(this.addr() + STATE).wrapping_sub(1) {
            0 => Some(STATE_1_UPDATE_009BC8F0),
            1 => Some(STATE_2_UPDATE_009B8C90),
            2 => Some(STATE_3_UPDATE_009B90B0),
            3 => Some(STATE_4_UPDATE_009BA660),
            _ => None,
        };
        if let Some(update) = update {
            e.call(update, &args![this]);
        }
    }
    processed
}

/// `005de0a0` (unnamed): `thiscall`, two words (`RET 8`): a flag mask and whether to set
/// it.
const SET_FLAG: u32 = 0x005d_e0a0;
/// `0047c850` (unnamed): `thiscall` on the object at `0x011de45c`; a `bool`.
const PAUSE_OWNER: u32 = 0x011d_e45c;
const IS_PAUSED: u32 = 0x0047_c850;
/// `FindFirstCollisionObject` (`004b5260`, Xbox PDB): `cdecl`, one word.
const FIND_FIRST_COLLISION_OBJECT: u32 = 0x004b_5260;
/// `00573ed0` (unnamed, `tesobjectrefr.cpp`): `thiscall`, one word (`RET 4`).
const COLLISION_SET_00573ED0: u32 = 0x0057_3ed0;
/// `00440460` / `0043fa80` / `00440490` (unnamed, `modelloader.cpp`): `thiscall`, one
/// word (`RET 4`): set a node's local translation (a point), rotation (a matrix) and scale
/// (a float).
const SET_LOCAL_TRANSLATE: u32 = 0x0044_0460;
const SET_LOCAL_ROTATE: u32 = 0x0043_fa80;
const SET_LOCAL_SCALE: u32 = 0x0044_0490;
/// `0043d410` (unnamed, `modelloader.cpp`): `thiscall`, three words (`RET 0xc`);
/// constructs a point.
const POINT_CONSTRUCT: u32 = 0x0043_d410;
/// `00a59c60` (unnamed): `thiscall`, one word (`RET 4`).
const NODE_SET_POINT: u32 = 0x00a5_9c60;
/// `bhkWorld::SetMotion` (`00c6a350`), `bhkWorld::UpdatePosition` (`00c69f50`), Xbox PDB,
/// and `00c6a3d0`, `00c6a0b0` (unnamed, `bhkworld.obj`): `cdecl`, five, three, four and
/// four words.
const SET_MOTION: u32 = 0x00c6_a350;
const UPDATE_POSITION: u32 = 0x00c6_9f50;
const SET_COLLISION_KEY: u32 = 0x00c6_a3d0;
const SET_COLLISION_00C6A0B0: u32 = 0x00c6_a0b0;
/// `005dbe10` (unnamed): no words; the next key of a counter at `0x011b0808`.
const NEXT_KEY: u32 = 0x005d_be10;
/// The point `0x011f426c` (three words) the code copies as a zero point.
const ZERO_POINT: u32 = 0x011f_426c;
/// `009b7010` (unnamed, `hitdata.cpp`): `thiscall`, one word (`RET 4`): an out point; the
/// projectile's direction.
const HIT_DIRECTION: u32 = 0x009b_7010;
/// `004a3760` (unnamed, `bgsdecalmanager.cpp`): `cdecl` (out point, float, point): the
/// point times the float.
const FLOAT_TIMES_POINT: u32 = 0x004a_3760;
/// `-1.0` (`float`, `0x01012054`) and `2.0` (`float`, `0x010162c0`).
const MINUS_ONE: u32 = 0x0101_2054;
const TWO_FLOAT: u32 = 0x0101_62c0;
/// `00525340` (unnamed, `tesobjectweap.cpp`): `cdecl`, one word; normalizes the point.
const NORMALIZE: u32 = 0x0052_5340;
/// `NiPoint3::Dot` (`004b6190`, Xbox PDB): `thiscall`, one word (`RET 4`); `ST0`.
const DOT: u32 = 0x004b_6190;
/// `00439e90` / `00439ef0` / `0045bb20` (unnamed): `thiscall`, two words (`RET 8`): an out
/// point and a point (sum, difference) or a float (scaled).
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
const POINT_SCALED: u32 = 0x0045_bb20;
/// `00403e20` (unnamed, `effectitem.cpp`): `thiscall` on a `Setting`; the address of its
/// float. The `Setting`s at `0x011ce1dc` and `0x011ce1e8`.
const SETTING_FLOAT: u32 = 0x0040_3e20;
const SETTING_011CE1DC: u32 = 0x011c_e1dc;
const SETTING_011CE1E8: u32 = 0x011c_e1e8;
/// `0069ef80` (unnamed): `thiscall` on the shooter; a float in `ST0`.
const SHOOTER_SPEED: u32 = 0x0069_ef80;
/// `00439180` (unnamed): `thiscall`, one float (`RET 4`); scales the point.
const SCALE_BY: u32 = 0x0043_9180;
/// `TESHavokUtilities::AddVelocity` (`0062b8d0`, Xbox PDB) and `0062ba80` (unnamed, the
/// same unit): `cdecl`, three words.
const ADD_VELOCITY: u32 = 0x0062_b8d0;
const ADD_VELOCITY_0062BA80: u32 = 0x0062_ba80;
/// `00461130` (unnamed): `thiscall`, returns `this + 0x68` (the node's world transform).
const WORLD_TRANSFORM: u32 = 0x0046_1130;
/// `NiMatrix3::Inverse` (`004b46a0`, the map's `Inverse_ov2`): `thiscall`, one word
/// (`RET 4`): the out matrix.
const MATRIX_INVERSE: u32 = 0x004b_46a0;
/// `004b4500` (unnamed, `globalfunc.cpp`): `thiscall`, two words (`RET 8`): an out point
/// and a point; the matrix times the point.
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
/// `00408820` (unnamed, `effectsetting.cpp`): `cdecl`, one float; a float in `ST0`.
const CALLEE_00408820: u32 = 0x0040_8820;
/// `00631070` (unnamed, `teswaterlistener.cpp`): `cdecl`, one float; a float in `ST0`.
const CALLEE_00631070: u32 = 0x0063_1070;
/// `00416870` (unnamed, `extradatalist.cpp`): `thiscall`, three floats (`RET 0xc`);
/// constructs a point.
const POINT3_CONSTRUCT: u32 = 0x0041_6870;

/// `Projectile` limb-cut state (`fn_009b90b0`): the function-local `static float` at
/// `0x011f2018` and its guard bit (bit 0 of `0x011f201c`).
const STATIC_FLOAT: u32 = 0x011f_2018;
const STATIC_GUARD: u32 = 0x011f_201c;
/// `0042ce10` (unnamed, `extradatalist.cpp`): `thiscall` on the object at `0x011ddf38`; a
/// `bool`.
const VATS_OWNER: u32 = 0x011d_df38;
const VATS_TEST: u32 = 0x0042_ce10;
/// `00726070` (unnamed): `thiscall`; the list's next node (also a count of the ray hits).
const NODE_NEXT: u32 = 0x0072_6070;
/// `10.0` (`float`, `0x01017b78`), `512.0` (`0x010231a0`), `1024.0` (`0x010236e0`) and
/// `0.5` (`0x01016248`).
const TEN: u32 = 0x0101_7b78;
const RANGE_512: u32 = 0x0102_31a0;
const RANGE_1024: u32 = 0x0102_36e0;
const HALF: u32 = 0x0101_6248;
/// `TESObjectREFR::GetDismembered` (`00573090`, Xbox PDB): `thiscall`, one word (`RET 4`).
const GET_DISMEMBERED: u32 = 0x0057_3090;
/// `005d43c0` (unnamed): `thiscall`; the actor's extra-data list.
const EXTRA_LIST: u32 = 0x005d_43c0;
/// `ExtraDataList::GetDismembermentExtra` (`0042e8c0`, Xbox PDB): `thiscall`.
const DISMEMBER_EXTRA: u32 = 0x0042_e8c0;
/// `0044ddc0` / `00877a30` (unnamed): an array's count (`+8`) and the address of element
/// `index` (`RET 4`).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// The table of limb node names (`0x01196d6c`, one word per part).
const LIMB_NAMES: u32 = 0x0119_6d6c;
/// `00438170` / `004381b0` (unnamed, `extradataobjects.cpp`): construct (one word, `RET 4`)
/// and destroy (`ECX` only) the fixed-string local.
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
/// `008d01e0` (unnamed): `thiscall`; a float in `ST0` (the limb's scale).
const NODE_SCALE: u32 = 0x008d_01e0;
/// `004ad050` / `004ad240` (unnamed, `bipedanim.cpp`): `thiscall`, one float (`RET 4`);
/// construct and set the cloning-process local. `004ad270` destroys it
/// (`NiCloningProcess::~NiCloningProcess`, engine map).
const CLONING_CONSTRUCT: u32 = 0x004a_d050;
const CLONING_SET: u32 = 0x004a_d240;
const CLONING_DESTRUCT: u32 = 0x004a_d270;
/// `NiObject::Clone` (`00a5d2c0`, Xbox PDB): `thiscall`, one word (`RET 4`).
const CLONE: u32 = 0x00a5_d2c0;
/// `008b6df0` (unnamed, `actor.cpp`): `thiscall`, one word (`RET 4`).
const ACTOR_HIDE_LIMB: u32 = 0x008b_6df0;
/// `00450f90` (unnamed, `tes.cpp`): `thiscall`, one word (`RET 4`).
const NODE_SET_00450F90: u32 = 0x0045_0f90;
/// `0043b480` / `0043b4a0` / `0045bc00` (unnamed): a node's child count, child `index`
/// (`RET 4`) and whether child `index` is set (`RET 4`).
const CHILD_COUNT: u32 = 0x0043_b480;
const CHILD_AT: u32 = 0x0043_b4a0;
const CHILD_PRESENT: u32 = 0x0045_bc00;
/// `Animation::FindSkinnedNode` (`004910d0`) and `BipedAnim::AttachToSkeleton`
/// (`004ade40`), engine map: `cdecl`, one and four words.
const FIND_SKINNED_NODE: u32 = 0x0049_10d0;
const ATTACH_TO_SKELETON: u32 = 0x004a_de40;
/// `008d6f30` / `004543c0` (unnamed): an actor's cell and the cell's world (havok) object.
const PARENT_CELL: u32 = 0x008d_6f30;
const CELL_WORLD: u32 = 0x0045_43c0;
/// The identity rotation (`0x011a9448`) and the pivot point (`0x011a9478`) the code uses.
const IDENTITY_ROTATION: u32 = 0x011a_9448;
const PIVOT_POINT: u32 = 0x011a_9478;
/// `TESObjectREFR::SetLocationOnReference` (`00575830`, Xbox PDB, `RET 4`) and
/// `Projectile::Sync3DWithReference` (`009bf470`, Xbox PDB, `RET 8`).
const SET_LOCATION_ON_REFERENCE: u32 = 0x0057_5830;
const SYNC_3D: u32 = 0x009b_f470;
/// `00476a80` (unnamed): constructs a 0x34-byte transform local (`ECX` only).
const TRANSFORM_CONSTRUCT: u32 = 0x0047_6a80;
/// `008b42e0` (unnamed, `actor.cpp`): `thiscall`, one word (`RET 4`): the inverse into
/// the out transform.
const TRANSFORM_INVERT: u32 = 0x008b_42e0;
/// `0062c250` (unnamed, `teshavokutilities.cpp`): `thiscall`, two words (`RET 8`): an out
/// transform and another; the product.
const TRANSFORM_MULTIPLY: u32 = 0x0062_c250;
/// `00a5b950` (unnamed, `niobjectnet.cpp`): `thiscall`, one word (`RET 4`).
const NODE_SET_KEY: u32 = 0x00a5_b950;
/// `009b7cb0` (unnamed, `run_009b7a20`): no words; the word at `0x011c6278`.
const GLOBAL_KEY: u32 = 0x009b_7cb0;
/// The ray-cast helpers (see `run_009b36f0`): input constructor `004a3c20`, no-op
/// constructor `006815c0`, rotation `004a0c90`, `MobileObject::GetLooking` `00931d70`,
/// `NiMatrix3::MakeXRotation` `00524ac0`, `NiMatrix3::operator*` `0043f8d0`, column
/// `00439f50`, normalize `004a0c10`, the ray's start `004a3da0`, end `007f68a0`, filter
/// `004a3f70`, collector `004a3fb0`, `hkpAllRayHitCollector` `004a3a70` / `004a3bc0`.
const RAY_CONSTRUCT: u32 = 0x004a_3c20;
const MATRIX_ROTATION: u32 = 0x004a_0c90;
const GET_LOOKING: u32 = 0x0093_1d70;
const MAKE_X_ROTATION: u32 = 0x0052_4ac0;
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
const MATRIX_COLUMN: u32 = 0x0043_9f50;
const POINT_NORMALIZE: u32 = 0x004a_0c10;
const RAY_SET_START: u32 = 0x004a_3da0;
const RAY_SET_END: u32 = 0x007f_68a0;
const RAY_SET_FILTER: u32 = 0x004a_3f70;
const RAY_SET_COLLECTOR: u32 = 0x004a_3fb0;
const COLLECTOR_CONSTRUCT: u32 = 0x004a_3a70;
const COLLECTOR_DESTRUCT: u32 = 0x004a_3bc0;
/// `009c5ba0` (unnamed, `projectile.cpp`): `cdecl`, four words; a `bool`.
const RAY_CAST: u32 = 0x009c_5ba0;
/// `008cdd90` (unnamed): `thiscall` on the ray; its hits holder, whose `00460140` member
/// (`+0x10`) holds the hits (`00726070`: count; `004a46b0`: hit `index`, `RET 4`).
const RAY_HITS: u32 = 0x008c_dd90;
const MEMBER_AT_10: u32 = 0x0046_0140;
const HIT_AT: u32 = 0x004a_46b0;
/// `GetAVObjectForCollidable` (`004b5820`, Xbox PDB): `cdecl`, one word.
const AV_OBJECT_FOR_COLLIDABLE: u32 = 0x004b_5820;
/// `006fa820` (unnamed): `thiscall`; the collision object's rigid body.
const RIGID_BODY: u32 = 0x006f_a820;
/// `009ba550` / `009ba610` (unnamed): construct and destroy the hinge-constraint info local
/// (`ECX` only); `hkHingeConstraintCinfo::CreateData` (`00cbdcb0`, engine map).
const HINGE_CINFO_CONSTRUCT: u32 = 0x009b_a550;
const HINGE_CINFO_DESTRUCT: u32 = 0x009b_a610;
const HINGE_CREATE_DATA: u32 = 0x00cb_dcb0;
/// `004b4f00` (unnamed): `thiscall`; the body's transform; `00632b40` (`RET 4`) copies it
/// into a 0x40-byte local; `hkRotation::renormalize` (`00cb2fc0`, engine map).
const BODY_TRANSFORM: u32 = 0x004b_4f00;
const TRANSFORM_FROM: u32 = 0x0063_2b40;
const RENORMALIZE: u32 = 0x00cb_2fc0;
/// `00553fc0` (unnamed): `cdecl` (vector, point); `0062aea0` (unnamed): `thiscall`,
/// normalizes the vector.
const VECTOR_FROM: u32 = 0x0055_3fc0;
const VECTOR_NORMALIZE: u32 = 0x0062_aea0;
/// `004a0bd0` (unnamed, `bgsdecalmanager.cpp`): `thiscall`, one word (`RET 4`): an out
/// point.
const POINT_FROM_004A0BD0: u32 = 0x004a_0bd0;
/// `00524c40` (unnamed, `tesobjectweap.cpp`): `thiscall`, two words (`RET 8`); the
/// transform applied to a point.
const TRANSFORM_POINT: u32 = 0x0052_4c40;
/// `004a3e00` (unnamed): `cdecl` (vector, point).
const VECTOR_FROM_POINT: u32 = 0x004a_3e00;
/// `00aa13e0` (unnamed): `cdecl`, one word (a size); a block.
const ALLOCATE: u32 = 0x00aa_13e0;
/// `bhkRigidBody::AddActCon` (`00c8ef80`) and `bhkRigidBody::AddActConToWorld`
/// (`00c8dac0`), Xbox PDB: `thiscall`, one word (`RET 4`).
const ADD_ACT_CON: u32 = 0x00c8_ef80;
const ADD_ACT_CON_TO_WORLD: u32 = 0x00c8_dac0;
/// The ray filter: `MobileObject::GetCharController` (`009306d0`), its filter word
/// (`0070c440`, `RET 4`), layer (`004a39f0`, `RET 4`), the source object (`00874480`), its
/// filter (`00931ed0`, `RET 4`), the group (`004a3a20`) and setting it (`0059ce80`,
/// `RET 4`).
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
const CONTROLLER_FILTER: u32 = 0x0070_c440;
const FILTER_SET_LAYER: u32 = 0x004a_39f0;
const SOURCE_OBJECT: u32 = 0x0087_4480;
const SOURCE_FILTER: u32 = 0x0093_1ed0;
const FILTER_GROUP: u32 = 0x004a_3a20;
const FILTER_SET_GROUP: u32 = 0x0059_ce80;
/// The debug line helpers (dead code here; see `fn_009b90b0`): colour `00414430`, line
/// `004b3890`, the `Setting` `0x011f210c`, `TES::AddTempDebugObject` (`00458e20`) on the
/// object at `0x011dea10`.
const COLOR_CONSTRUCT: u32 = 0x0041_4430;
const DEBUG_LINE: u32 = 0x004b_3890;
const DEBUG_TIME_SETTING: u32 = 0x011f_210c;
const TES: u32 = 0x011d_ea10;
const ADD_TEMP_DEBUG_OBJECT: u32 = 0x0045_8e20;
/// `00696c30` (unnamed, `navmesh_util.cpp`): `thiscall`, one word (`RET 4`); copies a hit
/// into the local (its collidable at `+0x50`).
const HIT_COPY: u32 = 0x0069_6c30;
/// `004b59f0` / `004b5a80` (unnamed, `globalfunc.cpp`): `cdecl`, one word each.
const COLLIDABLE_OBJECT: u32 = 0x004b_59f0;
const OBJECT_BODY: u32 = 0x004b_5a80;
/// `TESObjectREFR::FindReferenceFor3D` (`0056f930`, Xbox PDB): `cdecl`, one word.
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// `0043b4f0` (unnamed, `modelloader.cpp`): `thiscall`, one word (`RET 4`); then
/// `0043b4d0` gives the layer.
const LAYER_SOURCE: u32 = 0x0043_b4f0;
/// `00517630` (unnamed): `thiscall`; the body's motion type.
const MOTION_TYPE: u32 = 0x0051_7630;
/// `MiddleHighProcess::GetCurrentFurniture` (`008d8b80`, engine map) and `008126e0`
/// (unnamed, `RET 4`).
const GET_CURRENT_FURNITURE: u32 = 0x008d_8b80;
const SET_FURNITURE_008126E0: u32 = 0x0081_26e0;
/// `bhkCharacterController::SetPosition` (`00c6e390`, the map's `SetPosition_ov2`):
/// `thiscall`, one word (`RET 4`).
const SET_CONTROLLER_POSITION: u32 = 0x00c6_e390;
/// `0053d280` (unnamed, `tesland.cpp`): `thiscall`, two words (`RET 8`): an out point and
/// a float; the point times the float.
const POINT_SCALE_INTO: u32 = 0x0053_d280;
/// `00cbe240` (unnamed, `hkphingeconstraintdata.cpp`): `thiscall`, four words (`RET 0x10`).
const HINGE_SET_FRAMES: u32 = 0x00cb_e240;
/// `bhkConstraint::Init` (`00c89dd0`, Xbox PDB): `thiscall`, one word (`RET 4`).
const CONSTRAINT_INIT: u32 = 0x00c8_9dd0;
/// `004b5120` (unnamed, `globalfunc.cpp`): the base constructor (`ECX` only).
const CONSTRAINT_BASE_CONSTRUCT: u32 = 0x004b_5120;
/// The `bhkHingeConstraint` and `bhkConstraint` vtables (RTTI `.?AVbhkHingeConstraint@@`,
/// `.?AVbhkConstraint@@`) and the two instance counters (`0x01268434`, `0x01268188`).
const HINGE_VTABLE: u32 = 0x0108_fddc;
const CONSTRAINT_VTABLE: u32 = 0x0108_febc;
const HINGE_COUNT: u32 = 0x0126_8434;
const CONSTRAINT_COUNT: u32 = 0x0126_8188;

/// The ray filter word as both ray tests build it into `filter` (`008c71b0` with 0, then
/// the controller's or layer 6 with the source's group or a new key).
fn build_filter(e: &mut Engine, this: Ptr, filter: u32, controller_slot: u32, source_slot: u32) {
    e.call(KIND_LOCAL_008C71B0, &args![filter, 0u32]);
    if e.call(GET_CHAR_CONTROLLER, &args![this]).u32() != 0 {
        let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
        let word = e
            .call(CONTROLLER_FILTER, &args![controller, controller_slot])
            .u32();
        let value = e.mem.u32(word);
        e.mem.set_u32(filter, value);
        return;
    }
    e.call(FILTER_SET_LAYER, &args![filter, 6u32]);
    let source_counts = if e.call(SOURCE_OBJECT, &args![this]).u32() != 0 {
        let source = e.call(SOURCE_OBJECT, &args![this]).u32();
        e.vcall(source, 0x100, &[]).u8() != 0
    } else {
        false
    };
    if source_counts {
        let source = e.call(SOURCE_OBJECT, &args![this]).u32();
        let object = e.call(SOURCE_FILTER, &args![source, source_slot]).u32();
        let group = e.call(FILTER_GROUP, &args![object]).u32();
        e.call(FILTER_SET_GROUP, &args![filter, group]);
    } else {
        let key = e.call(NEXT_KEY, &[]).u32();
        e.call(FILTER_SET_GROUP, &args![filter, key]);
    }
}

/// The aim the ray tests use: position (slot `+0x1f4`) into `position`, and the facing
/// (slot `+0x2bc` with 0) and looking angles (`00931d70`) as a rotation whose column 1,
/// normalized, goes into `direction`. `matrix`, `looking` and `product` are the locals.
fn aim(e: &mut Engine, this: Ptr, locals: [u32; 5]) {
    let [position, direction, matrix, looking, product] = locals;
    let p = this.addr();
    e.call(LIST_FIRST_006815C0, &args![position]);
    e.call(LIST_FIRST_006815C0, &args![direction]);
    let source = e.vcall(p, 0x1f4, &[]).u32();
    let words = e.mem.bytes(source, 0xc);
    e.mem.write(position, &words);
    e.call(LIST_FIRST_006815C0, &args![matrix]);
    let facing = e.vcall(p, 0x2bc, &args![0u32]).f64();
    e.call(MATRIX_ROTATION, &args![matrix, x87_store32(facing)]);
    e.call(LIST_FIRST_006815C0, &args![looking]);
    let look = e.call(GET_LOOKING, &args![this]).f64();
    e.call(MAKE_X_ROTATION, &args![looking, x87_store32(look)]);
    let result = e
        .call(MATRIX_MULTIPLY, &args![matrix, product, looking])
        .u32();
    let words = e.mem.bytes(result, 0x24);
    e.mem.write(matrix, &words);
    e.call(MATRIX_COLUMN, &args![matrix, 1u32, direction]);
    e.call(POINT_NORMALIZE, &args![direction]);
}

/// The bounce: the reflected direction of the projectile (`009b7010` times -1.0,
/// normalized, minus the normal times twice its dot with the normal) scaled by the
/// `Setting` `0x011ce1dc` times the shooter's speed, added to the node's velocity.
fn reflect_velocity(e: &mut Engine, this: Ptr, node: u32, impact: u32) {
    e.with_stack(0xe0, |e, frame| {
        let base = frame.addr() + 0xe0;
        let at = |offset: u32| base - offset;
        let velocity = at(0x24);
        let reflected = at(0x4c);
        for word in 0..3 {
            let value = e.mem.u32(ZERO_POINT + 4 * word);
            e.mem.set_u32(velocity + 4 * word, value);
        }
        let direction = e.call(HIT_DIRECTION, &args![this, at(0x94)]).u32();
        let minus_one = x87_quiet_bits(e.mem.u32(MINUS_ONE));
        let flipped = e
            .call(FLOAT_TIMES_POINT, &args![at(0xa0), minus_one, direction])
            .u32();
        let words = e.mem.bytes(flipped, 0xc);
        e.mem.write(reflected, &words);
        e.call(NORMALIZE, &args![reflected]);
        e.call(NORMALIZE, &args![impact + 0x10]);
        let dot = x87_store32(e.call(DOT, &args![impact + 0x10, reflected]).f64());
        let along = e
            .call(
                FLOAT_TIMES_POINT,
                &args![at(0xac), x87_quiet_bits(dot), impact + 0x10],
            )
            .u32();
        let difference = e
            .call(POINT_SUBTRACT, &args![along, at(0xb8), reflected])
            .u32();
        let two = x87_quiet_bits(e.mem.u32(TWO_FLOAT));
        e.call(FLOAT_TIMES_POINT, &args![at(0x40), two, difference]);
        let sum = e
            .call(POINT_ADD, &args![reflected, at(0xc4), at(0x40)])
            .u32();
        let words = e.mem.bytes(sum, 0xc);
        e.mem.write(velocity, &words);
        let setting = e.call(SETTING_FLOAT, &args![SETTING_011CE1DC]).u32();
        let shooter = e.call(OBJECT_004181E0, &args![this]).u32();
        let speed = e.call(SHOOTER_SPEED, &args![shooter]).f64();
        let factor = x87_store32(x87_mul(speed, x87_m32(e.mem.u32(setting))));
        e.call(SCALE_BY, &args![velocity, x87_quiet_bits(factor)]);
        e.call(ADD_VELOCITY, &args![node, velocity, 1u32]);
    });
}

// Translated from 009b8c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX` only; state 2 of the projectile update): with a node (slot
/// `+0x1d0`) and an impact (the list at `+0x88`): releases the holder at `0x011f2250` when
/// it holds `this` (`009c8c40` with 0), sets flag 4 (`005de0a0`), calls slot `+0x2a4`,
/// passes 1 to the node's first collision object (`004b5260`, `00573ed0`); unless paused
/// (`0047c850`) or `009d9f00` answers non-zero, moves the node to the impact point (`+4`)
/// and clears its rotation point (`0043d410`, `00a59c60`); sets the node's motion (5
/// words 1, 1, 0, 1), a new collision key (`005dbe10`, 1, 0) and `00c6a0b0` (1, 1, 0).
/// Then, unless paused, the bounce: the reflected direction (`reflect_velocity`) is
/// added to the node's velocity, and its angular velocity (`0062ba80`) becomes the
/// impact normal in the node's frame (`004b46a0`, `004b4500`), its height dropped and its
/// sideways part scaled (`00408820`, `00631070`: when the second `00408820` value is
/// below the first, the z part times `00631070` of it times the first), normalized, as
/// `(-z, 0, -x)` turned back to the world (`004b4500`) and scaled by the `Setting`
/// `0x011ce1dc`. Returns true.
pub fn fn_009b8c90(e: &mut Engine, this: Ptr) -> bool {
    let p = this.addr();
    if e.vcall(p, 0x1d0, &[]).u32() == 0 {
        return true;
    }
    if e.call(LIST_EMPTY_008256D0, &args![p + 0x88]).u8() != 0 {
        return true;
    }
    if e.call(HOLDER_GET_007AF430, &args![HOLDER_011F2250]).u32() == p {
        e.call(HOLDER_SET_009C8C40, &args![HOLDER_011F2250, 0u32]);
    }
    let node = e.vcall(p, 0x1d0, &[]).u32();
    let slot = e.call(LIST_FIRST_006815C0, &args![p + 0x88]).u32();
    let impact = e.mem.u32(slot);
    e.call(SET_FLAG, &args![this, 4u32, 1u32]);
    e.vcall(p, 0x2a4, &[]);
    let collision = e.call(FIND_FIRST_COLLISION_OBJECT, &args![node]).u32();
    if collision != 0 {
        e.call(COLLISION_SET_00573ED0, &args![collision, 1u32]);
    }
    let pause_owner = e.mem.u32(PAUSE_OWNER);
    if e.call(IS_PAUSED, &args![pause_owner]).u8() == 0
        && e.call(TEST_009D9F00, &args![this]).u32() == 0
    {
        e.call(SET_LOCAL_TRANSLATE, &args![node, impact + 0x4]);
        e.with_stack(0xc, |e, point| {
            e.call(POINT_CONSTRUCT, &args![point, 0.0f32.to_bits(), 0u32, 0u32]);
            e.call(NODE_SET_POINT, &args![node, point]);
        });
    }
    e.call(SET_MOTION, &args![node, 1u32, 1u32, 0u32, 1u32]);
    let key = e.call(NEXT_KEY, &[]).u32();
    e.call(SET_COLLISION_KEY, &args![node, key, 1u32, 0u32]);
    e.call(SET_COLLISION_00C6A0B0, &args![node, 1u32, 1u32, 0u32]);
    let pause_owner = e.mem.u32(PAUSE_OWNER);
    if e.call(IS_PAUSED, &args![pause_owner]).u8() != 0 {
        return true;
    }
    reflect_velocity(e, this, node, impact);
    e.with_stack(0xe0, |e, frame| {
        let base = frame.addr() + 0xe0;
        let at = |offset: u32| base - offset;
        let spin = at(0x30);
        let inverse = at(0x70);
        let local = at(0x88);
        let side = at(0x7c);
        for word in 0..3 {
            let value = e.mem.u32(ZERO_POINT + 4 * word);
            e.mem.set_u32(spin + 4 * word, value);
        }
        e.call(LIST_FIRST_006815C0, &args![inverse]);
        let own = e.vcall(p, 0x1d0, &[]).u32();
        let transform = e.call(WORLD_TRANSFORM, &args![own]).u32();
        e.call(MATRIX_INVERSE, &args![transform, inverse]);
        e.call(MATRIX_TIMES_POINT, &args![inverse, local, impact + 0x10]);
        let y = x87_quiet_bits(e.mem.u32(local + 4));
        let first = e.call(CALLEE_00408820, &args![y]).f64();
        let z = x87_quiet_bits(e.mem.u32(local + 8));
        let second = e.call(CALLEE_00408820, &args![z]).f64();
        // `FCOMP` of the second against the first, `TEST AH,0x5; JP`: on only when below.
        if second < first {
            let y = x87_quiet_bits(e.mem.u32(local + 4));
            let again = e.call(CALLEE_00408820, &args![y]).f64();
            let z = x87_quiet_bits(e.mem.u32(local + 8));
            let water = e.call(CALLEE_00631070, &args![z]).f64();
            e.mem.set_u32(local + 8, x87_store32(x87_mul(water, again)));
        }
        e.mem.set_u32(local + 4, 0.0f32.to_bits());
        e.call(NORMALIZE, &args![local]);
        let x = x87_load32(e.mem.u32(local));
        let z = x87_load32(e.mem.u32(local + 8));
        e.call(
            POINT3_CONSTRUCT,
            &args![side, x87_store32(-z), 0.0f32.to_bits(), x87_store32(-x)],
        );
        let node_transform = e.call(WORLD_TRANSFORM, &args![node]).u32();
        let turned = e
            .call(MATRIX_TIMES_POINT, &args![node_transform, at(0xd0), side])
            .u32();
        let words = e.mem.bytes(turned, 0xc);
        e.mem.write(spin, &words);
        let setting = e.call(SETTING_FLOAT, &args![SETTING_011CE1DC]).u32();
        let factor = x87_quiet_bits(e.mem.u32(setting));
        e.call(SCALE_BY, &args![spin, factor]);
        e.call(ADD_VELOCITY_0062BA80, &args![node, spin, 1u32]);
    });
    true
}

/// How `fn_009b90b0` ends before its cloning local exists.
enum CutEnd {
    /// Nothing more (`009ba2f6`).
    Done,
    /// The projectile expires (`009bc8f0`).
    Expire,
    /// State 4 and `009ba660`.
    GiveUp,
}

// Translated from 009b90b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009b90b0 - the C++ exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated.
/// Unnamed in the map (`ECX` only; state 3 of the projectile update: a limb cut off by the
/// impact). Keeps the `Setting` `0x011ce1e8`'s float in a function-local `static` on the
/// first call. Without an impact (the list at `+0x88`; in state 4 its second entry), or one
/// whose actor's slot `+0x100` is false, the state becomes 4 (`009ba660`). An impact
/// already handled (`+0x28`) does nothing; one whose part (`+0x24`) the actor has not lost
/// (`TESObjectREFR::GetDismembered`), or lost already as the actor's dismemberment extra
/// says (`0042e8c0`, a set byte at `+1` of the entry for the part), or without a node,
/// expires the projectile (`009bc8f0`). Otherwise the limb node (the actor node's slot
/// `+0x9c` with the part's name from `0x01196d6c`) is cloned (`NiObject::Clone`), the
/// actor hides it (`008b6df0`), its skinned children are attached to its skeleton
/// (`004ade40`), it goes into the cell world (slot `+0xd0`) at the origin, its first child
/// placed where the actor's was (`008b42e0`, `0062c250`, lowered by the static), and it is
/// attached under the projectile node (slot `+0xdc`). A ray 512 below to 1024 ahead
/// (`009c5ba0`) that hits a different rigid body hinges the limb to it
/// (`hkHingeConstraintCinfo`, `fn_009ba3d0`, `fn_009ba410`, `bhkRigidBody::AddActCon`), and
/// a second ray along the projectile (the dead debug drawing after it is kept) says the
/// limb stuck: a hit on a layer 1, 2, 9, 0xd, 0x11 or 0x14 body with motion type 4 or 5
/// (an earlier other reference ends the search). Stuck, the limb keeps the controller's
/// collision key and the projectile follows it; otherwise the projectile is synced, its
/// node gets the actor's key, motion and the projectile's velocity (`009b7010`), and the
/// holder at `0x011f2250` lets go of it.
pub fn fn_009b90b0(e: &mut Engine, this: Ptr) {
    let p = this.addr();
    if e.mem.u32(STATIC_GUARD) & 1 == 0 {
        let guard = e.mem.u32(STATIC_GUARD);
        e.mem.set_u32(STATIC_GUARD, guard | 1);
        let value = e.call(SETTING_FLOAT, &args![SETTING_011CE1E8]).u32();
        let value = x87_quiet_bits(e.mem.u32(value));
        e.mem.set_u32(STATIC_FLOAT, value);
    }
    let vats_owner = e.mem.u32(VATS_OWNER);
    let in_vats = e.call(VATS_TEST, &args![vats_owner]).u8() != 0;
    let end = fn_009b90b0_cut(e, this, in_vats);
    match end {
        CutEnd::Done => {}
        CutEnd::Expire => {
            e.call(STATE_1_UPDATE_009BC8F0, &args![this]);
        }
        CutEnd::GiveUp => {
            e.mem.set_u32(p + STATE, 4);
            e.call(STATE_4_UPDATE_009BA660, &args![this]);
        }
    }
}

/// The body of `fn_009b90b0` after the static and the VATS test.
fn fn_009b90b0_cut(e: &mut Engine, this: Ptr, in_vats: bool) -> CutEnd {
    let p = this.addr();
    if e.call(LIST_EMPTY_008256D0, &args![p + 0x88]).u8() != 0 {
        return CutEnd::GiveUp;
    }
    let node = e.vcall(p, 0x1d0, &[]).u32();
    let children = e.vcall(node, 0xc, &[]).u32();
    let slot = e.call(LIST_FIRST_006815C0, &args![p + 0x88]).u32();
    let mut impact = e.mem.u32(slot);
    e.mem.set_u32(p + 0x158, x87_quiet_bits(e.mem.u32(TEN)));
    if e.mem.u32(p + STATE) == 4 {
        impact = if e.call(NODE_NEXT, &args![p + 0x88]).u32() != 0 {
            let next = e.call(NODE_NEXT, &args![p + 0x88]).u32();
            let slot = e.call(LIST_FIRST_006815C0, &args![next]).u32();
            e.mem.u32(slot)
        } else {
            0
        };
    }
    if impact == 0 || e.mem.u32(impact) == 0 {
        return CutEnd::GiveUp;
    }
    let target = e.mem.u32(impact);
    if e.vcall(target, 0x100, &[]).u8() == 0 {
        return CutEnd::GiveUp;
    }
    if e.mem.u8(impact + 0x28) != 0 {
        return CutEnd::Done;
    }
    let actor = e.mem.u32(impact);
    let part = e.mem.u32(impact + 0x24);
    if e.call(GET_DISMEMBERED, &args![actor, part]).u8() == 0 {
        return CutEnd::Expire;
    }
    let extra_list = e.call(EXTRA_LIST, &args![actor]).u32();
    let extra = e.call(DISMEMBER_EXTRA, &args![extra_list]).u32();
    let mut cut_before = false;
    let mut index = 0u32;
    while index < e.call(ARRAY_COUNT, &args![extra + 0x20]).u32() {
        let entry_slot = e.call(ARRAY_ELEMENT, &args![extra + 0x20, index]).u32();
        let entry = e.mem.u32(entry_slot);
        let wanted = i32::from(e.mem.i8(impact + 0x24));
        if i32::from(e.mem.u8(entry)) == wanted {
            let entry_slot = e.call(ARRAY_ELEMENT, &args![extra + 0x20, index]).u32();
            let entry = e.mem.u32(entry_slot);
            if e.mem.u8(entry + 1) != 0 {
                cut_before = true;
            }
        }
        index = index.wrapping_add(1);
    }
    let part_node = if e.mem.u32(impact) != 0 {
        let actor = e.mem.u32(impact);
        e.vcall(actor, 0x1d0, &[]).u32()
    } else {
        0
    };
    if part_node == 0 || cut_before {
        return CutEnd::Expire;
    }
    let part = e.mem.u32(impact + 0x24);
    let name = e.mem.u32(part.wrapping_mul(4).wrapping_add(LIMB_NAMES));
    e.with_stack(0xd00, |e, frame| {
        let base = frame.addr() + 0xd00;
        let at = |offset: u32| base - offset;
        // `part_node` is set here, so the exe's null branch (limb 0) is not taken.
        let mut name_constructed = false;
        let limb = if part_node != 0 {
            let string = e.call(FIXED_STRING_CONSTRUCT, &args![at(0x50), name]).u32();
            name_constructed = true;
            e.vcall(part_node, 0x9c, &args![string]).u32()
        } else {
            0
        };
        if name_constructed {
            e.call(FIXED_STRING_DESTRUCT, &args![at(0x50)]);
        }
        let scale = x87_store32(e.call(NODE_SCALE, &args![limb]).f64());
        if limb == 0 {
            e.mem.set_u8(p + 0x90, 0);
            return CutEnd::Done;
        }
        let cloning = at(0x80);
        e.call(CLONING_CONSTRUCT, &args![cloning, 1.0f32.to_bits()]);
        e.call(CLONING_SET, &args![cloning, 1.0f32.to_bits()]);
        let clone = e.call(CLONE, &args![limb, cloning]).u32();
        let part = e.mem.u32(impact + 0x24);
        e.call(ACTOR_HIDE_LIMB, &args![actor, part]);
        let limb = clone;
        e.call(NODE_SET_00450F90, &args![limb, 0u32]);
        e.call(SET_LOCAL_SCALE, &args![limb, x87_quiet_bits(scale)]);
        let skin = e.vcall(limb, 0xc, &[]).u32();
        if skin != 0 {
            let mut index = 0u32;
            while index < e.call(CHILD_COUNT, &args![skin]).u32() {
                let child = e.call(CHILD_AT, &args![skin, index]).u32();
                if child != 0 && e.call(FIND_SKINNED_NODE, &args![child]).u8() != 0 {
                    e.call(ATTACH_TO_SKELETON, &args![skin, child, skin, 0u32]);
                }
                index = index.wrapping_add(1);
            }
        }
        e.call(
            POINT_CONSTRUCT,
            &args![at(0x9c), 0.0f32.to_bits(), 0u32, 0u32],
        );
        e.call(NODE_SET_POINT, &args![limb, at(0x9c)]);
        let cell = e.call(PARENT_CELL, &args![this]).u32();
        let world = e.call(CELL_WORLD, &args![cell]).u32();
        if world != 0 {
            e.vcall(world, 0xd0, &args![limb, 1u32, 0u32, 0u32, 1u32]);
        }
        e.call(SET_LOCAL_TRANSLATE, &args![limb, ZERO_POINT]);
        e.call(SET_LOCAL_ROTATE, &args![limb, IDENTITY_ROTATION]);
        if skin != 0 && e.call(CHILD_PRESENT, &args![skin, 0u32]).u32() != 0 {
            let placed = at(0xd4);
            let first = e.call(CHILD_AT, &args![skin, 0u32]).u32();
            let transform = e.call(WORLD_TRANSFORM, &args![first]).u32();
            let words = e.mem.bytes(transform, 0x34);
            e.mem.write(placed, &words);
            if !in_vats {
                e.call(SET_LOCATION_ON_REFERENCE, &args![this, impact + 0x4]);
            }
            e.call(SYNC_3D, &args![this, 0u32, 0u32]);
            e.call(
                POINT_CONSTRUCT,
                &args![at(0xe0), 0.0f32.to_bits(), 0u32, 0u32],
            );
            e.vcall(children, 0xb8, &args![at(0xe0)]);
            let parent = at(0x114);
            let transform = e.call(WORLD_TRANSFORM, &args![children]).u32();
            let words = e.mem.bytes(transform, 0x34);
            e.mem.write(parent, &words);
            let inverse = at(0x148);
            e.call(TRANSFORM_CONSTRUCT, &args![inverse]);
            e.call(TRANSFORM_INVERT, &args![parent, inverse]);
            let product = e
                .call(TRANSFORM_MULTIPLY, &args![inverse, at(0x17c), placed])
                .u32();
            let words = e.mem.bytes(product, 0x34);
            e.mem.write(placed, &words);
            let height = x87_load32(e.mem.u32(placed + 0x28));
            let lowered = x87_sub(height, x87_m32(e.mem.u32(STATIC_FLOAT)));
            e.mem.set_u32(placed + 0x28, x87_store32(lowered));
            let first = e.call(CHILD_AT, &args![skin, 0u32]).u32();
            e.call(SET_LOCAL_TRANSLATE, &args![first, placed + 0x24]);
            let first = e.call(CHILD_AT, &args![skin, 0u32]).u32();
            e.call(SET_LOCAL_ROTATE, &args![first, placed]);
            let scale = x87_quiet_bits(e.mem.u32(placed + 0x30));
            let first = e.call(CHILD_AT, &args![skin, 0u32]).u32();
            e.call(SET_LOCAL_SCALE, &args![first, scale]);
        }
        e.vcall(children, 0xdc, &args![limb, 1u32]);
        let key = e.call(GLOBAL_KEY, &[]).u32();
        e.call(NODE_SET_KEY, &args![limb, key]);
        e.call(UPDATE_POSITION, &args![limb, 1u32, 1u32]);
        e.call(SET_MOTION, &args![limb, 4u32, 1u32, 1u32, 1u32]);
        e.call(
            POINT_CONSTRUCT,
            &args![at(0x188), 0.0f32.to_bits(), 0u32, 0u32],
        );
        e.call(NODE_SET_POINT, &args![children, at(0x188)]);
        let mut stuck = false;
        if limb != 0 {
            stuck = hinge_and_test(e, this, impact, limb, at);
        }
        e.mem.set_u8(impact + 0x28, 1);
        if stuck {
            if e.call(GET_CURRENT_FURNITURE, &args![this]).u32() != 0 {
                e.call(SET_FURNITURE_008126E0, &args![this, 0u32]);
            }
            e.call(UPDATE_POSITION, &args![limb, 1u32, 1u32]);
            e.call(SET_MOTION, &args![limb, 4u32, 1u32, 1u32, 1u32]);
            if e.call(GET_CHAR_CONTROLLER, &args![this]).u32() != 0 {
                let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
                let word = e
                    .call(CONTROLLER_FILTER, &args![controller, at(0xcac)])
                    .u32();
                let group = e.call(FILTER_GROUP, &args![word]).u32();
                e.call(SET_COLLISION_KEY, &args![limb, group, 1u32, 1u32]);
                if !in_vats {
                    e.call(SET_LOCATION_ON_REFERENCE, &args![this, impact + 0x4]);
                    let vector = at(0xcc0);
                    e.call(LIST_FIRST_006815C0, &args![vector]);
                    e.call(VECTOR_FROM_POINT, &args![vector, impact + 0x4]);
                    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
                    e.call(SET_CONTROLLER_POSITION, &args![controller, vector]);
                }
                e.call(SYNC_3D, &args![this, 0u32, 0u32]);
            }
            if !in_vats {
                e.mem.set_u8(p + 0x90, 0);
            }
        } else {
            if !in_vats {
                e.call(SET_LOCATION_ON_REFERENCE, &args![this, impact + 0x4]);
            }
            e.call(SYNC_3D, &args![this, 0u32, 0u32]);
            e.call(SET_FLAG, &args![this, 4u32, 1u32]);
            e.vcall(p, 0x2a4, &[]);
            e.call(UPDATE_POSITION, &args![limb, 1u32, 1u32]);
            let object = e.call(SOURCE_FILTER, &args![actor, at(0xcc4)]).u32();
            let group = e.call(FILTER_GROUP, &args![object]).u32();
            e.call(SET_COLLISION_KEY, &args![children, group, 1u32, 1u32]);
            e.call(SET_MOTION, &args![children, 1u32, 1u32, 1u32, 1u32]);
            e.call(SET_COLLISION_00C6A0B0, &args![children, 1u32, 1u32, 0u32]);
            let two = x87_quiet_bits(e.mem.u32(TWO_FLOAT));
            let speed = x87_store32(e.call(VALUE_009669C0, &args![this]).f64());
            let direction = e.call(HIT_DIRECTION, &args![this, at(0xce8)]).u32();
            let scaled = e
                .call(POINT_SCALED, &args![direction, at(0xcdc), speed])
                .u32();
            e.call(POINT_SCALE_INTO, &args![scaled, at(0xcd0), two]);
            e.call(ADD_VELOCITY, &args![children, at(0xcd0), 1u32]);
            if e.call(HOLDER_GET_007AF430, &args![HOLDER_011F2250]).u32() == p {
                e.call(HOLDER_SET_009C8C40, &args![HOLDER_011F2250, 0u32]);
            }
        }
        e.call(CLONING_DESTRUCT, &args![cloning]);
        CutEnd::Done
    })
}

/// `fn_009b90b0`'s two rays (frame offsets as the exe's): the hinge to what the first ray
/// hits, then whether the second ray says the limb stuck.
fn hinge_and_test(
    e: &mut Engine,
    this: Ptr,
    impact: u32,
    limb: u32,
    at: impl Fn(u32) -> u32,
) -> bool {
    let p = this.addr();
    let mut hit_object = 0u32;
    let cell = e.call(PARENT_CELL, &args![this]).u32();
    let _world = e.call(CELL_WORLD, &args![cell]).u32();
    let ray = at(0x250);
    let position = at(0x25c);
    let direction = at(0x268);
    e.call(RAY_CONSTRUCT, &args![ray]);
    aim(
        e,
        this,
        [position, direction, at(0x28c), at(0x2b0), at(0x2d4)],
    );
    let below = x87_quiet_bits(e.mem.u32(RANGE_512));
    let down = e
        .call(POINT_SCALED, &args![direction, at(0x2e0), below])
        .u32();
    let start = e
        .call(POINT_SUBTRACT, &args![position, at(0x2ec), down])
        .u32();
    e.call(RAY_SET_START, &args![ray, start]);
    let ahead = x87_quiet_bits(e.mem.u32(RANGE_1024));
    let end = e
        .call(POINT_SCALED, &args![direction, at(0x2f8), ahead])
        .u32();
    e.call(RAY_SET_END, &args![ray, end]);
    let collector = at(0x620);
    e.call(COLLECTOR_CONSTRUCT, &args![collector]);
    e.call(RAY_SET_COLLECTOR, &args![ray, collector]);
    if e.call(RAY_CAST, &args![ray, limb, 0u32, 1u32]).u8() != 0 {
        let holder = e.call(RAY_HITS, &args![ray]).u32();
        let hits = e.call(MEMBER_AT_10, &args![holder]).u32();
        if e.call(NODE_NEXT, &args![hits]).i32() > 0 {
            let holder = e.call(RAY_HITS, &args![ray]).u32();
            let hits = e.call(MEMBER_AT_10, &args![holder]).u32();
            let hit = e.call(HIT_AT, &args![hits, 0u32]).u32();
            let collidable = e.mem.u32(hit + 0x50);
            hit_object = e.call(AV_OBJECT_FOR_COLLIDABLE, &args![collidable]).u32();
        }
    }
    e.call(COLLECTOR_DESTRUCT, &args![collector]);
    if hit_object == 0 {
        return false;
    }
    let own_node = e.vcall(p, 0x1d0, &[]).u32();
    let own_collision = e.call(FIND_FIRST_COLLISION_OBJECT, &args![own_node]).u32();
    let own_body = if own_collision != 0 {
        e.call(RIGID_BODY, &args![own_collision]).u32()
    } else {
        0
    };
    let hit_collision = e
        .call(FIND_FIRST_COLLISION_OBJECT, &args![hit_object])
        .u32();
    let hit_body = if hit_collision != 0 {
        e.call(RIGID_BODY, &args![hit_collision]).u32()
    } else {
        0
    };
    if hit_body == 0 || hit_body == own_body {
        return false;
    }
    let hinge = at(0x650);
    e.call(HINGE_CINFO_CONSTRUCT, &args![hinge]);
    e.call(HINGE_CREATE_DATA, &args![hinge]);
    fn_009ba320(e, Ptr::new(hinge), hit_body, own_body);
    let hit_frame = at(0x690);
    let own_frame = at(0x6d0);
    let converted = e.call(CONVERT_004AE750, &args![hit_body]).u32();
    let transform = e.call(BODY_TRANSFORM, &args![converted]).u32();
    e.call(TRANSFORM_FROM, &args![hit_frame, transform]);
    let converted = e.call(CONVERT_004AE750, &args![own_body]).u32();
    let transform = e.call(BODY_TRANSFORM, &args![converted]).u32();
    e.call(TRANSFORM_FROM, &args![own_frame, transform]);
    let rotation = e.call(LIST_FIRST_006815C0, &args![hit_frame]).u32();
    e.call(RENORMALIZE, &args![rotation]);
    let rotation = e.call(LIST_FIRST_006815C0, &args![own_frame]).u32();
    e.call(RENORMALIZE, &args![rotation]);
    let column = at(0x6dc);
    e.call(LIST_FIRST_006815C0, &args![column]);
    let own_node = e.vcall(p, 0x1d0, &[]).u32();
    let transform = e.call(WORLD_TRANSFORM, &args![own_node]).u32();
    e.call(MATRIX_COLUMN, &args![transform, 1u32, column]);
    let axis = at(0x6f0);
    e.call(LIST_FIRST_006815C0, &args![axis]);
    e.call(VECTOR_FROM, &args![axis, column]);
    e.call(VECTOR_NORMALIZE, &args![axis]);
    let node_transform = at(0x724);
    let own_node = e.vcall(p, 0x1d0, &[]).u32();
    let transform = e.call(WORLD_TRANSFORM, &args![own_node]).u32();
    let words = e.mem.bytes(transform, 0x34);
    e.mem.write(node_transform, &words);
    let drop = x87_quiet_bits(e.mem.u32(STATIC_FLOAT));
    let pivot = e
        .call(POINT_FROM_004A0BD0, &args![PIVOT_POINT, at(0x73c)])
        .u32();
    e.call(POINT_SCALED, &args![pivot, at(0x730), drop]);
    e.call(
        TRANSFORM_POINT,
        &args![node_transform, at(0x748), at(0x730)],
    );
    let pivot_vector = at(0x760);
    e.call(LIST_FIRST_006815C0, &args![pivot_vector]);
    e.call(VECTOR_FROM_POINT, &args![pivot_vector, at(0x748)]);
    fn_009ba3d0(e, Ptr::new(hinge), hit_frame, own_frame, pivot_vector, axis);
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    let constraint = if block != 0 {
        fn_009ba410(e, Ptr::new(block), hinge).addr()
    } else {
        0
    };
    e.call(ADD_ACT_CON, &args![hit_body, constraint]);
    let cell = e.call(PARENT_CELL, &args![this]).u32();
    let world = e.call(CELL_WORLD, &args![cell]).u32();
    e.call(ADD_ACT_CON_TO_WORLD, &args![hit_body, world]);

    let cell = e.call(PARENT_CELL, &args![this]).u32();
    let _world = e.call(CELL_WORLD, &args![cell]).u32();
    let ray = at(0x830);
    let position = at(0x83c);
    let direction = at(0x848);
    e.call(RAY_CONSTRUCT, &args![ray]);
    aim(
        e,
        this,
        [position, direction, at(0x86c), at(0x890), at(0x8b4)],
    );
    let filter = at(0x8b8);
    build_filter(e, this, filter, at(0x8bc), at(0x8c4));
    let full = e.call(VALUE_00979260, &args![this]).f64();
    let used = e.call(VALUE_00508070, &args![this]).f64();
    let length = x87_store32(x87_sub(full, used));
    let end = at(0x8e0);
    e.call(POINT_SCALED, &args![direction, end, x87_quiet_bits(length)]);
    let filter_word = e.mem.u32(filter);
    e.call(RAY_SET_FILTER, &args![ray, filter_word]);
    e.call(RAY_SET_START, &args![ray, position]);
    e.call(RAY_SET_END, &args![ray, end]);
    let collector = at(0xc00);
    e.call(COLLECTOR_CONSTRUCT, &args![collector]);
    e.call(RAY_SET_COLLECTOR, &args![ray, collector]);
    let cell = e.call(PARENT_CELL, &args![this]).u32();
    let world = e.call(CELL_WORLD, &args![cell]).u32();
    e.vcall(world, 0xc8, &args![ray]);
    // `XOR ECX,ECX; JZ`: the zero flag is always set, so the debug line below is never
    // drawn (a compiler-folded condition). Kept as the exe has it.
    let zero_flag = true;
    if !zero_flag {
        let one = 1.0f32.to_bits();
        let half = x87_quiet_bits(e.mem.u32(HALF));
        let color = at(0xc14);
        e.call(COLOR_CONSTRUCT, &args![color, one, half, half, one]);
        let sum = e.call(POINT_ADD, &args![position, at(0xc20), end]).u32();
        let line = e
            .call(DEBUG_LINE, &args![position, color, sum, color, 1u32])
            .u32();
        let time = e.call(SETTING_FLOAT, &args![DEBUG_TIME_SETTING]).u32();
        let time = x87_quiet_bits(e.mem.u32(time));
        let tes = e.mem.u32(TES);
        e.call(ADD_TEMP_DEBUG_OBJECT, &args![tes, line, time]);
    }
    let mut stuck = false;
    let mut index = 0i32;
    while !stuck {
        let holder = e.call(RAY_HITS, &args![ray]).u32();
        let hits = e.call(MEMBER_AT_10, &args![holder]).u32();
        if index >= e.call(NODE_NEXT, &args![hits]).i32() {
            break;
        }
        let holder = e.call(RAY_HITS, &args![ray]).u32();
        let hits = e.call(MEMBER_AT_10, &args![holder]).u32();
        let hit = e.call(HIT_AT, &args![hits, index as u32]).u32();
        let copy = at(0xc90);
        e.call(HIT_COPY, &args![copy, hit]);
        let collidable = e.mem.u32(copy + 0x50);
        let av_object = e.call(AV_OBJECT_FOR_COLLIDABLE, &args![collidable]).u32();
        let collidable = e.mem.u32(copy + 0x50);
        let object = e.call(COLLIDABLE_OBJECT, &args![collidable]).u32();
        let body = e.call(OBJECT_BODY, &args![object]).u32();
        let reference = e.call(FIND_REFERENCE_FOR_3D, &args![av_object]).u32();
        let layer = if body != 0 {
            let source = e.call(LAYER_SOURCE, &args![body, at(0xca0)]).u32();
            e.call(KIND_OF_0043B4D0, &args![source]).u32()
        } else {
            0
        };
        let mut sticks = false;
        if body != 0 && matches!(layer, 1 | 2 | 9 | 0x11 | 0xd | 0x14) {
            sticks = e.call(MOTION_TYPE, &args![body]).u32() == 5
                || e.call(MOTION_TYPE, &args![body]).u32() == 4;
        }
        if sticks {
            stuck = true;
        } else if body != 0 && reference != p && reference != 0 && reference != e.mem.u32(impact) {
            break;
        }
        index = index.wrapping_add(1);
    }
    e.call(COLLECTOR_DESTRUCT, &args![collector]);
    e.call(HINGE_CINFO_DESTRUCT, &args![hinge]);
    stuck
}

// Translated from 009ba3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 0x10`; the hinge-constraint info): with its data
/// (`+4`), passes the four words to `00cbe240` (the hinge's frames and axes).
pub fn fn_009ba3d0(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32, fourth: u32) {
    let data = e.mem.u32(this.addr() + 0x4);
    if data != 0 {
        e.call(HINGE_SET_FRAMES, &args![data, first, second, third, fourth]);
    }
}

// Translated from 009ba410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ba410 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `bhkHingeConstraint`'s constructor (unnamed in the map; `thiscall`, `RET 4`): the base
/// (`fn_009ba4a0`), the vtable `0108fddc`, `bhkConstraint::Init` with `cinfo + 4` (null for
/// a null `cinfo`), and one more hinge constraint counted at `0x01268434`. Returns `this`.
pub fn fn_009ba410(e: &mut Engine, this: Ptr, cinfo: u32) -> Ptr {
    fn_009ba4a0(e, this);
    e.mem.set_u32(this.addr(), HINGE_VTABLE);
    let data = if cinfo != 0 { cinfo + 0x4 } else { 0 };
    e.call(CONSTRAINT_INIT, &args![this, data]);
    let count = e.mem.u32(HINGE_COUNT);
    e.mem.set_u32(HINGE_COUNT, count.wrapping_add(1));
    this
}

// Translated from 009ba4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint`'s constructor (unnamed in the map; `ECX` only): `004b5120`, the vtable
/// `0108febc`, and one more constraint counted at `0x01268188`. Returns `this`.
pub fn fn_009ba4a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CONSTRAINT_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), CONSTRAINT_VTABLE);
    let count = e.mem.u32(CONSTRAINT_COUNT);
    e.mem.set_u32(CONSTRAINT_COUNT, count.wrapping_add(1));
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009b_8c90, fn_009b8c90(Ptr) -> bool),
        entry!(0x009b_90b0, fn_009b90b0(Ptr)),
        entry!(0x009b_a3d0, fn_009ba3d0(Ptr, u32, u32, u32, u32)),
        entry!(0x009b_a410, fn_009ba410(Ptr, u32) -> Ptr),
        entry!(0x009b_a4a0, fn_009ba4a0(Ptr) -> Ptr),
        entry!(0x009b88e0, fn_009b88e0(Ptr) -> u32),
        entry!(0x009ba350, fn_009ba350(Ptr, u32)),
        entry!(0x009ba390, fn_009ba390(Ptr, u32)),
        entry!(0x009ba320, fn_009ba320(Ptr, u32, u32)),
        entry!(0x009b8a20, fn_009b8a20(Ptr)),
        entry!(0x009b8900, fn_009b8900(Ptr, u32, u32, u32, u32, u32, u32) -> bool),
        entry!(0x009b8b10, fn_009b8b10(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn answer(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    #[test]
    fn fn_009b88e0_tests_flag_0x800() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(TEST_FLAGS_004FD420, |_, _| answer(0x101));
        assert_eq!(e.call(0x009b88e0, &args![0x5000u32]).u32(), 0x101);
        assert_eq!(calls_at(&e, TEST_FLAGS_004FD420), vec![vec![0x5000, 0x800]]);
    }

    #[test]
    fn pointer_setters_convert_or_clear() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(CONVERT_004AE750, |_, a| answer(a[0] + 1));
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0xc, 9);
        e.call(0x009ba320, &args![this, 0x100u32, 0x200u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0x101);
        assert_eq!(e.mem.u32(this + 0x10), 0x201);
        e.call(0x009ba350, &args![this, 0u32]);
        e.call(0x009ba390, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(calls_at(&e, CONVERT_004AE750).len(), 2);
    }

    /// A projectile-like object with the state at +0x150 and the list helpers doubled.
    fn projectile(e: &mut Engine, state: u32) -> u32 {
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this + STATE, state);
        this
    }

    #[test]
    fn fn_009b8a20_moves_the_state_on() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(LIST_EMPTY_008256D0, |_, _| answer(0));
        e.register(OBJECT_004181E0, |_, _| answer(0x7000));
        e.register(OBJECT_TEST_009B7C90, |_, _| answer(0));
        e.register(TEST_BIT_00444D00, |_, _| answer(0));
        e.register(TEST_FLAGS_004FD420, |_, _| answer(0));
        e.register(TEST_009D9F00, |_, _| answer(0));
        let this = projectile(&mut e, 3);
        e.call(0x009b8a20, &args![this]);
        assert_eq!(e.mem.u32(this + STATE), 4);
        // Not 3: 1, then 2 through 009d9f00 since bit 1 is clear.
        e.register(TEST_009D9F00, |_, _| answer(5));
        e.call(0x009b8a20, &args![this]);
        assert_eq!(e.mem.u32(this + STATE), 2);
        // Bit 1 set and bit 2 clear: 0.
        e.register(TEST_BIT_00444D00, |_, a| answer(u32::from(a[1] == 1)));
        e.call(0x009b8a20, &args![this]);
        assert_eq!(e.mem.u32(this + STATE), 0);
        // Both bits set and the object test true: 3.
        e.register(TEST_BIT_00444D00, |_, _| answer(1));
        e.register(OBJECT_TEST_009B7C90, |_, _| answer(1));
        e.call(0x009b8a20, &args![this]);
        assert_eq!(e.mem.u32(this + STATE), 3);
        // An empty list leaves the state.
        e.mem.set_u32(this + STATE, 7);
        e.register(LIST_EMPTY_008256D0, |_, _| answer(1));
        e.call(0x009b8a20, &args![this]);
        assert_eq!(e.mem.u32(this + STATE), 7);
    }

    #[test]
    fn fn_009b8900_adds_allowed_impacts() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(LIST_EMPTY_008256D0, |_, _| answer(0));
        let item = e.mem.alloc(4);
        let record = e.mem.alloc(0x2c);
        e.mem.set_u32(item, record);
        e.register_double(LIST_FIRST_006815C0, move |_, _| answer(item));
        e.register(KIND_SOURCE_0043B540, |_, _| answer(0x10));
        e.register(KIND_LOCAL_008C71B0, |_, a| answer(a[0]));
        e.register(KIND_OF_0043B4D0, |_, _| answer(0x11));
        e.register(OBJECT_004181E0, |_, _| answer(0x7000));
        e.register(IMPACT_TEST_009C2C80, |_, _| answer(1));
        e.register(PROJECTILE_ADD_IMPACT, |_, _| answer(1));
        e.register(TEST_BIT_00444D00, |_, _| answer(1));
        e.register(OBJECT_TEST_009B7C90, |_, _| answer(0));
        let this = projectile(&mut e, 3);
        let words = |fourth: u32| args![this, 1u32, 2u32, 3u32, fourth, 5u32, 6u32];
        // The first item's byte at +0x28 clear: not allowed.
        assert!(!e.call(0x009b8900, &words(0x4000)).bool());
        assert!(calls_at(&e, IMPACT_TEST_009C2C80).is_empty());
        e.mem.set_u8(record + 0x28, 1);
        // State 3 with kind 0x11: added, and the state moves on (3 -> 4).
        assert!(e.call(0x009b8900, &words(0x4000)).bool());
        assert_eq!(
            calls_at(&e, IMPACT_TEST_009C2C80),
            vec![vec![1, 0x4000, this, 0x7000]]
        );
        assert_eq!(
            calls_at(&e, PROJECTILE_ADD_IMPACT),
            vec![vec![this, 1, 2, 3, 0x4000, 5, 6]]
        );
        assert_eq!(e.mem.u32(this + STATE), 4);
        // State 3 without the fourth word (kind 0): refused.
        e.mem.set_u32(this + STATE, 3);
        assert!(!e.call(0x009b8900, &words(0)).bool());
        assert_eq!(calls_at(&e, PROJECTILE_ADD_IMPACT).len(), 1);
        // AddImpact refusing: false.
        e.mem.set_u32(this + STATE, 0);
        e.register(PROJECTILE_ADD_IMPACT, |_, _| answer(0));
        assert!(!e.call(0x009b8900, &words(0)).bool());
    }

    #[test]
    fn fn_009b8b10_processes_impacts_and_runs_the_state_update() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(ONE_THIRTIETH, 8);
        e.map(ZERO, 8);
        e.set_global::<f64>(ONE_THIRTIETH, 0.5);
        e.register(LIST_EMPTY_008256D0, |_, _| answer(0));
        e.register(TEST_BIT_00444D00, |_, _| answer(1));
        e.register(VALUE_00979260, |_, _| float(10.0));
        e.register(VALUE_009669C0, |_, _| float(4.0));
        e.register(FLOAT_OF_00408840, |_, a| {
            float(f64::from(f32::from_bits(a[0])) * 2.0)
        });
        e.register(VALUE_00644660, |_, _| float(1.0));
        e.register(VALUE_00508070, |_, _| float(6.0));
        e.register(PROJECTILE_PROCESS_IMPACTS, |_, _| answer(1));
        let this = projectile(&mut e, 2);
        e.register_double(HOLDER_GET_00441110, move |_, _| answer(this));
        e.register(HOLDER_GET_007AF430, |_, _| answer(0));
        e.register(HOLDER_SET_009C8B90, |_, _| Ret::default());
        e.register(HOLDER_SET_009C8C40, |_, _| Ret::default());
        for update in [
            STATE_1_UPDATE_009BC8F0,
            STATE_2_UPDATE_009B8C90,
            STATE_3_UPDATE_009B90B0,
            STATE_4_UPDATE_009BA660,
        ] {
            e.register(update, |_, _| Ret::default());
        }
        // 10 - (4 * 0.5) * 2 = 6, not below 6, and the limit 1 is above 0: not processed.
        assert!(!e.call(0x009b8b10, &args![this]).bool());
        assert_eq!(
            calls_at(&e, FLOAT_OF_00408840),
            vec![vec![2.0f32.to_bits()]]
        );
        assert!(calls_at(&e, PROJECTILE_PROCESS_IMPACTS).is_empty());
        // A bound above what is left: processed; state 2 runs 009b8c90.
        e.register(VALUE_00508070, |_, _| float(6.5));
        assert!(e.call(0x009b8b10, &args![this]).bool());
        assert_eq!(
            calls_at(&e, HOLDER_SET_009C8B90),
            vec![vec![HOLDER_011F2250, 0]]
        );
        assert!(calls_at(&e, HOLDER_SET_009C8C40).is_empty());
        assert_eq!(calls_at(&e, STATE_2_UPDATE_009B8C90), vec![vec![this]]);
        // A limit at 0 processes too; bit 1 clear skips the floats; state 5 runs nothing.
        e.register(VALUE_00644660, |_, _| float(0.0));
        e.register(VALUE_00508070, |_, _| float(0.0));
        assert!(e.call(0x009b8b10, &args![this]).bool());
        e.register(TEST_BIT_00444D00, |_, _| answer(0));
        e.mem.set_u32(this + STATE, 5);
        assert!(e.call(0x009b8b10, &args![this]).bool());
        assert_eq!(calls_at(&e, STATE_2_UPDATE_009B8C90).len(), 2);
        // The byte at +0x90 set: nothing.
        e.mem.set_u8(this + 0x90, 1);
        assert!(!e.call(0x009b8b10, &args![this]).bool());
    }
}

#[cfg(test)]
mod n029_tests {
    use super::*;

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

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// An object (0x200 bytes) whose vtable has `targets` at the given slots.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x400);
        for (slot, target) in slots {
            e.mem.set_u32(table + slot, *target);
        }
        let p = e.mem.alloc(0x200);
        e.mem.set_u32(p, table);
        p
    }

    fn globals(e: &mut Engine) {
        for page in [
            PAUSE_OWNER,
            ZERO_POINT,
            MINUS_ONE,
            TWO_FLOAT,
            SETTING_011CE1DC,
            STATIC_FLOAT,
            VATS_OWNER,
            TEN,
            RANGE_512,
            RANGE_1024,
            LIMB_NAMES,
            HOLDER_011F2250,
            HINGE_COUNT,
        ] {
            e.map(page & !0xfff, 0x2000);
        }
        e.mem.set_f32(MINUS_ONE, -1.0);
        e.mem.set_f32(TWO_FLOAT, 2.0);
        e.mem.set_f32(SETTING_011CE1DC + 4, 0.5);
        e.mem.set_f32(SETTING_011CE1E8 + 4, 3.0);
        e.mem.set_f32(TEN, 10.0);
        e.mem.set_f32(RANGE_512, 512.0);
        e.mem.set_f32(RANGE_1024, 1024.0);
        e.register(SETTING_FLOAT, |_, a| ret(a[0] + 4));
        e.map(0x7e00, 0x200);
    }

    fn bounce_doubles(e: &mut Engine) {
        globals(e);
        e.register(LIST_EMPTY_008256D0, |_, _| ret(0));
        e.register(LIST_FIRST_006815C0, |_, a| ret(a[0]));
        e.register(HOLDER_GET_007AF430, |e, _| ret(e.mem.u32(0x7e00)));
        e.register(IS_PAUSED, |e, _| ret(e.mem.u32(0x7e04)));
        e.register(TEST_009D9F00, |_, _| ret(0));
        e.register(FIND_FIRST_COLLISION_OBJECT, |_, _| ret(0x4c00));
        e.register(NEXT_KEY, |_, _| ret(0x0b));
        e.register(HIT_DIRECTION, |_, a| ret(a[1]));
        e.register(FLOAT_TIMES_POINT, |_, a| ret(a[0]));
        e.register(DOT, |_, _| st0(0.25));
        e.register(POINT_SUBTRACT, |_, a| ret(a[1]));
        e.register(POINT_ADD, |_, a| ret(a[1]));
        e.register(OBJECT_004181E0, |_, _| ret(0x55));
        e.register(SHOOTER_SPEED, |_, _| st0(4.0));
        e.register(WORLD_TRANSFORM, |_, a| ret(a[0] + 0x68));
        e.register(MATRIX_TIMES_POINT, |e, a| {
            for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            ret(a[1])
        });
        e.register(CALLEE_00408820, |_, a| st0(f64::from(f32::from_bits(a[0]))));
        e.register(CALLEE_00631070, |_, _| st0(0.5));
        stub(
            e,
            &[
                HOLDER_SET_009C8C40,
                SET_FLAG,
                COLLISION_SET_00573ED0,
                SET_LOCAL_TRANSLATE,
                POINT_CONSTRUCT,
                NODE_SET_POINT,
                SET_MOTION,
                SET_COLLISION_KEY,
                SET_COLLISION_00C6A0B0,
                NORMALIZE,
                SCALE_BY,
                ADD_VELOCITY,
                MATRIX_INVERSE,
                POINT3_CONSTRUCT,
                ADD_VELOCITY_0062BA80,
                0x72a4,
            ],
        );
        e.register(0x71d0, |e, a| ret(e.mem.u32(a[0] + 0x1f0)));
    }

    #[test]
    fn fn_009b8c90_bounces_the_projectile() {
        let mut e = Engine::new();
        bounce_doubles(&mut e);
        let this = object(&mut e, &[(0x1d0, 0x71d0), (0x2a4, 0x72a4)]);
        e.mem.set_u32(this + 0x1f0, 0x0d00);
        let impact = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x88, impact);
        e.mem.set_u32(0x7e00, this);
        e.call_log = Some(vec![]);
        assert!(e.call(0x009b_8c90, &args![this]).bool());
        assert_eq!(
            calls(&e, HOLDER_SET_009C8C40),
            vec![vec![HOLDER_011F2250, 0]]
        );
        assert_eq!(calls(&e, SET_FLAG), vec![vec![this, 4, 1]]);
        assert_eq!(calls(&e, COLLISION_SET_00573ED0), vec![vec![0x4c00, 1]]);
        assert_eq!(
            calls(&e, SET_LOCAL_TRANSLATE),
            vec![vec![0x0d00, impact + 4]]
        );
        assert_eq!(calls(&e, SET_MOTION), vec![vec![0x0d00, 1, 1, 0, 1]]);
        assert_eq!(calls(&e, SET_COLLISION_KEY), vec![vec![0x0d00, 0x0b, 1, 0]]);
        assert_eq!(
            calls(&e, SET_COLLISION_00C6A0B0),
            vec![vec![0x0d00, 1, 1, 0]]
        );
        // The velocity is scaled by speed 4.0 times the setting 0.5.
        let scales: Vec<u32> = calls(&e, SCALE_BY).iter().map(|c| c[1]).collect();
        assert_eq!(scales, vec![2.0f32.to_bits(), 0.5f32.to_bits()]);
        assert_eq!(calls(&e, ADD_VELOCITY)[0][0], 0x0d00);
        // The local normal (1, 2, 3): |z| 3 is not below |y| 2, so z stays; the side
        // vector is (-z, 0, -x).
        assert!(calls(&e, CALLEE_00631070).is_empty());
        let side = calls(&e, POINT3_CONSTRUCT);
        assert_eq!(side[0][1..], [(-3.0f32).to_bits(), 0, (-1.0f32).to_bits()]);
        assert_eq!(calls(&e, ADD_VELOCITY_0062BA80)[0][0], 0x0d00);
        // Paused: no bounce.
        e.mem.set_u32(0x7e04, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x009b_8c90, &args![this]).bool());
        assert!(calls(&e, SET_LOCAL_TRANSLATE).is_empty());
        assert!(calls(&e, ADD_VELOCITY).is_empty());
        assert_eq!(calls(&e, SET_MOTION).len(), 1);
    }

    #[test]
    fn fn_009b8c90_scales_a_small_height() {
        let mut e = Engine::new();
        bounce_doubles(&mut e);
        e.register(MATRIX_TIMES_POINT, |e, a| {
            for (i, v) in [1.0f32, 4.0, 2.0].iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            ret(a[1])
        });
        let this = object(&mut e, &[(0x1d0, 0x71d0), (0x2a4, 0x72a4)]);
        e.mem.set_u32(this + 0x1f0, 0x0d00);
        let impact = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x88, impact);
        e.call_log = Some(vec![]);
        e.call(0x009b_8c90, &args![this]);
        assert!(calls(&e, HOLDER_SET_009C8C40).is_empty());
        // |z| 2 below |y| 4: z becomes 0.5 * 4.
        assert_eq!(calls(&e, CALLEE_00631070), vec![vec![2.0f32.to_bits()]]);
        let side = calls(&e, POINT3_CONSTRUCT);
        assert_eq!(side[0][1], (-2.0f32).to_bits());
        // No node: nothing.
        e.mem.set_u32(this + 0x1f0, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x009b_8c90, &args![this]).bool());
        assert!(calls(&e, SET_FLAG).is_empty());
    }

    /// The doubles of `fn_009b90b0`: objects answer from their own words (see each).
    fn cut_doubles(e: &mut Engine) {
        globals(e);
        e.register(VATS_TEST, |_, _| ret(0));
        e.register(LIST_EMPTY_008256D0, |e, a| {
            ret((e.mem.u32(a[0]) == 0) as u32)
        });
        e.register(LIST_FIRST_006815C0, |_, a| ret(a[0]));
        e.register(GET_DISMEMBERED, |e, a| ret(e.mem.u32(a[0] + 0x1e0)));
        e.register(EXTRA_LIST, |_, a| ret(a[0] + 0x100));
        e.register(DISMEMBER_EXTRA, |_, a| ret(a[0]));
        e.register(ARRAY_COUNT, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(ARRAY_ELEMENT, |e, a| ret(e.mem.u32(a[0] + 4) + 4 * a[1]));
        e.register(FIXED_STRING_CONSTRUCT, |_, a| ret(a[0]));
        e.register(NODE_SCALE, |_, _| st0(1.5));
        e.register(CLONE, |e, a| ret(e.mem.u32(a[0] + 0x1e4)));
        e.register(CHILD_COUNT, |_, _| ret(1));
        e.register(CHILD_AT, |e, a| ret(e.mem.u32(a[0] + 0x1e8)));
        e.register(FIND_SKINNED_NODE, |_, _| ret(1));
        e.register(PARENT_CELL, |_, _| ret(0x4000));
        e.register(CELL_WORLD, |e, _| ret(e.mem.u32(0x7e10)));
        e.register(CHILD_PRESENT, |_, _| ret(0));
        e.register(GLOBAL_KEY, |_, _| ret(0x77));
        e.register(MATRIX_MULTIPLY, |_, a| ret(a[1]));
        e.register(GET_LOOKING, |_, _| st0(0.5));
        e.register(POINT_SCALED, |_, a| ret(a[1]));
        e.register(POINT_SUBTRACT, |_, a| ret(a[1]));
        e.register(RAY_CAST, |e, _| ret(e.mem.u32(0x7e14)));
        e.register(SOURCE_FILTER, |_, a| ret(a[1]));
        e.register(FILTER_GROUP, |_, _| ret(0x99));
        e.register(VALUE_009669C0, |_, _| st0(2.0));
        e.register(HIT_DIRECTION, |_, a| ret(a[1]));
        e.register(HOLDER_GET_007AF430, |e, _| ret(e.mem.u32(0x7e00)));
        e.register(GET_CHAR_CONTROLLER, |e, _| ret(e.mem.u32(0x7e18)));
        e.register(CONTROLLER_FILTER, |_, a| ret(a[1]));
        e.register(SOURCE_OBJECT, |_, _| ret(0));
        e.register(NEXT_KEY, |_, _| ret(0x0b));
        e.register(VALUE_00979260, |_, _| st0(10.0));
        e.register(VALUE_00508070, |_, _| st0(4.0));
        e.register(GET_CURRENT_FURNITURE, |_, _| ret(0));
        e.register(0x71d0, |e, a| ret(e.mem.u32(a[0] + 0x1f0)));
        e.register(0x7100, |e, a| ret(e.mem.u32(a[0] + 0x1f4)));
        e.register(0x709c, |e, a| ret(e.mem.u32(a[0] + 0x1f8)));
        e.register(0x700c, |e, a| ret(e.mem.u32(a[0] + 0x1fc)));
        e.register(0x71f4, |_, a| ret(a[0] + 0x40));
        e.register(0x72bc, |_, _| st0(0.25));
        stub(
            e,
            &[
                STATE_1_UPDATE_009BC8F0,
                STATE_4_UPDATE_009BA660,
                FIXED_STRING_DESTRUCT,
                CLONING_CONSTRUCT,
                CLONING_SET,
                CLONING_DESTRUCT,
                ACTOR_HIDE_LIMB,
                NODE_SET_00450F90,
                SET_LOCAL_SCALE,
                ATTACH_TO_SKELETON,
                POINT_CONSTRUCT,
                NODE_SET_POINT,
                SET_LOCAL_TRANSLATE,
                SET_LOCAL_ROTATE,
                NODE_SET_KEY,
                UPDATE_POSITION,
                SET_MOTION,
                RAY_CONSTRUCT,
                MATRIX_ROTATION,
                MAKE_X_ROTATION,
                MATRIX_COLUMN,
                POINT_NORMALIZE,
                RAY_SET_START,
                RAY_SET_END,
                RAY_SET_FILTER,
                RAY_SET_COLLECTOR,
                COLLECTOR_CONSTRUCT,
                COLLECTOR_DESTRUCT,
                SET_LOCATION_ON_REFERENCE,
                SYNC_3D,
                SET_FLAG,
                SET_COLLISION_KEY,
                SET_COLLISION_00C6A0B0,
                POINT_SCALE_INTO,
                ADD_VELOCITY,
                HOLDER_SET_009C8C40,
                KIND_LOCAL_008C71B0,
                FILTER_SET_LAYER,
                FILTER_SET_GROUP,
                SET_CONTROLLER_POSITION,
                VECTOR_FROM_POINT,
                0x72a4,
                0x70d0,
                0x70c8,
                0x70dc,
                0x70b8,
            ],
        );
    }

    /// The scene: the projectile (`this`), its impact on an actor whose part 2 is cut, the
    /// actor's node with the limb, the clone and its skin node.
    struct Scene {
        this: u32,
        impact: u32,
        actor: u32,
        children: u32,
        clone: u32,
        skin: u32,
    }

    fn scene(e: &mut Engine) -> Scene {
        let world = object(e, &[(0xd0, 0x70d0), (0xc8, 0x70c8)]);
        e.mem.set_u32(0x7e10, world);
        let this = object(
            e,
            &[
                (0x1d0, 0x71d0),
                (0x2a4, 0x72a4),
                (0x1f4, 0x71f4),
                (0x2bc, 0x72bc),
            ],
        );
        let node = object(e, &[(0xc, 0x700c)]);
        let children = object(e, &[(0xdc, 0x70dc), (0xb8, 0x70b8)]);
        e.mem.set_u32(this + 0x1f0, node);
        e.mem.set_u32(node + 0x1fc, children);
        let actor = object(e, &[(0x100, 0x7100), (0x1d0, 0x71d0)]);
        e.mem.set_u32(actor + 0x1f4, 1);
        e.mem.set_u32(actor + 0x1e0, 1);
        let actor_node = object(e, &[(0x9c, 0x709c)]);
        e.mem.set_u32(actor + 0x1f0, actor_node);
        let limb = e.mem.alloc(0x200);
        e.mem.set_u32(actor_node + 0x1f8, limb);
        let clone = object(e, &[(0xc, 0x700c)]);
        e.mem.set_u32(limb + 0x1e4, clone);
        let skin = e.mem.alloc(0x200);
        e.mem.set_u32(clone + 0x1fc, skin);
        let skin_child = e.mem.alloc(0x100);
        e.mem.set_u32(skin + 0x1e8, skin_child);
        // The dismemberment extra: an array at `+0x20` (count at `+0x28`) of one entry for
        // part 3.
        let extra = actor + 0x100;
        let entry = e.mem.alloc(4);
        e.mem.set_u8(entry, 3);
        e.mem.set_u8(entry + 1, 1);
        let entries = e.mem.alloc(4);
        e.mem.set_u32(entries, entry);
        e.mem.set_u32(extra + 0x24, entries);
        e.mem.set_u32(extra + 0x28, 1);
        let impact = e.mem.alloc(0x30);
        e.mem.set_u32(impact, actor);
        e.mem.set_u32(impact + 0x24, 2);
        e.mem.set_u32(this + 0x88, impact);
        e.mem.set_u32(LIMB_NAMES + 8, 0x5150);
        Scene {
            this,
            impact,
            actor,
            children,
            clone,
            skin,
        }
    }

    #[test]
    fn fn_009b90b0_gives_up_or_expires() {
        let mut e = Engine::new();
        cut_doubles(&mut e);
        let s = scene(&mut e);
        // An impact already handled: nothing.
        e.mem.set_u8(s.impact + 0x28, 1);
        e.call_log = Some(vec![]);
        e.call(0x009b_90b0, &args![s.this]);
        assert!(calls(&e, STATE_1_UPDATE_009BC8F0).is_empty());
        assert!(calls(&e, CLONE).is_empty());
        // The static took the setting's float.
        assert_eq!(e.mem.f32(STATIC_FLOAT), 3.0);
        assert_eq!(e.mem.f32(s.this + 0x158), 10.0);
        e.mem.set_u8(s.impact + 0x28, 0);
        // The part was not cut: the projectile expires.
        e.mem.set_u32(s.actor + 0x1e0, 0);
        e.call_log = Some(vec![]);
        e.call(0x009b_90b0, &args![s.this]);
        assert_eq!(calls(&e, STATE_1_UPDATE_009BC8F0), vec![vec![s.this]]);
        e.mem.set_u32(s.actor + 0x1e0, 1);
        // The part was cut before (the extra's entry for part 2 has its byte set).
        let entries = e.mem.u32(s.actor + 0x124);
        let entry = e.mem.u32(entries);
        e.mem.set_u8(entry, 2);
        e.call_log = Some(vec![]);
        e.call(0x009b_90b0, &args![s.this]);
        assert_eq!(calls(&e, STATE_1_UPDATE_009BC8F0).len(), 1);
        e.mem.set_u8(entry, 3);
        // No impact: state 4.
        e.mem.set_u32(s.this + 0x88, 0);
        e.call_log = Some(vec![]);
        e.call(0x009b_90b0, &args![s.this]);
        assert_eq!(e.mem.u32(s.this + STATE), 4);
        assert_eq!(calls(&e, STATE_4_UPDATE_009BA660), vec![vec![s.this]]);
    }

    #[test]
    fn fn_009b90b0_cuts_the_limb_and_throws_it() {
        let mut e = Engine::new();
        cut_doubles(&mut e);
        let s = scene(&mut e);
        e.mem.set_u32(0x7e00, s.this);
        e.call_log = Some(vec![]);
        e.call(0x009b_90b0, &args![s.this]);
        assert_eq!(calls(&e, CLONE).len(), 1);
        assert_eq!(calls(&e, ACTOR_HIDE_LIMB), vec![vec![s.actor, 2]]);
        assert_eq!(
            calls(&e, SET_LOCAL_SCALE),
            vec![vec![s.clone, 1.5f32.to_bits()]]
        );
        let skin_child = e.mem.u32(s.skin + 0x1e8);
        assert_eq!(
            calls(&e, ATTACH_TO_SKELETON),
            vec![vec![s.skin, skin_child, s.skin, 0]]
        );
        assert_eq!(calls(&e, 0x70dc), vec![vec![s.children, s.clone, 1]]);
        assert_eq!(calls(&e, NODE_SET_KEY), vec![vec![s.clone, 0x77]]);
        assert_eq!(e.mem.u8(s.impact + 0x28), 1);
        // Not stuck: thrown with the actor's key and the projectile's velocity.
        assert_eq!(
            calls(&e, SET_COLLISION_KEY),
            vec![vec![s.children, 0x99, 1, 1]]
        );
        assert_eq!(calls(&e, SET_MOTION)[1], vec![s.children, 1, 1, 1, 1]);
        assert_eq!(calls(&e, ADD_VELOCITY)[0][0], s.children);
        assert_eq!(calls(&e, HOLDER_SET_009C8C40).len(), 1);
        assert_eq!(calls(&e, CLONING_DESTRUCT).len(), 1);
        assert_eq!(
            calls(&e, SET_LOCATION_ON_REFERENCE),
            vec![vec![s.this, s.impact + 4]]
        );
        // The first ray: 512 below to 1024 ahead.
        let scaled: Vec<u32> = calls(&e, POINT_SCALED).iter().map(|c| c[2]).collect();
        assert_eq!(scaled[..2], [512.0f32.to_bits(), 1024.0f32.to_bits()]);
    }

    #[test]
    fn fn_009b90b0_hinges_a_limb_that_sticks() {
        let mut e = Engine::new();
        cut_doubles(&mut e);
        let s = scene(&mut e);
        // The ray hits: one hit record at 0x7e80 whose collidable (+0x50) gives an object
        // with another rigid body; the second ray's hit has layer 1 and motion type 5.
        e.mem.set_u32(0x7e14, 1);
        e.mem.set_u32(0x7e18, 0x6600);
        e.register(RAY_HITS, |_, _| ret(0x7e40));
        e.register(MEMBER_AT_10, |_, a| ret(a[0] + 0x10));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0])));
        e.mem.set_u32(0x7e50, 1);
        e.register(HIT_AT, |_, _| ret(0x7e80));
        let hit_object = e.mem.alloc(0x100);
        let collidable = e.mem.alloc(0x10);
        e.mem.set_u32(collidable, hit_object);
        e.mem.set_u32(0x7e80 + 0x50, collidable);
        e.register(AV_OBJECT_FOR_COLLIDABLE, |e, a| ret(e.mem.u32(a[0])));
        e.register(FIND_FIRST_COLLISION_OBJECT, |e, a| {
            ret(e.mem.u32(a[0] + 0x30))
        });
        e.register(RIGID_BODY, |e, a| ret(e.mem.u32(a[0])));
        let own_collision = e.mem.alloc(4);
        e.mem.set_u32(own_collision, 0x6100);
        let node = e.mem.u32(s.this + 0x1f0);
        e.mem.set_u32(node + 0x30, own_collision);
        let hit_collision = e.mem.alloc(4);
        let hit_body = e.mem.alloc(0x20);
        e.mem.set_u32(hit_collision, hit_body);
        e.mem.set_u32(hit_object + 0x30, hit_collision);
        e.register(CONVERT_004AE750, |_, a| ret(a[0]));
        e.register(BODY_TRANSFORM, |_, a| ret(a[0]));
        e.register(WORLD_TRANSFORM, |_, a| ret(a[0] + 0x68));
        e.register(POINT_FROM_004A0BD0, |_, a| ret(a[1]));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(HIT_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x60);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let body = e.mem.alloc(0x20);
        e.mem.set_u32(body + 0x10, 1);
        e.mem.set_u32(body + 0x14, 5);
        e.mem.set_u32(collidable + 4, body);
        e.register(COLLIDABLE_OBJECT, |_, a| ret(a[0] + 4));
        e.register(OBJECT_BODY, |e, a| ret(e.mem.u32(a[0])));
        e.register(FIND_REFERENCE_FOR_3D, |_, _| ret(0));
        e.register(LAYER_SOURCE, |_, a| ret(a[0]));
        e.register(KIND_OF_0043B4D0, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(MOTION_TYPE, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        stub(
            &mut e,
            &[
                HINGE_CINFO_CONSTRUCT,
                HINGE_CREATE_DATA,
                HINGE_CINFO_DESTRUCT,
                TRANSFORM_FROM,
                RENORMALIZE,
                VECTOR_FROM,
                VECTOR_NORMALIZE,
                TRANSFORM_POINT,
                CONSTRAINT_BASE_CONSTRUCT,
                CONSTRAINT_INIT,
                ADD_ACT_CON,
                ADD_ACT_CON_TO_WORLD,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x009b_90b0, &args![s.this]);
        let added = calls(&e, ADD_ACT_CON);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], hit_body);
        assert_eq!(e.mem.u32(added[0][1]), HINGE_VTABLE);
        assert_eq!(calls(&e, ADD_ACT_CON_TO_WORLD)[0][0], hit_body);
        let converted = calls(&e, CONVERT_004AE750);
        assert_eq!(converted[..2], [vec![hit_body], vec![0x6100]]);
        // Stuck: the controller's key on the limb, the projectile follows it.
        assert_eq!(
            calls(&e, SET_COLLISION_KEY),
            vec![vec![s.clone, 0x99, 1, 1]]
        );
        assert_eq!(calls(&e, SET_CONTROLLER_POSITION)[0][0], 0x6600);
        assert_eq!(e.mem.u8(s.this + 0x90), 0);
        assert!(calls(&e, ADD_VELOCITY).is_empty());
        assert_eq!(calls(&e, HINGE_CINFO_DESTRUCT).len(), 1);
    }

    #[test]
    fn fn_009ba3d0_passes_the_frames_to_the_data() {
        let mut e = Engine::new();
        stub(&mut e, &[HINGE_SET_FRAMES]);
        let cinfo = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x009b_a3d0, &args![cinfo, 1u32, 2u32, 3u32, 4u32]);
        assert!(calls(&e, HINGE_SET_FRAMES).is_empty());
        e.mem.set_u32(cinfo + 4, 0x6000);
        e.call(0x009b_a3d0, &args![cinfo, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(calls(&e, HINGE_SET_FRAMES), vec![vec![0x6000, 1, 2, 3, 4]]);
    }

    #[test]
    fn fn_009ba410_constructs_a_hinge_constraint() {
        let mut e = Engine::new();
        e.map(HINGE_COUNT & !0xfff, 0x1000);
        e.map(CONSTRAINT_COUNT & !0xfff, 0x1000);
        e.mem.set_u32(HINGE_COUNT, 5);
        e.mem.set_u32(CONSTRAINT_COUNT, 7);
        stub(&mut e, &[CONSTRAINT_BASE_CONSTRUCT, CONSTRAINT_INIT]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009b_a410, &args![this, 0x3000u32]).u32(), this);
        assert_eq!(e.mem.u32(this), HINGE_VTABLE);
        assert_eq!(calls(&e, CONSTRAINT_BASE_CONSTRUCT), vec![vec![this]]);
        assert_eq!(calls(&e, CONSTRAINT_INIT), vec![vec![this, 0x3004]]);
        assert_eq!(e.mem.u32(HINGE_COUNT), 6);
        assert_eq!(e.mem.u32(CONSTRAINT_COUNT), 8);
        e.call(0x009b_a410, &args![this, 0u32]);
        assert_eq!(calls(&e, CONSTRAINT_INIT)[1], vec![this, 0]);
        // The base alone: its own vtable.
        assert_eq!(e.call(0x009b_a4a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), CONSTRAINT_VTABLE);
        assert_eq!(e.mem.u32(CONSTRAINT_COUNT), 10);
    }
}
